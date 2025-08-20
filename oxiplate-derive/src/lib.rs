#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_expand)]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

mod source;
mod state;
mod syntax;

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Range;
use std::path::PathBuf;

use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{quote, quote_spanned};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprLit, Fields, Ident, Lit, LitStr, MetaList,
    MetaNameValue,
};

pub(crate) use self::source::Source;
use self::source::SourceOwned;
use self::state::build_config;
pub(crate) use self::state::State;
use crate::state::Config;

/// Derives the `::std::fmt::Display` implementation for a template's struct.
///
/// # Usage
///
/// See the [getting started docs](https://0b10011.io/oxiplate/getting-started.html) for more information.
///
/// ```
/// # use oxiplate_derive::Oxiplate;
/// #[derive(Oxiplate)]
/// #[oxiplate = "example.html.oxip"]
/// struct Homepage {
///     // ...
/// #    site_name: &'static str,
/// #    title: &'static str,
/// #    message: &'static str,
/// }
///
/// fn main() {
///     let homepage = Homepage {
///         // ...
/// #        site_name: "Oxiplate Documentation",
/// #        title: "Derive Macro Description",
/// #        message: "Hello world!",
///     };
///     print!("{}", homepage);
/// }
/// ```
///
/// or:
///
/// ```
/// # use oxiplate_derive::Oxiplate;
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(
///     "{-}
/// <!DOCTYPE html>
/// <title>{{ title }} - {{ site_name }}</title>
/// <h1>{{ title }}</h1>
/// <p>{{ message }}</p>
/// "
/// )]
/// struct Homepage {
///     // ...
/// #    site_name: &'static str,
/// #    title: &'static str,
/// #    message: &'static str,
/// }
///
/// fn main() {
///     let homepage = Homepage {
///         // ...
/// #        site_name: "Oxiplate Documentation",
/// #        title: "Derive Macro Description",
/// #        message: "Hello world!",
///     };
///     print!("{}", homepage);
/// }
/// ```
#[proc_macro_derive(
    Oxiplate,
    attributes(oxiplate, oxiplate_inline, oxiplate_extends, oxiplate_include)
)]
pub fn oxiplate(input: TokenStream) -> TokenStream {
    oxiplate_internal(input, &VecDeque::from([&HashMap::new()])).0
}

/// Internal derive function that allows for block token streams to be passed in.
pub(crate) fn oxiplate_internal(
    input: TokenStream,
    blocks: &VecDeque<&HashMap<&str, (&syntax::Template, Option<&syntax::Template>)>>,
) -> (TokenStream, usize) {
    match parse_template_and_data(input, blocks) {
        Ok(token_stream) => token_stream,
        Err(err) => (err.to_compile_error().into(), 0),
    }
}

/// Parses the template information from the attributes
/// and data information from the associated struct.
/// Returns the token stream for the `::std::fmt::Display` implementation for the struct.
fn parse_template_and_data(
    input: TokenStream,
    blocks: &VecDeque<&HashMap<&str, (&syntax::Template, Option<&syntax::Template>)>>,
) -> Result<(TokenStream, usize), syn::Error> {
    let input = syn::parse(input).unwrap();
    let DeriveInput {
        attrs,
        ident,
        data,
        generics,
        ..
    } = &input;

    // Ensure the data is a struct
    match data {
        Data::Struct(ref _struct_item) => (),
        _ => {
            return Err(syn::Error::new(input.span(), "Expected a struct"));
        }
    }

    // Build the shared state from the `oxiplate.toml` file.
    let config = build_config(&input)?;
    let mut state = State {
        local_variables: &HashSet::new(),
        inferred_escaper_group: None,
        config: &config,
        blocks,
        has_content: &false,
    };

    // Parse the template type and code literal.
    let (attr, template_type) = parse_template_type(attrs);
    let parsed_tokens = parse_source_tokens(attr, &template_type, &mut state);
    let (template, estimated_length): (proc_macro2::TokenStream, usize) =
        process_parsed_tokens(parsed_tokens, &config, &template_type, data, &state)?;

    // Internally, the template is used directly instead of via `Display`/`Render`.
    if template_type == TemplateType::Extends || template_type == TemplateType::Include {
        return Ok((template.into(), estimated_length));
    }

    let where_clause = &generics.where_clause;
    let expanded = if *state.config.optimized_renderer {
        quote! {
            impl #generics ::std::fmt::Display for #ident #generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::oxiplate::Render::render_into(self, f)
                }
            }
            impl #generics ::oxiplate::Render for #ident #generics #where_clause {
                const ESTIMATED_LENGTH: usize = #estimated_length;

                #[inline]
                fn render_into<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
                    use ::std::fmt::Write;
                    use ::oxiplate::unescaped_text::UnescapedText;
                    #template
                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl #generics ::std::fmt::Display for #ident #generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let string = {
                        use ::std::fmt::Write;
                        let mut string = String::with_capacity(#estimated_length);
                        let f = &mut string;
                        #template
                        string
                    };
                    f.write_str(&string)
                }
            }
        }
    };

    Ok((TokenStream::from(expanded), estimated_length))
}

fn process_parsed_tokens(
    parsed_tokens: Result<(Span, proc_macro2::TokenStream, Option<PathBuf>), (String, Span)>,
    config: &Config,
    template_type: &TemplateType,
    data: &Data,
    state: &State,
) -> Result<(proc_macro2::TokenStream, usize), syn::Error> {
    match parsed_tokens {
        Err((escaper, span)) => {
            let registered_escaper_groups = config
                .escaper_groups
                .keys()
                .map(|key| &**key)
                .collect::<Vec<&str>>()
                .join(", ");
            let template = match template_type {
                TemplateType::Path | TemplateType::Extends | TemplateType::Include => {
                    quote_spanned! {span=> compile_error!(concat!("The file extension `.", #escaper, "` is not registered as an escaper group in `/oxiplate.toml`. All used template extensions must be registered. Registered escaper groups: ", #registered_escaper_groups)); }
                }
                TemplateType::Inline => {
                    quote_spanned! {span=> compile_error!(concat!("The specified escaper group `", #escaper, "` is not registered in `/oxiplate.toml`. Registered escaper groups: ", #registered_escaper_groups)); }
                }
            };
            Ok((template, 0))
        }
        Ok((span, input, origin)) => {
            let (code, literal) = parse_code_literal(template_type, &input.into(), span)?;

            // Parse the fields and adjust the data type if needed.
            let (_fields, blocks) = parse_fields(data, *template_type == TemplateType::Extends);

            // Build the source.
            let owned_source = SourceOwned {
                blocks,
                code,
                literal,
                span_hygiene: span,
                origin,
                is_extending: *template_type == TemplateType::Extends,
            };
            let source = Source {
                original: &owned_source,
                range: Range {
                    start: 0,
                    end: owned_source.code.len(),
                },
            };

            // Build the `::std::fmt::Display` implementation for the struct.
            // (This is where the template is actually parsed.)
            Ok(syntax::parse(state, source))
        }
    }
}

#[derive(PartialEq, Eq)]
enum TemplateType {
    Path,
    Inline,
    Extends,
    Include,
}

/// Parse the attributes to figure out what type of template this struct references.
fn parse_template_type(attrs: &Vec<Attribute>) -> (&Attribute, TemplateType) {
    for attr in attrs {
        let path = attr.path();
        let template_type = if path.is_ident("oxiplate_inline") {
            TemplateType::Inline
        } else if path.is_ident("oxiplate_extends") {
            TemplateType::Extends
        } else if path.is_ident("oxiplate_include") {
            TemplateType::Include
        } else if path.is_ident("oxiplate") {
            TemplateType::Path
        } else {
            continue;
        };

        return (attr, template_type);
    }

    unimplemented!("Must specify an attribute");
}

fn parse_code_literal(
    template_type: &TemplateType,
    input: &TokenStream,
    span: Span,
) -> Result<(String, Literal), syn::Error> {
    let invalid_attribute_message = match template_type {
        TemplateType::Path | TemplateType::Inline => {
            r#"Must provide either an external or internal template:
External: #[oxiplate = "path/to/template/from/templates/directory.html.oxip"]
Internal: #[oxiplate_inline(html: "{{ your_var }}")]"#
        }
        TemplateType::Extends => {
            r#"Must provide a path to a template that exists. E.g., `{% extends "path/to/template.html.oxip" %}`"#
        }
        TemplateType::Include => {
            r#"Must provide a path to a template that exists. E.g., `{% include "path/to/template.html.oxip" %}`"#
        }
    };

    // Expand any macros, or fallback to the unexpanded input
    let input = input.expand_expr();
    if input.is_err() {
        return Err(syn::Error::new(span, invalid_attribute_message));
    }
    let input = input.unwrap();

    // Parse the string and token out of the expanded expression
    let parser = |input: syn::parse::ParseStream| input.parse::<LitStr>();
    let code = syn::parse::Parser::parse(parser, input)?;
    Ok((code.value(), code.token()))
}

fn parse_source_tokens(
    attr: &Attribute,
    template_type: &TemplateType,
    #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))] state: &mut State,
) -> Result<(Span, proc_macro2::TokenStream, Option<PathBuf>), (String, Span)> {
    match template_type {
        TemplateType::Inline => parse_source_tokens_for_inline(attr, state),
        TemplateType::Path | TemplateType::Extends | TemplateType::Include => {
            parse_source_tokens_for_path(attr, state)
        }
    }
}

/// An inline template, with or without escaper information.
enum Template {
    WithEscaper(TemplateWithEscaper),
    WithoutEscaper(TemplateWithoutEscaper),
}

impl Parse for Template {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            input.parse().map(Template::WithEscaper)
        } else {
            input.parse().map(Template::WithoutEscaper)
        }
    }
}

/// An inline template with escaper information.
struct TemplateWithEscaper {
    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    escaper: Ident,
    #[allow(dead_code)]
    colon: Colon,
    #[cfg_attr(not(feature = "oxiplate"), allow(dead_code))]
    template: Expr,
}

impl Parse for TemplateWithEscaper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(TemplateWithEscaper {
            escaper: input.parse()?,
            colon: input.parse()?,
            template: input.parse()?,
        })
    }
}

/// An inline template without escaper information.
struct TemplateWithoutEscaper {
    template: Expr,
}

impl Parse for TemplateWithoutEscaper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(TemplateWithoutEscaper {
            template: input.parse()?,
        })
    }
}

#[cfg_attr(not(feature = "oxiplate"), allow(clippy::unnecessary_wraps))]
fn parse_source_tokens_for_inline(
    attr: &Attribute,
    #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))] state: &mut State,
) -> Result<(Span, proc_macro2::TokenStream, Option<PathBuf>), (String, Span)> {
    match attr.meta.clone() {
        syn::Meta::Path(_path) => unimplemented!(
            r#"Must provide either an external or internal template:
External: #[oxiplate = "/path/to/template/from/templates/directory.txt.oxip"]
Internal: #[oxiplate_inline(html: "{{ your_var }}")]"#
        ),
        syn::Meta::List(MetaList {
            path: _,
            delimiter: _,
            tokens,
        }) => match syn::parse2::<Template>(tokens) {
            #[cfg(not(feature = "oxiplate"))]
            Ok(Template::WithEscaper(_template)) => unimplemented!(
                "Escaping requires the `oxiplate` library, but you appear to be using \
                 `oxiplate-derive` directly. Replacing `oxiplate-derive` with `oxiplate` in the \
                 dependencies should fix this issue, although you may need to turn off some \
                 default features if you want it to work the same way."
            ),
            #[cfg(feature = "oxiplate")]
            Ok(Template::WithEscaper(TemplateWithEscaper {
                escaper,
                colon: _,
                template,
            })) => {
                let span = template.span();
                if *state.config.infer_escaper_group_from_file_extension {
                    if let Some(group_path) = state.config.escaper_groups.get(&escaper.to_string())
                    {
                        state.inferred_escaper_group = Some(group_path);
                    } else {
                        return Err((escaper.to_string(), escaper.span()));
                    }
                }
                Ok((span, quote::quote_spanned!(span=> #template), None))
            }
            Ok(Template::WithoutEscaper(TemplateWithoutEscaper { template })) => {
                let span = template.span();
                Ok((span, quote::quote_spanned!(span=> #template), None))
            }
            Err(_) => unimplemented!(
                r#"Must provide either an external or internal template:
External: #[oxiplate = "/path/to/template/from/templates/directory.txt.oxip"]
Internal: #[oxiplate_inline(html: "{{ your_var }}")]"#
            ),
        },
        syn::Meta::NameValue(_) => unimplemented!(
            r#"Inline templates must be defined with the following syntax:
With an escaper group: #[oxiplate_inline(html: "{{ your_var }}"]
Without an escaper group: #[oxiplate_inline("{{ your_var }}")]"#
        ),
    }
}

#[cfg_attr(not(feature = "oxiplate"), allow(clippy::unnecessary_wraps))]
fn parse_source_tokens_for_path(
    attr: &Attribute,
    #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))] state: &mut State,
) -> Result<(Span, proc_macro2::TokenStream, Option<PathBuf>), (String, Span)> {
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
        panic!(
            "OXIP_TEMPLATE_DIR must be a relative path; example: 'templates' instead of \
             '/templates'. Provided: {}",
            templates_dir.display()
        );
    }

    // Path::join() doesn't play well with absolute paths (for our use case).
    let full_path = templates_dir_root.join(path.value());
    if !full_path.starts_with(templates_dir_root) {
        panic!(
            "Template path must be a relative path; example 'template.oxip' instead of \
             '/template.oxip'. Provided: {}",
            path.value()
        );
    }
    let span = path.span();
    let path = syn::LitStr::new(&full_path.to_string_lossy(), span);

    // Infer the escaper from the template's file extension.
    // Only works when using `oxiplate` rather than `oxiplate-derive` directly.
    #[cfg(feature = "oxiplate")]
    if *state.config.infer_escaper_group_from_file_extension {
        // Get the template's file extension,
        // but ignore `.oxip`.
        let path_value = path.value();
        let mut extensions = path_value.split('.');
        let mut extension = extensions.next_back();
        if extension == Some("oxip") {
            extension = extensions.next_back();
        }

        // `raw` is a special keyword that should be ignored.
        if extension == Some("raw") {
            extension = None;
        }

        // Set the inferred escaper group if the extension mapped to one.
        if let Some(extension) = extension {
            if let Some(group_path) = state.config.escaper_groups.get(extension) {
                state.inferred_escaper_group = Some(group_path);
            } else {
                return Err((extension.to_string(), path.span()));
            }
        }
    }

    // Change the `syn::Expr` into a `proc_macro2::TokenStream`
    Ok((
        span,
        quote::quote_spanned!(span=> include_str!(#path)),
        Some(full_path),
    ))
}

fn parse_fields(
    data: &Data,
    is_extending: bool,
) -> (
    std::vec::Vec<&proc_macro2::Ident>,
    std::vec::Vec<std::string::String>,
) {
    let mut field_names: Vec<&syn::Ident> = Vec::new();
    let mut blocks: Vec<String> = vec![];

    match data {
        Data::Struct(ref struct_item) => match &struct_item.fields {
            // A named struct like `Data { title: &'static str }`.
            Fields::Named(fields) => {
                for field in &fields.named {
                    match &field.ident {
                        Some(name) => {
                            if is_extending {
                                blocks.push(name.to_string());
                            } else {
                                field_names.push(name);
                            }
                        }
                        None => unreachable!("Named fields should always have a name."),
                    }
                }
            }

            // While there aren't any accessible fields,
            // it could still be useful to have a template set up as one of these.
            Fields::Unnamed(_) | Fields::Unit => (),
        },
        _ => unreachable!("Data should have already been verified to be a struct"),
    }

    (field_names, blocks)
}
