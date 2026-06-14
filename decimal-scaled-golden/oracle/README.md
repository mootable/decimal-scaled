# Golden oracle generation tool

Generates and revalidates the singular golden set (`golden/<func>.au`,
signed `digits.digits` values to `GEN_PRECISION = 1233` fractional digits plus
`GUARD = 2` guard digits). Each value is computed by one per-function
**generator** oracle and cross-checked by every other available **validator**
oracle before a line is accepted. The golden set is the only thing the Rust
crate reads — it never links an oracle.

## Layout

| File | Role |
| --- | --- |
| `generate.py` | The CLI (`generate` / `revalidate`): harvests inputs, computes each line with the function's generator, cross-validates, writes `golden/<func>.au` with a per-line provenance comment. Holds `GENERATOR_POLICY`, `DEFAULT_GENERATOR`, `VALIDATOR_ORDER`, `ACCEPT_ULPS`. |
| `harvest.py` | Reads the `.pb` input layer (below): dedup, domain filter, and the per-input WHY carried into the provenance comment. |
| `oracle.py` | The `Oracle` interface (name / radix / supports / value) and the registry the adapters self-register into. |
| `exactness.py` | The terminate-vs-truncate decision: a value stays stripped (claiming exactness) only when exactness is PROVABLE — by irrationality theorem or exact rational inverse-check; everything else is re-padded to the full truncated form. |
| `functions.py` | Function registry mirroring the Rust `Function` enum: arity + in-domain predicate per function. |
| `adapters/` | One adapter per oracle backend: `fraction` (exact rational), `decimal` (correctly-rounded base 10), `flint` (FLINT/Arb rigorous intervals), `mpmath`, `mpfr` (gmpy2/MPFR), `sympy`. |

## The `.pb` input layer

Inputs live in `../lead/<func>.pb` — the lead the generator
transmutes into gold. A `.pb` file is the `.au` shape minus the output
column: one case per line (`arity` space-separated decimal literals), split
purely by function, with no width or scale anywhere (inputs are width-agnostic;
the gate derives every `(width, scale)` cell from each input). A `//` comment
line sets the WHY for every following input until the next comment; the
generator carries that WHY into the `.au` per-line provenance comment.

## The radix oracle policy

The generator is chosen per function by **radix**, not convenience
(`GENERATOR_POLICY` in `generate.py`); see also "The radix oracle policy" in
[`../README.md`](../README.md):

| Functions | Generator | Why |
| --- | --- | --- |
| `add`, `sub`, `mul`, `div`, `rem` | `fraction` | exact base-10 rational arithmetic — finite results carry no rounding at all |
| `sqrt`, `exp`, `ln`, `log10` | `decimal` | correctly-rounded native base-10 computation |
| the remaining irrational transcendentals | `flint` | Arb's rigorous intervals pin the true value, artifact-free |

A binary oracle must never generate an exactly-representable decimal: an exact
decimal has no finite binary form, so a binary oracle bakes a spurious tail
into the last digits, and a point-float oracle can floor an exact result one
unit below it.

## The validator stack

Every generated line is cross-checked by every other available oracle that
supports the function, in `VALIDATOR_ORDER` (`mpmath`, `flint`, `mpfr`,
`sympy`, `decimal`, `fraction`). A validator that cannot compute an input
abstains — it never vetoes; an exact agreement lists the validator's name in
the provenance comment; a disagreement within `ACCEPT_ULPS = 2` units at the
generation precision is a legitimate radix-rounding artifact, annotated as
`name(delta~MAGNITUDE, radix)`; anything beyond the bound drops the line and
flags it for investigation — never silently kept. A line no oracle could
confirm is also dropped.

## Usage

```
cd decimal-scaled-golden
pip install -r oracle/requirements.txt            # mpmath (BSD)
# optional extra validators (sympy BSD; python-flint / gmpy2 are LGPL, not bundled):
pip install -r oracle/requirements-extra.txt

# generate a few functions (inputs from lead/<func>.pb):
python -m oracle.generate generate --functions sqrt,exp,ln,sin --out golden --precision 1233

# re-check the committed golden set against the validators:
python -m oracle.generate revalidate --functions sqrt,exp,ln,sin --out golden --precision 1233
```

`--jobs` defaults to ~80% of the CPU cores (lines are independent, so
generation parallelises per line). `--generator` / `--validators` override the
policy for a run; `--limit N` caps the harvested inputs per function for a
quick proof set. Regenerating the FULL golden set (every function x all
harvested inputs at precision 1233) is a long compute — a maintainer/CI step,
not run on every change.

## Licensing

`mpmath` / `sympy` are BSD. The `flint` and `mpfr` adapters are OUR code
(MIT/Apache) that lazily import the user-installed LGPL packages
`python-flint` / `gmpy2` at arm's length — "works that use the Library"
(LGPL section 5), not derivatives; the LGPL packages are not bundled.
