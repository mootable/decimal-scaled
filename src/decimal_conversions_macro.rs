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
