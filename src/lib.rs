#![feature(proc_macro_diagnostic)]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

mod parser;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields};

#[proc_macro_derive(Oxiplate, attributes(code))]
pub fn oxiplate(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();
    let DeriveInput {
        attrs, ident, data, ..
    } = &input;

    let source = get_source(&attrs);

    let mut field_names = Vec::new();
    match data {
        Data::Struct(ref struct_item) => match &struct_item.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    match &field.ident {
                        Some(name) => field_names.push(name),
                        None => field.span().unwrap().error("Expected a named field").emit(),
                    }
                }
            }
            _ => (),
        },
        _ => {
            input.span().unwrap().error("Expected a struct").emit();
            return TokenStream::new();
        }
    };

    let expanded = quote! {
        use std::fmt;

        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                #(write!(f, "{} {}", #source, self.#field_names)?;)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_source(attrs: &Vec<Attribute>) -> String {
    for attr in attrs {
        if !attr.path.is_ident("code") {
            continue;
        }

        match attr.parse_meta().expect("Unable to parse attribute") {
            syn::Meta::NameValue(syn::MetaNameValue {
                lit: syn::Lit::Str(code),
                ..
            }) => return code.value(),
            _ => panic!(r#"Must provide template with #[code = "{{ your_var }}"]"#),
        }
    }

    unimplemented!();
}
