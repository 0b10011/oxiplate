#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Item};

struct Template {}

impl Parse for Template {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Template {})
    }
}

#[proc_macro]
pub fn template(tokens: TokenStream) -> TokenStream {
    let Template {} = parse_macro_input!(tokens as Template);

    let expanded = quote! {};

    TokenStream::from(expanded)
}

struct Rustem {
    template: Expr,
    data: Item,
}

impl Parse for Rustem {
    fn parse(input: ParseStream) -> Result<Self> {
        let template: Expr = input.parse()?;
        let data = input.parse::<Item>()?;

        Ok(Rustem { template, data })
    }
}

#[proc_macro]
pub fn rustem(tokens: TokenStream) -> TokenStream {
    let Rustem { template, data } = parse_macro_input!(tokens as Rustem);

    let (name, fields) = match data {
        Item::Struct(ref struct_item) => (&struct_item.ident, &struct_item.fields),
        _ => {
            data.span().unwrap().error("Expected a struct").emit();
            return TokenStream::new();
        }
    };

    let expanded = quote! {
        #data

        use std::fmt;

        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Test")
            }
        }
    };

    TokenStream::from(expanded)
}
