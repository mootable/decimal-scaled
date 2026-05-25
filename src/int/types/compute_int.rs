// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `ComputeInt` — the storage integer's compute-scratch capability.
//!
//! [`crate::int::types::traits::BigInt`] is the cheap, in-place integer
//! surface (add/sub/mul/sqr/cube/pow/cmp/convert/bit). `ComputeInt: BigInt`
//! is the *compute* tier on top of it: the operations whose working width
//! exceeds the value's own `N` limbs and so need a wider scratch buffer.
//!
//! Stable Rust cannot name `[u64; 2N]` from a generic `N`, so each buffer is
//! an **associated type** whose size lives in the `impl` (where `N` is
//! concrete) and never appears in a kernel signature. A kernel bounds on
//! `Int<N>: ComputeInt` and reads the buffer it needs as `&mut [u64]`; the
//! bound discharges for free at the concrete `N` every type method
//! dispatches at, so it does not cascade. The whole decimal layer reaches it
//! through its storage (`D<Int<N>, SCALE>` requires `Int<N>: ComputeInt`).
//!
//! The buffers are clean limb-multiples; **the algorithm chooses** which it
//! needs (e.g. a value divide takes [`single_limbs`], a cbrt radicand divide
//! takes [`quad_limbs`]) — the buffer never has to guess the caller's width.
//!
//! Three build forms, identical sizes numerically — only *who pays for the
//! slack* differs:
//! - **default** — one blanket impl, build-max for every `N`.
//! - **`exact-scratch`** (stable) — one impl per concrete width, each a
//!   size literal.
//! - **`exact-scratch-nightly`** — one blanket impl sized per-`N` via
//!   const-expr under `generic_const_exprs`.
//!
//! [`single_limbs`]: ComputeInt::single_limbs
//! [`quad_limbs`]: ComputeInt::quad_limbs

use crate::int::algos::support::limbs::{max_n_limbs, MAX_WORK_N};
use crate::int::types::traits::BigInt;
use crate::int::types::Int;

// ── Build-max blanket sizes — the all-`N` counterparts of the per-`N`
// `ComputeInt` buffers ────────────────────────────────────────────────────
//
// The COLD paths that structurally cannot size scratch exactly — the bare
// `Int<N>` `/` / `%` operators and all-`N` `BigInt` methods (blanket over
// every `N`, so they can neither name `[u64; N + 2]` on stable nor carry a
// per-width `ComputeInt` bound — see the wall note on [`ComputeInt`]), plus
// the schoolbook reference baselines — use these build-max blankets. Each is
// the widest value of the matching [`ComputeInt`] buffer over every `N` a
// build can form, feature-gated through
// [`MAX_WORK_N`](crate::int::algos::support::limbs) so a narrow build does
// NOT pay the widest tier's size. The `single`/`u128` blankets cover the
// wide-transcendental work widths (`Int<256>` = `4·MAX_WORK_N`); the
// `double`/`quad` radicand blankets are storage-scoped (`max_n_limbs`).
//
// **Hot paths never touch these.** A concrete-`N` caller carrying
// `Int<N>: ComputeInt` sources the exact per-width [`single_limbs`] /
// [`double_limbs`] / [`quad_limbs`] / [`u128_limbs`] buffers instead. These
// blankets are the fallback the exact-scratch migration is progressively
// starving; the aim is to retire them once every reaching path is exact.
//
// [`single_limbs`]: ComputeInt::single_limbs
// [`double_limbs`]: ComputeInt::double_limbs
// [`quad_limbs`]: ComputeInt::quad_limbs
// [`u128_limbs`]: ComputeInt::u128_limbs

/// Build-max [`single_limbs`](ComputeInt::single_limbs) — a value-width
/// divide's normalised `u`/`v`, covering the widest work value
/// (`4·MAX_WORK_N + 2`).
pub(crate) const MAX_SINGLE_LIMBS: usize = 4 * MAX_WORK_N + 2;
/// Build-max [`double_limbs`](ComputeInt::double_limbs) — the `2N`-family
/// sqrt/isqrt radicand (`max_n_limbs(2)`, storage-scoped).
pub(crate) const MAX_DOUBLE_LIMBS: usize = max_n_limbs(2);
/// Build-max [`quad_limbs`](ComputeInt::quad_limbs) — the `4N`-family
/// cbrt/icbrt radicand (`max_n_limbs(4)`, storage-scoped).
pub(crate) const MAX_QUADRUPLE_LIMBS: usize = max_n_limbs(4);
/// Build-max [`u128_limbs`](ComputeInt::u128_limbs) — the MG `÷10^w`
/// magnitude, covering the widest work value (`4·MAX_WORK_N` u128).
pub(crate) const MAX_U128_LIMB: usize = 4 * MAX_WORK_N;

// The build-max blanket buffers, freshly zeroed — the `N`-free counterparts
// of the per-width `ComputeInt::{single,double,quad,u128}_limbs` methods.
// A cold blanket caller takes the *created* limbs directly (no size to
// restate); the `MAX_*` constants above are there when only the number is
// needed (array types, length asserts, derived sizes).

/// A freshly zeroed build-max [`single_limbs`](ComputeInt::single_limbs)
/// `u`/`v` divide buffer.
#[inline]
pub(crate) fn max_single_limbs() -> [u64; MAX_SINGLE_LIMBS] {
    [0u64; MAX_SINGLE_LIMBS]
}
/// A freshly zeroed build-max [`double_limbs`](ComputeInt::double_limbs)
/// radicand buffer.
#[inline]
pub(crate) fn max_double_limbs() -> [u64; MAX_DOUBLE_LIMBS] {
    [0u64; MAX_DOUBLE_LIMBS]
}
/// A freshly zeroed build-max [`quad_limbs`](ComputeInt::quad_limbs)
/// radicand buffer.
#[inline]
pub(crate) fn max_quadruple_limbs() -> [u64; MAX_QUADRUPLE_LIMBS] {
    [0u64; MAX_QUADRUPLE_LIMBS]
}
/// A freshly zeroed build-max [`u128_limbs`](ComputeInt::u128_limbs)
/// magnitude buffer.
#[inline]
pub(crate) fn max_u128_limb() -> [u128; MAX_U128_LIMB] {
    [0u128; MAX_U128_LIMB]
}

// ── The `Limb` axis — `u64` / `u128` width-generic kernels ─────────────────
//
// A wide-tier kernel can run faster in u128 limbs (half the limbs/carries).
// Which width wins is a per-`(N, SCALE)` const property, so it is a second
// matcher axis (the `Select` verdict carries a `LimbSize`). The width is
// delivered BY TYPE: a `<L: Limb>`-generic kernel is monomorphised per width
// via a const-folded `match` on the verdict — ONE kernel, never a per-limb
// copy. See `docs/ARCHITECTURE.md` → "Limb width — the matcher's second axis".

/// The limb width a `<L: Limb>` kernel runs in, chosen by the matcher per
/// `(N, SCALE)`. A const, value-independent property (the const part of the
/// `Select` verdict) — packing pairs two u64 into one u128, so `U128` is only
/// valid for an even limb count (the matcher gates this).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum LimbSize {
    U64,
    U128,
}

impl LimbSize {
    /// The limb-width verdict for a kernel over `n` u64 limbs: `U128` when the
    /// packing is exact (even `n` — two u64 fold into one u128, the wide-tier
    /// win), else `U64`. A `const fn` so callers fold it in a `const { … }`
    /// block per monomorphisation (the unchosen `match` arm is then dead-arm
    /// eliminated, like any policy verdict).
    ///
    /// This is the limb-width axis as a verdict; the algorithm axis composes
    /// alongside it where a function also chooses *which* algorithm (a full
    /// `Select<N>` carrying `(Algorithm, LimbSize)`). The even-`n` rule is the
    /// correctness gate; a per-`(N, SCALE)` *perf* refinement (which even
    /// widths actually win the u128 packing) is a microbench tuning follow-up.
    #[inline]
    pub(crate) const fn for_packing(n: usize) -> Self {
        if n % 2 == 0 {
            LimbSize::U128
        } else {
            LimbSize::U64
        }
    }
}

/// The scalar limb type a width-generic kernel computes in — `u64`, or the
/// packed `u128` (two storage u64 limbs per limb). Carries the primitives a
/// slice kernel needs plus the pack/unpack to/from the `Int<N>` u64 storage.
/// Implemented for exactly `u64` and `u128`.
pub(crate) trait Limb: Copy + PartialEq {
    /// Additive identity (the array-repeat seed for scratch buffers).
    const ZERO: Self;
    /// Number of `L` limbs holding an `n`-u64-limb value: `n` for `u64`,
    /// `n / 2` for `u128` (caller guarantees even `n` for the `u128` impl).
    fn packed_len(n_u64: usize) -> usize;
    /// Pack the low `dst.len()` little-endian u64 limbs of `src_u64` into
    /// `dst` `L` limbs.
    fn pack(src_u64: &[u64], dst: &mut [Self]);
    /// Unpack `src` `L` limbs back into the low little-endian u64 limbs of
    /// `dst_u64`.
    fn unpack(src: &[Self], dst_u64: &mut [u64]);
    /// Full widening product `self · rhs → (low, high)` limbs.
    fn widening_mul(self, rhs: Self) -> (Self, Self);
    /// `self + rhs → (sum, carry)`.
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    /// `self + c1 + c2` — the schoolbook carry merge. The column bound
    /// (`hi ≤ MAX − 1`, and `c1`/`c2` never both set) guarantees no overflow.
    fn add_carries(self, c1: bool, c2: bool) -> Self;
}

impl Limb for u64 {
    const ZERO: Self = 0;
    #[inline]
    fn packed_len(n_u64: usize) -> usize {
        n_u64
    }
    #[inline]
    fn pack(src_u64: &[u64], dst: &mut [Self]) {
        let h = dst.len();
        dst.copy_from_slice(&src_u64[..h]);
    }
    #[inline]
    fn unpack(src: &[Self], dst_u64: &mut [u64]) {
        let h = src.len();
        dst_u64[..h].copy_from_slice(src);
    }
    #[inline]
    fn widening_mul(self, rhs: Self) -> (Self, Self) {
        let p = (self as u128) * (rhs as u128);
        (p as u64, (p >> 64) as u64)
    }
    #[inline]
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        u64::overflowing_add(self, rhs)
    }
    #[inline]
    fn add_carries(self, c1: bool, c2: bool) -> Self {
        self.wrapping_add(c1 as u64).wrapping_add(c2 as u64)
    }
}

impl Limb for u128 {
    const ZERO: Self = 0;
    #[inline]
    fn packed_len(n_u64: usize) -> usize {
        n_u64 / 2
    }
    #[inline]
    fn pack(src_u64: &[u64], dst: &mut [Self]) {
        for (k, d) in dst.iter_mut().enumerate() {
            *d = (src_u64[2 * k] as u128) | ((src_u64[2 * k + 1] as u128) << 64);
        }
    }
    #[inline]
    fn unpack(src: &[Self], dst_u64: &mut [u64]) {
        for (k, &s) in src.iter().enumerate() {
            dst_u64[2 * k] = s as u64;
            dst_u64[2 * k + 1] = (s >> 64) as u64;
        }
    }
    #[inline]
    fn widening_mul(self, rhs: Self) -> (Self, Self) {
        // `a · b → (low128, high128)` from four u64·u64→u128 partials
        // (`MUL`+`UMULH`); the full 256-bit product is `low + (high << 128)`.
        let a_lo = self as u64 as u128;
        let a_hi = (self >> 64) as u64 as u128;
        let b_lo = rhs as u64 as u128;
        let b_hi = (rhs >> 64) as u64 as u128;
        let ll = a_lo * b_lo;
        let lh = a_lo * b_hi;
        let hl = a_hi * b_lo;
        let hh = a_hi * b_hi;
        let (mid, mid_carry) = lh.overflowing_add(hl);
        let (low, c) = ll.overflowing_add(mid << 64);
        let high = hh + (mid >> 64) + (c as u128) + ((mid_carry as u128) << 64);
        (low, high)
    }
    #[inline]
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        u128::overflowing_add(self, rhs)
    }
    #[inline]
    fn add_carries(self, c1: bool, c2: bool) -> Self {
        self.wrapping_add(c1 as u128).wrapping_add(c2 as u128)
    }
}

/// The storage integer's compute-scratch capability: clean limb-multiple
/// stack buffers for the operations that work wider than `N` limbs.
///
/// - [`single_limbs`](ComputeInt::single_limbs) — `N + 2` u64: a value
///   divide's normalised dividend/divisor (`div_rem` sources two).
/// - [`double_limbs`](ComputeInt::double_limbs) — `2N + ⌈N/2⌉` u64: the
///   sqrt/hypot/isqrt radicand and the decimal scaled-numerator.
/// - [`quad_limbs`](ComputeInt::quad_limbs) — `4N + ⌈N/2⌉` u64: the
///   cbrt/icbrt radicand (and that radicand's internal divide).
/// - [`u128_limbs`](ComputeInt::u128_limbs) — `⌈N/2⌉` u128: the MG `÷10^w`
///   magnitude (`== U128_LIMBS`).
///
/// **Add more limb buffers if an algorithm needs them** — there is nothing
/// special about 1×/2×/4×; just add a `<name>_limbs` method + its associated
/// `LimbBuf<k>` and a size literal in each of the three build-form impls
/// below. The format for the two element types:
/// - **u64 buffer:** `[u64; mult·N + ⌈N/2⌉]` — a limb-multiple of the
///   storage width plus the `⌈N/2⌉` carry/headroom slack the radicands need.
///   (`single_limbs` is the deliberate exception, `N + 2`: a divide's
///   normalised dividend needs only its one extra top limb, not the slack.)
/// - **u128 buffer:** `[u128; mult·⌈N/2⌉]` — the packed-u128 form, `mult`
///   times the value's `⌈N/2⌉ == U128_LIMBS` count.
pub(crate) trait ComputeInt: BigInt {
    /// `N + 2` u64 — a value-width divide's normalised `u`/`v`.
    type LimbBuf1: AsMut<[u64]> + AsRef<[u64]>;
    /// `2N + ⌈N/2⌉` u64 — sqrt/hypot/isqrt radicand, scaled numerator.
    type LimbBuf2: AsMut<[u64]> + AsRef<[u64]>;
    /// `4N + ⌈N/2⌉` u64 — cbrt/icbrt radicand.
    type LimbBuf4: AsMut<[u64]> + AsRef<[u64]>;
    /// `⌈N/2⌉` u128 — MG `÷10^w` magnitude (`== U128_LIMBS`).
    type LimbBufU128: AsMut<[u128]> + AsRef<[u128]>;
    /// A freshly zeroed `N + 2` u64 buffer.
    fn single_limbs() -> Self::LimbBuf1;
    /// A freshly zeroed `2N`-family u64 buffer.
    fn double_limbs() -> Self::LimbBuf2;
    /// A freshly zeroed `4N`-family u64 buffer.
    fn quad_limbs() -> Self::LimbBuf4;
    /// A freshly zeroed `⌈N/2⌉` u128 buffer.
    fn u128_limbs() -> Self::LimbBufU128;
}

// ── default: one blanket impl, build-max for every N ──────────────────
#[cfg(not(feature = "exact-scratch"))]
mod imp {
    use super::{
        max_double_limbs, max_quadruple_limbs, max_single_limbs, max_u128_limb, ComputeInt, Int,
        MAX_DOUBLE_LIMBS, MAX_QUADRUPLE_LIMBS, MAX_SINGLE_LIMBS, MAX_U128_LIMB,
    };

    // The blanket build-max impl: every buffer is its `MAX_*` blanket size
    // (the `single`/`u128` value buffers cover the wide-transcendental work
    // widths; the `double`/`quad` radicands are storage-scoped), and every
    // constructor is the matching `max_*` builder. The exact-scratch form
    // sizes per width instead.
    impl<const N: usize> ComputeInt for Int<N> {
        type LimbBuf1 = [u64; MAX_SINGLE_LIMBS];
        type LimbBuf2 = [u64; MAX_DOUBLE_LIMBS];
        type LimbBuf4 = [u64; MAX_QUADRUPLE_LIMBS];
        type LimbBufU128 = [u128; MAX_U128_LIMB];
        #[inline]
        fn single_limbs() -> Self::LimbBuf1 {
            max_single_limbs()
        }
        #[inline]
        fn double_limbs() -> Self::LimbBuf2 {
            max_double_limbs()
        }
        #[inline]
        fn quad_limbs() -> Self::LimbBuf4 {
            max_quadruple_limbs()
        }
        #[inline]
        fn u128_limbs() -> Self::LimbBufU128 {
            max_u128_limb()
        }
    }
}

// ── exact-scratch (stable): one impl per concrete width ───────────────
#[cfg(all(feature = "exact-scratch", not(feature = "exact-scratch-nightly")))]
mod imp {
    use super::{ComputeInt, Int};

    /// `impl ComputeInt for Int<$n>` per concrete width — every buffer a
    /// size literal. `single = n+2` (a value divide's `u`/`v`, no headroom
    /// beyond the normalised top limb); `double`/`quad` = `mult·n + ⌈n/2⌉`
    /// radicands; `u128 = ⌈n/2⌉`. Covers storage widths AND the wide
    /// transcendental work widths (96..256, where the value-width divide and
    /// `÷10^w` run).
    macro_rules! exact_compute {
        ($($n:literal),+ $(,)?) => { $(
            impl ComputeInt for Int<$n> {
                type LimbBuf1 = [u64; $n + 2];
                type LimbBuf2 = [u64; 2 * $n + ($n + 1) / 2];
                type LimbBuf4 = [u64; 4 * $n + ($n + 1) / 2];
                type LimbBufU128 = [u128; ($n + 1) / 2];
                #[inline]
                fn single_limbs() -> Self::LimbBuf1 {
                    [0u64; $n + 2]
                }
                #[inline]
                fn double_limbs() -> Self::LimbBuf2 {
                    [0u64; 2 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn quad_limbs() -> Self::LimbBuf4 {
                    [0u64; 4 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn u128_limbs() -> Self::LimbBufU128 {
                    [0u128; ($n + 1) / 2]
                }
            }
        )+ };
    }
    // Decimal storage widths + the transcendental work widths (96..256).
    exact_compute!(1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256);
}

// ── exact-scratch-nightly: one blanket impl, exact per-N via const-expr ─
#[cfg(feature = "exact-scratch-nightly")]
mod imp {
    use super::{ComputeInt, Int};
    use crate::int::algos::support::limbs::n_limbs;

    impl<const N: usize> ComputeInt for Int<N>
    where
        [(); N + 2]:,
        [(); n_limbs(2, N)]:,
        [(); n_limbs(4, N)]:,
        [(); (N + 1) / 2]:,
    {
        type LimbBuf1 = [u64; N + 2];
        type LimbBuf2 = [u64; n_limbs(2, N)];
        type LimbBuf4 = [u64; n_limbs(4, N)];
        type LimbBufU128 = [u128; (N + 1) / 2];
        #[inline]
        fn single_limbs() -> Self::LimbBuf1 {
            [0u64; N + 2]
        }
        #[inline]
        fn double_limbs() -> Self::LimbBuf2 {
            [0u64; n_limbs(2, N)]
        }
        #[inline]
        fn quad_limbs() -> Self::LimbBuf4 {
            [0u64; n_limbs(4, N)]
        }
        #[inline]
        fn u128_limbs() -> Self::LimbBufU128 {
            [0u128; (N + 1) / 2]
        }
    }
}
