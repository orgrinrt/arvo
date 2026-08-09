//! Five target-scoped codegen regression tests, committed as artifacts rather
//! than as a recommendation, per the persona checkpoint's fourth item
//! (`49_consolidation_four.md:773, 814-821`) and its own naming of the
//! standing finding these pin: "a dependency on an optimiser heuristic
//! holding, not a guarantee" (`26_consolidation_two.md:452-457`, restated at
//! `40:662-665` and `49:814-816`).
//!
//! DESTINATION (on move): `mock/crates/arvo/tests/codegen_regression.rs`,
//! fixtures at `mock/crates/arvo/tests/codegen/*.rs`. Every fixture here is
//! `#![no_std]` and free-standing (no `use arvo::...`) because the design
//! these five facts are about (`mul_full`, the biased-product accumulator,
//! `div_floor`/`rem`, `Precise`'s saturating resolution, `WideBits` carry
//! chains) has no shipped source yet (`grep -rln "Adjustment\|Bias\|Numeral"
//! mock/crates/ --include="*.rs"` returns nothing, reproduced in
//! `52_ringer_the_tests_that_were_owed.md`). On the move, each fixture's
//! hand-written stand-in (`a.wrapping_mul(b)` for Hot's direct multiply,
//! the schoolbook 128-bit product for `mul_full`, `saturating_add` for a
//! `Precise`-resolution fold, `carrying_add` for a `WideBits` limb chain,
//! `wrapping_div`/`wrapping_rem` for `div_floor`/`rem`) is replaced by the
//! real arvo call, and the assertions are UNCHANGED: they pin instruction
//! shapes and counts, not which source line produced them.
//!
//! WHAT EACH TEST PINS, AND WHAT GREEN AND RED MEAN. This is a MEASUREMENT
//! suite, not a CONTRACT suite (52_ringer's own section 1 states the
//! distinction and why it matters here specifically). Every assertion below
//! is "the optimiser currently does X on this target"; none is "the design
//! requires X". A red run after a toolchain bump is NEWS, to be read and
//! re-recorded (does the new number still satisfy the design's cost promise,
//! or has a preset silently gotten slower), never patched by loosening the
//! assertion to whatever the new compiler emits. A red run with the pin
//! and target UNCHANGED is a regression in this harness or in the fixture,
//! full stop.
//!
//! TARGET SCOPE. All five are `aarch64-apple-darwin` facts, `rustc
//! 1.98.0-nightly (57d06900f 2026-05-27)`, `-C opt-level=3 -C
//! codegen-units=1 -C panic=abort --emit=asm`, no other codegen flags. Test
//! 4 is flag-sensitive in a way worth stating loudly (section 4's own
//! doc comment): the same source vectorises differently under the
//! rustc-default `codegen-units=16` than under `codegen-units=1`, so a
//! CI invocation of this harness that does not pin the flag is not running
//! the same experiment the design's own claim was measured under.
//!
//! Every test resolves `rustc` explicitly via `RUSTC` (falling back to the
//! `rustc` on `PATH`, which resolves correctly only when the harness itself
//! runs from inside a toolchain-pinned tree, i.e. after this file lands in
//! `mock/crates/arvo/tests/` where the repo's own `rust-toolchain.toml`
//! governs). Resolving it implicitly from outside such a tree is exactly
//! what silently ran this dispatch's first draft of every one of these five
//! tests against the machine's stable 1.94.0 rather than the pin, an error
//! this file's own OUTCOMES.md records finding and correcting.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const FIXTURE_DIR: &str = "codegen"; // sibling to this file once it ships

fn rustc_path() -> String {
    env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string())
}

fn require_target() {
    // These are `aarch64-apple-darwin` facts. A CI matrix running this
    // harness on another host target should skip it, not reinterpret its
    // assertions; the instruction shapes named below are architecture-
    // specific by construction (NEON mnemonics, aarch64 carry-flag
    // instructions).
    if !cfg!(target_arch = "aarch64") || !cfg!(target_os = "macos") {
        panic!(
            "codegen_regression: every assertion in this file is an \
             aarch64-apple-darwin fact; skip on other targets rather than \
             reinterpret the expected instruction counts"
        );
    }
}

fn emit_asm(fixture: &str, extra_flags: &[&str]) -> String {
    require_target();
    let out_dir = env::temp_dir().join("arvo_codegen_regression");
    fs::create_dir_all(&out_dir).expect("scratch dir");
    let src = PathBuf::from(FIXTURE_DIR).join(fixture);
    let out = out_dir.join(format!("{fixture}.s"));
    let mut cmd = Command::new(rustc_path());
    cmd.arg("--edition").arg("2021");
    cmd.arg("--crate-type").arg("lib");
    cmd.arg("-C").arg("opt-level=3");
    cmd.arg("-C").arg("codegen-units=1");
    cmd.arg("-C").arg("panic=abort");
    cmd.arg("--emit").arg("asm");
    cmd.arg(&src);
    cmd.arg("-o").arg(&out);
    for f in extra_flags {
        cmd.arg(f);
    }
    let status = cmd.status().expect("spawn rustc");
    assert!(status.success(), "fixture {fixture} failed to compile");
    fs::read_to_string(&out).expect("read emitted asm")
}

fn body_of<'a>(asm: &'a str, symbol: &str) -> &'a str {
    let start_marker = format!("_{symbol}:");
    let start = asm
        .find(&start_marker)
        .unwrap_or_else(|| panic!("symbol {symbol} not found in emitted asm"));
    let rest = &asm[start..];
    // stop at the first `ret` line, which is where every fixture's single
    // basic-block-shaped function ends; this is a fixture-shape assumption,
    // not a general disassembly parser.
    let end = rest
        .find("\tret\n")
        .map(|i| i + "\tret\n".len())
        .unwrap_or(rest.len());
    &rest[..end]
}

fn count(text: &str, needle: &str) -> usize {
    text.matches(needle).count()
}

/// Test 1: the multi-limb (`WideBits`) carry chain compiles clean, no
/// function calls, straight-line carry-propagating instructions. Pins
/// `35_dolan_does_widening_collapse.md:113-119`: "A 256-bit carry chain
/// already compiles cleanly because LLVM recognises the `carrying_add`
/// idiom... This is a dependency on an optimiser heuristic holding, not a
/// guarantee." `core::arch::aarch64` has no carry-propagating fallback
/// intrinsic (unlike x86_64's `_addcarry_u64`), so if idiom recognition
/// ever regresses on this target there is no cfg-gated escape hatch; that
/// absence is exactly why this fact needs a falsifiable pin rather than a
/// standing belief.
#[test]
fn multi_limb_carry_chain_compiles_to_straight_line_adc_no_calls() {
    let asm = emit_asm("carry_chain.rs", &[]);
    let body = body_of(&asm, "add256");
    assert_eq!(
        count(body, "\tbl\t"),
        0,
        "the 256-bit carrying_add chain must compile to a straight-line \
         sequence with no function calls; a call here means the idiom \
         recognition this test exists to pin has regressed"
    );
    let carry_instrs = count(body, "\tadcs\t") + count(body, "\tadc\t");
    assert_eq!(
        carry_instrs, 3,
        "a 4-limb carrying_add chain needs one plain `adds` (the first \
         limb, no incoming carry) and three carry-consuming instructions \
         (`adcs`/`adc`); a different count means LLVM stopped recognising \
         the idiom as one ripple-carry chain"
    );
}

/// Test 2a: `mul_full` at native width folds to the identical symbol as a
/// direct truncating multiply. Pins `35_dolan_does_widening_collapse.md:
/// 111-120`: "hot_mul_via_full_then_quantize and precise_mul_widens all
/// compile to the same symbol... LLVM folds all three before the codegen
/// shape even reaches the assembler."
#[test]
fn fold_vs_direct_multiply_native_width_folds_to_one_instruction() {
    let asm = emit_asm("mul_fold_native.rs", &[]);
    let direct = body_of(&asm, "hot_mul_direct");
    assert_eq!(
        count(direct, "\tmul\t"),
        1,
        "the direct native multiply must be exactly one `mul` instruction"
    );
    assert!(
        asm.contains("_hot_mul_via_full_then_quantize = _hot_mul_direct"),
        "the widen-then-truncate composite must fold to the SAME symbol as \
         the direct multiply (LLVM's own alias-emission, not merely \
         'similar instruction count'); its absence means the fold this \
         axis's zero-cost claim rests on no longer happens on this pin"
    );
}

/// Test 2b: `mul_full` at 128-bit (multi-limb) width compiles to the same
/// instruction SHAPE as a direct 128-bit multiply, up to commutative
/// operand order. Pins `35_dolan_does_widening_collapse.md:129-137`: "both
/// compile to four instructions (umulh, two madd, mul), the same shape up
/// to commutative operand order. The optimiser eliminates the hi_hi
/// limb-product and its carries."
#[test]
fn fold_vs_direct_multiply_multi_limb_width_matches_instruction_shape() {
    let asm = emit_asm("mul_fold_128.rs", &[]);
    let direct = body_of(&asm, "hot_128_direct");
    let full_then_quantize = body_of(&asm, "warm_mul_via_full_then_quantize_128");
    for (name, body) in [
        ("hot_128_direct", direct),
        ("warm_mul_via_full_then_quantize_128", full_then_quantize),
    ] {
        assert_eq!(
            count(body, "\tumulh\t") + count(body, "\tmadd\t") + count(body, "\tmul\t"),
            4,
            "{name} must compile to exactly four instructions (one umulh, \
             two madd, one mul); a different count means the schoolbook \
             mul_full path no longer folds down to the hardware-multiply \
             shape at multi-limb width and the design's Widening-removal \
             cost claim (49:280-289) needs re-measuring, not re-asserting"
        );
        assert_eq!(
            count(body, "\tsdiv\t") + count(body, "\tudiv\t") + count(body, "\tbl\t"),
            0,
            "{name} must contain no division and no call; either would \
             mean the 256-bit-intermediate path stopped collapsing to \
             register arithmetic"
        );
    }
}

/// Test 3: a `Precise`-resolution (saturating) fold does not vectorise;
/// the wrapping control (same shape, same width, no saturation) does.
/// Pins `35_dolan_does_widening_collapse.md:103-110`: "Saturating integer
/// reductions do not vectorise... unlike the float case, source-level
/// regrouping does NOT recover parallel lanes... there is no LLVM IR flag
/// for integer saturating arithmetic to grant in the first place."
#[test]
fn saturating_reduction_stays_scalar_wrapping_control_vectorises() {
    let asm = emit_asm("sat_reduce.rs", &[]);
    let sat = body_of(&asm, "sat_sum4");
    let wrap = body_of(&asm, "wrap_sum4");
    assert_eq!(
        count(sat, ".2d") + count(sat, ".4s") + count(sat, ".2s"),
        0,
        "sat_sum4 (a Precise-resolution-shaped saturating reduction) must \
         stay fully scalar; any NEON lane instruction here means arvo has \
         gained a licence (an LLVM IR flag or idiom) to regroup saturating \
         arithmetic that did not exist on this pin, which is worth a design \
         update, not a loosened test"
    );
    assert!(
        count(wrap, ".2d") + count(wrap, ".4s") + count(wrap, ".2s") > 0,
        "wrap_sum4 (the non-saturating control, identical shape) must \
         vectorise; if it stops vectorising the control has broken and \
         sat_sum4's zero above stops being informative"
    );
}

/// Test 4: the `assert!(a.len() == b.len() && ...)` equal-length loop idiom
/// defeats the vectoriser; the same indexing loop with no prior length
/// assertion vectorises. Pins `34_giesen_the_three_halves_assembled.md:
/// 120-125`. FLAG-SENSITIVE, and the sensitivity is itself part of what
/// this test pins: under the rustc-default `codegen-units=16` this
/// distinction DISAPPEARS on this pin (both idioms vectorise identically);
/// it reproduces only under `codegen-units=1`, which is why every
/// invocation of this harness fixes that flag rather than the rustc
/// default. `52_ringer`'s own OUTCOMES.md records the flag search that
/// found this and the fact that the review's own prose claim
/// (`34:122-124`) was never re-verified against its own corrected shape-A
/// methodology before this dispatch.
#[test]
fn assert_equal_length_idiom_defeats_vectoriser_bare_loop_does_not() {
    let asm = emit_asm("loop_idiom.rs", &[]);
    let asserted = body_of(&asm, "add_assert_idiom");
    let bare = body_of(&asm, "add_no_assert_idiom");
    assert_eq!(
        count(asserted, ".2d") + count(asserted, ".4s"),
        0,
        "add_assert_idiom (the `assert!(a.len()==b.len()&&...)` idiom) must \
         stay scalar under -C codegen-units=1; if it starts vectorising, \
         LLVM's loop-vectoriser legality analysis changed for this pattern \
         and the design's own posture ('do not lean on autovectorisation \
         as a guarantee', 32:325-334) just gained supporting evidence \
         rather than losing it, still worth recording as news"
    );
    assert!(
        count(bare, ".2d") + count(bare, ".4s") > 0,
        "add_no_assert_idiom (identical arithmetic, no prior length \
         assertion) must vectorise; if it stops, the control has broken \
         and the zero above stops being informative"
    );
}

/// Test 5: `div_floor` and `rem` fuse into one hardware divide when both
/// are computed on the same operands, per `43_smith_division.md:283-287`:
/// "the optimiser fuses the two calls into one hardware divide when both
/// are used (the fusion is a codegen-regression test to add to the four
/// the consolidation already owes, `40:662-665`; noted, not built)."
/// aarch64 has no combined div+rem instruction (unlike x86's `idiv`), so
/// the fusion this test pins is specifically "one `sdiv`, remainder via
/// `msub` from the already-computed quotient", not "one instruction
/// total".
#[test]
fn div_floor_and_rem_fuse_into_one_hardware_divide() {
    let asm = emit_asm("div_rem_fusion.rs", &[]);
    let combined = body_of(&asm, "div_floor_and_rem");
    assert_eq!(
        count(combined, "\tsdiv\t"),
        1,
        "div_floor_and_rem, computing both the floor quotient and the \
         Euclidean remainder from the same two operands in one call, must \
         emit exactly one sdiv; two would mean the fusion this test pins \
         has regressed and a fold's MAC-style div_floor/rem pairing (49:\
         449-458) would silently pay for two hardware divides instead of \
         one on this target"
    );
    assert!(
        count(combined, "\tmsub\t") >= 1,
        "the remainder must be recovered from the already-computed \
         quotient via multiply-subtract (r = a - q*b), not by a second \
         division"
    );
}
