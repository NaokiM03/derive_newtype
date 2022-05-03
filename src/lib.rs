use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Newtype)]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    if item.fields.len() != 1 {
        panic!("The newtype pattern must be a tuple struct with only one type.");
    }
    let struct_name = &item.ident;
    let inner_type = &item.fields.iter().next().unwrap().ty;
    let generics = &item.generics;

    quote!(
        impl #generics core::ops::Deref for #struct_name #generics {
            type Target = #inner_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl #generics core::ops::DerefMut for #struct_name #generics {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl #generics From<#inner_type> for #struct_name #generics {
            fn from(inner: #inner_type) -> #struct_name #generics {
                #struct_name (inner)
            }
        }
    )
    .into()
}
