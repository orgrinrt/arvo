//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Reading a suppression, which is the one number here that rests on no
//! judgement of the tool's.

use crate::allows::allows_in;

const REAL: &str = r#"
pub struct NodeId(pub u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) tracked: #207
pub const fn new(b: u32) -> Self { // lint:allow(no-bare-numeric) reason: wire-format bit width fixed for on-disk representation; exact-width refinement tracked: #72
    Self(b)
}
unsafe fn allocate(&self, len: USize) -> *mut u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72
pub fn plain(x: u32) {}
"#;

#[test]
fn every_watched_lint_is_read_from_a_site_that_names_several() {
    let rows = allows_in("t", "src/lib.rs", REAL);
    let names: Vec<&str> = rows.iter().map(|a| a.lint).collect();
    assert_eq!(
        names.iter().filter(|n| **n == "no-bare-numeric").count(),
        3,
        "{names:?}"
    );
    assert_eq!(
        names.iter().filter(|n| **n == "arvo-types-only").count(),
        2,
        "{names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|n| **n == "no-public-raw-field")
            .count(),
        1,
        "{names:?}"
    );
}

#[test]
fn a_tracked_identifier_is_read_off_the_site() {
    let rows = allows_in("t", "src/lib.rs", REAL);
    let tracked: Vec<&str> = rows.iter().map(|a| a.tracked.as_str()).collect();
    assert!(tracked.contains(&"#207"), "{tracked:?}");
    assert!(tracked.contains(&"#72"), "{tracked:?}");
}

#[test]
fn a_reason_stops_at_the_next_marker_rather_than_running_to_end_of_line() {
    // The real sites write `reason: ... tracked: #72`, so a read to end of line
    // takes the task identifier into the reason and makes every reason unique,
    // which then makes a tally over reasons useless.
    let rows = allows_in("t", "src/lib.rs", REAL);
    let reason = rows
        .iter()
        .find(|a| a.reason.contains("wire-format"))
        .expect("the reason was not read at all");
    assert!(!reason.reason.contains("tracked"), "`{}`", reason.reason);
    assert!(reason.reason.ends_with("refinement"), "`{}`", reason.reason);
}

#[test]
fn the_control_a_line_with_no_suppression_yields_nothing() {
    assert!(allows_in("t", "src/lib.rs", "pub fn plain(x: u32) {}").is_empty());
}

#[test]
fn the_control_a_suppression_of_an_unwatched_lint_is_not_counted() {
    // The four names are the ones enforcing the constraint the obligation comes
    // from. A tally over every `lint:allow` in the stack would be a tally of
    // something else entirely and would be larger, which reads as worse.
    let text = "pub fn f() {} // lint:allow(file-size) lint:allow(no-alloc) tracked: #1";
    assert!(allows_in("t", "src/lib.rs", text).is_empty());
}

#[test]
fn the_control_a_site_with_no_task_is_distinguishable_from_one_with_it() {
    let text = "pub fn f(x: u32) {} // lint:allow(no-bare-numeric)";
    let rows = allows_in("t", "src/lib.rs", text);
    assert_eq!(rows.len(), 1);
    assert!(rows[0].tracked.is_empty(), "`{}`", rows[0].tracked);
    assert!(rows[0].reason.is_empty(), "`{}`", rows[0].reason);
}

#[test]
fn the_line_number_is_the_files_own_and_one_based() {
    let rows = allows_in(
        "t",
        "src/lib.rs",
        "a\nb\npub fn f() {} // lint:allow(no-bare-numeric)\n",
    );
    assert_eq!(rows[0].line, 3);
}

#[test]
fn a_marker_on_its_own_comment_line_is_recorded_as_inert() {
    // The pack that reads these is strictly per line: `line_lint_allowed`
    // looks at one line and no neighbour, and `no_bare_numeric` returns early
    // on any line whose trimmed text begins `//`. So a marker written above the
    // item it means to cover suppresses nothing, and the item below is checked
    // as though the marker were not there.
    let text = "// lint:allow(no-bare-numeric) reason: const arithmetic; tracked: #121\nconst N: usize = 4;";
    let rows = allows_in("t", "src/lib.rs", text);
    assert_eq!(rows.len(), 1);
    assert!(
        rows[0].comment_only,
        "a comment-only marker was not recorded as one"
    );
}

#[test]
fn the_control_a_trailing_marker_on_a_real_line_is_not_inert() {
    // The mutation: the same marker, moved onto the line it covers.
    let text = "const N: usize = 4; // lint:allow(no-bare-numeric) tracked: #121";
    let rows = allows_in("t", "src/lib.rs", text);
    assert_eq!(rows.len(), 1);
    assert!(!rows[0].comment_only);
}

#[test]
fn a_doc_comment_carrying_a_marker_is_a_comment_line_too() {
    let text = "/// lint:allow(no-bare-numeric)\npub fn f(x: u32) {}";
    let rows = allows_in("t", "src/lib.rs", text);
    assert_eq!(rows.len(), 1);
    assert!(rows[0].comment_only);
}

#[test]
fn the_line_text_is_kept_so_a_report_can_say_what_the_marker_sits_beside() {
    let text = "  const N: usize = 4; // lint:allow(no-bare-numeric)";
    let rows = allows_in("t", "src/lib.rs", text);
    assert_eq!(
        rows[0].text,
        "const N: usize = 4; // lint:allow(no-bare-numeric)"
    );
}

#[test]
fn the_packs_own_rule_fires_on_a_line_carrying_a_primitive() {
    use crate::allows::the_pack_would_flag;
    for line in [
        "pub fn f(x: u32) {}",
        "const N: usize = 4;",
        "let y = x as u8;",
        "pub struct S(pub bool);",
        "fn g() -> Outcome<u64, E> { todo!() }",
    ] {
        assert!(the_pack_would_flag(line), "`{line}` was not flagged");
    }
}

#[test]
fn the_control_the_packs_rule_does_not_fire_where_the_pack_does_not() {
    use crate::allows::the_pack_would_flag;
    for line in [
        // A comment line, which the pack returns early on.
        "// lint:allow(no-bare-numeric) reason: const arithmetic",
        // A primitive named only inside a string, which the pack blanks.
        r#"write!(f, "PhaseId({}) is a u32", n)"#,
        // A primitive named only in the trailing comment, which is stripped.
        "let y = x.widen(); // widens to u64",
        // A longer identifier containing a primitive's name.
        "let boolean = compute_u32ish();",
        // The two shapes the corpus actually carries at these sites.
        "Self(Uint::<5>::from_raw(i.0 as _))",
        r#"write!(f, "PhaseId({})", (*self).index().0)"#,
    ] {
        assert!(
            !the_pack_would_flag(line),
            "`{line}` was flagged and the pack would not"
        );
    }
}

#[test]
fn a_literal_suffix_is_not_flagged_which_the_pack_says_of_itself() {
    use crate::allows::the_pack_would_flag;
    // The pack's own header names this as its one gap: a suffix always carries
    // a digit or an underscore in front of the name, so the word-boundary check
    // never matches. Reimplementing the rule means reimplementing the gap, and
    // asserting it is what says the two are the same rule.
    assert!(!the_pack_would_flag("let x = 0u32;"));
    assert!(!the_pack_would_flag("let y = 1_usize;"));
    assert!(!the_pack_would_flag("let z = 0.0_f32;"));
}

#[test]
fn the_control_the_same_values_with_an_annotation_are_flagged() {
    use crate::allows::the_pack_would_flag;
    // Without this the test above passes for a rule that flags nothing at all.
    assert!(the_pack_would_flag("let x: u32 = 0;"));
    assert!(the_pack_would_flag("let y: usize = 1;"));
}

#[test]
fn an_escaped_quote_inside_a_string_does_not_end_the_blanking_early() {
    use crate::allows::the_pack_would_flag;
    // Ending the string early leaves the rest of the line scanned as code, so a
    // primitive named after the escape reads as a real one.
    assert!(!the_pack_would_flag(r#"write!(f, "a \" u32 b", n)"#));
}
