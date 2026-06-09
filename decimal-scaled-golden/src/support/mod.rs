//! Shared leaf types used across the extension-point folders: the function
//! enumeration, rounding mode, the `Outcome` verdict, the `string_decimal`
//! comparison helper, and the run `collector` records. These carry no
//! extension seam of their own — they are the vocabulary the seams speak in.

pub mod collector;
pub mod function;
pub mod outcome;
pub mod rounding;
pub mod string_decimal;
