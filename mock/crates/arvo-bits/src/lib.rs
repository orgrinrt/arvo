//! arvo-bits: L1 bit-level contracts.
//!
//! Three public traits (`BitWidth`, `BitAccess`, `BitSequence`) give
//! arvo numeric types bit-level introspection and manipulation.
//! Impl'd concretely on `UFixed<I, F, S>` and `IFixed<I, F, S>`;
//! consumers reach the surface through the numeric type itself.
//!
//! The sealed `BitPrim` / `IBitPrim` helper traits bridge the concrete
//! impls down to Rust's fixed-width integer primitives. They are
//! public (so the `impl` where-clauses can name them) but cannot be
//! implemented downstream: the supertrait is private.
//!
//! Semantic aliases: `Bit`, `Nibble`, `Byte`, `Word`, `DWord`, `QWord`.
//! All default to the `Hot` strategy, because bit work wants minimum
//! container and wrapping ops.
//!
//! `#![no_std]`, no alloc, no platform dep. L1 of the arvo stack;
//! depends only on `arvo`.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]

pub mod alias;
pub mod bitfield;
pub mod bits;
pub mod ifixed_impl;
pub mod prim;
pub mod traits;
pub mod ufixed_impl;

pub use alias::{Bit, Byte, DWord, Nibble, QWord, Word};
pub use bits::Bits;
pub use prim::{BitPrim, IBitContainer, IBitPrim, UBitContainer};
pub use traits::{BitAccess, BitLogic, BitSequence, BitWidth};

// Consumer convenience: re-export the strategy / container-dispatch
// names that appear in `Bits<N, S>` where-clauses. Downstream crates
// (arvo-hash, hilavitkutin-str) can write the bound
// `Hot: UContainerFor<N>` without pulling arvo directly just for
// the names.
pub use arvo::strategy::{Hot, Strategy, UContainerFor};
