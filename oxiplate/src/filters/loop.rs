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
    iterator.into_iter().peekable().scan(0, |index1, item| {
        *index1 += 1;
        Some((
            Loop {
                index0: *index1 - 1,
                index1: *index1,
                is_first: *index1 == 1,
            },
            item,
        ))
    })
}

/// Loop iteration info.
pub struct Loop {
    /// Iteration number starting from 0.
    pub index0: usize,

    /// Iteration number starting from 1.
    pub index1: usize,

    /// Whether this iteration is the first.
    pub is_first: bool,
}
