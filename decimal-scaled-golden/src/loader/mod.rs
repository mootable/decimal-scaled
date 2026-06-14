// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Loading golden cases + the golden value type. `CaseLoader` yields a function's
//! `GoldenCase`s and declares the oracle's reach; `FileLoader` reads + parses files
//! (parsing is its private detail); `GoldenValue` is the parsed golden number.

mod file;
mod filter;
mod loader;
#[cfg(feature = "net")]
mod url;
pub mod value;

pub use file::FileLoader;
pub use filter::FilterLoader;
pub use loader::{CaseLoader, GoldenCase};
#[cfg(feature = "net")]
pub use url::{UrlLoader, DEFAULT_REF};
pub use value::GoldenValue;
