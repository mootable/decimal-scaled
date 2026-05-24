// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Per-layer working-scratch traits for the width-agnostic kernels.
//!
//! Several kernels need stack scratch whose size is a multiple of the limb
//! count `N` (a sqrt/hypot radicand spans `2N` limbs, a cbrt radicand `4N`,
//! the MG `÷10^w` magnitude `⌈N/2⌉` u128 limbs, the Knuth normalised
//! dividend/divisor up to `4N`). Stable Rust cannot name `[u64; 2N]` from a
//! generic `N`, so each buffer is carried as an **associated type** whose
//! size lives in the `impl` (where `N` is concrete) and never appears in the
//! kernel signature. A kernel bounds on its layer's trait and reads the
//! buffer it needs as `&mut [u64]`; the bound is discharged for free at the
//! concrete `N` every type method dispatches at, so it does not cascade.
//!
//! The surface is split by LAYER (it consolidates the former single-purpose
//! `WorkScratch` + `MgScratch`):
//! - [`WorkingDecimal`] — buffers the decimal algos (`src/algos/…`) need:
//!   the `2N`/`4N` root radicands and the `⌈N/2⌉` u128 MG magnitude.
//! - [`WorkingInt`] — buffers the int algos (`src/int/algos/…`) need: the
//!   `2N`/`4N` root radicands and the Knuth divide scratch (`work_div`).
//!
//! Three build forms, identical sizes numerically — only *who pays for the
//! slack* differs:
//! - **default** — one blanket impl, build-max for every `N`.
//! - **`exact-scratch`** (stable) — one impl per concrete width, each a
//!   size literal.
//! - **`exact-scratch-nightly`** — one blanket impl sized per-`N` via
//!   const-expr under `generic_const_exprs`.

use crate::int::types::Int;

/// Decimal-layer working scratch: `2N`/`4N` root radicands and the
/// `⌈N/2⌉` u128 MG `÷10^w` magnitude buffer.
pub(crate) trait WorkingDecimal {
    /// 2N-family radicand scratch (`sqrt`/`hypot`).
    type Buf2: AsMut<[u64]> + AsRef<[u64]>;
    /// 4N-family radicand scratch (`cbrt`).
    type Buf4: AsMut<[u64]> + AsRef<[u64]>;
    /// `⌈N/2⌉` u128 magnitude scratch (`exp_generic` ÷10^w; `== U128_LIMBS`).
    type BufU128: AsMut<[u128]> + AsRef<[u128]>;
    /// Freshly zeroed 2N radicand buffer.
    fn work2() -> Self::Buf2;
    /// Freshly zeroed 4N radicand buffer.
    fn work4() -> Self::Buf4;
    /// Freshly zeroed u128 magnitude buffer.
    fn work_u128() -> Self::BufU128;
}

/// Int-layer working scratch: `2N`/`4N` root radicands (`isqrt`/`icbrt`/
/// `sum_sq`/`hypot`) and the Knuth normalised dividend/divisor scratch.
///
/// `work_div` sizing (in u64 limbs): the divide engine sees operands up to
/// `4N` when keyed at a STORAGE width `N ∈ {1..64}` (the cbrt `4N` radicand
/// divide originates there), but only `≤ N` when keyed at a wide WORK width
/// `N ∈ {96,128,192,256}` (the transcendental reciprocal divides at
/// `Wexp`). So it is `4N + ⌈N/2⌉` at storage widths and `N + ⌈N/2⌉` at work
/// widths — the absolute max (`Int<256>` → ~288) is the old build-max
/// constant, now exact per width.
pub(crate) trait WorkingInt {
    /// 2N-family radicand scratch (`isqrt`/`hypot`/`sum_sq`).
    type Buf2: AsMut<[u64]> + AsRef<[u64]>;
    /// 4N-family radicand scratch (`icbrt`).
    type Buf4: AsMut<[u64]> + AsRef<[u64]>;
    /// Knuth `u`/`v` divide scratch (one buffer; the engine takes two).
    type DivBuf: AsMut<[u64]> + AsRef<[u64]>;
    /// Freshly zeroed 2N radicand buffer.
    fn work2() -> Self::Buf2;
    /// Freshly zeroed 4N radicand buffer.
    fn work4() -> Self::Buf4;
    /// Freshly zeroed divide-scratch buffer.
    fn work_div() -> Self::DivBuf;
}

// ── default: one blanket impl, build-max for every N ──────────────────
#[cfg(not(feature = "exact-scratch"))]
mod imp {
    use super::{Int, WorkingDecimal, WorkingInt};
    use crate::int::algos::support::limbs::{work_scratch, MAX_WORK_N};

    // `Wexp` runs up to 8× the storage width, so its `U128_LIMBS ≤
    // 4·MAX_WORK_N`; `work_div` likewise tops out at `4·MAX_WORK_N + slack`
    // (the cbrt 4N radicand at the widest tier) — the old ~288 constant.
    impl<const N: usize> WorkingDecimal for Int<N> {
        type Buf2 = [u64; work_scratch(2)];
        type Buf4 = [u64; work_scratch(4)];
        type BufU128 = [u128; 4 * MAX_WORK_N];
        #[inline]
        fn work2() -> Self::Buf2 {
            [0u64; work_scratch(2)]
        }
        #[inline]
        fn work4() -> Self::Buf4 {
            [0u64; work_scratch(4)]
        }
        #[inline]
        fn work_u128() -> Self::BufU128 {
            [0u128; 4 * MAX_WORK_N]
        }
    }

    impl<const N: usize> WorkingInt for Int<N> {
        type Buf2 = [u64; work_scratch(2)];
        type Buf4 = [u64; work_scratch(4)];
        type DivBuf = [u64; 4 * MAX_WORK_N + (MAX_WORK_N + 1) / 2];
        #[inline]
        fn work2() -> Self::Buf2 {
            [0u64; work_scratch(2)]
        }
        #[inline]
        fn work4() -> Self::Buf4 {
            [0u64; work_scratch(4)]
        }
        #[inline]
        fn work_div() -> Self::DivBuf {
            [0u64; 4 * MAX_WORK_N + (MAX_WORK_N + 1) / 2]
        }
    }
}

// ── exact-scratch (stable): one impl per concrete width ───────────────
#[cfg(all(feature = "exact-scratch", not(feature = "exact-scratch-nightly")))]
mod imp {
    use super::{Int, WorkingDecimal, WorkingInt};

    /// `impl WorkingDecimal for Int<$n>` per concrete width: radicands
    /// `mult·n + ⌈n/2⌉`, MG magnitude `⌈n/2⌉` u128. Covers storage widths
    /// AND the transcendental work widths (96..256, where `exp_generic`
    /// runs its ÷10^w).
    macro_rules! exact_decimal {
        ($($n:literal),+ $(,)?) => { $(
            impl WorkingDecimal for Int<$n> {
                type Buf2 = [u64; 2 * $n + ($n + 1) / 2];
                type Buf4 = [u64; 4 * $n + ($n + 1) / 2];
                type BufU128 = [u128; ($n + 1) / 2];
                #[inline]
                fn work2() -> Self::Buf2 {
                    [0u64; 2 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn work4() -> Self::Buf4 {
                    [0u64; 4 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn work_u128() -> Self::BufU128 {
                    [0u128; ($n + 1) / 2]
                }
            }
        )+ };
    }
    exact_decimal!(1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256);

    /// `impl WorkingInt for Int<$n>` per concrete width: radicands
    /// `mult·n + ⌈n/2⌉`, divide scratch `$div_mult·n + ⌈n/2⌉`. STORAGE
    /// widths use `$div_mult = 4` (cbrt 4N radicand divide); WORK widths
    /// use `$div_mult = 1` (only the transcendental reciprocal divides
    /// there, operands ≤ N) — so `Int<256>` is ~288, not 1024.
    macro_rules! exact_int {
        ($div_mult:literal; $($n:literal),+ $(,)?) => { $(
            impl WorkingInt for Int<$n> {
                type Buf2 = [u64; 2 * $n + ($n + 1) / 2];
                type Buf4 = [u64; 4 * $n + ($n + 1) / 2];
                type DivBuf = [u64; $div_mult * $n + ($n + 1) / 2];
                #[inline]
                fn work2() -> Self::Buf2 {
                    [0u64; 2 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn work4() -> Self::Buf4 {
                    [0u64; 4 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn work_div() -> Self::DivBuf {
                    [0u64; $div_mult * $n + ($n + 1) / 2]
                }
            }
        )+ };
    }
    // Storage widths (operands up to 4N via the cbrt radicand divide):
    exact_int!(4; 1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64);
    // Wide work widths (only the transcendental ÷ at Wexp, operands ≤ N):
    exact_int!(1; 96, 128, 192, 256);
}

// ── exact-scratch-nightly: one blanket impl, exact per-N via const-expr ─
#[cfg(feature = "exact-scratch-nightly")]
mod imp {
    use super::{Int, WorkingDecimal, WorkingInt};
    use crate::int::algos::support::limbs::work_scratch_n;

    /// Divide-scratch limbs per `N`: `4N` at storage widths, `1N` at the
    /// wide work widths, both `+ ⌈N/2⌉`.
    const fn div_scratch_n(n: usize) -> usize {
        if n <= 64 {
            4 * n + (n + 1) / 2
        } else {
            n + (n + 1) / 2
        }
    }

    impl<const N: usize> WorkingDecimal for Int<N>
    where
        [(); work_scratch_n(2, N)]:,
        [(); work_scratch_n(4, N)]:,
        [(); (N + 1) / 2]:,
    {
        type Buf2 = [u64; work_scratch_n(2, N)];
        type Buf4 = [u64; work_scratch_n(4, N)];
        type BufU128 = [u128; (N + 1) / 2];
        #[inline]
        fn work2() -> Self::Buf2 {
            [0u64; work_scratch_n(2, N)]
        }
        #[inline]
        fn work4() -> Self::Buf4 {
            [0u64; work_scratch_n(4, N)]
        }
        #[inline]
        fn work_u128() -> Self::BufU128 {
            [0u128; (N + 1) / 2]
        }
    }

    impl<const N: usize> WorkingInt for Int<N>
    where
        [(); work_scratch_n(2, N)]:,
        [(); work_scratch_n(4, N)]:,
        [(); div_scratch_n(N)]:,
    {
        type Buf2 = [u64; work_scratch_n(2, N)];
        type Buf4 = [u64; work_scratch_n(4, N)];
        type DivBuf = [u64; div_scratch_n(N)];
        #[inline]
        fn work2() -> Self::Buf2 {
            [0u64; work_scratch_n(2, N)]
        }
        #[inline]
        fn work4() -> Self::Buf4 {
            [0u64; work_scratch_n(4, N)]
        }
        #[inline]
        fn work_div() -> Self::DivBuf {
            [0u64; div_scratch_n(N)]
        }
    }
}
