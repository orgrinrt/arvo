//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A comment that counts the rows below it is a second copy of a fact.
//!
//! It is right on the day it is written. Then a row lands and nothing tells the
//! author the sentence above it has stopped being true, because a comment is not
//! read by anything. The two then disagree, silently, and the prose is what a
//! later reader quotes: this workspace has already had a prose count say
//! twenty-two while the table held twenty-three and the disk held twenty-four.
//!
//! The cure is not a check that keeps the number current. It is to stop writing
//! the number, because the answer is one query away and cannot go stale. So this
//! reports a count in a comment rather than correcting it.
//!
//! **A lint rather than a tool.** The refused state is a comment stating a count
//! of its own file's rows, there is no legitimate reason to write one, and the
//! repair is deleting a clause.
//!
//! **It reads the registry files rather than the registry.** A comment is
//! discarded by every parser before a row reaches a lint, so the text is where
//! this lives. That also makes it the one canon lint whose whole subject
//! survives in a planted tree, and every arm below drives one.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity, View};

use crate::panel_corpus::{files, finding};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoRegistryCommentCountsItsOwnRows)
}

const NAME: &str = "no-registry-comment-counts-its-own-rows";

/// The nouns a count in a registry comment would be counting.
///
/// Deliberately narrow. A comment saying "six of the 242 seed files" is counting
/// something else and is not this lint's business, so the noun has to name what
/// the file itself holds.
const COUNTED: [&str; 4] = ["row", "rows", "entry", "entries"];

struct NoRegistryCommentCountsItsOwnRows;
impl Lint for NoRegistryCommentCountsItsOwnRows {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoRegistryCommentCountsItsOwnRows {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(ctx.view(), &ctx.mock_dir.join("registry"))
    }
}

/// Every counting comment under one directory of registry files.
fn check(view: &View, dir: &Path) -> Vec<LintError> {
    let mut out = Vec::new();
    for path in files(view, dir, &["toml"]) {
        let Some(text) = view.text(&path) else {
            continue;
        };
        let shown = crate::panel_corpus::shown(&path, dir);
        for (n, line) in text.lines().enumerate() {
            for (count, noun) in counts_in_a_comment(line) {
                out.push(finding(
                    NAME,
                    &shown,
                    n + 1,
                    format!(
                        "the comment says `{count} {noun}`, which is a second copy of a fact \
                         the file already holds. It is right today and nothing will tell \
                         anybody when it stops being. Say what the rows are for and let a \
                         query say how many."
                    ),
                ));
            }
        }
    }
    out
}

/// The `<number> <noun>` pairs one line states, where the line is a comment.
///
/// A line whose first non-blank character is not `#` is not a comment and is not
/// read, which is what keeps a row value reading like a count out of the report.
fn counts_in_a_comment(line: &str) -> Vec<(&str, String)> {
    let Some(comment) = line.trim_start().strip_prefix('#') else {
        return Vec::new();
    };
    let words: Vec<&str> = comment.split_whitespace().collect();
    words
        .windows(2)
        .filter_map(|pair| {
            let noun = pair[1]
                .trim_matches(|c: char| !c.is_ascii_alphabetic())
                .to_ascii_lowercase();
            (is_count(pair[0]) && COUNTED.contains(&noun.as_str())).then(|| (pair[0], noun))
        })
        .collect()
}

/// The number words a count is written in, the units and the tens a compound is
/// built from, and the round numbers.
const NUMBER_WORDS: [&str; 32] = [
    "zero",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
    "hundred",
    "thousand",
    "dozen",
    "score",
    "million",
];

/// Whether a word is a number a reader would take as a count.
///
/// Digits, or a number word, or a hyphenated compound of them
/// (`twenty-two`). A count written in words is the same second copy of a fact
/// as one in digits, and three registry comments were written that way and
/// passed this lint when it read digits only. `one` is not read as a count:
/// "one row per crate" and "one entry whose scope" state a shape, not a tally,
/// and every such comment in the registry did. The digit `1` still is. A
/// compound spelled with a space
/// (`twenty two rows`) is caught by its last word, which is a number word on
/// its own.
fn is_count(word: &str) -> bool {
    let w = word.trim_matches(|c: char| !c.is_ascii_alphanumeric());
    if w.is_empty() {
        return false;
    }
    if w.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    let lower = w.to_ascii_lowercase();
    lower.split('-').all(|part| NUMBER_WORDS.contains(&part))
}

#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block_at,
        assert_not_declared_off,
        assert_registered,
        ctx_at,
        plant,
        planted_tree,
        through_view,
        view,
    };

    /// The messages one planted registry produces, with the line each names.
    fn findings(files: &[(&str, &str)]) -> Vec<(usize, String)> {
        let dir = planted_tree("registry-comments");
        for (at, text) in files {
            plant(&dir, &format!("registry/{at}"), text);
        }
        through_view(&dir, |v| super::check(v, &dir.join("registry")))
            .into_iter()
            .map(|e| (e.line, e.message))
            .collect()
    }

    #[test]
    fn a_comment_counting_rows_is_reported_and_names_the_line() {
        let f = findings(&[(
            "dimension.toml",
            "\n# The axes, all 17 rows of them.\n\n[[dimension]]\nid = \"one\"\n",
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, 2, "the report names the line: {f:?}");
        assert!(f[0].1.contains("17 rows"), "{}", f[0].1);
    }

    #[test]
    fn the_plural_and_the_singular_are_both_caught() {
        let f = findings(&[(
            "a.toml",
            "# There is 1 row here.\n# And 12 entries over there.\n",
        )]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_a_number_about_something_else_is_left_alone() {
        // Without this the lint becomes a ban on numbers in prose and gets
        // switched off, and every arm above would still pass.
        let f = findings(&[(
            "a.toml",
            "# Measured over 242 seed files, at width 13, on 4096 triples.\n\
             # The claim holds at F = 0 and fails above it.\n\
             [[proposal]]\nid = \"a_claim\"\n",
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_count_inside_a_value_is_not_a_comment() {
        // The discrimination that keeps this off the rows themselves. A row may
        // legitimately say how many of something a sweep covered, and that
        // sentence is a claim rather than a stale copy of the file's own shape.
        let f = findings(&[(
            "a.toml",
            "[[proposal]]\nid = \"a_claim\"\nsays = \"the sweep covered 6 rows of the table\"\n",
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// The planted case the ticket asks for: three comments counting their rows
    /// in words passed this lint when it read digits only. It has to fail now.
    #[test]
    fn a_count_written_in_words_is_reported() {
        let f = findings(&[(
            "a.toml",
            "# Seventeen rows, spelled out.\n\
             # There are twenty-two entries below.\n\
             # About a dozen rows follow.\n\
             # Twenty two rows in all.\n",
        )]);
        assert_eq!(f.len(), 4, "{f:?}");
        assert!(f[0].1.contains("Seventeen rows"), "{}", f[0].1);
        assert!(f[1].1.contains("twenty-two entries"), "{}", f[1].1);
        assert!(f[2].1.contains("dozen rows"), "{}", f[2].1);
        assert!(f[3].1.contains("two rows"), "{}", f[3].1);
    }

    /// `one` states a shape and is not a tally, so these stay silent while the
    /// digit form beside them is still caught.
    #[test]
    fn control_one_row_per_crate_is_a_shape_and_one_digit_is_a_count() {
        let f = findings(&[(
            "a.toml",
            "# one row per crate, and one entry whose scope was limited.\n# 1 row below.\n",
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, 2);
    }

    /// The controls for the arm above: a word that is no number, a hyphenated
    /// word that is not a compound of numbers, and a number with no noun.
    #[test]
    fn control_a_comment_with_no_number_and_a_number_with_no_noun_are_both_silent() {
        let f = findings(&[(
            "a.toml",
            "# The axes this registry turns on.\n# 17 axes.\n# Seventeen axes.\n\
             # Some rows follow.\n# A well-known rows table.\n# Twenty-odd rows.\n",
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_nested_registry_file_is_read_and_a_non_toml_file_is_not() {
        let f =
            findings(&[("nested/b.toml", "# 4 rows below.\n"), ("notes.md", "# 9 rows below.\n")]);
        assert_eq!(
            f.len(),
            1,
            "a nested file counts and a markdown file does not: {f:?}"
        );
    }

    #[test]
    fn a_registry_directory_that_is_not_there_is_silent_rather_than_a_panic() {
        let dir = planted_tree("registry-comments-absent");
        assert!(through_view(&dir, |v| super::check(v, &dir.join("registry"))).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("registry-comments-severity");
        plant(&dir, "registry/a.toml", "# all 17 rows of them\n");
        let empty = view(&[], &[]);
        assert_findings_block_at(
            &super::NoRegistryCommentCountsItsOwnRows,
            ctx_at(&dir, &empty),
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoRegistryCommentCountsItsOwnRows);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::NoRegistryCommentCountsItsOwnRows.name(), super::NAME);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
