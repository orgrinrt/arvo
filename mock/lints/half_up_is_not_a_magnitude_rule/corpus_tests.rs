//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the lint reads: the prose of a Rust file, the files of a tree and the
//! fields of a registry, each with the part it leaves alone beside it. Whether
//! it reaches the gate at all is asked in the lint's own file.

use super::comments::passages;
use super::{check_registry, check_tree};
use crate::canon_lint_testkit::{plant, planted_tree, view};

/// The passages of `source` as text and first line.
fn read(source: &str) -> Vec<(String, usize)> {
    passages(source)
        .into_iter()
        .map(|p| (p.text, p.line))
        .collect()
}

#[test]
fn consecutive_line_comments_of_one_kind_are_one_passage() {
    let source = "/// one\n/// two\n//! inner\n// plain\n// more\nfn f() {} // trailing\n";
    assert_eq!(read(source), [
        (" one\n two".to_string(), 1),
        (" inner".to_string(), 3),
        (" plain\n more".to_string(), 4),
        (" trailing".to_string(), 6),
    ]);
}

#[test]
fn a_gap_or_code_between_comments_splits_them() {
    let source = "// a\n\n// b\nlet x = 1;\n// c\n";
    let lines: Vec<usize> = read(source).into_iter().map(|(_, l)| l).collect();
    assert_eq!(lines, [1, 3, 5]);
}

#[test]
fn four_slashes_is_a_plain_comment_and_not_a_doc() {
    let p = passages("//// banner\nstruct HalfUp;\n");
    assert_eq!(p.len(), 1);
    assert_eq!(p[0].subject, None, "a plain comment binds to nothing");
}

#[test]
fn nothing_inside_a_literal_is_prose() {
    // Each line ends in a comment that is prose, and each literal before it
    // holds something a reader that got the literal wrong would take for one,
    // or would read past the comment for.
    let source = concat!(
        "let a = \"// not a comment `half_up` away from zero\";\n",
        "let b = r#\"/* nor this \"quoted\" */\"#;\n",
        "let c = '\"'; // after a quote char\n",
        "let d = b\"// bytes\";\n",
        "fn e<'a>(x: &'a str) {} // after a lifetime\n",
        "let f = '\\''; // after an escaped quote\n",
        "let g = \"a \\\" // still the string\"; // after an escaped string quote\n",
        "let h = r\"C:\\\"; // after a raw backslash\n",
        "let i = r#\"say \"// no\"\"#; // after hashes\n",
    );
    let texts: Vec<String> = read(source).into_iter().map(|(t, _)| t).collect();
    assert_eq!(texts, [
        " after a quote char",
        " after a lifetime",
        " after an escaped quote",
        " after an escaped string quote",
        " after a raw backslash",
        " after hashes",
    ]);
}

#[test]
fn a_block_comment_nests_and_keeps_its_first_line() {
    let source = "fn f() {}\n/* outer /* inner */ still\nouter */ fn g() {}\n";
    assert_eq!(read(source), [(
        " outer /* inner */ still\nouter ".to_string(),
        2
    )]);
}

#[test]
fn a_doc_block_takes_the_mode_its_item_names_past_attributes() {
    let source = "/// Nearest.\n#[derive(Clone)]\n// note\npub struct HalfUp;\n";
    assert_eq!(passages(source)[0].subject, Some("HalfUp"));
    let variant = "enum Mode {\n    /// Nearest.\n    HalfUp,\n}\n";
    assert_eq!(passages(variant)[0].subject, Some("HalfUp"));
}

#[test]
fn control_a_doc_block_over_another_item_takes_nothing() {
    for source in [
        "/// Nearest.\npub struct HalfEven;\n",
        "/// Nearest.\nfn half_up_ties_away_from_zero() {}\n",
        "/// Nearest.\nfn f() {} // HalfUp\n",
        "//! Nearest.\npub struct HalfUp;\n",
    ] {
        assert_eq!(passages(source)[0].subject, None, "{source}");
    }
}

#[test]
fn a_line_is_counted_from_the_passage_to_the_reading() {
    let p = &passages("\n\n// first\n// `half_up` goes away from zero.\n")[0];
    let at = super::hits(&p.text, None)[0].at;
    assert_eq!(p.line_of(at), 4);
}

/// The findings over a planted tree, as path and line.
fn tree(what: &str, files: &[(&str, &str)]) -> Vec<(String, usize)> {
    let dir = planted_tree(what);
    for (at, text) in files {
        plant(&dir, at, text);
    }
    check_tree(&dir)
        .into_iter()
        .map(|e| (e.path.unwrap_or_default(), e.line))
        .collect()
}

const BAD: &str = "`half_up` takes a tie away from zero.";

#[test]
fn a_doc_comment_a_comment_and_a_template_each_fire_where_they_are() {
    let f = tree("half-up-fires", &[
        (
            "crates/a/src/lib.rs",
            &format!("fn f() {{}}\n/// {BAD}\nfn g() {{}}\n"),
        ),
        ("tools/t/src/lib.rs", &format!("// {BAD}\n")),
        ("benches/b/src/lib.rs", &format!("//! {BAD}\n")),
        ("crates/a/DESIGN.md.tmpl", &format!("# A\n\nText.\n{BAD}\n")),
        ("DESIGN.md.tmpl", &format!("{BAD}\n")),
    ]);
    assert_eq!(f, [
        ("DESIGN.md.tmpl".to_string(), 1),
        ("benches/b/src/lib.rs".to_string(), 1),
        ("crates/a/DESIGN.md.tmpl".to_string(), 4),
        ("crates/a/src/lib.rs".to_string(), 2),
        ("tools/t/src/lib.rs".to_string(), 1),
    ]);
}

#[test]
fn control_the_record_the_build_a_literal_and_plain_markdown_are_not_read() {
    let f = tree("half-up-silent", &[
        ("research/a/notes.md.tmpl", BAD),
        ("research/sketches/s/src/main.rs", &format!("// {BAD}\n")),
        ("design_rounds/202601010000_topic.md.tmpl", BAD),
        ("target/x/src/lib.rs", &format!("// {BAD}\n")),
        ("lints/l.rs", &format!("/* {BAD} */\n")),
        (".hidden/a.rs", &format!("// {BAD}\n")),
        ("crates/a/README.md", BAD),
        (
            "crates/a/src/lib.rs",
            &format!("const S: &str = \"{BAD}\";\n"),
        ),
    ]);
    assert!(f.is_empty(), "{f:?}");
}

#[test]
fn a_tree_that_is_not_there_is_silent_rather_than_a_panic() {
    assert!(check_tree(&planted_tree("half-up-absent").join("nothing")).is_empty());
}

#[test]
fn a_prose_field_fires_and_names_the_row_and_the_field() {
    let v = view(&[("law::a", &[("note", BAD), ("claim", "Fine.")])], &[]);
    let f = check_registry(&v);
    assert_eq!(
        f.len(),
        1,
        "{:?}",
        f.iter().map(|e| &e.message).collect::<Vec<_>>()
    );
    assert_eq!(f[0].path.as_deref(), Some("`law::a`, field `note`"));
}

#[test]
fn control_what_is_not_prose_or_is_the_record_is_not_read() {
    let v = view(
        &[
            ("law::a", &[
                ("keywords", BAD),
                ("quote", BAD),
                ("provenance", BAD),
                ("options", BAD),
            ]),
            ("retirement::b", &[("claim", BAD)]),
            // The answer names both readings to say which was taken, so the
            // field is the record. The rest of the row is not.
            ("question::c", &[("answered", BAD)]),
            ("ruling::d", &[("note", BAD), ("rung", "ratified")]),
        ],
        &[],
    );
    assert!(check_registry(&v).is_empty());
}

#[test]
fn an_open_question_and_an_unratified_ruling_are_read() {
    let v = view(
        &[
            ("question::c", &[("asks", BAD)]),
            // An answered question keeps its prose inside the gate. Skipping
            // the whole row put `asks`, `note` and `because` outside it.
            ("question::f", &[("asks", BAD), ("answered", "The first.")]),
            ("ruling::d", &[("note", BAD), ("rung", "stated")]),
            ("ruling::e", &[("note", BAD)]),
        ],
        &[],
    );
    let at: Vec<String> = check_registry(&v)
        .into_iter()
        .filter_map(|e| e.path)
        .collect();
    assert_eq!(at, [
        "`question::c`, field `asks`",
        "`question::f`, field `asks`",
        "`ruling::d`, field `note`",
        "`ruling::e`, field `note`",
    ]);
}
