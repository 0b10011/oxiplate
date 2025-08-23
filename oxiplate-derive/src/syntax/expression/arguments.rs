use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::pair;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens, TokenStreamExt};

use super::Res;
use crate::syntax::expression::{expression, ExpressionAccess};
use crate::syntax::template::whitespace;
use crate::{Source, State};

type FirstArgument<'a> = Box<ExpressionAccess<'a>>;
type RemainingArguments<'a> = Vec<(Source<'a>, ExpressionAccess<'a>)>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ArgumentsGroup<'a> {
    open_paren: Source<'a>,
    arguments: Option<(FirstArgument<'a>, RemainingArguments<'a>)>,
    close_paren: Source<'a>,
}

impl ArgumentsGroup<'_> {
    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let mut tokens = TokenStream::new();

        if let Some((first_argument, remaining_arguments)) = &self.arguments {
            // First argument
            tokens.append_all(first_argument.to_tokens(state).0);

            // Remaining arguments
            for (comma, expression) in remaining_arguments {
                let comma_span = comma.span();
                tokens.append_all(quote_spanned! {comma_span=> , });
                tokens.append_all(expression.to_tokens(state).0);
            }
        }

        proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens).to_token_stream()
    }
}

pub(crate) fn arguments(input: Source) -> Res<Source, ArgumentsGroup> {
    let (input, (open_paren, (_leading_whitespace, parsed_arguments, close_paren))) = pair(
        tag("("),
        context(
            "Expected comma-separated list of arguments followed by `)`",
            cut((
                opt(whitespace),
                opt((
                    expression(true, true),
                    opt(whitespace),
                    many0(pair(
                        (tag(","), opt(whitespace), expression(true, true)),
                        opt(whitespace),
                    )),
                    opt(tag(",")),
                    opt(whitespace),
                )),
                context("Expected closing `)`", cut(tag(")"))),
            )),
        ),
    )
    .parse(input)?;

    let arguments = if let Some((
        first_argument,
        _whitespace,
        parsed_remaining_arguments,
        _trailing_comma,
        _trailing_whitespace,
    )) = parsed_arguments
    {
        let mut remaining_arguments = Vec::new();
        for ((comma, _whitespace, expression), _trailing_whitespace) in parsed_remaining_arguments {
            remaining_arguments.push((comma, expression));
        }
        Some((Box::new(first_argument), remaining_arguments))
    } else {
        None
    };

    Ok((
        input,
        ArgumentsGroup {
            open_paren,
            arguments,
            close_paren,
        },
    ))
}
