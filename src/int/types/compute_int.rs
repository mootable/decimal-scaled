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
//! # The buffer family — value-width × element × plain/buffered
//!
//! The buffers are clean limb-multiples organised on three orthogonal axes;
//! **the algorithm chooses** which it needs — the buffer never has to guess
//! the caller's width.
//!
//! 1. **value width**, in 64-bit limbs: `single` = `N`, `double` = `2N`,
//!    `quad` = `4N`. There is **no `half`** — half is `single`. `N` is
//!    ALWAYS a count of 64-bit limbs, even for a `u128` buffer (which then
//!    packs `⌈width/2⌉` u128 limbs holding the same value).
//! 2. **element** — `u64` or `u128`, selected by the method suffix
//!    (`_u64` / `_u128`) or, generically, by [`Limb`] (`L::single::<I>()`).
//!    The `u128` buffer holds the SAME value in `⌈width/2⌉` u128 limbs.
//! 3. **plain vs buffered** — plain is the exact value width; buffered adds
//!    the carry/normalisation headroom an algorithm needs. The headroom is
//!    per-use and genuinely differs: `single_buffered` is `+2` (a value
//!    divide's normalised top limb), `double`/`quad` buffered are `+⌈N/2⌉`
//!    (the radicand slack — the `mult·N + ⌈N/2⌉` work-scratch formula).
//!
//! | buffer             | u64 size            | u128 size                    |
//! |--------------------|---------------------|------------------------------|
//! | `single`           | `[u64; N]`          | `[u128; ⌈N/2⌉]`              |
//! | `single_buffered`  | `[u64; N+2]`        | `[u128; ⌈(N+2)/2⌉]`         |
//! | `double`           | `[u64; 2N]`         | `[u128; N]`                  |
//! | `double_buffered`  | `[u64; 2N+⌈N/2⌉]`  | `[u128; ⌈(2N+⌈N/2⌉)/2⌉]`   |
//! | `quad`             | `[u64; 4N]`         | `[u128; 2N]`                 |
//! | `quad_buffered`    | `[u64; 4N+⌈N/2⌉]`  | `[u128; ⌈(4N+⌈N/2⌉)/2⌉]`   |
//!
//! Three build forms, identical sizes numerically — only *who pays for the
//! slack* differs:
//! - **default** — one blanket impl, build-max for every `N`.
//! - **`exact-scratch`** (stable) — one impl per concrete width, each a
//!   size literal.
//! - **`exact-scratch-nightly`** — one blanket impl sized per-`N` via
//!   const-expr under `generic_const_exprs`.

// `⌈x/2⌉` is written `(x + 1) / 2` throughout this file — the crate's const
// limb-sizing idiom (see `max_n_limbs`, `BigInt::mag_into_u128`). The
// `.div_ceil()` method form is unusable in exactly the positions this file
// needs it: in the `exact-scratch` macro `$n.div_ceil(2)` mis-lexes a bare
// integer literal (`16.` parses as a float), and in the `exact-scratch-nightly`
// `generic_const_exprs` bounds a method call is not a reliable const-generic
// expression. The plain division is the correct, portable form here.
#![allow(clippy::manual_div_ceil)]

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
// `Int<N>: ComputeInt` sources the exact per-width family methods
// ([`single_buffered_u64`](ComputeInt::single_buffered_u64), …) instead.
// These blankets are the fallback the exact-scratch migration is
// progressively starving; the aim is to retire them once every reaching path
// is exact.

/// Build-max `single_buffered` u64 — a value-width divide's normalised
/// `u`/`v`, covering the widest work value (`4·MAX_WORK_N + 2`).
pub(crate) const MAX_SINGLE_LIMBS: usize = 4 * MAX_WORK_N + 2;
/// Build-max `double_buffered` u64 — the `2N`-family sqrt/isqrt radicand
/// (`max_n_limbs(2)`, storage-scoped).
pub(crate) const MAX_DOUBLE_LIMBS: usize = max_n_limbs(2);
/// Build-max `quad_buffered` u64 — the `4N`-family cbrt/icbrt radicand
/// (`max_n_limbs(4)`, storage-scoped).
pub(crate) const MAX_QUADRUPLE_LIMBS: usize = max_n_limbs(4);
/// Build-max `single` u128 — the MG `÷10^w` magnitude, covering the widest
/// work value (`4·MAX_WORK_N` u128).
pub(crate) const MAX_U128_LIMB: usize = 4 * MAX_WORK_N;

// The remaining family members' build-max sizes. The plain `single` u64 is
// the value width itself (work-scoped); the plain `double`/`quad` u64 reuse
// their buffered blanket (build-max over-allocation of the `⌈N/2⌉` slack is
// harmless on the cold blanket path). Every u128 blanket is `⌈u64/2⌉` of its
// own u64 blanket, so it covers each width's exact `⌈value/2⌉` packing.

/// Build-max `single` u64 — the plain value width (`4·MAX_WORK_N`).
const MAX_SINGLE_U64: usize = 4 * MAX_WORK_N;
/// Build-max `single` u128 — `⌈(4·MAX_WORK_N)/2⌉`.
const MAX_SINGLE_U128: usize = (MAX_SINGLE_U64 + 1) / 2;
/// Build-max `single_buffered` u128 — `⌈(4·MAX_WORK_N + 2)/2⌉`.
const MAX_SINGLE_BUF_U128: usize = (MAX_SINGLE_LIMBS + 1) / 2;
/// Build-max `double`/`double_buffered` u128 — `⌈max_n_limbs(2)/2⌉`.
const MAX_DOUBLE_U128: usize = (MAX_DOUBLE_LIMBS + 1) / 2;
/// Build-max `quad`/`quad_buffered` u128 — `⌈max_n_limbs(4)/2⌉`.
const MAX_QUAD_U128: usize = (MAX_QUADRUPLE_LIMBS + 1) / 2;

// The build-max blanket constructors still called from the genuinely-`N`-less
// blanket paths (the `Int<N>` operators, `BigInt` methods, schoolbook
// baselines): a freshly zeroed widest buffer the caller takes directly.

/// A freshly zeroed build-max `single_buffered` u64 (`u`/`v` divide) buffer.
#[inline]
pub(crate) fn max_single_limbs() -> [u64; MAX_SINGLE_LIMBS] {
    [0u64; MAX_SINGLE_LIMBS]
}
/// A freshly zeroed build-max `double_buffered` u64 radicand buffer.
#[inline]
pub(crate) fn max_double_limbs() -> [u64; MAX_DOUBLE_LIMBS] {
    [0u64; MAX_DOUBLE_LIMBS]
}
/// A freshly zeroed build-max `quad_buffered` u64 radicand buffer.
#[inline]
pub(crate) fn max_quadruple_limbs() -> [u64; MAX_QUADRUPLE_LIMBS] {
    [0u64; MAX_QUADRUPLE_LIMBS]
}
/// A freshly zeroed build-max `single` u128 magnitude buffer.
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
/// slice kernel needs, the pack/unpack to/from the `Int<N>` u64 storage, and
/// the width-generic scratch fetch (the [`ComputeInt`] buffer family, by
/// element). Implemented for exactly `u64` and `u128`.
pub(crate) trait Limb: Copy + PartialEq {
    /// Additive identity (the array-repeat seed for scratch buffers).
    const ZERO: Self;
    /// Multiplicative identity — materialises a carry *bit* as a limb value
    /// for multi-limb carry propagation (e.g. the symmetric-square doubling
    /// pass), where `overflowing_add` needs a value addend, not a `bool`.
    const ONE: Self;
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

    // Width-generic scratch fetch. Each forwards to the matching
    // [`ComputeInt`] per-element buffer for this limb type, so a
    // `<L: Limb>` kernel sources its own exactly-sized scratch as
    // `L::double_buffered::<Int<N>>()` regardless of `L`.
    /// `single` (`N`-value-width) buffer in this limb type.
    type Single<I: ComputeInt>: AsMut<[Self]> + AsRef<[Self]>;
    /// `single_buffered` (`N`-value + headroom) buffer in this limb type.
    type SingleBuffered<I: ComputeInt>: AsMut<[Self]> + AsRef<[Self]>;
    /// `double` (`2N`-value-width) buffer in this limb type.
    type Double<I: ComputeInt>: AsMut<[Self]> + AsRef<[Self]>;
    /// `double_buffered` (`2N`-value + radicand slack) buffer.
    type DoubleBuffered<I: ComputeInt>: AsMut<[Self]> + AsRef<[Self]>;
    /// `quad` (`4N`-value-width) buffer in this limb type.
    type Quad<I: ComputeInt>: AsMut<[Self]> + AsRef<[Self]>;
    /// `quad_buffered` (`4N`-value + radicand slack) buffer.
    type QuadBuffered<I: ComputeInt>: AsMut<[Self]> + AsRef<[Self]>;
    /// Fetch a freshly zeroed `single` buffer in this limb type.
    fn single<I: ComputeInt>() -> Self::Single<I>;
    /// Fetch a freshly zeroed `single_buffered` buffer in this limb type.
    fn single_buffered<I: ComputeInt>() -> Self::SingleBuffered<I>;
    /// Fetch a freshly zeroed `double` buffer in this limb type.
    fn double<I: ComputeInt>() -> Self::Double<I>;
    /// Fetch a freshly zeroed `double_buffered` buffer in this limb type.
    fn double_buffered<I: ComputeInt>() -> Self::DoubleBuffered<I>;
    /// Fetch a freshly zeroed `quad` buffer in this limb type.
    fn quad<I: ComputeInt>() -> Self::Quad<I>;
    /// Fetch a freshly zeroed `quad_buffered` buffer in this limb type.
    fn quad_buffered<I: ComputeInt>() -> Self::QuadBuffered<I>;
}

impl Limb for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
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

    type Single<I: ComputeInt> = I::SingleU64;
    type SingleBuffered<I: ComputeInt> = I::SingleBufferedU64;
    type Double<I: ComputeInt> = I::DoubleU64;
    type DoubleBuffered<I: ComputeInt> = I::DoubleBufferedU64;
    type Quad<I: ComputeInt> = I::QuadU64;
    type QuadBuffered<I: ComputeInt> = I::QuadBufferedU64;
    #[inline]
    fn single<I: ComputeInt>() -> Self::Single<I> {
        I::single_u64()
    }
    #[inline]
    fn single_buffered<I: ComputeInt>() -> Self::SingleBuffered<I> {
        I::single_buffered_u64()
    }
    #[inline]
    fn double<I: ComputeInt>() -> Self::Double<I> {
        I::double_u64()
    }
    #[inline]
    fn double_buffered<I: ComputeInt>() -> Self::DoubleBuffered<I> {
        I::double_buffered_u64()
    }
    #[inline]
    fn quad<I: ComputeInt>() -> Self::Quad<I> {
        I::quad_u64()
    }
    #[inline]
    fn quad_buffered<I: ComputeInt>() -> Self::QuadBuffered<I> {
        I::quad_buffered_u64()
    }
}

impl Limb for u128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
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

    type Single<I: ComputeInt> = I::SingleU128;
    type SingleBuffered<I: ComputeInt> = I::SingleBufferedU128;
    type Double<I: ComputeInt> = I::DoubleU128;
    type DoubleBuffered<I: ComputeInt> = I::DoubleBufferedU128;
    type Quad<I: ComputeInt> = I::QuadU128;
    type QuadBuffered<I: ComputeInt> = I::QuadBufferedU128;
    #[inline]
    fn single<I: ComputeInt>() -> Self::Single<I> {
        I::single_u128()
    }
    #[inline]
    fn single_buffered<I: ComputeInt>() -> Self::SingleBuffered<I> {
        I::single_buffered_u128()
    }
    #[inline]
    fn double<I: ComputeInt>() -> Self::Double<I> {
        I::double_u128()
    }
    #[inline]
    fn double_buffered<I: ComputeInt>() -> Self::DoubleBuffered<I> {
        I::double_buffered_u128()
    }
    #[inline]
    fn quad<I: ComputeInt>() -> Self::Quad<I> {
        I::quad_u128()
    }
    #[inline]
    fn quad_buffered<I: ComputeInt>() -> Self::QuadBuffered<I> {
        I::quad_buffered_u128()
    }
}

/// The storage integer's compute-scratch capability: clean limb-multiple
/// stack buffers for the operations that work wider than `N` limbs, on the
/// `single`/`double`/`quad` × `u64`/`u128` × plain/`buffered` family (see the
/// module docs for the size table). The element-suffixed methods are the
/// direct accessors; [`Limb`] forwards to them so a width-generic kernel can
/// fetch its own-typed buffer as `L::double_buffered::<Int<N>>()`.
///
/// **Add more limb buffers if an algorithm needs them** — there is nothing
/// special about 1×/2×/4×; add a size axis the same way (a literal in each of
/// the three build-form impls below, an associated type per element, and the
/// [`Limb`] forwarder).
pub(crate) trait ComputeInt: BigInt {
    /// `[u64; N]` — plain value width.
    type SingleU64: AsMut<[u64]> + AsRef<[u64]>;
    /// `[u128; ⌈N/2⌉]` — plain value width (the MG `÷10^w` magnitude).
    type SingleU128: AsMut<[u128]> + AsRef<[u128]>;
    /// `[u64; N + 2]` — a value-width divide's normalised `u`/`v`.
    type SingleBufferedU64: AsMut<[u64]> + AsRef<[u64]>;
    /// `[u128; ⌈(N + 2)/2⌉]` — the packed-u128 `single_buffered`.
    type SingleBufferedU128: AsMut<[u128]> + AsRef<[u128]>;
    /// `[u64; 2N]` — plain double value width.
    type DoubleU64: AsMut<[u64]> + AsRef<[u64]>;
    /// `[u128; N]` — packed-u128 plain double.
    type DoubleU128: AsMut<[u128]> + AsRef<[u128]>;
    /// `[u64; 2N + ⌈N/2⌉]` — sqrt/hypot/isqrt radicand, scaled numerator.
    type DoubleBufferedU64: AsMut<[u64]> + AsRef<[u64]>;
    /// `[u128; ⌈(2N + ⌈N/2⌉)/2⌉]` — packed-u128 `double_buffered`.
    type DoubleBufferedU128: AsMut<[u128]> + AsRef<[u128]>;
    /// `[u64; 4N]` — plain quad value width.
    type QuadU64: AsMut<[u64]> + AsRef<[u64]>;
    /// `[u128; 2N]` — packed-u128 plain quad.
    type QuadU128: AsMut<[u128]> + AsRef<[u128]>;
    /// `[u64; 4N + ⌈N/2⌉]` — cbrt/icbrt radicand.
    type QuadBufferedU64: AsMut<[u64]> + AsRef<[u64]>;
    /// `[u128; ⌈(4N + ⌈N/2⌉)/2⌉]` — packed-u128 `quad_buffered`.
    type QuadBufferedU128: AsMut<[u128]> + AsRef<[u128]>;

    /// A freshly zeroed `[u64; N]` buffer.
    fn single_u64() -> Self::SingleU64;
    /// A freshly zeroed `[u128; ⌈N/2⌉]` buffer.
    fn single_u128() -> Self::SingleU128;
    /// A freshly zeroed `[u64; N + 2]` buffer.
    fn single_buffered_u64() -> Self::SingleBufferedU64;
    /// A freshly zeroed `[u128; ⌈(N + 2)/2⌉]` buffer.
    fn single_buffered_u128() -> Self::SingleBufferedU128;
    /// A freshly zeroed `[u64; 2N]` buffer.
    fn double_u64() -> Self::DoubleU64;
    /// A freshly zeroed `[u128; N]` buffer.
    fn double_u128() -> Self::DoubleU128;
    /// A freshly zeroed `[u64; 2N + ⌈N/2⌉]` buffer.
    fn double_buffered_u64() -> Self::DoubleBufferedU64;
    /// A freshly zeroed `[u128; ⌈(2N + ⌈N/2⌉)/2⌉]` buffer.
    fn double_buffered_u128() -> Self::DoubleBufferedU128;
    /// A freshly zeroed `[u64; 4N]` buffer.
    fn quad_u64() -> Self::QuadU64;
    /// A freshly zeroed `[u128; 2N]` buffer.
    fn quad_u128() -> Self::QuadU128;
    /// A freshly zeroed `[u64; 4N + ⌈N/2⌉]` buffer.
    fn quad_buffered_u64() -> Self::QuadBufferedU64;
    /// A freshly zeroed `[u128; ⌈(4N + ⌈N/2⌉)/2⌉]` buffer.
    fn quad_buffered_u128() -> Self::QuadBufferedU128;
}

// ── default: one blanket impl, build-max for every N ──────────────────
#[cfg(not(feature = "exact-scratch"))]
mod imp {
    use super::{
        ComputeInt, Int, MAX_DOUBLE_LIMBS, MAX_DOUBLE_U128, MAX_QUADRUPLE_LIMBS, MAX_QUAD_U128,
        MAX_SINGLE_BUF_U128, MAX_SINGLE_LIMBS, MAX_SINGLE_U128, MAX_SINGLE_U64,
    };

    // The blanket build-max impl: every buffer is its `MAX_*` blanket size.
    // The `single`/`u128` value buffers cover the wide-transcendental work
    // widths; the `double`/`quad` radicands are storage-scoped. The plain
    // `double`/`quad` u64 reuse the buffered blanket (the cold blanket can
    // over-allocate the `⌈N/2⌉` slack harmlessly). The exact-scratch form
    // sizes per width instead.
    impl<const N: usize> ComputeInt for Int<N> {
        type SingleU64 = [u64; MAX_SINGLE_U64];
        type SingleU128 = [u128; MAX_SINGLE_U128];
        type SingleBufferedU64 = [u64; MAX_SINGLE_LIMBS];
        type SingleBufferedU128 = [u128; MAX_SINGLE_BUF_U128];
        type DoubleU64 = [u64; MAX_DOUBLE_LIMBS];
        type DoubleU128 = [u128; MAX_DOUBLE_U128];
        type DoubleBufferedU64 = [u64; MAX_DOUBLE_LIMBS];
        type DoubleBufferedU128 = [u128; MAX_DOUBLE_U128];
        type QuadU64 = [u64; MAX_QUADRUPLE_LIMBS];
        type QuadU128 = [u128; MAX_QUAD_U128];
        type QuadBufferedU64 = [u64; MAX_QUADRUPLE_LIMBS];
        type QuadBufferedU128 = [u128; MAX_QUAD_U128];
        #[inline]
        fn single_u64() -> Self::SingleU64 {
            [0u64; MAX_SINGLE_U64]
        }
        #[inline]
        fn single_u128() -> Self::SingleU128 {
            [0u128; MAX_SINGLE_U128]
        }
        #[inline]
        fn single_buffered_u64() -> Self::SingleBufferedU64 {
            [0u64; MAX_SINGLE_LIMBS]
        }
        #[inline]
        fn single_buffered_u128() -> Self::SingleBufferedU128 {
            [0u128; MAX_SINGLE_BUF_U128]
        }
        #[inline]
        fn double_u64() -> Self::DoubleU64 {
            [0u64; MAX_DOUBLE_LIMBS]
        }
        #[inline]
        fn double_u128() -> Self::DoubleU128 {
            [0u128; MAX_DOUBLE_U128]
        }
        #[inline]
        fn double_buffered_u64() -> Self::DoubleBufferedU64 {
            [0u64; MAX_DOUBLE_LIMBS]
        }
        #[inline]
        fn double_buffered_u128() -> Self::DoubleBufferedU128 {
            [0u128; MAX_DOUBLE_U128]
        }
        #[inline]
        fn quad_u64() -> Self::QuadU64 {
            [0u64; MAX_QUADRUPLE_LIMBS]
        }
        #[inline]
        fn quad_u128() -> Self::QuadU128 {
            [0u128; MAX_QUAD_U128]
        }
        #[inline]
        fn quad_buffered_u64() -> Self::QuadBufferedU64 {
            [0u64; MAX_QUADRUPLE_LIMBS]
        }
        #[inline]
        fn quad_buffered_u128() -> Self::QuadBufferedU128 {
            [0u128; MAX_QUAD_U128]
        }
    }
}

// ── exact-scratch (stable): one impl per concrete width ───────────────
#[cfg(all(feature = "exact-scratch", not(feature = "exact-scratch-nightly")))]
mod imp {
    use super::{ComputeInt, Int};

    /// `impl ComputeInt for Int<$n>` per concrete width — every buffer a
    /// size literal. `single` = `n` (value width), `single_buffered` = `n+2`
    /// (a value divide's `u`/`v`); `double`/`quad` = `mult·n` plain,
    /// `mult·n + ⌈n/2⌉` buffered radicands; each u128 buffer is `⌈u64/2⌉` of
    /// its u64 sibling. Covers storage widths AND the wide transcendental
    /// work widths (96..256, where the value-width divide and `÷10^w` run).
    macro_rules! exact_compute {
        ($($n:literal),+ $(,)?) => { $(
            impl ComputeInt for Int<$n> {
                type SingleU64 = [u64; $n];
                type SingleU128 = [u128; ($n + 1) / 2];
                type SingleBufferedU64 = [u64; $n + 2];
                type SingleBufferedU128 = [u128; ($n + 3) / 2];
                type DoubleU64 = [u64; 2 * $n];
                type DoubleU128 = [u128; $n];
                type DoubleBufferedU64 = [u64; 2 * $n + ($n + 1) / 2];
                type DoubleBufferedU128 = [u128; (2 * $n + ($n + 1) / 2 + 1) / 2];
                type QuadU64 = [u64; 4 * $n];
                type QuadU128 = [u128; 2 * $n];
                type QuadBufferedU64 = [u64; 4 * $n + ($n + 1) / 2];
                type QuadBufferedU128 = [u128; (4 * $n + ($n + 1) / 2 + 1) / 2];
                #[inline]
                fn single_u64() -> Self::SingleU64 {
                    [0u64; $n]
                }
                #[inline]
                fn single_u128() -> Self::SingleU128 {
                    [0u128; ($n + 1) / 2]
                }
                #[inline]
                fn single_buffered_u64() -> Self::SingleBufferedU64 {
                    [0u64; $n + 2]
                }
                #[inline]
                fn single_buffered_u128() -> Self::SingleBufferedU128 {
                    [0u128; ($n + 3) / 2]
                }
                #[inline]
                fn double_u64() -> Self::DoubleU64 {
                    [0u64; 2 * $n]
                }
                #[inline]
                fn double_u128() -> Self::DoubleU128 {
                    [0u128; $n]
                }
                #[inline]
                fn double_buffered_u64() -> Self::DoubleBufferedU64 {
                    [0u64; 2 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn double_buffered_u128() -> Self::DoubleBufferedU128 {
                    [0u128; (2 * $n + ($n + 1) / 2 + 1) / 2]
                }
                #[inline]
                fn quad_u64() -> Self::QuadU64 {
                    [0u64; 4 * $n]
                }
                #[inline]
                fn quad_u128() -> Self::QuadU128 {
                    [0u128; 2 * $n]
                }
                #[inline]
                fn quad_buffered_u64() -> Self::QuadBufferedU64 {
                    [0u64; 4 * $n + ($n + 1) / 2]
                }
                #[inline]
                fn quad_buffered_u128() -> Self::QuadBufferedU128 {
                    [0u128; (4 * $n + ($n + 1) / 2 + 1) / 2]
                }
            }
        )+ };
    }
    // Decimal storage widths + the transcendental work widths (96..256).
    // `Int<512>` is the D1232 `exp`/hyperbolic large-result `Wexp`: D1232's
    // own work integer is already the widest storage-derived width (`Int<256>`
    // = `4·MAX_WORK_N`), so its `exp_fixed` overflow regime (the `2·w_ext`
    // squaring peak at a large argument and high directed-Ziv guard) cannot be
    // lifted to a *wider* tier-derived width — it needs a dedicated wider work
    // integer. `Int<512>` (32768 bits) holds that peak with comfortable margin
    // at every reachable working scale.
    exact_compute!(1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256, 512);
}

// ── exact-scratch-nightly: one blanket impl, exact per-N via const-expr ─
#[cfg(feature = "exact-scratch-nightly")]
mod imp {
    use super::{ComputeInt, Int};
    use crate::int::algos::support::limbs::n_limbs;

    impl<const N: usize> ComputeInt for Int<N>
    where
        [(); N]:,
        [(); (N + 1) / 2]:,
        [(); N + 2]:,
        [(); (N + 3) / 2]:,
        [(); 2 * N]:,
        [(); n_limbs(2, N)]:,
        [(); (n_limbs(2, N) + 1) / 2]:,
        [(); 4 * N]:,
        [(); n_limbs(4, N)]:,
        [(); (n_limbs(4, N) + 1) / 2]:,
    {
        type SingleU64 = [u64; N];
        type SingleU128 = [u128; (N + 1) / 2];
        type SingleBufferedU64 = [u64; N + 2];
        type SingleBufferedU128 = [u128; (N + 3) / 2];
        type DoubleU64 = [u64; 2 * N];
        type DoubleU128 = [u128; N];
        type DoubleBufferedU64 = [u64; n_limbs(2, N)];
        type DoubleBufferedU128 = [u128; (n_limbs(2, N) + 1) / 2];
        type QuadU64 = [u64; 4 * N];
        type QuadU128 = [u128; 2 * N];
        type QuadBufferedU64 = [u64; n_limbs(4, N)];
        type QuadBufferedU128 = [u128; (n_limbs(4, N) + 1) / 2];
        #[inline]
        fn single_u64() -> Self::SingleU64 {
            [0u64; N]
        }
        #[inline]
        fn single_u128() -> Self::SingleU128 {
            [0u128; (N + 1) / 2]
        }
        #[inline]
        fn single_buffered_u64() -> Self::SingleBufferedU64 {
            [0u64; N + 2]
        }
        #[inline]
        fn single_buffered_u128() -> Self::SingleBufferedU128 {
            [0u128; (N + 3) / 2]
        }
        #[inline]
        fn double_u64() -> Self::DoubleU64 {
            [0u64; 2 * N]
        }
        #[inline]
        fn double_u128() -> Self::DoubleU128 {
            [0u128; N]
        }
        #[inline]
        fn double_buffered_u64() -> Self::DoubleBufferedU64 {
            [0u64; n_limbs(2, N)]
        }
        #[inline]
        fn double_buffered_u128() -> Self::DoubleBufferedU128 {
            [0u128; (n_limbs(2, N) + 1) / 2]
        }
        #[inline]
        fn quad_u64() -> Self::QuadU64 {
            [0u64; 4 * N]
        }
        #[inline]
        fn quad_u128() -> Self::QuadU128 {
            [0u128; 2 * N]
        }
        #[inline]
        fn quad_buffered_u64() -> Self::QuadBufferedU64 {
            [0u64; n_limbs(4, N)]
        }
        #[inline]
        fn quad_buffered_u128() -> Self::QuadBufferedU128 {
            [0u128; (n_limbs(4, N) + 1) / 2]
        }
    }
}
