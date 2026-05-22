//! Macro-generated `PartialEq` / `PartialOrd` operator overloads
//! between adjacent decimal widths at the *same* `SCALE`.
//!
//! With these in place, ordinary `==` / `<` / etc. work across widths
//! without an explicit `.widen()`:
//!
//! ```ignore
//! let a: D38<12> = D38::<12>::from_int(5);
//! let b: D18<12> = D18::<12>::from_int(5);
//! assert!(a == b);   // stable on cross-width same-SCALE
//! assert!(b < D38::<12>::from_int(6));
//! ```
//!
//! The matrix is bidirectional: each (narrower, wider) pair gets both
//! `impl PartialEq<Wider<S>> for Narrower<S>` and
//! `impl PartialEq<Narrower<S>> for Wider<S>`. Same for `PartialOrd`.
//!
//! Cross-SCALE comparators live on the nightly-gated `cross-scale-ops`
//! feature; see `src/cross_scale.rs`. The cross-SCALE case requires
//! `generic_const_exprs` to compute the common-scale type at the impl
//! site, which is not yet stable.

/// Emits `PartialEq` and `PartialOrd` between two decimal widths at
/// the *same* `SCALE`. Both directions of each trait are emitted.
///
/// The comparison widens the narrower operand to the wider storage
/// via `WidthLE::widen_into` and then defers to the wider storage's
/// `Ord` / `Eq`.
macro_rules! decl_cross_width_eq_ord {
    ($Narrower:ident, $NarrowStorage:ty, $Wider:ident, $BigInt:ty) => {
        // Narrower<S> == Wider<S>
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Wider<SCALE>> for $Narrower<SCALE> {
            #[inline]
            fn eq(&self, other: &$Wider<SCALE>) -> bool {
                let widened: $BigInt =
                    <$NarrowStorage as $crate::WidthLE<$BigInt>>::widen_into(self.0);
                widened == other.0
            }
        }
        // Wider<S> == Narrower<S>
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Narrower<SCALE>> for $Wider<SCALE> {
            #[inline]
            fn eq(&self, other: &$Narrower<SCALE>) -> bool {
                let widened: $BigInt =
                    <$NarrowStorage as $crate::WidthLE<$BigInt>>::widen_into(other.0);
                self.0 == widened
            }
        }
        // Narrower<S> < Wider<S> etc.
        impl<const SCALE: u32> ::core::cmp::PartialOrd<$Wider<SCALE>> for $Narrower<SCALE> {
            #[inline]
            fn partial_cmp(
                &self,
                other: &$Wider<SCALE>,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let widened: $BigInt =
                    <$NarrowStorage as $crate::WidthLE<$BigInt>>::widen_into(self.0);
                ::core::cmp::PartialOrd::partial_cmp(&widened, &other.0)
            }
        }
        // Wider<S> < Narrower<S> etc.
        impl<const SCALE: u32> ::core::cmp::PartialOrd<$Narrower<SCALE>> for $Wider<SCALE> {
            #[inline]
            fn partial_cmp(
                &self,
                other: &$Narrower<SCALE>,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let widened: $BigInt =
                    <$NarrowStorage as $crate::WidthLE<$BigInt>>::widen_into(other.0);
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &widened)
            }
        }
    };
}

pub(crate) use decl_cross_width_eq_ord;
