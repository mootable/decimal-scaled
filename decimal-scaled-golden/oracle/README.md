# Golden oracle generation tool

Generates the singular golden set (`golden/<func>.golden`, `digits.digits` to
`GEN_PRECISION = 1233` fractional digits) from one configurable **generator**
oracle, cross-validated by any number of **validator** oracles. The golden set is
the only thing the Rust crate reads — it never links an oracle.

## Usage

```
cd decimal-scaled-golden
pip install -r oracle/requirements.txt            # mpmath (BSD)
# optional extra validators (sympy BSD; python-flint / gmpy2 are LGPL, not bundled):
pip install -r oracle/requirements-extra.txt

# generate a few functions (cross-validated per oracles.json):
python -m oracle.generate generate --functions sqrt,exp,ln,sin --out golden --precision 1233

# re-check the committed golden set against the validators:
python -m oracle.generate revalidate --functions sqrt,exp,ln,sin --out golden --precision 1233
```

Override the config from the CLI: `--generator mpmath --validators flint,mpfr`.

## Oracles & licensing

Each oracle implements one interface and is usable as generator OR validator
(role set by `oracles.json`). `mpmath` / `sympy` are BSD. The `flint` and `mpfr`
adapters are OUR code (MIT/Apache) that call the user-installed LGPL packages
`python-flint` / `gmpy2` at arm's length — "works that use the Library"
(LGPL section 5), not derivatives; the LGPL packages are not bundled.

## Full golden set (maintainer run)

The committed `golden/` is a small proof set. Regenerating the FULL golden set
(every function x all harvested inputs at precision 1233) is a long compute — a
maintainer/CI step, not run on every change.
