// Probe 03: double rounding under Narrowed growth, and the round-to-odd cure.
//
// Under `Growth::Narrowed` a product is quantised twice: once when the wide
// exact intermediate is narrowed to W fractional bits, once when the result
// lands at the numeral's F. Rounding to nearest twice is not rounding to
// nearest once: a value just below a final-precision midpoint can be pushed
// exactly onto it by the first rounding, and the second then resolves a tie
// that the exact value never presented. The classical cure (Boldo-Melquiond)
// is round-to-odd at the intermediate: it preserves "was strictly below /
// above / exactly on" every coarser midpoint, provided the intermediate
// carries at least two more fractional bits than the destination. The spec's
// own vocabulary already contains ToOdd; this measures what it buys.
//
// Model: signed products with 6 fractional bits, destination F = 2, so the
// narrowing drops 4 bits in one step (reference) or via an intermediate at
// W = 3 (one extra bit, the theorem's precondition VIOLATED) and W = 4
// (two extra bits, precondition met). Exhaustive over all products in
// [-2^13, 2^13).

fn rne(x: i64, drop: u32) -> i64 {
    if drop == 0 {
        return x;
    }
    let half = 1i64 << (drop - 1);
    let fl = x >> drop;
    let rem = x - (fl << drop);
    if rem > half {
        fl + 1
    } else if rem < half {
        fl
    } else if fl & 1 == 0 {
        fl
    } else {
        fl + 1
    }
}

fn rodd(x: i64, drop: u32) -> i64 {
    if drop == 0 {
        return x;
    }
    let fl = x >> drop;
    let rem = x - (fl << drop);
    if rem == 0 {
        fl
    } else if fl & 1 == 1 {
        fl // already odd: sticky info preserved in the odd lsb
    } else {
        fl + 1 // force odd
    }
}

fn main() {
    // fractional-bit ledger: value has 6, intermediate keeps W, final keeps 2.
    for w in [3u32, 4] {
        let first_drop = 6 - w; // wide -> intermediate
        let second_drop = w - 2; // intermediate -> final
        let mut mism_rne = 0u64;
        let mut mism_rodd = 0u64;
        let mut total = 0u64;
        for x in -(1i64 << 13)..(1i64 << 13) {
            let direct = rne(x, 4);
            let two_rne = rne(rne(x, first_drop), second_drop);
            let two_rodd = rne(rodd(x, first_drop), second_drop);
            if two_rne != direct {
                mism_rne += 1;
            }
            if two_rodd != direct {
                mism_rodd += 1;
            }
            total += 1;
        }
        println!(
            "W = {w} ({} extra bit(s)): rne-then-rne mismatches {mism_rne}/{total}, odd-then-rne mismatches {mism_rodd}/{total}",
            w - 2
        );
    }
    println!();
    println!("rne twice disagrees with rne once; round-to-odd at the intermediate");
    println!("restores exact agreement, but only once the intermediate carries at");
    println!("least two more fractional bits than the destination.");
}
