// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Cross-WIDTH operators, at any SCALEs.
//!
//! `a + b` where the operands differ in storage width, SCALE, or both.
//!
//! Result type = the WIDER storage width, at the LEFT operand's SCALE. The width
//! promotion is lossless; a finer-scaled right-hand side is rescaled with the
//! crate default rounding mode, exactly as `add_of` does.
//!
//! Left-first is deliberate. `max(S1, S2)` was REJECTED: it makes the result type
//! depend on the VALUES of the const params rather than operand position, which
//! breaks the determinism Rust guarantees elsewhere. It also could not be a Cargo
//! feature (non-additive: unification would let any crate in the graph silently
//! change what `a + b` returns for everyone). Left-first also keeps
//! `a += b` == `a = a + b`, which a right-first rule would break.
//!
//! Per-pair rather than blanket: `Add` carries an associated `Output`, and
//! deriving it from two generic widths needs a computed const in type position,
//! which stable rejects. At a macro invocation both widths are literals, so
//! `Output` is concrete. Coherence is free -- same-width operators are already
//! emitted per concrete type, so every pair here is a distinct (Self, Rhs) and
//! the N == M diagonal is untouched. Measured: 66 pairs cost no compile time.
//!
//! Comparisons need none of this: they return bool, have no `Output`, and are
//! already blanket over (N, M, S1, S2).
//!
//! Compound assignment keeps the LEFT operand's width AND scale. Widening always
//! fits. Narrowing computes at the wider width then converts back down, and
//! panics with Rust's standard overflow message if it does not fit -- a value
//! that overflows on a width change behaves like any other operator overflow,
//! not a compile error.

/// Emit every cross-width operator for ONE unordered width pair.
macro_rules! decl_cross_width_pair {
    ($Narrow:ident, $NarrowLimbs:literal, $Wide:ident, $WideLimbs:literal) => {
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Add, add, add_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Sub, sub, sub_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Mul, mul, mul_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Div, div, div_of);
        $crate::macros::cross_width_ops::decl_cross_width_value_op!($Narrow, $Wide, Rem, rem, rem_of);

        $crate::macros::cross_width_ops::decl_cross_width_assign_op!(
            $Narrow, $NarrowLimbs, $Wide, AddAssign, add_assign, add_of,
            "attempt to add with overflow");
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!(
            $Narrow, $NarrowLimbs, $Wide, SubAssign, sub_assign, sub_of,
            "attempt to subtract with overflow");
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!(
            $Narrow, $NarrowLimbs, $Wide, MulAssign, mul_assign, mul_of,
            "attempt to multiply with overflow");
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!(
            $Narrow, $NarrowLimbs, $Wide, DivAssign, div_assign, div_of,
            "attempt to divide with overflow");
        $crate::macros::cross_width_ops::decl_cross_width_assign_op!(
            $Narrow, $NarrowLimbs, $Wide, RemAssign, rem_assign, rem_of,
            "attempt to calculate the remainder with overflow");
    };
}

/// One value operator, both directions. Output = wider type at the LEFT scale.
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

/// One compound-assignment operator, BOTH directions.
macro_rules! decl_cross_width_assign_op {
    ($Narrow:ident, $NarrowLimbs:literal, $Wide:ident, $Trait:ident, $method:ident, $of:ident, $overflow:literal) => {
        impl<const S1: u32, const S2: u32> ::core::ops::$Trait<$crate::$Narrow<S2>> for $crate::$Wide<S1> {
            #[inline]
            fn $method(&mut self, rhs: $crate::$Narrow<S2>) {
                *self = $crate::$Wide::<S1>::$of(*self, rhs);
            }
        }

        impl<const S1: u32, const S2: u32> ::core::ops::$Trait<$crate::$Wide<S2>> for $crate::$Narrow<S1> {
            #[inline]
            fn $method(&mut self, rhs: $crate::$Wide<S2>) {
                let wide_value = $crate::$Wide::<S1>::$of(*self, rhs);
                let narrowed = wide_value.0.try_narrow::<$NarrowLimbs>().expect($overflow);
                *self = $crate::$Narrow::<S1>::from_bits(narrowed);
            }
        }
    };
}

pub(crate) use {decl_cross_width_assign_op, decl_cross_width_pair, decl_cross_width_value_op};
