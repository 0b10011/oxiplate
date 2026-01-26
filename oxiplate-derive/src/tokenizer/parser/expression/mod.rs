mod char;
mod number;
mod string;

use crate::Source;
use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::parser::expression::char::consume_char;
use crate::tokenizer::parser::expression::number::{consume_alternative_base, consume_decimal};
use crate::tokenizer::parser::expression::string::{consume_raw_string, consume_string};
use crate::tokenizer::parser::{
    consume_possible_tag_end, consume_possible_tag_end_whitespace_adjustment,
};
use crate::tokenizer::{
    Context, ParseError, TagKind, Token, TokenKind, WhitespacePreference, whitespace,
};

#[allow(clippy::too_many_lines)]
pub fn consume_expression_token<'a>(
    source: &mut BufferedSource<'a>,
    has_unclosed_char_pairs: bool,
    in_tag_kind: &TagKind,
) -> (Option<Context>, Token<'a>) {
    let leading_whitespace = source
        .consume_while(|char| matches!(char, whitespace!()))
        .ok();

    macro_rules! if_matches {
        ($char:literal => $if:ident else $else:ident) => {{
            if source.next_if(|char| char == $char) {
                TokenKind::$if
            } else {
                TokenKind::$else
            }
        }};
    }

    let kind = match source.next() {
        Some('"') => return consume_string(source, leading_whitespace),
        Some('#') => return consume_raw_string(source, leading_whitespace),
        Some('\'') => return consume_char(source, leading_whitespace),
        Some('}') => {
            return consume_possible_tag_end(
                source,
                leading_whitespace,
                TagKind::Writ,
                in_tag_kind,
                has_unclosed_char_pairs,
            );
        }
        Some('%') => {
            return consume_possible_tag_end(
                source,
                leading_whitespace,
                TagKind::Statement,
                in_tag_kind,
                has_unclosed_char_pairs,
            );
        }
        Some('(') => TokenKind::OpenParenthese,
        Some(')') => TokenKind::CloseParenthese,
        Some('[') => TokenKind::OpenBracket,
        Some(']') => TokenKind::CloseBracket,
        Some('{') => TokenKind::OpenBrace,
        Some('+') => TokenKind::Plus,
        Some('-') => {
            return consume_possible_tag_end_whitespace_adjustment(
                source,
                leading_whitespace,
                in_tag_kind,
                has_unclosed_char_pairs,
                WhitespacePreference::Remove,
            );
        }
        Some('_') => {
            return consume_possible_tag_end_whitespace_adjustment(
                source,
                leading_whitespace,
                in_tag_kind,
                has_unclosed_char_pairs,
                WhitespacePreference::Replace,
            );
        }
        Some('*') => TokenKind::Asterisk,
        Some('/') => TokenKind::ForwardSlash,
        Some('~') => TokenKind::Tilde,
        Some(',') => TokenKind::Comma,
        Some(':') => if_matches!(':' => PathSeparator else Colon),
        Some('&') => if_matches!('&' => And else Ampersand),
        Some('!') => if_matches!('=' => NotEq else Exclamation),
        Some('=') => if_matches!('=' => Eq else Equal),
        Some('<') => if_matches!('=' => LessThanOrEqualTo else LessThan),
        Some('>') => if_matches!('=' => GreaterThanOrEqualTo else GreaterThan),
        Some('|') => if_matches!('|' => Or else VerticalBar),
        Some('.') => {
            if source.next_if(|char| char == '.') {
                if_matches!('=' => RangeInclusive else RangeExclusive)
            } else {
                TokenKind::Period
            }
        }
        Some('a'..='z' | 'A'..='Z') => return consume_ident(source, leading_whitespace),
        Some('0') => return consume_alternative_base(source, leading_whitespace),
        Some('1'..='9') => return consume_decimal(source, leading_whitespace),
        Some(_) => TokenKind::Unexpected(ParseError::boxed("Unexpected character in expression")),
        None => {
            let message = match in_tag_kind {
                TagKind::Writ => {
                    "End of file encountered while parsing a writ. Expected `}}`, `-}}`, or `_}}`"
                }
                TagKind::Statement => {
                    "End of file encountered while parsing a statement. Expected `%}`, `-%}`, or \
                     `_%}`"
                }
                TagKind::Comment => {
                    unreachable!("Expressions should not be parsed in comments")
                }
            };

            return (
                Some(Context::Static),
                Token::new(
                    TokenKind::Unexpected(ParseError::boxed(message)),
                    source.eof().source(),
                    None,
                ),
            );
        }
    };

    let source = source
        .consume()
        .expect("Buffer should contain at least one character");

    (None, Token::new(kind, &source, leading_whitespace))
}

pub fn consume_ident<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> (Option<Context>, Token<'a>) {
    let source = source
        .consume_while(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
        .expect("Buffer should contain at least one character");

    let kind = match source.as_str() {
        "true" => TokenKind::Bool(true),
        "false" => TokenKind::Bool(false),
        _ => TokenKind::Ident,
    };

    (None, Token::new(kind, &source, leading_whitespace))
}
