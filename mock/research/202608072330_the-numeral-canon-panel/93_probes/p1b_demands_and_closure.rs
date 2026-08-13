//! P1b. The sharper form of P1, after P1 refuted my prediction.
//!
//! P1 predicted no semilattice on four markers would survive the intents.
//! Nine did. So the flat-set question is not settled by the three laws alone,
//! and the real constraint has to be stated differently.
//!
//! HYPOTHESIS: a resolution must not silently discard a guarantee the consumer
//! explicitly asked for. `arvo-toolbox-not-policer.md` states this directly, in
//! its list of things the substrate does NOT provide: "Default selections that
//! quietly change semantics (auto-resolve to a more conservative strategy
//! without flagging)."
//!
//! Model each strategy as the SET OF DEMANDS it carries, and resolution as the
//! demand set the result must satisfy, which is the union: mixing a value whose
//! type demands minimal storage with one whose type demands maximal speed
//! produces something that has been asked for both. Then:
//!
//!   Resolve is total and loses nothing  <=>  the strategy set is closed
//!                                            under union of demands.
//!
//! Part A re-runs P1's enumeration with the full constraint set, including the
//! three constraints P1 left out (Warm against each of the other three).
//!
//! Part B checks closure under demand-union for three candidate strategy sets:
//! four singleton-demand markers, the full powerset of demands, and a product
//! of per-axis chains.
//!
//! Run: rustc --edition 2024 -O p1b_demands_and_closure.rs -o /tmp/p1b && /tmp/p1b

const N: usize = 4;
const HOT: usize = 0;
const WARM: usize = 1;
const COLD: usize = 2;
const PRECISE: usize = 3;
const NAMES: [&str; N] = ["Hot", "Warm", "Cold", "Precise"];
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

fn table_from(choices: [usize; 6]) -> [[usize; N]; N] {
    let mut t = [[0usize; N]; N];
    for i in 0..N {
        t[i][i] = i;
    }
    for (k, &(a, b)) in PAIRS.iter().enumerate() {
        t[a][b] = choices[k];
        t[b][a] = choices[k];
    }
    t
}

fn is_associative(t: &[[usize; N]; N]) -> bool {
    for a in 0..N {
        for b in 0..N {
            for c in 0..N {
                if t[t[a][b]][c] != t[a][t[b][c]] {
                    return false;
                }
            }
        }
    }
    true
}

struct Constraint {
    name: &'static str,
    why: &'static str,
    holds: fn(&[[usize; N]; N]) -> bool,
}

const CONSTRAINTS: [Constraint; 6] = [
    Constraint {
        name: "K1 Resolve(Hot,Precise) = Precise",
        why: "toolbox-not-policer's own worked example; accuracy is not lost to speed",
        holds: |t| t[HOT][PRECISE] == PRECISE,
    },
    Constraint {
        name: "K2 Resolve(Cold,Hot) != Hot",
        why: "I17: the storage-minimising path is not deprioritised",
        holds: |t| t[COLD][HOT] != HOT,
    },
    Constraint {
        name: "K3 Resolve(Cold,Hot) != Cold",
        why: "I5: Hot's intent is performance, and must be reachable in expressions",
        holds: |t| t[COLD][HOT] != COLD,
    },
    Constraint {
        name: "K4 Resolve(Warm,Precise) = Precise",
        why: "same argument as K1, against the default rather than against Hot",
        holds: |t| t[WARM][PRECISE] == PRECISE,
    },
    Constraint {
        name: "K5 Resolve(Warm,Cold) != Warm",
        why: "I17 again: cold storage is not silently traded for the default",
        holds: |t| t[WARM][COLD] != WARM,
    },
    Constraint {
        name: "K6 Resolve(Warm,Hot) != Warm",
        why: "I5 again: speed is not silently traded for the default",
        holds: |t| t[WARM][HOT] != WARM,
    },
];

fn part_a() {
    let mut semilattices: Vec<[[usize; N]; N]> = Vec::new();
    let mut choices = [0usize; 6];
    'outer: loop {
        let t = table_from(choices);
        if is_associative(&t) {
            semilattices.push(t);
        }
        let mut i = 0;
        loop {
            if i == 6 {
                break 'outer;
            }
            choices[i] += 1;
            if choices[i] < N {
                break;
            }
            choices[i] = 0;
            i += 1;
        }
    }

    println!("PART A. Flat four-marker set under the full constraint list");
    println!("===========================================================");
    println!();
    println!("join semilattices on 4 labelled elements : {}", semilattices.len());
    println!();
    // Cumulative survival, in order.
    let mut live: Vec<&[[usize; N]; N]> = semilattices.iter().collect();
    for c in CONSTRAINTS.iter() {
        live.retain(|t| (c.holds)(t));
        println!("  after {:<36} : {:>3} survive   ({})", c.name, live.len(), c.why);
    }
    println!();
    if live.is_empty() {
        println!("RESULT: no join semilattice on these four markers satisfies all six.");
        println!("So on a flat four-element set, SOME cross-strategy resolution");
        println!("silently discards an intent the consumer asked for, whichever");
        println!("resolution table is chosen. That is a property of the SET, not of");
        println!("the table, and it is not fixable by picking a better table.");
    } else {
        println!("RESULT: {} table(s) survive all six:", live.len());
        for t in live {
            println!("  HvC={} HvW={} CvW={} CvP={} WvP={} HvP={}",
                NAMES[t[HOT][COLD]], NAMES[t[HOT][WARM]], NAMES[t[COLD][WARM]],
                NAMES[t[COLD][PRECISE]], NAMES[t[WARM][PRECISE]], NAMES[t[HOT][PRECISE]]);
        }
    }
    // Isolate which single constraint pair is unsatisfiable, so the finding is
    // located rather than merely reported.
    println!();
    println!("Located: which minimal subsets are already unsatisfiable?");
    for a in 0..CONSTRAINTS.len() {
        for b in (a + 1)..CONSTRAINTS.len() {
            let n = semilattices
                .iter()
                .filter(|t| (CONSTRAINTS[a].holds)(t) && (CONSTRAINTS[b].holds)(t))
                .count();
            if n == 0 {
                println!("  UNSAT already at {{{}, {}}}", CONSTRAINTS[a].name, CONSTRAINTS[b].name);
            }
        }
    }
}

// ---------------------------------------------------------------------------

/// Demands a consumer can place on a numeral, one bit each. Illustrative, and
/// the argument does not depend on this exact list: it depends only on there
/// being more than one and on the presets carrying different ones.
const D_SPEED: u8 = 1 << 0;
const D_SPACE: u8 = 1 << 1;
const D_ACCURACY: u8 = 1 << 2;
const D_FAMILIARITY: u8 = 1 << 3;
const D_ALL: u8 = D_SPEED | D_SPACE | D_ACCURACY | D_FAMILIARITY;

fn demand_names(d: u8) -> String {
    let mut v: Vec<&str> = Vec::new();
    if d & D_SPEED != 0 { v.push("speed"); }
    if d & D_SPACE != 0 { v.push("space"); }
    if d & D_ACCURACY != 0 { v.push("accuracy"); }
    if d & D_FAMILIARITY != 0 { v.push("familiarity"); }
    if v.is_empty() { "{}".to_string() } else { format!("{{{}}}", v.join(",")) }
}

/// A set is closed under demand-union when for every pair of members, the
/// union of their demands is itself a member's demand set.
fn closure_report(label: &str, set: &[(String, u8)]) {
    let mut missing: Vec<(String, String, u8)> = Vec::new();
    for (na, a) in set {
        for (nb, b) in set {
            let u = a | b;
            if !set.iter().any(|(_, d)| *d == u) {
                missing.push((na.clone(), nb.clone(), u));
            }
        }
    }
    println!("  {label}: {} members, {} unresolvable pairs", set.len(), missing.len());
    for (a, b, u) in missing.iter().take(6) {
        println!("      {a} v {b} needs {} and no member carries it", demand_names(*u));
    }
    if missing.len() > 6 {
        println!("      ... and {} more", missing.len() - 6);
    }
}

fn part_b() {
    println!();
    println!("PART B. Closure under demand-union");
    println!("==================================");
    println!();
    println!("Resolution loses nothing exactly when the strategy set is closed");
    println!("under union of the demands its members carry.");
    println!();

    let flat: Vec<(String, u8)> = vec![
        ("Hot".into(), D_SPEED),
        ("Cold".into(), D_SPACE),
        ("Precise".into(), D_ACCURACY),
        ("Warm".into(), D_FAMILIARITY),
    ];
    closure_report("four singleton-demand markers", &flat);

    let powerset: Vec<(String, u8)> = (0..=D_ALL).map(|d| (demand_names(d), d)).collect();
    closure_report("full powerset of demands", &powerset);

    println!();
    println!("The powerset is closed by construction, which is the point: closure");
    println!("is not something a marker set can be given, it is what a LATTICE is.");
    println!("A named preset is then a point in it and naming more costs nothing");
    println!("structural, which is what I1 says the design needs.");
    println!();
    println!("Size of the smallest closed set containing the four singletons:");
    let mut closed: Vec<u8> = flat.iter().map(|(_, d)| *d).collect();
    loop {
        let mut added = false;
        let snapshot = closed.clone();
        for a in &snapshot {
            for b in &snapshot {
                let u = a | b;
                if !closed.contains(&u) {
                    closed.push(u);
                    added = true;
                }
            }
        }
        if !added {
            break;
        }
    }
    closed.sort_unstable();
    println!("  {} elements (from 4), namely:", closed.len());
    for d in &closed {
        println!("    {}", demand_names(*d));
    }
    println!();
    println!("So closing the four presets under their own resolution generates");
    println!("{} strategies. Either the design carries them, or resolution is not", closed.len());
    println!("a join and has to be something else.");
}

fn main() {
    part_a();
    part_b();
}
