use std::collections::HashMap;
use std::fmt;

use nom::Parser as _;
use nom::bytes::complete::{escaped, is_not, tag, take_while1};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};
use syn::{GenericArgument, Ident, LitStr};

use super::super::Res;
use super::super::expression::keyword;
use super::{Statement, StatementKind, StaticType};
use crate::syntax::Item;
use crate::syntax::template::{Template, is_whitespace};
use crate::{Source, State};

pub struct Extends<'a> {
    is_extending: bool,
    blocks: HashMap<&'a str, (Template<'a>, Option<Template<'a>>)>,
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
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        #[allow(clippy::match_same_arms)]
        match item {
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
                self.blocks
                    .insert(block.name.ident, (block.prefix, block.suffix));
            }
            Item::Statement(statement) => self.template.0.push(Item::CompileError(
                "Only block statements are allowed here, along with comments and whitespace."
                    .to_owned(),
                statement.source.clone(),
            )),

            // No static text or writs allowed
            Item::Static(_, static_type) => {
                if static_type != StaticType::Whitespace {
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

    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let span = self.path.span();
        let path = LitStr::new(self.path.as_str(), span);

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

        let (template, _template_length) = &self.template.to_tokens(state);
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

        let mut block_stack = state.blocks.clone();
        let mut blocks = HashMap::new();
        for (name, block) in &self.blocks {
            blocks.insert(*name, (&block.0, block.1.as_ref()));
        }
        block_stack.push_back(&blocks);

        let (template, estimated_length) =
            crate::oxiplate_internal(template_to_extend.into(), &block_stack);
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
                path,
                template: Template(vec![]),
            }
            .into(),
            source,
        },
    ))
}
