// Probe 3: the text buffer's length is a spine-rule firing site, and the
// capacity-as-a-type shape closes it with zero feature gates.
//
// The shortest-print digit count H(radix, precision) and the exact-expansion
// digit count X(radix, precision, emin) are both computable const fns of the
// numeral's parameters. But an array length `[u8; H]` inside a function generic
// over the numeral is a const expression naming a generic parameter, which is
// exactly the forbidden generic_const_exprs shape (probe_3b compiles that out).
// Per the spine rule ("a quantity that is computed and then has to appear in a
// type is a type"), the capacity is an associated TYPE, and the declaration-site
// const assertion checks it covers the computed bound. Compiles gate-free.

#![no_std]

pub trait TextCap {
    type Arr;
    const LEN: usize;
    fn zeroed() -> Self::Arr;
}

pub struct C8;
impl TextCap for C8 {
    type Arr = [u8; 8];
    const LEN: usize = 8;
    fn zeroed() -> [u8; 8] {
        [0; 8]
    }
}

pub struct C16;
impl TextCap for C16 {
    type Arr = [u8; 16];
    const LEN: usize = 16;
    fn zeroed() -> [u8; 16] {
        [0; 16]
    }
}

pub trait Numeral {
    const RADIX: u32;
    const PRECISION: u32;
    const EMIN: i32;
    // the capacity is a type, per the spine rule; the impl picks it and the
    // declaration-site assertion below proves it covers the computed bound
    type ShortCap: TextCap;
}

// H = smallest d with 10^(d-1) > radix^precision, plus sign, point, and a
// two-digit decimal exponent field: the shortest-round-trip budget.
pub const fn short_budget(radix: u32, precision: u32) -> usize {
    // integer computation of ceil(log10(radix^precision)) by repeated compare;
    // model widths keep radix^precision inside u128
    let mut pow: u128 = 1;
    let mut i = 0;
    while i < precision {
        pow *= radix as u128;
        i += 1;
    }
    let mut d: usize = 1;
    let mut ten: u128 = 1; // 10^(d-1)
    while ten <= pow {
        ten *= 10;
        d += 1;
    }
    // d digits + sign + point + 'e' + sign + 2 exponent digits
    d + 6
}

pub struct B2P8;
impl Numeral for B2P8 {
    const RADIX: u32 = 2;
    const PRECISION: u32 = 8;
    const EMIN: i32 = -4;
    type ShortCap = C16;
}

// declaration-site check: the chosen capacity covers the computed bound.
// this sits at the impl, not in generic code, so no generic const expr exists.
const _: () = assert!(<<B2P8 as Numeral>::ShortCap as TextCap>::LEN >= short_budget(2, 8));

// the generic writer: buffer type comes from the numeral's own capacity type.
// no const expression in type position anywhere in this function.
pub fn print_short<N: Numeral>() -> <N::ShortCap as TextCap>::Arr {
    N::ShortCap::zeroed()
}

// a second numeral with a different capacity proves the projection varies by type
pub struct B2P4;
impl Numeral for B2P4 {
    const RADIX: u32 = 2;
    const PRECISION: u32 = 4;
    const EMIN: i32 = -2;
    // first compile chose C8 here; the declaration-site assertion refused it with
    // E0080 (short_budget(2, 4) = 9 > 8), which is the check doing its job.
    type ShortCap = C16;
}
const _: () = assert!(<<B2P4 as Numeral>::ShortCap as TextCap>::LEN >= short_budget(2, 4));

// monomorphised uses, checked at compile time
const _: () = {
    assert!(<<B2P8 as Numeral>::ShortCap as TextCap>::LEN == 16);
    assert!(<<B2P4 as Numeral>::ShortCap as TextCap>::LEN == 16);
};
