// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Reusable width-matched integer algorithms.
//!
//! The integer layer's algorithm bucket, organised by function so the
//! layout mirrors the decimal `crate::algos::<fn>/` tree: each dispatched
//! function's algorithm/kernel fns live in `int/algos/<function>/`, and the
//! per-width *choice* between them lives in `crate::int::policy::<function>`.
//! The call graph only ever points DOWN — a type method delegates to its
//! `policy::<fn>::dispatch`, the dispatch selects an algorithm fn, and the
//! algorithm computes via the kernels here (never a method back on its own
//! type).
//!
//! Function families:
//!
//! - [`mul`] — schoolbook (+ fixed/single-word/truncated specialisations)
//!   and Karatsuba multiply kernels.
//! - [`sqr`] — the half-product squaring algorithm and its low-`N` kernel.
//! - [`cube`] — sqr-then-multiply cubing.
//! - [`pow`] — binary square-and-multiply exponentiation.
//! - [`div`] — the division engines (`div_rem` const fast path, Knuth,
//!   Burnikel–Ziegler, the Möller–Granlund reciprocal estimators) and the
//!   const-`N` fast-arm divmod wrappers.
//! - [`isqrt`] — Newton integer square root + its const-`N` fast-arm wrapper.
//! - [`icbrt`] — Newton integer cube root.
//!
//! Shared low-level limb PRIMITIVES that are not a single function's
//! headline algorithm — comparison, equality, bit-length, ripple add/sub,
//! shifts, single-limb fit test, signed compare — stay in the shared
//! [`support::limbs`] bucket, since they are composed by several families (the
//! divide engines, Karatsuba, the roots, and the type layer itself).

pub(crate) mod cube;
pub(crate) mod div;
pub(crate) mod icbrt;
pub(crate) mod isqrt;
pub(crate) mod mul;
pub(crate) mod pow;
pub(crate) mod sqr;
pub(crate) mod support;
