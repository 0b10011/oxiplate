use std::collections::HashMap;
use std::mem;

use crate::config::tokenizer::TokenKind;
use crate::config::{Config, EscaperGroup, InferEscaperGroupFromFileExtension, Token, TokenSlice};
use crate::parser::{Error, Parser as _, alt, cut, into, many0, opt, parse_all, take};
use crate::{OptimizedRenderer, Source};

type Res<'a, S> = crate::parser::Res<'a, TokenKind, S>;

type Table<'a> = HashMap<&'a str, TableOrValue<'a>>;

#[derive(Debug)]
enum TableOrValue<'a> {
    Table(Table<'a>),
    Value(Value<'a>),
}

impl<'a> TableOrValue<'a> {
    fn table_mut(&mut self) -> Option<&mut Table<'a>> {
        match self {
            Self::Table(table) => Some(table),
            Self::Value(_value) => None,
        }
    }
}

pub fn parse(tokens: TokenSlice) -> Res<Config> {
    let (tokens, items) = parse_all(Item::parse).parse(tokens)?;

    let (mut data, source) = parse_data(items)?;

    let mut config = Config::default();

    macro_rules! set_field {
        ($name:literal, $field:ident, $expected_kind:ident, $message:literal,) => {
            if let Some(value) = data.remove($name) {
                match value {
                    TableOrValue::Value(Value::$expected_kind(value)) => {
                        config.$field = value.into();
                    }
                    _ => {
                        return Err(Error::unrecoverable(
                            $message.to_string(),
                            source.unwrap_or_else(|| tokens.eof().source().clone()),
                        ));
                    }
                }
            }
        };
    }

    set_field!(
        "fallback_escaper_group",
        fallback_escaper_group,
        String,
        "Boolean value not allowed for `fallback_escaper_group`",
    );
    set_field!(
        "require_specifying_escaper",
        require_specifying_escaper,
        Bool,
        "String value not allowed for `require_specifying_escaper`",
    );
    set_field!(
        "infer_escaper_group_from_file_extension",
        infer_escaper_group_from_file_extension,
        Bool,
        "String value not allowed for `infer_escaper_group_from_file_extension`",
    );
    set_field!(
        "optimized_renderer",
        optimized_renderer,
        Bool,
        "String value not allowed for `optimized_renderer`",
    );

    if let Some(escaper_groups) = data.remove("escaper_groups") {
        parse_escaper_groups(escaper_groups, &tokens, &mut config)?;
    }

    if !data.is_empty() {
        let mut errors = vec![];
        let mut keys = vec![];

        check_table_for_invalid_keys(data, &mut keys, &mut errors);

        if !errors.is_empty() {
            return Err(Error::Multiple(errors));
        }
    }

    Ok((tokens, config))
}

fn check_table_for_invalid_keys<'a>(
    table: Table<'a>,
    keys: &mut Vec<&'a str>,
    errors: &mut Vec<Error<'a>>,
) {
    for (key, value) in table {
        keys.push(key);
        match value {
            TableOrValue::Table(table) => check_table_for_invalid_keys(table, keys, errors),
            TableOrValue::Value(value) => errors.push(Error::unrecoverable(
                format!("Invalid key: `{}`", keys.join(".")),
                value.source().clone(),
            )),
        }
        keys.pop();
    }
}

fn parse_escaper_groups<'a>(
    escaper_groups: TableOrValue<'a>,
    tokens: &TokenSlice<'a>,
    config: &mut Config,
) -> Result<(), Error<'a>> {
    let TableOrValue::Table(escaper_groups) = escaper_groups else {
        return Err(Error::unrecoverable(
            "Expected `escaper_groups` to be a table, found a value".to_string(),
            tokens.eof().source().clone(),
        ));
    };

    for (escaper_group, value) in escaper_groups {
        if !escaper_group.starts_with(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '_'))
            || !escaper_group
                .chars()
                .all(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
        {
            return Err(Error::unrecoverable(
                format!("Escaper group `{escaper_group}` must match `[a-zA-Z_][a-zA-Z0-9_]*`"),
                tokens.eof().source().clone(),
            ));
        }

        let TableOrValue::Table(mut value) = value else {
            return Err(Error::unrecoverable(
                format!("Expected escaper group `{escaper_group}` to be a table, found a value"),
                tokens.eof().source().clone(),
            ));
        };

        if let Some(escaper) = value.remove("escaper") {
            let path = match escaper {
                TableOrValue::Table(_) => {
                    return Err(Error::unrecoverable(
                        format!(
                            "`escaper_groups.{escaper_group}.escaper` should be a path to an \
                             escaper enum (e.g., `::your_package::Escaper`; table found"
                        ),
                        tokens.eof().source().clone(),
                    ));
                }
                TableOrValue::Value(Value::Bool(value)) => {
                    return Err(Error::unrecoverable(
                        format!(
                            "`escaper_groups.{escaper_group}.escaper` should be a path to an \
                             escaper enum (e.g., `::your_package::Escaper`; `{}` found",
                            value.value
                        ),
                        tokens.eof().source().clone(),
                    ));
                }
                TableOrValue::Value(Value::String(ref value)) => value.value,
            };
            if !path.starts_with("::") {
                return Err(Error::unrecoverable(
                    format!("Expected path `{path}` to start with `::`"),
                    tokens.eof().source().clone(),
                ));
            }
            let mut split_path = path.split("::");
            split_path.next();
            if !split_path.all(|ident| {
                ident.starts_with(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '_'))
                    && ident
                        .chars()
                        .all(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
            }) {
                return Err(Error::unrecoverable(
                    format!(
                        "Expected path `{path}` to be a valid path (`(::[a-zA-Z_][a-zA-Z0-9_]*)+`)"
                    ),
                    tokens.eof().source().clone(),
                ));
            }

            let path = EscaperGroup {
                escaper: path.to_owned(),
            };

            config
                .escaper_groups
                .insert(escaper_group.to_string(), path);
        }
    }

    Ok(())
}

macro_rules! handle_expression {
    ($lookup_table:ident, $expression:ident) => {{
        let mut lookup_table: &mut Table = &mut $lookup_table;

        let mut ancestor_keys: Vec<&str> = Vec::with_capacity($expression.ancestor_keys.len());
        for ancestor_key in $expression.ancestor_keys {
            ancestor_keys.push(ancestor_key.value);
        }

        // Make sure a table is created for each key
        for ancestor_key in &ancestor_keys {
            let Some(new_table) = lookup_table
                .entry(ancestor_key)
                .or_insert_with(|| TableOrValue::Table(Table::with_capacity(1)))
                .table_mut()
            else {
                let ancestor_key = *ancestor_key;
                let mut keys = ancestor_keys;
                keys.extend([$expression.key.source.as_str()]);

                return Err(Error::unrecoverable(
                    format!(
                        "`{ancestor_key}` in `{}` is already set to a value and cannot be made \
                         into a table",
                        keys.join(".")
                    ),
                    $expression.source,
                ));
            };

            lookup_table = new_table;
        }

        if lookup_table.contains_key($expression.key.value) {
            let mut keys = ancestor_keys;
            keys.extend([$expression.key.source.as_str()]);

            return Err(Error::unrecoverable(
                format!("`{}` is already set", keys.join("."),),
                $expression.key.source.clone(),
            ));
        }

        lookup_table
    }};
}

fn parse_data(items: Vec<Item>) -> Result<(Table, Option<Source>), Error> {
    let mut data: Table = Table::new();
    let mut current_table_parent: Option<&mut Table> = None;
    let mut current_table_key: &str = "";
    let mut current_table: Table = Table::new();

    let mut source: Option<Source> = None;
    for item in items {
        source = Some(
            item.source()
                .append_to_some(source, "Item expected after previous item"),
        );

        match item {
            Item::Comment(_source) | Item::Newline(_source) => (),
            Item::Expression(expression @ Expression { value: None, .. }) => {
                // New table
                let table = mem::take(&mut current_table);
                if let Some(table_parent) = current_table_parent {
                    table_parent.insert(current_table_key, TableOrValue::Table(table));
                } else {
                    data = table;
                }

                let lookup_table = handle_expression!(data, expression);

                // Table definition, set defined table to the current one
                current_table_parent = Some(lookup_table);
                current_table_key = expression.key.value;
            }
            Item::Expression(expression @ Expression { value: Some(_), .. }) => {
                let lookup_table = handle_expression!(current_table, expression);

                // Table definition, set defined table to the current one
                lookup_table.insert(
                    expression.key.value,
                    TableOrValue::Value(
                        expression
                            .value
                            .expect("Value was already checked to exist"),
                    ),
                );
            }
        }
    }

    if let Some(table_parent) = current_table_parent {
        table_parent.insert(current_table_key, TableOrValue::Table(current_table));
        Ok((data, source))
    } else {
        Ok((current_table, source))
    }
}

enum Item<'a> {
    Comment(Source<'a>),
    Expression(Expression<'a>),
    Newline(Source<'a>),
}

impl<'a> Item<'a> {
    fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((Self::expression, Self::table, Self::comment, Self::newline)).parse(tokens)
    }

    fn source(&self) -> &Source<'a> {
        match self {
            Self::Comment(source) | Self::Newline(source) => source,
            Self::Expression(expression) => &expression.source,
        }
    }

    fn table(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
        let (tokens, (open_bracket, ancestors, key, close_bracket, newline)): (
            _,
            (_, Vec<(Key, _)>, Key, _, _),
        ) = (
            take(TokenKind::BracketOpen),
            many0((into(string), take(TokenKind::DotSeparator))),
            into(string),
            take(TokenKind::BracketClose),
            cut(
                "Newline or end of file expected after expression",
                newline_or_eof,
            ),
        )
            .parse(tokens)?;

        let mut ancestor_keys = Vec::with_capacity(ancestors.len());
        let mut ancestor_source = None;
        for (ancestor, dot) in ancestors {
            ancestor_source = Some(
                ancestor
                    .source()
                    .append_to_some(
                        ancestor_source,
                        "Ancestor expected after previous ancestory",
                    )
                    .merge(dot.source(), "`.` expected after ancestor"),
            );

            ancestor_keys.push(ancestor);
        }

        let source = open_bracket
            .source()
            .clone()
            .merge_some(ancestor_source.as_ref(), "Ancestors expected after `[`")
            .merge(key.source(), "Key expected after ancestors")
            .merge(close_bracket.source(), "`]` expected after key")
            .merge(newline, "Newline or end of file expected after comment");

        let expression = Expression {
            ancestor_keys,
            key,
            value: None,
            source,
        };

        Ok((tokens, Self::Expression(expression)))
    }

    fn expression(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
        let (tokens, (ancestors, key, equal, value, comment, newline)): (
            _,
            (Vec<(Key, _)>, Key, _, _, _, _),
        ) = (
            many0((into(string), take(TokenKind::DotSeparator))),
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

        let mut ancestor_keys = Vec::with_capacity(ancestors.len());
        let mut ancestor_source = None;
        for (ancestor, dot) in ancestors {
            ancestor_source = Some(
                ancestor
                    .source()
                    .append_to_some(
                        ancestor_source,
                        "Ancestor expected after previous ancestory",
                    )
                    .merge(dot.source(), "`.` expected after ancestor"),
            );

            ancestor_keys.push(ancestor);
        }

        let source = key
            .source()
            .clone()
            .append_to_some(ancestor_source, "Key expected after ancestors")
            .merge(equal.source(), "`=` expected after key`")
            .merge(value.source(), "Value expected after `=`")
            .merge_some(comment.map(Token::source), "Comment expected after value")
            .merge(newline, "Newline or end of file expected after comment");

        let expression = Expression {
            ancestor_keys,
            key,
            value: Some(value),
            source,
        };

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

fn string(tokens: TokenSlice) -> Res<StringValue> {
    let (tokens, token) = tokens.take()?;

    let value = match token.kind() {
        TokenKind::String(value) => StringValue {
            value,
            source: token.source(),
        },
        _ => {
            return Err(Error::recoverable(
                format!(r"Expected a string, found: {token:#?}"),
                token.source().clone(),
            ));
        }
    };

    Ok((tokens, value))
}

fn value(tokens: TokenSlice) -> Res<Value> {
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
            return Err(Error::recoverable(
                format!("Expected a string or boolean value, found: {token:#?}"),
                token.source().clone(),
            ));
        }
    };

    Ok((tokens, value))
}

fn newline_or_eof(tokens: TokenSlice<'_>) -> Res<'_, &Source<'_>> {
    if tokens.is_empty() {
        let eof = tokens.eof().source();
        return Ok((tokens, eof));
    }

    let (tokens, token) = tokens.take()?;

    match token.kind() {
        TokenKind::Newline => (),
        _ => {
            return Err(Error::recoverable(
                format!(r"Expected a newline (`\n` or `\r\n`), found: {token:#?}"),
                token.source().clone(),
            ));
        }
    }

    Ok((tokens, token.source()))
}

struct Expression<'a> {
    ancestor_keys: Vec<Key<'a>>,
    key: Key<'a>,
    value: Option<Value<'a>>,
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
