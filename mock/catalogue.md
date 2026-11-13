# The catalogue's tracked gaps

Every `#[ignore]` in this repository is a catalogue entry, and its reason ends `tracked #<id>`. The
`catalogue-ignore` lint resolves each id to a row of this file, so a row is the one place an ignored
test says which ticket of the workspace tracker holds its gap. The ticket carries the gap in full; a
row names it in a line. A gap that closes takes its row with it, in the same change that removes the
last ignore citing it.

- arvo_placement_ships_one_packing_rule_so_the_test_that_the_c: `arvo-placement` ships one packing
  rule, so the test that the carrier is not a function of the access width stays red.
- arvo_s_half_up_reader_reads_a_capitalised_method_name_as_the: the `half_up` reader reads a
  capitalised method name opening a clause as the method before a word of an open class, so an
  imperative on such an object is let through.
- arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi: the `half_up` reader reads a pronoun
  after a rule the mode is denied to be, across a full stop and inside one clause, as the rule standing
  last, which a regression arm asserts and two catalogue entries contradict.
- arvo_s_half_up_reader_refuses_a_clause_whose_subject_is_name: the `half_up` reader refuses a clause
  whose subject is named in words outside its nouns for a subject, since the mode carries into it.
- crate_root_paths_reads_a_use_alias_in_one_module_as_the_crat: `crate_root_paths` reads aliases by name over
  the whole file, not by what they point to or the module they sit in, so one module's `use std as s`
  reads as the crate in another, and `extern crate self as std` makes a later `std::x()` a false hit.
