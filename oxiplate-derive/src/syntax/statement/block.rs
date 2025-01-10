use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::sequence::{delimited, tuple};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use super::super::expression::{ident, keyword, Identifier};
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::template::{is_whitespace, Template};
use crate::Source;

#[derive(Debug, Default, PartialEq, Eq)]
enum Position {
    /// The highest level of block that is automatically applied.
    /// Cannot be selected by a template with `{% block(source) ... %}`.
    Source,

    /// New content appears before the parent's content.
    Prefix,

    /// New content replaces the parent's content.
    #[default]
    Replace,

    /// New content appears after the parent's content.
    Suffix,

    /// New content appears before and after the parent's content.
    /// The boundary is specified with `{% parent %}`
    Surround,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub(super) name: Identifier<'a>,
    position: Position,
    pub(super) use_override: bool,
    template: Template<'a>,
    surround_suffix: Option<Template<'a>>,
    pub(super) is_ended: bool,
}

impl<'a> Block<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            todo!();
        }

        match self.position {
            Position::Source | Position::Prefix | Position::Replace | Position::Suffix => {
                match item {
                    Item::Statement(Statement {
                        kind: StatementKind::EndBlock,
                        ..
                    }) => {
                        self.is_ended = true;
                    }
                    _ => {
                        self.template.0.push(item);
                    }
                }
            }
            Position::Surround => match item {
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
                    self.surround_suffix = Some(Template(vec![]));
                }
                _ => {
                    if let Some(template) = &mut self.surround_suffix {
                        template.0.push(item);
                    } else {
                        self.template.0.push(item);
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
            template,
            surround_suffix,
            ..
        } = self;
        if self.position == Position::Source {
            if self.use_override {
                // FIXME: It'd be better if the position information was passed around in the macro instead.
                tokens.append_all(quote! {
                    let #name = |f: &mut ::std::fmt::Formatter<'_>| -> ::std::fmt::Result {
                        #template
                        Ok(())
                    };
                    (self.#name)(#name, f)?;
                });
            } else {
                tokens.append_all(quote! {
                    #template
                });
            }
        } else if self.use_override {
            tokens.append_all(quote! {
                let #name = self.#name;
            });
        } else {
            let output = match self.position {
                Position::Source => unreachable!("Source should have already been handled"),
                Position::Prefix => quote! {
                    #template
                    callback(f)?;
                },
                Position::Replace => quote! {
                    #template
                },
                Position::Suffix => quote! {
                    callback(f)?;
                    #template
                },
                Position::Surround => quote! {
                    #template
                    callback(f)?;
                    #surround_suffix
                },
            };

            tokens.append_all(quote! {
                let #name = |
                    callback: fn(f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result,
                    f: &mut ::std::fmt::Formatter<'_>
                | -> ::std::fmt::Result {
                    #output
                    Ok(())
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

        let (input, position) = if *is_extending {
            let (input, position) = opt(delimited(
                char('('),
                alt((
                    tag("prefix"),
                    tag("replace"),
                    tag("suffix"),
                    tag("surround"),
                )),
                char(')'),
            ))(input)?;
            let position = position
                .map(|position| -> Position {
                    match position.as_str() {
                        "prefix" => Position::Prefix,
                        "replace" => Position::Replace,
                        "suffix" => Position::Suffix,
                        "surround" => Position::Surround,
                        _ => unreachable!("All parsed cases should have been covered"),
                    }
                })
                .unwrap_or_default();
            (input, position)
        } else {
            (input, Position::Source)
        };

        let (input, (_, name)) = cut(tuple((
            context("Expected space after 'block'", take_while1(is_whitespace)),
            context("Expected an identifier", ident),
        )))(input)?;

        let source = block_keyword.0.clone();
        let use_override = input.original.blocks.contains(&name.ident.to_string());

        Ok((
            input,
            Statement {
                kind: Block {
                    name,
                    position,
                    use_override,
                    template: Template(vec![]),
                    surround_suffix: None,
                    is_ended: false,
                }
                .into(),
                source,
            },
        ))
    }
}

pub(super) fn parse_parent(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("parent")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::Parent,
            source: output,
        },
    ))
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
