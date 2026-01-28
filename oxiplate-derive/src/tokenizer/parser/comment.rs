use super::Token;
use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::parser::Res;
use crate::tokenizer::parser::kind::TokenKind;

pub fn consume_comment<'a>(source: &mut BufferedSource<'a>) -> Res<'a> {
    let source = source
        .consume_until(|char| matches!(char, '#' | '-' | '_'))
        .expect("Buffer should contain at least one character");

    (None, Ok(Token::new(TokenKind::Comment, &source, None)))
}
