use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::types::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/pointer.html#railroad-RawPointerType>
#[derive(Debug)]
pub(crate) enum RawPointerType<'a> {
    Mut {
        deref: Source<'a>,
        r#mut: Source<'a>,
        type_no_bounds: Box<TypeNoBounds<'a>>,
    },
    Const {
        deref: Source<'a>,
        r#const: Source<'a>,
        type_no_bounds: Box<TypeNoBounds<'a>>,
    },
}

impl<'a> RawPointerType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/pointer.html#railroad-RawPointerType>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (deref, keyword, type_no_bounds)): (_, (_, Source<'a>, _)) = (
            into(take(TokenKind::Asterisk)),
            alt((
                into(KeywordParser::new("mut")),
                into(KeywordParser::new("const")),
            )),
            TypeNoBounds::parse,
        )
            .parse(tokens)?;

        let raw_pointer = match keyword.as_str() {
            "mut" => Self::Mut {
                deref,
                r#mut: keyword,
                type_no_bounds: Box::new(type_no_bounds),
            },
            "const" => Self::Const {
                deref,
                r#const: keyword,
                type_no_bounds: Box::new(type_no_bounds),
            },
            _ => unreachable!(
                "Internal Oxiplate error: Unexpected keyword parsed while parsing raw pointer"
            ),
        };

        Ok((tokens, raw_pointer))
    }

    pub(crate) fn source(&self) -> Source<'a> {
        let (deref, keyword, type_no_bounds) = match &self {
            Self::Mut {
                deref,
                r#mut,
                type_no_bounds,
            } => (deref, r#mut, type_no_bounds),
            Self::Const {
                deref,
                r#const,
                type_no_bounds,
            } => (deref, r#const, type_no_bounds),
        };

        deref
            .clone()
            .merge(keyword, "`mut` or `const` expected after `*`")
            .merge(
                &type_no_bounds.source(),
                "Type no bounds expected after `mut` or `const`",
            )
    }
}

impl<'a> ToTokensWithState<'a> for RawPointerType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        match &self {
            Self::Mut {
                deref,
                r#mut,
                type_no_bounds,
            } => {
                let span = deref.span_token();
                tokens.append_all(quote_spanned! {span=> * });
                let span = r#mut.span_token();
                tokens.append_all(quote_spanned! {span=> mut });
                let (type_no_bounds, _estimated_length) =
                    type_no_bounds.to_tokens_with_state(state);
                tokens.append_all(type_no_bounds);
            }
            Self::Const {
                deref,
                r#const,
                type_no_bounds,
            } => {
                let span = deref.span_token();
                tokens.append_all(quote_spanned! {span=> * });
                let span = r#const.span_token();
                tokens.append_all(quote_spanned! {span=> const });
                let (type_no_bounds, _estimated_length) =
                    type_no_bounds.to_tokens_with_state(state);
                tokens.append_all(type_no_bounds);
            }
        }

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<RawPointerType<'a>> for TypeNoBounds<'a> {
    fn from(value: RawPointerType<'a>) -> Self {
        TypeNoBounds::RawPointer(value)
    }
}
