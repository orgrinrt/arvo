# Classification of the twelve hits

The census in `output.txt` is exhaustive over 54 governing rows (31 ratified
rulings, 23 two-expert proposals) and the pattern is broad on purpose. The
classification below is mine, by reading, and it is over the complete list rather
than a sample, so it can be checked row by row.

The question it answers: does any governing sentence have a **candidate** as its
subject and read a reduction, an adaptation, a verdict, an encoding or a law
inventory? That sentence would refute the claim that what admission asks a
candidate to expose is fixed by the ratified identity clause.

| row | subject of the sentence | reads something outside identity? | refutes? |
|---|---|---|---|
| `adaptation_is_conditional_on_proof_and_on_soundness` | arvo's multi-threadability | no. The hit is on the **id**, not the statement, which says "multi-threadable" and nothing else | no |
| `the_format_spine_is_canon` | the format concept | its adaptation clause is about **arithmetic on** a format, and its identity clause puts adaptation and encoding outside identity by name | no, it is the claim's own source |
| `behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` | an operation, and the footprint observation | "arithmetic and encoding are stated over the declared width" is about where behaviour is stated | no |
| `the_additive_and_absorption_verdicts_are_canon` | a reduction's induced operation | yes, a verdict, and its subject is the reduction | no |
| `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` | arithmetic on a format, i.e. an operation | yes, the adaptation, whose subject is the operation | no |
| `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | a format | states the exclusion outright | no, it is the claim's own source |
| `membership_of_the_representable_set_is_one_affine_predicate` | membership | mentions the identity adaptation as a **consequence** of the phase coordinate | no, and it is an instance of the claim: a coordinate is exposed, an adaptation fact is derived |
| `absorption_decides_associativity_of_a_clamped_reduction` | a reduction's induced operation | yes, subject is the reduction | no |
| `the_model_band_transfer_is_defeated_in_both_fragments` | a law's verdicts across widths | methodological | no |
| `inside_a_fragment_with_a_complete_test_set_the_verdict_is_computed_at_the_shipped_width` | a law's verdict | yes, and it says the verdict is **computed** | no, it supports the claim |
| `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit` | an accumulator width for a fold | no | no |
| `the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion` | an accumulator width for a fold | no | no |

**Zero of twelve refutes.** Every hit has as its subject an operation, a
reduction, a law, an accumulator, the concept itself, or something unrelated. Not
one has a candidate as its subject while reading a reduction, an adaptation, a
verdict or a law inventory.

**One false positive is worth naming**, because it is a defect in the instrument
rather than a result: the first row matches on its `id` rather than on its
`says`, since the extractor emits `id<TAB>says` on one line. It does not change
the count of genuine hits, and a reader tightening the instrument should anchor
the pattern to the `says` field.

**The control fires**: a planted row carrying exactly the refuting sentence is
extracted and flagged by the same pipeline, so the zero above is a fact about the
corpus rather than about the grep.
