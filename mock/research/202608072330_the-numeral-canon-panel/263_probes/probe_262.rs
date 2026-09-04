//! Probe for seat 262: what the two region checks say about a structural claim.
//!
//! An ad-hoc spike, not a lint and not a bench. It plants five proposal rows
//! and runs the two shipped lints that read a `predicate` against them:
//! `a-region-agrees-with-the-sentence-kind` and
//! `every-predicate-names-a-declared-axis`. The question it answers is
//! qualitative: which filings of a reasoned claim about the canon's own
//! structure does the shipped checker refuse, and which does it pass.
//!
//! The declared axes planted are two of the real twenty-five, `threads` and
//! `fraction_width`; the declared-axis lint reads the vocabulary from the
//! registry view it is handed, so two are enough to make the refusal and the
//! pass both reachable.
//!
//! The control that must fail: cell A and cell B are the two refusals, and if
//! either lint stopped firing the assertion on that cell fails. Cell E is the
//! positive control in the other direction, an ordinary claim about arvo that
//! both lints must pass.

use mockspace::RepoLint;

use crate::canon_lint_testkit::{ctx, view};

fn kinds_of(lint: &dyn RepoLint, rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
    let v = view(rows, &[]);
    lint.check_repo(&ctx(&v))
        .into_iter()
        .map(|e| e.finding_kind.unwrap_or("<unkinded>").to_string())
        .collect()
}

#[test]
fn the_four_filings_of_a_structural_claim_against_the_two_checks() {
    let region = crate::a_region_agrees_with_the_sentence_kind::repo_lint();
    let axis = crate::every_predicate_names_a_declared_axis::repo_lint();
    let dims: [(&str, &[(&str, &str)]); 2] = [
        ("dimension::threads", &[("what", "how many threads")]),
        ("dimension::fraction_width", &[(
            "what",
            "bits below the point",
        )]),
    ];

    // The one sentence under test, in every filing the schema offers it.
    let says =
        "`the_format` cannot be stated without a decision belonging to `the_container_premise`.";
    let cells: [(&str, &str, &[(&str, &str)]); 5] = [
        ("A", "argument, no predicate", &[
            ("sentence_kind", "argument"),
            ("says", says),
        ]),
        ("B", "argument, predicate over a topic", &[
            ("sentence_kind", "argument"),
            ("says", says),
            ("predicate", "topic: topic = the_format"),
        ]),
        ("C", "argument, predicate `threads = 1` (the fig leaf)", &[
            ("sentence_kind", "argument"),
            ("says", says),
            ("predicate", "threads: threads = 1"),
        ]),
        ("D", "normative, no predicate (the refile)", &[
            ("sentence_kind", "normative"),
            ("says", says),
        ]),
        ("E", "control: an arvo claim, argument, `F = 0`", &[
            ("sentence_kind", "argument"),
            ("says", "addition is exact"),
            ("predicate", "fraction_width: F = 0"),
        ]),
    ];

    println!();
    println!(
        "| cell | filing | a-region-agrees-with-the-sentence-kind | every-predicate-names-a-declared-axis |"
    );
    println!("|---|---|---|---|");
    let mut got: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();
    for (cell, filing, fields) in cells {
        let mut rows: Vec<(&str, &[(&str, &str)])> = dims.to_vec();
        rows.push(("proposal::structural_claim", fields));
        let r = kinds_of(region.as_ref(), &rows);
        let a = kinds_of(axis.as_ref(), &rows);
        let show = |v: &Vec<String>| if v.is_empty() { "silent".to_string() } else { v.join(", ") };
        println!("| {cell} | {filing} | {} | {} |", show(&r), show(&a));
        got.push((cell.to_string(), r, a));
    }
    println!();

    // A and B are the refusals; if either goes quiet the finding is gone.
    assert_eq!(
        got[0].1,
        ["an-established-claim-carries-no-region"],
        "cell A: {:?}",
        got[0]
    );
    assert!(got[0].2.is_empty(), "cell A: {:?}", got[0]);
    assert!(got[1].1.is_empty(), "cell B: {:?}", got[1]);
    assert_eq!(got[1].2, ["undeclared-axis"], "cell B: {:?}", got[1]);
    // C and D are the two passes, and C is the one the seat is about.
    assert!(
        got[2].1.is_empty() && got[2].2.is_empty(),
        "cell C: {:?}",
        got[2]
    );
    assert!(
        got[3].1.is_empty() && got[3].2.is_empty(),
        "cell D: {:?}",
        got[3]
    );
    // E is the positive control: an ordinary claim about arvo passes both.
    assert!(
        got[4].1.is_empty() && got[4].2.is_empty(),
        "cell E: {:?}",
        got[4]
    );
}
