use super::GenericParam;
use crate::template::parser::expression::trait_bounds::lifetime::Lifetime;
use crate::template::parser::expression::trait_bounds::lifetime_bounds::LifetimeBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-LifetimeParam>
#[derive(Debug)]
pub(crate) struct LifetimeParam<'a> {
    lifetime: Lifetime<'a>,
    bounds: Option<(Source<'a>, Option<Box<LifetimeBounds<'a>>>)>,
}

impl<'a> LifetimeParam<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-LifetimeParam>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (lifetime, bounds)) = (
            Lifetime::parse,
            ignore_recoverable_errors((
                into(take(TokenKind::Colon)),
                ignore_recoverable_errors(into(LifetimeBounds::parse)),
            )),
        )
            .parse(tokens)?;

        Ok((tokens, Self { lifetime, bounds }))
    }

    pub fn source(&self) -> Source<'a> {
        let mut source = self.lifetime.source().clone();

        if let Some((colon, bounds)) = &self.bounds {
            source = source.merge(colon, "`:` expected after identifier");

            if let Some(bounds) = bounds {
                source = source.merge(&bounds.source(), "Bounds expected after `:`");
            }
        }

        source
    }
}

impl<'a> ToTokensWithState<'a> for LifetimeParam<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        tokens.append_all(self.lifetime.to_tokens_with_state(state).0);

        if let Some((colon, bounds)) = &self.bounds {
            let span = colon.span_token();
            tokens.append_all(quote_spanned! {span=> : });

            if let Some(bounds) = bounds {
                tokens.append_all(bounds.to_tokens_with_state(state).0);
            }
        }

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<LifetimeParam<'a>> for GenericParam<'a> {
    fn from(value: LifetimeParam<'a>) -> Self {
        Self::Lifetime(value)
    }
}
