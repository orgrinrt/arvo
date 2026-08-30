//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A count in a comment is a second copy of a fact, and it drifts silently.

use arvo_checks::{canon, comments, parse};

#[test]
fn no_committed_registry_comment_counts_its_own_rows() {
    let found = comments::comments_counting_their_own_rows(&canon());
    assert!(
        found.is_empty(),
        "a comment states how many rows its file holds, which is right today and will not \
         announce the day it stops being: {found:#?}"
    );
}

#[test]
fn a_comment_counting_rows_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
# The axes, all 17 rows of them.

[[dimension]]
id = "one"
what = "an axis"
"#,
    );
    let found = comments::comments_counting_their_own_rows(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "a-comment-counts-its-own-rows");
    assert!(found[0].at.ends_with(":2"), "the report names the line: {}", found[0].at);
}

#[test]
fn the_plural_and_the_singular_are_both_caught() {
    let reg = parse(
        "planted.toml",
        r#"
# There is 1 row here.
# And 12 entries over there.
"#,
    );
    assert_eq!(comments::comments_counting_their_own_rows(&reg).len(), 2);
}

/// A number in a comment about something other than this file's rows is left
/// alone, or the arm becomes a ban on numbers in prose and gets switched off.
#[test]
fn a_number_about_something_else_is_left_alone() {
    let reg = parse(
        "planted.toml",
        r#"
# Measured over 242 seed files, at width 13, on 4096 triples.
# The claim holds at F = 0 and fails above it.

[[proposal]]
id = "a_claim"
"#,
    );
    assert!(
        comments::comments_counting_their_own_rows(&reg).is_empty(),
        "the arm is about a file counting itself, not about arithmetic in prose"
    );
}

/// A row value that happens to read like a count is not a comment.
#[test]
fn a_count_inside_a_value_is_not_a_comment() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
says = "the sweep covered 6 rows of the table"
"#,
    );
    assert!(comments::comments_counting_their_own_rows(&reg).is_empty());
}
