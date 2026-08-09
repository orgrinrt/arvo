//! p1a: a law stated as an author-written marker is a declaration checked against
//! nothing, and a false one compiles clean and produces a wrong answer silently.
//!
//! This is the law-layer analogue of `68_probes/p3_mutant_overdeclared_window.rs`,
//! which found the same shape one coordinate lower: an overstated declaration about
//! a container's window passed the whole validation suite because nothing tied the
//! declaration to the executable maps.
//!
//! Model: a 4-bit signed window Q = [-8, 7], two overflow policies, one law
//! (associativity of the policy's addition), one consumer of the law (a fold that
//! is licensed to reassociate into a balanced tree only when the law holds).
//!
//! The fold never mentions an identity element: it starts from `xs[0]`, so what is
//! measured is grouping alone. See NOTE_p1a_first_run.md for the first run, which
//! measured something else and is kept.
//!
//! Toolchain: nightly-2026-05-28. No feature gates.
//! Build: rustc --edition 2021 -O p1a_declared_law_lies.rs -o p1a && ./p1a

const LO: i32 = -8;
const HI: i32 = 7;
const N: i32 = HI - LO + 1;

/// The strategy's actual addition. This is the *map*: the thing a law is a claim about.
trait Policy {
    fn add(a: i32, b: i32) -> i32;
}

struct Wrap;
struct SatSigned;

impl Policy for Wrap {
    fn add(a: i32, b: i32) -> i32 {
        (a + b - LO).rem_euclid(N) + LO
    }
}

impl Policy for SatSigned {
    fn add(a: i32, b: i32) -> i32 {
        let s = a + b;
        if s > HI {
            HI
        } else if s < LO {
            LO
        } else {
            s
        }
    }
}

/// The law, stated the way a marker contract states it: an implementation declares
/// it satisfies the contract.
///
/// Nothing anywhere connects this trait to `Policy::add`. It is a permission slip.
trait AssocAdd: Policy {}

impl AssocAdd for Wrap {}

// THE LIE. Signed saturating addition is not associative on this window.
// Nothing in the language or in this file objects to the following line.
impl AssocAdd for SatSigned {}

/// The consumer of the law. Bounded on the marker, it reassociates into a balanced
/// tree, which is only sound if the marker is telling the truth.
fn reassociating_fold<P: AssocAdd>(xs: &[i32]) -> i32 {
    match xs.len() {
        1 => xs[0],
        n => {
            let mid = n / 2;
            P::add(
                reassociating_fold::<P>(&xs[..mid]),
                reassociating_fold::<P>(&xs[mid..]),
            )
        }
    }
}

/// The unlicensed shape: strictly left, no law required, no identity mentioned.
fn sequential_fold<P: Policy>(xs: &[i32]) -> i32 {
    let mut acc = xs[0];
    for &x in &xs[1..] {
        acc = P::add(acc, x);
    }
    acc
}

fn disagreements<P: AssocAdd>() -> (u64, u64, Option<([i32; 4], i32, i32)>) {
    let mut total = 0u64;
    let mut bad = 0u64;
    let mut witness = None;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                for d in LO..=HI {
                    let xs = [a, b, c, d];
                    total += 1;
                    let l = sequential_fold::<P>(&xs);
                    let t = reassociating_fold::<P>(&xs);
                    if l != t {
                        bad += 1;
                        if witness.is_none() {
                            witness = Some((xs, l, t));
                        }
                    }
                }
            }
        }
    }
    (total, bad, witness)
}

/// Control: the law is a claim about grouping, so an instrument that cannot see a
/// grouping difference cannot report one. At arity 2 the two folds are the same
/// expression, and the count must be zero for both policies or the instrument is
/// measuring something other than grouping.
fn control_arity_two<P: AssocAdd>() -> u64 {
    let mut bad = 0u64;
    for a in LO..=HI {
        for b in LO..=HI {
            let xs = [a, b];
            if sequential_fold::<P>(&xs) != reassociating_fold::<P>(&xs) {
                bad += 1;
            }
        }
    }
    bad
}

fn main() {
    println!("p1a: a declared law that nothing checks");
    println!(
        "model window Q = [{}, {}], fold arity 4, grouping only",
        LO, HI
    );
    println!();

    let w = disagreements::<Wrap>();
    let s = disagreements::<SatSigned>();

    println!(
        "{:>10}  declares AssocAdd: yes   vectors: {}   left-fold != tree-fold: {}",
        "Wrap", w.0, w.1
    );
    if let Some((x, l, t)) = w.2 {
        println!("{:>10}  witness {:?}: left={} tree={}", "", x, l, t);
    }
    println!(
        "{:>10}  declares AssocAdd: yes   vectors: {}   left-fold != tree-fold: {}",
        "SatSigned", s.0, s.1
    );
    if let Some((x, l, t)) = s.2 {
        println!("{:>10}  witness {:?}: left={} tree={}", "", x, l, t);
    }

    println!();
    println!(
        "control, arity 2 (no grouping choice exists): Wrap {}, SatSigned {}",
        control_arity_two::<Wrap>(),
        control_arity_two::<SatSigned>()
    );

    println!();
    println!("Both types satisfy the bound `P: AssocAdd`. The compiler raised nothing.");
    println!("One of the two declarations is false, and the consumer of the law returns");
    println!("a different answer on every disagreeing vector, with no signal at the site.");
}
