use super::Token;
use crate::Source;
use crate::template::tokenizer::expression::consume_ident;
use crate::template::tokenizer::kind::TokenKind;
use crate::template::tokenizer::{Context, Res};
use crate::tokenizer::{BufferedSource, ParseError, UnexpectedTokenError};

macro_rules! error {
    ($message:expr, $source:ident) => {
        (
            None::<Context>,
            Err(UnexpectedTokenError::new($message, $source)),
        )
    };
}

/// Parse char literal (e.g., `'a'`) or lifetime (e.g., `'a`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
/// See: <https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels>
pub fn consume_char_or_lifetime<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    // Check if this is a lifetime instead of a char.
    match source.peek_2() {
        // Single quoted char.
        Some([_, '\'']) => consume_char(source, leading_whitespace),

        // Possible lifetime, definitely not a char.
        Some(['a'..='z' | 'A'..='Z' | '_', _]) => consume_lifetime(source, leading_whitespace),

        // Invalid syntax; neither a char or lifetime.
        _ => consume_char(source, leading_whitespace),
    }
}

/// Parse char literal (e.g., `'a'`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
pub fn consume_char<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    let char = match source.next() {
        Some('\\') => match source.next() {
            Some(char @ ('\'' | '"' | '\\')) => char,
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some('0') => '\0',
            Some(_char) => {
                let source = source
                    .consume()
                    .expect("Buffer should contain `'` at least");

                return error!(
                    r#"Unknown character escape. Expected `\\`, `\"`, `\'`, `\n`, `\r`, `\t`, or `\0`"#,
                    source
                );
            }
            None => {
                let source = source
                    .consume()
                    .expect("Buffer should contain `'` at least");

                return error!(
                    "End of file encountered while parsing a character literal",
                    source
                );
            }
        },

        // Allow raw newlines, carriage returns, and tabs
        // to avoid having to double escape them in inline templates.
        // they'll be output as escapes in the final template
        // to prevent Rust from complaining.
        Some('\n') => '\n',
        Some('\r') => '\r',
        Some('\t') => '\t',

        Some('\'') => {
            let source = source
                .consume()
                .expect("Buffer should contain `'` at least");

            return error!("No character specified in the character literal", source);
        }

        // If the character isn't followed by `'`,
        // that'll get caught at the end by `parse_char_end()`
        Some(char) => char,

        None => {
            let source = source
                .consume()
                .expect("Buffer should contain `'` at least");

            return error!(
                "End of file encountered while parsing a character literal",
                source
            );
        }
    };

    if let Err(error) = parse_char_end(source) {
        let source = source
            .consume()
            .expect("Buffer should contain `'` at least");

        return error!(error.message(), source);
    }

    let source = source
        .consume()
        .expect("Buffer should contain `'` at least");

    (
        None,
        Ok((
            Token::new(TokenKind::Char(char), &source, leading_whitespace),
            None,
        )),
    )
}

/// Parse lifetime (e.g., `'a`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels>
pub fn consume_lifetime<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    let apostrophe = source.consume().expect("Buffer should contain `'`");
    let (_context, identifier) = consume_ident(source, None);
    let identifier = match identifier {
        Ok((identifier, None)) => identifier,
        Ok((first, Some(second))) => {
            unreachable!(
                "Identifiers should always be a single token. Found {:?} and {:?}",
                first, second
            );
        }
        err @ Err(_) => return (None, err),
    };

    // `'` not allowed immediately following lifetime
    if source.next_if(|char| char == '\'') {
        let trailing_apostrophe = source.consume().expect("Buffer should contain `'`");
        let source = apostrophe
            .merge(identifier.source(), "Identifier should be after `'`")
            .merge(&trailing_apostrophe, "`'` should be after identifier");

        return error!(
            r#"More than one character present in character literal. Consider using `"` instead of `'` to use a string literal instead."#,
            source
        );
    }

    (
        None,
        Ok((
            Token::new(TokenKind::Apostrophe, &apostrophe, leading_whitespace),
            Some(identifier),
        )),
    )
}

fn parse_char_end(source: &mut BufferedSource) -> Result<(), ParseError> {
    match source.next() {
        Some('\'') => Ok(()),
        Some(_) => {
            // Match any extra chars
            source.next_until(|char| char == '\'');

            // Match `'` if not EOF
            if source.next_if(|char| char == '\'') {
                Err(ParseError::new(
                    r#"More than one character present in character literal. Consider using `"` instead of `'` to use a string literal instead."#,
                ))
            } else {
                Err(ParseError::new(
                    "Unclosed character literal. Expected `'` after first character",
                ))
            }
        }
        None => Err(ParseError::new(
            "End of file encountered while parsing a character literal",
        )),
    }
}
