use super::trait_bound_inner::TraitBoundInner;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-TraitBound>
#[derive(Debug)]
pub(crate) enum TraitBound<'a> {
    Standalone(TraitBoundInner<'a>),
    Parenthesized {
        trait_bound: TraitBoundInner<'a>,
        source: Source<'a>,
    },
}

impl<'a> TraitBound<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-TraitBound>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, result) = ignore_recoverable_errors((
            take(TokenKind::OpenParenthese),
            TraitBoundInner::parse,
            take(TokenKind::CloseParenthese),
        ))
        .parse(tokens)?;

        let (tokens, trait_bound) = if let Some((open, trait_bound, close)) = result {
            let source = open
                .source()
                .clone()
                .merge(
                    &trait_bound.source(),
                    "Inner trait bound expected after `(`",
                )
                .merge(close.source(), "`)` expected after inner trait bound");

            (
                tokens,
                TraitBound::Parenthesized {
                    trait_bound,
                    source,
                },
            )
        } else {
            let (tokens, trait_bound) = TraitBoundInner::parse(tokens)?;

            (tokens, TraitBound::Standalone(trait_bound))
        };

        Ok((tokens, trait_bound))
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Standalone(trait_bound) => trait_bound.source(),
            Self::Parenthesized {
                trait_bound: _,
                source,
            } => source.clone(),
        }
    }
}

impl ToTokens for TraitBound<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Standalone(trait_bound) => trait_bound.to_tokens(tokens),
            Self::Parenthesized {
                trait_bound,
                source,
            } => {
                let span = source.span_token();
                tokens.append_all(quote_spanned! {span=> impl #trait_bound });
            }
        }
    }
}
