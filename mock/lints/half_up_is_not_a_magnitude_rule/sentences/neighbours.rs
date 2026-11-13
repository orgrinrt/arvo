//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The neighbours of the closed pairs: sentences that sit next to a closed
//! shape and that a list widened to close it would read wrong.
//!
//! A closed pair says the reader reads one sentence of a shape right. What it
//! cannot say is that the list it was closed with stops where it should, since
//! the next sentence over, with one word moved, is outside the pair. Each entry
//! here is such a sentence, beside a twin on the other side of the line, and
//! each shape has at least one entry in each direction where it has one: a true
//! sentence the widened list refuses, and a false one it lets through.
//!
//! They are kept apart from `corpus.rs` and `closed.rs` because each of those
//! counts its own pairs, and these are owed to the closed list rather than to
//! the corpus.

use super::{Pair, wrong_sides};

/// The neighbours, grouped by the closed shape they sit next to.
pub(super) const NEIGHBOURS: &[Pair] = &[
    // The method name followed by a preposition: a preposition is the
    // imperative only where an object follows it.
    Pair {
        fires:    "`half_up` sends a tie toward positive infinity. Round to the slot, and a tie goes \
                  away from zero.",
        silent:   "`half_up` sends a tie toward positive infinity. Round to nearest sends a tie away \
                  from zero.",
        subject:  None,
        differs:  "whether the preposition behind `Round` takes an object or names the method's \
                  own rounding",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is floor(x + q/2). Round toward it, and it sends a tie away from zero.",
        silent:   "`half_up` is floor(x + q/2). Round toward nearest is MATLAB's, and it sends a tie \
                  away from zero.",
        subject:  None,
        differs:  "whether `toward` behind `Round` takes a pronoun as its object or a word naming \
                  the method",
        measured: false,
    },
    Pair {
        fires:    "`half_up` sends a tie toward positive infinity. Round to it and it goes away from \
                  zero in `fi`.",
        silent:   "`half_up` sends a tie toward positive infinity. Round to nearest goes away from \
                  zero in `fi`.",
        subject:  None,
        differs:  "whether `to` behind `Round` takes a pronoun or names the method's rounding",
        measured: false,
    },
    // A code span holding a number, in a clause that names a rule in words.
    Pair {
        fires:    "`half_up` maps `-2.5` to `-2`. `-2.5` under its rule goes away from zero.",
        silent:   "`half_up` maps `-2.5` to `-2`. `-2.5` under Java's rule goes away from zero.",
        subject:  None,
        differs:  "whether the rule the clause names is the mode's own or one named by its owner",
        measured: false,
    },
    Pair {
        fires:    "`half_up` maps `-2.5` to `-2`. `-2.5` goes to `-3` under it, away from zero.",
        silent:   "`half_up` maps `-2.5` to `-2`. `-2.5` goes to `-3` under Java's rule, away from \
                  zero.",
        subject:  None,
        differs:  "whether the clause names another rule in words or points back at the mode",
        measured: false,
    },
    // A negator behind the reading that has the denotation behind it.
    Pair {
        fires:    "For `half_up`, a tie goes away from zero not toward positive infinity.",
        silent:   "For `half_up`, away from zero is not toward positive infinity.",
        subject:  None,
        differs:  "whether the reading is what a verb sends the tie to or the subject of a denied \
                  identity",
        measured: false,
    },
    // A denotation and a reading in different segments.
    Pair {
        fires:    "`half_up` is floor(x + q/2), as well as away from zero.",
        silent:   "`half_up` is floor(x + q/2), as opposed to away from zero.",
        subject:  None,
        differs:  "whether the words opening the reading's segment add it or set it against the \
                  denotation",
        measured: false,
    },
    // The lower-case `toward zero` behind a word that is not a lead.
    Pair {
        fires:    "`half_up` is floor(x + q/2). Adding sign(x) q/2 moves a tie toward zero and away \
                  from zero.",
        silent:   "`half_up` is floor(x + q/2). Adding sign(x) q/2 and truncating toward zero sends a \
                  tie away from zero.",
        subject:  None,
        differs:  "whether `toward zero` is where a tie is moved or the operation a gerund names",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is floor(x + q/2), and it moves a tie toward zero, then away from zero.",
        silent:   "`half_up` is floor(x + q/2), and rounding toward zero sends a tie away from zero.",
        subject:  None,
        differs:  "whether `toward zero` follows what moves or the gerund naming the operation",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is floor(x + q/2). Its tie goes toward zero after adding half, then away \
                  from zero.",
        silent:   "`half_up` is floor(x + q/2). Rounding toward zero after adding half sends a tie \
                  away from zero.",
        subject:  None,
        differs:  "whether the clause opens on the mode's tie or on the operation a gerund names",
        measured: false,
    },
    // The alias's spelling that is a reading, heading a segment that points
    // back at the mode.
    Pair {
        fires:    "`half_up` is nearest, and roundTiesToAway is what it does.",
        silent:   "`half_up` is nearest, and roundTiesToAway is what Java does.",
        subject:  None,
        differs:  "whether the segment the alias heads points back at the mode",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest, and ties-away is its rule.",
        silent:   "`half_up` is nearest, and ties-away is Java's rule.",
        subject:  None,
        differs:  "whether the rule the alias is said to be is the mode's or another's",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest, where ties-away is what it does.",
        silent:   "`half_up` is nearest, where ties-away is what Python does.",
        subject:  None,
        differs:  "the same behind `where`",
        measured: false,
    },
    // A relative after a negator that denies no identity.
    Pair {
        fires:    "`half_up` is not biased, which means a tie goes away from zero.",
        silent:   "`half_up` is not Java's rule, which means a tie goes away from zero.",
        subject:  None,
        differs:  "whether the negator denies a quality of the mode or that it is another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest and not biased, which means a tie goes away from zero.",
        silent:   "`half_up` is nearest and not the ties-to-even rule, which goes away from zero.",
        subject:  None,
        differs:  "whether the denied thing after `and not` is a quality or a rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` never sends a tie down, which is why it goes away from zero.",
        silent:   "`half_up` is never the rule of Java, which goes away from zero.",
        subject:  None,
        differs:  "whether the negator denies an act of the mode or that it is another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` does not truncate, which sends a tie away from zero.",
        silent:   "`half_up` is not the truncating rule, which sends a tie away from zero.",
        subject:  None,
        differs:  "whether the negator denies a verb of the mode or an identity",
        measured: false,
    },
    // A clause opening on a subject noun or a bare number.
    Pair {
        fires:    "`half_up` is the ruled mode. Every test shows it sends a tie away from zero.",
        silent:   "`half_up` is the ruled mode. Every test shows the crate sends a tie away from zero.",
        subject:  None,
        differs:  "whether the clause opening on a subject noun points back at the mode later",
        measured: false,
    },
    Pair {
        fires:    "`half_up` sends ties up. 5 of them go away from zero.",
        silent:   "`half_up` sends ties up. 270 read them as going away from zero.",
        subject:  None,
        differs:  "whether the bare number counts the ties or names a seat that acts",
        measured: false,
    },
    Pair {
        fires:    "`half_up` sends ties up. 5 of the ties go away from zero.",
        silent:   "`half_up` sends ties up. 270 read the ties as going away from zero.",
        subject:  None,
        differs:  "the same with no pronoun, the count told by the word behind the number",
        measured: false,
    },
    // A rule named by its owner: only an owner the ruling gives the alias to
    // names another rule. The ruled mode's own authorities do not.
    Pair {
        fires:    "`half_up` is the ruled mode. Under op's rule a tie goes away from zero.",
        silent:   "`half_up` is the ruled mode. Under Java's rule a tie goes away from zero.",
        subject:  None,
        differs:  "whether the owner of the rule is the ruled mode's own authority or one the \
                  ruling gives the alias to",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is the ruled mode. By the registry's rule a tie goes away from zero.",
        silent:   "`half_up` is the ruled mode. By Python's rule a tie goes away from zero.",
        subject:  None,
        differs:  "the same, the registry against Python",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is the ruled mode. Per the panel's rule, a tie goes away from zero.",
        silent:   "`half_up` is the ruled mode. Per IEEE's rule, a tie goes away from zero.",
        subject:  None,
        differs:  "the same, the panel against IEEE",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is the ruled mode. Under arvo's rounding a tie goes away from zero.",
        silent:   "`half_up` is the ruled mode. Under MATLAB's rounding a tie goes away from zero.",
        subject:  None,
        differs:  "the same, arvo against MATLAB, with `rounding` for the rule",
        measured: false,
    },
    // A relative after a negator binds to the denied thing only where the
    // words behind the negator name a rule: an article alone does not.
    Pair {
        fires:    "`half_up` is not the slow one, which is why it sends a tie away from zero.",
        silent:   "`half_up` is not the slow rule, which sends a tie away from zero.",
        subject:  None,
        differs:  "whether the article opens a description of the mode or a rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is not symmetric, which means a tie goes away from zero.",
        silent:   "`half_up` is not symmetric rounding, which sends a tie away from zero.",
        subject:  None,
        differs:  "whether the denied thing is a quality or a rule named with no article",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is not commercial, which means a tie goes away from zero.",
        silent:   "`half_up` is not commercial rounding, which sends a tie away from zero.",
        subject:  None,
        differs:  "the same with `commercial`",
        measured: false,
    },
    Pair {
        fires:    "`half_up` isn't symmetric, which means a tie goes away from zero.",
        silent:   "`half_up` isn't symmetric rounding, which sends a tie away from zero.",
        subject:  None,
        differs:  "the same behind `isn't`",
        measured: false,
    },
];

#[test]
fn every_neighbour_reads_right_on_both_sides() {
    let wrong = wrong_sides(NEIGHBOURS);
    assert!(
        wrong.is_empty(),
        "{} wrong:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

#[test]
fn the_neighbours_hold_twenty_eight() {
    // A count, so a neighbour dropped rather than read right is a failure here.
    assert_eq!(NEIGHBOURS.len(), 28);
}
