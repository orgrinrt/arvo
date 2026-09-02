# 246. The two promotions, and what each of them actually seconds

Seat 246. Dispatched on three corrections owed against
`244_orchard_consolidation_admission_and_the_number_system.md`, called `244` below, as found
by `245_ringer_entailment_check_on_the_admission_consolidation.md`, called `245`. The sources
under both are `241_kiselyov_admission_is_a_resolution_not_a_verdict.md`,
`242_what-admits-a-number-system.md` and `243_seat242_the_resolution_has_no_second_arm.md`,
called `241`, `242` and `243`.

Nothing here moves a registry row. Per op's `87` as carried in `AGREEMENTS.md`, the canon is
written once at the end; this is panel research and has no standing beyond what its
instruments show. `244` is the historical record and is not rewritten: every correction below
lands here.

## Gates, run before the assigned work

**Canon gate: aligned.** Checked against the typed registry at `mock/registry/*.toml`, which
`mockspace.toml` declares as `canon_paths`. My assigned work is checking a consolidation's
claims about instance counts, a term census and an instrument, and answering admission
questions from ratified text. Nothing in it edits a row, proposes an edit, or fills a question
the canon reserves. The one thing I looked hardest at is whether recording a promotion is
itself a canon act the panel may not perform mid-flight: it is not, because
`proposal.standing` records how many independent instances back a claim and a recording is a
count rather than a ratification, and `ruling::the_panel_finishes_the_canon_without_him`,
`rung = ratified`, `ratified_by = op`, licenses deriving an answer from the intent and putting
it through two independent agreements. I put nothing through as settled below; I say per claim
what tier it holds at and what it still owes.

I ran one further check the gate does not require and my claims do need. My worktree is later
than the tree `244` measured at, and `245` was caught out by exactly that: it re-ran one of
`244`'s probes at its own HEAD, got 23 where the committed output said 22, and had written it
up as a defect before finding the cause was the tree. So every row this file rests on is
compared byte for byte between `800e120a` and my tree. **Twenty rows, all SAME**, with a
control showing three rows were added elsewhere between the two trees so the comparison can
see a change. Evidence: `246_probes/the_rows_i_depend_on_have_not_moved.sh`,
`output_rows_have_not_moved.txt`.

**Test gate: run, and the suite is honest.** `cargo test --manifest-path mock/Cargo.toml` at
my worktree: 85 in `arvo-format`, 3 `compile_fail` arms, 14 in `matlab_fi_parity` with 1
ignored, 19 in `arvo-placement` with 1 ignored, 10 in `arvo-strategy`, 4 doctests and 4
`compile_fail` doctests. 137 passing, 2 ignored, 0 failing.

Both ignored tests are correctly formed catalogue-reds carrying a reason that names the
unbuilt thing:
`the_carrier_is_not_a_function_of_the_access_width` names the second packing rule, and
`some_shipped_mode_is_matlab_nearest` names two open question rows by slug. Neither is a
test weakened to green.

I read the bodies in the surface this file rests on, which is `Slots::ADMITTED` and the two
doctests attached to it at `slots.rs:93` and `slots.rs:114`. They are a matched pair and they
are real: the `compile_fail` arm declares an inverted range and forces the const, and the
control declares the same shape with the bounds in order and asserts the call returns. Without
the control the refusal would not be attributable to the obligation. `244` and `245` both
confirmed the weakness in `the_format_inventory_admits_a_member_this_crate_does_not_know_about`
at `tests.rs:391`, which is now three confirmations of one thing and which I do not rest on.

---

## 0. Two premises of my own brief, checked before anything else

**One is false, and it is the one that would have shaped section 3.** My brief says of
`244`'s L4 term census: "Three of four reproducing under plain substring is what identifies the
method."

**It identifies nothing.** Three of four also reproduce under trailing-boundary counting, with
a *different* one of the four failing. Two conventions each score three of four on disjoint
failure sets, so three-of-four is not identification; it is the shape you get when a census was
not run as one census. Measured, with four controls, in section 3. The premise came from
`245`, which reached it honestly from two conventions rather than four, and my brief inherited
it. It should not be cited again.

**The other is true and is narrower than it reads.** `245` found that `244`'s anchor-diff
pattern cannot match a registry row slug. Verified: I re-ran `244_probes/anchor_diff.sh` and it
reproduces its committed output byte for byte, and its pattern's three alternatives contain
nothing that could match a qualified row name, which is a namespace and a slug
joined by a double colon. What the brief does not say, and what `244`'s own
script header does say while `244`'s section 7 prose does not, is that the pattern has **two
further defects**: it is unanchored, so it manufactures two phantoms, and it has no hyphen in
its filename class, so `242_what-admits-a-number-system.md` is invisible to it in both
directions. That is three defects in one pattern, and section 4 fixes all three.

---

## 1. I am 241's persona, and that bounds what I am allowed to conclude

`241` is a Kiselyov seat. I am a Kiselyov seat. `241` itself set the standard I am now held to,
declining to count `08` for its Q22 route on the ground that "this is one persona agreeing with
itself and I do not count it."

**So nothing below where I agree with `241` is a second instance for anything `241` claimed.**
I say it here rather than leaving it to be found, because the dispatch that produced this file
asked me to establish whether `241` is an independent second instance, and an agreeing Kiselyov
answering that question is the one shape that would look like corroboration and be none.

What the constraint does not touch, and what this file is therefore built out of: dates, commit
hashes, section hashes, source reads, exhaustive classifications over a five-item set, and
refutations. None of those is an agreement. Where I nonetheless have to exercise judgement, I
mark it and I mark the persona hazard with it.

The panel's own convention supports the reading rather than my own memory of it: the persona is
in the filename, and `246_probes/the_persona_attribution.sh` extracts it under two controls, a
file known to be Kiselyov and a file known not to be, because an extractor that answers
kiselyov to everything passes the first control and says nothing.

---

## 2. `244`'s L2: the sitting has two candidate promotions, and neither seconds its row whole

`244` L2 concludes: "So the sitting's one promotable result is a promotion nobody proposed."
`245` found a second candidate of the identical shape in the table `244` itself built. Both
findings are right about what they assert. **Both are wrong about the promotions being
available as stated**, and the reason is one thing neither looked at.

### 2.1 The second candidate holds, and here is the mechanical case for it

`245`'s candidate: `241`'s Q31 answer seconds
`proposal::membership_and_hosting_are_two_questions`, which stands at `one_expert` with
provenance `74::536` and `73::667` and is cited zero times by `241`, `242` or `243`.

I did not take `245`'s word for the route disjointness, because `244` named its own failure to
read `73` and `74` as the weakness in L2, and `245` named the same limitation against itself.
**I read them.** Three checkable things, in `246_probes/the_second_instance_is_blind.sh` with
its output committed beside it.

**Route disjointness, by date rather than by argument.** `241`'s route runs through R2,
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`, whose
`provenance` field names `226` and `225`. Those two files landed on 2026-08-31 at 20:52 and
21:41. `73` landed on 2026-08-09 at 10:29 and `74` at 11:05, twenty-two days earlier. So `73`
could not have used R2, and this is settled by the repository rather than by anybody's account
of what they read. Control: the same comparison run the other way round returns false, so it is
a test and not a constant.

**The blind body was never edited.** `241` asserts its pre-reconciliation body was committed
before it opened anything under `mock/research/`. Reading leaves no trace, so that assertion is
not checkable, and I do not check it. What is checkable is the weaker and sufficient thing.
Section 6, the Q31 answer, first appears at `b7d7289f` and carries the hash `e85576137983` at
every commit from there to the file's head. The whole pre-reconciliation body carries
`cc293b3c98cc` at `a664fffb` and the same at the head once the appended horizontal rule and its
blank lines are stripped, the entire raw difference being those three lines. **`241` appended a
reconciliation and an erratum and edited nothing above them.** Control: the same instrument
reports section 5 changing between `b7d7289f` and `a664fffb`, so "unchanged" is a measurement
rather than a constant.

**And the passage the proposal comes from is not on `241`'s reading list even after the blind
phase.** `241`'s reconciliation names what it read: `74` sections 3.1 and 3.4 and the option
pass on Q30, `73` **section 7**, `08` section 4.5, and two paragraphs of `71`. The proposal's
provenance points at `73::667`, which is M1 in `73`'s section 12, and at `74::536`, which is
N10 in `74`'s candidate-sentence list. Neither is section 7 and neither is `74` 3.1 or 3.4. So
there is no path, blind or post-blind, by which `241` could have inherited it, which is
consistent with `245`'s observation that Q31 is absent from `241`'s "What this does to my own
answers" section entirely.

**One thing I can settle that neither `244` nor `245` could, having read `74`.** N10's own
entry reads "Permanence and equivalence: pass (`73:667-675`). ONE EXPERT on the split". N14, the
other proposal's source, reads "pass (`73:705-711`). ONE EXPERT". **`74` is the consolidation
recording `73`'s sentences, not a second derivation of them.** So both proposals' two-file
provenance lists are one instance wearing two hats, which is exactly what their `one_expert`
standing already says and which nobody in this sitting had confirmed from the sources. A reader
counting provenance entries would have counted two.

### 2.2 The seam nobody crossed: the two rows are written in a vocabulary `241` cannot speak

This is the finding, and it is a connection between two things `244` itself carries in
different sections without joining them.

`244`'s A9 records `241`'s reconciliation finding that **`coordinate` means two different
things in this panel**: the ten identity coordinates R3 counts, and the five chain components
`74` section 3.1 uses. `241` states the consequence in its own words: "a canon sentence reading
'admission returns which coordinate the candidate fixes' is true in `73`'s vocabulary and false
in R3's, because in R3's vocabulary a Gray code fixes none of the ten."

`244`'s L2 then proposes promoting a row whose `says` field is, verbatim:

> The useful question is which choice in the chain the candidate fixes and whose it is, because
> most things that are not systems are not outside the concept: they are choices at a later
> coordinate, and an encoding, a container, a stride and a housing are all answers rather than
> rejections.

**That is the sentence `241` says is false in the vocabulary `241` argues in.** The row says "in
the chain" explicitly, and it enumerates an encoding, a container, a stride and a housing.

Reading `73` section 7, which is M5's basis and which neither `244` nor `245` opened, settles
how load-bearing the enumeration is. It is not a rider. Section 7's whole content is the index:

> Gray code, two's complement, offset binary, signed-digit: **coordinate 4**, the encoding.
> [...] `Cold`'s stride, alignment, a wider housing: **coordinate 5**, the container [...]
> A candidate that fixes coordinates 1 through 3 is a system; one that fixes 4 or 5 is a
> realisation of one; one that fixes none of them is outside the concept entirely.

So `73`'s procedure *sorts* candidates by which of five components they fix. `241`'s procedure
returns a total assignment of ten constants, and by `241`'s own reconciliation every one of the
ten sits inside the **first two** of the five. I confirm the containment from source rather
than from `241`: `Ambient` declares `RADIX` and `SIGNED`, which is component 1; `Quantum`,
`Slots` and `Format` declare the remaining eight, and `slots.rs`'s own module documentation
says the set is "the slot times the quantum at its magnitude", with the phase in `format.rs`,
which is the affine predicate R1 ratifies as membership in the representable set, component 2.
Component 3, the reduction, lives in `adapt.rs` and has no coordinate among the ten.

**Therefore `73`'s discrimination is not merely differently worded in `241`'s address space; it
is inexpressible there.** Every candidate `241`'s resolution accepts fixes components 1 and 2
and nothing else, so no candidate it accepts can come back as "a realisation" or "outside", and
the three-way sort `73` section 7 exists to perform collapses to a single answer.

And R1 is not neutral about this. `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
ratified through `ruling::the_format_spine_is_canon`, says "Adaptation choice and encoding are
realisation, observable in computed values and in pattern-level properties respectively, and
not part of identity." So the ratified canon has already placed two of the row's four
enumerated objects outside identity.

**None of this makes `73` wrong.** `241` says so itself and it is right: `73` predates R1's
encoding clause, so this is a seam that opened underneath it. What it makes wrong is promoting
the row *as written* on the strength of `241`.

### 2.3 What each row is actually seconded on, clause by clause

The rule is that two instances agree about the intersection of what each claims, never the
union. Applied to a row's own sentence rather than to its dimensions:

**`proposal::admission_returns_a_coordinate_rather_than_a_verdict`, two clauses.**

| clause | `73` (one expert) | `241` | seconded |
|---|---|---|---|
| asking whether a candidate is a system is the wrong question; the useful output is an address | yes, section 7 | yes, section 2 | **yes** |
| an encoding, a container, a stride and a housing are answers at a later coordinate | yes, section 7, and it is the section's whole content | **no**, and `241`'s reconciliation says a Gray code fixes none of the ten | **no**, and `241` is a counter-instance in its own vocabulary |

**`proposal::membership_and_hosting_are_two_questions`, four clauses.**

| clause | `73` (one expert) | `241` section 6 | seconded |
|---|---|---|---|
| the two are different questions | yes, section 1 | "Two, and the second is not a second admission procedure" | **yes** |
| the first is structural, answered by locating on the chain of choices | yes, in chain vocabulary | "being a number system is fixing the coordinates. That is target-free", in identity vocabulary | **partly**: the shape is seconded, the address space is not the same one |
| the second is about residue at runtime, what a value at rest may carry | yes, and it is `68` section 5's three residue clauses | "a predicate over the values those coordinates take, against a particular target's realisation ladder" | **no**: a different index, and section 5.1 measures that `241`'s index is not the one its own evidence supports |
| a system the implementation cannot host is still a system | yes | "a width of 63 bits is a perfectly good coordinate assignment that this stack cannot carry" | **yes**, and independently constructed |

So: **two candidate promotions, four clauses seconded between them, and three not.** The
corrected form of `244`'s sentence is not "at least one, and a second candidate sits
uninspected", which is `245`'s form and is a count. It is:

**The sitting produced no promotion of a row as written. It produced independent second
instances for three clauses spread across two rows, and a counter-instance to a fourth. A row
is promoted whole or not at all, so what is available is a split or a rewrite of the two rows
into clauses that can carry a standing each, and neither of those is a promotion and neither is
mine to perform.**

### 2.4 The dimension intersection, over values, and it is empty for both pairings

Stated as the discipline demands, per instance rather than per convergence.

**`73` M1 and M5.** Both rest on `73` sections 1 and 7. Section 1 is a taxonomy over four prior
candidate admission tests plus a two-by-two exhibited by naming nine example systems; section 7
is a composition of `67`'s telescope, `70`'s ownership key and `71`'s crossing classes. `73`
ships nine committed instruments, `p0`, `p1`, `p1b`, `p1c`, `p1d`, `p2`, `p3`, `p5` and `p6`,
and **sections 1 and 7 cite none of them**: the sections that do are 2, 3, 4, 6, 8, 9, 11, 12,
13 and 14. Control: the same per-section counter returns 3 for section 2, so a zero is a fact
about the section. So M1 and M5 vary nothing. Axes swept: none.

**`241` sections 2, 6 and 8.** Section 8's Q30 answer is a reading of `slots.rs`, and `241`'s
own predicate for it says so: "Stated as a reading of shipped source rather than a
measurement." Section 6's Q31 answer carries **no predicate at all**: `241`'s section 10 has
five entries and not one of them mentions hosting or Q31. Axes swept: none.

**Intersection over values, both pairings: empty.** Neither pairing agrees about any measured
region, because neither side of either pairing measured anything. They converge on an argument.
That is what `244` said about L2 and it is correct; it holds identically for L2', and it is
worth saying that "empty intersection" here does not mean the two instruments disagreed. It
means there is no instrument on either side, so the convergence is over text and reasoning and
carries exactly the weight of two people reading two different ratified rows and reaching
compatible sentences.

The predicate a reader can gate on:

```
the Q31 split claim, clauses one and four, holds for:
  crate = arvo-format, toolchain = nightly-2026-05-28, edition = 2024, threads = 1,
  tree = a12d4d5d, instruments = none on either side
```

Threads is listed at 1 and nothing here was run at more than one, which is correct for all of
it: these are text, registry and repository facts.

### 2.5 The caveat `245` left open on the Q31 row is stale, and it is closed by op himself

`245` wrote: "The proposal's own `note` says 'the hosting half is a different author's and is
conditional on an open question of op's.' I did not track down which open question that is; it
may bear on whether the hosting half is ready to be raised."

**It is Q-B, and it is closed.** `74::541`, which is N10's entry and the note's source, says
"the hosting half is `68`'s and is Q-B-conditional". `74::1039` names Q-B: "Are the long-standing
constraints op's intents". `85_op_no_runtime_checks_ever_and_stop_policing_law_shapes.md`
section 3 records op answering it in his own words, and its heading is "The long-standing
constraints were always intents, and the provenance trace was beside the point". `85:86` reads
"**Q-B is closed and the panel was wrong about it.**" It is in the registry as
`ruling::the_operating_constraints_are_intents_and_rules`, `rung = in_force`, `key = I14`,
carrying his verbatim quote.

So the row's `note` is stale in the direction that removes the obstacle. The hosting half was
conditional on a question op has since answered, and answered by correcting the premise rather
than by choosing an option. **A later seat should not treat the note as a live condition.** The
note itself is a registry-hygiene matter and I do not edit it; I record that it is false as
written and why.

---

## 3. `244`'s L4 term census: no single counting convention reproduces the four numbers

`244` L4 states that `archive/OLD_CANON_CANDIDATE.md` "carries 61 uses of `sealed`, 8 of
`value-unique`, 6 of `NonZero` and 6 of `AtLeastTwo`". `245` measured plain substring at
61/8/13/6, a word-boundary `NonZero` at 5, and concluded that neither reading gives 6 and that
three-of-four identifies plain substring as the method.

**Both halves of that are wrong, and the reason is that a count is a number and a convention
and only one of the two was ever written down.** There are four ordinary conventions, not two.
`246_probes/l4_term_census.sh`, output committed beside it:

```
term             substr   trailB    bothB    lines   244 says
sealed               61       61       58       58         61
value-unique          8        6        6        8          8
NonZero              13        6        5       12          6
AtLeastTwo            6        6        6        6          6

VERDICT: no single convention reproduces all four of 244's numbers.
         Per-term, the conventions that give 244's number are:
           sealed          substr trailB
           value-unique    substr lines
           NonZero         trailB
           AtLeastTwo      substr trailB bothB lines
```

`trailB` is a trailing token boundary only, `grep -oE 'T\b'`. `bothB` is a free-standing token.
The differences have a cause and the probe prints it: `sealed` appears 59 times bare plus
`sealed-` and `sealed-bound-plus-const-assertion`, and three times inside `unsealed`;
`value-unique` appears 6 times bare and twice inside `value-uniqueness`; `NonZero` appears 6
times with a trailing boundary, of which 5 are free-standing and the sixth is inside
`IsNonZero`, plus `NonZeroCarrier` five times, `NonZeroUsize` once and `NonZeroable` once.

Four controls, all required before the census prints. A planted fixture in which every
convention agrees, on which a convention must reproduce, or the verdict would be about the
instrument. The same fixture against a deliberately wrong claim set, which must reproduce under
none. A term known absent returning zero under all four. And the conventions being required to
disagree on at least one term, or they are not four instruments.

**So the corrections owed are these.** `244`'s `NonZero = 6` is not wrong; it is uniquely
reproducible, under trailing-boundary counting. `245`'s "neither reading gives 6" is wrong, and
so is its identification of plain substring as the method, since `value-unique = 8` is not
reproducible under the convention that gives `NonZero = 6`. **The defect was never a number. It
was that four numbers were reported as one census and were taken under at least two mutually
inconsistent conventions**, which is invisible until somebody runs all of them and is exactly
what a committed script would have exposed at the time. Severity is unchanged from `245`'s
reading: the census corroborates L4 and nothing in L4 depends on it.

---

## 4. The anchor instrument rebuilt, and the corrected accounting

`244`'s section 7 reports 23 source anchors and 30 in the consolidation, with 3 lost and 10 new.
`245` established that the pattern cannot see a registry row slug and built a slug-only diff
beside it. Neither merged the classes, and `245`'s slug pattern has its own defect: its
namespace list is hand-written and holds 8 of the 10 tables the registry actually declares,
omitting `strategy` and `topic`.

`246_probes/anchor_diff_v2.sh` fixes all four things.

- **The namespace list is derived** from the registry's own `[[table]]` headers rather than
  typed, so it cannot go stale when a table is added. It resolves to ten.
- **Every pattern is anchored** at a non-token character, which kills both phantoms `244`
  disclosed: `topics.toml` out of `law-the-later-topics.toml`, and
  `242_the_resolution_has_no_second_arm.md` out of `243_seat242_the_resolution_has_no_second_arm.md`.
- **Hyphens are legal in a filename stem**, so `242_what-admits-a-number-system.md` is visible.
- **Both classes are reported together**, which is what an accounting is.

Five controls, and the census is refused unless all five hold. Both sides of both classes
nonempty. A slug planted in the sources only, which must come back lost. A slug planted in
`244` only, which must come back new. **A slug planted in both, which must come back carried
and appear in neither difference**, and this is the one a nonempty-both-sides control cannot
catch: a classifier that calls everything lost passes the first two and fails this. And a
phantom control requiring that v1 still produces both phantoms while v2 produces neither, since
otherwise the claim to have fixed them is not measured.

The corrected accounting, pasted from the run:

```
--- COMBINED, which is the corrected accounting ---
  in the three sources : 36
  in 244               : 55 (section 7 excluded)
  LOST  count = 8
  new   count = 27
  carried count = 28

--- v1's own numbers, reproduced here for the comparison ---
  v1 sources : 23      v1 244 : 30
  v1 lost    : 3       v1 new : 10
```

**The eight losses are `244`'s three plus five registry row slugs**, and the five are the ones
`245` named:
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`,
`ruling::the_canon_does_not_police_what_shape_a_law_takes`,
`ruling::the_option_set_is_not_a_boundary`,
`ruling::the_panel_finishes_the_canon_without_him`,
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`.

I read each in `241` rather than taking `245`'s characterisation. All five are `241`'s
methodological apparatus, and `245` is right that `244` does not silently reuse any of them.
**I add one consequence `245` did not draw.** Those five are the ratified ground on which `241`
declines to pick a letter for four of its six questions and answers with a shape instead:
`there_is_no_universal_answer_take_the_win_and_gate_it` carries op's own words,
`behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` is the ratified
precedent for closing such a question by dissolution, and `the_option_set_is_not_a_boundary`
licenses answering outside the recorded options. A reader of `244` alone has `241`'s dissolution
move with none of its authority behind it, and `244`'s section 3 presents four of the six back
as open contests. That is not a wrongness in `244`; it is what the loss costs.

**Two of `244`'s ten "new" anchors were one anchor and one phantom.** v1 reported `topics.toml`,
which is a phantom; v2 reports `law-the-later-topics.toml` **and**
`proposal-the-later-topics.toml`, two distinct files v1's unanchored pattern had collapsed into
a single false entry. And v2's file class reports two panel filenames v1 never
rendered correctly, each for a different reason: `242_what-admits-a-number-system.md` is
invisible to v1 because its stem carries hyphens, and `243_seat242_the_resolution_has_no_second_arm.md`
is what v1 turned into its other phantom, because `seat242` puts a digit run inside the stem and
an unanchored pattern starts matching there. So the file class goes from 10 new to 12, of which
all 12 are real, against v1's 10 of which 2 were phantoms and 2 real anchors were missing.

**What this does to the standing of `244`'s section 7.** Its sets were right about what it could
see and its counts were right about what its pattern extracted. Its sentence is what overreaches:
23 and 30 are stated as the anchors, and they are the anchors of one of the two classes a reader
follows, with the class that addresses the canon absent entirely. Since this panel's canon *is*
the registry, that is the half that matters most here.

---

## 5. Four findings outside the assignment, per the standing instruction

### 5.1 The shipped hosting predicate is indexed by this implementation, not by a target

`241` section 6: "Being hostable is a predicate over the *values* those coordinates take,
against a particular target's realisation ladder", and "A predicate over target-dependent facts
is target-indexed already." That is the price argument by which `241` reduces Q31's option 3
from "two words plus a quantifier over compilations" to "one admission and one residue
predicate".

The proposal it is claimed to second says something different:
"whether **this implementation** can carry one", "residue at runtime", "what a value at rest may
carry". Implementation, not target. Nobody noticed, because both files say "two questions" and
the divergence is in the second word.

**Measured, and `241`'s own index is the one its own evidence does not support.**
`246_probes/hosting_is_not_target_indexed.sh` greps every non-test source file of
`arvo-format` for `usize`, `isize`, `target_pointer_width`, `cfg(target`, and pointer-sized
`size_of`. **Zero hits across eleven files.** Control: the same pattern finds one in the crate's
own test files, so the zero is a fact about the source. The slot coordinates are declared
`i64`, `i64` and `Width`, and `Width` is `pub struct Width(u32)`. Every bound in
`Slots::ADMITTED` is a fixed-width constant of this crate.

**So with `MIN`, `MAX` and `WIDTH` held fixed, no assertion in `Slots::ADMITTED` can change
truth value between two targets.** The refusal fires at codegen, which is a fact about a
compilation, and the source's own note says so; but firing at codegen is not the same as
*varying* by target, and `241`'s inference runs the two together. The predicate is
implementation-indexed. The proposal's word is the correct one and `241`'s is not.

This does not sink the Q31 pairing, and it is why the table in 2.3 has four rows rather than
one. It removes clause three from what is seconded and it removes `241`'s price argument
against option 3, which rested on the target index falling out of the residue predicate for
free. **It does not restore option 3 either**, since a predicate that does not vary by target
does not need a quantifier over compilations. What it leaves is a question nobody has asked:
whether hosting *should* be target-indexed, which is a design question about a stack that
currently has no target-dependent surface at all.

### 5.2 `Slots::ADMITTED` is not one kind of condition, and `241` says it is

`241` section 6: "`Slots::ADMITTED` in the shipped crate is exactly this and nothing else",
where "this" is the hosting predicate.

Extracted from source rather than typed, all five assertion conditions, exhaustively, and
classified by whether they compare a coordinate against another coordinate or against a
capacity constant of a carrier this crate chose:

```
  wellformed Self::MIN <= Self::MAX
  wellformed Self::WIDTH.count() >= 1
  hosting    Self::WIDTH.count() <= 62
  hosting    (Self::MAX as i128) - (Self::MIN as i128) < i64::MAX as i128
  wellformed (Self::MAX as i128) - (Self::MIN as i128) < (1i128 << Self::WIDTH.count())

  hosting: 2    well-formedness: 3
```

Four controls: at least one flagged, at least one not flagged, and three planted conditions of
known kind classified correctly, the not-flagged control being the one that catches a
classifier saying the same thing about every input. The classifier keys on a literal carrier
capacity, which would misclassify a well-formedness condition that happened to contain the
literal 64; that is a real fragility and it does not reach this census, because the census is
exhaustive over five items and I read all five myself and agree with every row.

**So `241`'s "exactly this and nothing else" is measured false**, and there is a third question
sitting in the middle of the two the row splits: an inverted range and a zero-bit width are not
things a different implementation could carry, they are declarations that are incoherent
anywhere. **Membership, well-formedness, hosting.** `73` section 1's two-by-two has no cell for
a candidate that is neither a system nor a non-system but an incoherent declaration, and the
shipped mechanism enforces all three through one const.

I do not propose a third word. `241`'s Q31 answer dissolving "which vocabulary governs" into
"two different kinds of sentence are being asked to share one word" is the right move and it
generalises: the finding is that the count is three rather than two, and where the third belongs
is open, section 9.

### 5.3 `241` declined an instance it was entitled to count, on a false attribution, and two files inherited it

`241` reconciliation, quoted from the file:

> **Q22's compositional route was already there, and reached by my own persona, which is not
> independence.** `08` section 4.5 says "the interval is then a pair of numerals, built above",
> which is my third route almost verbatim. It is an earlier Kiselyov seat, so this is one
> persona agreeing with itself and I do not count it.

**`08` is `08_knuth_what_the_one_format_concept_covers.md`.** It is a Knuth seat. The panel
holds nineteen Kiselyov-named files counting this one and seven Knuth-named files, and `08` is
in the second list;
`HANDLES.md`'s member-handle table records it under its own task id as a knuth file, cited by
the table rather than by line, because a ledger is still being written and a line moves under a
citation without breaking it. The likely cause is visible in
`08`'s own header, which names `06_kiselyov_where_a_numeral_is_inferred.md` in its reading list.

`244`'s C3 repeats the attribution: "`241` correctly declines to count `08` as a second
instance, being an earlier seat of the same persona." `245` checked C3 and did not check this.

**So `241`'s Q22 compositional route has an independent prior instance and was recorded as
having none.** `08` section 4.5 reaches it with its own measurement, twenty thousand interval
pairs on `U<3,3>` under addition and multiplication, outward rounding failing zero times against
1036 and 946 for the two wrong disciplines. Different persona, twenty-four days earlier,
different instrument, and `241` derived its version blind of it.

**An under-count is the direction nobody audits**, because every mechanism in this workspace is
built to catch a convergence that is really one instance and none is built to catch an instance
declined for a false reason. It survived three files. The correction is one line and I do not
extend it further than it goes: it bears on Q22's compositional route only, and `244`'s C3
separately demotes `241`'s Q22 *refusal* argument, which is a different claim and is untouched
by this.

### 5.4 `244`'s C2 is filed against a row that decides nothing about it

`242` section 9 files the one-word-two-tiers defect against
`question::are_the_level_hierarchies_the_same_cut`, and `244`'s C2 carries that forward as the
sitting's second contested item, "the tier count", calling `241`'s `coordinate` finding "the
same seam from the other side" and quoting `242` that the admission subject "does not close
until the level cut does".

**Four lists are in play and the two pairs are not the same pair.**
`246_probes/four_lists_not_two.sh`, output committed beside it, prints all four from their own
sources.

- **A**, `65`'s three levels: system, representation, format.
- **B**, `66`'s five levels: number, system, representation scheme, format, container.
- **C**, `67`'s five chain components as `74` section 3.1 carries them: ambient domain,
  representable set, reduction, encoding, container.
- **D**, R3's ten identity coordinates, extracted from the four trait declarations and asserted
  to come to exactly ten.

**Q19 asks about A against B.** `OPTIONS.md` states it in those words under its
`#q19-are-the-proposed-level-hierarchies-the-same-cut-or-different-ones` heading, and the row's own
`options` field carries the two lists verbatim. **`241`'s seam is C against D.** A and C share
nothing; B and C share exactly one word, `container`, which is why the two pairs are easy to
conflate and is the only thing they have in common. Controls: a list compared with itself comes
back whole, and the one shared word between B and C is reported rather than swallowed.

**So answering Q19 would close nothing about the word `coordinate` in the two admission
proposals**, and the admission subject does not wait on it. That removes `244`'s C2 from the
admission sitting's dependency list, which is a subtraction rather than a finding against
anybody's reasoning: `242` pointed at the nearest row that looked right and `244` carried it.

**And the relation C against D is already settled, in three rows, which nobody has recorded.**
R1's ratified identity proposal puts adaptation choice and encoding, which are C's components 3
and 4, outside identity. R2's `says` puts the container derivation, C's component 5, at "a
placement rather than a semantics". `question::adaptation_in_identity_or_realisation` carries
an `answered` saying exactly that and naming the unfollowed edge that hid it. That leaves C's
components 1 and 2 as identity, and D refines precisely those two, which is `241`'s
reconciliation result now checked against the rows rather than taken from `241`.

**What is left open is a drafting convention and nothing else**, and no row asks it. Every
`the_number_system` question row whose text contains a phrase the question would be written in
comes to one, `the_ownership_key_as_a_structural_axis`, whose `asks` is whether the concept
adopts an ownership key as a structural axis, so the hit is the phrase and not the question.
Control: the same grep over the same rows finds `level` eight times.


---

## 6. The six admission questions, and what the canon already carries

The subject `244` was dispatched on. I state per question what the canon answers, what it
reserves, and what I establish, which for four of the six is nothing.

I re-derived `244`'s L1 table by a different instrument, awk over the raw `[[proposal]]` and
`[[question]]` records rather than `244`'s shell probe, and it reproduces exactly: five of six
carry a standing answer row, `are_set_valued_carriers_admitted` carries none, and
`what_the_admission_contract_asks_a_candidate_to_expose` carries two. That is an independent
second instance for L1's table, and it is not persona-blocked, since L1 is `244`'s finding and
`244` is an Orchard seat.

**Q20, `is_the_number_system_inventory_open`. Answered, and closed by ratification.** The row
carries an `answered` field resolving to `ruling::the_format_spine_is_canon`, `rung = ratified`,
`ratified_by = both`, through `proposal::the_concept_is_closed_and_the_inventory_is_open`. Both
cold seats found it and both declined to re-derive it. Nothing to add and nothing owed.

**Q30, `is_admission_a_predicate_or_a_location`. The canon reserves it; the shape clause of the
standing answer is now at two instances and its enumeration clause is not.** Section 2.3. What
the canon does answer, and what neither seat used: R1 ratifies encoding and adaptation choice
out of identity, which decides that two of the four objects the row enumerates are not
coordinates in the identity vocabulary. What it reserves: which vocabulary a canon
sentence about admission speaks, which section 5.4 establishes is **not**
`question::are_the_level_hierarchies_the_same_cut` and is not any row the registry carries. The
substance is settled by R1 and R2 together; only the drafting convention is unheld.

**Q31, `one_word_or_two_for_is_a_number_system`. The canon reserves it; two of the standing
answer's four clauses are now at two instances.** Section 2.3, and section 5.1 for the clause
that is not. What I establish beyond that: the split is three-way rather than two-way in the
shipped mechanism, section 5.2.

**Q21, `is_number_system_broad_enough_for_non_magnitude`. Reserved, and the standing answer is
still at one instance.** `245` checked whether either cold seat seconds
`proposal::the_concepts_edge_is_not_an_order_and_wrapping_is_the_test` and found no overlap. I
re-ran it independently: `wrapping` appears once in `241` and zero times in `242` and `243`, and
that one occurrence is at `241:732`, inside the reconciliation, reporting `74`'s measurement
rather than deriving anything. Positive control: the same greps over `73` return 14 and 5. So
`245`'s finding reproduces. **My one qualification, marked as a judgement and persona-hazarded:**
`241` and the proposal share a negative, that the concept's edge is not a breadth boundary, and
diverge on everything positive, since the proposal's content is that order-compatibility is a
property of the reduction and `241`'s is that the operation family is not a coordinate. A shared
negative is not an instance and I do not record it as one.

**Q22, `are_set_valued_carriers_admitted`. Reserved, with no standing answer row anywhere in the
registry**, which makes `241`'s answer unopposed rather than converged, as `244` correctly says.
What changes is section 5.3: the compositional route inside that answer has a prior independent
instance in `08`, wrongly declined. `244`'s C3 demotion of the refusal argument stands and is
untouched.

**Q29, `what_the_admission_contract_asks_a_candidate_to_expose`. Reserved, two standing answers,
both at one instance, neither cited.** `244`'s C5 reads them as not competing with `242`'s
fourth option and `245` confirms. I add nothing and I did not attack it.

---

## 7. Findings, each with its predicate

Per `ruling::a_predicate_lists_only_what_holds` and I13 as extended by op's `217`. An axis
listed with a value holds only there; an absent axis holds nowhere. Per `242`'s and `240`'s
finding that `dimension.toml` carries no axis able to hold a claim about the canon, the registry
and repository claims below are predicated on the tree, which is outside the declared grammar,
and I say so rather than smuggling it.

- **Every one of the twenty registry rows this file depends on is byte-identical between
  `800e120a` and my tree.** Registry claim at tree `a12d4d5d`, `threads = 1`. Bounded whole-set
  range over the twenty rows, exhaustively. Control: three rows were added elsewhere between the
  two trees, so the comparison can report a change. Evidence:
  `246_probes/the_rows_i_depend_on_have_not_moved.sh`, `output_rows_have_not_moved.txt`.

- **`73` and `74` landed twenty-two days before `225` and `226`, which are R2's provenance, so
  neither could have used R2.** Repository claim at tree `a12d4d5d`, `threads = 1`. Control: the
  same comparison run in reverse returns false. Evidence:
  `246_probes/the_second_instance_is_blind.sh`, `output_second_instance_is_blind.txt`.

- **`241` edited nothing above its reconciliation after committing it.** Section 6 carries one
  hash across four commits; the whole pre-reconciliation body carries one hash from `a664fffb`
  to the file's head, the entire raw difference being an appended horizontal rule and two blank
  lines. Repository claim at tree `a12d4d5d`, `threads = 1`. Bounded whole-range over the file's
  five commits, exhaustively. Control: section 5 differs across the same commits. Evidence: same
  probe.

- **No single one of the four ordinary counting conventions reproduces all four of `244`'s L4
  numbers, and each individual number is reproducible under at least one.** Panel claim at tree
  `a12d4d5d`, `threads = 1`. Bounded whole-set range over four terms and four conventions,
  exhaustively, sixteen cells. Four controls, all required before the census prints. Evidence:
  `246_probes/l4_term_census.sh`, `output_l4_term_census.txt`.

- **The corrected anchor accounting for `244` is 36 source anchors and 55 in `244`, with 8 lost,
  27 new and 28 carried, against v1's 23, 30, 3 and 10.** Panel claim at tree `a12d4d5d`,
  `threads = 1`. Five controls including a both-sides plant and a phantom control. Evidence:
  `246_probes/anchor_diff_v2.sh`, `output_anchor_diff_v2.txt`.

- **No assertion in `Slots::ADMITTED` can change truth value between two targets with `MIN`,
  `MAX` and `WIDTH` held fixed.** `crate = arvo-format`, `toolchain = nightly-2026-05-28`,
  `edition = 2024`, `threads = 1`, tree `a12d4d5d`. Established by construction over the eleven
  non-test source files: zero target-dependent constructs, and every coordinate carrier is a
  fixed-width type. Control: the same pattern finds one target-dependent construct in the
  crate's test files. Evidence: `246_probes/hosting_is_not_target_indexed.sh`,
  `output_hosting_is_not_target_indexed.txt`.

- **`Slots::ADMITTED`'s five assertions are three well-formedness conditions and two capacity
  conditions.** `crate = arvo-format`, `toolchain = nightly-2026-05-28`, `edition = 2024`,
  `threads = 1`, tree `a12d4d5d`. Bounded whole-container range over the five assertions,
  exhaustively, extracted from source rather than typed. Four controls, including the
  not-flagged control. Evidence: same probe.

- **`08_knuth_what_the_one_format_concept_covers.md` is a Knuth seat and not a Kiselyov seat.**
  Panel claim at tree `a12d4d5d`, `threads = 1`. Established from the filename convention under
  two controls and corroborated by `HANDLES.md`'s member-handle table. Evidence:
  `246_probes/the_persona_attribution.sh`, `output_persona_attribution.txt`.

- **Five of the six admission questions carry a standing answer row and one carries none, with
  `what_the_admission_contract_asks_a_candidate_to_expose` carrying two.** Registry claim at tree
  `a12d4d5d`, `threads = 1`. Bounded whole-set range over the six, exhaustively, by awk over raw
  records. Second instance for `244`'s L1 table by a disjoint instrument.

- **`question::are_the_level_hierarchies_the_same_cut` is about `65`'s three levels against
  `66`'s five, and not about R3's ten identity coordinates against `67`'s five chain
  components.** Registry and panel claim at tree `a12d4d5d`, `threads = 1`. Established by
  printing all four lists from their own sources and intersecting them: A and C share nothing, B
  and C share exactly `container`. Controls: a list against itself comes back whole, and the one
  shared word is reported. A further assertion requires the trait extraction to come to exactly
  ten or the probe refuses. Evidence: `246_probes/four_lists_not_two.sh`,
  `output_four_lists_not_two.txt`.

- **The suite is 137 passing and 2 ignored across the workspace.** `cargo test --manifest-path
  mock/Cargo.toml`, `toolchain = nightly-2026-05-28`, `edition = 2024`, `debug-assertions = on`,
  tree `a12d4d5d`, `threads = 1`.

Nothing above was measured at more than one thread, so under the standing reading none of it
holds where threads exist. Correct for all of them: they are compile-time, registry, repository
and panel facts.

---

## 8. What I carry forward unchanged, and from whom

Nine items, and the count is nine.

1. **From `244`, section 1's located disagreement and its three separators against the failure
   arm.** I did not re-derive them and I did not attack them. `245` re-derived all three from
   source and they held.
2. **From `244`, L1's table.** I re-derived it by a disjoint instrument and it reproduces, so I
   carry it as a two-instance finding rather than unchanged.
3. **From `244`, L5's resolution of `242`'s provenance hole**, that `ratified_by` records how a
   panel-derived proposition was ratified and an op-quote row carries the quote instead. Not
   re-run; `245` spot-checked two of eighteen and I checked none.
4. **From `244`, section 1.5's cross-file connection** between `241` section 4's "vacuously
   broad" caveat and its section 7's absence of it. `245` verified both halves against the files
   and the probe row.
5. **From `244`, C3's demotion of `241`'s "the arity is fixed at one by a ratified count".** I
   read R3's own scope statement and agree, and I am persona-blocked from being the second
   reader on a demotion of a Kiselyov claim only in the direction of agreeing with `241`, which
   is not this direction. It still wants a second independent reading and it does not have one.
6. **From `245`, the identification of the second promotion candidate.** Its finding, and
   sections 2.1 and 2.3 above are the establishment it asked a later seat to do.
7. **From `245`, the no-overlap result on Q21**, re-run under my own instrument with a positive
   control, so it is two instances rather than carried.
8. **From `241`, the `coordinate`-spans-two-tiers finding**, which is `244`'s A9 at two
   independent instances with `242`'s tabulation. Section 2.2 is that finding applied to the two
   rows, which is a use rather than a second instance.
9. **From `242` via `244`'s A10, and `240` independently, that `dimension.toml` has no axis able
   to carry a claim about the canon.** My section 7 inherits the gap and states its tree the
   same way.

---

## 9. Options opened, and what would close each

Stated as regions and arms rather than as a fork over a category, because a fork over a category
is the shape op has refused three times and the registry records the refusal at
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`.

**O1. The two rows carry clauses at different standings, and a row's `standing` is one field.**
Three arms, each holding in a region: split each row into the clauses its instances actually
cover, so each clause carries its own standing; or leave the rows and record the standing per
clause in a note, which costs nothing and is not machine-readable; or rewrite each row's
sentence down to the clauses two instances support, which loses the one-expert content. **What
would close it: whether any consumer of the registry gates on `standing` mechanically.** Nobody
has looked, and the answer decides whether a note is sufficient. Not mine to perform either way,
since it is a registry edit and op's `87` puts those at the end.

**O2. The vocabulary a canon sentence about admission speaks.** Not open as any row, which
section 5.4 establishes: `question::are_the_level_hierarchies_the_same_cut` asks about a
different pair of lists and answering it closes nothing here. Section 2.2 says what turns on
the question anyway, and it is not a naming preference: it decides whether `73`'s three-way
sort is expressible at all, and section 2.2 works one pair of cases, the Gray code and the
stride, and finds the identity cut cannot separate them. **What would close it, and it is
smaller than it looks:** the substance is already settled by R1, R2 and
`question::adaptation_in_identity_or_realisation`'s `answered`, so what remains is choosing
between `241`'s own two candidate spellings, calling the ten identity coordinates and the five
chain components, or keeping one word and stating the projection. `241` states both and
declines to pick, and I decline too, for the reason it gives: what would distinguish them is
whether any canon sentence needs to quantify over both tiers at once, and `73`'s location
procedure does, since its own disputed cases split across the boundary. **That is an argument
for the first spelling and it is one seat's, twice over, since `241` is my persona.** It wants
a reader who is not.

**O3. Whether hosting should be target-indexed.** Section 5.1 measures that it is not today and
that `241`'s argument for saying it is does not run. Neither `241`'s position nor Q31's option 3
survives that intact. **What would close it: whether the stack ever intends a target-dependent
numeric surface**, which is a design question about a crate that currently has no `usize`, no
`cfg(target`, and no pointer-sized anything in its non-test source.

**O4. Where well-formedness belongs.** Section 5.2 measures three kinds of condition enforced by
one const. **What would close it: whether a candidate that fixes ten coherent coordinates and a
candidate that fixes ten incoherent ones should receive different answers**, which `242`'s
fourth Q29 option is already circling from the other side with its three derived obligations,
and which `244`'s L4 says the archived design solved once as type-level bounds.

---

## 10. Coverage, and what to distrust

**Read in full:** `241`, `242`, `243`, `244`, `245`. `73` sections 1, 7 and 12, and `74`
section 3.1 and the N9 through N15 block, which are the passages `244` and `245` both named as
the weakness in their own L2 readings. `08` section 4.5. `85` section 3. `OPTIONS.md`'s
statement of Q19 through Q21.

**Read from the registry, raw:** `ruling.toml` for R1, R2, R3, R4 and
`the_operating_constraints_are_intents_and_rules`; `proposal.toml` for the seven admission-topic
answer rows and the identity proposal; `question.toml` for the six admission questions plus
`is_the_ambient_operation_family_fixed` and `are_the_level_hierarchies_the_same_cut`;
`retirement.toml` for `r161_r9_the_number_system_convergence_as_two_instances`, which is about a
different topic and stands as precedent rather than as a hit.

**Read from source:** `arvo-format/src/slots.rs` lines 55 to 175 in full, `width.rs`'s `Width`
declaration, `format.rs`'s `PHASE_NUM` and the phase arithmetic, and a target-dependence sweep
over all eleven non-test files.

**Deliberately not read, and each could have changed something.** The other roughly 440 panel
entries. `65`, `66` and `67`, which are lists A, B and C's own sources, so section 5.4's A and
B rest on `OPTIONS.md`'s statement of them and on the registry row's `options`, and its C rests
on `74` section 3.1's account of `67`. All three are quotations of the lists rather than
readings of the files, which is enough to establish that four lists are four and is not enough
to say anything about what any of them means. `DROPLIST.md`, `RULES.md`, `PERSONA_CALLS.md`
beyond one grep, `PRIOR_CALLS.md`, the `seed` and `catalogue` directories. `68` section 5, which is the origin of the hosting half's
three residue clauses, so my reading of what the proposal's hosting clause *means* rests on
`73`'s and `74`'s account of `68` rather than on `68`. **That is the one weakness in section
2.3's third row and I name it rather than letting the citation stand for a reading I did not
do.** `225` and `226`, which are R2's two instances, so my reading of R2 rests on the row.
`240`, beyond what `244` quotes.

**Not attempted.** Any consumer survey, which O1 and C4 both need. Any re-run of `241`'s or
`242`'s compiled probes. Any web search. Any edit to a registry row, which op's `87` reserves.

**What a reader should distrust most in this file.** Section 2.2, which is the load-bearing new
argument and which is a reading of two ratified rows plus a reading of `73` section 7. It has no
second reader. It is also the one place where being a Kiselyov seat cuts in my favour rather than
against me, since it uses `241`'s own reconciliation finding to *narrow* what `241` can second,
and a reader should still ask whether a Kiselyov seat is the right one to be drawing that
boundary. Section 5.2's classifier, which is fragile on inputs it did not see, though the census
it produced is exhaustive over five items I also read by hand. Section 5.4's negative half, that
no row asks the drafting question: it is an absence claim, it carries the search that
established it and a control that the search can find a phrase, and an absence claim is still
the shape that inverts silently the moment somebody writes the row. And section 6's
qualification on Q21, which is a judgement I have marked twice and which nothing measures.

**What I could not do.** I could not establish that `241` read nothing under `mock/research/`
before its blind commits, because reading leaves no trace in a repository and no instrument I
could build reaches it. What I have instead is the weaker fact that it edited nothing
afterwards, and I have said so every time it comes up rather than letting the stronger claim
stand on the weaker evidence. Somebody with access to the dispatching session's transcripts
could settle it; I cannot from here.
