---
name: cleanroom
description: Use when an algorithm must be implemented and the available reference is under an incompatible licence — establishes when a wall is actually needed and how to run one that would survive scrutiny
---

# Cleanroom

> **Not legal advice.** This is a working process, not a legal opinion. Where a decision turns on
> facts or jurisdiction, get a lawyer. The value here is that the process produces evidence; only a
> lawyer can tell you whether that evidence is enough.

## The rules

**1. The wall is the LAST resort. Exhaust the cheaper routes first.** In order: (a) the published
paper, standard, or textbook; (b) **a permissively licensed implementation** — MIT, BSD, Apache,
BSL — which you may simply read, subject to its attribution terms; (c) your own derivation from the
mathematics; (d) only then a wall. Most work stops at (a) or (b). A two-role wall is expensive and
slow, so invoking one you did not need is its own kind of failure.

**2. The question is whether YOUR OUTPUT is a derivative work — not whether you read the source.**
Copyright restricts copying, distribution, adaptation and communication to the public. **It does not
restrict reading.** Reading a lawfully obtained copy is not a restricted act, and an open-source
licence additionally grants copying and modification outright. What creates liability is shipping
something substantially similar IN PROTECTED EXPRESSION under an incompatible licence — filtration
strips ideas, functionally compelled elements and public-domain material first, so a great deal of
similarity is fine. So input purity is **evidence**, not
compliance; output dissimilarity is what actually decides. Weight your effort accordingly — the
instinct is to spend everything on who read what, and that is backwards.

*(Where reading is ITSELF prohibited — a trade secret received in confidence, a licence forbidding
disassembly, source obtained without any licence — the calculus changes completely and the wall
becomes mandatory rather than optional. Know which situation you are in before you start.)*

**3. Record the algorithm, not the code.** The recorder's output is a specification a competent
implementer could have written from first principles: what is computed, why it is correct, what it
costs. Not identifiers, not statement order, not loop shape, not comments, not constants without
their derivation.

**4. Clean contexts, and the dispatcher is inside the wall.** Each role is a fresh agent whose
context holds nothing but its own brief. The implementer's context must contain nothing that crossed
the wall — no excerpt, no paraphrase, no "the reference does X here". That includes whatever the
person *dispatching* them has learned: **a coordinator who has read the recorder's findings and then
writes the implementer's brief is the breach**, and it is the easiest one to commit because it feels
like ordinary handover rather than leaking. Compose the brief from the specification alone, and keep
it so you can show that is all it contained.

**5. The specification is REVIEWED before it crosses, by someone who is not the recorder.** Not
read — reviewed, against stated criteria, producing a verdict that it is cleanroom-ready. The
recorder judging its own output is the one reviewer whose position is structurally compromised, and
"I checked it carefully" is not a finding. The reviewer need not be a lawyer and need not be a
third agent — a person working through the checklist satisfies it — but they must not be the party
that read the source, they must have the cited papers to hand, and criteria 6 and 7 need enough
grasp of the mathematics to judge what the algorithm genuinely fixes. **A reviewer who cannot
assess those two should record that, not pass them by default.** The same review applies to every
answer that later crosses the wall. See **The specification review** below for what the check
actually consists of.

**6. The wall is one-way and value-only.** The recorder may answer questions and supply test
vectors. The moment an answer describes the reference's *structure* — "it does the shift before the
compare" — the wall is breached and the work is tainted. One exception, and it is not an answer:
where a discrepancy cannot be resolved without describing the reference, the recorder amends **the
specification** in algorithm terms and the amended specification goes back through the review.
Nothing crosses the wall except values and a re-reviewed specification.

**7. Compare the output before shipping.** The already-tainted recorder compares the finished
implementation against the reference and returns a **verdict only** — "function F is too close,
rewrite it" — never structural feedback. This is the only mechanism that detects convergence, and
convergence is what would sink you: independent creation remains a complete defence in law, but the
greater the similarity the less likely anyone is to believe it. Control the direction; do not
prohibit the check.

The verdict channel is a **rejection oracle, not a construction oracle** — it can only push away
from the reference, never toward it, which is what makes it safe. It still leaks a little per
round, so bound the rounds: a verdict names a **location and a verdict, never a property**; it
goes through the reviewer like any other cross-wall traffic; each round is logged with its number;
and a function still failing on a third rewrite is a suspected-convergence event under rule 9, not
a fourth iteration.

**8. A true implementation, not a translation.** The implementer must be able to say why each step
exists before writing it. "The specification says so" is not a reason; it is transcription with
extra steps. See below — this is the rule that is actually hard.

**9. On suspicion of a breach: STOP, notify, tear down, rebuild.** An implementer who suspects or
discovers the wall is broken stops work **immediately** — not at the end of the function, not after
finishing the thought — and notifies the dispatcher. **Suspicion is the trigger; certainty is not
required, and waiting to be sure is itself the failure.** The dispatcher then puts a remediation in
place so the same path cannot leak again, **removes the contaminated cleanroom entirely**, and
stands up a fresh one that restarts the work from the specification stage.

Nothing is salvaged from a broken wall — not the implementation, not the "clean parts", not the
half-finished function. The point of the process is being able to say where every line came from,
and after a breach you cannot say that about any of it. Salvage is what turns a contained incident
into a contaminated deliverable. Record the breach, its cause, and the remediation: a disclosed and
remediated breach is survivable; a concealed one is not.

**Reporting a suspicion that turns out to be nothing is a cost-free good outcome — say so out
loud.** A trigger set at suspicion, with total loss of the work as the consequence, pulls against
itself: an implementer under pressure has every incentive to resolve an ambiguous doubt as
"probably fine", which is the exact failure the rule exists to prevent. Nobody is weighing their
own work against disclosure, and a false alarm costs a re-run.

**The breach record is itself tainted material and sits on the recorder's side of the wall.** When
the replacement cleanroom is stood up the instinct is to warn it — "last time the implementer was
told the reference does X" — and that warning re-leaks exactly the content the teardown was for.
The replacement's brief is composed from the specification alone, as rule 4 requires.

**10. Record only what is true, and keep it.** A process that exists on paper and is not enforced is
**worse than none** — it evidences awareness of the risk without mitigating it, which is precisely
what sank the defendants in the leading modern case, whose clean-room procedures were found to have
had little effect, because those controlling the defendants took no meaningful steps to make the
clean-room procedures work in practice. The record is the deliverable.

## The recorder

Reads the reference. Writes a specification. Never writes the implementation.

**The specification must carry:**

- **The mathematical statement** — what is computed, and the identity or theorem it rests on.
- **Why it is correct** — the invariant, and the convergence or termination argument.
- **Preconditions** — domain, ranges, what the caller must guarantee.
- **Cost** — the complexity, and *which operation dominates it*. An implementer who does not know
  what the algorithm is trying to avoid will not avoid it.
- **Accuracy** — exactness or error bounds, and where they come from.
- **Genuinely special cases** — those that are mathematically special. Not "there is a branch here".

**The specification must NOT carry:**

- Identifier names, statement ordering, loop or branch structure, or any comment from the source.
- A constant without its derivation. Derive every constant or compute it from its definition; never
  transcribe a numeric literal. Legally a bare number is unlikely to be protectable, but
  evidentially it is the worst thing that can cross, because it is a **fingerprint** — it cannot be
  explained by convergent derivation, since there is no derivation. Rights holders have historically
  seeded fictitious data precisely to detect copying.
- Choices that are implementation rather than algorithm — buffer layout, when to fuse, how to split
  precision. Fixing those in the specification hands over the source's shape.

**The taint test, applied to every line:** *could this have been written from the paper alone, by
someone who never saw the source?* If not, reduce it to the idea behind it or drop it.

**The second test:** *does this constrain a decision the algorithm does not?* If two correct
implementations could reasonably differ and the specification fixes the choice, that choice came
from the source.

## The specification review

Before the specification crosses, it is reviewed against these criteria by someone who has not read
the reference. The output is a **verdict** — cleanroom-ready, or not — recorded with the reviewer's
identity and the date. Work through it item by item; an overall impression is not a review.

**Why an uncontaminated reviewer rather than a careful one:** criterion 6 below asks what someone
who never saw the reference could have written from the published sources. Given those sources, the
reviewer *is* that person — they answer it by introspection, where anyone who has read the
reference could only infer it. That is the argument for an *uncontaminated* reviewer specifically,
over and above the general point that the recorder cannot mark its own homework.

1. **Provenance stated.** The reference is named with its exact version or commit and its licence,
   and the reason the wall was needed at all is given — that is, why rule 1's cheaper routes did not
   suffice.
2. **No code-shaped identifiers** — no token that reads as a function, variable, type or file name.
3. **No prescribed ordering or decomposition AT ALL** — the specification says what is computed, not
   in what sequence or in how many functions.
4. **No passage that explains THE CODE rather than THE ALGORITHM.**
5. **Every constant derived.** Each numeric value arrives with the derivation that produces it, or
   with its mathematical definition. A bare literal fails this item on its own.
6. **The taint test, per line.** Could this line have been written from the published sources alone,
   by someone who never saw the reference? If not, it fails.
7. **The second test, per decision.** Does the specification fix a choice the mathematics leaves
   open? If two correct implementations could reasonably differ and one is prescribed, that
   prescription came from the source.
8. **No test data lifted.** Any vectors present are generated or constructed, not taken from the
   reference's test files.
9. **Sufficient to implement from.** The specification answers what an implementer would otherwise
   have to ask. Stripping past that point does not make the wall safer — it moves the traffic into
   the question channel, which is where walls actually break.

**A failed item sends the specification back to the recorder** — the reviewer does not fix it. The
reviewer has not read the reference and so cannot know what the corrected line should say; editing
it would substitute a guess for the algorithm and disguise the failure as a pass.

**A review that has never rejected anything is not, on its own, evidence of a clean
specification.** One passing first time shows nothing either way. But across a run of them, an
unbroken record of passes is more likely to mean the criteria are not being applied than that the
recorder is unusually careful — so if several pass untouched, check the review is biting.

## The implementer

Reads the specification. Reads the cited papers. Never the source.

**Rule 8 in practice.** Before writing a step, state its purpose in your own terms. If the only
justification available is that the specification lists it, you do not have the algorithm yet — ask
what it is *for*. A specification describes an idea; it is not a script.

**Three tests a real implementation passes and a transliteration fails:**

- **You can derive a variant.** Reorder the loop, fuse two steps, change the precision split — and
  say whether it is still correct and why. Someone transcribing cannot, because they do not know
  which parts are load-bearing.
- **You hit the stated cost and accuracy.** If your code does not achieve the claimed complexity and
  error bound, you implemented something else that returns similar answers. This is the failure mode
  where a transliteration looks finished and is not: the shape survives, the property does not.
- **It is written in the host's idiom.** Types, sizing, allocation and control flow are what this
  codebase would use for this problem. A structure that looks foreign to its neighbours is usually
  the reference's shape showing through.

## Verifying without leaking

The implementer needs to know when the output is wrong. That channel is one-way and carries
**values only**.

- **Generate test vectors by running the reference.** A program's output is not, in general, covered
  by the copyright in its code — the exception being where the program copies parts of *itself* into
  its output. Values are facts.
- **Never copy the reference's test files.** Those are source under the source licence, and a
  curated set of adversarial cases is exactly the selection and arrangement in which originality can
  subsist. In the UK a database right can be infringed by extracting a substantial part **even where
  every individual value is an unprotectable fact** — a point a US-only analysis misses.
- A failure is reported as *"input X gives Y, expected Z"*. Never as *"the reference handles this
  by …"*. Diagnosing it is the implementer's job.
- If a discrepancy cannot be resolved without describing the reference, that is a **gap in the
  specification**: the recorder fixes the specification, in algorithm terms, and it goes back
  through the reviewer.
- Prefer vectors that probe the reasoning — boundaries, degenerate inputs, the cases the correctness
  argument turns on — over bulk random values, which confirm agreement without locating disagreement.

## The record

Keep enough to demonstrate the wall existed AND was enforced.

1. **Who read what, and when.** The recorder; the reference's **exact version or commit** and its
   licence; the dates. The implementer, **the brief they were given verbatim**, and the fact that
   nothing else was placed in its context. A brief you can produce is evidence; a claim about what
   you told someone is not.
2. **Log what is provable.** Record that *the implementer did not consult the reference while
   authoring*. Do **not** record that it *had no prior access* — for a model-based implementer that
   is not provable, and asserting it discredits the whole record if challenged. Compensate with
   derivation records and a measured output comparison instead.
3. **The specification as handed over**, and **its review verdict** — who reviewed it, when, and the
   result against each of the eight criteria. Keep the rejected versions too: a specification that
   went back twice before passing is stronger evidence than one that passed first time.
4. **The cross-wall questions and answers, verbatim.** This is where a breach would show, so an
   empty log is less credible than a real one.
5. **The citations** — papers, standards, textbooks, and any permissively licensed implementation
   relied on under rule 1(b), with its attribution.
6. **The output comparison** and its verdict.
7. **Anything the recorder deliberately dropped** as expression rather than idea.
8. **Every breach or suspected breach** under rule 9: what happened, when work stopped, what was
   discarded, the remediation put in place, and the identity of the replacement cleanroom. A
   disclosed and remediated breach is survivable; the record is what makes it so.

Store it with the implementation. Keep it for the life of the code and for at least six years
after its last distribution — limitation periods run from the act, so the clock starts at each
distribution rather than when the code is removed. Six years is a working default for England and
Wales; confirm it for any jurisdiction you actually ship into.

## What this does not cover

**Patents.** A clean room is a copyright and trade-secret device and gives **zero** protection
against a patent claim. Independent creation is no defence to infringement of a valid patent. If the
algorithm may be patented, that is a separate question and this process does not touch it.

## Failure modes

- **Invoking the wall when a paper or a permissive implementation would have done.** The most common
  and the most expensive.
- **The specification is code in prose.** Every step in order, every branch preserved, variables
  renamed. Fails the taint test on nearly every line.
- **Pasting reference source into a shared channel** — a conversation, an issue, a commit message.
  The wall is defined by where the source text goes.
- **"I just checked one thing."** One look ends the wall. Rule 9 applies: stop, notify, tear down,
  restart. The temptation is to keep the work and note the lapse, which is exactly the salvage that
  rule 9 forbids.
- **Carrying on to a natural stopping point** after noticing a breach. Every line written between
  noticing and stopping is written by someone who has seen the reference, and it is the hardest
  contamination to argue away afterwards, because the timestamps show it.
- **Debugging through the wall.** It usually breaks under a failing test, not while writing the
  specification.
- **The implementer optimising toward the vectors** rather than the algorithm — passing the tests
  while missing the property the algorithm exists to provide.
- **A policy nobody enforces.** See rule 10. This is the failure the case law actually punishes.

---

# Notes on the law behind the rules

> **This section is background detail *about* the skill, not part of the skill.** The rules above
> are usable without it. This records what each one answers to, so that a rule is not weakened by
> someone who cannot see why it exists. Still not legal advice; jurisdictions differ and the leading
> authorities are mostly US, with one recent English case. **Verify any citation before relying on
> it** — these are given so the reasoning can be traced, not as a substitute for advice.

**Copyright protects expression, not ideas.** An algorithm as such is not protected; its expression
is. Directive 2009/24/EC (the Software Directive), Art 1(2): ideas and principles underlying any
element of a computer program, including those underlying its interfaces, are not protected.
*Met by rule 3 and the taint test: the specification carries the idea and is stripped of the
expression.*

**Copying is proved circumstantially, by access plus substantial similarity.** Where similarity is
great enough, a claim of independent creation is unlikely to be *believed* as a matter of fact — but
striking similarity alone does not establish copying absent some evidence of access: *Selle v Gibb*,
741 F.2d 896 (7th Cir. 1984), where the plaintiff failed on exactly that point and judgment
notwithstanding the verdict was affirmed against him. Independent creation remains a complete
defence in law; the risk is evidential, not doctrinal. *Met by rule 2 — the output side is where the
case is decided — and by rule 7, the only step that can detect convergence before it ships.*

**Courts filter before they compare.** The abstraction-filtration-comparison approach —
*Computer Associates International v Altai*, 982 F.2d 693 (2d Cir. 1992) — removes ideas, elements
dictated by efficiency or by external constraints, and public-domain material, before asking whether
what remains is similar. *This is why functionally compelled similarity survives scrutiny, and why
rule 8's "written in the host's idiom" matters: idiomatic differences are real differences.*

**Reading a competitor's code to reimplement is not automatically infringement.** In the US,
intermediate copying undertaken to reach unprotected functional elements has been held fair use —
*Sega Enterprises v Accolade*, 977 F.2d 1510 (9th Cir. 1992); *Sony Computer Entertainment v
Connectix*, 203 F.3d 596 (9th Cir. 2000) — and reimplementing an API's declaring code was held fair
use in *Google v Oracle America*, 593 U.S. 1 (2021). These are US doctrines and do not transplant
wholesale. *Relevant to rule 1: they bear on how much risk the cheaper routes actually carry, not
on whether the wall is run correctly.*

**Reading is not a restricted act.** Copyright restricts copying, distribution, adaptation and
communication to the public. Reading a lawfully obtained copy is none of those. An open-source
licence then grants more on top: LGPL 2.1 §1 permits copying and distributing verbatim copies of the
source, and §2 permits modifying it. (§0 is definitions and scope, and grants nothing — a common
miscitation.) *Met by rule 2, which relocates the question from input to output.* Where reading IS
prohibited — a trade secret received in confidence, a licence bar, source obtained without any
licence — the classical clean-room canon applies instead and the wall becomes mandatory rather than
optional.

**The founding episode is our situation, not the other one.** The clean-room BIOS
reimplementations of the early 1980s — Compaq and Columbia Data Products, then Phoenix
Technologies commercially in 1984 — are usually cited as the origin of the two-team pattern. What
is worth knowing is *why* they ran a wall: IBM had **published** the BIOS assembly listings in its
PC Technical Reference Manual, so the source was lawfully readable by anyone. The wall existed
because *copying* it would infringe, not because reading it was barred. That is structurally the
same position as reimplementing from a copyleft source, differing only in that an express licence
adds rights on top. *Supports rule 2: the wall answers an output question even when the input is
entirely lawful.*

**A narrower point often overstated:** LGPL 2.1 §6 is a *condition on distributing a combined work*
— your terms must permit modification for the customer's own use and reverse engineering for
debugging **those modifications**. It is not a general grant of a right to reverse engineer.

**The copyleft boundary is drawn around the library.** LGPL 2.1 §0: a "work based on the Library"
contains the library or a portion of it, verbatim or modified. §5: a "work that uses the Library" is
outside the licence's scope in isolation. The LGPL is more permissive than the GPL only in the
*linking* dimension — for a reimplementation that distinction does little work, because if the
output is a derivative of the source then copyleft attaches either way. *Met by rule 2.*

**A program's output is generally not covered by the copyright in its code** — the GNU project
states this directly, the exception being where the program copies parts of *itself* into its
output, which is why Bison needed an explicit exception for its parser skeleton. *This is what makes
generated test vectors safe.*

**Facts are not protectable, but their selection and arrangement can be** — *Feist Publications v
Rural Telephone Service*, 499 U.S. 340 (1991), which found a telephone directory's listings
unprotectable as facts while confirming that originality can subsist in selection, coordination and
arrangement — and Rural had seeded fictitious listings precisely to detect copying, which Feist
reproduced. And in the UK a sui generis database right — Directive 96/9/EC, in the UK the Copyright
and Rights in Databases Regulations 1997 (SI 1997/3032) — can be infringed by extracting a
substantial part **even where every individual value is an unprotectable fact**, a point a US-only
analysis misses. *Met by the rule to generate vectors and never copy the reference's test files: a curated
adversarial test set is precisely a protectable selection.*

**A policy that is not enforced can be found to have had little effect.** In *IBM UK v LzLabs*
[2025] EWHC 532 (TCC), O'Farrell J, 10 March 2025, the defendants had clean-room procedures and
the court nonetheless found against them: on the reported accounts, those controlling the
defendants took no meaningful steps to ensure the procedures were effective in practice, and the
procedures degraded over time. Nothing here is quoted — the judgment runs past 250 pages and the
phrasing circulating in commentary has not been traced to it. *Met by rule 10 and by the record —
the enforcement evidence is the deliverable, not the policy document.*

**A wall alone has not always been what carried a case.** *NEC v Intel*, 10 U.S.P.Q.2d 1177 (N.D. Cal. 1989) succeeded
on the wall together with an argument that the residual similarity was functionally compelled, and a
patent licence. Notably the clean room there operated as a **cure**: NEC's engineers had already
seen the reference, and the wall was built afterwards to redo the work. *Met by rules 5 and 7 — the
screening and the output comparison are what turn a wall into evidence rather than an assertion —
and it is the authority for prior exposure not being automatically fatal.*

**Prior exposure establishes access, which is one element and not the conclusion.** Independent
creation remains a complete defence, though a claim of it is unlikely to be believed where
similarity is great enough. For an implementer that
cannot credibly claim never to have encountered a widely mirrored source, a non-access claim is
unprovable and, if challenged, discredits the record that contains it — the 2026 dispute over an
LGPL library relicensed to MIT after an agent-assisted rewrite is the live example, where a low
measured code-similarity figure was rebutted on prior exposure. *Met by the record's rule to log
non-consultation during authorship, which is provable, and never non-access, which is not.*

**Constants are a fingerprint rather than a copyright question.** A bare numeric value is unlikely
to be protectable, but it cannot be explained by convergent derivation, because there is no
derivation. Rights holders have historically seeded fictitious entries specifically to detect
copying. *Met by the requirement that every constant arrive with its derivation.*

**A contaminated artefact is not salvageable; a rebuilt one is clean.** In *Altai* the version
written with knowledge of the plaintiff's code was held infringing, while the version rewritten by
programmers walled from it survived. In *NEC v Intel* the cure was an independent contractor
redoing the work from specifications alone. In both, the contaminated work product was replaced
rather than patched. *Met by rule 9's teardown — the discarded thing is the implementer's output,
not the specification: the boundary NEC drew explicitly, and which Altai is consistent with.*

**Clean-room design gives no protection against patents.** Independent creation is not a defence to
infringement of a valid patent. *Stated separately above because it is a limit of the technique, not
a rule within it.*

**Licence terms and versions change over a project's life**, and the version in force when the
reference was read is the one that governs. *Met by recording the exact version or commit alongside
the licence.*

**Limitation periods run from the act, not from the last time anyone looked.** *Met by retaining the
record well beyond the code's last distribution; the applicable period is jurisdictional and worth
confirming.*
