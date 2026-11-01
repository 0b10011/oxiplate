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
    pub(in crate::template::parser) fn parser() -> SimplePathSegmentParser<'a> {
        SimplePathSegmentParser {
            phantom_data: PhantomData,
        }
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

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-SimplePathSegment>
pub(in crate::template::parser) struct SimplePathSegmentParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for SimplePathSegmentParser<'a> {
    type TokenKind = TokenKind;
    type Output = SimplePathSegment<'a>;

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-SimplePathSegment>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, simple_path_segment) = alt((
            into(Identifier::parser()),
            into(Super::parser()),
            into(SelfCurrentModule::parser()),
            into(Crate::parser()),
        ))
        .parse(tokens)?;

        Ok((tokens, simple_path_segment))
    }
}
