// PROBE p4. The band mechanism of 80's p2c, applied to the SATURATING
// threshold family E_63 from p3. This is the compiled form of the answer to
// 84 section 11's open search: the defeat is not a ring-fragment artifact.
//
// The construction mirrors 80_probes/p2c_closed_form_checked_on_a_model.rs
// and 84_probes/p2_defeat_the_cross_check.rs exactly, with the law exchanged
// for E_63: forall x: x^63 == x^64 under unsigned saturating multiplication.
//
//   - the closed form is the plausible generalisation "saturating powers of
//     any x >= 2 collapse to MAX, and 0, 1 are fixed points, so consecutive
//     high powers agree at every width": constant true. The reasoning is
//     right at every width where 2^63 clamps, wrong by exactly one width
//     among the reachable ones: at width 64, x = 2 gives 2^63 unclamped
//     against sat(2^64) = MAX.
//   - the model band 2..=8 sweeps the law exhaustively through the
//     saturating map at compile time and asserts agreement with the closed
//     form: GREEN, and no band below width 64 could do otherwise, because
//     the truth set is exactly widths 1..=63 (p3).
//   - the perturbation control (--cfg badclosed) still refuses, so the
//     mechanism is not broken; it is checking the wrong thing.
//   - the arm gated on the closed form at width 64 is LICENSED, and the
//     licensed law is FALSE at width 64.
//   - the audit build (--cfg audit) adds the one check the mechanism lacks,
//     the law itself at the gated width on the known witness x = 2, and
//     refuses in ~126 const multiplies.
//
// The difference criterion CANNOT replace the band here: sat_mul is not a
// ring term (p4 of 84, first negative control), so for this family there is
// no procedure row available at all. Witness rows still work (falsity at 64
// is one evaluation), but nothing can certify saturating TRUTH at a width
// the sweep cannot reach, short of a per-law structural argument.
//
// Toolchain: pinned nightly-2026-05-28. The gate constructions are const fns
// with no feature gates, no dyn, no TypeId, no allocation; main() is print
// scaffolding only.

const fn umax(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

const fn sat_mul_w(a: u64, b: u64, w: u32) -> u64 {
    let m = umax(w);
    match a.checked_mul(b) {
        None => m,
        Some(s) => {
            if s > m {
                m
            } else {
                s
            }
        }
    }
}

/// x^d under width-w saturating multiplication, left fold.
const fn sat_pow(x: u64, d: u32, w: u32) -> u64 {
    let mut acc = x;
    let mut i = 1;
    while i < d {
        acc = sat_mul_w(acc, x, w);
        i += 1;
    }
    acc
}

/// The swept verdict for E_63 at width w: forall x mod 2^w: x^63 == x^64.
/// Exhaustive through the saturating map; usable at band widths only.
const fn swept_verdict(w: u32) -> bool {
    let n = 1u64 << w;
    let mut x = 0u64;
    while x < n {
        if sat_pow(x, 63, w) != sat_pow(x, 64, w) {
            return false;
        }
        x += 1;
    }
    true
}

/// The closed form an arm gates on. The generalisation reads soundly: every
/// x >= 2 has x^63 >= 2^63, "which saturates", and 0 and 1 are fixed points.
/// It is wrong at exactly the widths where 2^63 does not saturate.
#[cfg(not(badclosed))]
const fn closed_verdict(_w: u32) -> bool {
    true
}

/// The perturbation control: one band entry wrong.
#[cfg(badclosed)]
const fn closed_verdict(w: u32) -> bool {
    w != 5
}

/// The band cross-check, a crate-level const, unskippable (rung 0).
const BAND_LO: u32 = 2;
const BAND_HI: u32 = 8;
const BAND_AGREEMENT: () = {
    let mut w = BAND_LO;
    while w <= BAND_HI {
        assert!(
            closed_verdict(w) == swept_verdict(w),
            "the closed-form law verdict disagrees with the swept verdict somewhere in \
             the model band, so the closed form is wrong and no arm may be gated on it"
        );
        w += 1;
    }
};

/// The licence at the shipped width, gated on the closed form (rung 3 shape,
/// forced through a top-level const here so the transcript covers it).
const LICENSED_AT_64: bool = closed_verdict(64);

/// The audit the mechanism does not contain: the law itself at the gated
/// width, on the witness p3 pinned. Refuses when enabled.
#[cfg(audit)]
const AUDIT: () = assert!(
    sat_pow(2, 63, 64) == sat_pow(2, 64, 64),
    "the licensed law is FALSE at the gated width: 2^63 does not saturate at \
     width 64, and sat(2^64) = MAX != 2^63"
);

fn main() {
    let _ = BAND_AGREEMENT;
    println!("p4: the band mechanism licensing a false SATURATING law at width 64");
    println!("  model band: widths {}..={}", BAND_LO, BAND_HI);
    println!("  agreement over the band, evaluated at compile time: true");
    println!(
        "  closed_verdict(w = 64) = {}   (constant time, no enumeration)",
        closed_verdict(64)
    );
    println!(
        "  arm at width 64: {}",
        if LICENSED_AT_64 {
            "licensed"
        } else {
            "refused"
        }
    );
    println!("  and the licensed law is false at width 64, witness x = 2:");
    println!(
        "    sat_pow(2, 63, 64) = {:#x}, sat_pow(2, 64, 64) = {:#x}",
        sat_pow(2, 63, 64),
        sat_pow(2, 64, 64)
    );
    assert!(sat_pow(2, 63, 64) != sat_pow(2, 64, 64));
    println!("  (--cfg badclosed refuses at the band; --cfg audit refuses at width 64)");
}
