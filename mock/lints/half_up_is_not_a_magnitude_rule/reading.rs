//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Reading one passage of prose for a magnitude reading of the mode's name.
//!
//! A passage is cut into clauses, a clause into segments, and the two cuts do
//! different work. The clause is where a name and a reading are paired at all.
//! The segment is what an escape is bound to: a negator voids the pairing where
//! it sits with the reading or with the name, and says nothing where it sits in
//! another segment of the same sentence, denying something else there. A reader
//! taking any negator before the later of the two lets a sentence contrasting
//! the mode with another mode through, and a reader taking any two commas for a
//! list lets a parenthetical through. Both were measured doing exactly that, on
//! eight sentences. Those sentences are in `sentences.rs` rather than here, each
//! beside the near-twin on the other side of the line, which is also where to
//! read what each rule below is for.
//!
//! Table rows are read apart from the clauses, one row at a time, because a row
//! pairs its first cell with the others and a clause cut at every `|` would
//! never see that pairing.

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
pub(super) const READINGS: &[&str] = &[
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
pub(super) const QUALIFIED: &str = "in magnitude";

/// What has to stand in front of `in magnitude`, in its segment, for those two
/// words to be about where a tie goes rather than about how big an error is.
pub(super) const DIRECTIONS: &[&str] = &[
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
pub(super) const NEGATORS: &[&str] = &[
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
pub(super) const CONJUNCTIONS: &[&str] = &[
    "but", "and", "yet", "though", "although", "while", "whereas", "or", "because", "since", "so",
];

/// Markers that put a whole clause about some other rule: a planted one, a
/// wrong one, the one that stood before, or no tie at all. Matched without case
/// and bounded as a word.
pub(super) const ELSEWHERE: &[&str] = &[
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

/// Words that open a clause by pointing back at whatever the clause before it
/// left named, rather than by naming anything.
pub(super) const PRONOUNS: &[&str] =
    &["it", "its", "it's", "this", "that", "these", "those", "they", "their"];

/// Words that may stand in an item of a list beside the term itself, so an item
/// carrying one of them is still a bare item.
const FILLER: &[&str] = &["a", "an", "the", "and", "or", "nor", "but", "then", "also", "of"];

/// Words that make a term beside them a name being quoted rather than a reading
/// being given. A sentence counting the rows whose entries read `away from
/// zero` is about a corpus of text, and the registry's residue rows are written
/// that way.
const MENTIONED: &[&str] = &[
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
    "reading",
    "readings",
];

/// One pairing the lint refuses: where in the passage, which spelling, which
/// reading.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Hit {
    pub(super) at:      usize,
    pub(super) name:    String,
    pub(super) reading: &'static str,
}

/// Every refused pairing in `text`. With `subject`, a clause is read as already
/// naming the mode, which is how a doc block reads when the item it documents is
/// the mode.
pub(super) fn hits(text: &str, subject: Option<&str>) -> Vec<Hit> {
    let mut out = Vec::new();
    // The prose with every table row blanked to spaces, so offsets still name
    // the passage and a row ends whatever clause was open, as a blank line does.
    let mut prose = String::with_capacity(text.len());
    let mut start = 0;
    for line in text.split_inclusive('\n') {
        if line.trim_start().starts_with('|') {
            out.extend(row(line, start, subject));
            let body = line.trim_end_matches('\n');
            prose.push_str(&" ".repeat(body.len()));
            prose.push_str(&line[body.len() ..]);
        } else {
            prose.push_str(line);
        }
        start += line.len();
    }
    // What a clause opening with a pronoun is about: the last thing the clause
    // before it named. That is the mode only where the mode is what it named
    // last, so a sentence contrasting the mode with something else leaves the
    // something else, and a pronoun after it points there.
    let mut carried: Option<&'static str> = None;
    for (from, clause) in clauses(&prose) {
        if let Some(hit) = clause_hit(clause, subject, carried) {
            out.push(Hit {
                at: from + hit.at,
                ..hit
            });
        }
        carried = if clause.ends_with('\n') { None } else { antecedent(clause) };
    }
    out.sort_by_key(|h| h.at);
    out
}

/// A table row: the first cell is what the row is about, and a reading in any
/// other cell is paired with it unless that cell contrasts it.
fn row(line: &str, start: usize, subject: Option<&str>) -> Option<Hit> {
    let mut cells = Vec::new();
    let mut from = 0;
    for (i, c) in line.char_indices() {
        if c == '|' {
            cells.push((from, &line[from .. i]));
            from = i + 1;
        }
    }
    cells.push((from, &line[from ..]));
    let mut named = cells.iter().filter(|(_, c)| !c.trim().is_empty());
    let (_, first) = named.next()?;
    let name = match spellings_in(first).first() {
        Some((_, name)) => (*name).to_string(),
        None => subject?.to_string(),
    };
    for (at, cell) in named {
        let lower = cell.to_ascii_lowercase();
        if elsewhere(&lower) {
            continue;
        }
        let segs = segments(&lower);
        if let Some(&(r, reading)) = readings_in(&lower, &segs).first() {
            if !negator_binds(&lower, &segs, None, r) {
                return Some(Hit {
                    at: start + at + r,
                    name,
                    reading,
                });
            }
        }
    }
    None
}

/// The first refused pairing in one clause.
fn clause_hit(clause: &str, subject: Option<&str>, carried: Option<&'static str>) -> Option<Hit> {
    let lower = clause.to_ascii_lowercase();
    let segs = segments(&lower);
    let readings = readings_in(&lower, &segs);
    if readings.is_empty() || elsewhere(&lower) {
        return None;
    }
    let own: Vec<(usize, &str)> = spellings_in(clause);
    let names: Vec<(usize, &str)> = if !own.is_empty() {
        own
    } else if opens_with_a_pronoun(&lower) {
        // The clause is about whatever the one before it left named, and the
        // item a doc block sits on does not reach past a pronoun.
        carried.map(|n| vec![(0, n)]).unwrap_or_default()
    } else {
        subject.map(|s| vec![(0, s)]).unwrap_or_default()
    };
    for &(n, name) in &names {
        for &(r, reading) in &readings {
            if listed_apart(&lower, &segs, n, r) || negator_binds(&lower, &segs, Some(n), r) {
                continue;
            }
            return Some(Hit {
                at: r,
                name: name.to_string(),
                reading,
            });
        }
    }
    None
}

/// The name a pronoun opening the next clause points at: the last spelling of
/// the mode in this clause, where nothing else is named after it.
fn antecedent(clause: &str) -> Option<&'static str> {
    let (at, name) = *spellings_in(clause).last()?;
    // A backticked span opening after the spelling is something else being
    // named, so a pronoun after this clause points at that instead. Two ticks
    // are one span.
    let named_after = clause
        .match_indices('`')
        .filter(|(i, _)| *i > at + name.len())
        .count();
    (named_after < 2).then_some(name)
}

/// Whether the clause opens by pointing back rather than by naming.
fn opens_with_a_pronoun(lower: &str) -> bool {
    let first = lower
        .trim_start_matches(|c: char| !c.is_alphanumeric())
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .next()
        .unwrap_or_default();
    PRONOUNS.contains(&first)
}

/// Whether the name and the reading sit in different items of one list.
///
/// Three segments or more, since one comma between two things is a sentence.
/// The reading has to stand alone in its own item, and a neighbouring item has
/// to be a bare name, which is what makes the run a list rather than a sentence
/// with an aside in it. The last item of a list runs into the predicate, so the
/// name's item is not required to be bare: "floor, ceiling, away-from-zero,
/// nearest-half-up and nearest-half-even give a zero difference" is a list.
fn listed_apart(lower: &str, segs: &[(usize, &str)], n: usize, r: usize) -> bool {
    if segs.len() < 3 {
        return false;
    }
    let (i, j) = (segment_of(segs, n), segment_of(segs, r));
    if i == j || !is_bare(segs[j].1, lower, r) {
        return false;
    }
    [j.checked_sub(1), (j + 1 < segs.len()).then_some(j + 1)]
        .into_iter()
        .flatten()
        .any(|k| words_of(segs[k].1) <= 1)
}

/// Whether a segment holds nothing besides the term starting at `at`.
fn is_bare(segment: &str, lower: &str, at: usize) -> bool {
    let term = term_at(lower, at);
    let rest = segment
        .to_ascii_lowercase()
        .replacen(&term.to_ascii_lowercase(), " ", 1);
    words_of(&rest) == 0
}

/// How many words a segment holds that are not the filler a list item allows.
fn words_of(text: &str) -> usize {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '\''))
        .filter(|w| !w.is_empty() && !FILLER.contains(&w.to_ascii_lowercase().as_str()))
        .count()
}

/// The term starting at `at`, whichever of the two it is, as the table holds it.
fn term_at(lower: &str, at: usize) -> &'static str {
    let rest = &lower[at ..];
    SPELLINGS
        .iter()
        .chain(READINGS.iter())
        .filter(|t| rest.starts_with(t.to_ascii_lowercase().as_str()))
        .max_by_key(|t| t.len())
        .copied()
        .unwrap_or_default()
}

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
fn negator_binds(lower: &str, segs: &[(usize, &str)], n: Option<usize>, r: usize) -> bool {
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

/// Whether a negator in `at`'s own segment reaches it. With `after`, one
/// standing behind the term counts too, which is what the reading wants and the
/// name does not: a negator behind the name is the sentence denying the reading
/// of it, and that is the span rule's business rather than this one's.
fn bound_in_segment(lower: &str, segs: &[(usize, &str)], at: usize, after: bool) -> bool {
    let (from, segment) = segs[segment_of(segs, at)];
    negators_in(segment)
        .into_iter()
        .map(|p| from + p)
        .filter(|p| after || *p < at)
        .any(|p| {
            let (lo, hi) = if p < at { (p, at) } else { (at, p) };
            no_conjunction(&lower[lo .. hi])
        })
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

/// Every segment of a clause with the offset it starts at. A segment ends at a
/// comma or at either side of a bracket, which is what a parenthetical, an
/// apposition and a list item are all cut by.
fn segments(clause: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut from = 0;
    for (i, c) in clause.char_indices() {
        if matches!(c, ',' | '(' | ')' | '{' | '}') {
            out.push((from, &clause[from .. i]));
            from = i + 1;
        }
    }
    out.push((from, &clause[from ..]));
    out
}

/// Which segment an offset lies in.
fn segment_of(segs: &[(usize, &str)], at: usize) -> usize {
    segs.iter()
        .rposition(|&(from, _)| from <= at)
        .unwrap_or_default()
}

/// Every clause of `text` with the offset it starts at. A clause ends at `;`,
/// at `.`, `:`, `?` or `!` before whitespace or the end, and at a blank line.
fn clauses(text: &str) -> Vec<(usize, &str)> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut from = 0;
    for i in 0 .. bytes.len() {
        let next = bytes.get(i + 1).copied();
        let ends = match bytes[i] {
            b';' => true,
            b'.' | b':' | b'?' | b'!' => next.is_none_or(|b| b.is_ascii_whitespace()),
            b'\n' => {
                text[i + 1 ..]
                    .split('\n')
                    .next()
                    .is_some_and(|l| l.trim().is_empty())
            },
            _ => false,
        };
        if ends {
            out.push((from, &text[from ..= i]));
            from = i + 1;
        }
    }
    if from < text.len() {
        out.push((from, &text[from ..]));
    }
    out
}

/// Every spelling of the name in `text`, bounded, with where it starts.
pub(crate) fn spellings_in(text: &str) -> Vec<(usize, &'static str)> {
    let mut out: Vec<(usize, &'static str)> = SPELLINGS
        .iter()
        .flat_map(|s| bounded(text, s).into_iter().map(move |at| (at, *s)))
        .collect();
    out.sort();
    out
}

/// Every reading in an already lowered `lower`, bounded, with where it starts.
/// The qualified one counts only behind a direction word in its own segment.
pub(super) fn readings_in(lower: &str, segs: &[(usize, &str)]) -> Vec<(usize, &'static str)> {
    let mut out: Vec<(usize, &'static str)> = READINGS
        .iter()
        .flat_map(|s| bounded(lower, s).into_iter().map(move |at| (at, *s)))
        .filter(|&(at, s)| s != QUALIFIED || directed(lower, segs, at))
        .filter(|&(at, s)| !mentioned(lower, at, s))
        .collect();
    out.sort();
    out
}

/// Whether the reading at `at` is a term being quoted rather than a reading
/// being given: inside a code span, with a word beside the span that says the
/// span is a name. A code span on its own is not enough, since a clause saying
/// the mode does `roundTiesToAway` is asserting exactly the thing this reads
/// for.
fn mentioned(lower: &str, at: usize, term: &str) -> bool {
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
    MENTIONED.contains(&after.as_str()) || MENTIONED.contains(&before.as_str())
}

/// Whether a direction word stands in front of `at`, in the same segment.
fn directed(lower: &str, segs: &[(usize, &str)], at: usize) -> bool {
    let from = segs[segment_of(segs, at)].0;
    let head = &lower[from .. at];
    DIRECTIONS.iter().any(|d| !bounded(head, d).is_empty())
}

/// Whether the clause says it is about some other rule than the shipped one.
fn elsewhere(lower: &str) -> bool {
    ELSEWHERE.iter().any(|m| !bounded(lower, m).is_empty())
}

/// Where `needle` occurs in `hay` with no identifier character on either side.
pub(super) fn bounded(hay: &str, needle: &str) -> Vec<usize> {
    let ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    hay.match_indices(needle)
        .filter(|(at, _)| {
            !ident(hay[.. *at].chars().next_back())
                && !ident(hay[at + needle.len() ..].chars().next())
        })
        .map(|(at, _)| at)
        .collect()
}
