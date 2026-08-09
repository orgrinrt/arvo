// p5: what a datum-keyed digest masks to, when the placement map has a hole.
//
// The ratified datum-keyed digest "masks the container straight to the fields'
// own width", which is a PREFIX mask: one AND against `(1 << W_F) - 1`. That is
// correct for a numeral, whose fields occupy a contiguous low run.
//
// A bitfield's placement map need not be contiguous. A declared gap (reserved
// bits in a foreign register, an ignored lane) leaves an interior region that
// no ratified statement names: statement P covers [W_F, W_S), statement C
// covers [W_S, W_C), and neither covers a hole strictly inside [0, W_F).
//
// This probe exhibits the consequence and the one-word repair.
//
// No feature gates. Edition 2024.

// Reg: 16 bits. enable at 0 (1 bit), divisor at 5 (9 bits). Bits 1..5 and
// 14..16 are declared by nothing.
const FIELDS: [(u32, u32); 2] = [(0, 1), (5, 9)];

const fn extent(fs: &[(u32, u32)]) -> u32 {
    let mut i = 0;
    let mut m = 0;
    while i < fs.len() {
        let e = fs[i].0 + fs[i].1;
        if e > m {
            m = e;
        }
        i += 1;
    }
    m
}
const fn union_mask(fs: &[(u32, u32)]) -> u16 {
    let mut i = 0;
    let mut m: u16 = 0;
    while i < fs.len() {
        m |= (((1u32 << fs[i].1) - 1) << fs[i].0) as u16;
        i += 1;
    }
    m
}

pub const W_F: u32 = extent(&FIELDS); // 14
pub const PREFIX_MASK: u16 = ((1u32 << W_F) - 1) as u16; // 0x3FFF
pub const UNION_MASK: u16 = union_mask(&FIELDS); // 0x3FE1

#[inline(never)]
pub fn digest_prefix(v: u16) -> u16 {
    v & PREFIX_MASK
}
#[inline(never)]
pub fn digest_union(v: u16) -> u16 {
    v & UNION_MASK
}

fn get(v: u16, k: usize) -> u16 {
    (v >> FIELDS[k].0) & ((1u16 << FIELDS[k].1) - 1)
}

fn main() {
    println!("W_F = {}, prefix mask = {:#018b}", W_F, PREFIX_MASK);
    println!("       union  mask = {:#018b}", UNION_MASK);
    println!(
        "hole  = prefix & !union = {:#018b}",
        PREFIX_MASK & !UNION_MASK
    );

    // exhaustive over the whole 16-bit container, both directions
    let mut prefix_separates = 0u32; // equal at every field, different digest
    let mut union_separates = 0u32;
    let mut prefix_conflates = 0u32; // different at some field, same digest
    let mut union_conflates = 0u32;
    for a in 0u16..=u16::MAX {
        for b in [a ^ 0x001E, a ^ 0xC000, a ^ 0x0020] {
            // hole bits, container padding, a real field bit
            let same_fields = (0..FIELDS.len()).all(|k| get(a, k) == get(b, k));
            let dp = digest_prefix(a) == digest_prefix(b);
            let du = digest_union(a) == digest_union(b);
            if same_fields && !dp {
                prefix_separates += 1;
            }
            if same_fields && !du {
                union_separates += 1;
            }
            if !same_fields && dp {
                prefix_conflates += 1;
            }
            if !same_fields && du {
                union_conflates += 1;
            }
        }
    }
    println!(
        "prefix mask: separates {} equal pairs, conflates {} unequal pairs",
        prefix_separates, prefix_conflates
    );
    println!(
        "union  mask: separates {} equal pairs, conflates {} unequal pairs",
        union_separates, union_conflates
    );
}
