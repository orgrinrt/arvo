//! Ordinary generic Rust at scale, the shape a downstream stack crate is full of.
#![no_std]
#![crate_type = "lib"]
use core::marker::PhantomData;

pub trait Store {
    type Item: Copy;
    fn get(&self, i: usize) -> Self::Item;
    fn len(&self) -> usize;
}
pub trait Fold {
    type Acc;
    fn unit() -> Self::Acc;
    fn step(a: Self::Acc, x: u32) -> Self::Acc;
}
pub struct Arr<T: Copy, const N: usize>([T; N]);
impl<T: Copy, const N: usize> Store for Arr<T, N> {
    type Item = T;
    fn get(&self, i: usize) -> T {
        self.0[i]
    }
    fn len(&self) -> usize {
        N
    }
}
pub struct Chain<A, B>(A, B);
impl<A: Store, B: Store<Item = A::Item>> Store for Chain<A, B> {
    type Item = A::Item;
    fn get(&self, i: usize) -> A::Item {
        if i < self.0.len() {
            self.0.get(i)
        } else {
            self.1.get(i - self.0.len())
        }
    }
    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}
pub struct Tagged<T, M>(T, PhantomData<M>);
impl<T: Store, M> Store for Tagged<T, M> {
    type Item = T::Item;
    fn get(&self, i: usize) -> T::Item {
        self.0.get(i)
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}
pub fn reduce<S: Store<Item = u32>, F: Fold<Acc = u32>>(s: &S) -> u32 {
    let mut a = F::unit();
    let mut i = 0;
    while i < s.len() {
        a = F::step(a, s.get(i));
        i += 1;
    }
    a
}
// hlist membership, indexed so the impls do not overlap (the AccessSet shape).
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);
pub struct Here;
pub struct There<N>(PhantomData<N>);
pub trait Contains<X, Ix> {}
impl<H, T> Contains<H, Here> for Cons<H, T> {}
impl<H, T, X, N> Contains<X, There<N>> for Cons<H, T> where T: Contains<X, N> {}
pub fn needs<L, A, B, C>()
where
    L: Contains<u8, A> + Contains<u16, B> + Contains<u32, C>,
{
}
pub fn hlist_ok() {
    needs::<Cons<u8, Cons<u16, Cons<u32, Nil>>>, _, _, _>();
}

pub struct F0;
impl Fold for F0 {
    type Acc = u32;
    fn unit() -> u32 {
        0
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 0)
    }
}
pub fn use0(a: Arr<u32, 8>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F0>(a, PhantomData), b);
    reduce::<_, F0>(&c)
}

pub struct F1;
impl Fold for F1 {
    type Acc = u32;
    fn unit() -> u32 {
        1
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 1)
    }
}
pub fn use1(a: Arr<u32, 9>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F1>(a, PhantomData), b);
    reduce::<_, F1>(&c)
}

pub struct F2;
impl Fold for F2 {
    type Acc = u32;
    fn unit() -> u32 {
        2
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 2)
    }
}
pub fn use2(a: Arr<u32, 10>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F2>(a, PhantomData), b);
    reduce::<_, F2>(&c)
}

pub struct F3;
impl Fold for F3 {
    type Acc = u32;
    fn unit() -> u32 {
        3
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 3)
    }
}
pub fn use3(a: Arr<u32, 11>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F3>(a, PhantomData), b);
    reduce::<_, F3>(&c)
}

pub struct F4;
impl Fold for F4 {
    type Acc = u32;
    fn unit() -> u32 {
        4
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 4)
    }
}
pub fn use4(a: Arr<u32, 12>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F4>(a, PhantomData), b);
    reduce::<_, F4>(&c)
}

pub struct F5;
impl Fold for F5 {
    type Acc = u32;
    fn unit() -> u32 {
        5
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 5)
    }
}
pub fn use5(a: Arr<u32, 13>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F5>(a, PhantomData), b);
    reduce::<_, F5>(&c)
}

pub struct F6;
impl Fold for F6 {
    type Acc = u32;
    fn unit() -> u32 {
        6
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 6)
    }
}
pub fn use6(a: Arr<u32, 14>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F6>(a, PhantomData), b);
    reduce::<_, F6>(&c)
}

pub struct F7;
impl Fold for F7 {
    type Acc = u32;
    fn unit() -> u32 {
        7
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 7)
    }
}
pub fn use7(a: Arr<u32, 15>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F7>(a, PhantomData), b);
    reduce::<_, F7>(&c)
}

pub struct F8;
impl Fold for F8 {
    type Acc = u32;
    fn unit() -> u32 {
        8
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 8)
    }
}
pub fn use8(a: Arr<u32, 16>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F8>(a, PhantomData), b);
    reduce::<_, F8>(&c)
}

pub struct F9;
impl Fold for F9 {
    type Acc = u32;
    fn unit() -> u32 {
        9
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 9)
    }
}
pub fn use9(a: Arr<u32, 17>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F9>(a, PhantomData), b);
    reduce::<_, F9>(&c)
}

pub struct F10;
impl Fold for F10 {
    type Acc = u32;
    fn unit() -> u32 {
        10
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 10)
    }
}
pub fn use10(a: Arr<u32, 18>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F10>(a, PhantomData), b);
    reduce::<_, F10>(&c)
}

pub struct F11;
impl Fold for F11 {
    type Acc = u32;
    fn unit() -> u32 {
        11
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 11)
    }
}
pub fn use11(a: Arr<u32, 19>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F11>(a, PhantomData), b);
    reduce::<_, F11>(&c)
}

pub struct F12;
impl Fold for F12 {
    type Acc = u32;
    fn unit() -> u32 {
        12
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 12)
    }
}
pub fn use12(a: Arr<u32, 20>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F12>(a, PhantomData), b);
    reduce::<_, F12>(&c)
}

pub struct F13;
impl Fold for F13 {
    type Acc = u32;
    fn unit() -> u32 {
        13
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 13)
    }
}
pub fn use13(a: Arr<u32, 21>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F13>(a, PhantomData), b);
    reduce::<_, F13>(&c)
}

pub struct F14;
impl Fold for F14 {
    type Acc = u32;
    fn unit() -> u32 {
        14
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 14)
    }
}
pub fn use14(a: Arr<u32, 22>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F14>(a, PhantomData), b);
    reduce::<_, F14>(&c)
}

pub struct F15;
impl Fold for F15 {
    type Acc = u32;
    fn unit() -> u32 {
        15
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 15)
    }
}
pub fn use15(a: Arr<u32, 23>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F15>(a, PhantomData), b);
    reduce::<_, F15>(&c)
}

pub struct F16;
impl Fold for F16 {
    type Acc = u32;
    fn unit() -> u32 {
        16
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 16)
    }
}
pub fn use16(a: Arr<u32, 24>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F16>(a, PhantomData), b);
    reduce::<_, F16>(&c)
}

pub struct F17;
impl Fold for F17 {
    type Acc = u32;
    fn unit() -> u32 {
        17
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 17)
    }
}
pub fn use17(a: Arr<u32, 25>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F17>(a, PhantomData), b);
    reduce::<_, F17>(&c)
}

pub struct F18;
impl Fold for F18 {
    type Acc = u32;
    fn unit() -> u32 {
        18
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 18)
    }
}
pub fn use18(a: Arr<u32, 26>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F18>(a, PhantomData), b);
    reduce::<_, F18>(&c)
}

pub struct F19;
impl Fold for F19 {
    type Acc = u32;
    fn unit() -> u32 {
        19
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 19)
    }
}
pub fn use19(a: Arr<u32, 27>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F19>(a, PhantomData), b);
    reduce::<_, F19>(&c)
}

pub struct F20;
impl Fold for F20 {
    type Acc = u32;
    fn unit() -> u32 {
        20
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 20)
    }
}
pub fn use20(a: Arr<u32, 28>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F20>(a, PhantomData), b);
    reduce::<_, F20>(&c)
}

pub struct F21;
impl Fold for F21 {
    type Acc = u32;
    fn unit() -> u32 {
        21
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 21)
    }
}
pub fn use21(a: Arr<u32, 29>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F21>(a, PhantomData), b);
    reduce::<_, F21>(&c)
}

pub struct F22;
impl Fold for F22 {
    type Acc = u32;
    fn unit() -> u32 {
        22
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 22)
    }
}
pub fn use22(a: Arr<u32, 30>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F22>(a, PhantomData), b);
    reduce::<_, F22>(&c)
}

pub struct F23;
impl Fold for F23 {
    type Acc = u32;
    fn unit() -> u32 {
        23
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 23)
    }
}
pub fn use23(a: Arr<u32, 31>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F23>(a, PhantomData), b);
    reduce::<_, F23>(&c)
}

pub struct F24;
impl Fold for F24 {
    type Acc = u32;
    fn unit() -> u32 {
        24
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 24)
    }
}
pub fn use24(a: Arr<u32, 8>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F24>(a, PhantomData), b);
    reduce::<_, F24>(&c)
}

pub struct F25;
impl Fold for F25 {
    type Acc = u32;
    fn unit() -> u32 {
        25
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 25)
    }
}
pub fn use25(a: Arr<u32, 9>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F25>(a, PhantomData), b);
    reduce::<_, F25>(&c)
}

pub struct F26;
impl Fold for F26 {
    type Acc = u32;
    fn unit() -> u32 {
        26
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 26)
    }
}
pub fn use26(a: Arr<u32, 10>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F26>(a, PhantomData), b);
    reduce::<_, F26>(&c)
}

pub struct F27;
impl Fold for F27 {
    type Acc = u32;
    fn unit() -> u32 {
        27
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 27)
    }
}
pub fn use27(a: Arr<u32, 11>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F27>(a, PhantomData), b);
    reduce::<_, F27>(&c)
}

pub struct F28;
impl Fold for F28 {
    type Acc = u32;
    fn unit() -> u32 {
        28
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 28)
    }
}
pub fn use28(a: Arr<u32, 12>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F28>(a, PhantomData), b);
    reduce::<_, F28>(&c)
}

pub struct F29;
impl Fold for F29 {
    type Acc = u32;
    fn unit() -> u32 {
        29
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 29)
    }
}
pub fn use29(a: Arr<u32, 13>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F29>(a, PhantomData), b);
    reduce::<_, F29>(&c)
}

pub struct F30;
impl Fold for F30 {
    type Acc = u32;
    fn unit() -> u32 {
        30
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 30)
    }
}
pub fn use30(a: Arr<u32, 14>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F30>(a, PhantomData), b);
    reduce::<_, F30>(&c)
}

pub struct F31;
impl Fold for F31 {
    type Acc = u32;
    fn unit() -> u32 {
        31
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 31)
    }
}
pub fn use31(a: Arr<u32, 15>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F31>(a, PhantomData), b);
    reduce::<_, F31>(&c)
}

pub struct F32;
impl Fold for F32 {
    type Acc = u32;
    fn unit() -> u32 {
        32
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 32)
    }
}
pub fn use32(a: Arr<u32, 16>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F32>(a, PhantomData), b);
    reduce::<_, F32>(&c)
}

pub struct F33;
impl Fold for F33 {
    type Acc = u32;
    fn unit() -> u32 {
        33
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 33)
    }
}
pub fn use33(a: Arr<u32, 17>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F33>(a, PhantomData), b);
    reduce::<_, F33>(&c)
}

pub struct F34;
impl Fold for F34 {
    type Acc = u32;
    fn unit() -> u32 {
        34
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 34)
    }
}
pub fn use34(a: Arr<u32, 18>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F34>(a, PhantomData), b);
    reduce::<_, F34>(&c)
}

pub struct F35;
impl Fold for F35 {
    type Acc = u32;
    fn unit() -> u32 {
        35
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 35)
    }
}
pub fn use35(a: Arr<u32, 19>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F35>(a, PhantomData), b);
    reduce::<_, F35>(&c)
}

pub struct F36;
impl Fold for F36 {
    type Acc = u32;
    fn unit() -> u32 {
        36
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 36)
    }
}
pub fn use36(a: Arr<u32, 20>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F36>(a, PhantomData), b);
    reduce::<_, F36>(&c)
}

pub struct F37;
impl Fold for F37 {
    type Acc = u32;
    fn unit() -> u32 {
        37
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 37)
    }
}
pub fn use37(a: Arr<u32, 21>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F37>(a, PhantomData), b);
    reduce::<_, F37>(&c)
}

pub struct F38;
impl Fold for F38 {
    type Acc = u32;
    fn unit() -> u32 {
        38
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 38)
    }
}
pub fn use38(a: Arr<u32, 22>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F38>(a, PhantomData), b);
    reduce::<_, F38>(&c)
}

pub struct F39;
impl Fold for F39 {
    type Acc = u32;
    fn unit() -> u32 {
        39
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 39)
    }
}
pub fn use39(a: Arr<u32, 23>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F39>(a, PhantomData), b);
    reduce::<_, F39>(&c)
}

pub struct F40;
impl Fold for F40 {
    type Acc = u32;
    fn unit() -> u32 {
        40
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 40)
    }
}
pub fn use40(a: Arr<u32, 24>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F40>(a, PhantomData), b);
    reduce::<_, F40>(&c)
}

pub struct F41;
impl Fold for F41 {
    type Acc = u32;
    fn unit() -> u32 {
        41
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 41)
    }
}
pub fn use41(a: Arr<u32, 25>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F41>(a, PhantomData), b);
    reduce::<_, F41>(&c)
}

pub struct F42;
impl Fold for F42 {
    type Acc = u32;
    fn unit() -> u32 {
        42
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 42)
    }
}
pub fn use42(a: Arr<u32, 26>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F42>(a, PhantomData), b);
    reduce::<_, F42>(&c)
}

pub struct F43;
impl Fold for F43 {
    type Acc = u32;
    fn unit() -> u32 {
        43
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 43)
    }
}
pub fn use43(a: Arr<u32, 27>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F43>(a, PhantomData), b);
    reduce::<_, F43>(&c)
}

pub struct F44;
impl Fold for F44 {
    type Acc = u32;
    fn unit() -> u32 {
        44
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 44)
    }
}
pub fn use44(a: Arr<u32, 28>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F44>(a, PhantomData), b);
    reduce::<_, F44>(&c)
}

pub struct F45;
impl Fold for F45 {
    type Acc = u32;
    fn unit() -> u32 {
        45
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 45)
    }
}
pub fn use45(a: Arr<u32, 29>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F45>(a, PhantomData), b);
    reduce::<_, F45>(&c)
}

pub struct F46;
impl Fold for F46 {
    type Acc = u32;
    fn unit() -> u32 {
        46
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 46)
    }
}
pub fn use46(a: Arr<u32, 30>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F46>(a, PhantomData), b);
    reduce::<_, F46>(&c)
}

pub struct F47;
impl Fold for F47 {
    type Acc = u32;
    fn unit() -> u32 {
        47
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 47)
    }
}
pub fn use47(a: Arr<u32, 31>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F47>(a, PhantomData), b);
    reduce::<_, F47>(&c)
}

pub struct F48;
impl Fold for F48 {
    type Acc = u32;
    fn unit() -> u32 {
        48
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 48)
    }
}
pub fn use48(a: Arr<u32, 8>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F48>(a, PhantomData), b);
    reduce::<_, F48>(&c)
}

pub struct F49;
impl Fold for F49 {
    type Acc = u32;
    fn unit() -> u32 {
        49
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 49)
    }
}
pub fn use49(a: Arr<u32, 9>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F49>(a, PhantomData), b);
    reduce::<_, F49>(&c)
}

pub struct F50;
impl Fold for F50 {
    type Acc = u32;
    fn unit() -> u32 {
        50
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 50)
    }
}
pub fn use50(a: Arr<u32, 10>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F50>(a, PhantomData), b);
    reduce::<_, F50>(&c)
}

pub struct F51;
impl Fold for F51 {
    type Acc = u32;
    fn unit() -> u32 {
        51
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 51)
    }
}
pub fn use51(a: Arr<u32, 11>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F51>(a, PhantomData), b);
    reduce::<_, F51>(&c)
}

pub struct F52;
impl Fold for F52 {
    type Acc = u32;
    fn unit() -> u32 {
        52
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 52)
    }
}
pub fn use52(a: Arr<u32, 12>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F52>(a, PhantomData), b);
    reduce::<_, F52>(&c)
}

pub struct F53;
impl Fold for F53 {
    type Acc = u32;
    fn unit() -> u32 {
        53
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 53)
    }
}
pub fn use53(a: Arr<u32, 13>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F53>(a, PhantomData), b);
    reduce::<_, F53>(&c)
}

pub struct F54;
impl Fold for F54 {
    type Acc = u32;
    fn unit() -> u32 {
        54
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 54)
    }
}
pub fn use54(a: Arr<u32, 14>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F54>(a, PhantomData), b);
    reduce::<_, F54>(&c)
}

pub struct F55;
impl Fold for F55 {
    type Acc = u32;
    fn unit() -> u32 {
        55
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 55)
    }
}
pub fn use55(a: Arr<u32, 15>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F55>(a, PhantomData), b);
    reduce::<_, F55>(&c)
}

pub struct F56;
impl Fold for F56 {
    type Acc = u32;
    fn unit() -> u32 {
        56
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 56)
    }
}
pub fn use56(a: Arr<u32, 16>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F56>(a, PhantomData), b);
    reduce::<_, F56>(&c)
}

pub struct F57;
impl Fold for F57 {
    type Acc = u32;
    fn unit() -> u32 {
        57
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 57)
    }
}
pub fn use57(a: Arr<u32, 17>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F57>(a, PhantomData), b);
    reduce::<_, F57>(&c)
}

pub struct F58;
impl Fold for F58 {
    type Acc = u32;
    fn unit() -> u32 {
        58
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 58)
    }
}
pub fn use58(a: Arr<u32, 18>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F58>(a, PhantomData), b);
    reduce::<_, F58>(&c)
}

pub struct F59;
impl Fold for F59 {
    type Acc = u32;
    fn unit() -> u32 {
        59
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 59)
    }
}
pub fn use59(a: Arr<u32, 19>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F59>(a, PhantomData), b);
    reduce::<_, F59>(&c)
}

pub struct F60;
impl Fold for F60 {
    type Acc = u32;
    fn unit() -> u32 {
        60
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 60)
    }
}
pub fn use60(a: Arr<u32, 20>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F60>(a, PhantomData), b);
    reduce::<_, F60>(&c)
}

pub struct F61;
impl Fold for F61 {
    type Acc = u32;
    fn unit() -> u32 {
        61
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 61)
    }
}
pub fn use61(a: Arr<u32, 21>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F61>(a, PhantomData), b);
    reduce::<_, F61>(&c)
}

pub struct F62;
impl Fold for F62 {
    type Acc = u32;
    fn unit() -> u32 {
        62
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 62)
    }
}
pub fn use62(a: Arr<u32, 22>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F62>(a, PhantomData), b);
    reduce::<_, F62>(&c)
}

pub struct F63;
impl Fold for F63 {
    type Acc = u32;
    fn unit() -> u32 {
        63
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 63)
    }
}
pub fn use63(a: Arr<u32, 23>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F63>(a, PhantomData), b);
    reduce::<_, F63>(&c)
}

pub struct F64;
impl Fold for F64 {
    type Acc = u32;
    fn unit() -> u32 {
        64
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 64)
    }
}
pub fn use64(a: Arr<u32, 24>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F64>(a, PhantomData), b);
    reduce::<_, F64>(&c)
}

pub struct F65;
impl Fold for F65 {
    type Acc = u32;
    fn unit() -> u32 {
        65
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 65)
    }
}
pub fn use65(a: Arr<u32, 25>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F65>(a, PhantomData), b);
    reduce::<_, F65>(&c)
}

pub struct F66;
impl Fold for F66 {
    type Acc = u32;
    fn unit() -> u32 {
        66
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 66)
    }
}
pub fn use66(a: Arr<u32, 26>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F66>(a, PhantomData), b);
    reduce::<_, F66>(&c)
}

pub struct F67;
impl Fold for F67 {
    type Acc = u32;
    fn unit() -> u32 {
        67
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 67)
    }
}
pub fn use67(a: Arr<u32, 27>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F67>(a, PhantomData), b);
    reduce::<_, F67>(&c)
}

pub struct F68;
impl Fold for F68 {
    type Acc = u32;
    fn unit() -> u32 {
        68
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 68)
    }
}
pub fn use68(a: Arr<u32, 28>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F68>(a, PhantomData), b);
    reduce::<_, F68>(&c)
}

pub struct F69;
impl Fold for F69 {
    type Acc = u32;
    fn unit() -> u32 {
        69
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 69)
    }
}
pub fn use69(a: Arr<u32, 29>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F69>(a, PhantomData), b);
    reduce::<_, F69>(&c)
}

pub struct F70;
impl Fold for F70 {
    type Acc = u32;
    fn unit() -> u32 {
        70
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 70)
    }
}
pub fn use70(a: Arr<u32, 30>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F70>(a, PhantomData), b);
    reduce::<_, F70>(&c)
}

pub struct F71;
impl Fold for F71 {
    type Acc = u32;
    fn unit() -> u32 {
        71
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 71)
    }
}
pub fn use71(a: Arr<u32, 31>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F71>(a, PhantomData), b);
    reduce::<_, F71>(&c)
}

pub struct F72;
impl Fold for F72 {
    type Acc = u32;
    fn unit() -> u32 {
        72
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 72)
    }
}
pub fn use72(a: Arr<u32, 8>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F72>(a, PhantomData), b);
    reduce::<_, F72>(&c)
}

pub struct F73;
impl Fold for F73 {
    type Acc = u32;
    fn unit() -> u32 {
        73
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 73)
    }
}
pub fn use73(a: Arr<u32, 9>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F73>(a, PhantomData), b);
    reduce::<_, F73>(&c)
}

pub struct F74;
impl Fold for F74 {
    type Acc = u32;
    fn unit() -> u32 {
        74
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 74)
    }
}
pub fn use74(a: Arr<u32, 10>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F74>(a, PhantomData), b);
    reduce::<_, F74>(&c)
}

pub struct F75;
impl Fold for F75 {
    type Acc = u32;
    fn unit() -> u32 {
        75
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 75)
    }
}
pub fn use75(a: Arr<u32, 11>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F75>(a, PhantomData), b);
    reduce::<_, F75>(&c)
}

pub struct F76;
impl Fold for F76 {
    type Acc = u32;
    fn unit() -> u32 {
        76
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 76)
    }
}
pub fn use76(a: Arr<u32, 12>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F76>(a, PhantomData), b);
    reduce::<_, F76>(&c)
}

pub struct F77;
impl Fold for F77 {
    type Acc = u32;
    fn unit() -> u32 {
        77
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 77)
    }
}
pub fn use77(a: Arr<u32, 13>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F77>(a, PhantomData), b);
    reduce::<_, F77>(&c)
}

pub struct F78;
impl Fold for F78 {
    type Acc = u32;
    fn unit() -> u32 {
        78
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 78)
    }
}
pub fn use78(a: Arr<u32, 14>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F78>(a, PhantomData), b);
    reduce::<_, F78>(&c)
}

pub struct F79;
impl Fold for F79 {
    type Acc = u32;
    fn unit() -> u32 {
        79
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 79)
    }
}
pub fn use79(a: Arr<u32, 15>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F79>(a, PhantomData), b);
    reduce::<_, F79>(&c)
}

pub struct F80;
impl Fold for F80 {
    type Acc = u32;
    fn unit() -> u32 {
        80
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 80)
    }
}
pub fn use80(a: Arr<u32, 16>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F80>(a, PhantomData), b);
    reduce::<_, F80>(&c)
}

pub struct F81;
impl Fold for F81 {
    type Acc = u32;
    fn unit() -> u32 {
        81
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 81)
    }
}
pub fn use81(a: Arr<u32, 17>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F81>(a, PhantomData), b);
    reduce::<_, F81>(&c)
}

pub struct F82;
impl Fold for F82 {
    type Acc = u32;
    fn unit() -> u32 {
        82
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 82)
    }
}
pub fn use82(a: Arr<u32, 18>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F82>(a, PhantomData), b);
    reduce::<_, F82>(&c)
}

pub struct F83;
impl Fold for F83 {
    type Acc = u32;
    fn unit() -> u32 {
        83
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 83)
    }
}
pub fn use83(a: Arr<u32, 19>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F83>(a, PhantomData), b);
    reduce::<_, F83>(&c)
}

pub struct F84;
impl Fold for F84 {
    type Acc = u32;
    fn unit() -> u32 {
        84
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 84)
    }
}
pub fn use84(a: Arr<u32, 20>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F84>(a, PhantomData), b);
    reduce::<_, F84>(&c)
}

pub struct F85;
impl Fold for F85 {
    type Acc = u32;
    fn unit() -> u32 {
        85
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 85)
    }
}
pub fn use85(a: Arr<u32, 21>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F85>(a, PhantomData), b);
    reduce::<_, F85>(&c)
}

pub struct F86;
impl Fold for F86 {
    type Acc = u32;
    fn unit() -> u32 {
        86
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 86)
    }
}
pub fn use86(a: Arr<u32, 22>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F86>(a, PhantomData), b);
    reduce::<_, F86>(&c)
}

pub struct F87;
impl Fold for F87 {
    type Acc = u32;
    fn unit() -> u32 {
        87
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 87)
    }
}
pub fn use87(a: Arr<u32, 23>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F87>(a, PhantomData), b);
    reduce::<_, F87>(&c)
}

pub struct F88;
impl Fold for F88 {
    type Acc = u32;
    fn unit() -> u32 {
        88
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 88)
    }
}
pub fn use88(a: Arr<u32, 24>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F88>(a, PhantomData), b);
    reduce::<_, F88>(&c)
}

pub struct F89;
impl Fold for F89 {
    type Acc = u32;
    fn unit() -> u32 {
        89
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 89)
    }
}
pub fn use89(a: Arr<u32, 25>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F89>(a, PhantomData), b);
    reduce::<_, F89>(&c)
}

pub struct F90;
impl Fold for F90 {
    type Acc = u32;
    fn unit() -> u32 {
        90
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 90)
    }
}
pub fn use90(a: Arr<u32, 26>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F90>(a, PhantomData), b);
    reduce::<_, F90>(&c)
}

pub struct F91;
impl Fold for F91 {
    type Acc = u32;
    fn unit() -> u32 {
        91
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 91)
    }
}
pub fn use91(a: Arr<u32, 27>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F91>(a, PhantomData), b);
    reduce::<_, F91>(&c)
}

pub struct F92;
impl Fold for F92 {
    type Acc = u32;
    fn unit() -> u32 {
        92
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 92)
    }
}
pub fn use92(a: Arr<u32, 28>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F92>(a, PhantomData), b);
    reduce::<_, F92>(&c)
}

pub struct F93;
impl Fold for F93 {
    type Acc = u32;
    fn unit() -> u32 {
        93
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 93)
    }
}
pub fn use93(a: Arr<u32, 29>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F93>(a, PhantomData), b);
    reduce::<_, F93>(&c)
}

pub struct F94;
impl Fold for F94 {
    type Acc = u32;
    fn unit() -> u32 {
        94
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 94)
    }
}
pub fn use94(a: Arr<u32, 30>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F94>(a, PhantomData), b);
    reduce::<_, F94>(&c)
}

pub struct F95;
impl Fold for F95 {
    type Acc = u32;
    fn unit() -> u32 {
        95
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 95)
    }
}
pub fn use95(a: Arr<u32, 31>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F95>(a, PhantomData), b);
    reduce::<_, F95>(&c)
}

pub struct F96;
impl Fold for F96 {
    type Acc = u32;
    fn unit() -> u32 {
        96
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 96)
    }
}
pub fn use96(a: Arr<u32, 8>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F96>(a, PhantomData), b);
    reduce::<_, F96>(&c)
}

pub struct F97;
impl Fold for F97 {
    type Acc = u32;
    fn unit() -> u32 {
        97
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 97)
    }
}
pub fn use97(a: Arr<u32, 9>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F97>(a, PhantomData), b);
    reduce::<_, F97>(&c)
}

pub struct F98;
impl Fold for F98 {
    type Acc = u32;
    fn unit() -> u32 {
        98
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 98)
    }
}
pub fn use98(a: Arr<u32, 10>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F98>(a, PhantomData), b);
    reduce::<_, F98>(&c)
}

pub struct F99;
impl Fold for F99 {
    type Acc = u32;
    fn unit() -> u32 {
        99
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 99)
    }
}
pub fn use99(a: Arr<u32, 11>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F99>(a, PhantomData), b);
    reduce::<_, F99>(&c)
}

pub struct F100;
impl Fold for F100 {
    type Acc = u32;
    fn unit() -> u32 {
        100
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 100)
    }
}
pub fn use100(a: Arr<u32, 12>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F100>(a, PhantomData), b);
    reduce::<_, F100>(&c)
}

pub struct F101;
impl Fold for F101 {
    type Acc = u32;
    fn unit() -> u32 {
        101
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 101)
    }
}
pub fn use101(a: Arr<u32, 13>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F101>(a, PhantomData), b);
    reduce::<_, F101>(&c)
}

pub struct F102;
impl Fold for F102 {
    type Acc = u32;
    fn unit() -> u32 {
        102
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 102)
    }
}
pub fn use102(a: Arr<u32, 14>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F102>(a, PhantomData), b);
    reduce::<_, F102>(&c)
}

pub struct F103;
impl Fold for F103 {
    type Acc = u32;
    fn unit() -> u32 {
        103
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 103)
    }
}
pub fn use103(a: Arr<u32, 15>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F103>(a, PhantomData), b);
    reduce::<_, F103>(&c)
}

pub struct F104;
impl Fold for F104 {
    type Acc = u32;
    fn unit() -> u32 {
        104
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 104)
    }
}
pub fn use104(a: Arr<u32, 16>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F104>(a, PhantomData), b);
    reduce::<_, F104>(&c)
}

pub struct F105;
impl Fold for F105 {
    type Acc = u32;
    fn unit() -> u32 {
        105
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 105)
    }
}
pub fn use105(a: Arr<u32, 17>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F105>(a, PhantomData), b);
    reduce::<_, F105>(&c)
}

pub struct F106;
impl Fold for F106 {
    type Acc = u32;
    fn unit() -> u32 {
        106
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 106)
    }
}
pub fn use106(a: Arr<u32, 18>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F106>(a, PhantomData), b);
    reduce::<_, F106>(&c)
}

pub struct F107;
impl Fold for F107 {
    type Acc = u32;
    fn unit() -> u32 {
        107
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 107)
    }
}
pub fn use107(a: Arr<u32, 19>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F107>(a, PhantomData), b);
    reduce::<_, F107>(&c)
}

pub struct F108;
impl Fold for F108 {
    type Acc = u32;
    fn unit() -> u32 {
        108
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 108)
    }
}
pub fn use108(a: Arr<u32, 20>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F108>(a, PhantomData), b);
    reduce::<_, F108>(&c)
}

pub struct F109;
impl Fold for F109 {
    type Acc = u32;
    fn unit() -> u32 {
        109
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 109)
    }
}
pub fn use109(a: Arr<u32, 21>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F109>(a, PhantomData), b);
    reduce::<_, F109>(&c)
}

pub struct F110;
impl Fold for F110 {
    type Acc = u32;
    fn unit() -> u32 {
        110
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 110)
    }
}
pub fn use110(a: Arr<u32, 22>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F110>(a, PhantomData), b);
    reduce::<_, F110>(&c)
}

pub struct F111;
impl Fold for F111 {
    type Acc = u32;
    fn unit() -> u32 {
        111
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 111)
    }
}
pub fn use111(a: Arr<u32, 23>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F111>(a, PhantomData), b);
    reduce::<_, F111>(&c)
}

pub struct F112;
impl Fold for F112 {
    type Acc = u32;
    fn unit() -> u32 {
        112
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 112)
    }
}
pub fn use112(a: Arr<u32, 24>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F112>(a, PhantomData), b);
    reduce::<_, F112>(&c)
}

pub struct F113;
impl Fold for F113 {
    type Acc = u32;
    fn unit() -> u32 {
        113
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 113)
    }
}
pub fn use113(a: Arr<u32, 25>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F113>(a, PhantomData), b);
    reduce::<_, F113>(&c)
}

pub struct F114;
impl Fold for F114 {
    type Acc = u32;
    fn unit() -> u32 {
        114
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 114)
    }
}
pub fn use114(a: Arr<u32, 26>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F114>(a, PhantomData), b);
    reduce::<_, F114>(&c)
}

pub struct F115;
impl Fold for F115 {
    type Acc = u32;
    fn unit() -> u32 {
        115
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 115)
    }
}
pub fn use115(a: Arr<u32, 27>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F115>(a, PhantomData), b);
    reduce::<_, F115>(&c)
}

pub struct F116;
impl Fold for F116 {
    type Acc = u32;
    fn unit() -> u32 {
        116
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 116)
    }
}
pub fn use116(a: Arr<u32, 28>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F116>(a, PhantomData), b);
    reduce::<_, F116>(&c)
}

pub struct F117;
impl Fold for F117 {
    type Acc = u32;
    fn unit() -> u32 {
        117
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 117)
    }
}
pub fn use117(a: Arr<u32, 29>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F117>(a, PhantomData), b);
    reduce::<_, F117>(&c)
}

pub struct F118;
impl Fold for F118 {
    type Acc = u32;
    fn unit() -> u32 {
        118
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 118)
    }
}
pub fn use118(a: Arr<u32, 30>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F118>(a, PhantomData), b);
    reduce::<_, F118>(&c)
}

pub struct F119;
impl Fold for F119 {
    type Acc = u32;
    fn unit() -> u32 {
        119
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x ^ 119)
    }
}
pub fn use119(a: Arr<u32, 31>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, F119>(a, PhantomData), b);
    reduce::<_, F119>(&c)
}
