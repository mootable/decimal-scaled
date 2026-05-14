//! Macro-generated radix formatters for the raw storage of a decimal
//! type.
//!
//! These format the underlying integer (= `value * 10^SCALE`), not the
//! decimal value, so they're useful for inspecting bit patterns. All
//! delegate to the storage type's primitive formatter so format flags
//! (`#`, `0`, width, precision) are forwarded unchanged.

/// Emits `LowerHex`, `UpperHex`, `Octal`, `Binary` for the raw storage
/// of `$Type<SCALE>`.
macro_rules! decl_decimal_storage_formatters {
    ($Type:ident) => {
        impl<const SCALE: u32> ::core::fmt::LowerHex for $Type<SCALE> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.0, f)
            }
        }
        impl<const SCALE: u32> ::core::fmt::UpperHex for $Type<SCALE> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.0, f)
            }
        }
        impl<const SCALE: u32> ::core::fmt::Octal for $Type<SCALE> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Octal::fmt(&self.0, f)
            }
        }
        impl<const SCALE: u32> ::core::fmt::Binary for $Type<SCALE> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Binary::fmt(&self.0, f)
            }
        }
    };
}

pub(crate) use decl_decimal_storage_formatters;
