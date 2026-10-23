use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::items::generics::GenericParams;
use crate::template::parser::expression::paths::path::TypePath;
use crate::template::parser::prelude::*;

/// Non-parenthesized part of trait bound.
/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-TraitBound>
#[derive(Debug)]
pub(crate) struct TraitBoundInner<'a> {
    prefix: Option<TraitBoundPrefix<'a>>,
    type_path: TypePath<'a>,
}

impl<'a> TraitBoundInner<'a> {
    /// Non-parenthesized part of trait bound.
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-TraitBound>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (prefix, type_path)) = alt((
            (
                some(TraitBoundPrefix::parse),
                cut("Type path expected in trait bound", TypePath::parse),
            ),
            (none(), TypePath::parse),
        ))
        .parse(tokens)?;

        Ok((tokens, Self { prefix, type_path }))
    }

    pub(super) fn source(&self) -> Source<'a> {
        self.type_path.source().append_to_some(
            self.prefix.as_ref().map(|prefix| prefix.source().clone()),
            "Type path expected after trait bound prefix",
        )
    }
}

impl<'a> ToTokensWithState<'a> for TraitBoundInner<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        if let Some(prefix) = &self.prefix {
            tokens.append_all(prefix.to_tokens_with_state(state).0);
        }
        tokens.append_all(self.type_path.to_tokens_with_state(state).0);

        (tokens, EstimatedLength::new(0))
    }
}

#[derive(Debug)]
enum TraitBoundPrefix<'a> {
    QuestionMark(Source<'a>),
    ForLifetimes {
        r#for: Source<'a>,
        generic_params: Box<GenericParams<'a>>,
    },
}

impl<'a> TraitBoundPrefix<'a> {
    /// `?` or `ForLifetimes` from `TraitBound` definition.
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-TraitBound>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, question_mark) =
            ignore_recoverable_errors(take(TokenKind::QuestionMark)).parse(tokens)?;

        if let Some(question_mark) = question_mark {
            return Ok((tokens, Self::QuestionMark(question_mark.source().clone())));
        }

        let (tokens, (r#for, generic_params)) = (
            KeywordParser::new("for"),
            cut("Generic params expected after `for`", GenericParams::parse),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self::ForLifetimes {
                r#for: r#for.source().clone(),
                generic_params: Box::new(generic_params),
            },
        ))
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::QuestionMark(source) => source.clone(),
            Self::ForLifetimes {
                r#for,
                generic_params,
            } => r#for.clone().merge(
                generic_params.source(),
                "Generic params expected after `for`",
            ),
        }
    }
}

impl<'a> ToTokensWithState<'a> for TraitBoundPrefix<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::QuestionMark(source) => {
                let span = source.span_token();
                (quote_spanned! {span=> ? }, EstimatedLength::new(0))
            }
            Self::ForLifetimes {
                r#for,
                generic_params,
            } => {
                let span = r#for.span_token();
                let generic_params = generic_params.to_tokens_with_state(state).0;
                (
                    quote_spanned! {span=> for #generic_params },
                    EstimatedLength::new(0),
                )
            }
        }
    }
}
