// The trait's impl calls the inherent method, so the refusal reaches through a
// function generic over `BitAccess` as well, which is where a delegation that
// stopped forcing the assertion would go unnoticed.

#![feature(const_trait_impl)]

use arvo_bits::{BitAccess, Bits};
use arvo_format::Bool;

const fn eighth<B: [const] BitAccess>(b: B) -> Bool {
    b.bit::<8>()
}

const _REFUSED: Bool = eighth(Bits::<8>::masked(1));

fn main() {}
