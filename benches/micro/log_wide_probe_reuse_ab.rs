// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier `log(x, base)` base-guard PROBE-REUSE A/B — does the strict
//! kernel still evaluate its two natural logs twice?
//!
//! # The defect this measures
//!
//! `log_strict_with_kernel` (`src/macros/wide_transcendental.rs`) evaluates
//! `ln(x)` and `ln(base)` at the base working scale `SCALE + GUARD` for its
//! exact-power pin, then hands a `recompute` closure to the directed-rounding
//! Ziv walker. The walker's FIRST probe is always `recompute(GUARD)`
//! (`wide_trig_core::round_to_storage_directed_tagged_impl_g` — both the
//! nearest and the directed branch open with `recompute(base_guard_digits)`),
//! and at `guard_digits == GUARD` that probe is BIT-IDENTICAL to the pin's:
//! `working_scale == SCALE + GUARD`, and `to_work_scaled_agm(v, GUARD)` is
//! `to_work_agm(v)` by definition. So the kernel computed `ln` FOUR times
//! where two suffice, and divided twice where one divide suffices. The fix
//! memoises the pin's ratio and hands it back for `guard_digits == GUARD`;
//! only an ESCALATED Ziv rung recomputes. `log2` and `log10` carry the same
//! duplicate against their const `ln 2` / `ln 10` divisor.
//!
//! # The candidates — all three in ONE process on ONE machine
//!
//! A micro-bench ranks candidates in one binary, so the runner's absolute
//! speed cancels out of every ratio here. (That is exactly what the surface
//! sweep cannot do: it puts each cell on its own runner VM.)
//!
//! * **`log`** — the production `log_strict_with` as this branch builds it.
//! * **`core`** — ONE base-guard core evaluation on its own:
//!   `log_approx_with(base, GUARD + 1, mode)`, which is exactly `2 ln + 1
//!   div + 1 narrowing` at `SCALE + GUARD + 1` with no pin and no Ziv walk.
//!   This is the price of the duplicate that was removed.
//! * **`log_dup`** — the PRE-change shape reconstructed from the public API:
//!   one discarded `core` evaluation followed by the production `log`. It
//!   overstates the old kernel by one narrowing and by one decimal digit of
//!   working scale, so `log_dup / log` is a mild UPPER bound on the gain.
//!
//! # Reading the ranking — three signatures, and only one is correct
//!
//! | observed | meaning |
//! |---|---|
//! | `log` ~ `core` (within ~1.4x), `log_dup` ~ 2x `log` | the memo FIRED — expected |
//! | `log` ~ 2x `core` | the memo did NOT fire; the duplicate is still there |
//! | `core` ~ 2x `log` | the exact-power pin returned early — the cell is degenerate and measures the pin, not the kernel |
//!
//! The third row is why the operand contract below is enforced rather than
//! assumed.
//!
//! # OPERAND CONTRACT — read this before changing `log_inputs`
//!
//! Derived and enforced identically in
//! `benches/micro/ln_wide_series_tang_ab.rs`; the rule is reproduced here
//! because `log` calls `ln` twice and BOTH operands must clear it.
//!
//! Both `ln` kernels short-circuit on the same degenerate operand classes,
//! keyed on the binary range reduction `v = 2^k * m`, `m` in [1, 2):
//!
//! * **Trap 1 — exact power of two (`m == 1`).** `ln_tang.rs` and
//!   `exp_generic::ln_fixed` both return `k * ln2` from a one-word product;
//!   neither the artanh series nor the Brent sqrt reduction runs. `0.5`, `1`,
//!   `2.0`, `4` all land here at every width and every scale.
//! * **Trap 2 — exact Tang table boundary (`t == 0`), Tang only.** Tang picks
//!   `f_i = 1 + i/M` with `M = 128`; when `m` is an exact multiple of `1/128`
//!   the residual `t` is exactly zero and the artanh series breaks on its
//!   first term. `7.5` is `2^2 * 1.875` and `1.875 = 1 + 112/128` exactly.
//!
//! > **`raw` ODD and `raw % 5 != 0` defeats BOTH traps at every `SCALE >= 1`,
//! > at every width, for every `k`.** At `SCALE == 0` there is no `10^SCALE`
//! > factor to lean on and the rule becomes **`raw` odd AND `raw >= 257`**.
//!
//! [`assert_non_degenerate`] enforces exactly that on every measured operand,
//! so the defect cannot silently return. This matters acutely for `log`: the
//! tracked surface benches `log(2.0, 7.0)` and BOTH ends are degenerate —
//! `2.0` is Trap 1, and `7.0` gives `m = 1.75 = 1 + 96/128`, Tang table index
//! 96 exactly, so `t` is exactly zero. That cell measures two fast paths
//! racing, at any width and any scale.
//!
//! The measured operands are `1/3` and `7/3` truncated to `SCALE` digits —
//! `0.333...3` and `2.333...3`. Both end in 3, so neither is divisible by 2
//! or 5 at any scale: the contract holds by construction rather than by luck.
//! Both are repeating decimals, so every fraction digit is significant and the
//! series runs its true term count; one sits below 1 (`k < 0`) and one above
//! (`k > 0`); and `log_{2.333}(0.333) ~ -1.2966` is not an integer, so the
//! exact-power pin cannot fire. SCALE 0 uses `333` and `2333` — the same
//! digit strings read as integers, both odd and both >= 257.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench log_wide_probe_reuse_ab`
//!
//! `compare_all`'s coarse re-time runs a fixed pass count and ignores
//! criterion's name filter, so a criterion filter alone does not bound the
//! wall time. Set `LOG_AB_ONLY` to a substring of the group name to skip
//! whole cells and chunk a long sweep:
//! `LOG_AB_ONLY=log_d1232 cargo bench ... --bench log_wide_probe_reuse_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::int_from_mag_limbs;
use decimal_scaled::{Int, RoundingMode, D};
use std::hint::black_box;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;

/// Every wide tier declares `GUARD = 30`
/// (`src/macros/wide_transcendental.rs`, `pub(crate) const GUARD: u32 = 30`).
const GUARD: u32 = 30;

/// The approx guard that prices ONE base-guard core evaluation. It MUST NOT
/// equal `GUARD`: `policy::log::ln_divide_with_routed` short-circuits
/// `working_digits == GUARD` straight to the strict kernel, which would price
/// the whole function again instead of its core. `GUARD + 1` is one extra
/// decimal digit of working scale — the same limb count, the same work rung,
/// the same kernel.
const CORE_GUARD: u32 = GUARD + 1;

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
/// must defeat BOTH degeneracy traps at its `(N, scale)` cell. Panicking here
/// is the point — a silently degenerate operand is exactly the defect that
/// voided the tracked surface's `log` row.
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

/// Build a validated non-degenerate operand at this cell.
fn build_raw<const N: usize>(x_num: u64, x_den: u64, scale: u32, label: &str) -> Int<N> {
    let mag = build_mag::<N>(x_num, x_den, scale);
    assert_non_degenerate::<N>(&mag, label, scale);
    int_from_mag_limbs::<N>(&mag)
}

/// One measured `(x, base)` pair at a concrete tier and scale.
#[derive(Clone, Copy)]
struct Pair<const N: usize, const SCALE: u32> {
    label: &'static str,
    x: D<Int<N>, SCALE>,
    base: D<Int<N>, SCALE>,
}

/// The single measured `log` argument pair at decimal `SCALE`, both halves
/// satisfying the operand contract in this file's header:
///
/// * `x` = `1/3` truncated to `SCALE` digits = `0.333...3` (below 1, `k < 0`);
/// * `base` = `7/3` truncated to `SCALE` digits = `2.333...3` (above 1,
///   `k > 0`).
///
/// `log_{2.333}(0.333) ~ -1.2966` needs one integer digit, so the result fits
/// every tier through `SCALE = cap - 1`, and it is not an integer, so the
/// exact-power pin cannot short-circuit the cell.
///
/// SCALE 0 cannot use those (`floor(1/3) = 0` is outside `ln`'s domain and
/// `floor(7/3) = 2` is an exact power of two), so it uses `333` and `2333` —
/// the same digit strings read as integers, both odd and both >= 257.
fn log_inputs<const N: usize, const SCALE: u32>() -> Vec<Pair<N, SCALE>> {
    let (x, base) = if SCALE == 0 {
        (build_raw::<N>(333, 1, 0, "x333"), build_raw::<N>(2333, 1, 0, "b2333"))
    } else {
        (build_raw::<N>(1, 3, SCALE, "x_lo"), build_raw::<N>(7, 3, SCALE, "b_hi"))
    };
    vec![Pair { label: "x1_3_b7_3", x: D(x), base: D(base) }]
}

/// `true` if this group is selected. `compare_all`'s coarse re-time ignores
/// criterion's own name filter, so cell selection has to happen here for a
/// filter to actually bound the wall time.
fn selected(group: &str) -> bool {
    match std::env::var("LOG_AB_ONLY") {
        Ok(filter) if !filter.is_empty() => group.contains(&filter),
        _ => true,
    }
}

/// Emit one `(tier, scale)` cell: `log` vs `core` vs the reconstructed
/// pre-change `log_dup`, over the single non-degenerate operand pair.
///
/// A free macro rather than a generic fn because `D<Int<N>, SCALE>`'s
/// transcendental surface is emitted per tier by `decl_wide_transcendental!`
/// and is not reachable through a `const N` generic.
macro_rules! cell_log {
    ($c:expr, $n:literal, $name:literal, $scale:literal) => {{
        let group = concat!("log_", $name, "_s", stringify!($scale));
        if selected(group) {
            type In = Pair<$n, $scale>;
            type Out = D<Int<$n>, $scale>;
            let runs: Vec<(&'static str, Box<dyn Fn(In) -> Out>)> = vec![
                // Production, post-fix: one base-guard core evaluation,
                // reused by the walker's first probe.
                ("log", Box::new(|p: In| p.x.log_strict_with(p.base, MODE))),
                // ONE base-guard core evaluation on its own: 2 ln + 1 div +
                // 1 narrowing, no pin and no Ziv walk. The price of the
                // duplicate that was removed.
                ("core", Box::new(|p: In| p.x.log_approx_with(p.base, CORE_GUARD, MODE))),
                // The PRE-change shape: one DISCARDED base-guard core
                // evaluation, then the rest of the function.
                (
                    "log_dup",
                    Box::new(|p: In| {
                        black_box(p.x.log_approx_with(p.base, CORE_GUARD, MODE));
                        p.x.log_strict_with(p.base, MODE)
                    }),
                ),
            ];
            compare_all($c, group, |p: &In| p.label.to_string(), log_inputs::<$n, $scale>(), runs);
        }
    }};
}

/// Sweep one tier across the FIVE coarse scale points
/// `{0, S/4, S/2, 3S/4, S-1}` (the owner-standard sampling) as literals.
macro_rules! tier {
    ($c:expr, $n:literal, $name:literal,
     $s0:literal, $s1:literal, $s2:literal, $s3:literal, $s4:literal) => {{
        cell_log!($c, $n, $name, $s0);
        cell_log!($c, $n, $name, $s1);
        cell_log!($c, $n, $name, $s2);
        cell_log!($c, $n, $name, $s3);
        cell_log!($c, $n, $name, $s4);
    }};
}

fn benches(c: &mut Criterion) {
    // Narrowest tier first, cheapest scale first, so a run that is cut short
    // still leaves a usable map rather than only the widest cells.
    // Per tier: scales {0, S/4, S/2, 3S/4, S-1}, top point = the tier's
    // max-scale extreme (`cap - 1`).
    tier!(c, 3, "d57", 0, 14, 28, 42, 56);
    tier!(c, 4, "d76", 0, 19, 38, 57, 75);
    tier!(c, 6, "d115", 0, 28, 57, 86, 114);
    tier!(c, 8, "d153", 0, 38, 76, 114, 152);
    tier!(c, 12, "d230", 0, 57, 115, 172, 229);
    tier!(c, 16, "d307", 0, 76, 153, 230, 306);
    tier!(c, 24, "d462", 0, 115, 231, 346, 461);
    tier!(c, 32, "d616", 0, 154, 308, 462, 615);
    tier!(c, 48, "d924", 0, 231, 462, 693, 923);
    tier!(c, 64, "d1232", 0, 308, 616, 924, 1231);
}

fn main() {
    // `configure_from_args` so a criterion name filter is honoured at all --
    // without it a `--bench <name> -- <filter>` argument is silently ignored
    // and the whole suite runs. `LOG_AB_ONLY` is still what bounds wall time,
    // because the coarse re-time inside `compare_all` is not filtered.
    let mut c = micro_criterion().configure_from_args();
    benches(&mut c);
    c.final_summary();
}
