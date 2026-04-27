//! arvo-hash — L2 hash-domain types and algorithm contracts.
//!
//! `ContentHash` aliases `arvo::Bits<28, Hot>` for call-site domain
//! naming. Streaming `Hasher<const N: u8>` is canonical; oneshot
//! ergonomics come via the blanket-impl `HasherExt<N>`. The
//! `Fnv1a<const N: u8>` struct is the first shipped algorithm;
//! xxHash, CRC32, and others land in follow-up rounds.
//!
//! `#![no_std]`, no alloc. Depends on `arvo` (for `Bits` / `Hot` /
//! strategy bounds).

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod algo;
pub mod aliases;
pub mod fnv1a;

pub use algo::{Hasher, HasherExt, fnv1a_64};
pub use aliases::ContentHash;
pub use fnv1a::Fnv1a;
