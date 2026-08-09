// p2: does a `place` map compose?
//
// A bitpacked column places element i of stride W_S at absolute bit i*W_S.
// A bitfield places field f of width w at intra-element bit o.
// Reading field f of element i is therefore a composite of two placements.
//
// Question one (correctness): is the composite a placement, i.e. does the
// one-step read at absolute bit i*W_S + o agree with the two-step read at
// every element and every field?
//
// Question two (cost): does the composite have to be STATED, or does the
// optimiser collapse the two-step form on its own? File 81 found the analogous
// answer for the group arithmetic and it was "stated".
//
// No feature gates. Edition 2024.

const WS: usize = 13; // element stride, bits
                      // (offset, width) inside the element. Sums to 13, contiguous, no hole.
const FIELDS: [(usize, usize); 3] = [(0, 3), (3, 5), (8, 5)];

const N: usize = 4096; // elements
const BYTES: usize = (N * WS + 7) / 8 + 8; // + read headroom

fn pack(vals: &[[u32; 3]; N]) -> Vec<u8> {
    let mut buf = vec![0u8; BYTES];
    for (i, v) in vals.iter().enumerate() {
        // build the element from its fields, then place the element
        let mut elem: u64 = 0;
        for (k, &(o, w)) in FIELDS.iter().enumerate() {
            elem |= ((v[k] as u64) & ((1u64 << w) - 1)) << o;
        }
        let bit = i * WS;
        let byte = bit / 8;
        let sh = bit % 8;
        let cur = u64::from_le_bytes(buf[byte..byte + 8].try_into().unwrap());
        let placed = cur | (elem << sh);
        buf[byte..byte + 8].copy_from_slice(&placed.to_le_bytes());
    }
    buf
}

// two-step: materialise the element, then slice the field out of it
#[inline(never)]
fn read_two_step(buf: &[u8], i: usize, k: usize) -> u32 {
    let bit = i * WS;
    let byte = bit / 8;
    let sh = bit % 8;
    let word = u64::from_le_bytes(buf[byte..byte + 8].try_into().unwrap());
    let elem = (word >> sh) & ((1u64 << WS) - 1);
    let (o, w) = FIELDS[k];
    ((elem >> o) & ((1u64 << w) - 1)) as u32
}

// one-step: slice the field out of the buffer at the composed offset
#[inline(never)]
fn read_one_step(buf: &[u8], i: usize, k: usize) -> u32 {
    let (o, w) = FIELDS[k];
    let bit = i * WS + o;
    let byte = bit / 8;
    let sh = bit % 8;
    let word = u64::from_le_bytes(buf[byte..byte + 8].try_into().unwrap());
    ((word >> sh) & ((1u64 << w) - 1)) as u32
}

// the two loop bodies whose emitted code the asm dump compares. Field 1
// (offset 3, width 5) chosen because neither its offset nor its width is a
// multiple of eight, so nothing degenerates.
#[inline(never)]
pub fn sum_two_step(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for i in 0..N {
        let bit = i * WS;
        let byte = bit / 8;
        let sh = bit % 8;
        let word = u64::from_le_bytes(buf[byte..byte + 8].try_into().unwrap());
        let elem = (word >> sh) & ((1u64 << WS) - 1);
        acc += (elem >> 3) & ((1u64 << 5) - 1);
    }
    acc
}

#[inline(never)]
pub fn sum_one_step(buf: &[u8]) -> u64 {
    let mut acc = 0u64;
    for i in 0..N {
        let bit = i * WS + 3;
        let byte = bit / 8;
        let sh = bit % 8;
        let word = u64::from_le_bytes(buf[byte..byte + 8].try_into().unwrap());
        acc += (word >> sh) & ((1u64 << 5) - 1);
    }
    acc
}

fn main() {
    // every element gets a distinct field triple; sweep the whole 13-bit
    // element space by construction rather than by sampling.
    let mut vals = [[0u32; 3]; N];
    for i in 0..N {
        let e = (i as u32) & 0x1FFF;
        vals[i] = [e & 0x7, (e >> 3) & 0x1F, (e >> 8) & 0x1F];
    }
    let buf = pack(&vals);

    let mut mismatch_pack = 0usize;
    let mut mismatch_compose = 0usize;
    for i in 0..N {
        for k in 0..3 {
            let two = read_two_step(&buf, i, k);
            let one = read_one_step(&buf, i, k);
            if two != vals[i][k] {
                mismatch_pack += 1;
            }
            if two != one {
                mismatch_compose += 1;
            }
        }
    }
    println!(
        "elements = {}, stride = {} bits, fields = {:?}",
        N, WS, FIELDS
    );
    println!(
        "round-trip mismatches (two-step vs packed input): {}",
        mismatch_pack
    );
    println!(
        "composition mismatches (one-step vs two-step):    {}",
        mismatch_compose
    );
    println!(
        "sum_two_step = {}, sum_one_step = {}",
        sum_two_step(&buf),
        sum_one_step(&buf)
    );

    // the group arithmetic, keyed on the stride rather than on the field width
    let g = gcd(WS, 8);
    println!(
        "period P = 8/gcd(WS,8) = {}, group bytes G = WS*P/8 = {}",
        8 / g,
        WS * (8 / g) / 8
    );
    for (k, &(o, w)) in FIELDS.iter().enumerate() {
        // lane shift of field k in element j of the period
        let shifts: Vec<usize> = (0..8 / g).map(|j| (j * WS + o) % 8).collect();
        println!(
            "  field {} (o={}, w={}): mask width {}, lane shifts over the period {:?}",
            k, o, w, w, shifts
        );
    }
}

const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
