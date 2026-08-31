// Probe 5, seat 225. The selector-against-key disagreement is about factoring one
// map, and no declared signature separates the two factorings.
//
// The standing disagreement (register, "The derivation's outputs"): file 10 has the
// strategy upstream, supplying crossover parameters to a ladder that maps width to
// container; file 15 keys the map on (strategy, width, sign) directly. Both compile.
// This probe asks the question the dissolution makes primary: do the two spellings
// denote one assignment, and does any declared observation separate them?
//
// The model realisation rule is scaffolding (a spike's incidental choices are not
// decisions): hot takes the minimum byte-aligned rung and pads wide payloads to a
// 16-byte alignment past 128 bits; warm doubles the logical width; cold takes the
// minimum rung; precise doubles. What is under test is not the rule but the
// factoring: `keyed` implements it as one match on (strategy, width); `selector`
// implements it as params(strategy) fed to a strategy-blind ladder. If the two
// agree pointwise over the whole box, they are one design under the ratified
// identity clause, because the interior factoring is reachable through no declared
// signature.
//
// Arms:
//   arm A (measurement): keyed and selector agree on (carrier bytes, sole-occupancy
//     stride) at every cell of S x W in 1..=256 x sign in {u,s}, exhaustively.
//   arm B (occupancy split): at shared occupancy the aggregate extent is not
//     k * size_of(carrier) almost everywhere, so the per-aggregate fact is
//     independent exactly there. Counted over W in 1..=16, k in 1..=8.
//   arm C (negative control, MUST FAIL): a variant of the keyed spelling carrying
//     one exceptional cell (a different rung at exactly one (strategy, width)) is
//     asserted equal to the selector spelling. The sweep must catch it, or the
//     instrument cannot distinguish factorings from assignments at all.
//   arm D (negative control, MUST FAIL): asserts shared extent equals
//     k * carrier bytes at W = 13, k = 5. Fails 9 against 10: the packing fact is
//     not recoverable from the carrier, which is the ownership clause's test for
//     a carried fact.
//
// holds for (arm A): the model rule, S in {hot, warm, cold, precise},
// W in 1..=256, signedness in {unsigned, signed}, threads = 1, toolchain in
// toolchain.txt. Arm B: W in 1..=16, k in 1..=8, container = dense bit stream
// against minimum rung. The claim that the REAL design's two spellings agree is
// not established here; what is established is the shape of the check that
// decides it, with both controls shown firing.

#[derive(Clone, Copy, PartialEq, Debug)]
enum S {
    Hot,
    Warm,
    Cold,
    Precise,
}
const ALL: [S; 4] = [S::Hot, S::Warm, S::Cold, S::Precise];

fn rung_bytes(bits: u32) -> u32 {
    // minimum byte-aligned rung: 1, 2, 4, 8, 16, 32 bytes
    let mut b = 1u32;
    while b * 8 < bits {
        b *= 2;
    }
    b
}

// --- the keyed spelling: one match on (strategy, width) -----------------------
fn keyed(s: S, w: u32) -> (u32, u32) {
    let (bits, min_align) = match s {
        S::Hot => (w, if w > 128 { 16 } else { 1 }),
        S::Warm => (2 * w, 1),
        S::Cold => (w, 1),
        S::Precise => (2 * w, 1),
    };
    let c = rung_bytes(bits).max(min_align);
    (c, c) // (carrier bytes, sole-occupancy stride = size_of(carrier))
}

// --- the selector spelling: params(strategy) into a strategy-blind ladder ------
struct Params {
    mult: u32,
    min_align: u32,
    wide_from: u32,
}
fn params(s: S) -> Params {
    match s {
        S::Hot => Params {
            mult: 1,
            min_align: 16,
            wide_from: 129,
        },
        S::Warm => Params {
            mult: 2,
            min_align: 1,
            wide_from: u32::MAX,
        },
        S::Cold => Params {
            mult: 1,
            min_align: 1,
            wide_from: u32::MAX,
        },
        S::Precise => Params {
            mult: 2,
            min_align: 1,
            wide_from: u32::MAX,
        },
    }
}
fn ladder(p: &Params, w: u32) -> (u32, u32) {
    let align = if w >= p.wide_from { p.min_align } else { 1 };
    let c = rung_bytes(p.mult * w).max(align);
    (c, c)
}
fn selector(s: S, w: u32) -> (u32, u32) {
    ladder(&params(s), w)
}

// --- the exceptional-cell mutant for arm C ------------------------------------
fn keyed_mutant(s: S, w: u32) -> (u32, u32) {
    if s == S::Cold && w == 24 {
        return (8, 8);
    } // one cell nobody's params express
    keyed(s, w)
}

fn shared_extent_bytes(w: u32, k: u32) -> u32 {
    (w * k + 7) / 8
}

fn main() {
    println!("arm A: keyed and selector agree pointwise over the whole box");
    let mut cells = 0u32;
    for s in ALL {
        for w in 1..=256u32 {
            // signedness enters the model rule nowhere, which the sweep makes
            // explicit by running both values through one path
            for _sign in 0..2 {
                cells += 1;
                let (a, b) = (keyed(s, w), selector(s, w));
                if a != b {
                    println!("  DISAGREE at {s:?} W={w}: keyed {a:?} selector {b:?}");
                    println!("  (if this fires the two spellings were two assignments, not two factorings)");
                    std::process::exit(2);
                }
            }
        }
    }
    println!("  {cells} cells, zero disagreements: one map, two spellings");

    println!();
    println!("arm B: at shared occupancy the extent is not k * carrier bytes");
    let mut differ = 0u32;
    let mut total = 0u32;
    for w in 1..=16u32 {
        for k in 1..=8u32 {
            total += 1;
            let (c, _) = keyed(S::Cold, w);
            if shared_extent_bytes(w, k) != k * c {
                differ += 1;
            }
        }
    }
    println!("  {differ} of {total} cells differ, so the aggregate fact is not a function of the carrier there");
    if differ == 0 {
        println!("  UNEXPECTED: box too small to show the split");
        std::process::exit(2);
    }

    println!();
    println!("arm C (negative control, MUST FAIL): the mutant with one exceptional cell");
    let mut caught = None;
    'outer: for s in ALL {
        for w in 1..=256u32 {
            if keyed_mutant(s, w) != selector(s, w) {
                caught = Some((s, w));
                break 'outer;
            }
        }
    }
    match caught {
        Some((s, w)) => println!(
            "  FAILED AS REQUIRED: first witness at {s:?} W={w}: the sweep separates assignments"
        ),
        None => {
            println!("  UNEXPECTED PASS: instrument cannot see an exceptional cell");
            std::process::exit(2);
        }
    }

    println!();
    println!(
        "arm D (negative control, MUST FAIL): shared extent == k * carrier bytes at W=13, k=5"
    );
    let (c, _) = keyed(S::Cold, 13);
    let ext = shared_extent_bytes(13, 5);
    if ext == 5 * c {
        println!("  UNEXPECTED PASS");
        std::process::exit(2);
    }
    println!(
        "  FAILED AS REQUIRED: extent {ext} bytes against k * carrier = {}",
        5 * c
    );
    println!("  producing the packed extent applies the strategy's packing rule, so the");
    println!("  ownership clause carries it; the sole-occupancy stride is size_of(carrier)");
    println!("  and is recomputed.");
}
