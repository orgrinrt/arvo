// Probe: is the ambient domain's operation family a coordinate of the format
// concept as shipped, or is it a doc comment?
//
// `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`
// is ratified and says: "an operation is admitted exactly when it is a function
// of the declared signature, and where two realisations of one name disagree,
// the signature is missing a coordinate."
//
// So the question this asks is the ratified rule's own diagnostic, pointed at the
// ambient: declare two ambients that are DIFFERENT ALGEBRAS and identical in the
// two coordinates `Ambient` carries, and ask whether anything in the crate can
// tell them apart.
//
// The case that must fail, stated before the run: the negative control varies
// RADIX, which IS a declared coordinate, and every observation must move. If the
// control does not move, the instrument observes nothing and the main arm's
// agreement means nothing.

use arvo_format::ambient::Ambient;
use arvo_format::format::{contains, has_additive_identity, radix, step_exponent, Format};
use arvo_format::quantum::{is_constant_family, Constant};
use arvo_format::slots::{Signed, Slots};

// --- arm 1: the rationals under (+, *), radix 2, signed ----------------------
struct RationalsUnderPlusTimes;
impl Ambient for RationalsUnderPlusTimes {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

// --- arm 2: the TROPICAL semiring over the same carrier ------------------------
// (min, +) rather than (+, *). A genuinely different algebra: no additive
// inverse, addition idempotent, multiplication distributes over min. Nothing
// about it is the rationals under plus and times.
struct TropicalMinPlus;
impl Ambient for TropicalMinPlus {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

// --- arm 3: the two-element Boolean algebra ------------------------------------
// (and, or). Not about magnitude at all, which is exactly what
// `question::is_number_system_broad_enough_for_non_magnitude` asks about.
struct BooleanAlgebra;
impl Ambient for BooleanAlgebra {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

// --- arm 4: the interval algebra over the binary rationals ---------------------
// A SET-VALUED ambient: each element is a set of rationals.
// `question::are_set_valued_carriers_admitted` asks whether this is in or out.
struct IntervalAlgebra;
impl Ambient for IntervalAlgebra {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

// --- the negative control: same everything, RADIX moved ------------------------
struct ControlRadixTen;
impl Ambient for ControlRadixTen {
    const RADIX: u32 = 10;
    const SIGNED: bool = true;
}

macro_rules! fmt_over {
    ($name:ident, $amb:ty) => {
        struct $name;
        impl Format for $name {
            type Ambient = $amb;
            type Quantum = Constant<-3>;
            type Slots = Signed<8>;
            const PHASE_NUM: i64 = 0;
            const PHASE_DEN: i64 = 1;
        }
    };
}

fmt_over!(FRationals, RationalsUnderPlusTimes);
fmt_over!(FTropical, TropicalMinPlus);
fmt_over!(FBoolean, BooleanAlgebra);
fmt_over!(FInterval, IntervalAlgebra);
fmt_over!(FControl, ControlRadixTen);

/// Every observation the crate offers about a format, as one tuple.
///
/// If two formats give the same tuple, nothing in this crate distinguishes them.
fn observe<F: Format>() -> (u32, bool, i32, bool, bool, bool, i64, i64, bool) {
    (
        radix::<F>(),
        <F::Ambient as Ambient>::SIGNED,
        step_exponent::<F>(0),
        is_constant_family::<F::Quantum>(),
        has_additive_identity::<F>(),
        contains::<F>(0, 0),
        <F::Slots as Slots>::MIN,
        <F::Slots as Slots>::MAX,
        contains::<F>(<F::Slots as Slots>::MAX, 0),
    )
}

fn main() {
    let rationals = observe::<FRationals>();
    let tropical = observe::<FTropical>();
    let boolean = observe::<FBoolean>();
    let interval = observe::<FInterval>();
    let control = observe::<FControl>();

    println!("rationals (+,*) : {rationals:?}");
    println!("tropical (min,+): {tropical:?}");
    println!("boolean (and,or): {boolean:?}");
    println!("interval algebra: {interval:?}");
    println!("CONTROL radix 10: {control:?}");
    println!();

    let main_arm_agrees = rationals == tropical && rationals == boolean && rationals == interval;
    let control_moved = rationals != control;

    println!("four different algebras indistinguishable : {main_arm_agrees}");
    println!("negative control (RADIX) moved            : {control_moved}");
    println!();

    assert!(
        control_moved,
        "NEGATIVE CONTROL FAILED: the instrument cannot see a declared coordinate \
         move, so its agreement finding is worthless"
    );

    if main_arm_agrees {
        println!(
            "FINDING: `Ambient` declares RADIX and SIGNED and no operation family, so \
             the rationals under plus and times, the tropical semiring under min and \
             plus, the two-element Boolean algebra and the interval algebra over the \
             rationals are one ambient domain to every function this crate ships. \
             Under the ratified rule that where two realisations of one name disagree \
             the signature is missing a coordinate, the signature is missing the \
             operation family."
        );
    } else {
        println!("REFUTED: something distinguishes them. The finding does not hold.");
    }
}
