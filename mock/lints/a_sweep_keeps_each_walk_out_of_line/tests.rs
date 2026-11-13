//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for `a-sweep-keeps-each-walk-out-of-line`.

use mockspace::testkit::LintFixture;

use super::*;

/// The walks as the suite writes them, each marked as the lint wants, beside
/// a trait declaration and a `run` outside any walk.
const MARKED: &str = "\
pub(crate) trait PerFormat {
    /// Do it at one format.
    fn run<F: Format>(&mut self);
}

impl<T: Step> PerFormat for Denotation<T> {
    #[inline(never)]
    fn run<F: Format>(&mut self) {
        self.formats += 1;
    }
}

impl PerSignature for EveryMember {
    type Out = Tally;

    /// The tally at one signature.
    #[inline(never)]
    fn run<S: DeclaredSignature>(&self) -> Tally {
        Tally::default()
    }
}

impl Runner {
    fn run(&self) -> u32 {
        1
    }
}
";

fn hits_in(path: &str, source: &str) -> Vec<LintError> {
    let fixture = LintFixture::new(source)
        .with_crate_name("arvo-format", "format")
        .with_sources(&[(path, source)]);
    ASweepKeepsEachWalkOutOfLine.check(&fixture.ctx())
}

fn hits(source: &str) -> Vec<LintError> {
    hits_in("src/addition/tests/the_step.rs", source)
}

/// The lines the findings name, in order.
fn lines(source: &str) -> Vec<usize> {
    hits(source).iter().map(|e| e.line).collect()
}

#[test]
fn it_answers_to_the_name_the_gate_and_the_config_use() {
    assert_eq!(ASweepKeepsEachWalkOutOfLine.name(), NAME);
    assert_eq!(NAME, "a-sweep-keeps-each-walk-out-of-line");
}

#[test]
fn it_is_not_declared_off_so_it_runs_at_all() {
    crate::canon_lint_testkit::assert_not_declared_off(&ASweepKeepsEachWalkOutOfLine);
}

#[test]
fn it_reaches_the_pack_the_engine_is_handed() {
    crate::crate_lint_testkit::assert_registered(NAME);
}

#[test]
fn its_findings_carry_its_own_declared_severity() {
    let errors = hits("impl PerFormat for W {\n    fn run<F: Format>(&mut self) {}\n}\n");
    crate::crate_lint_testkit::assert_findings_carry(
        &ASweepKeepsEachWalkOutOfLine,
        &errors,
        Severity::HARD_ERROR,
    );
}

#[test]
fn the_control_the_marked_walks_are_silent() {
    assert!(hits(MARKED).is_empty(), "{:?}", hits(MARKED));
}

#[test]
fn a_walk_with_no_mark_fires_at_the_line_its_item_starts() {
    let source = "\
impl<T: Step> PerFormat for Denotation<T> {
    /// Doc.
    fn run<F: Format>(&mut self) {
        self.formats += 1;
    }
}
";
    assert_eq!(lines(source), vec![3]);
    assert!(hits(source)[0].message.contains("`PerFormat`"));
}

#[test]
fn a_walk_marked_inline_rather_than_never_fires() {
    for mark in ["#[inline]", "#[inline(always)]", "#[must_use]"] {
        let source = format!(
            "impl PerSignature for W {{\n    type Out = ();\n    {mark}\n    fn run<S: DeclaredSignature>(&self) {{}}\n}}\n"
        );
        assert_eq!(lines(&source), vec![4], "{mark}");
    }
}

#[test]
fn every_unmarked_walk_in_a_file_is_named() {
    let source = "\
impl PerFormat for A {
    fn run<F: Format>(&mut self) {}
}

impl PerFormat for B {
    #[inline(never)]
    fn run<F: Format>(&mut self) {}
}

impl<T> PerSignature for C<T> {
    fn run<S: DeclaredSignature>(&self) {}
}
";
    assert_eq!(lines(source), vec![2, 11]);
}

#[test]
fn a_run_outside_a_walk_impl_is_not_read() {
    let source = "\
impl Runner {
    fn run(&self) {}
}

impl Clone for PerFormatLike {
    fn run(&self) {}
}
";
    assert!(hits(source).is_empty(), "{:?}", hits(source));
}

#[test]
fn shipped_source_and_another_crate_are_not_read() {
    let source = "impl PerFormat for W {\n    fn run<F: Format>(&mut self) {}\n}\n";
    assert!(hits_in("src/apply.rs", source).is_empty());
    let fixture = LintFixture::new(source)
        .with_crate_name("arvo-numeral", "numeral")
        .with_sources(&[("src/tests.rs", source)]);
    assert!(
        ASweepKeepsEachWalkOutOfLine
            .check(&fixture.ctx())
            .is_empty()
    );
}
