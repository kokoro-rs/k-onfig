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
                        __map.insert(stringify!(#name),<#ty as ::k_onfig::konfig::Konfig>::konfig());
                    }
                })
                .collect()
        }
        _ => {
            panic!("必须是具名结构体")
        }
    };
    quote! {
        const _:() = {
            extern crate k_onfig as _k;
            #[automatically_derived]
            impl _k::konfig::Konfig for #s_ident {
                fn konfig() -> _k::KType {
                    let mut __map: ::std::collections::BTreeMap<&'static str, _k::KType> = ::std::collections::BTreeMap::new();

                    #(#exps)*

                    _k::KType::Object(__map)
                }
            }
        };
    }
    .into()
}
