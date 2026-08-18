//! 162 P2. `159` F159-2 argued that I15 cannot constrain a `cfg`-varying
//! realisation map, because every single build satisfies I15 completely. That
//! argument rests on a premise I took from `157` and did not verify: that a
//! `const fn` body may read `cfg` at all. The candidate records F159-2 as one
//! of two instances at TWO+ INSTANCES, so the premise had better be mine as
//! well as `157`'s.
//!
//! This verifies it independently of `157`'s probe, which I read and did not
//! re-run.
//!
//! NEGATIVE CONTROL, stated before the run. `CONTROL_STABLE` is a const fn the
//! build cannot reach; it must return the same value under both builds. If it
//! moves, the two builds differ for some reason other than the `cfg` read and
//! the hazard line proves nothing. And `HAZARD` must DIFFER between builds; if
//! it does not, a `const fn` cannot read `cfg` and F159-2's premise is false,
//! in which case the finding is withdrawn rather than defended.

pub const W: u32 = 13;
const MASK: u64 = (1u64 << W) - 1;

/// A realisation map that reads the build. Its signature says it reads only
/// its argument.
pub const fn realise(k: u64) -> u64 {
    if cfg!(feature = "alt_policy") {
        k & MASK // wrapping
    } else if k > MASK {
        MASK // saturating
    } else {
        k
    }
}

/// The control: a realisation map the build cannot reach.
pub const fn realise_stable(k: u64) -> u64 {
    if k > MASK { MASK } else { k }
}

pub const HAZARD: u64 = realise(MASK + 1);
pub const CONTROL_STABLE: u64 = realise_stable(MASK + 1);

/// The second half of F159-2: whatever the build chooses, the emitted code is
/// one path with no runtime check, which is exactly what I15 asks for. If this
/// function compiled to a branch on the build, I15 would see the hazard.

