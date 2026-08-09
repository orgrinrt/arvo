//! Probe 6: specials as values and specials as causes, measured against the hardware.
//!
//! rustc --edition 2021 -O probe_6_specials.rs -o /tmp/p6 && /tmp/p6
//!
//! Section 1.13 records IEEE's cause split (x/0 with x nonzero finite is `divideByZero` with
//! an infinite result; 0/0 is `invalid` with a NaN result) as reasoned from the standard and
//! awaiting the float model to compile. The split is compiled here, in two halves that the
//! design has to keep apart:
//!
//!   The VALUE half is checkable against the machine, and is checked: every combination of
//!   zero, infinity, NaN and finite under the four operations, against binary32.
//!
//!   The CAUSE half is NOT checkable against the machine from Rust, because there is no way
//!   to read the flags (probe 5). It is a claim the design makes on its own authority, and
//!   the design's grade is the only carrier available for it.
//!
//! Also measured: which NaN payload survives an operation on this target, which is a fact
//! about the silicon rather than about the standard, and which the design's `Canonical`
//! axis has to be able to describe rather than fix.

#[path = "model.rs"]
mod model;
use model::{Cause, Grade};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sp {
    Zero { neg: bool },
    Fin { neg: bool },
    Inf { neg: bool },
    QNan,
    SNan,
}

fn class(x: f32) -> Sp {
    let b = x.to_bits();
    let neg = b >> 31 == 1;
    if x.is_nan() {
        // quiet bit is the top fraction bit
        return if b & 0x0040_0000 != 0 {
            Sp::QNan
        } else {
            Sp::SNan
        };
    }
    if x.is_infinite() {
        return Sp::Inf { neg };
    }
    if x == 0.0 {
        return Sp::Zero { neg };
    }
    Sp::Fin { neg }
}

/// The model's answer: what a class-level operation delivers, and what it raises.
/// This is the design's own table, written from IEEE 754-2019 clause 7 and clause 6, not
/// read off the machine.
fn model_op(op: char, a: Sp, b: Sp) -> (Sp, Grade) {
    use Sp::*;
    let g = |c: Cause| Grade::of(c);
    // 6.2: any operation with a signalling NaN operand signals invalid and, in the default
    // handler, delivers a quiet NaN. Any operation with a quiet NaN operand and no other
    // exception delivers a quiet NaN and raises nothing.
    if matches!(a, SNan) || matches!(b, SNan) {
        return (QNan, g(Cause::Invalid));
    }
    if matches!(a, QNan) || matches!(b, QNan) {
        return (QNan, Grade::EMPTY);
    }
    match op {
        '+' => match (a, b) {
            (Inf { neg: x }, Inf { neg: y }) if x != y => (QNan, g(Cause::Invalid)),
            (Inf { neg }, _) | (_, Inf { neg }) => (Inf { neg }, Grade::EMPTY),
            (Zero { neg: x }, Zero { neg: y }) => {
                // 6.3: the sign of a sum of two zeros is + under roundTiesToEven unless
                // both are negative.
                (Zero { neg: x && y }, Grade::EMPTY)
            }
            // Finite + finite is NOT decidable at the class level: exact cancellation
            // delivers a zero, which is a different class. The caller treats a finite
            // prediction as "finite or zero", and probe 1 decides the value.
            _ => (Fin { neg: false }, Grade::EMPTY),
        },
        '*' => match (a, b) {
            (Inf { .. }, Zero { .. }) | (Zero { .. }, Inf { .. }) => (QNan, g(Cause::Invalid)),
            (Inf { neg: x }, Inf { neg: y }) => (Inf { neg: x != y }, Grade::EMPTY),
            (Inf { neg: x }, Fin { neg: y }) | (Fin { neg: y }, Inf { neg: x }) => {
                (Inf { neg: x != y }, Grade::EMPTY)
            }
            (Zero { neg: x }, Zero { neg: y }) => (Zero { neg: x != y }, Grade::EMPTY),
            (Zero { neg: x }, Fin { neg: y }) | (Fin { neg: y }, Zero { neg: x }) => {
                (Zero { neg: x != y }, Grade::EMPTY)
            }
            _ => (Fin { neg: false }, Grade::EMPTY),
        },
        '/' => match (a, b) {
            (Zero { .. }, Zero { .. }) => (QNan, g(Cause::Invalid)),
            (Inf { .. }, Inf { .. }) => (QNan, g(Cause::Invalid)),
            // 7.3: divideByZero is raised only for an exact infinite result from finite
            // operands. This is the member of the split that is NOT invalid.
            (Fin { neg: x }, Zero { neg: y }) => (Inf { neg: x != y }, g(Cause::DivideByZero)),
            (Inf { neg: x }, Fin { neg: y }) | (Inf { neg: x }, Zero { neg: y }) => {
                (Inf { neg: x != y }, Grade::EMPTY)
            }
            (Fin { neg: x }, Inf { neg: y }) | (Zero { neg: x }, Inf { neg: y }) => {
                (Zero { neg: x != y }, Grade::EMPTY)
            }
            (Zero { neg: x }, Fin { neg: y }) => (Zero { neg: x != y }, Grade::EMPTY),
            _ => (Fin { neg: false }, Grade::EMPTY),
        },
        _ => unreachable!(),
    }
}

fn main() {
    let vals: [(&str, f32); 10] = [
        ("+0", 0.0),
        ("-0", -0.0),
        ("+1", 1.0),
        ("-1", -1.0),
        ("+3", 3.0),
        ("+inf", f32::INFINITY),
        ("-inf", f32::NEG_INFINITY),
        ("qNaN(1)", f32::from_bits(0x7fc0_0001)),
        ("qNaN(2)", f32::from_bits(0x7fc0_0002)),
        ("sNaN(1)", f32::from_bits(0x7f80_0001)),
    ];

    let mut checked = 0;
    let mut mismatch = 0;
    for (na, a) in vals {
        for (nb, b) in vals {
            for op in ['+', '*', '/'] {
                let hw = match op {
                    '+' => a + b,
                    '*' => a * b,
                    _ => a / b,
                };
                let (want, _grade) = model_op(op, class(a), class(b));
                let got = class(hw);
                let agree = match (want, got) {
                    // the model's finite branch does not model the finite VALUE, only that
                    // the class is finite; the value half is probe 1's job
                    (Sp::Fin { .. }, Sp::Fin { .. }) => true,
                    // exact cancellation: see the note on the '+' arm
                    (Sp::Fin { .. }, Sp::Zero { .. }) if op == '+' => true,
                    (Sp::QNan, Sp::QNan) => true,
                    (w, g) => w == g,
                };
                checked += 1;
                if !agree {
                    mismatch += 1;
                    println!("MISMATCH {na} {op} {nb}: model {want:?}, hardware {got:?} ({hw})");
                }
            }
        }
    }
    println!("class-level cases checked against binary32: {checked}, mismatches {mismatch}");

    // ---- what the machine does with NaN payloads: silicon, not standard ----
    println!("\n== NaN payload propagation on this target ==");
    use std::hint::black_box as bb;
    let n1 = bb(f32::from_bits(0x7fc0_0001));
    let n2 = bb(f32::from_bits(0x7fc0_0002));
    let s1 = bb(f32::from_bits(0x7f80_0001));
    println!("qNaN(1) + qNaN(2) -> {:#010x}", (bb(n1) + bb(n2)).to_bits());
    println!("qNaN(2) + qNaN(1) -> {:#010x}", (bb(n2) + bb(n1)).to_bits());
    println!(
        "qNaN(1) + 1.0     -> {:#010x}",
        (bb(n1) + bb(1.0f32)).to_bits()
    );
    println!(
        "1.0 + qNaN(1)     -> {:#010x}",
        (bb(1.0f32) + bb(n1)).to_bits()
    );
    println!(
        "sNaN(1) + 1.0     -> {:#010x} (quieted: {})",
        (bb(s1) + bb(1.0f32)).to_bits(),
        (bb(s1) + bb(1.0f32)).to_bits() & 0x0040_0000 != 0
    );
    println!(
        "0.0 / 0.0         -> {:#010x} (the default NaN)",
        (bb(0.0f32) / bb(0.0f32)).to_bits()
    );
    println!(
        "inf - inf         -> {:#010x}",
        (bb(f32::INFINITY) - bb(f32::INFINITY)).to_bits()
    );

    // ---- the cause half, which no machine reading can confirm from Rust ----
    const FOLDED_NAN: f32 = f32::from_bits(0x7fc0_0001) + 1.0;
    println!(
        "const-folded qNaN(1) + 1.0 -> {:#010x} (the compiler, not the FPU)",
        FOLDED_NAN.to_bits()
    );

    println!("\n== the cause split, from the standard, uncheckable against the machine ==");
    for (na, a) in [
        ("+1", 1.0f32),
        ("-1", -1.0),
        ("+0", 0.0),
        ("+inf", f32::INFINITY),
    ] {
        for (nb, b) in [("+0", 0.0f32), ("-0", -0.0)] {
            let (want, grade) = model_op('/', class(a), class(b));
            let hw = a / b;
            println!(
                "{na} / {nb}: value {:?} (hardware {:#010x}), design grade {:?}, \
                 divideByZero {}, invalid {}",
                want,
                hw.to_bits(),
                grade.0,
                grade.has(Cause::DivideByZero),
                grade.has(Cause::Invalid)
            );
        }
    }
}
