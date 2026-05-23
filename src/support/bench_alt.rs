//! Bench-only alias wrappers for the per-type override policy.
//!
//! Per `OVERRIDE_POLICY.md`: every precision-bearing method has a
//! canonical name (`f_strict` / `f_lossy`) plus, where applicable,
//! a `_default` and `_override` variant. The canonical name is the
//! chosen winner; the other variant exists too but is opt-in via
//! this feature so a single benchmark binary can compare both
//! without recompiling.
//!
//! With `feature = "bench-alt"` enabled, this module emits
//! `#[inline]` wrappers that forward each `_default` / `_override`
//! alias to its canonical implementation. The wrappers compile away
//! at link time — direct calls in the bench output.
//!
//! Without the feature, this module is not compiled and the aliases
//! don't exist. Production code never uses these aliases; canonical
//! `ln_strict` / `exp_strict` / etc. is the only public surface.

#![cfg(feature = "bench-alt")]

use crate::types::widths::D38;

// D38 — every strict transcendental is an `override` (hand-tuned
// per `algos/fixed_d38.rs`). The `_default` alias would be the
// macro-generated `decl_wide_transcendental!` path; that path is
// not invoked for D38 because it's more than 1.5× slower than the
// override. With `bench-alt` a separate invocation could add the
// `_default` aliases — recorded as a follow-up so the macro can
// emit suffix-renamed methods.
impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    #[inline]
    pub fn ln_strict_override(self) -> Self {
        self.ln_strict()
    }
    #[inline]
    pub fn log_strict_override(self, base: Self) -> Self {
        self.log_strict(base)
    }
    #[inline]
    pub fn log2_strict_override(self) -> Self {
        self.log2_strict()
    }
    #[inline]
    pub fn log10_strict_override(self) -> Self {
        self.log10_strict()
    }
    #[inline]
    pub fn exp_strict_override(self) -> Self {
        self.exp_strict()
    }
    #[inline]
    pub fn exp2_strict_override(self) -> Self {
        self.exp2_strict()
    }
    #[inline]
    pub fn sin_strict_override(self) -> Self {
        self.sin_strict()
    }
    #[inline]
    pub fn cos_strict_override(self) -> Self {
        self.cos_strict()
    }
    #[inline]
    pub fn tan_strict_override(self) -> Self {
        self.tan_strict()
    }
    #[inline]
    pub fn asin_strict_override(self) -> Self {
        self.asin_strict()
    }
    #[inline]
    pub fn acos_strict_override(self) -> Self {
        self.acos_strict()
    }
    #[inline]
    pub fn atan_strict_override(self) -> Self {
        self.atan_strict()
    }
    #[inline]
    pub fn atan2_strict_override(self, other: Self) -> Self {
        self.atan2_strict(other)
    }
    #[inline]
    pub fn sinh_strict_override(self) -> Self {
        self.sinh_strict()
    }
    #[inline]
    pub fn cosh_strict_override(self) -> Self {
        self.cosh_strict()
    }
    #[inline]
    pub fn tanh_strict_override(self) -> Self {
        self.tanh_strict()
    }
    #[inline]
    pub fn asinh_strict_override(self) -> Self {
        self.asinh_strict()
    }
    #[inline]
    pub fn acosh_strict_override(self) -> Self {
        self.acosh_strict()
    }
    #[inline]
    pub fn atanh_strict_override(self) -> Self {
        self.atanh_strict()
    }
    #[inline]
    pub fn to_degrees_strict_override(self) -> Self {
        self.to_degrees_strict()
    }
    #[inline]
    pub fn to_radians_strict_override(self) -> Self {
        self.to_radians_strict()
    }
    #[inline]
    pub fn powf_strict_override(self, exp: Self) -> Self {
        self.powf_strict(exp)
    }
    #[inline]
    pub fn sqrt_strict_override(self) -> Self {
        self.sqrt_strict()
    }
    #[inline]
    pub fn cbrt_strict_override(self) -> Self {
        self.cbrt_strict()
    }
}

// Wide tiers — every strict transcendental is a `default` (macro-
// emitted). The `_default` aliases forward to the canonical.
macro_rules! decl_default_aliases {
    ($Type:ident) => {
        impl<const SCALE: u32> crate::types::widths::$Type<SCALE> {
            #[inline]
            pub fn ln_strict_default(self) -> Self {
                self.ln_strict()
            }
            #[inline]
            pub fn log_strict_default(self, base: Self) -> Self {
                self.log_strict(base)
            }
            #[inline]
            pub fn log2_strict_default(self) -> Self {
                self.log2_strict()
            }
            #[inline]
            pub fn log10_strict_default(self) -> Self {
                self.log10_strict()
            }
            #[inline]
            pub fn exp_strict_default(self) -> Self {
                self.exp_strict()
            }
            #[inline]
            pub fn exp2_strict_default(self) -> Self {
                self.exp2_strict()
            }
            #[inline]
            pub fn sin_strict_default(self) -> Self {
                self.sin_strict()
            }
            #[inline]
            pub fn cos_strict_default(self) -> Self {
                self.cos_strict()
            }
            #[inline]
            pub fn tan_strict_default(self) -> Self {
                self.tan_strict()
            }
            #[inline]
            pub fn asin_strict_default(self) -> Self {
                self.asin_strict()
            }
            #[inline]
            pub fn acos_strict_default(self) -> Self {
                self.acos_strict()
            }
            #[inline]
            pub fn atan_strict_default(self) -> Self {
                self.atan_strict()
            }
            #[inline]
            pub fn atan2_strict_default(self, other: Self) -> Self {
                self.atan2_strict(other)
            }
            #[inline]
            pub fn sinh_strict_default(self) -> Self {
                self.sinh_strict()
            }
            #[inline]
            pub fn cosh_strict_default(self) -> Self {
                self.cosh_strict()
            }
            #[inline]
            pub fn tanh_strict_default(self) -> Self {
                self.tanh_strict()
            }
            #[inline]
            pub fn asinh_strict_default(self) -> Self {
                self.asinh_strict()
            }
            #[inline]
            pub fn acosh_strict_default(self) -> Self {
                self.acosh_strict()
            }
            #[inline]
            pub fn atanh_strict_default(self) -> Self {
                self.atanh_strict()
            }
            #[inline]
            pub fn to_degrees_strict_default(self) -> Self {
                self.to_degrees_strict()
            }
            #[inline]
            pub fn to_radians_strict_default(self) -> Self {
                self.to_radians_strict()
            }
            #[inline]
            pub fn powf_strict_default(self, exp: Self) -> Self {
                self.powf_strict(exp)
            }
            #[inline]
            pub fn sqrt_strict_default(self) -> Self {
                self.sqrt_strict()
            }
            #[inline]
            pub fn cbrt_strict_default(self) -> Self {
                self.cbrt_strict()
            }
        }
    };
}

#[cfg(any(feature = "d76", feature = "wide"))]
decl_default_aliases!(D76);
#[cfg(any(feature = "d153", feature = "wide"))]
decl_default_aliases!(D153);
#[cfg(any(feature = "d307", feature = "wide"))]
decl_default_aliases!(D307);
