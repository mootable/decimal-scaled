//! Cross-width decimal comparison — now handled by a single generic
//! impl, not per-pair macro expansion.
//!
//! Previously this module emitted a `PartialEq` / `PartialOrd` pair per
//! adjacent `(narrower, wider)` decimal-width pair at the same `SCALE`,
//! widening the narrower operand into the wider storage before
//! deferring to the wider storage's `Ord` / `Eq`.
//!
//! That matrix is superseded by a single generic pair in
//! `src/types/unified.rs`:
//!
//! ```ignore
//! impl<const N: usize, const M: usize, const S: u32>
//!     PartialEq<D<Int<M>, S>> for D<Int<N>, S> { … }
//! impl<const N: usize, const M: usize, const S: u32>
//!     PartialOrd<D<Int<M>, S>> for D<Int<N>, S> { … }
//! ```
//!
//! which delegates to the int-layer cross-width comparator
//! (`Int::cmp_cross`) on the storages at the same `SCALE`. One
//! instantiation covers every width combination, including the
//! same-type (`N == M`) case, so no per-pair widening macro is needed.
//!
//! ```ignore
//! let a: D38<12> = D38::<12>::from_int(5);
//! let b: D18<12> = D18::<12>::from_int(5);
//! assert!(a == b);   // cross-width, same SCALE
//! assert!(b < D38::<12>::from_int(6));
//! ```
