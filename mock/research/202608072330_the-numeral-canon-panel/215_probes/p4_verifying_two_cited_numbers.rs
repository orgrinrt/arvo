// Probe 4 for seat 215. Two numbers other rows rest on, recomputed.
//
// Section 1 recomputes the composed add-and-subtract failure count that the
// never-inherited row's `because` states as 82.7484%. The row cites only the
// consolidation; the figure is established at `79:27` as 13,882,880 of
// 16,777,216 triples, unsigned saturating, at eight bits. That is the same
// width my own sweep runs at, so it is checkable rather than merely citable.
//
//   The case that must fail: if this comes back a different count, the row's
//   headline number is wrong and I say so. A probe that can only agree is not
//   a check. The negative control is the same composition under WRAPPING,
//   which is a group and must therefore report zero failures; if wrapping also
//   fails, the harness is broken and neither number counts.
//
// Section 2 extends my overlap with the min-plus row. That row states the top
// absorbing at every cell under saturation and at no cell under wrapping, over
// `W in 2..=10` and `F in 0..=W`, unsigned. My probe 1 covered one cell of
// that, W = 8 and F = 0. One cell is a point, not an overlap, so this sweeps
// the whole rectangle to see whether the two of us genuinely agree over a
// region or only at a point.
//
//   The case that must fail: any saturating cell where the top does NOT absorb,
//   or any wrapping cell where it DOES. Either refutes the row. Wrapping must
//   report absorption nowhere, which group cancellation guarantees, so a single
//   wrapping hit means the instrument is wrong.
//
// Build: rustc -O p4_verifying_two_cited_numbers.rs -o p4

fn sat_add_u(a: i64, b: i64, max: i64) -> i64 {
    (a + b).min(max)
}
fn sat_sub_u(a: i64, b: i64) -> i64 {
    (a - b).max(0)
}
fn wrap_add_u(a: i64, b: i64, m: i64) -> i64 {
    (a + b).rem_euclid(m)
}
fn wrap_sub_u(a: i64, b: i64, m: i64) -> i64 {
    (a - b).rem_euclid(m)
}

fn section_1() -> bool {
    println!("== 1. the composed add-and-subtract count, recomputed at W = 8 ==");
    println!();
    let max = 255i64;
    let m = 256i64;

    let mut sat_bad = 0u64;
    let mut wrap_bad = 0u64;
    let mut total = 0u64;
    for a in 0..=max {
        for b in 0..=max {
            for c in 0..=max {
                total += 1;
                // (a + b) - c  against  a + (b - c)
                if sat_sub_u(sat_add_u(a, b, max), c) != sat_add_u(a, sat_sub_u(b, c), max) {
                    sat_bad += 1;
                }
                if wrap_sub_u(wrap_add_u(a, b, m), c, m) != wrap_add_u(a, wrap_sub_u(b, c, m), m) {
                    wrap_bad += 1;
                }
            }
        }
    }
    let pct = (sat_bad as f64) * 100.0 / (total as f64);
    println!("  unsigned SATURATING, (a+b)-c against a+(b-c):");
    println!("    {sat_bad} of {total} triples fail, {pct:.4}%");
    println!("  the cited figure at 79:27 is 13882880 of 16777216, 82.7484%");
    let matches = sat_bad == 13_882_880 && total == 16_777_216;
    if matches {
        println!("    RECOMPUTED EXACTLY. the count and the denominator both agree.");
    } else {
        println!("    DOES NOT MATCH. the row's headline number is wrong at this width.");
    }
    println!();
    println!("  the negative control, the same composition under WRAPPING:");
    println!("    {wrap_bad} of {total} triples fail");
    if wrap_bad == 0 {
        println!("    zero, as a group requires. the harness is not simply reporting failure.");
    } else {
        println!("    NONZERO, which a group forbids. the harness is broken.");
    }
    println!();
    println!("  worth stating plainly: the composition fails on 82.7% of triples while BOTH");
    println!("  its parts are associative on their own at this exact coordinate. probe 1");
    println!("  measures unsigned saturating addition associative on every one of the same");
    println!("  16777216 triples. so this is the composite failing where the parts hold, and");
    println!("  it is a second independent witness for that direction beside probe 1's own.");

    matches && wrap_bad == 0
}

fn section_2() -> bool {
    println!();
    println!("== 2. the absorbing top, over the whole rectangle rather than one cell ==");
    println!();
    println!("  W in 2..=10, F in 0..=W, unsigned. F does not change the raw arithmetic here,");
    println!("  so it is swept to match the cited region rather than because it can move the");
    println!("  answer, and saying that is cheaper than implying it was a free variable.");
    println!();

    let mut sat_absorbs = 0u32;
    let mut sat_fails = 0u32;
    let mut wrap_absorbs = 0u32;
    let mut wrap_cells = 0u32;

    for w in 2..=10u32 {
        let max = (1i64 << w) - 1;
        let m = 1i64 << w;
        for _f in 0..=w {
            // saturation: does MAX absorb addition, for every operand?
            let sat_ok = (0..=max).all(|x| sat_add_u(max, x, max) == max);
            if sat_ok {
                sat_absorbs += 1;
            } else {
                sat_fails += 1;
            }
            // wrapping: does ANY element absorb addition?
            let wrap_any = (0..=max).any(|t| (0..=max).all(|x| wrap_add_u(t, x, m) == t));
            if wrap_any {
                wrap_absorbs += 1;
            }
            wrap_cells += 1;
        }
    }
    println!("  cells swept: {wrap_cells}");
    println!("  saturating, the top absorbs: {sat_absorbs} cells; fails to absorb: {sat_fails}");
    println!("  wrapping, ANY absorbing element exists: {wrap_absorbs} cells");
    println!();
    let ok = sat_fails == 0 && wrap_absorbs == 0 && wrap_cells == 63;
    if ok {
        println!("  63 cells, absorption at every saturating one and at no wrapping one. that");
        println!("  reproduces the cited sweep's shape independently, and 63 is the cited cell");
        println!("  count as well, which is a coincidence worth checking rather than assuming:");
        println!("  sum over W in 2..=10 of (W+1) = 9*2 + (2+..+10) = 18 + 54 = 63. it matches");
        println!("  because the rectangle is the same rectangle.");
    } else {
        println!("  the sweep does NOT reproduce the cited shape. cells={wrap_cells}");
    }
    ok
}

fn main() {
    println!("seat 215, probe 4.");
    println!();
    let a = section_1();
    let b = section_2();
    println!();
    println!(
        "== every control: {} ==",
        if a && b {
            "PASSED"
        } else {
            "FAILED, numbers above are void"
        }
    );
    if !(a && b) {
        std::process::exit(1);
    }
}
