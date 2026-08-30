//! Are the nine views monoid homomorphisms out of the grade, and is the
//! nine-point domain closed under join with the join equal to the kernel
//! intersection?
//!
//! MODEL. The grade is the free commutative monoid over the five IEEE clause-7
//! exception generators, split into the two classes the design names at
//! `110:2106-2107`: causes are {invalid, divideByZero}; events are {inexact,
//! underflow, overflow}. A grade is a multiplicity vector in N^5 truncated at
//! MAXMUL. The operation is componentwise addition; the identity is the zero
//! vector.
//!
//! A view is a pair of detail levels, one per class, from {Ignore, Presence,
//! Exact} (`37:130`, `37_probes/probe_1:105-107`). Nine views.
//!
//! Presence has two readings the design does not distinguish, so both are
//! checked: PER_CLASS collapses the whole class to one bit ("did any occur"),
//! which is what probe 1 models with its single `c` and `e` counters;
//! PER_GENERATOR keeps the support set, which is what the IEEE sticky-flag
//! convergence at `110:2103-2105` needs (a five-bit word joined by bitwise or).
//!
//! CHECK H1. v(g1 + g2) = v(g1) + v(g2), over every pair whose componentwise
//! sum stays inside the model.
//! CHECK H2. v(0) = 0.
//! CHECK J1. For every pair of views, the componentwise max of detail levels is
//! in the domain (true by construction, so what is actually checked is J2).
//! CHECK J2. ker(join(v1, v2)) = ker(v1) INTERSECT ker(v2), over every pair of
//! grades and every pair of views. This is the pullback step file 37's
//! uniqueness argument runs and never asserted.

const NGEN: usize = 5;
/// Index 0..2 are causes, 2..5 are events.
const CAUSE_END: usize = 2;
const MAXMUL: u8 = 3;

const IGNORE: u8 = 0;
const PRESENCE: u8 = 1;
const EXACT: u8 = 2;

type Grade = [u8; NGEN];

/// The image of one class under one detail level, encoded so that equality of
/// encodings is equality of images, and `join_img` is the target monoid's op.
fn img(g: &Grade, lo: usize, hi: usize, detail: u8, per_generator: bool) -> u64 {
    match detail {
        IGNORE => 0,
        PRESENCE => {
            if per_generator {
                let mut m = 0u64;
                for i in lo..hi {
                    if g[i] > 0 {
                        m |= 1 << (i as u64);
                    }
                }
                m
            } else {
                let mut any = 0u64;
                for i in lo..hi {
                    if g[i] > 0 {
                        any = 1;
                    }
                }
                any
            }
        }
        _ => {
            let mut m = 0u64;
            for i in lo..hi {
                m = m * 16 + g[i] as u64;
            }
            m
        }
    }
}

/// The target monoid's operation on encoded images.
fn join_img(a: u64, b: u64, detail: u8, per_generator: bool) -> u64 {
    match detail {
        IGNORE => 0,
        PRESENCE => {
            if per_generator {
                a | b
            } else {
                if a | b != 0 {
                    1
                } else {
                    0
                }
            }
        }
        _ => {
            // componentwise add in base 16
            let mut out = 0u64;
            let mut scale = 1u64;
            let (mut x, mut y) = (a, b);
            for _ in 0..NGEN {
                out += ((x % 16) + (y % 16)) * scale;
                x /= 16;
                y /= 16;
                scale *= 16;
            }
            out
        }
    }
}

fn view(g: &Grade, dc: u8, de: u8, pg: bool) -> (u64, u64) {
    (
        img(g, 0, CAUSE_END, dc, pg),
        img(g, CAUSE_END, NGEN, de, pg),
    )
}

fn all_grades() -> Vec<Grade> {
    let mut out = Vec::new();
    let mut g: Grade = [0; NGEN];
    loop {
        out.push(g);
        let mut i = 0;
        loop {
            if i == NGEN {
                return out;
            }
            if g[i] < MAXMUL {
                g[i] += 1;
                break;
            }
            g[i] = 0;
            i += 1;
        }
    }
}

fn main() {
    let grades = all_grades();
    println!("grades in model: {}", grades.len());

    for &pg in &[false, true] {
        let name = if pg { "PER_GENERATOR" } else { "PER_CLASS" };
        let mut h1_checked: u64 = 0;
        let mut h1_failed: u64 = 0;
        let mut h2_failed: u64 = 0;

        for dc in 0..3u8 {
            for de in 0..3u8 {
                // H2
                let zero: Grade = [0; NGEN];
                let (zc, ze) = view(&zero, dc, de, pg);
                let idc = img(&zero, 0, CAUSE_END, dc, pg);
                let ide = img(&zero, CAUSE_END, NGEN, de, pg);
                // the target monoid's identity is the image of the zero grade;
                // check it is a left and right unit for join_img
                for g in &grades {
                    let (a, b) = view(g, dc, de, pg);
                    if join_img(idc, a, dc, pg) != a || join_img(a, idc, dc, pg) != a {
                        h2_failed += 1;
                    }
                    if join_img(ide, b, de, pg) != b || join_img(b, ide, de, pg) != b {
                        h2_failed += 1;
                    }
                }
                let _ = (zc, ze);

                // H1
                for g1 in &grades {
                    for g2 in &grades {
                        let mut sum: Grade = [0; NGEN];
                        let mut in_range = true;
                        for i in 0..NGEN {
                            let s = g1[i] + g2[i];
                            if s > MAXMUL {
                                in_range = false;
                                break;
                            }
                            sum[i] = s;
                        }
                        if !in_range {
                            continue;
                        }
                        let (sc, se) = view(&sum, dc, de, pg);
                        let (ac, ae) = view(g1, dc, de, pg);
                        let (bc, be) = view(g2, dc, de, pg);
                        h1_checked += 1;
                        if join_img(ac, bc, dc, pg) != sc || join_img(ae, be, de, pg) != se {
                            h1_failed += 1;
                        }
                    }
                }
            }
        }
        println!(
            "{}: H1 pairs checked {} failures {} | H2 unit failures {}",
            name, h1_checked, h1_failed, h2_failed
        );
    }

    // J2: the join of two views is the kernel intersection.
    let pg = false;
    let mut j2_checked: u64 = 0;
    let mut j2_failed: u64 = 0;
    for dc1 in 0..3u8 {
        for de1 in 0..3u8 {
            for dc2 in 0..3u8 {
                for de2 in 0..3u8 {
                    let jc = dc1.max(dc2);
                    let je = de1.max(de2);
                    for g1 in &grades {
                        for g2 in &grades {
                            let a = view(g1, dc1, de1, pg) == view(g2, dc1, de1, pg);
                            let b = view(g1, dc2, de2, pg) == view(g2, dc2, de2, pg);
                            let j = view(g1, jc, je, pg) == view(g2, jc, je, pg);
                            j2_checked += 1;
                            if j != (a && b) {
                                j2_failed += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("J2: comparisons {} failures {}", j2_checked, j2_failed);

    // The definedness witness. If a cause can be recorded on a DEFINED term,
    // (Presence, Ignore) is not the Kleene equation.
    // t1: defined, value INF, causes {divideByZero}
    // t2: undefined,          causes {invalid}
    let t1: Grade = [0, 1, 0, 0, 0];
    let t2: Grade = [1, 0, 0, 0, 0];
    let same_at_kleene_point =
        view(&t1, PRESENCE, IGNORE, false) == view(&t2, PRESENCE, IGNORE, false);
    println!(
        "definedness witness: (Presence, Ignore) identifies a defined divideByZero term \
with an undefined invalid term: {}",
        same_at_kleene_point
    );
}
