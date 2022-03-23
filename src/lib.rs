#![feature(proc_macro_diagnostic)]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

mod syntax;

use proc_macro::TokenStream;
use quote::quote;
use std::path::PathBuf;
use std::{env, fs};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields};

struct Source {
    code: String,
    origin: Option<PathBuf>,
}

#[proc_macro_derive(Oxiplate, attributes(oxi_code, oxi_path))]
pub fn oxiplate(input: TokenStream) -> TokenStream {
    match parse(input) {
        Ok(token_stream) => token_stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input = syn::parse(input).unwrap();
    let DeriveInput {
        attrs, ident, data, ..
    } = &input;

    let mut field_names: Vec<&syn::Ident> = Vec::new();
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
            return Err(syn::Error::new(input.span(), "Expected a struct"));
        }
    };

    let source = get_source(&attrs)?;
    let template = match syntax::parse(source.code.as_str().into(), &field_names) {
        Ok(template) => template,
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
            let origin = match source.origin {
                Some(origin) => origin,
                None => "Main file".into(),
            };
            panic!("{:?} in {:?}", err, origin);
        }
        Err(nom::Err::Incomplete(_)) => panic!("Unexpected incomplete error"),
    };

    let expanded = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #template
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

fn get_source(attrs: &Vec<Attribute>) -> Result<Source, syn::Error> {
    let invalid_attribute_message = r#"Must provide either an external or internal template:
External: #[oxi_path = "/absolute/path/to/template/within/project.txt.oxip"]
External: #[oxi_path = "./relative/path/to/template/from/current/file.txt.oxip"]
Internal: #[oxi_code = "{{ your_var }}"]"#;
    for attr in attrs {
        if attr.path.is_ident("oxi_code") {
            return match attr.parse_meta() {
                Ok(syn::Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(code),
                    ..
                })) => Ok(Source {
                    code: code.value(),
                    origin: None,
                }),
                Err(err) => Err(err),
                _ => Err(syn::Error::new(attr.span(), invalid_attribute_message)),
            };
        } else if attr.path.is_ident("oxi_path") {
            return match attr.parse_meta().expect("Unable to parse attribute") {
                syn::Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(path),
                    ..
                }) => {
                    let base_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                        .canonicalize()
                        .expect("Could not canonicalize CARGO_MANIFEST_DIR");
                    let path = PathBuf::from(path.value());
                    let path = if path.starts_with("/") {
                        base_path.join(
                            path.strip_prefix("/")
                                .expect("Could not strip leading slash"),
                        )
                    } else {
                        base_path
                            .join(
                                PathBuf::from(file!())
                                    .parent()
                                    .expect("Could not get parent directory of current file"),
                            )
                            .join(path)
                    };
                    let path = path.canonicalize().expect("Could not canonicalize");

                    if !path.starts_with(&base_path) {
                        panic!("Path {:?} must start with {:?}", path, base_path);
                    }

                    Ok(Source {
                        code: fs::read_to_string(&path).expect("Could not read file"),
                        origin: Some(path),
                    })
                }
                _ => Err(syn::Error::new(attr.span(), invalid_attribute_message)),
            };
        }
    }

    unimplemented!();
}
