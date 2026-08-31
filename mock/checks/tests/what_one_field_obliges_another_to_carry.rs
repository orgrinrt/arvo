//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conditional obligations, which a schema can state none of.
//!
//! Every arm here is required-if-something-else, and each has both directions
//! planted: the case that must be reported, and the neighbouring case that must
//! not be, because an arm that reports everything passes the first half of its
//! own control and is useless.

use arvo_checks::{canon, parse, shape};

#[test]
fn the_committed_canon_leaves_no_refusal_without_an_alternative() {
    let found = shape::refusals_without_an_instead(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

/// The claims that ran something and name no instrument, pinned by count.
///
/// **This carried an `#[ignore]` whose stated reason was false**, and it is
/// worth saying so rather than quietly swapping it. The reason read "the
/// `probe` namespace is empty and is being written". It was true when written
/// and stopped being true when 48 probe rows landed; nobody updated it, so a
/// catalogue-red sat behind an explanation that had become a lie, which is the
/// exact class this suite exists to catch in other people's work. The seat that
/// found it ran the test with `--ignored` and read what came out, which is the
/// one thing an ignore stops anybody doing by accident.
///
/// What is actually true: 48 probes cover 27 of 135 directories, chosen by one
/// seat's selection, and the claims from the earlier topics are not among them.
/// Eight further claims are settled and unwritable for the same reason, the
/// instrument being committed with no row naming it. So the gap is a namespace
/// half-filled rather than an empty one, and filling the rest is a dispatch.
///
/// A ceiling rather than an ignore, because a ceiling keeps reporting. Lower it
/// as probe rows land; raising it means a new claim was written with nothing
/// behind it, and that is not the fix.
#[test]
fn no_new_measurement_lands_without_an_instrument() {
    /// Measured after the evidence pass wired fifteen of the twenty-one.
    ///
    /// The six that remain are not a backlog of unwritten rows, and no two of
    /// them are stuck for the same reason. One is an absence claim nobody
    /// built an instrument for and nobody should. Two ran an arm whose
    /// directory has a probe row describing a different arm, so a row moves
    /// before an edge can. One ran in a directory with no probe row at all,
    /// and the nearest row by name is its predecessor in another directory.
    /// One rests on a probe whose own control admits none was run, so the arm
    /// above refuses the edge and the honest repair is to the sentence rather
    /// than to this number. And one was offered a probe whose `establishes`
    /// states the proposition the claim records as refuted.
    const WITHOUT_AN_INSTRUMENT: usize = 6;

    let found = shape::measured_without_evidence(&canon());
    assert!(
        found.len() <= WITHOUT_AN_INSTRUMENT,
        "{} claims ran something and name no instrument, against a ceiling of \
         {WITHOUT_AN_INSTRUMENT}. Name the committed probe, or mark the sentence as the \
         argument it is: {found:#?}",
        found.len()
    );
}

/// The ratchet above is green, and this is what says the green means anything.
///
/// A ceiling passes two ways: the finder found few, or the finder found none
/// because it stopped working. The second is silent, survives a refactor of the
/// field names it reads, and leaves a check that can never fail again sitting in
/// the suite looking like coverage.
///
/// So the finder is asserted non-empty. That is the reverse of what a check
/// usually wants and it is correct here: while any row is stuck, an empty result
/// is evidence about the instrument rather than about the canon. The day the
/// last one is wired this test is what fails, which is the right moment to
/// notice, and the repair is to drop the ceiling and this control together.
///
/// Run by hand first, by setting the ceiling to zero and reading what it named:
/// six rows, which is the ceiling exactly. A header elsewhere in the registry
/// had claimed the check was red on purpose because no probe rows existed, and
/// that is what sent somebody to look.
#[test]
fn the_measured_without_evidence_finder_still_finds_the_stuck_rows() {
    let found = shape::measured_without_evidence(&canon());
    assert!(
        !found.is_empty(),
        "the finder reports nothing, so the ceiling beside it is passing \
         vacuously. Either every stuck row was wired, in which case drop the \
         ceiling and this control, or the finder stopped reading the fields it \
         means to read."
    );
}

#[test]
fn the_committed_canon_agrees_with_itself_about_regions() {
    let found = shape::predicate_disagrees_with_the_sentence_kind(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn the_committed_canon_stamps_nothing_on_an_ack() {
    let found = shape::stamps_from_an_unratified_ruling(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

/// Two measurements rest on an instrument that ran no case that had to fail,
/// pinned by name.
///
/// **This is the check working rather than a hole in it.** Both edges were
/// written from a committed map, both probes had already been named in prose as
/// thin, and the register saying so changed nothing because prose is not a
/// gate. Declaring `standing = "uncontrolled"` moved the statement to where the
/// gate reads, and these two turned red the moment it did.
///
/// **Both are repaired and neither repair was the one this comment predicted.**
/// It said the honest options were marking the sentence as the argument it is,
/// or building the case that had to fail. The second happened once and the other
/// turned out not to need it.
///
/// The no-residue finding got the arm its own note had named in one sentence:
/// the identical argmin over a weighting arriving as an argument, so nothing can
/// fold it. It comes out the other way by a wide margin, nine instructions
/// against seventy-six and two branches against eleven, with the const form
/// identical to the hand-written arm after label normalisation.
/// `197_probes/c1`.
///
/// The binding-time finding needed no new instrument at all. **Its row cited the
/// wrong file**: it named the operand-subset lattice, whose subject is which
/// subsets license an arm, where the finding is about when a refusal fires. The
/// instrument is the binding-time ladder, which compiles a licensed declaration
/// beside the offending one at every rung and so carries the case that had to
/// fail four times over. Re-run and reproduces its committed output exactly.
///
/// **The lesson is the second one.** An uncontrolled standing can mean the
/// control was never built, and it can mean the row is pointing at a file that
/// is not the instrument, and reading the cited file is what tells the two
/// apart.
#[test]
fn no_measurement_rests_on_an_uncontrolled_instrument() {
    const KNOWN: &[&str] = &[];
    let mut found: Vec<String> = shape::measurements_resting_on_an_unusable_instrument(&canon())
        .into_iter()
        .map(|f| f.at)
        .collect();
    found.sort();
    found.dedup();
    assert_eq!(
        found, KNOWN,
        "a claim rests on an instrument that cannot be refuted. Mark the sentence as the \
         argument it is, or build the case that had to fail; adding it here is neither."
    );
}

/// The three ways an instrument cannot be used, and the one way it can.
///
/// This arm exists because the gate beside it was accidentally strong while
/// the `probe` namespace was empty and got weaker the moment somebody filled
/// it: naming a probe became enough. The seat that filled it predicted that
/// before its own run, which is the shape of finding worth acting on rather
/// than filing.
#[test]
fn a_measurement_may_not_rest_on_a_defective_withdrawn_or_uncontrolled_probe() {
    let reg = parse(
        "planted.toml",
        r#"
[[probe]]
id = "the_sound_one"
standing = "sound"
control = "the wrap arm had to disagree with clamp at width 5, and it fired"

[[probe]]
id = "the_defective_one"
standing = "defective"
control = "the arm compared two placements of sixteen; no control caught it"

[[probe]]
id = "the_withdrawn_one"
standing = "withdrawn"
control = "the author retracted it before reporting"

[[probe]]
id = "the_uncontrolled_one"
standing = "uncontrolled"
control = "no control was run"

[[proposal]]
id = "rests_on_the_sound_one"
sentence_kind = "measured"
evidence = ["the_sound_one"]

[[proposal]]
id = "rests_on_the_defective_one"
sentence_kind = "measured"
evidence = ["the_defective_one"]

[[proposal]]
id = "rests_on_the_withdrawn_one"
sentence_kind = "enumeration"
evidence = ["the_withdrawn_one"]

[[proposal]]
id = "rests_on_the_uncontrolled_one"
sentence_kind = "measured"
evidence = ["the_uncontrolled_one"]

[[proposal]]
id = "an_argument_citing_a_defective_one"
sentence_kind = "argument"
evidence = ["the_defective_one"]
"#,
    );
    let found = shape::measurements_resting_on_an_unusable_instrument(&reg);
    let mut at: Vec<&str> = found.iter().map(|f| f.at.as_str()).collect();
    at.sort();
    assert_eq!(
        at,
        [
            "proposal::rests_on_the_defective_one",
            "proposal::rests_on_the_uncontrolled_one",
            "proposal::rests_on_the_withdrawn_one",
        ],
        "the sound one must pass, and an argument citing a defective probe is not a \
         measurement resting on one: {found:#?}"
    );
    assert!(
        found
            .iter()
            .any(|f| f.kind == "measurement-rests-on-an-uncontrolled-instrument"),
        "the uncontrolled case is reported under its own kind, because the fix differs: \
         {found:#?}"
    );
}

/// The value that replaces the guess, and why there is a value at all.
///
/// A word list cannot tell a report from a counterfactual. Two probes in this
/// corpus open their control with the same sentence and the prose matcher
/// catches one, because the other contains `reported` inside a clause about
/// what a different outcome would have meant. That was established by a second
/// reader after the matcher's own doc comment asked for one, and it is not
/// fixable by a longer list.
///
/// So the admission is data. `standing = "uncontrolled"` says it, the arm reads
/// it, and no sentence is parsed. The prose matcher stays only as a backstop
/// for rows not yet triaged onto the value.
#[test]
fn an_uncontrolled_standing_is_read_without_looking_at_the_prose() {
    let reg = parse(
        "planted.toml",
        r#"
[[probe]]
id = "declared_uncontrolled"
standing = "uncontrolled"
control = "None was run as a case that had to fail. Had the arms disagreed it would have been reported, and they did not."

[[probe]]
id = "declared_sound"
standing = "sound"
control = "None was run as a case that had to fail. Had the arms disagreed it would have been reported, and they did not."

[[proposal]]
id = "rests_on_the_declared_one"
sentence_kind = "measured"
evidence = ["declared_uncontrolled"]

[[proposal]]
id = "rests_on_the_undeclared_one"
sentence_kind = "measured"
evidence = ["declared_sound"]
"#,
    );
    let found = shape::measurements_resting_on_an_unusable_instrument(&reg);
    assert_eq!(
        found.len(),
        1,
        "the two probes carry the identical control text and differ only in `standing`, which \
         is the whole point: the prose matcher lets this sentence through and the value does \
         not. If both are reported the matcher has started catching it and this test is \
         measuring something else: {found:#?}"
    );
    assert!(
        found[0].at.contains("rests_on_the_declared_one"),
        "{}",
        found[0].at
    );
    assert_eq!(
        found[0].kind,
        "measurement-rests-on-an-uncontrolled-instrument"
    );
}

// Three tests that exercised a prose matcher used to sit here, one for the
// admissions it had to catch, one for the controls it had to leave alone, and
// one control on the control. They are gone with the function, and what they
// established is kept in the comment where it stood in `shape.rs`: a word list
// caught one admission in five, then six in nine, and failed in both directions
// for reasons no longer list can fix. It cannot tell a report from a
// counterfactual and it cannot see a negation on a different noun.
//
// The test that replaces all three is the one above, which plants two probes
// with **identical** control text differing only in `standing`, and asserts
// that exactly one is reported. That is the whole property in one arm: the
// value decides and the prose does not.

#[test]
fn the_committed_canon_defines_no_term_twice() {
    let found = shape::a_term_defined_twice(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn the_committed_canon_has_no_definition_without_a_term() {
    let found = shape::definitions_with_no_term(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

/// A definition states no region for a different reason than an imposition
/// does, and both must be silent while everything else is not.
#[test]
fn a_definition_carries_no_region_and_neither_does_an_imposition() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_stipulation"
sentence_kind = "definition"
defines = "chain"

[[proposal]]
id = "a_stipulation_with_a_region"
sentence_kind = "definition"
defines = "stretch"
predicate = ["fraction_width: 0"]

[[proposal]]
id = "an_imposed_one"
sentence_kind = "normative"
"#,
    );
    let found = shape::predicate_disagrees_with_the_sentence_kind(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("a_stipulation_with_a_region"),
        "{}",
        found[0].at
    );
    assert!(
        found[0].says.contains("definition"),
        "the report names which of the two kinds it was: {}",
        found[0].says
    );
}

#[test]
fn one_term_stipulated_twice_is_reported_and_a_supersession_is_not() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "the_first_reading"
sentence_kind = "definition"
defines = "chain"

[[proposal]]
id = "a_rival_reading"
sentence_kind = "definition"
defines = "chain"

[[proposal]]
id = "an_older_reading_of_something_else"
sentence_kind = "definition"
defines = "stretch"

[[proposal]]
id = "the_reading_that_replaced_it"
sentence_kind = "definition"
defines = "stretch"
supersedes = ["an_older_reading_of_something_else"]
"#,
    );
    let found = shape::a_term_defined_twice(&reg);
    assert_eq!(
        found.len(),
        1,
        "two live readings of `chain` are a finding; a replaced reading of `stretch` is the \
         mechanism working: {found:#?}"
    );
    assert!(found[0].at.contains("a_rival_reading"), "{}", found[0].at);
}

/// A supersession of something unrelated does not excuse a rival definition.
///
/// The first version of this arm skipped any row carrying a `supersedes` at
/// all, so a definition that replaced some unrelated claim stopped counting as
/// a definition and its rival vanished with it. The arm then read green over a
/// selection that one line had emptied, which is the shape a checker cannot
/// see from inside: it reports nothing and nothing is what a clean corpus
/// reports too.
///
/// It was caught by a control that refused to fire, which is worth more than
/// one that fires.
#[test]
fn superseding_something_unrelated_does_not_hide_a_rival_definition() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "the_first_reading"
sentence_kind = "definition"
defines = "strategy"

[[proposal]]
id = "a_rival_that_replaced_something_else"
sentence_kind = "definition"
defines = "strategy"
supersedes = ["an_unrelated_claim"]

[[proposal]]
id = "an_unrelated_claim"
sentence_kind = "definition"
defines = "container"
"#,
    );
    let found = shape::a_term_defined_twice(&reg);
    assert_eq!(
        found.len(),
        1,
        "`strategy` has two live readings and the second replaces a definition of something \
         else entirely, which excuses nothing: {found:#?}"
    );
    assert!(found[0].says.contains("strategy"), "{}", found[0].says);
}

#[test]
fn a_definition_that_says_nothing_about_what_it_defines_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_stipulation_of_nothing"
sentence_kind = "definition"

[[proposal]]
id = "a_stipulation_of_something"
sentence_kind = "definition"
defines = "chain"
"#,
    );
    let found = shape::definitions_with_no_term(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "definition-names-no-term");
}

#[test]
fn the_committed_canon_leaves_no_row_unfindable() {
    let found = shape::rows_with_no_keywords(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

/// The rulings with nothing verbatim behind them, pinned by name.
///
/// This one does not assert an empty list, because the list is not empty and
/// will not become empty: for every row named here the corpus holds no words of
/// op's, only a record of which option he took. Asserting zero would be a red
/// test nobody can fix, and ignoring it would stop the arm reporting the next
/// one.
///
/// So the known hole is written down and anything else fails. **The rows on it
/// are not equally thin, and the difference is whether the options survived.**
///
/// The first four are the bad kind. Their only record is an agent's sentence
/// reporting the outcome, and the most consequential of them governs when
/// anything becomes canon at all with nothing behind it but "He took the third."
/// The choice he was choosing among is gone, so nobody can reconstruct what he
/// decided, only that he decided.
///
/// The last two are the tolerable kind: he selected from an option list rather
/// than writing prose, and the options as worded are carried on the row's own
/// `ratification` field along with which ones he did not take. A later reader
/// can see the alternatives and judge the call. That is the shape every future
/// option-selected answer should have, and a row reaching this list without it
/// is a defect in the asking rather than a gap in the corpus.
#[test]
fn the_rulings_with_no_verbatim_are_the_ones_the_corpus_has_no_words_for() {
    const KNOWN: &[&str] = &[
        "ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked",
        "ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names",
        "ruling::the_branch_waits_for_the_canon",
        "ruling::the_canon_is_written_once_at_the_end",
        "ruling::the_d_numbered_decisions_are_dead",
        "ruling::the_family_question_wants_the_comparison_first",
    ];
    let mut found: Vec<String> = shape::rulings_with_no_verbatim(&canon())
        .into_iter()
        .map(|f| f.at)
        .collect();
    found.sort();
    assert_eq!(
        found, KNOWN,
        "a ruling claims op's authority with no words of his behind it. Either quote the \
         source, or add it here with the reason the corpus holds none."
    );
}

#[test]
fn a_ruling_with_a_quote_is_not_reported_and_one_without_is() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "his_own_words"
says = "the strategy set is not closed at four"
quote = "the strategy set is not closed at exactly four"

[[ruling]]
id = "somebody_elses_words"
says = "he took the third option"
"#,
    );
    let found = shape::rulings_with_no_verbatim(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("somebody_elses_words"),
        "{}",
        found[0].at
    );
    assert_eq!(found[0].kind, "ruling-carries-no-verbatim");
}

/// A proposal has no `quote` by construction: there are no words but the
/// panel's, which is what `says` holds. Reporting one would be reporting the
/// namespace rather than a defect in a row.
#[test]
fn a_proposal_is_not_asked_for_a_verbatim() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
says = "the partition is derivable without the observability rule"
"#,
    );
    assert!(shape::rulings_with_no_verbatim(&reg).is_empty());
}

#[test]
fn a_refusal_with_no_alternative_is_reported_in_either_namespace() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_refusal"
kind = "refusal"

[[proposal]]
id = "another_refusal"
kind = "refusal"

[[ruling]]
id = "a_refusal_that_answers"
kind = "refusal"
instead = "the overlay stays, and it costs the support this brings"
"#,
    );
    let found = shape::refusals_without_an_instead(&reg);
    assert_eq!(
        found.len(),
        2,
        "both namespaces carry the obligation and the one that meets it is not reported: \
         {found:#?}"
    );
    assert!(found.iter().all(|f| f.kind == "refusal-owes-an-instead"));
}

/// A deferral owes the same sentence for a different reason.
///
/// Op declining to make a call and handing it back is a distinct act from
/// refusing a thing, and it was being recorded as `refusal` because nothing
/// else fitted. A reader of a design would take that as arvo refusing to do
/// something, which is the opposite of what happened. What a deferral owes is
/// who it went back to.
#[test]
fn a_deferral_owes_the_same_sentence_and_says_which_kind_it_was() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "handed_back_with_nothing"
kind = "deferral"

[[ruling]]
id = "handed_back_to_somebody"
kind = "deferral"
instead = "the panel settles it: impl detail, optimal and converged to by experts, iteratively"
"#,
    );
    let found = shape::refusals_without_an_instead(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].says.contains("deferral"),
        "the report names which kind it was, or a reader fixing it writes the wrong \
         sentence: {}",
        found[0].says
    );
}

#[test]
fn a_measurement_with_no_instrument_is_reported_and_an_argument_is_not() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_bare_number"
sentence_kind = "measured"

[[proposal]]
id = "a_reasoned_claim"
sentence_kind = "argument"

[[proposal]]
id = "a_real_measurement"
sentence_kind = "measured"
evidence = ["the_sweep"]
"#,
    );
    let found = shape::measured_without_evidence(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].at.contains("a_bare_number"), "{}", found[0].at);
}

/// A sweep is a measurement whatever the sentence was called.
///
/// This corpus labels its sweeps `enumeration`, which is accurate: somebody
/// walked a bounded set and reported what was in it. Gating only `measured`
/// left the arm with no purchase on most of what the corpus actually ran, and
/// the seat that met the gate reported exactly that.
///
/// A `theorem` owes its route rather than a run, so it stays out, and so do the
/// two kinds that claim no run at all.
#[test]
fn an_enumeration_owes_an_instrument_and_a_theorem_does_not() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_sweep_with_nothing_behind_it"
sentence_kind = "enumeration"

[[proposal]]
id = "a_sweep_that_names_its_run"
sentence_kind = "enumeration"
evidence = ["the_walk"]

[[proposal]]
id = "a_proof"
sentence_kind = "theorem"

[[proposal]]
id = "an_imposed_one"
sentence_kind = "normative"
"#,
    );
    let found = shape::measured_without_evidence(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("a_sweep_with_nothing_behind_it"),
        "{}",
        found[0].at
    );
    assert!(
        found[0].says.contains("enumeration"),
        "the report names the kind, or a reader fixing it looks for the wrong field: {}",
        found[0].says
    );
}

#[test]
fn an_imposed_proposition_carrying_a_region_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_firewall"
sentence_kind = "normative"
predicate = ["fraction_width: 0"]
"#,
    );
    let found = shape::predicate_disagrees_with_the_sentence_kind(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "an-imposed-proposition-carries-a-region");
}

#[test]
fn an_established_claim_with_no_region_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_theorem_with_no_region"
sentence_kind = "theorem"
"#,
    );
    let found = shape::predicate_disagrees_with_the_sentence_kind(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "an-established-claim-carries-no-region");
}

/// The two halves of that arm point opposite ways, so the pair that satisfies
/// both must be silent or the arm is simply reporting everything.
#[test]
fn the_two_correct_shapes_are_both_silent() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "an_imposed_one"
sentence_kind = "normative"

[[proposal]]
id = "an_established_one"
sentence_kind = "theorem"
predicate = ["fraction_width: 0", "threads: 1"]
"#,
    );
    assert!(
        shape::predicate_disagrees_with_the_sentence_kind(&reg).is_empty(),
        "an imposed proposition with no region and an established one with a region are \
         both correct, and an arm reporting either is reporting the rule rather than a \
         breach of it"
    );
}

#[test]
fn a_stamp_from_anything_but_a_ratification_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "an_ack_that_stamps"
rung = "stated"
ratifies = ["some_claim"]

[[ruling]]
id = "a_real_ratification"
rung = "ratified"
ratifies = ["some_other_claim"]

[[ruling]]
id = "an_ack_that_stamps_nothing"
rung = "stated"
"#,
    );
    let found = shape::stamps_from_an_unratified_ruling(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("an_ack_that_stamps"),
        "{}",
        found[0].at
    );
    assert!(found[0].says.contains("stated"), "{}", found[0].says);
}

#[test]
fn a_row_with_no_keywords_is_reported_only_in_the_namespaces_a_reader_searches() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "unfindable"

[[dimension]]
id = "found_by_enumeration"
what = "an axis"

[[ruling]]
id = "findable"
keywords = ["width", "carrier"]
"#,
    );
    let found = shape::rows_with_no_keywords(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].at.contains("unfindable"), "{}", found[0].at);
}
