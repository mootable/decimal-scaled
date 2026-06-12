---
name: policy-mapper
description: Empirically MAP ONE function's algorithm policy by N-way-benchmarking every candidate algorithm × LimbSize across the full width/scale surface (+ edge input values) — producing the verified win-region map. DECIMAL sweeps the 5-point scale grid per width and bisects where winners change; INT brute-forces every width. Measure-only: the map is the deliverable; `policy-applier` wires it. One agent per policy; the coordinator verifies the map is complete.
---

# Policy mapper

The standing, repeatable way EVERY `policy::<fn>::select` is determined: **measure, don't reason.** One subagent maps a single function's algorithm choice across the full axes and **writes the policy** to match the data. Pairs with `algorithim-optimiser` (the per-algo perf lens) and `hunters` (the produce/wire split).

## The input

A single function, on the decimal tier OR the int tier.

## The result (the deliverable — this IS the proof of work)

1. **The map** — a table giving, for every `(width, scale)` cell at the sample points, the measured speed of EVERY algorithm × LimbSize (u64 / u128) candidate, with the winner marked; plus the boundary scales (decimal) or shapes (int) where the winning algorithm and/or LimbSize changes.
2. **The failures list** — every candidate that is NOT bit-identical to the reference (or fails in any other way) at a cell, so it can be excluded as ineligible.

The map must cover the WHOLE surface — every width, every sample point. A partial map is not a result; if a width or sample point is missing, the job is not done.

## The one rule

**The winner of a cell is the FASTEST algorithm that is also VALID at that cell** (bit-identical to the reference where correctness is required). Speed never overrides correctness — a faster-but-not-bit-identical candidate is INELIGIBLE there (the validity wall). The coordinator's golden gate is the final correctness arbiter; the agent's per-cell bit-identicality check is what makes a candidate *eligible* to be wired.

**CORE RULE — map the WHOLE surface, never fit one cell.** A fix or `select` arm fitted to the single benched `(width, scale)` cell means the neighbours (adjacent scales AND widths) don't get the boost — or cliff at the boundary. Map every width × its sample points, place each arm at the TRUE crossover covering its continuous win-region, and validate the straddling + immediate-neighbour cells — never a single sampled point alone. The architectural-review **Class I** check exists to catch a point-snapped gate that slips through.

## Axes to sweep (ALL of them, for the target policy)

- **algorithm** — N-way over **every** variant in that policy's `Algorithm` enum, including reference baselines and `#[allow(dead_code)]` arms. No exceptions, no pre-dropping an arm by reasoning it is dead — sweep it, measure it, check its validity. **AWARENESS:** some policies have multiple `Algorithm` variants that delegate to the SAME kernel (e.g. `add` RippleCarry≡Schoolbook, `cmp`/`eq` Limbwise≡Schoolbook, `neg` TwosComplement≡Schoolbook, `sub` RippleBorrow≡Schoolbook, icbrt `Native`≡`Newton` before its collapse). These bench as a TIE (same code). Report a same-kernel pair AS a tie from the data — do not treat it as a real crossover or try to "choose" between them. De-dup is NOT your job (leave the duplicate-name arms alone); just be aware so the verdict isn't confused by a same-kernel tie.
- **width** — every supported tier `N` (int `Int<N>`; decimal D-tier).
- **scale** — DECIMAL only: `0..=maxscale` of the tier. INT has NO scale axis; its continuous axis is operand **limb-length / magnitude / shape** (e.g. mul `(a_len, b_len)`; div_rem divisor-limbs + dividend-length). Sweep whatever continuous axis the policy keys on.
- **LimbSize** — u64 vs u128 packing (the matcher's second axis) wherever the op supports it.
- **edge input values** — where the function has VALUE-dependent control: gates (`|x| < 100`), near-grid / tie / boundary inputs, sign, 0, MIN/MAX. Sweep those explicitly. If different input values favour different algorithms, that MUST appear in the report.

## DECIMAL sampling — adaptive (do NOT brute-force every cell)

1. For every width `N`, coarse-sample the scale at the 5 points **`{0, S/4, S/2, 3S/4, S-1}`** (S = the tier's max scale; `S-1` = the top scale).
2. At each sample, run the **N-way A/B PINNED** (`scripts/pin_run.ps1 -Core <high, e.g. 22>`), assert each candidate's validity (bit-identical to the reference), and record the winner (fastest valid).
3. Where two ADJACENT samples have DIFFERENT winners, **BISECT** the midpoint (a disagreement between `S/2` and `S/4` → test `(S/2 + S/4)/2`) and recurse until the crossover is localized — down to a single scale if needed.
4. Sweep a variety of input values per cell; if some values win on a different algorithm, that must be included in the report.

## INT sampling — exhaustive (brute-force every width)

1. No subdivision: every width is sampled fully across its continuous axis (operand limb-length / magnitude / shape).
2. At each sample, run the **N-way A/B PINNED** (`scripts/pin_run.ps1 -Core <high, e.g. 22>`), assert each candidate's validity (bit-identical to the reference), and record the winner (fastest valid).
3. Sweep a variety of input values per cell; if some values win on a different algorithm, that must be included in the report.

## What the AGENT does (measure only)

- Extend/author the A/B bench for the policy's candidates across the matrix — thin `__bench_internals` wrappers over the GENERIC kernels, NEVER per-tier algorithm copies; respect the Constitution rules 1–6 and run `architectural-review` on anything added.
- **RUN** the pinned A/B microbenches (this IS the charter — NOT cargo-check-only). Record medians + per-cell bit-identicality.
- always include a timeout, if a function times-out that may be indicative of an infinite loop and should be recorded in the output, along with all parameters so it can be tried out by another agent.
- **STOP at the map. Do NOT wire `policy::<fn>::select`.** Wiring the map is `policy-applier`'s job — mixing measure + wire is exactly what this split exists to prevent. The agent produces the MAP, not a policy edit.
- `cargo check` clean for any bench harness added (default + `--features wide,x-wide,xx-wide,macros` + `--all-targets`). Commit the bench/harness to your worktree branch; **NO policy edit, NO push, NO golden, NO `cargo test`** (the coordinator gates).
- Report: **the map** (the full table — every width × every sample point + bisected boundaries × algorithm × LimbSize, winner marked) + the **failures list** (ineligible candidates per cell) + per-cell validity evidence + `cargo check` results + commit hash.

## What the COORDINATOR does

- Dispatch ONE policy-mapper agent at a time (serial queue — NOT concurrent). **Start with the INT policies, then decimal.**
  - Int queue (perf-relevant first): `mul, div_rem, rem, sqr, mul_low, sqr_low, hypot, isqrt, icbrt, sum_sq, cube, pow` (add/sub/cmp/neg are single-algorithm — skip).
  - Then decimal: `exp, ln, log, sin/cos/tan, asin/acos/atan, sinh/cosh/tanh, asinh/acosh/atanh, atan2, powf, sqrt, cbrt, hypot, exp2/log2/log10`.
- On return, **verify the map is COMPLETE before acting on it**: confirm it covers every width and every sample point (decimal: the 5 points + a bisection wherever winners change; int: every width). A missing width or sample point means the job is incomplete — send it back; never accept the agent's narrative in place of the map.
- **The map is the deliverable — there is NO policy diff to golden-gate here.** Once the map is complete and verified (and reviewed by the owner where it backs a perf claim), hand it to **`policy-applier`**, which wires `select`, proves the wired policy reproduces the map, and runs the architectural-review + golden gate. `policy-mapper` does not wire, gate, or merge.
- This is how KARATSUBA_THRESHOLD / BZ_THRESHOLD / U128_DIV_THRESHOLD and the mul/sqr LimbSize axis get empirically confirmed rather than inherited (cross-ref [[feedback_validate_selects_with_compare_all]]).

## Launch-prompt skeleton (fill `<policy>` + tier + core)

> POLICY MAPPER for `decimal_scaled`, target = `<int::policy::… | policy::…>` (CWD `<repo>`). Reset to `origin/release/0.5.0`. Load the `policy-mapper`, `algorithim-optimiser`, `hunters`, `architectural-review` skills + read `CLAUDE.md` (Constitution 1–6), `docs/ARCHITECTURE.md`, `research/phase4/RULES.md`.
> N-way A/B EVERY `Algorithm` variant of this policy (no exceptions, incl. dead/reference arms) × LimbSize across the full surface. DECIMAL target: per width, sample the 5 scale points `{0, S/4, S/2, 3S/4, S-1}`, then BISECT where adjacent winners differ down to the exact crossover. INT target: brute-force every width across its operand-length/magnitude/shape axis (no subdivision). Sweep edge input values. PIN every run: `scripts/pin_run.ps1 -Core 22`. A candidate is eligible only where bit-identical to the reference (validity wall).
> Produce the MAP — do NOT wire `select` (that is `policy-applier`'s job). `cargo check` only for any bench harness added (default + wide + all-targets) — NO policy edit / push / test / golden. Commit the harness to your branch. Report the full map (every width × every sample point + bisected boundaries × algorithm × LimbSize, winners marked), the failures list, per-cell validity evidence, and the commit hash. The coordinator verifies the map is complete, then hands it to `policy-applier`.
