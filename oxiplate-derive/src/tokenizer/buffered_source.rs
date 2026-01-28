use std::ops::Range;

use super::ParseError;
use crate::tokenizer::Eof;
use crate::{Source, SourceOwned};

/// A clonable range within a template with a buffer for parsing.
#[derive(Clone, Debug)]
pub struct BufferedSource<'a> {
    original: &'a SourceOwned,
    range: Range<usize>,
    buffer_length: usize,
}

impl<'a> BufferedSource<'a> {
    #[must_use]
    pub fn eof(&self) -> Eof<'a> {
        Eof {
            source: Source::new_with_range(
                self.original,
                Range {
                    start: self.range.end,
                    end: self.range.end,
                },
            ),
        }
    }

    #[must_use]
    pub fn peek(&self) -> Option<char> {
        let start = self.range.start + self.buffer_length;
        self.original.code[start..self.range.end].chars().next()
    }

    #[must_use]
    pub fn peek_2(&self) -> Option<[char; 2]> {
        let start = self.range.start + self.buffer_length;
        let mut iterator = self.original.code[start..self.range.end].chars();

        Some([iterator.next()?, iterator.next()?])
    }

    #[must_use]
    pub fn next(&mut self) -> Option<char> {
        let char = self.peek();

        if let Some(char) = char {
            self.buffer_length += char.len_utf8();
        }

        char
    }

    pub fn next_if(&mut self, matcher: fn(char) -> bool) -> bool {
        let Some(char) = self.peek() else {
            return false;
        };

        if matcher(char) {
            let _ = self.next();
            true
        } else {
            false
        }
    }

    pub fn next_if_not(&mut self, matcher: fn(char) -> bool) -> bool {
        let Some(char) = self.peek() else {
            return false;
        };

        if matcher(char) {
            false
        } else {
            let _ = self.next();
            true
        }
    }

    #[must_use]
    pub fn expect(&mut self, matcher: fn(char) -> bool) -> Result<(), ()> {
        if self.next_if(matcher) {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn next_until(&mut self, matcher: fn(char) -> bool) -> bool {
        let mut has_matched = false;
        while self.next_if_not(matcher) {
            has_matched = true;
        }
        has_matched
    }

    pub fn next_while(&mut self, matcher: fn(char) -> bool) -> usize {
        let mut matched_count = 0;
        while self.next_if(matcher) {
            matched_count += 1;
        }

        matched_count
    }

    #[must_use]
    pub fn consume_until(&mut self, matcher: fn(char) -> bool) -> Result<Source<'a>, ParseError> {
        self.next_until(matcher);

        self.consume()
    }

    #[must_use]
    pub fn consume_while(&mut self, matcher: fn(char) -> bool) -> Result<Source<'a>, ParseError> {
        self.next_while(matcher);

        self.consume()
    }

    #[must_use]
    pub fn consume(&mut self) -> Result<Source<'a>, ParseError> {
        if self.buffer_length == 0 {
            return Err(ParseError::new(
                "Internal Oxiplate error. No buffer present when attempting to consume it. Please \
                 open an issue.",
            ));
        }

        // Build an unbuffered `Source` from the buffered source.
        let consumed = Source::new_with_range(
            self.original,
            Range {
                start: self.range.start,
                end: self.range.start + self.buffer_length,
            },
        );

        // Move pointer ahead and reset buffer.
        self.range.start += self.buffer_length;
        self.buffer_length = 0;

        Ok(consumed)
    }
}

impl<'a> From<Source<'a>> for BufferedSource<'a> {
    fn from(value: Source<'a>) -> Self {
        Self {
            original: value.original,
            range: value.range_full(),
            buffer_length: 0,
        }
    }
}
