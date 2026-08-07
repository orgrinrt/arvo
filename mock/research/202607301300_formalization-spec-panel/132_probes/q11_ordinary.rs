//! Ordinary generic Rust, of the shape a downstream stack crate is full of.
//! Does -Znext-solver=globally accept it, and what does it cost?
#![no_std]
#![crate_type = "lib"]
use core::marker::PhantomData;

pub trait Store {
    type Item: Copy;
    fn get(&self, i: usize) -> Self::Item;
    fn len(&self) -> usize;
}
pub trait Push<T> {
    fn push(&mut self, t: T);
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

pub struct Sum;
impl Fold for Sum {
    type Acc = u32;
    fn unit() -> u32 {
        0
    }
    fn step(a: u32, x: u32) -> u32 {
        a.wrapping_add(x)
    }
}
pub struct Max;
impl Fold for Max {
    type Acc = u32;
    fn unit() -> u32 {
        0
    }
    fn step(a: u32, x: u32) -> u32 {
        if x > a {
            x
        } else {
            a
        }
    }
}

pub fn reduce<S: Store<Item = u32>, F: Fold<Acc = u32>>(s: &S) -> u32 {
    let mut a = F::unit();
    for i in 0..s.len() {
        a = F::step(a, s.get(i));
    }
    a
}

pub trait Pipe: Sized {
    fn pipe<U, G: FnOnce(Self) -> U>(self, g: G) -> U {
        g(self)
    }
}
impl<T> Pipe for T {}

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

pub trait Contains<X> {}
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);
impl<H, T> Contains<H> for Cons<H, T> {}
impl<H, T, X> Contains<X> for Cons<H, T> where T: Contains<X> {}

pub fn needs<L: Contains<u8> + Contains<u16> + Contains<u32>>() {}
pub fn ok() {
    needs::<Cons<u8, Cons<u16, Cons<u32, Nil>>>>();
}

pub fn iter_chain(xs: &[u32]) -> u32 {
    xs.iter()
        .copied()
        .filter(|x| *x > 2)
        .map(|x| x * 3)
        .take(100)
        .fold(0u32, |a, b| a.wrapping_add(b))
}

pub fn deep(a: Arr<u32, 8>, b: Arr<u32, 4>) -> u32 {
    let c = Chain(Tagged::<_, Sum>(a, PhantomData), b);
    reduce::<_, Sum>(&c).wrapping_add(reduce::<_, Max>(&c))
}
