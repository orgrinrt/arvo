//! p5. Which of the two interior projections in a chained reduction actually
//! costs: the one on the per-element value, or the one on the loop-carried
//! accumulator?
//!
//! p4 established that for a per-element affine chain the two projections
//! placements differ by exactly one instruction and neither vectorises, which
//! does not reproduce the 23.95x that `warm-affine-collapse-l1_n130403`
//! reports. So the mechanism is not "the interior value projection blocks the
//! algebraic collapse" on its own. This probe separates the two projections
//! and asks which one the backend actually reacts to.
//!
//! Container is `u16` at `W = 13`, matching the bench's `minimum` arm, whose
//! eager and deferred forms are the 9380 ns and 405 ns rows of that family.
//!
//! Four arms, one per cell of (value projection eager | deferred) x
//! (accumulator projection eager | deferred).
//!
//! THE CASE THAT MUST DIFFER, or this instrument reports nothing: at least one
//! pair of arms must differ in whether the loop vectorises. If all four emit
//! the same shape the probe has not isolated anything and says so.
//!
//! Run: sh p5_run.sh

#![crate_type = "lib"]

const W: u32 = 13;
const MASK: u16 = (1u16 << W) - 1;

#[inline(always)]
fn m(v: u16) -> u16 {
    v & MASK
}

macro_rules! arm {
    ($name:ident, $ve:expr, $ae:expr) => {
        #[unsafe(no_mangle)]
        pub fn $name(data: &[u16], k: u16) -> u16 {
            let mut acc: u16 = 0;
            for &x in data {
                let mut v = x;
                v = v.wrapping_add(k);
                if $ve {
                    v = m(v);
                }
                v = v.wrapping_mul(3);
                if $ve {
                    v = m(v);
                }
                v = v.wrapping_sub(k);
                if $ve {
                    v = m(v);
                }
                acc = acc.wrapping_add(v);
                if $ae {
                    acc = m(acc);
                }
            }
            m(acc)
        }
    };
}

arm!(both_deferred, false, false);
arm!(value_eager, true, false);
arm!(acc_eager, false, true);
arm!(both_eager, true, true);

/// POSITIVE CONTROL. A plain wrapping reduction with no projection anywhere.
/// This is the shape the backend vectorises unaided, and if it does not
/// vectorise here then this probe's compile setup, not the arms, is what is
/// suppressing vectorisation, and no conclusion about the masks may be drawn.
#[unsafe(no_mangle)]
pub fn control_plain_sum(data: &[u16]) -> u16 {
    let mut acc: u16 = 0;
    for &x in data {
        acc = acc.wrapping_add(x);
    }
    acc
}

/// SECOND POSITIVE CONTROL, with the loop written over a slice index range so
/// the trip count is unambiguous, in case the iterator form is what blocks it.
#[unsafe(no_mangle)]
pub fn control_indexed_sum(data: &[u16]) -> u16 {
    let mut acc: u16 = 0;
    let n = data.len();
    let mut i = 0;
    while i < n {
        acc = acc.wrapping_add(data[i]);
        i += 1;
    }
    acc
}
