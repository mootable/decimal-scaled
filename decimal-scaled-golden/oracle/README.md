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
| `generate.py` | The CLI (`generate` / `revalidate`): harvests inputs, computes each line with the function's generator, cross-validates, writes `golden/<func>.au` with a per-line provenance comment. Holds `GENERATOR_POLICY`, `DEFAULT_GENERATOR`, `VALIDATOR_ORDER`, `VALIDATOR_EXCLUDE`, `ACCEPT_ULPS`. |
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

### Per-function exclusions (`VALIDATOR_EXCLUDE`)

A validator that *cannot represent* an input abstains harmlessly. A validator
that computes a **wrong** value does not: its disagreement passes `ACCEPT_ULPS`
and **drops the line**, so a validator that is unsound for a particular function
can veto a vector the reliable oracles agree on — leaving a silent hole in
coverage exactly where the hardest inputs live. `VALIDATOR_EXCLUDE` in
`generate.py` maps a function to the validators that must not be consulted for
it.

**It is currently EMPTY, and that is the intended state.**

### The exclusion that was, and why it is gone (issue #66)

It briefly excluded `mpmath` from `log1p`, `atanh` and `acosh` near their domain
edges. The stated reason was that mpmath was unsound there. **That diagnosis was
wrong, and the lesson is worth more than the entry was.** The defect was ours:
this repo's mpmath adapter budgeted working precision from the **result's**
integer-digit count with no term for the condition number. For `log1p` at
`1 + t = 1e-70` it sized against `ln(1e-70) ≈ -161` — three integer digits —
while the derivative `1 / (1 + t)` destroyed seventy. A hard cliff at `A ≈ 72`,
where the headroom was exactly `dps - precision = 70`.

Fixed at the root: the adapter now sizes by the **worse** of the result's
magnitude and `A = log10 |x · ∇f(x)|`, the digits the function itself destroys.
mpmath then agrees with FLINT *exactly* at every depth tested, out to `A ≈ 1200`
— zero delta annotations across 120/120 `atanh` and 120/120 `acosh` lines. It is
a clean third opinion on these functions, not a tolerated one, so the entries
were removed. `oracle/conditioning_check.py` guards the fix: 53 probes, and it
reports 21 failures against the old budget, so it is a live check rather than a
decorative one.

The same latent defect affected `sin`/`cos`/`tan` of a large argument, where
`A = log10|x|` is exactly the cost of range reduction. No exclusion ever named
those — the committed trig inputs top out at four integer digits, so nothing was
visibly broken, and a fourth exclusion would eventually have been added for a
cause that was never mpmath's.

### If you are about to add an entry

**Diagnose the root cause first.** An exclusion added over an unproven "that
library is just bad here" hides our own bug and costs a real cross-check. Prefer
fixing the adapter; exclude only where the validator is demonstrably unsound
*and* two independent oracles agree without it.

**Declared is not effective.** An exclusion removes a validator from the declared
list; whether the *remaining* ones can run depends on what is installed.
`python-flint` and `mpmath` ship readily, but `gmpy2` (→ `mpfr`) and `sympy` are
optional and frequently absent, and an uninstalled backend abstains silently. So
on a typical machine an exclusion does not drop a function from four validators
to three — it can drop it to **one** (`decimal`). That is what the three removed
entries did here. It is not a correctness risk in itself (`_validate_line`
requires at least one validator to confirm, and base-10 `decimal` against a
binary/Arb generator is a good single vote to be left holding), but it means
regeneration is confirmed by one oracle rather than several, with no second
opinion on an adapter edit. **`pip install gmpy2` restores `mpfr`** — worth doing
before regenerating anything near a domain edge.

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
