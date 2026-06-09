//! Loading golden cases + the golden value type. `CaseLoader` yields a function's
//! `GoldenCase`s and declares the oracle's reach; `FileLoader` reads + parses files
//! (parsing is its private detail); `GoldenValue` is the parsed golden number.

mod file;
mod loader;
pub mod value;

pub use file::FileLoader;
pub use loader::{CaseLoader, GoldenCase};
pub use value::GoldenValue;
