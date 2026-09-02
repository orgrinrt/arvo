// p1: what does the shipped agreement test actually quantify over?
//
// WHY THIS RUNS. `164` section 1.4, reproduction catch two, narrows `163`'s
// stated risk on the strength of a claim about the shipped suite:
//
//   "`163` section 9 names as its load-bearing risk that its model 'assumes
//    two markers over one `(I, F)` can agree in value set and realisation
//    map'. The shipped corpus already asserts exactly that agreement:
//    `warm-container-shared`'s
//    `all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`
//    ... requires every arm over every carrier rule to produce the
//    byte-identical result against an independent oracle on every declared
//    key. So on the shipped evidence the pair `163` constructs has identical
//    denotation by test, and reading 2 (a strategy separates them
//    arithmetically) is, for this crate's swept keys, measured false rather
//    than open."
//
// That paragraph moved a live question toward closed, in `164`'s own words
// "in the direction unfavourable to comfort", so it is worth opening.
//
// THE STRUCTURAL FACT the arms below test. `key_op(key)` selects the SEMANTICS
// (0 wrapping for Warm, 1 saturating for Precise) and is a component of the
// key. Every arm is called as `arms::headroom(key, cols)`, one key at a time.
// So each assertion in that test holds `op` FIXED and varies only the carrier
// rule. Four carrier rules under one semantics is not two markers under one
// realisation map.
//
// WHAT MUST FAIL, declared before the run. If the two `op` values agreed on
// the same input, the distinction drawn here would be vacuous and the
// paragraph would be right by accident.
//   A1  `reference` at op=0 and op=1, same values, same width, same depth,
//       must DIFFER on at least one swept key.
//   A2  `reference` called twice at the same op must AGREE, so A1's
//       disagreement is the operand and not nondeterminism.
//   A3  the four arms must agree with each other at a fixed key, which is the
//       shipped claim reproduced rather than doubted.
//
// SCOPE. Every key in the crate's own `ALL_KEYS` whose `op` is 0 or 1, seed 3,
// threads = 1, target features any, toolchain: the repository pin,
// build profile: release.

use bench_warm_container_shared as b;

fn main() {
    println!("### p1. what the shipped agreement test quantifies over");
    println!("### crate: bench-warm-container-shared, via a path dependency, so the oracle");
    println!("### under test is the shipped oracle and not a copy of it.");
    println!();

    let mut all_ok = true;

    // -- A3: the shipped claim, reproduced ---------------------------------
    println!("A3. the shipped claim: four carrier rules agree at a FIXED key");
    let mut checked = 0usize;
    let mut disagreements = 0usize;
    for &key in b::ALL_KEYS.iter() {
        let buf = b::build_bytes(key, 3);
        let cols: &b::Cols = unsafe { &*(buf.as_ptr() as *const b::Cols) };
        let h = b::arms::headroom(key, cols);
        let m = b::arms::minimum(key, cols);
        let p = b::arms::plusone(key, cols);
        let n = b::arms::native(key, cols);
        checked += 1;
        if !(h == m && h == p && h == n) {
            disagreements += 1;
        }
    }
    let a3 = disagreements == 0;
    all_ok &= a3;
    println!(
        "  {checked} keys, {disagreements} disagreements   required=0   {}",
        if a3 {
            "as required"
        } else {
            "*** NOT AS REQUIRED ***"
        }
    );
    println!("  so the shipped assertion is about CARRIER RULES, and it holds.");
    println!();

    // -- A1: does the test's fixed axis actually carry a difference? -------
    println!("A1. the axis the shipped test holds FIXED: op=0 (wrapping, Warm) against");
    println!("    op=1 (saturating, Precise), on identical values, width and depth");
    let mut compared = 0usize;
    let mut differing = 0usize;
    let mut first: Option<(usize, u64, u64)> = None;
    for &key in b::ALL_KEYS.iter() {
        let op = b::key_op(key);
        if op > 1 {
            continue; // keys 2..5 are other shapes; the pair under test is 0/1
        }
        let w = b::key_w(key);
        let d = b::key_d(key);
        let n = b::key_n(key);
        let buf = b::build_bytes(key, 3);
        let vals = b::decode_min(&buf, n, w);
        let wrapped = b::reference(&vals, w, 0, d);
        let saturated = b::reference(&vals, w, 1, d);
        compared += 1;
        if wrapped != saturated {
            differing += 1;
            if first.is_none() {
                first = Some((key, wrapped, saturated));
            }
        }
    }
    let a1 = differing > 0;
    all_ok &= a1;
    println!(
        "  {compared} keys compared, {differing} where the two semantics differ   required=>0   {}",
        if a1 {
            "as required"
        } else {
            "*** NOT AS REQUIRED ***"
        }
    );
    if let Some((k, wv, sv)) = first {
        println!("  first witness: key {k}, wrapping gives {wv}, saturating gives {sv}");
    }
    println!();

    // -- A2: the comparator is not reporting noise -------------------------
    println!("A2. control: the same op twice, same input, must agree");
    let mut same_ok = true;
    for &key in b::ALL_KEYS.iter().take(12) {
        let w = b::key_w(key);
        let d = b::key_d(key);
        let n = b::key_n(key);
        let op = b::key_op(key);
        let buf = b::build_bytes(key, 3);
        let vals = b::decode_min(&buf, n, w);
        if b::reference(&vals, w, op, d) != b::reference(&vals, w, op, d) {
            same_ok = false;
        }
    }
    all_ok &= same_ok;
    println!(
        "  12 keys, repeated call agrees: {same_ok}   required=true   {}",
        if same_ok {
            "as required"
        } else {
            "*** NOT AS REQUIRED ***"
        }
    );
    println!();

    println!("### reading");
    println!("### A3 holds and A1 holds together. The shipped test asserts that four CARRIER");
    println!("### RULES agree under one fixed semantics, and the two semantics it holds fixed");
    println!("### do not agree with each other. So the suite establishes that the container is");
    println!("### invisible to arithmetic; it establishes nothing about two strategy markers");
    println!("### sharing a realisation map, because it never compares two of them.");
    println!();
    println!(
        "### overall: {}",
        if all_ok {
            "every arm as required"
        } else {
            "*** AT LEAST ONE ARM NOT AS REQUIRED ***"
        }
    );
    if !all_ok {
        std::process::exit(1);
    }
}
