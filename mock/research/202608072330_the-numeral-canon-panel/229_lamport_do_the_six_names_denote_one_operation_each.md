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

## The instruments, and what each one covers

Seven probes, all in `229_probes/`, all standalone `rustc` compilations over a shared `modes.rs`
so the operations are defined once and every probe measures the same functions.

`a_pairwise_disagreement.rs` counts, for every pair of candidate operations, the values of the
domain on which they disagree. It is the instrument that answers the question directly. Its two
positive controls are the load-bearing ones: it reproduces
`probe::the_two_toward_zero_spellings_differ_and_by_how_much` digit for digit at `W = 8`, 64 at
`F = 1`, 96 at `F = 2`, 120 at `F = 4` and 127 at `F = 7`, and it confirms the ratified note that
bit-drop equals floor, at zero disagreements over every width, fraction width and signedness it
sweeps. An instrument that failed either of those would be measuring something other than what the
canon measured, and nothing else it printed would be worth reading.

`b_translation_equivariance.rs` asks which operations satisfy `R(x + t) = R(x) + t` for integer
`t`. It exists because two canon law rows count rounding positions by exactly that property, so it
is the one place where the canon's own recorded numbers discriminate between two readings of a
name. Its must-fail case is `half_even`, which
`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned` puts alone in its failing region;
an instrument reporting `half_even` equivariant would contradict a canon row and be void.

`c_negation_conjugacy.rs` asks which operations are odd, and for the ones that are not, which
other operation is the mirror. It found an error in my own comment while I was writing it: I had
written that `away_from_zero` is `toward_zero`'s conjugate under negation, and it is not, because
`toward_zero` is odd and therefore its own. The comment is corrected in the committed source. The
instrument disagreeing with its author is the only reason that got fixed.

`d_stochastic_readings.rs` computes exact output distributions for five readings of `stochastic`
by enumerating the whole draw space. It reports per-value bias as an exact rational and whether
each reading retracts, which is `law::rounding_retraction_is_the_identity` applied to a randomised
mode. Its must-agree control is that the hardware realisation and the proportional definition
produce identical distributions on every value; its must-differ control is that the
equal-probability reading differs from the proportional one somewhere.

`e_mean_error.rs` sums `R(x) - x` over the whole domain, exactly, as a rational. This is the axis
on which the two readings of `half_up` come apart in the opposite direction from probe B, which is
what makes the choice between them a trade rather than a spelling.

`f_every_tie_rule.rs` enumerates the entire space of nearest modes over a domain, one per
assignment of up or down to each tie, at four `(W, F)` points, and asks how many are translation
equivariant, how many have zero mean error, and how many are both. This is the probe that turns
"the two readings differ" into a statement about every possible reading rather than the two that
happen to be current. Its controls are that each property is satisfiable alone, so an empty
intersection is a result rather than a test that cannot pass.

`g_stochastic_equivariance.rs` asks the same two questions of the randomised readings, with
equivariance taken of the whole distribution.

**What none of them covers.** Everything here is one rounding applied to one value. No chain, no
composition with an arithmetic operation, no range reduction, no container edge. Where a claim of
mine touches a canon law about fusion I am reasoning from the property that law is stated over,
not re-measuring the law. The widths swept are `W` in `{4, 6, 8, 10, 12}` and never beyond, on one
thread, on one host, under one toolchain, and the predicates below say so rather than implying
otherwise.
