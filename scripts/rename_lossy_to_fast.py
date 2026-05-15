"""Rename the f64-bridge transcendental dispatch path from "lossy"
to "fast". Float conversion methods (to_f64_lossy etc.) keep their
"lossy" suffix because their precision-loss semantics is a
separate concept from the strict/fast implementation dispatch.

Patterns rewritten:

- Module / file basenames containing `lossy`:
    log_exp_fast.rs       → log_exp_fast.rs
    trig_fast.rs          → trig_fast.rs
    powers_fast.rs        → powers_fast.rs
    macros/fast_transcendentals.rs → macros/fast_transcendentals.rs
- Module path tokens `fast_transcendentals` / `log_exp_fast` /
  `trig_fast` / `powers_fast` → `fast_*`.
- Macro `decl_fast_transcendentals_via_f64` → `decl_fast_transcendentals_via_f64`.
- Bench group label `fast/` → `fast/`.
- Prose tokens "fast transcendentals" / "fast variant" /
  "fast form" / "fast path" / "fast implementation" / "lossy
  block" → "fast" equivalent.

Patterns explicitly *preserved*:

- `to_f64_lossy`, `from_f64_lossy`, `to_f32_lossy`, …, including the
  experimental f16 / f128 conversions — these are precision-lossy
  conversions, not dispatch-strategy lossy.
- `to_i128_lossy`, `from_i128_lossy`, etc. — same reason.
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Substitutions, applied in order. Each is (regex, replacement). The
# negative-lookbehind on the prose rules avoids touching the
# `(to|from)_<type>_lossy` conversion-method family.

# Pattern guarding: never rename a `lossy` immediately preceded by
# `f64_`, `f32_`, `f16_`, `f128_`, `i32_`, `i64_`, `i128_`, `u32_`,
# `u64_`, `u128_`, `int_`, etc.
_PRESERVE_PREFIX = (
    r"(?<!_f8_)(?<!_f16_)(?<!_f32_)(?<!_f64_)(?<!_f128_)"
    r"(?<!_i8_)(?<!_i16_)(?<!_i32_)(?<!_i64_)(?<!_i128_)"
    r"(?<!_u8_)(?<!_u16_)(?<!_u32_)(?<!_u64_)(?<!_u128_)"
    r"(?<!_int_)(?<!_isize_)(?<!_usize_)"
)

SUBSTS: list[tuple[str, str]] = [
    # Module / token names with explicit `_lossy` suffix.
    (r"\blog_exp_lossy\b", "log_exp_fast"),
    (r"\btrig_lossy\b", "trig_fast"),
    (r"\bpowers_lossy\b", "powers_fast"),
    (r"\blossy_transcendentals\b", "fast_transcendentals"),
    (r"\bdecl_lossy_transcendentals_via_f64\b", "decl_fast_transcendentals_via_f64"),
    # Bench group label as it appears in criterion paths and labels.
    (r"\"fast/", '"fast/'),
    (r"`fast/", "`fast/"),
    # Bench macro identifier.
    (r"\blossy_block\b", "fast_block"),
    # Prose tokens — guarded by the preserve-prefix so float-conversion
    # names like `to_f64_lossy` aren't touched.
    (rf"{_PRESERVE_PREFIX}\blossy transcendentals\b", "fast transcendentals"),
    (rf"{_PRESERVE_PREFIX}\bLossy transcendentals\b", "Fast transcendentals"),
    (rf"{_PRESERVE_PREFIX}\blossy variant\b", "fast variant"),
    (rf"{_PRESERVE_PREFIX}\blossy form\b", "fast form"),
    (rf"{_PRESERVE_PREFIX}\blossy path\b", "fast path"),
    (rf"{_PRESERVE_PREFIX}\blossy implementation", "fast implementation"),
    (rf"{_PRESERVE_PREFIX}\blossy block\b", "fast block"),
    (rf"{_PRESERVE_PREFIX}\bLossy block\b", "Fast block"),
    (rf"{_PRESERVE_PREFIX}\blossy vs strict\b", "fast vs strict"),
    (rf"{_PRESERVE_PREFIX}\bstrict vs lossy\b", "strict vs fast"),
    (rf"{_PRESERVE_PREFIX}\bstrict / lossy\b", "strict / fast"),
    (rf"{_PRESERVE_PREFIX}\blossy / strict\b", "fast / strict"),
    (rf"{_PRESERVE_PREFIX}\blossy and strict\b", "fast and strict"),
    (rf"{_PRESERVE_PREFIX}\bstrict and lossy\b", "strict and fast"),
]


def rename_in_text(text: str) -> str:
    for pat, repl in SUBSTS:
        text = re.sub(pat, repl, text)
    return text


GLOBS = [
    "src/**/*.rs",
    "macros/**/*.rs",
    "tests/**/*.rs",
    "benches/**/*.rs",
    "scripts/**/*.py",
    "*.md",
    "docs/**/*.md",
    "docs/**/*.draft",
    "build.rs",
    "Cargo.toml",
    "macros/Cargo.toml",
]


def main() -> None:
    targets: list[Path] = []
    for g in GLOBS:
        targets.extend(sorted(ROOT.glob(g)))
    seen: set[Path] = set()
    files = [p for p in targets if not (p in seen or seen.add(p))]

    edited = 0
    for path in files:
        if not path.is_file():
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        new = rename_in_text(text)
        if new != text:
            path.write_text(new, encoding="utf-8")
            edited += 1
            print(f"  edited: {path.relative_to(ROOT)}")
    print(f"\n{edited} file(s) modified.")


if __name__ == "__main__":
    main()
