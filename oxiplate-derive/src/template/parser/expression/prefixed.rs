use super::prefix_operator::PrefixOperator;
use super::{Expression, expression};
use crate::template::parser::prelude::*;

/// Prefixed expression.
#[derive(Debug)]
pub(crate) struct Prefixed<'a> {
    operator: PrefixOperator<'a>,
    pub(super) expression: Box<Expression<'a>>,
}

impl<'a> Prefixed<'a> {
    /// Parses an expression prefixed with an operator.
    pub(in crate::template::parser) fn parser() -> PrefixedParser<'a> {
        PrefixedParser {
            phantom_data: PhantomData,
        }
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> Source<'a> {
        self.operator.source().clone().merge(
            &self.expression.source(),
            "Expression should follow operator",
        )
    }
}

impl<'a> ToTokensWithState<'a> for Prefixed<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let operator = &self.operator;
        let (expression, expression_length) = self.expression.to_tokens_with_state(state);
        (quote! { #operator #expression }, expression_length)
    }
}

impl<'a> From<Prefixed<'a>> for Expression<'a> {
    fn from(value: Prefixed<'a>) -> Self {
        Self::Prefixed(value)
    }
}

pub(in crate::template::parser) struct PrefixedParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for PrefixedParser<'a> {
    type TokenKind = TokenKind;
    type Output = Prefixed<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, operator) = PrefixOperator::parser().parse(tokens)?;

        let (tokens, expression) = if operator.cut_if_not_followed_by_expression() {
            cut(
                "Expected an expression after the operator",
                expression(false),
            )
            .parse(tokens)?
        } else {
            expression(false).parse(tokens)?
        };

        Ok((
            tokens,
            Prefixed {
                operator,
                expression: Box::new(expression),
            },
        ))
    }
}
