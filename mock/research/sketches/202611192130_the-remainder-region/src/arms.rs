//! The alternatives to the definition, run through arvo's own shipped
//! operations, against the definition in `reference`.
//!
//! Option b: `r = x - y q` with `q = div::<S>(x, y)`, the quotient on `S`'s grid,
//! taken exactly. Option c1: `sub::<S>(x, mul::<S>(div::<S>(x, y), y))`, the
//! consumer composing the shipped quotient, product and difference. Option c2:
//! the same with the count made whole first, by converting the quotient onto an
//! integer signature by the same mode and back onto `S`. Option c3, at a zero
//! phase only: the slots read as members of a wider integer format, the count,
//! product and difference taken there under wrapping, and the result adapted
//! onto `S`, which is the consumer reimplementing the remainder in integers.
//!
//! The definition's answer is the exact position from `reference`, handed to
//! arvo's own `adapt` under `S`, so the remainder's adaptation is the format's.

use core::marker::PhantomData;

use arvo_format::ambient::{Ambient, BinaryRationals, DecimalRationals};
use arvo_format::overflow::{Clamp, Overflow, Saturate, Wrap};
use arvo_format::quantum::{Constant, Quantum};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, Rounding, Stochastic, TowardZero};
use arvo_format::slots::{Signed, Slot, Slots, Unsigned};
use arvo_format::width::Width;
use arvo_format::{
    Adapt,
    Dither,
    Divisor,
    Exact,
    Format,
    Fraction,
    Member,
    Phase,
    Remainder,
    Signature,
    adapt,
    convert,
    div,
    mul,
    sub,
};

use crate::rat::Q;
use crate::reference::{Fmt, M, dithers, remainder};

pub struct Fm<A, Qn, S, const PN: i64, const PD: i64>(PhantomData<(A, Qn, S)>);

impl<A: Ambient, Qn: Quantum, S: Slots, const PN: i64, const PD: i64> Format
    for Fm<A, Qn, S, PN, PD>
{
    type Ambient = A;
    type Quantum = Qn;
    type Slots = S;

    const PHASE: Phase = Phase::of(PN, PD);
}

pub struct Rng<const LO: i128, const HI: i128, const W: u32>;

impl<const LO: i128, const HI: i128, const W: u32> Slots for Rng<LO, HI, W> {
    const MAX: Slot = Slot::at(HI);
    const MIN: Slot = Slot::at(LO);
    const WIDTH: Width = Width::bits(W);
}

/// How a format's divisors are built through the shipped doors.
pub trait Door {
    type F: Format;
    const FMT: Fmt;
    /// Whether the slots are the values in quanta, so option c3 applies.
    const ZERO_PHASE: bool;
    fn divisor(b: i128) -> Divisor<Self::F>;
}

fn member_divisor<F: Format, G: Format>(b: i128) -> Divisor<F> {
    let m = Member::<G>::adapted::<Signature<G, Adapt<Floor, Wrap>>>(
        Exact::on_grid(Slot::at(b)),
        Dither::UNUSED,
    );
    assert_eq!(m.slot().index(), b, "the member door moved the divisor");
    Divisor::<F>::member::<G>(m)
}

/// A whole-phase format, divided through the member door by the zero-free
/// range on each side of its zero slot.
macro_rules! whole_door {
    ($name:ident, $a:ty, $e:literal, $slots:ty, $pn:literal, $min:literal, $max:literal, $w:literal,
     pos $plo:literal ..= $phi:literal, neg $nlo:literal ..= $nhi:literal, $zero:literal) => {
        pub struct $name;
        impl Door for $name {
            type F = Fm<$a, Constant<$e>, $slots, $pn, 1>;
            const FMT: Fmt = Fmt {
                radix: <$a as RadixOf>::R,
                exp:   $e,
                ph_n:  $pn,
                ph_d:  1,
                min:   $min,
                max:   $max,
            };
            const ZERO_PHASE: bool = $zero;
            fn divisor(b: i128) -> Divisor<Self::F> {
                if b >= $plo {
                    member_divisor::<Self::F, Fm<$a, Constant<$e>, Rng<$plo, $phi, $w>, $pn, 1>>(b)
                } else {
                    member_divisor::<Self::F, Fm<$a, Constant<$e>, Rng<$nlo, $nhi, $w>, $pn, 1>>(b)
                }
            }
        }
    };
}

/// A fractional-phase format, where `Divisor::of` is admitted.
macro_rules! of_door {
    ($name:ident, $a:ty, $e:literal, $slots:ty, $pn:literal, $pd:literal, $min:literal, $max:literal) => {
        pub struct $name;
        impl Door for $name {
            type F = Fm<$a, Constant<$e>, $slots, $pn, $pd>;

            const FMT: Fmt = Fmt {
                radix: <$a as RadixOf>::R,
                exp:   $e,
                ph_n:  $pn,
                ph_d:  $pd,
                min:   $min,
                max:   $max,
            };
            const ZERO_PHASE: bool = false;

            fn divisor(b: i128) -> Divisor<Self::F> {
                Divisor::<Self::F>::of(Slot::at(b))
            }
        }
    };
}

pub trait RadixOf {
    const R: i128;
}
impl RadixOf for BinaryRationals {
    const R: i128 = 2;
}
impl RadixOf for DecimalRationals {
    const R: i128 = 10;
}

type Bin = BinaryRationals;
type Dec = DecimalRationals;

whole_door!(SignedInt, Bin, 0, Signed<6>, 0, -32, 31, 6, pos 1 ..= 31, neg -32 ..= -1, true);
whole_door!(UnsignedInt, Bin, 0, Unsigned<6>, 0, 0, 63, 6, pos 1 ..= 63, neg 1 ..= 63, true);
whole_door!(SignedFix, Bin, -3, Signed<6>, 0, -32, 31, 6, pos 1 ..= 31, neg -32 ..= -1, true);
whole_door!(UnsignedFix, Bin, -3, Unsigned<6>, 0, 0, 63, 6, pos 1 ..= 63, neg 1 ..= 63, true);
whole_door!(SignedScaled, Bin, 2, Signed<6>, 0, -32, 31, 6, pos 1 ..= 31, neg -32 ..= -1, true);
whole_door!(SignedDecimal, Dec, -1, Signed<6>, 0, -32, 31, 6, pos 1 ..= 31, neg -32 ..= -1, true);
// A whole phase of one over the unsigned range: values 1 to 64, the zero at slot -1.
whole_door!(FromOne, Bin, 0, Unsigned<6>, 1, 0, 63, 6, pos 0 ..= 63, neg 0 ..= 63, false);
// A whole phase of minus three at tenths... in halves: the zero at slot 3.
whole_door!(MinusThree, Bin, -1, Signed<6>, -3, -32, 31, 6, pos 4 ..= 31, neg -32 ..= 2, false);
of_door!(Halves, Bin, -1, Signed<6>, 1, 2, -32, 31);
of_door!(Thirds, Bin, -2, Unsigned<6>, 1, 3, 0, 63);

/// The wide integer format option c2 lands the count in and c3 computes in.
type Wide = Fm<Bin, Constant<0>, Signed<10>, 0, 1>;
whole_door!(WideInt, Bin, 0, Signed<10>, 0, -512, 511, 10, pos 1 ..= 511, neg -512 ..= -1, true);

fn exact_of(s: Q) -> Exact {
    let f = s.floor();
    let rest = s.n - f * s.d;
    Exact::at(Slot::at(f), Remainder::of(rest as u128, s.d as u128))
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Tally {
    pub pairs:       u64,
    /// The definition's exact position is on the grid.
    pub def_on_grid: u64,
    /// Option b's exact `x - y q` equals the definition's.
    pub b_same:      u64,
    pub b_on_grid:   u64,
    /// `|x - y q| < |y|`.
    pub b_bounded:   u64,
    /// Adapted under `S`, option b lands where the definition does.
    pub b_adapted:   u64,
    pub c1_same:     u64,
    pub c2_same:     u64,
    pub c3_same:     u64,
    pub c3_run:      u64,
}

fn sweep<D: Door, R: Rounding, O: Overflow>(m: M) -> Tally {
    type S<D, R, O> = Signature<<D as Door>::F, Adapt<R, O>>;
    let f = D::FMT;
    let mut t = Tally::default();
    for a in f.min ..= f.max {
        for b in f.min ..= f.max {
            if f.value(b).is_zero() {
                continue;
            }
            let divisor = D::divisor(b);
            for k in dithers(m) {
                let dither = Dither::at(Fraction::of(k as i64, 4));
                t.pairs += 1;
                let (_, s) = remainder(f, f, a, b, m, k);
                t.def_on_grid += u64::from(s.is_int());
                let want = adapt::<S<D, R, O>>(exact_of(s), dither);

                // Option b, exactly.
                let qb = div::<S<D, R, O>>(Slot::at(a), divisor, dither);
                let rb = f.value(a).sub(f.value(qb.index()).mul(f.value(b)));
                let sb = rb.div(f.q()).sub(f.phase());
                t.b_same += u64::from(sb == s);
                t.b_on_grid += u64::from(sb.is_int());
                t.b_bounded += u64::from(rb.abs().lt(f.value(b).abs()));
                t.b_adapted += u64::from(adapt::<S<D, R, O>>(exact_of(sb), dither) == want);

                // Option c1, the shipped operations composed under S.
                let c1 = sub::<S<D, R, O>>(
                    Slot::at(a),
                    mul::<S<D, R, O>>(qb, Slot::at(b), dither),
                    dither,
                );
                t.c1_same += u64::from(c1 == want);

                // Option c2, the count made whole by the same mode first.
                let n = convert::<D::F, Signature<Wide, Adapt<R, O>>>(qb, dither);
                let n_back = convert::<Wide, S<D, R, O>>(n, dither);
                let c2 = sub::<S<D, R, O>>(
                    Slot::at(a),
                    mul::<S<D, R, O>>(n_back, Slot::at(b), dither),
                    dither,
                );
                t.c2_same += u64::from(c2 == want);

                // Option c3, the remainder reimplemented in a wider integer format.
                if D::ZERO_PHASE {
                    type I<R> = Signature<Wide, Adapt<R, Wrap>>;
                    let count = div::<I<R>>(Slot::at(a), WideInt::divisor(b), dither);
                    let prod = mul::<I<R>>(count, Slot::at(b), dither);
                    let r = sub::<I<R>>(Slot::at(a), prod, dither);
                    let c3 = adapt::<S<D, R, O>>(Exact::on_grid(r), dither);
                    t.c3_run += 1;
                    t.c3_same += u64::from(c3 == want);
                }
            }
        }
    }
    t
}

pub struct Row {
    pub format: &'static str,
    pub mode:   M,
    pub policy: &'static str,
    pub tally:  Tally,
}

macro_rules! every_signature {
    ($rows:ident, $door:ty) => {
        every_signature!(@m $rows, $door, TowardZero, M::TowardZero);
        every_signature!(@m $rows, $door, Floor, M::Floor);
        every_signature!(@m $rows, $door, Ceil, M::Ceil);
        every_signature!(@m $rows, $door, HalfUp, M::HalfUp);
        every_signature!(@m $rows, $door, HalfEven, M::HalfEven);
        every_signature!(@m $rows, $door, Stochastic, M::Stochastic);
    };
    (@m $rows:ident, $door:ty, $r:ty, $m:expr) => {
        $rows.push(Row { format: stringify!($door), mode: $m, policy: "wrap", tally: sweep::<$door, $r, Wrap>($m) });
        $rows.push(Row { format: stringify!($door), mode: $m, policy: "saturate", tally: sweep::<$door, $r, Saturate>($m) });
        $rows.push(Row { format: stringify!($door), mode: $m, policy: "clamp", tally: sweep::<$door, $r, Clamp>($m) });
    };
}

pub fn run() -> Vec<Row> {
    let mut rows = Vec::new();
    every_signature!(rows, SignedInt);
    every_signature!(rows, UnsignedInt);
    every_signature!(rows, SignedFix);
    every_signature!(rows, UnsignedFix);
    every_signature!(rows, SignedScaled);
    every_signature!(rows, SignedDecimal);
    every_signature!(rows, FromOne);
    every_signature!(rows, MinusThree);
    every_signature!(rows, Halves);
    every_signature!(rows, Thirds);
    rows
}

/// The named pairs the write-up quotes, read off the shipped operations.
pub fn named() -> Vec<String> {
    let mut out = Vec::new();
    // Option b at `MIN / -1` on the signed integers under saturation.
    type Sat = Signature<<SignedInt as Door>::F, Adapt<TowardZero, Saturate>>;
    let q = div::<Sat>(Slot::at(-32), SignedInt::divisor(-1), Dither::UNUSED);
    let f = SignedInt::FMT;
    let rb = f.value(-32).sub(f.value(q.index()).mul(f.value(-1)));
    out.push(format!(
        "b, Signed<6> e=0 toward_zero saturate, -32 / -1: q = {}, r = x - y q = {}/{}",
        q.index(),
        rb.n,
        rb.d
    ));
    // Option b over the whole phase of one, 3 by 5 in values: slots 2 and 4.
    type One = Signature<<FromOne as Door>::F, Adapt<Floor, Saturate>>;
    let f = FromOne::FMT;
    let q = div::<One>(Slot::at(2), FromOne::divisor(4), Dither::UNUSED);
    let rb = f.value(2).sub(f.value(q.index()).mul(f.value(4)));
    let (_, s) = remainder(f, f, 2, 4, M::Floor, 0);
    out.push(format!(
        "b, FromOne (values 1..64) floor saturate, value 3 by value 5: q slot {} (value {}), r = {}/{}; definition's slot {}/{} (value 3)",
        q.index(), f.value(q.index()).n, rb.n, rb.d, s.n, s.d
    ));
    // Option c1 under saturation at e = 0: the first pair where the shipped
    // quotient, product and difference composed under S miss the definition.
    type Ce = Signature<<SignedInt as Door>::F, Adapt<Ceil, Saturate>>;
    let f = SignedInt::FMT;
    'c1: for a in f.min ..= f.max {
        for b in f.min ..= f.max {
            if b == 0 {
                continue;
            }
            let q = div::<Ce>(Slot::at(a), SignedInt::divisor(b), Dither::UNUSED);
            let p = mul::<Ce>(q, Slot::at(b), Dither::UNUSED);
            let c1 = sub::<Ce>(Slot::at(a), p, Dither::UNUSED);
            let (n, s) = remainder(f, f, a, b, M::Ceil, 0);
            if c1.index() != s.n {
                out.push(format!(
                    "c1, Signed<6> e=0 ceil saturate, {a} / {b}: q = {}, mul = {} (exact {}), sub = {}; definition count {n}, remainder {}",
                    q.index(), p.index(), n * b, c1.index(), s.n
                ));
                break 'c1;
            }
        }
    }
    // Option c2 at e = -3 under half to even: the quotient rounds onto eighths
    // first, then onto the integers.
    type He = Signature<<SignedFix as Door>::F, Adapt<HalfEven, Wrap>>;
    let f = SignedFix::FMT;
    let mut shown = 0;
    'outer: for a in f.min ..= f.max {
        for b in f.min ..= f.max {
            if b == 0 {
                continue;
            }
            let q = div::<He>(Slot::at(a), SignedFix::divisor(b), Dither::UNUSED);
            let n = convert::<<SignedFix as Door>::F, Signature<Wide, Adapt<HalfEven, Wrap>>>(
                q,
                Dither::UNUSED,
            );
            let (count, _) = remainder(f, f, a, b, M::HalfEven, 0);
            if n.index() != count {
                out.push(format!(
                    "c2, Fixed e=-3 half_even, slot {a} / slot {b} = {}/{}: quotient on eighths slot {} ({}), count by conversion {}, true count {}",
                    a, b, q.index(), q.index() as f64 / 8.0, n.index(), count
                ));
                shown += 1;
                if shown == 2 {
                    break 'outer;
                }
            }
        }
    }
    out
}
