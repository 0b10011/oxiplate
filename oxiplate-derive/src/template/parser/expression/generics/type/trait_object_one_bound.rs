use super::no_bounds::NoBounds;
use super::trait_bound::TraitBound;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectType>
#[derive(Debug)]
pub(crate) enum TraitObjectTypeOneBound<'a> {
    Static {
        bounds: TraitBound<'a>,
    },
    Dynamic {
        r#dyn: Source<'a>,
        bounds: Option<TraitBound<'a>>,
    },
}

impl<'a> TraitObjectTypeOneBound<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectType>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, r#dyn) =
            ignore_recoverable_errors(into(KeywordParser::new("dyn"))).parse(tokens)?;

        if let Some(r#dyn) = r#dyn {
            let (tokens, bounds) = ignore_recoverable_errors(TraitBound::parse).parse(tokens)?;
            return Ok((tokens, Self::Dynamic { r#dyn, bounds }));
        }

        let (tokens, bounds) = TraitBound::parse(tokens)?;

        Ok((tokens, Self::Static { bounds }))
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Static { bounds } => bounds.source(),
            Self::Dynamic { r#dyn, bounds } => r#dyn.clone().merge_some(
                bounds.as_ref().map(TraitBound::source).as_ref(),
                "Bounds expected after `dyn`",
            ),
        }
    }
}

impl ToTokens for TraitObjectTypeOneBound<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Static { bounds } => bounds.to_tokens(tokens),
            Self::Dynamic { r#dyn, bounds } => {
                let span = r#dyn.span_token();
                tokens.append_all(quote_spanned! {span=> dyn });

                bounds.to_tokens(tokens);
            }
        }
    }
}

impl<'a> From<TraitObjectTypeOneBound<'a>> for NoBounds<'a> {
    fn from(value: TraitObjectTypeOneBound<'a>) -> Self {
        Self::TraitObjectOneBound(value)
    }
}
