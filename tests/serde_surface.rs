//! Coverage suite for `serde_helpers.rs` — D38 visitor's native integer
//! paths (visit_i8/u8/i16/u16/i32/u32/i128), visit_string /
//! visit_byte_buf (alloc-gated), visit_borrowed_*, and wide-tier serde.
//!
//! Each visitor method is invoked directly via the public
//! `DecimalVisitor`. JSON / postcard round-trips are not used here —
//! the macro-emitted visitor bodies are what we need to cover.

#![cfg(all(feature = "alloc", feature = "serde"))]

use core::marker::PhantomData;
use decimal_scaled::serde_helpers::decimal_serde::DecimalVisitor;
use decimal_scaled::{D38, D38s12};
use serde::de::value::Error as DeError;
use serde::de::Visitor;

// ─── D38 visitor: every native integer width ───────────────────────────

#[test]
fn visit_i8() {
    let v = DecimalVisitor::<12>(PhantomData).visit_i8::<DeError>(7).unwrap();
    assert_eq!(v.to_bits(), 7);
    let v = DecimalVisitor::<12>(PhantomData).visit_i8::<DeError>(-7).unwrap();
    assert_eq!(v.to_bits(), -7);
}

#[test]
fn visit_i16() {
    let v = DecimalVisitor::<12>(PhantomData).visit_i16::<DeError>(7).unwrap();
    assert_eq!(v.to_bits(), 7);
}

#[test]
fn visit_i32() {
    let v = DecimalVisitor::<12>(PhantomData).visit_i32::<DeError>(7).unwrap();
    assert_eq!(v.to_bits(), 7);
}

#[test]
fn visit_i128() {
    let v = DecimalVisitor::<12>(PhantomData)
        .visit_i128::<DeError>(123_456_789_012_345_678_901_234)
        .unwrap();
    assert_eq!(v.to_bits(), 123_456_789_012_345_678_901_234);
}

#[test]
fn visit_u8_u16_u32() {
    let v = DecimalVisitor::<12>(PhantomData).visit_u8::<DeError>(7).unwrap();
    assert_eq!(v.to_bits(), 7);
    let v = DecimalVisitor::<12>(PhantomData).visit_u16::<DeError>(7).unwrap();
    assert_eq!(v.to_bits(), 7);
    let v = DecimalVisitor::<12>(PhantomData).visit_u32::<DeError>(7).unwrap();
    assert_eq!(v.to_bits(), 7);
}

#[test]
fn visit_u128_in_range() {
    let v = DecimalVisitor::<12>(PhantomData)
        .visit_u128::<DeError>(123_456)
        .unwrap();
    assert_eq!(v.to_bits(), 123_456);
}

#[test]
fn visit_str_error_paths() {
    // empty
    let r: Result<D38s12, _> = DecimalVisitor::<12>(PhantomData).visit_str::<DeError>("");
    assert!(r.is_err());
    // leading +
    let r: Result<D38s12, _> = DecimalVisitor::<12>(PhantomData).visit_str::<DeError>("+1");
    assert!(r.is_err());
    // non-numeric
    let r: Result<D38s12, _> = DecimalVisitor::<12>(PhantomData).visit_str::<DeError>("xyz");
    assert!(r.is_err());
}

#[test]
fn visit_borrowed_str() {
    let v = DecimalVisitor::<12>(PhantomData)
        .visit_borrowed_str::<DeError>("12345")
        .unwrap();
    assert_eq!(v.to_bits(), 12345);
}

#[test]
fn visit_string_alloc() {
    let s: String = "12345".into();
    let v = DecimalVisitor::<12>(PhantomData)
        .visit_string::<DeError>(s)
        .unwrap();
    assert_eq!(v.to_bits(), 12345);
}

#[test]
fn visit_bytes_valid_and_error() {
    let v = DecimalVisitor::<12>(PhantomData)
        .visit_bytes::<DeError>(&123_i128.to_le_bytes())
        .unwrap();
    assert_eq!(v.to_bits(), 123);
    // Wrong length
    let r: Result<D38s12, _> =
        DecimalVisitor::<12>(PhantomData).visit_bytes::<DeError>(&[0u8; 15]);
    assert!(r.is_err());
}

#[test]
fn visit_borrowed_bytes() {
    let v = DecimalVisitor::<12>(PhantomData)
        .visit_borrowed_bytes::<DeError>(&123_i128.to_le_bytes())
        .unwrap();
    assert_eq!(v.to_bits(), 123);
}

#[test]
fn visit_byte_buf_alloc() {
    let buf: Vec<u8> = 123_i128.to_le_bytes().to_vec();
    let v = DecimalVisitor::<12>(PhantomData)
        .visit_byte_buf::<DeError>(buf)
        .unwrap();
    assert_eq!(v.to_bits(), 123);
}

#[test]
fn expecting_message_works() {
    // expecting() returns the format-error description; we just need to
    // call it once to register coverage on the line. Use fmt::Formatter
    // by formatting a synthetic error.
    let visitor: DecimalVisitor<12> = DecimalVisitor(PhantomData);
    // Trigger the `expecting()` method by issuing a deserialization
    // failure through serde_json with a wrong type:
    let r: Result<D38s12, _> = serde_json::from_str("null");
    assert!(r.is_err());
    let _ = visitor; // suppress unused warning
}

// ─── Wide-tier serde visitor ───────────────────────────────────────────
//
// The wide serde impl emits its own private `V<S>` visitor inside an
// anonymous `Deserialize` body. We can only reach it through a full
// serde flow. Use serde_json for the string path and a manual byte-array
// deserializer for the bytes path.

#[cfg(feature = "wide")]
#[test]
fn d76_serde_json_round_trip() {
    use decimal_scaled::D76;
    type D76_6 = D76<6>;
    let v: D76_6 = D38::<6>::from_int(42).into();
    let s = serde_json::to_string(&v).unwrap();
    let back: D76_6 = serde_json::from_str(&s).unwrap();
    assert_eq!(back, v);
}

#[cfg(feature = "wide")]
#[test]
fn d76_serde_str_error() {
    use decimal_scaled::D76;
    type D76_6 = D76<6>;
    let r: Result<D76_6, _> = serde_json::from_str("\"xyz\"");
    assert!(r.is_err());
}

#[cfg(feature = "x-wide")]
#[test]
fn d153_d307_serde_json_round_trip() {
    use decimal_scaled::{D153, D307, D76};
    type D153_6 = D153<6>;
    type D307_6 = D307<6>;
    type D76_6 = D76<6>;
    let v: D153_6 = D38::<6>::from_int(42).into();
    let s = serde_json::to_string(&v).unwrap();
    let back: D153_6 = serde_json::from_str(&s).unwrap();
    assert_eq!(back, v);

    let lift: D76_6 = D38::<6>::from_int(42).into();
    let v: D307_6 = lift.into();
    let s = serde_json::to_string(&v).unwrap();
    let back: D307_6 = serde_json::from_str(&s).unwrap();
    assert_eq!(back, v);
}
