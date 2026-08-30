// arvo's real shape: capacity is a TYPE with an associated array, no const expr.
pub trait Capacity {
    type Array<T>: AsRef<[T]>;
    const N: usize;
}
pub struct Dim<const N: usize>;
impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];
    const N: usize = N;
}

/// ONE generic kernel. Extent comes from the type; no GCE needed.
#[inline(always)]
pub fn sum<C: Capacity>(v: &C::Array<u32>) -> u32 {
    let s = v.as_ref();
    let mut acc = 0u32;
    for x in s {
        acc = acc.wrapping_add(*x);
    }
    acc
}

pub fn k16(v: &<Dim<16> as Capacity>::Array<u32>) -> u32 {
    sum::<Dim<16>>(v)
}
pub fn k17(v: &<Dim<17> as Capacity>::Array<u32>) -> u32 {
    sum::<Dim<17>>(v)
}
pub fn k1024(v: &<Dim<1024> as Capacity>::Array<u32>) -> u32 {
    sum::<Dim<1024>>(v)
}
pub fn kdyn(v: &[u32]) -> u32 {
    let mut a = 0u32;
    for x in v {
        a = a.wrapping_add(*x);
    }
    a
}
