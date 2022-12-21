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

/// Derive [`From<T>`] implementations for a type enumerating enum.
#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_from_impls(&enum_data)
}

/// Derive [`TryInto<T>`] for a type enumerating enum.
#[proc_macro_derive(TryInto)]
pub fn derive_try_int(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_try_into_impl(&enum_data)
}

/// Derive trait [`TryAsRef`] for a type enumerating enum.
#[proc_macro_derive(TryAsRef)]
pub fn derive_try_as_ref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_try_as_ref(&enum_data)
}

/// Derive trait [`TryAsMut`] for a type enumerating enum.
#[proc_macro_derive(TryAsMut)]
pub fn derive_try_as_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_try_as_mut(&enum_data)
}

/// Derive [`traits::TypedContainer`] for a type enumerating enum.
#[proc_macro_derive(TypedContainer)]
pub fn derive_typed_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = parse_enum_definition(&input);
    gen_typed_value(&enum_data)
}

fn parse_enum_definition(input: &DeriveInput) -> EnumData {
    // Make sure we have no generics
    if input.generics.type_params().count() > 0 {
        panic!("Type parameters aren't supported.");
    }
    if input.generics.lifetimes().count() > 0 {
        panic!("Lifetime parameters aren't supported.");
    }
    if input.generics.const_params().count() > 0 {
        panic!("Constnat parameters aren't supported.");
    }

    // Make sure we're deriving from an enum
    let data = if let Data::Enum(data) = &input.data {
        data
    } else {
        panic!("Can only be derived from enums.");
    };

    // Use to make sure that each type appears at most once
    let mut all_variant_types = HashSet::new();
    let mut variants: Vec<(Ident, Type)> = Vec::new();
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
                    panic!("Each variant argument type must be unique.");
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
            #enum_ident::#ident(_) => std::any::TypeId::of::<#type_>()
        }
    });

    TokenStream::from(quote! {
        impl traits::TypedContainer for #enum_ident {
            fn type_id(&self) -> std::any::TypeId {
                match self {
                    #(#type_id_match_arms),*
                }
            }
        }
    })
}
