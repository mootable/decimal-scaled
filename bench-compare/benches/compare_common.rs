//! Shared harness for the per-width `compare_d{N}` bench family.
//!
//! Side-by-side comparison of two `decimal-scaled` versions across the
//! whole public function surface, split one bench target per width so
//! each width runs as its own parallel CI job (wall time ≈ the slowest
//! single width, instead of one >90 min sweep over every width).
//!
//! `branch` = the branch under test (the `decimal_scaled` path dep);
//! `prod` = the latest published release (the baseline). Both run the
//! same functions at the same SCALE, with operands constructed via
//! `FromStr` so the harness depends only on the public surface common
//! to both versions. Each per-width target produces both columns for
//! its width; pair `<fn>/<W>/branch` against `<fn>/<W>/prod`.
//!
//! Each per-width file declares `#[macro_use] mod compare_common;` so
//! the macros are crate-local-visible without re-exporting.
//!
//! Coverage (the public surface present in BOTH prod and the branch — i.e.
//! prod's surface, of which the branch is a superset). `prod` is whatever
//! `bench-compare/Cargo.toml` pins, currently 0.5.1; name the version there,
//! not here, so this list cannot drift out of date behind the pin:
//!   * arith:               add, sub, mul, div, rem, neg
//!   * roots:               sqrt, cbrt
//!   * transcendental unary: exp, ln, log2, log10, sin, cos, tan, asin, acos,
//!                           atan, sinh, cosh, tanh, asinh, acosh, atanh,
//!                           to_degrees, to_radians
//!   * binary / other:      powf, log, hypot
//!
//! `log2` / `log10` are public API on both sides and were previously unbenched
//! anywhere in the sweep. `ln_nd` (VARIANT A below) is a second `ln` row at a
//! non-degenerate argument; it is diagnostic, not part of the prod surface
//! contract, and can be dropped without touching anything else.
//!
//! # Operand families — `op` and `op@<variant>`
//!
//! Every cell of this sweep measures exactly ONE input, so one unlucky operand
//! makes a whole row structurally blind — and that has repeatedly hidden real
//! defects. `ln` at `2.0` is an exact power of two, so its range reduction
//! collapses and the row times a short-circuit at every scale. `powf`'s
//! exponent spells `1` at SCALE 0, so that column times the integer-power pin
//! instead of the composition. `hypot` on the 3-4-5 triple only ever takes the
//! exact-hypotenuse path. A row that can reach only one branch of its kernel
//! cannot see a regression in any of the others, at any width or scale.
//!
//! So an op whose kernel BRANCHES on the value gets a small family: the base
//! row `op` — left exactly as it was, so its published figure and its history
//! stay comparable — plus one or more `op@<variant>` rows named for the path
//! they reach. `@hard` is the expensive branch; a more specific name is used
//! where it says more (`@near1`, `@int`). Each family is declared beside the
//! operands it uses in [`funcs!`], with the branch it targets and the source
//! that branches on it.
//!
//! An op whose cost does NOT vary with the value gets NO family — see "Ops
//! deliberately left as a single row" in [`funcs!`] for why each was left
//! alone. Rows cost sweep time linearly, so a family whose members all measure
//! the same thing is worse than the single row it replaced.
//!
//! `hypot` is benched across the full width set via the integer-only
//! correctly-rounded form — the one method whose NAME differs between the
//! two sides, so `scale_funcs!` passes the ident per side:
//!
//!   * branch — `hypot`. The API reduction dropped the `_strict` suffix, so
//!     the bare name IS the correctly-rounded form at every width.
//!   * prod   — `hypot_strict`. The pinned baseline predates that rename and
//!     exposes no plain `hypot` on the wide tiers at all (its `wide_roots`
//!     emits `hypot_strict`/`hypot_strict_with` only); the plain dispatcher
//!     exists on `D38` alone, where it delegates straight to `hypot_strict`.
//!
//! Both idents therefore name the SAME kernel on their own side, which is what
//! keeps the branch÷prod ratio a like-for-like comparison. Benching it at every
//! width closes the prior coverage hole where `hypot` was benched at `D38`
//! alone, leaving a regression at any wider width invisible.
//!
//! Each function is exercised through its DEFAULT public method (e.g.
//! `x.sqrt()`, `x.powf(y)`). On the branch those bare names are the
//! correctly-rounded kernels unconditionally. On prod they resolve to the same
//! kernels because the baseline is built WITH `strict` and WITHOUT `fast` —
//! its bare names are `#[cfg(all(feature = "strict", not(feature = "fast")))]`,
//! so dropping `strict` from the `prod` dep would not slow the baseline down,
//! it would delete the entire surface and fail the build. Operands are chosen
//! to be domain-valid for every function (e.g. `asin`/`acos` in [-1,1],
//! `ln`/`log` positive, `acosh` >= 1, `atanh` in (-1,1)) so nothing
//! panics or returns a degenerate result.
//!
//! `D9` is intentionally absent — it was removed in 0.5.0, so it has no
//! counterpart on the `branch` side.

/// Pick the operand string for the current SCALE: a value with a fractional
/// part can only be parsed when `SCALE >= 1` (more fractional digits than
/// `SCALE` is `ParseError::OverlongFractional`), so at `SCALE == 0` — a tier
/// in every wide width's scale set — fall back to the integer form. Both
/// forms are still domain-valid for every function below.
///
/// `$scale:literal` is a compile-time constant, so this `if` const-folds to
/// the chosen branch — no runtime cost in the benched closures (operands are
/// built once, outside `bn.iter`).
#[macro_export]
macro_rules! op_str {
    ($scale:literal, $frac:literal, $int:literal) => {
        if $scale == 0 { $int } else { $frac }
    };
}

/// Bench every covered function for one `Copy` decimal type, under the
/// given `$side` label, grouping each function into its own Criterion group
/// `<fn>_<W>_s<scale>` (branch/prod as the two rows).
///
/// `$hypot` is the side's spelling of the correctly-rounded hypotenuse
/// (`hypot` on the branch, `hypot_strict` on the pinned baseline — see the
/// module docs). Every other function is spelled identically on both sides.
///
/// Operands (all domain-valid for every function below; the scale-0 integer
/// fallback in parens via `op_str!`). EVERY operand — and every arithmetic
/// RESULT — is kept to a single integer digit (|v| < 10), because the highest
/// scale in each tier's set is `S-1`, leaving exactly ONE integer digit; a
/// 2-digit operand or product (e.g. the old `67.89`, `10.0`, or `2 * 67.89`)
/// is `ParseError::OutOfRange` / an overflow there. The op set is identical
/// across scales, so the narrow-integer-room S-1 cell dictates the choice.
///   * `x  = 2.0` (→ `2`)   — positive (sqrt/cbrt/ln/log arg/powf base),
///                            >= 1 for `acosh`; results (sqrt≈1.41, ln≈0.69,
///                            exp/cosh of `s` ≈1.1–1.7, acosh≈1.32) all < 10.
///   * `s  = 0.1` (→ `0`)   — in [-1, 1] and (-1, 1): `asin`/`acos`/`atanh`;
///                            a benign small argument for the periodic and
///                            hyperbolic functions and `to_*` (0 stays in
///                            every domain at scale 0). Kept small enough that
///                            `to_degrees(0.1) ≈ 5.73 < 10` survives the S-1
///                            single-integer-digit cell (`to_degrees(0.5)≈28.6`
///                            would overflow it).
///   * `b  = 3.5` (→ `3`)   — second arithmetic operand; `x±b`, `x*b`(=7),
///                            `x/b`, `x%b` all stay |·| < 10.
///   * `e  = 1.5` (→ `1`)   — powf exponent; `x^e = 2^1.5 ≈ 2.83 < 10`.
///   * `base = 7.0` (→ `7`) — log base (> 0, ≠ 1); `log(2, 7)` valid. Also the
///                            `ln_nd` argument (VARIANT A): unlike `x = 2.0` it
///                            is not a power of two, so it does not collapse
///                            the log range reduction to `m = 1`.
///   * `sw = 0.1` (→ `2`)   — VARIANT B: `s` with a NON-degenerate scale-0
///                            spelling, for the wide-domain small-argument
///                            functions. Identical to `s` at every scale >= 1,
///                            so no cell except s0 moves and the S-1 analysis
///                            above is untouched — the integer spelling is
///                            reached ONLY at SCALE 0, where every tier has
///                            ample integer room (`to_degrees(2) ≈ 114.6` fits
///                            at s0; it is the S-1 cell, on `0.1`, that the
///                            single-integer-digit rule binds).
///
/// The FAMILY operands (`nd1`, `nd2`, `n1x`, `n1b`, `eh`, `ang`, `tp`, `n1`,
/// `atb`, `big`, `ac1`, `pint`, `phard`) are declared and justified in the
/// body below, each next to the branch it exists to reach — see "Operand
/// families" in the module docs. They obey the same S-1 rule as the operands
/// above: every one of them, and every result they produce, was checked to
/// stay `|·| < 10` at the S-1 cell and inside `D18<0>`'s 18 digits at SCALE 0.
#[macro_export]
macro_rules! funcs {
    ($c:expr, $w:literal, $scale:literal, $side:literal, $ty:ty, $hypot:ident) => {{
        use ::std::hint::black_box;
        let x: $ty = $crate::op_str!($scale, "2.0", "2").parse().unwrap();
        let s: $ty = $crate::op_str!($scale, "0.1", "0").parse().unwrap();
        let b: $ty = $crate::op_str!($scale, "3.5", "3").parse().unwrap();
        let e: $ty = $crate::op_str!($scale, "1.5", "1").parse().unwrap(); // powf exponent
        let ten: $ty = $crate::op_str!($scale, "7.0", "7").parse().unwrap(); // log base
        // ── VARIANT B (revert = change this "2" back to "0") ─────────────
        // `s` above spells `0` at SCALE 0, so the whole s0 column measures
        // `f(0)` — which every small-argument function short-circuits (the s0
        // cells run 1-4 ns against thousands at s9). `sw` is the SAME operand
        // at every scale >= 1 ("0.1", so no non-s0 cell moves and no S-1
        // bound changes) and a non-degenerate `2` at SCALE 0.
        //
        // Only the wide-domain functions can take it. `asin`/`acos` are capped
        // at |v| <= 1, and `atanh` needs |v| < 1 STRICTLY — the sole integer in
        // that open interval is 0 — so those three keep `s` and their s0 cell
        // stays irreducibly degenerate at SCALE 0. That is forced by the
        // domains, not a choice.
        let sw: $ty = $crate::op_str!($scale, "0.1", "2").parse().unwrap();
        // ── end VARIANT B declaration ───────────────────────────────────
        // `hypot` operands: the 3-4-5 Pythagorean triple — both legs and the
        // result (5) stay single-integer-digit (|·| < 10), surviving the S-1 cell.
        let c3: $ty = $crate::op_str!($scale, "3.0", "3").parse().unwrap();
        let d4: $ty = $crate::op_str!($scale, "4.0", "4").parse().unwrap();

        // ── FAMILY OPERANDS (see "Operand families" in the module docs) ──
        //
        // Every operand below obeys the SAME two rules as the ones above, and
        // both were checked numerically for every operand AND every result:
        //   * S-1: `|v| < 10` and `|f(v)| < 10`, the one integer digit the top
        //     benched scale leaves;
        //   * SCALE 0: a DELIBERATE integer spelling whose result still fits
        //     the narrowest tier at s0 (`D18<0>`, 18 significant digits).
        // Fraction digits are held to <= 2 so every spelling parses at the
        // smallest benched non-zero scale, 4 (D18's [0, 4, 9, 13, 17]); the
        // bound is 4, so there is room but none is spent without reason.
        //
        // ── the NON-DYADIC pair, `nd1` / `nd2` ──────────────────────────
        //
        // The log family range-reduces on the BINARY mantissa `v = 2^k · m`,
        // `m ∈ [1, 2)`, and has TWO degeneracies — both properties of the
        // VALUE, not of the stored raw, hence scale-invariant, hence fixable
        // with one literal that covers every scale and width:
        //   * Trap 1, `m == 1` — an exact power of two. Both `ln` kernels
        //     return `k · ln2` from a one-word product and NEITHER the artanh
        //     series nor the Brent sqrt reduction runs. `x = 2.0` — the `ln`,
        //     `log2`, `log10`, `log` and `powf` operand above — is exactly
        //     this, which is why the `ln` row reads flat in scale.
        //   * Trap 2, `t == 0` — `m` an exact multiple of `1/128`, so Tang's
        //     residual is EXACTLY zero and its artanh series breaks on the
        //     first iteration. `ten = 7.0 = 2² · 1.75` and `1.75 = 1 + 96/128`
        //     EXACTLY, so the `log` base — and `ln_nd`'s argument — is this
        //     trap at every width the policy routes Tang, i.e. every wide
        //     tier. `b = 3.5`, `e = 1.5` and `c3 = 3.0` are caught too: ANY
        //     value whose binary mantissa terminates within 7 fraction bits
        //     is. `ln_nd` therefore defeats Trap 1 and lands on Trap 2; it
        //     measures the narrow (Series) tiers, where only Trap 1 exists,
        //     and a table read at every Tang tier.
        // Both traps require the value to be DYADIC (denominator a power of
        // two in lowest terms). `2.3 = 23/10` and `3.7 = 37/10` keep a factor
        // of 5, so their binary expansions never terminate and they defeat
        // BOTH traps at EVERY scale, width and `k`. At SCALE 0 there is no
        // `10^SCALE` to lean on and the rule becomes "odd and >= 257" (odd
        // forces `k <= 7` on the boundary test, so `>= 257` forces `k >= 8`);
        // 259 and 263 satisfy it, and 259 also keeps `ln(259) ≈ 5.56 < 10`.
        // The analysis is `benches/micro/ln_wide_series_tang_ab.rs`, which
        // states the same contract on the raw.
        let nd1: $ty = $crate::op_str!($scale, "2.3", "259").parse().unwrap();
        let nd2: $ty = $crate::op_str!($scale, "3.7", "263").parse().unwrap();
        // `log@near1` — the base within 0.1 of 1 that routes the SECOND `log`
        // algorithm. `policy::log::select` is `ByValue` on the BASE and
        // `log_near_one_base` classifies it by `k = ceil(-log10 |b - 1|)`:
        // `k == 0` is `LnDivide` (every base this sweep has ever benched),
        // `k > 0` is `LnDivideConditioned` — a different kernel at guard
        // `30 + k`. `1.05` gives `k = 2`. The RESULT is what binds the pair:
        // `log(x, b) = ln x / ln b` with `ln b ≈ 0.0488`, so `x` must also sit
        // near 1 to keep the quotient under the S-1 single integer digit —
        // `ln(1.4)/ln(1.05) ≈ 6.90 < 10`. At SCALE 0 the only integer within
        // 0.1 of 1 is 1 itself, and base 1 is not a valid logarithm base, so
        // this row's s0 cell CANNOT reach the conditioned arm; it falls back
        // to a non-degenerate ordinary pair (269/271, both odd and >= 257) so
        // it still measures `LnDivide` properly rather than a short-circuit.
        let n1x: $ty = $crate::op_str!($scale, "1.4", "269").parse().unwrap();
        let n1b: $ty = $crate::op_str!($scale, "1.05", "271").parse().unwrap();
        // `exp@hard` and the hyperbolics' `@hard` — a LARGE argument, where
        // `exp`'s adaptive `r/2^n` reduction pays for real halvings and
        // squarings instead of returning after a couple of Taylor terms.
        // `exp` bounds the family: the S-1 cell needs `e^v < 10`, i.e.
        // `v < ln 10 ≈ 2.3026`, so `2.2` (→ `e^2.2 ≈ 9.03`) is the largest
        // round value that fits. At SCALE 0 the bound is `e^v < 10^18`, and
        // `9` is chosen well inside it (`e^9 ≈ 8103`) so `tanh` does not
        // saturate to exactly 1 and stop measuring anything.
        let eh: $ty = $crate::op_str!($scale, "2.2", "9").parse().unwrap();
        // `sin@hard` / `cos@hard` — an angle that actually needs the mod-τ
        // range reduction AND lands near π/4, where the Taylor series runs
        // its longest. `6.9 - 2π ≈ 0.6168` and `7 - 2π ≈ 0.7168`, both inside
        // (0, π/4) so no quadrant fold, and both far from the `s = 0.1` the
        // base rows use — whose square is 0.01, so each term drops two
        // decades and the series ends almost immediately. Results are in
        // [-1, 1], so the S-1 bound is never in question for these two.
        let ang: $ty = $crate::op_str!($scale, "6.9", "7").parse().unwrap();
        // `tan@hard` — the NEAR-POLE guard lift. `tan` is `sin(r)/cos(r)`, and
        // `algos::trig::near_pole_tan` adds roughly `log10(|tan|)` guard
        // digits and RECOMPUTES (a second `sin_cos_fixed` + `div` at the
        // lifted scale) whenever the quotient exceeds magnitude 1; at
        // `s = 0.1` the lift is skipped entirely. `tan(1.47) ≈ 9.887` is the
        // largest such value the S-1 single integer digit admits. At SCALE 0,
        // `11` sits 0.0044 from `7π/2`, giving `tan(11) ≈ -226` — the same
        // branch, harder.
        let tp: $ty = $crate::op_str!($scale, "1.47", "11").parse().unwrap();
        // `atanh@hard` — just inside the domain wall, where the gap `1 - |x|`
        // is small and the composition is at its most conditioned.
        // `atanh(0.99) ≈ 2.647 < 10` bounds the choice. MEASURED (run
        // 33938890210): 2.91x at D18 and 2.94x at D38, and 1.01-1.03x at every
        // wider tier — so this row earns its place at the NARROW tiers and is
        // flat above them. (The same operand was tried for `asin`/`acos` and
        // was flat everywhere; see the note at those rows.)
        // SCALE 0 is IRREDUCIBLE here, exactly as it is for the base row:
        // `atanh` needs `|v| < 1` STRICTLY, so the only integer available is 0
        // and this row's s0 cell repeats the base row's. Forced by the domain,
        // not chosen — and the measurement above confirms it (1.00x at s0).
        let n1: $ty = $crate::op_str!($scale, "0.99", "0").parse().unwrap();
        // `atan@hard` — `atan_fixed` folds `|x| > 1` through `π/2 - atan(1/x)`
        // and THEN halves via `atan(x) = 2·atan(x/(1+√(1+x²)))` while the
        // argument is at or above ~0.2, each halving costing a wide sqrt +
        // divide + multiply. `3.3` folds to `0.303` and `3` to `0.333`, both
        // above the threshold, so this row pays fold AND halvings; the base
        // row's `0.1` pays neither. (A large argument alone is NOT enough:
        // `9.3` folds to `0.108`, below the threshold, and would have skipped
        // the halvings — nearly the base row's work plus one divide.)
        let atb: $ty = $crate::op_str!($scale, "3.3", "3").parse().unwrap();
        // `asinh@hard` — `asinh_series_composition` splits at `|x| = 1`: below
        // it is one sqrt and ONE `ln_series`, at or above it is a reciprocal,
        // a sqrt and TWO `ln_series` calls. `9.3` is the two-`ln` branch.
        let big: $ty = $crate::op_str!($scale, "9.3", "9").parse().unwrap();
        // `acosh@near1` — `acosh_ln_composition` splits at exactly `x = 2`:
        // `x >= 2` factors the radicand and takes TWO `ln` calls, `x < 2`
        // takes the `log1p` GAP form `acosh(1+t) = log1p(t + √(t(t+2)))`. The
        // base row's `x = 2.0` sits ON the boundary and so measures the
        // two-`ln` branch; `1.01` is the only way into the gap form. At
        // SCALE 0 the only integer below 2 in the domain is 1, where the gap
        // is 0 and every internal series short-circuits — measuring nothing —
        // so the s0 spelling stays `2` and repeats the base row's branch
        // rather than timing an empty one.
        let ac1: $ty = $crate::op_str!($scale, "1.01", "2").parse().unwrap();
        // `powf@int` — the EXACT integer-power pin
        // (`algos::pow::powi_exact`), which answers an integer exponent by
        // binary exponentiation and never forms `exp(y · ln x)` at all. The
        // base `powf` row reaches this pin ONLY at SCALE 0, where `e = 1.5`
        // spells `1`; this row reaches it at EVERY scale, deliberately, so
        // the pin has a row of its own instead of silently owning one column
        // of another. `x^3 = 8 < 10` survives S-1.
        let pint: $ty = $crate::op_str!($scale, "3.0", "3").parse().unwrap();
        // `powf@hard` — the full composition: an exponent that is neither an
        // integer (the pin above) nor exactly 0.5 (the algebraic `√x` pin),
        // over the non-dyadic base `nd1` so the inner `ln` is not Trap 1
        // either. `2.3^1.3 ≈ 2.95 < 10`. At SCALE 0 an exponent is an integer
        // BY DEFINITION, so this row necessarily falls onto the pin there —
        // irreducible, and now explained rather than accidental.
        let phard: $ty = $crate::op_str!($scale, "1.3", "2").parse().unwrap();

        // ── OPS DELIBERATELY LEFT AS A SINGLE ROW ───────────────────────
        //
        // A family is only worth its sweep time where the KERNEL branches on
        // the value. These ten do not, so they keep one row each:
        //
        //   add, sub, neg — fixed-width limb ripple over the tier's `N`
        //     limbs. The work is the width, not the value.
        //   mul — `policy::mul` says it plainly: "mul has no value split, so
        //     `ByValue` is never returned". Its two internal paths (product
        //     fits `Int<N>` vs widening) are chosen by whether `a_raw·b_raw`
        //     overflows `N` limbs, and with every operand held under 10 by
        //     the S-1 rule that is decided by SCALE — which this sweep
        //     ALREADY fans out over. The fast path is the s0 column and the
        //     widening path is everything above it, so the split is measured;
        //     a value family would only re-measure the same two paths.
        //   div, rem — the divisor's LIMB COUNT drives the Knuth engine, and
        //     at a fixed scale a small-looking divisor still fills its raw
        //     with trailing zeros. Same reasoning as `mul`: scale decides.
        //   sqrt, cbrt — Newton on a radicand whose bit length is set by
        //     SCALE; there is no perfect-square/cube short-circuit (the
        //     `diff_nonzero` test in `algos::sqrt::sqrt_newton` is the round
        //     step, not an early exit), and the seed comes from the shared
        //     `f64` bootstrap, so the iteration count does not move between
        //     `2.0` and `9.9`.
        //   to_degrees, to_radians — one multiply by a constant.
        //
        // If a future bbc surface shows any of these moving with the value
        // after all, the family goes in then; the reason it is absent is
        // recorded here so the next reader does not have to re-derive it.

        // ── arithmetic ──
        $crate::bench_one!($c, "add", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) + black_box(b))
        });
        $crate::bench_one!($c, "sub", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) - black_box(b))
        });
        $crate::bench_one!($c, "mul", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) * black_box(b))
        });
        $crate::bench_one!($c, "div", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) / black_box(b))
        });
        $crate::bench_one!($c, "rem", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) % black_box(b))
        });
        $crate::bench_one!($c, "neg", $w, $scale, $side, |bn| bn.iter(|| -black_box(x)));

        // ── roots ──
        $crate::bench_one!($c, "sqrt", $w, $scale, $side, |bn| bn.iter(|| black_box(x).sqrt()));
        $crate::bench_one!($c, "cbrt", $w, $scale, $side, |bn| bn.iter(|| black_box(x).cbrt()));

        // ── transcendental, single argument ──
        $crate::bench_one!($c, "exp", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).exp()));
        // FAMILY: the deep-reduction end of `exp`. `sw` is a small argument
        // that exits after a couple of Taylor terms; `eh` forces the adaptive
        // `r/2^n` halvings and the squarings that reassemble them.
        $crate::bench_one!($c, "exp@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(eh).exp())
        });
        $crate::bench_one!($c, "ln", $w, $scale, $side, |bn| bn.iter(|| black_box(x).ln()));
        // FAMILY: the only `ln` row that runs the artanh series. `ln` above is
        // Trap 1 (exact power of two) and `ln_nd` below is Trap 2 (an exact
        // Tang table boundary), so between them they measure a one-word
        // product and a table read; `nd1` is non-dyadic and defeats both at
        // every scale and width.
        $crate::bench_one!($c, "ln@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(nd1).ln())
        });
        // `log2` / `log10` — the CONSTANT-base logarithms, public API on both
        // sides and previously unbenched anywhere in the sweep. They share the
        // narrow `log` kernel MINUS the base's own `ln` series: `log(x, base)`
        // computes `ln(base)` at run time, whereas these divide by the baked
        // `wide_ln2` / `wide_ln10` constant. Read against the `log` row they
        // therefore isolate that one term.
        $crate::bench_one!($c, "log2", $w, $scale, $side, |bn| bn.iter(|| black_box(x).log2()));
        $crate::bench_one!($c, "log10", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).log10())
        });
        // FAMILY: both constant-base logs share `ln`'s binary reduction, so
        // both inherit Trap 1 from `x = 2.0` — `log2(2)` is not merely a fast
        // path, it is EXACTLY 1. These two rows are the only measurement
        // either function has of its own series.
        $crate::bench_one!($c, "log2@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(nd1).log2())
        });
        $crate::bench_one!($c, "log10@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(nd1).log10())
        });
        // ── VARIANT A (drop this one row to revert) ──────────────────────
        // `ln` above is measured at `x = 2.0`, which is DEGENERATE for the
        // logarithm: range reduction gives mantissa m = 1 exactly, so the
        // artanh argument t = (m-1)/(m+1) is 0 and the series breaks on its
        // first iteration at every working scale. That is why the `ln` row is
        // flat in scale while every other transcendental rises.
        //
        // This row re-runs `ln` at the `base` operand (7.0 → m = 1.75,
        // t = 3/11), which exercises the series properly. It ADDS a row and
        // changes nothing existing, so the historical `ln` baseline is intact.
        // 7.0 is already an operand of this harness, so it needs no new S-1
        // bound check: ln(7) ≈ 1.946 < 10.
        //
        // NAMING IS LOAD-BEARING: the `_nd` suffix is what keeps this row OUT
        // of the published Performance page. `scripts/render_docs.py`
        // (`is_diagnostic_op`) drops it at both TSV readers, because `ln_nd`
        // names no callable function. Any future row that measures a kernel
        // rather than documenting public API must carry that suffix — or add
        // its own marker there — otherwise it WILL be published. The row stays
        // in the bbc artifacts either way; only publication is filtered.
        $crate::bench_one!($c, "ln_nd", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(ten).ln())
        });
        // ── end VARIANT A ───────────────────────────────────────────────
        $crate::bench_one!($c, "sin", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).sin()));
        $crate::bench_one!($c, "cos", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).cos()));
        $crate::bench_one!($c, "tan", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).tan()));
        // FAMILY: `ang` needs the mod-τ reduction the base rows skip entirely,
        // and its residue sits near π/4 where the Taylor series is longest.
        $crate::bench_one!($c, "sin@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(ang).sin())
        });
        $crate::bench_one!($c, "cos@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(ang).cos())
        });
        // FAMILY: `tp` drives `|tan|` past magnitude 1, which is what arms the
        // near-pole guard lift and its second `sin_cos_fixed` + `div` at the
        // lifted working scale.
        $crate::bench_one!($c, "tan@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(tp).tan())
        });
        // `asin` / `acos` stay on `s`: capped at |v| <= 1, so SCALE 0 admits
        // only 0 or the domain edge +-1, and the edge is a pathological
        // near-boundary case rather than a representative one.
        $crate::bench_one!($c, "asin", $w, $scale, $side, |bn| bn.iter(|| black_box(s).asin()));
        $crate::bench_one!($c, "acos", $w, $scale, $side, |bn| bn.iter(|| black_box(s).acos()));
        $crate::bench_one!($c, "atan", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).atan()));
        // NO FAMILY for `asin` / `acos` — MEASURED, not assumed. Both were
        // benched with an `@hard` row at `n1 = 0.99`, which crosses the
        // `|x| = 1/2` switch in `inverse_schoolbook` into the half-angle
        // identity while the base rows' `0.1` takes the direct path. Run
        // 33938890210 (the full default sweep) put the ratio at 1.01x median,
        // 0.96-1.07x overall, and 0.98-1.04x at EVERY ONE of the twelve widths
        // and all five scale positions. The branch is real but it is not a COST
        // boundary: both arms are dominated by the shared inner `atan_fixed`,
        // and the half-angle arm's three extra wide sqrts are cheap beside it.
        // Two rows measuring the base row's number are worse than no rows, so
        // they were removed. Do not re-add them without a different operand AND
        // a reason to expect a different answer.
        // FAMILY: `atb` pays BOTH the `|x| > 1` reciprocal fold and the
        // argument halvings; the base row's small argument pays neither.
        $crate::bench_one!($c, "atan@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(atb).atan())
        });
        $crate::bench_one!($c, "sinh", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).sinh()));
        $crate::bench_one!($c, "cosh", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).cosh()));
        $crate::bench_one!($c, "tanh", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).tanh()));
        // FAMILY: the three hyperbolics are built on `exp`, so they inherit
        // its reduction depth — `eh` exercises it, the small `sw` does not.
        $crate::bench_one!($c, "sinh@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(eh).sinh())
        });
        $crate::bench_one!($c, "cosh@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(eh).cosh())
        });
        $crate::bench_one!($c, "tanh@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(eh).tanh())
        });
        $crate::bench_one!($c, "asinh", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(sw).asinh())
        });
        // FAMILY: `big` is on the far side of the `|x| = 1` split, where the
        // composition takes a reciprocal, a sqrt and TWO `ln_series` calls
        // instead of one.
        $crate::bench_one!($c, "asinh@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(big).asinh())
        });
        $crate::bench_one!($c, "acosh", $w, $scale, $side, |bn| bn.iter(|| black_box(x).acosh()));
        // FAMILY: the base row's `x = 2.0` sits exactly ON the `x >= 2`
        // boundary and takes the two-`ln` branch, so the `log1p` gap form —
        // the branch that guards against cancellation as `x -> 1` — has never
        // been benched. `ac1` is the only way into it.
        $crate::bench_one!($c, "acosh@near1", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(ac1).acosh())
        });
        // `atanh` stays on `s`: its domain is the OPEN interval (-1, 1), whose
        // only integer is 0, so its SCALE 0 cell cannot be de-degenerated.
        $crate::bench_one!($c, "atanh", $w, $scale, $side, |bn| bn.iter(|| black_box(s).atanh()));
        // FAMILY: `n1` sits just inside the `|x| < 1` wall, where the gap
        // `1 - |x|` is small and the composition is at its most conditioned.
        $crate::bench_one!($c, "atanh@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(n1).atanh())
        });
        $crate::bench_one!($c, "to_degrees", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(sw).to_degrees())
        });
        $crate::bench_one!($c, "to_radians", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(sw).to_radians())
        });

        // ── binary / other ──
        $crate::bench_one!($c, "powf", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).powf(black_box(e)))
        });
        // FAMILY: `powf` has THREE paths and the single row above straddles
        // two of them by accident — `e = 1.5` spells `1` at SCALE 0, so the
        // s0 column times the integer pin while every other scale times the
        // composition over a Trap-1 base. These two rows own one path each,
        // at every scale.
        $crate::bench_one!($c, "powf@int", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).powf(black_box(pint)))
        });
        $crate::bench_one!($c, "powf@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(nd1).powf(black_box(phard)))
        });
        $crate::bench_one!($c, "log", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).log(black_box(ten)))
        });
        // FAMILY: `log`'s matcher is `ByValue` on the BASE, and every base
        // this sweep has benched gives `k = 0` — so `LnDivideConditioned`,
        // one of the policy's two routed algorithms, has never been measured
        // at any width or scale. `log@near1` is the row that reaches it.
        // `log@hard` keeps the ordinary `LnDivide` arm but on operands that
        // are not Trap 1 / Trap 2, which the `x = 2.0`, `ten = 7.0` pair both
        // are.
        $crate::bench_one!($c, "log@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(nd1).log(black_box(nd2)))
        });
        $crate::bench_one!($c, "log@near1", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(n1x).log(black_box(n1b)))
        });
        // `hypot` via this side's spelling of the integer-only
        // correctly-rounded form — the only `hypot` method exposed at EVERY
        // width in both versions (at D38 the plain dispatcher delegates
        // straight to it). Benched at every width × scale, closing the prior
        // D38-only coverage hole. The group id stays "hypot" on both sides so
        // the branch and prod rows still pair.
        $crate::bench_one!($c, "hypot", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(c3).$hypot(black_box(d4)))
        });
        // FAMILY: decimal `hypot` is exactly integer hypot on the raws, so the
        // 3-4-5 triple makes `a² + b²` a PERFECT SQUARE at every scale — the
        // int `isqrt` lands exactly and the round step's `diff_nonzero` test
        // is always false. `nd1`/`nd2` give `1898 · 10^(2S-2)` (and `136250`
        // at s0), neither a perfect square, so this row takes the inexact
        // path the base row can never reach.
        $crate::bench_one!($c, "hypot@hard", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(nd1).$hypot(black_box(nd2)))
        });
    }};
}

/// Register one function's bench into a Criterion group named
/// `<fn>_<W>_s<scale>`, with the row label `<side>` (branch|prod). The full
/// benched id is therefore `<fn>_<W>_s<scale>/<side>`.
///
/// A single scale is selectable by a criterion name-filter: pass
/// `--bench _s<scale>/` — the trailing `/` (the group/function separator)
/// anchors the scale so `_s30/` matches `<op>_D307_s30/branch` but NOT
/// `<op>_D307_s306/branch`.
///
/// NOTE: Criterion 0.8 LOWERCASES the on-disk report directory (report.rs
/// `.to_lowercase()`), so the dir is `exp_d307_s153`, NOT the original-case
/// id. The collator (`summarise.py`) therefore reads the canonical-case
/// `group_id` from each `benchmark.json` (which preserves `exp_D307_s153`)
/// rather than parsing the lowercased path.
#[macro_export]
macro_rules! bench_one {
    ($c:expr, $fn:literal, $w:literal, $scale:literal, $side:literal, $body:expr) => {{
        let mut g = $c.benchmark_group(concat!($fn, "_", $w, "_s", $scale));
        g.bench_function($side, $body);
        g.finish();
    }};
}

/// Run the full function surface for one width at ONE SCALE: the branch
/// and prod at that scale, so every `<fn>_<W>_s<scale>` group holds a
/// `branch` row and a `prod` row.
///
/// The op set is identical across all scales (it just re-instantiates the
/// const-generic `D###<SCALE>` types), so a width's bench file invokes this
/// once per scale in its scale set. Width drives limb width; scale drives
/// the per-tier scale-dependent cost the matcher routes on — fanning out
/// over scale surfaces scale-dependent regressions a single-scale bench hides.
#[macro_export]
macro_rules! scale_funcs {
    ($c:expr, $w:literal, $scale:literal, $newmod:ident, $oldmod:ident) => {{
        // Only the hypotenuse ident differs: the branch dropped the `_strict`
        // suffix, the pinned baseline still carries it. Same kernel each side.
        //
        // WHEN THE BASELINE ROLLS PAST THE API REDUCTION (i.e. once the newest
        // published release is one built from this branch), the prod side must
        // become `hypot` too — prod will no longer have `hypot_strict`, and this
        // line is the only place that needs the edit. It is left as a plain
        // ident rather than a `#[cfg]` on purpose: a cfg arm that no build
        // exercises until the day it activates is a landmine, whereas this
        // fails at compile time with `no method named hypot_strict`, on the
        // build gate, naming its own fix. The `strict` feature on the prod dep
        // stops being added by itself (the workflow probes the pinned version),
        // so this ident is the one manual step of that release.
        $crate::funcs!($c, $w, $scale, "branch", ::decimal_scaled::$newmod<$scale>, hypot);
        $crate::funcs!($c, $w, $scale, "prod", ::prod::$oldmod<$scale>, hypot_strict);
    }};
}

/// Run the full function surface for one width across that tier's scale set.
/// `$scale:literal` repetition is the width's chosen scales (see the per-width
/// files); each expands to one `scale_funcs!` (branch + prod at that scale).
/// Each per-width bench target invokes this once with its scale list.
#[macro_export]
macro_rules! width_bench {
    ($w:literal, $newmod:ident, $oldmod:ident, [$($scale:literal),+ $(,)?]) => {
        fn bench(c: &mut ::criterion::Criterion) {
            $(
                $crate::scale_funcs!(c, $w, $scale, $newmod, $oldmod);
            )+
        }

        // Faster bbc turnaround (owner 2026-06-03): 0.5s warm-up + 2.5s
        // measurement (was the criterion default 3s + 5s). The branch÷prod
        // ratio is read WITHIN one run (both sides at this same config), so the
        // central ratio is config-robust; the shorter budgets trade some
        // measurement variance — watch the slow wide-tier (ms-scale) cells,
        // which warm/sample least. CLI args (--save-baseline etc.) still apply
        // on top via the macro's internal configure_from_args.
        ::criterion::criterion_group! {
            name = benches;
            config = ::criterion::Criterion::default()
                .warm_up_time(::core::time::Duration::from_millis(500))
                .measurement_time(::core::time::Duration::from_millis(2500));
            targets = bench
        }
        ::criterion::criterion_main!(benches);
    };
}
