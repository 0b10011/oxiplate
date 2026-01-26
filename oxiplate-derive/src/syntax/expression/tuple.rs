use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::{Expression, Res, expression};
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::parser::{Parser as _, context, many1, opt, take};
use crate::tokenizer::{Token, TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

#[derive(Debug)]
pub struct Tuple<'a> {
    items: Vec<TupleItem<'a>>,
    source: Source<'a>,
}

impl<'a> Tuple<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Expression<'a>> {
        let (tokens, (open, leading_items, trailing_item, close)) = (
            take(TokenKind::OpenParenthese),
            context(
                "Expected at least one tuple item",
                many1(TupleItem::parse(true)),
            ),
            // Last tuple item doesn't need a comma after it,
            // but the first one does.
            opt(TupleItem::parse(false)),
            context(
                "Expected `)` after tuple item",
                take(TokenKind::CloseParenthese),
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

        source = source.merge(close.source(), "`)` should follow item");

        Ok((tokens, Expression::Tuple(Tuple { items, source })))
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
        (quote_spanned! {span=> ( #(#items)* ) }, expression_length)
    }
}

#[derive(Debug)]
struct TupleItem<'a> {
    expression: ExpressionAccess<'a>,
    comma: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> TupleItem<'a> {
    pub fn parse(require_comma: bool) -> impl Fn(TokenSlice<'a>) -> Res<'a, TupleItem<'a>> {
        move |tokens| {
            let (tokens, (expression, comma)) = if require_comma {
                let (tokens, (expression, comma)) = (
                    context("Expected an expression", expression(true, true)),
                    context("Expected `,` after expression", take(TokenKind::Comma)),
                )
                    .parse(tokens)?;

                (tokens, (expression, Some(comma)))
            } else {
                (
                    context("Expected an expression", expression(true, true)),
                    opt(take(TokenKind::Comma)),
                )
                    .parse(tokens)?
            };

            let source = expression
                .source()
                .merge_some(comma.map(Token::source), "Comma expected after expression");

            Ok((
                tokens,
                TupleItem {
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
