use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::pair;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};

use super::Res;
use crate::syntax::expression::{ExpressionAccess, expression};
use crate::syntax::template::whitespace;
use crate::{Source, State, quote_spanned};

pub(crate) type FirstArgument<'a> = Box<ExpressionAccess<'a>>;
pub(crate) type RemainingArguments<'a> = Vec<(Source<'a>, ExpressionAccess<'a>)>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ArgumentsGroup<'a> {
    pub(crate) open_paren: Source<'a>,
    pub(crate) arguments: Option<(FirstArgument<'a>, RemainingArguments<'a>)>,
    pub(crate) close_paren: Source<'a>,
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
                let comma_span = comma.span();
                tokens.append_all(quote_spanned! {comma_span=> , });
                tokens.append_all(expression.to_tokens(state).0);
            }
        }

        proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens).to_token_stream()
    }

    /// Get the `Source` for the entire arguments group.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

pub(crate) fn arguments(input: Source) -> Res<Source, ArgumentsGroup> {
    let (input, (open_paren, (leading_whitespace, parsed_arguments, close_paren))) = pair(
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

    let mut source = open_paren.clone().merge_some(
        leading_whitespace.as_ref(),
        "Whitespace expected after open parenthese",
    );

    let arguments = if let Some((
        first_argument,
        whitespace,
        parsed_remaining_arguments,
        trailing_comma,
        trailing_whitespace,
    )) = parsed_arguments
    {
        source = source
            .merge(
                &first_argument.source(),
                "First argument expected after whitespace",
            )
            .merge_some(
                whitespace.as_ref(),
                "Whitespace expected after first argument",
            );

        let mut remaining_arguments = Vec::new();
        for ((comma, whitespace_after_comma, expression), whitespace_after_expression) in
            parsed_remaining_arguments
        {
            source = source
                .merge(&comma, "Comma expected after whitespace")
                .merge_some(
                    whitespace_after_comma.as_ref(),
                    "Whitespace expected after comma",
                )
                .merge(&expression.source(), "Expression expected after whitespace")
                .merge_some(
                    whitespace_after_expression.as_ref(),
                    "Whitespace expected after expression",
                );

            remaining_arguments.push((comma, expression));
        }

        source = source
            .merge_some(trailing_comma.as_ref(), "Comma expected after whitespace")
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after comma",
            );

        Some((Box::new(first_argument), remaining_arguments))
    } else {
        None
    };

    source = source.merge(&close_paren, "Closing parenthese expected after whitespace");

    Ok((
        input,
        ArgumentsGroup {
            open_paren,
            arguments,
            close_paren,
            source,
        },
    ))
}
