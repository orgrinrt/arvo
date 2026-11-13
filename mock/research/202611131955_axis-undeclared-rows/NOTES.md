# Rows saying an axis is undeclared, read against the dimensions declared now

The tool `mock/tools/axis-undeclared-rows` listed nineteen sentences on sixteen
rows (`output.txt`, over 1924 sentences and 25 declared dimensions). Each row
was read against the dimension list as it stands and sorted into the four
classes `CHECK.md` section 1.10 used: still undeclared, declared since, a dated
account that is true as history, and not an instance. Nothing in the registry
was edited by this job, which holds `mock/lints` and `mock/tools`; the rows in
the declared-since class are for the registry lane.

## Still undeclared

- `law::multiplicative_associativity_under_saturation`, first `gap`: the
  geometry of the representable range. `operand_window` restricts the operands
  to a sub-range and is not the range's geometry.
- `proposal::the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy`:
  same axis, the geometry the sign entry conflates into `signedness`.
- `proposal::a_law_verdict_is_invariant_under_change_of_encoding_and_container`,
  first `note`: an encoding axis. `container` is the machine storage and is not
  an encoding of the value.
- `proposal::a_strategy_is_an_assignment_and_a_weighting`: the assignment set.
- `proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`:
  the intermediate fraction width of a staged narrowing. `accumulator_width` is
  the nearest declared axis and counts the accumulator's width, where this wants
  the fraction bits of the intermediate, so it is not the same axis.
- `ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family`:
  the compilation target's pointer width. `target_features` is the nearest and
  names features, not a width. `dimension.toml` carries no pointer-width word.

## Declared since, and the sentence still says otherwise

- `proposal::at_shared_occupancy_no_per_element_footprint_observation_exists`:
  "the axis this really wants is occupancy ... no dimension row declares one".
  `dimension::occupancy` is that axis. The sentence is stale.
- `proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`,
  second `note`: `operand window in {full, declared non-negative}` dropped
  because no row declares it. `dimension::operand_window` does. The region can
  now be written, so the sentence is stale.
- `law::the_fused_and_the_stepwise_multiply_add_denote_one_function`: the
  clause was left out because no row declared an accumulator width and an
  operation set. Both are declared. The "when this row was written" clause is
  true as history, and the consequence, that the clause is not in the canon at
  all, is the part to revisit.
- `proposal::fusing_a_multiply_add_is_free_exactly_at_translation_equivariance`:
  "the domain the cell reaches" has no axis. `dimension::ambient_domain` is the
  domain the values are taken to live in. Probably the same axis; the row's
  author should confirm before the sentence changes.

## Dated account, true as history

- `law::multiplicative_associativity_under_saturation`, second `gap`, and
  `law::quantise_then_reduce_commutes`: "no `dimension` row declared an axis for
  the ambient domain when this row was written". Past tense, dated, and true.
- `law::additive_associativity_recovered_under_declared_nonnegative_operand_window`
  and `law::distributivity_recovered_under_declared_nonnegative_operand_window`:
  both say the axis was missing and that `operand_window` now declares it.
- `dimension::occupancy`: reports what the container derivation said at the time.
- `proposal::the_realisation_map_has_an_algebraic_character`, `note`: "declared
  such an axis when this row was written".

## Cannot tell without reading the clauses

- `proposal::the_realisation_map_has_an_algebraic_character`, `gap`: "the
  topic's ten remaining clauses all carry regions naming axes the registry does
  not declare". Whether each of the ten is now expressible needs each clause
  read against `ambient_domain`, `association` and `leaf_aliasing`, which were
  declared after the sentence was written.

## Not an instance

- `proposal::a_law_is_inherited_...`, first `note` ("minus what is not an
  axis"): instrument parameters, not a missing axis.
- `proposal::a_law_verdict_is_invariant_...`, second `note`: a general
  statement about the notation.
- `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`: about a
  predicate packing an undeclared key inside a slug, which is a spelling fault.

## What the tool does not find

It reads `gap` and `note`, on the twelve phrasings in `CUES`. A row saying the
same in `answered`, `because` or `says`, or in a phrasing outside the list, is
invisible to it. The first hand search found twenty-six sentences, fourteen of
them instances; this finds nineteen over the same two fields, which is a
difference the other fields and the amendments since account for in part, and
the two lists were not reconciled sentence by sentence.
