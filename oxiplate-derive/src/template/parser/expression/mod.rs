use std::mem;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};

mod arguments;
mod concat;
mod field_or_method;
mod group;
mod ident;
mod keyword;
mod literal;
mod operator;
mod prefix_operator;
mod tuple;

use self::arguments::arguments;
use self::concat::Concat;
use self::ident::IdentifierOrFunction;
pub(super) use self::ident::{Identifier, identifier};
pub(super) use self::keyword::{Keyword, KeywordParser};
pub(super) use self::literal::{Bool, Char, Float, Integer, Number, String};
use super::Res;
use super::expression::arguments::ArgumentsGroup;
use super::expression::operator::{Operator, parse_operator};
use super::expression::prefix_operator::{PrefixOperator, parse_prefixed_expression};
use crate::parser::{Parser as _, alt, cut, ignore_recoverable_errors, into, many0, take};
use crate::template::parser::expression::field_or_method::FieldOrMethod;
use crate::template::parser::expression::group::Group;
use crate::template::parser::expression::tuple::Tuple;
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

type NestedExpression<'a> = dyn FnOnce(Expression<'a>) -> Expression<'a> + 'a;

#[derive(Debug, Default)]
pub(crate) enum Expression<'a> {
    /// Placeholder expression used during precedence fixing.
    /// Will generate a compile error if not replaced.
    #[default]
    Placeholder,

    IdentifierOrFunction(IdentifierOrFunction<'a>),
    Char(Char<'a>),
    String(String<'a>),
    Integer(Integer<'a>),
    Float(Float<'a>),
    Bool(Bool<'a>),
    Group(Group<'a>),
    Tuple(Tuple<'a>),
    Concat(Concat<'a>),
    Calc {
        left: Box<Expression<'a>>,
        operator: Operator<'a>,
        right: Box<Option<Expression<'a>>>,
    },
    Prefixed(PrefixOperator<'a>, Box<Expression<'a>>),
    Cow {
        prefix: Source<'a>,
        expression: Box<Expression<'a>>,
    },

    /// `..` that represents a range
    /// where the start/end matches whatever it is applied to.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
    FullRange {
        source: Source<'a>,
    },

    /// `expr[expr]`
    /// See:
    /// - <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
    /// - <https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices>
    Index(
        Box<Expression<'a>>,
        Source<'a>,
        Box<Expression<'a>>,
        Source<'a>,
    ),

    /// `expr | filter(args)`
    Filter {
        name: Identifier<'a>,
        expression: Box<Expression<'a>>,
        vertical_bar: Source<'a>,
        cow_prefix: Option<Source<'a>>,
        arguments: Option<ArgumentsGroup<'a>>,
    },

    /// `expr.field` or `expr.method(args)`
    FieldOrMethod(FieldOrMethod<'a>),
}

impl<'a> Expression<'a> {
    /// Fix expression precedence.
    pub(self) fn fix_precedence(&mut self) {
        match self {
            Self::Placeholder => todo!("Placeholder expression not yet handled a"),

            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Tuple(_)
            | Self::IdentifierOrFunction(_)
            | Self::Prefixed(_, _)
            | Self::Cow { .. } => (),

            Self::Index(left, _, _, _) | Self::Calc { left, .. } => left.fix_precedence(),
            Self::Filter { expression, .. } => expression.fix_precedence(),
            Self::FieldOrMethod(field_or_method) => {
                field_or_method.expression.as_mut().fix_precedence();
            }
            Self::Concat(concat) => concat.first_expression.fix_precedence(),
        }

        if self.needs_precedence_fixed() {
            let Some(mut left) = self.take_left() else {
                return;
            };

            let Some(mut lefts_right) = left.take_right() else {
                self.give_left(&mut left);
                return;
            };

            lefts_right.fix_precedence();
            self.give_left(&mut lefts_right);
            left.give_right(self);
            mem::swap(&mut left, self);
        }
    }

    fn needs_precedence_fixed(&self) -> bool {
        let left = match self {
            Self::Placeholder => todo!("Placeholder expression not yet handled b"),

            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Tuple(_)
            | Self::IdentifierOrFunction(_)
            | Self::Prefixed(_, _)
            | Self::Cow { .. } => return false,

            Self::Index(left, _, _, _) | Self::Calc { left, .. } => left,
            Self::Filter { expression, .. } => expression,
            Self::FieldOrMethod(field_or_method) => field_or_method.expression.as_ref(),
            Self::Concat(concat) => concat.first_expression.as_ref(),
        };

        left.precedence() < self.precedence()
    }

    fn take_left(&mut self) -> Option<Expression<'a>> {
        match self {
            Self::Placeholder => todo!("Placeholder expression not yet handled c"),

            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Tuple(_)
            | Self::IdentifierOrFunction(_)
            | Self::Prefixed(_, _)
            | Self::Cow { .. } => None,

            Self::Index(left, _, _, _) | Self::Calc { left, .. } => Some(mem::take(left)),
            Self::Filter { expression, .. } => Some(mem::take(expression)),
            Self::FieldOrMethod(field_or_method) => {
                Some(mem::take(&mut field_or_method.expression))
            }
            Self::Concat(concat) => Some(mem::take(concat.first_expression.as_mut())),
        }
    }

    fn give_left(&mut self, new_left: &mut Expression<'a>) {
        let placeholder_left = match self {
            Self::Placeholder => todo!("Placeholder expression not yet handled d"),

            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Tuple(_)
            | Self::IdentifierOrFunction(_)
            | Self::Prefixed(_, _)
            | Self::Cow { .. } => {
                unreachable!("Only placeholder expressions should ever be overwritten")
            }

            Self::Index(left, _, _, _) | Self::Calc { left, .. } => left.as_mut(),
            Self::Filter { expression, .. } => expression.as_mut(),
            Self::FieldOrMethod(field_or_method) => field_or_method.expression.as_mut(),
            Self::Concat(concat) => concat.first_expression.as_mut(),
        };

        if !matches!(placeholder_left, Self::Placeholder) {
            unreachable!("Only placeholder expressions should ever be overwritten");
        }

        mem::swap(placeholder_left, new_left);
    }

    fn take_right(&mut self) -> Option<Expression<'a>> {
        match self {
            Self::Placeholder => todo!("Placeholder expression not yet handled e"),

            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Tuple(_)
            | Self::Index(_, _, _, _)
            | Self::Filter { .. }
            | Self::IdentifierOrFunction(_)
            | Self::FieldOrMethod(_) => None,

            Self::Concat(concat) => match concat.additional_expressions.last_mut() {
                Some((_tilde, expression)) => Some(mem::take(expression)),
                None => unreachable!("Concats should always contain at least 2 expressions"),
            },
            Self::Calc { right, .. } => match right.as_mut() {
                Some(right) => Some(mem::take(right)),
                None => None,
            },
            Self::Prefixed(_, right) => Some(mem::take(right)),
            Self::Cow { expression, .. } => Some(mem::take(expression)),
        }
    }

    fn give_right(&mut self, new_right: &mut Expression<'a>) {
        let placeholder_right = match self {
            Self::Placeholder => todo!("Placeholder expression not yet handled f"),

            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Tuple(_)
            | Self::Index(_, _, _, _)
            | Self::Filter { .. }
            | Self::IdentifierOrFunction(_)
            | Self::FieldOrMethod(_) => {
                unreachable!("Only placeholder expressions should ever be overwritten")
            }

            Self::Concat(concat) => match concat.additional_expressions.last_mut() {
                Some((_tilde, last)) => last,
                None => unreachable!("Concats should have at least 2 expressions"),
            },
            Self::Calc { right, .. } => match right.as_mut() {
                Some(right) => right,
                None => unreachable!("Only placeholder expressions should ever be overwritten"),
            },
            Self::Prefixed(_, right) => right,
            Self::Cow { expression, .. } => expression,
        };

        if !matches!(placeholder_right, Self::Placeholder) {
            unreachable!("Only placeholder expressions should ever be overwritten");
        }

        mem::swap(placeholder_right, new_right);
    }

    /// Expression precedence.
    fn precedence(&self) -> u8 {
        match self {
            Self::Placeholder => todo!("Placeholder expression not yet handled g"),

            // Rust expressions are assumed to be the same precedence
            // to let Rust handle the details.
            Self::FieldOrMethod(_)
            | Self::IdentifierOrFunction(_)
            | Self::Index(_, _, _, _)
            | Self::Calc { .. }
            | Self::Prefixed(_, _) => u8::MAX,

            // Oxiplate expressions
            Self::Concat(_) => 3,
            Self::Cow { .. } => 2,

            // Filters should always be handled last.
            Self::Filter { .. } => 1,

            // These expressions only contain a single operand
            // so the actual precedence value doesn't really matter.
            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::Group(_)
            | Self::Tuple(_)
            | Self::FullRange { .. } => 0,
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> BuiltTokens {
        match self {
            Expression::Placeholder => (
                quote! { compile_error!("Placeholder expression was never replaced.") },
                0,
            ),
            Expression::IdentifierOrFunction(identifier) => match &identifier {
                IdentifierOrFunction::Identifier(identifier) => {
                    let span = identifier.source().span_token();
                    if state.local_variables.contains(identifier.as_str()) {
                        (quote! { #identifier }, 1)
                    } else {
                        (quote_spanned! {span=> self.#identifier }, 1)
                    }
                }
                IdentifierOrFunction::Function(identifier, arguments) => {
                    let arguments = arguments.to_tokens(state);

                    let span = identifier.source().span_token();
                    if state.local_variables.contains(identifier.as_str()) {
                        (quote! { #identifier #arguments }, 1)
                    } else {
                        (quote_spanned! {span=> (self.#identifier)#arguments }, 1)
                    }
                }
            },
            Expression::Group(group) => group.to_tokens(state),
            Expression::Tuple(tuple) => tuple.to_tokens(state),
            Expression::Concat(concat) => concat.to_tokens(state),
            Expression::Calc {
                left,
                operator,
                right,
                ..
            } => {
                let (left, left_length) = left.to_tokens(state);
                let (right, right_length) = if let Some(right) = right.as_ref() {
                    right.to_tokens(state)
                } else {
                    (TokenStream::new(), left_length)
                };
                (
                    quote! { #left #operator #right },
                    left_length.min(right_length),
                )
            }
            Expression::Prefixed(operator, expression) => {
                let (expression, expression_length) = expression.to_tokens(state);
                (quote! { #operator #expression }, expression_length)
            }
            Expression::Cow {
                prefix, expression, ..
            } => {
                #[cfg_attr(not(feature = "_oxiplate"), allow(unused_variables))]
                let (expression, expression_length) = expression.to_tokens(state);
                let span = prefix.span_token();

                #[cfg(feature = "_oxiplate")]
                let expression = quote_spanned! {span=>
                    ::oxiplate::CowStrWrapper::new((&&::oxiplate::ToCowStrWrapper::new(&(#expression))).to_cow_str())
                };

                #[cfg(not(feature = "_oxiplate"))]
                let expression = quote_spanned! {span=>
                    compile_error!("Cow prefix requires the `oxiplate` library due to trait usage")
                };

                (expression, expression_length)
            }
            Expression::Char(char) => char.to_tokens(),
            Expression::String(string) => string.to_tokens(),
            Expression::Integer(number) => number.to_tokens(),
            Expression::Float(number) => number.to_tokens(),
            Expression::Bool(bool) => bool.to_tokens(),
            Expression::FullRange { source, .. } => {
                let span = source.span_token();
                (quote_spanned! {span=> .. }, 0)
            }
            Expression::Index(expression, open_bracket, range, _close_bracket) => {
                let span = open_bracket.span_token();
                let (expression, estimated_length) = expression.to_tokens(state);
                let (range, _range_length) = range.to_tokens(state);
                (
                    quote_spanned! {span=> #expression [ #range ] },
                    estimated_length,
                )
            }
            Expression::Filter {
                name,
                expression,
                vertical_bar,
                cow_prefix,
                arguments,
            } => Self::filter(
                state,
                name,
                expression,
                vertical_bar,
                cow_prefix.as_ref(),
                arguments.as_ref(),
                &self.source(),
            ),
            Expression::FieldOrMethod(field_or_method) => field_or_method.to_tokens(state),
        }
    }

    /// Generate tokens for a filter expression.
    fn filter(
        state: &State,
        name: &Identifier,
        expression: &Expression,
        vertical_bar: &Source,
        cow_prefix: Option<&Source>,
        arguments: Option<&ArgumentsGroup>,
        source: &Source,
    ) -> BuiltTokens {
        let (expression, estimated_length) = expression.to_tokens(state);
        let mut argument_tokens = expression;

        let arguments = if let Some(arguments) = arguments {
            if let Some((first_argument, remaining_arguments, _trailing_comma)) =
                &arguments.arguments
            {
                // First argument
                let comma_span = vertical_bar.span_token();
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
            group.set_span(source.span_token());
            group.to_token_stream()
        } else {
            let mut group =
                proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, argument_tokens);
            group.set_span(name.source().span_token());
            group.to_token_stream()
        };

        let span = name.source().span_token();
        if let Some(cow_prefix) = cow_prefix {
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
                    0,
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

    /// Get the `Source` for the expression.
    pub(crate) fn source(&self) -> Source<'a> {
        match self {
            Expression::Placeholder => todo!("Placeholder not yet handled h"),
            Expression::IdentifierOrFunction(identifier_or_function) => {
                identifier_or_function.source()
            }
            Expression::Char(value) => value.source().clone(),
            Expression::String(value) => value.source().clone(),
            Expression::Integer(value) => value.source().clone(),
            Expression::Float(value) => value.source().clone(),
            Expression::Bool(value) => value.source().clone(),
            Expression::Calc {
                left,
                operator,
                right,
            } => {
                if let Some(right) = right.as_ref() {
                    left.source()
                        .merge(operator.source(), "Operator should follow whitespace")
                        .merge(&right.source(), "Right expression should follow whitespace")
                } else {
                    left.source()
                        .merge(operator.source(), "Operator should follow left expression")
                }
            }
            Expression::FullRange { source, .. } => source.clone(),
            Expression::Filter {
                name,
                expression,
                vertical_bar,
                cow_prefix,
                arguments,
            } => expression
                .source()
                .merge(
                    vertical_bar,
                    "Vertical bar should follow leading whitespace",
                )
                .merge_some(cow_prefix.as_ref(), "Cow prefix should follow whitespace")
                .merge(name.source(), "Filter name should follow whitespace")
                .merge_some(
                    arguments.as_ref().map(ArgumentsGroup::source).as_ref(),
                    "Arguments should follow trailing whitespace",
                ),
            Expression::Cow { prefix, expression } => prefix
                .clone()
                .merge(&expression.source(), "Expression should follow whitespace"),
            Expression::Group(group) => group.source().clone(),
            Expression::Tuple(tuple) => tuple.source().clone(),
            Expression::Concat(concat) => concat.source().clone(),
            Expression::Prefixed(prefix_operator, expression) => prefix_operator
                .source()
                .clone()
                .merge(&expression.source(), "Expression should follow operator"),
            Expression::Index(left, open_bracket, index, close_bracket) => left
                .source()
                .merge(open_bracket, "Open bracket should follow left expression")
                .merge(&index.source(), "Index should follow open bracket")
                .merge(close_bracket, "Close bracket should follow index"),
            Expression::FieldOrMethod(field_or_method) => field_or_method.source().clone(),
        }
    }
}

pub(super) fn expression<'a>(
    allow_nesting: bool,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, Expression<'a>> {
    move |tokens| {
        let (tokens, mut expression) = alt((
            parse_cow_prefix,
            into(Char::parse),
            into(String::parse),
            into(Number::parse),
            into(Bool::parse),
            identifier,
            parse_prefixed_expression,
            into(Group::parse),
            Tuple::parse,
            full_range,
        ))
        .parse(tokens)?;

        if !allow_nesting {
            return Ok((tokens, expression));
        }

        let (tokens, expression_callbacks): (TokenSlice, Vec<Box<NestedExpression<'a>>>) = many0(
            alt((filters, Concat::parser, calc, index, FieldOrMethod::parser)),
        )
        .parse(tokens)?;

        for callback in expression_callbacks {
            expression = callback(expression);
        }

        expression.fix_precedence();

        Ok((tokens, expression))
    }
}

fn calc<'a>(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
    let (tokens, operator) = parse_operator.parse(tokens)?;

    let (tokens, right) = if operator.requires_expression_after() {
        let (tokens, expression) =
            cut("Expected an expression", expression(false)).parse(tokens)?;
        (tokens, Some(expression))
    } else {
        ignore_recoverable_errors(expression(false)).parse(tokens)?
    };

    let callback = Box::new(|left: Expression<'a>| -> Expression<'a> {
        Expression::Calc {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    });

    Ok((tokens, callback))
}

/// Parses a full range expression (`..`).
/// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
fn full_range(tokens: TokenSlice) -> Res<Expression> {
    let (tokens, token) = take(TokenKind::RangeExclusive).parse(tokens)?;

    Ok((
        tokens,
        Expression::FullRange {
            source: token.source().clone(),
        },
    ))
}

/// Parses an index expression (`expr[expr]`).
/// See: <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
fn index<'a>(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
    let (tokens, (open, (range, close))) = (
        take(TokenKind::OpenBracket),
        cut(
            "Expected an expression",
            (expression(true), take(TokenKind::CloseBracket)),
        ),
    )
        .parse(tokens)?;

    Ok((
        tokens,
        Box::new(|expression: Expression<'a>| {
            Expression::Index(
                Box::new(expression),
                open.source().clone(),
                Box::new(range),
                close.source().clone(),
            )
        }),
    ))
}

/// Parses filters (`expr | filter()`).
fn filters<'a>(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
    let (tokens, (vertical_bar, cow_prefix, name, arguments)) = (
        take(TokenKind::VerticalBar),
        ignore_recoverable_errors(take(TokenKind::GreaterThan)),
        cut("Expected a filter name", Identifier::parse),
        ignore_recoverable_errors(arguments),
    )
        .parse(tokens)?;

    let callback = Box::new(move |expression: Expression<'a>| Expression::Filter {
        name,
        expression: Box::new(expression),
        vertical_bar: vertical_bar.source().clone(),
        cow_prefix: cow_prefix.map(|token| token.source().clone()),
        arguments,
    });

    Ok((tokens, callback))
}

fn parse_cow_prefix(tokens: TokenSlice) -> Res<Expression> {
    let (tokens, (prefix, expression)) = (
        take(TokenKind::GreaterThan),
        cut("Expected an expression after cow prefix", expression(false)),
    )
        .parse(tokens)?;

    Ok((
        tokens,
        Expression::Cow {
            prefix: prefix.source().clone(),
            expression: Box::new(expression),
        },
    ))
}
