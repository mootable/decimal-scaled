//! Per-width branch-vs-prod compare bench for D38, fanned out over SCALE.
//! Run: cargo bench --bench compare_d38            (all scales for this width)
//!      cargo bench --bench compare_d38 -- s30     (just scale 30)
//!
//! Branch (`decimal_scaled::D38<S>`) vs prod (`prod::D38<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = dedup{0, S/2, S-1, 30 if 30 <= S-1} with S=38 (D38's capacity):
//! {0, 19, 37, 30}. Each scale becomes a criterion group suffix `_s<scale>`,
//! selectable via `-- s<scale>`.

#[macro_use]
mod compare_common;

width_bench!("D38", D38, D38, [0, 19, 30, 37]);
