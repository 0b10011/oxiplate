use proc_macro2::Literal;

use crate::template::parser::expression::Expression;
use crate::template::parser::prelude::*;

#[derive(Debug)]
pub(in crate::template::parser) struct Integer<'a> {
    source: &'a Source<'a>,
    literal: Literal,
}

impl<'a> Integer<'a> {
    #[cfg(test)]
    pub(in crate::template::parser) fn new_for_test(source: &'a Source<'a>) -> Self {
        let literal: Literal = source
            .as_str()
            .parse()
            .expect("Should have parsed as a literal");

        Self { source, literal }
    }

    /// See: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, source): (_, &Source) = (alt((
            into(take(TokenKind::DecLiteral)),
            into(take(TokenKind::BinLiteral)),
            into(take(TokenKind::OctLiteral)),
            into(take(TokenKind::HexLiteral)),
        )))
        .parse(tokens)?;

        let mut literal = match source.as_str().parse::<Literal>() {
            Ok(literal) => literal,
            Err(error) => {
                #[cfg(not(feature = "better-internal-errors"))]
                let _ = error;

                internal_error!(
                    source.span_token().unwrap(),
                    "Failed to parse integer",
                    .span_help(error.span().unwrap(), format!("Original error: {error}")
                ));
            }
        };
        literal.set_span(source.span_token());

        Ok((tokens, Self { source, literal }))
    }

    pub(in crate::template::parser) fn source(&self) -> &Source<'a> {
        self.source
    }
}

impl<'a> ToTokensWithState<'a> for Integer<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        (
            self.literal.to_token_stream(),
            EstimatedLength::new(self.source.as_str().len()),
        )
    }
}

impl<'a> From<Integer<'a>> for Number<'a> {
    fn from(value: Integer<'a>) -> Self {
        Number::Integer(value)
    }
}

#[derive(Debug)]
pub(in crate::template::parser) struct Float<'a> {
    source: Source<'a>,
}

impl<'a> Float<'a> {
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, float) = take(TokenKind::Float).parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: float.source().clone(),
            },
        ))
    }

    pub(in crate::template::parser) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for Float<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let literal = ::syn::LitFloat::new(self.source.as_str(), self.source.span_token());
        (
            quote! { #literal },
            EstimatedLength::new(self.source.as_str().len()),
        )
    }
}

impl<'a> From<Float<'a>> for Number<'a> {
    fn from(value: Float<'a>) -> Self {
        Number::Float(value)
    }
}

pub(crate) enum Number<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
}

impl<'a> Number<'a> {
    /// Parse a number.
    /// See: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((into(Integer::parse), into(Float::parse))).parse(tokens)
    }
}

impl<'a> From<Number<'a>> for Expression<'a> {
    fn from(value: Number<'a>) -> Self {
        match value {
            Number::Integer(integer) => Expression::Integer(integer),
            Number::Float(float) => Expression::Float(float),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::source::test_source;
    use crate::template::tokenizer::Token;
    use crate::tokenizer::Eof;

    #[test]
    #[should_panic = "internal error: entered unreachable code: Internal Oxiplate error. Enable \
                      `better-internal-errors` feature for an easier-to-debug error message. \
                      Error: Failed to parse integer"]
    fn malformed_number() {
        test_source!(source = "19 89");

        Integer::parse
            .parse(TokenSlice::new(
                &[Ok(Token::new(TokenKind::DecLiteral, &source, None))],
                &Eof::for_test(source),
            ))
            .unwrap();
    }
}
