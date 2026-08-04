// Probe 1: is a numeral's byte-serialisation buffer expressible gate-free, and is
// this the spine rule firing again on a fourth quantity (after ShortCap in file 72)?
//
// The claim: byte_width(N) is a quantity computed from a numeral's StoredWidth
// (bits) and then has to appear in a type (an array length, sizes const at type
// level, no alloc). The spine rule says such a quantity is a type, not a const
// generic expression. Compiled here as two halves: the const-expression route
// refuses under the forbidden `generic_const_exprs` (probe_1b, isolated so this
// file's happy path compiles standalone), the associated-type route does not,
// and is const-callable end to end, per the brief's "everything wants a
// const-callable form" constraint.

pub trait ByteCapacity {
    const BYTES: u16;
    type Arr: Copy;
    const ZERO: Self::Arr;
}

pub struct C4;
impl ByteCapacity for C4 {
    const BYTES: u16 = 4;
    type Arr = [u8; 4];
    const ZERO: [u8; 4] = [0u8; 4];
}

pub struct C8;
impl ByteCapacity for C8 {
    const BYTES: u16 = 8;
    type Arr = [u8; 8];
    const ZERO: [u8; 8] = [0u8; 8];
}

// a minimal stand-in for `Lowering`: only the one member this probe needs.
pub trait HasStoredWidthBits {
    const STORED_WIDTH_BITS: u16;
    type ByteCap: ByteCapacity;
}

pub const fn byte_width_of(bits: u16) -> u16 {
    (bits + 7) / 8
}

// declaration-site check: the chosen capacity must cover the computed width.
// this is the same shape file 72's probe_3 used for ShortCap, reapplied to a
// different computed quantity (byte width instead of digit-string length).
pub struct ThirteenBitDense;
impl HasStoredWidthBits for ThirteenBitDense {
    const STORED_WIDTH_BITS: u16 = 13;
    type ByteCap = C4; // covers: byte_width_of(13) = 2 <= 4
}
const _: () = assert!(C4::BYTES >= byte_width_of(ThirteenBitDense::STORED_WIDTH_BITS));

pub struct SixtyFourBitDense;
impl HasStoredWidthBits for SixtyFourBitDense {
    const STORED_WIDTH_BITS: u16 = 64;
    type ByteCap = C8; // covers: byte_width_of(64) = 8 <= 8, exact
}
const _: () = assert!(C8::BYTES >= byte_width_of(SixtyFourBitDense::STORED_WIDTH_BITS));

// generic, const-callable function over any numeral-shaped type; no const
// expression appears in type position anywhere in this signature or body.
pub const fn zero_bytes<N: HasStoredWidthBits>() -> <N::ByteCap as ByteCapacity>::Arr {
    <N::ByteCap as ByteCapacity>::ZERO
}

const _CHECK_13: [u8; 4] = zero_bytes::<ThirteenBitDense>();
const _CHECK_64: [u8; 8] = zero_bytes::<SixtyFourBitDense>();

fn main() {
    let a: [u8; 4] = zero_bytes::<ThirteenBitDense>();
    let b: [u8; 8] = zero_bytes::<SixtyFourBitDense>();
    assert_eq!(a, _CHECK_13);
    assert_eq!(b, _CHECK_64);
    assert_eq!(a, [0u8; 4]);
    assert_eq!(b, [0u8; 8]);
    println!(
        "byte_width_of(13) = {}, byte_width_of(64) = {}, both routed through ByteCap, const-callable, zero gates",
        byte_width_of(13),
        byte_width_of(64)
    );
}
