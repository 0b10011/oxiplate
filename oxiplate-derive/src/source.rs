use std::fmt;
use std::iter::{Enumerate, Peekable};
use std::ops::Range;
use std::path::PathBuf;
use std::str::{CharIndices, Chars};

use nom::{Compare, Input, Needed, Offset};
use proc_macro2::{Literal, Span};
use syn::Type;

type CharIterator<'a> = Peekable<Enumerate<Chars<'a>>>;

/// Source of a single template.
/// Does not contain the source of parent/children templates.
pub(crate) struct SourceOwned {
    /// Type of data passed to extended templates.
    pub(crate) data_type: Type,

    /// List of names of all blocks in this template and childen templates.
    pub(crate) blocks: Vec<String>,

    /// The template code.
    pub(crate) code: String,

    /// The template code's literal.
    pub(crate) literal: Literal,

    /// The template code's span.
    pub(crate) span_hygiene: Span,

    /// The file path for external templates.
    pub(crate) origin: Option<PathBuf>,

    /// Whether this is extending another template.
    pub(crate) is_extending: bool,
}

impl fmt::Debug for SourceOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceOwned")
            // .field("data_type", &"UNSUPPORTED_SORRY")
            .field("blocks", &self.blocks)
            .field("code", &self.code)
            .field("literal", &self.literal)
            .field("span_hygiene", &self.span_hygiene)
            .field("origin", &self.origin)
            .field("is_extending", &self.is_extending)
            .finish_non_exhaustive()
    }
}

/// A clonable range within a template.
#[derive(Clone, Debug)]
pub(crate) struct Source<'a> {
    pub(crate) original: &'a SourceOwned,
    pub(crate) range: Range<usize>,
}

impl<'a> Source<'a> {
    pub fn as_str(&self) -> &'a str {
        &self.original.code[self.range.clone()]
    }

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

        let hash_count = Self::parse_open(&mut chars, &mut range);
        Self::parse_interior(&mut chars, &mut range, hash_count);

        self.original
            .literal
            .subspan(range)
            .unwrap_or_else(proc_macro2::Span::call_site)
            .resolved_at(self.original.span_hygiene)
    }

    pub fn merge(self, source_to_merge: &Source) -> Self {
        if self.range.end != source_to_merge.range.start {
            panic!("Expected end of own range to match start of next range");
        }

        let mut range = self.range;
        range.end = source_to_merge.range.end;

        Source {
            original: self.original,
            range,
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

    fn parse_open(chars: &mut CharIterator<'_>, range: &mut Range<usize>) -> Option<usize> {
        let (pos, char) = chars.next().expect("Unexpected end of string");
        match char {
            'r' => (),
            '"' => {
                Self::update_range(range, pos);
                return None;
            }
            _ => panic!("Expected 'r' or '\"', found: {char}"),
        }

        Self::update_range(range, pos);

        let mut hash_count = 0;
        for (pos, char) in chars.by_ref() {
            match char {
                '#' => hash_count += 1,
                '"' => {
                    Self::update_range(range, pos);
                    break;
                }
                _ => panic!("Expected '#' or '\"'; found: {char}"),
            }
            Self::update_range(range, pos);
        }

        Some(hash_count)
    }

    fn parse_7_bit_character_code(chars: &mut CharIterator<'_>, range: &mut Range<usize>) {
        // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
        // Up to 0x7F
        match chars.next().expect("Unexpected end of string") {
            (pos, '0'..='7') => Self::update_range(range, pos),
            (_pos, char) => panic!("Expected [0-7]; found: {char}"),
        }
        match chars.next().expect("Unexpected end of string") {
            (pos, '0'..='9' | 'a'..='f' | 'A'..='F') => Self::update_range(range, pos),
            (_pos, char) => panic!("Expected [0-9a-f]; found: {char}"),
        }
    }

    fn parse_unicode_escape(chars: &mut CharIterator<'_>, range: &mut Range<usize>) {
        let mut unicode_chars_parsed = -1;
        let mut unicode_code = String::new();
        loop {
            let (pos, char) = chars.next().expect("Unexpected end of string");
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
                    let code = u32::from_str_radix(&unicode_code, 16).expect("Should be a u32");
                    let char = char::from_u32(code).expect("Should be a unicode char");
                    let byte_count = char.to_string().len();
                    if range.start >= pos {
                        range.start -= byte_count - 1;
                    }
                    if range.end >= pos {
                        range.end -= byte_count - 1;
                    }
                    return;
                }
                (-1, _) => panic!("Expected {}; found {char}", '{'),
                (0, _) => panic!("Expected a hex character (0-9a-f)]; found {char}"),
                (1..=3, _) => panic!(
                    "Expected a hex character (0-9a-f) or {}]; found {char}",
                    '{'
                ),
                (4, _) => panic!("Expected {}; found {char}", '}'),
                (_, _) => unreachable!(
                    "All possible cases should be covered; found {} with count {}",
                    char, unicode_chars_parsed
                ),
            }
        }
    }

    fn parse_string_continuation(chars: &mut CharIterator, range: &mut Range<usize>) {
        while let Some((_pos, char)) = chars.peek() {
            match char {
                '\u{0009}' | '\u{000A}' | '\u{000D}' | '\u{0020}' => {
                    let (pos, _char) = chars.next().unwrap();
                    Self::update_range(range, pos);
                }
                _ => return,
            }
        }
    }

    fn parse_escape(chars: &mut CharIterator<'_>, range: &mut Range<usize>) {
        let (pos, char) = chars.next().expect("Unexpected end of string");
        match char {
            // https://doc.rust-lang.org/reference/tokens.html#quote-escapes
            // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
            '\'' | '"' | 'n' | 'r' | 't' | '\\' | '0' => (),
            // https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
            'x' => Self::parse_7_bit_character_code(chars, range),
            // https://doc.rust-lang.org/reference/tokens.html#unicode-escapes
            'u' => Self::parse_unicode_escape(chars, range),
            // https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-continuation-escapes
            '\n' => {
                Self::update_range(range, pos);
                Self::parse_string_continuation(chars, range);
            }
            _ => panic!(r#"Expected ', ", n, r, t, \, 0, x, u, or \n; found: {char}"#),
        }
    }

    fn parse_interior(
        chars: &mut CharIterator<'_>,
        range: &mut Range<usize>,
        hash_count: Option<usize>,
    ) {
        while let Some((pos, char)) = chars.next() {
            match (char, hash_count) {
                ('"', _) => return,
                ('\\', None) => {
                    Self::update_range(range, pos);
                    Self::parse_escape(chars, range);
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

impl PartialEq<char> for Source<'_> {
    fn eq(&self, char: &char) -> bool {
        self.as_str().len() == 1 && char == &self.as_str().chars().next().unwrap()
    }
}

impl<'a> Compare<&Source<'a>> for Source<'a> {
    fn compare(&self, other_source: &Source) -> nom::CompareResult {
        self.as_str().compare(other_source.as_str())
    }

    fn compare_no_case(&self, other_source: &Source) -> nom::CompareResult {
        self.as_str().compare_no_case(other_source.as_str())
    }
}

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

impl<'a> Iterator for Source<'a> {
    type Item = Source<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}
