//! p4. Is the affine-collapse mechanism a fact about the code or about the
//! codegen profile the arvo bench tree happens to have been built at?
//!
//! `117` establishes that every number in `mock/benches/` was taken at cargo's
//! DEFAULT release profile (`lto = false`, `codegen-units = 16`) rather than
//! the fat-LTO / one-codegen-unit profile the harness documents, and says
//! plainly that anyone citing a bench number should establish what that means
//! for their citation first. This is that establishment for one citation:
//! `warm-affine-collapse-l1`, which is the largest chain result in the tree.
//!
//! This is an AD-HOC QUICK SPIKE WITH NO SUBSTANCE as a measurement. It prices
//! nothing and no timing appears in it. What it can do is establish a
//! QUALITATIVE fact: whether the interior projection blocks the affine
//! collapse under BOTH profiles, or only under one.
//!
//! Method: emit assembly for four functions at each profile and count the
//! multiply-class instructions in each function's body.
//!
//! WHAT I PREDICTED, AND WHAT HAPPENED. I predicted `affine_deferred` would
//! differ from `affine_eager` (the collapse) while `blocked_deferred` and
//! `blocked_eager` would NOT differ, because the interposed right shift is not
//! affine over the ring, so deferring cannot buy the collapse there. That was
//! the control.
//!
//! **The control fired in the wrong direction.** Both deferred arms vectorise
//! and both eager arms do not, at both profiles, `blocked` included. So the
//! difference is not the affine collapse, because it is present in a chain
//! where the collapse is impossible. That is a refutation of my own hypothesis
//! and it is what sent this to `p5`, which separates the projection on the
//! per-element VALUE from the projection on the loop-carried ACCUMULATOR and
//! finds that only the second one matters.
//!
//! So read this probe for two things and nothing else: the emitted code is
//! identical at both codegen profiles, and the mechanism is not the one this
//! file was built to test.
//!
//! Run: see p4_run.sh beside this file.

#![crate_type = "lib"]

const W: u32 = 13;
const MASK: u32 = (1u32 << W) - 1;

#[inline(always)]
fn m(v: u32) -> u32 {
    v & MASK
}

/// Three ring-affine steps, projection written ONCE at the boundary.
/// `+k`, `*3`, `-k` composes to `3v + 2k` by ordinary algebra, so a backend
/// that can see the chain may emit one multiply-add.
#[unsafe(no_mangle)]
pub fn affine_deferred(data: &[u32], k: u32) -> u32 {
    let mut acc: u32 = 0;
    for &x in data {
        let mut v = x;
        v = v.wrapping_add(k);
        v = v.wrapping_mul(3);
        v = v.wrapping_sub(k);
        acc = acc.wrapping_add(v);
    }
    m(acc)
}

/// The identical chain with the projection written after every step.
#[unsafe(no_mangle)]
pub fn affine_eager(data: &[u32], k: u32) -> u32 {
    let mut acc: u32 = 0;
    for &x in data {
        let mut v = x;
        v = m(v.wrapping_add(k));
        v = m(v.wrapping_mul(3));
        v = m(v.wrapping_sub(k));
        acc = m(acc.wrapping_add(v));
    }
    m(acc)
}

/// CONTROL. The same three affine steps with a right shift interposed. A shift
/// is not affine over the ring, so the chain does not compose to one
/// multiply-add and deferring the projection cannot buy the collapse.
#[unsafe(no_mangle)]
pub fn blocked_deferred(data: &[u32], k: u32) -> u32 {
    let mut acc: u32 = 0;
    for &x in data {
        let mut v = x;
        v = v.wrapping_add(k);
        v = v.wrapping_mul(3);
        v >>= 1;
        v = v.wrapping_mul(5);
        v = v.wrapping_sub(k);
        acc = acc.wrapping_add(v);
    }
    m(acc)
}

#[unsafe(no_mangle)]
pub fn blocked_eager(data: &[u32], k: u32) -> u32 {
    let mut acc: u32 = 0;
    for &x in data {
        let mut v = x;
        v = m(v.wrapping_add(k));
        v = m(v.wrapping_mul(3));
        v = m(v >> 1);
        v = m(v.wrapping_mul(5));
        v = m(v.wrapping_sub(k));
        acc = m(acc.wrapping_add(v));
    }
    m(acc)
}
