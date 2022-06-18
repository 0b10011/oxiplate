use std::collections::HashSet;

use super::expression::{ident, keyword, Identifier, Keyword};
use super::{expression::expression, Item, Res, Static};
use crate::syntax::item::tag_end;
use crate::syntax::template::{is_whitespace, parse_item};
use crate::syntax::Expression;
use crate::Source;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::combinator::cut;
use nom::combinator::fail;
use nom::error::context;
use nom::sequence::{preceded, tuple};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub(crate) struct Statement<'a> {
    source: Source<'a>,
    kind: StatementKind<'a>,
}

#[derive(Debug)]
pub(crate) enum StatementKind<'a> {
    If(If<'a>),
    ElseIf(ElseIf<'a>),
    Else,
    EndIf,
    For(For<'a>),
    EndFor,
}

impl<'a> Statement<'a> {
    pub fn is_ended(&self) -> bool {
        match &self.kind {
            StatementKind::If(statement) => statement.is_ended,
            StatementKind::For(statement) => statement.is_ended,
            _ => true,
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        match &mut self.kind {
            StatementKind::If(statement) => statement.add_item(item),
            StatementKind::For(statement) => statement.add_item(item),
            _ => unreachable!(),
        }
    }

    pub fn get_active_variables(&self) -> HashSet<&'a str> {
        match &self.kind {
            StatementKind::For(statement) => statement.get_active_variables(),
            _ => HashSet::new(),
        }
    }
}

impl<'a> From<Statement<'a>> for Item<'a> {
    fn from(statement: Statement<'a>) -> Self {
        Item::Statement(statement)
    }
}

impl ToTokens for Statement<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match &self.kind {
            StatementKind::If(statement) => quote! { #statement },
            StatementKind::ElseIf(_) => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'elseif' statement"); }
            }
            StatementKind::Else => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'else' statement"); }
            }
            StatementKind::EndIf => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'endif' statement"); }
            }

            StatementKind::For(statement) => quote! { #statement },
            StatementKind::EndFor => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'endfor' statement"); }
            }
        });
    }
}

pub(super) fn statement<'a>(
    local_variables: &'a HashSet<&'a str>,
) -> impl Fn(Source) -> Res<Source, (Item, Option<Static>)> + 'a {
    |input| {
        // Ignore any leading inner whitespace
        let (input, _) = take_while(is_whitespace)(input)?;

        // Parse statements
        let (input, mut statement) = context(
            "Expected one of: if, elseif, else, endif, for, endfor",
            cut(alt((
                parse_if(local_variables),
                parse_elseif(local_variables),
                parse_else,
                parse_endif,
                parse_for(local_variables),
                parse_endfor,
            ))),
        )(input)?;

        // Parse the closing tag and any trailing whitespace
        let (mut input, trailing_whitespace) =
            preceded(take_while(is_whitespace), cut(tag_end("%}")))(input)?;

        if !statement.is_ended() {
            // Merge new variables from this statement into the existing local variables
            let mut new_local_variables = statement.get_active_variables();
            for value in local_variables.iter() {
                new_local_variables.insert(value);
            }
            let local_variables = new_local_variables;

            loop {
                let parsed_item = parse_item(&local_variables)(input);
                if parsed_item.is_err() {
                    return context("This statement is never closed.", fail)(statement.source);
                }
                let (new_input, items) =
                    parsed_item.expect("Error possibility should have been checked already");
                input = new_input;
                for item in items {
                    if statement.is_ended() {
                        todo!("This can happen with tags and trailing whitespace I think");
                    }

                    statement.add_item(item);
                }
                if statement.is_ended() {
                    break;
                }
            }
            drop(local_variables);
        }

        // Return the statement and trailing whitespace
        Ok((input, (statement.into(), trailing_whitespace)))
    }
}

#[derive(Debug)]
pub(crate) struct If<'a> {
    pub ifs: Vec<(Expression<'a>, Vec<Item<'a>>)>,
    pub otherwise: Option<Vec<Item<'a>>>,
    pub is_ended: bool,
}

impl<'a> If<'a> {
    pub fn add_item(&mut self, item: Item<'a>) {
        match item {
            Item::Statement(Statement {
                kind: StatementKind::ElseIf(ElseIf(expression)),
                source: _,
            }) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.ifs.push((expression, vec![]));
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
            let if_or_elseif = if !is_elseif {
                quote! { if }
            } else {
                quote! { else if }
            };
            is_elseif = true;
            tokens.append_all(quote! { #if_or_elseif #expression { #(#items);* } });
        }
        if let Some(items) = &self.otherwise {
            tokens.append_all(quote! { else { #(#items)* } });
        }
    }
}

fn parse_if<'a>(
    local_variables: &'a HashSet<&'a str>,
) -> impl FnMut(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, statement_source) = tag("if")(input)?;

        // Consume at least one whitespace.
        let (input, _) = cut(take_while1(is_whitespace))(input)?;

        let (input, output) = cut(expression(local_variables))(input)?;

        Ok((
            input,
            Statement {
                kind: If {
                    ifs: vec![(output, vec![])],
                    otherwise: None,
                    is_ended: false,
                }
                .into(),
                source: statement_source,
            },
        ))
    }
}

fn parse_elseif<'a>(
    local_variables: &'a HashSet<&'a str>,
) -> impl Fn(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, statement_source) = tag("elseif")(input)?;

        // Consume at least one whitespace.
        let (input, _) = cut(take_while1(is_whitespace))(input)?;

        let (input, output) = cut(expression(local_variables))(input)?;

        Ok((
            input,
            Statement {
                kind: ElseIf(output).into(),
                source: statement_source,
            },
        ))
    }
}

fn parse_else(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("else")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::Else,
            source: output,
        },
    ))
}

fn parse_endif(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endif")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndIf,
            source: output,
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElseIf<'a>(Expression<'a>);

impl<'a> From<ElseIf<'a>> for StatementKind<'a> {
    fn from(statement: ElseIf<'a>) -> Self {
        StatementKind::ElseIf(statement)
    }
}

#[derive(Debug)]
pub struct For<'a> {
    for_keyword: Keyword<'a>,
    ident: Identifier<'a>,
    in_keyword: Keyword<'a>,
    expression: Expression<'a>,
    items: Vec<Item<'a>>,
    is_ended: bool,
}

impl<'a> For<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            todo!();
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::EndFor,
                ..
            }) => {
                self.is_ended = true;
            }
            _ => {
                self.items.push(item);
            }
        }
    }

    pub(crate) fn get_active_variables(&self) -> HashSet<&'a str> {
        HashSet::from([self.ident.0])
    }
}

impl<'a> From<For<'a>> for StatementKind<'a> {
    fn from(statement: For<'a>) -> Self {
        StatementKind::For(statement)
    }
}

impl ToTokens for For<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let For {
            for_keyword,
            ident,
            in_keyword,
            expression,
            items,
            ..
        } = self;
        let ident = syn::Ident::new(ident.1.as_str(), ident.1.span());
        tokens.append_all(quote! { #for_keyword #ident #in_keyword #expression { #(#items)* } });
    }
}

fn parse_for<'a>(
    local_variables: &'a HashSet<&'a str>,
) -> impl Fn(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, for_keyword) = keyword("for")(input)?;

        let (input, (_, ident, _, in_keyword, _, expression)) = cut(tuple((
            context("Expected space after 'for'", take_while1(is_whitespace)),
            context("Expected an identifier", ident),
            context(
                "Expected space after identifier",
                take_while1(is_whitespace),
            ),
            context("Expected 'in'", keyword("in")),
            context("Expected space after 'in'", take_while1(is_whitespace)),
            context(
                "Expected an expression that is iterable",
                expression(local_variables),
            ),
        )))(input)?;

        let source = for_keyword.0.clone();

        Ok((
            input,
            Statement {
                kind: For {
                    for_keyword,
                    ident,
                    in_keyword,
                    expression,
                    items: vec![],
                    is_ended: false,
                }
                .into(),
                source,
            },
        ))
    }
}

fn parse_endfor(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endfor")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndFor,
            source: output,
        },
    ))
}
