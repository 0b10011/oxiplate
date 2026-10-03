use super::{Expression, expression};
use crate::template::parser::prelude::*;

/// Cow prefixed expression (`>expr`).
#[derive(Debug)]
pub(crate) struct Cow<'a> {
    prefix: Source<'a>,
    pub(super) expression: Box<Expression<'a>>,
}

impl<'a> Cow<'a> {
    /// Parses a cow prefixed expression (`>expr`).
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (prefix, expression)) = (
            take(TokenKind::GreaterThan),
            cut("Expected an expression after cow prefix", expression(false)),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Cow {
                prefix: prefix.source().clone(),
                expression: Box::new(expression),
            },
        ))
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> Source<'a> {
        self.prefix.clone().merge(
            &self.expression.source(),
            "Expression should follow whitespace",
        )
    }
}

impl<'a> ToTokensWithState<'a> for Cow<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        #[cfg_attr(not(feature = "_oxiplate"), allow(unused_variables))]
        let (expression, expression_length) = self.expression.to_tokens_with_state(state);
        let span = self.prefix.span_token();

        #[cfg(feature = "_oxiplate")]
        let expression = quote_spanned! {span=>
            ::oxiplate::CowStrWrapper::new((&&::oxiplate::ToCowStrWrapper::new(&(#expression))).to_cow_str())
        };

        #[cfg(not(feature = "_oxiplate"))]
        let expression = quote_spanned! {span=>
            compile_error!("Cow prefix requires the `oxiplate` library due to trait usage")
        };

        (expression, expression_length)
    }
}

impl<'a> From<Cow<'a>> for Expression<'a> {
    fn from(value: Cow<'a>) -> Self {
        Self::Cow(value)
    }
}
