// p2b: the placement map at type position against value position.
//
// File 81 found that the group arithmetic has to be written in a const
// position to be settled there; a `const fn` called from the decode is left to
// LLVM and is not folded. This probe asks the same of a bitfield's own
// placement, and adds the composition question: the two-step read collapses to
// the one-step read only when the compiler can see that the field is contained
// in the element, which is exactly the obligation the declaration carries.
//
// Four loop bodies over one packed column, stride 13, field (o=3, w=5):
//   const_two  : placement as literals, element then field
//   const_one  : placement as literals, composed offset
//   dyn_two    : placement as runtime arguments, element then field
//   dyn_one    : placement as runtime arguments, composed offset
//
// No feature gates. Edition 2024.

pub const WS: usize = 13;
pub const N: usize = 4096;

#[inline(never)]
pub fn const_two(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for i in 0..N {
        let bit = i * WS;
        let word = u64::from_le_bytes(buf[bit / 8..bit / 8 + 8].try_into().unwrap());
        let elem = (word >> (bit % 8)) & ((1u64 << WS) - 1);
        acc += (elem >> 3) & ((1u64 << 5) - 1);
    }
    acc
}

#[inline(never)]
pub fn const_one(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for i in 0..N {
        let bit = i * WS + 3;
        let word = u64::from_le_bytes(buf[bit / 8..bit / 8 + 8].try_into().unwrap());
        acc += (word >> (bit % 8)) & ((1u64 << 5) - 1);
    }
    acc
}

#[inline(never)]
pub fn dyn_two(buf: &[u8], ws: usize, o: usize, w: usize) -> u64 {
    let mut acc = 0u64;
    for i in 0..N {
        let bit = i * ws;
        let word = u64::from_le_bytes(buf[bit / 8..bit / 8 + 8].try_into().unwrap());
        let elem = (word >> (bit % 8)) & ((1u64 << ws) - 1);
        acc += (elem >> o) & ((1u64 << w) - 1);
    }
    acc
}

#[inline(never)]
pub fn dyn_one(buf: &[u8], ws: usize, o: usize, w: usize) -> u64 {
    let mut acc = 0u64;
    for i in 0..N {
        let bit = i * ws + o;
        let word = u64::from_le_bytes(buf[bit / 8..bit / 8 + 8].try_into().unwrap());
        acc += (word >> (bit % 8)) & ((1u64 << w) - 1);
    }
    acc
}

fn main() {
    let buf = vec![0xA5u8; N * WS / 8 + 16];
    println!(
        "{} {} {} {}",
        const_two(&buf),
        const_one(&buf),
        dyn_two(&buf, WS, 3, 5),
        dyn_one(&buf, WS, 3, 5)
    );
}
