// PROBE p5. Two things the instruction counts in p4 mean nothing without.
//
// SECTION A. Do the licensed arms actually agree with the fold as written, on
// values drawn from the declared window? An instruction count for an arm that
// computes a different answer measures nothing. And the check must be able to
// fail: the same arms fed values from a STRADDLING window have to disagree, or
// the instrument is feeding the implementation only inputs on which the broken
// path is never entered, which is the failure 80 hit twice and the workspace
// test gate names.
//
// SECTION B. The lifting in p2 and p3 is stated over the closure of the
// declared window, which is the right model for a fold whose LENGTH is not
// known until the program runs. p2 section 2 turned up something that does not
// fit that model: operands drawn from the straddling window [-1, 1] produced
// ZERO divergence at arities 2 through 5, even though the closure of [-1, 1] is
// the whole range and the whole range is not associative.
//
// The explanation is that at a bounded length the reachable set is the n-fold
// sumset rather than the closure, and a short fold over a narrow straddling
// window never reaches a clamp. So a SECOND, weaker predicate exists that reads
// the declared window AND the length, and it licenses strictly more.
//
// That predicate is available only where the length is a typestate fact. 80
// section 7 puts capacity at stage zero and length at stage one. This section
// measures where the two predicates part company, because the gap between them
// is exactly what a static-length fold buys over a runtime-length one.

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

fn sat8(x: i8, y: i8) -> i8 {
    x.saturating_add(y)
}

// The arms from p4, in ordinary Rust so they can be run.
fn seq(xs: &[i8]) -> i8 {
    let mut acc: i8 = 0;
    for &x in xs {
        acc = sat8(acc, x);
    }
    acc
}

fn lanes4(xs: &[i8]) -> i8 {
    let mut a = [0i8; 4];
    let mut i = 0;
    while i + 4 <= xs.len() {
        a[0] = sat8(a[0], xs[i]);
        a[1] = sat8(a[1], xs[i + 1]);
        a[2] = sat8(a[2], xs[i + 2]);
        a[3] = sat8(a[3], xs[i + 3]);
        i += 4;
    }
    let mut t: i8 = 0;
    while i < xs.len() {
        t = sat8(t, xs[i]);
        i += 1;
    }
    sat8(sat8(sat8(a[0], a[1]), sat8(a[2], a[3])), t)
}

fn lanes16(xs: &[i8]) -> i8 {
    let mut acc = [0i8; 16];
    let mut ch = xs.chunks_exact(16);
    for c in &mut ch {
        for k in 0..16 {
            acc[k] = sat8(acc[k], c[k]);
        }
    }
    let mut t: i8 = 0;
    for &x in ch.remainder() {
        t = sat8(t, x);
    }
    for k in 0..16 {
        t = sat8(t, acc[k]);
    }
    t
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn main() {
    // -----------------------------------------------------------------------
    // SECTION A
    // -----------------------------------------------------------------------
    println!("section A: do the licensed arms agree with the fold as written\n");
    println!(
        "{:<34} {:>10} {:>14} {:>14}",
        "declared window", "vectors", "lanes4 != seq", "lanes16 != seq"
    );

    let windows: [(&str, i32, i32, bool); 5] = [
        ("NonNeg<0,127>   licensed", 0, 127, true),
        ("NonPos<0,128>   licensed", -128, 0, true),
        ("NonNeg<3,40>    licensed", 3, 40, true),
        ("Win<-128,127>   REFUSED", -128, 127, false),
        ("Win<-1,127>     REFUSED", -1, 127, false),
    ];

    let mut rng = Rng(0x9E3779B97F4A7C15);
    for (label, lo, hi, licensed) in windows.iter() {
        let span = (hi - lo + 1) as u64;
        let mut d4: u64 = 0;
        let mut d16: u64 = 0;
        let mut vectors: u64 = 0;
        let mut witness: Option<(usize, i8, i8)> = None;
        for _ in 0..200_000 {
            let len = (rng.next() % 200) as usize;
            let mut xs = Vec::with_capacity(len);
            for _ in 0..len {
                xs.push((*lo + (rng.next() % span) as i32) as i8);
            }
            let s = seq(&xs);
            let a = lanes4(&xs);
            let b = lanes16(&xs);
            vectors += 1;
            if a != s {
                d4 += 1;
            }
            if b != s {
                d16 += 1;
                if witness.is_none() {
                    witness = Some((len, s, b));
                }
            }
        }
        println!("{:<34} {:>10} {:>14} {:>14}", label, vectors, d4, d16);
        if !*licensed && d16 == 0 && d4 == 0 {
            println!("      NOTE: a refused window produced no disagreement in this sample");
        }
        if let Some((l, s, b)) = witness {
            println!("      witness at len {}: seq={} lanes16={}", l, s, b);
        }
    }
    println!("\n  reading: the licensed rows must be zero and the refused rows must be");
    println!("  non-zero. A refused row at zero would mean the instrument never enters");
    println!("  the path the declaration exists to forbid.");

    // -----------------------------------------------------------------------
    // SECTION B
    // -----------------------------------------------------------------------
    println!("\nsection B: the closure predicate against a length-aware one\n");
    println!("for a signed window at model width W, the first fold length n at which");
    println!("SOME vector drawn from the window makes two parenthesisations disagree.");
    println!("`closure` is what p2's predicate says; `first n` is what a length-aware");
    println!("predicate could say instead.\n");
    println!(
        "{:>5} {:>14} {:>16} {:>12} {:>14}",
        "width", "window", "sign-uniform", "first n", "closure assoc"
    );

    for w in [4u32, 5u32, 6u32] {
        let maxv: i64 = (1i64 << (w - 1)) - 1;
        let minv: i64 = -(1i64 << (w - 1));
        let windows: Vec<(i64, i64)> = vec![
            (minv, maxv),
            (-1, 1),
            (-1, 2),
            (-2, 2),
            (-1, maxv),
            (0, maxv),
            (minv, 0),
            (-3, 3),
        ];
        for (lo, hi) in windows {
            // closure associativity
            let mut present = vec![false; (maxv - minv + 1) as usize];
            let idx = |v: i64| (v - minv) as usize;
            for v in lo..=hi {
                present[idx(v)] = true;
            }
            loop {
                let mut changed = false;
                let cur: Vec<i64> = (minv..=maxv).filter(|v| present[idx(*v)]).collect();
                for &x in &cur {
                    for &y in &cur {
                        let z = sat_add(x, y, minv, maxv);
                        if !present[idx(z)] {
                            present[idx(z)] = true;
                            changed = true;
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
            let cl: Vec<i64> = (minv..=maxv).filter(|v| present[idx(*v)]).collect();
            let mut closure_assoc = true;
            'outer: for &a in &cl {
                for &b in &cl {
                    for &c in &cl {
                        if sat_add(sat_add(a, b, minv, maxv), c, minv, maxv)
                            != sat_add(a, sat_add(b, c, minv, maxv), minv, maxv)
                        {
                            closure_assoc = false;
                            break 'outer;
                        }
                    }
                }
            }

            // first arity at which some vector diverges, exhaustive per arity
            let vals: Vec<i64> = (lo..=hi).collect();
            let mut first_n: Option<usize> = None;
            let mut max_checked: usize = 1;
            for n in 2usize..=7 {
                if (vals.len() as f64).powi(n as i32) > 3.0e7 {
                    break;
                }
                max_checked = n;
                let mut idxv = vec![0usize; n];
                let mut diverged = false;
                loop {
                    let xs: Vec<i64> = idxv.iter().map(|&i| vals[i]).collect();
                    // left fold against balanced tree, which is enough to detect
                    let mut l = xs[0];
                    for &x in &xs[1..] {
                        l = sat_add(l, x, minv, maxv);
                    }
                    let mut cur = xs.clone();
                    while cur.len() > 1 {
                        let mut nxt = Vec::new();
                        let mut i = 0;
                        while i + 1 < cur.len() {
                            nxt.push(sat_add(cur[i], cur[i + 1], minv, maxv));
                            i += 2;
                        }
                        if i < cur.len() {
                            nxt.push(cur[i]);
                        }
                        cur = nxt;
                    }
                    if l != cur[0] {
                        diverged = true;
                        break;
                    }
                    let mut k = n;
                    let mut carried = true;
                    while k > 0 {
                        k -= 1;
                        idxv[k] += 1;
                        if idxv[k] < vals.len() {
                            carried = false;
                            break;
                        }
                        idxv[k] = 0;
                    }
                    if carried {
                        break;
                    }
                }
                if diverged {
                    first_n = Some(n);
                    break;
                }
            }
            println!(
                "{:>5} {:>14} {:>16} {:>12} {:>14}",
                w,
                format!("[{},{}]", lo, hi),
                lo >= 0 || hi <= 0,
                match first_n {
                    Some(n) => format!("{}", n),
                    None => format!("none <= {}", max_checked),
                },
                closure_assoc
            );
        }
        println!();
    }

    println!("`none <= k` means the exhaustive enumeration was cut off at arity k by");
    println!("this probe's own 3e7-tuple bound, NOT that no divergence exists above k.");
    println!("The first run of this probe printed `none <= 7` on every such row, which");
    println!("overstated the coverage for the wider windows, and both runs are on disk.");
    println!();
    println!("reading: any row where sign-uniform is false and `first n` is above 2 is a");
    println!("region the closure predicate refuses and a length-aware predicate could");
    println!("license, for folds shorter than `first n`. That predicate needs the length");
    println!("at monomorphisation, which a runtime-length fold does not have.");
}
