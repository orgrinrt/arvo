# The catalogue's tracked gaps

Every `#[ignore]` in this repository is a catalogue entry, and its reason ends `tracked #<id>`. The
`catalogue-ignore` lint resolves each id to a row of this file, so a row is the one place an ignored
test says which ticket of the workspace tracker holds its gap. The ticket carries the gap in full; a
row names it in a line. A gap that closes takes its row with it, in the same change that removes the
last ignore citing it.

- arvo_placement_ships_one_packing_rule_so_the_test_that_the_c: `arvo-placement` ships one packing
  rule, so the test that the carrier is not a function of the access width stays red.
- arvo_s_a_lint_guards_a_crate_that_exists_reads_a_commented_o: `a-lint-guards-a-crate-that-exists`
  reads a commented-out gate as a gate.
- arvo_s_a_standing_is_reachable_lint_counts_a_member_file_and: `a-standing-is-reachable-from-what-it-cites`
  counts a member file and the consolidation over it as two arrivals.
- arvo_s_half_up_reader_gets_eighteen_catalogued_sentence_pair: the `half_up` reader gets eighteen
  catalogued sentence pairs wrong on one named side.
- arvo_s_line_scanning_enforcer_lints_read_a_trailing_comment: the line-scanning enforcer lints read a
  trailing `//` comment on a line of code as code.
- arvo_s_line_scanning_enforcer_lints_flag_a_forbidden_name_in: the line-scanning enforcer lints flag a
  forbidden name inside a string literal.
- arvo_s_line_scanning_enforcer_lints_flag_a_forbidden_name_on: the line-scanning enforcer lints flag a
  forbidden name on an interior line of a block comment.
- arvo_s_no_alloc_enforcer_misses_a_type_inferred_heap_constru: the no-alloc enforcer misses a
  type-inferred heap construction with no generic annotation.
- arvo_s_no_std_enforcer_misses_a_fully_qualified_std_path_use: the no-std enforcer misses a fully
  qualified `std::` path used outside a `use` statement.
- arvo_s_write_hook_pairs_lifetime_ticks_as_a_char_literal_and: the write hook pairs lifetime ticks as a
  char literal and lets a bare primitive through.
- arvo_s_write_hook_never_reaches_a_bare_primitive_written_as: the write hook never reaches a bare
  primitive written as a literal suffix.
