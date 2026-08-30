// p1a. Route one: the operation supplied as a function pointer value.
//
// This is the shape `109` section 4 reports rustc refusing, quoted there as
// `function pointer calls are not allowed in constant functions`. Reproduced
// here rather than taken on report, because everything else in p1 is measured
// against whether this wall is real.
//
// Expected: FAILS.

const fn sat_add(a: i32, b: i32) -> i32 {
    let s = a + b;
    if s > 7 {
        7
    } else if s < -8 {
        -8
    } else {
        s
    }
}

/// The census: does the supplied binary operation associate over -8..=7?
const fn associates(op: fn(i32, i32) -> i32) -> bool {
    let mut a = -8;
    while a <= 7 {
        let mut b = -8;
        while b <= 7 {
            let mut c = -8;
            while c <= 7 {
                if op(op(a, b), c) != op(a, op(b, c)) {
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

const VERDICT: bool = associates(sat_add);

fn main() {
    println!("{VERDICT}");
}
