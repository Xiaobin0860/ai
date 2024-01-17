extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields};

#[proc_macro_derive(FromNode)]
pub fn derive_from_node(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        panic!("FromNode can only be applied to enums");
    };

    let from_impls = variants.into_iter().map(|variant| {
        let variant_name = &variant.ident;
        let fields = match variant.fields {
            Fields::Unnamed(fields) => fields.unnamed,
            _ => panic!("Enum variants should be tuples"),
        };

        let field_type = &fields.first().expect("Enum variant should have a field").ty;

        quote! {
            impl From<&crate::Node> for #field_type {
                fn from(value: &crate::Node) -> Self {
                    match &value.inputs {
                        Inputs::#variant_name(v) => v.clone(),
                        _ => panic!(stringify!(#variant_name)),
                    }
                }
            }
        }
    });

    let expanded = quote! {
        #(#from_impls)*
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl From<serde_json::Value> for #name {
            fn from(v: serde_json::Value) -> Self {
                serde_json::from_value(v)
                    .context(stringify!(#name))
                    .unwrap()
            }
        }
    };
    TokenStream::from(expanded)
}
