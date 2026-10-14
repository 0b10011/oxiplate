use crate::template::parser::expression::Identifier;
use crate::template::parser::expression::path::{Crate, SelfCurrentModule, Super};
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-SimplePathSegment>
///
/// `$crate` support is intentionally skipped at this time,
/// but may be added at some point in the future.
#[derive(Debug)]
pub(in crate::template::parser) enum SimplePathSegment<'a> {
    Identifier(Identifier<'a>),

    /// `super`
    Super(Super<'a>),

    /// `self`
    SelfCurrentModule(SelfCurrentModule<'a>),

    /// `crate`
    Crate(Crate<'a>),
}

impl<'a> SimplePathSegment<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-SimplePathSegment>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, simple_path_segment) = alt((
            into(Identifier::parse),
            into(Super::parse),
            into(SelfCurrentModule::parse),
            into(Crate::parse),
        ))
        .parse(tokens)?;

        Ok((tokens, simple_path_segment))
    }

    pub(in crate::template::parser) fn source(&self) -> &Source<'a> {
        match self {
            Self::Identifier(identifier) => identifier.source(),
            Self::Super(keyword) => keyword.source(),
            Self::SelfCurrentModule(keyword) => keyword.source(),
            Self::Crate(keyword) => keyword.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for SimplePathSegment<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Identifier(identifier) => (identifier.to_token_stream(), EstimatedLength::new(0)),
            Self::Super(keyword) => keyword.to_tokens_with_state(state),
            Self::SelfCurrentModule(keyword) => keyword.to_tokens_with_state(state),
            Self::Crate(keyword) => keyword.to_tokens_with_state(state),
        }
    }
}

impl<'a> From<Identifier<'a>> for SimplePathSegment<'a> {
    fn from(value: Identifier<'a>) -> Self {
        Self::Identifier(value)
    }
}

impl<'a> From<Super<'a>> for SimplePathSegment<'a> {
    fn from(value: Super<'a>) -> Self {
        Self::Super(value)
    }
}

impl<'a> From<SelfCurrentModule<'a>> for SimplePathSegment<'a> {
    fn from(value: SelfCurrentModule<'a>) -> Self {
        Self::SelfCurrentModule(value)
    }
}

impl<'a> From<Crate<'a>> for SimplePathSegment<'a> {
    fn from(value: Crate<'a>) -> Self {
        Self::Crate(value)
    }
}
