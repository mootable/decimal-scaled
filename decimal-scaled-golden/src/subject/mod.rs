//! The library-facing surface: the `DecimalSubject` trait a decimal library
//! implements to be tested, plus the value types the runner reasons about
//! (`Capabilities`, `Limits`, `Computed`). No subject impls live in this crate.

mod capabilities;
mod computed;
mod limits;
mod overflow;
mod subject;

pub use capabilities::{Capabilities, FnSupport, Radix};
pub use computed::{Computed, NonReal};
pub use limits::Limits;
pub use overflow::Overflow;
pub use subject::DecimalSubject;
