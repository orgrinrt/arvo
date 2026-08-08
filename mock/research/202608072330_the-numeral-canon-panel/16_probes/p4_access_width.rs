// p4: is the access width a third output, or a projection of the pair?
//
// Checks TWO things.
//   (a) The access width for a packed field is a function of W alone, so a site holding
//       (carrier, extent) can compute it and a site holding only the carrier cannot.
//       If that holds, three outputs is one too many and one is too few, in the same table.
//   (b) The truncation failure from p3 is DATA-DEPENDENT: a too-narrow access returns the
//       right answer whenever the value's high bits happen to be zero. This matters because
//       it means even the catching check can be made green by a test that uses small values.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p4_access_width.rs -o bin/p4 && ./bin/p4
//
// Spike.

const fn carrier_bits(w: u32) -> u32 {
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else if w <= 64 {
        64
    } else {
        128
    }
}

// The maximum number of bytes a W-bit field touches, over all bit phases 0..8.
// Derived rather than assumed: computed by exhausting the phases.
const fn span_bytes_max(w: u32) -> u32 {
    let mut p = 0;
    let mut worst = 0;
    while p < 8 {
        let first = p / 8;
        let last = (p + w - 1) / 8;
        let span = last - first + 1;
        if span > worst {
            worst = span;
        }
        p += 1;
    }
    worst
}

// And the closed form I claimed in the writeup, so the two can be compared.
const fn span_bytes_closed_form(w: u32) -> u32 {
    (w + 6) / 8 + 1
}

// The load a machine actually issues: round the span up to a power-of-two byte count.
const fn access_bits(w: u32) -> u32 {
    let b = span_bytes_max(w);
    let mut p = 1;
    while p < b {
        p *= 2;
    }
    p * 8
}

fn main() {
    // (a) the table.
    println!("W    carrier  stride(Cold)  span_bytes  access  carrier==access?");
    let mut disagree = 0;
    let mut total = 0;
    for w in 1..=64u32 {
        let c = carrier_bits(w);
        let a = access_bits(w);
        total += 1;
        if c != a {
            disagree += 1;
        }
        if matches!(
            w,
            1 | 3 | 5 | 7 | 8 | 9 | 13 | 16 | 17 | 23 | 25 | 31 | 32 | 33 | 47 | 57 | 64
        ) {
            println!(
                "{w:<4} u{c:<6} {w:<13} {:<11} u{a:<5} {}",
                span_bytes_max(w),
                if c == a { "yes" } else { "NO" }
            );
        }
    }
    println!();
    println!(
        "across W = 1..=64: carrier and access width disagree for {disagree} of {total} widths"
    );

    // the closed form, checked against the exhaustive one rather than asserted
    let mut mismatch = 0;
    for w in 1..=1024u32 {
        if span_bytes_max(w) != span_bytes_closed_form(w) {
            mismatch += 1;
        }
    }
    println!("closed form floor((W+6)/8)+1 vs exhaustive phase scan, W = 1..=1024: {mismatch} mismatches");

    // is access recoverable from the carrier alone? it is recoverable iff every W sharing a
    // carrier shares an access width.
    println!();
    println!("recoverability, checked rather than argued:");
    for cb in [8u32, 16, 32, 64] {
        let ws: Vec<u32> = (1..=64).filter(|&w| carrier_bits(w) == cb).collect();
        let mut accs: Vec<u32> = ws.iter().map(|&w| access_bits(w)).collect();
        accs.sort_unstable();
        accs.dedup();
        println!(
            "  from carrier u{cb:<4}: the {} widths behind it need access widths {:?} -> {}",
            ws.len(),
            accs,
            if accs.len() == 1 {
                "recoverable"
            } else {
                "NOT recoverable"
            }
        );
    }
    println!(
        "  from the extent  : W is carried directly, so access = f(W) is recoverable for every W"
    );

    // (b) data dependence of the truncation failure.
    println!();
    println!("(b) a too-narrow access is wrong only when the truncated bits are non-zero.");
    const W: u32 = 13;
    const MASK: u32 = (1 << W) - 1;
    for (label, gen) in [
        ("values 0..64 (a small-value test)", 0u32),
        ("values with the top bit set", 1u32),
        ("all bits set", 2u32),
    ] {
        let mut wrong = 0;
        let mut checked = 0;
        for k in 0..64usize {
            let v = match gen {
                0 => k as u32,
                1 => (k as u32) | (1 << (W - 1)),
                _ => MASK,
            };
            // simulate: write at bit 13k, read back with a 2-byte access
            let bit = k * W as usize;
            let phase = bit % 8;
            let visible = if phase + W as usize <= 16 {
                W
            } else {
                (16 - phase) as u32
            };
            let got = v & ((1u32 << visible) - 1);
            checked += 1;
            if got != v {
                wrong += 1;
            }
        }
        println!("  {label:<34} -> {wrong} of {checked} wrong with a 2-byte access");
    }
    println!();
    println!("so the catching check is itself blind to a test that uses small values.");
    println!("only a test whose data fills the declared width exercises the truncation.");
}
