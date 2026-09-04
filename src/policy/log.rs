// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Log-base policy — the per-(N, SCALE) algorithm matcher for the
//! arbitrary-base decimal logarithm `log(self, base)`.
//!
//! `D<Int<N>, SCALE>::log_with(base, mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`.
//!
//! # One composition, two guards — `LnDivide` and `LnDivideConditioned`
//!
//! `log(self, base) = ln(self) / ln(base)` at every tier and scale. The
//! matcher's value axis is the BASE: [`select`] is `Select::ByValue` on it,
//! and [`log_near_one_base`] classifies by the base's conditioning number
//! `k = ceil(-log10 |b - 1|)` (`algos::log::log_ln_divide::near_one_digits`):
//!
//! - `k == 0` (every ordinary base, `|b - 1| >= 0.1`) — `LnDivide`, the
//!   fixed-guard shells: the narrow tiers through the
//!   `crate::algos::log::log_ln_divide` kernels (D18 widens to Int<2>; D38
//!   calls `ln::ln_series_2limb`), the wide tiers through the per-tier
//!   `wide_trig_<tier>::log_strict_with_kernel` free functions (emitted by
//!   `decl_wide_transcendental!`), reached by a `match N` with `resize_to`
//!   bridges (identity at the matched `N`). Unchanged.
//! - `k > 0` (a base within 0.1 of 1) — `LnDivideConditioned`, the ONE
//!   generic `log_ln_divide_conditioned` kernel at guard `30 + 2k`, in a
//!   work integer the policy chooses from `k`
//!   ([`ln_divide_conditioned_routed`]): the tier's composition width
//!   `Wagm` while its digit budget carries the lift, then the next-wider
//!   `Wexp`, then one documented widest width where even `Wexp` lacks
//!   capacity; the narrow tiers run it in the `Int<24>` Ziv work integer
//!   and fit the result back (`None` = out of storage range, their
//!   `checked_` contract). Why a base near 1 needs the lift, and the
//!   measured law behind it, is on the kernel's module.
//!
//! `Schoolbook` is the unrouted naive `ln(x)/ln(b)` reference.

use crate::algos::log::log_ln_divide::{lifted_guard, near_one_digits};
use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    LnDivide,
    LnDivideConditioned,
    #[allow(dead_code)]
    Schoolbook,
}

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    #[allow(dead_code)]
    ByAlgorithm(Algorithm),
    ByValue(fn(&Int<N>) -> Algorithm),
}

/// The value classifier — `log`'s runtime arm is keyed on the BASE (the
/// operand whose conditioning sizes the guard), so [`checked_dispatch`]
/// hands it `braw`, never `raw`. One comparison against `10^(SCALE - 1)`
/// settles every ordinary base (tier-1 work); only a base within 0.1 of 1
/// pays the `pow10` bisection that pins `k`.
#[inline]
fn log_near_one_base<const N: usize, const SCALE: u32>(base_raw: &Int<N>) -> Algorithm {
    if near_one_digits::<Int<N>>(*base_raw, SCALE) == 0 {
        Algorithm::LnDivide
    } else {
        Algorithm::LnDivideConditioned
    }
}

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    Select::ByValue(log_near_one_base::<N, SCALE>)
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    checked_dispatch::<N, SCALE>(raw, braw, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("log", SCALE)
    })
}

/// The `checked` primitive under [`dispatch`]: exact out-of-range
/// `None` on the narrow tiers; the wide arms call the per-tier kernel
/// shells, whose internal out-of-range panic is not yet threaded
/// through. Domain errors
/// stay kernel panics — the `checked_` surface prechecks the domain.
#[inline]
#[must_use]
pub(crate) fn checked_dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        // `log`'s value axis is the base — see `log_near_one_base`.
        Select::ByValue(choose) => choose(&braw),
    };
    match algo {
        Algorithm::LnDivide => ln_divide_routed::<N, SCALE>(raw, braw, mode),
        Algorithm::LnDivideConditioned => ln_divide_conditioned_routed::<N, SCALE>(raw, braw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, braw, mode),
    }
}

/// The conditioned arm: recover `k` (the classifier's verdict is a tag),
/// lift the guard, and run the ONE generic kernel at a work width chosen
/// from `k`.
///
/// ── THE WORK WIDTH, PER TIER ──
///
/// The lifted working scale `SCALE + 30 + 2k` outruns the tier's composition
/// width `Wagm` in its worst band. Two tests, both in
/// `algos::log::log_ln_divide`: `fits_budget` (the `8 · limbs` digit budget
/// the Ziv walker escalates within — the width the walker needs) and
/// `fits_capacity` (`2w + 40 <= 19 · limbs` — the width the arithmetic
/// needs; below it the limb ops would wrap into a plausible wrong value).
/// The ladder is `Wagm`, then `Wexp` (the tier's next-wider work integer),
/// then one widest width, chosen by budget and ASSERTED by capacity.
/// `needed_digits = SCALE + 30 + 3k + 12` at the worst cell whose result can
/// still fit storage (`k = min(SCALE, D - SCALE)`), and the capacity corner
/// `2w + 40` at `k = SCALE = max` (`w = 3·SCALE + 30`; a result that only
/// fits when `x` is itself within `10^-(k-1)` of 1). `k <= SCALE` because the
/// base must be representable, so this corner is the LARGEST working scale
/// any legal input can ask for — a finite, known width per tier, which is
/// why no arbitrary-precision fallback is needed:
///
///   tier    Wagm      budget  worst   Wexp      budget  worst  Wmax      capacity corner
///   D57     Int<16>    128     155    Int<32>    256     yes   Int<32>    436 <=  608
///   D76     Int<16>    128     194    Int<32>    256     yes   Int<32>    550 <=  608
///   D115    Int<32>    256     271    Int<64>    512     yes   Int<64>    784 <= 1216
///   D153    Int<32>    256     347    Int<64>    512     yes   Int<64>   1012 <= 1216
///   D230    Int<48>    384     502    Int<96>    768     yes   Int<96>   1474 <= 1824
///   D307    Int<64>    512     655    Int<128>  1024     yes   Int<128>  1936 <= 2432
///   D462    Int<64>    512     966    Int<128>  1024     yes   Int<176>  2866 <= 3344
///   D616    Int<128>  1024    1274    Int<256>  2048     yes   Int<256>  3790 <= 4864
///   D924    Int<192>  1536    1890    Int<256>  2048     yes   Int<512>  5638 <= 9728
///   D1232   Int<256>  2048    2506    Int<512>  4096     yes   Int<512>  7486 <= 9728
///
/// D462 and D924 are the two tiers whose `Wexp` cannot hold the capacity
/// corner (`Int<128>` 2432 < 2866; `Int<256>` 4864 < 5638), so their
/// widest width is the next `ComputeLimbs` member above it. Every corner
/// fits its `Wmax`, so the assert never fires for a legal input; it is the
/// wall that turns a future width pairing that broke this table into a
/// panic instead of a wrong digit — not a contract limit.
#[inline]
fn ln_divide_conditioned_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    let k = near_one_digits::<Int<N>>(braw, SCALE);
    match N {
        1 | 2 => conditioned_narrow::<N, SCALE>(raw, braw, mode, k),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(conditioned_wide::<3, crate::types::widths::wide_trig_d57::Wagm, crate::types::widths::wide_trig_d57::Wexp, crate::types::widths::wide_trig_d57::Wexp, SCALE, 100>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(conditioned_wide::<4, crate::types::widths::wide_trig_d76::Wagm, crate::types::widths::wide_trig_d76::Wexp, crate::types::widths::wide_trig_d76::Wexp, SCALE, 400>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(conditioned_wide::<6, crate::types::widths::wide_trig_d115::Wagm, crate::types::widths::wide_trig_d115::Wexp, crate::types::widths::wide_trig_d115::Wexp, SCALE, 200>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(conditioned_wide::<8, crate::types::widths::wide_trig_d153::Wagm, crate::types::widths::wide_trig_d153::Wexp, crate::types::widths::wide_trig_d153::Wexp, SCALE, 200>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(conditioned_wide::<12, crate::types::widths::wide_trig_d230::Wagm, crate::types::widths::wide_trig_d230::Wexp, crate::types::widths::wide_trig_d230::Wexp, SCALE, 400>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(conditioned_wide::<16, crate::types::widths::wide_trig_d307::Wagm, crate::types::widths::wide_trig_d307::Wexp, crate::types::widths::wide_trig_d307::Wexp, SCALE, 400>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(conditioned_wide::<24, crate::types::widths::wide_trig_d462::Wagm, crate::types::widths::wide_trig_d462::Wexp, Int<176>, SCALE, 400>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(conditioned_wide::<32, crate::types::widths::wide_trig_d616::Wagm, crate::types::widths::wide_trig_d616::Wexp, crate::types::widths::wide_trig_d616::Wexp, SCALE, 400>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(conditioned_wide::<48, crate::types::widths::wide_trig_d924::Wagm, crate::types::widths::wide_trig_d924::Wexp, Int<512>, SCALE, 400>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode, k).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(conditioned_wide::<64, crate::types::widths::wide_trig_d1232::Wagm, crate::types::widths::wide_trig_d1232::Wexp, crate::types::widths::wide_trig_d1232::Wexp, SCALE, 400>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode, k).resize_to::<Int<N>>()),
        _ => conditioned_narrow::<N, SCALE>(raw, braw, mode, k),
    }
}

/// The wide conditioned path: `Wagm` while its budget carries the lift,
/// else `Wexp`, else `Wmax` — asserted for capacity (see
/// [`ln_divide_conditioned_routed`]). `CAP` is the tier's Tang artanh cap,
/// the same value `policy::ln` threads.
#[cfg(feature = "_wide-support")]
#[inline]
fn conditioned_wide<
    const N: usize,
    Wagm: BigInt,
    Wexp: BigInt,
    Wmax: BigInt,
    const SCALE: u32,
    const CAP: u128,
>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
    k: u32,
) -> Int<N>
where
    <Wagm as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <Wexp as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <Wmax as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::log::log_ln_divide::{fits_budget, fits_capacity};
    if fits_budget(SCALE, k, <Wagm as BigInt>::LIMBS) {
        conditioned_at::<N, Wagm, SCALE, CAP>(raw, braw, mode, k)
    } else if fits_budget(SCALE, k, <Wexp as BigInt>::LIMBS) {
        conditioned_at::<N, Wexp, SCALE, CAP>(raw, braw, mode, k)
    } else {
        // Unreachable for any legal input, NOT a contract limit: the base is
        // representable at `SCALE`, so `|b - 1| >= 10^-SCALE` and `k <= SCALE`;
        // the largest lift any input can ask for is therefore `w = 3·SCALE + 30`,
        // and `2w + 40` fits every tier's `Wmax` (the table above). It stays
        // because the defect this arm fixes was a SILENT wrong digit: if a
        // future width pairing broke the table, this turns silent into loud.
        assert!(
            fits_capacity(SCALE, k, <Wmax as BigInt>::LIMBS),
            "log: a base within 10^-{k} of 1 at scale {SCALE} exceeds the widest \
             composition work integer's capacity — see policy::log"
        );
        conditioned_at::<N, Wmax, SCALE, CAP>(raw, braw, mode, k)
    }
}

/// One wide conditioned run at work width `Wk`. The natural-log core follows
/// `policy::ln`'s verdict for this cell — Tang wherever the baked table
/// reaches the lifted working scale, the kept Series alternative past it
/// (the two widest tiers above scale ~702, where `w = 3·SCALE + 30`
/// outruns the table's 2138 digits).
#[cfg(feature = "_wide-support")]
#[inline]
fn conditioned_at<const N: usize, Wk: BigInt, const SCALE: u32, const CAP: u128>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
    k: u32,
) -> Int<N>
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::log::log_ln_divide::{
        log_ln_divide_conditioned, series_core, tang_core, tang_table_reaches,
    };
    let guard = lifted_guard(k);
    if const { super::ln::is_tang::<N, SCALE>() } && tang_table_reaches(SCALE + guard) {
        log_ln_divide_conditioned::<N, Wk, SCALE>(raw, braw, mode, guard, tang_core::<Wk, SCALE, CAP>())
    } else {
        log_ln_divide_conditioned::<N, Wk, SCALE>(raw, braw, mode, guard, series_core::<Wk, SCALE>())
    }
}

/// The narrow conditioned path: the same kernel in the `Int<24>` narrow Ziv
/// work integer (`narrow_ziv::WZiv`, the width every narrow near-tie probe
/// already runs in), with `Int<24>` also standing as the range-checked
/// width so the walker never panics, then fitted back to `Int<N>` —
/// `None` when the result does not fit storage, the narrow tiers' `checked_`
/// contract. Series core (the narrow `ln` verdict). Capacity is never in
/// question: the worst narrow lift (`D38`, `k = SCALE = 38`, `w = 144`)
/// asks `328` of `Int<24>`'s `456` digits.
#[inline]
fn conditioned_narrow<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
    k: u32,
) -> Option<Int<N>> {
    use crate::algos::log::log_ln_divide::{log_ln_divide_conditioned, series_core};
    use crate::algos::support::narrow_ziv::WZiv;
    let wide = log_ln_divide_conditioned::<24, WZiv, SCALE>(
        raw.resize_to::<WZiv>(),
        braw.resize_to::<WZiv>(),
        mode,
        lifted_guard(k),
        series_core::<WZiv, SCALE>(),
    );
    fit_from_ziv::<N>(wide)
}

/// Storage fit of an `Int<24>` result — `super::narrow_fit`'s round-trip
/// check from the Ziv width instead of `Int<2>`.
#[inline]
fn fit_from_ziv<const N: usize>(wide: Int<24>) -> Option<Int<N>> {
    let out = wide.resize_to::<Int<N>>();
    if out.resize_to::<Int<24>>() != wide {
        return None;
    }
    Some(out)
}

#[inline]
fn ln_divide_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        // N == 1 runs the same D38-width log kernel the widen-narrow
        // `log_ln_divide_d18` composition resolves to; the `narrow_fit`
        // reproduces its try-into fit check as `None`.
        1 | 2 => crate::algos::log::log_ln_divide::log_ln_divide_d38::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::types::widths::wide_trig_d57::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::types::widths::wide_trig_d76::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::types::widths::wide_trig_d115::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::types::widths::wide_trig_d153::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::types::widths::wide_trig_d230::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::types::widths::wide_trig_d307::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::types::widths::wide_trig_d462::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::types::widths::wide_trig_d616::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::types::widths::wide_trig_d924::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::types::widths::wide_trig_d1232::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => crate::algos::log::log_ln_divide::log_ln_divide_d38::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => super::narrow_fit::<N>(crate::algos::log::log_schoolbook::log_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode)),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => super::narrow_fit::<N>(crate::algos::log::log_schoolbook::log_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode)),
    }
}

