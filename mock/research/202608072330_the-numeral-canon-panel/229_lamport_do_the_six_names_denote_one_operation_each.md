# 229. Do the six names denote one operation each

Seat 229, cold derivation, written from the registry and from outside sources with the panel
directory unopened. The probes are in `229_probes/` and were committed before this file was
started.

## The gates

**Canon gate: aligned.** I checked the assigned work against
`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`, which is `ratified` and
which fixes the vocabulary at the six names the question is about, and against
`dimension::rounding`, `dimension::signedness`, `topic::rounding`,
`law::rounding_retraction_is_the_identity`, `law::quantise_then_reduce_commutes`,
`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`,
`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` and
`probe::the_two_toward_zero_spellings_differ_and_by_how_much`. Asking what each name denotes is
not a question the ratified ruling closed. It closed which names exist, and it is defended here
rather than weighed: nothing below proposes adding, removing or renaming any of the six.

**Two rows are open and I have not filled either.**
`question::is_the_rounding_vocabulary_complete_at_six` reserves whether the set is short a name,
and `question::what_region_does_a_predicate_naming_no_mode_state` reserves what a predicate naming
no mode states. My findings touch the neighbourhood of both. Where they do I say so and stop.

**Test gate.** Seven probes, each carrying the case that must fail, stated in the source before
the sweep and asserted after it. Two of them reproduce numbers the canon already records, which
is the only reason to believe the rest of what they print. All seven print `instrument: sound`
and exit non-zero if they do not. No probe is cited here that is not committed with its output.

## The question, as I took it

A name denotes exactly one operation when, given a value and a target grid, the name determines
the result. It denotes more than one when two readings of it are each defensible and disagree on
some input the format can hold.

Three things separate a real second reading from one I could invent. The reading has to be
written down in a standard, or shipped in an implementation people use, or forced by a hardware
realisation somebody actually builds. A reading that is merely conceivable does not count, and I
have thrown several away on that test.

The domain matters and the question fixes it: signed. Every name in the set behaves identically
on the non-negative half of any format, and that is exactly what makes this worth asking, because
a reading that is wrong only on negatives is a reading nothing catches until a negative arrives.
That is the shape the ratified ruling already named once, in its own words: bit-drop "differs
from toward-zero on signed rows only, so a reader coming from the hardware and a reader coming
from C would have understood the same word as two operations that genuinely differ".

The value model throughout is a scaled integer `k` denoting `k / 2^F`, rounded to an integer, with
`k` ranging over the whole representable domain of a `W`-bit format. Where a mode is randomised the
output is a distribution and the comparison is of distributions, computed by enumerating the entire
draw space rather than by sampling, so no random number generator appears anywhere in this work and
every number reproduces to the digit.
