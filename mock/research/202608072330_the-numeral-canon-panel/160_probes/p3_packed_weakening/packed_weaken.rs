// 160 P3. Weakening is free at the packed end too, and tightening is refused.
//
// CLAIM UNDER TEST
//   111 p8 compiled "widening a carried bound is the identity in the emitted code"
//   for a DENSE carrier (F111-12). The refinement-weakening clause of the offered
//   statement quantifies over the whole declared range, and the packed end is the one
//   region I17 forbids deprioritising, so the clause needs an instance there or its
//   predicate must exclude the region. This probe supplies the instance: a bitpacked
//   column type carrying a declared bound weakens to a looser bound as the identity,
//   with no per-element type existing anywhere (the elements are 13-bit lenses into
//   shared u64 words).
//
// THE CASE THAT MUST FAIL, declared before the run
//   The tightening direction, weaken::<200, 100>, must be refused at compile time
//   (E0080 naming the instantiation), built with --cfg control. If it compiles, the
//   weakening order is not being enforced and the "free" claim is about an unordered
//   cast rather than a weakening.
//
// SECOND CHECK, declared before the run
//   The emitted body of the monomorphised weaken_ref must be a pointer identity:
//   either aliased to a plain identity function or a lone `ret`. Checked by reading
//   the emitted assembly, committed alongside.

#![allow(dead_code)]

const N: usize = 64; // elements
const W: u32 = 13;
const WORDS: usize = (N * W as usize + 63) / 64;

// A bitpacked column of 13-bit elements with a declared upper bound on every element.
// No element type exists; an element is (column, index).
#[repr(transparent)]
pub struct PackedCol<const BOUND: u64> {
    words: [u64; WORDS],
}

impl<const BOUND: u64> PackedCol<BOUND> {
    pub fn get(&self, i: usize) -> u64 {
        let bit = i * W as usize;
        let (wi, off) = (bit / 64, (bit % 64) as u32);
        let lo = self.words[wi] >> off;
        let v = if off + W > 64 {
            lo | (self.words[wi + 1] << (64 - off))
        } else {
            lo
        };
        v & ((1u64 << W) - 1)
    }
}

// Weakening: from a tighter declared bound to a looser one. Identity on the
// representation, refused in the tightening direction at compile time.
#[inline(never)]
pub fn weaken_ref<const A: u64, const B: u64>(c: &PackedCol<A>) -> &PackedCol<B> {
    const { assert!(A <= B, "weakening must not tighten the bound") };
    // Layout-identical by repr(transparent) over the same field; the bound is a
    // compile-time fact with no representation.
    unsafe { &*(c as *const PackedCol<A> as *const PackedCol<B>) }
}

#[inline(never)]
pub fn plain_identity(c: &PackedCol<100>) -> &PackedCol<100> {
    c
}

fn main() {
    let mut words = [0u64; WORDS];
    // pack i*3 % 101 at each slot (all <= 100)
    for i in 0..N {
        let v = ((i as u64) * 3) % 101;
        let bit = i * W as usize;
        let (wi, off) = (bit / 64, (bit % 64) as u32);
        words[wi] |= v << off;
        if off + W > 64 {
            words[wi + 1] |= v >> (64 - off);
        }
    }
    let tight: PackedCol<100> = PackedCol { words };
    let loose: &PackedCol<200> = weaken_ref::<100, 200>(&tight);

    // representation unchanged: every element reads back identically through both.
    let mut disagreements = 0u32;
    for i in 0..N {
        if tight.get(i) != loose.get(i) {
            disagreements += 1;
        }
    }
    println!("packed weakening, {} elements: {} disagreements", N, disagreements);
    println!("same address through the weakening: {}", std::ptr::eq(
        &tight as *const _ as *const u8,
        loose as *const _ as *const u8,
    ));

    #[cfg(control)]
    {
        // MUST FAIL TO COMPILE: tightening.
        let _bad: &PackedCol<100> = weaken_ref::<200, 100>(loose);
    }
}
