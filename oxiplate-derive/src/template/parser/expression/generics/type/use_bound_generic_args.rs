use crate::template::parser::expression::Identifier;
use crate::template::parser::expression::generics::lifetime::Lifetime;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-UseBoundGenericArgs>
#[derive(Debug)]
pub(super) struct UseBoundGenericArgs<'a> {
    first_generics: Vec<(UseBoundGenericArg<'a>, Source<'a>)>,
    last_generic: Option<(UseBoundGenericArg<'a>, Option<Source<'a>>)>,

    /// Full source, including wrapping `<` and `>`.
    source: Source<'a>,
}

impl<'a> UseBoundGenericArgs<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-UseBoundGenericArgs>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, (mut first_generics, last_generic), greater_than)) = (
            take(TokenKind::LessThan),
            (
                many0((UseBoundGenericArg::parse, take(TokenKind::Comma))),
                ignore_recoverable_errors((
                    UseBoundGenericArg::parse,
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

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl ToTokens for UseBoundGenericArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut inner_tokens = TokenStream::new();

        for (arg, comma) in &self.first_generics {
            let span = comma.span_token();
            inner_tokens.append_all(quote_spanned! {span=> #arg, });
        }

        if let Some((generic, comma)) = &self.last_generic {
            generic.to_tokens(&mut inner_tokens);
            if let Some(comma) = comma {
                let span = comma.span_token();
                inner_tokens.append_all(quote_spanned! {span=> , });
            }
        }

        let span = self.source.span_token();
        tokens.append_all(quote_spanned! {span=> <#inner_tokens> });
    }
}

/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-UseBoundGenericArg>
#[derive(Debug)]
pub(super) enum UseBoundGenericArg<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Lifetime>
    Lifetime(Lifetime<'a>),

    /// See: <https://doc.rust-lang.org/reference/identifiers.html#railroad-IDENTIFIER>
    Identifier(Identifier<'a>),

    /// `Self`
    SelfKeyword(Source<'a>),
}

impl<'a> UseBoundGenericArg<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArg>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, keyword) =
            ignore_recoverable_errors(take(TokenKind::SelfCurrentType)).parse(tokens)?;

        if let Some(keyword) = keyword {
            return Ok((tokens, Self::SelfKeyword(keyword.source().clone())));
        }

        alt((into(Lifetime::parse), into(Identifier::parse))).parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Lifetime(lifetime) => lifetime.source().clone(),
            Self::Identifier(identifier) => identifier.source().clone(),
            Self::SelfKeyword(keyword) => keyword.clone(),
        }
    }
}

impl ToTokens for UseBoundGenericArg<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lifetime(lifetime) => lifetime.to_tokens(tokens),
            Self::Identifier(identifier) => identifier.to_tokens(tokens),
            Self::SelfKeyword(keyword) => {
                let span = keyword.span_token();
                tokens.append_all(quote_spanned! {span=> Self });
            }
        }
    }
}

impl<'a> From<Identifier<'a>> for UseBoundGenericArg<'a> {
    fn from(value: Identifier<'a>) -> Self {
        Self::Identifier(value)
    }
}

impl<'a> From<Lifetime<'a>> for UseBoundGenericArg<'a> {
    fn from(value: Lifetime<'a>) -> Self {
        Self::Lifetime(value)
    }
}
