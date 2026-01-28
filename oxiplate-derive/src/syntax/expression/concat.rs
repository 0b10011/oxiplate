use quote::{quote, quote_spanned};

use super::Res;
use crate::syntax::expression::{Expression, ExpressionAccess, expression};
use crate::syntax::parser::{Parser as _, context, cut, fail, many1, take};
use crate::tokenizer::parser::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

#[derive(Debug)]
pub(crate) struct Concat<'a> {
    pub expressions: Vec<ExpressionAccess<'a>>,
    source: Source<'a>,
}

impl<'a> Concat<'a> {
    pub(super) fn to_tokens(&self, state: &State) -> BuiltTokens {
        {
            let mut format_tokens = vec![];
            let mut argument_tokens = vec![];
            let mut estimated_length = 0;
            for expression in &self.expressions {
                match expression {
                    ExpressionAccess {
                        expression: Expression::String(string),
                        fields,
                    } if fields.is_empty() => {
                        estimated_length += string.as_str().len();
                        let string =
                            syn::LitStr::new(string.as_str(), string.source().span_token());
                        format_tokens.push(quote! { #string });
                    }
                    _ => {
                        let span = expression.source().span_token();
                        format_tokens.push(quote_spanned! {span=> "{}" });
                        let (expression, expression_length) = expression.to_tokens(state);
                        estimated_length += expression_length;
                        argument_tokens.push(quote!(#expression));
                    }
                }
            }

            let span = self.source.span_token();
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
    pub(super) fn parser(allow_concat: bool) -> impl Fn(TokenSlice<'a>) -> Res<'a, Expression<'a>> {
        move |tokens| {
            if !allow_concat {
                return context("Concat not allowed in this context", fail()).parse(tokens);
            }
            let (tokens, (left, concats)) = (
                expression(false, false),
                many1((
                    take(TokenKind::Tilde),
                    cut("Expected an expression", expression(true, false)),
                )),
            )
                .parse(tokens)?;

            let mut expressions = Vec::with_capacity(concats.len() + 1);
            expressions.push(left);
            let mut source: Source<'a> = expressions[0].source();

            for (tilde, expression) in concats {
                source = source
                    .merge(tilde.source(), "Tilde should follow leading whitespace")
                    .merge(
                        &expression.source(),
                        "Expression should follow trailing whitespace",
                    );

                expressions.push(expression);
            }

            Ok((
                tokens,
                Expression::Concat(Concat {
                    expressions,
                    source,
                }),
            ))
        }
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}
