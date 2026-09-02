# Q31 attacked: both routes break, the verdict survives on one leg, and the split wants a test rather than a list

Seat q31-attack. **Verdict: option two, two words.** Same verdict as both seats
and a different warrant, because the argument both of them led with does not
survive. What survives is one compiled witness, from one of them, plus a
measurement I built here. Neither seat's central refutation holds, the row's own
stated cost for option one is not a real cost, and the discriminator both seats
left as a reading exercise can be made to compile.

## 0. The gates

**Canon gate: aligned.** `question::one_word_or_two_for_is_a_number_system` is an
open row in the `question` namespace, `decider = "panel"`, no `answered` field.
`ruling::the_panel_finishes_the_canon_without_him` puts it here.
`ruling::the_option_set_is_not_a_boundary` is what licenses section 4 to propose
a shape the row does not carry. Checked against `mock/registry/*.toml`, declared
at `mockspace.toml:32`.

**Test gate: run, and it fails one check.** `cargo test --workspace`: **165
passing, 0 failing, 2 ignored, over nine result blocks.** I
read the bodies of every test in `arvo-format/src/tests/obligations.rs` and
`the_inventory.rs`. They are real: wrong constructions kept permanently, a live
positive control beside each, a separation arm per obligation, `compile_fail`
doctests with building controls. Nothing tautological and nothing to delete.

**But the suite does not isolate three of the nine clauses it appears to cover**,
and section 3 measures which. That is a coverage gap rather than a decorative
suite, one of the three is redundant, and none of the three sits in the path of
this answer, so I proceeded rather than refusing.

**Blindness, per surface.** I read both seats' files and both probe directories
in full before building anything, so **I am not a cold instance of anything and
I count as none.** I also read `proposal::membership_and_hosting_are_two_questions`,
which already reaches this verdict. What is mine is the two instruments in
sections 2 and 3 and the discriminator in section 4; the verdict is not.

---

## 1. The argument both seats led with does not survive

Both files lead with the same shape: one word is not a cost, it is a
contradiction with `ruling::the_format_spine_is_canon`. They reach it two ways
and both ways need a premise the canon does not carry.

### 1.1 Seat q31a: an ambient domain is not a candidate system

`202609021400_q31_two_words_and_the_second_is_indexed_by_this_implementation.md:64-121`
argues that under one word "the rationals" is not a number system, so the
ratified factoring "quantifies over a non-system", the ratified identity clause
"cannot state the identity of a hostable format without naming an unhostable
domain", and `dimension::ambient_domain`'s grammar becomes unwritable.

**All three need an ambient domain to be a candidate for the predicate "is a
number system". Nothing in the registry files it as one, and the ratified row
the argument cites files it as a component of one.** From
`is_an_ambient_domain_a_candidate_system.sh`, quoting the registry through the
harness's own reader, with two positive and two negative controls that all fire:

- `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
  ratified through the spine: "A format is identified **by** its ambient domain
  **and** its representable set." A pair. The domain is one member of it.
- `proposal::the_numeral_concept_is_a_dependent_sequence_of_choices`: "an ambient
  domain, a representable set over it, a reduction ... an encoding ... a
  container." Coordinate one of five.
- `proposal::a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts`:
  "A system **exposes** its ambient domain together with that domain's own law
  inventory". A system has one; it is not one.
- `dimension::ambient_domain`: "The mathematical domain a computation's values
  are taken to live in, **which the format approximates**."

So "is a number system" is a predicate over sequences. Folding residue clauses
into it constrains which **sequences** are systems and does not touch the noun
"ambient domain", which keeps its own dimension, its own grammar and its own
values. The ratified sentence names a domain, not a system, and stays stateable
word for word. **§2.3 is the clearest instance**: the predicate
`ambient domain in {the rationals, the rationals at radix ten}` is a claim about
a dimension's value, and one word leaves it writable verbatim.

**This is the file's entire section 2**, which is the file's entire refutation of
option one, and the file says so itself: "That moves the objection from a matter
of taste about redefining a term of art to a conflict inside the canon." There
is no conflict inside the canon. There is a redefinition, and the row already
prices it.

### 1.2 Seat q31b: the factoring never asks the exact product to be a format

`202609021500_q31_one_word_or_two.md:151-198` is a better argument and fails
differently. It takes two admitted `Signed<32>` operands, observes that the exact
product occupies a 63-bit slot range, builds that range as a `Slots` impl, and
watches `slots.rs:219` refuse it (arm `a1`). It concludes the ratified factoring's
first half "picks out nothing" and is "empt[ied] ... for every format arvo ships".

**The factoring does not ask the exact product to be a declared format, and
arvo's own code does not either.** The ratified sentence is an exact operation in
an **ambient domain** composed with a total adaptation **onto the representable
set** of the target. The crate implements exactly that:

```
apply.rs:305  pub const fn adapt<S: DeclaredSignature>(exact: Exact, dither: Dither) -> Slot
```

One signature parameter, the target's. The exact value arrives as an `Exact`,
built by `Exact::on_grid(slot)` or `Exact::between(slot, part)`
(`apply.rs:128`, `:142`), neither of which takes a `Slots` type at all. There is
no intermediate format in the design, which is the point of factoring arithmetic
this way. **Arm `a1` constructs a contract the design never requires and reports
its refusal as a defect in the design.**

The arm's own header says the quiet part: "This is the slot range it occupies,
and arvo's own admission obligation refuses it." It occupies that range as a
mathematical fact; nothing in arvo declares a format there.

### 1.3 The repair, because breaking it is half the job

The factoring does pose a real question and it is one contract over: not whether
the exact product is *admitted*, but whether it can be *written down*. It cannot,
and the witness is smaller than the one it replaces.

`carrier_dependence/arms/e1_the_exact_product_of_admitted_operands_is_inexpressible.rs`.
`Signed<62>` is the widest range arvo admits and arvo admits it. The exact product
of two operands at its top sits at slot `2^122` in the target's own units. `Slot`
carries an `i64` (`slots.rs:41`), so:

```
error: literal out of range for `i64`
  --> e1_the_exact_product_of_admitted_operands_is_inexpressible.rs:21:38
   |
21 | const EXACT_PRODUCT: Slot = Slot::at(5316911983139663491615228241121378304);
```

**No obligation fires and there is nothing to refuse.** The value cannot be
spelled. `e2` is the same construction at eight bits and builds, so the failure
is about the width and not about `Slot::at`. Output in
`output_exact_product_arms.txt`.

**What that repairs and what it does not.** It is a sharp instance of the canon
asserting an operation whose values no arvo type carries, which is the two-by-two
inhabited on the value axis. It is **not** a contradiction with one word either,
for §1.1's reason: the exact operation happens in the ambient domain, and one
word leaves that noun alone. I offer it as the honest form of what `a1` was
reaching for, not as a rescue of the conclusion `a1` was used for.

### 1.4 What is actually left of the case against one word

One thing, and it is seat q31b's, and it is not the leg that file led with.

**Arm `a3`**: a 63-bit two's complement grid over `BinaryRationals`. That is a
full candidate by the ratified identity clause, an ambient domain and a
representable set, and arvo refuses it at `slots.rs:219` while `a4` at one bit
narrower builds. **A thing that is a format by ratified canon and that this
implementation will not carry.** That is the row's own stated distinguisher, met,
compiled, with its control.

And **§2.2**, the closed-concept argument, which needs neither mistake: under one
word the concept's obligations contain a bound whose value is a function of
`Slot`'s carrier, so the extension of a "closed" concept moves when a machine
type moves with no canon amendment. I do not think that is a *contradiction*
either, because "closed" in `proposal::the_concept_is_closed_and_the_inventory_is_open`
is set against an open inventory and is about amendment procedure rather than
about extensional fixity. It is a real and large cost, stated exactly. Section 3
measures the mechanism under it.

**So the honest verdict is cost, not contradiction**, and the difference matters
because "one word contradicts the ratified spine" is a sentence a consolidation
would carry forward and it is false. Option one is available, expensive, and
already violated in the tree. Option two is right on cost.

---

## 2. The carrier classifier measures the experimenter, not the carrier

Seat q31b's `carrier/` probe is the strongest instrument in either file and it
does not establish what it is cited for. `202609021500_q31_one_word_or_two.md:200-217`
and its F2 and its section-8 rider all rest on it.

The probe holds two copies of `slots.rs` "differing in exactly one thing that is
not a design choice: whether a slot index is `i64` or `i128`, with the two
literals written from that type moved with it". **The parenthesis is the whole
result.** `diff carrier/src/slots_i64.rs carrier/src/slots_i128.rs` shows
`Self::WIDTH.count() <= 62` becoming `<= 126` and `< i64::MAX as i128` becoming
`< i128::MAX`. Neither edit is forced: `WIDTH.count()` is a `u32` off `Width` and
has no relationship to `Slot`'s carrier, so `<= 62` compiles unchanged under
`i128`.

**Ask of it what value would have made it fail.** Leave `62` alone and
`MUTATED_ADMITS_63` is false, and `const _: () = assert!(MUTATED_ADMITS_63)` at
`carrier/src/lib.rs` stops compiling. The probe's positive result is produced by
the same edit whose necessity it is cited to demonstrate.

`carrier_dependence.sh` splits the mutation in two, with the compiler as the
arbiter of which half is forced, over a battery of eleven candidates put through
the shipped verdict functions:

```
baseline (unmutated)         Grid63=0 Grid62=1 Grid8=1 Inverted=0 WidthZero=0 ...
slot NAIVE  (forced only)    Grid63=0 ...   identical to baseline, every column
slot FULL   (+ chosen)       Grid63=1 ...
exp  NAIVE  (forced only)    ReachPast=0 ...  identical to baseline
exp  FULL   (+ chosen)       ReachPast=1 ...
```

**The naive arm is not small.** Widening `Slot` and `SlotCount` from `i64` to
`i128` forces edits in three files, because the carrier leaks out of `slots.rs`
into `apply.rs:146`, `apply.rs:273`, `apply.rs:281` and `format.rs:372`. Every
one of them applied, and **not one verdict moves.** The exponent arm forces edits
in three files, including fourteen public const-generic positions in `lib.rs` and
`standards.rs`, and again nothing moves until the `i32::MIN`/`i32::MAX` pair in
`reach_is_representable` is hand-changed.

**So the classifier reports back the annotation it was handed.** Under it, a
clause is residue exactly when the experimenter decided its literal belonged to
the carrier, which is the reading the instrument was built to replace.

### 2.1 The replacement, built and run

Write the bound as a computation over the carrier and the classifier becomes a
measurement. Two more arms in the same probe:

```
derived @ i64  (control)     Grid63=0 ...   bit-identical to baseline
derived @ i128 (one token)   Grid63=1 ...
```

The rewrite is `pub type SlotIndex = i64;` named once, `Slot(SlotIndex)`,
`Self::WIDTH.count() <= SlotIndex::BITS - 2` and `< SlotIndex::MAX as i128`. The
control says it changed no behaviour at the shipped carrier: every one of the
eleven columns reproduces the baseline. Then **one token** moves and `Grid63`
flips.

**This is a code finding as well as a method finding.** `quantum.rs:186-194`
names this exact hazard in its own doc comment, that "widening any one of them
would break the narrower form silently while every test still passed", and says
the reach check is "computed one domain wider than it has to be, on purpose" for
that reason. The *bound* is still `i32::MIN`/`i32::MAX` and `slots.rs` is still
`62`. The crate names the hazard and then ships it, three times.

---

## 3. The suite is blind to three clauses, not one, and the three do not line up with the split

`mutate_every_admitted_clause.sh` neutralises each of the nine `ADMITTED`
clauses in turn, in both the `assert!` and the matching verdict conjunct, and
runs the whole workspace suite. Control C0 is the unmutated tree at 165 passing;
C1 is that the method can report red, which `slots_range_not_inverted` supplies.

| clause | suite | isolated by |
|---|---|---|
| `ambient.rs:160` radix positional | RED | `obligations::*` |
| `quantum.rs:318` ranges over a magnitude | RED | `obligations::*` |
| `quantum.rs:322` reach is representable | RED | `the_law_rejects_a_step_law_that_runs_off_the_exponent` |
| `slots.rs:211` range not inverted | RED | the `compile_fail` doctest at `slots.rs:166` |
| `slots.rs:215` width at least one | **GREEN** | nothing |
| `slots.rs:219` width at most 62 | **GREEN** | nothing |
| `slots.rs:228` span fits a count | **GREEN** | nothing |
| `slots.rs:232` width addresses span | RED | `the_law_rejects_a_range_that_passes_the_easy_obligations` |
| `format.rs:228` phase denotes | RED | `obligations::*` |

**Seat q31a found one of these three and framed it as diagnostic.** Its §0a and
§6.3 say "the gap ... sits exactly on the clause this question turns on", and its
F4 covers `WIDTH <= 62` alone. Two more clauses are equally invisible and **one
of them, `slots.rs:215`, is a concept clause by that same file's own table**
(`WIDTH >= 1` refuses "zero bits", "behind the refusal: nothing"). So the
suite's blindness is not about hosting at all and cannot be read as evidence for
the split. The framing goes; the finding stands and widens.

**It also confirms seat q31b's F4 by a second instrument.** Deleting
`slots.rs:228` leaves the suite green, and the test whose message names it,
`the_law_rejects_a_range_that_passes_the_easy_obligations`, is isolated by
`slots.rs:232` instead. Their exhaustive subsumption proof and my suite mutation
agree, and the two could have disagreed.

**Seat q31a's own suite figure does not describe the suite it claims to have
run.** `...202609021400...md:38-39` reports the workspace as "104, 13, 8, 5 and
4", which is `cargo test -p arvo-format` at 134. The workspace has five test
binaries; `arvo-placement`'s 22 and `arvo-strategy`'s 10 are absent, and the real
total is 165. The gate was run and the number reported is of a different tree
slice.

---

## 4. The option list, and the shape I would take

**Option one's stated cost is not a real cost.** The row says one word "makes
the concept exclude unbounded exact rationals as a matter of mathematics, which
is false". Per §1.1 the rationals are an ambient domain, a coordinate value, and
never a candidate system, so one word excludes nothing of the sort. **The row
mis-prices its own first option, both seats amplified that mis-pricing into their
leading argument, and neither checked it.** Whoever edits this row replaces that
clause with the cost that is real: one word makes the concept's extension a
function of `SlotIndex::BITS`.

Seat q31b is right at `:288-298` that an option arguing against itself in its own
text is a conclusion with a letter in front of it. That half stands and the fix
is the same edit.

**Option three is refused and I add nothing.** Two independent target sweeps
already measure the quantifier as ranging over a constant, and
`ruling::the_operating_constraints_are_intents_and_rules` forbids it ranging over
anything else. Carried unchanged from both seats.

**What I take is option two with the second predicate defined by a test rather
than by an enumeration**, which is a refinement of option two and not a rival to
it. The canon says a number system is the sequence of choices, and says the
second predicate exists and is not the concept's. What it does not do is list
which clauses fall where, because there is a discriminator and section 2.1 built
it:

> A clause belongs to the second word exactly when its bound is a function of a
> carrier's range. Written as such a function, the clause moves when the carrier
> moves and the concept clauses do not.

That is checkable rather than told, it is what makes "the concept is closed"
mean something a reader can test, and it retires the row's stated price for
option two, "a reader told which is which at every use". Nobody is told. The
source says which by its own shape.

**The design act it implies**, which is not mine to make and is one paste: write
the three residue bounds as computations over their carriers. `slots.rs:219` as
`SlotIndex::BITS - 2`, `slots.rs:228` as `SlotIndex::MAX` (or delete it, per §3),
`quantum.rs:203` as the `Exponent` carrier's bounds. Behaviour-preserving,
measured, and it removes the silent-staleness hazard the crate documents against
itself.

**A fourth option I considered and dropped.** That the fork is malformed the way
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
found the container premise malformed, both branches false. It is not: seat
q31b's `§4.4` closes it correctly, silence about the second word is one word by
default, so the canon needs the word in order to say the word is not its
business. Recorded because the next reader should not have to re-derive that it
fails.

---

## 5. What the two seats' agreement is worth, plainly

**On the verdict: nothing.** Both disclaim being cold instances and both name
`proposal::membership_and_hosting_are_two_questions` (`standing = "one_expert"`)
as read before writing. I read both files plus that row before building anything,
so I am not one either. **The count of independent arrivals at "two words" is
one, it is that row, and it has not moved.** Three files agreeing after reading
it is the shared-drift shape, not corroboration, and the standing field should
stay where it is until somebody derives it who has not read the row.

**On the route: less than nothing, because the two routes were different and both
are broken**, §1.1 and §1.2, in ways that do not overlap. Their agreement on the
verdict concealed that neither route was checked against the other's.

**On the descriptive claim, that the crate welds both kinds into one const: it is
established and over-established.** Four instruments now, three of them
independent of each other: seat q31a by exhibiting refused candidates, seat q31b
by compiling `a3` against `a5`, the two earlier design-side files by reading the
carrier declarations, and my carrier arms. It was never the contested half.

**On the split's content:** seat q31a classified seven of nine by reading, seat
q31b five of nine. My §2.1 discriminator measures the residue side on both
carriers. **For the six concept clauses I did not construct a derived form and I
do not claim none exists**, so that half is still read, by three of us now, and
it is where the next instrument goes.

---

## 6. Outside the question, per the standing instruction

**6.1 `format.rs:369` is a tenth carrier-derived site and no classification
covers it.** `if slot < i64::MIN as i128 || slot > i64::MAX as i128` bounds a slot
into the carrier's range inside a conversion function rather than inside an
`ADMITTED` block. Both seats enumerate nine assertions and this is not one of
them, so a split derived from the nine is incomplete about where residue lives.

**6.2 Seat q31b attributes to op a phrase he did not say.**
`202609021500_q31_one_word_or_two.md:353-357` argues Q1 does not license folding
because `ruling::validate_means_all_three_readings` states the admissibility
reading as "the typestate refuses declarations **it cannot serve**", and calls
that "op's own framing of the reading". That phrase is in the row's `note`, which
is panel-authored annotation. Op's `quote` on that row names the three readings in four words, "Usage,
Admissibility, Self-validation, All that makes sense", and says nothing further
about any of them; the rest of the quote is about the call not being a strict
one. **The conclusion survives**,
because it follows from the row's `says`, which is about what validation covers
and not about vocabulary. The attribution does not, and it is the sentence a later
reader would quote.

**6.3 `topic::the_number_system`'s `what` writes option one into the taxonomy.**
Both seats found this and both are right. Carried unchanged.

**6.4 A tracked generated document carries a timestamp and nothing else volatile.**
Running `cargo mock` in a clean worktree dirties `docs/STRATEGY.md` with a
`Generated at:` line and no content change, so anybody who runs the tool gets a
diff that is not one. Small, and it makes `git status` lie about a public surface.

---

## 7. Predicates

**F1. No registry row files an ambient domain as a candidate for the predicate
"is a number system"; every row that names one names it as a component.**
*Holds for: the registry at `2a274940`; the `proposal` and `dimension`
namespaces; the eight `proposal` rows whose `says` names an ambient domain;
established by reading returned text with two positive and two negative controls
on the query, so it is a reading of a complete enumeration rather than a
measurement; threads = 1; toolchain any, by construction, the argument reads no
compiled artifact; target features any, same.*

**F2. Widening the slot index carrier from `i64` to `i128`, applying every edit
the compiler forces across all three files that touch it, moves no verdict in an
eleven-candidate battery.**
*Holds for: the tree at `2a274940`; slot index carrier in {i64, i128}; the eleven
candidates named in `carrier_dependence/src/battery.rs`; declared width in {0, 2,
8, 13, 62, 63}; toolchain = rustc 1.98.0-nightly (57d06900f); target =
aarch64-apple-darwin; build profile = dev; threads = 1; target features any.*

**F3. The same widening moves `Grid63` once `<= 62` is hand-changed to `<= 126`,
and that edit compiles unchanged without it.**
*Same region as F2.*

**F4. Written as `SlotIndex::BITS - 2`, the bound reproduces all eleven shipped
verdicts at `i64` and moves `Grid63` under a one-token carrier change.**
*Same region as F2.*

**F5. The exponent carrier behaves identically: `i32` to `i64` across three files
moves nothing until the `i32::MIN`/`i32::MAX` pair is hand-changed.**
*Holds for: the tree at `2a274940`; exponent carrier in {i32, i64}; the four
quantum candidates in the battery; `SLOPE` in {0, 1, 2147483647}; `MAGNITUDES` in
{0, 1, 4}; toolchain = rustc 1.98.0-nightly (57d06900f); target =
aarch64-apple-darwin; build profile = dev; threads = 1; target features any.*

**F6. Three of the nine `ADMITTED` clauses can be neutralised with the whole
workspace suite staying green: `slots.rs:215`, `:219`, `:228`.**
*Holds for: the tree at `2a274940`; `cargo test --workspace`, 165 passing over nine result blocks; the nine mutations named in
`mutate_every_admitted_clause.sh`, each applied to both the assert and the verdict
conjunct; toolchain = rustc 1.98.0-nightly (57d06900f); build profile = dev,
debug-assertions on; threads = 1; target features any.*

**F7. The exact product of two operands at the top of `Signed<62>` cannot be
expressed as a `Slot`.**
*Holds for: the tree at `2a274940`; declared width 62 for the witness and 8 for
the control; operation = multiplication; arity = 2; slot index carrier = i64;
toolchain = rustc 1.98.0-nightly (57d06900f); target = aarch64-apple-darwin;
threads any, by construction, the failure is a literal the parser refuses.*

---

## 8. What I could not break, and what I did not settle

- **Seat q31b's arm `a3` and its control `a4`.** I tried to find a reading under
  which a 63-bit grid over `BinaryRationals` is not a candidate by the ratified
  identity clause, and there is none: the clause names an ambient domain and a
  representable set, and `a3` supplies both. It is the one leg of the case
  against one word that I could not get through, and the verdict rests on it.
- **Whether the six concept clauses have a carrier-derived form.** Not
  constructed, so not claimed. Three of us have now read them the same way, which
  is three readings and not a measurement.
- **The content of the second predicate.** Not this row's, per both seats, and I
  agree.
- **`.data/op-responses/`.** Outside this worktree, so §6.2 rests on the `quote`
  field in `mock/registry/ruling.toml` and on nothing else. If that field is a
  bad transcription, §6.2 falls and seat q31b's attribution may be right.
- **Whether `slots.rs:228` should be deleted or derived.** §3 says the suite does
  not see it and seat q31b proves it subsumed. Deleting it and deriving it are
  both defensible and I did not pick, because the crate's own comment records a
  measured defect it was added for and the subsumption argument depends on `:219`
  staying where it is.

## 9. What I carried forward unchanged, and from whom

Five, and the count is the point.

1. **The verdict, two words**, from `proposal::membership_and_hosting_are_two_questions`
   via both seats. Not independently reached here.
2. **Option three is refused**, from both seats' target sweeps, replicated by
   neither of them from the other and by me from nobody.
3. **`topic::the_number_system`'s `what` needs editing**, from both seats.
4. **The row's option-one text argues against itself**, from seat q31b `:291-298`.
5. **`slots.rs:228` is subsumed**, from seat q31b's exhaustive proof, confirmed
   here by a second instrument in §3.

And one carried **against** both: the descriptive claim that the crate welds two
kinds of refusal into one `const`. Neither seat is wrong about it and neither
needed the arguments I broke to establish it.
