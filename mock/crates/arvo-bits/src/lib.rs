//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A value at an exact declared width, and nothing else.
//!
//! `obligation::an_exact_width_container_a_consumer_can_alias_and_pin` asks for
//! a container a consumer can name at their own boundary, where the width is
//! fixed by something arvo does not own: a disk format, a wire format, a hash
//! function's output size. `Bits<const N: u32>` is that container. It is not a
//! numeral: there is no sign, no ambient domain, no quantum, no rounding, no
//! arithmetic operation anywhere in this crate. It is the mechanical fact of a
//! value known to be exactly `N` bits wide, constructed from a wider host value
//! by masking and cast to a different declared width by masking again.
//!
//! This crate introduces the numeric category a second time, the way
//! `arvo_format` does for its own reasons: the crate that defines a value at an
//! exact width cannot express itself in types that do not exist yet, so `u64`
//! appears here as the one door this crate owns. Every crate above this one
//! uses `Bits` and is checked normally.
//!
//! It depends on nothing in this workspace. `arvo_format`'s own design states
//! plainly that a format is not a container and holds no machine carrier;
//! `arvo_placement`'s own design derives placement metadata and is checked
//! against ever holding a host primitive itself. Neither crate can hold a
//! runtime value at a declared width, by its own stated charter, and this
//! crate is that hole, closed narrowly.
//!
//! `ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` carries
//! op's words that arvo may ship casting and conversion helpers and may not use
//! them in place of the consumer. Nothing in this crate calls `masked` or
//! `cast` on a consumer's behalf, and neither can fail: construction masks
//! rather than rejects, so there is no validating entry point for any reading
//! of that ruling's still-open question to disagree with.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

/// A value known to be exactly `N` bits wide, for `N` from one to sixty-four.
///
/// `repr(transparent)` over a `u64`. Construction masks whatever is handed to
/// it down to the low `N` bits rather than rejecting anything: a bit pattern of
/// the wrong width is not an invalid value, it is a value with some bits asked
/// to be dropped. There is no `TryFrom` and no `notko::Boundable` impl here,
/// because both are shaped around rejecting a value, and construction here
/// never rejects.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Bits<const N: u32>(u64);

impl<const N: u32> Bits<N> {
    /// What every constructor and cast forces before doing anything else.
    ///
    /// Checked at compile time, not asked for in a comment: the assertion is
    /// evaluated where this constant is used, so a call reaches it at codegen
    /// and `cargo build` refuses a bad `N` where `cargo check` would not have.
    /// The same shape `arvo_format::slots::Slots::ADMITTED` uses, narrowed to
    /// a private inherent constant because there is no outside implementor of
    /// anything here: `Bits<N>` is a concrete type, and only its own
    /// constructors force the check.
    ///
    /// The bound is not borrowed from `arvo_format::slots::Slots`, whose own
    /// admitted range stops at sixty-two because a slot index is carried in a
    /// signed 64-bit integer. This bound stops at sixty-four because the value
    /// itself is carried in a `u64`, which runs out of capacity there and not
    /// two bits earlier. The two bounds answer different overflow questions
    /// and asserting one from the other would cost this crate two widths for
    /// no reason.
    const ADMITTED: () = {
        assert!(
            N >= 1,
            "a declared width of zero bits admits no values and is not a bit container"
        );
        assert!(
            N <= 64,
            "declared width is wider than a u64 carries; the value itself is stored in one, and \
             a width past sixty-four has nowhere to be masked into"
        );
    };
    /// The mask covering exactly the low `N` bits, itself a value at width `N`
    /// with every one of those bits set.
    ///
    /// Carried as `Self` rather than as a bare `u64`: this crate is the door
    /// that admits the host's own integer types, and that door is for
    /// defining `Bits` itself, not for spelling an internal constant in the
    /// host's vocabulary when this crate's own type already says the same
    /// thing. A mask at width `N` is exactly a value at width `N`.
    ///
    /// `1u64 << 64` is a shift-overflow panic in this compiler, so the case
    /// where `N` is the full width is taken explicitly rather than trusted to
    /// fall out of the general formula.
    const MASK: Self = if N == 64 { Self(u64::MAX) } else { Self((1u64 << N) - 1) };

    /// A value at width `N`, masked down from a wider host value.
    ///
    /// Total: there is no `raw` this refuses, only bits above `N` that get
    /// dropped. The compile-time refusal is on `N` at the type's own
    /// construction, never on a value handed to this function.
    #[must_use]
    pub const fn masked(raw: u64) -> Self {
        let () = Self::ADMITTED;
        Self(raw & Self::MASK.0)
    }

    /// The value at its natural host width, for the one place a host contract
    /// needs it back.
    ///
    /// The unwrap door, declared as one. `repr(transparent)` and this accessor
    /// are the whole observation surface.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// This value reinterpreted at a different declared width, by masking
    /// again.
    ///
    /// The same masking arithmetic at a different bound, covering both
    /// directions with one operation rather than a widen method and a
    /// separate narrow method for what is structurally one fact. Widening
    /// (`M > N`) never drops a bit, because the wider mask is a strict
    /// superset of the narrower one. Narrowing (`M < N`) drops the bits above
    /// `M`, which is exactly what a masked cast at a boundary the consumer
    /// controls already means in the consumer's own vocabulary.
    #[must_use]
    pub const fn cast<const M: u32>(self) -> Bits<M> {
        Bits::<M>::masked(self.raw())
    }
}

#[cfg(test)]
mod tests;
