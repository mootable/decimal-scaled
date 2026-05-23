// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Cross-algorithm support library — the shared **leaf** bucket.
//!
//! `algo_x_support` is the one home for support code that several
//! algorithm families share *across tiers* (integer and decimal alike).
//! Everything here obeys the **leaf invariant**:
//!
//! - A support leaf calls **nothing** in-crate. It composes only on Rust
//!   primitives (and, where a leaf has a `std`-specific fast path, the
//!   **inherent** `f64` intrinsics such as `(x as f64).sqrt()` /
//!   `.cbrt()`). Algorithms call *into* leaves; leaves call back into
//!   nothing.
//! - **std owns floats; `no_std` is integer-only.** A leaf's `std` body may
//!   use inherent `f64` methods (no `libm`, the intrinsics are built into
//!   the compiler under `std`). Its `no_std` body is **pure-integer** math
//!   on Rust primitives only — **never** `libm`, **never**
//!   `num_traits::Float`, never any external math crate. The `std`/`no_std`
//!   choice is cfg-swapped *inside* each leaf so callers stay agnostic.
//!
//! The seed library ([`seed`]) is the first inhabitant: initial-estimate
//! leaves for the integer (and, later, decimal) Newton roots.

pub(crate) mod seed;
