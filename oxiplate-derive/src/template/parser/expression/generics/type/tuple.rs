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
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (open_paren, first_types, last_type, close_paren)) = (
            into(take(TokenKind::OpenParenthese)),
            many0((Type::parse, into(take(TokenKind::Comma)))),
            ignore_recoverable_errors(Type::parse),
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
            Self {
                first_types,
                last_type: last_type.map(Box::new),
                source,
            },
        ))
    }

    pub(super) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl ToTokens for TupleType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let first_types = self.first_types.iter().map(|(r#type, comma)| {
            let span = comma.span_token();
            quote_spanned! {span=> #r#type , }
        });
        let last_type = &self.last_type;
        let span = self.source.span_token();
        tokens.append_all(quote_spanned! {span=>
            (
                #(#first_types)* #last_type
            )
        });
    }
}

impl<'a> From<TupleType<'a>> for TypeNoBounds<'a> {
    fn from(value: TupleType<'a>) -> Self {
        Self::Tuple(value)
    }
}
