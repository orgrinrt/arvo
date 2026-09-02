//! p7: does a composition's `len <= capacity` invariant survive to the backend?
//!
//! Section 6.3 of the file names indexing as the second eliminator and the one
//! I had not investigated.  This probe attacks it.
//!
//! The claim under test is a staging claim.  A composition's static part
//! carries a capacity; its dynamic part carries a length; and `len <= capacity`
//! is the composition's own invariant.  If that invariant reaches the backend,
//! an indexed traversal needs no bounds check, because the index is bounded by
//! the length which is bounded by the array's own const size.  If it does not
//! reach the backend, the check is emitted and is provably dead, which is the
//! shape `small-wins-compound-into-the-program.md` describes: one instruction
//! the compiler cannot prove away, blocking something larger.
//!
//! Arms, each a separate exported function so the emitted symbols can be told
//! apart:
//!
//!   sum_slice        a runtime-length slice, indexed by a runtime index.
//!                    The baseline: nothing static is known.
//!   sum_run_unproven a fixed-capacity array plus a length field the type
//!                    system does not relate to it.  The invariant exists in
//!                    the author's head only.
//!   sum_run_clamped  the same, with the length clamped to the capacity at the
//!                    loop bound.  The proof supplied by an instruction.
//!   sum_run_iter     the same, traversed through the capacity-bounded slice
//!                    rather than by index.  The proof supplied by the shape.
//!   sum_full         the whole capacity, no length at all.  The upper bound on
//!                    what removing the check can buy.
//!
//! This is a QUALITATIVE check and is called one.  It reads emitted assembly
//! for the presence or absence of a panic path.  It is not a bench, it prices
//! nothing, and no timing figure appears anywhere in it.
//!
//! Build:
//!   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -O \
//!         --emit asm p7_does_the_capacity_bound_survive_lowering.rs

#![no_std]
#![forbid(unsafe_code)]

pub const K: usize = 64;

/// arm 1: a bare slice.  Length is runtime, capacity does not exist.
#[inline(never)]
pub fn sum_slice(data: &[i64], len: usize) -> i64 {
    let mut acc = 0i64;
    let mut i = 0usize;
    while i < len {
        acc = acc.wrapping_add(data[i]);
        i += 1;
    }
    acc
}

/// A composition: a static capacity and a dynamic length.
pub struct Run {
    pub data: [i64; K],
    pub len: usize,
}

/// arm 2: the invariant exists but nothing states it.  `len` is an ordinary
/// field and the compiler has no reason to believe it is at most `K`.
#[inline(never)]
pub fn sum_run_unproven(r: &Run) -> i64 {
    let mut acc = 0i64;
    let mut i = 0usize;
    while i < r.len {
        acc = acc.wrapping_add(r.data[i]);
        i += 1;
    }
    acc
}

/// arm 3: the invariant supplied by an instruction at the loop bound.
#[inline(never)]
pub fn sum_run_clamped(r: &Run) -> i64 {
    let n = if r.len < K { r.len } else { K };
    let mut acc = 0i64;
    let mut i = 0usize;
    while i < n {
        acc = acc.wrapping_add(r.data[i]);
        i += 1;
    }
    acc
}

/// arm 4: the invariant supplied by the shape.  The traversal goes through a
/// subslice whose bound the compiler establishes once, rather than through an
/// index it has to bound every iteration.
#[inline(never)]
pub fn sum_run_iter(r: &Run) -> i64 {
    let n = if r.len < K { r.len } else { K };
    let mut acc = 0i64;
    for &x in &r.data[..n] {
        acc = acc.wrapping_add(x);
    }
    acc
}

/// arm 5: no length at all.  The whole capacity, which is the ceiling on what
/// any of the above can reach.
#[inline(never)]
pub fn sum_full(r: &Run) -> i64 {
    let mut acc = 0i64;
    for &x in r.data.iter() {
        acc = acc.wrapping_add(x);
    }
    acc
}

/// A composition whose length cannot exceed its capacity, because the only
/// constructor refuses.  This is the invariant stated where the type system can
/// see it, which is the arrangement the file argues a composition contract is
/// for.
pub struct BoundedRun {
    data: [i64; K],
    len: usize,
}

impl BoundedRun {
    /// The one door.  A length past the capacity is refused rather than stored,
    /// so every value of this type satisfies `len <= K` by construction.
    pub const fn new(data: [i64; K], len: usize) -> Option<Self> {
        if len <= K {
            Some(Self { data, len })
        } else {
            None
        }
    }

    /// The traversal, written the way a consumer would.
    #[inline(never)]
    pub fn sum(&self) -> i64 {
        let mut acc = 0i64;
        let mut i = 0usize;
        while i < self.len {
            acc = acc.wrapping_add(self.data[i]);
            i += 1;
        }
        acc
    }

    /// The same traversal through the shape rather than the index.
    #[inline(never)]
    pub fn sum_via_slice(&self) -> i64 {
        let mut acc = 0i64;
        for &x in &self.data[..self.len] {
            acc = acc.wrapping_add(x);
        }
        acc
    }
}

// no_std needs a panic handler only for a binary; a lib does not, and this file
// is compiled as a lib so the panic paths stay visible as calls rather than
// being inlined into an abort.
