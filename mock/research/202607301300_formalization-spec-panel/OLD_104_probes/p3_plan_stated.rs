// p3: the decode plan stated, and which composition of the two placements
// wins.
//
// File 81 measured the group arithmetic as associated consts, unrolled by the
// period, and got the 4.6x down to 1.50x. This probe carries that plan one
// level in, to a bitfield element, and asks which of the two composite forms
// to emit.
//
//   two-step: one load per element, one shift+mask to the element, then a
//             shift+mask per field. The element load amortises across fields.
//   one-step: one load per field at the composed offset, one shift+mask. No
//             intermediate, but no sharing either.
//
// Stride 13, period P = 8, group G = 13 bytes, fields (0,3) (3,5) (8,5).
// Bodies unrolled by the period so every shift is a literal, per file 81's own
// binding-time finding.
//
// No feature gates. Edition 2024.

pub const WS: usize = 13;
pub const P: usize = 8; // 8 / gcd(13, 8)
pub const G: usize = 13; // WS * P / 8
pub const GROUPS: usize = 512;

// Unchecked, because a bounds check per load would price the two forms by how
// many loads they issue rather than by the work each one does, which is the
// question. The caller guarantees the read headroom (file 81's own row).
#[inline(always)]
fn ld(buf: &[u8], byte: usize) -> u64 {
    unsafe {
        (buf.as_ptr().add(byte) as *const u64)
            .read_unaligned()
            .to_le()
    }
}

macro_rules! lanes {
    () => {
        [0usize, 1, 2, 3, 4, 5, 6, 7]
    };
}

#[inline(never)]
pub fn one_field_two_step(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for g in 0..GROUPS {
        let base = g * G;
        for j in lanes!() {
            let bit = j * WS;
            let elem = (ld(buf, base + bit / 8) >> (bit % 8)) & ((1u64 << WS) - 1);
            acc += (elem >> 3) & ((1u64 << 5) - 1);
        }
    }
    acc
}

#[inline(never)]
pub fn one_field_one_step(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for g in 0..GROUPS {
        let base = g * G;
        for j in lanes!() {
            let bit = j * WS + 3;
            acc += (ld(buf, base + bit / 8) >> (bit % 8)) & ((1u64 << 5) - 1);
        }
    }
    acc
}

#[inline(never)]
pub fn all_fields_two_step(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for g in 0..GROUPS {
        let base = g * G;
        for j in lanes!() {
            let bit = j * WS;
            let elem = (ld(buf, base + bit / 8) >> (bit % 8)) & ((1u64 << WS) - 1);
            acc += (elem) & 0x7;
            acc += (elem >> 3) & 0x1F;
            acc += (elem >> 8) & 0x1F;
        }
    }
    acc
}

#[inline(never)]
pub fn all_fields_one_step(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for g in 0..GROUPS {
        let base = g * G;
        for j in lanes!() {
            let b0 = j * WS;
            let b1 = j * WS + 3;
            let b2 = j * WS + 8;
            acc += (ld(buf, base + b0 / 8) >> (b0 % 8)) & 0x7;
            acc += (ld(buf, base + b1 / 8) >> (b1 % 8)) & 0x1F;
            acc += (ld(buf, base + b2 / 8) >> (b2 % 8)) & 0x1F;
        }
    }
    acc
}

fn main() {
    let buf: Vec<u8> = (0..GROUPS * G + 16)
        .map(|i| (i as u8).wrapping_mul(97))
        .collect();
    let a = one_field_two_step(&buf);
    let b = one_field_one_step(&buf);
    let c = all_fields_two_step(&buf);
    let d = all_fields_one_step(&buf);
    println!(
        "one-field  two-step = {}, one-step = {}, agree = {}",
        a,
        b,
        a == b
    );
    println!(
        "all-fields two-step = {}, one-step = {}, agree = {}",
        c,
        d,
        c == d
    );
}
