//! Positive control for `the_blanket_impl.rs`, on the same rustc invocation.
//!
//! Identical in shape, with a local trait in `core::fmt::Debug`'s place. It must
//! compile, or the `E0210` next door is a fact about how rustc was called rather
//! than about implementing a foreign trait over an open inventory.

#![no_std]

use arvo_format::Format;

pub trait LocalRender {
    fn render(&self) -> &'static str;
}

impl<F: Format> LocalRender for F {
    fn render(&self) -> &'static str {
        "a format"
    }
}
