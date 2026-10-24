use super::GenericArgs;
use crate::template::parser::expression::types::Type;
use crate::template::parser::expression::types::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypeList>
#[derive(Debug)]
pub(super) struct TypeList<'a> {
    first_types: Vec<(Type<'a>, Source<'a>)>,
    last_type: Option<(Type<'a>, Option<Source<'a>>)>,

    /// Source for the full parentheses group.
    parentheses: Source<'a>,

    /// `Source` is the array `->`.
    type_no_bounds: Option<(Source<'a>, TypeNoBounds<'a>)>,
}

impl<'a> TypeList<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypeList>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        type FirstTypes<'a> = Vec<(Type<'a>, Source<'a>)>;
        type LastType<'a> = Option<(Type<'a>, Option<Source<'a>>)>;
        type OptionalTypeNoBounds<'a> = Option<(Source<'a>, Source<'a>, TypeNoBounds<'a>)>;
        let (tokens, (open_paren, mut first_types, last_type, close_paren, type_no_bounds)): (
            _,
            (Source, FirstTypes, LastType, Source, OptionalTypeNoBounds),
        ) = (
            into(take(TokenKind::OpenParenthese)),
            many0((Type::parse, into(take(TokenKind::Comma)))),
            ignore_recoverable_errors((
                Type::parse,
                ignore_recoverable_errors(into(take(TokenKind::Comma))),
            )),
            into(take(TokenKind::CloseParenthese)),
            ignore_recoverable_errors((
                into(take(TokenKind::Minus)),
                into(take(TokenKind::GreaterThan)),
                TypeNoBounds::parse,
            )),
        )
            .parse(tokens)?;

        let last_type = if last_type.is_some() {
            last_type
        } else {
            first_types.pop().map(|(ty, comma)| (ty, Some(comma)))
        };

        let mut parentheses = open_paren;

        for (ty, comma) in &first_types {
            parentheses = parentheses
                .merge(&ty.source(), "Type expected after `,` or `<`")
                .merge(comma, "`,` expected after type");
        }

        if let Some((ty, comma)) = &last_type {
            parentheses = parentheses
                .merge(&ty.source(), "Type expected after `,` or `<`")
                .merge_some(comma.to_owned().as_ref(), "`,` expected after last type");
        }

        parentheses = parentheses.merge(&close_paren, "`>` expected after type");

        let type_no_bounds = if let Some((minus, greater_than, type_no_bounds)) = type_no_bounds {
            Some((
                minus.merge(&greater_than, "`>` expected after `-`"),
                type_no_bounds,
            ))
        } else {
            None
        };

        Ok((
            tokens,
            Self {
                first_types: first_types.into_iter().collect(),
                last_type,
                parentheses,
                type_no_bounds,
            },
        ))
    }

    pub fn source(&self) -> Source<'a> {
        if let Some((arrow, type_no_bounds)) = &self.type_no_bounds {
            self.parentheses
                .clone()
                .merge(arrow, "`->` expected after type list")
                .merge(
                    &type_no_bounds.source(),
                    "Type no bounds expected after `->`",
                )
        } else {
            self.parentheses.clone()
        }
    }
}

impl<'a> ToTokensWithState<'a> for TypeList<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut inner_tokens = TokenStream::new();

        for (ty, comma) in &self.first_types {
            let span = comma.span_token();
            let ty = ty.to_tokens_with_state(state).0;
            inner_tokens.append_all(quote_spanned! {span=> #ty, });
        }

        if let Some((ty, comma)) = &self.last_type {
            inner_tokens.append_all(ty.to_tokens_with_state(state).0);
            if let Some(comma) = comma {
                let span = comma.span_token();
                inner_tokens.append_all(quote_spanned! {span=> , });
            }
        }

        let type_no_bounds = if let Some((arrow, type_no_bounds)) = &self.type_no_bounds {
            let span = arrow.span_token();
            let type_no_bounds = type_no_bounds.to_tokens_with_state(state).0;
            quote_spanned! {span=> -> #type_no_bounds }
        } else {
            quote! {}
        };

        let span = self.parentheses.span_token();
        (
            quote_spanned! {span=> ( #inner_tokens ) #type_no_bounds },
            EstimatedLength::new(0),
        )
    }
}

impl<'a> From<TypeList<'a>> for GenericArgs<'a> {
    fn from(value: TypeList<'a>) -> Self {
        Self::Types(value)
    }
}
