//! Public-API surface tests over decimal-scaled, hosted in `decimal-scale-test`.
//! Thin directory-style root (`tests/api/main.rs`): each sibling module hosts
//! one suite, and inside each file every `mod from_<source>` block preserves
//! one migrated source file together with its original feature gates.

mod constants;
mod conversions;
mod dyn_facade;
mod equalities;
mod fast_surface;
mod parsing;
mod proc_macro;
mod serde;
mod tiers;
