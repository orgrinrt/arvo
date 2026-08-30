// p1: a chain component owned by the compilation, constant at monomorphisation.
//
// Hypothesis: a representable set Q resolved per compilation (cfg) supports every
// compile-time validation instrument unchanged. Within one compilation Q is a constant
// of the type, membership is a const fn of (type, bits) alone with no extra runtime
// argument, the induced law check runs exhaustively at the model width, and the erased
// carrier has the container's size. Two compilations under two cfg flags produce two
// different constant Qs, each internally validated.
//
// The cfg flag is a custom stand-in for cfg(target_pointer_width): the same cfg
// machinery, chosen so the probe needs no second installed target. Compile once with
// --cfg q_small and once with --cfg q_wide; both transcripts are committed beside this
// file. This is a spike: names and shapes are scaffolding, not proposals.
//
// Bears on: OPTIONS.md Q26 (what kind of thing is a platform-width type), 63:141-147
// (C2's "Q is a constant of the type"), 68 section 5 (validity from (type, bits) alone).

#![no_std]

pub struct W;

#[cfg(q_small)]
impl W {
    pub const Q_MAX: u32 = 15; // a 4-bit window on this "target"
}

#[cfg(q_wide)]
impl W {
    pub const Q_MAX: u32 = 63; // a 6-bit window on this "target"
}

impl W {
    pub const Q_SIZE: u32 = W::Q_MAX + 1;

    // membership: pure const fn of (type, bits); no runtime state consulted
    pub const fn valid(bits: u32) -> bool {
        bits <= W::Q_MAX
    }
}

// the wrap reduction induced onto this compilation's Q
const fn wrap_add(a: u32, b: u32) -> u32 {
    (a + b) % W::Q_SIZE
}

// exhaustive associativity over this compilation's own Q, in const context
const ASSOC_HOLDS: bool = {
    let mut a = 0;
    let mut ok = true;
    while a < W::Q_SIZE {
        let mut b = 0;
        while b < W::Q_SIZE {
            let mut c = 0;
            while c < W::Q_SIZE {
                let l = wrap_add(wrap_add(a, b), c);
                let r = wrap_add(a, wrap_add(b, c));
                if l != r {
                    ok = false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    ok
};

const _: () = assert!(ASSOC_HOLDS);
const _: () = assert!(W::valid(W::Q_MAX));
const _: () = assert!(!W::valid(W::Q_MAX + 1));

// the two compilations genuinely carry different constant Qs
#[cfg(q_small)]
const _: () = assert!(W::Q_SIZE == 16);
#[cfg(q_wide)]
const _: () = assert!(W::Q_SIZE == 64);

// erasure instrument: the carrier newtype has the container's size
#[repr(transparent)]
pub struct Num(u32);

const _: () = assert!(core::mem::size_of::<Num>() == core::mem::size_of::<u32>());
