use super::{Res, Span};
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Identifier(&'a str),
}

impl ToTokens for Expression<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Expression::Identifier(identifier) => {
                let identifier = syn::Ident::new(&identifier, proc_macro2::Span::call_site());
                quote! {write!(f, "{}", self.#identifier)?;}
            }
        });
    }
}

pub(super) fn expression(input: Span) -> Res<&str, Expression> {
    fn identifier(input: Span) -> Res<&str, Expression> {
        let (input, output) = take_while1(|char| match char {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
            _ => false,
        })(input)?;

        Ok((input, Expression::Identifier(output.fragment())))
    }

    alt((identifier,))(input)
}
