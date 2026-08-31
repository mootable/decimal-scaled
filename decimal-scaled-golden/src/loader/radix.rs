// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Radix-aware golden value selection — the VALUE-CHOOSER layer.
//!
//! A golden output field is a single whitespace-free token: either today's bare
//! `value` (the catch-all for every radix) or a comma-separated list of entries,
//! each `radix:value` (base tags `10` / `2` only — no precision, no fixed/float
//! kind) or a bare `value` catch-all. `select_radix_output` picks the entry a
//! subject of a given storage radix should be graded against; the caller then parses
//! that single value with `GoldenValue::parse`. The value table and the
//! subject↔value match stay separate concerns (spec §1).

use crate::subject::Radix;

/// Choose the golden value string for `radix` from a (possibly tagged) output field.
///
/// Rules (spec §1.2/§1.3):
/// - **No `:`** anywhere → the field is today's single value (the catch-all for every
///   radix) → returned VERBATIM, byte-for-byte. The current untagged corpus grades
///   identically, so landing this is a no-op on it.
/// - Otherwise the field is a comma-separated list: the entry whose `radix:` base tag
///   matches `radix` wins; absent that, the untagged bare `value` (the catch-all) is
///   used; absent BOTH (every entry tagged, none for this radix — a malformed/gap
///   line) it falls back deterministically to the first entry, which the caller will
///   fail to `GoldenValue::parse` (its `radix:` prefix is not a valid number) and so
///   surface rather than silently mis-grade.
///
/// Decimal values never contain `,` or `:`, so the separators are unambiguous, and a
/// leading `-` is safe because any `radix:` prefix (and its `:`) precedes it.
pub fn select_radix_output(output_raw: &str, radix: Radix) -> &str {
    // Backward-compatible fast path: a field with no `:` is exactly today's
    // single-value format — return it untouched.
    if !output_raw.contains(':') {
        return output_raw;
    }
    let mut catch_all: Option<&str> = None;
    for entry in output_raw.split(',') {
        match entry.split_once(':') {
            // `base:value` whose base is this subject's radix — the exact match wins.
            Some((tag, value)) if Radix::from_tag(tag) == Some(radix) => return value,
            // `base:value` for a different (or unrecognised) base — skip it.
            Some(_) => {}
            // a bare `value` (no `:`) is the untagged catch-all — keep the first one.
            None => {
                catch_all.get_or_insert(entry);
            }
        }
    }
    catch_all
        .or_else(|| output_raw.split(',').next())
        .unwrap_or(output_raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_tag_returns_verbatim_for_every_radix() {
        // Today's single-value format — the catch-all for every radix, byte-for-byte.
        assert_eq!(select_radix_output("1.2345", Radix::Decimal), "1.2345");
        assert_eq!(select_radix_output("1.2345", Radix::Binary), "1.2345");
        // A leading '-' is safe — no `:` so the whole string is returned verbatim.
        assert_eq!(select_radix_output("-0.5", Radix::Binary), "-0.5");
        // An exact integer (no fraction) likewise.
        assert_eq!(select_radix_output("42", Radix::Decimal), "42");
        // Empty / degenerate input is passed through (the caller's parse rejects it).
        assert_eq!(select_radix_output("", Radix::Decimal), "");
    }

    #[test]
    fn fully_tagged_picks_the_matching_radix() {
        let field = "10:1.5,2:1.6";
        assert_eq!(select_radix_output(field, Radix::Decimal), "1.5");
        assert_eq!(select_radix_output(field, Radix::Binary), "1.6");
        // Order-independent: the binary tag may come first.
        let field2 = "2:1.6,10:1.5";
        assert_eq!(select_radix_output(field2, Radix::Decimal), "1.5");
        assert_eq!(select_radix_output(field2, Radix::Binary), "1.6");
    }

    #[test]
    fn absent_tag_falls_back_to_untagged_catch_all() {
        // `catchall,2:v2`: radix 2 differs; every other radix uses the catch-all.
        // Negative values exercise the leading-'-' safety after a `2:` prefix.
        let field = "-0.123,2:-0.124";
        assert_eq!(select_radix_output(field, Radix::Decimal), "-0.123");
        assert_eq!(select_radix_output(field, Radix::Binary), "-0.124");
    }

    #[test]
    fn catch_all_alongside_tags_serves_unlisted_radixes() {
        // Only `2:` is tagged; the decimal subject (its radix unlisted) takes the bare
        // catch-all, the binary subject takes its explicit tag.
        let field = "3.14159,2:3.14160";
        assert_eq!(select_radix_output(field, Radix::Decimal), "3.14159");
        assert_eq!(select_radix_output(field, Radix::Binary), "3.14160");
    }

    #[test]
    fn malformed_gap_falls_back_to_first_entry() {
        // Every entry tagged, none for the requested radix and no catch-all: a gap.
        // Deterministically returns the first entry so the caller surfaces it (the
        // `radix:` prefix makes `GoldenValue::parse` fail rather than mis-grade).
        let field = "10:1.5";
        assert_eq!(select_radix_output(field, Radix::Binary), "10:1.5");
    }

    #[test]
    fn radix_tag_round_trips() {
        assert_eq!(Radix::Decimal.tag(), "10");
        assert_eq!(Radix::Binary.tag(), "2");
        assert_eq!(Radix::from_tag("10"), Some(Radix::Decimal));
        assert_eq!(Radix::from_tag("2"), Some(Radix::Binary));
        assert_eq!(Radix::from_tag("16"), None);
        assert_eq!(Radix::from_tag(""), None);
    }
}
