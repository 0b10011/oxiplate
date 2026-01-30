use crate::config::tokenizer::TokenKind;
use crate::config::{Config, InferEscaperGroupFromFileExtension, TokenSlice};
use crate::parser::{Error, Parser as _, alt, cut, into, opt, parse_all, take};
use crate::{OptimizedRenderer, Source};

type Res<'a, S> = crate::parser::Res<'a, TokenKind, S>;

pub fn parse<'a>(tokens: TokenSlice<'a>) -> Res<'a, Config> {
    let (tokens, items) = parse_all(Item::parse).parse(tokens)?;

    let mut config = Config::default();

    for item in items {
        match item {
            Item::Comment(_source) | Item::Newline(_source) => (),
            Item::Expression(expression) => {
                macro_rules! set_field {
                    ($field:ident, $expected_kind:ident, $message:literal,) => {
                        match expression.value {
                            Value::$expected_kind(value) => {
                                config.$field = value.into();
                            }
                            _ => {
                                return Err(Error::Recoverable {
                                    message: $message.to_string(),
                                    source: expression.source,
                                    previous_error: None,
                                    is_eof: false,
                                });
                            }
                        }
                    };
                }

                match expression.key.value {
                    "fallback_escaper_group" => set_field!(
                        fallback_escaper_group,
                        String,
                        "Boolean value not allowed for `fallback_escaper_group`",
                    ),
                    "require_specifying_escaper" => set_field!(
                        require_specifying_escaper,
                        Bool,
                        "String value not allowed for `require_specifying_escaper`",
                    ),
                    "infer_escaper_group_from_file_extension" => set_field!(
                        infer_escaper_group_from_file_extension,
                        Bool,
                        "String value not allowed for `infer_escaper_group_from_file_extension`",
                    ),
                    "optimized_renderer" => set_field!(
                        optimized_renderer,
                        Bool,
                        "String value not allowed for `optimized_renderer`",
                    ),
                    _ => todo!("handle escaper groups hashmap"),
                }
            }
        }
    }

    Ok((tokens, config))
}

enum Item<'a> {
    Comment(Source<'a>),
    Expression(Expression<'a>),
    Newline(Source<'a>),
}

impl<'a> Item<'a> {
    fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((Self::expression, Self::comment, Self::newline)).parse(tokens)
    }

    fn expression(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
        let (tokens, (key, equal, value, comment, newline)): (_, (Key, _, _, _, _)) = (
            into(string),
            cut("`=` expected after key", take(TokenKind::Equal)),
            cut("Boolean or string value expected after `=`", value),
            opt(take(TokenKind::Comment)),
            cut(
                "Newline or end of file expected after expression",
                newline_or_eof,
            ),
        )
            .parse(tokens)?;

        let source = key
            .source()
            .clone()
            .merge(equal.source(), "`=` expected after key`")
            .merge(value.source(), "Value expected after `=`")
            .merge_some(
                comment.map(|token| token.source()),
                "Comment expected after value",
            )
            .merge(newline, "Newline or end of file expected after comment");

        let expression = Expression { key, value, source };

        Ok((tokens, Self::Expression(expression)))
    }

    fn comment(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
        let (tokens, (comment, newline)) = (
            take(TokenKind::Comment),
            cut(
                "Newline or end of file expected after comment",
                newline_or_eof,
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self::Comment(
                comment
                    .source()
                    .clone()
                    .merge(newline, "Newline expected after comment"),
            ),
        ))
    }

    fn newline(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
        let (tokens, newline) = take(TokenKind::Newline).parse(tokens)?;

        Ok((tokens, Self::Newline(newline.source().clone())))
    }
}

fn string<'a>(tokens: TokenSlice<'a>) -> Res<'a, StringValue<'a>> {
    let (tokens, token) = tokens.take()?;

    let value = match token.kind() {
        TokenKind::String(value) => StringValue {
            value,
            source: token.source(),
        },
        _ => {
            return Err(Error::Recoverable {
                message: format!(r"Expected a string, found: {token:#?}"),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        }
    };

    Ok((tokens, value))
}

fn value<'a>(tokens: TokenSlice<'a>) -> Res<'a, Value<'a>> {
    let (tokens, token) = tokens.take()?;

    let value = match token.kind() {
        TokenKind::String(value) => Value::String(StringValue {
            value,
            source: token.source(),
        }),
        TokenKind::Bool(value) => Value::Bool(BoolValue {
            value: *value,
            source: token.source(),
        }),
        _ => {
            return Err(Error::Recoverable {
                message: format!("Expected a string or boolean value, found: {token:#?}"),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        }
    };

    Ok((tokens, value))
}

fn newline_or_eof<'a>(tokens: TokenSlice<'a>) -> Res<'a, &'a Source<'a>> {
    if tokens.is_empty() {
        let eof = tokens.eof().source();
        return Ok((tokens, eof));
    }

    let (tokens, token) = tokens.take()?;

    match token.kind() {
        TokenKind::Newline => (),
        _ => {
            return Err(Error::Recoverable {
                message: format!(r"Expected a newline (`\n` or `\r\n`), found: {token:#?}"),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        }
    }

    Ok((tokens, token.source()))
}

struct Expression<'a> {
    key: Key<'a>,
    value: Value<'a>,
    source: Source<'a>,
}

struct Key<'a> {
    value: &'a str,
    source: &'a Source<'a>,
}

impl<'a> Key<'a> {
    pub fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

enum Value<'a> {
    Bool(BoolValue<'a>),
    String(StringValue<'a>),
}

impl<'a> Value<'a> {
    pub fn source(&self) -> &'a Source<'a> {
        match self {
            Self::Bool(bool_value) => bool_value.source(),
            Self::String(string_value) => string_value.source(),
        }
    }
}

struct StringValue<'a> {
    value: &'a str,
    source: &'a Source<'a>,
}

impl<'a> StringValue<'a> {
    pub fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

impl<'a> From<StringValue<'a>> for Key<'a> {
    fn from(value: StringValue<'a>) -> Self {
        let StringValue { value, source } = value;

        Self { value, source }
    }
}

impl<'a> From<StringValue<'a>> for Option<String> {
    fn from(value: StringValue<'a>) -> Self {
        let StringValue { value, source: _ } = value;

        Some(value.to_owned())
    }
}

struct BoolValue<'a> {
    value: bool,
    source: &'a Source<'a>,
}

impl<'a> BoolValue<'a> {
    pub fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

impl<'a> From<BoolValue<'a>> for bool {
    fn from(value: BoolValue<'a>) -> Self {
        let BoolValue { value, source: _ } = value;

        value
    }
}

impl<'a> From<BoolValue<'a>> for InferEscaperGroupFromFileExtension {
    fn from(value: BoolValue<'a>) -> Self {
        let BoolValue { value, source: _ } = value;

        Self(value)
    }
}

impl<'a> From<BoolValue<'a>> for OptimizedRenderer {
    fn from(value: BoolValue<'a>) -> Self {
        let BoolValue { value, source: _ } = value;

        Self(value)
    }
}
