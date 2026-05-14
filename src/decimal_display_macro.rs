//! Macro-generated `Display` and `Debug` for narrow decimal widths.
//!
//! The body is the same algorithm as `D128`'s hand-coded `Display`
//! (in `src/display.rs`): split the unsigned magnitude into integer
//! and fractional parts, format the fractional part zero-padded to
//! `SCALE` digits. The macro parameterises only the type name and
//! relies on `to_bits()` to fetch the storage value uniformly.

/// Emits `Display` and `Debug` impls for a decimal type.
macro_rules! decl_decimal_display {
    ($Type:ident) => {
        impl<const SCALE: u32> ::core::fmt::Display for $Type<SCALE> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let raw = self.to_bits();
                let negative = raw < 0;
                let mag: u128 = raw.unsigned_abs() as u128;
                let multiplier: u128 = 10u128.pow(SCALE);
                let int_part = mag / multiplier;
                let frac_part = mag % multiplier;

                if negative {
                    f.write_str("-")?;
                }
                if SCALE == 0 {
                    return write!(f, "{int_part}");
                }
                let width = SCALE as usize;
                write!(f, "{int_part}.{frac_part:0>width$}", width = width)
            }
        }

        impl<const SCALE: u32> ::core::fmt::Debug for $Type<SCALE> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(
                    f,
                    concat!(stringify!($Type), "<{}>({})"),
                    SCALE,
                    self
                )
            }
        }
    };
}

pub(crate) use decl_decimal_display;
