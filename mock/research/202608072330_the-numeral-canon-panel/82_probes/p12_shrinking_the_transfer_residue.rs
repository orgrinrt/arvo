// PROBE p12. Section 15 item 1 of the file names the width transfer as this
// file's weakest point: p2 checks widths 2 to 6 and p3's compile-time band is 2
// to 4, and the claim at 8 and 64 rests on prose. This probe attacks that.
//
// It attacks it in two pieces rather than by pushing the same sweep wider,
// because pushing the sweep wider buys three bits and then stops, which p9
// already measured.
//
// PIECE ONE, the closed form. The prose argument is that on a non-negative
// operand set the only reachable clamp is the ceiling, so the operation is
// `min(x + y, MAX)`, and every parenthesisation of a chain of such operations
// equals `min(sum of the whole chain, MAX)`. That second statement is
// ARITY-INDEPENDENT: if both parenthesisations equal one closed form, they equal
// each other at every arity, and no n-ary sweep is needed.
//
// So the probe checks the closed form rather than the agreement. That is a
// strictly stronger claim than p2 measured, since agreement follows from it and
// it does not follow from agreement.
//
// PIECE TWO, how far the underlying identity reaches. `sat_add(x, y) ==
// min(x + y, MAX)` on non-negative operands is a fact about PAIRS, so it costs
// `2^(2(W-1))` rather than `2^(3(W-1))` and reaches much wider. Checking it
// wide, and the closed form as far as triples allow, splits the residue into two
// named pieces instead of one.
//
// A control runs throughout: the same closed form over the FULL representable
// set, which must fail, or neither piece is measuring anything.

fn sat_add(x: i64, y: i64, minv: i64, maxv: i64) -> i64 {
    let s = x + y;
    if s > maxv {
        maxv
    } else if s < minv {
        minv
    } else {
        s
    }
}

fn main() {
    println!("p12: splitting the width-transfer residue into two named pieces\n");

    // -----------------------------------------------------------------------
    // PIECE TWO first, because it reaches widest and the closed-form check
    // depends on it holding.
    // -----------------------------------------------------------------------
    println!("piece two: does `sat_add(x, y) == min(x + y, MAX)` hold on the");
    println!("non-negative half, and `max(x + y, MIN)` on the non-positive half\n");
    println!(
        "{:>5} {:>16} {:>14} {:>14} {:>18}",
        "width", "pairs per half", "nonneg fails", "nonpos fails", "control: full fails"
    );
    for w in 4u32..=16 {
        let maxv: i64 = (1i64 << (w - 1)) - 1;
        let minv: i64 = -(1i64 << (w - 1));
        let mut nn = 0u64;
        let mut np = 0u64;
        let mut ctl = 0u64;
        for x in 0..=maxv {
            for y in 0..=maxv {
                if sat_add(x, y, minv, maxv) != (x + y).min(maxv) {
                    nn += 1;
                }
            }
        }
        for x in minv..=0 {
            for y in minv..=0 {
                if sat_add(x, y, minv, maxv) != (x + y).max(minv) {
                    np += 1;
                }
            }
        }
        // Control: the same non-negative closed form applied over the FULL set,
        // which must fail, because the floor clamp is reachable there.
        for x in minv..=maxv {
            for y in minv..=maxv {
                if sat_add(x, y, minv, maxv) != (x + y).min(maxv) {
                    ctl += 1;
                }
            }
        }
        println!(
            "{:>5} {:>16} {:>14} {:>14} {:>18}",
            w,
            (maxv + 1) * (maxv + 1),
            nn,
            np,
            ctl
        );
    }

    // -----------------------------------------------------------------------
    // PIECE ONE. Both parenthesisations against the closed form, over triples.
    // -----------------------------------------------------------------------
    println!("\npiece one: do BOTH parenthesisations equal `min(a + b + c, MAX)` on the");
    println!("non-negative half. This is arity-independent: if both equal one closed");
    println!("form at arity 3, and the closed form is the whole chain's sum clamped");
    println!("once, then every parenthesisation at every arity equals it too.\n");
    println!(
        "{:>5} {:>16} {:>16} {:>16} {:>20}",
        "width", "triples per half", "left != closed", "right != closed", "control: full left!=cf"
    );
    for w in 4u32..=12 {
        let maxv: i64 = (1i64 << (w - 1)) - 1;
        let minv: i64 = -(1i64 << (w - 1));
        let mut lf = 0u64;
        let mut rf = 0u64;
        for a in 0..=maxv {
            for b in 0..=maxv {
                for c in 0..=maxv {
                    let closed = (a + b + c).min(maxv);
                    if sat_add(sat_add(a, b, minv, maxv), c, minv, maxv) != closed {
                        lf += 1;
                    }
                    if sat_add(a, sat_add(b, c, minv, maxv), minv, maxv) != closed {
                        rf += 1;
                    }
                }
            }
        }
        // Control at a width where the full set is affordable.
        let mut ctl: i64 = -1;
        if w <= 8 {
            let mut c2 = 0u64;
            for a in minv..=maxv {
                for b in minv..=maxv {
                    for c in minv..=maxv {
                        let closed = (a + b + c).min(maxv);
                        if sat_add(sat_add(a, b, minv, maxv), c, minv, maxv) != closed {
                            c2 += 1;
                        }
                    }
                }
            }
            ctl = c2 as i64;
        }
        println!(
            "{:>5} {:>16} {:>16} {:>16} {:>20}",
            w,
            (maxv + 1) * (maxv + 1) * (maxv + 1),
            lf,
            rf,
            if ctl < 0 {
                "not run".to_string()
            } else {
                ctl.to_string()
            }
        );
    }

    // -----------------------------------------------------------------------
    // The arity-independence claim, checked directly rather than only argued,
    // at a width where higher arities are affordable.
    // -----------------------------------------------------------------------
    println!("\npiece one, continued: every parenthesisation at arities 4 and 5 against");
    println!("the same closed form, at width 6, non-negative half.\n");
    let w = 6u32;
    let maxv: i64 = (1i64 << (w - 1)) - 1;
    let minv: i64 = -(1i64 << (w - 1));

    fn all_par(xs: &[i64], minv: i64, maxv: i64, out: &mut Vec<i64>) {
        if xs.len() == 1 {
            if !out.contains(&xs[0]) {
                out.push(xs[0]);
            }
            return;
        }
        for split in 1..xs.len() {
            let mut l = Vec::new();
            let mut r = Vec::new();
            all_par(&xs[..split], minv, maxv, &mut l);
            all_par(&xs[split..], minv, maxv, &mut r);
            for &a in &l {
                for &b in &r {
                    let v = sat_add(a, b, minv, maxv);
                    if !out.contains(&v) {
                        out.push(v);
                    }
                }
            }
        }
    }

    for n in 4usize..=5 {
        let vals: Vec<i64> = (0..=maxv).collect();
        let mut total = 0u64;
        let mut off_closed = 0u64;
        let mut idx = vec![0usize; n];
        loop {
            let xs: Vec<i64> = idx.iter().map(|&i| vals[i]).collect();
            let closed = xs.iter().sum::<i64>().min(maxv);
            let mut out = Vec::new();
            all_par(&xs, minv, maxv, &mut out);
            total += 1;
            if out.len() != 1 || out[0] != closed {
                off_closed += 1;
            }
            let mut k = n;
            let mut carried = true;
            while k > 0 {
                k -= 1;
                idx[k] += 1;
                if idx[k] < vals.len() {
                    carried = false;
                    break;
                }
                idx[k] = 0;
            }
            if carried {
                break;
            }
        }
        println!(
            "  arity {}: {} tuples, tuples where some parenthesisation differs from the closed form: {}",
            n, total, off_closed
        );
    }

    println!("\nreading: every residue column on the non-negative and non-positive halves");
    println!("must be zero and every control column must be non-zero. A zero control");
    println!("would mean the closed form is trivially true and the probe establishes");
    println!("nothing. What remains unchecked after this is the transfer from the widest");
    println!("width reached here to 32 and 64, which is one named sentence rather than");
    println!("the whole verdict.");
}
