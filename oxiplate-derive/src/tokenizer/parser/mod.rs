pub mod comment;
pub mod expression;
pub mod r#static;

use crate::Source;
use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::parser::expression::consume_ident;
use crate::tokenizer::{Context, ParseError, TagKind, Token, TokenKind, WhitespacePreference};

/// See: <https://doc.rust-lang.org/reference/whitespace.html>
macro_rules! whitespace {
    () => {
        '\u{0009}' // (horizontal tab, '\t')
        | '\u{000A}' // (line feed, '\n')
        | '\u{000B}' // (vertical tab)
        | '\u{000C}' // (form feed)
        | '\u{000D}' // (carriage return, '\r')
        | '\u{0020}' // (space, ' ')
        | '\u{0085}' // (next line)
        | '\u{200E}' // (left-to-right mark)
        | '\u{200F}' // (right-to-left mark)
        | '\u{2028}' // (line separator)
        | '\u{2029}' // (paragraph separator)
    };
}

pub(super) use whitespace;

pub fn consume_possible_tag_end<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
    tag_end_kind: TagKind,
    in_tag_kind: &TagKind,
    has_unclosed_char_pairs: bool,
) -> (Option<Context>, Token<'a>) {
    if has_unclosed_char_pairs || source.peek() != Some('}') {
        let source = source
            .consume()
            .expect("Buffer should contain `}`, `%`, or `#`");

        let kind = match tag_end_kind {
            TagKind::Writ => TokenKind::CloseBrace,
            TagKind::Statement => TokenKind::Percent,
            TagKind::Comment => TokenKind::Comment,
        };

        return (None, Token::new(kind, &source, leading_whitespace));
    }

    let _ = source.next();
    let source = source
        .consume()
        .expect("Buffer should contain `}}`, `%}`, or `#}`");

    if tag_end_kind == *in_tag_kind {
        // Ending current tag
        (
            Some(Context::Static),
            Token::new(
                TokenKind::TagEnd {
                    kind: tag_end_kind,
                    whitespace_preference: WhitespacePreference::Indifferent,
                },
                &source,
                leading_whitespace,
            ),
        )
    } else {
        // Ending wrong tag
        let message = match in_tag_kind {
            TagKind::Writ => "Expected `}}`, `-}}`, or `_}}`",
            TagKind::Statement => "Expected `%}`, `-%}`, or `_%}`",
            TagKind::Comment => {
                unreachable!("Comment should treat anything other than `#}}` as comment text")
            }
        };
        (
            None,
            Token::new(
                TokenKind::Unexpected(ParseError::boxed(message)),
                &source,
                leading_whitespace,
            ),
        )
    }
}

pub fn consume_possible_tag_end_whitespace_adjustment<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
    in_tag_kind: &TagKind,
    has_unclosed_char_pairs: bool,
    whitespace_preference: WhitespacePreference,
) -> (Option<Context>, Token<'a>) {
    let tag_end_kind = match source.peek_2() {
        Some(['}', '}']) => Some(TagKind::Writ),
        Some(['%', '}']) => Some(TagKind::Statement),
        Some(['#', '}']) => Some(TagKind::Comment),
        _ => None,
    };

    if has_unclosed_char_pairs || tag_end_kind.is_none() {
        #[allow(clippy::enum_glob_use)]
        use TagKind::*;
        #[allow(clippy::enum_glob_use)]
        use WhitespacePreference::*;
        let kind = match (whitespace_preference, in_tag_kind) {
            (Indifferent, _) => unreachable!(
                "Tag end parser for whitespace adjustment should never be called when indifferent \
                 about whitespace"
            ),
            (Remove, Writ | Statement) => TokenKind::Minus,
            (Replace, Writ | Statement) => {
                return consume_ident(source, leading_whitespace);
            }
            (Remove | Replace, Comment) => TokenKind::Comment,
        };

        let source = source.consume().expect("Buffer should contain `-` or `_`");

        return (None, Token::new(kind, &source, leading_whitespace));
    }

    let _ = source.next();
    let _ = source.next();

    let tag_end_kind = tag_end_kind.expect("`None` should have just been checked for");
    let source = source
        .consume()
        .expect("Buffer should contain `_}}`, `_%}`, `_#}`, `-}}`, `-%}`, or `-#}`");

    if tag_end_kind == *in_tag_kind {
        // Ending current tag
        (
            Some(Context::Static),
            Token::new(
                TokenKind::TagEnd {
                    kind: tag_end_kind,
                    whitespace_preference,
                },
                &source,
                leading_whitespace,
            ),
        )
    } else {
        // Ending wrong tag
        let message = match in_tag_kind {
            TagKind::Writ => "Expected `}}`, `-}}`, or `_}}`",
            TagKind::Statement => "Expected `%}`, `-%}`, or `_%}`",
            TagKind::Comment => {
                unreachable!("Comment should treat anything other than `#}}` as comment text")
            }
        };
        (
            None,
            Token::new(
                TokenKind::Unexpected(ParseError::boxed(message)),
                &source,
                leading_whitespace,
            ),
        )
    }
}
