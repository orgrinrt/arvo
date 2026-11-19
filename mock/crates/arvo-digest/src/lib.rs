//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A digest of explicit bytes, at a width the type names.
//!
//! `obligation::a_content_hash` asks for a hash of content that is stable
//! across sessions and machines and whose width the consumer names. A digest
//! here is a function of a byte string and a width and of nothing else: the
//! same bytes give the same `Digest` in every session, on every target, at
//! every pointer width, and in const context as at run time.
//!
//! [`Fnv1a`] and [`Xxh3`] are the algorithms by name. None of it is
//! cryptographic.
//!
//! Input is `arvo_bits::Bytes` everywhere, the door where the host's byte image
//! enters, the result is held in `arvo_bits::Bits`, and nothing here implements
//! or accepts `core::hash::Hash`, which feeds lengths at the host's pointer
//! width.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

mod digest;
mod fnv1a;
mod xxh3;

pub use digest::{Carrier, Digest, Output};
pub use fnv1a::Fnv1a;
pub use xxh3::Xxh3;

// FIXME(the_switched_64_bit_digest_and_its_crossover_bench_in_the_ha): the switched
// 64-bit digest, FNV-1a below a measured length and XXH3 from it on, joins once the
// `digest-crossover` bench in the harness has measured that length.
