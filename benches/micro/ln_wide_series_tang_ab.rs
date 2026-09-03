// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier ln Series-vs-Tang map (the N-way width x scale x algorithm x
//! GUARD sweep). For each wide tier, at the 5-point scale set
//! {0, S/4, S/2, 3S/4, S-1}, asserts each Tang candidate == Series across
//! the operand spread x all eight modes (the **validity wall** -- if any
//! disagree, single-shot Tang is not correctly-rounded there, that candidate
//! is reported INVALID and dropped), then ranks the surviving Tang
//! candidates against Series.
//!
//! Wire a wide Tang `select`/`tang_routed` arm ONLY for a cell where a Tang
//! candidate is BOTH bit-identical to Series AND faster here.
//!
//! Tang configs probed per cell: (G=8, CAP=200) the production narrow-wide
//! config; (G=10, CAP=400) the wider-guard / wider-cap config the wider
//! tiers already use; (G=12, CAP=400) the max-scale-extreme candidate.
//!
//! Establishes the continuous win-region for each wide tier so the
//! `policy::ln::tang_routed` gates can be widened off their bbc-cell point
//! ranges (the Class-I single-cell fit) onto the bisected true crossover.
//!
//! # OPERAND CONTRACT -- read this before changing `ln_inputs`
//!
//! BOTH kernels short-circuit on the SAME degenerate operand classes, and an
//! operand that lands on one measures a table lookup instead of the kernel
//! the cell claims to be racing. The first version of this bench used
//! `{0.5, 2.0, 7.5}` and ALL THREE were degenerate, which voided both the
//! timing map and the validity wall it was built on. Two traps, both keyed on
//! the binary range reduction `v = 2^k * m`, `m` in [1, 2):
//!
//! * **Trap 1 -- exact power of two (`m == 1`).** `ln_tang.rs` (the
//!   `mantissa_w == one_at_extended_scale` arm) and `exp_generic::ln_fixed`
//!   (the `mantissa_w == one_at_working_scale` arm) BOTH return `k * ln2`
//!   from a one-word `scale_by_k` product. Neither the artanh series nor the
//!   Brent sqrt reduction runs. `0.5`, `1`, `2.0`, `4` all land here, at every
//!   width and every scale.
//! * **Trap 2 -- exact Tang table boundary (`t == 0`), Tang only.** Tang
//!   picks `f_i = 1 + i/M` with `M = 128` and forms `t = (m - f_i)/(m + f_i)`.
//!   When `m` is an exact multiple of `1/128` the residual `t` is EXACTLY
//!   zero, so `atanh_arg_sq` is zero, the first loop term is zero, and the
//!   artanh series breaks on its first iteration -- Tang degenerates to a
//!   table read while Series still pays its full sqrt reduction. `7.5` is
//!   `2^2 * 1.875` and `1.875 = 1 + 112/128` EXACTLY, so `7.5` is this trap
//!   at every width and every scale. Every value whose binary mantissa
//!   terminates within 7 fraction bits is caught by it.
//!
//! **The sufficient condition used here, in terms of the stored integer
//! `raw` (= `x * 10^SCALE`).** Let `w` be the working scale and
//! `wv = raw * 10^GUARD`. `m == 1` iff `raw == 2^k * 10^SCALE`, and
//! `t == 0` iff `2^k * 10^SCALE` divides `128 * raw` (`k >= 0`; the `k < 0`
//! mirror needs `10^SCALE | 2^(7+|k|) * raw`). Both need `5^SCALE | raw`
//! once `SCALE >= 1`, and both need `raw` even unless the power of two is
//! trivial. So:
//!
//! > **`raw` ODD and `raw % 5 != 0` defeats BOTH traps at every `SCALE >= 1`,
//! > at every width, for every `k`.**
//!
//! At `SCALE == 0` there is no `10^SCALE` factor to lean on, so the boundary
//! test reduces to `2^k | 128 * raw`; with `raw` odd that is `k <= 7`, so the
//! rule becomes **`raw` odd AND `raw >= 257`** (which also excludes `raw == 1`,
//! the only odd power of two). [`assert_non_degenerate`] enforces exactly
//! these two conditions on every measured operand, so the defect cannot
//! silently return: `raw` odd is limb 0's low bit, and `raw % 5` is the sum
//! of the limbs mod 5 because `2^64 == 1 (mod 5)`.
//!
//! The measured operands are `1/3` and `7/3`, truncated to `SCALE` digits:
//! `0.333...3` and `2.333...3`. Both are odd and end in 3, so neither is
//! divisible by 2 or 5 at any scale -- the contract holds by construction
//! rather than by luck. They also fill every one of the tier's `SCALE`
//! fraction digits (a repeating decimal has no trailing zeros), so the artanh
//! series runs its true term count instead of exiting early on an exactly
//! representable short operand. One sits BELOW 1 (`k < 0`, the branch whose
//! truncation-sign argument `ln_tang.rs` documents at length) and one ABOVE
//! (`k > 0`), and both are far enough from 1 that the near-1 value gate in
//! `ln_tang_g` stays shut, so the cell measures the ordinary path.
//!
//! **The `_p2` control group** benches `2.0` -- deliberately Trap 1 -- in its
//! OWN criterion group so it never contaminates the main cell's verdict. It
//! is what bbc and the previous version of this bench measured, so its ratio
//! against the main group is the direct size of the power-of-two fast path,
//! and its presence proves the main group is not sitting on that fast path.
//!
//! # KNOWN INSTRUMENT GAPS (do not read the map past these)
//!
//! * **Work rung.** The `__bench_internals` Tang exports call
//!   `ln_tang::ln_tang` (`Wk = C::W`, the tier's full work integer), but
//!   `policy::ln::tang_routed` routes through `tang_at_rung`, which picks the
//!   NARROWEST `ln_rung::<C, SCALE>()` that clears the cell's digit budget.
//!   At low scales that is much narrower than `C::W` (D307<0> routes `Int<16>`
//!   where this bench runs `Int<64>`), so **this bench HANDICAPS Tang** by up
//!   to ~2.75x work width. A Tang win here is a fortiori a production win; a
//!   Tang LOSS here does not transfer and needs a rung-faithful re-race.
//!   Series is unaffected -- `series_routed` calls the `Wk = C::W` alias too.
//! * **D462 `INTERNAL_EXTRA`.** Production D462 passes `INTERNAL_EXTRA = true`;
//!   the `_p` exports hardcode `false`. The flag is value-gated to near-1
//!   inputs, so it is inert for these operands -- but it means the near-1 band
//!   cannot be mapped through this bench at D462.
//! * **Tiers D57 / D115 / D153 are absent.** `select` routes Tang at `(3, ..)`,
//!   `(6, ..)` and `(8, ..)`, but those three tiers have no `(G, CAP)`-
//!   parameterised export, so 3 of the 10 wide tiers have never been in this
//!   map at all.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench ln_wide_series_tang_ab`
//!
//! `compare_all`'s coarse re-time runs a fixed pass count and ignores
//! criterion's name filter, so a criterion filter alone does not bound the
//! wall time. Set `LN_AB_ONLY` to a substring of the group name to skip whole
//! cells and chunk a long sweep:
//! `LN_AB_ONLY=ln_d1232 cargo bench ... --bench ln_wide_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    int_from_mag_limbs, ln_series_d1232, ln_series_d230, ln_series_d307, ln_series_d462,
    ln_series_d616, ln_series_d76, ln_series_d924, ln_tang_d1232_p, ln_tang_d230_p,
    ln_tang_d307_p, ln_tang_d462_p, ln_tang_d616_p, ln_tang_d76_p, ln_tang_d924_p,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;
const ALL_MODES: [RoundingMode; 8] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
    RoundingMode::AwayFromZero,
    RoundingMode::ZeroFiveUp,
];

/// `acc = acc * m + add`, in place across the little-endian limb array. Pure
/// big-int limb arithmetic so any decimal scale (even scale >= 39, where
/// `10^scale` overflows u128) is representable in the wide `Int<N>`.
fn mul_add_small<const N: usize>(acc: &mut [u64; N], m: u64, add: u64) {
    let mut carry = add as u128;
    for limb in acc.iter_mut() {
        let prod = (*limb as u128) * (m as u128) + carry;
        *limb = prod as u64;
        carry = prod >> 64;
    }
}

/// The magnitude limbs of `floor(x_num * 10^scale / x_den)`, little-endian.
/// Never via `10u128.pow` (which overflows for scale >= 39): computes
/// `x_num * 10^scale` limb-wise then divides exactly by the small denominator.
fn build_mag<const N: usize>(x_num: u64, x_den: u64, scale: u32) -> [u64; N] {
    let mut mag = [0u64; N];
    mag[0] = x_num;
    for _ in 0..scale {
        mul_add_small::<N>(&mut mag, 10, 0);
    }
    if x_den != 1 {
        let mut rem = 0u128;
        for limb in mag.iter_mut().rev() {
            let cur = (rem << 64) | (*limb as u128);
            *limb = (cur / x_den as u128) as u64;
            rem = cur % x_den as u128;
        }
    }
    mag
}

/// `raw mod 2` and `raw mod 5` straight off the magnitude limbs. `raw` is odd
/// iff limb 0's low bit is set; `2^64 == 1 (mod 5)` (because `2^4 == 1`), so
/// `raw mod 5` is just the sum of the limbs mod 5.
fn parity_and_mod5<const N: usize>(mag: &[u64; N]) -> (u64, u64) {
    let mut sum_mod5 = 0u64;
    for &limb in mag.iter() {
        sum_mod5 = (sum_mod5 + limb % 5) % 5;
    }
    (mag[0] & 1, sum_mod5)
}

/// The operand contract from this file's header, enforced. A measured operand
/// must defeat BOTH degeneracy traps at its `(N, scale)` cell:
///
/// * `raw` ODD kills the power-of-two short-circuit (`m == 1`) at every scale,
///   because `raw == 2^k * 10^scale` forces `raw` even for `scale >= 1` and
///   forces `raw == 1` at scale 0;
/// * `raw % 5 != 0` kills the exact-table-boundary (`t == 0`) for `scale >= 1`,
///   because that needs `5^scale | raw`;
/// * at scale 0 there is no `5^scale` to lean on, so `raw >= 257` is required
///   instead: with `raw` odd the boundary test collapses to `k <= 7`, and
///   `raw >= 257` forces `k >= 8`.
///
/// Panicking here is the point -- a silently degenerate operand is exactly the
/// defect that voided the first version of this map.
fn assert_non_degenerate<const N: usize>(mag: &[u64; N], label: &str, scale: u32) {
    let nonzero = mag.iter().any(|&l| l != 0);
    assert!(nonzero, "operand {label} at scale {scale} is zero -- outside ln's domain");
    let (parity, mod5) = parity_and_mod5::<N>(mag);
    assert_eq!(
        parity, 1,
        "operand {label} at scale {scale}: raw must be ODD or it can take the \
         power-of-two short-circuit (ln_tang.rs `mantissa_w == one`, \
         exp_generic::ln_fixed `mantissa_w == one`)"
    );
    if scale == 0 {
        // Odd and >= 257 => k >= 8 > 7 => `2^k` cannot divide `128 * raw`.
        let above_256 = mag[1..].iter().any(|&l| l != 0) || mag[0] > 256;
        assert!(
            above_256,
            "operand {label} at scale 0: raw must be >= 257 or its mantissa \
             terminates within 7 fraction bits and lands exactly on a Tang \
             table boundary (t == 0)"
        );
    } else {
        assert_ne!(
            mod5, 0,
            "operand {label} at scale {scale}: raw must NOT be divisible by 5 or \
             `5^scale | raw` becomes reachable and the Tang residual t can be \
             exactly zero"
        );
    }
}

/// Build a validated non-degenerate operand.
fn build_raw<const N: usize>(x_num: u64, x_den: u64, scale: u32, label: &str) -> Int<N> {
    let mag = build_mag::<N>(x_num, x_den, scale);
    assert_non_degenerate::<N>(&mag, label, scale);
    int_from_mag_limbs::<N>(&mag)
}

/// Build the power-of-two CONTROL operand. Deliberately degenerate -- asserts
/// the opposite of [`assert_non_degenerate`] so the control cannot silently
/// stop being a control.
fn build_raw_pow2<const N: usize>(scale: u32) -> Int<N> {
    let mag = build_mag::<N>(2, 1, scale);
    let (parity, _) = parity_and_mod5::<N>(&mag);
    assert_eq!(parity, 0, "the _p2 control must be an exact power of two times 10^scale");
    int_from_mag_limbs::<N>(&mag)
}

#[derive(Clone)]
struct One<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

/// The two MEASURED ln arguments at decimal `SCALE`, both satisfying the
/// operand contract in this file's header:
///
/// * `x_lo` = `1/3` truncated to `SCALE` digits = `0.333...3`. Below 1, so the
///   reduction gives `k < 0` and the kernel takes the left-shift branch and the
///   `-ln2 + ln(m)` assembly whose one-sided truncation argument `ln_tang.rs`
///   documents. `raw` ends in 3: odd, not divisible by 5.
/// * `x_hi` = `7/3` truncated to `SCALE` digits = `2.333...3`. Above 1 (`k > 0`),
///   the same digits, the other sign of `k`. `raw` ends in 3: odd, not
///   divisible by 5.
///
/// Both are repeating decimals, so every one of the tier's `SCALE` fraction
/// digits is significant and the artanh series runs its true term count. Both
/// are far from 1, so `ln_tang_g`'s near-1 value gate stays shut and the cell
/// measures the ordinary path. `7 * 10^SCALE` is the largest intermediate and
/// fits every tier through `SCALE = cap - 1`.
///
/// SCALE 0 cannot use those (`floor(1/3) = 0` is outside the domain and
/// `floor(7/3) = 2` is an exact power of two). It uses `333` and `2333` --
/// the same digit strings read as integers, both odd and both >= 257, which is
/// the scale-0 form of the contract.
fn ln_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    if scale == 0 {
        vec![
            One { label: "x333", raw: build_raw::<N>(333, 1, 0, "x333") },
            One { label: "x2333", raw: build_raw::<N>(2333, 1, 0, "x2333") },
        ]
    } else {
        vec![
            One { label: "x_lo", raw: build_raw::<N>(1, 3, scale, "x_lo") },
            One { label: "x_hi", raw: build_raw::<N>(7, 3, scale, "x_hi") },
        ]
    }
}

/// The power-of-two CONTROL input: `2.0` (`2` at scale 0). Deliberately takes
/// the `m == 1` short-circuit in BOTH kernels, so this group prices the
/// power-of-two fast path rather than the ln kernels. Benched in its own
/// criterion group so it never enters the main cell's verdict.
fn ln_pow2_input<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![One { label: "x2.0", raw: build_raw_pow2::<N>(scale) }]
}

type LnFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

/// `true` if this group is selected. `compare_all`'s coarse re-time ignores
/// criterion's own name filter, so cell selection has to happen here for a
/// filter to actually bound the wall time.
fn selected(group: &str) -> bool {
    match std::env::var("LN_AB_ONLY") {
        Ok(filter) if !filter.is_empty() => group.contains(&filter),
        _ => true,
    }
}

/// One cell: validate each Tang candidate against Series (bit-identical
/// across the spread x all modes), drop the invalid ones, then rank the
/// survivors and Series in a single `compare_all` run.
fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    inputs: Vec<One<N>>,
    series: LnFn<N>,
    candidates: &[(&'static str, LnFn<N>)],
) {
    if !selected(group) {
        return;
    }
    let mut runs: Vec<(&'static str, Box<dyn Fn(One<N>) -> Int<N>>)> =
        vec![("series", Box::new(move |o: One<N>| series(o.raw, MODE)))];
    for &(label, tang) in candidates {
        let mut valid = true;
        'outer: for o in inputs.iter() {
            for m in ALL_MODES {
                if tang(o.raw, m) != series(o.raw, m) {
                    println!("VALIDITY [{group}]: {label} != series ({}, mode {m:?}) -> INVALID, skipping", o.label);
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            runs.push((label, Box::new(move |o: One<N>| tang(o.raw, MODE))));
        }
    }
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all Tang candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), inputs, runs);
}

/// One (tier, scale) cell: Series vs three Tang configs (G=8/CAP=200 = the
/// narrow-wide production config; G=10/CAP=400 = wider guard + wider cap
/// the wider tiers use; G=12/CAP=400 = wider-guard max-scale-extreme
/// candidate), over the two non-degenerate operands -- plus the separate
/// `_p2` power-of-two control group. SCALE is a literal const generic so each
/// cell is its own monomorphisation, as the policy sees it.
macro_rules! cell_ln {
    ($c:expr, $n:literal, $name:literal, $scale:literal, $series:ident, $tang:ident) => {
        cell::<$n>(
            $c,
            concat!("ln_", $name, "_s", stringify!($scale)),
            ln_inputs::<$n>($scale),
            $series::<$scale>,
            &[
                ("tang_g8_c200", $tang::<$scale, 8, 200>),
                ("tang_g10_c400", $tang::<$scale, 10, 400>),
                ("tang_g12_c400", $tang::<$scale, 12, 400>),
            ],
        );
        cell::<$n>(
            $c,
            concat!("ln_", $name, "_s", stringify!($scale), "_p2"),
            ln_pow2_input::<$n>($scale),
            $series::<$scale>,
            &[
                ("tang_g8_c200", $tang::<$scale, 8, 200>),
                ("tang_g10_c400", $tang::<$scale, 10, 400>),
                ("tang_g12_c400", $tang::<$scale, 12, 400>),
            ],
        );
    };
}

/// Sweep a tier across the FIVE coarse scale points
/// `{0, S/4, S/2, 3S/4, S-1}` (the owner-standard sampling) as literals.
macro_rules! tier {
    ($c:expr, $n:literal, $name:literal, $series:ident, $tang:ident,
     $s0:literal, $s1:literal, $s2:literal, $s3:literal, $s4:literal) => {{
        cell_ln!($c, $n, $name, $s0, $series, $tang);
        cell_ln!($c, $n, $name, $s1, $series, $tang);
        cell_ln!($c, $n, $name, $s2, $series, $tang);
        cell_ln!($c, $n, $name, $s3, $series, $tang);
        cell_ln!($c, $n, $name, $s4, $series, $tang);
    }};
}

fn benches(c: &mut Criterion) {
    // Per tier: scales {0, S/4, S/2, 3S/4, S-1} (the owner-standard 5-point
    // coarse sampling). The top point is the tier's MAX-SCALE EXTREME; ln's
    // domain x > 0 plus the operand spread fits storage at every wide tier
    // through s = cap-1 (the largest intermediate is `7 * 10^s`).
    // D76 (Int<4>, cap 76).
    tier!(c, 4, "d76", ln_series_d76, ln_tang_d76_p, 0, 19, 38, 57, 75);
    // D230 (Int<12>, cap 230).
    tier!(c, 12, "d230", ln_series_d230, ln_tang_d230_p, 0, 57, 115, 172, 229);
    // D307 (Int<16>, cap 307).
    tier!(c, 16, "d307", ln_series_d307, ln_tang_d307_p, 0, 76, 153, 230, 306);
    // D462 (Int<24>, cap 462).
    tier!(c, 24, "d462", ln_series_d462, ln_tang_d462_p, 0, 115, 231, 346, 461);
    // D616 (Int<32>, cap 616).
    tier!(c, 32, "d616", ln_series_d616, ln_tang_d616_p, 0, 154, 308, 462, 615);
    // D924 (Int<48>, cap 924).
    tier!(c, 48, "d924", ln_series_d924, ln_tang_d924_p, 0, 231, 462, 693, 923);
    // D1232 (Int<64>, cap 1232).
    tier!(c, 64, "d1232", ln_series_d1232, ln_tang_d1232_p, 0, 308, 616, 924, 1231);
}

fn main() {
    // `configure_from_args` so a criterion name filter is honoured at all --
    // without it a `--bench <name> -- <filter>` argument is silently ignored
    // and the whole suite runs. `LN_AB_ONLY` is still what bounds wall time,
    // because the coarse re-time inside `compare_all` is not filtered.
    let mut c = micro_criterion().configure_from_args();
    benches(&mut c);
    c.final_summary();
}
