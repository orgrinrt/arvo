// p2: under Reading A the site holds only the derivation's outputs. Does it then hold enough to
// do fixed-point arithmetic at all?
//
// 16:100-101 clause one excludes the declared width from being an output ("the consumer did not
// write it"). 16:572-577 applies exactly that to demote EXTENT_BITS. Under Reading A the site
// therefore never holds W, and this file counts how often W is recoverable from what it does
// hold, over the whole swept box.
//
// This is not an argument about which reading is intended. It is the exhaustive cost of one of
// them, so the cost is on the record rather than assumed.
//
// Model, and every line of it is a modelling choice a reader may reject:
//   - the carrier is the smallest native rung that holds W (8/16/32/64/128), which is 15:317-319's
//     and 16 section 2's shared finding that a lone value has the same carrier under every strategy;
//   - Cold's stride is W, every other strategy's is 8*size_of(carrier) (16:153-157);
//   - Precise is modelled as not widening, which is the reading that keeps the box smallest. The
//     widening reading only adds collisions, so this count is a lower bound under it.
//   - for W <= 128 every carrier is a native integer, whose alignment equals its size, so
//     identifying a carrier by its size is faithful in this range and is NOT faithful above it
//     (45_probes/p1, 47_probes/p5: at the wide rung two carriers share a size and differ in align).
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p2_reading_a_loses_the_width.rs -o bin/p2 && ./bin/p2
//
// No #![feature] gate.

#![no_std]
extern crate std;
use std::collections::{BTreeMap, BTreeSet};
use std::println;
use std::vec::Vec;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Strat {
    Hot,
    Warm,
    Cold,
    Precise,
}

const STRATS: [Strat; 4] = [Strat::Hot, Strat::Warm, Strat::Cold, Strat::Precise];

const fn native_rung(w: u32) -> u32 {
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

// carrier identified by its bit size. faithful for W <= 128 (all native), see header.
const fn carrier_bits(w: u32) -> u32 {
    native_rung(w)
}

fn stride_bits(w: u32, s: Strat) -> u32 {
    match s {
        Strat::Cold => w,
        _ => carrier_bits(w),
    }
}

// the native rungs really are the sizes of real Rust types, asserted rather than assumed.
const _: () = assert!(core::mem::size_of::<u8>() * 8 == 8);
const _: () = assert!(core::mem::size_of::<u16>() * 8 == 16);
const _: () = assert!(core::mem::size_of::<u32>() * 8 == 32);
const _: () = assert!(core::mem::size_of::<u64>() * 8 == 64);
const _: () = assert!(core::mem::size_of::<u128>() * 8 == 128);
// and for these, align == size, which is what makes size a faithful carrier identity below 129 bits.
const _: () = assert!(core::mem::align_of::<u16>() == core::mem::size_of::<u16>());
const _: () = assert!(core::mem::align_of::<u64>() == core::mem::size_of::<u64>());

fn main() {
    // (carrier_bits, stride_bits) -> the set of declared widths that land there
    let mut classes: BTreeMap<(u32, u32), BTreeSet<u32>> = BTreeMap::new();
    // sign is not a key here: for W <= 128 the container rung is the same for both signs, and the
    // sign is a declaration input in every reading. it multiplies every count below by two and
    // changes no ratio, so it is left out and said so.
    for w in 1..=128u32 {
        for s in STRATS {
            classes
                .entry((carrier_bits(w), stride_bits(w, s)))
                .or_default()
                .insert(w);
        }
    }

    let decls: u32 = 128 * 4;
    // count DECLARATIONS, not classes: a declaration (w, s) has its width recoverable exactly when
    // the class it lands in carries no other width.
    let mut recoverable = 0usize;
    let mut lost = 0usize;
    for w in 1..=128u32 {
        for s in STRATS {
            let key = (carrier_bits(w), stride_bits(w, s));
            if classes[&key].len() == 1 {
                recoverable += 1;
            } else {
                lost += 1;
            }
        }
    }

    println!("Reading A: a site holds only the derivation's outputs (carrier, stride).");
    println!();
    println!(
        "swept box: widths 1..=128, four strategies, one sign  =  {} declarations",
        decls
    );
    println!(
        "distinct (carrier, stride) classes                    =  {}",
        classes.len()
    );
    println!(
        "classes carrying more than one declared width         =  {}",
        classes.values().filter(|ws| ws.len() > 1).count()
    );
    println!(
        "declarations whose W IS recoverable from the pair     =  {} of {}",
        recoverable, decls
    );
    println!(
        "declarations whose W is NOT recoverable from the pair =  {} of {}  ({:.1}%)",
        lost,
        decls,
        100.0 * lost as f64 / decls as f64
    );
    println!();
    println!("the largest ambiguous classes:");
    let mut big: Vec<_> = classes.iter().filter(|(_, ws)| ws.len() > 1).collect();
    big.sort_by_key(|(_, ws)| core::cmp::Reverse(ws.len()));
    for ((c, st), ws) in big.iter().take(5) {
        let lo = ws.iter().next().unwrap();
        let hi = ws.iter().next_back().unwrap();
        println!(
            "  carrier {:>3} bits, stride {:>3} bits  <-  {} widths, {}..={}",
            c,
            st,
            ws.len(),
            lo,
            hi
        );
    }
    println!();

    // the same question restricted to the packed strategy, where 16's own argument lives
    let mut cold: BTreeMap<(u32, u32), BTreeSet<u32>> = BTreeMap::new();
    for w in 1..=128u32 {
        cold.entry((carrier_bits(w), stride_bits(w, Strat::Cold)))
            .or_default()
            .insert(w);
    }
    println!(
        "restricted to Cold alone: {} classes over 128 widths, ambiguous classes {}",
        cold.len(),
        cold.values().filter(|ws| ws.len() > 1).count()
    );
    println!("  so W IS recoverable from the pair when the strategy packs at stride == W,");
    println!("  and that recovery is what the unpacked strategies destroy.");
    println!();

    // what the site needs W for, spelled out with one case
    println!("what a site that has lost W cannot then do, at one worked declaration:");
    let (w1, w2) = (13u32, 16u32);
    println!(
        "  UFixed<{},0,Warm> and UFixed<{},0,Warm> both present as (carrier {}, stride {})",
        w1,
        w2,
        carrier_bits(w1),
        stride_bits(w1, Strat::Warm)
    );
    println!("  a multiply must shift right by F and mask to W bits. both are functions of the");
    println!("  declaration, and neither is a function of (carrier, stride). so under Reading A");
    println!("  the criterion's clause one has excluded a fact the machine needs and the site");
    println!("  cannot reach. that is not a count being wrong. it is the reading being unsound.");
}
