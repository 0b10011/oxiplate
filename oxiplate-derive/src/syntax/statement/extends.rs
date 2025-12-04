use std::collections::HashMap;

use nom::Parser as _;
use nom::bytes::complete::{escaped, is_not, tag, take_while1};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};
use syn::LitStr;

use super::super::Res;
use super::super::expression::keyword;
use super::{Statement, StatementKind, StaticType};
use crate::syntax::Item;
use crate::syntax::template::{Template, is_whitespace};
use crate::{Source, State};

#[derive(Debug)]
pub struct Extends<'a> {
    blocks: HashMap<&'a str, (Template<'a>, Option<Template<'a>>)>,
    path: Source<'a>,
    template: Template<'a>,
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
            Item::Static(text, static_type) => {
                if static_type != StaticType::Whitespace {
                    self.template.0.push(Item::CompileError(
                        "Text is not allowed here. Only comments, whitespace, and block \
                         statements are allowed."
                            .to_owned(),
                        text.1.clone(),
                    ));
                }
            }
            Item::Writ(writ) => {
                self.template.0.push(Item::CompileError(
                    "Writs are not allowed here. Only comments, whitespace, and block statements \
                     are allowed."
                        .to_owned(),
                    writ.source().clone(),
                ));
            }
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let span = self.path.span();
        let path = LitStr::new(self.path.as_str(), span);

        #[cfg(feature = "oxiplate")]
        let oxiplate = quote_spanned! {span=> ::oxiplate::Oxiplate };
        #[cfg(not(feature = "oxiplate"))]
        let oxiplate = quote_spanned! {span=> ::oxiplate_derive::Oxiplate };

        let (template, _template_length) = &self.template.to_tokens(state);
        let mut tokens: TokenStream = quote! { #template };

        let template_to_extend = quote_spanned! {span=>
            #[derive(#oxiplate)]
            #[oxiplate_extends = #path]
            struct Template {}
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

    let source = extends_keyword.0;

    Ok((
        input,
        Statement {
            kind: Extends {
                blocks: HashMap::new(),
                path,
                template: Template(vec![]),
            }
            .into(),
            source,
        },
    ))
}
