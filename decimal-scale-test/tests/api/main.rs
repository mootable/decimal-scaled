//! Public-API behaviour suite — directory-style target root. Each `mod`
//! is a sibling file in `tests/api/`, and inside each file every relocated
//! source file's tests sit in their own `mod from_<source>` block.

mod arithmetic;
mod constants;
mod display;
mod foundation;
mod num_traits;
mod pow;
mod trig;
