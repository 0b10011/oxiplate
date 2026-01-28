use super::Token;
use crate::Source;
use crate::syntax::UnexpectedTokenError;
use crate::tokenizer::ParseError;
use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::parser::{Res, TokenKind};

fn parse_string(source: &mut BufferedSource) -> Result<String, ParseError> {
    let mut string = String::new();
    while let Some(char) = source.next() {
        if char == '"' {
            return Ok(string);
        }

        string.push(char);
    }

    Err(ParseError::new("Unclosed string"))
}

pub fn consume_string<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    match parse_string(source) {
        Ok(string) => {
            let source = source
                .consume()
                .expect("Buffer should contain `\"` at least");

            (
                None,
                Ok(Token::new(
                    TokenKind::String(Box::new(string)),
                    &source,
                    leading_whitespace,
                )),
            )
        }
        Err(parse_error) => {
            let source = source
                .consume()
                .expect("Buffer should contain `\"` at least")
                .append_to_leading_whitespace(
                    leading_whitespace,
                    "String expected after whitespace",
                );

            (
                None,
                Err(UnexpectedTokenError::new(parse_error.message(), source)),
            )
        }
    }
}

pub fn consume_raw_string<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    let mut string = String::new();

    let opening_hashes = 1 + source.next_while(|char| char == '#');

    if !source.next_if(|char| char == '"') {
        let source = source
            .consume()
            .expect("At least one `#` should be in the buffer")
            .append_to_leading_whitespace(
                leading_whitespace,
                "Hashes should follow leading whitespace",
            );

        return (
            None,
            Err(UnexpectedTokenError::new(
                "Malformed raw string. Expected `\"` after opening hashes",
                source,
            )),
        );
    }

    while let Some(char) = source.next() {
        match char {
            '"' => {
                // Check if hashes after the `"` match the hashes before the string
                let closing_hashes = source.next_while(|char| char == '#');

                if closing_hashes == opening_hashes {
                    let source = source
                        .consume()
                        .expect("Full string should be in the buffer");
                    return (
                        None,
                        Ok(Token::new(
                            TokenKind::RawString(Box::new(string)),
                            &source,
                            leading_whitespace,
                        )),
                    );
                } else if closing_hashes > opening_hashes {
                    let source = source
                        .consume()
                        .expect("Full string plus extra hashes should be in the buffer")
                        .append_to_leading_whitespace(
                            leading_whitespace,
                            "Raw string should follow leading whitespace",
                        );

                    return (
                        None,
                        Err(UnexpectedTokenError::new(
                            r"The number of hashes (`#`) before and after the string should be the same",
                            source,
                        )),
                    );
                }

                // Append the `"` and hashes to the string
                string.push('"');
                string.push_str(&"#".repeat(closing_hashes));
            }
            _ => {
                string.push(char);
            }
        }
    }

    let source = source
        .consume()
        .expect("At least one `#` should be in the buffer")
        .append_to_leading_whitespace(
            leading_whitespace,
            "Raw string should follow leading whitespace",
        );

    (
        None,
        Err(UnexpectedTokenError::new("Raw string never closed", source)),
    )
}
