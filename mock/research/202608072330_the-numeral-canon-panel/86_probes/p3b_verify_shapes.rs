// PROBE p3b. Direct (hash-free) verification of the signed gapped and
// interior-run truth sets p3's catalogue reported, with the terms printed and
// the truth sets extended through width 11 by direct table comparison.
// Same term pool as p3: constant-free, {sat_add, sat_mul} on {x, y}, depth 2.
// Toolchain: pinned nightly-2026-05-28. Runtime probe; spike scaffolding.

fn smin(w: u32) -> i64 {
    -(1i64 << (w - 1))
}
fn smax(w: u32) -> i64 {
    (1i64 << (w - 1)) - 1
}
fn sclamp(v: i128, w: u32) -> i64 {
    let lo = smin(w) as i128;
    let hi = smax(w) as i128;
    if v < lo {
        lo as i64
    } else if v > hi {
        hi as i64
    } else {
        v as i64
    }
}
fn sat_add_s(a: i64, b: i64, w: u32) -> i64 {
    sclamp(a as i128 + b as i128, w)
}
fn sat_mul_s(a: i64, b: i64, w: u32) -> i64 {
    sclamp(a as i128 * b as i128, w)
}

#[derive(Clone)]
enum T {
    X,
    Y,
    Op(bool, Box<T>, Box<T>),
}

fn eval_s(t: &T, x: i64, y: i64, w: u32) -> i64 {
    match t {
        T::X => x,
        T::Y => y,
        T::Op(add, a, b) => {
            let (va, vb) = (eval_s(a, x, y, w), eval_s(b, x, y, w));
            if *add {
                sat_add_s(va, vb, w)
            } else {
                sat_mul_s(va, vb, w)
            }
        }
    }
}

fn show(t: &T) -> String {
    match t {
        T::X => "x".into(),
        T::Y => "y".into(),
        T::Op(add, a, b) => format!(
            "{}({}, {})",
            if *add { "sadd" } else { "smul" },
            show(a),
            show(b)
        ),
    }
}

fn equal_at(a: &T, b: &T, w: u32) -> bool {
    for x in smin(w)..=smax(w) {
        for y in smin(w)..=smax(w) {
            if eval_s(a, x, y, w) != eval_s(b, x, y, w) {
                return false;
            }
        }
    }
    true
}

fn main() {
    // rebuild the identical term pool
    let atoms: Vec<T> = vec![T::X, T::Y];
    let mut d1: Vec<T> = Vec::new();
    for op in [true, false] {
        for a in &atoms {
            for b in &atoms {
                d1.push(T::Op(op, Box::new(a.clone()), Box::new(b.clone())));
            }
        }
    }
    let pool: Vec<T> = atoms.iter().chain(d1.iter()).cloned().collect();
    let mut d2: Vec<T> = Vec::new();
    for op in [true, false] {
        for a in &pool {
            for b in &pool {
                d2.push(T::Op(op, Box::new(a.clone()), Box::new(b.clone())));
            }
        }
    }
    let mut terms: Vec<T> = atoms;
    terms.extend(d1);
    terms.extend(d2);

    println!("p3b: hash-free verification of the non-monotone signed truth sets\n");
    let wmax_scan = 8u32;
    let wmax_ext = 11u32;
    let mut gapped = 0u64;
    let mut interior = 0u64;
    let mut printed = 0;
    for i in 0..terms.len() {
        for j in (i + 1)..terms.len() {
            let set: Vec<bool> = (1..=wmax_scan)
                .map(|w| equal_at(&terms[i], &terms[j], w))
                .collect();
            let trues: Vec<usize> = (0..set.len()).filter(|&k| set[k]).collect();
            if trues.is_empty() || trues.len() == set.len() {
                continue;
            }
            let contiguous = trues[trues.len() - 1] - trues[0] + 1 == trues.len();
            let is_gapped = !contiguous;
            let is_interior =
                contiguous && trues[0] != 0 && trues[trues.len() - 1] != set.len() - 1;
            if is_gapped {
                gapped += 1;
            }
            if is_interior {
                interior += 1;
            }
            let quota = if is_gapped {
                gapped <= 4
            } else {
                interior <= 4
            };
            if (is_gapped || is_interior) && quota {
                printed += 1;
                let ext: String = (1..=wmax_ext)
                    .map(|w| {
                        if equal_at(&terms[i], &terms[j], w) {
                            'T'
                        } else {
                            'f'
                        }
                    })
                    .collect();
                println!("  {}  ==  {}", show(&terms[i]), show(&terms[j]));
                println!(
                    "    truth set widths 1..={}: {}   ({})",
                    wmax_ext,
                    ext,
                    if is_gapped { "gapped" } else { "interior run" }
                );
            }
        }
    }
    println!(
        "\n  direct-comparison totals over widths 1..=8: gapped {}, interior runs {}",
        gapped, interior
    );
    assert!(
        gapped > 0 && interior > 0,
        "the hash-based catalogue overstated: no non-monotone sets survive direct check"
    );
    println!("\nall checks passed");
}
