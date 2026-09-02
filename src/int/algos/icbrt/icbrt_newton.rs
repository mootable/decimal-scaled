// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newton integer cube root over little-endian `u64` limb slices.
//!
//! [`icbrt_newton`] is the width-agnostic Brent–Zimmermann integer Newton
//! cube root used by the fixed-width fast-arm dispatch in
//! [`crate::int::policy::icbrt`] (`N >= 3`). Pure kernel — it takes the
//! operand and writes `floor(cbrt(radicand))`; no algorithm choice.

use crate::algo_x_support::seed::cbrt_seed;
use crate::int::algos::div::div_rem_into::div_rem_into;
use crate::int::algos::support::limbs::{add_assign, bit_len, cmp};
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;

/// Scratch capacity for the width-agnostic [`icbrt_newton`] door — the
/// build-max `4·MAX_WORK_N + ⌈MAX_WORK_N/2⌉`, the only sizing available to a
/// caller with no `N`. Callers that DO hold a concrete `N` must use
/// [`icbrt_newton_into`] instead: this constant is selected by the build's
/// WIDTH FEATURES (72 limbs at `wide`, 288 at `xx-wide`), so a kernel sized
/// from it makes every narrow call pay for the widest enabled tier — the
/// feature-coupling the exact-scratch mechanism exists to remove.
use crate::int::algos::support::limbs::max_n_limbs;
use crate::int::types::compute_limbs::MAX_SINGLE_LIMBS;

const SCRATCH_LIMBS: usize = max_n_limbs(4);

/// Build-max Knuth normalisation scratch for [`icbrt_newton`] — the same
/// `MAX_SINGLE_LIMBS` budget [`div_knuth`](crate::int::algos::div::div_knuth)
/// allocates at its own blanket door, hoisted here so it is zeroed ONCE per
/// root instead of once per Newton iteration.
const DIV_SCRATCH_LIMBS: usize = MAX_SINGLE_LIMBS;

/// Build-max base-2¹²⁸ packed scratch — `⌈MAX_SINGLE_LIMBS/2⌉ + 2`, matching
/// `div_knuth_u128_limb`'s own blanket sizing.
const DIV_SCRATCH_LIMBS_128: usize = MAX_SINGLE_LIMBS / 2 + 2;

/// `out = floor(cbrt(radicand))`. Newton iteration for the integer cube root.
///
/// Implements the Brent–Zimmermann integer Newton iteration for cube root
/// (Modern Computer Arithmetic §1.5.2): starting from a safe over-estimate
/// of the root, each step applies
///
///   `s_new = (2·s + n / s²) / 3`
///
/// which converges monotonically downward to `floor(n^(1/3))`. Convergence
/// is quadratic once the error is small, so the total iteration count is
/// `O(log₂(bits))`.
///
/// The seed is delegated to the cross-algorithm seed leaf
/// ([`crate::algo_x_support::seed::cbrt_seed`]): under `std` it is derived
/// from the hardware `f64::cbrt` of the top 64 bits of `radicand`, scaled
/// back and rounded up to a safe over-estimate; under `no_std` it is the
/// classical pure-integer 1-bit seed `2^ceil(bits/3)`. No `libm` /
/// `num_traits::Float` is reached either way.
///
/// All arithmetic uses fixed-size scratch buffers — no heap allocation,
/// `core`/no_std-safe.
///
/// Hasselgren seed strategy: see Crandall & Pomerance 2005, "Prime Numbers:
/// A Computational Perspective" §9.2.1.
///
/// This is the **build-max** door, for callers with no `N` to size scratch
/// from (the `Uint<N>` fast-arm dispatch, the bench seam). It allocates the
/// Newton working buffers plus the divide's normalisation scratch at the
/// build-max width and delegates to [`icbrt_newton_into`]; a caller holding a
/// concrete `N` (`Limbs<N>: ComputeLimbs` — the decimal `cbrt` kernel) calls
/// that door directly with its own exactly-sized buffers.
pub(crate) fn icbrt_newton(radicand: &[u64], out: &mut [u64]) {
    debug_assert!(radicand.len() + 1 <= SCRATCH_LIMBS, "icbrt scratch overflow");
    let mut x = [0u64; SCRATCH_LIMBS];
    let mut sq = [0u64; SCRATCH_LIMBS];
    let mut q = [0u64; SCRATCH_LIMBS];
    let mut r = [0u64; SCRATCH_LIMBS];
    let mut u = [0u64; DIV_SCRATCH_LIMBS];
    let mut v = [0u64; DIV_SCRATCH_LIMBS];
    let mut u128_u = [0u128; DIV_SCRATCH_LIMBS_128];
    let mut u128_v = [0u128; DIV_SCRATCH_LIMBS_128];
    icbrt_newton_into(
        radicand, out, &mut x, &mut sq, &mut q, &mut r, &mut u, &mut v, &mut u128_u,
        &mut u128_v,
    );
}

/// `out = floor(cbrt(radicand))` in **caller-provided scratch** — the real
/// implementation, and the exact-scratch sibling of [`icbrt_newton`].
///
/// The Brent–Zimmermann recurrence `s ← (2·s + n/s²) / 3` from a guaranteed
/// over-estimate seed. Both divides go through the exact-scratch door
/// [`div_rem_into`] rather than the build-max
/// [`dispatch`](crate::int::policy::div_rem::dispatch), whose `div_knuth`
/// wrapper allocates and zeroes two `MAX_SINGLE_LIMBS` arrays **per
/// iteration** (2 × 258 limbs at `xx-wide`, 2 × 66 at `wide` — for the same
/// work).
///
/// Required scratch lengths, with `L = radicand.len()` and `W = L + 1`:
///
/// | buffer | minimum | role |
/// |---|---|---|
/// | `x`, `q` | `W` | estimate, `2·s + n/s²` accumulator |
/// | `sq` | enough for `s²` (`≈ 2W/3` significant limbs) | the squared estimate |
/// | `r` | `max(sq.len(), W)` | divide remainder sink, then the `/3` quotient |
/// | `u` | `L + 2` | Knuth normalised dividend |
/// | `v` | the EFFECTIVE limb count of `s²` (`≈ 2W/3`) | Knuth normalised divisor |
/// | `u128_u`, `u128_v` | see [`div_rem_into`] | base-2¹²⁸ packed scratch |
///
/// `v` is stated in effective limbs rather than `sq.len()` because the divisor
/// slice is deliberately over-long: the engines strip trailing zeros before
/// indexing `v`, and `s²` occupies about two thirds of the radicand's width
/// however far `sq` extends. The build-max door relies on the same fact — its
/// `v` is `MAX_SINGLE_LIMBS`, which is SHORTER than its `sq`.
///
/// `sq` is used at `min(2W, sq.len())` limbs — the same clamp the build-max
/// door applies against `SCRATCH_LIMBS`. Clamping is safe because `s ≈ n^(1/3)`
/// makes `s²` about `2/3` of the radicand's length regardless of how far `x`
/// is zero-padded, so the truncated high limbs are zero; a shorter divisor
/// SLICE holding the same value takes the same matcher verdict (both
/// `select_for_limbs` and `div_knuth_into` strip trailing zeros) and yields the
/// same quotient.
///
/// The buffers may be **dirty** on entry and are reusable across calls: `x` and
/// `sq` are zeroed over their live prefix here, `r` is zeroed before the `/3`
/// divide, and the divide engines re-zero their own outputs.
#[allow(clippy::too_many_arguments)]
pub(crate) fn icbrt_newton_into(
    radicand: &[u64],
    out: &mut [u64],
    x: &mut [u64],
    sq: &mut [u64],
    q: &mut [u64],
    r: &mut [u64],
    u: &mut [u64],
    v: &mut [u64],
    u128_u: &mut [u128],
    u128_v: &mut [u128],
) {
    for limb in out.iter_mut() {
        *limb = 0;
    }
    let bits = bit_len(radicand);
    if bits == 0 {
        return;
    }
    if bits <= 1 {
        // radicand == 0 already handled; radicand == 1 → root is 1.
        out[0] = 1;
        return;
    }
    // The cube root has at most `ceil(bits / 3)` bits, so the intermediate s²
    // has at most `2·ceil(bits/3)` — comfortably inside the caller's `sq`.
    let work_len = radicand.len() + 1;

    // ── seed ──────────────────────────────────────────────────────────
    // Delegated to the cross-algorithm seed leaf
    // (`algo_x_support::seed::cbrt_seed`): under `std` it is the hardware
    // `f64::cbrt` of the top 64 bits of `radicand` scaled back and rounded up
    // to a strict over-estimate; under `no_std` it is the classical
    // pure-integer 1-bit seed `2^ceil(bits/3)`. Both over-estimate, so the
    // monotone-downward Newton loop below converges to the same floor root.
    // The leaf calls nothing in-crate — `num_traits::Float`/libm is never
    // reached.
    //
    // The seed ORs into its destination, so the live prefix is zeroed first
    // (the caller's buffer may carry a previous call's limbs).
    for limb in x[..work_len].iter_mut() {
        *limb = 0;
    }
    cbrt_seed(radicand, bits, &mut x[..work_len]);

    // ── Newton loop ───────────────────────────────────────────────────
    // Invariant: x ≥ floor(cbrt(radicand)) at entry of each iteration.
    // The iteration s_new = (2*s + n/s²) / 3 is monotone-non-increasing
    // and halts when s_new ≥ s (i.e. s is the floor root).
    //
    // Per pass only the live slices are touched: `sq` is re-zeroed
    // (mul_schoolbook accumulates); the n/s² divide re-zeros `q`/`r`; the /3
    // divide's quotient and remainder are re-zeroed defensively (the
    // single-limb divisor path may not). `r` serves twice — first as the n/s²
    // remainder sink (never read), then, over its `[..work_len]` prefix, as
    // the `/3` quotient `y` — so the two roles need one buffer, not two.
    // `x = y` is a `[..work_len]` copy.
    let three = [3u64];
    let mut rem3_buf = [0u64; 1];
    let sq_len = (work_len * 2).min(sq.len());
    loop {
        // t = s²  (2 * work_len limbs, but only work_len+1 matter)
        for limb in sq[..sq_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&x[..work_len], &x[..work_len], &mut sq[..sq_len]);

        // q = n / s²  (the divide engine re-zeros q[..work_len] / r[..sq_len])
        div_rem_into(radicand, &sq[..sq_len], &mut q[..work_len], &mut r[..sq_len],
            u, v, u128_u, u128_v);

        // t = 2*s + q: add 2*x into q.
        // 2*s = s << 1: add s twice (no overflow into extra limbs because
        // the result fits in work_len+1 limbs by the cube-root bound).
        add_assign(&mut q[..work_len], &x[..work_len]);
        add_assign(&mut q[..work_len], &x[..work_len]);

        // y = t / 3, written over `r`'s live prefix.
        for limb in r[..work_len].iter_mut() {
            *limb = 0;
        }
        rem3_buf[0] = 0;
        div_rem_into(&q[..work_len], &three, &mut r[..work_len], &mut rem3_buf[..1],
            u, v, u128_u, u128_v);

        if cmp(&r[..work_len], &x[..work_len]) >= 0 {
            break;
        }
        x[..work_len].copy_from_slice(&r[..work_len]);
    }
    let copy_len = if out.len() < work_len { out.len() } else { work_len };
    out[..copy_len].copy_from_slice(&x[..copy_len]);
}
