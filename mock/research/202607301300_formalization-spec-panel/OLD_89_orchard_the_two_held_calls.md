# 89. The two held calls, made rulable: they are one call, and one of the two readings is already ratified in a section nobody has read beside the other

Dominic Orchard, file 89. I wrote file 17 (whether the design's axes are grades) and file 37
(which relation), the second of which dissolved the three-relation fork into a single finest-view
lattice and is ratified at `39b`. I do not treat file 37 as standing here. This file's sharpest
finding is against it: **file 37's own ratified view table was measured under one of the two
readings op has been declining to choose between, and nobody, myself included, noticed that a
measurement had made the choice.** That is stated at the top rather than buried, because it
changes what op is being asked.

**What I read.** `78_consolidation_eight.md` in full, the standing base. Every deliverable since,
in order and in full: `79`, `79b`, `80`, `81`, `82`, `82b`, `83`, `84`, `85`, `86`, `86b`, `87`,
`88`. Behind the consolidation, with licence since each is a derivation one of the two calls sits
directly on and the consolidation compresses it past the clause I need: `43_smith_division.md` in
full plus `43_probes/probe_5_the_roundtrip_law_and_its_view.rs` as source; `44b` in full (the
hold itself); my own `37` sections 1, 2, 5 and 7 plus `37_probes/probe_1_the_ladder_is_a_view_
lattice.rs` as source, read as a claim list rather than trusted; `49:464-546` and `58` section 1.14
(the grade's shape, which no consolidation since restates); `50:294-327` (the generator table and
the IEEE convergence); `40:265-300` (the passage probe 5 cites); `70:140-180` (the presets' own
method). One `ls` of the panel directory, current through `88_probes`.

The shipped tree I touched for exactly three things and no more: the standing canon-gate greps,
the bodies of the tests in the surface my subject touches, and one factual check on the
`Capacity` trait behind a claim I repeat. **No conclusion below rests on a shipped-source
citation, and every one survives deleting it.**

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty.
Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary by parsing every
`test result:` line across 149 binaries, **666 passed, 0 failed, 9 ignored**, matching files 81
through 84. I read the bodies of the division tests rather than their names:
`arvo/tests/fixed_point_div.rs` (thirteen real assertions with deliberate container-overflow
setups, plus one correctly formed catalogue red at line 111 whose assertion states the intended
widened result rather than the current wrong one) and `arvo/tests/strategy_wrapping_div_zero.rs`
(read by file 84, six real assertions, confirmed). The one disqualifying test on record stands
exactly as `78:874-876` carries it, and I confirmed its tautology rather than repeating the claim:
`arvo-tensor/src/capacity.rs:49` declares `const CAP: Cap = cap(N)`, so
`arvo-tensor/tests/capacity.rs:14-18`'s three lines reduce after monomorphisation to
`cap(3) == cap(3)`, `cap(1) == cap(1)`, `cap(47) == cap(47)`. Deletion, not improvement, and
outside the panel's scope to touch. Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`, resolved from `rust-toolchain.toml`, confirmed inside the tree; outside
it the same command gives stable `1.94.0`, and one result below (the FPSR read) depends on which.

**What is compiled, what is measured, what is reasoned.** Sections 1 through 5 and section 7 are
compiled and trace to `89_probes/` (five probes plus one expected-fail, outcomes and exact
commands in `89_probes/OUTCOMES.md`), written and run fresh this session. Every instruction count
is read out of an object file with `objdump -d`; there is no timing claim anywhere and the bench
harness was not run, because its orchestrator overwrites every committed CSV in `mock/benches`
(`81:38-44`). Sections 6 and 8 are reasoned from ratified text and compiled pieces and say so per
claim. **Everything is offered as material for a ruling, not as a ruling.** Both questions are
op's, both have been declined deliberately, and my job is to make them cheap to decide rather than
to decide them.

**The deletion test, applied** (`78:372-380`). No row below is justified from shipped source. The
two `tree-fact` citations that appear (the `Capacity` trait's shipped shape above, and the current
divide-by-zero behaviour at `arvo/tests/fixed_point_div.rs:69-72`) are facts of existence offered
as evidence about why the redesign is happening, and every design conclusion here survives their
deletion.

---

## 0. The verdict, stated first

**The two held calls are one call, and the review has been carrying both of its answers at once
without either file knowing.**

The event-counting question has two readings, and the corpus contains one file measured under each:

- **Reading A, per quantiser application.** An event is charged whenever a quantiser sits at a
  site in the term, whether or not it moved anything. The count is a function of the term's
  monomorphised type alone. This is what `43_probes/probe_5_the_roundtrip_law_and_its_view.rs:15`
  states it assumes, and what file 43's division work is built on.
- **Reading B, per value moved.** An event is charged only where the delivered value differs from
  the exact one. The count is a function of the data. This is what
  `37_probes/probe_1_the_ladder_is_a_view_lattice.rs:130-165` actually computes: its `resolve` is
  reached only when the exact partial sum leaves the value set, and only there does it set `e: 1`.

**File 37's finest-view table is therefore a reading-B measurement, and it is ratified.** It is
carried at `37:171-179`, into `40`, and through every consolidation since. Compiled: my probe 1
reproduces that table row for row under reading B, and under reading A **two of its five rows
change and the lattice becomes a chain** (`89_probes/probe_1`, exhaustive over every input tuple of
every composition, no sampling).

That is the whole reframing. The question is not open in the direction the record says. One reading
is embedded in a ratified measurement, the other in the division work that is held, and choosing
against either is a re-derivation rather than a preference.

**The separating condition, which is the thing worth handing over.** The two readings agree exactly
when the quantiser never acts as the identity on the inputs a law quantifies over, and differ
exactly on the exact cases inside an inexact operation. Three consequences follow, each compiled:

1. On **folds**, reading A makes the event component of every associativity law hold, for every
   composition, by construction: every grouping of an n-element fold has exactly n-1 sites, so a
   site count is grouping-invariant whatever the data does. The event axis of file 37's lattice
   carries no information at all under reading A.
2. On **division's round-trip law**, `div(mul_full(a, b), b) = a`, the readings differ in the other
   direction. Under reading A the event counts are one against zero on every defined pair and the
   law sits at the weak-equation corner; under reading B they are zero against zero and it sits at
   `(Ignore, Exact)`, which is `Precise`'s own point. File 43 named this and left it as a
   one-sentence fork (`43:324-328`); it is compiled here.
3. On **binding time**, reading A's count is a type and reading B's cannot be. The design's own
   published-grade contract is a type mismatch on `Folded<N>` (`37:441-444`, `49:464-475`). Reading
   A inhabits it with a literal; reading B is refused with `E0435` (`89_probes/probe_3b`, expected
   fail, with the reading-A positive control compiling in the same file).

**And the price is measured, not argued.** On a 64-element fixed-point fold, reading A's entire
cost is one instruction outside the loop, `mov w1, #0x40`. Reading B doubles the loop body (2.5 to
5.0 instructions per element) and costs the same at the presence level as at the exact level, so
the intuition that presence is the cheap one is false on this shape. On the float side, where the
ratified preset table sends `Hot` and `Warm` through the hardware door, reading B costs seven to
nine times the instruction count and destroys the unrolling entirely, by either of its two routes.

**On the division call.** File 43's finding survives every coordinate change since `44b` intact,
and I re-derived its accumulator table independently rather than carrying it (six of seven cells
reproduce exactly, one differs by a bit, the growth law reproduces exactly). What has changed is
that the machinery its open ends were waiting for has landed, and it lands in division's favour
three times: the far-point rule now answers what `x/0` resolves to, and answers it by a limit
argument that yields IEEE clause 7's own two-way split as a theorem rather than a citation
(`89_probes/probe_5`); file 84's three-kind failure sort gives division's failure a home and a
derived preset row; and file 84's own independent finding that lifting an operand to type position
makes an operation value-keyed and law-eligible is the same move file 43's `div_exact::<C>` made
forty-one files earlier. One live fork remains and I state both sides with their costs in section 6.

**The connection, which is why one dispatch can carry both.** Division is the only operation in the
design whose grade is *decided* by the choice, because division is the only operation whose
quantiser is applied unconditionally and moves the value on only some inputs. Addition's quantiser
is applied unconditionally and *fires* conditionally, so the two readings differ there too but the
fold's grouping-invariance masks it. Multiplication routes through `mul_full` and has no quantiser
in the interior at all. So `86b`'s own phrase, "division's grading axis", is exact: settling the
counting question settles division's grade, and settling division's grade settles the counting
question, and neither can be settled without the other.

---

## 1. Four factual claims checked before reasoning from them, and each needs correcting

Per the standing discipline, and per the dispatch's own instruction to check cheap claims including
the consolidation's.

**The dispatch's paraphrase of the counting question is a different question from the one on
record.** It reads "whether an event is counted once per operation that produces it, or once per
value that carries it through." "Carries it through" describes multiplicity under sharing: whether
a value used twice contributes its grade twice. That is a real question and I answer it in section
4.4, but it is not the one op has declined four times. The recorded question is
`43_probes/probe_5:15-16`, verbatim: "an event is counted per quantiser APPLICATION, not per value
actually moved." I use the recorded wording throughout and name the paraphrase's question
separately, because collapsing them would have produced a well-answered wrong question.

**The dispatch's paraphrase of file 43's division proposal drops half of it.** It reads "shipping a
floor-division and a remainder as two exact-partial operations bound by a compiled Euclidean law,
rather than one rounding division." File 43 does not propose that. `43:264-272`: "The pair ships as
**two single-valued operations, `div_floor` and `rem`** ... General `div` remains the atomic
`quantize(exact quotient)` surface, implemented FROM the pair." The proposal is three operations,
not two, and the rounding division is kept. This matters for the ruling, because the version in the
paraphrase would remove a capability from every consumer wanting a rounded quotient, which
`arvo-toolbox-not-policer.md:33` forbids by name and which file 84 already rejected one section
over when the same shape was offered for `quantize` (`84:167-171`).

**Probe 5's citation for reading A does not support it.** `43_probes/probe_5:16-19` grounds the
per-application reading in `40:279-287`, glossing it as "the operation marker's IS_EXACT, a
type-level constant, is what trivialises the grade monoid, so grade content cannot depend on which
values a run happened to see." The cited passage, read fresh, says something strictly weaker
(`40:282-283`): "`IS_EXACT` alone does not trivialise an operation's grade monoid; `IS_EXACT` and
`Total<Op>` together do." That is a statement about when the monoid is **trivial**. It says nothing
about whether a **nontrivial** monoid's content is value-dependent, and the inference from one to
the other is the whole of reading A. The citation is real, traceable, and authoritative for nothing
it was asked to support, which is the exact shape file 69 traced and `78:341-383` adopted a
grounding split to catch. **The design has never committed to reading A.** It has also never
committed to reading B in prose; what it has done is measure under it (section 2).

**File 50's stronger availability sentence does not survive, and its weaker one does.**
`50:319`: "the standard's carrier is not available to us in any case." Its evidence is a grep
of `core` and `std` for `fetestexcept`, `feclearexcept`, `fegetround`, `fesetround`, and an
observation that `core::arch::aarch64` has no FPCR access. I re-ran both greps fresh inside the
tree and both reproduce, empty. But a grep of `core` cannot answer an availability question, and
the register is reachable: `core::arch::asm!("mrs {0}, fpsr")` compiles with no feature gate on the
pin and returns `0x10` after one inexact addition (`89_probes/probe_4`, CLAIM A). Workspace policy
licenses exactly this route (`arvo-always-optimal-internals.md`, inline asm when the intrinsic
chain misses). **File 50's other and load-bearing argument is untouched**: the flag word is
per-thread and sticky, so under a pluggable executor a per-thread accumulator is nondeterministic
on unchanged data. That argument never needed the availability claim and it is what actually
disqualifies IEEE's carrier. I correct the sentence rather than the conclusion.

*Grounded on: settled shapes (`43:264-272`, `40:282-283`, `50:319-327`), compiled
(`89_probes/probe_4` CLAIM A, the fresh greps in `OUTCOMES.md`), tree-fact (none load-bearing),
reasoned (the inference gap in probe 5's citation, mine).*

---

## 2. The two readings, stated so a reader arriving cold can rule on them

An operation's pipeline may contain a **quantiser site**: a place where an exact intermediate is
rounded onto a target grid and then classified against a range (`78:206-210`, round first, classify
second). Whether a site exists is a fact about the operation's type. Whether the site's rounding
*changes* the value is a fact about the data reaching it.

The design's grade is a free commutative monoid over refusal causes and quantisation events
(`37:507-511`), and the published carrier is that monoid at the presence level, which over five
generators is a five-bit word joined by bitwise or (`50:294-307`, `58` section 1.14). Multiplicities
live one level finer, at what file 37's view lattice calls the `Exact` detail level, and they exist
so that a law can say whether two groupings produced the same events and so that a consumer
propagating an error bound has something to multiply (`37:361-366`).

**Reading A.** The event multiset of a term is the multiset of its quantiser sites. Every site
contributes one event of its kind, unconditionally.

**Reading B.** The event multiset of a term is the multiset of its sites at which the delivered
value differs from the exact one. A site that rounds nothing contributes nothing.

Three properties follow immediately and none of them is a preference.

**Reading A is a function of the type; reading B is a function of the value.** So they key at
different layers, and the layer-keying rule (`78:137-150`, a fact is keyed on the coarsest layer
whose identity its truth depends on) does not choose between them. It tells you what each is: A is
an operation-layer fact, B is a value-layer fact. The rule's real bite here is elsewhere and I take
it in section 5.

**Reading A over-approximates reading B, pointwise, always.** Every moved value sits at a site.
So A is a sound upper bound on B, and the direction is the one the design takes everywhere on
lattice containment (`40:308-312`: overstating compiles and is merely pessimistic). That is a real
argument for A and I do not want it lost among A's costs.

**Neither is IEEE's `inexact`, but B is much closer.** IEEE 754-2019 signals inexact when the
rounded result of an operation is not exact, which is reading B at the presence level. I did not
read the standard's own text this session and mark this as a secondary read, owed to the same
primary-source bundle `78:934-941` already carries; the derivation below does not rest on it, and
where it bears I say so.

*Grounded on: ratified (`78:206-210`, `78:137-150`, `39b` the finest-view mechanism), settled
shapes (`37:507-511`, `50:294-307`, `58` section 1.14, `40:308-312`), external (IEEE 754-2019
clause 7.6, secondary, primary read owed), reasoned (the three properties, mine).*

---

## 3. The separating condition, compiled

`86b:8-10` requires that a claim about a distinction be checked where the distinction is
nonvacuous, and that a model state what it separates. Each probe below carries that statement in
its own header. Here is the condition itself, because it converts a taste question into a scoped
one, which is what the dispatch asked for.

**The two readings give the same answer on a term exactly when every quantiser site in it moves the
value on every input the claim quantifies over, or none does.** They differ exactly on the mixed
case. Three regions of the design fall out.

**Region 1, where they agree because nothing ever moves: the exact family.** `mul_full` has no
quantiser site at all, so both readings charge zero (`89_probes/probe_3`, `_A_MUL_64` asserts zero
in const position). Division by a fixed nonzero representable constant has its site, but the
numeral-level map is the identity on indices (`43:168-175`), so nothing moves. `IS_EXACT` together
with `Total<Op>` (`40:282-283`) is the type-level predicate for this region, and inside it the
question is vacuous.

**Region 2, where they agree because the disagreement is invisible: the fold family's laws.** Every
grouping of an n-element fold has exactly n-1 sites, so under reading A the event counts of two
groupings agree by construction, for every input, at every resolution. Asserted over every grouping
of every input tuple of every composition (`89_probes/probe_1`, CLAIM B: the site count is 3 in all
of them). The consequence is not that they agree; it is that **under reading A the event component
of a fold law is uninformative**, and file 37's whole event axis measures nothing there.

**Region 3, where they differ and the design already lives: everything with a conditional
resolution or a conditionally exact quotient.** A wrapping or saturating fold whose reduction fires
on some inputs and not others. A general division whose quotient is on-grid on some inputs and not
others. This is where file 37 measured and where file 43 assumed, and it is the region both of
op's held calls sit in.

The compiled separation, side by side (`89_probes/probe_1`, exhaustive):

| composition | finest view, reading B | finest view, reading A |
|---|---|---|
| `Hot`, unsigned wrapping | (Exact, Exact) | (Exact, Exact) |
| `Hot`, signed wrapping | **(Exact, Ignore)** | **(Exact, Exact)** |
| `Warm` / `Cold`, saturating | no law at any view | no law at any view |
| `Precise`, refusing | (Ignore, Exact) | (Ignore, Exact) |
| refuse one end, reduce the other | **(Ignore, Ignore)** | **(Ignore, Exact)** |

The left column is file 37's ratified table, `37:171-179`, reproduced row for row by an
independently written model. That reproduction is the calibration: without it nothing else in this
file would be evidence.

**And the structural consequence, which is the cost of reading A that nobody has priced.** Under
reading B the measured views contain one incomparable pair and under reading A they contain none:
the set is a chain (`89_probes/probe_1`, CLAIM C, computed on the product order rather than eyeballed).
The design's ratified reason for computing a finest view instead of naming three relations is that
the lattice is **not** a chain, and specifically that `Hot` on a signed numeral and `Precise` below
interior safety are incomparable and both are shipped presets (`37:62-69`, `37:181-185`, carried
into `40` and `78`). Reading A collapses `Hot` signed to the top and removes that pair. The
mechanism survives, because the cause axis still has three levels and `Precise` still sits below
`Hot` on it, but **its strongest single piece of evidence is a reading-B measurement**, and op
should know that before ruling.

*Grounded on: ratified (`39b`, `37:171-179` via `40`), settled shapes (`40:282-283`, `43:168-175`,
`37:62-69`, `37:181-185`), compiled (`89_probes/probe_1` all four claims, `probe_3`'s const
assertions), reasoned (the three-region sort, mine).*

---

## 4. What each reading claims, makes provable, and costs

Stated symmetrically, each consequence derived rather than asserted, so that the two columns can be
read against each other rather than against my preference.

### 4.1 What each makes provable

**Reading A proves things about programs.** "This fold applies at most 64 quantisers" is a theorem
about the term, true of every run, checkable from the monomorphised type with no value in scope.
Compiled in const position at every operation in the design's vocabulary
(`89_probes/probe_3`, `_A_ADD_64`, `_A_MUL_64`, `_A_DIV_64`, `_A_ROUNDTRIP`). A consumer
propagating a worst-case error bound gets a sound, pessimistic multiplier at compile time and pays
nothing for it.

**Reading B proves things about runs.** "This particular result is exact" is a statement a consumer
can act on, and acting on it is not hypothetical: it is the precondition for treating a fixed-point
value as an exact rational, which is what the design's own exact family exists to exploit. A
worst-case bound built from B is not available at compile time at all, because B is not a compile-time
quantity; what B gives is a *realised* count, tighter than A's bound by however much the data
allowed. On probe 4's input the two are 58 against 64.

**Neither proves the other's statement, and the design currently claims both.** `78:311-316`'s
`AbsorbingFarPoint` bound is an A-shaped claim (a type-level refusal at a call site). The IEEE
generator identification at `50:294-307` is a B-shaped claim (the generators are named `inexact`,
`underflow`, `overflow`, `invalid`, `divideByZero`, and IEEE's raising conditions for the first
three are all "the result differs from" conditions). **If op picks reading A, the design's
generator names stop denoting what the standard denotes by them**, and the standards test at `13c`
gains an item: a `conv-ieee754` composition must be able to express B, whatever the default is. If
op picks reading B, the names are honest and the `AbsorbingFarPoint`-shaped bounds are unaffected,
because they are bounds on the numeral rather than counts on the term.

### 4.2 What each costs a consumer who reads the grade to act on it

Measured, from the object file, never from a timer (`89_probes/OUTCOMES.md` carries the commands).

64-element `i64` fixed-point fold, `FRAC = 4`:

| fold | emitted instructions | steady state per element | unrolled |
|---|---:|---:|---|
| no grade published | 160 | 2.5 | fully, 64x |
| reading A published | 161 | 2.5 | fully, 64x |
| reading B, counting | 32 | 5.0 | 4x |
| reading B, presence only | 34 | 5.0 | 4x |

64-element `f64` fold, the shape `Hot` and `Warm` float take through the hardware door
(`78:435-441`):

| fold | emitted instructions | steady state per element | unrolled |
|---|---:|---:|---|
| no grade published | 98 | 1.5 | fully, 64x |
| reading A published | 100 | 1.5 | fully, 64x |
| reading B via the FPSR flag | 16 | 11 | not at all |
| reading B via 2Sum recomputation | 18 | 13 | not at all |

Four things these numbers say that the argument did not.

**Reading A's entire cost is one instruction, and it is outside the loop.** `mov w1, #0x40` at
`0x27c` of `fold_grade_reading_a`. That is not "cheap"; it is the count already being a literal.

**On the fixed-point side reading B costs the test, not the accumulation.** Counting and presence
cost the same 5.0 instructions per element, because both need the same `and` against the discarded
bits and differ only in whether the following instruction is `cinc` or `orr`. The intuition that
the presence level is the cheap one, which the design's published-carrier choice quietly leans on,
is false on this shape.

**On the float side reading B is expensive by either route, and the expense is structural.** The
flag route emits `mrs x9, FPSR` and `msr FPSR, x9` inside the loop and the loop stops unrolling
entirely; the recompute route emits five extra floating-point operations plus a compare and also
stops unrolling. Seven to nine times the instruction count. And the recompute route is precisely
what the ratified `Warm` float row forbids by name: `78:448-455` grounds `Warm`'s minimum stored
width on IEEE delivering correctly-rounded results "for free by the hardware, invisibly", and on
"doubling `Warm`'s float storage would add bookkeeping the hardware never asks for and the 'no
framework on top of it' intuition explicitly forbids." **So reading B, published by default under
the ratified float table, is either a per-operation FPU serialisation or the bookkeeping that table
rejects.** That is the sharpest single cost on either side of this call and it is derived from two
separately ratified pieces rather than asserted.

**One honest limit on all of it.** In every fold above the accumulator is a serial dependency, so
reading B's added work sits off the critical path and an instruction count overstates its latency
cost. A throughput claim needs the bench harness, which I did not run. Named as owed rather than
guessed.

### 4.3 What each does to the laws, now that the lattice is settled and its join is built

The join is what makes the finest view unique (`37:136-143`), and it is closed under both readings,
because both are monoid homomorphisms out of the same grade: reading A forgets which sites fired,
reading B forgets which sites existed. So neither reading breaks the mechanism. What changes is the
verdicts, and section 3's table is the whole of it for the fold family.

For division the change runs the other way and is compiled at `89_probes/probe_2`. The round-trip
law `div(mul_full(a, b), b) = a`, exhaustive over the model's 256 pairs:

| | reading A | reading B |
|---|---|---|
| values agree wherever both defined | yes, 240/240 | yes, 240/240 |
| definedness | disagrees at the 16 zero-divisor pairs | same |
| event counts | 1 against 0 on every defined pair | 0 against 0 on every defined pair |
| finest view | (Ignore, Ignore), the weak equation | (Ignore, Exact), `Precise`'s own point |

And the cell that makes it concrete: `x / 1 == x`, exact on every input, charges sixteen events
under reading A over the model's sixteen values and zero under reading B.

The model separates in both directions, which `86b` requires it to state: over the whole
general-division matrix the divider moves the value on 144 of 240 defined pairs, so reading B is
not vacuously zero here and the round-trip's zero is a property of that law rather than of the
model.

### 4.4 The other question the dispatch's paraphrase raises, answered separately

"Once per value that carries it through" describes multiplicity under sharing: in `let y = q(x) in
y + y`, does `y`'s event count once or twice? The design answers this already and the answer is
independent of A against B. `49:490-497` builds the combination as a **join** over a sealed grade
lattice, and a join is idempotent, so a shared operand contributes once. That is consistent with
the published carrier being at the presence level, where a multiset and a set coincide.

It is worth one sentence in the spec anyway, because the underlying monoid is a multiset and the
`Exact` view reads its multiplicities, and at that view idempotent combination and additive
combination genuinely differ. My reading, offered rather than ruled: **the underlying grade is a
multiset; the published grade is its presence view; multiplicities appear only inside a law's
verdict, never in a shipped carrier.** Under that statement the sharing question never reaches a
consumer, and the design is coherent as it stands.

There is one derived consequence of it that op should see, because it is not obvious. **A
value-carried grade cannot implement per-application counting at the `Exact` view at all.** Take
two terms: `let y = q(x) in y + y`, and `let y1 = q(x); let y2 = q(x) in y1 + y2`. Reading A
charges one site in the first and two in the second. But `y`, `y1` and `y2` carry identical values
and identical grades, so any function of the operands' grades and the operation gives the same
answer for both additions. Reading A at the `Exact` view is therefore not computable from what the
value carries; it is computable only from the term, at compile time. Reading B is computable from
either. This does not decide the call, and it does not bite while multiplicities stay inside law
verdicts as I propose above, but it does say that the two readings live at different places in the
design rather than being two settings of one dial.

*Grounded on: ratified (`78:435-441` and `78:448-455` the float preset row and its own grounding,
`78:311-316`), settled shapes (`37:136-143`, `49:490-497`, `50:294-307`, `40:308-312`), compiled and
measured (`89_probes/probe_1`, `probe_2`, `probe_3`, `probe_3b`, `probe_4`, all fresh, counts and
disassembly in `OUTCOMES.md`), external (IEEE 754-2019 clause 7, secondary, primary read owed),
reasoned (the two-term argument in 4.4 and the multiset-presence statement, mine).*

---

## 5. The two readings against the four design rules

Because the rules are what the review decides with, and because three of the four have something to
say here and one has nothing, which is itself worth recording.

**The spine rule** (a quantity that is computed and then has to appear in a type is a type; one that
only ever has to be read is a const). This fires, and it fires cleanly. Reading A's count has to
appear in a type, because the published grade is `Folded<N>` and the caller's contract is a type
mismatch on it (`37:441-444`). It can: it is a function of types. Reading B's count only ever has to
be read, so under the spine rule it is a const or a runtime value and never a type, and
`89_probes/probe_3b` is what happens when it is asked to be one (`E0435`). **The spine rule does not
choose between them. It says that under reading B the published grade stops being a type
parameter**, and that is a change to a shape the review has built twice (file 47's projection, file
48's join).

**The carrier-at-birth rule.** Nothing new fires. `Grade` is already sealed (`49:497-500`), and
neither reading mints a vocabulary.

**The layer-keying rule** (a fact is keyed on the coarsest layer whose identity its truth depends
on). This fires, and its answer is the sharpest thing the rules give. "How many quantisers does this
term apply" depends on the operation and the numerals and nothing finer: it is an
operation-layer fact, and reading A is its honest carrier. "Did this computation round" depends on
the values: it is a value-layer fact, and reading B is its honest carrier. **They are two different
facts at two different layers, and the rule's own discipline is that a design carrying both keys
each where it belongs rather than picking one and calling the other its approximation.** Under that
reading the call op has been declining is malformed as posed, and the well-formed version is
section 8's proposal.

**The pricing pillar** (compile time is a bucket to pour into; nothing defers to runtime what it
could settle at compile or const time). This is the one that looks decisive for reading A and is
not, and I want to be careful because the rule's own guard clause warns against exactly the
misreading available here. The rule forbids deferring to runtime *what could be settled at compile
time*. Reading B's count is not settleable at compile time, because it depends on values the
compiler does not have. So it is not a deferral; it is a different fact, and the rule is silent on
whether the design should carry it. Where the rule does bite is on the standing test at `78:161-166`:
is anything the design does at runtime available at compile time and rejected for compile cost? For
reading A the answer is no rejection, it is available and free. For reading B the answer is that no
compile-time alternative exists, which the rule names as the one real constraint rather than the
violation. **So the pricing pillar licenses reading A and does not forbid reading B**, and reading
it as forbidding B would be the guard clause's own named misreading arriving in a new place.

**The separation requirement** (`86b`, not a fifth rule, a check on models). Applied to this
review's own record, it is what found section 0's finding: file 37's probe and file 43's probe both
looked like measurements of the same object, and neither stated what it separated. Had either
carried the sentence `86b` now requires, the collision would have been visible in July.

*Grounded on: ratified (`78:120-129` the spine rule, `78:131-135`, `78:137-150`, `78:152-166` and its
guard clause at `78:168-180`, `86b:8-13`), settled shapes (`37:441-444`, `49:490-507`), compiled
(`89_probes/probe_3b`), reasoned (the four applications, mine).*

---

## 6. The first call: division, re-derived against what has landed since `44b`

Op held this "for a later stretch" at `44b:21-30`, said the finding does not expire, and noted the
Euclidean law is compiled so a later stretch picks it up. Forty-five files have landed. This section
is what a reader who has not followed them needs in order to rule.

### 6.1 What file 43 established, and what survives the coordinate changes

Four findings, each re-checked against the design as it now stands.

**The no-finite-accumulator prediction is false in the ratified coordinates and true in the ones it
was written in.** Under a rational adjustment every quotient of two numerals with divisor index
bound K lies on the grid with relative denominator `lcm(1..K)`, and the lcm is minimal. **Survives
untouched.** Nothing ratified since has moved the identity contract's rational adjustment
(`78:190-192`).

**Division is a third growth class, at an accumulator width exponential in the precision.** I
recomputed the table offline rather than carrying it: six of seven cells reproduce exactly, `p = 5`
differs by one bit (52 against 51, an off-by-one in the value-range term rather than a disagreement),
and the growth law reproduces exactly, giving 94,547 bits at `p = 16` against file 43's "on the
order of 94,500" (`43:153`). **Survives.** One clarification the three-level width correction now
forces: the accumulator width is a fact about the **fields' extent**, the numeral-level width, not
about a container or a stored width (`83`, `86`), so nothing about it moves when a lowering does.

**The exact subfamily is division by any fixed nonzero representable constant, at zero new
mechanism.** **Survives, and gains an independent corroboration.** File 84 derived, forty-one files
later and for a different operation, that lifting an operand out of value position into type
position makes an operation value-keyed and law-eligible where the two-operand form is not
(`84:419-427`, `quantise::<Q>` against `quantise_to(x, y)`). `div_exact::<C>` is that same move,
made first. One clarifying sentence is owed so nobody transfers file 84's carve-out to division by
analogy: `quantize`'s carve-out exists because it reads its second operand's **datum** (its
exponent), and division reads its divisor's **value**, so general `div(a, b)` is value-keyed and
law-eligible in both forms and only the type-level form gets the codegen and the trivial grade.

**The overflow band is empty for same-precision division and inhabited once precisions decouple.**
**Survives**, and is now more useful than when written, because the ratified preset tables give
every preset a definite `OverRange` row (`78:409-441`) that the correction lands in.

### 6.2 What has landed since that division was waiting for

**The far-point rule answers what `x/0` resolves to, and answers it by a limit argument that yields
the standard's own split as a theorem.** `78:275-286` ratifies the far point as the supremum of a
numeral's ordered representable values. Apply it to division: as the divisor is driven to zero with
a nonzero dividend, the exact quotient's magnitude exceeds every representable magnitude, so the
one-sided limit is the supremum in the dividend's sign direction and the far point is the answer, at
every one of the four `Specials` members (`89_probes/probe_5`, CLAIM A). With a zero dividend the
limit does not exist, so the far point has no answer, at any `Specials` member (CLAIM B).

**IEEE clause 7's two-way split between `divideByZero` (result: the correctly signed infinity) and
`invalid` (result: NaN) is therefore the presence or absence of a supremum.** That closes the cause
split file 43 left owed (`43:229-240`) with a derivation from a ratified rule instead of a citation
to a standard, which is the direction this review prefers everywhere else. It also delivers file
34's stability hypothesis its standard-blessed instance for free (`43:236-238`): where `Specials`
carries infinity the far point is `Absorbing` in section 1.16's own vocabulary, and where it does
not the far point is `Finite` and silent, which is exactly the kind the ratified projection already
publishes (`78:308-316`).

**File 84's three-kind failure sort gives division's failure a home.** Kind 2 is "the result value
does not exist mathematically", and `84:239-243` names division by zero as its canonical member,
with generators `invalid` and `divideByZero` already in the design's own table since file 50 and
carried unchanged through four consolidations. So the design has a place for it, and `84:257-263`
states the shape a preset row would take: a **third position on the existing `Resolution` axis**,
never a second axis, with each preset's cell derived from its own intent by file 70's method.

**File 84 §5 changes division's cost picture.** A kind-2 failure whose admissible domain is
expressible as a predicate is refused at the declaration and every downstream operation is total
(`84:386-392`), and where it is not, the biased-niche lowering makes the refusing carrier the same
width as the infallible one at nine extra instructions across sixty-four elements. Division by a
`NonZeroable` divisor is exactly the first case. So file 43's `div_floor`/`rem` as
exact-and-partial operations no longer carry a layout penalty, which was the one real cost of
shipping them.

### 6.3 The one live fork, stated symmetrically with its consequences derived

Both alternatives resolve `x/0`; they disagree about what kind of event it is.

**Alternative 1: `x/0` with a nonzero dividend is a range event on the result numeral, and `0/0` is
the only genuinely kind-2 division failure.** The exact result's magnitude exceeds the numeral's,
which is `84:232-237`'s own definition of kind 1, and the preset tables' `OverRange` rows govern it
as written.

- What it buys. No new axis position. The far-point rule extends with no new text. IEEE's two-way
  split falls out as a theorem (6.2). Division's failure vocabulary shrinks from two kinds to one
  plus a boundary case.
- What it costs, found by the model rather than assumed. **Exactly one cell has no answer**, and it
  is `Hot` fixed-point (`89_probes/probe_5`, CLAIM C, computed over the whole preset matrix). The
  ratified fixed-point row resolves `OverRange` by `ReduceModulo` (`78:411-412`), and an unbounded
  exact result has no residue modulo anything. Clamping does have an answer (it is the far point),
  and refusing needs no value, so `Warm`, `Cold` and `Precise` are unaffected. So alternative 1
  buys its economy at the price of one stated exception to a ratified table.

**Alternative 2: division by zero stays kind 2 and gets a third position on the `Resolution` axis,
whose four cells are derived by file 70's method.** This is `84:257-263`'s own shape, applied.

- The derived row, with each cell's reasoning attached rather than asserted. `Hot`, "as fast as
  possible", takes the cheapest defined value the target gives away; on this host the integer divide
  instruction defines division by zero as zero and does not trap, read via `sdiv` rather than
  assumed (`89_probes/probe_5`, CLAIM D). `Warm` and `Cold`, "behaves intuitively" and "between warm
  and precise", take the nearest defined value, which is the far point in the dividend's sign
  direction, so they agree with alternative 1 exactly. `Precise` refuses.
- What it buys. No exception to any ratified table. Every cell derived from a preset's own stated
  intent. The kind sort stays as file 84 derived it.
- What it costs. A third position on an axis that has had two since the design began, and the
  pairwise-difference discipline the preset table carries has to be re-run over a wider table. And
  the far-point agreement for `x/0` becomes a per-preset coincidence to be re-derived rather than a
  theorem.

**One thing both alternatives share and the design should state either way.** `Hot`'s cell is a
target fact, not a resolution constant: aarch64's integer divide returns zero for a zero divisor and
x86's traps. `84:145-160` already derived that a preset row can be a consequence of the target's
lowering rather than a table lookup, for `At<N, Q>`. Division is the second instance, and it means
`Hot`'s division-by-zero answer belongs on the `Door`, beside `HostFloat<E>`'s reachability, rather
than on `Resolution`.

### 6.4 What file 43's proposal looks like restated for a ruling

Unchanged in substance from `43:264-288`, with the two things the intervening files supply folded in
and the paraphrase's error corrected.

The operation surface is **three** operations, not two. `div_floor` and `rem`, each exact, each
partial on the divisor's nonzero-ness, jointly bound by a compiled Euclidean law. And general `div`,
the atomic `quantize(exact quotient)`, implemented from the pair rather than from a wide quotient,
which is what file 43's probe 4 licensed by showing correct rounding is a function of the scaled
remainder. Removing `div` would be the policer posture (`arvo-toolbox-not-policer.md:33`) and would
repeat the shape file 84 rejected for `quantize` one section over.

What the intervening files add. The partiality is refused at the declaration wherever the divisor's
domain is a predicate, and carried in a niche at no layout cost otherwise (`84:386-397`). The
divisor-is-a-constant predicate still decides exactness, totality, accumulator existence at linear
width, and `Direction`'s presence in the key, all at once (`43:242-248`), **and whether it also
decides grade triviality is exactly the second call**: under reading A it does, because the site is
there whether or not it moves; under reading B it does not, because a general division whose
quotient lands on-grid raises nothing either. That coincidence, which is one of file 43's best
findings, is a reading-A property.

*Grounded on: ratified (`78:275-286` the far point, `78:308-316` the Absorbing/Finite kind,
`78:409-441` both preset tables, `arvo-toolbox-not-policer.md:33`, `84`'s kind sort as carried),
settled shapes (`43` in full, `84:145-160`, `84:239-243`, `84:257-263`, `84:419-427`, `84:386-397`,
`70:140-180`, `83`/`86` on the three width levels), compiled (`89_probes/probe_5` all four claims,
the offline lcm recomputation in `OUTCOMES.md`), tree-fact (`arvo/tests/fixed_point_div.rs:69-72`,
the current behaviour only, as evidence of why the redesign exists), external (IEEE 754-2019 clause
7, secondary, primary read owed), reasoned (the far-point extension to a limit, the alternatives'
consequences, and 6.4's restatement, mine).*

---

## 7. Why these are one call

Three ways, each compiled or cited rather than asserted.

**Division is where the counting question has teeth.** Addition's quantiser site fires
conditionally, but a fold's grouping-invariance hides the difference under reading A (section 3,
region 2). Multiplication has no interior site at all. Division is the one operation whose site is
applied unconditionally and moves the value on only some inputs, so its laws read the difference
directly and its grade is decided by it (`89_probes/probe_2`, the round-trip's finest view moves a
full rung).

**Op's own phrase for the held item names the grade, not the surface.** `86b:56-57`: "division's
grading axis, held since checkpoint ten." Checkpoint ten held file 43's whole finding, and the part
of it that has been unresolvable since is the one sentence file 43 handed back
(`43:324-328`): the event reading, "op's, not mine."

**The two calls' answers constrain each other.** If op takes reading A, division's grade is a
compile-time constant, file 43's five-way coincidence at `43:242-248` holds as stated, and `div`
publishes one event on every call including `x / 1`. If op takes reading B, the coincidence loses
its grade-triviality member, `div_exact::<C>` earns its separate existence on codegen and
law-eligibility rather than on grade, and division's round-trip law moves from the weak-equation
corner to `Precise`'s point. Neither call can be ruled without ruling the other, and that is why
they have both sat.

*Grounded on: ratified (`86b:56-57`, `44b:21-30`), settled shapes (`43:242-248`, `43:324-328`),
compiled (`89_probes/probe_1`, `probe_2`), reasoned (the connection, mine).*

---

## 8. What a consolidation could take close to verbatim, if op rules the way section 5 points

Offered as one candidate shape, not as the answer. Its virtue is that it does not require op to
choose between two facts, because it keys each where the layer-keying rule puts it. Its cost is
stated with it.

*The design's grade is a free commutative monoid over refusal causes and quantisation events. It is
counted twice, at two layers, because two different facts are wanted and they key differently.*

*The **site count** is a fact about the operation and its numerals: the multiset of quantiser sites a
term contains, charged whether or not a site moves a value. It is a function of the monomorphised
type, it is the published grade's parameter, and it costs one instruction outside any loop. It is
what a compile-time worst-case bound reads and what a caller's `Folded<N>` contract carries. It is a
sound upper bound on the moved count, in the same over-approximating direction the design takes
everywhere on lattice containment.*

*The **moved count** is a fact about a run: the sites at which the delivered value differed from the
exact one. It is IEEE's own raising condition for `inexact`, `underflow` and `overflow`, it cannot
be a type, and it is not published by default. A numeral declaring standards conformance carries it;
so does a consumer that asks for it by taking a combinator that returns it. Under the ratified float
preset table it is unavailable through the hardware door without either serialising the FPU on a
per-operation flag read or adding the bookkeeping `Warm`'s own row rejects, so a numeral publishing
it takes `Door = Quantised`. That is a diagnostic, not a directive.*

*A law's event component reads the moved count, because a site count is grouping-invariant by
construction and a law comparing site counts across groupings says nothing. The finest-view table
therefore stands exactly as measured.*

*Multiplicities live in the underlying monoid and appear only inside a law's verdict. The published
carrier is the monoid's presence view, joined idempotently, so a shared operand contributes once and
no consumer ever sees a multiset.*

**The cost of that shape, stated rather than smoothed.** It carries two counts where the review has
been assuming one, so every place the corpus says "the grade" has to say which, and the `Folded<N>`
contract's `N` becomes unambiguously the site count with the moved count riding beside the value.
Against that: it is the only shape found here in which no ratified artifact has to be re-derived.
Both single-reading answers cost a re-derivation, of file 37's table under A or of file 43's grade
triviality under B.

**This is one expert's reading and it is owed a second**, per the review's own convention. I have
stated it in the form a second reader can attack: the layer assignment is the claim, the two counts
are compiled, and the cost sentence names what it would take back.

*Grounded on: ratified (`78:137-150`, `78:409-441`, `78:448-455`, `40:308-312`), settled shapes
(`37:171-179`, `49:464-546`, `50:294-307`), compiled and measured (`89_probes/` in full), reasoned
(the proposal itself, mine, and owed a second read).*

---

## 9. What this file does not decide

**Both calls.** They are op's, they were op's before this dispatch, and nothing above is a ruling.
What I have tried to change is the cost of ruling: each alternative now has its consequences derived
and compiled rather than argued, and the one that was invisible (that a ratified table already
answers half of it) is on the page.

**Which of section 6.3's two division alternatives is right.** I state both with their derived costs
and decline to pick, because the choice turns on how much a stated exception to a ratified table
weighs against a third position on a two-position axis, and that is a taste about the shape of the
canon rather than a mathematical fact.

**The signed halves of file 43's probes 2, 4 and 5**, still unbuilt, still owed before the division
spec text hardens, exactly as `43:314-317` left them. My probes are unsigned or use a small signed
model and do not close that.

**The float-division path against IEEE's tables**, still unbuilt (`43:319-322`). Section 6.2's
far-point derivation makes it cheaper to build than it was, because the `Specials`-carrying model
numeral it needs is now specified, but it is still a compile nobody has run.

**The primary-source read on IEEE clause 7.6's raising condition for `inexact`.** I use it as a
secondary read and mark every place it bears. It belongs in the same bundle `78:934-941` already
carries and would corroborate section 4.1's standards consequence rather than establish it.

**A throughput number for reading B.** The instruction counts are honest and the dependency-chain
observation says they overstate the latency cost. The bench belongs in `mock/benches` under the
harness and I did not run it, because the orchestrator overwrites committed artifacts.

**Whether the FPSR route is ever worth shipping.** I established it exists, which corrects one
sentence. Its per-thread nondeterminism under a pluggable executor is file 50's own argument and it
stands, so my expectation is that it is not, but that is a reasoned expectation and the design
question belongs to whoever prices the conformance path.

---

## 10. Standing

Nothing here overturns a ratified call. Section 3's table reproduces `37:171-179` rather than
replacing it, and the finding is about what that table was measured under, not about whether it is
right. Section 6 leaves every one of file 43's four findings standing and adds the machinery that
landed after it. The three corrections in section 1 are to unratified prose and to one probe
header, and the fourth (file 50's availability sentence) leaves that file's conclusion intact and
narrows its argument to the half that carries it.

I have contradicted my own earlier file once, and it is the finding: file 37 reported a finest-view
table without stating which counting discipline it computed, and I did not notice when I wrote it or
when I read it back three times since. `86b`'s separation requirement is what makes that visible,
one file after it was adopted, and it is worth recording that the requirement caught something in
its first stretch rather than only being agreed with.

Every proposed sentence above is two-expert-agreement-shaped, not mine to close. Each carries a
compiled artifact, so agreeing costs a read and disagreeing costs a named compile.
