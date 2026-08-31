//! Are association shape and leaf aliasing one coordinate or two?
//!
//! `229` named `term_shape` an axis on a left-fold-against-balanced-tree
//! divergence. `230` named `leaf_aliasing` an axis and said in terms that
//! "term shape is not the right name for it", decomposing the corpus's
//! `term shapes` compound into depth (`chain_length`), operator arity (`arity`)
//! and leaf identification, with nothing left over for association.
//!
//! Neither of us varied the other's coordinate. This varies each with the other
//! held fixed, exhaustively over a finite domain, so it is a proof over that
//! domain rather than a sample.
//!
//! Build and run: `rustc -O -o p1_shape_vs_aliasing p1_shape_vs_aliasing.rs && ./p1_shape_vs_aliasing`
//! The binary is gitignored; the source and the captured output are tracked.
//!
//! Everything below is at W = 4 signed, F = 0, radix 2, arity 2, threads 1.
//! Container is `i32`, wide enough that no intermediate of the model overflows
//! before the policy is applied, so the policy is the only thing clamping.
//!
//! Arms:
//!   A  association varies, aliasing fixed at "all leaves distinct", policy
//!      saturating. If the answer moves, association is a coordinate that
//!      neither `chain_length` nor `arity` nor `leaf_aliasing` pins.
//!   B  aliasing varies, association fixed at `(x op y) op z`, and the question
//!      is whether the interval rule is exact against an enumerating oracle.
//!      This is the corpus's own condition, F111-15 at `111:1237`: "every leaf
//!      occurs at most once".
//!
//! Negative controls, outcomes written before the run:
//!   C1 under WRAPPING, arm A must report zero. Two's-complement addition is
//!      associative, so a nonzero count means the harness is broken rather than
//!      that association matters.
//!   C2 under SATURATION restricted to a declared non-negative operand window,
//!      arm A must report zero. That is the corpus's own result at
//!      `82_probes/p2_output.txt` reproduced exhaustively instead of sampled;
//!      a nonzero count means this model disagrees with the committed one.
//!   C3 arm B on a term whose leaves are all distinct must report the interval
//!      rule EXACT. If it reports conservative there, the rule is being
//!      mis-evaluated and arm B's positive result would mean nothing.

const W: i32 = 4;
const LO: i32 = -(1 << (W - 1));
const HI: i32 = (1 << (W - 1)) - 1;

#[derive(Clone, Copy, PartialEq)]
enum Policy {
    Saturate,
    Wrap,
}

fn clamp(v: i32, p: Policy) -> i32 {
    match p {
        Policy::Saturate => v.max(LO).min(HI),
        Policy::Wrap => {
            let m = 1 << W;
            let mut r = (v % m + m) % m;
            if r > HI {
                r -= m;
            }
            r
        }
    }
}

fn add(a: i32, b: i32, p: Policy) -> i32 {
    clamp(a + b, p)
}
fn sub(a: i32, b: i32, p: Policy) -> i32 {
    clamp(a - b, p)
}

/// `((a+b)+c)+d`, the left-nested fold.
fn left(a: i32, b: i32, c: i32, d: i32, p: Policy) -> i32 {
    add(add(add(a, b, p), c, p), d, p)
}
/// `(a+b)+(c+d)`, the balanced tree. Same leaf count, same operator, same arity,
/// same number of operations, same leaves, all distinct occurrences.
fn balanced(a: i32, b: i32, c: i32, d: i32, p: Policy) -> i32 {
    add(add(a, b, p), add(c, d, p), p)
}

fn arm_a(p: Policy, lo: i32, hi: i32) -> (u64, u64, Option<[i32; 4]>) {
    let (mut n, mut bad, mut wit) = (0u64, 0u64, None);
    for a in lo..=hi {
        for b in lo..=hi {
            for c in lo..=hi {
                for d in lo..=hi {
                    n += 1;
                    if left(a, b, c, d, p) != balanced(a, b, c, d, p) {
                        bad += 1;
                        if wit.is_none() {
                            wit = Some([a, b, c, d]);
                        }
                    }
                }
            }
        }
    }
    (n, bad, wit)
}

// --- arm B -------------------------------------------------------------------
//
// The interval rule: propagate a declared operand interval through the term with
// per-node interval arithmetic, clamping at each node, and ask whether the
// result can leave the representable range. The oracle enumerates the term over
// every assignment drawn from the declared interval and asks the same thing.
// The rule is EXACT on a term when it reports "may overflow" exactly where the
// oracle does.

fn iv_add(x: (i32, i32), y: (i32, i32)) -> (i32, i32) {
    (x.0 + y.0, x.1 + y.1)
}
fn iv_sub(x: (i32, i32), y: (i32, i32)) -> (i32, i32) {
    (x.0 - y.1, x.1 - y.0)
}
fn escapes(iv: (i32, i32)) -> bool {
    iv.0 < LO || iv.1 > HI
}

/// `(x + y) - z` with three independent leaves, over a declared box.
fn rule_distinct(lo: i32, hi: i32) -> bool {
    let d = (lo, hi);
    let s = iv_add(d, d);
    escapes(s) || escapes(iv_sub(s, d))
}
fn oracle_distinct(lo: i32, hi: i32) -> bool {
    for x in lo..=hi {
        for y in lo..=hi {
            for z in lo..=hi {
                if x + y < LO || x + y > HI {
                    return true;
                }
                let s = x + y;
                if s - z < LO || s - z > HI {
                    return true;
                }
            }
        }
    }
    false
}

/// `(x + y) - x`, the same tree, the same operators, the same arity, the same
/// depth. The third leaf is the first leaf again. That is the ONLY difference.
fn rule_aliased(lo: i32, hi: i32) -> bool {
    // Interval arithmetic cannot see that the two occurrences are one value.
    let d = (lo, hi);
    let s = iv_add(d, d);
    escapes(s) || escapes(iv_sub(s, d))
}
fn oracle_aliased(lo: i32, hi: i32) -> bool {
    for x in lo..=hi {
        for y in lo..=hi {
            if x + y < LO || x + y > HI {
                return true;
            }
            let s = x + y;
            if s - x < LO || s - x > HI {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("W = {W}, representable [{LO}, {HI}], F = 0, signedness = signed, arity = 2");
    println!();

    println!("## arm A: association varies, every leaf a distinct occurrence");
    let (n, bad, wit) = arm_a(Policy::Saturate, LO, HI);
    println!("  saturating, full range      : {bad} of {n} tuples disagree");
    if let Some(w) = wit {
        println!(
            "      witness {:?}  left = {}  balanced = {}",
            w,
            left(w[0], w[1], w[2], w[3], Policy::Saturate),
            balanced(w[0], w[1], w[2], w[3], Policy::Saturate)
        );
    }

    println!();
    println!("## C1 negative control: same arm under wrapping, must be 0");
    let (n1, bad1, _) = arm_a(Policy::Wrap, LO, HI);
    println!("  wrapping, full range        : {bad1} of {n1}");
    println!(
        "  {}",
        if bad1 == 0 {
            "  PASS"
        } else {
            "  FAIL, the harness reports movement where the algebra says none"
        }
    );

    println!();
    println!("## C2 negative control: saturating on a declared non-negative window, must be 0");
    let (n2, bad2, _) = arm_a(Policy::Saturate, 0, HI);
    println!("  saturating, operands in [0, {HI}] : {bad2} of {n2}");
    println!(
        "  {}",
        if bad2 == 0 {
            "  PASS, agrees with 82_probes/p2_output.txt exhaustively"
        } else {
            "  FAIL, disagrees with the committed measurement"
        }
    );

    println!();
    println!("## arm B: aliasing varies, association fixed at (x op y) op z");
    let mut mismatch_distinct = 0u32;
    let mut mismatch_aliased = 0u32;
    let mut first_alias_witness: Option<(i32, i32)> = None;
    let mut boxes = 0u32;
    for lo in LO..=HI {
        for hi in lo..=HI {
            boxes += 1;
            if rule_distinct(lo, hi) != oracle_distinct(lo, hi) {
                mismatch_distinct += 1;
            }
            if rule_aliased(lo, hi) != oracle_aliased(lo, hi) {
                mismatch_aliased += 1;
                if first_alias_witness.is_none() {
                    first_alias_witness = Some((lo, hi));
                }
            }
        }
    }
    println!("  declared operand boxes swept: {boxes}");
    println!(
        "  (x + y) - z, leaves distinct : rule disagrees with oracle on {mismatch_distinct} boxes"
    );
    println!(
        "  (x + y) - x, leaf 3 aliases 1: rule disagrees with oracle on {mismatch_aliased} boxes"
    );
    if let Some((lo, hi)) = first_alias_witness {
        println!(
            "      witness box [{lo}, {hi}]: rule says may-overflow = {}, oracle says {}",
            rule_aliased(lo, hi),
            oracle_aliased(lo, hi)
        );
    }

    println!();
    println!("## C3 negative control: the rule must be exact on the distinct-leaf term");
    println!(
        "  {}",
        if mismatch_distinct == 0 {
            "  PASS, 0 disagreements, so the rule is being evaluated correctly"
        } else {
            "  FAIL, the rule is mis-evaluated and arm B proves nothing"
        }
    );

    println!();
    println!("## verdict");
    println!(
        "  association moves the answer with aliasing held fixed: {}",
        bad > 0
    );
    println!(
        "  aliasing moves the verdict with association held fixed: {}",
        mismatch_aliased > 0
    );
}
