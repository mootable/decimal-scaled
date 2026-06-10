// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Macro-generated `Display` and `Debug` for the decimal widths.
//!
//! The algorithm is the same as `D38`'s hand-coded `Display` (in
//! `src/display.rs`): split the unsigned magnitude into integer and
//! fractional parts, format the fractional part zero-padded to `SCALE`
//! digits.
//!
//! Two arms exist:
//!
//! - `decl_decimal_display!(D18)` — *native* storage. The magnitude
//! fits in `u128`, so the split arithmetic is done there.
//! - `decl_decimal_display!(wide D76, Uint<4>)` — *wide* storage. The
//! magnitude is an unsigned wide integer (`unsigned_abs()` handles
//! the `MIN` corner case without overflow); the split arithmetic is
//! done at that width and the `10^SCALE` factor is built via
//! `from_str_radix`.

/// Emits `Display` and `Debug` impls for a decimal type.
macro_rules! decl_decimal_display {
    // Wide storage. `$Unsigned` is the storage type's
    // unsigned counterpart (e.g. `Uint<4>` for `Int<4>`).
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

}

pub(crate) use decl_decimal_display;
