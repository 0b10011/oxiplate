use std::iter::{Enumerate, Peekable};
use std::ops::Range;
use std::path::PathBuf;
use std::str::Chars;

use proc_macro2::{Literal, Span};
use syn::LitStr;

use crate::internal_error;

type CharIterator<'a> = Peekable<Enumerate<Chars<'a>>>;

#[cfg(test)]
macro_rules! test_source {
    ($var:ident = $string:literal) => {
        let span = ::proc_macro2::Span::mixed_site();
        let owned = crate::SourceOwned::new(&::syn::LitStr::new($string, span), span, None);
        let $var = crate::Source::new(&owned);
    };
}

#[cfg(test)]
pub(crate) use test_source;

/// Source of a single template.
/// Does not contain the source of parent/children templates.
#[derive(Debug)]
pub(crate) struct SourceOwned {
    /// The template code.
    pub(crate) code: String,

    /// The template code without unescaping applied.
    code_escaped: String,

    /// The template code's literal.
    pub(crate) literal: Literal,

    /// The template code's span.
    pub(crate) span_hygiene: Span,

    /// The file path for external templates.
    pub(crate) origin: Option<PathBuf>,
}

impl SourceOwned {
    pub fn new(code: &LitStr, span: Span, path: Option<PathBuf>) -> Self {
        let literal = code.token();

        Self {
            code: code.value(),
            code_escaped: literal.to_string(),
            literal,
            span_hygiene: span,
            origin: path,
        }
    }
}

/// A clonable range within a template.
#[derive(Clone, Debug)]
pub(crate) struct Source<'a> {
    pub(crate) original: &'a SourceOwned,

    /// Range start for whitespace + token.
    start_full: usize,

    /// Range start for token (excludes whitespace).
    start_token: usize,

    /// Range end for token (with or without whitespace).
    end: usize,
}

macro_rules! bail {
    ($message:expr, $original:ident, $debug_range:ident) => {{
        crate::internal_error!(
            $original
                .literal
                .subspan($debug_range.clone())
                .unwrap_or_else(proc_macro2::Span::call_site)
                .resolved_at($original.span_hygiene)
                .unwrap(),
            $message,
        );
    }};
}

macro_rules! bail_eof {
    ($message:expr, $original:ident, $debug_range:ident) => {{
        #[cfg(feature = "better-internal-errors")]
        {
            $debug_range.start -= 1;
            $debug_range.end -= 1;
        }
        bail!($message, $original, $debug_range);
    }};
}

impl<'a> Source<'a> {
    pub fn new(owned_source: &'a SourceOwned) -> Self {
        Self {
            original: owned_source,
            start_full: 0,
            start_token: 0,
            end: owned_source.code.len(),
        }
    }

    pub fn new_with_range(owned_source: &'a SourceOwned, range: Range<usize>) -> Self {
        if range.end > owned_source.code.len() {
            panic!(
                "Range end {} must be less than or equal to the code length {}",
                range.end,
                owned_source.code.len()
            );
        }

        Self {
            original: owned_source,
            start_full: range.start,
            start_token: range.start,
            end: range.end,
        }
    }

    pub fn range_full(&self) -> Range<usize> {
        Range {
            start: self.start_full,
            end: self.end,
        }
    }

    pub fn range_token(&self) -> Range<usize> {
        Range {
            start: self.start_token,
            end: self.end,
        }
    }

    pub fn as_str(&self) -> &'a str {
        &self.original.code[self.range_token()]
    }

    #[cfg(feature = "better-internal-errors")]
    fn span_full(&self) -> Span {
        self.span(self.start_full)
    }

    #[must_use]
    pub fn span_token(&self) -> Span {
        self.span(self.start_token)
    }

    #[must_use]
    fn span(&self, start: usize) -> Span {
        let end = self.end;

        // Customize the range to map properly to the literal.
        let mut range = Range { start, end };

        // Uses the span from the included file.
        if self.original.origin.is_some() {
            if range.start == range.end && range.start > 1 {
                let last_char_byte_count = self.original.code[0..range.end]
                    .chars()
                    .last()
                    .map_or(0, char::len_utf8);

                range.start -= last_char_byte_count;
            }

            return self
                .original
                .literal
                .subspan(range)
                .unwrap_or_else(proc_macro2::Span::call_site)
                .resolved_at(self.original.span_hygiene);
        }

        Self::fix_range(
            &self.original.code_escaped,
            &mut range,
            #[cfg(feature = "better-internal-errors")]
            self.original,
        );

        if range.start == range.end && range.start > 1 {
            let last_char_byte_count = self.original.code_escaped[0..range.end]
                .chars()
                .last()
                .map_or(0, char::len_utf8);

            range.start -= last_char_byte_count;
        }

        self.original
            .literal
            .subspan(range)
            .unwrap_or_else(proc_macro2::Span::call_site)
            .resolved_at(self.original.span_hygiene)
    }

    #[must_use]
    pub fn with_collapsed_to_start_full(&self) -> Self {
        let mut source = self.clone();

        source.start_token = source.start_full;
        source.end = source.start_full;

        source
    }

    #[must_use]
    pub fn with_collapsed_to_end(&self) -> Self {
        let mut source = self.clone();

        source.start_full = source.end;
        source.start_token = source.end;

        source
    }

    #[must_use]
    pub fn with_start(&self, other: &Self) -> Self {
        let mut source = self.clone();

        source.start_full = other.start_full;
        source.start_token = other.start_token;

        source
    }

    #[must_use]
    pub fn append_to_leading_whitespace(
        &self,
        leading_whitespace: Option<Source<'a>>,
        error_message: &str,
    ) -> Self {
        if let Some(leading_whitespace) = leading_whitespace {
            // Save the start of the token for later
            let token_start = self.start_token;

            // Merge the whitespace and token together
            let mut new_source = leading_whitespace.merge(self, error_message);

            // Set the start of the token to match where the token actually starts
            new_source.start_token = token_start;

            new_source
        } else {
            self.clone()
        }
    }

    #[must_use]
    pub fn append_to_some(
        &self,
        source_to_append_to: Option<Source<'a>>,
        error_message: &str,
    ) -> Self {
        if let Some(source) = source_to_append_to {
            source.merge(self, error_message)
        } else {
            self.clone()
        }
    }

    #[must_use]
    pub fn merge(self, source_to_merge: &Source<'a>, error_message: &str) -> Self {
        if self.end != source_to_merge.start_full {
            internal_error!(
                vec![self.span_full().unwrap(), source_to_merge.span_full().unwrap()],
                format!("Disjointed ranges cannot be merged. Error: {error_message}"),

                // Spans are sometimes overlapping,
                // so having them split into separate messages is helpful sometimes.
                .span_help(self.span_full().unwrap(), "First range here")
                .span_help(source_to_merge.span_full().unwrap(), "Second range here")
            );
        }

        Source {
            original: self.original,
            start_full: self.start_full,
            start_token: self.start_token,
            end: source_to_merge.end,
        }
    }

    #[must_use]
    pub fn merge_some(self, source_to_merge: Option<&Source<'a>>, error_message: &str) -> Self {
        if let Some(source_to_merge) = source_to_merge {
            self.merge(source_to_merge, error_message)
        } else {
            self
        }
    }

    fn update_range(range: &mut Range<usize>, pos: usize) {
        if range.start >= pos {
            range.start += 1;
        }
        if range.end >= pos {
            range.end += 1;
        }
    }

    fn fix_range(
        code_unescaped: &str,
        range: &mut Range<usize>,
        #[cfg(feature = "better-internal-errors")] owned_source: &SourceOwned,
    ) {
        let mut chars: CharIterator = code_unescaped.chars().enumerate().peekable();

        let mut debug_range = range.clone();
        debug_range.start = 0;
        debug_range.end = 1;

        let Some((pos, char)) = chars.next() else {
            bail_eof!(
                r"Failed to parse start of string. Unexpected end of string",
                owned_source,
                debug_range
            )
        };
        match char {
            'r' => (),
            '"' => {
                Self::update_range(range, pos);
                debug_range.start += 1;
                debug_range.end += 1;

                Self::fix_range_for_interior(
                    &mut chars,
                    range,
                    #[cfg(feature = "better-internal-errors")]
                    owned_source,
                    &mut debug_range,
                );

                return;
            }
            _ => bail!(
                r#"Failed to parse start of string. Expected `r` or `"`"#,
                owned_source,
                debug_range
            ),
        }

        Self::update_range(range, pos);

        for (pos, char) in chars.by_ref() {
            match char {
                '#' => (),
                '"' => {
                    Self::update_range(range, pos);
                    break;
                }
                _ => bail!(
                    r#"Failed to parse start of raw string. Expected `#` or `"`"#,
                    owned_source,
                    debug_range
                ),
            }
            Self::update_range(range, pos);
        }
    }

    /// Consume `"` if present. For testing unreachable match arms.
    #[cfg(feature = "unreachable")]
    fn consume_quote(chars: &mut CharIterator<'_>, range: &mut Range<usize>) {
        if let Some((_, '"')) = chars.peek() {
            let (pos, _) = chars.next().unwrap();
            Self::update_range(range, pos);
        }
    }

    fn parse_7_bit_character_code(
        chars: &mut CharIterator<'_>,
        range: &mut Range<usize>,
        #[cfg(feature = "better-internal-errors")] owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        #[cfg(feature = "unreachable")]
        Self::consume_quote(chars, range);

        // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
        // Up to 0x7F
        match chars.next() {
            Some((pos, '0'..='7')) => Self::update_range(range, pos),
            Some(_) => bail!(
                r"Failed to parse 7-bit character code. Expected `[0-7]`",
                owned_source,
                debug_range
            ),
            None => bail_eof!(
                r"Failed to parse 7-bit character code. Unexpected end of string",
                owned_source,
                debug_range
            ),
        }
        debug_range.start += 1;
        debug_range.end += 1;

        #[cfg(feature = "unreachable")]
        Self::consume_quote(chars, range);

        match chars.next() {
            Some((pos, '0'..='9' | 'a'..='f' | 'A'..='F')) => Self::update_range(range, pos),
            Some(_) => bail!(
                r"Failed to parse 7-bit character code. Expected `[0-9a-f]`",
                owned_source,
                debug_range
            ),
            None => bail_eof!(
                r"Failed to parse 7-bit character code. Unexpected end of string",
                owned_source,
                debug_range
            ),
        }
        debug_range.start += 1;
        debug_range.end += 1;
    }

    fn parse_unicode_escape(
        chars: &mut CharIterator<'_>,
        range: &mut Range<usize>,
        #[cfg(feature = "better-internal-errors")] owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        let mut unicode_chars_parsed = -1;
        let mut unicode_code = String::new();
        loop {
            #[cfg(feature = "unreachable")]
            Self::consume_quote(chars, range);

            let Some((pos, char)) = chars.next() else {
                bail_eof!(
                    r"Failed to parse unicode escape. Unexpected end of string",
                    owned_source,
                    debug_range
                )
            };
            Self::update_range(range, pos);
            match (unicode_chars_parsed, char) {
                (-1, '{') => {
                    unicode_chars_parsed += 1;
                }
                (0..=3, '0'..='9' | 'a'..='f' | 'A'..='F') => {
                    unicode_chars_parsed += 1;
                    unicode_code.push(char);
                }
                (1..=4, '}') => {
                    #[cfg(feature = "unreachable")]
                    {
                        unicode_chars_parsed += 1;
                    }

                    let code = match u32::from_str_radix(&unicode_code, 16) {
                        Ok(code) => code,
                        Err(err) => bail!(
                            format!(
                                r"Failed to parse unicode escape. Expected a u32, found `{unicode_code}`. Error: {err}"
                            ),
                            owned_source,
                            debug_range
                        ),
                    };
                    let Some(char) = char::from_u32(code) else {
                        bail!(
                            format!(
                                r"Failed to parse unicode escape. `{unicode_code}` did not map to a char"
                            ),
                            owned_source,
                            debug_range
                        );
                    };
                    let byte_count = char.to_string().len();
                    if range.start >= pos {
                        range.start -= byte_count - 1;
                    }
                    if range.end >= pos {
                        range.end -= byte_count - 1;
                    }
                    debug_range.start += 1;
                    debug_range.end += 1;

                    #[cfg(not(feature = "unreachable"))]
                    return;
                }
                (-1, _) => bail!(
                    r"Failed to parse unicode escape. Expected `{`",
                    owned_source,
                    debug_range
                ),
                (0, _) => bail!(
                    r"Failed to parse unicode escape. Expected `[0-9a-f]`",
                    owned_source,
                    debug_range
                ),
                (1..=3, _) => bail!(
                    r"Failed to parse unicode escape. Expected `[0-9a-f]` or `}`",
                    owned_source,
                    debug_range
                ),
                (4, _) => bail!(
                    r"Failed to parse unicode escape. Expected `}`",
                    owned_source,
                    debug_range
                ),
                (_, _) => bail!(
                    format!(
                        r"Failed to parse unicode escape. All possible cases should be covered. Found {char} with count {unicode_chars_parsed}"
                    ),
                    owned_source,
                    debug_range
                ),
            }
            debug_range.start += 1;
            debug_range.end += 1;
        }
    }

    fn parse_string_continuation(
        chars: &mut CharIterator,
        range: &mut Range<usize>,
        debug_range: &mut Range<usize>,
    ) {
        while let Some((_pos, char)) = chars.peek() {
            debug_range.start += 1;
            debug_range.end += 1;
            match char {
                '\u{0009}' | '\u{000A}' | '\u{000D}' | '\u{0020}' => {
                    let (pos, _char) = chars.next().unwrap();
                    Self::update_range(range, pos);
                }
                _ => return,
            }
        }
    }

    fn parse_escape(
        chars: &mut CharIterator<'_>,
        range: &mut Range<usize>,
        #[cfg(feature = "better-internal-errors")] owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        #[cfg(feature = "unreachable")]
        Self::consume_quote(chars, range);

        let Some((pos, char)) = chars.next() else {
            bail_eof!(
                r"Failed to parse escape. Unexpected end of string",
                owned_source,
                debug_range
            )
        };
        match char {
            // https://doc.rust-lang.org/reference/tokens.html#quote-escapes
            // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
            '\'' | '"' | 'n' | 'r' | 't' | '\\' | '0' => {
                debug_range.start += 1;
                debug_range.end += 1;
            }
            // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
            'x' => {
                debug_range.start += 1;
                debug_range.end += 1;
                Self::parse_7_bit_character_code(
                    chars,
                    range,
                    #[cfg(feature = "better-internal-errors")]
                    owned_source,
                    debug_range,
                );
            }
            // https://doc.rust-lang.org/reference/tokens.html#unicode-escapes
            'u' => {
                debug_range.start += 1;
                debug_range.end += 1;
                Self::parse_unicode_escape(
                    chars,
                    range,
                    #[cfg(feature = "better-internal-errors")]
                    owned_source,
                    debug_range,
                );
            }
            // https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-continuation-escapes
            '\n' => {
                Self::update_range(range, pos);
                debug_range.start += 1;
                debug_range.end += 1;
                Self::parse_string_continuation(chars, range, debug_range);
            }
            _ => bail!(
                r#"Failed to parse escape. Expected ', ", n, r, t, \, 0, x, u, or \n"#,
                owned_source,
                debug_range
            ),
        }
    }

    fn fix_range_for_interior(
        chars: &mut CharIterator<'_>,
        range: &mut Range<usize>,
        #[cfg(feature = "better-internal-errors")] owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        while let Some((pos, char)) = chars.next() {
            debug_range.start += 1;
            debug_range.end += 1;
            match char {
                '"' => return,
                // Escapes are parsed by Rust first,
                // so invalid escape sequences are only reachable
                // if the code is reached without a `\` before them.
                #[cfg(feature = "unreachable")]
                '/' => {
                    Self::update_range(range, pos);
                    Self::parse_escape(
                        chars,
                        range,
                        #[cfg(feature = "better-internal-errors")]
                        owned_source,
                        debug_range,
                    );
                }
                '\\' => {
                    Self::update_range(range, pos);
                    Self::parse_escape(
                        chars,
                        range,
                        #[cfg(feature = "better-internal-errors")]
                        owned_source,
                        debug_range,
                    );
                }
                _ => (),
            }
        }
    }
}

#[cfg(not(feature = "better-internal-errors"))]
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use proc_macro2::{Literal, Span};

    use super::Source;

    #[cfg(not(feature = "better-internal-errors"))]
    #[test]
    #[should_panic = "Disjointed ranges cannot be merged. Error: B does not follow A"]
    fn disjointed_ranges() {
        let literal = Literal::string("hello world");
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                code_escaped: literal.to_string(),
                literal,
                span_hygiene: Span::call_site(),
                origin: None,
            },
            start_full: 0,
            start_token: 0,
            end: 1,
        };
        let literal = Literal::string("hello world");
        let b = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                code_escaped: literal.to_string(),
                literal,
                span_hygiene: Span::call_site(),
                origin: None,
            },
            start_full: 2,
            start_token: 2,
            end: 3,
        };
        let _ = a.merge(&b, "B does not follow A");
    }

    #[cfg(not(feature = "better-internal-errors"))]
    #[test]
    #[should_panic = "Failed to parse start of string. Expected `r` or `"]
    fn non_string_literal() {
        let literal = Literal::usize_unsuffixed(0);
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                code_escaped: literal.to_string(),
                literal,
                span_hygiene: Span::call_site(),
                origin: None,
            },
            start_full: 0,
            start_token: 0,
            end: 1,
        };
        let _ = a.span_token();
    }
}
