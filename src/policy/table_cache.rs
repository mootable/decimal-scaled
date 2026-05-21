//! Per-thread working-scale table memoisation — the one place the
//! `std` vs `no_std` divergence for lookup-table kernels lives.
//!
//! Every Tang-style lookup kernel in [`crate::algos`] precomputes a
//! table of transcendental constants indexed by a quantised argument,
//! at a *working scale* `w` that depends on the caller's `SCALE`. The
//! table is expensive to build (one `*_fixed` kernel call per slot) but
//! identical for every call at the same `w`, so it wants memoising.
//!
//! Where that memo lives is the **only** difference between the `std`
//! and `no_std` builds of these kernels:
//!
//! - **`std`** — a per-thread `thread_local!` cache keyed on `w`, so the
//!   table is built once per `(thread, w)` and every later call borrows
//!   the entry. The hot-path cost collapses to a slice index.
//! - **`no_std`** — no thread-local storage, so the table is rebuilt on
//!   every call (`compute(w)[idx]`). Correct, just not amortised.
//!
//! [`decl_table_cache!`] emits a per-kernel cache module carrying that
//! divergence inside the macro body, so the algorithm kernel itself is
//! written once, free of any `#[cfg(feature = "std")]`. Each kernel
//! invokes the macro with its own entry type and table builder; the
//! generated `table_entry(w, idx)` is the cfg-free abstraction the
//! kernel calls.
//!
//! The `compute(w)` table builder stays in the kernel file — it is the
//! algorithm-specific part (which constant, which quantisation). Only
//! the memo wrapper is hoisted here.

/// Emit a per-thread, working-scale-keyed table memo for a lookup
/// kernel.
///
/// Generates, in the calling module:
///
/// - a `cache` submodule holding the `std` `thread_local!` slot and the
///   `no_std` recompute fallback, gated by a single `#[cfg]` inside this
///   macro;
/// - a `table_entry(w: u32, idx: usize) -> $Entry` free fn that the
///   kernel calls with no `#[cfg]` in sight.
///
/// Parameters:
///
/// - `entry = $Entry:ty` — the per-slot table value type (e.g. the
///   kernel's `core::W`, or a `(W, W)` sin/cos pair).
/// - `compute = $compute:path` — the kernel-local builder
///   `fn(w: u32) -> alloc::vec::Vec<$Entry>` that fills the whole table
///   for working scale `w`.
///
/// The slot is a `Vec<(u32, Vec<Entry>)>` linear-scanned on `w`; the
/// live `w` count is one to a few per build (one per distinct `SCALE`
/// choice routed to this kernel), so the scan is trivially short.
macro_rules! decl_table_cache {
    (entry = $Entry:ty, compute = $compute:path $(,)?) => {
        #[cfg(feature = "std")]
        mod table_cache_impl {
            use super::*;

            ::std::thread_local! {
                /// Per-thread cache of computed tables keyed on the
                /// working scale `w`. One entry per distinct `w`.
                static TABLE_CACHE:
                    ::core::cell::RefCell<alloc::vec::Vec<(u32, alloc::vec::Vec<$Entry>)>> =
                    const { ::core::cell::RefCell::new(alloc::vec::Vec::new()) };
            }

            /// Return the slot at `idx` of the table for working scale
            /// `w`, building (and caching) the table on first request.
            #[inline]
            pub(super) fn table_entry(w: u32, idx: usize) -> $Entry {
                TABLE_CACHE.with(|c| {
                    {
                        let cache = c.borrow();
                        for (cw, tbl) in cache.iter() {
                            if *cw == w {
                                return tbl[idx];
                            }
                        }
                    }
                    let tbl = $compute(w);
                    let entry = tbl[idx];
                    c.borrow_mut().push((w, tbl));
                    entry
                })
            }
        }

        #[cfg(not(feature = "std"))]
        mod table_cache_impl {
            use super::*;

            /// No thread-local storage on `no_std`: rebuild the table on
            /// every call and index it.
            #[inline]
            pub(super) fn table_entry(w: u32, idx: usize) -> $Entry {
                $compute(w)[idx]
            }
        }

        use table_cache_impl::table_entry;
    };
}

pub(crate) use decl_table_cache;
