use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};

use super::super::Item;
use super::super::expression::{Keyword, expression};
use super::{State, Statement, StatementKind};
use crate::parser::{Parser as _, cut, into};
use crate::template::parser::Res;
use crate::template::parser::expression::{ExpressionAccess, KeywordParser};
use crate::template::parser::statement::helpers::pattern::Pattern;
use crate::template::parser::template::Template;
use crate::template::tokenizer::TokenSlice;
use crate::{BuiltTokens, Source, internal_error};

#[derive(Debug)]
pub struct For<'a> {
    #[allow(clippy::struct_field_names)]
    for_keyword: Keyword<'a>,
    pattern: Pattern<'a>,
    in_keyword: Keyword<'a>,
    expression: ExpressionAccess<'a>,
    template: Template<'a>,
    otherwise: Option<Template<'a>>,
    pub(super) is_ended: bool,
}

impl<'a> For<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            internal_error!(
                item.source().span_token().unwrap(),
                "Attempted to add item to ended `for` statement",
            );
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::Else,
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError {
                        message: "`else` previously present in this `for` statement; expected \
                                  `endfor`"
                            .to_string(),
                        error_source: source.clone(),
                        consumed_source: source,
                    });
                } else {
                    self.otherwise = Some(Template(vec![]));
                }
            }
            Item::Statement(Statement {
                kind: StatementKind::EndFor,
                ..
            }) => {
                self.is_ended = true;
            }
            Item::Statement(Statement { kind, source }) if !kind.expected_in_statements() => {
                if let Some(ref mut template) = self.otherwise {
                    template
                } else {
                    &mut self.template
                }
                .0
                .push(Item::CompileError {
                    message: "Unexpected statement in `for` statement; is an `endfor` statement \
                              missing?"
                        .to_string(),
                    error_source: source.clone(),
                    consumed_source: source,
                });
            }
            _ => {
                if let Some(otherwise) = &mut self.otherwise {
                    otherwise.0.push(item);
                } else {
                    self.template.0.push(item);
                }
            }
        }
    }

    pub(crate) fn get_active_variables(&'a self) -> HashSet<&'a str> {
        self.pattern.get_variables()
    }

    pub fn to_tokens<'b: 'a>(&self, state: &mut State<'b>) -> BuiltTokens {
        let mut tokens = TokenStream::new();
        let mut estimated_length = 0;

        let For {
            for_keyword,
            pattern,
            in_keyword,
            expression,
            template,
            otherwise,
            is_ended: _,
        } = self;

        let (expression, _expression_length) = expression.to_tokens(state);

        state.local_variables.push_stack();

        state.local_variables.add(
            self.get_active_variables()
                .iter()
                .map(ToString::to_string)
                .collect(),
        );
        let (template, template_length) = template.to_tokens(state);

        // Loops will very likely run at least twice.
        estimated_length += template_length * 2;

        let pattern = pattern.to_tokens(state);
        if let Some(otherwise) = otherwise {
            let (otherwise, otherwise_length) = otherwise.to_tokens(state);
            estimated_length = estimated_length.min(otherwise_length);
            tokens.append_all(quote! {
                {
                    let mut loop_ran = false;
                    #for_keyword #pattern #in_keyword #expression {
                        loop_ran = true;
                        #template
                    }
                    if !loop_ran {
                        #otherwise
                    }
                }
            });
        } else {
            tokens
                .append_all(quote! { #for_keyword #pattern #in_keyword #expression { #template } });
        }

        state.local_variables.pop_stack();

        (tokens, estimated_length)
    }
}

impl<'a> From<For<'a>> for StatementKind<'a> {
    fn from(statement: For<'a>) -> Self {
        StatementKind::For(statement)
    }
}

pub(super) fn parse_for(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, for_keyword) = KeywordParser::new("for").parse(tokens)?;

    let (tokens, (pattern, in_keyword, expression)) = (
        cut("Expected a pattern", Pattern::parse),
        cut("Expected 'in'", KeywordParser::new("in")),
        cut(
            "Expected an expression that is iterable",
            expression(true, true),
        ),
    )
        .parse(tokens)?;

    let source = for_keyword
        .source()
        .clone()
        .merge(pattern.source(), "Ident expected after whitespace")
        .merge(in_keyword.source(), "`in` expected after whitespace")
        .merge(&expression.source(), "Expression expected after whitespace");

    Ok((
        tokens,
        Statement {
            kind: For {
                for_keyword,
                pattern,
                in_keyword,
                expression,
                template: Template(vec![]),
                otherwise: None,
                is_ended: false,
            }
            .into(),
            source,
        },
    ))
}

pub(super) fn parse_endfor(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, output) = KeywordParser::new("endfor").parse(tokens)?;

    Ok((
        tokens,
        Statement {
            kind: StatementKind::EndFor,
            source: output.source().clone(),
        },
    ))
}

#[derive(Debug)]
pub(crate) struct Break<'a>(Keyword<'a>);

impl<'a> Break<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Break<'a>> {
        into(KeywordParser::new("break")).parse(tokens)
    }

    pub fn source(&self) -> &Source<'a> {
        self.0.source()
    }

    pub fn to_tokens(&self) -> BuiltTokens {
        let span = self.0.source().span_token();
        let keyword = &self.0;

        (quote_spanned! {span=> #keyword; }, 0)
    }
}

impl<'a> From<Keyword<'a>> for Break<'a> {
    fn from(value: Keyword<'a>) -> Self {
        Break(value)
    }
}

impl<'a> From<Break<'a>> for Statement<'a> {
    fn from(value: Break<'a>) -> Self {
        Statement {
            source: value.source().clone(),
            kind: StatementKind::Break(value),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Continue<'a>(Keyword<'a>);

impl<'a> Continue<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Continue<'a>> {
        into(KeywordParser::new("continue")).parse(tokens)
    }

    pub fn source(&self) -> &Source<'a> {
        self.0.source()
    }

    pub fn to_tokens(&self) -> BuiltTokens {
        let span = self.0.source().span_token();
        let keyword = &self.0;

        (quote_spanned! {span=> #keyword; }, 0)
    }
}

impl<'a> From<Keyword<'a>> for Continue<'a> {
    fn from(value: Keyword<'a>) -> Self {
        Continue(value)
    }
}

impl<'a> From<Continue<'a>> for Statement<'a> {
    fn from(value: Continue<'a>) -> Self {
        Statement {
            source: value.source().clone(),
            kind: StatementKind::Continue(value),
        }
    }
}
