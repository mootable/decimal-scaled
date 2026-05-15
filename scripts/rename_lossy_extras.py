"""Catch the remaining 'lossy' prose tokens that the first pass
missed — short adjectival uses like 'lossy ln', '(lossy)', 'Lossy
(...) variants' — without touching the float-conversion method
names which keep their `_lossy` suffix.
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Skip rules — never rewrite `lossy` immediately preceded by these.
PRESERVE_PREFIX = (
    r"(?<!_f8_)(?<!_f16_)(?<!_f32_)(?<!_f64_)(?<!_f128_)"
    r"(?<!_i8_)(?<!_i16_)(?<!_i32_)(?<!_i64_)(?<!_i128_)"
    r"(?<!_u8_)(?<!_u16_)(?<!_u32_)(?<!_u64_)(?<!_u128_)"
    r"(?<!_int_)(?<!_isize_)(?<!_usize_)"
)

SUBSTS = [
    (rf"{PRESERVE_PREFIX}\blossy `ln`", "fast `ln`"),
    (rf"{PRESERVE_PREFIX}\blossy `exp`", "fast `exp`"),
    (rf"{PRESERVE_PREFIX}\blossy `sin`", "fast `sin`"),
    (rf"{PRESERVE_PREFIX}\blossy `cos`", "fast `cos`"),
    (rf"{PRESERVE_PREFIX}\blossy `sqrt`", "fast `sqrt`"),
    (rf"{PRESERVE_PREFIX}\blossy bridge\b", "fast bridge"),
    (rf"{PRESERVE_PREFIX}\bno lossy form\b", "no fast form"),
    (rf"{PRESERVE_PREFIX}\bthe lossy `f64` bridge\b", "the fast `f64` bridge"),
    (rf"{PRESERVE_PREFIX}\blossy `f64` bridge\b", "fast `f64` bridge"),
    (rf"{PRESERVE_PREFIX}\blossy is the\b", "fast is the"),
    (rf"{PRESERVE_PREFIX}\b\(lossy\)", "(fast)"),
    (rf"{PRESERVE_PREFIX}\bLossy \(`f64`-bridge\)\b", "Fast (`f64`-bridge)"),
    (rf"{PRESERVE_PREFIX}\bLossy \(\\`f64\\`-bridge\)\b", "Fast (`f64`-bridge)"),
    # The "lossy" column heading immediately after a width name.
    (rf"^### (D\d+(?: / D\d+)*(?: / D\d+)?) lossy$", r"### \1 fast"),
    # "Lossy transcendentals (...)" heading.
    (rf"^## (\d+\. )?Lossy transcendentals\b", r"## \1Fast transcendentals"),
]


def rename(text: str) -> str:
    for pat, repl in SUBSTS:
        text = re.sub(pat, repl, text, flags=re.MULTILINE)
    return text


GLOBS = [
    "src/**/*.rs",
    "macros/**/*.rs",
    "tests/**/*.rs",
    "benches/**/*.rs",
    "*.md",
    "docs/**/*.md",
    "docs/**/*.draft",
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
        new = rename(text)
        if new != text:
            path.write_text(new, encoding="utf-8")
            edited += 1
            print(f"  edited: {path.relative_to(ROOT)}")
    print(f"\n{edited} file(s) modified.")


if __name__ == "__main__":
    main()
