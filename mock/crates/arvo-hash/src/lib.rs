//! arvo-hash — L2 hash-domain types and algorithm contracts.
//!
//! `ContentHash` aliases `arvo_bits::Bits<28>` for call-site domain
//! naming. Streaming `Hasher<const N: u8>` is canonical; oneshot
//! ergonomics come via the blanket-impl `HasherExt<N>`. Concrete
//! algorithms (FNV, xxHash, CRC32) ship as trait implementors in
//! follow-up rounds.
//!
//! `#![no_std]`, no alloc. Depends on `arvo-bits` only.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod algo;
pub mod aliases;

pub use algo::{fnv1a_64, Hasher, HasherExt};
pub use aliases::ContentHash;
