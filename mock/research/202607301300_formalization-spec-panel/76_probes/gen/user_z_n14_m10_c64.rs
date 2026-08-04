#![no_std]
use mach::*;
pub type Ib0 = Pz<H>;
pub type Fb0 = Z;
pub type N0 = UFixed<Ib0, Fb0, Hot>;
pub const W0: usize = <N0 as Stored>::W;
const _: () = assert!(W0 == 1);
pub fn one_ok_0()
where
    N0: HasOne,
{
}
pub fn int_like_0()
where
    Fb0: IsZero,
{
}
pub type Ib1 = Pz<O<H>>;
pub type Fb1 = Z;
pub type N1 = UFixed<Ib1, Fb1, Warm>;
pub const W1: usize = <N1 as Stored>::W;
const _: () = assert!(W1 == 2);
pub fn one_ok_1()
where
    N1: HasOne,
{
}
pub fn int_like_1()
where
    Fb1: IsZero,
{
}
pub type Ib2 = Pz<I<H>>;
pub type Fb2 = Z;
pub type N2 = UFixed<Ib2, Fb2, Cold>;
pub const W2: usize = <N2 as Stored>::W;
const _: () = assert!(W2 == 3);
pub fn one_ok_2()
where
    N2: HasOne,
{
}
pub fn int_like_2()
where
    Fb2: IsZero,
{
}
pub type Ib3 = Pz<O<O<H>>>;
pub type Fb3 = Z;
pub type N3 = UFixed<Ib3, Fb3, Hot>;
pub const W3: usize = <N3 as Stored>::W;
const _: () = assert!(W3 == 4);
pub fn one_ok_3()
where
    N3: HasOne,
{
}
pub fn int_like_3()
where
    Fb3: IsZero,
{
}
pub type Ib4 = Pz<I<O<H>>>;
pub type Fb4 = Z;
pub type N4 = UFixed<Ib4, Fb4, Warm>;
pub const W4: usize = <N4 as Stored>::W;
const _: () = assert!(W4 == 5);
pub fn one_ok_4()
where
    N4: HasOne,
{
}
pub fn int_like_4()
where
    Fb4: IsZero,
{
}
pub type Ib5 = Pz<O<I<H>>>;
pub type Fb5 = Z;
pub type N5 = UFixed<Ib5, Fb5, Cold>;
pub const W5: usize = <N5 as Stored>::W;
const _: () = assert!(W5 == 6);
pub fn one_ok_5()
where
    N5: HasOne,
{
}
pub fn int_like_5()
where
    Fb5: IsZero,
{
}
pub type Ib6 = Pz<I<I<H>>>;
pub type Fb6 = Z;
pub type N6 = UFixed<Ib6, Fb6, Hot>;
pub const W6: usize = <N6 as Stored>::W;
const _: () = assert!(W6 == 7);
pub fn one_ok_6()
where
    N6: HasOne,
{
}
pub fn int_like_6()
where
    Fb6: IsZero,
{
}
pub type Ib7 = Pz<I<I<O<H>>>>;
pub type Fb7 = Z;
pub type N7 = UFixed<Ib7, Fb7, Warm>;
pub const W7: usize = <N7 as Stored>::W;
const _: () = assert!(W7 == 11);
pub fn one_ok_7()
where
    N7: HasOne,
{
}
pub fn int_like_7()
where
    Fb7: IsZero,
{
}
pub type Ib8 = Pz<O<I<I<H>>>>;
pub type Fb8 = Z;
pub type N8 = UFixed<Ib8, Fb8, Cold>;
pub const W8: usize = <N8 as Stored>::W;
const _: () = assert!(W8 == 14);
pub fn one_ok_8()
where
    N8: HasOne,
{
}
pub fn int_like_8()
where
    Fb8: IsZero,
{
}
pub type Ib9 = Pz<O<O<O<O<H>>>>>;
pub type Fb9 = Z;
pub type N9 = UFixed<Ib9, Fb9, Hot>;
pub const W9: usize = <N9 as Stored>::W;
const _: () = assert!(W9 == 16);
pub fn one_ok_9()
where
    N9: HasOne,
{
}
pub fn int_like_9()
where
    Fb9: IsZero,
{
}
pub type Ib10 = Pz<I<I<O<I<H>>>>>;
pub type Fb10 = Z;
pub type N10 = UFixed<Ib10, Fb10, Warm>;
pub const W10: usize = <N10 as Stored>::W;
const _: () = assert!(W10 == 27);
pub fn one_ok_10()
where
    N10: HasOne,
{
}
pub fn int_like_10()
where
    Fb10: IsZero,
{
}
pub type Ib11 = Pz<O<O<I<I<H>>>>>;
pub type Fb11 = Z;
pub type N11 = UFixed<Ib11, Fb11, Cold>;
pub const W11: usize = <N11 as Stored>::W;
const _: () = assert!(W11 == 28);
pub fn one_ok_11()
where
    N11: HasOne,
{
}
pub fn int_like_11()
where
    Fb11: IsZero,
{
}
pub type Ib12 = Pz<O<O<O<O<O<O<H>>>>>>>;
pub type Fb12 = Z;
pub type N12 = UFixed<Ib12, Fb12, Hot>;
pub const W12: usize = <N12 as Stored>::W;
const _: () = assert!(W12 == 64);
pub fn one_ok_12()
where
    N12: HasOne,
{
}
pub fn int_like_12()
where
    Fb12: IsZero,
{
}
pub type Ib13 = Z;
pub type Fb13 = Pz<O<O<O<O<H>>>>>;
pub type N13 = UFixed<Ib13, Fb13, Warm>;
pub const W13: usize = <N13 as Stored>::W;
const _: () = assert!(W13 == 16);
pub fn frac_like_13()
where
    Fb13: NonZero,
{
}
pub type Sum0 = <N0 as AddNum<N3>>::Out;
pub const SW0: usize = <Sum0 as Stored>::W;
pub type Sum1 = <N1 as AddNum<N4>>::Out;
pub const SW1: usize = <Sum1 as Stored>::W;
pub type Sum2 = <N2 as AddNum<N5>>::Out;
pub const SW2: usize = <Sum2 as Stored>::W;
pub type Sum3 = <N3 as AddNum<N6>>::Out;
pub const SW3: usize = <Sum3 as Stored>::W;
pub type Sum4 = <N4 as AddNum<N7>>::Out;
pub const SW4: usize = <Sum4 as Stored>::W;
pub type Sum5 = <N5 as AddNum<N8>>::Out;
pub const SW5: usize = <Sum5 as Stored>::W;
pub type Sum6 = <N6 as AddNum<N9>>::Out;
pub const SW6: usize = <Sum6 as Stored>::W;
pub type Sum7 = <N7 as AddNum<N10>>::Out;
pub const SW7: usize = <Sum7 as Stored>::W;
pub type Sum8 = <N8 as AddNum<N11>>::Out;
pub const SW8: usize = <Sum8 as Stored>::W;
pub type Sum9 = <N9 as AddNum<N12>>::Out;
pub const SW9: usize = <Sum9 as Stored>::W;
pub type Sum10 = <N10 as AddNum<N13>>::Out;
pub const SW10: usize = <Sum10 as Stored>::W;
pub type C0 = Slot<Pz<H>, 1>;
pub fn build0() -> <C0 as Capacity>::Array<u32> {
    C0::build(0)
}
pub type C1 = Slot<Pz<I<H>>, 3>;
pub fn build1() -> <C1 as Capacity>::Array<u32> {
    C1::build(0)
}
pub type C2 = Slot<Pz<O<O<H>>>, 4>;
pub fn build2() -> <C2 as Capacity>::Array<u32> {
    C2::build(0)
}
pub type C3 = Slot<Pz<I<I<H>>>, 7>;
pub fn build3() -> <C3 as Capacity>::Array<u32> {
    C3::build(0)
}
pub type C4 = Slot<Pz<O<O<O<H>>>>, 8>;
pub fn build4() -> <C4 as Capacity>::Array<u32> {
    C4::build(0)
}
pub type C5 = Slot<Pz<I<O<I<H>>>>, 13>;
pub fn build5() -> <C5 as Capacity>::Array<u32> {
    C5::build(0)
}
pub type C6 = Slot<Pz<O<O<O<O<H>>>>>, 16>;
pub fn build6() -> <C6 as Capacity>::Array<u32> {
    C6::build(0)
}
pub type C7 = Slot<Pz<O<O<I<I<H>>>>>, 28>;
pub fn build7() -> <C7 as Capacity>::Array<u32> {
    C7::build(0)
}
pub type C8 = Slot<Pz<O<O<O<O<O<H>>>>>>, 32>;
pub fn build8() -> <C8 as Capacity>::Array<u32> {
    C8::build(0)
}
pub type C9 = Slot<Pz<O<O<O<O<O<O<H>>>>>>>, 64>;
pub fn build9() -> <C9 as Capacity>::Array<u32> {
    C9::build(0)
}

// Obligation 4: build and walk, generic over the capacity.
pub fn fold_generic<C: Capacity>(seed: u32) -> u32 {
    let mut a = C::build(seed);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0;
    while i < s.len() {
        s[i] = s[i].wrapping_add(i as u32);
        i += 1;
    }
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32;
    let mut j = 0;
    while j < r.len() {
        acc = acc.wrapping_add(r[j]);
        j += 1;
    }
    acc
}
// Obligation 5: generic over a numeral AND a capacity at once. This is the
// site staging cannot reach, because neither width is known here.
pub fn scaled_fold<Ib, Fb, S, C>(seed: u32) -> u32
where
    Ib: Nat + AddN<Fb>,
    Fb: Nat,
    S: Strategy,
    C: Capacity,
    UFixed<Ib, Fb, S>: Stored + HasOne,
{
    fold_generic::<C>(seed).wrapping_mul(<UFixed<Ib, Fb, S> as Stored>::W as u32)
}

pub fn call0() -> u32 {
    scaled_fold::<Ib0, Fb0, Hot, C0>(0)
}
pub fn call1() -> u32 {
    scaled_fold::<Ib1, Fb1, Warm, C1>(1)
}
pub fn call2() -> u32 {
    scaled_fold::<Ib2, Fb2, Cold, C2>(2)
}
pub fn call3() -> u32 {
    scaled_fold::<Ib3, Fb3, Hot, C3>(3)
}
pub fn call4() -> u32 {
    scaled_fold::<Ib4, Fb4, Warm, C4>(4)
}
pub fn call5() -> u32 {
    scaled_fold::<Ib5, Fb5, Cold, C5>(5)
}
pub fn call6() -> u32 {
    scaled_fold::<Ib6, Fb6, Hot, C6>(6)
}
pub fn call7() -> u32 {
    scaled_fold::<Ib7, Fb7, Warm, C7>(7)
}
pub fn call8() -> u32 {
    scaled_fold::<Ib8, Fb8, Cold, C8>(8)
}
pub fn call9() -> u32 {
    scaled_fold::<Ib9, Fb9, Hot, C9>(9)
}
