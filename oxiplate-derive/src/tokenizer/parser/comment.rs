use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::{Context, Token, TokenKind};

pub fn consume_comment<'a>(source: &mut BufferedSource<'a>) -> (Option<Context>, Token<'a>) {
    let source = source
        .consume_until(|char| matches!(char, '#' | '-' | '_'))
        .expect("Buffer should contain at least one character");

    (None, Token::new(TokenKind::Comment, &source, None))
}
