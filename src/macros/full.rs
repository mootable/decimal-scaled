// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Umbrella `decl_decimal_full!` macro: emits the *common* per-width
//! surface in one invocation, replacing ~15 individual `decl_*!` calls
//! per type in `types/widths.rs`.
//!
//! Not all per-width macro invocations are inside the umbrella:
//!
//! - **Cross-width conversions** (`decl_cross_width_widening!` /
//!   `decl_cross_width_narrowing!`) take pairs of `(source, target)`
//!   types — they're emitted at the call site once per pair, outside
//!   the umbrella.
//! - **`decl_try_from_primitive!`** is invoked once per primitive
//!   (i8..i64, u8..u64). Could be inlined into the umbrella but the
//!   eight-line repetition is already terse.
//!
//! The umbrella is feature-gated implicitly via its callers; D76 /
//! D153 / D307 invocations live behind `#[cfg(any(feature = "d76",
//! feature = "wide"))]` etc. in `types/widths.rs`.

/// Wide-tier umbrella. Emits every macro that takes only
/// `($Type, $Storage[, ...])` args plus the wide-specific
/// `decl_wide_roots!` and `decl_wide_transcendental!` invocations.
///
/// Args:
/// - `$Type`       — decimal type name (`D76`, `D153`, `D307`).
/// - `$Storage`    — signed storage int (`Int<4>`, `Int<8>`, `Int<16>`).
/// - `$Unsigned`   — matching unsigned int for `Display` (`Uint<4>`, …).
/// - `$Wider`      — next-up signed int for mul/div (`Int<8>`, `Int<16>`, `Int<32>`).
/// - `$SqrtWide`   — wide int for `sqrt`'s `r·10^SCALE` (usually `$Storage`).
/// - `$CbrtWide`   — wide int for `cbrt`'s `r·10^(2·SCALE)` (next-up usually).
/// - `$Work`       — wide int for the transcendental core (`Int<16>` for D76, …).
/// - `$core`       — unique module name for the transcendental core.
/// - `$max_scale`  — the type's maximum supported `SCALE`.
#[cfg(any(
    feature = "d57",
    feature = "d76",
    feature = "d115",
    feature = "d153",
    feature = "d230",
    feature = "d307",
    feature = "d462",
    feature = "d616",
    feature = "d924",
    feature = "d1232",
    feature = "wide",
    feature = "x-wide",
    feature = "xx-wide",
))]
macro_rules! decl_decimal_full {
    // `no_const_table` arm — used by D924 / D1232 where the per-tier
    // const POW10_TABLE build exceeds the stable-rust const-eval step
    // budget. Delegates to the main arm and overrides only the
    // `decl_wide_transcendental!` table-mode token (handled below by
    // re-emitting the macro body inline rather than recursing, so the
    // override stays a single source of truth).
    (
        wide $Type:ident,
        $Storage:ty,
        $Unsigned:ty,
        $Wider:ty,
        $SqrtWide:ty,
        $CbrtWide:ty,
        $Work:ty,
        $Wexp:ty,
        $AgmWork:ty,
        $core:ident,
        $max_scale:literal,
        $n_limbs:literal,
        $ln_tang_cap:literal,
        $exp_tang_m:literal,
        no_const_table
    ) => {
        $crate::macros::full::decl_decimal_full!(
            @body $Type, $Storage, $Unsigned, $Wider, $SqrtWide, $CbrtWide,
            $Work, $Wexp, $AgmWork, $core, $max_scale, $n_limbs, $ln_tang_cap, $exp_tang_m, no_const_table
        );
    };
    (
        wide $Type:ident,
        $Storage:ty,
        $Unsigned:ty,
        $Wider:ty,
        $SqrtWide:ty,
        $CbrtWide:ty,
        $Work:ty,
        $Wexp:ty,
        $AgmWork:ty,
        $core:ident,
        $max_scale:literal,
        $n_limbs:literal,
        $ln_tang_cap:literal,
        $exp_tang_m:literal
    ) => {
        $crate::macros::full::decl_decimal_full!(
            @body $Type, $Storage, $Unsigned, $Wider, $SqrtWide, $CbrtWide,
            $Work, $Wexp, $AgmWork, $core, $max_scale, $n_limbs, $ln_tang_cap, $exp_tang_m, with_const_table
        );
    };
    (
        @body $Type:ident,
        $Storage:ty,
        $Unsigned:ty,
        $Wider:ty,
        $SqrtWide:ty,
        $CbrtWide:ty,
        $Work:ty,
        $Wexp:ty,
        $AgmWork:ty,
        $core:ident,
        $max_scale:literal,
        $n_limbs:literal,
        $ln_tang_cap:literal,
        $exp_tang_m:literal,
        $table_mode:ident
    ) => {
        $crate::macros::basics::decl_decimal_basics!(wide $Type, $Storage, $max_scale);
        $crate::macros::arithmetic::decl_decimal_arithmetic!(wide $Type, $Storage, $Wider);
        $crate::macros::display::decl_decimal_display!(wide $Type, $Unsigned);
        $crate::macros::overflow::decl_decimal_overflow_variants!(wide $Type, $Storage, $Wider);
        $crate::macros::num_traits::decl_decimal_num_traits_basics!($Type);
        $crate::macros::sign::decl_decimal_sign_methods!(wide $Type, $Storage);
        $crate::macros::from_str::decl_decimal_from_str!(wide $Type, $Storage);
        $crate::macros::storage_formatters::decl_decimal_storage_formatters!($Type);
        $crate::macros::helpers::decl_decimal_helpers!(wide $Type);
        $crate::macros::bitwise::decl_decimal_bitwise!(wide $Type, $Storage);
        $crate::macros::int_methods::decl_decimal_int_methods!(wide $Type, $Storage);
        $crate::macros::wide_roots::decl_wide_roots!($Type, $Storage, $SqrtWide, $CbrtWide);
        $crate::macros::wide_transcendental::decl_wide_transcendental!(
            $Type, $Storage, $Work, $Wexp, $AgmWork, $core, $max_scale, $table_mode,
            $n_limbs, $ln_tang_cap, $exp_tang_m
        );
        $crate::macros::transcendental_trait::decl_decimal_transcendental_impl!($Type);
        $crate::macros::fast_transcendentals::decl_fast_transcendentals_via_f64!($Type);
        $crate::macros::pow::decl_decimal_pow!($Type);
        $crate::macros::num_traits::decl_decimal_num_traits_conversions!(wide $Type, $Storage);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, i8);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, i16);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, i32);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, i64);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, u8);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, u16);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, u32);
        $crate::macros::conversions::decl_try_from_primitive!(wide $Type, $Storage, u64);
        $crate::macros::conversions::decl_try_from_i128!(wide $Type, $Storage);
        $crate::macros::conversions::decl_try_from_u128!(wide $Type, $Storage);
        $crate::macros::conversions::decl_try_from_f64!(wide $Type, $Storage);
        $crate::macros::conversions::decl_try_from_f32!(wide $Type, $Storage);
        $crate::macros::conversions::decl_decimal_int_conversion_methods!(wide $Type, $Storage);
        $crate::macros::float_bridge::decl_decimal_float_bridge!(wide $Type, $Storage);
        $crate::macros::rescale::decl_decimal_rescale!(wide $Type, $Storage);
        $crate::macros::rounding_methods::decl_decimal_rounding_methods!(wide $Type);
    };
}

#[cfg(any(
    feature = "d57",
    feature = "d76",
    feature = "d115",
    feature = "d153",
    feature = "d230",
    feature = "d307",
    feature = "d462",
    feature = "d616",
    feature = "d924",
    feature = "d1232",
    feature = "wide",
    feature = "x-wide",
    feature = "xx-wide",
))]
pub(crate) use decl_decimal_full;
