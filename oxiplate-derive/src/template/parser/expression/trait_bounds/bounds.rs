use super::bound::Bound;
use crate::template::parser::prelude::*;

/// Between `first_bounds` and `last_bound`,
/// there should always be at least one bound.
///
/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Bounds>
#[derive(Debug)]
pub(crate) struct Bounds<'a> {
    /// `Source` is a plus sign (`+`)
    first_bounds: Vec<(Bound<'a>, Source<'a>)>,

    last_bound: Option<Bound<'a>>,
}

impl<'a> Bounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Bounds>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (first_bounds, last_bound)) = (
            many0((Bound::parse, into(take(TokenKind::Plus)))),
            ignore_recoverable_errors(Bound::parse),
        )
            .parse(tokens)?;

        if first_bounds.is_empty() && last_bound.is_none() {
            return context("Expected one or more bounds", fail()).parse(tokens);
        }

        Ok((
            tokens,
            Self {
                first_bounds,
                last_bound,
            },
        ))
    }

    pub(crate) fn source(&self) -> Source<'a> {
        let mut source = None;

        for (bound, plus) in &self.first_bounds {
            source = Some(
                bound
                    .source()
                    .append_to_some(source, "Bound expected")
                    .merge(plus, "`+` expected after bound"),
            );
        }

        if let Some(bound) = &self.last_bound {
            source = Some(bound.source().append_to_some(source, "Bound expected"));
        }

        source.expect(
            "`Bounds` should always contain at least one bound, therefore a `Source` should \
             always be present",
        )
    }
}

impl<'a> ToTokensWithState<'a> for Bounds<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        for (bound, plus) in &self.first_bounds {
            tokens.append_all(bound.to_tokens_with_state(state).0);
            let span = plus.span_token();
            tokens.append_all(quote_spanned! {span=> + });
        }

        if let Some(bound) = &self.last_bound {
            tokens.append_all(bound.to_tokens_with_state(state).0);
        }

        (tokens, EstimatedLength::new(0))
    }
}
