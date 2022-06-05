use super::{Res, Span, template::whitespace};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{pair, terminated, tuple};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

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
pub struct Identifier<'a>(pub &'a str);

#[derive(Debug, PartialEq, Eq)]
pub enum IdentifierOrFunction<'a> {
    Identifier(Identifier<'a>),
    Function(Identifier<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdentField<'a>(Vec<Identifier<'a>>, IdentifierOrFunction<'a>);

#[derive(Debug, PartialEq, Eq)]
pub enum Expression<'a> {
    Identifier(IdentifierOrFunction<'a>),
    FieldAccess(IdentField<'a>),
    // Group(Box<Expression<'a>>),
    Calc(Box<Expression<'a>>, Operator, Box<Expression<'a>>),
}

impl ToTokens for Expression<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Expression::Identifier(identifier) => match &identifier {
                IdentifierOrFunction::Identifier(identifier) => {
                    let identifier = syn::Ident::new(identifier.0, proc_macro2::Span::call_site());
                    quote! { self.#identifier }
                }
                IdentifierOrFunction::Function(identifier) => {
                    let identifier = syn::Ident::new(identifier.0, proc_macro2::Span::call_site());
                    quote! { self.#identifier() }
                }
            },
            Expression::FieldAccess(identifier) => {
                let mut parents = Vec::with_capacity(identifier.0.len());
                for parent in &identifier.0 {
                    parents.push(syn::Ident::new(parent.0, proc_macro2::Span::call_site()));
                }
                match &identifier.1 {
                    IdentifierOrFunction::Identifier(identifier) => {
                        let identifier =
                            syn::Ident::new(identifier.0, proc_macro2::Span::call_site());
                        quote! { self.#(#parents.)*#identifier }
                    }
                    IdentifierOrFunction::Function(identifier) => {
                        let identifier =
                            syn::Ident::new(identifier.0, proc_macro2::Span::call_site());
                        quote! { self.#(#parents.)*#identifier() }
                    }
                }
            }
            Expression::Calc(left, operator, right) => quote!(#left #operator #right),
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

pub(super) fn expression(input: Span) -> Res<&str, Expression> {
    fn field_or_identifier(input: Span) -> Res<&str, Expression> {
        let ident = take_while1(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
        let (input, (parsed_parents, (parsed_field, maybe_function))) = pair(
            many0(terminated(&ident, char('.'))),
            pair(&ident, opt(tag("()"))),
        )(input)?;

        let ident = Identifier(parsed_field);
        let field = if maybe_function.is_some() {
            IdentifierOrFunction::Function(ident)
        } else {
            IdentifierOrFunction::Identifier(ident)
        };

        if parsed_parents.is_empty() {
            return Ok((input, Expression::Identifier(field)));
        }

        let mut parents = Vec::with_capacity(parsed_parents.len());
        for parent in parsed_parents {
            parents.push(Identifier(parent));
        }

        Ok((input, Expression::FieldAccess(IdentField(parents, field))))
    }
    fn operator(input: Span) -> Res<&str, Operator> {
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

        let operator = match operator {
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
    fn calc(input: Span) -> Res<&str, Expression> {
        let (input, (left, _leading_whitespace, operator, _trailing_whitespace, right)) = tuple((field_or_identifier, opt(whitespace), operator, opt(whitespace), field_or_identifier))(input)?;
        Ok((input, Expression::Calc(Box::new(left), operator, Box::new(right))))
    }

    alt((calc, field_or_identifier))(input)
}
