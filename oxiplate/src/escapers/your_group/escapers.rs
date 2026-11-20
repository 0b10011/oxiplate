use core::fmt::{Result, Write};

#[inline]
pub fn bar_escaper<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains("bar") {
        return f.write_str(value);
    }

    f.write_str(&value.replace("bar", "b@r"))
}

#[inline]
pub fn escape_foo<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains("foo") {
        return f.write_str(value);
    }

    f.write_str(&value.replace("foo", "f00"))
}