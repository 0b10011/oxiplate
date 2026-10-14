use super::GenericArgs;
use super::binding::Binding;
use super::bounds::GenericArgsBounds;
use crate::template::parser::expression::trait_bounds::lifetime::Lifetime;
use crate::template::parser::expression::types::Type;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgList>
#[derive(Debug)]
pub(super) struct GenericArgList<'a> {
    first_generics: Vec<(GenericArg<'a>, Source<'a>)>,
    last_generic: Option<(GenericArg<'a>, Option<Source<'a>>)>,
    source: Source<'a>,
}

impl<'a> GenericArgList<'a> {
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

impl<'a> ToTokensWithState<'a> for GenericArgList<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut inner_tokens = TokenStream::new();

        for (arg, comma) in &self.first_generics {
            inner_tokens.append_all(arg.to_tokens_with_state(state).0);
            let span = comma.span_token();
            inner_tokens.append_all(quote_spanned! {span=> , });
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

impl<'a> From<GenericArgList<'a>> for GenericArgs<'a> {
    fn from(value: GenericArgList<'a>) -> Self {
        Self::List(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArg>
#[derive(Debug)]
pub(in crate::template::parser) enum GenericArg<'a> {
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
    Bounds(GenericArgsBounds<'a>),
}

impl<'a> GenericArg<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArg>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(Lifetime::parse),
            into(Type::parse),
            into(Binding::parse),
            into(GenericArgsBounds::parse),
        ))
        .parse(tokens)
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        match self {
            Self::Lifetime(lifetime) => lifetime.source().clone(),
            Self::Type(r#type) => r#type.source(),
            Self::Const => todo!("GenericArgsConst not yet handled"),
            Self::Binding(binding) => binding.source(),
            Self::Bounds(bounds) => bounds.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for GenericArg<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        match self {
            Self::Lifetime(lifetime) => lifetime.to_tokens(&mut tokens),
            Self::Type(r#type) => return r#type.to_tokens_with_state(state),
            Self::Const => todo!("GenericArgsConst not yet handled"),
            Self::Binding(binding) => return binding.to_tokens_with_state(state),
            Self::Bounds(bounds) => return bounds.to_tokens_with_state(state),
        }

        (tokens, EstimatedLength::new(0))
    }
}
