use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::{parse, parse_macro_input, punctuated::Punctuated, Ident, ItemStruct, LitStr, Token};

#[proc_macro_attribute]
pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub a: String })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[derive(Debug)]
struct Args {
    pub vars: Vec<LitStr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect::<Vec<LitStr>>(),
        })
    }
}

#[proc_macro_attribute]
pub fn add_field2(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let args = parse_macro_input!(args as Args);

    let extra_fields = args
        .vars
        .iter()
        .flat_map(|arg| {
            let ident = Ident::new(&arg.value(), arg.span());
            syn::Field::parse_named.parse2(quote! {
                pub #ident: String
            })
        })
        .collect::<Vec<_>>();

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in extra_fields {
            fields.named.push(field);
        }
    }

    return quote! {
        #item_struct
    }
    .into();
}
