use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::{Expression, Res, expression};
use crate::parser::{Parser as _, context, cut, ignore_recoverable_errors, many0, take};
use crate::template::tokenizer::{Token, TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

#[derive(Debug)]
pub struct Array<'a> {
    items: Vec<ArrayItem<'a>>,
    length_expression: Option<(Source<'a>, Box<Expression<'a>>)>,
    source: Source<'a>,
}

impl<'a> Array<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Expression<'a>> {
        let (tokens, (open, leading_items, trailing_item, length_expression, close)) = (
            take(TokenKind::OpenBracket),
            many0(ArrayItem::parse(true)),
            // Last array item doesn't need a comma after it,
            // but the first one does.
            ignore_recoverable_errors(ArrayItem::parse(false)),
            ignore_recoverable_errors((
                take(TokenKind::Semicolon),
                cut(
                    "Expression to specify array length expected after `;`",
                    expression(true),
                ),
            )),
            cut(
                "Expected `]` at end of array",
                take(TokenKind::CloseBracket),
            ),
        )
            .parse(tokens)?;

        let mut source = open.source().clone();

        let mut items = vec![];
        for item in leading_items {
            source = source.merge(&item.source, "Item should follow previous");
            items.push(item);
        }

        if let Some(trailing_item) = trailing_item {
            source = source.merge(&trailing_item.source, "Item should follow previous");
            items.push(trailing_item);
        }

        if let Some((semicolon, ref length_expression)) = length_expression {
            source = source
                .merge(semicolon.source(), "`;` expected after array items")
                .merge(&length_expression.source(), "Expression expected after `;`");
        }

        source = source.merge(close.source(), "`]` should follow item");

        Ok((
            tokens,
            Expression::Array(Array {
                items,
                length_expression: length_expression.map(|(semicolon, expression)| {
                    (semicolon.source().clone(), Box::new(expression))
                }),
                source,
            }),
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn to_tokens(&self, state: &State) -> BuiltTokens {
        let mut items = vec![];
        let span = self.source.span_token();
        let mut expression_length = usize::MAX;
        for item in &self.items {
            let (item, item_length) = item.to_tokens(state);
            items.push(item);
            expression_length = expression_length.min(item_length);
        }

        let length_expression =
            if let Some((ref semicolon, ref expression)) = self.length_expression {
                let (expression, _expression_length) = expression.to_tokens(state);
                let span = semicolon.span_token();

                quote_spanned! {span=>
                    ; #expression
                }
            } else {
                quote! {}
            };

        (
            quote_spanned! {span=> [ #(#items)* #length_expression ] },
            expression_length,
        )
    }
}

#[derive(Debug)]
struct ArrayItem<'a> {
    expression: Expression<'a>,
    comma: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> ArrayItem<'a> {
    pub fn parse(require_comma: bool) -> impl Fn(TokenSlice<'a>) -> Res<'a, ArrayItem<'a>> {
        move |tokens| {
            let (tokens, (expression, comma)) = if require_comma {
                let (tokens, (expression, comma)) = (
                    context("Expected an expression", expression(true)),
                    context("Expected `,` after expression", take(TokenKind::Comma)),
                )
                    .parse(tokens)?;

                (tokens, (expression, Some(comma)))
            } else {
                (
                    context("Expected an expression", expression(true)),
                    ignore_recoverable_errors(take(TokenKind::Comma)),
                )
                    .parse(tokens)?
            };

            let source = expression
                .source()
                .merge_some(comma.map(Token::source), "Comma expected after expression");

            Ok((
                tokens,
                ArrayItem {
                    expression,
                    comma: comma.map(|token| token.source().clone()),
                    source,
                },
            ))
        }
    }

    pub fn to_tokens(&self, state: &State) -> BuiltTokens {
        let (expression, expression_length) = self.expression.to_tokens(state);
        let comma = self.comma.clone().map_or_else(TokenStream::new, |comma| {
            let span = comma.span_token();
            quote_spanned! {span=> , }
        });
        (quote! { #expression #comma }, expression_length)
    }
}
