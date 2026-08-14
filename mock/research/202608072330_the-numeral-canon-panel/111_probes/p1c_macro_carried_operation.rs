// p1c. Route three: the operation supplied as macro syntax.
//
// Third carrier for a statically-resolved operation, after the const trait
// (`109`'s route) and the const generic tag (p1b). The operation here is
// neither a type nor a value: it is a token tree the expander substitutes, so
// the const fn the compiler sees has the operation written into its body.
//
// This is the shape arvo's own shipped bench code already uses.
// `mock/benches/variants/satfold-shared/src/lib.rs:519` and `:547` are two
// const fns computing the same associativity census over two different
// operations, written out twice with the operation inline. Neither is a type,
// neither is a value, and both compute a law at const time.
//
// Expected: COMPILES with no feature gate, and reproduces p1b's counts, which
// is what makes it the same census rather than a different one.

const LO: i32 = -8;
const HI: i32 = 7;

macro_rules! census_over {
    ($name:ident, |$a:ident, $b:ident| $body:expr) => {
        const fn $name() -> (u32, u32) {
            const fn op($a: i32, $b: i32) -> i32 {
                $body
            }
            let mut assoc_fail = 0u32;
            let mut escapes = 0u32;
            let mut a = LO;
            while a <= HI {
                let mut b = LO;
                while b <= HI {
                    let r = op(a, b);
                    if r < LO || r > HI {
                        escapes += 1;
                    }
                    let mut c = LO;
                    while c <= HI {
                        if op(op(a, b), c) != op(a, op(b, c)) {
                            assoc_fail += 1;
                        }
                        c += 1;
                    }
                    b += 1;
                }
                a += 1;
            }
            (assoc_fail, escapes)
        }
    };
}

census_over!(sat_both, |a, b| {
    let s = a + b;
    if s > HI {
        HI
    } else if s < LO {
        LO
    } else {
        s
    }
});

census_over!(wrap, |a, b| ((a + b - LO).rem_euclid(HI - LO + 1)) + LO);

census_over!(sat_top_only, |a, b| {
    let s = a + b;
    if s > HI { HI } else { s }
});

const SAT_BOTH: (u32, u32) = sat_both();
const WRAP: (u32, u32) = wrap();
const SAT_TOP: (u32, u32) = sat_top_only();

fn main() {
    println!("operation carried as MACRO SYNTAX, neither a type nor a value\n");
    println!("op              assoc-failures  escapes-from-set");
    println!("sat-both        {:>14}  {:>16}", SAT_BOTH.0, SAT_BOTH.1);
    println!("wrap            {:>14}  {:>16}", WRAP.0, WRAP.1);
    println!("sat-top-only    {:>14}  {:>16}", SAT_TOP.0, SAT_TOP.1);
    println!(
        "\nagrees with p1b (const generic value carrier): {}",
        SAT_BOTH == (952, 0) && WRAP == (0, 0) && SAT_TOP == (448, 36)
    );
}
