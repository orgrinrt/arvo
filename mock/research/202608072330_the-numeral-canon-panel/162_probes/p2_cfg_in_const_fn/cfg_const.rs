// 162 P2. `159` F159-2 argued that I15 cannot constrain a `cfg`-varying
// realisation map, because every single build satisfies I15 completely: one
// lowered path, no runtime check. That argument rests on a premise I took from
// `157` and never verified myself, namely that a `const fn` body may read
// `cfg` at all. `161` records F159-2 as one of two instances at TWO+
// INSTANCES, so the premise had better be mine as well as `157`'s.
//
// Verified here independently of `157`'s probe, which I read and did not
// re-run.
//
// NEGATIVE CONTROLS, stated before the run.
//   HAZARD must DIFFER between the two builds. If it does not, a `const fn`
//   cannot read `cfg`, F159-2's premise is false, and the finding is withdrawn
//   rather than defended.
//   CONTROL_STABLE, a const fn the build cannot reach, must be IDENTICAL in
//   both builds. If it moves, the builds differ for some other reason and the
//   hazard line shows nothing.
//   And `lowered` must contain no branch on a build value in either build,
//   which is the half that makes I15 blind to it.

pub const W: u32 = 13;
const MASK: u64 = (1u64 << W) - 1;

// A realisation map that reads the build. Its signature says it reads only
// its argument.
pub const fn realise(k: u64) -> u64 {
    if cfg!(feature = "alt_policy") {
        k & MASK
    } else if k > MASK {
        MASK
    } else {
        k
    }
}

// The control: a realisation map the build cannot reach.
pub const fn realise_stable(k: u64) -> u64 {
    if k > MASK {
        MASK
    } else {
        k
    }
}

pub const HAZARD: u64 = realise(MASK + 1);
pub const CONTROL_STABLE: u64 = realise_stable(MASK + 1);

#[unsafe(no_mangle)]
pub fn lowered(k: u64) -> u64 {
    realise(k)
}

fn main() {
    println!("  HAZARD          R(MAX+1) = {HAZARD}");
    println!("  CONTROL_STABLE  R(MAX+1) = {CONTROL_STABLE}");
}
