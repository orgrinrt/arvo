#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
pub struct Unsigned;
pub struct Signed;
pub trait Sign {
    const EXTRA: u32;
}
impl Sign for Unsigned {
    const EXTRA: u32 = 0;
}
impl Sign for Signed {
    const EXTRA: u32 = 1;
}
pub struct Hot;
pub struct Warm;

pub const fn tag_hot(n: u32) -> usize {
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else {
        5
    }
}
pub const fn tag_warm(n: u32) -> usize {
    if n <= 8 {
        1
    } else if n <= 16 {
        2
    } else if n <= 32 {
        3
    } else {
        5
    }
}
pub const fn bytes_for(n: u32) -> usize {
    (n as usize).div_ceil(8)
}

pub struct Picker;
pub trait Project<const TAG: usize, const BYTES: usize> {
    type T: Copy;
}
impl<const B: usize> Project<0, B> for Picker {
    type T = u8;
}
impl<const B: usize> Project<1, B> for Picker {
    type T = u16;
}
impl<const B: usize> Project<2, B> for Picker {
    type T = u32;
}
impl<const B: usize> Project<3, B> for Picker {
    type T = u64;
}
#[derive(Copy, Clone)]
pub struct Wide<const B: usize>(pub [u8; B]);
impl<const B: usize> Project<5, B> for Picker {
    type T = Wide<B>;
}

pub struct Rung<const I: u32, const F: u32, G, S>(core::marker::PhantomData<(G, S)>);
pub trait Tagged {
    type const TAG: usize;
    type const BYTES: usize;
}
impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, Hot> {
    type const TAG: usize = const { tag_hot(G::EXTRA + I + F) };
    type const BYTES: usize = const { bytes_for(G::EXTRA + I + F) };
}
impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, Warm> {
    type const TAG: usize = const { tag_warm(G::EXTRA + I + F) };
    type const BYTES: usize = const { bytes_for(G::EXTRA + I + F) };
}

pub trait Store<const I: u32, const F: u32, G: Sign> {
    type T: Copy;
}
impl<const I: u32, const F: u32, G: Sign> Store<I, F, G> for Hot
where
    Picker:
        Project<{ <Rung<I, F, G, Hot> as Tagged>::TAG }, { <Rung<I, F, G, Hot> as Tagged>::BYTES }>,
{
    type T = <Picker as Project<
        { <Rung<I, F, G, Hot> as Tagged>::TAG },
        { <Rung<I, F, G, Hot> as Tagged>::BYTES },
    >>::T;
}
impl<const I: u32, const F: u32, G: Sign> Store<I, F, G> for Warm
where
    Picker: Project<
        { <Rung<I, F, G, Warm> as Tagged>::TAG },
        { <Rung<I, F, G, Warm> as Tagged>::BYTES },
    >,
{
    type T = <Picker as Project<
        { <Rung<I, F, G, Warm> as Tagged>::TAG },
        { <Rung<I, F, G, Warm> as Tagged>::BYTES },
    >>::T;
}

pub struct Fixed<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> {
    raw: <S as Store<I, F, G>>::T,
    _m: core::marker::PhantomData<G>,
}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;

// the typestate a downstream optimisation layer would read
pub trait Lowering {
    type Container: Copy;
    const STORED_WIDTH: u32;
    const RUNG: usize;
    const BYTES: usize;
}
impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Lowering for Fixed<I, F, G, S> {
    type Container = <S as Store<I, F, G>>::T;
    const STORED_WIDTH: u32 = G::EXTRA + I + F;
    const RUNG: usize = 0; // placeholder read, kept plain on purpose
    const BYTES: usize = bytes_for(G::EXTRA + I + F);
}

impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Fixed<I, F, G, S> {
    pub fn from_raw(raw: <S as Store<I, F, G>>::T) -> Self {
        Fixed {
            raw,
            _m: core::marker::PhantomData,
        }
    }
    pub fn to_raw(self) -> <S as Store<I, F, G>>::T {
        self.raw
    }
}

// a width-generic law, the thing 131/132 measured as needing the flag downstream
pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
    G: Sign,
    S,
>(
    _a: Fixed<I, F, G, S>,
    _b: Fixed<J, K, G, S>,
) -> Fixed<M, N, G, S>
where
    S: Store<I, F, G> + Store<J, K, G> + Store<M, N, G>,
    <S as Store<M, N, G>>::T: Default,
{
    Fixed {
        raw: Default::default(),
        _m: core::marker::PhantomData,
    }
}
