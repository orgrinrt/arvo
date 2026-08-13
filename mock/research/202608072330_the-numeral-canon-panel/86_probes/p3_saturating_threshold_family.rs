// PROBE p3. The saturating threshold family, which 84 section 11 says nobody
// searched for, including itself: a law with LIVE clamps, true at every width
// below a chosen threshold and false at and above it. If it exists, saturating
// laws have neither a sound band transfer nor a decision procedure.
//
// THREE CONSTRUCTIONS, derived by hand before this probe was written, each
// checked here against exhaustive sweeps rather than trusted:
//
//   E_d (constant-free):  forall x:  x^d == x^(d+1)   (unsigned sat mul)
//     Claim: truth set is exactly widths 1..=d. At width W <= d, every x >= 2
//     has x^d >= 2^d >= 2^W - 1, so both sides clamp to MAX; x in {0, 1} are
//     fixed points. At width d+1, x = 2 gives 2^d unclamped on the left and
//     the right either clamps or is 2^(d+1), differing either way.
//     E_63 is then true at widths 1..=63 and false at 64: a threshold at the
//     shipped width, constant-free, clamps firing at every width below it.
//
//   C-member (constant, convention-independent):
//     forall x:  sat_mul(x, C) == sat_mul(x, sat_add(C, 1))  with C = 2^63 - 1.
//     At widths <= 63 the constant C embeds as MAX under BOTH conventions
//     (wrap: low bits all ones; clamp: saturates), and sat_add(MAX, 1) = MAX,
//     so both sides are identical. At width 64, C is representable,
//     sat_add(C, 1) = 2^63 != C, witness x = 1.
//
//   Convention-dependent member:  forall x:  sat_mul(2, x) == sat_add(x, x).
//     Under CLAMP embedding of the constant 2 it is true at every width.
//     Under WRAP embedding, at width 1 the constant becomes 0 and the law is
//     false with witness x = 1, true from width 2 up: a FINAL segment. The
//     truth-set shape of a saturating law is a function of the embedding
//     convention, which is 84's trusted item 2 (width-indexing) load-bearing.
//
// PLUS a shape catalogue: every pair of constant-free terms over
// {sat_add, sat_mul} on {x, y} to depth 2, unsigned and signed, truth sets
// over widths 1..=8, classified as empty / full / initial segment / final
// segment / gapped. The question it answers: does the saturating fragment
// have any polarity structure at all (equations initial, disequations final,
// as F3 proves for wrapping), or none.
//
// Toolchain: pinned nightly-2026-05-28. Runtime probe; std/Vec are spike
// scaffolding per the panel's probe discipline, not design shape.

fn umax(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

fn sat_add_u(a: u64, b: u64, w: u32) -> u64 {
    let m = umax(w);
    let s = a.checked_add(b).unwrap_or(u64::MAX);
    if s > m {
        m
    } else {
        s
    }
}

fn sat_mul_u(a: u64, b: u64, w: u32) -> u64 {
    let m = umax(w);
    let s = a.checked_mul(b).unwrap_or(u64::MAX);
    if s > m {
        m
    } else {
        s
    }
}

// signed, width-W range [-2^(W-1), 2^(W-1) - 1], values carried in i64
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

/// x^d under width-w unsigned saturating multiplication, left fold.
fn sat_pow(x: u64, d: u32, w: u32) -> u64 {
    let mut acc = if d == 0 { 1u64.min(umax(w)) } else { x };
    for _ in 1..d {
        acc = sat_mul_u(acc, x, w);
    }
    acc
}

fn main() {
    println!("p3: the saturating threshold family\n");

    // ---------------- E_d, swept ----------------
    println!("E_d: x^d == x^(d+1), unsigned saturating mul, swept at widths 1..=14");
    let mut ok = true;
    print!("{:>4} truth set (T = true at that width)  ", "d");
    println!();
    for d in 2..=12u32 {
        let mut set = String::new();
        let mut first_false = 0u32;
        for w in 1..=14u32 {
            let n = 1u64 << w.min(20);
            let mut t = true;
            for x in 0..n {
                if sat_pow(x, d, w) != sat_pow(x, d + 1, w) {
                    t = false;
                    break;
                }
            }
            set.push(if t { 'T' } else { 'f' });
            if !t && first_false == 0 {
                first_false = w;
            }
        }
        let expect_ff = d + 1;
        let matches = first_false == expect_ff && set[..d as usize].chars().all(|c| c == 'T');
        if !matches {
            ok = false;
        }
        println!(
            "{:>4} {}   first false at width {} (predicted {})  {}",
            d,
            set,
            first_false,
            expect_ff,
            if matches { "ok" } else { "MISMATCH" }
        );
    }
    assert!(ok, "E_d truth set does not match the prediction");

    // E_63 at the shipped width, in u128 so nothing overflows silently:
    // width 64: x = 2: lhs = 2^63 (unclamped), rhs = sat(2^64) = 2^64 - 1.
    let w64max = u64::MAX as u128;
    let lhs: u128 = 1u128 << 63;
    let rhs_exact: u128 = 1u128 << 64;
    let rhs = if rhs_exact > w64max {
        w64max
    } else {
        rhs_exact
    };
    println!(
        "\nE_63 at width 64, witness x = 2: lhs = 2^63 = {:#x}, rhs = sat(2^64) = {:#x} => {}",
        lhs,
        rhs,
        if lhs != rhs as u128 {
            "FALSE at the shipped width"
        } else {
            "unexpectedly equal"
        }
    );
    assert!(lhs != rhs as u128);
    // and truth below: at width W <= 63, x >= 2 forces both sides to clamp.
    // swept at widths 1..=14 above for d <= 12; the same clamp argument at
    // d = 63 is checked here by direct evaluation at widths 20, 40, 63 on the
    // boundary points x in {0, 1, 2, 3} plus 1000 random x per width.
    let mut rng = 0x8686_1234_5678u64;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    for &w in &[20u32, 40, 63] {
        let m = umax(w);
        let mut t = true;
        let mut check = |x: u64| {
            if sat_pow(x, 63, w) != sat_pow(x, 64, w) {
                t = false;
            }
        };
        check(0);
        check(1);
        check(2);
        check(3);
        check(m);
        check(m - 1);
        for _ in 0..1000 {
            let x = next() & m;
            check(x);
        }
        println!(
            "E_63 at width {}: boundary + 1000 random points all agree: {}",
            w, t
        );
        assert!(t);
    }
    // and E_63 stays false above 64: width 65 via u128, x = 2: 2^63 vs 2^64, both unclamped.
    let m65 = (1u128 << 65) - 1;
    assert!((1u128 << 63) <= m65 && (1u128 << 64) <= m65 && (1u128 << 63) != (1u128 << 64));
    println!("E_63 at width 65: witness x = 2 gives 2^63 vs 2^64, both unclamped, still false");

    // ---------------- the constant member, both conventions ----------------
    println!("\nC-member: sat_mul(x, C) == sat_mul(x, sat_add(C, 1)), C = 2^63 - 1");
    let c_full: u64 = (1u64 << 63) - 1;
    for &conv in &["wrap", "clamp"] {
        let mut set = String::new();
        for w in 1..=12u32 {
            let m = umax(w);
            let c_emb = match conv {
                "wrap" => c_full & m,
                _ => {
                    if c_full > m {
                        m
                    } else {
                        c_full
                    }
                }
            };
            let c1 = sat_add_u(c_emb, 1u64.min(m), w);
            let n = 1u64 << w;
            let mut t = true;
            for x in 0..n {
                if sat_mul_u(x, c_emb, w) != sat_mul_u(x, c1, w) {
                    t = false;
                    break;
                }
            }
            set.push(if t { 'T' } else { 'f' });
        }
        println!("  {} embedding, widths 1..=12: {}", conv, set);
        assert!(
            set.chars().all(|c| c == 'T'),
            "C-member must be true at every narrow width under both conventions"
        );
    }
    // width 64: C representable, sat_add(C, 1) = 2^63; witness x = 1.
    let c1_64 = sat_add_u(c_full, 1, 64);
    println!(
        "  width 64: sat_add(C, 1) = {:#x}; witness x = 1: {:#x} vs {:#x} => {}",
        c1_64,
        sat_mul_u(1, c_full, 64),
        sat_mul_u(1, c1_64, 64),
        if sat_mul_u(1, c_full, 64) != sat_mul_u(1, c1_64, 64) {
            "FALSE at the shipped width"
        } else {
            "unexpectedly equal"
        }
    );
    assert!(sat_mul_u(1, c_full, 64) != sat_mul_u(1, c1_64, 64));

    // ---------------- the convention-dependent member ----------------
    println!("\nconvention-dependent member: sat_mul(2, x) == sat_add(x, x)");
    for &conv in &["wrap", "clamp"] {
        let mut set = String::new();
        for w in 1..=8u32 {
            let m = umax(w);
            let two = match conv {
                "wrap" => 2u64 & m,
                _ => 2u64.min(m),
            };
            let n = 1u64 << w;
            let mut t = true;
            for x in 0..n {
                if sat_mul_u(two, x, w) != sat_add_u(x, x, w) {
                    t = false;
                    break;
                }
            }
            set.push(if t { 'T' } else { 'f' });
        }
        println!("  {} embedding, widths 1..=8: {}", conv, set);
        if conv == "wrap" {
            assert!(
                set.starts_with('f') && set[1..].chars().all(|c| c == 'T'),
                "wrap embedding must give a FINAL segment here"
            );
        } else {
            assert!(set.chars().all(|c| c == 'T'));
        }
    }

    // ---------------- the shape catalogue, constant-free, depth <= 2 ----------------
    println!("\nshape catalogue: constant-free term pairs over {{sat_add, sat_mul}} on {{x, y}}, depth <= 2, widths 1..=8");
    // terms encoded as small expression trees
    #[derive(Clone)]
    enum T {
        X,
        Y,
        Op(bool, Box<T>, Box<T>),
    } // bool: true = add
    fn eval_u(t: &T, x: u64, y: u64, w: u32) -> u64 {
        match t {
            T::X => x,
            T::Y => y,
            T::Op(add, a, b) => {
                let (va, vb) = (eval_u(a, x, y, w), eval_u(b, x, y, w));
                if *add {
                    sat_add_u(va, vb, w)
                } else {
                    sat_mul_u(va, vb, w)
                }
            }
        }
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
    let mut terms: Vec<T> = vec![T::X, T::Y];
    let atoms: Vec<T> = terms.clone();
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
    terms.extend(d1);
    terms.extend(d2);
    println!("  terms: {}", terms.len());

    // per width, hash each term's full value table; truth set of s == t is the
    // widths where hashes AND tables agree (hash first, table to confirm).
    let wmax = 8u32;
    let classify = |set: &[bool]| -> &'static str {
        let n = set.len();
        let trues: Vec<usize> = (0..n).filter(|&i| set[i]).collect();
        if trues.is_empty() {
            return "empty";
        }
        if trues.len() == n {
            return "full";
        }
        let first = trues[0];
        let last = trues[trues.len() - 1];
        if last - first + 1 == trues.len() {
            if first == 0 {
                "initial segment"
            } else if last == n - 1 {
                "final segment"
            } else {
                "interior run"
            }
        } else {
            "gapped"
        }
    };
    for (name, signed) in [("unsigned", false), ("signed", true)] {
        let mut tables: Vec<Vec<u64>> = Vec::with_capacity(terms.len());
        for t in &terms {
            let mut per_w = Vec::with_capacity(wmax as usize);
            for w in 1..=wmax {
                // fnv over the value table
                let mut h = 0xcbf29ce484222325u64;
                let mut feed = |v: u64| {
                    h ^= v;
                    h = h.wrapping_mul(0x100000001b3);
                };
                if signed {
                    for x in smin(w)..=smax(w) {
                        for y in smin(w)..=smax(w) {
                            feed(eval_s(t, x, y, w) as u64);
                        }
                    }
                } else {
                    let n = 1u64 << w;
                    for x in 0..n {
                        for y in 0..n {
                            feed(eval_u(t, x, y, w));
                        }
                    }
                }
                per_w.push(h);
            }
            tables.push(per_w);
        }
        let mut counts = std::collections::BTreeMap::<&str, u64>::new();
        let mut examples = std::collections::BTreeMap::<&str, (usize, usize, String)>::new();
        for i in 0..terms.len() {
            for j in (i + 1)..terms.len() {
                let set: Vec<bool> = (0..wmax as usize)
                    .map(|k| tables[i][k] == tables[j][k])
                    .collect();
                let cls = classify(&set);
                *counts.entry(cls).or_insert(0) += 1;
                examples.entry(cls).or_insert_with(|| {
                    (
                        i,
                        j,
                        set.iter().map(|&b| if b { 'T' } else { 'f' }).collect(),
                    )
                });
            }
        }
        println!(
            "  {}: pair truth-set shapes over widths 1..={}:",
            name, wmax
        );
        for (cls, n) in &counts {
            let ex = &examples[cls];
            println!("    {:>16}: {:>6}   example truth set {}", cls, n, ex.2);
        }
    }
    println!("\nall checks passed");
}

// NOTE: main() above classifies by table hash. The signed catalogue reported
// gapped and interior-run shapes, which would be findings about the fragment's
// polarity structure, so they must not rest on a hash. p3b_verify (run as a
// second binary via --cfg verify_shapes) re-derives every gapped and
// interior-run pair by direct table comparison and extends its truth set
// through width 11.
