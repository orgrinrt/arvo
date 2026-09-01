//! q2. The two repairs q1's findings ask for, derived and swept.
//!
//! q1 reported two defects and stopped there. This one attacks both.
//!
//! **Repair one: the additive-identity predicate.** Zero is a member of the
//! representable set exactly when some admitted slot `s` and magnitude `m` give
//! `PHASE_NUM/PHASE_DEN + s * radix^(SLOPE*m) = 0`. That is decidable without
//! enumeration: for each magnitude solve for `s` and ask whether it is an integer
//! in range. The repair is that predicate, and the check is a sweep against the
//! enumerated set, in both directions, over a parameter space that includes every
//! coordinate the trait admits and not only the ones the shipped points reach.
//!
//! **Repair two: denotation equality.** `ruling::the_format_spine_is_canon`
//! identifies a format by its ambient domain and its representable set, so the
//! question "are these two formats the same format" has an answer and the design
//! should be able to compute it. For the constant-quantum family it is three
//! comparisons. The check is every ordered pair in a swept pool, against
//! enumerated set equality.
//!
//! **The cases that must fail, stated before the run.**
//!
//! A mutant of repair one that tests only `PHASE_NUM == 0`, which is the shipped
//! predicate, must be reported as differing from the enumeration. If it is not,
//! the sweep never reaches a format where the two can differ and the section
//! establishes nothing.
//!
//! A mutant of repair two that compares only the step and the count, dropping the
//! lowest element, must be reported as differing from enumerated equality. If it
//! is not, the pool contains no two grids at one step and count with different
//! offsets, and the section establishes nothing.
//!
//! A third control: the pool must contain at least one pair that IS equal and at
//! least one that is NOT, or the equality decision is answering one way.
//!
//! **Bound on the bridge to the shipped crate.** This instrument is a model of the
//! coordinates rather than a link against `arvo-format`, because the sweep varies
//! coordinates that are const parameters and associated types over there. The
//! bridge is `q1`, which links the real crate and whose enumerator is this
//! formula: on all ten control rows the crate's own predicate and this formula
//! agreed. Everything below is about the coordinate space; that it is the same
//! space the crate carries rests on `q1`.
//!
//! Build: `rustc --edition 2024 -O q2_the_two_repairs_swept.rs -o /tmp/q2`

use std::collections::BTreeSet;

/// The coordinates a `Format` carries, as plain data.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Coords {
    radix: i128,
    base: i32,
    slope: i32,
    magnitudes: u32,
    min: i64,
    max: i64,
    phase_num: i64,
    phase_den: i64,
}

/// A value in units of the quantum at magnitude zero, reduced.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Q {
    num: i128,
    den: i128,
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 {
        1
    } else {
        a
    }
}

impl Q {
    fn new(num: i128, den: i128) -> Self {
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let g = gcd(num, den);
        Self {
            num: num / g,
            den: den / g,
        }
    }
}

/// The enumerated representable set, in units of `q(0)`. The oracle.
fn denotation(c: Coords) -> BTreeSet<Q> {
    let mut out = BTreeSet::new();
    for m in 0..c.magnitudes as i64 {
        let e = (c.slope as i64) * m;
        let (rn, rd) = if e >= 0 {
            (c.radix.pow(e as u32), 1i128)
        } else {
            (1i128, c.radix.pow((-e) as u32))
        };
        for s in c.min..=c.max {
            let s = s as i128;
            out.insert(Q::new(
                c.phase_num as i128 * rd + s * rn * c.phase_den as i128,
                c.phase_den as i128 * rd,
            ));
        }
    }
    out
}

// --- repair one ---------------------------------------------------------------

/// Whether zero is a member, decided rather than enumerated.
///
/// For each magnitude, `s * radix^d = -PHASE_NUM/PHASE_DEN` with `d = SLOPE*m`.
/// Multiplying out, `s` is an integer exactly when the divisibility below holds,
/// and the format has an additive identity exactly when some such `s` is in range.
///
/// Written in the shape a `const fn` over a `Format`'s associated items would take:
/// a bounded loop over the magnitudes and integer arithmetic, no allocation and no
/// enumeration of the set.
fn has_additive_identity_repaired(c: Coords) -> bool {
    let mut m: i64 = 0;
    while m < c.magnitudes as i64 {
        let d = (c.slope as i64) * m;
        // s = -phase_num / (phase_den * radix^d)   for d >= 0
        // s = -phase_num * radix^(-d) / phase_den  for d < 0
        let (numer, denom): (i128, i128) = if d >= 0 {
            (
                -(c.phase_num as i128),
                c.phase_den as i128 * c.radix.pow(d as u32),
            )
        } else {
            (
                -(c.phase_num as i128) * c.radix.pow((-d) as u32),
                c.phase_den as i128,
            )
        };
        if denom != 0 && numer % denom == 0 {
            let s = numer / denom;
            if s >= c.min as i128 && s <= c.max as i128 {
                return true;
            }
        }
        m += 1;
    }
    false
}

/// The shipped predicate, transcribed, as the mutant that must be reported.
///
/// `arvo-format/src/format.rs:has_additive_identity` is
/// `F::PHASE_NUM == 0 && slot_in_range::<F::Slots>(0)`.
fn has_additive_identity_shipped(c: Coords) -> bool {
    c.phase_num == 0 && 0 >= c.min && 0 <= c.max
}

fn zero_is_denoted(c: Coords) -> bool {
    denotation(c).iter().any(|q| q.num == 0)
}

// --- repair two ---------------------------------------------------------------

/// Whether two constant-quantum formats over one radix denote the same set,
/// decided rather than enumerated.
///
/// The set is an arithmetic progression, so three things decide it: how many
/// points, where the lowest one is, and how far apart they are. The count is
/// `max - min + 1`. The lowest value in absolute units is
/// `(PHASE_NUM/PHASE_DEN + MIN) * radix^BASE`. The step is `radix^BASE`.
///
/// The singleton case is separate on purpose: one point has no step, so two
/// singletons at different steps and one value are the same set.
fn same_set_constant_family(a: Coords, b: Coords) -> bool {
    debug_assert!(a.magnitudes == 1 && b.magnitudes == 1);
    if a.radix != b.radix {
        return false;
    }
    let count_a = a.max as i128 - a.min as i128 + 1;
    let count_b = b.max as i128 - b.min as i128 + 1;
    if count_a != count_b {
        return false;
    }
    if lowest_absolute(a) != lowest_absolute(b) {
        return false;
    }
    if count_a == 1 {
        // One point, and it has just been compared. No step to compare.
        return true;
    }
    a.base == b.base
}

/// The mutant: step and count only, offset dropped.
fn same_set_mutant_no_offset(a: Coords, b: Coords) -> bool {
    if a.radix != b.radix {
        return false;
    }
    let count_a = a.max as i128 - a.min as i128 + 1;
    let count_b = b.max as i128 - b.min as i128 + 1;
    count_a == count_b && a.base == b.base
}

/// The lowest value of a constant-family format, as an exact rational in absolute
/// units: `(PHASE_NUM/PHASE_DEN + MIN) * radix^BASE`.
fn lowest_absolute(c: Coords) -> Q {
    let unit_num = c.phase_num as i128 + c.min as i128 * c.phase_den as i128;
    let unit_den = c.phase_den as i128;
    if c.base >= 0 {
        Q::new(unit_num * c.radix.pow(c.base as u32), unit_den)
    } else {
        Q::new(unit_num, unit_den * c.radix.pow((-c.base) as u32))
    }
}

/// The absolute denotation, for the equality oracle. Constant family only.
fn denotation_absolute(c: Coords) -> BTreeSet<Q> {
    let mut out = BTreeSet::new();
    for s in c.min..=c.max {
        let unit_num = c.phase_num as i128 + s as i128 * c.phase_den as i128;
        let unit_den = c.phase_den as i128;
        out.insert(if c.base >= 0 {
            Q::new(unit_num * c.radix.pow(c.base as u32), unit_den)
        } else {
            Q::new(unit_num, unit_den * c.radix.pow((-c.base) as u32))
        });
    }
    out
}

fn main() {
    let mut findings = 0usize;

    // ---------------- section 1 ----------------
    println!("== section 1: the repaired additive-identity predicate, swept ==\n");

    let mut pool = Vec::new();
    for &radix in &[2i128, 10] {
        for base in -2i32..=2 {
            for &(slope, mags) in &[(0i32, 1u32), (1, 1), (1, 3), (2, 3), (-1, 3)] {
                for &(min, max) in &[
                    (-8i64, 7i64),
                    (0, 15),
                    (-7, 8),
                    (1, 8),
                    (-4, -1),
                    (3, 3),
                    (-6, 6),
                ] {
                    for phase_den in [1i64, 2, 3, 4] {
                        for phase_num in -8i64..=8 {
                            pool.push(Coords {
                                radix,
                                base,
                                slope,
                                magnitudes: mags,
                                min,
                                max,
                                phase_num,
                                phase_den,
                            });
                        }
                    }
                }
            }
        }
    }
    println!("   pool: {} coordinate tuples", pool.len());

    let mut repaired_wrong = 0usize;
    let mut shipped_wrong = 0usize;
    let mut first_shipped_witness: Option<Coords> = None;
    let mut true_rows = 0usize;
    let mut false_rows = 0usize;

    for &c in &pool {
        let oracle = zero_is_denoted(c);
        if oracle {
            true_rows += 1
        } else {
            false_rows += 1
        }
        if has_additive_identity_repaired(c) != oracle {
            repaired_wrong += 1;
        }
        if has_additive_identity_shipped(c) != oracle {
            shipped_wrong += 1;
            if first_shipped_witness.is_none() {
                first_shipped_witness = Some(c);
            }
        }
    }

    println!("   oracle says zero is a member on {true_rows} rows and not on {false_rows}");
    println!("   the repaired predicate differs from the oracle on {repaired_wrong} rows");
    println!("   the shipped predicate differs from the oracle on {shipped_wrong} rows");

    if true_rows == 0 || false_rows == 0 {
        println!("\n   CONTROL FAILED: the oracle answers one way over the whole pool.");
        std::process::exit(2);
    }
    if shipped_wrong == 0 {
        println!("\n   CONTROL FAILED: the shipped predicate was never reported wrong, so the");
        println!("   pool does not reach a format where the two can differ.");
        std::process::exit(2);
    }
    println!("\n   controls hold: the oracle answers both ways, and the mutant is caught.");

    if let Some(w) = first_shipped_witness {
        println!("   first witness against the shipped predicate: {w:?}");
    }
    if repaired_wrong == 0 {
        println!("\n   REPAIR ONE HOLDS over the swept space.");
    } else {
        findings += 1;
        println!("\n   REPAIR ONE FAILS on {repaired_wrong} rows; it is not the predicate.");
    }

    // ---------------- section 2 ----------------
    println!("\n== section 2: denotation equality over the constant-quantum family ==\n");

    let mut cpool = Vec::new();
    for base in -1i32..=1 {
        for &(min, max) in &[
            (-8i64, 7i64),
            (-7, 8),
            (0, 15),
            (1, 16),
            (-4, 3),
            (0, 0),
            (5, 5),
        ] {
            for phase_den in [1i64, 2] {
                for phase_num in -4i64..=4 {
                    cpool.push(Coords {
                        radix: 2,
                        base,
                        slope: 0,
                        magnitudes: 1,
                        min,
                        max,
                        phase_num,
                        phase_den,
                    });
                }
            }
        }
    }
    println!("   pool: {} constant-family tuples", cpool.len());

    let sets: Vec<BTreeSet<Q>> = cpool.iter().map(|&c| denotation_absolute(c)).collect();

    let mut decided_wrong = 0usize;
    let mut mutant_wrong = 0usize;
    let mut equal_pairs = 0usize;
    let mut unequal_pairs = 0usize;
    let mut first_decided_witness: Option<(Coords, Coords)> = None;

    for i in 0..cpool.len() {
        for j in 0..cpool.len() {
            let oracle = sets[i] == sets[j];
            if oracle {
                equal_pairs += 1
            } else {
                unequal_pairs += 1
            }
            if same_set_constant_family(cpool[i], cpool[j]) != oracle {
                decided_wrong += 1;
                if first_decided_witness.is_none() {
                    first_decided_witness = Some((cpool[i], cpool[j]));
                }
            }
            if same_set_mutant_no_offset(cpool[i], cpool[j]) != oracle {
                mutant_wrong += 1;
            }
        }
    }

    println!("   ordered pairs: {}", cpool.len() * cpool.len());
    println!("   the oracle calls {equal_pairs} equal and {unequal_pairs} unequal");
    println!("   the decision procedure differs from the oracle on {decided_wrong} pairs");
    println!("   the offset-dropping mutant differs on {mutant_wrong} pairs");

    if equal_pairs == 0 || unequal_pairs == 0 {
        println!("\n   CONTROL FAILED: the oracle answers one way.");
        std::process::exit(2);
    }
    if mutant_wrong == 0 {
        println!("\n   CONTROL FAILED: the mutant was never caught, so the pool holds no two");
        println!("   grids at one step and count with different offsets.");
        std::process::exit(2);
    }
    println!("\n   controls hold.");

    if decided_wrong == 0 {
        println!("   REPAIR TWO HOLDS over the swept space: three comparisons decide it.");
    } else {
        findings += 1;
        println!("   REPAIR TWO FAILS on {decided_wrong} pairs.");
        if let Some((a, b)) = first_decided_witness {
            println!("   first witness: {a:?}\n              vs {b:?}");
        }
    }

    // How much the coordinates over-count the sets, which is the size of the gap
    // between the ratified identity and the shipped one.
    let distinct_sets: BTreeSet<Vec<Q>> = sets
        .iter()
        .map(|s| s.iter().copied().collect::<Vec<_>>())
        .collect();
    println!(
        "\n   {} distinct coordinate tuples denote {} distinct sets.",
        cpool.len(),
        distinct_sets.len()
    );
    println!(
        "   So {} tuples are a second name for a set another tuple already names.",
        cpool.len() - distinct_sets.len()
    );

    println!("\n== verdict ==");
    println!("   repairs that failed: {findings}");
    std::process::exit(if findings == 0 { 0 } else { 1 });
}
