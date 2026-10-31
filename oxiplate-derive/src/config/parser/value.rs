use std::marker::PhantomData;

use super::expression::Key;
use super::prelude::*;
use crate::OptimizedRenderer;
use crate::config::InferEscaperGroupFromFileExtension;

#[derive(Debug)]
pub(in crate::config::parser) enum Value<'a> {
    Bool(BoolValue<'a>),
    String(StringValue<'a>),
}

impl<'a> Value<'a> {
    pub(crate) fn source(&self) -> &'a Source<'a> {
        match self {
            Self::Bool(bool_value) => bool_value.source(),
            Self::String(string_value) => string_value.source(),
        }
    }
}

pub(in crate::config::parser) fn value<'a>() -> ValueParser<'a> {
    ValueParser {
        phantom_data: PhantomData,
    }
}
pub(in crate::config::parser) struct ValueParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ValueParser<'a> {
    type TokenKind = TokenKind;
    type Output = Value<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, token) = tokens.take()?;

        let value = match token.kind() {
            TokenKind::String(value) => Value::String(StringValue {
                value,
                source: token.source(),
            }),
            TokenKind::Bool(value) => Value::Bool(BoolValue {
                value: *value,
                source: token.source(),
            }),
            _ => {
                return Err(Error::recoverable(
                    format!("Expected a string or boolean value, found: {token:#?}"),
                    token.source().clone(),
                ));
            }
        };

        Ok((tokens, value))
    }
}

#[derive(Debug)]
pub(in crate::config::parser) struct StringValue<'a> {
    value: &'a str,
    source: &'a Source<'a>,
}

impl<'a> StringValue<'a> {
    pub(in crate::config::parser) fn new(value: &'a str, source: &'a Source<'a>) -> Self {
        Self { value, source }
    }

    pub(in crate::config::parser) fn value(&self) -> &'a str {
        self.value
    }

    pub(crate) fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

impl<'a> From<StringValue<'a>> for Key<'a> {
    fn from(value: StringValue<'a>) -> Self {
        let StringValue { value, source } = value;

        Self::new(value, source)
    }
}

impl<'a> From<StringValue<'a>> for Option<String> {
    fn from(value: StringValue<'a>) -> Self {
        let StringValue { value, source: _ } = value;

        Some(value.to_owned())
    }
}

#[derive(Debug)]
struct BoolValue<'a> {
    value: bool,
    source: &'a Source<'a>,
}

impl<'a> BoolValue<'a> {
    pub(crate) fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

impl<'a> From<BoolValue<'a>> for bool {
    fn from(value: BoolValue<'a>) -> Self {
        let BoolValue { value, source: _ } = value;

        value
    }
}

impl<'a> From<BoolValue<'a>> for InferEscaperGroupFromFileExtension {
    fn from(value: BoolValue<'a>) -> Self {
        let BoolValue { value, source: _ } = value;

        Self(value)
    }
}

impl<'a> From<BoolValue<'a>> for OptimizedRenderer {
    fn from(value: BoolValue<'a>) -> Self {
        let BoolValue { value, source: _ } = value;

        Self(value)
    }
}
