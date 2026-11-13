# The catalogue's tracked gaps

Every `#[ignore]` in this repository is a catalogue entry, and its reason ends `tracked #<id>`. The
`catalogue-ignore` lint resolves each id to a row of this file, so a row is the one place an ignored
test says which ticket of the workspace tracker holds its gap. The ticket carries the gap in full; a
row names it in a line. A gap that closes takes its row with it, in the same change that removes the
last ignore citing it.

- arvo_placement_ships_one_packing_rule_so_the_test_that_the_c: `arvo-placement` ships one packing
  rule, so the test that the carrier is not a function of the access width stays red.
- arvo_s_half_up_reader_misses_a_motion_gerund_outside_motions: the `half_up` reader reads a motion
  gerund `MOTIONS` does not hold as naming `toward zero`, so the clause names another rule and is let
  through.
- arvo_s_half_up_reader_reads_a_bare_number_before_a_verb_as_a: the `half_up` reader reads a bare
  number before a verb as a seat, so a count of ties takes the clause from the mode.
- arvo_s_half_up_reader_reads_a_bare_number_before_or_or_in_as: the `half_up` reader reads a seat
  number before `or` or `in` as a count, so the clause carries the mode and is refused.
- arvo_s_half_up_reader_reads_a_capitalised_method_name_as_the: the `half_up` reader reads a
  capitalised method name opening a clause as the method before a word of an open class, so an
  imperative on such an object is let through.
- arvo_s_half_up_reader_reads_a_gerund_naming_toward_zero_befo: the `half_up` reader gives a relative
  to a gerund naming `toward zero` even where the clause says the mode is computed that way.
- arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi: how the `half_up` reader resolves a
  pronoun is one open call: one opening a clause or a segment after a rule the mode is denied to be
  takes that rule, `it` later in a clause opening on a subject is the mode, and a personal pronoun in
  an alias's segment keeps the alias a reading. Six catalogue entries contradict a reading of it.
- arvo_s_half_up_reader_reads_any_copula_before_a_negator_as_a: the `half_up` reader reads a copula
  between the reading and a negator as a denied identity even where it predicates a quality.
- arvo_s_half_up_reader_refuses_a_clause_whose_subject_is_name: the `half_up` reader refuses a clause
  whose subject is named in words outside its nouns for a subject, since the mode carries into it.
- arvo_s_half_up_reader_takes_a_rule_named_only_by_a_hyphenate: the `half_up` reader reads a rule
  named by a hyphenated phrase alone, behind a negator, as a quality, so the relative after it is the
  mode's and the clause is refused.
- crate_root_paths_reads_a_use_alias_in_one_module_as_the_crat: `crate_root_paths` reads aliases by name over
  the whole file, not by what they point to or the module they sit in, so one module's `use std as s`
  reads as the crate in another, and `extern crate self as std` makes a later `std::x()` a false hit.
