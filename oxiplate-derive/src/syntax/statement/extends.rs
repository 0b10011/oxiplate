use std::collections::HashMap;
use std::fmt;

use nom::bytes::complete::{escaped, is_not, tag, take_while1};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, TokenStreamExt};
use syn::{GenericArgument, Ident};

use super::super::expression::keyword;
use super::super::Res;
use super::{Statement, StatementKind, StaticType};
use crate::syntax::template::{is_whitespace, Template};
use crate::syntax::Item;
use crate::Source;

pub struct Extends<'a> {
    is_extending: bool,
    blocks: HashMap<&'a str, ((TokenStream, Option<TokenStream>), usize)>,
    path: Source<'a>,
    template: Template<'a>,
}

impl fmt::Debug for Extends<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extends")
            // .field("data_type", &"UNSUPPORTED_SORRY")
            .field("blocks", &self.blocks)
            .field("path", &self.path)
            .field("template", &self.template)
            .finish()
    }
}

impl<'a> Extends<'a> {
    /// Get the estimated length of all blocks.
    pub(crate) fn estimated_length(&self) -> usize {
        let mut estimated_length = 0;
        for item in self.blocks.values() {
            estimated_length += item.1;
        }
        estimated_length
    }

    pub(crate) fn add_item(&mut self, mut item: Item<'a>) {
        #[allow(clippy::match_same_arms)]
        match &mut item {
            // Comments are fine to keep
            Item::Comment => self.template.0.push(item),

            // Compile errors must be kept
            Item::CompileError(_, _) => self.template.0.push(item),

            // Whitespace should be ignored
            Item::Whitespace(_) => (),

            // Block statements are allowed, but other statements should fail
            Item::Statement(Statement {
                kind: StatementKind::Block(block),
                ..
            }) => {
                let prefix = {
                    let prefix = &block.prefix;
                    quote! { #prefix }
                };

                let (prefix, suffix) = match (&block.child.0, &block.suffix) {
                    (None, None) => (quote! { #prefix }, None),
                    (None, Some(suffix)) => (quote! { #prefix }, Some(quote! { #suffix })),
                    (Some((child_prefix, None)), _) => (quote! { #child_prefix }, None),
                    (Some((child_prefix, Some(child_suffix))), None) => {
                        (quote! { #child_prefix #prefix #child_suffix }, None)
                    }
                    (Some((child_prefix, Some(child_suffix))), Some(suffix)) => (
                        quote! { #child_prefix #prefix },
                        Some(quote! { #suffix #child_suffix }),
                    ),
                };

                let estimated_length = block.child.1
                    + block.prefix.estimated_length()
                    + if let Some(suffix) = &block.suffix {
                        suffix.estimated_length()
                    } else {
                        0
                    };

                self.blocks
                    .insert(block.name.ident, ((prefix, suffix), estimated_length));
            }
            Item::Statement(statement) => self.template.0.push(Item::CompileError(
                "Only block statements are allowed here, along with comments and whitespace."
                    .to_owned(),
                statement.source.clone(),
            )),

            // No static text or writs allowed
            Item::Static(_, static_type) => {
                if static_type != &StaticType::Whitespace {
                    unimplemented!(
                        "Text is not allowed here. Only comments, whitespace, and blocks are \
                         allowed."
                    )
                }
            }
            Item::Writ(_) => unimplemented!(
                "Writs are not allowed here. Only comments, whitespace, and blocks are allowed."
            ),
        }
    }

    pub(crate) fn build_template(&self) -> (TokenStream, usize) {
        let span = self.path.span();
        let path = self.path.as_str();

        // FIXME: Should also include local vars here I think
        let mut block_generics = vec![];
        let mut block_constraints = vec![];
        let mut block_definitions = vec![];
        let mut block_values = vec![];
        let mut generic_name_i = 0;
        for item in &self.template.0 {
            if let Item::Statement(Statement {
                kind: StatementKind::Block(block),
                ..
            }) = item
            {
                // Build generic name and bounds
                generic_name_i += 1;
                let generic_name = Ident::new(&format!("Block{generic_name_i}"), span);
                let constraint: GenericArgument = syn::parse_quote_spanned!(span=>
                    #generic_name: Fn(fn(f: &mut dyn Write) -> ::std::fmt::Result, &mut dyn Write) -> ::std::fmt::Result
                );
                let block_name = &block.name;

                block_definitions.push(quote_spanned! {span=>
                    #block_name: &'a #generic_name
                });
                block_generics.push(generic_name);
                block_constraints.push(constraint);

                if self.blocks.contains_key(block.name.ident) {
                    block_values.push(quote_spanned! {span=>
                        #block_name: &self.#block_name
                    });
                } else {
                    block_values.push(quote_spanned! {span=>
                        #block_name: &#block_name
                    });
                }
            }
        }

        #[cfg(feature = "oxiplate")]
        let oxiplate = quote_spanned! {span=> ::oxiplate::Oxiplate };
        #[cfg(not(feature = "oxiplate"))]
        let oxiplate = quote_spanned! {span=> ::oxiplate_derive::Oxiplate };

        let template = &self.template;
        let mut tokens: TokenStream = quote! { #template };

        let template_to_extend = if self.is_extending {
            quote_spanned! {span=>
                #[derive(#oxiplate)]
                #[oxiplate_extends = #path]
                struct ExtendingTemplate<'a, #(#block_generics),*>
                where #(#block_constraints),*
                {
                    #(#block_definitions,)*
                }
            }
        } else {
            quote_spanned! {span=>
                #[derive(#oxiplate)]
                #[oxiplate_extends = #path]
                struct Template<'a, #(#block_generics),*>
                where #(#block_constraints),*
                {
                    #(#block_definitions,)*
                }
            }
        };
        let (template, estimated_length) =
            crate::oxiplate_internal(template_to_extend.into(), &self.blocks);
        let template: TokenStream = template.into();

        tokens.append_all(quote! {
            #template
        });
        (tokens, estimated_length)
    }
}

impl<'a> From<Extends<'a>> for StatementKind<'a> {
    fn from(statement: Extends<'a>) -> Self {
        StatementKind::Extends(statement)
    }
}

pub(super) fn parse_extends(input: Source) -> Res<Source, Statement> {
    let (input, extends_keyword) = keyword("extends").parse(input)?;

    let (input, (_, _, path, _)) = cut((
        context("Expected space after 'extends'", take_while1(is_whitespace)),
        context(r#"Expected ""#, tag(r#"""#)),
        context(
            "Expected path to the template to extend",
            escaped(is_not(r#"""#), '\\', one_of(r#"""#)),
        ),
        context(r#"Expected ""#, tag(r#"""#)),
    ))
    .parse(input)?;

    let is_extending = input.original.is_extending;

    let source = extends_keyword.0;

    Ok((
        input,
        Statement {
            kind: Extends {
                is_extending,
                blocks: HashMap::new(),
                path: path.clone(),
                template: Template(vec![]),
            }
            .into(),
            source,
        },
    ))
}
