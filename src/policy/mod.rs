//! Per-family policy traits — which algorithm each `Dxx<S>` calls.
//!
//! The typed method shell on each `Dxx<S>` (e.g. `D57::<SCALE>::exp_strict`)
//! delegates to a policy trait method (`ExpPolicy::exp_impl`). Every family
//! follows the canonical `(N, SCALE)` matcher (`sqrt` is the exemplar): a
//! per-function `Algorithm` enum + a `const fn select<N, SCALE>()` + an
//! exhaustive `match algo`, dispatched via an inline
//! `const { select::<N, SCALE>() }` block. See [`sqrt`] and
//! `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! The keys (`N` and `SCALE`) are `const` at every monomorphisation, so
//! `select` const-folds to its single live arm — every concrete `Dxx<S>`
//! compiles to a direct call to one kernel. Zero runtime dispatch cost.
//!
//! Stable Rust does not allow trait-impl specialisation on const-generic
//! types, so a width's per-`(N, SCALE)` realisations live as arms inside
//! the canonical `match algo` (with a const-folding inner `match SCALE`
//! where one algorithm has several per-band kernels at a width) rather
//! than as separate `impl SqrtPolicy for D57<20>` blocks. The few inverse
//! / hyperbolic trig methods whose fall-through is an inherent
//! `*_strict_with` shell (not a raw-storage kernel) realise their single
//! algorithm through that shell.

// Unconditional — D18/D38 impls live here too. Wide-tier impls
// inside each family are individually feature-gated.
pub(crate) mod add;
pub(crate) mod cbrt;
pub(crate) mod div;
pub(crate) mod exp;
pub(crate) mod float_seed;
pub(crate) mod ln;
pub(crate) mod mul;
pub(crate) mod neg;
pub(crate) mod pow;
pub(crate) mod rem;
pub(crate) mod sqrt;
pub(crate) mod sub;
pub(crate) mod table_cache;
pub(crate) mod trig;
