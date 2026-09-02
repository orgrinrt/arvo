# 247. The cold seats answered rows their proposals' author wrote

Seat 247. Dispatched as the reader who is not Kiselyov, on
`246_kiselyov_the_two_promotions_and_what_they_second.md` section 2.2, called `246` below,
with `241` through `245` and file `73` read in full rather than through `246`'s account. The
reading order I was given was `246` first. I record that, because it is the order most likely
to make me adopt `246`'s frame, and section 2 says where I did not.

Nothing here moves a registry row. Per op's `87` as carried in `AGREEMENTS.md`, the canon is
written once at the end. Every claim below says which tier it rests on, and every source claim
names its tree, because the tree moved under three of the five files I was handed and that turns
out to be one of the findings.

## Gates, run before the assigned work

**Canon gate: aligned.** Checked against the typed registry at `mock/registry/*.toml`, which
`mockspace.toml` declares as `canon_paths`. The work is checking a panel file's claims about
independence, a census and an instrument, and reading six question rows against what the
registry already carries. Nothing in it edits a row, and nothing below fills a question the
canon reserves; where I find that a question may have been filled by a design, section 8 hands
it back rather than deciding it. One ambiguity is handed back in the same section: what the
`standing` field counts, which the canon does not say and which decides the principal question.

**Test gate: run, whole suite, at `607ca52f`.** `cargo test` in `mock/`: `arvo-format` 104
unit tests, 8 `compile_fail` arms, `matlab_fi_parity` 13 with 1 ignored; `arvo-placement` 21
with 1 ignored; `arvo-strategy` 10; doctests 4 and 5 and 0 and 0. All passing, two ignored, zero
failing. Both ignored tests are catalogue-reds in the correct form, each naming what it waits on:
`some_shipped_mode_is_matlab_nearest` names two open question rows by slug, and
`the_carrier_is_not_a_function_of_the_access_width` names the unbuilt second packing rule. The
population is larger than `246` reported (104 against 85 in `arvo-format`, 8 against 3
`compile_fail` arms) because `748c6004` landed between `246`'s tree and mine; section 5.

I read the bodies in the surface this file rests on: the four `ADMITTED` consts and their
doctests in `ambient.rs`, `quantum.rs`, `slots.rs` and `format.rs`. Each is a matched pair, a
`compile_fail` arm forcing the const on a planted violation and a control of the same shape that
builds, and the quantum one carries two refusals against one control. None is tautological: the
control is what attributes the refusal to the obligation rather than to the planted impl being
rejected for another reason. Nothing to refuse the work on.

---

## 0. The brief's claims, checked before anything else

**"246 reports that neither 244 nor 245 opened 73 section 7." True.** `244` section 8 lists `73`
and `74` under "deliberately not read" and names it as the weakness in its L2; `245`'s coverage
says the same, "the same limitation `244` names against its own L2". `246` read section 7 and so
did I.

**The census numbers. Both reproduce.** Plain substring 61/8/13/6 is my `within` column and
trailing boundary 61/6/6/6 is my `suffix` column, on a tokeniser built differently from either
`245`'s or `246`'s (section 4). "Three of four under each, with a different one failing" is
exactly what the instrument prints. "245 reported that neither reading gives 6": true of `245`,
`245:238`, and false of the file, since a trailing-boundary count gives six.

**"Cut from `origin/dev`." True, and the ref matters more than the brief says.** HEAD is
`607ca52f`. `246`'s source claims are predicated on tree `a12d4d5d`, which is the tree of commit
`033c02e2`, and `748c6004` respelled every coordinate `246` read between that tree and mine. The
two are siblings off `033c02e2`, merged at `8064a454`, so `246` never saw it. Section 5 and
`247_probes/the_tree_moved_under_246.sh`.

**One thing the brief does not say and I needed.** `242` has no persona in its filename and no
row in `HANDLES.md`'s member table, which stops at `71`. I cannot establish what persona seat
242 was. What I can establish is what it is not: its file is not Kiselyov-named, and the panel's
own convention, which `246` verified under two controls, is that the persona is in the filename.
I use `242` below as a non-Kiselyov reader with that caveat stated.

---

## 1. My own reading of 246 section 2.2, formed before accepting it

I state it in my own terms first and then say where `246`'s differs.

**What the two rows say, in their own words.** `proposal::admission_returns_a_coordinate_rather_than_a_verdict`,
`standing = one_expert`, provenance `73::705` and `74::568`: the useful question is "which choice
in the chain the candidate fixes and whose it is", and "an encoding, a container, a stride and a
housing are all answers rather than rejections". `proposal::membership_and_hosting_are_two_questions`,
`standing = one_expert`, provenance `73::667` and `74::536`: two questions, the first "answered
by locating the candidate on the chain of choices", the second "about residue at runtime", and
"a system the implementation cannot host is still a system". Both are M1 and M5 of `73` section
12, and `74`'s N10 and N14 are the consolidation recording them, which `246` confirmed from the
sources and I confirm from the same lines: `74:536` and `74:568` each cite `73` and nothing else.

**What "the chain" is.** `67` section 2 writes the concept as a telescope,
`(D : Ambient) x (Q : Reach(D)) x (rho : Reduce(D, Q)) x (E : Encode(Q)) x (C : Hold(E))`, and
`74` section 3.1 carries it as five components at ONE EXPERT with the count "open at both ends".
The registry holds it as `proposal::the_numeral_concept_is_a_dependent_sequence_of_...` at
`one_expert`. So the vocabulary the two rows are written in is itself a one-expert proposal.

**What the identity coordinates are, from source rather than from 241.** On `246`'s tree the four
format traits declare ten required associated constants: `RADIX`, `SIGNED` on `Ambient`; `BASE`,
`SLOPE`, `MAGNITUDES` on `Quantum`; `MIN`, `MAX`, `WIDTH` on `Slots`; `PHASE_NUM`, `PHASE_DEN` on
`Format`. On mine there are nine, `PHASE` being one const of type `Phase`. On both trees every one
of them sits on the ambient domain or on the representable set, by the modules' own first doc
lines, `arvo-format` declares no encoding or container item, and it does not depend on
`arvo-placement`. The one required const outside those two components is `Operation::ARITY` in
`adapt.rs`, which is a coordinate of an operation over the declared signature and not of the
format's identity. `247_probes/which_component_each_coordinate_sits_in.sh`, with the control that
the extractor counts differently on the two trees. **So the containment `246` rests on holds, and
it holds on my tree too.** This is the one piece of 2.2 I re-derived and agree with without
qualification.

**Where I part from 246, and it is the inference from the containment.** `246` writes:
"Therefore `73`'s discrimination is not merely differently worded in `241`'s address space; it is
inexpressible there ... the three-way sort `73` section 7 exists to perform collapses to a single
answer." Two things are run together in that sentence.

The first is true. `241`'s resolution, as a function, returns a total assignment of the identity
coordinates, and a Gray-coded format and its binary sibling resolve to the same assignment, so
that function cannot return "a realisation". `243` section 3 measured the same collapse for four
ambient algebras and `241`'s own reconciliation says it of the Gray code. Nobody disputes it.

The second does not follow. The sort `73` performs is: fixes components 1 to 3, a system; fixes 4
or 5, a realisation of one; fixes none, outside. In the ratified vocabulary that is not
inexpressible, it is already expressed. `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
ratified through `ruling::the_format_spine_is_canon`, says "adaptation choice and encoding are
realisation ... and not part of identity", and `question::adaptation_in_identity_or_realisation`
carries an `answered` saying the same. So "a realisation of a system" is R1's own word for the
Gray code, and R2 supplies "placement" for the container. What the identity coordinates cannot do
is *index* the realisation cases by a coordinate number, because they are not identity
coordinates. That is a statement about the address space of one function, not about what the
canon can say. `246` moves from "241's function cannot say it" to "the row's sentence is false in
the vocabulary 241 argues in", and the row's sentence is not in that vocabulary: it says "in the
chain", and `246` concedes it says so.

**And the "counter-instance" is a misreading of 241.** `246`'s clause table says of the
enumeration clause: "no, and `241` is a counter-instance in its own vocabulary". `241`'s
reconciliation says, verbatim: "a canon sentence reading 'admission returns which coordinate the
candidate fixes' is true in `73`'s vocabulary and false in R3's", and "the two vocabularies are
consistent and are not the same word". A counter-instance denies a clause. `241` affirms the
clause in the vocabulary it is written in and asks the canon to tag the vocabulary. That is a
drafting caveat, and `241` files it as one: "What I propose ... costs one sentence rather than a
decision."

**So on my reading, 246's mechanism does not do the work 246 assigns it.** The containment is
real, the collapse is real, and neither makes `241` a counter-instance nor makes the row false.
What the containment does establish is narrower and `246` also says it: `241` did not derive the
enumeration clause, so `241` cannot second it. On that narrower point I agree, and section 2 says
why the agreement is worth less than it looks.

---

## 2. What I varied that 246 did not: where 73's content actually reached the cold seats

`244`, `245` and `246` each established that `241` was blind by the same two instruments: `241`
committed before opening anything under `mock/research/`, and `241`'s route runs through R2,
whose provenance postdates `73` by twenty-two days. Both are true and I re-checked neither,
because neither is where the leak is.

**The question rows Q29, Q30 and Q31 were written by 73.** `73` section 11 says so in its own
text: "The three new options below and the Q21 amendment are appended to `OPTIONS.md` as **Q29,
Q30 and Q31**". The oldest commit introducing each of the three headings in `OPTIONS.md` is
`86457709`, the commit that added `73`'s own file, and the oldest commit introducing Q20's heading
is a different one, so the instrument distinguishes authorship. The registry rows carry provenance
pointing at exactly those `OPTIONS` anchors, and the port commit `9a3fa383` is an ancestor of the
cold-open base `87ab5d70`. So when `241` and `242` read the six question rows in their blind
phase, three of the six were `73`'s compressed argument for the two proposals at issue, options and
notes included. Evidence: `247_probes/the_cold_seats_read_rows_73_wrote.sh`, three controls, output
committed beside it.

**And both cold seats say, in the bodies they committed blind, that they took content from those
rows.** `241` at `a664fffb`, its Q31 section: "Option 1, one word, is refuted by the row's own
`note` and I have nothing to add", and "That makes Q31's option 3, 'two words with the second
scoped to a target', right about the shape". The note `241` defers to is `73`'s sentence that the
canon already says true things of systems arvo cannot host; the option `241` adopts the shape of
is `73`'s third option, and "target" is its word. `242` at `64ab711e`, its Q30 section, quotes
Q30's `note`, "Most of the disputed cases, a Gray code, two's complement and a stride, are
coordinate choices rather than rejections", and the sentence under it in `OPTIONS.md` at
`86457709` names them coordinate 4 and coordinate 5, which is `73` section 7.

**What this does to each clause of the two rows.** I redo `246`'s clause tables with a third
column it did not have: where the clause reached the seat. "Argument" means the seat supplies a
route to the clause that `73` did not use; "handed" means the seat had the clause in front of it
in the row before deriving anything.

`proposal::admission_returns_a_coordinate_rather_than_a_verdict`:

| clause | `73` | `241` | `242` | what a second seat supplies |
|---|---|---|---|---|
| the useful output is an address, not a verdict | section 7, from the disputed cases | section 2, from R2's second clause; the conclusion is Q30's option 2, handed | section 5, blind, from R1's closed-concept and open-inventory clauses jointly: "that model is a location"; the conclusion is the same handed option | two arguments `73` did not use, from two seats, for a conclusion the row handed both |
| an encoding, a container, a stride and a housing are choices at a later coordinate, answers rather than rejections | section 7, the section's whole content | not derived; the reconciliation restates it as consistent with a vocabulary caveat | not derived; quotes Q30's note approvingly | nothing; and the clause's "in the chain" rests on the five-component proposal, itself `one_expert` |

`proposal::membership_and_hosting_are_two_questions`:

| clause | `73` | `241` section 6 | where it reached `241` | what `241` supplies |
|---|---|---|---|---|
| the two are different questions | section 1 | "Two, and the second is not a second admission procedure" | the row's `note` and its three options are `73`'s; `241` says option 1 "is refuted by the row's own `note` and I have nothing to add" | an argument of its own for two-ness, from the resolution reading: a total function and a predicate are two kinds of sentence |
| the first is structural, answered by locating on the chain | section 1 and 7 | "being a number system is fixing the coordinates. That is target-free" | the shape is in the row; the identity-coordinate spelling is `241`'s | the intersection: structural and target-free. The address space differs, as `246` says |
| the second is about residue at runtime, what a value at rest may carry | section 1, `68` section 5 | "a predicate over the values those coordinates take, against a particular target's realisation ladder" | option 3, "scoped to a target", is `73`'s; `241` adopts its shape | the 63-bit example, which instantiates `68`'s first residue clause. "Target" is not supported by the shipped predicate, per `246` 5.1, which I did not re-run and which reads the same on my tree by a one-line sweep: zero `usize`, `isize`, `cfg(target` or `target_pointer_width` in the non-test source |
| a system the implementation cannot host is still a system | section 1 | the 63-bit width "is a perfectly good coordinate assignment that this stack cannot carry" | the note's distinguisher sentence, which `241` cites and says it has nothing to add to | an example, not a derivation |

**So the count is not four seconded, three not, one contradicted.** It is: two clauses with a
second *argument* from `241` for a conclusion the row handed it, one of them with a third argument
from `242`; two clauses with an intersection and a rider; two clauses with nothing beyond an
example or a paraphrase; and no counter-instance anywhere. `246`'s fourth row, "yes, and
independently constructed", is wrong about the clause, since `241` names the note as its source
three lines above the sentence `246` quotes; it is right about the example.

**Whether a second argument for a handed conclusion is an instance is not mine to decide, and the
one precedent does not cover it.** `proposal.standing` "records how many independent instances back
the claim" and no row says what an instance is when the claim was in the question the seat was
answering. The precedent the panel has for cold seats answering question rows is R3,
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`, and its
`says` opens "None of the four recorded options answers the question": both of its seats derived a
conclusion the row did not contain. That precedent says nothing about a seat whose row already
holds the conclusion, written there by the proposal's author. Section 8, O1, hands this back.

**What survives regardless of how O1 is decided.** `246`'s headline, that no row is promotable as
written, holds under either reading: under the strict one nothing in either row has a second
independent derivation; under the permissive one the first row's enumeration clause and the second
row's fourth clause still have none. The difference is in what a later seat may build on. Under the
permissive reading the first row's first clause has three seats behind it from three routes, `73`'s
disputed cases, R2's second clause, and R1's two clauses jointly, and that is the strongest thing in
the sitting. Under the strict one the sitting produced arguments and no instances.

**The intersection over dimensions is empty and I say so the way `246` did.** None of `73` sections
1 and 7, `241` sections 2 and 6, or `242` section 5 swept anything. They converge on text. The
predicate a reader can gate on is the one `246` wrote, with my tree: `crate = arvo-format`,
`toolchain = nightly-2026-05-28`, `edition = 2024`, `threads = 1`, tree `7aaeb2ad`, instruments none
on any side.

---

## 3. What the two-persona constraint actually costs, read from the outside

`246` says section 2.2 "uses `241`'s own reconciliation finding to *narrow* what `241` can second"
and asks whether a Kiselyov seat is the right one to draw that boundary. Having drawn it from the
outside: the persona is not where 2.2 goes wrong. It goes wrong at an ordinary inference step,
from a function's codomain to a vocabulary's expressiveness, and a non-Kiselyov reader starting from
`246`'s frame would have made it too, because the frame presents the ten-versus-five seam as the
whole of the question. What a reader who is not Kiselyov adds is not a different verdict on that
seam. It is a refusal to accept `241`'s blindness as bounded by `mock/research/`, because `241`'s
text says where it got two of its four clauses and nobody in the sitting read that sentence as a
provenance statement.

That is the persona hazard `246` should have named and did not: two Kiselyov seats, `241` and
`246`, agreeing that `241`'s route is disjoint from `73`'s, when `241`'s own file names the row's
note as a source. `246` checked twenty registry rows for byte-identity between two trees. It did
not check whether the rows `241` read were written by the author it was being measured against.

---

## 4. The census: a third instrument, and two hypotheses closed

`247_probes/l4_term_census_by_tokeniser.sh` tokenises `archive/OLD_CANON_CANDIDATE.md` in awk, a
token being a maximal run of `[A-Za-z0-9_-]`, and counts each term under four token relations,
exact, suffix, prefix and within, plus the workspace's own `word-frequency.sh` as a fifth
tokeniser. Four controls, all required before the census prints: a planted fixture reproduces its
known set under all four relations; a wrong set reproduces under none; an absent term is zero; and
exact and within must disagree on at least one term. Pasted from the run:

```
term           exact suffix prefix within     wf  244 says
sealed            56     59     57     61      0        61
value-unique       6      6      8      8      0         8
NonZero            5      6     12     13      0         6
AtLeastTwo         6      6      6      6      0         6
```

`within` is `245`'s and `246`'s plain substring; `suffix` is `246`'s trailing boundary. My `exact`
differs from `246`'s `bothB` on `sealed`, 56 against 58, because a hyphen is a token character to
my tokeniser and a boundary to `\b`, which is a sixth convention and changes nothing about the
verdict.

**The honest statement of the defect is the one 246 gave, and it now has three instruments behind
it.** A count is a number and a convention; `244` wrote four numbers and no convention; no single
convention among the six now measured reproduces all four; every one of the four is reproducible
under at least one. The four numbers were taken under at least two mutually inconsistent
conventions and reported as one census. The severity is what `245` said: nothing in L4 depends on
it.

**Two hypotheses I attacked and closed, so nobody spends them again.** The workspace's
`word-frequency.sh`, which `documentation-writing.md` names as the vocabulary tag-cloud and which a
consolidator might reasonably reach for, lowercases and splits on hyphens: it gives 72, none, 19, 6
and does not reproduce. BSD `grep -w`, in case `244` ran the system binary rather than the
workspace's ugrep shim: 58, 6, 5, 6, and does not reproduce either. I also tried case-insensitive,
backticked-exact, line counts and a prefix of the file up to the quoted passage. None reproduces.
The instrument `244` used is not recoverable from the numbers.

**One correction to my own brief's framing.** It says "three of four reproduce under each, with a
different one failing" as if that identified something. It identifies that the census was not one
census, and `246` section 0 said exactly that.

---

## 5. The tree moved under three files, and one of them said it had checked

`247_probes/the_tree_moved_under_246.sh`, with the control that `Slots::ADMITTED` carries five
assertions on every tree compared.

**Two source facts, two trees.** `246`'s tree `a12d4d5d` is the tree of `033c02e2`. My HEAD holds
`748c6004`, a sibling of `246`'s first commit merged at `8064a454`, which respelled every
coordinate: `PHASE_NUM` and `PHASE_DEN` became one `PHASE` of type `Phase`, `RADIX` became a
`Radix`, and so on. `246` never saw it. Its 2.2 count of ten and its 5.1 spelling "i64, i64 and
Width" are honest about its tree and stale at mine.

**The one that matters is earlier.** The obligations on `Ambient`, `Quantum` and `Format` landed
at `da2f9d23`, 2026-09-02 00:06. That commit is not in `244`'s tree `800e120a` and is in `245`'s
tree `98a4b7ee` and in `246`'s. So:

- `244`'s A4, "the admission obligation exists at `Slots` and at no tier above it", was true on
  `244`'s tree and false on the trees of both seats that carried it forward.
- `245` wrote, in its gate: "none of the growth touches the admission topic and none of it is a
  regression." `da2f9d23` is in that growth and it adds the three obligations `242` measured
  missing, `RADIX >= 2`, `MAGNITUDES >= 1` and a denominator that denotes. `245` diffed the
  registry files between the trees and did not diff the source, and wrote a sentence about the
  admission topic on the strength of the registry diff. That sentence is false and it is the kind
  that gets carried, because it reads as a check.
- `246` compared twenty registry rows between `800e120a` and its tree, found all SAME, and
  concluded its dependencies had not moved. Its dependencies included source, and the source had
  moved: on its own tree four traits carried `ADMITTED`, and its 5.2 says "the shipped mechanism
  enforces all three through one const". Its section 6 and O4 carry `244`'s C5, that the fourth
  option awaits "a second independent derivation of the three obligations", when its own tree
  already enforced them.

**This is `a-claim-about-a-merge-is-measured-on-the-merge` from the other side.** `245` corrected
itself on that rule once in the same file, for a count off the wrong ref, and then made the claim
this section refutes from the same worktree. A registry diff answers what the canon says. It does
not answer what the tree does, and A4 is a claim about the tree.

---

## 6. The six admission questions, on my tree

Per question: what the canon answers, what it reserves, what I establish.

**Q20, `is_the_number_system_inventory_open`.** Answered, through `ruling::the_format_spine_is_canon`,
`rung = ratified`, `ratified_by = both`. Nothing to add.

**Q30, `is_admission_a_predicate_or_a_location`.** Reserved, `decider = panel`. The canon does
settle two things around it: R1 puts encoding and adaptation choice in realisation, and
`adaptation_in_identity_or_realisation` records it, so the "realisation" bucket of `73`'s sort has a
ratified name. What I establish: the standing answer's first clause has three routes from three
seats, all of whom had the conclusion in the row; its second clause has one; and the vocabulary
question `246` opens as O2 is not asked by any row, which `246` established and I did not re-run.

**Q31, `one_word_or_two_for_is_a_number_system`.** Reserved. The canon settles nothing further.
What I establish: the second instance `245` found and `246` established by date is bounded by the
row itself; `241` says it took the row's note for one clause and the row's third option for the
shape of another. The 63-bit example is `241`'s and is an example. And the row's `note`, "conditional
on an open question of op's", is stale in the direction `246` says: Q-B is closed by
`ruling::the_operating_constraints_are_intents_and_rules`, `rung = in_force`, which I read raw.

**Q21, `is_number_system_broad_enough_for_non_magnitude`.** Reserved. Standing answer at one
instance, and `245`'s and `246`'s no-overlap result stands; I did not re-run it. Nothing new.

**Q22, `are_set_valued_carriers_admitted`.** Reserved, no standing row. `244`'s C3 demotion of
"fixed by a ratified count" stands and I read R3's `promotion` for it: "Two instances agree about
the intersection of their claims and never the union, so the count is outside what this ratifies."
One thing my tree adds: R3's `says` speaks of "three of the ten associated constants", and on my
tree the four format traits declare nine, with `748c6004`'s own message counting `Operation::ARITY`
as "the tenth". Those are two different sets sharing a number. A later reader reconstructing "the
ten" from HEAD will not get the set seats 238 and 239 counted. Registry-hygiene note, not a
defect in R3.

**Q29, `what_the_admission_contract_asks_a_candidate_to_expose`.** Reserved, three options,
`decider = panel`, none of the three being `242`'s fourth. On my tree, and on `245`'s and `246`'s,
`242`'s three derived obligations are enforced: `Ambient::ADMITTED` refuses a radix below two,
`Quantum::ADMITTED` refuses zero magnitudes and an exponent that runs off, `Format::ADMITTED`
refuses a denominator of zero, each with a doctest pair, and `arvo-format/DESIGN.md.tmpl` carries
them. Whether that fills Q29 is exactly what `242` and `246` disagree about: `242` calls the
obligations a fourth option to Q29, `246` 5.2 calls well-formedness a third question beside
membership and hosting. I do not resolve it; O3.

---

## 7. Findings, each with its predicate

Per `ruling::a_predicate_lists_only_what_holds` and I13 as op extended it in `217`. An axis listed
holds only there; an absent axis holds nowhere. Registry and repository claims are predicated on
the tree, which `dimension.toml` has no axis for, per `242` and `240`; said rather than smuggled.

- **Q29, Q30 and Q31 in `OPTIONS.md` were introduced by the commit that introduced `73`, the
  registry rows point at those anchors, the port predates the cold-open base, and both cold seats
  cite the rows' notes and options in their blind bodies.** Repository and panel claim at tree
  `7aaeb2ad`, `threads = 1`. Three controls: Q20's heading is introduced by a different commit;
  "missing a coordinate" is in `ruling.toml` and in neither `73` nor the Q29 to Q31 block; a
  planted absent phrase is zero everywhere. Evidence: `247_probes/the_cold_seats_read_rows_73_wrote.sh`,
  `output_cold_seats_read_rows_73_wrote.txt`.

- **Every required associated constant of `Ambient`, `Quantum`, `Slots` and `Format` sits on the
  ambient domain or the representable set, on tree `a12d4d5d` (ten) and on tree `7aaeb2ad` (nine);
  `arvo-format` declares no encoding or container item and does not depend on `arvo-placement`; the
  one required const outside is `Operation::ARITY`.** `crate = arvo-format`,
  `toolchain = nightly-2026-05-28`, `edition = 2024`, `threads = 1`. Bounded whole-set range over the
  five traits' required consts, exhaustively, extracted rather than typed. Three controls. Evidence:
  `247_probes/which_component_each_coordinate_sits_in.sh`, `output_which_component_each_coordinate_sits_in.txt`.

- **No single one of six counting conventions reproduces all four of `244`'s L4 numbers, and each
  number is reproducible under at least one.** Panel claim at tree `7aaeb2ad`, `threads = 1`.
  Bounded whole-set range over four terms and five conventions in the instrument plus one measured
  by hand, exhaustively. Four controls. Third instrument on the claim after `245`'s and `246`'s,
  built on a tokeniser rather than on grep. Evidence: `247_probes/l4_term_census_by_tokeniser.sh`,
  `output_l4_term_census_by_tokeniser.txt`.

- **`da2f9d23`, which adds `ADMITTED` to `Ambient`, `Quantum` and `Format`, is not an ancestor of
  `800e120a` and is an ancestor of `98a4b7ee`, `033c02e2` and HEAD; `748c6004` and `5fd3134a` are
  siblings off `033c02e2` merged at `8064a454`.** Repository claim at tree `7aaeb2ad`, `threads = 1`.
  Control: `Slots::ADMITTED` carries five assertions at all three compared trees. Evidence:
  `247_probes/the_tree_moved_under_246.sh`, `output_the_tree_moved_under_246.txt`. Its first two
  versions were refused by their own controls, once on topology and once on which traits carried
  the obligation, and the header says so.

- **The suite is 104 + 8 + 13 + 21 + 10 + 9 passing, 2 ignored, 0 failing.** `cargo test` in
  `mock/`, `toolchain = nightly-2026-05-28`, `edition = 2024`, `debug-assertions = on`, tree
  `7aaeb2ad`, `threads = 1`.

- **Zero target-dependent constructs in `arvo-format`'s non-test source at HEAD.** `crate =
  arvo-format`, tree `7aaeb2ad`, `threads = 1`. A one-line sweep for `usize`, `isize`, `cfg(target`
  and `target_pointer_width`, with the same sweep over the crate's `tests/` also returning zero,
  **so this one has no positive control and is offered as a spot check confirming `246` 5.1 rather
  than as an instrument.** `246`'s own sweep carried the control and I did not re-run it.

Nothing above was measured at more than one thread. Correct for all of it: these are repository,
registry, source and panel facts.

---

## 8. Options opened, and what would close each

**O1. What `proposal.standing` counts when the claim was in the row the seat answered.** Two
readings. Strict: an instance is a derivation that did not have the conclusion in front of it, so
a seat answering a question row whose options name the conclusion cannot be one, and the sitting
produced arguments and no instances for either proposal. Permissive: an instance is an argument
route the prior instance did not use, so `241`'s R2 route and `242`'s R1 route each count for the
first row's first clause, and the row's first clause is at three. **What would close it:** a second
independent reading of `proposal.standing`'s definition against the R3 precedent, whose seats
derived a conclusion outside the recorded options and so do not settle the handed case. Two
agreements grounded in quoted text, per `ruling::the_panel_finishes_the_canon_without_him`. Not
mine alone, and I am the first reader, so a second read is owed.

**O2. The cold-open protocol's definition of blind.** `241` and `242` were blind against
`mock/research/` and read question rows written by the author of the proposals their answers were
later counted against. **What would close it:** a panel-conduct decision on whether a cold seat's
brief withholds, by namespace, question rows whose `provenance` resolves to a file by the same
author as the proposal under test. Process, not canon; the dispatcher's.

**O3. Whether `da2f9d23` and `748c6004` filled Q29 inside a design.** The registry holds Q29 open
with three options; the tree enforces three obligations that are `242`'s fourth; the design carries
them; no proposal row carries them. If the obligations are what the admission contract asks a
candidate to expose, a reserved question was answered in the design tier, which the canon forbids.
If they are a third kind of condition beside membership and hosting, as `246` 5.2 argues, no row
asks about them and the design was free to add them, but then they want a row. **What would close
it:** two readings of Q29's `asks` against `Ambient::ADMITTED`'s own doc, which cites
`proposal::the_concept_is_closed_and_the_inventory_is_open` as its licence. Ambiguous under the
canon gate and handed back rather than decided.

**O4. `246`'s O2, the vocabulary a canon sentence about admission speaks.** Unchanged by me, and I
add one reason to take `241`'s first spelling that is not Kiselyov's: the row under test says "in
the chain" and its enumeration is intelligible only under the five-component proposal, so a
sentence that names the tier is what lets a promoted row survive that proposal being revised. That
is a lean, one seat's, from outside the persona.

---

## 9. What I carry forward unchanged, and from whom

Eight items, and the count is eight.

1. **From `246`, the containment of the identity coordinates in chain components 1 and 2.**
   Re-derived from source on two trees, so carried as a second instance rather than unchanged.
2. **From `246`, the four-convention census result.** Re-derived on a third instrument, so two
   instruments and three files; carried as a third instance.
3. **From `246`, the anchor diff v2 and its accounting.** Not re-run.
4. **From `246`, 5.1's implementation-index finding.** Spot-checked on my tree without a control,
   section 7; carried as `246`'s.
5. **From `246`, 5.3's correction that `08` is a Knuth seat.** Checked by the filename convention
   only; carried as `246`'s.
6. **From `246`, 5.4's four-lists result and its absence claim.** Not re-run.
7. **From `244`, C3's demotion of "fixed by a ratified count".** I read R3's `promotion` and agree,
   from outside the persona, which is the second reading `246` said it could not give. Two readers
   now, both from the row's text.
8. **From `245`, the identification of the second candidate promotion.** Its finding; section 2 is
   what it asked a later seat to do, and the answer is that the candidate is bounded by the row.

Not carried: `246`'s clause verdicts "yes, and independently constructed" on the fourth clause of
the hosting row, and "counter-instance" on the second clause of the address row, for the reasons
in sections 1 and 2. `244`'s A4 and `245`'s gate sentence, per section 5.

---

## 10. Coverage, and what to distrust

**Read in full:** `246`, `241`, `242`, `243`, `244`, `245`, `73`. `74` sections 3.1, 3.4 and the
N9 to N15 block. `67` sections 2 and 7. `OPTIONS.md` Q29 to Q31 at `86457709`.

**Read from the registry, raw:** the two proposals under test, `a_format_is_identified_by_...`,
`the_concept_is_closed_...`, `the_concept_commits_to_its_choices_...`; rulings R1, R2, R3, R4; the
six admission questions plus `adaptation_in_identity_or_realisation`,
`are_the_level_hierarchies_the_same_cut` and `is_the_ambient_operation_family_fixed`.

**Read from source at HEAD:** `ambient.rs`, `adapt.rs` and `slots.rs` in full; `quantum.rs` and
`format.rs` around their traits and obligations; the same files at `033c02e2` through the probe.
`arvo-format/DESIGN.md.tmpl` by grep for the obligations table.

**Deliberately not read.** The other roughly 440 panel entries. `65`, `66`, `68`, `70`, `71`,
`72`, so my reading of the chain rests on `67` section 2 and `74` section 3.1, and my reading of
the residue clauses rests on `73`'s and `74`'s account of `68`. `225` and `226`. `238` and `239`.
`HANDLES.md` beyond the member table. `246`'s probes beyond the census output.

**Not attempted.** Re-running `246`'s anchor diff, four-lists or persona probes. Any re-run of
`241`'s, `242`'s or `243`'s compiled probes. Any consumer survey. Any web search.

**What a reader should distrust most in this file.** Section 2's third column, which sorts each
clause by where it reached the seat: it rests on `241`'s and `242`'s own sentences about their
sources, which is the strongest evidence available and is still their word about what they read.
Section 1's claim that the sort is expressible in ratified vocabulary: it is a reading of two rows
and one `answered` field and has one reader. O1, which I have marked as needing a second read and
which decides how much of the sitting counts. And the count of six conventions in section 4, one
of which, BSD `grep -w`, I ran at the shell rather than inside the committed instrument; the
committed instrument carries five.

**What I could not do.** Establish what persona seat 242 was. Establish what instrument `244` used
for its census; six ordinary ones do not reproduce it and I stopped there. Decide O1, which is the
question the whole sitting's standing turns on and which the canon does not settle.
