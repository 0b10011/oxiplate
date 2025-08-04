use nom::bytes::complete::{tag, take_while1};
use nom::combinator::cut;
use nom::error::context;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use super::super::expression::{ident, keyword, Identifier};
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::template::{is_whitespace, Template};
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
enum Position {
    /// The highest level of block that is automatically applied.
    /// Will be used if not overridden.
    Source,

    /// Overridden content.
    /// The parent's content will be placed where the `{% parent %}` tag appears,
    /// otherwise the source content will be completely replaced.
    Override,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub(super) name: Identifier<'a>,
    position: Position,

    /// Token streams for child blocks.
    /// Makes it possible to inline the child blocks
    /// directly into the parent template's block.
    pub(super) child: Option<(TokenStream, Option<TokenStream>)>,
    pub(super) prefix: Template<'a>,
    pub(super) suffix: Option<Template<'a>>,
    pub(super) is_ended: bool,
}

impl<'a> Block<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            todo!();
        }

        match self.position {
            Position::Source => match item {
                Item::Statement(Statement {
                    kind: StatementKind::EndBlock,
                    ..
                }) => {
                    self.is_ended = true;
                }
                _ => {
                    self.prefix.0.push(item);
                }
            },
            Position::Override => match item {
                Item::Statement(Statement {
                    kind: StatementKind::EndBlock,
                    ..
                }) => {
                    self.is_ended = true;
                }
                Item::Statement(Statement {
                    kind: StatementKind::Parent,
                    ..
                }) => {
                    self.suffix = Some(Template(vec![]));
                }
                _ => {
                    if let Some(template) = &mut self.suffix {
                        template.0.push(item);
                    } else {
                        self.prefix.0.push(item);
                    }
                }
            },
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
        let Block { prefix, suffix, .. } = self;
        if let Some((child_prefix, child_suffix)) = &self.child {
            if let Some(child_suffix) = child_suffix {
                tokens.append_all(quote! {
                    #child_prefix
                    #prefix
                    #suffix
                    #child_suffix
                });
            } else {
                tokens.append_all(quote! {
                    #child_prefix
                });
            }
        } else {
            tokens.append_all(quote! {
                #prefix
            });
        }
    }
}

pub(super) fn parse_block<'a>(
    state: &'a State,
    is_extending: &'a bool,
) -> impl FnMut(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, block_keyword) = keyword("block")(input)?;

        let position = if *is_extending {
            Position::Override
        } else {
            Position::Source
        };

        let (input, (_, name)) = cut((
            context("Expected space after 'block'", take_while1(is_whitespace)),
            context("Expected an identifier", ident),
        ))
        .parse(input)?;

        let source = block_keyword.0.clone();
        let child_block = state.blocks.get(name.ident).cloned();

        Ok((
            input,
            Statement {
                kind: Block {
                    name,
                    position,
                    child: child_block,
                    prefix: Template(vec![]),
                    suffix: None,
                    is_ended: false,
                }
                .into(),
                source,
            },
        ))
    }
}

pub(super) fn parse_parent(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("parent").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::Parent,
            source: output,
        },
    ))
}

pub(super) fn parse_endblock(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endblock").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndBlock,
            source: output,
        },
    ))
}
