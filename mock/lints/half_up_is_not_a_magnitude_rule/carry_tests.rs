//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Which rule a clause, a pronoun and a reading are about: what a name carries
//! from one clause to the next, where the carry stops, and which rule a reading
//! in a clause naming two of them is attributed to.
//!
//! Every arm here is walked over the whole of the list it depends on, other
//! names, relatives, pronouns, conjunctions and negators, with the control
//! beside it that the same sentence with the roles swapped fires. The walks
//! skip exactly the names that are also a reading or a negator, and the arm
//! that computes that set asserts what it is, so the skip is a fact about the
//! lists rather than a choice of which names to ask about.

use super::reading::{
    CONJUNCTIONS,
    NEGATORS,
    OTHER_NAMES,
    PRONOUNS,
    READINGS,
    RELATIVES,
    bounded,
    hits,
};

/// The readings found in `text`, as written.
fn found(text: &str) -> Vec<&'static str> {
    hits(text, None).into_iter().map(|h| h.reading).collect()
}

/// The other names a sentence can put in a role without the name itself
/// deciding the answer: every one that is neither a reading nor a negator.
fn plain_names() -> Vec<&'static str> {
    OTHER_NAMES
        .iter()
        .copied()
        .filter(|n| !doubles_as_a_term(n))
        .collect()
}

/// Whether an other name is also a reading or a negator, as the reader matches
/// either: lowered, and bounded as a word.
fn doubles_as_a_term(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    READINGS
        .iter()
        .chain(NEGATORS)
        .any(|t| !bounded(&lower, t).is_empty())
}

/// A word with its first letter upper-cased, as it opens a sentence.
fn cap(word: &str) -> String {
    let mut c = word.chars();
    c.next()
        .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
        .unwrap_or_default()
}

#[test]
fn the_names_the_walks_skip_are_exactly_the_ones_that_double_as_a_term() {
    // The skip in `plain_names` is computed, and this names what it computes,
    // so a list edit that changes it is read here rather than absorbed. Three
    // of the four are a reading as well as a name, which is the vocabulary
    // overlap the catalogue carries; `alias` is a negator.
    let mut skipped: Vec<&str> = OTHER_NAMES
        .iter()
        .copied()
        .filter(|n| doubles_as_a_term(n))
        .collect();
    skipped.sort_unstable();
    assert_eq!(skipped, [
        "alias",
        "roundTiesToAway",
        "ties-away",
        "ties_away"
    ]);
    assert_eq!(plain_names().len() + skipped.len(), OTHER_NAMES.len());
}

#[test]
fn a_clause_boundary_separates_the_two_only_where_the_later_clause_names_another_rule() {
    // A boundary on its own does not separate them. The clause after it names
    // no rounding rule, so what it says is about the one the clause before it
    // named, which is how anybody writes a definition after a colon or a second
    // sentence about the same mode. What does separate them is the later clause
    // naming a rule of its own.
    for cut in [". ", "; ", ": ", "? ", "! ", ".\n", ";"] {
        let other = format!("`half_up` goes up{cut}`HalfEven` takes ties away from zero.");
        assert!(found(&other).is_empty(), "{other:?}");
        let alias = format!("`half_up` goes up{cut}ties away from zero is the alias.");
        assert!(found(&alias).is_empty(), "{alias:?}");
        // The control. The same cut with nothing else named pairs the two, so
        // the two silences above are the naming rather than the cut, and a
        // reader that stopped carrying anything at all would fail here.
        let carried = format!("`half_up` goes up{cut}ties away from zero is what it does.");
        assert!(!found(&carried).is_empty(), "{carried:?}");
    }
}

#[test]
fn a_blank_line_starts_afresh_whatever_was_named() {
    // The one boundary that does separate them on its own, which is why it is
    // tested apart from the cuts above rather than among them.
    assert!(found("`half_up` goes up.\n\nties away from zero is what it does.").is_empty());
    assert!(found("`half_up` goes up.\n\n`HalfEven` takes ties away from zero.").is_empty());
    // The control: the same two lines with one newline between them carry.
    assert_eq!(
        found("`half_up` goes up.\nties away from zero is what it does."),
        ["away from zero"]
    );
}

#[test]
fn every_other_name_defeats_the_carry() {
    // A pronoun standing for a rounding rule points at the last rule named, so
    // each of these standing after the mode takes the carry away. Walked whole
    // and in both spellings, because a reader counting backticks let the
    // backticked half through and refused the bare half.
    for n in OTHER_NAMES {
        for span in ["`", ""] {
            let text = format!("`half_up` is not {span}{n}{span}. It sends a tie away from zero.");
            assert!(found(&text).is_empty(), "{text}");
        }
    }
    // The control. The same sentence naming no other rule fires, so the silence
    // above is the name rather than the shape of the sentence or the negator.
    assert_eq!(
        found("`half_up` is not Java's rule. It sends a tie away from zero."),
        ["away from zero"]
    );
}

#[test]
fn every_other_name_opening_a_sentence_capitalised_stops_the_carry() {
    // The design documents open sentences on a rule's name, bare, and that name
    // is then capitalised whatever the list spells it as. Every plain name whose
    // capitalised form differs from the listed one, opening the clause after the
    // mode's, takes that clause for itself.
    for n in plain_names() {
        let opened = cap(n);
        if opened == n {
            continue;
        }
        let text = format!("`half_up` adds half a step. {opened} sends a tie away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    // The control: a capitalised word that is no rule's name opens the same
    // clause and the carry reaches through it.
    assert_eq!(
        found("`half_up` adds half a step. Either way a tie goes away from zero."),
        ["away from zero"]
    );
}

#[test]
fn a_name_that_is_not_a_rounding_rule_does_not_defeat_the_carry() {
    // Most of what this repository names in a code span is not something a
    // pronoun standing for a mode could mean. A reader counting backticks lost
    // the carry to every one of these.
    for span in ["`q/2`", "`Exact::slot`", "`i64::MAX`", "`-2.5 q`"] {
        let text = format!("`half_up` pairs with {span}; it sends a tie away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
}

#[test]
fn every_negator_ahead_of_the_mode_takes_it_out_of_the_carry() {
    // A clause saying something is not the mode leaves that something as what a
    // pronoun after it means. The governing ruling writes exactly this: "Ties
    // away from zero is a different operation and is not `half_up`: it is what
    // IEEE 754's `roundTiesToAway` computes."
    for n in NEGATORS {
        let text = format!("The operation is {n} `half_up`. It goes away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    // The control: the same two sentences without the negator carry the mode.
    assert_eq!(
        found("The operation is `half_up`. It goes away from zero."),
        ["away from zero"]
    );
    // A negator behind the mode denies something of it and leaves it named, so
    // the carry stands.
    assert_eq!(
        found("`half_up` is not the even rule. It goes away from zero."),
        ["away from zero"]
    );
}

#[test]
fn the_carry_reaches_across_a_clause_that_names_nothing() {
    // Anaphora over more than one sentence, which is ordinary prose: the mode
    // is named once and two sentences follow about it.
    assert_eq!(
        found("`half_up` is a mode. It is nearest. Its tie goes away from zero."),
        ["away from zero"]
    );
    // And stops at a clause that names a rule of its own.
    assert!(
        found("`half_up` is a mode. `HalfEven` is another. Its tie goes away from zero.")
            .is_empty()
    );
}

#[test]
fn every_conjunction_opening_a_clause_is_read_past_to_the_pronoun_behind_it() {
    // "And it is nearest" points back as plainly as "It is nearest" does, so it
    // passes the carry on, and with a doc block's item standing by it does not
    // let the item fill in where the clause before named something else.
    for c in CONJUNCTIONS {
        let opened = cap(c);
        let carried =
            format!("`half_up` is a mode. {opened} it is nearest. Its tie goes away from zero.");
        assert_eq!(found(&carried), ["away from zero"], "{carried}");
        let pointed = format!("`HalfEven` is a mode. {opened} it sends a tie away from zero.");
        assert!(hits(&pointed, Some("HalfUp")).is_empty(), "{pointed}");
    }
    // The control: without the pronoun the clause names nothing and points at
    // nothing, so the doc block's item fills in, and the silences above are
    // the pronoun rather than the conjunction.
    assert_eq!(
        hits(
            "`HalfEven` is a mode. And a tie goes away from zero.",
            Some("HalfUp")
        )
        .len(),
        1
    );
}

#[test]
fn a_doc_blocks_item_does_not_reach_a_clause_that_names_another_rule() {
    assert!(
        hits(
            "A tie goes away from zero under `HalfEven`.",
            Some("HalfUp")
        )
        .is_empty()
    );
    // The control: the same clause naming no rule takes the item.
    assert_eq!(
        hits("A tie goes away from zero under it.", Some("HalfUp")).len(),
        1
    );
}

#[test]
fn a_clause_opening_with_a_pronoun_takes_the_name_the_one_before_it_left() {
    // The sentence is about the mode and says so once, which is how anybody
    // writes two sentences about one thing.
    assert_eq!(
        found("`half_up` is nearest. Its tie goes away from zero."),
        ["away from zero"]
    );
    assert_eq!(found("`half_up` is nearest. It goes away from zero."), [
        "away from zero"
    ]);
}

#[test]
fn control_a_pronoun_points_at_whatever_was_named_last() {
    // The clause before names the other operation last, so the pronoun after it
    // is about that and not about the mode.
    assert!(
        found("`half_up` is not the `HALF_UP` of Java. That one goes away from zero.").is_empty()
    );
    // Nothing named at all, so the pronoun points at nothing this reads.
    assert!(found("A tie at -2.5 goes to -2. It is ties away from zero.").is_empty());
    // And a clause that opens with a pronoun after a blank line starts afresh.
    assert!(found("`half_up` is nearest.\n\nIts tie goes away from zero.").is_empty());
}

#[test]
fn control_what_is_not_a_boundary_keeps_the_clause_whole() {
    assert_eq!(found("`half_up` at 2.5 goes away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`ruling::half_up` goes away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`half_up` sends a tie\naway from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`half_up`, away from zero."), ["away from zero"]);
}

#[test]
fn a_name_carries_one_clause_and_no_further_unless_a_pronoun_re_points_it() {
    // Anaphora has a reach. A name reaches the clause after the one that gave
    // it, and a clause that points back re-points it so the next one has it
    // too. What it does not do is run through material that never mentions the
    // mode, which is what a passage laying out two readings of a word does:
    // `question::which_operation_half_up_denotes` reads its `asks` field that
    // way, and a reader without this bound refuses the row that asks the
    // question the ruling answers.
    assert_eq!(
        found("`half_up` is a mode. It is read up the number line. A tie goes away from zero."),
        ["away from zero"]
    );
    assert!(
        found("`half_up` is a mode. Read it as up the number line. A tie goes away from zero.")
            .is_empty()
    );
    // Two clauses of reach is the boundary, so the one-clause case still fires.
    assert_eq!(found("`half_up` is a mode. A tie goes away from zero."), [
        "away from zero"
    ]);
}

#[test]
fn a_clause_opening_with_a_code_span_has_a_subject_of_its_own() {
    // The reader has no vocabulary for most of what this repository names, and
    // a clause opening by naming one of them is about that thing. The registry
    // row recording what the crate asserted before the ruling reads this way.
    assert!(found("`half_up` is a mode. `arvo-format` sends a tie away from zero.").is_empty());
    // The same clause naming its subject in words rather than in a code span is
    // not distinguished, and carries: neither reading is a claim about what the
    // mode denotes, so both have to stay silent, and this one does not.
    // `sentences/catalogue.rs` carries the sentence it costs rather than a
    // watered-down assertion here.
}

#[test]
fn every_other_name_ahead_of_the_reading_in_its_segment_takes_the_reading() {
    // A clause naming the mode and then, in a later segment, another rule
    // doing something, gives what that segment says to the other rule. Walked
    // over every plain name, backticked and bare.
    for n in plain_names() {
        for span in ["`", ""] {
            let text =
                format!("`half_up` is nearest, and {span}{n}{span} sends a tie away from zero.");
            assert!(found(&text).is_empty(), "{text}");
        }
    }
    // The control: the same segment with a pronoun where the name was is about
    // the mode, which is the one rule standing before it.
    assert_eq!(
        found("`half_up` is nearest, and it sends a tie away from zero."),
        ["away from zero"]
    );
    // And a segment naming the mode ahead of the reading keeps it, whatever
    // else that segment names: the mode is what the segment is about.
    assert_eq!(
        found("`half_up` is nearest, but `HalfEven` like `half_up` sends a tie away from zero."),
        ["away from zero"]
    );
}

#[test]
fn every_relative_binds_the_reading_to_the_rule_standing_last() {
    // Every relative, against every plain name, in both orders: the relative
    // points at whatever the clause named last, so the reading is the other
    // rule's in one order and the mode's in the other.
    for rel in RELATIVES {
        for n in plain_names() {
            let other_last =
                format!("`half_up` is next to `{n}`, {rel} sends a tie away from zero.");
            assert!(found(&other_last).is_empty(), "{other_last}");
            let mode_last =
                format!("`{n}` is next to `half_up`, {rel} sends a tie away from zero.");
            assert_eq!(found(&mode_last), ["away from zero"], "{mode_last}");
        }
    }
    // A relative standing behind an apposition binds the same way.
    assert!(
        found("`half_up` is not `ROUND_HALF_UP`, the rule that goes away from zero.").is_empty()
    );
    assert_eq!(
        found("`half_up` is a nearest rule, the one that goes away from zero."),
        ["away from zero"]
    );
}

#[test]
fn every_pronoun_opening_a_segment_binds_the_reading_to_the_rule_standing_last() {
    // The same binding with a pronoun in place of the relative, opening the
    // segment or opening it behind a conjunction.
    for p in PRONOUNS {
        for c in ["", "and ", "but ", "so "] {
            let other_last =
                format!("Next to `half_up` stands `HalfEven`, {c}{p} sends a tie away from zero.");
            assert!(found(&other_last).is_empty(), "{other_last}");
            let mode_last =
                format!("Next to `HalfEven` stands `half_up`, {c}{p} sends a tie away from zero.");
            assert_eq!(found(&mode_last), ["away from zero"], "{mode_last}");
        }
    }
}

#[test]
fn a_segment_that_neither_names_nor_points_back_leaves_the_reading_with_the_mode() {
    // An aside naming another rule between the mode and the reading does not
    // take the reading: the segment carrying it names nothing and binds nothing.
    for n in plain_names() {
        let text = format!("`half_up`, like `{n}`, sends a tie away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
    // The control: the same aside turned into a relative on the other rule.
    assert!(found("`half_up` is like `floor`, which sends a tie away from zero.").is_empty());
}

#[test]
fn a_table_cell_attributes_its_reading_the_same_way() {
    assert_eq!(
        found("| `half_up` | nearest, and it sends a tie away from zero |"),
        ["away from zero"]
    );
    assert!(found("| `half_up` | nearest, and `HalfEven` sends a tie away from zero |").is_empty());
    assert_eq!(
        found("| `half_up` | nearest, which goes away from zero |"),
        ["away from zero"]
    );
    assert!(found("| `half_up` | next to `HalfEven`, which goes away from zero |").is_empty());
}
