// Probe 2. The counterexample: a property of a `Ranged` numeral whose truth
// value flips along the exponent-span axis while the precision is held fixed,
// the implementation is one parametric function, and monomorphisation is
// uniform throughout.
//
// If such a property exists, then "the specialization and TypeId bans make
// monomorphisation uniform and the transfer is sound" is false as stated: the
// bans are necessary and not sufficient, and a per-property argument is owed.
//
// Property A, absorption-free: for all x, y in V with y != 0, quantise(x+y) != x.
//   Reasoned prediction before running: absorption needs a pair whose exponent
//   difference reaches p+1, and the largest difference a normal-only format
//   offers is span-1, so the threshold should be span >= p+2. Under Gradual the
//   subnormal grid reaches p-1 further down, so the threshold should drop.
//
// Property B, associative: for all x, y, z in V, the two bracketings of the
//   format-level sum agree.
//
// Both are exhaustive over the whole value set at each cell, never sampled.

#[path = "model.rs"]
mod model;
use model::*;

fn absorption_free(f: &Fmt) -> Option<(Val, Val)> {
    let vals = enumerate(f);
    for x in &vals {
        for y in &vals {
            if y.is_zero() {
                continue;
            }
            let (s, o) = quantise(f, &x.add_exact(y, f.r));
            if o == Outcome::Value && s.eq_exact(x, f.r) {
                return Some((*x, *y));
            }
        }
    }
    None
}

fn associative(f: &Fmt) -> Option<(Val, Val, Val)> {
    let vals = enumerate(f);
    for x in &vals {
        for y in &vals {
            let (xy, o1) = quantise(f, &x.add_exact(y, f.r));
            if o1 != Outcome::Value {
                continue;
            }
            for z in &vals {
                let (l, ol) = quantise(f, &xy.add_exact(z, f.r));
                let (yz, o2) = quantise(f, &y.add_exact(z, f.r));
                if o2 != Outcome::Value {
                    continue;
                }
                let (r2, or) = quantise(f, &x.add_exact(&yz, f.r));
                if ol != Outcome::Value || or != Outcome::Value {
                    continue;
                }
                if !l.eq_exact(&r2, f.r) {
                    return Some((*x, *y, *z));
                }
            }
        }
    }
    None
}

fn main() {
    for &r in &[2i128, 10] {
        for &gradual in &[false, true] {
            println!(
                "\nradix {r}, Underflow = {}",
                if gradual { "Gradual" } else { "Abrupt" }
            );
            println!("  p  span   absorption-free   associative   |V|");
            for p in 2u32..=4 {
                for span in 1i32..=(p as i32 + 4) {
                    let f = Fmt {
                        r,
                        p,
                        emin: 0,
                        emax: span - 1,
                        gradual,
                    };
                    let n = enumerate(&f).len();
                    // absorption is quadratic in |V|, associativity cubic; the two
                    // caps differ so the quadratic property gets the wider coverage
                    // it can afford rather than being clipped to the cubic one's reach.
                    let a = if n <= 6000 {
                        Some(absorption_free(&f).is_none())
                    } else {
                        None
                    };
                    let b = if n <= 320 {
                        Some(associative(&f).is_none())
                    } else {
                        None
                    };
                    let fmt = |o: Option<bool>| match o {
                        Some(true) => "TRUE",
                        Some(false) => "FALSE",
                        None => "-",
                    };
                    println!("  {p}  {span:>4}   {:<15}   {:<11}   {n}", fmt(a), fmt(b));
                }
            }
        }
    }

    // The finding, stated as an assertion so it fails loudly if it stops holding.
    // Radix two, precision three, Abrupt: absorption-free at span 3, absorbing at
    // span 4. One parameter moved by one; the property's truth value moved with it.
    // The measured threshold is span = p+1, one lower than the reasoned prediction
    // of p+2, because round-half-even absorbs at an exact half-ulp tie when the
    // retained digit is even. The experiment beat the argument by one.
    let small = Fmt {
        r: 2,
        p: 3,
        emin: 0,
        emax: 2,
        gradual: false,
    };
    let large = Fmt {
        r: 2,
        p: 3,
        emin: 0,
        emax: 3,
        gradual: false,
    };
    assert!(
        absorption_free(&small).is_none(),
        "expected absorption-free at span 3"
    );
    let witness = absorption_free(&large).expect("expected an absorbing pair at span 4");
    println!(
        "\nwitness at r=2 p=3 Abrupt span=4: x = {:?}, y = {:?}, quantise(x+y) = x, y != 0",
        witness.0, witness.1
    );
    // And the Underflow axis moves the threshold, which is the second coupling:
    // under Gradual the subnormal grid reaches p-1 further down, so absorption is
    // already present at span 2 for every precision checked.
    for p in 2u32..=4 {
        let g1 = Fmt {
            r: 2,
            p,
            emin: 0,
            emax: 0,
            gradual: true,
        };
        let g2 = Fmt {
            r: 2,
            p,
            emin: 0,
            emax: 1,
            gradual: true,
        };
        assert!(
            absorption_free(&g1).is_none(),
            "Gradual span 1 should be absorption-free"
        );
        assert!(
            absorption_free(&g2).is_some(),
            "Gradual span 2 should absorb at p={p}"
        );
    }
    println!("Gradual: absorbing from span 2 at p = 2, 3, 4; Abrupt: from span p+1.");
    println!("same p, same code, same monomorphisation discipline; only EMAX moved by one.");
}
