use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while1};
use nom::character::complete::{char as nom_char, none_of};
use nom::combinator::{cut, not, opt, peek};
use nom::error::context;
use nom::multi::many_till;
use nom::sequence::{pair, preceded, terminated};
use nom::{AsChar as _, Parser as _};
use proc_macro::Diagnostic;

use super::{Expression, Res};
use crate::Source;

/// Parses a bool value: `true` or `false`
pub(crate) fn bool(input: Source) -> Res<Source, Expression> {
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
pub(crate) fn number(input: Source) -> Res<Source, Expression> {
    alt((alternative_bases, decimal)).parse(input)
}

/// Parse alternative base number literals with a `0b`, `0x`, and `0o` prefixes.
/// Will fail if there's not at least one valid digit following the prefix.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
fn alternative_bases(input: Source) -> Res<Source, Expression> {
    let (input, prefix) =
        peek(preceded(nom_char('0'), alt((tag("b"), tag("x"), tag("o"))))).parse(input)?;
    let (prefix, matcher) = match prefix.as_str() {
        "b" => ("0b", char::is_bin_digit as fn(char) -> bool),
        "x" => ("0x", char::is_hex_digit as fn(char) -> bool),
        "o" => ("0o", char::is_oct_digit as fn(char) -> bool),
        _ => {
            Diagnostic::spanned(
                prefix.span().unwrap(),
                proc_macro::Level::Error,
                "Internal Oxiplate error. Unhandled alternative base prefix.",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unhandled+alternative+base+prefix")
            .help("Include template that caused the issue.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }
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
                .merge(&number.1.0, "Leading underscores should follow prefix")
                .merge(&number.1.1, "Digits should follow leading underscores")
                .merge(&number.1.2, "Trailing underscores should follow digits"),
        ),
    ))
}

/// Parse decimal and floating-point literals.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
/// See: <https://doc.rust-lang.org/reference/tokens.html#floating-point-literals>
fn decimal(input: Source) -> Res<Source, Expression> {
    let (input, integer) = integer_literal.parse(input)?;

    // Decimal points won't exist for integers (with or without exponents).
    // E.g., `19` or `19e0`.
    let (input, point) = opt(terminated(tag("."), not(tag(".")))).parse(input)?;
    let Some(point) = point else {
        let (input, exponent) = opt(exponent).parse(input)?;

        return if let Some(exponent) = exponent {
            Ok((
                input,
                Expression::Float(integer.merge(&exponent, "Exponent should follow integer")),
            ))
        } else {
            Ok((input, Expression::Integer(integer)))
        };
    };

    // Parse the fractional part of the decimal and the exponent, if any.
    let (input, end) = opt(pair(integer_literal, opt(exponent))).parse(input)?;

    // If there's no fractional part or exponent,
    // the float (e.g., `19.`) can be returned early.
    let Some((fractional, exponent)) = end else {
        return Ok((
            input,
            Expression::Float(integer.merge(&point, "Decimal point should follow integer")),
        ));
    };

    Ok((
        input,
        Expression::Float(
            integer
                .merge(&point, "Decimal point should follow integer")
                .merge(&fractional, "Fractional should follow decimal point")
                .merge_some(exponent.as_ref(), "Exponent should follow fractional"),
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
        e.merge_some(sign.as_ref(), "Sign should follow 'e'")
            .merge(&separators, "Underscores should follow sign")
            .merge(&number, "Integer should follow underscores"),
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

    Ok((
        input,
        number
            .0
            .merge(&number.1, "Digits/underscores should follow leading digits"),
    ))
}

/// Parse char literal (e.g., `'a'`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
pub(crate) fn char(input: Source) -> Res<Source, Expression> {
    let (input, (opening_quote, value, closing_quote)) = (
        tag("'"),
        context(
            r"Expected `\'`, `\\`, or a single char followed by `'`.",
            cut(alt((
                // Char
                preceded(peek(none_of("'\\\n\r\t")), take(1usize)),
                // Quote/ascii escape
                alt((
                    tag(r"\'"),
                    tag(r#"\""#),
                    tag(r"\n"),
                    tag(r"\r"),
                    tag(r"\t"),
                    tag(r"\\"),
                    tag(r"\0"),
                )),
            ))),
        ),
        context(r"Expected `'`.", cut(tag("'"))),
    )
        .parse(input)?;

    let source = opening_quote
        .merge(&value, "Char should follow opening quote")
        .merge(&closing_quote, "Closing quote should follow char");

    let value = match value.as_str() {
        r"\'" => '\'',
        r#"\""# => '"',
        r"\n" => '\n',
        r"\r" => '\r',
        r"\t" => '\t',
        r"\\" => '\\',
        r"\0" => '\0',
        str => {
            let mut chars = str.chars();
            let Some(char) = chars.next() else {
                Diagnostic::spanned(
                    source.span().unwrap(),
                    proc_macro::Level::Error,
                    "No char present in char expression",
                )
                .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=No+char+present+in+char+expression")
                .help("Include template that caused the issue.")
                .emit();
                unreachable!("Internal Oxiplate error. See previous error for more information.");
            };
            if chars.count() > 0 {
                Diagnostic::spanned(
                    source.span().unwrap(),
                    proc_macro::Level::Error,
                    "More than one char present in char expression",
                )
                .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=More+than+one+char+present+in+char+expression")
                .help("Include template that caused the issue.")
                .emit();
                unreachable!("Internal Oxiplate error. See previous error for more information.");
            }
            char
        }
    };

    Ok((input, Expression::Char { value, source }))
}

pub(crate) fn string(input: Source) -> Res<Source, Expression> {
    let (input, (opening_hashes, opening_quote)) =
        pair(take_while(|c| c == '#'), tag("\"")).parse(input)?;

    let closing = pair(tag("\""), tag(opening_hashes.as_str()));
    let (input, (string, (closing_quote, closing_hashes))) = context(
        r#"String is opened but never closed. The string ending must be a double quote (") followed by the same number of hashes (#) as the string opening."#,
        cut(many_till(take(1u32), closing)),
    ).parse(input)?;

    let value = if let Some(value) = string.first() {
        let mut value = value.clone();
        value.range.end = string.last().unwrap().range.end;
        value
    } else {
        let mut value = opening_quote.clone();
        value.range.start = value.range.end;
        value
    };

    let source = opening_hashes
        .merge(&opening_quote, "Opening quote should follow opening hashes")
        .merge(&value, "Value should follow opening quote")
        .merge(&closing_quote, "Closing quote should follow value")
        .merge(
            &closing_hashes,
            "Closing hashes should follow closing quote",
        );

    Ok((input, Expression::String { value, source }))
}
