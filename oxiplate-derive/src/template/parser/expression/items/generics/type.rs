use super::GenericParam;
use crate::template::parser::expression::Identifier;
use crate::template::parser::expression::trait_bounds::bounds::Bounds;
use crate::template::parser::expression::types::Type;
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
    pub(in crate::template::parser) fn parser() -> TypeParamParser<'a> {
        TypeParamParser {
            phantom_data: PhantomData,
        }
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

impl<'a> ToTokensWithState<'a> for TypeParam<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        self.identifier.to_tokens(&mut tokens);

        if let Some((colon, bounds)) = &self.bounds {
            let span = colon.span_token();
            tokens.append_all(quote_spanned! {span=> : });

            if let Some(bounds) = bounds {
                tokens.append_all(bounds.to_tokens_with_state(state).0);
            }
        }

        if let Some((equals, r#type)) = &self.r#type {
            let span = equals.span_token();
            tokens.append_all(quote_spanned! {span=> = });

            tokens.append_all(r#type.to_tokens_with_state(state).0);
        }

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<TypeParam<'a>> for GenericParam<'a> {
    fn from(value: TypeParam<'a>) -> Self {
        Self::Type(value)
    }
}

pub(in crate::template::parser) struct TypeParamParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for TypeParamParser<'a> {
    type TokenKind = TokenKind;
    type Output = TypeParam<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (identifier, bounds, r#type)) = (
            Identifier::parser(),
            ignore_recoverable_errors((
                into(take(TokenKind::Colon)),
                ignore_recoverable_errors(into(Bounds::parser())),
            )),
            ignore_recoverable_errors((into(take(TokenKind::Equal)), into(Type::parser()))),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            TypeParam {
                identifier,
                bounds,
                r#type,
            },
        ))
    }
}
