//! Would `#[derive(Debug)]` on the declarations that lack it be worth anything?
//!
//! Twenty-five public types across the three crates carry no derive. The obvious
//! act is to add one. This asks what that would buy, before proposing it.
//!
//! The shapes are reproduced here rather than imported, because the question is
//! about what `derive` emits for a shape and not about arvo: `Integer<BITS>` is
//! a unit struct with a const parameter, and so are eleven of its siblings.

#![no_std]
extern crate alloc;

/// The shape of `arvo_format::points::Integer<BITS>` and eleven siblings: a unit
/// struct carrying its whole content in a const parameter.
#[derive(Debug)]
pub struct AUnitStructWithAConstParameter<const BITS: u32>;

/// The shape of `arvo_format::overflow::Wrap` and twelve siblings: a plain
/// marker whose whole identity is its name.
#[derive(Debug)]
pub struct APlainMarker;

#[derive(Debug)]
pub struct AnotherPlainMarker;

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    /// The finding. `derive` renders the name and drops the const, so two
    /// declarations that are different formats render identically. A rendering
    /// that cannot tell them apart carries no information about the thing it
    /// renders, which is the shape the test gate says to delete rather than
    /// improve.
    #[test]
    fn derive_on_a_const_generic_unit_struct_cannot_discriminate() {
        let narrow = format!("{:?}", AUnitStructWithAConstParameter::<8>);
        let wide = format!("{:?}", AUnitStructWithAConstParameter::<32>);
        assert_eq!(narrow, "AUnitStructWithAConstParameter");
        assert_eq!(
            narrow, wide,
            "derive drops the const, so every width renders the same"
        );
    }

    /// The control, and it is what makes the finding a fact about const
    /// parameters rather than about `derive`. On a plain marker the same derive
    /// does discriminate, because the name is the whole identity.
    #[test]
    fn the_control_derive_on_a_plain_marker_does_discriminate() {
        assert_ne!(
            format!("{:?}", APlainMarker),
            format!("{:?}", AnotherPlainMarker)
        );
    }
}
