---
name: dispute-resolution
description: Use when two parties disagree about a fact, a measurement, or what code does — validates methods rather than figures, so disagreements resolve instead of hardening
---

# Dispute resolution

For settling a disagreement about what code does, what a number means, or what a measurement
shows — between two agents, an agent and a coordinator, or an agent and a human.

## The rules

**1. Validate the METHOD, not the figure.** Facts and figures may be presented. They are never
*trusted* until the method that produced them has been validated by both sides. A fact offered
without its method is an assertion, not evidence — say so, and set it aside until the method
arrives. Two numbers cannot argue with each other; two methods can be compared, and one is
usually wrong in a way both parties can see.

**2. Neither party trusts the other.** A prohibition, not a courtesy, and it runs both ways.
Anything stated as fact gets verified before it is built on, by whoever receives it, whatever
their role. Trust flows from a validated method, never from who said it — position, role, and
having been right last time confer nothing.

**3. Disagreement is legitimate when facts conflict.** Neither side must yield. Each owes: state
the case plainly, substantiate so someone else can check, share the method.

**4. Aim for agreement.** Escalation is not the endpoint. Argue it through until the evidence
persuades one side. Most disputes about what code does or what a number means ARE resolvable by
looking harder. **Being persuaded is the goal, not a concession** — say so plainly when it
happens.

**5. Hunt the hole, in either account.** Never restate your position more firmly. Look for what
is missing or unsound in *either* side and narrow the space of explanations — test hypotheses
about the disagreement itself, not just its subject. **Beware the near-miss:** narrowing
generates coincidences, and a near-miss reads like a match when you want the question closed.
State the arithmetic, not the impression.

**6. Record the outcome.** Every dispute leaves a written record, resolved or not.

**7. The human resolves genuine deadlocks** — or, with their agreement, a tie-breaker does.
When both sides have argued properly and still disagree, or the question is a judgement call:
scope, priorities, what ships. Backstop, not route.

## What a fact must carry

- **Source** — file, ref, artifact, run ID, commit.
- **Field** — which rows, which column, which subset. *Name the rows.* Unstated selection is the
  most common cause of two correct extractions disagreeing.
- **Operation** — exactly what was computed.
- **Its own doubts** — the assumptions that could be wrong.

## Shared data, and when to re-run

**Both sides may use the same data, provided the method that generated it is known.** Sharing an
artifact is not trusting a figure — the method is still what gets validated.

**Do not re-run when the preconditions are unchanged.** A repeat under the same conditions
verifies nothing: it lands in different circumstances, returns different numbers, and adds a
third reading to argue about.

**Do re-run when there is a reason**, and say which: you suspect the result is environmental
rather than real; the method itself has changed; or a precondition has moved.

The split that keeps this cheap: the **production** of data is validated as a *procedure* —
stated fully enough that both sides agree it would reproduce — not by re-executing it. The
**derivation** over that data (which rows, which field, what arithmetic) is re-derived by the
other side every time, because it costs nothing and it is where the errors live.

## The record

Not the transcript — the distillate. Without it the same argument is had again in a month, by
people who cannot see why it was settled.

1. **Raised** — the competing claims, and who held which.
2. **Methods** — how each side got its figures.
3. **Argued** — the objections put to each side, including the ones that turned out wrong.
4. **Agreed** — what is now taken as established.
5. **Why** — which method was validated and which was discarded, and on what evidence.

Record what was **discarded and why**, not only what survived; a conclusion without its discards
invites the same wrong turn later. Record **unresolved** disputes too: the deadlock, what each
side holds, what evidence would settle it, and if a human adjudicated, what they decided *and on
what grounds* — an adjudication with no reasoning is a fact with no method.

**Put it in a decision log** — one file per dispute, default
`decisions/<iso-datetime>_<argument>.md`, named for what was argued. Follow the repository's own
convention for decision records where it has one.

Not in code comments. Comments earn their place explaining the code they sit beside; a dispute
record is a different kind of thing and putting it there overloads them. And not in a chat log —
that is a transcript, not a record.

## The tie-breaker

A genuine deadlock can be broken by a fresh party instead of the human — **but only with the
human's agreement**, given either for this instance or as a standing blanket permission. Without
that, deadlocks go to the human.

It is spun up new, having taken no part in the argument, and is handed **the dispute record** —
which is what that record is for.

**Nobody tells it how to work.** It decides for itself what skills to load, how to read the
codebase, and what questions to ask. Being handed an approach means inheriting a framing, and
the framing is half of what is in dispute.

It may put any question to either party, and may ask either to carry out any task that would
help resolve the matter, within the constraints those parties already operate under.

**It is read-only.** It does not fix, refactor, or land anything. Its single output is a
judgement, with the reasoning that produced it, recorded like any other outcome — a ruling with
no reasoning is a fact with no method.

## The shape of a good resolution

Both extractions correct; the disagreement lives in an unstated difference. One side finds a
better objection to its own position than the other had. Both concede something. Neither is
overruled.

Self-correction *after* winning the point is the signal the method is finding truth rather than
producing a negotiated settlement.

## Preventing them

- State the symptom and the constraints, never the diagnosis.
- Do not pre-supply counts, line numbers or measurements.
- Say explicitly that you may be wrong and are to be checked.
- Ask to be corrected where your framing was wrong — and mean it.
- A figure that genuinely helps orient comes with its method and its doubts, and carries no
  weight until that method is validated.
