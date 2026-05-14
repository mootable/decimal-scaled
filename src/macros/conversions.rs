//! Macro-generated `From` impls for primitive integer inputs.
//!
//! Each impl multiplies the input by `multiplier()` (= `10^SCALE`) and
//! stores the result in the width's native integer storage. Overflow
//! follows Rust's default integer arithmetic: debug-mode panic,
//! release-mode wrap. Users with overflow risk should reach for the
//! eventual `TryFrom` variants (Phase 3D pending).

/// Generates `From<$Src> for $Type<SCALE>` that scales the value by
/// `10^SCALE` and stores it in `$Storage`. The cast `value as $Storage`
/// happens first when the source is narrower than the storage, which
/// is the lossless case; the subsequent multiply is the overflow risk.
macro_rules! decl_from_primitive {
    ($Type:ident, $Storage:ty, $Src:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src> for $Type<SCALE> {
            /// Constructs from an integer by scaling to `value * 10^SCALE`.
            /// Overflows follow Rust's default integer arithmetic semantics
            /// (debug-mode panic, release-mode wrap).
            #[inline]
            fn from(value: $Src) -> Self {
                Self((value as $Storage) * Self::multiplier())
            }
        }
    };
}

pub(crate) use decl_from_primitive;

/// Generates `From<$Src<SCALE>> for $Dest<SCALE>` for a lossless
/// widening conversion (e.g. D32 -> D64, D64 -> D128). `$SrcStorage`
/// must widen losslessly to `$DestStorage` via an `as` cast.
macro_rules! decl_cross_width_widening {
    ($Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src<SCALE>> for $Dest<SCALE> {
            /// Widens a narrower decimal type to this wider one. The
            /// scale is unchanged and the storage is widened by an
            /// `as` cast (lossless because the source storage type is
            /// strictly narrower than the destination).
            #[inline]
            fn from(value: $Src<SCALE>) -> Self {
                Self(value.to_bits() as $DestStorage)
            }
        }
    };
}

pub(crate) use decl_cross_width_widening;

/// Generates `TryFrom<$Src<SCALE>> for $Dest<SCALE>` for a fallible
/// narrowing conversion. Returns
/// `Err(ConvertError::OutOfRange)` when the source value exceeds
/// the destination's representable range; otherwise returns the
/// narrowed value bit-for-bit (same logical decimal value).
macro_rules! decl_cross_width_narrowing {
    ($Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<$Src<SCALE>> for $Dest<SCALE> {
            type Error = $crate::conversions::ConvertError;
            /// Attempts to narrow a wider decimal type to this narrower
            /// one. Fails with `OutOfRange` when the source value
            /// exceeds the destination's `MIN..=MAX`. The scale is
            /// unchanged.
            #[inline]
            fn try_from(value: $Src<SCALE>) -> ::core::result::Result<Self, Self::Error> {
                let bits = value.to_bits();
                if bits > (<$DestStorage>::MAX as $SrcStorage)
                    || bits < (<$DestStorage>::MIN as $SrcStorage)
                {
                    return ::core::result::Result::Err(
                        $crate::conversions::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self(bits as $DestStorage))
            }
        }
    };
}

pub(crate) use decl_cross_width_narrowing;
