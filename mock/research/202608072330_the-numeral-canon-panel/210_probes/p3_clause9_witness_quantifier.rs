// p3: clause 9 refuses the carrier pair because its witness quantifies over
// inputs. What happens when the quantifier is the one a Rust consumer has.
//
// WHY THIS RUNS. `163` p1 modelled clause 9's three-outcome question over the
// carrier pair and got `directions=2, witness=no -> REFUSED as a spurious
// split` on the footprint-internal branch. That reproduces here and is not
// disputed. What is disputed is the WITNESS SET the model inherited from
// clause 9's own wording: "every pair of distinct shipped instantiations is
// either separated by one witness, ... a pair with neither is a spurious split
// and is refused". A witness there is an INPUT. `size_of` takes no input.
//
// So clause 9 as written cannot see a nullary observation, and the container
// premise's two branches are, in that model, a knob toggling whether one is
// counted. Part A asks whether that knob describes anything a design can
// choose. Part B runs clause 9 under both witness sets over five pairs.
//
// WHAT MUST FAIL, declared before the run.
//   G1  the genuinely spurious pair (two names, ONE carrier, one V, one R)
//       must be REFUSED under BOTH witness sets. If widening the witness set
//       separates everything then clause 9 has no teeth left and the repair is
//       worse than the defect it repairs. This is the control the whole
//       argument rests on.
//   G2  the refinement pair must come out REFINEMENT under both, so the
//       widening does not reclassify an ordered pair.
//   G3  the carrier pair must be REFUSED under the input-only witness set,
//       reproducing `163` rather than disagreeing with it.
//   G4  a policy pair and a width pair must SEPARATE under both, so the
//       verdict function can report separation without help from part A.
//
// SCOPE. W in {13, 14}, F = 0, unsigned, overflow policy in {wrap, sat},
// carrier in {u16, u32}, refinement one-sided [0, b], signature
// {add, mul, encode}, arity 2, threads = 1, target features any,
// toolchain: the repository pin.

use core::marker::PhantomData;

// ===========================================================================
// PART A. Is the footprint-internal branch a design choice, or a claim about
// the host language? Not modelled: these are real types and `size_of` is the
// language's answer rather than this probe's.
// ===========================================================================

/// The shipped shape: a phantom strategy marker over a carrier.
#[repr(transparent)]
struct Num<C, S>(C, PhantomData<S>);

struct Cold;
struct Precise;

/// `Hot` and `Cold` take the minimum native container for W = 13.
type Cold13 = Num<u16, Cold>;
/// `Warm` and `Precise` take `rung(rung_bits(W) + 1)`.
type Precise13 = Num<u32, Precise>;

fn part_a() -> bool {
    println!("PART A. the ambient observation, as a language fact rather than a modelled flag");

    let s_cold = core::mem::size_of::<Cold13>();
    let s_prec = core::mem::size_of::<Precise13>();
    let a_cold = core::mem::align_of::<Cold13>();
    let a_prec = core::mem::align_of::<Precise13>();

    // That this compiles at all is the finding: `size_of` is not in arvo's
    // gift. It is available for every `Sized` type, so a design cannot
    // withhold it from a consumer, only decline to talk about it.
    println!("  size_of::<Cold13>()    = {s_cold}   (Hot/Cold rule: minimum native)");
    println!("  size_of::<Precise13>() = {s_prec}   (Warm/Precise rule: rung(rung_bits(13)+1))");
    println!("  align_of               = {a_cold} against {a_prec}");

    let separates = s_cold != s_prec;
    println!(
        "  the ambient observation separates the pair: {}   required=true   {}",
        separates,
        if separates {
            "as required"
        } else {
            "*** NOT AS REQUIRED ***"
        }
    );
    println!("  reading: the footprint-internal branch does not describe a design arvo could");
    println!("  ship. It describes a host language in which `size_of` is unavailable on a");
    println!("  `Sized` type.");
    println!();
    separates
}

// ===========================================================================
// PART B. Clause 9's verdict function, parameterised by the witness set.
// ===========================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Wrap,
    Sat,
}

/// One shipped instantiation.
#[derive(Clone, Copy)]
struct Inst {
    name: &'static str,
    /// Declared width. Fixes the value set at F = 0, unsigned.
    w: u32,
    /// The realisation map's range policy. Part of R.
    policy: Policy,
    /// Bytes of the machine carrier. NOT part of V and NOT part of R.
    carrier_bytes: usize,
    /// One-sided refinement `[0, b]`. `None` is the undischarged top.
    refinement: Option<u64>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WitnessSet {
    /// Clause 9 as written: witnesses are inputs, so only value-indexed
    /// operations count. This is the set `163`'s model inherited.
    InputsOnly,
    /// What a Rust consumer holds: the declared operations plus the ambient
    /// nullary layout observations part A shows cannot be withheld.
    InputsAndNullary,
}

#[derive(PartialEq, Eq, Debug)]
enum Verdict {
    Separated,
    Refinement,
    Refused,
}

/// Is there an INPUT on which the two instantiations' declared operations
/// disagree? Swept exhaustively over the declared domain, with the projection
/// back to W applied, which is the discipline p2 measures.
fn input_witness(a: &Inst, b: &Inst) -> Option<String> {
    // A different declared width is a different value set, so the encoding
    // separates at once and there is nothing to sweep.
    if a.w != b.w {
        return Some(format!("value sets differ: W={} against W={}", a.w, b.w));
    }
    let n: u64 = 1 << a.w;
    let mask: u64 = n - 1;
    let apply = |p: Policy, x: u64, y: u64, mul: bool| -> u64 {
        let raw = if mul { x * y } else { x + y };
        match p {
            Policy::Wrap => raw & mask,
            Policy::Sat => raw.min(mask),
        }
    };
    for x in 0..n {
        for y in 0..n {
            if apply(a.policy, x, y, false) != apply(b.policy, x, y, false) {
                return Some(format!("add({x}, {y})"));
            }
            if apply(a.policy, x, y, true) != apply(b.policy, x, y, true) {
                return Some(format!("mul({x}, {y})"));
            }
        }
    }
    None
}

/// The nullary observations. Available only in the wider witness set, and a
/// function of the carrier, never of a value.
fn nullary_witness(a: &Inst, b: &Inst) -> Option<String> {
    if a.carrier_bytes != b.carrier_bytes {
        Some(format!(
            "size_of {} against {}",
            a.carrier_bytes, b.carrier_bytes
        ))
    } else {
        None
    }
}

/// How many directions a total denotation-preserving weakening connects the
/// pair in. Two means connected both ways, which with no witness is clause 9's
/// spurious split.
fn weakening_directions(a: &Inst, b: &Inst) -> u8 {
    if a.w != b.w || a.policy != b.policy {
        return 0;
    }
    match (a.refinement, b.refinement) {
        // Neither restricts, or both restrict identically: each weakens to the
        // other, so both directions are available.
        (None, None) => 2,
        (Some(x), Some(y)) if x == y => 2,
        // One is strictly tighter: weakening runs one way only.
        _ => 1,
    }
}

fn clause9(a: &Inst, b: &Inst, ws: WitnessSet) -> (Verdict, u8, String) {
    let mut w = input_witness(a, b);
    if w.is_none() && ws == WitnessSet::InputsAndNullary {
        w = nullary_witness(a, b);
    }
    let dirs = weakening_directions(a, b);
    let verdict = if w.is_some() {
        Verdict::Separated
    } else if dirs == 1 {
        Verdict::Refinement
    } else {
        Verdict::Refused
    };
    (verdict, dirs, w.unwrap_or_else(|| "none".into()))
}

fn part_b() -> bool {
    // The five pairs. Every one is a pair of instantiations a design could
    // ship, and each isolates one axis.
    let cold13 = Inst {
        name: "Cold<13>",
        w: 13,
        policy: Policy::Sat,
        carrier_bytes: 2,
        refinement: None,
    };
    let precise13 = Inst {
        name: "Precise<13>",
        w: 13,
        policy: Policy::Sat,
        carrier_bytes: 4,
        refinement: None,
    };
    // Two names, one carrier, one V, one R: nothing anywhere separates these.
    let alias_a = Inst {
        name: "AliasA<13>",
        w: 13,
        policy: Policy::Sat,
        carrier_bytes: 2,
        refinement: None,
    };
    let alias_b = Inst {
        name: "AliasB<13>",
        w: 13,
        policy: Policy::Sat,
        carrier_bytes: 2,
        refinement: None,
    };
    let refined = Inst {
        name: "Cold<13>[0,99]",
        w: 13,
        policy: Policy::Sat,
        carrier_bytes: 2,
        refinement: Some(99),
    };
    let wrapping13 = Inst {
        name: "Warm<13>wrap",
        w: 13,
        policy: Policy::Wrap,
        carrier_bytes: 2,
        refinement: None,
    };
    let cold14 = Inst {
        name: "Cold<14>",
        w: 14,
        policy: Policy::Sat,
        carrier_bytes: 2,
        refinement: None,
    };

    // label, pair, required verdict under InputsOnly, under InputsAndNullary
    let cases: [(&str, &Inst, &Inst, Verdict, Verdict); 5] = [
        (
            "THE CARRIER PAIR",
            &cold13,
            &precise13,
            Verdict::Refused,
            Verdict::Separated,
        ),
        (
            "G1 spurious ctl",
            &alias_a,
            &alias_b,
            Verdict::Refused,
            Verdict::Refused,
        ),
        (
            "G2 refinement ctl",
            &cold13,
            &refined,
            Verdict::Refinement,
            Verdict::Refinement,
        ),
        (
            "G4 policy ctl",
            &cold13,
            &wrapping13,
            Verdict::Separated,
            Verdict::Separated,
        ),
        (
            "G4 width ctl",
            &cold13,
            &cold14,
            Verdict::Separated,
            Verdict::Separated,
        ),
    ];

    let mut all_ok = true;
    for ws in [WitnessSet::InputsOnly, WitnessSet::InputsAndNullary] {
        println!("PART B. clause 9 under witness set {ws:?}");
        for (label, a, b, req_i, req_n) in cases.iter() {
            let required = if ws == WitnessSet::InputsOnly {
                req_i
            } else {
                req_n
            };
            let (got, dirs, wit) = clause9(a, b, ws);
            let ok = &got == required;
            all_ok &= ok;
            println!(
                "  {:<18} {:<30} dirs={dirs} witness={:<30} got={:<11} required={:<11} {}",
                label,
                format!("{} / {}", a.name, b.name),
                wit,
                format!("{got:?}"),
                format!("{required:?}"),
                if ok {
                    "as required"
                } else {
                    "*** NOT AS REQUIRED ***"
                }
            );
        }
        println!();
    }
    all_ok
}

fn main() {
    println!("### p3. clause 9's witness quantifier, and what the container premise's knob toggles");
    println!();
    let a_ok = part_a();
    let b_ok = part_b();

    println!("### reading");
    println!("### G3 holds: under InputsOnly the carrier pair is REFUSED, which is `163` p1");
    println!("### reproduced rather than contradicted. G1 holds: widening the witness set does");
    println!("### NOT separate the genuinely spurious pair, so clause 9 keeps its teeth. What");
    println!("### moves is one pair, and it moves because the observation separating it is");
    println!("### nullary, which clause 9's wording quantifies past.");
    println!();
    println!(
        "### overall: {}",
        if a_ok && b_ok {
            "every arm as required"
        } else {
            "*** AT LEAST ONE ARM NOT AS REQUIRED ***"
        }
    );
    if !(a_ok && b_ok) {
        std::process::exit(1);
    }
}
