#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

pub use oxiplate_derive::Oxiplate;

pub mod escapers;
