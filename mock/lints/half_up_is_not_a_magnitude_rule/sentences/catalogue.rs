//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The catalogue: pairs the reader gets wrong on one named side.
//!
//! These are the directions the gate is incomplete in, and the lint warns
//! rather than blocks because they exist. Each entry is a pair like the
//! corpus's, with the side the reader gets wrong named and the other side, the
//! twin, one it gets right. The arm asserting the whole of each pair is
//! ignored as catalogue, so it is red when asked for and does not fail the
//! suite; the arm asserting the twins is not ignored, because a twin going
//! wrong is a regression rather than a known gap.
//!
//! The wrong sides say what the reader would have to read that it cannot. Five
//! are clauses it lets through: a capitalised method name used as an
//! imperative or an ordinary word, and a clause opening on a code span that is
//! a number rather than a name. Three are clauses it refuses: the alias named
//! beside the mode under a name that is also a reading, and a relative or a
//! clause whose subject is a rule or a seat named in words.
//!
//! The last arm reads the live registry's ratified rulings, which the lint
//! itself does not read, and asserts none of their prose is refused. It is
//! red, on the governing ruling's `promotion` field, and ignored as catalogue.

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
];

#[test]
fn the_catalogue_holds_eight_five_let_through_and_three_refused() {
    // A count, so an entry dropped rather than fixed is a failure here.
    assert_eq!(CATALOGUE.len(), 8);
    let through = CATALOGUE
        .iter()
        .filter(|k| matches!(k.wrong, Wrong::LetsThrough))
        .count();
    assert_eq!(through, 5);
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

#[test]
#[ignore = "catalogue: the reader's known wrong readings, both directions; the lint warns rather \
            than blocks because of them"]
fn catalogue_every_known_pair_reads_right() {
    let wrong = wrong_sides(CATALOGUE.iter().map(|k| &k.pair));
    assert!(
        wrong.is_empty(),
        "{} of {} known pairs still wrong:\n{}",
        wrong.len(),
        CATALOGUE.len(),
        wrong.join("\n")
    );
}

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
