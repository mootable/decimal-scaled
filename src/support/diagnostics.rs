// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Diagnostic helpers for narrowing-overflow panics.
//!
//! Every strict transcendental kernel computes at a wider working
//! precision and narrows the result back to the storage scale.
//! When the narrowed result does not fit the target type's
//! representable range, the wrapping `try_into` returns `Err` and we
//! panic with a uniform message via this helper.
//!
//! ## Message shape
//!
//! ```text
//! {method}: result out of range — SCALE={scale}
//! ```
//!
//! The substring `"{method}: result out of range"` is held stable
//! across crate versions so existing
//! `#[should_panic(expected = "...")]` tests continue to match; the
//! trailing `— SCALE={scale}` is the new diagnostic surface added
//! to tell the caller which compile-time `SCALE` instantiation
//! tripped the bound (the panic site otherwise can't identify which
//! `Dxx<S>` it came from when the same code path serves many
//! monomorphisations).
//!
//! For the deeper-diagnostic sites in `algos/*/borrow_d57.rs` we
//! also include the offending wide-tier value inline — those
//! wrappers have it in scope cheaply. The macro-emitted call sites
//! in `macros/strict_transcendentals.rs` route through this helper
//! instead, since plumbing the value through the macro emission
//! would inflate the generated code at every per-width
//! instantiation for little incremental signal beyond the
//! `(method, SCALE)` pair.

/// Canonical panic for "result doesn't fit the target type".
///
/// The first parameter is the method label (e.g. `"D38::log_strict"`
/// or `"powf kernel"`); see the module docstring for the formatted
/// message shape.
#[cold]
#[inline(never)]
#[track_caller]
pub(crate) fn overflow_panic_with_scale(method: &str, scale: u32) -> ! {
    panic!("{method}: result out of range — SCALE={scale}");
}
