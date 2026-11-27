use std::mem;

use super::{Statement, StatementKind};
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::item::Item;
use crate::template::parser::prelude::*;
use crate::template::parser::r#static::Static;
use crate::template::parser::template::Template;

#[derive(Debug)]
pub(in crate::template::parser) struct PlainText<'a> {
    leading_whitespace: Option<Static<'a>>,
    text: Option<Source<'a>>,
    trailing_whitespace: Option<Static<'a>>,
    errors: Template<'a>,
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
            Item::CompileError { .. } => self.errors.0.push(item),
            Item::Whitespace(whitespace) => {
                if self.text.is_none() {
                    self.leading_whitespace = Some(whitespace);
                } else {
                    self.trailing_whitespace = Some(whitespace);
                }
            }
            _ => {
                if self.trailing_whitespace.is_some() {
                    let trailing_whitespace = mem::take(&mut self.trailing_whitespace);
                    self.text = Some(item.source().append_to_some(
                        trailing_whitespace.map(|whitespace| whitespace.1),
                        "Item expected after previous item",
                    ));
                }

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
    fn to_tokens_with_mut_state<'b: 'a>(&'a self, state: &mut State<'b>) -> BuiltTokens {
        let mut tokens = TokenStream::new();
        let mut estimated_length = EstimatedLength::new(0);

        // Append leading whitespace that may have been adjusted by the tag.
        if let Some(ref leading_whitespace) = self.leading_whitespace {
            let span = leading_whitespace.1.span_token();
            let leading_whitespace = leading_whitespace.0;
            if !leading_whitespace.is_empty() {
                tokens.append_all(quote_spanned! {span=>
                    oxiplate_formatter.write_str(#leading_whitespace)?;
                });
                estimated_length += EstimatedLength::new(leading_whitespace.len());
            }
        }

        // Append text.
        if let Some(ref text) = self.text {
            let span = text.span_token();
            let text = text.as_str();
            tokens.append_all(quote_spanned! {span=>
                oxiplate_formatter.write_str(#text)?;
            });
            estimated_length += EstimatedLength::new(text.len());
        }

        // Append leading whitespace that may have been adjusted by the tag.
        if let Some(ref trailing_whitespace) = self.trailing_whitespace {
            let span = trailing_whitespace.1.span_token();
            let trailing_whitespace = trailing_whitespace.0;
            if !trailing_whitespace.is_empty() {
                tokens.append_all(quote_spanned! {span=>
                    oxiplate_formatter.write_str(#trailing_whitespace)?;
                });
                estimated_length += EstimatedLength::new(trailing_whitespace.len());
            }
        }

        // Append any errors.
        let (errors, _) = self.errors.to_tokens_with_mut_state(state);
        tokens.append_all(quote! { #errors });

        (tokens, estimated_length)
    }
}

pub(in crate::template::parser) struct PlainTextParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for PlainTextParser<'a> {
    type TokenKind = TokenKind;
    type Output = Statement<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, plaintext_keyword) = KeywordParser::new("plaintext").parse(tokens)?;

        let source = plaintext_keyword.source().to_owned();

        Ok((
            tokens,
            Statement {
                kind: PlainText {
                    leading_whitespace: None,
                    text: None,
                    trailing_whitespace: None,
                    errors: Template(Vec::new()),
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
