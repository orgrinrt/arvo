// P9 core. Everything the machinery needs, entirely at the type level, with
// ZERO feature gates: nat, addition, comparison, ceil-to-bytes, an exact-size
// byte buffer, and container selection across the native rungs plus the wide
// rung. The only const in the whole design is the literal a consumer writes.
#![no_std]
#![allow(dead_code)]

pub struct Z;
pub struct O<N>(N);
pub struct E<N>(N);

pub trait Add<R> {
    type Out;
}
pub trait AddC<R> {
    type Out;
}
pub type Sum<A, B> = <A as Add<B>>::Out;
pub type SumC<A, B> = <A as AddC<B>>::Out;

impl Add<Z> for Z {
    type Out = Z;
}
impl<B> Add<O<B>> for Z {
    type Out = O<B>;
}
impl<B> Add<E<B>> for Z {
    type Out = E<B>;
}
impl<A> Add<Z> for O<A> {
    type Out = O<A>;
}
impl<A> Add<Z> for E<A> {
    type Out = E<A>;
}
impl<A: AddC<B>, B> Add<O<B>> for O<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: Add<B>, B> Add<E<B>> for O<A> {
    type Out = O<Sum<A, B>>;
}
impl<A: Add<B>, B> Add<O<B>> for E<A> {
    type Out = O<Sum<A, B>>;
}
impl<A: Add<B>, B> Add<E<B>> for E<A> {
    type Out = E<Sum<A, B>>;
}

impl AddC<Z> for Z {
    type Out = O<Z>;
}
impl<B: AddC<Z>> AddC<O<B>> for Z {
    type Out = E<SumC<B, Z>>;
}
impl<B> AddC<E<B>> for Z {
    type Out = O<B>;
}
impl<A: AddC<Z>> AddC<Z> for O<A> {
    type Out = E<SumC<A, Z>>;
}
impl<A> AddC<Z> for E<A> {
    type Out = O<A>;
}
impl<A: AddC<B>, B> AddC<O<B>> for O<A> {
    type Out = O<SumC<A, B>>;
}
impl<A: AddC<B>, B> AddC<E<B>> for O<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: AddC<B>, B> AddC<O<B>> for E<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: Add<B>, B> AddC<E<B>> for E<A> {
    type Out = O<Sum<A, B>>;
}

// ordering, and the two finite selectors that consume it
pub struct Lt;
pub struct Eqq;
pub struct Gt;

pub trait OrElse<D> {
    type Out;
}
impl<D> OrElse<D> for Eqq {
    type Out = D;
}
impl<D> OrElse<D> for Lt {
    type Out = Lt;
}
impl<D> OrElse<D> for Gt {
    type Out = Gt;
}

pub trait IfLe<T, F> {
    type Out;
}
impl<T, F> IfLe<T, F> for Lt {
    type Out = T;
}
impl<T, F> IfLe<T, F> for Eqq {
    type Out = T;
}
impl<T, F> IfLe<T, F> for Gt {
    type Out = F;
}

pub trait Cmp<R> {
    type Out;
}
pub type Ord2<A, B> = <A as Cmp<B>>::Out;

impl Cmp<Z> for Z {
    type Out = Eqq;
}
impl<B> Cmp<O<B>> for Z {
    type Out = Lt;
}
impl<B> Cmp<E<B>> for Z {
    type Out = Lt;
}
impl<A> Cmp<Z> for O<A> {
    type Out = Gt;
}
impl<A> Cmp<Z> for E<A> {
    type Out = Gt;
}
impl<A: Cmp<B>, B> Cmp<O<B>> for O<A> {
    type Out = Ord2<A, B>;
}
impl<A: Cmp<B>, B> Cmp<E<B>> for E<A> {
    type Out = Ord2<A, B>;
}
impl<A: Cmp<B>, B> Cmp<E<B>> for O<A>
where
    Ord2<A, B>: OrElse<Gt>,
{
    type Out = <Ord2<A, B> as OrElse<Gt>>::Out;
}
impl<A: Cmp<B>, B> Cmp<O<B>> for E<A>
where
    Ord2<A, B>: OrElse<Lt>,
{
    type Out = <Ord2<A, B> as OrElse<Lt>>::Out;
}

// ceiling halve, three times, gives ceil(n/8) with no arithmetic anywhere
pub trait CeilHalf {
    type Out;
}
pub type CH<A> = <A as CeilHalf>::Out;
impl CeilHalf for Z {
    type Out = Z;
}
impl<A: AddC<Z>> CeilHalf for O<A> {
    type Out = SumC<A, Z>;
}
impl<A> CeilHalf for E<A> {
    type Out = A;
}

pub type Bytes<W> = CH<CH<CH<W>>>;

// an exact-size byte buffer built from the nat's binary structure, log depth
pub struct Nil;
#[repr(C)]
pub struct One<T>(u8, T, T);
#[repr(C)]
pub struct Two<T>(T, T);

pub trait Buf {
    type Out;
}
pub type BufOf<A> = <A as Buf>::Out;
impl Buf for Z {
    type Out = Nil;
}
impl<A: Buf> Buf for O<A> {
    type Out = One<BufOf<A>>;
}
impl<A: Buf> Buf for E<A> {
    type Out = Two<BufOf<A>>;
}

// container selection: five native rungs then the wide rung. Finite impls.
pub type Rung<W, Bound, Narrow, Wide> = <Ord2<W, Bound> as IfLe<Narrow, Wide>>::Out;

// literals 0..=130, machine generated
pub type N0 = Z;
pub type N1 = O<Z>;
pub type N2 = E<O<Z>>;
pub type N3 = O<O<Z>>;
pub type N4 = E<E<O<Z>>>;
pub type N5 = O<E<O<Z>>>;
pub type N6 = E<O<O<Z>>>;
pub type N7 = O<O<O<Z>>>;
pub type N8 = E<E<E<O<Z>>>>;
pub type N9 = O<E<E<O<Z>>>>;
pub type N10 = E<O<E<O<Z>>>>;
pub type N11 = O<O<E<O<Z>>>>;
pub type N12 = E<E<O<O<Z>>>>;
pub type N13 = O<E<O<O<Z>>>>;
pub type N14 = E<O<O<O<Z>>>>;
pub type N15 = O<O<O<O<Z>>>>;
pub type N16 = E<E<E<E<O<Z>>>>>;
pub type N17 = O<E<E<E<O<Z>>>>>;
pub type N18 = E<O<E<E<O<Z>>>>>;
pub type N19 = O<O<E<E<O<Z>>>>>;
pub type N20 = E<E<O<E<O<Z>>>>>;
pub type N21 = O<E<O<E<O<Z>>>>>;
pub type N22 = E<O<O<E<O<Z>>>>>;
pub type N23 = O<O<O<E<O<Z>>>>>;
pub type N24 = E<E<E<O<O<Z>>>>>;
pub type N25 = O<E<E<O<O<Z>>>>>;
pub type N26 = E<O<E<O<O<Z>>>>>;
pub type N27 = O<O<E<O<O<Z>>>>>;
pub type N28 = E<E<O<O<O<Z>>>>>;
pub type N29 = O<E<O<O<O<Z>>>>>;
pub type N30 = E<O<O<O<O<Z>>>>>;
pub type N31 = O<O<O<O<O<Z>>>>>;
pub type N32 = E<E<E<E<E<O<Z>>>>>>;
pub type N33 = O<E<E<E<E<O<Z>>>>>>;
pub type N34 = E<O<E<E<E<O<Z>>>>>>;
pub type N35 = O<O<E<E<E<O<Z>>>>>>;
pub type N36 = E<E<O<E<E<O<Z>>>>>>;
pub type N37 = O<E<O<E<E<O<Z>>>>>>;
pub type N38 = E<O<O<E<E<O<Z>>>>>>;
pub type N39 = O<O<O<E<E<O<Z>>>>>>;
pub type N40 = E<E<E<O<E<O<Z>>>>>>;
pub type N41 = O<E<E<O<E<O<Z>>>>>>;
pub type N42 = E<O<E<O<E<O<Z>>>>>>;
pub type N43 = O<O<E<O<E<O<Z>>>>>>;
pub type N44 = E<E<O<O<E<O<Z>>>>>>;
pub type N45 = O<E<O<O<E<O<Z>>>>>>;
pub type N46 = E<O<O<O<E<O<Z>>>>>>;
pub type N47 = O<O<O<O<E<O<Z>>>>>>;
pub type N48 = E<E<E<E<O<O<Z>>>>>>;
pub type N49 = O<E<E<E<O<O<Z>>>>>>;
pub type N50 = E<O<E<E<O<O<Z>>>>>>;
pub type N51 = O<O<E<E<O<O<Z>>>>>>;
pub type N52 = E<E<O<E<O<O<Z>>>>>>;
pub type N53 = O<E<O<E<O<O<Z>>>>>>;
pub type N54 = E<O<O<E<O<O<Z>>>>>>;
pub type N55 = O<O<O<E<O<O<Z>>>>>>;
pub type N56 = E<E<E<O<O<O<Z>>>>>>;
pub type N57 = O<E<E<O<O<O<Z>>>>>>;
pub type N58 = E<O<E<O<O<O<Z>>>>>>;
pub type N59 = O<O<E<O<O<O<Z>>>>>>;
pub type N60 = E<E<O<O<O<O<Z>>>>>>;
pub type N61 = O<E<O<O<O<O<Z>>>>>>;
pub type N62 = E<O<O<O<O<O<Z>>>>>>;
pub type N63 = O<O<O<O<O<O<Z>>>>>>;
pub type N64 = E<E<E<E<E<E<O<Z>>>>>>>;
pub type N65 = O<E<E<E<E<E<O<Z>>>>>>>;
pub type N66 = E<O<E<E<E<E<O<Z>>>>>>>;
pub type N67 = O<O<E<E<E<E<O<Z>>>>>>>;
pub type N68 = E<E<O<E<E<E<O<Z>>>>>>>;
pub type N69 = O<E<O<E<E<E<O<Z>>>>>>>;
pub type N70 = E<O<O<E<E<E<O<Z>>>>>>>;
pub type N71 = O<O<O<E<E<E<O<Z>>>>>>>;
pub type N72 = E<E<E<O<E<E<O<Z>>>>>>>;
pub type N73 = O<E<E<O<E<E<O<Z>>>>>>>;
pub type N74 = E<O<E<O<E<E<O<Z>>>>>>>;
pub type N75 = O<O<E<O<E<E<O<Z>>>>>>>;
pub type N76 = E<E<O<O<E<E<O<Z>>>>>>>;
pub type N77 = O<E<O<O<E<E<O<Z>>>>>>>;
pub type N78 = E<O<O<O<E<E<O<Z>>>>>>>;
pub type N79 = O<O<O<O<E<E<O<Z>>>>>>>;
pub type N80 = E<E<E<E<O<E<O<Z>>>>>>>;
pub type N81 = O<E<E<E<O<E<O<Z>>>>>>>;
pub type N82 = E<O<E<E<O<E<O<Z>>>>>>>;
pub type N83 = O<O<E<E<O<E<O<Z>>>>>>>;
pub type N84 = E<E<O<E<O<E<O<Z>>>>>>>;
pub type N85 = O<E<O<E<O<E<O<Z>>>>>>>;
pub type N86 = E<O<O<E<O<E<O<Z>>>>>>>;
pub type N87 = O<O<O<E<O<E<O<Z>>>>>>>;
pub type N88 = E<E<E<O<O<E<O<Z>>>>>>>;
pub type N89 = O<E<E<O<O<E<O<Z>>>>>>>;
pub type N90 = E<O<E<O<O<E<O<Z>>>>>>>;
pub type N91 = O<O<E<O<O<E<O<Z>>>>>>>;
pub type N92 = E<E<O<O<O<E<O<Z>>>>>>>;
pub type N93 = O<E<O<O<O<E<O<Z>>>>>>>;
pub type N94 = E<O<O<O<O<E<O<Z>>>>>>>;
pub type N95 = O<O<O<O<O<E<O<Z>>>>>>>;
pub type N96 = E<E<E<E<E<O<O<Z>>>>>>>;
pub type N97 = O<E<E<E<E<O<O<Z>>>>>>>;
pub type N98 = E<O<E<E<E<O<O<Z>>>>>>>;
pub type N99 = O<O<E<E<E<O<O<Z>>>>>>>;
pub type N100 = E<E<O<E<E<O<O<Z>>>>>>>;
pub type N101 = O<E<O<E<E<O<O<Z>>>>>>>;
pub type N102 = E<O<O<E<E<O<O<Z>>>>>>>;
pub type N103 = O<O<O<E<E<O<O<Z>>>>>>>;
pub type N104 = E<E<E<O<E<O<O<Z>>>>>>>;
pub type N105 = O<E<E<O<E<O<O<Z>>>>>>>;
pub type N106 = E<O<E<O<E<O<O<Z>>>>>>>;
pub type N107 = O<O<E<O<E<O<O<Z>>>>>>>;
pub type N108 = E<E<O<O<E<O<O<Z>>>>>>>;
pub type N109 = O<E<O<O<E<O<O<Z>>>>>>>;
pub type N110 = E<O<O<O<E<O<O<Z>>>>>>>;
pub type N111 = O<O<O<O<E<O<O<Z>>>>>>>;
pub type N112 = E<E<E<E<O<O<O<Z>>>>>>>;
pub type N113 = O<E<E<E<O<O<O<Z>>>>>>>;
pub type N114 = E<O<E<E<O<O<O<Z>>>>>>>;
pub type N115 = O<O<E<E<O<O<O<Z>>>>>>>;
pub type N116 = E<E<O<E<O<O<O<Z>>>>>>>;
pub type N117 = O<E<O<E<O<O<O<Z>>>>>>>;
pub type N118 = E<O<O<E<O<O<O<Z>>>>>>>;
pub type N119 = O<O<O<E<O<O<O<Z>>>>>>>;
pub type N120 = E<E<E<O<O<O<O<Z>>>>>>>;
pub type N121 = O<E<E<O<O<O<O<Z>>>>>>>;
pub type N122 = E<O<E<O<O<O<O<Z>>>>>>>;
pub type N123 = O<O<E<O<O<O<O<Z>>>>>>>;
pub type N124 = E<E<O<O<O<O<O<Z>>>>>>>;
pub type N125 = O<E<O<O<O<O<O<Z>>>>>>>;
pub type N126 = E<O<O<O<O<O<O<Z>>>>>>>;
pub type N127 = O<O<O<O<O<O<O<Z>>>>>>>;
pub type N128 = E<E<E<E<E<E<E<O<Z>>>>>>>>;
pub type N129 = O<E<E<E<E<E<E<O<Z>>>>>>>>;
pub type N130 = E<O<E<E<E<E<E<O<Z>>>>>>>>;

// container: five native rungs then the wide rung, expressed once
pub type Container<W> = Rung<
    W,
    N8,
    u8,
    Rung<W, N16, u16, Rung<W, N32, u32, Rung<W, N64, u64, Rung<W, N128, u128, BufOf<Bytes<W>>>>>>,
>;

pub trait Same<T> {}
impl<T> Same<T> for T {}

pub fn size_of_buf<W>() -> usize
where
    W: CeilHalf,
    CH<W>: CeilHalf,
    CH<CH<W>>: CeilHalf,
    Bytes<W>: Buf,
{
    core::mem::size_of::<BufOf<Bytes<W>>>()
}
fn _bytes_matrix()
where
    Bytes<N0>: Same<N0>,
    Bytes<N1>: Same<N1>,
    Bytes<N2>: Same<N1>,
    Bytes<N3>: Same<N1>,
    Bytes<N4>: Same<N1>,
    Bytes<N5>: Same<N1>,
    Bytes<N6>: Same<N1>,
    Bytes<N7>: Same<N1>,
    Bytes<N8>: Same<N1>,
    Bytes<N9>: Same<N2>,
    Bytes<N10>: Same<N2>,
    Bytes<N11>: Same<N2>,
    Bytes<N12>: Same<N2>,
    Bytes<N13>: Same<N2>,
    Bytes<N14>: Same<N2>,
    Bytes<N15>: Same<N2>,
    Bytes<N16>: Same<N2>,
    Bytes<N17>: Same<N3>,
    Bytes<N18>: Same<N3>,
    Bytes<N19>: Same<N3>,
    Bytes<N20>: Same<N3>,
    Bytes<N21>: Same<N3>,
    Bytes<N22>: Same<N3>,
    Bytes<N23>: Same<N3>,
    Bytes<N24>: Same<N3>,
    Bytes<N25>: Same<N4>,
    Bytes<N26>: Same<N4>,
    Bytes<N27>: Same<N4>,
    Bytes<N28>: Same<N4>,
    Bytes<N29>: Same<N4>,
    Bytes<N30>: Same<N4>,
    Bytes<N31>: Same<N4>,
    Bytes<N32>: Same<N4>,
    Bytes<N33>: Same<N5>,
    Bytes<N34>: Same<N5>,
    Bytes<N35>: Same<N5>,
    Bytes<N36>: Same<N5>,
    Bytes<N37>: Same<N5>,
    Bytes<N38>: Same<N5>,
    Bytes<N39>: Same<N5>,
    Bytes<N40>: Same<N5>,
    Bytes<N41>: Same<N6>,
    Bytes<N42>: Same<N6>,
    Bytes<N43>: Same<N6>,
    Bytes<N44>: Same<N6>,
    Bytes<N45>: Same<N6>,
    Bytes<N46>: Same<N6>,
    Bytes<N47>: Same<N6>,
    Bytes<N48>: Same<N6>,
    Bytes<N49>: Same<N7>,
    Bytes<N50>: Same<N7>,
    Bytes<N51>: Same<N7>,
    Bytes<N52>: Same<N7>,
    Bytes<N53>: Same<N7>,
    Bytes<N54>: Same<N7>,
    Bytes<N55>: Same<N7>,
    Bytes<N56>: Same<N7>,
    Bytes<N57>: Same<N8>,
    Bytes<N58>: Same<N8>,
    Bytes<N59>: Same<N8>,
    Bytes<N60>: Same<N8>,
    Bytes<N61>: Same<N8>,
    Bytes<N62>: Same<N8>,
    Bytes<N63>: Same<N8>,
    Bytes<N64>: Same<N8>,
    Bytes<N65>: Same<N9>,
    Bytes<N66>: Same<N9>,
    Bytes<N67>: Same<N9>,
    Bytes<N68>: Same<N9>,
    Bytes<N69>: Same<N9>,
    Bytes<N70>: Same<N9>,
    Bytes<N71>: Same<N9>,
    Bytes<N72>: Same<N9>,
    Bytes<N73>: Same<N10>,
    Bytes<N74>: Same<N10>,
    Bytes<N75>: Same<N10>,
    Bytes<N76>: Same<N10>,
    Bytes<N77>: Same<N10>,
    Bytes<N78>: Same<N10>,
    Bytes<N79>: Same<N10>,
    Bytes<N80>: Same<N10>,
    Bytes<N81>: Same<N11>,
    Bytes<N82>: Same<N11>,
    Bytes<N83>: Same<N11>,
    Bytes<N84>: Same<N11>,
    Bytes<N85>: Same<N11>,
    Bytes<N86>: Same<N11>,
    Bytes<N87>: Same<N11>,
    Bytes<N88>: Same<N11>,
    Bytes<N89>: Same<N12>,
    Bytes<N90>: Same<N12>,
    Bytes<N91>: Same<N12>,
    Bytes<N92>: Same<N12>,
    Bytes<N93>: Same<N12>,
    Bytes<N94>: Same<N12>,
    Bytes<N95>: Same<N12>,
    Bytes<N96>: Same<N12>,
    Bytes<N97>: Same<N13>,
    Bytes<N98>: Same<N13>,
    Bytes<N99>: Same<N13>,
    Bytes<N100>: Same<N13>,
    Bytes<N101>: Same<N13>,
    Bytes<N102>: Same<N13>,
    Bytes<N103>: Same<N13>,
    Bytes<N104>: Same<N13>,
    Bytes<N105>: Same<N14>,
    Bytes<N106>: Same<N14>,
    Bytes<N107>: Same<N14>,
    Bytes<N108>: Same<N14>,
    Bytes<N109>: Same<N14>,
    Bytes<N110>: Same<N14>,
    Bytes<N111>: Same<N14>,
    Bytes<N112>: Same<N14>,
    Bytes<N113>: Same<N15>,
    Bytes<N114>: Same<N15>,
    Bytes<N115>: Same<N15>,
    Bytes<N116>: Same<N15>,
    Bytes<N117>: Same<N15>,
    Bytes<N118>: Same<N15>,
    Bytes<N119>: Same<N15>,
    Bytes<N120>: Same<N15>,
    Bytes<N121>: Same<N16>,
    Bytes<N122>: Same<N16>,
    Bytes<N123>: Same<N16>,
    Bytes<N124>: Same<N16>,
    Bytes<N125>: Same<N16>,
    Bytes<N126>: Same<N16>,
    Bytes<N127>: Same<N16>,
    Bytes<N128>: Same<N16>,
    Bytes<N129>: Same<N17>,
    Bytes<N130>: Same<N17>,
{
}
fn _container_matrix()
where
    Container<N0>: Same<u8>,
    Container<N1>: Same<u8>,
    Container<N2>: Same<u8>,
    Container<N3>: Same<u8>,
    Container<N4>: Same<u8>,
    Container<N5>: Same<u8>,
    Container<N6>: Same<u8>,
    Container<N7>: Same<u8>,
    Container<N8>: Same<u8>,
    Container<N9>: Same<u16>,
    Container<N10>: Same<u16>,
    Container<N11>: Same<u16>,
    Container<N12>: Same<u16>,
    Container<N13>: Same<u16>,
    Container<N14>: Same<u16>,
    Container<N15>: Same<u16>,
    Container<N16>: Same<u16>,
    Container<N17>: Same<u32>,
    Container<N18>: Same<u32>,
    Container<N19>: Same<u32>,
    Container<N20>: Same<u32>,
    Container<N21>: Same<u32>,
    Container<N22>: Same<u32>,
    Container<N23>: Same<u32>,
    Container<N24>: Same<u32>,
    Container<N25>: Same<u32>,
    Container<N26>: Same<u32>,
    Container<N27>: Same<u32>,
    Container<N28>: Same<u32>,
    Container<N29>: Same<u32>,
    Container<N30>: Same<u32>,
    Container<N31>: Same<u32>,
    Container<N32>: Same<u32>,
    Container<N33>: Same<u64>,
    Container<N34>: Same<u64>,
    Container<N35>: Same<u64>,
    Container<N36>: Same<u64>,
    Container<N37>: Same<u64>,
    Container<N38>: Same<u64>,
    Container<N39>: Same<u64>,
    Container<N40>: Same<u64>,
    Container<N41>: Same<u64>,
    Container<N42>: Same<u64>,
    Container<N43>: Same<u64>,
    Container<N44>: Same<u64>,
    Container<N45>: Same<u64>,
    Container<N46>: Same<u64>,
    Container<N47>: Same<u64>,
    Container<N48>: Same<u64>,
    Container<N49>: Same<u64>,
    Container<N50>: Same<u64>,
    Container<N51>: Same<u64>,
    Container<N52>: Same<u64>,
    Container<N53>: Same<u64>,
    Container<N54>: Same<u64>,
    Container<N55>: Same<u64>,
    Container<N56>: Same<u64>,
    Container<N57>: Same<u64>,
    Container<N58>: Same<u64>,
    Container<N59>: Same<u64>,
    Container<N60>: Same<u64>,
    Container<N61>: Same<u64>,
    Container<N62>: Same<u64>,
    Container<N63>: Same<u64>,
    Container<N64>: Same<u64>,
    Container<N65>: Same<u128>,
    Container<N66>: Same<u128>,
    Container<N67>: Same<u128>,
    Container<N68>: Same<u128>,
    Container<N69>: Same<u128>,
    Container<N70>: Same<u128>,
    Container<N71>: Same<u128>,
    Container<N72>: Same<u128>,
    Container<N73>: Same<u128>,
    Container<N74>: Same<u128>,
    Container<N75>: Same<u128>,
    Container<N76>: Same<u128>,
    Container<N77>: Same<u128>,
    Container<N78>: Same<u128>,
    Container<N79>: Same<u128>,
    Container<N80>: Same<u128>,
    Container<N81>: Same<u128>,
    Container<N82>: Same<u128>,
    Container<N83>: Same<u128>,
    Container<N84>: Same<u128>,
    Container<N85>: Same<u128>,
    Container<N86>: Same<u128>,
    Container<N87>: Same<u128>,
    Container<N88>: Same<u128>,
    Container<N89>: Same<u128>,
    Container<N90>: Same<u128>,
    Container<N91>: Same<u128>,
    Container<N92>: Same<u128>,
    Container<N93>: Same<u128>,
    Container<N94>: Same<u128>,
    Container<N95>: Same<u128>,
    Container<N96>: Same<u128>,
    Container<N97>: Same<u128>,
    Container<N98>: Same<u128>,
    Container<N99>: Same<u128>,
    Container<N100>: Same<u128>,
    Container<N101>: Same<u128>,
    Container<N102>: Same<u128>,
    Container<N103>: Same<u128>,
    Container<N104>: Same<u128>,
    Container<N105>: Same<u128>,
    Container<N106>: Same<u128>,
    Container<N107>: Same<u128>,
    Container<N108>: Same<u128>,
    Container<N109>: Same<u128>,
    Container<N110>: Same<u128>,
    Container<N111>: Same<u128>,
    Container<N112>: Same<u128>,
    Container<N113>: Same<u128>,
    Container<N114>: Same<u128>,
    Container<N115>: Same<u128>,
    Container<N116>: Same<u128>,
    Container<N117>: Same<u128>,
    Container<N118>: Same<u128>,
    Container<N119>: Same<u128>,
    Container<N120>: Same<u128>,
    Container<N121>: Same<u128>,
    Container<N122>: Same<u128>,
    Container<N123>: Same<u128>,
    Container<N124>: Same<u128>,
    Container<N125>: Same<u128>,
    Container<N126>: Same<u128>,
    Container<N127>: Same<u128>,
    Container<N128>: Same<u128>,
    Container<N129>: Same<BufOf<N17>>,
    Container<N130>: Same<BufOf<N17>>,
{
}

// ---- Arrangement B: the WIDTH TYPE is the consumer's literal, `L<K>`.
// Structural nats stay, but only as a hidden representation. Every operation
// output is mapped back to an `L<K>` through the reverse table, so both
// directions of the table are used and the ceiling becomes real.
pub struct L<const K: u32>;

pub trait Named {
    type Out;
} // structural nat -> L<K>
pub trait Repr {
    type R;
} // L<K> -> structural nat
pub type ReprOf<W> = <W as Repr>::R;
pub type NameOf<N> = <N as Named>::Out;

pub struct Warm;
pub struct Unsigned;

pub struct Pair<I, F>(I, F);
pub trait Shape {
    type Cont;
}

impl<I: Repr, F: Repr> Shape for Pair<I, F>
where
    ReprOf<I>: Add<ReprOf<F>>,
    Sum<ReprOf<I>, ReprOf<F>>: Cmp<N8> + Cmp<N16> + Cmp<N32> + Cmp<N64> + Cmp<N128>,
    Sum<ReprOf<I>, ReprOf<F>>: CeilHalf,
    CH<Sum<ReprOf<I>, ReprOf<F>>>: CeilHalf,
    CH<CH<Sum<ReprOf<I>, ReprOf<F>>>>: CeilHalf,
    Bytes<Sum<ReprOf<I>, ReprOf<F>>>: Buf,
    Ord2<Sum<ReprOf<I>, ReprOf<F>>, N128>: IfLe<u128, BufOf<Bytes<Sum<ReprOf<I>, ReprOf<F>>>>>,
    Ord2<Sum<ReprOf<I>, ReprOf<F>>, N64>: IfLe<
        u64,
        Rung<Sum<ReprOf<I>, ReprOf<F>>, N128, u128, BufOf<Bytes<Sum<ReprOf<I>, ReprOf<F>>>>>,
    >,
    Ord2<Sum<ReprOf<I>, ReprOf<F>>, N32>: IfLe<
        u32,
        Rung<
            Sum<ReprOf<I>, ReprOf<F>>,
            N64,
            u64,
            Rung<Sum<ReprOf<I>, ReprOf<F>>, N128, u128, BufOf<Bytes<Sum<ReprOf<I>, ReprOf<F>>>>>,
        >,
    >,
    Ord2<Sum<ReprOf<I>, ReprOf<F>>, N16>: IfLe<
        u16,
        Rung<
            Sum<ReprOf<I>, ReprOf<F>>,
            N32,
            u32,
            Rung<
                Sum<ReprOf<I>, ReprOf<F>>,
                N64,
                u64,
                Rung<
                    Sum<ReprOf<I>, ReprOf<F>>,
                    N128,
                    u128,
                    BufOf<Bytes<Sum<ReprOf<I>, ReprOf<F>>>>,
                >,
            >,
        >,
    >,
    Ord2<Sum<ReprOf<I>, ReprOf<F>>, N8>: IfLe<
        u8,
        Rung<
            Sum<ReprOf<I>, ReprOf<F>>,
            N16,
            u16,
            Rung<
                Sum<ReprOf<I>, ReprOf<F>>,
                N32,
                u32,
                Rung<
                    Sum<ReprOf<I>, ReprOf<F>>,
                    N64,
                    u64,
                    Rung<
                        Sum<ReprOf<I>, ReprOf<F>>,
                        N128,
                        u128,
                        BufOf<Bytes<Sum<ReprOf<I>, ReprOf<F>>>>,
                    >,
                >,
            >,
        >,
    >,
{
    type Cont = Container<Sum<ReprOf<I>, ReprOf<F>>>;
}

pub struct Numeral<I, F, Sign, S>(
    <Pair<I, F> as Shape>::Cont,
    core::marker::PhantomData<(Sign, S)>,
)
where
    Pair<I, F>: Shape;

pub type UFixed<const I: u32, const F: u32, S = Warm> = Numeral<L<I>, L<F>, Unsigned, S>;
pub type UInt<const N: u32, S = Warm> = UFixed<N, 0, S>;

// ---- the table, both directions ----
impl Repr for L<0> {
    type R = N0;
}
impl Repr for L<1> {
    type R = N1;
}
impl Repr for L<2> {
    type R = N2;
}
impl Repr for L<3> {
    type R = N3;
}
impl Repr for L<4> {
    type R = N4;
}
impl Repr for L<5> {
    type R = N5;
}
impl Repr for L<6> {
    type R = N6;
}
impl Repr for L<7> {
    type R = N7;
}
impl Repr for L<8> {
    type R = N8;
}
impl Repr for L<9> {
    type R = N9;
}
impl Repr for L<10> {
    type R = N10;
}
impl Repr for L<11> {
    type R = N11;
}
impl Repr for L<12> {
    type R = N12;
}
impl Repr for L<13> {
    type R = N13;
}
impl Repr for L<14> {
    type R = N14;
}
impl Repr for L<15> {
    type R = N15;
}
impl Repr for L<16> {
    type R = N16;
}
impl Repr for L<17> {
    type R = N17;
}
impl Repr for L<18> {
    type R = N18;
}
impl Repr for L<19> {
    type R = N19;
}
impl Repr for L<20> {
    type R = N20;
}
impl Repr for L<21> {
    type R = N21;
}
impl Repr for L<22> {
    type R = N22;
}
impl Repr for L<23> {
    type R = N23;
}
impl Repr for L<24> {
    type R = N24;
}
impl Repr for L<25> {
    type R = N25;
}
impl Repr for L<26> {
    type R = N26;
}
impl Repr for L<27> {
    type R = N27;
}
impl Repr for L<28> {
    type R = N28;
}
impl Repr for L<29> {
    type R = N29;
}
impl Repr for L<30> {
    type R = N30;
}
impl Repr for L<31> {
    type R = N31;
}
impl Repr for L<32> {
    type R = N32;
}
impl Repr for L<33> {
    type R = N33;
}
impl Repr for L<34> {
    type R = N34;
}
impl Repr for L<35> {
    type R = N35;
}
impl Repr for L<36> {
    type R = N36;
}
impl Repr for L<37> {
    type R = N37;
}
impl Repr for L<38> {
    type R = N38;
}
impl Repr for L<39> {
    type R = N39;
}
impl Repr for L<40> {
    type R = N40;
}
impl Repr for L<41> {
    type R = N41;
}
impl Repr for L<42> {
    type R = N42;
}
impl Repr for L<43> {
    type R = N43;
}
impl Repr for L<44> {
    type R = N44;
}
impl Repr for L<45> {
    type R = N45;
}
impl Repr for L<46> {
    type R = N46;
}
impl Repr for L<47> {
    type R = N47;
}
impl Repr for L<48> {
    type R = N48;
}
impl Repr for L<49> {
    type R = N49;
}
impl Repr for L<50> {
    type R = N50;
}
impl Repr for L<51> {
    type R = N51;
}
impl Repr for L<52> {
    type R = N52;
}
impl Repr for L<53> {
    type R = N53;
}
impl Repr for L<54> {
    type R = N54;
}
impl Repr for L<55> {
    type R = N55;
}
impl Repr for L<56> {
    type R = N56;
}
impl Repr for L<57> {
    type R = N57;
}
impl Repr for L<58> {
    type R = N58;
}
impl Repr for L<59> {
    type R = N59;
}
impl Repr for L<60> {
    type R = N60;
}
impl Repr for L<61> {
    type R = N61;
}
impl Repr for L<62> {
    type R = N62;
}
impl Repr for L<63> {
    type R = N63;
}
impl Repr for L<64> {
    type R = N64;
}
impl Named for Z {
    type Out = L<0>;
}
impl Named for O<Z> {
    type Out = L<1>;
}
impl Named for E<O<Z>> {
    type Out = L<2>;
}
impl Named for O<O<Z>> {
    type Out = L<3>;
}
impl Named for E<E<O<Z>>> {
    type Out = L<4>;
}
impl Named for O<E<O<Z>>> {
    type Out = L<5>;
}
impl Named for E<O<O<Z>>> {
    type Out = L<6>;
}
impl Named for O<O<O<Z>>> {
    type Out = L<7>;
}
impl Named for E<E<E<O<Z>>>> {
    type Out = L<8>;
}
impl Named for O<E<E<O<Z>>>> {
    type Out = L<9>;
}
impl Named for E<O<E<O<Z>>>> {
    type Out = L<10>;
}
impl Named for O<O<E<O<Z>>>> {
    type Out = L<11>;
}
impl Named for E<E<O<O<Z>>>> {
    type Out = L<12>;
}
impl Named for O<E<O<O<Z>>>> {
    type Out = L<13>;
}
impl Named for E<O<O<O<Z>>>> {
    type Out = L<14>;
}
impl Named for O<O<O<O<Z>>>> {
    type Out = L<15>;
}
impl Named for E<E<E<E<O<Z>>>>> {
    type Out = L<16>;
}
impl Named for O<E<E<E<O<Z>>>>> {
    type Out = L<17>;
}
impl Named for E<O<E<E<O<Z>>>>> {
    type Out = L<18>;
}
impl Named for O<O<E<E<O<Z>>>>> {
    type Out = L<19>;
}
impl Named for E<E<O<E<O<Z>>>>> {
    type Out = L<20>;
}
impl Named for O<E<O<E<O<Z>>>>> {
    type Out = L<21>;
}
impl Named for E<O<O<E<O<Z>>>>> {
    type Out = L<22>;
}
impl Named for O<O<O<E<O<Z>>>>> {
    type Out = L<23>;
}
impl Named for E<E<E<O<O<Z>>>>> {
    type Out = L<24>;
}
impl Named for O<E<E<O<O<Z>>>>> {
    type Out = L<25>;
}
impl Named for E<O<E<O<O<Z>>>>> {
    type Out = L<26>;
}
impl Named for O<O<E<O<O<Z>>>>> {
    type Out = L<27>;
}
impl Named for E<E<O<O<O<Z>>>>> {
    type Out = L<28>;
}
impl Named for O<E<O<O<O<Z>>>>> {
    type Out = L<29>;
}
impl Named for E<O<O<O<O<Z>>>>> {
    type Out = L<30>;
}
impl Named for O<O<O<O<O<Z>>>>> {
    type Out = L<31>;
}
impl Named for E<E<E<E<E<O<Z>>>>>> {
    type Out = L<32>;
}
impl Named for O<E<E<E<E<O<Z>>>>>> {
    type Out = L<33>;
}
impl Named for E<O<E<E<E<O<Z>>>>>> {
    type Out = L<34>;
}
impl Named for O<O<E<E<E<O<Z>>>>>> {
    type Out = L<35>;
}
impl Named for E<E<O<E<E<O<Z>>>>>> {
    type Out = L<36>;
}
impl Named for O<E<O<E<E<O<Z>>>>>> {
    type Out = L<37>;
}
impl Named for E<O<O<E<E<O<Z>>>>>> {
    type Out = L<38>;
}
impl Named for O<O<O<E<E<O<Z>>>>>> {
    type Out = L<39>;
}
impl Named for E<E<E<O<E<O<Z>>>>>> {
    type Out = L<40>;
}
impl Named for O<E<E<O<E<O<Z>>>>>> {
    type Out = L<41>;
}
impl Named for E<O<E<O<E<O<Z>>>>>> {
    type Out = L<42>;
}
impl Named for O<O<E<O<E<O<Z>>>>>> {
    type Out = L<43>;
}
impl Named for E<E<O<O<E<O<Z>>>>>> {
    type Out = L<44>;
}
impl Named for O<E<O<O<E<O<Z>>>>>> {
    type Out = L<45>;
}
impl Named for E<O<O<O<E<O<Z>>>>>> {
    type Out = L<46>;
}
impl Named for O<O<O<O<E<O<Z>>>>>> {
    type Out = L<47>;
}
impl Named for E<E<E<E<O<O<Z>>>>>> {
    type Out = L<48>;
}
impl Named for O<E<E<E<O<O<Z>>>>>> {
    type Out = L<49>;
}
impl Named for E<O<E<E<O<O<Z>>>>>> {
    type Out = L<50>;
}
impl Named for O<O<E<E<O<O<Z>>>>>> {
    type Out = L<51>;
}
impl Named for E<E<O<E<O<O<Z>>>>>> {
    type Out = L<52>;
}
impl Named for O<E<O<E<O<O<Z>>>>>> {
    type Out = L<53>;
}
impl Named for E<O<O<E<O<O<Z>>>>>> {
    type Out = L<54>;
}
impl Named for O<O<O<E<O<O<Z>>>>>> {
    type Out = L<55>;
}
impl Named for E<E<E<O<O<O<Z>>>>>> {
    type Out = L<56>;
}
impl Named for O<E<E<O<O<O<Z>>>>>> {
    type Out = L<57>;
}
impl Named for E<O<E<O<O<O<Z>>>>>> {
    type Out = L<58>;
}
impl Named for O<O<E<O<O<O<Z>>>>>> {
    type Out = L<59>;
}
impl Named for E<E<O<O<O<O<Z>>>>>> {
    type Out = L<60>;
}
impl Named for O<E<O<O<O<O<Z>>>>>> {
    type Out = L<61>;
}
impl Named for E<O<O<O<O<O<Z>>>>>> {
    type Out = L<62>;
}
impl Named for O<O<O<O<O<O<Z>>>>>> {
    type Out = L<63>;
}
impl Named for E<E<E<E<E<E<O<Z>>>>>>> {
    type Out = L<64>;
}

pub type Money = UFixed<13, 3>;
pub type StrHandle = UInt<5>;

// multiply: the output must be mapped BACK through the table to be an L<K>
pub trait MulShape<R> {
    type Out;
}
impl<I1: Repr, F1: Repr, I2: Repr, F2: Repr, Sign, S> MulShape<Numeral<I2, F2, Sign, S>>
    for Numeral<I1, F1, Sign, S>
where
    Pair<I1, F1>: Shape,
    Pair<I2, F2>: Shape,
    ReprOf<I1>: Add<ReprOf<I2>>,
    ReprOf<F1>: Add<ReprOf<F2>>,
    Sum<ReprOf<I1>, ReprOf<I2>>: Named,
    Sum<ReprOf<F1>, ReprOf<F2>>: Named,
    Pair<NameOf<Sum<ReprOf<I1>, ReprOf<I2>>>, NameOf<Sum<ReprOf<F1>, ReprOf<F2>>>>: Shape,
{
    type Out =
        Numeral<NameOf<Sum<ReprOf<I1>, ReprOf<I2>>>, NameOf<Sum<ReprOf<F1>, ReprOf<F2>>>, Sign, S>;
}
pub type Prod<A, B> = <A as MulShape<B>>::Out;

fn _one_multiply()
where
    Prod<Money, Money>: Same<UFixed<26, 6>>,
{
}

pub trait IsLe {}
impl IsLe for Lt {}
impl IsLe for Eqq {}

pub trait MulInto<R, Out> {}
impl<I1: Repr, F1: Repr, I2: Repr, F2: Repr, OI: Repr, OF: Repr, Sign, S>
    MulInto<Numeral<I2, F2, Sign, S>, Numeral<OI, OF, Sign, S>> for Numeral<I1, F1, Sign, S>
where
    Pair<I1, F1>: Shape,
    Pair<I2, F2>: Shape,
    Pair<OI, OF>: Shape,
    ReprOf<I1>: Add<ReprOf<I2>>,
    ReprOf<F1>: Add<ReprOf<F2>>,
    Sum<ReprOf<I1>, ReprOf<I2>>: Cmp<ReprOf<OI>>,
    Sum<ReprOf<F1>, ReprOf<F2>>: Cmp<ReprOf<OF>>,
    Ord2<Sum<ReprOf<I1>, ReprOf<I2>>, ReprOf<OI>>: IsLe,
    Ord2<Sum<ReprOf<F1>, ReprOf<F2>>, ReprOf<OF>>: IsLe,
{
}

// A consumer needs width 104. Extending the bridge does NOT require spelling a
// digit tower: the row is written as arithmetic over widths the library ships.
impl Repr for L<104> {
    type R = Sum<N64, Sum<N32, N8>>;
}
impl Repr for L<208> {
    type R = Sum<Sum<N64, N64>, Sum<N64, N16>>;
}

pub type Half = UFixed<52, 12>;
fn _declared_at_an_extended_width()
where
    Half: MulInto<Half, UFixed<104, 24>>,
{
}

// and the extension is arithmetically right, checked against the algebra
fn _extension_is_correct()
where
    ReprOf<L<104>>: Same<Sum<N52, N52>>,
    ReprOf<L<208>>: Same<Sum<Sum<N52, N52>, Sum<N52, N52>>>,
{
}
