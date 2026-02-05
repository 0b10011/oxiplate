#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Comment,
    Newline,

    String(String),
    Bool(bool),

    /// `.`
    DotSeparator,

    /// `=`
    Equal,

    /// `[`
    BracketOpen,

    /// `]`
    BracketClose,

    /// `[[`
    DoubleBracketOpen,

    /// `]]`
    DoubleBracketClose,

    /// `{`
    BraceOpen,

    /// `}`
    BraceClose,

    /// `,`
    Comma,
}
