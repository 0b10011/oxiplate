mod bool;
mod char;
mod number;
mod string;

pub(in crate::template::parser) use self::bool::Bool;
pub(in crate::template::parser) use self::char::Char;
pub(in crate::template::parser) use self::number::{Float, Integer, Number};
pub(in crate::template::parser) use self::string::String;
