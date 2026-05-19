//! Algorithm library for numerical kernels.
//!
//! Self-contained algorithm implementations grouped by mathematical
//! family. Each kernel is a free function that takes the **raw storage
//! integer** plus runtime parameters (scale, rounding mode, …) and
//! returns a raw storage integer. None of them know about the typed
//! `Dxx<S>` wrapper; that lives in the type's method shell.
//!
//! # Why a separate algorithm library
//!
//! Three properties we want:
//!
//! 1. **Direct benchability.** A kernel is a function, not a method on
//!    a typed value; benchmarks can call it head-to-head against an
//!    alternative without going through the type wrapper.
//! 2. **Replaceable per (width, scale).** The choice of which kernel a
//!    given `Dxx<S>` uses lives in `crate::policy`, not in the type's
//!    method body. Swapping the chosen winner for a single cell is a
//!    one-line change in the policy file.
//! 3. **Multiple implementations side-by-side.** Newton vs Goldschmidt
//!    sqrt, Taylor vs AGM ln, lookup-table-bootstrapped vs
//!    cold-start — they coexist in this tree, ready to be promoted by
//!    a policy override.
//!
//! # Layout
//!
//! `algos::<family>::<variant>` — `algos::sqrt::generic_wide`,
//! `algos::sqrt::lookup_d57_s20`, `algos::ln::taylor`, etc. The
//! `<variant>` name describes the algorithm, not the type that uses
//! it; one variant may serve many cells.

// `sqrt` / `cbrt` are unconditional — the narrow tier (D9/D18/D38)
// kernels in `<family>::mg_divide_d38` and `<family>::widen_to_d38`
// are always built; each wide-tier kernel inside
// `<family>::generic_wide` is independently feature-gated.
pub(crate) mod cbrt;
pub(crate) mod exp;
pub(crate) mod ln;
pub(crate) mod pow;
pub(crate) mod sqrt;
pub(crate) mod trig;

// Shared kernels consumed by multiple families. `mg_divide` is the
// Moller-Granlund magic-number divide used by every multiplicative
// path; `fixed_d38` is the 256-bit sign-magnitude `Fixed` type used
// by the strict-transcendental fallback paths.
pub(crate) mod mg_divide;
pub(crate) mod fixed_d38;

// Research kernel — Newton-Raphson reciprocal divide for `n / 10^SCALE`.
// Gated to the deepest wide tiers only; head-to-head benched against
// [`mg_divide::div_wide_pow10_chain_with`] in `benches/newton_vs_mg.rs`.
#[cfg(any(feature = "d616", feature = "d924", feature = "d1232", feature = "x-wide", feature = "xx-wide"))]
pub mod newton_reciprocal;
