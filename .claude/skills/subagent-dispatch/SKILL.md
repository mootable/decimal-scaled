---
name: subagent-dispatch
description: Use when dispatching background/worktree subagents on the decimal-scaled project, or coordinating several in parallel. Bakes in the base-reset guard, no-AI-trailer rule, golden gate, keep-green discipline, no-push/merge protocol, check-in cadence, and surface-don't-proceed rule — all learned the hard way during the 0.5.0 multi-agent work.
---

# Subagent dispatch (decimal-scaled)

Standard requirements for **every** background/worktree subagent on this repo, plus the
coordinator's protocol for running them. Built from real defects hit during the 0.5.0
parallel-agent work. When dispatching an agent, copy the relevant clauses into its prompt.

## Isolation is the #1 rule (the agents-trip-over-main fix)

Agents were corrupting the coordinator's checkout because they ran in the **shared main
tree** and executed destructive git (the old STEP 0 `git reset --hard`, mid-wave
`git checkout <ref> -- file`). Agents MUST run in their own worktree. BUT — and this is the
trap discovered 2026-05-29 — **the Agent tool's `isolation: "worktree"` forks the worktree
from `origin/main`, NOT from your current HEAD.** On this repo all work lives on
`release/0.5.0`, which is *ahead* of `origin/main` (OpenSSF keeps `main` protected/behind, so
`main` is never the working tip). So a harness-isolated agent lands on a **stale base missing
every `release/0.5.0` commit** — unusable. (The STEP-0 guard below catches it: the agent stops
instead of working on the stale base.)

**The working pattern (coordinator pre-creates the worktree):**

1. **Commit your WIP on `release/0.5.0` first** (the worktree forks from a *commit*; uncommitted
   changes do NOT travel). No push needed — the next step forks from your **local** branch tip,
   including unpushed commits.
2. **Create the worktree yourself, IN-PROJECT under `.claude/worktrees/`** (owner directive
   2026-05-30 — NOT a sibling `../<dir>`):
   `git worktree add -b <phase-descriptive-branch> .claude/worktrees/<dir> release/0.5.0`
   This forks from your local `release/0.5.0` tip — correct base, spec-current, no `origin/main`,
   no push. **Why in-project, not a sibling dir:** a sibling (`../decimal_scaled-foo`) sits
   OUTSIDE the harness's primary-dir permission scope, so EVERY command the coordinator runs
   in it triggers a fresh approval prompt — and the stray folders pile up needing manual
   cleanup (the friction the owner hit 2026-05-30). `.claude/` is **git-ignored** (`.git/info/exclude`)
   AND the cargo workspace `members = [".", "./macros"]` is an **explicit list, not a glob**, so an
   in-project worktree is NEITHER tracked NOR swept into a build — the old "sibling keeps it out of
   the globs" rationale was never needed here. Keep worktrees under `.claude/worktrees/<dir>`.
3. **Dispatch the agent WITHOUT `isolation: "worktree"`** (you already made the worktree), and
   tell it in the prompt: *its working tree is `<absolute worktree path>`; use absolute paths
   under it for ALL file ops; prefix every bash with `cd <path>` (or use `git -C` /
   `cargo --manifest-path`); commit on that worktree's branch; never touch the coordinator's
   `…/decimal_scaled` tree.* It is isolated by construction (own dir + branch).
4. **Merge + clean up:** when it returns, review the diff, golden-gate, `git merge --no-ff` the
   branch into `release/0.5.0`, then `git worktree remove -f .claude/worktrees/<dir>` +
   `git worktree prune`. **Prune the worktree the moment its branch is merged** — a leftover
   merged worktree (the `decimal_scaled-mul` cruft 2026-05-30) is exactly the tidy-up burden the
   owner objected to. For purely SERIAL coordinator work (one algorithm at a time, no parallel
   agents), the owner prefers working **directly in the main checkout** over spinning a worktree
   at all — reserve worktrees for genuinely isolated/parallel agent dispatches.

- **`git reset --hard origin/<branch>` is SAFE inside a genuinely-isolated worktree** (it moves
  only that worktree's own branch — it cannot touch the coordinator's tree). It is forbidden
  ONLY in a shared/main tree. So an agent in its OWN worktree MAY base-reset; the STEP-0 guard
  below is non-destructive simply because the coordinator already put it on the right base.
- **Read-only research / audit agents are the ONE main-tree allowance** — Read/Grep/Glob only,
  **no git mutation**, at most a single report file under git-ignored `research/`. When in doubt,
  give it a worktree.
- **`isolation: "worktree"` is usable ONLY when `origin/main` is the intended base** (rare here).
  Until `main` catches up to the work, prefer the pre-created-worktree pattern above.
- **Concurrency:** pre-created worktrees run in parallel (up to 4) as long as their eventual
  **merge targets** don't collide — partition by file area ([[feedback_resource_limits]]).
  CPU-pin concurrent *benches* (`coordinator-perf-multiagent` / `pin_run.ps1`); never run an
  unpinned measurement while siblings are active.

## Model selection — match the model to the work (owner 2026-06-12)

Set the Agent tool's `model` parameter per dispatch, judged from the charter (not the file
count):

- **simple** → `sonnet` — mechanical edits, applying an already-reviewed patch, doc/config
  touch-ups, a scripted refactor with no judgment calls.
- **medium** → `opus` — well-scoped multi-file work, mirroring an existing pattern (e.g. a CI
  shard matrix that copies an established one), test re-gating, focused harness work.
- **complicated** → the **highest available model** — `fable`, or `opus` if Fable is not
  available — anything needing real diagnosis: kernel/rounding/wide-band fixes, shared
  high-blast-radius code, macro machinery, policy-mapper measurement runs.

If a "simple" dispatch surfaces real diagnosis mid-flight, pull it back and re-dispatch at the
higher tier rather than letting the smaller model improvise. The coordinator reviews every
diff regardless of the model that produced it.

**Concurrency budget — 6 points (owner 2026-06-12).** Concurrent subagents are budgeted by
model: `fable` costs **4**, `opus` costs **2**, `sonnet` costs **1**; the points of all
*currently running* agents must total **≤ 6** (e.g. one fable + one opus, or one fable + two
sonnets, or three opus). A returning agent frees its points for the next dispatch. The budget
supersedes the older one-agent-at-a-time rule; the disjoint-file-area partition rule still
applies to everything running concurrently.

## Every agent prompt MUST include

### 1. Verify your base — NON-destructive (no reset, ever)
The coordinator dispatches you into your **own worktree** at the intended tip. Your first
step CONFIRMS that base; it must never mutate git:
> **STEP 0:** Verify a KNOWN tip-only file exists (something the latest merge added — the
> coordinator names it explicitly). If it's **absent**, STOP and report — you are on a stale
> base and the coordinator must re-dispatch. Do **NOT** `git fetch`/`reset --hard`/`checkout`
> to "fix" it: you may be in a shared tree and a reset would wipe the coordinator's work.

### 2. No AI attribution (recurring defect — see the no-ai-references memory)
> NO AI / Claude / Anthropic / `Co-Authored-By` / 🤖 anywhere — commit messages,
> comments, code, docs, file headers. Plain `git commit -m`, no trailer.

### 3. Correctness — test ONLY your change; the FULL golden/suite is the COORDINATOR's, not yours
> **Do NOT run the full `ulp_strict_golden` suite or the full `cargo test`.** Those are the
> coordinator's merge gate (coordinator protocol step 4) — an agent running them too just burns
> time on a multi-minute suite the coordinator re-runs anyway. **Your scope:** `cargo check` across
> the configs (#4) to prove it compiles, plus — only if it directly exercises your change and is
> quick — at most ONE focused/narrow test (e.g. a single `--test <name> <filter>` or the kernel's
> own unit test). Then report and stop.
> The behaviour-preserving CONTRACT still holds (the change must keep `ulp_strict_golden` green at
> delta==0, and the vector count must never go backwards) — but it is **verified by the coordinator**,
> not pre-run by you. If your focused check or `cargo check` surfaces a problem, report it; if you
> believe a change is behaviour-affecting (not a clean refactor), say so explicitly so the coordinator
> golden-gates with extra care.

### 4. Keep green — `cargo check` across configs (NOT the full `cargo test`)
> Run **`cargo check`** green across: default, `--no-default-features` (no_std), and
> `--features wide,x-wide,xx-wide,macros`; when touching algo/policy code verify BOTH the std and
> no_std build paths compile. Paste the check results before reporting. **Do NOT run the full
> `cargo test`** — that's the coordinator's gate (#3). (Docs/scripts-only changes: confirm no
> `.rs` changed.)

### 5. No push — the coordinator merges
> Commit on your worktree branch; do NOT push; never touch main / release / tags;
> no publishing. (`cargo publish`, tag pushes, GitHub releases need explicit per-action
> human authorisation — never an agent's call.)

### 6. Scope isolation (when parallel agents run)
> Stay in your file area so merges stay clean. If another agent owns `src/`, touch only
> `docs/` / `scripts/` / `.github/`. Name the disjoint set explicitly per agent.

### 7. No fabrication
> Cite real data (benchmarks.md / results/precision/*.tsv); flag NEEDS-BENCH where the
> data can't decide; never invent timings or precision numbers.

### 7b. Full-surface validation — CORE RULE (perf / policy / select changes)
> A `select` arm, gate, threshold, or kernel change must be validated/placed across its WHOLE
> continuous win-region — every supported width × a scale set per width (coarse-sample
> `{0, S/4, S/2, 3S/4, S-1}`, then bisect the true crossover where adjacent winners differ) —
> NOT the single benched `(width, scale)` cell where the
> regression surfaced. **Fitting to one scale/width means the neighbours (adjacent scales AND
> widths) don't get the boost, or cliff at the boundary.** Place a gate at the bisected true
> crossover and check the straddling + immediate-neighbour cells. A point-range arm
> (`SCALE == 19`, `(4, 75..=75)`) or a fix justified by ONE bbc cell is the trap (architectural-
> review **Class I**). Exempt: a blanket-generic fix with no gate. Report the grid/region you
> validated, not just the target cell.

### 8. Report format
> Report: base verification (the tip-only file was present — no reset done), what changed, what was
> REMOVED, the green-check lines (golden + feature combos), the width×scale region validated
> (not just the target cell), and anything deferred.

## Coordinator protocol

- **Before dispatching ANY writing agent: commit your WIP on `release/0.5.0`, then pre-create
  its worktree IN-PROJECT** (`git worktree add -b <branch> .claude/worktrees/<dir> release/0.5.0`
  — under `.claude/`, NOT a sibling `../<dir>`; see the Isolation rule above for why) and dispatch a
  non-isolated agent told to work only in that path — see the Isolation rule above for why
  `isolation: "worktree"` is NOT used here (it forks from stale `origin/main`). Read-only
  research agents may run in the main tree.
- **Parallelise up to 4** disjoint agents; **sequence** any that touch the same files
  (e.g. two `src/algos` refactors must run one after the other, not together).
- **Check-in cadence:** for long-running (multi-hour) agents, check progress about every
  **15 minutes** — read partial output or arm a `Monitor`; don't go silent for the whole
  task. Give the user a one-line status at each merge/checkpoint, not a wall of text.
- **Merge protocol per agent:**
  1. Verify AI-trailer-clean: `git log <base>..<branch> --format=%B | grep -iE "co-authored-by|noreply@anthropic|🤖"` → expect empty.
  2. **Check the merge-base is the CURRENT tip** (`git merge-base release/0.5.0 <branch>`) — catches the stale-base bug before merging.
  3. `git merge --no-ff <branch> -m "<clean message>"`; resolve conflicts.
  4. **Golden + the full `cargo test` LOCALLY on the merged tip — BEFORE the push — is the correctness gate** (owner rule 2026-05-29, [[feedback_coordinator_runs_golden_tests_locally]]; SUPERSEDES the earlier "CI is the gate" — owner's directive to gate locally; CI is reliable, the non-completing golden runs this session were caused by my own churn, not a CI defect). Run `cargo test --release --features wide,x-wide,xx-wide,macros,golden --test ulp_strict_golden` + the full `cargo test`; require delta==0 + the vector count NOT dropped. **Green → push (step 5). RED → reset the merge, bounce the failing cells to the agent, do NOT push.** The agent's golden is a *signal, not proof* (one agent claimed all-green while its tree failed 70 cells) — the coordinator's own local run is the proof. **Benches are NEVER run locally** (bbc/sweeps are GHA-only). **Docs-only / data-source-only merges** (nothing under `src/`/`tests/`) can't move golden — verify the drift-gate / docs regeneration instead.
  5. Push; then prune: `git worktree remove -f -f <path>` + `git branch -D <branch>` + `git worktree prune` (dirs may be Windows-locked — harmless, prune later).
- **CI:** after pushing, the `ci.yml` run is the formal gate — `Monitor` it (allowlisted,
  no prompt). `concurrency: cancel-in-progress` means rapid pushes cancel older runs;
  only the latest tip's run completes — ignore `cancelled`/superseded ones.
- **Surface, don't silently proceed:** if an agent removed documented data, made a content
  decision (dropped a table, changed a claim), or hit a correctness risk, surface it to the
  user BEFORE merging — don't bury it in a status line.

## Monitor / autonomous cadence
- `Monitor` is allowlisted (no prompt) — use it freely for CI/agent watches.
- Agent-completion + background-command notifications arrive automatically (no prompt) —
  lean on them; do NOT poll or sleep waiting.
- Autonomous mode (user AFK / "carry on"): keep committing, don't pause for permission;
  surface only genuine decisions — content removals, correctness risks, irreversible/
  outward-facing actions (publish, force-push, protection changes).
