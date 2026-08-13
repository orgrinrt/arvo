// Probe F. WHERE does each component of a strategy have to be carried?
//
// Probe B separated three components. This probe asks a different question
// about them: which of the three is a property of the VALUE, and which is a
// property of the SITE that consumes the value.
//
//   POLICY   travels with the value. A value whose overflow semantics is
//            wrapping stays wrapping when it is passed into a function that
//            knows nothing about where it came from. The callee cannot supply
//            it, so it has to be in the type.
//
//   COST     is a property of the site. Which arm is cheapest depends on this
//            loop's arity, this target's features, this call's access pattern.
//            The value has no opinion.
//
//   LICENCE  is a property of the site too. Whether THIS fold may be split into
//            lanes is a fact about the fold, not about the numbers in it.
//
// The claim under test: if COST and LICENCE are supplied at the call site
// rather than by the value's type, then (a) everything still reaches one
// lowered path, and (b) the SAME value can be folded two different ways at two
// different sites with no cast between them.
//
// (b) is the discriminator. Under type-carried cost, changing the arm means
// changing the value's type, which means a cast. That cast is free at runtime
// and is not free in the design: it puts a conversion in the source that says
// the value changed, when the only thing that changed was the plan.
//
// Build:
//   rustc --edition 2024 -O --emit asm -C panic=abort -o f_where_carried.s f_where_carried.rs

#![no_std]
#![crate_type = "lib"]

// ---------------------------------------------------------------------------
// The value's type carries the policy, and only the policy.
// ---------------------------------------------------------------------------

pub trait Policy {
    const SATURATING: bool;
}
pub struct Wrapping;
pub struct Saturating;
impl Policy for Wrapping {
    const SATURATING: bool = false;
}
impl Policy for Saturating {
    const SATURATING: bool = true;
}

/// The numeral. One type parameter, and it is the policy.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Num<const W: u32, P: Policy>(u32, core::marker::PhantomData<P>);

impl<const W: u32, P: Policy> Num<W, P> {
    pub const fn from_raw(x: u32) -> Self {
        Self(x, core::marker::PhantomData)
    }
    pub const fn raw(self) -> u32 {
        self.0
    }
    pub const fn limit() -> u32 {
        if W >= 32 { u32::MAX } else { (1u32 << W) - 1 }
    }
}

// ---------------------------------------------------------------------------
// The site carries the plan: which measurement it weighs and which rewrites it
// is allowed to invoke. Neither is a fact about the numbers.
// ---------------------------------------------------------------------------

pub trait Plan {
    /// 0 = smallest working set, 1 = widest accumulator, 2 = fewest cycles.
    const PREFER: u32;
    const MAY_REASSOCIATE: bool;
}

pub struct Tight;
pub struct Fast;
pub struct Faithful;

impl Plan for Tight {
    const PREFER: u32 = 0;
    const MAY_REASSOCIATE: bool = false;
}
impl Plan for Fast {
    const PREFER: u32 = 2;
    const MAY_REASSOCIATE: bool = true;
}
impl Plan for Faithful {
    const PREFER: u32 = 1;
    const MAY_REASSOCIATE: bool = false;
}

// ---------------------------------------------------------------------------
// Shared arms.
// ---------------------------------------------------------------------------

#[inline(never)]
pub fn arm_tight_sat(v: &[u32], limit: u32) -> u32 {
    let mut a: u32 = 0;
    for &x in v {
        a = a.saturating_add(x).min(limit);
    }
    a
}
#[inline(never)]
pub fn arm_tight_wrap(v: &[u32], limit: u32) -> u32 {
    let mut a: u32 = 0;
    for &x in v {
        a = a.wrapping_add(x) & limit;
    }
    a
}
#[inline(never)]
pub fn arm_wide_sat(v: &[u32], limit: u32) -> u32 {
    let mut a: u64 = 0;
    for &x in v {
        a += x as u64;
    }
    if a > limit as u64 { limit } else { a as u32 }
}
#[inline(never)]
pub fn arm_wide_wrap(v: &[u32], limit: u32) -> u32 {
    let mut a: u64 = 0;
    for &x in v {
        a += x as u64;
    }
    (a as u32) & limit
}
#[inline(never)]
pub fn arm_lanes_sat(v: &[u32], limit: u32) -> u32 {
    let mut p: [u64; 4] = [0; 4];
    let c = v.len() / 4;
    for i in 0..c {
        p[0] += v[i * 4] as u64;
        p[1] += v[i * 4 + 1] as u64;
        p[2] += v[i * 4 + 2] as u64;
        p[3] += v[i * 4 + 3] as u64;
    }
    let mut a = p[0] + p[1] + p[2] + p[3];
    let mut i = c * 4;
    while i < v.len() {
        a += v[i] as u64;
        i += 1;
    }
    if a > limit as u64 { limit } else { a as u32 }
}
#[inline(never)]
pub fn arm_lanes_wrap(v: &[u32], limit: u32) -> u32 {
    let mut p: [u64; 4] = [0; 4];
    let c = v.len() / 4;
    for i in 0..c {
        p[0] += v[i * 4] as u64;
        p[1] += v[i * 4 + 1] as u64;
        p[2] += v[i * 4 + 2] as u64;
        p[3] += v[i * 4 + 3] as u64;
    }
    let mut a = p[0] + p[1] + p[2] + p[3];
    let mut i = c * 4;
    while i < v.len() {
        a += v[i] as u64;
        i += 1;
    }
    (a as u32) & limit
}

// ---------------------------------------------------------------------------
// The fold. The value's policy comes from the slice's element type; the plan
// comes from the call site. Both are monomorphisation-time constants and the
// whole selection is a const decision tree over them.
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn fold<const W: u32, P: Policy, L: Plan>(v: &[Num<W, P>]) -> Num<W, P> {
    // repr(transparent) makes this reinterpretation the identity at runtime; a
    // shipped design would put this behind whatever the typed unwrap door is.
    let raw: &[u32] = unsafe { core::slice::from_raw_parts(v.as_ptr() as *const u32, v.len()) };
    let limit = Num::<W, P>::limit();

    let r = if P::SATURATING {
        match L::PREFER {
            0 => arm_tight_sat(raw, limit),
            1 => arm_wide_sat(raw, limit),
            _ => {
                if L::MAY_REASSOCIATE {
                    arm_lanes_sat(raw, limit)
                } else {
                    arm_wide_sat(raw, limit)
                }
            }
        }
    } else {
        match L::PREFER {
            0 => arm_tight_wrap(raw, limit),
            1 => arm_wide_wrap(raw, limit),
            _ => {
                if L::MAY_REASSOCIATE {
                    arm_lanes_wrap(raw, limit)
                } else {
                    arm_wide_wrap(raw, limit)
                }
            }
        }
    };
    Num::from_raw(r)
}

// ---------------------------------------------------------------------------
// THE DISCRIMINATOR. One value type. Three sites. No cast anywhere, and each
// site gets a different arm.
// ---------------------------------------------------------------------------

pub type Sat13 = Num<13, Saturating>;

#[unsafe(no_mangle)]
pub fn site_tight(v: &[Sat13]) -> Sat13 {
    fold::<13, Saturating, Tight>(v)
}

#[unsafe(no_mangle)]
pub fn site_fast(v: &[Sat13]) -> Sat13 {
    fold::<13, Saturating, Fast>(v)
}

#[unsafe(no_mangle)]
pub fn site_faithful(v: &[Sat13]) -> Sat13 {
    fold::<13, Saturating, Faithful>(v)
}

/// And the policy still travels with the value, which is the half that cannot
/// move to the site: a wrapping value folded under the same plan gets the
/// wrapping arm, and no plan can override it.
pub type Wrap13 = Num<13, Wrapping>;

#[unsafe(no_mangle)]
pub fn site_fast_wrapping(v: &[Wrap13]) -> Wrap13 {
    fold::<13, Wrapping, Fast>(v)
}
