//! Probe 4: if the environment type's denotation is a field set, the receipt
//! is a fold over that field set, not a hand-written sequence per target.
//!
//! Probes 1 and 2 established two defects in the hand-written receipt: the
//! aarch64 mask omits FZ16, which this host latches (probe 2), and the x86
//! form is a different register with a field (DAZ) that has no counterpart in
//! the aarch64 mask plus a second register (x87 FCW) with a field (PC) that
//! has no counterpart on aarch64 at all (probe 1). Both are the same defect:
//! the receipt was written, per target, by hand, so it can disagree with the
//! bundle the name denotes and nothing catches the disagreement.
//!
//! The pricing pillar's standing test (`91:117-121`): "is a quantity computed
//! inside a per-element or per-step loop a function of the type's parameters
//! alone? If so it belongs on the type as an associated const". The receipt's
//! mask and expected value are functions of the environment type and the
//! target alone. This probe puts them there and asks what it costs.
//!
//! The claim under test is a cost claim, so it is checked in emitted assembly
//! rather than asserted: does routing the receipt through an associated const
//! emit the same three instructions file 90 priced (`90:203`)?
//!
//! Separation statement per `86b`: the hand-written receipt and the derived
//! receipt emit identical code at the one environment everyone writes
//! (`IeeeDefault` on aarch64), which is where a cost check would be run. The
//! probe therefore also instantiates a SECOND environment whose field set
//! differs, so that "the derived form tracks the denotation" is checked where
//! the two forms come apart rather than only where they agree.
//!
//! Build (asm):
//!   rustc --edition 2021 --target aarch64-apple-darwin --crate-type=lib -O \
//!         --emit=asm -o probe_4_aarch64.s probe_4_the_receipt_derives_from_the_denotation.rs
//!   rustc --edition 2021 --target x86_64-apple-darwin --crate-type=lib -O \
//!         --emit=asm -o probe_4_x86.s probe_4_the_receipt_derives_from_the_denotation.rs

#![no_std]

/// One control-register field the environment's correctness is conditional on.
/// `mask` selects the bits, `expected` is the value the environment declares
/// them to hold. Both are per target, because the same abstract bundle is a
/// different bit layout on each.
#[derive(Clone, Copy)]
pub struct Field {
    pub mask: u64,
    pub expected: u64,
}

/// An environment parameter's denotation, stated as data rather than as prose.
/// The receipt is derived from `FIELDS`; nothing about it is hand-written per
/// environment, so an environment that adds a field cannot keep passing a
/// receipt that does not check it.
pub trait FloatEnv {
    const NAME: &'static str;
    /// Every field of the ambient control state this environment assumes.
    /// A target that has no field for a member of the abstract bundle states
    /// a zero mask, which is a claim the target cannot express the field, not
    /// a claim the field is satisfied.
    const FIELDS: &'static [Field];
}

/// The IEEE 754 default environment, per target.
pub struct IeeeDefault;

#[cfg(target_arch = "aarch64")]
impl FloatEnv for IeeeDefault {
    const NAME: &'static str = "IeeeDefault";
    // FPCR. RMode [23:22] = 00 (RNE), FZ [24] = 0, FZ16 [19] = 0.
    // FZ16 is the field probe 2 found missing from the hand-written mask and
    // demonstrated to latch on this host.
    const FIELDS: &'static [Field] = &[
        Field {
            mask: 0b11 << 22,
            expected: 0,
        },
        Field {
            mask: 1 << 24,
            expected: 0,
        },
        Field {
            mask: 1 << 19,
            expected: 0,
        },
    ];
}

#[cfg(target_arch = "x86_64")]
impl FloatEnv for IeeeDefault {
    const NAME: &'static str = "IeeeDefault";
    // MXCSR. RC [14:13] = 00 (RNE), FTZ [15] = 0, DAZ [6] = 0.
    // DAZ is the field probe 1's transliterated form omitted.
    const FIELDS: &'static [Field] = &[
        Field {
            mask: 0b11 << 13,
            expected: 0,
        },
        Field {
            mask: 1 << 15,
            expected: 0,
        },
        Field {
            mask: 1 << 6,
            expected: 0,
        },
    ];
}

/// A second environment, so the derived form is checked where the two forms
/// come apart. This one declares round-toward-zero with flush-to-zero, which
/// is a real deployed configuration and is NOT the IEEE default environment.
pub struct FastMathEnv;

#[cfg(target_arch = "aarch64")]
impl FloatEnv for FastMathEnv {
    const NAME: &'static str = "FastMathEnv";
    const FIELDS: &'static [Field] = &[
        Field {
            mask: 0b11 << 22,
            expected: 0b11 << 22,
        }, // RMode = toward zero
        Field {
            mask: 1 << 24,
            expected: 1 << 24,
        }, // FZ set
        Field {
            mask: 1 << 19,
            expected: 1 << 19,
        }, // FZ16 set
    ];
}

#[cfg(target_arch = "x86_64")]
impl FloatEnv for FastMathEnv {
    const NAME: &'static str = "FastMathEnv";
    const FIELDS: &'static [Field] = &[
        Field {
            mask: 0b11 << 13,
            expected: 0b11 << 13,
        },
        Field {
            mask: 1 << 15,
            expected: 1 << 15,
        },
        Field {
            mask: 1 << 6,
            expected: 1 << 6,
        },
    ];
}

/// The whole denotation folded to one mask and one expected value, at compile
/// time. This is the pricing pillar's own move: the fold is a function of the
/// type's parameters alone, so it happens once, in a const position, and never
/// at the check site.
pub const fn folded<E: FloatEnv>() -> (u64, u64) {
    let mut mask = 0u64;
    let mut expected = 0u64;
    let mut i = 0;
    while i < E::FIELDS.len() {
        mask |= E::FIELDS[i].mask;
        expected |= E::FIELDS[i].expected;
        i += 1;
    }
    (mask, expected)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn read_control() -> u64 {
    let v: u64;
    unsafe { core::arch::asm!("mrs {0}, fpcr", out(reg) v, options(nomem, nostack)) };
    v
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn read_control() -> u64 {
    let mut csr: u32 = 0;
    unsafe { core::arch::asm!("stmxcsr [{0}]", in(reg) &mut csr, options(nostack)) };
    csr as u64
}

/// The derived receipt. One generic body, every target, every environment.
#[inline(always)]
pub fn receipt<E: FloatEnv>() -> bool {
    let (mask, expected) = folded::<E>();
    (read_control() & mask) == expected
}

// Monomorphised entry points, so the emitted code is inspectable per instance.

#[no_mangle]
#[inline(never)]
pub extern "C" fn receipt_ieee_default() -> bool {
    receipt::<IeeeDefault>()
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn receipt_fast_math() -> bool {
    receipt::<FastMathEnv>()
}

/// The hand-written aarch64 form from `90_probes/probe_1`, for the cost
/// comparison. Two fields, because that is what the hand-written mask had.
#[cfg(target_arch = "aarch64")]
#[no_mangle]
#[inline(never)]
pub extern "C" fn receipt_hand_written_file_90() -> bool {
    let v = read_control();
    (v & ((0b11 << 22) | (1 << 24))) == 0
}

/// The folded constants are available at compile time, which is what lets a
/// declaration site state its own denotation and a diagnostic print it.
pub const IEEE_MASK: u64 = folded::<IeeeDefault>().0;
pub const IEEE_EXPECTED: u64 = folded::<IeeeDefault>().1;
pub const FAST_MASK: u64 = folded::<FastMathEnv>().0;
pub const FAST_EXPECTED: u64 = folded::<FastMathEnv>().1;

// The two environments must not fold to the same check. If they ever do, the
// name distinguishes two things the receipt cannot.
const _: () = assert!(IEEE_EXPECTED != FAST_EXPECTED);
// And the derived IEEE mask must be strictly wider than file 90's hand-written
// one, which is probe 2's finding restated where the compiler can hold it.
#[cfg(target_arch = "aarch64")]
const _: () = assert!(IEEE_MASK != ((0b11 << 22) | (1 << 24)));
#[cfg(target_arch = "aarch64")]
const _: () = assert!(IEEE_MASK & ((0b11 << 22) | (1 << 24)) == ((0b11 << 22) | (1 << 24)));
