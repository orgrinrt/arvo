//! Probe 1: the `Specials`-carrying, radix-parameterised numeral, with every new carrier
//! sealed at declaration time and its positive control compiled here.
//!
//! rustc --edition 2021 --crate-type lib probe_1_carriers_born_sealed.rs
//!
//! Four carriers are minted in `numeral.rs`: `Radix` (one constructor over the sealed
//! `Pos`, bounded on the sealed `AtLeastTwo`), `Specials`, `Underflow`, `SignDomain`. Each
//! gets a private supertrait at birth. The negative controls live in `probe_1b` through
//! `probe_1e`, one file per introduction route, because a compile-fail fixture that shares
//! a crate with a compiling one reports only the first error.
//!
//! What this file establishes positively:
//!   - every axis the numeral carries is readable as a const from the type alone,
//!   - the real formats instantiate (binary32, binary64, decimal64, OFP8 E4M3 and E5M2),
//!   - legitimate extension still works (a fourth radix nobody declared, by naming a `Pos`),
//!   - `mulnum` over two `Ranged` numerals still compiles at equal radix, and there is no
//!     impl at unequal radix, which is the correct refusal rather than a missing feature.

#![allow(dead_code)]

#[path = "vu_bias_sealed_adj.rs"]
pub mod bias;
#[path = "numeral.rs"]
pub mod numeral;

use bias::nat::{Pos, H, I, O};
use numeral::*;

// ---------------------------------------------------------------------------
// 1. the radix predicate does what it says
// ---------------------------------------------------------------------------

const _: () = assert!(<Two as Radix>::R == 2);
const _: () = assert!(<Ten as Radix>::R == 10);
const _: () = assert!(<Sixteen as Radix>::R == 16);

/// Legitimate extension: a radix nobody declared, expressed by naming a `Pos`. No new
/// inhabitant of any sealed trait is introduced; `Rad` is the only constructor and the
/// `Pos` is built from the sealed constructors. This is the "observation, not inhabitation"
/// clause of the seal's own quantification (`49:394-399`) applied to `Radix`.
type P3Local = I<H>;
type Three = Rad<P3Local>;
const _: () = assert!(<Three as Radix>::R == 3);

type P100 = O<O<I<O<O<I<H>>>>>>; // 100 = 0b1100100, low bit first: 0,0,1,0,0,1,1
const _: () = assert!(<P100 as Pos>::VAL == 100);
type Hundred = Rad<P100>;
const _: () = assert!(<Hundred as Radix>::R == 100);

// ---------------------------------------------------------------------------
// 2. Specials is a product, and its four corners
// ---------------------------------------------------------------------------

const _: () = assert!(!<NoSpecials as Specials>::INF && !<NoSpecials as Specials>::NAN);
const _: () = assert!(<InfOnly as Specials>::INF && !<InfOnly as Specials>::NAN);
const _: () = assert!(!<NanOnly as Specials>::INF && <NanOnly as Specials>::NAN);
const _: () = assert!(<IeeeSpecials as Specials>::INF && <IeeeSpecials as Specials>::NAN);

// ---------------------------------------------------------------------------
// 3. the real formats, spelled
// ---------------------------------------------------------------------------

type P63 = I<I<I<I<I<H>>>>>;
type P126 = O<P63>;
type P127 = I<P63>;

/// binary32. Radix two, precision 24, e in [-126, 127], gradual underflow, full IEEE
/// specials, a symmetric sign domain.
pub type Binary32 = Fl<Two, P24, ENeg<P126>, EPos<P127>, Gradual, IeeeSpecials, Symmetric>;
const _: () = assert!(<Binary32 as Numeral>::R == 2);
const _: () = assert!(<Binary32 as Numeral>::P == 24);
const _: () = assert!(<Binary32 as Numeral>::EMIN == -126);
const _: () = assert!(<Binary32 as Numeral>::EMAX == 127);
const _: () = assert!(<Binary32 as Numeral>::INF && <Binary32 as Numeral>::NAN);
const _: () = assert!(<Binary32 as Numeral>::GRADUAL);

/// OCP OFP8 `E5M2`: precision 3, e in [-14, 15], infinities and NaN. The IEEE-shaped
/// sibling.
type P14 = O<I<I<H>>>;
type P15 = I<I<I<H>>>;
pub type Fp8E5M2 = Fl<Two, P3, ENeg<P14>, EPos<P15>, Gradual, IeeeSpecials, Symmetric>;
const _: () = assert!(<Fp8E5M2 as Numeral>::P == 3);
const _: () = assert!(<Fp8E5M2 as Numeral>::INF);

/// OCP OFP8 `E4M3`: precision 4, e in [-6, 8], **NaN and no infinity**. The corner file
/// 50's three-instance chain cannot name, and the one with a shipping witness. The extra
/// binade (emax 8 rather than 7) is exactly what the freed infinity code buys.
type P6 = O<P3>;
pub type Fp8E4M3 = Fl<Two, P4, ENeg<P6>, EPos<P8>, Gradual, NanOnly, Symmetric>;
const _: () = assert!(<Fp8E4M3 as Numeral>::P == 4);
const _: () = assert!(!<Fp8E4M3 as Numeral>::INF);
const _: () = assert!(<Fp8E4M3 as Numeral>::NAN);
const _: () = assert!(<Fp8E4M3 as Numeral>::EMAX == 8);

/// decimal64. Radix **ten**, precision 16 decimal digits, e in [-383, 384]. The format the
/// `Radix` axis exists for, and the one whose encodings are not injective (probe 3).
type P16d = O<P8>;
type P383 = I<I<I<I<I<I<I<O<H>>>>>>>>; // 383 = 0b101111111
type P384 = O<O<O<O<O<O<O<I<H>>>>>>>>; // 384 = 0b110000000
const _: () = assert!(<P383 as Pos>::VAL == 383);
const _: () = assert!(<P384 as Pos>::VAL == 384);
pub type Decimal64 = Fl<Ten, P16d, ENeg<P383>, EPos<P384>, Gradual, IeeeSpecials, Symmetric>;
const _: () = assert!(<Decimal64 as Numeral>::R == 10);
const _: () = assert!(<Decimal64 as Numeral>::P == 16);
const _: () = assert!(<Decimal64 as Numeral>::EMIN == -383);
const _: () = assert!(<Decimal64 as Numeral>::EMAX == 384);

// ---------------------------------------------------------------------------
// 4. mulnum still composes, at equal radix, with the specials gate intact
// ---------------------------------------------------------------------------

pub type M1 = Fl<Two, P4, ENeg<P3>, EPos<P4>, Gradual, NoSpecials, Symmetric>;
pub type M2 = Fl<Two, P3, ENeg<P2>, EPos<P3>, Gradual, NoSpecials, Symmetric>;
pub type M12 = <M1 as MulNum<M2>>::Out;

const _: () = assert!(<M12 as Numeral>::P == 7);
const _: () = assert!(<M12 as Numeral>::EMIN == -5);
const _: () = assert!(<M12 as Numeral>::EMAX == 7);
const _: () = assert!(<M12 as Numeral>::R == 2);

/// The decimal half of the same map. Nothing about `mulnum` knew the radix was two.
pub type D1 = Fl<Ten, P4, ENeg<P3>, EPos<P4>, Gradual, NoSpecials, Symmetric>;
pub type D2 = Fl<Ten, P3, ENeg<P2>, EPos<P3>, Gradual, NoSpecials, Symmetric>;
pub type D12 = <D1 as MulNum<D2>>::Out;
const _: () = assert!(<D12 as Numeral>::R == 10);
const _: () = assert!(<D12 as Numeral>::P == 7);
const _: () = assert!(<D12 as Numeral>::EMIN == -5);

/// Forced through a signature rather than left in an inert alias. A bare alias defers its
/// bound checks; file 46's probe 3d was green while asserting nothing until it forced.
pub fn forced<N1, N2>() -> i64
where
    N1: Numeral + MulNum<N2>,
    N2: Numeral,
{
    <<N1 as MulNum<N2>>::Out as Numeral>::EMAX
}

pub fn call_forced() -> (i64, i64) {
    (forced::<M1, M2>(), forced::<D1, D2>())
}
