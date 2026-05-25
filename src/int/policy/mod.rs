//! Per-width / limb-count algorithm-selection dispatch.
//!
//! The integer layer's counterpart to the decimal `policy/` bucket:
//! the place where operand width / limb count selects which kernel
//! runs. Unlike the decimal families, the integer types carry **no
//! `SCALE`** — `Int<N>` / `Uint<N>` and the named `Int*` / `Uint*`
//! aliases are pure fixed-width integers — so the dispatch key is the
//! limb count `N` (a compile-time const at every monomorphisation) and,
//! for division, the **runtime shape** of the operands (effective limb
//! count after stripping leading zeros). There is therefore no
//! `match (W, SCALE)` table to write for this layer: the decimal
//! `match (W, SCALE)` form has no `SCALE` axis here, and the divmod
//! selection cannot be a compile-time match because it keys on a
//! property only known at run time.
//!
//! The per-family dispatch as it actually ships today:
//!
//! - **add** — width-keyed only, one algorithm at every `N`. Routes
//!   through [`add::dispatch`] (a `const fn` policy so the block folds
//!   per monomorphisation) to [`crate::int::algos::support::limbs::add_assign_fixed`]
//!   (the ripple-carry kernel).
//! - **sub** — width-keyed only, one algorithm at every `N`. Routes
//!   through [`sub::dispatch`] (`const fn`) to
//!   [`crate::int::algos::support::limbs::sub_assign_fixed`] (the ripple-borrow kernel).
//! - **neg** — unary, one algorithm at every `N`. Routes through
//!   [`neg::dispatch`] (`const fn`) to the two's-complement bitwise-NOT-
//!   plus-one kernel.
//! - **cmp** — width-keyed only, one algorithm at every `N`. Routes the
//!   signed same-width total-order comparison through [`cmp::dispatch`]
//!   (`const fn`) to the limbwise signed-comparison kernel.
//! - **eq** — width-keyed only, one algorithm at every `N`. Routes the
//!   equality test through [`eq::dispatch`] (`const fn`) to the limbwise
//!   equality kernel.
//! - **rem** — derives the remainder via the division policy. Routes the
//!   `Rem` operator through [`rem::dispatch`] (NOT `const fn`, because
//!   it delegates to [`div_rem::dispatch`] which is runtime-shape keyed).
//!   `Int<N>::wrapping_rem` (which IS `const fn`) calls the const
//!   [`crate::int::algos::div::div_rem`] algo directly.
//! - **mul** — schoolbook at every width. The fixed-width types use the
//!   truncated low-`N` schoolbook product
//!   ([`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] /
//!   [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`]);
//!   the slice dispatcher [`mul::dispatch`] additionally
//!   crosses over to Karatsuba at the benched 48-limb threshold, so
//!   D924 (48) / D1232 (64) storage products and wider cross-scale
//!   multiplies take Karatsuba while the narrower widths stay schoolbook.
//! - **÷ 10^SCALE** (decimal scale-narrowing) — this `(W, SCALE)`-keyed
//!   path is part of the **decimal** storage boundary, not the integer
//!   layer: D18 narrow tiers divide on hardware
//!   (`i128_divrem_by_u64_with_mode`), D38 takes the 2-word MG path, and
//!   the wide tiers route `mg_divide::div_wide_pow10`, which selects
//!   MG-single (`SCALE ≤ 19`, divisor fits one word) vs MG-lifted
//!   (wider) internally. It lives in `crate::algos::support::mg_divide` /
//!   `crate::macros::arithmetic`; the integer layer exposes only the raw
//!   divmod the wide path builds on.
//! - **divmod** — divisor-shape keyed at run time
//!   ([`div_rem::dispatch`]): single-limb divisor (incl. every
//!   `10^scale`, `scale ≤ 19`) takes the hardware fast path; a divisor of
//!   `n ≥ 16` limbs whose numerator top ≥ `2·n` takes Burnikel–Ziegler;
//!   everything else takes Knuth Algorithm D. The const-evaluable
//!   `wrapping_div` / `wrapping_rem` stay on the `const fn`
//!   [`div_rem::div_rem`] so they can run at compile time.
//! - **isqrt** — `N ∈ {1, 2}` takes the hardware native path (`u64::isqrt`/
//!   `u128::isqrt`); `N >= 3` takes the Newton limb kernel
//!   ([`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`]). Routes through
//!   [`isqrt::dispatch`] (const `N`-keyed; not `const fn`).
//! - **icbrt** — same shape as isqrt: `N ∈ {1, 2}` routes through the
//!   Newton kernel at small width; `N >= 3` takes the full Newton limb
//!   iteration ([`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`]). Routes through
//!   [`icbrt::dispatch`] (not `const fn`).
//! - **pow** — binary square-and-multiply at every `N`. Routes through
//!   [`pow::dispatch`] (`const fn`).
//! - **sqr** — half-product squaring kernel at every `N`. Routes through
//!   [`sqr::dispatch`] (`const fn`).
//! - **cube** — sqr-then-multiply at every `N`. Routes through
//!   [`cube::dispatch`] (`const fn`).
//! - **sum_sq** — `a^2 + b^2` (schoolbook) at every `N`. Routes through
//!   [`sum_sq::dispatch`]; the hypot kernel shares its radicand former.
//! - **hypot** — `round(sqrt(a^2 + b^2))` at every `N`. Routes through
//!   [`hypot::dispatch`] to the Pythagoras kernel (forms the `sum_sq`
//!   radicand, then the Newton slice `isqrt` + round step).
//!
//! All dispatchers follow the canonical [`Select`] / `select` /
//! exhaustive-`match algo` policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"). The add / sub / neg / cmp / eq dispatchers
//! are pure `ByAlgorithm` `const fn` (the const block folds, no runtime
//! branch). The divmod and mul dispatchers key on the operands' *runtime*
//! shape (effective limb count / operand length) and are
//! `Select::ByValue`-style value matchers: the const layer settles on
//! "the shape decides", the matcher classifies, and the dispatcher does an
//! exhaustive `match algo` to the pure engines / kernels in
//! [`crate::int::algos::div`] / [`crate::int::algos::support::limbs`]. The rem
//! dispatcher is non-const and delegates to the divmod dispatcher.
//! The benched crossover thresholds ([`div_rem::BZ_THRESHOLD`] and `mul`'s
//! file-private Karatsuba threshold) are policy DATA in those files, not
//! magic numbers in the kernels.

/// Addition policy: default-delegating ripple-carry matcher for `Int<N>`.
pub(crate) mod add;
/// Signed comparison policy: default-delegating limbwise matcher for `Int<N>`.
pub(crate) mod cmp;
/// Integer cube policy: sqr-then-multiply matcher for `Uint<N>`.
pub(crate) mod cube;
/// Division/remainder policy: divisor-shape algorithm matcher for `Int<N>`.
pub(crate) mod div_rem;
/// Equality policy: default-delegating limbwise matcher for `Int<N>`.
pub(crate) mod eq;
/// Integer hypotenuse policy: isqrt-vs-schoolbook matcher for `Int<N>`.
pub(crate) mod hypot;
/// Integer cube-root policy: native-vs-Newton matcher for `Uint<N>`.
pub(crate) mod icbrt;
/// Integer square-root policy: native-vs-Newton matcher for `Uint<N>`.
pub(crate) mod isqrt;
/// Multiply policy: schoolbook-vs-Karatsuba algorithm matcher.
pub(crate) mod mul;
/// Negate policy: default-delegating two's-complement matcher for `Int<N>`.
pub(crate) mod neg;
/// Integer exponentiation policy: square-and-multiply matcher for `Uint<N>`.
pub(crate) mod pow;
/// Remainder policy: default-delegating via-div_rem matcher for `Int<N>`.
pub(crate) mod rem;
/// Integer squaring policy: half-product via-mul matcher for `Uint<N>`.
pub(crate) mod sqr;
/// Subtract policy: default-delegating ripple-borrow matcher for `Int<N>`.
pub(crate) mod sub;
/// Integer sum-of-squares policy: `a^2 + b^2` schoolbook matcher for `Int<N>`.
pub(crate) mod sum_sq;
