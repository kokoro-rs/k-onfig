use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Konfig)]
pub fn konfig(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let s_ident = input.ident;
    let exps: Vec<TokenStream2> = match &input.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let named = &fields.named.iter();
            named
                .clone()
                .map(|n| {
                    let name = n.ident.clone().unwrap();
                    let ty = n.ty.clone();
                    quote! {
                        map.insert(stringify!(#name),<#ty as ::k_onfig::konfig::Konfig>::konfig());
                    }
                })
                .collect()
        }
        _ => {
            panic!("必须是具名结构体")
        }
    };
    quote! {
        impl ::k_onfig::konfig::Konfig for #s_ident {
            fn konfig() -> ::k_onfig::KType {
                let mut map: ::std::collections::BTreeMap<&'static str, ::k_onfig::KType> = ::std::collections::BTreeMap::new();

                #(#exps)*

                ::k_onfig::KType::Object(map)
            }
        }
    }
    .into()
}
