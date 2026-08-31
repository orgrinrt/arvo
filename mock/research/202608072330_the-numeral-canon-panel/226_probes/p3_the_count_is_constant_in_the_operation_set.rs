//! p3: does adding an operation to the declared set change how many primitives
//! there are?
//!
//! Seat 226, for `question::which_operation_set_the_design_ships`. That question
//! was blocking because the count of distinguishable primitives was held to be a
//! function of the operation set, so nobody could write the shape-to-count table
//! until the set was named. The floor ratified that every operation the design
//! declares is a function of the declared width. If that is true the count
//! cannot move as operations are added, and the set may be left open.
//!
//! The model. A realisation of a declared width W is a choice of native carrier
//! C >= W. Two realisations are the same primitive when every declared operation
//! gives the same answer on every input. The count is the number of classes.
//! Exhaustive over the whole value domain at each width, over every admissible
//! carrier, over every prefix of the operation list.
//!
//! The cases that must fail, run and reported:
//!   C1  a footprint observation, which is NOT a function of the declared width,
//!       must split the classes, and must split them to exactly the admissible
//!       carrier count rather than merely to more than one. That is the
//!       counter's floor and its ceiling in one arm.
//!   C2  an operation whose intermediate is not projected back to the declared
//!       width must split them. This is the ratified row's own note reproduced
//!       independently: a design writing the projection lazily puts the carrier
//!       into the value observation.
//!
//!       C2's first draft used an unprojected ADD alone and it did not split
//!       below W = 8. The reason is a finding rather than a broken control: the
//!       narrowest admissible carrier already has headroom above the declared
//!       width, and the projection is unobservable while that headroom exceeds
//!       the operation's growth. An add grows by one bit and a multiply by W,
//!       so the multiply is the arm that can fire. Both are run and both
//!       reported, and the headroom is printed beside them.

const NATIVE: [u32; 5] = [8, 16, 32, 64, 128];

fn carriers(w: u32) -> Vec<u32> { NATIVE.iter().copied().filter(|&c| c >= w).collect() }
fn mask(bits: u32) -> u128 { if bits >= 128 { u128::MAX } else { (1u128 << bits) - 1 } }

/// An operation is a rule from (declared width, carrier, inputs) to an answer.
/// The declared-width ones ignore `c`; the two controls do not.
type Op = (&'static str, fn(u32, u32, u128, u128) -> u128);

fn op_encode(w: u32, _c: u32, a: u128, _b: u128) -> u128 { a & mask(w) }
fn op_add(w: u32, _c: u32, a: u128, b: u128) -> u128 { (a + b) & mask(w) }
fn op_mul(w: u32, _c: u32, a: u128, b: u128) -> u128 { (a * b) & mask(w) }
fn op_xor(w: u32, _c: u32, a: u128, b: u128) -> u128 { (a ^ b) & mask(w) }
/// A fifth operation, still a function of the declared width: a multiply-add
/// whose intermediate is declared exact and projected once at the end.
fn op_fma(w: u32, _c: u32, a: u128, b: u128) -> u128 { (a * b + a) & mask(w) }

/// C1: reads the carrier and nothing else.
fn op_footprint(_w: u32, c: u32, _a: u128, _b: u128) -> u128 { c as u128 }
/// C2, growth one bit.
fn op_add_unprojected(_w: u32, c: u32, a: u128, b: u128) -> u128 { (a + b) & mask(c) }
/// C2, growth W bits. This is the arm that can fire.
fn op_mul_unprojected(_w: u32, c: u32, a: u128, b: u128) -> u128 { (a * b) & mask(c) }

const DECLARED: [Op; 5] = [
    ("encode", op_encode),
    ("add", op_add),
    ("mul", op_mul),
    ("xor", op_xor),
    ("fma", op_fma),
];

/// How many classes the carriers of `w` fall into under `ops`.
fn classes(w: u32, ops: &[Op]) -> usize {
    let lim = 1u128 << w;
    let mut sigs: Vec<Vec<u128>> = Vec::new();
    for c in carriers(w) {
        let mut sig = Vec::new();
        for (_, f) in ops {
            for a in 0..lim {
                for b in 0..lim {
                    sig.push(f(w, c, a, b));
                }
            }
        }
        if !sigs.contains(&sig) { sigs.push(sig); }
    }
    sigs.len()
}

fn main() {
    let widths: Vec<u32> = (3..=10).collect();

    println!("== the declared set, one operation at a time ==");
    println!("  W  carriers  classes after each prefix of [{}]",
        DECLARED.iter().map(|o| o.0).collect::<Vec<_>>().join(", "));
    let mut constant = true;
    for &w in &widths {
        let n = carriers(w).len();
        let counts: Vec<usize> = (1..=DECLARED.len()).map(|k| classes(w, &DECLARED[..k])).collect();
        if counts.iter().any(|&c| c != 1) { constant = false; }
        println!("  {w:<3}{n:<10}{counts:?}");
    }

    println!("\n== C1: the declared set plus a footprint observation ==");
    let mut c1 = true;
    for &w in &widths {
        let mut ops = DECLARED.to_vec();
        ops.push(("footprint", op_footprint));
        let k = classes(w, &ops);
        let n = carriers(w).len();
        if k != n { c1 = false; }
        println!("  W={w:<4}classes {k}, admissible carriers {n}");
    }

    // The C2 arms are checked against the mechanism rather than against
    // "it always splits". An unprojected result is observable exactly when the
    // operation's growth above the declared width exceeds the headroom the
    // narrowest admissible carrier already has. Growth is 1 bit for a sum and W
    // bits for a product. Checking the biconditional is a far stronger control
    // than checking that the arm fires: it can fail in two directions, and both
    // outcomes are present in the sweep, so neither branch is vacuous.
    println!("\n== C2: one operation with its projection omitted ==");
    println!("  W  add  mul  headroom    predicted split by growth > headroom");
    let mut c2 = true;
    for &w in &widths {
        let ka = classes(w, &[("add_unprojected", op_add_unprojected)]);
        let km = classes(w, &[("mul_unprojected", op_mul_unprojected)]);
        let headroom = carriers(w)[0] - w;
        let split_add = ka >= 2;
        let split_mul = km >= 2;
        if split_add != (1 > headroom) { c2 = false; }
        if split_mul != (w > headroom) { c2 = false; }
        println!(
            "  {w:<3}{ka:<5}{km:<5}{headroom:<10}add {} / mul {}",
            1 > headroom,
            w > headroom
        );
    }

    println!("\n== verdict ==");
    println!("  count constant at 1 across every prefix, W in 3..=10:      {constant}");
    println!("  C1 the footprint observation splits to the carrier count:  {c1}");
    println!("  C2 both arms split exactly where growth exceeds headroom: {c2}");
    let pass = constant && c1 && c2;
    println!("\n  RESULT: {}", if pass {
        "the class count does not move as declared-width operations are added"
    } else { "INCONCLUSIVE, see which arm failed above" });
    std::process::exit(if pass { 0 } else { 1 });
}
