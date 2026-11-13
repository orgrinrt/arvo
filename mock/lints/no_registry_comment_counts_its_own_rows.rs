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
//! A lint rather than a tool. The refused state is a comment stating a count
//! of its own file's rows, there is no legitimate reason to write one, and the
//! repair is deleting a clause.
//!
//! It reads the registry files rather than the registry. A comment is
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

/// The nouns any registry file's comment may count its own rows with, singular
/// then plural. The file's own namespaces are added to these per file: a comment in
/// `ruling.toml` counting "rulings" counts what the file holds, and one in
/// `law.toml` saying "two rulings" is counting something else.
///
/// Deliberately narrow. A comment saying "six of the 242 seed files" is counting
/// something else and is not this lint's business, so the noun has to name what
/// the file itself holds.
const COUNTED: [&str; 4] = ["row", "rows", "entry", "entries"];

/// The determiners a place word may follow, so "The four below" counts and "see row
/// 3 below" and "widths up to 64 follow the same rule" do not.
const DETERMINERS: [&str; 4] = ["the", "these", "those", "all"];

/// The words that stand for the rows of the file in a count with no noun: "The
/// four below were not in the sweep", "six follow".
const PLACE: [&str; 4] = ["below", "above", "follow", "follows"];

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
        let own = own_nouns(&text);
        for (n, line) in text.lines().enumerate() {
            for (count, noun) in counts_in_a_comment(line, &own) {
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

/// The singular and plural of every namespace the file declares with `[[ns]]`.
fn own_nouns(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(ns) = line
            .trim()
            .strip_prefix("[[")
            .and_then(|l| l.strip_suffix("]]"))
        else {
            continue;
        };
        let ns = ns.trim();
        if ns.is_empty() || out.iter().any(|n| n == ns) {
            continue;
        }
        let plural = match ns.strip_suffix('y') {
            Some(stem) => format!("{stem}ies"),
            None => format!("{ns}s"),
        };
        out.push(ns.to_string());
        out.push(plural);
    }
    out
}

/// Whether a noun is one of the singular forms: `row`, `entry` and the file's own
/// namespaces. A singular noun is counted only after the digit `1`, because after
/// any other number it is the first half of a compound (`two entry points`).
fn is_singular(noun: &str, own: &[String]) -> bool {
    matches!(noun, "row" | "entry") || own.iter().step_by(2).any(|n| n == noun)
}

/// The `<number> <noun>` pairs one line states, where the line is a comment.
///
/// A line whose first non-blank character is not `#` is not a comment and is not
/// read, which is what keeps a row value reading like a count out of the report.
fn counts_in_a_comment<'a>(line: &'a str, own: &[String]) -> Vec<(&'a str, String)> {
    let Some(comment) = line.trim_start().strip_prefix('#') else {
        return Vec::new();
    };
    let words: Vec<&str> = comment.split_whitespace().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < words.len() {
        // A number spelled with a space (`twenty two rows`) is counted from its last word.
        let continues = words.get(i + 1).is_some_and(|w| is_count(w));
        if is_count(words[i])
            && !continues
            && let Some((at, noun)) = what_is_counted(&words, i, own)
        {
            out.push((words[i], noun));
            i = at;
        }
        i += 1;
    }
    out
}

/// What the number at `words[from]` counts: a noun this lint names within the next
/// three words (`17 rows`, `17 new rows`, `fifteen of the rows`), or the place
/// word that stands for the rows themselves (`the four below`, `six follow`),
/// with the word index it was found at.
fn what_is_counted(words: &[&str], from: usize, own: &[String]) -> Option<(usize, String)> {
    let ends_a_clause = |w: &str| w.ends_with([',', ';', ':', '.']);
    let alpha = |w: &str| {
        w.trim_matches(|c: char| !c.is_ascii_alphabetic())
            .to_ascii_lowercase()
    };
    let opens_or_follows_a_determiner =
        from == 0 || DETERMINERS.contains(&alpha(words[from - 1]).as_str());
    let digit_one = words[from].trim_matches(|c: char| !c.is_ascii_alphanumeric()) == "1";
    if ends_a_clause(words[from]) {
        return None;
    }
    for at in from + 1 .. words.len().min(from + 4) {
        let word = alpha(words[at]);
        let is_noun = COUNTED.contains(&word.as_str()) || own.contains(&word);
        if is_noun && (!is_singular(&word, own) || digit_one) {
            return Some((at, word));
        }
        if at == from + 1 && opens_or_follows_a_determiner && PLACE.contains(&word.as_str()) {
            return Some((at, word));
        }
        if ends_a_clause(words[at]) {
            return None;
        }
    }
    None
}

/// The number words a count is written in, the units and the tens a compound is
/// built from, and the round numbers.
const NUMBER_WORDS: [&str; 31] = [
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
    "million",
];

/// Whether a word is a number a reader would take as a count.
///
/// Digits, or a number word, or a hyphenated compound of them
/// (`twenty-two`). A count written in words is the same second copy of a fact
/// as one in digits, and three registry comments were written that way and
/// passed this lint when it read digits only. A bare `one` is not read as a count:
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
    let parts: Vec<&str> = lower.split('-').collect();
    // `one` states a shape alone and is a unit inside a compound: `twenty-one`.
    parts
        .iter()
        .all(|p| NUMBER_WORDS.contains(p) || (parts.len() > 1 && *p == "one"))
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

    /// The three comments the ticket was filed over, in the words the registry held
    /// them in before they were reworded. Each says how many rows follow without the
    /// shape `<number> <noun>` the lint first read, and each has to fail.
    #[test]
    fn the_three_comments_the_ticket_names_are_each_reported() {
        let originals = [
            "# The six below were added after the first port of op's rulings, which reported\n",
            "# The four below were not in the sweep that produced this file, because that\n",
            "# is at `109:156-159`, which names eleven axes in one line. Fifteen of the\n\
             # sixteen rows below cite that one file, which is a member file from topic\n",
        ];
        for text in originals {
            let f = findings(&[("a.toml", text)]);
            assert_eq!(f.len(), 1, "{text}: {f:?}");
        }
    }

    /// A count with a word between it and the noun, a compound ending in `one`, and a
    /// place word instead of a noun.
    #[test]
    fn a_count_survives_a_word_before_its_noun_and_a_compound_ending_in_one() {
        let f = findings(&[(
            "a.toml",
            "# Twenty-one rows.\n# Seventeen new rows follow.\n# 17 proposal rows below.\n\
             # The nine above.\n",
        )]);
        assert_eq!(f.len(), 4, "{f:?}");
    }

    #[test]
    fn control_a_number_that_is_not_counting_this_files_rows_stays_silent() {
        let f = findings(&[(
            "a.toml",
            "# Seven axes below.\n# The first six of these were read.\n# one row per crate.\n",
        )]);
        assert!(f.is_empty(), "{f:?}");
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

    /// A file's own namespace is a noun for what it holds: `ruling.toml` counting
    /// `rulings` is counting its rows. The same sentence in `law.toml` is counting
    /// something else, and a plural never turns into a singular match.
    #[test]
    fn a_count_of_the_files_own_namespace_is_reported_and_another_files_is_not() {
        let f = findings(&[
            (
                "ruling.toml",
                "# four rulings below.\n\n[[ruling]]\nid = \"a_ruling_row\"\n",
            ),
            (
                "law.toml",
                "# four rulings below.\n\n[[law]]\nid = \"a_law_row\"\n",
            ),
            (
                "proposal.toml",
                "# Nine proposals follow.\n\n[[proposal]]\nid = \"a_claim\"\n",
            ),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(f.iter().all(|(line, _)| *line == 1), "{f:?}");
    }

    /// Prose that carries a number and a place word, or a number before an
    /// ordinary word that happens to be a counted noun's first half, is not a
    /// count of the file's rows. The comment that does count, beside them, still is.
    #[test]
    fn control_a_number_and_a_place_word_in_ordinary_prose_stay_silent() {
        let f = findings(&[(
            "a.toml",
            "# Widths up to 64 follow the same rule.\n\
             # See row 3 below.\n\
             # The error stays 3 above the bound.\n\
             # The two entry points into this file.\n\
             # The score rows below.\n\
             # The four below were not in the sweep.\n",
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, 6, "only the last comment counts rows: {f:?}");
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
