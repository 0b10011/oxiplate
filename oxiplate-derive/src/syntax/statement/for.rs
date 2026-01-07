use std::collections::HashSet;

use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, into};
use nom::error::context;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};

use super::super::expression::{Keyword, expression, keyword};
use super::super::{Item, Res};
use super::{State, Statement, StatementKind};
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::statement::helpers::pattern::Pattern;
use crate::syntax::template::{Template, whitespace};
use crate::{Source, Tokens};

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
            Diagnostic::spanned(
                item.source().span().unwrap(),
                Level::Error,
                "Internal Oxiplate error: Attempted to add item to ended `for` statement.",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Attempted+to+add+item+to+ended+for+statement")
            .help("Include template that caused the issue and the associated note.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
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

    pub fn to_tokens<'b: 'a>(&self, state: &mut State<'b>) -> Tokens {
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

pub(super) fn parse_for(input: Source) -> Res<Source, Statement> {
    let (input, for_keyword) = keyword("for").parse(input)?;

    let (input, (for_whitespace, pattern, ident_whitespace, in_keyword, in_whitespace, expression)) =
        cut((
            context("Expected space after 'for'", whitespace),
            context("Expected a pattern", Pattern::parse),
            context("Expected space after identifier", whitespace),
            context("Expected 'in'", keyword("in")),
            context("Expected space after 'in'", whitespace),
            context(
                "Expected an expression that is iterable",
                expression(true, true),
            ),
        ))
        .parse(input)?;

    let source = for_keyword
        .0
        .clone()
        .merge(&for_whitespace, "Whitespace expected after `for`")
        .merge(pattern.source(), "Ident expected after whitespace")
        .merge(&ident_whitespace, "Whitespace expected after ident")
        .merge(&in_keyword.0, "`in` expected after whitespace")
        .merge(&in_whitespace, "Whitespace expected after `in`")
        .merge(&expression.source(), "Expression expected after whitespace");

    Ok((
        input,
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

pub(super) fn parse_endfor(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endfor").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndFor,
            source: output,
        },
    ))
}

#[derive(Debug)]
pub(crate) struct Break<'a>(Keyword<'a>);

impl<'a> Break<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        into(keyword("break")).parse(input)
    }

    pub fn source(&self) -> &Source<'a> {
        &self.0.0
    }

    pub fn to_tokens(&self) -> Tokens {
        let span = self.0.0.span();
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
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        into(keyword("continue")).parse(input)
    }

    pub fn source(&self) -> &Source<'a> {
        &self.0.0
    }

    pub fn to_tokens(&self) -> Tokens {
        let span = self.0.0.span();
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
