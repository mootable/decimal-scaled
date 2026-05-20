//! Per-family policy traits — which algorithm each `Dxx<S>` calls.
//!
//! The typed method shell on each `Dxx<S>` (e.g. `D57::<SCALE>::sqrt_strict`)
//! delegates to a policy trait method (`SqrtPolicy::sqrt_impl`). The
//! policy trait is implemented once per width, generic over `SCALE`,
//! with a `const SCALE`-branch picking the right kernel from
//! [`crate::algos`] for each cell:
//!
//! ```ignore
//! impl<const SCALE: u32> SqrtPolicy for D57<SCALE> {
//!     fn sqrt_impl(self, mode: RoundingMode) -> Self {
//!         if SCALE == 20 {
//!             // bespoke kernel for D57<20>
//!             D57(algos::sqrt::lookup_d57_s20::sqrt_with(self.0, SCALE, mode))
//!         } else {
//!             // default
//!             D57(algos::sqrt::generic_wide::sqrt_with::<_, Int384>(self.0, SCALE, mode))
//!         }
//!     }
//! }
//! ```
//!
//! Because `SCALE` is `const`, the `if` is dead-code-eliminated per
//! monomorphisation — every concrete `D57<S>` compiles to a direct call
//! to one kernel only. Zero runtime dispatch cost.
//!
//! Stable Rust does not allow trait-impl specialisation on
//! const-generic types, so per-(width, scale) overrides live as
//! `if SCALE == X` arms inside the per-width policy impl rather than
//! as separate `impl SqrtPolicy for D57<20>` blocks. The override
//! list is grep-able in one place per family.

// Unconditional — D9/D18/D38 impls live here too. Wide-tier impls
// inside each family are individually feature-gated.
pub(crate) mod cbrt;
pub(crate) mod exp;
pub(crate) mod ln;
pub(crate) mod pow;
pub(crate) mod sqrt;
pub(crate) mod triplet;
pub(crate) mod trig;
