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

// ── default: one blanket impl, build-max for every N ──────────────────
#[cfg(not(feature = "exact-scratch"))]
mod imp {
    use super::{Int, WorkScratch};
    use crate::int::algos::support::limbs::work_scratch;

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
    use super::{Int, WorkScratch};

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
    use super::{Int, WorkScratch};
    use crate::int::algos::support::limbs::work_scratch_n;

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
