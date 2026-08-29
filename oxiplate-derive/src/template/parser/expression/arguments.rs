use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};

use super::Res;
use crate::parser::{Parser as _, cut, ignore_all_errors, many0, take};
use crate::template::parser::expression::{Expression, expression};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, State, quote_spanned};

pub(crate) type FirstArgument<'a> = Box<Expression<'a>>;
pub(crate) type RemainingArguments<'a> = Vec<(Source<'a>, Expression<'a>)>;

#[derive(Debug)]
pub(crate) struct ArgumentsGroup<'a> {
    open_paren: Source<'a>,
    pub(crate) arguments: Option<(
        FirstArgument<'a>,
        RemainingArguments<'a>,
        Option<Source<'a>>,
    )>,
    close_paren: Source<'a>,
}

impl<'a> ArgumentsGroup<'a> {
    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let mut tokens = TokenStream::new();

        if let Some((first_argument, remaining_arguments, _trailing_comma)) = &self.arguments {
            // First argument
            tokens.append_all(first_argument.to_tokens(state).0);

            // Remaining arguments
            for (comma, expression) in remaining_arguments {
                let comma_span = comma.span_token();
                tokens.append_all(quote_spanned! {comma_span=> , });
                tokens.append_all(expression.to_tokens(state).0);
            }
        }

        let mut group = proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens);
        group.set_span(self.source().span_token());
        group.to_token_stream()
    }

    /// Get the `Source` for the entire arguments group.
    pub fn source(&self) -> Source<'a> {
        let mut source = self.open_paren.clone();

        if let Some((first_argument, remaining_arguments, trailing_comma)) = &self.arguments {
            source = source.merge(&first_argument.source(), "Argument expected after previous");

            for (comma, expression) in remaining_arguments {
                source = source
                    .merge(comma, "Comma expected after expression")
                    .merge(&expression.source(), "Expression expected after comma");
            }

            source = source.merge_some(trailing_comma.as_ref(), "Comma expected after expression");
        }

        source = source.merge(
            &self.close_paren,
            "Closing parenthese expected after arguments",
        );

        source
    }
}

pub(crate) fn arguments(tokens: TokenSlice) -> Res<ArgumentsGroup> {
    let (tokens, (open_paren, (parsed_arguments, close_paren))) = (
        take(TokenKind::OpenParenthese),
        cut(
            "Expected comma-separated list of arguments followed by `)`",
            (
                ignore_all_errors((
                    expression(true),
                    many0((take(TokenKind::Comma), expression(true))),
                    ignore_all_errors(take(TokenKind::Comma)),
                )),
                take(TokenKind::CloseParenthese),
            ),
        ),
    )
        .parse(tokens)?;

    let arguments = if let Some((first_argument, parsed_remaining_arguments, trailing_comma)) =
        parsed_arguments
    {
        let mut remaining_arguments = Vec::new();
        for (comma, expression) in parsed_remaining_arguments {
            remaining_arguments.push((comma.source().clone(), expression));
        }

        Some((
            Box::new(first_argument),
            remaining_arguments,
            trailing_comma.map(|comma| comma.source().clone()),
        ))
    } else {
        None
    };

    Ok((
        tokens,
        ArgumentsGroup {
            open_paren: open_paren.source().clone(),
            arguments,
            close_paren: close_paren.source().clone(),
        },
    ))
}
