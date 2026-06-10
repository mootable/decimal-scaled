//! Behavioural-contract tests over decimal-scaled, hosted in
//! `decimal-scale-test`. Thin directory-style root (`tests/contracts/main.rs`):
//! each sibling module hosts one contract family, and inside each file every
//! `mod from_<source>` block preserves one migrated source file together with
//! its original feature gates.

mod mode_delegation;
mod overflow;
mod routing;
