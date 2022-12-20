//! Macros deriving traits `From`, `TryAsMut`, `TryAsRef`, `TryInto` and `TypedContainer`
//! for enums of the following shape:
//! * All variants must have exactly one unnamed field (e.g. `Foo(u32)`)
//! * The type in a given variant must be unique to the enum
//!
//! ### Ok:
//!
//! ```
//! #[derive(TryInto)]
//! enum Foo {
//!     Bar(u32),
//!     Baz(MyStruct)
//!     Baf(Box<dyn MyTrait>)
//!     Boo((u32, u32, f32))
//! }
//! ```
//!
//! ### Not Ok:
//! ```
//! #[derive(TryInto)]
//! enum Foo<T> {
//!     Bar(T),
//!     Baz { x: u32 },
//!     Baf,
//! }
//!
//!```
//!
//!
//! # Example
//! When parsing data stemming from a more dynamically typed
//! language, it might happen that certain values can be one of n types.
//!
//! In this situation, it can be very useful to pass these values around
//! as "one of" these types, yet still be able to modify the values
//! as though they were specific types.
//!
//! With this crate, this can conveniently be accomplished using macros.
//!
//! In the following, we define a following type, applying all macros defined
//! in this trait, and then show what each macro adds to that type:
//!
//! ```
//! use try_as::macros;
//!
//! #[derive(macros::From,
//!         macros::TryInto,
//!         macros::TryAsRef,
//!         macros::TryAsMut,
//!         macros::TypedContainer)]
//! enum Value {
//!     Int(i64),
//!     Bool(bool),
//!     String(String)
//! }
//! ```
//!
//! ## `From`
//!
//! The macro `From` implements [`From`] (and as a result also [`Into`]) for all types enumerated by the
//! enum. This allows to convert from values with specific types to `Value` trivially:
//!
//! ```
//! let v = Value::from(-12);
//! let v2 =  false.into();
//! ```
//!
//! ## `TryInto`
//!
//! Converting _away_ from `Value` to a concrete type might fail, since the `Value` might not be holding the required type.
//! The macro `TryInto` this implements the trait [`TryInto`] to convert `Value` back to a concrete type:
//! ```
//! let v = Value::Int(0);
//! let int: Result<i64, Value> = v.clone().try_into();
//! let bool: Result<bool, Value> = v.try_into();
//!
//! assert_eq!(int.unwrap(), 0);
//! assert!(bool.is_err());
//! ```
//!
//! If the conversion fails, we get the original value back.
//!
//! If we _know_ that the contained value is of a certain type `T`,
//! we can use `UnwrapInto<T>`, which is implemented for all types `TryInto<T>`,
//! and panics if the value isn't as expected:
//!
//! ```
//! let v = Value::Int(8);
//! let int: i64 = v.clone().unwrap_into();
//! // This will panic:
//! let bool: bool = v.unwrap_into();
//! ```
//!
//! ## `TryAsRef` and `TryAsMut`
//! `try_into` and `unwrap_into` both take ownership of the value. To only get a reference, we can use [`TryAsRef`] and [`TryAsMut`]:
//! ```
//! let mut v = Value::Int(8);
//!
//! let int: &mut i64 = v.try_as_mut().unwrap();
//! *int += 1;
//!
//!
//! let int: &i64 = v.try_as_ref().unwrap();
//! println!("v is {}", int);
//!
//! let foo: Option<&bool> = v.try_as_ref();
//! assert!(foo.is_none());
//! ```
//!
//! Same as before, if we are _certain_ that a `Value` is of a specific type, we can use the blanket implementations
//! of [`UnwrapAsRef`] and [`UnwrapAsMut`]:
//!
//! ```
//! let mut v = Value::Int(8);
//! *v.unwrap_as_mut() += 22;
//! ```
//!
//! ## `TypedContainer`
//! If we don't know type a `Value` contains, we could pattern match on the `Value` itself,
//! check with `is_err()` using `TryAsRef`, or, we can use the [`TypedContainer`] implementation,
//! to get access to the [`std::any::TypeId`] of the contained value:
//!
//! ```
//! use std::any::TypeId;
//!
//! let v = Value::Int(8);
//!
//! assert!(TypeId::of::<i64>() == v.type_id());
//! assert!(v.is<i64>());
//! ```
//! Check the [`std::any`] crate for more infos.

extern crate proc_macro;
use core::panic;
use std::collections::HashSet;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, token::Enum, Data, DeriveInput, Fields, Ident, Type};

/// Contains all data of an enum we need:
/// It's identifier, and a vector of variants, each with
/// the variant's identifier and type.
type EnumData = (Ident, Vec<(Ident, Type)>);

/// Derive [`From<T>`] implementations for a typed value enum.
///
/// This macro allows the automatic implementation of `From<T>`
/// traits for a typed value enum `T`. The enum should follow the
/// specification described in the main page.
///
/// ## Example
///
/// ```
/// #[derive(From, PartialEq, Eq, Debug)]
/// enum Integer {
///     U8(u8),
///     String(String)
/// }
///  
/// let int = Integer::from(2);
/// assert_eq!(int, Integer::U8(2));
/// ```
///
/// Note that by implementing `From`, `Into` is automatically implemented
/// as well:
///
/// ```
/// #[derive(From, PartialEq, Eq, Debug)]
/// enum Integer {
///     U8(u8),
///     String(String)
/// }
///  
/// let int: Integer = 2.into();
/// assert_eq!(int, Integer::U8(2));
/// ```
///
#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_from_impls(&enum_data)
}

/// Derive [TryInto<T>]` for P` for all types `P` in a typed enum `T`.
///
/// Causes [`traits::UnwrapInto<T>`] to be implemented as well.
///
/// ## Example
///
/// ```
/// #[derive(TryInto)]
/// struct Integer {
///     U8(u8),
///     String(String)
/// }
///
/// let int = Integer::U8(22);
///
/// assert!(u8::try_from(int).is_ok());
/// assert!(String::try_from(int).is_err());
/// ```
#[proc_macro_derive(TryInto)]
pub fn derive_try_int(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_try_into_impl(&enum_data)
}

/// Derive trait [`TryAsRef`] for a type enum
#[proc_macro_derive(TryAsRef)]
pub fn derive_try_as_ref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_try_as_ref(&enum_data)
}

/// Derive trait [`TryAsMut`] for a type enum
#[proc_macro_derive(TryAsMut)]
pub fn derive_try_as_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_try_as_mut(&enum_data)
}

/// Derive [`traits::TypedContainer`] for a type enum
#[proc_macro_derive(TypedContainer)]
pub fn derive_typed_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_typed_value(&enum_data)
}

fn parse_enum_definition(input: &DeriveInput) -> EnumData {
    // Make sure we're deriving from an enum
    let data = if let Data::Enum(data) = &input.data {
        data
    } else {
        panic!("try_as::Into can only be derived from enums.");
    };

    let mut all_variant_types = HashSet::new();
    let mut variants: Vec<(Ident, Type)> = Vec::new();
    // Make sure each type appears exactly once
    for variant in data.variants.iter() {
        let field_type = match &variant.fields {
            Fields::Unit => panic!("Every variant must have at least one unnamed field."),
            Fields::Named(_) => panic!("Can't have variant with named fields."),
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() > 1 {
                    panic!("Each enum variant can have at most one type.");
                }

                let field_type = fields.unnamed.first().unwrap().ty.clone();
                if !all_variant_types.insert(field_type.clone()) {
                    panic!("Each enum variant type must be unique.");
                }
                field_type
            }
        };

        variants.push((variant.ident.clone(), field_type));
    }

    (input.ident.clone(), variants)
}

fn gen_from_impls(enum_data: &EnumData) -> TokenStream {
    let (enum_ident, variants) = enum_data;

    let impls = variants.iter().map(|(ident, type_)| {
        quote! {
            impl From<#type_> for #enum_ident {
                fn from(a: #type_) -> #enum_ident {
                    Self::#ident(a)
                }
            }
        }
    });

    TokenStream::from(quote! { #(#impls)* })
}

fn gen_try_into_impl(enum_data: &EnumData) -> TokenStream {
    let (enum_ident, variants) = enum_data;
    let impls = variants.iter().map(|(ident, type_)| {
        quote! {
            impl TryInto<#type_> for #enum_ident {
                type Error = Self;
                fn try_into(self) -> Result<#type_, Self::Error> {
                    if let Self::#ident(a) = self {
                        Ok(a)
                    } else {
                        Err(self)
                    }
                }
            }
        }
    });

    TokenStream::from(quote! { #(#impls)* })
}

fn gen_try_as_ref(enum_data: &EnumData) -> TokenStream {
    let (enum_ident, variants) = enum_data;

    let impls = variants.iter().map(|(ident, type_)| {
        quote! {
            impl traits::TryAsRef<#type_> for #enum_ident {
                fn try_as_ref(&self) -> Option<&#type_>{
                    if let Self::#ident(a) = self {
                        Some(a)
                    } else {
                        None
                    }
                }
            }
        }
    });

    TokenStream::from(quote! { #(#impls)* })
}

fn gen_try_as_mut(enum_data: &EnumData) -> TokenStream {
    let (enum_ident, variants) = enum_data;

    let impls = variants.iter().map(|(ident, type_)| {
        quote! {
            impl traits::TryAsMut<#type_> for #enum_ident {
                fn try_as_mut(&mut self) -> Option<&mut #type_>{
                    if let Self::#ident(a) = self {
                        Some(a)
                    } else {
                        None
                    }
                }
            }
        }
    });

    TokenStream::from(quote! { #(#impls)* })
}

fn gen_typed_value(enum_data: &EnumData) -> TokenStream {
    let (enum_ident, variants) = enum_data;

    let type_id_match_arms = variants.iter().map(|(ident, type_)| {
        quote! {
            #enum_ident::#ident(_) => TypeId::of::<#type_>(),
        }
    });

    TokenStream::from(quote! {
        impl traits::TypedValue {
            fn type_id(&self) -> std::any::TypeId {
                match self {
                    #(#type_id_match_arms),*
                }
            }
        }
    })
}
