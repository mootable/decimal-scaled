//! Tier-invariant strict-transcendental overflow contract.
//!
//! `docs/ARCHITECTURE.md` → "Overflow & domain behaviour — one contract,
//! invariant across tier and scale": a strict transcendental whose result
//! is out of the storage range PANICS — identically at every width and
//! scale, in BOTH debug and release. There is no ∞/NaN in a fixed-width
//! decimal, so a wrapped/saturated value would be a silent wrong number;
//! the default fails loudly instead.
//!
//! The historic bug this guards: a sufficiently large argument made the
//! internal `exp` working-width arithmetic WRAP (`wrapping_sqr_low_u128`
//! truncates to the low bits → an overflowed square collapses to 0), and
//! the post-narrowing fit check — seeing only the small wrapped value —
//! never fired, so `D18<0>::from(349).exp_strict()` returned `0` instead
//! of panicking, while neighbouring arguments and the wide tiers panicked.
//! `exp_generic::exp_fixed` now rejects such an argument up front, so the
//! contract is uniform. Each `#[should_panic]` below fires in debug AND
//! release because the panic route (`overflow_panic_with_scale` / the
//! kernel's own `panic!`) is NOT gated behind `cfg!(debug_assertions)` —
//! that gating is the separate i64-style ARITHMETIC contract, out of scope
//! here.

// The strict surface is the default build; skip under a non-default
// rounding mode or the f64-bridge `fast` path (same gate as the sibling
// `narrow_strict_transcendentals` suite).
#![cfg(all(
    not(feature = "fast"),
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]

use decimal_scaled::{D18, D38};

// ── Narrow tier (D18, i64 storage) ─────────────────────────────────────
//
// e^349 ≈ 10^151, far beyond i64. Pre-fix this RETURNED 0 (the internal
// squaring wrapped the work integer to a value that survived the storage
// fit check). Must panic, uniform with every other tier.

#[test]
#[should_panic(expected = "result out of range")]
fn narrow_d18_exp_far_overflow_panics() {
    let _ = D18::<0>::from(349).exp_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn narrow_d18_exp2_far_overflow_panics() {
    // 2^400 ≈ 2.6e120, far beyond i64. Pre-fix returned 0.
    let _ = D18::<0>::from(400).exp2_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn narrow_d18_cosh_far_overflow_panics() {
    let _ = D18::<0>::from(500).cosh_strict();
}

// ── Mid tier (D38, i128 storage) ───────────────────────────────────────
//
// e^349 ≈ 10^151, far beyond i128. Pre-fix this RETURNED 0.

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp_far_overflow_panics() {
    let _ = D38::<0>::from(349).exp_strict();
}

// ── Wide tier (feature-gated) ──────────────────────────────────────────
//
// The wide tiers were the half that ALWAYS panicked (the contract the
// narrow tier failed to match); the same far-out-of-range argument that
// wrapped a wide work integer (e.g. D57 exp(1000) → 0 pre-fix) now panics
// too, proving the fix closes the wrap on every tier — narrow AND wide.

#[cfg(any(feature = "d57", feature = "wide"))]
mod wide {
    use decimal_scaled::D57;

    #[test]
    #[should_panic(expected = "result out of range")]
    fn wide_d57_exp_far_overflow_panics() {
        // e^1000 ≈ 10^434, far beyond D57 storage AND its Wexp work integer.
        // Pre-fix this RETURNED 0 (the Wexp squaring wrapped).
        let _ = D57::<0>::from(1000).exp_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn wide_d57_cosh_overflow_panics() {
        let _ = D57::<0>::from(140).cosh_strict();
    }
}

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
mod widest {
    use decimal_scaled::D1232;

    #[test]
    #[should_panic(expected = "result out of range")]
    fn widest_d1232_cosh_far_overflow_panics() {
        // cosh(5000) ≈ e^5000/2 ≈ 10^2171, far beyond D1232 storage. Pre-fix
        // this RETURNED a wrapped multi-limb value.
        let _ = D1232::<0>::from(5000).cosh_strict();
    }
}

// ── In-range cells must still return a value (no false-positive panic) ──
//
// The guard must reject ONLY genuinely out-of-range results. These cells
// sit just inside the storage edge and must compute normally.

#[test]
fn narrow_d18_exp_in_range_returns_value() {
    // e^43 ≈ 4.7e18 < i64::MAX ≈ 9.2e18.
    let _ = D18::<0>::from(43).exp_strict();
}

#[test]
fn mid_d38_exp_in_range_returns_value() {
    // e^88 ≈ 1.65e38 < i128::MAX ≈ 1.70e38 — the last in-range integer arg.
    let _ = D38::<0>::from(88).exp_strict();
}
