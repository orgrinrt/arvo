// P2c. Two constructions refuted before this one. Both runs kept.
//
//   v1 (partial_v1_REFUTED.out): both realisations used the SAME divisor, so
//      both were undefined on the same 37 of 256 inputs. 0 splits. Widening a
//      carrier does not by itself move which inputs are undefined.
//   v2 (partial2_v2_REFUTED.out): I "widened" the divisor as (x*37-60) & 0xFF in
//      u32, which is EQUAL to the u8 wrapping form by construction, so again 0
//      splits. A widening that is a no-op modulo the container is not a widening.
//
// What actually produces a definedness split with full value agreement is an
// ALGEBRAIC SIMPLIFICATION of the interior, and that is a real design case
// rather than a toy: (x*x)/x simplifies to x, the two agree on every input where
// the first is defined, and the second is additionally defined at x = 0.
//
// THE CASES THAT MUST FAIL
//   C-A  on inputs where BOTH are defined the two must agree exactly, else they
//        are not one boundary function and no clause is being tested
//   C-B  a control pair with no partial operation must show zero splits
//   C-C  the split count must be > 0
//   C-D  the two refuted constructions must still show zero splits here, so the
//        difference is the construction rather than the harness
//   C-E  a VALUE-ONLY equivalence check that skips undefined inputs must certify
//        the pair as equivalent. That is the check clause 1 refuses, and if it
//        also refused the pair there would be nothing for the definedness
//        qualifier to add.

use std::panic;

fn profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug-assertions=on"
    } else {
        "debug-assertions=off"
    }
}

// A: the interior as written. Undefined at x = 0.
#[inline(never)]
fn simp_a(x: u32) -> u32 {
    (x.wrapping_mul(x)) / x
}
// B: the interior simplified. Total.
#[inline(never)]
fn simp_b(x: u32) -> u32 {
    x
}

// C-D, the two refuted pairs.
#[inline(never)]
fn v1_a(x: u32) -> u32 {
    let d = x % 7;
    255 / d
}
#[inline(never)]
fn v1_b(x: u32) -> u32 {
    let d = (x % 7) as u8 as u32;
    255 / d
}
#[inline(never)]
fn v2_a(x: u32) -> u32 {
    let d = (x as u8).wrapping_mul(37).wrapping_sub(60);
    255 / (d as u32)
}
#[inline(never)]
fn v2_b(x: u32) -> u32 {
    let d = ((x.wrapping_mul(37).wrapping_sub(60)) & 0xFF) as u8;
    255 / (d as u32)
}

// C-B, no partial operation anywhere.
#[inline(never)]
fn tot_a(x: u32) -> u32 {
    x.wrapping_mul(x).wrapping_add(1)
}
#[inline(never)]
fn tot_b(x: u32) -> u32 {
    (x.wrapping_mul(x)).wrapping_add(1)
}

fn sweep(a: fn(u32) -> u32, b: fn(u32) -> u32, n: u32) -> (u32, u32, u32, u32) {
    let (mut ok, mut bad, mut split, mut dis) = (0, 0, 0, 0);
    for x in 0..n {
        let ra = panic::catch_unwind(|| a(x));
        let rb = panic::catch_unwind(|| b(x));
        match (ra, rb) {
            (Ok(va), Ok(vb)) => {
                ok += 1;
                if va != vb {
                    dis += 1;
                }
            }
            (Err(_), Err(_)) => bad += 1,
            _ => split += 1,
        }
    }
    (ok, bad, split, dis)
}

fn main() {
    panic::set_hook(Box::new(|_| {}));
    const N: u32 = 4096;
    println!("== P2c, profile = {}, inputs 0..{} ==", profile(), N);
    println!(
        "{:>38} {:>8} {:>9} {:>7} {:>10}",
        "pair", "both ok", "both bad", "SPLIT", "value dis"
    );
    let mut r = std::collections::HashMap::new();
    for (name, a, b) in [
        (
            "constructed: (x*x)/x against x",
            simp_a as fn(u32) -> u32,
            simp_b as fn(u32) -> u32,
        ),
        (
            "C-D  v1, shared divisor",
            v1_a as fn(u32) -> u32,
            v1_b as fn(u32) -> u32,
        ),
        (
            "C-D  v2, no-op widening",
            v2_a as fn(u32) -> u32,
            v2_b as fn(u32) -> u32,
        ),
        (
            "C-B  no partial operation",
            tot_a as fn(u32) -> u32,
            tot_b as fn(u32) -> u32,
        ),
    ] {
        let t = sweep(a, b, N);
        println!("{name:>38} {:>8} {:>9} {:>7} {:>10}", t.0, t.1, t.2, t.3);
        r.insert(name, t);
    }

    // C-E: the value-only check that skips undefined inputs.
    let mut certified = true;
    for x in 0..N {
        let ra = panic::catch_unwind(|| simp_a(x));
        let rb = panic::catch_unwind(|| simp_b(x));
        if let (Ok(va), Ok(vb)) = (ra, rb) {
            if va != vb {
                certified = false;
            }
        }
        // inputs where either is undefined are SKIPPED, which is the defect
    }

    let c = r["constructed: (x*x)/x against x"];
    println!();
    println!(
        "C-A  value disagreements where both defined: {}   (must be 0)",
        c.3
    );
    println!(
        "C-B  no-partial control splits: {}, disagreements: {}   (both must be 0)",
        r["C-B  no partial operation"].2, r["C-B  no partial operation"].3
    );
    println!("C-C  constructed splits: {}   (must be > 0)", c.2);
    println!(
        "C-D  refuted constructions still at zero splits: v1 {}, v2 {}   (both must be 0)",
        r["C-D  v1, shared divisor"].2, r["C-D  v2, no-op widening"].2
    );
    println!("C-E  a value-only check skipping undefined inputs CERTIFIES the pair: {certified}");
    println!("     (must be true: that is the check clause 1 refuses)");
    println!();
    println!(
        "VERDICT at {}: a partial interior gives a binding-free definedness",
        profile()
    );
    println!(
        "  channel with full value agreement: {}",
        c.2 > 0 && c.3 == 0 && certified
    );
}
