//! Division policy — the divisor-shape algorithm matcher.
//!
//! The integer layer carries **no `SCALE`**, and the divmod choice does
//! not key on the const limb count `N` — it keys on the **runtime shape**
//! of the operands (their effective limb counts after stripping leading
//! zeros). That makes division a [`Select::ByValue`] case in the
//! canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure" and the value-matcher tier): the const layer settles on
//! "the value decides", the value-matcher classifies the divisor/dividend
//! shape and returns an [`Algorithm`] tag, and the dispatcher does an
//! **exhaustive** `match algo` to a pure engine in
//! [`crate::int::algos::div`].
//!
//! The engines stay pure — each takes an already-chosen algorithm. This
//! file owns the *choice*: the benched crossover thresholds
//! ([`BZ_THRESHOLD`]) are policy DATA here, not magic numbers buried in a
//! kernel.

use crate::int::algos::div::div_burnikel_ziegler_with_knuth::div_burnikel_ziegler_with_knuth;
use crate::int::algos::div::div_knuth::div_knuth;
use crate::int::algos::div::div_rem::div_rem;
use crate::int::algos::div::div_rem_schoolbook::div_rem_schoolbook;

// ── 1. the real division engines — NAMED, no `Default` ────────────────

/// The division engines the divisor-shape matcher chooses between.
/// Variants are the CamelCase of each engine fn's name minus the `div_`
/// function prefix (`div_knuth` → `Knuth`, …) — strict 1:1 with the
/// engine fns in [`crate::int::algos::div`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`div_rem`] — the `const fn` single-limb hardware fast path
    /// (`div_rem`'s Fast B is one hardware `u128 / u64` per dividend
    /// limb — already optimal for a single-limb divisor).
    Rem,
    /// [`div_knuth`] — Knuth Algorithm D at base 2⁶⁴.
    Knuth,
    /// [`div_burnikel_ziegler_with_knuth`] — Burnikel–Ziegler outer
    /// chunking, recursing to Knuth as its base case.
    BurnikelZieglerWithKnuth,
    /// [`div_rem_schoolbook`] — binary shift-subtract long division,
    /// the naive reference baseline. Registered but unrouted: `select`
    /// never returns this variant; it exists for unit-test reachability
    /// and future routing experiments. `#[allow(dead_code)]` suppresses
    /// the compiler warning.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled engine, or "the (runtime) shape decides". Division always
/// returns `ByValue`: the choice is fully determined by the operands'
/// effective limb counts, known only at run time. `ByAlgorithm` is part
/// of the canonical shape for uniformity across functions.
#[derive(Clone, Copy)]
enum Select {
    #[allow(dead_code)]
    ByAlgorithm(Algorithm),
    /// Classifier over the operands' effective limb counts `(den_n,
    /// num_top)` (leading zeros stripped) → the chosen engine.
    ByShape(fn(usize, usize) -> Algorithm),
}

// ── policy data: the benched crossover threshold ──────────────────────

/// Burnikel–Ziegler engagement threshold, in u64 limbs: a divisor of at
/// least this many effective limbs whose dividend is at least twice as
/// wide takes Burnikel–Ziegler; otherwise Knuth. Doubled from the legacy
/// u128 path's `8` to match the u64 base. **Policy data** — the kernels
/// take an already-chosen engine and never see this number.
pub(crate) const BZ_THRESHOLD: usize = 16;

// ── 3. the matcher: keyed on the runtime divisor shape ────────────────

/// Pick the division engine for the operands' effective limb counts.
/// Total over the shape; reproduces the exact byte-for-byte routing of
/// the historic inline dispatcher:
///
/// * single-limb divisor → [`Algorithm::Rem`] (the const hardware path,
///   covers every `10^scale` with `scale ≤ 19`);
/// * `den_n ≥ BZ_THRESHOLD` and `num_top ≥ 2·den_n` → Burnikel–Ziegler;
/// * everything else → Knuth.
const fn select() -> Select {
    Select::ByShape(|den_n: usize, num_top: usize| {
        if den_n == 1 {
            Algorithm::Rem
        } else if den_n >= BZ_THRESHOLD && num_top >= 2 * den_n {
            Algorithm::BurnikelZieglerWithKnuth
        } else {
            Algorithm::Knuth
        }
    })
}

// ── 4. the dispatcher: classify the shape, then dispatch ──────────────

/// Runtime divide dispatcher at u64 base — the single entry every
/// multi-limb divide flows through. Strips leading zeros to get the
/// effective shape, asks the matcher which engine, then dispatches.
///
/// `quot` / `rem` are written by the chosen engine; the divisor must be
/// non-zero.
pub(crate) fn dispatch(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "dispatch: divide by zero");

    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }

    let algo = match const { select() } {
        Select::ByAlgorithm(a) => a,
        Select::ByShape(f) => f(n, top),
    };
    match algo {
        Algorithm::Rem => div_rem(num, den, quot, rem),
        Algorithm::Knuth => div_knuth(num, den, quot, rem),
        Algorithm::BurnikelZieglerWithKnuth => {
            div_burnikel_ziegler_with_knuth(num, den, quot, rem)
        }
        Algorithm::Schoolbook => div_rem_schoolbook(num, den, quot, rem),
    }
}
