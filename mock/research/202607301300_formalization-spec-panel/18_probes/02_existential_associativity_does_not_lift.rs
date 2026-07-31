//! PROBE 2: existential (weak) associativity at arity 3 does NOT imply that all
//! defined groupings of a 4-element fold agree.
//!
//! WHY THIS MATTERS FOR THE SPEC
//!
//! File 17 measured that `Precise` has regrouping diameter 0 and 10992
//! grouping-dependent refusals, and named the law it needs "partially
//! associative". Every statement of that notion in the review so far is the
//! BINARY one: for all a, b, c, if both `(a.b).c` and `a.(b.c)` are defined then
//! they are equal. Call that EXISTENTIAL ASSOCIATIVITY.
//!
//! The thing a combinator actually needs is the N-ARY one: over the Catalan-many
//! groupings of an n-element fold, any two that are defined agree. For a TOTAL
//! operation these are the same statement, because the standard generalised-
//! associativity proof rewrites any grouping into any other through a chain of
//! single applications of the binary law.
//!
//! For a PARTIAL operation that proof does not go through, because every step of
//! the chain passes through an intermediate grouping, and an intermediate that is
//! undefined breaks the chain. Two groupings at the ends of a broken chain can be
//! defined and disagree while every binary instance holds.
//!
//! So the question this probe answers by exhaustion, not by argument: is the
//! binary law actually weaker than the n-ary one, or does the gap close for some
//! reason the sketch above misses?
//!
//! WHAT IS SEARCHED
//!
//! Every partial magma on a 3-element carrier: 9 table cells, each holding one of
//! {undefined, 0, 1, 2}, so 4^9 = 262144 tables. For each, check binary
//! existential associativity over all 27 triples, and if it holds, check all five
//! groupings of every one of the 81 quadruples for a defined disagreement.
//!
//! Build:  rustc -O 02_existential_associativity_does_not_lift.rs -o p2 && ./p2

const K: usize = 3; // carrier size
type Cell = Option<u8>;

#[derive(Clone, Copy)]
struct Magma {
    t: [Cell; K * K],
}

impl Magma {
    #[inline]
    fn op(&self, a: Cell, b: Cell) -> Cell {
        match (a, b) {
            (Some(x), Some(y)) => self.t[(x as usize) * K + (y as usize)],
            _ => None,
        }
    }
}

/// Binary existential associativity: whenever both bracketings are defined they
/// agree. Undefined on either side imposes nothing.
fn existentially_associative(m: &Magma) -> bool {
    for a in 0..K as u8 {
        for b in 0..K as u8 {
            for c in 0..K as u8 {
                let l = m.op(m.op(Some(a), Some(b)), Some(c));
                let r = m.op(Some(a), m.op(Some(b), Some(c)));
                if let (Some(x), Some(y)) = (l, r) {
                    if x != y {
                        return false;
                    }
                }
            }
        }
    }
    true
}

/// Kleene (strong) associativity: definedness agrees too.
fn kleene_associative(m: &Magma) -> bool {
    for a in 0..K as u8 {
        for b in 0..K as u8 {
            for c in 0..K as u8 {
                let l = m.op(m.op(Some(a), Some(b)), Some(c));
                let r = m.op(Some(a), m.op(Some(b), Some(c)));
                if l != r {
                    return false;
                }
            }
        }
    }
    true
}

/// The five groupings of four elements, in Tamari order:
///   0: ((ab)c)d   1: (a(bc))d   2: a((bc)d)   3: a(b(cd))   4: (ab)(cd)
fn groupings4(m: &Magma, a: u8, b: u8, c: u8, d: u8) -> [Cell; 5] {
    let (a, b, c, d) = (Some(a), Some(b), Some(c), Some(d));
    [
        m.op(m.op(m.op(a, b), c), d),
        m.op(m.op(a, m.op(b, c)), d),
        m.op(a, m.op(m.op(b, c), d)),
        m.op(a, m.op(b, m.op(c, d))),
        m.op(m.op(a, b), m.op(c, d)),
    ]
}

fn quad_disagreement(m: &Magma) -> Option<(u8, u8, u8, u8, [Cell; 5])> {
    for a in 0..K as u8 {
        for b in 0..K as u8 {
            for c in 0..K as u8 {
                for d in 0..K as u8 {
                    let g = groupings4(m, a, b, c, d);
                    let defined: Vec<u8> = g.iter().filter_map(|x| *x).collect();
                    if defined.len() >= 2 && defined.iter().any(|v| *v != defined[0]) {
                        return Some((a, b, c, d, g));
                    }
                }
            }
        }
    }
    None
}

fn show(c: Cell) -> String {
    match c {
        None => "_".to_string(),
        Some(v) => v.to_string(),
    }
}

fn main() {
    let total = 4usize.pow((K * K) as u32);
    println!("exhaustive search over every partial magma on {K} elements");
    println!("tables searched: {total}\n");

    let mut n_exist = 0usize;
    let mut n_kleene = 0usize;
    let mut n_counterexample = 0usize;
    let mut first: Option<(Magma, (u8, u8, u8, u8, [Cell; 5]))> = None;

    for code in 0..total {
        let mut t = [None; K * K];
        let mut c = code;
        for cell in t.iter_mut() {
            let d = c % 4;
            c /= 4;
            *cell = if d == 0 { None } else { Some((d - 1) as u8) };
        }
        let m = Magma { t };

        if !existentially_associative(&m) {
            continue;
        }
        n_exist += 1;
        if kleene_associative(&m) {
            n_kleene += 1;
        }
        if let Some(w) = quad_disagreement(&m) {
            n_counterexample += 1;
            if first.is_none() {
                first = Some((m, w));
            }
        }
    }

    println!("existentially associative (binary):            {n_exist}");
    println!("  of those, Kleene associative (binary):       {n_kleene}");
    println!("  of those, with a DEFINED disagreement at 4:  {n_counterexample}");
    println!();

    match first {
        None => {
            println!("NO COUNTEREXAMPLE at carrier size {K}.");
            println!("The binary law would then be enough at this size, and the");
            println!("lifting question needs a larger carrier before it is settled.");
        }
        Some((m, (a, b, c, d, g))) => {
            println!("WITNESS. Binary existential associativity holds everywhere,");
            println!("and two groupings of four elements are both defined and differ.\n");
            println!("  operation table (rows = left operand, `_` = undefined):");
            print!("        ");
            for j in 0..K {
                print!("{j:>4}");
            }
            println!();
            for i in 0..K {
                print!("    {i:>3} ");
                for j in 0..K {
                    print!("{:>4}", show(m.t[i * K + j]));
                }
                println!();
            }
            println!();
            println!("  input: a={a} b={b} c={c} d={d}");
            let names = ["((ab)c)d", "(a(bc))d", "a((bc)d)", "a(b(cd))", "(ab)(cd)"];
            for (n, v) in names.iter().zip(g.iter()) {
                println!("    {n:<10} = {}", show(*v));
            }
            println!();
            println!("  the chain is broken exactly where the review's own argument said");
            println!("  it would be: the intermediate groupings that would connect the two");
            println!("  defined ends are themselves undefined, so no sequence of binary");
            println!("  rewrites reaches from one to the other.");
        }
    }

    println!();
    println!("WHAT THIS SETTLES, AND WHAT IT DOES NOT");
    println!();
    println!("Settles: 'partially associative', stated as the binary law, is strictly");
    println!("weaker than 'every defined grouping of an n-element fold agrees'. A design");
    println!("that states the binary law and uses it to license an n-way accumulator");
    println!("split has an unproven step, and the arity the law is quantified over has");
    println!("to be written down.");
    println!();
    println!("Does not settle: whether arvo's own `Refuse` addition is in the bad region.");
    println!("It is not, and probe 3 shows the reason is a property of its recovery map");
    println!("rather than anything the binary law says.");
}
