# Follow-ups in progress

Status of the four follow-ups from the previous session, after this
autonomous pass.

## 1. D128 strict transcendentals on `decl_wide_transcendental!` — **partial**

The original blocker was a feature gate: `wide_int` only compiled
under `feature = "wide"`, so D128 could not reach for `Int512` as
its guard-digit work integer without forcing the wide tier on.

**This session:** lifted that gate. `wide_int` is now compiled in
every feature configuration (commit `Lift wide_int feature gate…`).
~2k extra LOC compile in default builds, not material.

**Remaining:** make `decl_wide_transcendental!` accept primitive
storage (`$Storage = i128`) in addition to wide-int storage. The
macro currently uses `<$Storage>::resize::<W>()` and
`<$Storage>::MAX.resize::<W>()` — both wide-int trait methods that
don't exist on `i128`. The fix is a tiny conversion trait the macro
takes as a parameter (`fn into_work(self) -> W`,
`fn max_as_w() -> W`, etc.), or alternatively a free
`to_work::<S, W>(s)` helper that specialises on `S: WideInt` vs
`S: PrimInt`. Once that's in, D128 strict invokes
`decl_wide_transcendental!(D128, i128, Int512, ...)` and ~775 lines
across `d128_kernels.rs` + `log_exp_strict.rs` / `trig_strict.rs` /
`powers_strict.rs` disappear. Worth benchmarking against the
current hand-tuned 256-bit path before merging.

## 2. Per-width raw constants for wide tiers — **architectural plan only**

The bug: `D256<76>::pi()` panics on the rescale-up overflow because
the wide-tier `DecimalConsts` impl widens the i128 SCALE_REF=37
reference. For `SCALE > 38` the intermediate `i128 * 10^k` doesn't
fit.

**The right fix is *not* hand-transcribed string literals.**
Transcription risk at 75 digits per constant is real, and at 153
(D512) / 307 (D1024) it's prohibitive. Three feasible vehicles:

1. **`build.rs` + a high-precision crate.** Call `rug` or `astro-float`
   under a `[build-dependencies]` to compute the constants at the
   per-width target precision and emit them as Rust source that the
   crate `include!`s. The build dependency does not affect runtime
   `dependencies` or `dev-dependencies`. Cleanest answer.
2. **Runtime computation via `decl_wide_transcendental!`'s own
   Machin/series for π, artanh for ln2/ln10.** The wide-tier core
   already computes these correctly. Move the per-width
   `DecimalConsts` impl into `decl_wide_transcendental!` (the macro
   that owns the series machinery) and have it dispatch into
   `pi(w) -> $Storage`. Per-call cost a few µs; cache via
   `core::cell::OnceLock` (`std` only) or compute once per scale.
3. **Hand-transcribe + cross-check tests.** Use the runtime
   computation (option 2) as the test oracle: after hand-writing,
   assert `hand_pi == runtime_pi ± 1 LSB`. Catches transcription
   errors without paying runtime cost in production.

Recommended: (1) for D1024, (2) or (3) for D256/D512. Pick one
vehicle per the build-dep policy.

## 3. `decl_decimal_full!` umbrella for `core_type.rs` — **deferred**

`core_type.rs` is ~1600 lines, ~70 % macro invocations per width.
A `decl_decimal_full!(D256, Int256, Int512, Int1024, 76)` umbrella
would expand to the existing `basics!` + `arithmetic!` + `display!`
+ `overflow!` + `sign!` + `consts!` + `from_str!` + `bitwise!` +
`int_methods!` + `decl_wide_roots!` + `decl_wide_transcendental!` +
`num_traits!` + `conversions!`(×8) + `float_bridge!` + `rescale!`
calls — ~25 lines collapsing to one.

Risk: the wider macro arity makes type errors harder to diagnose,
and the per-call args don't fully line up (some take `$Wider`,
some don't; conversions take per-source-type lists). The current
shape is debuggable. Worth doing once the API stabilises; not
urgent.

## 4. Karatsuba / AGM — **deferred**

**Karatsuba multiplication**. Schoolbook `limbs_mul` is currently
`O(n²)`. For `Int1024` (8 limbs) the cross-over to Karatsuba is
break-even at best (constant factors dominate). For `Int4096` (32
limbs) and `Int8192` (64 limbs) Karatsuba would deliver
~2–4×. Implementation is a recursive splitter over `&[u128]` plus
careful carry handling on the `x0 + x1`, `y0 + y1` half-sums (they
need `n/2 + 1` limbs). ~200 lines of code with thorough tests
against the schoolbook reference. Worth shipping when D2048/D4096
mul/div lands as a public surface.

> Karatsuba, A. and Ofman, Yu. (1962). "Multiplication of
> Multidigit Numbers on Automata." *Doklady Akad. Nauk SSSR* 145,
> 293–294.

**AGM-based `ln` / `exp` (Brent–Salamin)**. Quadratic convergence;
beats Taylor at ~50+ digits of working precision. Would let the
wide tier reclaim approximately half the perf cost of the recent
`round_div` rounded-intermediate switch (D256 strict ln 337 µs →
~150 µs estimated). Implementation is straightforward but the
correctness analysis (residual rounding budget vs the half-to-even
contract) needs care. ~300 lines, including the `M(a, b)` AGM
iterator and the auxiliary `K(k)` for the log path.

> Brent, R. P. (1976). "Fast multiple-precision evaluation of
> elementary functions." *Journal of the ACM* 23(2), 242–251.
> DOI: [10.1145/321941.321944](https://doi.org/10.1145/321941.321944).

---

## Recommendation: build-dep policy decision

Items (1) and (2) are both partially blocked on whether the crate
should grow `[build-dependencies]`. Today there are none. The two
candidates would be:

- For (2): a high-precision library (`rug`, `astro-float`,
  `dashu-float`) at build time to materialise the per-width
  constants. Build-only — does not affect downstream consumers'
  runtime dependency graph.
- For (1): nothing needed; just the macro refactor.

Decide on (2) first. If build-deps are acceptable, both follow-ups
unblock cleanly.
