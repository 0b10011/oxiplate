mod char;
mod number;
mod string;

use self::char::consume_char_or_lifetime;
use self::number::{consume_alternative_base, consume_decimal};
use self::string::{consume_raw_string, consume_string};
use super::{
    Res, TagKind, Token, TokenKind, WhitespacePreference, consume_possible_tag_end,
    consume_possible_tag_end_whitespace_adjustment, whitespace,
};
use crate::Source;
use crate::tokenizer::{BufferedSource, UnexpectedTokenError};

#[allow(clippy::too_many_lines)]
pub(crate) fn consume_expression_token<'a>(
    source: &mut BufferedSource<'a>,
    has_unclosed_char_pairs: bool,
    in_tag_kind: &TagKind,
) -> Option<Res<'a>> {
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

    let kind = match source.next()? {
        '"' => return Some(consume_string(source, leading_whitespace)),
        '#' => return Some(consume_raw_string(source, leading_whitespace)),
        '\'' => return Some(consume_char_or_lifetime(source, leading_whitespace)),
        '}' => {
            return Some(consume_possible_tag_end(
                source,
                leading_whitespace,
                TagKind::Writ,
                has_unclosed_char_pairs,
            ));
        }
        '%' => {
            return Some(consume_possible_tag_end(
                source,
                leading_whitespace,
                TagKind::Statement,
                has_unclosed_char_pairs,
            ));
        }
        '(' => TokenKind::OpenParenthese,
        ')' => TokenKind::CloseParenthese,
        '[' => TokenKind::OpenBracket,
        ']' => TokenKind::CloseBracket,
        '{' => TokenKind::OpenBrace,
        '+' => TokenKind::Plus,
        '-' => {
            return Some(consume_possible_tag_end_whitespace_adjustment(
                source,
                leading_whitespace,
                in_tag_kind,
                has_unclosed_char_pairs,
                WhitespacePreference::Remove,
            ));
        }
        '_' => {
            return Some(consume_possible_tag_end_whitespace_adjustment(
                source,
                leading_whitespace,
                in_tag_kind,
                has_unclosed_char_pairs,
                WhitespacePreference::Replace,
            ));
        }
        '*' => TokenKind::Asterisk,
        '/' => TokenKind::ForwardSlash,
        '~' => TokenKind::Tilde,
        ',' => TokenKind::Comma,
        ':' => if_matches!(':' => PathSeparator else Colon),
        ';' => TokenKind::Semicolon,
        '&' => if_matches!('&' => And else Ampersand),
        '!' => if_matches!('=' => NotEq else Exclamation),
        '=' => if_matches!('=' => Eq else Equal),
        '<' => if_matches!('=' => LessThanOrEqualTo else LessThan),
        '>' => if_matches!('=' => GreaterThanOrEqualTo else GreaterThan),
        '|' => if_matches!('|' => Or else VerticalBar),
        '.' => {
            if source.next_if(|char| char == '.') {
                if_matches!('=' => RangeInclusive else RangeExclusive)
            } else {
                TokenKind::Period
            }
        }
        '@' => TokenKind::At,
        'a'..='z' | 'A'..='Z' => return Some(consume_ident(source, leading_whitespace)),
        '0' => return Some(consume_alternative_base(source, leading_whitespace)),
        '1'..='9' => return Some(consume_decimal(source, leading_whitespace)),
        _ => {
            let source = source
                .consume()
                .expect("Buffer should contain at least one char");
            return Some((
                None,
                Err(UnexpectedTokenError::new(
                    "Unexpected character in expression",
                    source,
                )),
            ));
        }
    };

    let source = source
        .consume()
        .expect("Buffer should contain at least one character");

    Some((
        None,
        Ok((Token::new(kind, &source, leading_whitespace), None)),
    ))
}

pub(crate) fn consume_ident<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    let source = source
        .consume_while(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
        .expect("Buffer should contain at least one character");

    let kind = match source.as_str() {
        "true" => Ok(TokenKind::Bool(true)),
        "false" => Ok(TokenKind::Bool(false)),
        "oxiplate_formatter" => Err("`oxiplate_formatter` is a reserved name"),
        "oxiplate_loop_had_item" => Err("`oxiplate_loop_had_item` is a reserved name"),
        "crate" => Ok(TokenKind::Crate),
        "Self" => Ok(TokenKind::SelfCurrentType),
        "self" => Ok(TokenKind::SelfCurrentModule),
        "super" => Ok(TokenKind::Super),
        _ => Ok(TokenKind::Ident),
    };

    let token = match kind {
        Ok(kind) => Ok((Token::new(kind, &source, leading_whitespace), None)),
        Err(message) => Err(UnexpectedTokenError::new(message, source)),
    };

    (None, token)
}
