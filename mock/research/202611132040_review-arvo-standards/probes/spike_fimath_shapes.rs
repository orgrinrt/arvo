// Spike: the fimath sweep at more shapes than Fi<8, 0>.
mod ties_away;
use arvo_format::adapt::DeclaredSignature;
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::slots::Slot;
use arvo_format::standards::matlab::overflow_action::{Saturate, Wrap};
use arvo_format::standards::matlab::rounding_method::{Ceiling, Convergent, Floor, Nearest, Zero};
use arvo_format::standards::matlab::{FiMath, UfiMath};

fn floor(q: i128) -> i128 { q.div_euclid(4) }
fn ceiling(q: i128) -> i128 { -(-q).div_euclid(4) }
fn zero(q: i128) -> i128 { q / 4 }
fn nearest(q: i128) -> i128 { (q + 2).div_euclid(4) }
fn convergent(q: i128) -> i128 {
    let down = q.div_euclid(4);
    match (q.rem_euclid(4) * 2).cmp(&4) {
        core::cmp::Ordering::Less => down,
        core::cmp::Ordering::Greater => down + 1,
        core::cmp::Ordering::Equal => down + down.rem_euclid(2),
    }
}
fn position(q: i128) -> Exact { Exact::between(Slot::at(q.div_euclid(4)), Fraction::of(q.rem_euclid(4) as i64, 4)) }

fn qs(lo: i128, hi: i128) -> Vec<i128> {
    let mut v = vec![];
    for c in [lo, 0, hi] { for d in -40..=40 { v.push(4 * c + d); } }
    v
}
fn sat(lo: i128, hi: i128) -> impl Fn(i128) -> i128 { move |r| r.clamp(lo, hi) }
fn wrp(lo: i128, w: u32) -> impl Fn(i128) -> i128 { move |r| (r - lo).rem_euclid(1i128 << w) + lo }

fn misses<S: DeclaredSignature>(lo: i128, hi: i128, rule: fn(i128) -> i128, act: &dyn Fn(i128) -> i128) -> usize {
    qs(lo, hi).into_iter().filter(|&q| adapt::<S>(position(q), Dither::UNUSED).index() != act(rule(q))).count()
}

macro_rules! shape {
    ($name:ident, $signed:ident, $w:literal, $f:literal) => {
        #[test]
        fn $name() {
            let (lo, hi): (i128, i128) = if stringify!($signed) == "s" { (-(1i128 << ($w - 1)), (1i128 << ($w - 1)) - 1) } else { (0, (1i128 << $w) - 1) };
            let s = sat(lo, hi); let w = wrp(lo, $w);
            macro_rules! m { ($R:ty, $O:ty) => { shape!(@ty $signed, $w, $f, $R, $O) } }
            let mut bad = vec![];
            bad.push(("floor sat", misses::<m!(Floor, Saturate)>(lo, hi, floor, &s)));
            bad.push(("ceil sat", misses::<m!(Ceiling, Saturate)>(lo, hi, ceiling, &s)));
            bad.push(("zero sat", misses::<m!(Zero, Saturate)>(lo, hi, zero, &s)));
            bad.push(("nearest sat", misses::<m!(Nearest, Saturate)>(lo, hi, nearest, &s)));
            bad.push(("conv sat", misses::<m!(Convergent, Saturate)>(lo, hi, convergent, &s)));
            bad.push(("floor wrap", misses::<m!(Floor, Wrap)>(lo, hi, floor, &w)));
            bad.push(("ceil wrap", misses::<m!(Ceiling, Wrap)>(lo, hi, ceiling, &w)));
            bad.push(("zero wrap", misses::<m!(Zero, Wrap)>(lo, hi, zero, &w)));
            bad.push(("nearest wrap", misses::<m!(Nearest, Wrap)>(lo, hi, nearest, &w)));
            bad.push(("conv wrap", misses::<m!(Convergent, Wrap)>(lo, hi, convergent, &w)));
            let bad: Vec<_> = bad.into_iter().filter(|b| b.1 > 0).collect();
            assert!(bad.is_empty(), "{:?}", bad);
        }
    };
    (@ty s, $w:literal, $f:literal, $R:ty, $O:ty) => { FiMath<$w, $f, $R, $O> };
    (@ty u, $w:literal, $f:literal, $R:ty, $O:ty) => { UfiMath<$w, $f, $R, $O> };
}
shape!(s1, s, 1, 0);
shape!(s2, s, 2, 0);
shape!(s7, s, 7, 3);
shape!(s16, s, 16, 15);
shape!(s33, s, 33, -4);
shape!(s63, s, 63, 0);
shape!(s64, s, 64, 10);
shape!(u1, u, 1, 0);
shape!(u8, u, 8, 0);
shape!(u16, u, 16, 14);
shape!(u63, u, 63, 2);
shape!(u64, u, 64, 0);
