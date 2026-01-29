use super::Token;
use crate::template::tokenizer::Res;
use crate::template::tokenizer::kind::TokenKind;
use crate::tokenizer::BufferedSource;

pub fn consume_comment<'a>(source: &mut BufferedSource<'a>) -> Res<'a> {
    let source = source
        .consume_until(|char| matches!(char, '#' | '-' | '_'))
        .expect("Buffer should contain at least one character");

    (None, Ok(Token::new(TokenKind::Comment, &source, None)))
}
