pub use self::kind::TokenKind;
use super::Token;
use crate::tokenizer::{BufferedSource, Eof, UnexpectedTokenError};
use crate::Source;

type Res<'a> = Result<Token<'a>, UnexpectedTokenError<'a>>;

/// See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L29-L31>
macro_rules! whitespace {
    () => {
        '\u{0009}' // (horizontal tab, '\t')
        | '\u{0020}' // (space, ' ')
    };
}

// See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L41>
macro_rules! non_ascii {
    () => {
        '\u{0080}' ..= '\u{d7ff}' | '\u{e000}' ..= '\u{10ffff}'
    };
}

// See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L247>
macro_rules! hex {
    () => {
        'a' ..= 'f' | 'A' ..= 'F' | '0' ..= '9'
    };
}

mod kind;

pub fn tokens_and_eof(source: Source) -> (Vec<Result<Token, UnexpectedTokenError>>, Eof) {
    let tokens = Tokens::new(source);
    let eof = tokens.source.eof();

    (tokens.collect(), eof)
}

#[derive(Debug)]
pub struct Tokens<'a> {
    source: BufferedSource<'a>,
    char_pair_stack: Vec<CharPairKind>,
}

impl<'a> Tokens<'a> {
    pub fn new(template: Source<'a>) -> Self {
        Self {
            source: template.into(),
            char_pair_stack: vec![],
        }
    }

    fn consume_next_item(
        &mut self,
        leading_whitespace: Option<Source<'a>>,
    ) -> Option<Result<Token<'a>, UnexpectedTokenError<'a>>> {
        macro_rules! consume_and_return_token {
            ($kind:ident) => {
                Ok(Token::new(
                    TokenKind::$kind,
                    &self.source.consume().expect("Buffer should contain a char"),
                    leading_whitespace,
                ))
            };
        }

        let item = match self.source.next()? {
            '#' => consume_comment(&mut self.source, leading_whitespace),
            '\n' => consume_and_return_token!(Newline),
            '\r' if self.source.peek() == Some('\n') => {
                // Match newline
                let _ = self.source.next();

                consume_and_return_token!(Newline)
            }
            '"' => consume_basic_string(&mut self.source, leading_whitespace),
            '\'' => consume_literal_string(&mut self.source, leading_whitespace),

            '=' => consume_and_return_token!(Equal),
            '[' => {
                if self.source.peek() == Some('[') {
                    let _ = self.source.next();
                    consume_and_return_token!(DoubleBracketOpen)
                } else {
                    consume_and_return_token!(BracketOpen)
                }
            }
            ']' => {
                if self.source.peek() == Some(']') {
                    let _ = self.source.next();
                    consume_and_return_token!(DoubleBracketClose)
                } else {
                    consume_and_return_token!(BracketClose)
                }
            }
            '{' => consume_and_return_token!(BraceOpen),
            '}' => consume_and_return_token!(BraceClose),
            ',' => consume_and_return_token!(Comma),
            '.' => consume_and_return_token!(DotSeparator),

            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
                let source = self
                    .source
                    .consume_while(
                        |char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'),
                    )
                    .expect("Buffer should contain at least one char");

                let value = source.as_str();
                let kind = match value {
                    "true" => TokenKind::Bool(true),
                    "false" => TokenKind::Bool(false),
                    _ => TokenKind::String(value.to_owned()),
                };

                Ok(Token::new(kind, &source, leading_whitespace))
            }
            _ => Err(UnexpectedTokenError::new(
                "Unexpected token found",
                self.source
                    .consume()
                    .expect("Buffer should contain one character"),
            )),
        };

        Some(item)
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, UnexpectedTokenError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let leading_whitespace = self
            .source
            .consume_while(|char| matches!(char, whitespace!()))
            .ok();

        let token: Self::Item = self.consume_next_item(leading_whitespace)?;

        let token = match token {
            Ok(token) => token,
            err => return Some(err),
        };

        // Ensure all char pairs are matched.
        let char_pair_check = match token.kind() {
            TokenKind::BraceOpen => {
                self.char_pair_stack.push(CharPairKind::Brace);
                None
            }
            TokenKind::BracketOpen => {
                self.char_pair_stack.push(CharPairKind::Bracket);
                None
            }
            TokenKind::DoubleBracketOpen => {
                self.char_pair_stack.push(CharPairKind::DoubleBracket);
                None
            }
            TokenKind::BraceClose => Some((
                matches!(self.char_pair_stack.last(), Some(CharPairKind::Brace)),
                "Expected `}`",
            )),
            TokenKind::BracketClose => Some((
                matches!(self.char_pair_stack.last(), Some(CharPairKind::Bracket)),
                "Expected `]`",
            )),
            TokenKind::DoubleBracketClose => Some((
                matches!(
                    self.char_pair_stack.last(),
                    Some(CharPairKind::DoubleBracket)
                ),
                "Expected `)`",
            )),
            _ => None,
        };

        if let Some((char_pair_matched, error_message)) = char_pair_check {
            if char_pair_matched {
                self.char_pair_stack.pop();
            } else {
                return Some(Err(UnexpectedTokenError::new(
                    error_message,
                    token.source().clone(),
                )));
            }
        }

        Some(Ok(token))
    }
}

#[derive(Debug)]
enum CharPairKind {
    /// `{` and `}`
    Brace,

    /// `[` and `]`
    Bracket,

    /// `[[` and `]]`
    DoubleBracket,
}

/// See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L44>
#[allow(clippy::unnecessary_wraps)]
fn consume_comment<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    #[allow(clippy::unnested_or_patterns)]
    let source = source
        .consume_while(|char| matches!(char, '\u{0009}' | '\u{0020}'..='\u{007e}' | non_ascii!()))
        .expect("Buffer should at least contain `#`");

    Ok(Token::new(TokenKind::Comment, &source, leading_whitespace))
}

/// See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L69>
#[allow(clippy::many_single_char_names)]
#[allow(clippy::too_many_lines)]
fn consume_basic_string<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    let mut value = String::new();

    macro_rules! next_and_push {
        ($char:literal) => {{
            let _ = source.next();
            value.push($char);
        }};
    }

    macro_rules! bad_hex {
        ($count:literal, $escape_character:literal) => {{
            source.next_until(|char| char == '"');

            let message = if source.next_if(|char| char == '"') {
                concat!(
                    $count,
                    " hex characters (`[0-9a-fA-f]{",
                    $count,
                    "}`) expected after `\\",
                    $escape_character,
                    "`"
                )
            } else {
                "End of file encountered while parsing basic string"
            };

            return Err(UnexpectedTokenError::new(
                message,
                source
                    .consume()
                    .expect("Buffer should contain beginning of string"),
            ));
        }};
    }

    macro_rules! hex_chars {
        ($count:literal, $peek_fn:ident, $($char:ident),+) => {
            {
                let _ = source.next();
                let mut hex = String::new();
                let Some([$($char @ (hex!())),*]) = source.$peek_fn() else {
                    bad_hex!($count, 'x');
                };

                $(
                    let _ = source.next();
                    hex.push($char);
                )+
            }
        };
    }

    while let Some(char) = source.next() {
        match char {
            '"' => {
                return Ok(Token::new(
                    TokenKind::String(value),
                    &source
                        .consume()
                        .expect("Buffer should contain a quoted string"),
                    leading_whitespace,
                ));
            }
            '\\' => match source.peek() {
                Some('"') => next_and_push!('"'),
                Some('\\') => next_and_push!('\\'),
                Some('b') => next_and_push!('\u{0008}'),
                Some('e') => next_and_push!('\u{001b}'),
                Some('f') => next_and_push!('\u{000c}'),
                Some('n') => next_and_push!('\n'),
                Some('r') => next_and_push!('\r'),
                Some('t') => next_and_push!('\t'),
                // See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L86>
                Some('x') => hex_chars!(2, peek_2, a, b),
                // See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L87>
                Some('u') => hex_chars!(4, peek_4, a, b, c, d),
                // See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L88>
                Some('U') => hex_chars!(8, peek_8, a, b, c, d, e, f, g, h),
                _ => {
                    source.next_until(|char| char == '"');

                    let message = if source.next_if(|char| char == '"') {
                        r#"Unexpected escape sequence. Expected `\"`, `\\`, `\b`, `\e`, `\f`, `\n`, `\r`, `\t`, `\x[0-9a-fA-F]{2}`, `\u[0-9a-fA-F]{4}`, or `\U[0-9a-fA-F]{8}`"#
                    } else {
                        "End of file encountered while parsing basic string"
                    };

                    return Err(UnexpectedTokenError::new(
                        message,
                        source
                            .consume()
                            .expect("Buffer should contain beginning of string"),
                    ));
                }
            },
            #[allow(clippy::unnested_or_patterns)]
            whitespace!()
            | '\u{0021}'
            | '\u{0023}'..='\u{005b}'
            | '\u{005d}'..='\u{007e}'
            | non_ascii!() => value.push(char),
            _ => {
                source.next_until(|char| char == '"');

                let message = if source.next_if(|char| char == '"') {
                    "Unexpected character in basic string"
                } else {
                    "End of file encountered while parsing basic string"
                };

                return Err(UnexpectedTokenError::new(
                    message,
                    source.consume().expect("Buffer should contain one char"),
                ));
            }
        }
    }

    Err(UnexpectedTokenError::new(
        "End of file encountered while parsing basic string",
        source
            .consume()
            .expect("Buffer should contain `\"` followed by zero or more characters"),
    ))
}

/// See: <https://github.com/toml-lang/toml/blob/bcbbd1c1f03473ffe97b8bf26a0fc945efe2b4a1/toml.abnf#L103>
fn consume_literal_string<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    source.next_while(|char| matches!(char, '\u{0009}' | '\u{0020}' ..= '\u{0026}' | '\u{0028}' ..= '\u{007e}' | '\u{e000}' ..= '\u{10ffff}'));

    if source.next_if(|char| char == '\'') {
        let source = source
            .consume()
            .expect("Buffer should contain a quoted string");
        Ok(Token::new(
            TokenKind::String(
                source
                    .as_str()
                    .strip_prefix('\'')
                    .expect("Should start with `'`")
                    .strip_suffix('\'')
                    .expect("Should end with `'`")
                    .to_owned(),
            ),
            &source,
            leading_whitespace,
        ))
    } else {
        Err(UnexpectedTokenError::new(
            "Expected `'` to end literal string",
            source
                .consume()
                .expect("Buffer should contain `'` followed by zero or more characters"),
        ))
    }
}
