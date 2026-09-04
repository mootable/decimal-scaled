//! The narrow (D18 / D38) strict transcendentals must not outrun their
//! compute scratch in ANY feature configuration.
//!
//! The narrow tiers do not compute at their storage width. `policy::expm1`
//! (and every other narrow strict transcendental) lifts the argument into
//! `narrow_ziv::WZiv = Int<24>` — 1536 bits, TWELVE times D38's own
//! `Int<2>` storage — and the rescale that ends each Ziv probe asks that
//! work integer's scratch carrier for a `single_u128()` magnitude buffer.
//!
//! Every buffer in the `ComputeLimbs` family is sized per-`N` under
//! `exact-scratch`, but the no-`exact-scratch` build form is ONE build-max
//! blanket derived from `MAX_WORK_N` — the widest *storage* width the
//! build's tier features enable. With no tier feature at all `MAX_WORK_N`
//! is 2, so the blanket's `single_u128` is 4 u128 limbs while `Int<24>`
//! needs 12, and the rescale panicked with
//! `range end index 12 out of range for slice of length 4`.
//!
//! `WZiv` is a fixed `Int<24>` regardless of features, so the mismatch is
//! not scale- or value-specific: it is reached by the first narrow strict
//! transcendental call any such build makes.
//!
//! Run this under the no-`exact-scratch` configuration it exists for:
//!
//! ```text
//! cargo test -p decimal-scaled --no-default-features --test narrow_work_scratch
//! ```
//!
//! Use `-p decimal-scaled`, never `--workspace`: under `--workspace` cargo's
//! feature unification pulls the default features (and with them
//! `exact-scratch`) back in through the dev-dependency graph, so the lib
//! compiles WITH exact scratch and this file passes without exercising the
//! configuration it exists to cover.

use decimal_scaled::{D18, D38};

type W38 = D38<20>;
type W18 = D18<16>;

fn d38(v: i64) -> W38 {
    W38::try_from(v).unwrap()
}

fn d18(v: i64) -> W18 {
    W18::try_from(v).unwrap()
}

/// `expm1_strict` on both narrow widths, across the sign/regime split the
/// `expm1` matcher routes on (`|x| <= 1` takes the Taylor series, `|x| > 1`
/// takes the `e^x - 1` composition). Both regimes run in `WZiv`, so either
/// one alone would catch the undersized buffer; naming both keeps the check
/// honest if the routing wall ever moves.
#[test]
fn narrow_expm1_strict_does_not_outrun_its_work_scratch() {
    let half = d38(1) / d38(2);
    let two = d38(2);

    // The assertion is that these RETURN. `expm1` is strictly increasing and
    // `expm1(x) > x` for every non-zero `x`, which is enough of a shape check
    // to prove a real value came back rather than a zero.
    for x in [half, -half, two, -two] {
        assert!(x.expm1_strict() > x, "expm1(x) must exceed x, x = {x}");
    }

    // D18 shares the same `WZiv` work integer through the same policy.
    let half18 = d18(1) / d18(2);
    assert!(
        half18.expm1_strict() > half18,
        "D18 expm1(0.5) must exceed 0.5"
    );
}

/// The same scratch carrier backs the rest of the narrow strict surface, so
/// a buffer sized from the wrong width takes them all down together. These
/// are the neighbours of the reported failure, not a broader sweep.
#[test]
fn narrow_strict_neighbours_share_the_work_scratch() {
    let half = d38(1) / d38(2);
    let zero = d38(0);

    // log1p / expm1 are the inverse pair; exp and ln are their unreduced
    // siblings. All lift into `WZiv`.
    assert!(half.log1p_strict() > zero, "log1p(0.5) must be positive");
    assert!(half.exp_strict() > zero, "exp(0.5) must be positive");
    assert!(half.ln_strict() < zero, "ln(0.5) must be negative");
    assert!(half.sin_strict() > zero, "sin(0.5) must be positive");
    assert!(half.sqrt_strict() > zero, "sqrt(0.5) must be positive");
}
