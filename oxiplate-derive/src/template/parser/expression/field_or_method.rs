use proc_macro2::TokenStream;
use quote::{TokenStreamExt as _, quote, quote_spanned};
use syn::token::Dot;

use crate::parser::{Parser as _, context, fail, many1, opt, take};
use crate::template::parser::Res;
use crate::template::parser::expression::arguments::arguments;
use crate::template::parser::expression::ident::IdentifierOrFunction;
use crate::template::parser::expression::{Expression, Identifier, expression};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, State};

/// A field or method.
#[derive(Debug)]
pub struct FieldOrMethod<'a> {
    expression: Box<Expression<'a>>,
    fields: Vec<Field<'a>>,
}

impl<'a> FieldOrMethod<'a> {
    /// Parse a field or method.
    pub fn parser(allow_generic_nesting: bool) -> impl Fn(TokenSlice<'a>) -> Res<'a, Self> + 'a {
        move |tokens| {
            if !allow_generic_nesting {
                return context(
                    "Generic nesting of field or method not allowed in this context",
                    fail(),
                )
                .parse(tokens);
            }

            let (tokens, (expression, fields)) =
                (expression(false, true), many1(Field::parse)).parse(tokens)?;

            Ok((
                tokens,
                Self {
                    expression: Box::new(expression),
                    fields,
                },
            ))
        }
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
            tokens.append_all(field.to_tokens(state));
        }
        (tokens, estimated_length)
    }
}

impl<'a> From<FieldOrMethod<'a>> for Expression<'a> {
    fn from(value: FieldOrMethod<'a>) -> Self {
        Expression::FieldOrMethod(value)
    }
}

#[derive(Debug)]
pub(crate) struct Field<'a> {
    dot: Source<'a>,
    ident_or_fn: IdentifierOrFunction<'a>,
}

impl<'a> Field<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (dot, ident, arguments)) =
            (take(TokenKind::Period), Identifier::parse, opt(arguments)).parse(tokens)?;

        let ident_or_fn = if let Some(arguments) = arguments {
            IdentifierOrFunction::Function(ident, arguments)
        } else {
            IdentifierOrFunction::Identifier(ident)
        };

        Ok((
            tokens,
            Field {
                dot: dot.source().clone(),
                ident_or_fn,
            },
        ))
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let span = self.dot.span_token();
        let dot = syn::parse2::<Dot>(quote_spanned! {span=> . })
            .expect("Dot should be able to be parsed properly here");

        let ident_or_fn = &self.ident_or_fn.to_tokens(state);
        quote! { #dot #ident_or_fn }
    }

    /// Get the `Source` for the field.
    pub(crate) fn source(&self) -> Source<'a> {
        self.dot.clone().merge(
            &self.ident_or_fn.source(),
            "Field or method name should immediately follow the dot",
        )
    }
}
