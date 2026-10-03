use std::collections::HashSet;

use super::Pattern;
use crate::template::parser::prelude::*;

#[derive(Debug)]
pub(crate) struct Tuple<'a> {
    /// `Source` is the trailing comma.
    values: Vec<(Pattern<'a>, Source<'a>)>,
    last_value: Option<Box<Pattern<'a>>>,
    /// `..`
    #[allow(dead_code)]
    remaining: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> Tuple<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (open_paren, (fields_and_commas, last_field, close_paren))) = (
            take(TokenKind::OpenParenthese),
            (
                many1((
                    context("Expected a pattern", Pattern::parse),
                    context("Expected `,`", take(TokenKind::Comma)),
                )),
                ignore_recoverable_errors(context("Expected a pattern", Pattern::parse)),
                cut("Expected `)`", take(TokenKind::CloseParenthese)),
            ),
        )
            .parse(tokens)?;

        let mut source = open_paren.source().clone();

        let mut values = Vec::with_capacity(fields_and_commas.len());
        for (field, comma) in fields_and_commas {
            source = source
                .merge(field.source(), "Field expected")
                .merge(comma.source(), "Comma expected after previous field");
            values.push((field, comma.source().clone()));
        }

        let last_value = if let Some(last_field) = last_field {
            source = source.merge(last_field.source(), "Field expected after comma");

            Some(Box::new(last_field))
        } else {
            None
        };

        source = source.merge(
            close_paren.source(),
            "Closing brace expected after whitespace",
        );

        Ok((
            tokens,
            Self {
                values,
                last_value,
                remaining: None,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars = HashSet::new();

        for (value, _comma) in &self.values {
            vars.extend(value.get_variables());
        }

        if let Some(last_value) = &self.last_value {
            vars.extend(last_value.get_variables());
        }

        vars
    }
}

impl<'a> ToTokensWithState<'a> for Tuple<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();
        let mut estimated_length = EstimatedLength::new(0);

        for (value, comma) in &self.values {
            let comma_span = comma.span_token();
            let comma = quote_spanned! {comma_span=> , };
            let (value, value_estimated_length) = value.to_tokens_with_state(state);
            estimated_length += value_estimated_length;

            tokens.append_all([value, comma]);
        }

        if let Some(last_value) = &self.last_value {
            let (last_value, _estimated_length) = last_value.to_tokens_with_state(state);
            tokens.append_all(last_value);
        }

        let span = self.source.span_token();

        (quote_spanned! {span=> (#tokens) }, estimated_length)
    }
}

impl<'a> From<Tuple<'a>> for Pattern<'a> {
    fn from(value: Tuple<'a>) -> Self {
        Pattern::Tuple(value)
    }
}
