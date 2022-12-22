Macros and traits to ease using enums whose sole purpose is to
enumerate a set of types.

It exports a set of traits that help to this end:
* `traits::TryAsRef` - like `AsRef<T>`, but allowed to fail
* `traits::TryAsMut` - like `AsMut<T>`, but allowed to fail
* `traits::TypedContainer` - inspect types of a container

And a set of macros that derive implementations from these and some
standard traits, namely:
* `macros::From` to convert from the types to the enum
* `macros::TryInto` to convert from the enum back into the types
* `macros::TryAsMut` to get references of the values of the enum
* `macros::TryAsRef` to get mutable references of the values of the enum
* `macros::TypedContainer` to inspect the type in the enum

To derive the traits for an enum, the enum has to have the following shape:
* Each variant must have exactly one unnamed parameter
* Each variant argument type must appear at most once

## Documentation

The documentation can be read [here](https://nearoo.github.io/try_as/try_as/).

## Example

A short example where all macros are used:
We have an enum containing values of one of 3 types: `i64`, `String` and `bool`:
```rust
enum Value{
    Number(i64),
    String(String),
    Bool(bool)
}
```

And we want to convert between this enum and values of types `i64`, `String` and `bool`.
Using the macros of this crate, this can be achieved easily:
```rust
#[derive(try_as::From, try_as::TryInto, Debug, PartialEq, Eq)]
enum Value{
    Number(i64),
    String(String),
    Bool(bool)
}

let x = Value::from(0);
assert_eq!(x, Value::Number(0));

let maybe_i64: Result<i64, _> = x.try_into();
assert_eq!(maybe_i64.unwrap(), 0);

```

Read more in the [documentation](https://nearoo.github.io/try_as/).





## Todos and notes

The API isn't necessairly stable yet. Suggestions for improvements and PR's welcome!

---

Some traits in some situations are verbose to use, e.g:

``` rust
#[derive(try_as::From, try_as::TryInto, Debug)]
enum Value{
    Number(i64),
    String(String),
    Bool(bool)
}

let x = Value::from(0);
// This doesn't work; rust can't resolve the type on its own
assert_eq!(x.try_into().unwrap(), 0);
// So etiher we write this
assert_eq!((x as dyn TryInto<i64, _>).try_into().unwrap(), 0);
// Or this
let maybe_i64: Result<i64, _> = x.try_into();
assert_eq!(maybe_i64.unwrap(), 0);
```

This problem might not come up that often in practice, to be determined.

If it does, maybe we could define helper methods of those traits that allow specifying the
return types in generic parameters of methods,


```rust
pub trait TryAsMut<T> {
    fn try_as_mut(&mut self) -> Option<&mut T>;

    fn try_as_mut_of<U>(&mut self) -> Option<&mut U>
    where
        Self: TryAsMut<U>,
    {
        self.try_as_mut()
    }
}

/// Now we can get an `Option<&i64>` simply by writing
let maybe_val = x.try_as_mut_of::<i64>();
// Or to unwrap
let val = x.try_as_mut_of::<i64>().unwrap();
```

And for `TryInto`, we could provide blanket implementations, like:


```rust
pub trait TryAs {
    fn try_as<T>(self) -> Result<T, Self>
    where
        Self: TryInto<T, Error = Self>,
    {
        self.try_into()
    }
}

impl<U> TryAs for U {}

// Now, if `x` has an implementation of TryInto<u64>, we can write
let maybe_val = x.try_as::<i64>();
// Or to unwrap
let val = x.try_as::<i64>().unwrap();
```