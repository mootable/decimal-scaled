# The build-max divide bound

Audit of the `MAX_WORK_N`-sized scratch in the integer divide family — the defect
class behind #86 (`asinh` panicking at D924/D1232) and #87 (`mul_karatsuba`).

**Scope.** The tree was audited at `dfc4302f`. The one thing added since that
matters for this class — `adjust_alternating_bracket`, from the sin/cos bracket
work — was verified separately at the feature tip and is not covered by the audit
below. This is not a tip-wide sweep.

## The rule

> **Exposed** ⟺ effective dividend limbs ≥ `MAX_SINGLE_LIMBS`, **and** the divisor
> is even with ≥ 24 limbs, **and** `num_m >= 2 * den_n`.

`MAX_SINGLE_LIMBS = 4 * MAX_WORK_N + 2`:

| build | `MAX_WORK_N` | `MAX_SINGLE_LIMBS` |
| :-- | --: | --: |
| xx-wide | 64 | 258 |
| x-wide | 32 | 130 |
| wide | 16 | 66 |
| narrow (default) | 2 | 10 |

Three things follow from stating it this way rather than as a list of call sites.

**It is checkable against a new call site without re-deriving anything.** The three
conjuncts are the exact conditions under which `int::policy::div_rem::select_for_limbs`
returns `Algorithm::KnuthU128Limb`, which is the only arm reaching a build-max
buffer: `div_knuth_u128_limb`'s `[0u64; MAX_SINGLE_LIMBS]`. `div_knuth` has no such
array.

**It explains why the failure panics rather than corrupting.** The bound is
*value*-derived, not type-derived: `div_knuth_u128_limb_into` strips leading zeros
to `top64` and then writes `u64buf[top64] = carry`, a real bounds-checked index. A
slice that is wide by *type* but small by *value* is fine.

**It shows the single-limb-divisor cases are unreachable by routing, not by luck.**
A divisor of one limb returns `Algorithm::Rem` before Knuth is ever considered, so
`/ lit(2)`, `/ lit(M)`, `/ lit(2j+1)` and `/ j` cannot reach the blanket at any
width.

## Verdicts at `dfc4302f`

| site | verdict |
| :-- | :-- |
| `div_rem_mag_fixed<N>`, `N <= 2` | unreachable — native u64/u128, no dispatch |
| single-limb divisors (`exp_tang:169,171,186`, `ln_tang:211,223`, `hyper_exp_identity:85,137` on `Wagm`) | unreachable — `Algorithm::Rem` |
| `ln_tang:203`, `ln_tang:261,266` | unreachable by shape — `num_m ~ den_n + 1`, so `num_m >= 2*den_n` fails; routes to `Knuth` |
| `rescale::dispatch_wide_pow10` | guarded — `blanket_ok = width_limbs <= 4*MAX_WORK_N`, falls to `MgChain` |
| `div_rem_exact` | guarded — scratch from `ComputeLimbs::single_u64()`, exact per-`N` |
| `newton_reciprocal` | guarded — own `MAX_R_U64`/`MAX_POW_U64` family; documents the 258 wall |
| `rem_int_layer:171`, `div_widen_scale:236` | guarded — call the `_into` door with caller-sized buffers |

## The `Int<512>` knife-edge

`exp_fixed_tagged` instantiates `expm1_fixed_tagged` at `C::Wexp`, which is
`Int<512>` at D1232. Its one multi-limb-divisor divide is `round_div_pow10(prod, w)`
in the tagged branch, where `prod ~ 10^(2w)` is about 131 limbs against a 66-limb
`10^w`. The u128 arm needs `num_m >= 2 * den_n = 132`.

**It misses by one limb.**

Two independent reasons keep it safe — that, and routing through the guarded
`rescale` rather than the raw operator. But a one-limb margin is not a safety
property; it is a coincidence that survives until `w` shifts the rounding of
`limbs(10^(2w))` up by one. And 131 limbs sits only 65 under the **`wide`**
blanket's 66: safe at xx-wide, marginal in a narrower `MAX_WORK_N` build.

## The root cause, once

`MAX_WORK_N` is scoped to **storage**-derived widths, while `Wexp` and `Wagm` are
chosen independently of it. D1232 pairs `Int<64>` storage with `Int<512>` `Wexp`
and `Int<256>` `Wagm`. Any blanket engine sized from `MAX_WORK_N` can therefore
meet an operand it cannot hold, and no gate catches it because every gate pairs the
wide flags — the blanket path is never fed an over-long operand.

#87 is the same family on the multiply side: `mul_karatsuba`'s build-max scratch
fits the widest work integer with exactly zero margin (`KARATSUBA_MAX_WIDTH = 256`
against `Wagm = Int<256>`), and its overflow guard is a `debug_assert!` that is
compiled out of release — so that one does not even panic.

The durable fix is for the blanket engines to take their scratch from
`ComputeLimbs`, as `div_rem_exact` already does, instead of every caller being
audited for reachability.
