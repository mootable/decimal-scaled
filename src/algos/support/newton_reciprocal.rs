//! Newton–Raphson reciprocal divide for `n / 10^SCALE` at storage width.
//!
//! Research kernel — **not wired into the dispatcher**. Built behind a
//! `pub(crate)` API so micro-benches can compare it head-to-head against
//! [`crate::algos::support::mg_divide::div_wide_pow10_chain`].
//!
//! # Algorithm
//!
//! For invariant divisor `D = 10^SCALE`, precompute a fixed-point
//! reciprocal
//!
//! ```text
//!   R = floor(2^k / D)
//! ```
//!
//! where `k` is chosen so that `k - bit_length(D) ≥ bit_length(N_max)`,
//! i.e. `R` carries enough fractional bits to represent the storage-width
//! numerator's worth of quotient. The per-call divide reduces to
//!
//! ```text
//!   q_approx = (n * R) >> k
//!   r        = n - q_approx * D
//!   if r >= D { q_approx += 1; r -= D; }   // single correction step
//! ```
//!
//! The estimate `q_approx` is off by at most 1 (analogous to the
//! Möller-Granlund add-back correction), so a single comparison suffices
//! after the multiply.
//!
//! # Setup
//!
//! `R` is computed once per `(SCALE, width)` pair via the int-algos-layer
//! variable-length divmod [`crate::int::algos::div::div_rem_mag_slice`].
//! Setup cost is one wide divide; per-call cost is one wide multiply +
//! one narrow multiply + one comparison + one optional subtract.
//!
//! # Storage — raw `u64` limb slices, below the `Int<N>` abstraction
//!
//! All scratch is held in fixed-size `u64` limb buffers (little-endian),
//! `core`-only — no heap, no `alloc`. The `BigInt` magnitude/sign bridge
//! still moves through the `u128`-limb buffer (`mag_into_u128` /
//! `from_mag_sign_u128`), but every arithmetic step runs on `u64` limbs.
//!
//! This kernel deliberately does **not** route through the `Int<N>` type
//! methods (`Int::<N>::div_rem`, `*`, …). The reciprocal `R` and the
//! `10^SCALE` divisor have a **runtime live limb count** (`k_u64 + 1` and
//! `pow_len`, both functions of the runtime `scale` / `width` arguments)
//! that no const-generic `Int<N>` width can express: a single
//! `precompute` call can produce an `R` anywhere from ~18 to 144 u64
//! limbs. `Int::<N>::div_rem` fixes `N` at monomorphisation, so the
//! variable-width reciprocal divide cannot be expressed through it. The
//! per-call multiplies are likewise schoolbook over variable-length live
//! slices. The kernel therefore stays on raw `u64` slices — but it
//! reaches the dispatching divmod through the int-**algos** layer
//! ([`crate::int::algos::div::div_rem_mag_slice`], which fronts the
//! divisor-shape policy and picks the optimal engine), never the
//! `int::policy` layer directly, keeping the decimal→int layering intact.
//!
//! # Reference
//!
//! Granlund, T. & Montgomery, P. L. (1994). *Division by Invariant
//! Integers using Multiplication*, PLDI '94. Möller, N. & Granlund, T.
//! (2011). *Improved Division by Invariant Integers*, IEEE TC 60(2).
//! The Newton-iteration view of the same reciprocal is
//! Wikipedia — [Division algorithm § Newton–Raphson division](https://en.wikipedia.org/wiki/Division_algorithm#Newton%E2%80%93Raphson_division).

use crate::int::algos::div::div_fixed::div_rem_mag_slice;
use crate::int::algos::support::limbs::{cmp, sub_assign};
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;

// ── Fixed buffer sizing (in u64 limbs) ──────────────────────────────
//
// The widest cell exercised is `width_limbs = 32` u128 limbs (Int<64>,
// 4096-bit storage) at `scale` up to ~1231 (the bench sweep). Working in
// u64 limbs (two per u128 limb), the worst-case sizes are:
//
//   pow_scale : pow_u128 = scale/38 + 2 ≤ 36 u128 → 72 u64
//   r         : (k_u128 + 1) = (width + pow + 1) ≤ 68 u128 → 136 u64
//   mag (n)   : 64 u128 → 128 u64
//   product   : n.len() + r.len() ≤ 128 + 136 = 264 u64
//
// All buffers are over-sized to a single generous ceiling so the same
// type serves every tier without const-generic gymnastics.

/// Max `u64` limbs for the `10^SCALE` (`pow_scale`) buffer.
const MAX_POW_U64: usize = 80;
/// Max `u64` limbs for the reciprocal (`r`) buffer.
const MAX_R_U64: usize = 144;
/// Max `u64` limbs for the magnitude / quotient buffers.
const MAX_MAG_U64: usize = 128;
/// Max `u64` limbs for product / scratch buffers (`n·r`, `q·D`, …).
const MAX_PROD_U64: usize = 288;

/// Pre-computed reciprocal table for a single `(SCALE, mag_width)` pair.
///
/// `r` is the reciprocal `floor(2^k / 10^SCALE)` in little-endian
/// u64 limbs; `k_u64` is `k / 64` (we always pick `k` as a multiple of
/// 64 so the shift is a limb-aligned slice).
///
/// `pow_scale` is `10^SCALE` in little-endian u64 limbs, kept for the
/// correction step.
///
/// All storage is fixed-size — no heap. `r_len` / `pow_len` record the
/// live limb counts within the over-sized backing arrays.
#[derive(Clone)]
pub struct NewtonReciprocal {
    /// Reciprocal limbs (little-endian, u64), live count `r_len`.
    r: [u64; MAX_R_U64],
    /// Live limb count of `r`.
    r_len: usize,
    /// Right-shift amount in u64 limbs (quotient = (n·r) limbs >> k_u64 words).
    k_u64: usize,
    /// `10^SCALE` limbs (little-endian, u64), live count `pow_len`.
    pow_scale: [u64; MAX_POW_U64],
    /// Live limb count of `pow_scale`.
    pow_len: usize,
}

impl NewtonReciprocal {
    /// Compute reciprocal table for `D = 10^scale` at the given
    /// magnitude width.
    ///
    /// `width_u64_limbs` is the upper bound on the numerator magnitude's
    /// limb count, expressed in **u64 limbs** — the unit the kernel's
    /// arithmetic actually runs in.
    pub fn precompute(scale: u32, width_u64_limbs: usize) -> Self {
        let width_limbs = width_u64_limbs;

        // pow_scale = 10^scale via repeated *10 on a wide u64 buffer.
        // 10^scale needs about scale * log2(10) ≈ scale * 3.322 bits.
        // Each u64 limb absorbs ~19 decimal digits; use scale/19 + 2
        // u64 limbs (matches the prior u128 path's scale/38 + 2 u128
        // budget, doubled).
        let pow_len = (scale as usize / 19 + 3).max(1);
        debug_assert!(pow_len <= MAX_POW_U64, "pow_scale buffer too small");
        let mut pow_scale = [0u64; MAX_POW_U64];
        pow_scale[0] = 1u64;
        for _ in 0..scale {
            // multiply pow_scale[..pow_len] by 10
            let mut carry: u64 = 0;
            for limb in pow_scale[..pow_len].iter_mut() {
                let prod = (*limb as u128) * 10u128 + (carry as u128);
                *limb = prod as u64;
                carry = (prod >> 64) as u64;
            }
            debug_assert_eq!(carry, 0, "pow_scale buffer too small at scale={scale}");
        }

        // Pick k_u64: quotient room of `width_limbs` u64 limbs. Set
        // k = 64 * (width_limbs + pow_len) bits — then R = 2^k / 10^scale
        // has bit-length about k - bits(10^scale), and (n·R) >> k yields
        // a width_limbs-wide quotient with at most 1 ULP error.
        let k_u64 = width_limbs + pow_len;

        // numerator = 2^(64 * k_u64) — a single 1 in limb position k_u64.
        debug_assert!(k_u64 < MAX_R_U64, "num buffer too small");
        let mut num = [0u64; MAX_R_U64];
        num[k_u64] = 1u64;

        // r = num / pow_scale.
        let mut r = [0u64; MAX_R_U64];
        let mut rem = [0u64; MAX_POW_U64];
        div_rem_mag_slice(
            &num[..k_u64 + 1],
            &pow_scale[..pow_len],
            &mut r[..k_u64 + 1],
            &mut rem[..pow_len],
        );

        Self {
            r,
            r_len: k_u64 + 1,
            k_u64,
            pow_scale,
            pow_len,
        }
    }
}

/// Per-call Newton-reciprocal divide.
///
/// `n` is the unsigned numerator magnitude in little-endian u64 limbs.
/// The quotient `floor(n / 10^scale)` is written into `quot` (caller-
/// sized to the target width); the remainder is written into `rem_out`
/// and its live limb count returned, for rounding-aware callers.
///
/// # Precision
///
/// Strict: the result is bit-exact `floor(n / 10^scale)`. The Newton
/// add-back step ensures correctness for the at-most-1 over/under
/// estimate the truncated reciprocal produces.
fn div_newton(
    n: &[u64],
    table: &NewtonReciprocal,
    quot: &mut [u64],
    rem_out: &mut [u64],
) -> usize {
    let r = &table.r[..table.r_len];
    let pow_scale = &table.pow_scale[..table.pow_len];

    // product = n * r
    let prod_len = n.len() + r.len();
    debug_assert!(prod_len <= MAX_PROD_U64, "product buffer too small");
    let mut prod = [0u64; MAX_PROD_U64];
    mul_schoolbook(n, r, &mut prod[..prod_len]);

    // q_approx = prod >> (64 * k_u64)
    let lo = table.k_u64.min(prod_len);
    let q_slice = &prod[lo..prod_len];
    for (dst, src) in quot.iter_mut().zip(q_slice.iter()) {
        *dst = *src;
    }
    for dst in quot.iter_mut().skip(q_slice.len()) {
        *dst = 0;
    }

    // r_approx = n - q_approx * pow_scale  (mod 2^width)
    let prod2_len = quot.len() + pow_scale.len();
    debug_assert!(prod2_len <= MAX_PROD_U64, "product buffer too small");
    let mut prod2 = [0u64; MAX_PROD_U64];
    mul_schoolbook(quot, pow_scale, &mut prod2[..prod2_len]);

    // rem = n - prod2 (mod 2^width), held in n.len()+1 limbs.
    let rem_len = n.len() + 1;
    debug_assert!(rem_len <= MAX_MAG_U64 + 1, "rem buffer too small");
    for (dst, src) in rem_out.iter_mut().take(rem_len).zip(n.iter()) {
        *dst = *src;
    }
    rem_out[rem_len - 1] = 0;
    let sub_len = prod2_len.min(rem_len);
    let _ = sub_assign(&mut rem_out[..sub_len], &prod2[..sub_len]);

    // Correction loop: while rem >= pow_scale, bump quotient by 1 and
    // decrement remainder. With a correctly-sized k_u64 the loop runs at
    // most once or twice.
    loop {
        if cmp(&rem_out[..rem_len], pow_scale) < 0 {
            break;
        }
        let s = rem_len.min(pow_scale.len());
        let _ = sub_assign(&mut rem_out[..s], &pow_scale[..s]);
        // quot += 1
        let mut carry: u64 = 1;
        for limb in quot.iter_mut() {
            let (s, c) = limb.overflowing_add(carry);
            *limb = s;
            if !c {
                carry = 0;
                break;
            }
        }
        let _ = carry;
    }

    rem_len
}

/// Full `n / 10^SCALE` with rounding for a `BigInt`-backed value.
///
/// Direct analogue of [`crate::algos::support::mg_divide::div_wide_pow10_chain`]
/// — same signature, same semantics, different inner algorithm.
pub(crate) fn div_wide_pow10_newton_with<W: crate::int::types::traits::BigInt>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
    table: &NewtonReciprocal,
) -> W {
    // BigInt bridge is u128-limb; the arithmetic core operates on that
    // magnitude slice in place (shared with the `Int<N>`-only decimal
    // `mul` kernel, which builds its product directly in u128 scratch).
    let mut mag_u128 = [0u128; 64];
    let neg = n.mag_into_u128(&mut mag_u128);
    newton_pow10_mag_u128(&mut mag_u128, neg, mode, table);
    W::from_mag_sign_u128(&mag_u128, neg)
}

/// Width-agnostic Newton-reciprocal divide of a u128-limb magnitude slice
/// by `10^scale`, in place, with `mode`-aware rounding. `table` is the
/// pre-computed reciprocal for `(scale, width)`. Slice core extracted from
/// [`div_wide_pow10_newton_with`]; the only difference from the typed path
/// is the `BigInt` pack/unpack the wrapper does around this call.
///
/// The interior arithmetic runs on u64 limbs (`div_newton`), so this
/// transcodes the u128 magnitude to u64, divides, rounds, and transcodes
/// the quotient back into `mag` in place.
pub(crate) fn newton_pow10_mag_u128(
    mag_u128: &mut [u128],
    neg: bool,
    mode: crate::support::rounding::RoundingMode,
    table: &NewtonReciprocal,
) {
    use crate::support::rounding;

    // Transcode the u128 magnitude to the u64 limbs the kernel runs in.
    let mut mag = [0u64; MAX_MAG_U64];
    for (i, &v) in mag_u128.iter().enumerate() {
        mag[2 * i] = v as u64;
        mag[2 * i + 1] = (v >> 64) as u64;
    }
    let mag_len = mag_u128.len() * 2;

    let mut top = mag_len;
    while top > 0 && mag[top - 1] == 0 {
        top -= 1;
    }

    let n_slice = &mag[..top.max(1)];
    let mut quot = [0u64; MAX_MAG_U64];
    let mut rem = [0u64; MAX_MAG_U64 + 1];
    let rem_len = div_newton(n_slice, table, &mut quot[..mag_len], &mut rem);

    // Round per `mode`: compare remainder with pow_scale / 2.
    let rem_is_zero = rem[..rem_len].iter().all(|&x| x == 0);
    if !rem_is_zero {
        // half = pow_scale / 2 (pow_scale is even for scale >= 1)
        let pow_len = table.pow_len;
        let mut half = [0u64; MAX_POW_U64];
        half[..pow_len].copy_from_slice(&table.pow_scale[..pow_len]);
        // shift right by 1
        let mut i = pow_len;
        let mut carry_in: u64 = 0;
        while i > 0 {
            i -= 1;
            let next_carry = half[i] & 1;
            half[i] = (carry_in << 63) | (half[i] >> 1);
            carry_in = next_carry;
        }

        let cmp_r = match cmp(&rem[..rem_len], &half[..pow_len]) {
            n if n < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        };
        let q_is_odd = (quot[0] & 1) != 0;
        if rounding::should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u64 = 1;
            for limb in quot[..mag_len].iter_mut() {
                let (s, c) = limb.overflowing_add(carry);
                *limb = s;
                if !c {
                    carry = 0;
                    break;
                }
            }
            let _ = carry;
        }
    }

    // Re-pack the u64 quotient into the caller's u128 magnitude slice.
    for (i, slot) in mag_u128.iter_mut().enumerate() {
        let lo = quot[2 * i] as u128;
        let hi = quot[2 * i + 1] as u128;
        *slot = lo | (hi << 64);
    }
}

/// Width-keyed dispatch decision for `n / 10^SCALE`.
///
/// Returns `true` when the bench-validated Newton-vs-MG matrix says
/// Newton wins for this `(width_bits, scale)` cell. The matrix:
///
/// | Storage  | bits | Newton min SCALE |
/// |----------|------|------------------|
/// | Int<32>    | 2048 |  ≥ 200           |
/// | Int<48>    | 3072 |  ≥ 200           |
/// | Int<64>    | 4096 |  ≥ 400           |
///
/// Bench source: `benches/newton_vs_mg.rs` head-to-head against
/// [`crate::algos::support::mg_divide::div_wide_pow10_chain`] at the
/// listed widths × representative SCALE bands. Larger widths (Int<128>
/// / Int<192> / Int<256> — used by the transcendental work integers)
/// have no bench data and fall through to MG.
///
/// Scale `≤ 38` always returns `false`: the single-pass MG kernel
/// `div_wide_pow10` is the chosen winner there and a chain-Newton
/// would be both slower and indistinguishable rounding-wise.
#[inline]
const fn newton_wins(width_bits: u32, scale: u32) -> bool {
    if scale <= 38 {
        return false;
    }
    match width_bits {
        2048 if scale >= 200 => true,
        3072 if scale >= 200 => true,
        4096 if scale >= 400 => true,
        _ => false,
    }
}

/// Per-`(width_bits, scale)` reciprocal table cache.
///
/// Mirrors the existing `pow10_cached` / `pi_cached` / `ln2_cached`
/// thread-local `Vec<(u32, …)>` pattern in
/// [`crate::macros::wide_transcendental`]. Linear scan over the live
/// SCALEs (typically 1–3 entries per build); each miss runs one
/// `NewtonReciprocal::precompute(scale, width_limbs)` then keeps the
/// table for the rest of the thread's lifetime.
///
/// Three separate slots — one per cached width — because the
/// `width_limbs` argument differs (32 / 48 / 64 u64 limbs for
/// Int<32> / Int<48> / Int<64>) and the `NewtonReciprocal` allocates
/// limb-storage sized to that argument.
#[cfg(feature = "std")]
mod cache {
    use super::NewtonReciprocal;
    use ::std::thread_local;

    thread_local! {
        static C_2048: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
            ::core::cell::RefCell::new(alloc::vec::Vec::new())
        };
        static C_3072: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
            ::core::cell::RefCell::new(alloc::vec::Vec::new())
        };
        static C_4096: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
            ::core::cell::RefCell::new(alloc::vec::Vec::new())
        };
    }

    /// Run `f` with a borrowed reciprocal table for `(width_bits, scale)`.
    /// On first call per `(thread, width_bits, scale)` the table is
    /// computed and stashed; subsequent calls borrow it from the slot.
    pub(super) fn with_table<R>(
        width_bits: u32,
        scale: u32,
        width_limbs: usize,
        f: impl FnOnce(&NewtonReciprocal) -> R,
    ) -> R {
        let slot = match width_bits {
            2048 => &C_2048,
            3072 => &C_3072,
            4096 => &C_4096,
            _ => unreachable!("with_table called on un-cached width {width_bits}"),
        };
        // Ensure the slot has an entry for `scale`; insert one if not.
        // The thread_local + RefCell pattern avoids ever holding the
        // borrow across the precompute itself (precompute does not
        // re-enter the cache, but keeping the borrow scope tight is
        // robust against future changes).
        let needs_insert = slot.with(|c| {
            let cache = c.borrow();
            !cache.iter().any(|(s, _)| *s == scale)
        });
        if needs_insert {
            let table = NewtonReciprocal::precompute(scale, width_limbs);
            slot.with(|c| {
                let mut cache = c.borrow_mut();
                if !cache.iter().any(|(s, _)| *s == scale) {
                    cache.push((scale, table));
                }
            });
        }
        slot.with(|c| {
            let cache = c.borrow();
            let entry = cache
                .iter()
                .find(|(s, _)| *s == scale)
                .expect("cache invariant: entry inserted above");
            f(&entry.1)
        })
    }
}

/// Width-class dispatch for `n / 10^SCALE`.
///
/// When the `(W::BITS, scale)` cell wins under [`newton_wins`] the
/// call routes through the Newton kernel with a thread-local cached
/// reciprocal table; otherwise it forwards to the MG chain kernel.
///
/// Used at the `mul` / transcendental-rounding call sites where the
/// numerator width is `W` and `scale` is a runtime value — see the
/// matching call sites in `macros::arithmetic::decl_decimal_arithmetic`
/// and `macros::wide_transcendental::decl_wide_transcendental`.
#[inline]
pub(crate) fn dispatch_wide_pow10<W>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
) -> W
where
    W: crate::int::types::traits::BigInt + crate::int::types::compute_int::ComputeInt,
{
    let bits = <W as crate::int::types::traits::BigInt>::BITS;
    // u128 magnitude buffer from ComputeInt (size lives in the impl); no
    // const work-width parameter — same mechanism as `div_wide_pow10`.
    let mut buf = <W as crate::int::types::compute_int::ComputeInt>::single_u128();
    let mag = &mut buf.as_mut()[..W::U128_LIMBS];
    let neg = n.mag_into_u128(mag);
    dispatch_pow10_mag_u128(mag, scale, neg, mode, bits);
    W::from_mag_sign_u128(mag, neg)
}

/// Width-agnostic dispatch for `mag / 10^scale`, in place on a u128-limb
/// magnitude slice. `width_bits` is the work-width in bits (`mag.len() *
/// 128`-bounded; supplied by the caller as the cache / `newton_wins` key).
///
/// Routes Newton vs MG-chain by [`newton_wins`], threading the
/// thread-local reciprocal cache when Newton wins (std only). Shared with
/// the typed [`dispatch_wide_pow10`] wrapper and the `Int<N>`-only
/// decimal `mul` kernel.
#[inline]
pub(crate) fn dispatch_pow10_mag_u128(
    mag: &mut [u128],
    scale: u32,
    neg: bool,
    mode: crate::support::rounding::RoundingMode,
    width_bits: u32,
) {
    if !newton_wins(width_bits, scale) {
        crate::algos::support::mg_divide::div_pow10_chain_mag_u128(mag, scale, neg, mode);
        return;
    }

    #[cfg(feature = "std")]
    {
        // `width_limbs` in u64 limbs — the `precompute` unit.
        let width_limbs = (width_bits as usize) / 64;
        cache::with_table(width_bits, scale, width_limbs, |table| {
            newton_pow10_mag_u128(mag, neg, mode, table);
        });
    }

    #[cfg(not(feature = "std"))]
    {
        // no_std fallback: no thread-local cache available; per-call
        // precompute is too costly for the wide tier (one Knuth divide
        // at storage width). Forward to MG instead — Newton wins
        // depend on amortising the table across many calls.
        crate::algos::support::mg_divide::div_pow10_chain_mag_u128(mag, scale, neg, mode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::support::mg_divide::div_wide_pow10_chain;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    #[test]
    fn newton_matches_mg_chain_d307_s150() {
        let scale = 150u32;
        let width_limbs = 16; // Int<16> = 8 u128 = 16 u64 limbs
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[6] = 1u128 << 32;
        limbs[0] = 42;
        let n = <Int<16> as crate::int::types::traits::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain::<Int<16>>(n, scale, RoundingMode::HalfToEven);
        assert_eq!(got, want, "Newton differs from MG chain at D307 s=150");
    }

    #[test]
    fn newton_matches_mg_chain_d616_s308() {
        let scale = 308u32;
        let width_limbs = 32; // Int<32> = 16 u128 = 32 u64 limbs
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[14] = 1u128 << 16;
        limbs[3] = 0xdeadbeef;
        let n = <Int<32> as crate::int::types::traits::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain::<Int<32>>(n, scale, RoundingMode::HalfToEven);
        assert_eq!(got, want, "Newton differs from MG chain at D616 s=308");
    }

    #[test]
    fn newton_matches_mg_chain_d1232_s615() {
        let scale = 615u32;
        let width_limbs = 64; // Int<64> = 32 u128 = 64 u64 limbs
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[30] = 1u128 << 8;
        limbs[5] = 0xcafef00d;
        let n = <Int<64> as crate::int::types::traits::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain::<Int<64>>(n, scale, RoundingMode::HalfToEven);
        assert_eq!(got, want, "Newton differs from MG chain at D1232 s=615");
    }
}
