//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A comment that counts the rows below it is a second copy of a fact.
//!
//! It is right on the day it is written. Then a row lands and nothing tells the
//! author the sentence above it has stopped being true, because a comment is
//! not read by anything. The two then disagree, silently, and the prose is what
//! a later reader quotes: this workspace has already had a prose count say
//! twenty-two while the table held twenty-three and the disk held twenty-four.
//!
//! The cure is not a check that keeps the number current. It is to stop
//! writing the number, because the answer is one command away and cannot go
//! stale. So this arm reports a count in a comment rather than correcting it.

use crate::{Finding, Registry};

/// The nouns a count in a registry comment would be counting.
///
/// Deliberately narrow. A comment saying "six of the 242 seed files" is
/// counting something else and is not this arm's business, so the noun has to
/// name what this file holds.
const COUNTED: &[&str] = &["row", "rows", "entry", "entries"];

/// Whether a word is a number a reader would take as a count.
///
/// Digits only. A spelled-out number is not caught, which is stated rather than
/// hidden: this arm is a tripwire on the shape that actually recurs, and a
/// reader who writes "seven rows" in words has gone out of their way.
fn is_count(word: &str) -> bool {
    let w = word.trim_matches(|c: char| !c.is_ascii_alphanumeric());
    !w.is_empty() && w.chars().all(|c| c.is_ascii_digit())
}

/// Comments in a registry file that state how many rows it holds.
pub fn comments_counting_their_own_rows(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for (file, text) in &reg.texts {
        for (n, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            let Some(comment) = trimmed.strip_prefix('#') else {
                continue;
            };
            let words: Vec<&str> = comment.split_whitespace().collect();
            for pair in words.windows(2) {
                let noun = pair[1]
                    .trim_matches(|c: char| !c.is_ascii_alphabetic())
                    .to_ascii_lowercase();
                if is_count(pair[0]) && COUNTED.contains(&noun.as_str()) {
                    out.push(Finding::new(
                        "a-comment-counts-its-own-rows",
                        format!("{}:{}", file.display(), n + 1),
                        format!(
                            "the comment says `{} {}`, which is a second copy of a fact the \
                             file already holds. It is right today and nothing will tell \
                             anybody when it stops being. Say what the rows are for and let \
                             a query say how many.",
                            pair[0], pair[1]
                        ),
                    ));
                }
            }
        }
    }
    out
}
