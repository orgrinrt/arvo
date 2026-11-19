//! The quotient, the product and the conversion of a 16-bit fixed point, three
//! ways each, for the `numeral-ratio` bench.
//!
//! The signature is `Fixed<16, -8>`, eighths of a slot by 256, rounded half to
//! even and saturated. The input is a run of 4-byte records, two little-endian
//! `i16` slot indices `a` and `b`. Each result is folded into one 64-bit word in
//! order, which is the output, so every result is live and checked. Every arm computes the same output from the same input,
//! which the harness validates before it times anything.
//!
//! `hand` is the operation as a consumer writes it on the machine's integers:
//! the scaled numerator, one division, the residue rounded half to even, the
//! result clamped. `room` is the value's own operation, which forms the ratio in
//! the host's 128-bit integer. `wide` is the format's own exact step, formed four
//! 64-bit limbs wide, on the same slots, which is what the value called before.
//!
//! The quotient's divisor is read at run time as a value is: landed in a format
//! on the same grid whose range is 1 to 32767, by `Member::adapted`, which is a
//! clamp of an on-grid slot, and the hand arm clamps the same way.

use core::marker::PhantomData;

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::Saturate;
use arvo_format::points::Fixed;
use arvo_format::quantum::Constant;
use arvo_format::rounding::HalfEven;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;
use arvo_format::{Adapt, Dither, Divisor, Exact, Member, Signature};
use arvo_numeral::{Num, Quiet};

/// The value's signature.
pub type S = Signature<Fixed<16, -8>, Adapt<HalfEven, Saturate>>;

/// The conversion's target, sixteenths at 16 bits.
pub type T = Signature<Fixed<16, -4>, Adapt<HalfEven, Saturate>>;

/// The slots 1 to 32767.
pub struct Positive;

impl Slots for Positive {
    const MAX: Slot = Slot::at(32767);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(16);
}

/// The divisor's format: the value's grid over the positive slots.
pub struct Beside<R>(PhantomData<R>);

impl<R: Slots> Format for Beside<R> {
    type Ambient = BinaryRationals;
    type Quantum = Constant<-8>;
    type Slots = R;

    const PHASE: Phase = Phase::ZERO;
}

type G = Beside<Positive>;

/// The record count a buffer of `n` bytes holds.
#[inline]
pub fn records(n: usize) -> usize {
    n / 4
}

/// The output size: one 64-bit word.
#[inline]
pub fn output_size(_n: usize) -> usize {
    8
}

#[inline(always)]
fn record(input: &[u8], i: usize) -> (i16, i16) {
    let at = 4 * i;
    (
        i16::from_le_bytes([input[at], input[at + 1]]),
        i16::from_le_bytes([input[at + 2], input[at + 3]]),
    )
}

/// The fold of the results so far with one more.
#[inline(always)]
fn put(acc: &mut u64, x: i16) {
    *acc = acc.rotate_left(5) ^ (x as u16 as u64);
}

/// The floor of `num / den` rounded half to even, `den` above zero.
#[inline(always)]
fn half_even(num: i32, den: i32) -> i32 {
    let q = num.div_euclid(den);
    let r = num.rem_euclid(den);
    let twice = 2 * r;
    if twice > den || (twice == den && q & 1 != 0) { q + 1 } else { q }
}

#[inline(always)]
fn clamp16(x: i32) -> i16 {
    x.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

#[inline(always)]
fn divisor(b: i16) -> Divisor<Fixed<16, -8>> {
    let member = Member::<G>::adapted::<Signature<G, Adapt<HalfEven, Saturate>>>(
        Exact::on_grid(Slot::at(b as i128)),
        Dither::UNUSED,
    );
    Divisor::member(member)
}

#[inline(always)]
fn value(x: i16) -> Num<S, Quiet> {
    Num::<S, Quiet>::from_index(x)
}

/// The quotient on the machine's integers.
pub fn quotient_hand(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, b) = record(input, i);
        let b = (b as i32).clamp(1, i16::MAX as i32);
        put(&mut acc, clamp16(half_even((a as i32) << 8, b)));
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The value's quotient.
pub fn quotient_room(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, b) = record(input, i);
        put(&mut acc, value(a).div(divisor(b)).index::<i16>());
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The format's four-limb quotient on the same slots.
pub fn quotient_wide(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, b) = record(input, i);
        let slot = arvo_format::div::<S>(value(a).slot(), divisor(b), Dither::UNUSED);
        put(&mut acc, slot.index() as i16);
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The product on the machine's integers.
pub fn product_hand(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, b) = record(input, i);
        put(&mut acc, clamp16(half_even(a as i32 * b as i32, 256)));
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The value's product.
pub fn product_room(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, b) = record(input, i);
        put(&mut acc, value(a).mul(value(b)).index::<i16>());
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The format's four-limb product on the same slots.
pub fn product_wide(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, b) = record(input, i);
        let slot = arvo_format::mul::<S>(value(a).slot(), value(b).slot(), Dither::UNUSED);
        put(&mut acc, slot.index() as i16);
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The conversion onto sixteenths on the machine's integers. The target holds
/// every member of the source, so nothing clamps.
pub fn conversion_hand(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, _) = record(input, i);
        put(&mut acc, half_even(a as i32, 16) as i16);
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The value's conversion.
pub fn conversion_room(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, _) = record(input, i);
        put(&mut acc, value(a).convert::<T>().index::<i16>());
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}

/// The format's four-limb conversion on the same slot.
pub fn conversion_wide(input: &[u8], output: &mut [u8]) {
    let mut acc = 0u64;
    for i in 0 .. records(input.len()) {
        let (a, _) = record(input, i);
        let slot = arvo_format::convert::<Fixed<16, -8>, T>(value(a).slot(), Dither::UNUSED);
        put(&mut acc, slot.index() as i16);
    }
    output[.. 8].copy_from_slice(&acc.to_le_bytes());
}
