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
// The widest cell exercised is `width_limbs = 96` u64 limbs (6144-bit,
// the D230 Wexp / D924 Work tier). Working in u64 limbs (two per u128
// limb), the worst-case sizes — taking the AGM-widened scale ceiling
// `w_prime = 2·SCALE + 4` (D924 max SCALE 923 → w_prime ≤ 1850) — are:
//
//   pow_scale : pow_u64 = scale/19 + 3 ≤ 100 u64 (at w_prime ≤ 1850)
//   r         : (k_u64 + 1) = (width + pow + 1) ≤ 96 + 100 + 1 = 197 u64
//   mag (n)   : 96 u64 (Int<96>); legacy 4096 path still uses ≤ 64
//   product   : n.len() + r.len() ≤ 128 + 200 = 328 u64
//
// All buffers are over-sized to a single ceiling that covers every
// width the matcher routes Newton-vs-MG against, so the same type
// serves every tier without const-generic gymnastics.
//
// The build-max is internal to the runtime-instantiated
// `NewtonReciprocal` cache type — it never leaks onto a concrete-`N`
// path (those still size their scratch via `ComputeInt::single_*` etc.
// per Constitution rule 6). The cache stores ONE struct per cached
// width per thread; over-sizing here costs constant per-slot stack +
// per-slot struct memory, not per-tier code duplication.
//
// The 8192 / 12288 / 16384 / 32768 widths the 2026-05-28 audit also
// identified (D462 Wexp / D1232 Work / D924 Wide / D616 Wexp / D924
// Wexp / D1232 Wide / D1232 Wexp) are deferred. At those widths the
// Newton precompute's `2^k / 10^scale` numerator at the AGM-widened
// scale exceeds the routed `div_knuth`'s `MAX_SINGLE_LIMBS = 258`
// build-max scratch — D1232 strict_agm runs at `w_prime ≈ 2466`
// which puts an 8192-bit Newton's numerator at 260+ limbs, and a
// 12288-bit Newton's at 293+, both past the build-max scratch the
// routed Knuth can hold. The sibling-agent atanh-diagnosis bench
// also reported Newton LOSING by 5–58× at Int<192>/Int<256> w=38
// (low-scale shape), so even with extended scratch the integrated
// picture isn't settled. Revisit when both the wider-numerator divide
// scratch and the AGM-scale `newton_vs_mg` evidence line up.

/// Max `u64` limbs for the `10^SCALE` (`pow_scale`) buffer. Covers the
/// D924 AGM scale at width 6144 (`w_prime ≤ 1850`, `pow_u64 ≤ 100`).
const MAX_POW_U64: usize = 104;
/// Max `u64` limbs for the reciprocal (`r`) buffer. Covers the
/// 6144 + D924 AGM cell (`k_u64 + 1 ≤ 197`).
const MAX_R_U64: usize = 200;
/// Max `u64` limbs for the magnitude / quotient buffers. Covers
/// the widest enabled width's magnitude (6144 → 96 u64), with margin.
const MAX_MAG_U64: usize = 128;
/// Max `u64` limbs for product / scratch buffers (`n·r`, `q·D`, …).
/// Covers `MAX_MAG_U64 + MAX_R_U64`.
const MAX_PROD_U64: usize = 332;

// -- u128-limb sibling sizes (packed pairs of u64) --------------------
//
// The cached `r_u128`/`pow_u128` mirror the u64 versions packed pairwise
// (`limb = lo | hi << 64`). All sizes are `ceil(u64_size/2)`.

/// Max `u128` limbs for the `10^scale` (`pow_u128`) buffer.
const MAX_POW_U128: usize = MAX_POW_U64.div_ceil(2);
/// Max `u128` limbs for the reciprocal (`r_u128`) buffer.
const MAX_R_U128: usize = MAX_R_U64.div_ceil(2);
/// Max `u128` limbs for the magnitude / quotient u128 buffers.
const MAX_MAG_U128: usize = MAX_MAG_U64.div_ceil(2);
/// Max `u128` limbs for product / scratch u128 buffers.
const MAX_PROD_U128: usize = MAX_PROD_U64.div_ceil(2);

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

    // -- u128-packed mirrors of `r` and `pow_scale` -------------------
    //
    // Populated once at the end of `precompute` by pairwise packing the
    // u64 limbs above (`limb = lo | hi << 64`). The u128 Newton kernel
    // (`div_newton_u128`) consumes these directly with NO per-call pack,
    // recovering the v0.4.4 u128-slice mul throughput at the cost of one
    // extra pack pass during the (already amortised) precompute.
    /// Reciprocal limbs packed as u128 pairs, live count `r_u128_len`.
    r_u128: [u128; MAX_R_U128],
    /// Live limb count of `r_u128` = `r_len.div_ceil(2)`.
    r_u128_len: usize,
    /// Right-shift amount in u128 limbs (`k_u64 / 2`). `precompute`
    /// rounds `k_u64` UP to even so this shift is limb-aligned in u128.
    k_u128: usize,
    /// `10^scale` packed as u128 pairs, live count `pow_u128_len`.
    pow_u128: [u128; MAX_POW_U128],
    /// Live limb count of `pow_u128` = `pow_len.div_ceil(2)`.
    pow_u128_len: usize,
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
        //
        // Round UP to even so the u128-packed mirror right-shift is
        // limb-aligned in u128 (`k_u128 = k_u64 / 2`). Adding one to k
        // grows R by a single limb (over-estimate); the Newton add-back
        // correction absorbs the +1 ULP. The u64 kernel sees the bumped
        // `k_u64` too — bit-identical correction either way.
        let k_u64_raw = width_limbs + pow_len;
        let k_u64 = if k_u64_raw % 2 == 0 { k_u64_raw } else { k_u64_raw + 1 };

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

        // -- u128-packed mirrors ----------------------------------
        //
        // Pack the u64 limbs of `r` and `pow_scale` pairwise into u128
        // limbs (`limb = lo | hi << 64`). Buffers are zero-initialised,
        // so packing the rounded-even live count is safe (any odd live
        // tail of the u64 buffer pairs with a zeroed neighbour).
        let r_len = k_u64 + 1;
        let r_u128_len = r_len.div_ceil(2);
        let pow_u128_len = pow_len.div_ceil(2);

        let mut r_u128 = [0u128; MAX_R_U128];
        let mut i = 0;
        while i < r_u128_len {
            let lo = r[2 * i] as u128;
            let hi = r[2 * i + 1] as u128;
            r_u128[i] = lo | (hi << 64);
            i += 1;
        }
        let mut pow_u128 = [0u128; MAX_POW_U128];
        let mut i = 0;
        while i < pow_u128_len {
            let lo = pow_scale[2 * i] as u128;
            let hi = pow_scale[2 * i + 1] as u128;
            pow_u128[i] = lo | (hi << 64);
            i += 1;
        }
        let k_u128 = k_u64 / 2;

        Self {
            r,
            r_len,
            k_u64,
            pow_scale,
            pow_len,
            r_u128,
            r_u128_len,
            k_u128,
            pow_u128,
            pow_u128_len,
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

// -- u128-packed Newton kernel ----------------------------------------
//
// Mirrors `div_newton` but operates entirely on packed u128 limb slices.
// The cached `r_u128`/`pow_u128` are consumed directly with NO per-call
// pack/unpack; the per-call operand (n) and output (quot, rem) all stay
// in u128 throughout. Recovers the v0.4.4 `limbs_mul`-style throughput
// (half the limb count, ~1/4 the partial products per schoolbook) by
// paying the pack cost ONCE in the amortised `precompute`.

#[inline]
const fn cmp_u128(a: &[u128], b: &[u128]) -> i32 {
    let mut alen = a.len();
    while alen > 0 && a[alen - 1] == 0 { alen -= 1; }
    let mut blen = b.len();
    while blen > 0 && b[blen - 1] == 0 { blen -= 1; }
    if alen != blen { return if alen > blen { 1 } else { -1 }; }
    let mut i = alen;
    while i > 0 {
        i -= 1;
        if a[i] != b[i] { return if a[i] > b[i] { 1 } else { -1 }; }
    }
    0
}

#[inline]
const fn sub_assign_u128(a: &mut [u128], b: &[u128]) -> bool {
    let mut borrow: u128 = 0;
    let mut i = 0;
    while i < a.len() {
        let bi = if i < b.len() { b[i] } else { 0 };
        let (s1, c1) = a[i].overflowing_sub(bi);
        let (s2, c2) = s1.overflowing_sub(borrow);
        a[i] = s2;
        borrow = (c1 as u128) | (c2 as u128);
        i += 1;
    }
    borrow != 0
}

/// `out = a * b` schoolbook on u128 limb slices. Inner step uses the
/// 4xu64*u64->u128 partials decomposition (`<u128 as Limb>::widening_mul`).
#[inline]
fn mul_schoolbook_u128(a: &[u128], b: &[u128], out: &mut [u128]) {
    use crate::int::types::compute_limbs::Limb;
    let mut i = 0;
    while i < a.len() {
        if a[i] != 0 {
            let mut carry: u128 = 0;
            let mut j = 0;
            while j < b.len() {
                if b[j] != 0 || carry != 0 {
                    let (prod_lo, prod_hi) = <u128 as Limb>::widening_mul(a[i], b[j]);
                    let idx = i + j;
                    let (s1, c1) = out[idx].overflowing_add(prod_lo);
                    let (s2, c2) = s1.overflowing_add(carry);
                    out[idx] = s2;
                    carry = prod_hi.wrapping_add(c1 as u128).wrapping_add(c2 as u128);
                }
                j += 1;
            }
            let mut idx = i + b.len();
            while carry != 0 && idx < out.len() {
                let (s, c) = out[idx].overflowing_add(carry);
                out[idx] = s;
                carry = c as u128;
                idx += 1;
            }
        }
        i += 1;
    }
}

/// Per-call Newton-reciprocal divide, u128-packed sibling of `div_newton`.
/// All multiplies run on the cached u128-packed `r` and `pow_scale` (NO
/// per-call pack); operand/output stay in u128 throughout.
fn div_newton_u128(
    n: &[u128],
    table: &NewtonReciprocal,
    quot: &mut [u128],
    rem_out: &mut [u128],
) -> usize {
    let r = &table.r_u128[..table.r_u128_len];
    let pow_scale = &table.pow_u128[..table.pow_u128_len];

    let prod_len = n.len() + r.len();
    debug_assert!(prod_len <= MAX_PROD_U128, "u128 product buffer too small");
    let mut prod = [0u128; MAX_PROD_U128];
    mul_schoolbook_u128(n, r, &mut prod[..prod_len]);

    let lo = table.k_u128.min(prod_len);
    let q_slice = &prod[lo..prod_len];
    for (dst, src_) in quot.iter_mut().zip(q_slice.iter()) { *dst = *src_; }
    for dst in quot.iter_mut().skip(q_slice.len()) { *dst = 0; }

    let prod2_len = quot.len() + pow_scale.len();
    debug_assert!(prod2_len <= MAX_PROD_U128, "u128 product buffer too small");
    let mut prod2 = [0u128; MAX_PROD_U128];
    mul_schoolbook_u128(quot, pow_scale, &mut prod2[..prod2_len]);

    let rem_len = n.len() + 1;
    debug_assert!(rem_len <= MAX_MAG_U128 + 1, "u128 rem buffer too small");
    for (dst, src_) in rem_out.iter_mut().take(rem_len).zip(n.iter()) { *dst = *src_; }
    rem_out[rem_len - 1] = 0;
    let sub_len = prod2_len.min(rem_len);
    let _ = sub_assign_u128(&mut rem_out[..sub_len], &prod2[..sub_len]);

    loop {
        if cmp_u128(&rem_out[..rem_len], pow_scale) < 0 { break; }
        let s = rem_len.min(pow_scale.len());
        let _ = sub_assign_u128(&mut rem_out[..s], &pow_scale[..s]);
        let mut carry: u128 = 1;
        for limb in quot.iter_mut() {
            let (s, c) = limb.overflowing_add(carry);
            *limb = s;
            if !c { carry = 0; break; }
        }
        let _ = carry;
    }

    rem_len
}

/// Width-agnostic Newton-reciprocal divide of a u128 magnitude slice by
/// `10^scale`, in place, with `mode`-aware rounding. Operates entirely
/// in u128 limbs (NO transcoding to/from u64). Bit-identical to the u64
/// path `newton_pow10_mag_u128`.
pub(crate) fn newton_pow10_mag_u128_packed(
    mag_u128: &mut [u128],
    neg: bool,
    mode: crate::support::rounding::RoundingMode,
    table: &NewtonReciprocal,
) {
    use crate::support::rounding;

    let mag_len = mag_u128.len();
    let mut top = mag_len;
    while top > 0 && mag_u128[top - 1] == 0 { top -= 1; }

    let n_slice = &mag_u128[..top.max(1)];
    let mut quot = [0u128; MAX_MAG_U128];
    let mut rem = [0u128; MAX_MAG_U128 + 1];
    let rem_len = div_newton_u128(n_slice, table, &mut quot[..mag_len], &mut rem);

    let rem_is_zero = rem[..rem_len].iter().all(|&x| x == 0);
    if !rem_is_zero {
        let pow_len = table.pow_u128_len;
        let mut half = [0u128; MAX_POW_U128];
        half[..pow_len].copy_from_slice(&table.pow_u128[..pow_len]);
        let mut i = pow_len;
        let mut carry_in: u128 = 0;
        while i > 0 {
            i -= 1;
            let next_carry = half[i] & 1;
            half[i] = (carry_in << 127) | (half[i] >> 1);
            carry_in = next_carry;
        }

        let cmp_r = match cmp_u128(&rem[..rem_len], &half[..pow_len]) {
            n if n < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        };
        let q_is_odd = (quot[0] & 1) != 0;
        if rounding::should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u128 = 1;
            for limb in quot[..mag_len].iter_mut() {
                let (s, c) = limb.overflowing_add(carry);
                *limb = s;
                if !c { carry = 0; break; }
            }
            let _ = carry;
        }
    }

    for (i, slot) in mag_u128.iter_mut().enumerate() { *slot = quot[i]; }
}

/// Per-width Limb-axis matcher: does the cached `(width_bits, scale)`
/// cell run the u128-packed Newton kernel? Continuous width region per
/// Constitution rule 6 + Class I (never a per-scale carve-out).
///
/// 2026-05-28 audit extension: the u128-packed kernel wins at the
/// existing 1536–4096 band and at the new 6144 width (`newton_vs_mg`
/// integrated bench, cores 22–23, sees u128 1.18–3.46× over MG and
/// 1.0–1.24× over u64 across s115–s953).
///
/// Wider widths (8192/12288/16384/32768) stay on MG entirely — see
/// [`newton_wins`] for the AGM-widening / buffer-scratch / contradicting
/// integrated-bench reasons.
#[inline]
const fn newton_u128_wins(width_bits: u32) -> bool {
    matches!(width_bits, 1536 | 2048 | 3072 | 4096 | 6144)
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
    //
    // Buffer sized to 256 u128 limbs to fit the widest tier exercised
    // (`Int<512>` = 32768-bit, 256 u128 limbs). The kernel is sliced
    // to `W::U128_LIMBS` so narrower widths don't pay the wide cost.
    let mut mag_u128 = [0u128; 256];
    let limbs = <W as crate::int::types::traits::BigInt>::U128_LIMBS;
    let mag = &mut mag_u128[..limbs];
    let neg = n.mag_into_u128(mag);
    newton_pow10_mag_u128(mag, neg, mode, table);
    W::from_mag_sign_u128(mag, neg)
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
/// | bits  | Storage match                       | Newton min SCALE |
/// |-------|-------------------------------------|------------------|
/// | 1536  | Int<24>  (D462 stg / D230 Work)     |  ≥ 200           |
/// | 2048  | Int<32>  (D616 stg / D307 Work)     |  ≥ 200           |
/// | 3072  | Int<48>  (D924 stg / D462 Work)     |  ≥ 200           |
/// | 4096  | Int<64>  (D1232 stg / D616 Work)    |  ≥ 400           |
/// | 6144  | Int<96>  (D230 Wexp / D924 Work)    |  ≥ 200           |
///
/// Bench source: `benches/micro/newton_vs_mg.rs` head-to-head against
/// [`crate::algos::support::mg_divide::div_wide_pow10_chain`] at the
/// listed widths × representative SCALE bands.
///
/// Widths 8192 / 12288 / 16384 / 32768 (D462 Wexp / D1232 Work / D924
/// Wide / D616 Wexp / D924 Wexp / D1232 Wide / D1232 Wexp) are
/// **deferred**. Two-part reason:
///
/// 1. **AGM widening invalidates the simple max-scale analysis.** The
///    `_strict_agm` ln/exp paths lift the working scale to
///    `w_prime = SCALE + GUARD + guard_agm(SCALE) ≈ 2·SCALE + 4` at
///    high SCALE — so D924 Wide=12288 actually sees `w ≈ 1850` (not
///    953), D1232 Work=8192 actually sees `w ≈ 2466` (not 1261). The
///    Newton precompute's `2^k / 10^scale` numerator at those scales
///    needs more u64 limbs than the routed `div_knuth`'s build-max
///    scratch (`MAX_SINGLE_LIMBS = 258`) can hold: 12288 + AGM scale
///    1850 → ~293 num limbs, 8192 + AGM scale 2466 → ~260 num limbs.
///    The first golden run after enabling 8192/12288 panicked
///    `div_knuth.rs:81` with the AGM-scale numerator. 6144 is the
///    widest width whose AGM-scale numerator (~197 limbs) still fits.
/// 2. **Sibling-agent integrated bench (atanh diagnosis) at
///    Int<192>/Int<256>** reported Newton losing 5–58× to MG chain in
///    the low-scale shape (e.g. w=38 → MG 2.63 µs vs Newton 109 µs).
///    The crossover may exist but is structurally higher than at the
///    narrower widths, and the picture isn't yet settled.
///
/// Revisit when (a) a wider-numerator divide kernel is wired into the
/// Newton precompute (Knuth-into with caller-sized scratch), and (b)
/// the integrated `newton_vs_mg` evidence at the AGM scale (not just
/// the SCALE-band) confirms Newton wins.
///
/// The 1536-bit row recovers the bench-branch-compare wide-`÷10^SCALE`
/// regression at D230's work width: `exp_D230_s{172,229}` (1.38× and
/// 1.61× cells) rescale at Int<24>=1536 bits with working scale
/// `w = SCALE + GUARD(30) ∈ {202, 259}`. The newton_vs_mg micro at
/// 1536 bits puts the Newton crossover between s190 (MG 1.10×) and
/// s195 (Newton 1.03×), with Newton 1.05–1.51× across s202..461 — so
/// the conservative `≥ 200` threshold (same value the other rows use)
/// covers the bbc cells with margin and avoids snapping to either edge.
/// The same 1536-bit divide is shared with D462's storage `mul`/`div`
/// rescale fast-path (where the work width also lands at 1536 bits)
/// and D230's storage `mul` slow-path — see `mul_widen_divide`.
///
/// The 6144-bit row (audit 2026-05-28) covers the D230 `exp`/`atanh`
/// Wexp band and the D924 Work integer's transcendental rescales.
/// Bench evidence (cores 22–23, integrated `newton_vs_mg`):
///
/// | scale | mg_chain   | newton     | newton_u128 | best win |
/// |-------|------------|------------|-------------|----------|
/// | 38    | 1.17 µs MG | 2.68 µs    | 2.70 µs     | MG 2.3×  |
/// | 115   | 3.06 µs    | 2.99 µs    | 2.60 µs     | u128 1.18× |
/// | 200   | 4.75 µs    | 3.27 µs    | 2.92 µs     | u128 1.63× |
/// | 400   | 8.13 µs    | 4.44 µs    | 4.15 µs     | u128 1.96× |
/// | 600   | 10.67 µs   | 4.87 µs    | 3.90 µs     | u128 2.73× |
/// | 800   | 12.36 µs   | 4.75 µs    | 4.34 µs     | u128 2.85× |
/// | 953   | 13.90 µs   | 4.97 µs    | 4.02 µs     | u128 3.46× |
///
/// The crossover is between s38 (MG wins, single-pass) and s115 (Newton
/// edges). The `≥ 200` threshold matches every other 0.5.0 row, keeps
/// the gate on a continuous region per Constitution rule 6 + Class I,
/// and avoids snapping to a specific bbc cell.
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
        1536 if scale >= 200 => true,
        2048 if scale >= 200 => true,
        3072 if scale >= 200 => true,
        4096 if scale >= 400 => true,
        6144 if scale >= 200 => true,
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
/// One slot per cached width — because the `width_limbs` argument
/// differs (24 / 32 / 48 / 64 u64 limbs for
/// Int<24> / Int<32> / Int<48> / Int<64>) and the `NewtonReciprocal`
/// allocates limb-storage sized to that argument.
#[cfg(feature = "std")]
mod cache {
    use super::NewtonReciprocal;
    use ::std::thread_local;

    thread_local! {
        static C_1536: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
            ::core::cell::RefCell::new(alloc::vec::Vec::new())
        };
        static C_2048: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
            ::core::cell::RefCell::new(alloc::vec::Vec::new())
        };
        static C_3072: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
            ::core::cell::RefCell::new(alloc::vec::Vec::new())
        };
        static C_4096: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
            ::core::cell::RefCell::new(alloc::vec::Vec::new())
        };
        // Wider widths added 2026-05-28 — the wide-tier transcendental
        // work integers covered by the buffer-raise. Currently 6144
        // only; 8192/12288/16384/32768 are deferred (see
        // [`super::newton_wins`] for the AGM-widening / scratch /
        // integrated-bench reasons).
        // - 6144 = Int<96> (D230 Wexp / D924 Work)
        static C_6144: ::core::cell::RefCell<alloc::vec::Vec<(u32, NewtonReciprocal)>> = const {
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
            1536 => &C_1536,
            2048 => &C_2048,
            3072 => &C_3072,
            4096 => &C_4096,
            6144 => &C_6144,
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
    W: crate::int::types::traits::BigInt,
    W::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let bits = <W as crate::int::types::traits::BigInt>::BITS;
    // u128 magnitude buffer from `W`'s scratch carrier (`W::Scratch =
    // Limbs<N>`, size lives in the impl); no const work-width parameter —
    // same mechanism as `div_wide_pow10`.
    let mut buf = <W::Scratch as crate::int::types::compute_limbs::ComputeLimbs>::single_u128();
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
            if newton_u128_wins(width_bits) {
                newton_pow10_mag_u128_packed(mag, neg, mode, table);
            } else {
                newton_pow10_mag_u128(mag, neg, mode, table);
            }
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

    // Exercises `Int<16>` (D307 storage); the divide scratch is sized by the
    // build's `MAX_WORK_N`, so this only runs where that tier is enabled.
    #[cfg(feature = "d307")]
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

    // Exercises `Int<24>` (D462 storage AND D230 Work) at the bbc anchor
    // scales — bit-identical agreement is the validity wall for the new
    // 1536-bit `newton_wins` entry.
    #[cfg(feature = "d462")]
    #[test]
    fn newton_matches_mg_chain_d462_s202() {
        let scale = 202u32;
        let width_limbs = 24; // Int<24> = 12 u128 = 24 u64 limbs
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[10] = 1u128 << 24;
        limbs[2] = 0xfeedfacecafef00d_u128;
        let n = <Int<24> as crate::int::types::traits::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain::<Int<24>>(n, scale, RoundingMode::HalfToEven);
        assert_eq!(got, want, "Newton differs from MG chain at Int<24> s=202");
    }

    #[cfg(feature = "d462")]
    #[test]
    fn newton_matches_mg_chain_d462_s259() {
        let scale = 259u32;
        let width_limbs = 24;
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[10] = 1u128 << 8;
        limbs[1] = 0xdeadbeef_cafef00d_u128;
        let n = <Int<24> as crate::int::types::traits::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain::<Int<24>>(n, scale, RoundingMode::HalfToEven);
        assert_eq!(got, want, "Newton differs from MG chain at Int<24> s=259");
    }

    // Exercises `Int<32>` (D616 storage) — runs only where that tier is on.
    #[cfg(feature = "d616")]
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

    // Exercises `Int<64>` (D1232 storage) — runs only where that tier is on.
    #[cfg(feature = "d1232")]
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

    // -- u64-vs-u128 Newton bit-identity (validity wall) ---------------
    //
    // The u128-packed kernel MUST produce limb-identical output to the
    // u64 kernel for every supported width x representative scale, in
    // every rounding mode. Any divergence is a kernel bug.

    fn assert_u64_u128_match(
        scale: u32,
        width_limbs: usize,
        mag_limbs: usize,
        top_limb_idx: usize,
        top_limb_val: u128,
        low_perturbation: (usize, u128),
    ) {
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let modes = [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ];

        for mode in modes {
            // Buffer sized to 128 u128 limbs to fit Int<192>=96 u128
            // (the widest mag the audit-extended widths exercise).
            let mut mag_a = [0u128; 128];
            mag_a[top_limb_idx] = top_limb_val;
            mag_a[low_perturbation.0] = low_perturbation.1;
            let mut mag_b = mag_a;

            super::newton_pow10_mag_u128(&mut mag_a[..mag_limbs], false, mode, &table);
            super::newton_pow10_mag_u128_packed(&mut mag_b[..mag_limbs], false, mode, &table);
            assert_eq!(
                mag_a, mag_b,
                "u64 != u128 Newton at scale={scale} width={width_limbs} mode={mode:?}"
            );

            let mut mag_a = [0u128; 128];
            mag_a[top_limb_idx] = top_limb_val;
            mag_a[low_perturbation.0] = low_perturbation.1;
            let mut mag_b = mag_a;
            super::newton_pow10_mag_u128(&mut mag_a[..mag_limbs], true, mode, &table);
            super::newton_pow10_mag_u128_packed(&mut mag_b[..mag_limbs], true, mode, &table);
            assert_eq!(
                mag_a, mag_b,
                "u64 != u128 Newton (neg) at scale={scale} width={width_limbs} mode={mode:?}"
            );
        }
    }

    #[cfg(feature = "d307")]
    #[test]
    fn newton_u64_eq_u128_d307_s150() {
        assert_u64_u128_match(150, 16, 8, 6, 1u128 << 32, (1, 0xdeadbeef_cafef00d_u128));
    }

    #[cfg(feature = "d307")]
    #[test]
    fn newton_u64_eq_u128_d307_s307() {
        assert_u64_u128_match(307, 16, 8, 7, 0x1234_5678_9abc_def0u128, (0, 1));
    }

    // Int<24> = 1536-bit. The PRODUCTION row: D462 storage, D230 Work.
    #[cfg(feature = "d462")]
    #[test]
    fn newton_u64_eq_u128_b1536_s200() {
        assert_u64_u128_match(200, 24, 12, 10, 1u128 << 24, (2, 0xfeedfacecafef00d_u128));
    }

    #[cfg(feature = "d462")]
    #[test]
    fn newton_u64_eq_u128_b1536_s202() {
        assert_u64_u128_match(202, 24, 12, 10, 1u128 << 24, (2, 0xfeedfacecafef00d_u128));
    }

    #[cfg(feature = "d462")]
    #[test]
    fn newton_u64_eq_u128_b1536_s259() {
        assert_u64_u128_match(259, 24, 12, 10, 1u128 << 8, (1, 0xdeadbeef_cafef00d_u128));
    }

    #[cfg(feature = "d462")]
    #[test]
    fn newton_u64_eq_u128_b1536_s461() {
        assert_u64_u128_match(461, 24, 12, 11, 0x1u128, (3, 0xfacefacef00d_u128));
    }

    #[cfg(feature = "d616")]
    #[test]
    fn newton_u64_eq_u128_d616_s308() {
        assert_u64_u128_match(308, 32, 16, 14, 1u128 << 16, (3, 0xdeadbeef));
    }

    #[cfg(feature = "d616")]
    #[test]
    fn newton_u64_eq_u128_d616_s616() {
        assert_u64_u128_match(616, 32, 16, 15, 1u128 << 8, (4, 0xfeedface));
    }

    #[cfg(feature = "d924")]
    #[test]
    fn newton_u64_eq_u128_d924_s460() {
        assert_u64_u128_match(460, 48, 24, 22, 1u128 << 8, (5, 0xcafef00d));
    }

    #[cfg(feature = "d924")]
    #[test]
    fn newton_u64_eq_u128_d924_s924() {
        assert_u64_u128_match(924, 48, 24, 23, 1u128, (6, 0xfeedfacef00d));
    }

    #[cfg(feature = "d1232")]
    #[test]
    fn newton_u64_eq_u128_d1232_s615() {
        assert_u64_u128_match(615, 64, 32, 30, 1u128 << 8, (5, 0xcafef00d));
    }

    #[cfg(feature = "d1232")]
    #[test]
    fn newton_u64_eq_u128_d1232_s1231() {
        assert_u64_u128_match(1231, 64, 32, 31, 1u128, (7, 0xdeadbeef_feedface_u128));
    }

    // ── Wider-width validity wall (audit 2026-05-28) ──────────────────
    //
    // Int<96> / Int<128> / Int<192> exercised at the bbc-anchor scales
    // and the maxima. Bit-identical agreement with the Knuth-routed
    // `div_rem_mag_slice` path is the validity wall for the new
    // `newton_wins` (6144 / 8192 / 12288) entries.

    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    #[test]
    fn newton_matches_mg_chain_b6144_s200() {
        let scale = 200u32;
        let width_limbs = 96; // Int<96> = 48 u128 = 96 u64
        let table = NewtonReciprocal::precompute(scale, width_limbs);
        let mut limbs = [0u128; 64];
        limbs[40] = 1u128 << 24;
        limbs[3] = 0xfeedfacecafef00d_u128;
        let n = <Int<96> as crate::int::types::traits::BigInt>::from_mag_sign_u128(&limbs, false);
        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain::<Int<96>>(n, scale, RoundingMode::HalfToEven);
        assert_eq!(got, want, "Newton differs from MG chain at Int<96> s=200");
    }

    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    #[test]
    fn newton_matches_mg_chain_b6144_s953() {
        let scale = 953u32;
        let width_limbs = 96;
        let table = NewtonReciprocal::precompute(scale, width_limbs);
        let mut limbs = [0u128; 64];
        limbs[46] = 1u128 << 8;
        limbs[1] = 0xdeadbeef_cafef00d_u128;
        let n = <Int<96> as crate::int::types::traits::BigInt>::from_mag_sign_u128(&limbs, false);
        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain::<Int<96>>(n, scale, RoundingMode::HalfToEven);
        assert_eq!(got, want, "Newton differs from MG chain at Int<96> s=953");
    }

    // u64 vs u128 bit-identity at the new 6144 width covered by
    // `newton_u128_wins` — production-shape cells across the D924
    // SCALE band AND the AGM-widened scales the strict_agm transcendentals
    // exercise.

    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    #[test]
    fn newton_u64_eq_u128_b6144_s200() {
        assert_u64_u128_match(200, 96, 48, 40, 1u128 << 24, (3, 0xfeedfacecafef00d_u128));
    }

    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    #[test]
    fn newton_u64_eq_u128_b6144_s953() {
        assert_u64_u128_match(953, 96, 48, 46, 1u128 << 8, (1, 0xdeadbeef_cafef00d_u128));
    }

    // AGM-band cells — D924 strict_agm runs at `w_prime ≤ 1850`.
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    #[test]
    fn newton_u64_eq_u128_b6144_s1234() {
        assert_u64_u128_match(1234, 96, 48, 46, 1u128 << 16, (2, 0xcafef00dbeef_u128));
    }

    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    #[test]
    fn newton_u64_eq_u128_b6144_s1850() {
        assert_u64_u128_match(1850, 96, 48, 47, 1u128, (5, 0xfacefacef00d_u128));
    }
}
