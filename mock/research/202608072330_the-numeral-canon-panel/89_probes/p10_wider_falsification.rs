// PROBE p10 (file 89). A wider falsification search against Theorem C, aimed
// at this file's own least-certain item: the theorem is proved at every arity
// and measured at arities 2 and 3, with p4's systematic search running only at
// depth 2 and widths 2 and 3.
//
// A counterexample to Theorem C is a pair of monotone unsigned saturating
// terms that AGREE on the degree box and DISAGREE somewhere in the domain.
// This probe enumerates deeper terms, dedupes them by value table so the count
// is of functions rather than spellings, and checks every pair.
//
// It also reports the branch counts, because a search that only ever produces
// pairs with an empty clamped set is searching the trivial half: when nothing
// clamps inside the box, agreement on the box forces polynomial identity and
// the theorem is the interpolation lemma. The interesting pairs are the ones
// that agree on the box WITH a nonempty clamped set, and those are counted
// separately.
//
// Runtime spike; std/Vec/Box are scaffolding, not design shape.

fn umax(w: u32) -> u128 {
    (1u128 << w) - 1
}

#[derive(Clone)]
enum T {
    V(usize),
    C(u64),
    Add(Box<T>, Box<T>),
    Mul(Box<T>, Box<T>),
}

impl T {
    fn sat(&self, xs: &[u128], w: u32) -> u128 {
        let m = umax(w);
        match self {
            T::V(i) => xs[*i],
            T::C(c) => {
                let c = *c as u128;
                if c > m {
                    m
                } else {
                    c
                }
            }
            T::Add(a, b) => {
                let s = a.sat(xs, w) + b.sat(xs, w);
                if s > m {
                    m
                } else {
                    s
                }
            }
            T::Mul(a, b) => {
                let s = a.sat(xs, w) * b.sat(xs, w);
                if s > m {
                    m
                } else {
                    s
                }
            }
        }
    }
    fn pdeg(&self, k: usize) -> Vec<u64> {
        match self {
            T::V(i) => {
                let mut v = vec![0; k];
                v[*i] = 1;
                v
            }
            T::C(_) => vec![0; k],
            T::Add(a, b) => {
                let (x, y) = (a.pdeg(k), b.pdeg(k));
                (0..k).map(|i| x[i].max(y[i])).collect()
            }
            T::Mul(a, b) => {
                let (x, y) = (a.pdeg(k), b.pdeg(k));
                (0..k).map(|i| x[i] + y[i]).collect()
            }
        }
    }
    fn depth(&self) -> u32 {
        match self {
            T::V(_) | T::C(_) => 0,
            T::Add(a, b) | T::Mul(a, b) => 1 + a.depth().max(b.depth()),
        }
    }
    fn show(&self) -> String {
        match self {
            T::V(i) => (if *i == 0 {
                "x"
            } else if *i == 1 {
                "y"
            } else {
                "z"
            })
            .into(),
            T::C(c) => format!("{}", c),
            T::Add(a, b) => format!("({}+{})", a.show(), b.show()),
            T::Mul(a, b) => format!("({}*{})", a.show(), b.show()),
        }
    }
}

fn points(k: usize, bounds: &[u128]) -> Vec<Vec<u128>> {
    let mut out = vec![vec![]];
    for i in 0..k {
        let mut nxt = Vec::new();
        for pre in &out {
            for v in 0..=bounds[i] {
                let mut p = pre.clone();
                p.push(v);
                nxt.push(p);
            }
        }
        out = nxt;
    }
    out
}

fn main() {
    println!("p10: a wider falsification search against Theorem C\n");

    for (k, depth, widths, cap) in [
        (2usize, 3u32, vec![2u32, 3, 4], 700usize),
        (3, 2, vec![2u32, 3], 400),
    ] {
        // enumerate terms
        let mut atoms: Vec<T> = (0..k).map(|i| T::V(i)).collect();
        for c in [0u64, 1, 2, 3, 5] {
            atoms.push(T::C(c));
        }
        // build level by level, deduping by value table at each level so the
        // combinatorial explosion is cut where it starts rather than after.
        let wtop0 = *widths.last().unwrap();
        let all0 = points(k, &vec![umax(wtop0); k]);
        let tab = |t: &T| -> Vec<u128> { all0.iter().map(|p| t.sat(p, wtop0)).collect() };
        let mut seen0: std::collections::HashSet<Vec<u128>> = std::collections::HashSet::new();
        let mut cur: Vec<T> = Vec::new();
        for a in &atoms {
            if seen0.insert(tab(a)) {
                cur.push(a.clone());
            }
        }
        let level_cap = if k == 2 { 260usize } else { 130 };
        for _ in 0..depth {
            let mut nxt = cur.clone();
            'outer: for a in &cur {
                for b in &cur {
                    for t in [
                        T::Add(Box::new(a.clone()), Box::new(b.clone())),
                        T::Mul(Box::new(a.clone()), Box::new(b.clone())),
                    ] {
                        let pd = t.pdeg(k);
                        if pd.iter().any(|&d| d > 6) {
                            continue;
                        }
                        if seen0.insert(tab(&t)) {
                            nxt.push(t);
                        }
                        if nxt.len() >= level_cap {
                            break 'outer;
                        }
                    }
                }
            }
            cur = nxt;
        }
        let wtop = *widths.last().unwrap();
        let terms: Vec<T> = cur.into_iter().take(cap).collect();

        println!("k = {}, terms of depth <= {}, deduped by value table at width {}: {} distinct functions",
                 k, depth, wtop, terms.len());
        for &w in &widths {
            let m = umax(w);
            let dom = points(k, &vec![m; k]);
            let (mut pairs, mut agree_box, mut agree_box_clamped, mut falsify) =
                (0u64, 0u64, 0u64, 0u64);
            let mut example: Option<(String, String)> = None;
            for i in 0..terms.len() {
                for j in (i + 1)..terms.len() {
                    let (a, b) = (&terms[i], &terms[j]);
                    let pa = a.pdeg(k);
                    let pb = b.pdeg(k);
                    let d: Vec<u128> = (0..k).map(|t| (pa[t].max(pb[t]) as u128).min(m)).collect();
                    let bx = points(k, &d);
                    pairs += 1;
                    let mut ok = true;
                    let mut clamped = false;
                    for p in &bx {
                        let (va, vb) = (a.sat(p, w), b.sat(p, w));
                        if va != vb {
                            ok = false;
                            break;
                        }
                        if va == m && vb == m {
                            clamped = true;
                        }
                    }
                    if !ok {
                        continue;
                    }
                    agree_box += 1;
                    if clamped {
                        agree_box_clamped += 1;
                    }
                    for p in &dom {
                        if a.sat(p, w) != b.sat(p, w) {
                            falsify += 1;
                            if example.is_none() {
                                example = Some((a.show(), b.show()));
                            }
                            break;
                        }
                    }
                }
            }
            println!("  width {}: {} pairs, {} agree on the box, {} of those with a clamped box point, {} COUNTEREXAMPLES",
                     w, pairs, agree_box, agree_box_clamped, falsify);
            assert!(
                falsify == 0,
                "THEOREM C REFUTED at k={} w={}: {:?}",
                k,
                w,
                example
            );
            if w == wtop {
                // the terms were deduped by their value table at wtop, so EVERY
                // pair differs somewhere in the domain at this width. The theorem's
                // contrapositive therefore predicts every pair differs on the box,
                // and "agree on the box" must be exactly zero.
                assert!(agree_box == 0,
                    "at the dedup width every pair differs in the domain, so any pair agreeing on the box is a counterexample");
                println!("    (dedup width: every pair differs in the domain by construction, so the {} zero above",
                         pairs);
                println!("     is the theorem's contrapositive tested at full strength)");
            } else {
                assert!(
                    agree_box_clamped > 0,
                    "the search never entered the clamped branch at k={} w={}",
                    k,
                    w
                );
            }
        }
        println!();
    }
    println!("all checks passed");
}
