pub trait Capacity {
    type Array<T>: AsRef<[T]>;
    const N: usize;
    /// DERIVED property. The type computes it; no consumer states it.
    const LANE_ALIGNED: bool;
    const SMALL: bool;
}
pub struct Dim<const N: usize>;
impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];
    const N: usize = N;
    const LANE_ALIGNED: bool = N.is_multiple_of(4);
    const SMALL: bool = N <= 8;
}

// Three explicit kernel bodies, as a hand-written microkernel set would be.
#[inline(always)]
fn k_unrolled4(s: &[u32]) -> u32 {
    let mut a = [0u32; 4];
    let mut i = 0;
    while i + 4 <= s.len() {
        a[0] = a[0].wrapping_add(s[i]);
        a[1] = a[1].wrapping_add(s[i + 1]);
        a[2] = a[2].wrapping_add(s[i + 2]);
        a[3] = a[3].wrapping_add(s[i + 3]);
        i += 4;
    }
    a[0].wrapping_add(a[1])
        .wrapping_add(a[2])
        .wrapping_add(a[3])
}
#[inline(always)]
fn k_flat(s: &[u32]) -> u32 {
    let mut a = 0u32;
    for x in s {
        a = a.wrapping_add(*x);
    }
    a
}
#[inline(always)]
fn k_pairwise(s: &[u32]) -> u32 {
    if s.len() < 2 {
        return s.iter().copied().fold(0u32, |a, b| a.wrapping_add(b));
    }
    let (l, r) = s.split_at(s.len() / 2);
    k_flat(l).wrapping_add(k_flat(r))
}

/// ONE entry point. Kernel chosen from properties the TYPE derived.
#[inline(always)]
pub fn sum<C: Capacity>(v: &C::Array<u32>) -> u32 {
    let s = v.as_ref();
    if C::SMALL {
        k_flat(s)
    } else if C::LANE_ALIGNED {
        k_unrolled4(s)
    } else {
        k_pairwise(s)
    }
}

pub fn m8(v: &[u32; 8]) -> u32 {
    sum::<Dim<8>>(v)
} // SMALL
pub fn m64(v: &[u32; 64]) -> u32 {
    sum::<Dim<64>>(v)
} // LANE_ALIGNED
pub fn m66(v: &[u32; 66]) -> u32 {
    sum::<Dim<66>>(v)
} // neither
