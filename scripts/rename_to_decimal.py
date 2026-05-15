"""One-shot rename pass: public decimal types from binary-bit-width
names to decimal-digit-capacity names. Run from the crate root.

Mappings (verified via floor((bits-1) * log10(2))):

| binary | decimal |
|--------|---------|
| D9    | D9      |
| D18    | D18     |
| D38   | D38     |
| D76   | D76     |
| D115   | D115    |
| D153   | D153    |
| D230   | D230    |
| D307  | D307    |
| D462  | D462    |
| D616  | D616    |
| D924  | D924    |
| D1232  | D1232   |

Same mapping applies to the lowercase Cargo features (d76 → d76,
etc.). The internal binary kernel module `d_w128_kernels` is renamed
separately to `d_w128_kernels` so it stays unambiguously "the
binary 128-bit kernel" in source.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Largest first so substring overlap doesn't bite (D307 before D102… etc.).
TYPE_MAP = [
    ("D1232", "D1232"),
    ("D924", "D924"),
    ("D616", "D616"),
    ("D462", "D462"),
    ("D307", "D307"),
    ("D230", "D230"),
    ("D153", "D153"),
    ("D115", "D115"),
    ("D76", "D76"),
    ("D38", "D38"),
    ("D18", "D18"),
    ("D9", "D9"),
]

# Cargo feature names — only those that actually exist as features in
# Cargo.toml. D9/D18/D38 aren't feature-gated, so no lowercase
# entries for them.
FEATURE_MAP = [
    ("d1232", "d1232"),
    ("d924", "d924"),
    ("d616", "d616"),
    ("d462", "d462"),
    ("d307", "d307"),
    ("d230", "d230"),
    ("d153", "d153"),
    ("d115", "d115"),
    ("d76", "d76"),
]


def type_pattern(old: str) -> re.Pattern[str]:
    """`old` like `D38`. Matches when:
    - not preceded by a word character (so `Int128` / `i128` are safe);
    - followed by either a word boundary, a scale-alias suffix
      `s\\d+`, or an underscore-separated scale suffix `_s\\d+`
      (criterion bench labels look like `D38_s12`).
    """
    return re.compile(
        rf"(?<![A-Za-z0-9_]){re.escape(old)}(?=\b|s\d|_s\d)"
    )


def feature_pattern(old: str) -> re.Pattern[str]:
    """Lowercase feature name, plain word boundary."""
    return re.compile(rf"\b{re.escape(old)}\b")


def rename_in_text(text: str) -> str:
    for old, new in TYPE_MAP:
        text = type_pattern(old).sub(new, text)
    for old, new in FEATURE_MAP:
        text = feature_pattern(old).sub(new, text)
    # Internal binary kernel module — keep its bit-width visible.
    text = text.replace("d_w128_kernels", "d_w128_kernels")
    # Bench file referenced by name in Cargo.toml.
    text = text.replace("d_w128_mul_div_paths", "d_w128_mul_div_paths")
    return text


# Files to sweep. Anything outside these globs we leave alone (target/,
# .git/, etc.).
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
    # Deduplicate.
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
