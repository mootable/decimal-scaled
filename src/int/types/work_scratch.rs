// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Work-scratch surface for the width-agnostic decimal root kernels.
//!
//! `sqrt_newton` / `hypot_pythagoras` form a radicand spanning up to `2N` limbs,
//! `cbrt_newton` up to `4N`; they do that arithmetic in a fixed stack
//! buffer rather than threading a work *type* `Int<2N>`/`Int<4N>` (which
//! stable Rust cannot name from `N`). This trait supplies that buffer as an
//! **associated type**, so its size lives in the `impl` — where `N` is
//! concrete — and never appears as a generic const expression in the
//! kernels. The kernels bound only on `Int<N>: WorkScratch` and read
//! `Buf2`/`Buf4` as `&mut [u64]`; that bound is discharged for free at the
//! concrete `N` every type method dispatches at, so it does not cascade.
//!
//! Three build forms, selected by feature (size is identical numerically;
//! only *who pays for the slack* differs):
//!
//! - **default** — one blanket impl, build-max [`work_scratch(mult)`] for
//!   every `N` (the widest enabled tier's size, applied uniformly).
//! - **`exact-scratch`** (stable) — one impl per concrete storage width,
//!   each sized exactly `mult·N + ceil(N/2)`. No nightly: the size is a
//!   literal in each impl.
//! - **`exact-scratch-nightly`** — one blanket impl sized per-`N` via
//!   [`work_scratch_n`] under `generic_const_exprs`; the const-expr (and its
//!   `where [(); …]:` obligation) stays confined to this impl block.
//!
//! [`work_scratch(mult)`]: crate::int::algos::support::limbs::work_scratch
//! [`work_scratch_n`]: crate::int::algos::support::limbs::work_scratch_n

use crate::int::types::Int;

/// Stack scratch buffers for the 2N-family (`sqrt`/`hypot`) and 4N-family
/// (`cbrt`) root kernels. The associated types keep the buffer *size* in the
/// `impl`, out of the kernels' generics — see the module docs.
pub(crate) trait WorkScratch {
    /// 2N-family scratch: radicand ≤ `2N` limbs plus the carry margin.
    type Buf2: AsMut<[u64]> + AsRef<[u64]>;
    /// 4N-family scratch: radicand ≤ `4N` limbs plus the carry margin.
    type Buf4: AsMut<[u64]> + AsRef<[u64]>;
    /// A freshly zeroed 2N-family buffer.
    fn work2() -> Self::Buf2;
    /// A freshly zeroed 4N-family buffer.
    fn work4() -> Self::Buf4;
}

/// Stack u128-limb magnitude scratch for the MG (magic-multiply) `÷ 10^w`
/// divide. The width-generic transcendental core (`exp_generic`) divides
/// by `10^w` once per Taylor term / squaring at its work width `W`; the
/// MG kernel needs a `ceil(N/2)`-limb u128 magnitude buffer
/// (`== BigInt::U128_LIMBS`). Like [`WorkScratch`], this carries that
/// buffer as an **associated type** so its size lives in the `impl`
/// (where the width is concrete) and never appears as a generic const
/// expression in the divide — the generic `div_wide_pow10` slices it to
/// `W::U128_LIMBS`. Same three build forms as [`WorkScratch`].
pub(crate) trait MgScratch {
    /// u128 magnitude buffer, at least `ceil(N/2)` limbs (`U128_LIMBS`).
    type BufU128: AsMut<[u128]> + AsRef<[u128]>;
    /// A freshly zeroed magnitude buffer.
    fn work_u128() -> Self::BufU128;
}

// ── default: one blanket impl, build-max for every N ──────────────────
#[cfg(not(feature = "exact-scratch"))]
mod imp {
    use super::{Int, MgScratch, WorkScratch};
    use crate::int::algos::support::limbs::{work_scratch, MAX_WORK_N};

    // The transcendental work integer `Wexp` runs up to 8× the storage
    // width (e.g. D307's `Int<16>` → `Wexp = Int<128>`; D616's `Int<32>`
    // → `Int<256>`), so the widest `Wexp` is `Int<8·MAX_WORK_N>`, whose
    // `U128_LIMBS = (8·MAX_WORK_N + 1) / 2 ≤ 4·MAX_WORK_N`. A uniform
    // `4·MAX_WORK_N`-limb buffer therefore covers every width; the generic
    // `div_wide_pow10` uses only the leading `W::U128_LIMBS` limbs.
    impl<const N: usize> MgScratch for Int<N> {
        type BufU128 = [u128; 4 * MAX_WORK_N];
        #[inline]
        fn work_u128() -> Self::BufU128 {
            [0u128; 4 * MAX_WORK_N]
        }
    }

    impl<const N: usize> WorkScratch for Int<N> {
        type Buf2 = [u64; work_scratch(2)];
        type Buf4 = [u64; work_scratch(4)];
        #[inline]
        fn work2() -> Self::Buf2 {
            [0u64; work_scratch(2)]
        }
        #[inline]
        fn work4() -> Self::Buf4 {
            [0u64; work_scratch(4)]
        }
    }
}

// ── exact-scratch (stable): one impl per concrete storage width ───────
#[cfg(all(feature = "exact-scratch", not(feature = "exact-scratch-nightly")))]
mod imp {
    use super::{Int, MgScratch, WorkScratch};

    /// `impl MgScratch for Int<$n>` per concrete width, each sized exactly
    /// `ceil(n/2)` from the literal `$n`. Covers the transcendental *work*
    /// widths (`Wexp`: …, 96, 128, 192, 256) as well as the storage widths,
    /// since `exp_generic` runs its `÷ 10^w` at `Wexp`.
    macro_rules! exact_mg_scratch {
        ($($n:literal),+ $(,)?) => { $(
            impl MgScratch for Int<$n> {
                type BufU128 = [u128; ($n + 1) / 2];
                #[inline]
                fn work_u128() -> Self::BufU128 {
                    [0u128; ($n + 1) / 2]
                }
            }
        )+ };
    }
    exact_mg_scratch!(1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256);

    /// `impl WorkScratch for Int<$n>` per concrete width, each sized exactly
    /// `mult·n + ceil(n/2)` from the literal `$n` — stable, no const-expr
    /// generics.
    macro_rules! exact_scratch {
        ($($n:literal),+ $(,)?) => { $(
            impl WorkScratch for Int<$n> {
                type Buf2 = [u64; 2 * $n + ($n + 1) / 2];
                type Buf4 = [u64; 4 * $n + ($n + 1) / 2];
                #[inline]
                fn work2() -> Self::Buf2 {
                    [0u64; 2 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn work4() -> Self::Buf4 {
                    [0u64; 4 * $n + ($n + 1) / 2]
                }
            }
        )+ };
    }

    // Every decimal storage width in limbs: D18=1, D38=2, D57=3, D76=4,
    // D115=6, D153=8, D230=12, D307=16, D462=24, D616=32, D924=48, D1232=64.
    exact_scratch!(1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64);
}

// ── exact-scratch-nightly: one blanket impl, exact per-N via const-expr ─
#[cfg(feature = "exact-scratch-nightly")]
mod imp {
    use super::{Int, MgScratch, WorkScratch};
    use crate::int::algos::support::limbs::work_scratch_n;

    impl<const N: usize> MgScratch for Int<N>
    where
        [(); (N + 1) / 2]:,
    {
        type BufU128 = [u128; (N + 1) / 2];
        #[inline]
        fn work_u128() -> Self::BufU128 {
            [0u128; (N + 1) / 2]
        }
    }

    impl<const N: usize> WorkScratch for Int<N>
    where
        [(); work_scratch_n(2, N)]:,
        [(); work_scratch_n(4, N)]:,
    {
        type Buf2 = [u64; work_scratch_n(2, N)];
        type Buf4 = [u64; work_scratch_n(4, N)];
        #[inline]
        fn work2() -> Self::Buf2 {
            [0u64; work_scratch_n(2, N)]
        }
        #[inline]
        fn work4() -> Self::Buf4 {
            [0u64; work_scratch_n(4, N)]
        }
    }
}
