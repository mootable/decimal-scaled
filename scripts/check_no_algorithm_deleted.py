#!/usr/bin/env python3
"""Fail a diff that DELETES AN ALGORITHM, or fixes a precision defect without pinning it.

This exists because the architectural rules were read, the review was run, and the
verdict was still wrong. On 2026-09-03 a diff replaced two computing bodies with
one-line delegations. It removed no file and no enum variant, so a file-tree
comparison, an enum-variant diff and every check in the architectural-review skill
all passed. The routed path lost its faster algorithm; the whole-surface
improvement fell from +20.4% to +14.3%, with individual cells 10.75x slower.

A grep cannot talk itself into "it was only a duplicate shell". That sentence is
what waved the deletion through, and it is why this file is a script and not a
paragraph in a skill.

    python scripts/check_no_algorithm_deleted.py <base>..<head>
    python scripts/check_no_algorithm_deleted.py            # defaults to HEAD~1..HEAD

Exit 0 = clean. Exit 1 = at least one FAIL. Findings print with file and hunk.

THE THREE CHECKS
  1 DELETED COMPUTATION  a hunk whose removed lines compute and whose added lines
                         are a delegation. This is the one that would have caught it.
  2 NET COMPUTATION LOSS a diff removing computation from algos/macros with no new
                         kernel file to receive it.
  3 UNPINNED PRECISION   a commit claiming a correctness/precision fix with no
                         change under decimal-scaled-golden/lead/. A precision
                         defect is not fixed until the input that exposed it is in
                         the golden suite -- the `hypot` 8-13 ULP error survived
                         precisely because nothing pinned its inputs.

Every check reports the evidence it matched, so a reviewer can judge rather than
trust the exit code. A finding is not automatically a defect: a genuine
consolidation may legitimately move computation into a kernel. The script's job is
to make that a DECISION SOMEONE STATES, not an omission nobody notices.
"""
from __future__ import annotations

import re
import subprocess
import sys

# Tokens that mean a line performs mathematics rather than routing. Deliberately
# tuned to this crate's vocabulary: a generic "does it compute" test would drown
# in false positives.
COMPUTE = re.compile(
    r"\b("
    r"working_scale|to_work_|one_agm|zero_agm"
    r"|mul_agm|div_agm|add_agm|sub_agm"
    r"|sqrt_fixed|cbrt_fixed|log1p_fixed|ln_fixed|exp_fixed|atan_fixed|sin_fixed|cos_fixed"
    r"|round_to_storage|scale_by_k|bit_length"
    r"|mul_schoolbook|karatsuba|newton|isqrt|icbrt"
    r"|for\s+\w+\s+in\s+0\.\.|while\s+"
    r")",
    re.I,
)

# Tokens that mean a line merely routes.
DELEGATE = re.compile(r"(::dispatch(::<|\()|_dispatch(::<|\()|::resolve(::<|\())")

# Where algorithms are supposed to live, and where they are not.
ALGO_DIRS = ("src/algos/", "src/int/algos/")
MACRO_DIRS = ("src/macros/",)
WATCHED = ALGO_DIRS + MACRO_DIRS + ("src/policy/", "src/int/policy/")

PRECISION_WORDS = re.compile(
    r"\b(ulp|precision|correctly[- ]rounded|mis-?round|round(ing)? defect"
    r"|wrong (value|digit)|accuracy|diverge)", re.I
)


def sh(*args: str) -> str:
    # Decode as UTF-8 explicitly: this repo's sources contain non-ASCII (maths
    # symbols in doc comments), and the platform default (cp1252 on Windows)
    # raises UnicodeDecodeError on them. `errors="replace"` keeps a stray byte
    # from aborting the scan -- a checker that dies on one character is worse
    # than one that mangles it, because a crash exits non-zero and reads as a
    # finding.
    return subprocess.run(
        args, capture_output=True, check=True,
        encoding="utf-8", errors="replace",
    ).stdout


def hunks(rng: str):
    """Yield (path, header, removed_lines, added_lines) per hunk in the range."""
    diff = sh("git", "diff", "-U0", rng)
    path = None
    header = None
    rem: list[str] = []
    add: list[str] = []
    for line in diff.splitlines():
        if line.startswith("+++ b/"):
            if path and (rem or add):
                yield path, header, rem, add
                rem, add = [], []
            path = line[6:]
            header = None
        elif line.startswith("@@"):
            if path and (rem or add):
                yield path, header, rem, add
                rem, add = [], []
            header = line
        elif line.startswith("-") and not line.startswith("---"):
            rem.append(line[1:])
        elif line.startswith("+") and not line.startswith("+++"):
            add.append(line[1:])
    if path and (rem or add):
        yield path, header, rem, add


def code(lines: list[str]) -> list[str]:
    """Drop comments and blanks -- a comment is not an algorithm."""
    out = []
    for ln in lines:
        s = ln.strip()
        if not s or s.startswith("//") or s.startswith("*") or s.startswith("/*"):
            continue
        out.append(ln)
    return out


def main() -> int:
    rng = sys.argv[1] if len(sys.argv) > 1 else "HEAD~1..HEAD"
    findings: list[str] = []

    # ---- check 1: computation replaced by a delegation ----------------------
    for path, header, rem, add in hunks(rng):
        if not path.startswith(WATCHED):
            continue
        r, a = code(rem), code(add)
        if not r:
            continue
        removed_compute = [ln for ln in r if COMPUTE.search(ln)]
        # The signature: several computing lines out, a delegation in, and the
        # replacement is much smaller than what it replaced.
        if len(removed_compute) >= 3 and a and any(DELEGATE.search(ln) for ln in a) and len(a) <= max(3, len(r) // 4):
            findings.append(
                "FAIL  DELETED COMPUTATION\n"
                f"      {path}  {header or ''}\n"
                f"      {len(removed_compute)} computing lines removed, replaced by "
                f"{len(a)} line(s) of delegation.\n"
                "      evidence (first 3 removed):\n"
                + "".join(f"        - {ln.strip()[:110]}\n" for ln in removed_compute[:3])
                + "      evidence (added):\n"
                + "".join(f"        + {ln.strip()[:110]}\n" for ln in a[:3])
                + "      An algorithm may have been deleted. De-route, never delete:\n"
                "      lift it to a kernel file and register an Algorithm variant so the\n"
                "      matcher can select it per cell. If this IS the right change, say so\n"
                "      explicitly in the commit message -- do not let it pass silently."
            )

    # ---- check 2: net computation loss with no new kernel to receive it -----
    names = sh("git", "diff", "--name-status", rng).splitlines()
    added_kernels = [n.split("\t")[-1] for n in names
                     if n.startswith("A") and n.split("\t")[-1].startswith(ALGO_DIRS)]
    lost = 0
    for path, _h, rem, add in hunks(rng):
        if path.startswith(ALGO_DIRS + MACRO_DIRS):
            lost += len([l for l in code(rem) if COMPUTE.search(l)])
            lost -= len([l for l in code(add) if COMPUTE.search(l)])
    if lost >= 10 and not added_kernels:
        findings.append(
            "FAIL  NET COMPUTATION LOSS\n"
            f"      {lost} more computing lines removed than added across algos/macros,\n"
            "      and no new kernel file under src/algos or src/int/algos received them.\n"
            "      Where did the algorithm go?"
        )

    # ---- check 3: a precision fix with nothing pinned in golden -------------
    msg = sh("git", "log", "--format=%B", rng)
    if PRECISION_WORDS.search(msg):
        touched_lead = [n for n in names if "decimal-scaled-golden/lead/" in n]
        if not touched_lead:
            hit = PRECISION_WORDS.search(msg)
            findings.append(
                "FAIL  UNPINNED PRECISION FIX\n"
                f"      commit message mentions {hit.group(0)!r} but no file under\n"
                "      decimal-scaled-golden/lead/ changed.\n"
                "      A precision defect is not fixed until the INPUT that exposed it is\n"
                "      in the golden suite. The hypot 8-13 ULP error survived for exactly\n"
                "      this reason. Add the operand, at that width and scale, with a\n"
                "      '// why this case matters' comment, and regenerate the .au answers\n"
                "      from the EXTERNAL oracle -- never from this crate's own output."
            )

    # REPORTER, NOT GATE. Owner's call, 2026-09-03, and it is the right one.
    # A blocking gate has two fates and both end with it unread: tuned until it
    # never fires, so it becomes decoration; or firing often enough that clearing
    # it becomes reflex. Worse, a gate invites the belief that passing it means
    # the change is sound -- and these are three pattern matches over a diff, so
    # it means nothing of the kind. Reporting into a log reviewed AFTER the
    # decision keeps the finding in front of a person, off the critical path, and
    # leaves a record of what was knowable at merge time rather than what someone
    # recalls later.
    #
    # --report <path> appends; exit is 0 unless --strict is passed, so this can
    # run everywhere without blocking anything.
    report = None
    if "--report" in sys.argv:
        report = sys.argv[sys.argv.index("--report") + 1]
    strict = "--strict" in sys.argv

    try:
        head = sh("git", "log", "-1", "--format=%h %s", rng.split("..")[-1] or "HEAD").strip()
    except Exception:
        head = rng

    if findings:
        body = (f"\n=== check_no_algorithm_deleted: {len(findings)} FINDING(S) in {rng} ===\n"
                f"    {head}\n\n" + "\n".join(findings) + "\n\n"
                "These are findings, not verdicts, and the script cannot tell a MOVE from a\n"
                "DELETION -- a genuine consolidation into a kernel fires check 1 too. What it\n"
                "guarantees is only that the question was ASKED. Record a disposition per\n"
                "finding: what matched, why it is or is not a defect, what you did. An\n"
                "undisposed finding is the same silent omission this script exists to catch.\n")
        print(body)
        if report:
            with open(report, "a", encoding="utf-8") as fh:
                fh.write(body)
        return 1 if strict else 0

    clean = (f"check_no_algorithm_deleted: no findings over {rng}  ({head})\n"
             "    NOT a certificate of correctness. These are three pattern matches over a\n"
             "    diff; they cannot see a defect nobody thought to look for, and absence of\n"
             "    failure is not evidence. On 2026-09-03 two algorithms were deleted while\n"
             "    every check then in existence passed.\n")
    print(clean)
    if report:
        with open(report, "a", encoding="utf-8") as fh:
            fh.write(clean)
    return 0


if __name__ == "__main__":
    sys.exit(main())
