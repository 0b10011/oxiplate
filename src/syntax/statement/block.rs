use super::super::expression::{ident, keyword, Identifier};
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::template::is_whitespace;
use crate::Source;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub struct Block<'a> {
    pub(super) name: Identifier<'a>,
    pub(super) use_override: bool,
    pub(super) should_output: bool,
    items: Vec<Item<'a>>,
    pub(super) is_ended: bool,
}

impl<'a> Block<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            todo!();
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::EndBlock,
                ..
            }) => {
                self.is_ended = true;
            }
            _ => {
                self.items.push(item);
            }
        }
    }
}

impl<'a> From<Block<'a>> for StatementKind<'a> {
    fn from(statement: Block<'a>) -> Self {
        StatementKind::Block(statement)
    }
}

impl ToTokens for Block<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Block { name, items, .. } = self;
        if self.should_output {
            if self.use_override {
                tokens.append_all(quote! {
                    (self.#name)(f)?;
                });
            } else {
                tokens.append_all(quote! {
                    #(#items)*
                });
            }
        } else if self.use_override {
            tokens.append_all(quote! {
                let #name = self.#name;
            });
        } else {
            tokens.append_all(quote! {
                let #name = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {
                    #(#items)*
                    Ok(())
                };
            });
        }
    }
}

pub(super) fn parse_block(
    should_output_blocks: &bool,
) -> impl FnMut(Source) -> Res<Source, Statement> + '_ {
    |input| {
        let (input, block_keyword) = keyword("block")(input)?;

        let (input, (_, name)) = cut(tuple((
            context("Expected space after 'block'", take_while1(is_whitespace)),
            context("Expected an identifier", ident),
        )))(input)?;

        let source = block_keyword.0.clone();
        let use_override = input.original.blocks.contains(&name.0.to_string());

        Ok((
            input,
            Statement {
                kind: Block {
                    name,
                    use_override,
                    should_output: *should_output_blocks,
                    items: vec![],
                    is_ended: false,
                }
                .into(),
                source,
            },
        ))
    }
}

pub(super) fn parse_endblock(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endblock")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndBlock,
            source: output,
        },
    ))
}
