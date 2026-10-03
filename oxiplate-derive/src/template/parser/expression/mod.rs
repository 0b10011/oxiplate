mod arguments;
mod array;
mod calc;
mod call;
mod concat;
mod cow;
mod fields;
mod filter;
mod full_range;
mod generics;
mod group;
mod index;
mod keyword;
mod literal;
mod operator;
mod path;
mod prefix_operator;
mod prefixed;
mod tuple;

use std::mem;

use self::array::Array;
use self::calc::Calc;
use self::call::Call;
use self::concat::Concat;
use self::cow::Cow;
use self::fields::Fields;
use self::filter::Filter;
use self::full_range::FullRange;
use self::group::Group;
use self::index::Index;
pub(super) use self::keyword::{Keyword, KeywordParser};
pub(super) use self::literal::{Bool, Char, Float, Integer, Number, String};
pub(super) use self::path::Identifier;
use self::path::Path;
use self::prefixed::Prefixed;
use self::tuple::Tuple;
use super::Res;
use super::expression::operator::parse_operator;
use crate::template::parser::prelude::*;

type NestedExpression<'a> = dyn FnOnce(Expression<'a>) -> Expression<'a> + 'a;

#[derive(Debug, Default)]
pub(crate) enum Expression<'a> {
    /// Placeholder expression used during precedence fixing.
    /// Will generate a compile error if not replaced.
    #[default]
    Placeholder,

    Path(Path<'a>),
    Char(Char<'a>),
    String(String<'a>),
    Integer(Integer<'a>),
    Float(Float<'a>),
    Bool(Bool<'a>),
    Group(Group<'a>),
    Array(Array<'a>),
    Tuple(Tuple<'a>),
    Concat(Concat<'a>),
    Calc(Calc<'a>),
    Prefixed(Prefixed<'a>),
    Cow(Cow<'a>),

    /// `..` that represents a range
    /// where the start/end matches whatever it is applied to.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
    FullRange(FullRange<'a>),

    /// `expr[expr]`
    /// See:
    /// - <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
    /// - <https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices>
    Index(Index<'a>),

    /// `expr | filter(args)`
    Filter(Filter<'a>),

    /// `expr.field`
    Fields(Fields<'a>),

    /// `expr(args)`
    Call(Call<'a>),
}

impl<'a> Expression<'a> {
    /// Rearranges expression tree to match expression precedence.
    pub(self) fn fix_precedence(&mut self) {
        // Fix the precedence of the left expression, if any.
        match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => unreachable!(
                "Placeholder expression should not have `fix_precedence()` called for it"
            ),

            // Single-expression items
            // or those with expressions only on the right side
            // do not need to be changed.
            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::Path(_)
            | Self::Prefixed(_)
            | Self::Cow { .. } => (),

            // Fix the precedence of the leftmost expression.
            Self::Index(index) => index.expression.fix_precedence(),
            Self::Calc(calc) => calc.left.fix_precedence(),
            Self::Filter(filter) => filter.expression.fix_precedence(),
            Self::Fields(fields) => {
                fields.expression.as_mut().fix_precedence();
            }
            Self::Concat(concat) => concat.first_expression.fix_precedence(),
            Self::Call(call) => call.expression.fix_precedence(),
        }

        // If the precedence is already correct, bail early.
        if !self.needs_precedence_fixed() {
            return;
        }

        // Grab the leftmost expression.
        let mut left = self.take_left();

        // Grab the rightmost expression from the left expression.
        let Some(mut lefts_right) = left.take_right() else {
            // Return the leftmost expression if there's no rightmost expression.
            self.give_left(&mut left);
            return;
        };

        // Fix the precedence of the expression
        // about to become the leftmost expression of this one.
        lefts_right.fix_precedence();

        // Move the middle expression from the left one to this one,
        // then move this one onto the left expression,
        // and finally make it official.
        self.give_left(&mut lefts_right);
        left.give_right(self);
        mem::swap(&mut left, self);
    }

    /// Check if the precedence of the left expression
    /// is lower than the precedence of this expression
    /// and therefore needs to be rearranged
    /// to fix the precedence of the final expression.
    fn needs_precedence_fixed(&self) -> bool {
        let left = match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => unreachable!(
                "Placeholder expression should not have `needs_precedence_fixed()` called for it"
            ),

            // Single-expression items
            // or those with expressions only on the right side
            // do not need to be changed.
            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::Path(_)
            | Self::Prefixed(_)
            | Self::Cow { .. } => return false,

            // Grab the leftmost expression.
            Self::Index(index) => &index.expression,
            Self::Calc(calc) => &calc.left,
            Self::Filter(filter) => &filter.expression,
            Self::Fields(fields) => fields.expression.as_ref(),
            Self::Concat(concat) => concat.first_expression.as_ref(),
            Self::Call(call) => call.expression.as_ref(),
        };

        // Ensure the precedence of the left and this expression are correct.
        left.precedence() < self.precedence()
    }

    /// Take the left expression,
    /// replacing it with `Expression::Placeholder` temporarily
    /// until a new expression is placed via `give_left()`.
    fn take_left(&mut self) -> Expression<'a> {
        match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => {
                unreachable!("Placeholder expression should not have `take_left()` called for it")
            }

            // No expression on the left.
            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::Path(_)
            | Self::Prefixed(_)
            | Self::Cow { .. } => unreachable!(
                "Expressions without an expression on the left should never have `take_left()` \
                 called for them"
            ),

            // Take the leftmost expression and return it.
            Self::Index(index) => mem::take(&mut index.expression),
            Self::Calc(calc) => mem::take(&mut calc.left),
            Self::Filter(filter) => mem::take(&mut filter.expression),
            Self::Fields(fields) => mem::take(&mut fields.expression),
            Self::Concat(concat) => mem::take(&mut concat.first_expression),
            Self::Call(call) => mem::take(&mut call.expression),
        }
    }

    /// Give a new expression for the left side of this one,
    /// usually taken via `take_right()`,
    /// replacing the `Expression::Placeholder` set by `take_left()`.
    fn give_left(&mut self, new_left: &mut Expression<'a>) {
        let placeholder_left = match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => {
                unreachable!("Placeholder expression should not have `give_left()` called for it")
            }

            // No expression on the left.
            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::Path(_)
            | Self::Prefixed(_)
            | Self::Cow { .. } => {
                unreachable!(
                    "Only expressions that hold an expression on the left side should ever be \
                     given a left expression"
                )
            }

            // Take a mutable reference to the placeholder.
            Self::Index(index) => index.expression.as_mut(),
            Self::Calc(calc) => calc.left.as_mut(),
            Self::Filter(filter) => filter.expression.as_mut(),
            Self::Fields(fields) => fields.expression.as_mut(),
            Self::Concat(concat) => concat.first_expression.as_mut(),
            Self::Call(call) => call.expression.as_mut(),
        };

        // Ensure the expression is actually a placeholder.
        if !matches!(placeholder_left, Self::Placeholder) {
            unreachable!("Only placeholder expressions should ever be overwritten");
        }

        // Replace the placeholder with the new left expression.
        mem::swap(placeholder_left, new_left);
    }

    /// Take the right expression,
    /// replacing it with `Expression::Placeholder` temporarily
    /// until a new expression is placed via `give_right()`.
    fn take_right(&mut self) -> Option<Expression<'a>> {
        match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => {
                unreachable!("Placeholder expression should not have `take_right()` called for it")
            }

            // No expression on the right.
            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::Index(_)
            | Self::Filter { .. }
            | Self::Path(_)
            | Self::Fields(_)
            | Self::Call(_) => None,

            // Take the rightmost expression and return it.
            Self::Concat(concat) => match concat.additional_expressions.last_mut() {
                Some((_tilde, expression)) => Some(mem::take(expression)),
                None => unreachable!("Concats should always contain at least 2 expressions"),
            },
            Self::Calc(calc) => match calc.right.as_mut() {
                Some(right) => Some(mem::take(right)),
                None => None,
            },
            Self::Prefixed(prefixed) => Some(mem::take(&mut prefixed.expression)),
            Self::Cow(cow) => Some(mem::take(cow.expression.as_mut())),
        }
    }

    /// Give a new expression for the right side of this one,
    /// usually taken via `take_left()`,
    /// replacing the `Expression::Placeholder` set by `take_right()`.
    fn give_right(&mut self, new_right: &mut Expression<'a>) {
        let placeholder_right = match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => {
                unreachable!("Placeholder expression should not have `give_right()` called for it")
            }

            // No expression on the right.
            Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::FullRange { .. }
            | Self::Group(_)
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::Index(_)
            | Self::Filter { .. }
            | Self::Path(_)
            | Self::Fields(_)
            | Self::Call(_) => {
                unreachable!(
                    "Only expressions that hold an expression on the right side should ever be \
                     given a right expression"
                )
            }

            // Take a mutable reference to the placeholder.
            Self::Concat(concat) => match concat.additional_expressions.last_mut() {
                Some((_tilde, last)) => last,
                None => unreachable!("Concats should have at least 2 expressions"),
            },
            Self::Calc(calc) => match calc.right.as_mut() {
                Some(right) => right,
                None => unreachable!("Only placeholder expressions should ever be overwritten"),
            },
            Self::Prefixed(prefixed) => &mut prefixed.expression,
            Self::Cow(cow) => cow.expression.as_mut(),
        };

        // Ensure the expression is actually a placeholder.
        if !matches!(placeholder_right, Self::Placeholder) {
            unreachable!("Only placeholder expressions should ever be overwritten");
        }

        // Replace the placeholder with the new right expression.
        mem::swap(placeholder_right, new_right);
    }

    /// Get the precedence of the current expression.
    fn precedence(&self) -> u8 {
        match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => {
                unreachable!("Placeholder expression should not have `precedence()` called for it")
            }

            // Rust expressions are assumed to be the same precedence
            // to let Rust handle the details.
            Self::Fields(_)
            | Self::Call(_)
            | Self::Path(_)
            | Self::Index(_)
            | Self::Calc(_)
            | Self::Prefixed(_) => u8::MAX,

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
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::FullRange { .. } => 0,
        }
    }

    /// Merges expressions that can be joined together.
    pub(self) fn merge_joinable(&mut self) {
        match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => {
                unreachable!(
                    "Placeholder expression should not have `merge_joinable()` called for it"
                )
            }

            // No expressions to merge.
            Self::Path(_)
            | Self::Char(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bool(_)
            | Self::Group(_)
            | Self::Array(_)
            | Self::Tuple(_)
            | Self::FullRange { .. } => (),

            // Concatenations are joinable
            // resulting in a single `format!()` call
            // per group of concatenated items.
            Self::Concat(Concat {
                first_expression,
                additional_expressions,
            }) => {
                // Only the first and last expressions should need to be merged.
                // The middle ones should always be
                // single-expression items
                // or items merged via this method.
                // The last expression is handled later.
                first_expression.merge_joinable();

                // If the first item is also a concatenation,
                // remove it,
                // insert the first item from it here,
                // and prepend the additional expressions from it
                // to this concat's additional expressions.
                if matches!(first_expression.as_ref(), Expression::Concat(_)) {
                    let Self::Concat(Concat {
                        first_expression: mut new_first_expression,
                        additional_expressions: mut additional_expressions_to_add,
                    }) = *mem::take(first_expression)
                    else {
                        unreachable!(
                            "Expression is verified to be `Concat` immediately before doing the \
                             destructuring"
                        );
                    };

                    mem::swap(first_expression, &mut new_first_expression);
                    mem::swap(additional_expressions, &mut additional_expressions_to_add);
                    additional_expressions.extend(additional_expressions_to_add);
                }

                // If the last item is also a concatenation,
                // remove it,
                // merge joinable,
                // and append all of the parts to this concat.
                // Otherwise,
                // just merge joinable.
                if matches!(
                    additional_expressions.last(),
                    Some((_, Expression::Concat(_)))
                ) {
                    let (tilde, mut additional_expressions_to_add) = additional_expressions
                        .pop()
                        .expect("There should always be at least one expression");

                    additional_expressions_to_add.merge_joinable();

                    let Self::Concat(Concat {
                        first_expression: first_expression_to_add,
                        additional_expressions: additional_expressions_to_add,
                    }) = additional_expressions_to_add
                    else {
                        unreachable!(
                            "Expression is confirmed to be `Concat` just a few statements up"
                        );
                    };

                    additional_expressions.push((tilde, *first_expression_to_add));
                    additional_expressions.extend(additional_expressions_to_add);
                } else {
                    let (_tilde, additional_expressions_to_add) = additional_expressions
                        .last_mut()
                        .expect("There should always be at least one expression");

                    additional_expressions_to_add.merge_joinable();
                }
            }

            // Other expressions contain expressions
            // that may need to have their expressions merged.
            // Expressions that allow nesting,
            // like the expression between brackets in index expressions,
            // already handled merging joinable.
            Self::Calc(calc) => {
                calc.left.merge_joinable();
                if let Some(right) = calc.right.as_mut() {
                    right.merge_joinable();
                }
            }
            Self::Prefixed(prefixed) => {
                prefixed.expression.merge_joinable();
            }
            Self::Index(index) => index.expression.merge_joinable(),
            Self::Cow(cow) => cow.expression.merge_joinable(),
            Self::Filter(filter) => filter.expression.merge_joinable(),
            Self::Fields(fields) => fields.expression.merge_joinable(),
            Self::Call(call) => call.expression.merge_joinable(),
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> BuiltTokens {
        match self {
            Expression::Placeholder => (
                quote! { compile_error!("Placeholder expression was never replaced.") },
                EstimatedLength::new(0),
            ),
            Expression::Path(path) => path.to_tokens_with_state(state),
            Expression::Group(group) => group.to_tokens_with_state(state),
            Expression::Array(array) => array.to_tokens_with_state(state),
            Expression::Tuple(tuple) => tuple.to_tokens_with_state(state),
            Expression::Concat(concat) => concat.to_tokens_with_state(state),
            Expression::Calc(calc) => calc.to_tokens_with_state(state),
            Expression::Prefixed(prefixed) => prefixed.to_tokens_with_state(state),
            Expression::Cow(cow) => cow.to_tokens_with_state(state),
            Expression::Char(char) => char.to_tokens(),
            Expression::String(string) => string.to_tokens(),
            Expression::Integer(number) => number.to_tokens(),
            Expression::Float(number) => number.to_tokens(),
            Expression::Bool(bool) => bool.to_tokens(),
            Expression::FullRange(full_range) => full_range.to_tokens(state),
            Expression::Index(index) => index.to_tokens(state),
            Expression::Filter(filter) => filter.to_tokens(state),
            Expression::Fields(fields) => fields.to_tokens(state),
            Expression::Call(call) => call.to_tokens(state),
        }
    }

    /// Get the `Source` for the expression.
    pub(crate) fn source(&self) -> Source<'a> {
        match self {
            // Placeholder is only temporary
            // and should never have this method called for it.
            Self::Placeholder => {
                unreachable!("Placeholder expression should not have `source()` called for it")
            }

            Expression::Path(identifier_or_function) => identifier_or_function.source(),
            Expression::Char(value) => value.source().clone(),
            Expression::String(value) => value.source().clone(),
            Expression::Integer(value) => value.source().clone(),
            Expression::Float(value) => value.source().clone(),
            Expression::Bool(value) => value.source().clone(),
            Expression::Calc(calc) => calc.source().clone(),
            Expression::FullRange(full_range) => full_range.source().clone(),
            Expression::Filter(filter) => filter.source(),
            Expression::Cow(cow) => cow.source().clone(),
            Expression::Group(group) => group.source().clone(),
            Expression::Array(array) => array.source().clone(),
            Expression::Tuple(tuple) => tuple.source().clone(),
            Expression::Concat(concat) => concat.source().clone(),
            Expression::Prefixed(prefixed) => prefixed.source().clone(),
            Expression::Index(index) => index.source().clone(),
            Expression::Fields(fields) => fields.source().clone(),
            Expression::Call(call) => call.source().clone(),
        }
    }
}

pub(super) fn expression<'a>(
    allow_nesting: bool,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, Expression<'a>> {
    move |tokens| {
        let (tokens, mut expression) = alt((
            into(Cow::parse),
            into(Char::parse),
            into(String::parse),
            into(Number::parse),
            into(Bool::parse),
            into(Path::parser),
            into(Prefixed::parse),
            into(Group::parse),
            Tuple::parse,
            Array::parse,
            into(FullRange::parse),
        ))
        .parse(tokens)?;

        if !allow_nesting {
            return Ok((tokens, expression));
        }

        let (tokens, expression_callbacks): (TokenSlice, Vec<Box<NestedExpression<'a>>>) =
            many0(alt((
                Filter::parse,
                Concat::parser,
                Calc::parse,
                Index::parse,
                Fields::parser,
                Call::parser,
            )))
            .parse(tokens)?;

        for callback in expression_callbacks {
            expression = callback(expression);
        }

        expression.fix_precedence();

        expression.merge_joinable();

        Ok((tokens, expression))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod test {
    use super::Char;
    use super::concat::Concat;
    use crate::source::test_source;
    use crate::template::parser::expression::full_range::FullRange;
    use crate::{EstimatedLength, State};

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `fix_precedence()` called for it"]
    fn fix_precedence_on_placeholder() {
        super::Expression::Placeholder.fix_precedence();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `needs_precedence_fixed()` called for it"]
    fn needs_precedence_fixed_on_placeholder() {
        super::Expression::Placeholder.needs_precedence_fixed();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `take_left()` called for it"]
    fn take_left_on_placeholder() {
        super::Expression::Placeholder.take_left();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Expressions without an expression \
                      on the left should never have `take_left()` called for them"]
    fn take_left_on_full_range() {
        test_source!(source = "..");
        super::Expression::FullRange(FullRange::new_for_test(source)).take_left();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `give_left()` called for it"]
    fn give_left_on_placeholder() {
        super::Expression::Placeholder.give_left(&mut super::Expression::Placeholder);
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Only expressions that hold an \
                      expression on the left side should ever be given a left expression"]
    fn give_left_on_full_range() {
        test_source!(source = "..");
        super::Expression::FullRange(FullRange::new_for_test(source))
            .give_left(&mut super::Expression::Placeholder);
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Only placeholder expressions \
                      should ever be overwritten"]
    fn give_left_with_placeholder() {
        test_source!(a = "'a'");
        test_source!(concat_b = " ~ 'b'");
        test_source!(b = " 'b'");
        let a = Char::new_for_test('a', &a).into();
        let b = (concat_b, Char::new_for_test('b', &b).into());
        super::Expression::Concat(Concat {
            first_expression: Box::new(a),
            additional_expressions: vec![b],
        })
        .give_left(&mut super::Expression::Placeholder);
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `take_right()` called for it"]
    fn take_right_on_placeholder() {
        super::Expression::Placeholder.take_right();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Concats should always contain at \
                      least 2 expressions"]
    fn take_right_on_concat_empty_additional() {
        test_source!(a = "'a'");
        let a = Char::new_for_test('a', &a).into();
        super::Expression::Concat(Concat {
            first_expression: Box::new(a),
            additional_expressions: vec![],
        })
        .take_right();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `give_right()` called for it"]
    fn give_right_on_placeholder() {
        super::Expression::Placeholder.give_right(&mut super::Expression::Placeholder);
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Only expressions that hold an \
                      expression on the right side should ever be given a right expression"]
    fn give_right_on_full_range() {
        test_source!(source = "..");
        super::Expression::FullRange(FullRange::new_for_test(source))
            .give_right(&mut super::Expression::Placeholder);
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Only placeholder expressions \
                      should ever be overwritten"]
    fn give_right_with_placeholder() {
        test_source!(a = "'a'");
        test_source!(concat_b = " ~ 'b'");
        test_source!(b = " 'b'");
        let a = Char::new_for_test('a', &a).into();
        let b = (concat_b, Char::new_for_test('b', &b).into());
        super::Expression::Concat(Concat {
            first_expression: Box::new(a),
            additional_expressions: vec![b],
        })
        .give_right(&mut super::Expression::Placeholder);
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `precedence()` called for it"]
    fn precedence_on_placeholder() {
        super::Expression::Placeholder.precedence();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `merge_joinable()` called for it"]
    fn merge_joinable_on_placeholder() {
        super::Expression::Placeholder.merge_joinable();
    }

    #[test]
    #[should_panic = "There should always be at least one expression"]
    fn merge_joinable_on_concat_with_empty_additional() {
        test_source!(a = "'a'");
        let a = Char::new_for_test('a', &a).into();
        super::Expression::Concat(Concat {
            first_expression: Box::new(a),
            additional_expressions: vec![],
        })
        .merge_joinable();
    }

    #[test]
    fn to_tokens() {
        let (tokens, length) = super::Expression::Placeholder.to_tokens(&State::new_for_test());

        assert_eq!(length, EstimatedLength::new(0));
        assert_eq!(
            format!("{tokens}"),
            r#"compile_error ! ("Placeholder expression was never replaced.")"#
        );
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Placeholder expression should not \
                      have `source()` called for it"]
    fn source() {
        super::Expression::Placeholder.source();
    }
}
