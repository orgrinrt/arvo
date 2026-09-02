//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ref is not optional, which is the whole methodology in one refusal.

use crate::corpus::TreeSpec;

#[test]
fn a_tree_spec_carries_a_repository_and_a_ref() {
    let spec = TreeSpec::parse("../notko@origin/dev").expect("a well-formed spec was refused");
    assert_eq!(spec.repo, "../notko");
    assert_eq!(spec.git_ref, "origin/dev");
    assert_eq!(spec.label(), "notko@origin/dev");
}

#[test]
fn a_spec_with_no_ref_is_refused_rather_than_defaulted() {
    // The obligation's own `gap` records a pass that "reported that consumer at
    // zero and called the figure controlled. It was reading a single-branch
    // clone." A default ref here would be that read, and it would be invisible.
    let err = TreeSpec::parse("../notko").expect_err("a spec with no ref was accepted");
    assert!(err.contains("names no ref"), "{err}");
}

#[test]
fn a_spec_with_an_empty_half_is_refused() {
    assert!(TreeSpec::parse("@origin/dev").is_err());
    assert!(TreeSpec::parse("../notko@").is_err());
}

#[test]
fn a_path_containing_an_at_sign_still_resolves_by_the_last_one() {
    let spec = TreeSpec::parse("/tmp/a@b/notko@origin/dev").expect("refused");
    assert_eq!(spec.repo, "/tmp/a@b/notko");
    assert_eq!(spec.git_ref, "origin/dev");
}

#[test]
fn a_build_directory_is_not_source_and_a_design_template_is_not_rust() {
    use crate::corpus::{is_design, is_rust};
    assert!(is_rust("mock/crates/a/src/lib.rs"));
    assert!(!is_rust("mock/target/debug/build/x/out/gen.rs"));
    assert!(!is_rust("target/x.rs"));
    assert!(!is_rust("README.md"));
    assert!(is_design("mock/DESIGN.md.tmpl", true));
    assert!(!is_design("mock/DESIGN.md", true));
}

#[test]
fn a_repository_with_no_templates_has_its_design_read_from_plain_markdown() {
    use crate::corpus::is_design;
    // Two trees in this stack carry no mockspace and so no template. A rule of
    // "templates only" answers those at zero, which is the demand side read
    // wrong in the direction that looks clean.
    assert!(is_design("DESIGN.md", false));
    assert!(is_design("docs/findings.md", false));
    assert!(
        !is_design("README.md", false),
        "the landing page is prose, not a design"
    );
    assert!(!is_design("CHANGELOG.md", false));
    assert!(!is_design("src/lib.rs", false));
}

#[test]
fn the_control_a_generated_markdown_is_skipped_where_its_template_exists() {
    use crate::corpus::is_design;
    // The same path, the two answers, decided by whether the tree has templates
    // at all. Without this the mockspace repositories count every design
    // position twice, once in the template and once in what it generated.
    assert!(!is_design("mock/DESIGN.md", true));
    assert!(is_design("mock/DESIGN.md", false));
}

#[test]
fn a_crates_own_src_ships_and_everything_beside_it_does_not() {
    use crate::corpus::is_shipped;
    assert!(is_shipped("mock/crates/arvo-format/src/lib.rs"));
    assert!(is_shipped("src/lib.rs"));
    for path in [
        "mock/crates/kolli/tests/examples.rs",
        "mock/crates/kolli/examples/ask.rs",
        "mock/benches/variants/a/src/lib.rs",
        "mock/research/sketches/x/src/lib.rs",
        "mock/research/202608072330_the-numeral-canon-panel/238_probes/x.rs",
        "mock/tools/the-positions/src/lib.rs",
    ] {
        // The last one is this tool itself, which is a check rather than a
        // crate anybody depends on, and it must not count either.
        if path.contains("/tools/") {
            continue;
        }
        assert!(!is_shipped(path), "`{path}` was counted as shipped surface");
    }
}

#[test]
fn the_control_the_shipped_rule_actually_discriminates() {
    use crate::corpus::is_shipped;
    // A rule answering the same thing for every path is not a rule. The pair
    // differs in one component and must differ in the answer.
    assert!(is_shipped("mock/crates/a/src/lib.rs"));
    assert!(!is_shipped("mock/crates/a/tests/lib.rs"));
    assert!(!is_shipped("mock/benches/variants/a/src/lib.rs"));
    assert!(!is_shipped("mock/research/sketches/a/src/lib.rs"));
}

#[test]
fn the_control_a_path_merely_containing_the_word_target_is_still_source() {
    use crate::corpus::is_rust;
    assert!(
        is_rust("mock/crates/retargeting/src/lib.rs"),
        "the exclusion matched a path component's substring rather than the component"
    );
}

#[test]
fn a_fixture_crate_under_tests_is_not_shipped_however_many_src_directories_it_has() {
    use crate::corpus::is_shipped;
    // Found on the real corpus rather than by reading the rule: a `trybuild`
    // fixture is a whole crate, so it has a `src/lib.rs`, and a rule that only
    // asked whether `src` appears counted `notko-macros`'s fixture consumer as
    // part of notko's public surface.
    assert!(!is_shipped(
        "notko-macros/tests/fixtures/consumer/src/lib.rs"
    ));
    assert!(!is_shipped(
        "mock/crates/kolli-layout/tests/ui/a_row_of_the_wrong_arity.rs"
    ));
}

#[test]
fn a_tool_and_a_lint_are_checks_rather_than_published_surface() {
    use crate::corpus::is_shipped;
    // This tool's own source sits at exactly this shape and would otherwise
    // count itself into the answer it reports.
    assert!(!is_shipped("mock/tools/the-positions/src/lib.rs"));
    assert!(!is_shipped("mock/lints/a_row_carries_keywords.rs"));
}

#[test]
fn the_control_the_exclusions_match_a_path_component_and_not_a_substring() {
    use crate::corpus::is_shipped;
    // Every excluded name, planted inside a longer directory name that must
    // still ship. A substring match would take all of these.
    for path in [
        "mock/crates/testing-kit/src/lib.rs",
        "mock/crates/toolbox/src/lib.rs",
        "mock/crates/lintable/src/lib.rs",
        "mock/crates/researcher/src/lib.rs",
        "mock/crates/benchmarking/src/lib.rs",
        "mock/crates/uikit/src/lib.rs",
        "mock/crates/exampled/src/lib.rs",
    ] {
        assert!(is_shipped(path), "`{path}` was excluded on a substring");
    }
}
