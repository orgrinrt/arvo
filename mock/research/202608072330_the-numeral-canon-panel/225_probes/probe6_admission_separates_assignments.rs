// Probe 6, seat 225. Admitting an operation can separate two assignments that were
// one denotation under the core, so the open half of the operation-set answer
// carries an admission obligation.
//
// Model: signed fixed point, W = 4, F = 1 (raw -8..=7, value = raw/2), saturating
// overflow, floor rounding. Two assignments differ in exactly one axis, the
// intermediate treatment of a fused multiply-add: `exact` rounds and saturates
// once over the full intermediate; `stepwise` rounds and saturates the product
// first. Single operations round once by what the axis means, so the two
// assignments share them by construction (construction warrant: the axis is
// defined over composite intermediates only, and a single operation has none;
// the instrument that varies the axis and finds no movement on singles is arm B
// read together with arm A's nine composites, all of which move nothing).
//
// Arms:
//   arm A (measurement): under the core {add, sub, mul}, no depth-two composite
//     op2(op1(a, b), c) computes the exact-intermediate fused result: all nine
//     composites are swept against exact madd over all 4096 triples and every
//     one differs somewhere. So the exact behaviour is unreachable through the
//     core and the intermediate axis is invisible there: the two assignments
//     have one observable denotation under the core.
//   arm B (control, must agree): mul-then-add equals stepwise madd exhaustively.
//     The stepwise behaviour IS a core-expressible chain, which is what makes
//     the axis a real axis rather than two private rules.
//   arm C (negative control, MUST FAIL): asserts exact madd equals stepwise madd
//     exhaustively. Fails with a witness, which is the separation the admitted
//     operation creates: one declared name, two answers, unless the axis joins
//     the declared policy at admission.
//
// holds for: W = 4, F = 1, signedness = signed, overflow policy = saturate,
// rounding = floor, operation in {add, sub, mul, madd}, arity in {2, 3},
// chain length in {1, 2}, values exhaustive over the representable set,
// threads = 1, toolchain in toolchain.txt, edition = 2024.

const LO: i32 = -8;
const HI: i32 = 7;

fn sat(x: i32) -> i32 { x.clamp(LO, HI) }
fn floor_div2(x: i32) -> i32 { x.div_euclid(2) } // floor, negatives included

// single operations: one rounding, one saturation each
fn add(a: i32, b: i32) -> i32 { sat(a + b) }
fn sub(a: i32, b: i32) -> i32 { sat(a - b) }
fn mul(a: i32, b: i32) -> i32 { sat(floor_div2(a * b)) }

// the admitted operation, under each assignment of the intermediate axis
fn madd_exact(a: i32, b: i32, c: i32) -> i32 { sat(floor_div2(a * b + 2 * c)) }
fn madd_stepwise(a: i32, b: i32, c: i32) -> i32 { sat(mul(a, b) + c) }

fn main() {
    let ops: [(&str, fn(i32, i32) -> i32); 3] = [("add", add), ("sub", sub), ("mul", mul)];

    println!("arm A: no depth-two core composite computes exact madd");
    let mut reachable = false;
    for (n1, f1) in ops {
        for (n2, f2) in ops {
            let mut differs = 0u32;
            let mut first = None;
            for a in LO..=HI {
                for b in LO..=HI {
                    for c in LO..=HI {
                        if f2(f1(a, b), c) != madd_exact(a, b, c) {
                            differs += 1;
                            if first.is_none() { first = Some((a, b, c)); }
                        }
                    }
                }
            }
            println!("  {n2}({n1}(a,b),c) vs exact madd: {differs} of 4096 differ, first {first:?}");
            if differs == 0 { reachable = true; }
        }
    }
    if reachable {
        println!("  UNEXPECTED: exact madd is core-reachable, the axis was visible all along");
        std::process::exit(2);
    }
    println!("  the exact intermediate is unreachable through the core: one denotation there");

    println!();
    println!("arm B (control, must agree): mul-then-add IS stepwise madd");
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                if add(mul(a, b), c) != madd_stepwise(a, b, c) {
                    println!("  INSTRUMENT BROKEN at ({a},{b},{c})");
                    std::process::exit(2);
                }
            }
        }
    }
    println!("  equal on all 4096 triples, as required");

    println!();
    println!("arm C (negative control, MUST FAIL): exact madd == stepwise madd exhaustively");
    let mut differs = 0u32;
    let mut first = None;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                if madd_exact(a, b, c) != madd_stepwise(a, b, c) {
                    differs += 1;
                    if first.is_none() { first = Some((a, b, c)); }
                }
            }
        }
    }
    if differs == 0 {
        println!("  UNEXPECTED PASS: admitting madd separates nothing in this cell");
        std::process::exit(2);
    }
    let (a, b, c) = first.unwrap();
    println!("  FAILED AS REQUIRED: {differs} of 4096 triples differ; first ({a},{b},{c}):");
    println!("    exact {} against stepwise {}", madd_exact(a, b, c), madd_stepwise(a, b, c));
    println!("  so admitting the fused operation turns a resolver-free interior choice into");
    println!("  an answer-moving one: the axis must join the declared policy at admission,");
    println!("  or one type name carries two semantics, which is the shape the fast-math");
    println!("  obligation's own gap already condemns.");
}
