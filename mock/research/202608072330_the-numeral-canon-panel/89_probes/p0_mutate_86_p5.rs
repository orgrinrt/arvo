// PROBE p0 (file 89). Attack on 86's p5 battery BEFORE relying on F7.
//
// 86 F7 says the piecewise procedure agrees with exhaustive sweeps over 3,708
// verdicts. A battery can produce that number while never exercising the part
// of the procedure that could be wrong. The test the panel's own probe
// discipline asks for: mutate the procedure and see whether the battery
// notices. If a mutant that deletes the piece structure still scores zero
// mismatches, the validation is setup that helps.
//
// Mutants:
//   M0  the procedure as 86 wrote it (control, must score 0)
//   M1  no breakpoints at all: one piece [0, MAX], D+1 samples
//   M2  breakpoints, but ONE sample per piece (its left endpoint)
//   M3  breakpoints, D samples per piece instead of D+1
//   M4  breakpoints found by LINEAR scan of the first 64 values instead of
//       binary search (what a "small widths only" instrument would pass with)
//
// The battery is 86's, seed and all, so the counts are comparable to its
// transcript. Runtime spike; std/Vec/Box are scaffolding, not design shape.

fn umax(w: u32) -> u128 {
    if w >= 64 {
        u64::MAX as u128
    } else {
        (1u128 << w) - 1
    }
}

#[derive(Clone)]
enum T {
    X,
    C(u64),
    Add(Box<T>, Box<T>),
    Mul(Box<T>, Box<T>),
}

impl T {
    fn eval(&self, x: u128, w: u32, evals: &mut u64) -> u128 {
        let m = umax(w);
        match self {
            T::X => x,
            T::C(c) => {
                let c = *c as u128;
                if c > m {
                    m
                } else {
                    c
                }
            }
            T::Add(a, b) => {
                let (va, vb) = (a.eval(x, w, evals), b.eval(x, w, evals));
                *evals += 1;
                let s = va + vb;
                if s > m {
                    m
                } else {
                    s
                }
            }
            T::Mul(a, b) => {
                let (va, vb) = (a.eval(x, w, evals), b.eval(x, w, evals));
                *evals += 1;
                let s = va * vb;
                if s > m {
                    m
                } else {
                    s
                }
            }
        }
    }
    fn clamp_mask(&self, x: u128, w: u32, idx: &mut u32, mask: &mut u128) -> u128 {
        let m = umax(w);
        match self {
            T::X => x,
            T::C(c) => {
                let c = *c as u128;
                if c > m {
                    m
                } else {
                    c
                }
            }
            T::Add(a, b) => {
                let va = a.clamp_mask(x, w, idx, mask);
                let vb = b.clamp_mask(x, w, idx, mask);
                let my = *idx;
                *idx += 1;
                let s = va + vb;
                if s > m {
                    *mask |= 1u128 << my;
                    m
                } else {
                    s
                }
            }
            T::Mul(a, b) => {
                let va = a.clamp_mask(x, w, idx, mask);
                let vb = b.clamp_mask(x, w, idx, mask);
                let my = *idx;
                *idx += 1;
                let s = va * vb;
                if s > m {
                    *mask |= 1u128 << my;
                    m
                } else {
                    s
                }
            }
        }
    }
    fn n_ops(&self) -> u32 {
        match self {
            T::X | T::C(_) => 0,
            T::Add(a, b) | T::Mul(a, b) => 1 + a.n_ops() + b.n_ops(),
        }
    }
    fn degree(&self) -> u64 {
        match self {
            T::X => 1,
            T::C(_) => 0,
            T::Add(a, b) => a.degree().max(b.degree()),
            T::Mul(a, b) => a.degree() + b.degree(),
        }
    }
}

fn mask_at(t: &T, x: u128, w: u32) -> u128 {
    let mut i = 0;
    let mut m = 0;
    t.clamp_mask(x, w, &mut i, &mut m);
    m
}

fn breakpoints(a: &T, b: &T, w: u32, evals: &mut u64, linear_cap: Option<u128>) -> Vec<u128> {
    let m = umax(w);
    let mut bps: Vec<u128> = Vec::new();
    for term in [a, b] {
        let n = term.n_ops();
        for node in 0..n {
            let bit = 1u128 << node;
            let clamps_at = |x: u128, evals: &mut u64| -> bool {
                *evals += term.n_ops() as u64;
                mask_at(term, x, w) & bit != 0
            };
            if !clamps_at(m, evals) {
                continue;
            }
            if clamps_at(0, evals) {
                continue;
            }
            match linear_cap {
                None => {
                    let (mut lo, mut hi) = (0u128, m);
                    while hi - lo > 1 {
                        let mid = lo + (hi - lo) / 2;
                        if clamps_at(mid, evals) {
                            hi = mid;
                        } else {
                            lo = mid;
                        }
                    }
                    bps.push(hi);
                }
                Some(cap) => {
                    // M4: linear scan of the first `cap` values, giving up after that
                    let mut found = None;
                    let mut x = 1u128;
                    while x <= m && x <= cap {
                        if clamps_at(x, evals) {
                            found = Some(x);
                            break;
                        }
                        x += 1;
                    }
                    if let Some(f) = found {
                        bps.push(f);
                    }
                }
            }
        }
    }
    bps.sort_unstable();
    bps.dedup();
    bps
}

#[derive(Clone, Copy, PartialEq)]
enum Mut {
    M0,
    M1,
    M2,
    M3,
    M4,
}

fn verdict(a: &T, b: &T, w: u32, mu: Mut) -> (bool, Option<u128>, u64) {
    let m = umax(w);
    let mut evals = 0u64;
    let d = a.degree().max(b.degree()) as u128;
    let bps = match mu {
        Mut::M1 => Vec::new(),
        Mut::M4 => breakpoints(a, b, w, &mut evals, Some(64)),
        _ => breakpoints(a, b, w, &mut evals, None),
    };
    let mut cuts: Vec<u128> = vec![0];
    cuts.extend(bps.iter().copied());
    cuts.push(m + 1);
    cuts.dedup();
    for win in cuts.windows(2) {
        let (lo, hi) = (win[0], win[1]);
        let len = hi - lo;
        let want = match mu {
            Mut::M2 => 1,
            Mut::M3 => d.max(1),
            _ => d + 1,
        };
        let probe_n = if len <= want { len } else { want };
        for i in 0..probe_n {
            let x = lo + i;
            let va = a.eval(x, w, &mut evals);
            let vb = b.eval(x, w, &mut evals);
            if va != vb {
                return (false, Some(x), evals);
            }
        }
    }
    (true, None, evals)
}

fn sweep(a: &T, b: &T, w: u32) -> bool {
    let m = umax(w);
    let mut e = 0u64;
    let mut x = 0u128;
    while x <= m {
        if a.eval(x, w, &mut e) != b.eval(x, w, &mut e) {
            return false;
        }
        x += 1;
    }
    true
}

struct Xorshift(u64);
impl Xorshift {
    fn next(&mut self) -> u64 {
        let mut v = self.0;
        v ^= v << 13;
        v ^= v >> 7;
        v ^= v << 17;
        self.0 = v;
        v
    }
}

fn random_term(rng: &mut Xorshift, depth: u32) -> T {
    if depth == 0 || rng.next() % 4 == 0 {
        if rng.next() % 2 == 0 {
            T::X
        } else {
            T::C((rng.next() % 8) << (rng.next() % 4))
        }
    } else {
        let a = Box::new(random_term(rng, depth - 1));
        let b = Box::new(random_term(rng, depth - 1));
        if rng.next() % 2 == 0 {
            T::Add(a, b)
        } else {
            T::Mul(a, b)
        }
    }
}
fn pow_term(d: u32) -> T {
    let mut t = T::X;
    for _ in 1..d {
        t = T::Mul(Box::new(t), Box::new(T::X));
    }
    t
}

fn main() {
    // 86's battery, same seed, same shape.
    let mut rng = Xorshift(0x5555_8686_5555);
    let mut pairs: Vec<(T, T)> = Vec::new();
    while pairs.len() < 300 {
        let a = random_term(&mut rng, 3);
        let b = random_term(&mut rng, 3);
        if a.degree() <= 16 && b.degree() <= 16 && (a.n_ops() + b.n_ops()) > 0 {
            pairs.push((a, b));
        }
    }
    for d in 2..=10u32 {
        pairs.push((pow_term(d), pow_term(d + 1)));
    }

    // How much piece structure does the battery actually contain?
    let wmax = 12u32;
    let mut with_interior_bp = 0u64;
    let mut total = 0u64;
    let mut max_bps = 0usize;
    for (a, b) in &pairs {
        for w in 1..=wmax {
            let mut e = 0;
            let bps = breakpoints(a, b, w, &mut e, None);
            total += 1;
            if !bps.is_empty() {
                with_interior_bp += 1;
            }
            if bps.len() > max_bps {
                max_bps = bps.len();
            }
        }
    }
    println!("p0: is 86's p5 battery capable of failing?\n");
    println!(
        "battery shape: {} pairs x widths 1..={} = {} cases",
        pairs.len(),
        wmax,
        total
    );
    println!(
        "  cases with at least one interior breakpoint: {}",
        with_interior_bp
    );
    println!(
        "  max breakpoints in any case:                 {}\n",
        max_bps
    );

    for mu in [Mut::M0, Mut::M1, Mut::M2, Mut::M3, Mut::M4] {
        let (mut mis, mut t, mut f) = (0u64, 0u64, 0u64);
        for (a, b) in &pairs {
            for w in 1..=wmax {
                let s = sweep(a, b, w);
                let (p, _, _) = verdict(a, b, w, mu);
                if s != p {
                    mis += 1;
                }
                if p {
                    t += 1
                } else {
                    f += 1
                }
            }
        }
        let name = match mu {
            Mut::M0 => "M0 control (86's procedure)",
            Mut::M1 => "M1 no breakpoints",
            Mut::M2 => "M2 one sample per piece",
            Mut::M3 => "M3 D samples not D+1",
            Mut::M4 => "M4 linear scan capped at 64",
        };
        println!(
            "{:<30} mismatches vs sweep: {:>5}   (true {}, false {})",
            name, mis, t, f
        );
    }

    // And at the shipped width, where it matters: does the mutant differ there?
    println!("\nwidth-64 verdicts per mutant on the E_d family:");
    for (label, a, b) in [
        ("E_63", pow_term(63), pow_term(64)),
        ("E_64", pow_term(64), pow_term(65)),
    ] {
        print!("  {}: ", label);
        for mu in [Mut::M0, Mut::M1, Mut::M2, Mut::M3, Mut::M4] {
            let (v, _, _) = verdict(&a, &b, 64, mu);
            print!("{} ", if v { "T" } else { "F" });
        }
        println!(" (M0 M1 M2 M3 M4)");
    }
}
