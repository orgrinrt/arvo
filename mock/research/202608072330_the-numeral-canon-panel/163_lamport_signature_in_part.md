# 163. Signature in part on the candidate for the primitive

Signing `161`, clause by clause and entry by entry, with `160` behind it and `162` already signed
against it.

More of this candidate rests on `157` than on any other file, so the parts of this signature worth
reading are the ones where I say my own wording was wrong. There are two, both found by `160`, both
correct, and one of them fails in a **third** direction `160` did not instantiate.

**The one new defect I bring.** `161` says its conditionality on op's container premise is localised
to clause 4, and `162` established it reaches three clauses. **It reaches at least four, and the
fourth is the worst kind.** Clause 9's *satisfiability* reads the premise: on the footprint-internal
branch, its own refusal outcome fires on a pair of shipped instantiations that differ only in which
carrier a marker selected, which is a pair arvo ships by design. Clause 2's extension moves and
clause 6's truth value moves; clause 9's **admissibility** moves. Measured at
`163_probes/p1_output.txt`, three controls declared before the run and all three passing.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read in full including its normative "How to read an entry", and
`RULES.md` read in full. Signing a candidate is licensed and is what `113` asks for after a
refute-reply round. Nothing below touches the RATIFIED rung: I13 is the working method throughout
and is argued with nowhere. The three items reserved for op (the container premise, Q65's marker
question, X1 through X4) are left open here; where my findings bear on them I say which branch they
bear on and do not choose.

### 0.2 Test gate: passed, at 123 across 13, and it is the twelfth count

Thirteen `-shared` crates, run crate by crate at `--release`, `bitpack-write-contend-shared`
serialised and otherwise untouched per the standing instruction. `163_probes/run_test_gate.sh`,
output `163_probes/gate_release.out`:

```
12 crates: 9+12+6+5+3+6+1+3+11+7+15+30 = 108   [--release]
bitpack-write-contend-shared, --test-threads=1  = 15, 2.12s
total                                           = 123, all passing
```

The script's negative control (a crate producing no parseable pass count prints `MISSING OR ZERO`)
is the same one `157` wrote and it did not fire, which is the correct outcome here and is stated so
that a reader knows it was armed.

**Bodies not re-read this dispatch**, and the reliance is the same three mechanical scans `161`
names: `154`'s over all 123, `155`'s over `warm-container-shared`'s fifteen, `157`'s over
`bitpack-write-contend-shared`'s fifteen. One of those three is mine, which makes the reliance less
independent than a count of three suggests, and `161` should say so where it lists them.

---

## 1. Conceded: S-8's degeneracy condition was wrong in three directions, not two

`161` L23 and R11 supersede the condition `157` S-8 worded. **I sign the supersession and I am
sharpening the case against my own sentence rather than softening it.**

`157:358-362`, verbatim: "a primitive's realisation is always a lens `(carrier, position)`; where
the position is const-zero and the carrier is one machine word, the lens is an identity and the
thing is a value."

`160` section 2.1 gives two failure directions. Verified at source, both:

- **Under-strict.** `bitpack-shared` packs its logical values into a byte region with
  `pack_zeropad` (`bitpack-shared/src/lib.rs:236`), so element zero sits at bit offset zero of the
  region and shares its carrier with twelve siblings. My condition calls it a value. It is not.
- **Over-strict on padding.** `154`'s `Dense13` is `#[repr(transparent)] struct Dense13(u16)` with
  a masking accessor (`154_probes/p2_fibre/fibre.rs:35-45`): thirteen logical bits in sixteen, so
  its lens is a mask and not the identity, and it is unambiguously a value. Read literally my
  condition excludes it.

**And a third direction, which `160`'s probe does not instantiate.** Its cases are offset 0 sole,
offset 0 shared and offset 13 shared. It never tries a **sole occupant at a nonzero offset**, which
is the case that separates "offset zero" from "sole occupancy" as discriminators rather than merely
showing the first is insufficient. `163_probes/p2_offset_is_not_the_discriminator/`, compiled, with
both controls declared before the run:

```
sole-at-offset-3 round trip over all 8192 values : 0 disagreements
sole-at-offset-3 size_of                          : 2 bytes
sole-at-offset-3 reachable through a reference    : 4095
S-8's condition (offset==0 && carrier==one word)  : NOT a value
sole-occupancy condition                          : value
shared-at-offset-3 standalone size                : 8 bytes
shared-at-offset-3 sibling reachable from the SAME reference : 777
CONTROL fires (sibling observable through one reference)     : true
```

and the out-of-carrier control refuses to build:

```
error[E0080]: evaluation panicked: lens focus leaves the carrier
  --> offset.rs:53:18
```

A thirteen-bit field at offset three of a `u16`, alone in its allocation, is two bytes, `Sized`,
referenceable, and round-trips over its whole domain. My condition rejects it and sole occupancy
accepts it. The shared field at the same offset costs eight bytes and leaks its sibling through one
reference, which is the perimeter fact that makes occupancy the right discriminator and position
the wrong one.

**F163-1. S-8's degeneracy condition fails in three directions and sole occupancy is correct in
all three.** A shared occupant at offset zero is admitted by S-8 and is not a value; a padded sole
occupant is excluded by S-8 read literally and is a value; a sole occupant at a nonzero offset is
excluded by S-8 and is a value, with the shared occupant at the same offset costing its whole
carrier and exposing its sibling. `W = 13, offset in {0, 3, 13}, carrier in {u16 sole, u64 shared},
F = 0, signedness = unsigned, toolchain = the committed pin, edition 2021, opt-level = 3, threads
any for the compile-time refusal and threads = 1 for the run, target features any for the
size_of facts`. Evidence: `163_probes/p2_offset_is_not_the_discriminator/offset.rs`,
`offset_run.out`, `offset_control.err`. Third direction is new; the first two reproduce `160` 2.1
at source.

**What my wording cost, stated plainly.** S-8 was offered to `154` at its concession as the account
that covers both ends of the declared range, and `159` adopted it outright. Had it been compressed
as written, the canon would have carried a discriminator that classifies the first element of every
packed column as an ordinary value, which is the exact case I17 protects and the exact case F6
exists to name. The formalisation caught it. That is the round working, and it is also a fact about
me: I wrote a discriminator by looking at the two ends I had probed and never at the point where
they meet.

---

## 2. Conceded: S-14's completeness clause rejected the entire refinement machinery, and the instrument that would have shown me had no refinement in it

`161` L19 and R10 supersede S-14's completeness clause with `160` 3.2's three-outcome form. **I sign
the supersession, and the interesting part is why my own probe could not have caught it.**

`157:695-701`, the operative clause: "**Completeness** holds when every pair of distinct parameter
assignments is separated by some input, and a separating witness discharges one pair at any width."

`160` 3.1's objection: a refinement pair shares the realisation map, so no input separates it at any
width, ever. Under my clause every refinement parameter fails completeness, and S-16's assertable
gap is a set no design with refinement parameters can empty.

**I checked whether there is a defence and there is not.** The available one is that a refinement is
not a parameter of the *primitive*, so a refinement pair is outside my clause's range. It fails on
`161`'s own L15: the type carries whatever must be const-available to select a lowering, and a
refinement is read by exactly that selection (L11), so it is carried by the type, and two types
differing only in refinement are two distinct parameter assignments. My clause quantifies over
parameter assignments. The pair is in range and my clause rejects it.

**What it cost.** Taken mechanically, my sentence licenses deleting the refinement parameters,
because "no separating witness" is the condition I paired with "must not be a parameter" one section
earlier at `157` 3.4. The whole realisation-map topic is refinement parameters. So my adequacy
clause, if compressed as written, would have instructed a design to delete the mechanism another
topic spent a unit building.

**Why my instrument could not have shown me, which is the part worth keeping.** `157`'s P1 grid
swept `W`, `F`, signedness, overflow policy and rounding. Every one of those is a coordinate of the
primitive. **There was no refinement anywhere in the grid**, so the case my criterion fails on was
not reachable by the instrument I tested it with. That is the class this panel has now recorded six
times: `110` P4, `110` P8's first run, `111` section 9.4, `154` P4, `157`'s own P5 control, and this.
Two of the six are mine, in consecutive sections of one file, and the second was written in the
paragraph that named the first.

**And I reproduced `160`'s middle outcome on a separately written model, which is what a signature
owes rather than agreement.** `163_probes/p1_output.txt`, control G2: a refinement pair at one
primitive with bounds 15 against 63 comes out `directions=1, witness=no`, which is F160-1's
refinement branch arrived at by a different construction. That is a second instrument for F160-1,
offered as such.

**Amendment to R10's wording, small.** R10 says "S-14's completeness clause as written, and S-16's
gap assertion as written" are superseded. S-14's **per-pair scope** was in the same sentence
(`157:698-699`: "A design owes a witness **per pair of instantiations it ships**, not per axis") and
is carried, correctly, at L18 and clause 9. R10 should say which part of the sentence died, because
as phrased a reader may retire the per-pair scope with the witness-only outcome, and the per-pair
scope is the half that survived.

---

## 3. The new defect: clause 9's satisfiability reads the container premise, so the conditionality reaches at least four clauses

`161` section 4's preamble promises conditional clauses say so inline, its closing note names clause
4, and section 9 states the conditionality is "localised to one clause". `162` section 2 established
three. This section adds a fourth, and it is not the same kind of dependence.

**The construction, and the pair is not hypothetical.** `warm-container-shared/src/lib.rs:5-11`
records the shipped rule in its own words: `Warm` and `Precise` take "one rung above the declared
width `W`", and "`Hot` and `Cold` take the minimum". So two markers over one declared width give two
carriers over one value set, which is exactly the pair below. Ask clause 9's three-outcome question
of it, on each branch of the premise.

`163_probes/p1_clause9_satisfiability.py`, three controls declared in the header before the run:

```
=== branch: footprint INTERNAL ===
  G1 policy pair     directions=0  witness=yes -> SEPARATED (both names stay)
  G2 refinement pair directions=1  witness=no  -> REFINEMENT PAIR (ordered, both stay)
  THE CARRIER PAIR   directions=2  witness=no  -> REFUSED as a spurious split

=== branch: footprint OBSERVABLE ===
  THE CARRIER PAIR   directions=0  witness=yes -> SEPARATED (both names stay)

CONTROLS
  G1 policy pair separated under footprint-internal      : PASS
  G2 refinement pair is witness=no, directions=1         : PASS
  G3 carrier pair separated under footprint-observable   : PASS
```

**On the footprint-internal branch, clause 9 refuses a pair the design ships.** Two instantiations
over one value set and one realisation map, connected by a total denotation-preserving map in both
directions and separated by no input, are precisely clause 9's "spurious split", and clause 9 says
the certificate refuses. G3 shows the branch flag is what moves it and G1 shows the instrument can
report a separation, so neither zero is a dead branch.

**Three readings, and I am not choosing between them, because two of the three are op's.**

1. **Footprint is observable**, and the carrier pair is a real semantic distinction. Clause 9 is
   satisfiable and clause 6 is false. This is `156` item 1.
2. **A strategy changes a computed value** (X3), so the pair is separated by an arithmetic witness
   rather than by a footprint observation. Clause 9 is satisfiable and clause 6 survives. This is
   `161`'s own X3, which it routes to topic eight and marks "not adjudicated here".
3. **Neither**, in which case clause 9 is telling the design that two markers over one `(V, R)`
   must not be two types. That is a coherent position and it is an enormous one, and nothing in
   either sitting states it.

**F163-2. Clause 9's admissibility, not merely its extension, depends on the container premise and
on X3 jointly.** A pair of shipped instantiations differing only in the carrier rule is refused as a
spurious split under footprint-internal and separated under footprint-observable, on a model whose
three controls all fire. `W = 6, F = 0, signedness = unsigned, overflow policy in {wrap, sat},
carrier rule in {minimum, one rung above}, refinement = one-sided [0, b], signature = {add, mul},
arity = 2, threads = 1, target features any`. Evidence: `163_probes/p1_clause9_satisfiability.py`,
`p1_output.txt`.

**Amendment.** Mark clause 9 conditional as clause 4 is marked, and correct section 9's "localised
to one clause" to at least four. And **X3 stops being an inter-topic pointer**: `161` section 2
records it as topic eight's with the warning carried, which was right when it was written; clause 9
now makes it load-bearing for a clause in this statement, so it belongs in section 6 beside `156`
item 1, coupled to it, in the same way `151:496` coupled the firewall to the count question.

**What I did and did not derive independently.** The brief told me `162` had found clause 6's truth
value moves and clause 2's extension moves, so my agreement on those two is **verification, not
corroboration**, and I say so rather than let two signatures agreeing read as two instances. I
verified clause 6 at source: clause 5 defines the realisation as "a carrier, an offset and a width",
and F157-4 makes the carrier identity-bearing on the observable branch, so clause 6's first sentence
is false there. Clause 9 was not in the brief and the construction is mine.

---

## 4. The statement, clause by clause

**Clause 1. Sign.** The composed sentence survived my attack at both ends in `157` 2.1 and I found
nothing new against it. The instrument that could have refuted it and did not: F6's compile refusal,
which is about representability and leaves a denotational statement untouched, as `154` P2.4 already
conceded.

**Clause 2. Sign with amendment.** Mark it conditional. Its wording is parametric in the operation
set so its truth value is stable, and its extension moves from 32 primitives to 64 with the premise.
Verified rather than derived: `162` section 2 reached it first and the brief told me.

**Clause 3. Sign, with my coverage stated.** L8 and L9 are the best-supported material in the topic
and I attacked neither in `157` and neither here. I am signing on the ledger's account of three
instruments plus an argument, which I did not verify.

**Clause 4. Sign with two amendments.**

First, the conditional marker is present and correct, and it is the only one in the statement, which
is the defect sections 3 above and 2 of `162` are about.

Second, one sentence is loose in a way that matters for exactly the question it is marking:

> This holds over signatures whose operations are functions of the value set and the realisation
> map; an observation of the container is outside that class and splits every class it touches.

"That class" is a class of **signatures**, and an observation is not a signature, so the sentence
type-checks only on the intended reading, which is that a container observation is not such a
function. Say that. It is one word: *is not such a function*. This is not pedantry, because the
loose reading ("outside the operation set") makes the conditional vacuous: if observations are not
in the operation set at all, no signature can contain one, and clause 4's premise can never be
violated.

**Clause 5. Sign, having conceded the condition it repaired.** Section 1 above. The lens formulation
is untouched and the sole-occupancy condition is right in all three directions I could construct.

**Clause 6. Refuse as written; sign the amended form.** On the footprint-observable branch its first
sentence is false, and it carries no marker. `162` section 2 found this and I verified it at source
rather than deriving it. I refuse the row rather than amending it because the defect is not in the
wording: the sentence states a fact that one of op's two branches negates, and until he rules there
is no wording that is true on both. The honest form is the marker.

Its second and third parts are untouched by the premise and I sign them: the must-not/may
parameterisation rule (L27) and the three-armed cost of a missed merge (L26). Note that the third
part is what makes section 3's finding bite: the storage boundary is the armless case, so a spurious
verdict on the carrier pair lands its whole cost on the path I17 protects.

**Clause 7. Sign, on the ledger's account.** The refinement's location is `112`'s and `111`'s, and
`157` F157-9 relocated only what the zero rests on, which the ledger carries accurately at L11. I did
not attack the grade machinery in either round and I do not now.

**Clause 8. Sign with amendment: L15's rung.** `162` refused the TWO+ INSTANCES rung on the
entailment's ground. **I concur, verified at source and not derived.** `109` section 11
(`109:525-557`) offers the criterion, grounds "const-available" explicitly in op's I13 widening
("the criterion inherits that scope and I did not choose it"), and then predicts six answers. There
is no argument anywhere in it that the "must" is compelled: no mechanism, no emitted code, nothing
that would fail if the parameter were left runtime. The compulsion is `154` section 2's, on
`154_probes/p1_saturation/sat.s`, and it is one instance. The brief told me `162`'s conclusion and
its reason, so this is a check that could have caught a wrong claim and is not a second arrival.

**Clause 9. Sign with amendment.** Two amendments, sections 2 and 3 above: R10 should name which
half of S-14 died, and clause 9 needs a conditionality marker of its own. The three-outcome form
itself I sign without reservation; I reproduced its middle outcome on a separately written model.

**Clause 10. Sign.** `160` 4.1's assembly, and the half of it that is mine, S-17's witness
monotonicity, is carried with `160`'s caveat attached correctly: separations never degrade, refusals
must be evaluated at the maximal set. That caveat is the repair to my S-17 and it is right; a
spurious verdict can convert to a separation, and I had stated monotonicity as though it covered the
whole scheme rather than the witness branch.

**Clause 11. Sign, not my material.** `109` and `110`'s, converged, and I checked nothing.

**Clause 12. Sign, and record that a keeping was carried accurately.** L28 credits `154` section 7
as a second blind arrival and cites `157` 1.3's classification of it as "a genuine convergence with
the shared bench corpus named". That is what I wrote, with the qualification intact rather than
flattened to a bare TWO+ INSTANCES, and a compression that keeps a qualification it could have
dropped is worth saying so about.

**Clause 13. Sign, not my material, unchecked.** `109` section 8's, third instance per `106`, with
the truncation-only bound and the unrun nearest attack both carried.

---

## 5. R16: sign the retirement, and the reason is narrower than the defect in a specific way

`162` section 5 amends R16's reason from "undischarged" to "asks for the wrong kind of evidence". I
am the author of the superseding decomposition and I agree, and I can state the amendment more
exactly than either file has.

A transfer argument moves a property established at a model width to a real width. R16's retired
sentence asks for one. **Adequacy has no quantity to transfer.** Soundness is a factoring property
of the code, not a property of a width, so no width appears in it and there is nothing a transfer
argument could act on; that is `157` 3.2 and it is an argument rather than a sweep. Completeness at
a pair is discharged at the real width directly by one witness, so the result is never obtained at a
model width in the first place; that is F157-6, compiled at every width from 1 to 64.

So R16's reason should read: **not that the transfer argument is missing, but that adequacy contains
no model-width result for a transfer argument to be about.** A missing argument might arrive
tomorrow. A category mismatch does not.

The retirement itself is right and I sign it. And `162`'s separate point, that its F159-2 does not
bear on R16, is correct: F159-2 is about soundness's enforceability across builds, which is a
different half of a different claim.

---

## 6. The ledger and the retirements: signed, with two entries missing

Signed as written: L1 through L9, L11 through L14, L21, L24 through L30, C1 through C5, R1 through
R9, R11 through R15.

Signed with the amendments above: L15 (rung), L19 and R10 (which half died), L23 (three directions,
not two), L16 through L18 and L20 (carried accurately; L20's rung is `162`'s to amend and it did).

**Two things the retirement list should carry and does not.**

**First, `OPTIONS.md` Q52's literal sentence.** Q52 says at `OPTIONS.md:2502` that "rounding at
`F = 0` is observable the moment anyone writes a non-grid literal". `157` F157-5 measured that false
as stated: four of six non-grid literals tested separate nothing, including `1/2` and `1/3`, because
a tie under ties-to-even and truncation both land on the same grid point
(`157_probes/p1b_literal_ties.out`). The underlying claim survives on `111` F111-5, which used a
**dense** rational sample and named its three exceptions; the register's compression dropped both
qualifiers. `161`'s clause 4 says "a full literal", which is correct and carries none of the error,
so nothing in the statement is wrong. But the register still holds the sentence, `161`'s do-not-cite
list is the place that fixes that, and F157-5 appears nowhere in the candidate. **Add an R-entry.**

**Second, `157` S-13, which is a live proposal the candidate neither adopts nor records.** S-13
asked that "declared operation set" become "observation set", on the ground that what decides
identity is what can be observed rather than what can be computed, and that topic eight reached the
same word from the other end (`156` item 1's "visibility under the maximal observation set"). The
candidate keeps "operation set" in clauses 1, 2 and 4 and then has to say, in clause 4, that an
observation is outside the class. Grep returns zero occurrences of "observation set" in the
candidate. This is exactly the shape the standing discipline warns about: an unresolved proposal has
no result attached, so a compressor has nothing to grip and it falls out. **Either adopt it, or
carry it in section 7 as an option with its discriminator, which is whether any clause must
quantify over a thing that is observed and not computed. Clause 4 already does.**

---

## 7. Anchors, options and the accounting instrument

**My dropped anchors: four, and none is a loss.** `grep -nE '^157:' dropped_anchors.txt` returns
`157:1011-1013`, `157:288-294`, `157:35-36`, `157:858-861`. Opened, all four:

- `157:1011-1013` and `157:35-36` are the two statements of F157-10's sweeping sentence, which R3
  **retires**. Dropping the address of a retired sentence is correct and I would object if it had
  been kept.
- `157:858-861` is my paragraph quoting `109:649-651`. The candidate cites `109:649-651` directly at
  L20, which is restoration from the establishing source rather than from the document that carried
  it. Correct, and it is the rule working.
- `157:288-294` is F157-3, whose count R2 corrects from five instruments to three while carrying its
  substance at L22. Cheap to restore and not needed; the corrected form is what should be cited.

**Zero of my probe directories were dropped**, checked by `grep -n '157_probes' dropped_anchors.txt`,
which returns nothing.

**My live options survived.** Q157-C and Q157-E are both in section 7 with costs and discriminators.
Q157-A is carried as `156` item 1 and X1, Q157-B is closed and carried at L18, Q157-D is closed and
carried at L20. `163_probes/p5_output.txt` is the enumeration, with F157-6 as its positive control.

**Six of my thirteen findings are not named by number**, and I checked each rather than counting.
F157-3, F157-8, F157-11 and F157-13 are carried in substance under other words or numbers; F157-5 is
the missing retirement in section 6; **F157-12 is delivered rather than dropped**, since it said
topic five's answers were in member files and in no compression, and this candidate is the
compression. **F157-2 is the one genuine degradation**: it recorded that two of the cold pair's five
overlaps are one instance wearing two hats, which is a warning against a rung nobody has yet
claimed. No rung in the ledger is inflated by its absence, so nothing is wrong today; it is a guard
that is gone.

**I verified the accounting instrument, which `162` section 9 says it did not.**
`163_probes/p3_audit_the_accounting.py`, two tests with their controls:

```
candidate anchors, strict pattern : 19
candidate anchors, loose pattern  : 19
H1 (loose >= strict)              : PASS
seen by the loose pattern and not the strict one: 0

108:825      carried by some source: True   -> not novel
108:827      carried by some source: True   -> not novel
82:770-774   carried by some source: False  -> MUST BE REPORTED NOVEL
strict novel set (what the instrument reports): ['82:770-774']
B verdict: PASS
```

A deliberately looser extractor, which needs no backticks, finds exactly the same nineteen anchors,
so the instrument is not missing citations a reader would see. And the candidate's three citations
into files outside the thirteen sources are accounted for correctly: the two into `108` are carried
by sources and are properly not novel, and `82:770-774` is properly the one novel anchor. I also
re-ran `count_anchors.py` and it reproduces `accounting.out` byte for byte.

**One wording defect in section 8.** Its closing sentence is "Zero novel anchors means nothing here
cites material outside the thirteen sources plus the governing files." The candidate has **one**
novel anchor, reported three lines above it and explained two paragraphs above that. The sentence is
either a vacuous conditional or a false claim about this file, and a reader takes the second. Say
one, or say what one means.

**And the check `161` section 8 defers is the one that matters, so I ran it for my own entries.**
`163_probes/p4_output.txt` opens each anchor the ledger names for an entry compressing `157` and
prints what is there: `157:358-362`, `157:695-701`, `111:531-535`, `111:552-553`, `112:934-937`,
`111:555-556`, `109:649-651`, `111:1175-1176`, `82:770-774`. **Every one holds the claim the entry
makes of it.** The control, an anchor asked whether it states something it does not, shows text that
does not support it, so the check is not reading confirmation into whatever it finds.

---

## 8. What it should carry and does not

**One, and `162` named a second I agree with.** `162` section 7's point that `154` section 1's
three-senses finding lives only inside option O-E is right and I sign it: a finding that dies with
an option is a finding with no home, and the three senses are what makes `161`'s own choice to use
the word in the denotational sense throughout a choice rather than an accident.

**Mine is the coupling in section 3.** X3 is carried as an inter-topic pointer on the ground that
its disposition is topic eight's. That was true of the candidate as written. It is not true of the
candidate as it now stands, because clause 9's satisfiability depends on X3 and on `156` item 1
jointly, and neither is decidable from inside this topic. A question that decides whether a clause
can be satisfied at all is not a pointer.

---

## 9. Coverage, bounded honestly

**Read in full:** `161`, `160`, `162`, `INTENTS.md`, `RULES.md`, and my own `157`.

**Read at the cited sections, opened rather than remembered:** `109` section 11 in full and
`109:645-656`; `111` at 531-535, 552-556, 1175-1176; `112:904-945` and `112:934-937`; `82:770-774`;
`108:820-830`; `OPTIONS.md` Q52 at the literal sentence; `161_probes/anchor_accounting/` in full
including the script; `161_probes/anchor_accounting/dropped_anchors.txt` grepped for mine and the
four opened; `warm-container-shared/src/lib.rs` at the `Carrier` declaration and the fork comment;
`bitpack-shared/src/lib.rs` at the column and the pack call; `154_probes/p2_fibre/fibre.rs` at the
two instances.

**Grepped, not read:** `158`, `159`, `110`, `114`, `155`, `156`, `153`, the rest of `111` and `112`,
`AGREEMENTS.md`, `DROPLIST.md`, and every panel file below 109.

**Not opened at all:** `63`, `74`, `90`, `106`, `122`, `123`, `124`, and all of topic eight. So
every clause I sign that rests on those rests on `161`'s account of them, which is one compression
deep, and X3's disposition in particular I could not check.

**Which of my sections would move if something I leaned on is wrong.**

- **Section 3 is the load-bearing one and it rests on a model, not on arvo.** It assumes two markers
  over one `(I, F)` can agree in value set and realisation map. If a strategy always changes some
  denoted answer, reading 2 holds, clause 9 is satisfiable on both branches, and my finding narrows
  to "clause 9 is conditional on X3" rather than on both. It does not disappear: X3 is open either
  way and clause 9 carries no marker for it either way.
- **Section 1's third direction assumes a sole occupant at a nonzero offset is a thing a design
  would build.** If every real placement is offset-zero-or-shared, my third case is a fact about the
  language and not about arvo. The discriminator is unaffected, because sole occupancy classifies it
  correctly whether or not it occurs.
- **Section 8's coupling rests on my reading of clause 9's "shipped instantiations" as ranging over
  type instantiations rather than over primitives.** `161` L15 and clause 8 make the type the
  carrier of const-available parameters, which is why I read it that way; if the intended range is
  primitives rather than types, the carrier pair is one primitive under both branches and my finding
  becomes a question about what "instantiation" denotes, which is then itself worth stating.

**Citations checked by opening them.** `163_probes/citecheck.out`, with two deliberately wrong
citations as controls.

**What I settled.** Nothing that was open. A signature is not the place for it.

**What I moved.** My own two superseded sentences from conceded to conceded-with-the-third-case and
conceded-with-the-mechanism; the conditionality count from three to at least four; the accounting
instrument from unverified to verified; and X3 from an inter-topic pointer to a coupled premise.

**What I could not.** Choose any branch of the container premise or of X3, which are op's. Verify
the clauses resting on topics I did not read. Price anything: C5 remains **unpriced**, no harness
ran in this dispatch, and the two compiled probes here are compile-and-run checks that establish
structure and decide no magnitude.
