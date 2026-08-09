// Probe 3. A second axis on which transfer already fails, re-derived rather than
// inherited: the radix.
//
// `63:220-223` records "a tie is reachable only at an even radix ... `2 * lost ==
// R^s` has no solution for odd `R` at any `s`". I re-derive it here from the
// model's own quantiser rather than from the formula, because the formula is the
// argument and the quantiser is the thing.
//
// Why it belongs in a file about transfer: every exhaustive check in this review
// runs at radix two. Tie-breaking is a property that is inhabited at the model
// radix and vacuous at half the radix axis's own domain, with the forbidden-
// feature bans in force throughout and monomorphisation uniform throughout. It is
// the design's own standing counterexample to "the bans make the transfer sound".

#[path = "model.rs"]
mod model;
use model::*;

/// Count exact half-ulp ties encountered while quantising every pairwise sum.
fn tie_count(f: &Fmt) -> (u64, u64) {
    let vals = enumerate(f);
    let mut ties = 0u64;
    let mut roundings = 0u64;
    for x in &vals {
        for y in &vals {
            let v = x.add_exact(y, f.r);
            if v.is_zero() {
                continue;
            }
            let a = Val {
                m: v.m.abs(),
                q: v.q,
            };
            let e = exponent_of(&a, f.r);
            let qt = f.quantum_exp(e);
            if qt > a.q {
                roundings += 1;
                let k = (qt - a.q) as u32;
                let d = ipow(f.r, k);
                let rem = a.m % d;
                if rem != 0 && 2 * rem == d {
                    ties += 1;
                }
            }
        }
    }
    (ties, roundings)
}

fn main() {
    println!("  r   parity   ties   roundings   |V|");
    for r in 2i128..=13 {
        let f = Fmt {
            r,
            p: 2,
            emin: 0,
            emax: 2,
            gradual: false,
        };
        let n = enumerate(&f).len();
        let (ties, roundings) = tie_count(&f);
        println!(
            " {r:>2}   {:<6}   {ties:>4}   {roundings:>9}   {n}",
            if r % 2 == 0 { "even" } else { "odd" }
        );
    }

    // The assertion, so this fails loudly rather than being read off a table.
    for r in 2i128..=13 {
        let f = Fmt {
            r,
            p: 2,
            emin: 0,
            emax: 2,
            gradual: false,
        };
        let (ties, roundings) = tie_count(&f);
        assert!(
            roundings > 0,
            "r={r}: no rounding at all, the check would be vacuous"
        );
        if r % 2 == 0 {
            assert!(ties > 0, "r={r} is even and should reach a tie");
        } else {
            assert_eq!(ties, 0, "r={r} is odd and should reach no tie");
        }
    }
    println!(
        "\nevery even radix in 2..=13 reaches a tie; every odd one reaches none, \
         with rounding occurring in both cases so the odd rows are not vacuous."
    );
    println!(
        "the quantiser is one function; the bans are in force; the property's truth \
         value still moves with the radix."
    );
}
