use super::args::GenericArg;
use super::simple_path_segment::SimplePathSegment;
use crate::template::parser::expression::literal::PossiblyNegativeLiteralExpression;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsConst>
#[derive(Debug)]
pub(in crate::template::parser) enum GenericArgsConst<'a> {
    /// See: <https://doc.rust-lang.org/reference/expressions/block-expr.html#railroad-BlockExpression>
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "`BlockExpression` isn't currently supported")
    )]
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod test {
    use super::*;
    use crate::source::test_source;
    use crate::template::tokenizer::Token;
    use crate::tokenizer::Eof;

    #[test]
    #[should_panic = "internal error: entered unreachable code: \
                      `GenericArgsConst::BlockExpression` isn't currently supported"]
    fn block_expression_source() {
        let _ = GenericArgsConst::BlockExpression.source();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: \
                      `GenericArgsConst::BlockExpression` isn't currently supported"]
    fn block_expression_tokens() {
        let _ = GenericArgsConst::BlockExpression.to_tokens_with_state(&State::new_for_test());
    }

    #[test]
    fn simple_path_segment_ident() {
        test_source!(source = "foo");
        let source_backup = source.clone();

        let token_slice = [Ok(Token::new(TokenKind::Ident, &source, None))];
        let eof = Eof::for_test(source);
        let (_, value @ GenericArgsConst::SimplePathSegment(SimplePathSegment::Identifier(_))) =
            GenericArgsConst::parse
                .parse(TokenSlice::new(&token_slice, &eof))
                .unwrap()
        else {
            panic!("Unexpected value parsed");
        };

        assert_eq!(
            value.source().as_str(),
            source_backup.as_str(),
            "Source did not match"
        );
        assert_eq!(
            format!("{:?}", value.to_tokens_with_state(&State::new_for_test()).0),
            format!("{:?}", quote! { foo }),
            "Tokens did not match"
        );
    }

    #[test]
    fn simple_path_segment_super() {
        test_source!(source = "super");
        let source_backup = source.clone();

        let token_slice = [Ok(Token::new(TokenKind::Super, &source, None))];
        let eof = Eof::for_test(source);
        let (_, value @ GenericArgsConst::SimplePathSegment(SimplePathSegment::Super(_))) =
            GenericArgsConst::parse
                .parse(TokenSlice::new(&token_slice, &eof))
                .unwrap()
        else {
            panic!("Unexpected value parsed");
        };

        assert_eq!(
            value.source().as_str(),
            source_backup.as_str(),
            "Source did not match"
        );
        assert_eq!(
            format!("{:?}", value.to_tokens_with_state(&State::new_for_test()).0),
            format!("{:?}", quote! { super }),
            "Tokens did not match"
        );
    }

    #[test]
    fn simple_path_segment_self() {
        test_source!(source = "self");
        let source_backup = source.clone();

        let token_slice = [Ok(Token::new(TokenKind::SelfCurrentModule, &source, None))];
        let eof = Eof::for_test(source);
        let (
            _,
            value @ GenericArgsConst::SimplePathSegment(SimplePathSegment::SelfCurrentModule(_)),
        ) = GenericArgsConst::parse
            .parse(TokenSlice::new(&token_slice, &eof))
            .unwrap()
        else {
            panic!("Unexpected value parsed");
        };

        assert_eq!(
            value.source().as_str(),
            source_backup.as_str(),
            "Source did not match"
        );
        assert_eq!(
            format!("{:?}", value.to_tokens_with_state(&State::new_for_test()).0),
            format!("{:?}", quote! { self }),
            "Tokens did not match"
        );
    }

    #[test]
    fn simple_path_segment_crate() {
        test_source!(source = "crate");
        let source_backup = source.clone();

        let token_slice = [Ok(Token::new(TokenKind::Crate, &source, None))];
        let eof = Eof::for_test(source);
        let (_, value @ GenericArgsConst::SimplePathSegment(SimplePathSegment::Crate(_))) =
            GenericArgsConst::parse
                .parse(TokenSlice::new(&token_slice, &eof))
                .unwrap()
        else {
            panic!("Unexpected value parsed");
        };

        assert_eq!(
            value.source().as_str(),
            source_backup.as_str(),
            "Source did not match"
        );
        assert_eq!(
            format!("{:?}", value.to_tokens_with_state(&State::new_for_test()).0),
            format!("{:?}", quote! { crate }),
            "Tokens did not match"
        );
    }
}
