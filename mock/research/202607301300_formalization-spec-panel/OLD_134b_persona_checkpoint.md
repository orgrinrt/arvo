# Persona checkpoint: what does not need op, and what the panel does while he sleeps

**Date:** 2026-08-07
**Position:** after `134_leijen_the_cost_of_a_structural_magnitude.md`. Required reading with the standing
base until op replaces it.

**A persona stood in for op to write this. Op has not seen `131`, `132`, `133`, `134`, or this file. Nothing
here is ratification, nothing here is a call op reserved for himself, and every line below is agent output on
the suspect rung of the provenance ladder.** A later reader who cites this file as op's word is citing a
stand-in. Where I say a question is settled, I am reading a sentence op already wrote and saying which one;
where a question is genuinely his, I say so and stop.

The stretch since `130b` produced four good files and a growing open list, which is the wrong direction at the
point op asked for convergence (`127b:12-18`). Nine items were handed over as his. Four of them are not his,
and saying which is the useful half of this checkpoint. The other half is that the container fork is blocked
on him and the canon is not, and there is a great deal of canon work that does not touch the fork at all.

## What is settled by something op already wrote

### Precision is sign-free. This was never open.

`130:240` writes `const PRECISION: u32 = G::EXTRA + I + F`, folding the sign bit into a mathematical
coordinate, two lines after citing the sentence that forbids it. `129:521` does the same inside its signed
surface macro. The sentence is op's, at `30b:9`, carried at `110:869-873`: identity is parameterised in
mathematical coordinates rather than encoding coordinates, and "precision and the exponent bounds are
primitive; total width, the hidden bit, and field encoding are derived on the physical side". A sign bit is
field encoding. It is not a significand digit and it never was.

Two experts confirmed it independently and neither needed the ratified line to do it. `131` section 7 got
there from a compiled diagnostic, `132` section 7.2 from the arithmetic, and the arithmetic is the part worth
keeping: a two's-complement product of two n-bit signed values needs `2n - 1` bits because the product carries
one sign and the inputs carried two. A law written over stored widths says `16 + 16 == 32`, and it is wrong by
the one bit that crosses the sixty-four rung and doubles the container (`132:568-576`).

So this does not go to op. It goes back to the two files that contradicted him. `130:240` and `129:521` are
drift against a ratified line, they are corrected rather than escalated, and the correction is the one
`131:194-196` already wrote: `PRECISION` is `I + F`, `STORED_WIDTH` is `G::EXTRA + I + F`, and the product law
is sign-free and correct for both families at once.

Two experts agreeing is what the two-expert rule asks for and it is also the shape op has broken four times,
so I will name the shared premise rather than leave it: both experts assumed the design wants one number per
numeral that both the law and the ladder read. It does not, and that is the whole finding. Two coordinates and
a sign marker, with the precision and the stored width both derived, is the smallest set from which neither
consumer is wrong.

### The register container and the collection layout are already two members

`132:424-425` asks whether `110:3251`'s single `Lowering::Container` should become two, the register container
and the collection layout, and hands it to op as a change to a ratified structure.

It already is two. `110:3249`, one line above the line `132` cites, declares `type Layout: StorageLayout;
// {Dense, Bitpacked}, selects the container granularity`. Dense against bitpacked is exactly `132`'s second
question, what many values are in memory, as distinct from what one value is in a register. The trait table
has carried both since it was assembled.

**Declined, and no change is owed to the trait table.** What `132` section 5 actually produced is worth
keeping and is not a structural change: the two members have different mechanisms and different costs, the
first must be a Rust type settled at monomorphisation and the second is ordinary generic code, and the design
states that nowhere. That is a sentence for section 1.22, not a member for section 1.23.

I am being blunt about this because the pattern matters more than the item. A file read the line it needed and
not the line above it, then routed the result to op as a ratified-structure question. That is the second time
in this stretch a file has gone to op with something the record already answered, and `130` was the first:
`131:18-20` found that op's container ruling was op catching a regression against `110:3251` rather than op
overruling the panel, because nobody had quoted the line. Grep the trait table before proposing a member for
it.

### The surface arity is settled, and it is independent of the fork

`133:471-485` compiles a defaulted type parameter, `Fixed<const I, const F, S, C = <S as Store<I, F>>::T>`,
with no gates, giving three written parameters and a derived container, and observes that the wall is entirely
inside `Store`'s impls.

The load-bearing half of that is not the default. It is the second sentence: **whichever route wins for step
A, the surface does not move.** D48's arity is available on every route in the fork, gate-free, today. So the
surface arity is not a cost column, is not a tiebreaker, and should not appear in any further comparison table
as one. Take it off the fork.

### The defaulted container parameter is what the ruling forbids

`133:620-625` declines to read op's ruling either way, on the grounds that reading a ruling more narrowly than
it was written is how drift starts. That is right about the risk and it stops one step early, because reading
it more broadly is also a reading and one of the two is what he wrote.

What he wrote (`130b:41-44`): "Container naming is explicitly wrong. The entire idea of arvo is that the
strategy guides container selection, not the user." Not "a required container parameter is wrong". Explicitly
wrong. And the second half is the one that binds: "the same semantics and typestate will be used by other
optimisation steps, such as the already well designed hilavitkutin-build". A container a consumer pinned by
hand is a decision taken away from every later stage, and that is true of a pinned default exactly as it is
true of a pinned argument. A default that is never exercised changes nothing; a default that is exercised is
the thing he refused.

The toolbox rule does not rescue it, and `131:174-178` already said why: `arvo-toolbox-not-policer.md` exposes
choices the consumer knows the answer to, and its own list is workload shape, access pattern, perf budget,
semantic intent. Which machine integer holds a 16-bit Q13.3 is not on that list and cannot be, because the
consumer's answer to it is a function of the strategy they already wrote. `no-bare-primitives.md` says it in
as many words: consumers do not know and should not care which primitive the transparent repr lowers to.

**Declined. The escape hatch is not opened.** The residue worth carrying is that the mechanism now exists,
costs nothing, and can be opened later by one character, so nothing is foreclosed if op wants it. Opening it
is his and he has not been asked; not opening it is what his sentence already says.

### The deleted macro material does not need restoring, because nothing was deleted

`134:558-561` asks whether `48b`'s digit-emitting macro and `62b`'s proc-macro vehicle come back, on the
grounds that three deletions have now been found to have removed something nobody had priced.

The finding is right and the framing is wrong. `47`, `48b`, `58`, `61` and `62b` are in the archive, immutable,
and every one of them is readable today. What the deletion at `126` cost is not the content, it is the
citation: eighty-six files went past `48b:67-70`'s own flagged debt ("someone spends the twenty minutes before
we accept that the message stays in binary") without anyone spending them. That is not a restoration question,
it is `108b:22-32`'s archive-instrumented-for-error finding firing for the fourth time, and op has already
adopted the corrective.

**Not op's, and not a restoration.** Any dispatch on the structural family reads `47`, `48b`, `58`, `61` and
`62b` as required source before deriving anything, per `autonomous-overnight-work.md`'s mine-the-design-history
instruction. That is a dispatch instruction and it is mine to give.

### The second read op ordered on `126` is discharged, and nobody recorded it

`127b:63-66` ordered an independent second read on `126` before the container dispatch, because `126`
overturned a call two experts had converged on. The panel went to `128` and then to the container question, and
no file presents itself as that read.

Reading the record rather than dispatching again: the substance is discharged across two files that reached
`126`'s mechanism from their own probes. `129` attacked what the enumeration was buying and settled the
canonicity half from a different direction. `133:76-90` ran the carry-and-read discipline harder than `126`
did, on its own construction, and both reproduced it (a `type const` carrying a parameter reaches an array
length under `min_generic_const_args` alone, and chains through a second projection) and corrected the
statement of it, which is section 2 of that file.

So it is done, and the record does not say so, which is the same defect as the item above. Recorded here with
those citations. This is a question of fact about the archive, not a call.

## What is op's, one line each, and I am not touching any of them

1. **The blocking trade**, GCA plus `-Znext-solver=globally` against a structural magnitude. His, and see the
   next section for what changes about it.
2. **Whether the numeral's magnitude is a const or a type.** The encoding fork, which decides whether step A
   exists at all. His.
3. **Whether D48's literal spelling survives**, given that `134:462-469` reads `E0117` as saying the literal
   spelling and the absent cap cannot both be had structurally. His, and I have a challenge to the premise in
   the push-back section that should be settled before it reaches him.
4. **Where `-Znext-solver=globally` sits in `unstable-features.md`**, which has no tier for a `-Z` flag.
   His, and it is a workspace-rule change, which is not a panel act under any reading.
5. **Whether the numeral's base is his to decide at all.** I rule below that it is not, and if he disagrees
   the ruling reverts to him.

On item 4, four consecutive files have handed it back and each added a real input. That was correct four
times and it is waste the fifth. The inputs are now complete: `132` section 6 measured the compile cost at
nothing and the gate compatibility against every feature the named downstream consumer ships including full
`specialization`; `133` section 7 narrowed the exposure to consumer code that is itself width-generic and
measured a downstream typestate reader at zero. **Stop adding to it.** It is one line for op and the file that
adds a sixth input has spent a dispatch on a question that was ready.

## The base is not op's, and the answer is base ten if any structural route lives

`134:552-556` hands the numeral's base over as a third axis op has not seen. I am taking it back, and the
reason is the one `autonomous-overnight-work.md` states for the whole class: a fork that measurements decide
is not a taste call, and the design director is not the tiebreaker for a question that has numbers on both
sides.

It has numbers on every side and they point one way. Base ten prints the magnitude's digits in reading order
with no elision on the differing coordinate (`134:200-212`), is **strictly shorter** than binary at every
width and increasingly so (`134:342-345`), keeps the total ladder and the absent ceiling through a million
bits at 0.05 s (`134:347-356`), and costs 1.9x compile at an identical workload, 0.17 s against 0.09 s, which
`arvo-compile-time-last.md` names as the bucket to pour into rather than a cost to minimise. The 300 extra
impls are a table over **digits**, a domain of ten fixed by a design decision, which is not what `127b:36-50`
refused: op refused a table whose domain is a guess about which widths a consumer will want, and there is no
guess in a base.

**Ruled, conditionally and reversibly: if any structural route survives, it is base ten.** The condition is
op's, the ruling is not, and it costs nothing if the fork lands on GCA because the whole family goes with it.

One thing is owed before it is believed, and `134:570-574` names it against itself: sixty-four compositions
and four large magnitudes is a sample, the digit-pair table is generated, and a generator bug would be uniform
and invisible to a sample. That is `catalogue-edge-cases-as-tests.md`'s sampled-law failure exactly, and the
whole-matrix const assertion over digit pairs and carries is a precondition rather than a follow-up. It is
cheap. Whoever touches the structural route next discharges it first.

## What the panel does next, and none of it waits on the fork

Op added a requirement on the way to bed: the consolidation is promoted to canon whole and supersedes
everything before it, so it has to be sufficiently comprehensive. That changes what is worth doing tonight
more than the fork does.

An open fork is visible. A reader of the promoted canon sees a question with two priced answers and knows
what they do not have. **A gap is invisible.** A section that defines what a law is in full and never lists
one reads as complete, and the reader who takes it as canon does not know that a third of the design's
subject is missing from it. Under the promote-whole requirement the gaps outrank the fork, and three of them
are in the canon right now while the fork is not in the canon at all.

Consolidation stays held, per `127b:71-73`. The three dispatches below are what it absorbs when it unholds,
and they are ordered by how much of the promoted canon is missing without them.

### One. The law list, and it is not close

`110:1420-1430` opens the algebra section, defines a law completely (a claim that the terms of one grouping
class stand in a relation, under a stated view, over a value set, quantified over the class, keyed on every
parameter its proof used), establishes that every law has a unique finest view, and then **never enumerates a
single law**. Three files have now reported this, each as a measurement they could not take rather than as a
hole in the canon: `131:663-666`, `132:658`, `133:665`.

It is a hole in the canon and it is the biggest one. It blocks more than anything else on the open list. The
named-item diagnostic's price is parametric because of it, and op adopted that diagnostic at `130b:70-80` on
an "almost free" estimate he explicitly asked to be checked at the real count. The witness set costs two lines
per law and nobody can multiply. The Boolean-algebra law suite is already the largest owed item in the standing
base (`110:5620-5622`) and it is one family of a list nobody has written. The implementation phase's test list
is the law list, and the tautology checklist op is collecting at `108b:174-181` is what gets deleted against
it.

It has been dodged three times and not once because it was blocked. It was dodged because writing it is work
and reporting it is not, which is the defer instinct choosing the task list
(`prefer-hard-unblocking-work.md`).

**The artifact:** the enumeration, one row per law, each with its relation, its grouping class, its finest
view, its key, and whether it is derived or asserted. Not a count. The list.

### Two. The four families beyond fixed point, and their laws

`130` section 10 is the only place the design has four families interpreting one contract, it is one expert's
first read, it says so and asks for a second (`130:704-706`), and it is written with the container as a
written parameter, which op refused hours later. `131:866-868` flagged that it was never redone against the
projected container and did not redo it. The exponent form's own laws are open there.

Float and decimal are not an appendix. Section 1.16 is the float model and section 1.17 is radix ten, and 1.17
carries the most consequential sentence of its stretch, that arvo's decimal `Ranged` numerals deliver IEEE's
values and are not conformant to its preferred-exponent rules while decimal `Implicit` numerals are strictly
stronger than the standard (`110:2379-2381`). That is a claim a promoted canon makes to the world, and the
laws it rests on are not written.

This does not wait on the fork, and `133:568-573` is why: the downstream contract is stable across the
mechanism, because every fact a lowering layer needs is reachable as a value-position const rather than only
through the projection. So the families' interpretation of one contract can be settled now and survives
whichever way the container lands.

**The artifact:** the four families against the contract with the container derived rather than written, plus
the exponent form's laws, plus the second read `130` asked for on D53's expansion.

### Three. The conversion story, second read plus the relation nobody designed

Op withdrew a ratification of his own at `130b:11-30` and said in as many words that what replaces it is open
and is a real question rather than a gap, and that the design owes a statement of which conversions are
implicit and which are written. `131` section 5 answered it in one pass with five relations: identity,
inferred at the operation, `widen` as the only candidate for `From`, `rescale` as never implicit, and refused.
It reads complete, and it is one expert's first read on the replacement for an op withdrawal, which is the
worst provenance any canon section can have.

It also has a hole its own author named: narrowing the integer part is deliberately absent, because it is
lossy in a way the strategy has to adjudicate, and `131:625-628` leaves it undesigned and on the open list. A
conversion chapter missing the one lossy conversion is the chapter a consumer reaches for first.

**The artifact:** a second read on the five relations, plus narrowing designed rather than deferred, plus the
one sentence `131:570-573` proposes as the honest restatement of the withdrawn ratification, which is that two
numerals of equal precision have the same `Precision`, `Precision` is a type, and they are not the same
numeral. That sentence is a candidate for op and it should reach him written rather than as a question.

### And a correction the standing base needs regardless

`110:2501-2505` states that the decoder ring is a confirmed ceiling and that the one lever that moves it is
restating a comparison as a bound. `134` establishes that both halves are wrong: the ceiling is an artifact of
the numeral's base and a change of base removes it, and the bound lever fires only where the expected width is
fixed independently of the operation, which rules it out at exactly the common case (`134:168-178`). A face
layer was built downstream of the false ceiling and priced at `58:655-656` as doubling the relevant trait
surface.

That is a live defect in the standing base, not a nuance. It is also one expert's first read, so it does not
land on `134`'s word alone. **Fold the second read into dispatch one or two rather than dispatching an audit
for it.** Op's late-panel instruction is to converge and build on each other rather than lengthen a findings
list, and a member already reading the algebra can carry a paragraph correction without a dispatch of its own.

### What is not on this list, and why

Not the container fork. It is blocked on op, and adding a sixth file to it is the failure
`panels-argue-the-intent-not-the-wording.md` names: contributing when the instruction was to converge.

Not the `unstable-features.md` tier. It is prepared and it is one line of his.

Not the three named-and-unrun archive instruments at `110:5802-5815`. They are real and they find material,
and they are audit rather than construction. Under the promote-whole requirement a missing law list costs more
than an unaudited restoration ledger. They come after.

## What I would push back on

Op's move in this panel has been to ask why an assumption was there rather than whether an argument was sound,
and it has broken a converged conclusion four times. Read with that eye, seven things in this stretch look
reached more easily than they should have been. I cannot refute most of them and I am naming them anyway,
because the last four times the thing that broke was not something anyone could refute either.

### One. `E0117` is probably not the wall, and `134`'s own file contains the counter-shape

This is the sharpest and it is load-bearing, because `134:462-469` uses it to conclude that keeping D48's
literal spelling and having no cap requires the magnitude to be a const, which is the whole reason item 3 goes
to op as a fork.

The probe is `impl ToNat for Idx<14>` in a consumer crate: foreign trait, foreign type, `E0117`, correct. But
four hundred lines earlier the same file writes `<Idx<14> as ToNat<Marker>>::N` (`134:118`), a **parameterised**
`ToNat` carrying a second type. Rust's coherence rule admits `impl ForeignTrait<Local> for Foreign` when a
local type appears in the trait's parameter list with no generic parameter before it. So `impl ToNat<MyCrate>
for Idx<14>` is plausibly accepted in the consumer's crate, the bridge is consumer-extensible, the cap
disappears, and route B survives with D48's spelling intact.

I have not compiled it and I am not claiming it works. I am claiming that a load-bearing coherence finding was
reached in one pass, that the shape which would defeat it appears in the same file for an unrelated reason,
that nobody connected them, and that ten minutes settles it. That is precisely the arvo-premise failure of
2026-07-28 in miniature: a false premise inherited from one probe, propagated into a fork, and one grep away
from correction the whole time.

The escape may not be worth having even if it compiles, and the second read should price that too: the marker
drags into the projection, so a consumer would have to name it wherever the coordinate is projected, and a
surface that reads `UFixed<13, 3, Warm>` has nowhere to put it. If that kills it, the finding stands and it
stands on evidence rather than on one probe.

**Nothing goes to op on item 3 until this is compiled.**

### Two. "Fixable" and "false" are stronger than what `134` measured

`134:17` opens with "the diagnostic is fixable, and the premise `133` handed back is false". Its own table
three hundred lines later says something weaker: on the common case, a mistyped width, GCA prints
`expected 13, found 16` and base ten prints `expected N1<N3<End>>, found N1<N6<End>>` (`134:373`). The consumer
still does not read their own number back. `134:378-382` is honest about this and calls the residual one
lexical decode step, which is right and is a real improvement over evaluating a positional binary sum in your
head.

But the verdict sentence is what propagates. A reader six files from now will carry "the diagnostic objection
to the structural route was refuted", and it was **mitigated**, from unreadable to readable-with-a-convention,
while still losing to the route it competes with on the error a consumer hits most. Say mitigated. The word
matters because it is a cost column in a fork op is about to decide.

### Three. The step A and step B seam is carried as fact, and was flagged as premise-dependent once

`132` drew the seam, `133` confirmed it and relocated the purchase, `134`'s brief states it as established,
and `134:82-84` accepts it. That is four files agreeing, which is the shape.

`133:72-73` flagged it in one sentence: "The seam is real. Section 1 says why it is also, as drawn, a
consequence of a premise rather than a fact about the problem." Nobody carried that sentence. And `133`'s own
structural construction is the counterexample to it: under a structural magnitude the rung is a **pattern**
read off the digit count (`133:305-309`), there is no rung index computed, and there is therefore no step A
and no step B, only trait resolution. The seam is an artifact of const keying, which is exactly the premise
under dispute.

That does not make the seam useless; it is a good description of the const-keyed route and it correctly says
what a future stabilisation would relieve there. It makes it wrong to use as a neutral frame for comparing the
routes, which is what the fork table does.

### Four. Op's two calls are not in tension, and saying they are will be cited

`133:518-523` concludes that `130b:39-48` and `127b:41-44` are in tension for this derivation, that the
precedent asks for the arrow to be inverted while the container ruling forbids the one party who could invert
it, and that the tension is what makes step A hard.

They are not in tension. `Capacity` inverts the arrow because a dimension is something the consumer knows;
the container is something the consumer does not know, which is the entire content of `130b:41-44`. A
heuristic that does not reach a case is not a contradiction with a ruling, and `133:521-523` says as much two
sentences later by noting that `127b:41-44` was offered as a heuristic rather than a theorem. Having said it,
the file keeps the tension framing in its section heading and its conclusion.

The reason this matters is not tidiness. A sentence saying two of op's calls conflict will be picked up by a
later file as licence to choose between them, and neither needs choosing between. Delete the framing and keep
the finding, which is that four files reached for the precedent and it does not apply.

### Five. The flag's narrowing is unchecked in the one place it is cheapest to check

`133:575-579` narrows `131`'s "any consumer doing arithmetic" to "consumers whose own code is width-generic",
which is good news and is measured. `133:661-662` then records that whether hilavitkutin and vehje are
width-generic in that sense is unchecked.

They are not where I would look first. **arvo's own algorithm crates are width-generic by construction**:
`arvo-graph`, `arvo-sparse`, `arvo-comb` and `arvo-spectral` are specified to be generic over numeric trait
bounds and forbidden from importing `UFixed` directly. If instantiating one of those pulls the flag into a
consumer, the narrowing is much narrower than it reads, and the check is inside this repository rather than in
someone else's. Nobody asked.

That input belongs on op's `unstable-features.md` line before he draws it, and it is the only addition to that
question I would still accept.

### Six. The compile-time comparison separates encodings and nothing else

`134:327-340` measures base ten against binary at sixty-four compositions and applies
`arvo-compile-time-last.md` correctly to the 0.08 s difference. `86b:6-21` is op's own separation requirement:
a claim about a distinction is checked where that distinction is nonvacuous, and every model states what it
separates. This model separates two encodings at one workload size. It says nothing about scale, and
`134:588-589` records that every further magnitude operation is another hundred-impl table, which is a curve
nobody has sampled at two points.

One sentence of scope on the measurement, or one more workload size. Either is cheap and the figure is going
into a fork.

### Seven. Six consecutive test-gate declines, each citing the last

`129`, `130`, `131`, `132`, `133` and `134` all declined to run the suite, each citing `108b:174-181` and the
one before it, and all six rest on `126:47-48`'s claim that the tree has not moved. That claim was true when
`126` made it and it has been inherited five times without being checked. Op's ruling at `108b` was against
re-reporting the collected tautologies, not against measuring.

The decline is defensible: the panel produces canon, `mock/crates` is out of bounds, and re-running would
measure nothing new **if** the tree has not moved. One `git status` converts an inherited claim into a
measurement and costs a second. The seventh file runs it.

## Standing

Only op's calls are final, and by his own principle at `108b:11-20` they go stale when their evidence moves.
Nothing in this file is one of his. The four items I declined are declined by reading a sentence of his and
naming it; if he reads any of those sentences differently, his reading is the answer and mine was a stand-in's
guess at it.

The panel produces canon, not source: `mock/research/` and `mock/benches/` are its ground and `mock/crates` is
out of bounds until the canon is complete and earmarked as arvo's first full canon. Experts are dispatched one
at a time, never in parallel, and each reads the ones before it. The consolidation stays held per `127b:71-73`.

Five things wait for him, and four of them fit on one line each: the blocking trade, the encoding fork, D48's
spelling under whatever `E0117` turns out to say, and where a `-Z` flag sits in the workspace's feature rule.
The fifth is whether the base was his to decide, and if it was, I took it and he should take it back.
