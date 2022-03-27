use super::{Res, Span};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{pair, terminated};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq)]
pub struct Identifier<'a>(pub &'a str);

#[derive(Debug, PartialEq)]
pub enum IdentifierOrFunction<'a> {
    Identifier(Identifier<'a>),
    Function(Identifier<'a>),
}

#[derive(Debug, PartialEq)]
pub struct IdentField<'a>(Vec<Identifier<'a>>, IdentifierOrFunction<'a>);

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Identifier(IdentifierOrFunction<'a>),
    FieldAccess(IdentField<'a>),
}

impl ToTokens for Expression<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Expression::Identifier(identifier) => match &identifier {
                IdentifierOrFunction::Identifier(identifier) => {
                    let identifier = syn::Ident::new(&identifier.0, proc_macro2::Span::call_site());
                    quote! {
                        write!(f, "{}", self.#identifier)?;
                    }
                }
                IdentifierOrFunction::Function(identifier) => {
                    let identifier = syn::Ident::new(&identifier.0, proc_macro2::Span::call_site());
                    quote! {
                        write!(f, "{}", self.#identifier())?;
                    }
                }
            },
            Expression::FieldAccess(identifier) => {
                let mut parents = Vec::with_capacity(identifier.0.len());
                for parent in &identifier.0 {
                    parents.push(syn::Ident::new(&parent.0, proc_macro2::Span::call_site()));
                }
                match &identifier.1 {
                    IdentifierOrFunction::Identifier(identifier) => {
                        let identifier =
                            syn::Ident::new(&identifier.0, proc_macro2::Span::call_site());
                        quote! {
                            write!(f, "{}", self.#(#parents.)*#identifier)?;
                        }
                    }
                    IdentifierOrFunction::Function(identifier) => {
                        let identifier =
                            syn::Ident::new(&identifier.0, proc_macro2::Span::call_site());
                        quote! {
                            write!(f, "{}", self.#(#parents.)*#identifier())?;
                        }
                    }
                }
            }
        });
    }
}

pub(super) fn expression(input: Span) -> Res<&str, Expression> {
    fn field_or_identifier(input: Span) -> Res<&str, Expression> {
        let ident = take_while1(|char| match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
            _ => false,
        });
        let (input, (parsed_parents, (parsed_field, maybe_function))) = pair(
            many0(terminated(&ident, char('.'))),
            pair(&ident, opt(tag("()"))),
        )(input)?;

        let ident = Identifier(parsed_field.fragment());
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
            parents.push(Identifier(parent.fragment()));
        }

        Ok((input, Expression::FieldAccess(IdentField(parents, field))))
    }

    alt((field_or_identifier,))(input)
}
