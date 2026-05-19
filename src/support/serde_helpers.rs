//! `serde` integration for every decimal width.
//!
//! D38 has a dedicated [`Serialize`] / [`Deserialize`] pair plus the
//! richer [`decimal_serde::DecimalVisitor`] used for `#[serde(with =
//! "...")]` field annotations. The wide tiers (D76 / D153 / D307)
//! use a slimmer implementation emitted by `decl_wide_serde!`: a
//! decimal-string wire format for human-readable serializers and a
//! little-endian limb-bytes wire format for binary serializers.
//! Cross-tier wire-format parity is intentional — a D38 produced
//! at SCALE = 12 serialises to the same string as a D76 at SCALE =
//! 12 carrying the same logical value.
//!
//!
//! # Wire format
//!
//! `D38<SCALE>` chooses its wire encoding based on the serializer's
//! [`serde::Serializer::is_human_readable`] flag:
//!
//! - **Human-readable formats** (JSON, TOML, YAML): a base-10 integer
//! string of the underlying `i128` storage value. For example,
//! `D38s12::ONE` (storage `1_000_000_000_000`) serialises as the
//! JSON string `"1000000000000"`. This is not a decimal string like
//! `"1.0"` — that is the job of `Display`, not the wire format.
//!
//! A string rather than a JSON number is used because JSON numbers
//! are effectively `f64` in most runtimes (max safe integer =
//! `2^53 - 1`), while `i128` storage requires up to 127 bits. A
//! BigInt-compatible integer string is the only lossless option for
//! interoperability with JavaScript, where `BigInt(s).toString()`
//! round-trips the same digits.
//!
//! - **Binary formats** (postcard, bincode, etc.): 16 little-endian
//! bytes from `i128::to_le_bytes`. Compact and endian-canonical.
//!
//! On deserialise, the internal `DecimalVisitor` handles both wire forms plus
//! `visit_i64` / `visit_u64` / `visit_i128` / `visit_u128` callbacks,
//! which are used when the underlying format yields a native integer.
//! The integer is interpreted directly as the scaled `i128` storage.

use core::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "alloc")]
use alloc::string::ToString;

use crate::types::widths::D38;

// ── Serialize ─────────────────────────────────────────────────────────

impl<const SCALE: u32> Serialize for D38<SCALE> {
    /// Serialise `self` as a base-10 integer string for human-readable
    /// formats, or as 16 little-endian bytes for binary formats.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            // Formatting an i128 as a decimal string requires heap
            // allocation. Every real human-readable format already
            // depends on alloc, so this is not a practical constraint.
            #[cfg(feature = "alloc")]
            {
                serializer.serialize_str(&self.0.to_string())
            }
            // Human-readable serialisation without alloc is not
            // supported. A 40-byte stack buffer would technically
            // suffice for an i128 decimal, but no real target combines
            // `no_std + !alloc + serde + human-readable format`.
            #[cfg(not(feature = "alloc"))]
            {
                let _ = serializer;
                Err(serde::ser::Error::custom(
                    "decimal-scaled: human-readable serialisation requires the `alloc` feature",
                ))
            }
        } else {
            // Binary path: emit the raw i128 as 16 little-endian bytes.
            serializer.serialize_bytes(&self.0.to_le_bytes())
        }
    }
}

// ── Deserialize ───────────────────────────────────────────────────────

impl<'de, const SCALE: u32> Deserialize<'de> for D38<SCALE> {
    /// Deserialise from a base-10 integer string (human-readable
    /// formats), 16 little-endian bytes (binary formats), or a native
    /// integer (self-describing binary formats such as CBOR).
    ///
    /// Human-readable formats route via `deserialize_any` so a JSON
    /// string, JSON number, or TOML integer all reach the correct
    /// visitor branch. Binary formats that are not self-describing
    /// (postcard, bincode) route via `deserialize_bytes` directly.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let visitor = decimal_serde::DecimalVisitor::<SCALE>(PhantomData);
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(visitor)
        } else {
            deserializer.deserialize_bytes(visitor)
        }
    }
}

// ── Free-function helpers and visitor ─────────────────────────────────

/// Serde helper module for `#[serde(with = "...")]` field annotations.
///
/// Use this module when you want to control serialisation of a `D38`
/// field on a struct that derives `Serialize` / `Deserialize`:
///
/// ```ignore
/// use decimal_scaled::D38;
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct MyStruct {
/// #[serde(with = "decimal_scaled::serde_helpers::decimal_serde")]
/// length: D38<12>,
/// }
/// ```
///
/// The free functions delegate to the inherent `Serialize` /
/// `Deserialize` impls; they exist so users can annotate
/// `#[serde(with = ...)]` on fields in generic containing types or in
/// newtype wrappers where the trait impl may be shadowed.
pub mod decimal_serde {
    use super::{Serializer, D38, Serialize, Deserializer, Deserialize, PhantomData, Visitor};

    /// Serialise `v` using the `D38` wire format.
    ///
    /// Intended for use under `#[serde(serialize_with = "...")]` or
    /// `#[serde(with = "...")]`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    pub fn serialize<const SCALE: u32, S: Serializer>(
        v: &D38<SCALE>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        v.serialize(s)
    }

    /// Deserialise a `D38` using the wire format.
    ///
    /// Intended for use under `#[serde(deserialize_with = "...")]` or
    /// `#[serde(with = "...")]`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    pub fn deserialize<'de, const SCALE: u32, D: Deserializer<'de>>(
        d: D,
    ) -> Result<D38<SCALE>, D::Error> {
        D38::<SCALE>::deserialize(d)
    }

    /// Visitor that backs [`deserialize`]. Public so external helper
    /// modules can reuse it under custom `#[serde(deserialize_with)]`
    /// shapes.
    ///
    /// Accepted inputs:
    ///
    /// - `&str` / borrowed string: parsed as a strict base-10 `i128`
    /// integer (no decimal point, no whitespace, no leading `+`).
    /// - `&[u8]` / byte buf: interpreted as exactly 16 little-endian
    /// `i128` bytes.
    /// - Native integer (`i8` through `i128`, `u8` through `u128`):
    /// widened into `i128` storage directly. The integer is treated as
    /// the scaled storage value, not as a logical decimal value.
    pub struct DecimalVisitor<const SCALE: u32>(pub PhantomData<()>);

    impl<'de, const SCALE: u32> Visitor<'de> for DecimalVisitor<SCALE> {
        type Value = D38<SCALE>;

        fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(
                "a base-10 i128 integer string, 16 little-endian bytes, \
                 or a native integer",
            )
        }

        // ── String wire form (human-readable) ─────────────────────────

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            // The wire format is a strict base-10 i128 integer literal
            // matching `-?[0-9]+`. No whitespace, no leading `+`, no
            // decimal point, no scientific notation, no underscores.
            // Display's canonical decimal form (e.g. "1.500") is NOT
            // accepted here — that belongs to the FromStr parse path.
            //
            // A leading `+` is rejected explicitly to keep one canonical
            // wire form per value, matching JavaScript BigInt.toString()
            // output which is never `+`-prefixed.
            let bytes = v.as_bytes();
            if bytes.is_empty() {
                return Err(serde::de::Error::custom(
                    "decimal-scaled: empty string is not a valid i128 wire",
                ));
            }
            if bytes[0] == b'+' {
                return Err(serde::de::Error::custom(
                    "decimal-scaled: leading `+` is not part of the canonical wire format",
                ));
            }
            v.parse::<i128>()
                .map(D38::<SCALE>::from_bits)
                .map_err(|_| {
                    serde::de::Error::custom(
                        "decimal-scaled: expected a base-10 i128 integer string",
                    )
                })
        }

        fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
            self.visit_str(v)
        }

        #[cfg(feature = "alloc")]
        fn visit_string<E: serde::de::Error>(
            self,
            v: alloc::string::String,
        ) -> Result<Self::Value, E> {
            self.visit_str(&v)
        }

        // ── Bytes wire form (binary) ───────────────────────────────────

        fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
            // Require exactly 16 bytes: the little-endian i128 layout.
            let arr: [u8; 16] = v.try_into().map_err(|_| {
                serde::de::Error::invalid_length(
                    v.len(),
                    &"exactly 16 little-endian bytes for an i128",
                )
            })?;
            Ok(D38::<SCALE>::from_bits(i128::from_le_bytes(arr)))
        }

        fn visit_borrowed_bytes<E: serde::de::Error>(
            self,
            v: &'de [u8],
        ) -> Result<Self::Value, E> {
            self.visit_bytes(v)
        }

        #[cfg(feature = "alloc")]
        fn visit_byte_buf<E: serde::de::Error>(
            self,
            v: alloc::vec::Vec<u8>,
        ) -> Result<Self::Value, E> {
            self.visit_bytes(&v)
        }

        // ── Native-integer wire forms ──────────────────────────────────
        //
        // These branches are entered when the underlying format yields a
        // typed integer rather than a string or byte slice (e.g. CBOR
        // major types 0/1, MessagePack integer family). The value is
        // interpreted as the scaled i128 storage, matching the binary
        // serialise path.

        fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(v))
        }

        fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(D38::<SCALE>::from_bits(i128::from(v)))
        }

        fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
            // u128 values above i128::MAX cannot be represented; reject
            // explicitly rather than wrapping silently.
            i128::try_from(v).map(D38::<SCALE>::from_bits).map_err(|_| {
                serde::de::Error::custom(
                    "decimal-scaled: u128 value exceeds i128 storage range",
                )
            })
        }

        // ── Float inputs are not a supported wire format ───────────────
        //
        // The wire format is integer-string or little-endian bytes.
        // Floats are not accepted. If a human-edited TOML file contains
        // a bare integer that fits in i64, the format's deserializer
        // routes via visit_i64 / visit_u64 / visit_i128 above, which is
        // correct. A genuine f64 value (e.g. 1.5) is rejected as
        // "expected i128 integer".
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(all(test, feature = "alloc", feature = "serde"))]
mod tests {
    use super::*;
    use crate::types::widths::{D38, D38s12};
    use serde::de::value::{Error as DeError, StrDeserializer};
    use serde::de::IntoDeserializer;
    use alloc::format;

    // ── String wire form round-trips ──────────────────────────────────

    /// `"0"` deserialises to `ZERO` via the canonical-string path.
    #[test]
    fn deserialize_canonical_zero_string() {
        let de: StrDeserializer<DeError> = "0".into_deserializer();
        let v: D38s12 = D38s12::deserialize(de).unwrap();
        assert_eq!(v, D38s12::ZERO);
    }

    /// The visitor accepts the scaled integer representation of `ONE`
    /// (`10^12` for `D38s12`) when fed via `visit_str`.
    #[test]
    fn visitor_accepts_scaled_one_str() {
        let visitor = decimal_serde::DecimalVisitor::<12>(PhantomData);
        let v: D38s12 =
            <_ as Visitor>::visit_str::<DeError>(visitor, "1000000000000").unwrap();
        assert_eq!(v, D38s12::ONE);
    }

    /// The visitor rejects a decimal-point string. `"1.5"` is the
    /// Display format, not the wire format.
    #[test]
    fn visitor_rejects_decimal_point_str() {
        let visitor = decimal_serde::DecimalVisitor::<12>(PhantomData);
        let res: Result<D38s12, _> =
            <_ as Visitor>::visit_str::<DeError>(visitor, "1.5");
        assert!(res.is_err(), "expected reject; got Ok({:?})", res);
    }

    // ── Native-integer wire form round-trips ──────────────────────────

    /// `visit_i64` interprets the input as scaled storage; `-5` stored
    /// directly produces a `D38` whose raw bits are `-5`.
    #[test]
    fn visitor_accepts_i64_as_storage() {
        let visitor = decimal_serde::DecimalVisitor::<12>(PhantomData);
        let v: D38s12 = <_ as Visitor>::visit_i64::<DeError>(visitor, -5).unwrap();
        assert_eq!(v.to_bits(), -5);
    }

    /// `visit_u64` with `u64::MAX` widens cleanly into `i128` storage.
    #[test]
    fn visitor_accepts_u64_max() {
        let visitor = decimal_serde::DecimalVisitor::<12>(PhantomData);
        let v: D38s12 =
            <_ as Visitor>::visit_u64::<DeError>(visitor, u64::MAX).unwrap();
        assert_eq!(v.to_bits(), u64::MAX as i128);
    }

    /// `visit_u128` past `i128::MAX` yields an explicit out-of-range
    /// error rather than wrapping silently.
    #[test]
    fn visitor_rejects_u128_above_i128_max() {
        let visitor = decimal_serde::DecimalVisitor::<12>(PhantomData);
        let res: Result<D38s12, _> = <_ as Visitor>::visit_u128::<DeError>(
            visitor,
            (i128::MAX as u128) + 1,
        );
        assert!(res.is_err(), "expected overflow reject; got Ok({:?})", res);
    }

    // ── JSON round-trips ──────────────────────────────────────────────

    /// `D38s12::ONE` serialises as the JSON string `"1000000000000"`.
    /// This is the BigInt-compatible wire form, not the Display form
    /// `"1.000000000000"`.
    #[test]
    fn json_one_serialises_as_scaled_integer_string() {
        let json = serde_json::to_string(&D38s12::ONE).unwrap();
        assert_eq!(json, "\"1000000000000\"");
    }

    #[test]
    fn json_zero_serialises_as_zero_string() {
        let json = serde_json::to_string(&D38s12::ZERO).unwrap();
        assert_eq!(json, "\"0\"");
    }

    #[test]
    fn json_one_round_trips() {
        let json = serde_json::to_string(&D38s12::ONE).unwrap();
        let back: D38s12 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, D38s12::ONE);
    }

    #[test]
    fn json_zero_round_trips() {
        let json = serde_json::to_string(&D38s12::ZERO).unwrap();
        let back: D38s12 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, D38s12::ZERO);
    }

    /// Negative values round-trip through JSON. `from(-5_i32)` stores
    /// `-5 * 10^12 = -5_000_000_000_000`.
    #[test]
    fn json_negative_round_trips() {
        let v = D38s12::from(-5_i32);
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "\"-5000000000000\"");
        let back: D38s12 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, v);
        assert_eq!(back.to_bits(), -5_000_000_000_000_i128);
    }

    /// `D38::MAX` and `D38::MIN` round-trip exactly through the
    /// JSON-string wire format.
    #[test]
    fn json_max_round_trips() {
        let json = serde_json::to_string(&D38s12::MAX).unwrap();
        let back: D38s12 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, D38s12::MAX);
    }

    #[test]
    fn json_min_round_trips() {
        let json = serde_json::to_string(&D38s12::MIN).unwrap();
        let back: D38s12 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, D38s12::MIN);
    }

    /// The JSON string representation matches `i128::to_string` exactly.
    /// On the JavaScript side, `BigInt(json).toString()` reproduces the
    /// same digits.
    #[test]
    fn json_string_matches_i128_to_string() {
        let raw: i128 = -123_456_789_012_345_678_901_234_567_890_i128;
        let v = D38s12::from_bits(raw);
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, format!("\"{}\"", raw));
    }

    // ── JSON: malformed input rejection ───────────────────────────────

    #[test]
    fn json_rejects_decimal_point_string() {
        let res: Result<D38s12, _> = serde_json::from_str("\"1.5\"");
        assert!(res.is_err(), "expected reject; got Ok({:?})", res);
    }

    #[test]
    fn json_rejects_scientific_notation_string() {
        let res: Result<D38s12, _> = serde_json::from_str("\"1e6\"");
        assert!(res.is_err(), "expected reject; got Ok({:?})", res);
    }

    #[test]
    fn json_rejects_not_a_number_string() {
        let res: Result<D38s12, _> = serde_json::from_str("\"not-a-number\"");
        assert!(res.is_err(), "expected reject; got Ok({:?})", res);
    }

    #[test]
    fn json_rejects_empty_string() {
        let res: Result<D38s12, _> = serde_json::from_str("\"\"");
        assert!(res.is_err(), "expected reject; got Ok({:?})", res);
    }

    #[test]
    fn json_rejects_leading_whitespace_string() {
        // `i128::from_str` does not trim whitespace; the wire format
        // requires a strict integer literal.
        let res: Result<D38s12, _> = serde_json::from_str("\"  42\"");
        assert!(res.is_err(), "expected reject; got Ok({:?})", res);
    }

    #[test]
    fn json_rejects_plus_prefix() {
        let res: Result<D38s12, _> = serde_json::from_str("\"+42\"");
        assert!(res.is_err(), "expected reject; got Ok({:?})", res);
    }

    /// A bare JSON integer (not a string) is accepted via `visit_i64`.
    /// The number is interpreted as the scaled storage value.
    #[test]
    fn json_accepts_bare_integer_number_as_storage() {
        let back: D38s12 = serde_json::from_str("42").unwrap();
        assert_eq!(back.to_bits(), 42_i128);
    }

    // ── Postcard binary 16-byte LE round-trips ────────────────────────

    #[test]
    fn postcard_one_round_trips() {
        let bytes: alloc::vec::Vec<u8> = postcard::to_allocvec(&D38s12::ONE).unwrap();
        // Verify the raw 16 LE bytes appear somewhere in the postcard
        // output (postcard may prepend a varint length prefix).
        let raw = D38s12::ONE.to_bits().to_le_bytes();
        assert!(bytes.windows(16).any(|w| w == raw));
        let back: D38s12 = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(back, D38s12::ONE);
    }

    #[test]
    fn postcard_zero_round_trips() {
        let bytes: alloc::vec::Vec<u8> = postcard::to_allocvec(&D38s12::ZERO).unwrap();
        let back: D38s12 = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(back, D38s12::ZERO);
    }

    #[test]
    fn postcard_negative_round_trips() {
        let v = D38s12::from(-5_i32);
        let bytes: alloc::vec::Vec<u8> = postcard::to_allocvec(&v).unwrap();
        let back: D38s12 = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn postcard_max_round_trips() {
        let bytes: alloc::vec::Vec<u8> = postcard::to_allocvec(&D38s12::MAX).unwrap();
        let back: D38s12 = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(back, D38s12::MAX);
    }

    #[test]
    fn postcard_min_round_trips() {
        let bytes: alloc::vec::Vec<u8> = postcard::to_allocvec(&D38s12::MIN).unwrap();
        let back: D38s12 = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(back, D38s12::MIN);
    }

    /// The postcard payload contains the raw `i128::to_le_bytes`
    /// representation. The first LE byte is the LSB and the last is
    /// the MSB.
    #[test]
    fn postcard_byte_order_matches_le() {
        let v = D38s12::from_bits(0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210_i128);
        let bytes: alloc::vec::Vec<u8> = postcard::to_allocvec(&v).unwrap();
        let raw = v.to_bits().to_le_bytes();
        let found = bytes.windows(16).position(|w| w == raw);
        assert!(found.is_some(), "expected raw LE bytes embedded; got {:?}", bytes);
        assert_eq!(raw[0], 0x10);  // LSB of the i128
        assert_eq!(raw[15], 0x01); // MSB of the i128
    }

    // ── Cross-format compatibility ─────────────────────────────────────

    /// The JSON integer string, when parsed back to `i128` and converted
    /// to `to_le_bytes`, matches the binary wire representation directly.
    #[test]
    fn cross_format_json_string_matches_le_bytes() {
        let v = D38s12::from(42_i32);
        let json = serde_json::to_string(&v).unwrap();
        let inner = json.trim_matches('"');
        let parsed: i128 = inner.parse().unwrap();
        let json_bytes = parsed.to_le_bytes();
        let direct_bytes = v.to_bits().to_le_bytes();
        assert_eq!(json_bytes, direct_bytes);
    }

    /// Different SCALE values serialise identically when they share the
    /// same raw storage. The SCALE is a compile-time type parameter and
    /// is not encoded in the wire.
    #[test]
    fn cross_scale_wire_is_storage_only() {
        let raw: i128 = 1_500_000_000_000;
        let v12 = D38::<12>::from_bits(raw);
        let v6 = D38::<6>::from_bits(raw);
        assert_eq!(serde_json::to_string(&v12).unwrap(), "\"1500000000000\"");
        assert_eq!(serde_json::to_string(&v6).unwrap(), "\"1500000000000\"");
    }

    // ── decimal_serde free-function helpers ───────────────────────────

    /// The `#[serde(with = "...")]` helpers delegate to the inherent
    /// impls and produce the correct JSON output.
    #[test]
    fn decimal_serde_helper_round_trips() {
        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
        struct Holder {
            #[serde(with = "crate::serde_helpers::decimal_serde")]
            length: D38<12>,
        }

        let h = Holder {
            length: D38s12::from(7_i32),
        };
        let json = serde_json::to_string(&h).unwrap();
        assert_eq!(json, r#"{"length":"7000000000000"}"#);
        let back: Holder = serde_json::from_str(&json).unwrap();
        assert_eq!(back, h);
    }
}

// ─── Wide-tier serde (D76 / D153 / D307) ────────────────────────────
//
// The wide-tier wire format mirrors D38's: a base-10 integer string
// of the raw storage value for human-readable serializers, and the
// raw little-endian limb bytes for binary serializers. The
// implementation is intentionally slimmer than D38's — no
// native-integer visit methods, since no native int can losslessly
// carry the >128-bit storage anyway.

/// Emits `Serialize` / `Deserialize` for a wide-tier decimal type
/// (D76 / D153 / D307). `$bytes_len` is `mem::size_of::<$Storage>()`
/// (e.g. 32 for `Int256`).
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide", feature = "x-wide"))]
macro_rules! decl_wide_serde {
    ($Type:ident, $Storage:ty, $bytes_len:literal) => {
        impl<const SCALE: u32> Serialize for $crate::types::widths::$Type<SCALE> {
            /// Serialise as a base-10 integer string for human-
            /// readable formats, or as `$bytes_len` little-endian
            /// bytes for binary formats.
            #[inline]
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                if s.is_human_readable() {
                    #[cfg(feature = "alloc")]
                    {
                        s.serialize_str(&self.0.to_string())
                    }
                    #[cfg(not(feature = "alloc"))]
                    {
                        let _ = s;
                        Err(serde::ser::Error::custom(
                            "decimal-scaled: human-readable serialisation requires `alloc`",
                        ))
                    }
                } else {
                    let mut bytes = [0u8; $bytes_len];
                    let limbs = self.0.limbs_le();
                    // limbs is [u64; $bytes_len / 8]; write 8 bytes
                    // little-endian per limb. On LE targets this
                    // produces the same byte sequence as the historic
                    // [u128; $bytes_len / 16] path.
                    for (i, limb) in limbs.iter().enumerate() {
                        bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_le_bytes());
                    }
                    s.serialize_bytes(&bytes)
                }
            }
        }

        impl<'de, const SCALE: u32> Deserialize<'de> for $crate::types::widths::$Type<SCALE> {
            #[inline]
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                struct V<const S: u32>;
                impl<'de, const S: u32> Visitor<'de> for V<S> {
                    type Value = $crate::types::widths::$Type<S>;
                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str(concat!(
                            "a base-10 integer string or ",
                            stringify!($bytes_len),
                            " little-endian bytes for ",
                            stringify!($Type),
                        ))
                    }
                    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                        let parsed = <$Storage>::from_str_radix(v, 10).map_err(|_| {
                            serde::de::Error::custom(concat!(
                                stringify!($Type),
                                ": invalid base-10 integer string",
                            ))
                        })?;
                        Ok(<$crate::types::widths::$Type<S>>::from_bits(parsed))
                    }
                    fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
                        self.visit_str(v)
                    }
                    #[cfg(feature = "alloc")]
                    fn visit_string<E: serde::de::Error>(self, v: alloc::string::String) -> Result<Self::Value, E> {
                        self.visit_str(&v)
                    }
                    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                        if v.len() != $bytes_len {
                            return Err(serde::de::Error::invalid_length($bytes_len, &self));
                        }
                        // 8 bytes per u64 limb; bytes_len/8 limbs total.
                        let mut limbs = [0u64; $bytes_len / 8];
                        for (i, limb) in limbs.iter_mut().enumerate() {
                            let mut buf = [0u8; 8];
                            buf.copy_from_slice(&v[i * 8..(i + 1) * 8]);
                            *limb = u64::from_le_bytes(buf);
                        }
                        Ok(<$crate::types::widths::$Type<S>>::from_bits(<$Storage>::from_limbs_le(limbs)))
                    }
                    fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                        self.visit_bytes(v)
                    }
                }
                if d.is_human_readable() {
                    d.deserialize_str(V::<SCALE>)
                } else {
                    d.deserialize_bytes(V::<SCALE>)
                }
            }
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
decl_wide_serde!(D57, crate::wide_int::Int192, 24);
#[cfg(any(feature = "d76", feature = "wide"))]
decl_wide_serde!(D76, crate::wide_int::Int256, 32);
#[cfg(any(feature = "d115", feature = "wide"))]
decl_wide_serde!(D115, crate::wide_int::Int384, 48);
#[cfg(any(feature = "d153", feature = "wide"))]
decl_wide_serde!(D153, crate::wide_int::Int512, 64);
#[cfg(any(feature = "d230", feature = "wide"))]
decl_wide_serde!(D230, crate::wide_int::Int768, 96);
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_wide_serde!(D307, crate::wide_int::Int1024, 128);
#[cfg(any(feature = "d462", feature = "x-wide"))]
decl_wide_serde!(D462, crate::wide_int::Int1536, 192);
#[cfg(any(feature = "d616", feature = "x-wide"))]
decl_wide_serde!(D616, crate::wide_int::Int2048, 256);
#[cfg(any(feature = "d924", feature = "xx-wide"))]
decl_wide_serde!(D924, crate::wide_int::Int3072, 384);
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
decl_wide_serde!(D1232, crate::wide_int::Int4096, 512);

#[cfg(all(test, feature = "wide"))]
mod wide_serde_tests {
    use crate::D76;

    #[test]
    fn d76_human_readable_round_trip() {
        let v = D76::<12>::from_int(1_234_567_i128);
        let json = serde_json::to_string(&v).unwrap();
        let back: D76<12> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn d76_negative_human_readable_round_trip() {
        let v = -D76::<12>::from_int(987_654_321_i128);
        let json = serde_json::to_string(&v).unwrap();
        let back: D76<12> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn d76_binary_round_trip() {
        // postcard is a binary, non-self-describing format.
        let v = D76::<12>::from_int(42_i128);
        let bytes = postcard::to_allocvec(&v).unwrap();
        let back: D76<12> = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(back, v);
    }
}
