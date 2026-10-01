use super::Generics;
use super::binding::Binding;
use super::lifetime::Lifetime;
use super::r#type::Type;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgList>
#[derive(Debug)]
pub(super) struct GenericArgs<'a> {
    first_generics: Vec<(GenericArg<'a>, Source<'a>)>,
    last_generic: Option<(GenericArg<'a>, Option<Source<'a>>)>,
    source: Source<'a>,
}

impl<'a> GenericArgs<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgList>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, (mut first_generics, last_generic), greater_than)) = (
            take(TokenKind::LessThan),
            (
                many0((GenericArg::parse, take(TokenKind::Comma))),
                ignore_recoverable_errors((
                    GenericArg::parse,
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

impl ToTokens for GenericArgs<'_> {
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

impl<'a> From<GenericArgs<'a>> for Generics<'a> {
    fn from(value: GenericArgs<'a>) -> Self {
        Self::GenericArgs(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArg>
#[derive(Debug)]
pub(super) enum GenericArg<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Lifetime>
    Lifetime(Lifetime<'a>),

    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-Type>
    Type(Type<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsConst>
    #[expect(dead_code, reason = "Not yet implemented")]
    Const,

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
    Binding(Binding<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBounds>
    #[expect(dead_code, reason = "Not yet implemented")]
    Bounds,
}

impl<'a> GenericArg<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArg>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(Lifetime::parse),
            into(Type::parse),
            into(Binding::parse),
        ))
        .parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Lifetime(lifetime) => lifetime.source().clone(),
            Self::Type(r#type) => r#type.source(),
            Self::Const => todo!("GenericArgsConst not yet handled"),
            Self::Binding(binding) => binding.source(),
            Self::Bounds => todo!("GenericArgsBounds not yet handled"),
        }
    }
}

impl ToTokens for GenericArg<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lifetime(lifetime) => lifetime.to_tokens(tokens),
            Self::Type(r#type) => r#type.to_tokens(tokens),
            Self::Const => todo!("GenericArgsConst not yet handled"),
            Self::Binding(binding) => binding.to_tokens(tokens),
            Self::Bounds => todo!("GenericArgsBounds not yet handled"),
        }
    }
}
