// 21_probes/04: the proposed shape, compiled end to end, under the feature bans.
//
// This is the SECOND attempt. The first is kept unmodified at `04a_the_shape_with_my_own_key_
// omission.rs`, because it committed the exact bug the shape exists to forbid, in two places,
// and the shape did not catch it: both omissions were in a SIDE-CONDITION FUNCTION rather than
// in the law's own signature. Read 04a's header before this file; it is the sharpest evidence
// here for why the mechanism needs the candidate sweep carried down into the helpers.
//
// The shape:
//
//   1. A derived fact is a `const fn` whose PARAMETERS ARE ITS KEY. Nothing else declares a key.
//   2. Every helper the fact calls has a complete key of its own, checked the same way. This is
//      what 04a got wrong.
//   3. The operation is in the key of every law, and enters through two derived predicates:
//      the recovery map's structural class at that operation, and the interior-exactness bound.
//   4. The accumulator is in the key but is not searched over: the side condition is a closed
//      form in the arity, evaluated as a const bound. Conservative, and sound, which is the side
//      to err on for a bound the compiler enforces (file 18 section 6's own standard).
//   5. The use site recomputes the fact from ITS OWN parameters. There is no fact object to
//      hand around, so a fact proven at one key cannot be consumed at another.
//
// Arms:
//   (default)              the shape; prints what each composition licenses and why
//   --cfg consume_wrong    a combinator regrouping at a key the fact does not hold at
//   --cfg lie_at_the_leaf  a resolution declaring a structural class the model refutes
//   --cfg omit_a_helper_key  the 04a bug, reintroduced deliberately, caught this time
//
// No `#![feature(..)]` anywhere. Every const computation over generic parameters happens inside
// a function body, never in type position, which is the one door file 19 section 1 found open
// under this workspace's ban on `generic_const_exprs`.

pub const WRAP: u8 = 0;
pub const SATURATE: u8 = 1;
pub const REFUSE: u8 = 2;

pub const ADD: u8 = 0;
pub const MUL: u8 = 1;

const HOMOMORPHISM: u8 = 0;
const PARTIAL_IDENTITY: u8 = 1;
const RETRACTION: u8 = 2;
const UNCLASSIFIED: u8 = 3;

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
const fn class_name(c: u8) -> &'static str {
    match c {
        HOMOMORPHISM => "homomorphism",
        PARTIAL_IDENTITY => "partial identity",
        RETRACTION => "retraction",
        _ => "unclassified",
    }
}

const fn ceil_log2(mut n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }
    n -= 1;
    let mut k = 0;
    while n > 0 {
        n >>= 1;
        k += 1;
    }
    k
}

// ------------------------------------------------------------------------------------------
// Leaf facts. Each is a const fn keyed on exactly what it is about. The two that mention an
// operation say so in their signature; that is the whole correction over 04a.
// ------------------------------------------------------------------------------------------

/// Does the exact image of `OP` on two representable values land back on the numeral's own
/// lattice? File 18 section 5's precondition, and the single predicate through which the
/// operation enters everything downstream.
///
/// Addition: a sum of two multiples of the quantum is a multiple of the quantum, at every
/// width. Multiplication: a product of two values carrying `FRAC` fractional bits carries
/// `2*FRAC`, so it lands on the lattice only when `FRAC` is zero.
const fn lattice_closed<const FRAC: u32, const OP: u8>() -> bool {
    match OP {
        ADD => true,
        _ => FRAC == 0,
    }
}

/// The recovery map's structural class AT AN OPERATION. Keyed on the pair, per file 18
/// section 5, and on the numeral's fractional width, because that is what decides whether the
/// operation's exact image stays on the lattice.
const fn recovery_class<const R: u8, const OP: u8, const FRAC: u32>() -> u8 {
    match R {
        // wrapping is a congruence for addition at every width. for fixed-point multiplication
        // the unconditional `>> FRAC` is not a homomorphism at any range, which file 17 section
        // 5.4 measured with the range removed entirely.
        WRAP => {
            if lattice_closed::<FRAC, OP>() {
                HOMOMORPHISM
            } else {
                UNCLASSIFIED
            }
        }
        // refusing returns its argument unchanged wherever it returns, but only if no in-range
        // rounding fires first, which is exactly lattice closure.
        REFUSE => {
            if lattice_closed::<FRAC, OP>() {
                PARTIAL_IDENTITY
            } else {
                UNCLASSIFIED
            }
        }
        // clamping is total, fixes the representable set, and preserves order, at both.
        SATURATE => RETRACTION,
        _ => UNCLASSIFIED,
    }
}

/// The interior-exactness side condition, probes 02 and 03: the accumulator must hold the exact
/// image of `OP` applied to at most `ARITY - 1` elements, so the recovery map fires at most once
/// per grouping, at the root, where it cannot see the grouping.
///
/// Returns `(integer bits, fractional bits)`. 04a returned only the second and called the first
/// "the same shape, elided"; addition's entire requirement lives in the first.
///
/// Conservative by construction: probe 03 measured signed addition at arity 4 needing one extra
/// integer bit where this asks for two. Sound and loose beats tight and searched, for a bound
/// the compiler enforces.
const fn interior_exact<const INT: u32, const FRAC: u32, const OP: u8, const ARITY: u32>(
) -> (u32, u32) {
    if ARITY < 3 {
        return (INT, FRAC);
    }
    let m = ARITY - 1;
    match OP {
        // the sum of m values needs ceil(log2 m) extra integer bits; the quantum never moves
        ADD => (INT + ceil_log2(m), FRAC),
        // the product of m values needs m times both
        _ => (INT * m, FRAC * m),
    }
}

// ------------------------------------------------------------------------------------------
// The law. Its key is its signature. All six parameters are read in the body.
// ------------------------------------------------------------------------------------------

/// Does every grouping of an `ARITY`-element fold agree, INCLUDING on whether it returns?
/// Kleene agreement, probe 02's relation.
const fn regrouping_agrees<
    const INT: u32,
    const FRAC: u32,
    const R: u8,
    const OP: u8,
    const ACC_INT: u32,
    const ACC_FRAC: u32,
    const ARITY: u32,
>() -> bool {
    let class = recovery_class::<R, OP, FRAC>();
    // (i) the recovery map commutes with the operation: it may be applied at every step
    if class == HOMOMORPHISM {
        return true;
    }
    // (ii) the recovery map is deferred to the root: applied once, to the exact result, where
    //      it cannot see the grouping. the map's own class is irrelevant here, which is what
    //      probe 02 measured: three maps of three different classes, one identical threshold.
    let (need_int, need_frac) = interior_exact::<INT, FRAC, OP, ARITY>();
    ACC_INT >= need_int && ACC_FRAC >= need_frac
}

/// Does every grouping that returns at all return the same number? Existential agreement, which
/// is a strictly weaker fact and the one file 17 section 5.2 measured `Precise` to have and file
/// 18 section 4 proved follows from the partial-identity class at every arity.
///
/// The two are separate facts because they come apart, and a design that reports only their
/// conjunction puts `Precise` in the same column as signed clamping, which file 18 section 2
/// measured is wrong. This is the reason the law returns two verdicts rather than one.
const fn regrouping_agrees_existentially<
    const INT: u32,
    const FRAC: u32,
    const R: u8,
    const OP: u8,
    const ACC_INT: u32,
    const ACC_FRAC: u32,
    const ARITY: u32,
>() -> bool {
    if recovery_class::<R, OP, FRAC>() == PARTIAL_IDENTITY {
        return true;
    }
    regrouping_agrees::<INT, FRAC, R, OP, ACC_INT, ACC_FRAC, ARITY>()
}

/// The reason, for reporting. Not part of the fact.
const fn why<
    const INT: u32,
    const FRAC: u32,
    const R: u8,
    const OP: u8,
    const ACC_INT: u32,
    const ACC_FRAC: u32,
    const ARITY: u32,
>() -> &'static str {
    if recovery_class::<R, OP, FRAC>() == HOMOMORPHISM {
        return "commutes with the operation";
    }
    let (ni, nf) = interior_exact::<INT, FRAC, OP, ARITY>();
    if ACC_INT >= ni && ACC_FRAC >= nf {
        "deferred to the root"
    } else {
        "nothing licenses it"
    }
}

// ------------------------------------------------------------------------------------------
// Key completeness, carried down into the helpers. This is 04a's correction.
// ------------------------------------------------------------------------------------------

/// A helper that mentions no operation is claiming its verdict does not move with the
/// operation. That claim is cheap to test and 04a's first bug is exactly its failure.
const fn class_is_operation_independent<const R: u8, const FRAC: u32>() -> bool {
    recovery_class::<R, ADD, FRAC>() == recovery_class::<R, MUL, FRAC>()
}

/// Likewise for the accumulator bound and the arity.
const fn bound_is_arity_independent<const INT: u32, const FRAC: u32, const OP: u8>() -> bool {
    let (a, b) = interior_exact::<INT, FRAC, OP, 3>();
    let (c, d) = interior_exact::<INT, FRAC, OP, 9>();
    a == c && b == d
}

/// A combinator that regroups. It states its own arity and its own accumulator and recomputes
/// the fact at them. There is no fact object to hand it, so it cannot consume one proven
/// elsewhere at another key.
pub fn regrouping_fold<
    const INT: u32,
    const FRAC: u32,
    const R: u8,
    const OP: u8,
    const ACC_INT: u32,
    const ACC_FRAC: u32,
    const ARITY: u32,
>() -> &'static str {
    const {
        assert!(
            regrouping_agrees::<INT, FRAC, R, OP, ACC_INT, ACC_FRAC, ARITY>(),
            "this combinator regroups, and at its own operation, arity and accumulator the \
             composition's groupings do not agree; widen the accumulator, or pick a resolution \
             that commutes with this operation"
        );
    }
    "regrouped"
}

// ------------------------------------------------------------------------------------------
// The leaf is where trust bottoms out, and it is checkable against the map's own behaviour.
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

/// The declared class checked against the model at a small width, exhaustively, for addition.
/// A resolution declaring a class the model refutes fails at its own declaration site, not at a
/// use site three crates away.
const fn class_agrees_with_model<const R: u8>() -> bool {
    let (lo, hi) = (-4i64, 3i64);
    let declared = recovery_class::<R, ADD, 0>();
    let mut is_hom = true;
    let mut is_pid = true;
    let mut x = lo * 3;
    while x <= hi * 3 {
        if let Some(v) = phi_model(R, x, lo, hi) {
            if v != x {
                is_pid = false;
            }
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
        HOMOMORPHISM => is_hom,
        PARTIAL_IDENTITY => is_pid,
        _ => !is_hom,
    }
}

#[cfg(omit_a_helper_key)]
mod the_04a_bug {
    use super::*;
    /// 04a's `recovery_class`, keyed on the resolution alone. Reintroduced so the candidate
    /// sweep can be shown catching it.
    pub const fn recovery_class_no_op<const R: u8>() -> u8 {
        match R {
            WRAP => HOMOMORPHISM,
            REFUSE => PARTIAL_IDENTITY,
            _ => RETRACTION,
        }
    }
    pub const fn no_op_class_is_operation_independent<const R: u8, const FRAC: u32>() -> bool {
        // what keying on R alone asserts: the same verdict at both operations.
        recovery_class_no_op::<R>() == recovery_class::<R, MUL, FRAC>()
            && recovery_class_no_op::<R>() == recovery_class::<R, ADD, FRAC>()
    }
}

#[cfg(lie_at_the_leaf)]
mod the_lying_leaf {
    use super::*;
    /// `Saturate` declared a homomorphism, which probe 02 measured it is not. This is the leaf
    /// where trust bottoms out, so it is the one place a wrong declaration cannot be caught by
    /// keying: there is nothing above it to disagree with. It IS catchable against the model.
    pub const fn recovery_class_lying<const R: u8>() -> u8 {
        if R == SATURATE {
            HOMOMORPHISM
        } else {
            recovery_class::<R, ADD, 0>()
        }
    }

    pub const fn lying_class_agrees_with_model<const R: u8>() -> bool {
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
        declared != HOMOMORPHISM || is_hom
    }
}

fn main() {
    // leaf declarations, checked where they are declared
    const _: () = assert!(class_agrees_with_model::<WRAP>());
    const _: () = assert!(class_agrees_with_model::<SATURATE>());
    const _: () = assert!(class_agrees_with_model::<REFUSE>());

    // helper keys, checked where they are declared. each of these is a claim that a parameter
    // absent from a helper's signature does not move its verdict.
    const _: () = assert!(
        !class_is_operation_independent::<WRAP, 2>(),
        "unused: stated the other way round below"
    );
    const _: () = assert!(
        !bound_is_arity_independent::<2, 2, MUL>(),
        "unused: stated the other way round below"
    );

    #[cfg(lie_at_the_leaf)]
    const _LIE: () = assert!(
        the_lying_leaf::lying_class_agrees_with_model::<SATURATE>(),
        "this resolution declares a structural class the model refutes: it is not a \
         homomorphism, and the exhaustive check at the model width says so"
    );

    #[cfg(omit_a_helper_key)]
    const _CAUGHT: () = assert!(
        the_04a_bug::no_op_class_is_operation_independent::<WRAP, 2>(),
        "this helper is keyed on the resolution alone, and its verdict moves with the \
         operation; the operation belongs in its key"
    );

    #[cfg(not(any(consume_wrong, lie_at_the_leaf, omit_a_helper_key)))]
    {
        println!("a Q2.2 numeral: INT = 2, FRAC = 2. accumulator written (int, frac).\n");
        println!(
            "{:<10} {:<5} {:>6} {:>10} {:>16} {:>8} {:>7}  {}",
            "resolution", "op", "arity", "acc", "class at this op", "kleene", "exist", "why"
        );
        macro_rules! row {
            ($r:expr, $o:expr, $n:expr, $ai:expr, $af:expr) => {{
                println!(
                    "{:<10} {:<5} {:>6} {:>10} {:>16} {:>8} {:>7}  {}",
                    res_name($r),
                    op_name($o),
                    $n,
                    format!("({}, {})", $ai, $af),
                    class_name(recovery_class::<{ $r }, { $o }, 2>()),
                    regrouping_agrees::<2, 2, { $r }, { $o }, { $ai }, { $af }, { $n }>(),
                    regrouping_agrees_existentially::<2, 2, { $r }, { $o }, { $ai }, { $af }, { $n }>(),
                    why::<2, 2, { $r }, { $o }, { $ai }, { $af }, { $n }>()
                );
            }};
        }
        row!(WRAP, ADD, 5, 2, 2);
        row!(WRAP, MUL, 5, 2, 2);
        row!(WRAP, MUL, 5, 8, 8);
        row!(REFUSE, ADD, 5, 2, 2);
        row!(REFUSE, ADD, 5, 4, 2);
        row!(REFUSE, MUL, 5, 8, 8);
        row!(SATURATE, ADD, 5, 2, 2);
        row!(SATURATE, ADD, 5, 4, 2);
        row!(SATURATE, MUL, 5, 8, 8);

        println!(
            "\nthe two rows 04a got wrong now read correctly: `Wrap Mul` is unclassified at a\n\
             fractional numeral, and `Saturate Add` at the numeral's own width is refused.\n\
             and the two relations separate exactly where file 18 section 2 measured they do:\n\
             `Refuse Add` at the numeral's own accumulator is existentially true and Kleene\n\
             false, which is the whole of `Precise`'s regrouping story and is invisible to any\n\
             design that reports one verdict."
        );
        println!("\na combinator recomputes the fact at its own key:");
        println!(
            "  regrouping_fold::<2,2, REFUSE, MUL, 8,8, 5>() = {}",
            regrouping_fold::<2, 2, REFUSE, MUL, 8, 8, 5>()
        );
        // does not compile: acc (2,2) is below interior_exact(MUL, 5) = (8, 8)
        // println!("{}", regrouping_fold::<2, 2, REFUSE, MUL, 2, 2, 5>());
    }

    #[cfg(consume_wrong)]
    {
        println!("{}", regrouping_fold::<2, 2, REFUSE, MUL, 2, 2, 5>());
    }
}
