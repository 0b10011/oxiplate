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

    /// `Source` is a plus sign (`+`)
    last_bound: Option<(Bound<'a>, Option<Source<'a>>)>,
}

impl<'a> Bounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Bounds>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (first_bounds, last_bound)) = (
            many0((Bound::parse, into(take(TokenKind::Plus)))),
            ignore_recoverable_errors((
                Bound::parse,
                ignore_recoverable_errors(into(take(TokenKind::Plus))),
            )),
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

        if let Some((bound, plus)) = &self.last_bound {
            source = Some(
                bound
                    .source()
                    .append_to_some(source, "Bound expected")
                    .merge_some(plus.as_ref(), "`+` expected after bound"),
            );
        }

        source.expect(
            "`Bounds` should always contain at least one bound, therefore a `Source` should \
             always be present",
        )
    }
}

impl ToTokens for Bounds<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (bound, plus) in &self.first_bounds {
            bound.to_tokens(tokens);
            let span = plus.span_token();
            tokens.append_all(quote_spanned! {span=> + });
        }

        if let Some((bound, plus)) = &self.last_bound {
            bound.to_tokens(tokens);

            if let Some(plus) = plus {
                let span = plus.span_token();
                tokens.append_all(quote_spanned! {span=> + });
            }
        }
    }
}
