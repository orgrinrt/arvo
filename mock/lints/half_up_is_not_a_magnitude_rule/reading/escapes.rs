//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What lets a clause through once a name and a reading have been found in it.
//!
//! Each of these answers a question about the clause rather than about the
//! shape of the text: whether the clause denies the pairing, whether it is
//! about a rule this crate does not ship, whether the name and the reading are
//! two items of one list, whether the clause states the settled denotation and
//! contrasts the reading against it, and whether the reading is a term being
//! quoted. Which rule a reading is attributed to is a different question and is
//! answered where the pairing is made.

use super::cutting::{bounded, segment_of, words_of};
use super::vocabulary::{
    CONJUNCTIONS,
    CONTRASTS,
    COPULAS,
    DENOTES,
    DIRECTIONS,
    ELSEWHERE,
    ERRORS,
    MENTIONED,
    NEGATORS,
    PHRASE_OPENERS,
    READINGS,
    RESULT_MENTIONS,
    RESULTS,
    RULE_NOUNS,
    TIES,
};

/// Whether a negator is bound to the reading at `r`, or to the name at `n`.
///
/// Three ways it can be, and a conjunction ends its reach in each of them,
/// because a conjunction turns the sentence back to what it asserts: a clause
/// denying one reading and then asserting another is asserting the second.
///
/// - In the reading's own segment, on either side of it, since a clause naming
///   the alias carries its escape after the reading as often as before it.
/// - In the name's own segment, ahead of it.
/// - Between the two, where the reading comes first, since a negator opening a
///   list after the assertion takes every name in that list out of it, which is
///   how the design states the reflection partition.
///
/// A negator anywhere else is about something else in the sentence, which is
/// what a clause contrasting the mode with another mode is, and reading one of
/// those as a contrast against the mode is how the reading got into prose.
pub(super) fn negator_binds(
    lower: &str,
    segs: &[(usize, &str)],
    n: Option<usize>,
    r: usize,
) -> bool {
    if bound_in_segment(lower, segs, r, true) {
        return true;
    }
    match n {
        Some(n) => {
            bound_in_segment(lower, segs, n, false)
                || (r < n && reaches(lower, &lower[r .. n], r, n))
        },
        // A table cell is read whole: the row's name is in another cell, so
        // there is no segment of its own to bind to.
        None => reaches(lower, &lower[.. r], 0, r),
    }
}

/// Whether the term at `at` is the thing its clause denies: a negator stands
/// ahead of it in its own segment with no conjunction between.
///
/// What a pronoun after "ties away from zero is not `half_up`" points at is the
/// operation the clause was about, not the mode it said the operation is not,
/// so a denied spelling is not an antecedent.
pub(super) fn negated(lower: &str, segs: &[(usize, &str)], at: usize) -> bool {
    bound_in_segment(lower, segs, at, false)
}

/// Whether a negator in `at`'s own segment reaches it. With `after`, one
/// standing behind the term counts too, which is what the reading wants and the
/// name does not: a negator behind the name is the sentence denying the reading
/// of it, and that is the span rule's business rather than this one's.
///
/// A negator behind the term that has the settled denotation behind it in turn
/// is denying the denotation, not the term: "`half_up` sends a tie away from
/// zero rather than toward positive infinity" asserts the reading and denies the
/// denotation, so the negator is the denotation's and the reading stands.
/// Where a copula stands between the two, the reading is the subject of a
/// denied identity instead and is denied with it: "away from zero is not toward
/// positive infinity" says the alias is not the mode's denotation.
fn bound_in_segment(lower: &str, segs: &[(usize, &str)], at: usize, after: bool) -> bool {
    let (from, segment) = segs[segment_of(segs, at)];
    let end = from + segment.len();
    negators_in(segment)
        .into_iter()
        .map(|p| from + p)
        .filter(|p| after || *p < at)
        .any(|p| {
            let (lo, hi) = if p < at { (p, at) } else { (at, p) };
            let identity = || {
                COPULAS
                    .iter()
                    .any(|c| !bounded(&lower[lo .. hi], c).is_empty())
            };
            no_conjunction(&lower[lo .. hi])
                && !(p > at && denies_the_denotation(lower, p, end) && !identity())
        })
}

/// Whether the negator at `p` is bound to a statement of the settled
/// denotation standing behind it, opening before `end`, with no conjunction
/// between. The statement may run past `end`, since `floor(x + q/2)` holds a
/// bracket and a bracket cuts a segment.
fn denies_the_denotation(lower: &str, p: usize, end: usize) -> bool {
    DENOTES
        .iter()
        .flat_map(|d| bounded(&lower[p ..], d))
        .any(|d| p + d < end && no_conjunction(&lower[p .. p + d]))
}

/// Whether a negator stands in `lower[start .. to]` with no conjunction between
/// it and `to`, and a rule named behind it: the stretch denies that the thing
/// before it is the thing it names.
///
/// What a relative opening the next segment binds to is the last thing named,
/// and where the stretch denies that thing is the mode, the thing is another
/// rule whatever words it is named in: "`half_up` is not Java's rule, which
/// goes away from zero" gives the reading to Java's rule.
///
/// A negator denying a quality or an act names no thing, so the relative binds
/// to the mode: "`half_up` is not biased, which means a tie goes away from
/// zero", "`half_up` does not truncate, which sends a tie away from zero", and
/// so does a description, "is not the slow one, which". What names a rule is
/// read by `names_a_rule_behind`, past a `from`, `to` or `than` the negator takes.
pub(super) fn denied_since(text: &str, lower: &str, start: usize, to: usize) -> bool {
    negators_in(&lower[start .. to])
        .into_iter()
        .map(|p| start + p)
        .any(|p| no_conjunction(&lower[p .. to]) && names_a_rule_behind(text, lower, p, to))
}

/// Whether what stands behind the negator at `p`, up to `to`, names a rule: it
/// opens on a code span or a capital, a name; it holds one of `RULE_NOUNS` or
/// `rounding` before the first of `PHRASE_OPENERS`, a rule described in words;
/// or it holds one behind that preposition in a phrase that names a rule, with
/// `the`, an owner's `'s` or a capitalised word.
///
/// An article alone is not enough: "is not the slow one, which" denies a
/// description of the mode and names nothing the relative could mean, where
/// "is not the truncating rule, which" and "is not symmetric rounding, which"
/// name a rule. A rule noun behind a preposition with nothing naming it sits in
/// a phrase about the denial, so "is not slow as a rule, which", "is not slow in
/// any mode, which" and "does not drift under this rule, which" deny a quality
/// or an act and the relative binds to the mode, where "is not slow like the
/// truncating rule, which" and "is not as slow as Java's rule, which" name the
/// rule the relative binds to. An aside in commas right behind the negator is
/// passed over, "is not, as a rule, slow, which" against "is not, as a rule,
/// the truncating rule, which".
fn names_a_rule_behind(text: &str, lower: &str, p: usize, to: usize) -> bool {
    // The word starting at or after `at`, past whitespace, and where it starts.
    let word = |at: usize| -> (usize, &str) {
        let past = &lower[at .. to];
        let start = at + (past.len() - past.trim_start().len());
        let len = lower[start .. to]
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '\'' || c == '-' || c == '_'))
            .unwrap_or(to - start);
        (start, &lower[start .. start + len])
    };
    // Past the whole negator, which may be several words or the `n't` of one.
    let negator = NEGATORS
        .iter()
        .filter(|n| lower[p .. to].starts_with(*n))
        .map(|n| n.len())
        .max()
        .unwrap_or("n't".len());
    let (mut at, mut next) = word(p + negator);
    // An aside set off by commas right behind the negator is passed over.
    if lower[at .. to].starts_with(',') {
        if let Some(close) = lower[at + 1 .. to].find(',') {
            (at, next) = word(at + 1 + close + 1);
        }
    }
    if matches!(next, "from" | "to" | "than") {
        (at, next) = word(at + next.len());
    }
    let first = text[at ..].chars().next();
    let end = PHRASE_OPENERS
        .iter()
        .filter_map(|w| bounded(&lower[at .. to], w).first().copied())
        .min()
        .map_or(to, |q| at + q);
    let ruled = |span: &str| {
        RULE_NOUNS
            .iter()
            .chain(&["rounding"])
            .any(|n| !bounded(span, n).is_empty())
    };
    // Behind a preposition a rule noun counts only in a rule named as one: by
    // `the`, by an owner's `'s`, or by a word opening on a capital or a code span.
    let phrase = &lower[end .. to];
    let named = !bounded(phrase, "the").is_empty()
        || phrase.contains("'s ")
        || text[end .. to]
            .split_whitespace()
            .skip(1)
            .any(|w| w.starts_with(|c: char| c.is_ascii_uppercase() || c == '`'));
    next.is_empty() && first == Some('`')
        || first.is_some_and(|c| c.is_ascii_uppercase())
        || ruled(&lower[at .. end])
        || named && ruled(phrase)
}

/// Whether a negator inside `span` reaches `to`, where `span` starts at `from`.
fn reaches(lower: &str, span: &str, from: usize, to: usize) -> bool {
    negators_in(span)
        .into_iter()
        .map(|p| from + p)
        .any(|p| no_conjunction(&lower[p .. to]))
}

/// Whether nothing in `span` turns the sentence back to what it asserts.
fn no_conjunction(span: &str) -> bool {
    !CONJUNCTIONS.iter().any(|c| !bounded(span, c).is_empty())
}

/// Where every negator in `text` starts, counting a word ending in `n't`.
fn negators_in(text: &str) -> Vec<usize> {
    let mut out: Vec<usize> = NEGATORS.iter().flat_map(|n| bounded(text, n)).collect();
    out.extend(text.match_indices("n't").map(|(at, _)| at).filter(|at| {
        text[.. *at]
            .chars()
            .next_back()
            .is_some_and(char::is_alphanumeric)
    }));
    out.sort_unstable();
    out
}

/// Whether the clause states the settled denotation ahead of the reading at
/// `r`, with no negator bound to that statement and nothing in the reading's own
/// segment conjoining the reading to it.
///
/// A clause saying the mode sends a tie toward positive infinity is not also
/// giving it the other reading, so a reading standing after that statement is a
/// contrast: "a tie goes toward positive infinity at every sign, so `-2.5` goes
/// to `-2` and `-0.5` to `0`, where ties away from zero would give `-3` and
/// `-1`". The statement has to come first, because the order is what separates
/// a contrast from a clause that denies the denotation and then asserts the
/// reading.
///
/// Order alone is not enough: "`half_up` goes toward positive infinity and away
/// from zero" conjoins the two readings rather than contrasting them. So the
/// conjunction rule the negators take applies here too, and over the same
/// stretch a negator's does: the reading's own segment, from the statement where
/// the two share one, from the segment's start where they do not. A conjunction
/// in an earlier segment joins two other things, as the "so" and the "and" in
/// the example do, and says nothing about whether the reading is asserted.
///
/// Where the statement stands in an earlier segment, the reading's own segment
/// also has to open the contrast, with a word from `CONTRASTS` ahead of the
/// reading. A bare comma between the two is an apposition, which restates the
/// first as the second: "`half_up` goes toward positive infinity, away from
/// zero" gives the mode both.
pub(super) fn denotation_before(lower: &str, segs: &[(usize, &str)], r: usize) -> bool {
    let from = segs[segment_of(segs, r)].0;
    let contrasted = CONTRASTS
        .iter()
        .any(|c| !bounded(&lower[from .. r], c).is_empty());
    DENOTES
        .iter()
        .flat_map(|d| bounded(lower, d))
        .filter(|d| *d < r)
        .any(|d| {
            !bound_in_segment(lower, segs, d, false)
                && no_conjunction(&lower[d.max(from) .. r])
                && (d >= from || contrasted)
        })
}

/// Whether the name and the reading sit in different items of one list.
///
/// Three segments or more, since one comma between two things is a sentence.
/// The reading has to stand alone in its own item, and a neighbouring item has
/// to be a bare name, which is what makes the run a list rather than a sentence
/// with an aside in it. The last item of a list runs into the predicate, so the
/// name's item is not required to be bare: "floor, ceiling, away-from-zero,
/// nearest-half-up and nearest-half-even give a zero difference" is a list.
///
/// A neighbour is the nearest item on each side that holds any text. Two
/// delimiters side by side, a closing bracket and then a comma, cut an empty
/// segment between them, and an empty segment is not an item: counted as one,
/// it reads as a bare name, and "`half_up` goes floor(x + q/2), but away from
/// zero" became a list.
///
/// A parenthetical is not an item either. A segment that is the whole of one
/// pair of round brackets is an aside on what stands before it, so the
/// neighbour is looked for past it: "`half_up` rounds x (a tie) away from zero"
/// has one item, the sentence, with an aside in it.
pub(super) fn listed_apart(lower: &str, segs: &[(usize, &str)], n: usize, r: usize) -> bool {
    if segs.len() < 3 {
        return false;
    }
    let (i, j) = (segment_of(segs, n), segment_of(segs, r));
    if i == j || !is_bare(segs[j].1, lower, r) {
        return false;
    }
    let bytes = lower.as_bytes();
    let bracketed = |k: usize| {
        let (from, text) = segs[k];
        from > 0 && bytes[from - 1] == b'(' && bytes.get(from + text.len()) == Some(&b')')
    };
    let item = |k: &usize| !segs[*k].1.trim().is_empty() && !bracketed(*k);
    [(0 .. j).rev().find(item), (j + 1 .. segs.len()).find(item)]
        .into_iter()
        .flatten()
        .any(|k| words_of(segs[k].1) <= 1)
}

/// Whether a segment of the lowered clause holds nothing besides the reading
/// starting at `at`.
fn is_bare(segment: &str, lower: &str, at: usize) -> bool {
    words_of(&segment.replacen(reading_at(lower, at), " ", 1)) == 0
}

/// The reading starting at `at`, taken longest first because one reading can
/// open another. A reading is the only thing ever looked up here: a list is
/// refused on the item holding the reading, and the item holding the name runs
/// into the predicate, so it is never asked to be bare.
fn reading_at(lower: &str, at: usize) -> &'static str {
    READINGS
        .iter()
        .filter(|r| lower[at ..].starts_with(**r))
        .max_by_key(|r| r.len())
        .copied()
        .unwrap_or_default()
}

/// Whether the reading at `at` is a term being quoted rather than a reading
/// being given: inside a code span, with a word beside the span that says the
/// span is a piece of a corpus. A code span on its own is not enough, since a
/// clause saying the mode does `roundTiesToAway` is asserting exactly the thing
/// this reads for.
///
/// What makes the span a quotation is the word beside it naming a row, a cell,
/// an entry or a spelling, which is a claim about where the text sits rather
/// than about where a tie goes. `reading` is not such a word and is not in the
/// list: a clause saying the mode has the reading `away from zero` gives it
/// that reading, in the one word that was supposed to prove it had not.
pub(super) fn mentioned(lower: &str, at: usize, term: &str) -> bool {
    let end = at + term.len();
    if !(lower[.. at].ends_with('`') && lower[end ..].starts_with('`')) {
        return false;
    }
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let after: String = lower[end + 1 ..]
        .trim_start_matches(|c: char| !ident(c))
        .chars()
        .take_while(|c| ident(*c))
        .collect();
    let head = lower[.. at - 1].trim_end_matches(|c: char| !ident(c));
    let before: String = head
        .chars()
        .rev()
        .take_while(|c| ident(*c))
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    // `value` names what the mode gives as well as a piece of a corpus, so
    // ahead of a verb of giving it is the first: "`half_up` returns the `away
    // from zero` value" asserts the reading.
    let gives = || {
        RESULTS
            .iter()
            .any(|v| !bounded(&lower[.. at], v).is_empty())
    };
    let quotes = |w: &str| MENTIONED.contains(&w) && !(RESULT_MENTIONS.contains(&w) && gives());
    quotes(&after) || quotes(&before)
}

/// Whether a direction word stands in front of `at`, in the same segment, and
/// the size it gives a direction to is a tie's.
///
/// A direction word ahead of `in magnitude` says where something goes, and what
/// goes is the nearest noun ahead of it that a size can be about. Where that is
/// an error rather than a tie, the two words are the error's: "an error that
/// increases in magnitude with the width" says nothing of where a tie goes.
pub(super) fn directed(lower: &str, segs: &[(usize, &str)], at: usize) -> bool {
    let from = segs[segment_of(segs, at)].0;
    let head = &lower[from .. at];
    let last = |words: &[&str]| words.iter().flat_map(|w| bounded(head, w)).max();
    let errors = last(ERRORS);
    DIRECTIONS.iter().any(|d| !bounded(head, d).is_empty())
        && (errors.is_none() || last(TIES) > errors)
}

/// Whether the clause says it is about some other rule than the shipped one.
pub(super) fn elsewhere(lower: &str) -> bool {
    ELSEWHERE.iter().any(|m| !bounded(lower, m).is_empty())
}
