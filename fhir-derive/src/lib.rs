//! Procedural macros for the `fhir-specifications-parser` crate.
//!
//! Provides `#[derive(Validate)]`, which generates a recursive
//! `crate::r5::validate::Validate` implementation that validates every field
//! (for structs) or the active variant's data (for enums), prefixing each
//! nested issue's `path` with the field name.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

/// Derive a recursive `Validate` implementation.
#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_g, ty_g, where_g) = input.generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(s) => {
            let stmts = struct_field_stmts(&s.fields);
            quote! {
                let mut issues = ::std::vec::Vec::new();
                #(#stmts)*
                issues
            }
        }
        Data::Enum(e) => {
            let arms = e.variants.iter().map(|v| {
                let vname = &v.ident;
                match &v.fields {
                    Fields::Unit => quote! { #name::#vname => {} },
                    Fields::Unnamed(f) => {
                        let binds: Vec<syn::Ident> = (0..f.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("__f{i}"), proc_macro2::Span::call_site()))
                            .collect();
                        let calls = binds.iter().map(|b| quote! {
                            issues.extend(crate::r5::validate::Validate::validate(#b));
                        });
                        quote! { #name::#vname( #(#binds),* ) => { #(#calls)* } }
                    }
                    Fields::Named(f) => {
                        let names: Vec<syn::Ident> =
                            f.named.iter().map(|x| x.ident.clone().unwrap()).collect();
                        let calls = names.iter().map(|b| quote! {
                            issues.extend(crate::r5::validate::Validate::validate(#b));
                        });
                        quote! { #name::#vname { #(#names),* } => { #(#calls)* } }
                    }
                }
            });
            quote! {
                let mut issues = ::std::vec::Vec::new();
                match self { #(#arms),* }
                issues
            }
        }
        Data::Union(_) => quote! { ::std::vec::Vec::new() },
    };

    quote! {
        impl #impl_g crate::r5::validate::Validate for #name #ty_g #where_g {
            fn validate(&self) -> ::std::vec::Vec<crate::r5::validate::ValidationIssue> {
                #body
            }
        }
    }
    .into()
}

fn struct_field_stmts(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(f) => f
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().unwrap();
                let fname = ident.to_string();
                quote! {
                    for mut __issue in crate::r5::validate::Validate::validate(&self.#ident) {
                        __issue.path = if __issue.path.is_empty() {
                            #fname.to_string()
                        } else {
                            format!("{}.{}", #fname, __issue.path)
                        };
                        issues.push(__issue);
                    }
                }
            })
            .collect(),
        Fields::Unnamed(f) => f
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let idx = Index::from(i);
                quote! {
                    issues.extend(crate::r5::validate::Validate::validate(&self.#idx));
                }
            })
            .collect(),
        Fields::Unit => vec![],
    }
}
