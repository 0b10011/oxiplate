use super::{Expression, expression};
use crate::template::parser::prelude::*;
use crate::template::tokenizer::Token;

#[derive(Debug)]
pub(crate) struct Array<'a> {
    items: Vec<ArrayItem<'a>>,
    length_expression: Option<(Source<'a>, Box<Expression<'a>>)>,
    source: Source<'a>,
}

impl<'a> Array<'a> {
    pub(in crate::template::parser) fn parser() -> ArrayParser<'a> {
        ArrayParser {
            phantom_data: PhantomData,
        }
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for Array<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut items = vec![];
        let span = self.source.span_token();
        let mut expression_length = EstimatedLength::new(usize::MAX);
        for item in &self.items {
            let (item, item_length) = item.to_tokens_with_state(state);
            items.push(item);
            expression_length = expression_length.min(item_length);
        }

        let length_expression =
            if let Some((ref semicolon, ref expression)) = self.length_expression {
                let (expression, _expression_length) = expression.to_tokens_with_state(state);
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

impl<'a> From<Array<'a>> for Expression<'a> {
    fn from(value: Array<'a>) -> Self {
        Expression::Array(value)
    }
}

pub(in crate::template::parser) struct ArrayParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ArrayParser<'a> {
    type TokenKind = TokenKind;
    type Output = Array<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
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
            Array {
                items,
                length_expression: length_expression.map(|(semicolon, expression)| {
                    (semicolon.source().clone(), Box::new(expression))
                }),
                source,
            },
        ))
    }
}

#[derive(Debug)]
struct ArrayItem<'a> {
    expression: Expression<'a>,
    comma: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> ArrayItem<'a> {
    pub(in crate::template::parser) fn parse(require_comma: bool) -> ArrayItemParser<'a> {
        ArrayItemParser {
            require_comma,
            phantom_data: PhantomData,
        }
    }
}

impl<'a> ToTokensWithState<'a> for ArrayItem<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let (expression, expression_length) = self.expression.to_tokens_with_state(state);
        let comma = self.comma.clone().map_or_else(TokenStream::new, |comma| {
            let span = comma.span_token();
            quote_spanned! {span=> , }
        });
        (quote! { #expression #comma }, expression_length)
    }
}

pub(in crate::template::parser) struct ArrayItemParser<'a> {
    require_comma: bool,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ArrayItemParser<'a> {
    type TokenKind = TokenKind;
    type Output = ArrayItem<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (expression, comma)) = if self.require_comma {
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
