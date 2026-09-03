//! Shared machinery for the four steps.
//!
//! Every adaptation point in every arm goes through `arvo_format::apply::adapt`,
//! which is the shipped realisation of the ratified factoring. Nothing here
//! reimplements a rounding mode or a range policy, so a disagreement between two
//! arms is a disagreement between two schedules of that one map rather than
//! between this file and the crate.
//!
//! Slot coordinates throughout. A numeral of fraction length `F` has quantum
//! `2^-F`, so the value on slot `k` is `k * 2^-F`, and the exact product of slots
//! `A` and `B` sits at slot position `A * B / 2^F`. That position is carried as a
//! `Fraction` rather than as an approximation, which is what makes a tie a tie.

use arvo_format::adapt::DeclaredSignature;
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::slots::{Slot, Slots};

/// The declared slot range of a signature's format.
#[must_use]
pub const fn bounds<S: DeclaredSignature>() -> (i64, i64) {
    (
        <<S::Format as Format>::Slots as Slots>::MIN.index(),
        <<S::Format as Format>::Slots as Slots>::MAX.index(),
    )
}

/// The exact position of `a * b + c` in slot units, adapted once.
///
/// One adaptation point, after the whole exact expression. This is the fused
/// realisation.
#[must_use]
pub fn fused<S: DeclaredSignature>(a: i64, b: i64, c: i64, den: i64) -> i64 {
    adapt::<S>(
        Exact::between(Slot::at(c), Fraction::of(a * b, den)),
        Dither::UNUSED,
    )
    .index()
}

/// The exact product adapted, then the sum with `c` adapted.
///
/// Two adaptation points. The second one's rounding region is dead by
/// construction, because a slot plus a slot is a slot, so the whole of what
/// separates this from `fused` is one rounding and one completion.
#[must_use]
pub fn stepwise<M: DeclaredSignature, A: DeclaredSignature>(
    a: i64,
    b: i64,
    c: i64,
    den: i64,
) -> i64 {
    let product = adapt::<M>(
        Exact::between(Slot::ZERO, Fraction::of(a * b, den)),
        Dither::UNUSED,
    )
    .index();
    adapt::<A>(Exact::on_grid(Slot::at(product + c)), Dither::UNUSED).index()
}

/// What one cell of the sweep returned.
#[derive(Clone, Debug)]
pub struct Cell {
    pub signedness: &'static str,
    pub width:      u32,
    pub fraction:   u32,
    pub mode:       &'static str,
    pub policy:     &'static str,
    /// Triples where the two realisations returned different slots.
    pub differing:  u64,
    /// Triples visited.
    pub total:      u64,
    /// The first triple that differed, with both answers.
    pub witness:    Option<(i64, i64, i64, i64, i64)>,
}

impl Cell {
    #[must_use]
    pub fn agrees(&self) -> bool {
        self.differing == 0
    }

    #[must_use]
    pub fn rate(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        100.0 * (self.differing as f64) / (self.total as f64)
    }

    pub fn print(&self) {
        let witness = match self.witness {
            None => String::from("-"),
            Some((a, b, c, f, s)) => format!("a={a} b={b} c={c} fused={f} stepwise={s}"),
        };
        println!(
            "cell {} W={} F={} mode={} policy={} differing={} of {} rate={:.4} witness: {}",
            self.signedness,
            self.width,
            self.fraction,
            self.mode,
            self.policy,
            self.differing,
            self.total,
            self.rate(),
            witness,
        );
    }
}

/// One cell: every triple in the declared range, fused against the natural
/// stepwise composition at the same signature.
#[must_use]
pub fn natural_cell<S: DeclaredSignature>(
    signedness: &'static str,
    width: u32,
    fraction: u32,
    mode: &'static str,
    policy: &'static str,
) -> Cell {
    let (lo, hi) = bounds::<S>();
    let den = 1i64 << fraction;
    let mut differing = 0u64;
    let mut total = 0u64;
    let mut witness = None;
    for a in lo ..= hi {
        for b in lo ..= hi {
            for c in lo ..= hi {
                let f = fused::<S>(a, b, c, den);
                let s = stepwise::<S, S>(a, b, c, den);
                total += 1;
                if f != s {
                    differing += 1;
                    if witness.is_none() {
                        witness = Some((a, b, c, f, s));
                    }
                }
            }
        }
    }
    Cell {
        signedness,
        width,
        fraction,
        mode,
        policy,
        differing,
        total,
        witness,
    }
}

/// Emits the ten mode-policy cells of one format at one fraction length.
#[macro_export]
macro_rules! cells_of {
    ($rows:ident, $fmt:ty, $sign:literal, $w:literal, $f:literal) => {{
        $crate::cells_of!(@mode $rows, $fmt, $sign, $w, $f, Floor, "floor");
        $crate::cells_of!(@mode $rows, $fmt, $sign, $w, $f, Ceil, "ceil");
        $crate::cells_of!(@mode $rows, $fmt, $sign, $w, $f, TowardZero, "toward_zero");
        $crate::cells_of!(@mode $rows, $fmt, $sign, $w, $f, HalfUp, "half_up");
        $crate::cells_of!(@mode $rows, $fmt, $sign, $w, $f, HalfEven, "half_even");
    }};
    (@mode $rows:ident, $fmt:ty, $sign:literal, $w:literal, $f:literal, $m:ident, $mn:literal) => {{
        $rows.push($crate::natural_cell::<Signature<$fmt, Adapt<$m, Wrap>>>(
            $sign, $w, $f, $mn, "wrap",
        ));
        $rows.push($crate::natural_cell::<Signature<$fmt, Adapt<$m, Saturate>>>(
            $sign, $w, $f, $mn, "saturate",
        ));
    }};
}

/// Emits both signednesses of one width and fraction length.
#[macro_export]
macro_rules! both_signs {
    ($rows:ident, $w:literal, $f:literal) => {{
        $crate::cells_of!($rows, Ufi<$w, $f>, "unsigned", $w, $f);
        $crate::cells_of!($rows, Fi<$w, $f>, "signed", $w, $f);
    }};
}

/// The width and fraction grid the sweep runs over.
#[macro_export]
macro_rules! the_grid {
    ($rows:ident) => {{
        $crate::both_signs!($rows, 3, 0);
        $crate::both_signs!($rows, 3, 1);
        $crate::both_signs!($rows, 3, 2);
        $crate::both_signs!($rows, 4, 0);
        $crate::both_signs!($rows, 4, 1);
        $crate::both_signs!($rows, 4, 2);
        $crate::both_signs!($rows, 4, 3);
        $crate::both_signs!($rows, 5, 0);
        $crate::both_signs!($rows, 5, 1);
        $crate::both_signs!($rows, 5, 2);
        $crate::both_signs!($rows, 5, 3);
        $crate::both_signs!($rows, 5, 4);
        $crate::both_signs!($rows, 6, 0);
        $crate::both_signs!($rows, 6, 1);
        $crate::both_signs!($rows, 6, 2);
        $crate::both_signs!($rows, 6, 3);
        $crate::both_signs!($rows, 6, 4);
        $crate::both_signs!($rows, 6, 5);
        $crate::both_signs!($rows, 7, 0);
        $crate::both_signs!($rows, 7, 1);
        $crate::both_signs!($rows, 7, 2);
        $crate::both_signs!($rows, 7, 3);
        $crate::both_signs!($rows, 7, 4);
        $crate::both_signs!($rows, 7, 5);
        $crate::both_signs!($rows, 7, 6);
        $crate::both_signs!($rows, 8, 0);
        $crate::both_signs!($rows, 8, 1);
        $crate::both_signs!($rows, 8, 2);
        $crate::both_signs!($rows, 8, 3);
        $crate::both_signs!($rows, 8, 4);
        $crate::both_signs!($rows, 8, 5);
        $crate::both_signs!($rows, 8, 6);
        $crate::both_signs!($rows, 8, 7);
    }};
}
