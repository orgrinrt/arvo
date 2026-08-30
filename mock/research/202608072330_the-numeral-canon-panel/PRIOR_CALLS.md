# Prior calls: the historical log of the lead designer's decisions

**A reference layer, not a register of authority.** This file collects op's prior design calls from
arvo's whole history: 91 design rounds back to 2026-04-19, and the closed formalization panel at
`mock/research/202607301300_formalization-spec-panel/`. Every call in it belongs to a lineage that
ended in failure: the crate tree those calls produced was nuked on 2026-08-08
(`mock/design_rounds/202608082157/202608082156_topic.nuking-the-tree-for-the-canon.md`), and the
canon this panel is writing replaces everything they built.

## The status of everything below, in op's own words

Quoted from the dispatch that commissioned this file:

> So all my prior calls can be mined and should be collected for reference, but not as calls, not as
> ratified intents, but as historical log of my calls, explicitly connected to a *failure* which means
> they aren't "canon" so to say. All of them I've made in earnest, all of them made sense at the time,
> but none of them relate to this new panel or its convergence or settled intents, and should not act
> as if it did. They should be added as an extra layer of reference, but nothing more, and mostly its
> usefulness is giving experts ideas what to test (why did they make that call, and does it hold here?
> Etc) and explore, and also, for you and the orgrinrt persona especially, gives material to learn my
> preference, taste, gut instincts and intuition by inference. But again, none of it absolute. The
> answers are likely wrong, and the questions they answer, are also probably wrong. So substance
> itself is only good as extra stuff to consider or explore, nothing more. The flavor and intuition
> inferred can feed my persona and also feed exploration and testing though.

Four consequences govern every entry:

1. **Nothing here is a call.** Not ratified, not canon, not a tiebreaker. The panel's live surfaces
   are `INTENTS.md`, `OPTIONS.md`, `DROPLIST.md`; conflicts resolve there, never here.
2. **The questions are as suspect as the answers.** Each entry states which question the call was
   answering, because a faithful record of a good answer to a badly framed question has recorded a
   badly framed question. Several of the sharpest reversals below happened exactly when a question's
   framing broke, not when an answer did.
3. **Made in earnest, sensible at the time.** This is not a catalogue of errors and does not read as
   one. The register is factual: what was decided, what it answered, what it rested on.
4. **The useful residue is the reason, the constraint, and the taste**, not the conclusion. A call
   often encodes a real worry that still holds after its conclusion dies.

### And the gap to the current intents is narrower than "dead against binding"

Op has since demoted his own voice across this panel's transcripts and checkpoints too, reserving
absoluteness for roughly two abstract intents. Quoted from the same dispatch channel:

> I have demoted my voice in all but the instances where, in this panels transcripts and checkpoints
> or other files, I explicitly frame something I say as absolute. Not many have I done so, I remember
> only two, and both of those were very abstract intents only, not specific calls of concrete things

`INTENTS.md` agrees from its own side: no entry there currently holds RATIFIED, all twelve are
STATED, and it carries his standing instruction that nothing about them is absolute. So the
difference between this file and `INTENTS.md` is **currency and lineage, not rung**: the intents are
stated direction inside the live panel; these are stated direction inside a lineage that failed. The
demotion recorded here is not an unusual act performed once on this history. It is the same
discipline he now applies generally, applied backwards.

That cuts one way worth naming: if almost nothing he has said is a ruling, then what this corpus
reliably shows is precisely what he asked to have inferred from it: direction, preference, instinct,
and what he consistently reaches for. Section 8 carries that inference and is weighted accordingly.

## How to read an entry

Each entry carries: **the call** (what was decided), **answering** (the question it was posed
against, which is often the more suspect half), **provenance** (op verbatim, op decision as recorded
by an agent, or an agent derivation under explicit delegation, with the file), and **register** (which
live `OPTIONS.md` question it bears on, whether it revives anything on `DROPLIST.md`, or whether it
touches nothing the panel is currently asking).

### The D-number hazard, which has already burned readers

The corpus numbers decisions D1 through D75, and **a bare D-number is ambiguous**. Three collisions
are known:

- The restructure round (`mock/design_rounds/202607300800/`) and the formalization talk
  (`mock/design_rounds/202608082157/202607301100_topic.the-formalization-talk.md`) share one
  numbering run (D1 to D52, then D53 to D75), **but** the talk file's own open-questions table uses
  IDs `D1` to `D3` for question rows (`:72-74`) in the same file that records decisions D53 to D75.
- The predicate round reused `D18` (`202607290200`, Warm/Precise band) after `202607282100` had a D1
  to D4 of its own, and `202607290500` patched around a collision with `D18b`.
- The closed panel's own audit found 28 of the 47 D-numbers in the two register topic files absent
  from its standing base by number, and two colliding `D1`-`D4` sequences in one frozen file
  (`mock/research/202607301300_formalization-spec-panel/112_the_op_material_sweep.md`, section 6.1).

**Every D-number below is round-qualified.** Unqualified numbers in other documents should be treated
as unresolved references until qualified.

### The attribution-marker hazard

Where op's words and an agent's annotation sit adjacent in these files, they are separated only by a
blockquote marker, and annotation has repeatedly been promoted into op's voice by readers in a
sibling repo. Every quote below was checked against its marker at source. Where a "decision" is an
agent's derivation under delegation, the source says so and so does the entry; the corpus is unusually
disciplined about this from 2026-07-28 onward (closing provenance notes per file, "the most suspect
thing in this file" self-markings), and that discipline is itself one of the taste findings.

---

## 1. The corpus and its eras

**Era one, 2026-04-19 to 2026-07-27: the silent era.** Roughly 85 rounds. The "Decision (op)"
convention did not exist; decisions lived in changelists and topic files without attribution
markers. A grep across all of them for op-voice markers surfaces almost nothing explicit. Two
survivals are entered below (section 7). The absence is itself a finding: **the original design this
whole corpus descends from, the four strategy markers, `UFixed`/`IFixed`, the container projection,
carries no recorded op ratification anywhere in the round files.** Op's own later gloss on that
design, from the current panel (`INTENTS.md` I1): "what my amateur ass had written for arvo that we
are now redesigning (infer from that what you will..)". So the deepest layer of the failed lineage
was op's authorship without op's process, and everything after 2026-07-28 is, in part, the process
arriving.

**Era two, 2026-07-28 to 2026-07-30: the restructure round** (`mock/design_rounds/202607300800/`,
consolidated into
`mock/design_rounds/202608082157/202607301000_topic.inherited-state-from-the-formalization-round.md`).
Decisions D1 to D52. Opened by a trust collapse (the forbidden-feature discovery, section 5 below)
and characterised by rapid per-file op calls on crate placement, naming, and the first formalization
proposal.

**Era three, 2026-07-30: the formalization talk**
(`mock/design_rounds/202608082157/202607301100_topic.the-formalization-talk.md`, 1848 lines).
Decisions D53 to D75 in a single sitting: the three-contract spine, the axes, the presets redefined
from intent. The densest single day of op calls in the corpus, and the day with the most in-sitting
reversals (section 6).

**Era four, 2026-07-30 to 2026-08-05: the closed formalization panel**
(`mock/research/202607301300_formalization-spec-panel/`). Twenty-three files record op directly
(census: `OLD_112_the_op_material_sweep.md` section 1); nine more are persona stand-ins he later walked
individually. Op's material here divides into design ratifications (checkpoints seven onward) and
posture, standard and process statements (the eleven earlier files), and the panel's own audit found
the second kind was systematically the kind that dropped out of circulation.

**The boundary of this file:** the current panel's op files (`01`, `04`, `28`, `32`, `34`, `36`
through `39` in this directory) are live material, carried by `INTENTS.md` and `OPTIONS.md`, and are
**not** entered here. This file ends where the failure ends.

---

## 2. The strategy axis and the presets

### The standard the axis was held to

**The call.** Design questions are not answered by op's preference per question. The standard is
fixed: optimal and ideal rather than adequate; representative of the mathematics; and capable of
representing MATLAB, IEEE 754, SystemC and the rest, as a test rather than an inspiration. "The
abstractions are what truly matter, the typestate."
**Answering:** where should the algebraic laws sit (op refused the question as not his).
**Provenance:** op verbatim,
`mock/research/202607301300_formalization-spec-panel/13c_op_the_standard_and_the_mode.md`.
**Register:** bears on Q1 (the representability half of his acceptance criterion has this as its
ancestor) and on every expressibility question; see section 9 for the in-flight caution.

### Strategy is a mapping of mathematical intent (restructure D34)

**The call.** What a marker does at any width follows from what the marker promises, not from what
currently compiles or which document is newer. Warm promises headroom, so Warm and Precise carry
2x-logical at every width; the concrete wide-bucket sizing is the agent's derivation from that
principle and is marked so at source.
**Answering:** what do Warm and Precise resolve to between 65 and 128 bits, where source, diagnostic
and design document all disagreed (three artifacts, no two agreeing, no test in any direction).
**Provenance:** op decision, `mock/design_rounds/202607300800/202607291800_topic.the-last-four-calls.md`
heading D34; the band conflict described in
`202607300800/202607300100_topic.warm-and-precise-above-64-bits.md`.
**Register:** the principle (intent defines the marker) survives as `INTENTS.md` I2 through I8 in
restated form; the mechanism is dead with the tree. Bears on Q5 and Q13.

### The presets redefined from intent, and the two rows that had to die (talk D70, D71; panel 68b, 70b)

**The call, in stages.** D71 (talk `:1683`) redefined all four presets from what their names promise,
spread across six axes, every pair differing in at least two cells; D70 (`:1648`) made
`Deterministic` a derived marker over the composition. The panel then found two of D71's four rows
were justified from shipped doc comments rather than from the design; op voided them ("we are fully
free to restructure the strategies and their meanings",
`OLD_68b_op_checkpoint_sixteen.md`) and stated Warm and Cold fresh:

> For warm, I think we should assume that it'll work the same as writing regular old floats would
> work. ... The intuition is that it works and behaves as f32 and f64 etc in rust today without any
> framework on top of it.

> It should be something between warm and precise. Cold also tells us it's seldom computed or used,
> it's on a cold path. It can take more cost than warm, but shouldn't just be precise in disguise.

The re-derived tables were ratified at `OLD_70b_op_checkpoint_seventeen.md` (fixed-point and float
separately, the float table newly derived, `Warm`'s stored width diverging between the two kinds of
number).
**Answering:** what does each preset concretely mean, cell by cell. The question assumed a closed set
of four, which op has since opened (`INTENTS.md` I1), so the question itself is now marked wrong.
**Provenance:** op decisions and op verbatim as cited; the voiding at `68b` is op verbatim.
**Register:** Q6 (the ratified table gave Warm the clamp and Hot alone the wrap; `OPTIONS.md` Q6
already records this), Q14 (the exchange-rate question is op's own and unset in his sentences),
Q13. The Warm and Cold statements are restated in `INTENTS.md` I3, I4, I6 with newer wording; where
they differ, the newer wording governs.

### The far point, and the refusal that was declined (panel 74b)

**The call.** The far point is the supremum of a numeral's ordered representable values; the
infinity case, the fixed-point clamp case and the no-infinity case are one rule. The alternative,
refusing such a numeral at declaration, was declined with teeth: it would forbid `Cold` with `E4M3`,
"exactly that format's silicon deployment profile, and it crosses the warn-never-police line."
**Answering:** what do Warm and Cold do out of range on a float numeral whose specials carry no
infinity. Op gave an instinct first ("Option 2 as my instinct, stress tested and evaluated by an
expert still, before locking", `70b`), and the instinct survived its stress test and generalised.
**Provenance:** op decision, `OLD_74b_op_checkpoint_eighteen.md`.
**Register:** touches nothing the panel currently asks by number; the instinct-then-stress-test
shape feeds section 8.

### Bitpacking: two instincts, one correction, one price rewritten (panel 74b, 77b, 82b, 108b)

**The calls.** At `74b` op leaned to `Layout::Bitpacked` carrying two instances so the cost confines
to Cold, explicitly flagging his own uncertainty ("This requires someone more versed in the compute
side theory to confirm"). The compute-side dispatch replaced that: one meaning, zero inter-value
padding, byte-aligned slots being what `Dense` already does at a narrow width, ratified at `77b`
with the note that op's own second sentence had been the right one. The measured price then fell
from 4.6x to about 1.50x when the fourth design rule (everything derivable at compile time moves
there) was applied to the decoder (`82b`); at `108b` the footprint bench stayed preliminary on op's
word: "Confirm the digest, keep pushing the bench," with multi-column contention named as the
measurement Cold's intent actually waits on.
**Answering:** what bitpacked storage means and costs. The cost question was repeatedly found to be
measuring the wrong thing (L1-resident sizes, standalone probes), which is why its numbers moved.
**Provenance:** op calls as cited, with the delegation language his own.
**Register:** Q7 directly (the current panel's contention run at `27` is the successor of the `108b`
instruction). The corrected-price episode feeds section 8 (a scoped bench measures a scope).

### Determinism was never a gap (restructure D49)

**The call.** Deterministic reproducibility is the property the design already guarantees, not an
unnamed axis: "fixed point is reached for precisely because it gives accuracy together with
demonstrable, proven determinism that floats cannot provide." Two consumers on different strategies
disagreeing was never the promise. Constant-time execution explicitly left untouched by the
correction.
**Answering:** a research pass's framing of determinism as a vocabulary gap. Op rejected the framing;
the file records the rejection as the substance.
**Provenance:** op correction,
`mock/design_rounds/202607300800/202607292300_topic.the-axes-want-research-and-the-ladder-follows-theory.md`
(D49).
**Register:** bears on Q13's observable-axis classification; the reject-the-framing move feeds
section 8.

---

## 3. The spine: what the number is against how it is computed

The formalization talk built the three-contract spine in one sitting, correcting itself repeatedly as
it went. The whole arc is dead as mechanism; its residue is the sorting test and the naming
corrections.

### Identity splits from policy, with a test (talk D54)

**The call.** "What the number *is* does not change through strategies." An axis is identity if
changing it changes the set of representable values, policy if the same values remain and only the
arithmetic differs. Every axis sorted cleanly under it; underflow landed on identity because Flocq's
three underflow regimes give three different value sets, which is why overflow (policy) and underflow
(identity) are not two ends of one axis.
**Answering:** whether every axis hangs on one widened `HasAxes` (the direction the walk had been
heading three messages earlier, corrected by this call).
**Provenance:** op decision, talk `:334-360`; the underflow application at `:734-755`.
**Register:** Q5 (one axis or two), Q15 (resolution order), Q18 (identity against realisation is
this same cut asked about number systems). The *test* is the durable part.

### Numeral, Policy, Lowering, and one type (talk D55, D58, D59, D60)

**The calls.** The identity contract is `Numeral`, the policy contract `Policy`, the type
`Number<N: Numeral, S>` with `S` implementing both non-identity contracts directly, no bundle trait,
no projection hop (D55 `:392`, D59 `:580`). The third contract began as `Container` (D58 `:539`,
argued from IEEE 754's own format-against-encoding split, one numeral with two encodings in the
decimal formats) and was renamed `Lowering` within the sitting (D60 `:606`) when D59 made the marker
the implementor and "Warm is a container" read as false.
**Answering:** what the composition's shape is, after the walk restarted because `Num<...>` had
appeared in options without ever being agreed ("asks op to decorate a room in a building nobody
approved", `:81-93`).
**Provenance:** op decisions as cited.
**Register:** Q2, Q8, Q9 (the public spelling constraint), Q16 (which sense "composition" carries).
The restart-at-the-composition episode feeds section 8: op does not answer parameter questions on an
unratified type.

### The quantum thread: three corrections in one sitting (talk D57, D61, D62, D64, D65, D68)

**The arc, compressed.** "Scale" was renamed to the field's own **quantum** (with the sources
carried in the file); the exponent method moved onto the quantum bound (D57 `:466`); the quantum
decomposed as adjustment times a power of the radix, making dyadic membership a structural condition
rather than an asserted claim (D61 `:714`); a `Quantum` *type* was rejected because "a quantum was
never a type" and the member became `Quantisation` (D62 `:895`, explicitly "overturnable in a word");
the word was then found to be sitting one contract too early, since the field's quantizer is the
whole five-situation landing map, so `Quantisation` moved to the policy axis (D64 `:1387`) and the
numeral member was re-derived as the exponent's representation, `Implicit<E>` against
`Stored<BITS, U>` (D65 `:1394`); and D68 (`:1529`) amended D65 with flat four members
(`ExponentForm`, `Adjustment`, `Bias`, `Sign`) after the UNORM counterexample showed a bias cannot
express an adjustment.
**Answering:** in order: what the scale parameter is, whether a numeral is its own quantum, what the
numeral's member is called, and whether flat or nested members. Each correction retired its
predecessor's question as wrongly framed, which is the cleanest worked example in the corpus of the
questions being as suspect as the answers.
**Provenance:** op decisions throughout, with the reversal of D62 put to op as a reversal
(`:1277-1299`) because D62 was his.
**Register:** Q2 (coordinates), Q4 (what a datum stands for), Q10 (shapes denoting the same value
set). The naming discipline feeds section 8.

### The landing vocabulary (talk D63, D64)

**The call.** Rounding is a triple over `Direction` with IEEE's own member names (D63 `:1128`);
rounding and overflow are the in-range and out-of-range halves of one map over five situations, with
`Direction` a subtrait of `Resolution` so the meaningless combinations are unspellable (D64).
Marker types beat const enums here for exactly one reason: a const enum cannot be
blanket-implemented over, and the one-line `Monotone` blanket was the demonstration.
**Answering:** what the rounding instance set is (question A3 of the talk's own table), and why
overflow looked like it needed a different vocabulary (it did not; the round's own Layer 3 had
already said so and the walk had undone it without noticing).
**Provenance:** op decisions; the triple-table derivation is the file's, the calls his.
**Register:** Q5; the "the field's own word is direction" move feeds section 8.

### The ten-axis ratification and its overturn (talk D69; panel 30b)

**The call, and the reversal.** D69 (talk `:1621`) ratified ten axes across three contracts with
`LogicalWidth` primitive on `Numeral`, marked at source "overturnable if the reasoning does not
hold". It did not hold: at `OLD_30b_op_checkpoint_seven.md` the identity contract was re-parameterised in
**mathematical coordinates** (precision and exponent bounds primitive; total width, hidden bit and
encoding derived), on two independent readings, with the recorded moral that "the off-by-one against
real hardware formats was never a gap to patch. It was the parameterisation reporting that it was
pointed the wrong way."
**Answering:** is the axis set complete and is the logical width primitive. The overturn answered a
different and better question: which coordinate system the identity is stated in.
**Provenance:** op decision and op overturn as cited.
**Register:** Q2 descends directly from the overturn (the current panel's total-and-fraction answer
is the same move one step further). Also bears on Q5, Q15.

### What left the table afterwards (panel 39b, 44b)

**The calls.** `Widening` left `Lowering` and `Growth` left the law key, ratified only after op
gated the ratification on his own question ("if we lose widening and growth, do we still retain the
behavior therein, so the strategies make sense?") and after an intent check he ordered found the two
table-changing files had never once tested against MATLAB, IEEE 754 or SystemC. The value-unique
encoding replaced the width chain at `44b` after its known defect (rational bias) was repaired and
priced. Division was held at `44b` and confirmed at `108b` in the three-clause exact-quotient form,
with `Hot`'s cell going to the consumer because three independent inventions existed with no
derivation among them.
**Answering:** whether the axis table survives its own redundancies.
**Provenance:** op calls, `OLD_39b_op_checkpoint_nine.md`, `OLD_44b_op_checkpoint_ten.md`,
`OLD_108b_op_checkpoint_twentysix.md`.
**Register:** Q5, Q15; the gating question and the intent check feed section 8.

---

## 4. One type, aliases, and the public spelling

### One numeric type, every family an alias (talk D53); the format unifies (restructure D50)

**The calls.** There is one numeric type and `UFixed`, `IFixed`, `FastFloat`, `StrictFloat` become
four names for four compositions (D53, talk `:326`). Underneath it, D50
(`202607300800/202607300400_topic.the-formats-named-and-the-taxonomy-revisited.md`) had adopted the
Flocq reading: fixed point and float are one thing at different exponent functions, and `arvo-float`
survives as packaging only, its founding reasoning (D29's "not the same kind of thing") explicitly
replaced while its placement stood.
**Answering:** whether the four families are four types. The float wrappers' own inconsistency was
the lever: `FastFloat`/`StrictFloat` spell as two types the distinction `Warm`/`Precise` spells as
one type with two markers, and unifying "deletes a special case rather than adding a mechanism"
(talk `:283-314`).
**Provenance:** op decisions as cited.
**Register:** Q8 (op's current instinct is one family, explicitly not to be acted on; this is the
prior call that instinct descends from, and testing whether its reasons hold is exactly the use op
named for this file). Also Q3 (no mixed-numeral addition existed anywhere in this lineage either).

### The spelling is load-bearing (restructure D31, D48; talk's standing table)

**The calls.** The four ergonomic aliases survive a deletion proposal despite near-zero use: "they
were written for a reason", and a surface being unexercised "says more about how young the consumer
set is than about whether the surface is wanted" (D31,
`202607300800/202607291720_topic.the-aliases-are-kept.md`). Width stays a const parameter publicly;
the typestate form is internal (D48, `202607292300`). `UFixed<13, 3, Warm>` and `Uint<13>` must keep
reading as themselves through every restructure; the talk carried both as fixed constraints
(`:653`).
**Answering:** whether internal migrations may charge call sites.
**Provenance:** op decisions.
**Register:** Q9 (the width-surface crossing is this constraint's live successor), Q28.

### Compositions are public (restructure D52; talk D66, D67)

**The calls.** Compositions are public and bindable by anyone; semantic names and presets are the
default documented path, not the only path, with `arvo-toolbox-not-policer` cited as deciding it
(D52). Vendor mode names ship as optional feature-gated alias sets, off by default (D66, talk
`:1402`), generalised by D67 (`:1459`) into a standing principle with a falsifiable test attached:
**if a convention's mode cannot be written as an alias over arvo's abstraction, the abstraction is
not general enough.** The test caught its first gap in the same file (MATLAB slope-bias not
expressible, forcing the bias member).
**Answering:** whether arvo seals its internals, and what "can represent the standards" means
operationally.
**Provenance:** op decisions.
**Register:** bears on Q1 (see section 9) and Q16; D67's test is one of the most re-testable items
in this file, since the current panel's format unit asks the same adequacy question with different
machinery.

---

## 5. Expressibility, the forbidden features, and the trust collapse

### The forbid, and the audit it triggered (restructure D1 to D4 of `202607282100`)

**The calls.** `generic_const_exprs` is FORBIDDEN, same reasoning as full `specialization`: a sound
minimally-scoped successor exists, so the full feature is never enabled. The standing gate predates
this workspace, settled during loimu-era vetting: allowed only if not proven unsound or unstable
and, absent a very strong reason, itself on the stabilisation path. The container projection
migrates to typestate (the same cure `Capacity` already proved). Every remaining forbidden gate
goes. And the vetting record itself became suspect: "trust in the rule corpus has dropped to zero",
op having been told at each turn the forbidden features were fine, with a line-by-line audit of
every rule surface ordered, "no greps, no skips".
**Answering:** how a research pass came to be framed on the false claim that arvo depends on GCE
pervasively. The discovery was that the claim had been true, was fixed, and the record had rewritten
the forbid into a WATCH.
**Provenance:** op decisions,
`mock/design_rounds/202607300800/202607282100_topic.remove-the-forbidden-feature-gates.md`.
**Register:** `DROPLIST.md` carries the const-generic width-comparison closure from the current
panel's own probes, consistent with this; Q28 bears. The trust-collapse response (audit everything,
line by line) feeds section 8.

### Capacity: op's own antidote, defended and then reframed (panel 74b, 77b, 108b)

**The calls.** The capacity-as-a-type migration was op's own escape from GCE, and when the panel's
carrier unification threatened to undo it, op's condition was precise: "Unify. But I would still
keep the semantic alias of a Capacity ... everything following the same typestate explicitness and
clever design to get all of our contracts expressed without forbidden features is a net win in my
books" (`74b`). At `77b` he reframed the concept past the question he had been asked: "Capacity
simply denotes a fixed length. It's not a numeral itself. But it contains a numeral that expresses
this length. ... it is also the same as infinity on infinite number sets, and the lastmost number in
finite sets." The is-a answer survived re-derivation against that wording at `108b`.
**Answering:** whether two type-level naturals that differ only in arrival date should unify. Op's
reframing replaced the mechanism question with a concept question, and the concept question was the
better one.
**Provenance:** op verbatim as cited.
**Register:** Q26 (platform-width types), Q28. The reframe-the-question move feeds section 8.

### The pricing pillar (panel 77b)

**The call, verbatim because the wording is load-bearing:**

> Compile time is nothing. That can be literal minutes for all we care ... We *want* long compile
> times, if it resolves to snappy optimal runtime with the extra soundness, safety and numeric
> machinery amortized fully at compile.

> it's always amortize runtime cost in compile, const time, absolutely always, no matter the
> strategy ... NEVER do any strategy defer the cost to runtime that it can avoid!

A strategy marker changes what happens at runtime; it never changes how much is amortised at compile
time. Adopted as the fourth design rule, and one file later it produced the bitpack price
correction.
**Answering:** whether compile-time cost may decide a design fork (the audit found it had not in
substance, only in wording).
**Provenance:** op verbatim, `OLD_77b_op_checkpoint_nineteen.md`.
**Register:** Q24 (does the canon speak about cost), Q32 (what workload evidence prices anything);
also section 9, second question.

---

## 6. Algebra, laws, predicates, number systems

### Derived against asserted (predicate round D16), and everything it licensed

**The call.** A derived property is computed from the thing itself, cannot lie, and is a plain safe
impl. An asserted property is a promise whose falsehood returns a wrong answer, and asserting one is
an `unsafe impl` carrying a stated contract, because from the call site the two are
indistinguishable and the distinction must live at the declaration.
**Answering:** whether typestate properties on predicates are admissible at all.
**Provenance:** op decision,
`mock/design_rounds/202607300800/202607290200_topic.the-predicate-decisions.md` heading D16.
**Register:** this is the most-cited prior call in the corpus. It licensed D51 (laws derived by
blanket impls, never declared per type), D61 (membership structural rather than asserted), D70
(`Deterministic` derived), and the talk's second steer ("the typestate proof was sitting in comments
rather than in machinery", `:999-1009`). Bears on Q25 (law inventory form) and Q30 (admission as
predicate). If any single prior reason deserves independent re-testing against the new panel's
shape, it is this one.

### The ladder goes as deep as the theory (restructure D47), and the laws that refused to ladder

**The calls.** The algebraic ladder goes as deep as the theory does, not as deep as a named consumer
forces, with NumHask's collapse and alga's succession rejected as decision inputs ("facts about
those projects' machinery, not measurements of what arvo can do"), and an obligation attached:
every rung sketched and benched (D47, `202607292300`). `Combine<Op>` is `Magma<Op>` and the ladder
is named in full, declaring free because mathematics fixed the vocabulary, implementing gated by
D47's obligation (talk D75 `:1802`). Against this, the panel's algebra dive found the ladder was
not the algebra arvo needs (max-plus recurrences, distributivity over maximum,
`OLD_13b_op_checkpoint_five.md`), and partial associativity was adopted by name at `17b` and later
superseded in substance by the view lattice with its disposition never recorded
(`OLD_112_the_op_material_sweep.md` section 3.10).
**Answering:** how deep to name structure ahead of use. Note the tension the corpus never resolved:
D47's "as deep as theory" and the panel's finding that the theory's own ladder was the wrong shape
for the actual workloads. Both are prior; neither governs.
**Provenance:** op decisions as cited.
**Register:** Q11, Q12, Q25 directly. The current panel's law-layer work (`42`) is the successor of
exactly this unresolved tension.

### Parallel associativity stays in arvo (panel 13b)

**The call, verbatim:** "if the parallel associativity is part of the numeric substrate here, it
belongs in arvo, but perhaps a separate place. Not hilavitkutin. I would guess other downstream
users would seek for that in arvo too and make use of it, without having to pull in hilavitkutin."
File 13 then independently removed the argument for moving it: the regrouping pays at a single
thread, inside the unrolled accumulator, so the law belongs where the reordering happens.
**Answering:** file 12's proposal to relocate associativity to the engine layer.
**Provenance:** op verbatim, `OLD_13b_op_checkpoint_five.md`.
**Register:** Q11, Q12 (what a numeral guarantees a fold is this question re-posed inside the new
panel's vocabulary).

### Number systems ship ahead of use (restructure D38, D39; panel 30b)

**The calls.** arvo gets the whole membership family, `Natural` through p-adics, shipping even if
nothing uses them, **explicitly the opposite of the anti-speculation default applied elsewhere in the
same round**, for a stated reason: the vocabulary is fixed by mathematics, cannot be got wrong in a
way that later needs undoing, and retrofitting after consumers improvise costs more (D38,
`202607300800/202607291900_topic.the-number-systems-crate.md`). Membership is defined through
algebraic structure, `Real` as a complete ordered field (D39). The predicate is **inhabits, not
equals**. At `30b` D39 was **held rather than overturned** despite two readings finding its stated
mechanism does not compile: "a decision should not be withdrawn on the strength of two findings that
agree only on a negative."
**Answering:** what to do with the unclaimed `Natural`, which turned out not to be a placement
question at all.
**Provenance:** op decisions; the hold at `30b` is op's.
**Register:** Q20, Q21, Q30, Q31 are all successors. The held-not-overturned shape, and the recorded
reason, feed section 8.

### UNORM, NaN, and encodings as unnamed parameters (restructure D36, D30)

**The calls.** UNORM showed the identity impl was incomplete: `UFixed<0, F>` having no multiplicative
identity "is actually a fact about one encoding of that type", so the encoding is a property the
type never carried, and fixing that is what places UNORM (D36, `202607291800`). NaN is a typestate
question and the `ConstSign` carve-out is a workaround for a property never given a type (D30,
`202607291710`, explicitly a direction rather than a decision, with the file warning that building a
crate around a workaround is how the workaround becomes permanent).
**Answering:** where to file a colour encoding; whether floats can have ordering. Both answers moved
the question: from placement to parameterisation.
**Provenance:** op decisions/observations as marked at source.
**Register:** Q4, Q10, Q18. The pattern (an "exception" is usually a parameter nobody named) is one
of the strongest recurring moves in the corpus and feeds section 8.

---

## 7. Placement calls, and the two era-one survivals

### The placement reasons, kept for their reasons

Every crate placement of the restructure is dead with the tree. The reasons recur and are the
useful residue; the sources are
`mock/design_rounds/202607300800/` topic files and the talk (D72 `:1723`, D73 `:1758`).

| Prior call | The reason that outlived it |
|---|---|
| D2, D3: four new crates, facade terminal, `arvo-ndim` dropped | a name that abbreviates where no sibling does is wrong; the facade became load-bearing standing in for a crate never created |
| D5, D9: the hlist extracted to notko as `notko-hlist` | a pattern reinvented per repo is formalised once, at the lowest layer that can hold it; orphan rules decide the workable shape |
| D6: `Cardinal`, not `Countable`, not `Natural` | the type IS the count, so an -able suffix inverts meaning; a name claiming a mathematical primitive belongs to the mathematics layer, so notko takes properties and arvo takes mathematics |
| D19, D37: `Enumerator` and `ConstDefault` to notko | subject matter over proximity; "filing by proximity is exactly how arvo-storage became what this decision is undoing" (D27) |
| D23, D26: `Identity` to algebra; `Bounded` dissolves into two identities | one concept never splits across two crates; two constants named after their values is the shape `Identity<Op>` was built to replace |
| D25: `bitfield!` its own crate | place by what a thing becomes, not what it is (a macro crate that may become proc-macro cannot be the same crate as a library) |
| D27, D28: storage decomposes, refit comes home, `arvo-refit` deleted | an empty crate is "a gateway to a family filed in the wrong place"; pre-1.0, no shim owed |
| D42: hashing distributes along both axes; `arvo-pseudorand` | distribute by mathematical fact rather than convenience; "pseudo is not padding, it is the guarantee"; do not split a contracts crate until there are three traits to hold |
| D72: one crate per contract; `arvo-strategy` keeps only the presets | a crate's name was never wrong, what was wrong was what accumulated inside it |

**Register:** none maps to a live Q by number; the current panel's decomposition is open ground. The
reasons column is the part worth testing against whatever decomposition the canon produces.

### Era one: the two explicit survivals

**Fixed-point multiply becomes real, now (2026-06-23).** "Decision (op, 2026-06-23): make bare `*`
rescale for `F > 0`, with overflow-safe widening", coupled with `Identity::ONE` becoming the
fixed-point one; and on the deferred widening machinery, "op's call is that the first real need
surfaces that impl now"
(`mock/design_rounds/202606231001/202606231001_topic.fixed-point-multiply-widening.md`).
**Answering:** what `*` means on a fixed-point type, after the `x * ONE != x` defect. The call
pattern (a deferral dies the moment a real need arrives) recurs at D25 and D42.

**Bad behaviour must be loud (2026-06-23).** "per op, the bad Precise behaviour must be loud. A
#[ignore] catalogue red" over a quiet skip
(`mock/design_rounds/202606231218/202606231218_changelist.src.lock.md:39`).
**Answering:** how to record a known-wrong behaviour in the suite. Feeds section 8 (loud failure
over quiet wrongs), and is continuous with `Precise` staying fallible and refusing through every
later design.

---

## 8. Taste, inferred

**This section is inference, not record.** Op asked for exactly this use: material "to learn my
preference, taste, gut instincts and intuition by inference", none of it absolute. Every pattern
below is grounded in at least two independent instances; where a pattern rests on fewer, it says so.
Design preference only; no psychology.

**T1. Intent outranks mechanism, always, and the mechanism is disposable.** The Warm-behaves-like-
Rust call was restated four times across two days because it kept failing to stick (`INTENTS.md` I3
notes the count); "the intent is what remains and matters" (I3); "none of this should *override* the
real intent of the design ... The spirit" (`16d`); D34's whole method (derive the band from what the
marker promises); "we are fully free to restructure the strategies and their meanings" (`68b`).
Corollary he states himself: the intent is vague **on purpose**, cannot be read literally, and two
honest readers can differ (`16d`), which is why he refuses single-angle resolutions.

**T2. He pushes past the offered options, reflexively.** All three answers at the first checkpoint
were "iterate further" rather than a pick ("Option 1 but not just price, iterate on; there might be
ergonomics to be won", "I feel there are potential yet untapped", "perhaps a better shape emerges",
`04b`); preset divergence "deserves more than the first mechanism that works" (`30b`); the novelty
posture ("attempt to find solutions that seem unsolvable, but only in lack of prior art on it",
`34b`). Offering him a menu invites a fourth option.

**T3. Derived over asserted; absence as a statement.** D16 is his sharpest formulation; D61 turns
membership structural; D51 derives laws; his objection that "the typestate proof was sitting in
comments rather than in machinery" (talk `:999`) killed an entire option class; the
`OneRepresentable` absence mechanism is the shape he approves everywhere it appears. A property that
could lie should be unable to compile, not documented.

**T4. The field's own word beats a coined one, and full words beat abbreviations.** D56 ("Coining
short forms of words that were already a sensible length is not arvo's style and has never been");
quantum and quantisation adopted from the standards with sources; "the field's own word is
direction" (D63); `Magma` over `Combine` (D75); `Orthotope` chosen by collision analysis (D40);
`arvo-ndim` dropped for abbreviating (D3); `notko-typelist` read as hype (D9). He accepts a
near-collision when the relationship is real: "Two words that sound related, for two things that are
related, in a fixed direction, is accurate naming rather than a collision" (talk `:385-388`).

**T5. Vocabulary survives mechanism unification.** `Capacity` keeps its name over the shared carrier
(`74b`: "I would still keep the semantic alias"); every domain aliases the hlist cell to its own
words (D7); the four ergonomic aliases survive near-zero use because they name an axis the stack
will want (D31). Deleting a name because its mechanism merged reads to him as discarding reasoning.

**T6. Usage counts do not decide surfaces; intent does.** D31 verbatim ("a surface being
unexercised today says more about how young the consumer set is than about whether the surface is
wanted"); D38 ships a whole family ahead of any consumer; the `06b` correction ("what a consumer
currently writes is evidence of what was absent when they wrote it ... we focus on the optimal, what
the consumers would ideally deal with and in").

**T7. Anti-speculation by default, with exceptions argued loudly.** The default: no crate ahead of
its contents (D42's sequencing, with `arvo-refit` as the cautionary worked example; D28 deleting
it). The exceptions are always argued from irreversibility or fixedness: D38 (mathematics fixes the
vocabulary, so it cannot be got wrong), D25 (a future crate split is a future migration, so split
now). When he breaks his own default he says he is doing it.

**T8. Measurement decides forks, but he interrogates what was measured.** D11 and D41 (bench the
broadest matrix, requirements named first, "the round had reduced this to a binary and the binary
was wrong"); D46 (three mechanisms offered, none chosen, because the premise was unchecked:
"research question, not a mechanism to pick"); `108b` ("Confirm the digest, keep pushing the bench",
the crossover standing "as a first honest signal and not as Cold's price"); `77b`'s follow-up on
whether the bitpack multiple was inherent or an artifact of the access pattern. A number that might
be measuring the wrong subject does not settle anything for him.

**T9. Toolbox, never policer; refusal only where the mathematics refuses.** D52 (sealing the
composition would be arvo choosing for the consumer); D66/D67 (vendor vocabularies as opt-in
aliases); `74b` (declining declaration-time refusal because it forbids a shipped silicon profile);
`108b` (`Hot`'s division cell "goes to the consumer" because the design has no derivation and the
consumer knows). But where the mathematics itself refuses, he wants the refusal in the type
(`Precise` fallible, folds refused for non-associative pairs, overlap refused unless declared).

**T10. He delegates at the edge of his expertise, marks it, and wants the delegation attacked.**
"stated they are not a mathematician, and delegated the call" (D24, which the file itself then
labels "the most suspect thing in this file"); "This requires someone more versed in the compute
side theory to confirm" (`74b`); the `KCell` naming question put as a question (D40); "Option 2 as
my instinct, stress tested and evaluated by an expert still, before locking" (`70b`). An instinct
from him arrives with an explicit invitation to break it, and twice in this corpus the stress test
did (bitpacked layout) or generalised it past his framing (the far point).

**T11. Runtime cost is never deferred; compile time is a resource to spend.** `77b` verbatim
(section 5); the whole GCE cure (hundreds of per-width impls accepted as the price of no runtime
check); D38's "expressing centuries-settled distinctions as bounds costs nothing at runtime" as
recorded reasoning.

**T12. Loud over quiet, in failures and in process.** The era-one catalogue-red call; `Precise`
refusing rather than lying, kept through every redesign; the D4 trust-collapse response (line-by-line
audit, "no greps, no skips"); the ratification-goes-stale principle with mandatory re-derivation
rather than silent citation (`108b`); "the archive is instrumented for error and not for loss, and
that is the defect" (`108b`). He treats silent disagreement between artifacts as the worst state a
record can be in, worse than being wrong.

**T13. Blunt about his own past work, and it carries no sunk-cost weight.** "what my amateur ass
had written" (I1); "the existing shit is fucked" as the premise of the rework (`16c`); "I am not
sure, since I was away for the night" (`68b`, on Cold's concrete meaning). One instance short of the
two-instance bar as a *design* pattern, but it is corroborated by the balancing call: keep the
current shape where keeping costs nothing, rewrite cost is real (`16d`). The self-dismissal never
extends to the intent behind the old work, only to its execution.

---

## 9. Bearing on the two questions currently in flight with op

Neither question is answered here. The corpus bears on both, and the material is surfaced as
material.

### Which verb "validate" is, in his acceptance criterion

Three prior statements show what validation has meant to him operationally, in his own words:

- **Run both and compare.** The parity-suite mandate: "actual tests where we run same stuff in our
  api and then on matlab/sysc/etc APIs, and assert that both actually return the same ... on a wide
  selection of different usages", macro-driven for volume
  (`OLD_79b_op_the_verification_mandate.md`).
- **Both directions, including refusal.** "things that should express and manifest in a certain way,
  do so, and those that should simply be inexpressible in our typestate and contracts, actually do
  fail to compile ... we'll have to start from a huge enormous amount of red tests, TDD style"
  (`79b`).
- **Expressibility as a falsifiable adequacy test.** D67: a convention mode that cannot be written
  as an alias over the abstraction is a gap in the abstraction, testable per standard before
  shipping (talk `:1459`); and `13c`'s framing of the standards as "a test" rather than an
  inspiration.

Note the scope he attached at the time: the `79b` mandate was explicitly recorded as binding the
implementation phase, not the panel's own derivation. Whether the current criterion's "validate"
carries the `79b` sense, the D67 sense, both, or a new one is exactly the open question, and this
history must not be read as answering it.

### Whether the long-standing constraints are his intents at all

The specific finding, stated carefully: **across everything read for this file, no op-verbatim
statement was found mandating `no_std`, no `alloc`, const sizes, no `dyn`, or no `TypeId` as such.**
The constraints appear pervasively in agent-authored surfaces (workspace rules, generated
instruction files, round prose), and `mock/../.claude/CLAUDE.md`'s own provenance section now says
the same. What op's recorded voice does carry, repeatedly, is the *reasoning family* those
constraints serve:

- amortise everything at compile time, never defer avoidable cost to runtime (`77b`, verbatim in
  section 5);
- "everything following the same typestate explicitness and clever design to get all of our
  contracts expressed without forbidden features is a net win in my books" (`74b`);
- the feature gate itself: allowed only if sound and on the stabilisation path, a standing personal
  policy predating this workspace ("settled during loimu-era feature vetting", D1 of
  `202607282100`);
- determinism as the reason arvo exists (D49).

So the corpus supports "op consistently reasons as if these constraints hold and has never once
pushed against them", and does not supply a ratification of the constraints themselves. That
distinction is the finding; the call on it is his.

---

## 10. Reversals, as their own index

A call reversed tells more than a call standing. Chronological; each names both positions.

| When | From | To | The gap |
|---|---|---|---|
| 2026-05-29 to 07-28 | GCE at WATCH (vetting resolution) | GCE FORBIDDEN (D1, `202607282100`) | the vetting feeding the WATCH had itself flagged FORBID; the record drifted, trust in the rule corpus reset to zero |
| 07-28, one day | `arvo-shape` scoped 2D (`202607281005`) | N-dimensional, rank a parameter (`202607281052`) | scoping had been driven by one consumer; "a point in a 17-dimensional parameter space and a point on a screen are the same construction" |
| 07-28 to 07-29 | D13 proposes the algebra mechanism arvo lacks | `202607281547`: arvo already ships `Identity<Op>` in a better form | the corrective instruction attached: read the existing designs properly before designing, never reason from summaries |
| 07-29 to 07-30 | D29: floats split off because "not the same kind of thing" | D50: one thing at different exponent functions; the crate survives as packaging | the placement stood while its founding reason was replaced; he distinguishes the two |
| 07-30, in-sitting | D62 `Quantisation` on the numeral | D64/D65: the word belongs to the policy axis; the numeral member is the exponent's representation | "the naming difficulty was the symptom"; a concept that resists naming is the wrong concept |
| 07-30, in-sitting | D65 three numeral members | D68 four flat members with `Bias` | one counterexample (UNORM under bias-only) forced the amendment within hours |
| 07-30 to 08-03 | D69: ten axes, `LogicalWidth` primitive | `30b`: mathematical coordinates, width derived | "the off-by-one ... was the parameterisation reporting that it was pointed the wrong way" |
| 07-30 to 08-04 | D71 preset table, all four rows | `68b`: two rows voided (justified from shipped doc comments); `70b`: re-derived tables ratified | the void was procedural before it was substantive: the evidence class was illegitimate, so the rows died regardless of whether they were right |
| 08-04, two checkpoints | `74b`: bitpacked as two instances (op's lean) | `77b`: one meaning, zero inter-value padding | his own second sentence at `74b` had flagged the ambiguity that dissolved his own first |
| 08-04 | `74b`: bitpack at 4.6x dense | `82b`: about 1.50x | the measured decoder had left compile-time-derivable work at runtime; the fourth rule, one file old, produced the correction |
| 08-08 | the whole lineage | the tree nuked, the canon panel opened | the terminal reversal this file exists inside |

One reversal in the corpus is the agent's rather than op's and is kept for what it says about the
record: the PGA-fork correction at D41 was itself retracted the same day
(`202607300800/202607291910_topic.the-box-and-the-rotation-bench.md`), with the recorded lesson that
"unratified" is not a licence to overturn a checkable claim on weaker evidence than built it.

---

## 11. Coverage

**Read in full at source:** the two D-register topic files
(`202608082157/202607301100_topic.the-formalization-talk.md`, all 1848 lines;
`202608082157/202607301000_topic.inherited-state-from-the-formalization-round.md`, all 2284 lines);
all 23 op-authored files of the closed panel as censused by its `OLD_112_the_op_material_sweep.md`
(`04b`, `06b`, `08b`, `12b`, `13b`, `13c`, `16b`, `16c`, `16d`, `17b`, `24b`, `30b`, `34b`, `39b`,
`44b`, `68b`, `70b`, `74b`, `77b`, `79b`, `82b`, `86b`, `108b`); `112` itself in full; this panel's
`INTENTS.md` in full; the relevant sections of `OPTIONS.md` (the per-question gists, Q1 through
Q32) and `DROPLIST.md` (sections 6 headings, 7, and the closed-by-this-panel entries).

**Read via a consolidation, with spot checks:** the restructure round's D1 to D52 were read through
the inherited-state file, which is an agent compression. Two originals were opened to verify
fidelity (`202607300800/202607291800_topic.the-last-four-calls.md` for D34 to D37 and
`202607300800/202607290200_topic.the-predicate-decisions.md` for D16): decision numbers,
attributions and dates match. The remaining originals in `202607300800/` were not individually
opened; where an entry above quotes D1-to-D52 material, the quote is the consolidation's text and
the cited original is where the decision lives.

**Not read:** the closed panel's 99 numbered member files (quoted only where an op file quotes
them); its consolidations `109`, `110`, `111`; the nine persona stand-in checkpoint files; the
current panel's numbered member files (out of scope by design); and roughly 85 era-one rounds,
which were swept by grep for op-voice markers rather than read. The grep vocabulary was: `op's
call`, `op said`, `op decided`, `op ratified`, `op confirmed`, `per op`, `op asked`, `op's steer`,
`greenlit`, `human overseer`, `Decision (op`. **A grep verifies the terms are absent, not the
ideas**: era one may hold op decisions recorded without any marker, and this file's era-one section
should be read as "what the markers surface", not "what era one contains". That residual is the
known weakest coverage claim here, and widening it is a bounded follow-up if any expert needs the
early history.

**The universal-negative in section 9** (no op-verbatim mandate of the five constraints) is bounded
by the same limits: it quantifies over the files listed as read in full, plus the marker-grep over
everything else. It is stated as "not found", not as "does not exist".
