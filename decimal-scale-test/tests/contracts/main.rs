//! Behaviour-contract tests over decimal-scaled (overflow policy, rounding
//! mode delegation, dispatch routing, storage boundaries), consolidated from
//! the root crate's integration tests. Inside each module every source file's
//! tests ride in their own `mod from_<source>` block, so later batches
//! concatenate cleanly into the same files.

mod boundaries;
mod mode_delegation;
mod overflow;
mod routing;
