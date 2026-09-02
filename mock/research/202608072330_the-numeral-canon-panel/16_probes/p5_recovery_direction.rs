// p5: could the derivation emit only the EXTENT and recompute the carrier at each use?
//
// This is the strongest objection to the pair, because for most rungs the carrier is
// next_pow2(max(W,8)) and W is carried by the extent, so the arithmetic works.
//
// Checks TWO things.
//   (a) Whether the map extent -> carrier is a function at all, under a strategy that widens
//       compute past storage. If it is not, the pair is irreducible in both directions.
//   (b) Whether "recoverable by arithmetic" is the same as "available at the type level".
//       The naive const-to-type recovery is attempted below and the diagnostic is recorded.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p5_recovery_direction.rs -o bin/p5 && ./bin/p5
//
// Part (b)'s failing form is in the sibling file p5b_const_to_type.rs, which does NOT compile
// on purpose. A test that cannot compile is the finding, so it is kept in its failing state and
// its diagnostic is committed beside it.
//
// Spike.

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Strat {
    Hot,
    Warm,
    Cold,
    Precise,
}

const fn storage_bits(w: u32, s: Strat) -> u32 {
    match s {
        Strat::Cold => w,
        _ => native(w),
    }
}

const fn native(w: u32) -> u32 {
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

// Two readings of Precise, because I do not know which one the design takes and will not
// reason from mock/crates. Both are modelled and the consequence of each is reported.
const fn carrier_bits(w: u32, s: Strat, precise_widens: bool) -> u32 {
    match s {
        // reading A: Precise widens the COMPUTE type past the storage type to keep
        // intermediates exact.
        Strat::Precise if precise_widens => {
            let n = native(w);
            if n < 128 {
                n * 2
            } else {
                128
            }
        }
        _ => native(w),
    }
}

fn main() {
    for precise_widens in [false, true] {
        println!(
            "=== reading {}: Precise {} the compute type past storage ===",
            if precise_widens { "A" } else { "B" },
            if precise_widens {
                "DOES widen"
            } else {
                "does NOT widen"
            }
        );

        // (a) is extent -> carrier a function? it is iff every declaration with the same
        //     (storage width, stride) has the same carrier.
        //     extent modelled as (W, stride).
        let mut collisions: Vec<((u32, u32), Vec<(u32, Strat, u32)>)> = Vec::new();
        for w in 1..=128u32 {
            for s in [Strat::Hot, Strat::Warm, Strat::Cold, Strat::Precise] {
                let extent = (w, storage_bits(w, s));
                let carrier = carrier_bits(w, s, precise_widens);
                if let Some(e) = collisions.iter_mut().find(|(k, _)| *k == extent) {
                    e.1.push((w, s, carrier));
                } else {
                    collisions.push((extent, vec![(w, s, carrier)]));
                }
            }
        }

        let mut bad = 0;
        let mut example: Option<String> = None;
        for (extent, members) in &collisions {
            let mut cs: Vec<u32> = members.iter().map(|m| m.2).collect();
            cs.sort_unstable();
            cs.dedup();
            if cs.len() > 1 {
                bad += 1;
                if example.is_none() {
                    example = Some(format!(
                        "extent (W={}, stride={}) is shared by {:?}, which need carriers {:?}",
                        extent.0,
                        extent.1,
                        members.iter().map(|m| m.1).collect::<Vec<_>>(),
                        cs
                    ));
                }
            }
        }
        println!("  distinct extents: {}", collisions.len());
        println!("  extents mapping to more than one carrier: {bad}");
        if let Some(e) = example {
            println!("  witness: {e}");
            println!(
                "  -> extent -> carrier is NOT a function. the pair is irreducible both ways."
            );
        } else {
            println!("  -> extent -> carrier IS a function under this reading.");
            println!(
                "     so the objection survives the arithmetic, and only part (b) answers it."
            );
        }
        println!();
    }

    println!("part (b) is in p5b_const_to_type.rs, which does not compile on purpose.");
    println!("see p5b_const_to_type.err for the diagnostic.");
}
