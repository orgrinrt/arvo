// p8: file 103 offers a `select` on the TRUTH type, thunked, as the spelling
// that never names `bool`. Question: is that an escape from the exit, or the
// exit renamed? And is it the same object as a lane-wise blending select?
#![crate_type = "lib"]

pub trait Algebra: Copy {
    fn and(self, o: Self) -> Self;
}

// A: file 103's shape. A selector ON THE TRUTH, thunked, generic in R.
pub trait TruthSelect: Algebra {
    fn select<R, T: FnOnce() -> R, F: FnOnce() -> R>(self, t: T, f: F) -> R;
}
// B: the exit.
pub trait Exit: Algebra {
    fn is_true(self) -> bool;
}

// each defines the other, in both directions, for any type at all
pub fn exit_from_select<S: TruthSelect>(s: S) -> bool {
    s.select(|| true, || false)
}
pub fn select_from_exit<E: Exit, R>(e: E, t: impl FnOnce() -> R, f: impl FnOnce() -> R) -> R {
    if e.is_true() {
        t()
    } else {
        f()
    }
}

// so a truth contract carrying A carries B, and inherits its whole problem:
#[derive(Clone, Copy)]
pub struct Mask2([bool; 2]);
impl Algebra for Mask2 {
    fn and(self, o: Self) -> Self {
        Mask2([self.0[0] & o.0[0], self.0[1] & o.0[1]])
    }
}
impl TruthSelect for Mask2 {
    // there is no correct body here. one of the lanes, or a reduction, must be
    // chosen, and the choice is invisible from every call site.
    fn select<R, T: FnOnce() -> R, F: FnOnce() -> R>(self, t: T, f: F) -> R {
        if self.0[0] & self.0[1] {
            t()
        } else {
            f()
        } // silently ALL
    }
}
pub fn mask_has_an_exit_after_all(m: Mask2) -> bool {
    exit_from_select(m)
}
