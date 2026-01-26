mod buffered_source;
mod parser;
mod slice;
mod token;

use std::fmt::Debug;

pub use self::slice::TokenSlice;
pub use self::token::{TagKind, Token, TokenKind, WhitespacePreference};
use crate::Source;
use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::parser::comment::consume_comment;
use crate::tokenizer::parser::expression::consume_expression_token;
use crate::tokenizer::parser::r#static::{
    consume_possible_tag_start, consume_static_text, consume_static_whitespace,
};
use crate::tokenizer::parser::{
    consume_possible_tag_end, consume_possible_tag_end_whitespace_adjustment, whitespace,
};
use crate::tokenizer::token::ParseError;

#[derive(Debug)]
pub struct Eof<'a> {
    source: Source<'a>,
}

impl<'a> Eof<'a> {
    #[cfg(test)]
    pub fn for_test(source: Source<'a>) -> Self {
        Self { source }
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

#[derive(Debug)]
pub struct Tokens<'a> {
    source: BufferedSource<'a>,
    context: Context,
    char_pair_stack: Vec<CharPairKind>,
}

impl<'a> Tokens<'a> {
    pub fn new(template: Source<'a>) -> Self {
        Self {
            source: template.into(),
            context: Context::Static,
            char_pair_stack: vec![],
        }
    }

    pub fn tokens_and_eof(mut self) -> (Vec<Token<'a>>, Eof<'a>) {
        let eof = self.source.eof();

        (self.collect(), eof)
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (new_context, token): (Option<Context>, Token) = match self.context {
            Context::Static => match self.source.next()? {
                '{' => consume_possible_tag_start(&mut self.source),
                whitespace!() => consume_static_whitespace(&mut self.source),
                _ => consume_static_text(&mut self.source),
            },
            Context::Comment => {
                let (new_context, token) = match self.source.next() {
                    Some('-') => consume_possible_tag_end_whitespace_adjustment(
                        &mut self.source,
                        None,
                        &TagKind::Comment,
                        !self.char_pair_stack.is_empty(),
                        WhitespacePreference::Remove,
                    ),
                    Some('_') => consume_possible_tag_end_whitespace_adjustment(
                        &mut self.source,
                        None,
                        &TagKind::Comment,
                        !self.char_pair_stack.is_empty(),
                        WhitespacePreference::Replace,
                    ),
                    Some('#') => consume_possible_tag_end(
                        &mut self.source,
                        None,
                        TagKind::Comment,
                        &TagKind::Comment,
                        !self.char_pair_stack.is_empty(),
                    ),
                    Some(_char) => consume_comment(&mut self.source),
                    None => (
                        Some(Context::Static),
                        Token::new(
                            TokenKind::Unexpected(ParseError::boxed(
                                "End of file encountered while parsing a comment. Expected `#}`, \
                                 `-#}`, or `_#}`",
                            )),
                            self.source.eof().source(),
                            None,
                        ),
                    ),
                };
                (new_context, token)
            }
            Context::Statement => consume_expression_token(
                &mut self.source,
                !self.char_pair_stack.is_empty(),
                &TagKind::Statement,
            ),
            Context::Writ => consume_expression_token(
                &mut self.source,
                !self.char_pair_stack.is_empty(),
                &TagKind::Writ,
            ),
        };

        if let Some(new_context) = new_context {
            self.context = new_context;
        }

        // Ensure all char pairs are matched.
        let char_pair_check = match token.kind() {
            TokenKind::OpenBrace => {
                self.char_pair_stack.push(CharPairKind::Brace);
                None
            }
            TokenKind::OpenBracket => {
                self.char_pair_stack.push(CharPairKind::Bracket);
                None
            }
            TokenKind::OpenParenthese => {
                self.char_pair_stack.push(CharPairKind::Parenthese);
                None
            }
            TokenKind::CloseBrace => Some((
                matches!(self.char_pair_stack.last(), Some(CharPairKind::Brace)),
                "Expected `}`",
            )),
            TokenKind::CloseBracket => Some((
                matches!(self.char_pair_stack.last(), Some(CharPairKind::Bracket)),
                "Expected `]`",
            )),
            TokenKind::CloseParenthese => Some((
                matches!(self.char_pair_stack.last(), Some(CharPairKind::Parenthese)),
                "Expected `)`",
            )),
            _ => None,
        };

        if let Some((char_pair_matched, error_message)) = char_pair_check {
            if char_pair_matched {
                self.char_pair_stack.pop();
            } else {
                return Some(
                    token.with_kind(TokenKind::Unexpected(ParseError::boxed(error_message))),
                );
            }
        }

        Some(token)
    }
}

#[derive(Debug)]
enum Context {
    Comment,
    Statement,
    Static,
    Writ,
}

#[derive(Debug)]
enum CharPairKind {
    /// `{` and `}`
    Brace,

    /// `[` and `]`
    Bracket,

    /// `(` and `)`
    Parenthese,
}

#[test]
fn test() {
    use proc_macro2::Span;
    use syn::LitStr;

    use crate::source::SourceOwned;

    let span = Span::mixed_site();
    let string = "a {# whoa #} \n\thello \t\n{{ name }} b";
    assert_eq!(
        Tokens::new(Source::new(&SourceOwned::new(
            &LitStr::new(string, span),
            span,
            None
        )))
        .into_iter()
        .map(|token| format!("{:?}", token))
        .collect::<Vec<String>>(),
        vec![
            "StaticText[a]",
            "StaticWhitespace[ ]",
            "TagStart { kind: Comment, whitespace_preference: Indifferent }[{#]",
            "Comment[ whoa ]",
            "TagEnd { kind: Comment, whitespace_preference: Indifferent }[#}]",
            "StaticWhitespace[ \n\t]",
            "StaticText[hello]",
            "StaticWhitespace[ \t\n]",
            "TagStart { kind: Writ, whitespace_preference: Indifferent }[{{]",
            "Ident[name]",
            "TagEnd { kind: Writ, whitespace_preference: Indifferent }[}}]",
            "StaticWhitespace[ ]",
            "StaticText[b]",
        ],
    );
}
