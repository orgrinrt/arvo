# 91. Entailment check: derived algebraic laws consolidation

**Role.** Independent check on `90_giesen_consolidation_derived_algebraic_laws.md`. I wrote neither `90`
nor any of the nine files it compresses. Working from the members forward, per the brief and per
`RULES.md`'s own instruction that the author of a compression is the person who believes it entails and
cannot be the one who checks it.

**Gates.** Canon gate: passes, situation two. No canon exists; `mock/canon/` is absent and `mock/crates/`
is empty by the declared mutation order; this panel is writing the first canon. Checked against
`INTENTS.md` in full and `RULES.md` in full; this dispatch is exactly the entailment-check mechanism
`RULES.md:277` and `87` require, so it is licensed rather than merely permitted. Test gate: no suite
exists. Applied as: for every claim `90` carries as established, I checked whether the member's own
instrument was shown able to fail (a negative control, a mutant, a refusal). I did not find a case where
`90` carries a claim whose instrument was never shown able to fail; the four such instances the unit
itself produced (`89` section 1, carried at `90`'s R13) are correctly reported as a result rather than
smoothed over.

## Coverage, stated first because it bounds everything below

**Read end to end:** all nine members (`76`, `77`, `79`, `80`, `81`, `82`, `84`, `86`, `89`), both phases
of both cold derivations, all four op files (`83`, `85`, `87`, `88`), `INTENTS.md`, `RULES.md`, and `90`
itself, all in full.

**Not read:** any probe source. Every numeric claim I verified below was verified by re-reading the
member's own prose and by hand-recomputing the arithmetic where the arithmetic was cheap (percentages,
ratios); I did not rebuild or re-run any `.rs`/`.py` instrument, and I did not open `OPTIONS.md` or
`DROPLIST.md` beyond the specific line ranges the members and `90` themselves quote. Anything I say about
`63`, `74`, `35`, `42`, `55b`, `67`, `68`, `73` is routed through the members' or `90`'s own citation of
it and inherits their errors if any.

**What I built:** the anchor diff below, run by shell command, reproducible. I did not build a compiled
probe of my own; nothing in this check required one, since the question is whether `90` entails its
sources, not whether the sources' own claims hold.

## The anchor diff

Per member, `grep -oE` for the panel's two citation shapes (`file:line[-line]` and `NN_probes/name`),
unioned, diffed against `90`'s own anchor set:

```
union of nine members: 232 unique anchors
carried into 90:        184  (79.3%)
dropped:                  48  (20.7%)
```

I read every one of the 48. The overwhelming majority are not losses:

- **Subsumed by a broader range `90` does carry.** `INTENTS.md:177-198` (an early I13 citation, before
  op's amendments extended it) is subsumed by `INTENTS.md:200-252`, which `90:21` uses and which `90`'s
  own coverage section (`90:769-772`) explains is deliberate: `INTENTS.md` moved while members were
  writing, and citing the current lines rather than restoring stale ones is the right call, stated as
  such. `84:347` is subsumed by `84:344-347` at `90:329-330`. `84:368-370` is subsumed by `84:362-370` at
  `90:582`. `63:665-673` is subsumed by `63:659-673` at `90:142`. `OPTIONS.md:1901` is subsumed by
  `OPTIONS.md:1896-1901`. `82:455-473`/`82:455-480` are subsumed in substance by `90`'s discussion of the
  binding-time ladder at `90:384-388` (rung 0 unskippable, rung 3 reachability-dependent).
- **Duplicate/abbreviated forms of a citation whose full form is carried.** `84_probes/p4:331-336` and
  `84_probes/p4` are `86`'s and `84`'s own shorthand for `84_probes/p4_difference_certificate.rs`, whose
  full form with the identical line range is at `90:472`. `p2c:30-33` and `p2c:134-143` are `84`'s
  shorthand for `80_probes/p2c_closed_form_checked_on_a_model.rs`, carried without the line suffix at
  `90:68`. `86_probes/p5` is carried in full at `90:472`.
- **Coverage-list citations carrying no unique claim.** `68:92-220`, `28:1-120`,
  `seed/OLD_SETTLED_container.md:20-55`, `DROPLIST.md:205-260`/`210-260`/`220-260`, `RULES.md:99-101`,
  `RULES.md:126-133` are all "what I read" ranges from a member's own coverage section, not citations
  supporting a specific claim. The load-bearing sub-citations inside those ranges (`28:67-95`, `28:82-95`,
  `seed/OLD_SETTLED_container.md:33-36`) are all carried, at `90:46,48,373`.

**Two genuine, minor losses**, neither reopening anything: `35_probes/p1` (77's citation, via 42, of a
different unit's fold-widening refusal) and `67:440-446` (79's pointer connecting this unit's "whatever
named axis a strategy resolves to" placeholder to a specific open question in a different unit). Both are
connective tissue to material outside this unit's own topic; both concepts they point at are otherwise
present in `90` (the staging boundary via `80` section 7, and the placeholder itself via R1), just without
that specific cross-reference. I would fix the `67:440-446` one if I were repairing this file, because
`90` section 6 is explicitly the cross-topic handoff section and this is exactly the kind of pointer it
exists to carry; I would not spend effort on the other.

**Conclusion on the anchor diff: the citation trail is intact.** No claim in the nine members loses its
only route back to evidence. This is a genuinely well-executed compression on the axis `RULES.md` warns
about most specifically (citation counting), and it should be said plainly rather than buried under the
findings below.

## Findings

### 1. R8 misattributes which files "share an author" (moderate; correct before further citation)

`90:346-348`:

> `82` section 12, stated as one-directional sufficiency, with the member's own honesty note that its
> three instances, `77:250`, `76:370-372` and its own box characterisation, share an author and a
> framing

This reads as: 76, 77, and 82's own box characterisation are three items that "share an author." That is
false on its face (Willsey, Amin, and Jhala are three different personas) and it is not what `82` says.

`82` makes two separate honesty notes, at two different places, about two different sets of "three":

**`82:184-192` (section 4)**, about the closure criterion generally: *"It is the same criterion two other
files reached for the chain-machinery question, `77:250` and `76:370-372`... I reached it from `p1`'s box
characterisation before reading either file and can state that ordering, which is not the same as being
able to claim independence."* This is about three different authors converging on a related criterion,
with the independence claim limited to ordering (reached-before-reading), not to strength.

**`82:951-953` (section 15, item 5)**, about `82`'s own three internal lifting instances (declared-range
box, sign uniformity, length-aware): *"It is drawn from three measured instances and two files that
reached the same criterion for a different question. Three instances is the panel's bar for a claim, and
these three are not fully independent: they share one author and one framing."* "These three" here is
`82`'s own P4-lifting, sign-uniformity, and length-aware findings, all inside `82`'s own file. The "two
files" are named separately, in the same sentence, as a distinct category from "these three."

`90` has taken the "share one author" clause from the second note and attached it to the three items named
in the first note. The result inverts the epistemic picture: it reads as though the cross-persona
convergence (76, 77, 82 independently reaching a related closure argument, which is real corroborating
signal precisely because it is cross-persona) is instead a single-author artifact, while the genuinely
single-author, non-independent set (82's own three internal findings) goes unflagged.

This sits inside an already-hedged section ("stated as one-directional sufficiency") and R8 is listed
among the "genuinely open" items in `90` section 8, so no downstream design work rests on the
misattribution as written. But it is exactly the class of error the panel's own independence discipline
exists to catch (`RULES.md:47-48`: "agreement among unratified artifacts is not corroboration... agents
copy each other's framing"), and a canon writer skimming R8 for "how independent is this" would come away
with the wrong answer in both directions at once: understating the 76/77/82 convergence, overstating
82's own three-in-one-file result.

### 2. A live option is silently dropped: 76's ordering candidate for the strategy space (severe; should be recovered before the strategy-axis unit starts)

`76:200-210`, phase one, offered explicitly as a candidate rather than a finding:

> Reading I7 and I5 alongside I8, one candidate shape is: Precise's congruence is, in the cases where the
> underlying arithmetic actually agrees, a refinement of (at least as strong as) Hot's, because Precise is
> defined to preserve chain-level facts Hot is explicitly licensed to give up. That would make "how many
> chain-level laws are honored" a genuine partial order with Precise at the top and Hot's honored set a
> subset of it wherever they overlap. I want to be explicit that this ordering claim is my own synthesis,
> not stated anywhere in the premises, and I have not checked it exhaustively the way I checked the
> associativity claims; it is offered as a candidate the panel might want to test the same way I tested the
> associativity claims, not as a result.

This is a member explicitly flagging a testable, falsifiable candidate and inviting the panel to attack it
the way it attacked the associativity claims. I checked whether any of `77`, `79`, `80`, `81`, `82`, `84`,
`86`, `89` engaged it: none does. Nobody mentions "refinement," "partial order," or the Precise/Hot
congruence-ordering idea again anywhere in the unit.

`90` does not carry it. I checked all three places it would belong:

- **Section 5, live options.** Lists Q38, Q39, Q40, O-I, the defect-and-benefit pair, cross-strategy
  resolution laws (`77` probe 3), the length-aware predicate, the expensive general-signed route, the
  per-point-cost experiment, and `76`'s tier taxonomy. Not this.
- **Section 8, genuinely open.** Lists eight items, none of them this.
- **Section 6, where this topic touches the others.** This is the section built specifically to hand
  material to the next unit, and `90` names it explicitly: "What this unit hands the strategy unit: the
  resolve-operator laws (`77` probe 3), I13's arms as the mechanism a strategy's weighting would select
  among, and the observation that `(operation, strategy)` was already too coarse before the axis was even
  settled." A partial-order candidate over the strategy space, explicitly offered for testing, is exactly
  the shape of thing this section exists to carry forward, and it is a closer match to "the strategy unit"
  than the resolve-operator laws that did get carried.

This is not a hedge or a passing remark; it is an author explicitly marking a candidate as untested and
askable, in the same rhetorical register the panel uses everywhere else for options it wants kept alive
(compare `82` section 12's own candidate framing, which `90` does carry as R8). `RULES.md:198-202` names
exactly this failure mode: *"An option that no member resolved has no result attached, so there is nothing
for a compressor to grip, and it falls out precisely because it is still open... the options most likely
to be lost are the ones the panel most needs carried."* This is a textbook instance of it, and it is worse
timed than usual: op's `87` section 3 just named the strategy axis as the next unit
(`87:67-68`, `"the strategy axis is next, and topic four follows it"`), which makes this exactly the
candidate that unit would want in its inbox and does not have.

I do not think this needs a repair pass on `90` itself; `90` is right that repairs are a separate act. I
am naming it here so the strategy-axis unit's dispatcher pulls `76:200-210` directly rather than starting
from `90`'s section 6 and never finding it.

### 3. Op's file 88 is under-cited relative to its stated governing status (minor, bounded)

`90`'s own framing (`90:9-10`) says all four op files "govern everything below." Two of `88`'s four
sections bear directly on this unit and are cited only for their meta-methodological content, never for
their substance:

**Section 1** (`88:8-30`) is op's own structural answer for what a strategy is: "mostly option 1, but a
little bit of option 3 with it," a preset naming a point in a space of axes, where the axes exist because
of a weighting the preset expresses. This bears directly on R1's placeholder, "whatever named axis a
strategy resolves to" (`79:200-219`, carried at `90:106-107`), which is the exact question section 1
partially answers. `90:657` gestures at "a strategy's weighting would select among" without citing `88`
section 1 or stating that op has already partially answered the structural question. I checked and this
omission does not misstate anything; it just under-delivers on `90`'s own promise that this file governs
the unit's content.

**Section 2** (`88:32-63`) corrects `I8`: the clause "for the most part, they probably agree" is not part
of the ratified intent, only the sentence before it is. `90:555-557` cites `88` sections 2 and 4 but only
for the general methodological lesson ("quoting verbatim and naming the intent inside the quotation are
two acts... a fork asking one rule to govern a category is the rejected universal wearing new clothes"),
never stating that I8 itself was corrected. I checked whether this bears on anything `90` carries: `76`'s
ordering candidate (finding 2 above) is the only place in the unit that leans on I8's second half, and
`76` itself already marks that half as "his own possibly-wrong instinct rather than a settled claim"
before op's correction existed. So the correction changes nothing that `90` carries, and this is a
completeness gap rather than a factual error.

## What I verified and found correct

Stated because a report that only lists defects gives a false picture of how sound the file is.

- **The band-transfer defeat chain (R4, R5, R6).** I re-traced the four-instrument correction sequence
  (`80` proposes the band, `82` corrects its frontier by one bit, `84` defeats its transfer by
  construction, `86` proves the criterion and extends the defeat to saturating laws, `89` simplifies it and
  proves it at every arity) against each member's own text and found `90`'s account faithful at every step,
  including the specific numbers (82.7484%, 21.98%, 24.8%, the width-64/65 flip on `(x)_34(y)_34`) which I
  hand-recomputed and which match.
- **The four op files' answers on I15, I16, and Q-B (the long-standing constraints).** All three are
  carried without softening. I15's "never any runtime checks, ever" is not weakened into "as much as
  possible" anywhere in `90`; the value-gated arm is correctly reported as killed twice, once by
  measurement and once by principle (`90:508-509`). I16's refusal to rank law constructions is stated
  functionally, matching `85:49-59` exactly. Q-B's closure (the constraints are intents, not inherited
  ground, nothing built on them needs redoing) is carried without the hedge that had propagated through
  five earlier files.
- **The TWO EXPERTS rung.** Checked; it appears exactly once (`90:98`), correctly applied to `76`/`77` on
  R1, and `79`'s correction to `76`'s inflated self-agreement claim is correctly reflected rather than the
  original inflated count.
- **The test-gate marking (R13).** All four instances of an instrument shown unable to fail are present
  with correct attribution of who caught what, matching my own read of `84`, `86`, and `89`'s own
  self-reports.
- **The Q39 synthesis (`90` section 5), which `90` itself flags as its weakest joint.** I checked whether
  joining `83`, `85:20-25`, and `88` section 3 into one reading of Q39 overreaches. It does not: option
  (c) was already framed by the panel itself (`80`'s O-G(c)) in terms of an "ingest boundary," so `88`
  section 3's answer about who owns the ingest boundary is a real answer to that option, not an imported
  question. The synthesis is a judgment call, correctly flagged as one, and it holds up.
- **No predicate found silently widened.** I checked R1's dimension list, R6's "any arity" claim, and R7's
  necessity/sufficiency split against their sources and found each attribution correctly kept to its
  establishing member, with widenings stated as the later member's own claim rather than merged into the
  earlier one's.

## Verdict

`90` should be read with finding 2 in hand before anyone starts the strategy-axis unit, since that is
where the dropped candidate belongs and where its absence will otherwise cost a rediscovery. Finding 1 is
worth a one-line correction whenever `90` is next touched, but nothing in the unit currently cites R8 for
its independence count, so it is not urgent. Finding 3 needs no action; it is a note for whoever writes
the strategy-axis unit's brief, not a defect in this file.

Everything else checked, including the part of this job most likely to fail silently (the anchor diff),
came back clean.
