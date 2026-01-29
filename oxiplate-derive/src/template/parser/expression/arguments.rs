use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};

use super::Res;
use crate::parser::{Parser as _, cut, many0, opt, take};
use crate::template::parser::expression::{ExpressionAccess, expression};
use crate::template::tokenizer::{Token, TokenKind, TokenSlice};
use crate::{Source, State, quote_spanned};

pub(crate) type FirstArgument<'a> = Box<ExpressionAccess<'a>>;
pub(crate) type RemainingArguments<'a> = Vec<(Source<'a>, ExpressionAccess<'a>)>;

#[derive(Debug)]
pub(crate) struct ArgumentsGroup<'a> {
    pub(crate) arguments: Option<(FirstArgument<'a>, RemainingArguments<'a>)>,
    source: Source<'a>,
}

impl<'a> ArgumentsGroup<'a> {
    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let mut tokens = TokenStream::new();

        if let Some((first_argument, remaining_arguments)) = &self.arguments {
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
        group.set_span(self.source.span_token());
        group.to_token_stream()
    }

    /// Get the `Source` for the entire arguments group.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

pub(crate) fn arguments(tokens: TokenSlice) -> Res<ArgumentsGroup> {
    let (tokens, (open_paren, (parsed_arguments, close_paren))) = (
        take(TokenKind::OpenParenthese),
        cut(
            "Expected comma-separated list of arguments followed by `)`",
            (
                opt((
                    expression(true, true),
                    many0((take(TokenKind::Comma), expression(true, true))),
                    opt(take(TokenKind::Comma)),
                )),
                take(TokenKind::CloseParenthese),
            ),
        ),
    )
        .parse(tokens)?;

    let mut source = open_paren.source().clone();

    let arguments = if let Some((first_argument, parsed_remaining_arguments, trailing_comma)) =
        parsed_arguments
    {
        source = source.merge(&first_argument.source(), "Argument expected after previous");

        let mut remaining_arguments = Vec::new();
        for (comma, expression) in parsed_remaining_arguments {
            source = source
                .merge(comma.source(), "Comma expected after expression")
                .merge(&expression.source(), "Expression expected after comma");

            remaining_arguments.push((comma.source().clone(), expression));
        }

        source = source.merge_some(
            trailing_comma.map(Token::source),
            "Comma expected after expression",
        );

        Some((Box::new(first_argument), remaining_arguments))
    } else {
        None
    };

    source = source.merge(
        close_paren.source(),
        "Closing parenthese expected after arguments",
    );

    Ok((tokens, ArgumentsGroup { arguments, source }))
}
