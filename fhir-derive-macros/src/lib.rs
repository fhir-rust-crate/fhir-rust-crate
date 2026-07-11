//! Procedural macros for the `fhir-specifications-parser` crate.
//!
//! Provides `#[derive(Validate)]`, which generates a recursive
//! `crate::r5::validate::Validate` implementation that validates every field
//! (for structs) or the active variant's data (for enums), prefixing each
//! nested issue's `path` with the field name.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Index, LitStr, PathArguments,
    Type,
};

/// Derive a recursive `Validate` implementation.
#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_g, ty_g, where_g) = input.generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(s) => {
            let stmts = struct_field_stmts(&s.fields, &name.to_string());
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

/// Snake-case identifier (after stripping a raw `r#`) to camelCase, matching the
/// FHIR element name.
fn to_camel(s: &str) -> String {
    let bare = s.strip_prefix("r#").unwrap_or(s);
    let mut out = String::new();
    let mut upper = false;
    for c in bare.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.extend(c.to_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// Whether a field type is a bare `Vec<…>` (not `Option<Vec<…>>`).
fn is_bare_vec(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.segments.last().is_some_and(|s| s.ident == "Vec"))
}

fn struct_field_stmts(fields: &Fields, struct_name: &str) -> Vec<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(f) => f
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().unwrap();
                let fname = ident.to_string();
                // Cardinality: a bare `Vec` that FHIR marks `1..*` must be
                // non-empty. Which fields are `1..*` is not encoded in the type
                // (bare `Vec` is also used for some `0..*`), so consult `meta` at
                // runtime keyed by the struct's FHIR path prefix.
                let cardinality = if is_bare_vec(&field.ty) {
                    let fhir_field = to_camel(&fname);
                    quote! {
                        if self.#ident.is_empty() {
                            if let ::core::option::Option::Some(__prefix) =
                                crate::r5::meta::struct_prefix(#struct_name)
                            {
                                let __path = format!("{}.{}", __prefix, #fhir_field);
                                if crate::r5::meta::element(&__path)
                                    .is_some_and(|__e| __e.min >= 1 && __e.is_multiple())
                                {
                                    issues.push(crate::r5::validate::ValidationIssue::new(
                                        #fname,
                                        "cardinality: a 1..* element must have at least one entry",
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    quote! {}
                };
                quote! {
                    for mut __issue in crate::r5::validate::Validate::validate(&self.#ident) {
                        __issue.path = if __issue.path.is_empty() {
                            #fname.to_string()
                        } else {
                            format!("{}.{}", #fname, __issue.path)
                        };
                        issues.push(__issue);
                    }
                    #cardinality
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

/// If `ty` is `Primitive<T>`, return `T` (the inner primitive type).
fn primitive_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(tp) = ty else { return None };
    let seg = tp.path.segments.last()?;
    if seg.ident != "Primitive" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

/// Derive flatten-compatible serde for a FHIR `value[x]` choice enum.
///
/// Each variant carries a single unnamed field and a `#[fhir("valueQuantity")]`
/// attribute giving its FHIR key. A variant whose field type is `Primitive<T>`
/// is a *primitive* choice: it serializes both the value key and, when present,
/// the paired `_value<Type>` extension key. Other variants serialize the value
/// key directly (their field is typically `Box<ComplexType>`).
///
/// `Serialize` emits a one- or two-entry map so `#[serde(flatten)]` merges the
/// keys onto the parent object; `Deserialize` scans a (flattened) map for those
/// keys, ignoring all others. Deserialization is lenient: a map with no value
/// key errors, which under `#[serde(flatten)]` on `Option<_>` becomes `None`.
#[proc_macro_derive(FhirChoice, attributes(fhir))]
pub fn derive_fhir_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input.ident, "FhirChoice can only derive on enums")
            .to_compile_error()
            .into();
    };

    let mut ser_arms = Vec::new();
    let mut val_decls = Vec::new();
    let mut key_arms = Vec::new();
    let mut build_arms = Vec::new();

    for variant in &data.variants {
        let vident = &variant.ident;
        // The FHIR key from #[fhir("...")].
        let key: LitStr = match variant
            .attrs
            .iter()
            .find(|a| a.path().is_ident("fhir"))
            .map(|a| a.parse_args::<LitStr>())
        {
            Some(Ok(k)) => k,
            _ => {
                return syn::Error::new_spanned(
                    vident,
                    "each FhirChoice variant needs #[fhir(\"valueXxx\")]",
                )
                .to_compile_error()
                .into();
            }
        };
        let key_str = key.value();
        let ext_key = LitStr::new(&format!("_{key_str}"), key.span());

        let Fields::Unnamed(f) = &variant.fields else {
            return syn::Error::new_spanned(vident, "FhirChoice variants must be tuple variants")
                .to_compile_error()
                .into();
        };
        let field_ty = &f.unnamed.first().expect("one field").ty;
        let val_var = format_ident!("__val_{}", vident);

        if let Some(inner) = primitive_inner(field_ty) {
            // Primitive variant: value + optional `_value<Type>` extension.
            let ext_var = format_ident!("__ext_{}", vident);
            ser_arms.push(quote! {
                #name::#vident(p) => {
                    ::serde::ser::SerializeMap::serialize_entry(&mut map, #key, &p.value)?;
                    if let ::core::option::Option::Some(e) = &p.extension {
                        ::serde::ser::SerializeMap::serialize_entry(&mut map, #ext_key, e)?;
                    }
                }
            });
            val_decls.push(quote! {
                let mut #val_var: ::core::option::Option<#inner> = ::core::option::Option::None;
                let mut #ext_var: ::core::option::Option<crate::r5::types::Element> =
                    ::core::option::Option::None;
            });
            key_arms.push(quote! {
                #key => { #val_var = ::core::option::Option::Some(map.next_value()?); }
                #ext_key => { #ext_var = ::core::option::Option::Some(map.next_value()?); }
            });
            build_arms.push(quote! {
                if let ::core::option::Option::Some(value) = #val_var {
                    return ::core::result::Result::Ok(#name::#vident(
                        crate::r5::choice::Primitive { value, extension: #ext_var }
                    ));
                }
            });
        } else {
            // Complex variant: a single value key.
            ser_arms.push(quote! {
                #name::#vident(v) => {
                    ::serde::ser::SerializeMap::serialize_entry(&mut map, #key, v)?;
                }
            });
            val_decls.push(quote! {
                let mut #val_var: ::core::option::Option<#field_ty> = ::core::option::Option::None;
            });
            key_arms.push(quote! {
                #key => { #val_var = ::core::option::Option::Some(map.next_value()?); }
            });
            build_arms.push(quote! {
                if let ::core::option::Option::Some(v) = #val_var {
                    return ::core::result::Result::Ok(#name::#vident(v));
                }
            });
        }
    }

    let expecting = format!("a FHIR {name} choice element");
    let visitor = format_ident!("__{}Visitor", name);

    quote! {
        impl ::serde::Serialize for #name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S)
                -> ::core::result::Result<S::Ok, S::Error>
            {
                let mut map = serializer.serialize_map(::core::option::Option::None)?;
                match self { #(#ser_arms)* }
                ::serde::ser::SerializeMap::end(map)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D)
                -> ::core::result::Result<Self, D::Error>
            {
                struct #visitor;
                impl<'de> ::serde::de::Visitor<'de> for #visitor {
                    type Value = #name;
                    fn expecting(&self, f: &mut ::core::fmt::Formatter)
                        -> ::core::fmt::Result
                    {
                        f.write_str(#expecting)
                    }
                    #[allow(non_snake_case)]
                    fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A)
                        -> ::core::result::Result<Self::Value, A::Error>
                    {
                        #(#val_decls)*
                        while let ::core::option::Option::Some(__k) =
                            map.next_key::<::std::string::String>()?
                        {
                            match __k.as_str() {
                                #(#key_arms)*
                                _ => { map.next_value::<::serde::de::IgnoredAny>()?; }
                            }
                        }
                        #(#build_arms)*
                        ::core::result::Result::Err(::serde::de::Error::custom(
                            "no value[x] variant present"
                        ))
                    }
                }
                deserializer.deserialize_map(#visitor)
            }
        }
    }
    .into()
}
