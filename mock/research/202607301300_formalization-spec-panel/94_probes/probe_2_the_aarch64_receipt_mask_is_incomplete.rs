//! Probe 2: is `90_probes/probe_1`'s own field set complete on its own target?
//!
//! File 90's receipt masks FPCR RMode [23:22] and FZ [24] and calls that the
//! IEEE default environment (`90:198-204`). The ninth consolidation carries
//! the same two fields as the denotation of `IeeeDefault`: "round-to-nearest-
//! even, gradual underflow, no flush-to-zero" (`91:856-857`).
//!
//! Two more FPCR fields bear on that bundle and neither is in the mask:
//!   FZ16, bit [19]: flush-to-zero for half-precision. The design commits to
//!     the IEEE interchange rows, binary16 among them (`91:519-520`, the
//!     hardware-door precondition quantified over "every IEEE interchange
//!     row"), so a half-precision numeral under FZ16 = 1 does not have
//!     gradual underflow while the receipt reports that it does.
//!   FIZ, bit [0] (FEAT_AFP, Armv8.7): flush denormal INPUTS to zero, the
//!     aarch64 analogue of x86's DAZ. Same defect as probe 1's transliterated
//!     x86 form, on the architecture the receipt was written for.
//!
//! This probe does not argue either point. It writes each bit, reads FPCR
//! back, and reports which bits this host actually latches, then reports
//! whether file 90's mask would have caught each latched divergence.
//!
//! Separation statement per `86b`: the two masks (file 90's, and one covering
//! every underflow-relevant field) coincide on any process that touches only
//! RMode and FZ, which is every process file 90's probe created. This probe
//! instantiates where they differ.
//!
//! Run: rustc -O probe_2_the_aarch64_receipt_mask_is_incomplete.rs -o /tmp/p2 && /tmp/p2

#[cfg(target_arch = "aarch64")]
fn read_fpcr() -> u64 {
    let v: u64;
    unsafe { core::arch::asm!("mrs {0}, fpcr", out(reg) v, options(nomem, nostack)) };
    v
}

#[cfg(target_arch = "aarch64")]
fn write_fpcr(v: u64) {
    unsafe { core::arch::asm!("msr fpcr, {0}", in(reg) v, options(nomem, nostack)) };
}

/// File 90's mask, verbatim in content: RMode [23:22] plus FZ [24].
const RECEIPT_MASK_FILE_90: u64 = (0b11 << 22) | (1 << 24);

/// Every FPCR field that bears on "round to nearest even, gradual underflow".
const RMODE: u64 = 0b11 << 22;
const FZ: u64 = 1 << 24;
const FZ16: u64 = 1 << 19;
const FIZ: u64 = 1 << 0; // FEAT_AFP
const AH: u64 = 1 << 1; // FEAT_AFP, alternate handling

#[cfg(target_arch = "aarch64")]
fn latches(base: u64, bit: u64) -> bool {
    write_fpcr(base | bit);
    let back = read_fpcr();
    write_fpcr(base);
    (back & bit) != 0
}

#[cfg(target_arch = "aarch64")]
fn main() {
    let base = read_fpcr();
    println!("fpcr on entry: {base:#018x}");
    println!(
        "file 90 receipt on entry: {}",
        if (base & RECEIPT_MASK_FILE_90) == 0 {
            "PASS"
        } else {
            "FAIL"
        }
    );

    for (name, bit, in_mask) in [
        ("RMode[23:22]", RMODE & (1 << 22), true),
        ("FZ[24]", FZ, true),
        ("FZ16[19]", FZ16, false),
        ("FIZ[0] (FEAT_AFP)", FIZ, false),
        ("AH[1] (FEAT_AFP)", AH, false),
    ] {
        let held = latches(base, bit);
        let caught = (bit & RECEIPT_MASK_FILE_90) != 0;
        println!(
            "{name:<20} latched_by_host={held:<5} in_file_90_mask={in_mask:<5} \
             receipt_would_catch={caught}"
        );
    }

    // The load-bearing case, run rather than argued: set every underflow-
    // relevant bit the host latches OUTSIDE file 90's mask, then ask the
    // receipt. If it says PASS, the receipt reports IEEE-default while the
    // environment is not the IEEE default environment.
    let mut outside: u64 = 0;
    for bit in [FZ16, FIZ] {
        if latches(base, bit) {
            outside |= bit;
        }
    }
    if outside == 0 {
        println!("\nno underflow-relevant bit outside the mask latches on this host;");
        println!("the incompleteness is architectural, not observable here.");
    } else {
        write_fpcr(base | outside);
        let dirty = read_fpcr();
        let verdict = if (dirty & RECEIPT_MASK_FILE_90) == 0 {
            "PASS"
        } else {
            "FAIL"
        };
        println!("\nfpcr with outside-mask bits set: {dirty:#018x}");
        println!("file 90 receipt says: {verdict}  (should be FAIL)");
        write_fpcr(base);
    }
    assert_eq!(read_fpcr(), base, "probe failed to restore fpcr");
    println!("\nfpcr restored: {:#018x}", read_fpcr());
}

#[cfg(not(target_arch = "aarch64"))]
fn main() {
    println!("aarch64-only probe");
}
