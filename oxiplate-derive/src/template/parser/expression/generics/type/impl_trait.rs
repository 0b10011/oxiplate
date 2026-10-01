use super::Type;
use super::bounds::Bounds;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
#[derive(Debug)]
pub(crate) struct ImplTraitType<'a> {
    r#impl: Source<'a>,
    bounds: Option<Bounds<'a>>,
}

impl<'a> ImplTraitType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (r#impl, bounds)) = (
            KeywordParser::new("impl"),
            cut(
                "Trait bound expected after `impl`",
                ignore_recoverable_errors(Bounds::parse),
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self {
                r#impl: r#impl.source().clone(),
                bounds,
            },
        ))
    }

    pub(super) fn source(&self) -> Source<'a> {
        self.r#impl.clone().merge_some(
            self.bounds.as_ref().map(Bounds::source).as_ref(),
            "Trait bound expected after `impl`",
        )
    }
}

impl ToTokens for ImplTraitType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.r#impl.span_token();
        tokens.append_all(quote_spanned! {span=> impl });

        if let Some(bounds) = &self.bounds {
            bounds.to_tokens(tokens);
        }
    }
}

impl<'a> From<ImplTraitType<'a>> for Type<'a> {
    fn from(value: ImplTraitType<'a>) -> Self {
        Self::ImplTrait(value)
    }
}
