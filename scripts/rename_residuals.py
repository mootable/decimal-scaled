"""Residual rename pass — catches the corner cases the initial
type-rename regex missed:

- `D38_MAX_SCALE` style constants (underscore-uppercase suffix).
- `PI_D128_S37` style constant names emitted by `build.rs`.
- `D38_6` style test-local type aliases.
- `d38!` proc-macro invocations and the `d38` function/module
  identifier in the proc-macro crate.
- `BnumD76` baseline shim → `BnumD76` for consistency.
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Mapping table.
TYPE_MAP = [
    ("D4096", "D1232"),
    ("D3072", "D924"),
    ("D2048", "D616"),
    ("D1536", "D462"),
    ("D1024", "D307"),
    ("D768", "D230"),
    ("D512", "D153"),
    ("D384", "D115"),
    ("D256", "D76"),
    ("D128", "D38"),
    ("D64", "D18"),
    ("D32", "D9"),
]

# Two new pattern shapes that the first pass missed:
# - `D128_` followed by uppercase (e.g. `D38_MAX_SCALE`,
#   `D38_S38`, `PI_D128_S37`).
# - `D128_` followed by a digit (e.g. `D38_6` test alias).
# - `D38Invocation` style PascalCase composition.
def passes(text: str) -> str:
    for old, new in TYPE_MAP:
        # Constant / type alias: D<old>_<UPPERCASE>
        text = re.sub(
            rf"(?<![A-Za-z0-9_]){old}(?=_[A-Z])",
            new,
            text,
        )
        # Numbered test alias: D<old>_<digit>
        text = re.sub(
            rf"(?<![A-Za-z0-9_]){old}(?=_\d)",
            new,
            text,
        )
        # PascalCase composition: D<old><UPPERCASE letter or word>
        text = re.sub(
            rf"(?<![A-Za-z0-9_]){old}(?=[A-Z][a-z])",
            new,
            text,
        )
    return text


# Lowercase d38 → d38 (proc-macro). Match: word boundary, `!`, `(`,
# or end-of-line after the digits. Don't touch `d_w128_kernels`.
def lower_d38(text: str) -> str:
    return re.sub(
        r"(?<![A-Za-z0-9_])d38(?=[\b!(]|$|[^A-Za-z0-9_])",
        "d38",
        text,
    )


def bnum_rename(text: str) -> str:
    # BnumD76 baseline shim → BnumD76 for consistency with the
    # public type naming.
    return re.sub(
        r"(?<![A-Za-z0-9_])BnumD76\b",
        "BnumD76",
        text,
    )


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
    targets = []
    for g in GLOBS:
        targets.extend(sorted(ROOT.glob(g)))
    seen = set()
    files = [p for p in targets if not (p in seen or seen.add(p))]
    edited = 0
    for path in files:
        if not path.is_file():
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        new = bnum_rename(lower_d38(passes(text)))
        if new != text:
            path.write_text(new, encoding="utf-8")
            edited += 1
            print(f"  edited: {path.relative_to(ROOT)}")
    print(f"\n{edited} file(s) modified.")


if __name__ == "__main__":
    main()
