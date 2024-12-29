use super::super::{expression::expression, Item, Res};
use super::{State, Statement, StatementKind};
use crate::syntax::expression::{ident, Identifier};
use crate::syntax::template::is_whitespace;
use crate::syntax::Expression;
use crate::Source;
use nom::bytes::complete::take_while1;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::char;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::collections::HashSet;

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
            Type(_, _, TypeOrIdent::Identifier(ident)) => HashSet::from([ident.0]),
            Type(_, _, TypeOrIdent::Type(ty)) => ty.get_variables(),
        }
    }

    pub fn get_ident(&self) -> Option<&Identifier> {
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
    If(Expression<'a>),
    IfLet(Type<'a>, Option<Expression<'a>>),
}

#[derive(Debug)]
pub(crate) struct If<'a> {
    pub ifs: Vec<(IfType<'a>, Vec<Item<'a>>)>,
    pub otherwise: Option<Vec<Item<'a>>>,
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
        match item {
            Item::Statement(Statement {
                kind: StatementKind::ElseIf(ElseIf(if_type)),
                source: _,
            }) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.ifs.push((if_type, vec![]));
            }
            Item::Statement(Statement {
                kind: StatementKind::Else,
                source: _,
            }) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.otherwise = Some(vec![]);
            }
            Item::Statement(Statement {
                kind: StatementKind::EndIf,
                source: _,
            }) => {
                self.is_ended = true;
            }
            _ => {
                if self.is_ended {
                    todo!();
                }
                if let Some(items) = &mut self.otherwise {
                    items.push(item);
                } else {
                    self.ifs.last_mut().unwrap().1.push(item);
                }
            }
        }
    }
}

impl<'a> From<If<'a>> for StatementKind<'a> {
    fn from(statement: If<'a>) -> Self {
        StatementKind::If(statement)
    }
}

impl ToTokens for If<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut is_elseif = false;
        for (expression, items) in &self.ifs {
            match expression {
                IfType::If(expression) => {
                    if is_elseif {
                        tokens.append_all(quote! { else if #expression { #(#items);* } });
                    } else {
                        tokens.append_all(quote! { if #expression { #(#items);* } });
                    }
                }
                IfType::IfLet(ty, Some(expression)) => {
                    if is_elseif {
                        tokens
                            .append_all(quote! { else if let #ty = &#expression { #(#items);* } });
                    } else {
                        tokens.append_all(quote! { if let #ty = &#expression { #(#items);* } });
                    }
                }
                IfType::IfLet(ty, None) => {
                    let expression = ty
                        .get_ident()
                        .expect("Expressionless if let statements should have an ident available");
                    if is_elseif {
                        tokens.append_all(quote! { else if let #ty = #expression { #(#items);* } });
                    } else {
                        tokens.append_all(quote! { if let #ty = #expression { #(#items);* } });
                    }
                }
            }

            is_elseif = true;
        }
        if let Some(items) = &self.otherwise {
            tokens.append_all(quote! { else { #(#items)* } });
        }
    }
}

pub(super) fn parse_type_name(input: Source) -> Res<Source, TypeName> {
    let (input, ident) =
        take_while1(|char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))(input)?;
    Ok((input, TypeName(ident.as_str(), ident)))
}

pub(super) fn parse_type<'a>(_state: &'a State) -> impl FnMut(Source) -> Res<Source, Type> + 'a {
    |input| {
        let (input, (path_segments, type_name, _open, identifier, _close)) = cut(tuple((
            many0(tuple((parse_type_name, tag("::")))),
            parse_type_name,
            char('('),
            ident,
            char(')'),
        )))(input)?;
        Ok((
            input,
            Type(
                path_segments,
                type_name,
                TypeOrIdent::Identifier(identifier),
            ),
        ))
    }
}

pub(super) fn parse_if<'a>(state: &'a State) -> impl FnMut(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, statement_source) = tag("if")(input)?;

        let (input, if_type) = parse_if_generic(state)(input)?;

        Ok((
            input,
            Statement {
                kind: If {
                    ifs: vec![(if_type, vec![])],
                    otherwise: None,
                    is_ended: false,
                }
                .into(),
                source: statement_source,
            },
        ))
    }
}

fn parse_if_generic<'a>(state: &'a State) -> impl FnMut(Source) -> Res<Source, IfType> + 'a {
    |input| {
        let ws1 = take_while1(is_whitespace);
        let ws0 = take_while(is_whitespace);

        // Consume at least one whitespace.
        let (input, _) = ws1(input)?;

        let (input, r#let) = cut(opt(tuple((tag("let"), &ws1))))(input)?;

        if r#let.is_some() {
            let (input, ty) =
                context(r#"Expected a type after "let""#, cut(parse_type(state)))(input)?;
            let (input, expression) = if ty.get_variables().len() == 1 {
                opt(preceded(
                    &ws0,
                    preceded(
                        char('='),
                        preceded(
                            &ws0,
                            context("Expected an expression after `=`", cut(expression(state))),
                        ),
                    ),
                ))(input)?
            } else {
                let (input, expression) = preceded(
                    &ws0,
                    preceded(
                        context("Expected `=`", cut(char('='))),
                        preceded(
                            &ws0,
                            context("Expected an expression after `=`", cut(expression(state))),
                        ),
                    ),
                )(input)?;
                (input, Some(expression))
            };
            Ok((input, IfType::IfLet(ty, expression)))
        } else {
            let (input, output) = cut(expression(state))(input)?;
            Ok((input, IfType::If(output)))
        }
    }
}

pub(super) fn parse_elseif<'a>(state: &'a State) -> impl Fn(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, statement_source) = tag("elseif")(input)?;

        let (input, if_type) = parse_if_generic(state)(input)?;

        Ok((
            input,
            Statement {
                kind: ElseIf(if_type).into(),
                source: statement_source,
            },
        ))
    }
}

pub(super) fn parse_else(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("else")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::Else,
            source: output,
        },
    ))
}

pub(super) fn parse_endif(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endif")(input)?;

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
