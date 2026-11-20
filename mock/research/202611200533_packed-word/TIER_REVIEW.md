# The bit tier's review, before it lands with the packed word

The bit tier (`work/arvo-bits-tier`, `d15dd473..1fd4d78e`) lands on `dev` together with the packed word,
which was cut from it. The packed word's three review passes read its diff against the tier, so the tier's
own commits got one pass of their own, outside that cap, at the coordinator's word.

The verdict was merge after fixes. The canon gate passed against
`ruling::never_a_runtime_check_and_one_lowered_path`,
`ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`,
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`,
`obligation::an_exact_width_container_a_consumer_can_alias_and_pin` and
`obligation::set_operations_over_a_fixed_size_bit_set`, all live at `1fd4d78e`. Every test body was read and
none is a tautology; the twelve mutant flags of
`mock/research/sketches/202611200415_bit-tier-mutants-after-second-review/` are each shown red. The
rotations were checked by hand at every width, `Within` has one maker and derives nothing, and the
lowering claim carries its predicate and its committed disassembly.

## What was fixed, and what was left

The crate readme stood at sixteen lines against the three to ten of `readmes.md`. Round `202611200711-kg12`
cut it to its purpose, its dependencies and its dependents.

`docs/STRUCTURE.md` lists only `Bits` under the crate's public items, because the generator reads a public
item only where it is declared and every other item of the crate is a re-export. That is the generator's
defect, not the branch's, and it is accepted at landing against the ticket that already tracks it,
`arvo_docs_structure_md_lists_none_of_a_crate_s_re_exported_p`, bumped with this case.

Left as follow-ups:

- `tests/the_bit_tier_in_a_constant.rs` runs only `and`, `is_empty`, `bit` and `rotate_up` in a constant,
  the sealing and own-container refusals have no `BitSequence` twin, and the src changelist of round
  `202611200402` names the wrong job. Task
  `arvo_bits_tier_tests_leave_the_const_law_and_sequence_refusa`.
- A consumer evaluating the tier generically in a constant enables `const_trait_impl`, and one naming
  `Bits` or implementing the tier without `const` needs no gate. Whether that meets
  `obligation::the_unstable_machinery_does_not_reach_a_consumer` is the question already open,
  `does_implementing_an_arvo_trait_count_as_naming_arvo_s_types`, bumped with this case.
