use nom::bytes::complete::take_while1;
use crate::syntax::Expression;
use nom::combinator::cut;
use crate::syntax::item::tag_end;
use crate::syntax::template::{is_whitespace, parse_item};
use nom::bytes::complete::take_while;
use nom::sequence::preceded;
use nom::branch::alt;
use super::{expression::expression, Item, Res, Span, Static};
use nom::bytes::complete::tag;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    If(If<'a>),
    ElseIf(ElseIf<'a>),
    Else,
    EndIf,
}

impl<'a> Statement<'a> {
    pub fn is_ended(&self) -> bool {
        match self {
            Statement::If(statement) => statement.is_ended,
            _ => true,
        }
    }
    
    pub fn add_item(&mut self, item: Item<'a>) {
        match self {
            Statement::If(statement) => statement.add_item(item),
            _ => unreachable!(),
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
        match self {
            Statement::If(statement) => {
                tokens.append_all(quote! { #statement });
            }
            Statement::ElseIf(_) | Statement::Else | Statement::EndIf => {
                unreachable!("These should be removed by If statements");
            }
        }
    }
}

pub(super) fn statement(input: Span) -> Res<&str, (Item, Option<Static>)> {
    // Ignore any leading inner whitespace
    let (input, _) = take_while(is_whitespace)(input)?;

    // Parse statements
    let (input, mut statement) = cut(alt((parse_if, parse_else, parse_endif)))(input)?;

    // Parse the closing tag and any trailing whitespace
    let (mut input, trailing_whitespace) =
        preceded(take_while(is_whitespace), cut(tag_end("%}")))(input)?;

    if !statement.is_ended() {
        loop {
            let (new_input, items) = parse_item(input)?;
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
    }

    // Return the statement and trailing whitespace
    Ok((input, (statement.into(), trailing_whitespace)))
}

#[derive(Debug, PartialEq, Eq)]
pub struct If<'a> {
    pub ifs: Vec<(Expression<'a>, Vec<Item<'a>>)>,
    pub otherwise: Option<Vec<Item<'a>>>,
    pub is_ended: bool,
}

impl<'a> If<'a> {
    pub fn add_item(&mut self, item: Item<'a>) {
        match item {
            Item::Statement(Statement::ElseIf(ElseIf(expression))) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.ifs.push((expression, vec![]));
            }
            Item::Statement(Statement::Else) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.otherwise = Some(vec![]);
            }
            Item::Statement(Statement::EndIf) => {
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

impl<'a> From<If<'a>> for Statement<'a> {
    fn from(statement: If<'a>) -> Self {
        Statement::If(statement)
    }
}

impl ToTokens for If<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut is_elseif = false;
        for (expression, items) in &self.ifs {
            let if_or_elseif = if !is_elseif { quote! { if } } else { quote! { else if } };
            is_elseif = true;
            tokens.append_all(quote! { #if_or_elseif #expression { #(#items);* } });
        }
        if let Some(items) = &self.otherwise {
            tokens.append_all(quote! { else { #(#items)* } });
        }
    }
}

fn parse_if(input: Span) -> Res<&str, Statement> {
    let (input, _) = tag("if")(input)?;

    // Consume at least one whitespace.
    let (input, _) = cut(take_while1(is_whitespace))(input)?;

    let (input, output) = cut(expression)(input)?;

    Ok((input, If {
        ifs: vec![(output, vec![])],
        otherwise: None,
        is_ended: false,
    }.into()))
}

fn parse_else(input: Span) -> Res<&str, Statement> {
    let (input, _) = tag("else")(input)?;

    Ok((input, Statement::Else))
}

fn parse_endif(input: Span) -> Res<&str, Statement> {
    let (input, _) = tag("endif")(input)?;

    Ok((input, Statement::EndIf))
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElseIf<'a>(Expression<'a>);
