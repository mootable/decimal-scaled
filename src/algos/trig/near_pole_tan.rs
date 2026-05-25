//! Shared near-pole guard-digit sizing for the `tan` kernels.
//!
//! `tan(x)` is computed as `sin(r)/cos(r)` on the range-reduced residue
//! `r`. When the input lies close to an odd multiple of π/2 the residue
//! folds toward ±π/2, where `cos(r) → 0` and the quotient diverges. A
//! division at a fixed working scale `w` then amplifies the per-op
//! working-scale rounding error by the quotient magnitude
//! `1/cos(r) ≈ |tan|`. Holding the 0.5 ULP storage contract therefore
//! requires the working scale to carry roughly `log10(|tan|)` extra
//! guard digits beyond the magnitude-1 baseline — the same
//! conditioning class as the exp `2^k` reassembly budget.
//!
//! Reference: Muller, *Elementary Functions: Algorithms and
//! Implementation* (3rd ed., 2016), §11.1 — range-reduction error
//! budget when the reduced function has a pole inside the reduction
//! interval.

/// Extra working-scale guard digits a near-pole `tan` evaluation needs,
/// derived from the bit length of a base-width probe quotient.
///
/// The probe `q = sin(r)/cos(r)` is held at working scale `w0`, i.e.
/// `q ≈ |tan| · 10^{w0}`. Its bit length is
/// `bit_length(q) ≈ log2(|tan|) + w0 · log2(10)`, so
/// `log10(|tan|) ≈ bit_length(q) / log2(10) − w0`.
///
/// `bit_length / log2(10)` is computed in integers via the rational
/// `1 / log2(10) ≈ 100_000 / 332_193`. Anything ≤ 0 means the result is
/// magnitude-1 or smaller (no near-pole amplification) and the caller
/// keeps the base width. A small constant margin absorbs the
/// recompute-stage rounding (one extra `sin_cos_fixed` + `div` at the
/// lifted scale) plus the final narrowing half-LSB.
///
/// Scaling with the *measured* magnitude (not a flat constant) is what
/// keeps the lift cheap for near-pole-but-modest inputs while still
/// covering extreme inputs where `|tan|` is many digits large.
#[inline]
#[must_use]
pub(crate) fn tan_extra_digits(probe_bit_length: u32, w0: u32) -> u32 {
    // Integer `log10` of the probe magnitude: bits / log2(10).
    let log10_probe = (probe_bit_length as u64 * 100_000) / 332_193;
    let log10_tan = log10_probe as i64 - w0 as i64;
    if log10_tan <= 0 {
        return 0;
    }
    // +6 margin: the lifted-scale recompute adds one more rounded
    // `sin_cos_fixed` (Taylor + sqrt) and one rounded `div`, and the
    // final `round_to_storage_with` sheds at most a half-LSB. Six
    // decimal digits sits many orders below half a storage ULP.
    //
    // The lift is bounded so the lifted working scale `w0 + extra` stays
    // within `2·w0` digits — the work integer is sized for `2·(SCALE +
    // GUARD)` products, so a lift past `w0` risks overflowing it. An
    // input that close to the pole rounds to a value too large for
    // storage anyway, and `round_to_storage_with` panics with an
    // out-of-range message rather than returning a silently-wrong
    // result.
    let extra = log10_tan as u64 + 6;
    extra.min(w0 as u64) as u32
}

#[cfg(test)]
mod tests {
    use super::tan_extra_digits;

    #[test]
    fn magnitude_one_quotient_needs_no_lift() {
        // |tan| ≈ 1 ⇒ probe ≈ 10^{w0} ⇒ bit_length ≈ w0·log2(10).
        let w0 = 158u32;
        let bits = ((w0 as f64) * std::f64::consts::LOG2_10) as u32;
        assert_eq!(tan_extra_digits(bits, w0), 0);
    }

    #[test]
    fn five_digit_quotient_lifts_about_five_plus_margin() {
        // |tan| ≈ 10^5 ⇒ probe bit length ≈ (w0 + 5)·log2(10).
        let w0 = 158u32;
        let bits = (((w0 + 5) as f64) * std::f64::consts::LOG2_10).ceil() as u32;
        let extra = tan_extra_digits(bits, w0);
        // ~5 magnitude digits + 6 margin, allowing ±1 for integer rounding.
        assert!((10..=12).contains(&extra), "extra = {extra}");
    }

    #[test]
    fn large_quotient_scales_linearly() {
        // |tan| ≈ 10^40 ⇒ ~40 magnitude digits + margin.
        let w0 = 90u32;
        let bits = (((w0 + 40) as f64) * std::f64::consts::LOG2_10).ceil() as u32;
        let extra = tan_extra_digits(bits, w0);
        assert!((45..=47).contains(&extra), "extra = {extra}");
    }
}
