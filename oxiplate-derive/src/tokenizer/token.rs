use std::fmt::Debug;

use super::Source;

pub struct Token<'a, K> {
    source: Source<'a>,
    kind: K,
}

impl<'a, K> Token<'a, K> {
    pub fn new(kind: K, source: &Source<'a>, leading_whitespace: Option<Source<'a>>) -> Self {
        Self {
            source: source.append_to_leading_whitespace(
                leading_whitespace,
                "Token expected after leading whitespace",
            ),
            kind,
        }
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn kind(&self) -> &K {
        &self.kind
    }
}

impl<K: Debug> Debug for Token<'_, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{}]", self.kind, self.source.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    message: &'static str,
}

impl ParseError {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}
