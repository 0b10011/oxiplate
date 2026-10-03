use super::GenericParam;
use crate::template::parser::expression::Identifier;
use crate::template::parser::expression::generics::r#type::Type;
use crate::template::parser::expression::generics::r#type::bounds::Bounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-TypeParam>
#[derive(Debug)]
pub(crate) struct TypeParam<'a> {
    identifier: Identifier<'a>,
    bounds: Option<(Source<'a>, Option<Box<Bounds<'a>>>)>,
    r#type: Option<(Source<'a>, Box<Type<'a>>)>,
}

impl<'a> TypeParam<'a> {
    /// See: <https://doc.rust-lang.org/reference/items/generics.html#railroad-TypeParam>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (identifier, bounds, r#type)) = (
            Identifier::parse,
            ignore_recoverable_errors((
                into(take(TokenKind::Colon)),
                ignore_recoverable_errors(into(Bounds::parse)),
            )),
            ignore_recoverable_errors((into(take(TokenKind::Equal)), into(Type::parse))),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self {
                identifier,
                bounds,
                r#type,
            },
        ))
    }

    pub fn source(&self) -> Source<'a> {
        let mut source = self.identifier.source().clone();

        if let Some((colon, bounds)) = &self.bounds {
            source = source.merge(colon, "`:` expected after identifier");

            if let Some(bounds) = bounds {
                source = source.merge(&bounds.source(), "Bounds expected after `:`");
            }
        }

        if let Some((equals, r#type)) = &self.r#type {
            source = source
                .merge(equals, "`=` expected after identifier or bounds")
                .merge(&r#type.source(), "Type expected after `=`");
        }

        source
    }
}

impl ToTokens for TypeParam<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.identifier.to_tokens(tokens);

        if let Some((colon, bounds)) = &self.bounds {
            let span = colon.span_token();
            tokens.append_all(quote_spanned! {span=> : });

            if let Some(bounds) = bounds {
                bounds.to_tokens(tokens);
            }
        }

        if let Some((equals, r#type)) = &self.r#type {
            let span = equals.span_token();
            tokens.append_all(quote_spanned! {span=> = });

            r#type.to_tokens(tokens);
        }
    }
}

impl<'a> From<TypeParam<'a>> for GenericParam<'a> {
    fn from(value: TypeParam<'a>) -> Self {
        Self::Type(value)
    }
}
