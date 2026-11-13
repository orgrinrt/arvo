//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The catalogue: pairs the reader gets wrong on one named side.
//!
//! These are the directions the gate is incomplete in, and the lint warns
//! rather than blocks because they exist. Each entry is a pair like the
//! corpus's, with the side the reader gets wrong named and the other side, the
//! twin, one it gets right. Every entry owns one test asserting the reading its
//! wrong side is owed, ignored as catalogue so it is red when asked for and does
//! not fail the suite, each under the ticket that tracks its own shape. The arm
//! asserting the twins is not ignored, because a twin going wrong is a
//! regression rather than a known gap. A pair the reader learns to read moves
//! to `closed.rs`, with a test of its own.
//!
//! Three entries are left. One is not a gap the reader can close alone: a
//! pronoun after a negated identity across a full stop, which the entry reads
//! as the sentence's subject and `carry_tests::every_other_name_defeats_the_carry`
//! reads as the rule standing last, and its ticket holds which of the two
//! governs. The other two are what remains of two classes the reader now reads
//! by closed lists: an imperative whose object opens on a word of an open class,
//! and a subject named in words no list holds.
//!
//! The last arm reads the live registry's ratified rulings, which the lint
//! itself does not read, and asserts none of their prose is refused. It held
//! the governing ruling's `promotion` field, which opens a clause on a seat's
//! number, until a clause opening on a bare whole number was read as having a
//! subject of its own; it is asserted now.
//!
//! Test code only: `sentences` is declared under `#[cfg(test)]`, and the
//! attribute below says so in this file too, where the read-through-the-view
//! guard looks for it.

#![cfg(test)]

use std::path::{Path, PathBuf};

use super::super::NOT_PROSE;
use super::{Pair, fires_right, found, silent_right, wrong_sides};

/// Which side of a catalogued pair the reader gets wrong.
pub(super) enum Wrong {
    /// It is silent on the sentence that has to fire.
    LetsThrough,
    /// It fires on the sentence that has to be silent.
    Refuses,
}

/// A pair the reader gets wrong on one side, and which side.
pub(super) struct Known {
    pub(super) pair:  Pair,
    pub(super) wrong: Wrong,
}

impl Known {
    /// The sentence the reader gets wrong.
    pub(super) fn wrong_side(&self) -> &'static str {
        match self.wrong {
            Wrong::LetsThrough => self.pair.fires,
            Wrong::Refuses => self.pair.silent,
        }
    }

    /// Whether the reader gets the twin, the side it is not known to get wrong,
    /// right.
    fn twin_right(&self) -> bool {
        match self.wrong {
            Wrong::LetsThrough => silent_right(&self.pair),
            Wrong::Refuses => fires_right(&self.pair),
        }
    }
}

/// Every known wrong reading, with its twin.
pub(super) const CATALOGUE: &[Known] = &[
    Known {
        pair:  Pair {
            fires:    "`half_up` is not Java's `HALF_UP`. It sends a tie away from zero.",
            silent:   "`half_up` is not the `HALF_UP` of Java. That one goes away from zero.",
            subject:  None,
            differs:  "whether the pronoun following a negated identity is `It`, referring to the \
                      sentence's subject, or `that one`, referring to what the identity denied",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is nearest. Round halfway ties away from zero.",
            silent:   "`half_up` is nearest. Round sends halfway ties away from zero.",
            subject:  None,
            differs:  "whether `Round` opening the clause is an imperative whose object opens on a \
                      word of an open class, or the method followed by its verb",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is a mode. Its decimal module sends a tie away from zero.",
            silent:   "`half_up` is a mode. Python's decimal module sends a tie away from zero.",
            subject:  None,
            differs:  "whether the clause's subject is the mode's own, by a pronoun, or a thing \
                      named in words the reader has no noun for",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
];

#[test]
fn the_catalogue_holds_three_two_let_through_and_one_refused() {
    // A count, so an entry dropped rather than fixed or deleted is a failure
    // here. Seventeen of the eighteen it held moved to `closed.rs`, and two
    // entries are the residue of the classes two of those closed.
    assert_eq!(CATALOGUE.len(), 3);
    let through = CATALOGUE
        .iter()
        .filter(|k| matches!(k.wrong, Wrong::LetsThrough))
        .count();
    assert_eq!(through, 2);
}

#[test]
fn every_twin_in_the_catalogue_reads_right() {
    // The side of each pair the reader is not known to get wrong. This is a
    // regression arm rather than a catalogue one: a twin going wrong is the
    // reader getting worse.
    for k in CATALOGUE {
        assert!(k.twin_right(), "{}", k.pair.differs);
    }
}

/// `CATALOGUE[i]`'s known-wrong side, isolated so a fix landing on one entry
/// turns only that entry's test green.
fn known_wrong(i: usize) -> Vec<String> {
    wrong_sides(std::iter::once(&CATALOGUE[i].pair))
}

/// One catalogued entry, as a test asserting the reading the reader owes on its
/// known-wrong side. Red while the gap is open, which is what the catalogue
/// ignore is for, per entry, so a reader running `--ignored` sees which specific
/// class is still open and a fix turns only its own entry green.
macro_rules! catalogue_case {
    ($(#[$attr:meta])* $name:ident, $i:expr) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            let wrong = known_wrong($i);
            assert!(
                wrong.is_empty(),
                "CATALOGUE[{}]'s named side is still read wrong: {wrong:?}. Once it reads \
                 right, the entry moves out of the catalogue and into `closed.rs`",
                $i
            );
        }
    };
}

catalogue_case!(
    #[ignore = "catalogue: the pronoun `It` after a negated identity named in a code span is read as \
                the rule standing last, as every_other_name_defeats_the_carry asserts, not as the \
                sentence's subject; tracked #arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi"]
    the_java_pair_it_after_a_negated_identity_lets_through,
    0
);
catalogue_case!(
    #[ignore = "catalogue: a capitalised method name opening a clause is read as the method where \
                the word behind it is of an open class, so an imperative on such an object is let \
                through; tracked #arvo_s_half_up_reader_reads_a_capitalised_method_name_as_the"]
    an_imperative_on_an_open_class_object_is_read_as_the_method,
    1
);
catalogue_case!(
    #[ignore = "catalogue: a clause whose subject is named in words outside `SUBJECTS` and every list \
                of rules is read as naming nothing, so the mode carries into it and it is refused; \
                tracked #arvo_s_half_up_reader_refuses_a_clause_whose_subject_is_name"]
    a_subject_named_in_words_outside_the_vocabulary_is_refused,
    2
);

/// The repository this file sits in, found from its own path.
fn repo_root() -> PathBuf {
    let here = Path::new(file!());
    let mut dir = if here.is_absolute() {
        here.to_path_buf()
    } else {
        std::env::current_dir()
            .expect("a working directory")
            .join(here)
    };
    // `<root>/mock/lints/<lint>/sentences/catalogue.rs` is five components down.
    for _ in 0 .. 5 {
        dir.pop();
    }
    dir
}

/// A TOML basic string's body with its escapes resolved, for the escapes the
/// registry writes.
fn unescaped(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut chars = body.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => {},
        }
    }
    out
}

/// Every one-line string field of every ratified row in the ruling file that
/// the lint would read as prose, as `(row, field, value)`.
fn ratified_prose(text: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for row in text.split("\n[[") {
        if !row.lines().any(|l| l.trim() == "rung = \"ratified\"") {
            continue;
        }
        let id = row
            .lines()
            .find_map(|l| l.strip_prefix("id = \""))
            .map(|s| s.trim_end_matches('"').to_string())
            .unwrap_or_default();
        for line in row.lines() {
            let Some((key, value)) = line.split_once(" = \"") else {
                continue;
            };
            if key.contains(' ') || !value.ends_with('"') || NOT_PROSE.contains(&key) {
                continue;
            }
            let body = &value[.. value.len() - 1];
            out.push((id.clone(), key.to_string(), unescaped(body)));
        }
    }
    out
}

#[test]
fn control_the_ratified_rows_are_read_and_the_reader_fires_on_a_planted_field() {
    // The arm below reads a file and could read nothing. This says it reads the
    // governing ruling, skips what the lint skips, and that a field giving the
    // mode the reading is refused by the same call.
    let text = std::fs::read_to_string(repo_root().join("mock/registry/ruling.toml"))
        .expect("the ruling file");
    let fields = ratified_prose(&text);
    assert!(
        fields.iter().any(|(id, key, _)| {
            id == "half_up_denotes_ties_toward_positive_infinity" && key == "says"
        }),
        "the governing ruling's `says` is not among the fields read"
    );
    let planted = "[[ruling]]\nid = \"p\"\nrung = \"ratified\"\nnote = \"`half_up` takes a tie \
                   away from zero.\"\nquote = \"`half_up` takes a tie away from zero.\"\n";
    let read = ratified_prose(&format!("\n{planted}"));
    assert_eq!(read.len(), 1, "{read:?}");
    assert_eq!(read[0].1, "note");
    assert!(!found(&read[0].2, None).is_empty());
    let unratified = planted.replace("ratified", "stated");
    assert!(ratified_prose(&format!("\n{unratified}")).is_empty());
}

#[test]
fn no_ratified_ruling_field_is_refused() {
    let text = std::fs::read_to_string(repo_root().join("mock/registry/ruling.toml"))
        .expect("the ruling file");
    let refused: Vec<String> = ratified_prose(&text)
        .into_iter()
        .filter(|(_, _, v)| !found(v, None).is_empty())
        .map(|(id, key, _)| format!("{id}: {key}"))
        .collect();
    assert!(refused.is_empty(), "{}", refused.join("\n"));
}
