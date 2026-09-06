//! `bbc_pinned_probe` — the PINNED probe series for `bench-branch-compare`.
//!
//! # ⚠ THE CONTRACT — READ BEFORE CHANGING ANYTHING IN THIS FILE
//!
//! **This probe times one deliberately fixed operation, and its value is the
//! COMPARABILITY OF ITS HISTORY. Changing what it measures — the operation, the
//! width, the scale, the operands, or the expression's shape — INVALIDATES THE
//! ENTIRE HISTORICAL SERIES. Readings taken before the change cannot be
//! compared with readings taken after it.**
//!
//! That is not a reason never to change it. It is a reason that changing it
//! must be a deliberate, announced act rather than a tidy-up: say so in the
//! commit, and treat every earlier reading as belonging to a different series.
//! The `subject` string below travels inside every reading precisely so such a
//! change is visible IN THE DATA rather than inferred from a commit log.
//!
//! # What it is for
//!
//! The synthetic series (`probe/bbc_probe.rs`) answers what the RUNNER can
//! resolve. This one answers whether OUR OWN BASELINE moved.
//!
//! Nothing else in the sweep can see that. Every bbc cell is a ratio against
//! the previous release, so a slow, uniform creep — each release a little
//! slower than the last — reads as no change at all, every time, forever. A
//! fixed operation measured against a synthetic reference is what makes that
//! creep visible.
//!
//! The two series are reported SEPARATELY and must never be blended: when a
//! merged figure moves you cannot tell a slower runner from slower code, which
//! is the exact ambiguity the synthetic series exists to remove. Read together
//! they do give the useful derived quantity — pinned ÷ synthetic normalises the
//! machine out, so what remains is our code — and `probe_summary.py` computes
//! that ratio explicitly rather than by merging the raw readings.
//!
//! # Why THIS operation
//!
//! `decimal_scaled::D18<0>` addition, on the branch under test.
//!
//! * **Its cost has no reason to change.** `policy::add::select` returns
//!   `ByAlgorithm` at every arm — the work is a fixed-width limb ripple set by
//!   the width, not by the operands — so there is no value routing and no
//!   algorithm choice that could legitimately move it.
//! * **`D18` is one `Int<1>` limb at `SCALE 0`**: the narrowest, simplest,
//!   cheapest operation the library has. There is very little in it that could
//!   get faster, which is what makes it a good detector of getting slower.
//! * **It sits in the cost region the regression filter cannot currently
//!   assess** — a couple of nanoseconds, where the fixed 10 ns floor is a 1000%
//!   change. So it doubles as a direct check of whether that region is
//!   measurable at all.
//! * **The expression's shape is deliberately identical to the sweep's own
//!   `add` row** in `benches/compare_common.rs` (`black_box(x) + black_box(b)`),
//!   so this reading and that cell measure the same thing and can be read
//!   against each other.
//!
//! The BRANCH side is the subject, not `prod`. The question is whether OUR
//! baseline crept, and `prod` is a different published version every release —
//! pinning it would break the series at every release by construction, and it
//! would answer a question the synthetic already answers better.
//!
//! # Why it is a separate binary from the synthetic probe
//!
//! It has to link the crate. The synthetic probe must not be ABLE to, and that
//! is guaranteed by compiling it with `rustc` directly, outside any manifest.
//! The two therefore cannot share an executable — but they do share
//! `probe/probe_core.rs`, so both are measured identically and a difference in
//! method cannot masquerade as a difference in result.
//!
//! # Usage
//!
//! ```text
//! bbc_pinned_probe <start|end> <job-name> <out-dir>
//! ```

// The measurement methodology, shared with the synthetic probe. Reached by
// path because that file is not part of this package's module tree: it is also
// compiled standalone by `rustc` for the synthetic probe, which is what keeps
// the crate out of that binary.
#[path = "../probe/probe_core.rs"]
mod probe_core;

use std::env;
use std::hint::black_box;

use decimal_scaled::D18;

/// What is being timed, carried inside every reading. **This string and the
/// code below must always agree** — it is how a reader of the data knows which
/// series a reading belongs to, and it is the whole protection against a
/// silently changed subject.
const SUBJECT: &str = "decimal_scaled::D18<0> add";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: bbc_pinned_probe <start|end> <job-name> <out-dir>");
        // Exit 0 even here: the probe is never allowed to red a bench cell.
        return;
    }
    let (position, job, out_dir) = (&args[1], &args[2], &args[3]);
    let build = env::var("BBC_PROBE_BUILD").unwrap_or_default();

    // Parsed via `FromStr`, exactly as the sweep's own operands are, so the
    // values are constructed through the public surface and no internal
    // representation is assumed here. Both operands and the result stay single
    // digit, so nothing overflows `D18<0>`'s 18 digits.
    let a: D18<0> = "2".parse().expect("D18<0> parses \"2\"");
    let b: D18<0> = "3".parse().expect("D18<0> parses \"3\"");

    // The clock is characterised in BOTH series rather than only the
    // synthetic. It costs a fraction of a millisecond, and having it on both
    // sides means a pinned reading can be sanity-checked on its own without
    // being joined to its synthetic partner first.
    let clock = probe_core::probe_clock();

    // `black_box` on both operands AND the result: the same shape as the
    // sweep's `add` row, so the two measure the same thing.
    let entry = probe_core::measure("add", || {
        black_box(black_box(a) + black_box(b));
    });

    probe_core::emit(
        "pinned",
        SUBJECT,
        position,
        job,
        out_dir,
        &build,
        &clock,
        &[entry],
    );
}
