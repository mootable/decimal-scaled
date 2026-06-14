//! Regression suite: pinned reproducers for fixed defects whose inputs the
//! width-agnostic golden leads cannot express (per-tier band edges, storage
//! extremes, cross-surface parity). Each module under `regressions/` carries
//! its source files' original feature gates.

mod ln_lookup_bands;
mod max_scale;
mod powf_integer;
