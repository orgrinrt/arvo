// PROBE 3b. Companion to probe3a: the identical shape of claim (associativity
// of `+`, exhaustive over the same narrow domain), under saturating
// semantics instead of wrapping. Expected to FAIL TO COMPILE, reproducing at
// compile time, through const evaluation, the same class of fact probe1b
// found at runtime over the full i8 domain (4,177,792 failing triples out of
// 16,777,216). This file existing and refusing to compile is the result: the
// claim "saturating addition is associative" has no valid instance on this
// domain, and the const assertion is what makes that unrepresentable rather
// than merely wrong.

const fn narrow_sat_add(a: i32, b: i32) -> i32 {
    let lo = -8;
    let hi = 7;
    let sum = a + b;
    if sum < lo {
        lo
    } else if sum > hi {
        hi
    } else {
        sum
    }
}

const fn check_saturating_associative_narrow() -> bool {
    let lo = -8i32;
    let hi = 7i32;
    let mut a = lo;
    while a <= hi {
        let mut b = lo;
        while b <= hi {
            let mut c = lo;
            while c <= hi {
                let lhs = narrow_sat_add(narrow_sat_add(a, b), c);
                let rhs = narrow_sat_add(a, narrow_sat_add(b, c));
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

const SATURATING_LAW_HOLDS_ON_NARROW_DOMAIN: () = {
    assert!(
        check_saturating_associative_narrow(),
        "saturating addition is not associative on this domain: found at least one counterexample"
    );
};

fn main() {
    let _ = SATURATING_LAW_HOLDS_ON_NARROW_DOMAIN;
    println!("unreachable if this compiled");
}
