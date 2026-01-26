#![cfg_attr(feature = "better-internal-errors", feature(proc_macro_diagnostic))]
#![cfg_attr(feature = "external-template-spans", feature(proc_macro_expand))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

mod source;
mod state;
mod syntax;
mod tokenizer;

use std::collections::{HashMap, VecDeque};
#[cfg(not(feature = "external-template-spans"))]
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[cfg(feature = "better-internal-errors")]
use percent_encoding::percent_encode_byte;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprLit, Ident, Lit, LitStr, MetaList, MetaNameValue,
};

type BuiltTokens = (proc_macro2::TokenStream, usize);

pub(crate) use self::source::Source;
use self::source::SourceOwned;
pub(crate) use self::state::State;
use self::state::build_config;
use crate::state::{LocalVariables, OptimizedRenderer};
use crate::tokenizer::{TokenSlice, Tokens};

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
    #[cfg(feature = "unreachable")]
    let input = {
        // Compare tokens as a string to avoid having to parse unnecessarily.
        if input.to_string()
            == r#"#[oxiplate_inline("hello world")] struct UnreachableUnparseableInput;"#
                .to_string()
        {
            // Unparseable code that would otherwise fail before reaching this point.
            quote! { struct 19foo; }.into()
        } else {
            input
        }
    };

    oxiplate_internal(input, &VecDeque::from([&HashMap::new()])).0
}

/// Internal derive function that allows for block token streams to be passed in.
pub(crate) fn oxiplate_internal(
    input: TokenStream,
    blocks: &VecDeque<&HashMap<&str, (BuiltTokens, Option<BuiltTokens>)>>,
) -> (TokenStream, usize) {
    let input = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return (err.to_compile_error().into(), 0),
    };
    parse_input(&input, blocks)
}

/// Parses the template information from the attributes
/// and data information from the associated struct.
/// Returns the token stream for the `::std::fmt::Display` implementation for the struct.
fn parse_input(
    input: &DeriveInput,
    blocks: &VecDeque<&HashMap<&str, (BuiltTokens, Option<BuiltTokens>)>>,
) -> (TokenStream, usize) {
    let DeriveInput {
        ident, generics, ..
    } = &input;

    let (template, estimated_length, template_type, optimized_renderer): (
        proc_macro2::TokenStream,
        usize,
        TemplateType,
        OptimizedRenderer,
    ) = match parse_template_and_data(input, blocks) {
        Ok(data) => data,
        Err((err, template_type, optimized_renderer)) => (
            err.to_compile_error(),
            0,
            template_type.unwrap_or(TemplateType::Inline),
            optimized_renderer,
        ),
    };

    // Internally, the template is used directly instead of via `Display`/`Render`.
    if let TemplateType::Extends | TemplateType::Include = template_type {
        return (template.into(), estimated_length);
    }

    let where_clause = &generics.where_clause;
    let expanded = if *optimized_renderer {
        #[cfg(not(feature = "oxiplate"))]
        quote! {
            compile_error!(
                "`optimized_renderer` config option specified in `/oxiplate.toml` is only available when using `oxiplate`. It looks like `oxiplate-derive` is being used directly instead."
            );
            impl #generics ::std::fmt::Display for #ident #generics #where_clause {
                fn fmt(&self, oxiplate_formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let string = {
                        use ::std::fmt::Write;
                        let mut string = String::with_capacity(#estimated_length);
                        let oxiplate_formatter = &mut string;
                        #template
                        string
                    };
                    oxiplate_formatter.write_str(&string)
                }
            }
        }

        #[cfg(feature = "oxiplate")]
        quote! {
            impl #generics ::std::fmt::Display for #ident #generics #where_clause {
                fn fmt(&self, oxiplate_formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::oxiplate::Render::render_into(self, oxiplate_formatter)
                }
            }
            impl #generics ::oxiplate::Render for #ident #generics #where_clause {
                const ESTIMATED_LENGTH: usize = #estimated_length;

                #[inline]
                fn render_into<W: ::std::fmt::Write>(&self, oxiplate_formatter: &mut W) -> ::std::fmt::Result {
                    use ::std::fmt::Write;
                    use ::oxiplate::{ToCowStr, UnescapedText};
                    #template
                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl #generics ::std::fmt::Display for #ident #generics #where_clause {
                fn fmt(&self, oxiplate_formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let string = {
                        use ::std::fmt::Write;
                        let mut string = String::with_capacity(#estimated_length);
                        let oxiplate_formatter = &mut string;
                        #template
                        string
                    };
                    oxiplate_formatter.write_str(&string)
                }
            }
        }
    };

    (TokenStream::from(expanded), estimated_length)
}

type ParsedTemplate = (
    proc_macro2::TokenStream,
    usize,
    TemplateType,
    OptimizedRenderer,
);

fn parse_template_and_data(
    input: &DeriveInput,
    blocks: &VecDeque<&HashMap<&str, (BuiltTokens, Option<BuiltTokens>)>>,
) -> Result<ParsedTemplate, (syn::Error, Option<TemplateType>, OptimizedRenderer)> {
    // Build the shared config from the `oxiplate.toml` file.
    let config =
        build_config(input).map_err(|(err, optimized_renderer)| (err, None, optimized_renderer))?;

    let DeriveInput {
        attrs, ident, data, ..
    } = &input;

    // Ensure the data is a struct
    match data {
        Data::Struct(_struct_item) => (),
        _ => {
            return Err((
                syn::Error::new(input.span(), "Expected a struct"),
                None,
                config.optimized_renderer,
            ));
        }
    }

    let (attr, template_type) = parse_template_type(attrs, ident.span())
        .map_err(|err: syn::Error| (err, None, config.optimized_renderer.clone()))?;

    let optimized_renderer = config.optimized_renderer.clone();

    let mut state = State {
        local_variables: LocalVariables::new(),
        inferred_escaper_group: None,
        default_escaper_group: None,
        failed_to_set_default_escaper_group: false,
        config,
        blocks,
        has_content: false,
    };

    let parsed_tokens = parse_source_tokens(attr, &template_type, &mut state);
    let (template, estimated_length): BuiltTokens = process_parsed_tokens(
        parsed_tokens,
        &mut state,
        #[cfg(any(feature = "oxiplate", feature = "external-template-spans"))]
        &template_type,
    )
    .map_err(|err: syn::Error| (err, Some(template_type.clone()), optimized_renderer.clone()))?;

    Ok((
        template,
        estimated_length,
        template_type,
        optimized_renderer,
    ))
}

type ParsedTokens = Result<
    (
        Span,
        proc_macro2::TokenStream,
        Option<PathBuf>,
        Option<String>,
    ),
    ParsedEscaperError,
>;

fn process_parsed_tokens<'a>(
    parsed_tokens: ParsedTokens,
    state: &'a mut State<'a>,
    #[cfg(any(feature = "oxiplate", feature = "external-template-spans"))]
    template_type: &TemplateType,
) -> Result<BuiltTokens, syn::Error> {
    match parsed_tokens {
        #[cfg(feature = "oxiplate")]
        Err(ParsedEscaperError::EscaperNotFound((escaper, span))) => {
            let mut available_escaper_groups = state
                .config
                .escaper_groups
                .keys()
                .map(|key| &**key)
                .collect::<Vec<&str>>();
            available_escaper_groups.sort_unstable();
            let available_escaper_groups = available_escaper_groups.join(", ");
            let template = match template_type {
                TemplateType::Path | TemplateType::Extends | TemplateType::Include => {
                    internal_error!(
                        span.unwrap(),
                        "Unregistered file extension causing `EscaperNotFound` error",
                        .help(format!("Extension found: {escaper}"))
                        .help(format!("Registered escaper groups: {available_escaper_groups}"))
                    );
                }
                TemplateType::Inline => {
                    let available_escaper_groups = LitStr::new(&available_escaper_groups, span);
                    quote_spanned! {span=> compile_error!(concat!("The specified escaper group `", #escaper, "` is not registered in `/oxiplate.toml`. Registered escaper groups: ", #available_escaper_groups)); }
                }
            };
            Ok((template, 0))
        }
        Err(ParsedEscaperError::ParseError(compile_error)) => Ok((compile_error, 0)),
        Ok((span, input, origin, inferred_escaper_group_name)) => {
            let code = parse_code_literal(
                &input.into(),
                #[cfg(feature = "external-template-spans")]
                template_type,
                #[cfg(feature = "external-template-spans")]
                span,
            )?;

            if let Some(inferred_escaper_group_name) = &inferred_escaper_group_name {
                state.inferred_escaper_group = Some((
                    inferred_escaper_group_name.to_owned(),
                    state
                        .config
                        .escaper_groups
                        .get(inferred_escaper_group_name)
                        .expect("Escaper group should have already been checked for existence")
                        .clone(),
                ));
            }

            // Build the source.
            let owned_source = SourceOwned::new(&code, span, origin);
            let source = Source::new(&owned_source);
            let (tokens, eof) = Tokens::new(source).tokens_and_eof();
            let tokens = TokenSlice::new(&tokens, &eof);

            // Build the `::std::fmt::Display` implementation for the struct.
            // (This is where the template is actually parsed.)
            Ok(syntax::parse(state, tokens))
        }
    }
}

#[derive(Clone)]
enum TemplateType {
    Path,
    Inline,
    Extends,
    Include,
}

/// Parse the attributes to figure out what type of template this struct references.
fn parse_template_type(
    attrs: &Vec<Attribute>,
    span: Span,
) -> Result<(&Attribute, TemplateType), syn::Error> {
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

        return Ok((attr, template_type));
    }

    Err(syn::Error::new(
        span,
        r#"Expected an attribute named `oxiplate_inline` or `oxiplate` to specify the template:
External: #[oxiplate = "path/to/template/from/templates/directory.html.oxip"]
Internal: #[oxiplate_inline(html: "{{ your_var }}")]"#,
    ))
}

fn parse_code_literal(
    input: &TokenStream,
    #[cfg(feature = "external-template-spans")] template_type: &TemplateType,
    #[cfg(feature = "external-template-spans")] span: Span,
) -> Result<LitStr, syn::Error> {
    #[cfg(feature = "external-template-spans")]
    let input = {
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

        // Expand macros
        let input = input.expand_expr();
        if input.is_err() {
            return Err(syn::Error::new(span, invalid_attribute_message));
        }
        input.unwrap()
    };

    #[cfg(not(feature = "external-template-spans"))]
    let input = input.clone();

    // Parse the string and token out of the expanded expression
    let parser = |input: syn::parse::ParseStream| input.parse::<LitStr>();
    let code = syn::parse::Parser::parse(parser, input)?;
    Ok(code)
}

fn parse_source_tokens(
    attr: &Attribute,
    template_type: &TemplateType,
    #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))] state: &mut State,
) -> ParsedTokens {
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

#[cfg_attr(not(feature = "external-template-spans"), derive(Debug))]
enum ParsedEscaperError {
    #[cfg(feature = "oxiplate")]
    EscaperNotFound((String, Span)),
    ParseError(proc_macro2::TokenStream),
}

#[cfg_attr(not(feature = "oxiplate"), allow(clippy::unnecessary_wraps))]
fn parse_source_tokens_for_inline(
    attr: &Attribute,
    #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))] state: &mut State,
) -> ParsedTokens {
    match &attr.meta {
        syn::Meta::Path(path) => {
            let span = path.span();
            Err(ParsedEscaperError::ParseError(quote_spanned! {span=>
                compile_error!(r#"Must provide either an external or internal template:
External: #[oxiplate = "/path/to/template/from/templates/directory.txt.oxip"]
Internal: #[oxiplate_inline(html: "{{ your_var }}")]"#);
            }))
        }
        syn::Meta::List(MetaList {
            path: _,
            delimiter: _,
            tokens,
        }) => match syn::parse2::<Template>(tokens.clone()) {
            #[cfg(not(feature = "oxiplate"))]
            Ok(Template::WithEscaper(template)) => {
                let span = template.escaper.span();
                Err(ParsedEscaperError::ParseError(quote_spanned! {span=>
                    compile_error!("Escaping requires the `oxiplate` library, but you appear to be using \
                 `oxiplate-derive` directly. Replacing `oxiplate-derive` with `oxiplate` in the \
                 dependencies should fix this issue, although you may need to turn off some \
                 default features if you want it to work the same way.");
                }))
            }
            #[cfg(feature = "oxiplate")]
            Ok(Template::WithEscaper(TemplateWithEscaper {
                escaper,
                colon: _,
                template,
            })) => {
                let span = template.span();

                let escaper_name = escaper.to_string();
                if !state.config.escaper_groups.contains_key(&escaper_name) {
                    return Err(ParsedEscaperError::EscaperNotFound((
                        escaper_name,
                        escaper.span(),
                    )));
                }

                Ok((
                    span,
                    quote::quote_spanned!(span=> #template),
                    None,
                    Some(escaper_name),
                ))
            }
            Ok(Template::WithoutEscaper(TemplateWithoutEscaper { template })) => {
                let span = template.span();
                Ok((span, quote::quote_spanned!(span=> #template), None, None))
            }
            Err(error) => {
                let span = error.span();
                let compile_error = error.to_compile_error();
                Err(ParsedEscaperError::ParseError(quote_spanned! {span=>
                    compile_error!("Failed to parse inline template. Should look something like:\n#[oxiplate_inline(html: \"{{ your_var }}\")]");
                    #compile_error
                }))
            }
        },
        syn::Meta::NameValue(meta) => {
            let span = meta.span();
            Err(ParsedEscaperError::ParseError(quote_spanned! {span=>
                compile_error!("Incorrect syntax for inline template. Should look something like:\n#[oxiplate_inline(html: \"{{ your_var }}\")]");
            }))
        }
    }
}

/// Build the absolute template directory
/// from the package's directory and provided relative template directory.
fn templates_dir(span: Span) -> Result<PathBuf, ParsedEscaperError> {
    let default_template_dir = String::from("templates");

    let (specified_templates_dir, using_default_template_dir) =
        if let Ok(templates_dir) = ::std::env::var("OXIP_TEMPLATE_DIR") {
            (templates_dir, false)
        } else {
            (default_template_dir, true)
        };
    let root = PathBuf::from(
        ::std::env::var("CARGO_MANIFEST_DIR_OVERRIDE")
            .or(::std::env::var("CARGO_MANIFEST_DIR"))
            .expect("`CARGO_MANIFEST_DIR` should be present"),
    );

    // Path::join() doesn't play well with absolute paths (for our use case).
    root
        .append_path(&specified_templates_dir, false)
        .map_err(|err| -> ParsedEscaperError {
            match err {
                AppendPathError::DoesNotExist(path_buf) => {
                    let path_buf = path_buf.to_string_lossy();
                    ParsedEscaperError::ParseError(quote_spanned! {span=>
                        compile_error!(concat!("Template directory `", #path_buf, "` not found."));
                    })
                },
                AppendPathError::IsSymlink(path_buf) => {
                    let path_buf = path_buf.to_string_lossy();
                    ParsedEscaperError::ParseError(quote_spanned! {span=>
                        compile_error!(concat!("Template directory `", #path_buf, "` cannot be a symlink."));
                    })
                },
                AppendPathError::CanonicalizeError(path_buf, error) => {
                    if using_default_template_dir {
                        unreachable!(
                            "Failed to normalize default template directory. Original error: {error}",
                        );
                    } else {
                        let path_buf = path_buf.to_string_lossy();
                        let error = error.to_string();
                        ParsedEscaperError::ParseError(quote_spanned! {span=>
                            compile_error!(concat!("Failed to normalize `", #path_buf, "`. Original error: ", #error));
                        })
                    }
                },
                AppendPathError::PrefixNotPresent { prefix, final_path } => {
                    if using_default_template_dir {
                        let _ = prefix;
                        let _ = final_path;
                        unreachable!(
                            "`default_template_dir` variable in `oxiplate-derive` code must be a relative \
                            path; example: 'templates' instead of '/templates'. Provided: {specified_templates_dir}",
                        );
                    } else {
                        let prefix = prefix.to_string_lossy();
                        let final_path = final_path.to_string_lossy();
                        ParsedEscaperError::ParseError(quote_spanned! {span=>
                            compile_error!(concat!(
                                "`OXIP_TEMPLATE_DIR` environment variable must be a relative path that resolves under `",
                                #prefix,
                                "`; example: 'templates' instead of '/templates'. Provided: ",
                                #final_path
                            ));
                        })
                    }
                },
                AppendPathError::NotDirectory(path_buf) => {
                    let path_buf = path_buf.to_string_lossy();
                    ParsedEscaperError::ParseError(quote_spanned! {span=>
                        compile_error!(concat!("Template directory `", #path_buf, "` was not a directory."));
                    })
                },
                AppendPathError::NotFile(_path_buf) => unreachable!("Directory is expected, not a file"),
            }
        })
}

/// Build the template path.
fn template_path(path: &LitStr, attr_span: Span) -> Result<PathBuf, ParsedEscaperError> {
    let templates_dir = templates_dir(attr_span)?;

    // Path::join() doesn't play well with absolute paths (for our use case).
    let span = path.span();

    templates_dir
            .append_path(path.value(), true)
            .map_err(|err| -> ParsedEscaperError {
                match err {
                    AppendPathError::DoesNotExist(path_buf) => {
                        let path_buf = path_buf.to_string_lossy();
                        ParsedEscaperError::ParseError(quote_spanned! {span=>
                            compile_error!(concat!("Path does not exist: `", #path_buf, "`"));
                        })
                    },
                    AppendPathError::IsSymlink(path_buf) => {
                        let path_buf = path_buf.to_string_lossy();
                        ParsedEscaperError::ParseError(quote_spanned! {span=>
                            compile_error!(concat!("Symlinks are not allowed for template paths: `", #path_buf, "`"));
                        })
                    },
                    AppendPathError::CanonicalizeError(path_buf, error) => {
                        let path_buf = path_buf.to_string_lossy();
                        let error = error.to_string();
                        ParsedEscaperError::ParseError(quote_spanned! {span=>
                            compile_error!(concat!("Failed to canonicalize path: `", #path_buf, "`. Original error: ", #error));
                        })
                    },
                    AppendPathError::PrefixNotPresent { prefix, final_path } => {
                        let prefix = prefix.to_string_lossy();
                        let final_path = final_path.to_string_lossy();
                        ParsedEscaperError::ParseError(quote_spanned! {span=>
                            compile_error!(concat!("Template path `", #final_path, "` not within template directory `", #prefix, "`"));
                        })
                    },
                    AppendPathError::NotDirectory(_path_buf) => unreachable!("File is expected, not a directory"),
                    AppendPathError::NotFile(path_buf) => {
                        let path_buf = path_buf.to_string_lossy();
                        ParsedEscaperError::ParseError(quote_spanned! {span=>
                            compile_error!(concat!("Path is not a file: `", #path_buf, "`"));
                        })
                    },
                }
            })
}

fn parse_source_tokens_for_path(
    attr: &Attribute,
    #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))] state: &mut State,
) -> ParsedTokens {
    let syn::Meta::NameValue(MetaNameValue {
        path: _,
        eq_token: _,
        value: Expr::Lit(ExprLit {
            attrs: _,
            lit: Lit::Str(path),
        }),
    }) = &attr.meta
    else {
        let span = attr.span();
        return Err(ParsedEscaperError::ParseError(quote_spanned! {span=>
            compile_error!("Incorrect syntax for external template. Should look something like:\n#[oxiplate = \"/path/to/template/from/templates/directory.txt.oxip\"]");
        }));
    };

    let full_path = template_path(path, attr.span())?;

    let span = path.span();

    #[cfg(feature = "oxiplate")]
    let mut escaper_name: Option<String> = None;

    // Infer the escaper from the template's file extension.
    // Only works when using `oxiplate` rather than `oxiplate-derive` directly.
    #[cfg(feature = "oxiplate")]
    if *state.config.infer_escaper_group_from_file_extension {
        // Get the template's file extension,
        // but ignore `.oxip`.
        let path_value = full_path.to_string_lossy();
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
            if state.config.escaper_groups.contains_key(extension) {
                escaper_name = Some(extension.to_owned());
            } else {
                // `None` will normally be returned for the escaper,
                // but there's a match arm that is unreachable because of it.
                #[cfg(feature = "unreachable")]
                return Err(ParsedEscaperError::EscaperNotFound((
                    extension.to_string(),
                    span,
                )));
            }
        }
    }

    #[cfg(feature = "external-template-spans")]
    let tokens = {
        let path = syn::LitStr::new(&full_path.to_string_lossy(), span);
        quote::quote_spanned!(span=> include_str!(#path))
    };

    #[cfg(not(feature = "external-template-spans"))]
    let tokens = {
        let template_string = fs::read_to_string(&full_path)
            .expect("Template has already been checked to exist; perhaps a permissions issue?");

        quote_spanned! {span=> #template_string }
    };

    #[cfg(feature = "oxiplate")]
    return Ok((span, tokens, Some(full_path), escaper_name));

    #[cfg(not(feature = "oxiplate"))]
    Ok((span, tokens, Some(full_path), None))
}

/// Error when attempting to append one path onto another one.
enum AppendPathError {
    /// Path does not exist.
    DoesNotExist(PathBuf),
    /// Path is a symlink instead of a file or directory.
    IsSymlink(PathBuf),
    /// Canonicalizing the path failed.
    /// More information in the IO error.
    CanonicalizeError(PathBuf, io::Error),
    /// Final path is outside the directory being appended to.
    /// Absolute paths (`/templates`) or `..` directories can cause this.
    PrefixNotPresent {
        prefix: PathBuf,
        final_path: PathBuf,
    },
    /// Path is not a directory (probably a file).
    NotDirectory(PathBuf),
    /// Path is not a file (probably a directory).
    NotFile(PathBuf),
}

/// Trait to append one path onto another.
trait AppendPath<P: AsRef<Path>> {
    /// Append a path onto an existing path.
    /// Will fail if the new path is outside of the existing path.
    fn append_path(&self, suffix: P, expecting_file: bool) -> Result<Self, AppendPathError>
    where
        Self: Sized;
}
impl<P: AsRef<Path>> AppendPath<P> for PathBuf {
    fn append_path(&self, suffix: P, expecting_file: bool) -> Result<Self, AppendPathError> {
        // Append the suffix to the main path
        let new_path = self.join(suffix);

        // Do some checks before canonicalizing
        // in order to return better error messages.
        if !new_path.starts_with(self) {
            return Err(AppendPathError::PrefixNotPresent {
                prefix: self.clone(),
                final_path: new_path,
            });
        } else if !new_path.exists() {
            return Err(AppendPathError::DoesNotExist(new_path));
        } else if new_path.is_symlink() {
            return Err(AppendPathError::IsSymlink(new_path));
        }

        // Canonicalize to ensure prefix check later is against final path.
        let new_path = new_path
            .canonicalize()
            .map_err(|error| AppendPathError::CanonicalizeError(new_path, error))?;

        // Ensure path is within the original directory
        // and the new path is a file/directory.
        if !new_path.starts_with(self) {
            return Err(AppendPathError::PrefixNotPresent {
                prefix: self.clone(),
                final_path: new_path,
            });
        } else if !expecting_file && !new_path.is_dir() {
            return Err(AppendPathError::NotDirectory(new_path));
        } else if expecting_file && !new_path.is_file() {
            return Err(AppendPathError::NotFile(new_path));
        }

        Ok(new_path)
    }
}

macro_rules! internal_error {
    ($span:expr, $message:expr) => {{
        internal_error!($span, $message, );
    }};
    ($span:expr, $message:expr, $($help:tt)*) => {{
        let message = $message;

        #[cfg(not(feature = "better-internal-errors"))]
        unreachable!(
            "Internal Oxiplate error. Enable `better-internal-errors` feature for an \
             easier-to-debug error message. Error: {}",
            message
        );

        #[cfg(feature = "better-internal-errors")]
        {
            let url = format!(
                "https://github.com/0b10011/oxiplate/issues/new?title={}&labels=internal+error&body={}",
                crate::encode_query_value(&message),
                crate::encode_query_value(
                    r"Error:

```text
PASTE_FULL_ERROR_HERE
```

Template:

```
PASTE_TEMPLATE_HERE
```"
                ),
            );

            ::proc_macro::Diagnostic::spanned(
                $span,
                ::proc_macro::Level::Error,
                format!("Internal Oxiplate error: {}", &message),
            )
            .help(format!("Please open an issue: {url}"))
            $($help)*
            .emit();

            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }
    }};
}

/// Hacky URL query value encoder for internal errors based on:
/// <https://github.com/servo/rust-url/blob/b381851473a5a516f58f2cfc9db300abf7a4eaa7/form_urlencoded/src/lib.rs#L138-L163>
#[cfg(feature = "better-internal-errors")]
pub(crate) fn encode_query_value(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for byte in input.as_bytes() {
        match byte {
            b' ' => output.push('+'),
            b'*' | b'-' | b'.' | b'0'..=b'9' | b'A'..=b'Z' | b'_' | b'a'..=b'z' => output.push_str(
                ::std::str::from_utf8(&[*byte]).expect("Error messages should always be UTF8-safe"),
            ),
            _ => output.push_str(percent_encode_byte(*byte)),
        }
    }

    output
}

pub(crate) use internal_error;
