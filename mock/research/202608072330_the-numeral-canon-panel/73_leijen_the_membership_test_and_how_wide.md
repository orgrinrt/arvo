# 73. The membership test is two tests, and neither of them is about magnitude

**Author lens:** Leijen. What a type-theoretic feature costs to run, evidence passing, precise
counting, and the habit of stating a mechanism's price in the same sentence as its design.
**Position:** closing the number-systems unit's second half, after `70`, `71` and `72`. This is
an argue-and-converge dispatch, not a cold derivation: I read the panel before writing, so every
agreement below is a read plus my own derivation and I mark which is which. `72` landed while my
probes were running and is read in full.
**Probes:** `73_probes/`, nine instruments, each committed with its output before the section
citing it was written (`584072ec`, `c9f46818`, `004384e3`, `24fc13f8`). Pinned `nightly-2026-05-28`
(`73_probes/p0_toolchain.txt`), zero feature gates, no `dyn`, no `TypeId`, no `alloc`, no `std`.

**The assigned question.** What is a number system's membership test, and how wide does the
concept reach. Q20 and Q21.

**The answer in one line, before the argument.** There is no single membership test, because the
word is doing two jobs: **is this a number system** and **can arvo carry it** are different
questions with different answers, different decidability, and an inhabited two-by-two between
them, and every candidate test this unit has produced answers sometimes one and sometimes the
other. Separated, both are answerable and neither enumerates, so Q20's open reading costs
nothing. And Q21's "how wide" then turns out not to be about magnitude at all: it is `67`
section 7's question about whether the ambient domain is a parameter, and I can show
mechanically that every order-based or magnitude-based reading of it puts a kernel item that op's
I3 demands on the same side as the candidate the narrow reading was invented to exclude.

## 0. Gates, what I ran, and what I did not

**Canon gate: passes, situation two.** `mock/canon/` does not exist, `mock/crates/` is empty by
the declared mutation order (`mock/Cargo.toml` says so in its own header), and this panel is
writing the first canon. There is nothing to defend and nothing binds but op's intents. I re-read
`INTENTS.md` in full before writing. Nothing below settles anything.

**Test gate: no suite exists.** `cargo test --workspace` in `mock/` returns "the manifest is
virtual, and the workspace has no members", which is the mutation order rather than a defect. The
substitute is the probe discipline, and this file applies it to its own instruments hardest at
the place `72` section 2 taught the unit to look: **p1 reports a column of zeros, and p1c exists
solely to find out whether that column can be made nonzero.** It cannot, by four mutations
including two reductions nobody would ship, and the honest consequence is that p1's zero column is
not a measurement and is not cited as one anywhere below.

**Read end to end:** `INTENTS.md`, `RULES.md`, `69`, `70`, `71`, `72`, `67`, `68`, `65` (both
phases), `66` (both phases), `63` sections 3, 6 and 7, `OPTIONS.md` Q18 through Q28 and the
standing section, `DROPLIST.md` in full.
**Opened at the source to check a specific claim:** `71:464-469` and `71:675-682` (the membership
test and X3), `72:227-230` (its acceptance), `67:440-446` and `67:641-645` (the D-family question
and K5), `63:625-631` and `63:659-673` (C2 and C6 with its provisional marking), `63:219-222`
(the four inhabited law cells), `65:65-66` and `65:80-86`, `66:286-288` and `66:455-459`,
`68:314-337` and `68:196-211`, `70:369-384`, `OPTIONS.md:1604-1617`, `DROPLIST.md:106-108`,
`69:125-135`, and four generated files in `.claude/` and `mock/agent/` for section 9.
**Not read:** `01` through `62` except as reached through `63`, `65` through `68`, `70` through
`72`, `OPTIONS.md` and `DROPLIST.md`; `seed/`; `archive/`. Every statement here about `16`, `35`,
`42`, `51`, `55` through `62` is sourced to one of those and inherits their errors if any.
**Not re-run:** any other member's instrument. `72` re-ran `71`'s p1 and `68` re-ran `65`'s and
`66`'s; I rely on those reports rather than duplicating them. My p1b independently measures a
number two files attribute differently, which is the one place I check another member's
arithmetic, and it is to settle an attribution rather than to audit anyone.

**I built no bench and nothing here is priced.** Every count below is a count of counterexamples
from a committed probe. Where a sentence sounds like cost it means "does work a machine performs"
and never a magnitude.

**Both of op's in-flight questions are carried in section 10 rather than assumed**, and I say
there where my reasoning would move under either answer. The short version, stated up front
because it is unusually clean here: **nothing in sections 1 through 8 moves under either.** The
arithmetic is arithmetic and the type-level demonstration in section 6 has an erasure claim that
depends on Q-B and is separable from everything else in the section.

## 1. The word is doing two jobs, and that is why the tests keep disagreeing

Before attacking anything, the frame, because it is what makes the rest of the file converge
rather than accumulate.

Four candidate admission tests are now on the table, and they were produced by four members
answering what reads as one question:

- **`65` section 7 and `63` section 7**, "the concept is closed; the inventory is open", with an
  admission contract: a candidate supplies its carrier, its operation family with totality
  statements, its law inventory with bounded failures, and its correctness relation.
- **`68` section 5**, three residue clauses, sharpened at `68:335-337` to "a system is admissible
  exactly when its **ingest predicate** is writable as a pure function of (type parameters,
  bits)".
- **`71` section 7**, "a system is a member when it can expose prefix 3, that is, when it names an
  ambient domain, a representable set that is a constant of the type, and a reduction onto that
  set whose two law verdicts are decidable" (`71:466-469`), carried into X3 at `71:675-679`.
- **`70` L3 and L4**, which do not propose a test but re-key one clause of the others: "constant
  of the type" splits into on-whom and resolved-when, and a target-owned representable set is
  constant at every monomorphisation (`70:369-384`).

They are not four attempts at one predicate. **Two of them answer a mathematical question and two
answer an engineering one**, and once that is said the disagreements between them stop being
disagreements.

- **Membership.** Is this thing a number system at all? This is a question about structure. Exact
  rationals with unbounded denominators are a number system. So is the field of reals. So, if the
  concept reaches that far, is GF(2)^n. Nothing about arvo enters.
- **Hosting.** Can arvo carry it? This is `68` section 5's question, and its answer is the three
  residue clauses: bounded information at rest, monomorphic operation selection, legality from
  (type, bits) alone. Exact rationals with unbounded denominators fail it while remaining a
  perfectly good number system.

**The two-by-two is inhabited**, which is the standard this unit already uses for showing two
properties independent (`63:219-222` for the law families, `71`'s p3 for the crossing
preservations):

| | hosts | does not host |
|---|---|---|
| **is a system** | the windowed integers under any reduction; GF(2)^n | unbounded exact rationals; arbitrary-precision integers; continued fractions of unbounded depth |
| **is not a system** | a Gray code; two's complement; a container width | a validity rule that consults a locale, over an unbounded carrier |

The bottom-left cell is the one that matters and the one nobody has named. A Gray code is
hostable, is not a number system, and is not a non-member either: it is a **member of the concept
located at a different coordinate**, which is section 7. The panel keeps asking a yes-or-no
question of candidates whose real answer is a coordinate.

So the assigned question splits, and the rest of this file answers it in that shape: section 2
through 6 are about the membership test, sections 7 and 8 are about what its output should be and
about Q20, and hosting is `68`'s and I add one asymmetry to it in section 8.

## 2. The proposed membership test fails in its sufficient direction, and the failure is cheap

`71`'s prefix-3 test is the best candidate the unit has and `72:227-230` accepts it. I attacked
it, and one half of it does not hold. I want to be exact about which half, because the repair is
additive and the surviving half is the important one.

**The necessary direction survives untouched.** `71:679`, "a system that cannot exhibit the three
cannot be crossed into and composes with nothing", is right, and its argument (the three are
exactly what one system must show another for a value to move between them) is the strongest
thing in that section. Nothing below touches it.

**The sufficient direction fails.** The test as stated at `71:466-469` and accepted at `72:227`
is a biconditional in use: a candidate is a member **when** it can expose the three. Exposing
them is free.

**Every system has a second declaration of itself.** Take its own induced operation
`f(a, b) = rho(a op_D b)`, and declare `D' = (Q, f)` with `rho' = the identity on Q`. That term
satisfies the exposure list verbatim. It names an ambient domain. It names a representable set,
which is a constant of the type. It names a total reduction onto that set, whose law verdicts are
decidable. And it computes the identical function.

`73_probes/p1` measures what it reports (`73_probes/p1_output.txt`), exhaustively at the 4-bit
model width, over six declarations:

| declaration | ambient assoc | coherence fails | monotone fails | induced assoc fails |
|---|---|---|---|---|
| i4 saturate add, honest | 0 of 29791 | **322 of 961** | 0 of 961 | 952 of 4096 |
| i4 saturate add, collapsed | **952 of 4096** | 0 of 256 | 0 of 256 | 952 of 4096 |
| i4 wrap add, honest | 0 of 29791 | 0 of 961 | **232 of 961** | 0 of 4096 |
| i4 wrap add, collapsed | 0 of 4096 | 0 of 256 | 0 of 256 | 0 of 4096 |
| gf(2)^4 xor | 0 of 4096 | 0 of 256 | 0 of 256 | 0 of 4096 |

Two things in that table, and only the first is a measurement.

**The measured part.** The honest declarations fail the law families in the places the panel
already knows they do: signed saturation fails coherence, wrapping fails monotonicity, and the
induced associativity failure count for signed saturating addition is 952 of 4096, which is
section 9's second report.

**The part that is true by construction, said plainly rather than sold as a result.** Every
collapsed row reports zero on both law families. `rho' = id` is monotone under any order and is a
homomorphism onto its own operation by definition. `72` section 2 established that a row which
cannot fail is not a measurement and that the way to show it cannot fail is to corrupt what it is
nominally about. `73_probes/p1c` does exactly that, with four reductions
(`73_probes/p1c_output.txt`):

| reduction | honest coh / mono | collapsed coh / mono |
|---|---|---|
| honest saturate | 322 of 961, 0 of 961 | 0, 0 |
| constant zero | 0, 0 | 0, 0 |
| opposite bound (`56`'s mutant) | 633 of 961, 281 of 961 | 0, 0 |
| parity scramble | 735 of 961, 189 of 961 | 0, 0 |

The mutation set breaks the honest verdicts in three of four cases and reaches the collapsed
verdict in zero of four. So this is not a fact about four bits and not a fact about saturation:
**a membership test that reads a candidate's law verdicts off the pair the candidate itself named
is passed by a reduction that adapts every ambient value to zero.**

**And the second hole, which the mutation found and I did not predict.** Look at the
`constant zero` row again. Under its **honest** declaration, with the ambient domain being genuine
integer addition, it fails nothing: zero coherence failures, zero monotonicity failures, and its
induced operation is associative. A numeral that names every value zero satisfies both of the
verdicts `71`'s X3 asks for, and computes nothing. That is not the collapse. It is the exposure
list being one clause short even when the candidate is honest.

## 3. What the exposure list is missing, derived from what breaks it

Two holes, two repairs, and neither is a new mechanism. `73_probes/p1d` runs eight shapes and
their eight collapses against five candidate clauses (`73_probes/p1d_output.txt`).

**Repair one, for the constant-zero hole: the retraction clause.** A reduction may not move a
value that is already representable. `p1d` shows the clause is independent of the two verdicts in
both directions, which is what makes it a clause rather than a corollary: `u4 constant zero`
passes monotone and coherent and fails retraction, and `i4 opposite bound` passes retraction and
fails both verdicts. This is not a new idea in the panel; it is the shape `DROPLIST.md:106-108`
records the project adopting once already, where two round-trip identities were refuted and
replaced by "the section-retraction triple". It arrives here at the admission contract rather than
at the crossing contract, for a different reason, which is the third time this panel has watched
that shape turn up somewhere new.

**Repair two, for the collapse, and it is the one worth the file.** The instinct is to forbid the
collapse. That instinct is wrong, and `p1d` shows why in one column: of the eight collapsed rows,
**five keep ambient associativity and three lose it.** For `i4 wrap`, `u4 wrap`, `u4 saturate` and
`gf(2)^4 xor`, the collapsed declaration is not a laundering at all. It is a true and useful second
description of the same system, and for GF(2)^4 it is the **only** honest description, because
xor is closed and there is nothing for a reduction to do. Forbidding the collapse would forbid
`65`'s K5 outright.

So the repair is not a prohibition on the reduction. **It is an item the exposure list does not
have: the ambient domain's own law inventory.** `p1d` establishes the reason as a biconditional:

> induced operation associates **iff** (ambient operation associates on the reachable set **and**
> the reduction is coherent)

sixteen of sixteen cells, with both sides observed true and false, and with **neither conjunct
alone predicting it**: ambient-only disagrees on three rows and coherence-only disagrees on three
rows. With that item in the exposure list the collapse becomes harmless, because a collapsed term
reports its own ambient's laws honestly and the conjunction returns the right answer: collapsed
signed saturation reports its ambient as non-associative, at the same 952 of 4096, and is refused.

**This is `63` C6's frame tested at a place C6 was not written for, by a third instrument, and it
held.** `63:665-673` marks C6 as provisionally passing equivalence and says plainly that "nobody
has tried to break the frame" and that this "is the attack this file most wants made next". I made
it, with a mutation set chosen to break things rather than to confirm them, including a reduction
that maps everything to zero and one that scrambles on parity. It did not break. That is an honest
failed refutation and not a proof, exactly as `57`'s failed refutation of the ladder was
(`63:233-235`), and it is worth what a failed refutation is worth: C6's provisional marking has now
survived a deliberate attempt, from a direction it was not built to face.

**And the third verdict is the weaker of the two candidates, which is not the obvious choice.**
The retraction is implied by the distance-minimising law that `63:216-218` names beside monotone,
so the tidy move is to demand the stronger one and get the retraction for free. That move is
wrong, and `73_probes/p5` measures why (`73_probes/p5_output.txt`): distance-minimising **excludes
both wrapping rows**, which are kernel item K1 and which `65:258-259` derives from op's I3, while
the retraction excludes no kernel item and still excludes the reduction that computes nothing.
Eight shapes, the implication holding in every row and strict in three of them.

That is the second time in one unit that the stronger, better-sounding clause turns out to throw
out something op's own intent demands: section 4 is the first, for an order, and this is the
second, for a distance. Both are the same reflex, and the lesson is worth one sentence in a
consolidation: **wrapping arithmetic satisfies fewer of the laws a numeral is expected to satisfy
than almost anything else the concept contains, and it is not negotiable, so any clause proposed
as a boundary of the concept is tested against wrapping first.**

**The corrected exposure list, offered as a candidate rather than a settlement.** A system
exposes: its ambient domain **together with that domain's own law inventory**; its representable
set; and its selected reduction with that reduction's verdicts, of which there are three and not
two, the third being the retraction. Read against `71`'s X3, the change is one added item and one
added verdict, and X3's necessary direction and its crossing argument are untouched.

## 4. Q21's discriminator is empty, and this is measured over every order rather than argued

`OPTIONS.md:1611-1617` states Q21 as broad against narrow, with the narrow reading being
"ordered value sets with a notion of magnitude" (`66:286-288`) and the broad one naming the
two-element Boolean algebra and GF(2)^n as systems "not about magnitude at all" (`65:65-66`). The
register's own note is that one cold derivation leaning broad is one instance and one instance
decides nothing.

I am not a second instance and will not be counted as one: I read `65`, `66` and `71` before
forming a view. What I can do instead is take the narrow reading's own discriminator seriously and
find out whether it cuts. It does not.

`73_probes/p2` enumerates **every total order** on the carrier at widths 2 and 3, which is 24 and
40320 orders, and tests each for compatibility with the operation in both arguments. It is
exhaustive over orders, not a sample (`73_probes/p2_output.txt`):

| operation | compatible orders, w = 2 | compatible orders, w = 3 | natural order at w = 4 |
|---|---|---|---|
| wrap add, Z/2^n | **0 of 24** | **0 of 40320** | 680 failures |
| xor, GF(2)^n | **0 of 24** | **0 of 40320** | 960 failures |
| and, lattice meet | 0 of 24 | 0 of 40320 | 220 failures |
| or, lattice join | 0 of 24 | 0 of 40320 | 220 failures |
| min, tropical add | 8 of 24 | 128 of 40320 | 0 failures |
| saturating add | 2 of 24 | 2 of 40320 | 0 failures |

**The first two rows are the finding.** Wrapping addition is kernel item K1, which `65:258-259`
derives from I3, op's four-times-restated call that "it should behave like native primitives in
regular old rust would". GF(2)^n is the candidate the narrow reading exists to exclude. They
receive **the same verdict from every order-based test run here**, and for the same structural
reason, which the probe also prints: both are finite groups, every non-identity element has finite
order, and a translation-invariant total order on a group forces `a > e` to give `a^k > e` for
every k, which finite order contradicts. So the two zeros are not a fact about widths 2 and 3; the
enumeration is that argument checked, and the conclusion holds at every width.

**And the same test splits a kernel item down the middle.** Wrapping and saturating over one
window are both K1, both demanded by I3 because Rust surfaces both, and they land on opposite
sides of the discriminator.

An order-or-magnitude reading of Q21 therefore excludes something op's stated intent demands and
admits nothing it excludes. It is not a boundary of the concept. It is a property some reductions
have and others do not, which is `63` C4's adaptation-law family (`63:216-219`, monotone facing
the source) under a different name, sitting at telescope coordinate 3 rather than at the concept's
edge.

## 5. So Q21 is `67` section 7's question, and the register carries them as two

Take the discriminator away and ask what is actually being disputed. It is this, and `67:440-446`
already posed it and declined to answer it:

> is D's operation family fixed at (+, x), or is it a parameter?

**Q21 and that question are the same question.** If the ambient operation family is a parameter of
the concept, then GF(2)^n with xor, the Boolean lattice with meet and join, and the tropical
semiring with min are ordinary members, each a different D over the same carrier, and the broad
reading follows with nothing added. If it is fixed at (+, x), all three are outside, and the
narrow reading follows, taking the tropical semiring with it.

That last clause is what makes the collapse of the two entries worth having. **The narrow reading
does not only exclude masks. It excludes the arithmetic that op's own named selling point computes
in.** I11 says "our main selling point are the algo crates that hilavitkutin, vehje, pretty much
every single repo and project I have, downstream, use", and `67` section 5 established, with `35`
measured from the algorithm side, that those crates' arithmetic is min and plus rather than plus
and times. A concept that fixes D at (+, x) does not describe what the graph crates compute in.
Nobody weighing Q21 as a question about whether a bitmask is a number has been weighing that.

**So Q21 does not need a second cold derivation on masks.** It needs an answer to `67`'s question,
which is a real fork with real costs on both sides, both of which `67:571-584` writes out. My
contribution is that the two entries are one, and that the magnitude framing should go, because it
names a criterion that has been measured not to cut.

## 6. The broad reading costs no mechanism, and the repair is expressible as a bound

A canon must be able to say which things are doable. `73_probes/p3_membership_contract.rs`
compiles clean on the pin, exit 0, zero warnings, zero feature gates, `#![no_std]`, no `dyn`, no
`TypeId` (`73_probes/p3_positive.txt`).

**H1, the broad reading costs zero mechanism.** GF(2)^4 under xor and Z/16 under wrapping
addition are terms of one contract, accepted by one bound, with no coordinate added for either and
no second concept anywhere. The magnitude-free carrier is the term whose reduction is the identity
**because its ambient operation is closed on its representable set**, which is `67`'s K5
(`67:641-645`) used as a construction rather than stated as a remark. And the closure fact is
**computed rather than declared**: two `const fn` loops of identical shape return `true` for xor on
the window and `false` for integer addition on it, both asserted at compile time, so neither
assertion is a tautology because one of them is false. That discipline is `68` section 2.2's
finding applied before it could bite: an overstated declaration passed `65`'s validation suite with
`EXIT=0`, and the repair is that a declaration is worth nothing unless a check runs through the
maps.

**H2, the repair is a blanket implication rather than a per-instance assertion.** The conjunction
p1d measured is written once, as an implication from coherence plus the ambient domain's own law
to the induced law, so a design cannot state one half and forget the other. Every law
implementation in the probe is a transcribed measurement from `73_probes/p1d` with the
transcription stated at the impl, so nobody cites an impl as a fact.

**H3, and this is the half that matters, because both designs are writable.** The same file
carries two folds. One is bounded on the conjunction. The other is bounded on the reduction's own
verdict alone, which is what an exposure list omitting the ambient's laws hands a consumer. The
collapsed signed-saturating term is **honestly coherent**, so:

- `reassociating_fold_verdict_only::<IdentCollapsed>` **compiles**, in the positive file, on
  purpose. The unsound design type-checks.
- `reassociating_fold::<IdentCollapsed>` is **refused**: `error[E0277]: the trait bound
  SatOwnAlgebra: AmbientAssociates is not satisfied`, with the note chain naming the blanket impl
  and the bound it flows through (`73_probes/p3_n1.stderr`).

So the difference between the sound and the unsound admission contract is a compile outcome rather
than an argument, and **the typestate does not choose between them.** That is the same shape `71`
section 4 found for the ordering question at `71:341-344`, arriving at the admission contract from
a different direction: both programs are right programs, and only a canon sentence says which bound
a design carries.

Two further negatives, each generated from the positive file by a committed script
(`73_probes/p3_negatives.sh`):

- **n2**, a reduction declared over one ambient attached to a reach over another: refused. My
  prediction was `E0271`, matching `67`'s p1; the diagnostic is **`E0053`**, because in this
  spelling the dependency surfaces in the method's own signature rather than in an associated-type
  equality. The substance held and my prediction of the code did not, and I record the miss. What
  it adds: the telescope's dependency is enforceable at either place, and which diagnostic a design
  gets is spelling rather than structure, so neither error code belongs in a canon sentence.
- **n3**, asserting the false closure fact: refused at const evaluation,
  `error[E0080]: evaluation panicked: assertion failed: add_closed_on_window()`.

**Erasure, stated with its bound.** Three `const` assertions that the carrier newtype has the
container's size, discharged at compile time. That is layout erasure and `68:158-163` is right that
it is close to a language tautology; I claim nothing beyond it, and nothing here is priced.

## 7. Admission's output should be a coordinate, not a boolean

The constructive part, and it composes `67`, `70` and `71` rather than competing with any of them.

The bottom-left cell of section 1's table is full of things that are hostable and are not number
systems: a Gray code, two's complement, offset binary, a container width, `Cold`'s stride. Asking
each of them "are you a number system" gets "no" and throws away the answer that was wanted.
**Each of them is a choice at a coordinate of the same chain**, and the telescope already names
which:

- Gray code, two's complement, offset binary, signed-digit: **coordinate 4**, the encoding. A
  change there preserves the value-level operation and destroys the pattern-level one, which is
  `71`'s p1 index-4 row. **Cited as `72` section 2 requires rather than as `71` presented it:** the
  value half of that row is true by construction and not a measurement, since the system's own
  operation reads no coordinate past the third, and `72_probes/p1` demonstrates it by mutation. The
  pattern half is a measurement, and `72`'s own table is the better one for it.
- `Cold`'s stride, alignment, a wider housing: **coordinate 5**, the container, with `70`'s
  ownership key set to the aggregate.
- Wrapping against saturating over one window: **coordinate 3**.
- A block exponent: **coordinate 2**, with `70`'s ownership key deciding whether it resolves at
  monomorphisation or per datum, which is the whole content of `67`'s p3 result read through
  `70` section 4.
- A platform-resolved width: **coordinate 2**, owner the compilation (`70` L4).

So the procedure a canon should describe is not a predicate returning yes or no. **It is a
location: which coordinate does this candidate fix, and whose is it.** A candidate that fixes
coordinates 1 through 3 is a system; one that fixes 4 or 5 is a realisation of one; one that fixes
none of them is outside the concept entirely, and that last set is much smaller than the panel's
arguments suggest.

Three things follow and each is cheap.

**It explains why the panel keeps arguing about admission.** Every disputed case in the register
is a case where a candidate fixes a coordinate at an unexpected index or an unexpected owner.
Q26's platform width is coordinate 2 at an unexpected owner. Q22's intervals are, on `63:280-281`'s
filing, a composition over coordinate-1-through-3 terms rather than a term. `67`'s block float is
coordinate 2 owned by a runtime datum. None of them is a borderline member; each is a member whose
location surprised somebody.

**It gives Q23 the same treatment, and I support `71` section 6 here with my own derivation rather
than by reading.** A role is a codomain of a crossing at coordinates 4 and 5, so storage, compute
and interchange are realisation variants of one identity. Chain extent is not a coordinate at all;
it is `63` C9's schedule. `72:232-255` attacks the clean two-way split and I think its first
objection lands: `OPTIONS.md:991-994` carries a live reading under which the compute role widens
past storage, which is coordinate 2, and `65:188-189` says the compute role may be "a redundant
intermediate", which holds what the format cannot. Under the location reading that is not a
counterexample to the frame, it is the frame working: **the compute role is a member of the role
set only under the reading where it does not widen, and the register's open question about widening
is therefore also the question of whether the role set is homogeneous.** Two register entries, one
question, again.

**And it says what a canon sentence about admission should quantify over**, which is `67`'s K2
applied to this question: a sentence beginning "every number system" means something different
under each cut, and the location procedure makes the cut explicit at the point of admission rather
than leaving it to be inferred.

## 8. Q20, and the bound the membership test actually carries

**The inventory is open, and the openness costs nothing, because both tests are non-enumerating.**
The membership test is a location on a five-coordinate chain plus the corrected exposure list of
section 3. The hosting test is `68:314-337`'s three residue clauses. Neither mentions a candidate
by name, so the canon can say what it covers without listing what it covers, which is exactly what
`OPTIONS.md:1608-1609` asks for as the thing that would distinguish the open reading from the
closed one.

**An asymmetry I have not seen stated anywhere, which I then had to narrow.** The two tests are
not decidable in the same sense. The first form of this, before I attacked it, was:

- **Hosting is decidable at any width.** "Is the ingest predicate writable as a pure function of
  (type parameters, bits)" is a typing question. The compiler answers it, at real widths, with no
  exhaustive evaluation anywhere. `70`'s p2 is that answer taken as evidence: the runtime-owned
  block exponent fails because the honest signature carries the exponent as a further argument, and
  that is visible in an arity rather than in a sweep.
- **Membership is decidable only at the model width.** Its law verdicts, including the ambient
  domain's own law inventory that section 3 adds to the list, are exhaustive checks, and `68:196`
  through `68:211` re-established inside this panel that rustc refuses the 9-bit exhaustive const
  check under `deny(long_running_const_eval)`.

which gave the draft conclusion that **the membership test inherits the transfer proviso and the
hosting test does not.** A canon sentence saying "a system is a member when it exhibits the
following" would then be, at real widths, a sentence whose antecedent is assumed uniform rather
than checked, and `DROPLIST.md`'s two-mechanism entry already carries a compiled counterexample of
a property true at eight bits and false at nine with no forbidden feature. That would put the
mathematical test on `68` section 3's trusted-base list, which is the opposite of where instinct
puts it.

**And then I attacked it, because a reported asymmetry is not a deliverable, and it is narrower
than it reads.** The ceiling is a property of a verdict rather than of the test. `63` C6
(`63:659-664`) says the congruence half is "decided by the range's geometry per operation", and a
geometric condition on a range is O(1) in the width, so wherever C6 supplies one the verdict needs
no sweep at all. `73_probes/p6` measures whether the closed form actually replaces the sweep for
the family the panel has one for (`73_probes/p6_output.txt`): clamped addition against sign
confinement, over **every window at widths 3 through 6, 2780 windows, zero residue**, exhaustive
over windows rather than sampled, with the predicate confirmed non-constant so the agreement is
not vacuous.

So the honest statement is not "membership is stuck at the model width". It is: **membership's
verdicts split into the computed and the swept, and only the swept ones carry the proviso.** For
clamped addition the verdict is a range-geometry check that runs at width 64 as cheaply as at
width 4, and the membership test is decidable there exactly where hosting is.

**The control, and it matters because it stops this becoming a general claim.** The same probe
tries the analogous geometric predicate for clamped multiplication, mirror symmetry, and it does
not hold: 23 of 36 windows agree at width 3, 93 of 136 at width 4, 377 of 528 at width 5, with
every disagreement in the same direction, measured coherent and predicted not. **That is a control
on my own naive spelling and is not an attack on C6**, which says a geometric condition decides
the congruence half rather than that mirror symmetry is the whole of it, and which is about
congruence where I measured coherence. What it establishes is only what it needs to: the lift is
per-operation, obtained by having a closed form rather than by being a membership question, and
nobody should read the additive result as licensing the multiplicative one.

**What this changes for the canon, which is the point of attacking it.** A sentence about
membership owes one distinction it does not currently make: **which of its verdicts are computed
and which are assumed uniform above the model width.** That is not a hedge on the test, it is the
form `68` section 3 already argues every guarantee here should take, and it turns a proviso that
looked like it covered the whole concept into one that covers a nameable part of it.

## 9. Two reports outside the question, per the standing instruction

**One. A count in `66` is attributed to the wrong operation, and it is checkable in one command.**
`66:455-457` writes: "the panel's measured 952 of a comparable space for signed saturating
**multiplication** at width 4". `73_probes/p1b` measures both operations exhaustively over the
4096 triples of `[-8, 7]`: **signed saturating addition fails associativity at 952, and signed
saturating multiplication at 160** (`73_probes/p1b_output.txt`). The number belongs to addition.
`63:230` carries it without naming an operation, which is how a reader acquired a wrong one, so
the repair is two words in `66` and one word in `63`. I did not open `55_probes/p4`, so this
settles which operation carries the number and does not settle what `55` measured.

**Two. `69`'s account of the generated-instruction repair overclaims, for the third time about the
same repair, and this one is grep-checkable.** `69:133-134` states that "the two pure-architecture
tables (the layer dependency table and the intent-to-crate cookbook rows) are removed outright".
Checked today at the source: the layer dependency table **is** gone and `.claude/rules/cargo.md`
says so in its place. The cookbook rows are **not**. `.claude/rules/cookbook.md:136-137` still
reads "| Graph analysis | `arvo-graph`: `topo_sort`, `rank_levels`, `waist`, `spanning_tree` |" and
"| Partition a matrix | `arvo-spectral::Laplacian` -> `fiedler_partition` -> `Mask64` |", the
`.github` twin carries them at 134 and 135, and the source template carries the first at
`mock/agent/rules/cookbook.md.tmpl:119`. A banner was added above them; the rows were not removed.

The sequence is now: `65` and `66` report the stale instructions, `69` records them fixed, `70`
finds most of the residue standing and says so, `69` is corrected, and **the correction contains
its own overclaim about the same files.** That is the fabricated-diligence shape `DROPLIST.md`
names twice in its own entries, and it is worth saying plainly rather than filing gently: the
useful lesson is not that someone should have grepped, it is that a repair reported by the person
who made it is a compression checked by its own author, and this workspace has a rule about that.
The fix is one edit to the template and a regeneration.

**Neither report changes anything in sections 1 through 8.**

## 10. Where this depends on the two questions in flight, and both branches

`69` records two questions as op's. I do not answer them and do not assume either answer.

**Q-A, which verb "validate" is.** Sections 1 through 8 do not move. The collapse is arithmetic;
the order enumeration is arithmetic; the exposure list is a statement about what a candidate must
show, not about when it is checked. The one place the two readings differ is **where the two tests
of section 1 run**, and they differ in opposite directions, which is worth stating because it
sharpens the question rather than hedging it. Under the **compile-time** reading, membership is
checked once per type at monomorphisation and hosting is checked by the type system's acceptance of
the declaration; section 6's probe is what that looks like. Under the **runtime ingest** reading,
`68` section 4's door is mandatory where bits arrive without their history, and there the hosting
test is the operative one, because an ingest predicate is exactly what a door is, while the
membership test has nothing to attach to, since the source term was never witnessed. So the two
readings do not compete for one test; **each reading makes a different one of section 1's two tests
the operative one at the boundary**, which is an argument that both tests are wanted and that
`68`'s boundary keying is right.

**Q-B, whether the long-standing constraints are op's intents.** One paragraph of this file rests
on them: section 6's erasure assertion, which rests on monomorphisation being the dispatch, which
rests on no `dyn` and no `TypeId`, which `67:48-59` and `69:37-44` both report appears nowhere in
`INTENTS.md`. If those constraints are not op's intent, that paragraph is a fact about a design
choice rather than about the concept. **Everything else is independent**, and specifically: the
hosting test of section 1 is itself derived from those constraints and is therefore Q-B-conditional
in full, while the membership test, the collapse, the order enumeration and the location procedure
are arithmetic and would hold in any language under any dispatch discipline. That is a cleaner
split than it looks: **Q-B does not threaten the concept, it threatens exactly one of the two tests
this file separates**, and the separation is what makes that visible.

## 11. Fits against the register

**Kills nothing.** No live option anywhere is closed by this file. Written out in full so a
consolidator can lift them, per the register's own convention and because two prior consolidations
each lost a live option.

**And extended in place, which `71` and `72` did not do and `RULES.md:1674-1678` asks for.** The
three new options below and the Q21 amendment are appended to `OPTIONS.md` as **Q29, Q30 and Q31**
plus a Q21 amendment, written out in full there rather than only here. The append is at the end of
the unit-three section, so no line above it moves and no existing citation into that file shifts,
which matters because four member files now cite it by line.

**Q20 (open or closed) gains its answer's shape and one bound.** The inventory is open at no cost,
because the concept has two non-enumerating tests rather than one. Section 1 for the split, section
7 for what the membership test returns, section 8 for the bound: hosting is decidable at every
width, and membership is decidable at every width **for those verdicts the law frame gives a closed
form** and only at the model width for the rest, so the transfer proviso attaches to a nameable
part of the test rather than to the concept. Measured for clamped addition over 2780 windows at
four widths with zero residue, with a control showing the lift is per-operation
(`73_probes/p6`).

**Q21 (broad or narrow) loses its stated discriminator and gains a different question.** Every
total order enumerated at widths 2 and 3 puts wrapping addition, which I3 demands, on the same side
as GF(2)^n, which the narrow reading exists to exclude, and splits wrapping from saturating within
one kernel item (section 4, `73_probes/p2`). What remains of Q21 is `67:440-446`'s question about
whether the ambient operation family is a parameter, and the entry should say so. **The count is
unchanged: still one instance leaning broad, still `65`'s.** I read it first and am not a second.

**Q23 (is the role set closed) gains a reading that composes `71` and `72` rather than choosing.**
Under section 7's location procedure a role is a codomain at coordinates 4 and 5, so the question
"is the set closed" is well formed for realisation roles and malformed for a set that also contains
a schedule. `72:236-247`'s objection is that the compute role may itself widen, which is coordinate
2; that makes the widening question and the role-homogeneity question one question, which is
section 7's third consequence.

**Q19 (are the two hierarchies one cut) gains a small support for `70`'s answer, not a new one.**
Section 7's location procedure is a partition by dependency-order position, which is `70` L1's
first instrument. I derived it for a different purpose and it does not touch `70`'s finding that
the effect partition has a non-contiguous class, so this is a support with its own route and not a
second instance of L1's joint statement.

**`63` C6 gains an attack that failed, from a direction it was not built to face.** `63:665-673`
asks for exactly this and names it as the thing the consolidation most wants. `73_probes/p1d`
returns the biconditional in 16 of 16 cells with both sides inhabited and neither conjunct alone
predicting, over a shape set chosen to break it. The provisional marking should stay, because a
failed refutation is not a proof, and it should record that one was attempted.

**A new option, written out in full: what does the admission contract expose?**
**(1) Prefix 3 with the reduction's two verdicts**, which is `71` X3 as written and `72` accepted.
Cost: refuted in its sufficient direction, by a reduction that adapts everything to zero and by
the collapsed declaration, both in `73_probes/p1` and `p1c`. Its necessary direction survives and
is untouched.
**(2) Prefix 3, plus the ambient domain's own law inventory, plus a third verdict, the
retraction.** Cost: one more item and one more verdict, and the ambient's law inventory is an
exhaustive check that inherits the transfer proviso. Buys: the collapse becomes harmless rather
than forbidden, so `65`'s K5 systems keep their only honest declaration; and the conjunction is
expressible as a blanket implication so a design cannot state half of it (`73_probes/p3`).
**(3) Admission relative to a consumer-supplied ambient domain**, with the system exposing only
its representable set and its reduction, and the ambient domain being the frame the question is
asked in rather than a field of the answer. Buys: the collapse is unstateable rather than merely
caught, because no candidate names its own D. Cost: a candidate does not determine its own
identity until a frame is chosen.

**I stated a cost for (3) that I then attacked and had to withdraw, and I record both.** The first
version of this entry said (3) "contradicts `63` C2" (`63:625-626`) and would need that sentence
reopened. It does not. C2 says a format **is identified by** its ambient domain and its
representable set, which (3) keeps intact: D stays in identity under both options. What (3) denies
is the different and unstated proposition that a candidate **supplies** every component of its own
identity, and that proposition is precisely what the collapse exploits. So the real cost of (3) is
one order of magnitude smaller than I wrote, and the two options are closer than the entry first
made them look.

**And I attacked the discriminator I proposed for them, with the same result.** The first version
said the case that would distinguish (2) from (3) is a consumer asking a question of a system
without an ambient domain in view, and named the ingest door as the candidate. **The ingest door is
not that case.** `68:335-337` states the ingest predicate as a pure function of (type parameters,
bits): it never mentions the ambient domain or the reduction, because membership of bits in a
representable set is a coordinate-2 and coordinate-4 question. So the one place a frame might be
absent turns out not to need one, and the objection to (3) from that direction dissolves.
**What would distinguish (2) from (3), restated:** whether any canon sentence needs to be true of a
system with no ambient domain named at all. I could not construct one and I do not conclude none
exists.

**A new option, written out in full: is admission a boolean or a location?**
**A predicate**, returning member or not. Cost: it discards the coordinate, which is the part a
consumer needs, and it makes every mislocated member look like a non-member, which section 7 argues
is most of the disputed cases. **A location**, returning which coordinate the candidate fixes and
whose it is. Cost: the procedure has an output type rather than a truth value, so a canon sentence
about it is longer, and it presumes the coordinate count, which `70` L2 argues the canon should not
commit to. **A location for members and a predicate for hosting**, which is the shape section 1
falls into. Cost: two procedures where a reader expects one, and the canon owes a sentence saying
why the word is doing two jobs. **What would distinguish them:** whether any canon sentence needs
to quantify over non-members, and section 7 says the useful sentences all quantify over coordinates
instead.

**A new option, written out in full: does the canon use one word or two for membership and
hosting?** **One word**, with the residue clauses folded into the concept. Cost: the concept then
excludes unbounded exact rationals as a matter of mathematics, which is false and would be read as
canon; and every hosting clause becomes Q-B-conditional inside a sentence about arithmetic.
**Two words**, membership mathematical and hosting arvo's. Cost: two admission passages instead of
one, and a reader must be told which is which at every use. **Two words with the second scoped to a
target**, so hosting is stated as "what this implementation can carry" and can differ per
compilation, which composes with `70` L4's target-indexed families. Cost: the same as two words,
plus a quantifier over compilations. **What would distinguish them:** whether the canon ever wants
to say something true of a system arvo cannot host, and section 1's top-right cell says it already
does, since the bounded windows the concept admits are defined as bounded windows **of** systems it
cannot host.

## 12. Candidate canon sentences

Each offered to the consolidation, not as a settlement, each tested against permanence (still true
and useful after a from-scratch rewrite in another language in another decade) and equivalence
(three independent implementations behave the same). Rungs stated honestly.

**M1, the word is two words.** *Whether something is a number system and whether this
implementation can carry one are different questions. The first is about structure and is answered
by locating the candidate on the chain of choices. The second is about residue at runtime and is
answered by what a value at rest may carry. Neither enumerates, both are open, and a system the
implementation cannot carry is still a system.* Permanence: passes, and it is the sentence most
likely to survive a rewrite, since the second question's answer changes with the implementation and
the first does not. Equivalence: passes; three implementations conflating the two would disagree
about whether unbounded rationals are excluded by mathematics or by choice. Rests on: section 1,
with the hosting half being `68` section 5's and the split being mine. ONE EXPERT on the split.

**M2, what a system exposes.** *A system exposes its ambient domain together with that domain's own
law inventory, its representable set, and its selected reduction with that reduction's verdicts,
including that the reduction moves no value already representable. The reduction's verdicts carry
no information alone: the properties a consumer relies on are conjunctions of an ambient law and a
reduction verdict, and a system that names its own computed algebra as its ambient domain satisfies
every verdict while voiding every conclusion.* Permanence: passes. Equivalence: passes, and
sharply, since an implementation omitting the ambient half admits a term another refuses, which
`73_probes/p3` shows as two compile outcomes in one file. Rests on: sections 2 and 3,
`73_probes/p1`, `p1c`, `p1d`, and `63` C6 for the conjunction. ONE EXPERT, and it is the sentence
I would most want attacked.

**M3, closure forces the reduction, and that is where the magnitude-free systems live.** *Where an
ambient operation is closed on the representable set, no adaptation is possible and the reduction
is the identity. Such a system exposes everything the concept asks and needs none of the adaptation
machinery, and it is a member for that reason rather than in spite of it.* Permanence: passes.
Equivalence: passes. Rests on: `67`'s K5 for the closure fact, `73_probes/p3` for the construction
and the compile-time closure check, `73_probes/p1d` for GF(2)^4 passing every clause. This is
`67`'s sentence with a construction under it; the construction is mine and the sentence is its
author's.

**M4, no order-shaped boundary.** *The concept's edge is not an order and not a magnitude. Whether
a system's values carry an order compatible with its operations is a property of its selected
reduction, it varies within a single family the design must contain, and it groups a wrapped
integer with a bit vector.* Permanence: passes. Equivalence: passes; three implementations
disagreeing here would disagree about whether wrapping arithmetic is inside the concept. Rests on:
section 4 and `73_probes/p2`, exhaustive over every total order at two widths plus the group
argument at all widths. ONE EXPERT.

**M5, admission returns a coordinate.** *Asking whether a candidate is a number system is usually
the wrong question. The useful question is which choice in the chain the candidate fixes, because
most things that are not systems are not outside the concept: they are choices at a later
coordinate. An encoding, a container, a stride and a housing are all answers to that question and
none of them is a rejection.* Permanence: passes, since it names no coordinate count and no
mechanism. Equivalence: passes. Rests on: section 7, composing `67`'s telescope, `70`'s ownership
key and `71`'s crossing classes. ONE EXPERT on the composition; each ingredient is its author's.

**M6, the two tests are not decidable in the same sense, and one of them splits.** *Whether an
implementation can carry a system is settled by the shape of its membership question, at every
width. Whether a candidate's laws hold is settled two ways: by a condition on the shape of its
representable set, which holds at every width, or by exhaustive check at a model width assumed
uniform above it. A canon that states a law verdict says which of the two it is.* Permanence:
passes. Equivalence: passes as a constraint on how sentences are written, in the same sense `67`'s
K2 does. Rests on: section 8, `68:196-211` for the ceiling, `DROPLIST.md`'s two-mechanism entry
for the counterexample, and `73_probes/p6` for the split, over 2780 windows at four widths with a
per-operation control. ONE EXPERT.

**M7, wrapping is the test every proposed boundary must pass.** *A wrapped numeral satisfies fewer
of the properties a numeral is expected to satisfy than almost anything else the concept contains:
it has no order compatible with its arithmetic, and its reduction does not minimise distance.
Both are properties a designer reaches for when looking for the concept's edge, and each of them,
used as an edge, excludes wrapping. Any clause proposed as a boundary of the concept is checked
against wrapping before anything else.* Permanence: passes, and it is the sentence most likely to
save a later designer time, because the reflex it names is the one that produced two wrong
boundaries in one unit. Equivalence: passes; an implementation adopting either excluded clause
would refuse a type op's I3 requires. Rests on: section 4 and `73_probes/p2` for the order,
section 3 and `73_probes/p5` for the distance, and `65:258-259` for wrapping being kernel. ONE
EXPERT, and it is the cheapest sentence in this file to check and the easiest to forget.

**Deliberately not offered as sentences:** any ruling on whether the ambient operation family is a
parameter, because that is section 5's question and it is `67`'s and op's; any count of coordinates,
because `70` L2 argues the canon should not commit to one and I did not attack that; any name for
the two tests, because naming is op's and two bad names would be worse than the two descriptions;
and any magnitude, because nothing here is a bench and nothing is priced.

## 13. What I could not settle

**Whether the ambient operation family is a parameter.** Section 5 collapses Q21 into it and does
not answer it. It is the largest live question this unit leaves, it interacts with the whole law
layer's scope and with I11's named consumers, and one file's evidence should not decide it.

**Whether option (3) of section 11 is better than option (2).** Narrowed rather than settled.
Section 11 records that I attacked both of my own reasons for holding it open, and both gave way:
(3) does not contradict `63` C2, and the ingest door does not supply the discriminator I claimed
for it. What survives is a genuine fork with a much smaller gap between its arms, and I still
decline to lean, because the remaining discriminator is whether a canon sentence must be true of a
system with no ambient domain named, and I could not construct one either way.

**Two items that stood here in the first draft are closed and are not listed as open.** Whether
the third verdict should be the retraction or the distance-minimising law: closed by
`73_probes/p5`, and it is the retraction, because the stronger clause excludes both wrapping rows
of kernel item K1. Whether the membership test is stuck at the model width: narrowed by
`73_probes/p6`, and it is stuck only for the verdicts the law frame gives no closed form for. Both
were reported as open in a draft of this file and neither survived being attacked, which is the
argument for attacking rather than reporting, made on my own material.

**Whether the biconditional of section 3 survives at nonzero fraction width.** Everything here is
at F = 0. `63` C7 (`63:676-681`) records that no multiplicative structure survives a nonzero
fraction width for any policy, so a multiplicative row above says nothing about F > 0, and I did
not build the additive check there either.

**Transfer past the model width.** Every exhaustive result here is at 4 bits, except the order
enumeration, which is at 2 and 3 with a structural argument covering all widths for the two group
rows. My results inherit `68` section 2.4's proviso in full. The two I would worry about most are
p1d's biconditional, which is a universal over a small shape set, and p1's coherence counts, which
are counts. The existence claims (that a reduction exists which passes both verdicts and computes
nothing; that a collapsed declaration exists for every system) are witnessed by named constructions
and do not depend on the width.

**Whether "membership" and "hosting" are the right words.** They are descriptions, not proposals,
and `71:757-763` already records that this panel has two load-bearing uses of "crossing" and that
renaming is cheap to propose and expensive to get wrong. I follow it.

## 14. Coverage, bounded honestly

**Built and committed:** nine instruments in `73_probes/`, each with its output or refusal
transcript, all committed before the section that cites them was written. `p1` (six declarations,
honest and collapsed, exhaustive at 4 bits), `p1b` (the 952 attribution, exhaustive over 4096
triples), `p1c` (the mutation set, four reductions), `p1d` (five clauses against sixteen shapes,
with the biconditional asserted rather than eyeballed), `p2` (every total order at widths 2 and 3,
the natural order at 4, plus the element-order witness at three widths), `p3` (the contract, one
positive compile and three generated negatives with transcripts), `p4` (the citation checker),
`p5` (the third verdict, eight shapes against four clauses), `p6` (2780 windows at four widths,
with a per-operation control).

**Citations checked mechanically.** `73_probes/p4_check_my_own_citations.py` opens every
`file:line` this document cites and tests that the target **contains the text the claim depends
on** rather than merely resolving. Forty-six citations, including op's own words where I quote
them. Zero failures at the end (`73_probes/p4_output.txt`).

It failed twice on the way, and the two failures are different in kind, which is why both are
recorded. **The first was a real miscitation of mine**: `63:220-223` for "all four combinations
are inhabited", which is at 219, off by one line, and it is corrected in the document and in the
checker's table. **The second was a defect in the checker**: a quotation of op's I11 that wraps a
line fails, because markdown's blockquote marker on the continuation line lands inside the
substring. Fixed in the committed source by stripping leading markers before collapsing
whitespace, which is the same shape of fix `71_probes/p6` needed for prose wrapping, and the fix
is noted in the instrument rather than only here.

The instrument is `71`'s, adopted rather than reinvented, and one real miscitation in forty-six is
exactly the class `RULES.md:126-133` records five prior instances of.

**Verified after the commit hook ran, not before it.** The pre-commit autofix re-stages files, so
every source was re-read from `git show HEAD:` and re-run: every Python probe reproduces at
`EXIT=0` from its landed text, the landed Rust sources are byte-identical to what was compiled, and
`p3_n1.stderr`'s line numbers resolve against the landed file. That check exists because a locked
changelist in this workspace once named a grep the autofix had already falsified.

**Predictions that failed, listed rather than replaced by their corrected forms.** Five, in nine
instruments. The `n2` refusal: predicted `E0271`, got `E0053`; substance held, code did not. The
constant-zero reduction: predicted one of the two proposed verdicts would exclude it, and it passes
both, which is how the retraction clause was found rather than assumed. The Boolean lattice's meet
and join: predicted they would separate from xor under the order enumeration because they are not
group operations, and they do not, at zero compatible orders each. The third verdict: predicted the
stronger distance law would be the right clause and get the retraction for free, and it excludes
two kernel rows. And two costs I wrote into section 11's option (3) and then had to withdraw, both
recorded there rather than deleted.

**Not done:** no bench, so nothing is priced and every cost-flavoured word here means "does work"
rather than "costs this much". Nothing at nonzero fraction width, anywhere. No instrument for
`Q10`'s inclusion predicate, whose second read `71:283` records as still unrun. No re-run of any
other member's probe. No attack on `63`'s H1/H2 frame, which remains out of this unit's scope and
is still the panel's most attack-worthy claim. No correct geometric predicate for clamped
multiplication, only the control showing my naive one is not it. And I did not open `55_probes/p4`,
so section 9's first report settles an attribution and not what `55` measured.

**Nothing here settles anything.** The mode is explore. Sections 3, 5 and 7 are what I would most
want a consolidation to take and a later expert to attack: section 3 because M2 is the sentence
every other sentence here is scoped by, section 5 because it collapses two register entries into
one and a consolidator who keeps both will ship a duplicate, and section 7 because the location
reading is the only thing in this file that makes the disputed cases stop being disputes.
