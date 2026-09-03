// Arm B, the door side. A library that puts one of the stack's own types in a
// const generic parameter position, which is the position
// `obligation::a_primitive_for_every_position_a_bare_number_would_take` excepts
// by name and the position `tests/ui/an_arvo_type_as_a_const_parameter.rs`
// shows the compiler refuses without a feature.
//
// `Width` is reproduced here rather than imported, at the shape `arvo-format`
// ships it: `repr(transparent)` over a `u32`, private field, public struct.
// The private field is what the door ruling's promotion records as the reason
// `min_adt_const_params` refuses it while `adt_const_params` accepts it, so the
// shape matters and a public-field imitation would measure a different type.
#![no_std]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, core::marker::ConstParamTy)]
pub struct Width(u32);

impl Width {
    pub const fn bits(n: u32) -> Self {
        Self(n)
    }

    pub const fn count(self) -> u32 {
        self.0
    }
}

/// The declaration carrying the stack's own type at a const generic parameter.
pub struct Signed<const BITS: Width>;

impl<const BITS: Width> Signed<BITS> {
    pub const DECLARED: u32 = BITS.count();
}

/// The control item: same crate, same feature, no ADT const parameter anywhere
/// in the signature.
pub const fn plain(w: Width) -> u32 {
    w.count()
}
