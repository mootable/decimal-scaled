//! Per-width emission of the [`crate::types::traits::dyn_decimal::DynDecimal`] impl.
//!
//! Invoked once per supported width with the full enumeration of legal
//! `SCALE` values for that width. The body is a single
//! `impl<const SCALE: u32> DynDecimal for $Type<SCALE>` block plus the
//! private match-on-scale dispatch for binary ops, rescale, equality,
//! and ordering.
//!
//! The macro is intentionally only invoked for the narrow-tier widths
//! (`D9`, `D18`, `D38`) — see the module docs for
//! [`crate::types::traits::dyn_decimal`] for the scope rationale.

/// Emits `impl<const SCALE: u32> DynDecimal for $Type<SCALE>` plus
/// internal dispatch helpers.
///
/// Args:
/// - `$Type`           — concrete decimal type (`D9` / `D18` / `D38`).
/// - `$Storage`        — underlying primitive integer (`i32` / `i64` / `i128`).
/// - `$width_variant`  — matching variant of [`crate::types::traits::dyn_decimal::DecimalWidth`].
/// - `$raw_variant`    — matching variant of [`crate::types::traits::dyn_decimal::RawStorage`].
/// - `$max_scale`      — `MAX_SCALE` constant for the width.
/// - `$($scale)+`      — every legal `SCALE` literal, `0..=$max_scale`.
///
/// All match-on-scale dispatches enumerate `$($scale)+`. The wildcard
/// arm returns `None` (covers scales above `$max_scale`, which the
/// typed surface rejects at compile time anyway).
macro_rules! decl_decimal_dyn_impl {
    (
        $Type:ident,
        $Storage:ty,
        $width_variant:ident,
        $raw_variant:ident,
        $max_scale:literal,
        scales = [$($scale:literal)+]
    ) => {
        impl<const SCALE: u32> $crate::types::traits::dyn_decimal::DynDecimal for $crate::$Type<SCALE> {
            fn width(&self) -> $crate::types::traits::dyn_decimal::DecimalWidth {
                $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant
            }

            fn scale_dyn(&self) -> u32 { SCALE }

            fn max_scale(&self) -> u32 { $max_scale }

            fn raw_storage(&self) -> $crate::types::traits::dyn_decimal::RawStorage {
                $crate::types::traits::dyn_decimal::RawStorage::$raw_variant(self.0)
            }

            fn as_any(&self) -> &dyn ::core::any::Any { self }

            fn clone_box(&self) -> ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal> {
                ::alloc::boxed::Box::new(*self)
            }

            fn is_zero(&self) -> bool {
                *self == <Self as $crate::DecimalArithmetic>::ZERO
            }

            fn is_one(&self) -> bool {
                *self == <Self as $crate::DecimalArithmetic>::ONE
            }

            fn is_positive(&self) -> bool {
                <Self as $crate::DecimalArithmetic>::is_positive(*self)
            }

            fn is_negative(&self) -> bool {
                <Self as $crate::DecimalArithmetic>::is_negative(*self)
            }

            fn signum(&self) -> ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal> {
                ::alloc::boxed::Box::new(<Self as $crate::DecimalArithmetic>::signum(*self))
            }

            fn abs(&self) -> ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal> {
                ::alloc::boxed::Box::new(<Self as $crate::DecimalArithmetic>::abs(*self))
            }

            fn neg(&self) -> ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal> {
                ::alloc::boxed::Box::new(-*self)
            }

            fn add(
                &self,
                rhs: &dyn $crate::types::traits::dyn_decimal::DynDecimal,
            ) -> Option<::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>> {
                if rhs.width() != $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant {
                    return None;
                }
                let target_scale = SCALE.max(rhs.scale_dyn());
                let lhs_box = $crate::types::traits::dyn_decimal::DynDecimal::rescale_to(self, target_scale)?;
                let rhs_box = rhs.rescale_to(target_scale)?;
                match target_scale {
                    $(
                        $scale => {
                            let l = *lhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            let r = *rhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            <$crate::$Type<$scale> as $crate::DecimalArithmetic>::checked_add(l, r)
                                .map(|res| ::alloc::boxed::Box::new(res)
                                    as ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>)
                        }
                    )+
                    _ => None,
                }
            }

            fn sub(
                &self,
                rhs: &dyn $crate::types::traits::dyn_decimal::DynDecimal,
            ) -> Option<::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>> {
                if rhs.width() != $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant {
                    return None;
                }
                let target_scale = SCALE.max(rhs.scale_dyn());
                let lhs_box = $crate::types::traits::dyn_decimal::DynDecimal::rescale_to(self, target_scale)?;
                let rhs_box = rhs.rescale_to(target_scale)?;
                match target_scale {
                    $(
                        $scale => {
                            let l = *lhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            let r = *rhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            <$crate::$Type<$scale> as $crate::DecimalArithmetic>::checked_sub(l, r)
                                .map(|res| ::alloc::boxed::Box::new(res)
                                    as ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>)
                        }
                    )+
                    _ => None,
                }
            }

            fn mul(
                &self,
                rhs: &dyn $crate::types::traits::dyn_decimal::DynDecimal,
            ) -> Option<::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>> {
                if rhs.width() != $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant {
                    return None;
                }
                let target_scale = SCALE.max(rhs.scale_dyn());
                let lhs_box = $crate::types::traits::dyn_decimal::DynDecimal::rescale_to(self, target_scale)?;
                let rhs_box = rhs.rescale_to(target_scale)?;
                match target_scale {
                    $(
                        $scale => {
                            let l = *lhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            let r = *rhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            <$crate::$Type<$scale> as $crate::DecimalArithmetic>::checked_mul(l, r)
                                .map(|res| ::alloc::boxed::Box::new(res)
                                    as ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>)
                        }
                    )+
                    _ => None,
                }
            }

            fn div(
                &self,
                rhs: &dyn $crate::types::traits::dyn_decimal::DynDecimal,
            ) -> Option<::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>> {
                if rhs.width() != $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant {
                    return None;
                }
                let target_scale = SCALE.max(rhs.scale_dyn());
                let lhs_box = $crate::types::traits::dyn_decimal::DynDecimal::rescale_to(self, target_scale)?;
                let rhs_box = rhs.rescale_to(target_scale)?;
                match target_scale {
                    $(
                        $scale => {
                            let l = *lhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            let r = *rhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            <$crate::$Type<$scale> as $crate::DecimalArithmetic>::checked_div(l, r)
                                .map(|res| ::alloc::boxed::Box::new(res)
                                    as ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>)
                        }
                    )+
                    _ => None,
                }
            }

            fn rem(
                &self,
                rhs: &dyn $crate::types::traits::dyn_decimal::DynDecimal,
            ) -> Option<::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>> {
                if rhs.width() != $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant {
                    return None;
                }
                let target_scale = SCALE.max(rhs.scale_dyn());
                let lhs_box = $crate::types::traits::dyn_decimal::DynDecimal::rescale_to(self, target_scale)?;
                let rhs_box = rhs.rescale_to(target_scale)?;
                match target_scale {
                    $(
                        $scale => {
                            let l = *lhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            let r = *rhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            <$crate::$Type<$scale> as $crate::DecimalArithmetic>::checked_rem(l, r)
                                .map(|res| ::alloc::boxed::Box::new(res)
                                    as ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>)
                        }
                    )+
                    _ => None,
                }
            }

            fn rescale_to(
                &self,
                target_scale: u32,
            ) -> Option<::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>> {
                $crate::types::traits::dyn_decimal::DynDecimal::rescale_to_with(
                    self,
                    target_scale,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            fn rescale_to_with(
                &self,
                target_scale: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Option<::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>> {
                // Scale-up overflow check: the typed `rescale_with` panics on
                // overflow, but the dyn surface contract is to return `None`.
                // For target_scale > SCALE the storage gets multiplied by
                // `10^(target - SCALE)`; pre-check with `checked_mul` so we
                // never reach the typed panic path.
                if target_scale > SCALE && target_scale <= $max_scale {
                    let shift = target_scale - SCALE;
                    let multiplier = (10 as $Storage).pow(shift);
                    self.0.checked_mul(multiplier)?;
                }
                match target_scale {
                    $(
                        $scale => Some(::alloc::boxed::Box::new(self.rescale_with::<$scale>(mode))
                            as ::alloc::boxed::Box<dyn $crate::types::traits::dyn_decimal::DynDecimal>),
                    )+
                    _ => None,
                }
            }

            fn eq_dyn(&self, rhs: &dyn $crate::types::traits::dyn_decimal::DynDecimal) -> bool {
                if rhs.width() != $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant {
                    return false;
                }
                let target_scale = SCALE.max(rhs.scale_dyn());
                let lhs_box = match $crate::types::traits::dyn_decimal::DynDecimal::rescale_to(self, target_scale) {
                    Some(b) => b,
                    None => return false,
                };
                let rhs_box = match rhs.rescale_to(target_scale) {
                    Some(b) => b,
                    None => return false,
                };
                match target_scale {
                    $(
                        $scale => {
                            let l = match lhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>() {
                                Some(v) => *v,
                                None => return false,
                            };
                            let r = match rhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>() {
                                Some(v) => *v,
                                None => return false,
                            };
                            l == r
                        }
                    )+
                    _ => false,
                }
            }

            fn cmp_dyn(
                &self,
                rhs: &dyn $crate::types::traits::dyn_decimal::DynDecimal,
            ) -> Option<::core::cmp::Ordering> {
                if rhs.width() != $crate::types::traits::dyn_decimal::DecimalWidth::$width_variant {
                    return None;
                }
                let target_scale = SCALE.max(rhs.scale_dyn());
                let lhs_box = $crate::types::traits::dyn_decimal::DynDecimal::rescale_to(self, target_scale)?;
                let rhs_box = rhs.rescale_to(target_scale)?;
                match target_scale {
                    $(
                        $scale => {
                            let l = *lhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            let r = *rhs_box.as_any()
                                .downcast_ref::<$crate::$Type<$scale>>()?;
                            Some(l.cmp(&r))
                        }
                    )+
                    _ => None,
                }
            }

            fn display(&self) -> ::alloc::string::String {
                ::alloc::format!("{}", self)
            }

            #[cfg(feature = "std")]
            fn to_f64(&self) -> f64 {
                <Self as $crate::DecimalConvert>::to_f64(*self)
            }

            fn to_int(&self) -> i64 {
                <Self as $crate::DecimalConvert>::to_int(*self)
            }
        }
    };
}

pub(crate) use decl_decimal_dyn_impl;
