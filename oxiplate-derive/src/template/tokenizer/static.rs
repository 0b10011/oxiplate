use crate::template::tokenizer::kind::WhitespacePreference;
use crate::template::tokenizer::{Context, Res, TagKind, TokenKind, whitespace};
use crate::tokenizer::{BufferedSource, Token};

pub fn consume_static_whitespace<'a>(source: &mut BufferedSource<'a>) -> Res<'a> {
    let source = source
        .consume_while(|char| matches!(char, whitespace!()))
        .expect("Buffer should contain at least one whitespace");

    (
        None,
        Ok(Token::new(TokenKind::StaticWhitespace, &source, None)),
    )
}

#[allow(clippy::unnested_or_patterns)]
pub fn consume_static_text<'a>(source: &mut BufferedSource<'a>) -> Res<'a> {
    let source = source
        .consume_until(|char| matches!(char, '{' | whitespace!()))
        .expect("Buffer should contain at least one character");

    (None, Ok(Token::new(TokenKind::StaticText, &source, None)))
}

pub fn consume_possible_tag_start<'a>(source: &mut BufferedSource<'a>) -> Res<'a> {
    let (new_context, kind) = match source.peek() {
        Some('{') => {
            let _ = source.next();
            (Some(Context::Writ), TagKind::Writ)
        }
        Some('%') => {
            let _ = source.next();
            (Some(Context::Statement), TagKind::Statement)
        }
        Some('#') => {
            let _ = source.next();
            (Some(Context::Comment), TagKind::Comment)
        }
        _ => {
            let (kind, source) = match source.peek_2() {
                Some(['-', '}']) => {
                    let _ = source.next();
                    let _ = source.next();

                    let source = source.consume().expect("Buffer should contain `{-}`");

                    (
                        TokenKind::WhitespaceAdjustmentTag {
                            whitespace_preference: WhitespacePreference::Remove,
                        },
                        source,
                    )
                }
                Some(['_', '}']) => {
                    let _ = source.next();
                    let _ = source.next();

                    let source = source.consume().expect("Buffer should contain `{_}`");
                    (
                        TokenKind::WhitespaceAdjustmentTag {
                            whitespace_preference: WhitespacePreference::Replace,
                        },
                        source,
                    )
                }
                _ => {
                    let source = source
                        .consume()
                        .expect("Buffer should contain at least `{`");
                    (TokenKind::StaticText, source)
                }
            };
            return (None, Ok(Token::new(kind, &source, None)));
        }
    };

    let whitespace_preference = match source.peek() {
        Some('-') => {
            let _ = source.next();
            WhitespacePreference::Remove
        }
        Some('_') => {
            let _ = source.next();
            WhitespacePreference::Replace
        }
        _ => WhitespacePreference::Indifferent,
    };

    let source = source
        .consume()
        .expect("Buffer should contain at least `{`");

    (
        new_context,
        Ok(Token::new(
            TokenKind::TagStart {
                kind,
                whitespace_preference,
            },
            &source,
            None,
        )),
    )
}
