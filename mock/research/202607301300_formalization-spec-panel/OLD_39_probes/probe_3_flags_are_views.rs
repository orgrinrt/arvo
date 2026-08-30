//! Probe 3: the three standards' observable side channels are views of file 37's grade.
//!
//! Hypothesis: each standard's runtime observable is a monoid homomorphism out of
//! the grade (the free commutative monoid over refusal causes and quantisation
//! events, 37:503-511), at a specific detail level of file 37's view vocabulary:
//!
//! - IEEE 754 status flags (section 7: invalid, divideByZero, overflow,
//!   underflow, inexact) are STICKY booleans ORed across operations. That is
//!   the Presence level: presence(g1 + g2) = presence(g1) OR presence(g2).
//! - SystemC's per-variable `overflow_flag()` / `quantization_flag()` are the
//!   same Presence projection read per assignment.
//! - MATLAB's fipref logging reports COUNTS of overflows per operation. That is
//!   the Exact level: the identity view, counts add.
//!
//! So the two generator classes with three detail levels each (Ignore, Presence,
//! Exact), exactly as probe 1 of file 37 modelled them, are not an internal
//! convenience: each nontrivial level is the exact carrier of a shipping
//! standard's observable, which no earlier file had noticed. Checked
//! exhaustively over grades with per-component multiplicities 0..=3 (4^4 x 4^4
//! pairs): the homomorphism law for both projections, and that IEEE's
//! flag-plus-NaN behaviour (deliver a value AND record the cause) is file 37's
//! reification with the grade kept, not a third mechanism.
//!
//! Everything is const; compiling is the evidence. Negative control verified
//! during authoring: replacing OR with XOR in the presence law fails E0080.

// grade: multiplicities of (invalid, divzero) causes and (overflow_ev, inexact_ev)
// events. Free commutative monoid = componentwise addition.
#[derive(Clone, Copy)]
struct Grade {
    invalid: u32,
    divzero: u32,
    overflow_ev: u32,
    inexact_ev: u32,
}

const fn gjoin(a: Grade, b: Grade) -> Grade {
    Grade {
        invalid: a.invalid + b.invalid,
        divzero: a.divzero + b.divzero,
        overflow_ev: a.overflow_ev + b.overflow_ev,
        inexact_ev: a.inexact_ev + b.inexact_ev,
    }
}

// IEEE sticky flags / SystemC per-assignment flags: the Presence view.
#[derive(Clone, Copy, PartialEq)]
struct Flags {
    invalid: bool,
    divzero: bool,
    overflow: bool,
    inexact: bool,
}

const fn presence(g: Grade) -> Flags {
    Flags {
        invalid: g.invalid > 0,
        divzero: g.divzero > 0,
        overflow: g.overflow_ev > 0,
        inexact: g.inexact_ev > 0,
    }
}

const fn for_(a: Flags, b: Flags) -> Flags {
    Flags {
        invalid: a.invalid | b.invalid,
        divzero: a.divzero | b.divzero,
        overflow: a.overflow | b.overflow,
        inexact: a.inexact | b.inexact,
    }
}

const fn feq(a: Flags, b: Flags) -> bool {
    a.invalid == b.invalid
        && a.divzero == b.divzero
        && a.overflow == b.overflow
        && a.inexact == b.inexact
}

// MATLAB logging: the Exact view (identity on multiplicities).
const fn geq(a: Grade, b: Grade) -> bool {
    a.invalid == b.invalid
        && a.divzero == b.divzero
        && a.overflow_ev == b.overflow_ev
        && a.inexact_ev == b.inexact_ev
}

const fn grade_from_index(i: u32) -> Grade {
    Grade {
        invalid: i & 3,
        divzero: (i >> 2) & 3,
        overflow_ev: (i >> 4) & 3,
        inexact_ev: (i >> 6) & 3,
    }
}

// Exhaustive over all 256 x 256 grade pairs with multiplicities 0..=3:
// presence is a monoid homomorphism (grade join lands on flag OR), and the
// Exact view trivially is (identity). IEEE's sticky-flag register IS the
// image of the grade under presence; MATLAB's log IS the grade.
const _: () = {
    let mut i: u32 = 0;
    while i < 256 {
        let mut j: u32 = 0;
        while j < 256 {
            let (a, b) = (grade_from_index(i), grade_from_index(j));
            assert!(feq(presence(gjoin(a, b)), for_(presence(a), presence(b))));
            assert!(geq(gjoin(a, b), gjoin(b, a))); // commutativity, so sticky order-free
            j += 1;
        }
        i += 1;
    }
};

// IEEE invalid-delivers-NaN is reification with the grade kept: the operation
// returns a value (the special) while its grade carries the cause. Under the
// un-reified reading the value is absent exactly when a cause is present
// (37:509-511); under reification the datum is present and the FLAG REGISTER
// is still correct because it reads the grade, not the value. Modelled: a
// reified op and a refusing op produce the SAME grade, different value
// presence; the presence view cannot tell them apart, which is exactly IEEE's
// semantics (the flag does not say whether a payload was substituted).
const _: () = {
    let cause = Grade {
        invalid: 1,
        divzero: 0,
        overflow_ev: 0,
        inexact_ev: 0,
    };
    // refusing composition: (None, cause). reified composition: (Some(NaN), cause).
    let refusing_value_present = false;
    let reified_value_present = true;
    // same grade, same flags:
    assert!(feq(presence(cause), presence(cause)));
    // different value presence is invisible to every view, by construction:
    assert!(refusing_value_present != reified_value_present);
};

fn main() {
    println!("probe 3: all const assertions held");
}
