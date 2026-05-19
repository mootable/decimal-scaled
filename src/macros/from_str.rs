//! Macro-generated `FromStr` for the decimal widths.
//!
//! The string-parsing front-end ([`crate::support::display::parse_components`])
//! is shared across every width — it does sign / dot / digit-character
//! validation plus the overlong-fractional and leading-zero checks.
//! The accumulator that turns the validated digit slices into a
//! storage value is *per-storage*:
//!
//! - The **native** arm calls [`crate::support::display::parse_decimal_bits`],
//!   which accumulates in `u128` and narrows to the target storage,
//!   reporting [`crate::types::widths::ParseError::OutOfRange`] when the
//!   narrowing would lose information. SCALE is bounded by 38 here
//!   so `10^SCALE` always fits in `u128`.
//! - The **wide** arm emits a per-storage accumulator that does the
//!   base-10 arithmetic at the storage width using the inherent
//!   `checked_mul` / `checked_add` / `checked_sub` / `checked_neg` /
//!   `pow` / `from_i128` items the wide-int macro already emits. This
//!   removes the historical `u128` ceiling — at the deepest tier
//!   (D1231 with SCALE = 1230) the multiplier `10^SCALE` needs ~4087
//!   bits, which fits in the 4096-bit storage.
//!
//! Negative values are accumulated directly into the signed storage
//! (`acc * 10 - digit` rather than `acc * 10 + digit`) so the
//! two's-complement `MIN` boundary is reached without going through a
//! positive intermediate that would overflow `MAX`.

/// Emits `core::str::FromStr` for a decimal type with the given
/// storage.
macro_rules! decl_decimal_from_str {
    // Wide storage. Accumulates the storage value at the storage
    // width — see the module docs for the rationale.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::str::FromStr for $Type<SCALE> {
            type Err = $crate::types::widths::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                let comps = $crate::support::display::parse_components::<SCALE>(s)?;
                let negative = comps.negative;
                let int_str = comps.int_str;
                let frac_str = comps.frac_str;

                // Storage-width constants. `from_i128(10).pow(SCALE)`
                // is wrapping in the inherent impl, but SCALE is
                // bounded by the type's MAX_SCALE and `10^MAX_SCALE`
                // fits the storage by design (e.g. 10^307 fits the
                // 1024-bit storage of D307; 10^1230 fits the 4096-bit
                // storage of D1231).
                let ten = <$Storage>::from_i128(10);
                let multiplier = <$Storage>::pow(ten, SCALE);
                let zero = <$Storage>::from_i128(0);

                // Accumulate the integer part directly into the
                // signed storage. The negative branch subtracts each
                // digit so we approach `MIN` from above without ever
                // forming the positive intermediate `MAX + 1`.
                let mut int_value = zero;
                for &b in int_str {
                    let digit = <$Storage>::from_i128((b - b'0') as i128);
                    let scaled = match <$Storage>::checked_mul(int_value, ten) {
                        ::core::option::Option::Some(v) => v,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                $crate::types::widths::ParseError::OutOfRange,
                            )
                        }
                    };
                    int_value = if negative {
                        match <$Storage>::checked_sub(scaled, digit) {
                            ::core::option::Option::Some(v) => v,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    $crate::types::widths::ParseError::OutOfRange,
                                )
                            }
                        }
                    } else {
                        match <$Storage>::checked_add(scaled, digit) {
                            ::core::option::Option::Some(v) => v,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    $crate::types::widths::ParseError::OutOfRange,
                                )
                            }
                        }
                    };
                }
                let int_scaled = match <$Storage>::checked_mul(int_value, multiplier) {
                    ::core::option::Option::Some(v) => v,
                    ::core::option::Option::None => {
                        return ::core::result::Result::Err(
                            $crate::types::widths::ParseError::OutOfRange,
                        )
                    }
                };

                // Accumulate the fractional part the same way.
                let mut frac_value = zero;
                let frac_len = frac_str.len();
                for &b in frac_str {
                    let digit = <$Storage>::from_i128((b - b'0') as i128);
                    let scaled = match <$Storage>::checked_mul(frac_value, ten) {
                        ::core::option::Option::Some(v) => v,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                $crate::types::widths::ParseError::OutOfRange,
                            )
                        }
                    };
                    frac_value = if negative {
                        match <$Storage>::checked_sub(scaled, digit) {
                            ::core::option::Option::Some(v) => v,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    $crate::types::widths::ParseError::OutOfRange,
                                )
                            }
                        }
                    } else {
                        match <$Storage>::checked_add(scaled, digit) {
                            ::core::option::Option::Some(v) => v,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    $crate::types::widths::ParseError::OutOfRange,
                                )
                            }
                        }
                    };
                }
                let pad = (SCALE as usize) - frac_len;
                if pad > 0 {
                    let pad_factor = <$Storage>::pow(ten, pad as u32);
                    frac_value = match <$Storage>::checked_mul(frac_value, pad_factor) {
                        ::core::option::Option::Some(v) => v,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                $crate::types::widths::ParseError::OutOfRange,
                            )
                        }
                    };
                }

                // `int_scaled` and `frac_value` already share the
                // value's sign, so a plain add combines them.
                let combined = match <$Storage>::checked_add(int_scaled, frac_value) {
                    ::core::option::Option::Some(v) => v,
                    ::core::option::Option::None => {
                        return ::core::result::Result::Err(
                            $crate::types::widths::ParseError::OutOfRange,
                        )
                    }
                };
                ::core::result::Result::Ok(Self(combined))
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::str::FromStr for $Type<SCALE> {
            type Err = $crate::types::widths::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                let bits_i128 = $crate::support::display::parse_decimal_bits::<SCALE>(s)?;
                if bits_i128 > <$Storage>::MAX as i128 || bits_i128 < <$Storage>::MIN as i128 {
                    return ::core::result::Result::Err(
                        $crate::types::widths::ParseError::OutOfRange,
                    );
                }
                ::core::result::Result::Ok(Self(bits_i128 as $Storage))
            }
        }
    };
}

pub(crate) use decl_decimal_from_str;
