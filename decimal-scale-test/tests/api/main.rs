//! Public-API surface tests over decimal-scaled, consolidated from the root
//! crate's integration tests (one test TARGET, many `api/<area>.rs` modules,
//! to keep link time sane). Inside each module every source file's tests ride
//! in their own `mod from_<source>` block, so later batches concatenate
//! cleanly into the same files.

mod bitwise;
mod conversions;
mod cross_scale;
mod equalities;
mod errors;
mod foundation;
mod int_methods;
mod num_traits;
mod parsing;
mod pow;
mod rescale;
mod traits;
