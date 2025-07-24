use std::fmt;

use nom::bytes::complete::{escaped, is_not, tag, take_while1};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{GenericArgument, Ident, Type};

use super::super::expression::keyword;
use super::super::Res;
use super::{Statement, StatementKind, StaticType};
use crate::syntax::template::{is_whitespace, Template};
use crate::syntax::Item;
use crate::Source;

pub struct Extends<'a> {
    is_extending: bool,
    data_type: Type,
    blocks: Vec<String>,
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
                kind: StatementKind::Block(_),
                ..
            }) => self.template.0.push(item),
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
}

impl<'a> From<Extends<'a>> for StatementKind<'a> {
    fn from(statement: Extends<'a>) -> Self {
        StatementKind::Extends(statement)
    }
}

impl ToTokens for Extends<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Extends { path, template, .. } = self;

        let span = path.span();
        let path = path.as_str();

        let data_type = &self.data_type;
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
                    #generic_name: Fn(fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result, &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
                );
                let block_name = &block.name;

                block_definitions.push(quote_spanned! {span=>
                    #block_name: &'a #generic_name
                });
                block_generics.push(generic_name);
                block_constraints.push(constraint);

                if self.blocks.contains(&block.name.ident.to_string()) {
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

        if self.is_extending {
            tokens.append_all(quote_spanned! {span=>
                #template
                #[derive(#oxiplate)]
                #[oxiplate_extends = #path]
                struct ExtendingTemplate<'a, #(#block_generics),*>
                where #(#block_constraints),*
                {
                    #[allow(dead_code)]
                    oxiplate_extends_data: &'a #data_type,
                    #(#block_definitions,)*
                }

                let template = ExtendingTemplate {
                    oxiplate_extends_data: &self.oxiplate_extends_data,
                    #(#block_values,)*
                };
            });
        } else {
            tokens.append_all(quote_spanned! {span=>
                #template
                #[derive(#oxiplate)]
                #[oxiplate_extends = #path]
                struct Template<'a, #(#block_generics),*>
                where #(#block_constraints),*
                {
                    #[allow(dead_code)]
                    oxiplate_extends_data: &'a #data_type,
                    #(#block_definitions,)*
                }

                let template = Template {
                    oxiplate_extends_data: self,
                    #(#block_values,)*
                };
            });
        }
        tokens.append_all(quote! {
            write!(f, "{}", template)?;
        });
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
    let data_type = input.original.data_type.clone();
    let blocks = input.original.blocks.clone();

    let source = extends_keyword.0;

    Ok((
        input,
        Statement {
            kind: Extends {
                is_extending,
                data_type,
                blocks,
                path: path.clone(),
                template: Template(vec![]),
            }
            .into(),
            source,
        },
    ))
}
