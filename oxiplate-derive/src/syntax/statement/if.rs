use std::collections::HashSet;

use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use super::super::expression::expression;
use super::super::{Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::expression::{ident, ExpressionAccess, Identifier};
use crate::syntax::template::{is_whitespace, Template};
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TypeName<'a>(&'a str, Source<'a>);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Type<'a>(
    Vec<(TypeName<'a>, Source<'a>)>,
    TypeName<'a>,
    TypeOrIdent<'a>,
);

impl<'a> Type<'a> {
    pub fn get_variables(&self) -> HashSet<&'a str> {
        match self {
            Type(_, _, TypeOrIdent::Identifier(ident)) => HashSet::from([ident.ident]),
            Type(_, _, TypeOrIdent::Type(ty)) => ty.get_variables(),
        }
    }

    pub fn get_ident(&'_ self) -> Option<&'_ Identifier<'_>> {
        match self {
            Type(_, _, TypeOrIdent::Identifier(ident)) => Some(ident),
            Type(_, _, TypeOrIdent::Type(ty)) => ty.get_ident(),
        }
    }
}

impl ToTokens for Type<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.2 {
            TypeOrIdent::Type(_) => todo!(),
            TypeOrIdent::Identifier(ident) => {
                for (type_name, separator) in &self.0 {
                    let type_name: proc_macro2::TokenStream =
                        type_name.0.parse().expect("Should be able to parse type");
                    let separator: proc_macro2::TokenStream = separator
                        .as_str()
                        .parse()
                        .expect("Should be able to parse type");
                    tokens.append_all(quote! {
                        #type_name #separator
                    });
                }

                let type_name: proc_macro2::TokenStream =
                    self.1 .0.parse().expect("Should be able to parse type");
                tokens.append_all(quote! {
                    #type_name(#ident)
                });
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TypeOrIdent<'a> {
    #[allow(dead_code)]
    Type(Box<Type<'a>>),
    Identifier(Identifier<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IfType<'a> {
    If(ExpressionAccess<'a>),
    IfLet(Type<'a>, Option<ExpressionAccess<'a>>),
}

#[derive(Debug)]
pub(crate) struct If<'a> {
    pub ifs: Vec<(IfType<'a>, Template<'a>)>,
    pub otherwise: Option<Template<'a>>,
    pub is_ended: bool,
}

impl<'a> If<'a> {
    pub fn get_active_variables(&self) -> HashSet<&'a str> {
        match self.ifs.last() {
            Some((IfType::If(_), _)) => HashSet::new(),
            Some((IfType::IfLet(ty, _), _)) => ty.get_variables(),
            None => unreachable!("If statements should always have at least one if"),
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            unreachable!(
                "Should not attempt to add item to `if` statement after statement is ended."
            );
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::ElseIf(ElseIf(if_type)),
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError(
                        "`else` previously present in this if statement; expected `endif`"
                            .to_string(),
                        source,
                    ));
                } else {
                    self.ifs.push((if_type, Template(vec![])));
                }
            }
            Item::Statement(Statement {
                kind: StatementKind::Else,
                source,
            }) => {
                if let Some(ref mut ifs) = self.otherwise {
                    ifs.0.push(Item::CompileError(
                        "`else` already present in this if statement; expected `endif`".to_string(),
                        source,
                    ));
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
                    self.ifs.last_mut().unwrap().1 .0.push(item);
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
                IfType::IfLet(ty, Some(expression)) => {
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

                    if is_elseif {
                        tokens.append_all(quote! { else if let #ty = &#expression { #template } });
                    } else {
                        tokens.append_all(quote! { if let #ty = &#expression { #template } });
                    }
                }
                IfType::IfLet(ty, None) => {
                    let expression = ty
                        .get_ident()
                        .expect("Expressionless if let statements should have an ident available");

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

pub(super) fn parse_type_name(input: Source) -> Res<Source, TypeName> {
    let (input, ident) =
        take_while1(|char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
            .parse(input)?;
    Ok((input, TypeName(ident.as_str(), ident)))
}

pub(super) fn parse_type(input: Source) -> Res<Source, Type> {
    let (input, (path_segments, type_name, _open, identifier, _close)) = cut((
        many0((parse_type_name, tag("::"))),
        parse_type_name,
        char('('),
        ident,
        char(')'),
    ))
    .parse(input)?;
    Ok((
        input,
        Type(
            path_segments,
            type_name,
            TypeOrIdent::Identifier(identifier),
        ),
    ))
}

pub(super) fn parse_if(input: Source) -> Res<Source, Statement> {
    let (input, statement_source) = tag("if")(input)?;

    let (input, if_type) = parse_if_generic(input)?;

    Ok((
        input,
        Statement {
            kind: If {
                ifs: vec![(if_type, Template(vec![]))],
                otherwise: None,
                is_ended: false,
            }
            .into(),
            source: statement_source,
        },
    ))
}

fn parse_if_generic(input: Source) -> Res<Source, IfType> {
    // Consume at least one whitespace.
    let (input, _) = take_while1(is_whitespace).parse(input)?;

    let (input, r#let) = cut(opt((tag("let"), take_while1(is_whitespace)))).parse(input)?;

    if r#let.is_some() {
        let (input, ty) =
            context(r#"Expected a type after "let""#, cut(parse_type)).parse(input)?;
        let (input, expression) = if ty.get_variables().len() == 1 {
            opt(preceded(
                take_while(is_whitespace),
                preceded(
                    char('='),
                    preceded(
                        take_while(is_whitespace),
                        context(
                            "Expected an expression after `=`",
                            cut(expression(true, true)),
                        ),
                    ),
                ),
            ))
            .parse(input)?
        } else {
            let (input, expression) = preceded(
                take_while(is_whitespace),
                preceded(
                    context("Expected `=`", cut(char('='))),
                    preceded(
                        take_while(is_whitespace),
                        context(
                            "Expected an expression after `=`",
                            cut(expression(true, true)),
                        ),
                    ),
                ),
            )
            .parse(input)?;
            (input, Some(expression))
        };
        Ok((input, IfType::IfLet(ty, expression)))
    } else {
        let (input, output) = context(
            "Expected an expression after `if`",
            cut(expression(true, true)),
        )
        .parse(input)?;
        Ok((input, IfType::If(output)))
    }
}

pub(super) fn parse_elseif(input: Source) -> Res<Source, Statement> {
    let (input, statement_source) = tag("elseif").parse(input)?;

    let (input, if_type) = parse_if_generic(input)?;

    Ok((
        input,
        Statement {
            kind: ElseIf(if_type).into(),
            source: statement_source,
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
