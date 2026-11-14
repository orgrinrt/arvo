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
//! [`Fnv1a`] and [`Xxh3`] are the algorithms by name. `Digest::<64>::of` is
//! the digest to reach for when no existing format has to be matched: FNV-1a
//! on input shorter than a measured length and XXH3 from it on. None of it is
//! cryptographic.
//!
//! Input is `&[u8]` everywhere, the one door where the host's byte image
//! enters, and nothing here implements or accepts `core::hash::Hash`, which
//! feeds lengths at the host's pointer width.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

mod digest;
mod fnv1a;
mod switched;
mod xxh3;

pub use digest::{Carrier, Digest, Width};
pub use fnv1a::Fnv1a;
pub use xxh3::Xxh3;
