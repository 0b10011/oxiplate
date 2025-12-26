use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, fail, opt};
use nom::error::context;
use nom::multi::many1;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::Res;
use crate::syntax::expression::{Expression, ExpressionAccess, expression};
use crate::syntax::template::whitespace;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Concat<'a> {
    pub expressions: Vec<ExpressionAccess<'a>>,
    pub source: Source<'a>,
}

impl Concat<'_> {
    pub(super) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
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
                        let string = syn::LitStr::new(string.as_str(), string.source().span());
                        format_tokens.push(quote! { #string });
                    }
                    _ => {
                        let span = expression.source().span();
                        format_tokens.push(quote_spanned! {span=> "{}" });
                        let (expression, expression_length) = expression.to_tokens(state);
                        estimated_length += expression_length;
                        argument_tokens.push(quote!(#expression));
                    }
                }
            }

            let span = self.source.span();
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
    pub(super) fn parser<'a>(
        allow_concat: bool,
    ) -> impl Fn(Source<'a>) -> Res<Source<'a>, Expression<'a>> {
        move |input| {
            if !allow_concat {
                return fail().parse(input);
            }
            let (input, (left, concats)) = (
                expression(false, false),
                many1((
                    opt(whitespace),
                    tag("~"),
                    opt(whitespace),
                    context("Expected an expression", cut(expression(true, false))),
                )),
            )
                .parse(input)?;

            let mut expressions = Vec::with_capacity(concats.len() + 1);
            expressions.push(left);
            let mut source: Source<'a> = expressions[0].source();

            for (leading_whitespace, tilde, trailing_whitespace, expression) in concats {
                source = source
                    .merge_some(
                        leading_whitespace.as_ref(),
                        "Leading whitespace should follow previous expressions",
                    )
                    .merge(&tilde, "Tilde should follow leading whitespace")
                    .merge_some(
                        trailing_whitespace.as_ref(),
                        "Trailing whitespace should follow tilde",
                    )
                    .merge(
                        &expression.source(),
                        "Expression should follow trailing whitespace",
                    );

                expressions.push(expression);
            }
            Ok((
                input,
                Expression::Concat(Concat {
                    expressions,
                    source,
                }),
            ))
        }
    }
}
