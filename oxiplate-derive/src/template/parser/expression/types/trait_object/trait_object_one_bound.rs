use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::trait_bounds::trait_bound::TraitBound;
use crate::template::parser::expression::types::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectTypeOneBound>
#[derive(Debug)]
pub(in crate::template::parser) enum TraitObjectTypeOneBound<'a> {
    Static {
        bounds: TraitBound<'a>,
    },
    Dynamic {
        r#dyn: Source<'a>,
        bounds: Option<TraitBound<'a>>,
    },
}

impl<'a> TraitObjectTypeOneBound<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectTypeOneBound>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, r#dyn) =
            ignore_recoverable_errors(into(KeywordParser::new("dyn"))).parse(tokens)?;

        if let Some(r#dyn) = r#dyn {
            let (tokens, bounds) = ignore_recoverable_errors(TraitBound::parse).parse(tokens)?;
            return Ok((tokens, Self::Dynamic { r#dyn, bounds }));
        }

        let (tokens, bounds) = TraitBound::parse(tokens)?;

        Ok((tokens, Self::Static { bounds }))
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        match self {
            Self::Static { bounds } => bounds.source(),
            Self::Dynamic { r#dyn, bounds } => r#dyn.clone().merge_some(
                bounds.as_ref().map(TraitBound::source).as_ref(),
                "Bounds expected after `dyn`",
            ),
        }
    }
}

impl<'a> ToTokensWithState<'a> for TraitObjectTypeOneBound<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Static { bounds } => bounds.to_tokens_with_state(state),
            Self::Dynamic {
                r#dyn,
                bounds: None,
            } => {
                let span = r#dyn.span_token();

                (quote_spanned! {span=> dyn }, EstimatedLength::new(0))
            }
            Self::Dynamic {
                r#dyn,
                bounds: Some(bounds),
            } => {
                let span = r#dyn.span_token();

                let bounds = bounds.to_tokens_with_state(state).0;

                (
                    quote_spanned! {span=> dyn #bounds },
                    EstimatedLength::new(0),
                )
            }
        }
    }
}

impl<'a> From<TraitObjectTypeOneBound<'a>> for TypeNoBounds<'a> {
    fn from(value: TraitObjectTypeOneBound<'a>) -> Self {
        Self::TraitObjectOneBound(value)
    }
}
