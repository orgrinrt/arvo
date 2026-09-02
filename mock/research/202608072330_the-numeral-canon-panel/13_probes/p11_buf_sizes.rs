// P9 core. Everything the machinery needs, entirely at the type level, with
// ZERO feature gates: nat, addition, comparison, ceil-to-bytes, an exact-size
// byte buffer, and container selection across the native rungs plus the wide
// rung. The only const in the whole design is the literal a consumer writes.
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

// runtime confirmation that the type-level buffer is EXACTLY n bytes, align 1
fn main() {
    let mut bad = 0usize;
    if core::mem::size_of::<BufOf<N0>>() != 0 {
        bad += 1;
        println!("size N0 = {}", core::mem::size_of::<BufOf<N0>>());
    }
    if core::mem::align_of::<BufOf<N0>>() != 1 {
        bad += 1;
        println!("align N0 = {}", core::mem::align_of::<BufOf<N0>>());
    }
    if core::mem::size_of::<BufOf<N1>>() != 1 {
        bad += 1;
        println!("size N1 = {}", core::mem::size_of::<BufOf<N1>>());
    }
    if core::mem::align_of::<BufOf<N1>>() != 1 {
        bad += 1;
        println!("align N1 = {}", core::mem::align_of::<BufOf<N1>>());
    }
    if core::mem::size_of::<BufOf<N2>>() != 2 {
        bad += 1;
        println!("size N2 = {}", core::mem::size_of::<BufOf<N2>>());
    }
    if core::mem::align_of::<BufOf<N2>>() != 1 {
        bad += 1;
        println!("align N2 = {}", core::mem::align_of::<BufOf<N2>>());
    }
    if core::mem::size_of::<BufOf<N3>>() != 3 {
        bad += 1;
        println!("size N3 = {}", core::mem::size_of::<BufOf<N3>>());
    }
    if core::mem::align_of::<BufOf<N3>>() != 1 {
        bad += 1;
        println!("align N3 = {}", core::mem::align_of::<BufOf<N3>>());
    }
    if core::mem::size_of::<BufOf<N4>>() != 4 {
        bad += 1;
        println!("size N4 = {}", core::mem::size_of::<BufOf<N4>>());
    }
    if core::mem::align_of::<BufOf<N4>>() != 1 {
        bad += 1;
        println!("align N4 = {}", core::mem::align_of::<BufOf<N4>>());
    }
    if core::mem::size_of::<BufOf<N5>>() != 5 {
        bad += 1;
        println!("size N5 = {}", core::mem::size_of::<BufOf<N5>>());
    }
    if core::mem::align_of::<BufOf<N5>>() != 1 {
        bad += 1;
        println!("align N5 = {}", core::mem::align_of::<BufOf<N5>>());
    }
    if core::mem::size_of::<BufOf<N6>>() != 6 {
        bad += 1;
        println!("size N6 = {}", core::mem::size_of::<BufOf<N6>>());
    }
    if core::mem::align_of::<BufOf<N6>>() != 1 {
        bad += 1;
        println!("align N6 = {}", core::mem::align_of::<BufOf<N6>>());
    }
    if core::mem::size_of::<BufOf<N7>>() != 7 {
        bad += 1;
        println!("size N7 = {}", core::mem::size_of::<BufOf<N7>>());
    }
    if core::mem::align_of::<BufOf<N7>>() != 1 {
        bad += 1;
        println!("align N7 = {}", core::mem::align_of::<BufOf<N7>>());
    }
    if core::mem::size_of::<BufOf<N8>>() != 8 {
        bad += 1;
        println!("size N8 = {}", core::mem::size_of::<BufOf<N8>>());
    }
    if core::mem::align_of::<BufOf<N8>>() != 1 {
        bad += 1;
        println!("align N8 = {}", core::mem::align_of::<BufOf<N8>>());
    }
    if core::mem::size_of::<BufOf<N9>>() != 9 {
        bad += 1;
        println!("size N9 = {}", core::mem::size_of::<BufOf<N9>>());
    }
    if core::mem::align_of::<BufOf<N9>>() != 1 {
        bad += 1;
        println!("align N9 = {}", core::mem::align_of::<BufOf<N9>>());
    }
    if core::mem::size_of::<BufOf<N10>>() != 10 {
        bad += 1;
        println!("size N10 = {}", core::mem::size_of::<BufOf<N10>>());
    }
    if core::mem::align_of::<BufOf<N10>>() != 1 {
        bad += 1;
        println!("align N10 = {}", core::mem::align_of::<BufOf<N10>>());
    }
    if core::mem::size_of::<BufOf<N11>>() != 11 {
        bad += 1;
        println!("size N11 = {}", core::mem::size_of::<BufOf<N11>>());
    }
    if core::mem::align_of::<BufOf<N11>>() != 1 {
        bad += 1;
        println!("align N11 = {}", core::mem::align_of::<BufOf<N11>>());
    }
    if core::mem::size_of::<BufOf<N12>>() != 12 {
        bad += 1;
        println!("size N12 = {}", core::mem::size_of::<BufOf<N12>>());
    }
    if core::mem::align_of::<BufOf<N12>>() != 1 {
        bad += 1;
        println!("align N12 = {}", core::mem::align_of::<BufOf<N12>>());
    }
    if core::mem::size_of::<BufOf<N13>>() != 13 {
        bad += 1;
        println!("size N13 = {}", core::mem::size_of::<BufOf<N13>>());
    }
    if core::mem::align_of::<BufOf<N13>>() != 1 {
        bad += 1;
        println!("align N13 = {}", core::mem::align_of::<BufOf<N13>>());
    }
    if core::mem::size_of::<BufOf<N14>>() != 14 {
        bad += 1;
        println!("size N14 = {}", core::mem::size_of::<BufOf<N14>>());
    }
    if core::mem::align_of::<BufOf<N14>>() != 1 {
        bad += 1;
        println!("align N14 = {}", core::mem::align_of::<BufOf<N14>>());
    }
    if core::mem::size_of::<BufOf<N15>>() != 15 {
        bad += 1;
        println!("size N15 = {}", core::mem::size_of::<BufOf<N15>>());
    }
    if core::mem::align_of::<BufOf<N15>>() != 1 {
        bad += 1;
        println!("align N15 = {}", core::mem::align_of::<BufOf<N15>>());
    }
    if core::mem::size_of::<BufOf<N16>>() != 16 {
        bad += 1;
        println!("size N16 = {}", core::mem::size_of::<BufOf<N16>>());
    }
    if core::mem::align_of::<BufOf<N16>>() != 1 {
        bad += 1;
        println!("align N16 = {}", core::mem::align_of::<BufOf<N16>>());
    }
    if core::mem::size_of::<BufOf<N17>>() != 17 {
        bad += 1;
        println!("size N17 = {}", core::mem::size_of::<BufOf<N17>>());
    }
    if core::mem::align_of::<BufOf<N17>>() != 1 {
        bad += 1;
        println!("align N17 = {}", core::mem::align_of::<BufOf<N17>>());
    }
    if core::mem::size_of::<BufOf<N18>>() != 18 {
        bad += 1;
        println!("size N18 = {}", core::mem::size_of::<BufOf<N18>>());
    }
    if core::mem::align_of::<BufOf<N18>>() != 1 {
        bad += 1;
        println!("align N18 = {}", core::mem::align_of::<BufOf<N18>>());
    }
    if core::mem::size_of::<BufOf<N19>>() != 19 {
        bad += 1;
        println!("size N19 = {}", core::mem::size_of::<BufOf<N19>>());
    }
    if core::mem::align_of::<BufOf<N19>>() != 1 {
        bad += 1;
        println!("align N19 = {}", core::mem::align_of::<BufOf<N19>>());
    }
    if core::mem::size_of::<BufOf<N20>>() != 20 {
        bad += 1;
        println!("size N20 = {}", core::mem::size_of::<BufOf<N20>>());
    }
    if core::mem::align_of::<BufOf<N20>>() != 1 {
        bad += 1;
        println!("align N20 = {}", core::mem::align_of::<BufOf<N20>>());
    }
    if core::mem::size_of::<BufOf<N21>>() != 21 {
        bad += 1;
        println!("size N21 = {}", core::mem::size_of::<BufOf<N21>>());
    }
    if core::mem::align_of::<BufOf<N21>>() != 1 {
        bad += 1;
        println!("align N21 = {}", core::mem::align_of::<BufOf<N21>>());
    }
    if core::mem::size_of::<BufOf<N22>>() != 22 {
        bad += 1;
        println!("size N22 = {}", core::mem::size_of::<BufOf<N22>>());
    }
    if core::mem::align_of::<BufOf<N22>>() != 1 {
        bad += 1;
        println!("align N22 = {}", core::mem::align_of::<BufOf<N22>>());
    }
    if core::mem::size_of::<BufOf<N23>>() != 23 {
        bad += 1;
        println!("size N23 = {}", core::mem::size_of::<BufOf<N23>>());
    }
    if core::mem::align_of::<BufOf<N23>>() != 1 {
        bad += 1;
        println!("align N23 = {}", core::mem::align_of::<BufOf<N23>>());
    }
    if core::mem::size_of::<BufOf<N24>>() != 24 {
        bad += 1;
        println!("size N24 = {}", core::mem::size_of::<BufOf<N24>>());
    }
    if core::mem::align_of::<BufOf<N24>>() != 1 {
        bad += 1;
        println!("align N24 = {}", core::mem::align_of::<BufOf<N24>>());
    }
    if core::mem::size_of::<BufOf<N25>>() != 25 {
        bad += 1;
        println!("size N25 = {}", core::mem::size_of::<BufOf<N25>>());
    }
    if core::mem::align_of::<BufOf<N25>>() != 1 {
        bad += 1;
        println!("align N25 = {}", core::mem::align_of::<BufOf<N25>>());
    }
    if core::mem::size_of::<BufOf<N26>>() != 26 {
        bad += 1;
        println!("size N26 = {}", core::mem::size_of::<BufOf<N26>>());
    }
    if core::mem::align_of::<BufOf<N26>>() != 1 {
        bad += 1;
        println!("align N26 = {}", core::mem::align_of::<BufOf<N26>>());
    }
    if core::mem::size_of::<BufOf<N27>>() != 27 {
        bad += 1;
        println!("size N27 = {}", core::mem::size_of::<BufOf<N27>>());
    }
    if core::mem::align_of::<BufOf<N27>>() != 1 {
        bad += 1;
        println!("align N27 = {}", core::mem::align_of::<BufOf<N27>>());
    }
    if core::mem::size_of::<BufOf<N28>>() != 28 {
        bad += 1;
        println!("size N28 = {}", core::mem::size_of::<BufOf<N28>>());
    }
    if core::mem::align_of::<BufOf<N28>>() != 1 {
        bad += 1;
        println!("align N28 = {}", core::mem::align_of::<BufOf<N28>>());
    }
    if core::mem::size_of::<BufOf<N29>>() != 29 {
        bad += 1;
        println!("size N29 = {}", core::mem::size_of::<BufOf<N29>>());
    }
    if core::mem::align_of::<BufOf<N29>>() != 1 {
        bad += 1;
        println!("align N29 = {}", core::mem::align_of::<BufOf<N29>>());
    }
    if core::mem::size_of::<BufOf<N30>>() != 30 {
        bad += 1;
        println!("size N30 = {}", core::mem::size_of::<BufOf<N30>>());
    }
    if core::mem::align_of::<BufOf<N30>>() != 1 {
        bad += 1;
        println!("align N30 = {}", core::mem::align_of::<BufOf<N30>>());
    }
    if core::mem::size_of::<BufOf<N31>>() != 31 {
        bad += 1;
        println!("size N31 = {}", core::mem::size_of::<BufOf<N31>>());
    }
    if core::mem::align_of::<BufOf<N31>>() != 1 {
        bad += 1;
        println!("align N31 = {}", core::mem::align_of::<BufOf<N31>>());
    }
    if core::mem::size_of::<BufOf<N32>>() != 32 {
        bad += 1;
        println!("size N32 = {}", core::mem::size_of::<BufOf<N32>>());
    }
    if core::mem::align_of::<BufOf<N32>>() != 1 {
        bad += 1;
        println!("align N32 = {}", core::mem::align_of::<BufOf<N32>>());
    }
    if core::mem::size_of::<BufOf<N33>>() != 33 {
        bad += 1;
        println!("size N33 = {}", core::mem::size_of::<BufOf<N33>>());
    }
    if core::mem::align_of::<BufOf<N33>>() != 1 {
        bad += 1;
        println!("align N33 = {}", core::mem::align_of::<BufOf<N33>>());
    }
    if core::mem::size_of::<BufOf<N34>>() != 34 {
        bad += 1;
        println!("size N34 = {}", core::mem::size_of::<BufOf<N34>>());
    }
    if core::mem::align_of::<BufOf<N34>>() != 1 {
        bad += 1;
        println!("align N34 = {}", core::mem::align_of::<BufOf<N34>>());
    }
    if core::mem::size_of::<BufOf<N35>>() != 35 {
        bad += 1;
        println!("size N35 = {}", core::mem::size_of::<BufOf<N35>>());
    }
    if core::mem::align_of::<BufOf<N35>>() != 1 {
        bad += 1;
        println!("align N35 = {}", core::mem::align_of::<BufOf<N35>>());
    }
    if core::mem::size_of::<BufOf<N36>>() != 36 {
        bad += 1;
        println!("size N36 = {}", core::mem::size_of::<BufOf<N36>>());
    }
    if core::mem::align_of::<BufOf<N36>>() != 1 {
        bad += 1;
        println!("align N36 = {}", core::mem::align_of::<BufOf<N36>>());
    }
    if core::mem::size_of::<BufOf<N37>>() != 37 {
        bad += 1;
        println!("size N37 = {}", core::mem::size_of::<BufOf<N37>>());
    }
    if core::mem::align_of::<BufOf<N37>>() != 1 {
        bad += 1;
        println!("align N37 = {}", core::mem::align_of::<BufOf<N37>>());
    }
    if core::mem::size_of::<BufOf<N38>>() != 38 {
        bad += 1;
        println!("size N38 = {}", core::mem::size_of::<BufOf<N38>>());
    }
    if core::mem::align_of::<BufOf<N38>>() != 1 {
        bad += 1;
        println!("align N38 = {}", core::mem::align_of::<BufOf<N38>>());
    }
    if core::mem::size_of::<BufOf<N39>>() != 39 {
        bad += 1;
        println!("size N39 = {}", core::mem::size_of::<BufOf<N39>>());
    }
    if core::mem::align_of::<BufOf<N39>>() != 1 {
        bad += 1;
        println!("align N39 = {}", core::mem::align_of::<BufOf<N39>>());
    }
    if core::mem::size_of::<BufOf<N40>>() != 40 {
        bad += 1;
        println!("size N40 = {}", core::mem::size_of::<BufOf<N40>>());
    }
    if core::mem::align_of::<BufOf<N40>>() != 1 {
        bad += 1;
        println!("align N40 = {}", core::mem::align_of::<BufOf<N40>>());
    }
    if core::mem::size_of::<BufOf<N41>>() != 41 {
        bad += 1;
        println!("size N41 = {}", core::mem::size_of::<BufOf<N41>>());
    }
    if core::mem::align_of::<BufOf<N41>>() != 1 {
        bad += 1;
        println!("align N41 = {}", core::mem::align_of::<BufOf<N41>>());
    }
    if core::mem::size_of::<BufOf<N42>>() != 42 {
        bad += 1;
        println!("size N42 = {}", core::mem::size_of::<BufOf<N42>>());
    }
    if core::mem::align_of::<BufOf<N42>>() != 1 {
        bad += 1;
        println!("align N42 = {}", core::mem::align_of::<BufOf<N42>>());
    }
    if core::mem::size_of::<BufOf<N43>>() != 43 {
        bad += 1;
        println!("size N43 = {}", core::mem::size_of::<BufOf<N43>>());
    }
    if core::mem::align_of::<BufOf<N43>>() != 1 {
        bad += 1;
        println!("align N43 = {}", core::mem::align_of::<BufOf<N43>>());
    }
    if core::mem::size_of::<BufOf<N44>>() != 44 {
        bad += 1;
        println!("size N44 = {}", core::mem::size_of::<BufOf<N44>>());
    }
    if core::mem::align_of::<BufOf<N44>>() != 1 {
        bad += 1;
        println!("align N44 = {}", core::mem::align_of::<BufOf<N44>>());
    }
    if core::mem::size_of::<BufOf<N45>>() != 45 {
        bad += 1;
        println!("size N45 = {}", core::mem::size_of::<BufOf<N45>>());
    }
    if core::mem::align_of::<BufOf<N45>>() != 1 {
        bad += 1;
        println!("align N45 = {}", core::mem::align_of::<BufOf<N45>>());
    }
    if core::mem::size_of::<BufOf<N46>>() != 46 {
        bad += 1;
        println!("size N46 = {}", core::mem::size_of::<BufOf<N46>>());
    }
    if core::mem::align_of::<BufOf<N46>>() != 1 {
        bad += 1;
        println!("align N46 = {}", core::mem::align_of::<BufOf<N46>>());
    }
    if core::mem::size_of::<BufOf<N47>>() != 47 {
        bad += 1;
        println!("size N47 = {}", core::mem::size_of::<BufOf<N47>>());
    }
    if core::mem::align_of::<BufOf<N47>>() != 1 {
        bad += 1;
        println!("align N47 = {}", core::mem::align_of::<BufOf<N47>>());
    }
    if core::mem::size_of::<BufOf<N48>>() != 48 {
        bad += 1;
        println!("size N48 = {}", core::mem::size_of::<BufOf<N48>>());
    }
    if core::mem::align_of::<BufOf<N48>>() != 1 {
        bad += 1;
        println!("align N48 = {}", core::mem::align_of::<BufOf<N48>>());
    }
    if core::mem::size_of::<BufOf<N49>>() != 49 {
        bad += 1;
        println!("size N49 = {}", core::mem::size_of::<BufOf<N49>>());
    }
    if core::mem::align_of::<BufOf<N49>>() != 1 {
        bad += 1;
        println!("align N49 = {}", core::mem::align_of::<BufOf<N49>>());
    }
    if core::mem::size_of::<BufOf<N50>>() != 50 {
        bad += 1;
        println!("size N50 = {}", core::mem::size_of::<BufOf<N50>>());
    }
    if core::mem::align_of::<BufOf<N50>>() != 1 {
        bad += 1;
        println!("align N50 = {}", core::mem::align_of::<BufOf<N50>>());
    }
    if core::mem::size_of::<BufOf<N51>>() != 51 {
        bad += 1;
        println!("size N51 = {}", core::mem::size_of::<BufOf<N51>>());
    }
    if core::mem::align_of::<BufOf<N51>>() != 1 {
        bad += 1;
        println!("align N51 = {}", core::mem::align_of::<BufOf<N51>>());
    }
    if core::mem::size_of::<BufOf<N52>>() != 52 {
        bad += 1;
        println!("size N52 = {}", core::mem::size_of::<BufOf<N52>>());
    }
    if core::mem::align_of::<BufOf<N52>>() != 1 {
        bad += 1;
        println!("align N52 = {}", core::mem::align_of::<BufOf<N52>>());
    }
    if core::mem::size_of::<BufOf<N53>>() != 53 {
        bad += 1;
        println!("size N53 = {}", core::mem::size_of::<BufOf<N53>>());
    }
    if core::mem::align_of::<BufOf<N53>>() != 1 {
        bad += 1;
        println!("align N53 = {}", core::mem::align_of::<BufOf<N53>>());
    }
    if core::mem::size_of::<BufOf<N54>>() != 54 {
        bad += 1;
        println!("size N54 = {}", core::mem::size_of::<BufOf<N54>>());
    }
    if core::mem::align_of::<BufOf<N54>>() != 1 {
        bad += 1;
        println!("align N54 = {}", core::mem::align_of::<BufOf<N54>>());
    }
    if core::mem::size_of::<BufOf<N55>>() != 55 {
        bad += 1;
        println!("size N55 = {}", core::mem::size_of::<BufOf<N55>>());
    }
    if core::mem::align_of::<BufOf<N55>>() != 1 {
        bad += 1;
        println!("align N55 = {}", core::mem::align_of::<BufOf<N55>>());
    }
    if core::mem::size_of::<BufOf<N56>>() != 56 {
        bad += 1;
        println!("size N56 = {}", core::mem::size_of::<BufOf<N56>>());
    }
    if core::mem::align_of::<BufOf<N56>>() != 1 {
        bad += 1;
        println!("align N56 = {}", core::mem::align_of::<BufOf<N56>>());
    }
    if core::mem::size_of::<BufOf<N57>>() != 57 {
        bad += 1;
        println!("size N57 = {}", core::mem::size_of::<BufOf<N57>>());
    }
    if core::mem::align_of::<BufOf<N57>>() != 1 {
        bad += 1;
        println!("align N57 = {}", core::mem::align_of::<BufOf<N57>>());
    }
    if core::mem::size_of::<BufOf<N58>>() != 58 {
        bad += 1;
        println!("size N58 = {}", core::mem::size_of::<BufOf<N58>>());
    }
    if core::mem::align_of::<BufOf<N58>>() != 1 {
        bad += 1;
        println!("align N58 = {}", core::mem::align_of::<BufOf<N58>>());
    }
    if core::mem::size_of::<BufOf<N59>>() != 59 {
        bad += 1;
        println!("size N59 = {}", core::mem::size_of::<BufOf<N59>>());
    }
    if core::mem::align_of::<BufOf<N59>>() != 1 {
        bad += 1;
        println!("align N59 = {}", core::mem::align_of::<BufOf<N59>>());
    }
    if core::mem::size_of::<BufOf<N60>>() != 60 {
        bad += 1;
        println!("size N60 = {}", core::mem::size_of::<BufOf<N60>>());
    }
    if core::mem::align_of::<BufOf<N60>>() != 1 {
        bad += 1;
        println!("align N60 = {}", core::mem::align_of::<BufOf<N60>>());
    }
    if core::mem::size_of::<BufOf<N61>>() != 61 {
        bad += 1;
        println!("size N61 = {}", core::mem::size_of::<BufOf<N61>>());
    }
    if core::mem::align_of::<BufOf<N61>>() != 1 {
        bad += 1;
        println!("align N61 = {}", core::mem::align_of::<BufOf<N61>>());
    }
    if core::mem::size_of::<BufOf<N62>>() != 62 {
        bad += 1;
        println!("size N62 = {}", core::mem::size_of::<BufOf<N62>>());
    }
    if core::mem::align_of::<BufOf<N62>>() != 1 {
        bad += 1;
        println!("align N62 = {}", core::mem::align_of::<BufOf<N62>>());
    }
    if core::mem::size_of::<BufOf<N63>>() != 63 {
        bad += 1;
        println!("size N63 = {}", core::mem::size_of::<BufOf<N63>>());
    }
    if core::mem::align_of::<BufOf<N63>>() != 1 {
        bad += 1;
        println!("align N63 = {}", core::mem::align_of::<BufOf<N63>>());
    }
    if core::mem::size_of::<BufOf<N64>>() != 64 {
        bad += 1;
        println!("size N64 = {}", core::mem::size_of::<BufOf<N64>>());
    }
    if core::mem::align_of::<BufOf<N64>>() != 1 {
        bad += 1;
        println!("align N64 = {}", core::mem::align_of::<BufOf<N64>>());
    }
    if core::mem::size_of::<BufOf<N65>>() != 65 {
        bad += 1;
        println!("size N65 = {}", core::mem::size_of::<BufOf<N65>>());
    }
    if core::mem::align_of::<BufOf<N65>>() != 1 {
        bad += 1;
        println!("align N65 = {}", core::mem::align_of::<BufOf<N65>>());
    }
    if core::mem::size_of::<BufOf<N66>>() != 66 {
        bad += 1;
        println!("size N66 = {}", core::mem::size_of::<BufOf<N66>>());
    }
    if core::mem::align_of::<BufOf<N66>>() != 1 {
        bad += 1;
        println!("align N66 = {}", core::mem::align_of::<BufOf<N66>>());
    }
    if core::mem::size_of::<BufOf<N67>>() != 67 {
        bad += 1;
        println!("size N67 = {}", core::mem::size_of::<BufOf<N67>>());
    }
    if core::mem::align_of::<BufOf<N67>>() != 1 {
        bad += 1;
        println!("align N67 = {}", core::mem::align_of::<BufOf<N67>>());
    }
    if core::mem::size_of::<BufOf<N68>>() != 68 {
        bad += 1;
        println!("size N68 = {}", core::mem::size_of::<BufOf<N68>>());
    }
    if core::mem::align_of::<BufOf<N68>>() != 1 {
        bad += 1;
        println!("align N68 = {}", core::mem::align_of::<BufOf<N68>>());
    }
    if core::mem::size_of::<BufOf<N69>>() != 69 {
        bad += 1;
        println!("size N69 = {}", core::mem::size_of::<BufOf<N69>>());
    }
    if core::mem::align_of::<BufOf<N69>>() != 1 {
        bad += 1;
        println!("align N69 = {}", core::mem::align_of::<BufOf<N69>>());
    }
    if core::mem::size_of::<BufOf<N70>>() != 70 {
        bad += 1;
        println!("size N70 = {}", core::mem::size_of::<BufOf<N70>>());
    }
    if core::mem::align_of::<BufOf<N70>>() != 1 {
        bad += 1;
        println!("align N70 = {}", core::mem::align_of::<BufOf<N70>>());
    }
    if core::mem::size_of::<BufOf<N71>>() != 71 {
        bad += 1;
        println!("size N71 = {}", core::mem::size_of::<BufOf<N71>>());
    }
    if core::mem::align_of::<BufOf<N71>>() != 1 {
        bad += 1;
        println!("align N71 = {}", core::mem::align_of::<BufOf<N71>>());
    }
    if core::mem::size_of::<BufOf<N72>>() != 72 {
        bad += 1;
        println!("size N72 = {}", core::mem::size_of::<BufOf<N72>>());
    }
    if core::mem::align_of::<BufOf<N72>>() != 1 {
        bad += 1;
        println!("align N72 = {}", core::mem::align_of::<BufOf<N72>>());
    }
    if core::mem::size_of::<BufOf<N73>>() != 73 {
        bad += 1;
        println!("size N73 = {}", core::mem::size_of::<BufOf<N73>>());
    }
    if core::mem::align_of::<BufOf<N73>>() != 1 {
        bad += 1;
        println!("align N73 = {}", core::mem::align_of::<BufOf<N73>>());
    }
    if core::mem::size_of::<BufOf<N74>>() != 74 {
        bad += 1;
        println!("size N74 = {}", core::mem::size_of::<BufOf<N74>>());
    }
    if core::mem::align_of::<BufOf<N74>>() != 1 {
        bad += 1;
        println!("align N74 = {}", core::mem::align_of::<BufOf<N74>>());
    }
    if core::mem::size_of::<BufOf<N75>>() != 75 {
        bad += 1;
        println!("size N75 = {}", core::mem::size_of::<BufOf<N75>>());
    }
    if core::mem::align_of::<BufOf<N75>>() != 1 {
        bad += 1;
        println!("align N75 = {}", core::mem::align_of::<BufOf<N75>>());
    }
    if core::mem::size_of::<BufOf<N76>>() != 76 {
        bad += 1;
        println!("size N76 = {}", core::mem::size_of::<BufOf<N76>>());
    }
    if core::mem::align_of::<BufOf<N76>>() != 1 {
        bad += 1;
        println!("align N76 = {}", core::mem::align_of::<BufOf<N76>>());
    }
    if core::mem::size_of::<BufOf<N77>>() != 77 {
        bad += 1;
        println!("size N77 = {}", core::mem::size_of::<BufOf<N77>>());
    }
    if core::mem::align_of::<BufOf<N77>>() != 1 {
        bad += 1;
        println!("align N77 = {}", core::mem::align_of::<BufOf<N77>>());
    }
    if core::mem::size_of::<BufOf<N78>>() != 78 {
        bad += 1;
        println!("size N78 = {}", core::mem::size_of::<BufOf<N78>>());
    }
    if core::mem::align_of::<BufOf<N78>>() != 1 {
        bad += 1;
        println!("align N78 = {}", core::mem::align_of::<BufOf<N78>>());
    }
    if core::mem::size_of::<BufOf<N79>>() != 79 {
        bad += 1;
        println!("size N79 = {}", core::mem::size_of::<BufOf<N79>>());
    }
    if core::mem::align_of::<BufOf<N79>>() != 1 {
        bad += 1;
        println!("align N79 = {}", core::mem::align_of::<BufOf<N79>>());
    }
    if core::mem::size_of::<BufOf<N80>>() != 80 {
        bad += 1;
        println!("size N80 = {}", core::mem::size_of::<BufOf<N80>>());
    }
    if core::mem::align_of::<BufOf<N80>>() != 1 {
        bad += 1;
        println!("align N80 = {}", core::mem::align_of::<BufOf<N80>>());
    }
    if core::mem::size_of::<BufOf<N81>>() != 81 {
        bad += 1;
        println!("size N81 = {}", core::mem::size_of::<BufOf<N81>>());
    }
    if core::mem::align_of::<BufOf<N81>>() != 1 {
        bad += 1;
        println!("align N81 = {}", core::mem::align_of::<BufOf<N81>>());
    }
    if core::mem::size_of::<BufOf<N82>>() != 82 {
        bad += 1;
        println!("size N82 = {}", core::mem::size_of::<BufOf<N82>>());
    }
    if core::mem::align_of::<BufOf<N82>>() != 1 {
        bad += 1;
        println!("align N82 = {}", core::mem::align_of::<BufOf<N82>>());
    }
    if core::mem::size_of::<BufOf<N83>>() != 83 {
        bad += 1;
        println!("size N83 = {}", core::mem::size_of::<BufOf<N83>>());
    }
    if core::mem::align_of::<BufOf<N83>>() != 1 {
        bad += 1;
        println!("align N83 = {}", core::mem::align_of::<BufOf<N83>>());
    }
    if core::mem::size_of::<BufOf<N84>>() != 84 {
        bad += 1;
        println!("size N84 = {}", core::mem::size_of::<BufOf<N84>>());
    }
    if core::mem::align_of::<BufOf<N84>>() != 1 {
        bad += 1;
        println!("align N84 = {}", core::mem::align_of::<BufOf<N84>>());
    }
    if core::mem::size_of::<BufOf<N85>>() != 85 {
        bad += 1;
        println!("size N85 = {}", core::mem::size_of::<BufOf<N85>>());
    }
    if core::mem::align_of::<BufOf<N85>>() != 1 {
        bad += 1;
        println!("align N85 = {}", core::mem::align_of::<BufOf<N85>>());
    }
    if core::mem::size_of::<BufOf<N86>>() != 86 {
        bad += 1;
        println!("size N86 = {}", core::mem::size_of::<BufOf<N86>>());
    }
    if core::mem::align_of::<BufOf<N86>>() != 1 {
        bad += 1;
        println!("align N86 = {}", core::mem::align_of::<BufOf<N86>>());
    }
    if core::mem::size_of::<BufOf<N87>>() != 87 {
        bad += 1;
        println!("size N87 = {}", core::mem::size_of::<BufOf<N87>>());
    }
    if core::mem::align_of::<BufOf<N87>>() != 1 {
        bad += 1;
        println!("align N87 = {}", core::mem::align_of::<BufOf<N87>>());
    }
    if core::mem::size_of::<BufOf<N88>>() != 88 {
        bad += 1;
        println!("size N88 = {}", core::mem::size_of::<BufOf<N88>>());
    }
    if core::mem::align_of::<BufOf<N88>>() != 1 {
        bad += 1;
        println!("align N88 = {}", core::mem::align_of::<BufOf<N88>>());
    }
    if core::mem::size_of::<BufOf<N89>>() != 89 {
        bad += 1;
        println!("size N89 = {}", core::mem::size_of::<BufOf<N89>>());
    }
    if core::mem::align_of::<BufOf<N89>>() != 1 {
        bad += 1;
        println!("align N89 = {}", core::mem::align_of::<BufOf<N89>>());
    }
    if core::mem::size_of::<BufOf<N90>>() != 90 {
        bad += 1;
        println!("size N90 = {}", core::mem::size_of::<BufOf<N90>>());
    }
    if core::mem::align_of::<BufOf<N90>>() != 1 {
        bad += 1;
        println!("align N90 = {}", core::mem::align_of::<BufOf<N90>>());
    }
    if core::mem::size_of::<BufOf<N91>>() != 91 {
        bad += 1;
        println!("size N91 = {}", core::mem::size_of::<BufOf<N91>>());
    }
    if core::mem::align_of::<BufOf<N91>>() != 1 {
        bad += 1;
        println!("align N91 = {}", core::mem::align_of::<BufOf<N91>>());
    }
    if core::mem::size_of::<BufOf<N92>>() != 92 {
        bad += 1;
        println!("size N92 = {}", core::mem::size_of::<BufOf<N92>>());
    }
    if core::mem::align_of::<BufOf<N92>>() != 1 {
        bad += 1;
        println!("align N92 = {}", core::mem::align_of::<BufOf<N92>>());
    }
    if core::mem::size_of::<BufOf<N93>>() != 93 {
        bad += 1;
        println!("size N93 = {}", core::mem::size_of::<BufOf<N93>>());
    }
    if core::mem::align_of::<BufOf<N93>>() != 1 {
        bad += 1;
        println!("align N93 = {}", core::mem::align_of::<BufOf<N93>>());
    }
    if core::mem::size_of::<BufOf<N94>>() != 94 {
        bad += 1;
        println!("size N94 = {}", core::mem::size_of::<BufOf<N94>>());
    }
    if core::mem::align_of::<BufOf<N94>>() != 1 {
        bad += 1;
        println!("align N94 = {}", core::mem::align_of::<BufOf<N94>>());
    }
    if core::mem::size_of::<BufOf<N95>>() != 95 {
        bad += 1;
        println!("size N95 = {}", core::mem::size_of::<BufOf<N95>>());
    }
    if core::mem::align_of::<BufOf<N95>>() != 1 {
        bad += 1;
        println!("align N95 = {}", core::mem::align_of::<BufOf<N95>>());
    }
    if core::mem::size_of::<BufOf<N96>>() != 96 {
        bad += 1;
        println!("size N96 = {}", core::mem::size_of::<BufOf<N96>>());
    }
    if core::mem::align_of::<BufOf<N96>>() != 1 {
        bad += 1;
        println!("align N96 = {}", core::mem::align_of::<BufOf<N96>>());
    }
    if core::mem::size_of::<BufOf<N97>>() != 97 {
        bad += 1;
        println!("size N97 = {}", core::mem::size_of::<BufOf<N97>>());
    }
    if core::mem::align_of::<BufOf<N97>>() != 1 {
        bad += 1;
        println!("align N97 = {}", core::mem::align_of::<BufOf<N97>>());
    }
    if core::mem::size_of::<BufOf<N98>>() != 98 {
        bad += 1;
        println!("size N98 = {}", core::mem::size_of::<BufOf<N98>>());
    }
    if core::mem::align_of::<BufOf<N98>>() != 1 {
        bad += 1;
        println!("align N98 = {}", core::mem::align_of::<BufOf<N98>>());
    }
    if core::mem::size_of::<BufOf<N99>>() != 99 {
        bad += 1;
        println!("size N99 = {}", core::mem::size_of::<BufOf<N99>>());
    }
    if core::mem::align_of::<BufOf<N99>>() != 1 {
        bad += 1;
        println!("align N99 = {}", core::mem::align_of::<BufOf<N99>>());
    }
    if core::mem::size_of::<BufOf<N100>>() != 100 {
        bad += 1;
        println!("size N100 = {}", core::mem::size_of::<BufOf<N100>>());
    }
    if core::mem::align_of::<BufOf<N100>>() != 1 {
        bad += 1;
        println!("align N100 = {}", core::mem::align_of::<BufOf<N100>>());
    }
    if core::mem::size_of::<BufOf<N101>>() != 101 {
        bad += 1;
        println!("size N101 = {}", core::mem::size_of::<BufOf<N101>>());
    }
    if core::mem::align_of::<BufOf<N101>>() != 1 {
        bad += 1;
        println!("align N101 = {}", core::mem::align_of::<BufOf<N101>>());
    }
    if core::mem::size_of::<BufOf<N102>>() != 102 {
        bad += 1;
        println!("size N102 = {}", core::mem::size_of::<BufOf<N102>>());
    }
    if core::mem::align_of::<BufOf<N102>>() != 1 {
        bad += 1;
        println!("align N102 = {}", core::mem::align_of::<BufOf<N102>>());
    }
    if core::mem::size_of::<BufOf<N103>>() != 103 {
        bad += 1;
        println!("size N103 = {}", core::mem::size_of::<BufOf<N103>>());
    }
    if core::mem::align_of::<BufOf<N103>>() != 1 {
        bad += 1;
        println!("align N103 = {}", core::mem::align_of::<BufOf<N103>>());
    }
    if core::mem::size_of::<BufOf<N104>>() != 104 {
        bad += 1;
        println!("size N104 = {}", core::mem::size_of::<BufOf<N104>>());
    }
    if core::mem::align_of::<BufOf<N104>>() != 1 {
        bad += 1;
        println!("align N104 = {}", core::mem::align_of::<BufOf<N104>>());
    }
    if core::mem::size_of::<BufOf<N105>>() != 105 {
        bad += 1;
        println!("size N105 = {}", core::mem::size_of::<BufOf<N105>>());
    }
    if core::mem::align_of::<BufOf<N105>>() != 1 {
        bad += 1;
        println!("align N105 = {}", core::mem::align_of::<BufOf<N105>>());
    }
    if core::mem::size_of::<BufOf<N106>>() != 106 {
        bad += 1;
        println!("size N106 = {}", core::mem::size_of::<BufOf<N106>>());
    }
    if core::mem::align_of::<BufOf<N106>>() != 1 {
        bad += 1;
        println!("align N106 = {}", core::mem::align_of::<BufOf<N106>>());
    }
    if core::mem::size_of::<BufOf<N107>>() != 107 {
        bad += 1;
        println!("size N107 = {}", core::mem::size_of::<BufOf<N107>>());
    }
    if core::mem::align_of::<BufOf<N107>>() != 1 {
        bad += 1;
        println!("align N107 = {}", core::mem::align_of::<BufOf<N107>>());
    }
    if core::mem::size_of::<BufOf<N108>>() != 108 {
        bad += 1;
        println!("size N108 = {}", core::mem::size_of::<BufOf<N108>>());
    }
    if core::mem::align_of::<BufOf<N108>>() != 1 {
        bad += 1;
        println!("align N108 = {}", core::mem::align_of::<BufOf<N108>>());
    }
    if core::mem::size_of::<BufOf<N109>>() != 109 {
        bad += 1;
        println!("size N109 = {}", core::mem::size_of::<BufOf<N109>>());
    }
    if core::mem::align_of::<BufOf<N109>>() != 1 {
        bad += 1;
        println!("align N109 = {}", core::mem::align_of::<BufOf<N109>>());
    }
    if core::mem::size_of::<BufOf<N110>>() != 110 {
        bad += 1;
        println!("size N110 = {}", core::mem::size_of::<BufOf<N110>>());
    }
    if core::mem::align_of::<BufOf<N110>>() != 1 {
        bad += 1;
        println!("align N110 = {}", core::mem::align_of::<BufOf<N110>>());
    }
    if core::mem::size_of::<BufOf<N111>>() != 111 {
        bad += 1;
        println!("size N111 = {}", core::mem::size_of::<BufOf<N111>>());
    }
    if core::mem::align_of::<BufOf<N111>>() != 1 {
        bad += 1;
        println!("align N111 = {}", core::mem::align_of::<BufOf<N111>>());
    }
    if core::mem::size_of::<BufOf<N112>>() != 112 {
        bad += 1;
        println!("size N112 = {}", core::mem::size_of::<BufOf<N112>>());
    }
    if core::mem::align_of::<BufOf<N112>>() != 1 {
        bad += 1;
        println!("align N112 = {}", core::mem::align_of::<BufOf<N112>>());
    }
    if core::mem::size_of::<BufOf<N113>>() != 113 {
        bad += 1;
        println!("size N113 = {}", core::mem::size_of::<BufOf<N113>>());
    }
    if core::mem::align_of::<BufOf<N113>>() != 1 {
        bad += 1;
        println!("align N113 = {}", core::mem::align_of::<BufOf<N113>>());
    }
    if core::mem::size_of::<BufOf<N114>>() != 114 {
        bad += 1;
        println!("size N114 = {}", core::mem::size_of::<BufOf<N114>>());
    }
    if core::mem::align_of::<BufOf<N114>>() != 1 {
        bad += 1;
        println!("align N114 = {}", core::mem::align_of::<BufOf<N114>>());
    }
    if core::mem::size_of::<BufOf<N115>>() != 115 {
        bad += 1;
        println!("size N115 = {}", core::mem::size_of::<BufOf<N115>>());
    }
    if core::mem::align_of::<BufOf<N115>>() != 1 {
        bad += 1;
        println!("align N115 = {}", core::mem::align_of::<BufOf<N115>>());
    }
    if core::mem::size_of::<BufOf<N116>>() != 116 {
        bad += 1;
        println!("size N116 = {}", core::mem::size_of::<BufOf<N116>>());
    }
    if core::mem::align_of::<BufOf<N116>>() != 1 {
        bad += 1;
        println!("align N116 = {}", core::mem::align_of::<BufOf<N116>>());
    }
    if core::mem::size_of::<BufOf<N117>>() != 117 {
        bad += 1;
        println!("size N117 = {}", core::mem::size_of::<BufOf<N117>>());
    }
    if core::mem::align_of::<BufOf<N117>>() != 1 {
        bad += 1;
        println!("align N117 = {}", core::mem::align_of::<BufOf<N117>>());
    }
    if core::mem::size_of::<BufOf<N118>>() != 118 {
        bad += 1;
        println!("size N118 = {}", core::mem::size_of::<BufOf<N118>>());
    }
    if core::mem::align_of::<BufOf<N118>>() != 1 {
        bad += 1;
        println!("align N118 = {}", core::mem::align_of::<BufOf<N118>>());
    }
    if core::mem::size_of::<BufOf<N119>>() != 119 {
        bad += 1;
        println!("size N119 = {}", core::mem::size_of::<BufOf<N119>>());
    }
    if core::mem::align_of::<BufOf<N119>>() != 1 {
        bad += 1;
        println!("align N119 = {}", core::mem::align_of::<BufOf<N119>>());
    }
    if core::mem::size_of::<BufOf<N120>>() != 120 {
        bad += 1;
        println!("size N120 = {}", core::mem::size_of::<BufOf<N120>>());
    }
    if core::mem::align_of::<BufOf<N120>>() != 1 {
        bad += 1;
        println!("align N120 = {}", core::mem::align_of::<BufOf<N120>>());
    }
    if core::mem::size_of::<BufOf<N121>>() != 121 {
        bad += 1;
        println!("size N121 = {}", core::mem::size_of::<BufOf<N121>>());
    }
    if core::mem::align_of::<BufOf<N121>>() != 1 {
        bad += 1;
        println!("align N121 = {}", core::mem::align_of::<BufOf<N121>>());
    }
    if core::mem::size_of::<BufOf<N122>>() != 122 {
        bad += 1;
        println!("size N122 = {}", core::mem::size_of::<BufOf<N122>>());
    }
    if core::mem::align_of::<BufOf<N122>>() != 1 {
        bad += 1;
        println!("align N122 = {}", core::mem::align_of::<BufOf<N122>>());
    }
    if core::mem::size_of::<BufOf<N123>>() != 123 {
        bad += 1;
        println!("size N123 = {}", core::mem::size_of::<BufOf<N123>>());
    }
    if core::mem::align_of::<BufOf<N123>>() != 1 {
        bad += 1;
        println!("align N123 = {}", core::mem::align_of::<BufOf<N123>>());
    }
    if core::mem::size_of::<BufOf<N124>>() != 124 {
        bad += 1;
        println!("size N124 = {}", core::mem::size_of::<BufOf<N124>>());
    }
    if core::mem::align_of::<BufOf<N124>>() != 1 {
        bad += 1;
        println!("align N124 = {}", core::mem::align_of::<BufOf<N124>>());
    }
    if core::mem::size_of::<BufOf<N125>>() != 125 {
        bad += 1;
        println!("size N125 = {}", core::mem::size_of::<BufOf<N125>>());
    }
    if core::mem::align_of::<BufOf<N125>>() != 1 {
        bad += 1;
        println!("align N125 = {}", core::mem::align_of::<BufOf<N125>>());
    }
    if core::mem::size_of::<BufOf<N126>>() != 126 {
        bad += 1;
        println!("size N126 = {}", core::mem::size_of::<BufOf<N126>>());
    }
    if core::mem::align_of::<BufOf<N126>>() != 1 {
        bad += 1;
        println!("align N126 = {}", core::mem::align_of::<BufOf<N126>>());
    }
    if core::mem::size_of::<BufOf<N127>>() != 127 {
        bad += 1;
        println!("size N127 = {}", core::mem::size_of::<BufOf<N127>>());
    }
    if core::mem::align_of::<BufOf<N127>>() != 1 {
        bad += 1;
        println!("align N127 = {}", core::mem::align_of::<BufOf<N127>>());
    }
    if core::mem::size_of::<BufOf<N128>>() != 128 {
        bad += 1;
        println!("size N128 = {}", core::mem::size_of::<BufOf<N128>>());
    }
    if core::mem::align_of::<BufOf<N128>>() != 1 {
        bad += 1;
        println!("align N128 = {}", core::mem::align_of::<BufOf<N128>>());
    }
    println!("buffer sizes checked 0..=128, mismatches = {}", bad);
}
