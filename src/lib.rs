//! Macros and traits to ease using enums whose sole purpose is to
//! enumerate a set of types.
//!
//! It exports a set of traits that help to this end:
//! * [`traits::TryAsRef`] - like `AsRef<T>`, but allowed to fail
//! * [`traits::TryAsMut`] - like `AsMut<T>`, but allowed to fail
//! * [`traits::TypedContainer`] - inspect types of a container
//!
//! And a set of macros that derive implementations from these and some
//! standard traits, namely:
//! * [`macros::From`] to convert from the types to the enum
//! * [`macros::TryInto`] to convert from the enum back into the types
//! * [`macros::TryAsMut`] to get references of the values of the enum
//! * [`macros::TryAsRef`] to get mutable references of the values of the enum
//! * [`macros::TypedContainer`] to inspect the type in the enum
//!
//! To derive the traits for an enum, the enum has to have the following shape:
//! * Each variant must have exactly one unnamed parameter
//! * Each variant argument type must appear at most once
//!
//! ## Example
//!
//! Assume we have an enum that enumerates values of `i64`, `String` and `bool`:
//! ```rust
//! enum Value{
//!     Number(i64),
//!     String(String),
//!     Bool(bool)
//! }
//! ```
//!
//! And we want to convert between this enum and values of types `i64`, `String` and `bool`.
//! This crate exposes the following macros to ease this conversion:
//!
//! ```rust
//! # extern crate macros;
//! # use macros as try_as;
//! # use std::convert::TryInto;
//! #[derive(try_as::From, try_as::TryInto, Debug)]
//! enum Value{
//!     Number(i64),
//!     String(String),
//!     Bool(bool)
//! }
//!
//! // Convert to `Value` from `i64`, `String` or `bool` using `into`/`from`:
//! let x = Value::from(0);
//! let name = Value::from("Hello".to_owned());
//! let yes_or_no: Value = false.into();
//!
//! // Convert back with `try_into`
//! let maybe_i64: Result<i64, _> = x.try_into();
//! assert_eq!(maybe_i64.unwrap(), 0);
//! let maybe_i64: Result<i64, Value> = name.try_into();
//! assert!(maybe_i64.is_err());
//!
//! ```
//! If we only need a reference to the value, we can use the macros `TryAsRef` and `TryAsMut`,
//! which derive implementations for the new traits [`traits::TryAsRef`] and [`traits::TryAsMut`], respectively:
//!
//!
//! ```rust
//! # mod try_as {
//! #    pub extern crate traits;
//! #    extern crate macros;
//! #    pub use self::macros::*;
//! # }
//! use try_as::traits::{TryAsRef, TryAsMut};
//!
//! #[derive(try_as::TryAsRef, try_as::TryAsMut)]
//! enum Value{
//!     Number(i64),
//!     String(String),
//!     Bool(bool)
//! }
//!
//! let mut x = Value::Number(0);
//!
//! let x_ref: &i64 = x.try_as_ref().unwrap();
//! assert_eq!(*x_ref, 0);
//!
//! let x_mut: &mut i64 = x.try_as_mut().unwrap();
//! *x_mut = 4;
//!
//! let x_ref: &i64 = x.try_as_ref().unwrap();
//! assert_eq!(*x_ref, 4);
//!
//! let str_ref: Option<&String> = x.try_as_ref();
//! assert!(str_ref.is_none());
//! ```
//!
//! Finally, to inspect the type, we can use the trait `traits::TypedContainer`, which allows
//! us to look at the [`std::any::TypeId`] of the contained type:
//! ```
//! # mod try_as {
//! #    pub extern crate traits;
//! #    extern crate macros;
//! #    pub use self::macros::*;
//! # }
//! use try_as::traits::TypedContainer;
//!
//! #[derive(try_as::TypedContainer)]
//! enum Value{
//!     Number(i64),
//!     String(String),
//!     Bool(bool)
//! }
//!
//! let x = Value::Number(0);
//! let boolean: Value = Value::Bool(false);
//! assert!(x.holds::<i64>());
//! assert!(!boolean.holds::<i64>());
//! assert!(std::any::TypeId::of::<bool>() == boolean.type_id());
//!
//! ```

extern crate macros;
pub use macros::*;

/// Contains the traits `TryAsRef`, `TryAsMut` and `TypedContainer`
pub mod traits {
    extern crate traits;
    pub use self::traits::*;
}
