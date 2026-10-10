#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TokenKind {
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
        whitespace_preference: WhitespaceAdjustmentTagPreference,
    },

    Ident,
    /// `::`
    PathSeparator,
    /// `self`
    SelfCurrentModule,
    /// `Self`
    SelfCurrentType,
    /// `super`
    Super,
    /// `crate`
    Crate,

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

    /// `'`
    Apostrophe,
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
    /// `;`
    Semicolon,
    /// `=`
    Equal,
    /// `@`
    At,

    /// `..`
    RangeExclusive,
    /// `..=`
    RangeInclusive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TagKind {
    Writ,
    Statement,
    Comment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WhitespacePreference {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WhitespaceAdjustmentTagPreference {
    /// Remove all matched whitespace (`{-}`).
    Remove,

    /// Replace all matched whitespace with a single space (`{_}`).
    Replace,
}

impl From<WhitespaceAdjustmentTagPreference> for WhitespacePreference {
    fn from(value: WhitespaceAdjustmentTagPreference) -> Self {
        match value {
            WhitespaceAdjustmentTagPreference::Remove => Self::Remove,
            WhitespaceAdjustmentTagPreference::Replace => Self::Replace,
        }
    }
}
