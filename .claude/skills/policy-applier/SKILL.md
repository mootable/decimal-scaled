---
name: policy-applier
description: Take a COMPLETED, verified policy-map (the table produced by `policy-mapper`) and wire it into ONE function's `policy::<fn>::select` — faithfully reproducing the map, validity-walled, at true crossovers — then prove the wired policy reproduces the map and golden-gate it. The apply half of the measure/apply split; `policy-mapper` measures, `policy-applier` wires.
---

# Policy applier

The second half of the policy workflow. `policy-mapper` MEASURES (produces the map). `policy-applier` APPLIES (wires the map into the policy and proves it). They are split so the map is a reviewable checkpoint BEFORE any code is touched — the map is approved first, then applied, then verified to match.

**The map is the spec. The job is to make `select` reproduce the map — nothing more, nothing less.** No routing decision may be invented here that the map does not contain; if the map can't answer a cell, the map is incomplete — go back to `policy-mapper`, do NOT guess.

## The input

A COMPLETE, verified policy-map from `policy-mapper` for one function:
- every width × its sample points (decimal: the 5-point grid + bisected boundaries; int: every width across its shape axis),
- the winning algorithm + LimbSize per cell, with the continuous win-regions and their TRUE crossovers,
- the failures list (candidates ineligible at a cell — not bit-identical to the reference).

**Gate-in check (do this FIRST):** confirm the map is complete — every width present, every sample point present, every winner-change boundary localized to a crossover. If a width, a sample point, or a boundary is missing, STOP: the input is not a map, it's a draft. Return it to `policy-mapper`. Applying an incomplete map is the substitution trap — wiring a guess where the data is silent.

## The one rule — fidelity

**The wired `select` must reproduce the map EXACTLY.** For every cell the map covers: `select` routes to the algorithm + LimbSize the map recorded as the winner there, and to nothing else. An ineligible candidate (failed the validity wall in the map) is NEVER wired anywhere. There is no cell where the wired policy and the map disagree. If you find yourself wanting to route a cell to something the map didn't pick — stop; that decision belongs to `policy-mapper`, not here.

## Wiring the map

Translate each contiguous win-region in the map into a `select` arm:
- **decimal** — `(N, scale-range)` arms placed at the **TRUE crossover** the map localized (a continuous range, never a single benched point — architectural-review **Class I**). Value-dependent regions → a `ByValue` gate at the measured value boundary. Set the **LimbSize** per the map where the op has that axis.
- **int** — `(operand-length / magnitude / shape)` arms / thresholds at the crossovers the map gives; LimbSize per the map.
- **keep ineligible candidates out** — never wire a cell to a candidate the map's failures list marks non-bit-identical there.

Constraints (the wiring must stay Constitution-clean):
- **Keep kept-alternatives** — an algorithm that the map shows losing everywhere is NOT deleted; it stays as an unrouted alternative (architectural-review **Class E**). Only drop/repoint its stale `*_routed` delegation.
- **No matcher bypass** introduced (**Class G**), **no per-tier pollution** (**Class C**), **no build-max / derived-const** sizing (**Class A/B**).
- `select` stays `const fn` / const-foldable — the routing must still fold to one direct call per monomorphisation.
- Keep an existing config/flag triple (M / GUARD / shape) unless the map carries band-edge bit-identicality evidence for the changed config.

## The proof of work — the wired policy reproduces the map

Just as the map is `policy-mapper`'s proof, **"`select` reproduces the map" is `policy-applier`'s proof.** Demonstrate it, don't assert it:
- For every `(width, sample-scale/shape)` in the map, evaluate `select::<N, SCALE>()` (or the `ByValue` gate on the map's edge inputs) and confirm it returns the map's recorded winner. Do this arm-by-arm against the map, and where practical with a small `#[cfg(test)]` harness that walks the map's cells and asserts the routing — so a future edit that drifts from the map fails loudly.
- Report the cell-by-cell agreement (map winner vs wired `select`) — any divergence is a wiring bug, fix it before gating.

## Verify + gate (correctness)

- **architectural-review the diff (A–I)** — especially **Class I** (continuous region at the bisected crossover, never a point-snap), **Class G** (no engine hardcoded past the matcher), **Class E** (alternatives kept), **Class C** (one generic kernel, no per-tier copy).
- **golden gate** (the final correctness arbiter): `cargo test --release --features wide,x-wide,xx-wide,macros --test ulp_strict_golden` — delta==0, vector count must NOT drop. Plus `cargo check` (default + `--features wide,x-wide,xx-wide,macros` + `--all-targets`) and the full `cargo test`. A routing change among bit-identical candidates should keep golden green; if golden moves, a wired arm routed to an ineligible candidate — bounce that exact `(tier, scale/shape, mode)` back, fix, re-gate.

## Who does it / boundaries

- **The map is reviewed and approved BEFORE wiring** (the checkpoint the split exists for). Show the owner the map; do not wire a map the owner hasn't seen if it is the basis of a perf claim.
- Whether the coordinator wires it directly or a worktree agent does, the **map-fidelity proof + golden gate are mandatory**, and the wired `select` is checked against the map before merge — never merged on a narrative ("I applied the map") without the cell-by-cell agreement shown.
- NO push until golden is green; the coordinator merges/pushes. No AI trailer on the commit.

## Hand-off contract with `policy-mapper`

`policy-mapper` → produces the verified map (measure-only; stops at the map). `policy-applier` → consumes that map, wires `select`, proves `select` reproduces the map, gates. If the apply step exposes a hole in the map (an un-localized boundary, a value-gate the map never measured, a cell with no eligible winner), it is sent BACK to `policy-mapper` — `policy-applier` never fills the hole by reasoning.
