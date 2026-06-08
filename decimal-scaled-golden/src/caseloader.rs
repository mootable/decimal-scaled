//! Loads the golden cases for a function. The tester's `run` is given a list of
//! `Function`s and uses a `CaseLoader` to fetch each one's cases — so the tester
//! never receives `GoldenCase`s directly. For now this just reads the function's
//! golden file into memory and parses it.

use std::path::PathBuf;

use crate::function::Function;
use crate::parser::{self, GoldenCase};

/// Loads every golden case for a function.
pub trait CaseLoader {
    fn load(&self, function: Function) -> Vec<GoldenCase>;
}

/// Reads `<dir>/<function>.<extension>` into memory and parses it. Missing files
/// yield no cases.
pub struct FileCaseLoader {
    pub dir: PathBuf,
    pub extension: &'static str,
}

impl FileCaseLoader {
    pub fn new(dir: impl Into<PathBuf>) -> FileCaseLoader {
        FileCaseLoader { dir: dir.into(), extension: "txt" }
    }
}

impl CaseLoader for FileCaseLoader {
    fn load(&self, function: Function) -> Vec<GoldenCase> {
        let path = self.dir.join(format!("{}.{}", function.name(), self.extension));
        match std::fs::read_to_string(&path) {
            Ok(body) => parser::parse(function, &body),
            Err(_) => Vec::new(),
        }
    }
}
