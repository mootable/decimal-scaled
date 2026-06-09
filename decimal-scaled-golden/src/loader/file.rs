//! `FileLoader` — reads + parses golden files. Parsing is its private detail:
//! `//` line comments, `/* … */` block comments, `#key=value` metadata.

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use crate::function::Function;
use crate::subject::Limits;

use super::loader::{CaseLoader, GoldenCase};

/// Golden-set precision / guard assumed when a file carries no `#` metadata header.
const DEFAULT_GEN_PRECISION: u32 = 1233;
const DEFAULT_GUARD: u32 = 2;

/// Reads `<dir>/<function>.<ext>`, parsing on each call (a re-readable view).
/// Missing files yield no cases. `gen_precision`/`guard` are read once, from the
/// golden-file `#` header, at construction.
pub struct FileLoader {
    pub dir: PathBuf,
    pub extension: &'static str,
    oracle: Limits,
}

impl FileLoader {
    pub fn new(dir: impl Into<PathBuf>) -> FileLoader {
        let dir = dir.into();
        let (gen_precision, guard) =
            read_golden_header(&dir, "golden").unwrap_or((DEFAULT_GEN_PRECISION, DEFAULT_GUARD));
        FileLoader {
            dir,
            extension: "golden",
            oracle: Limits {
                min_value: None,
                max_value: None,
                max_precision: gen_precision.saturating_sub(guard),
            },
        }
    }
}

impl CaseLoader for FileLoader {
    fn load(&self, func: Function) -> Cow<'_, [GoldenCase]> {
        let path = self.dir.join(format!("{}.{}", func.name(), self.extension));
        match std::fs::read_to_string(&path) {
            Ok(body) => Cow::Owned(parse(func, &body)),
            Err(_) => Cow::Owned(Vec::new()),
        }
    }
    fn oracle_limits(&self) -> Limits {
        self.oracle.clone()
    }
}

/// Scan `dir` for the first `.<ext>` file carrying a `#gen_precision`/`#guard`
/// header and read it. `None` if none is found.
fn read_golden_header(dir: &Path, ext: &str) -> Option<(u32, u32)> {
    for entry in std::fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some(ext) {
            continue;
        }
        if let Ok(body) = std::fs::read_to_string(&path) {
            let gp = metadata_value(&body, "gen_precision");
            let g = metadata_value(&body, "guard");
            if gp.is_some() || g.is_some() {
                return Some((gp.unwrap_or(DEFAULT_GEN_PRECISION), g.unwrap_or(DEFAULT_GUARD)));
            }
        }
    }
    None
}

/// Read a `#key=value` metadata line's `u32` value from a file body.
fn metadata_value(body: &str, key: &str) -> Option<u32> {
    for line in body.lines() {
        if let Some(rest) = line.trim().strip_prefix('#') {
            if let Some((k, v)) = rest.split_once('=') {
                if k.trim() == key {
                    return v.trim().parse().ok();
                }
            }
        }
    }
    None
}

/// Parse a golden file body for `func`: one test per line, `[ \t]+`-separated.
/// Skips blank lines, `#` metadata, `//` line comments, and `/* … */` block
/// comments (which may span lines). A line whose field count != `arity + 1` is
/// skipped.
fn parse(func: Function, body: &str) -> Vec<GoldenCase> {
    let arity = func.arity();
    let body = strip_block_comments(body);
    let mut out = Vec::new();
    // `strip_block_comments` preserves one newline per original newline, so the
    // enumeration index matches the line's position in the original file.
    for (idx, raw) in body.lines().enumerate() {
        let text = raw.trim_end_matches('\r').trim();
        if text.is_empty() || text.starts_with('#') || text.starts_with("//") {
            continue;
        }
        let fields: Vec<&str> = text.split([' ', '\t']).filter(|f| !f.is_empty()).collect();
        if fields.len() != arity + 1 {
            continue;
        }
        out.push(GoldenCase {
            inputs: fields[..arity].iter().map(|s| s.to_string()).collect(),
            output_raw: fields[arity].to_string(),
            line: idx + 1,
        });
    }
    out
}

/// Strip `/* … */` block comments (which may span newlines), keeping newlines so
/// the remaining line-based parse still aligns. Char-safe (comments may be UTF-8).
fn strip_block_comments(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut chars = body.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume '*'
            while let Some(c2) = chars.next() {
                if c2 == '\n' {
                    out.push('\n');
                }
                if c2 == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unary_and_skips_noise() {
        let txt = "#gen_precision=1233\n#guard=2\n// a comment\n\n2.0   1.4142135\n9 3\n";
        let cases = parse(Function::Sqrt, txt);
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].inputs, vec!["2.0".to_string()]);
        assert_eq!(cases[0].output_raw, "1.4142135");
    }

    #[test]
    fn parses_binary_lines() {
        let cases = parse(Function::Hypot, "3 4 5\n");
        assert_eq!(cases[0].inputs, vec!["3".to_string(), "4".to_string()]);
        assert_eq!(cases[0].output_raw, "5");
    }

    #[test]
    fn strips_block_comments_spanning_lines() {
        let txt = "/* header\n   spanning */\n2 1.4142135\n";
        let cases = parse(Function::Sqrt, txt);
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].output_raw, "1.4142135");
    }

    #[test]
    fn reads_metadata_values() {
        let body = "#gen_precision=1233\n#guard=2\n2 1.4\n";
        assert_eq!(metadata_value(body, "gen_precision"), Some(1233));
        assert_eq!(metadata_value(body, "guard"), Some(2));
        assert_eq!(metadata_value(body, "missing"), None);
    }
}
