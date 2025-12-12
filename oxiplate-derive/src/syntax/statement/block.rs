use std::collections::{HashMap, VecDeque};

use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::cut;
use nom::error::context;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};
use syn::spanned::Spanned;

use super::super::expression::{Identifier, ident, keyword};
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::template::{Template, whitespace};
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
            unreachable!(
                "Should not attempt to add item to `block` statement after statement is ended."
            );
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
                    suffix.0.push(Item::CompileError {
                        message: "Multiple parent blocks present in block".to_string(),
                        error_source: source.clone(),
                        consumed_source: source,
                    });
                } else {
                    self.suffix = Some(Template(vec![]));
                }
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
        self.build_block((quote! {}, 0), (Some(quote! {}), 0), state, block_stack)
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
            Diagnostic::spanned(
                child_prefix.span().unwrap(),
                proc_macro::Level::Error,
                "Internal Oxiplate error: `build_block()` should not be called with an empty block stack.",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=%60build_block()%60+should+not+be+called+with+an+empty+block")
            .help("Include template that caused the issue.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
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

    let (input, (leading_whitespace, name)) = cut((
        context("Expected space after 'block'", whitespace),
        context("Expected an identifier", ident),
    ))
    .parse(input)?;

    let source = block_keyword
        .0
        .merge(&leading_whitespace, "Whitespace expected after `block`")
        .merge(&name.source, "Block name expected after whitespace");

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
            source,
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
