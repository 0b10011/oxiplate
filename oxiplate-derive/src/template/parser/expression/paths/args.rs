use super::GenericArgs;
use super::binding::GenericArgsBinding;
use super::bounds::GenericArgsBounds;
use super::generic_args_const::GenericArgsConst;
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
    /// `GenericArgList` plus wrapping `<` and `>`.
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgList>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, (mut first_generics, last_generic), greater_than)) = (
            take(TokenKind::LessThan),
            (
                many0((GenericArg::parse, into(take(TokenKind::Comma)))),
                ignore_recoverable_errors((
                    GenericArg::parse,
                    ignore_recoverable_errors(into(take(TokenKind::Comma))),
                )),
            ),
            cut(
                "Expected `>` to end generic arg list",
                take(TokenKind::GreaterThan),
            ),
        )
            .parse(tokens)?;

        let last_generic = if last_generic.is_some() {
            last_generic
        } else {
            first_generics
                .pop()
                .map(|(generic, comma)| (generic, Some(comma)))
        };

        let mut source = less_than.source().clone();

        for (generic, comma) in &first_generics {
            source = source
                .merge(&generic.source(), "Generic expected after `,` or `<`")
                .merge(comma, "`,` expected after generic");
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
                first_generics,
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
    Const(GenericArgsConst<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
    Binding(GenericArgsBinding<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBounds>
    Bounds(GenericArgsBounds<'a>),
}

impl<'a> GenericArg<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArg>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(Lifetime::parse),
            into(GenericArgsBinding::parse),
            into(GenericArgsBounds::parse),
            into(Type::parse),
            into(GenericArgsConst::parse),
        ))
        .parse(tokens)
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        match self {
            Self::Lifetime(lifetime) => lifetime.source().clone(),
            Self::Type(r#type) => r#type.source(),
            Self::Const(generic_args_const) => generic_args_const.source(),
            Self::Binding(binding) => binding.source(),
            Self::Bounds(bounds) => bounds.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for GenericArg<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Lifetime(lifetime) => lifetime.to_tokens_with_state(state),
            Self::Type(r#type) => r#type.to_tokens_with_state(state),
            Self::Const(generic_args_const) => generic_args_const.to_tokens_with_state(state),
            Self::Binding(binding) => binding.to_tokens_with_state(state),
            Self::Bounds(bounds) => bounds.to_tokens_with_state(state),
        }
    }
}
