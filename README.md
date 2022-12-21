Macros and traits to ease using "type-enumerating" enums.

This crate defines traits that are useful when dealing with
containers that can contain one of a finite set types.

Further, it contains a set of macros that can automatically derive
implementations for these traits for enums of a specific structure,
namely structs where:
* Each variant has one single unnamed variant
* Each argument types appears at most once

For enums like this, this crate provides macros to easily convert between
the enums, and values of the types in the enum.

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
# extern crate macros;
# use macros as try_as;
# use std::convert::TryInto;
#[derive(try_as::From, try_as::TryInto, Debug)]
enum Value{
    Number(i64),
    String(String),
    Bool(bool)
}

// Convert to `Value` from `i64`, `String` or `bool` using `into`/`from`:
let x = Value::from(0);
let name = Value::from("Hello".to_owned());
let yes_or_no: Value = false.into();

// Convert back with `try_into`
let maybe_i64: Result<i64, _> = x.try_into();
assert_eq!(maybe_i64.unwrap(), 0);
let maybe_i64: Result<i64, Value> = name.try_into();
assert!(maybe_i64.is_err());

```
If we only need a reference to the value, we can use the macros `TryAsRef` and `TryAsMut`,
which derive implementations for the new traits [`traits::TryAsRef`] and [`traits::TryAsMut`], respectively:


```rust
# mod try_as {
#    pub extern crate traits;
#    extern crate macros;
#    pub use self::macros::*;
# }
use try_as::traits::{TryAsRef, TryAsMut};

#[derive(try_as::TryAsRef, try_as::TryAsMut)]
enum Value{
    Number(i64),
    String(String),
    Bool(bool)
}

let mut x = Value::Number(0);

let x_ref: &i64 = x.try_as_ref().unwrap();
assert_eq!(*x_ref, 0);

let x_mut: &mut i64 = x.try_as_mut().unwrap();
*x_mut = 4;

let x_ref: &i64 = x.try_as_ref().unwrap();
assert_eq!(*x_ref, 4);

let str_ref: Option<&String> = x.try_as_ref();
assert!(str_ref.is_none());
```

Finally, to inspect the type, we can use the trait `traits::TypedContainer`, which allows
us to look at the [`std::any::TypeId`] of the contained type:
```
# mod try_as {
#    pub extern crate traits;
#    extern crate macros;
#    pub use self::macros::*;
# }
use try_as::traits::TypedContainer;

#[derive(try_as::TypedContainer)]
enum Value{
    Number(i64),
    String(String),
    Bool(bool)
}

let x = Value::Number(0);
let boolean: Value = Value::Bool(false);
assert!(x.holds::<i64>());
assert!(!boolean.holds::<i64>());
assert!(std::any::TypeId::of::<bool>() == boolean.type_id());

```

## Todos and notes

In the doctests, the traits were very verbose to use, e.g. in the example
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

This problem might not come up that often in practice, to be determined.s

If it does, maybe we could define traits that allow specifying the
return types in generic parameters of methods, and implement them for all
types that implement the traits we already have, like this:

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

// Now, if `x` is implemented for TryInto<u64>, we can write
let maybe_x = x.try_as::<i64>();
// Or to unwrap
let val = x.try_as::<i64>().unwrap();
```