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

// ── D76 storage-narrow gap (peak-margin regression guard) ──────────────
//
// D76 is the SOLE tier whose exp work integer `Wexp == W == Int<16>`
// (1024 bits): a result overflowing the work integer cannot lift any
// wider, so `exp_generic`'s peak gate IS the last line for it. The peak
// model's flat margin was over-large (+512 bits ≈ half D76's work budget)
// and false-panicked IN-RANGE band-edge cosh/sinh at D76<0> (those
// in-range cells are covered by `ulp_strict_golden` band_edges); the
// margin is now a small +64 slack. This pins the COUNTERPART case the
// smaller margin newly routes through the storage-narrowing path: a cosh
// whose result overflows STORAGE but whose internal squaring peak still
// fits Int<16> — cosh(180) ≈ 7e77 is 78 digits (> D76's 76-digit storage)
// while its internal peak is ≈ 764 bits (< 1024). It must still PANIC,
// caught by the narrowing fit check (`round_to_storage_with_g`), not the
// work-int peak gate — keeping the out-of-range contract uniform.
#[cfg(any(feature = "d76", feature = "wide"))]
mod d76_gap {
    use decimal_scaled::D76;

    #[test]
    #[should_panic(expected = "result out of range")]
    fn d76_cosh_storage_overflow_panics() {
        let _ = D76::<0>::from(180).cosh_strict();
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

// ── exp2 deep-overflow band (fractional arguments) ─────────────────────
//
// Pre-fix, a FRACTIONAL deep-overflow exp2 argument (the integer ones are
// caught by the exact-power pin) crashed in INTERNAL kernel machinery
// instead of the contractual panic: the narrow kernel lifted its working
// scale by the result's integer-digit count, so the kernel's own
// `k = round(v/ln 2)` divide outgrew the build's divide scratch
// (`div_knuth` index-out-of-bounds / `top < u.len()` assertion). The
// analytic gate now proves the overflow from the result's integer-digit
// count BEFORE any working-scale arithmetic: `2^x ≥ 10^(d-1)` carries
// `d + scale` storage digits, and `i128` holds fewer than 40, so
// `d + scale >= 40` panics up front — at every scale and width the
// shared narrow kernel serves.

#[test]
#[should_panic(expected = "result out of range")]
fn narrow_d18_exp2_deep_band_low_scale_panics() {
    // Pre-fix: index-out-of-bounds inside div_knuth.
    let v: D18<5> = "150.5".parse().unwrap();
    let _ = v.exp2_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn narrow_d18_exp2_deep_band_far_panics() {
    // Pre-fix: `top < u.len()` assertion inside div_knuth.
    let v: D18<5> = "400.5".parse().unwrap();
    let _ = v.exp2_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp2_deep_band_low_scale_panics() {
    // Scale 1 is the lowest scale with a fractional (non-pin) argument.
    let v: D38<1> = "200.5".parse().unwrap();
    let _ = v.exp2_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp2_deep_band_mid_scale_panics() {
    // Pre-fix: index-out-of-bounds inside div_knuth.
    let v: D38<10> = "150.5".parse().unwrap();
    let _ = v.exp2_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp2_deep_band_far_mid_scale_panics() {
    // Pre-fix: `top < u.len()` assertion inside div_knuth.
    let v: D38<10> = "400.5".parse().unwrap();
    let _ = v.exp2_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp2_deep_band_high_scale_panics() {
    // 2^20.5 ≈ 1.5e6: 7 integer digits + scale 35 ≥ 40 storage digits.
    let v: D38<35> = "20.5".parse().unwrap();
    let _ = v.exp2_strict();
}

#[test]
fn mid_d38_exp2_near_edge_still_computes() {
    // 2^93.5 ≈ 1.4e28 at scale 10 is the last fractional in-range cell of
    // the probe grid; the gate must not touch it.
    let v: D38<10> = "93.5".parse().unwrap();
    assert_eq!(
        format!("{}", v.exp2_strict()),
        "14005692743696534979682984556.8645429406"
    );
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp2_first_out_of_range_cell_panics() {
    // 2^94.5 ≈ 2.8e28 · 10^10 just exceeds i128 — the clean-path edge.
    let v: D38<10> = "94.5".parse().unwrap();
    let _ = v.exp2_strict();
}

// ── exp2 deep-underflow band (negative fractional arguments) ───────────
//
// The mirror band: a deep NEGATIVE exp2 argument inflated the working
// scale by the (meaningless for a sub-one result) integer-digit count of
// `2^|x|` and crashed the same way — on an IN-RANGE cell whose result is
// 0 (or 1 ULP under Ceiling). These must COMPUTE, never panic.

#[test]
fn mid_d38_exp2_deep_underflow_computes_zero() {
    // Pre-fix: `top < u.len()` assertion inside div_knuth.
    let v: D38<10> = "-150.5".parse().unwrap();
    assert_eq!(format!("{}", v.exp2_strict()), "0.0000000000");
}

#[test]
fn mid_d38_exp2_deep_underflow_high_scale_computes() {
    // 2^-80.5 ≈ 5.9e-25 ≈ 0.59 ULP at scale 24 — rounds to 1 ULP under
    // nearest. Pre-fix: index-out-of-bounds inside div_knuth.
    let v: D38<24> = "-80.5".parse().unwrap();
    assert_eq!(
        format!("{}", v.exp2_strict()),
        "0.000000000000000000000001"
    );
}

#[test]
fn mid_d38_exp2_deep_underflow_ceiling_is_one_ulp() {
    use decimal_scaled::RoundingMode;
    // A positive sub-resolution result must round UP under Ceiling.
    let v: D38<10> = "-150.5".parse().unwrap();
    assert_eq!(
        format!("{}", v.exp2_strict_with(RoundingMode::Ceiling)),
        "0.0000000001"
    );
}

// ── exp/sinh extreme band (argument magnitude beyond the work model) ───
//
// Pre-fix the extreme band was WORSE than a crash: `k = x/ln 2` exceeded
// `i128`, truncated, and flipped sign — `exp(1.2e38)` and `sinh(1.2e38)`
// SILENTLY returned 0 / a small wrapped value, and the peak model's
// `|k|·30103` product overflowed `u128` (debug panic, release wrap). The
// argument-magnitude pre-gate classifies these from the bit length alone.

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp_extreme_band_panics() {
    // Pre-fix: debug `attempt to multiply with overflow` in the peak model.
    let v: D38<0> = "99999999999999999999999999999999999".parse().unwrap();
    let _ = v.exp_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_exp_max_arg_panics() {
    // Pre-fix: k wrapped negative through i128 and exp(MAX) returned 0.
    let _ = D38::<0>::MAX.exp_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_sinh_max_arg_panics() {
    // Pre-fix: returned a small wrapped value silently.
    let _ = D38::<0>::MAX.sinh_strict();
}

#[test]
fn mid_d38_exp_extreme_negative_computes_zero() {
    // In-range deep underflow. Pre-fix: internal `div_u512_by_pow10`
    // invariant panic in the 256-bit kernel's range reduction.
    let v: D38<0> = "-150000000000000000000000000000000000000".parse().unwrap();
    assert_eq!(format!("{}", v.exp_strict()), "0");
}

// ── powf deep-overflow band ─────────────────────────────────────────────
//
// powf composes `exp(y·ln x)` on the 256-bit kernel with NO result-regime
// routing, so a deep-overflow exponent reached the kernel's 2^k
// reassembly assertion — loud, but not the contractual message — and an
// extreme one could wrap the `k` shift narrowing. The analytic gate on
// the composition argument (`arg ≥ (39−scale)·ln 10` proves the storage
// overflow) fires first at every scale.

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_powf_deep_band_panics() {
    // 1.5^300.5 ≈ 10^52.9, far past i128 at scale 10.
    let b: D38<10> = "1.5".parse().unwrap();
    let e: D38<10> = "300.5".parse().unwrap();
    let _ = b.powf_strict(e);
}

#[test]
#[should_panic(expected = "result out of range")]
fn mid_d38_powf_extreme_band_panics() {
    // y·ln x / ln 2 ≫ 2^32: the band where the pre-fix `k as u32` shift
    // narrowing could wrap.
    let b: D38<10> = "2.5".parse().unwrap();
    let e: D38<10> = "10000000000.5".parse().unwrap();
    let _ = b.powf_strict(e);
}

#[test]
fn mid_d38_powf_near_edge_still_computes() {
    // 1.5^150.5 ≈ 10^26.5 fits scale 10 with room; the gate must not fire.
    let b: D38<10> = "1.5".parse().unwrap();
    let e: D38<10> = "150.5".parse().unwrap();
    let r = b.powf_strict(e);
    assert!(r > D38::<10>::from(0));
}

#[test]
fn mid_d38_powf_deep_underflow_computes_zero() {
    let b: D38<10> = "0.5".parse().unwrap();
    let e: D38<10> = "10000000000.5".parse().unwrap();
    assert_eq!(format!("{}", b.powf_strict(e)), "0.0000000000");
}

// ── Wide-tier uniformity of the same bands ──────────────────────────────

#[cfg(any(feature = "d76", feature = "wide"))]
mod wide_deep_bands {
    use decimal_scaled::D76;

    #[test]
    #[should_panic(expected = "result out of range")]
    fn wide_d76_exp2_deep_band_panics() {
        // 2^300.5 ≈ 10^90.5 exceeds D76<10>'s 66 integer digits.
        let v: D76<10> = "300.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn wide_d76_exp2_extreme_band_panics() {
        let v: D76<10> = "100000.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    fn wide_d76_exp_deep_negative_computes_zero() {
        // Regression guard for the peak-model pre-gate: a deep-underflow
        // argument must keep reporting "does not fit" so the cell takes the
        // wider lift (the ungated per-tier body's `k·ln2` formation cannot
        // carry it), landing on the canonical 0.
        let v: D76<10> = "-1000.5".parse().unwrap();
        assert_eq!(format!("{}", v.exp_strict()), "0.0000000000");
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn wide_d76_powf_deep_band_panics() {
        let b: D76<10> = "1.5".parse().unwrap();
        let e: D76<10> = "1000.5".parse().unwrap();
        let _ = b.powf_strict(e);
    }
}
