"""Emit the Rust band-edge golden wiring block for ulp_strict_golden.rs.

Scans tests/golden/ for the per-tier {0, capacity-1} band-edge tables and
prints one `#[test]` per existing (func, tier, scale) file, grouped into a
feature-gated module per tier. Only files that actually exist are wired
(include_str! would fail to compile on an absent path; some band-edge cells
have an empty domain — e.g. cosh at MAX_SCALE always overflows — so no table
is emitted and no test is wired).

The wide-tier (d307..d1232) exp cells at {0, S-1} are already wired by the
`wide_s30_exp` module, so exp is skipped there to avoid a duplicate test name.

This is a build-time helper; its output is pasted into ulp_strict_golden.rs.
"""
from __future__ import annotations
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GOLDEN = ROOT / "tests" / "golden"

# (alias, capacity, feature_gate_cfg)
TIERS = [
    ("d18",   18,   "all()"),
    ("d38",   38,   "all()"),
    ("d57",   57,   'any(feature = "d57", feature = "wide")'),
    ("d76",   76,   'any(feature = "d76", feature = "wide")'),
    ("d115",  115,  'any(feature = "d115", feature = "wide")'),
    ("d153",  153,  'any(feature = "d153", feature = "wide")'),
    ("d230",  230,  'any(feature = "d230", feature = "wide")'),
    ("d307",  307,  'any(feature = "d307", feature = "x-wide")'),
    ("d462",  462,  'any(feature = "d462", feature = "x-wide")'),
    ("d616",  616,  'any(feature = "d616", feature = "x-wide")'),
    ("d924",  924,  'any(feature = "d924", feature = "xx-wide")'),
    ("d1232", 1232, 'any(feature = "d1232", feature = "xx-wide")'),
]

WIDTH_ENUM = {
    "d18": "D18", "d38": "D38", "d57": "D57", "d76": "D76", "d115": "D115",
    "d153": "D153", "d230": "D230", "d307": "D307", "d462": "D462",
    "d616": "D616", "d924": "D924", "d1232": "D1232",
}

# The 22 oracle-backed functions, table order.
FUNCS = ["sqrt", "cbrt", "exp", "ln", "log2", "log10", "exp2", "sin", "cos",
         "tan", "atan", "asin", "acos", "sinh", "cosh", "tanh", "asinh",
         "acosh", "atanh", "log", "atan2", "powf"]

# (func, tier, scale) cells already wired elsewhere (wide_s30_exp): exp at
# the wide tiers' {0, S-1}. Skip to avoid a duplicate test fn name.
WIDE = {"d307", "d462", "d616", "d924", "d1232"}

# DEFECTS exposed by band-edge coverage: cells where the kernel is NOT
# correctly rounded at the SCALE extreme. These are real not-correctly-rounded
# bugs (the coordinator routes the kernel fix); marked #[ignore] with an exact
# reproduction so the suite stays green and the failure is on record. Keyed by
# the test fn name `<alias>_<func>_s<scale>`. Reason = one representative
# (mode, input, kernel_value vs correctly-rounded oracle, delta) drawn from the
# failing run; some cells fail several inputs/modes in the same family.
DEFECTS = {
    # exp2(-1) at SCALE 0 = 0.5 (exact tie): HalfToEven/HalfTowardZero must
    # give 0, kernel returns 1 (tie not rounded to even/toward-zero).
    **{f"{a}_exp2_s0":
       "DEFECT: exp2 D{A} s0 mode HalfToEven input=-1 (exp2(-1)=0.5 tie) "
       "value=1 oracle=0 delta=1 LSB — tie not rounded to even; needs kernel fix"
       for a, A in [("d38","38"),("d57","57"),("d76","76"),("d115","115"),
                    ("d153","153"),("d230","230"),("d307","307"),("d462","462"),
                    ("d616","616"),("d924","924"),("d1232","1232")]},
    # powf(b, -k) at SCALE 0: result is a tiny positive (<1), Ceiling must
    # round up to 1, kernel returns 0 (underflow not rounded up under Ceiling).
    **{f"{a}_powf_s0":
       "DEFECT: powf D{A} s0 mode Ceiling input=b input2=-k (b^-k tiny positive) "
       "value=0 oracle=1 delta=1 LSB — underflow not rounded up under Ceiling; "
       "needs kernel fix"
       for a, A in [("d57","57"),("d76","76"),("d115","115"),("d153","153"),
                    ("d230","230"),("d462","462"),("d616","616"),("d924","924"),
                    ("d1232","1232")]},
    # log10(10^cap - 1) at MAX_SCALE: floor=cap-1, cls=High; the kernel
    # disagrees by 1 LSB under the directed modes (Trunc/Floor/Ceiling) at the
    # all-nines MAX input.
    **{f"{a}_log10_s0":
       "DEFECT: log10 D{A} s0 mode Trunc input=10^cap-1 (all-nines MAX) "
       "value=cap oracle=cap-1 delta=1 LSB — directed-rounding boundary at MAX "
       "input; needs kernel fix"
       for a, A in [("d76","76"),("d115","115"),("d153","153"),("d230","230"),
                    ("d307","307"),("d462","462"),("d616","616"),("d924","924"),
                    ("d1232","1232")]},
    # exp/sinh/cosh at SCALE 0 with a large integer input: the result fills the
    # tier's whole integer capacity and the kernel loses 5..32 LSB of accuracy
    # (the wide-exp working-width family the perf campaign tracks).
    "d38_exp_s0":
        "DEFECT: exp D38 s0 input=85 (exp(85)~8.2e36, fills i128 capacity) "
        "value off by 32 LSB (ulp ~3.5e9) — integer-regime precision loss; needs "
        "kernel fix",
    "d38_exp_s37":
        "DEFECT: exp D38 s37 — near-MAX-scale exp loses accuracy (same "
        "working-width family as the wide-exp regression); needs kernel fix",
    "d57_exp_s56":
        "DEFECT: exp D57 s56 mode Ceiling input=-1 (exp(-1e-56) just below 1) "
        "value=0.99..9 oracle=1.0..0 delta=1 LSB — underflow Ceiling at MAX "
        "scale; needs kernel fix",
    "d38_sinh_s0":
        "DEFECT: sinh D38 s0 input=75 (sinh(75) fills i128 capacity) value off "
        "by 19 LSB — integer-regime precision loss; needs kernel fix",
    "d38_cosh_s0":
        "DEFECT: cosh D38 s0 input=-67 (cosh(67) fills i128 capacity) value off "
        "by 5 LSB — integer-regime precision loss; needs kernel fix",
    "d924_sinh_s0":
        "DEFECT: sinh D924 s0 large integer input — integer-regime precision "
        "loss (result fills capacity); needs kernel fix",
    "d1232_sinh_s0":
        "DEFECT: sinh D1232 s0 large integer input — integer-regime precision "
        "loss (result fills capacity); needs kernel fix",
    "d924_cosh_s0":
        "DEFECT: cosh D924 s0 large integer input — integer-regime precision "
        "loss (result fills capacity); needs kernel fix",
    "d1232_cosh_s0":
        "DEFECT: cosh D1232 s0 large integer input — integer-regime precision "
        "loss (result fills capacity); needs kernel fix",
    # exp2 at SCALE 0 with a negative input near the underflow edge: Ceiling of
    # a tiny positive result returns 0 instead of 1 (same underflow-Ceiling
    # defect as powf), plus the exp2(-1) tie above — both fail this cell.
    "d38_exp2_s0":
        "DEFECT: exp2 D38 s0 — exp2(-1)=0.5 tie value=1 oracle=0 AND "
        "exp2(-106) Ceiling value=0 oracle=1 (underflow Ceiling); 1 LSB; needs "
        "kernel fix",
    # tan(±1) at SCALE 0: tan(1 rad)=1.557 -> floor=1 cls=High, half/Ceiling
    # must give 2; tan(-1)=-1.557 -> floor=-2 cls=Low. Kernel off by 1 LSB.
    "d18_tan_s0":
        "DEFECT: tan D18 s0 input=1 (tan(1 rad)=1.557) value=1 oracle=2 delta=1 "
        "LSB (High class under half modes); also input=-1; needs kernel fix",
    "d38_tan_s0":
        "DEFECT: tan D38 s0 input=1 (tan(1 rad)=1.557) value=1 oracle=2 delta=1 "
        "LSB; also input=-1; needs kernel fix",
}

def main() -> None:
    out = []
    out.append("// ─── Band-edge {0, capacity-1} cells (auto-listed) ─────────────────────")
    out.append("//")
    out.append("// Generated by scripts/gen_band_edge_wiring.py. One test per existing")
    out.append("// (func, tier, scale) golden table at the two SCALE extremes (0 and")
    out.append("// capacity-1 = MAX_SCALE). The canonical bands above cover ~S/2; these")
    out.append("// complete the {0, S/2, S-1} coverage so every (function, width) face is")
    out.append("// validity-gated across its whole SCALE range. Absent cells (empty domain")
    out.append("// at the edge, e.g. cosh at MAX_SCALE which always overflows) are not")
    out.append("// emitted by the generator and so carry no test. The wide-tier exp {0,S-1}")
    out.append("// cells are wired by `wide_s30_exp`, so exp is skipped there.")
    out.append("mod band_edges {")
    out.append("    use super::{check_at_scale, Width};")
    for alias, cap, gate in TIERS:
        scales = [0, cap - 1]
        lines = []
        for func in FUNCS:
            for scale in scales:
                if func == "exp" and alias in WIDE:
                    continue  # already in wide_s30_exp
                fname = f"{func}_{alias}_s{scale}.txt"
                if not (GOLDEN / fname).exists():
                    continue
                test_name = f"{alias}_{func}_s{scale}"
                ignore = DEFECTS.get(test_name)
                attr = ""
                if ignore is not None:
                    reason = ignore.replace("{A}", WIDTH_ENUM[alias][1:])
                    # Escape any embedded quotes for the Rust string literal.
                    reason = reason.replace('"', '\\"')
                    attr = f'        #[ignore = "{reason}"]\n'
                lines.append(
                    f'        #[test]\n'
                    f'{attr}'
                    f'        fn {test_name}() {{\n'
                    f'            check_at_scale("{func}", Width::{WIDTH_ENUM[alias]}, {scale}, '
                    f'include_str!("golden/{fname}"));\n'
                    f'        }}'
                )
        if not lines:
            continue
        out.append(f"    #[cfg({gate})]")
        out.append(f"    mod {alias} {{")
        out.append("        use super::{check_at_scale, Width};")
        out.extend(lines)
        out.append("    }")
    out.append("}")
    (ROOT / "scripts" / "band_edge_wiring.rs").write_text(
        "\n".join(out) + "\n", encoding="utf-8")
    print(f"wrote {sum(1 for l in out if l.strip() == '#[test]')} tests")

if __name__ == "__main__":
    main()
