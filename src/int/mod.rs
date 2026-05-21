//! Const-generic fixed-width integer layer.
//!
//! The integer side of the crate, mirroring the decimal layer's
//! split into four buckets:
//!
//! - [`types`] — the `Int<N>` / `Uint<N>` const-generic type
//!   definitions, their `FixedInt` / `FixedIntConvert` traits, and the
//!   named `IntXXXX` / `UintXXXX` `pub type` aliases preserved for the
//!   existing call sites.
//! - [`policy`] — per-width / limb-count algorithm-selection dispatch
//!   (the schoolbook-vs-Karatsuba threshold, the Knuth vs
//!   Burnikel–Ziegler divide selection). See its module docs for what
//!   currently lives inline in the kernels.
//! - [`algos`] — the reusable width-matched algorithms the integer
//!   types compose on (truncated mul / sqr, isqrt, root_int, the
//!   re-exported full-width products and divide entry points).
//! - [`limbs`] — the raw `&[u64]` / `&[u128]` slice limb primitives
//!   (add/sub/cmp/shift/mul/divmod) plus the `decl_wide_int!` named-type
//!   generator, absorbed from the former `src/wide_int/`.

pub(crate) mod limbs;
pub(crate) mod algos;
pub(crate) mod policy;
pub(crate) mod types;

#[allow(unused_imports)]
pub(crate) use types::{FixedInt, FixedIntConvert, Int, Uint};
