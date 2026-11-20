//! Exhaustive check, on the landed `Num` and nothing else, of the region
//! sentences the shape writes for its law suite. For every configuration it
//! counts failing triples (or pairs, or ordered pairs with a third operand):
//! additive associativity, multiplicative associativity, and monotonicity of
//! addition (a <= b implies a + c <= b + c). A count of zero means the law
//! holds exhaustively at that configuration; nonzero means it fails, with the
//! first witness printed. Values are built through `adapted` on the slot grid
//! under `Quiet`, so no debug abort interferes.

use core::cmp::Ordering;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, UFixed};
use arvo_format::rounding::{HalfEven, TowardZero};
use arvo_format::{Adapt, Exact, Signature, Slot};
use arvo_numeral::{Num, Quiet, Sole};

fn members<S: Sole>(w: u32, signed: bool) -> Vec<Num<S, Quiet>> {
    let (lo, hi): (i128, i128) = if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    };
    (lo ..= hi)
        .map(|i| Num::<S, Quiet>::adapted(Exact::on_grid(Slot::at(i))))
        .collect()
}

fn idx<S: Sole>(n: Num<S, Quiet>) -> i128 {
    n.slot().index()
}

struct Row {
    add:  (u64, Option<(i128, i128, i128)>),
    mul:  (u64, Option<(i128, i128, i128)>),
    mono: (u64, Option<(i128, i128, i128)>),
}

fn run<S: Sole>(w: u32, signed: bool) -> Row {
    let xs = members::<S>(w, signed);
    let mut row = Row {
        add:  (0, None),
        mul:  (0, None),
        mono: (0, None),
    };
    for &a in &xs {
        for &b in &xs {
            for &c in &xs {
                if a.add(b).add(c) != a.add(b.add(c)) {
                    row.add.0 += 1;
                    row.add.1.get_or_insert((idx(a), idx(b), idx(c)));
                }
                if a.mul(b).mul(c) != a.mul(b.mul(c)) {
                    row.mul.0 += 1;
                    row.mul.1.get_or_insert((idx(a), idx(b), idx(c)));
                }
                if !matches!(a.cmp(b), Ordering::Greater)
                    && matches!(a.add(c).cmp(b.add(c)), Ordering::Greater)
                {
                    row.mono.0 += 1;
                    row.mono.1.get_or_insert((idx(a), idx(b), idx(c)));
                }
            }
        }
    }
    row
}

fn show(name: &str, w: u32, f: u32, signed: bool, policy: &str, rounding: &str, r: Row) {
    let fmt = |(n, wit): (u64, Option<(i128, i128, i128)>)| {
        match wit {
            None => "holds".to_string(),
            Some(t) => format!("FAILS {n} first {t:?}"),
        }
    };
    println!(
        "{name:<6} W={w} F={f} {:<8} {policy:<8} {rounding:<10} add_assoc={} mul_assoc={} add_monotone={}",
        if signed { "signed" } else { "unsigned" },
        fmt(r.add),
        fmt(r.mul),
        fmt(r.mono),
    );
}

macro_rules! cell {
    ($w:literal, $f:literal) => {{
        show(
            "cell",
            $w,
            $f,
            false,
            "wrap",
            "TowardZero",
            run::<Signature<UFixed<$w, { -$f }>, Adapt<TowardZero, Wrap>>>($w, false),
        );
        show(
            "cell",
            $w,
            $f,
            true,
            "wrap",
            "TowardZero",
            run::<Signature<Fixed<$w, { -$f }>, Adapt<TowardZero, Wrap>>>($w, true),
        );
        show(
            "cell",
            $w,
            $f,
            false,
            "saturate",
            "TowardZero",
            run::<Signature<UFixed<$w, { -$f }>, Adapt<TowardZero, Saturate>>>($w, false),
        );
        show(
            "cell",
            $w,
            $f,
            true,
            "saturate",
            "TowardZero",
            run::<Signature<Fixed<$w, { -$f }>, Adapt<TowardZero, Saturate>>>($w, true),
        );
        show(
            "cell",
            $w,
            $f,
            false,
            "wrap",
            "HalfEven",
            run::<Signature<UFixed<$w, { -$f }>, Adapt<HalfEven, Wrap>>>($w, false),
        );
        show(
            "cell",
            $w,
            $f,
            true,
            "wrap",
            "HalfEven",
            run::<Signature<Fixed<$w, { -$f }>, Adapt<HalfEven, Wrap>>>($w, true),
        );
        show(
            "cell",
            $w,
            $f,
            false,
            "saturate",
            "HalfEven",
            run::<Signature<UFixed<$w, { -$f }>, Adapt<HalfEven, Saturate>>>($w, false),
        );
        show(
            "cell",
            $w,
            $f,
            true,
            "saturate",
            "HalfEven",
            run::<Signature<Fixed<$w, { -$f }>, Adapt<HalfEven, Saturate>>>($w, true),
        );
    }};
}

fn main() {
    cell!(1, 0);
    cell!(1, 1);
    cell!(2, 0);
    cell!(2, 1);
    cell!(2, 2);
    cell!(3, 0);
    cell!(3, 1);
    cell!(3, 2);
    cell!(3, 3);
    cell!(4, 0);
    cell!(4, 1);
    cell!(4, 2);
    cell!(4, 3);
    cell!(4, 4);
    cell!(5, 0);
    cell!(5, 1);
    cell!(5, 3);
    cell!(5, 5);
    cell!(6, 0);
    cell!(6, 1);
    cell!(6, 3);
    cell!(6, 6);
    cell!(7, 0);
    cell!(7, 1);
    cell!(7, 4);
    cell!(7, 7);
    cell!(8, 0);
    cell!(8, 1);
    cell!(8, 4);
    cell!(8, 8);
}
