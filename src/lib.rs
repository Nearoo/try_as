//! This crate defines traits that are useful when dealing with
//! containers that can contain one of a finite set types.
//!
//! Further, it contains a set of macros that can automatically derive
//! implementations for these traits for enums of a specific structure.
//! See [`macros`] for more info.

pub extern crate macros;

extern crate traits;
pub use traits::*;
