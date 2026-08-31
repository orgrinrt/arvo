//! B1, must FAIL, and the failure is the staging boundary.
//!
//! The accumulator width is a function of the element count. The count is a runtime
//! quantity (`slice.len()`), so there is no stage-zero value to instantiate the
//! accumulator with, and the fold has no monomorphic form.
#![allow(dead_code)]

struct Acc<const A: usize>([i64; A]);

const fn acc_width(count: usize) -> usize {
    let mut k = 0;
    while (1usize << k) < count {
        k += 1;
    }
    8 + k
}

/// The count comes from the value, not from the type.
fn fold(xs: &[i8]) -> Acc<{ acc_width(xs.len()) }> {
    let mut a = [0i64; acc_width(xs.len())];
    a[0] = xs.iter().map(|&v| v as i64).sum();
    Acc(a)
}

fn main() {
    let v = [1i8, 2, 3];
    let _ = fold(&v);
}
