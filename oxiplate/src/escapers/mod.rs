//! Built-in escapers.
//!
//! # Build your own escaper
//!
//! To build your own escaper,
//! make an enum with variants matching the escaper names you'd like to use,
//! and have it implement [`oxiplate_traits::Escaper`].
//! Oxiplate templates usually use `snake_case` escaper names,
//! so it's suggested to use those on your escaper group enum.
//!
//! [`your_group`] contains a full example
//! of implementing your own escaper.
//! Consider copying that
//! and replacing `YourEscaper` and `YourGroup`
//! with the names of your escaper and group.
//!
//! ## TOML config
//!
//! ```toml
//! [escaper_groups.your_group]
//! escaper = "::your_crate::YourEscaper"
//! ```
//!
//! ## Rust code
//!
//! ```
//! use oxiplate::{Oxiplate, Render};
//!
//! #[derive(Oxiplate)]
//! // Because the TOML isn't included in this,
//! // `your_group` isn't available and the example doesn't compile.
//! #[oxiplate_inline(your_group: "{{ foo: value_to_escape }} | {{ bar: value_to_escape }}")]
//! struct Data<'a> {
//!     value_to_escape: &'a str,
//! }
//!
//! let data = Data {
//!     value_to_escape: "foo bar",
//! };
//!
//! assert_eq!(data.render()?, r#"f00 bar | foo b@r"#);
//!
//! # Ok::<(), ::core::fmt::Error>(())
//! ```

pub mod html;
pub mod json;
pub mod markdown;
pub mod your_group;
