// p1b. Route two: the operation supplied as a const generic VALUE, dispatched
// by a `match` inside the const fn.
//
// `109` section 4 concludes from the p1a wall that "a law cannot be computed at
// const time about an operation supplied as a value; the operation has to be a
// type." This probe is the direct test of that conclusion. The operation here is
// a value, a `u8` tag, not a type, and it is a const generic parameter so it is
// statically resolved.
//
// Expected: COMPILES, no `#![feature(...)]` line, and the two verdicts differ,
// which is what makes it a census rather than a constant.

const LO: i32 = -8;
const HI: i32 = 7;

const OP_SAT_BOTH: u8 = 0;
const OP_WRAP: u8 = 1;
const OP_SAT_TOP_ONLY: u8 = 2;

const fn apply<const OP: u8>(a: i32, b: i32) -> i32 {
    let s = a + b;
    match OP {
        OP_SAT_BOTH => {
            if s > HI {
                HI
            } else if s < LO {
                LO
            } else {
                s
            }
        }
        OP_WRAP => ((s - LO).rem_euclid(HI - LO + 1)) + LO,
        OP_SAT_TOP_ONLY => {
            if s > HI {
                HI
            } else {
                s
            }
        }
        _ => panic!("unknown operation tag"),
    }
}

/// Two censuses at once: how many triples associate, and how many pairs leave
/// the value set. The second is the closure question `109` found is prior to
/// the first, and it is computed here for the same reason: a verdict about
/// associativity is meaningless where the operation is not closed.
const fn census<const OP: u8>() -> (u32, u32, u32) {
    let mut assoc_fail = 0u32;
    let mut escapes = 0u32;
    let mut pairs = 0u32;

    let mut a = LO;
    while a <= HI {
        let mut b = LO;
        while b <= HI {
            let r = apply::<OP>(a, b);
            if r < LO || r > HI {
                escapes += 1;
            }
            pairs += 1;
            let mut c = LO;
            while c <= HI {
                if apply::<OP>(apply::<OP>(a, b), c) != apply::<OP>(a, apply::<OP>(b, c)) {
                    assoc_fail += 1;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    (assoc_fail, escapes, pairs)
}

const SAT_BOTH: (u32, u32, u32) = census::<OP_SAT_BOTH>();
const WRAP: (u32, u32, u32) = census::<OP_WRAP>();
const SAT_TOP: (u32, u32, u32) = census::<OP_SAT_TOP_ONLY>();

fn main() {
    println!("value set        : {LO}..={HI}, 4096 triples, 256 pairs");
    println!("operation carried as a const generic VALUE, not a type\n");
    println!("op              assoc-failures  escapes-from-set");
    println!("sat-both        {:>14}  {:>16}", SAT_BOTH.0, SAT_BOTH.1);
    println!("wrap            {:>14}  {:>16}", WRAP.0, WRAP.1);
    println!("sat-top-only    {:>14}  {:>16}", SAT_TOP.0, SAT_TOP.1);
    println!("\npairs enumerated per op: {}", SAT_BOTH.2);
    println!(
        "verdicts differ across ops: {}",
        SAT_BOTH.0 != WRAP.0 || WRAP.0 != SAT_TOP.0
    );
}
