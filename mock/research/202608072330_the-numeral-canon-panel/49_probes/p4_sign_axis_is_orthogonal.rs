// Probe 4: does adding the Sign axis (an input the schema must expose
// downstream, per my cold derivation) compose cleanly with the same
// schema, without needing a second trait or a special case?
//
// Compile with:
//   rustc +nightly-2026-05-28 --edition 2024 --crate-type lib \
//     p4_sign_axis_is_orthogonal.rs

#![no_std]
#![allow(dead_code)]

include!("_shared_schema.rs");

// signed, N = 13. one extra sign bit is needed to represent the same
// logical magnitude range signed, so the natural container width shifts
// up by one bit's worth relative to the unsigned case at the same N. this
// impl exists to check the schema *permits* that divergence, not to claim
// this is the only correct container choice for signed-13; that policy
// question is out of scope for this probe.
impl NumeralFacts<Signed, 13> for Warm {
    type Storage = i16;
    type Operand = i16;
    const ALIGN: usize = 2;
    const PACKED: bool = false;
}

impl NumeralFacts<Signed, 13> for Cold {
    type Storage = [u8; 2];
    type Operand = i16;
    const ALIGN: usize = 1;
    const PACKED: bool = true;
}

const _: () = assert!(core::mem::size_of::<<Warm as NumeralFacts<Signed, 13>>::Storage>() == 2);
const _: () = assert!(core::mem::size_of::<<Cold as NumeralFacts<Signed, 13>>::Operand>() == 2);
const _: () = assert!(<Cold as NumeralFacts<Signed, 13>>::PACKED);
const _: () = assert!(!<Warm as NumeralFacts<Signed, 13>>::PACKED);

// the unsigned-13 and signed-13 impls for the same strategy (Warm) coexist
// without conflict: the trait is generic over Sign, so this is ordinary
// trait resolution, not a special case bolted onto the schema.
pub const fn both_widths_present() -> bool {
    core::mem::size_of::<<Warm as NumeralFacts<Unsigned, 13>>::Storage>() > 0
        && core::mem::size_of::<<Warm as NumeralFacts<Signed, 13>>::Storage>() > 0
}
const _: () = assert!(both_widths_present());
