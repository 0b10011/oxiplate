use super::GenericArgs;
use crate::template::parser::expression::path::PathIdentSegment;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePathSegment>
#[derive(Debug)]
pub(crate) struct TypePathSegment<'a> {
    ident_segment: PathIdentSegment<'a>,

    /// `Source` is optional path separator (`::`).
    generics: Option<(Option<Source<'a>>, Box<GenericArgs<'a>>)>,
}

impl<'a> TypePathSegment<'a> {
    /// Helper function to build a type path segment for a test.
    #[cfg(test)]
    pub(crate) fn new_for_test(
        ident_segment: PathIdentSegment<'a>,
        generics: Option<(Option<Source<'a>>, Box<GenericArgs<'a>>)>,
    ) -> Self {
        Self {
            ident_segment,
            generics,
        }
    }

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePathSegment>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (ident_segment, generics)) = (
            PathIdentSegment::parse,
            ignore_recoverable_errors((
                ignore_recoverable_errors(take(TokenKind::PathSeparator)),
                GenericArgs::parse,
            )),
        )
            .parse(tokens)?;

        let generics = generics.map(|(separator, generics)| {
            let separator = separator.map(|separator| separator.source().clone());
            let generics = Box::new(generics);

            (separator, generics)
        });

        Ok((
            tokens,
            Self {
                ident_segment,
                generics,
            },
        ))
    }

    pub(crate) fn source(&self) -> Source<'a> {
        let mut source = self.ident_segment.source().clone();

        if let Some((separator, generics)) = &self.generics {
            source = source
                .merge_some(separator.as_ref(), "`::` expected after ident segment")
                .merge(
                    &generics.source(),
                    "Generics expected after ident segment or `::`",
                );
        }

        source
    }
}

impl<'a> ToTokensWithState<'a> for TypePathSegment<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        self.ident_segment.to_tokens(&mut tokens);

        if let Some((separator, generics)) = &self.generics {
            if let Some(separator) = separator {
                let span = separator.span_token();
                tokens.append_all(quote_spanned! {span=> :: });
            }

            tokens.append_all(generics.to_tokens_with_state(state).0);
        }

        (tokens, EstimatedLength::new(0))
    }
}
