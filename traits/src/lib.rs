//! Contains the traits `TryAsRef`, `TryAsMut` and `TypedContainer`, used by the crate [try_as](https://crates.io/crates/try_as) to simplfy dealing with enums enumerating types.
//!
//! See the the [crate documentation](https://nearoo.github.io/try_as/try_as/) for more information
//! and documentation on how to use the traits.

use std::any::TypeId;

/// A version of `AsRef<T>` that can fail.
pub trait TryAsRef<T> {
    fn try_as_ref(&self) -> Option<&T>;
}

/// A version of `AsMut<T>` that can fail.
pub trait TryAsMut<T> {
    fn try_as_mut(&mut self) -> Option<&mut T>;
}

/// A trait for types that can hold values of different types.
pub trait TypedContainer {
    /// Returns `true` excactly if the type of the contained vlaue is `T`.
    fn holds<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    /// Returns the [`std::any::TypeId`] of the contained value.
    fn type_id(&self) -> TypeId;
}
