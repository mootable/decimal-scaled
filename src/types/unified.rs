// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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
//! `S` is one of the `crate::int::types::Int{192,256,384,…,4096}`
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
// impls are bound to `Int` storage and delegate to the policy dispatchers
// in `policy::dcmp` / `policy::deq`. ONE generic `PartialEq` / `PartialOrd`
// pair, parameterised over both widths (`N`, `M`) AND both scales (`S1`,
// `S2`), covers every `(width, scale) × (width, scale)` combination — the
// same-type case (`N == M`, `S1 == S2`) is just one instantiation, so no
// separate same-type impl is needed (a derived or hand-written same-type
// comparison would collide — E0119). This 4-param impl subsumes (and
// replaces) the earlier 3-param same-scale impl for the same coherence
// reason.
//
// The `S1 == S2` branch const-folds in the policy dispatcher, so the
// common same-scale path monomorphises to a plain cross-width compare
// (`Int::cmp_cross`, no multiply); only `S1 != S2` reaches the cross-scale
// comparator (`Int::cmp_cross_scaled`), oriented so the higher-scale (more
// decimal digits) operand is the one scaled down by `10^|S1−S2|`.
use crate::int::types::Int;

impl<const N: usize, const M: usize, const S1: u32, const S2: u32> PartialEq<D<Int<M>, S2>>
    for D<Int<N>, S1>
{
    #[inline]
    fn eq(&self, other: &D<Int<M>, S2>) -> bool {
        crate::policy::deq::deq_dispatch::<N, M, S1, S2>(self.0, other.0)
    }
}

// `Eq` requires only `PartialEq<Self>`, provided by the generic above
// (the `N == M`, `S1 == S2` instantiation).
impl<const N: usize, const S: u32> Eq for D<Int<N>, S> {}

impl<const N: usize, const M: usize, const S1: u32, const S2: u32> PartialOrd<D<Int<M>, S2>>
    for D<Int<N>, S1>
{
    #[inline]
    fn partial_cmp(&self, other: &D<Int<M>, S2>) -> Option<core::cmp::Ordering> {
        Some(crate::policy::dcmp::dcmp_dispatch::<N, M, S1, S2>(self.0, other.0))
    }
}

// Same-type total order via the policy dispatcher (same-scale fast path).
impl<const N: usize, const S: u32> Ord for D<Int<N>, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        crate::policy::dcmp::dcmp_dispatch::<N, N, S, S>(self.0, other.0)
    }
}

impl<S: core::hash::Hash, const SCALE: u32> core::hash::Hash for D<S, SCALE> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
