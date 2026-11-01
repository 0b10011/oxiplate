use super::Type;
use super::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/tuple.html#railroad-TupleType>
#[derive(Debug)]
pub(crate) struct TupleType<'a> {
    /// `Source` is the comma (`,`)
    first_types: Vec<(Type<'a>, Source<'a>)>,
    last_type: Option<Box<Type<'a>>>,

    /// Full source including wrapping `(` and `)`
    source: Source<'a>,
}

impl<'a> TupleType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/tuple.html#railroad-TupleType>
    pub(in crate::template::parser) fn parser() -> TupleTypeParser<'a> {
        TupleTypeParser {
            phantom_data: PhantomData,
        }
    }

    pub(super) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for TupleType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let first_types = self.first_types.iter().map(|(r#type, comma)| {
            let span = comma.span_token();
            let r#type = r#type.to_tokens_with_state(state).0;
            quote_spanned! {span=> #r#type , }
        });
        let last_type = self
            .last_type
            .as_ref()
            .map(|r#type| r#type.to_tokens_with_state(state).0);
        let span = self.source.span_token();
        (
            quote_spanned! {span=>
                (
                    #(#first_types)* #last_type
                )
            },
            EstimatedLength::new(0),
        )
    }
}

impl<'a> From<TupleType<'a>> for TypeNoBounds<'a> {
    fn from(value: TupleType<'a>) -> Self {
        Self::Tuple(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/types/tuple.html#railroad-TupleType>
pub(in crate::template::parser) struct TupleTypeParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for TupleTypeParser<'a> {
    type TokenKind = TokenKind;
    type Output = TupleType<'a>;

    /// See: <https://doc.rust-lang.org/reference/types/tuple.html#railroad-TupleType>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (open_paren, first_types, last_type, close_paren)) = (
            into(take(TokenKind::OpenParenthese)),
            many1((Type::parser(), into(take(TokenKind::Comma)))),
            ignore_recoverable_errors(Type::parser()),
            into(take(TokenKind::CloseParenthese)),
        )
            .parse(tokens)?;

        let mut source: Source<'a> = open_paren;

        for (r#type, comma) in &first_types {
            source = source
                .merge(&r#type.source(), "Type expected after `(` or `,`")
                .merge(comma, "`,` expected after type");
        }

        source = source
            .merge_some(
                last_type.as_ref().map(Type::source).as_ref(),
                "Type expected after `(` or `,`",
            )
            .merge(&close_paren, "`)` expected after last type");

        Ok((
            tokens,
            TupleType {
                first_types,
                last_type: last_type.map(Box::new),
                source,
            },
        ))
    }
}
