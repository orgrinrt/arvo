//! Probe 2. The zero-mask clause's honest completion is a verdict split, not
//! a declaration refusal. The receipt's checked and unchecked field sets are
//! both const-derivable from the per-target field set (file 94's fold shape,
//! extended by one per-field flag), the unchecked set is surfaced in the
//! type where a consumer must acknowledge it in const position, and the
//! emitted receipt body is unchanged, because all of the bookkeeping folds
//! at compile time.
//!
//! Separation statement per 86b: this model separates "field unchecked" from
//! "field holds" at a target with a genuinely uncheckable field (the x87
//! precision-control shape file 94 found). At an all-checkable target the
//! two verdict shapes coincide, which is why the partial target is the
//! instantiation that matters.
//!
//! Build (asm): rustc --edition 2021 --target aarch64-apple-darwin \
//!   --crate-type=lib -O --emit=asm -o out/probe_2_aarch64.s \
//!   probe_2_receipt_verdict_split.rs
//! Build (run): rustc --edition 2021 -O --cfg run_it \
//!   probe_2_receipt_verdict_split.rs -o out/probe_2

#![cfg_attr(not(run_it), no_std)]

#[derive(Clone, Copy)]
pub struct Field {
    pub mask: u64,
    pub expected: u64,
    /// false = this target cannot observe the field (the zero-mask case,
    /// carried as a flag so the field keeps its identity when unobservable)
    pub checkable: bool,
    pub id: u32,
}

pub trait FloatEnv {
    const FIELDS: &'static [Field];

    const CHECKED_MASK: u64 = {
        let mut m = 0u64;
        let mut i = 0;
        while i < Self::FIELDS.len() {
            if Self::FIELDS[i].checkable {
                m |= Self::FIELDS[i].mask;
            }
            i += 1;
        }
        m
    };
    const EXPECTED: u64 = {
        let mut e = 0u64;
        let mut i = 0;
        while i < Self::FIELDS.len() {
            if Self::FIELDS[i].checkable {
                e |= Self::FIELDS[i].expected;
            }
            i += 1;
        }
        e
    };
    /// The unchecked set as a bitset of field ids: the honest residue the
    /// boolean verdict was silently absorbing. Const, greppable, assertable.
    const UNCHECKED_IDS: u32 = {
        let mut u = 0u32;
        let mut i = 0;
        while i < Self::FIELDS.len() {
            if !Self::FIELDS[i].checkable {
                u |= 1 << Self::FIELDS[i].id;
            }
            i += 1;
        }
        u
    };
}

pub const ID_RMODE: u32 = 0;
pub const ID_FZ: u32 = 1;
pub const ID_FZ16: u32 = 2;
pub const ID_PC: u32 = 3; // precision control: exists on x87, no aarch64 form

/// The aarch64 bundle: all three FPCR fields checkable, PC not expressible
/// and declared so rather than silently omitted.
pub struct IeeeDefaultAarch64;
impl FloatEnv for IeeeDefaultAarch64 {
    const FIELDS: &'static [Field] = &[
        Field {
            mask: 0b11 << 22,
            expected: 0,
            checkable: true,
            id: ID_RMODE,
        },
        Field {
            mask: 1 << 24,
            expected: 0,
            checkable: true,
            id: ID_FZ,
        },
        Field {
            mask: 1 << 19,
            expected: 0,
            checkable: true,
            id: ID_FZ16,
        },
        Field {
            mask: 0,
            expected: 0,
            checkable: false,
            id: ID_PC,
        },
    ];
}

// The consumer's acknowledgment of the unchecked residue is explicit and
// const: it cannot silently widen, because adding an unchecked field to the
// bundle changes UNCHECKED_IDS and this assertion refuses until the
// acknowledgment names the new field too.
const _: () = assert!(IeeeDefaultAarch64::UNCHECKED_IDS == 1 << ID_PC);
const _: () = assert!(IeeeDefaultAarch64::CHECKED_MASK == 0x1C80000);
const _: () = assert!(IeeeDefaultAarch64::EXPECTED == 0);

/// The receipt body: identical shape to 94_probes/probe_5, over the checked
/// mask only. The verdict-split bookkeeping costs zero instructions here,
/// because the unchecked set never reaches value position.
#[cfg(all(target_arch = "aarch64", not(run_it)))]
#[no_mangle]
pub extern "C" fn receipt_checked() -> bool {
    let v: u64;
    unsafe { core::arch::asm!("mrs {0}, fpcr", out(reg) v, options(nomem, nostack)) };
    (v & IeeeDefaultAarch64::CHECKED_MASK) == IeeeDefaultAarch64::EXPECTED
}

#[cfg(run_it)]
fn main() {
    println!("CHECKED_MASK = {:#x}", IeeeDefaultAarch64::CHECKED_MASK);
    println!(
        "UNCHECKED_IDS = {:#b} (bit {} = precision control)",
        IeeeDefaultAarch64::UNCHECKED_IDS,
        ID_PC
    );
    println!(
        "boolean-shape receipt would report: true; split shape reports: (checked ok, {} field unchecked)",
        IeeeDefaultAarch64::UNCHECKED_IDS.count_ones()
    );
}
