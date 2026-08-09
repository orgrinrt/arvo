//! The sixth codegen regression test the owed-test-debt list names
//! (`49_consolidation_four.md:814-821`, "the multi-limb carry chain,
//! the fold-versus-direct-multiply fold... the saturating-reduction
//! non-vectorisation, the vectorisable-loop-idiom sensitivity, and (new
//! this stretch) the `div_floor`/`rem` fusion", now joined by the
//! licence-leak pair the persona checkpoint named at `53b`:57-59,
//! "The `51` codegen regression pair (interior-safe `fold` vectorises,
//! `fold_compensated` on identical data stays scalar and unfused) joins
//! the owed test list so a toolchain change cannot leak the licence
//! across the combinator boundary silently."
//!
//! Fixture: `codegen/licence_leak.rs`. `fold_interior_safe` reproduces
//! `51_probes/probe_4_licence_reassoc_vectorizes.rs`'s `sum_algebraic`
//! (the form a build layer is licensed to emit once interior safety is
//! proven, `51` section 2.4). `fold_compensated_step` reproduces
//! `51_probes/probe_6_licence_destroys_compensation.rs`'s
//! `kahan_step_strict` (the form `fold_compensated` must always compile
//! from; the licence must never reach it, `51` sections 2.3-2.4).
//!
//! DESTINATION (on move): `mock/crates/arvo/tests/codegen_regression.rs`
//! (the same file `52_probes/codegen_regression_harness.rs` is destined
//! for; the two harnesses merge into one file with six `#[test]`
//! functions rather than shipping as two files, since they are the same
//! measurement suite), fixture at
//! `mock/crates/arvo/tests/codegen/licence_leak.rs`. No shipped arvo
//! source exists yet (`grep -rln "Adjustment\|Bias\|Numeral"
//! mock/crates/ --include="*.rs"` returns nothing, reproduced fresh for
//! this dispatch), so the fixture is free-standing exactly as file 52's
//! five already are: `.algebraic_add()`/plain `+`/`-` stand in for
//! whatever a build layer eventually lowers `fold`/`fold_compensated` to.
//! On the move, the stand-in bodies are replaced by the real combinator
//! calls; the assertions are unchanged, because they pin instruction
//! shapes and the presence/absence of NEON lane instructions, not which
//! source line produced them.
//!
//! MEASUREMENT, NOT CONTRACT (`52_ringer_the_tests_that_were_owed.md`
//! section 1 states the distinction this harness follows exactly).
//! `fold_interior_safe` vectorising is "the optimiser currently grants
//! `reassoc`'s vectorisation on this target when `.algebraic_add()` is
//! written"; a red run here after a toolchain bump is news about the
//! optimiser, not a design defect, and gets read and re-recorded rather
//! than patched to whatever the new compiler emits.
//! `fold_compensated_step` staying scalar and unfused is closer to a
//! contract in spirit (the design says this combinator's meaning
//! depends on the plain, unreassociated sequence, `49:184-186`) but the
//! test itself only pins what LLVM does with the PLAIN source it is
//! given; it does not and cannot prove `fold_compensated` will never be
//! handed algebraic operations by a future build layer bug. That
//! coverage is the receipt file 51 section 2.4 proposes and nobody has
//! built (the fourth receipt clause, "the combinator is `fold`, never
//! `fold_compensated`"); this test is the fallback that catches the
//! narrower case where the plain source itself starts getting
//! reassociated by the optimiser without being asked, which would be a
//! much larger and more visible event than a single build-layer bug but
//! costs nothing extra to also pin here.
//!
//! TARGET SCOPE. `aarch64-apple-darwin`, `rustc 1.98.0-nightly
//! (57d06900f 2026-05-27)`. Flag-checked for this dispatch's own audit
//! item (file 57, the codegen-flag sweep): identical emitted asm under
//! `-C opt-level=3` at both `-C codegen-units=1` and the rustc default
//! (`codegen-units=16`), unlike test 4 in file 52's own harness. This
//! harness still pins `-C opt-level=3 -C codegen-units=1 -C panic=abort
//! --emit=asm` as the standing flag set, matching file 52's five, so the
//! six tests share one build invocation once merged; the flag-
//! insensitivity found here is recorded as a finding, not used to argue
//! the flag no longer needs stating.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const FIXTURE_DIR: &str = "codegen"; // sibling to this file once it ships

fn rustc_path() -> String {
    env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string())
}

fn require_target() {
    if !cfg!(target_arch = "aarch64") || !cfg!(target_os = "macos") {
        panic!(
            "codegen_regression_licence_leak: every assertion in this file \
             is an aarch64-apple-darwin fact; skip on other targets rather \
             than reinterpret the expected instruction shapes"
        );
    }
}

fn emit_asm(fixture: &str, extra_flags: &[&str]) -> String {
    require_target();
    let out_dir = env::temp_dir().join("arvo_codegen_regression_licence_leak");
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
    let end = rest
        .find("\tret\n")
        .map(|i| i + "\tret\n".len())
        .unwrap_or(rest.len());
    &rest[..end]
}

fn count(text: &str, needle: &str) -> usize {
    text.matches(needle).count()
}

/// Test 6: interior-safe `fold` vectorises via the algebraic-reassociation
/// licence; `fold_compensated`, computing from identical plain arithmetic,
/// stays scalar and unfused. The negative half is the one that actually
/// guards the boundary (51 section 2.3's finding that the algebraic form,
/// if it ever reached this combinator, would fold the whole expression to
/// a constant zero, `fsub s0, s1, s1`); this test's own negative assertion
/// checks the plain form never drifts toward that shape on its own.
#[test]
fn interior_safe_fold_vectorises_compensated_fold_stays_scalar_unfused() {
    let asm = emit_asm("licence_leak.rs", &[]);

    let interior_safe = body_of(&asm, "fold_interior_safe");
    assert!(
        count(interior_safe, ".4s") + count(interior_safe, ".2s") + count(interior_safe, ".2d") > 0,
        "fold_interior_safe (.algebraic_add() throughout, the form a build \
         layer is licensed to emit once interior safety is proven) must \
         vectorise; if it stops, the reassociation licence file 51 section \
         2.2 measured has regressed on this target and the whole \
         'laws as backend licences' argument for this combinator needs \
         re-measuring, not re-asserting"
    );

    let compensated = body_of(&asm, "fold_compensated_step");
    assert_eq!(
        count(compensated, ".4s") + count(compensated, ".2s") + count(compensated, ".2d"),
        0,
        "fold_compensated_step (plain +/-, never algebraic) must stay fully \
         scalar; any NEON lane instruction here means LLVM started \
         reassociating float arithmetic without an explicit fast-math \
         flag, an event much larger than this one fixture and worth \
         escalating immediately"
    );
    assert_eq!(
        count(compensated, "\tfadd\t"),
        1,
        "fold_compensated_step must compute the sum step exactly once"
    );
    assert_eq!(
        count(compensated, "\tfsub\t"),
        2,
        "fold_compensated_step must compute both subtraction steps \
         separately; a count of 1 (or 0) means the compensation \
         expression got folded or eliminated, exactly the correctness \
         failure 51 section 2.3 found under the algebraic form and this \
         test exists to notice if it ever happens to the PLAIN form"
    );
    assert_eq!(
        count(compensated, "\tfmadd\t") + count(compensated, "\tfmsub\t"),
        0,
        "fold_compensated_step must show no fused multiply-add/subtract; \
         there is no multiply in this expression, so any fmadd/fmsub here \
         would mean the compiler restructured the computation in a way \
         this test's whole premise (the plain sequence is the meaning) \
         no longer holds"
    );
}
