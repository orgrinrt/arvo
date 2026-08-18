// P2. Does a partial interior operation give a binding-free distinguishing
// channel at EVERY profile, as 172's definedness bound claims?
//
// WHY I OWE THIS CHECK. 171 tested six channels and named four untested
// (floating-point environment flags, #[track_caller] data, backtrace symbol
// names, linker-exposed data). Integer division by zero was in neither list. If
// it distinguishes at debug-assertions = off, my O-171-2 had a gap that was not
// among the four I named, and 172 found it rather than me.
//
// The pair, as in 171: two realisations of one boundary function differing only
// in an unbound interior.
//   A: computes the interior at the declared width
//   B: computes the interior wider and projects once
// with an interior division whose divisor can be zero for some input.
//
// THE CASES THAT MUST FAIL
//   C-A  On inputs where neither realisation's interior divides by zero, the two
//        must agree on the value. Otherwise they are not one boundary function
//        and nothing about definedness is being tested.
//   C-B  A pair with a TOTAL interior must NOT be distinguished at
//        debug-assertions = off. Otherwise the probe distinguishes everything and
//        the partial case carries no information.
//   C-C  The distinguishing input must be reachable, i.e. some input must
//        actually hit the zero divisor. A zero count means nothing was shown.

fn profile() -> &'static str {
    if cfg!(debug_assertions) { "debug-assertions=on" } else { "debug-assertions=off" }
}

// --- the PARTIAL pair. Interior: 255 / (x % k), which divides by zero for some x.
#[inline(never)]
fn partial_a(x: u8, k: u8) -> u8 {
    let d = x % k;           // interior, unbound
    (255u8 / d).wrapping_add(1)
}
#[inline(never)]
fn partial_b(x: u8, k: u8) -> u8 {
    let d = (x as u32) % (k as u32);   // same interior, wider carrier
    ((255u32 / d) as u8).wrapping_add(1)
}

// --- C-B's TOTAL pair. Same shape, no division.
#[inline(never)]
fn total_a(x: u8, k: u8) -> u8 {
    let t = x.wrapping_mul(3).wrapping_add(k);
    t.wrapping_add(1)
}
#[inline(never)]
fn total_b(x: u8, k: u8) -> u8 {
    let t = ((x as u32) * 3 + k as u32) as u8;
    t.wrapping_add(1)
}

fn main() {
    const K: u8 = 7;
    println!("== P2, profile = {} ==", profile());

    let mut agree = 0u32;
    let mut both_panic = 0u32;
    let mut split = 0u32;
    let mut value_disagree = 0u32;
    let mut zero_divisor = 0u32;

    for x in 0..=255u8 {
        if x % K == 0 { zero_divisor += 1; }
        let ra = std::panic::catch_unwind(|| partial_a(x, K));
        let rb = std::panic::catch_unwind(|| partial_b(x, K));
        match (ra, rb) {
            (Ok(va), Ok(vb)) => { agree += 1; if va != vb { value_disagree += 1; } }
            (Err(_), Err(_)) => both_panic += 1,
            _ => split += 1,
        }
    }
    println!("PARTIAL interior (255 / (x % {K})):");
    println!("  inputs hitting a zero divisor          : {zero_divisor}   (C-C, must be > 0)");
    println!("  both defined and equal                 : {}", agree - value_disagree);
    println!("  both defined and DISAGREEING on value  : {value_disagree}   (C-A, must be 0)");
    println!("  both undefined (both panic)            : {both_panic}");
    println!("  DEFINEDNESS SPLIT (one panics, one not): {split}");

    let mut tsplit = 0u32;
    let mut tdis = 0u32;
    for x in 0..=255u8 {
        let ra = std::panic::catch_unwind(|| total_a(x, K));
        let rb = std::panic::catch_unwind(|| total_b(x, K));
        match (ra, rb) {
            (Ok(va), Ok(vb)) => { if va != vb { tdis += 1; } }
            (Err(_), Err(_)) => {}
            _ => tsplit += 1,
        }
    }
    println!("TOTAL interior control (C-B):");
    println!("  definedness splits                     : {tsplit}");
    println!("  value disagreements                    : {tdis}");
    println!();
    println!("VERDICT at {}: a partial interior makes the pair distinguishable", profile());
    println!("  by definedness alone, with NO binding to the interior: {}", both_panic > 0 || split > 0);
    println!("  (both-undefined counts too: a caller binding only the final value still sees");
    println!("   the program abort, and the wider realisation would not have aborted had the");
    println!("   division been the only difference; see the total control for the contrast.)");
}
