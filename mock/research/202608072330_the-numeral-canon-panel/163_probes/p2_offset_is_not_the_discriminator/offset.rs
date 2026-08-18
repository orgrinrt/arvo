// 163 P2. S-8's degeneracy condition fails in a THIRD direction, which 160's probe does
// not instantiate: a SOLE occupant at a NONZERO offset.
//
// CLAIM UNDER TEST
//   157 S-8 (157:358-362): "where the position is const-zero and the carrier is one
//   machine word, the lens is an identity and the thing is a value."
//   160 section 2.1 shows the condition admits a packed column's first element (offset 0,
//   shared) and, read literally, excludes a padded sole occupant (Dense13's lens is a mask,
//   not the identity). This file adds the third case: sole occupancy at offset != 0.
//
// NEGATIVE CONTROL, declared before the run
//   A SHARED occupant at the same nonzero offset must NOT behave as a value: its only
//   standalone form must carry the whole carrier, and a sibling's bits must be reachable
//   through the same reference. If the shared case behaves like the sole case, this probe
//   is not measuring occupancy and proves nothing.
//   A second control: an out-of-carrier focus must be refused at compile time.
//
// BUILD
//   rustc --edition 2021 -O offset.rs -o off && ./off
//   rustc --edition 2021 -O --cfg oob offset.rs        (must FAIL: E0080)

#![allow(dead_code)]

const fn fits(off: u32, w: u32, carrier: u32) -> bool { off + w <= carrier }

// A sole occupant at a nonzero offset: 13 bits at offset 3 of a u16, bits 0..3 unused.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
struct SoleAt3(u16);
impl SoleAt3 {
    const OFF: u32 = 3;
    const W: u32 = 13;
    const _CHK: () = assert!(fits(Self::OFF, Self::W, 16), "lens focus leaves the carrier");
    const fn new(v: u16) -> Self { SoleAt3((v & 0x1FFF) << Self::OFF) }
    const fn get(self) -> u16 { (self.0 >> Self::OFF) & 0x1FFF }
}

// A shared occupant at the same nonzero offset: 13 bits at offset 3 of a u64 that also
// carries a sibling in bits 16..29. The CONTROL.
#[derive(Clone, Copy)]
struct SharedCarrier(u64);
impl SharedCarrier {
    const OFF_A: u32 = 3;
    const OFF_B: u32 = 16;
    const fn new(a: u16, b: u16) -> Self {
        SharedCarrier(((a as u64 & 0x1FFF) << Self::OFF_A) | ((b as u64 & 0x1FFF) << Self::OFF_B))
    }
    const fn get_a(self) -> u16 { ((self.0 >> Self::OFF_A) & 0x1FFF) as u16 }
    const fn get_b(self) -> u16 { ((self.0 >> Self::OFF_B) & 0x1FFF) as u16 }
}

#[cfg(oob)]
const _OOB: () = assert!(fits(60, 13, 64), "lens focus leaves the carrier");

fn main() {
    // 1. The sole occupant at offset 3 round-trips over the whole 13-bit domain.
    let mut bad = 0u32;
    let mut v = 0u16;
    while v < 8192 {
        if SoleAt3::new(v).get() != v { bad += 1; }
        v += 1;
    }
    println!("sole-at-offset-3 round trip over all 8192 values : {} disagreements", bad);

    // 2. It is Sized, referenceable, and its standalone size is the carrier's, not 13 bits.
    let x = SoleAt3::new(4095);
    let r: &SoleAt3 = &x;
    println!("sole-at-offset-3 size_of                          : {} bytes", core::mem::size_of::<SoleAt3>());
    println!("sole-at-offset-3 reachable through a reference    : {}", r.get());

    // 3. S-8 as worded classifies it. Offset is 3, so S-8 says NOT a value.
    println!("S-8's condition (offset==0 && carrier==one word)  : {}",
             if SoleAt3::OFF == 0 { "value" } else { "NOT a value" });
    println!("sole-occupancy condition                          : {}",
             "value (nothing else lives in this allocation)");

    // 4. CONTROL: the shared occupant at the same offset.
    let s = SharedCarrier::new(4095, 777);
    println!("shared-at-offset-3 standalone size                : {} bytes",
             core::mem::size_of::<SharedCarrier>());
    println!("shared-at-offset-3 sibling reachable from the SAME reference : {}", s.get_b());
    println!("CONTROL fires (sibling observable through one reference)     : {}",
             s.get_b() == 777);
}
