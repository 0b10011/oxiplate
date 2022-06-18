use std::collections::HashSet;

use super::{template::whitespace, Res};
use crate::Source;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{pair, terminated, tuple};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

// #[derive(Debug, PartialEq)]
// // https://doc.rust-lang.org/reference/expressions/literal-expr.html
// enum Literal<'a> {
//     Char(char),
//     String(&'a str),
//     Byte(u8),
//     ByteString(&'a Vec<u8>),
//     Integer(i64),
//     Float(f64),
//     Boolean(bool),
// }

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Keyword<'a>(pub Source<'a>);

impl ToTokens for Keyword<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.0.span();
        let keyword = syn::Ident::new(self.0.as_str(), span);
        tokens.append_all(quote_spanned! {span=> #keyword });
    }

    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

pub(super) fn keyword<'a>(
    keyword: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, Keyword<'a>> + 'a {
    move |input: Source<'a>| {
        let (input, keyword) = tag(keyword)(input)?;
        Ok((input, Keyword(keyword)))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Identifier<'a>(pub &'a str, pub Source<'a>);

impl ToTokens for Identifier<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.1.as_str();
        let span = self.1.span();
        tokens.append_all(quote_spanned! {span=> #ident });
    }
}

pub(super) fn ident(input: Source) -> Res<Source, Identifier> {
    let (input, ident) =
        take_while1(|char: char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))(input)?;
    Ok((input, Identifier(ident.as_str(), ident)))
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IdentifierOrFunction<'a> {
    Identifier(Identifier<'a>),
    Function(Identifier<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdentField<'a>(Vec<Identifier<'a>>, IdentifierOrFunction<'a>);

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression<'a> {
    LocalVariable(Identifier<'a>),
    Identifier(IdentifierOrFunction<'a>),
    FieldAccess(IdentField<'a>),
    // Group(Box<Expression<'a>>),
    Calc(Box<Expression<'a>>, Operator, Box<Expression<'a>>),
    Prefixed(PrefixOperator, Box<Expression<'a>>),
}

impl ToTokens for Expression<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Expression::LocalVariable(identifier) => {
                let span = identifier.1.span();
                let identifier = syn::Ident::new(identifier.0, span);
                quote_spanned! {span=> #identifier }
            }
            Expression::Identifier(identifier) => match &identifier {
                IdentifierOrFunction::Identifier(identifier) => {
                    let span = identifier.1.span();
                    let identifier = syn::Ident::new(identifier.0, span);
                    quote_spanned! {span=> self.#identifier }
                }
                IdentifierOrFunction::Function(identifier) => {
                    let span = identifier.1.span();
                    let identifier = syn::Ident::new(identifier.0, span);
                    quote_spanned! {span=> self.#identifier() }
                }
            },
            Expression::FieldAccess(identifier) => {
                let mut parents = Vec::with_capacity(identifier.0.len());
                let parent_identifiers = &identifier.0;
                for parent in parent_identifiers {
                    parents.push(syn::Ident::new(parent.0, parent.1.span()));
                }
                match &identifier.1 {
                    IdentifierOrFunction::Identifier(identifier) => {
                        let span = identifier.1.span();
                        for parent in parent_identifiers {
                            span.join(parent.1.span());
                        }
                        let identifier = syn::Ident::new(identifier.0, span);
                        quote_spanned! {span=> self.#(#parents.)*#identifier }
                    }
                    IdentifierOrFunction::Function(identifier) => {
                        let span = identifier.1.span();
                        for parent in parent_identifiers {
                            span.join(parent.1.span());
                        }
                        let identifier = syn::Ident::new(identifier.0, span);
                        quote_spanned! {span=> self.#(#parents.)*#identifier() }
                    }
                }
            }
            Expression::Calc(left, operator, right) => quote!(#left #operator #right),
            Expression::Prefixed(operator, expression) => quote!(#operator #expression),
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    Or,
    And,
}

impl ToTokens for Operator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Operator::Addition => quote!(+),
            Operator::Subtraction => quote!(-),
            Operator::Multiplication => quote!(*),
            Operator::Division => quote!(/),
            Operator::Remainder => quote!(%),

            Operator::Equal => quote!(==),
            Operator::NotEqual => quote!(!=),
            Operator::GreaterThan => quote!(>),
            Operator::LessThan => quote!(<),
            Operator::GreaterThanOrEqual => quote!(>=),
            Operator::LessThanOrEqual => quote!(<=),

            Operator::Or => quote!(||),
            Operator::And => quote!(&&),
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOperator {
    Borrow,
    Dereference,
}

impl ToTokens for PrefixOperator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            PrefixOperator::Borrow => quote!(&),
            PrefixOperator::Dereference => quote!(*),
        });
    }
}

pub(super) fn expression<'a>(
    local_variables: &'a HashSet<&'a str>,
) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
    |input| {
        fn field_or_identifier<'a>(
            local_variables: &'a HashSet<&'a str>,
        ) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
            |input| {
                let (input, (parsed_parents, (ident, maybe_function))) = pair(
                    many0(terminated(&ident, char('.'))),
                    pair(&ident, opt(tag("()"))),
                )(input)?;

                let field = if maybe_function.is_some() {
                    IdentifierOrFunction::Function(ident)
                } else if local_variables.contains(ident.0) {
                    return Ok((input, Expression::LocalVariable(ident)));
                } else {
                    IdentifierOrFunction::Identifier(ident)
                };

                if parsed_parents.is_empty() {
                    return Ok((input, Expression::Identifier(field)));
                }

                let mut parents = Vec::with_capacity(parsed_parents.len());
                for parent in parsed_parents {
                    parents.push(parent);
                }

                Ok((input, Expression::FieldAccess(IdentField(parents, field))))
            }
        }
        fn operator(input: Source) -> Res<Source, Operator> {
            let (input, operator) = alt((
                tag("+"),
                tag("-"),
                tag("*"),
                tag("/"),
                tag("%"),
                tag("=="),
                tag("!="),
                tag(">="),
                tag("<="),
                tag(">"),
                tag("<"),
                tag("||"),
                tag("&&"),
            ))(input)?;

            let operator = match operator.as_str() {
                "+" => Operator::Addition,
                "-" => Operator::Subtraction,
                "*" => Operator::Multiplication,
                "/" => Operator::Division,
                "%" => Operator::Remainder,

                "==" => Operator::Equal,
                "!=" => Operator::NotEqual,
                ">" => Operator::GreaterThan,
                "<" => Operator::LessThan,
                ">=" => Operator::GreaterThanOrEqual,
                "<=" => Operator::LessThanOrEqual,

                "||" => Operator::Or,
                "&&" => Operator::And,

                _ => unreachable!("All cases should be covered"),
            };

            Ok((input, operator))
        }
        fn calc<'a>(
            local_variables: &'a HashSet<&'a str>,
        ) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
            |input| {
                let (input, (left, _leading_whitespace, operator, _trailing_whitespace, right)) =
                    tuple((
                        field_or_identifier(local_variables),
                        opt(whitespace),
                        operator,
                        opt(whitespace),
                        field_or_identifier(local_variables),
                    ))(input)?;
                Ok((
                    input,
                    Expression::Calc(Box::new(left), operator, Box::new(right)),
                ))
            }
        }
        fn prefix_operator(input: Source) -> Res<Source, PrefixOperator> {
            let (input, operator) = alt((tag("&"), tag("*")))(input)?;
            let operator = match operator.as_str() {
                "&" => PrefixOperator::Borrow,
                "*" => PrefixOperator::Dereference,
                _ => unreachable!("All cases should be covered"),
            };

            Ok((input, operator))
        }
        fn prefixed_expression<'a>(
            local_variables: &'a HashSet<&'a str>,
        ) -> impl Fn(Source) -> Res<Source, Expression> + 'a {
            |input| {
                let (input, (prefix_operator, expression)) =
                    tuple((prefix_operator, expression(local_variables)))(input)?;

                Ok((
                    input,
                    Expression::Prefixed(prefix_operator, Box::new(expression)),
                ))
            }
        }

        alt((
            calc(local_variables),
            field_or_identifier(local_variables),
            prefixed_expression(local_variables),
        ))(input)
    }
}
