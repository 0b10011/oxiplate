use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::cut;
use nom::error::context;
use nom::multi::many_till;
use nom::sequence::pair;

use super::{Expression, Res};
use crate::Source;

/// Parses a bool value: `true` or `false`
pub(super) fn bool(input: Source) -> Res<Source, Expression> {
    let (input, source) = alt((tag("true"), tag("false")))(input)?;
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
    alt((binary, decimal))(input)
}

/// Parse binary literals with a `0b` prefix.
/// Will fail if there's not at least one 1 or 0 following the prefix.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
fn binary(input: Source) -> Res<Source, Expression> {
    let (input, number) = pair(
        tag("0b"),
        cut(take_while1(|char: char| char == '0' || char == '1')),
    )(input)?;
    Ok((input, Expression::Number(number.0.merge(&number.1))))
}

/// Parse decimal literals.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
fn decimal(input: Source) -> Res<Source, Expression> {
    let (input, number) = take_while1(|char: char| char.is_ascii_digit())(input)?;
    Ok((input, Expression::Number(number)))
}

pub(super) fn string(input: Source) -> Res<Source, Expression> {
    let (input, (opening_hashes, _opening_quote)) =
        pair(take_while(|c| c == '#'), char('"'))(input)?;

    let closing = pair(char('"'), tag(opening_hashes.as_str()));
    let (input, (string, _)) = context(
        r#"String is opened but never closed. The string ending must be a double quote (") followed by the same number of hashes (#) as the string opening."#,
        cut(many_till(take(1u32), closing)),
    )(input)?;
    let (input, _closing_hashes) = tag(opening_hashes.as_str())(input)?;

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
