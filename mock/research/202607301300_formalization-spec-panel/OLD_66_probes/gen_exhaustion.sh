#!/bin/bash
# emit a const-eval exhaustive pair sweep at width $1
N=$1
cat <<RS
// exhaustive const-eval sweep over all ordered pairs of ${N}-bit values.
// the property is trivial on purpose: what is measured is the cost of the
// quantification, not the cost of the predicate.
const fn sweep() -> u64 {
    let n: u64 = 1u64 << ${N};
    let mut acc: u64 = 0;
    let mut a: u64 = 0;
    while a < n {
        let mut b: u64 = 0;
        while b < n {
            // one addition and one comparison, the cheapest non-vacuous body
            let s = a.wrapping_add(b);
            if s >= n { acc = acc.wrapping_add(1); }
            b += 1;
        }
        a += 1;
    }
    acc
}
pub const RESULT: u64 = sweep();
RS
