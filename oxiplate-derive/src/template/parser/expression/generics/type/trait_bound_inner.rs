use super::path::Path;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::generics::params::GenericParams;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-TraitBound>
#[derive(Debug)]
pub(crate) struct TraitBoundInner<'a> {
    prefix: Option<TraitBoundPrefix<'a>>,
    type_path: Path<'a>,
}

impl<'a> TraitBoundInner<'a> {
    /// Non-parenthesized part of trait bound.
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-TraitBound>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (prefix, type_path)) = (
            ignore_recoverable_errors(TraitBoundPrefix::parse),
            Path::parse,
        )
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

impl ToTokens for TraitBoundInner<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.prefix.to_tokens(tokens);
        self.type_path.to_tokens(tokens);
    }
}

#[derive(Debug)]
enum TraitBoundPrefix<'a> {
    QuestionMark(Source<'a>),
    ForLifetimes {
        r#for: Source<'a>,
        generic_params: GenericParams<'a>,
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

        let (tokens, (r#for, generic_params)) =
            (KeywordParser::new("for"), GenericParams::parse).parse(tokens)?;

        Ok((
            tokens,
            Self::ForLifetimes {
                r#for: r#for.source().clone(),
                generic_params,
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

impl ToTokens for TraitBoundPrefix<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::QuestionMark(source) => {
                let span = source.span_token();
                tokens.append_all(quote_spanned! {span=> ? });
            }
            Self::ForLifetimes {
                r#for,
                generic_params,
            } => {
                let span = r#for.span_token();
                tokens.append_all(quote_spanned! {span=> for });
                generic_params.to_tokens(tokens);
            }
        }
    }
}
