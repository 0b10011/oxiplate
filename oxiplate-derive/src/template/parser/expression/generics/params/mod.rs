use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParams>
#[derive(Debug)]
pub(super) struct GenericParams<'a> {
    /// `Source` is the comma (`,`)
    first_generics: Vec<(GenericParam, Source<'a>)>,

    /// `Source` is the comma (`,`)
    last_generic: Option<(GenericParam, Option<Source<'a>>)>,

    /// `Source` is the full list including wrapping `<` and `>`
    source: Source<'a>,
}

impl<'a> GenericParams<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParams>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
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

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl ToTokens for GenericParams<'_> {
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

/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParam>
#[derive(Debug)]
pub(super) enum GenericParam {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-LifetimeParam>
    #[expect(dead_code, reason = "Not yet implemented")]
    Lifetime,

    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-TypeParam>
    #[expect(dead_code, reason = "Not yet implemented")]
    Type,

    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-ConstParam>
    #[expect(dead_code, reason = "Not yet implemented")]
    Const,
}

impl<'a> GenericParam {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-GenericParams>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let _ = tokens;
        todo!("GenericParam::parse() not yet written");
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Lifetime => todo!("GenericParam::Lifetime.source() not yet written"),
            Self::Type => todo!("GenericParam::Type.source() not yet written"),
            Self::Const => todo!("GenericParam::Const.source() not yet written"),
        }
    }
}

impl ToTokens for GenericParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ = tokens;

        match self {
            Self::Lifetime => todo!("GenericParam::Lifetime.to_tokens() not yet written"),
            Self::Type => todo!("GenericParam::Type.to_tokens() not yet written"),
            Self::Const => todo!("GenericParam::Const.to_tokens() not yet written"),
        }
    }
}
