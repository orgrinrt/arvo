// Probe 3: does the fact schema erase, in the sense the acceptance
// criterion in 00_brief.md means it (no runtime tag survives lowering)?
//
// This is a qualitative compiled check, not a bench: it asks "does a
// runtime tag survive", not "how fast is it". No timing, no iteration
// loop, no performance claim. Per evidence-lives-in-the-repo-or-it-never-
// happened.md this is within what an ad-hoc compiled check may establish.
//
// Compile with:
//   rustc +nightly-2026-05-28 --edition 2024 --crate-type lib \
//     -C opt-level=3 --emit=asm p3_erasure_check.rs -o p3.s
// then read p3.s for the functions below: each should lower to a bare
// move/return with no extra field, no branch, no vtable load.

#![no_std]
#![allow(dead_code)]

include!("_shared_schema.rs");

// a lowering-site stand-in: takes a Cold<13> storage value, hands back the
// same bits. if PACKED/ALIGN/WIDTH survived as runtime state, this would
// have to carry or consult them; if they are erased, this is a bare copy.
#[unsafe(no_mangle)]
pub extern "C" fn cold13_storage_roundtrip(
    x: <Cold as NumeralFacts<Unsigned, 13>>::Storage,
) -> <Cold as NumeralFacts<Unsigned, 13>>::Storage {
    x
}

// same check for the case where Storage and Operand diverge: converting
// from Cold's packed-array storage form to its u16 operand form should be
// a fixed, fully-determined-at-compile-time bit reinterpretation, not a
// runtime dispatch over which strategy or width produced the bytes.
#[unsafe(no_mangle)]
pub extern "C" fn cold13_storage_to_operand(
    x: <Cold as NumeralFacts<Unsigned, 13>>::Storage,
) -> <Cold as NumeralFacts<Unsigned, 13>>::Operand {
    u16::from_le_bytes(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn precise13_operand_widen(
    x: <Precise as NumeralFacts<Unsigned, 13>>::Storage,
) -> <Precise as NumeralFacts<Unsigned, 13>>::Operand {
    x as u32
}
