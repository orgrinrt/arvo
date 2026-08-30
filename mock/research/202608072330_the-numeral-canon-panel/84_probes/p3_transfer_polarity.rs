// PROBE p3. Which direction a verdict transfers, and what decides it: polarity.
//
// For the wrapping fragment (ring terms: wrapping add/sub/mul/neg, integer
// constants reduced on embedding), reduction mod 2^(W-1) of a mod-2^W value is
// the mod-2^(W-1) value, and integer polynomials respect argument congruence.
// Two closure theorems follow, dual to each other:
//
//   EQUATIONS  "forall x: p(x) == q(x)":   truth is DOWNWARD closed.
//     A width-W counterexample embeds upward literally, so FALSE at a model
//     width is sound at every wider width. TRUE at a model width says nothing
//     upward (p1/p2: thresholds at every width).
//
//   DISEQUATIONS  "forall x: p(x) != q(x)":  truth is UPWARD closed.
//     A width-W solution of p == q reduces to a width-(W-1) solution, so
//     no-solution at W-1 implies no-solution at W: TRUE at a model width IS
//     sound at every wider width, and FALSE at a model width says nothing.
//
// So the band's evidential direction flips with the polarity of the law, and
// the mechanism's cross-check as built (80 p2c) distinguishes neither.
//
// This probe checks both closure shapes exhaustively over width ranges, on the
// threshold family, on hand-written polynomial laws, and on the disequation
// family x + c != x. It also demonstrates that the closure claims are about
// the FRAGMENT: nothing here touches saturating or order-predicated laws, and
// the file states that absence rather than measuring it.
//
// Toolchain: pinned nightly-2026-05-28. Runtime probe; no feature gates.

fn mask_of(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

fn falling_prod(x: u64, k: u32, w: u32) -> u64 {
    let m = mask_of(w);
    let mut acc: u64 = 1;
    for i in 0..k as u64 {
        acc = acc.wrapping_mul(x.wrapping_sub(i) & m) & m;
    }
    acc
}

/// p(x) = 8x^3 + 24x, a mixed-coefficient equation law: p == 0.
fn poly_a(x: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let x3 = x.wrapping_mul(x).wrapping_mul(x) & m;
    x3.wrapping_mul(8).wrapping_add(x.wrapping_mul(24)) & m
}

/// p(x) = 4x^2 + 4x  (= 4x(x+1), v2 >= 3 always, exactly 3 at x = 1).
fn poly_b(x: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let x2 = x.wrapping_mul(x) & m;
    x2.wrapping_mul(4).wrapping_add(x.wrapping_mul(4)) & m
}

fn main() {
    let cap = 16u32;
    println!("p3: transfer polarity in the wrapping fragment\n");

    // ---- equations: truth downward closed, counterexample embeds upward ----
    // laws: L_4, L_6, L_8, L_12, poly_a == 0, poly_b == 0
    struct EqLaw {
        name: &'static str,
        f: Box<dyn Fn(u64, u32) -> u64>,
    }
    let eq_laws: Vec<EqLaw> = vec![
        EqLaw {
            name: "L_4",
            f: Box::new(|x, w| falling_prod(x, 4, w)),
        },
        EqLaw {
            name: "L_6",
            f: Box::new(|x, w| falling_prod(x, 6, w)),
        },
        EqLaw {
            name: "L_8",
            f: Box::new(|x, w| falling_prod(x, 8, w)),
        },
        EqLaw {
            name: "L_12",
            f: Box::new(|x, w| falling_prod(x, 12, w)),
        },
        EqLaw {
            name: "8x^3+24x",
            f: Box::new(poly_a),
        },
        EqLaw {
            name: "4x^2+4x",
            f: Box::new(poly_b),
        },
    ];

    println!("equations (forall x: p(x) == 0):");
    println!(
        "{:>10} {:>14} {:>16} {:>22}",
        "law", "truth set", "initial segment?", "witness embeds upward?"
    );
    let mut eq_defects = 0u32;
    for law in &eq_laws {
        let mut verdicts = Vec::new();
        let mut first_wit: Option<(u32, u64)> = None;
        for w in 1..=cap {
            let n = 1u64 << w;
            let mut wit = None;
            for x in 0..n {
                if (law.f)(x, w) != 0 {
                    wit = Some(x);
                    break;
                }
            }
            verdicts.push(wit.is_none());
            if wit.is_some() && first_wit.is_none() {
                first_wit = Some((w, wit.unwrap()));
            }
        }
        // initial segment: no true after a false
        let mut seen_false = false;
        let mut segment = true;
        for &v in &verdicts {
            if !v {
                seen_false = true;
            } else if seen_false {
                segment = false;
            }
        }
        let true_count = verdicts.iter().filter(|&&v| v).count();
        // witness embedding
        let embeds = match first_wit {
            Some((w0, x0)) => (w0..=cap).all(|w| (law.f)(x0, w) != 0),
            None => true, // nothing to embed
        };
        if !segment || !embeds {
            eq_defects += 1;
        }
        println!(
            "{:>10} {:>14} {:>16} {:>22}",
            law.name,
            format!("1..={}", true_count),
            if segment { "yes" } else { "NO" },
            if embeds { "yes" } else { "NO" }
        );
    }
    assert!(eq_defects == 0);

    // ---- disequations: truth upward closed ----
    // family: forall x: x + c != x, for c in {32, 8, 1}; plus 2x != 1 (true
    // at every width: 2x is even) and 3x != 5 (false at every width: 3 is
    // invertible mod 2^W).
    println!("\ndisequations (forall x: lhs(x) != rhs(x)):");
    println!(
        "{:>12} {:>20} {:>15}",
        "law", "truth per width 1..=16", "final segment?"
    );
    let dis_laws: Vec<(&str, Box<dyn Fn(u64, u32) -> bool>)> = vec![
        (
            "x+32 != x",
            Box::new(|x: u64, w: u32| x.wrapping_add(32) & mask_of(w) != x),
        ),
        (
            "x+8 != x",
            Box::new(|x: u64, w: u32| x.wrapping_add(8) & mask_of(w) != x),
        ),
        (
            "x+1 != x",
            Box::new(|x: u64, w: u32| x.wrapping_add(1) & mask_of(w) != x),
        ),
        (
            "2x != 1",
            Box::new(|x: u64, w: u32| x.wrapping_mul(2) & mask_of(w) != 1),
        ),
        (
            "3x != 5",
            Box::new(|x: u64, w: u32| x.wrapping_mul(3) & mask_of(w) != 5 & mask_of(w)),
        ),
    ];
    let mut dis_defects = 0u32;
    for (name, f) in &dis_laws {
        let mut verdicts = Vec::new();
        for w in 1..=cap {
            let n = 1u64 << w;
            let ok = (0..n).all(|x| f(x, w));
            verdicts.push(ok);
        }
        // final segment: no false after a true
        let mut seen_true = false;
        let mut final_segment = true;
        for &v in &verdicts {
            if v {
                seen_true = true;
            } else if seen_true {
                final_segment = false;
            }
        }
        if !final_segment {
            dis_defects += 1;
        }
        let pat: String = verdicts
            .iter()
            .map(|&v| if v { 'T' } else { 'f' })
            .collect();
        println!(
            "{:>12} {:>20} {:>15}",
            name,
            pat,
            if final_segment { "yes" } else { "NO" }
        );
    }
    assert!(dis_defects == 0);

    println!("\nreading:");
    println!("  equations:    truth sets are initial segments; every counterexample");
    println!("                embeds upward. A band TRUE proves nothing above the band;");
    println!("                a band FALSE refutes every wider width.");
    println!("  disequations: truth sets are final segments (x+32 != x is false at");
    println!("                widths 1..=5, where 32 reduces to 0, and true from 6 up).");
    println!("                A band TRUE at width B proves TRUE at every wider width;");
    println!("                a band FALSE proves nothing above the band.");
    println!("  Nothing here touches saturating, order-predicated, or mixed laws;");
    println!("  no closure claim is made for them.");
}
