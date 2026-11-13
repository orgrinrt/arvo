//! The probe on the arm built on: a second type per signature holding an exact
//! sum in the double-width machine integer, with `widen`, exact `add` and `sub`,
//! and `narrow` as the one adaptation. And `Counted`, the same accumulator with
//! its term count in its type.

use core::mem::size_of;

use common::{Machine, Sole, rung};

use super::{Num, copy};

/// An exact sum of members of `S`, held in the signed integer twice the
/// carrier's width.
pub struct Wide<S: Sole>(S::Wide);

/// One operation on the accumulator, dispatched on the carrier's size to the
/// double-width integer. The match is on constants of the instantiation.
macro_rules! wide {
    ($r:ty, $w:ty, |$t:ident| $body:expr) => {
        match size_of::<$r>() {
            1 => {
                type $t = i16;
                copy::<$t, $w>($body)
            },
            2 => {
                type $t = i32;
                copy::<$t, $w>($body)
            },
            4 => {
                type $t = i64;
                copy::<$t, $w>($body)
            },
            8 => {
                type $t = i128;
                copy::<$t, $w>($body)
            },
            _ => unreachable!(),
        }
    };
}

/// How many members of a `width`-bit range a signed register of `register`
/// bits holds the sum of exactly, whatever the members.
///
/// Signed: the members reach `-2^(width-1)`, the register `-2^(register-1)`, so
/// `2^(register-width)` of the lowest member is the most it holds; the top side
/// holds more. Unsigned: `(2^(register-1) - 1) / (2^width - 1)`.
#[must_use]
pub const fn max_terms(width: u32, register: u32, signed: bool) -> u128 {
    if signed {
        1u128 << (register - width)
    } else {
        ((1u128 << (register - 1)) - 1) / ((1u128 << width) - 1)
    }
}

impl<S: Sole> Wide<S> {
    /// How many members' sum the register holds exactly, in any order.
    pub const MAX_TERMS: u128 = max_terms(
        S::WIDTH.count(),
        2 * <S::Repr as Machine>::BITS,
        <S::Repr as Machine>::SIGNED,
    );
    /// The empty sum.
    pub const ZERO: Self = Self(<S::Repr as Machine>::WIDE_ZERO);

    /// The exact sum. Past the register it panics in a build with overflow
    /// checks, refuses in const evaluation, and wraps in a build without.
    #[must_use]
    pub const fn add(self, rhs: Self) -> Self {
        Self(wide!(S::Repr, S::Wide, |T| {
            copy::<S::Wide, T>(self.0) + copy::<S::Wide, T>(rhs.0)
        }))
    }

    /// The exact difference, with `add`'s behaviour past the register.
    #[must_use]
    pub const fn sub(self, rhs: Self) -> Self {
        Self(wide!(S::Repr, S::Wide, |T| {
            copy::<S::Wide, T>(self.0) - copy::<S::Wide, T>(rhs.0)
        }))
    }

    /// The one adaptation of the sum onto the format.
    #[must_use]
    pub const fn narrow(self) -> Num<S> {
        let (w, p) = (S::WIDTH, S::POLICY);
        Num(match (size_of::<S::Repr>(), <S::Repr as Machine>::SIGNED) {
            (1, true) => copy::<i8, S::Repr>(rung::narrow_s8(copy(self.0), w, p)),
            (1, false) => copy::<u8, S::Repr>(rung::narrow_u8(copy(self.0), w, p)),
            (2, true) => copy::<i16, S::Repr>(rung::narrow_s16(copy(self.0), w, p)),
            (2, false) => copy::<u16, S::Repr>(rung::narrow_u16(copy(self.0), w, p)),
            (4, true) => copy::<i32, S::Repr>(rung::narrow_s32(copy(self.0), w, p)),
            (4, false) => copy::<u32, S::Repr>(rung::narrow_u32(copy(self.0), w, p)),
            (8, true) => copy::<i64, S::Repr>(rung::narrow_s64(copy(self.0), w, p)),
            (8, false) => copy::<u64, S::Repr>(rung::narrow_u64(copy(self.0), w, p)),
            _ => unreachable!(),
        })
    }

    /// The register, an unwrap door for the tests.
    #[must_use]
    pub const fn raw(self) -> S::Wide {
        self.0
    }
}

impl<S: Sole> Num<S> {
    /// The member, exactly, in the accumulator.
    #[must_use]
    pub const fn widen(self) -> Wide<S> {
        Wide(match (size_of::<S::Repr>(), <S::Repr as Machine>::SIGNED) {
            (1, true) => copy::<i16, S::Wide>(copy::<S::Repr, i8>(self.0) as i16),
            (1, false) => copy::<i16, S::Wide>(copy::<S::Repr, u8>(self.0) as i16),
            (2, true) => copy::<i32, S::Wide>(copy::<S::Repr, i16>(self.0) as i32),
            (2, false) => copy::<i32, S::Wide>(copy::<S::Repr, u16>(self.0) as i32),
            (4, true) => copy::<i64, S::Wide>(copy::<S::Repr, i32>(self.0) as i64),
            (4, false) => copy::<i64, S::Wide>(copy::<S::Repr, u32>(self.0) as i64),
            (8, true) => copy::<i128, S::Wide>(copy::<S::Repr, i64>(self.0) as i128),
            (8, false) => copy::<i128, S::Wide>(copy::<S::Repr, u64>(self.0) as i128),
            _ => unreachable!(),
        })
    }
}

impl<S: Sole> Clone for Wide<S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S: Sole> Copy for Wide<S> {}
impl<S: Sole> PartialEq for Wide<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<S: Sole> core::fmt::Debug for Wide<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// A term count as a type: no terms.
pub struct Zero;
/// A term count as a type: one more than `N`.
pub struct Succ<N>(core::marker::PhantomData<N>);

/// A term count carried in a type, with its value as a constant.
pub trait Count {
    /// How many terms.
    const COUNT: u128;
}
impl Count for Zero {
    const COUNT: u128 = 0;
}
impl<N: Count> Count for Succ<N> {
    const COUNT: u128 = N::COUNT + 1;
}

/// The accumulator with its term count in its type. `push` moves it to the
/// next count; `narrow` refuses at build a count past `MAX_TERMS`. No
/// `generic_const_exprs`: the count is a type, and the check a constant of it.
pub struct Counted<S: Sole, N: Count>(Wide<S>, core::marker::PhantomData<N>);

impl<S: Sole> Counted<S, Zero> {
    /// The empty sum.
    pub const ZERO: Self = Self(Wide::ZERO, core::marker::PhantomData);
}

impl<S: Sole, N: Count> Counted<S, N> {
    /// The sum with one more member.
    #[must_use]
    pub const fn push(self, member: Num<S>) -> Counted<S, Succ<N>> {
        Counted(self.0.add(member.widen()), core::marker::PhantomData)
    }

    /// The one adaptation. Refused at build where the count is past the bound.
    #[must_use]
    pub const fn narrow(self) -> Num<S> {
        const {
            assert!(
                N::COUNT <= Wide::<S>::MAX_TERMS,
                "more terms than the register holds the sum of exactly"
            )
        };
        self.0.narrow()
    }
}
