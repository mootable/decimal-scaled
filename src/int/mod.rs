// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Const-generic fixed-width integer layer.
//!
//! The integer side of the crate, mirroring the decimal layer's
//! split into three buckets:
//!
//! - [`types`] — the `Int<N>` / `Uint<N>` const-generic type
//!   definitions, their `BigInt` / `BigInt` traits, and the
//!   named `IntXXXX` / `UintXXXX` `pub type` aliases preserved for the
//!   existing call sites.
//! - [`policy`] — per-function algorithm-selection matchers: the
//!   schoolbook-vs-Karatsuba mul picker ([`policy::mul`]) and the
//!   divisor-shape divide picker ([`policy::div_rem`]), each in the canonical
//!   `Select` / `select` / exhaustive-`match algo` shape with the benched
//!   crossover thresholds held as policy data.
//! - [`algos`] — the reusable algorithms the integer types compose on:
//!   the generic `u64` limb arithmetic ([`algos::support::limbs`]), the pure
//!   division engines ([`algos::div`]), and the Newton integer square
//!   root ([`algos::roots`]).

pub(crate) mod algos;
pub(crate) mod convert;
pub(crate) mod policy;
pub(crate) mod types;

#[allow(unused_imports)]
pub(crate) use types::{BigInt, Int, Uint};
