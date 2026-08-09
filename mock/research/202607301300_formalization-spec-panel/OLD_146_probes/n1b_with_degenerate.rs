//! The narrowing, second-read, and a hunt for the missing key column.
//!
//! Two jobs. First, rebuild `145`'s quantiser from the ratified preset rows without
//! reading its probe, and see whether its five reported numbers come back. Second,
//! ask what the map depends on that its stated key does not record.
//!
//! The stated key (`145:757-761`): the identity operation marker, the source numeral,
//! the target numeral, the TARGET strategy's five resolutions, and its in-range
//! Direction. A conversion has TWO strategies, one on each side. Nothing in the
//! design says the target's is the one that adjudicates, and the other two readings
//! (the source's, and the `Resolve` join) are equally stateable.
//!
//!   rustc -O n1_quantise_key.rs -o n1_quantise_key && ./n1_quantise_key

const FMAX: u32 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum S {
    Hot,
    Warm,
    Cold,
    Precise,
}
const ALL: [S; 4] = [S::Hot, S::Warm, S::Cold, S::Precise];

/// The strategy chain, `Hot < Warm < Cold < Precise`. `Resolve` is its join.
fn rank(s: S) -> u8 {
    match s {
        S::Hot => 0,
        S::Warm => 1,
        S::Cold => 2,
        S::Precise => 3,
    }
}
fn resolve(a: S, b: S) -> S {
    if rank(a) >= rank(b) {
        a
    } else {
        b
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct N {
    i: u32,
    f: u32,
}
impl N {
    fn count(&self) -> i64 {
        1i64 << (self.i + self.f)
    }
    /// raw datum j denotes j * 2^-f; scaled to the common denominator 2^-FMAX
    fn scaled(&self, j: i64) -> i64 {
        j << (FMAX - self.f)
    }
    fn step(&self) -> i64 {
        1i64 << (FMAX - self.f)
    }
}

/// The quantiser: round onto the target's grid, then classify against its range.
/// The order is round-then-classify and is not a choice (Q4, Q5).
fn quantise(u: i64, b: N, s: S) -> Option<i64> {
    let step = b.step();
    let d = u.div_euclid(step);
    let r = u.rem_euclid(step);
    // in-range direction
    let j = match s {
        S::Hot => d, // TowardNegative
        _ => {
            // ToEven
            if 2 * r < step {
                d
            } else if 2 * r > step {
                d + 1
            } else if d % 2 == 0 {
                d
            } else {
                d + 1
            }
        }
    };
    // out-of-range resolution
    let n = b.count();
    if j >= 0 && j < n {
        return Some(j);
    }
    match s {
        S::Hot => Some(j.rem_euclid(n)),              // ReduceModulo
        S::Warm | S::Cold => Some(j.clamp(0, n - 1)), // clamp
        S::Precise => None,                           // Refuse
    }
}

fn embeds(a: N, b: N) -> bool {
    a.i <= b.i && a.f <= b.f
}

fn main() {
    let mut shapes = Vec::new();
    for i in 0..=6u32 {
        for f in 0..=6u32 {
            if i + f <= 6 {
                shapes.push(N { i, f });
            }
        }
    }
    println!("shapes {} strategies {}", shapes.len(), ALL.len());

    // ---- C1: quantise from a numeral to itself is the identity --------------
    let (mut c1n, mut c1f) = (0usize, 0usize);
    for &a in &shapes {
        for &s in &ALL {
            for j in 0..a.count() {
                c1n += 1;
                if quantise(a.scaled(j), a, s) != Some(j) {
                    c1f += 1;
                }
            }
        }
    }
    println!("C1 checked {} failures {}", c1n, c1f);

    // ---- C2: on the embedding region, quantise agrees with embed ------------
    let (mut c2n, mut c2f) = (0usize, 0usize);
    for &a in &shapes {
        for &b in &shapes {
            if embeds(a, b) {
                for &s in &ALL {
                    for j in 0..a.count() {
                        c2n += 1;
                        let exact = a.scaled(j) / b.step();
                        if quantise(a.scaled(j), b, s) != Some(exact) {
                            c2f += 1;
                        }
                    }
                }
            }
        }
    }
    println!("C2 checked {} failures {}", c2n, c2f);

    // ---- C3: embed then quantise equals quantise ---------------------------
    let (mut c3n, mut c3f) = (0usize, 0usize);
    for &a in &shapes {
        for &b in &shapes {
            if embeds(a, b) {
                for &c in &shapes {
                    for &s in &ALL {
                        for j in 0..a.count() {
                            c3n += 1;
                            let mid = a.scaled(j) / b.step();
                            let via = quantise(b.scaled(mid), c, s);
                            let direct = quantise(a.scaled(j), c, s);
                            if via != direct {
                                c3f += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("C3 checked {} failures {}", c3n, c3f);

    // ---- C5: monotonicity, per strategy ------------------------------------
    let mut c5n = 0usize;
    let mut c5f = [0usize; 4];
    for &a in &shapes {
        for &b in &shapes {
            for (si, &s) in ALL.iter().enumerate() {
                for x in 0..a.count() {
                    for y in (x + 1)..a.count() {
                        if si == 0 {
                            c5n += 1;
                        }
                        let (qx, qy) = (quantise(a.scaled(x), b, s), quantise(a.scaled(y), b, s));
                        if let (Some(px), Some(py)) = (qx, qy) {
                            if px > py {
                                c5f[si] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "C5 ordered pairs {} monotonicity failures Hot {} Warm {} Cold {} Precise {}",
        c5n, c5f[0], c5f[1], c5f[2], c5f[3]
    );

    // ---- the key question: WHICH strategy adjudicates? ---------------------
    // A conversion has a source strategy and a target strategy. Three readings.
    let mut total = 0usize;
    let mut tgt_vs_src = 0usize;
    let mut tgt_vs_join = 0usize;
    let mut src_vs_join = 0usize;
    let mut lossy_total = 0usize;
    let mut lossy_tgt_vs_src = 0usize;
    let mut first: Option<String> = None;
    for &a in &shapes {
        for &b in &shapes {
            for &sa in &ALL {
                for &sb in &ALL {
                    let lossy = !embeds(a, b);
                    for j in 0..a.count() {
                        let u = a.scaled(j);
                        let by_tgt = quantise(u, b, sb);
                        let by_src = quantise(u, b, sa);
                        let by_join = quantise(u, b, resolve(sa, sb));
                        total += 1;
                        if lossy {
                            lossy_total += 1;
                        }
                        if by_tgt != by_src {
                            tgt_vs_src += 1;
                            if lossy {
                                lossy_tgt_vs_src += 1;
                            }
                            if first.is_none() {
                                first = Some(format!(
                        "Q{}.{} {:?} into Q{}.{} {:?}, raw {}: target says {:?}, source says {:?}, join says {:?}",
                        a.i, a.f, sa, b.i, b.f, sb, j, by_tgt, by_src, by_join));
                            }
                        }
                        if by_tgt != by_join {
                            tgt_vs_join += 1;
                        }
                        if by_src != by_join {
                            src_vs_join += 1;
                        }
                    }
                }
            }
        }
    }
    println!(
        "K. conversions checked {} of which lossy {}",
        total, lossy_total
    );
    println!(
        "K. target vs source disagree {} ({} of them lossy)",
        tgt_vs_src, lossy_tgt_vs_src
    );
    println!("K. target vs join   disagree {}", tgt_vs_join);
    println!("K. source vs join   disagree {}", src_vs_join);
    if let Some(s) = first {
        println!("K. first: {}", s);
    }

    // ---- and on the EMBEDDING region, do the three readings agree? ---------
    let mut emb_total = 0usize;
    let mut emb_disagree = 0usize;
    for &a in &shapes {
        for &b in &shapes {
            if embeds(a, b) {
                for &sa in &ALL {
                    for &sb in &ALL {
                        for j in 0..a.count() {
                            emb_total += 1;
                            let u = a.scaled(j);
                            if quantise(u, b, sb) != quantise(u, b, sa) {
                                emb_disagree += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "K. on the embedding region: {} checks, {} disagreements",
        emb_total, emb_disagree
    );
}
