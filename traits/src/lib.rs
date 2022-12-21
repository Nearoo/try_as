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
