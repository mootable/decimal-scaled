// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `*_strict` vs `*_fast` — the price of the f64 bridge, measured.
//!
//! Every public transcendental in this crate ships TWO named surfaces:
//! `<op>_strict` (integer-only, correctly rounded to 0.5 ULP) and
//! `<op>_fast` (`Self::from_f64(self.to_f64().<op>())`, the platform-libm
//! bridge). Twenty-seven op families have both, at all twelve widths — and
//! **nothing in this repo has ever raced them against each other.** The
//! `bench-compare` surface benches only whichever one the build's feature
//! flags make the plain method, so the two have never appeared in the same
//! process.
//!
//! This bench races them IN ONE PROCESS on ONE machine, so the ratio is a
//! within-cell quantity and carries no machine-to-machine floor. The
//! deliverable is the ABSOLUTE nanoseconds on both sides as much as the
//! ratio: a "2x" that is 3 ns against 6 ns is a different fact from a "2x"
//! that is 3 ms against 6 ms, and only the pair of numbers says which.
//!
//! # The grid
//!
//! 27 ops x 12 widths x the 5-point scale grid `{0, S/4, S/2, 3S/4, S}`
//! where `S` is the tier's `MAX_SCALE` (`= name - 1`). That is 1620 cells,
//! less the 36 that have no legal operand (see "Scale 0" below), so **1584
//! verdicts** is the expected count. Read the COUNT, never the exit code: a
//! bench excluded by missing `required-features` exits 0 having graded
//! nothing.
//!
//! # BOTH feature families in ONE build — verified, not assumed
//!
//! `strict` and `fast` are not mutually exclusive at the Cargo level and the
//! `<op>_strict` / `<op>_fast` methods are emitted UNCONDITIONALLY (the
//! `_fast` family behind `feature = "std"`, which `fast` implies). Only the
//! PLAIN dispatcher `<op>` is feature-selected, by two different and mutually
//! exclusive `cfg`s:
//!
//! * `src/macros/strict_transcendentals.rs` — `not(all(fast, not(strict)))`
//! * `src/macros/fast_transcendentals.rs` — `all(std, fast, not(strict))`
//!
//! So `--features "strict fast"` compiles both named families and this bench
//! never touches a plain method. That matters: under `strict + fast` the
//! plain dispatchers are NOT uniformly resolved (D18 keeps strict; D38 flips
//! to fast via `types/log_exp_fast.rs`'s `any(not(strict), fast)` gate; the
//! ten wide tiers get NO plain method at all, because BOTH their gates —
//! `wide_transcendental.rs` `all(strict, not(fast))` and
//! `fast_transcendentals.rs` `all(std, fast, not(strict))` — are false). That
//! asymmetry is invisible to this measurement precisely because every call
//! here is to an explicitly named method.
//!
//! `required-features` therefore lists **`std`, `strict`, `fast`** as well as
//! the tier umbrellas. Miss `fast` and cargo silently SKIPS this bench and
//! still exits 0.
//!
//! # OPERAND CONTRACT — read before changing an operand
//!
//! A degenerate operand VOIDS a cell whatever its magnitude: the range
//! reduction short-circuits and the cell prices a table lookup instead of the
//! kernel it claims to be racing. The contract this file enforces is the one
//! derived and enforced in `benches/micro/ln_wide_series_tang_ab.rs`, stated
//! on the stored integer `raw` (`= x * 10^SCALE`):
//!
//! > **`raw` ODD and `raw % 5 != 0` defeats both the power-of-two
//! > short-circuit and the exact-Tang-table-boundary trap at every
//! > `SCALE >= 1`.** At `SCALE 0` there is no `10^SCALE` factor to lean on,
//! > so an ln-family argument additionally needs **`raw >= 257`**.
//!
//! Both conditions read off the LAST DECIMAL DIGIT of the operand string
//! (`raw mod 2` and `raw mod 5` are functions of it alone), which is why
//! [`assert_non_degenerate`] can enforce the contract on the literal text and
//! the defect cannot silently return. The operands are built as digit strings
//! carrying EXACTLY `SCALE` fraction digits, so `FromStr` is exact and the
//! text IS the raw.
//!
//! The measured operands, all repeating decimals so every one of the tier's
//! `SCALE` fraction digits is significant (a series runs its true term count
//! rather than exiting early on a short exactly-representable operand):
//!
//! * `a` = `1/9` = `0.111…1` — last digit 1. The ln-family argument (`k < 0`
//!   in the binary reduction), the unit-interval argument for
//!   asin / acos / atanh, and the small argument for the angle conversions.
//! * `b` = `7/3` = `2.333…3` — last digit 3. Above 1 (so `k > 0`, and legal
//!   for `acosh`), and past `pi/2` so sin / cos / tan pay a real range
//!   reduction rather than a two-term series.
//! * `small` — `a` at every `SCALE >= 1`; `7` at `SCALE 0`, because
//!   `exp(333)` and `sinh(333)` overflow every tier narrower than D153.
//! * `pexp` — `b` at every `SCALE >= 1`; `3` at `SCALE 0`, as `powf`'s
//!   exponent (never logged, so the `>= 257` ln rule does not apply to it),
//!   because `333^2333` overflows everything.
//!
//! Every result was checked to fit ONE integer digit, which is all a tier has
//! at `SCALE = MAX_SCALE`: the widest are `acos(0.111…) = 1.4595`,
//! `hypot(0.111…, 2.333…) = 2.3359` and `to_degrees(0.111…) = 6.3662`.
//!
//! # SCALE 0 — three ops have NO legal operand, and they are SKIPPED loudly
//!
//! At `SCALE 0` the only representable values are integers, so
//! **asin / acos / atanh have no non-degenerate argument at all**: their
//! domain is `[-1, 1]`, whose only integer members are `0` and `+/-1`, and
//! all three are the degenerate arguments those kernels special-case. Those
//! 3 ops x 12 widths = 36 cells print a `SKIP` line instead of a verdict.
//! This is a property of the TYPE, not of this bench — the tracked surface
//! benches those cells at argument exactly zero today.
//!
//! Known residual, recorded rather than papered over: `exp2` at `SCALE 0` has
//! an integer argument, so its result is an exact power of two at every
//! width. There is no scale-0 operand that avoids it. Read the `exp2 s0` row
//! as a lower bound on strict's cost.
//!
//! # WHERE THE f64 PATH ACTUALLY EXISTS — read the ratio with this
//!
//! `_fast` is `from_f64(to_f64(x).op())`, and `to_f64` divides the raw by
//! `multiplier().as_f64()` = `10^SCALE` as an `f64`. `10^SCALE` exceeds
//! `f64::MAX` (~1.798e308) at **`SCALE >= 309`**, where `as_f64` accumulates
//! to `+inf`; the raw overflows too, `inf/inf` is NaN, and `from_f64(NaN)`
//! returns `ZERO`. So above that scale `_fast` does not compute a wrong
//! answer — it returns zero, quickly. Its timings are still real timings of
//! the shipped method, but they price a conversion, not a transcendental.
//! Long before that, `f64`'s ~15-17 significant digits are already fewer than
//! the storage carries at every tier from D18<17> up.
//!
//! # INSTRUMENT GAPS (do not read the map past these)
//!
//! * **Operand clone overhead.** `compare_all` clones the input through
//!   `black_box` on every iteration. At D1232 that is a 512-byte memcpy per
//!   candidate per iteration (1 KB for the binary ops), which is a
//!   double-digit-nanosecond constant added to BOTH sides. It is negligible
//!   against a millisecond-scale `_strict` and material against a
//!   nanosecond-scale `_fast`, so it **understates** the ratio at the wide
//!   tiers. Every ratio here is therefore a lower bound on fast's advantage.
//! * **`_strict` and `_fast` return DIFFERENT VALUES by design.** `fast` is
//!   not correctly rounded; that is what it is for. There is no bit-identity
//!   wall here and none is asserted — unlike an algorithm A/B, where
//!   agreement is the validity gate.
//! * The coarse re-time inside `compare_all` runs a fixed pass count and
//!   ignores criterion's name filter, so a criterion filter alone does not
//!   bound the wall time. Set `SF_AB_ONLY` to a comma-separated list of group
//!   substrings to skip whole cells and chunk a long sweep.
//!
//! Run with:
//! `cargo bench --features "std strict fast wide x-wide xx-wide" --bench strict_fast_ab`
//! `SF_AB_ONLY=D18,D38 cargo bench ... --bench strict_fast_ab`

use criterion::Criterion;
use decimal_scaled::{D18, D38, D57, D76, D115, D153, D230, D307, D462, D616, D924, D1232};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

// ── operand construction ────────────────────────────────────────────────

/// Marks an operand that is fed to a LOGARITHM's binary range reduction
/// (`ln`, `log`, `log2`, `log10`, `log1p`, and `powf`'s base). Those carry
/// the extra `raw >= 257` obligation at `SCALE 0`.
const LN_ARG: bool = true;
/// Marks an operand that never reaches a logarithm's range reduction.
const PLAIN_ARG: bool = false;

/// `a` = `1/9` truncated to `scale` digits (`0.111…1`); `333` at `SCALE 0`.
/// Last digit 1 at every scale, so `raw` is odd and `raw % 5 == 1`.
fn op_a(scale: u32) -> String {
    if scale == 0 { "333".to_string() } else { format!("0.{}", "1".repeat(scale as usize)) }
}

/// `b` = `7/3` truncated to `scale` digits (`2.333…3`); `2333` at `SCALE 0`.
/// Last digit 3 at every scale. Above 1 (legal for `acosh`) and past `pi/2`
/// (so the trig kernels pay a real range reduction).
fn op_b(scale: u32) -> String {
    if scale == 0 { "2333".to_string() } else { format!("2.{}", "3".repeat(scale as usize)) }
}

/// `a`, except at `SCALE 0` where it becomes `7` — for the ops whose result
/// grows like `e^x` and would overflow the narrow tiers at `a = 333`.
fn op_small(scale: u32) -> String {
    if scale == 0 { "7".to_string() } else { op_a(scale) }
}

/// `b`, except at `SCALE 0` where it becomes `3` — `powf`'s exponent, chosen
/// so `333^y` stays inside `i64` at D18<0>.
fn op_pexp(scale: u32) -> String {
    if scale == 0 { "3".to_string() } else { op_b(scale) }
}

/// The operand contract from this file's header, enforced on the literal
/// digits. `raw mod 2` and `raw mod 5` are both functions of the LAST decimal
/// digit alone, so the text carries the whole contract:
///
/// * `raw` ODD kills the power-of-two short-circuit (`m == 1`), which the ln
///   kernels take at `ln_tang.rs`'s `mantissa_w == one` arm and
///   `exp_generic::ln_fixed`'s equivalent;
/// * `raw % 5 != 0` kills the exact-Tang-table-boundary (`t == 0`) for
///   `scale >= 1`, because that trap needs `5^scale | raw`;
/// * at `scale == 0` there is no `5^scale` to lean on, so an ln-family
///   argument needs `raw >= 257` instead: with `raw` odd the boundary test
///   collapses to `k <= 7`, and `raw >= 257` forces `k >= 8`.
///
/// It also pins the fraction-digit count to `SCALE`, so `FromStr` is exact
/// and the text really is the stored `raw`. Panicking here is the point — a
/// silently degenerate operand is the defect that voided a whole map once
/// already.
fn assert_non_degenerate(text: &str, scale: u32, ln_arg: bool) {
    let (int_part, frac_part) = match text.split_once('.') {
        Some((i, f)) => (i, f),
        None => (text, ""),
    };
    assert_eq!(
        frac_part.len() as u32,
        scale,
        "operand must carry EXACTLY SCALE fraction digits so FromStr is exact \
         (SCALE {scale}, got {} fraction digits)",
        frac_part.len()
    );
    let last = text.as_bytes()[text.len() - 1];
    assert!(
        int_part.bytes().chain(frac_part.bytes()).any(|d| d != b'0'),
        "operand at scale {scale} is zero -- every range reduction special-cases it"
    );
    assert!(
        (last - b'0') % 2 == 1,
        "operand at scale {scale} must end in an ODD digit or `raw` can take \
         the power-of-two short-circuit (got last digit '{}')",
        last as char
    );
    if scale == 0 {
        if ln_arg {
            let value: u128 = int_part.parse().expect("scale-0 operand must be a small integer");
            assert!(
                value >= 257,
                "ln-family operand at scale 0 must be >= 257 or its mantissa \
                 terminates within 7 fraction bits and lands exactly on a Tang \
                 table boundary (t == 0); got {value}"
            );
        }
    } else {
        assert_ne!(
            last, b'5',
            "operand at scale {scale} must not end in 5 or `5^scale | raw` \
             becomes reachable and the Tang residual t can be exactly zero"
        );
    }
}

/// Validate the contract, then parse. `T` is the concrete `Dxxx<SCALE>`, so
/// the parse is exact by construction (fraction digits == SCALE).
fn operand<T: core::str::FromStr>(text: &str, scale: u32, ln_arg: bool) -> T {
    assert_non_degenerate(text, scale, ln_arg);
    match text.parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("a {}-char operand does not parse at SCALE {scale}", text.len()),
    }
}

/// `true` when the tier's `SCALE` admits a value strictly inside `(0, 1)`.
///
/// At `SCALE 0` only integers are representable, so `asin` / `acos` / `atanh`
/// — whose domain is `[-1, 1]` — have NO non-degenerate argument: the only
/// candidates are `0` and `+/-1`, all three of which those kernels
/// special-case. Those cells are skipped rather than measured degenerately.
fn has_unit_interval(scale: u32) -> bool {
    scale > 0
}

// ── cell runners ────────────────────────────────────────────────────────

/// `true` if this group is selected. `compare_all`'s coarse re-time runs a
/// fixed pass count and ignores criterion's own name filter, so cell
/// selection has to happen here for a filter to actually bound wall time.
/// `SF_AB_ONLY` is a comma-separated list of group-name substrings.
fn selected(group: &str) -> bool {
    match std::env::var("SF_AB_ONLY") {
        Ok(filter) if !filter.is_empty() => {
            filter.split(',').any(|part| !part.is_empty() && group.contains(part.trim()))
        }
        _ => true,
    }
}

/// One unary cell: `<op>_strict` against `<op>_fast` on the same operand, in
/// one process. No agreement assertion — the two return different values by
/// design, and that is the whole reason `fast` exists.
fn cell_unary<T: Clone + 'static>(
    c: &mut Criterion,
    group: &str,
    x: T,
    strict: fn(T) -> T,
    fast: fn(T) -> T,
) {
    if !selected(group) {
        return;
    }
    let candidates: Vec<(&'static str, Box<dyn Fn(T) -> T>)> = vec![
        ("strict", Box::new(move |v: T| strict(v))),
        ("fast", Box::new(move |v: T| fast(v))),
    ];
    compare_all(c, group, |_: &T| "x".to_string(), vec![x], candidates);
}

/// One binary cell (`log` base, `powf` exponent, `hypot` / `atan2` other).
fn cell_binary<T: Clone + 'static>(
    c: &mut Criterion,
    group: &str,
    x: T,
    y: T,
    strict: fn(T, T) -> T,
    fast: fn(T, T) -> T,
) {
    if !selected(group) {
        return;
    }
    let candidates: Vec<(&'static str, Box<dyn Fn((T, T)) -> T>)> = vec![
        ("strict", Box::new(move |p: (T, T)| strict(p.0, p.1))),
        ("fast", Box::new(move |p: (T, T)| fast(p.0, p.1))),
    ];
    compare_all(c, group, |_: &(T, T)| "x,y".to_string(), vec![(x, y)], candidates);
}

// ── the stamp ───────────────────────────────────────────────────────────

/// One unary `(op, width, scale)` cell. `SCALE` is a literal const generic so
/// each cell is its own monomorphisation, exactly as the policy sees it.
macro_rules! unary_cell {
    ($c:expr, $T:ty, $w:literal, $s:literal, $op:literal, $strict:ident, $fast:ident, $x:expr) => {
        cell_unary::<$T>(
            $c,
            &format!("{}_{}_s{}", $op, $w, $s),
            $x,
            |v: $T| <$T>::$strict(v),
            |v: $T| <$T>::$fast(v),
        )
    };
}

/// One binary `(op, width, scale)` cell.
macro_rules! binary_cell {
    ($c:expr, $T:ty, $w:literal, $s:literal, $op:literal, $strict:ident, $fast:ident,
     $x:expr, $y:expr) => {
        cell_binary::<$T>(
            $c,
            &format!("{}_{}_s{}", $op, $w, $s),
            $x,
            $y,
            |lhs: $T, rhs: $T| <$T>::$strict(lhs, rhs),
            |lhs: $T, rhs: $T| <$T>::$fast(lhs, rhs),
        )
    };
}

/// A unit-interval-only cell (`asin` / `acos` / `atanh`): measured where the
/// scale admits a non-degenerate argument, SKIPPED loudly where it does not.
macro_rules! unit_cell {
    ($c:expr, $T:ty, $w:literal, $s:literal, $op:literal, $strict:ident, $fast:ident, $x:expr) => {
        if has_unit_interval($s) {
            unary_cell!($c, $T, $w, $s, $op, $strict, $fast, $x);
        } else {
            println!(
                "SKIP [{}_{}_s{}]: domain is [-1, 1] and SCALE 0 represents only \
                 integers, so every legal argument (0, +/-1) is degenerate",
                $op, $w, $s
            );
        }
    };
}

/// All 27 op families that carry BOTH a `_strict` and a `_fast` variant, at
/// one `(width, scale)` cell. The list is the exact intersection of the two
/// method families: 23 unary + 4 binary. `sin_cos_strict` / `sinh_cosh_strict`
/// are excluded because they have no `_fast` sibling to race.
macro_rules! all_ops {
    ($c:expr, $Tn:ident, $w:literal, $s:literal) => {{
        type Cell = $Tn<$s>;
        let a: Cell = operand::<Cell>(&op_a($s), $s, LN_ARG);
        let b: Cell = operand::<Cell>(&op_b($s), $s, LN_ARG);
        let small: Cell = operand::<Cell>(&op_small($s), $s, PLAIN_ARG);
        let pexp: Cell = operand::<Cell>(&op_pexp($s), $s, PLAIN_ARG);

        // Logarithms — `a` is below 1, so the binary reduction takes `k < 0`.
        unary_cell!($c, Cell, $w, $s, "ln", ln_strict, ln_fast, a);
        unary_cell!($c, Cell, $w, $s, "log2", log2_strict, log2_fast, a);
        unary_cell!($c, Cell, $w, $s, "log10", log10_strict, log10_fast, a);
        unary_cell!($c, Cell, $w, $s, "log1p", log1p_strict, log1p_fast, a);
        binary_cell!($c, Cell, $w, $s, "log", log_strict, log_fast, a, b);

        // Exponentials — `small` keeps `e^x` inside the narrow tiers at s0.
        unary_cell!($c, Cell, $w, $s, "exp", exp_strict, exp_fast, small);
        unary_cell!($c, Cell, $w, $s, "exp2", exp2_strict, exp2_fast, small);
        unary_cell!($c, Cell, $w, $s, "expm1", expm1_strict, expm1_fast, small);
        binary_cell!($c, Cell, $w, $s, "powf", powf_strict, powf_fast, a, pexp);

        // Roots.
        unary_cell!($c, Cell, $w, $s, "sqrt", sqrt_strict, sqrt_fast, b);
        unary_cell!($c, Cell, $w, $s, "cbrt", cbrt_strict, cbrt_fast, b);
        binary_cell!($c, Cell, $w, $s, "hypot", hypot_strict, hypot_fast, a, b);

        // Circular — `b` is past pi/2, so the range reduction really runs.
        unary_cell!($c, Cell, $w, $s, "sin", sin_strict, sin_fast, b);
        unary_cell!($c, Cell, $w, $s, "cos", cos_strict, cos_fast, b);
        unary_cell!($c, Cell, $w, $s, "tan", tan_strict, tan_fast, b);
        unit_cell!($c, Cell, $w, $s, "asin", asin_strict, asin_fast, a);
        unit_cell!($c, Cell, $w, $s, "acos", acos_strict, acos_fast, a);
        unary_cell!($c, Cell, $w, $s, "atan", atan_strict, atan_fast, b);
        binary_cell!($c, Cell, $w, $s, "atan2", atan2_strict, atan2_fast, a, b);

        // Hyperbolic — `small` again for the three that grow like `e^x`.
        unary_cell!($c, Cell, $w, $s, "sinh", sinh_strict, sinh_fast, small);
        unary_cell!($c, Cell, $w, $s, "cosh", cosh_strict, cosh_fast, small);
        unary_cell!($c, Cell, $w, $s, "tanh", tanh_strict, tanh_fast, small);
        unary_cell!($c, Cell, $w, $s, "asinh", asinh_strict, asinh_fast, b);
        // acosh's domain is x >= 1, so `b` is the only legal operand here.
        unary_cell!($c, Cell, $w, $s, "acosh", acosh_strict, acosh_fast, b);
        unit_cell!($c, Cell, $w, $s, "atanh", atanh_strict, atanh_fast, a);

        // Angle conversions.
        unary_cell!($c, Cell, $w, $s, "to_degrees", to_degrees_strict, to_degrees_fast, a);
        unary_cell!($c, Cell, $w, $s, "to_radians", to_radians_strict, to_radians_fast, a);
    }};
}

/// One tier across the FIVE-point scale grid `{0, S/4, S/2, 3S/4, S}` with
/// `S = MAX_SCALE`, as literals so every cell is its own monomorphisation.
macro_rules! tier {
    ($c:expr, $Tn:ident, $w:literal,
     $s0:literal, $s1:literal, $s2:literal, $s3:literal, $s4:literal) => {{
        all_ops!($c, $Tn, $w, $s0);
        all_ops!($c, $Tn, $w, $s1);
        all_ops!($c, $Tn, $w, $s2);
        all_ops!($c, $Tn, $w, $s3);
        all_ops!($c, $Tn, $w, $s4);
    }};
}

// One function per tier: 27 ops x 5 scales is a large body, and splitting it
// keeps each monomorphisation batch independently compilable. Narrow first,
// so a truncated run still leaves the tiers where an f64 bridge could
// plausibly be useful complete.
fn tier_d18(c: &mut Criterion) {
    tier!(c, D18, "D18", 0, 4, 8, 12, 17);
}
fn tier_d38(c: &mut Criterion) {
    tier!(c, D38, "D38", 0, 9, 18, 27, 37);
}
fn tier_d57(c: &mut Criterion) {
    tier!(c, D57, "D57", 0, 14, 28, 42, 56);
}
fn tier_d76(c: &mut Criterion) {
    tier!(c, D76, "D76", 0, 18, 37, 56, 75);
}
fn tier_d115(c: &mut Criterion) {
    tier!(c, D115, "D115", 0, 28, 57, 85, 114);
}
fn tier_d153(c: &mut Criterion) {
    tier!(c, D153, "D153", 0, 38, 76, 114, 152);
}
fn tier_d230(c: &mut Criterion) {
    tier!(c, D230, "D230", 0, 57, 114, 171, 229);
}
fn tier_d307(c: &mut Criterion) {
    tier!(c, D307, "D307", 0, 76, 153, 229, 306);
}
fn tier_d462(c: &mut Criterion) {
    tier!(c, D462, "D462", 0, 115, 230, 345, 461);
}
fn tier_d616(c: &mut Criterion) {
    tier!(c, D616, "D616", 0, 153, 307, 461, 615);
}
fn tier_d924(c: &mut Criterion) {
    tier!(c, D924, "D924", 0, 230, 461, 692, 923);
}
fn tier_d1232(c: &mut Criterion) {
    tier!(c, D1232, "D1232", 0, 307, 615, 923, 1231);
}

fn benches(c: &mut Criterion) {
    tier_d18(c);
    tier_d38(c);
    tier_d57(c);
    tier_d76(c);
    tier_d115(c);
    tier_d153(c);
    tier_d230(c);
    tier_d307(c);
    tier_d462(c);
    tier_d616(c);
    tier_d924(c);
    tier_d1232(c);
}

fn main() {
    // `configure_from_args` so a criterion name filter is honoured at all --
    // without it a `--bench <name> -- <filter>` argument is silently ignored
    // and the whole suite runs anyway. `SF_AB_ONLY` is still what bounds wall
    // time, because the coarse re-time inside `compare_all` is not filtered.
    let mut c = micro_criterion().configure_from_args();
    benches(&mut c);
    c.final_summary();
}
