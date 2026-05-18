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
//! `S` is `i32` (D9), `i64` (D18), `i128` (D38). For the wide tiers
//! `S` is one of the `crate::wide_int::Int{192,256,384,…,4096}`
//! types.
//!
//! Methods on `D<S, SCALE>` are added per-`S` in the macros / impl
//! blocks scattered across the crate — see `core_type.rs`, the
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
//! Existing names (`D9`, `D18`, `D38`, `D57`, …, `D1232`) become
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

impl<S: PartialEq, const SCALE: u32> PartialEq for D<S, SCALE> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: Eq, const SCALE: u32> Eq for D<S, SCALE> {}

impl<S: PartialOrd, const SCALE: u32> PartialOrd for D<S, SCALE> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<S: Ord, const SCALE: u32> Ord for D<S, SCALE> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S: core::hash::Hash, const SCALE: u32> core::hash::Hash for D<S, SCALE> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
