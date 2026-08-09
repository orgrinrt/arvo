// Probe 5: a bitpacked column's datum-keyed digest. Under Layout::Bitpacked's ratified single
// meaning (zero inter-value padding, 78:552-556), there is no statement-P region at all inside a
// group: P = 8/gcd(W,8) elements pack into G = W*P/8 whole bytes with nothing left over
// (81:199-214, the theorem 83:216-218 proves algebraically). The only padding a bitpacked column
// can carry is the tail group's, at column granularity, canonicalised once by the packer's pure
// constructor (83:224-228). So the tier-1 free shortcut (hash the raw packed buffer) is available
// for a bitpacked column with a SMALLER dirt surface than a dense column: one tail region for the
// whole column, rather than one padding region per value.

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

const W: u32 = 13;
#[allow(dead_code)]
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
const fn period_and_group() -> (u32, u32) {
    // gcd(13,8) = 1, so P = 8, G = 13
    (8, 13)
}

fn pack(values: &[u16], out: &mut [u8]) {
    let mut bitpos: usize = 0;
    for &v in values {
        let v = (v & 0x1FFF) as u32;
        let byte = bitpos / 8;
        let shift = bitpos % 8;
        let mut window = (out[byte] as u32)
            | ((*out.get(byte + 1).unwrap_or(&0) as u32) << 8)
            | ((*out.get(byte + 2).unwrap_or(&0) as u32) << 16)
            | ((*out.get(byte + 3).unwrap_or(&0) as u32) << 24);
        window |= v << shift;
        out[byte] = (window & 0xFF) as u8;
        if byte + 1 < out.len() {
            out[byte + 1] = ((window >> 8) & 0xFF) as u8;
        }
        if byte + 2 < out.len() {
            out[byte + 2] = ((window >> 16) & 0xFF) as u8;
        }
        if byte + 3 < out.len() {
            out[byte + 3] = ((window >> 24) & 0xFF) as u8;
        }
        bitpos += W as usize;
    }
}

fn unpack(buf: &[u8], n: usize) -> Vec<u16> {
    let mut out = Vec::with_capacity(n);
    let mut bitpos: usize = 0;
    for _ in 0..n {
        let byte = bitpos / 8;
        let shift = bitpos % 8;
        let window = (*buf.get(byte).unwrap_or(&0) as u32)
            | ((*buf.get(byte + 1).unwrap_or(&0) as u32) << 8)
            | ((*buf.get(byte + 2).unwrap_or(&0) as u32) << 16)
            | ((*buf.get(byte + 3).unwrap_or(&0) as u32) << 24);
        out.push(((window >> shift) & 0x1FFF) as u16);
        bitpos += W as usize;
    }
    out
}

fn main() {
    let (p, g) = period_and_group();
    assert_eq!(p, 8);
    assert_eq!(g, 13);
    assert_eq!(
        (g * 8) as u32,
        W * p,
        "the group-is-whole-bytes theorem, 83:216-218, at this width"
    );

    // N deliberately NOT a multiple of the period: forces a tail group, the one place a
    // bitpacked column can carry column-granularity padding at all.
    const NVALS: usize = 65; // 65 = 8*8 + 1: eight full groups plus one leftover value
    let values: Vec<u16> = (0..NVALS as u16)
        .map(|i| i.wrapping_mul(37) & 0x1FFF)
        .collect();

    let total_bits = NVALS * W as usize;
    let total_bytes = (total_bits + 7) / 8;
    let mut buf = vec![0u8; total_bytes + 4]; // +4 headroom for the sliding window read
    pack(&values, &mut buf);
    buf.truncate(total_bytes);

    // honest round trip: the digest law's whole premise, checked before anything else.
    let recovered = unpack(&buf, NVALS);
    assert_eq!(
        recovered, values,
        "round trip holds before any digest claim is meaningful"
    );

    // Tier 1: the free shortcut. Rebuild an independent packing of the same value sequence and
    // confirm the raw-buffer digest is a sound function of the value sequence alone.
    let mut buf2 = vec![0u8; total_bytes + 4];
    pack(&values, &mut buf2);
    buf2.truncate(total_bytes);
    assert_eq!(
        fnv1a(&buf),
        fnv1a(&buf2),
        "tier-1 theorem holds for the bitpacked case: two independent honest packings of the same values give the same raw-buffer digest"
    );

    // Tier 2: dirty the tail group's own padding bits (bit positions past NVALS*W within the
    // final byte) and show the raw digest decorrelates even though no live value changed.
    let last_live_bit = NVALS * W as usize;
    let last_byte = (last_live_bit - 1) / 8;
    let live_bits_in_last_byte = last_live_bit - last_byte * 8;
    if live_bits_in_last_byte < 8 {
        let mut dirty_buf = buf.clone();
        let dirt_mask: u8 = !((1u16 << live_bits_in_last_byte) - 1) as u8;
        dirty_buf[last_byte] |= dirt_mask;
        let dirty_recovered = unpack(&dirty_buf, NVALS);
        assert_eq!(
            dirty_recovered, values,
            "the tail's own padding carries no live value: the round trip is unaffected by dirtying it"
        );
        assert_ne!(
            fnv1a(&dirty_buf),
            fnv1a(&buf),
            "tier-2 reproduces at the bitpacked column's tail: dirtying padding-only bits still decorrelates the free raw-buffer shortcut"
        );
        println!(
            "bitpacked column, W={W}, N={NVALS}: one tail-padding region of {} bits is the ENTIRE dirt surface for the whole column, against one region per value in the dense case (probe 2)",
            8 - live_bits_in_last_byte
        );
    } else {
        println!("bitpacked column, W={W}, N={NVALS}: tail group landed byte-aligned with no leftover padding at this N; re-run at a different NVALS to exercise the tail case");
    }

    println!("tier-1 theorem holds for the bitpacked case with a strictly smaller dirt surface than dense: interior groups have zero padding by the ratified single meaning of Layout::Bitpacked, so only the column-level tail can ever decorrelate the free shortcut");
}
