# Strict mode - integer-only, correctly-rounded transcendentals

Transcendental functions (logarithms, exponentials, roots,
trigonometry) have exactly one implementation here: an integer-only
one. It is platform-independent, bit-identical on every target,
`no_std`-compatible, and correctly rounded to ≤ 0.5 ULP.

There used to be a second: an **f64 bridge** that converted to `f64`,
applied the platform intrinsic and converted back. It was removed in
0.6.0. It capped every result at f64's ~15 significant digits whatever
the tier's scale, so past that it was not a faster route to the same
answer but a different, worse one - and the strict path is now fast
enough that the trade had no buyer. Its `*_fast` methods and the `fast`
Cargo feature are both gone.

## The `*_strict` name

**Every transcendental is exposed under a `*_strict` name, always
compiled, regardless of which Cargo feature is active.** The plain `*`
form delegates to it - there is no configuration in which it resolves
to anything else.

```rust
use decimal_scaled::D38s12;

let x = D38s12::try_from(2i64).unwrap();

// The integer-only path, explicitly:
let r1 = x.sqrt();
let l1 = x.ln();

// The plain method delegates to exactly the same thing:
let r2 = x.sqrt();
assert_eq!(r1, r2);
```

Why the explicit name is kept: `*_strict` means strict, full stop. It
states the guarantee at the call site, and it cannot be silently
repointed by a downstream crate flipping a feature.

The `*_strict` surface covers, on `D38` (and on `D18` by
widen-compute-narrow delegation):

| Group | `*_strict` methods |
|---|---|
| Logarithms | `ln`, `log`, `log2`, `log10` |
| Exponentials | `exp`, `exp2` |
| Roots / powers | `sqrt`, `cbrt`, `powf`, `hypot` |
| Forward trig | `sin`, `cos`, `tan` |
| Inverse trig | `asin`, `acos`, `atan`, `atan2` |
| Hyperbolic | `sinh`, `cosh`, `tanh` |
| Inverse hyperbolic | `asinh`, `acosh`, `atanh` |
| Angle conversion | `to_degrees`, `to_radians` |

## Checked siblings — `checked_*_strict`

The default strict form **panics** on a domain error (`ln` of a
non-positive value, `asin` outside `[-1, 1]`, …) or when the
correctly-rounded result does not fit the storage range. Every strict
transcendental in the table above also ships a non-panicking
**`checked_`** pair returning `Option<Self>`:

- `checked_<fn>_strict_with(self, …, mode) -> Option<Self>`
- `checked_<fn>_strict(self, …) -> Option<Self>` — the default-mode sibling.

```rust
use decimal_scaled::D38s12;

let neg = D38s12::try_from(-2i64).unwrap();
assert_eq!(neg.checked_ln(), None);                // domain error -> None

let two = D38s12::try_from(2i64).unwrap();
assert_eq!(two.checked_sqrt(), Some(two.sqrt())); // in range
```

`None` covers exactly the inputs the default form would reject:

- **Domain errors** (`asin`, `acos`, `acosh`, `ln`, `log`, `log2`,
  `log10`, `atanh`): `None` on the out-of-domain inputs, at every tier.
- **Out-of-range results** (`exp`, `ln`, `hypot`, …): `None` when the
  correctly-rounded result does not fit storage — exact on `D18` / `D38`;
  on the wide tiers an out-of-range result still panics where the kernel
  seam is not yet threaded (each method's API docs state which applies).
- **Total methods** (`sqrt`, `cbrt`, `sin`, `cos`, `atan`, `atan2`,
  `tanh`, `asinh`, `to_radians`, …) cannot fail and always return `Some`.

Both forms run the same policy-dispatched kernel, so an in-range
`checked_*` result is **bit-identical** to the default form's.

## The `strict` feature

```toml
decimal-scaled = { version = "0.5", features = ["strict"] }
```

With `strict` enabled, the plain methods (`sqrt`, `ln`, `sin`, …)
dispatch to their `*_strict` form. `strict` does not require `std`; the
integer algorithms compile under `no_std + alloc`. The explicit
float-conversion methods (`to_f64`, `from_f64`,
`TryFrom<f64>`, …) remain available - they are type conversions, not
transcendental operations.

## Dispatch, in one line

There is one definition of each bare name, so no feature combination
changes what `sqrt` / `ln` / `sin` / … resolve to:

| Features | `*_strict` named methods | plain `sqrt` / `ln` / … |
|---|---|---|
| *(any combination)* | present | dispatches to `*_strict` |

This was not always true. The `fast` feature used to move the plain
names onto the f64 bridge when `strict` was absent, which made the
guarantee behind a bare `ln()` depend on a feature a downstream crate
could flip — and on which width you were calling it on, since the D38
shells and the macro-generated ones were gated independently. That
combination shipped a real defect: under `--no-default-features
--features std`, `D38::ln` became the f64 bridge at 426 ULP while
`D18::ln` stayed correctly rounded. Removing the bridge removes the
combinatorics along with it.

## The 0.5 ULP accuracy guarantee

Every strict method is held to the **[IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
correctly-rounded standard**: the returned value is within **0.5
[ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place)** (unit in
the last place) of the mathematically exact result - i.e. it is the
exact result rounded to the nearest representable value at the type's
last decimal place.

How it is achieved, per function family:

- **Algebraic roots** (`sqrt`, `cbrt`) form the exact wide-integer
  radicand (`r · 10^SCALE` for sqrt as a 256-bit value, `r · 10^(2·SCALE)`
  for cbrt as a 384-bit value), take its *exact* integer root, and
  decide the rounding with an exact integer comparison
  (`8·N ≥ (2q+1)³` for cbrt, etc.). No approximation enters.
- **Transcendentals** (`ln`, `log`, `log2`, `log10`, `exp`, `exp2`,
  `powf`, and the whole trig / hyperbolic / angle-conversion family)
  evaluate their range reduction and series in the in-tree
  `crate::algos::support::fixed::Fixed` intermediate - a 256-bit value
  at `SCALE + 30` decimal *guard digits* for the narrow tiers (D18 /
  D38); the wide tiers carry the same guard digits in their wider
  working integers. The 30 guard digits bound the total
  accumulated rounding error to roughly `1e-17` of an output ULP, far
  inside the 0.5 ULP margin, and the value is rounded once
  (half-to-even) at the very end.

This holds across the whole `SCALE` range, including `SCALE = 38`,
because the guard-digit intermediate is wider than `i128`. Every
strict transcendental is cross-checked against the platform `f64`
implementation at `D38<9>` (where `f64` is comfortably more precise
than the type's ULP) - see the in-crate tests.

All wide tiers (`D57` / `D76` / `D115` / `D153` / `D230` / `D307`
under the `wide` umbrella; `D462` / `D616` under `x-wide`; `D924` /
`D1232` under `xx-wide`) ship the full strict transcendental
surface — every method has a `*_strict` form plus a mode-aware
`*_strict_with(mode)` sibling. The wide tiers also expose
paired-output transcendentals that compute both members of a pair in
one pass and return `(Self, Self)`: `sin_cos` /
`sin_cos_with` (sine and cosine together) and `sinh_cosh`
/ `sinh_cosh_with` (hyperbolic sine and cosine together). Two
alternate implementations are
also exposed: `ln_agm` and `exp_agm` use the
quadratically-convergent Brent–Salamin / Newton path that scales
better than the artanh / Taylor canonical at very high working
scales; the canonical paths remain the default until a bench at
the relevant working scale shows AGM winning. The accuracy
contract is the same ≤ 0.5 ULP at storage as D38.

## Choosing the configuration

| You want… | Use |
|---|---|
| Bit-identical results everywhere; correct rounding | default — there is no other option |
| To state the guarantee at the call site | `*_strict`, always available |
| `no_std + alloc` | default works (the integer path is `no_std`-compatible) |
