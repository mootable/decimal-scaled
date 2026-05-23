//! Unified decimal type: `D<S, const SCALE: u32>` — a generic
//! `#[repr(transparent)]` wrapper over the storage integer `S`.
//!
//! # Why
//!
//! Each concrete decimal width in the crate (`D38`, `D57`, `D76`, …)
//! was originally its own `#[repr(transparent)]` newtype. That worked
//! but meant every per-width macro invocation, every method shell,
//! every cross-width helper had to be duplicated by name. The
//! [`D<S, SCALE>`](crate::D) type collapses that: the per-width
//! struct definitions become type aliases over a single generic
//! type, and methods can be implemented once per storage (generic
//! over `SCALE`) rather than once per `(width, scale)` pair.
//!
//! # Storage parameterisation
//!
//! `S` is the storage integer. For the narrow primitive tiers
//! `S` is `i64` (D18), `i128` (D38). For the wide tiers
//! `S` is one of the `crate::wide_int::Int{192,256,384,…,4096}`
//! types.
//!
//! Methods on `D<S, SCALE>` are added per-`S` in the macros / impl
//! blocks scattered across the crate — see `types/widths.rs`, the
//! `macros/` directory, and `policy/`. This file only carries the
//! struct definition and the most foundational `impl`s
//! (`Clone` / `Copy` / `Default` derivation patterns that need
//! tighter bounds than the derive macro provides).
//!
//! `Debug` is deliberately NOT a blanket impl on `D<S, SCALE>`: it is
//! emitted per-width by `decl_decimal_display!` so the output is the
//! canonical decimal string rather than the raw integer. A blanket
//! `Debug` would collide with the macro-emitted impls once per-width
//! types alias `D<…, SCALE>`.
//!
//! # `SCALE` parameterisation
//!
//! `SCALE` is the base-10 exponent: the logical value of
//! `D::<S, SCALE>(raw)` is `raw / 10^SCALE`. Same semantics as the
//! original per-width types.
//!
//! # Compatibility
//!
//! Existing names (`D18`, `D38`, `D57`, …, `D1232`) become
//! type aliases of `D<…, SCALE>`. Source-compatible. The
//! `#[repr(transparent)]` layout is preserved per storage, so the
//! raw-bytes representation of `D38<5>` is unchanged.

/// Generic scaled fixed-point decimal: storage integer `S`, base-10
/// scale `SCALE`. The logical value is `self.0 / 10^SCALE`.
///
/// See the module docs for the parameterisation contract.
#[repr(transparent)]
pub struct D<S, const SCALE: u32>(pub S);

// `Clone` / `Copy` need explicit bounds — `#[derive]` would require
// `S: Clone + Copy` to be inferable on the struct, which isn't always
// what we want. Hand-rolling keeps the bounds tight per-call.

impl<S: Clone, const SCALE: u32> Clone for D<S, SCALE> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S: Copy, const SCALE: u32> Copy for D<S, SCALE> {}

// `Debug` is intentionally NOT provided here as a blanket impl. Each
// concrete storage's `Debug` impl is emitted by the per-width display
// macro (`decl_decimal_display!`) so the output is the canonical
// decimal string rather than the raw integer. A blanket impl on
// `D<S, SCALE>` would collide with those macro-emitted impls once
// the per-width types are aliases of `D<…, SCALE>`.

// Equality / ordering. The `D` type is always `Int<N>`-backed, so these
// impls are bound to `Int` storage and delegate to the int-layer
// cross-width comparator (`Int::cmp_cross`) on the storages at the same
// `SCALE`. One generic `PartialEq` / `PartialOrd` pair covers every
// `(width, width)` combination — the same-type case (`N == M`) is one
// instantiation, so no separate same-type impl is needed (a derived or
// hand-written same-type comparison would collide — E0119).
use crate::int::types::Int;

impl<const N: usize, const M: usize, const S: u32> PartialEq<D<Int<M>, S>> for D<Int<N>, S> {
    #[inline]
    fn eq(&self, other: &D<Int<M>, S>) -> bool {
        self.0.cmp_cross(other.0) == core::cmp::Ordering::Equal
    }
}

// `Eq` requires only `PartialEq<Self>`, provided by the generic above
// (the `N == M`, `S == S` instantiation).
impl<const N: usize, const S: u32> Eq for D<Int<N>, S> {}

impl<const N: usize, const M: usize, const S: u32> PartialOrd<D<Int<M>, S>> for D<Int<N>, S> {
    #[inline]
    fn partial_cmp(&self, other: &D<Int<M>, S>) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp_cross(other.0))
    }
}

// Same-type total order via the same comparator path.
impl<const N: usize, const S: u32> Ord for D<Int<N>, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp_cross(other.0)
    }
}

impl<S: core::hash::Hash, const SCALE: u32> core::hash::Hash for D<S, SCALE> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::types::widths::{D18, D38};

    /// Equal logical values compare equal across decimal widths at the
    /// same SCALE.
    #[test]
    fn cross_width_equal_values() {
        let narrow: D18<2> = D18::<2>::from_int(5_i64);
        let wide: D38<2> = D38::<2>::from_int(5_i64);
        assert!(narrow == wide);
        assert!(wide == narrow);
    }

    /// Ordering holds across widths at the same SCALE, both directions.
    #[test]
    fn cross_width_ordering() {
        let narrow: D18<2> = D18::<2>::from_int(5_i64);
        let wide_bigger: D38<2> = D38::<2>::from_int(6_i64);
        assert!(narrow < wide_bigger);
        assert!(wide_bigger > narrow);
        assert_ne!(narrow, wide_bigger);
    }

    /// A value that fits only in the wider tier still compares correctly
    /// against a narrow value (no overflow, no wraparound).
    #[test]
    fn cross_width_value_only_in_wider_tier() {
        // D38<2> scales by 10^2, so from_int(10^16) stores 10^18 in the
        // i128 backend — beyond the i64-backed D18 storage range, so the
        // value only fits the wider tier. The comparison must not wrap.
        let huge: D38<2> = D38::<2>::from_int(10_000_000_000_000_000_i64);
        let small: D18<2> = D18::<2>::from_int(1_i64);
        assert!(small < huge);
        assert!(huge > small);
    }

    /// Negative values compare correctly across widths.
    #[test]
    fn cross_width_negatives() {
        let narrow_neg: D18<2> = D18::<2>::from_int(-3_i64);
        let wide_neg: D38<2> = D38::<2>::from_int(-3_i64);
        let wide_more_neg: D38<2> = D38::<2>::from_int(-4_i64);
        assert_eq!(narrow_neg, wide_neg);
        assert!(wide_more_neg < narrow_neg);
        assert!(narrow_neg > wide_more_neg);
        // Sign boundary: negative narrow < non-negative wide.
        let wide_pos: D38<2> = D38::<2>::from_int(1_i64);
        assert!(narrow_neg < wide_pos);
    }

    /// Same-type values sort via the generic `Ord` path.
    #[test]
    fn same_type_sort() {
        let mut v = [
            D38::<2>::from_int(3_i64),
            D38::<2>::from_int(-1_i64),
            D38::<2>::from_int(2_i64),
            D38::<2>::from_int(0_i64),
        ];
        v.sort();
        assert_eq!(
            v,
            [
                D38::<2>::from_int(-1_i64),
                D38::<2>::from_int(0_i64),
                D38::<2>::from_int(2_i64),
                D38::<2>::from_int(3_i64),
            ]
        );
    }
}
