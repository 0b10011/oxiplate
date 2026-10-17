pub(in crate::template::parser) mod trait_object_one_bound;

use super::Type;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::trait_bounds::bounds::Bounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectType>
#[derive(Debug)]
pub(crate) enum TraitObjectType<'a> {
    Static {
        bounds: Bounds<'a>,
    },
    Dynamic {
        r#dyn: Source<'a>,
        bounds: Option<Bounds<'a>>,
    },
}

impl<'a> TraitObjectType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectType>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, r#dyn) =
            ignore_recoverable_errors(into(KeywordParser::new("dyn"))).parse(tokens)?;

        if let Some(r#dyn) = r#dyn {
            let (tokens, bounds) = ignore_recoverable_errors(Bounds::parse).parse(tokens)?;
            return Ok((tokens, Self::Dynamic { r#dyn, bounds }));
        }

        let (tokens, bounds) = Bounds::parse(tokens)?;

        Ok((tokens, Self::Static { bounds }))
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Static { bounds } => bounds.source(),
            Self::Dynamic { r#dyn, bounds } => r#dyn.clone().merge_some(
                bounds.as_ref().map(Bounds::source).as_ref(),
                "Bounds expected after `dyn`",
            ),
        }
    }
}

impl<'a> ToTokensWithState<'a> for TraitObjectType<'a> {
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

impl<'a> From<TraitObjectType<'a>> for Type<'a> {
    fn from(value: TraitObjectType<'a>) -> Self {
        Type::TraitObject(value)
    }
}
