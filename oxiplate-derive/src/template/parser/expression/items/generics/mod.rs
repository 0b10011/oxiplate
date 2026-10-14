pub(in crate::template::parser) mod lifetime;
pub(in crate::template::parser) mod r#type;

use self::lifetime::LifetimeParam;
use self::r#type::TypeParam;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParams>
#[derive(Debug)]
pub(in crate::template::parser) struct GenericParams<'a> {
    /// `Source` is the comma (`,`)
    first_generics: Vec<(GenericParam<'a>, Source<'a>)>,

    /// `Source` is the comma (`,`)
    last_generic: Option<(GenericParam<'a>, Option<Source<'a>>)>,

    /// `Source` is the full list including wrapping `<` and `>`
    source: Source<'a>,
}

impl<'a> GenericParams<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParams>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, (mut first_generics, last_generic), greater_than)) = (
            take(TokenKind::LessThan),
            (
                many0((GenericParam::parse, take(TokenKind::Comma))),
                ignore_recoverable_errors((
                    GenericParam::parse,
                    ignore_recoverable_errors(take(TokenKind::Comma)),
                )),
            ),
            take(TokenKind::GreaterThan),
        )
            .parse(tokens)?;

        let last_generic = if last_generic.is_some() {
            last_generic
                .map(|(generic, comma)| (generic, comma.map(|comma| comma.source().clone())))
        } else {
            first_generics
                .pop()
                .map(|(generic, comma)| (generic, Some(comma.source().clone())))
        };

        let mut source = less_than.source().clone();

        for (generic, comma) in &first_generics {
            source = source
                .merge(&generic.source(), "Generic expected after `,` or `<`")
                .merge(comma.source(), "`,` expected after generic");
        }

        if let Some((generic, comma)) = &last_generic {
            source = source
                .merge(&generic.source(), "Generic expected after `,` or `<`")
                .merge_some(comma.as_ref(), "`,` expected after last generic");
        }

        source = source.merge(greater_than.source(), "`>` expected after generics");

        Ok((
            tokens,
            Self {
                first_generics: first_generics
                    .into_iter()
                    .map(|(generic, comma)| (generic, comma.source().clone()))
                    .collect(),
                last_generic,
                source,
            },
        ))
    }

    pub(in crate::template::parser) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for GenericParams<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut inner_tokens = TokenStream::new();

        for (arg, comma) in &self.first_generics {
            let span = comma.span_token();
            let arg = arg.to_tokens_with_state(state).0;
            inner_tokens.append_all(quote_spanned! {span=> #arg, });
        }

        if let Some((generic, comma)) = &self.last_generic {
            inner_tokens.append_all(generic.to_tokens_with_state(state).0);
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

/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParam>
#[derive(Debug)]
pub(super) enum GenericParam<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-LifetimeParam>
    Lifetime(LifetimeParam<'a>),

    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-TypeParam>
    Type(TypeParam<'a>),

    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-ConstParam>
    #[expect(dead_code, reason = "Not yet implemented")]
    Const,
}

impl<'a> GenericParam<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParams>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((into(LifetimeParam::parse), into(TypeParam::parse))).parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Lifetime(lifetime) => lifetime.source(),
            Self::Type(type_param) => type_param.source().clone(),
            Self::Const => todo!("GenericParam::Const.source() not yet written"),
        }
    }
}

impl<'a> ToTokensWithState<'a> for GenericParam<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Lifetime(lifetime) => lifetime.to_tokens_with_state(state),
            Self::Type(type_param) => type_param.to_tokens_with_state(state),
            Self::Const => todo!("GenericParam::Const.to_tokens() not yet written"),
        }
    }
}
