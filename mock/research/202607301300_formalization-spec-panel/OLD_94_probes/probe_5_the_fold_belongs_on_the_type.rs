//! Probe 5: the receipt's fold belongs on the type as a defaulted associated
//! const, not in a `const fn` called from value position.
//!
//! Probe 4 derived the receipt from the environment's field set through a
//! `const fn`. The pricing pillar warns that a `const fn` in value position
//! "folds or does not fold at the optimiser's discretion" and that a quantity
//! which is a function of the type's parameters alone "belongs on the type as
//! an associated const" (`91:114-121`). This probe checks whether the correct
//! shape is expressible on the permitted feature set.
//!
//! The load-bearing evidence is the `const _: () = assert!(...)` below. An
//! assertion in const position cannot pass unless the fold was evaluated at
//! compile time, so it distinguishes "the optimiser happened to fold it" from
//! "the language guarantees it folded", which is exactly the distinction the
//! pricing pillar's clause is about.
//!
//! Separation statement per `86b`: at a single-field environment the const-fn
//! form and the associated-const form are indistinguishable in emitted code
//! and in evaluation guarantee. The three-field IEEE set is where the fold is
//! a real computation, so that is what this instantiates.
//!
//! Build:
//!   rustc --edition 2021 --target aarch64-apple-darwin --crate-type=lib -O \
//!         --emit=asm -o probe_5_aarch64.s probe_5_the_fold_belongs_on_the_type.rs
//!
//! Result: compiles with no feature gates; both const assertions hold; the
//! emitted body is 4 instructions (`mrs`, `mov`, `tst`, `cset`), identical to
//! probe 4's derived form.

#![no_std]
#[derive(Clone, Copy)]
pub struct Field {
    pub mask: u64,
    pub expected: u64,
}

pub trait FloatEnv {
    const FIELDS: &'static [Field];
    // the pricing pillar's shape: the fold lives ON the type, in a const
    // position, not as a const fn called from value position.
    const MASK: u64 = {
        let mut m = 0u64;
        let mut i = 0;
        while i < Self::FIELDS.len() {
            m |= Self::FIELDS[i].mask;
            i += 1;
        }
        m
    };
    const EXPECTED: u64 = {
        let mut e = 0u64;
        let mut i = 0;
        while i < Self::FIELDS.len() {
            e |= Self::FIELDS[i].expected;
            i += 1;
        }
        e
    };
}

pub struct IeeeDefault;
impl FloatEnv for IeeeDefault {
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

const _: () = assert!(IeeeDefault::MASK == 0x1C80000);
const _: () = assert!(IeeeDefault::EXPECTED == 0);

#[no_mangle]
pub extern "C" fn receipt_assoc() -> bool {
    let v: u64;
    unsafe { core::arch::asm!("mrs {0}, fpcr", out(reg) v, options(nomem, nostack)) };
    (v & IeeeDefault::MASK) == IeeeDefault::EXPECTED
}
