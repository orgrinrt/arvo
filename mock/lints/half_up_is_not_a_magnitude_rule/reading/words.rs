//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Whether a word that could be a name is standing as one, and whether a
//! clause opens on a subject the lists have no name for.
//!
//! Some of what the reader matches is ordinary English as well as a name:
//! MATLAB's method names under their capital, the lower-case `toward zero`, and
//! the alias's spellings that are also readings. And a clause can open on a
//! subject no list names: a crate, a seat, a number. Each question here is
//! answered from the words right beside the match, read against closed lists in
//! `vocabulary.rs`, and none of them decides a pairing on its own.

use super::vocabulary::{
    ARTICLES,
    CONJUNCTIONS,
    IMPERATIVE_NEXT,
    NAME_LEADS,
    OTHER_NAMES,
    RULE_NOUNS,
    SUBJECTS,
};

/// Whether the other rule's name, found at `at` in `text` in the form `form`,
/// stands there as the name rather than as an ordinary word.
pub(super) fn stands_as_a_name(text: &str, at: usize, form: &str) -> bool {
    if form == "toward zero" {
        return toward_zero_names(text, at);
    }
    !imperative(text, at, form)
}

/// Whether a capitalised ordinary word opening `text` at `at` is an imperative
/// or a plain word: the word right behind it is one `IMPERATIVE_NEXT` holds.
///
/// Only a single word with one capital is asked, which is the shape a MATLAB
/// method name and an English word share. `TowardZero` and `HALF_UP` are never
/// English and are not asked.
fn imperative(text: &str, at: usize, form: &str) -> bool {
    let mut chars = form.chars();
    let ordinary = chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && chars.all(|c| c.is_ascii_lowercase());
    ordinary
        && !text[.. at].chars().any(char::is_alphanumeric)
        && IMPERATIVE_NEXT.contains(&next_word(&text[at + form.len() ..]).as_str())
}

/// The word right behind a match, lowered, where only whitespace stands
/// between. Punctuation in between gives no word.
fn next_word(text: &str) -> String {
    text.trim_start_matches(char::is_whitespace)
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '\'')
        .collect::<String>()
        .to_ascii_lowercase()
}

/// The word right ahead of `at`, lowered, where only whitespace stands between.
fn word_before(text: &str, at: usize) -> Option<String> {
    let head = text[.. at].trim_end_matches(char::is_whitespace);
    let word: String = head
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '\'')
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    (!word.is_empty()).then(|| word.to_ascii_lowercase())
}

/// Whether the lower-case `toward zero` at `at` is the rule's name: nothing
/// stands ahead of it but whitespace, a bracket, a comma or a backtick, or the
/// word ahead of it is one of `NAME_LEADS`.
fn toward_zero_names(text: &str, at: usize) -> bool {
    match word_before(text, at) {
        None => true,
        Some(word) => NAME_LEADS.contains(&word.as_str()),
    }
}

/// Whether a clause opens on a subject of its own that no list of rules names.
///
/// Three shapes. A code span that is not a number, since a code span names a
/// thing and a number in one is a value the mode acts on: "`-2.5` goes away
/// from zero" is about the mode, "`arvo-format` sends a tie away from zero" is
/// about a crate. A bare whole number, which is how the registry names a seat:
/// "270 read blind". And a noun from `SUBJECTS`, behind an article or not, in
/// the first two words: "The crate", "The shipped crate", "Seat 270".
pub(super) fn opens_on_a_subject(clause: &str) -> bool {
    let text = clause.trim_start();
    if let Some(span) = text.strip_prefix('`') {
        return !a_number(span.split('`').next().unwrap_or_default());
    }
    let first = text.split(char::is_whitespace).next().unwrap_or_default();
    if !first.is_empty() && first.bytes().all(|b| b.is_ascii_digit()) {
        return true;
    }
    let words: Vec<String> = text
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .filter(|w| !w.is_empty())
        .take(3)
        .map(str::to_ascii_lowercase)
        .collect();
    let skip = usize::from(
        words
            .first()
            .is_some_and(|w| ARTICLES.contains(&w.as_str())),
    );
    words
        .iter()
        .skip(skip)
        .take(2)
        .any(|w| SUBJECTS.contains(&w.as_str()))
}

/// Whether the body of a code span is a number: a digit first, behind a sign or
/// a point where one stands.
fn a_number(body: &str) -> bool {
    body.trim_start_matches(['-', '+', '.'])
        .starts_with(|c: char| c.is_ascii_digit())
}

/// Whether the reading at `r`, in the segment starting at `from`, is a rule's
/// name standing as one: it is spelled there as an entry of `OTHER_NAMES`, and
/// it either opens its segment, past a conjunction, `where` and an article, or
/// has one of `RULE_NOUNS` right behind it.
///
/// The caller asks this only of a segment that does not name the mode.
pub(super) fn a_reading_stands_as_a_name(
    text: &str,
    lower: &str,
    from: usize,
    r: usize,
    reading: &str,
) -> bool {
    let spelled = &text[r .. r + reading.len()];
    if !OTHER_NAMES.contains(&spelled) {
        return false;
    }
    let opens = lower[from .. r]
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .filter(|w| !w.is_empty())
        .all(|w| {
            CONJUNCTIONS.contains(&w) || ARTICLES.contains(&w) || matches!(w, "where" | "when")
        });
    let behind = next_word(text[r + reading.len() ..].trim_start_matches('`'));
    opens || RULE_NOUNS.contains(&behind.as_str())
}

#[cfg(test)]
mod tests {
    use super::{a_number, opens_on_a_subject, stands_as_a_name};

    #[test]
    fn a_method_name_followed_by_a_verb_is_the_method_and_by_an_object_is_not() {
        assert!(stands_as_a_name("Round is what goes", 0, "Round"));
        assert!(stands_as_a_name("Ceiling sends a tie", 0, "Ceiling"));
        assert!(!stands_as_a_name("Round every tie", 0, "Round"));
        assert!(!stands_as_a_name("Fix the doc", 0, "Fix"));
        assert!(!stands_as_a_name(" Ceiling aside, a tie", 1, "Ceiling"));
        // Not opening the text, so a possessive or anything else stands ahead.
        assert!(stands_as_a_name("MATLAB's Round the tie", 9, "Round"));
        // Never English, never asked.
        assert!(stands_as_a_name("TowardZero the tie", 0, "TowardZero"));
    }

    #[test]
    fn toward_zero_is_a_name_where_a_noun_phrase_stands() {
        assert!(stands_as_a_name("for toward zero and", 4, "toward zero"));
        assert!(stands_as_a_name("toward zero switches", 0, "toward zero"));
        assert!(stands_as_a_name("is not `toward zero`", 8, "toward zero"));
        assert!(!stands_as_a_name(
            "send a tie toward zero",
            11,
            "toward zero"
        ));
        assert!(!stands_as_a_name(
            "it rounds toward zero",
            10,
            "toward zero"
        ));
    }

    #[test]
    fn a_clause_opens_on_a_subject_where_a_list_has_no_name_for_it() {
        for subject in [
            "`arvo-format` sends a tie",
            "270 read blind",
            "Seat 270 read blind",
            "The crate sends a tie",
            "The shipped crate sends a tie",
        ] {
            assert!(opens_on_a_subject(subject), "{subject}");
        }
        for not in [
            "`-2.5` goes away from zero",
            "`0.5` goes",
            "A tie goes away",
            "Its tie goes",
            "2.5 goes to 3",
            "-3 is away from zero",
            "Either way a tie goes",
        ] {
            assert!(!opens_on_a_subject(not), "{not}");
        }
    }

    #[test]
    fn a_number_is_a_digit_behind_a_sign_or_a_point() {
        assert!(a_number("-2.5"));
        assert!(a_number(".5"));
        assert!(a_number("3 q"));
        assert!(!a_number("q/2"));
        assert!(!a_number("arvo-format"));
        assert!(!a_number("-"));
    }
}
