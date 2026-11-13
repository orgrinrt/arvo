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
//! The entries themselves are in `known.rs`, sixteen of them, and the lint's
//! module doc names each class. Six are not a gap the reader can close alone:
//! how a pronoun resolves, which the entries read one way and
//! `carry_tests::every_other_name_defeats_the_carry` reads the other, and one
//! question ticket holds which governs. The other ten are what remains of the
//! classes the reader now reads by closed lists or by a rule over a few words:
//! an imperative on an open-class object, a subject in words no list holds,
//! the numbers `COUNTED` reads as counts, a count before a verb, a motion
//! gerund `MOTIONS` lacks, a gerund saying how the mode is computed, a
//! predicating copula, and a rule named by a hyphenated phrase alone.
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
use super::known::CATALOGUE;
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

#[test]
fn the_catalogue_holds_sixteen_seven_let_through_and_nine_refused() {
    // A count, so an entry dropped rather than fixed or deleted is a failure
    // here. Sixteen of the eighteen it first held moved to `closed.rs`; two
    // entries are the residue of the classes two of those closed, and the
    // twelve after them the residue of the lists the reader reads by.
    assert_eq!(CATALOGUE.len(), 16);
    let through = CATALOGUE
        .iter()
        .filter(|k| matches!(k.wrong, Wrong::LetsThrough))
        .count();
    assert_eq!(through, 7);
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
catalogue_case!(
    #[ignore = "catalogue: the pronoun `it` opening a segment of the clause that denies the mode is \
                another rule is read as that rule, the one standing last, which is the same open \
                call as the Java pair; tracked #arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi"]
    a_pronoun_in_one_clause_after_a_denied_rule_lets_through,
    3
);
catalogue_case!(
    #[ignore = "catalogue: a rule named by a hyphenated phrase alone, behind a negator, is read as \
                a quality, so the relative after it is the mode's and the clause is refused; \
                tracked #arvo_s_half_up_reader_takes_a_rule_named_only_by_a_hyphenate"]
    a_rule_named_by_a_hyphenated_phrase_is_read_as_a_quality,
    4
);
catalogue_case!(
    #[ignore = "catalogue: `it` in a clause opening on a subject is read as the mode, though here \
                it is what the seat read, the open pronoun call; tracked \
                #arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi"]
    it_after_a_seat_is_read_as_the_mode,
    5
);
catalogue_case!(
    #[ignore = "catalogue: `it` in an aside of a subject clause is read as the mode, though it is \
                the subject, the open pronoun call; tracked \
                #arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi"]
    it_in_an_aside_of_the_subject_is_read_as_the_mode,
    6
);
catalogue_case!(
    #[ignore = "catalogue: `it` behind the reading in a subject clause is read as the mode, though \
                it is the subject, the open pronoun call; tracked \
                #arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi"]
    it_behind_the_reading_of_a_subject_is_read_as_the_mode,
    7
);
catalogue_case!(
    #[ignore = "catalogue: a personal pronoun in the alias's segment keeps the alias a reading, \
                though here the pronoun is the alias, the open pronoun call; tracked \
                #arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi"]
    a_pronoun_for_the_alias_keeps_it_a_reading,
    8
);
catalogue_case!(
    #[ignore = "catalogue: a seat number before `in` is read as a count, so the clause carries the \
                mode and is refused; tracked #arvo_s_half_up_reader_reads_a_bare_number_before_or_or_in_as"]
    a_seat_before_in_a_place_is_read_as_a_count,
    9
);
catalogue_case!(
    #[ignore = "catalogue: a seat number before `in` is read as a count, so the clause carries the \
                mode and is refused; tracked #arvo_s_half_up_reader_reads_a_bare_number_before_or_or_in_as"]
    a_seat_before_in_its_answer_is_read_as_a_count,
    10
);
catalogue_case!(
    #[ignore = "catalogue: two seat numbers joined by `or` are read as a count, so the clause \
                carries the mode and is refused; tracked \
                #arvo_s_half_up_reader_reads_a_bare_number_before_or_or_in_as"]
    two_seats_joined_by_or_are_read_as_a_count,
    11
);
catalogue_case!(
    #[ignore = "catalogue: a bare number before a verb is read as a seat, so a count of ties takes \
                the clause from the mode; tracked #arvo_s_half_up_reader_reads_a_bare_number_before_a_verb_as_a"]
    a_count_before_a_verb_is_read_as_a_seat,
    12
);
catalogue_case!(
    #[ignore = "catalogue: a motion gerund outside `MOTIONS` is read as naming `toward zero`, so \
                the clause names another rule and is let through; tracked \
                #arvo_s_half_up_reader_misses_a_motion_gerund_outside_motions"]
    a_motion_gerund_outside_the_list_names_the_rule,
    13
);
catalogue_case!(
    #[ignore = "catalogue: a gerund naming `toward zero` takes the relative behind it, though the \
                clause says the mode is computed that way; tracked \
                #arvo_s_half_up_reader_reads_a_gerund_naming_toward_zero_befo"]
    a_relative_after_how_the_mode_is_computed_goes_to_the_gerund,
    14
);
catalogue_case!(
    #[ignore = "catalogue: a copula before a negator behind the reading is read as a denied \
                identity even where it predicates a quality; tracked \
                #arvo_s_half_up_reader_reads_any_copula_before_a_negator_as_a"]
    a_predicating_copula_is_read_as_an_identity,
    15
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
