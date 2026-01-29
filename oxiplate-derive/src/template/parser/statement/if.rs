use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};

use super::super::expression::expression;
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::parser::{Parser as _, cut, opt, take};
use crate::template::parser::expression::{ExpressionAccess, KeywordParser};
use crate::template::parser::statement::helpers::pattern::Pattern;
use crate::template::parser::template::Template;
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State, internal_error};

#[derive(Debug)]
pub(crate) enum IfType<'a> {
    If(ExpressionAccess<'a>),
    IfLet(Pattern<'a>, Box<ExpressionAccess<'a>>),
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
            internal_error!(
                item.source().span_token().unwrap(),
                "Attempted to add item to ended `if` statement",
            );
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
            Item::Statement(Statement { kind, source }) if !kind.expected_in_statements() => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs
                } else {
                    &mut self.ifs.last_mut().unwrap().1
                }
                .0
                .push(Item::CompileError {
                    message: "Unexpected statement in `if` statement; is an `endif` statement \
                              missing or `if` used instead of `elseif`?"
                        .to_string(),
                    error_source: source.clone(),
                    consumed_source: source,
                });
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

    pub(crate) fn to_tokens<'b: 'a>(&self, state: &mut State<'b>) -> BuiltTokens {
        let mut tokens = TokenStream::new();
        let mut estimated_length = usize::MAX;

        let mut is_elseif = false;
        for (expression, template) in &self.ifs {
            state.local_variables.push_stack();

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
                IfType::IfLet(pattern, expression) => {
                    let (expression, _expression_length) = expression.to_tokens(state);

                    state.local_variables.add(
                        pattern
                            .get_variables()
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                    );
                    let (template, template_length) = template.to_tokens(state);
                    estimated_length = estimated_length.min(template_length);

                    let pattern = pattern.to_tokens(state);

                    if is_elseif {
                        tokens.append_all(
                            quote! { else if let #pattern = #expression { #template } },
                        );
                    } else {
                        tokens.append_all(quote! { if let #pattern = #expression { #template } });
                    }
                }
            }

            state.local_variables.pop_stack();
            is_elseif = true;
        }
        if let Some(template) = &self.otherwise {
            state.local_variables.push_stack();

            let (template, template_length) = template.to_tokens(state);
            estimated_length = estimated_length.min(template_length);
            tokens.append_all(quote! { else { #template } });

            state.local_variables.pop_stack();
        }

        (tokens, estimated_length)
    }
}

impl<'a> From<If<'a>> for StatementKind<'a> {
    fn from(statement: If<'a>) -> Self {
        StatementKind::If(statement)
    }
}

pub(super) fn parse_if(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, (keyword, (if_type, if_type_source))) = (
        KeywordParser::new("if"),
        cut("Expected an if expression", parse_if_generic),
    )
        .parse(tokens)?;

    let source = keyword
        .source()
        .clone()
        .merge(&if_type_source, "Type source expected after if");

    Ok((
        tokens,
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

fn parse_if_generic(tokens: TokenSlice) -> Res<(IfType, Source)> {
    let (tokens, keyword) = opt(KeywordParser::new("let")).parse(tokens)?;

    if let Some(keyword) = keyword {
        let (tokens, (pattern, equal, expression)) = (
            cut(r#"Expected a pattern after "let""#, Pattern::parse),
            cut("Expected `=`", take(TokenKind::Equal)),
            cut("Expected an expression after `=`", expression(true, true)),
        )
            .parse(tokens)?;

        let source = keyword
            .source()
            .clone()
            .merge(pattern.source(), "Pattern expected after `let`")
            .merge(equal.source(), "`=` expected after pattern")
            .merge(&expression.source(), "Expression expected after `=`");

        Ok((
            tokens,
            (IfType::IfLet(pattern, Box::new(expression)), source),
        ))
    } else {
        let (tokens, output) =
            cut("Expected an expression after `if`", expression(true, true)).parse(tokens)?;

        let source = output.source().clone();

        Ok((tokens, (IfType::If(output), source)))
    }
}

pub(super) fn parse_elseif(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, (keyword, (if_type, if_source))) = (
        KeywordParser::new("elseif"),
        cut("Expected an if expression", parse_if_generic),
    )
        .parse(tokens)?;

    let source = keyword
        .source()
        .clone()
        .merge(&if_source, "Expression expected after `elseif`");

    Ok((
        tokens,
        Statement {
            kind: ElseIf(if_type).into(),
            source,
        },
    ))
}

pub(super) fn parse_else(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, output) = KeywordParser::new("else").parse(tokens)?;

    Ok((
        tokens,
        Statement {
            kind: StatementKind::Else,
            source: output.source().clone(),
        },
    ))
}

pub(super) fn parse_endif(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, output) = KeywordParser::new("endif").parse(tokens)?;

    Ok((
        tokens,
        Statement {
            kind: StatementKind::EndIf,
            source: output.source().clone(),
        },
    ))
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ElseIf<'a>(IfType<'a>);

impl<'a> From<ElseIf<'a>> for StatementKind<'a> {
    fn from(statement: ElseIf<'a>) -> Self {
        StatementKind::ElseIf(statement)
    }
}
