use std::{any::TypeId, fmt::Debug};

/// A version of `AsRef<T>` that can fail.
pub trait TryAsRef<T> {
    fn try_as_ref(&self) -> Option<&T>;
}

/// A version of `AsMut<T>` that can fail.
pub trait TryAsMut<T> {
    fn try_as_mut(&mut self) -> Option<&mut T>;
}

/// A version of `AsRef<T>` that can fail, panicking if it does.
///
/// It is implemented automatically by all values that implement `TryAsRef<T>`.
pub trait UnwrapAsRef<T> {
    fn unwrap_as_ref(&self) -> &T;
}

/// Implement for all values implementing `TryAsRef<T>`.
impl<T, P> UnwrapAsRef<T> for P
where
    P: TryAsRef<T>,
{
    fn unwrap_as_ref(&self) -> &T {
        self.try_as_ref().unwrap()
    }
}

/// A version of `AsMut<T>` that can fail, panicking if it does.
///
/// It is implemented automatically by all values that implement `TryAsMut<T>`.
pub trait UnwrapAsMut<T> {
    fn unwrap_as_mut(&mut self) -> &mut T;
}

impl<T, P> UnwrapAsMut<T> for P
where
    P: TryAsMut<T>,
{
    fn unwrap_as_mut(&mut self) -> &mut T {
        self.try_as_mut().unwrap()
    }
}

/// A version of  `Into<T>` that can fail, panicking if it does.
///
/// Essentially behaves like `TryInto<T>`, but panicks if conversion isn't possible.
///
/// Automatically implemented for all types that implement `TryInto<Err, T>` for which
/// `Err` implements `Debug`.
pub trait UnwrapInto<T> {
    /// Convert `Self` to `T`; might fail, and if it does, panicks.
    fn unwrap_into(self) -> T;
}

/// Implement `UnwrapInto<T>` for values implementing `TryInto<T>`.
impl<T, P> UnwrapInto<T> for P
where
    P: TryInto<T>,
    <P as TryInto<T>>::Error: Debug,
{
    fn unwrap_into(self) -> T {
        self.try_into().unwrap()
    }
}

/// A trait for types that can hold values of different types.
pub trait TypedContainer {
    /// Returns true excactly if the type of the contained vlaue is `T`.
    fn holds<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    /// Returns the [`std::any::TypeId`] of the contained value.
    fn type_id(&self) -> TypeId;
}
