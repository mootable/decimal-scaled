//! Serde wire-format surface (visitor paths + full-width round-trips).
//! Migrated from `tests/serde_surface.rs`.

#[cfg(all(feature = "alloc", feature = "serde"))]
mod from_serde_surface {
    //! Coverage suite for `serde_helpers.rs` — D38 visitor's native integer
    //! paths (visit_i8/u8/i16/u16/i32/u32/i128), visit_string /
    //! visit_byte_buf (alloc-gated), visit_borrowed_*, and wide-tier serde.
    //!
    //! Each visitor method is invoked directly via the public
    //! `DecimalVisitor`. JSON / postcard round-trips are not used here —
    //! the macro-emitted visitor bodies are what we need to cover.

    use core::marker::PhantomData;
    use decimal_scaled::serde_helpers::decimal_serde::DecimalVisitor;
    #[cfg(any(feature = "wide", feature = "x-wide"))]
    use decimal_scaled::D38;
    use decimal_scaled::D38s12;
    use serde::de::Visitor;
    use serde::de::value::Error as DeError;

    // ─── D38 visitor: every native integer width ───────────────────────────

    #[test]
    fn visit_i8() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_i8::<DeError>(7)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 7);
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_i8::<DeError>(-7)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), -7);
    }

    #[test]
    fn visit_i16() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_i16::<DeError>(7)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 7);
    }

    #[test]
    fn visit_i32() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_i32::<DeError>(7)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 7);
    }

    #[test]
    fn visit_i128() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_i128::<DeError>(123_456_789_012_345_678_901_234)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 123_456_789_012_345_678_901_234);
    }

    #[test]
    fn visit_u8_u16_u32() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_u8::<DeError>(7)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 7);
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_u16::<DeError>(7)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 7);
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_u32::<DeError>(7)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 7);
    }

    #[test]
    fn visit_u128_in_range() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_u128::<DeError>(123_456)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 123_456);
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
        assert_eq!(i128::from(v.to_bits()), 12345);
    }

    #[test]
    fn visit_string_alloc() {
        let s: String = "12345".into();
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_string::<DeError>(s)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 12345);
    }

    #[test]
    fn visit_bytes_valid_and_error() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_bytes::<DeError>(&123_i128.to_le_bytes())
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 123);
        // Wrong length
        let r: Result<D38s12, _> = DecimalVisitor::<12>(PhantomData).visit_bytes::<DeError>(&[0u8; 15]);
        assert!(r.is_err());
    }

    #[test]
    fn visit_borrowed_bytes() {
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_borrowed_bytes::<DeError>(&123_i128.to_le_bytes())
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 123);
    }

    #[test]
    fn visit_byte_buf_alloc() {
        let buf: Vec<u8> = 123_i128.to_le_bytes().to_vec();
        let v = DecimalVisitor::<12>(PhantomData)
            .visit_byte_buf::<DeError>(buf)
            .unwrap();
        assert_eq!(i128::from(v.to_bits()), 123);
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

        let v: D76<6> = D38::<6>::try_from(42).unwrap().into();
        let s = serde_json::to_string(&v).unwrap();
        let back: D76<6> = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_serde_str_error() {
        use decimal_scaled::D76;

        let r: Result<D76<6>, _> = serde_json::from_str("\"xyz\"");
        assert!(r.is_err());
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d153_d307_serde_json_round_trip() {
        use decimal_scaled::{D76, D153, D307};

        let v: D153<6> = D38::<6>::try_from(42).unwrap().into();
        let s = serde_json::to_string(&v).unwrap();
        let back: D153<6> = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);

        let lift: D76<6> = D38::<6>::try_from(42).unwrap().into();
        let v: D307<6> = lift.into();
        let s = serde_json::to_string(&v).unwrap();
        let back: D307<6> = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);
    }

    // ─── Full-surface round-trip across every width ────────────────────────
    //
    // The 0.5.0 storage rewrite moved every decimal type onto `Int<N>` and
    // re-pointed `to_bits`/`from_bits` at `Int<N>` rather than `i128`. These
    // tests pin that the on-disk serde format still round-trips bit-exactly
    // for both the human-readable (decimal-string) and binary (LE limb-byte)
    // wire forms, across signs, zero, `MIN`/`MAX`, and a high-scale value —
    // for D38 and every wide tier. They also confirm the format is
    // internally consistent: serialize → deserialize → serialize is stable.

    use decimal_scaled::Int;

    /// Asserts JSON and postcard each round-trip `$value` bit-exactly, and
    /// that re-serialising the deserialised value reproduces the original
    /// wire bytes (format stability).
    fn assert_serde_stable<T>(value: T)
    where
        T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + core::fmt::Debug + Copy,
    {
        // Human-readable (JSON): serialize → deserialize → serialize.
        let json1 = serde_json::to_string(&value).unwrap();
        let back_json: T = serde_json::from_str(&json1).unwrap();
        assert_eq!(back_json, value, "JSON round-trip changed the value");
        let json2 = serde_json::to_string(&back_json).unwrap();
        assert_eq!(json1, json2, "JSON wire form is not stable on re-serialise");

        // Binary (postcard): serialize → deserialize → serialize.
        let bin1 = postcard::to_allocvec(&value).unwrap();
        let back_bin: T = postcard::from_bytes(&bin1).unwrap();
        assert_eq!(back_bin, value, "postcard round-trip changed the value");
        let bin2 = postcard::to_allocvec(&back_bin).unwrap();
        assert_eq!(bin1, bin2, "postcard wire form is not stable on re-serialise");
    }

    /// Generates a round-trip test for one decimal width `$ty` whose storage
    /// is `Int<$n>`. Covers zero, a small positive/negative pair, a
    /// high-scale fractional value, and `MIN`/`MAX` via raw `from_bits`.
    macro_rules! width_round_trip_test {
        ($name:ident, $ty:ident, $n:literal, $scale:literal) => {
            #[test]
            fn $name() {
                use decimal_scaled::$ty;

                let bits = |n: i128| Int::<$n>::try_from(n).unwrap();
                // Zero.
                assert_serde_stable($ty::<$scale>::ZERO);
                // Small positive and negative logical values.
                assert_serde_stable($ty::<$scale>::from_bits(bits(7)));
                assert_serde_stable($ty::<$scale>::from_bits(bits(-7)));
                // A high-scale fractional value: 1 ULP and a few scaled units.
                assert_serde_stable($ty::<$scale>::from_bits(bits(1)));
                assert_serde_stable($ty::<$scale>::from_bits(bits(
                    -123_456_789_012_345_678_901_234_567_890_i128,
                )));
                // Full-width extremes — the classic two's-complement asymmetry.
                assert_serde_stable($ty::<$scale>::MAX);
                assert_serde_stable($ty::<$scale>::MIN);
            }
        };
    }

    // D38 (Int<2>) is always available under the serde feature.
    width_round_trip_test!(d38_full_surface_round_trip, D38, 2, 12);

    #[cfg(feature = "wide")]
    mod wide_full_surface {
        use super::{assert_serde_stable, Int};

        width_round_trip_test!(d57_full_surface_round_trip, D57, 3, 18);
        width_round_trip_test!(d76_full_surface_round_trip, D76, 4, 24);
        width_round_trip_test!(d115_full_surface_round_trip, D115, 6, 32);
        width_round_trip_test!(d153_full_surface_round_trip, D153, 8, 48);
        width_round_trip_test!(d230_full_surface_round_trip, D230, 12, 64);
    }

    #[cfg(feature = "x-wide")]
    mod x_wide_full_surface {
        use super::{assert_serde_stable, Int};

        width_round_trip_test!(d307_full_surface_round_trip, D307, 16, 96);
        width_round_trip_test!(d462_full_surface_round_trip, D462, 24, 153);
        width_round_trip_test!(d616_full_surface_round_trip, D616, 32, 200);
    }

    #[cfg(feature = "xx-wide")]
    mod xx_wide_full_surface {
        use super::{assert_serde_stable, Int};

        width_round_trip_test!(d924_full_surface_round_trip, D924, 48, 461);
        width_round_trip_test!(d1232_full_surface_round_trip, D1232, 64, 615);
    }
}
