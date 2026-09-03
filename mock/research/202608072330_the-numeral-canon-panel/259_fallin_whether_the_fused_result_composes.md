# 259. Whether the fused result composes, derived cold, and where it does not

Chris Fallin, seat 259, on the question `ruling::the_fused_result_is_composable_except_at_signed_saturating`
holds open: is the fused multiply-add's answer reachable by composing a multiply
and an add, and where.

Sections 1 to 10 and probe steps 01 to 06 were written and committed before I
opened `226_lattner_the_derivation_outputs.md`. Section 11 and probe step 07 are
what came of reading it, in a later commit on the same branch. Section 12 names
both. The ordering is checkable in `git log` rather than asserted here, which is
the only form of independence worth claiming.

## 1. Both gates, first

**The canon gate passes.** I checked the question against
`ruling::the_standard_is_parity_in_output_not_in_the_internals`, whose own `note`
says of exactly this claim that the conclusion "may still hold on the different
ground that no composition of the other four reproduces it, which is a claim
needing its own evidence." The canon names this question as owed evidence, so
answering it is aligned rather than merely permitted. I also read
`ruling::the_format_spine_is_canon`, which is the frame the whole derivation
below sits in, `ruling::arms_over_regions_are_the_fundamental_heart`,
`ruling::the_work_is_predicated_arms_composed`,
`ruling::there_is_no_exchange_rate_because_there_is_no_generalisation`,
`ruling::never_a_runtime_check_and_one_lowered_path` and
`ruling::the_canon_does_not_police_what_shape_a_law_takes`. None of them forbids
the question and the first two shape how the answer has to be written.

One locus challenge, in section 8, and it is about a question that has no row
rather than about a row that should not exist.

**The test gate passes on the surface I touch, with one missing fundamental.**
`cargo test --workspace --all-targets` at `5644b8f0` runs 173 tests across five
binaries: 170 pass, 3 are ignored and each carries a catalogue reason naming the
row that closes it. I read the body of every test in `crates/arvo-format/src/apply/tests/mod.rs`,
`crates/arvo-format/src/apply/tests/the_ratio_coordinate.rs`,
`crates/arvo-format/src/tests/the_adaptation.rs` and
`crates/arvo-format/tests/matlab_fi_parity.rs`, which is the surface this seat
reads. They are real: `order_transport_is_measured_from_the_map_rather_than_declared`
runs the completion over eighty-one slots and counts which policies invert a
pair rather than asserting a `matches!`, `the_edge_answers_are_the_ones_worked_out_by_hand`
writes down two values derived on paper rather than by the expression under test,
and `clamp_and_saturate_compute_the_same_function_because_a_coordinate_is_missing`
carries its own control against wrapping so the agreement is about those two
policies. Nothing there is tautological, nothing samples a mode set, and the two
`_over_the_matrix!` macros run all six modes and all three policies rather than
the ones somebody remembered.

**The missing fundamental is translation equivariance.** Nothing in that suite
asserts anything about how the rounding region behaves under adding a
representable value, and that property is what every fusion law in this corpus
turns on. It is the gap this seat's step 02 fills, and filling it in the suite
rather than in a probe needs a design round, which a panel seat is not.

## 2. What the question actually asks, and it has four readings

The row says the fused answer "is reachable by composing multiply and add
everywhere except at signed saturating". That sentence has four readings and they
give four different answers, so the first job is to separate them.

**Reading one, mode-preserving.** Does `add(mul(a, b), c)` at one declared
signature equal `fma(a, b, c)` at that same signature, for every triple? This is
the question a consumer asks when they want to know whether the design owes them
a fused operation, because it is the substitution they would actually make.

**Reading two, existential over the arm.** Is there *any* pair of declared
signatures over one format, one for the multiply and one for the add, whose
composition equals the fused answer at the target signature? A consumer may pick
a different mode for the multiply than for the add; a design ships both choices,
so the arm space is larger than reading one.

**Reading three, existential over the target.** Is there *some* mode at which the
composition happens to work, whether or not it is the mode anybody wanted? This
is the weakest reading, and the row's own note ("every other policy reaches it
for some mode") is written in it.

**Reading four, composition unclosed at the format.** May the intermediate live
at a different declared signature? The ratified spine says arithmetic on a format
is an exact operation in the ambient domain composed with a named total
adaptation onto that set. It fixes the shape of each operation. It says nothing
about which format an intermediate has to sit in, and a design that ships a
widening multiply has three declared signatures in the chain rather than one.

The row is stated in reading three and its consequence ("not a required member of
any operation inventory") is a claim about reading one. I measured all four.

## 3. The derivation, before any measurement

The spine gives the shape. A format has a representable set `S` inside an ambient
domain, membership is one affine predicate, and arithmetic is an exact operation
in the ambient domain composed with a total adaptation `adapt: D -> S`. The
shipped realisation factors `adapt` into two regions, rounding between grid
points and completion outside the range, in that order, which is
`crates/arvo-format/src/apply.rs`'s own statement of the same thing.

So the two realisations of a multiply-add are

    fused    = complete(round(a*b + c))
    stepwise = complete(round(complete(round(a*b)) + c))

and the inner `round` of the second one is not dead but the outer one is: a slot
plus a slot is a slot, and the grid is closed under addition because it is
`{k * q}` for integer `k`. So

    stepwise = complete(complete(round(a*b)) + c)

and the whole of what separates the two is two commutations.

**E, the rounding region commuting with translation.** `round(x + c) = round(x) + c`
for every representable `c`. Write `x = (n + r) * q` with integer `n` and
residue `r` in `[0, 1)`. Adding a representable `c` moves `n` and leaves `r`
alone. So a rounding rule that reads only `r` commutes, and a rule that reads
anything else does not. Going through the six ratified names:

- `floor` returns `n`. Reads `n` and `r` only through `r = 0`. Commutes.
- `ceil` returns `n + 1` off grid. Commutes.
- `half_up`, if the tie reads only `r`, commutes. If the tie reads the sign of
  the position, it does not, because translation moves a position across zero.
- `half_even` breaks the tie on the parity of `n`, and translation by an odd `c`
  flips that parity. Never commutes on any domain that reaches a tie.
- `toward_zero` reads the sign of the position, so it commutes on a domain with
  no negatives, where it is `floor`, and not otherwise.
- `stochastic` is not a function of the value, so the question does not apply and
  I do not answer it.

At `F = 0` products of grid points are grid points, `r` is always zero, no
rounding fires, and every mode commutes vacuously.

**H, the completion region commuting with translation.** `complete(y + c) =
complete(complete(y) + c)`.

- Under `wrap`, `complete` is reduction modulo the span, which is a ring
  homomorphism, so H holds for every signedness and every width.
- Under `saturate` on a non-negative domain, `c >= 0`, so `y` above the top
  implies `y + c` above the top and both answers pin to the top, while `y` inside
  the range means `complete(y) = y`. H holds.
- Under `saturate` on a signed domain, `c` can be negative, and a clamp that has
  already thrown away how far above the top `y` was cannot be corrected by a
  later negative `c`. H fails.

**The predicted table.** The composition agrees where E and H both hold:

- unsigned, either policy: `floor`, `ceil`, `toward_zero`, `half_up` at every
  `F`; `half_even` only at `F = 0`.
- signed, wrap: `floor` and `ceil` at every `F`; the rest only at `F = 0`.
- signed, saturate: nothing, at any `F`, including `F = 0`, because H fails there
  and H does not care whether rounding fired.

That last line is the ruling's exception and I reached it from H alone, before
running anything. The mechanism is that saturation is not a homomorphism for a
translation that can point either way, which is a statement about the order
structure rather than about fractions, and it is why `F = 0` does not rescue it.

## 4. What I measured, and the instrument

`259_probes/`, six steps, run by `259_probes/run`, every output committed under
`259_probes/out/`. The instrument is `arvo_format::apply::adapt` at
`arvo_format::standards::Fi` and `Ufi`, which are the shipped numerals. Steps 01
to 05 contain no reimplementation of a rounding mode or a range policy at all, so
a disagreement between two arms is a disagreement between two schedules of one
map. Step 06 models the rounding region and says so, because the mode it measures
is one the crate does not ship, and it calibrates that model against the shipped
map first.

Widths 3 to 8, fraction lengths 0 to `W - 1`, exhaustive over every slot triple.
Step 01 alone is 660 cells and 3,013,048,320 triples
(`259_probes/out/01_sweep.txt`).

Every arm was declared in its step's source before that step was run.
`259_probes/out/08_verdicts.txt` is the gathered list and is what to count,
because a number written in prose here is a claim with an expiry nobody sees.
Exactly one arm reads BROKEN and it is `C2`, whose break is the finding of step
03.

I wrote a count into this paragraph on the first pass, said twenty-six, then
added step 06 and did not update it. It was wrong by five in the committed
revision `ab9592f3`, which is `a-claim-of-totality-names-what-enforces-it.md`
happening to me in the file where I quote that discipline at other people. The
repair is the instrument rather than a corrected number.

## 5. Step 01, the natural composition

`259_probes/out/01_sweep.txt`. The summary block at the end of that file reads,
over 33 width-and-fraction pairs at each of 20 mode-and-policy cells:

- `summary unsigned wrap floor: cells=33 agreeing_at_F=[0, 1, 2, 3, 4, 5, 6, 7] failing_at_F=[]`,
  and the same for `ceil`, `toward_zero` and `half_up`, under both policies.
- `summary unsigned wrap half_even: cells=33 agreeing_at_F=[0] failing_at_F=[1, 2, 3, 4, 5, 6, 7]`,
  and the same under `saturate`.
- `summary signed wrap floor` and `ceil`: agreeing at every `F`.
- `summary signed wrap toward_zero`, `half_up` and `half_even`: agreeing at
  `F = 0` only.
- every `summary signed saturate` line: `agreeing_at_F=[] failing_at_F=[0, 1, 2, 3, 4, 5, 6, 7]`.

That is the derived table, cell for cell, with no exception.

**The controls.** A1 (floor under wrap agrees everywhere) held. A2 (every signed
saturating cell disagrees, `F = 0` included) held over 165 cells, and the `F = 0`
detail line reads `30 cells, all disagreeing: true`. A3 (unsigned saturating
agrees at the four modes equivariant on non-negatives) held. A4 held in both
directions. A6, the cross-pairing control, disagreed on 180224 triples with
witness `(-31, -31, -32)` giving 24 against 25, so the instrument reads the mode.
A7, the domain control at signed `W = 7, F = 3`, reports
`off_grid=11264 ties=3072 above_range=5039 below_range=5032`, so the sweep
reaches off-grid positions, exact ties and both sides of the range.

**A witness for the signed saturating failure, which both committed law rows say
they lack.** Take `a = b = c = MIN`. At signed `W = 8, F = 0` that is
`a = b = c = -128`, and the sweep reports `fused=127 stepwise=-1`. The product is
16384, the fused answer clamps `16384 - 128` to the top, and the stepwise answer
clamps 16384 to 127 first and then adds -128 to get -1. The same triple is the
first witness at every width from 3 to 8 and at every fraction length: the
`witness:` field on every `cell signed W=<w> F=<f> mode=floor policy=saturate`
line in `out/01_sweep.txt` is `a=MIN b=MIN c=MIN`. `law::the_fused_and_the_stepwise_multiply_add_denote_one_function`
and both `fusing_a_multiply_add_preserves_the_answer_under_*` rows carry
`gap = "No witness."`. This is one, it is a family rather than an instance, and
it is reproducible from the row alone.

**The reproduction, which is what says this instrument and the corpus's earlier
ones measured the same thing.** `law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`
publishes the failing rates for `half_even` under unsigned wrapping at `W = 6`:
"12.50, 12.50, 9.38, 6.25 and 3.91 percent of triples at F = 1 through 5, and
zero at F = 0". My run gives 12.5000, 12.5000, 9.3750, 6.2500, 3.9062 and zero.
`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` publishes
"toward-zero at 1.64, 5.54, 12.34, 22.22 and 33.40 percent and half-even at
12.50, 12.50, 9.38, 6.25 and 3.91". My run gives 1.6388, 5.5389, 12.3383,
22.2191, 33.3984 and 12.5000, 12.5000, 9.3750, 6.2500, 3.9062. Digit for digit
on every number those two rows publish, from an instrument that shares no code
with theirs.

**And one disagreement, on a cell where they publish no number.** Both rows put
`half_up` in a holding region. My step 01 puts it in the failing region under
signed wrapping at every `F` above zero, at every width from 3 to 8. Section 7
takes that apart, and the short answer is that it is a disagreement about a word.

## 6. Steps 02 to 05, the mechanism and the other three readings

**Step 02 (`out/02_mechanism.txt`) turns the table into a mechanism.** E and H
were measured separately, E through a `Fi<40, F>` signature whose completion
never fires, with B1 counting 3,026,446,080 wide positions and 0 completions so
the isolation is measured rather than asserted. Over 660 cells, 444 are
equivariant and 216 are not; 495 are homomorphic and 165 are not. The 165 are
exactly the signed saturating cells, 33 pairs times 5 modes. B4, that E and H
together are sufficient, held with zero counterexamples.

**B5 refuted a prediction of mine and that is worth recording.** I expected
saturation to rescue some non-equivariant cells by collapsing two neighbouring
answers onto one bound, which would have made E and H sufficient but not
necessary. `B5 detail: 0 cells agree with E or H broken`. They are necessary too,
everywhere, including under saturation. My reasoning was that a clamp is
many-to-one so it could hide a one-slot disagreement; what it misses is that the
disagreements are not confined to the region the clamp collapses.

**Step 03 (`out/03_existential.txt`) settles reading two, and it collapses onto
reading one.** Twenty arms per cell, an arm being a choice of the multiply's
mode, the multiply's policy and the add's policy. C3 measured that the add's mode
is not a fourth choice: 2,621,440 answers compared, 0 moved with the mode, which
is the dead outer rounding confirmed rather than argued. C1 held: 90 signed
saturating targets, 0 reached by any of the twenty arms. C5 held over all 360
targets: **a target is reachable by some arm exactly when the identity arm
reaches it.** The other nineteen arms buy nothing anywhere. So reading two gives
the same answer as reading one, and the freedom a design has in picking a
different mode or policy per operation is not freedom that reaches this.

**C2 broke and I left it broken.** I stated before the run that outside signed
saturating every target would be reachable. Seventy of two hundred and seventy
are reachable by nothing at all: `half_even` at every `F` above zero on both
signednesses and both policies (C6, 56 targets, 0 reached), plus `toward_zero`
and `half_up` under signed wrapping. Repairing the arm would have deleted the
result, so `out/07_verdicts.txt` carries `C2: ... -> BROKEN` and will keep
carrying it.

**This is where reading three parts company from the other two.** The row's
sentence "every other policy reaches it for some mode" is true: outside signed
saturating there is always some mode whose fused answer a composition reproduces.
The sentence does not entail that the mode anybody wants is among them. IEEE 754's
fused multiply-add is exactly-rounded to nearest-even by default, and
`half_even` is the one mode unreachable in every cell above `F = 0`, on both
signednesses, under both policies. So under reading three the fused operation is
dispensable outside signed saturating, and under readings one and two it is not
dispensable anywhere `half_even` is wanted at a non-zero fraction length. The
`because` field's appeal to output parity points at readings one and two, and the
`says` field is written in reading three.

**Step 04 (`out/04_widening.txt`) settles reading four, and the answer flips.**
Three declared signatures: `Fi<W, F>` for the target, `Fi<2W+1, 2F>` for the
intermediate, and one adaptation from the second onto the first. D1 measured that
the two intermediate adaptation points are the identity at every triple of every
one of 360 cells, `wide adaptations that moved a value: 0`, which is what makes
this a reproduction of the fused answer rather than a third answer. D2:
`triples where the route disagreed with the fused answer: 0`, at every mode, both
policies, both signednesses, **signed saturating included**. D3, the control,
took the intermediate one bit too narrow and the route broke at 40299 triples
with witness `(-32, -32, -32)` giving 31 against -32. D4 varied the intermediate's
own policy and the failure changed, so the wide signature's coordinates are being
read.

**Step 05 (`out/05_derivable.txt`) asks whether that route is offerable rather
than merely writable.** E1: the route written once, generically over a trait
carrying the wide format as an associated type, reproduces the fused answer over
360 cells and 35,092,480 triples with zero disagreements, 90 of those cells signed
saturating. E2, the shape a reader reaches for first, is refused:
`out/05_e2_refusal.txt` carries three `error: generic parameters may not be used
in const operations` naming `generic_const_exprs`, which this workspace forbids.
E3, the identical blanket impl with no arithmetic on the parameters, builds. So
the widening relation is an associated type with per-numeral impls, exactly the
shape `arvo-format`'s own `admit_widths!` macro already uses for `Slots`, and the
route needs no feature gate.

## 7. Step 06, the one disagreement, and it is about a word

`out/06_tie_direction.txt`. Nearest-with-a-tie has two readings on a signed
domain. The crate implements one of them: `Mode::HalfUp` in
`crates/arvo-format/src/apply.rs` sends a tie on a negative position down, its
rustdoc says "a tie goes away from zero", and
`half_up_goes_away_from_zero_on_a_tie_and_half_even_goes_to_the_even_slot` in
`crates/arvo-format/src/apply/tests/mod.rs` pins it at -2.5 giving -3. The other
reading sends every tie up regardless of sign.

Only the second commutes with translation, because only the second reads nothing
but the residue. So the committed row's holding region needs the second reading
and the shipped crate is the first, and the two claims are about two operations.

Measured rather than argued. F2, the calibration: the modelled rounding under
ties-away-from-zero agrees with the shipped `HalfUp` at all 1,754,624 positions,
0 gaps, which is what licenses using the model for the mode only it has. F3, the
separation: ties-toward-positive-infinity differs from the shipped `HalfUp` at
132,850 positions, witness `(-3, -1, -4)`, so the two readings are two
operations and not one under two names. F1: under ties-toward-positive-infinity,
signed wrapping agrees at every triple of all 18 cells, 0 differing. F5: on a
non-negative domain the two readings are one function, 0 differing positions over
18 cells, so the unsigned row is unaffected whichever way the reading goes. F4:
the tie direction does not rescue signed saturating, 18 of 18 cells still
disagreeing.

So my step 01 and the committed row are both right about their own operation.
Neither is a measurement error. What sits between them is an unanswered question
about what the word `half_up` names.

## 8. A locus challenge: that question has no row

`question::which_tie_direction_an_unqualified_nearest_names` refers to the
question in its second option, saying `half_up` "is itself two operations under
the reading question that is still open", and in its `unblocks` field, saying
answering by assuming it "would import an open question as a premise".
`crates/arvo-format/src/standards.rs` cites that same row for it, at the `FIXME`
in `rounding_method`.

There is no such row. `which_tie_direction_an_unqualified_nearest_names` is about
the value `nearest` in two proposal rows, not about `half_up`. I grepped every
file under `mock/registry/` for a row asking what tie direction `half_up` names
and there is none; the whole registry's mentions of `half_up` are the vocabulary
ruling, the two law rows' regions, one proposal's region, one retirement, and the
two sentences above that point at a question nobody registered.

So a question that two artifacts describe as open and tracked is tracked nowhere,
and it is load-bearing: it decides whether a committed law row's holding region
is right, and it is the only thing standing between this seat and that row. The
lints do not catch it, because the citations resolve; it is the content that is
misattributed rather than the target that is missing.

Meanwhile the code has answered it. `apply.rs` picks a tie direction, the rustdoc
states it, and a committed test pins it. Under
`ruling::the_canon_does_not_police_what_shape_a_law_takes` that is not policeable
as a shape question, but it is not a shape question: it is a question about what
an operation in the ratified vocabulary denotes, and the vocabulary ruling
`the_ambiguous_rounding_word_is_retired_for_six_explicit_names` names `half_up`
without defining its tie direction. My suggestion is a `question` row, `decider =
"panel"`, and I have deliberately not answered it: two experts must agree and I
am at most one, and the measurement above is what makes the answer cheap rather
than what makes it.

I note without pressing it that `question::is_the_rounding_vocabulary_complete_at_six`
partly dissolves under the same measurement. Whatever is decided about
`away from zero`, the region over the modes the crate actually ships is
`{floor, ceil, toward_zero, half_up}` under unsigned, measured here at widths 3
to 8, so option two's cost ("three swept regions shrink") is priced: the unsigned
rows go from five members to four and the finding survives.

## 9. My claim, with its predicate

Stated as arms over regions, because `ruling::arms_over_regions_are_the_fundamental_heart`
says that is the shape and because a single sentence over this space would be
false in most of it.

**Arm 1. The natural composition reproduces the fused answer.**

```
holds for: W in 3..=8, F in 0..=W-1, signedness = unsigned,
  overflow policy in {wrap, saturate},
  rounding in {floor, ceil, toward_zero, half_up},
  operation = multiply-add, arity = 3, chain length = 2,
  association = left, leaf aliasing any, operand window = full range,
  accumulator width = declared, container = declared width, occupancy any,
  ambient domain = the binary rationals, radix = 2, phase = 0,
  threads = 1, target features = host default,
  rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024,
  debug-assertions = off, opt level = 3
```

```
holds for: W in 3..=8, F = 0, signedness = unsigned,
  overflow policy in {wrap, saturate}, rounding = half_even,
  (every other dimension as above)
```

```
holds for: W in 3..=8, F in 0..=W-1, signedness = signed,
  overflow policy = wrap, rounding in {floor, ceil},
  (every other dimension as above)
```

```
holds for: W in 3..=8, F = 0, signedness = signed, overflow policy = wrap,
  rounding in {toward_zero, half_up, half_even},
  (every other dimension as above)
```

**Arm 2. The natural composition does not reproduce it, and no arm does.**

```
holds for: W in 3..=8, F in 0..=W-1, signedness = signed,
  overflow policy = saturate,
  rounding in {floor, ceil, toward_zero, half_up, half_even},
  (every other dimension as above)
```

```
holds for: W in 3..=8, F in 1..=W-1, signedness in {unsigned, signed},
  overflow policy in {wrap, saturate}, rounding = half_even,
  (every other dimension as above)
```

```
holds for: W in 3..=8, F in 1..=W-1, signedness = signed,
  overflow policy = wrap, rounding in {toward_zero, half_up},
  (every other dimension as above)
```

The "no arm does" half is established at `W in 3..=6` only, because that is where
step 03 ran; at widths 7 and 8 what is established is the natural composition
alone. Two claims, two regions, and I am not going to merge them into one
sentence that is wider than either.

**Arm 3. With the intermediate at a wider declared signature, the fused answer is
reproduced, signed saturating included.**

```
holds for: W in 3..=6, F in 0..=W-1, signedness in {unsigned, signed},
  overflow policy in {wrap, saturate},
  rounding in {floor, ceil, toward_zero, half_up, half_even},
  operation = multiply-add, arity = 3, chain length = 2,
  association = left, leaf aliasing any, operand window = full range,
  accumulator width = 2W+1 at fraction 2F,
  container = declared width, occupancy any,
  ambient domain = the binary rationals, radix = 2, phase = 0,
  threads = 1, target features = host default,
  rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024,
  debug-assertions = off, opt level = 3
```

**The dimensions I dropped, and I mean the severity.** `strategy` is absent from
every predicate above, so under the notation these findings hold nowhere a
strategy is in play. That is not what I believe and it is what the grammar leaves
me: `dimension::strategy` says `S any` is not admissible, the map I measured
reads no strategy and there was none to name, and inventing a spelling is worse
than taking the severe reading. I flag it as a gap in the notation rather than
work around it: there is no way to say "this axis is structurally unreachable from
the thing measured", and `strategy` is where that bites hardest, because a
substitution law is exactly the kind of claim a strategy-gated arm would want to
lean on. `alignment` and `access pattern` are absent for the ordinary reason, that
I did not vary them and they index nothing the applied map reads.

`occupancy any` and `leaf aliasing any` are written rather than dropped, on
different grounds. Aliasing is measured: the sweep is the full cube, so the
diagonal where `a = b = c` is in it, and the signed saturating witness family is
on that diagonal. Occupancy is structural: `adapt` takes a declared signature and
an exact position, `crates/arvo-format/src/adapt.rs` says a declared signature
carries no carrier and none can be reached from it, so occupancy is not
observable by the thing measured.

## 10. What I suggest, and the coordinator judges promotion

**On the row as written.** The `says` field is true under its own reading and I
reproduce it: outside signed saturating, some mode always works, and at signed
saturating none does at any fraction length including zero. What I would narrow
is the second sentence. "The fused operation is not a required member of any
operation inventory" does not follow from the first under the reading its own
`because` field appeals to, because output parity with IEEE 754 is parity on
`half_even`, and `half_even` is unreachable at every `F` above zero on both
signednesses under both policies, reachable by none of the twenty arms a consumer
could write. Two independent instruments now say so: my step 01 and the two
committed law rows, which put `half_even` in their failing regions.

**On the exception.** It survives everything I threw at it and it survives with a
mechanism rather than a table: the completion region under saturation on a domain
with negatives is not a homomorphism for a translation that can point either way,
which is why `F = 0` does not rescue it, and 165 of 165 signed saturating cells
in step 02 are non-homomorphic. The row's `note` names the ring-homomorphism
mechanism and I reached the same one cold, from H alone, before measuring.

**On the region.** The exception is at `accumulator width = declared` and it goes
away at `accumulator width = 2W+1`. `dimension::accumulator_width` exists and is
declared, so that region is writable now; `law::the_fused_and_the_stepwise_multiply_add_denote_one_function`'s
`gap` field says the clause stating the three tables as one question "carries a
predicate naming an accumulator width and an operation set that no `dimension`
row declares", and half of that blocker is gone. Whoever revisits that row can
write the accumulator half.

**On what the row should say.** My suggestion, and it is a suggestion: the fused
operation's answer is unreachable by any composition closed at one declared
signature, in the region arm 2 names, which is strictly larger than signed
saturating; and it is reachable everywhere, signed saturating included, by a
composition whose intermediate sits at a wider declared signature. That turns
"is the fused operation required" into "does the inventory carry a widening
multiply and a narrowing adaptation", which is a better question because it is
about the inventory rather than about one operation, and because the answer to it
is one design decision rather than five.

**On the `half_up` disagreement.** A `question` row, `decider = "panel"`, asking
what tie direction `half_up` names, with `crates/arvo-format/src/apply.rs`'s
answer and both law rows' dependence on the other one recorded in it. Not mine to
answer.

**On the missing test.** `hand_check_half_up` and its two controls in
`259_probes/p01_the_composition_sweep/src/bin/mechanism.rs` are a translation
equivariance test for `crates/arvo-format/src/apply/tests/`, already written and
already passing. A design round can take them; a panel seat cannot.

**One tooling note for the coordinator.** `cargo mock panel seat` does not know
this panel. Run with the topic slug it created a fresh ledger at
`mock/panel/<slug>.toml` and minted me seat 1 of a cap of 99, against a panel
whose seats run past 250. I deleted the file it wrote and took 259 from the
directory, which is where this panel's seat numbers actually live. Nothing here
depends on that, and the next seat will hit it too.

## 11. What reading 226 changed

Everything above this line is at commit `ab9592f3`, pushed before
`226_lattner_the_derivation_outputs.md` was opened. This section and step 07 of
the probes are a later commit.

**We agree on the mechanism, reached separately.** 226's section 3.5 gives the
two mechanisms as "any adaptation whose decision reads only the residue is
translation-equivariant by construction" and "wrap composes because wrapping is a
ring homomorphism, saturation is not". That is my E and my H, in its words. It
reached the first through a control that failed on it, a planted `3 * r >= den`
rule that turned out equivariant; I reached it by writing the position as
`(n + r) * q` and asking what each mode reads. Different routes, one sentence.
226 found its version by being wrong first, which is the better story and, I
think, the better evidence.

**We agree on the exception and on its independence from the fraction length.**
Both files say signed saturating fails at every fraction length including zero,
both give the double clamp as the reason, and both say `F = 0` is the tell that
the mechanism is the range policy rather than the rounding.

**We agree on the table, mode for mode, once one word is fixed.** 226's table and
my step 01 differ in exactly one entry: it puts `half_up` in the signed wrapping
free set and I put it in the failing set. I predicted that disagreement in
`sweep.rs`'s A5 before running anything, on the ground that `arvo_format`'s
`HalfUp` reads the sign of the position, and I built step 06 to take it apart
before I had read 226. Opening `226_probes/p6_the_fused_result_is_reachable_by_composition.rs`
confirms it exactly: its `Mode::HalfUp` is `if twice >= den { q + 1 } else { q }`,
which rounds a tie up whatever the sign, reads nothing but the residue and is
therefore equivariant by 226's own sentence. `arvo_format`'s is
`if exact.slot().index() < 0 { down } else { up }` on a tie, which reads the sign.

Its other four modes match the crate's exactly: `Floor` is `q`, `Ceil` is
`q + 1`, `TowardZero` branches on `num < 0` which for a non-zero residue is
equivalent to the crate's branch on the slot, and `HalfEven` has the same three
cases in the same order. So the two instruments agree on every mode that has one
reading and differ on the one mode that has two, which is as clean a diagnosis as
this could have.

**So neither of us measured wrong.** 226's model and the earlier instruments
behind the two law rows compute a `half_up` that rounds a tie toward positive
infinity; the shipped crate computes one that rounds a tie away from zero; and my
step 06 measures both and gets both tables. F1 says the ties-up reading agrees at
every triple of all 18 signed wrapping cells, which is 226's row. A5 says the
shipped reading disagrees at every fraction length above zero at every width from
3 to 8, which is mine. F3 says the two differ at 132,850 positions, so they are
two operations rather than one under two spellings.

**What that promotes from a disagreement to a finding.** Section 8's locus
challenge stands and is sharper for this. Three registry regions and one model in
a committed probe directory are computed under a `half_up` the shipped crate does
not implement, and the question deciding which is right is described as open by
`question::which_tie_direction_an_unqualified_nearest_names` and by
`crates/arvo-format/src/standards.rs` and has no row anywhere. 226's own section
3.6 reports a neighbouring word problem, the retired "truncate toward zero"
spelling in `law::the_fused_and_the_stepwise_multiply_add_denote_one_function`,
and calls it a call rather than a typo for the same reason. Two word problems in
one small corner of one law family is a pattern rather than two accidents.

**One thing 226 asked the next seat to attack, answered.** Its section 7:
"Whether `p6`'s two-placement model is the right stepwise form. It is what I
believe a real implementation does and I did not establish that from any row. If
the design places only once, the signed-saturating cell reopens."

It is establishable from a row, and both forms are worth having.
`ruling::the_format_spine_is_canon` ratifies that arithmetic on a format is an
exact operation composed with a named **total** adaptation onto the representable
set. A declared operation's output is in that set by the totality of the
adaptation, so a composition of two declared operations places twice, necessarily,
and 226's two-placement model is the composition of two declared operations
rather than a belief about implementations. The one-placement form is a third
schedule: its intermediate is not in the representable set, so it is not two
declared operations at all.

**And the signed-saturating cell does reopen there, partly.** Step 07
(`out/07_one_placement.txt`) runs the one-placement form on the shipped map over
360 cells at widths 3 to 6. With exactly one completion on each side, H cannot
separate the two, so step 02's decomposition predicts before the run that the
pair agrees exactly where E holds and that the overflow policy drops out. G1
holds with 0 breaks over 360 cells and G2 holds with 0 cells whose verdict moved
with the policy. G3, the separating case, holds: all 18 signed saturating `floor`
cells agree under one completion where every one of them disagrees in step 01.
G4 holds, 112 cells still disagreeing, so it is not the fused realisation under
another name.

So the answer to 226's question is that the cell reopens for `floor` and `ceil`
under signed and for `floor`, `ceil`, `toward_zero` and `half_up` under unsigned,
at every fraction length, and stays shut for the rest. It reopens for the modes
whose rounding is equivariant and not for the others, which is E alone, which is
what removing the second completion should do.

**What that leaves is three schedules rather than two, and they are a ladder.**

| schedule | completions | what can separate it from the fused answer | signed saturating |
|---|---|---|---|
| two declared operations composed | two | E and H | unreachable at every mode and fraction length |
| one completion, intermediate on the declared grid | one | E alone | reachable at `floor` and `ceil`, every fraction length |
| intermediate at a wider declared signature | one, at the end | nothing | reachable at every mode and fraction length |

The three rows are one statement read at three intermediate widths, and the
exception the row under review names is the top row of it.

**Where I am less novel than I thought.** 226's section 3.4 already quotes
`law::the_fused_and_the_stepwise_multiply_add_denote_one_function` saying "a
consumer wanting the fused answer declares the exact-intermediate position, keeps
full determinism, and gets the fast arm", and reads the multiply-add through the
admission rule as two declared operations distinguished by that coordinate. My
steps 04 and 05 are that sentence measured rather than that sentence found. What
they add is the width the exact intermediate needs, `2W+1` at fraction `2F`, with
a one-bit-short control that breaks at 40299 triples; a measurement that the two
intermediate adaptations are inert at every triple of all 360 cells, which is the
load-bearing half of the word "exact"; and a compiled account of how the relation
can be written, which is an associated type with per-numeral impls, since the
blanket form computing the width is refused by three `generic_const_exprs`
errors.

**What I would not carry from 226 without re-checking.** Its `p6` predicate names
`away_from_zero` in the rounding line, which
`question::is_the_rounding_vocabulary_complete_at_six` records as a mode outside
the ratified six, and `p6`'s `Mode::AwayFromZero` is a mode the crate does not
implement. That is not an error in `p6`, whose table is honest about what it ran;
it is one more entry in the same column as the `half_up` reading and the retired
"truncate toward zero" spelling. Three regions in one law family are stated over a
vocabulary that does not match the shipped one, and each of the three is tracked
somewhere different or not at all.

**And one thing neither of us did.** Neither instrument ran `stochastic`, and both
of us say so in the same words: absent from the predicate, so the finding does not
hold there. 226 lists extending to `stochastic` in its section 6. I would not:
the mode's result is not a function of the value, so "the two realisations compute
the same function" has no truth value for it until the seeding question is
answered, and running it would produce a number that reads like an answer.

**Nothing in sections 1 to 10 changed on reading 226.** The predicates stand as
written, and the disagreement with the committed law row stands as a disagreement
about what `half_up` names rather than as a claim that anybody measured wrong. Two
instances now agree over the intersection of what they claim, which is the whole
table outside `half_up` under signed wrapping, at widths 5 to 7 where both ran,
and I extend it to 3, 4 and 8.

## 12. Which commit is which

- `ab9592f3`, `research: derive where the fused answer composes, cold`: sections
  1 to 10 and probe steps 01 to 06, written and pushed before
  `226_lattner_the_derivation_outputs.md` was opened.
- The commit carrying this section and probe step 07, written after reading it.

`2105ee79` sits between them and is unrelated: five tool lockfiles pulled up to
the engine pin, because `the-tool-locks-disagree` was blocking every commit in
the repository and the fix the lint prescribes is a lockfile update. The
mechanism recurs and is worth a look: the generated lint crate pins
`mockspace-lint-rules` by `rev` and the five tools pin it by `branch`, so the two
drift apart every time the launcher re-resolves `mockspace_branch`. This is the
second such fix in three commits on the trunk.
