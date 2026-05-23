// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer exponentiation algorithm family.
//!
//! - [`pow_square_and_multiply`] — binary exponentiation by squaring over
//!   the const [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] and
//!   [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] kernels. The per-`N`
//!   choice lives in [`crate::int::policy::pow`].
//!
//! [`pow_square_and_multiply`]: crate::int::algos::pow::pow_square_and_multiply::pow_square_and_multiply

pub(crate) mod pow_square_and_multiply;
