// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! candidate: rectangular-splitting Taylor core for the wide `exp` Smith
//! chain — `O(w^(1/3))` full-width multiplies where the wired core spends
//! `O(w^(1/2))`. **NOT WIRED.**
//!
//! ## What this replaces
//!
//! [`exp_generic::try_exp_fixed`] evaluates `e^v` by Smith's method: range
//! reduce `v = k·ln2 + s`, halve `s` down by `2^n`, sum the Taylor series
//! for `e^(s/2^n)`, then square `n` times. Its two loops
//! (`exp_generic.rs:1696` and `:1711`) are the whole cost, and each pass of
//! each loop spends one full-width multiply plus one full-width `÷10^ext`.
//!
//! With `n = squaring_levels(ext) ≈ √(3·ext)` and `T ≈ 1.2·n + 4` Taylor
//! terms, the wired core spends
//!
//! ```text
//! n + T  ≈  2.2·√(3·ext)  ≈  3.8·√ext      full-width multiplies
//! ```
//!
//! — 136 of them at `D1232<1231>` (`ext ≈ 1261`), which is what the
//! `squaring_levels` doc comment's "61 of each" is counting.
//!
//! ## The mechanism
//!
//! The Taylor sum is a POLYNOMIAL in the halved argument, and a degree-`T`
//! polynomial does not need `T` non-scalar multiplies. Paterson &
//! Stockmeyer showed `O(√T)` suffice: precompute `h^0 … h^(m-1)` once, then
//! run a Horner recurrence in `h^m` whose inner terms are the precomputed
//! powers scaled by SMALL INTEGER coefficients. With `m ≈ √T` the count is
//! `m + T/m ≈ 2√T` instead of `T`.
//!
//! That changes the balance point of the whole chain. Writing
//! `T(n) = log2(10)·ext / n` (each halving buys `n+1` bits per term), the
//! wired core minimises `n + T(n)` at `n ≈ √(log2(10)·ext)`; this one
//! minimises `n + m + T(n)/m`, which for a free `m` lands at
//! `n ≈ (log2(10)·ext)^(1/3)` and a total of `3·(log2(10)·ext)^(1/3)` —
//! **cube root, not square root**.
//!
//! Capping `m` at [`MAX_BLOCK`] (the stack cost, below) leaves the optimum
//! essentially untouched, because it is flat near the minimum:
//!
//! | `D1232<1231>`, `ext ≈ 1261` | `n` | `T`  | full-width muls |
//! |---|---:|---:|---:|
//! | wired (`try_exp_fixed`)     |  60 |  76  | **136** |
//! | this candidate, `m ≤ 8`     |  23 | 182  | **54**  |
//! | this candidate, `m` free    |  15 | 279  | **50**  |
//!
//! ## Why it is also MORE accurate — the load-bearing half
//!
//! Every squaring DOUBLES the chain's accumulated relative error
//! (`exp_generic.rs:1639`, whose bound carries a `2^n` factor). The wired
//! core's `n = 60` at `D1232<1231>` amplifies its ~136 units of series
//! rounding by `2^60`, consuming ~20 of the 30 guard digits before the
//! narrowing ever runs. This candidate's `n = 23` amplifies ~250 units by
//! `2^23` — ~7 digits. **It leaves ~13 more guard digits at the widest
//! cell**, which is the reverse of the usual speed/accuracy trade and is
//! why it is worth racing even where the timing is a wash.
//!
//! Both loops are re-derived, so the result is NOT bit-identical to
//! [`exp_generic::try_exp_fixed`] — a different summation order rounds
//! differently. It cannot be, and no test here claims otherwise: the
//! contract this must meet is the crate's storage-scale one (0 LSBe at
//! every mode after Ziv narrowing), and the golden gate is its wall. The
//! working-scale test below asserts agreement inside the wired core's OWN
//! published error bound, which is what "consistent with both being
//! correct" means at this layer.
//!
//! ## Reference
//!
//! M. S. Paterson and L. J. Stockmeyer, "On the number of nonscalar
//! multiplications necessary to evaluate polynomials", SIAM Journal on
//! Computing 2(1):60-66, 1973 — the `O(√T)` polynomial-evaluation scheme.
//!
//! D. M. Smith, "Efficient multiple-precision evaluation of elementary
//! functions", Mathematics of Computation 52:131-134, 1989 — the
//! argument-reduction/series balance the wired core implements.
//!
//! F. Johansson, "Efficient implementation of elementary functions in the
//! medium-precision range", 22nd IEEE Symposium on Computer Arithmetic
//! (ARITH-22), 2015 — the combination of the two, and the `O(p^(1/3))`
//! multiply count for `exp`.
//!
//! Implemented from the published descriptions only. No GPL/LGPL source
//! was read.
//!
//! ## Wiring (for the coordinator, AFTER the race — not done here)
//!
//! [`exp_reduced_rectangular`] is a drop-in for `try_exp_fixed`'s block at
//! `exp_generic.rs:1690-1720`: same input (the reduced argument at the
//! extended working scale), same output (`e^reduced` at that scale). The
//! surrounding range reduction, peak gate, `k`-reassembly and narrowing are
//! untouched.
//!
//! Two callers read `squaring_levels` for their OWN estimates and would
//! then be reading a level count this core no longer uses:
//!
//! * the `k < 0` capacity clamp (`exp_generic.rs:1667`) turns
//!   `clamped_levels` into a precision FLOOR `(levels + 10 − |k|)`. This
//!   candidate's level count is `≤ squaring_levels(ext)` at every
//!   `ext ≥ 0` (cube root below square root throughout, verified at the
//!   `ext ≤ 10` small end where the two ladders are closest), so the wired
//!   floor stays an over-estimate and remains SAFE unchanged.
//! * `direct_series_pays` (`exp_generic.rs:1462`) uses it as a cost budget.
//!   Its own doc calls it "a cost gate, not a validity wall", so a stale
//!   budget costs routing quality, never accuracy — but it should be
//!   re-derived against this core's count when wiring.

use crate::algos::exp::exp_generic as eg;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;

/// Largest number of precomputed powers `h^0 … h^(m-1)` the rectangular
/// step may hold.
///
/// This is the candidate's ONE new resource cost: the powers live in an
/// inline `[S; MAX_BLOCK]` (stack, never heap — the crate's absolute
/// invariant), so the frame grows by `MAX_BLOCK · size_of::<S>()`. At the
/// storage-strict rung ceiling `Int<176>` that is ~11 KB; at the
/// composition path's widest work integer `Int<512>` (`D1232`'s `Wexp`) it
/// is ~32 KB.
///
/// 8 rather than the unconstrained `√T` because the cost curve is FLAT
/// near its minimum — at `D1232<1231>` the free-`m` optimum is 50
/// full-width multiplies and `m ≤ 8` gives 54, an 8% concession for a 4×
/// smaller frame. Raising it is a bench question, not a correctness one;
/// lowering it to 1 degenerates this core into the wired one's shape.
const MAX_BLOCK: usize = 8;

/// Ceiling on the block denominator `g_i`, which is injected into the work
/// integer through `lit::<S>(i128)`. Blocks are sized so `g_i` provably
/// stays below this, and [`rectangular_series`] re-checks per step and
/// bails rather than let a wrapped coefficient through.
const MAX_COEFFICIENT: u128 = 1_000_000_000_000_000_000_000_000_000_000; // 10^30

/// Squaring levels for the rectangular core: the `n` minimising
/// `n + m + T(n)/m` rather than the wired core's `n + T(n)`.
///
/// Each halving buys `n + 1` bits per Taylor term, so `T(n) ≈ log2(10)·ext / n`
/// and — with the rectangular step evaluating `T` terms in `≈ 2√T`
/// multiplies — the total is minimised at `n = (log2(10)·ext)^(1/3)`.
/// `log2(10) ≈ 3322/1000`.
///
/// **Never exceeds `exp_generic::squaring_levels(ext)`** (a cube root is
/// below a square root over this whole range), which is what keeps the
/// wired `k < 0` precision floor a valid over-estimate if this core is
/// dropped in behind it.
fn rectangular_levels(extended_working_scale: u32) -> u32 {
    let target = (extended_working_scale as u64).saturating_mul(3322) / 1000;
    let mut levels: u32 = 1;
    while u64::from(levels + 1).saturating_pow(3) <= target {
        levels += 1;
    }
    levels
}

/// Taylor terms needed for `e^h` to reach scale `ext`, from `h`'s actual
/// magnitude.
///
/// `|h| < 2^-(drop_bits - 1)` where `drop_bits` is how many bits `h` sits
/// below `1` at this scale, so `|h|^(T+1)` clears `10^-ext` once
/// `(T + 1)·(drop_bits - 1) ≥ ext·log2(10) + slack`. The `(T+1)!` divisor
/// is DROPPED from the estimate, which only ever over-counts terms — the
/// safe direction, and the same conservatism the wired core's own
/// `direct_series_pays` applies.
fn series_terms<S: BigInt>(halved_arg: S, one_at_scale: S, extended_working_scale: u32) -> u32 {
    let magnitude_bits = eg::bit_length::<S>(halved_arg);
    let one_bits = eg::bit_length::<S>(one_at_scale);
    // `halved_arg` is a reduced argument shifted down by at least one
    // level, so it sits at least 2 bits below `one`; the guard keeps the
    // subtraction total for any caller.
    let drop_bits = one_bits.saturating_sub(magnitude_bits);
    if drop_bits < 2 {
        return 0;
    }
    let need_bits = (u64::from(extended_working_scale) * 3322) / 1000 + 64;
    let terms = need_bits.div_ceil(u64::from(drop_bits - 1)) + 1;
    u32::try_from(terms).unwrap_or(u32::MAX)
}

/// Decimal digit count of `value` (1 for 0) — sizes the coefficient wall.
fn decimal_digits(value: u32) -> u32 {
    let mut digits = 1;
    let mut remaining = value / 10;
    while remaining > 0 {
        digits += 1;
        remaining /= 10;
    }
    digits
}

/// Block width `m` for a `terms`-term sum: `⌊√terms⌋`, capped by
/// [`MAX_BLOCK`] and by the coefficient wall.
///
/// The block denominator `g_i = ∏_{u=1..m}(i·m + u)` has at most `m`
/// factors, none exceeding `terms + 1`, so `g_i ≤ (terms + 1)^m`. Capping
/// `m` at `30 / digits(terms + 1)` holds that under `10^30`, two orders
/// inside `MAX_COEFFICIENT` and eight inside `i128::MAX`.
fn block_size(terms: u32) -> usize {
    if terms <= 3 {
        return 1;
    }
    let mut block: u32 = 1;
    while (block + 1) * (block + 1) <= terms {
        block += 1;
    }
    let coefficient_cap = (30 / decimal_digits(terms + 1)).max(1);
    let chosen = block.min(coefficient_cap).max(1) as usize;
    chosen.min(MAX_BLOCK)
}

/// `Σ_{j=0}^{T} h^j / j!` at `extended_working_scale`, by rectangular
/// splitting. `None` if a block coefficient would exceed
/// [`MAX_COEFFICIENT`] (unreachable under [`block_size`]'s cap — a
/// fail-closed belt, never a silently wrapped coefficient).
///
/// ## The recurrence
///
/// With `A_i = Σ_{j ≥ i·m} h^(j - i·m) · (i·m)! / j!` the sum is `A_0`, and
///
/// ```text
/// A_i = Σ_{r<m} h^r / d(i,r)   +   h^m · A_(i+1) / g(i)
/// d(i,r) = ∏_{u=1..r} (i·m + u)          (a small integer)
/// g(i)   = ∏_{u=1..m} (i·m + u)          (a small integer, = d(i,m))
/// ```
///
/// so every coefficient is a SCALAR: the inner terms reuse the
/// precomputed `h^r` and cost one small-divisor divide each, and the only
/// full-width multiply per block is the single `h^m` Horner step.
fn rectangular_series<S: BigInt>(
    halved_arg: S,
    one_at_scale: S,
    extended_working_scale: u32,
) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    let terms = series_terms::<S>(halved_arg, one_at_scale, extended_working_scale);
    if terms == 0 {
        return Some(one_at_scale);
    }
    let block = block_size(terms);

    // `h^0 … h^(block-1)`, formed once and reused by every block — this is
    // the whole trick, and the only place the candidate holds more state
    // than the wired core.
    let mut powers = [one_at_scale; MAX_BLOCK];
    let mut power_index = 1;
    while power_index < block {
        powers[power_index] =
            eg::mul::<S>(powers[power_index - 1], halved_arg, extended_working_scale);
        power_index += 1;
    }
    // The outer Horner step `h^block`.
    let step = eg::mul::<S>(powers[block - 1], halved_arg, extended_working_scale);

    let block_u128 = block as u128;
    let terms_u128 = u128::from(terms);
    let mut accumulator = eg::zero::<S>();
    let mut block_index = terms / (block as u32) + 1;
    while block_index > 0 {
        block_index -= 1;
        let base = u128::from(block_index) * block_u128;

        // `inner = Σ_{r<block} h^r / d(i,r)`, and `denominator` walks up
        // through `d(i,0)=1, d(i,1), …` finishing at `g(i)`.
        let mut inner = eg::zero::<S>();
        let mut denominator: u128 = 1;
        let mut term_offset: u128 = 0;
        while term_offset < block_u128 {
            if denominator > MAX_COEFFICIENT {
                return None;
            }
            if base + term_offset <= terms_u128 {
                inner = inner + powers[term_offset as usize] / eg::lit::<S>(denominator as i128);
            }
            denominator *= base + term_offset + 1;
            term_offset += 1;
        }
        if denominator > MAX_COEFFICIENT {
            return None;
        }

        // `A_i = inner + h^block · A_(i+1) / g(i)`. The first (highest)
        // block has nothing above it, so the Horner step is skipped rather
        // than multiplying a zero at full width.
        if accumulator != eg::zero::<S>() {
            accumulator = eg::mul::<S>(accumulator, step, extended_working_scale)
                / eg::lit::<S>(denominator as i128);
        }
        accumulator = accumulator + inner;
    }
    Some(accumulator)
}

/// `e^reduced_arg` at `extended_working_scale`, for an argument ALREADY
/// range-reduced into `|reduced_arg| ≤ (ln 2)/2 · 10^ext`.
///
/// The drop-in for [`exp_generic::try_exp_fixed`]'s Taylor + squaring block
/// (`exp_generic.rs:1690-1720`). `None` propagates
/// [`rectangular_series`]'s fail-closed coefficient check; every other
/// input returns `Some`.
///
/// The caller keeps ownership of the range reduction, the overflow peak
/// gate, the `2^k` reassembly and the storage narrowing — none of which
/// this touches.
#[allow(dead_code)]
pub(crate) fn exp_reduced_rectangular<S: BigInt>(
    reduced_arg: S,
    extended_working_scale: u32,
) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    let one_at_scale = eg::one::<S>(extended_working_scale);
    let levels = rectangular_levels(extended_working_scale);
    let halved_arg = reduced_arg >> levels;

    let mut squared = if halved_arg == eg::zero::<S>() {
        // `e^0 = 1` exactly; the squarings below preserve it exactly.
        one_at_scale
    } else {
        rectangular_series::<S>(halved_arg, one_at_scale, extended_working_scale)?
    };

    let mut level = 0;
    while level < levels {
        squared = eg::round_div_pow10::<S>(
            squared.wrapping_sqr_low_u128(),
            extended_working_scale,
        );
        level += 1;
    }
    Some(squared)
}

#[cfg(test)]
mod tests {
    //! NOT RUN by the author — the candidate-writer contract. The
    //! coordinator races this against the wired core and gates on golden.

    use super::*;
    use crate::int::types::Int;

    /// `10^scale` divided by `divisor` — a reduced argument inside
    /// `|v| ≤ (ln 2)/2 ≈ 0.3466`, built without a parser.
    fn reduced<const N: usize>(scale: u32, divisor: i128) -> Int<N> {
        eg::pow10::<Int<N>>(scale) / Int::<N>::from_i128(divisor)
    }

    /// `e^0 = 1` must come back EXACTLY, at every scale and both loops —
    /// the one anchor that admits no tolerance. A wrong term count or a
    /// mis-indexed block cannot survive it, because the squaring chain
    /// preserves an exact `1` and destroys anything else.
    #[test]
    fn zero_argument_is_exactly_one() {
        let mut checked = 0u32;
        for scale in [0u32, 1, 20, 70] {
            let one = eg::pow10::<Int<8>>(scale);
            assert_eq!(
                exp_reduced_rectangular::<Int<8>>(Int::<8>::ZERO, scale),
                Some(one),
                "e^0 at scale {scale} must be exactly 1"
            );
            checked += 1;
        }
        assert!(checked > 0, "this test must never pass without asserting");
    }

    /// The rectangular core must agree with the wired
    /// [`exp_generic::try_exp_fixed`] on a reduced argument (where the
    /// wired core's own `k` is 0, so the two evaluate the SAME quantity by
    /// different routes).
    ///
    /// The tolerance is the wired core's OWN published error bound
    /// (`exp_generic.rs:1639`: `2^n·(T+2)` working units, `n` its squaring
    /// count) — not a slack invented here. Agreement inside it means the
    /// two are consistent with both being correct; a structural defect in
    /// the recurrence, the term count or the block indexing perturbs the
    /// result by a fraction of the VALUE, which is larger than this bound
    /// by scores of orders of magnitude and cannot hide under it.
    ///
    /// Bit-identity is NOT asserted and is not achievable: the summation
    /// order differs, so the roundings differ. The storage-scale contract
    /// is the golden gate's, not this test's.
    #[test]
    fn agrees_with_wired_core_within_its_error_bound() {
        let mut checked = 0u32;

        macro_rules! cell {
            ($limbs:literal, $scale:literal) => {{
                // `2^n·(T+2)` at the WIRED core's level count, with
                // `T ≤ 1.2·n + 4` per its own bound. Saturating so a wide
                // cell cannot overflow the tolerance itself.
                let wired_levels = {
                    let bound = ($scale as u32) * 3 + 1;
                    let mut n: u32 = 1;
                    while (n + 1) * (n + 1) <= bound {
                        n += 1;
                    }
                    n
                };
                let taylor_terms = wired_levels * 12 / 10 + 4;
                let tolerance = Int::<$limbs>::from_i128(2)
                    .pow(wired_levels + 1)
                    * Int::<$limbs>::from_i128(i128::from(taylor_terms + 2));

                for divisor in [3i128, 4, 5, -4, -7, 1000] {
                    let argument = reduced::<$limbs>($scale, divisor);
                    let candidate =
                        exp_reduced_rectangular::<Int<$limbs>>(argument, $scale)
                            .expect("coefficient wall must not fire on a reduced argument");
                    let wired = eg::try_exp_fixed::<Int<$limbs>>(argument, $scale)
                        .expect("wired core must accept a reduced argument");
                    let difference = (candidate - wired).abs();
                    assert!(
                        difference <= tolerance,
                        "Int<{}> scale {}: 1/{divisor} differs by {difference:?}, \
                         outside the wired core's own {tolerance:?}-unit bound",
                        $limbs,
                        $scale
                    );
                    checked += 1;
                }
            }};
        }

        // Int<8> holds 154 decimal digits, so the `2·ext`-digit squaring
        // peak caps these cells at ext = 70.
        cell!(8, 20);
        cell!(8, 40);
        cell!(8, 70);
        // Int<24> holds 462, capping at ext = 230.
        cell!(24, 120);
        cell!(24, 200);

        assert!(checked > 0, "this test must never pass without asserting");
    }

    /// The level ladder must stay AT OR BELOW the wired
    /// `squaring_levels`, which is the premise the wiring note rests on:
    /// the wired `k < 0` precision floor reads a level count, and a
    /// candidate that ran MORE levels would invalidate it.
    #[test]
    fn levels_never_exceed_the_wired_ladder() {
        let mut checked = 0u32;
        for scale in [0u32, 1, 5, 10, 30, 100, 300, 954, 1261, 2000] {
            let wired = {
                let bound = scale.saturating_mul(3).saturating_add(1);
                let mut n: u32 = 1;
                while (n + 1) * (n + 1) <= bound {
                    n += 1;
                }
                n
            };
            assert!(
                rectangular_levels(scale) <= wired,
                "scale {scale}: rectangular levels {} exceed the wired ladder's {wired}",
                rectangular_levels(scale)
            );
            checked += 1;
        }
        assert!(checked > 0, "this test must never pass without asserting");
    }

    /// The block denominator `g_i = ∏_{u=1..m}(i·m + u)` must stay inside
    /// the coefficient wall for every block the sizing admits — the
    /// property that lets [`rectangular_series`]'s fail-closed check be
    /// unreachable rather than load-bearing.
    #[test]
    fn block_coefficients_stay_inside_the_wall() {
        let mut checked = 0u32;
        for terms in [4u32, 16, 60, 182, 279, 700, 4000, 20_000] {
            let block = block_size(terms) as u128;
            let top_base = u128::from(terms / block as u32) * block;
            let mut denominator: u128 = 1;
            let mut offset: u128 = 0;
            while offset < block {
                denominator = denominator
                    .checked_mul(top_base + offset + 1)
                    .expect("block denominator must not overflow u128");
                offset += 1;
            }
            assert!(
                denominator <= MAX_COEFFICIENT,
                "terms {terms}: block denominator {denominator} exceeds the wall"
            );
            checked += 1;
        }
        assert!(checked > 0, "this test must never pass without asserting");
    }
}
