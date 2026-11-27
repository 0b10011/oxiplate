use super::{Statement, StatementKind};
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::item::Item;
use crate::template::parser::prelude::*;

#[derive(Debug)]
pub(in crate::template::parser) struct PlainText<'a> {
    text: Option<Source<'a>>,
    is_ended: bool,
}

impl<'a> PlainText<'a> {
    pub(in crate::template::parser) fn parser() -> PlainTextParser<'a> {
        PlainTextParser {
            phantom_data: PhantomData,
        }
    }

    pub(in crate::template::parser) fn end_parser() -> PlainTextEndParser<'a> {
        PlainTextEndParser {
            phantom_data: PhantomData,
        }
    }

    pub(in crate::template::parser) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            internal_error!(
                item.source().span_token().unwrap(),
                "Attempted to add item to ended `plaintext` statement",
            );
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::EndPlainText,
                ..
            }) => self.is_ended = true,
            item => {
                self.text = Some(
                    item.source()
                        .append_to_some(self.text.clone(), "Item expected after previous item"),
                );
            }
        }
    }

    pub(in crate::template::parser) fn is_ended(&self) -> bool {
        self.is_ended
    }
}

impl<'a> From<PlainText<'a>> for StatementKind<'a> {
    fn from(statement: PlainText<'a>) -> Self {
        StatementKind::PlainText(statement)
    }
}

impl<'a> ToTokensWithMutState<'a> for PlainText<'a> {
    fn to_tokens_with_mut_state<'b: 'a>(&'a self, _state: &mut State<'b>) -> BuiltTokens {
        if let Some(ref text) = self.text {
            let span = text.span_token();
            let text = text.as_str();

            (
                quote_spanned! {span=>
                    oxiplate_formatter.write_str(#text)?;
                },
                EstimatedLength::new(text.len()),
            )
        } else {
            (TokenStream::new(), EstimatedLength::new(0))
        }
    }
}

pub(in crate::template::parser) struct PlainTextParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for PlainTextParser<'a> {
    type TokenKind = TokenKind;
    type Output = Statement<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, include_keyword) = KeywordParser::new("plaintext").parse(tokens)?;

        let source = include_keyword.source().to_owned();

        Ok((
            tokens,
            Statement {
                kind: PlainText {
                    text: None,
                    is_ended: false,
                }
                .into(),
                source,
            },
        ))
    }
}

pub(in crate::template::parser) struct PlainTextEndParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for PlainTextEndParser<'a> {
    type TokenKind = TokenKind;
    type Output = Statement<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, output) = KeywordParser::new("endplaintext").parse(tokens)?;

        Ok((
            tokens,
            Statement {
                kind: StatementKind::EndPlainText,
                source: output.source().clone(),
            },
        ))
    }
}
