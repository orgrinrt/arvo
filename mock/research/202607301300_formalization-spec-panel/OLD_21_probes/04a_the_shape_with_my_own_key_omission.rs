// 21_probes/04a: the FIRST attempt at probe 04's shape, kept because it commits the exact bug
// the shape exists to forbid, and the shape did not catch it.
//
// Two rows of its own output are false:
//
//   Wrap  Mul  arity 5  acc frac 2  ->  true, "commutes"
//     `recovery_class<R>()` is keyed on the resolution alone. File 18 section 5 proved the class
//     is a property of the PAIR `(phi, Op)`. Wrapping is a congruence for addition; for
//     fixed-point multiplication the unconditional `>> FRAC` is not a homomorphism at any range,
//     which file 17 section 5.4 measured directly. So this row asserts a false thing about
//     multiplication for exactly the reason file 18 predicted any composition-keyed vocabulary
//     would.
//
//   Saturate  Add  arity 5  acc frac 2  ->  true, "deferred to the root"
//     `interior_exact_frac` returns only the FRACTIONAL half of the side condition, with a
//     comment saying the integer half "is the same shape and is elided here to keep the probe
//     short". Addition's whole requirement lives in the integer half. Probe 02 measured that
//     signed saturating addition at arity 5 needs an accumulator four times the numeral's range;
//     this says the numeral itself will do.
//
// Both are the same error and neither was caught, because the omission is in a SIDE-CONDITION
// FUNCTION rather than in the law's own signature. The scope discipline in probe 01 constrains
// what a body may name; it says nothing about whether a helper the body calls has a complete key
// of its own. That is the honest limit of the mechanism and it is why `04_the_shape_compiled.rs`
// carries the candidate-key sweep down into the helpers rather than only at the law.
//
// Kept unmodified as the audit trail. Run it to reproduce the two false rows.

// 21_probes/04: the proposed shape, compiled end to end, under the feature bans.
//
// Probes 01, 02 and 03 established three things separately. This one puts them in one program to
// check that the shape is expressible at all, because this dive's record is that each unbuilt
// shape had a hole the next member found by compiling it.
//
// The shape:
//
//   1. A derived fact is a `const fn` whose PARAMETERS ARE ITS KEY. Nothing else declares a key.
//   2. The accumulator is in the key, but is not searched over: the side condition is a closed
//      form, `interior_exact(Op, n)`, evaluated as a const bound.
//   3. The operation is in the key of every law, and enters through one derived predicate.
//   4. The use site recomputes the fact from its own parameters, so a fact proven at one key
//      cannot be consumed at another.
//
// Arms:
//   (default)              the shape; prints what each composition licenses
//   --cfg consume_wrong    a combinator consuming a fact proven at a different key
//   --cfg lie_at_the_leaf  a resolution declaring a structural class it does not have
//
// No `#![feature(..)]` anywhere. `generic_const_exprs` is not reachable from this shape, which
// is the point: every const computation over generic parameters happens inside a function body,
// never in type position, which is the one door file 19 section 1 found open.

// ------------------------------------------------------------------------------------------
// The axes, as const-generic tags. `adt_const_params` would give these real marker types; a
// tag keeps this probe feature-free and changes nothing about the shape.
// ------------------------------------------------------------------------------------------

pub const WRAP: u8 = 0;
pub const SATURATE: u8 = 1;
pub const REFUSE: u8 = 2;

pub const ADD: u8 = 0;
pub const MUL: u8 = 1;

const fn res_name(r: u8) -> &'static str {
    match r {
        WRAP => "Wrap",
        SATURATE => "Saturate",
        _ => "Refuse",
    }
}
const fn op_name(o: u8) -> &'static str {
    if o == ADD {
        "Add"
    } else {
        "Mul"
    }
}

// ------------------------------------------------------------------------------------------
// 1. Leaf facts. Each is a const fn keyed on exactly what it is about, and nothing more.
// ------------------------------------------------------------------------------------------

/// Structural class of the recovery map, file 18 section 4. Keyed on the resolution alone,
/// because it is a fact about `phi` at a single argument and mentions no operation.
/// 0 = homomorphism, 1 = partial identity, 2 = retraction, 3 = none.
const fn recovery_class<const R: u8>() -> u8 {
    match R {
        WRAP => 0,
        REFUSE => 1,
        SATURATE => 2,
        _ => 3,
    }
}

/// Does the exact image of `OP` on two representable values land back on the numeral's own
/// lattice? File 18 section 5's precondition. Keyed on the NUMERAL and the OPERATION: it is
/// the single predicate through which the operation enters every downstream law, which is what
/// keeps the vocabulary from multiplying per operation.
const fn lattice_closed<const FRAC: u32, const OP: u8>() -> bool {
    match OP {
        // a sum of two multiples of the quantum is a multiple of the quantum, at every width
        ADD => true,
        // a product of two values carrying FRAC fractional bits carries 2*FRAC
        _ => FRAC == 0,
    }
}

/// The interior-exactness side condition, probes 02 and 03. The accumulator must hold the exact
/// image of `OP` applied to at most `ARITY - 1` elements, so the recovery map fires at most once
/// per grouping, at the root, where it cannot see the grouping. Returns the required fractional
/// width; the integer half is the same shape and is elided here to keep the probe short.
const fn interior_exact_frac<const FRAC: u32, const OP: u8, const ARITY: u32>() -> u32 {
    if ARITY < 2 {
        return FRAC;
    }
    match OP {
        ADD => FRAC,
        _ => FRAC * (ARITY - 1),
    }
}

// ------------------------------------------------------------------------------------------
// 2. The law. Its key is its signature: numeral fractional width, resolution, operation,
//    accumulator fractional width, arity. Every one of the five is used in the body, and the
//    body cannot reach anything the signature does not bind.
// ------------------------------------------------------------------------------------------

/// Does every grouping of an `ARITY`-element fold agree, on the numbers it returns?
const fn regrouping_agrees<
    const FRAC: u32,
    const R: u8,
    const OP: u8,
    const ACC_FRAC: u32,
    const ARITY: u32,
>() -> bool {
    // (i) the recovery map commutes with the operation: it may be applied at every step.
    let commutes = recovery_class::<R>() == 0;
    // (ii) the recovery map is deferred to the root: applied once, to the exact result.
    let deferred = ACC_FRAC >= interior_exact_frac::<FRAC, OP, ARITY>();
    // (iii) or the exact image never leaves the lattice, so no in-range rounding fires at all.
    let never_rounds = lattice_closed::<FRAC, OP>() && recovery_class::<R>() == 1;
    commutes || deferred || never_rounds
}

// ------------------------------------------------------------------------------------------
// 3. The key-completeness check from probe 01, generalised: the verdict must move only when a
//    parameter IN the key moves. A candidate parameter left out of the key has to be shown not
//    to move it. Here the candidate is the accumulator, which is the one file 18 found missing.
// ------------------------------------------------------------------------------------------

#[cfg(consume_wrong)]
const fn verdict_independent_of_accumulator<
    const FRAC: u32,
    const R: u8,
    const OP: u8,
    const ARITY: u32,
>() -> bool {
    regrouping_agrees::<FRAC, R, OP, 2, ARITY>() == regrouping_agrees::<FRAC, R, OP, 16, ARITY>()
}

/// A combinator that regroups. It states its OWN arity and its OWN accumulator, and recomputes
/// the fact at them. There is no way to hand it a fact proven elsewhere at another key, because
/// there is no fact object to hand: the only fact is this call.
pub fn regrouping_fold<
    const FRAC: u32,
    const R: u8,
    const OP: u8,
    const ACC_FRAC: u32,
    const ARITY: u32,
>() -> &'static str {
    const {
        assert!(
            regrouping_agrees::<FRAC, R, OP, ACC_FRAC, ARITY>(),
            "this combinator regroups, and at its own arity and accumulator the composition's \
             groupings do not agree; widen the accumulator or pick a resolution that commutes"
        );
    }
    "regrouped"
}

/// The same combinator written the way the draft keys its law: on the composition alone, with
/// the accumulator and arity defaulted invisibly. Compiles, and licenses a regrouping the
/// measurement in probe 02 says is unsound.
#[cfg(consume_wrong)]
pub fn regrouping_fold_as_drafted<const FRAC: u32, const R: u8>() -> &'static str {
    const {
        assert!(
            regrouping_agrees::<FRAC, R, ADD, FRAC, 2>(),
            "the composition does not fold"
        );
    }
    "regrouped, at a key nobody named"
}

// ------------------------------------------------------------------------------------------
// 4. The leaf is where trust bottoms out, and it is checkable against the map's own behaviour.
// ------------------------------------------------------------------------------------------

const fn phi_model(r: u8, x: i64, lo: i64, hi: i64) -> Option<i64> {
    if x >= lo && x <= hi {
        return Some(x);
    }
    match r {
        WRAP => {
            let m = hi - lo + 1;
            let mut v = (x - lo) % m;
            if v < 0 {
                v += m;
            }
            Some(v + lo)
        }
        SATURATE => Some(if x > hi { hi } else { lo }),
        _ => None,
    }
}

/// The declared class checked against the model at a small width, exhaustively. A resolution
/// that declares a class it does not have fails at its own declaration site, not at a use site
/// three crates away.
const fn class_agrees_with_model<const R: u8>() -> bool {
    let (lo, hi) = (-4i64, 3i64);
    let declared = recovery_class::<R>();
    let mut is_hom = true;
    let mut is_pid = true;
    let mut x = lo * 3;
    while x <= hi * 3 {
        match phi_model(R, x, lo, hi) {
            Some(v) if v != x => is_pid = false,
            _ => {}
        }
        let mut y = lo * 3;
        while y <= hi * 3 {
            let direct = phi_model(R, x + y, lo, hi);
            let staged = match (phi_model(R, x, lo, hi), phi_model(R, y, lo, hi)) {
                (Some(a), Some(b)) => phi_model(R, a + b, lo, hi),
                _ => None,
            };
            let same = match (direct, staged) {
                (None, None) => true,
                (Some(a), Some(b)) => a == b,
                _ => false,
            };
            if !same {
                is_hom = false;
            }
            y += 1;
        }
        x += 1;
    }
    match declared {
        0 => is_hom,
        1 => is_pid,
        _ => !is_hom,
    }
}

#[cfg(lie_at_the_leaf)]
const fn recovery_class_lying<const R: u8>() -> u8 {
    // Saturate declared as a homomorphism, which probe 02 measured it is not.
    if R == SATURATE {
        0
    } else {
        recovery_class::<R>()
    }
}

#[cfg(lie_at_the_leaf)]
const fn lying_class_agrees<const R: u8>() -> bool {
    let (lo, hi) = (-4i64, 3i64);
    let declared = recovery_class_lying::<R>();
    let mut is_hom = true;
    let mut x = lo * 3;
    while x <= hi * 3 {
        let mut y = lo * 3;
        while y <= hi * 3 {
            let direct = phi_model(R, x + y, lo, hi);
            let staged = match (phi_model(R, x, lo, hi), phi_model(R, y, lo, hi)) {
                (Some(a), Some(b)) => phi_model(R, a + b, lo, hi),
                _ => None,
            };
            let same = match (direct, staged) {
                (None, None) => true,
                (Some(a), Some(b)) => a == b,
                _ => false,
            };
            if !same {
                is_hom = false;
            }
            y += 1;
        }
        x += 1;
    }
    declared != 0 || is_hom
}

fn main() {
    // the leaf declarations are checked at the point they are declared
    const _: () = assert!(class_agrees_with_model::<WRAP>());
    const _: () = assert!(class_agrees_with_model::<SATURATE>());
    const _: () = assert!(class_agrees_with_model::<REFUSE>());

    #[cfg(lie_at_the_leaf)]
    const _LIE: () = assert!(
        lying_class_agrees::<SATURATE>(),
        "this resolution declares a structural class the model refutes"
    );

    #[cfg(not(any(consume_wrong, lie_at_the_leaf)))]
    {
        println!("the fact, keyed on all five, at a Q2.2 numeral (FRAC = 2)\n");
        println!(
            "{:<10} {:<5} {:>6} {:>7} {:>10}  {}",
            "resolution", "op", "arity", "acc frac", "agrees", "why"
        );
        macro_rules! row {
            ($r:expr, $o:expr, $n:expr, $af:expr) => {{
                let v = regrouping_agrees::<2, { $r }, { $o }, { $af }, { $n }>();
                let why = if recovery_class::<{ $r }>() == 0 {
                    "commutes"
                } else if $af >= interior_exact_frac::<2, { $o }, { $n }>() {
                    "deferred to the root"
                } else if lattice_closed::<2, { $o }>() && recovery_class::<{ $r }>() == 1 {
                    "never rounds"
                } else {
                    "nothing licenses it"
                };
                println!(
                    "{:<10} {:<5} {:>6} {:>7} {:>10}  {}",
                    res_name($r),
                    op_name($o),
                    $n,
                    $af,
                    v,
                    why
                );
            }};
        }
        row!(WRAP, ADD, 5, 2);
        row!(WRAP, MUL, 5, 2);
        row!(REFUSE, ADD, 5, 2);
        row!(REFUSE, MUL, 5, 2);
        row!(REFUSE, MUL, 5, 8);
        row!(SATURATE, ADD, 5, 2);
        row!(SATURATE, ADD, 5, 8);
        row!(SATURATE, MUL, 5, 8);

        println!("\nand a combinator that regroups recomputes it at its own key:");
        println!(
            "  regrouping_fold::<2, REFUSE, MUL, 8, 5>() = {}",
            regrouping_fold::<2, REFUSE, MUL, 8, 5>()
        );
        // the next line does not compile; ACC_FRAC = 2 is below interior_exact_frac(MUL, 5) = 8
        // println!("{}", regrouping_fold::<2, REFUSE, MUL, 2, 5>());
    }

    #[cfg(consume_wrong)]
    {
        println!(
            "the drafted key, compiled clean: {}",
            regrouping_fold_as_drafted::<2, SATURATE>()
        );
        println!(
            "  its own claim of accumulator-independence: {}",
            verdict_independent_of_accumulator::<2, SATURATE, ADD, 5>()
        );
        println!("  ... which is false, and nothing in the drafted shape asks.");
        // this is the line the shape refuses
        println!("{}", regrouping_fold::<2, REFUSE, MUL, 2, 5>());
    }
}
