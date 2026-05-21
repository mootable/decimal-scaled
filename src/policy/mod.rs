//! Per-family policy traits — which algorithm each `Dxx<S>` calls.
//!
//! The typed method shell on each `Dxx<S>` (e.g. `D57::<SCALE>::sqrt_strict`)
//! delegates to a policy trait method (`SqrtPolicy::sqrt_impl`). The
//! policy trait is implemented once per width, generic over `SCALE`,
//! routing through a `match (W, SCALE)` table that picks the right
//! kernel from [`crate::algos`] for each cell. The table is emitted by
//! [`triplet::policy_triplet`] as `base`/`std`/`no_std` free fns keyed on
//! a const `match (W, SCALE)`, with inclusive range patterns keeping the
//! arm count down and a per-family catch-all:
//!
//! ```ignore
//! policy_triplet! {
//!     storage = crate::wide_int::Int192,
//!     base_fn = sqrt_d57_base, std_fn = sqrt_d57_std, no_std_fn = sqrt_d57_no_std,
//!     recv = raw, mode = mode, params = {},
//!     base = {
//!         (wtag::D57, 20) => sqrt::lookup_d57_s20::sqrt(raw, mode),
//!         (wtag::D57, _)  => sqrt::generic_wide::sqrt_d57(raw, SCALE, mode),
//!     },
//!     std = { (wtag::D57, 20) => sqrt::lookup_d57_s20::sqrt(raw, mode) },
//! }
//! ```
//!
//! Because both `W` and `SCALE` are `const` at every monomorphisation,
//! the `match` const-folds to its single live arm — every concrete
//! `D57<S>` compiles to a direct call to one kernel only. Zero runtime
//! dispatch cost.
//!
//! Stable Rust does not allow trait-impl specialisation on
//! const-generic types, so per-(width, scale) overrides live as
//! `match (W, SCALE)` arms inside the per-width triplet rather than as
//! separate `impl SqrtPolicy for D57<20>` blocks. The override list is
//! grep-able in one place per family. The few inverse / hyperbolic trig
//! methods whose fall-through is an inherent `*_strict_with` shell (not a
//! raw-storage kernel) keep a hand-written `match SCALE` instead — the
//! triplet emits raw-storage free fns only.

// Unconditional — D9/D18/D38 impls live here too. Wide-tier impls
// inside each family are individually feature-gated.
pub(crate) mod cbrt;
pub(crate) mod exp;
pub(crate) mod ln;
pub(crate) mod pow;
pub(crate) mod sqrt;
pub(crate) mod triplet;
pub(crate) mod trig;
