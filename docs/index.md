---
hide:
  - navigation
  - toc
---

<div class="hero-row" markdown>
<div class="hero-copy" markdown>
<div class="hero-tagline">Bit-exact decimal arithmetic for Rust</div>
<h1 class="hero-wordmark"><span class="a">decimal</span><span class="b">-scaled</span></h1>

Const-generic base-10 fixed-point decimals — twelve widths from **D18 to D1232** — with **≤ 0.5 ULP correctly-rounded** integer-only transcendentals. Deterministic across every platform. `no_std`-friendly. Caller-chosen rounding mode at every lossy operation.
{ .hero-lede }

<div class="hero-install"><span class="prompt">$ </span>cargo add decimal-scaled</div>

[:material-rocket-launch: Get started](getting-started.md){ .md-button .md-button--primary }
[:material-chart-line: Benchmarks](performance.md){ .md-button }
[:fontawesome-brands-github: GitHub](https://github.com/mootable/decimal-scaled){ .md-button }
{ .hero-actions }
</div>
<div class="hero-mark">
--8<-- "assets/mootable-cow-dusk.svg"
</div>
</div>

---

<div class="grid cards" markdown>

- :material-rocket-launch: **Getting started**

    Install snippet, the first `D38<2>` value, and a tour of the per-width macros.

    [:octicons-arrow-right-24: getting-started.md](getting-started.md)

- :material-book-open-page-variant: **Reference**

    Per-topic reference pages: widths, rounding, conversions, serde, cross-scale operations, glossary.

    [:octicons-arrow-right-24: reference.md](reference.md)

- :material-chart-line: **Bench**

    Per-width speed tables, the precision surface, version history, and the like-for-like library comparison vs the top crates.io peers.

    [:octicons-arrow-right-24: performance.md](performance.md)

- :material-check-decagram: **Correctness, proven**

    The four-layer validation story: golden oracle tables asserted bit-exact, for every rounding mode, across every shipped width.

    [:octicons-arrow-right-24: precision-testing.md](precision-testing.md)

- :material-code-tags: **API reference**

    Rustdoc for every type, trait, method, and Cargo feature across every supported width.

    [:octicons-arrow-right-24: api/decimal_scaled/index.html](api/decimal_scaled/){ .external }

- :material-cog-outline: **Algorithms**

    Möller–Granlund magic-multiply, artanh-series transcendentals, Cody-Waite range reduction - the catalogue of techniques the crate uses and why.

    [:octicons-arrow-right-24: ALGORITHMS.md](ALGORITHMS.md)

- :material-map: **Project**

    Where the crate is heading, what changed in every release, and how to contribute.

    [:octicons-arrow-right-24: ROADMAP.md](ROADMAP.md) ·
    [:octicons-arrow-right-24: CHANGELOG.md](CHANGELOG.md)

</div>

---

## Why decimal-scaled

| You need… | decimal-scaled gives you… |
|---|---|
| Decimal arithmetic that doesn't drift (`0.1 + 0.2 == 0.3`) | Base-10 storage; exact `+ - %`, correctly-rounded `* /`. |
| Bit-identical results across Linux / macOS / Windows / ARM / x86 | `*_strict` transcendentals - integer-only, no platform libm. |
| Compile-time-fixed precision with zero per-value scale byte | Const-generic `D38<19>`, `D76<35>` etc. - scale is in the type. |
| `no_std` (or `no_std + alloc`) | Builds under `no_std + alloc` with `default-features = false`; the strict, integer-only path needs no libm. |
| ≤ 0.5 ULP correctly-rounded `ln` / `exp` / `sin` / `cos` / `tan` / `sqrt` / `atan` / `sinh` / `cosh` / `tanh` and friends — by default | At every shipped width, HalfToEven by default, bit-identical across every platform. Switch rounding mode per call via `*_with(mode)` or crate-wide via the `rounding-*` features. See [Comparisons](comparisons.md). |

## What it isn't

For runtime-variable scale, look at [`rust_decimal`](https://crates.io/crates/rust_decimal). For arbitrary precision at runtime, look at [`bigdecimal`](https://crates.io/crates/bigdecimal) or [`dashu-float`](https://crates.io/crates/dashu-float). For binary fixed-point (DSP, embedded radio), look at [`fixed`](https://crates.io/crates/fixed). decimal-scaled is for the case where you want **decimal**, **compile-time-fixed**, and **deterministic** - all three.
