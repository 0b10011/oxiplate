use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::error::context;
use proc_macro2::TokenStream;
use quote::quote_spanned;

use crate::syntax::Res;
use crate::syntax::expression::{Expression, ExpressionAccess, expression};
use crate::syntax::template::whitespace;
use crate::{Source, State};

/// A parenthesized group.
/// E.g., `(a + b)`.
#[derive(Debug, PartialEq, Eq)]
pub struct Group<'a> {
    /// Expression contained by the parentheses.
    expression: Box<ExpressionAccess<'a>>,

    /// Source for the entire group, including the parentheses.
    source: Source<'a>,
}

impl<'a> Group<'a> {
    /// Parse a parenthesized group.
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (open, leading_whitespace, expression, trailing_whitespace, close)) = (
            tag("("),
            opt(whitespace),
            context("Expected an expression", expression(true, true)),
            opt(whitespace),
            context("Expected `)` after expression", tag(")")),
        )
            .parse(input)?;

        let source = open
            .merge_some(leading_whitespace.as_ref(), "Whitespace expected after `(`")
            .merge(&expression.source(), "Expression expected after whitespace")
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after expression",
            )
            .merge(&close, "`)` expected after whitespace");

        Ok((
            input,
            Self {
                expression: Box::new(expression),
                source,
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
        let span = self.source.span();
        (quote_spanned! {span=> ( #expression ) }, expression_length)
    }
}

impl<'a> From<Group<'a>> for Expression<'a> {
    fn from(value: Group<'a>) -> Self {
        Expression::Group(value)
    }
}
