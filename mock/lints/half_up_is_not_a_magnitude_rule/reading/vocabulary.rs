//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every word list the reader knows, and nothing that decides anything.
//!
//! The reader has no grammar. What it has instead is these lists, and each one
//! answers a question about what a stretch of text names: which rule, which
//! reading, which rule other than this one, which word turns a pairing into a
//! contrast, which word points back at something already named. A decision the
//! reader makes is a decision about one of those answers, so the lists are kept
//! apart from the deciding, where a reader can see the whole vocabulary at once
//! and check it against the prose this repository actually writes.

/// Every spelling of the mode's name this repository writes. Matched with case,
/// and bounded by a character that cannot sit inside an identifier, so a
/// longer identifier carrying one of them is not a mention of the mode. Java's
/// and Python's upper-case constant is not among them: it names the other
/// operation, and saying so is true.
pub(crate) const SPELLINGS: &[&str] =
    &["half_up", "HalfUp", "half-up", "half up", "Half-up", "Half up", "Half_up"];

/// The readings that put a tie by its magnitude or its sign. Matched without
/// case, with the same bounds.
///
/// `in magnitude` is here as a reading of the tie, and a sentence bounding an
/// error carries the same two words about something else entirely. It counts
/// only behind one of the direction words below, in its own segment.
pub(crate) const READINGS: &[&str] = &[
    "away from zero",
    "away_from_zero",
    "away-from-zero",
    "ties to away",
    "tiestoaway",
    "roundtiestoaway",
    "ties-away",
    "ties_away",
    "greater absolute value",
    "larger absolute value",
    "greater magnitude",
    "larger magnitude",
    "in magnitude",
    "reads the sign",
    "sign of the slot",
    "commutes with reflection",
    "reflection equivariant",
    "reflection-equivariant",
];

/// The reading that only counts behind a direction word.
pub(crate) const QUALIFIED: &str = "in magnitude";

/// What a passage says when it states the settled denotation rather than
/// mentioning the mode, in the words the ruling states it in.
///
/// Read here and by `half-up-carries-its-note`, which is where it was written
/// first. It is vocabulary about the name, so it sits beside the spellings and
/// the readings rather than inside one of the two lints that ask about it.
pub(crate) const DENOTES: &[&str] = &[
    "toward positive infinity",
    "towards positive infinity",
    "to positive infinity",
    "floor(x + q/2)",
    "floor(x+q/2)",
];

/// What has to stand in front of `in magnitude`, in its segment, for those two
/// words to be about where a tie goes rather than about how big an error is.
pub(crate) const DIRECTIONS: &[&str] = &[
    "up",
    "upward",
    "upwards",
    "down",
    "away",
    "outward",
    "outwards",
    "larger",
    "greater",
    "bigger",
    "higher",
    "toward",
    "towards",
    "grows",
    "growing",
    "increases",
    "increasing",
    "rises",
];

/// Words that turn a pairing into a contrast where they are bound to one of the
/// two. Matched without case and bounded as a word, and any word ending in
/// `n't` counts too.
pub(crate) const NEGATORS: &[&str] = &[
    "not",
    "never",
    "no",
    "nor",
    "neither",
    "unlike",
    "differs",
    "differ",
    "different",
    "alias",
    "rather than",
    "instead",
    "other than",
    "against",
    "versus",
    "vs",
    "except",
];

/// Words that end a negator's reach inside one segment, so the clause turns
/// back to what it asserts. "not the even rule but sends a tie away from zero"
/// says the thing the reading names.
///
/// The same list bounds the denotation's reach, for the same reason: a clause
/// that states the denotation and then conjoins a second predicate is asserting
/// both of them, so the statement excuses a reading standing behind it only
/// where nothing between the two turns the clause back to what it asserts.
pub(crate) const CONJUNCTIONS: &[&str] = &[
    "but", "and", "yet", "though", "although", "while", "whereas", "or", "because", "since", "so",
];

/// Markers that put a whole clause about some other rule: a planted one, a
/// wrong one, the one that stood before, or no tie at all. Matched without case
/// and bounded as a word.
pub(crate) const ELSEWHERE: &[&str] = &[
    "planted",
    "wrong",
    "broken",
    "defect",
    "mutant",
    "mutation",
    "formerly",
    "previously",
    "used to",
    "old rule",
    "before the ruling",
    "no tie",
    "without a tie",
    "away from a tie",
    "reaches no tie",
];

/// Words that open a clause, or a segment of one, by pointing back at whatever
/// stood before it, rather than by naming anything.
pub(crate) const PRONOUNS: &[&str] =
    &["it", "its", "it's", "this", "that", "these", "those", "they", "their"];

/// Words that bind a segment to the thing the segment before it named, standing
/// anywhere in the segment rather than only at its head.
///
/// "`half_up` is not Java's `HALF_UP`, which goes away from zero" attributes the
/// reading to the other operation, and so does "..., the rule that goes away
/// from zero", where the relative stands behind an apposition instead of
/// opening the segment. Both are the anaphora the reader already reads across
/// clauses, one cut finer.
pub(crate) const RELATIVES: &[&str] = &["which", "whose", "that", "who", "whom"];

/// Words that may stand in an item of a list beside the term itself, so an item
/// carrying one of them is still a bare item.
pub(crate) const FILLER: &[&str] =
    &["a", "an", "the", "and", "or", "nor", "but", "then", "also", "of"];

/// Words that make a term beside them a name being quoted rather than a reading
/// being given. A sentence counting the rows whose entries read `away from
/// zero` is about a corpus of text, and the registry's residue rows are written
/// that way.
///
/// Every one of them names a piece of a corpus: a row, a cell, an entry, a
/// spelling, a string. `reading` and `readings` are deliberately absent, and
/// that absence is load-bearing. They name what the sentence gives rather than
/// what the sentence is about, so "`half_up` has the reading `away from zero`"
/// asserts the reading of the mode with the word that was supposed to prove it
/// was only being quoted. Nothing on the published surface or in the registry
/// uses either word to quote a term, which is what makes taking them out safe
/// as well as right.
pub(crate) const MENTIONED: &[&str] = &[
    "entry",
    "entries",
    "spelling",
    "spellings",
    "name",
    "names",
    "named",
    "naming",
    "word",
    "words",
    "value",
    "values",
    "label",
    "labels",
    "row",
    "rows",
    "cell",
    "cells",
    "column",
    "columns",
    "string",
    "mention",
    "mentions",
];

/// Every other name a pronoun standing for a rounding rule can point at.
///
/// The five names of the six-name vocabulary that are not this mode, the
/// external constants this repository contrasts the mode with, and MATLAB's
/// method names for the rules that are not this one. Matched with case and
/// bounded as a word, exactly as `SPELLINGS` is, which is what keeps the
/// lower-case English words apart from the identifiers.
///
/// MATLAB's `Nearest` is deliberately absent: it is this mode rather than
/// another one, so a pronoun after it points here.
///
/// Six of these are an ordinary English word under their capital: `Round`,
/// `Fix`, `Floor`, `Ceiling`, `Convergent` and `Stochastic`. The collision is
/// real in both directions and is not repairable from here. This repository's
/// own published prose writes each of them bare and capitalised, at the head of
/// a sentence, meaning the method, and `crates/arvo-format/DESIGN.md.tmpl` does
/// it in consecutive sentences, so a reader that stopped taking the bare form
/// as a name would refuse that page. A reader that keeps taking it stays silent
/// on an imperative that opens the same way. The second is the safe direction
/// in a gate that refuses, and the sentences it is wrong about are in the
/// corpus as catalogue pairs rather than only in this paragraph.
pub(crate) const OTHER_NAMES: &[&str] = &[
    "toward_zero",
    "TowardZero",
    "toward zero",
    "floor",
    "Floor",
    "ceil",
    "Ceil",
    "Ceiling",
    "half_even",
    "HalfEven",
    "half-even",
    "half even",
    "Half-even",
    "Half even",
    "Half_even",
    "stochastic",
    "Stochastic",
    "HALF_UP",
    "ROUND_HALF_UP",
    "roundTiesToAway",
    "roundTiesToEven",
    "RoundingMode",
    "Round",
    "Fix",
    "Convergent",
    // The alias, in the spellings this repository writes it in. A clause naming
    // it is about it: "`half_up` is `floor(x + q/2)`; the ties-away alias
    // commutes with reflection and reads the sign at a tie" states both rules
    // and gives the second reading to the second one.
    "TiesAwayFromZero",
    "ties_away",
    "ties-away",
    "alias",
];
