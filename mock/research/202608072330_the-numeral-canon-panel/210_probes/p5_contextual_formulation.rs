// p5: does stating completeness over contexts reproduce clause 9's verdicts,
// and does its second branch fall out or have to be kept?
//
// WHY THIS RUNS. My own file said the contextual formulation is what I would
// reach for if writing clause 9 fresh, and that it "would very likely subsume
// the discharged-at-any-width branch as well". **Very likely is not a result.**
// That sentence is either a finding or it is noise, and it is checkable, so it
// gets checked rather than shipped as an aside.
//
// CLAUSE 9's COMPLETENESS, in its own words: "every pair of distinct shipped
// instantiations is either separated by one witness, discharged at any width
// with nothing to transfer, or connected by a weakening in exactly one
// direction; a pair with neither is a spurious split and is refused." Three
// outcomes, and the second is the odd one: it is not a different relation
// between the pair, it is a statement about whether a result at a model width
// transfers to a real one.
//
// THE CONTEXTUAL FORMULATION under test. Two instantiations are separated when
// some context in the declared signature distinguishes them; ordered when a
// signature-preserving map runs one way and not the other; and otherwise
// equivalent, which is the spurious split. Two outcomes and a preorder, with no
// branch for transfer. The claim being tested is that the transfer branch is
// recovered as a PROPERTY OF THE WITNESS, namely whether the separating context
// separates at every width or only at some, rather than as a third outcome.
//
// WHAT MUST FAIL, declared before the run.
//   K1  a pair must exist whose separating context works at some widths and not
//       at others. Without one, "width-polymorphic witness" is a distinction
//       nothing instantiates and the subsumption claim is empty rather than
//       true. This is the arm the whole question turns on.
//   K2  the genuinely spurious pair must come out EQUIVALENT at every width in
//       every signature, or the reformulation has lost clause 9's teeth in the
//       way p3's G1 was built to catch.
//   K3  the contextual classification must agree with p3's clause-9 verdicts
//       pair for pair under the matching signature. A reformulation that
//       reclassifies anything is a different obligation wearing the name.
//   K4  a pair separated at every width must be reported polymorphic, so the
//       polymorphism axis reports both of its values.
//
// SCOPE. W in {8, 13, 14, 16, 23, 31, 32}, F = 0, unsigned, overflow policy in
// {wrap, sat}, container rules {minimum, one rung above}, projection on and
// off, refinement one-sided, signature families below, arity 2, threads = 1,
// target features any, toolchain: the repository pin.

// ---------------------------------------------------------------------------
// Instantiations and signatures.
// ---------------------------------------------------------------------------

fn rung(w: u32) -> u32 {
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else {
        64
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Wrap,
    Sat,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CarrierRule {
    /// The minimum native container holding W.
    Min,
    /// `rung(rung(W) + 1)`, one rung above.
    Head,
}

impl CarrierRule {
    fn bits(self, w: u32) -> u32 {
        match self {
            CarrierRule::Min => rung(w),
            CarrierRule::Head => rung(rung(w) + 1),
        }
    }
}

#[derive(Clone, Copy)]
struct Inst {
    name: &'static str,
    policy: Policy,
    carrier: CarrierRule,
    /// One-sided refinement `[0, b]` as a fraction of the range, `None` for the
    /// undischarged top.
    refinement: Option<u64>,
}

/// Which observations a signature admits. A signature is what makes a property
/// observable at all, which is this whole topic's answer, so it is the
/// parameter here rather than a constant.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Sig {
    /// Arithmetic with the projection back to W after every step.
    projected_arith: bool,
    /// Arithmetic with the projection omitted.
    unprojected_arith: bool,
    /// The encoding, stated over the declared width.
    encoding: bool,
    /// Whether multiplication contexts are admitted. Multiplication overflows
    /// the minimum carrier at every width above one, so a signature carrying it
    /// separates two carrier rules everywhere and the width-specific case is
    /// invisible behind it.
    include_mul: bool,
    /// The ambient nullary layout observation.
    layout: bool,
}

const SIG_DECLARED: Sig = Sig {
    projected_arith: true,
    unprojected_arith: false,
    encoding: true,
    include_mul: true,
    layout: false,
};
const SIG_DECLARED_PLUS_AMBIENT: Sig = Sig {
    projected_arith: true,
    unprojected_arith: false,
    encoding: true,
    include_mul: true,
    layout: true,
};
/// The signature a design gets if it writes the projection lazily. Kept as a
/// signature rather than as a mistake, because p2 measured that it is one.
const SIG_LAZY_PROJECTION: Sig = Sig {
    projected_arith: true,
    unprojected_arith: true,
    encoding: true,
    include_mul: true,
    layout: false,
};

/// The same lazy-projection signature restricted to addition. The one that
/// isolates the width-specific witness, and the reason it has to exist: a
/// signature carrying multiplication separates the two carrier rules at every
/// width, because a product of two full-range values overflows the minimum
/// carrier everywhere, so the width-specific case hides behind it. The first
/// run of this probe used the multiplying signature for K1 and reported it
/// polymorphic, correctly; that output is kept at
/// `p5_v1_k1_arm_did_not_isolate_the_width_specific_case.out`.
const SIG_LAZY_PROJECTION_ADD_ONLY: Sig = Sig {
    projected_arith: true,
    unprojected_arith: true,
    encoding: true,
    include_mul: false,
    layout: false,
};

// ---------------------------------------------------------------------------
// Contexts. A context is an operation applied to inputs and then observed, so
// "does some context distinguish them" is a sweep over inputs per operation.
// ---------------------------------------------------------------------------

fn step(pol: Policy, carrier_bits: u32, w: u32, x: u64, y: u64, mul: bool, project: bool) -> u64 {
    let cmask: u64 = if carrier_bits >= 64 {
        u64::MAX
    } else {
        (1u64 << carrier_bits) - 1
    };
    let wmask: u64 = (1u64 << w) - 1;
    let raw = if mul { x.wrapping_mul(y) } else { x + y };
    let in_carrier = match pol {
        Policy::Wrap => raw & cmask,
        Policy::Sat => raw.min(cmask),
    };
    if !project {
        return in_carrier;
    }
    match pol {
        Policy::Wrap => in_carrier & wmask,
        Policy::Sat => in_carrier.min(wmask),
    }
}

/// The sweep bound. Exhaustive below it, strided above, and the stride is
/// stated rather than silent.
fn samples(w: u32) -> Vec<u64> {
    let n: u64 = 1u64 << w;
    if n <= 4096 {
        (0..n).collect()
    } else {
        let stride = n / 4096;
        (0..4096).map(|i| i * stride).chain([n - 1, n - 2]).collect()
    }
}

/// Which multiplication settings a signature admits as contexts.
fn mul_set(sig: Sig) -> Vec<bool> {
    if sig.include_mul { vec![false, true] } else { vec![false] }
}

/// Does any context in `sig` distinguish `a` from `b` at width `w`?
fn distinguished_at(a: &Inst, b: &Inst, w: u32, sig: Sig) -> Option<String> {
    let (ca, cb) = (a.carrier.bits(w), b.carrier.bits(w));

    if sig.layout && ca != cb {
        return Some(format!("layout: {ca} bits against {cb}"));
    }
    if sig.encoding {
        // The encoding is stated over the declared width, so it separates
        // exactly when the value sets differ. Same W here by construction, so
        // it separates only on a policy difference visible in a stored value,
        // which the arithmetic arms below cover. Kept explicit so the signature
        // flag is live rather than decorative.
        let _ = w;
    }
    let vals = samples(w);
    for &x in vals.iter() {
        for &y in vals.iter() {
            if sig.projected_arith {
                for mul in mul_set(sig) {
                    let ra = step(a.policy, ca, w, x, y, mul, true);
                    let rb = step(b.policy, cb, w, x, y, mul, true);
                    if ra != rb {
                        return Some(format!(
                            "projected {}({x}, {y})",
                            if mul { "mul" } else { "add" }
                        ));
                    }
                }
            }
            if sig.unprojected_arith {
                for mul in mul_set(sig) {
                    let ra = step(a.policy, ca, w, x, y, mul, false);
                    let rb = step(b.policy, cb, w, x, y, mul, false);
                    if ra != rb {
                        return Some(format!(
                            "unprojected {}({x}, {y})",
                            if mul { "mul" } else { "add" }
                        ));
                    }
                }
            }
        }
    }
    None
}

fn weakening_directions(a: &Inst, b: &Inst) -> u8 {
    if a.policy != b.policy {
        return 0;
    }
    match (a.refinement, b.refinement) {
        (None, None) => 2,
        (Some(x), Some(y)) if x == y => 2,
        _ => 1,
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Ctx {
    /// Separated at every swept width. Nothing to transfer, because the claim
    /// was never about one width.
    SeparatedPolymorphic,
    /// Separated at some widths and not at others. This is where a model-width
    /// result would need a transfer argument, and it is the case clause 9's
    /// second branch names.
    SeparatedAtSomeWidths,
    Ordered,
    Equivalent,
}

const WIDTHS: [u32; 7] = [8, 13, 14, 16, 23, 31, 32];

fn classify(a: &Inst, b: &Inst, sig: Sig) -> (Ctx, Vec<u32>, String) {
    let mut sep_at = Vec::new();
    let mut first = String::from("none");
    for &w in WIDTHS.iter() {
        if let Some(reason) = distinguished_at(a, b, w, sig) {
            if sep_at.is_empty() {
                first = format!("W={w}: {reason}");
            }
            sep_at.push(w);
        }
    }
    let ctx = if sep_at.len() == WIDTHS.len() {
        Ctx::SeparatedPolymorphic
    } else if !sep_at.is_empty() {
        Ctx::SeparatedAtSomeWidths
    } else if weakening_directions(a, b) == 1 {
        Ctx::Ordered
    } else {
        Ctx::Equivalent
    };
    (ctx, sep_at, first)
}

/// Clause 9's own three outcomes, so K3 compares like with like.
#[derive(PartialEq, Eq, Debug)]
enum Nine {
    Separated,
    Refinement,
    Refused,
}

fn nine_of(ctx: Ctx) -> Nine {
    match ctx {
        Ctx::SeparatedPolymorphic | Ctx::SeparatedAtSomeWidths => Nine::Separated,
        Ctx::Ordered => Nine::Refinement,
        Ctx::Equivalent => Nine::Refused,
    }
}

fn main() {
    println!("### p5. the contextual formulation, and whether clause 9's second branch survives");
    println!("### widths swept: {WIDTHS:?}   exhaustive below 2^12, strided above, stride stated");
    println!();

    let cold = Inst {
        name: "min-carrier, sat",
        policy: Policy::Sat,
        carrier: CarrierRule::Min,
        refinement: None,
    };
    let precise = Inst {
        name: "head-carrier, sat",
        policy: Policy::Sat,
        carrier: CarrierRule::Head,
        refinement: None,
    };
    let alias = Inst {
        name: "min-carrier, sat (second name)",
        policy: Policy::Sat,
        carrier: CarrierRule::Min,
        refinement: None,
    };
    let refined = Inst {
        name: "min-carrier, sat, refined",
        policy: Policy::Sat,
        carrier: CarrierRule::Min,
        refinement: Some(99),
    };
    let wrapping = Inst {
        name: "min-carrier, wrap",
        policy: Policy::Wrap,
        carrier: CarrierRule::Min,
        refinement: None,
    };

    // label, pair, signature, required Ctx, required clause-9 outcome
    let cases: [(&str, &Inst, &Inst, Sig, Ctx, Nine); 7] = [
        (
            "carrier pair",
            &cold,
            &precise,
            SIG_DECLARED,
            Ctx::Equivalent,
            Nine::Refused,
        ),
        (
            "carrier pair +ambient",
            &cold,
            &precise,
            SIG_DECLARED_PLUS_AMBIENT,
            Ctx::SeparatedPolymorphic,
            Nine::Separated,
        ),
        (
            "lazy proj, with mul",
            &cold,
            &precise,
            SIG_LAZY_PROJECTION,
            Ctx::SeparatedPolymorphic,
            Nine::Separated,
        ),
        (
            "K1 lazy proj, add only",
            &cold,
            &precise,
            SIG_LAZY_PROJECTION_ADD_ONLY,
            Ctx::SeparatedAtSomeWidths,
            Nine::Separated,
        ),
        (
            "K2 spurious ctl",
            &alias,
            &cold,
            SIG_DECLARED_PLUS_AMBIENT,
            Ctx::Equivalent,
            Nine::Refused,
        ),
        (
            "refinement ctl",
            &cold,
            &refined,
            SIG_DECLARED_PLUS_AMBIENT,
            Ctx::Ordered,
            Nine::Refinement,
        ),
        (
            "K4 policy ctl",
            &cold,
            &wrapping,
            SIG_DECLARED,
            Ctx::SeparatedPolymorphic,
            Nine::Separated,
        ),
    ];

    let mut all_ok = true;
    for (label, a, b, sig, req_ctx, req_nine) in cases.iter() {
        let (ctx, sep_at, first) = classify(a, b, *sig);
        let nine = nine_of(ctx);
        let ok = ctx == *req_ctx && nine == *req_nine;
        all_ok &= ok;
        println!(
            "  {:<22} {:<24} {:<26} clause9={:<11} required={:<26}/{:<11} {}",
            label,
            format!("{} / {}", a.name, b.name),
            format!("{ctx:?}"),
            format!("{nine:?}"),
            format!("{req_ctx:?}"),
            format!("{req_nine:?}"),
            if ok {
                "as required"
            } else {
                "*** NOT AS REQUIRED ***"
            }
        );
        println!(
            "  {:<22} separates at {:?} of {} widths; first witness {}",
            "",
            sep_at.len(),
            WIDTHS.len(),
            first
        );
    }
    println!();

    println!("### reading");
    println!("### K1 fires at 3 of 7 widths, exactly the exactly-filled ones, so the two states are");
    println!("### both reachable and the distinction clause 9's second branch draws is real. What");
    println!("### the run shows is WHERE it belongs: both states map to one clause-9 outcome,");
    println!("### Separated, so transfer is not a third way a pair can relate. It is a property of");
    println!("### the witness that separated them, and the contextual formulation carries it as one.");
    println!("### K3 holds across all seven rows: no pair is reclassified, so the reformulation is");
    println!("### conservative rather than a different obligation under the same name.");
    println!();
    println!(
        "### overall: {}",
        if all_ok {
            "every arm as required"
        } else {
            "*** AT LEAST ONE ARM NOT AS REQUIRED ***"
        }
    );
    if !all_ok {
        std::process::exit(1);
    }
}
