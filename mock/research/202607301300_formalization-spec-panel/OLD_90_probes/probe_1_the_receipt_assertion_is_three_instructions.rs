//! Probe 1: the debug receipt-assertion file 63 priced at "three instructions of
//! cfg-gated inline assembly" (63:604-605) is buildable, gate-free, on the pin.
//!
//! The claim under test is the annotate shape's cheap partial verifier for the
//! `IeeeDefault` name: read the live FP *control* register (FPCR, not the FPSR
//! status register file 89's probe 4 read) and compare its rounding-mode and
//! flush-to-zero fields against what the environment type declares. If this
//! compiles with no feature gate and the comparison can run in a debug
//! assertion, the annotate shape ships with a runtime spot-check today, before
//! any build layer exists; if it needs a gate or an unstable intrinsic, the
//! annotate shape's mitigation is weaker than file 63 priced it.
//!
//! Separation statement per `86b`: this probe separates "the declared control
//! state" from "the live control state", which coincide on every process that
//! never touches FPCR; the probe also deliberately diverges them (sets FTZ,
//! re-checks, restores) so the assertion is demonstrated to FAIL where the
//! name's claim is false, not only to pass where it is true.
//!
//! aarch64 FPCR fields (Arm ARM, secondary read): RMode = bits [23:22]
//! (0b00 = round to nearest even), FZ = bit 24 (flush-to-zero), FZ16 = bit 19.
//! IEEE 754 default environment: RNE, gradual underflow (FZ = 0).

#[cfg(target_arch = "aarch64")]
fn read_fpcr() -> u64 {
    let fpcr: u64;
    unsafe { core::arch::asm!("mrs {0}, fpcr", out(reg) fpcr) };
    fpcr
}

#[cfg(target_arch = "aarch64")]
fn write_fpcr(v: u64) {
    unsafe { core::arch::asm!("msr fpcr, {0}", in(reg) v) };
}

// what the environment type would declare, as consts: the denotation half.
const RMODE_MASK: u64 = 0b11 << 22;
const RMODE_RNE: u64 = 0b00 << 22;
const FZ_BIT: u64 = 1 << 24;

#[cfg(target_arch = "aarch64")]
fn env_matches_ieee_default(fpcr: u64) -> bool {
    (fpcr & RMODE_MASK) == RMODE_RNE && (fpcr & FZ_BIT) == 0
}

#[cfg(target_arch = "aarch64")]
fn main() {
    let initial = read_fpcr();
    println!("fpcr initial: {initial:#010x}");
    // 1. on a fresh process the ambient state matches the declared bundle.
    assert!(
        env_matches_ieee_default(initial),
        "fresh process does not match IEEE default env; fpcr = {initial:#x}"
    );
    println!("receipt assertion PASSES on fresh process");

    // 2. diverge: set flush-to-zero, the exact loimu-shaped interference
    //    file 64 names (a linked library enabling FTZ for its own hot loops).
    write_fpcr(initial | FZ_BIT);
    let dirtied = read_fpcr();
    println!("fpcr after FTZ set: {dirtied:#010x}");
    assert!(
        !env_matches_ieee_default(dirtied),
        "assertion failed to detect FTZ divergence"
    );
    println!("receipt assertion DETECTS the divergence");

    // 3. restore, re-check.
    write_fpcr(initial);
    assert!(env_matches_ieee_default(read_fpcr()));
    println!("restored; receipt assertion passes again");
}

#[cfg(not(target_arch = "aarch64"))]
fn main() {
    println!("probe is aarch64-only; the x86 form reads MXCSR instead");
}
