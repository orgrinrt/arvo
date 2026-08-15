// p8: is the replacement I am proposing actually doable?
//
// NOT A BENCHMARK. It times nothing and prices nothing. It answers a
// yes-or-no question about what the compiler does, which is the only kind of
// question a probe of this shape can answer.
//
// p2 establishes that a linear weighting cannot select every Pareto-optimal
// arm, and that a weighted Chebyshev selector with a utopia reference point
// reaches all of them with a closed-form certificate weight. That is only a
// replacement worth offering if it costs nothing at compile time, because
// I15 requires everything to reach one lowered path with no runtime check, and
// 139's p5 established that property for a LINEAR selector and only for that.
//
// So the question here is narrow: does a max-based selector const-evaluate, and
// does the chosen arm survive to a single unconditional call with the other
// arms gone? Both selectors are built in one file so the comparison is against
// the incumbent rather than against nothing.
//
// PREDICTIONS, before compiling:
//   V1 the Chebyshev selector const-evaluates. A max over a small array of
//      integer products is nothing a const evaluator struggles with, and there
//      is no floating point in it because the weights are integers and the
//      reference point is an integer vector.
//   V2 each monomorphisation lowers to one unconditional call to one arm, with
//      the other arms absent from that function body.
//   V3 the two selectors pick DIFFERENT arms for the same weight on the same
//      cost table, on the arm set p2 uses, because the compromise arm is
//      unreachable linearly and reachable under Chebyshev. If they always agree
//      the file proves nothing about the replacement.
//
// CONTROLS:
//   W1 THE CASE THAT MUST FAIL. A runtime-selected version, taking the index as
//      an argument, must keep every arm and a conditional branch in its body.
//      If the scan reports no arms there either, the scan is looking at the
//      wrong thing, which is exactly what happened to 139's first scan.
//   W2 the arms must be distinguishable in the emitted code, so each carries a
//      distinct constant that appears in its body.
//   W3 the const selectors must agree with a runtime recomputation of the same
//      formula, so that a const-eval bug cannot pass silently.

#![allow(dead_code)]

const NARMS: usize = 3;
const NCOORD: usize = 2;

// The arm set from p2's witness: two endpoints and a compromise arm that is
// Pareto-optimal and sits above the line joining them. Costs are integers.
const COSTS: [[i64; NCOORD]; NARMS] = [
    [1, 21], // endpoint A
    [21, 1], // endpoint B
    [13, 13], // compromise C, unreachable by any linear weighting
];

// Utopia point: one below the componentwise minimum, so every deviation is
// strictly positive and the certificate weight is well defined.
const fn utopia() -> [i64; NCOORD] {
    let mut z = [i64::MAX; NCOORD];
    let mut k = 0;
    while k < NCOORD {
        let mut i = 0;
        while i < NARMS {
            if COSTS[i][k] < z[k] {
                z[k] = COSTS[i][k];
            }
            i += 1;
        }
        z[k] -= 1;
        k += 1;
    }
    z
}

const Z: [i64; NCOORD] = utopia();

const fn lin_pick(w: [i64; NCOORD]) -> usize {
    let mut best = 0usize;
    let mut bestv = i64::MAX;
    let mut i = 0;
    while i < NARMS {
        let mut v = 0i64;
        let mut k = 0;
        while k < NCOORD {
            v += w[k] * COSTS[i][k];
            k += 1;
        }
        if v < bestv {
            bestv = v;
            best = i;
        }
        i += 1;
    }
    best
}

// Weighted Chebyshev with a utopia reference: minimise the largest weighted
// deviation. The whole selector is integer max, multiply and compare.
const fn cheb_pick(w: [i64; NCOORD]) -> usize {
    let mut best = 0usize;
    let mut bestv = i64::MAX;
    let mut i = 0;
    while i < NARMS {
        let mut m = i64::MIN;
        let mut k = 0;
        while k < NCOORD {
            let t = w[k] * (COSTS[i][k] - Z[k]);
            if t > m {
                m = t;
            }
            k += 1;
        }
        if m < bestv {
            bestv = m;
            best = i;
        }
        i += 1;
    }
    best
}

trait Selection {
    const PICK: usize;
}

// A linear weighting that leans on the first coordinate.
struct LinearFirst;
impl Selection for LinearFirst {
    const PICK: usize = lin_pick([1, 3]);
}

// A linear weighting that balances. It still cannot reach the compromise arm.
struct LinearBalanced;
impl Selection for LinearBalanced {
    const PICK: usize = lin_pick([1, 1]);
}

// The Chebyshev certificate weight for the compromise arm: w_k proportional to
// 1 / (c_k - z_k), scaled to integers. c - z is [13, 13] here, so the weight is
// uniform, and the arm is optimal at it.
struct ChebCompromise;
impl Selection for ChebCompromise {
    const PICK: usize = cheb_pick([1, 1]);
}

#[inline(never)]
fn arm_a(x: i64) -> i64 {
    x.wrapping_mul(1_000_003).wrapping_add(11)
}

#[inline(never)]
fn arm_b(x: i64) -> i64 {
    x.wrapping_mul(2_000_003).wrapping_add(22)
}

#[inline(never)]
fn arm_c(x: i64) -> i64 {
    x.wrapping_mul(3_000_003).wrapping_add(33)
}

#[inline(always)]
fn dispatch<S: Selection>(x: i64) -> i64 {
    match S::PICK {
        0 => arm_a(x),
        1 => arm_b(x),
        _ => arm_c(x),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn entry_linear_first(x: i64) -> i64 {
    dispatch::<LinearFirst>(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn entry_linear_balanced(x: i64) -> i64 {
    dispatch::<LinearBalanced>(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn entry_cheb_compromise(x: i64) -> i64 {
    dispatch::<ChebCompromise>(x)
}

// W1: the control. The index arrives at runtime, so every arm must survive and
// a conditional branch must remain.
#[unsafe(no_mangle)]
pub extern "C" fn entry_runtime_selected(sel: usize, x: i64) -> i64 {
    match sel {
        0 => arm_a(x),
        1 => arm_b(x),
        _ => arm_c(x),
    }
}

fn rt_lin(w: [i64; NCOORD]) -> usize {
    let mut best = 0usize;
    let mut bestv = i64::MAX;
    for i in 0..NARMS {
        let v: i64 = (0..NCOORD).map(|k| w[k] * COSTS[i][k]).sum();
        if v < bestv {
            bestv = v;
            best = i;
        }
    }
    best
}

fn rt_cheb(w: [i64; NCOORD]) -> usize {
    let mut best = 0usize;
    let mut bestv = i64::MAX;
    for i in 0..NARMS {
        let m = (0..NCOORD).map(|k| w[k] * (COSTS[i][k] - Z[k])).max().unwrap();
        if m < bestv {
            bestv = m;
            best = i;
        }
    }
    best
}

fn main() {
    println!("p8: NOT A BENCHMARK. a compile-time question with a yes or no answer.");
    println!("utopia reference point Z = {Z:?}");
    println!();
    println!("V1 the const selectors evaluated at compile time:");
    println!("   linear   w=(1,3) -> arm {}", LinearFirst::PICK);
    println!("   linear   w=(1,1) -> arm {}", LinearBalanced::PICK);
    println!("   chebyshev w=(1,1) -> arm {}", ChebCompromise::PICK);
    println!("   (the file would not compile if any of these were not const, so");
    println!("    V1 is answered by the build succeeding at all)");
    println!();

    let mut fails = 0;

    // W3: the const answers against a runtime recomputation of the same formula
    let checks = [
        ("linear (1,3)", LinearFirst::PICK, rt_lin([1, 3])),
        ("linear (1,1)", LinearBalanced::PICK, rt_lin([1, 1])),
        ("cheb (1,1)", ChebCompromise::PICK, rt_cheb([1, 1])),
    ];
    for (name, c, r) in checks {
        let ok = c == r;
        println!("W3 {name}: const {c}, runtime {r} -> {}", if ok { "PASS" } else { "FAIL" });
        if !ok {
            fails += 1;
        }
    }

    // V3: the two selectors must disagree somewhere, or there is no replacement
    let mut disagreements = 0;
    let mut lin_reached = [false; NARMS];
    let mut cheb_reached = [false; NARMS];
    for a in 1..=40i64 {
        for b in 1..=40i64 {
            let l = rt_lin([a, b]);
            let c = rt_cheb([a, b]);
            lin_reached[l] = true;
            cheb_reached[c] = true;
            if l != c {
                disagreements += 1;
            }
        }
    }
    println!();
    println!("V3 over 1600 integer weights on the same cost table:");
    println!("   arms a linear selector ever picks:    {lin_reached:?}");
    println!("   arms a chebyshev selector ever picks: {cheb_reached:?}");
    println!("   they disagree at {disagreements} of 1600 weights");
    let v3 = disagreements > 0 && !lin_reached[2] && cheb_reached[2];
    println!("   V3 -> {}", if v3 { "CONFIRMED" } else { "REFUTED" });
    if !v3 {
        fails += 1;
    }
    println!("   so the compromise arm is unreachable linearly and reachable under");
    println!("   chebyshev, on the same arms, which is what the replacement is for.");

    // W2: the arms carry distinguishable constants
    println!();
    println!("W2 the three arms return different values for the same input:");
    let (x, y, z) = (arm_a(7), arm_b(7), arm_c(7));
    println!("   arm_a(7)={x} arm_b(7)={y} arm_c(7)={z}");
    let w2 = x != y && y != z && x != z;
    println!("   W2 -> {}", if w2 { "PASS" } else { "FAIL" });
    if !w2 {
        fails += 1;
    }

    println!();
    println!("entry points return: {} {} {}",
             entry_linear_first(7), entry_linear_balanced(7), entry_cheb_compromise(7));
    println!("runtime-selected control returns: {} {} {}",
             entry_runtime_selected(0, 7), entry_runtime_selected(1, 7),
             entry_runtime_selected(2, 7));

    println!();
    println!("control failures: {fails}");
    if fails > 0 {
        std::process::exit(1);
    }
}
