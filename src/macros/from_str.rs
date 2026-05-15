//! Macro-generated `FromStr` for the decimal widths.
//!
//! The full parser logic lives in `display.rs::parse_decimal_bits`,
//! which returns the raw bits as `i128`.
//!
//! - The *native* arm narrows that `i128` to the target storage and
//!   reports `ParseError::OutOfRange` when the narrowing would lose
//!   information.
//! - The *wide* arm widens the `i128` into `bnum` storage via
//!   `bnum::cast::As`. Note: because the shared parser accumulates in
//!   `i128`, wide-width parsing is currently limited to values whose
//!   scaled magnitude fits in `i128`; literals beyond that report
//!   `ParseError::OutOfRange` from the parser itself. A full-range
//!   wide parser is future work.

/// Emits `core::str::FromStr` for a decimal type with the given
/// storage. Reuses the central `parse_decimal_bits` helper.
macro_rules! decl_decimal_from_str {
    // Wide (bnum-backed) storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::str::FromStr for $Type<SCALE> {
            type Err = $crate::core_type::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                let bits_i128 = $crate::display::parse_decimal_bits::<SCALE>(s)?;
                ::core::result::Result::Ok(Self(<$Storage>::from_i128(bits_i128)))
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::str::FromStr for $Type<SCALE> {
            type Err = $crate::core_type::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                let bits_i128 = $crate::display::parse_decimal_bits::<SCALE>(s)?;
                if bits_i128 > <$Storage>::MAX as i128 || bits_i128 < <$Storage>::MIN as i128 {
                    return ::core::result::Result::Err(
                        $crate::core_type::ParseError::OutOfRange,
                    );
                }
                ::core::result::Result::Ok(Self(bits_i128 as $Storage))
            }
        }
    };
}

pub(crate) use decl_decimal_from_str;
