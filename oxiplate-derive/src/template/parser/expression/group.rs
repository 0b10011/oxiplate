use proc_macro2::TokenStream;
use quote::quote_spanned;

use crate::parser::{Parser as _, context, take};
use crate::template::parser::Res;
use crate::template::parser::expression::{Expression, ExpressionAccess, expression};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, State};

/// A parenthesized group.
/// E.g., `(a + b)`.
#[derive(Debug)]
pub struct Group<'a> {
    /// Expression contained by the parentheses.
    expression: Box<ExpressionAccess<'a>>,

    /// Source for the entire group, including the parentheses.
    source: Source<'a>,
}

impl<'a> Group<'a> {
    /// Parse a parenthesized group.
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (open, expression, close)) = (
            take(TokenKind::OpenParenthese),
            context("Expected an expression", expression(true, true)),
            context(
                "Expected `)` after expression",
                take(TokenKind::CloseParenthese),
            ),
        )
            .parse(tokens)?;

        let source = expression
            .source()
            .merge(close.source(), "`)` expected after expression");

        Ok((
            tokens,
            Self {
                expression: Box::new(expression),
                source: open
                    .source()
                    .clone()
                    .merge(&source, "Expression and `)` expected after `(`"),
            },
        ))
    }

    /// Source for the entire group, including the parentheses.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    /// Build token stream for the group.
    pub fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let (expression, expression_length) = self.expression.to_tokens(state);
        let span = self.source().span_token();
        (quote_spanned! {span=> ( #expression ) }, expression_length)
    }
}

impl<'a> From<Group<'a>> for Expression<'a> {
    fn from(value: Group<'a>) -> Self {
        Expression::Group(value)
    }
}
