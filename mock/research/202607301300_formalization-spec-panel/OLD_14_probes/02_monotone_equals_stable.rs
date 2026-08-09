// Probe 02: is "monotone" the same SET of maps as "translation-stable",
// not merely the same cardinality? File 13's probe 01 found, on one model
// (representable [-2,1], exact [-6,5]): 65536 total maps, 1 monotone
// (clamp), 1 translation-stable, and did not check whether they are the
// SAME map (as opposed to two different singleton sets that happen to
// both have size 1). This probe checks the identity directly, decodes
// every found map so the answer is inspectable, and repeats the check on
// three further, differently-shaped models to see whether any coincidence
// survives a shape change.
//
// A recovery map phi: EXACT -> REPRESENTABLE is total and fixes every
// representable point (phi(r) = r for r representable). The free points
// are EXACT \ REPRESENTABLE; each is independently assigned one of
// |REPRESENTABLE| values, giving |REPRESENTABLE| ^ |free| maps. EXACT is
// sized [3*rep_lo, 3*rep_hi] so that every quantity the stability check
// evaluates (an exact sum of two representables, translated by a third
// representable, twice) stays inside it; the code asserts this rather
// than trusting the arithmetic.
//
// monotone(phi): x <= y implies phi(x) <= phi(y), for all x, y in EXACT.
// stable(phi): for every EXACT SUM x = a + b (a, b representable) and
// every representable c: phi(phi(x) + c) == phi(x + c). This matches the
// draft's own wording (11_current_shape_draft.md:257-258): "for every
// exact sum x and every representable c". x ranges over exact sums of
// two representables, not over the whole EXACT domain; an earlier draft
// of this probe conflated the two and made the check vacuous (x
// representable means phi(x) = x by construction, so phi(x)+c == x+c
// trivially and every map passed).
//
// Run: rustc -O 02_monotone_equals_stable.rs -o /tmp/mono_stable && /tmp/mono_stable

struct Model {
    rep_lo: i64,
    rep_hi: i64,
}

fn run_model(name: &str, m: &Model) {
    let rep: Vec<i64> = (m.rep_lo..=m.rep_hi).collect();
    let exact_lo = 3 * m.rep_lo;
    let exact_hi = 3 * m.rep_hi;
    let exact: Vec<i64> = (exact_lo..=exact_hi).collect();
    let is_rep = |x: i64| x >= m.rep_lo && x <= m.rep_hi;
    let free: Vec<i64> = exact.iter().cloned().filter(|x| !is_rep(*x)).collect();

    let mut exact_sums: Vec<i64> = Vec::new();
    for &a in &rep {
        for &b in &rep {
            let s = a + b;
            if !exact_sums.contains(&s) {
                exact_sums.push(s);
            }
        }
    }

    let n_rep = rep.len() as u64;
    let n_free = free.len() as u32;
    let total_maps = n_rep.pow(n_free);

    println!(
        "{name}: representable=[{}, {}] ({} values), exact=[{}, {}] ({} values), free points={}, maps to search={total_maps}",
        m.rep_lo, m.rep_hi, rep.len(), exact_lo, exact_hi, exact.len(), free.len()
    );

    for &x in &exact_sums {
        for &c in &rep {
            assert!(
                x + c >= exact_lo && x + c <= exact_hi,
                "margin too small: x+c out of range"
            );
        }
    }

    let decode = |id: u64| -> Vec<i64> {
        let mut assign = id;
        let mut phi_free: Vec<i64> = Vec::with_capacity(free.len());
        for _ in 0..n_free {
            let digit = (assign % n_rep) as usize;
            phi_free.push(rep[digit]);
            assign /= n_rep;
        }
        exact
            .iter()
            .map(|&x| {
                if is_rep(x) {
                    x
                } else {
                    phi_free[free.iter().position(|&f| f == x).unwrap()]
                }
            })
            .collect()
    };

    let mut monotone_ids: Vec<u64> = Vec::new();
    let mut stable_ids: Vec<u64> = Vec::new();

    for id in 0..total_maps {
        let table = decode(id);
        let phi = |x: i64| table[(x - exact_lo) as usize];

        let monotone = table.windows(2).all(|w| w[0] <= w[1]);
        if monotone {
            monotone_ids.push(id);
        }

        let stable = exact_sums
            .iter()
            .all(|&x| rep.iter().all(|&c| phi(phi(x) + c) == phi(x + c)));
        if stable {
            stable_ids.push(id);
        }
    }

    println!(
        "  monotone maps: {} ids={:?}",
        monotone_ids.len(),
        monotone_ids
    );
    println!("  stable maps:   {} ids={:?}", stable_ids.len(), stable_ids);
    println!("  identical sets: {}", monotone_ids == stable_ids);

    let show = |label: &str, ids: &[u64]| {
        for &id in ids.iter().take(6) {
            println!("    {label} id={id}: {:?} -> {:?}", exact, decode(id));
        }
    };
    show("monotone", &monotone_ids);
    show("stable", &stable_ids);
    println!();
}

fn main() {
    run_model(
        "A (signed, [-2,1], matches file13 probe01's representable range)",
        &Model {
            rep_lo: -2,
            rep_hi: 1,
        },
    );
    run_model(
        "B (signed, [-1,1], smaller, different parity)",
        &Model {
            rep_lo: -1,
            rep_hi: 1,
        },
    );
    run_model(
        "C (unsigned, [0,3], UFixed-shaped domain)",
        &Model {
            rep_lo: 0,
            rep_hi: 3,
        },
    );
    run_model(
        "D (signed, [-3,2], asymmetric, larger)",
        &Model {
            rep_lo: -3,
            rep_hi: 2,
        },
    );
}
