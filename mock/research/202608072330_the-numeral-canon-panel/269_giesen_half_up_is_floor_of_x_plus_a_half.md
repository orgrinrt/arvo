# 269. Which operation `half_up` denotes: an audit of 229 and 267, and an answer

Seat 269, an audit seat over seats 229 and 267 on `question::which_operation_half_up_denotes`. I read
both seats, re-ran every probe they committed, read the registry rows they cite and several they do
not, read op's words at `.data/op-responses/202608311735_arvo-canon-four-questions.md` whole, and
wrote three probes of my own, committed under `269_probes/` before this file.

The short version is that the two seats agree on every measured fact and neither of them answered the
question. Seat 229 established that the word is ambiguous in the world and found, in its second phase,
that this panel had only ever used it one way; it declined to say which way the canon should read it.
Seat 267 answered a different question, whether the vocabulary needs a seventh name, and got there by
quoting the first sentence of a stated row whose second sentence forbids the conclusion. What neither
seat did was follow the ratified vocabulary back to the file it was lifted from, where `half_up` was
minted with a definition, and ask whether the standards bound needs a name at all when the other
reading is two operations away from the names the set already has. Both of those settle most of this.

My answer is that `half_up` denotes one operation, nearest with ties toward positive infinity, which is
`floor(x + 1/2)`. Ties away from zero is a legitimate operation with a real region of its own, and it
is reached as an alias over `toward_zero` or `floor` plus one exact addition, which p270a checks over
every value of every container from two to sixteen bits. The shipped `Mode::HalfUp` does the other
thing, and it acquired that reading in source five hours after the vocabulary was ratified, with no
design sentence under it.

## The gates

Canon gate: aligned for the question of what the name denotes, misposed in the question's third
option. Checked against `ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`
(ratified by op), `ruling::the_panel_finishes_the_canon_without_him` (ratified by op),
`ruling::the_standard_is_parity_in_output_not_in_the_internals` and
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` (both `rung = "stated"`, op's own
words in `quote`), `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
and `question::the_rounding_mode_vocabulary`.

The denotation question is not answered by any stamp. The ratified ruling's `says` lists six names and
defines none of them. It is delegated: `ruling::the_panel_finishes_the_canon_without_him` puts every
remaining canon question with the panel, to be "derived from what he has already said" and "put
through two independent agreements", and the question row carries `decider = "panel"`. So deriving an
answer is the licensed work and I did it.

The third option, "two names, one per reading", is not an answer to this question. It is an answer to
`question::is_the_rounding_vocabulary_complete_at_six`, which asks whether the set is short a name, and
it widens a set an op-ratified ruling fixed. It also rests, in the only file that argues for it, on the
reading of the standards bound that op corrected in so many words (section on seat 267 below). I have
not weighed it here. Whether the vocabulary is complete belongs to its own row, and nothing below
depends on how that row is answered.

Test gate. I ran `cargo mock test` over the whole tree at `d425aff5`. Seven of eight trees are green,
707 tests in the largest, 16 ignored, each ignore carrying a `catalogue:` reason. The eighth tree,
`mock/benches`, is red: four variant manifests (`fnv1a`, `spectral-bisection`,
`structural-decomposition`, `xxhash3`) fail to parse with "error inheriting `arvo` from workspace root
manifest's `workspace.dependencies.arvo`: failed to find a workspace root". That red is outside the
rounding surface and is somebody's to fix; it is reported here so it is not lost.

In the rounding surface I read every test that names `HalfUp`. None is tautological in the strict
sense. Four things are wrong with it, and every one of them is downstream of this question:

1. `src/apply/tests/mod.rs:268-285` asserts that a tie at slot -3 plus a half gives -3 under
   `Mode::HalfUp`. The expected value is the question. Under the answer below it is -2.
2. `src/apply/tests/the_oracle.rs:178` computes `HalfUp` as `nearer.unwrap_or(larger_magnitude)`, and
   the tie cells at `:319-322` check the implementation against a second copy of its own reading. The
   module doc at `:31-33` says so honestly. It is still an oracle that cannot disagree with the map at
   the one input this question is about.
3. `src/apply/tests/the_translation_law.rs:6-8` opens on "Translation by a whole number of slots moves
   every answer by that number. Under every mode and every policy". The setup at `:9-13` keeps every
   position to one sign and every shift even, which is exactly the region where the sign-reading and
   parity-reading modes are equivariant, and nothing anywhere in the crate asserts the other half: that
   `TowardZero`, `HalfEven` and the away-reading of `HalfUp` fail translation by an odd shift or across
   zero. The law the canon's two fusion rows depend on has no test that could fail.
4. `tests/matlab_fi_parity.rs:270-289`, `some_shipped_mode_is_matlab_nearest`, is a catalogue red
   whose comment says it "goes green when the vocabulary gains the name". That assertion encodes the
   inventory reading of the standards bound that `ruling::the_standard_is_parity_in_output_not_in_the_internals`
   exists to correct. The owed test is that MATLAB `Nearest`'s output is reachable through an alias
   over the primitives, not that some enum variant equals it.

The gate says refuse until fixed. I did not, and this is my call rather than a canon ruling, so read it
as suspect. Items 1 to 4 cannot be fixed before this question is answered, because the expected values
they need are the answer. Refusing would make the fix unreachable. The bench red is unrelated to the
surface and does not block reasoning about it.

## What the two seats claim, against the brief's reading of them

The brief summarises 229 as reading `half_up` as one operation, ties toward positive infinity, and 267
as reading it as two names. The second half is right. The first is not. Seat 229's answer, at
`229_lamport_do_the_six_names_denote_one_operation_each.md:406`, is "Finding 2. `half_up` denotes two
operations". Its second phase found that the panel had chosen ties toward positive infinity twice
(`125` and `142`) and said the gap is "one sentence in one row", and then at `:663` wrote "I am not
writing that note". So 229 reports the ambiguity and the panel's usage, and proposes nothing. Seat 267,
at `267_dolan_half_up_denotes_two_operations.md:165-171`, declines to say which reading keeps the name.
Neither seat answered which operation `half_up` denotes.

## Seat 229, against its probes and the canon

All seven of its rounding probes reproduce byte for byte on this host (`269_probes/reproduction.txt`).
Its measured claims hold: the two readings disagree on `2^(W-1-F)` values at every signed row swept and
on none unsigned (a_output); under ties toward positive infinity the equivariant counts are five of six
unsigned and three of six signed, and under ties away they are five and two (b_output); ties toward
positive infinity carries a bias of half a step per tie on the signed domain and ties away carries none
(e_output); no deterministic nearest mode at `(6,2)`, `(6,3)`, `(8,4)`, `(8,5)` is both translation
equivariant and zero mean error (f_output).

Where it fails, or reaches past its evidence:

- `:417-423`, finding 4, writes `overflow policy any`. Probe B never applies an overflow policy; it
  restricts to translations whose endpoints are representable, and the file argues that therefore every
  policy agrees. That is an argument, and op's ratified notation (question 1 of the capture, options 1
  and 2 selected) gives an argument its own marker rather than a sweep's `any`. It is also not the fusion
  law: probe B measures pointwise equivariance of the rounding functions, and the step to the fusion rows
  runs through the proposal that fusion is free exactly at equivariance, which 229 cites. The correct
  reading of finding 4 is "the equivariance partition of the modes matches the partition the two fusion
  rows record", which is true and is weaker than reproducing the rows. The rows themselves were
  reproduced under both readings by `228_probes/p2`, which I re-ran and which agrees digit for digit.
- `:176-178` asserts that `floor(x + 1/2)` is "what hardware and DSP practice mean by round-half-up" and
  that "the DSP literature calls it the asymmetric form". Neither is sourced or stored anywhere I could
  find. The claim is plausible and I have not checked it; Wikipedia's rounding article (stored,
  `269_probes/sources.out`) says "round half up (or round half toward positive infinity)" and notes that
  "Java and Python use half up to refer to round half away from zero", which is the same split from a
  secondary source.
- `:115` and `:197-200` say the ruling "said nothing about what each denotes". About the `says` field,
  true. About the ratified question's lineage, not quite: see the section on what neither seat checked.

None of those moves 229's conclusion that the word is ambiguous in the world. They move how far its
finding 4 reaches.

## Seat 267, against its probes and the canon

Its probe reproduces byte for byte. Its measured numbers are right as printed: ties toward positive
infinity passes 35,775 of 35,775 translation checks, ties away fails 100, half-even fails 1,260. What
fails is almost everything written around those numbers.

1. The load-bearing argument is built on half a row. `267:19-25` quotes
   `ruling::the_standard_is_parity_in_output_not_in_the_internals` as "The standards bound is met by
   agreeing with what a standard documents as its result, for the cases it covers." The row's `says`
   continues, in the next sentence: "It does not oblige matching the standard's internals, and it does
   not oblige carrying the standard's operation list." Its `because` records why the row exists: "An
   expert had derived the operation-set floor as the union of the two named standards' operation sets,
   reading the bound as constraining arvo's inventory. He said unprompted that the bound is about
   output." Seat 267's conclusion at `:71-90` and `:169-171`, "whichever reading loses the name
   `half_up`, both readings need a name, because both are standards-documented operations the canon is
   already bound to reproduce", is that exact inventory reading, re-derived and presented as forced by
   the row that was written to refute it. This is not a difference of approach. It is quoting a
   correction selectively enough to reinstate what it corrected, and every downstream step in 267 that
   says "the standards bound forces" inherits the defect. The bound forces expressibility of the result,
   and op's own words put that expressibility in the alias layer: "provide first-class matlab and ieee754
   compatible apis as aliases over arvo primitives", carried in
   `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`.
2. `267:17` and `:20` give both standards rows as `ratified_by = "op"`. Both are `rung = "stated"` and
   carry no `ratified_by` field. They bind as his direction, which is real, and they are not ratified
   rulings.
3. `267:33-43` presents a block quotation as MathWorks' `fimath` documentation. The stored `fimath` page
   (hash `c3201caf…`, fetched 2026-09-18) contains none of "greater absolute value", "closest
   representable" or "rounds ties to the nearest integer" (counts in `269_probes/sources.out`, all zero).
   Two of the quoted sentences are on a different page, the `nearest` function reference (stored,
   `8fed3544…`, lines 240-246 of its text). "Round to the closest representable number with ties rounded
   based on the sign of the value" appears on no page fetched. The substance is correct and the
   `fimath` page says it in other words ("Ties round toward positive infinity" for `Nearest`, "Ties round
   toward negative infinity for negative numbers, and toward positive infinity for positive numbers" for
   `Round`). A quotation that is not verbatim and is attributed to the wrong page is still a defect, and
   in a file whose argument is "the standard documents X", it is the defect that matters.
4. `267:51-62` presents a block quotation as IEEE 754-2008's definition of `roundTiesToAway`. It is not
   the standard's wording, the standard is not stored, and the file omits the fact that decides how far
   the IEEE half of its argument reaches: `roundTiesToAway` is required only of decimal implementations
   (Wikipedia, stored, "only required for decimal implementations"; secondary, the primary is not open
   access). For a binary fixed-point grid, which is every row this panel has measured, IEEE requires no
   nearest mode but ties to even.
5. `267:111-115` states the probe's domain as "every representable tie in the signed 6-bit domain" and
   its predicate as `total_width = 6, fraction_width in 0..=5`. The probe sweeps `x_num` over
   `(lo * den) ..= (hi * den)` with `lo = -32`, `hi = 31` (`267_probes/a_equivariance_proof.rs:64` and
   `:84-85`), which is an integer part of six bits plus `F` fraction bits, total width `6 + F`. A signed
   six-bit container at fraction width `F` holds `2^(5-F)` negative ties, 31 in total over `F` in 1 to 5;
   the probe's 160 is `32 * 5`, the count for the wider domain. The probe's own header, `:1-3`, calls it
   "the exact domain the fma law row states"; it has no wrap, no multiply-add and no six-bit container.
   The numbers are true of the domain swept and the predicate names a different one.
6. `267:124-131` says the probe "reproduces this partition exactly". It measures three modes, two of
   them the readings in question and the third a control. The partition has six positions.
7. `267:133-144` and `:240-252` treat the shipped `mod.rs:268-279` test as evidence against 229, and
   call 229's not opening it "a gap in what 229's derivation checked". A test in `mock/crates` is the
   leaf tier. What a canon name denotes is not decided by, and cannot be corrected by, what the code
   under a design happens to compute; the code is what gets nuked when the design moves. Not opening it
   was the correct procedure.
8. `267:221-226` says 229 proposes "to pin `half_up` to reading one alone and leaving reading two
   unnamed, absorbed informally into the case for `half_even`". 229 at `:663` declines to write the
   note, and at `:355-359` compares ties away and half-even on mean error only.

What survives of 267: the MATLAB facts, which are right in substance and are the only primary-sourced
external claims in either seat, and the observation that the shipped crate and the recorded law rows
disagree today. Everything that makes 267 an argument for a seventh name does not survive.

## Where the seats disagree, and where they only name differently

On facts they do not disagree anywhere. Both define the two readings identically, both find ties toward
positive infinity equivariant and ties away not, both find the fusion rows true under the first only,
both find the readings coincide on unsigned domains and at `F = 0`.

The one real disagreement is whether the vocabulary must gain a name, and it is not this question. 229
says one sentence, no seventh name. 267 says two names, forced by the standards. With 267's premise
gone, nothing forces the seventh name, and whether one is wanted on other grounds is
`question::is_the_rounding_vocabulary_complete_at_six`.

The rest is naming. 229 says "`half_up` denotes two operations" meaning the word is read two ways in
the world. 267 says the same with "two operations, not a documentation ambiguity". Both are statements
about usage. The question is about the canon, where a name must denote one thing, and on that neither
seat committed.

## What neither seat checked

### The name was minted with a definition, and the ratified row points at it

`question::the_rounding_mode_vocabulary`, the row op answered, carries a `note` saying the coordinator
"directing op to read the source entry directly" and that "this row should be read as a pointer to it
rather than as a substitute". Its provenance is `156` section 6 and `132` section 1.6. `132` C1, at
`132_leroy_the_canon_candidate_for_the_rounding_axis.md:172`, says "`125` section 7 and `128` argue for
six mode names". `125` is where the list comes from, word for word:
`125_knuth_rounding_cold_derivation.md:326` and `:597` propose "retire both spellings and name the
modes `floor`, `ceil`, `toward_zero`, `half_up`, `half_even`, `stochastic`", which is the list op took,
and `125:53` defines the name in the same file: "`half_up` (nearest, ties toward positive infinity)",
with `125:110` giving `half_up(x) = floor(x + q/2)`.

That is not a stamp. Op's question, as captured, listed the names and no definitions, and I do not
claim he ratified the parenthesis. It is the only denotation the name had anywhere in the corpus on the
day it was ratified: `228` F6 (`228_leroy_the_rounding_vocabulary.md:622-627`, census committed as
`228_probes/p5_how_the_corpus_spells_half_up.txt`) counts twenty-five instruments defining a `half_up`,
nine spellings, one function, `floor(x + 1/2)`. 229 found two of these in its second phase and called
them parentheses in unratified files; it did not notice that the ratified row names the file they sit
in as its source.

### Where the other reading came from

`git log` on `mock/crates/arvo-format/src/rounding.rs` puts its first commit, `6436e516`, at 2026-08-31
22:39, about five hours after the ratification commit `807d4b77` at 17:48. That first version already
reads "To the nearest, and a tie goes away from zero" at `:45` and "Nearest, ties away from zero" at
`:62`. The crate's design at the same commit, `DESIGN.md.tmpl:83`, lists the six names and says nothing
about ties, and the round's source changelist, `202608311902_changelist.src.md`, says nothing about
ties either. So the away reading entered arvo in the leaf tier with no design sentence and no canon row
above it: something appeared in code that the design did not say, which is exactly the move the mutation
order forbids. Every later artifact reading `half_up` as ties away, the oracle, the parity file, the
symmetry classification at `src/symmetry.rs:137-142`, the design text at `DESIGN.md.tmpl:657-658` and
`:1163-1172`, and the `asks` of `question::which_tie_direction_an_unqualified_nearest_names`, which
states "Ties away and ties to even are separate members of the ratified six", descends from that
undeclared line. That is how a name with one denotation in the corpus came to have two.

### The other reading needs no name to meet the standards bound

`p270a` checks four compositions against definitions written independently of them, over every value
of every container at `W` in 2..=16, `F` in 1..=W-1, both signednesses: 240 rows, 3,670,024 values.

- Ties toward positive infinity equals `floor(x + 1/2)`: zero failures.
- Ties away from zero equals `toward_zero(x + sign(x) / 2)`: zero failures.
- Ties away from zero equals `x < 0 ? -floor(-x + 1/2) : floor(x + 1/2)`: zero failures.
- Ties away from zero equals `floor(x + 1/2 - [x < 0] * ulp)`, the branchless form: zero failures.

The controls fail exactly as stated before the run: `floor(x + 1/2)` against ties away disagrees on
`2^(W-1-F)` values on every signed row and none unsigned; the sign-blind `toward_zero(x + 1/2)` fails on
every signed row; and the cheap form with the addition done in `W`-bit wrapping arithmetic is wrong on
exactly `2^(F-1)` values at the top of the container, which is the one bit of headroom the exact
addition costs. So whichever reading `half_up` is given, MATLAB `Nearest` and MATLAB `Round` are both an
exact addition and one directed name away. The standards bound, read as op corrected it, is met under
either answer. Which reading the name carries is decided on other grounds.

### A third recorded row depends on the reading

`probe::rounding_commutes_with_the_overflow_policies` says wrapping "commutes with floor, ceiling,
half-up and half-even and fails to commute with toward-zero" at `W` in {3, 4, 5} over sixteen subquanta.
Neither seat lists it. `p270b` runs that shape, eight whole wraps of exact points per width, compared in
the quotient group as `125` compared it: floor, ceiling, half-even and ties toward positive infinity
commute at zero failures on every width, toward-zero fails at 480, 960 and 1920, and ties away fails at
32, 64 and 128. The row is true under the first reading and false under the second. My toward-zero
counts are not `125`'s digits, since my range is wider, so this is a reproduction of the row's pattern
and not of its numbers.

### What each reading lowers to

`p270c` is an ad-hoc quick spike, not a bench: one listing from one compiler on one target, and it
prices nothing. On aarch64 at `opt-level=3`, with `F = 8` over `i32` and the addition widened, ties
toward positive infinity is three instructions (`sxtw`, `add`, `lsr`), ties away in its branchless form
is five, half-even is seven, and all three are branch free. With the addition left in 32 bits, the form
that is wrong at the top of the range, they are two and three. The shape is that neither reading needs a
branch and ties away pays for reading the sign; how much that costs in a real calling context is
unpriced, and deciding anything by it would need the harness in `mock/benches/`.

## The answer

### The name denotes one operation, and arms do not apply to a denotation

The brief asks for arms with predicates where both readings are right in different regions. Both
operations are right in different regions, and that is the next section. The name cannot be. A name that
denotes ties toward positive infinity in one region and ties away in another is a name that denotes two
operations on a signed domain, which is the defect the ratified ruling states as the reason the retired
word went: "On a signed domain the retired word named two different operations", so that "a reader
coming from the hardware and a reader coming from C would have understood the same word as two
operations that genuinely differ". Answering with arms on the denotation would rebuild that defect under
a name the same ruling kept.

`half_up` denotes nearest with ties toward positive infinity, `floor(x + 1/2)`, at every sign.

Holds for: radix = 2, every total width, every fraction width, signedness any, as a statement of what
the name means. It is a definition and not a measurement, so it carries no sweep region; the evidence for
choosing it is below and each piece carries its own.

The grounds, in order of weight:

1. It is the denotation the name was minted with, in the file the ratified row names as its source
   (`125:53`, `125:110`, reached from `question::the_rounding_mode_vocabulary` through `132` C1), and the
   only denotation any instrument in the corpus used on the day of ratification (`228` F6). Choosing it
   is recording what the ratified name already meant in its own lineage. Choosing the other is a new
   decision the lineage does not contain.
2. Two recorded rows are true as written under it and false under the other:
   `law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` (`228_probes/p2`: ties away
   fails at 1.64 to 2.93 percent for `F` in 1 to 5; `229_probes/b` for the partition) and
   `probe::rounding_commutes_with_the_overflow_policies` (`269_probes/p270b`). Their unsigned
   counterparts, `law::fusing_a_multiply_add_preserves_the_answer_under_unsigned` and
   `probe::fusion_under_unsigned_over_six_rounding_modes`, hold under both, because unsigned is where the
   readings coincide.
3. The vocabulary covers the forced trade under it and does not under the other. `229_probes/f` shows no
   deterministic nearest mode at its four points is both translation equivariant and zero mean error.
   With `half_up` as ties toward positive infinity the six carry the equivariant nearest mode (`half_up`),
   the odd zero-mean nearest mode (`half_even`), and the mode that has both in distribution
   (`stochastic`, `229_probes/g`). With `half_up` as ties away the set has no equivariant nearest mode on
   a signed domain at all, and `half_up` duplicates `half_even`'s corner there: identical equivariance
   failure counts, 682 and 682 at `W = 6` (`229_probes/b`), both odd, identical aggregate error on the
   symmetric domain (`229_probes/e`). Op's intent for the framework is that it be "sound, expressible,
   mathematically accurate and exhaustive"; a vocabulary whose names sit on distinct corners serves that
   and one with two names on one corner does not.
4. It is MATLAB `fi`'s default: `fimath` displays `RoundingMethod: Nearest` by default (stored,
   `269_probes/sources.out`).

What weighs the other way, stated at full strength: the spelling `HALF_UP` means ties away in Java's
`RoundingMode` (stored; its own text says it "corresponds to the IEEE 754 rounding-direction attribute
roundTiesToAway" and its table maps -2.5 to -3) and in Python's `decimal` (stored, "Round to nearest
with ties going away from zero"). A reader arriving from either reads the other operation into the
name. That is the ruling's reader hazard, and it survives any pin. The ruling's own instrument for it
is the note it attached to `floor`, "so the hardware operation is not read back into the name", and the
answer carries the same kind of note. Renaming `half_up` would remove the hazard and would change a
ratified list, which the panel cannot do.

### The operations, as arms

These are the two operations and where each is the right one to reach for. The vocabulary names the
first; the second is an alias.

Arm A, ties toward positive infinity, the named mode `half_up`. Right where translation equivariance is
what the design needs: fusing a multiply-add under signed wrapping, commuting rounding with a wrapping
reduction, relocating a range. Holds for: radix = 2, signedness = signed, overflow policy = wrap,
`W = 6`, `F` in 0..=5 for the fusion claim (`228_probes/p2`); `W` in {3, 4, 5} at sixteen subquanta for
the wrap claim (`p270b`); `W` in {6, 8}, `F` in {2, 3} and {4, 5} for the impossibility of having both
properties deterministically (`229_probes/f`). Costs: a bias of half a step per tie on the signed
domain, 128/4 at `W = 8, F = 2` (`229_probes/e`); not odd, so its mirror, ties toward negative
infinity, is outside the six and is reached as `-half_up(-x)`, which needs one bit at the most negative
value (`229_probes/c`, `p270a` C3); the cheap form needs one bit of headroom or it is wrong at the top
`2^(F-1)` values (`p270a` N3).

Arm B, ties away from zero, an alias over `toward_zero` or `floor` and one exact addition. Right where
a standard's output is the requirement: MATLAB `Round`, IEEE 754 `roundTiesToAway` for a decimal grid,
Java `HALF_UP` and Python `ROUND_HALF_UP` parity. Also odd and zero mean on the signed domain, where it
competes with `half_even` and is the one to use when the spec being matched says so. Holds for: radix =
2, `W` in 2..=16, `F` in 1..=W-1, signedness any, as an exact identity (`p270a` C2 to C4). Costs: one
extra exact addition's width, the same bit as arm A; reads the sign, two more instructions than arm A in
the one listing taken (`p270c`, ad-hoc, unpriced); fails translation equivariance on the signed domain,
so every fusion and wrap licence arm A carries is unavailable to it (`229_probes/b`, `228_probes/p2`,
`p270b`).

Where the choice is vacuous: signedness = unsigned, or `F = 0`. The two operations are one function
there (`229_probes/a` control 4, `p270a` N1 at zero on every unsigned row), so no design in that region
depends on this answer.

## Rows this confirms, and rows it corrects

Confirmed as written: `law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping`,
`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`,
`proposal::fusing_a_multiply_add_is_free_exactly_at_translation_equivariance`,
`probe::fusion_under_unsigned_over_six_rounding_modes`, and
`probe::rounding_commutes_with_the_overflow_policies`.

Corrected: `question::which_tie_direction_an_unqualified_nearest_names`, whose `asks` states "Ties away
and ties to even are separate members of the ratified six"; under this answer ties away is not a member,
and the row's second option, "Read `nearest` as `half_up` throughout", no longer imports an open
question. `question::which_operation_half_up_denotes` itself, whose third option is refused at the gate
as belonging to `question::is_the_rounding_vocabulary_complete_at_six`.

Untouched, and not decided here: `question::is_the_rounding_vocabulary_complete_at_six`. Nothing in this
answer needs a seventh name. Whether one is wanted is that row's.

Not decidable from what I have: `law::quantise_then_reduce_commutes`'s note records "32 for ceiling,
half-up and half-even" without the setup; 229 could not reproduce it and I did not try.
`retirement::r132_midpoint_identified_with_half_up` corrects a midpoint identification at ties using
half-up and half-down, and whether its tie points include negative ones, which is what would make it
depend on the reading, I did not open `128` to check.

Design text, under the answer: `arvo-format/DESIGN.md.tmpl:657-658` and `:1163-1172` are wrong, the
second because it carries the inventory reading ("MATLAB needs two nearest-with-ties operations and this
crate names one, so whatever `half_up` turns out to mean, one of the two has nowhere to land"), which is
false once the alias is admitted.

## The shipped `Mode::HalfUp`, against each answer

Under the answer above, it is wrong at every negative tie. The sites are `src/rounding.rs:44` and `:61`,
the tie branch at `src/apply.rs:156-166`, the classification at `src/symmetry.rs:137-142` (under ties
toward positive infinity it reads nothing and does not reflect, like `Floor`), the tests at
`src/apply/tests/mod.rs:268-285`, `the_oracle.rs:178` and `:319-322`, `tests/matlab_fi_parity.rs:270-330`,
and the design text named above. Per the mutation order, the design changes first and the code under it
is rewritten from it, not patched. MATLAB `Round` parity then becomes an alias arm with its own parity
test, which is the catalogue red at `matlab_fi_parity.rs:270` rewritten to what the bound asks.

Under the other answer, ties away, the shipped code is consistent, and the two rows named in the
answer's second ground are false as written, with `half_up` moving from each holding region to its
failing region.

Under the refused third option, the code would be consistent for one of two names and the vocabulary
would be seven, which is outside what this question may decide.

## Consolidation-ready statement

For a ruling row answering `question::which_operation_half_up_denotes`:

> `half_up` denotes rounding to the nearest grid point with a tie going toward positive infinity at
> every sign, which is `floor(x + q/2)` for quantum `q`. It is the denotation the name was proposed
> with in the source the vocabulary ruling was taken from, and the one every instrument in the corpus
> implemented. Ties away from zero is a different operation on a signed domain: it is not `half_up`,
> and it is what Java's `RoundingMode.HALF_UP`, Python's `ROUND_HALF_UP`, IEEE 754's `roundTiesToAway`
> and MATLAB `fi`'s `Round` compute. It is reached as an alias, `toward_zero(x + sign(x) q/2)`, which
> meets the standards bound as a result rather than as a name. The two coincide on unsigned domains and
> wherever no tie is representable.

Suggested fields: `answers = ["which_operation_half_up_denotes"]`, and the note attached to the name in
the manner of the ruling's floor note. A ruling's `corrects` takes ruling slugs only, so the `asks` of
`question::which_tie_direction_an_unqualified_nearest_names` is not corrected through this row; it wants
its own amendment under the coordinator's gate once this is ratified.

It needs a second independent read before it is ratified, and I cannot be the first of two. I read 229
and 267 before forming my answer, and 229's second phase leans the same way, so my agreement with it is
confirmation, not corroboration. The second read should come from a seat that has not opened 229, 267,
228 or this file, given the question row, the vocabulary ruling and its question row, the two standards
rows and the alias obligation, and asked what the name denotes. If it reaches ties away from zero, the
call goes to the coordinator's gate as a disagreement between two reads and not to a third seat.

## Routes I tried and dropped, and alternatives for the next seat

- Arms on the denotation, with ties toward positive infinity on binary grids and ties away on decimal
  ones following IEEE's requirement split. Dropped: it is a name with two denotations on a signed domain
  by construction, which is the ruling's stated defect.
- Deciding by the reader hazard alone. Dropped: it cuts both ways (Java and Python one way, MATLAB's
  default and the DSP form the other), so it cannot pick, and the ruling's own remedy for it is a note.
- Deciding by the shipped code. Dropped: leaf tier, and its reading has no design sentence above it.
- Deciding by cost. Dropped as a decider: `p270c` is an ad-hoc spike showing a two-instruction shape
  difference, and no harness run exists. If anybody wants cost to weigh, it wants a bench with both
  readings, half-even and the narrow forms as arms, in a real calling context.
- Treating the standards bound as forcing a name. Dropped: `ruling::the_standard_is_parity_in_output_not_in_the_internals`
  says it does not, and `p270a` shows it need not.
- Reproducing `law::quantise_then_reduce_commutes`'s half-up column. Not attempted; 229 tried twice and
  stopped for the right reason.

For whoever writes the design round after ratification: the mis-denotation happened because nothing in
the tree could notice a tie rule being chosen in source. Two checks would have caught it. A test pinning
the equivariance partition of all six modes, with the failing modes asserted failing under odd shifts
and sign crossings, fails the moment `HalfUp` reads the sign. And `mock rounding-vocabulary`, which by
the question row's note reports only which names appear, could report any prose pairing `half_up` with
"away from zero"; that is a gotcha and it wants to be a lint.

## Evidence

Committed at `269_probes/`, before this file: `p270a` and its output, `p270b` and its output, `p270c`
and its emitted listing, `sources.sh` and `sources.out` with the external passages quoted above, and
`reproduction.txt` recording that every probe of 229 and 267, and `228_probes/p2`, reproduced on
`aarch64-apple-darwin` under `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.
