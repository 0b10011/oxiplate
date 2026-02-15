mod comment;
mod expression;
mod kind;
mod r#static;

use self::expression::consume_ident;
pub use self::kind::{TagKind, TokenKind, WhitespacePreference};
use crate::Source;
use crate::template::tokenizer::comment::consume_comment;
use crate::template::tokenizer::expression::consume_expression_token;
use crate::template::tokenizer::r#static::{
    consume_possible_tag_start, consume_static_text, consume_static_whitespace,
};
pub use crate::tokenizer::Eof;
use crate::tokenizer::{BufferedSource, UnexpectedTokenError};

pub type Token<'a> = crate::tokenizer::Token<'a, TokenKind>;
pub type TokenSlice<'a> = crate::tokenizer::TokenSlice<'a, TokenKind>;

type Res<'a> = (Option<Context>, Result<Token<'a>, UnexpectedTokenError<'a>>);

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

pub fn tokens_and_eof(template: Source) -> (Vec<Result<Token, UnexpectedTokenError>>, Eof) {
    let tokens = Tokens::new(template);
    let eof = tokens.source.eof();

    (tokens.collect(), eof)
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
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, UnexpectedTokenError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (new_context, token): (Option<Context>, Self::Item) = match self.context {
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
                        Err(UnexpectedTokenError::new(
                            "End of file encountered while parsing a comment. Expected `#}`, \
                             `-#}`, or `_#}`",
                            self.source.eof().source().clone(),
                        )),
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

        let token = match token {
            Ok(token) => token,
            err => return Some(err),
        };

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
                return Some(Err(UnexpectedTokenError::new(
                    error_message,
                    token.source().clone(),
                )));
            }
        }

        Some(Ok(token))
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

fn consume_possible_tag_end<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
    tag_end_kind: TagKind,
    in_tag_kind: &TagKind,
    has_unclosed_char_pairs: bool,
) -> Res<'a> {
    if has_unclosed_char_pairs || source.peek() != Some('}') {
        let source = source
            .consume()
            .expect("Buffer should contain `}`, `%`, or `#`");

        let kind = match tag_end_kind {
            TagKind::Writ => TokenKind::CloseBrace,
            TagKind::Statement => TokenKind::Percent,
            TagKind::Comment => TokenKind::Comment,
        };

        return (None, Ok(Token::new(kind, &source, leading_whitespace)));
    }

    let _ = source.next();
    let source = source
        .consume()
        .expect("Buffer should contain `}}`, `%}`, or `#}`");

    if tag_end_kind == *in_tag_kind {
        // Ending current tag
        (
            Some(Context::Static),
            Ok(Token::new(
                TokenKind::TagEnd {
                    kind: tag_end_kind,
                    whitespace_preference: WhitespacePreference::Indifferent,
                },
                &source,
                leading_whitespace,
            )),
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
            Err(UnexpectedTokenError::new(
                message,
                source.append_to_leading_whitespace(
                    leading_whitespace,
                    "Tag end should follow whitespace",
                ),
            )),
        )
    }
}

fn consume_possible_tag_end_whitespace_adjustment<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
    in_tag_kind: &TagKind,
    has_unclosed_char_pairs: bool,
    whitespace_preference: WhitespacePreference,
) -> Res<'a> {
    let tag_end_kind = match source.peek_2() {
        Some(['}', '}']) => Some(TagKind::Writ),
        Some(['%', '}']) => Some(TagKind::Statement),
        Some(['#', '}']) => Some(TagKind::Comment),
        _ => None,
    };

    // Handle `-`, `_`, and non-comment close tags in comments.
    if let TagKind::Comment = in_tag_kind {
        if let None | Some(TagKind::Statement | TagKind::Writ) = tag_end_kind {
            return (
                None,
                Ok(Token::new(
                    TokenKind::Comment,
                    &source.consume().expect("Buffer should contain `-` or `_`"),
                    leading_whitespace,
                )),
            );
        }
    }

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

        return (None, Ok(Token::new(kind, &source, leading_whitespace)));
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
            Ok(Token::new(
                TokenKind::TagEnd {
                    kind: tag_end_kind,
                    whitespace_preference,
                },
                &source,
                leading_whitespace,
            )),
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
            Err(UnexpectedTokenError::new(
                message,
                source.append_to_leading_whitespace(
                    leading_whitespace,
                    "Tag end with whitespace adjustment should follow whitespace",
                ),
            )),
        )
    }
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
        .map(|token| match token {
            Ok(token) => format!("{:?}", token),
            Err(err) => format!("{:?}", err),
        })
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
