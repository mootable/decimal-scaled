//! Reusable width-matched integer algorithms.
//!
//! The integer layer's algorithm bucket: the width-matched routines that
//! `Int<N>` / `Uint<N>` compose on.
//!
//! - [`limbs`] — generic little-endian `u64` limb arithmetic
//!   (is_zero/cmp/add_assign/sub_assign/shift/bit_len, the schoolbook and
//!   Karatsuba multiply kernels, the truncated low-`N` product / squaring,
//!   signed compare).
//! - [`div`] — the pure division engines (`div_rem`, `div_knuth`,
//!   `div_burnikel_ziegler_with_knuth`, the `Mg2By1` / `Mg3By2`
//!   Möller–Granlund reciprocal engines) plus the const-`N` fast-arm
//!   wrappers `div_rem_mag_fixed` / `isqrt_mag_fixed`. The divisor-shape
//!   *choice* between the engines lives in [`crate::int::policy::div`].
//! - [`roots`] — the Newton integer square root `isqrt_newton`.

pub(crate) mod div;
pub(crate) mod limbs;
pub(crate) mod roots;
