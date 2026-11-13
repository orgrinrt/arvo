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
    "as opposed to",
    "in contrast to",
    "contrary to",
];

/// The copula, whose negation denies an identity rather than an act.
///
/// "away from zero is not toward positive infinity" denies that one reading is
/// the other, where "sends a tie away from zero rather than toward positive
/// infinity" asserts one and denies the other. And "is not Java's rule" denies
/// that the mode is a thing, where "does not truncate" denies an act of it.
pub(crate) const COPULAS: &[&str] =
    &["is", "are", "was", "were", "be", "isn't", "aren't", "wasn't"];

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

/// The personal pronouns of `PRONOUNS`: each points back at a thing named
/// before, where `this` and `that` can point at a clause or open a relative.
pub(crate) const PERSONAL: &[&str] = &["it", "its", "it's", "they", "their", "them"];

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
/// `Fix`, `Floor`, `Ceiling`, `Convergent` and `Stochastic`. This repository's
/// own published prose writes each of them bare and capitalised, at the head of
/// a sentence, meaning the method, and `crates/arvo-format/DESIGN.md.tmpl` does
/// it in consecutive sentences, followed by the method's verb. An imperative
/// opening the same way is followed by its object, so the word right behind
/// tells the two apart where that word is of a closed class, as
/// `IMPERATIVE_NEXT` lists. Where the object opens on an adjective or a noun
/// the name reading stands, which is the safe direction in a gate that refuses,
/// and the sentence it costs is a catalogue pair rather than only this
/// paragraph.
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

/// Words that, standing right behind a capitalised method name opening a
/// clause, make that word an imperative or an ordinary word rather than the
/// method.
///
/// A method opening a sentence is its subject and is followed by a verb:
/// "Round does", "Floor is", "Ceiling sends". An imperative is followed by its
/// object, and an ordinary word by what it heads: "Round every tie", "Fix the
/// doc", "Ceiling aside". Every word here is of a closed class, a determiner, a
/// pronoun, or the two words that set a term aside, so the list does not have
/// to guess at verbs.
pub(crate) const IMPERATIVE_NEXT: &[&str] = &[
    "the", "a", "an", "every", "each", "all", "any", "it", "them", "this", "that", "these",
    "those", "its", "their", "half", "first", "aside", "apart",
];

/// Prepositions that, behind a capitalised method name, make it an imperative
/// only where one of `IMPERATIVE_NEXT` follows them.
///
/// "Round to it" is an imperative taking its object behind a preposition, and
/// "Round to nearest" is MATLAB's method for this mode's alias, named by what it
/// rounds to. The word behind the preposition tells the two apart: a determiner
/// or a pronoun opens an object, anything else names the method.
pub(crate) const IMPERATIVE_PREPOSITIONS: &[&str] = &["to", "toward", "towards", "up", "down"];

/// Words that, standing right ahead of the lower-case `toward zero`, make it
/// the name of the rule rather than a direction.
///
/// A rule's name is a noun phrase, so it stands where one does: after a
/// conjunction or a preposition that takes a noun, after an article, after a
/// copula or a negator. A direction stands after a verb or after what moves:
/// "sends a tie toward zero", "rounds toward zero". A phrase opening the text,
/// or standing behind a bracket, a comma or a backtick, is read as the name.
pub(crate) const NAME_LEADS: &[&str] = &[
    "and", "or", "nor", "for", "with", "under", "like", "unlike", "than", "of", "from", "the",
    "is", "are", "was", "be", "not", "never", "isn't", "versus", "vs", "between", "both", "either",
    "neither", "also",
];

/// Gerunds of motion, behind which the lower-case `toward zero` is where a
/// thing goes rather than the operation the gerund names.
pub(crate) const MOTIONS: &[&str] = &[
    "going", "moving", "heading", "tending", "pointing", "leaning", "sending", "falling",
];

/// Nouns that, opening a clause, name a subject of its own other than a
/// rounding rule: a crate, a seat of a panel, a review, a test.
///
/// A clause opening on one of these is about it, as a clause opening on a code
/// span is: "The crate sends a tie away from zero" records what a crate did and
/// "Seat 270 read blind" what a seat did, and neither is a claim about what the
/// mode denotes.
pub(crate) const SUBJECTS: &[&str] = &[
    "crate",
    "crates",
    "seat",
    "seats",
    "panel",
    "review",
    "reviewer",
    "oracle",
    "probe",
    "test",
    "tests",
    "implementation",
    "library",
    "standard",
];

/// Words that, right behind a bare whole number opening a clause, make it a
/// count or a quantity rather than the number a seat is named by: "5 of the
/// ties", "3 out of 4", "2 ties", "10 percent".
pub(crate) const COUNTED: &[&str] = &[
    "of", "out", "in", "per", "percent", "times", "or", "more", "fewer", "less", "tie", "ties",
    "value", "values", "slot", "slots", "step", "steps", "bit", "bits",
];

/// The owners the governing ruling gives the alias to, lowered: Java, Python,
/// IEEE 754 and MATLAB. An owner's rule named in words is another rule only
/// where the owner is one of these.
pub(crate) const ALIAS_OWNERS: &[&str] = &["java", "python", "ieee", "754", "matlab"];

/// Articles a subject noun may stand behind at the head of a clause.
pub(crate) const ARTICLES: &[&str] = &["the", "a", "an", "each", "every", "one"];

/// Nouns that, standing right behind a name that is also a reading, say the
/// name is being used as a name: "the ties-away rule".
pub(crate) const RULE_NOUNS: &[&str] = &["rule", "rules", "alias", "mode", "operation", "method"];

/// Words that open a contrast in the reading's own segment, so a reading
/// standing behind the settled denotation is what the mode is set against
/// rather than a second thing it does.
///
/// "toward positive infinity, where ties away from zero would give -3" sets the
/// two apart; "toward positive infinity, away from zero" restates the first as
/// the second, which is the defect in an apposition.
pub(crate) const CONTRASTS: &[&str] = &["where", "wherever"];

/// Nouns whose size `in magnitude` can be about, other than a tie's.
pub(crate) const ERRORS: &[&str] =
    &["error", "errors", "bias", "drift", "difference", "residual", "deviation"];

/// Nouns naming what a tie rule moves, against which `ERRORS` is weighed.
pub(crate) const TIES: &[&str] = &["tie", "ties"];

/// Verbs that, ahead of a code span, make `value` the thing the mode gives
/// rather than a word saying the span is quoted.
///
/// "`half_up` returns the `away from zero` value at a tie" says what the mode
/// gives; "`half_up` has the `away from zero` value in it" says what a text
/// holds. Only `value` and `values` among the mention words name a result as
/// well as a piece of a corpus, so only they consult this.
pub(crate) const RESULTS: &[&str] = &[
    "returns", "return", "returned", "gives", "give", "gave", "yields", "yield", "produces",
    "produce", "picks", "pick", "chooses", "choose", "takes", "take", "sends", "send", "rounds",
    "round", "lands", "goes",
];

/// The mention words that also name a result.
pub(crate) const RESULT_MENTIONS: &[&str] = &["value", "values"];
