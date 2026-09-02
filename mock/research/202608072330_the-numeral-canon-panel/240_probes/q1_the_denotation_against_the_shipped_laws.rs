//! q1. The denotation of a shipped `Format`, enumerated, against the laws the
//! crate states about it.
//!
//! Two questions, both about `arvo-format` as it stands rather than about a model
//! of it. The probe links the real crate and reads its real associated items.
//!
//! **Question one: does `has_additive_identity` decide what it says it decides?**
//! Its doc says a zero phase puts zero on the grid and a nonzero phase takes it
//! off. The body is `PHASE_NUM == 0 && slot_in_range(0)`. The denotation says zero
//! is on the grid exactly when some admitted slot `s` and magnitude `m` satisfy
//! `PHASE_NUM/PHASE_DEN + s * radix^(SLOPE*m) == 0`, so the question is whether
//! the shipped predicate agrees with that on every format reachable here.
//!
//! **Question two: do two formats with different coordinates denote one set?**
//! `ruling::the_format_spine_is_canon` says a format is identified by its ambient
//! domain and its representable set. If two distinct `Format` types over one
//! ambient domain enumerate the same set, then type identity is a finer relation
//! than the ratified identity, and the agreement between the two is a theorem
//! nothing in the crate states or checks.
//!
//! **The case that must fail, stated before the run.** The instrument computes a
//! denotation independently of the crate. If that computation is wrong, the four
//! shipped points, whose phases are `0/1`, `0/1`, `n/2` and `0/1`, will disagree
//! with `has_additive_identity` too. Section 1's control requires every format
//! with `PHASE_NUM = 0` and every format with an odd `PHASE_NUM` over
//! `PHASE_DEN = 2` to AGREE. A run in which the control disagrees establishes
//! nothing about section 2 and says so.
//!
//! Build (from the mock workspace root, after `cargo build -p arvo-format`):
//!
//! ```text
//! rustc --edition 2024 -O \
//!   --extern arvo_format=target/debug/deps/libarvo_format-<hash>.rlib \
//!   -L target/debug/deps \
//!   240_probes/q1_the_denotation_against_the_shipped_laws.rs -o /tmp/q1
//! ```
//!
//! `RUN.md` beside this file carries the exact line and the exit code.

use arvo_format::ambient::{Ambient, BinaryRationals};
use arvo_format::format::{has_additive_identity, Format};
use arvo_format::points::{Biased, Floating, Integer, UFixed};
use arvo_format::quantum::{Constant, Quantum};
use arvo_format::slots::{Signed, Slots};
use arvo_format::width::Width;

/// A value of the format, as an exact rational in units of the quantum at
/// magnitude zero: `num / den`.
///
/// Units of `q(0)` rather than absolute, because the phase is declared in those
/// units and because a common scale is all the two questions need. Reduced, so
/// equality of the pair is equality of the value.
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
    fn is_zero(self) -> bool {
        self.num == 0
    }
}

/// The value set of a format, in units of `q(0)`, enumerated over every admitted
/// slot and magnitude.
///
/// `value / q(0) = PHASE_NUM/PHASE_DEN + slot * radix^(SLOPE * magnitude)`.
///
/// Derived from the trait's own words rather than from any function in the crate,
/// so the two can disagree. `Format::PHASE_NUM` is documented as "the numerator of
/// the phase, in units of the quantum at magnitude zero", and `Quantum` as "the
/// quantum at magnitude `m` is `radix^(BASE + SLOPE * m)`", so the ratio to `q(0)`
/// is `radix^(SLOPE * m)`.
fn denotation<F: Format>() -> Vec<Q> {
    let radix = <F::Ambient as Ambient>::RADIX as i128;
    let slope = <F::Quantum as Quantum>::SLOPE as i128;
    let mags = <F::Quantum as Quantum>::MAGNITUDES as i128;
    let min = <F::Slots as Slots>::MIN as i128;
    let max = <F::Slots as Slots>::MAX as i128;
    let pn = F::PHASE_NUM as i128;
    let pd = F::PHASE_DEN as i128;

    let mut out = Vec::new();
    for m in 0..mags {
        let e = slope * m;
        // Ratio of q(m) to q(0), as an exact rational.
        let (rn, rd) = if e >= 0 {
            (radix.pow(e as u32), 1i128)
        } else {
            (1i128, radix.pow((-e) as u32))
        };
        for s in min..=max {
            // pn/pd + s * rn/rd
            // pn/pd + s * (rn/rd) = (pn*rd + s*rn*pd) / (pd*rd)
            out.push(Q::new(pn * rd + s * rn * pd, pd * rd));
        }
    }
    out.sort();
    out.dedup();
    out
}

fn zero_is_denoted<F: Format>() -> bool {
    denotation::<F>().iter().any(|v| v.is_zero())
}

/// One row of the comparison.
fn row<F: Format>(name: &str, agree: &mut usize, disagree: &mut usize) -> bool {
    let stated = has_additive_identity::<F>();
    let denoted = zero_is_denoted::<F>();
    let ok = stated == denoted;
    if ok {
        *agree += 1;
    } else {
        *disagree += 1;
    }
    println!(
        "  {:<34} phase = {:>3}/{:<2}  has_additive_identity = {:<5}  0 in the set = {:<5}  {}",
        name,
        F::PHASE_NUM,
        F::PHASE_DEN,
        stated,
        denoted,
        if ok { "agree" } else { "DISAGREE" }
    );
    ok
}

// --- section 2's constructions ------------------------------------------------

/// A slot range from outside the crate, at the same width as `Signed<4>` and
/// shifted one step up.
///
/// The `Slots` trait is open and documents what an implementor owes; this meets
/// all of it. Span 15, width 4, not inverted, countable.
struct ShiftedFour;

impl Slots for ShiftedFour {
    const MIN: i64 = -7;
    const MAX: i64 = 8;
    const WIDTH: Width = Width::bits(4);
}

/// A format at phase zero over the shifted range.
struct ShiftedGrid;

impl Format for ShiftedGrid {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = ShiftedFour;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}

/// A second format, over the crate's own `Signed<4>`, whose phase is one whole
/// quantum. `Biased` fixes `PHASE_DEN` at two, so `PHASE = 2` is a phase of one.
type ShiftedByPhase = Biased<4, 0, 2>;

fn main() {
    let mut bad = 0usize;

    println!("== section 1: the control. Formats whose phase is zero, or odd over two. ==");
    println!("   These must AGREE. If any disagrees, the instrument's denotation is wrong");
    println!("   and nothing in section 2 or 3 counts.\n");

    let mut agree = 0usize;
    let mut disagree = 0usize;

    row::<Integer<8>>("Integer<8>", &mut agree, &mut disagree);
    row::<Integer<4>>("Integer<4>", &mut agree, &mut disagree);
    row::<UFixed<13, -4>>("UFixed<13,-4>", &mut agree, &mut disagree);
    row::<UFixed<3, -1>>("UFixed<3,-1>", &mut agree, &mut disagree);
    row::<Floating<11, -14, 4>>("Floating<11,-14,4>", &mut agree, &mut disagree);
    row::<Biased<7, -2, 1>>("Biased<7,-2,1>   phase 1/2", &mut agree, &mut disagree);
    row::<Biased<7, -2, 3>>("Biased<7,-2,3>   phase 3/2", &mut agree, &mut disagree);
    row::<Biased<7, -2, -1>>("Biased<7,-2,-1>  phase -1/2", &mut agree, &mut disagree);
    row::<Biased<7, -2, 0>>("Biased<7,-2,0>   phase 0/2", &mut agree, &mut disagree);
    row::<ShiftedGrid>("ShiftedGrid (outside impl)", &mut agree, &mut disagree);

    println!(
        "\n   control: {agree} agree, {disagree} disagree. Control {}.",
        if disagree == 0 { "HOLDS" } else { "FAILED" }
    );
    if disagree != 0 {
        println!("\n   The instrument is wrong. Stopping rather than reporting sections 2 and 3.");
        std::process::exit(2);
    }

    println!("\n== section 2: an even nonzero phase numerator over PHASE_DEN = 2. ==");
    println!("   A phase of a whole quantum. The crate's predicate reads PHASE_NUM == 0,");
    println!("   so it answers on the numerator rather than on the phase.\n");

    let mut agree2 = 0usize;
    let mut disagree2 = 0usize;
    row::<Biased<7, -2, 2>>("Biased<7,-2,2>   phase 1", &mut agree2, &mut disagree2);
    row::<Biased<7, -2, 4>>("Biased<7,-2,4>   phase 2", &mut agree2, &mut disagree2);
    row::<Biased<7, -2, -2>>("Biased<7,-2,-2>  phase -1", &mut agree2, &mut disagree2);
    row::<Biased<13, 0, 6>>("Biased<13,0,6>   phase 3", &mut agree2, &mut disagree2);
    row::<Biased<9, -8, 8>>("Biased<9,-8,8>   phase 4", &mut agree2, &mut disagree2);

    // The other side of the same predicate: a phase whose numerator is zero and
    // whose slot range does not admit the slot that would denote zero. Reachable
    // only through an outside `Slots` impl, which the trait permits.
    println!();
    let mut agree2b = 0usize;
    let mut disagree2b = 0usize;
    row::<PositiveOnly>(
        "PositiveOnly     phase 0, no slot 0",
        &mut agree2b,
        &mut disagree2b,
    );

    println!(
        "\n   section 2: {} disagreements over {} rows.",
        disagree2 + disagree2b,
        agree2 + disagree2 + agree2b + disagree2b
    );
    if disagree2 > 0 {
        bad += 1;
        println!("   FINDING: `has_additive_identity` answers on the numerator, not on the phase.");
        println!("   The phase is PHASE_NUM/PHASE_DEN, so a numerator that is a nonzero multiple");
        println!("   of the denominator is a whole number of quanta and leaves zero on the grid.");
    }

    println!("\n== section 3: two formats, different coordinates, one representable set. ==\n");

    let a = denotation::<ShiftedGrid>();
    let b = denotation::<ShiftedByPhase>();
    println!("   ShiftedGrid     coords: phase 0/1, slots [-7, 8],  quantum exponent 0");
    println!("   Biased<4,0,2>   coords: phase 2/2, slots [-8, 7],  quantum exponent 0");
    println!(
        "   both over ambient BinaryRationals, radix {}",
        <BinaryRationals as Ambient>::RADIX
    );
    println!("   |set A| = {}, |set B| = {}", a.len(), b.len());
    println!(
        "   set A = {:?}",
        a.iter()
            .map(|q| q.num as f64 / q.den as f64)
            .collect::<Vec<_>>()
    );
    println!(
        "   set B = {:?}",
        b.iter()
            .map(|q| q.num as f64 / q.den as f64)
            .collect::<Vec<_>>()
    );
    let same = a == b;
    println!(
        "   the two sets are {}",
        if same { "IDENTICAL" } else { "different" }
    );

    // The control for section 3: a pair that must NOT be identical, or "identical"
    // is what this instrument says about everything.
    let c = denotation::<Integer<4>>();
    println!(
        "   control: Integer<4> against ShiftedGrid are {}",
        if a == c {
            "IDENTICAL (control FAILED)"
        } else {
            "different (control holds)"
        }
    );
    if a == c {
        println!("   The set comparison is degenerate. Section 3 establishes nothing.");
        std::process::exit(2);
    }

    if same {
        bad += 1;
        println!(
            "\n   FINDING: two distinct `Format` types over one ambient domain denote one set."
        );
        println!(
            "   `ruling::the_format_spine_is_canon` identifies a format by its ambient domain"
        );
        println!("   and its representable set. Nothing in the crate quotients the coordinates by");
        println!(
            "   the set they denote, so the ratified identity and the shipped identity are two"
        );
        println!("   different relations and their agreement is a theorem nobody has stated.");
    }

    println!("\n== verdict ==");
    println!("   findings: {bad}");
    std::process::exit(if bad == 0 { 0 } else { 1 });
}

/// A slot range admitting no slot zero, from outside the crate.
///
/// The second half of section 2: `has_additive_identity` already conjoins
/// `slot_in_range(0)`, so this row is expected to AGREE, and it is here so the
/// section reports the predicate's correct half beside its wrong one rather than
/// only the half that fails.
struct PositiveSlots;

impl Slots for PositiveSlots {
    const MIN: i64 = 1;
    const MAX: i64 = 8;
    const WIDTH: Width = Width::bits(4);
}

struct PositiveOnly;

impl Format for PositiveOnly {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = PositiveSlots;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}
