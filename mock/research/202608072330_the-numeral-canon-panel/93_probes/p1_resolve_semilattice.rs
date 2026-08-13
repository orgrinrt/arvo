//! P1. What can relate two strategies, exhaustively.
//!
//! HYPOTHESIS: a binary `Resolve<S1, S2>` on a flat marker set is usable in a
//! type system only if it is commutative, associative and idempotent, because
//! otherwise `x + y` and `y + x`, or `(x+y)+z` and `x+(y+z)`, have different
//! TYPES. Those three laws together are exactly a join semilattice. So the
//! question "what relates two strategies" has a forced answer: a semilattice,
//! or nothing.
//!
//! This probe enumerates every commutative idempotent binary operation on a
//! four-element set (4^6 = 4096 of them, one choice per unordered off-diagonal
//! pair), filters for associativity, and then reports what each surviving
//! semilattice is forced to decide for the pairs the intents constrain.
//!
//! Constraints taken from INTENTS.md, and each is cited in the output:
//!   K1  Resolve(Hot, Precise) = Precise.
//!       arvo-toolbox-not-policer.md's own worked example: "Hot wrapping +
//!       Precise saturating -> Precise".
//!   K2  Resolve(Cold, Hot) != Hot.
//!       I17: the storage-minimising path is not deprioritised.
//!   K3  Resolve(Cold, Hot) != Cold.
//!       I5: Hot's intent is performance; if contact with cold storage always
//!       wins, Hot's intent is unreachable in any expression touching storage.
//!
//! K2 and K3 together say Hot v Cold must be a THIRD element. The probe reports
//! whether any semilattice on four elements can satisfy all three, and what it
//! costs when it does.
//!
//! Run: rustc --edition 2024 -O p1_resolve_semilattice.rs -o /tmp/p1 && /tmp/p1

const N: usize = 4;
const HOT: usize = 0;
const WARM: usize = 1;
const COLD: usize = 2;
const PRECISE: usize = 3;
const NAMES: [&str; N] = ["Hot", "Warm", "Cold", "Precise"];

/// A commutative idempotent binary op is fixed by its six off-diagonal
/// unordered pairs. Diagonal is forced by idempotence.
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

fn table_from(choices: [usize; 6]) -> [[usize; N]; N] {
    let mut t = [[0usize; N]; N];
    for i in 0..N {
        t[i][i] = i; // idempotent
    }
    for (k, &(a, b)) in PAIRS.iter().enumerate() {
        t[a][b] = choices[k];
        t[b][a] = choices[k]; // commutative
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

/// Every commutative, associative, idempotent op induces a partial order
/// a <= b iff a v b == b. Report it so the reader can see the shape.
fn order_string(t: &[[usize; N]; N]) -> String {
    let mut s = String::new();
    for a in 0..N {
        for b in 0..N {
            if a != b && t[a][b] == b {
                if !s.is_empty() {
                    s.push_str(", ");
                }
                s.push_str(&format!("{}<={}", NAMES[a], NAMES[b]));
            }
        }
    }
    if s.is_empty() {
        s.push_str("(antichain: no two distinct elements comparable)");
    }
    s
}

fn main() {
    let mut total = 0usize;
    let mut semilattices = 0usize;
    let mut k1 = 0usize;
    let mut k1k2 = 0usize;
    let mut all_three = 0usize;
    let mut survivors: Vec<[[usize; N]; N]> = Vec::new();
    // What each semilattice satisfying K1 is forced to decide for (Hot, Cold).
    let mut hotcold_hist = [0usize; N];

    let mut choices = [0usize; 6];
    loop {
        total += 1;
        let t = table_from(choices);
        if is_associative(&t) {
            semilattices += 1;
            let c1 = t[HOT][PRECISE] == PRECISE;
            let c2 = t[COLD][HOT] != HOT;
            let c3 = t[COLD][HOT] != COLD;
            if c1 {
                k1 += 1;
                hotcold_hist[t[COLD][HOT]] += 1;
            }
            if c1 && c2 {
                k1k2 += 1;
            }
            if c1 && c2 && c3 {
                all_three += 1;
                survivors.push(t);
            }
        }
        // odometer over 4^6
        let mut i = 0;
        loop {
            if i == 6 {
                print_report(
                    total,
                    semilattices,
                    k1,
                    k1k2,
                    all_three,
                    &survivors,
                    &hotcold_hist,
                );
                return;
            }
            choices[i] += 1;
            if choices[i] < N {
                break;
            }
            choices[i] = 0;
            i += 1;
        }
    }
}

fn print_report(
    total: usize,
    semilattices: usize,
    k1: usize,
    k1k2: usize,
    all_three: usize,
    survivors: &[[[usize; N]; N]],
    hotcold_hist: &[usize; N],
) {
    println!("P1. Resolve on a flat four-marker set");
    println!("=====================================");
    println!();
    println!("commutative + idempotent binary ops on 4 elements : {total}");
    println!("of those, also associative (= join semilattices)  : {semilattices}");
    println!();
    println!("K1  Resolve(Hot, Precise) = Precise               : {k1} survive");
    println!("K2  and Resolve(Cold, Hot) != Hot   [I17]         : {k1k2} survive");
    println!("K3  and Resolve(Cold, Hot) != Cold  [I5]          : {all_three} survive");
    println!();
    println!("What K1-satisfying semilattices are FORCED to decide for Hot v Cold:");
    for i in 0..N {
        println!("  Hot v Cold = {:<8} : {} of {}", NAMES[i], hotcold_hist[i], k1);
    }
    println!();
    if survivors.is_empty() {
        println!("NO semilattice on these four markers satisfies all three.");
    } else {
        println!(
            "{} semilattice(s) satisfy all three. Each is listed with the partial",
            survivors.len()
        );
        println!("order it induces, so the reader can check it against the intents:");
        for (n, t) in survivors.iter().enumerate() {
            println!();
            println!("  --- survivor {} ---", n + 1);
            println!("  Hot v Cold    = {}", NAMES[t[HOT][COLD]]);
            println!("  Hot v Warm    = {}", NAMES[t[HOT][WARM]]);
            println!("  Cold v Warm   = {}", NAMES[t[COLD][WARM]]);
            println!("  Cold v Precise= {}", NAMES[t[COLD][PRECISE]]);
            println!("  Warm v Precise= {}", NAMES[t[WARM][PRECISE]]);
            println!("  order: {}", order_string(t));
        }
    }
    println!();
    println!("--- the same question on a PRODUCT of per-axis chains ---");
    product_check();
}

/// A strategy as a vector of per-axis policy choices, each axis a small chain
/// ordered by how much information the choice preserves. The join is
/// componentwise max, which is commutative, associative and idempotent for
/// free because max on a chain is. Checked exhaustively rather than asserted.
///
/// Axes and chains used here, and they are illustrative rather than proposed:
///   overflow  : wrap(0) < saturate(1) < widen(2)
///   rounding  : truncate(0) < nearest(1) < exact(2)
///   interm.   : narrow(0) < native(1) < wide(2)
const AX: usize = 3;
const CH: usize = 3;

fn join(a: [usize; AX], b: [usize; AX]) -> [usize; AX] {
    let mut r = [0usize; AX];
    for i in 0..AX {
        r[i] = if a[i] > b[i] { a[i] } else { b[i] };
    }
    r
}

fn product_check() {
    let mut all: Vec<[usize; AX]> = Vec::new();
    for a in 0..CH {
        for b in 0..CH {
            for c in 0..CH {
                all.push([a, b, c]);
            }
        }
    }
    let n = all.len();
    let mut comm = 0usize;
    let mut idem = 0usize;
    let mut assoc = 0usize;
    for &x in &all {
        if join(x, x) == x {
            idem += 1;
        }
        for &y in &all {
            if join(x, y) == join(y, x) {
                comm += 1;
            }
            for &z in &all {
                if join(join(x, y), z) == join(x, join(y, z)) {
                    assoc += 1;
                }
            }
        }
    }
    println!("elements in the product lattice        : {n}");
    println!("idempotence  holds on {idem}/{n}");
    println!("commutativity holds on {comm}/{}", n * n);
    println!("associativity holds on {assoc}/{}", n * n * n);
    println!();
    println!("The product needs no order to be chosen: every pair has a join by");
    println!("construction, so the four presets can be NAMED POINTS in it without");
    println!("the resolution question ever arising as a separate design decision.");
}
