use super::prelude::*;
use super::table::Table;
use super::value::Value;

#[derive(Debug)]
pub(in crate::config::parser) enum TableOrValue<'a> {
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
