use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{cut, not, opt, peek};
use nom::error::context;
use nom::multi::many_till;
use nom::sequence::{pair, preceded, terminated};
use nom::{AsChar as _, Parser as _};

use super::{Expression, Res};
use crate::Source;

/// Parses a bool value: `true` or `false`
pub(super) fn bool(input: Source) -> Res<Source, Expression> {
    let (input, source) = alt((tag("true"), tag("false"))).parse(input)?;
    let bool = match source.as_str() {
        "true" => true,
        "false" => false,
        _ => unreachable!("All cases should be covered"),
    };

    Ok((input, Expression::Bool(bool, source)))
}

/// Parse a number.
/// See: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
pub(super) fn number(input: Source) -> Res<Source, Expression> {
    alt((alternative_bases, decimal)).parse(input)
}

/// Parse alternative base number literals with a `0b`, `0x`, and `0o` prefixes.
/// Will fail if there's not at least one valid digit following the prefix.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
fn alternative_bases(input: Source) -> Res<Source, Expression> {
    let (input, prefix) =
        peek(preceded(char('0'), alt((char('b'), char('x'), char('o'))))).parse(input)?;
    let (prefix, matcher) = match prefix {
        'b' => ("0b", char::is_bin_digit as fn(char) -> bool),
        'x' => ("0x", char::is_hex_digit as fn(char) -> bool),
        'o' => ("0o", char::is_oct_digit as fn(char) -> bool),
        _ => unimplemented!("All alternative base prefix cases should be covered"),
    };

    let (input, number) = pair(
        tag(prefix),
        cut((
            take_while(|char: char| char == '_'),
            take_while1(|char: char| matcher(char)),
            take_while(|char: char| char == '_' || matcher(char)),
        )),
    )
    .parse(input)?;

    Ok((
        input,
        Expression::Integer(
            number
                .0
                .merge(&number.1 .0)
                .merge(&number.1 .1)
                .merge(&number.1 .2),
        ),
    ))
}

/// Parse decimal and floating-point literals.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
fn decimal(input: Source) -> Res<Source, Expression> {
    let (input, integer) = integer_literal.parse(input)?;

    // Decimal points won't exist for integers (with or without exponents).
    // E.g., `19` or `19e0`.
    let (input, point) = opt(terminated(tag("."), not(tag(".")))).parse(input)?;
    let Some(point) = point else {
        let (input, exponent) = opt(exponent).parse(input)?;

        return if let Some(exponent) = exponent {
            Ok((input, Expression::Float(integer.merge(&exponent))))
        } else {
            Ok((input, Expression::Integer(integer)))
        };
    };

    // Parse the fractional part of the decimal and the exponent, if any.
    let (input, end) = opt(pair(integer_literal, opt(exponent))).parse(input)?;

    // If there's no fractional part or exponent,
    // the float (e.g., `19.`) can be returned early.
    let Some((fractional, exponent)) = end else {
        return Ok((input, Expression::Float(integer.merge(&point))));
    };

    Ok((
        input,
        Expression::Float(
            integer
                .merge(&point)
                .merge(&fractional)
                .merge_some(exponent.as_ref()),
        ),
    ))
}

/// Parse float exponent (e.g., `e-1`, `E+2`, or `e3`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-FLOAT_EXPONENT>
fn exponent(input: Source) -> Res<Source, Source> {
    let (input, (e, sign, separators, number)) = (
        alt((tag("e"), tag("E"))),
        opt(alt((tag("-"), tag("+")))),
        take_while(|char: char| char == '_'),
        cut(integer_literal),
    )
        .parse(input)?;

    Ok((
        input,
        e.merge_some(sign.as_ref())
            .merge(&separators)
            .merge(&number),
    ))
}

/// Parse decimal literals (e.g., `19`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
fn integer_literal(input: Source) -> Res<Source, Source> {
    let (input, number) = (
        take_while1(|char: char| char.is_ascii_digit()),
        take_while(|char: char| char.is_ascii_digit() || char == '_'),
    )
        .parse(input)?;

    Ok((input, number.0.merge(&number.1)))
}

pub(super) fn string(input: Source) -> Res<Source, Expression> {
    let (input, (opening_hashes, _opening_quote)) =
        pair(take_while(|c| c == '#'), char('"')).parse(input)?;

    let closing = pair(char('"'), tag(opening_hashes.as_str()));
    let (input, (string, _)) = context(
        r#"String is opened but never closed. The string ending must be a double quote (") followed by the same number of hashes (#) as the string opening."#,
        cut(many_till(take(1u32), closing)),
    ).parse(input)?;
    let (input, _closing_hashes) = tag(opening_hashes.as_str()).parse(input)?;

    let full_string = if let Some(full_string) = string.first() {
        let mut full_string = full_string.clone();
        full_string.range.end = string.last().unwrap().range.end;
        full_string
    } else {
        let mut full_string = opening_hashes.clone();
        full_string.range.start = full_string.range.end;
        full_string
    };
    Ok((input, Expression::String(full_string)))
}
