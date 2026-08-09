# What the restoration actually carries, and what it only appears to carry

**Date:** 2026-08-05
**Position in the panel:** after `110_consolidation_eleven.md`, which named the missing check and asked
for it. This file is the third party file 110 says it lacks.

File 110 states its own limit at `110:4548-4551` and again at `110:4882-4884`: "file 109 audited the
compressions, this document restores from that audit, and no third party has yet audited the
restoration." That is the honest sentence in the document and it is the one worth acting on, so this
file performs the check it names. I read `110` in full, `109` in full, and went to source for every
restoration I report on, rather than comparing `110` against `109`'s description of the source.

The verdict, stated once and argued below. **The restorations are, with a small number of exceptions,
faithful, and two of the exceptions matter a great deal.** One of them makes a mathematical claim its
own cited example falsifies. One reinstates an objection to a held decision while omitting the
correction that answers it. Beyond those, the more consequential finding is structural and follows from
the method rather than from any author: **restoring from an audit bounds the restoration by the audit's
recall**, and `109`'s recall is demonstrably incomplete, so `110`'s completeness claim inherits a
ceiling it does not state.

The canon gate first, since it comes before the assigned work. `108b:190-193` places `mock/crates` out
of bounds and gives the panel `mock/research/` and `mock/benches/`. This file writes one document in
`mock/research/` and touches nothing else. **Nothing in `110` proposes a mechanism the ratified rung
forbids**, and its one aggressive act, superseding a figure inside `70b`-ratified text, is licensed
explicitly by `108b:11-20` and is performed at the point of exercise with the licence quoted
(`110:2129-2142`). Its second aggressive act, reopening the array grammar's forcing argument, is
**offered rather than adopted**, with the two-agreement requirement cited as the reason
(`110:2935-2936`). Both are correct handling. The gate passes.

---

## 1. The entailment check nobody had run

I worked from `109`'s citations and from `110`'s own restoration list at `110:4572-4623`, went to the
source range in each case, and asked one question: does the restored text entail what stood there. Two
failures are load-bearing, four are real but smaller, and the rest of what I checked holds.

### 1.1 The additive lattice closure condition is restored wrong, and its own parenthetical falsifies it

This is the worst thing in the document and it sits inside the restoration `110` flags as its flagship,
section 1.7, "the section the audit measures as the archive's worst stub chain" (`110:908-910`).

The source, compiled and twice independently confirmed. `33:265-267`:

> **Additive lattice closure holds exactly when `bias / adjustment` is an integer.** The shipped
> `AddClosed` gate on `Bias = Zero` (`26:326-331`) is the special case. There are numerals with nonzero
> bias that are additively closed and that the shipped gate would refuse.

And `33:666-670`, the same file's own summary, in symbols: "**Additive lattice closure** holds exactly
when `b/q` is an integer... **Narrowed-multiplicative lattice closure** holds exactly when `q` and `b`
are both integers and `q` divides `b^2 - b`." Both checked against direct exhaustive computation in
both directions at `33_probes/probe_5`, and rebuilt fresh at `44:150-165`, which quotes the additive
condition verbatim precisely because a reader of `40` could misread its English.

`110:956-964` restores it as:

> Additive lattice closure holds exactly when **bias and adjustment are integers** (the shipped
> `AddClosed` gate on `Bias = Zero` is the special case); narrowed-multiplicative closure additionally
> needs the adjustment and bias both integers

Three things are wrong at once, and they compound.

**The condition is wrong.** `b/q` an integer is not `b` and `q` integers. A numeral with `q = 1/2` and
`b = 1/2` has `b/q = 1` and is additively closed under the compiled predicate; `110`'s wording reports
it as not closed. The direction of the error is the one that matters least for soundness and most for
the design's stated purpose: it under-admits, so it would refuse legal MATLAB numerictypes, which is the
standard's own test at section 0.1 failing on the same axis `40`'s droplist already records `Bias`
failing once (`110:4296-4297`, "`Bias` as a plain signed integer: made a legal MATLAB numerictype
unrepresentable (slope 1, bias 1/2)").

**The parenthetical it carries falsifies it.** The shipped gate is `impl<N: Numeral<Bias = Zero>>
AddClosed for N {}`. It constrains the bias and says nothing about the adjustment. Under `b/q` integer,
`Bias = Zero` is the special case exactly as both source statements say. Under `110`'s wording, the
shipped gate would be admitting fractional-adjustment numerals that the stated condition excludes, so
the sentence and the example inside it cannot both be true.

**The following clause goes vacuous.** "Narrowed-multiplicative closure **additionally** needs the
adjustment and bias both integers" adds nothing to a condition that already required both. In the
source the "additionally" is doing real work, because the additive condition is about the ratio and the
multiplicative one is about the terms, and `33:669-670` carries a third conjunct (`q` divides `b^2 - b`)
that `40:291-293` had already dropped and `110` inherits the loss of.

Two readings survive on how this happened and I cannot distinguish them from the record. Under the
first it is a transcription slip: `40:291-293` writes "bias/adjustment is an integer", the solidus reads
as "or", and the restoration disambiguated in the direction that makes the sentence agree with the
clause after it. Under the second it is a reading, in which case the reading is refuted by
`33_probes/probe_5` and by `44:154-160`, which quotes the correct form specifically to stop this. What
distinguishes them is whether the author saw `33` at all; `110`'s citation for the paragraph is
`40:288-294`, which is the compression, not the source. That points at the first reading, and it points
at a method problem rather than a comprehension problem: **the restoration went to the last document
that carried the sentence rather than to the document that established it.**

The repair is one clause and needs no design work: restore `bias / adjustment is an integer`, keep the
`AddClosed` parenthetical, and restore the third conjunct from `33:669-670` so the multiplicative
condition is complete. `44:150-165` should be cited alongside `33`, because it is the file that
re-verified the formula and explicitly warned about this English.

### 1.2 D39's objection is restored and D39's answer is not

`110:835-839` restores D38 verbatim, including the ten-member vocabulary, "shipped even if nothing uses
them, vocabulary fixed by mathematics", which is `109`'s R4 and is exactly right. In the same
blockquote it carries:

> **D39 is held, not overturned**, at op's seventh checkpoint (`30b`), despite two readings finding its
> stated mechanism does not compile and that a membership predicate over the whole ambient set is
> vacuously true of everything.

That sentence is `40:210-212` faithfully. What `110` does not carry is `40:227-231`, twenty lines further
down the same section, which answers the second objection:

> The topic's own text states the precision two panel members had missed: **the predicate is *inhabits*,
> not *equals*,** so `Inhabits<Real>` being vacuously true of every finite numeral is a correct fact
> about the top of the tower and not a refutation of the predicate.

`Inhabits` returns zero hits in `110`, re-run fresh for this file; the correction survives only in `27`,
`40` and `49`, and left the standing base at the fifth consolidation.

So a reader of `110` alone is told that op holds a decision against which two readings stand, and is not
told that one of the two was answered five consolidations ago by reading op's own topic file more
carefully. **That is a restoration that reinstates a defect and omits its repair**, and it is the
precise shape `109` was commissioned to find. It is worse than a plain drop because the objection is
carried with the authority of a verbatim quotation of op's own call, and the item is number sixteen on
`110`'s own loudest-for-op list (`110:3769-3770`), so it is being handed to op in that condition.

The repair is one sentence in section 1.6, and `49` still carries the wording.

### 1.3 A refutation stated as having two compiled supports, of which one is stated

`110:733-737` says a tie is reachable only at an even radix, "as one of the two compiled supports under
the transfer-argument refutation", and `110:462-464` repeats the phrase. The second support is nowhere
in the document. It is at `68:451-455`:

> absorption-freedom (for all nonzero `y`, `quantise(x + y) != x`) is exhaustively TRUE at exponent span
> `p` and FALSE at span `p + 1`, with the precision, the code and the bans all held fixed: `EMAX` moved
> by one and the property's truth value moved with it.

`absorption`, `EMAX moved`, `truth value moved` all return zero hits in `110`. So does the four-legs
analysis at `68:441-446` that the refutation rests on (leg one parametricity, which the bans enforce;
leg three width-uniformity, named unproved in `10` and never proved since). `parametricity` and `four
legs` likewise return zero.

This matters beyond bookkeeping. The absorption-freedom result is the sharper of the two, because it is
the one where the outer quantifier fails **with the bans in force**, which is the whole content of the
claim that implementation uniformity does not give property uniformity. And item 13 on `110`'s own
loudest-for-op list is the three `unstable-features.md` rule-wording edits, which `110` calls "the
largest single item on the list" (`110:3760-3764`). The first of the three is the last-sentence
correction. **Its evidence is these two counterexamples, and the standing base now states one of them
and gestures at the other.** Op is being asked to edit a ratified workspace rule on half the evidence
the panel produced for it.

### 1.4 Losses inside the restored transfer-ground section

The four-ground table at `110:344-349` and the `unargued` honesty rule at `110:351-352` are verbatim
from `68:461-469`. The worked `Ranged` coordinates at `110:357-367` lose four things against
`68:473-488`:

The two negative controls keep their existence and lose their figures ("a window-only shift disagrees on
8 of 13 values; adding a nonzero additive constant to the value map breaks the symmetry on 29 of 51
checks", `68:476-478`). A restored measurement without its numbers is a restored assertion.

"Two of the six coordinates collapse into one: only the span matters, not the absolute position of the
window" (`68:480-481`) is absent. That sentence is what makes the index set six coordinates rather than
seven and is load-bearing for anyone re-running the transfer argument.

The observation that the sixth consolidation's own models "cleared by luck rather than by design, since
nothing told their authors what the threshold was" (`68:484-486`) is absent. That is a statement about
the reliability of the review's own prior work, which is the class of statement this whole exercise
exists to preserve.

And `ffl` is dropped from the five-ground table's `physical grounds` row (`63:447` lists `pin`, `host`,
`flags`, `model`, `ffl`; `110:328` lists the first four). The drop is probably correct, since `68`'s own
section heading says the transfer scheme replaces what `ffl` was credited with, and `109:398` flagged
`ffl` as used-and-undefined. But `110`'s own rule is that section 6 is the cumulative diff, and a
silent removal from a table being restored under a completeness claim is the thing the rule forbids.
**One droplist line closes it.**

### 1.5 Thread B is restored at its costs and not at its results

`110:4103-4131` restores Thread B's reframe, its three costs, and the `ConstantTime`-keyed-on-the-wrong-
thing finding. `110:4581-4582` claims exactly that and no more, so the section-8 accounting is honest.
The section-5 text is not, because it presents itself as the thread's state and the thread's state
included two positive results that are now in one file in the corpus:

`11:509-515`, that one generic arithmetic body **can** serve both a total and a fallible composition
without duplication, provided the resolution rule constructs its own answer, together with the orphan-rule
reason the panel initially mistook for a fundamental limit. `constructs its own answer` returns hits in
`11` alone.

`11:517-520`, that with two range positions the return type any composition needs is the join of the two
resolutions' own carrier choices connected by a lift, "the same shape effect systems use for combining
independently-installed handlers", and that it scales to a third or fourth effect without redesign.
`independently-installed handlers` returns hits in `11` alone.

A future member opening Thread B from `110` reads three costs and no mechanism, and rebuilds both. That
is the same shape as the demand-driven clause `109:44-55` costs at forty-five files, and it is inside an
item `110` restored.

### 1.6 What I checked and found faithful

Reported at the same weight as the failures, because a restoration audit that only reports failures
tells you nothing about the document.

Both preset tables (`110:2075-2107` against `78:409-441`) are cell-for-cell identical, and the
surrounding derivations are restored close to verbatim. This is `109`'s R1 and it is discharged.

The `tree-meaning` prohibition (`110:408-409` against `78:368-370`) is verbatim, in its own paragraph,
with the motivating defect and the mechanical test. R2 discharged.

D38's vocabulary (`110:835-837` against `40:209-212`), the membership licence's `Specials = None` gate
(`110:847` against `58:266-267`), preset divergence with op's instruction intact (`110:2186-2192` against
`40:693-696`), op's standard restored from `13c:12-14` rather than from `40`'s compression, and both
`16d` directives quoted verbatim at `16d:14-15` and `16d:45-47`: all faithful. R4, R5, R6, R10, R11
discharged.

The cost model (`110:2502-2540` against `58:896-934`) carries all six rows, the cliff, the
numerator-dominates finding and the scope paragraph, and adds a correct clarification distinguishing
composition pricing from declaration pricing. Section 1.7's algebra restoration is otherwise faithful to
`40:243-327`, including the `IS_EXACT`/`Total<Op>` correction and the finest-view mechanism's price,
with two legitimate updates where later files moved the ground.

R7 is discharged mechanically and it is worth showing, since it was a count of zeros. Citations in
`110` to the previously-uncited op checkpoints: `04b` five, `06b` four, `08b` one, `12b` one, `13b` one,
`16b` one, `16c` one, `17b` one, `24b` one, `13c` seven, `16d` eleven. Every early checkpoint is back in
the standing base.

And two gates reproduce exactly, run for this file. `cargo test --offline --workspace` over
`mock/Cargo.toml` returns **155 result lines, 672 passed, 0 failed, 9 ignored**, matching `110:4774-4776`
to the unit. Both canon-gate greps at `110:4770-4772` exit 1 empty. The two shipped-source facts I
spot-checked in section 1.29 are exact: the overlap deferral is at `arvo/src/bitfield.rs:28-30` in those
words, and `_BOUNDS` is declared at `:377` and mentioned only at `:393` and `:399`.

---

## 2. What is established, what is stated, and what is neither

The panel has built a provenance apparatus and `110` restores it in full, which makes it fair to turn it
on `110` itself. I sort its claims into four bins. The first three are all legitimate; the fourth is the
one that does not announce itself.

**Performed and reported here.** The two canon-gate greps, the test gate, the freshly-performed searches
at `110:4794-4800`, the three requirement performances at `110:4809-4874`, and the table-diff, whose
three caught corrections are named individually at `110:4869-4874`. I reproduced two of these
independently and both hold. This bin is small and `110` does not pretend it is larger.

**Cited to something performed elsewhere.** Nearly every number in the document: the 509,660,160-instance
symmetry check, the 41,380,159-operation binary32 sweep, the 254,830,080-instance `mulnum` check, the
923 assertions, the 318,126 parse strings, the 5,184 triples, the 1,596 bitfield shapes, the 65,536
container values, the 131,072-case rank-N sweep, every instruction count, every millisecond. `110` states
the boundary plainly at `110:4877-4884` ("it is not a fresh re-read of every probe any of them
produced"), which is the correct disclosure and is more than any predecessor gave.

**Resting on an argument.** The lowering-authorship residue and its deletion test, the layer-keying
rule's dual failure, the composite-of-placements algebra, the variety-closure ground for the truth
contract, the three-rung predicate ladder. Most of these were walked individually by op at `108b`, so
they sit on the ratified rung as adopted, with the arguments in the member files. That is the correct
resting place and it is legible.

**Resting on nothing but having been written down repeatedly.** Six that I can name, and the point of
naming them is that each reads exactly like the bins above.

**The spine rule's eleven firings.** `110:176-184` states eleven and draws the design's strongest
methodological conclusion from the number: "Eleven independent firings of one rule across unrelated
quantities... is evidence the rule is a property of this design's shape rather than a coincidence noticed
repeatedly." The eleven has never been enumerated. Two are op's at `44b`, two are the capacity-shaped
pair with probes named, and the middle seven arrive as "seven through the eighth consolidation
(`68:98-101`)", where `68:98-101` in fact reads "**Nine occurrences stand from the sixth consolidation's
count**", which is itself a count inherited from a document whose section 1 is stubs. The five
categories offered in place of a list ("grade projections, notation faces, seal witnesses, container
widths, text and byte capacities") are five, not eleven.

The sharpness of this is internal. At `110:1232-1233`, the same document writes: "**Six named sealed
carriers, not a count** (`68:344-354` against `78`'s count with no names): the six are enumerated above
rather than tallied, **because a count cannot be checked and a list can**." And at `110:436-437` it
restores the `90b` discipline that "a count in a member file names the command that produced it, adopted
after three consecutive files published a count nobody re-derived." The document states the rule twice
and applies it to the seal and not to its own first design rule, a thousand lines apart. The repair is
an enumeration or a downgrade of the conclusion; it is not a defence.

**"The ten axes."** Quantified over twice as a closed set (`110:4092`, `110:4124`, the second being the
load-bearing "delivery decides it, and delivery is not one of the ten axes") and enumerated nowhere.
The axis table with instances lives at `11:156-181` and has been out of every consolidation since `26`
carried names only. `110:4709-4713` disposes of the table's **dead** instances correctly (`Widening`'s
three, `Growth`'s two, `LogicalWidth`, `Underflow`'s `Unbounded`/`Flushed`, `Narrowed<W, A>`) and never
restores the live ones.

**The `Resolution` axis's four members.** Invoked at `110:1607-1608` as "the `Resolution` axis's own
domain, best read as a totalisation axis whose four members are four ways of making an otherwise partial
operation total, ordered by how much they lie", and at `110:1415` as the design's single home for
resolution policy. The four are never listed. `SubstituteZero` appears twice, both inside droplist
entries; `ReduceModulo` appears in table cells; the ordering "by how much they lie" is stated and its
order is not.

**`Quantisation`.** The sole content of `Policy` in the ratified trait table (`110:2449-2451`) and the
subject of `110:1178`'s "`Policy` carries `Quantisation` alone". Six occurrences, all as a name. Its
declaration is at `11:212-216`.

**`Direction`.** Two occurrences, one in the law key (`110:942`) and one in the section heading about
when it enters (`110:956`). A reader cannot learn from `110` what its members are, which matters
because `TowardNegative`, `ToEven`, `ToOdd` and `TowardPositive` all appear elsewhere in the document as
cells of other tables without being connected to it.

**The transfer refutation's second support**, section 1.3 above.

The mechanism that lets all six coexist with a performed definitional-completeness line is one word.
`110:4837` closes that performance with "**No term in this document's own new prose** is left undefined
or uncited." The line itself, at `110:271-272`, says: "every term in its definition, including the name
being defined, is either defined or named open **in the ratifying text**." The ratifying text is the
consolidation, not the subset of it the author wrote fresh. **Scoping the performance to new prose
exempts exactly the population a restoration consists of**, which is why a document that restores eight
thousand words can pass a completeness line and still leave its own trait table's only associated type
undefined.

I do not think this was deliberate, and I do not think it is hard to fix. Drop "new" and re-run the
performance over the whole document, and the six above surface in an afternoon.

---

## 3. The audit's recall is the restoration's ceiling, and it is not stated

`110:4705-4706` says: "**Every item on file 109's ratified-or-op-authored list is restored**, and the
remainder is not." I checked and that is true. It is also a smaller claim than it reads as, because
`109`'s list is eleven items that `109` found, and `109` found them by diffing consolidations against
each other. Anything that never reached a consolidation at all is outside both instruments. `110` names
its own instrument honestly at `110:4548-4551` and does not draw this consequence anywhere.

`110` itself demonstrates the gap once, at `110:4658-4669`, where it finds the eighth consolidation's
open item 12 (the structural array construction as a recorded fallback) absent from nine and ten with no
entry, and notes "The audit's `78` to `91` pair list does not contain it." Worth recording alongside:
**op had already named that item** at `108b:22-26`, "an item filed as an open question for op was carried
by consolidation eight and dropped silently by nine and ten while a sibling item from the same list is
still reported twenty-eight files later." `110` presents it as its own discovery and does not cite
`108b` there. Under-crediting op is the harmless direction, but it is evidence about assembly: at that
point the document was working `109`'s list rather than `108b`'s text.

Five more items in the same class, found while checking the restorations rather than by searching for
them. Each is on the ratified rung or addressed to op, each is in one or two files, and none is on
`109`'s list or in `110`.

**Op's own statement of how the review runs, and its stopping condition.** `13c:38-53`, verbatim: "Don't
poll this. I will literally say when we are done. This current one should be another deep dive like the
prior ten... Then after that, we again consolidate and start a new fresh eyes based on that, do another
10 or so experts focusing on another area, and we do this until our very design is both concrete, valid
and critically, ideal, optimal, the dream achieved, nothing less will we stop for." With the cycle read
out into four steps, of which the third is "**A fresh read.** A member who is given only the
consolidation, with the transcripts withheld, so the next area is chosen by someone not carrying the
last one's assumptions." `fresh read`, `transcripts withheld`, `Don't poll` and `deep dive` all return
**zero hits in every consolidation, including `110`**.

That is `13c` restored at its standard and dropped at its mode, in a file titled "the standard and the
mode", by a document whose central property is the one op's third step exists to consume. The fresh-read
step is the acceptance test for a standalone consolidation, and it has never been in the standing base
to be run.

**Op's checkpoint cadence.** `04b:19-20` and `04b:42-43`: "Op takes a checkpoint like this one after
every two experts", op verbatim, "Let's get similar checkpoint with me every 2 experts, too." Zero hits
in every consolidation. Between `86b` and `108b` there are twenty-one numbered files and four persona
checkpoints. I am not in a position to say whether op relaxed this in conversation, and the record does
not show it either way, which is the point: **the instruction is on the ratified rung and the standing
base has never carried it, so nobody could notice the drift against it.**

**Op's licence to argue against a ratified call.** `04b:72-74`: "Any member is free to argue against any
of them, including the ones this file has just reaffirmed, **provided the argument is made rather than
asserted**." `110` carries the first half ("only op's calls are final, and even those go stale") at
`110:126` and not the second. The second is what makes section 1.27's reopening legitimate rather than
presumptuous, and the section hedges heavily without it.

**An op adoption on fidelity.** `17b:19-30` records "**Adopted.** ... a fidelity grant is checked rather
than asserted, on the same footing as the recovery map that the earlier verification thread ended up
witnessing", with the compiled asymmetry that motivated it. In `110`, `fidelity` appears only in two
droplist entries and in the not-restored list. The mechanism that was tried is droplisted correctly; the
op adoption it was trying to satisfy is neither restored nor droplisted nor named open. It should be one
or the other.

**A proposal to op about a ratified workspace rule.** `63:456-460`: the `unstable-features.md` width
ceiling rests on the quadrupling and the refusal, which are structural, while "only the specific
wall-clock figure (28.45 seconds) is one machine's measurement through one harness; the proposal to op is
to mark it as such wherever quoted." `28.45` occurs in twelve panel files and in **no consolidation after
the sixth**, and the workspace rule quotes the figure today as a durable fact. That is a fourth
`unstable-features.md` wording item, and `110`'s loudest list names three.

And one that is not a drop but a live ambiguity `110` created by restoring two spellings of one
constant without reconciling them. `110:506-508` restores the UNORM8 worked example with "an adjustment
factor of `256/255`"; `110:3038` states the same fix as "`Adjustment = 1/(r^F - 1)`". Both are correct
under different exponent conventions (`e = -F` and `e = 0` respectively) and `110` states neither
convention. Under its own widened line, "a name defined twice with different content is defined nowhere"
(`110:274-276`), and `FullRange`'s survival as a named `Adjustment` constructor is on the open list at
`110:4072`, so the two spellings are live rather than historical. One clause naming the exponent closes
it.

**Two readings on what to do with all of this, and the evidence does not force one.** Under the first,
these are six more items for a twelfth restoration and the method is sound. Under the second, six items
found by one reader in one pass, none of them by searching, is evidence that the population outside the
consolidation chain is large, which is `108b:24-25`'s own second adopted part ("a file stating a general
mechanism inside a specific finding flags it for absorption") aimed at exactly this population, and
`109:643-646` says so as well: "It did not check the probe directories or the member files for material
that no consolidation ever absorbed... it is larger than this one." What distinguishes the readings is
one dispatch that sweeps the member files rather than the consolidations, and I would run it before
earmarking anything.

---

## 4. Is this a canon

The question as posed to me presumes the earmarking is due now. `108b:184-186` does not:

> **Order of work.** Consolidation eleven, then close the queue. It absorbs this walk, the archive repair
> and the drop audit, and the remaining stretch works the open list down rather than opening ground.

and `108b:192-193` gates source work "until the canon is **complete** and earmarked as arvo's first full
canon". So op's own sequence is consolidation eleven, then the queue, then completeness, then the
earmark. On the ratified rung the earmark is not this document's to receive yet, and I would answer the
question in that order rather than around it.

**On the property `110` asks to be judged on**, at `110:29-35`: "A reader can reconstruct the design from
this file alone, without opening any prior consolidation. That is the whole point of the exercise, and it
is the one measurable property this document should be judged on."

Measured, the answer is **not yet, and by a small margin**. The document is transformed. Fourteen
content-free subsections have content; there are no pointer chains; the droplist is cumulative for the
first time since the fourth consolidation; every early op checkpoint is cited; the two ratified preset
tables exist as markdown in a standing base again. Against `102` this is not an increment, and the
format change at `110:4541-4551` is the right fix stated in the right terms.

But four of the design's own vocabulary items are used and not defined in it: `Quantisation`, the
`Resolution` members, `Direction`, and "the ten axes". A reader cannot reconstruct `Policy` from this
file, and `Policy` is one of the three contracts in the ratified trait table. That is a bounded, cheap,
named gap rather than a structural failure, and it is roughly one section's work.

**So: yes with conditions, and the conditions are short.** I would put five things in front of the
earmark, in this order.

1. **The two entailment failures repaired**: the additive closure condition restored from `33:265-267`
   with its third conjunct, and D39's `inhabits`-not-`equals` correction restored from `40:227-231`.
   These are two sentences and they are the only things I found that make the standing base say something
   false.
2. **The definitional-completeness line re-performed with "new" struck**, and the four undefined axis
   terms given their content. The axis table at `11:156-181` is the source for the live half.
3. **The spine rule's eleven either enumerated or the conclusion downgraded**, applying the document's own
   count-versus-list sentence at `110:1232-1233` to its own first design rule.
4. **One sweep of the member files and probe directories for material no consolidation ever absorbed**,
   which `109:643-646` names as owed and larger than its own list, and which section 3 above gives six
   samples of. This is the only one of the five that is a dispatch rather than an edit.
5. **The transfer refutation's second support restored** from `68:451-455`, before op is asked to edit
   `unstable-features.md` on it.

None of those is a design question. All five are the document saying what it already knows.

**What is genuinely undecided against what is decided but unwritten**, since the question asks for the
distinction. Reading `110`'s open list at `110:3909-4161` and its loudest-for-op list at
`110:3733-3779`, the genuinely undecided population is small and well-named: the array grammar's third
column, the truth contract's reduction spelling and `negative_impls` adoption, the FLX reinstatement,
division's grading axis, `Hot`'s default float environment, the uniform-sampling spec, D39's scoping and
the seven upper vocabulary members, the platform crate's name. Nine items, each one line for op, each
with the readings carried symmetrically. That is a design in good order.

The decided-but-unwritten population is the four vocabulary items above plus the six drops in section 3.
It is larger than it looks from inside the document and smaller than it looks from outside, and it is
entirely mechanical.

**One thing I want to say plainly, because a report that only sorts is not worth the dispatch.** The
format change is correct and it is not sufficient on its own, and the two entailment failures show why.
"A consolidation states its own content or dies" prevents a stub. It does not prevent a paraphrase from
weakening a claim, and both failures I found are paraphrases produced while restoring, from the
document that last carried the sentence rather than from the document that established it. The rule that
would have caught both is one line longer: **a restoration cites the source that established the
statement, not the last document that carried it, and where the two differ the restoration says so.**
`110` does this correctly in several places (section 0.1 goes to `13c` rather than to `40:602-605`;
section 1.21 transcribes `78:409-441` cell by cell and says so at `110:4869-4871`) and not in others
(section 1.7's closure sentence cites `40:288-294`; the transfer coordinates cite `68` and lose what `68`
cites). It is the same discipline as the deletion test, applied to provenance rather than to grounds.

---

## 5. The downstream contract for the check that is now owed

`110` names the residual and this file discharges one pass of it. What the next one needs is not more
diligence from its author, for the same reason `109:608-610` gives about the format, so here is the
mechanism rather than the exhortation.

**What the checker reads.** A restoration ledger, one row per restored item, carried in the
consolidation's section 8 in place of the current prose list. Three fields: the source range that
**established** the statement (not the last carrier), the section of the consolidation that now carries
it, and the author's own one-line statement of what the restoration is claimed to entail. `110`'s
section 8 already has the second and third informally; adding the first is what makes the row checkable
without a second full read of the archive.

**What the checker can determine from that alone.** Whether the restored text entails the source, by
opening two line ranges. That is the entire check, it is mechanical per row, and at `110`'s scale it is
roughly one hundred and twenty rows, which is a dispatch rather than a project. It is what I did by hand
this pass, and the hand part was finding the ranges, not comparing them.

**What the design needs back from the checker.** A verdict per row in three values only: *entails*,
*loses content* with the lost sentence quoted, *contradicts* with both texts quoted. Nothing else, and
specifically not a redesign, because a checker that proposes is a checker that has stopped checking. Two
of my six rows would have come back *contradicts* and four *loses content*, and the two contradictions
are the whole value of the pass.

**Where this genuinely stops.** The check is bounded by the ledger, so it cannot find material no
consolidation ever carried. That population needs the different instrument `109:643-646` names, and no
amount of entailment checking substitutes for it. Saying so is not a limitation of the ledger; it is the
ledger's perimeter, and a perimeter stated is worth more than one discovered.

---

## 6. What I did not check

Stated because the honest count matters more than a clean claim, and because this file's own negatives
are the weakest part of it.

I checked the eleven ratified items on `109`'s list, sections 1.7, 1.24, 1.21 and 1.5's restorations
against source, the four citation defects `110` reports at `110:4625-4654`, and the two gates. **I did
not** re-verify sections 1.12, 1.13, 1.14, 1.16, 1.22, 1.26, 1.27, 1.29 or 1.30 against their sources;
those are largely `108b`-ratified this stretch and their sources are recent, which lowers the drop risk
and does not remove it. I did not open a single probe directory. I did not verify any measurement.

My universal negatives were run fresh over the panel directory's `.md` files on 2026-08-05 and are
`inhabits`, `absorption`, `parametricity`, `collapse into one`, `by luck`, `8 of 13`, `29 of 51`,
`fresh read`, `transcripts withheld`, `every two experts`, `28.45`, `lint-escape`, `addressable-count`,
`constructs its own answer`, `independently-installed handlers`, `partial associativity`, `fidelity`,
`ten axes`, `Quantisation`, `Direction`. **They verify that these terms are absent, not that the ideas
are.** `110:4802-4807` states this limit better than I can and this stretch has now produced five
demonstrations that a grep's vocabulary is what fails, so I will not pretend mine is the exception.

One place I know my terms could be wrong. On **partial associativity**, adopted at `17b:44-51` ("the
design does not name it, and no standard vocabulary carried in the spec covers it"), my search finds it
in `18`, `26`, `33` and `17b` and in no consolidation after the second. Two readings survive. Under the
first, the finest-view lattice supersedes it outright, since `Precise` below interior safety sits at a
point that "preserves values and events while losing definedness" (`110:928-930`), which is the same
fact wearing the lattice's clothes, and the correct disposition is a droplist entry saying so. Under the
second, op adopted a **name** for a gap and the name is gone while the gap is described, which is a drop.
I lean to the first and I am not confident, and either way the disposition is one line and there is
currently none.

*Grounded on: ratified (`108b:11-20` the re-derivation licence this file relies on, `108b:184-193` the
order of work and the standing, `13c`, `16d`, `04b`, `17b`, `44b`, `70b` read at source), settled shapes
(`109` in full, `110` in full, `33:263-272`, `33:664-672`, `44:150-165`, `40:207-243`, `40:243-330`,
`58:888-936`, `63:437-475`, `68:425-509`, `78:336-465`, `11:156-181`, `11:444-560`), verified at source
(`arvo/src/bitfield.rs:26-32,377,393,399`), performed for this file (`cargo test --offline --workspace`
over `mock/Cargo.toml`: 155 result lines, 672 passed, 0 failed, 9 ignored; both canon-gate greps exit 1
empty; twenty fresh negative searches over the panel directory on 2026-08-05).*
