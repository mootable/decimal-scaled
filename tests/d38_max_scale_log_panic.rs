//! Regression test for the `log10_strict` panic on `D38::<38>` at
//! scale-saturated inputs.
//!
//! At SCALE=38 the storage `i128::MAX ≈ 1.7e38` only represents values
//! in roughly `[-1.7, 1.7]`. Calling `from_f64(2.0)` therefore
//! saturates to `Self::MAX`, and historically `log10_strict()` panicked
//! during the ln kernel's range reduction because the working-scale
//! intermediate `raw · 10^STRICT_GUARD = MAX · 10^30 ≈ 1.7e68` plus the
//! Mercator artanh series intermediate overflowed the 256-bit `Fixed`.

#![cfg(all(feature = "strict", not(feature = "fast")))]

use decimal_scaled::D38;

/// `D38::<38>::from_f64(2.0)` saturates to `MAX` then `log10_strict`
/// must produce a finite result without panicking. `log10(MAX) ≈ 0.23`
/// (because `MAX ≈ 1.7`), which fits the storage comfortably.
#[test]
fn log10_strict_on_saturated_d38_scale38() {
    let v = D38::<38>::from_f64(2.0);
    let r = v.log10_strict();
    // log10(1.7014…) ≈ 0.23099…  -> at scale 38, bits ≈ 2.31e37
    let bits = i128::from(r.to_bits());
    assert!(bits > 0, "log10(MAX) must be positive, got {bits}");
    assert!(
        bits < 3 * 10_i128.pow(37),
        "log10(MAX) ≈ 0.23, bits ≈ 2.3e37, got {bits}"
    );
}

/// And a direct "near-boundary" input via `from_bits` to pin the
/// failure to the kernel rather than `from_f64`'s saturation policy.
#[test]
fn log10_strict_on_near_max_d38_scale38() {
    let v = D38::<38>::from_bits(decimal_scaled::Int::<2>::try_from((i128::MAX) as i128).unwrap());
    let _ = v.log10_strict();
}
