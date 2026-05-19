//! Equality, ordering, and hashing — the equivalence-class contract for
//! the decimal types.
//!
//! Because every logical value has exactly one representation at a fixed
//! scale, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, and `Ord` are all
//! derived from the underlying integer storage. This bucket groups the
//! impl sites so the contract is visible in one place.
//!
//! `PartialOrd`, `Ord`, and `Hash` impls are emitted by
//! [`crate::macros::basics`] today; only the cross-primitive
//! `PartialEq<primitive>` wiring lives here.

pub(crate) mod equalities;
