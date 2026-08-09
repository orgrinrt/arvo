// Probe B: the overflow side of a chain schedule, and order-dependence.
//
// Hypothesis: adaptation has two sides, rounding (fractional excess) and
// overflow policy (integral excess), and the schedule places both. The overflow
// side alone already makes schedules semantically distinct:
//   - a per-step SATURATING fold is order-dependent,
//   - a per-step WRAPPING fold is order-independent (mod 2^n is a ring
//     homomorphism, so any association or order gives the exact sum mod 2^n),
//   - a wide exact accumulator with a single saturating adapt at the end is
//     order-independent and equals the once-adapted exact answer.
// A float arm shows the same schedule taxonomy covers the float half of the
// family: stepwise-rounded f64 summation is order-dependent for the classic
// reason, which is the thread-count question (I10) in miniature, since a
// parallel reduction is a reorder.
//
// Mutant check: the invariance checker is fed the per-step saturating fold as a
// deliberate mutant of the wide arm; it must report variance, or the harness is
// broken and the probe panics.
//
// Shortcuts (spike): i16 as the boundary format, hardcoded 3-element multiset,
// all 6 permutations enumerated by hand, bare primitives, std println.

const XS: [i16; 3] = [30000, 10000, -25000];
const PERMS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

fn sat_fold(order: &[usize; 3]) -> i16 {
    order.iter().fold(0i16, |acc, &i| acc.saturating_add(XS[i]))
}

fn wrap_fold(order: &[usize; 3]) -> i16 {
    order.iter().fold(0i16, |acc, &i| acc.wrapping_add(XS[i]))
}

fn wide_then_sat(order: &[usize; 3]) -> i16 {
    let exact: i64 = order.iter().map(|&i| XS[i] as i64).sum();
    exact.clamp(i16::MIN as i64, i16::MAX as i64) as i16
}

// returns the set of distinct results across all 6 orders
fn distinct<Fx: Fn(&[usize; 3]) -> i16>(f: Fx) -> Vec<i16> {
    let mut out: Vec<i16> = PERMS.iter().map(|p| f(p)).collect();
    out.sort();
    out.dedup();
    out
}

fn main() {
    let exact: i64 = XS.iter().map(|&x| x as i64).sum();
    let once_adapted = exact.clamp(i16::MIN as i64, i16::MAX as i64) as i16;
    println!("multiset {XS:?}, exact sum {exact}, once-adapted (saturated) exact {once_adapted}");

    let sat = distinct(sat_fold);
    let wrap = distinct(wrap_fold);
    let wide = distinct(wide_then_sat);

    println!("per-step saturating fold, distinct results across 6 orders: {sat:?}");
    println!("per-step wrapping fold,   distinct results across 6 orders: {wrap:?}");
    println!("wide exact + single sat,  distinct results across 6 orders: {wide:?}");

    // claims
    assert!(
        sat.len() > 1,
        "expected order-dependence in saturating fold"
    );
    assert_eq!(wrap.len(), 1, "wrapping fold must be order-independent");
    assert_eq!(
        wrap[0],
        (exact as u64 as u16) as i16,
        "wrapping fold must equal exact sum mod 2^16"
    );
    assert_eq!(wide.len(), 1, "wide arm must be order-independent");
    assert_eq!(
        wide[0], once_adapted,
        "wide arm must equal once-adapted exact"
    );

    // mutant: the invariance checker must be able to fail. Feed it the per-step
    // saturating fold in the wide arm's place; it must see variance.
    let mutant = distinct(sat_fold);
    assert!(
        mutant.len() > 1,
        "HARNESS BROKEN: invariance checker cannot detect a varying arm"
    );

    // float arm: same taxonomy, different granularity function
    let fs = [1e16f64, 1.0, -1e16];
    let mut fresults: Vec<f64> = PERMS
        .iter()
        .map(|p| p.iter().fold(0.0f64, |acc, &i| acc + fs[i]))
        .collect();
    fresults.sort_by(f64::total_cmp);
    fresults.dedup();
    println!("f64 stepwise sum of {fs:?}, distinct results across 6 orders: {fresults:?}");
    assert!(
        fresults.len() > 1,
        "expected order-dependence in stepwise f64 summation"
    );

    println!("OUTCOME: WORKS");
}
