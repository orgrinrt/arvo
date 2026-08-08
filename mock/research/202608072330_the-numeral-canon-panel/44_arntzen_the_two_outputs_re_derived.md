# 44. The two outputs, re-derived

**Status: COMPLETE.** Written to disk early per `RULES.md`; extended in place throughout.

**Persona:** Hans-Kristian Arntzen. **Date:** 2026-08-08. **Mode:** explore, do not settle
(`00_brief.md`, `04`, `28`). Nothing here settles anything; where I say a route holds or a route is
wrong, the diagnostic or the citation is attached.

## 0. Why this dispatch, and the gates

Two files, `35` and `43`, now build on `16` and neither has read it: both reach it exclusively
through `OPTIONS.md`'s compressed account and say so explicitly in their own coverage sections
(`35:816-817`: "The specific risk: `16` and `24` are the two I lean on most... and I have read
neither"; `43:869-874`: "`16` is the file I lean on most and I have not read it... two files now
depend on one unread source through one compression"). `RULES.md`'s standing rule ("A shared source
read by nobody is a single point of failure") requires the next dispatch on that material to read the
source. This is that dispatch.

**Canon gate.** There is no ratified canon for this panel to defend or diverge from. The fixed
material is `01`, `04`, `28`, `INTENTS.md`, the workspace discipline, and the forbidden-feature list,
plus the acceptance criterion named in `00_brief.md`'s "What is fixed" section. The container
derivation this file is about is not speculative scope. It is directly named by the acceptance
criterion itself: "the typestate derives the matching container and representation" (`00_brief.md:145`,
itself a compressed restatement of op's own words, on which more in section 3). Establishing what the
second noun of that criterion actually is, and whether the panel's derivation supplies it, is squarely
inside the fixed material rather than adjacent to it. **Gate: passes.**

**Test gate.** `mock/crates` is empty by construction (removed 2026-08-08). There is nothing to run.
Every claim below is either a direct reading of a frozen panel file, a `grep` I ran myself, or a
citation I opened myself. No new probes were needed: the question is whether a claim was established
and whether the register states it accurately, not whether new code compiles. Where I quote a compiled
result from `15` or `16`, I read the probe listing and the output blocks inline in the file rather than
re-running anything, because the files themselves are frozen artifacts of a closed dispatch and their
committed `NN_probes/` directories are the evidence; re-deriving would be redundant with the discipline
already applied to them by their authors and audited once already (`21`, on a different document, see
section 4.4).

## 1. Reading order, and what happened at each step

1. `INTENTS.md` in full.
2. `00_brief.md` in full.
3. `RULES.md` in full.
4. `15_giesen_the_axes_the_ladders_left_out.md` in full, direct, before touching `OPTIONS.md`.
5. `16_aaltonen_second_read_on_the_two_outputs.md` in full, direct, before touching `OPTIONS.md`.
6. `10_lattner_fresh_eyes_on_the_container_derivation.md` in full.
7. `17_leroy_what_would_actually_certify_this.md` in full.
8. `35_mcsherry_what_the_layers_above_need_from_the_numeral.md` in full.
9. `43_rompf_what_a_composition_is.md` in full.
10. Op's `32`, `36`-`39` in full (`34`'s and `33`'s contents were already fully captured verbatim in
    `INTENTS.md`, which I read first; I opened `32`, `36`, `37`, `38`, `39` as standalone files for their
    surrounding argument, since `INTENTS.md` deliberately strips that per its own stated scope).
11. Only then, `OPTIONS.md`'s section "The derivation's outputs" (lines 703-773), line by line, against
    what I had already derived from `15` and `16` directly.

## 2. My own derivation, reading `15` and `16` cold

Before comparing anything to the register, here is what the two source files actually establish, in
my own words, checked against the text I opened rather than against any summary.

**`15` (Giesen) built a three-input map, `(strategy, width, sign) -> two outputs`, and found the second
output the hard way, by getting it wrong twice.** Its first attempt computed the non-`Cold` stride as
`8 * ceil(W / 8)`, which is wrong at every width that rounds up a container rung (`W = 24` gives 3
where the true stride, matching a `u32` container, is 4 bytes; `15:345-349`). The repair is that the
stride for a non-packed strategy is the *container's* width, not the value's rounded-up bytes. Its
second attempt, having fixed that, was still wrong at the wide rung, because `Hot`'s wide arm pads to
align 16 while `Warm`, `Cold` and `Precise` pad to align 1: at `W = 200` the payload is 25 bytes and
`Hot`'s container is 32 (`15:351-354`). So `15`'s own stated conclusion is that **stride is keyed on
the `(strategy, rung)` pair**, not on width alone and not on rung alone, and both repairs are pinned by
compiled negative controls that refuse the wrong values (`15:366-373`, `q08_negctl.rs`,
`q08_negctl2.rs`).

**`16` (Aaltonen) derived independently, with a stated and specific contamination.** Its first
orientation command was `git log --oneline -8`, which printed a commit subject naming "two outputs"
before any derivation had been written (`16:17-24`). Aaltonen states plainly that this makes the
**count** worth "close to nothing" as independent evidence, while the **content** (which two, what the
second is keyed on, what fails without it, which check is blind) was derived before opening `15` and
is where the second read has value (`16:27-33`). I take this self-assessment at face value; it is
precise about which part of the file is and is not independent, and it is the kind of hedge
`21`'s audit (section 4.4 below) finds gets dropped in compression more often than it should.

Aaltonen's own hand derivation reaches two outputs he initially calls **carrier** and **extent**
(`16:145-152`), by a different route than `15`'s: the argument is an injectivity failure. The map from
a declared numeral to a container type collapses eight distinct `Cold` widths (9 through 16) onto one
sixteen-bit carrier, and that collapse destroys exactly the information `Cold` exists to carry
(`16:126-141`). A carrier-only derivation of `UFixed<13,0,Cold>` is 23.1% larger than the strategy
promises (`16:312-327`), and the panel's own erasure-and-codegen-equality check, the instrument that
certifies the acceptance criterion's fourth clause, passes it at full marks anyway, because its
instrument is a scalar comparison against one native primitive and has no array in it (`16:199-233`).
That is the blindness result, and it is Aaltonen's independent finding, built and run as
`16_probes/p3_blind_suite.rs` (four of four green against a derivation that is 23.1% too large).

**Then Aaltonen reads `15` and corrects himself, in the direction of tightening rather than
padding.** His own `p6` emitted three associated items (`Carrier`, `EXTENT_BITS`, `STRIDE_BITS`);
comparing against `15`'s two-item `q07_three_input_map.rs` (`Container`, `Stride`), he applies his own
stated criterion for what counts as a derivation output ("the consumer did not write it, the machine
needs it, a site holding the other components cannot recover it," `16:100-101`) to his own third item
and finds `EXTENT_BITS` fails clause one: it is the declared total width, which the consumer already
wrote and which the numeral type already carries as a structural parameter (`16:566-580`). So the
honest count is two: carrier and stride, and Aaltonen's original "extent" was a pair of which half was
an input travelling under a new name. This self-correction is itself an instance of the discipline the
panel asks for (`RULES.md:99-101`, keeping something when it survives, discarding what does not, even
when the discarded thing is your own coinage from three sections earlier).

**Both files independently attack and refute a third-output candidate.** `15` finds the wide-rung
alignment does not need a third slot because it rides on the carrier (`15:530-556`), and flags its own
residual doubt about whether alignment is a fifth axis. `16` builds the check `15` could not
(`16_probes/p7_alignment_is_not_a_third.rs`): two wide payloads of identical size and identical stride
at align 1 and align 16, confirming alignment is not recoverable from the pair `(carrier, stride)` and
must ride on the carrier as a type property (`16:605-621`). `16` also attacks a **different** candidate
third output, the packed access width (the number of bytes a load instruction must fetch to cover a
field at an arbitrary bit phase), and shows it is a closed-form function of the declared width alone,
`floor((W+6)/8)+1` bytes, checked against an exhaustive phase scan over widths 1 to 1024 with zero
mismatches (`16:173-198`, `16:664-673`). So two separate third-output candidates were proposed and both
were refuted with compiled or exhaustive evidence, not by assertion.

**On this much I fully agree with both files, having derived it from the same text they cite rather
than from a summary of it.** The map needs two outputs. Neither is recoverable from the other in
general. Alignment and access width are real quantities but are functions of the pair, not slots in
it.

## 3. Where I go further than either file: the acceptance criterion's own drift, now in three
generations, and a fourth I found in the panel's own brief

`16` opens with a careful section on this (`16:47-84`) that I read before forming my own view, and my
own check of the sources goes one generation further than `16`'s did.

Op's words, quoted directly and unchanged everywhere they are quoted (`seed/SETTLED_container.md:33-35`):

> There *is* a way to express usage through bits and bytes *and* have the typestate derive the matching
> container and **numeral representations**, then validate, and erase on lowering to be exactly what
> you describe before that caveat.

Two nouns, joined by "and," plural on the second. I checked this against the establishing source
myself (`seed/SETTLED_container.md:33-35`), not against `16`'s quotation of it, and it matches
character for character.

**First drift, inside the same file.** `seed/SETTLED_container.md`'s own "Claim" paragraph, three lines
above the quote it is glossing, already renders it singular: "the typestate derives the matching
container and numeral **representation**" (`seed/SETTLED_container.md:28`, checked directly). The quote
survives at line 33-35; the gloss above it does not.

**Second drift, in `SETTLED.md`.** `16:64-67` reports `SETTLED.md`'s own four-part gloss as "the
typestate derives the container and representation," dropping both the plural and the word "numeral,"
which I did not independently verify (I did not open `SETTLED.md`, which is not on my reading list and
is a document `RULES.md`'s own reading-list section marks as inherited from the predecessor panel's
classification and not fully trusted; I take `16`'s citation at face value here because it is a direct
quotation with a line number and the pattern it reports is exactly what I found independently one
generation further down, described next).

**Third drift, and this is the one I found myself, not reported by either `15` or `16`.** `00_brief.md`,
the panel's own founding document, marked "required reading" for every member and part of "What is
fixed," carries the same drift a third time:

```
grep -n "matching container" 00_brief.md
145:bytes, the typestate derives the matching container and representation, it validates, and it erases
```

**Singular. Not "representations." Not even "numeral representation."** This means the very document
that names the acceptance criterion as fixed, load-bearing, and answered-to by every mechanism
proposal in the panel, states it in the collapsed form that reads as one output rather than two. A
member arriving fresh, reading only `00_brief.md` (which every member is instructed to do), forms the
impression that the criterion asks for a container and stops there. The plural that is the only textual
evidence op ever gave for a second output is absent from the document every member is told to treat as
fixed.

**What this means, and what it does not.** It does not mean the two-output finding is wrong; the
finding as `15` and `16` establish it does not rest on the plural at all, and Aaltonen says so
explicitly and correctly: "The sentence is corroboration for a result, not the source of it"
(`16:83-84`). Both files reach two outputs by injectivity and compiled refutation, independent of what
op's sentence says. What the third-generation drift means is that **the panel's own founding brief now
misstates the one piece of textual evidence that would let a fresh reader recognise the finding as
consistent with op's stated intent**, and it does so in a document nobody but this dispatch has checked
against its own source, because checking a brief against itself is not a thing panel members are
usually asked to do. I flag this in section 8 as the highest-value single correction available, because
it costs nothing to fix (one word, in one file) and its cost if left is compounding: every future member
reads the collapsed version first.

## 4. Where the register's account differs from what I derived directly

This is the core of the dispatch. I read `OPTIONS.md:703-773` line by line against my own notes from
section 2, before writing this section, so the comparison below was made against my own derivation
rather than against the register's framing of the comparison.

### 4.1 The header rung claim, checked and confirmed accurate

`OPTIONS.md:705-713`:

> Rung: TWO EXPERTS on the identity of the two outputs and on what the second is keyed on; ONE EXPERT
> on the exact count, by `16`'s own downgrade after finding a commit-subject leak (`16` section 0, `21`
> section 2.2 confirms this rung split survives audit).

This matches what I derived in section 2 above, independently, before reading this line: `16` itself
downgrades the count to worth-nothing-as-corroboration while standing by the content as independently
derived. **This part of the register is accurate**, and I would keep it.

### 4.2 A misattribution I found: what "16 confirms" about wide-rung stride keying is not what 16 says it confirms

`OPTIONS.md:735-744`, in full, is the section stating what the second output is keyed on:

> **What the second output is keyed on, and two negative controls that pinned it there.** Not the
> width alone: at `W=24` a first attempt (`8 * ceil(W/8)`) gave three bytes for a `u32` container that
> is actually four (`15` section 3.4). Not the rung alone either: `Hot`'s wide-rung arm pads to align
> 16, so at `W=200` the byte payload is 25 bytes but the container-and-therefore-stride is 32, which a
> rung-only keying misses (`15` section 3.4, `16` section 10.2 independently confirms with an
> adversarial same-size-same-stride-different-alignment pair). The stride is keyed on the
> **strategy-and-rung pair**. Alignment specifically is **not** a third output: it rides on the carrier
> (a property of a type, via `align_of`), confirmed by an adversarial construction where two wide
> payloads have identical size and identical stride but different alignment (`16` section 10.2,
> `16_probes/p7`).

Read closely, the citation "`16` section 10.2 independently confirms with an adversarial
same-size-same-stride-different-alignment pair" is attached to the sentence claiming that rung-alone
keying is insufficient for **stride** at the wide rung (the second of `15`'s two defects,
`15:351-354`). But `16` itself says the opposite about exactly this claim, in its own coverage section,
which I opened and checked directly:

```
16:739-742
**Route I did not take.** The wide rung above 128 bits. `15:351-354` reports its second defect there,
that the stride belongs to the `(strategy, rung)` pair because `Hot` pads to align 16, and I did not
build a wide rung so I cannot confirm or contest it. My `p7` touches the alignment half of it and not
the stride half. Anyone picking this up should start from `15`'s `q13` rather than from my probes.
```

**`16` explicitly disclaims having tested the wide-rung stride-keying claim.** What `16`'s `p7` actually
establishes (`16:605-611`, checked directly) is a different, narrower claim: that alignment does not
reduce to stride and must ride on the carrier instead. That is a real, independently derived result and
it is correctly cited for that in the second sentence of the register paragraph above. But the first
sentence's citation of "16 section 10.2 independently confirms" attached to the **stride-is-keyed-on-
(strategy,rung)** claim is not supported by `16`'s own text; `16` says it did not build the apparatus
that would test it and names the file the next expert should start from instead of its own work.

**This is a genuine, narrow misattribution, and it is worth being precise about its size.** It does not
touch the underlying claim's truth: `15`'s finding that stride belongs to `(strategy, rung)` and not to
rung alone is a compiled result with two negative controls that refuse the wrong values
(`15:366-373`), and nothing here contests that it holds. What is wrong is the **provenance**: the
register states this specific sub-claim at a strength ("independently confirms") that its own cited
source disclaims for that specific sub-claim, in the same section the register cites. The register's
own header rung marker, "TWO EXPERTS... on what the second is keyed on," is therefore imprecise at this
resolution: it is accurate for the *general* claim that the second output is keyed on more than width
alone (both files independently reach this, by different routes: `15` by fixing two bugs, `16` by
deriving stride as `8 * size_of(carrier)` from the carrier's size directly rather than the value's
bytes, which `16:598-603` states is "an independent arrival at the same repair from a different
starting point" and which I confirm reading both texts is genuinely a separate derivation route
reaching the same non-`Cold` repair). It is not accurate for the **specific** wide-rung, align-16
sub-case, where only `15` has built and compiled the check, and `16` says so about itself.

**What I would do with this, offered as a correction rather than an edit.** Split the claim in the
register into the two sub-claims `15` itself distinguishes as two separate defects (`15:341-354`):
(a) non-`Cold` stride equals the carrier's width, not the value's rounded-up bytes, which **is**
independently confirmed by `16` from a different derivation route; and (b) stride at the wide rung
depends on the strategy's alignment choice, not on the rung alone, which is a `15`-only compiled result
with two controls and no second reproduction, and should carry a rung marker of ONE EXPERT until
someone builds the wide rung `16` says it did not build.

### 4.3 Everything else in the register's "derivation's outputs" section, checked

I checked every remaining paragraph in `OPTIONS.md:715-773` against the source text directly, the same
way as above. The rest holds up.

**"Why a one-output derivation is a real, silent, structural failure"** (`OPTIONS.md:715-733`): matches
`16` sections 2, 5, 7 and `17` section 0 exactly, including the 23.1%/1024-declarations/10-carriers
figures, the "same function" reading of the erasure check's own green result, the small-value-blindness
figures (0 of 64 versus 32 of 64), and the access-width figure (28 of 64). I opened every cited
sub-section and the numbers match. One phrasing choice worth flagging rather than a correction: the
register's citation of `17` for "it reports the Hot and Cold numerals as the same function" is accurate
(`17`'s own words, checked at the passage discussed in section 4.4 below), but the register does not
carry `17`'s further and sharper reframing that the instrument's green result is not merely
uninformative but is **the symptom itself**, stated more strongly than blindness ("its green result is
the assertion that Hot and Cold are the same function," which I read as a stronger and more precise
claim than "the instrument cannot see the collapse"). This is a place where the register slightly
undersells a finding rather than overclaiming one, which is the less dangerous direction but still
worth noting for whoever writes the canon sentence: the instrument does not merely fail to certify the
second output, its passing result is a false positive with a specific, nameable content.

**"Cold is not a container choice with a field attached"** (`OPTIONS.md:746-753`): matches `16` sections
10.2 and 12 exactly, and I verified the "TWO EXPERTS, both self-report independent arrival" marker is
accurate: `15:317-319` reaches it from the standalone-value side ("a lone `UFixed<13,3>` is a `u16`
whatever strategy you asked for") and `16` reaches it from the packed-value side (section 2, "a lone
packed value has to have a size, so whatever the derivation emits for `Cold`, it cannot be a type whose
`size_of` is the answer"), and `16:593-596` explicitly states this is an arrival from "the other end,"
which is genuine independence, not inherited agreement.

**"Whether the two-output shape is forced by arithmetic or only by the type system, blocked on the
`Precise` strategy's undecided semantics"** (`OPTIONS.md:755-763`): matches `16` sections 6 and 12
exactly, including the 0-of-251 versus 64-of-251 figures. This is accurately carried, and it is the
single most important open item in the whole finding, addressed in section 6 below.

**"A separate, standing disagreement never addressed by either file directly"** (`OPTIONS.md:765-773`):
about whether the strategy is upstream of the ladder or a key of it. This is accurately stated as
unaddressed by both `15` and `16`, and I confirm neither file resolves it: `15` builds the map with
strategy as a key from the start (`15:274-282`), and `10` (which predates `15` and is about a different
container attempt entirely, see section 5 below) states the ladder does not know what a strategy is.
Both are true of the artifacts they describe and the register is right that nobody has reconciled them.

## 5. A boundary the register does not draw, and I think it should: two different "container
derivation" artifacts exist in this panel

`10` (Lattner) is fresh eyes on a **different** container-derivation attempt than `15`/`16`'s
three-input map: it is about the closed panel's `137` artifact, a width-to-container ladder plus a
const-literal-to-type bridge, and its central finding is that the bridge (converting a written width
literal into something the type system can compute with) enumerates one impl per width and cannot be
made not to, under the allowed feature set, with three compiled refusals in different syntactic
positions all naming the forbidden `generic_const_args` (`10:372-444`). That artifact does not address
strategy or sign at all; it is purely `width -> container`, one output.

`15` and `16`'s three-input map is a **different construction**: `(strategy, width, sign) -> (container,
stride)`, and it does not use `10`'s bridge mechanism (`15` keys its width side on the same
non-enumerated structural ladder `13` built, per `15:277-279`: "The width side is unchanged from `13`'s
ladder and it is deliberately kept... it mentions no strategy and no sign"). So `10`'s finding about the
bridge's forced enumeration is a real, separate, and still-open cost that sits underneath the two-output
finding rather than inside it: even once you have derived that the map needs two outputs, you still need
`10`'s bridge (or something that replaces it) to get from a consumer's written literal width to the
structural nat the ladder consumes, and `10`'s concession (section 8 of that file) is that this bridge
"cannot be dissolved under the allowed feature set," with the resolution being op's, not a technical
question.

I checked whether `OPTIONS.md`'s "derivation's outputs" section conflates these two artifacts and it
does not: the section is entirely about `15`/`16`'s three-input map and does not cite `10` anywhere in
it. That is correct scoping and I would not change it. What I would add, because it is a real gap: the
register's account of the two-output finding does not say that the bridge problem `10` closes routes
around is still there, unaddressed, one layer below the two-output question. A reader taking the
two-output finding as "the container derivation is solved" would be wrong in a specific, citable way,
and nothing currently on this thread of the register says so.

## 6. Is two the right number, given `43`'s grid finding

The dispatch asks me to establish whether `43`'s finding that "no derivation reads the grid" bears on
what the outputs are. Having now read `43` directly (section not covered by `15`/`16`, since it postdates
both), here is what it actually says and does not say about this question.

`43_probes/p2` (`43:329-355`) tests whether the **carrier**, the **fold accumulator reach**, and the
**law bound** are the same type across three numeral "grids" that differ in adjustment, bias, phase,
and canonical exponent, quantities from `08`'s format concept that are entirely separate from strategy,
width, and sign. All three derivations are confirmed grid-invariant, with three negative controls
including one that proves the type-equality mechanism used to check this is not vacuous
(`43:350-355`). **This is evidence for, not against, the sufficiency of `(strategy, width, sign)` as
the container derivation's inputs**: it establishes that the carrier does not silently depend on a
fourth axis nobody had named. It is not evidence that the derivation is under-specified; it is a check
that closes a possible fourth-input hole and finds it empty, at least for the carrier.

**What `43` does not test, and I checked this explicitly by re-reading its table** (`43:340-346`): the
table lists carrier, fold accumulator reach, law bound, and value map. **Stride is not a row in that
table.** So `43` establishes grid-invariance for the carrier and the fold accumulator, and by extension
of the same argument (stride for non-`Cold` strategies is a function of the carrier's size, per `15`
and `16`'s independently-confirmed repair, and stride for `Cold` equals `W`, which is a strategy/width
fact) stride is very likely also grid-invariant, but this has not been directly checked and `43` does
not claim it has been. This is a small, cheap, specific gap: a fourth arm in `43_probes/p2` asserting
`Derive<S>::Stride` is unchanged across `GridA`/`GridB`/`GridC` would close it, using exactly the
apparatus `43` already built.

**So: two remains the right number for the inputs the derivation is keyed on, and `43`'s grid finding
is corroborating rather than complicating.** It does not add a third output candidate and it does not
suggest the two-output finding needs re-examination on the input side. It closes one plausible objection
(a hidden fourth input) for the carrier and leaves the same check for stride as a cheap, unclaimed item.

**The `Precise` question remains the single load-bearing open item, and here I have nothing to add
beyond what `16` and `35`'s addendum already establish.** If `Precise` widens compute past storage, the
pair `(carrier, stride)` is irreducible by arithmetic alone, not merely by the type system, at 64 of 251
swept extents (`16:696-701`). If it does not widen, the pair's irreducibility rests only on the
const-to-type argument, which is itself solid (a compiled refusal naming the forbidden
`generic_const_exprs`/`generic_const_args`, `16:436-459`), but weaker in kind. `35`'s addendum
(`35:849-856`) notes that op's later intent for `Precise` ("accurate... especially within chains and
ops, not only alone," `38`, quoted in `INTENTS.md` I7) points toward the widening reading without
settling it, since accuracy across a chain of operations is exactly the kind of property that benefits
from computing in a wider intermediate than the storage width. I read `36`, `37`, `38`, `39` directly
myself (not only through `35`'s citation of them) and confirm this reading is available from op's own
words but not stated by him in those terms; he never uses the word "widen" or discusses intermediates
in any of the four files. This is a real, cheap, and specific question for op: does `Precise` compute in
a container wider than what it stores. One sentence answers it and it decides whether the two-output
finding rests on arithmetic alone or also needs the type-system argument.

## 7. Keeping what holds

Per `RULES.md:99-101` and the dispatch's own framing, keeping something is a full result. Having derived
independently from the source text, here is what I would keep exactly as the register states it, with
my own reasoning behind the keep:

**Two outputs, not one, not three.** Established by two independent derivation routes (`15`'s
bug-driven route, `16`'s injectivity argument), with two separate third-output candidates (alignment,
access width) attacked and refuted by compiled or exhaustive evidence rather than by assertion. I would
keep this exactly as stated.

**The second output is keyed on more than width alone**, with the general form (carrier's size drives
non-`Cold` stride) genuinely confirmed by two independent derivation routes, and the specific wide-rung
alignment sub-case resting on one file's compiled result, correctable per section 4.2.

**`Cold` is a statement about composition, not about the standalone value.** Genuinely independent,
both files self-report arriving from opposite directions, and I confirm this reading it directly rather
than trusting the self-report: `15` starts from "what is a lone value" and finds `Cold` and `Warm`
agree; `16` starts from "what does a run occupy" and finds `Cold` cannot have a standalone size. These
are different starting points reaching the same conclusion, which is the strongest shape of
corroboration this panel's own rules recognise.

**The blindness of the panel's own certifying instrument.** Independently derived by `16` and
strengthened, not merely reproduced, by `17`, which reran the actual erasure check (not `16`'s stand-in
for it) and found the same result in sharper terms. This is not addressed in the register's rung
markers at all in the section I was sent to check (the rung marker at `OPTIONS.md:705-713` is scoped to
"the identity of the two outputs and... what the second is keyed on," not to blindness), so there is no
misattribution here to correct, only an absence of a rung marker for a claim that deserves one. I flag
this as a gap rather than an error.

## 8. What I would add to the register

**A correction to the stride-keying attribution**, per section 4.2: split the claim into the
carrier-size sub-claim (genuinely TWO EXPERTS, independent derivation routes) and the wide-rung
alignment sub-claim (ONE EXPERT, `16` explicitly disclaims having tested it), rather than citing `16`
for both under one "independently confirms."

**A note that a second, unrelated container-derivation artifact exists in the panel** (`10`'s `137`
bridge), per section 5, and that its enumeration problem is not solved by, and sits underneath, the
two-output finding. A reader taking the two-output result as evidence the whole derivation is settled
would be wrong in a way nothing currently states.

**A cheap, unclaimed check**: extend `43_probes/p2`'s grid-invariance apparatus to stride, which `43`
built the machinery for but did not run on this quantity, per section 6.

**The single highest-value correction, and it costs one word.** `00_brief.md:145` states the acceptance
criterion with the same singular "representation" that `16` traced through two prior generations of
drift in other documents, and I found is now present in the panel's own founding brief, the one
document every member is told to treat as fixed and required reading. This is not `OPTIONS.md` or
`INTENTS.md`, and per my brief I am not editing either of those; `00_brief.md` is a third file and I am
choosing not to edit it either, since I was not asked to and a founding brief is not mine to touch mid-
panel on my own initiative. But I am naming it as plainly as I can: the plural is the only piece of
op's own language that names a second output at all, and the panel's founding document currently states
it in the collapsed singular form, three generations removed from what op actually wrote. Whoever
maintains that file should fix it.

## 9. What I could not determine

**Whether `Precise` widens compute past storage.** Op has not been asked in these exact terms and his
four files on strategy intent (`36`-`39`) do not settle it, though `I7`'s "accurate... within chains" is
suggestive. This is the one item on which the two-output finding's *character* (arithmetic-forced versus
type-system-forced) turns, and it is cheap to settle.

**Whether stride is grid-invariant.** Very likely, by extension of the carrier's confirmed
grid-invariance and stride's dependence on the carrier's size, but not directly checked by any probe in
the panel. A one-arm extension of `43_probes/p2` would close it.

**Whether `10`'s bridge problem and `15`/`16`'s three-input map can share a resolution**, or whether
they are permanently two separate mechanisms that both have to work. I did not attempt this; it is a
genuinely separate design question from the one I was dispatched to check, and `10`'s own concession
already states clearly what would be needed (a change in feature policy, or the surface's first
parameter ceasing to be a const, both op's calls).

**Whether the register's other sections (`Q1` through `Q17`, the parts I did not check line by line)
carry similar misattributions.** I checked only the "derivation's outputs" section and the immediately
adjacent `Q10` section (read for context, not audited). A systematic pass over the rest of `OPTIONS.md`
against its cited sources, in the style `21` ran against `MORNING.md`, has not been run against
`OPTIONS.md` itself and would be a genuinely useful next dispatch.

## 10. Coverage, bounded honestly

**Read end to end, directly, before touching any summary of them:** `INTENTS.md`, `00_brief.md`,
`RULES.md`, `15`, `16`, `10`, `17`, `35`, `43`, `32`, `36`, `37`, `38`, `39`.

**Read at the specific passages I cite, by opening the lines:** `seed/SETTLED_container.md` (lines
25-40, the claim/quote pair discussed in section 3), `OPTIONS.md` (lines 703-773, the section under
audit, plus lines 57-90 for `Q1` and 775-820 for `Q10`, read for adjacent context), `21` (lines 1-45
and 150-229, to establish what document it audits and what its section 2.2 actually found, since
`OPTIONS.md` cites it as confirming the rung split).

**Not read:** `SETTLED.md` itself (I rely on `16`'s direct quotation of it, checked as internally
consistent with the pattern I independently found one generation further down), `MORNING.md` in full
(read only enough of `21` to establish it is a different document from `OPTIONS.md`), the closed
predecessor panel, `02` through `09`, `11` through `14`, `18` through `31` except as named above, `40`
through `42`, `CANON_CANDIDATE.md`, `DROPLIST.md` (checked by grep only, for entries about the
two-output finding, and found none, which I report as "not on the droplist" rather than "checked and
absent for a stated reason"), `PERSONA_CALLS.md`, `mock/benches/`, and every probe directory except by
opening the specific compiled outputs quoted inline in `15`, `16`, `35` and `43`'s own text.

**Not verified:** I did not re-run any of `15` or `16`'s probes myself. I read the source, the negative
controls, and the reported outputs as they appear in the frozen files, which is the correct treatment
of a probe per this panel's own discipline (a probe is cited for what it proved, checked by opening it,
not re-executed by every subsequent reader). I did not check `35` or `43`'s own probes at all beyond
the passages I quote, since my dispatch is about `15`/`16` and the register's account of them, not about
`35` or `43`'s independent work.

**The specific risk in what remains.** I found one real, narrow misattribution (section 4.2) and
confirmed the rest of the register's account of `15`/`16` against the source text directly. I did not
have grounds to suspect a second one and did not go looking past the section I was sent to check. If
the register's other sections carry the same class of defect, at the same rate, a systematic
`21`-style audit of the whole document would find more, and nothing in this dispatch rules that out.
