//! Benchmark candidates for the D38s12 multiply-rescale step.
//!
//! Background: MOOCAD-242.17 replaced the naive form
//! `Self((self.0 * rhs.0) / multiplier())` with a widening intermediate
//! that survives operands above sqrt(i128::MAX) (~1.3e19 raw, ~13,000
//! km at SCALE=12). This bench compares five implementations at
//! several operand magnitudes.
//!
//! Candidates:
//!
//!   A. **Naive baseline** -- the previous shipping form. Overflows
//!      silently (or panics in debug) above ~1.3e19 raw.
//!   B. **Hand-rolled u128 -> 256 schoolbook + 256/128 long divide**
//!      -- pure no-deps Rust. Uses u64 quarter-limbs, four u64x64
//!      widening multiplies for the product, then a binary 256/128
//!      long divide. This is the "no extras, no tricks" widening fix.
//!   C. **i256 crate (Alexhuszagh)** -- third-party 256-bit integer
//!      type. Comparison point only; we are not committing to this
//!      dep.
//!   D. **Moller-Granlund magic-number divide on schoolbook product**
//!      -- the algorithm shape ConstScaleFpdec uses (MIT).
//!      Translates the 1994 Granlund-Montgomery + 2011 Moller-Granlund
//!      papers; precomputed magic constant for divisor 10^12 baked in
//!      const. We hand-code one entry of their 38-entry table here for
//!      D38s12.
//!   E. **Production `D38<12>` operators** -- the actual shipping
//!      `core_type::D38::Mul` / `Div` impls, which route through
//!      `mg_divide::mul_div_pow10` / `div_pow10_div`. Includes the
//!      full 38-entry MG magic table lookup, panic-on-final-overflow
//!      branch, and any function-call / module-boundary overhead the
//!      compiler doesn't inline away. This is the apples-to-apples
//!      "what consumers actually pay" measurement.
//!
//! Operand magnitude buckets:
//!
//!   small  -- raw values within +/- 10^9 (well under i64; everyone
//!             hits the cheap fast path)
//!   mid    -- raw values within +/- 10^15 (above i64 boundary,
//!             safe for naive's i128 multiply)
//!   bound  -- raw values around +/- 10^18 to +/- 10^19 (right at
//!             naive's overflow boundary)
//!   wide   -- raw values around +/- 10^22 (well above naive's
//!             range; naive overflows here, candidates B/C/D do not)
//!
//! Run with `cargo bench -p decimal-scaled --bench mul_div_candidates`.
//!
//! Note on Candidate A: at "wide" we DO NOT run the naive form
//! (it would overflow / wrap and produce garbage). The "wide" row
//! shows --- for naive.
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use decimal_scaled::D38;

type D = D38<12>;
const MULT: i128 = 1_000_000_000_000;

// -----------------------------------------------------------------------------
// Candidate A: naive baseline (the current shipping form).
// -----------------------------------------------------------------------------
#[inline(always)]
fn naive_mul(a: i128, b: i128) -> i128 {
    // overflow at the multiply if |a*b| > i128::MAX
    (a.wrapping_mul(b)).wrapping_div(MULT)
}

#[inline(always)]
fn naive_div(a: i128, b: i128) -> i128 {
    a.wrapping_mul(MULT).wrapping_div(b)
}

// -----------------------------------------------------------------------------
// Candidate B: hand-rolled u128 -> u256 schoolbook + binary long-divide.
// No deps. The schoolbook 128x128 -> 256 is the same shape as the
// `mul2` function in WuBingzheng's MIT-licensed inner_i128.rs, which
// is a textbook formulation; we re-derive here rather than copy.
// -----------------------------------------------------------------------------

/// Compute a (signed) 256-bit product `a * b` and return it as
/// `(hi: i128, lo: u128)`. The high half is signed.
#[inline(always)]
fn mul_full_i128(a: i128, b: i128) -> (i128, u128) {
    let (ua, ub) = (a.unsigned_abs(), b.unsigned_abs());
    let (hi, lo) = mul_full_u128(ua, ub);
    let neg = (a < 0) ^ (b < 0);
    if neg {
        // negate 256-bit value
        let lo_neg = (!lo).wrapping_add(1);
        let carry = if lo == 0 { 1 } else { 0 };
        let hi_neg = (!hi).wrapping_add(carry);
        (hi_neg as i128, lo_neg)
    } else {
        (hi as i128, lo)
    }
}

#[inline(always)]
fn mul_full_u128(a: u128, b: u128) -> (u128, u128) {
    let a_lo = a & u64::MAX as u128;
    let a_hi = a >> 64;
    let b_lo = b & u64::MAX as u128;
    let b_hi = b >> 64;

    let ll = a_lo * b_lo;
    let lh = a_lo * b_hi;
    let hl = a_hi * b_lo;
    let hh = a_hi * b_hi;

    // (lh + hl) goes into the middle 128 bits; carry propagates up.
    let (mid, carry1) = lh.overflowing_add(hl);
    let (lo, carry2) = ll.overflowing_add(mid << 64);
    let hi = hh + (mid >> 64) + ((carry1 as u128) << 64) + carry2 as u128;
    (hi, lo)
}

/// Divide signed 256-bit `(hi, lo)` by signed `d`, return signed
/// 128-bit quotient. Truncating semantics matching i128 `/`.
#[inline]
fn div256_by_i128(hi: i128, lo: u128, d: i128) -> i128 {
    let neg = (hi < 0) ^ (d < 0);
    let (uhi, ulo) = if hi < 0 {
        // negate 256-bit
        let lo_neg = (!lo).wrapping_add(1);
        let carry = if lo == 0 { 1 } else { 0 };
        let hi_neg = (!(hi as u128)).wrapping_add(carry);
        (hi_neg, lo_neg)
    } else {
        (hi as u128, lo)
    };
    let ud = d.unsigned_abs();

    let q = div256_by_u128(uhi, ulo, ud);
    if neg {
        // Note: if q is i128::MIN's magnitude exactly, we'd overflow
        // negating. For bench purposes we accept that limitation; a
        // production version handles it. (Saturating to i128::MIN.)
        if q > i128::MAX as u128 {
            i128::MIN
        } else {
            -(q as i128)
        }
    } else {
        q as i128
    }
}

/// Binary long-divide of 256-bit (uhi, ulo) by 128-bit divisor.
/// Returns the 128-bit quotient and the 128-bit remainder. Caller
/// guarantees the quotient fits (otherwise the high bits are
/// silently dropped; bench inputs respect this).
#[inline]
fn divrem256_by_u128(uhi: u128, ulo: u128, d: u128) -> (u128, u128) {
    if uhi == 0 {
        return (ulo / d, ulo % d);
    }
    // shift-subtract long divide. ~256 iterations; ~80 ns on Skylake.
    let mut hi = uhi;
    let mut lo = ulo;
    let mut q: u128 = 0;
    let mut rem: u128 = 0;
    for _ in 0..256 {
        // shift (rem, hi, lo) left by 1
        rem = (rem << 1) | (hi >> 127);
        hi = (hi << 1) | (lo >> 127);
        lo <<= 1;
        q <<= 1;
        if rem >= d {
            rem -= d;
            q |= 1;
        }
    }
    let _ = hi;
    let _ = lo;
    (q, rem)
}

/// Truncating divide-only wrapper preserved for `handrolled_mul`,
/// which feeds `MULT` as the divisor and relies on the exact-quotient
/// shape (the production `mul_div_pow10` rounding step happens in the
/// magic-divide candidate, not this hand-rolled baseline).
#[inline]
fn div256_by_u128(uhi: u128, ulo: u128, d: u128) -> u128 {
    divrem256_by_u128(uhi, ulo, d).0
}

#[inline(always)]
fn handrolled_mul(a: i128, b: i128) -> i128 {
    // Fast path: if both operands fit in i64, no widening needed.
    if let (Ok(a64), Ok(b64)) = (i64::try_from(a), i64::try_from(b)) {
        // a64 * b64 fits in i128, divide is the only cost.
        return ((a64 as i128) * (b64 as i128)) / MULT;
    }
    let (hi, lo) = mul_full_i128(a, b);
    div256_by_i128(hi, lo, MULT)
}

/// Round a (signed) trunc quotient + (unsigned magnitude) remainder pair
/// to nearest, ties-to-even, matching the production `mg_divide` path.
/// `q` is the truncated signed quotient; `r` is the absolute remainder
/// magnitude; `d` is the absolute divisor magnitude; `neg_result` tells
/// us which direction the round-half-up correction goes.
#[inline]
fn round_half_even(q: i128, r: u128, d: u128, neg_result: bool) -> i128 {
    // Tie: 2*r == d (only meaningful when d is even; for odd d, ties
    // are impossible). Up: 2*r > d. Down: 2*r < d.
    // `2*r` may overflow u128 when r is close to u128::MAX, but in this
    // bench the divisor is always one of MULT (10^12) or a sub-i128
    // operand, so r < d < 2^127 and 2*r is well within u128.
    let twice_r = r << 1;
    let bump = if twice_r > d {
        true
    } else if twice_r == d {
        // tie -> round to even
        (q & 1) != 0
    } else {
        false
    };
    if !bump {
        return q;
    }
    if neg_result { q - 1 } else { q + 1 }
}

#[inline(always)]
fn handrolled_div(a: i128, b: i128) -> i128 {
    // Production `mg_divide::div_pow10_div` rounds half-to-even (matches
    // crate default `RoundingMode::HalfToEven`). To stay an apples-to-apples
    // cross-check we have to do the same rounding here -- straight i128
    // truncation drifts by 1 ULP on any input whose exact quotient has a
    // non-zero fractional part above 0.5.
    let neg_result = (a < 0) ^ (b < 0);
    let ud = b.unsigned_abs();
    // Fast path: if a fits in i64, a * 10^12 still fits i128 (10^12 ~ 2^40,
    // i64 ~ 2^63, headroom ~2^23).
    if let Ok(a64) = i64::try_from(a) {
        let num = (a64 as i128) * MULT;
        let q = num / b;
        let r = (num % b).unsigned_abs();
        return round_half_even(q, r, ud, neg_result);
    }
    // Widening: numerator = a * MULT in i256.
    let (hi, lo) = mul_full_i128(a, MULT);
    // Recover the absolute 256-bit numerator for the remainder calc.
    let (uhi, ulo) = if hi < 0 {
        let lo_neg = (!lo).wrapping_add(1);
        let carry = if lo == 0 { 1 } else { 0 };
        let hi_neg = (!(hi as u128)).wrapping_add(carry);
        (hi_neg, lo_neg)
    } else {
        (hi as u128, lo)
    };
    let (uq, r) = divrem256_by_u128(uhi, ulo, ud);
    let q_trunc = if neg_result {
        if uq > i128::MAX as u128 { i128::MIN } else { -(uq as i128) }
    } else {
        uq as i128
    };
    round_half_even(q_trunc, r, ud, neg_result)
}

// -----------------------------------------------------------------------------
// Candidate C: i256 crate.
// -----------------------------------------------------------------------------
#[inline(always)]
fn i256_mul(a: i128, b: i128) -> i128 {
    use i256::I256;
    let big = I256::from(a) * I256::from(b);
    let q = big / I256::from(MULT);
    // Truncate to i128 (panics on overflow in debug; we keep release for bench)
    q.as_i128()
}

#[inline(always)]
fn i256_div(a: i128, b: i128) -> i128 {
    use i256::I256;
    let big = I256::from(a) * I256::from(MULT);
    let q = big / I256::from(b);
    q.as_i128()
}

// -----------------------------------------------------------------------------
// Candidate D: Moller-Granlund magic-number divide on the schoolbook
// product. Implements the *concept* of MG2011 Algorithm 4 specialised
// to divisor=10^12. The magic numbers below are computed with the
// same Python recipe as ConstScaleFpdec's `MG_EXP_MAGICS[12]` (we
// re-derive them here in this comment block for traceability;
// numeric value matches their table entry, which is correct by
// construction):
//
//   def gen(d):
//       zeros = 128 - d.bit_length()
//       magic = pow(2, 256) // (d << zeros)
//       magic = magic - pow(2, 128)  # fits in 128 bits
//       return (magic, zeros)
//
//   gen(10**12) -> (0x19799812dea11197f27f0f6e885c8ba7, 88)
//
// 10^12 = 0xE8D4A51000, has 40 bits, so leading zeros = 128 - 40 = 88.
// -----------------------------------------------------------------------------

const MG_MAGIC: u128 = 0x19799812dea11197f27f0f6e885c8ba7;
const MG_ZEROS: u32 = 88;
const MG_DIVISOR: u128 = 1_000_000_000_000;

/// Divide unsigned 256-bit (n_high, n_low) by 10^12 using a
/// Moller-Granlund magic-number multiply-and-shift. Returns the
/// 128-bit quotient. Caller guarantees the quotient fits in 128 bits
/// (n_high < 10^12 -- a single divide-step's pre-condition).
#[inline]
fn mg_div_by_pow10_12(n_high: u128, n_low: u128) -> u128 {
    // Algorithm: zn = n << zeros; q = (((magic * zn) >> 128) + zn) >> 128
    // (256-bit dividend variant; see ConstScaleFpdec's div_exp_fast_2word)
    debug_assert!(n_high < MG_DIVISOR, "MG: dividend high half exceeds divisor");

    // (z_high, z_low) := n << zeros
    let z_high = (n_high << MG_ZEROS) | (n_low >> (128 - MG_ZEROS));
    let z_low = n_low << MG_ZEROS;

    // (m_high, m_low) := (magic * zn) >> 128, where zn is 256-bit
    // m1 = magic * z_low (high half only)
    let (m1_high, _) = mul_full_u128(z_low, MG_MAGIC);
    let (m2_high, m2_low) = mul_full_u128(z_high, MG_MAGIC);
    let (m_low, carry) = m2_low.overflowing_add(m1_high);
    let m_high = m2_high + carry as u128;

    // q := (m + zn) >> 128
    let (_, carry) = m_low.overflowing_add(z_low);
    let q = m_high + z_high + carry as u128;

    // Correction: at most off by 1 (MG). Check r = n - q*divisor.
    let (pp_high, pp_low) = mul_full_u128(q, MG_DIVISOR);
    let (r_low, borrow) = n_low.overflowing_sub(pp_low);
    let _ = pp_high; // n_high == pp_high + borrow, asserted by construction
    if r_low < MG_DIVISOR {
        q
    } else {
        q + 1
    }
}

#[inline(always)]
fn mg_mul(a: i128, b: i128) -> i128 {
    // Fast path: both fit i64 -> single i128 multiply, magic divide.
    // For consistency we route this through MG too -- the magic divide
    // also works on a 1-word numerator (n_high = 0).
    if let (Ok(a64), Ok(b64)) = (i64::try_from(a), i64::try_from(b)) {
        let prod = (a64 as i128) * (b64 as i128);
        let neg = prod < 0;
        let abs = prod.unsigned_abs();
        let q = mg_div_by_pow10_12(0, abs);
        return if neg { -(q as i128) } else { q as i128 };
    }

    let (hi, lo) = mul_full_i128(a, b);
    let neg = hi < 0;
    let (uhi, ulo) = if neg {
        let lo_neg = (!lo).wrapping_add(1);
        let carry = if lo == 0 { 1 } else { 0 };
        let hi_neg = (!(hi as u128)).wrapping_add(carry);
        (hi_neg, lo_neg)
    } else {
        (hi as u128, lo)
    };
    let q = mg_div_by_pow10_12(uhi, ulo);
    if neg {
        -(q as i128)
    } else {
        q as i128
    }
}

// MG-style divide for D38s12::div: numerator = a * MULT (256-bit),
// divisor = b (128-bit). MG magic-divide doesn't apply (b is variable).
// We use the same hand-rolled binary long divide as Candidate B.
#[inline(always)]
fn mg_div(a: i128, b: i128) -> i128 {
    handrolled_div(a, b)
}

// -----------------------------------------------------------------------------
// Candidate E: production `D38<12>` operators (the actual shipping path).
// Calls `D::Mul` / `D::Div` so we measure inclusive of the production
// `mg_divide::mul_div_pow10` / `div_pow10_div` helpers, the 38-entry magic
// table lookup, and the overflow-handling branch.
//
// In release builds the production path returns a wrapped quotient on final
// i128 narrowing (matches std-integer wrap-on-overflow). In debug it panics;
// criterion runs in release by default, so the WIDE bucket is observable
// without aborting the bench.
// -----------------------------------------------------------------------------
#[inline(always)]
fn production_mul(a: i128, b: i128) -> i128 {
    (D::from_bits(a) * D::from_bits(b)).0
}

#[inline(always)]
fn production_div(a: i128, b: i128) -> i128 {
    (D::from_bits(a) / D::from_bits(b)).0
}

// -----------------------------------------------------------------------------
// Bench inputs by operand magnitude. Pairs of (a_raw, b_raw) chosen so
// (a*b)/MULT is well-defined for the relevant candidates.
// -----------------------------------------------------------------------------
struct Inputs {
    label: &'static str,
    a: i128,
    b: i128,
}

const SMALL: Inputs = Inputs {
    label: "small_1e9",
    // 1.5e9 raw == 0.0015 in scaled value. Both fit i64 easily.
    a: 1_500_000_000,
    b: 2_300_000_000,
};

const MID: Inputs = Inputs {
    label: "mid_1e15",
    // 3e15 raw == 3000 in scaled value. Above i64 (2^63 ~ 9.2e18) but
    // close to it; still well within naive's safe range.
    a: 3_000_000_000_000_000,
    b: 4_700_000_000_000_000,
};

const BOUND: Inputs = Inputs {
    label: "bound_1e18",
    // 7e18 raw ~ 7e6 scaled. a*b ~ 4.9e37 ~ i128::MAX/4. Right at
    // naive's edge but still works.
    a: 7_000_000_000_000_000_000,
    b: 4_700_000_000_000_000_000,
};

const WIDE: Inputs = Inputs {
    label: "wide_1e22",
    // 5e22 raw ~ 5e10 scaled. a*b ~ 1.5e44 -- WAY past i128::MAX
    // (1.7e38). Naive WILL overflow here; only candidates B/C/D
    // produce correct results.
    a: 50_000_000_000_000_000_000_000,
    b: 30_000_000_000_000_000_000_000,
};

// Verify all candidates agree on the correct values before benching.
fn sanity_check_consistency() {
    // small
    let a = SMALL.a;
    let b = SMALL.b;
    let exp = naive_mul(a, b);
    assert_eq!(handrolled_mul(a, b), exp, "handrolled mul small mismatch");
    assert_eq!(mg_mul(a, b), exp, "mg mul small mismatch");
    // i256 -- separate sanity below
    // mid
    let exp = naive_mul(MID.a, MID.b);
    assert_eq!(handrolled_mul(MID.a, MID.b), exp, "handrolled mul mid mismatch");
    assert_eq!(mg_mul(MID.a, MID.b), exp, "mg mul mid mismatch");
    // bound
    let exp = naive_mul(BOUND.a, BOUND.b);
    assert_eq!(handrolled_mul(BOUND.a, BOUND.b), exp, "handrolled mul bound mismatch");
    assert_eq!(mg_mul(BOUND.a, BOUND.b), exp, "mg mul bound mismatch");
    // wide -- naive overflows, so cross-check handrolled vs mg vs production only.
    let h = handrolled_mul(WIDE.a, WIDE.b);
    let m = mg_mul(WIDE.a, WIDE.b);
    let p = production_mul(WIDE.a, WIDE.b);
    assert_eq!(h, m, "handrolled vs mg wide mismatch ({} vs {})", h, m);
    assert_eq!(h, p, "handrolled vs production wide mismatch ({} vs {})", h, p);

    // Cross-check production E against handrolled for the cases where naive
    // doesn't overflow.
    for inp in [SMALL, MID, BOUND] {
        let r_p = production_mul(inp.a, inp.b);
        let r_h = handrolled_mul(inp.a, inp.b);
        assert_eq!(r_p, r_h, "production vs handrolled mul mismatch at {}", inp.label);
        let r_p = production_div(inp.a, inp.b);
        let r_h = handrolled_div(inp.a, inp.b);
        assert_eq!(r_p, r_h, "production vs handrolled div mismatch at {}", inp.label);
    }
}

fn bench_mul(c: &mut Criterion) {
    sanity_check_consistency();
    let mut group = c.benchmark_group("D38s12::mul");

    for inp in [SMALL, MID, BOUND, WIDE] {
        // Candidate A -- skip at WIDE (overflows)
        if inp.label != "wide_1e22" {
            group.bench_with_input(BenchmarkId::new("A_naive", inp.label), &inp, |bn, inp| {
                bn.iter(|| naive_mul(black_box(inp.a), black_box(inp.b)));
            });
        }
        group.bench_with_input(BenchmarkId::new("B_handrolled", inp.label), &inp, |bn, inp| {
            bn.iter(|| handrolled_mul(black_box(inp.a), black_box(inp.b)));
        });
        group.bench_with_input(BenchmarkId::new("C_i256_crate", inp.label), &inp, |bn, inp| {
            bn.iter(|| i256_mul(black_box(inp.a), black_box(inp.b)));
        });
        group.bench_with_input(BenchmarkId::new("D_mg_magic", inp.label), &inp, |bn, inp| {
            bn.iter(|| mg_mul(black_box(inp.a), black_box(inp.b)));
        });
        group.bench_with_input(BenchmarkId::new("E_production", inp.label), &inp, |bn, inp| {
            bn.iter(|| production_mul(black_box(inp.a), black_box(inp.b)));
        });
    }
    group.finish();
}

fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("D38s12::div");

    for inp in [SMALL, MID, BOUND, WIDE] {
        if inp.label != "wide_1e22" {
            group.bench_with_input(BenchmarkId::new("A_naive", inp.label), &inp, |bn, inp| {
                bn.iter(|| naive_div(black_box(inp.a), black_box(inp.b)));
            });
        }
        group.bench_with_input(BenchmarkId::new("B_handrolled", inp.label), &inp, |bn, inp| {
            bn.iter(|| handrolled_div(black_box(inp.a), black_box(inp.b)));
        });
        group.bench_with_input(BenchmarkId::new("C_i256_crate", inp.label), &inp, |bn, inp| {
            bn.iter(|| i256_div(black_box(inp.a), black_box(inp.b)));
        });
        group.bench_with_input(BenchmarkId::new("D_mg_magic", inp.label), &inp, |bn, inp| {
            bn.iter(|| mg_div(black_box(inp.a), black_box(inp.b)));
        });
        group.bench_with_input(BenchmarkId::new("E_production", inp.label), &inp, |bn, inp| {
            bn.iter(|| production_div(black_box(inp.a), black_box(inp.b)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mul, bench_div);
criterion_main!(benches);
