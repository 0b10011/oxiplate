use proc_macro2::Ident;

use super::path::TypePath;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::types::Type;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathType>
#[derive(Debug)]
pub(crate) struct QualifiedPathType<'a> {
    r#type: Box<Type<'a>>,
    r#as: Option<(Source<'a>, TypePath<'a>)>,
    source: Source<'a>,
}

impl<'a> QualifiedPathType<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathType>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, r#type, r#as, greater_than)): (_, (_, _, _, Source)) = (
            into(take(TokenKind::LessThan)),
            Type::parse,
            ignore_recoverable_errors((into(KeywordParser::new("as")), TypePath::parse)),
            into(take(TokenKind::GreaterThan)),
        )
            .parse(tokens)?;

        let mut source: Source = less_than;
        if let Some((r#as, type_path)) = &r#as {
            source = source
                .merge(&r#type.source(), "Type expected after `<`")
                .merge(r#as, "`as` expected after type")
                .merge(&type_path.source(), "Type path expected after `as`");
        } else {
            source = source.merge(&r#type.source(), "Type expected after `<`");
        }
        source = source.merge(&greater_than, "`>` expected after type");

        Ok((
            tokens,
            Self {
                r#type: Box::new(r#type),
                r#as,
                source,
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for QualifiedPathType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.source.span_token();

        if let Some((r#as, type_path)) = &self.r#as {
            let r#type = self.r#type.to_tokens_with_state(state).0;
            let type_path = type_path.to_tokens_with_state(state).0;
            let r#as = Ident::new(r#as.as_str(), r#as.span_token());

            (
                quote_spanned! {span=> < #r#type #r#as #r#type_path > },
                EstimatedLength::new(0),
            )
        } else {
            let r#type = self.r#type.to_tokens_with_state(state).0;

            (
                quote_spanned! {span=> < #r#type > },
                EstimatedLength::new(0),
            )
        }
    }
}
