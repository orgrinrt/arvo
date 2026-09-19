//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Reading one passage of prose for a magnitude reading of the mode's name.
//!
//! A passage is cut into clauses, and a clause is where a name and a reading
//! are paired. Table rows are read apart from the clauses, one row at a time,
//! because a row pairs its first cell with the others and a clause cut at every
//! `|` would never see that pairing.

/// Every spelling of the mode's name this repository writes. Matched with case,
/// and bounded by a character that cannot sit inside an identifier, so a
/// longer identifier carrying one of them is not a mention of the mode. Java's
/// and Python's upper-case constant is not among them: it names the other
/// operation, and saying so is true.
pub(super) const SPELLINGS: &[&str] =
    &["half_up", "HalfUp", "half-up", "half up", "Half-up", "Half up", "Half_up"];

/// The readings that put a tie by its magnitude or its sign. Matched without
/// case, with the same bounds.
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

/// Words that turn a pairing into a contrast when they come before the later
/// of the two. Matched without case and bounded as a word, and any word ending
/// in `n't` counts too.
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

/// One pairing the lint refuses: where in the passage, which spelling, which
/// reading.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Hit {
    pub(super) at:      usize,
    pub(super) name:    String,
    pub(super) reading: &'static str,
}

/// Every refused pairing in `text`. With `subject`, every clause is read as
/// already naming the mode at its start, which is how a doc block reads when
/// the item it documents is the mode.
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
    for (from, clause) in clauses(&prose) {
        if let Some(hit) = clause_hit(clause, subject) {
            out.push(Hit {
                at: from + hit.at,
                ..hit
            });
        }
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
        if let Some((r, reading)) = readings_in(&lower).first() {
            if !negated(&lower, *r) {
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
fn clause_hit(clause: &str, subject: Option<&str>) -> Option<Hit> {
    let lower = clause.to_ascii_lowercase();
    let readings = readings_in(&lower);
    if readings.is_empty() || elsewhere(&lower) {
        return None;
    }
    let mut names: Vec<(usize, &str)> = spellings_in(clause);
    if let Some(s) = subject {
        names.insert(0, (0, s));
    }
    if names.is_empty() || listed_apart(clause, &names, &readings) {
        return None;
    }
    for &(n, name) in &names {
        for &(r, reading) in &readings {
            let later = if n > r { n } else { r };
            if !negated(&lower, later) {
                return Some(Hit {
                    at: r,
                    name: name.to_string(),
                    reading,
                });
            }
        }
    }
    None
}

/// Whether the clause is a list and no item of it holds both a name and a
/// reading. Items are cut at `,`, `{` and `}`, and a clause with fewer than two
/// of those is not a list.
fn listed_apart(clause: &str, names: &[(usize, &str)], readings: &[(usize, &str)]) -> bool {
    let cuts: Vec<usize> = clause
        .char_indices()
        .filter(|(_, c)| matches!(c, ',' | '{' | '}'))
        .map(|(i, _)| i)
        .collect();
    if cuts.len() < 2 {
        return false;
    }
    let item = |at: usize| cuts.iter().filter(|&&c| c < at).count();
    !names
        .iter()
        .any(|&(n, _)| readings.iter().any(|&(r, _)| item(n) == item(r)))
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
pub(super) fn spellings_in(text: &str) -> Vec<(usize, &'static str)> {
    let mut out: Vec<(usize, &'static str)> = SPELLINGS
        .iter()
        .flat_map(|s| bounded(text, s).into_iter().map(move |at| (at, *s)))
        .collect();
    out.sort();
    out
}

/// Every reading in an already lowered `lower`, bounded, with where it starts.
fn readings_in(lower: &str) -> Vec<(usize, &'static str)> {
    let mut out: Vec<(usize, &'static str)> = READINGS
        .iter()
        .flat_map(|s| bounded(lower, s).into_iter().map(move |at| (at, *s)))
        .collect();
    out.sort();
    out
}

/// Whether a negator sits in `lower` before `until`.
fn negated(lower: &str, until: usize) -> bool {
    let head = &lower[.. until];
    NEGATORS.iter().any(|n| !bounded(head, n).is_empty())
        || head
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
            .any(|w| w.ends_with("n't"))
}

/// Whether the clause says it is about some other rule than the shipped one.
fn elsewhere(lower: &str) -> bool {
    ELSEWHERE.iter().any(|m| !bounded(lower, m).is_empty())
}

/// Where `needle` occurs in `hay` with no identifier character on either side.
fn bounded(hay: &str, needle: &str) -> Vec<usize> {
    let ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    hay.match_indices(needle)
        .filter(|(at, _)| {
            !ident(hay[.. *at].chars().next_back())
                && !ident(hay[at + needle.len() ..].chars().next())
        })
        .map(|(at, _)| at)
        .collect()
}
