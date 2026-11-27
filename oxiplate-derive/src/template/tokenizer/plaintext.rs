use super::expression::consume_ident;
use super::r#static::consume_possible_tag_start;
use super::{Token, whitespace};
use crate::template::tokenizer::Res;
use crate::template::tokenizer::kind::TokenKind;
use crate::tokenizer::BufferedSource;

pub(crate) fn consume_plaintext<'a>(source: &mut BufferedSource<'a>) -> Res<'a> {
    let source = source
        .consume_until(|char| matches!(char, '{' | whitespace!()))
        .expect("Buffer should contain at least one character");

    (
        None,
        Ok((Token::new(TokenKind::StaticText, &source, None), None)),
    )
}

pub(crate) fn consume_endplaintext_tag<'a>(source: &mut BufferedSource<'a>) -> Res<'a> {
    let mut result = None;
    let mut cloned_buffer = source.clone();
    if cloned_buffer.next_if(|char| char == '%') {
        let _ = cloned_buffer.next_if(|char| matches!(char, '-' | '_'));
        let _ = cloned_buffer.next_while(|char| matches!(char, whitespace!()));
        let _ = cloned_buffer.consume();
        if cloned_buffer.next_if(|char| matches!(char, 'e')) {
            if let (_, Ok(token)) = consume_ident(&mut cloned_buffer, None) {
                if token.source().as_str() == "endplaintext" {
                    result = Some(consume_possible_tag_start(source));
                }
            }
        }
    }

    result.unwrap_or_else(|| {
        let source = source.consume().expect("Should contain `{`");
        (
            None,
            Ok((Token::new(TokenKind::StaticText, &source, None), None)),
        )
    })
}
