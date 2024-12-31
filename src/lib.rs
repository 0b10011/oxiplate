#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
// Clippy groups
#![warn(clippy::cargo, clippy::pedantic)]
// Clippy rules
#![allow(
    // rustfmt doesn't format `assert!()` :(
    clippy::manual_assert,
)]

pub use oxiplate_derive::Oxiplate;

pub mod escapers;
