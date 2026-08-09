//! Does a strategy retag commute with the operations?
//!
//! A retag changes no value: the numeral is fixed, so the value set is identical and the
//! map is the identity on values. The question for implicitness is not whether it
//! preserves values but whether it preserves ANSWERS, that is whether
//!
//!     retag(op_S(x, y))  ==  op_T(retag x, retag y)
//!
//! for the four strategies' own resolutions from the ratified fixed-point preset table
//! (`110:2705-2712`). Where a quantisation event fires the two rows differ by
//! construction, so the equation cannot hold there. This counts where it fails, over
//! every ordered strategy pair, every numeral shape at the checked bound, and every pair
//! of values.
//!
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), plain std probe.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Strat {
    Hot,
    Warm,
    Cold,
    Precise,
}
const STRATS: [Strat; 4] = [Strat::Hot, Strat::Warm, Strat::Cold, Strat::Precise];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Out {
    Raw(i64),
    Refused,
}

/// Same-format addition of two raw indices in a numeral holding indices [0, n).
/// The out-of-range resolution is the strategy's own row.
fn add_same(a: i64, b: i64, n: i64, s: Strat) -> Out {
    let e = a + b;
    if e < n {
        return Out::Raw(e);
    }
    match s {
        Strat::Hot => Out::Raw(e.rem_euclid(n)),
        Strat::Warm | Strat::Cold => Out::Raw(n - 1),
        Strat::Precise => Out::Refused,
    }
}

fn retag(o: Out) -> Out {
    o // the identity on values, by definition of a retag
}

fn main() {
    let mut checked = 0u64;
    let mut fail = 0u64;
    let mut first: Option<String> = None;
    let mut per_pair = [[0u64; 4]; 4];

    for width in 1..=6u32 {
        let n = 1i64 << width;
        for (si, &s) in STRATS.iter().enumerate() {
            for (ti, &t) in STRATS.iter().enumerate() {
                for a in 0..n {
                    for b in 0..n {
                        checked += 1;
                        let lhs = retag(add_same(a, b, n, s));
                        let rhs = add_same(a, b, n, t);
                        if lhs != rhs {
                            fail += 1;
                            per_pair[si][ti] += 1;
                            if first.is_none() {
                                first = Some(format!(
                                    "width {width}, {s:?} -> {t:?}, a={a} b={b}: retag(op_S) {lhs:?}, op_T(retag) {rhs:?}"
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    println!("checked {checked} non-commuting {fail}");
    if let Some(f) = first {
        println!("first counterexample: {f}");
    }
    println!("per ordered pair (rows = source, cols = target), Hot Warm Cold Precise:");
    for (si, s) in STRATS.iter().enumerate() {
        println!(
            "  {s:>8?}  {:>6} {:>6} {:>6} {:>6}",
            per_pair[si][0], per_pair[si][1], per_pair[si][2], per_pair[si][3]
        );
    }
}
