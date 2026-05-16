---
hide:
  - navigation
  - toc
---

<div class="hero-row" markdown>
<div class="hero-copy" markdown>
<div class="hero-tagline">Bit-exact decimal arithmetic for Rust</div>
<h1 class="hero-wordmark"><span class="a">decimal</span><span class="b">-scaled</span></h1>

Const-generic base-10 fixed-point decimals - D9 through D307 - with **0.5 ULP correctly-rounded** integer-only transcendentals. Deterministic across every platform. `no_std`-friendly.
{ .hero-lede }

<div class="hero-install"><span class="prompt">$ </span>cargo add decimal-scaled</div>

[:material-rocket-launch: Get started](getting-started.md){ .md-button .md-button--primary }
[:material-chart-line: Benchmarks](benchmarks.md){ .md-button }
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

- :material-book-open-page-variant: **Usage guides**

    Per-topic deep-dives: widths, scales, strict-vs-fast routing, rounding, macros, Cargo features.

    [:octicons-arrow-right-24: widths.md](widths.md)

- :material-chart-line: **Benchmarks**

    Per-width speed tables plus the §5 like-for-like library comparison vs the top crates.io peers.

    [:octicons-arrow-right-24: benchmarks.md](benchmarks.md)

- :material-code-tags: **API reference**

    Rustdoc for every type, trait, method, and Cargo feature across every supported width.

    [:octicons-arrow-right-24: api/decimal_scaled/index.html](api/decimal_scaled/){ .external }

- :material-cog-outline: **Algorithms**

    Möller–Granlund magic-multiply, artanh-series transcendentals, Cody-Waite range reduction - the catalogue of techniques the crate uses and why.

    [:octicons-arrow-right-24: ALGORITHMS.md](ALGORITHMS.md)

- :material-map: **Roadmap**

    The wide-tier `÷ 10^SCALE`, `mul`, and transcendental-throughput gaps and the planned algorithmic fix per item.

    [:octicons-arrow-right-24: ROADMAP.md](ROADMAP.md)

- :material-history: **Changelog**

    What changed in every release.

    [:octicons-arrow-right-24: CHANGELOG.md](CHANGELOG.md)

</div>

---

## Why decimal-scaled

| You need… | decimal-scaled gives you… |
|---|---|
| Decimal arithmetic that doesn't drift (`0.1 + 0.2 == 0.3`) | Base-10 storage; exact `+ - %`, correctly-rounded `* /`. |
| Bit-identical results across Linux / macOS / Windows / ARM / x86 | `*_strict` transcendentals - integer-only, no platform libm. |
| Compile-time-fixed precision with zero per-value scale byte | Const-generic `D38<19>`, `D76<35>` etc. - scale is in the type. |
| `no_std` (or `no_std + alloc`) | Default features build under `no_std`; the strict tier needs no libm. |
| 0.5 ULP correctly-rounded `ln` / `exp` / `sin` / `sqrt` at the type's storage place | The only crate on crates.io tested that holds this everywhere. See [Benchmarks §5](benchmarks.md#5-library-comparison). |

## What it isn't

For runtime-variable scale, look at [`rust_decimal`](https://crates.io/crates/rust_decimal). For arbitrary precision at runtime, look at [`bigdecimal`](https://crates.io/crates/bigdecimal) or [`dashu-float`](https://crates.io/crates/dashu-float). For binary fixed-point (DSP, embedded radio), look at [`fixed`](https://crates.io/crates/fixed). decimal-scaled is for the case where you want **decimal**, **compile-time-fixed**, and **deterministic** - all three.
