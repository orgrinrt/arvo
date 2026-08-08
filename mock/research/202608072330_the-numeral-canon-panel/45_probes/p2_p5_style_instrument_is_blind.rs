// p2: does `16`'s own p5 instrument (carrier represented as a bare bit count, `u32`) report
// the collision p1 just compiled, when swept over the SAME wide-rung domain?
//
// This is the direct check that p5's "0 of 251, extent -> carrier is a function absent
// Precise" claim is an artifact of two narrowings in its own model: (a) the sweep never
// reaches W > 128, and (b) even if it did, its `carrier_bits` return type (u32) cannot
// represent alignment, so it would report the wide-rung collision as "same carrier" when
// the two carriers are, per p1's const-checked witness, different TYPES.
//
// Reproduces 16_probes/p5_recovery_direction.rs's `carrier_bits`/`storage_bits` shape
// exactly (same match arms, same non-widening reading for Precise), extended only to
// sweep W up to 768 instead of stopping at 128, with no alignment tracked, matching p5's
// own representation.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p2_p5_style_instrument_is_blind.rs -o bin/p2 && ./bin/p2
//
// Spike.

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Strat {
    Hot,
    Warm,
    Cold,
    Precise,
}

// p5's own `native` and `storage_bits`, unchanged in shape, extended to a wider domain.
// p5 capped `native` at 128 for every width above 64; that is itself p5's second narrowing
// (it never models a native rung above 128, and it never models Hot's align-16 wide-rung
// padding at all). Reproduced here bit-for-bit so the comparison is fair, then contrasted
// against the type-aware `hot_bytes`/`warm_bytes` from p1.
const fn native_p5_style(w: u32) -> u32 {
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

const fn storage_bits_p5_style(w: u32, s: Strat) -> u32 {
    match s {
        Strat::Cold => w,
        _ => native_p5_style(w),
    }
}

// p5's carrier_bits, non-widening reading only (precise_widens = false), which is exactly
// the branch OPTIONS.md cites as "extent -> carrier IS a function".
const fn carrier_bits_p5_style(w: u32, s: Strat) -> u32 {
    match s {
        _ => native_p5_style(w),
    }
}

fn main() {
    println!("reproducing p5's OWN instrument (bit-count carrier, no alignment tracked),");
    println!("swept over W = 129..=768, the same domain p1 found 40 real collisions in.");
    println!();

    let mut collisions_bag: Vec<((u32, u32), Vec<(u32, Strat, u32)>)> = Vec::new();
    for w in 129u32..=768 {
        for s in [Strat::Hot, Strat::Warm, Strat::Cold, Strat::Precise] {
            let extent = (w, storage_bits_p5_style(w, s));
            let carrier = carrier_bits_p5_style(w, s);
            if let Some(e) = collisions_bag.iter_mut().find(|(k, _)| *k == extent) {
                e.1.push((w, s, carrier));
            } else {
                collisions_bag.push((extent, vec![(w, s, carrier)]));
            }
        }
    }

    let mut bad = 0;
    for (_extent, members) in &collisions_bag {
        let mut cs: Vec<u32> = members.iter().map(|m| m.2).collect();
        cs.sort_unstable();
        cs.dedup();
        if cs.len() > 1 {
            bad += 1;
        }
    }

    println!("p5-style instrument (carrier as bit count): extents mapping to more than one carrier value: {bad}");
    println!();
    println!("p5's own model conflates `native_p5_style` with 128 for every width above 64,");
    println!("so at W = 249..768 it reports a UNIFORM carrier bit count of 128 for Hot, Warm");
    println!("and Precise alike, never distinguishing the wide-rung align-16 case from the");
    println!("align-1 case at all, because its representation of a carrier is a number, not");
    println!("a type, and its own model of `native` never grows past 128 in the first place.");
    println!();
    println!("p1's type-aware instrument, same domain, found: 40 of 640 real collisions.");
    println!("this instrument, reproducing p5's own representation on the same domain: {bad}.");
    println!();
    println!(
        "0 versus 40 on the identical (W, strategy) domain is not a disagreement about facts;"
    );
    println!(
        "it is p5's own carrier representation being unable to express the fact that would"
    );
    println!(
        "refute its claim. this is the same blindness `16` itself diagnosed in section 5"
    );
    println!(
        "for the erasure-and-codegen-equality check (\"its instrument is a scalar... a"
    );
    println!(
        "carrier-only derivation passes it at full marks\"), now found in `16`'s OWN"
    );
    println!("instrument for the pair's irreducibility claim, in the same file.");
}
