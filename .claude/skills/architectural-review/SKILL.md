---
name: architectural-review
description: Use to verify the decimal_scaled repo still matches its architecture as defined — a read-only audit against the CLAUDE.md ARCHITECTURE CONSTITUTION (rules 1–6) and docs/ARCHITECTURE.md. Run it periodically, after any structural or perf change, or before a release. Carries the source-tree map, then five rules with the defect classes A–N beneath them, each giving its context, what is expected, what is disallowed, how to find it and how to fix it, followed by the read-only protocol and a launch skeleton for the auditor agent.
---

# Architectural review

A read-only audit of `src/` against the architecture as defined. The auditor reads and reports; the coordinator judges and fixes. The audit changes nothing itself, so it can run safely alongside other work.

## How this file is written

This file states rules. It never states the condition of the code. No sentence here says how a file works, names a kernel, or quotes a threshold or a buffer size, and nothing here carries a date or an incident.

That matters for a reason worth knowing. A description of current code goes stale, so an auditor gets sent after a defect that no longer exists. Worse, it seals in an old implementation: the auditor stops applying the principle and starts restoring the description, and will undo deliberate, measured work because this file still describes what came before it. If you find a sentence here describing current code, act on the rule instead, and report the sentence as a defect in this file.

Below the structure section, no identifier belonging to this project appears in backticks. Rust and std vocabulary does, because that is the language the rules are written in and no refactor here can rot it. Locations come from the structure section, and the names at those locations are read from the tree when the audit runs. The reason for the split is how each kind of mistake surfaces: a folder named wrongly here fails loudly, because you list it and see, while a kernel named wrongly inside a detector fails silently, since a search that matches nothing reads exactly like a clean audit.

The examples are real defect shapes with their pointers removed. What rots is the tie to a particular file or occasion, never the shape itself, so an example names nothing and sends you nowhere.

## The authority

1. `CLAUDE.md` — the ARCHITECTURE CONSTITUTION, rules 1–6. The law.
2. `docs/ARCHITECTURE.md` — layering, work-width scratch, the BigRule, policy file structure, keeping the alternatives, and the source map.
3. The scratch-carrier module. Read it before Class B.

Read all three before starting. The current names live there, not here.

## The code structure

The tree is the authority. If what you list disagrees with this map, the tree wins and the map is the defect — report it.

```
src/
  int/              integer layer
    types/          limb integer types, integer traits, the scratch carrier
    policy/         one matcher file per function
    algos/<fn>/     one file per algorithm; support/ = helpers shared in this layer
  types/            decimal typed shells; traits/ and consts/ beneath
  policy/           one matcher file per function
  algos/<fn>/       one file per algorithm; support/ = helpers shared in this layer
  algo_x_support/   the shared LEAF bucket, used by both layers
  consts/           generated constant tables
  macros/           per-type method surface only
  support/          rounding, errors, formatting, serde
  identity/         equality, ordering and hashing contract
```

The two layers mirror each other. An algorithm file is named for its algorithm and a matcher file for its function. A leaf calls nothing in-crate: algorithms call into leaves, and leaves call back into nothing. The call graph points down throughout, from a type to a matcher to an algorithm to a kernel or a leaf.

**The moving parts.** The *level's own consts* are the const generics an entry is entitled to: one width for a unary integer entry, both operands' widths for a binary one, width and scale for a unary decimal entry, and both widths and both scales for a binary one. The *scratch carrier* supplies work buffers wider than a value's own width, sized inside the impl where the width is concrete so that the size never reaches a signature. The *build-max family* sizes a buffer to the widest width the whole build configures, rather than to the width in hand. A *matcher* classifies a cell and returns a verdict, and choosing is all it does. An *algorithm enum* lists a function's selectable kernels as its variants, and the matcher can only choose something that has one. A *runtime verdict* depends on operand value or shape, so it survives compiling as a branch instead of folding away. A *kept alternative* is an algorithm kernel that nothing currently routes to.

## The rules

Each class below gives its context, what is expected, what is disallowed and how to find it. Where the fix is a procedure rather than a property it gets a step of its own, and where the shape is worth showing, an example. Where the fix is simply a property, it is stated in Expected and needs no separate step.

Report each finding as `file:line` — description — rule broken — severity — fix — how you found it — what would disprove it. Where you cannot resolve something, flag it with a "MIGHT be intentional — verify" note rather than passing it silently.

A finding is a claim, not a verdict. What makes it one is the method and the disproof: a conclusion stated bare can only be contested by re-deriving it from scratch.

Every detector works the same way: enumerate the current names from the module or matcher that owns them, then search the folders that rule names for uses. Never search from a list written in this file.

A comment that justifies a defect is a defect marker rather than a clearance. Quote it and flag the finding anyway.

The classes keep their letters, because other skills cite them. A and B sit under Rule 3; C, E and N under Rule 1; D and H under Rule 2; F, G, I, L and M under Rule 4; J and K under Rule 5.

### Rule 1 — One generic type, one generic algorithm

**C — Per-tier pollution, including algorithms hidden in macros** · HIGH · a blind spot

**Context.** One generic type covers every width, so an algorithm should exist once and serve all of them. A macro that emits computation breaks this twice over: the body is written again at every expansion site, and it is an algorithm with no file of its own and no variant naming it, so the matcher cannot select it for any cell — see Class N, which the two of them compound.

That is also why it is a blind spot rather than merely a defect, and the label is earned by how audits of it fail. The natural way to audit algorithms is to enumerate the files under the algos folders, and this class is invisible to that method by construction, because the body has no file to enumerate. It is found only by reading the macro folder, which is not where anyone looks for an algorithm. An audit that never opens that folder returns clean and reads exactly like an audit that found nothing.

**Expected.** Each algorithm sits in one file in its function's algos folder. A macro emits surface only, meaning a call through to a matcher or a kernel with trivial wrapping — a bit conversion, a sign flip, a mode default, and nothing more. Constants that must be generated per width or per scale belong in the tables and constants files, which is the answer to a macro defended on the grounds that each tier needs different constants.

**Disallowed.** A per-tier type, a per-width macro arithmetic arm, or a per-tier copy of an algorithm. A macro body that computes, which it does when it forms a working scale, calls a mul, div, sqrt, series or table primitive, branches on value ranges, or does its own rounding. A macro at all where the per-tier types are aliases of one generic type, since one generic impl already covers every tier.

**How to find it.** Read every file in the macro folder and confirm no macro is defined outside it. Classify each body it emits as computing or delegating, and size what you find as body lines multiplied by expansion sites, because the body is written again at each one.

**Example.** A body that computes sits inside a macro. It appears in no listing of algorithm files because it has none, so it can be removed entirely without any file changing and without any review noticing.

**N — Computation the matcher cannot route** · HIGH

**Context.** The matcher picks among the variants of a function's algorithm enum, so a computation with no variant can never be chosen for a cell. That is what turns an ordinary defect into a serious one, and it is the argument for ranking this HIGH: with no routing seam, a precision or performance problem inside such a computation cannot be fixed for the band that suffers it, because the only lever left is replacing the call site wholesale, which changes every cell in order to fix one band.

**Expected.** Every computation is reachable as a variant, so the matcher can select it per cell and a fault in it can be corrected where it occurs.

**Disallowed.** A computing macro body. A computing free function that no variant names. A type method that computes inline instead of delegating to its function's matcher. Two implementations of one public function where only one is reachable through the matcher.

**How to find it.** Ask of every computation in the algos and macro folders which variant selects it, and cross-check each matcher's enum against the computations that exist for that function.

**How to fix it.** Lift the computation into its own file in its function's algos folder and register it as a variant, so the matcher can select it per cell. Never delete it instead — see Class E.

**Example.** A precision fault appears in a narrow band at high scale. Nothing can route per cell, so the only fix available is swapping the kernel at the call site, and that one band is corrected at the cost of every other cell that was already right.

**E — Superseded duplicates and orphan cruft** · MED/LOW

**Context.** Algorithms are kept, not deleted. An unwired kernel, a candidate awaiting a bench, a reference baseline and a today-loser are all alternatives the matcher may select later, so being unrouted is their normal state rather than evidence of dead code. The test that settles any particular case is whether the thing could win under other conditions: two implementations that differ in cost or in accuracy are two algorithms, which is exactly what a kept alternative is, while a leftover signature of something now canonical is not.

**Expected.** Every algorithm kernel stays in the tree whether or not anything routes to it, and a defect inside an alternative is fixed in place. Where a candidate for removal is ambiguous, it is kept.

**Disallowed.** An obsolete signature left behind by a migration that replaced an algorithm in place, and an unused const, helper or import that implements no algorithm — those are the only two things this class flags. At review time: any recommendation to delete an algorithm kernel, and any diff that removes an alternative instead of repairing it.

**How to find it.** File-shaped checks are not enough, because a diff can delete an algorithm while removing no file and no enum variant. Read hunks for removed working scales, series, table reads and rounding steps, and treat "it was only a duplicate, a shell, a wrapper" as a defect marker rather than a clearance.

**Example.** A diff replaces a computing body with a one-line delegation. No file is removed and no enum variant moves, so every file-shaped check passes and the change is waved through as touching only a wrapper. An algorithm has gone.

### Rule 2 — Everything in its place, and the call graph points down

**D — Layering** · MED/LOW

**Context.** The call graph points down: a type reaches a matcher, a matcher chooses an algorithm, an algorithm calls kernels and leaves. Each layer can then be understood and replaced on its own. A call that goes up or sideways couples two layers that need to stay separable.

**Expected.** A matcher classifies and chooses and does nothing else. An algorithm lives in its own file in its function's algos folder and calls kernels rather than methods on its own type. A leaf calls nothing in-crate.

**Disallowed.** Scratch or output allocation inside a matcher, which means it has taken on work belonging to the algorithm. An algorithm calling a method on its own type, or a decimal algorithm calling a decimal method on its own value. A leaf calling back up into an algorithm, a matcher or a type. An algorithm that does not live in its own file in its function's algos folder, even when it computes the right answer.

**How to find it.** Read the matcher files for allocation, and the algorithm files for calls aimed at their own type rather than at a kernel. Check each leaf for in-crate calls.

**H — Hand-rolled seed bypassing the shared library** · MED

**Context.** Root and Newton estimates come from the shared seed library. It carries a hardware-float bootstrap where the build provides one and a pure-integer fallback where it does not, together with a guaranteed-over-estimate correction that keeps the refinement loop off a linear floor-walk. An inline estimate fails in one of two distinct ways. A pure-integer one computes the right answer but forfeits the fast bootstrap in every build, including the builds that could have had it. An uncorrected float one risks the floor-walk and makes the result depend on which build produced it, so determinism goes with it. Correct arithmetic in the estimate is therefore not a defence.

**Expected.** A kernel needing a root or Newton estimate calls the seed library, and a weak seed is fixed inside the library where every caller benefits.

**Disallowed.** A local starting value formed just before an iterative refinement — a float `sqrt` or `cbrt`, a shift-derived power of two, or any ad-hoc guess — computed in the kernel instead of taken from the library.

**How to find it.** Read the seed library and enumerate what it provides, rather than assuming a visibility and searching for that, since its entry points need not be public to the world. Then search the algos folders for the tell and check whether each candidate calls the library or rolls its own.

### Rule 3 — Sizing is exact and local to its width

**A — Derived const width** · HIGH

**Context.** An entry is entitled to its level's own consts, and the caller already holds those in its types, so they are inferred rather than derived. A const generic whose value is computed from another width is a work width smuggled into a signature, and work widths belong in the scratch carrier, where each width sizes itself.

**Expected.** An entry takes its level's own consts and no others, including one it does not use. It may read its operands as much as it likes — limb length, magnitude, sign — because that is the matcher's job. An operation whose operand lengths are genuinely runtime has no const width axis and is legitimately slice-based.

**Disallowed.** A const parameter derived from another width. A const argument passed at a call site as a computed expression. Any need for a const, type or context beyond the level's consts and the operands themselves. And the inverse: a call site forced to invent a const or a type in order to reach a matcher, which means the matcher's signature is wrong rather than the caller.

**How to find it.** Scan the matcher files and the algos folders for const parameters beyond the allowed set, and for call sites passing computed const arguments.

**B — Build-max sizing on a concrete-width path** · HIGH · the recurring one

**Context.** Every value has a width, and a kernel often needs a work buffer wider than the value it is handed. The scratch carrier supplies those buffers, sized inside the impl where the width is concrete, so each width gets exactly the buffer it needs. The build-max family sits alongside it and sizes instead to the widest width the whole build configures. Reaching for the second where the first is available forces the widest tier's buffer onto a width that could have sized itself exactly, so a narrow value carries the stack cost of the widest one.

It recurs for two reasons that do not go away. The build maximum is always available and always compiles, while sizing through the carrier requires knowing its vocabulary, so it is the path of least resistance whenever a size is awkward to express. And every new algorithm that needs a wider buffer is a fresh opportunity for it, so the class regenerates as the tree grows rather than being fixed once. Nothing catches it either: the wrong size compiles, computes the right answer and passes every gate, so only reading finds it.

**Expected.** Where a concrete width is in scope, a buffer wider than that width is sized through the scratch carrier for that width alone. A buffer is named by its multiple of the width rather than by the function that wants it, so any algorithm needing that size can use it, and adding a new higher multiple when an algorithm needs one is expected rather than a defect. A buffer whose size depends on two axes rather than one can still be threaded from the entry that carries them, so needing more than the width is not a reason to reach for the build maximum. The build-max family is correct in one situation only: a blanket impl covering every width, which can neither name a computed length on stable nor carry the carrier's bound. A `const` context that cannot reach the carrier should still derive its length from the build maximum rather than freeze a literal.

**Disallowed.** A buffer length written as a numeric literal. A length derived from the build maximum. The build-max family itself, anywhere a concrete width is in scope. A cold path is not an exemption and neither is an escape route to somewhere else. The exemption requires that the bound be impossible rather than merely absent, and impossible means requiring it there would cycle — a bound simply not written is a defect, not an exemption.

**How to find it.** Enumerate the carrier's constructors and the build-max family from the module that owns them, then search the algos, matcher and type files for both, and for buffer lengths written as literals. For each hit, ask whether a concrete width is in scope.

**How to fix it.** Size the replacement for what the algorithm actually needs at that width, taken from the algorithm rather than from the length being replaced. Never map the old literal onto whichever constructor happens to match its value: matching lengths are not the same as a correct size.

**Example.** A fixed-length buffer is replaced by the carrier constructor whose size matches that literal. The lengths agree, so it compiles and the timings do not move. But the algorithm needed headroom that the literal happened to include and the nearest matching constructor does not provide, and precision quietly degrades on the default build. The lengths matching is not the same as the size being right.

### Rule 4 — The matcher chooses

**G — Matcher bypass and hardcoded engines** · HIGH/MED · a blind spot

**Context.** Where a function has a matcher, choosing the engine per cell is the matcher's whole job. A call naming one engine directly pins that caller to a single algorithm for every shape it will ever see. Such a call can be correct today and wrong tomorrow without anything changing at the call site: the moment an arm is added that the matcher would choose for some of those shapes, the direct call never reaches it and the new engine is dead for that path. No gate will tell you. The correctness gate cannot see which engine ran, and the benchmark only ever sees what is wired, so this is caught by reading or not at all.

Nothing brings the reviewer back to the call site. An audit that asks whether the code is correct today will pass it every time; only one that asks what happens when an arm is added will catch it.

**Expected.** Callers route through the matcher and act on its verdict, and an exact-scratch need is met from the algorithm's own carrier buffers.

**Disallowed.** A call naming a specific engine from outside that function's matcher, where the matcher has more than one reachable arm for the caller's shapes. A comment justifying such a call as the matcher's identical choice, which is a defect marker rather than a clearance: quote it, flag it anyway, and note that it needs rechecking whenever an engine joins that matcher.

**How to find it.** Take each matcher and enumerate its variants and the kernels they name. That list is the current set of engines and stays complete as engines are added. Then search the algos folders and the type files for direct calls to those kernels **and their in-place variants**, from outside their own matcher. A caller that needs exact scratch usually calls an in-place sibling whose name carries a suffix, so searching for the base names alone will miss exactly the callers this class is about.

**Example.** A caller needs exact scratch, so it calls one wide engine by name rather than going through the matcher. While that engine is the only arm, every audit passes. A second arm arrives for a different limb width, and the caller silently never reaches it.

**F — Policy hygiene** · LOW

**Context.** A matcher's arms are read by people to learn what runs for a given cell. An arm that can never fire makes the table claim a choice it does not actually make, so anyone reading it to understand the routing is misled.

**Expected.** Every arm is reachable for some cell, and every threshold has been validated rather than inherited.

**Disallowed.** A threshold that looks unvalidated or inherited, or one set so that an algorithm never engages. An arm that can never fire. An override arm reaching the same kernel as the arm it overrides.

**How to find it.** Read each matcher's arms in order and ask of each which cells reach it. Check that each threshold is recorded as having been validated rather than inherited. The audit reads that record; it does not re-run the comparison that produced it.

**M — Arms not ordered by least work first** · MED, and HIGH where the const key could have decided

**Context.** Arms are ordered by the least work needed to reach a verdict, so the cheapest discriminator that can settle a cell runs first. There are three costs. A decision made on the const key is free, folding per monomorphisation to one direct call and eliminating every unchosen arm. Cheap operand inspection comes next: sign, is-zero, limb length, or a single comparison against a const bound. Derived quantities are dearest — assembling a magnitude across limbs, raising a power of ten, a bit length, a division — all real work done before any algorithm has started. A runtime verdict survives compiling as a branch, so a cell it decides that the const key could have settled pays for that branch and keeps every arm alive in every monomorphisation, permanently.

**Expected.** Const-decidable cells are settled by const arms placed above the runtime arm, the runtime arm covers exactly its value-dependent region, and the tests inside it run cheapest first. A genuinely operand-dependent choice over a region the const key cannot settle is what the runtime verdict is for, and the canonical unused runtime-verdict variant is sanctioned. The question is never whether a runtime verdict exists, but whether any cell is doing more work than it needed to reach the same verdict.

**Disallowed.** A closure branching on something the const key already fixes. A runtime arm placed above const arms it therefore shadows. A runtime arm covering a whole width when only some of its scales depend on values. An expensive quantity computed before a cheaper test that would have decided first.

**How to find it.** Read every runtime-verdict arm in both layers, asking which cells the const key alone could decide and whether the tests inside run cheapest first. Correctness is unaffected either way, so no test will ever catch this and reading is the only route to it.

**How to fix it.** Three moves, and all three are needed. Hoist const-decidable cells into arms above the runtime arm. Narrow the runtime arm to exactly its value-dependent region. Order the tests inside it cheapest first. Arm order and arm width are different things, so the narrowing is still required where the ordering is already right.

**Example.** A runtime arm covers a whole width when only two of its scales actually depend on operand values. Every other scale under that width pays a branch to reach a verdict its const key already fixed.

**I — Single-cell fit and the neighbour-coverage gap** · MED · a core rule

**Context.** A gate belongs across the whole region where it wins. Fitted to the single benched cell where a regression showed up, it leaves the neighbours — adjacent scales and adjacent widths alike — on the wrong side of it, or cliffing at the boundary. The bench only samples the surface, so a gate snapped to a sampled point also leaves the unsampled band uncovered. Nothing here changes an answer, which is why it has to be caught by reading rather than by testing.

It ranks as a core rule because of how it escapes notice. A point-fitted gate looks like a successful fix: the cell that was measured recovers, the number moves, and the work is closed. The neighbours were never measured, so nothing reports them, and no one returns to a question that appears to have been answered. The defect is created by the act of fixing something else.

**Expected.** A gate is stated as a condition correct at every scale and width in its region, placed at the bisected true crossover, with the straddling cells and their immediate neighbours validated in both scale and width. A blanket fix with no gate at all is fine, as is a mathematical pin correct at exactly one value.

**Disallowed.** A point-range arm. A threshold introduced or moved to exactly a benched value with no continuous range either side. A comment justifying an arm by naming one benched cell.

**How to find it.** Read the matcher arms for ranges whose ends coincide and for bounds that fall on sampled cells, then read the comments near them for a single named cell.

**Example.** A regression shows up at one benched scale and a gate is placed at exactly that value. That cell recovers. One scale either side, and one width either side, are untouched or land on the wrong side of the boundary.

**L — Constant lookup keyed on a runtime scale** · MED

**Context.** A constant looked up by the const scale is resolved in const context and folds to the single entry that monomorphisation needs. One looked up by the runtime working scale is selected afresh on every call. Using the runtime-keyed form where the const scale is in scope therefore pays at run time for a selection that could have been baked into the binary, which is precisely the cost the table exists to remove. Like Class I it changes no answer, so no test will catch it.

**Expected.** Where the const scale is in scope, the constant is looked up by that const. The runtime-keyed form is used only on the escalation path, where the working scale genuinely is not known at compile time.

**Disallowed.** A runtime-scale-keyed lookup inside a kernel or accessor that has the const scale in scope, outside that escalation branch.

**How to find it.** Each constant has more than one lookup family, so enumerate them from the tables folder and sort them into those keyed on the const scale, including any checked variant, and those keyed on the runtime working scale. Then check every call site of a runtime-keyed one for a const scale in scope.

### Rule 5 — Two absolute invariants: no heap, no state

These two are absolute, and they are absolute for the same reason. Both defects work. A heap allocation computes the right answer, and a cache computes it faster. Neither fails a test, neither looks wrong at the call site, and an ordinary review has nothing to react to, because the code is correct. That leaves a reviewer's judgement no signal to act on, which is why both classes flag every site and neither offers a case-by-case call. The discretion is designed out because there is nothing for it to work from.

**J — Heap allocation on the compute path** · HIGH

**Context.** The crate is pure stack and builds without an allocator at all, so any heap on the compute path — anything reachable from a runtime value computation — breaks that outright.

**Expected.** Every working buffer on the compute path is an inline array or a const-initialised static, and a slice or an inline array allocates nothing at all. Tests, benches, build scripts and doc examples are not the compute path, so heap in them falls outside this class rather than being permitted by it.

On the compute path all heap is denied, with exactly one exception: the boxed trait object in the dynamic-dispatch facade. It stands because it is signed off, not because it is unavoidable. Only the project owner grants an exception, and none will be granted, because the crate is `no_std` and exists to stay on the stack. So the question at a suspect site is never whether the allocation is defensible, but whether it is signed off — and only that one is.

**Disallowed.** `Vec`, `vec![…]`, `.to_vec()`, `Box`, `Rc`, `Arc`, `VecDeque`, `String`, `collect::<Vec<_>>()`, and anything reached through `alloc` or `std::collections`, anywhere on the compute path. The facade's exemption covers the box and nothing else. Dynamic dispatch is the extension mechanism for a whole family of heap-using patterns, so permitting the one box does not permit what dynamic dispatch is normally reached for: inside the facade, vectors, threading, reference counting and incidental owned strings are flagged exactly as anywhere. The exception is a door, not a room.

**How to find it.** The compiler finds this better than a search can. Run `cargo check --no-default-features --lib`, then repeat it with the widest width feature: that build has no allocator, so any heap it compiles is a hard compile error rather than a lint, and `--lib` keeps test modules out of the build so there are no false positives. Search for the shapes afterwards only to catch heap sitting behind a feature that build skips, bounding test blocks by the `mod tests {` line rather than by a `#[cfg(test)]` attribute line, which is not reliably on a line of its own.

**K — Mutable or shared state, meaning a cache anywhere** · HIGH

**Context.** The crate is stateless, and that alone is what buys `Send`, `Sync` and re-entrancy for free rather than by argument — which is why this ranks HIGH, since one cache costs all three at once. The line to hold is *when* a value is produced, not whether it is stored: a table computed at build time and never written afterwards is read-only data, while a value computed at run time on first use and kept for later calls is a cache.

**Expected.** Every function is a pure function of its inputs and recomputes on each call. A fixed precompute is lifted to compile time, and anything else is recomputed on the stack.

**Disallowed.** `thread_local!` and `static mut`. A `static` holding `RefCell`, `Cell`, `Mutex`, `RwLock`, `OnceCell`, `OnceLock` or an atomic and used as a cache or accumulator. A `lazy_static!` or `once_cell` memo, or any other memoization. Relocating a precompute into a mutable static.

**How to find it.** Search for those shapes and ask of each whether its content is fixed at build time or depends on prior calls.

### Housekeeping

**Encoding** · LOW

**Disallowed.** Mojibake from UTF-8 read as Latin-1 (`â”€`, doubled blank lines). Re-save as UTF-8.

## Read-only protocol (copy into the agent prompt)

- Inspect with Grep, Glob and Read. Make no Edit or Write to any source or config file, and run no git command.
- `cargo check` is permitted where a rule calls for it, because it compiles and changes nothing. That is the only cargo command allowed: not test, not bench, not build.
- The one write allowed is the report, at `research/<date>_architecture_audit.md`, in a git-ignored directory.
- No background jobs and no subagents. Keep CPU use modest, and finish within the run.

## Coordinator side

- Commit work in progress before spawning, because an agent in the main checkout can reset the tree.
- Re-dispatch the same agent for a second pass once fixes are in. It re-scans the current tree, confirms what is now fixed and reports what remains, so tell it which findings landed and it will not re-flag them.
- Reconcile before fixing: sanctioned in-progress work and parked pilots are kept rather than repaired.
- Golden-gate after fixes with `cargo test --release --features wide,x-wide,xx-wide,macros --test ulp_strict_golden`, then the full `cargo test`. Most of these fixes are behaviour-neutral, and golden is what proves it. Check the case count as well as the delta, comparing it against the previous run's count: a run that grades fewer cases than the one before it can report a zero delta while proving nothing.

## Launch-prompt skeleton

> ARCHITECTURE AUDITOR (read-only) for `decimal_scaled`, CWD `<repo>`. Report architecture violations and dead or stale code.
> LOAD the `architectural-review` skill and obey its read-only protocol. Work its structure section, then its five rules and the classes A–N beneath them, from the skill itself rather than from a summary, and carry no identifier into a search from this prompt.
> FIRST read `CLAUDE.md`, `docs/ARCHITECTURE.md` and the scratch-carrier module. Then list the tree and check it against the skill's map; if they disagree, the tree wins and the map is a defect to report.
> THEN scan `src/`, including the macro folder, and `Cargo.toml`.
> DELIVERABLE: write the report to `research/<date>_architecture_audit.md` as a summary table of class, count and highest severity, followed by grouped findings. Then return the counts per class and the HIGH items with `file:line`.
