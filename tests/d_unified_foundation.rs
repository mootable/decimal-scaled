//! Sanity tests for the unified `D<S, SCALE>` foundation.
//!
//! At this stage `D<S, SCALE>` carries the struct definition +
//! hand-rolled `Clone` / `Copy` / `PartialEq` / `Eq` /
//! `PartialOrd` / `Ord` / `Hash` impls (no arithmetic, no constants,
//! no method surface yet — those land as per-storage impls in
//! follow-up commits during width migration). These tests exist so
//! the foundation has real coverage before migration starts loading
//! it down with per-storage `impl<const SCALE: u32> D<…, SCALE>`
//! blocks.
//!
//! `Debug` is provided per-storage by the width-specific display
//! macro (`decl_decimal_display!`), not as a blanket on `D<S, SCALE>`
//! — so it is exercised by the per-width Debug tests, not here.

use core::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use decimal_scaled::D;

#[test]
fn construct_and_access_raw() {
    let d: D<i128, 3> = D(1500);
    assert_eq!(d.0, 1500);
}

#[test]
fn copy_and_clone() {
    let a: D<i64, 2> = D(42);
    let b = a; // Copy
    let c = a; // Clone
    assert_eq!(a.0, b.0);
    assert_eq!(a.0, c.0);
}

#[test]
fn equality_by_raw_storage() {
    // Uses `Int<1>` storage: `Debug` (needed by assert_eq!/assert_ne!) is
    // emitted per-storage by `decl_decimal_display!`. The primitive `i32`/
    // `i64`/`i128` types are no longer decimal storages (D18 backs onto
    // `Int<1>`, D38 onto `Int<2>`), so they carry no per-storage `Debug`.
    let a: D<decimal_scaled::Int<1>, 5> = D(decimal_scaled::Int::<1>::from(123_i64));
    let b: D<decimal_scaled::Int<1>, 5> = D(decimal_scaled::Int::<1>::from(123_i64));
    let c: D<decimal_scaled::Int<1>, 5> = D(decimal_scaled::Int::<1>::from(124_i64));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn ordering_by_raw_storage() {
    // `i64` is no longer a decimal storage (D18 backs onto `Int<1>`); ordering
    // operators live only on `Int<N>`-backed `D`, so use `Int<1>` here (mirrors
    // `equality_by_raw_storage`).
    let a: D<decimal_scaled::Int<1>, 4> = D(decimal_scaled::Int::<1>::from(10i64));
    let b: D<decimal_scaled::Int<1>, 4> = D(decimal_scaled::Int::<1>::from(20i64));
    assert!(a < b);
    assert_eq!(a.cmp(&b), Ordering::Less);
    assert_eq!(b.cmp(&a), Ordering::Greater);
    assert_eq!(a.cmp(&a), Ordering::Equal);
}

#[test]
fn hashable_consistent_with_eq() {
    let a: D<i128, 6> = D(9999);
    let b: D<i128, 6> = D(9999);
    let mut ha = DefaultHasher::new();
    let mut hb = DefaultHasher::new();
    a.hash(&mut ha);
    b.hash(&mut hb);
    assert_eq!(ha.finish(), hb.finish());
}

// `debug_includes_scale_and_raw` removed: `Debug` is no longer a
// blanket impl on `D<S, SCALE>` (it's emitted per-storage by
// `decl_decimal_display!` so the formatted form is the canonical
// decimal string, not the raw integer). Per-width Debug coverage
// lives in the width-specific display tests.

#[test]
fn repr_transparent_size_matches_storage() {
    assert_eq!(
        core::mem::size_of::<D<i32, 0>>(),
        core::mem::size_of::<i32>()
    );
    assert_eq!(
        core::mem::size_of::<D<i64, 0>>(),
        core::mem::size_of::<i64>()
    );
    assert_eq!(
        core::mem::size_of::<D<i128, 0>>(),
        core::mem::size_of::<i128>()
    );
}

#[test]
fn different_scales_are_distinct_types() {
    // This test exists to assert the type-level distinction even when
    // raw storage matches — it would fail to compile if D<S, A> and
    // D<S, B> were considered the same type. The body just observes
    // that we can hold both without coercion.
    let a: D<i64, 3> = D(100);
    let b: D<i64, 5> = D(100);
    assert_eq!(a.0, b.0);
    // a == b is a type error (different `SCALE`), demonstrating the
    // const-generic distinction. Don't try to compare them.
    let _ = (a, b);
}
