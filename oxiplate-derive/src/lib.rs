#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_expand)]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

mod source;
mod syntax;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use serde::Deserialize;
pub(crate) use source::Source;
use source::SourceOwned;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;
use syn::MetaNameValue;
use syn::Type;
use syn::{Attribute, Data, DeriveInput, Fields};

pub(crate) struct State<'a> {
    local_variables: &'a HashSet<&'a str>,
    config: &'a Config,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EscaperGroup {
    escaper: String,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(default)]
    default_escaper_group: Option<String>,
    #[serde(default)]
    escaper_groups: HashMap<String, EscaperGroup>,
}

#[proc_macro_derive(Oxiplate, attributes(oxiplate, oxiplate_inline, oxiplate_extends))]
pub fn oxiplate(input: TokenStream) -> TokenStream {
    match parse(input) {
        Ok(token_stream) => token_stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input = syn::parse(input).unwrap();
    let DeriveInput {
        attrs,
        ident,
        data,
        generics,
        ..
    } = &input;

    let mut field_names: Vec<&syn::Ident> = Vec::new();
    match data {
        Data::Struct(ref struct_item) => {
            if let Fields::Named(fields) = &struct_item.fields {
                for field in &fields.named {
                    match &field.ident {
                        Some(name) => field_names.push(name),
                        None => field.span().unwrap().error("Expected a named field").emit(),
                    }
                }
            }
        }
        _ => {
            return Err(syn::Error::new(input.span(), "Expected a struct"));
        }
    };

    let data_type = quote! { #ident #generics };
    let root = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(env::var("CARGO_MANIFEST_DIR"))
            .unwrap(),
    );
    let config_path = root.join("oxiplate.toml");
    let config: Config = if let Ok(toml) = fs::read_to_string(config_path.clone()) {
        toml::from_str(&toml).expect("Failed to parse oxiplate.toml")
    } else {
        Config {
            default_escaper_group: None,
            escaper_groups: HashMap::new(),
        }
    };
    let state = State {
        local_variables: &HashSet::new(),
        config: &config,
    };
    let source = parse_attributes(syn::parse2(data_type)?, data, attrs)?;
    let source = Source {
        original: &source,
        range: Range {
            start: 0,
            end: source.code.len(),
        },
    };
    let template = syntax::parse(&state, source);

    let where_clause = &generics.where_clause;
    let expanded = quote! {
        impl #generics std::fmt::Display for #ident #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #template
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

fn parse_attributes(
    data_type: Type,
    data: &Data,
    attrs: &Vec<Attribute>,
) -> Result<SourceOwned, syn::Error> {
    let invalid_attribute_message = r#"Must provide either an external or internal template:
External: #[oxiplate = "/path/to/template/from/templates/directory.txt.oxip"]
Internal: #[oxiplate_inline = "{{ your_var }}"]"#;
    for attr in attrs {
        let is_inline = attr.path().is_ident("oxiplate_inline");
        let is_extending = attr.path().is_ident("oxiplate_extends");
        if attr.path().is_ident("oxiplate") || is_inline || is_extending {
            return parse_attribute(
                data_type,
                data,
                attr,
                is_inline,
                is_extending,
                invalid_attribute_message,
            );
        }
    }

    unimplemented!();
}

fn parse_attribute(
    mut data_type: Type,
    data: &Data,
    attr: &Attribute,
    is_inline: bool,
    is_extending: bool,
    invalid_attribute_message: &str,
) -> Result<SourceOwned, syn::Error> {
    let (span, input, origin) = parse_source_tokens(attr, is_inline, is_extending);

    // Change the `proc_macro2::TokenStream` to a `proc_macro::TokenStream`
    let input = proc_macro::TokenStream::from(input);

    // Expand any macros, or fallback to the unexpanded input
    let input = input.expand_expr();
    if input.is_err() {
        return Err(syn::Error::new(span, invalid_attribute_message));
    }
    let input = input.unwrap();

    // Parse the string and token out of the expanded expression
    let parser = |input: syn::parse::ParseStream| input.parse::<syn::Lit>();
    let (code, literal) = match syn::parse::Parser::parse(parser, input)? {
        syn::Lit::Str(code) => (code.value(), code.token()),
        _ => Err(syn::Error::new(attr.span(), invalid_attribute_message))?,
    };

    let mut blocks = vec![];
    if is_extending {
        match data {
            Data::Struct(ref struct_item) => {
                if let Fields::Named(fields) = &struct_item.fields {
                    for field in &fields.named {
                        match &field.ident {
                            Some(name) => {
                                if *name == "_data" {
                                    data_type = field.ty.clone();
                                } else {
                                    blocks.push(name.to_string());
                                }
                            }
                            None => {
                                field.span().unwrap().error("Expected a named field").emit();
                            }
                        }
                    }
                }
            }
            _ => unreachable!("Should have checked this doesn't happen already"),
        }
    }

    // Return the source
    Ok(SourceOwned {
        data_type,
        blocks,
        code,
        literal,
        span_hygiene: span,
        origin,
        is_extending,
    })
}

fn parse_source_tokens(
    attr: &Attribute,
    is_inline: bool,
    is_extending: bool,
) -> (Span, proc_macro2::TokenStream, Option<PathBuf>) {
    if is_inline || is_extending {
        let syn::Meta::NameValue(MetaNameValue {
            path: _,
            eq_token: _,
            value: input,
        }) = attr.meta.clone()
        else {
            todo!("need to handle when non-name-value data is provided");
        };
        // Change the `syn::Expr` into a `proc_macro2::TokenStream`
        let span = input.span();
        return (span, quote::quote_spanned!(span=> #input), None);
    }

    let syn::Meta::NameValue(MetaNameValue {
        path: _,
        eq_token: _,
        value: Expr::Lit(ExprLit {
            attrs: _,
            lit: Lit::Str(path),
        }),
    }) = attr.meta.clone()
    else {
        todo!("need to handle when non-name-value data is provided");
    };
    let templates_dir = PathBuf::from(option_env!("OXIP_TEMPLATE_DIR").unwrap_or("templates"));
    let root = PathBuf::from(
        ::std::env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(::std::env::var("CARGO_MANIFEST_DIR"))
            .unwrap(),
    );

    // Path::join() doesn't play well with absolute paths (for our use case).
    let templates_dir_root = root.join(templates_dir.clone());
    if !templates_dir_root.starts_with(root) {
        panic!("OXIP_TEMPLATE_DIR must be a relative path; example: 'templates' instead of '/templates'. Provided: {}", templates_dir.display());
    }

    // Path::join() doesn't play well with absolute paths (for our use case).
    let full_path = templates_dir_root.join(path.value());
    if !full_path.starts_with(templates_dir_root) {
        panic!("Template path must be a relative path; example 'template.oxip' instead of '/template.oxip'. Provided: {}", path.value());
    }
    let span = path.span();
    let path = syn::LitStr::new(&full_path.to_string_lossy(), span);

    // Change the `syn::Expr` into a `proc_macro2::TokenStream`
    (
        span,
        quote::quote_spanned!(span=> include_str!(#path)),
        Some(full_path),
    )
}
