//! Parity / no-panic coverage for the deep-scale Tang-lookup `ln_strict`
//! bands, one parametrised arm per `(width, band)`.
//!
//! Only `ln_strict` is wired through the deep-band lookup at these widths;
//! exp / hyperbolics still route the canonical `wide_kernel`. The composed
//! `exp(ln(x)) ≈ x` identity therefore mixes both paths and is a strong
//! end-to-end correctness probe: lookup error feeds `exp_strict`'s reverse
//! mapping and must come back within a few storage LSBs.
//!
//! Band coverage policy (mirrors the golden grid):
//! * OFF-GRID bands — no golden cell lands inside them — keep the full
//!   round-trip parity plus the band-edge no-panic bounds:
//!   D307 s285..=295 (cell s290), D616 s585..=595 (s590),
//!   D924 s895..=905 (s900), D1232 s1195..=1205 (s1200).
//! * ON-GRID bands — the golden gate pins the mid-band cell bit-exact —
//!   keep only the band-EDGE no-panic bounds, which no golden cell covers:
//!   D924 s455 / s465 (cell D924<462> is on-grid),
//!   D1232 s610 / s620 (cell D1232<616> is on-grid).
//!
//! NOTE: `ln_strict_agm` is documented to drop to `~p/2` bits beyond
//! `w ~ 30`, so at these working widths AGM is the *lower*-accuracy kernel —
//! not a useful cross-witness. The identity round trip is the correct probe.

/// Round-trip parity arm: `exp(ln(x)) ≈ x` within `n_lsb` storage LSBs at
/// the band's mid cell, plus the `ln(1) = 0` Stage-1 short-circuit.
///
/// EPSILON = one storage LSB (the smallest representable positive value at
/// this `(width, SCALE)`). Avoids the `10.pow(SCALE)` overflow that bites
/// deep-band tests where SCALE x 2 exceeds the storage width's
/// representable range.
// Allow: every consumer module is feature-gated, so narrower builds compile
// the macro with no user.
#[allow(unused_macros)]
macro_rules! ln_band_round_trips {
    ($D:ident, $scale:literal, $n_lsb:expr) => {
        type D = decimal_scaled::$D<$scale>;

        fn from_int(n: i128) -> D {
            D::try_from(n).unwrap()
        }

        #[track_caller]
        fn agree_within_n_storage_lsb(label: &str, a: D, b: D, n_lsb: u128) {
            let diff = if a >= b { a - b } else { b - a };
            let lsb = D::EPSILON;
            let limit = D::try_from(n_lsb as i128).unwrap() * lsb;
            assert!(
                diff <= limit,
                "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
            );
        }

        #[test]
        fn exp_ln_round_trip_half() {
            let x = from_int(3) / from_int(2); // 1.5
            let round = x.ln_strict().exp_strict();
            agree_within_n_storage_lsb(
                concat!("exp(ln(1.5)) ", stringify!($D), "<", stringify!($scale), ">"),
                round,
                x,
                $n_lsb,
            );
        }

        #[test]
        fn exp_ln_round_trip_two() {
            let x = from_int(2);
            let round = x.ln_strict().exp_strict();
            agree_within_n_storage_lsb(
                concat!("exp(ln(2)) ", stringify!($D), "<", stringify!($scale), ">"),
                round,
                x,
                $n_lsb,
            );
        }

        #[test]
        fn exp_ln_round_trip_three() {
            let x = from_int(3);
            let round = x.ln_strict().exp_strict();
            agree_within_n_storage_lsb(
                concat!("exp(ln(3)) ", stringify!($D), "<", stringify!($scale), ">"),
                round,
                x,
                $n_lsb,
            );
        }

        #[test]
        fn ln_lookup_at_one_is_zero() {
            // ln(1) = 0 must hold exactly through the Stage-1 short-circuit.
            let one = from_int(1);
            let z = one.ln_strict();
            assert_eq!(
                z,
                D::ZERO,
                concat!("ln(1) ", stringify!($D), "<", stringify!($scale), ">: expected ZERO"),
            );
        }
    };
}

/// Band-EDGE no-panic bound: confirms the edge SCALE enters the lookup band
/// (no panic / no overflow) and lands `ln(1.5)` inside (0, 1).
#[allow(unused_macros)]
macro_rules! ln_band_edge_no_panic {
    ($name:ident, $D:ident, $scale:literal) => {
        #[test]
        fn $name() {
            let x = decimal_scaled::$D::<$scale>::from(3) / decimal_scaled::$D::<$scale>::from(2);
            let y = x.ln_strict();
            assert!(y < decimal_scaled::$D::<$scale>::from(1));
            assert!(y > decimal_scaled::$D::<$scale>::ZERO);
        }
    };
}

// ── D307 s285..=295 (off-grid band; cell s290) ─────────────────────────────
#[cfg(all(feature = "x-wide", not(feature = "fast")))]
mod from_d307_s290_lookup_parity {
    ln_band_round_trips!(D307, 290, 8);
    ln_band_edge_no_panic!(ln_lookup_band_lower_bound_s285, D307, 285);
    ln_band_edge_no_panic!(ln_lookup_band_upper_bound_s295, D307, 295);
}

// ── D616 s585..=595 (off-grid band; cell s590) ─────────────────────────────
#[cfg(all(feature = "x-wide", not(feature = "fast")))]
mod from_d616_s590_lookup_parity {
    ln_band_round_trips!(D616, 590, 8);
    ln_band_edge_no_panic!(ln_lookup_band_lower_bound_s585, D616, 585);
    ln_band_edge_no_panic!(ln_lookup_band_upper_bound_s595, D616, 595);
}

// ── D924 s455..=465 (mid-band on-grid at D924<462>; edges kept) ────────────
#[cfg(all(
    feature = "xx-wide",
    feature = "x-wide",
    feature = "wide",
    not(feature = "fast")
))]
mod from_d924_s460_lookup_parity {
    ln_band_edge_no_panic!(ln_lookup_band_lower_bound_s455, D924, 455);
    ln_band_edge_no_panic!(ln_lookup_band_upper_bound_s465, D924, 465);
}

// ── D924 s895..=905 (off-grid band; cell s900) ─────────────────────────────
#[cfg(all(
    feature = "xx-wide",
    feature = "x-wide",
    feature = "wide",
    not(feature = "fast")
))]
mod from_d924_s900_lookup_parity {
    ln_band_round_trips!(D924, 900, 8);
    ln_band_edge_no_panic!(ln_lookup_band_lower_bound_s895, D924, 895);
    ln_band_edge_no_panic!(ln_lookup_band_upper_bound_s905, D924, 905);
}

// ── D1232 s610..=620 (mid-band on-grid at D1232<616>; edges kept) ──────────
#[cfg(all(feature = "xx-wide", not(feature = "fast")))]
mod from_d1232_s615_lookup_parity {
    ln_band_edge_no_panic!(ln_lookup_band_lower_bound_s610, D1232, 610);
    ln_band_edge_no_panic!(ln_lookup_band_upper_bound_s620, D1232, 620);
}

// ── D1232 s1195..=1205 (off-grid band; cell s1200) ─────────────────────────
#[cfg(all(
    feature = "xx-wide",
    feature = "x-wide",
    feature = "wide",
    not(feature = "fast")
))]
mod from_d1232_s1200_lookup_parity {
    ln_band_round_trips!(D1232, 1200, 8);
    ln_band_edge_no_panic!(ln_lookup_band_lower_bound_s1195, D1232, 1195);
    ln_band_edge_no_panic!(ln_lookup_band_upper_bound_s1205, D1232, 1205);
}
