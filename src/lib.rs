use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, parse::Parser};
use quote::quote;

#[proc_macro_attribute]
pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! { pub a: String }).unwrap());
                }   
                _ => {
                    ()
                }
            }              
            
            // I tried
            // 
            return quote! {
                #ast
            }.into();
        }
        _ => panic!("AddField has to be used with structs "),
    }
}