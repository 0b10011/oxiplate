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
use crate::Source;

#[derive(Debug, PartialEq, Eq)]
enum Position {
    /// The highest level of block that is automatically applied.
    /// Will be used if not overridden.
    /// Cannot be selected by a template with `{% block(source) ... %}`.
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
    pub(super) use_override: bool,
    prefix: Template<'a>,
    suffix: Option<Template<'a>>,
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
        let Block {
            name,
            prefix,
            suffix,
            ..
        } = self;
        if self.position == Position::Source {
            if self.use_override {
                // FIXME: It'd be better if the position information was passed around in the macro instead.
                tokens.append_all(quote! {
                    {
                        use ::std::fmt::Write;
                        let #name = |f: &mut dyn Write| -> ::std::fmt::Result {
                            #prefix
                            Ok(())
                        };
                        (self.#name)(#name, f)?;
                    }
                });
            } else {
                tokens.append_all(quote! {
                    #prefix
                });
            }
        } else if self.use_override {
            tokens.append_all(quote! {
                let #name = self.#name;
            });
        } else {
            let output = if suffix.is_none() {
                quote! {
                    #prefix
                }
            } else {
                quote! {
                    #prefix
                    callback(f)?;
                    #suffix
                }
            };

            tokens.append_all(quote! {
                let #name = {
                    use ::std::fmt::Write;
                    |
                        callback: fn(f: &mut dyn Write) -> ::std::fmt::Result,
                        f: &mut dyn Write
                    | -> ::std::fmt::Result {
                        #output
                        Ok(())
                    }
                };
            });
        }
    }
}

pub(super) fn parse_block(
    is_extending: &bool,
) -> impl FnMut(Source) -> Res<Source, Statement> + '_ {
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
        let use_override = input.original.blocks.contains(&name.ident.to_string());

        Ok((
            input,
            Statement {
                kind: Block {
                    name,
                    position,
                    use_override,
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
