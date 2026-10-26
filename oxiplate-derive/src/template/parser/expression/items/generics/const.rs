use super::GenericParam;
use crate::template::parser::expression::literal::PossiblyNegativeLiteralExpression;
use crate::template::parser::expression::types::Type;
use crate::template::parser::expression::{Identifier, Keyword, KeywordParser};
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-ConstParam>
#[derive(Debug)]
pub(crate) struct ConstParam<'a> {
    r#const: Keyword<'a>,
    identifier: Identifier<'a>,
    colon: Source<'a>,
    r#type: Box<Type<'a>>,
    assignment: Option<(Source<'a>, ConstParamValue<'a>)>,
}

impl<'a> ConstParam<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-ConstParam>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (r#const, identifier, colon, r#type, assignment)) = (
            KeywordParser::new("const"),
            cut("Expected an identifier after `const`", Identifier::parse),
            cut(
                "Expected `:` after the identifier",
                into(take(TokenKind::Colon)),
            ),
            cut("Expected a type after `:`", Type::parse),
            ignore_recoverable_errors((
                into(take(TokenKind::Equal)),
                cut(
                    "Expected an identifier, literal expression, or block expression",
                    ConstParamValue::parse,
                ),
            )),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self {
                r#const,
                identifier,
                colon,
                r#type: Box::new(r#type),
                assignment,
            },
        ))
    }

    pub fn source(&self) -> Source<'a> {
        let mut source = self
            .r#const
            .source()
            .to_owned()
            .merge(
                self.identifier.source(),
                "Identifier expected after `const`",
            )
            .merge(&self.colon, "`:` expected after identifier")
            .merge(&self.r#type.source(), "Type expected after `:`");

        if let Some((equal, value)) = &self.assignment {
            source = source
                .merge(equal, "`=` expected after type")
                .merge(&value.source(), "Value expected after `=`");
        }

        source
    }
}

impl<'a> ToTokensWithState<'a> for ConstParam<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        self.r#const.to_tokens(&mut tokens);

        self.identifier.to_tokens(&mut tokens);

        let span = self.colon.span_token();
        tokens.append_all(quote_spanned! {span=> : });

        tokens.append_all(self.r#type.to_tokens_with_state(state).0);

        if let Some((equal, value)) = &self.assignment {
            let span = equal.span_token();
            tokens.append_all(quote_spanned! {span=> = });

            tokens.append_all(value.to_tokens_with_state(state).0);
        }

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<ConstParam<'a>> for GenericParam<'a> {
    fn from(value: ConstParam<'a>) -> Self {
        Self::Const(value)
    }
}

/// Value portion of `ConstParam`.
/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-ConstParam>
#[derive(Debug)]
pub(crate) enum ConstParamValue<'a> {
    /// See: <https://doc.rust-lang.org/reference/expressions/block-expr.html#railroad-BlockExpression>
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "`BlockExpression` isn't currently supported")
    )]
    BlockExpression,

    /// See: <https://doc.rust-lang.org/reference/identifiers.html#railroad-IDENTIFIER>
    Identifier(Identifier<'a>),

    /// `LiteralExpression` with optional leading `-`.
    /// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
    Literal(PossiblyNegativeLiteralExpression<'a>),
}

impl<'a> ConstParamValue<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-ConstParam>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, value): (_, Self) = alt((
            into(Identifier::parse),
            into(PossiblyNegativeLiteralExpression::parse),
        ))
        .parse(tokens)?;

        Ok((tokens, value))
    }

    pub fn source(&self) -> Source<'a> {
        match self {
            Self::BlockExpression => {
                unreachable!("`ConstParamValue::BlockExpression` isn't currently supported")
            }
            Self::Identifier(identifier) => identifier.source().to_owned(),
            Self::Literal(literal) => literal.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for ConstParamValue<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::BlockExpression => {
                unreachable!("`ConstParamValue::BlockExpression` isn't currently supported")
            }
            Self::Identifier(identifier) => {
                let mut tokens = TokenStream::new();
                identifier.to_tokens(&mut tokens);
                (tokens, EstimatedLength::new(0))
            }
            Self::Literal(literal) => literal.to_tokens_with_state(state),
        }
    }
}

impl<'a> From<Identifier<'a>> for ConstParamValue<'a> {
    fn from(value: Identifier<'a>) -> Self {
        Self::Identifier(value)
    }
}

impl<'a> From<PossiblyNegativeLiteralExpression<'a>> for ConstParamValue<'a> {
    fn from(value: PossiblyNegativeLiteralExpression<'a>) -> Self {
        Self::Literal(value)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "internal error: entered unreachable code: `ConstParamValue::BlockExpression` \
                      isn't currently supported"]
    fn block_expression_source() {
        ConstParamValue::BlockExpression.source();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: `ConstParamValue::BlockExpression` \
                      isn't currently supported"]
    fn block_expression_tokens() {
        let _ = ConstParamValue::BlockExpression.to_tokens_with_state(&State::new_for_test());
    }
}
