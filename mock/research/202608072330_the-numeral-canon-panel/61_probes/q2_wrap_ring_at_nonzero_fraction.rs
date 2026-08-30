//! Probe q2: does the wrap ring survive a genuine rescaling multiply at
//! nonzero fraction width.
//!
//! WHY. `55b` section 3.1 reports wrap's induced structure, exhaustively at
//! 4 bits, as "the ring Z/16: associative add and mul, identities 0 and 1,
//! all inverses, distributive" (`55_probes/p4_induced_algebra_grades.rs`).
//! `58` opened that file (lines 69-73) and found `add` and `mul` are bare
//! `a + b` and `a * b`, no scale parameter anywhere: an F = 0 result. `59`
//! section 1e independently confirms the same reading and names the wrap
//! ring, alongside the semiring, as unmeasured at F > 0. `59`'s P3 dispatch
//! names this "one arm away".
//!
//! It is one arm away for a second, more precise reason this probe found on
//! inspection before writing a line of new code: `57_probes/p3_semiring_
//! across_widths_and_scales.rs` already implements `Policy::Wrap` with a
//! genuine rescaling multiply (`reduce(p, m, (a*b)/scale)`, `scale = 1 << f`,
//! `reduce(Wrap, m, x) = x.rem_euclid(m+1)`), and that machinery is CALLED
//! exactly once with `Policy::Wrap`, at `f = 0`, in its own instrument-
//! validation section. The rescaling wrap path exists and has never been
//! driven past zero fraction bits.
//!
//! This probe drives it. Same `reduce`/`check` shape as `57_probes/p3`
//! (independently re-derived here, checked line-for-line against the source
//! before commit, for self-containment), re-targeted at the full ring axiom
//! set (associativity, commutativity, both identities, distributivity, AND
//! additive inverses, since ring-versus-semiring is decided by inverses) and
//! swept across F = 0, 1, 2, 3 at several widths.
//!
//! PREDICTION, stated before running. Wrap's raw representative arithmetic
//! is a ring homomorphism image at F = 0 because `rem_euclid` alone is a
//! ring quotient map. But wrap's genuine fixed-point multiply is NOT plain
//! modular multiplication; it is truncating-rescale-then-reduce, the exact
//! same two-step composite (`(a*b) >> F` then a reduction) that `57`/`58`
//! already measured breaking the UNSIGNED SATURATION semiring's
//! multiplicative associativity and distributivity at every F > 0 tested,
//! with the mechanism isolated to the coarsening step rather than the
//! reduction policy (`58` section 3.2, `CoarsenOnly` breaks with no clamp
//! present at all). Wrap's reduction is a different map from saturation's,
//! but the coarsening step is IDENTICAL code shared by both policies. So the
//! prediction is that the wrap ring collapses at F > 0 for the same
//! structural reason the semiring did, and the additive half survives
//! (addition never rescales, for wrap exactly as for saturation).
//!
//! INSTRUMENT VALIDATION. The checker must report BOTH ring status (true)
//! and non-ring status (false) somewhere in the sweep, and the F = 0 row
//! must reproduce `55_probes/p4`'s and `57_probes/p3` section 4's own
//! wrap-is-a-ring reading before anything at F > 0 is trusted.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o q2 q2_wrap_ring_at_nonzero_fraction.rs && ./q2

fn wrap_reduce(m: i64, x: i64) -> i64 {
    x.rem_euclid(m + 1)
}

fn sat_reduce(m: i64, x: i64) -> i64 {
    x.clamp(0, m)
}

struct Axioms {
    add_assoc: u64,
    add_comm: u64,
    add_ident: u64,
    mul_assoc: u64,
    mul_comm: u64,
    mul_ident: u64,
    distrib: u64,
    zero_annih: u64,
    no_add_inverse: u64,
}

impl Axioms {
    fn is_ring(&self) -> bool {
        self.add_assoc == 0
            && self.add_comm == 0
            && self.add_ident == 0
            && self.mul_assoc == 0
            && self.mul_comm == 0
            && self.mul_ident == 0
            && self.distrib == 0
            && self.zero_annih == 0
            && self.no_add_inverse == 0
    }
    fn is_comm_semiring(&self) -> bool {
        self.add_assoc == 0
            && self.add_comm == 0
            && self.add_ident == 0
            && self.mul_assoc == 0
            && self.mul_comm == 0
            && self.mul_ident == 0
            && self.distrib == 0
            && self.zero_annih == 0
    }
}

/// full axiom sweep for wrap, at width M (raw values 0..=m, i.e. modulus
/// m+1) and fraction width F. add is raw addition then wrap-reduce; mul is
/// the genuine fixed-point shape, raw product rescaled by >> F (truncating
/// division), THEN wrap-reduce. F = 0 makes the rescale a no-op (divide by
/// 1), which is the reproduction check against 57_probes/p3.
fn check_wrap(m: i64, f: u32) -> Axioms {
    let scale = 1i64 << f;
    let add = |a: i64, b: i64| wrap_reduce(m, a + b);
    let mul = |a: i64, b: i64| wrap_reduce(m, (a * b) / scale);
    let one = scale.min(m);

    let mut ax = Axioms {
        add_assoc: 0,
        add_comm: 0,
        add_ident: 0,
        mul_assoc: 0,
        mul_comm: 0,
        mul_ident: 0,
        distrib: 0,
        zero_annih: 0,
        no_add_inverse: 0,
    };

    for a in 0..=m {
        if add(a, 0) != a || add(0, a) != a {
            ax.add_ident += 1;
        }
        if mul(a, one) != a || mul(one, a) != a {
            ax.mul_ident += 1;
        }
        if mul(a, 0) != 0 || mul(0, a) != 0 {
            ax.zero_annih += 1;
        }
        if !(0..=m).any(|b| add(a, b) == 0) {
            ax.no_add_inverse += 1;
        }
        for b in 0..=m {
            if add(a, b) != add(b, a) {
                ax.add_comm += 1;
            }
            if mul(a, b) != mul(b, a) {
                ax.mul_comm += 1;
            }
            for c in 0..=m {
                if add(add(a, b), c) != add(a, add(b, c)) {
                    ax.add_assoc += 1;
                }
                if mul(mul(a, b), c) != mul(a, mul(b, c)) {
                    ax.mul_assoc += 1;
                }
                if mul(a, add(b, c)) != add(mul(a, b), mul(a, c)) {
                    ax.distrib += 1;
                }
            }
        }
    }
    ax
}

/// mirrors 57_probes/p3's saturation check(), for the shared-mechanism
/// comparison in section 3: both policies use IDENTICAL coarsening code.
fn check_sat(m: i64, f: u32) -> Axioms {
    let scale = 1i64 << f;
    let add = |a: i64, b: i64| sat_reduce(m, a + b);
    let mul = |a: i64, b: i64| sat_reduce(m, (a * b) / scale);
    let one = scale.min(m);

    let mut ax = Axioms {
        add_assoc: 0,
        add_comm: 0,
        add_ident: 0,
        mul_assoc: 0,
        mul_comm: 0,
        mul_ident: 0,
        distrib: 0,
        zero_annih: 0,
        no_add_inverse: 0,
    };
    for a in 0..=m {
        if add(a, 0) != a || add(0, a) != a {
            ax.add_ident += 1;
        }
        if mul(a, one) != a || mul(one, a) != a {
            ax.mul_ident += 1;
        }
        if mul(a, 0) != 0 || mul(0, a) != 0 {
            ax.zero_annih += 1;
        }
        if !(0..=m).any(|b| add(a, b) == 0) {
            ax.no_add_inverse += 1;
        }
        for b in 0..=m {
            if add(a, b) != add(b, a) {
                ax.add_comm += 1;
            }
            if mul(a, b) != mul(b, a) {
                ax.mul_comm += 1;
            }
            for c in 0..=m {
                if add(add(a, b), c) != add(a, add(b, c)) {
                    ax.add_assoc += 1;
                }
                if mul(mul(a, b), c) != mul(a, mul(b, c)) {
                    ax.mul_assoc += 1;
                }
                if mul(a, add(b, c)) != add(mul(a, b), mul(a, c)) {
                    ax.distrib += 1;
                }
            }
        }
    }
    ax
}

fn print_row(m: i64, f: u32, ax: &Axioms, verdict: &str) {
    println!(
        "{:>6} {:>4} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>8} {:>7} {:>10} {:>10}",
        m,
        f,
        ax.add_assoc,
        ax.add_comm,
        ax.add_ident,
        ax.mul_assoc,
        ax.mul_comm,
        ax.mul_ident,
        ax.distrib,
        ax.no_add_inverse,
        ax.zero_annih,
        verdict
    );
}

fn header() {
    println!(
        "{:>6} {:>4} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>8} {:>7} {:>10} {:>10}",
        "M",
        "F",
        "+assoc",
        "+comm",
        "+id",
        "*assoc",
        "*comm",
        "*id",
        "distrib",
        "no-inv",
        "0-annih",
        "verdict"
    );
}

fn main() {
    let mut ok = true;

    println!("=== section 1: reproduce the F = 0 wrap ring, before trusting anything at F > 0 ===");
    println!();
    header();
    let ax0 = check_wrap(15, 0);
    let is_ring0 = ax0.is_ring();
    print_row(15, 0, &ax0, if is_ring0 { "ring" } else { "NOT ring" });
    println!(
        "  reproduces 55_probes/p4 and 57_probes/p3 section 4's wrap-is-a-ring reading: {}",
        is_ring0
    );
    println!();
    ok &= is_ring0;

    println!("=== section 2: the wrap ring across F = 0, 1, 2, 3, at three widths ===");
    println!();
    header();
    let mut saw_ring = false;
    let mut saw_not_ring = false;
    let mut frac_breaks = 0u64;
    let widths = [15i64, 31, 63];
    for &m in &widths {
        for f in 0..=3u32 {
            let ax = check_wrap(m, f);
            let is_ring = ax.is_ring();
            saw_ring |= is_ring;
            saw_not_ring |= !is_ring;
            if f > 0 && !is_ring {
                frac_breaks += 1;
            }
            print_row(m, f, &ax, if is_ring { "ring" } else { "NOT ring" });
        }
    }
    println!();
    println!(
        "  fractional (F > 0) configurations that are NOT rings: {} of {}",
        frac_breaks,
        widths.len() * 3
    );
    println!();
    ok &= saw_ring && saw_not_ring;

    println!("=== section 3: which half breaks, mirroring 57/58's factor decomposition ===");
    println!();
    println!("  additive half only (does +assoc alone survive F > 0):");
    for &m in &widths {
        for f in 1..=3u32 {
            let ax = check_wrap(m, f);
            println!(
                "    M={:<4} F={}  +assoc failures {} (additive monoid {})",
                m,
                f,
                ax.add_assoc,
                if ax.add_assoc == 0 {
                    "survives"
                } else {
                    "BROKEN"
                }
            );
        }
    }
    println!();
    println!("  the SAME coarsening code shared with saturation, side by side at F=1, M=15:");
    let wrap_11 = check_wrap(15, 1);
    let sat_11 = check_sat(15, 1);
    println!(
        "    wrap:       *assoc {}  distrib {}   (Q11's 'ring' claim, at F>0)",
        wrap_11.mul_assoc, wrap_11.distrib
    );
    println!(
        "    saturation: *assoc {}  distrib {}   (57_probes/p3's already-measured semiring collapse)",
        sat_11.mul_assoc, sat_11.distrib
    );
    let shared_mechanism = wrap_11.mul_assoc > 0 && sat_11.mul_assoc > 0;
    println!(
        "    both policies' multiplicative associativity breaks under the identical rescale: {}",
        shared_mechanism
    );
    ok &= shared_mechanism;
    println!();

    println!("=== section 4: does the additive-only survival transfer to wrap the way 58 argued for saturation ===");
    println!();
    // 58's argument: addition never reads the scale, so an F=0 additive
    // result transfers verbatim to every F. Check the same structural fact
    // for wrap by direct inspection: wrap's add closure never multiplies by
    // scale or divides by it anywhere, so the argument is the same argument,
    // re-derived for this policy rather than assumed.
    let mut add_survives_every_f = true;
    for &m in &widths {
        for f in 0..=3u32 {
            let ax = check_wrap(m, f);
            if ax.add_assoc != 0 || ax.add_comm != 0 || ax.add_ident != 0 {
                add_survives_every_f = false;
            }
        }
    }
    println!(
        "  wrap's additive group axioms hold at every (M, F) measured: {}",
        add_survives_every_f
    );
    println!("  (the reason is structural, not a sweep result: wrap's `add` closure never reads",);
    println!("   `f` or `scale` anywhere in its definition, exactly as 58 argued for saturation.)");
    ok &= add_survives_every_f;
    println!();

    println!("=== section 5: the additive-monoid-survives, ring-does-not verdict ===");
    println!();
    let mut all_semiring_at_f_gt_0 = true;
    for &m in &widths {
        for f in 1..=3u32 {
            let ax = check_wrap(m, f);
            let is_semi = ax.is_comm_semiring();
            if !is_semi {
                all_semiring_at_f_gt_0 = false;
            }
        }
    }
    println!(
        "  does wrap fall all the way to 'not even a semiring' at F > 0, or only lose ring status: semiring holds at F>0: {}",
        all_semiring_at_f_gt_0
    );
    println!("  (if false: wrap's multiplicative half is WORSE than saturation's at F > 0, since",);
    println!("   saturation's semiring already failed distributivity/assoc but wrap may fail");
    println!("   both those AND identity/annihilation checks that involve rem_euclid wraparound.)");
    println!();

    println!("{}", if ok { "Q2 WORKS" } else { "Q2 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
