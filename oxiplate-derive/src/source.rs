use std::iter::{Enumerate, Peekable};
use std::ops::Range;
use std::path::PathBuf;
use std::str::{CharIndices, Chars};

use nom::{Compare, Input, Needed, Offset};
use proc_macro::Diagnostic;
use proc_macro2::{Literal, Span};

type CharIterator<'a> = Peekable<Enumerate<Chars<'a>>>;

/// Source of a single template.
/// Does not contain the source of parent/children templates.
#[derive(Debug)]
pub(crate) struct SourceOwned {
    /// The template code.
    pub(crate) code: String,

    /// The template code's literal.
    pub(crate) literal: Literal,

    /// The template code's span.
    pub(crate) span_hygiene: Span,

    /// The file path for external templates.
    pub(crate) origin: Option<PathBuf>,
}

/// A clonable range within a template.
#[derive(Clone, Debug)]
pub(crate) struct Source<'a> {
    pub(crate) original: &'a SourceOwned,
    pub(crate) range: Range<usize>,
}

macro_rules! bail {
    ($message:expr, $help:literal, $original:ident, $debug_range:ident) => {{
        let span = $original
            .literal
            .subspan($debug_range.clone())
            .unwrap_or_else(proc_macro2::Span::call_site)
            .resolved_at($original.span_hygiene);
        Diagnostic::spanned(span.unwrap(), proc_macro::Level::Error, $message)
            .help($help)
            .help("Include template that caused the issue.")
            .emit();
        unreachable!("Internal Oxiplate error. See previous error for more information.");
    }};
}

macro_rules! bail_eof {
    ($message:expr, $help:literal, $original:ident, $debug_range:ident) => {{
        $debug_range.start -= 1;
        $debug_range.end -= 1;
        bail!($message, $help, $original, $debug_range);
    }};
}

impl<'a> Source<'a> {
    pub fn as_str(&self) -> &'a str {
        &self.original.code[self.range.clone()]
    }

    #[must_use]
    pub fn span(&self) -> Span {
        let mut start = self.range.start;
        let end = self.range.end;
        if start == end && start > 1 {
            start -= 1;
        }

        // Customize the range to map properly to the literal.
        let mut range = Range { start, end };

        // Uses the span from the included file.
        if self.original.origin.is_some() {
            return self
                .original
                .literal
                .subspan(range)
                .unwrap_or_else(proc_macro2::Span::call_site)
                .resolved_at(self.original.span_hygiene);
        }

        let literal = self.original.literal.to_string();
        let mut chars: CharIterator = literal.chars().enumerate().peekable();

        let mut debug_range = self.range.clone();
        debug_range.start = 0;
        debug_range.end = 1;
        let hash_count = Self::parse_open(&mut chars, &mut range, self.original, &mut debug_range);
        Self::parse_interior(
            &mut chars,
            &mut range,
            hash_count,
            self.original,
            &mut debug_range,
        );

        self.original
            .literal
            .subspan(range)
            .unwrap_or_else(proc_macro2::Span::call_site)
            .resolved_at(self.original.span_hygiene)
    }

    #[must_use]
    pub fn merge(self, source_to_merge: &Source, error_message: &str) -> Self {
        if self.range.end != source_to_merge.range.start {
            Diagnostic::spanned(
                vec![self.span().unwrap(), source_to_merge.span().unwrap()],
                proc_macro::Level::Error,
                "Internal Oxiplate error: Disjointed ranges cannot be merged",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Disjointed+ranges+cannot+be+merged")
            .help("Include template that caused the issue and the associated note.")
            .note(format!("Error: {error_message}"))

            // Spans are sometimes overlapping,
            // so having them split into separate messages is helpful sometimes.
            .span_help(self.span().unwrap(), "First range here")
            .span_help(source_to_merge.span().unwrap(), "Second range here")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }

        let mut range = self.range;
        range.end = source_to_merge.range.end;

        Source {
            original: self.original,
            range,
        }
    }

    #[must_use]
    pub fn merge_some(self, source_to_merge: Option<&Source>, error_message: &str) -> Self {
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

    fn parse_open(
        chars: &mut CharIterator<'_>,
        range: &mut Range<usize>,
        owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) -> Option<usize> {
        let Some((pos, char)) = chars.next() else {
            bail_eof!(
                r"Internal Oxiplate error: Failed to parse start of string. Unexpected end of string.",
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+start+of+string",
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
                return None;
            }
            _ => bail!(
                r#"Internal Oxiplate error: Failed to parse start of string. Expected `r` or `"`."#,
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+start+of+string",
                owned_source,
                debug_range
            ),
        }

        Self::update_range(range, pos);
        debug_range.start += 1;
        debug_range.end += 1;

        let mut hash_count = 0;
        for (pos, char) in chars.by_ref() {
            match char {
                '#' => hash_count += 1,
                '"' => {
                    Self::update_range(range, pos);
                    break;
                }
                _ => bail!(
                    r#"Internal Oxiplate error: Failed to parse start of raw string. Expected `#` or `"`."#,
                    "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+start+of+raw+string",
                    owned_source,
                    debug_range
                ),
            }
            Self::update_range(range, pos);
            debug_range.start += 1;
            debug_range.end += 1;
        }

        Some(hash_count)
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
        owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        #[cfg(feature = "unreachable")]
        Self::consume_quote(chars, range);

        // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
        // Up to 0x7F
        match chars.next() {
            Some((pos, '0'..='7')) => Self::update_range(range, pos),
            Some(_) => bail!(
                r"Internal Oxiplate error: Failed to parse 7-bit character code. Expected `[0-7]`.",
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+7-bit+character+code",
                owned_source,
                debug_range
            ),
            None => bail_eof!(
                r"Internal Oxiplate error: Failed to parse 7-bit character code. Unexpected end of string.",
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+7-bit+character+code",
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
                r"Internal Oxiplate error: Failed to parse 7-bit character code. Expected `[0-9a-f]`.",
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+7-bit+character+code",
                owned_source,
                debug_range
            ),
            None => bail_eof!(
                r"Internal Oxiplate error: Failed to parse 7-bit character code. Unexpected end of string.",
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+7-bit+character+code",
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
        owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        let mut unicode_chars_parsed = -1;
        let mut unicode_code = String::new();
        loop {
            #[cfg(feature = "unreachable")]
            Self::consume_quote(chars, range);

            let Some((pos, char)) = chars.next() else {
                bail_eof!(
                    r"Internal Oxiplate error: Failed to parse unicode escape. Unexpected end of string.",
                    "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
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
                            format!(r"Internal Oxiplate error: Failed to parse unicode escape. Expected a u32, found `{unicode_code}`. Error: {err}"),
                            "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
                            owned_source,
                            debug_range
                        ),
                    };
                    let Some(char) = char::from_u32(code) else {
                        bail!(
                            format!(r"Internal Oxiplate error: Failed to parse unicode escape. `{unicode_code}` did not map to a char."),
                            "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
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
                    r"Internal Oxiplate error: Failed to parse unicode escape. Expected `{`.",
                    "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
                    owned_source,
                    debug_range
                ),
                (0, _) => bail!(
                    r"Internal Oxiplate error: Failed to parse unicode escape. Expected `[0-9a-f]`.",
                    "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
                    owned_source,
                    debug_range
                ),
                (1..=3, _) => bail!(
                    r"Internal Oxiplate error: Failed to parse unicode escape. Expected `[0-9a-f]` or `}`.",
                    "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
                    owned_source,
                    debug_range
                ),
                (4, _) => bail!(
                    r"Internal Oxiplate error: Failed to parse unicode escape. Expected `}`.",
                    "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
                    owned_source,
                    debug_range
                ),
                (_, _) => bail!(
                    format!(r"Internal Oxiplate error: Failed to parse unicode escape. All possible cases should be covered. Found {char} with count {unicode_chars_parsed}."),
                    "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+unicode+escape",
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
        owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        #[cfg(feature = "unreachable")]
        Self::consume_quote(chars, range);

        let Some((pos, char)) = chars.next() else {
            bail_eof!(
                r"Internal Oxiplate error: Failed to parse escape. Unexpected end of string.",
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+escape",
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
                Self::parse_7_bit_character_code(chars, range, owned_source, debug_range);
            }
            // https://doc.rust-lang.org/reference/tokens.html#unicode-escapes
            'u' => {
                debug_range.start += 1;
                debug_range.end += 1;
                Self::parse_unicode_escape(chars, range, owned_source, debug_range);
            }
            // https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-continuation-escapes
            '\n' => {
                Self::update_range(range, pos);
                debug_range.start += 1;
                debug_range.end += 1;
                Self::parse_string_continuation(chars, range, debug_range);
            }
            _ => bail!(
                r#"Internal Oxiplate error: Failed to parse escape. Expected ', ", n, r, t, \, 0, x, u, or \n."#,
                "Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Failed+to+parse+escape",
                owned_source,
                debug_range
            ),
        }
    }

    fn parse_interior(
        chars: &mut CharIterator<'_>,
        range: &mut Range<usize>,
        hash_count: Option<usize>,
        owned_source: &SourceOwned,
        debug_range: &mut Range<usize>,
    ) {
        while let Some((pos, char)) = chars.next() {
            debug_range.start += 1;
            debug_range.end += 1;
            match (char, hash_count) {
                ('"', _) => return,
                // Escapes are parsed by Rust first,
                // so invalid escape sequences are only reachable
                // if the code is reached without a `\` before them.
                #[cfg(feature = "unreachable")]
                ('/', None) => {
                    Self::update_range(range, pos);
                    Self::parse_escape(chars, range, owned_source, debug_range);
                }
                ('\\', None) => {
                    Self::update_range(range, pos);
                    Self::parse_escape(chars, range, owned_source, debug_range);
                }
                _ => (),
            }
        }
    }
}

impl<'a> Input for Source<'a> {
    type Item = char;
    type Iter = Chars<'a>;
    type IterIndices = CharIndices<'a>;

    fn input_len(&self) -> usize {
        self.as_str().input_len()
    }

    fn take(&self, index: usize) -> Self {
        let end = self.range.start + index;
        if end > self.range.end {
            panic!("End greater than end of string");
        }
        Source {
            original: self.original,
            range: Range {
                start: self.range.start,
                end,
            },
        }
    }

    fn take_from(&self, index: usize) -> Self {
        let start = self.range.start + index;
        if start > self.range.end {
            panic!("Start greater than end of string");
        }

        Source {
            original: self.original,
            range: Range {
                start,
                end: self.range.end,
            },
        }
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let split_point = self.range.start + index;
        if split_point > self.range.end {
            panic!("Split point greater than end of string");
        }

        (
            Source {
                original: self.original,
                range: Range {
                    start: split_point,
                    end: self.range.end,
                },
            },
            Source {
                original: self.original,
                range: Range {
                    start: self.range.start,
                    end: split_point,
                },
            },
        )
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.as_str().position(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.as_str().iter_elements()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.as_str().iter_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.as_str().slice_index(count)
    }
}

impl<'a> PartialEq<Source<'a>> for Source<'a> {
    fn eq(&self, other: &Source) -> bool {
        self.range == other.range
            && self.original.origin == other.original.origin
            && self.original.code == other.original.code
    }
}

impl Eq for Source<'_> {}

impl Compare<&str> for Source<'_> {
    fn compare(&self, string: &str) -> nom::CompareResult {
        self.as_str().compare(string)
    }

    fn compare_no_case(&self, string: &str) -> nom::CompareResult {
        self.as_str().compare_no_case(string)
    }
}

impl Offset for Source<'_> {
    fn offset(&self, offset: &Self) -> usize {
        self.as_str().offset(offset.as_str())
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use std::ops::Range;

    use nom::{Compare, CompareResult, Input};
    use proc_macro2::{Literal, Span};

    use super::Source;

    #[test]
    #[should_panic = "proc_macro::Span is only available in procedural macros"]
    fn disjointed_ranges() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 1 },
        };
        let b = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 2, end: 3 },
        };
        let _ = a.merge(&b, "B does not follow A");
    }

    #[test]
    #[should_panic = "proc_macro::Span is only available in procedural macros"]
    fn non_string_literal() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::usize_unsuffixed(0),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 1 },
        };
        let _ = a.span();
    }

    #[test]
    #[should_panic = "End greater than end of string"]
    fn take_too_many() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 1 },
        };
        a.take(5);
    }

    #[test]
    #[should_panic = "Start greater than end of string"]
    fn take_from_too_many() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 1 },
        };
        a.take_from(5);
    }

    #[test]
    #[should_panic = "Split point greater than end of string"]
    fn take_split_too_many() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 1 },
        };
        a.take_split(5);
    }

    #[test]
    fn take_split() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 11 },
        };

        let b = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 5 },
        };
        let c = Source {
            original: &crate::SourceOwned {
                code: "hello world".to_string(),
                literal: Literal::string("hello world"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 5, end: 11 },
        };

        assert_eq!((c, b), a.take_split(5));
    }

    #[test]
    fn iter_indices() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "hi".to_string(),
                literal: Literal::string("hi"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 2 },
        };
        let mut indices = vec![];
        let mut chars = vec![];
        for (index, char) in a.iter_indices() {
            indices.push(index);
            chars.push(char);
        }
        assert_eq!((vec![0, 1], vec!['h', 'i']), (indices, chars));
    }

    #[test]
    fn compare_no_case() {
        let a = Source {
            original: &crate::SourceOwned {
                code: "Hello World".to_string(),
                literal: Literal::string("Hello World"),
                span_hygiene: Span::call_site(),
                origin: None,
            },
            range: Range { start: 0, end: 11 },
        };
        assert_eq!(CompareResult::Ok, a.compare_no_case("hElLo wOrLd"));
        assert_eq!(CompareResult::Error, a.compare_no_case("goodbye world"));
        assert_eq!(CompareResult::Incomplete, a.compare_no_case("hello world!"));
    }
}
