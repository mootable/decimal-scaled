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
//!   per monomorphisation) to [`crate::int::algos::limbs::add_assign_fixed`]
//!   (the ripple-carry kernel). Sub / neg still call the limb kernels
//!   directly; they will gain policy files in a later pass.
//! - **mul** — schoolbook at every width. The fixed-width types use the
//!   truncated low-`N` schoolbook product
//!   ([`crate::int::algos::limbs::mul_low_fixed`] /
//!   [`crate::int::algos::limbs::sqr_low_fixed`]);
//!   the slice dispatcher [`mul::dispatch`] additionally
//!   crosses over to Karatsuba at [`mul::KARATSUBA_THRESHOLD`] limbs, but
//!   the named integer widths in this crate stay in the schoolbook
//!   range.
//! - **÷ 10^SCALE** (decimal scale-narrowing) — this `(W, SCALE)`-keyed
//!   path is part of the **decimal** storage boundary, not the integer
//!   layer: D18 narrow tiers divide on hardware
//!   (`i128_divrem_by_u64_with_mode`), D38 takes the 2-word MG path, and
//!   the wide tiers route `mg_divide::div_wide_pow10_with`, which selects
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
//! - **isqrt / icbrt** — D38 has bespoke 256/384-bit kernels in
//!   `crate::algos::sqrt` / `crate::algos::cbrt`; the generic fixed-width
//!   types fall through to the shared limb isqrt / Brent–Zimmermann
//!   `root_int` ([`crate::int::algos::roots::isqrt_newton`] and
//!   `Uint::root_int`).
//!
//! All dispatchers follow the canonical [`Select`] / `select` /
//! exhaustive-`match algo` policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"). The add dispatcher is a pure `ByAlgorithm`
//! `const fn` (the const block folds, no runtime branch). The divmod and
//! mul dispatchers key on the operands' *runtime* shape (effective limb
//! count / operand length) and are `Select::ByValue`-style value matchers:
//! the const layer settles on "the shape decides", the matcher classifies,
//! and the dispatcher does an exhaustive `match algo` to the pure engines /
//! kernels in [`crate::int::algos::div`] / [`crate::int::algos::limbs`].
//! The benched crossover thresholds ([`div_rem::BZ_THRESHOLD`],
//! [`mul::KARATSUBA_THRESHOLD`]) are policy DATA in those files, not magic
//! numbers in the kernels.

/// Addition policy: default-delegating ripple-carry matcher for `Int<N>`.
pub(crate) mod add;
pub(crate) mod div_rem;
pub(crate) mod mul;
