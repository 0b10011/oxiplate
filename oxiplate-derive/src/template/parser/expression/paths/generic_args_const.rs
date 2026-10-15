use super::args::GenericArg;
use super::simple_path_segment::SimplePathSegment;
use crate::template::parser::expression::literal::PossiblyNegativeLiteralExpression;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsConst>
#[derive(Debug)]
pub(in crate::template::parser) enum GenericArgsConst<'a> {
    /// See: <https://doc.rust-lang.org/reference/expressions/block-expr.html#railroad-BlockExpression>
    #[expect(dead_code, reason = "`BlockExpression` isn't currently supported")]
    BlockExpression,

    /// `LiteralExpression` with possible leading `-`.
    /// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
    LiteralExpression(PossiblyNegativeLiteralExpression<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-SimplePathSegment>
    SimplePathSegment(SimplePathSegment<'a>),
}

impl<'a> From<PossiblyNegativeLiteralExpression<'a>> for GenericArgsConst<'a> {
    fn from(value: PossiblyNegativeLiteralExpression<'a>) -> Self {
        Self::LiteralExpression(value)
    }
}

impl<'a> From<SimplePathSegment<'a>> for GenericArgsConst<'a> {
    fn from(value: SimplePathSegment<'a>) -> Self {
        Self::SimplePathSegment(value)
    }
}

impl<'a> GenericArgsConst<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsConst>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, generic_args_const) = alt((
            into(PossiblyNegativeLiteralExpression::parse),
            into(SimplePathSegment::parse),
        ))
        .parse(tokens)?;

        Ok((tokens, generic_args_const))
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        match self {
            Self::BlockExpression => {
                unreachable!("`GenericArgsConst::BlockExpression` isn't currently supported")
            }
            Self::LiteralExpression(literal) => literal.source(),
            Self::SimplePathSegment(simple_path_segment) => simple_path_segment.source().to_owned(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for GenericArgsConst<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::BlockExpression => {
                unreachable!("`GenericArgsConst::BlockExpression` isn't currently supported")
            }
            Self::LiteralExpression(literal) => literal.to_tokens_with_state(state),
            Self::SimplePathSegment(simple_path_segment) => {
                simple_path_segment.to_tokens_with_state(state)
            }
        }
    }
}

impl<'a> From<GenericArgsConst<'a>> for GenericArg<'a> {
    fn from(value: GenericArgsConst<'a>) -> Self {
        Self::Const(value)
    }
}
