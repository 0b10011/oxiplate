use crate::template::parser::expression::generics::GenericArgs;
use crate::template::parser::expression::path::IdentSegment;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePathSegment>
#[derive(Debug)]
pub(crate) struct PathSegment<'a> {
    ident_segment: IdentSegment<'a>,

    /// `Source` is optional path separator (`::`).
    generics: Option<(Option<Source<'a>>, GenericArgs<'a>)>,
}

impl<'a> PathSegment<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePathSegment>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (ident_segment, generics)) = (
            IdentSegment::parse,
            ignore_recoverable_errors((
                ignore_recoverable_errors(take(TokenKind::PathSeparator)),
                GenericArgs::parse,
            )),
        )
            .parse(tokens)?;

        let generics = generics.map(|(separator, generics)| {
            let separator = separator.map(|separator| separator.source().clone());

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

    pub(super) fn source(&self) -> Source<'a> {
        let mut source = self.ident_segment.source().clone();

        if let Some((separator, generics)) = &self.generics {
            source = source
                .merge_some(separator.as_ref(), "`::` expected after ident segment")
                .merge(
                    generics.source(),
                    "Generics expected after ident segment or `::`",
                );
        }

        source
    }
}

impl ToTokens for PathSegment<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident_segment.to_tokens(tokens);

        if let Some((separator, generics)) = &self.generics {
            if let Some(separator) = separator {
                let span = separator.span_token();
                tokens.append_all(quote_spanned! {span=> :: });
            }

            generics.to_tokens(tokens);
        }
    }
}
