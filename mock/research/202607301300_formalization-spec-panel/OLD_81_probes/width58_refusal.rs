//! Compile-fail companion for file 81: the plan's own width refusal.
//!
//! A field may begin at any of eight bit offsets within a byte, so a 64-bit
//! window covers a field only up to 57 bits wide. `Pack<58>` therefore has no
//! well-formed plan, and the refusal is a monomorphisation-time const
//! evaluation failure rather than a wrong lane at runtime.
//!
//! Expected: error[E0080] evaluation of `<Pack<58> as Packing>::WINDOW_FITS`
//! failed. Compiling this file successfully is the finding's failure.

const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
const fn period_of(w: usize) -> usize {
    8 / gcd(w, 8)
}

pub trait Packing {
    const P: usize;
    const WINDOW_FITS: ();
}
pub struct Pack<const W: usize>;
impl<const W: usize> Packing for Pack<W> {
    const P: usize = period_of(W);
    const WINDOW_FITS: () = assert!(
        W + 7 <= 64,
        "a 64-bit window cannot hold a field wider than 57 bits"
    );
}

pub fn use_plan<K: Packing>() -> usize {
    let () = K::WINDOW_FITS;
    K::P
}

fn main() {
    // 57 is the last width a 64-bit window admits.
    println!("{}", use_plan::<Pack<57>>());
    // 58 has no well-formed plan.
    println!("{}", use_plan::<Pack<58>>());
}
