//! Macro-generated `Display` and `Debug` for the decimal widths.
//!
//! The algorithm is the same as `D128`'s hand-coded `Display` (in
//! `src/display.rs`): split the unsigned magnitude into integer and
//! fractional parts, format the fractional part zero-padded to `SCALE`
//! digits.
//!
//! Two arms exist:
//!
//! - `decl_decimal_display!(D32)` — *native* storage. The magnitude
//!   fits in `u128`, so the split arithmetic is done there.
//! - `decl_decimal_display!(wide D256, U256)` — *wide* storage. The
//!   magnitude is a `bnum` unsigned integer (`unsigned_abs()` handles
//!   the `MIN` corner case without overflow); the split arithmetic is
//!   done at that width and the `10^SCALE` factor is built via
//!   `from_str_radix`.

/// Emits `Display` and `Debug` impls for a decimal type.
macro_rules! decl_decimal_display {
    // Wide (bnum-backed) storage. `$Unsigned` is the storage type's
    // unsigned counterpart (e.g. `U256` for `I256`).
    (wide $Type:ident, $Unsigned:ty) => {
        impl<const SCALE: u32> ::core::fmt::Display for $Type<SCALE> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let raw = self.to_bits();
                let negative = raw.is_negative();
                let mag: $Unsigned = raw.unsigned_abs();
                let multiplier: $Unsigned = <$Unsigned>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal")
                    .pow(SCALE);
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
                write!(f, concat!(stringify!($Type), "<{}>({})"), SCALE, self)
            }
        }
    };

    // Native (primitive integer) storage.
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
                write!(f, concat!(stringify!($Type), "<{}>({})"), SCALE, self)
            }
        }
    };
}

pub(crate) use decl_decimal_display;
