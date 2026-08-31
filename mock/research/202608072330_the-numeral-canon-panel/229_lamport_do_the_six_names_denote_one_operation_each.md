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

## The answer

Four of the six denote exactly one operation on a signed domain: `floor`, `ceil`, `toward_zero`,
`half_even`. One denotes two: `half_up`. One denotes a family rather than an operation, and is not
a function at all in the sense the other five are: `stochastic`.

So the ratified ruling retired one ambiguous word and left two names in the set it fixed that do
not pin an operation. That is not a defect in the ruling, which settled which names exist and said
nothing about what each denotes, and it is not a reason to reopen it. It is the next question in
the same series, and the ruling's own `because` supplies the test for it.

### `floor`, one operation

Greatest integer not above the value. The name is the mathematical function, pinned in every
standard and every language that ships it, and there is no tie case for a direction to be
ambiguous about. The only competing reading I could find is casual usage where "floor" is meant as
"round down in magnitude", which is `toward_zero`, and I discarded it: that usage appears in
informal writing and in no standard or shipped implementation I could name, so it fails the test I
set. Probe A confirms `floor` and `bit_drop` are the same operation at zero disagreements over
every row swept, which is the ratified note measured rather than assumed.

### `ceil`, one operation

Least integer not below the value. Same argument, mirrored. The competing reading here is "round
up in magnitude", which is `away_from_zero`, and probe A puts them at 120 of 256 disagreeing at
`W = 8, F = 4` signed and 0 unsigned, so they are genuinely different operations. I discarded the
reading for the same reason: `ceil` is toward positive infinity in C, in IEEE 754, and in every
library that uses the spelling. The word is pinned by universal usage in a way "up" is not, which
is the whole of the difference between this name and `half_up`.

### `toward_zero`, one operation

Discard the fraction, keep the sign. This name denotes one operation by construction, because it
is the name the ratified ruling minted precisely so that the direction is stated rather than
inferred. It is what makes the contrast sharp: the ruling fixed the directed family by naming each
direction outright, and the one name in the set that does not name its direction is the one that
broke.

### `half_even`, one operation

Nearest, with a tie going to the neighbour whose multiplier is even. I looked hard for a second
reading and found none that survives the test.

Evenness is not directional, which is the structural reason. Every other tie rule has to answer
"which way", and "which way" is what admits two answers on a signed domain; "which is even" admits
one, and the answer does not move when the sign flips, because `n` and `-n` have the same parity.
Probe C measures it: `half_even` is odd, at zero conjugacy failures across `F` in 1 to 4 at
`W = 8` signed, so the operation commutes with negation and there is no signed-domain seam for a
second reading to hide in.

Three candidate second readings, all discarded, with the reason for each. Ties to odd is a real
operation, von Neumann's, and it is a different name. Evenness judged on the pre-rounding
truncated value rather than the post-rounding one gives the identical function, which I checked by
implementing the rule both ways inside probe A's shared definitions. Evenness relative to a
coarser target grid than the last representable bit is a question about which grid is being
rounded to, and the target grid is a parameter of the operation rather than part of the name; every
mode in the set has that parameter and none of them is ambiguous because of it.

One caveat I want stated rather than buried. `half_even` on a decimal format is the same rule
applied to a decimal grid, and I measured only at `radix = 2`. The predicate below says `radix = 2`
and therefore claims nothing about decimal, per the notation.
