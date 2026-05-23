//! Per-family policy traits — which algorithm each `Dxx<S>` calls.
//!
//! The typed method shell on each `Dxx<S>` (e.g. `D57::<SCALE>::exp_strict`)
//! delegates to a policy trait method (`ExpPolicy::exp_impl`). Two policy
//! shapes coexist during the Phase-4 migration:
//!
//! - **The `(N, SCALE)` matcher (migrated; `sqrt` is the exemplar).** A
//!   per-function `Algorithm` enum + a `const fn select<N, SCALE>()` +
//!   an exhaustive `match algo`, dispatched via an inline
//!   `const { select::<N, SCALE>() }` block. See [`sqrt`] and
//!   `docs/ARCHITECTURE.md` → "Policy file structure".
//! - **The legacy `match (W, SCALE)` triplet (not yet migrated).** The
//!   policy trait is implemented once per width, generic over `SCALE`,
//!   routing through a `match (W, SCALE)` table that picks the right
//!   kernel from [`crate::algos`] for each cell. The table is emitted by
//!   [`triplet::policy_triplet`] as `base`/`std`/`no_std` free fns keyed
//!   on a const `match (W, SCALE)`:
//!
//! ```ignore
//! policy_triplet! {
//!     storage = crate::int::types::Int<3>,
//!     base_fn = exp_d57_base, std_fn = exp_d57_std, no_std_fn = exp_d57_no_std,
//!     recv = raw, mode = mode, params = {},
//!     base = { (wtag::D57, _) => exp::wide_kernel::exp_strict_d57(raw, SCALE, mode) },
//!     std  = {},
//! }
//! ```
//!
//! In both shapes the keys (`N`/`W` and `SCALE`) are `const` at every
//! monomorphisation, so the selection const-folds to its single live arm
//! — every concrete `Dxx<S>` compiles to a direct call to one kernel.
//! Zero runtime dispatch cost.
//!
//! Stable Rust does not allow trait-impl specialisation on
//! const-generic types, so per-(width, scale) overrides live as
//! `match (W, SCALE)` arms inside the per-width triplet rather than as
//! separate `impl SqrtPolicy for D57<20>` blocks. The override list is
//! grep-able in one place per family. The few inverse / hyperbolic trig
//! methods whose fall-through is an inherent `*_strict_with` shell (not a
//! raw-storage kernel) keep a hand-written `match SCALE` instead — the
//! triplet emits raw-storage free fns only.

// Unconditional — D18/D38 impls live here too. Wide-tier impls
// inside each family are individually feature-gated.
pub(crate) mod cbrt;
pub(crate) mod exp;
pub(crate) mod float_seed;
pub(crate) mod ln;
pub(crate) mod pow;
pub(crate) mod sqrt;
pub(crate) mod table_cache;
pub(crate) mod trig;
pub(crate) mod triplet;
