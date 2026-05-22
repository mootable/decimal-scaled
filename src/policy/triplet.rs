//! The reusable per-method policy-triplet emitter.
//!
//! Each per-(width, scale) policy method routes to exactly one kernel.
//! Historically that routing lived as an `if matches!(SCALE, range)`
//! cascade inside the per-width trait impl. [`policy_triplet!`] collapses
//! a width's routing into three free functions keyed on a const
//! `match (W, SCALE)`:
//!
//! - `<stem>_base` — the real algorithm: one arm per live `(W, SCALE)`
//!   cell, transcribed in first-match order.
//! - `<stem>_no_std` — a thin `#[inline]` pointer to `<stem>_base`.
//! - `<stem>_std` — a `match (W, SCALE)` carrying only the `std`-override
//!   cells, falling through to `<stem>_base` for everything else.
//!
//! The per-width trait method then picks `<stem>_std` vs `<stem>_no_std`
//! by a `#[cfg(feature = "std")]` select. Both the width tag `W` and
//! `SCALE` are compile-time const at every monomorphisation, so the
//! `match (W, SCALE)` const-folds to the single live arm — identical
//! codegen to the previous `if matches!` cascade, one direct kernel call,
//! zero dispatch.
//!
//! The width tag is `u16` (widths reach 1232; a `u8` would truncate and
//! collide). It must always be supplied as a const-generic argument
//! (`<stem>_std::<{ wtag::Dxx }, SCALE>`) — never a runtime field — or the
//! `match` stops folding and degrades into a jump table.

/// Width tags for the `(W, SCALE)` match discriminant. Each tag equals
/// the width literally; `u16` gives headroom past 1232.
pub(crate) mod wtag {
    pub(crate) const D18: u16 = 18;
    pub(crate) const D38: u16 = 38;
    pub(crate) const D57: u16 = 57;
    pub(crate) const D76: u16 = 76;
    pub(crate) const D115: u16 = 115;
    pub(crate) const D153: u16 = 153;
    pub(crate) const D230: u16 = 230;
    pub(crate) const D307: u16 = 307;
    pub(crate) const D462: u16 = 462;
    pub(crate) const D616: u16 = 616;
    pub(crate) const D924: u16 = 924;
    pub(crate) const D1232: u16 = 1232;
}

/// Emit the `base`/`no_std`/`std` free-function triplet for one policy
/// method on one width's raw storage.
///
/// - `storage`  — the per-width raw integer type (e.g. `Int192`).
/// - the three fn idents (`base_fn`/`std_fn`/`no_std_fn`) are passed
///   explicitly so the macro needs no `concat_idents!`/`paste` dependency.
/// - `recv` / `mode` — the identifiers the arm expressions use for the raw
///   storage value and the rounding mode (passed so they share macro
///   hygiene with the caller-supplied arm bodies).
/// - `params`   — zero or more `name: Ty` carried past `recv` into the
///   kernels (e.g. `exp_raw: i128` for `powf`).
/// - `base`     — the real algorithm: `(W_pat, SCALE_pat) => expr` arms in
///   first-match order. The arm body sees `recv`, the `params`, `mode`, and
///   `SCALE` (the impl's const generic).
/// - `std`      — `std`-override cells only; may be empty. Falls through to
///   the base fn for any non-override cell.
macro_rules! policy_triplet {
    (
        storage   = $Storage:ty,
        base_fn   = $base_fn:ident,
        std_fn    = $std_fn:ident,
        no_std_fn = $no_std_fn:ident,
        recv      = $recv:ident,
        mode      = $mode:ident,
        params    = { $($pname:ident : $pty:ty),* $(,)? },
        base      = { $( ($w:pat, $s:pat) => $base_expr:expr ),* $(,)? },
        std       = { $( ($sw:pat, $ss:pat) => $std_expr:expr ),* $(,)? } $(,)?
    ) => {
        #[inline]
        fn $base_fn<const W: u16, const SCALE: u32>(
            $recv: $Storage,
            $($pname: $pty,)*
            $mode: $crate::support::rounding::RoundingMode,
        ) -> $Storage {
            match (W, SCALE) {
                $( ($w, $s) => $base_expr, )*
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }

        #[inline]
        fn $no_std_fn<const W: u16, const SCALE: u32>(
            $recv: $Storage,
            $($pname: $pty,)*
            $mode: $crate::support::rounding::RoundingMode,
        ) -> $Storage {
            $base_fn::<W, SCALE>($recv, $($pname,)* $mode)
        }

        #[inline]
        #[cfg_attr(not(feature = "std"), allow(dead_code))]
        fn $std_fn<const W: u16, const SCALE: u32>(
            $recv: $Storage,
            $($pname: $pty,)*
            $mode: $crate::support::rounding::RoundingMode,
        ) -> $Storage {
            #[allow(unreachable_patterns, clippy::match_single_binding)]
            match (W, SCALE) {
                $( ($sw, $ss) => $std_expr, )*
                _ => $base_fn::<W, SCALE>($recv, $($pname,)* $mode),
            }
        }
    };
}

pub(crate) use policy_triplet;
