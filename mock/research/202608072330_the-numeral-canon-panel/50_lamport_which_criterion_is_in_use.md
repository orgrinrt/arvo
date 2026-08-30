# 50. Which criterion is in use

**Date:** 2026-08-09. **Persona:** Leslie Lamport. **Mode:** explore, do not settle (`00_brief.md`,
`04`, `28`). **Position in the unit:** sixth file on one topic, second of the second four, after
`48`'s checkpoint and `49`'s cold derivation. Nothing here settles anything.

**Status: COMPLETE.** Written to disk before the work and extended in place, per `RULES.md:328-329`.

My subject is the question `48` found and named highest priority for this slot: `16` applies its own
criterion two different ways inside `16`, and the two ways give different answers, so a topic six
files deep has been adjudicating against a sentence that does not decide its own cases.

The short version of what I found. The two readings are real and `48` is right that they disagree.
But "which criterion is in use" is the wrong question, and answering it would have produced a wrong
answer confidently. **The sentence at `16:100-101` is not a definition at all. It is a fixpoint
equation whose operator is non-monotone, so it has several solutions and no principle selects
among them, and it leaves three separate parameters unstated.** Enumerated exhaustively over those
parameters, one sentence admits output sets of size zero, two, three and four, and the two sets of
size two are not the same two. That is not an ambiguity to resolve by picking a reading. It is a
sentence that cannot be the topic's criterion under any reading, and section 3 says what has to
replace it.

Along the way: `48`'s Reading B does not give a count of one, `48`'s cited evidence for the collapse
does not show what it is cited for (although the collapse is real and I built the thing that shows
it), the access-width verdict `48` wanted reinstated should be reinstated but for a different reason
than either file gives, the `Precise` fork is not a fork on the axis this topic cares about, and one
magnitude three files have called unpriced has been sitting measured in `mock/benches/` the whole
time.

## 0. Gates

### 0.1 Canon gate

There is no ratified canon to defend or diverge from. The fixed material is `01`, `04`, `28`,
`INTENTS.md`, the workspace discipline, the forbidden-feature list, and the acceptance criterion in
`00_brief.md`'s "What is fixed". My question sits inside it: the criterion's second noun is what
"how many outputs" is about, and whether the panel's test for "output" decides anything is upstream
of every verdict in the unit. Nothing below proposes anything the forbidden-feature list excludes.
Every `.rs` file in `50_probes/` carries zero `#![feature]` gates and `verify.sh` prints the count
per file. **Gate: passes.**

### 0.2 Test gate

There is no suite. `mock/crates` is empty by construction. My evidence is seven probes in
`50_probes/`, six that run and one that is expected to fail to compile, plus citations I opened.

I applied the gate's own checks to my own probes before shipping them. Two things it caught. My
first non-monotonicity demonstration in `p1` printed the same value on both rows and the narrative
claimed a change, which is the "asserting a value against itself" shape and I replaced it with a
case that actually moves. And `p2`'s first version reported a percentage over the wrong denominator,
counting distinct class memberships while the label said declarations; the number was 128 of 251 and
the honest number is 389 of 512. Both are recorded here rather than quietly fixed, because `46`
found the same class of defect in `45` one file ago and the useful part is that it is easy to make.

### 0.3 Independence

My brief gave a reading order that puts the unit first, so **I did not derive cold**, and `49` is the
unit's cold derivation rather than me. Where I land where a predecessor lands, that is a read. Where
I attack, independence is not needed: an attack has to be correct and carry its own citation.

I did not run `git log` in this repository before writing, per `RULES.md:377-385`. I did run
`git ls-files` and `git status` afterwards while committing, which is how section 1.3's finding
surfaced.

## 1. Three things I checked before starting, two of which are findings

`RULES.md`'s standing instruction is that the brief is the one document nobody checks. `48` checked
its own brief and found two errors. I checked mine, and `48`, and the tree.

### 1.1 The brief's citations into `16` all hold

I opened every one. `16:100-101` is the criterion, verbatim as quoted. `16:187-189` is Reading A in
use. `16:572-577` is the passage `48` files as Reading B. `16:126-141` is the injectivity failure.
`16:280-282` is the second argument. `16:185-189` is the access-width dismissal. No defect.

The brief's summary of what the unit established also holds, with one wording I would tighten. It
says one richer output suffices "if and only if it is a type", which is `47`'s own phrasing
(`47:164`), and `47` immediately qualifies it: once the single output is a type with named
projections it **is** the pair wearing one name (`47:174-178`), so the biconditional is about
packaging and not about arity. The brief carries the slogan without the qualification, which is the
compression failure `48:116-122` names in `44`, one generation on.

### 1.2 Two labels collide inside this unit, and a reader will conflate them

`48` and my brief use **Reading A** and **Reading B** for two site models: the site holds only the
derivation's outputs, or the site holds the numeral type. `47:76-80` uses **Reading B** for something
else entirely, "how many facts must be independently observable", against a Reading A it never
defines in the text I read.

That is not a defect in either file. It is a collision that will produce a wrong reading in the
consolidation, because both files are cited on the same page about the same sentence. I use `48`'s
labels below because my dispatch does, and I flag it so the consolidation does not inherit two
meanings for one word. A better fix is to retire the letters entirely and say **site-holds-outputs**
and **site-holds-declaration**, which is what they mean and cannot collide with anything.

### 1.3 `46` had never been committed, and four files rest on it

This is the finding, and I found it by running `git log --oneline -- '46_*'` while staging my own
work, which returned nothing.

`46_dolan_the_carrier_collision_attacked.md` was untracked. It has no history. Yet `47` reads it in
full and builds sections 3 and 9 on it, `48:155-184` calls it "the cleanest work in the unit, and
the model", `49` reads it in phase two, `45` was resumed and conceded two claims to it, and
`OPTIONS.md` carries its downgrade of `45`'s wide-rung forcing as the current state of that thread.

`RULES.md:108-109`: "Evidence lives in the repo or it never happened. A spike outside the panel's own
`NN_probes/` may not be named, referenced, or reasoned from, and anything resting on one is void."
The rule names spikes and `46` is a panel file, so the letter of it does not fire. The reason behind
it does, and harder: a file nobody can fetch, carrying an attack that moved a register entry and
forced a predecessor to retract, is exactly the artifact the rule exists to stop a design resting on.

`evidence-lives-in-the-repo-or-it-never-happened.md` gives the remedy and it is one step: if the
thing exists and is recoverable, recover it and commit, with no audit pass and no rewrite. It
existed, unchanged on disk for over an hour while three files were written against it, and its own
header reads `**Status: COMPLETE.**` (`46:8`). I committed it as `7a3bddd`, alone, before my own
work, so the recovery is legible as its own act.

I did **not** touch `45_probes/p7_alignment_lemma_abstract.rs`, which is still modified in the tree
with the rustfmt delta `48:289-303` diffed and declined to commit. That file is committed, so nothing
resting on it is void; the delta is a hygiene item and `a-shared-clone-is-someone-elses-desk.md`'s
reasoning against tidying someone else's tree still applies to a tracked file's working-copy
changes. Two files have now named it and nobody owns it.

### 1.4 What `48` asked me to check first, and I did

`48:584-585` says to grep `DROPLIST.md` before carrying its three-question shape forward, in case it
is already a dead route. I grepped for the shape and for its vocabulary and found nothing: the
droplist has three headings (`DROPLIST.md:5`, `:424`, `:449`) and no entry about a per-value against
per-aggregate decomposition. So it is not a dead route. It is also not mine to propose, and section 3
reaches something adjacent by a different argument.

## 2. The criterion is a fixpoint equation, not a definition

### 2.1 What the sentence actually says

`16:100-101`:

> A component is an output of the derivation when the consumer did not write it, the machine needs
> it, and a downstream site that holds the other components cannot recover it.

"The other components" are the other members of the set being defined. So the sentence has the
defined object on both sides, and written out it is:

```
O  =  { f  in  NEEDED \ DECL  :  not derivable(f, (O \ {f}) union HELD union PRIM) }
```

where `HELD` is whatever a site holds besides the outputs, `PRIM` is the language's primitives, and
`DECL` is the declaration that clause one excludes.

A recursive definition of this shape needs two things the sentence does not supply. It needs the
operator to be monotone, so that a least or greatest fixpoint exists and one of them can be named.
And it needs `HELD` to be stated, because the predicate quantifies over a site whose contents the
sentence never fixes.

**The operator is not monotone.** Growing `O` makes more facts derivable, which removes facts from
`O`. `p1` prints the case: with the site holding `{CARRIER}`, the packed access width is not
derivable and is therefore an output; with the site holding `{CARRIER, STRIDE}`, the width becomes
recoverable and so does the access width, so it is not one. Adding a member removed a member. No
least or greatest fixpoint is available as a tie-break, and nothing else in the sentence selects.

`16` knew the criterion was the crux and said so at `16:95-97`: "How many outputs" needs a criterion
for what counts as an output, or the answer is unfalsifiable. That sentence is exactly right and the
criterion written to answer it inherits the problem in a more expensive form, because an
unfalsifiable question is visibly unfalsifiable while a fixpoint equation with several solutions
looks like a definition.

### 2.2 Solved exhaustively, one sentence admits four different counts

`50_probes/p1_criterion_fixpoints.py` writes the equation down and enumerates every subset of the
fact universe the unit has named, in each cell of a grid of three parameters the sentence leaves
unstated:

- **the site model**, `HELD` empty (Reading A) or `HELD` the declaration (Reading B);
- **the strategy set**, closed at the prior attempt's four or open per `INTENTS` I1;
- **the kind regime**, whether a rule may produce a type from consts, which
  `16_probes/p5b`, `47_probes/p2` and `47_probes/p3` between them refuse twelve times.

Every derivability rule in the model is taken from a cited panel result and is listed in the file's
own table with its provenance. The result, sixteen cells:

```
reading   stratset  kind     criterion-consistent output sets     unreachable but needed
A         S4        blind    {CARRIER, STRIDE}                    none
A         S4        aware    {ACCESS, CARRIER, STRIDE}            none
A         S5        blind    {ACCESS, CARRIER, STRIDE}            W
A         S5        aware    {ACCESS, CARRIER, STRIDE}            W
B         S4        blind    {}                                   none
B         S4        aware    {ACCESS, CARRIER}                    none
B         S5        blind    {}                                   none
B         S5        aware    {ACCESS, CARRIER}                    none
```

and with a compute carrier in play, the same eight with `COMPUTE` added to every non-empty row.

Seven distinct answers across the grid. **Counts of zero, two, three and four.** The unit's answer,
`{CARRIER, STRIDE}`, is produced by exactly one cell out of sixteen, and it is the cell that reads
the site as impoverished, the strategy set as closed, and the kind boundary as absent. Two of those
three are contradicted by things this panel already holds: `INTENTS` I1 says the strategy set is
open on op's direct word, and twelve compiled refusals say the kind boundary is there.

I want to be exact about what this is and is not. **It is a model of one sentence, not a measurement
of arvo**, and a reader who rejects one of its derivability rules should reject the row it produces.
Its value is not the particular sets. It is that the answer moves this far under parameters the
sentence does not name, which is a property of the sentence rather than of my modelling.

### 2.3 Reading A is not merely a different reading, it is unsound

`48` treats the two readings as two available meanings. One of them is not available.

Under Reading A, a downstream site holds only the derivation's outputs, and clause one of the
criterion excludes the declaration from being an output. So the site never holds the declared width,
and `16:572-577` applies exactly that exclusion to demote `EXTENT_BITS`.

`50_probes/p2_reading_a_loses_the_width.rs` counts what that costs, over widths one to 128 and four
strategies:

```
distinct (carrier, stride) classes                    =  128
declarations whose W IS recoverable from the pair     =  123 of 512
declarations whose W is NOT recoverable from the pair =  389 of 512  (76.0%)
```

The largest ambiguous class is the 128-bit carrier at stride 128, standing behind 64 declared widths.
And the pattern is the reverse of the one the topic has been arguing about: the width is recoverable
exactly where the strategy packs, because there stride is the width, and it is destroyed by the
unpacked strategies, which is where the topic assumed everything was fine.

A site that has lost the width cannot shift by `F`, cannot mask to `W`, and cannot detect overflow at
the declared width. Those are things the machine needs, and clause one has excluded them while the
site model has made them unreachable. **That is not a criterion producing a smaller count. It is a
criterion producing a result from which a needed fact cannot be obtained**, which is what a
specification is for and what this one fails at.

So Reading A is refuted by clause one's coexistence with it, not by preference. Whatever the
criterion means, it cannot mean that.

### 2.4 Reading B does not give one, and `48` is wrong about the count

`48:338-340`: "under Reading B the count is one, and under Reading A the count is two, and `16` uses
A to establish the second output and B to demote the third."

The first half does not survive its own test. Under Reading B the site holds the declaration, so the
criterion's third clause is applied to **every** candidate including the carrier. The carrier is a
function of the declaration, so if the kind boundary is ignored, the carrier is recoverable too and
the criterion demotes it as well. `p1`'s `B/S4/blind` and `B/S5/blind` rows both read `{}`: **the
derivation emits nothing at all, and every needed fact is still reachable.** That is the reductio the
criterion runs into when its own third clause is applied consistently, and it is a stronger objection
than the one `48` makes, because it does not depend on the stride specifically.

Once the kind boundary is honoured, Reading B gives `{ACCESS, CARRIER}`: two facts, which is the
unit's count, and **not the unit's two.** The stride drops out because it is a const recoverable from
the declaration and the carrier, and the packed access type enters because it is a type and a type
cannot be reached from consts. So the same sentence, read the other coherent way, produces a pair
that shares one member with the pair the topic has spent six files establishing.

That is the sharpest form of `48`'s finding and I would put it in its place: not "the two readings
give one and two", but **"the two readings give two different answers, one of which is degenerate and
one of which agrees on the arity while disagreeing on the members."** A criterion that gets the count
right and the contents wrong is the more dangerous failure, because the count is what gets quoted.

### 2.5 `48`'s evidence for the collapse does not show what it is cited for, and the collapse is real anyway

`48:331-336` says stride is recoverable under Reading B, and offers `47_probes/p2b` as the proof:
"This is not a hypothetical about what a sufficiently clever site could do. `47_probes/p2b` compiles
it."

I opened `p2b`. It does not compile that. Its `stride_of::<W13, Cold>()` reads
`<W13 as DeriveScalar<Cold>>::REPR`, and that `REPR` is written into the impl as `pack(16, 13, 32)`.
The thirteen comes **out of** the derivation; the site projects it. `p2b` is a probe about the kind
boundary, it is correct about that, and it is not evidence about recovery. Nobody in this unit has
built a site that recomputes the stride from the declaration.

So I built it. `50_probes/p3_site_recomputes_the_stride.rs`, arm one: a derivation with exactly one
associated item, `type Carrier`, a strategy marker carrying `const PACKED: bool`, and a site-side
`const fn` that computes the stride itself. It compiles gate-free and the answers are
const-asserted: 13 for `Cold` at thirteen bits, 16 for `Warm`. **The collapse `48` reports is real,
and now there is something in the repository that shows it.**

Then the rest of the probe, which is what makes it worth having.

**Arm two.** Add one fifth strategy that packs to a four-bit grid. Nothing about that is exotic: it
is what a design picks when it wants packing and cheap nibble-aligned addressing, and `INTENTS` I1
says the strategy set is open. The site's formula now answers 13 where the truth is 16, silently: it
type-checks and returns a number. And it **agrees** at width twelve, which is exactly the shape that
makes a sampled test report green over a broken rule.

**Arm three.** The repair: put the grid on the strategy marker as `const GRID_BITS`. The site is
correct again for all five. But the fact did not disappear. It moved from the derivation's result
onto the strategy, where the derivation still supplies it.

**Arm four.** A sixth strategy that pads each element by one bit to carry an inline validity flag.
Not a grid. The repaired formula is wrong again.

Every disagreement in that file is a `const _: () = assert!(...)`, so the compiler checked it.

The reading that follows: under Reading B the stride is "recoverable" only in the sense that the
site can re-implement the packing rule. Whether that counts as recovery is precisely the question
the criterion was supposed to answer and instead begs.

### 2.6 So what replaces the question

**"Which criterion is in use" presumes there is a criterion and asks which of its meanings the panel
selected.** Both halves fail. There is one sentence with several solutions, and no file in the unit
selected a meaning; `16` applied whichever clause reached the answer in front of it, which is not
dishonest and is what an underdetermined predicate invites.

The question that replaces it is: **what must a criterion state to be one?** Section 3.

## 3. What a criterion for this has to state, and it is three things rather than one

The sentence at `16:100-101` is trying to be three predicates at once. Separated, each is decidable
and none is controversial. Fused, they produce the fixpoint.

This is offered as the shape a criterion needs, not as canon text. Op's mode forbids settling and
`RULES.md:79-83`'s two tests are the consolidation's job.

### 3.1 The site model is a premise, and it is stated rather than inferred

**A lowering site holds the numeral's type, and therefore the declaration, the derivation's result,
and the language's primitives.**

That is not a consequence of anything. It is a design commitment, and every verdict in this topic
turns on it. `45:314-333` establishes that nothing in the panel proposes dropping the strategy
parameter early, and `46:244-251` confirms it independently by opening the acceptance criterion's
"erase on lowering" and finding erasure placed at codegen. So the commitment is already made
everywhere in practice and stated nowhere, which is how one sentence came to be readable two ways.

Write it down and Reading A ceases to exist. That alone removes half of `48`'s finding, and it costs
one sentence.

### 3.2 The locus clause fixes which facts, and it is about whose rule gets applied

Recoverability cannot fix the fact set. `16:95-97` says why: everything downstream is a function of
the declaration, so under any site model that includes the declaration, nothing is unrecoverable in
principle and the only thing keeping the count above zero is the kind boundary. `p1`'s `B/blind` rows
are that observation as a computation.

The clause that does the work is already in `16`, in a different section, and it does not use the
criterion at all. `16:280-282`:

> emitting the extent and recomputing the carrier at each use would re-enter, at every use site, the
> problem the derivation exists to solve once.

`48:352-355` reads this as a test about **where a rule is applied** and I agree with that reading.
Where I would sharpen it: "would have to re-derive" is as vague as "cannot recover" unless it says
whose rule is being re-derived. The version that decides cases is

> **a fact belongs in the derivation's result when obtaining it requires applying a rule the
> strategy owns.**

Applied to this unit's cases it is uniform, and it gives reasons rather than test results:

- **The carrier.** The strategy owns the ladder. In the result.
- **The stride.** The strategy owns the packing discipline. In the result, and `p3` arms two through
  four are the demonstration: a site computing it is re-implementing the strategy, and it is wrong
  the moment a strategy the rule did not anticipate exists.
- **Alignment.** `align_of` is the language's, not the strategy's. Not in the result, which is
  `16:605-613`'s verdict reached without its test.
- **The packed access width.** Section 4.
- **A compute carrier.** In the result exactly when some strategy is entitled to diverge it from the
  at-rest form, which is `45:346-362`'s "forced by semantics" reached from the same place.

Two independent instances of this clause already exist in the panel and neither author knew of the
other. `49:98-105`, written cold, before reading anything: a quantity earns a place "when getting its
value requires consulting the strategy as an actual decision (not a formula), and when an entity
other than the numeral itself needs the answer and cannot safely re-derive it without risking
disagreement with what the numeral's own definition intended". That is the locus clause plus a
single-source-of-truth clause, derived from op's intents alone. And `49:363-370` then adopts `48`'s
re-derivation test over `16`'s on reading the panel, while noting its own criterion is close enough
to `16`'s that `48`'s critique probably applies to it too. It does, and the part of `49`'s that
`16`'s lacks is the part that survives.

### 3.3 The kind clause fixes the form, and `47`'s sentence is already it

`47:506-510`:

> the derivation's result must make available, as types, every fact a lowering site cannot recompute
> from a const; facts recoverable as consts from those types are not further outputs.

`48:396-425` judges this true and insufficient, and is right on both. It is the kind clause and only
the kind clause. Its second half is the part that does the damage on its own: read literally, a
derivation emitting only a carrier satisfies it, which `p1`'s `B/blind` rows produce mechanically.

Under the three-clause split it stops being a candidate for the topic's sentence and becomes exactly
what it is: the answer to a different one of the three questions, and a good one. `48:414-418`'s
worry about "as types" and "from a const" being one language family's vocabulary is real and is
smaller than it looks, because the clause is about a boundary the derivation crosses rather than
about Rust: a compile-time derivation hands a site things of two sorts, and the asymmetry between
them is what the twelve refusals measure.

### 3.4 The count is then a consequence, and the canon does not state it

With the three clauses separated, "how many outputs" has an answer of the form: as many facts as
there are rules a strategy owns, carried in the form each consuming site needs. The number is a
function of the strategy set, and `INTENTS` I1 says that set is open.

That is why the count keeps moving. It is not underdetermined evidence, it is a quantity the design
has declared open, and a canon that fixes it is contradicting I1 in a place nobody would look for the
contradiction.

## 4. The access width, which the locus clause reinstates, and which I found `16` is over-computing

`48:361-366` says the packed access width is in the result under the locus test and that `16`
dismissed it on Reading A's test. I checked that, and found something neither file has.

### 4.1 `16`'s closed form is exactly right about what it computes

`16:186-189` says the maximum byte span of a `W`-bit field at unknown phase is
`floor((W + 6) / 8) + 1`. `50_probes/p4` brute-forces every phase for every width from one to 1024
and finds **zero mismatches**. Keeping something is a result and this is one: the arithmetic is
correct, and `47:363-366` reproducing `16`'s 28-of-64 from it is a faithful reproduction.

### 4.2 And it is the worst case over phases a packed run does not reach

`16:179-180` gives the reason it treats the phase as unknown:

> since thirteen and eight are coprime the phase cycles through all eight residues

True for thirteen. The phase set of a packed run at stride `s` is the multiples of `gcd(s, 8)`, so
all eight residues occur only when the stride is odd. At width twelve a `Cold` run reaches two
phases, not eight, and the true span is two bytes where the closed form says three.

`p4`, exhaustively over widths one to 128 under `Cold`:

```
widths whose packed run reaches all 8 byte phases : 64
widths where the closed form OVER-estimates       : 48 of 128
as a load type, rounded up to a power of two      : 15 of 128 pick a wider load than needed
```

Three consequences, each recomputed rather than argued.

**`16`'s own headline moves.** `16:384-386` reports the carrier is the wrong load type at 28 of 64
widths. Recomputed against the phases a `Cold` run actually reaches, it is **16 of 64**. The finding
survives and its size is roughly halved.

**`47`'s two-ladder cost is the opposite of what it reports.** `47:350-356` reports the native and
access rung partitions of widths one to 128 sharing zero jump points, and concludes a design needs
two width ladders or one over a ten-class common refinement. Recomputed from the true phase sets, the
access ladder jumps at 24 widths and the shared set is `[9, 17, 33, 65]`, which is **every** jump the
native ladder has. So the access partition **refines** the native one: one ladder keys both, and the
native rung falls out of it as a coarsening. It costs 25 classes rather than five, which is a real
price and a different price from the one `47` names, and the structural claim reverses.

**And it is a genuine fork rather than a correction.** The over-estimating form is
strategy-independent and gives six classes; the exact form is strategy-dependent and gives 25 and
one ladder. That is a composition, and I would put both in the register rather than replace one with
the other.

**The exact access width is a function of `(W, stride)`, not of `W`.** Under a grid-packing strategy
the same width needs a different span at 64 of 128 widths (`p4`'s third table). Since the stride is a
strategy's choice, the access width is strategy-keyed, which is the locus clause's test met, which is
`48`'s conclusion reached from arithmetic rather than from a change of test.

### 4.3 An independent instance of the phase-period fact, in the bench harness

`mock/benches/variants/bitpack-plan-windowed/src/lib.rs:1-4` describes its own decode plan as
"`P = 8 / gcd(W, 8)` fields per `W * P / 8` whole bytes, every byte offset and bit shift a
compile-time constant".

That is the same `gcd(W, 8)` period, written by another author, in committed harness code, before
this panel opened. So `p4`'s correction to `16:179-180` has an instance that is not mine and not a
restatement of mine.

## 5. The magnitude three files call unpriced has been measured the whole time

`47:536-541` declines to say whether any site needs the packed access width as a type, because a
width-generic byte loop is correct and what it costs against a fixed-window load is "unpriced".
`48:504-521` repeats it, notes that `15:337-339` and `16:286-289` say it too, and spends dispatch
slot 52 on pricing it, with a specification of the arms it would need.

`mock/benches/bitpack-decoder-shape` is committed, four sizes, and its two packed arms are those two
shapes:

- `bitpack-plan-naive`, "byte offset and bit shift both derived from the running index at runtime"
  (`mock/benches/variants/bitpack-plan-naive/src/lib.rs:1-3`);
- `bitpack-plan-windowed`, the compile-time plan quoted in section 4.3.

With `bitpack-plan-native`, the dense carrier, as the competitor arm `48` asked for.
`50_probes/p7_bench_readout.py` reads the committed CSVs and runs nothing:

```
n         naive / windowed       windowed / native      naive / native
16384     3.12                   1.51                   4.72
65536     3.06                   1.49                   4.56
98304     3.12                   1.50                   4.68
262144    3.04                   1.50                   4.55
```

**An access plan derived per element at runtime costs about 3.1 times one that is a compile-time
constant, stable to two percent across four sizes.** The packed compile-time plan costs 1.5 times the
dense carrier.

Two cautions on what that licenses. It prices **compile time against runtime**, not derivation
against site: a site holding the width and the strategy's rule as consts could reach the windowed
number itself, so the bench bounds the cost of the plan not being a const rather than the cost of the
site computing it. And it is one host, one width, sequential reads.

What it does settle is that the question is priced and slot 52 is redundant.

`RULES.md:337-357` records this panel losing a night to eighteen files calling the packed-storage
trade unpriced while `mock/benches/` held a committed harness run measuring exactly it, and adds the
lesson: "a negative claim about evidence is a claim about a place". **This is the second time, and it
happened in the checkpoint whose job was to redirect the second four.** `48:31` records not opening
`mock/benches/` and section 8 of that file says why it built nothing. That is honest and it is also
how the redirect came to spend a slot on an answered question.

The general repair is small and I would put it in `RULES.md` rather than in a file: before writing
the word unpriced, `ls mock/benches/ | grep <topic>`. It costs one command and it has now cost the
panel two dispatch budgets.

## 6. Is the `Precise` fork a fork?

`48:470-474` asks whether a strategy can do both, refuse when the result is inexact and compute wide
when it is not, and says that if it can, the fork dissolves and op's question changes shape before it
is put to him.

The answer is that the fork dissolves, and not by the route `48` guesses.

### 6.1 The two refusal designs are the same policy

`45:167-169` says the refuse-on-inexact reading "needs no extra compute width at all: refusing an
inexact result can be checked at the storage width plus one flag bit". Refusal can be applied per
step or once at the end of a chain, and applying it at the end requires carrying the chain exactly,
which is the widening the other reading was said to be alone in needing. So they look like different
designs with different costs.

They are not. `50_probes/p6` computes what each admits, exhaustively, with two independently coded
instruments (a brute force over every chain and a dynamic program over 2-adic valuations) that agree
on every cell the brute force can afford. **Once zero operands are excluded, per-step refusal and
end-of-chain refusal admit exactly the same chains, in all 42 cells checked**, and it is a theorem
rather than a coincidence:

Write `T_i` for the total 2-adic valuation of the start and the first `i` multipliers. Per-step
refusal admits iff `T_i >= i*F` for every `i`; end refusal admits iff `T_k >= k*F`. A nonzero raw
below `2^F` has valuation at most `F - 1`, so if `T_i <= i*F - 1` then
`T_k <= i*F - 1 + (k-i)(F-1) = k*F - 1 - (k-i) < k*F`. Contrapositive: `T_k >= k*F` forces every
`T_i >= i*F`. The hypothesis is exactly `W == F`, which is the probe's domain and is stated as its
bound.

The 1.5x to 4.5x gap in the with-zero columns is entirely chains containing a zero operand, which are
exact by accident.

### 6.2 And refusal is not a design for chains at all

Same probe, nonzero operands, purely fractional at `F` bits:

```
   F   k     admitted (per-step refusal, nonzero operands)
   8   1     1.182622%
   8   2     0.006170%
   8   3     0.000018%
   8   4     0.000000%
  10   1     0.391485%
  10   2     0.000693%
```

Eighteen chains in a hundred million, for three multiplies at eight fractional bits. A strategy whose
stated intent is to be accurate **within chains** (`INTENTS` I7) cannot be one that refuses all of
them, and that is a measurement rather than a reading of op's prose.

`18`'s 4.60% to 55.56% is a different domain, in-range multiplications and divisions over mixed
integer and fraction widths, so these numbers do not replace it and are not comparable to it row for
row. What they add is the chain dimension, which is the dimension I7 names and which nothing in the
panel had measured.

### 6.3 What actually survives as a question

Every fixed-point multiply forms a `2W`-bit product internally, under every strategy, because the
useful result is the high half. So forming a wide value is not what distinguishes `Precise`.
**Carrying it between operations is.** The container-derivation question is whether the numeral's
compute form, the thing a chain's intermediates live in, differs from its at-rest form. That is one
sentence, it is op's, and it is not the sentence `45:454-459` proposes putting to him.

### 6.4 A reading of `Precise` the whole unit has excluded without noticing

`INTENTS` I2, quoted from `seed/SETTLED_strategy.md:37`, in op's own words:

> `Precise` is the most precise **at the price of both storage and compute**

Every model of `Precise` in this unit assumes its storage equals `Warm`'s. `47_probes/p5`'s comment
says "Precise stores exactly as Warm does; only its COMPUTE type differs". `49:75` reasons that
"its storage stays at the logical width since Precise has no stated reason to spend extra storage at
rest". `16_probes/p5` models widening as compute-only.

Op's own words say `Precise` pays in storage. `36`, quoted at I7, adds "throwing out all cold or hot
axis optimisations", and the cold axis optimisation is minimising storage.

I am not proposing that `Precise` stores wider, and I have no evidence that it does. What I am
reporting is that **a live reading of op's stated intent was excluded by three files without any of
them noticing they were excluding it**, and that it is the reading under which `47`'s section 4
result collapses: if `Precise`'s at-rest form is wider than `Warm`'s, the pair separates them and
"two is already insufficient" has no witness.

That is a cheap thing to put to op and it is a different question from `45`'s.

## 7. `47` section 4 restated without its model, checked by someone who did not write it

`48:227-229` proposes the modelling-independent form:

> under the widening reading there are three distinct facts and one slot named "carrier", so
> whichever of the two that slot denotes, the other is unrecoverable from the result.

`50_probes/p5` builds both assignments with the three facts named apart, and `p5b` carries the two
refusals.

**Under M2, the slot denotes the at-rest type.** `Warm` and `Precise` at thirteen bits give `(u16,
16)` and `(u16, 16)`. The pair does not separate them. Compiled as an assertion that must **not** be
refused, so its absence from `p5b`'s error list is the result. This is `47`'s arm and it holds.

**Under M1, the slot denotes the compute type**, which is `16:148-150`'s own usage. The pair gives
`(u16, 16)` and `(u32, 16)`. It **does** separate them.

So the "either way" is not symmetric on information, and `48`'s stated reason is wrong. What holds is
the conclusion, for a different reason: under M1 the at-rest type is not among the slots, and the only
route to it is from the stride, which is a const. `p5b` is refused three times reaching a type from a
stride const generically, naming the forbidden `generic_const_exprs`, from a third starting point
after `16_probes/p5b` and `47_probes/p2` and `p3`. `p5b` also refuses `u16: SameType<u32>`, which is
the non-vacuity control for `p5`'s must-not-refuse assertion.

**Both halves close, and the same wall closes them**, which is `47`'s kind boundary and not
information at all. That is the third time in this unit that a result argued on information grounds
turned out to be a kind result, after `47:164-170` found it in `16` and `48:368` found it in the
count.

On the three-instance bar (`RULES.md:116-118`): the kind asymmetry now has `16_probes/p5b`,
`47_probes/p2`, `47_probes/p3` and `50_probes/p5b`, by three authors, from four starting points. `47`
called it two instances at best and asked for a third author attacking the wall rather than
confirming it. I attacked it, in the sense that I built the route `48`'s claim needed and expected it
to work; it did not. That is a third instance and I would still not call the question closed, because
all four probes share the assumption that the width arrives as a const, which is `10`'s bridge
problem and is where the assumption should be attacked next.

## 8. Is the fact set closed under an open strategy set?

No, and `p3` arms two through four are the demonstration rather than the argument. One plausible
fifth strategy makes a verdict derived over four silently wrong, the repair relocates the fact onto
the strategy rather than removing it, and a sixth strategy breaks the repair.

The condition under which a fact set is stable is visible in what went wrong. Every verdict in this
unit is a case analysis over the four names, so each one is a statement of the form "given these four
rules, this quantity is a projection". That form cannot survive a fifth rule, because the projection
was a fact about the four.

**A fact set is closed under an open strategy set when every member is defined as a question the
strategy answers, and no member's definition mentions a strategy.** `16:161-165` already asks for
half of this, that the arity not change with the strategy, and gets it by fixing the arity at two;
the other half is that the fact set's members be questions rather than answers, and then the arity is
fixed for free.

Tested against this unit's cases:

- "what an operation lowers to" is a question. Survives.
- "how a run repeats" is a question. Survives.
- "the access width is `floor((W+6)/8)+1`, a function of `W` alone" is an answer computed over four
  rules, and section 4.2 shows it is already wrong for one of the four and differently wrong for a
  fifth. Does not survive.
- "the stride is `W` for `Cold` and `8 * size_of(carrier)` otherwise" is an answer. `p3` arm two
  shows what happens to it.

`49:304-314` reached the same shape independently and cold, in different vocabulary: the schema is
strategy-independent and only the values filling it vary, so a new strategy adds impl bodies rather
than fact slots. That file did not know the question had been named. This is now **two instances**,
one of them cold, and `49` is right that neither is the dispatch `48` asked for.

One thing I would add to `49`'s form. A schema of questions is stable against a new strategy adding
values, and it is not stable against a new strategy asking a question nobody had. `p3` arm four is
that case: a strategy that pads each element for an inline validity flag is not answering "how does a
run repeat" differently, it is introducing a per-element fact the numeral did not have. Whether that
is a new fact or a different answer to the stride question is a real boundary and I did not settle
it.

## 9. Bearing on the live options

Per `RULES.md:264-266`. I cite `OPTIONS.md` by section and quoted phrase, never by line, per my
brief.

**The derivation's outputs section.** *Kills its criterion, keeps its finding, and moves two of its
sub-claims.* The section's verdicts are adjudicated against `16`'s criterion, and section 2 shows
that criterion admits four counts across parameters it does not name. The two-output finding itself
does not rest on the criterion: `Cold`'s injectivity failure is an argument about a map, and it
stands. What the section should stop saying is that the criterion decided anything. Two sub-claims
move: the access width's dismissal was decided on the reading section 2.3 refutes and should be
reopened per section 4; and the entry's account of `47`'s two-ladder cost reverses per section 4.2.

**The proposed permanent sentence** (`47`'s, quoted in the register). *Fits well as one clause of
three, badly as the topic's sentence.* Section 3.3. `48`'s verdict on it is confirmed and its reason
is sharpened: the sentence is not incomplete, it is complete about a different question.

**The `Precise`-on-inexact open item.** *Narrows sharply.* Section 6. One arm of the fork is
measurable and is not a design anyone ships. The question that survives is whether the wide value a
multiply forms is carried between operations, and a reading of I2 nobody has used may change the
answer before it is asked.

**Q7, which carrier is the packing claim about.** *No new bearing, and one connection.* Section 5's
readout is a different bench from `26`/`27`'s and answers a different question (decode shape, not
packed against dense at a carrier width). I name the connection because both live in the same
directory and a reader who finds one should know the other is there.

**Q11, what the numeral guarantees to a fold.** *A small addition.* Section 3.2's locus clause gives
`35`'s accumulator reach a test it did not have: the reach is a fact about what a computation needs
to hold, and whether it is the strategy's rule or the composition's decides which artifact owns it. I
have not checked `35` and I am naming the connection rather than claiming it.

**Q5, Q6, Q10, Q12 through Q16.** *No bearing.*

## 10. What I would put to the earlier files, since they may be resumed

**To `16`, three things.** Your criterion at `16:100-101` is a fixpoint equation and I have solved it
exhaustively at `50_probes/p1`; do you accept that it admits several solutions, or is there a
selection principle in it I have missed? Your access-width closed form is exactly the worst case over
eight phases (checked, zero mismatches over 1024 widths) and a packed run at stride `W` reaches all
eight only when `W` is odd, which halves your 28-of-64 to 16-of-64; do you agree, and does the
dismissal survive once the exact quantity is keyed on the stride? And your `16:280-282` is the
argument the topic should have been using; would you restate it as "a fact belongs in the result when
obtaining it requires applying a rule the strategy owns", which is what I think it means and which
decides the cases the criterion does not?

**To `47`, two things.** Your `p6`'s two-ladder result uses `16`'s closed form, and recomputed from
the phase sets a packed run actually reaches, the access partition refines the native one and the
shared jump set is all four rather than empty. Does your section 5.1 reverse, and is the honest form
a fork (strategy-independent and six classes, against strategy-dependent, exact, and 25 classes in
one ladder)? And your section 4's result survives without your model, but by a different route than
`48` proposes: under the compute-type assignment the pair does separate, and what is unreachable is
the at-rest type, across the kind boundary. Does that change how you would word the register entry?

**To `45`, one thing.** Your `45:167-169` says refusal on inexact needs no extra compute width. Two
questions. Does forming the high half of a fixed-point product not already require a wider
intermediate under every strategy, so that the distinguishing fact is whether it is carried rather
than whether it is formed? And given `p6`'s admitted fractions, is the refuse-on-inexact reading a
design you would still put to op as one of two alternatives, or is the honest shape one reading with
a policy question attached?

**To `48`, one thing.** Your section 5's finding holds and your count under Reading B does not: the
criterion applied consistently under that reading demotes the carrier too and returns the empty set,
and honouring the kind boundary returns two facts that are not the unit's two. Does the finding read
better as "the two readings disagree on the members while agreeing on the arity"?

## 11. What the register should gain

I am not editing `OPTIONS.md`, `INTENTS.md` or `00_brief.md`, per my brief.

**A statement that the criterion does not decide the topic**, with `p1`'s grid, so no later file
adjudicates against it again. This is the single highest-value entry, because six files have used it.

**The site model as a declared premise**, in the words of section 3.1. It is already everyone's
assumption and it is written nowhere, which is what made one sentence readable two ways.

**A correction to the Reading B count**, per section 2.4, before the consolidation carries `48`'s
"one".

**A correction to the `47_probes/p2b` citation**, per section 2.5, and a pointer to `50_probes/p3`
for the construction the claim needs.

**The access-width entry reopened**, per section 4, with three numbers: the closed form is exact as a
worst case over eight phases, it over-estimates at 48 of 128 widths under `Cold`, and `16`'s 28-of-64
becomes 16-of-64.

**A reversal of the two-ladder cost**, per section 4.2, and the fork it becomes.

**A price on the decode-shape magnitude**, per section 5, with the note that slot 52 is redundant and
that this is the second instance of `RULES.md:337-357`'s failure.

**The `Precise` entry narrowed**, per section 6, including the I2 reading nobody has used.

**A note that `46` was recovered** at `7a3bddd` after four files had built on an uncommitted file, so
a later reader knows why its history begins where it does.

## 12. What I could not determine

**Whether the locus clause in section 3.2 is decidable in general.** It decides every case in this
unit and I could not construct a case where "a rule the strategy owns" is genuinely ambiguous, but I
also did not look very hard outside the container question. The place I expect it to strain is the
boundary `p3` arm four touches: a strategy that introduces a per-element fact rather than a different
answer to an existing question. If that is common, the clause fixes the arity per strategy rather
than globally, which is what `16:161-165` wants to avoid.

**Whether the fact set is stable under an open strategy set in the stronger sense.** Section 8's
condition is stable against new answers and I could not show it is stable against new questions.

**Whether `Precise` stores wider than `Warm`.** Section 6.4 reports that I2's words say it pays in
storage and that three files assumed otherwise. I have no evidence either way and it is op's.

**Whether `p1`'s derivability rules are the right ones.** They are cited and they are a model. In
particular I gave the site the rule that a packed stride below the carrier's width implies the stride
is the declared width, which is true of the four and false of the fifth strategy `p3` builds, and I
parameterised on exactly that. A reader who thinks a different rule set is faithful should rerun with
it; the file is thirty lines of rules and the enumeration is instant.

**Whether the `W > F` case changes `p6`'s equivalence theorem.** The theorem needs every operand's
valuation to be at most `F - 1`, which holds exactly when the total width equals the fraction width.
With integer bits present the implication breaks and the two refusal designs may separate. A design
keyed on total-and-fraction width would want that measured and I did not measure it.

**Whether `mock/benches/` holds anything else this unit has called unpriced.** I grepped the
directory listing and read one bench's findings and variant sources. I did not audit the other
thirty-odd bench names against the unit's open magnitudes, and after finding one instance in one
look, I would not assume there is only one.

**Whether `45_probes/p3` and `p6`'s numbers reproduce.** `48:574-576` names these as the most
quotable and least checked figures in the unit. Still true. I did not check them either.

## 13. Coverage, bounded honestly

**Read end to end, directly:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `16` in full, `48` in full,
`44` in full, `45` in full including its sections 11 and 12 reply, `46` in full, `47` in full, `49`
in full including its phase two.

**Read at the specific passages I cite, by opening the lines:** `OPTIONS.md` lines 690 to 880 (the
derivation's-outputs section, read last per my brief and never cited by line) and lines 316 to 400
(Q7, for context on the bench directory), `seed/SETTLED_strategy.md` lines 30 to 40 (I2's
establishing source), `DROPLIST.md` (grepped for the three-question shape and for its vocabulary,
found nothing, plus its three section headings).

**Every `file:line` in this document was opened and its content checked against my claim**, not
merely resolved. One of them changed what I wrote: I had `45:180-181` for the "no extra compute
width" quote, from my brief's neighbourhood rather than from the file, and it is `45:167-169`. I
corrected it here and in `p6`'s header.

**Probes of other members opened as source:** `47_probes/p2b_kind_asymmetry_positive.rs` in full,
which is section 2.5 and is the one that mattered.
`mock/benches/variants/bitpack-plan-naive/src/lib.rs`,
`mock/benches/variants/bitpack-plan-windowed/src/lib.rs`,
`mock/benches/variants/bitpack-plan-native/src/lib.rs` in full, which is section 5. **Not opened:**
every `16_probes` file, every `45_probes` file, `47_probes` other than `p2b`, `49_probes`. Where I
refer to those I rely on their authors' accounts, and the two places that matters are `16_probes/p4`'s
28-of-64 (which `p4` recomputes from `16`'s stated closed form and reproduces exactly, so the figure
is checked even though the file is not) and `47_probes/p6`'s jump points (which `p4` also recomputes
and reproduces exactly before correcting the phase set).

**Not read:** `02` through `15`, `17` through `43`, `PERSONA_CALLS.md`, `SETTLED.md`, `archive/`,
`seed/` beyond the one file cited, the closed predecessor panel. `15` in particular is not read, and
it is the file that established the two outputs alongside `16`; where the unit's finding rests on
`15` rather than on `16`, my sections 2 and 4 do not touch it and my account of it is `44`'s, `46`'s
and `47`'s.

**The specific risk in what remains.** Section 2 is one reader's formalization of one sentence, and
if a second reader finds a selection principle in `16:100-101` that I missed, sections 2.2 through
2.6 and most of section 3 move. That is the largest risk in this file, it is the same risk `48:578-580`
named about its own section 5, and the two of us have now reached the same conclusion by different
routes: `48` by reading two passages, me by solving the equation. Two instances, and both are reads
of one sentence, so it is one sentence wearing two hats until someone attacks the formalization
itself.

**No bench harness ran in this dispatch.** Section 5 reads committed harness output and runs nothing.
Every other magnitude this file touches is **unpriced**, and section 12's fifth item says I would not
assume that word is safe without looking.
