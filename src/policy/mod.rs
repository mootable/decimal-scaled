//! Per-family policy traits — which algorithm each `Dxx<S>` calls.
//!
//! The typed method shell on each `Dxx<S>` (e.g. `D57::<SCALE>::exp_strict`)
//! delegates to the policy `dispatch` fn (`exp::dispatch`). Every family
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
pub(crate) mod dcmp;
pub(crate) mod deq;
pub(crate) mod div;
pub(crate) mod exp;
pub(crate) mod hypot;
pub(crate) mod ln;
pub(crate) mod log;
pub(crate) mod mul;
pub(crate) mod neg;
pub(crate) mod pow;
pub(crate) mod rem;
pub(crate) mod sqrt;
pub(crate) mod sub;
pub(crate) mod to_degrees;
pub(crate) mod to_radians;
pub(crate) mod trig;

// ── Narrow-tier checked narrow ──────────────────────────────────────
//
// The exp/ln/log/pow policies route the narrow tier (N == 1, i.e. D18,
// and the identity N == 2, i.e. D38) by computing the result at Int<2>
// (D38 width) and narrowing back to storage. For N == 1 that narrow is
// LOSSY and must PANIC when the D38 result exceeds the i64 storage range
// (e.g. exp(5) at D18<17>), restoring the documented strict-overflow
// contract. (For N == 2 it is identity; for the wide arms N >= 3 the
// result is already computed at Int<N> and routed through their own
// resize.)
//
// `Int<2>::narrow::<N>()` cannot be used here: these arms are generic
// over N and instantiate for wide N too (dead at runtime), and the
// inherent `narrow::<M>` requires `M <= N` at compile time. Instead we
// resize to the storage width and verify the value survives a round-trip
// back to Int<2>; any discrepancy means the value did not fit and we
// panic via the shared diagnostics helper (stable substring
// `"{method}: result out of range"`).
#[inline]
pub(crate) fn narrow_checked<const N: usize>(
    wide: crate::int::types::Int<2>,
    method: &str,
    scale: u32,
) -> crate::int::types::Int<N> {
    use crate::int::types::traits::BigInt;
    let out = wide.resize_to::<crate::int::types::Int<N>>();
    if out.resize_to::<crate::int::types::Int<2>>() != wide {
        crate::support::diagnostics::overflow_panic_with_scale(method, scale);
    }
    out
}
