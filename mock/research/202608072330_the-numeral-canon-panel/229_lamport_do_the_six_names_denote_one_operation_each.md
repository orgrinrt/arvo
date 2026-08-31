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

### `half_up`, two operations

Nearest, with a tie going up. "Up" is the whole problem, because on a signed domain it means two
different things and both are shipped.

**Reading one, ties toward positive infinity.** `floor(x + 1/2)`. This is what `java.lang.Math.round`
computes, and it is what hardware and DSP practice mean by round-half-up, because adding a half and
truncating needs no comparison and is therefore the cheap form. The DSP literature calls it the
asymmetric form and says so in those words.

**Reading two, ties away from zero.** This is IEEE 754's `roundTiesToAway`, `java.math.RoundingMode.HALF_UP`,
`decimal.ROUND_HALF_UP`, and `MidpointRounding.AwayFromZero`. It is what school arithmetic and
commercial rounding mean.

The two agree everywhere except at a negative tie, where reading one gives the neighbour toward
zero and reading two gives the neighbour away from it. Probe A counts the disagreement over the
whole domain and finds `2^(W-1-F)` values at every width and fraction width it sweeps, which is 64
of 256 at `W = 8, F = 1`, 8 of 256 at `W = 8, F = 4`, and 1024 of 4096 at `W = 12, F = 1`. On an
unsigned domain the count is 0 everywhere, and at `F = 0` it is 0 because no tie exists.

That last sentence is the ratified ruling's own criterion, met exactly. The retired word named two
operations that agree on unsigned rows and differ on signed ones. Probe A prints every pair with
that property at `W = 8, F = 4`, and there are four: floor against toward-zero, ceil against
away-from-zero, toward-zero against bit-drop, and the two readings of `half_up` at 8 of 256. Three
of the four are the retirement the ruling already made. The fourth is a name the ruling kept.

**This is not a criticism of the ruling and nothing here reopens it.** The ruling closed which names
exist and attached a note pinning a hardware operation to one of them, which is precisely the
instrument this finding calls for: a note per name saying which operation it denotes. What is
missing is one sentence, not a seventh name and not a rename.

### `stochastic`, a family, and not a function

The other five names denote functions. This one cannot, because two invocations on the same input
are permitted to differ, so the object it names is a distribution rather than a value. Every
comparison below is therefore of distributions, and probe D computes them exactly by enumerating
the whole draw space.

Four readings, all real, and they are not interchangeable.

**Proportional.** Round up with probability equal to the discarded fraction. Parker's definition,
the one the numerical literature builds on, and unbiased at every value: probe D reports a worst
per-value bias of exactly `0/1` over the whole domain at `W = 8, F = 2`.

**Equal probability.** Round up or down with probability one half whenever the value is off the
grid. This is a named mode in the current literature, called SR-up-or-down where the proportional
one is called SR-nearness, and it is not the same operation: probe D finds the two distributions
differing on 128 of 256 values at `W = 8, F = 2`, with a worst per-value bias of `4/16`. Its
aggregate bias over a uniformly swept domain is zero, which probe G measures, and that is a
different and weaker property than being unbiased at each value. Conflating the two is easy and I
nearly did.

**Add a random draw, then drop the bits.** The hardware realisation. Probe D's control is that this
must produce the proportional distribution exactly, and it does, on every value of every row swept,
which is what licenses treating the hardware form and the mathematical definition as one operation.

**Add a random draw, then round toward zero.** The same realisation with the other reading of the
word the canon retired, and it is a badly different operation. At `W = 8, F = 2` it returns 0 with
certainty for every value in the open interval between minus one and zero: probe D's worked table
shows `-3/4`, `-2/4` and `-1/4` all mapping to `0` with weight `4/4`. Its worst per-value bias is
`12/16` against the proportional reading's `0/1`.

**The canon already excludes that fourth reading, and I did not have to decide it.**
`law::rounding_retraction_is_the_identity` says a value already on the grid comes back unchanged.
Probe D checks retraction for all five readings and finds add-then-toward-zero failing at 64 signed
on-grid values at `W = 8, F = 1`, 32 at `F = 2`, 16 at `F = 3` and 8 at `F = 4`, and failing at
none of the unsigned ones. Every other reading retracts everywhere. So a canon law, stated for a
different purpose, rules out one of the four readings on the signed domain and leaves three.

**A fifth axis, not a reading but a parameter.** How wide the random draw is. Probe D models a draw
one bit narrower than the discarded field and gets a distribution that is neither the proportional
one nor the equal-probability one, with an aggregate bias of `-512` where the proportional reading
has `0`. Any implementation whose draw is narrower than the field it is choosing within is
approximating the proportional definition rather than computing it, and the name says nothing about
which.

**And the seed.** Without one, `stochastic` does not denote a repeatable computation at all, so
equality of two results under this name is not even a well-formed question.
`question::does_a_consumer_supplied_seed_surface_exist` has that reserved, with the call recorded
as the coordinator's and explicitly overturnable, and I have not touched it. What I can say is that
the reservation is load-bearing for this question rather than adjacent to it: until it is settled,
`stochastic` names something whose observable behaviour is not determined by the canon.

## Why `half_up` is the one that breaks, structurally

A nearest mode is two things composed: take the nearer neighbour, and where the two are equally
near, apply a tie rule. The tie rule is itself a directional choice between two neighbours, so a
nearest mode's name has to pin a direction exactly as a directed mode's name does.

The vocabulary pins three directions by naming them outright. `floor` is toward negative infinity,
`ceil` is toward positive infinity, `toward_zero` says where it goes in the name. Those three words
are unambiguous because each of them denotes its direction in every standard that uses it.

`half_up` uses a fourth directional word, "up", and the set defines it nowhere. Reading it as
"up the number line" gives toward positive infinity; reading it as "up in magnitude" gives away
from zero. Both readings are ordinary English and both are shipped. The set contains no mode named
`up`, so there is no other row in the vocabulary a reader can resolve the word against.

That is the whole mechanism, and it says why exactly one name broke rather than several. The
ratified ruling retired an ambiguous directional word and replaced it with directions named
explicitly. It did that for the directed family. The nearest family kept a compound name whose
directional half was never given the same treatment.

Probe C makes the same point from the other side, by asking what each mode becomes under negation.
`floor` and `ceil` are each other's mirror and both are in the set. `toward_zero` and `half_even`
are their own mirrors. `half_up` read as away-from-zero is its own mirror. `half_up` read as
toward-positive-infinity has a mirror that is not in the set at all, the mode whose ties go toward
negative infinity, so under that reading the vocabulary cannot state the mirror of a claim it can
state.

## The canon already depends on one of the two readings, without saying so

This is the part that makes the ambiguity load-bearing rather than tidy-up work.

`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` holds for
`rounding: in {floor, ceiling, nearest-half-up}` and fails for
`rounding: in {toward zero, away from zero, nearest-half-even}`, at `signedness: signed`,
`W = 6`, `F in 1..=5`, `overflow_policy: wrap`. Its unsigned twin holds for five of the six
positions and fails for `nearest-half-even` alone. Both rows count positions by translation
equivariance, which the proposal behind them states outright.

Probe B measures equivariance for every candidate and reproduces both counts, but only under one
reading. With `half_up` read as ties toward positive infinity: unsigned gives five of six
equivariant, naming floor, ceil, toward-zero, away-from-zero and half-up, and signed gives three of
six, naming floor, ceil and half-up. Both match the canon rows exactly, set for set. With `half_up`
read as ties away from zero: unsigned still gives five of six, and signed gives **two** of six,
naming only floor and ceil, which contradicts the recorded `holds` region.

So the canon carries a ratified-adjacent law row whose truth value depends on which operation
`half_up` denotes, and nothing in the canon says which. Under one reading the row is right as
written. Under the other it is wrong, and `nearest-half-up` belongs in its failing region.

I want to be exact about the status of that. It is not a claim that the row is wrong. It is a claim
that the row is not decidable from the canon as it stands, because a term it quantifies over has
two denotations and the canon fixes neither. The measurement says which denotation makes the row
true, which is evidence about what the instrument behind that row must have computed, and evidence
is not a ruling.

## The trade between the two readings is forced

Having found that the two readings differ, the obvious next question is whether one is simply
better, in which case a note picking it would cost nothing. It is not, and the reason is sharper
than a preference.

Probe E sums the error over the whole domain, exactly. On a signed domain, reading one carries a
bias of exactly half an LSB per tie and reading two carries none: at `W = 8, F = 2` the sums are
`128/4` against `0/4`, and the pattern holds at every width and fraction width swept, with the
biased column tracking the tie count and the unbiased column staying at zero. On an unsigned domain
both readings carry the identical bias, which is probe E's negative control on the claim that they
differ at all.

So the two readings sit on opposite sides of two different properties, in opposite directions.
Reading one is translation equivariant, which is what makes multiply-add fusion free under the
canon's own law row, and it is biased. Reading two is unbiased, which is what a numeric library
wants, and it is not equivariant, so the free-fusion licence is unavailable under it.

Probe F asks whether some third tie rule escapes. A nearest mode over a finite domain is fully
determined by one bit per tie, so the space is finite and can be enumerated whole. At four points,
`(W, F)` of `(6, 2)`, `(6, 3)`, `(8, 4)` and `(8, 5)`, it enumerates every nearest mode over the
domain, 65536 of them at the first, and finds exactly two translation equivariant, 12870 with zero
mean error, and **zero** in both sets. The two equivariant rules are all-ties-up and all-ties-down,
which is the argument's prediction: integer translation acts transitively on the ties, so an
equivariant tie rule has to be constant, and a constant rule cannot balance.

The controls are what make that emptiness a result. Each property is satisfiable alone at every
point, and each of the four named modes is located inside the enumeration and reconstructed against
its independent implementation before any of it is counted.

**So the trade is a theorem over these domains, not an accident of which two readings happen to be
current.** No deterministic nearest mode is both. Choosing a reading for `half_up` is choosing a
side, and the choice cannot be optimised away.

## The escape is the sixth name

Probe G asks the same two questions of the randomised readings, with equivariance taken of the
whole output distribution. The proportional reading is equivariant at zero failures across `F` in 1
to 4 at `W = 8` signed, unbiased at every value, and retracts. No deterministic mode can be
pointwise unbiased at all, since a deterministic map is pointwise unbiased only where it is the
identity, so this is not a close call.

That gives the composition rather than a winner, and it is per region rather than global.

Where a design needs the free relocation, `half_up` read as ties toward positive infinity is the
only nearest mode that gives it, and it pays half an LSB per tie of bias.

Where a design needs zero mean error and can pay for relocation, `half_up` read as ties away from
zero and `half_even` both give it, and probe E puts them at the identical aggregate sum on the
symmetric domain, so the choice between those two is on other grounds. `half_even` is additionally
its own mirror and additionally unbiased on unsigned, where both readings of `half_up` carry the
same nonzero bias, which is the ordinary reason to prefer it.

Where a design needs both, no deterministic mode exists, and `stochastic` under its proportional
reading is the only thing in the vocabulary that supplies it. That is a real answer to why the
sixth name is in the set, arrived at from the mathematics rather than from fashion.
