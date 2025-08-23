use std::collections::{HashMap, VecDeque};

use nom::Parser as _;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::cut;
use nom::error::context;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};

use super::super::expression::{Identifier, ident, keyword};
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::template::{Template, is_whitespace};
use crate::{Source, State};

#[derive(Debug)]
pub struct Block<'a> {
    pub(super) name: Identifier<'a>,
    pub(super) prefix: Template<'a>,
    pub(super) suffix: Option<Template<'a>>,
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
            Item::Statement(Statement {
                kind: StatementKind::Parent,
                source,
            }) => {
                if let Some(suffix) = &mut self.suffix {
                    suffix.0.push(Item::CompileError(
                        "Multiple parent blocks present in block".to_string(),
                        source,
                    ));
                }

                self.suffix = Some(Template(vec![]));
            }
            _ => {
                if let Some(template) = &mut self.suffix {
                    template.0.push(item);
                } else {
                    self.prefix.0.push(item);
                }
            }
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut block_stack = state.blocks.clone();
        let block = HashMap::from([(self.name.ident, (&self.prefix, self.suffix.as_ref()))]);
        block_stack.push_back(&block);
        self.build_block(
            // self.prefix.to_tokens(state),
            // self.suffix.as_ref().map_or((None, 0), |template| {
            //     let (template, estimated_length) = template.to_tokens(state);
            //     (Some(template), estimated_length)
            // }),
            (quote! {}, 0),
            (Some(quote! {}), 0),
            state,
            block_stack,
        )
    }

    fn build_block(
        &self,
        (child_prefix, child_prefix_length): (TokenStream, usize),
        (child_suffix, child_suffix_length): (Option<TokenStream>, usize),
        state: &State,
        mut block_stack: VecDeque<&HashMap<&str, (&Template, Option<&Template>)>>,
    ) -> (TokenStream, usize) {
        let mut estimated_length = child_prefix_length + child_suffix_length;
        let mut tokens = TokenStream::new();
        let Some(blocks) = block_stack.pop_front() else {
            return (quote! { #child_prefix #child_suffix }, estimated_length);
        };

        if let Some(&(prefix, suffix)) = blocks.get(self.name.ident) {
            let (prefix, prefix_length) = prefix.to_tokens(state);

            if let Some(child_suffix) = child_suffix {
                let (suffix, suffix_length) = suffix.as_ref().map_or((None, 0), |template| {
                    let (template, estimated_length) = template.to_tokens(state);
                    (Some(template), estimated_length)
                });

                if !block_stack.is_empty() {
                    return if suffix.is_some() {
                        self.build_block(
                            (
                                quote! { #child_prefix #prefix },
                                child_prefix_length + prefix_length,
                            ),
                            (
                                Some(quote! { #suffix #child_suffix }),
                                suffix_length + child_suffix_length,
                            ),
                            state,
                            block_stack,
                        )
                    } else {
                        self.build_block(
                            (
                                quote! { #child_prefix #prefix #child_suffix },
                                child_prefix_length + prefix_length + child_suffix_length,
                            ),
                            (None, 0),
                            state,
                            block_stack,
                        )
                    };
                }

                estimated_length += prefix_length + suffix_length;
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
            if !block_stack.is_empty() && child_suffix.is_some() {
                return self.build_block(
                    (child_prefix, child_prefix_length),
                    (child_suffix, child_suffix_length),
                    state,
                    block_stack,
                );
            }

            tokens.append_all(quote! {
                #child_prefix
                #child_suffix
            });
        }
        (tokens, estimated_length)
    }
}

impl<'a> From<Block<'a>> for StatementKind<'a> {
    fn from(statement: Block<'a>) -> Self {
        StatementKind::Block(statement)
    }
}

pub(super) fn parse_block(input: Source) -> Res<Source, Statement> {
    let (input, block_keyword) = keyword("block")(input)?;

    let (input, (_, name)) = cut((
        context("Expected space after 'block'", take_while1(is_whitespace)),
        context("Expected an identifier", ident),
    ))
    .parse(input)?;

    Ok((
        input,
        Statement {
            kind: Block {
                name,
                prefix: Template(vec![]),
                suffix: None,
                is_ended: false,
            }
            .into(),
            source: block_keyword.0,
        },
    ))
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
