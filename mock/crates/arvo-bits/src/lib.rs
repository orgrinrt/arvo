//! arvo-bits: L1 bit-level contracts.
//!
//! Four public traits (`HasBitWidth`, `BitAccess`, `BitSequence`,
//! `BitLogic`) give arvo numeric types bit-level introspection and
//! manipulation. Impl'd concretely on `UFixed<I, F, S>`,
//! `IFixed<I, F, S>`, and the L0 `Bits<N, S>` storage primitive;
//! consumers reach the surface through the numeric type itself.
//!
//! Post round 202604500000: the `Bits` storage primitive, bit-size
//! aliases, `Bitfield` macro, and `BitPrim` / `IBitPrim` sealed
//! bridges all live in arvo L0. arvo-bits hosts only the trait
//! contracts and their blanket impls.
//!
//! `#![no_std]`, no alloc, no platform dep. L1 of the arvo stack;
//! depends only on `arvo`.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod bits_impl;
pub mod ifixed_impl;
pub mod traits;
pub mod ufixed_impl;

pub use traits::{BitAccess, BitLogic, BitSequence, HasBitWidth};

// Consumer convenience: re-export L0 names that appear with arvo-bits
// trait bounds.
pub use arvo::strategy::{Hot, Strategy, UContainerFor};
pub use arvo::{
    Bit, BitPrim, Bits, Byte, DWord, IBitContainer, IBitPrim, Nibble, QWord, UBitContainer, Word,
    bitfield,
};
