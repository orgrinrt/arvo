//! Probe 01: is translation stability the same requirement as fold associativity?
//!
//! REPLACES an earlier file of this name that was left in this directory
//! untracked and does not terminate: it enumerated two length-12 nondecreasing
//! sequences over 8 values, which is C(19,7)^2 = 2.5e9 maps at 128 bytes each.
//! The model here is smaller on purpose so the exhaustion actually runs.
//!
//! The draft (11_current_shape_draft.md 3.4) derives the algebraic laws from
//! TRANSLATION STABILITY of the recovery map:
//!
//!     phi(phi(x) + c) == phi(x + c)   for every exact x and representable c
//!
//! A fold needs exactly one thing to survive regrouping:
//!
//!     op(op(a,b),c) == op(a,op(b,c))  where op(a,b) = phi(a + b)
//!
//! quantified over REPRESENTABLE a, b, c only. Stability quantifies over every
//! exact x, including exact values that are not the sum of two representables,
//! so it is at least as strong on its face. Two questions, answered by
//! exhaustion rather than by argument:
//!
//!   Q1. Do the two criteria agree on the shipped resolutions?
//!   Q2. Over every monotone total recovery map that fixes the representable
//!       points, how many are associative but not stable (the draft refuses a
//!       fold that is in fact well defined), and how many are stable but not
//!       associative (the draft's criterion would be unsound)?
//!
//! Run: rustc -O 01_stability_vs_associativity.rs -o /tmp/p01 && /tmp/p01

const MIN: i32 = -2;
const MAX: i32 = 1;
const EXACT_LO: i32 = -6;
const EXACT_HI: i32 = 5;

/// A recovery map. `None` is a refusal (the map is partial, per draft 3.4).
type Phi = fn(i32) -> Option<i32>;

fn clamp(x: i32) -> Option<i32> {
    Some(x.clamp(MIN, MAX))
}

fn wrap(x: i32) -> Option<i32> {
    let n = MAX - MIN + 1;
    Some(((x - MIN).rem_euclid(n)) + MIN)
}

fn substitute_zero(x: i32) -> Option<i32> {
    if x < MIN || x > MAX {
        Some(0)
    } else {
        Some(x)
    }
}

fn refuse(x: i32) -> Option<i32> {
    if x < MIN || x > MAX {
        None
    } else {
        Some(x)
    }
}

/// Clamp only above, refuse below. Not one of the shipped five; widens the
/// sample of hand-written maps to a mixed row.
fn clamp_hi_refuse_lo(x: i32) -> Option<i32> {
    if x > MAX {
        Some(MAX)
    } else if x < MIN {
        None
    } else {
        Some(x)
    }
}

fn representable() -> impl Iterator<Item = i32> {
    MIN..=MAX
}

fn exact() -> impl Iterator<Item = i32> {
    EXACT_LO..=EXACT_HI
}

/// phi(phi(x) + c) == phi(x + c), Kleene. A refusal on the inner phi
/// propagates: there is no value left to translate.
fn translation_stable(phi: Phi) -> Option<(i32, i32)> {
    for x in exact() {
        for c in representable() {
            let inner = match phi(x) {
                Some(v) => phi(v + c),
                None => None,
            };
            let outer = phi(x + c);
            if inner != outer {
                return Some((x, c));
            }
        }
    }
    None
}

/// op(a,b) = phi(a+b), lifted to Option so a refusal absorbs.
fn op(phi: Phi, a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => phi(x + y),
        _ => None,
    }
}

fn associative(phi: Phi) -> Option<(i32, i32, i32)> {
    for a in representable() {
        for b in representable() {
            for c in representable() {
                let l = op(phi, op(phi, Some(a), Some(b)), Some(c));
                let r = op(phi, Some(a), op(phi, Some(b), Some(c)));
                if l != r {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}

fn commutative(phi: Phi) -> Option<(i32, i32)> {
    for a in representable() {
        for b in representable() {
            if op(phi, Some(a), Some(b)) != op(phi, Some(b), Some(a)) {
                return Some((a, b));
            }
        }
    }
    None
}

fn report(name: &str, phi: Phi) {
    let s = translation_stable(phi);
    let a = associative(phi);
    let c = commutative(phi);
    let verdict = match (s, a) {
        (None, None) => String::from("both hold"),
        (Some(sx), None) => format!(
            "stability fails at x={} c={}, yet the fold IS associative: OVER-REFUSAL",
            sx.0, sx.1
        ),
        (None, Some(ax)) => format!("assoc fails at {:?} while stable: CRITERION UNSOUND", ax),
        (Some(sx), Some(ax)) => format!("stability fails at {:?}, assoc fails at {:?}", sx, ax),
    };
    println!(
        "{:<24} stable={:<5} assoc={:<5} commut={:<5}  {}",
        name,
        s.is_none(),
        a.is_none(),
        c.is_none(),
        verdict
    );
}

// ---------------------------------------------------------------------------
// Q2: exhaustive search over EVERY total recovery map that fixes the
// representable points and lands out-of-range values somewhere representable.
//
// A first cut of this probe restricted the family to monotone maps. That was
// wrong and proved nothing: a monotone total map that fixes [MIN, MAX] must
// send everything below MIN to MIN and everything above MAX to MAX, so the
// "monotone" family is the single map `clamp`. Wrapping and substitute-zero
// are both non-monotone, which is why the restricted search found them
// nowhere. The family below has no monotonicity constraint, so it contains
// every resolution the design ships plus every one nobody has written down.
// ---------------------------------------------------------------------------

fn enumerate_total_maps() -> Vec<Vec<i32>> {
    let n_out = ((MIN - EXACT_LO) + (EXACT_HI - MAX)) as usize;
    let reps: Vec<i32> = representable().collect();
    let k = reps.len();
    let total = k.pow(n_out as u32);
    let mut maps = Vec::with_capacity(total);
    for code in 0..total {
        let mut digits = Vec::with_capacity(n_out);
        let mut c = code;
        for _ in 0..n_out {
            digits.push(reps[c % k]);
            c /= k;
        }
        let mut m = Vec::with_capacity((EXACT_HI - EXACT_LO + 1) as usize);
        let split = (MIN - EXACT_LO) as usize;
        m.extend_from_slice(&digits[..split]); // EXACT_LO ..= MIN-1
        for v in MIN..=MAX {
            m.push(v); // identity on the representable set
        }
        m.extend_from_slice(&digits[split..]); // MAX+1 ..= EXACT_HI
        assert_eq!(m.len(), (EXACT_HI - EXACT_LO + 1) as usize);
        maps.push(m);
    }
    maps
}

fn table_monotone(m: &[i32]) -> bool {
    m.windows(2).all(|w| w[0] <= w[1])
}

fn table_stable(m: &[i32]) -> bool {
    let at = |x: i32| m[(x - EXACT_LO) as usize];
    for x in exact() {
        for c in representable() {
            let inner = at(x) + c;
            let xc = x + c;
            // Only compare where both arguments stay inside the modelled exact
            // domain; outside it the model says nothing.
            if inner < EXACT_LO || inner > EXACT_HI || xc < EXACT_LO || xc > EXACT_HI {
                continue;
            }
            if at(inner) != at(xc) {
                return false;
            }
        }
    }
    true
}

fn table_assoc(m: &[i32]) -> bool {
    let at = |x: i32| m[(x - EXACT_LO) as usize];
    for a in representable() {
        for b in representable() {
            let ab = at(a + b);
            for c in representable() {
                let bc = at(b + c);
                if at(ab + c) != at(a + bc) {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    println!(
        "model: representable [{}, {}], exact [{}, {}]\n",
        MIN, MAX, EXACT_LO, EXACT_HI
    );

    println!("Q1: the shipped resolutions");
    report("ReduceModulo (wrap)", wrap);
    report("Clamp (saturate)", clamp);
    report("SubstituteZero", substitute_zero);
    report("Refuse", refuse);
    println!("(signed model throughout: MIN < 0, so every row is the signed case)");
    report("ClampHi + RefuseLo", clamp_hi_refuse_lo);

    println!("\nQ2: exhaustive over every total map fixing the representable points");
    let maps = enumerate_total_maps();
    let mut n_stable = 0usize;
    let mut n_assoc = 0usize;
    let mut n_monotone = 0usize;
    let mut assoc_not_stable: Vec<&Vec<i32>> = Vec::new();
    let mut stable_not_assoc: Vec<&Vec<i32>> = Vec::new();
    for m in &maps {
        let s = table_stable(m);
        let a = table_assoc(m);
        if s {
            n_stable += 1;
        }
        if a {
            n_assoc += 1;
        }
        if table_monotone(m) {
            n_monotone += 1;
        }
        if a && !s {
            assoc_not_stable.push(m);
        }
        if s && !a {
            stable_not_assoc.push(m);
        }
    }
    println!("  maps searched:                {}", maps.len());
    println!(
        "  of which monotone:            {}   (the whole monotone family is `clamp`)",
        n_monotone
    );
    println!("  translation-stable:           {}", n_stable);
    println!("  fold-associative:             {}", n_assoc);
    println!(
        "  associative but NOT stable:   {}   <- refused by the draft, fine for a fold",
        assoc_not_stable.len()
    );
    println!(
        "  stable but NOT associative:   {}   <- would make the criterion unsound",
        stable_not_assoc.len()
    );

    let show = |label: &str, ms: &[&Vec<i32>]| {
        if let Some(w) = ms.first() {
            let mut s = String::new();
            for x in exact() {
                s.push_str(&format!("{}->{} ", x, w[(x - EXACT_LO) as usize]));
            }
            println!("\n  first {}:\n  {}", label, s.trim_end());
        }
    };
    show("associative-but-unstable map", &assoc_not_stable);
    show("stable-but-nonassociative map", &stable_not_assoc);
}
