# try_as crate

This crate defines traits that are useful when dealing with
containers that can contain one of a finite set types.

Further, it contains a set of macros that can automatically derive
implementations for these traits for enums of a specific structure.


The macros can derive traits `From`, `TryAsMut`, `TryAsRef`, `TryInto` and `TypedContainer`
for enums of the following shape:
* All variants must have exactly one unnamed field (e.g. `Foo(u32)`)
* The type in a given variant must be unique to the enum

 ### Ok:

 ```rust
 #[derive(TryInto)]
 enum Foo {
     Bar(u32),
     Baz(MyStruct)
     Baf(Box<dyn MyTrait>)
     Boo((u32, u32, f32))
 }
 ```

 ### Not Ok:
 ```rust
 #[derive(TryInto)]
 enum Foo<T> {
     Bar(T),
     Baz { x: u32 },
     Baf,
 }

```


 # Example
 When parsing data stemming from a more dynamically typed
 language, it might happen that certain values can be one of n types.

 In this situation, it can be very useful to pass these values around
 as "one of" these types, yet still be able to modify the values
 as though they were specific types.

 With this crate, this can conveniently be accomplished using macros.

 In the following, we define a following type, applying all macros defined
 in this trait, and then show what each macro adds to that type:

 ```rust
 use try_as::macros;

 #[derive(macros::From,
         macros::TryInto,
         macros::TryAsRef,
         macros::TryAsMut,
         macros::TypedContainer)]
 enum Value {
     Int(i64),
     Bool(bool),
     String(String)
 }
 ```

 ## `From`

 The macro `From` implements [`From`] (and as a result also [`Into`]) for all types enumerated by the
 enum. This allows to convert from values with specific types to `Value` trivially:

 ```rust
 let v = Value::from(-12);
 let v2 =  false.into();
 ```

 ## `TryInto`

 Converting _away_ from `Value` to a concrete type might fail, since the `Value` might not be holding the required type.
 The macro `TryInto` this implements the trait [`TryInto`] to convert `Value` back to a concrete type:
 ```rust
 let v = Value::Int(0);
 let int: Result<i64, Value> = v.clone().try_into();
 let bool: Result<bool, Value> = v.try_into();

 assert_eq!(int.unwrap(), 0);
 assert!(bool.is_err());
 ```

 If the conversion fails, we get the original value back.

 If we _know_ that the contained value is of a certain type `T`,
 we can use `UnwrapInto<T>`, which is implemented for all types `TryInto<T>`,
 and panics if the value isn't as expected:

 ```rust
 let v = Value::Int(8);
 let int: i64 = v.clone().unwrap_into();
 // This will panic:
 let bool: bool = v.unwrap_into();
 ```

 ## `TryAsRef` and `TryAsMut`
 `try_into` and `unwrap_into` both take ownership of the value. To only get a reference, we can use [`TryAsRef`] and [`TryAsMut`]:
 ```rust
 let mut v = Value::Int(8);

 let int: &mut i64 = v.try_as_mut().unwrap();
 *int += 1;


 let int: &i64 = v.try_as_ref().unwrap();
 println!("v is {}", int);

 let foo: Option<&bool> = v.try_as_ref();
 assert!(foo.is_none());
 ```

 Same as before, if we are _certain_ that a `Value` is of a specific type, we can use the blanket implementations
 of [`UnwrapAsRef`] and [`UnwrapAsMut`]:

 ```rust
 let mut v = Value::Int(8);
 *v.unwrap_as_mut() += 22;
 ```

 ## `TypedContainer`
 If we don't know type a `Value` contains, we could pattern match on the `Value` itself,
 check with `is_err()` using `TryAsRef`, or, we can use the [`TypedContainer`] implementation,
 to get access to the [`std::any::TypeId`] of the contained value:

 ```rust
 use std::any::TypeId;

 let v = Value::Int(8);

 assert!(TypeId::of::<i64>() == v.type_id());
 assert!(v.is<i64>());
 ```
 Check the [`std::any`] crate for more infos.