//! Cross-WIDTH operators at equal `SCALE`.
//!
//! `a + b` where `a` and `b` differ in storage width, `SCALE`, or both.
//!
//! **Result type = the WIDER storage width, at the LEFT operand's `SCALE`.**
//! The width promotion is lossless. The scale is taken from the left operand,
//! so when the right-hand side carries a finer scale its value is rescaled
//! using the crate's default rounding mode -- exactly what `add_of` does, and
//! the reason a `_with(mode)` sibling exists for explicit control.
//!
//! Two consequences worth knowing:
//! - `+` can ROUND here, where same-scale `+` never did;
//! - the result TYPE is not commutative in scale: `a + b` has the left's
//!   `SCALE`, `b + a` has the right's. The values agree to the shared scale.
//!
//! Both are lifted by the nightly `cross-scale-ops` feature, which sets the
//! result SCALE to `max(S1, S2)` — lossless in BOTH axes and commutative in
//! type. That needs a computed const in `Output` position, which stable Rust
//! rejects outright ("generic parameters may not be used in const operations",
//! verified 2026-08-30) — so it is a genuine added capability, not a
//! convenience, and nightly stays optional rather than required.
//!
//! # Why per-pair and not one blanket impl
//!
//! `Add` carries an associated `Output`. Making it the wider of two generic
//! widths would need `max(N, M)` in type position — a computed const generic,
//! i.e. the `generic_const_exprs` wall. Emitting one impl per concrete
//! `(Self, Rhs)` pair sidesteps it entirely: both widths are literals at the
//! invocation site, so `Output` is just a concrete type.
//!
//! It also keeps coherence free. The same-width operators are already emitted
//! per concrete type (`macros::arithmetic`), so every pair here is a distinct
//! `(Self, Rhs)` and the `N == M` diagonal is never touched.
//!
//! Comparisons need none of this — `PartialEq`/`PartialOrd` return `bool`, so
//! they have no `Output` and are already blanket over `(N, M, S1, S2)`,
//! cross-width *and* cross-scale.
//!
//! # Deliberate limitations (compile errors, not silent loss)
//!
//! - **Cross-SCALE is not offered *through operators*.** The crate fully
//!   supports cross-scale arithmetic — via `D<W>::add_of(a, b)` on stable and
//!   the nightly `cross::*` free functions. It is only the operator form that
//!   cannot: `Output` would need `max(S1, S2)`, and `SCALE` ranges `0..=1232`,
//!   so per-`(S1, S2)` impls are unbounded. The diagnostic says so explicitly
//!   rather than implying the capability is missing.
//! - **Narrowing compound assignment is not offered.** `wide += narrow` is
//!   fine (the rhs widens into `Self`); `narrow += wide` cannot be, because
//!   `AddAssign` writes back into `Self` and would have to discard digits.
//!
//! Cross-SCALE misuse reports the `SameScale` marker's message, which names
//! both SCALEs and points at the explicit `_of` form.
//!
//! Narrowing compound assignment reports `E0308: mismatched types` (expected
//! the narrower type). That one cannot carry a custom message: the same-type
//! `AddAssign<Self>` impl emitted by `macros::arithmetic` shadows any gated
//! impl we could add, so inference commits to it before a marker bound is ever
//! evaluated. Verified 2026-08-30 -- do not re-attempt the gated-impl trick.
//!
//! Neither is ever a silent truncation.


/// Emit every cross-width operator for ONE unordered width pair.
///
/// `$Narrow` must be the strictly narrower storage. Emits, for equal `SCALE`:
/// - `Add`/`Sub`/`Mul`/`Div`/`Rem` in **both** directions, `Output` = `$Wide`;
/// - `AddAssign`/…/`RemAssign` on `$Wide` taking `$Narrow` (widening only).
///
/// Every body delegates to the existing `<$Wide>::<op>_of`, so this macro adds
/// routing only — no arithmetic lives here.
macro_rules! decl_cross_width_pair {
    ($Narrow:ident, $Wide:ident) => {
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Add, add, add_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Sub, sub, sub_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Mul, mul, mul_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Div, div, div_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Rem, rem, rem_of);

        $crate::macros::cross_width_ops::decl_cross_width_assign_op!($Narrow, $Wide, AddAssign, add_assign, add_of);
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!($Narrow, $Wide, SubAssign, sub_assign, sub_of);
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!($Narrow, $Wide, MulAssign, mul_assign, mul_of);
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!($Narrow, $Wide, DivAssign, div_assign, div_of);
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!($Narrow, $Wide, RemAssign, rem_assign, rem_of);
    };
}

/// One value operator, both directions. `Output` is always the wider type.
macro_rules! decl_cross_width_value_op {
    ($Narrow:ident, $Wide:ident, $Trait:ident, $method:ident, $of:ident) => {
        impl<const S1: u32, const S2: u32> ::core::ops::$Trait<$crate::$Wide<S2>> for $crate::$Narrow<S1> {
            type Output = $crate::$Wide<S1>;
            #[inline]
            fn $method(self, rhs: $crate::$Wide<S2>) -> Self::Output {
                $crate::$Wide::<S1>::$of(self, rhs)
            }
        }

        impl<const S1: u32, const S2: u32> ::core::ops::$Trait<$crate::$Narrow<S2>> for $crate::$Wide<S1> {
            type Output = $crate::$Wide<S1>;
            #[inline]
            fn $method(self, rhs: $crate::$Narrow<S2>) -> Self::Output {
                $crate::$Wide::<S1>::$of(self, rhs)
            }
        }
    };
}

/// One compound-assignment operator, widening direction ONLY.
///
/// `Output` is `Self`, so the left operand's width AND scale are both kept and
/// the right-hand side rescales into them. The narrowing direction is simply
/// not emitted: it would have to discard digits to fit.
macro_rules! decl_cross_width_assign_op {
    ($Narrow:ident, $Wide:ident, $Trait:ident, $method:ident, $of:ident) => {
        impl<const S1: u32, const S2: u32> ::core::ops::$Trait<$crate::$Narrow<S2>> for $crate::$Wide<S1> {
            #[inline]
            fn $method(&mut self, rhs: $crate::$Narrow<S2>) {
                *self = $crate::$Wide::<S1>::$of(*self, rhs);
            }
        }
    };
}

pub(crate) use {decl_cross_width_assign_op, decl_cross_width_pair, decl_cross_width_value_op};
