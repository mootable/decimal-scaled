//! Newton–Raphson reciprocal divide for `n / 10^SCALE` at storage width.
//!
//! Research kernel — **not wired into the dispatcher**. Built behind a
//! `pub(crate)` API so micro-benches can compare it head-to-head against
//! [`crate::algos::mg_divide::div_wide_pow10_chain_with`].
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
//! `R` is computed once per `(SCALE, width)` pair via the existing
//! [`crate::wide_int::limbs_divmod_dispatch`] routine. Setup cost is one
//! wide divide; per-call cost is one wide multiply + one narrow
//! multiply + one comparison + one optional subtract.
//!
//! # Reference
//!
//! Granlund, T. & Montgomery, P. L. (1994). *Division by Invariant
//! Integers using Multiplication*, PLDI '94. Möller, N. & Granlund, T.
//! (2011). *Improved Division by Invariant Integers*, IEEE TC 60(2).
//! The Newton-iteration view of the same reciprocal is
//! Wikipedia — [Division algorithm § Newton–Raphson division](https://en.wikipedia.org/wiki/Division_algorithm#Newton%E2%80%93Raphson_division).

// `Vec` / `vec!` come from the prelude under `std`; on `no_std + alloc`
// they must be imported explicitly. Gated so the std prelude path is
// unaffected (no shadowing, no unused-import warning).
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use crate::wide_int::{limbs_divmod_dispatch, limbs_mul, limbs_sub_assign};

/// Pre-computed reciprocal table for a single `(SCALE, mag_width)` pair.
///
/// `r` is the reciprocal `floor(2^k / 10^SCALE)` in little-endian
/// u128 limbs; `k_limbs` is `k / 128` (we always pick `k` as a
/// multiple of 128 so the shift is a limb-aligned slice).
///
/// `pow_scale` is `10^SCALE` in little-endian u128 limbs, kept for the
/// correction step.
#[derive(Clone)]
pub struct NewtonReciprocal {
    /// Reciprocal limbs (little-endian).
    pub r: Vec<u128>,
    /// Right-shift amount in u128 limbs (so quotient = (n*r) limbs >> k_limbs words).
    pub k_limbs: usize,
    /// `10^SCALE` limbs (little-endian).
    pub pow_scale: Vec<u128>,
}

impl NewtonReciprocal {
    /// Compute reciprocal table for `D = 10^scale` at the given
    /// magnitude width (in u128 limbs).
    ///
    /// `width_limbs` is the upper bound on the numerator magnitude's
    /// limb count.
    pub fn precompute(scale: u32, width_limbs: usize) -> Self {
        // pow_scale = 10^scale via repeated *10 on a wide buffer.
        // Width: enough to hold 10^scale (ceil(scale * log2(10) / 128) limbs)
        // plus headroom. We allocate `width_limbs` to keep limb counts uniform.
        // 10^scale needs about scale * log2(10) ≈ scale * 3.322 bits.
        // 10^38 < 2^127, so each u128 limb absorbs at most 38 decimal digits.
        // Use scale/38 + 1 limbs plus 1 for headroom during *10 carry.
        let pow_limbs = (scale as usize / 38 + 2).max(1);
        let mut pow_scale = vec![0u128; pow_limbs];
        pow_scale[0] = 1u128;
        for _ in 0..scale {
            // multiply pow_scale by 10
            let mut carry: u128 = 0;
            for limb in pow_scale.iter_mut() {
                // 128x64 multiply: (limb * 10) + carry, with carry propagation
                let (lo, hi) = mul_u128_by_u64(*limb, 10);
                let (sum_lo, c1) = lo.overflowing_add(carry);
                *limb = sum_lo;
                carry = hi + u128::from(c1);
            }
            debug_assert_eq!(carry, 0, "pow_scale buffer too small at scale={scale}");
        }

        // Pick k_limbs: we need quotient room of `width_limbs` u128 limbs.
        // Set k = 128 * (width_limbs + pow_limbs) bits — then
        // R = 2^k / 10^scale has bit-length about k - bits(10^scale),
        // and (n * R) >> k yields a width_limbs-wide quotient with
        // at most 1 ULP error.
        let k_limbs = width_limbs + pow_limbs;

        // numerator = 2^(128 * k_limbs) — a single 1 in limb position k_limbs.
        let mut num = vec![0u128; k_limbs + 1];
        num[k_limbs] = 1u128;

        // divide num by pow_scale to get r.
        let mut r = vec![0u128; k_limbs + 1];
        let mut rem = vec![0u128; pow_limbs + 1];
        limbs_divmod_dispatch(&num, &pow_scale, &mut r, &mut rem);

        // Trim trailing zeros on r for cleanliness (but keep capacity).
        Self { r, k_limbs, pow_scale }
    }
}

#[inline]
fn mul_u128_by_u64(a: u128, b: u64) -> (u128, u128) {
    let a_lo = a as u64 as u128;
    let a_hi = a >> 64;
    let b = b as u128;
    let lo_full = a_lo * b;
    let hi_full = a_hi * b;
    let lo_full_hi = lo_full >> 64;
    let mid = hi_full + lo_full_hi;
    let lo = (lo_full as u64 as u128) | ((mid as u64 as u128) << 64);
    let hi = mid >> 64;
    (lo, hi)
}

/// Per-call Newton-reciprocal divide: returns `floor(n / 10^scale)`.
///
/// `n` is the unsigned numerator magnitude in little-endian u128 limbs.
/// Output `q` is written into `quot` (caller-sized to `width_limbs`),
/// and the remainder is returned packed into a `Vec<u128>` for
/// rounding-aware callers.
///
/// # Precision
///
/// Strict: the result is bit-exact `floor(n / 10^scale)`. The Newton
/// add-back step ensures correctness for the at-most-1 over/under
/// estimate the truncated reciprocal produces.
pub fn div_newton(
    n: &[u128],
    table: &NewtonReciprocal,
    quot: &mut [u128],
) -> Vec<u128> {
    // product = n * r
    let prod_len = n.len() + table.r.len();
    let mut prod = vec![0u128; prod_len];
    limbs_mul(n, &table.r, &mut prod);

    // q_approx = prod >> (128 * k_limbs)
    let lo = table.k_limbs.min(prod.len());
    let q_slice = &prod[lo..];
    for (dst, src) in quot.iter_mut().zip(q_slice.iter()) {
        *dst = *src;
    }
    for dst in quot.iter_mut().skip(q_slice.len()) {
        *dst = 0;
    }

    // r_approx = n - q_approx * pow_scale  (mod 2^(width))
    let prod2_len = quot.len() + table.pow_scale.len();
    let mut prod2 = vec![0u128; prod2_len];
    limbs_mul(quot, &table.pow_scale, &mut prod2);

    // Compute remainder = n - prod2 in n.len()+1 limbs.
    let mut rem = vec![0u128; n.len() + 1];
    for (dst, src) in rem.iter_mut().zip(n.iter()) {
        *dst = *src;
    }
    // Truncate prod2 to rem's width for subtraction.
    let sub_len = prod2.len().min(rem.len());
    let _ = limbs_sub_assign(&mut rem[..sub_len], &prod2[..sub_len]);

    // Correction loop: while rem >= pow_scale, bump quotient by 1
    // and decrement remainder. With a correctly-sized k_limbs the
    // loop runs at most once or twice.
    loop {
        // Compare rem vs pow_scale.
        let cmp = cmp_limbs(&rem, &table.pow_scale);
        if cmp == core::cmp::Ordering::Less {
            break;
        }
        // rem -= pow_scale
        let sub_len = rem.len().min(table.pow_scale.len());
        let _ = limbs_sub_assign(&mut rem[..sub_len], &table.pow_scale[..sub_len]);
        // quot += 1
        let mut carry: u128 = 1;
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

    rem
}

fn cmp_limbs(a: &[u128], b: &[u128]) -> core::cmp::Ordering {
    // Compare little-endian limb slices as unsigned integers.
    let mut a_top = a.len();
    while a_top > 0 && a[a_top - 1] == 0 {
        a_top -= 1;
    }
    let mut b_top = b.len();
    while b_top > 0 && b[b_top - 1] == 0 {
        b_top -= 1;
    }
    if a_top != b_top {
        return a_top.cmp(&b_top);
    }
    let mut i = a_top;
    while i > 0 {
        i -= 1;
        match a[i].cmp(&b[i]) {
            core::cmp::Ordering::Equal => continue,
            ord => return ord,
        }
    }
    core::cmp::Ordering::Equal
}

/// Full `n / 10^SCALE` with rounding for a `BigInt`-backed value.
///
/// Direct analogue of [`crate::algos::mg_divide::div_wide_pow10_chain_with`]
/// — same signature, same semantics, different inner algorithm.
pub(crate) fn div_wide_pow10_newton_with<W: crate::wide_int::BigInt>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
    table: &NewtonReciprocal,
) -> W {
    use crate::support::rounding;

    let mut mag = [0u128; 64];
    let neg = n.mag_into_u128(&mut mag);
    let mut top = mag.len();
    while top > 0 && mag[top - 1] == 0 {
        top -= 1;
    }

    let n_slice = &mag[..top.max(1)];
    let mut quot = vec![0u128; mag.len()];
    let rem = div_newton(n_slice, table, &mut quot);

    // Round per `mode`: compare remainder with pow_scale / 2.
    let rem_is_zero = rem.iter().all(|&x| x == 0);
    if !rem_is_zero {
        // half = pow_scale / 2 (pow_scale is even for scale >= 1)
        let mut half = table.pow_scale.clone();
        // shift right by 1
        let mut i = half.len();
        let mut carry_in: u128 = 0;
        while i > 0 {
            i -= 1;
            let next_carry = half[i] & 1;
            half[i] = (carry_in << 127) | (half[i] >> 1);
            carry_in = next_carry;
        }

        let cmp_r = cmp_limbs(&rem, &half);
        let q_is_odd = (quot[0] & 1) != 0;
        if rounding::should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u128 = 1;
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
    }

    // Copy quot into a fixed-size buffer for from_mag_sign_u128.
    let mut out = [0u128; 64];
    for (dst, src) in out.iter_mut().zip(quot.iter()) {
        *dst = *src;
    }
    W::from_mag_sign_u128(&out, neg)
}

/// Width-keyed dispatch decision for `n / 10^SCALE`.
///
/// Returns `true` when the bench-validated Newton-vs-MG matrix says
/// Newton wins for this `(width_bits, scale)` cell. The matrix:
///
/// | Storage  | bits | Newton min SCALE |
/// |----------|------|------------------|
/// | I2048    | 2048 |  ≥ 200           |
/// | I3072    | 3072 |  ≥ 200           |
/// | I4096    | 4096 |  ≥ 400           |
///
/// Bench source: `benches/newton_vs_mg.rs` head-to-head against
/// [`crate::algos::mg_divide::div_wide_pow10_chain_with`] at the
/// listed widths × representative SCALE bands. Larger widths (Int8192
/// / Int12288 / Int16384 — used by the transcendental work integers)
/// have no bench data and fall through to MG.
///
/// Scale `≤ 38` always returns `false`: the single-pass MG kernel
/// `div_wide_pow10_with` is the chosen winner there and a chain-Newton
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
/// `width_limbs` argument differs (16 / 24 / 32 u128 limbs for
/// Int2048 / Int3072 / Int4096) and the `NewtonReciprocal` allocates
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
pub(crate) fn dispatch_wide_pow10_with<W: crate::wide_int::BigInt, const N: usize>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
) -> W {
    debug_assert_eq!(N, W::U128_LIMBS, "magnitude buffer must match W's u128-limb width");
    let bits = <W as crate::wide_int::BigInt>::BITS;
    if !newton_wins(bits, scale) {
        return crate::algos::mg_divide::div_wide_pow10_chain_with::<W, N>(n, scale, mode);
    }

    #[cfg(feature = "std")]
    {
        let width_limbs = (bits as usize) / 128;
        return cache::with_table(bits, scale, width_limbs, |table| {
            div_wide_pow10_newton_with::<W>(n, scale, mode, table)
        });
    }

    #[cfg(not(feature = "std"))]
    {
        // no_std fallback: no thread-local cache available; per-call
        // precompute is too costly for the wide tier (one Knuth divide
        // at storage width). Forward to MG instead — Newton wins
        // depend on amortising the table across many calls.
        crate::algos::mg_divide::div_wide_pow10_chain_with::<W, N>(n, scale, mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::mg_divide::div_wide_pow10_chain_with;
    use crate::support::rounding::RoundingMode;
    use crate::wide_int::{I1024, I2048, I4096};

    #[test]
    fn newton_matches_mg_chain_d307_s150() {
        let scale = 150u32;
        let width_limbs = 8;
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[6] = 1u128 << 32;
        limbs[0] = 42;
        let n = <I1024 as crate::wide_int::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain_with::<I1024, { <I1024 as crate::wide_int::BigInt>::U128_LIMBS }>(
            n,
            scale,
            RoundingMode::HalfToEven,
        );
        assert_eq!(got, want, "Newton differs from MG chain at D307 s=150");
    }

    #[test]
    fn newton_matches_mg_chain_d616_s308() {
        let scale = 308u32;
        let width_limbs = 16;
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[14] = 1u128 << 16;
        limbs[3] = 0xdeadbeef;
        let n = <I2048 as crate::wide_int::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain_with::<I2048, { <I2048 as crate::wide_int::BigInt>::U128_LIMBS }>(
            n,
            scale,
            RoundingMode::HalfToEven,
        );
        assert_eq!(got, want, "Newton differs from MG chain at D616 s=308");
    }

    #[test]
    fn newton_matches_mg_chain_d1232_s615() {
        let scale = 615u32;
        let width_limbs = 32;
        let table = NewtonReciprocal::precompute(scale, width_limbs);

        let mut limbs = [0u128; 64];
        limbs[30] = 1u128 << 8;
        limbs[5] = 0xcafef00d;
        let n = <I4096 as crate::wide_int::BigInt>::from_mag_sign_u128(&limbs, false);

        let got = div_wide_pow10_newton_with(n, scale, RoundingMode::HalfToEven, &table);
        let want = div_wide_pow10_chain_with::<I4096, { <I4096 as crate::wide_int::BigInt>::U128_LIMBS }>(
            n,
            scale,
            RoundingMode::HalfToEven,
        );
        assert_eq!(got, want, "Newton differs from MG chain at D1232 s=615");
    }
}
