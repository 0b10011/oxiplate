use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::trait_bounds::lifetime::Lifetime;
use crate::template::parser::expression::types::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/pointer.html#railroad-ReferenceType>
#[derive(Debug)]
pub(crate) struct ReferenceType<'a> {
    borrow: Source<'a>,
    lifetime: Option<Lifetime<'a>>,
    r#mut: Option<Source<'a>>,
    type_no_bounds: Box<TypeNoBounds<'a>>,
}

impl<'a> ReferenceType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/pointer.html#railroad-ReferenceType>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (borrow, lifetime, r#mut, type_no_bounds)) = (
            into(take(TokenKind::Ampersand)),
            ignore_recoverable_errors(Lifetime::parse),
            ignore_recoverable_errors(into(KeywordParser::new("mut"))),
            TypeNoBounds::parse,
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self {
                borrow,
                lifetime,
                r#mut,
                type_no_bounds: Box::new(type_no_bounds),
            },
        ))
    }

    pub(crate) fn source(&self) -> Source<'a> {
        self.borrow
            .clone()
            .merge_some(
                self.lifetime.as_ref().map(Lifetime::source).as_ref(),
                "Lifetime expected after `&`",
            )
            .merge_some(self.r#mut.as_ref(), "`mut` expected after `&` or lifetime")
            .merge(
                &self.type_no_bounds.source(),
                "Type no bounds expected after `&`, lifetime, or `mut`",
            )
    }
}

impl<'a> ToTokensWithState<'a> for ReferenceType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        let span = self.borrow.span_token();
        tokens.append_all(quote_spanned! {span=> & });

        self.lifetime.to_tokens(&mut tokens);

        if let Some(r#mut) = &self.r#mut {
            let span = r#mut.span_token();
            tokens.append_all(quote_spanned! {span=> mut });
        }

        let (type_no_bounds, _estimated_length) = self.type_no_bounds.to_tokens_with_state(state);
        tokens.append_all(type_no_bounds);

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<ReferenceType<'a>> for TypeNoBounds<'a> {
    fn from(value: ReferenceType<'a>) -> Self {
        TypeNoBounds::Reference(value)
    }
}
