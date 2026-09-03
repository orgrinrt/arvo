// Probe 2. What "equality transports through a construction for free" means,
// measured on two constructions rather than argued about.
//
// `proposal::configuration_is_not_composition_and_a_composite_is_a_primitive`
// carries the clause "equality transports through a construction for free where
// a predicate never does". Its establishing file at `161` L29 cites, among
// other things, `110` R6's p11, "congruence transports 4 of 4 with the sabotage
// control caught 0 of 4". Congruence is one reading of that clause and it is
// not the only one, and the two readings license different code.
//
// READING A, congruence. If two values of the construction are related by the
// base's equality lifted componentwise, then they denote the same thing. This
// is what a `PartialEq` derive gives and it is what `110` R6 measured.
//
// READING B, adequacy. The lifted equality IS the construction's equality: two
// values denote the same thing exactly when the lifted relation holds. This is
// what a reader takes from the words "transports for free", because free means
// nothing further is owed, and it is what somebody writing `#[derive(PartialEq)]`
// on a composite and moving on has assumed.
//
// The two come apart on any construction that normalises, and the sibling row
// `proposal::the_format_concept_carries_three_things_upward_and_compositions_
// owe_their_own_laws` names one in its own sentence: stored pairs. A stored-pair
// rational is the smallest case. `(1, 2)` and `(2, 4)` denote one value and are
// componentwise different, so reading A holds and reading B fails, on a
// construction the canon already has in view.
//
// THE ARMS.
//   A1  rational, congruence:  lifted equality implies denotational equality.
//   A2  rational, adequacy:    denotational equality implies lifted equality.
//   A3  pair, congruence:      the same question on a non-normalising pair.
//   A4  pair, adequacy:        ditto.
//   A5  rational, predicate:   is the base's own admission rule sound for the
//                              construction? Counts values the borrowed rule
//                              admits and the construction refuses.
//   A6  pair, refinement:      is the base's refinement liftable componentwise?
//                              Counts values where the componentwise lift and
//                              the true refinement disagree.
//
// THE CASES THAT MUST FAIL, stated before the run.
//   C1. A1 and A3 must both report zero. A lifted equality that is not even a
//       congruence is not the relation the clause is about, and if either goes
//       nonzero the instrument is measuring something else and no arm counts.
//   C2. A4 must report zero. The pair construction's denotation IS the pair, so
//       adequacy is true by construction there. A nonzero count means the
//       counting is broken and A2's number means nothing.
//   C3. A2 must report nonzero, with a witness. If it is zero the reading-B
//       refutation fails and this file says the clause survives.
//   C4. The sabotage arm. Re-run A2 with the rational's denotational equality
//       replaced by componentwise equality. It must report zero: the two
//       relations are then the same relation and cannot disagree. If it stays
//       nonzero, the counting does not depend on the denotation at all.
//
// WHY THIS IS A PROBE AND NOT A BENCH. Nothing here is timed and nothing here
// claims a cost. It refutes a universal on one construction and confirms it on
// another, which is exactly what an ad-hoc instrument may establish and what a
// bench harness would add nothing to.
//
// THE ENUMERATION. Base width four, unsigned, fraction width zero, so the base
// carries 0..=15. The rational's value set is every `(n, d)` with `n` in 0..=15
// and `d` in 1..=15, which is 240 values and 57,600 ordered pairs. The pair's
// value set is every `(a, b)` in 0..=15 squared, 256 values and 65,536 ordered
// pairs. Both walked in full.

const W: u32 = 4;
const MAX: u32 = (1 << W) - 1;

/// A stored-pair rational over the base. `d` is never zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rational {
    n: u32,
    d: u32,
}

/// A pair with no normalisation: the denotation is the pair itself. The
/// control construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pair {
    a: u32,
    b: u32,
}

/// Every representable rational.
fn rationals() -> Vec<Rational> {
    let mut out = Vec::new();
    for n in 0 ..= MAX {
        for d in 1 ..= MAX {
            out.push(Rational {
                n,
                d,
            });
        }
    }
    out
}

/// Every representable pair.
fn pairs() -> Vec<Pair> {
    let mut out = Vec::new();
    for a in 0 ..= MAX {
        for b in 0 ..= MAX {
            out.push(Pair {
                a,
                b,
            });
        }
    }
    out
}

/// The equality the base hands up, lifted componentwise and for free.
fn lifted_rational(x: Rational, y: Rational) -> bool {
    x.n == y.n && x.d == y.d
}

/// What the rational actually denotes. Cross-multiplication, exact, in a width
/// wide enough that the product cannot overflow.
fn denoted_rational(x: Rational, y: Rational) -> bool {
    (x.n as u64) * (y.d as u64) == (y.n as u64) * (x.d as u64)
}

fn lifted_pair(x: Pair, y: Pair) -> bool {
    x.a == y.a && x.b == y.b
}

/// The pair denotes the pair, so its denotational equality is the lifted one
/// written out separately rather than aliased, so the arm is not comparing a
/// function with itself.
fn denoted_pair(x: Pair, y: Pair) -> bool {
    let xs = (x.a, x.b);
    let ys = (y.a, y.b);
    xs.0 == ys.0 && xs.1 == ys.1
}

/// One direction of one arm.
struct Arm {
    checked:    u64,
    violations: u64,
    witness:    Option<String>,
}

fn walk<T: Copy, F, G>(
    values: &[T],
    lifted: F,
    denoted: G,
    direction: Direction,
    show: impl Fn(T) -> String,
) -> Arm
where
    F: Fn(T, T) -> bool,
    G: Fn(T, T) -> bool,
{
    let mut arm = Arm {
        checked:    0,
        violations: 0,
        witness:    None,
    };
    for &x in values {
        for &y in values {
            arm.checked += 1;
            let l = lifted(x, y);
            let d = denoted(x, y);
            let violated = match direction {
                Direction::Congruence => l && !d,
                Direction::Adequacy => d && !l,
            };
            if violated {
                arm.violations += 1;
                if arm.witness.is_none() {
                    arm.witness = Some(format!("{} against {}", show(x), show(y)));
                }
            }
        }
    }
    arm
}

#[derive(Clone, Copy)]
enum Direction {
    /// Lifted implies denoted.
    Congruence,
    /// Denoted implies lifted.
    Adequacy,
}

fn report(name: &str, arm: &Arm) {
    println!(
        "  {name}: {} violations of {} ordered pairs",
        arm.violations, arm.checked
    );
    if let Some(w) = &arm.witness {
        println!("      first witness: {w}");
    }
}

fn main() {
    println!("probe 2: equality transport, as a congruence and as an adequacy.");
    println!("base: unsigned, width {W}, fraction width zero, values 0..={MAX}.");
    println!();

    let rs = rationals();
    let ps = pairs();
    let showr = |r: Rational| format!("({}/{})", r.n, r.d);
    let showp = |p: Pair| format!("({},{})", p.a, p.b);

    println!("== the rational construction, which normalises ==");
    let a1 = walk(
        &rs,
        lifted_rational,
        denoted_rational,
        Direction::Congruence,
        showr,
    );
    report("A1 congruence, lifted implies denoted", &a1);
    let a2 = walk(
        &rs,
        lifted_rational,
        denoted_rational,
        Direction::Adequacy,
        showr,
    );
    report("A2 adequacy,   denoted implies lifted", &a2);
    println!();

    println!("== the pair construction, which does not ==");
    let a3 = walk(&ps, lifted_pair, denoted_pair, Direction::Congruence, showp);
    report("A3 congruence, lifted implies denoted", &a3);
    let a4 = walk(&ps, lifted_pair, denoted_pair, Direction::Adequacy, showp);
    report("A4 adequacy,   denoted implies lifted", &a4);
    println!();

    println!("== the predicate half, for contrast ==");
    // A5. The base's own admission rule is "representable at W bits", which
    // every value in 0..=MAX satisfies. Borrowing it componentwise for the
    // rational admits every `(n, d)` including `d = 0`, which the construction
    // must refuse or the denotation is undefined.
    let mut a5_admitted_and_refused = 0u64;
    let mut a5_total = 0u64;
    for _n in 0 ..= MAX {
        for d in 0 ..= MAX {
            a5_total += 1;
            let borrowed_admits = true; // both components are representable
            let construction_admits = d != 0;
            if borrowed_admits && !construction_admits {
                a5_admitted_and_refused += 1;
            }
        }
    }
    println!(
        "  A5 rational, borrowed admission rule: admits {} of {} the construction refuses",
        a5_admitted_and_refused, a5_total
    );

    // A6. Take a refinement of the base, "nonzero", and ask whether lifting it
    // componentwise gives the construction's own refinement. Read the pair as a
    // complex number: it is nonzero when either component is, so the
    // componentwise lift, both components nonzero, is the wrong refinement and
    // disagrees on exactly the values with one zero component.
    let mut a6_disagree = 0u64;
    let mut a6_total = 0u64;
    for a in 0 ..= MAX {
        for b in 0 ..= MAX {
            a6_total += 1;
            let componentwise_lift = a != 0 && b != 0;
            let the_constructions_own = a != 0 || b != 0;
            if componentwise_lift != the_constructions_own {
                a6_disagree += 1;
            }
        }
    }
    println!(
        "  A6 pair, componentwise lift of the base refinement: disagrees on {} of {}",
        a6_disagree, a6_total
    );
    println!();

    println!("== C4, the sabotage arm ==");
    println!("A2 re-run with the rational's denotation replaced by the lifted");
    println!("relation itself. The two are then one relation and must agree.");
    let c4 = walk(
        &rs,
        lifted_rational,
        lifted_rational,
        Direction::Adequacy,
        showr,
    );
    report("C4 sabotaged adequacy", &c4);
    println!();

    let c1_ok = a1.violations == 0 && a3.violations == 0;
    let c2_ok = a4.violations == 0;
    let c3_ok = a2.violations > 0;
    let c4_ok = c4.violations == 0;

    println!("== the controls ==");
    println!(
        "  C1 both congruence arms report zero ............... {}",
        pass(c1_ok)
    );
    println!(
        "  C2 the pair's adequacy arm reports zero ........... {}",
        pass(c2_ok)
    );
    println!(
        "  C3 the rational's adequacy arm reports nonzero .... {}",
        pass(c3_ok)
    );
    println!(
        "  C4 the sabotaged adequacy arm reports zero ........ {}",
        pass(c4_ok)
    );
    println!();
    if c1_ok && c2_ok && c3_ok && c4_ok {
        println!("every control: PASSED");
    } else {
        println!("a control FAILED. no number in this file counts.");
    }
    println!();
    println!("what this says, and it is narrower than either side of the argument.");
    println!("the lifted equality is a congruence on both constructions, so the");
    println!("clause holds on reading A and `110` R6's 4 of 4 is not disturbed.");
    println!("it is not the construction's own equality on the rational, so the");
    println!("clause fails on reading B, on a construction the sibling row names.");
    println!("and the predicate half holds on both readings and on both arms.");
}

fn pass(b: bool) -> &'static str {
    if b { "PASS" } else { "FAIL" }
}
