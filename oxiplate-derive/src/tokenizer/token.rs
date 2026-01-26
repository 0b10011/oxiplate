use std::fmt::Debug;

use super::Source;

pub struct Token<'a> {
    source: Source<'a>,
    kind: TokenKind,
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenKind,
        source: &Source<'a>,
        leading_whitespace: Option<Source<'a>>,
    ) -> Self {
        Self {
            source: source.append_to_leading_whitespace(
                leading_whitespace,
                "Token expected after leading whitespace",
            ),
            kind,
        }
    }

    pub fn with_kind(self, kind: TokenKind) -> Self {
        Self { kind, ..self }
    }

    pub fn source(&'a self) -> &'a Source<'a> {
        &self.source
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{}]", self.kind, self.source.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    StaticText,
    StaticWhitespace,

    TagStart {
        kind: TagKind,
        whitespace_preference: WhitespacePreference,
    },
    TagEnd {
        kind: TagKind,
        whitespace_preference: WhitespacePreference,
    },

    Comment,

    /// `{-}` and `{_}`
    WhitespaceAdjustmentTag {
        whitespace_preference: WhitespacePreference,
    },

    Ident,
    /// `::`
    PathSeparator,

    Bool(bool),
    Char(char),
    Integer,
    Float,

    // Strings and raw strings should be fairly rare in templates,
    // but require 24 extra bytes as-is (32 total due to discriminant).
    // Boxing the value reduces this to 8 extra bytes (16 total due to discriminant).
    #[allow(clippy::box_collection)]
    String(Box<String>),
    #[allow(clippy::box_collection)]
    RawString(Box<String>),

    /// `(`
    OpenParenthese,
    /// `)`
    CloseParenthese,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,

    /// `||`
    Or,
    /// `&&`
    And,

    /// `!=`
    NotEq,
    /// `==`
    Eq,
    /// `<=`
    LessThanOrEqualTo,
    /// `<`
    LessThan,
    /// `>=`
    GreaterThanOrEqualTo,
    /// `>`
    GreaterThan,

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Asterisk,
    /// `/`
    ForwardSlash,
    /// `%`
    Percent,
    /// `~`
    Tilde,
    /// `,`
    Comma,
    /// `&`
    Ampersand,
    /// `!`
    Exclamation,
    /// `.`
    Period,
    /// `|`
    VerticalBar,
    /// `:`
    Colon,
    /// `=`
    Equal,

    /// `..`
    RangeExclusive,
    /// `..=`
    RangeInclusive,

    // Parse errors should be non-existent in release builds,
    // but require 16 extra bytes as-is (24 total due to discriminant).
    // Boxing the value reduces this to 8 extra bytes (16 total due to discriminant).
    Unexpected(Box<ParseError>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TagKind {
    Writ,
    Statement,
    Comment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WhitespacePreference {
    /// Remove all matched whitespace.
    /// Tags that suggest this: `{{- foo -}}` and `{-}`
    Remove,

    /// Replace all matched whitespace with a single space.
    /// Tags that suggest this: `{{_ foo _}}` and `{_}`
    Replace,

    /// Rely on surrounding tags to make a decision,
    /// otherwise leave the whitespace unchanged.
    /// Tag that suggests this: `{{ foo }}`
    Indifferent,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    message: &'static str,
}

impl ParseError {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }

    pub fn boxed(message: &'static str) -> Box<Self> {
        Box::new(Self { message })
    }

    pub fn message(&self) -> &str {
        self.message
    }
}
