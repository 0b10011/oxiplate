use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many1;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::{Expression, Res, expression};
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::template::whitespace;
use crate::{BuiltTokens, Source, State};

#[derive(Debug, PartialEq, Eq)]
pub struct Tuple<'a> {
    items: Vec<TupleItem<'a>>,
    source: Source<'a>,
}

impl<'a> Tuple<'a> {
    pub fn parse(input: Source) -> Res<Source, Expression> {
        let (
            input,
            (open, leading_whitespace, leading_items, trailing_item, trailing_whitespace, close),
        ) = (
            tag("("),
            opt(whitespace),
            cut(many1((TupleItem::parse(true), opt(whitespace)))),
            // Last tuple item doesn't need a comma after it,
            // but the first one does.
            opt(TupleItem::parse(false)),
            opt(whitespace),
            cut(context("Expected `)` after expression", tag(")"))),
        )
            .parse(input)?;

        let mut source =
            open.merge_some(leading_whitespace.as_ref(), "Whitespace should follow `(`");

        let mut items = vec![];
        for (item, whitespace) in leading_items {
            source = source
                .merge(&item.source, "Item should follow whitespace")
                .merge_some(whitespace.as_ref(), "Whitespace should follow item");
            items.push(item);
        }

        if let Some(trailing_item) = trailing_item {
            source = source
                .merge(&trailing_item.source, "Item should follow whitespace")
                .merge_some(
                    trailing_whitespace.as_ref(),
                    "Whitespace should follow item",
                );
            items.push(trailing_item);
        }

        source = source.merge(&close, "`)` should follow whitespace");

        Ok((input, Expression::Tuple(Tuple { items, source })))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn to_tokens(&self, state: &State) -> BuiltTokens {
        let mut items = vec![];
        let span = self.source.span();
        let mut expression_length = usize::MAX;
        for item in &self.items {
            let (item, item_length) = item.to_tokens(state);
            items.push(item);
            expression_length = expression_length.min(item_length);
        }
        (quote_spanned! {span=> ( #(#items)* ) }, expression_length)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TupleItem<'a> {
    expression: ExpressionAccess<'a>,
    comma: Option<Source<'a>>,
    source: Source<'a>,
}

impl TupleItem<'_> {
    pub fn parse(require_comma: bool) -> impl Fn(Source) -> Res<Source, TupleItem> {
        move |input| {
            let (input, (expression, whitespace, comma)) = if require_comma {
                let (input, (expression, whitespace, comma)) = (
                    context("Expected an expression", expression(true, true)),
                    opt(whitespace),
                    context("Expected `,` after expression", tag(",")),
                )
                    .parse(input)?;

                (input, (expression, whitespace, Some(comma)))
            } else {
                (
                    context("Expected an expression", expression(true, true)),
                    opt(whitespace),
                    opt(tag(",")),
                )
                    .parse(input)?
            };

            let source = expression
                .source()
                .merge_some(whitespace.as_ref(), "Whitespace expected after expression")
                .merge_some(comma.as_ref(), "Comma expected after whitespace");

            Ok((
                input,
                TupleItem {
                    expression,
                    comma,
                    source,
                },
            ))
        }
    }

    pub fn to_tokens(&self, state: &State) -> BuiltTokens {
        let (expression, expression_length) = self.expression.to_tokens(state);
        let comma = self.comma.clone().map_or_else(TokenStream::new, |comma| {
            let span = comma.span();
            quote_spanned! {span=> , }
        });
        (quote! { #expression #comma }, expression_length)
    }
}
