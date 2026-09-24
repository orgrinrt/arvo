//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The catalogue: pairs the reader gets wrong on one named side.
//!
//! These are the directions the gate is incomplete in, and the lint warns
//! rather than blocks because they exist. Each entry is a pair like the
//! corpus's, with the side the reader gets wrong named and the other side, the
//! twin, one it gets right. Every entry owns one test asserting its wrong side
//! stays wrong, ignored as catalogue so it is red when asked for and does not
//! fail the suite; a fix landing on one entry fails only that entry's test
//! rather than an aggregate that would still report red for the other
//! seventeen. The arm asserting the twins is not ignored, because a twin going
//! wrong is a regression rather than a known gap.
//!
//! The wrong sides say what the reader would have to read that it cannot.
//! Twelve are clauses it lets through: a capitalised method name used as an
//! imperative or an ordinary word, a clause opening on a code span that is a
//! number rather than a name, a negator standing behind the reading with no
//! comma before it, a denotation excusing a reading in a later segment with
//! only a comma between them, a one-word aside read as a list neighbour, a
//! lower-case entry in `OTHER_NAMES` matching ordinary English, a pronoun in
//! one clause resolved to the most recent name rather than to the sentence's
//! subject, `value` following the reading as a predicate rather than as a
//! quoting word, and the pronoun after a negated identity named in a code
//! span. Six are clauses it refuses: the alias named beside the mode under a
//! name that is also a reading, a relative or a clause whose subject is a
//! rule or a seat named in words, a direction word in front of `in magnitude`
//! that names how an error grows rather than where a tie goes, a segment
//! naming a rule under a word that is also a reading where the mode names
//! itself instead, and a clause naming its own subject in words rather than
//! in a code span.
//!
//! The last arm reads the live registry's ratified rulings, which the lint
//! itself does not read, and asserts none of their prose is refused. It is
//! red, on the governing ruling's `promotion` field, and ignored as catalogue.
//! `check_registry` skips every ratified ruling outright, so the gate never
//! reads that field: the real reason the demotion stands is the shapes above
//! that the gate does read, in `mock/lints/half_up_is_not_a_magnitude_rule.rs`'s
//! own module doc and `SEVERITY` doc.

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
            fires:    "`half_up` is nearest. Round every tie away from zero.",
            silent:   "`half_up` is nearest. MATLAB's Round takes every tie away from zero.",
            subject:  None,
            differs:  "whether `Round` opening the clause is an imperative or the MATLAB method",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is the mode. Fix the doc so a tie goes away from zero.",
            silent:   "`half_up` is the mode. MATLAB's Fix sends a tie away from zero.",
            subject:  None,
            differs:  "whether `Fix` opening the clause is an imperative or the MATLAB method",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is the mode. Ceiling aside, a tie goes away from zero.",
            silent:   "`half_up` is the mode. Ceiling sends a tie away from zero.",
            subject:  None,
            differs:  "whether `Ceiling` opens an aside or is the subject that sends the tie",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is nearest. Round to it and a tie goes away from zero.",
            silent:   "`half_up` is nearest. Round is what goes away from zero.",
            subject:  None,
            differs:  "whether `Round` is a verb taking the mode as its object or the method",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is a mode. `-2.5` goes away from zero.",
            silent:   "`half_up` is a mode. `arvo-format` sends a tie away from zero.",
            subject:  None,
            differs:  "whether the code span opening the clause is a value the mode acts on or a \
                      subject of its own",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is nearest, whereas it goes away from zero.",
            silent:   "`half_up` is nearest, whereas `roundTiesToAway` goes away from zero.",
            subject:  None,
            differs:  "whether the contrasted segment names the alias, under a name that is also a \
                      reading",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is nearest, which goes away from zero.",
            silent:   "`half_up` is not Java's rule, which goes away from zero.",
            subject:  None,
            differs:  "whether the relative binds to the mode or to a rule named in words",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up(x) = floor(x + q/2)` is stated first. It then answers ties away \
                      from zero.",
            silent:   "Seat 125 stated `half_up(x) = floor(x + q/2)` first. Seat 270 read blind \
                      and first answered ties away from zero.",
            subject:  None,
            differs:  "whether the clause after the naming opens on a pronoun or on a subject named \
                      in words, which is the shape of the governing ruling's `promotion`",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    // The seven classes the fourth review found uncatalogued, each a class
    // rather than the one sentence it happened to look at.
    Known {
        pair:  Pair {
            fires:    "`half_up` sends a tie away from zero rather than toward positive infinity.",
            silent:   "`half_up` sends a tie toward positive infinity rather than away from zero.",
            subject:  None,
            differs:  "whether the reading or the denotation stands last before `rather than` in \
                      the reading's own segment",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` goes toward positive infinity, away from zero.",
            silent:   "`half_up` sends a tie toward positive infinity, where ties away from zero \
                      would give -3.",
            subject:  None,
            differs:  "whether the reading follows the denotation by a bare comma or by a comma \
                      introducing a contrast",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` rounds x (a tie) away from zero.",
            silent:   "The modes floor, away from zero, half_up (the ruled one), and ceil.",
            subject:  None,
            differs:  "whether a one-word aside between the name and the reading is read as a list \
                      neighbour or the item beside it holds a real list",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` doesn't send a tie toward zero; it sends it away from zero.",
            silent:   "`half_up` differs from `TowardZero`; it sends a tie away from zero.",
            subject:  None,
            differs:  "whether `toward zero` names the alias `TowardZero` in a code span or merely \
                      uses the ordinary English words",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` differs from `floor`, and it sends a tie away from zero.",
            silent:   "`half_up` differs from `floor`. It sends a tie away from zero.",
            subject:  None,
            differs:  "whether the pronoun opens a segment joined by a comma and a conjunction \
                      within one clause, where the subject governs, or opens the next clause after \
                      a full stop, where the name standing last does",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` returns the `away from zero` value at a tie.",
            silent:   "`half_up` has the `away from zero` value in it.",
            subject:  None,
            differs:  "whether `value` follows the span as a predicate the sentence asserts or as a \
                      word saying the span is quoted",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` moves a tie up in magnitude.",
            silent:   "`half_up` has an error that increases in magnitude with the width.",
            subject:  None,
            differs:  "whether the direction word stands in front of `in magnitude` as the tie's \
                      own direction or names how an error grows",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    // The three pinned pairs the fourth review's blocker 2 settled: the
    // coordinator's intent for each, moving the side that contradicted it here.
    Known {
        pair:  Pair {
            fires:    "`half_up` is a nearest rule that reads nothing and so falls on the \
                      translation side, where `half_up` itself sends a tie away from zero.",
            silent:   "`half_up` is a nearest rule that reads nothing and so falls on the \
                      translation side, where the ties-away rule would fall on the reflection \
                      side.",
            subject:  None,
            differs:  "whether the segment names the mode itself or a rule named by a word that is \
                      also a reading",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is a mode. It sends a tie away from zero.",
            silent:   "`half_up` is a mode. The crate sends a tie away from zero.",
            subject:  None,
            differs:  "whether the clause opens with a pronoun for the mode or names a subject of \
                      its own in words, neither a claim about what the mode denotes",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
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
];

#[test]
fn the_catalogue_holds_eighteen_twelve_let_through_and_six_refused() {
    // A count, so an entry dropped rather than fixed is a failure here.
    assert_eq!(CATALOGUE.len(), 18);
    let through = CATALOGUE
        .iter()
        .filter(|k| matches!(k.wrong, Wrong::LetsThrough))
        .count();
    assert_eq!(through, 12);
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
/// fails only that entry's test rather than an aggregate covering all
/// eighteen.
fn known_wrong(i: usize) -> Vec<String> {
    wrong_sides(std::iter::once(&CATALOGUE[i].pair))
}

/// One catalogued entry, as a test asserting its known-wrong side stays
/// wrong. Ignored as catalogue, per entry, so a reader running `--ignored`
/// sees which specific class is still open rather than one aggregate result.
macro_rules! catalogue_case {
    ($name:ident, $i:expr, $reason:expr) => {
        #[test]
        #[ignore = $reason]
        fn $name() {
            let wrong = known_wrong($i);
            assert!(
                !wrong.is_empty(),
                "CATALOGUE[{}] now reads right; move it out of the catalogue \
                 and into the corpus",
                $i
            );
        }
    };
}

catalogue_case!(
    a_matlab_method_opening_a_clause_is_read_as_an_imperative,
    0,
    "catalogue: `Round` opening the clause reads as an imperative rather than the MATLAB method"
);
catalogue_case!(
    a_matlab_fix_opening_a_clause_is_read_as_an_imperative,
    1,
    "catalogue: `Fix` opening the clause reads as an imperative rather than the MATLAB method"
);
catalogue_case!(
    ceiling_opening_an_aside_is_read_as_the_clauses_subject,
    2,
    "catalogue: `Ceiling` opening an aside reads as the subject that sends the tie"
);
catalogue_case!(
    round_as_a_verb_taking_the_mode_is_read_as_the_method,
    3,
    "catalogue: `Round` as a verb taking the mode as its object reads as the MATLAB method"
);
catalogue_case!(
    a_code_span_value_opening_a_clause_is_read_as_a_subject,
    4,
    "catalogue: a code span value the mode acts on reads as a subject of its own"
);
catalogue_case!(
    the_alias_named_under_a_reading_word_is_refused,
    5,
    "catalogue: the alias named in a contrasted segment, under a name that is also a reading, is \
     refused"
);
catalogue_case!(
    a_relative_binding_to_a_rule_named_in_words_is_refused,
    6,
    "catalogue: a relative clause binds to a rule named in words rather than to the mode"
);
catalogue_case!(
    a_pronoun_after_a_seat_named_promotion_reaches_the_reading,
    7,
    "catalogue: the governing ruling's `promotion` opens a clause on a seat number, a subject the \
     reader has no word for, so the carry reaches it"
);
catalogue_case!(
    the_denotation_standing_last_before_rather_than_does_not_excuse,
    8,
    "catalogue: the reading standing last before `rather than` in its own segment is not excused by \
     the denotation ahead of it"
);
catalogue_case!(
    a_reading_after_a_contrast_comma_is_not_excused_by_the_denotation,
    9,
    "catalogue: a reading following the denotation by a comma introducing a contrast is not excused, \
     only a bare comma is"
);
catalogue_case!(
    a_one_word_aside_beside_the_reading_reads_as_a_list_neighbour,
    10,
    "catalogue: a one-word aside between the name and the reading reads as a list neighbour"
);
catalogue_case!(
    toward_zero_in_ordinary_words_is_read_as_the_alias,
    11,
    "catalogue: `toward zero` in ordinary English words, not a code span naming `TowardZero`, is \
     read as the alias"
);
catalogue_case!(
    a_pronoun_after_a_full_stop_carries_to_the_wrong_name,
    12,
    "catalogue: a pronoun opening the clause after a full stop carries to the name standing last \
     rather than to the sentence's subject"
);
catalogue_case!(
    value_following_the_span_as_a_predicate_reads_as_a_quoting_word,
    13,
    "catalogue: `value` following the span as a predicate the sentence asserts reads as the word \
     that makes the span a quote"
);
catalogue_case!(
    a_direction_word_before_in_magnitude_reads_an_errors_growth,
    14,
    "catalogue: a direction word in front of `in magnitude` naming how an error grows is refused as \
     the tie's own direction"
);
catalogue_case!(
    pair_22_the_ties_away_rule_named_in_words_is_refused,
    15,
    "catalogue: a segment naming a rule under a word that is also a reading, where the mode names \
     itself instead, is refused"
);
catalogue_case!(
    pair_35_the_crate_named_as_subject_is_refused,
    16,
    "catalogue: a clause naming its own subject in words rather than in a code span is refused"
);
catalogue_case!(
    the_java_pair_it_after_a_negated_identity_lets_through,
    17,
    "catalogue: the pronoun `It` after a negated identity named in a code span is not carried, so \
     the clause it opens is let through"
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
#[ignore = "catalogue: the governing ruling's `promotion` opens a clause on a seat number, a \
            subject the reader has no word for, so the carry reaches it"]
fn catalogue_no_ratified_ruling_field_is_refused() {
    let text = std::fs::read_to_string(repo_root().join("mock/registry/ruling.toml"))
        .expect("the ruling file");
    let refused: Vec<String> = ratified_prose(&text)
        .into_iter()
        .filter(|(_, _, v)| !found(v, None).is_empty())
        .map(|(id, key, _)| format!("{id}: {key}"))
        .collect();
    assert!(refused.is_empty(), "{}", refused.join("\n"));
}
