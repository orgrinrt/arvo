//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where a passage is cut, and where a word begins.
//!
//! Two cuts, doing different work. The clause is the unit a name and a reading
//! are paired in. The segment is the unit an escape is bound to, and is also
//! the unit a reading is attributed in: a segment that points back at the
//! segment before it is about whatever that one named, which is the same
//! anaphora the clause cut reads, one level down.

use super::vocabulary::{CONJUNCTIONS, PRONOUNS, RELATIVES};

/// Every clause of `text` with the offset it starts at. A clause ends at `;`,
/// at `.`, `:`, `?` or `!` before whitespace or the end, and at a blank line.
pub(super) fn clauses(text: &str) -> Vec<(usize, &str)> {
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

/// Every segment of a clause with the offset it starts at. A segment ends at a
/// comma or at either side of a bracket, which is what a parenthetical, an
/// apposition and a list item are all cut by.
pub(super) fn segments(clause: &str) -> Vec<(usize, &str)> {
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
pub(super) fn segment_of(segs: &[(usize, &str)], at: usize) -> usize {
    segs.iter()
        .rposition(|&(from, _)| from <= at)
        .unwrap_or_default()
}

/// Whether a stretch of lowered text opens by pointing back rather than by
/// naming.
///
/// A conjunction standing first is read past. "..., and it sends a tie away
/// from zero" points back exactly as "..., it sends a tie away from zero" does,
/// and a reader stopping at the conjunction reads the first as naming nothing,
/// so the pronoun never gets asked what it means.
pub(super) fn opens_with_a_pronoun(lower: &str) -> bool {
    PRONOUNS.contains(&first_word(past_a_conjunction(lower)))
}

/// The first word of a stretch of lowered text.
fn first_word(lower: &str) -> &str {
    lower
        .trim_start_matches(|c: char| !c.is_alphanumeric())
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .next()
        .unwrap_or_default()
}

/// `lower` with one conjunction opening it taken off, where one does.
fn past_a_conjunction(lower: &str) -> &str {
    let text = lower.trim_start_matches(|c: char| !c.is_alphanumeric());
    let first = first_word(text);
    if CONJUNCTIONS.contains(&first) { &text[first.len() ..] } else { text }
}

/// Whether a stretch of lowered text binds itself to what stood before it: it
/// opens with a pronoun, or a relative stands anywhere in it.
///
/// The second half is what an apposition needs. "..., the rule that goes away
/// from zero" points back as plainly as "..., which goes away from zero" does,
/// and only the second of the two opens with the word that says so.
pub(super) fn points_back(lower: &str) -> bool {
    opens_with_a_pronoun(lower) || RELATIVES.iter().any(|w| !bounded(lower, w).is_empty())
}

/// A name with its first character upper-cased, which is how it is written at
/// the start of a sentence.
pub(super) fn capitalised(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// How many words a segment holds that are not the filler a list item allows.
pub(super) fn words_of(text: &str) -> usize {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '\''))
        .filter(|w| {
            !w.is_empty() && !super::vocabulary::FILLER.contains(&w.to_ascii_lowercase().as_str())
        })
        .count()
}

/// Where `needle` occurs in `hay` with no identifier character on either side.
pub(crate) fn bounded(hay: &str, needle: &str) -> Vec<usize> {
    let ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    hay.match_indices(needle)
        .filter(|(at, _)| {
            !ident(hay[.. *at].chars().next_back())
                && !ident(hay[at + needle.len() ..].chars().next())
        })
        .map(|(at, _)| at)
        .collect()
}
