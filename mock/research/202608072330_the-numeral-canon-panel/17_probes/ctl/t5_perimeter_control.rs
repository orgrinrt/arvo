// t5. The C1 perimeter instrument: is there any public observation through which a
// container enters or leaves?
//
// Clause one says the consumer expresses usage in bits and bytes, and the ratified
// companion at SETTLED.md:93 says the container is never written by a consumer. Together
// those are a claim about the whole observation surface of the numeral type, and a claim
// of that shape holds only over the operations through which the type can be observed.
// One public field, one From impl, one Deref is a hole in it, and none of them appears
// in the same file as the argument they undermine.
//
// This file declares a numeral type with SIX doors. Three are legitimate. Three leak the
// container. Every one of them compiles, and every behavioural check below is green, so
// no test shape in the panel would notice. The instrument that notices is an enumeration
// of the surface, which t5_perimeter.py performs against rustdoc's JSON.
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib t5_perimeter.rs \
//     --emit metadata -o /dev/null
//   rustdoc +nightly-2026-05-28 --edition 2021 -Z unstable-options \
//     --output-format json --out-dir json t5_perimeter.rs
//
// Features: none in this file. The rustdoc JSON format is itself unstable and is a tool
// surface rather than a language feature; nothing the design ships depends on it.
//
// Spike. Presume it flawed. The leak list is one I chose, and the point is the shape of
// the check rather than the completeness of the list.

#![no_std]
#![allow(dead_code)]

pub struct Hot;
pub struct Cold;

/// The declaration. This is what a consumer writes: a width and a strategy.
pub struct UFixed<const W: usize, S> {
    // PRIVATE. The container is derived, and the consumer never writes it.
    raw: u16,
    _s: core::marker::PhantomData<S>,
}

// ---------------------------------------------------------------------------
// Three legitimate doors. None of them names the container in a consumer position.
// ---------------------------------------------------------------------------
impl<const W: usize, S> UFixed<W, S> {
    /// Legitimate: takes a declared-width value as bits, returns the numeral.
    pub const fn from_bits(_b: Bits<W>) -> Self {
        UFixed {
            raw: 0,
            _s: core::marker::PhantomData,
        }
    }
    /// Legitimate: the declared width, which the consumer wrote.
    pub const fn width(&self) -> usize {
        W
    }
    /// Legitimate: a numeral in, a numeral out.
    pub const fn add(self, _o: Self) -> Self {
        self
    }
}

/// A width-carrying bit value. Consumer-facing, and not a container.
pub struct Bits<const W: usize>(u8);

// ---------------------------------------------------------------------------
// Three leaks. Each is one line, each compiles, and each defeats the clause.
// ---------------------------------------------------------------------------

// (the three leaks are removed in this control)

// ---------------------------------------------------------------------------
// The behavioural suite. All green, over a surface with three holes in it.
// ---------------------------------------------------------------------------
pub const fn behaviour_checks_all_pass() -> bool {
    let a = UFixed::<13, Hot>::from_bits(Bits::<13>(0));
    let b = UFixed::<13, Hot>::from_bits(Bits::<13>(0));
    let c = a.add(b);
    c.width() == 13
}

const _: () = assert!(behaviour_checks_all_pass());
const _: () = assert!(core::mem::size_of::<UFixed<13, Hot>>() == 2);
const _: () = assert!(core::mem::size_of::<UFixed<13, Cold>>() == 2);
