use proc_macro2::TokenStream;
use quote::{TokenStreamExt as _, quote, quote_spanned};
use syn::token::Dot;

use crate::parser::{Parser as _, many1, take};
use crate::template::parser::Res;
use crate::template::parser::expression::{Expression, Identifier, NestedExpression};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, State};

/// A field or method.
#[derive(Debug)]
pub struct Fields<'a> {
    pub(super) expression: Box<Expression<'a>>,
    fields: Vec<Field<'a>>,
}

impl<'a> Fields<'a> {
    /// Parse a field or method.
    pub fn parser(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, fields) = many1(Field::parse).parse(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Self {
                    expression: Box::new(expression),
                    fields,
                }
                .into()
            }),
        ))
    }

    /// Source for the entire group, including the parentheses.
    pub fn source(&self) -> Source<'a> {
        let mut source: Source<'a> = self.expression.source();
        for field in &self.fields {
            source = source.merge(
                &field.source(),
                "Field source should be immediately after the rest of the expression",
            );
        }
        source
    }

    /// Build token stream for the group.
    pub fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let (expression, estimated_length) = self.expression.to_tokens(state);
        tokens.append_all(expression);
        for field in &self.fields {
            tokens.append_all(field.to_tokens());
        }
        (tokens, estimated_length)
    }
}

impl<'a> From<Fields<'a>> for Expression<'a> {
    fn from(value: Fields<'a>) -> Self {
        Expression::Fields(value)
    }
}

#[derive(Debug)]
pub(crate) struct Field<'a> {
    dot: Source<'a>,
    ident: Identifier<'a>,
}

impl<'a> Field<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (dot, ident)) = (take(TokenKind::Period), Identifier::parse).parse(tokens)?;

        Ok((
            tokens,
            Field {
                dot: dot.source().clone(),
                ident,
            },
        ))
    }

    pub fn to_tokens(&self) -> TokenStream {
        let span = self.dot.span_token();
        let dot = syn::parse2::<Dot>(quote_spanned! {span=> . })
            .expect("Dot should be able to be parsed properly here");

        let ident = &self.ident;
        quote! { #dot #ident }
    }

    /// Get the `Source` for the field.
    pub(crate) fn source(&self) -> Source<'a> {
        self.dot.clone().merge(
            self.ident.source(),
            "Field or method name should immediately follow the dot",
        )
    }
}
