use super::arguments::{ArgumentsGroup, arguments};
use super::{Expression, Identifier, NestedExpression};
use crate::template::parser::prelude::*;

/// `expr | filter(args)`
#[derive(Debug)]
pub(crate) struct Filter<'a> {
    name: Identifier<'a>,
    pub(super) expression: Box<Expression<'a>>,
    vertical_bar: Source<'a>,
    cow_prefix: Option<Source<'a>>,
    arguments: Option<ArgumentsGroup<'a>>,
}

impl<'a> Filter<'a> {
    /// Parses filters (`expr | filter()`).
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, (vertical_bar, cow_prefix, name, arguments)) = (
            take(TokenKind::VerticalBar),
            ignore_recoverable_errors(take(TokenKind::GreaterThan)),
            cut("Expected a filter name", Identifier::parse),
            ignore_recoverable_errors(arguments),
        )
            .parse(tokens)?;

        let callback = Box::new(move |expression: Expression<'a>| {
            Expression::Filter(Filter {
                name,
                expression: Box::new(expression),
                vertical_bar: vertical_bar.source().clone(),
                cow_prefix: cow_prefix.map(|token| token.source().clone()),
                arguments,
            })
        });

        Ok((tokens, callback))
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> Source<'a> {
        self.expression
            .source()
            .merge(
                &self.vertical_bar,
                "Vertical bar should follow leading whitespace",
            )
            .merge_some(
                self.cow_prefix.as_ref(),
                "Cow prefix should follow whitespace",
            )
            .merge(self.name.source(), "Filter name should follow whitespace")
            .merge_some(
                self.arguments.as_ref().map(ArgumentsGroup::source).as_ref(),
                "Arguments should follow trailing whitespace",
            )
    }
}

impl<'a> ToTokensWithState<'a> for Filter<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let (expression, estimated_length) = self.expression.to_tokens(state);
        let mut argument_tokens = expression;

        let arguments = if let Some(arguments) = &self.arguments {
            if let Some((first_argument, remaining_arguments, _trailing_comma)) =
                &arguments.arguments
            {
                // First argument
                let comma_span = self.vertical_bar.span_token();
                argument_tokens.append_all(quote_spanned! {comma_span=> , });
                argument_tokens.append_all(first_argument.to_tokens(state).0);

                // Remaining arguments
                for (comma, expression) in remaining_arguments {
                    let comma_span = comma.span_token();
                    argument_tokens.append_all(quote_spanned! {comma_span=> , });
                    argument_tokens.append_all(expression.to_tokens(state).0);
                }
            }

            let mut group =
                proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, argument_tokens);
            group.set_span(self.source().span_token());
            group.to_token_stream()
        } else {
            let mut group =
                proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, argument_tokens);
            group.set_span(self.name.source().span_token());
            group.to_token_stream()
        };

        let name = &self.name;
        let span = self.name.source().span_token();
        if let Some(cow_prefix) = &self.cow_prefix {
            let span = cow_prefix.span_token();

            if cfg!(feature = "_oxiplate") {
                (
                    quote_spanned! {span=>
                        ::oxiplate::CowStrWrapper::new(
                            (
                                &&::oxiplate::ToCowStrWrapper::new(
                                    &(filters_for_oxiplate::#name #arguments)
                                )
                            ).to_cow_str()
                        )
                    },
                    estimated_length,
                )
            } else {
                (
                    quote_spanned! {span=>
                        compile_error!("Cow prefix requires the `oxiplate` library due to trait usage")
                    },
                    EstimatedLength::new(0),
                )
            }
        } else {
            (
                quote_spanned! {span=>
                    filters_for_oxiplate::#name #arguments
                },
                estimated_length,
            )
        }
    }
}

impl<'a> From<Filter<'a>> for Expression<'a> {
    fn from(value: Filter<'a>) -> Self {
        Self::Filter(value)
    }
}
