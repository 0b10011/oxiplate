use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};

use super::super::expression::expression;
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::statement::helpers::pattern::{Type, parse_type};
use crate::syntax::template::{Template, whitespace};
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IfType<'a> {
    If(ExpressionAccess<'a>),
    IfLet(Type<'a>, ExpressionAccess<'a>),
}

#[derive(Debug)]
pub(crate) struct If<'a> {
    pub ifs: Vec<(IfType<'a>, Template<'a>)>,
    pub otherwise: Option<Template<'a>>,
    pub is_ended: bool,
}

impl<'a> If<'a> {
    pub fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            Diagnostic::spanned(
                item.source().span().unwrap(),
                Level::Error,
                "Internal Oxiplate error: Attempted to add item to ended `if` statement.",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Attempted+to+add+item+to+ended+if+statement")
            .help("Include template that caused the issue and the associated note.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::ElseIf(ElseIf(if_type)),
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError {
                        message: "`else` previously present in this if statement; expected `endif`"
                            .to_string(),
                        error_source: source.clone(),
                        consumed_source: source,
                    });
                } else {
                    self.ifs.push((if_type, Template(vec![])));
                }
            }
            Item::Statement(Statement {
                kind: StatementKind::Else,
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError {
                        message: "`else` already present in this if statement; expected `endif`"
                            .to_string(),
                        error_source: source.clone(),
                        consumed_source: source,
                    });
                } else {
                    self.otherwise = Some(Template(vec![]));
                }
            }
            Item::Statement(Statement {
                kind: StatementKind::EndIf,
                source: _,
            }) => {
                self.is_ended = true;
            }
            _ => {
                if let Some(template) = &mut self.otherwise {
                    template.0.push(item);
                } else {
                    self.ifs.last_mut().unwrap().1.0.push(item);
                }
            }
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let mut estimated_length = usize::MAX;

        let mut is_elseif = false;
        for (expression, template) in &self.ifs {
            match expression {
                IfType::If(expression) => {
                    let (expression, _expression_length) = expression.to_tokens(state);
                    let (template, template_length) = template.to_tokens(state);
                    estimated_length = estimated_length.min(template_length);
                    if is_elseif {
                        tokens.append_all(quote! { else if #expression { #template } });
                    } else {
                        tokens.append_all(quote! { if #expression { #template } });
                    }
                }
                IfType::IfLet(ty, expression) => {
                    let (expression, _expression_length) = expression.to_tokens(state);

                    let mut local_variables = ty.get_variables();
                    for value in state.local_variables {
                        local_variables.insert(value);
                    }
                    let branch_state = &State {
                        local_variables: &local_variables,
                        ..*state
                    };
                    let (template, template_length) = template.to_tokens(branch_state);
                    estimated_length = estimated_length.min(template_length);

                    let ty = ty.to_tokens(state);

                    if is_elseif {
                        tokens.append_all(quote! { else if let #ty = #expression { #template } });
                    } else {
                        tokens.append_all(quote! { if let #ty = #expression { #template } });
                    }
                }
            }

            is_elseif = true;
        }
        if let Some(template) = &self.otherwise {
            let (template, template_length) = template.to_tokens(state);
            estimated_length = estimated_length.min(template_length);
            tokens.append_all(quote! { else { #template } });
        }

        (tokens, estimated_length)
    }
}

impl<'a> From<If<'a>> for StatementKind<'a> {
    fn from(statement: If<'a>) -> Self {
        StatementKind::If(statement)
    }
}

pub(super) fn parse_if(input: Source) -> Res<Source, Statement> {
    let (input, statement_source) = tag("if")(input)?;

    let (input, (if_type, if_type_source)) = cut(parse_if_generic).parse(input)?;

    let source = statement_source.merge(&if_type_source, "Type source expected after if");

    Ok((
        input,
        Statement {
            kind: If {
                ifs: vec![(if_type, Template(vec![]))],
                otherwise: None,
                is_ended: false,
            }
            .into(),
            source,
        },
    ))
}

fn parse_if_generic(input: Source) -> Res<Source, (IfType, Source)> {
    // Consume at least one whitespace.
    let (input, leading_whitespace) = context("Expected a space", whitespace).parse(input)?;

    let mut source = leading_whitespace;

    let (input, r#let) = opt((tag("let"), whitespace)).parse(input)?;

    if let Some((let_tag, let_whitespace)) = r#let {
        let (input, ty) =
            context(r#"Expected a type after "let""#, cut(parse_type)).parse(input)?;
        let (input, (leading_whitespace, equal, trailing_whitespace, expression)) = (
            opt(whitespace),
            context("Expected `=`", cut(tag("="))),
            opt(whitespace),
            context(
                "Expected an expression after `=`",
                cut(expression(true, true)),
            ),
        )
            .parse(input)?;

        source = source
            .merge(&let_tag, "`let` expected after whitespace")
            .merge(&let_whitespace, "Whitespace expected after `let`")
            .merge(&ty.source(), "Type expected after whitespace")
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after type",
            )
            .merge(&equal, "`=` expected after whitespace")
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after `=`",
            )
            .merge(&expression.source(), "Expression expected after whitespace");

        Ok((input, (IfType::IfLet(ty, expression), source)))
    } else {
        let (input, output) = context(
            "Expected an expression after `if`",
            cut(expression(true, true)),
        )
        .parse(input)?;

        source = source.merge(&output.source(), "Expression expected after whitespace");

        Ok((input, (IfType::If(output), source)))
    }
}

pub(super) fn parse_elseif(input: Source) -> Res<Source, Statement> {
    let (input, statement_source) = tag("elseif").parse(input)?;

    let (input, (if_type, if_source)) = cut(parse_if_generic).parse(input)?;

    let source = statement_source.merge(&if_source, "Expression expected after `elseif`");

    Ok((
        input,
        Statement {
            kind: ElseIf(if_type).into(),
            source,
        },
    ))
}

pub(super) fn parse_else(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("else").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::Else,
            source: output,
        },
    ))
}

pub(super) fn parse_endif(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endif").parse(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndIf,
            source: output,
        },
    ))
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq)]
pub struct ElseIf<'a>(IfType<'a>);

impl<'a> From<ElseIf<'a>> for StatementKind<'a> {
    fn from(statement: ElseIf<'a>) -> Self {
        StatementKind::ElseIf(statement)
    }
}
