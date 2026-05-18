//! Narrow-tier sin / cos / tan via widen → D38 → narrow.

use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

macro_rules! delegate_via_d38 {
    ($d9_name:ident, $d18_name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        pub(crate) fn $d9_name<const SCALE: u32>(v: D9<SCALE>, mode: RoundingMode) -> D9<SCALE> {
            let widened: D38<SCALE> = v.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(widened.0, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
        #[inline]
        #[must_use]
        pub(crate) fn $d18_name<const SCALE: u32>(v: D18<SCALE>, mode: RoundingMode) -> D18<SCALE> {
            let widened: D38<SCALE> = v.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(widened.0, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
    };
}

macro_rules! delegate_via_d38_with {
    ($d9_name:ident, $d18_name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        pub(crate) fn $d9_name<const SCALE: u32>(
            v: D9<SCALE>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> D9<SCALE> {
            let widened: D38<SCALE> = v.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(widened.0, working_digits, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
        #[inline]
        #[must_use]
        pub(crate) fn $d18_name<const SCALE: u32>(
            v: D18<SCALE>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> D18<SCALE> {
            let widened: D38<SCALE> = v.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(widened.0, working_digits, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
    };
}

// ── atan2: two-argument variants ──────────────────────────────────
//
// Unlike the unary trig kernels, `atan2` takes both `y` and `x` and
// widens both to D38 before delegating.

macro_rules! delegate_via_d38_binary {
    ($d9_name:ident, $d18_name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        pub(crate) fn $d9_name<const SCALE: u32>(
            y: D9<SCALE>,
            x: D9<SCALE>,
            mode: RoundingMode,
        ) -> D9<SCALE> {
            let y_wide: D38<SCALE> = y.into();
            let x_wide: D38<SCALE> = x.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(y_wide.0, x_wide.0, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
        #[inline]
        #[must_use]
        pub(crate) fn $d18_name<const SCALE: u32>(
            y: D18<SCALE>,
            x: D18<SCALE>,
            mode: RoundingMode,
        ) -> D18<SCALE> {
            let y_wide: D38<SCALE> = y.into();
            let x_wide: D38<SCALE> = x.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(y_wide.0, x_wide.0, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
    };
}

macro_rules! delegate_via_d38_binary_with {
    ($d9_name:ident, $d18_name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        pub(crate) fn $d9_name<const SCALE: u32>(
            y: D9<SCALE>,
            x: D9<SCALE>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> D9<SCALE> {
            let y_wide: D38<SCALE> = y.into();
            let x_wide: D38<SCALE> = x.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(y_wide.0, x_wide.0, working_digits, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
        #[inline]
        #[must_use]
        pub(crate) fn $d18_name<const SCALE: u32>(
            y: D18<SCALE>,
            x: D18<SCALE>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> D18<SCALE> {
            let y_wide: D38<SCALE> = y.into();
            let x_wide: D38<SCALE> = x.into();
            let raw = super::fixed_d38::$kernel::<SCALE>(y_wide.0, x_wide.0, working_digits, mode);
            D38::<SCALE>(raw).try_into().expect($err)
        }
    };
}

delegate_via_d38!(sin_strict_d9, sin_strict_d18, sin_strict, "sin_strict: result out of range");
delegate_via_d38_with!(sin_with_d9, sin_with_d18, sin_with, "sin_with: result out of range");
delegate_via_d38!(cos_strict_d9, cos_strict_d18, cos_strict, "cos_strict: result out of range");
delegate_via_d38_with!(cos_with_d9, cos_with_d18, cos_with, "cos_with: result out of range");
delegate_via_d38!(tan_strict_d9, tan_strict_d18, tan_strict, "tan_strict: result out of range");
delegate_via_d38_with!(tan_with_d9, tan_with_d18, tan_with, "tan_with: result out of range");
delegate_via_d38!(atan_strict_d9, atan_strict_d18, atan_strict, "atan_strict: result out of range");
delegate_via_d38_with!(atan_with_d9, atan_with_d18, atan_with, "atan_with: result out of range");
delegate_via_d38!(asin_strict_d9, asin_strict_d18, asin_strict, "asin_strict: result out of range");
delegate_via_d38_with!(asin_with_d9, asin_with_d18, asin_with, "asin_with: result out of range");
delegate_via_d38!(acos_strict_d9, acos_strict_d18, acos_strict, "acos_strict: result out of range");
delegate_via_d38_with!(acos_with_d9, acos_with_d18, acos_with, "acos_with: result out of range");
delegate_via_d38_binary!(atan2_strict_d9, atan2_strict_d18, atan2_strict, "atan2_strict: result out of range");
delegate_via_d38_binary_with!(atan2_with_d9, atan2_with_d18, atan2_with, "atan2_with: result out of range");
