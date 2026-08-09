// Probe 4: a whole-column digest is a fold, but not an instance of the numeral fold's own
// interior/total-safety machinery (there is no quantiser in a hash combine: it never rounds,
// never leaves a value set, so 40:328-345's two conditions have no subject here). What it does
// need, for the same underlying reason morsel-parallel dispatch needs anything, is its own
// grouping-invariance property: a column digest computed by partitioning into morsels and
// combining partial results must equal the digest computed as one sequential fold, for every
// partition. A naive chained running hash does not have this property. A positional (polynomial)
// combine does, and the associativity-under-a-positional-shift it needs is the identical shape
// to the multiplicative half's own exponent-offset equivariance (68 section 1.9), applied to a
// different operation. Neither construction here is a proposed shipping algorithm; both are
// models built to test the property, exactly as the review's other digest probes model FNV-1a
// rather than propose it as the hash family.

fn fnv1a_step(h: u64, byte: u8) -> u64 {
    (h ^ byte as u64).wrapping_mul(0x100000001b3)
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h = fnv1a_step(h, b);
    }
    h
}

const N: usize = 64;

fn column() -> [u16; N] {
    let mut c = [0u16; N];
    for i in 0..N {
        c[i] = ((i as u32).wrapping_mul(2654435761) & 0x1FFF) as u16; // 13-bit field values
    }
    c
}

// --- naive: a chained running hash, sequential state, no defined combine of two partial states
// other than re-feeding one partial's bytes into the other, which is not symmetric and not what
// a real parallel dispatch would compute (each morsel produces an INDEPENDENT partial digest of
// its own slice; nothing re-feeds one morsel's raw bytes through the other's hasher).
fn naive_sequential(col: &[u16]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &v in col {
        for b in v.to_le_bytes() {
            h = fnv1a_step(h, b);
        }
    }
    h
}

// the morsel-parallel shape a naive scheme is actually asked to support: each morsel computes
// its own independent digest (as if it were the whole column), then partials are combined.
// there is no combine operator for fnv1a partials that reproduces the sequential result; the
// only thing available is fnv1a-of-the-two-digests, tested and shown wrong below.
fn naive_morsel_then_combine(col: &[u16], split: usize) -> u64 {
    let left = naive_sequential(&col[..split]);
    let right = naive_sequential(&col[split..]);
    let mut buf = [0u8; 16];
    buf[0..8].copy_from_slice(&left.to_le_bytes());
    buf[8..16].copy_from_slice(&right.to_le_bytes());
    fnv1a(&buf)
}

// --- positional (polynomial) combine: digest = sum_i (elem_i as u64) * B^i, wrapping u64.
// a morsel starting at offset k computes its own partial with LOCAL weights (B^0, B^1, ...) and
// the caller rescales by B^k before combining, exactly the multiplicative half's own
// exponent-offset shift (a product's equivariant home is a window shifted by the offset, applied
// here to a positional weight rather than to mulnum's exponent).
const B: u64 = 0x9E3779B97F4A7C15; // an odd 64-bit constant, invertible mod 2^64

fn pow_b(k: usize) -> u64 {
    let mut acc: u64 = 1;
    let mut base = B;
    let mut e = k;
    while e > 0 {
        if e & 1 == 1 {
            acc = acc.wrapping_mul(base);
        }
        base = base.wrapping_mul(base);
        e >>= 1;
    }
    acc
}

fn positional_partial(slice: &[u16]) -> u64 {
    let mut acc: u64 = 0;
    let mut weight: u64 = 1;
    for &v in slice {
        acc = acc.wrapping_add((v as u64).wrapping_mul(weight));
        weight = weight.wrapping_mul(B);
    }
    acc
}

fn positional_sequential(col: &[u16]) -> u64 {
    positional_partial(col)
}

fn positional_morsel_then_combine(col: &[u16], split: usize) -> u64 {
    let left = positional_partial(&col[..split]);
    let right = positional_partial(&col[split..]);
    left.wrapping_add(right.wrapping_mul(pow_b(split)))
}

fn main() {
    let col = column();

    // grouping invariance: naive fails
    let direct_naive = naive_sequential(&col);
    let mut naive_agrees_everywhere = true;
    for split in [0usize, 1, 16, 32, 48, 63, 64] {
        let combined = naive_morsel_then_combine(&col, split);
        if split != 0 && split != N && combined == direct_naive {
            naive_agrees_everywhere = false; // would be surprising; record if it ever happens
        }
    }
    // explicit witness at a genuine split
    assert_ne!(
        naive_sequential(&col),
        naive_morsel_then_combine(&col, 32),
        "naive chained hash is not grouping-invariant: splitting into two morsels and combining does not reproduce the sequential result"
    );
    let _ = naive_agrees_everywhere;

    // grouping invariance: positional combine holds at every split, including the degenerate ones
    let direct_positional = positional_sequential(&col);
    for split in [0usize, 1, 16, 32, 48, 63, 64] {
        assert_eq!(
            positional_morsel_then_combine(&col, split),
            direct_positional,
            "positional combine must reproduce the sequential digest at split point {split}"
        );
    }

    // order sensitivity: the property that must NOT be lost. Swap two elements and confirm both
    // constructions change (grouping invariance is not the same property as order invariance;
    // a column digest wants the first and must not accidentally acquire the second).
    let mut swapped = col;
    swapped.swap(3, 40);
    assert_ne!(
        positional_sequential(&col),
        positional_sequential(&swapped),
        "positional combine stays order-sensitive: a permutation is a detected change, not masked by associativity"
    );
    assert_ne!(
        naive_sequential(&col),
        naive_sequential(&swapped),
        "naive combine is also order-sensitive (it was never in question); only grouping invariance distinguishes the two constructions"
    );

    println!("naive sequential fold: order-sensitive but NOT grouping-invariant (morsel-then-combine diverges from the direct fold)");
    println!("positional combine: order-sensitive AND grouping-invariant at every tested split, by the identical exponent-offset shift argument the multiplicative half already compiled at 68 section 1.9");
}
