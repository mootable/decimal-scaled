// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Bench-only alias wrappers for the per-type override policy.
//!
//! The per-type override policy: every precision-bearing method has a
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
//! `ln` / `exp` / etc. is the only public surface.

#![cfg(feature = "bench-alt")]

// D38 — every strict transcendental is an `override` (hand-tuned
// per `algos/support/fixed.rs`). The `_default` alias would be the
// macro-generated `decl_wide_transcendental!` path; that path is
// not invoked for D38 because it's more than 1.5× slower than the
// override. With `bench-alt` a separate invocation could add the
// `_default` aliases — recorded as a follow-up so the macro can
// emit suffix-renamed methods.
impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    #[inline]
    pub fn ln_override(self) -> Self {
        self.ln()
    }
    #[inline]
    pub fn log_override(self, base: Self) -> Self {
        self.log(base)
    }
    #[inline]
    pub fn log2_override(self) -> Self {
        self.log2()
    }
    #[inline]
    pub fn log10_override(self) -> Self {
        self.log10()
    }
    #[inline]
    pub fn exp_override(self) -> Self {
        self.exp()
    }
    #[inline]
    pub fn exp2_override(self) -> Self {
        self.exp2()
    }
    #[inline]
    pub fn sin_override(self) -> Self {
        self.sin()
    }
    #[inline]
    pub fn cos_override(self) -> Self {
        self.cos()
    }
    #[inline]
    pub fn tan_override(self) -> Self {
        self.tan()
    }
    #[inline]
    pub fn asin_override(self) -> Self {
        self.asin()
    }
    #[inline]
    pub fn acos_override(self) -> Self {
        self.acos()
    }
    #[inline]
    pub fn atan_override(self) -> Self {
        self.atan()
    }
    #[inline]
    pub fn atan2_override(self, other: Self) -> Self {
        self.atan2(other)
    }
    #[inline]
    pub fn sinh_override(self) -> Self {
        self.sinh()
    }
    #[inline]
    pub fn cosh_override(self) -> Self {
        self.cosh()
    }
    #[inline]
    pub fn tanh_override(self) -> Self {
        self.tanh()
    }
    #[inline]
    pub fn asinh_override(self) -> Self {
        self.asinh()
    }
    #[inline]
    pub fn acosh_override(self) -> Self {
        self.acosh()
    }
    #[inline]
    pub fn atanh_override(self) -> Self {
        self.atanh()
    }
    #[inline]
    pub fn to_degrees_override(self) -> Self {
        self.to_degrees()
    }
    #[inline]
    pub fn to_radians_override(self) -> Self {
        self.to_radians()
    }
    #[inline]
    pub fn powf_override(self, exp: Self) -> Self {
        self.powf(exp)
    }
    #[inline]
    pub fn sqrt_override(self) -> Self {
        self.sqrt()
    }
    #[inline]
    pub fn cbrt_override(self) -> Self {
        self.cbrt()
    }
}

// Wide tiers — every strict transcendental is a `default` (macro-
// emitted). The `_default` aliases forward to the canonical.
macro_rules! decl_default_aliases {
    ($Type:ident) => {
        impl<const SCALE: u32> crate::types::widths::$Type<SCALE> {
            #[inline]
            pub fn ln_default(self) -> Self {
                self.ln()
            }
            #[inline]
            pub fn log_default(self, base: Self) -> Self {
                self.log(base)
            }
            #[inline]
            pub fn log2_default(self) -> Self {
                self.log2()
            }
            #[inline]
            pub fn log10_default(self) -> Self {
                self.log10()
            }
            #[inline]
            pub fn exp_default(self) -> Self {
                self.exp()
            }
            #[inline]
            pub fn exp2_default(self) -> Self {
                self.exp2()
            }
            #[inline]
            pub fn sin_default(self) -> Self {
                self.sin()
            }
            #[inline]
            pub fn cos_default(self) -> Self {
                self.cos()
            }
            #[inline]
            pub fn tan_default(self) -> Self {
                self.tan()
            }
            #[inline]
            pub fn asin_default(self) -> Self {
                self.asin()
            }
            #[inline]
            pub fn acos_default(self) -> Self {
                self.acos()
            }
            #[inline]
            pub fn atan_default(self) -> Self {
                self.atan()
            }
            #[inline]
            pub fn atan2_default(self, other: Self) -> Self {
                self.atan2(other)
            }
            #[inline]
            pub fn sinh_default(self) -> Self {
                self.sinh()
            }
            #[inline]
            pub fn cosh_default(self) -> Self {
                self.cosh()
            }
            #[inline]
            pub fn tanh_default(self) -> Self {
                self.tanh()
            }
            #[inline]
            pub fn asinh_default(self) -> Self {
                self.asinh()
            }
            #[inline]
            pub fn acosh_default(self) -> Self {
                self.acosh()
            }
            #[inline]
            pub fn atanh_default(self) -> Self {
                self.atanh()
            }
            #[inline]
            pub fn to_degrees_default(self) -> Self {
                self.to_degrees()
            }
            #[inline]
            pub fn to_radians_default(self) -> Self {
                self.to_radians()
            }
            #[inline]
            pub fn powf_default(self, exp: Self) -> Self {
                self.powf(exp)
            }
            #[inline]
            pub fn sqrt_default(self) -> Self {
                self.sqrt()
            }
            #[inline]
            pub fn cbrt_default(self) -> Self {
                self.cbrt()
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
