use core::iter::Peekable;

/// Groups an iterator with a `Loop` struct
/// containing additional loop iteration info.
/// Useful in `for` loops.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"
/// {% for (loop, value) in &values | loop -%}
///     {% if loop.is_first -%}
///         First:
///     {%_ endif -%}
///     
///     {{ value _}}
///     (#{{ loop.index1 }})
/// {% endfor %}"#)]
/// struct Data {
///     values: Vec<usize>,
/// }
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(
///         Data {
///             values: vec![19, 89]
///         }
///         .render()?,
///         r"
/// First: 19 (#1)
/// 89 (#2)
/// "
///     );
///     Ok(())
/// }
/// ```
pub fn r#loop<E: IntoIterator>(expression: E) -> impl Iterator<Item = (Loop, E::Item)> {
    let iterator = IntoIterator::into_iter(expression);
    LoopIter::new(iterator)
}

/// Loop iteration info.
pub struct Loop {
    /// Iteration number starting from 0.
    pub index0: usize,

    /// Iteration number starting from 1.
    pub index1: usize,

    /// Whether this iteration is the first.
    pub is_first: bool,

    /// Whether this iteration is the last.
    pub is_last: bool,
}

/// Iterator for `Loop` that is peekable
/// and tracks the index of the current iteration.
struct LoopIter<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
    index1: usize,
}

impl<I> LoopIter<I>
where
    I: Iterator,
{
    /// Create a new instance of `LoopIter`.
    fn new(iter: I) -> LoopIter<I> {
        LoopIter {
            iter: iter.peekable(),
            index1: 0,
        }
    }

    /// Returns a reference to the `next()` value
    /// without advancing the iterator.
    fn peek(&mut self) -> Option<&I::Item> {
        self.iter.peek()
    }
}

impl<I> Iterator for LoopIter<I>
where
    I: Iterator,
{
    type Item = (Loop, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.index1 += 1;

        let item = self.iter.next()?;

        Some((
            Loop {
                index0: self.index1 - 1,
                index1: self.index1,
                is_first: self.index1 == 1,
                is_last: self.peek().is_none(),
            },
            item,
        ))
    }
}
