// P8. Does selection on a COMPUTED demand set erase, or only on a written one?
//
// `93`'s P4 and `94`'s probes A, B and F all establish that a strategy's selection
// leaves no residue in the emitted body. Every one of them selects on a demand or a
// region written literally at the site.
//
// Section 4.1 of `97` proposes something those do not cover: the strategy space is
// generated, so the demand set a mixed expression carries is the OUTPUT of a
// type-level join rather than a name anybody wrote. If a selector reading a computed
// type costs anything the solver cannot fold away, the generated lattice is a nice
// idea with a price at every mixed site, and the whole of section 4.1 is worth less
// than it claims.
//
// So this compares four entry points at `-O`:
//
//   entry_speed          selects on a demand set written literally
//   entry_residency      selects on a different literal demand set
//   entry_joined         selects on `JoinOf<Speed, Residency>`, a COMPUTED type
//   entry_handwritten    calls the arm directly, with no selector at all
//
// The claim under test is narrow and is the one the evidence can carry: the computed
// demand leaves no residue relative to a written one, and the selected arm is reached
// as directly as a hand-written call. It says nothing about compile time, which is a
// how-much question and is unpriced here and said to be unpriced.
//
// This is a spike. The three arms are deliberately trivial and distinguishable so the
// comparison is about the selector rather than about the arithmetic.
//
// Build: rustc --edition 2024 -O --crate-type lib --emit asm p8_does_a_computed_demand_erase.rs

#![no_std]

// ---------------------------------------------------------------- the lattice
// Same shape as p4_demand_lattice.rs, cut to two coordinates because the question
// here is the computed selector rather than the lattice's laws.

pub struct Absent;
pub struct Present;

pub trait Or<Rhs> {
    type Out;
}
impl Or<Absent> for Absent {
    type Out = Absent;
}
impl Or<Present> for Absent {
    // MUTANT: the join drops a demand, so it collapses onto a generator. If the
    // compile-time tag assertion is real, this must not compile.
    type Out = Absent;
}
impl Or<Absent> for Present {
    type Out = Present;
}
impl Or<Present> for Present {
    type Out = Present;
}

pub struct Demands<S, R>(core::marker::PhantomData<(S, R)>);

pub trait Join<Rhs> {
    type Out;
}
impl<S1, R1, S2, R2> Join<Demands<S2, R2>> for Demands<S1, R1>
where
    S1: Or<S2>,
    R1: Or<R2>,
{
    type Out = Demands<<S1 as Or<S2>>::Out, <R1 as Or<R2>>::Out>;
}

pub type JoinOf<X, Y> = <X as Join<Y>>::Out;

pub type Nothing = Demands<Absent, Absent>;
pub type Speed = Demands<Present, Absent>;
pub type Residency = Demands<Absent, Present>;

// ---------------------------------------------------------------- the arms
// Three distinguishable kernels over one shared input. Deliberately different so a
// wrong selection is visible in the emitted body rather than folding into the others.

#[inline(never)]
pub fn arm_wide(xs: &[u32]) -> u64 {
    let mut a: u64 = 0;
    for &x in xs {
        a = a.wrapping_add(x as u64);
    }
    a
}

#[inline(never)]
pub fn arm_narrow(xs: &[u32]) -> u64 {
    let mut a: u32 = 0;
    for &x in xs {
        a = a.wrapping_add(x);
    }
    a as u64
}

#[inline(never)]
pub fn arm_pairwise(xs: &[u32]) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut i = 0;
    while i + 1 < xs.len() {
        a = a.wrapping_add(xs[i] as u64);
        b = b.wrapping_add(xs[i + 1] as u64);
        i += 2;
    }
    if i < xs.len() {
        a = a.wrapping_add(xs[i] as u64);
    }
    a.wrapping_add(b)
}

#[inline(never)]
pub fn arm_both(xs: &[u32]) -> u64 {
    // distinct from all three above: four accumulators rather than one or two
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;
    let mut d: u64 = 0;
    let mut i = 0;
    while i + 3 < xs.len() {
        a = a.wrapping_add(xs[i] as u64);
        b = b.wrapping_add(xs[i + 1] as u64);
        c = c.wrapping_add(xs[i + 2] as u64);
        d = d.wrapping_add(xs[i + 3] as u64);
        i += 4;
    }
    while i < xs.len() {
        a = a.wrapping_add(xs[i] as u64);
        i += 1;
    }
    a.wrapping_add(b).wrapping_add(c).wrapping_add(d)
}

// ---------------------------------------------------------------- the resolver
// One impl per demand set, which is the design tier writing a table. The table is
// read through the type, so a computed demand set resolves through exactly the same
// mechanism a written one does.

pub trait Resolve {
    fn run(xs: &[u32]) -> u64;
}

impl Resolve for Demands<Absent, Absent> {
    fn run(xs: &[u32]) -> u64 {
        arm_wide(xs)
    }
}
impl Resolve for Demands<Present, Absent> {
    fn run(xs: &[u32]) -> u64 {
        arm_pairwise(xs)
    }
}
impl Resolve for Demands<Absent, Present> {
    fn run(xs: &[u32]) -> u64 {
        arm_narrow(xs)
    }
}
// the element nobody named: demanding both. Under a closed set of four this point
// has no spelling, which is `93`'s F3 in one line. It gets its OWN arm, distinct
// from either generator's, so that a selector resolving the join wrongly lands on a
// different symbol and the comparison below cannot pass by coincidence.
impl Resolve for Demands<Present, Present> {
    fn run(xs: &[u32]) -> u64 {
        arm_both(xs)
    }
}

pub fn fold<D: Resolve>(xs: &[u32]) -> u64 {
    D::run(xs)
}

// ---------------------------------------------------------------- entry points

#[unsafe(no_mangle)]
pub fn entry_speed(xs: &[u32]) -> u64 {
    fold::<Speed>(xs)
}

#[unsafe(no_mangle)]
pub fn entry_residency(xs: &[u32]) -> u64 {
    fold::<Residency>(xs)
}

// the one the whole section rests on: the demand set is COMPUTED by the join rather
// than written, so the solver has to walk `Or` twice before it knows which impl.
#[unsafe(no_mangle)]
pub fn entry_joined(xs: &[u32]) -> u64 {
    fold::<JoinOf<Speed, Residency>>(xs)
}

// and the control: the same arm the joined demand resolves to, called with no
// selector machinery at all. `entry_joined` must reach this and nothing else.
#[unsafe(no_mangle)]
pub fn entry_handwritten(xs: &[u32]) -> u64 {
    arm_both(xs)
}

// two negative controls, so an identical pair above is not vacuous. Neither of the
// join's operands resolves to the join's arm, so `entry_joined` matching either of
// these would mean the join collapsed onto a generator.
#[unsafe(no_mangle)]
pub fn entry_different(xs: &[u32]) -> u64 {
    arm_narrow(xs)
}

// And a compile-time assertion that the join really did land on the both-demands
// point rather than on one of the generators, so the comparison is of the right
// thing. A mutant that breaks `Or` makes this fail.
pub trait Tag {
    const T: u8;
}
impl Tag for Demands<Absent, Absent> {
    const T: u8 = 0;
}
impl Tag for Demands<Present, Absent> {
    const T: u8 = 1;
}
impl Tag for Demands<Absent, Present> {
    const T: u8 = 2;
}
impl Tag for Demands<Present, Present> {
    const T: u8 = 3;
}

const _: () = assert!(
    <JoinOf<Speed, Residency> as Tag>::T == 3,
    "the join did not reach the both-demands point, so the comparison below is vacuous"
);
