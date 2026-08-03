# 44. What the overturn left behind

**Member:** Talia Ringer. I wrote file 19, twenty-five files back, on the witness and its upkeep; that
file's own subject was proof repair as a discipline, keeping a checked result alive as the thing it is
about keeps moving, and I carry none of its other conclusions forward unexamined except the habit of
mind, which this dispatch asked for by name. A checked claim is not a trophy filed away. It is a claim
about an object, and the moment the object's own definition moves, the claim's status becomes unknown
until someone re-runs it. The field I came from calls the failure to do that "bit rot" when it happens
to a proof script; this review has now produced two instances of the identical failure happening to
prose, found by accident, and my job here is to go looking for the third, fourth and fifth on purpose,
and to say what would have found the first two without luck.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9
ignored, summed per binary rather than trusted from a headline, matching files 41, 42 and 43 exactly
(9 = 6 + 2 + 1 across the three ignored groups I can see in the raw output, the same shape those files
report). `grep -rln "Adjustment\|Bias\|Numeral" crates/ --include="*.rs"` from the repo root returns
nothing, the same empty result files 36, 41, 42 and 43 all record: the coordinate this dispatch is about
has no shipped source and nothing shipped to regress. Canon gate: `40_consolidation_three.md` in full,
the required base reading, plus (per this dispatch's own explicit licence to reach behind it) `26_
consolidation_two.md` in full and the identity-half files that argued D69 (`27`, `28`, `30b`), because a
claim's derivation coordinates are exactly what a consolidation compresses away and what this file's
question needs visible. Nothing below overturns a ratified call. Every correction in this file is to an
unratified sentence, in the reasoned-without-artifact or carried-without-re-derivation bins by the
review's own four-bin discipline (`38`, restated `40:29-32`), which is precisely the bin this dispatch
exists to empty.

**What I read:** `40_consolidation_three.md` in full. `41_chlipala_the_rational_bias.md`, `42_arntzen_
the_observation_surface.md` and `43_smith_division.md` in full, the three deliverables since it. Then,
by this dispatch's own explicit exception, behind the consolidation: `26_consolidation_two.md` in full
(the pre-D69 base every later file's droplist and open list still cites by line number), `27_carmack_
what_a_number_is.md` and `28_leroy_what_identity_must_express.md` in full (the two files that formed the
D69 overturn independently, read to see which of their own sentences were written before op's checkpoint
ratified the coordinate they argue for and which after), `30b_op_checkpoint_seven.md` (the overturn
itself, in op's words), `33_lamport_the_laws_restated.md` in full (the algebra restatement I suspected,
and confirmed, runs ahead of the encoding on the one axis I checked), and `01_knuth_mathematical_rigour.
md` sections 9 and 10 at the source (the pre-D69 `FullRange<1>` finding, read because file 40's own open
list cites it only by consequence, `40:685-687`, not by its original argument). `36_kiselyov_the_normal_
form_and_its_price.md` and `39_knuth_does_it_still_represent_them.md` at the sections files 41/42/43
already cite, not reread in full, since those two files' own reading of them is already checked
independently by two later members. `ls` of the review directory once: 43 numbered deliverables plus
probe directories before this one.

**What I compiled or measured, separated from what I reasoned.** One new probe, `44_probes/probe_1_the_
overflow_band_for_mixed_format_addition.rs`, built fresh against the workspace pin (`rustc 1.98.0-
nightly (57d06900f 2026-05-27)`, confirmed with `rustc --version`), all outcomes in `44_probes/OUTCOMES.
md`. One existing probe rebuilt fresh rather than trusted from its own outcome file: `33_probes/probe_5_
direction_enters_the_key_iff_the_lattice_opens.rs`, to confirm section 2 below's claim that it still
compiles clean against the current pin. Everything else in this file, the sweep of which pre-D69 claims
were re-derived and which were not, the reading of the `FullRange<1>` finding's status under the current
tower, the mechanism analysis, and the discipline proposal, is reasoning on those results and on the
citations named in place, and is marked as such throughout.

## 0. The verdict, stated first

**The sweep found one more claim carrying the identical defect the dispatch's own two examples carry,
one claim that looked like a third candidate and checked out clean on inspection, and one already-open
item that reconnects to a discipline nobody had named for it.** None of the three is a new hole in the
ratified shape. All three are holes in the *review's own record of itself*: sentences sitting in the
carried-forward, evidence-class-tagged prose that the class tag does not actually protect, because the
tag records whether a claim was checked, never against what.

**The new finding.** `28:229-231`, carried unchanged into `40:178-180`, claims the overflow band (the
region between the largest representable value and half a quantum past it, where round-first and
classify-first disagree) is "inhabited for multiplication, division, mixed-format addition and every
float operation." Two of its four members were independently compiled before this dispatch:
multiplication (files 30/31/33) and, as of file 43, division, which turned out **wrong** as a blanket
statement (empty for same-precision division, inhabited only once precisions decouple, and even then
per-format-triple rather than unconditionally, `43:88-98`). The other two members, mixed-format addition
and every float operation, had never been checked at all; they were carried on the strength of their
two siblings. Compiled here (`44_probes/probe_1`): mixed-format addition **can** inhabit the band
(two independently shaped witnesses), and it is **not unconditional** either (a 40-triple sweep splits
36 inhabited, 4 empty, and the four empty cases are exactly the ones where one operand's quantum divides
the other, collapsing the pair into disguised single-quantum arithmetic). The sentence needs the same
per-format-triple correction division already received. The fourth member, every float operation,
remains completely unchecked after this file; nothing here or anywhere in the review speaks to it, and
the fact that its two nearest neighbours in the same sentence each turned out to need correction is a
reason to trust it *less*, not more, on the strength of the sentence it sits in.

**The claim that checked out.** File 33's additive and narrowed-multiplicative closure formulas
(`33:265-277`, carried into `40:291-293`) were derived and probed at a point in the review's history when
the design's stated `Bias` type was a plain signed integer, three files before file 39 found that wrong
and two members before files 41/42 repaired it. On first look this is exactly the shape of the other two
findings: a claim resting on a member's shape that later moved. It is not. File 33's own probe (`33_
probes/probe_5_direction_enters_the_key_iff_the_lattice_opens.rs:124-157`) parameterises bias as a
general rational (`bn/bd`) from the start, sweeps `bd` up to 2 in its cross-check (`predicates_match_
reality`, lines 342-370), and states the predicates "over a numeral whose adjustment (quantum) and bias
are rationals" in its own header comment, written before the design's shipped type agreed. The value-
level algebra was ahead of the type-level encoding on this one question, and rebuilding the probe fresh
against the current pin confirms it still compiles clean. This is worth reporting exactly because it
looked stale and was not: a re-derived "still holds" is worth more than an unexamined one, per this
dispatch's own instruction, and it is also the sharpest illustration in the review of the mechanism
section 3 names, because the reason it survived is the same reason the Bias defect happened at all,
approached from the other side.

**The reopened item.** File 01's `FullRange<1>` finding (`01:260-270`: `FullRange<1>` reduces to `2/1`,
value-equal to `Unit`, so a membership check keyed on which constructor a numeral names rather than what
value it denotes misses it) is not a live bug against the current shape, because the two-constructor
axis it was found against (`Adjustment = Unit | FullRange<F>`, `11:159`) no longer exists; D69 plus the
value-uniqueness obligation replaced it with a single value-unique encoding where there is, by
construction, exactly one type per rational value (`40:428-448`). But the open question the consolidation
already carries, whether `FullRange` survives as its own named constructor or gets reduced into a bare
ratio (`40:685-687`), determines whether file 01's finding is *dissolved* (the reduced-into-a-bare-ratio
answer, where there is structurally nothing left to collide) or *reopens as an unstated proof obligation*
(the survives-as-its-own-constructor answer, where value-uniqueness now additionally requires proving
`FullRange`'s own reduction agrees with `Adjustment`'s `Reduce` on every value where both are defined,
which nothing in the review states as a requirement). This connection between an already-open item and
the value-uniqueness discipline that would decide it had not been made before this file, as far as I can
find.

## 1. The sweep, and how it was run

The dispatch's brief is specific about what to look for: claims established before D69 (`30b`, argued by
files 27 and 28, both of which formed their own reading of the coordinate change independently before op
ratified it) and never re-derived after. I ran the sweep three ways, because a single grep for "dyadic"
would have found the words without finding the claims that depend on the *fact* without naming it.

**By vocabulary.** `grep -n "dyadic\|power of the radix\|width chain\|Adjustment = Unit\|integer
adjustment" *.md`, restricted to files before 27 (the pre-D69 stretch), then checked each hit against
whether a file 27 or later dispatch's own text engages the specific sentence, by line-number citation, or
merely inherits its conclusion. This found the `FullRange<1>` thread (files 01, 02, 07, 11, 12) and
confirmed the membership incompleteness finding (`01`, `02`) was properly retired by the total rewrite of
membership in section 1.6 (files 27, 28, 39), which cites neither file by number but replaces the entire
mechanism the finding was about, which is the correct way for a pre-D69 finding to stop mattering: not by
silent survival, but by the ground it stood on being rebuilt.

**By claim structure.** Every sentence in the consolidation asserting a property over a *list* of named
cases (operations, presets, format pairs) is a conjunction, and a conjunction's evidence class is only as
strong as its weakest conjunct. I read every such list sentence in `40` section 1 and checked, for each
member, whether a file 27+ dispatch names a compile against that specific member or only against a
sibling. This is what found the overflow-band claim: `40:178-180`'s four-member list had two compiled
members (multiplication, and division as of `43`, itself a correction) and two uncompiled ones carried
by the same sentence's own authority.

**By cross-workstream comparison.** The review runs two kinds of dispatch that do not automatically talk
to each other: value-coordinate work (the algebra, files 33/34/37, reasoning about real numbers) and
type-coordinate work (the encoding, files 34-36, choosing what Rust type represents a value). I checked
every place the two touch the same quantity for whether the value-coordinate derivation's own assumed
generality matches the type-coordinate encoding's actual generality. This is what found the closure-
formula result in section 0 above, and it is the one search of the three that produced a "still holds"
rather than a defect, because in this instance the value side happened to be more general than the type
side rather than less.

None of the three passes found a fourth genuine defect. I read this as a real finding rather than an
incomplete sweep: the review's own discipline (every load-bearing claim carries a citation and an
evidence-class tag, `38`'s four-bin system) is doing real work, and the two known defects plus the one
new one are exactly the sentences where that discipline's own blind spot (described in section 3) is
present. I did not check every sentence in every file; I checked every sentence shaped like the three
patterns above, which is where a coordinate change would hide.

## 2. The closure formula, checked and confirmed current (compiled, `33_probes/probe_5` rebuilt fresh)

Restated here because it is the strongest evidence for the mechanism in section 3, not because the claim
itself needed a correction.

`33:265-266` states additive lattice closure holds "exactly when `bias / adjustment` is an integer," and
`33:269-271` states narrowed-multiplicative closure additionally requires "the adjustment itself to be an
integer, and the bias too." Both are carried into the third consolidation as ratified-shape-adjacent
prose (`40:291-293`), inside the same section (1.7) that, one paragraph away (`40:283-286`), explicitly
notes the design's `IS_EXACT`/`Total` correction and cites the Bias defect elsewhere in the very same
document (`40:469-479`, section 1.11). A reader of `40` in full sees, in one document, both "Bias is not
an integer" (section 1.11) and a closure formula whose English phrasing ("the bias too... an integer")
reads as though it assumed the opposite (section 1.7), with nothing connecting the two.

The connection does not need making, because it turns out not to be there. `33_probes/probe_5_direction_
enters_the_key_iff_the_lattice_opens.rs:141-157` defines both closure predicates over four free
integers, `qn, qd, bn, bd`, i.e. over `bias = bn/bd` and `adjustment = qn/qd` as independent rationals,
not over an integer bias divided by a rational adjustment. The header comment states this explicitly
(lines 124-126): "over a numeral whose adjustment (quantum) and bias are rationals." The exhaustive cross-
check against direct computation (`predicates_match_reality`, lines 342-370) sweeps `bd` from 1 to 2,
which is exactly the axis that would have exposed a bug if the derivation secretly depended on bias being
integer (`bd = 1` only): it does not, `bd = 2` cases (half-integer biases, the MATLAB witness's own shape)
pass identically to `bd = 1` cases. Rebuilt fresh against the current pin for this dispatch, the probe
still compiles clean, all assertions holding.

So "the bias too, an integer" in `33:270` and `40:292` is not a leftover assumption from when the shipped
`Bias` type was `Int`. It is describing a genuine mathematical condition (narrowed-multiplicative closure
requires the *value* of bias to be an integer multiple of the quantum, which is a fact about specific
numerals, not about which numerals the type system can name) that happens to read, out of context, like a
type-level constraint on every `Bias`. The English is ambiguous between "closure holds only for numerals
whose bias-value is an integer" (true, and what was derived and checked) and "the `Bias` type must be
integer" (false, and not what either the derivation or the probe claims). I recommend the next
consolidation disambiguate the sentence, not because the mathematics is wrong, but because it sits one
section away from the finding that makes the wrong reading available, and a claim that survives contact
with the truth only because a reader does not make the connection its own neighbouring section invites is
a documentation debt even when it is not a soundness one.

**Why this is informative beyond its own correctness.** File 33 derived and checked the closure
predicates over general rational bias in the same review-week that file 36 was, independently, deciding
`Bias`'s type by an unrelated argument (a sign-of-a-product observation, `36:220-222`, `40:469-479`) that
happened to land on `Int`. Neither file's own text engages the other's assumption about what bias *is*.
Had file 36 read file 33's probe header, "adjustment (quantum) and bias are rationals," before writing
"so `Bias` is a signed integer," the mismatch would have been visible three files and one checkpoint
earlier than file 39 found it by checking MATLAB's documentation instead. This is not a criticism of
either file individually; both did the job their own dispatch asked for correctly. It is a fact about the
review's own topology: value-coordinate work and type-coordinate work on the *same quantity* ran as
separate dispatches with no step that forced them to agree before either shipped, and only one of the two
happened to be wrong.

## 3. The overflow band's uncompiled members (compiled, `44_probes/probe_1`)

`28:229-231` (Leroy, the same file that co-argued D69) states the band is "empty for same-format addition
and inhabited for multiplication, division, mixed-format addition and every float operation," carried
into the current consolidation unchanged at `40:178-180`. This sentence predates every exhaustive check
run against it: files 30/31 checked same-format addition (empty, confirmed) and files 30/31/33 checked
multiplication (inhabited, confirmed, `33:255-259` reproducing the consolidation's own earlier "roughly
half" figure exactly). Division was reasoned, not compiled, until file 43 compiled it and found the
blanket claim wrong: empty for same-precision, inhabited only once precisions decouple.

Two members were left uncompiled after file 43: mixed-format addition and "every float operation." The
dispatch's own instruction that a conjunctive claim's checked members do not vouch for its unchecked ones
applies here exactly as it applied to division, and division had already shown that a member of this
specific sentence can be wrong. I compiled the addition member.

**Mixed-format addition can inhabit the band** (`44_probes/probe_1`, `_WITNESS_1`, `_WITNESS_2`). Operand
quanta `1/3` and `1/6`, destination quantum `1/4`, destination range `Vmax = 5/4`: the exact sum `0/3 +
8/6 = 4/3` lies strictly inside `(5/4, 5/4 + 1/8] = (1.25, 1.375]`, confirmed both by the search and by
an independent hand-derived rational comparison (`4*4 > 5*3` and `4*8 <= 11*3`). A second, independently
shaped witness (quanta `1/5` and `1/8`, destination quantum `1/6`) confirms this is not one parameter
choice's coincidence. A negative control re-running the same search shape at `d1 = d2 = dr` (same-format
addition) finds nothing, at three parameter choices, matching `28`'s own claim for the case `28` actually
checked.

**But it is not unconditional**, and this is the part `28:229-231`'s prose does not say and could not
have said without checking it. A sweep of 40 genuinely mixed (`d1 != d2`) `(d1, d2, dr, m)` triples splits
36 inhabited, 4 empty (`_SWEEP_TRIED == 40`, `_SWEEP_INHABITED == 36`, `_SWEEP_EMPTY == 4`, pinned
exactly). The four empty cases are structurally distinct from the 36 inhabited ones: all four have
`d2 = 2 * d1` (the pair `(3, 6)` tried in both orders, against four destination choices), meaning one
operand's quantum divides the other, so the pair is not genuinely two independent grids, it is
single-quantum arithmetic on `1/6` wearing a mixed-format label. Swapping in a non-dividing pair at the
same relative magnitude (`4` and `5`, neither a multiple of the other) at the same two destination choices
that emptied for `(3, 6)` inhabits the band both times (`_NONDIVIDING_PAIR_INHABITED_1`, `_2`). The
deciding structural fact, whether one operand quantum divides the other, is new; nothing in the review
before this probe names it.

**The correction owed, stated the way file 43 stated division's**: `40:178-180`'s mixed-format-addition
entry moves from "inhabited" to a per-format-triple statement, with the dividing-quantum degeneracy named
as the boundary. This is a smaller correction than division's (the band CAN be, and usually is, inhabited,
rather than being empty except in a special case), but it is the same shape of correction, for the same
reason: an assertion carried by its neighbours in one sentence, never independently checked, and wrong in
the specific way "unconditional" claims about a lattice-alignment condition tend to be wrong, at exactly
the boundary where the condition degenerates.

**"Every float operation" remains fully open.** Nothing in this file, or anywhere else in the review,
builds a `Specials`-carrying model numeral and checks the band against it (the consolidation's own open
list already says this, `43:319-322` addressing division's float path specifically; the general "every
float operation" phrasing in `40:178-180` is broader than that and has even less behind it). Given that
the two checked-late members of this same sentence (division, and now mixed-format addition) both needed
correction on inspection, I recommend against treating "every float operation" as a safe extrapolation
from "multiplication and mixed-format addition both inhabit it usually." It should read as unverified
until someone runs the model-float check the consolidation already names as owed.

## 4. `FullRange<1>`, dissolved or reopened depending on an answer nobody has given yet (reasoned)

File 01 (`01:260-270`) found that under the pre-D69 two-instance `Adjustment` axis (`Unit | FullRange<F>`,
`11:159`), `FullRange<1>` reduces to exactly `2/1`, the same value `Unit` denotes, and a membership check
keyed on which constructor a numeral names rather than on the value it denotes therefore misses it: sound,
but incomplete, in file 01's own careful phrasing. The talk this traces to already knew the boundary
(`F >= 2` with the `F = 1` case documented as a degenerate 1-bit UNORM format, per file 01's own citation
of the source talk); the spec dropped the guard when the constructor was renamed.

This is not a live defect against the current ratified shape, and the reason is worth stating precisely
rather than left implicit. The axis it was found against, a closed two-instance enum where one instance
(`FullRange<F>`) could coincidentally denote the same value as the other (`Unit`) without either
constructor knowing it, does not exist anymore. D69 plus the value-uniqueness obligation (`34b`) replaced
it with a single, sealed, value-unique rational encoding (`Nat`/`Pos`/`Adjustment`, `40:428-448`, `41`,
`42`) where, by the induction files 36/41/42 each ran and checked, there is exactly one type per rational
value with no exceptions. A membership check under this encoding cannot miss a "secretly dyadic" case the
way the old check could, because there is no longer a second constructor path for a value to arrive by.

But this is conditional on an answer the consolidation already flags as open and unbuilt: whether
`FullRange` survives the transition as its own named `Adjustment` constructor, or gets reduced into a
bare ratio with no distinct type of its own (`40:685-687`). I read the two branches as follows, reasoned
from the value-uniqueness machinery files 36/41/42 already built and checked, not further compiled here
because building `FullRange` itself is out of this dispatch's scope and is already correctly named as
unbuilt.

**If `FullRange` reduces to a bare ratio**, file 01's finding is dissolved by construction, not fixed. A
`FullRange<1>`-shaped adjustment is simply the rational `2/1`, spelled through the same `Reduce` machinery
every other adjustment goes through, indistinguishable at the type level from any other numeral whose
quantum happens to be `2`. There is nothing left for the finding to be about.

**If `FullRange` survives as its own named constructor**, file 01's finding reopens, in a sharper and
previously unstated form: value-uniqueness (`40:430-432`'s own obligation, "numeral encodings must be
value-unique as types, or a law about a numeral-producing operation splits into a true value half and an
ill-formed type half") now requires proving that `FullRange<F>`'s own reduction agrees with `Adjustment`'s
`Reduce` on every value where the two coincide, which is exactly file 01's boundary case restated as a
proof obligation the design's own stated discipline already demands but nobody has connected to this
specific open question. This is new: `40:685-687` names the fork as unbuilt but does not say what the
"survives as its own constructor" branch owes, and file 01's finding, twenty-nine files earlier, already
names precisely what it owes. I recommend the next member to open `FullRange` read section 1.11's
value-uniqueness statement and file 01 section 9 together, rather than either alone; each supplies half of
what the other needs to be actionable.

The membership incompleteness half of file 01's own finding (the "`Adjustment = Unit` as a dyadic-
membership proxy" framing, `01:263-264`, echoed in `02:364` and `07:333`) is, separately, correctly
retired. Section 1.6's total rewrite of membership (files 27, 28, 39, the finest-inhabited-system reading)
replaces the entire mechanism this finding was about; the new mechanism does not use `Adjustment` type-
equality as a membership proxy at all, so the specific incompleteness file 01 found has no object left to
be incomplete about. Nobody droplisted it by name, but the ground it stood on was rebuilt out from under
it in the ordinary course of later work, which is what a pre-D69 finding retiring correctly looks like,
in contrast to the two cases in section 0 where the ground moved and the sentence did not notice.

## 5. The mechanism

Two properties of the review, acting together, are what let a claim survive its own foundation moving.
Neither is a mistake by any single file; both are properties of the review's *process*, visible only by
looking across many files at once, which is exactly why the dispatch that finds them has to be the kind
of dispatch this one is rather than an ordinary member's file.

**First: a consolidation compresses a claim's conclusion and discards its coordinates.** `40:178-180`
reads as one flat sentence asserting a property of four named operations. Nothing in its own text says
that two of the four members were exhaustively checked and two were carried on the strength of their
neighbours; that information lived in file 28's own prose (`28:229-231`, itself informal about which
members it had actually reasoned through versus asserted by pattern) and was lost the moment the sentence
was quoted into a later consolidation as settled prose. The review's own four-bin evidence discipline
(machine-checked-by-construction, machine-checked-by-bounded-exhaustion, measured, reasoned-without-
artifact, `38`, restated `40:29-32`) records evidence class at the *sentence* level, which is the wrong
granularity for a conjunction. A sentence asserting a property of four things is four claims wearing one
evidence tag, and the tag reports the strongest member's class, or an average impression, rather than
each member's own.

**Second: a ratified coordinate change has no automatic query for what depends on it.** D69 is recorded
once, at `30b`, and its consequence for the identity contract is stated once, in section 1.1 of every
consolidation since. But nothing in the review's own machinery asks, at the moment of ratification or at
any point after, "which currently-carried claims were derived using the coordinate this just changed."
The two known defects and the one this file adds were each found by a member who happened, for an
unrelated reason, to be standing next to the specific sentence that had gone stale: file 39 was checking
the design against MATLAB's own documentation and noticed `Bias` could not represent a value MATLAB
documents as legal; file 43 was building division from scratch and needed the prediction to be true or
false rather than merely cited; I was told, by this dispatch's own brief, to go looking. Absent that
brief, the overflow-band finding in section 3 would have waited for whichever future member happened to
need mixed-format addition to work, the same way the division prediction waited nineteen files for
someone to need division to work. Luck is not a discipline, and a review that has now produced three
instances of the identical failure (counting this file's finding as the third, the closure formula as a
near-miss that checked out by luck of a different kind) should stop treating each discovery as a one-off
and start asking what would make the next one findable on purpose.

**The two properties compound.** A conjunction's evidence tag hides which member is weak; a coordinate
change gives no signal about which sentences to re-examine. Put together, the review can carry a false
claim, correctly tagged as some flavour of "checked," through an arbitrary number of consolidations,
because nothing in the process ever asks the one question that would catch it: not "is this claim
checked" but "is this claim checked *against what the design currently is*."

## 6. The discipline

The dispatch asks whether the checked-versus-reasoned axis (file 38's four-bin discipline) is the right
one for this problem, or whether the right axis is something else. I think it is the wrong question
framed as a choice, because the two axes are not competitors. Checked-versus-reasoned answers "is there
an artifact behind this sentence." It says nothing about whether the artifact is still about the same
object, which is exactly the gap section 5 names. A checked claim can be stale (file 33's closure formula
would have been, if its probe had swept only integer bias); a reasoned claim can be perfectly current. The
missing axis is orthogonal to evidence class, not a replacement for it, and the fix is to add it, not to
argue about which of the two existing bins a stale claim belongs in.

**The proposal: every load-bearing claim's evidence tag carries a second, short field naming which
ratified coordinate decisions its derivation used.** Concretely, alongside the existing "Compiled (probe
N)" / "Reasoned, not compiled" convention this review already uses at nearly every claim, add "grounded
on: D69" or "grounded on: 34b (value-uniqueness)" or "grounded on: Bias is rational (41/42)" where the
claim's own derivation actually leans on that decision being the current one. This costs nothing new
mechanically; it is the same citation discipline the review already runs (`file:line` on every judgement)
applied to one more thing a citation can name: not only where a claim comes from, but what it assumes is
still true.

The payoff is mechanical, not aspirational. The moment a future checkpoint overturns or generalises a
named decision (the way `30b` overturned D69, the way `41`/`42` generalised Bias), the very next dispatch
can `grep -n "grounded on: <the decision that just moved>"` across every numbered file and get, in one
command, the exact worklist this dispatch had to reconstruct by hand across three passes and twenty-nine
files. That worklist is precisely the input to a proof-repair pass in the sense my own field uses the
term: not "find every claim that might be wrong," which is what this dispatch had to do, but "find every
claim that depends on the thing that just changed," which is a bounded, cheap, mechanical query once the
dependency is written down at the point the claim was made rather than reconstructed later by a member
reading the whole history.

**A second, smaller addition closes the conjunction-granularity gap from section 5.** A claim asserting a
property over a *named list* of cases (operations, presets, format pairs) states, per member, whether
that specific member was checked or carried by analogy to a sibling. `40:178-180` would read, under this
discipline, as four short clauses rather than one flat sentence: "empty for same-format addition
(compiled, 28/30/31); inhabited for multiplication (compiled, 30/31/33); inhabited for division except
same-precision (compiled, corrected, 43); [mixed-format addition and float operations, per their own
current status]." This is more words, and it is exactly as many words as the truth already required; the
flat sentence was not shorter because it said less, it was shorter because it hid which of its four
clauses had earned the tag it wore.

Neither addition requires new tooling. Mockspace's own `## CHANGE:` claim discipline in shipped design
rounds already establishes the principle that a claim about state carries its own verification command
(`cl-claim-sketch-discipline.md`); this review, being research prose rather than a locked changelist,
has been informally doing the same thing with `file:line` citations and evidence-class tags since its
first file. The "grounded on" field is that same convention, extended by one more piece of information,
adoptable starting with the very next file.

## 7. What this file does not decide

**Whether `40:178-180`'s mixed-format-addition correction should read "inhabited except when one operand's
quantum divides the other" as a closed-form law, or merely as a per-format-triple statement pending a
proof the way division's own correction was left**, is not decided here. Probe 1's sweep is exhaustive
over its own small parameter windows and finds the dividing-quantum condition sufficient for emptiness in
every case tried, but forty triples at denominators up to six is a sweep, not the closed-form membership
predicate file 43 built for division; a member with the budget should attempt the analogous algebraic
proof before the sentence hardens into the next consolidation.

**Whether "every float operation" is true, false, or a mix depending on which operation, remains
completely open.** This file rules nothing in or out about it; it only removes the false comfort of
reading it as safe by association with its now-partially-corrected neighbours.

**Whether `FullRange` should survive as its own constructor or reduce into a bare ratio (`40:685-687`) is
not decided here**, and was already correctly flagged as an open, unbuilt fork before this dispatch.
Section 4's contribution is naming what each branch owes, specifically the value-uniqueness proof
obligation the survives-as-its-own-constructor branch inherits from file 01's finding, not choosing
between the branches.

**The "grounded on" proposal in section 6 is not retrofitted onto the review's existing 43 files.** Doing
so by hand would itself be exactly the kind of manual archaeology this file's own three passes required,
and is disproportionate to what the proposal is for, which is making the *next* coordinate change's blast
radius findable in one command, not auditing the last one a second time. I recommend adopting it forward
from the next file, and, if op judges the two known defects plus this file's third instance sufficient
grounds, backfilling the "grounded on" field only on the specific sentences this file and files 41/42/43
have already identified as touching D69, the value-uniqueness obligation, or Bias's shape, which is a
short, named list rather than a full-review sweep.

**Whether a fourth stale claim exists that this file's three search passes missed is not foreclosed.**
The sweep in section 1 is thorough against the three patterns it targeted (vocabulary, conjunctive claim
structure, cross-workstream comparison) but is not a claim of completeness against every sentence in
forty-three files. Section 6's proposal is exactly the mechanism that would make the next instance,
whichever pattern it hides behind, findable without a member having to run the same three passes by hand
again.

## 8. Standing

Two claims the review already knew were stale (`Bias = Int`, the division prediction) are joined by a
third of the identical shape (the overflow band's mixed-format-addition and float-operation members),
found on purpose rather than by accident, which is the point of this dispatch existing. A fourth candidate
(the closure formula) checked out clean on inspection, and is reported anyway, per the dispatch's own
instruction that a re-derived claim is worth stating even when its answer does not move, because the
route by which it stayed correct (value-coordinate work running ahead of type-coordinate work, by
accident rather than by any check that forced them to agree) is itself the clearest available illustration
of why the other three went wrong. A fifth, already-open item (`FullRange` survival) gains a sharper
statement of what one of its two branches owes, connecting an unbuilt fork to a proof obligation the
review's own value-uniqueness discipline already states but had not yet been pointed at this specific
question. None of the five touches a ratified call; all five touch unratified prose sitting in the same
carried-forward, evidence-tagged, but coordinate-blind position. The mechanism section 5 names is not
particular to D69, and the discipline section 6 proposes is not particular to fixing what D69 broke; it
is the general shape every future ratified coordinate change will need, and the cheapest time to adopt it
is before the next one, not after the fourth accident.
