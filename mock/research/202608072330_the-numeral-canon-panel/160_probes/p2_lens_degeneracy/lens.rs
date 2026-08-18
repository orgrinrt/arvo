// 160 P2. The lens degenerates to a value on SOLE OCCUPANCY, not on position zero.
//
// CLAIM UNDER TEST
//   157 S-8 (adopted by 158 and 159): "a primitive's realisation is always a lens
//   (carrier, position); where the position is const-zero and the carrier is one
//   machine word, the lens is an identity and the thing is a value."
//   The stated condition is INSUFFICIENT: the first element of a packed column has
//   position zero in one machine word and is not a value, because siblings share the
//   carrier. The condition that discharges is sole occupancy of the carrier
//   allocation, with padding permitted.
//
// THE CASE THAT MUST FAIL, declared before the run
//   A lens whose extent leaves its carrier, Lens64::<60, 13>, must be refused at
//   compile time (E0080 via a const assertion in the type's own witness), built with
//   --cfg control. If that build succeeds the lens formalisation admits an
//   out-of-carrier focus and this probe proves nothing.
//
// WHAT IS CITED RATHER THAN REDONE
//   That no Rust type has exactly 13 bits is 154 F6 (fibre_refuted.err), widened by
//   159 to `W any where W mod 8 != 0` on the size_of-in-bytes argument. Not repeated.

#![allow(dead_code)]

use std::mem::size_of;

// The lens: a focus of W bits at offset OFF inside a u64 carrier.
struct Lens64<const OFF: u32, const W: u32> {
    carrier: *const u64,
}

impl<const OFF: u32, const W: u32> Lens64<OFF, W> {
    // The witness that the focus stays inside the carrier. Instantiating an
    // out-of-carrier lens evaluates this and refuses at compile time.
    const IN_CARRIER: () = assert!(OFF + W <= 64, "lens focus leaves the carrier");

    fn get(&self, idx: usize) -> u64 {
        let _ = Self::IN_CARRIER;
        let word = unsafe { *self.carrier.add(idx) };
        (word >> OFF) & ((1u64 << W) - 1)
    }
}

// The degenerate case as the discriminator states it: SOLE OCCUPANT of its carrier
// allocation, position zero, padding permitted. It is an ordinary Sized value.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Dense13(u16); // 13 logical bits, 3 bits padding, sole occupant

// The case S-8's wording wrongly admits: position zero, one machine word, SHARED.
// The only expressible standalone form carries the carrier, so it is pointer-sized,
// and observing it observes the allocation the siblings live in.
struct First13<'a>(&'a u64);

fn main() {
    // Sole occupant: a value. Referenceable, Sized, byte-addressed with padding.
    let v = Dense13(0x1abc & 0x1fff);
    let r: &Dense13 = &v;
    println!("dense13 size_of*8 = {} (13 logical + 3 padding)", size_of::<Dense13>() * 8);
    println!("dense13 referenceable: {:?}", *r);

    // Shared occupant at position zero: not a value. The standalone form is the
    // size of a pointer, not of the focus.
    let col: [u64; 2] = [(0x0aaa) | (0x1555 << 13), 0];
    let f = First13(&col[0]);
    println!("first13 size_of = {} bytes (pointer, not ceil(13/8) = 2)", size_of::<First13<'_>>());
    println!("first13 focus = {:#x}", f.0 & 0x1fff);
    println!("sibling observable through the same carrier: {:#x}", (f.0 >> 13) & 0x1fff);

    // The general lens over the same carrier, in-carrier instantiations.
    let l0 = Lens64::<0, 13> { carrier: col.as_ptr() };
    let l1 = Lens64::<13, 13> { carrier: col.as_ptr() };
    println!("lens<0,13>.get(0)  = {:#x}", l0.get(0));
    println!("lens<13,13>.get(0) = {:#x}", l1.get(0));

    // Degenerate-lens equivalence, checked over every 13-bit value: the sole-occupant
    // value and the lens at offset zero over a carrier holding only it agree.
    let mut disagreements = 0u32;
    for x in 0u64..(1 << 13) {
        let value_read = Dense13(x as u16).0 as u64 & 0x1fff;
        let word = x;
        let lens = Lens64::<0, 13> { carrier: &word };
        if lens.get(0) != value_read { disagreements += 1; }
    }
    println!("degenerate lens against sole-occupant value, 8192 values: {} disagreements", disagreements);

    #[cfg(control)]
    {
        // MUST FAIL TO COMPILE: focus leaves the carrier.
        let bad = Lens64::<60, 13> { carrier: col.as_ptr() };
        println!("{}", bad.get(0));
    }
}
