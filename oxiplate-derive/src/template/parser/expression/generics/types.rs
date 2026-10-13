use super::GenericArgs;
use super::r#type::Type;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypeList>
#[derive(Debug)]
pub(super) struct TypeList<'a> {
    first_types: Vec<(Type<'a>, Source<'a>)>,
    last_type: Option<(Type<'a>, Option<Source<'a>>)>,
    source: Source<'a>,
}

impl<'a> TypeList<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypeList>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, (mut first_types, last_type), greater_than)) = (
            take(TokenKind::LessThan),
            (
                many0((Type::parse, take(TokenKind::Comma))),
                ignore_recoverable_errors((
                    Type::parse,
                    ignore_recoverable_errors(take(TokenKind::Comma)),
                )),
            ),
            take(TokenKind::GreaterThan),
        )
            .parse(tokens)?;

        let last_type = if last_type.is_some() {
            last_type.map(|(ty, comma)| (ty, comma.map(|comma| comma.source().clone())))
        } else {
            first_types
                .pop()
                .map(|(ty, comma)| (ty, Some(comma.source().clone())))
        };

        let mut source = less_than.source().clone();

        for (ty, comma) in &first_types {
            source = source
                .merge(&ty.source(), "Type expected after `,` or `<`")
                .merge(comma.source(), "`,` expected after type");
        }

        if let Some((ty, comma)) = &last_type {
            source = source
                .merge(&ty.source(), "Type expected after `,` or `<`")
                .merge_some(comma.as_ref(), "`,` expected after last type");
        }

        source = source.merge(greater_than.source(), "`>` expected after type");

        Ok((
            tokens,
            Self {
                first_types: first_types
                    .into_iter()
                    .map(|(ty, comma)| (ty, comma.source().clone()))
                    .collect(),
                last_type,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for TypeList<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut inner_tokens = TokenStream::new();

        for (ty, comma) in &self.first_types {
            let span = comma.span_token();
            let ty = ty.to_tokens_with_state(state).0;
            inner_tokens.append_all(quote_spanned! {span=> #ty, });
        }

        if let Some((ty, comma)) = &self.last_type {
            inner_tokens.append_all(ty.to_tokens_with_state(state).0);
            if let Some(comma) = comma {
                let span = comma.span_token();
                inner_tokens.append_all(quote_spanned! {span=> , });
            }
        }

        let span = self.source.span_token();
        (
            quote_spanned! {span=> <#inner_tokens> },
            EstimatedLength::new(0),
        )
    }
}

impl<'a> From<TypeList<'a>> for GenericArgs<'a> {
    fn from(value: TypeList<'a>) -> Self {
        Self::Types(value)
    }
}
