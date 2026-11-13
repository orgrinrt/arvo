//! The checker driven against planted files: the quotation that must be found,
//! the three ways a quotation is not where it says, and the controls that keep
//! each of those from reading as a pass.

use super::{Verdict, fold, verify};

/// A reader's source with a numbered body, so a cited range means something.
const SOURCE: &str = "line one is about nothing\n\
    The shipped identity predicate asks the cancellation question at each admitted magnitude.\n\
    A second sentence holds the quantum at magnitude zero as a rational.\n\
    line four is padding\n\
    line five is padding too\n\
    Far from the citation sits the sentence that a reader may quote from the wrong place.\n";

fn read(path: &str) -> Option<String> {
    (path == "src.md").then(|| SOURCE.to_string())
}

fn one(block: &str, margin: usize) -> super::BlockResult {
    let mut out = verify(block, &read, margin);
    assert_eq!(out.len(), 1, "{out:?}");
    out.remove(0)
}

fn block(evidence: &str, passage: &str) -> String {
    format!("## proposal::a_claim\nclass: x\nevidence: {evidence}\npassage: {passage}\n")
}

// --- the quotation that must be found ---

#[test]
fn a_quote_in_the_cited_lines_is_ok() {
    let r = one(
        &block(
            "src.md:2-3",
            "\"asks the cancellation question at each admitted magnitude\"",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Ok, "{r:?}");
    assert_eq!((r.in_lines, r.elsewhere, r.nowhere), (1, 0, 0));
}

#[test]
fn a_typographic_quote_markdown_marks_and_case_are_folded() {
    let r = one(
        &block(
            "src.md:2",
            "\u{201C}Asks the **cancellation** question at each `admitted` magnitude\u{201D}",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Ok, "{r:?}");
}

#[test]
fn a_quote_with_an_elided_middle_is_judged_by_its_segments() {
    let r = one(
        &block(
            "src.md:2-3",
            "\"asks the cancellation question ... holds the quantum at magnitude zero\"",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Ok, "{r:?}");
    assert_eq!(r.in_lines, 2);
}

// --- the three ways it is not where it says, each of which must be seen ---

#[test]
fn a_quote_found_only_outside_the_cited_lines_is_far() {
    let r = one(
        &block(
            "src.md:2-3",
            "\"the sentence that a reader may quote from the wrong place\"",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Far, "{r:?}");
    assert_eq!(r.elsewhere, 1);
}

#[test]
fn a_quote_in_no_cited_file_is_a_miss() {
    let r = one(
        &block(
            "src.md:2-3",
            "\"a sentence that the source never contained at all\"",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Miss, "{r:?}");
    assert_eq!(r.nowhere, 1);
}

#[test]
fn one_missing_run_beside_a_found_one_still_makes_the_block_a_miss() {
    let r = one(
        &block(
            "src.md:2-3",
            "\"asks the cancellation question at each admitted magnitude\" and \"an invented \
             sentence that is nowhere in the file\"",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Miss, "{r:?}");
    assert_eq!((r.in_lines, r.nowhere), (1, 1));
}

/// The margin widens what counts as in the lines, and 0 does not.
#[test]
fn the_margin_moves_a_near_quote_from_far_to_ok() {
    let b = block(
        "src.md:2",
        "\"holds the quantum at magnitude zero as a rational\"",
    );
    assert_eq!(one(&b, 0).verdict, Verdict::Far);
    assert_eq!(one(&b, 1).verdict, Verdict::Ok);
}

// --- what the tool will not judge, and says so ---

#[test]
fn a_run_too_short_to_judge_is_named_and_never_counted_as_a_hit() {
    let r = one(&block("src.md:2", "\"the identity\""), 0);
    assert_eq!(r.verdict, Verdict::Unjudged, "{r:?}");
    assert_eq!(r.short, vec!["the identity".to_string()]);
    assert_eq!(
        r.in_lines, 0,
        "a short run that is present is still no evidence"
    );
}

#[test]
fn a_short_run_beside_a_judged_one_is_listed_and_the_block_is_still_judged() {
    let r = one(
        &block(
            "src.md:2",
            "\"magnitude\" and \"asks the cancellation question at each admitted magnitude\"",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Ok, "{r:?}");
    assert_eq!(r.short, vec!["magnitude".to_string()]);
}

#[test]
fn a_passage_with_no_quotes_is_one_run() {
    let r = one(
        &block(
            "src.md:2",
            "asks the cancellation question at each admitted magnitude",
        ),
        0,
    );
    assert_eq!(r.verdict, Verdict::Ok, "{r:?}");
}

// --- the blocks that cannot be checked at all ---

#[test]
fn a_block_with_no_evidence_or_no_passage_says_which() {
    let no_ev = "## proposal::a\nclass: x\npassage: \"some quoted run here that is long\"\n";
    let no_pass = "## proposal::b\nclass: x\nevidence: src.md:2\n";
    assert_eq!(verify(no_ev, &read, 0)[0].verdict, Verdict::NoEvidence);
    assert_eq!(verify(no_pass, &read, 0)[0].verdict, Verdict::NoPassage);
}

#[test]
fn an_evidence_that_is_not_a_path_and_lines_and_a_file_that_is_absent_say_so() {
    assert_eq!(
        one(&block("src.md", "\"some quoted run here that is long\""), 0).verdict,
        Verdict::NoParse
    );
    assert_eq!(
        one(
            &block("gone.md:1-2", "\"some quoted run here that is long\""),
            0
        )
        .verdict,
        Verdict::NoFile
    );
}

#[test]
fn several_evidence_lines_are_each_searched() {
    let b = "## proposal::a\nevidence: src.md:2\nevidence: src.md:6\n\
             passage: \"the sentence that a reader may quote from the wrong place\"\n";
    assert_eq!(verify(b, &read, 0)[0].verdict, Verdict::Ok);
}

#[test]
fn blocks_are_split_at_their_headings_and_each_judged_alone() {
    let two = format!(
        "{}\n{}",
        block(
            "src.md:2",
            "\"asks the cancellation question at each admitted magnitude\""
        ),
        block(
            "src.md:2",
            "\"an invented sentence that is nowhere in the file\""
        )
    );
    let out = verify(&two, &read, 0);
    assert_eq!(out.len(), 2);
    assert_eq!(
        (out[0].verdict, out[1].verdict),
        (Verdict::Ok, Verdict::Miss)
    );
}

#[test]
fn text_before_the_first_heading_is_not_a_block() {
    assert!(verify("a preamble\nevidence: src.md:2\n", &read, 0).is_empty());
}

#[test]
fn a_passage_may_continue_over_lines_until_the_next_key() {
    let b = "## proposal::a\nevidence: src.md:2\npassage: \"asks the cancellation question\n\
             at each admitted magnitude\"\nclass: late\n";
    assert_eq!(verify(b, &read, 0)[0].verdict, Verdict::Ok);
}

#[test]
fn a_toml_line_continuation_in_the_source_is_folded() {
    assert_eq!(fold("a long \\\n   sentence"), "a long sentence");
    assert_eq!(fold("> quoted\n> text"), "quoted text");
}

// --- an evidence line that cannot be used does not void the others ---

#[test]
fn an_unparseable_evidence_line_is_skipped_and_the_rest_are_checked() {
    let b = "## proposal::a\nevidence: not-a-range\nevidence: src.md:2\n\
             passage: \"asks the cancellation question at each admitted magnitude\"\n";
    let r = verify(b, &read, 0).remove(0);
    assert_eq!(r.verdict, Verdict::Ok, "{r:?}");
    assert_eq!(r.skipped, 1);
}

#[test]
fn an_unreadable_file_is_skipped_and_the_rest_are_checked() {
    let b = "## proposal::a\nevidence: gone.md:1-2\nevidence: src.md:2\n\
             passage: \"asks the cancellation question at each admitted magnitude\"\n";
    let r = verify(b, &read, 0).remove(0);
    assert_eq!(r.verdict, Verdict::Ok, "{r:?}");
    assert_eq!(r.skipped, 1);
}

#[test]
fn control_a_block_whose_every_evidence_line_is_unusable_says_which_way() {
    let bad = "## proposal::a\nevidence: not-a-range\nevidence: also-not\n\
               passage: \"some quoted run here that is long\"\n";
    let r = verify(bad, &read, 0).remove(0);
    assert_eq!((r.verdict, r.skipped), (Verdict::NoParse, 2));
    let gone = "## proposal::a\nevidence: not-a-range\nevidence: gone.md:1-2\n\
                passage: \"some quoted run here that is long\"\n";
    let r = verify(gone, &read, 0).remove(0);
    assert_eq!((r.verdict, r.skipped), (Verdict::NoFile, 2));
}

#[test]
fn a_block_with_evidence_and_no_passage_is_not_called_evidence_less() {
    let b = "## proposal::a\nclass: x\nevidence: src.md:2\n";
    assert_eq!(verify(b, &read, 0)[0].verdict, Verdict::NoPassage);
}

/// A margin that does not parse must not become zero: a reader who typed
/// `margin=12` would be checked in the cited lines alone and told so by nothing.
#[test]
fn control_a_margin_that_is_not_a_number_is_inconclusive() {
    use std::collections::BTreeMap;

    use mockspace::RegistryView;
    use mockspace::tool::{Outcome, Tool, ToolContext};

    let dir = std::env::temp_dir().join(format!("cited-passages-margin-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let out = dir.join("reader.md");
    std::fs::write(
        &out,
        block("src.md:2", "\"some quoted run here that is long\""),
    )
    .unwrap();
    let out_arg = out.to_string_lossy().to_string();
    let root_arg = dir.to_string_lossy().to_string();
    let view = RegistryView::new(BTreeMap::new(), BTreeMap::new());
    let crates = Default::default();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let args = [out_arg.as_str(), root_arg.as_str(), "twelve"];
    let ctx = ToolContext {
        mock_dir:   std::path::Path::new("."),
        repo_root:  std::path::Path::new("."),
        all_crates: &crates,
        src_dirs:   &dirs,
        args:       &args,
        stdin:      None,
        registry:   &view,
    };
    let rep = super::CitedPassagesCheck.run(&ctx);
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        matches!(rep.outcome, Outcome::Inconclusive { .. }),
        "{:?}",
        rep.outcome
    );
}
