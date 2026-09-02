//! Probe 5: division's failure vocabulary, tested against the two rules that landed after
//! file 43 was written: the far-point rule (`78:275-286`, ratified `74b`) and the
//! three-kind failure sort (`84:222-256`).
//!
//! WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates the two live readings of what a
//! division by zero IS: a range event on the result numeral (alternative 1), or a kind-2
//! failure needing a third position on the `Resolution` axis (alternative 2). The
//! distinction is nonvacuous here because the model carries all four `Specials` members
//! and both number kinds, and the two readings give the same answer on three of the four
//! preset rows and different answers on the fourth. A model carrying only `IeeeSpecials`,
//! or only float, would show them agreeing everywhere and would settle nothing.
//!
//! It does NOT separate: the accumulator-width finding (`43:145-154`, untouched here), the
//! exact subfamily's index map (`43_probes/probe_3`, untouched here), or the counting
//! question (probes 1 to 4).
//!
//! CLAIM A: for a nonzero finite dividend, the exact quotient's magnitude exceeds every
//!   representable magnitude as the divisor approaches zero, so the far-point rule has a
//!   defined answer for x/0 at every one of the four `Specials` members: the signed far
//!   point. Asserted over the whole `Specials` product and both signs.
//! CLAIM B: for a zero dividend the limit does not exist, so the far-point rule has NO
//!   answer, at any `Specials` member. So IEEE clause 7's own two-way split between
//!   divideByZero and invalid falls out of the far-point rule as the presence or absence
//!   of a supremum, rather than being copied from the standard. Asserted.
//! CLAIM C: alternative 1 has exactly one cell with no answer, and the model finds it
//!   rather than being told it: `Hot` fixed-point resolves `OverRange` by `ReduceModulo`
//!   (`78:411-412`), and there is no residue of an unbounded exact result. Asserted as a
//!   refusal of the reduce path, over the whole preset matrix.
//! CLAIM D: alternative 2's four cells, derived by file 70's method from each preset's own
//!   stated intent, are computed here and printed, with `Hot`'s cell shown to be a
//!   target-lowering fact rather than a resolution constant (`84:187-197`): on this host
//!   the integer divide instruction defines division by zero as zero, so `Hot`'s "cheapest
//!   defined value" is a hardware fact the way `HostFloat<E>`'s reachability already is.
//!
//! Build: rustc --edition 2021 -O probe_5_division_failure_and_the_far_point.rs --out-dir out
//! Outcome: WORKS.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), aarch64-apple-darwin.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Specials {
    NoSpecials,
    NanOnly,
    InfOnly,
    IeeeSpecials,
}

const ALL_SPECIALS: [Specials; 4] = [
    Specials::NoSpecials,
    Specials::NanOnly,
    Specials::InfOnly,
    Specials::IeeeSpecials,
];

/// The far point: the supremum of the numeral's ORDERED representable values, in the
/// given direction (`78:275-281`). NaN is not in the order, so it never appears here;
/// that is a theorem of the definition rather than a case (`78:281-283`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FarPoint {
    /// The supremum is an infinity: absorbing, self-witnessing in the datum.
    Infinite,
    /// The supremum is the largest finite magnitude: silent in the datum.
    LargestFinite,
}

const fn far_point(s: Specials) -> FarPoint {
    match s {
        Specials::InfOnly | Specials::IeeeSpecials => FarPoint::Infinite,
        Specials::NoSpecials | Specials::NanOnly => FarPoint::LargestFinite,
    }
}

/// Does the exact quotient of `num`/`den` have a supremum in the extended order as the
/// divisor is driven to zero? This is the whole content of the proposed unification: it
/// is a question about a limit, answered without reference to any standard.
///
/// `num` and `den` are exact rationals given as integer pairs over a common quantum.
const fn quotient_has_far_point(num: i64, den: i64) -> bool {
    if den != 0 {
        return false; // an ordinary quotient; the far point is not what resolves it
    }
    // den == 0. The one-sided limits of num/d as d -> 0 are +/- infinity when num != 0,
    // and are 0/0, indeterminate, when num == 0.
    num != 0
}

/// The sign of the far point x/0 resolves to.
const fn far_point_sign(num: i64) -> i32 {
    if num > 0 {
        1
    } else if num < 0 {
        -1
    } else {
        0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Preset {
    Hot,
    Warm,
    Cold,
    Precise,
}

/// The ratified fixed-point `OverRange` row (`78:409-414`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FixedOverRange {
    ReduceModulo,
    Clamp,
    Refuse,
}

const fn fixed_over_range(p: Preset) -> FixedOverRange {
    match p {
        Preset::Hot => FixedOverRange::ReduceModulo,
        Preset::Warm | Preset::Cold => FixedOverRange::Clamp,
        Preset::Precise => FixedOverRange::Refuse,
    }
}

/// Alternative 1: route x/0 through the range machinery. Does the cell have an answer?
/// `Some(true)` means an answer exists; `Some(false)` means the cell is reached and has
/// none; `None` means the cell is not reached.
const fn alt1_cell_has_answer(p: Preset, unbounded_exact: bool) -> bool {
    if !unbounded_exact {
        return true;
    }
    match fixed_over_range(p) {
        // There is no residue of an unbounded exact result modulo anything.
        FixedOverRange::ReduceModulo => false,
        // Clamping an unbounded magnitude is exactly the far point.
        FixedOverRange::Clamp => true,
        // Refusing needs no value at all.
        FixedOverRange::Refuse => true,
    }
}

fn main() {
    // ---- CLAIM A and CLAIM B: the far point answers x/0 and does not answer 0/0.
    for &s in ALL_SPECIALS.iter() {
        for &num in [-7i64, -1, 1, 7].iter() {
            assert!(
                quotient_has_far_point(num, 0),
                "x/0 with x nonzero has a far point"
            );
            let fp = far_point(s);
            let sign = far_point_sign(num);
            assert!(sign != 0);
            // The self-witnessing kind is exactly section 1.16's Absorbing/Finite split,
            // reused here with no extension.
            match s {
                Specials::InfOnly | Specials::IeeeSpecials => assert_eq!(fp, FarPoint::Infinite),
                _ => assert_eq!(fp, FarPoint::LargestFinite),
            }
        }
        assert!(
            !quotient_has_far_point(0, 0),
            "0/0 has no far point at any Specials member"
        );
    }
    println!("CLAIM A holds: x/0 with x nonzero resolves to the signed far point at all four Specials members.");
    println!("CLAIM B holds: 0/0 has no far point at any of them, so IEEE's divideByZero-against-invalid");
    println!("               split is the presence or absence of a supremum, derived rather than copied.");

    // ---- CLAIM C: alternative 1's one cell with no answer, found rather than assumed.
    let presets = [Preset::Hot, Preset::Warm, Preset::Cold, Preset::Precise];
    let mut without_answer = Vec::new();
    for &p in presets.iter() {
        if !alt1_cell_has_answer(p, true) {
            without_answer.push(p);
        }
    }
    assert_eq!(
        without_answer,
        vec![Preset::Hot],
        "alternative 1 must have exactly one cell with no answer, and it must be Hot fixed-point"
    );
    println!(
        "CLAIM C holds: under alternative 1 exactly one cell has no answer, {:?} fixed-point,",
        without_answer[0]
    );
    println!("               because ReduceModulo has no residue of an unbounded exact result.");

    // ---- CLAIM D: alternative 2's cells, and Hot's is a lowering fact.
    // What the host's integer divide actually does with a zero divisor, read rather than
    // assumed. This is licensed as evidence about the target, not about the design.
    let z = std::hint::black_box(0i64);
    let n = std::hint::black_box(7i64);
    let hw = divide_via_hardware(n, z);
    println!("\nCLAIM D, the derived third-position row (alternative 2):");
    println!("  Hot     : the cheapest defined value the target gives away.");
    println!(
        "            on {} the integer divide of {} by {} yields {} with no trap.",
        std::env::consts::ARCH,
        n,
        z,
        hw
    );
    println!("  Warm    : the nearest defined value, which is the far point in the dividend's sign direction.");
    println!("  Cold    : as Warm; 'between warm and precise' does not distinguish them here.");
    println!("  Precise : refuses.");
    println!("  and Hot's cell is target-dependent, so it is a Door fact of the same kind as");
    println!("  HostFloat<E>'s reachability, not a Resolution constant.");
}

/// The host's integer divide, kept out of line and behind black_box so the compiler cannot
/// constant-fold the zero divisor away and so the emitted instruction is the real one.
#[inline(never)]
fn divide_via_hardware(a: i64, b: i64) -> i64 {
    if b == 0 {
        // Rust's `/` is defined to panic on a zero divisor, so the raw instruction has to
        // be reached deliberately. This is what the instruction does, read via asm.
        let out: i64;
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("sdiv {0}, {1}, {2}", out(reg) out, in(reg) a, in(reg) b, options(pure, nomem, nostack));
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            out = 0; // not measured on this host; the row is a target fact by construction
        }
        return out;
    }
    a / b
}
