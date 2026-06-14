//! Per-width branch-vs-prod compare bench for D38, fanned out over SCALE.
//! Run: cargo bench --bench compare_d38            (all scales for this width)
//!      cargo bench --bench compare_d38 -- s30     (just scale 30)
//!
//! Branch (`decimal_scaled::D38<S>`) vs prod (`prod::D38<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = {0, S/4, S/2, 3S/4, S-1} with S=38: [0, 9, 19, 28, 37].
//! {0, 19, 37, 30}. Each scale becomes a criterion group suffix `_s<scale>`,
//! selectable via `-- s<scale>`.

#[macro_use]
mod compare_common;

width_bench!("D38", D38, D38, [0, 9, 19, 28, 37]);
