// PROBE 3a. Ties I9 ("the strategy is what makes an answer correct") to the
// type system rather than to a runtime flag or a philosophical claim: the
// SAME claimed law (associativity of `+`), checked EXHAUSTIVELY over a small
// domain narrow enough for const-eval to finish quickly (avoiding the
// documented const-eval wall for wider domains), compiles clean under one
// strategy's semantics and (probe3b) refuses to compile under another's.
//
// The domain is deliberately narrow (4-bit signed range, [-8, 7], 16 values,
// 16^3 = 4096 triples) so the WHOLE domain is checked, not a sample, while
// staying fast enough for const evaluation. Independent of probe1/probe1b,
// which established the same class of fact at runtime over the FULL i8
// domain (256^3 triples); this probe re-derives it through a completely
// different mechanism (const evaluation, not execution) at a smaller width.

const fn narrow_wrap_add(a: i32, b: i32) -> i32 {
    // 4-bit wrapping in [-8, 7]: modulo 16 with sign offset.
    let m = (a + b + 8).rem_euclid(16);
    m - 8
}

const fn check_wrapping_associative_narrow() -> bool {
    let lo = -8i32;
    let hi = 7i32;
    let mut a = lo;
    while a <= hi {
        let mut b = lo;
        while b <= hi {
            let mut c = lo;
            while c <= hi {
                let lhs = narrow_wrap_add(narrow_wrap_add(a, b), c);
                let rhs = narrow_wrap_add(a, narrow_wrap_add(b, c));
                if lhs != rhs {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

// A top-level const item is evaluated during compilation regardless of
// whether anything ever reads it: this is the VALIDATE step, at compile
// time, for the whole domain, not a sample of it.
const WRAPPING_LAW_HOLDS_ON_NARROW_RING: () = {
    assert!(
        check_wrapping_associative_narrow(),
        "wrapping should be associative on a narrow ring domain: this failing would be the finding"
    );
};

fn main() {
    let _ = WRAPPING_LAW_HOLDS_ON_NARROW_RING;
    println!("wrapping: associativity holds on the narrow ring domain, confirmed at compile time (const-eval, not sampled)");
}
