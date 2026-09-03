//! Can arvo supply "debug output from every numeral" at all?
//!
//! `obligation::debug_output_from_every_numeral_shape` asks for output from
//! every numeral. Under `ruling::the_format_spine_is_canon` the concept is
//! closed and the inventory is open, so "every" ranges over implementors of
//! `Format` that this crate does not know about. This asks whether a blanket
//! rendering over that open set is expressible.
//!
//! The arm is in `the_blanket_impl.rs` beside this file, compiled separately,
//! because a coherence refusal aborts the crate it sits in.
//!
//! What is here is the control: the same shape with a local trait in `Debug`'s
//! place, which must compile, or the refusal next door would be a fact about
//! how the arm was built rather than about coherence.

#![no_std]

use arvo_format::Format;

/// A local stand-in for `core::fmt::Debug`, foreign in nothing.
pub trait LocalRender {
    fn render(&self) -> &'static str;
}

/// The control. A blanket impl over `F: Format` for a *local* trait is
/// accepted, so the shape itself is legal and only foreignness can refuse it.
impl<F: Format> LocalRender for F {
    fn render(&self) -> &'static str {
        "a format"
    }
}

#[cfg(test)]
mod tests {
    use arvo_format::points::Integer;

    use super::*;

    /// The control fires: the blanket reaches a point of the inventory.
    #[test]
    fn the_control_a_blanket_over_a_local_trait_is_accepted() {
        assert_eq!(LocalRender::render(&Integer::<32>), "a format");
    }
}
