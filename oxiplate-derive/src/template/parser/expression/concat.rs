use quote::{quote, quote_spanned};

use super::Res;
use crate::parser::{Parser as _, cut, many1, take};
use crate::template::parser::expression::{Expression, NestedExpression, expression};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

#[derive(Debug)]
pub(crate) struct Concat<'a> {
    pub(super) first_expression: Box<Expression<'a>>,
    pub(super) additional_expressions: Vec<(Source<'a>, Expression<'a>)>,
}

impl<'a> Concat<'a> {
    pub(super) fn to_tokens(&self, state: &State) -> BuiltTokens {
        {
            let mut format_tokens = vec![];
            let mut argument_tokens = vec![];
            let mut estimated_length = 0;
            let mut expressions = Vec::with_capacity(self.additional_expressions.len() + 1);
            expressions.push(self.first_expression.as_ref());
            for (_tilde, expression) in &self.additional_expressions {
                expressions.push(expression);
            }

            for expression in expressions {
                if let Expression::String(string) = expression {
                    estimated_length += string.as_str().len();
                    let string = syn::LitStr::new(string.as_str(), string.source().span_token());
                    format_tokens.push(quote! { #string });
                } else {
                    let span = expression.source().span_token();
                    format_tokens.push(quote_spanned! {span=> "{}" });
                    let (expression, expression_length) = expression.to_tokens(state);
                    estimated_length += expression_length;
                    argument_tokens.push(quote!(#expression));
                }
            }

            let span = self.source().span_token();
            let format_concat_tokens = quote_spanned! {span=> concat!(#(#format_tokens),*) };
            format_tokens.clear();

            if argument_tokens.is_empty() {
                (format_concat_tokens, estimated_length)
            } else {
                (
                    quote_spanned! {span=> format!(#format_concat_tokens, #(#argument_tokens),*) },
                    estimated_length,
                )
            }
        }
    }

    /// Parser for concat expressions.
    pub(super) fn parser(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, concats) = many1((
            take(TokenKind::Tilde),
            cut("Expected an expression", expression(false)),
        ))
        .parse(tokens)?;

        let callback = Box::new(|left: Expression<'a>| {
            Expression::Concat(Concat {
                first_expression: Box::new(left),
                additional_expressions: concats
                    .into_iter()
                    .map(|(tilde, expression)| (tilde.source().clone(), expression))
                    .collect(),
            })
        });

        Ok((tokens, callback))
    }

    pub fn source(&self) -> Source<'a> {
        let mut source: Source<'a> = self.first_expression.source();

        for (tilde, expression) in &self.additional_expressions {
            source = source
                .merge(tilde, "Tilde should follow leading whitespace")
                .merge(
                    &expression.source(),
                    "Expression should follow trailing whitespace",
                );
        }

        source
    }
}
