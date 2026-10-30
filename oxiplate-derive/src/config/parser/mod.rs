mod comment;
mod expression;
mod item;
mod newline;
mod newline_or_eof;
mod string;
mod table;
mod table_or_value;
mod value;

use std::mem;

use self::expression::Expression;
use self::item::Item;
use self::prelude::*;
use self::table::Table;
use self::table_or_value::TableOrValue;
use self::value::Value;
use crate::config::{Config, EscaperGroup, TokenSlice};

type Res<'a, S> = crate::parser::Res<'a, TokenKind, S>;

mod prelude {
    pub(super) use proc_macro2::TokenStream;
    pub(super) use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};

    pub(super) use super::Res;
    pub(super) use crate::config::tokenizer::TokenKind;
    pub(super) use crate::config::{Token, TokenSlice};
    pub(super) use crate::parser::prelude::*;
    pub(super) use crate::{BuiltTokens, Source, State, internal_error};
}

pub(crate) fn parse(tokens: TokenSlice) -> Res<Config> {
    let (tokens, items) = parse_all(Item::parse).parse(tokens)?;

    let (mut data, config_source) = parse_data(items)?;

    let config_source = config_source.unwrap_or_else(|| tokens.eof().source().clone());

    let mut config = Config::default();

    macro_rules! set_field {
        ($name:literal, $field:ident, $expected_kind:ident, $message:literal,) => {
            if let Some(value) = data.remove($name) {
                match value {
                    TableOrValue::Value(Value::$expected_kind(value)) => {
                        config.$field = value.into();
                    }
                    TableOrValue::Value(value) => {
                        return Err(Error::unrecoverable(
                            $message.to_string(),
                            value.source().clone(),
                        ));
                    }
                    TableOrValue::Table(_) => {
                        return Err(Error::unrecoverable($message.to_string(), config_source));
                    }
                }
            }
        };
    }

    set_field!(
        "fallback_escaper_group",
        fallback_escaper_group,
        String,
        "Expected `fallback_escaper_group` to be a string",
    );
    set_field!(
        "require_specifying_escaper",
        require_specifying_escaper,
        Bool,
        "Expected `require_specifying_escaper` to be `true` or `false`",
    );
    set_field!(
        "infer_escaper_group_from_file_extension",
        infer_escaper_group_from_file_extension,
        Bool,
        "Expected `infer_escaper_group_from_file_extension` to be `true` or `false`",
    );
    set_field!(
        "optimized_renderer",
        optimized_renderer,
        Bool,
        "Expected `optimized_renderer` to be `true` or `false`",
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
                             escaper enum (e.g., `::your_package::Escaper`); table found"
                        ),
                        tokens.eof().source().clone(),
                    ));
                }
                TableOrValue::Value(Value::Bool(value)) => {
                    return Err(Error::unrecoverable(
                        format!(
                            "`escaper_groups.{escaper_group}.escaper` should be a path to an \
                             escaper enum (e.g., `::your_package::Escaper`); `{:?}` found",
                            value
                        ),
                        tokens.eof().source().clone(),
                    ));
                }
                TableOrValue::Value(Value::String(ref value)) => value.value(),
            };

            #[cfg(not(feature = "_unreachable"))]
            {
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
                            "Expected path `{path}` to be a valid path \
                             (`(::[a-zA-Z_][a-zA-Z0-9_]*)+`)"
                        ),
                        tokens.eof().source().clone(),
                    ));
                }
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

        let mut ancestor_keys: Vec<&str> = Vec::with_capacity($expression.ancestor_keys().len());
        for ancestor_key in $expression.ancestor_keys() {
            ancestor_keys.push(ancestor_key.value());
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
                keys.extend([$expression.key().source().as_str()]);

                return Err(Error::unrecoverable(
                    format!(
                        "`{ancestor_key}` in `{}` is already set to a value and cannot be made \
                         into a table",
                        keys.join(".")
                    ),
                    $expression.source().to_owned(),
                ));
            };

            lookup_table = new_table;
        }

        if lookup_table.contains_key($expression.key().value()) {
            let mut keys = ancestor_keys;
            keys.extend([$expression.key().source().as_str()]);

            return Err(Error::unrecoverable(
                format!("`{}` is already set", keys.join("."),),
                $expression.key().source().clone(),
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
            Item::Expression(expression) if expression.value().is_none() => {
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
                current_table_key = expression.key().value();
            }
            Item::Expression(expression) => {
                let lookup_table = handle_expression!(current_table, expression);

                // Table definition, set defined table to the current one
                lookup_table.insert(
                    expression.key().value(),
                    TableOrValue::Value(
                        expression
                            .value()
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
