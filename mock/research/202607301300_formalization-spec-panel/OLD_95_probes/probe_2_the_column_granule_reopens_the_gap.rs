//! Probe 2. File 92 section 2.1's fourth bullet, compiled rather than reasoned.
//!
//! File 92 states, in prose: "A column whose safe surface hands out `&mut [u8]`
//! of its backing bytes, an API every storage crate is tempted to ship, reopens
//! the gap at column granularity, tail-group padding included, with no
//! per-element accessor anywhere in sight" (92:247-250). It is offered as the
//! reason the mutation theorem must be quantified per byte-owner and per level.
//! The claim is right and it was not compiled. This compiles it, because a
//! structural theorem's counterexample is worth more as a program than as a
//! sentence.
//!
//! The column is `Layout::Bitpacked` at stored width 5, per the ratified group
//! arithmetic P = 8/gcd(W_S, 8) = 8 elements in G = W_S*P/8 = 5 bytes
//! (91:566-569). Twelve elements: one whole group of eight (bits 0..40) plus a
//! partial tail of four (bits 40..60), leaving bits 60..64 as container padding
//! at column granularity, which is exactly the region 91:568-569 calls "a
//! partial tail group's bits ... container padding at column granularity,
//! canonicalised by the packer's own pure constructor".
//!
//! There is no `unsafe` in this file, no niche, no transmute, and no
//! per-element accessor below the fields' width. The theorem as worded
//! (91:612-615, "the safe surface never exposes a raw accessor below the
//! fields' own width") is satisfied, and the gap is open anyway.
//!
//! Build: rustc --edition 2021 -O probe_2_*.rs -o out/probe_2 && ./out/probe_2

const W_S: u32 = 5;
const N: usize = 12;
const USED_BITS: u32 = W_S * N as u32; // 60
const BYTES: usize = 8; // ceil(60/8)

#[derive(Clone)]
struct Column {
    bytes: [u8; BYTES],
}

impl Column {
    /// The packer's own pure constructor. Canonicalises every bit outside the
    /// used extent by construction, because it starts from zero.
    fn pack(values: [u8; N]) -> Self {
        let mut bytes = [0u8; BYTES];
        for (i, &v) in values.iter().enumerate() {
            let bit = i as u32 * W_S;
            let masked = (v as u64) & ((1u64 << W_S) - 1);
            let mut word = u64::from_le_bytes(bytes);
            word |= masked << bit;
            bytes = word.to_le_bytes();
        }
        Column { bytes }
    }

    fn get(&self, i: usize) -> u8 {
        let bit = i as u32 * W_S;
        let word = u64::from_le_bytes(self.bytes);
        ((word >> bit) & ((1u64 << W_S) - 1)) as u8
    }

    /// The free raw-buffer digest shortcut (91:645-648): hash the contiguous
    /// bytes directly, at zero per-element cost, because every padding bit is
    /// canonical by construction along any safe path.
    fn digest(&self) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &b in self.bytes.iter() {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        h
    }

    /// The tempting API. Safe, no accessor below the fields' width, no
    /// per-element door, and it owns the whole level's bytes.
    fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

fn main() {
    let values: [u8; N] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let fresh = Column::pack(values);
    let mut dirtied = Column::pack(values);

    // One safe write, into bit 62, which is inside the tail group's padding
    // region [USED_BITS, 64) and outside every element's extent.
    assert!(USED_BITS == 60);
    dirtied.bytes_mut()[7] |= 0b0100_0000;

    // Every value-keyed and datum-keyed per-element read is unchanged.
    let mut reads_agree = true;
    for i in 0..N {
        if fresh.get(i) != dirtied.get(i) {
            reads_agree = false;
        }
    }

    println!(
        "W_S={W_S} N={N} used_bits={USED_BITS} bytes={BYTES} tail_padding_bits={}",
        BYTES as u32 * 8 - USED_BITS
    );
    println!("unsafe blocks in this file: 0");
    println!("per-element reads agree after the write: {reads_agree}");
    println!("fresh   digest: {:#018x}", fresh.digest());
    println!("dirtied digest: {:#018x}", dirtied.digest());
    println!("digests agree: {}", fresh.digest() == dirtied.digest());

    assert!(reads_agree, "every element still reads correctly");
    assert_ne!(
        fresh.digest(),
        dirtied.digest(),
        "the raw-buffer shortcut decorrelates from a fresh construction of the identical values"
    );

    // And the same write is unreachable once the level's own byte owner keeps
    // its granule: with `bytes_mut` withdrawn there is no safe expression that
    // reaches bit 62 at all, which is the theorem holding once its domain names
    // the column rather than the element.
    println!("theorem as worded (no raw accessor below the FIELDS' width): satisfied by this type");
    println!(
        "theorem quantified per byte-owner (no accessor below the COLUMN's write granule): violated by bytes_mut"
    );
}
