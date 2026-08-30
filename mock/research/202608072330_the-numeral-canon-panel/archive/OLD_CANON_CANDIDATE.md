# The canon candidate, carried forward and not trusted

This is `202607301300_formalization-spec-panel/124_consolidation_twelve.md` verbatim, the twelfth and last consolidation of the prior panel. It is the most complete statement the panel produced and it is **stale in named places**, listed below. It is a starting text to be re-derived from, never a citation.

## What is known stale in it

- **The inclusion order.** Its two-condition form is not incomplete, it is unsound: two experts independently found it admits conversions that lose values. The four-condition form (grid, phase, both endpoints) replaces it.
- **The algebraic structure.** Its lattice claim, and the two later answers that contradicted it and each other, all asked the wrong question. The structure is a closure question about the shape space, and the deciding ingredient is whether refinement and reach move together.
- **The adjudicating strategy.** Its claim that a conversion needs no new key does not hold.
- **The sign domain** is not a partition of the order.
- **The strategy cells** are functions of the profile, and every constant in it is one arm rather than the rule.

Its sections 6 and 7 were extracted into `DROPLIST.md` and are not repeated here.

---

# The current shape, twelfth consolidation: the corrections folded, the trail kept out of the way

**Date:** 2026-08-06
**Position in the panel:** after `110_consolidation_eleven.md` and the twelve files that repaired it
(`111` through `123`). This document replaces `110` as the sole reference for the design's current state. It
states its own content everywhere; no section body is a pointer, a summary of a predecessor, or a sentence
asserting that something is unchanged. A reader with this file alone, no directory listing, no citation
followed and no access to any other document, can reconstruct the design. That property is not this
document's invention. It is op's own acceptance test, stated at `13c:38-53` and restated at section 0.5, and
it will be run against this file by readers who have seen nothing else.

The eleventh consolidation was the rebuild. File 109 had diffed all ten prior consolidations against each
other and found **127 items of material that left the standing base with no droplist entry, eleven of them
tracing to text the lead designer wrote or ratified** (`109:13-15`), with a verdict that named the format
rather than any author: "this is the format, not any individual consolidation" (`109:20-21`). File 110
restored what that audit found and adopted the rule that fixes it: a consolidation states its own content or
dies, and "unchanged" is permitted only when followed by the text. Twelve files then went over the result.
Three audited the restoration itself (`111` against the sources, `112` against op's own twenty-three
checkpoint files, `113` against op's numbered decision register, which no instrument had ever touched). One
applied every repair the three identified (`114`). Two read the rebuilt document cold, with the transcripts
withheld, and both returned the same verdict, that it could not be implemented from (`115`, `116`). Six more
closed what those reads exposed (`117` through `123`).

The result was accurate and hard to read. **Fifty-one inline correction blocks** sat interleaved with the
prose they corrected, so a sentence and its repair were adjacent and a reader met the design's history and
the design itself in the same paragraph. This document folds all fifty-one into the prose. Where a
correction repaired a sentence, the sentence now reads correctly and the correction is gone from the flow.
Where a correction added material, the material sits in its own section as ordinary content. **The trail is
not lost**: section 9 records every one of the fifty-one, what was wrong, what it now says, which file found
it, and on whose agreement it landed. A reader who wants to know why a sentence changed can find out; a
reader who wants the design does not read it twice.

Three further things are this document's own rather than a restatement. The **citation form is
round-qualified** throughout, per op's own recovered practice, and the reason it had to be checked before it
could be adopted is at section 0.4. The **cumulative droplist** in section 6 and the **reversals** in section
7 stay under their own separate names, because file 109 found that sixty-six of the eighty-seven droplist
entries ever authored retired an in-stretch proposal rather than a predecessor's text, which made the
droplist report on nothing that had actually left the base (`109:539-574`). And the **open list in section 5
is complete and honest**: an item leaves it when it is answered or explicitly droplisted, never by attrition,
which is the failure file 109 found across three consolidation pairs (`109:487-501`).

---

## 0. The standard, the standing directives, and how a ratification is read

Five of the eleven ratified drops file 109 found are in this section. Op's own standard for the review left
the record at the fourth consolidation, and two of his four standing directives left at the fifth, and
neither was in a consolidation again until file 110 put them back (`109:119-135`). They are stated here
verbatim with their provenance, because they decide questions this document is about to answer.

### 0.1 The standard everything is measured against

Op, `13c:12-14`, verbatim, declining to pick between three offered options on the grounds that the question
was never his:

> This isn't my call. Already the instruction is clear: Optimal, ideal, representative of the math, and
> also, the principle that arvo has to be able to represent mathlab, ieee standard 754, systemc etc etc,
> which means the abstractions are what truly matter, the typestate

Read out into its three parts (`13c:19-24`), every design question in this review is answered by finding the
answer that is:

1. **Optimal and ideal**, not adequate, not conventional, not the smallest change from what ships.
2. **Representative of the mathematics.** The structure the design names should be the structure the
   mathematics has, not one adjacent to it that happens to be easier to encode.
3. **Capable of representing the established systems.** MATLAB, IEEE 754, SystemC and the rest are not
   inspirations to borrow from; they are a **test**. An abstraction that cannot express one of them is an
   abstraction that is not general enough, and that is a defect rather than a scope boundary.

And the consequence op draws from the three together (`13c:26-29`): **the abstractions are what truly matter,
the typestate.** Not the packaging, not the crate graph, not which preset carries which value. Those follow.
A member facing a choice between a cleaner abstraction and a cheaper arrangement resolves it toward the
abstraction and reports the cost rather than trading the abstraction away. A member should not ask what op
prefers on a question of this kind; it has been answered in advance (`13c:31-32`).

The third clause is not decoration. The third consolidation's entire stretch was organised around running
that test (`40:6-9`), and sections 1.6, 1.17 and 1.24 below each record what it found. `40:602-605` carried
it; `49:786-791` is the standing-directives paragraph that replaced it and does not, and it was absent from
every consolidation between (`109:119-125`).

**And the test has a shipping half, which is a separate ratified call.** D67 (from `202607301100`)
(`talk:1454-1470`, op, 2026-07-30): "**arvo writes the abstraction; every established convention ships as an
optional feature defining that convention's vocabulary as aliases over it.**" Generalising D66 (from
`202607301100`) from the quantisation modes to a standing principle across the crate family: `conv-ieee754`,
`conv-systemc`, `conv-matlab`, `conv-amd-vitis`, `conv-flocq` and whatever else earns one, **each off by
default, each containing type aliases and nothing else**, applied wherever it makes sense rather than
everywhere, because the internals stay arvo's own and what the conventions cover is the surface vocabulary a
consumer arrives already thinking in (`arvo-toolbox-not-policer.md`'s own reasoning, `talk:1404-1405`). D72
(from `202607301100`)'s crate table places them: `arvo-numeric` holds "the `conv-*` alias sets" (section
1.25).

**D67's second half is the falsifiable test itself**, which is what `13c`'s standard later restated in op's
own later words and which the three clauses above carry: if a convention's mode cannot be written as an alias
over arvo's abstraction, the abstraction is not general enough, and an axis needing a bespoke impl rather than
an alias has a gap in it (`talk:1467-1470`). D68 (from `202607301100`)'s own note records the test passing
where it had just failed: "`conv-matlab` becomes complete under this" (`talk:1547`).

The two halves are separable and both are live. `13c`'s standard is an **acceptance test for the review** and
says nothing about what ships. D67's first half is a **shipping mandate** naming feature-gated alias sets on
named crates. A sentence absorbing only the first reads as though the whole decision were carried, and that
is how the shipping half went missing for a stretch (`113:152-181`). The `conv-*` features are the concrete
artifact of the one test section 0.1 says every design question is answered against, and they are input to
the taxonomy round.

### 0.2 The four standing directives

All four are op's, all four stand, and two of them left the record at the fifth consolidation while a sentence
asserting they were "restated in the same words each time" carried forward without them (`109:127-135`).
Stated in full from `49:786-791` and `16d`, with each cited to op's own file rather than to the last document
that carried it.

**The intent outranks every instruction, is vague on purpose, and is inferred rather than read literally.**
Op, `16d:27-29`: "none of this should *override* the real intent of the design. Which the existing code also
attempted to solve. It wasn't good enough at that, but the intent, and the reshaped, reworded, one from the
talks if any lead designer calls change it, still trumps all else. The spirit." Where a posture instruction
and the design's actual intent point different ways, the intent wins and the instruction was badly worded
(`16d:33-35`).

**No member resolves to a single angle on anything substantive.** Op, `16d:45-47`: the intent "can be vague
and as such can't be taken literally, just inferred and evaluated against. It's subjective work, which means
the experts can't choose one angle to it." The vagueness is the reason for the rule rather than an awkward
exception to it: two competent readers can infer differently from the same intent and both be reasoning
honestly, so a member carries the readings, says what distinguishes them, and leaves the choice where it
belongs (`16d:49-56`). A member certain about what the intent requires should treat that certainty as a signal
to look for the reading they have discarded (`16d:58-59`).

**Where the current shape can be kept it should be, and rewrite cost is the tiebreaker between designs
otherwise equal against the intent.** Op, `16d:14-15`: "If we can keep the current shape, we should. The best
design is one that sacrifices none of the real design improvements and work, with least amount of work to
write on the current codebase." So "assume everything is rewritten" is a licence to stop defending the
existing code, not an instruction to discard it; a proposal that throws away a working shape for a marginally
better one, or for symmetry, is worse than one that reaches the same place by a smaller move (`16d:17-20`).
The existing code was an attempt at the same intent, which is why its shape is worth keeping where it still
serves, and why treating it as merely wrong is as much an error as treating it as authoritative (`16d:37-41`).
This is the clause that decides between two designs equal on the intent, which is a position this document is
repeatedly in, and it has been out of the record since `58:962-964`. It decides two live questions below, at
section 1.23's `Precision` collision and at section 2's width ceiling.

**Only op's calls are final, and even those go stale.** Carried in every consolidation, including the tenth
(`102:942`). Its content is sharpened by section 0.3, and its operative qualifier is at section 0.5.

**Three further op postures stand and are stated here because they are cited below**, each from op's own file
with the last carrier named beside it rather than instead of it.

**The novelty posture**, `34b:38-69`, carried into the record via `78:790-791`: attempt what looks unsolvable,
and distinguish "cannot, because impossible" (a constraint of the type system, of monomorphisation, of the
forbidden-feature list, or of mathematics) from "cannot, because nobody has done it" (no paper names it, no
library ships it, no vocabulary exists for it), **treating the second as an absence to fill rather than a
wall**. What it does not license is permuting the axes already in hand and calling it novelty, or proposing a
mechanism without compiling it (`34b:66-69`).

**The convergence directive**, `30b:40-57`, carried via `58:962`: where a predecessor's proposal survives an
attack, strengthen it and carry it forward as shape rather than restating that it survived; where it fails,
the replacement is the deliverable rather than the failure; **a file that leaves three more things open than
it closes has moved the wrong way, however true its findings.**

**The constructive-deliverable directive**, `24b:18-30`, restated at `40:610-612`, absent from every
consolidation between the third and the eleventh (`109:316-317`): every member owes a constructive
deliverable, not only findings, and **a proposal offered with stated low confidence beats a finding offered
alone.**

**And the four posture directives from `16b` and `16c`**, which were absent from every consolidation until
file 114 restored them, while the technical half of `16b` survived into the standing base unattributed
(section 1.25's "arvo grows no build harness of its own" is `16b:50-53` and `16c:57-64` in design form)
(`112:228-267`).

**The existing code is irrelevant. Assume everything is being rewritten.** Op, `16b:19-21`: "The existing code
is irrelevant to us. We have to assume we are rewriting everything." A finding of the form "the shipped source
does not currently do X" is not a finding about the design unless the design's *shape* is what makes X
impossible or wrong (`16b:25-27`). Reading source stays legitimate for exactly two purposes: checking a
factual claim in a brief before reasoning from it, and understanding what a mechanism is well enough to design
its successor (`16b:32-35`). This pillar had already decayed once and was repaired by the review itself at
file 69 (`77b:43-44`); the restoration is its second.

**The spec is the subject.** Op, `16c:14-16`: "Make them critique and analyse the spec we are writing. Current
code can exist as extra info about why we are redesigning; it is wrong, broken, insufficient, we know this,
this is why the panel exists." And the reasoning that disposes of a whole category of contribution,
`16c:23-25`: "the premise is, anything that exists nowadays, can be overwritten and shouldn't be analysed too
closely, because the very reason we are reworking things already builds in the implication that the existing
shit is fucked." The current state being broken is the premise of the exercise, not a finding within it.

**Every member owes its boundary a design, not an observation** (`16c:31-53`, the obligation `16c` calls "the
obligation nobody has been given"). For the part of the design a member touches: **how it works for a
downstream target doing the lowering**, concretely, naming what that target reads out of the types, what it
can determine from what it reads, and what it does with it; and **what arvo needs back from that target**
where arvo cannot express the thing concretely on its own side. Op's standard for it, `16c:49-50`: "It has to
be documented and designed for, no handwaved, but we should acknowledge this and answer to it." And the thing
that is not wanted, `16c:57-59`: do not "fault arvo or the design for being unable to express a thing it
fundamentally can't, unless we write our own build harness on arvo end that is a pita to maintain". Naming
that gap as a defect is not analysis.

**Novel answers to a boundary outrank observations that the boundary exists.** Op, `16c:68-69`: "answers to
these problems that I haven't thought of, are 100% welcome and should be considered, instead of stating
existing faults and limitations etc." This is the constructive twin of the boundary obligation and it is the
sentence that makes the boundary worth a dispatch rather than a note.

### 0.3 A ratification is made under the evidence available at the time

`108b:11-20`, adopted, and it governs how everything else in this document is read. Op:

> In general, the things I ratify are made under the understanding and evidence at hand at the time. If these
> two change, then it kind of already invalidates my ratification. Shouldn't read them too literally.

**Adopted, with re-derivation mandatory.** A file building on a ratified sentence whose grounds have visibly
moved **re-derives it before use rather than citing it** (`108b:17-19`). A ratified sentence is not frozen; it
is the best answer available when it was given. Several files have done exactly this and hedged about it, and
the hedging was unnecessary.

This document exercises the principle in two places and says so at each. Section 1.21 supersedes a bench
figure sitting inside `70b`-ratified text, because the evidence moved before the ratified text was written.
Section 1.21 also re-derives `Cold`'s nearest-rounding ground, because op's own stated ground for it depends
on the `Widening` axis, which was ratified out at `39b`. It does not exercise the principle anywhere else,
because nowhere else in the ratified rung has a stated ground visibly moved.

**A supersession that is stated is legitimate under this principle; one that is merely omitted is a drop
wearing better clothes.** That sentence is the reason section 1.2 says in its own first line that the identity
contract's member set supersedes a ratified call, and the reason section 9's ledger exists at all.

### 0.4 Provenance, and the citation form

The **ratified rung** has two parts, and the second was missing from the standing base's own enumeration
until file 113 found it, which is the structural cause of the register drift that file measured: the standing
base's definition of its own oracle excluded the register, so nobody diffed it, and the agreement rate by
number came out at fifteen of forty-six (`113:14-20`, `113:70-90`).

**Op's own numbered decision register**, in the round's three flat topic files. They are op's own text, they
carry their decisions marked "Decision (op, 2026-07-30)" inline, and they are frozen at TOPIC phase:
`mock/design_rounds/202607301100_topic.the-formalization-talk.md`,
`mock/design_rounds/202607301200_topic.the-formalization-spec.md`, and
`mock/design_rounds/202607301000_topic.inherited-state-from-the-formalization-round.md`, which is where the
decisions the first two cite by number are declared (`113:44-50`). **These govern this panel exactly as the
checkpoints do**, and the register is the input to the taxonomy round that follows the canon (`68b:14-21`).

**The panel's own op checkpoints**, twenty-three files, whose own text records op deciding: `04b`, `06b`,
`08b`, `12b`, `13b`, `13c`, `16b`, `16c`, `16d`, `17b`, `24b`, `30b`, `34b`, `39b`, `44b`, `68b`, `70b`,
`74b`, `77b`, `79b`, `82b`, `86b`, `108b`. **A roster entry is not a citation**: seven of the twenty-three
(`08b`, `12b`, `13b`, `16b`, `16c`, `17b`, `24b`) appeared in the tenth and eleventh consolidations exactly
once each, and the single appearance was this roster (`112:112-128`). All seven now have content cited above
and below.

The **persona-decided** checkpoints, which state in their own text that a persona stood in for op during his
absence, are `48b`, `53b`, `57b`, `62b`, `67b`, `90b`, `95b`, `101b`, `106b`: **nine files, and that is the
whole list.** They are agent output on the provenance ladder, individually walked by op at `68b:6-8` (the
first five) and `108b:6-8` (the last four); `106b:4-6` records that it ran at the persona's own tier rather
than Fable's, which is a weaker checkpoint again and says so. The persona checkpoints from this arc (`90b`,
`95b`, `101b`, `106b`) were walked individually rather than confirmed as a block, and section 2 records which
of their calls survive that walk.

Everything else, including every consolidation including this one, is agent output produced without a
recorded human decision. Where an unratified file conflicts with the ratified rung, the ratified rung wins and
the other is drift.

**The register's identifiers are not unique, and the citation form this document uses is the repair.** The
inherited-state file carries **two overlapping `D1` through `D4` sequences**, one at its `:495-644` (the
dimensional foundation, the four new crates, the hlist, curves) and a second at its `:763-798` (the forbidden
features, the container projection, the gate sweep, the vetting audit), with `D14` resuming at `:820`; both
are live and both are cited. The talk file's own question grid additionally runs rows `D1` through `D3`,
colliding with the decision prefix, and there is no decision `D3` at all (`113:32-58`). **A number that is not
unique is worse than no number**, because it resolves silently to the wrong decision: a reader following a
bare "D1" into the talk file reads a forbidden-feature ruling as a shape-crate ratification.

The form is **round-qualified**, written `D<n> (from <round-id>)`, and three things about it are worth stating
because each was checked before it was adopted rather than after.

**Qualifying by file cannot work, and it fails on exactly the collision it would be adopted for.** Both `D1`
through `D4` runs live in **one** file, the inherited-state topic, at `:495` and `:763`. A file-qualified
citation is therefore exactly as ambiguous as a bare one, and it is worse than a bare one, because it reads as
though it had been disambiguated.

**Qualifying by round works, and the reason is structural rather than lucky.** The inherited file is a
concatenation of prior-round summaries under `# <round-id> <title>` headings, so every decision already sits
under exactly one round: run one is under `# 202607281220` (`inherited:293`), run two under `# 202607282100`
(`inherited:719`), and `D14` resumes under `# 202607290050` (`inherited:814`). The qualifier is therefore
**derivable by reading upward** rather than remembered, which is what makes a wrong pairing fail loudly:
`D4 (from 202607282100)` is checkable and false, where a bare `D4` is uncheckable and silently one of two.

**And it is already op's own practice, in his own frozen text, three times**: `inherited:821`, "D1 and D2 (from
`202607281220`) stand"; `inherited:1792`, "the same pattern as `Rect` aliasing rank 2 under D40 (from
`202607291910`)"; and `talk:655`, "D1 and D2, `202607282100`". So the convention is **recovered rather than
designed**, and the form standardised on is op's own parenthetical one.

**Two register-side repairs are declined, for one reason.** Prefixing or renumbering the inherited sequence
edits a file frozen at TOPIC phase, and it would invalidate every existing citation to those numbers across
this document and the panel, which is a larger blast radius than the defect. Assigning numbers to op's
unnumbered decisions mints identifiers with **no definition site in the register**, so a reader following one
finds nothing, and it puts agent-assigned numbers in the same namespace as op's own. Qualification is additive
and touches no committed text; both of those are destructive. **An unnumbered decision is cited by
`file:line`**, and there are two the panel could establish: the two-impl faithfulness derivation at
`talk:1187-1203` and `spec:203-222`, and the four preset intents at `talk:1659-1661`. The other unnumbered
items reported elsewhere are numbered decisions a consolidation carried without their number, which is a
different defect with a different repair.

**The question grid is a third namespace and qualification only half-covers it.** The talk file's grid is
headed `ID` and its rows run `A1` through `E5` (`talk:55-79`), so `D1` through `D3` there are the fourth
letter-group of a question grid, not decisions. Round-qualification happens to separate them, but only because
the talk file's own decisions start at D53, which is a fact a reader has to know rather than one the citation
carries. A grid citation naming itself as one closes it properly. **Naming that marker is op's**, and it is on
section 2's list.

**Applied throughout this document.** Every `D`-number citation below carries its round. The mapping is
mechanical and reproducible by reading upward from each decision's definition site: D1 through D13 from
`202607281220`; the second D1 through D4 from `202607282100`; D14 from `202607290050`; D15 through D18 from
`202607290200`; D18b through D22 from `202607290500`; D23 and D24 from `202607291200`; D25 from
`202607291210`; D26 from `202607291300`; D27 and D28 from `202607291700`; D29 and D30 from `202607291710`;
D31 from `202607291720`; D32 and D33 from `202607291730`; D34 through D37 from `202607291800`; D38 and D39
from `202607291900`; D40 and D41 from `202607291910`; D42 from `202607291920`; D43 through D45 from
`202607292100`; D46 through D49 from `202607292300`; D50 through D52 from `202607300400`; and D53 through D75
from `202607301100`, the talk file, which carries no internal round headings. **The register itself is
untouched.**

### 0.5 How the review runs, how it ends, and what happens after

**Absent from every consolidation ever written before the eleventh** (`112:18-23`, `112:140-182`). Op has
stated the review's termination criteria three times in three files and no standing base carried any of the
three, which is why the earmarking question kept arriving as a matter of opinion rather than as a test with a
procedure. It is stated here in its own subsection because it is the instrument this document is measured by.

**Op's own statement of the mode and the stopping condition**, `13c:38-42`, verbatim:

> Don't poll this. I will literally say when we are done. This current one should be another deep dive like
> the prior ten, into this specifically. It may take another ten. Then after that, we again consolidate and
> start a new fresh eyes based on that, do another 10 or so experts focusing on another area, and we do this
> until our very design is both concrete, valid and critically, ideal, optimal, the dream achieved, nothing
> less will we stop for.

Read out into a repeating four-step cycle at `13c:44-53`, of which the third step is the one this document's
own central property exists to be measured by:

1. **A deep dive.** Roughly ten members, sequential and cumulative, going all the way into one area.
2. **A consolidation.** The area's result compacted into a standalone statement of the shape.
3. **A fresh read.** A member who is given **only the consolidation, with the transcripts withheld**, so the
   next area is chosen by someone not carrying the last one's assumptions.
4. **The next deep dive**, on whatever that read exposes.

And the closing line, `13c:52-53`: "Repeating until the design is concrete, valid, and ideal. **Nothing less
is a stopping condition, and no member should treat running long as a reason to converge early.**"

**Step three is the acceptance test for a standalone consolidation, and it is op's, not this document's.** The
eleventh consolidation independently arrived at the property that step consumes and named it as its own
invention, because the procedure that tests it had left the record eight consolidations earlier. The
instrument has now been run twice. The first run was at file 12, and `12b:18-21` records that it produced the
widest finding in the review: a missing axis, an entire absent area of the taxonomy, and a rule collision ten
prior members had walked past. The second run was files 115 and 116 against the eleventh consolidation, two
readers working independently who never saw each other, and **both returned the same verdict, that it could
not be implemented from, and both stopped at the same place, which was not a design question**: `Number<N, S>`
had no declaration anywhere in 5,867 lines, and around it sat a vocabulary of names used as bounds with
nothing to bind to (`115:23-35`, `116:64-70`). File 118 closed that gap. **Judging a consolidation by hand is
not the same act**; the fresh read is a dispatch shape with the transcripts withheld, and it is the third step
of op's cycle rather than a review courtesy.

**Op's checkpoint cadence.** `04b:42-43`, verbatim: "Let's get similar checkpoint with me every 2 experts,
too", read out at `04b:19-21` as "Op takes a checkpoint like this one after every two experts." The rhythm
actually run drifted twice: `77b:101-105` restates it as "the four-checkpoint-four-checkpoint-consolidate
rhythm" and `86b:50-53` as "four more on the open list, then consolidation nine" (`112:347-363`). **The record
cannot show whether the drift from two to four was op's or the dispatcher's**, because neither the instruction
nor either restatement had ever been in a standing base for a member to notice drift against. It is stated
here as instruction, and the discrepancy is on the open list as op's to settle.

**Op's licence to argue against a ratified call.** `04b:72-74`: "Any member is free to argue against any of
them, including the ones this file has just reaffirmed, **provided the argument is made rather than
asserted**." The standing bases carried the first half of the pair ("only op's calls are final, and even those
go stale") and not the qualifier, and the qualifier is the operative half: it is what makes a reopening
legitimate rather than presumptuous. Section 1.27's reopening of the array grammar's forcing argument is
authorised outright by this sentence, not merely tolerated under section 0.3's re-derivation licence.

**What happens after the canon**, stated twice by op three checkpoints apart. `68b:14-21`, verbatim:

> we are strictly designing here... we want to formalise, fully define, the ideal shape to set as the new
> canon. This is our job. Not implementing anything. We will settle the canon in full, and then start a design
> round about the new settled taxonomy, creating it and its docs, then implementing in source it as stubs.
> Then we'll start doing design rounds where we go through the settled canon piece by piece to implement it
> into the stubs. But we will not go there until the full design is settled, the spec is complete and answers
> all, and we can earmark it as the first full canon in arvo, to guide all future work.

So the sequence has four phases: **settle the canon in full; a design round creating the settled taxonomy and
its documents; source stubs; then design rounds implementing the canon piece by piece into those stubs.**
`79b:64-69` states the same four phases and binds the verification mandate (parity suites, exhaustiveness in
both directions, red as the starting state) to **the last two**, "recorded now so it is not rediscovered late
or watered down when the volume becomes apparent."

**The taxonomy round reads this document.** That is the direct consequence, and it is why the register
restorations below matter: the round that builds the crate structure will be briefed off the standing base, so
an op decision about the crate structure that is not in the standing base is a decision that round will
re-invent (`113:126-128`).

**Op's statement of the end state**, `70b:52-57`: settle the current focus, then explore for more, then close
the findings, then explore again, **alternating until a full spec emerges that is proven, valid, and
importantly efficient and ergonomic**, in op's words invisible for the most part to downstream consumers while
doing real work underneath and **lowering transparently to optimal instructions.**

This is a criterion rather than a process note, and it is the consumer-facing half of the bar. Section 0.1's
standard covers optimal, representative and representable; this covers invisible and ergonomic. **The two
together are the standard**, and the standing bases had one of them. Two live items below are decided against
the consumer-facing half rather than against the abstraction half: section 2's width ceiling, where a
refusal's legibility is the deciding cost, and section 1.23's `Precision` collision, which sits on the exact
step a first implementer reaches.
---

## The four design rules, and the three requirements that check the review's own models

Unchanged in count since the tenth consolidation, and every finding since has filed under one of the four
without needing a fifth. All four are stated in full here rather than named, because three of the four had
been carried as one-line stubs since the ninth consolidation and the pricing pillar's own site list left the
record entirely at `102:90` (`109:32-42`).

### The spine rule

**A quantity that is computed and then has to appear in a type is a type; a quantity that only ever has to be
read is a const.** **Eleven occurrences stand, and they are enumerated rather than counted**, from
`63:106-123` (the last document to carry the list) and `78:120-129` (the two added since). The enumeration
matters for its own sake: the design's strongest methodological conclusion is drawn from this number, and its
provenance had run through `78:120-129` to "seven through the eighth (`68:98-101`)", where `68:98-101` in fact
reads "**Nine occurrences stand from the sixth consolidation's count**", so the chain terminated in a count
inherited from a document rather than in a list (`111:261-277`). A count cannot be checked and a list can.

1. **The width chain.** Founding, op's own at `44b`.
2. **The biased-product formula.** Founding, op's own at `44b`.
3. **The fold's `Grade`** (files 47 and 48).
4. **The `Ranged` exponent bounds**, reasoned by file 48, compiled by file 50.
5. **`Implicit`'s single exponent**, compiled by file 54, overturning file 36's own contrary claim.
6. **`Capacity`'s size** (file 55), the first firing outside the `Numeral` contract entirely, at a different
   crate and a different layer.
7. **The notation macro's const-struct face**, sealing a computed reduced-fraction condition (file 56).
8. **The notation macro's magnitude**, a second and structurally distinct wall inside the same macro, compiled
   separately by file 61: a value-to-type escape for a literal's digits, refused identically to the exponent
   case.
9. **The shipped `arvo-strategy` container dispatch and its facade**, where file 59's reduced probe and file
   62's whole-crate compile (sixteen refusal sites in `arvo-strategy`, four hundred seventy-eight in the
   facade) confirm the wall is the reason two live crates carry a forbidden feature gate today.
10. **The print buffer's `ShortCap`** (file 72, `72_probes/probe_3`), refusing under the naive
    const-expression spelling with `generic_const_exprs` named in rustc's own help text, shipping as an
    associated type with a declaration-site coverage assertion instead.
11. **The byte buffer's `ByteCap`** (file 73, `73_probes/probe_1`), the identical refusal shape.

Eleven independent firings of one rule across unrelated quantities (grade projections, notation faces, seal
witnesses, container widths, text and byte capacities) is evidence the rule is a property of this design's
shape rather than a coincidence noticed repeatedly. No new instance and no new attack since.

### The carrier-at-birth rule

**A closed vocabulary that a guarantee quantifies over owes its seal and its adversary at birth, not after
three passes** (`78:131-135`). Unchanged in statement. The `NicheCarrier` candidate from the ninth
consolidation is now closed material at a narrower name, `NonZeroCarrier` (section 1.12). The capacity work is
downstream of the rule rather than a new instance of it: the shared bottom carrier the tower already sealed at
`44b` is what the capacity unification reaches for, not a fresh vocabulary needing its own seal.

### The layer-keying rule

**A fact is keyed on the coarsest layer whose identity its truth depends on** (`78:137-150`). Instances, in
order of arrival: `TotalOrd`'s level fork; the `arvo-spectral` NaN-classification defect; the digest, where a
datum-keyed digest pairs with the datum's total order and a value-keyed digest pairs with value equality, and
mixing the pairings breaks the consistency law either direction it is mixed, compiled; and carrier identity, a
**third** identity notion beneath the rule's own face/encoding-equals-value pair, strictly finer than datum
identity rather than coarser, on which almost nothing should ever be keyed because the padding bits it
distinguishes carry no denotational content by construction.

The rule's own display clause carries a completion: "a fact depending on where something was written belongs
on the face" quantifies over a layer that does not survive to runtime, since a face cannot reach a numeral
position and a computed value has no face by the time it exists. At runtime the honest split is a
**value-keyed display** (canonical, shortest round trip) against a **datum-keyed debug image** (raw fields,
NaN payload, cohort member), decided by the rule's own coarsest-layer test applied a second time to its own
clause (`78:145-150`).

**The rule has a dual failure, and it is named at section 1.30 rather than only illustrated there.** The
primary failure is a fact keyed too finely, which the rule calls a false statement. **The dual failure is a
fact keyed on something that does not determine it, which is not a false statement but a non-function
presented as one.** Both are live, and section 1.21's preset key is the one place where both fire at once,
which is what makes its answer forced rather than chosen.

Two further instances stand. The shape and storage split (section 1.28) is the `Lowering` charter's own
"changes no value, `Encoding` may change which datum carries it" statement applied one dimension up, to a
shape's rank and extents against the bytes that carry them (`102:85-88`). And section 1.29's byte-sharing law
is the rule applied at a composite, where the fields side is a list rather than a scalar.

### The pricing pillar

**Runtime and lowered code are the measurement. Compile time is not a cost to be minimised; it is a resource
to spend, without a ceiling stated in advance, whenever spending it buys a runtime saving, a soundness
property, or a correctness guarantee.** Op's own words, quoted rather than paraphrased because the wording is
load-bearing (`78:155-158`): "Compile time is nothing. That can be literal minutes for all we care... the
important measurement is the actual runtime and lowered code... We *want* long compile times, if it resolves
to snappy optimal runtime with the extra soundness, safety and numeric machinery amortized fully at compile."

And the sharper claim that does not follow from the general workspace rule and therefore earns its own
sentence: **a strategy marker changes what happens at runtime. It never changes how much is amortised at
compile or const time** (`78:159-161`). All four presets verify to the same depth; they differ only in what
they then emit.

**The standing test a member runs against any proposed mechanism** (`78:162-166`): does anything this design
does at runtime have a compile-time or const-time alternative that was rejected, and if so, was it rejected
because it does not exist under the permitted feature set, or because someone judged the compile cost too
high? Only the first is a real constraint; the second is the violation.

**The guard clause, carried explicitly because its absence is what let the rule's wording decay**
(`78:168-180`). `arvo-compile-time-last.md`'s own corrective section names and forbids the misreading this
pillar's name invites: "compile time last" states which cost is minimised least urgently, not which cost is
pushed downstream, and it does not license preferring a runtime check because it is cheaper to compile. Op's
audit found the rule has never been violated in substance; what had decayed was wording, and the guard clause
was quoted nowhere in seventy-seven files. It is stated inline here so the next reader does not have to
rediscover file 76's ordering to find it out.

**The sharpened clause, and the site list that goes with it.** Ratified content from `91:113-126`, absent from
the tenth consolidation entirely (`109:32-42`): rustc guarantees const evaluation in a const position and
nowhere else; a `const fn` called from value position folds or does not fold at the optimiser's discretion,
and on this target it left division residue in both a standalone probe and an in-loop decode (`81:220-239`,
`82:456-506`, compiled). The standing test this earns:

> Is a quantity computed inside a per-element or per-step loop a function of the type's parameters alone? If
> so it belongs on the type as an associated const, not a `const fn` called from value position, and it names
> the width **level** it is a function of, because two levels coinciding at the one preset everyone measures
> is exactly how a compile-time fact computed from the wrong level survives review (`83:290-316`).

**Known sites**, with their level: the bitpacked decode and encode plans (on the stored width), the value mask
(on the fields), the far point and the write granule (**unchecked, flagged**), the digest's own field mask (a
second consumer of an already-named site, **unchecked**). **A known non-site**: `Encoding::Canonical`'s
trailing-zero removal, genuinely data-dependent. Two of the four sites were flagged unchecked when the clause
was written and remain unchecked; the list is what makes them findable, and losing it lost not the rule's
sharpening but its own record of where it has not yet been applied.

**Three sites added at the tenth consolidation** (`102:90-95`), each a quantity that is a function of the
type's parameters alone and belongs on the type as an associated const: the capacity pairing's own agreement
fact (section 1.28), the environment receipt's per-field fold (section 1.27), and the hardness constant a
transcendental's correct rounding needs (section 1.16, the one place the pillar's own test is passed to the
letter while the value itself resists computation). **One added since**: the bitfield's containment check,
which moves to an emitted free const item (section 1.29).

### The definitional-completeness line

*When a structure is ratified, every term in its definition, including the name being defined, is either
defined or named open in the ratifying text.* Adopted at `90b`, widened at `95b` to reach the name being
defined, on the ground that a name defined twice with different content is defined nowhere, which is the
mechanical form of file 94's naming finding and requires no separate naming rule to state it
(`102:97-103`).

**The moment, adopted at `95b`**: the line is performed by the author of the ratifying text, on that text,
before it stands, and the performance is reported rather than cited, in the same shape the table-diff
obligation already has. The evidence for the moment is blunt: the ninth consolidation's claim to have applied
the line to everything it absorbed (`91:12-13`) is false at three of its own sentences, grep-checkable,
because the line had fired zero times in advance across the whole corpus before that stretch.

**And the line quantifies over the whole ratifying text rather than over its new prose**, which is not a
detail. The eleventh consolidation first scoped it to "this document's own new prose", which exempts exactly
the population a restoration consists of (`111:503-520`). With the exemption struck, six terms that document
used and defined nowhere fell out at once, and all six are disposed of in this one: the spine rule's eleven
firings, the ten axes, the `Resolution` axis's four members, `Quantisation`, `Direction`, and the transfer
argument's second compiled support. A seventh, op's axis-sorting test invoked by name and stated nowhere,
came out of the register diff (`113:310-315`). Section 10 performs the line over this document as a whole.

### The separation requirement

*A claim about a distinction is checked at an instantiation where the distinction is nonvacuous, and every
model states what it separates.* Adopted at `86b`, given the same owner-and-moment clause at `95b`: performed
by the author, on their own models, reported rather than cited (`102:110-117`). The evidence for keeping it
rather than replacing it is that it caught the largest finding of the ninth stretch in its own first hour of
existence and, applied to the tenth stretch's three one-pass files, it is the requirement that worked, missed
only because nobody had run it in advance. The correct response to a requirement that works but goes unrun is
a moment naming when it runs, not a new requirement.

### The freshly-performed-search requirement

*In an audit-shaped deliverable, every universally quantified negative claim ("no file has", "never", "nothing
touched", "unexamined") carries its own freshly performed search, quoted with its date, the way a judgement
carries a citation. Citing a search performed by an earlier file is not performing one.* Adopted at `101b`
(`102:119-130`). Not a fifth design rule; a requirement beside the separation requirement, costing one search
per negative claim. The evidence is the worst single finding of the tenth stretch: file 98 reported the
review's periphery as unbounded, unexamined ground on the authority of two files whose own searches had gone
stale, plus an `ls` of the design-round directory whose return value named the exact file needed and was never
opened. Both existing requirements passed genuinely on that file, because both are inward-facing: they audit a
file's own terms and models, not the sources it failed to read.

**And the requirement's own limit is now measured rather than asserted.** A search verifies a negative in the
searcher's vocabulary and nothing wider. This stretch produced **six demonstrations that the vocabulary is
what fails**: a bolded interposed word defeating an exact-phrase search, a transposed filename pair, a
`76_probes` row found only because one search of five used a word file 76 happened to use, `fires at use`
against a source that says `firing at use`, and two further verdicts that changed when a second vocabulary
found what the first had not (`112:487-505`, `113:450-457`). A second reader with different terms is the only
check on it.

---

## The grounding registry, stated because two consolidations used it everywhere and defined it nowhere

Eleven `*Grounded on:*` footers in the tenth consolidation use `ratified`, `settled shapes`, `compiled`,
`measured` and `reasoned`. The five rows those names come from and the four transfer-ground members are named
at `78:343-345` and at **zero** places in `91` or `102` (`109:97-105`). The apparatus that grades every claim
in this document is stated here in full, before section 1, because that is where a reader reaches it first.

### The five grounds

| kind | rung | examples |
|---|---|---|
| ratified decisions | op-ratified, governing | `d69`, `vu`, `enc`, `seal-owed`, `div-held`, `grounding` |
| settled shapes | panel-settled, presumed correct, overturnable with evidence | `round-first`, `crossing`, `bias-rational` |
| physical grounds | facts about the environment, change by act not argument | `pin`, `host`, `flags`, `model` |
| tree grounds | facts about the shipped source at a commit | `tree` |
| unreproducible | derived once, derivation not rebuildable from the committed trail | file 59's gitignore finding |

Table from `63:443-449`, the last consolidation to carry it before the eleventh (`109:396-399`). The
`unreproducible` ground's original exhibit (file 57's claim that file 8's five-shape instruction table could
not be rebuilt) was **struck** at `63:441`, refuted by file 62 following a recipe committed in the same
directory; the ground itself survives on its second exhibit, file 59's discovery that no bench artifact in the
repository had ever been committed (`63:464-468`).

### The four transfer grounds

Every claim established by bounded exhaustion at a model instance names the index set it is quantified over,
coordinate by coordinate, and carries one transfer ground per coordinate, drawn from a closed, sealed
vocabulary of four (`68:457-466`):

| ground | what it asserts | who supplies it |
|---|---|---|
| `symmetry` | an exact group action carries the model instance onto every target instance, under a stated condition | the claim's author, once per axis |
| `saturation` | the claim's dependence on the coordinate stops changing past a stated threshold, and the model's coordinate clears it | the claim's author, threshold stated |
| `induction` | the claim at `t + 1` follows from the claim at `t` by a stated argument | the claim's author, in prose |
| `unargued` | the claim is a fact about the model instance and nothing else | nobody: the default when no ground is named |

**`unargued` as the default is what makes the scheme honest: a claim naming no ground does not silently
inherit one** (`68:468-469`). The vocabulary is sealed at birth per the carrier-at-birth rule, citing file 64's
`Arity` result as the reason to seal a vocabulary the review is proposing rather than repeat the mistake on
it. That sentence left the record at the seventh consolidation (`109:102-105`); it is the sentence that makes
the scheme mean anything.

**Worked per coordinate for a `Ranged` numeral** (`68:473-488`). `EMIN` and `EMAX` carry `symmetry`: the
quantiser commutes with scaling a value by `r^k` when the window shifts by `k`, checked over **509,660,160
instances** (every value and every exact pairwise sum, two radices, two precisions, four spans, both underflow
policies, five shifts) with zero failures, plus two negative controls that both correctly disagree: **a
window-only shift disagrees on 8 of 13 values, and adding a nonzero additive constant to the value map breaks
the symmetry on 29 of 51 checks.** The condition, that no `Numeral` member contributes a nonzero additive
constant to the value, holds today because `Ranged` carries no `Bias` member; the day `Ranged` gains one, the
symmetry dies silently unless the condition is written down, which it now is. **Two of the six coordinates
collapse into one: only the span matters, not the absolute position of the window**, which is what makes the
index set six coordinates rather than seven and is load-bearing for anyone re-running the argument.
Multiplication is equivariant into a window shifted by `2k` rather than `k`, exactly `mulnum`'s own
construction. The span carries `saturation`, at a measured threshold of `p + 1` under `Abrupt` and 2 under
`Gradual`, independent of `p`, **and the sixth consolidation's own models (file 50's fold at span 8, its band
model at six binades) cleared that threshold by luck rather than by design, since nothing told their authors
what the threshold was.** **Precision and radix carry `unargued`**; no induction argument exists for either,
and the radix is known genuinely non-uniform.

The figures matter and so does the last sentence. A restored measurement without its numbers is a restored
assertion, and the cleared-by-luck sentence is a statement about the reliability of the review's own prior
work, which is exactly the class of statement this exercise exists to preserve (`111:151-168`).

**The third coordinate the mechanical bans do not close at all: container class** (`68:490-504`).
`arvo-strategy/src/container.rs:254-280` projects a width through a const-tag dispatch to a distinct
associated-type container. This is a type observing which instantiation it is in and behaving differently, is
permitted (no forbidden feature, no gate), and is shipped. Compiled: one parametric body, no specialization,
no `TypeId`, a property TRUE at eight bits (`u8` wraps on doubling 200) and FALSE at nine (`u16` does not).
The forbidden-feature bans close the ways an instantiation can get a different *body*; they never closed the
ways it can get a different *type*, and this is that third way. It takes a `saturation` ground with the
cleanest threshold in the scheme, one width per container class, read straight off `tag_hot_cold`.

**Op named this defect class at the fourth checkpoint and it was rediscovered forty files later.**
`12b:46-54`, adopted at the time: "**The verification spine gates the crate relocations.** Adopted. File 12
established that the spine has never met `Bits`-backed storage, and that the argument letting a check at a
small width stand for a large one **does not cover `WideBits` limb arithmetic, whose code path diverges per
bucket, a hole invisible to the checking apparatus by construction.**" The gating half is superseded and
correctly: `68b:23-28` puts all source work out of bounds, so no crate moves for a larger reason. **The
technical half is not superseded**, it is the same hole at the same mechanism, found independently by file 68
and credited there. `WideBits` had been in no consolidation since `26` (`112:300-322`). **The cost of that
drop was not the sentence, it was the rediscovery**: the review spent files 66 and 67 building the
transfer-grounds scheme around a defect class op had already flagged, and nobody could see that he had.

**Twelve distinct container types exist across the strategy markers** (six classes for `Hot`/`Cold`, five for
`Warm`/`Precise`), and every model claim this review has run, including file 50's 41-million-operation
binary32 check and file 64's exhaustive eight-bit `TotalOrd` matrix, exercises exactly one of them, none
saying so. Classes above `u32` are `unargued`, and the spec states that in those words. The owed companion is
a nine-bit `u16`-class model at `2^18` pairs, named owed by four consolidations and still unbuilt (section 5).
This coordinate left `78:490-504` with nothing in the eighth consolidation stating what the owed companion
model is for (`109:469-471`).

### The `tree-fact` and `tree-meaning` split, and the prohibition

Adopted in full at `70b` (`78:341-380`). The split is required at the point of citation, not retrofitted at
consolidation time.

**The defect that motivates it.** File 59's strategy-door table justified three of its four rows by quoting
shipped doc comments verbatim, under the header "shipped meaning," with the sentence "every row below is
derived from what the preset already means for fixed-point arithmetic in the shipped tree." Op's correction:
shipped source and its comments are deprecated and wrong on the new design by the review's own founding
instruction, and the design is fully free to restructure what a marker means. File 69 traced the propagation:
one origin (file 59), three carriers that repeated the claim without re-deriving it (`62b:166`, `63:576-591`,
`68:534-535`), and one file that touched the neighbourhood and correctly stayed silent because its own check
was narrower than what it sat beside (`64:406-413`). The review's existing verification machinery checked that
every citation resolved and traced to a real probe; it had no mechanism for checking what kind of thing the
citation was evidence **for**.

**`tree-fact`**: the shipped source establishes that a mechanism exists, compiles, or currently behaves a
stated way. Licensed for any claim about current state, and licensed as input to a design conclusion only when
the conclusion is argued independently and the citation is offered as corroboration that a needed piece
already exists. The worked example is Lamport's `TotalOrd` use at `33:186-198`: the mathematical argument for a
total-order-keyed law stands first, the doc comment only confirms the mechanism is already declared.

**`tree-meaning`**: the shipped source's own prose is offered as the reason a design construct should mean what
it means. **This ground is forbidden. No claim may carry it** (`78:368-370`).

This prohibition was adopted in full at `70b`, compressed to a name in a list at `91:485-486`, and was absent
from the tenth consolidation entirely (`109:87-95`). It exists because a shipped doc comment was used to
justify three rows of a design table, which op corrected personally, and a future member reading only that
consolidation had no way to know the ground is forbidden. It is the drop with the highest chance of being
repeated, which is why it stands in its own paragraph rather than folded into a list.

**The mechanical test**, run at the point a member writes the sentence and again at consolidation time
(`78:372-380`): does the row's justification survive if the citation is deleted and only the design's stated
intent remains? If yes, `tree-fact`, and the row stands. If no, the row was never grounded in the design, and
the consolidation says so rather than compressing a member's confident phrasing into settled fact.

### The two provenance classes added since

**`exhaustively-computed-or-cited`**, adopted at `101b` (`102:503-505`): for a constant that passes the pricing
pillar's test while resisting its usual derivation economics, computable only by exhaustion over the value set
or by citation of a published worst-case search. Where cited rather than exhausted, the constant is a
trusted-base entry with the citation as its named artifact, the same accounting as every hand-laid `Crosses`
entry. Its instance is the transcendental hardness constant (section 1.16), and its second use is a foreign
bitfield's correspondence to its datasheet (section 1.29).

**Three process disciplines on the registry's own upkeep** (`91:483-494`), all standing: an owed item names the
artifact whose existence would close it, so closing is a grep rather than a memory (`82b`); a primary-source
closure names its document, edition, and position inside the file that closes it, since a probes-directory
grep does not substitute for re-fetching a source (`82b`); and a count in a member file names the command that
produced it, adopted at `90b` after three consecutive files published a count nobody re-derived.

**The registry's own perimeter**, from `58:727-729` (`109:375`): no tier detects an unwritten grounding, and
the residual is caught only by the act of writing the field, the same act that catches a stale one. The two
tiers named and not built (a mockspace registry namespace; a probe-header line) stand unbuilt.

### The conventions the archive has adopted about its own claims

Collected here because each was adopted inside a specific finding and three of them left the record.

- **A universal "cannot" claim owes an exhaustive read of the committed trail before it ships**, the same
  discipline a "cannot compile" claim owes the whole matrix rather than a sample (`63:470-474`), adopted after
  file 57's universal unreproducibility claim was refuted by a file one directory listing away.
- **A universal "only" claim about the shipped tree owes a whole-crate compile before it ships**, symmetric
  with the above (`68:678-681`), adopted after a committed sketch's "the facade's only live GCE constructs are
  two static asserts" was refuted at two of 478 spans. One `cargo check` with the gate stripped costs four
  seconds and would have caught it. This convention left the record at `78:12-16` (`109:432-434`).
- **A dispatch brief names `mock/research/sketches/` explicitly in its surrounding-directories listing**
  (`68:681-683`), adopted after a committed sketch holding a third of an answer sat one directory listing away
  for two dispatches. Also dropped at the eighth consolidation.
- **A measurement of a declaration's price states which bounds it forces, and two arms are comparable only
  when they force the same ones** (`68:412-417`). The pricing hazard's own control, derivable rather than a
  trick to remember: an unused alias forces no bound, `Reduce` never runs, and the type checker does the work
  its instantiation demands and no more, which is monomorphisation behaving correctly rather than a
  benchmarking artifact. Dropped at `78` in the same document whose central new measurement is a declaration
  price (`109:425-427`).
- **The table-diff obligation** (`53b`, executed by every consolidation since): every table is checked against
  the prose of its section and against the source file that established each row, by the document's own
  author, before it stands.
- **A claim that a compression entails the prior text is checked by someone other than the author of the
  compression**, because the author of the compression is the person who believes it entails (`109:610-615`).
  The eleventh consolidation was the first attempt at satisfying it and satisfied it imperfectly; three
  parties have since audited that restoration, and section 8 records what each could see that the one before
  it could not.
- **A restoration cites the source that established the statement, not the last document that carried it, and
  where the two differ the restoration says so** (`111:544-551`). Both entailment failures file 111 found were
  paraphrases produced while restoring from the document that last carried the sentence; three further
  instances landed on op's own text (`112:414-427`).
- **The blocks a specification states are compiled as an assembly rather than in isolation, and the assembly's
  own preconditions are stated** (`121:101-134`). A specification whose declarations are distributed across
  its sections has no block that builds alone, and reporting that as failures reports on the format rather
  than the content. What has to be stated is what the assembly needs and the document does not say: section
  1.23 states the three additions at the first block that needs them.
---

## 1. The agreed shape

Thirty subsections, `1.1` through `1.30`, which is a range a reader can check against the headings below
rather than a count to trust. Every one states its content.

### 1.1 What a number is

**A value of `Number<N: Numeral, S>` is an integer `k`, drawn from a finite interval, together with a
type-level rule injecting `k` into a set of rationals** (plus, for floats, a handful of data that are not
rationals at all: `Specials`, section 1.16). The numeral has two jobs, naming the representable set and naming
the indexing, and D69 (from `202607301100`) put those two jobs on two different sides of the design rather
than deriving one from the other (`40:41-46`, `68:147-151`).

The phrase "integer k" returned zero hits in `78`, `91` and `102`: the design's central object was undefined
in the standing reference for three consolidations (`109:445-448`). The type itself was declared in none of
them, which is the gap both cold reads stopped at (`115:23-35`, `116:64-70`).

**The declaration.** This is the form a reader implementing today takes:

```rust
pub struct Number<N: Numeral, S: Policy<N::Exponent> + Lowering<N::Exponent>> {
    datum:    <S as Lowering<N::Exponent>>::Container,
    _numeral: PhantomData<N>,
}
```

Exit 0 under the pin. **Two parameters, and the second implements both strategy-side contracts, each keyed on
the numeral's own exponent form.** Three separate questions meet in that line and each is settled
independently.

**The arity is closed, on measurement, against the split.** Section 1.3 states that `Encoding` nests inside
`Lowering` **so that a third parameter is never paid**, resting on `26:32-35`'s measured 1.8x rendered
diagnostic cost, and D72 (from `202607301100`)'s crate table gives `arvo-numeric` the row "`Number<N, S>`".
File 117 separated the two questions the record had been reading as one (`117:47-52`): the **parameter count**
closed on that measurement, and the **trait count** is what op reserved at `08b:47-51`. Only the second was
ever open, and section 1.25 records that it is now settled too, on two independent reads.

**The bound is `S: Policy + Lowering`, and the conjunction is load-bearing rather than notation.** File 118
wrote it from `26:28-35`, the only statement of it anywhere in the record, and named itself the first read
(`118:321-324`). File 120 supplied the second from its own probes and gave it a reason the record did not have
(`120:386-390`): the conjunction is not shorthand for "S does both", it is **what makes the weaker bound
`S: Policy` a meaningful and enforceable thing for a law crate to write**. A `Lowering` member projected off a
`Policy` bound is refused at `E0220` whether or not the lowering crate is linked. **The bound and the
supertrait prohibition at section 1.25 are the same decision seen twice**, and they belong in one sentence.

**The exponent-form key is the one part that moved, and section 1.21 carries its derivation**, the two refuted
alternatives and the compiled evidence. In short: a preset name denotes two rows, one per number kind, and a
nullary associated type on a marker cannot be a function of two things; the kind **is** the exponent form,
since `Numeral::Exponent` is bounded by `ExponentForm` and `ExponentForm` is sealed with exactly the two
constructors `Implicit` and `Ranged`, against exactly two ratified tables. Keying on the numeral itself
over-keys and admits two same-kind numerals disagreeing on one preset; per-kind markers admit a well-typed
nonsense and delete four types a ratified decision names. The exponent form avoids both.

**The prior spelling, reproduced because an auditor should see what was proposed when.** Before the key was
derived, the declaration read:

```rust
pub struct Number<N: Numeral, S: Policy + Lowering> {
    datum:    <S as Lowering>::Container,  // the container level, W_C (1.22)
    _numeral: PhantomData<N>,
}
```

The two forms differ by exactly the projection. **Op's confirmation of the key is owed** and is on section
2's list, narrowed from a choice between two spellings to a confirmation of the one that survives, with both
alternatives refuted by compiled diagnostics rather than by preference. **What does not change either way**:
the arity, the consumer's spelling, the four preset names, `arvo-strategy`'s contents under D72 (from
`202607301100`), and every crate edge.

**`Container` is the container level's own type**, the type-valued projection of the stored width through the
dispatch menu that section 1.22 states is derived and never declared. It is spelled as a `Lowering` member
because that is where its inputs already are. **Whether it stays there or moves to a one-member carrier
contract below `Lowering`** is `117:452-458`'s second rider and is on the open list; the declaration does not
move either way, only the path the projection travels. File 120's own reading is that the carrier contract is
also where the residual discrimination surface closes, and its spelling rule is the load-bearing half: the
carrier's inhabitants must not be nameable from the law crate, which a primitive always is.

**A three-argument spelling appears once in this document and is not a second arity of this type.**
`Number<Fix13_3Signed, Warm, MinWidth>` at section 1.25 is file 09's own probe topology, where the lowering is
a third free parameter. Both cold reads took it for a second arity of the shipped type and could not reconcile
it with section 1.3 (`115:187-196`, `116:219-224`); file 09 was working a three-parameter topology throughout,
which is also why its own closure does not transpose to the fused form unaltered (`117:230-237`). It is marked
as such where it occurs.

**D65 and D69 (both from `202607301100`) were both overturned by op at `30b`**: identity is parameterised in
mathematical coordinates, not encoding coordinates. Precision and the exponent bounds are primitive; total
width, the hidden bit, and field encoding are derived on the physical side. **The sentence being overturned is
D65's**, `talk:1394-1400`, which had made precision and minimum exponent derive from the field width "which is
how IEEE defines its interchange formats"; **D69's own content is the ten-axis table with `LogicalWidth` on
`Numeral`** (`talk:1621-1641`), overturned at the same checkpoint by the same correction. Two independent
readings reached this (files 27 and 28, formed independently, one from the shipped facade's own declaration,
one from Flocq's two-sided float formalisation that CompCert ships on), which is the threshold this review's
discipline requires before a call of this kind reaches op. The standing consequence: the off-by-one against
real hardware float formats that file 26 carried as unresolved was never a gap to patch; it was the
parameterisation reporting that it pointed the wrong way (`40:48-55`).

**The affine value map**, stated once here because two later sections depend on it and the tenth consolidation
named the formula without stating it (`109:236-238`). **The value of a stored integer `k` under a numeral is
`Adjustment * radix^exponent * k + Bias`.** `Adjustment` and `Bias` cannot be folded into each other: one
changes the spacing between representable values, the other moves the origin, and an affine map is not
determined by either half alone. The worked example, which proves the independence rather than asserting it:
UNORM8's values are `k/255`; **at exponent `e = -F` an adjustment factor of `r^F / (r^F - 1)`, which is
`256/255` at `r = 2, F = 8`,** lands `k = 0` on 0 and `k = 255` on exactly 1, and no bias alone can do both,
since matching one endpoint with a bias displaces the other (`11:157`, `11:176-181`). Section 1.11 and section
1.28 are both this example generalised.

**The exponent convention is named at both sites where this constant appears, because it is written two ways
and both are correct.** Here it is `r^F / (r^F - 1)` at `e = -F`; at section 1.11 it is
`Adjustment = 1/(r^F - 1)` at `e = 0`. Under this document's own widened completeness line, a name defined
twice with different content is defined nowhere, so the convention travels with each spelling and both are
left standing because both are in use and neither is wrong (`111:381-388`). `FullRange`'s survival as a named
`Adjustment` constructor is on the open list, so this is live rather than historical.

### 1.2 The identity contract

```rust
// The member set is a stated supersession of D68's four flat members; see the
// correction below before reading this block as settled.
pub const trait Numeral {
    type Radix:     Radix;        // Rad<P>, one constructor family over sealed Pos
    type Precision: Precision;    // significand digit count, primitive (D69), a Nat
    type Exponent:  ExponentForm; // where the exponent lives; nests the rest
    type Domain:    SignDomain;   // NonNegative | Symmetric | AsymmetricLow, a value fact
}

pub struct Implicit<E: Exponent, A: Adjustment, B: Bias>(PhantomData<(E, A, B)>);
pub struct Ranged<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>(
    PhantomData<(EMIN, EMAX, U, S)>,
);
```

**The supersession the block's first line points at is the next four paragraphs, and it is stated rather than
silent.** Op ratified **D68 (from `202607301100`)** at `talk:1529-1545`: "`Numeral` carries **four flat
members**. Decision (op, 2026-07-30). `ExponentForm`, `Adjustment`, `Bias` and `Sign`", with `Adjustment`
defaulting to `Unit` and `Bias` to `Zero` so neither costs anything where it is unused, amending D65 (from
`202607301100`) which predated the bias question. `talk:1509-1527` records why flat won over grouping. The
declaration above nests `Adjustment` and `Bias` inside the exponent form and carries `Radix`, `Precision`,
`Exponent` and `Domain` at the top level, which is **neither of the two options op chose between**.

**Both of the nesting's grounds postdate D68**, which is what makes the supersession legitimate under section
0.3 rather than a drop: `Underflow` has no bottom to fall off under a constant exponent (`49:125-128`), and
D69's overturn at `30b` makes `Precision` and `Radix` primitive top-level members D68 could not have known
about. **A supersession that is stated is legitimate; one that is merely omitted is a drop wearing better
clothes**, and a future member re-deriving whether `Adjustment` should be flat would otherwise find a ratified
answer in the register and a contrary shape here with nothing joining them (`113:207-218`). **The nested shape
is current. Whether op accepts the supersession is op's**, and it is on the open list, because the two contrary
shapes are both his.

**The one half of D68 that survives by content unnumbered is its closure gate**,
`impl<N: Numeral<Bias = Zero>> AddClosed for N {}`, which section 1.7 carries as "the shipped `AddClosed` gate
on `Bias = Zero`".

**The two constructors carry `PhantomData` and the reason is the design's own idiom rather than rustc's
suggestion.** As unit structs they do not compile: seven instances of `error[E0392]: type parameter E is never
used`, reproduced under the pin, with rustc's own help line naming three repairs. **`PhantomData` is the
document's own idiom at exactly this position**, carried by nine sealed constructors in the section 1.23 block
(`O<P>`, `I<P>`, `Pz<P>`, `EPos<P>`, `ENeg<P>`, `Rad<P>`, `BPos<N, D>`, `BNeg<N, D>`, `HostFloat<E>`), so the
two identity structs were the odd ones out rather than the ones being changed.

**And the two forms differ on the seal, which is what section 1.12 spends eleven firings on.** A unit struct
is nameable and constructible as a value by any downstream crate. The tuple form carries a private field, so
the same attempt refuses:

```
error[E0423]: cannot initialize a tuple struct which contains private fields
 --> consumer2.rs:4:55
  |
4 | pub fn try_build() -> Implicit<EZero, BZero, BZero> { Implicit(PhantomData) }
  |                                                       ^^^^^^^^
  |
note: constructor is not visible here due to private fields
```

Compiled across a real crate boundary rather than within one file, which is the only arrangement in which the
distinction is visible at all. **These are type-level markers and no value of one should exist**, so the
pre-repair form was not merely unused-parameter noise: it left a value door open in a sealed vocabulary.
**Whether the markers should be uninhabited outright rather than merely unconstructible is not decided here**,
because the document nowhere says, the answer would apply to all eleven constructors at once, and on the
evidence available the perimeter is closed either way. It is a refinement rather than a gap, recorded so
whoever writes this in source sees the question was asked.

**One token collision the block cannot say for itself.** The member named `Exponent` is bounded by
`ExponentForm`, while the trait named `Exponent` is the sealed signed integer of section 1.15, and `Implicit`'s
own `E` is bounded by that one. Two ratified definitions, one token, different content, which is address one
of section 1.26's own three mechanical addresses. **Renaming either is op's**, and it is one line; the
collision is recorded rather than repaired here, and it is on section 2's list alongside the two other
instances of the same defect.

`Implicit`, `Ranged`, `NonNegative` and `AsymmetricLow` all returned zero hits in `78`, `91` and `102`, **and
the far-point rule and the float preset table both quantify over `Ranged`-shaped numerals the standing
document had not defined since the seventh consolidation** (`109:448-452`). The declaration above is from
`58:122-161` and `68:155-165`.

**Every exponent position is a type.** `Implicit`'s own `E` and `Ranged`'s `EMIN`/`EMAX` are all computed by
`mulnum` (the exponent sum for `Implicit`, the bound sums for `Ranged`) and all have to appear in the result
numeral's type, which is the spine rule firing on exactly the shape it was stated for. `Exponent` itself is
`EZero | EPos<P> | ENeg<P>` over the sealed `Pos`, sealed at birth, arrived at independently three times
(`Bias`'s own sign shape, file 42; a signed exponent, file 50; the fold's own headroom arithmetic during an
early wrong attempt at the negative impl, repaired by separating magnitude difference from sign the way `Bias`
already does). **No const route survives**: bare const arithmetic needs the forbidden `generic_const_exprs`;
`min_generic_const_args` refuses the shape outright ("complex const arguments must be placed inside of a
`const` block"); a `const { }` block refuses with the identical "generic parameters may not be used in const
operations" and asks for `generic_const_args`, which needs `-Znext-solver=globally`, mutually exclusive with
the rest of the arrangement per the workspace's own record. Every permitted door is closed, compiled shut in
both directions (`50_probes/probe_3b`, `54_probes/probe_4b`).

**`Radix` is a sealed carrier rather than an open trait.** The earlier spelling admitted `R = 1` (which
collapses the exponent's whole grid family into one grid) and `R = 0` (a zero quantum) as compiling instances
that falsify the float model's own founding sentence. `Rad<P>` is the sole constructor over the sealed `Pos`,
bounded by `AtLeastTwo` (covering `O<P>` and `I<P>`, excluding `H`, exhaustive by construction rather than by
enumeration, since `Pos` has exactly three constructors and `Pos` is sealed): every radix from two upward is
expressible by naming a `Pos`, and radix zero has no `Pos` spelling at all, while radix one refuses at the
bound (`H: AtLeastTwo is not satisfied`) rather than at some later arithmetic that would have produced a wrong
answer. Compiled positive and negative, and standing on five compiled routes (section 1.12). What `R = 1` and
`R = 0` admitted is stated because it is the whole reason `Radix` is sealed and it left the record at the
sixth consolidation (`109:367-368`).

**`Bias` and `Adjustment` are signed, gcd-normalised rationals, value-unique and sealed**, as ratified at
`44b`; every reference elsewhere in this document assumes that shape.

**`SC_SAT_SYM` and the payoff of the `Sign` split**, from `49:122-124` (dropped at the fifth consolidation and
absent until the eleventh; `SC_SAT` returned zero hits in `58` and in files 50 through 57, `109:321-323`): the
identical `TowardNegative` clamp delivers `-8` under `AsymmetricLow` and `-7` under `Symmetric`. That one cell
is the payoff of splitting a single three-instance `Sign` axis into `SignDomain` (identity, a value fact) and
`SignIndexing` (encoding, a datum fact), and it is what makes SystemC's own `SC_SAT_SYM`/`SC_SAT` pair
expressible at all, which is the standard's test from section 0.1 being passed.

**The nested shape's own argument**, from `49:125-128` (`109:333-336`): the argument the nested
`Implicit`/`Ranged` shape actually stands on is that `Underflow` has no bottom to fall off under a constant
exponent, so there is nothing for it to mean at every other numeral. Block floating point was withdrawn as
evidence for the nesting (droplisted, section 6) and the withdrawal was recorded; the positive argument was
not, leaving the ratified nested shape with its only support out of the record. It is the only support that
shape has.

### 1.3 Encoding, nested inside Lowering

```rust
pub const trait Encoding {
    type SignIndexing: SignIndexing;      // Unsigned | TwosComplement | SignMagnitude | OnesComplement
    type Fields:       FieldLayout;       // field widths, hidden bit, encoding bias, reserved codes
    type Canonical:    Canonicalisation;  // signed zero, preferred cohort, NaN canonicalisation
}
```

Nested inside `Lowering` rather than carried as a third type parameter on `Number<N, S>`, so the two-parameter
fused form survives and the **1.8x rendered-diagnostic-length cost of a three-parameter split** (measured,
`26:32-35`) is not paid a second time. That measurement is the reason the nested declaration exists, and it
left the record at the fourth consolidation, which shows the nested declaration with no reason attached
(`109:311-313`).

**`Lowering`'s charter, in one sentence, and this statement is load-bearing for everything downstream of it**
(`40:126-128`, unamended since):

> **`Lowering` changes no value. `Encoding`, nested inside it, may change which datum carries a value. Every
> operation whose result depends on that is declared a datum-level operation, and no law may read one.**

This is what lets the identity inversion and the datum and value distinction coexist without fighting, and it
is enforced structurally rather than by convention: a law's key is a `const fn` parameter list and `Lowering`
is not a parameter, so reading it fails with `E0425` at the point of use; a value-level fact declared in the
algebra-contracts crate cannot even name an `Encoding` or `Lowering` type, because the name does not resolve,
`E0433` (`40:129-135`).

**The first clause is the operative one.** File 93 first cited the weaker second sentence ("no law may read
`Lowering`") and found it satisfied vacuously, since every division law is conditioned on a nonzero divisor;
file 95 corrected the citation, and the clause that forbids a smuggled cell outright, with no new text, is
"`Lowering` changes no value" (`102:148-153`, section 1.13).

**The double-duty sharpening**, from `68:176-181` (`109:459-461`): this correct rule has been doing double duty
as "*nothing* may read `Lowering`", which is exactly what left the crossing contract's precondition unstated
and the transfer argument's index set six coordinates short of the seven it needed. The crossing contract
(section 1.4) and the transfer-ground scheme are both repairs to that double duty, not to the rule itself. Its
deletion at `78:193-196` removed the licence for statement P that the same document then introduced.

**Measured** (`40:137-143`): a trivial `Canonicalisation` (identity, every `Specials = None` composition) adds
zero instructions to an operation body; a trivial `FieldLayout` (no hidden bit, no encoding bias) produces
byte-identical code to a hand-rolled shift-and-mask extractor on a packed sixteen-bit column. A richer
`Canonicalisation` (real NaN payload rules, decimal preferred-exponent selection more complex than a range
test) costs a small, branchless, measured constant for the simplest real cohort collapse tested (seven
instructions against two, `csel`-based, no branch); **whether every plausible `Canonicalisation` instance stays
branchless is unmeasured and open** (section 5).

### 1.4 The crossing contract: a precondition, three statements, and a family of leaking configurations

**Three statements over the finite datum set of a numeral** (`58:173-177`, `68:186-190`, `91:161-176`):

1. `decode ∘ encode = id` on values, always.
2. `encode ∘ decode` is idempotent on data, always (canonicalisation).
3. `encode ∘ decode = id` on data iff the encoding is injective, a derived boolean.

`encode ∘ decode` returned zero hits in `78`; the eighth consolidation asserted the three statements untouched
without stating them, and no consolidation between stated them (`109:452-455`). They are the contract, so they
are stated.

**Statement 3's derivation**, written down and compiled against the whole configuration matrix rather than a
single witness (`58:182-197`):

> The encoding is injective iff no value has two data. A second datum arises from, and only from: an
> unrepurposed signed zero; more than one reserved NaN datum; a cohort, meaning an unnormalised significand
> over more than one exponent with room to shift; and a cohort of zeros, present even at precision one under
> an unnormalised encoding. The two infinity data are always distinct values and never a source.

Checked exhaustively over both radices, the whole `Specials` product, both underflow modes, both
cohort-selection rules, signed and unsigned domains, and with and without a repurposed negative zero.
`Specials::INF` never touches the boolean (infinities add values one for one with data); `Specials::NAN`
always can. **Statement 3 is genuinely two-valued, not a polite way of writing "always false"**: an unsigned,
no-specials, normalised numeral is injective; so is the OFP8 `E4M3FNUZ` variant, which repurposes the
negative-zero datum as its NaN and is injective while still carrying a special (compiled, 512 data, 256 live,
256 values). Before file 54 the boolean's only compiled witness in the whole review was signed zero, which is
why the statement had been exercised vacuously for twenty-four files.

**The precondition, not a fourth statement** (`68:201-211`). Writing out the three maps honestly:
`encode : V -> D`, and `decode`, at every numeral this review has built, is total arithmetic on the physical
fields landing in the rationals, `decode : D -> ℚ`. Nothing in that formula consults `V`. Statement 2
therefore requires `decode`'s output to lie in `encode`'s domain, which is `V`, not `ℚ`. **Under a hole,
statement 2 is not false: it is ill-typed**, and statement 3 inherits the same defect. Compiled out honestly (a
`Value` newtype whose only constructor checks membership) it gets `E0308: mismatched types`, with rustc's own
suggested fix, `Value { inner: decode(f, d) }`, being exactly the unchecked coercion the design performs
silently today. So the missing item is a **precondition** the other two statements are stated over, placed in
front of them rather than beside them.

**The escape is a family, not a cell** (`68:213-240`). File 66's sweep held `Specials` fixed while varying
radix, precision and normalisation, and concluded "exactly one cell of the matrix leaks". `Specials` is the
design's other value-set-shrinking axis, sitting on `Numeral`, and nothing couples it to the field layout that
decides which data exist, sitting on `Lowering`. Modelled at E4M3's own shape across the whole `Specials`
product:

| layout | `Specials` | data | escaping | percent |
|---|---|---:|---:|---:|
| ieee | `NoSpecials` | 128 | 8 | 6.2% |
| ieee | `NanOnly` | 128 | 1 | 0.8% |
| ieee | `InfOnly` | 128 | 7 | 5.5% |
| ieee | `IeeeSpecials` | 128 | 0 | 0.0% |
| ocp | `NoSpecials` | 128 | 1 | 0.8% |
| ocp | `NanOnly` | 128 | 0 | 0.0% |
| ocp | `InfOnly` | 128 | 1 | 0.8% |
| ocp | `IeeeSpecials` | 128 | 0 | 0.0% |

**Six of eight cells leak.** Under the IEEE layout, three of the four `Specials` members leak, the largest
being the entire top exponent code, one part in `2^ew` of the datum set; only the member the layout was
designed for does not. **The hazard fires whenever a `Numeral` axis shrinks the value set and no `Lowering`
axis correspondingly shrinks the datum set.** OCP's own E4M3 is not a counterexample: it is the existence proof
that real format designers do this coupling by hand and pay a standard's own paragraph for it (dropping
infinities, raising `emax` from 7 to 8 until `decode` was total again), and the same layout leaks the moment
its value set moves again. arvo currently has nowhere to state this coupling and nothing that notices when it
is skipped.

**The repair is derived, not chosen: only one exists** (`68:242-274`). The alternative to excluding the
escaping data at the encoding is widening the target, composing through the quantiser, and the quantiser
refuses on every escaping datum of every leaking cell, against a 2,701-value negative control confirming it is
otherwise the identity. So the obligation is `Crosses<N: Numeral>: Lowering`, an `unsafe impl` at the format
declaration site, exactly as `NumeralFace`'s bound sits at the face declaration site. Under the exponent-form
parameterisation of section 1.21 the supertrait edge is spelled `Crosses<N: Numeral>: Lowering<N::Exponent>`
and nothing else about it moves.

**Three alternatives are explicitly refused, and the refusal is recorded so it stops being re-proposed**
(`68:272-274`, dropped at `78:198-204`, `109:419-422`): `decode` returning a `Maybe`, a runtime check, and a
per-operation well-formedness predicate. **All three move a declaration-time fact into a use-time check, which
the design's own binding-time discipline exists to avoid.** A recorded exclusion exists to stop re-proposal,
and dropping it is how a refused alternative comes back.

**Statement P** (`73`, one member per `Lowering`, `78:569-578`): for every carrier a `Lowering` can produce,
the bits outside the placement map's occupancy are exactly the padding this `Lowering` declares. The tower's
own generated impls satisfy it for free by the purity argument at section 1.22; an `unsafe impl` for a
hand-laid format is where the obligation actually bites, at the identical declaration site statement 0 already
governs. The region is the complement of the occupancy rather than a suffix, which is section 1.22's one-word
correction stated in the place it applies.

**Statement C** (`91:548-555`, section 1.22): the container's bits outside the carrier are canonical,
established once by the projection's own pure constructor, and every value-keyed or datum-keyed observation
consumes the container through the canonicalising projection as its only door. Discharged by the tower itself;
an obligation only at the constructor that accepts foreign bytes.

**Every `unsafe impl Crosses` is an entry in the trusted base, named explicitly as such** (`80:104-108`), not a
derived fact D16 (from `202607290200`)'s split would call safe.

**Also adopted, file 66's toolbox note** (`68:282-286`): `Abrupt` on a decimal numeral is a pure
representability restriction with no encoding-space or performance dividend, because the minimum-exponent,
sub-normalised-significand cohort that radix two's `Abrupt` frees up for flush-to-zero hardware does not exist
under an unnormalised significand; those encodings were never free there. The axis stays available per the
warn-never-police discipline, documented at the point of choice. Dropped at `78:266-330` (`109:422-423`).

**Still open**: statement 0 against `quantize` and `roundToIntegralExact`, the two operations `63:338` carves
out as datum-dependent by definition. Flagged forward by four files and performed by none; file 67's guess that
it lives wholly inside `D` is marked a guess (`68:276-280`). Section 5.

### 1.5 The quantiser

**Round first, classify second.** The quantiser rounds on the target grid extended upward without bound, then
classifies the rounded result against the numeral's range, `Specials` and `Underflow` (`58:220-224`,
`29:111-116`). Classify-then-round, the spec's original order, disagrees with all three test standards on the
band past the largest representable but within half a quantum of it, and is droplisted (section 6).

Confirmed against binary32 on **41,380,159 operations with zero mismatches** (file 50); the radix-general
kernel repaired and regression-checked bit for bit against silicon at radix two (file 59).

**The transfer argument's refutation has two compiled supports, and both are stated because a restored
measurement without its second half is a restored assertion** (`111:127-149`). This matters beyond
bookkeeping: the design's largest pending item for op is three `unstable-features.md` wording edits whose first
is exactly this correction, and op was being asked to edit a ratified workspace rule on half the evidence the
panel produced for it.

**Support one.** **A tie is reachable only at an even radix**, re-derived independently from the quantiser
itself over radices two through thirteen (`68:290-294`), with the rounding counts recorded so the odd rows are
not vacuous (318 roundings at `r = 3`, 188,448 at `r = 13`, zero ties in either, `68:448-450`).

**Support two, and it is the sharper of the two** (`68:451-455`): **absorption-freedom (for all nonzero `y`,
`quantise(x + y) != x`) is exhaustively TRUE at exponent span `p` and FALSE at span `p + 1`, with the
precision, the code and the bans all held fixed.** `EMAX` moved by one and the property's truth value moved
with it. Same precision, same code, same bans, and the outer quantifier failed anyway, because nothing about
implementation uniformity ever bore on it. This is the sharper result because it is the one where the outer
quantifier fails **with the bans in force**, which is the whole content of the claim that implementation
uniformity does not give property uniformity.

**What the refutation is a refutation of**, stated with it because a refutation without its target is an
assertion (`68:438-446`). `unstable-features.md`'s last sentence promotes **implementation uniformity** (one
parametric function, no instantiation gets a different body) to a sufficient condition for **property
uniformity** (the truth value of a claim about the function's outputs does not move as the parameters move).
The rule's own source, file 10, gave the transfer argument **four legs** and knew the difference: **leg one is
parametricity**, which the bans enforce and which the ratified sentence kept; **leg three is width-uniformity
of the rule's own shape**, "arguable in prose... never mechanical", named correctly as unproved and never
proved since. The compression from four legs to one sentence kept the mechanical leg and attached the other
three legs' conclusion to it.

**A `Ranged` numeral's own quantiser statement**, from `58:220-224`:

> A `Ranged` numeral denotes the union, over `e` in `[EMIN, EMAX]`, of the grids with quantum
> `radix^(e - p + 1)` restricted to `[radix^e, radix^(e+1))`, together with the bottom grid extended down to
> zero when `Underflow = Gradual`, omitted when `Underflow = Abrupt`. Quantising selects the grid from the
> exact value's own magnitude, rounds on the selected grid extended upward without bound, then classifies
> against `[EMIN, EMAX]`, `Specials` and `Underflow`.

**The algebraic difference from `Implicit`, stated once because three separate results follow from it**
(`58:226-233`, dropped at `63:204-205` by a citation stopping one line short of it, `109:363-365`). An
`Implicit` numeral's value set is an interval of a rank-one subgroup of the rationals, closed under addition
wherever the sum stays in range. A `Ranged` numeral's is a union of intervals of subgroups whose generators
form a geometric chain, and **that union is not a subgroup**: `1 + 2^-24` is not a binary32 value, and the
machine agrees, delivering `0x3f800000`. Three results follow from that one sentence: the overflow band is
inhabited because the exact result lies on a finer subgroup than the result's own quantum; the fold needs an
exact accumulator because in-range closure is what an in-format accumulator would have needed; and
associativity fails at the format width and holds through the accumulator, for the same reason.

**The overflow band, closed form, two clauses rather than one** (`58:235-260`). The candidate closed form
`q_result <= 2 * lattice` was built and refuted by exhaustive enumeration (753/1000 addition, 639/1000
multiplication, both directions of error) before the correct form was found:

> **Lattice clause.** The band is empty unless some point of the exact-result lattice lies strictly inside
> `(max_r, max_r + q_r/2)`. Decidable from the three quanta alone by one Euclidean division for an operation
> whose exact results form a subgroup.
>
> **Reachability clause.** That point must be an actual exact result of two in-range operands.

Measured over **5,184 triples**: the lattice clause alone has **zero under-predictions** for both addition and
multiplication, which is the useful direction for a build layer to act on, since it never claims empty when
the band is inhabited. Its over-predictions are all reachability failures. Every prior band member the review
had stated maps onto one row of this table:

| case | exact-result lattice | q_r | band |
|---|---|---|---|
| fixed, same format, add | q | q | empty |
| fixed, same format, mul | q² | q | inhabited |
| fixed, mixed, dividing quanta, add | the finer quantum | the finer quantum | empty |
| fixed, mixed, non-dividing, add | gcd(q1, q2) | the finer quantum | inhabited |
| float, both operands in the top binade | the top quantum | the top quantum | empty |
| float, one operand three binades down | the finer quantum | the top quantum | inhabited |

**Division has no row**: its exact results are not lattice-valued at all, which is why file 43 had to compile
that member rather than derive it. The two clauses, the zero-under-prediction measurement, the six-row table
and the division sentence were all dropped at `63:205` (`109:362-363`).

**The domain widening adopted at `101b`** (`102:157-168`): `ExactValue`, the quantiser's input, had no ratified
domain statement, and every operation checked before that stretch delivers a rational exact result, so the gap
was invisible because it was vacuous at every instantiation the review had checked. **Adopted: `ExactValue` is
any real number equipped with a decidable ordering against the target grid's points and midpoints.** Every
existing operation's exact result is rational and the widening changes no ratified behaviour; it legalises the
roots' and the radix-power exponential's irrational inputs and states the decidability condition that grades
the elementary functions' carrier classes (section 1.16).

**The in-range and out-of-range boundary fix and the radix-general repair.** The quantiser's boundary fix was
found in file 11, lost at `26:44-48` (which restated the quantiser in the unfixed classify-first form), and
independently re-found nine files later and ratified at `40:165-180` (`109:242-244`). The radix-general
repair's three places and its radix-ten validation leg were dropped at `68:290-292` (`109:405-407`); both are
carried here as the current statement.

Dither and shaping stand as `49:179-186` states them. **The shaping-is-a-scan counter-reading**
(`40:202-205`, `109:302-304`, dropped at `49:183-186` which states the conclusion and deletes the condition
under which it fails, while file 45 records the claim's ground as resting on nothing ratified and says
explicitly that the counter-reading is carried in `40:200-205`): first-order error feedback bounds total error
within one quantum forever, but it needs state, so it belongs in a combinator's accumulator object rather than
in the type-level policy, for the identical reason stochastic rounding is already excluded (resolution
constructors are pure ZSTs). The condition under which the conclusion fails is that a future scheduler
consuming combinators generically would make the accumulator's residence a scheduling question rather than a
typing one.

**Rounding bias is a real system property, not a naming nicety** (`26:288-305`), and this is the paragraph
whose error-feedback half was absorbed at `40:193-201` while its owed const bound was not, making it a
selective drop rather than a section supersession (`109:284-288`). On a two's-complement value an arithmetic
right shift rounds toward negative infinity, a constant-offset bias drifting linearly with sustained
accumulation, measured roughly `-K/2` quanta after `K` operations, the textbook DC ramp, manifesting as limit
cycles in a feedback loop; that is a different filter from round-toward-zero, whose bias is sign-dependent,
symmetric, and actually stabilising in a feedback loop. The preset table's word "truncate" is ambiguous between
the two and the table below says which one is meant (`TowardNegative`). Round-to-nearest-even is zero-mean and
random-walks (`~q*sqrt(K/12)`, no ramp), confirming it as the right default for `Warm`/`Cold`/`Precise`.
**Double rounding is real**: a nearest-then-nearest two-step narrowing disagrees with a single nearest rounding
on 12 to 19 percent of inputs. Round-to-odd at the intermediate restores exact agreement, **but only above a
precondition** (the intermediate carries at least two more fractional bits than the destination,
`W >= F + 2`); below that precondition round-to-odd is measurably worse than the naive two-step rounding it was
meant to fix, **and this validity range must ship as a checked const bound alongside the credit given to it**.
That owed bound is on the open list.

*Grounded on: ratified (`44b`, `101b`, `39b`), settled shapes (`58` section 1.5, `29:111-116`, `26:288-305`),
compiled (`50` binary32 sweep, `54_probes/probe_2`/`probe_3`, `99_probes/probe_1` through `probe_4`),
measured (5,184 triples, 318,126 parse strings).*

### 1.6 Membership and the number-system layer

**D38 and D39 (both from `202607291900`) are op's calls, and D38's content is the enumerated vocabulary rather
than a crate name.** From `40:209-212`, compressed to a crate name at `49:206` and absent until the eleventh
consolidation (`109:107-117`):

> **D38 and D39 are op's calls** (the `arvo-num-systems` crate: ℕ, ℤ, ℚ, ℝ, ℂ, ℍ, 𝕆, surreal, hyperreal,
> p-adic, **shipped even if nothing uses them, vocabulary fixed by mathematics**; membership through algebraic
> structure, depending on the algebra ladder). **D39 is held, not overturned**, at op's seventh checkpoint
> (`30b`), despite two readings finding its stated mechanism does not compile and that a membership predicate
> over the whole ambient set is vacuously true of everything.

**And D39's objection has an answer, twenty lines below the text the block above comes from** (`40:227-235`).
The predicate is **inhabits**, not **equals**. So `Inhabits<Real>` being vacuously true of every finite numeral
is a correct fact about the top of the tower rather than a refutation of the predicate. The honest derived fact
is the **finest inhabited system**, which exists and is unique because the tower is a chain, and this is the
identical move the laws make: do not choose a relation, report the finest one that holds. Membership is then
decidable from the identity axes alone (`Natural` iff `Domain = NonNegative` and quantum and bias are naturals,
and so on), which makes it a derived fact keyed on the numeral rather than a declaration. **The objection and
its answer travel together**, because a held call described only by what was said against it is a held call a
future member will reopen from the wrong side.

The aggravating fact worth recording: file 45 defines the `d38` ground slug by pointing at `40:209-212`
(`45:127`, "op, carried `40:209-212`"), so the fourth consolidation deleted the text its own grounding registry
cites as a definition. The consequence lands seven consolidations later, where `78:678` instructs a future
member to "scope the 'finest' fact to the real/Cayley-Dickson chain explicitly" against a ten-member vocabulary
the standing reference had not contained since the third consolidation.

**Membership licenses only the exact, widening operation family, gated on `Specials = None`** (`40:215-225`,
`58:266-267`; the gate clause left the record at `63:227` and `Specials = None` returned zero hits in `63` and
every consolidation after until the eleventh, `109:168-172`). Quantised in-numeral operations get their laws
from the algebra ladder, keyed exactly as section 1.7 keys them, never from membership. Inclusion into an
ambient set is a homomorphism only for the exact family (`mul_full`, exact addition), where no recovery map
fires; every quantised in-numeral operation's recovery map is precisely the measure of inclusion's failure to
be a homomorphism, and the measured law inversions recorded in file 26 are its empirical face.
**`ExactWindow<Op, Rhs>`** (the derivable fact that an exact operation stays inside an expressible window) is
the concrete content membership licenses, and it is not total where `Specials` exists: with infinity
representable, `∞ * 0` lands in no window, so `ExactWindow` gates on `Specials = None` as the honest first
ship. `ExactWindow` returned zero hits in `58` and after; it is half of a claim file 45 grounds as a single
two-part claim carried by two independent readings, and dropping half of a two-part claim leaves the surviving
half ungrounded (`109:331-333`).

**The uniqueness theorem, in its current corrected form** (`91:183-195`). Two independent reads established
that every arvo value is `m · r^q` for integer `m`, `q`, and integer radix `r ≥ 2`, so every arvo value set is
a finite set of rationals, and **the finest inhabited system of any arvo numeral exists, is unique, and lies on
the sub-ℚ chain (ℕ ⊂ ℤ ⊂ ℚ, canonical embeddings), independent of every branch above ℚ.** This does not need
the whole vocabulary to be a chain and survives every branch the vocabulary grows.

**What the sixth and seventh consolidations got wrong about the branch count, and why the record needs both.**
`68:301-309` refuted the mechanism's stated uniqueness justification against the full ten-member vocabulary D38
ratified, by Ostrowski's theorem: "exists and is unique because the tower is a chain" is false against ℕ, ℤ, ℚ,
ℝ, ℂ, ℍ, 𝕆, surreal, hyperreal, p-adic taken together, because the surreals and hyperreals are both
ordered-field extensions of ℝ, and each p-adic completion is not an ordered field and is incomparable to the
rest. That refutation was dropped at `78:212-216` (`109:463-465`). The ninth consolidation then corrected the
branch count itself: Conway's surreals are the universal ordered field, so `*ℝ` embeds into `No` under
ordered-field embedding and the two are comparable, giving a chain alongside the non-orderable
Cayley-Dickson line and the p-adics; **but under plain field embedding every characteristic-zero field of
cardinality at most the continuum embeds into every other, so the branch count depends on which embedding
signature is being used and is not a fact to ship without naming one** (`82:246-257`). Both halves are carried:
the Ostrowski refutation is why the original justification failed, and the embedding-signature observation is
why a replacement must name its signature.

**The recommendation, unchanged**: scope the single "finest" associated fact explicitly to the
real/Cayley-Dickson chain, and give the surreal, hyperreal, and each p-adic branch their own independent,
non-competing membership predicates, before `arvo-num-systems`'s own type shape bakes in a uniqueness claim
that silently assumes a sub-chain op explicitly ratified as wider. **Final hardening stays op's, since the hold
was his.**

**What is genuinely open, and it is a new item rather than a residue of the old one** (`91:197-204`): seven of
the ten ratified vocabulary members are never the answer the "finest inhabited" mechanism returns, since its
output type is a three-element chain. Two readings are offered symmetrically. Under the first, the upper seven
exist as upward closure (may this value be used where a `ℂ` is expected, a query about a consumer's required
algebra rather than about the numeral), and `arvo-num-systems` ships two relations, not one. Under the second
they are anticipatory vocabulary for a value set arvo does not yet build, and the finest mechanism genuinely
returns from a three-element chain today. Op's call.

**The finest-system derivation table's staleness finding and its repair instruction**, from `40:238-241`
(`109:314`): the table's own agent-written contents are stale against the ratified contract in three ways (it
credits every fixed-point type to ℤ[1/2], missing the radix-ten case, the `FullRange` case, and any rational
bias) and **should be derived from the `Numeral` members rather than hand-fixed a second time**.

*Grounded on: ratified (D38, D39, `30b`, `70b`, `74b`), settled shapes (`64` section 3, `39:340-360`,
`40:215-241`), compiled and reasoned (`80` section 4, `82:246-257`), external (Ostrowski's theorem, Conway's
universal-embedding theorem, the characteristic-zero prime field, cited rather than compiled per the review's
standing practice for standard mathematics no toolchain here could check).*

### 1.7 The algebra: what a law is, and the finest view it holds at

This is the section the audit measures as the archive's worst stub chain: 1117 words at `40:243-327`, then
496, then 146, then 75, then five consecutive "unchanged this stretch" with no line range (`109:189`,
`109:203-209`).

**A law is a claim that the terms of one grouping class stand in a relation, under a stated view, over the
value set of a numeral, quantified over the class rather than pairwise, and keyed on every parameter its proof
used.** It is a `const fn` whose parameters are its key and whose return type is `Never` or the finest view
under which it holds, derived by blanket construction over the composition rather than declared per type (D51
from `202607300400`), safe when derived and `unsafe impl` when asserted (D16 from `202607290200`)
(`40:245-249`).

**The finest-view mechanism replaces the three-relation fork.** This is op's third and final ratification at
`39b` (`40:251-267`). File 33 imported the standard partial-algebra vocabulary (weak equation, existence
equation, Kleene equation) and file 26's fused-verdict question looked like a choice among these three. It is
not a choice: **a term's meaning is a grade (a free commutative monoid over refusal causes and quantisation
events) and a value; a view is a monoid homomorphism out of the grade; and two terms are equal under a view
when the view sends their grades to the same thing and their values agree wherever present.** Compiled
exhaustively over nine such views and nine compositions: the set of views under which a law holds is downward
closed and closed under join, so **every law has a unique finest view**, and that view is the law's content.
The named relations are three points of a nine-point lattice, **and the lattice is not a chain**: `Hot` on a
signed numeral and `Precise` below its accumulator's interior-safety threshold sit at incomparable points (one
preserves values and definedness while losing quantisation events, the other preserves values and events while
losing definedness). That incomparability is precisely why the open question about how `Precise` reads had
resisted three files: the vocabulary had no name for the point it actually sits at. Both of file 26's held
readings were right about their own half, and the lattice serves both without a trade.

**Law equality is the canonical quotient** (`40:269-274`): two results are law-equal when canonicalisation
sends their data to the same datum. The shipped `arvo-numeric-contracts::TotalOrd` induces a *datum*-level
order (it separates signed zeros and orders NaN payloads, matching `f64::total_cmp` and IEEE 754's
`totalOrder`), so it cannot be the definition of law equality; it survives, reclassified, and section 1.20
carries the trait split that resolves it.

**The key** (`40:276-286`): the operation, whose marker carries whether its grade monoid is trivial; the
operand numerals and, for a widening operation, the result numeral; the `Quantisation` resolutions and, where a
quantiser sits between the exact operation and the result, its `Direction`; for a fold, the accumulator numeral
and the arity. `Growth` is not in the key (section 1.10). `Lowering` cannot be named from where laws live
(section 1.3).

**One correction, compiled at an eight-value model: `IS_EXACT` alone does not trivialise an operation's grade
monoid; `IS_EXACT` and `Total<Op>` together do.** Exactness kills quantiser-generated events and causes;
totality kills causes with no quantiser origin, divide-by-zero being the design's own standing example. No
shipped or designed operation is exact-and-partial today, so nothing measured elsewhere is wrong, **but the
uncorrected sentence would be false the moment a value-level exact division exists**, which file 43's
`div_floor`/`rem` pair has since made real. This clause was a correction file 38 made to file 37, flagged at
`49:235-237` as moved "from prospective to load-bearing" by file 43, and dropped at `63:235-237`; `IS_EXACT`
returned zero hits in `63` and every consolidation after until the eleventh (`109:351-353`).

**Direction enters a law's key exactly when the exact result can leave the operand lattice** (`40:288-294`).
This is a single predicate replacing two separately-measured facts (`Precise` addition never rounds in range;
`Precise` multiplication rounds on roughly half of pairs). **Additive lattice closure holds exactly when
`bias / adjustment` is an integer**, and the shipped `AddClosed` gate on `Bias = Zero` is the special case of
that, which means there exist numerals with nonzero bias that are additively closed and that the shipped gate
would refuse (`33:264-267`, `33:665-667`, compiled at probe 5 against exhaustive computation in both
directions). **Narrowed-multiplicative closure holds exactly when the adjustment and the bias are both integers
and the adjustment divides `bias^2 - bias`**, and no fixed-point numeral with a fractional digit satisfies the
first conjunct, **which is the derived reason multiplication needs `mul_full` and addition does not**
(`33:669-671`). All three conjuncts are from `33`, where the probe established them, rather than from any later
carrier: a restoration that cites the most recent carrier inherits every paraphrase between it and the source,
and this condition had been restored once in a stronger and different form that its own parenthetical
falsified (`111:37-95`).

The fourth consolidation reproduced this heading sentence verbatim and replaced the body with meta-commentary
about how the probe was re-run (`109:307-309`); the derived reason is the content.

**The transfer rule, and why no consumer declares a waiver** (`40:296-312`). File 37's first mechanism let a
consumer declare a required view and checked the law against it; the compiler killed it, because the licence
check refused exactly the case the mechanism existed to handle. The repaired rule: **a regrouping publishes, in
its own result grade, exactly the grade generator classes its law fails to preserve. Tolerance is a transfer,
never a waiver.** Where the weak equation itself fails, the regrouping is refused outright rather than
published, because no publication rescues a genuine value divergence. There is no consumer-supplied index to be
too rich; the caller's contract is the ordinary type of the result, and a caller needing a definedness-faithful
fold takes `Folded<0>` while a `Precise` regrouping below interior safety delivers `Folded<1>`, refused by
`E0308` with no bespoke machinery. This is the coeffect-discharging-into-an-effect asymmetry file 26's graded
reading names as its single spec-worthy sentence, here used to make itself unnecessary: the permission-shaped
fact became data-shaped and the type system checks it for free. The published grade is declared and checked,
never computed (computing it in return position hits the forbidden `generic_const_exprs` wall); understating it
refuses, overstating it compiles and is merely pessimistic, the same safe direction the design takes everywhere
on lattice containment.

**The mechanism is priced against the alternative file 33 first proposed (five derived marker traits) and wins
on both axes: 0.130 ms per composition against 0.193 ms, and 907 bytes against 1854, at `--emit=metadata`**,
expressing nine points where the marker shape expresses eight of which five are junk (`40:314-318`). This is
the only measurement supporting one of op's three ratifications at `39b`, file 45 marks it current, and the
fourth consolidation dropped it (`109:298-300`). It is two orders of magnitude below file 36's type-level gcd
cost and is a neighbour to, not an answer for, the real-consumer compile-cost question.

**One sentence the design owes and does not yet state** (`40:320-326`): the evaluation strategy of a refusing
operand's sibling. Strict evaluation accumulates its quantisation events; a left-to-right short circuit does
not. Measured to change the published grade and no law's verdict, at every composition tested. File 39's
standards test tilts this toward the strict reading (IEEE's sticky flags, SystemC's per-variable flags, and
MATLAB's overflow logging are all strict-evaluation shaped: an operand's flags are raised by whatever computed
it, regardless of its sibling) but does not decide it; the fold combinators are the design's own object and the
choice is op's. The fused evaluation-strategy block was adopted at `48b` persona-tier and stands.

**The convergence worth naming** (`58:280-283`): the design's own grade, a free commutative monoid over refusal
causes and quantisation events joined by union, **is** IEEE 754's sticky flag register, bit for bit, over the
five clause-7 exceptions with no adaptation needed. Section 1.14 states the full finding.

**Whether a law attaches to a type or to nothing** was silently resolved by adoption at `40:243-247`, with the
discarded reading and the explicit warning against reading mechanism-agreement as noun-agreement both lost
(`109:288-290`). The caution stands: two mechanisms agreeing on how a fact is computed is not the same as
agreeing on what noun the fact is about, and the law's noun is the numeral's value set, not the type.

### 1.8 The fold: two conditions, two relations, and what the accumulator becomes for a float

From `40:328-357` and `58:285-307` (`109:190`).

**Interior safety** (the `n-1` factor): a fold of arity `n` over destination numeral `N` with accumulator
numeral `M` is interior-safe when `M`'s lattice refines `N`'s and `(n-1) * [min V(N), max V(N)]` is contained
in `[min V(M), max V(M)]`. No quantiser fires in the interior, so the fold is grouping-invariant, which is
**the law's own condition**, and the three-line proof (an exact total computed once at the root cannot depend
on grouping) survives unchanged.

**Total safety** (the `n` factor): the accumulator is invisible in the delivered function, so the fold equals
`quantize ∘ exact_sum`, which is **the specification's condition** and matches the DSP guard-bit sizing the
design already cites (eight guard bits for 256 MAC steps on the Motorola 56000).

**The two are related by the refinement order, not by any view.** Below total safety a fold is strictly less
defined than its own specification, and **interior safety can hold while total safety does not** (a refusal the
destination would have absorbed can surface as an accumulator refusal). **A combinator states which condition
it checked and the law it derives is keyed accordingly.** The formal definition and the relation between the
two, including both quoted sentences, were dropped at `49:256-258`, which keeps the headline, and the loss
propagated through `58:287-288`'s "unchanged from `49:255-265`" (`109:304-307`).

**For a multiply-accumulate**, the same two conditions apply with `N` replaced by the product numeral
`mulnum(N1, N2)`, with one repair for biased operands, exhaustively checked: the pairwise closure predicate
correctly reports a biased product numeral is not itself additively closed, and the fix generalises file 31's
gcd formula with a fourth monomial, so the accumulator is the zero-bias numeral with adjustment
`gcd(A1A2, A1B2, A2B1, B1B2)`.

**At interior safety, all three grade components (value, definedness, quantisation-event multiset) agree at
once**, because exactly one quantisation fires, at the root, on a grouping-independent argument. This is the
strongest row of file 37's compositions table and the design's clearest single argument for where the widening
effort belongs, since `Warm` and `Cold` go from having no law at any view to having every one, purely by
widening the accumulator, with no axis changed.

**What the accumulator becomes once the operand numeral is `Ranged`** (`58:290-302`):

> A `Ranged` numeral's entire representable set is contained in the single grid of quantum
> `radix^(EMIN - p + 1)` bounded above by `radix^(EMAX + 1)`. The exact sum of `n` values is exactly
> representable in an **`Implicit`** numeral of that quantum and width
> `(EMAX + 1) - (EMIN - p + 1) + ceil(log2 n)`, and interior safety for a float fold is satisfiable at that
> width, by a numeral of the design's other kind.

Checked, not asserted: **2,924,207 ordered triples exactly representable at the predicted width; 139,721
orderings agreeing under every rotation and reversal; the same folds held in-format instead show 23.17% of
triples disagreeing under left- against right-association.** At real formats the width is large and finite:
**binary32 needs 277 bits plus `ceil(log2 n)` for a sum, 554 for a dot product; binary64 needs 2,098 and
4,196.** The condition's statement is unchanged; only the sufficient-width formula changes. The object this
accumulator is is the one other fields call a quire or a long accumulator. The exact formula and the
real-format figures were dropped at `63:244-246`, which names the formula without stating it (`109:365-367`).

**A fourth reading of the growth-class question, agreeing with division**: a float fold grows as `ceil(log2 n)`
plus a term that is `Theta(2^w)` in the exponent field width, the same exponential class division's `Theta(2^p)`
belongs to. **The class is what appears whenever a field width indexes an exponent, not a peculiarity of either
operation** (`58:304-307`).

**A fixpoint's published grade is trip-count independent, and interior safety is not, unless the step
contracts** (`58:420-433`, dropped at `63:534-536` which names `Unbounded` without defining it,
`109:357-359`). Idempotence of the grade lattice's join was checked over the whole four-point carrier at widths
one through four, both associations. A fixpoint's grade is `join(seed grade, step grade)`, unaffected by how
many steps ran. Interior safety cannot follow the same argument: an unnormalised accumulating iteration's arity
is `trips * step_arity`, which needs `generic_const_exprs` to state as a bound. **What closes the gap is a
property of the algorithm, not of the trip count**: a step whose output range is bounded by its input range
(renormalisation) has a per-step bounded arity the capacity already covers, and "this step renormalises" is not
derivable from any numeral, so it is an `unsafe impl` under D16 (from `202607290200`), **the first
consumer-side asserted fact the review has found**, as against the operand-side facts D16 was written for. A
non-renormalising, data-driven trip count is given the arity **`Unbounded`**, which is not a `Pos`, so
`InteriorSafety<Unbounded>` coexists coherently with the `Pos` blanket with no specialisation of any kind.
`Unbounded` ships inside the sealed `Arity` wrapper (`Fin<P>` / `Unbounded`), settled at `67b`. The resulting
top-of-lattice grade is not decorative: it separates a sign-reading consumer (correct at `EventsTransferred`,
matching `fiedler.rs`'s own doc comment that only the sign pattern matters for `spectral_bisection`) from a
magnitude-reading consumer, which is refused against both a wrapping and a refusing solver.

**The digest is not an instance of this machinery and needs its own grouping-invariance law for a different
reason** (`91:220-231`). The numeral fold's two conditions both exist to characterise when a quantiser fires
mid-fold; a hash accumulator has no value set to leave and no quantiser in its interior, so the question does
not apply to it. What does apply is the underlying reason interior safety matters at all, morsel-parallel
dispatch: **a column digest's combine step is grouping-invariant when a digest computed by partitioning into
morsels and combining independent partials equals the sequential fold at every partition.** Compiled, and the
two properties shown independent as they must be: a naive chained running hash is order-sensitive but not
grouping-invariant; a positional (polynomial) combine is both, at every tested split. The correctness argument
the positional combine needs is not new mathematics; it is the multiplicative half's own exponent-offset shift,
reused with a position weight standing in for an exponent.

**`fold_compensated` must never receive the reassociation licence, and the reason is compiled rather than
argued** (`58:457-464`, dropped at `63:284-289`, `109:349-350`). The Kahan-style compensation step,
`(sum + y) - sum - y`, is algebraically zero as a real-number identity and numerically the exact bits lost when
`y` was added to `sum`. `reassoc` treats those readings as interchangeable, because algebraically they are, and
the compiled result is that the entire expression collapses to one instruction, `fsub s0, s1, s1`, always zero:
**the compensation term the combinator exists to compute is optimised away to nothing.** The scope boundary is
exactly the one the grade projection already draws at the type level (`fold` against `fold_sequential` against
`fold_compensated`), and the licence gates on that distinction directly. The licence's own design shape,
adopted at `53b`, was kept as a feature verdict at `78:866-867` with the design it licensed dropped
(`109:435-436`).

**The receipt the licence earns is designed, not built, and stays that way on purpose** (`58:466-470`). Four
clauses: the call site's monomorphised type carries a closed, constructor-headed `FoldGrowth` projection
showing no interior quantisation; the target numeral's `Canonical` fact matches the `nsz` grant; the interior
contains no adjacent same-format multiply for `contract` to fuse; and the combinator is `fold`, never
`fold_compensated`.

**"No division fold is owed"** (`49:460-462`, dropped at `58:374-386`, `109:328-329`): a statement about what
the combinator surface does not need, which reads in its absence as an unanswered question. It is stated here
so it stops reading as one.

### 1.9 The multiplicative half

From `40:358-385`, `58:309-313`, `68:325-330` (`109:191`).

**`mul_full` is a family of maps `N1 x N2 -> mulnum(N1, N2)`, not an operation on one set**, and its own
associativity does not typecheck until the numeral-level map's associativity is established first, a
precondition nobody had stated of the multiplicative half's own headline claim.

**The biased product numeral**: `bias = B1*B2`, `adjustment = gcd(A1*A2, A1*B2, A2*B1)`, file 31's closure
formula for the consolidation's own open closure gap. It collapses to the shipped exact-product rule when both
biases are zero, which is the property that decides whether a generalisation is worth having. **It generalises
to n factors**: the bias is the all-bias monomial, the adjustment is the gcd of every monomial carrying at
least one adjustment, associative and commutative because the monomial set is symmetric under permutation of
the factors, checked at arity three with a negative control confirming the cross terms are load-bearing rather
than an arity-two coincidence.

**`Distributes` is not an atom on a chain: it is `Monotone`**, checked as a biconditional both ways, but the
biconditional needs a stated relation to be true. **The split**: for a total operation on a totally ordered
value set, distributivity over the lattice operations is monotonicity, full stop; for a partial operation,
monotonicity gives only the weak-equation-level implication, and the Kleene-level statement additionally
depends on which of IEEE's two lattice-operation families is meant (`maximum`, which propagates an undefined
operand, or `maximumNumber`, which suppresses it; **both are required by the standards test**, section 0.1).

**No preset the design ships or can spell is a dioid over `(max, +)`**: wrapping addition fails
distributivity, saturating addition fails associativity and separately fails the annihilation axiom, `Precise`
addition is partial. This is not grounds to drop the `Dioid` rung under D47 (from `202607292300`) (the ladder
goes as deep as the theory does, which is **D75 (from `202607301100`)**, `talk:1802-1812`, op, 2026-07-30, and
it is D75's own reconciliation with D47 rather than a panel derivation: the two do not conflict once
*declaring* and *implementing* are separated, D47's sketch-and-bench obligation attaching to implementations,
which are claims about arvo, and not to declarations, which are not, `spec:308-311`); it is grounds for the
rung being **derived rather than declared**, reporting a correct "no" with the failing axiom named. A numeral
carrying an absorbing `Specials` element could make the rung non-empty, which is scoped as a requirement on the
identity contract discovered from the algebra side, not designed now.

**The rename that occasioned that reconciliation, and it carries the ladder's own rung vocabulary.** D75 (from
`202607301100`) (`talk:1802-1808`, op): **"`Combine<Op>` is `Magma<Op>`, and the ladder is named in full."** E2
asked whether the two were the same trait; they are not, and the confusion came from `Combine` being coined
before the ladder was named. **A magma is a set with a binary operation and no law claimed; a semigroup is a
magma whose operation is associative.** So the operation-carrier and the structure are two things, the laws
stay separate markers per D51 (from `202607300400`), and `Combine` was a placeholder for the one that already
had a name. **The ladder is written out to the depth the theory goes, not to the depth arvo's own numerals
reach** (`spec:307-308`), which is the sentence the `Dioid` paragraph above is applying. `Magma`, `Combine`,
`Semigroup`, `Monoid` and `semigroup` had all returned zero in the standing base, so **every rung name below
`Dioid` was gone** while the reconciliation D75 performed survived attributed entirely to D47
(`113:274-283`). The rung vocabulary is the algebra ladder's own surface and the taxonomy round mints it.

**`mul_full`'s exponent sum is computed at the type level, which the transfer scheme's exponent-offset symmetry
needs to hold multiplicatively** (`68:325-330`, dropped at `78`, `109:466`): the symmetry that lets an additive
claim transfer directly across a shifted window is not the symmetry a product needs, since a product's
equivariant home is a window shifted by **twice** the offset, and `mulnum`'s own construction already lands
exactly there, **checked at 254,830,080 instances with zero failures**.

**Reused a third time** in the elementary functions' derived class (section 1.16): integer `pow` is an iterated
`mul_full` with one quantisation at the root, the fold chapter verbatim (`102:188-190`).
### 1.10 Widening and Growth: two axes removed, closed shut

Both left the axis table at `39b`, ratified, closed shut. From `40:386-400` and `58:315-339`.

**The axis table they left, stated so the removals have a subject.** D69 (from `202607301100`)
(`talk:1621-1641`, op, 2026-07-30, and explicitly marked "overturnable if the reasoning does not hold")
ratified ten axes across three contracts:

| Contract | Axis | Instances | Status now |
|---|---|---|---|
| `Numeral` | `ExponentForm` | `Implicit<EXPONENT>`, `Stored<BITS, U>` | stands, reshaped: `Implicit`/`Ranged`, section 1.2 |
| `Numeral` | `Adjustment` | `Unit`, `FullRange<F>` | stands, nested inside the exponent form |
| `Numeral` | `Bias` | `Zero`, `Offset<..>` | stands, nested, signed gcd-normalised rational at `44b` |
| `Numeral` | `Sign` | `Unsigned`, `Signed` | stands, split into `SignDomain` and `SignIndexing` |
| `Numeral` | `LogicalWidth` | the total, significand derives | **removed as a primitive axis, `39b`** |
| `Policy` | `Quantisation` | a triple of `Direction` and a pair of `Resolution` | stands, section 1.23 |
| `Policy` | `Growth` | `Exact`, `Narrowed<Width, Anchor>` | **removed, `39b`, and left `Policy` entirely** |
| `Lowering` | `StoredWidth` | `Minimum`, `DoubleLogical` | stands, denotes the carrier (section 1.22) |
| `Lowering` | `Widening` | `InContainer`, `PerOperation` | **removed, `39b`** |
| `Lowering` | `Layout` | `Dense`, `Bitpacked` | stands |

`Underflow` is not a top-level axis, being nested in the stored-exponent form where it is the only place it
means anything, and number-system membership is not an axis at all, deriving from the numeral
(`talk:1643-1645`, section 1.6).

**So the set is not ten and has not been since `39b`.** Three of D69's ten are ratified out and the surviving
seven have since been reshaped, joined by `Radix`, `Precision`, `Encoding` and `Door` at the trait table
(section 1.23). **This document does not assert a replacement count**, because the trait table's members and
D69's axes are different populations and a count that cannot be checked against a list is the shape section
1.12 already refuses. The table above matters because the stale count was a **premise in a live finding**
("delivery is not one of the ten axes", Thread B, section 5), which is worse than an undefined number
(`113:414-425`, `111:279-284`).

**And op held the completeness claim open in those words.** `12b:28-33`: the arithmetic-fidelity axis is "a
proposal, not an adoption"; op "holds it open rather than reopening the ratified set"; and **"It stays
attackable, and so does the claim that the ten-axis set is complete."** The fidelity *mechanisms* are
droplisted in section 6 and correctly; **op's hold on the axis, and his statement that a completeness claim
over the axis set stays attackable, are separate from that**. Whether a completeness claim still stands over a
set that has since lost three members is a different question from whether any one member is open, and op left
it open.

**`Widening`'s three old instances (`None`, `InContainer`, `PerOperation`) decompose entirely into three
pre-existing mechanisms**: which primitive is named, what numeral type that primitive's return type is (the
`mul_full`/accumulator machinery the multiplicative half already built for an unrelated reason), and that
numeral's own `StoredWidth`/`Layout`. Measured at `-C opt-level=3`, no LTO: a direct wrapping multiply, a
composite `mul_full`-then-`quantize` call, and a `Precise`-shaped exact-widening call all fold to the identical
single instruction at native width; at a multi-limb width (128-bit operands, a real 256-bit intermediate, where
a truncated result needs fewer limb-products than a full one) the composite form still folds to the direct
hardware multiply's four instructions once the optimiser can see through it.

**`Growth` leaves the law key, ratified; and it leaves `Policy` entirely, closed by file 51, compiled in both
directions** (`58:321-339`).

**The positive enumeration, given as a list because the count that carried it is checkable and wrong at the
source.** Thirteen operations drawn from the design's surface, computed by ten growth traits, every one generic
over the operand numeral type(s) alone and none taking a `Policy` parameter: in-numeral `add`, `sub`, `mul`
and `div` (four operations sharing one growth trait, since none of them grows); `mul_full`; `mulnum` over
`Ranged`; `div_exact`; `div_floor` and `rem` (two operations, two traits); `fold`, `fold_sequential` and
`fold_compensated`; and `quantize`.

**The miscount is at the source rather than in transcription**: `51:56-64` says eleven, and `51:245` says
"none of the eleven trait declarations", while `51_probes/probe_1_growth_surface_enumeration.rs` declares
**ten** growth traits over **thirteen** operations, counted from the file. So eleven matches neither of its own
two candidate lists (`115:175-185`). Both numbers are stated above from the probe, and the enumeration is what
carries the claim. **Nothing downstream moves**, because the structural theorem is the result the universal
conclusion rests on, and it quantifies over every operation the design could have rather than over this list,
which is exactly why the miscount cost nothing beyond checkability.

**The structural theorem, which is the stronger of the two results**: every operation the design has or could
design computes its result numeral inside one trait impl, and for that impl's answer to vary "by policy"
without the parameter being inert, two impls disagreeing on the answer would have to coexist for the same
generic domain, and coherence refuses that outright (`E0119`), independent of which operation or which two
numerals. So the result is not "checked thirteen operations, found none" but **"no operation expressible in
this type system's dispatch discipline can have policy-dependent growth."** The positive enumeration and the
universal conclusion it licenses were dropped at `63:255-259` (`109:368-369`). `Policy` carries `Quantisation`
alone.

**One overtake recorded at `101b`** (`102:194-204`). The design round `202607300800`, decided the day before
this panel opened and unread by the panel in content until the tenth stretch, had independently proposed a
`Growth` policy axis (`FullPrecision`, `KeepLsb`, `KeepMsb`, `Specify`) and had renamed its own D34 (from
`202607291800`) finding (`Warm`'s doubled headroom) as an instance of it. The panel's later ratification at
`39b` removed the axis entirely, op-checkpointed, after the round had already locked its own version. **The
round's `Growth` vocabulary is dead; D34's own content survives, wearing different clothes, as the
`StoredWidth` axis's `doubled` instance on `Lowering`, with the container never declared and always
projected.** D34's principle stands; its vocabulary does not; the overtake is op-on-op, later checkpoint
winning over earlier round text, provenance clean at every step.

### 1.11 The value-unique encoding

Ratified in full at `44b`, sealed, priced. From `68:336-340`, `91:243-245`, `102:212-224`.

**Value-uniqueness is what makes "keyed on the encoding" and "keyed on the value" the same statement**, which
is the fact that collapses the notation's three apparent identity layers to two, and it is what the
layer-keying rule ultimately buys: the encoding is the value's representative, not merely a convenient carrier
for it.

**One overtake recorded at `101b`, the round's own D36 (from `202607291800`) answered from the panel's later
side** (`102:214-220`). The round left "the shape of the fix" for a UNORM-shaped encoding open, naming an
MV-chain as one candidate. The panel's ratified `Adjustment` grammar closes it: **a closed-interval numeral,
the kind a rotor component, a direction cosine, or a normalised colour channel all are, is not a new type. It
is `Adjustment = 1/(r^F - 1)` on an ordinary significand, a value of a parameter the identity contract already
seals**, at exponent `e = 0` (the same constant section 1.1 writes as `r^F / (r^F - 1)` at `e = -F`; the
convention travels with each spelling because the document carries both), compiled to `F = 24` with no new
mechanism (section 1.28). Round and panel converged on the same mechanism under different names; the MV-chain
observation survives as the literature name for what the instantiation is.

### 1.12 The seal: eleven firings, capacity closed, the niche vocabulary narrowed, the trusted base at one sentence

**Eleven firings stand**, enumerated at the spine rule above rather than counted here. **Capacity needs no
seal**, on grounds two independent reads established and which are not file 79's own (`91:249-256`):
`Capacity: Nat` has no generic parameter slot, so any foreign impl requires `Nat` first, and `Nat` is already
sealed by the carrier-at-birth mechanism; the uncovered-type-parameter forgery route the `Arity` carrier
needed sealing against has no analogue here, because the supertrait itself, not merely the derived trait, is
closed to outside implementation.

**The seal-as-free-diagnostic dividend has arrived independently at six carriers** (`68:344-354`), and the six
are named rather than tallied, because a count cannot be checked and a list can: `Rad<P>` (files 56, 62); the
strategy door's `HostImplemented` marker (file 59); the notation macro's `Bias`/`Adjustment` constructors,
unreachable from outside the tower crate (file 61); `Arity`'s sealed `Fin<P>`/`Unbounded` pair (file 64); a
per-width `WidthFor<Family>` table (file 65); and `NumeralFace`'s own coarsening bound, whose free-diagnostic
character is the layer-keying rule's enforcement mechanism (file 67). **Six independent arrivals is past the
point of coincidence**; it is a property of how this design's sealed carriers behave under rustc's own
trait-resolution diagnostics, and it is worth stating as a design fact rather than logging each new instance as
a surprise. The eighth consolidation carried a count with no names (`109:466`).

**The honest limit on "closed"** (`63:276-278`, dropped at `68:342-354`, `109:390-393`): **the enumeration is
verified as "every attack found lands in one of the routes," not as "the routes are the whole space."**
`58:370-372` explicitly instructed that the quantification block carry that phrasing rather than the stronger
reading, and its removal converts a sampled result into an unqualified one. It belongs in the seal's own
statement, which is where a reader meets the headline.

**`NonZeroCarrier`: the vocabulary renamed, narrowed, and its audited entry shrunk to one sentence.** Ratified
two-read complete (`102:234-250`). File 92's fabrication attack held: the seal holds over every introduction
route by a distinct compiler error class (direct impl `E0277`, supertrait `E0603`, layout-identity wrapper
`E0277`, non-member instantiation `E0277`, re-impl and downstream blanket closed by the orphan rules with no
probe needed), with two positive controls, and upstream growth closed by construction, since membership is
per-type impl and the language has no way to quantify over "carries a niche". What did not hold as adopted was
the vocabulary's wording: **`char` is a counterexample, not a member** (its validity set has a non-inhabitant
strictly between two inhabitants, so the audited entry's "single excluded run at zero" sentence is false of
it); **`bool` is redundant**, expressible by an ordinary field shrink at zero cost; **a reference is a locus
error**, since a byte-image carrier has no lifetime parameter. Only the `NonZero` family is exactly right and
exactly alone. The rename follows the seal's own principle, one entry per shape rather than one entry per
phrase: the genus name `NicheCarrier` should not wear one shape's own name, and it stays free for a second
shape's second vocabulary should one ever earn admission. **Ratified: `NonZeroCarrier`, the `NonZero` family,
closed and enumerated, one entry per shape.**

**The audited entry's trusted base is one sentence** (`102:252-267`). The entry originally bundled four facts;
three are const equations over type parameters and belong at declaration, per the pricing pillar. **The
trusted base is the excluded pattern's unreachability in safe code per the member's own documented contract in
`core`.** Inhabitant-totality of the decode (domain cardinality equal to `2^w - 1`, or a declared cohort-style
many-to-one decode, the identical two options the fields level already has) is a declaration-site const refusal
on the pairing of numeral and niche carrier, confirmed unconditional at file 97's second read: under this
refusal every declarable pairing has a total decode, so file 95's proposed door-side domain-preservation
equation guards a region the ratification in the same checkpoint already forecloses, and is retired to a
compile-fail pin rather than kept as spec text, **two organs for one fact being a defect this review has now
paid for three times**. The width claim rests on discriminant elision, documented by std only for the `NonZero`
family, and moves to a construction-door const assertion in the stack's own `Maybe`-shaped vocabulary, the
mechanism `notko` already ships (`notko/src/maybe.rs:40-45`, `MaybeNull`'s per-instantiation
sealed-bound-plus-const-assertion pattern). The no-wrap scope condition is already per-witness arithmetic and
needs only to be stated as a declaration-site refusal.

**The mutation gap is real, and the repair is a two-tier construction whose theorem's domain is stated**
(`102:269-294`, `91:598-626`). The only-door rule as written covers observation, not mutation;
`repr(transparent)` makes every bit writable through a transmute for the identical reason it makes every bit
observable, and this design ships raw mutable doors regardless. Compiled and executed: an ordinary safe `&mut`
into a padding region, needing no niche and no unsafe transmute at all, leaves every value-keyed read correct
while the raw byte image and any raw-byte-keyed digest decorrelate silently from a fresh construction of the
identical value. File 95 compiled the domain the first wording left unstated: "no raw accessor" undercounted
the perimeter, missing a public field, a safe `union` field write, and the `Bitpacked` layout's own granule,
where the byte owner at column granularity is the column, not the element, so a column exposing `&mut [u8]` of
its backing bytes reopens the gap with zero `unsafe` anywhere (per-element reads agree after the write, digests
diverge).

**The repaired statement, adopted.** The theorem runs **per byte-owner and per level**, mirroring statements 0,
P, and C. The safe surface of a level's byte owner (the carrier at `Dense`, the column group at `Bitpacked`)
exposes no public field, no `DerefMut` to the representation, no foreign-bytes constructor outside statement
C's named obligation site, and no accessor below that level's own write granule. Whole-value replacement and
interior mutability are inside the safe surface and preserve canonicality by move semantics. **A Rust `union`
anywhere in the chain would be a safe raw door and is excluded by name.**

**The terminating clause, second-read confirmed with its one undefined term repaired**: the statement runs over
every level whose byte owner is a type this design ships and terminates at the first level whose byte owner is
a **consumer type** (not "constructs", which would terminate nowhere under the no-heap rule); beyond it the
postcondition is one trusted-base entry stated once at the hand-over, discharged provably wherever
consumer-held bytes re-enter through statement C's foreign-bytes constructor, **so the entry governs residence
rather than return**. File 88's finding that a datum-keyed digest is immune to this gap in one operation
narrows its urgency and blast radius (raw-byte-keyed consumers only) without narrowing the theorem's domain,
since the free raw-buffer shortcut is the economically load-bearing digest path under `Bitpacked`.

**The general clause behind both repairs, adopted at `90b`** (`91:620-626`): `repr(transparent)`, together with
any hand-off of the underlying representation (a niche's validity range or a raw mutable accessor), **moves a
claim from the provable tier to the trusted-base tier regardless of which language mechanism makes the hand-off
convenient**, and the design's obligation is to name the hand-off once, in trusted-base vocabulary, never in
the provable tier's words. Whether the workspace-wide perimeter rule gains this clause is op's own hand, with a
lean toward yes.

**An integer-typed door onto a niche carrier is forbidden outright; a niche-typed door is safe and
unconditionally domain-preserving** (`102:296-307`). File 92's combined-case compile found the two working
shapes interact decisively: the identical integer-typed raw door shape whose violated postcondition costs
decorrelation on a padded carrier costs **undefined behaviour** on a niche carrier, with **zero diagnostics on
the violating store**, quieter than any transmute the review had previously found. The repair is to type the
door at the niche member, which returns the soundness obligation to the type system unconditionally: every
store is a safely-constructed member of the niche type, which cannot carry the excluded pattern, so no
obligation sits on the door at all. **This is not the policer posture**: a door whose violation is undefined
behaviour has no consumer who owns the consequence, because the optimiser owns it, which is a different case
from the toolbox rule's own protected choices.

**The perimeter block, confirmed whole at `108b:95-102`.** Op walked it individually: the sealed niche
vocabulary attacked at four introduction routes each refused at a distinct error class; the audited entry
shrunk to **exactly one** irreducibly trusted sentence with the other three moved to declaration-site const
refusals; the mutation theorem quantified per byte-owner and per level; and the integer-typed door on a niche
carrier forbidden outright with the niche-typed door safe, **because typing the door at the niche returns
soundness to the type system rather than to a rule someone has to remember**.

*Grounded on: ratified (`90b` the working shapes, `95b` the seal narrowing and entry shrink, `97` the
domain-preservation retirement and terminating-clause repair, `108b:95-102` op's own confirmation of the whole
block), settled shapes (`74:683` D16, `55`/`62`/`64` the `Arity` seal precedent, `88:228-249`), compiled
(`92_probes/` in full, `95_probes/probe_1`, `probe_1b`, `probe_2`, `probe_3`, `97_probes/probe_1`,
`probe_1b`), verified at source (`notko/src/maybe.rs:30-45`, `arvo-transparent/src/lib.rs:64-125`).*

### 1.13 Division: dissolved, and the design gained a general failure classifier in the doing

**The operation surface, adopted at `44b` exactly as file 43 stated it and confirmed whole at `108b:73-75`.**
Three operations, not two: `div_floor` and `rem`, each exact, each partial on the divisor's nonzero-ness, bound
by a compiled Euclidean law; general `div` kept, the atomic `quantize(exact quotient)`, implemented from the
pair. Dropping `div` would be the policer posture `arvo-toolbox-not-policer.md:33` forbids by name, and this
review already rejected the identical shape for `quantize` one section over. Partiality is refused at
declaration wherever the divisor's domain is a predicate, and carried in a niche otherwise, at no layout cost.

**File 43's own findings survive the five stretches since `44b` unchanged** (`91:286-294`): the accumulator
width is exponential in precision, a third growth class distinct from addition and multiplication (recomputed
independently, six of seven cells reproduce exactly, `p = 5` differs by one bit, an off-by-one in the
value-range term rather than a disagreement); the exact subfamily is division by a fixed nonzero representable
constant, at zero new mechanism, independently corroborated by file 84's identical move for `quantize`'s typed
quantum (both lift an operand out of value position into type position to make the operation value-keyed and
law-eligible); the overflow band is empty for same-precision division and inhabited once precisions decouple.

**The `x/0` fork, adopted at `90b` as an instinct pending a stress test, is dead, and both alternatives were
dead the moment the question was asked correctly** (`102:326-341`). The instinct placed `Hot`'s cell on the
`Door`. File 93's attack read its load-bearing sentence ("the target's own divide instruction defines the
answer") literally and checked it against four compiled or silicon-read facts. **aarch64 returns 0 for a zero
divisor at every dividend, including a zero dividend**, erasing the very two-way split the placement existed to
preserve. **x86-64 defines no value at all**; raw `idiv` faults and the process dies. **RISC-V defines a third
value.** **LLVM's own IR carries undefined behaviour on a zero divisor on every target, and the optimiser uses
the licence**: a zero-divisor guard placed after a division is deleted, on both compiled targets, so what the
target gives away for free the toolchain takes back before the design ever sees it. A safe, total operation
delivering target-varying values from identical operands has no precedent anywhere in this design. And file 95
showed the fork's other alternative carried the identical smuggle at a different address: op's own `Warm`
derivation, the shipped tree's own convention, and the instinct's `Door` placement were **three independent
inventions for the same cell with no derivation among any of them**, which is the strongest evidence available
that a value is being authored where none should be.

**The ground for the reversal was already ratified, and the deletion test is its decision procedure, not new
canon** (`102:343-357`). The operative clause is section 1.3's first sentence, "`Lowering` changes no value",
unamended since file 40. A cell whose content is 0 on ARM, a fault on x86, and a third value on RISC-V is
`Lowering` changing a value, forbidden outright, no new sentence needed.

**The deletion test, adopted as that clause's decision procedure, which it never had.** State the cell with the
`Lowering` deleted. If a value-layer sentence remains (a resolution-vocabulary member, a constant derived from
the numeral's parameters, or a parameter the consumer names), the lowering implements it. If nothing remains,
the lowering authored it, forbidden. The admissible remainder is a **closed condition** (no free term keyed on
the `Lowering` axis survives the deletion), with the three named kinds as examples rather than the exhaustive
list, and the test's scope is value-layer cells; datum-keyed facts (the digest's masks, the byte image)
legitimately read `Encoding` and are outside it by section 1.3's second sentence.

**The residue worth more than the cell, ratified at `108b:73-75`**: **a lowering may be a derivation input or
an implementation of a stated value, never the author of one**, with the deletion test as its decision
procedure.

**The solution-set derivation, confirmed in all three clauses at `108b:56-66`.** Division is the inverse of
multiplication: the exact quotient of `x` by `d` solves `q · d = x`, and **the solution set has three shapes,
with the failure vocabulary a function of the shape rather than of a target**.

1. **Singleton.** An ordinary quotient; the quantiser and range machinery govern as ratified.
2. **Empty with a direction** (`x/0`, `x` nonzero). The one-sided limit supplies a direction, and the failure
   **borrows the range event's own resolution row in that direction**: clamp reaches the far point, a numeral
   carrying infinity delivers it as the absorbing far point and raises `divideByZero`, refuse refuses. The
   limit fixes the dividend at exactly zero and drives the divisor, **with no licence to also perturb the
   dividend**, which the original limit argument's `0/0` clause had done without saying so, a false clause
   repaired rather than restated.
3. **Everything, or empty with no direction** (`0/0`, and square root of a negative). Nothing is privileged and
   there is nothing to lean toward, so the event is **`invalid`**, resolving to NaN exactly where the numeral
   carries one and leaving the operation **partial** at that input where it does not (refused at declaration
   where the divisor's domain is a predicate, carried in a niche otherwise, at zero layout cost).

**Both halves of IEEE clause 7 become theorems rather than citations. `Recip` at zero inherits the reading with
no list edit. No third position on the `Resolution` axis is needed**, and the "third position" need shrinks to
two mechanisms the design already has, NaN and partiality (`108b:67-68`, `102:359-374`). The sign convention
for a single-zero fixed-point numeral (no signed datum to derive a side from, so the `+0` convention is stated
once as a citation-shaped clause rather than derived) closes the last unstated term, per the
definitional-completeness line.

**`Hot`'s cell goes to the consumer, ratified at `108b:68-70`.** The record holds **three independent
inventions for it with no derivation among them**, which by the toolbox rule's own decision test means the
consumer knows and the design does not. **A total form over a possibly-zero divisor takes a fallback the
consumer names, `div_or`-shaped, with arity two, not one** (`102:380-387`): a single fallback collapses the
distinction the solution-set derivation just paid to establish, since for a NaN-less numeral the directional
cell and the indeterminate cell share the same input, `x = 0`, and one value cannot answer both without either
denying `0/0`'s own partiality or being unreachable at the directional cell. **The fallback's arity is the
arity of the failure taxonomy at that input**: a directional fallback for the empty-with-direction cell, an
indeterminate fallback for the everything-or-nothing cell.

**The slot's domain** (`102:387-395`): the fallback fills the cells the numeral's ratified resolution row
leaves unanswered, and **does not shadow a row that already answers a cell**. At `Warm`/`Cold`/`Precise` the
directional cell already has a clamp or far-point answer; only `Hot`'s `ReduceModulo` and the NaN-less `0/0`
are genuinely holes. A per-call override of a ratified resolution row would be a second home for resolution
policy, the two-organs defect this review has now paid for three times, and the design has one home for it, the
`Resolution` axis itself.

**`Hot`'s fast body survives with none of the disputed authorship**: where a consumer's fallback coincides with
the target's own constant (0 on ARM), the lowering emits the bare divide as an implementation of the stated
function, seven instructions, Kind-1 cfg-gated per `arvo-always-optimal-internals.md`, measured. **The `Door`
decides cost, never content**, is the entire legitimate residue of the original instinct, and it is enough.

**The float cause split, compiled and agreeing with the hardware** (`58:380-386`, dropped at `63:280-282`,
`109:355-356`): `x/0` with `x` finite and nonzero delivers a correctly-signed infinity and raises
`divideByZero` only; `0/0` and `inf/inf` deliver a quiet NaN and raise `invalid`; `inf/0` delivers infinity and
raises nothing, since `divideByZero` is defined only on finite operands. The value half of every case agrees
with the machine on 300 class-level cases; the cause half cannot be observed on this toolchain at all (section
1.14). Division's growth class is `Theta(2^p)` accumulator bits.

**Owed, named with closing artifacts.** The signed halves of file 43's probes 2, 4, and 5. The float-division
compile against a `Specials`-bearing model numeral, now also owed by the elementary functions chapter's `ln`,
one artifact serving both. The IEEE clause 7 and 7.6 primary-source reads, joined by the ISA bundle (ARM DDI
0487, Intel SDM, RISC-V unprivileged ISA, LLVM LangRef). **Division's grading axis: genuinely op's own, held
since checkpoint ten, untouched by the dissolution**, which resolved the fork without touching the axis
question.

*Grounded on: ratified (`44b`, `90b` the instinct, `95b`/`101b` the dissolution, `108b:56-75` op's own
confirmation of all three clauses plus the lowering-authorship residue and the `Hot` delegation), settled
shapes (`43` in full, `84:145-160`, `84:239-263`, `84:419-427`), compiled and silicon-read
(`93_probes/probe_1` through `probe_4`, `95_probes/`, `97_probes/probe_3`, `50_probes/probe_6`), external (ARM
DDI 0487, Intel SDM #DE, RISC-V unprivileged ISA, LLVM LangRef, all secondary, primary reads owed).*

### 1.14 The grade: two counts at two layers

**Confirmed by op at `108b:45-52`, after seeing both readings.** They are not two definitions of one number.

The design's grade was ratified as a free commutative monoid over refusal causes and quantisation events
(`37:507-511`). What was never given was a membership predicate for the generator "event": an operation may
contain a **quantiser site**, a place where an exact intermediate is rounded onto a target grid and classified
against a range, and whether a site's rounding **changes** the value is a fact about the data reaching it, not
about the operation's type (`91:336-341`).

**The site count.** How many quantiser sites an expression has, `n - 1` for any grouping, a function of the
monomorphised type, **one instruction outside the loop**, and it is what gets published. It is what `Folded<N>`
as a type parameter can be inhabited by at all: a moved count is refused with `E0435` when asked to fill it,
compiled.

**The moved count.** How many of those actually changed a value. Data-dependent, **the standard's own inexact
condition**, seven to nine times more expensive on a float path and destructive to unrolling, read by law
verdicts and **never published by default**.

**The design was committed to both at once, in different ratified organs, for forty-odd files, and nobody
noticed because the two readings agree everywhere the review had checked them** (`91:355-367`). Region 1 (the
exact family) is vacuous under both. Region 2 (the fold family's own laws) hides the difference: every grouping
of an n-element fold has exactly `n-1` sites, so under the site count the event component of every
associativity law holds by construction, for every input, and file 37's own view lattice measures nothing
there. Region 3 (a conditional resolution, a conditionally exact quotient) is where the two readings differ,
and it is exactly where file 37 measured and file 43 assumed: on signed `Hot` wrapping, the finest view moves
from `(Exact, Ignore)` under the moved count to `(Exact, Exact)` under the site count, and **the whole measured
lattice collapses from a genuine incomparable pair to a chain under the site count**, which is the review's own
strongest evidence for building a finest-view lattice at all rather than three named relations.

**The ruling** (`91:369-387`). The two counts key at two different layers by the layer-keying rule's own
coarsest-layer test, and a design carrying both keys each where it belongs rather than picking one and calling
the other its approximation. **The site count is an operation-layer fact**: the published grade's own
parameter, a sound pessimistic upper bound on the moved count in the same over-approximating direction the
design takes everywhere on lattice containment, what a caller's `Folded<N>` contract carries and what a
compile-time worst-case error bound reads. **The moved count is a value-layer fact**: IEEE's own raising
condition, read by law verdicts and conformance compositions, never published by default, because a numeral
publishing it needs bookkeeping the ratified `Warm` float row forbids by name (correctly-rounded intermediates
delivered free by the hardware, invisibly; adding a flag read or a recompute step is exactly the bookkeeping
that row rejects). A numeral wanting it takes `Door = Quantised`.

**Measured**: one instruction outside the loop for the site count against **seven to nine times** the
instruction count for the moved count on the float side, either route (an `mrs`/`msr` FPSR flag read that stops
the loop unrolling entirely, or a five-operation recompute); on the fixed-point side both counting and presence
at the moved-count level cost the same **5.0 instructions per element**, doubling the loop body, so the
intuition that presence is the cheap level is false on this shape.

**Multiplicities live in the underlying monoid, appear only inside a law's verdict, never in a shipped
carrier.** A shared operand contributes once under an idempotent join. One derived consequence worth carrying:
the site count at the `Exact` view is not computable from what a value carries, only from the term at compile
time, because two values with identical grades and identical operations must give the same answer under a
value-carried grade, and the site count under sharing does not; the moved count is computable from either. The
two readings live at different places in the design, not two settings of one dial.

**The grade is IEEE's flag word, and the design's own carrier is strictly the better one** (`58:395-409`). Over
the five clause-7 exceptions with no multiplicity, a free commutative monoid joined by union **is** a five-bit
word joined by bitwise or, and the design's code needed no adaptation to serve as either. The two-part
generator split lands exactly on the standard's own: inexact and underflow are quantisation events; invalid and
divideByZero are causes with no quantiser origin; overflow is raised by the classification step, which is the
quantiser's second half. Two consequences. First, the design's grade rides on the value rather than on a
per-thread accumulator, which is strictly sounder under a pluggable executor, because a per-thread register is
nondeterministic on unchanged data (the thread partition is the executor's choice, not the value's). Second,
and stronger: **the standard's own carrier is not merely worse, it is unavailable.** A grep of the pinned
toolchain's `rust-src` for `fetestexcept`, `feclearexcept`, `fegetround`, `fesetround` returns zero files;
there is no FPCR access in `core::arch::aarch64`; `_mm_setcsr` on x86 has been deprecated since 1.75.0. **A
design that wanted to mirror IEEE's flag mechanism could not read it. The value-carried grade is not
preferable, it is the only carrier that exists.** That unavailability finding was dropped at `63:284-289`
(`109:354-355`) and is the reason the design's carrier choice is not a preference.

**`Specials`' value half checks against the machine on all three operations, 300 cases, zero mismatches**, with
one real subtlety: finite plus finite is not decidable at the class level, since exact cancellation delivers a
zero of a different class, so the specials table is not a total function from classes to classes. NaN payload
propagation is silicon and is not commutative at the datum level (`qNaN(1) + qNaN(2)` and `qNaN(2) + qNaN(1)`
differ in the low payload bit); it is commutative at the value level, which is the design's already-ratified
value and datum split doing its job.

**Every "the grade" in the corpus now says which count.** `Folded<N>`'s own `N` is unambiguously the site
count, the moved count riding beside the value where a conformance surface asks for it. **Where the witness
rides**, as a returned value beside the fold's result or as a parameter on the result numeral, is one line in
section 5; the count itself is settled and compiled.

*Grounded on: ratified (`39b` via `37:171-179`, `78:137-150`, `90b`, `108b:45-52` op's own confirmation),
settled shapes (`37:441-444`, `49:464-546`, `50:294-307`, `58` section 1.14, `78:448-455`), verified at source
(`37_probes/probe_1:169-210`, `43_probes/probe_5:15-24`, `40:281-283`), compiled (`89_probes/probe_1` through
`probe_3b`).*

### 1.15 The exponent forces the spine rule open a second time

From `49:552-570` (`109:192`).

**Reasoned, a first read, not compiled at the time; compiled since (section 1.2).** The exact-widening family's
numeral-level maps compute result numerals from operand numerals. For `Implicit` numerals every computed member
is already a type (adjustment through `Reduce`, bias through `BiasProduct`). **For `Ranged` numerals the exact
product's exponent bounds are `EMIN1 + EMIN2` and `EMAX1 + EMAX2`, arithmetic over const parameters whose
result must appear in the result numeral's type**: a const computed in type position, the identical wall that
already pushed width arithmetic out of const generics. So the moment the exact family reaches `Ranged`
numerals (which `ExactWindow`'s own `Specials = None` gate already contemplates), **the exponent must become a
type or the family is unwritable there.** The fork the third consolidation carried as "a real fork nobody has
opened" is opened by the spine rule and answered yes.

**This also supplies the argument the `Int`-drop lean was missing.** The strongest objection to dropping `Int`
is that a future signed exponent might need a signed encoding, and the spine rule's derivation shows that
consumer, once it exists, lands on the constructor-sign shape `Bias` already uses (`EZero | EPos<P> | ENeg<P>`
over the sealed `Pos`), not on `Int`. Section 1.23 states the drop; this is why it survives the future case as
well as the past one.

### 1.16 The float model, and the elementary functions given their own chapter

**The far point is the supremum of a numeral's ordered representable values.** Ratified in full at `74b`
(`78:275-285`). **Three instances of one rule, not three cells**: an IEEE-shaped numeral's supremum is the
signed infinity; a no-infinity float numeral's supremum is its largest finite magnitude, a member of the set; a
fixed-point numeral's supremum is the already-ratified `Warm`/`Cold` clamp cell, which file 11 had already
expressed in exactly this shape ("clamping above the range is simply `TowardNegative`... at a bounded top there
is one candidate", `11:195-196`). **NaN needs no exclusion clause**: the supremum is taken over the *ordered*
values, and NaN is not in the order, so the agreement between the two no-infinity `Specials` members is a
theorem of the definition rather than a case written into it. Compiled as a total, const-callable projection
across the whole four-member `Specials` product, no feature gates, no refusal anywhere.

**The overflow boundary and the tie** (`78:287-296`). An out-of-range event begins past the extended-grid
rounding boundary, half a top-binade ulp beyond the maximum, with the tie resolved by the ordinary even rule on
the extended grid. For every IEEE format the maximum finite's stored significand is odd (all-ones), so the tie
rounds up, off the finite set, delivering the standard's own overflow-at-the-tie behaviour with no directional
constant needed. **E4M3's maximum finite is even** (its all-ones slot in the top binade is the NaN encoding
instead), so the identical rule delivers the opposite parity: the tie at 464 rounds down to 448, and
`(448, 464]` is ordinary in-range rounding rather than an overflow event at all. Compiled at the E4M3 model.

**The cost, stated rather than smoothed** (`78:298-306`). A come-back sum (`(448 + 448) - 448` at E4M3)
saturates to 0 against a true 448, a silent, full-scale, in-range error, and this is the worst number the
stress test produced. Three things bear on how much it weighs: the ratified fixed-point table already accepts
the identical shape of error under clamp; the alternatives (NaN-on-overflow, refusal) do not recover the true
value, they only fail louder, destroying the entire remainder of a downstream fold rather than delivering a
bounded, ordered, usable wrong answer; and the multiplicative half already removes the middle stress case (a
product routed through `mul_full` never overflows at the intermediate), leaving only the additive come-back
exposed, which no width short of the true accumulator range removes and which the grade must therefore witness.

**The mitigation, compiled in shape** (`78:308-316`). The far-point projection publishes a kind, `Absorbing`
(the far point is an infinity, self-witnessing in the datum) or `Finite` (the far point is finite, silent in
the datum), total over the `Specials` product. The kind joins through a fold with **silence dominating**: the
published grade records `Finite` the moment any operand's far point is finite, checked over the whole
two-element carrier's four join laws in const context, not a sample. A consumer needing the in-band witness
states it as a bound (`AbsorbingFarPoint`) and is refused a finite-far-point numeral at the call site. Whether
the kind is a parameter of the existing overflow grade generator or a sixth generator is left to the grade
machinery's own keeper; both are expressible with no gates.

**The well-formedness alternative, declined with teeth** (`78:318-326`). Refusing a no-infinity numeral under
`Warm`/`Cold` at declaration would forbid the design's own deployed `Specials` witness, E4M3, under the exact
preset (`Cold`) whose deployment profile matches E4M3's real silicon use, and it crosses the warn-never-police
line by name. **NaN-on-overflow was given its own hearing** (it is a real deployed OCP mode, not hypothetical)
**and declined for the preset table on four grounds**: it cannot close the cell alone (`NoSpecials` has no NaN
either), it surrenders the total order, it manufactures the design's already-catalogued NaN-poisoning defect
from a range event rather than avoiding it, and it is a deployment mode rather than a resolution constant, so
if the review ever wants it, it belongs on the hardware door's `FloatEnv` fact, not on `Resolution`.

**`Underflow`'s two instances** (`Gradual` | `Abrupt`, sealed, both change representability), **`Specials` as a
four-point product** (`NoSpecials`, `NanOnly`, `InfOnly`, `IeeeSpecials`, sealed), and the **`TotalOrd` split**
(section 1.20) stand. Flush-to-zero is a `Quantisation` resolution rather than a `Numeral` fact, droplisted in
that form (section 6).

**A failure is a point at which an operation cannot produce a datum of its result numeral, and there are
exactly three kinds** (`91:421-429`). **Kind 1**, the result value exists and lies outside the result numeral's
value set: a range event, the `Resolution` axis's own domain, best read as a totalisation axis whose four
members are four ways of making an otherwise partial operation total, ordered by how much they lie. **Kind 2**,
the result value does not exist mathematically: division by zero, `Recip` at zero, `Sqrt` of a negative in a
real domain, no range involved, its grade generators `invalid` and `divideByZero`. **Kind 3**, the operand is
not a datum: closed by construction, statement 0 already covers it, not a runtime failure kind in this design
at all.

**`quantize`'s hard failure is kind 1, not a new kind** (`91:431-435`). The operation targets `At<N, Q>`, the
numeral sharing `N`'s radix, precision, and domain with its exponent fixed at the requested quantum `Q`, which
is not new vocabulary but the constant exponent function the design's founding identity already requires the
sealed `Exponent` vocabulary to express.

**The quantiser's domain widening** is stated at section 1.5 and is what legalises what follows.

#### The elementary functions past division

**New at `101b`, confirmed whole at `108b:77-93`.** File 99's method is the one that carried multiplication and
division through: for each function, ask what the exact result is, what finite object decides its rounding, and
how wide that object grows. **The answer sorts the whole family into three classes by the kind of the exact
carrier**, and the solution-set derivation (section 1.13) classifies every domain event in the family without
edit, which is the derivation's second family and the strongest evidence it is the design's general failure
classifier rather than a division-specific mechanism.

| Class | Members | Exact carrier | Decision width | Ties | New mechanism |
|---|---|---|---|---|---|
| Derived | `recip`, integer `pow` | division's pair; the fold | inherited | inherited | none |
| Root | `sqrt`, fixed n-th roots | the root-residue pair `(m, r)` | linear, `P + F` bits | impossible, a parity theorem | none |
| Radix-power exponential | `exp2`-shaped, matching-radix grids | integer power comparison | exponential in `F` | impossible off integer exponents | none, refused at practical width like division's accumulator |
| Transcendental | `exp`, `ln`, `sin`, `cos`, `atan` | none exists | empirical, the hardness constant | impossible, Lindemann-Weierstrass | one: the hardness constant's provenance class |

**Two members are derivations, not chapters, and inherit everything.** `recip` is general division with the
dividend fixed at one, `quantize(1/x)`, not the exact subfamily (which lifts the divisor to type position;
`recip` fixes the dividend), inheriting the general quotient's quantiser path, growth class, and grading.
Integer `pow` is an iterated `mul_full` with one quantisation at the root, the fold chapter verbatim, with
`x^0`'s domain corollary (starting at one over an identity-free numeral) falling out of the `Identity` bound
with no new text.

**The root family joins multiplication's linear growth class, the cheapest new instance available.** For
same-grid unsigned sqrt at operand index `k` and quantum `2^-F`, the exact result index is
`t = sqrt(k * 2^F)`. The finite exact carrier is the pair `(m, r)` with `m = isqrt(k * 2^F)` and
`r = (k * 2^F) - m^2`, bound by `m^2 + r = k * 2^F`, and **correct rounding to nearest is the single comparison
`r > m`**, by squaring the midpoint, compiled exhaustively at nine `(P, F)` shapes against a definition-shaped
oracle that never computes a root. The widest integer the decision touches is the scaled operand itself,
`P + F` bits, so the root family gains a linear-growth-class member **without the design gaining a fourth
growth class**. **Ties are structurally impossible, a parity theorem**: a rounding tie requires `4r = 4m + 1`,
even equal to odd, zero ties at every sweep, and the classical shadow of this fact is that hardware square root
has never needed a tie path. The overflow band has a closed-form emptiness criterion and is inhabited exactly
on the identity-free numerals (section 1.28's own finding, the same absent element that breaks the
multiplicative identity opens the root's overflow band, discovered independently by two members in the same
stretch). Domain: total over `Domain = Unsigned` by the existing `SignDomain` axis (the
divisor-domain-as-predicate clause reused, not a new niche); over signed domains, the negative half is the
solution-set derivation's own everything-or-nothing clause, inherited. In const position the correctly-rounded
sqrt folds to the guaranteed constant; in value position, after the root extraction, **the entire
correct-rounding decision is three instructions, branchless**.

**The radix-power exponential is decidable and exponentially wide.** `2^(k/2^F)` is algebraic; exact hits and
ties on a dyadic grid are exact integer comparisons (exact hits only at integer exponents, ties never), but
**the comparison object doubles in width per fractional bit** (13, 25, 57, 113 bits at `F = 1` through `4`),
decidable in principle and refused as a practical carrier, exactly the shape of division's own `lcm`
accumulator (`43:145-155`). Even the decidable half of this family reaches the operational posture of the
transcendentals: bounded working precision, correctly rounded where the bound is known sufficient.

**The transcendentals have no exact carrier at all, and the moved count becomes a theorem instead of a
per-value fact.** For nonzero rational `x`, `exp(x)` is transcendental (Lindemann-Weierstrass, external, cited
per the review's standing practice for standard mathematics no toolchain here could check), and so are
`ln(x)`, `sin(x)`, `cos(x)`, `atan(x)` at nonzero rationals. Every grid point and every midpoint of an arvo
numeral is rational (the membership theorem), so **a transcendental result never lands on either**, off a
finite enumerable list of removable special points per function (`exp(0) = 1`, `ln(1) = 0`, `sin(0) = 0`). Ties
are therefore impossible for a deeper reason than the roots' parity argument, and the domain events (`ln` at
zero, `ln` of a negative) are solution-set-clause theorems that recover IEEE's own log-of-zero table entry as a
derivation rather than a citation. **For a transcendental member the moved count equals the site count at every
operand off that finite removable list**, the one place in the design where the expensive count is free
(`102:416-420`).

**The one genuinely new thing this design needed, and it is small: a provenance class for one constant, not a
mechanism.** Correct rounding of a transcendental terminates per value (Ziv's staged refinement, external), but
**the working precision sufficient for every operand of a numeral is not given by any formula**. It is a
well-defined constant of the type (the value set is finite, so the worst-case boundary distance exists),
passing the pricing pillar's own test to the letter while resisting the pillar's usual derivation economics:
the constant is computable only by exhaustion over the value set, or by citation of a published worst-case
search. **Measured at three model numerals: 11, 9, and 10 extra bits, zero ties anywhere, no visible formula
connecting the three.** **This constant does not transfer across widths**, unlike the laws whose model-width
checks transfer by uniformity: hardness at width `W` is established at width `W` by exhaustion or citation,
never carried across, because it is a maximum over a different set at every width.

**Op's own statement of the boundary, `108b:84-89`, which sharpens the class from an economics note into a
statement about the family**: "**The boundary is the family's own, not a weakness in the design.**" Correctly
rounding a transcendental terminates for any single value, but the working precision sufficient for *every*
operand is given by no formula. It is a genuine constant of the type, since the value set is finite so a worst
case exists, and it passes the pricing pillar's test to the letter. **What fails is the economics**: it comes
only by exhausting the value set or citing a published search, exhaustion is capped by the const-eval wall at
model widths, and published worst cases exist only for IEEE widths and some functions. So **a numeral promises
correct rounding exactly where its hardness constant is exhausted or cited, and nowhere else**, and
`exhaustively-computed-or-cited` enters the grounding registry as a provenance class.

**What the family may promise, derived from the ratified naming principle rather than invented.** The roots may
promise correct rounding, verified by the residue characterisation itself. The radix-power exponential may
promise correct rounding per width, verified by the width's own bounded sufficiency check. **A transcendental
may promise correct rounding exactly at the widths where its hardness constant is exhausted or cited, and
elsewhere ships as a licensed approximation**: a type-level error bound as the contract, an internal mechanism
(degree, table size, argument reduction) per `arvo-always-optimal-internals.md`, and the differential parity
suite op's own `79b` mandate designates as the verifier. IEEE's own posture corroborates the split (clause 5
requires correctly-rounded sqrt, clause 9.2 only recommends correctly-rounded transcendentals, secondary,
primary read owed).

**The closure sentence, adopted in the admission-test form rather than a list, because a list reopens with
every member and a test does not**, and confirmed in that form at `108b:92-93` so a reader can tell a deferred
operation from a forgotten one:

> *The operation surface is the following table, and an operation joins it by stating five things: its
> solution-set characterisation, its exact-carrier kind and width class, its grade sites at the site count, its
> result-numeral rule, and, where its name promises behaviour, its designated verifier. An operation absent
> from the table and not admitted through this test is not in the design.*

Every operation ratified as of this document satisfies all five statements. **The taxonomy absorbed a whole
family it was not built for without edit** (`108b:93`).

*Grounded on: ratified (`70b:8-10,30-38`, `74b:8-14`, `79b`, `91:284-294`, `91:420-429`, `95b` section 2,
`101b`, `108b:56-93` op's own confirmation of the chapter and the boundary), settled shapes (`11:195-196`,
`70:277-292`, `58` section 1.14, `93` section 5, `43` sections 2 and 5, `78:723`, `29:111-116`), compiled
(`71_probes/` all four, `99_probes/probe_1`, `probe_2` exhaustive const-position), measured
(`99_probes/probe_3`, `probe_4`), physical (OCP/E4M3, secondary, primary owed), external
(Lindemann-Weierstrass, Ziv's staged refinement, IEEE 754-2019 clauses 5, 7.3, 9.2, all secondary, primary
reads owed).*
### 1.17 Radix ten: the chain, the section, and where the design and the standard part company

From `58:561-623` (`109:193`).

**The chain, of which only the first link is about the radix.** Radix two normalises for free: a normalised
binary significand's leading digit is always one, so it need not be stored, and the hidden-bit trick both
enforces normalisation and costs nothing. **No radix above two has a constant leading digit to hide**, so its
significand is stored unnormalised, and a value has one datum per representable exponent shift: a cohort. The
remaining three links (unnormalised storage, the hidden digit's presence or absence, the preferred-cohort
choice) are already named by the design's own axes, `Encoding::Fields` and `Encoding::Canonical`. **Nothing new
is needed, and that is the finding: the design was built to express this before anyone checked that it did.**

**Measured, radix ten, `p = 2`, `e` in `[0, 2]`:** 600 data, 600 live (every datum is live under the
unnormalised encoding, since there is no reserved significand band the way a normalised counterfactual would
have 240 of its 800 data dead), **559 distinct values, statement 3 false and predicted false**. The value sets
of the normalised and unnormalised counterfactuals are **identical**: normalising a decimal numeral changes no
value, which means **cohorts are a choice, not forced by the value set, and the design has to know whose choice
it is**.

**`Encoding::Canonical` is a genuine choice under radix ten, and a formality under radix two.** Two natural
cohort-selection rules (smallest significand with the largest exponent; largest significand with the smallest
exponent) are the same function under radix two with a hidden digit, and different functions on the identical
value set under radix ten, with a named witness: the value 1 spells as both `1 x 10^0` and `10 x 10^-1`, and
the two rules pick different data that both decode back to 1.

**Non-canonical codes are a third, larger source of non-injectivity than cohorts, and live entirely on the
`Encoding` side.** Repacking a decimal significand is a bijection, so BID against DPD cannot itself change any
of the three statements. What is interesting is that a field wide enough to hold `10^p - 1` also holds codes
above it, which the standard reads as zero: **compiled at a seven-bit significand field, `p = 2`, 209 of 768
data are redundant, against 41 of 600 in the tight encoding.** That third source and its measurement were
dropped at `63:325-328` (`109:377-378`).

**Where the design and the standard genuinely part company, and it is the single most important sentence this
chapter produced.** IEEE 754 specifies, per operation, a **preferred exponent** for decimal results: which
cohort member an operation delivers, as a function of the operation and its operands' exponents rather than of
the result's value. The design's operations are value-valued; there is no place in that pipeline for an
operation to choose a datum, and `Canonical` cannot express the rule because `Canonical` is a function of the
value alone. Two responses are rejected (carry the cohort member in the value coordinates, which falsifies the
founding sentence that a value is a rational; make operations datum-valued for decimal, which evaporates the
algebra for one radix), leaving a third that is not a concession:

> **arvo's decimal `Ranged` numerals deliver IEEE's values and are not conformant to its preferred-exponent
> rules; a consumer for whom the quantum is part of the number uses a decimal `Implicit` numeral, where the
> exponent is a type, checked at compile time, and cannot drift through an arithmetic chain.**

That is **strictly stronger** than the standard's own rule, because it is checked rather than propagated at
runtime, and it is unavailable to a language with only runtime decimals. Compiled in support: a
single-exponent-row decimal `Implicit` numeral has no cohort at all beyond the signed zero, and dropping to a
non-negative domain makes it genuinely injective.

**The radix axis pays for itself, and the margin is a wall, not a percentage.** Absorbing a decimal quantum
into the rational adjustment (folding `A * radix^E` into one rational, which would make `Exponent` redundant if
it were free) is not merely expensive; **it does not compile at any real decimal format's exponent range.** Two
independent walls, both compiled: a `u64` readout ceiling at `10^20` (`Pos::VAL` cannot be read past it), and a
type wall at depth 130 (`Pos`'s own structural recursion against the default recursion limit, attributed by an
independent test to `Pos` itself rather than to the reduction machinery). **decimal64's own bottom grid
(exponent -398) compiles at 64 ms, flat, in the radix-and-exponent spelling, from a 519-byte source; the
absorbed spelling does not compile at all, from a 4,486-byte source.** `Exponent` is not a redundant axis; it
and `Radix` are what keep a decimal numeral off the cliff.

**The general fact**: a `Pos` may not exceed roughly `2^127` on any axis; a magnitude that would need more is
expressed as an exponent, never absorbed into a rational. Checked against every magnitude the design actually
spells (precision, exponent bounds, radix, MATLAB constants, the design's own division constants): all
comfortable. The `u64` readout ceiling at `2^63` is the tighter and more consumer-visible of the two, and
whether it should widen is open (section 5).

**The decimal `Canonical` default** is presumptive since `62b`, structurally confirmed against the
`Strategy = Warm` precedent at file 64, with one cheap confirming compile between reasoned and closed. One line
for op to kill the default entirely if he reads it otherwise.

### 1.18 The numeral notation: the macro, the face, and the two ceilings

From `63:362-436` and `68:383-427` (`109:194`).

**Closed as a vehicle question, not merely narrowed: a proc-macro, adopted, with no external dependency.**

**The declarative form does not fail because it is hairy. It cannot start, compiled twice over.** A decimal
literal is one atomic token to the lexer; no fragment specifier, no restringify-and-rematch trick reaches its
digits. The value-to-type escape (a `const fn` over the literal's text driving a recursive type-level peel)
fails identically to the exponent case, at the identical wall, confirming this is a fact about the position
rather than about the exponent specifically. One partial split is real and does not compromise the refusal: a
leading sign and an explicit `NUM / DEN` rational are each their own token and `macro_rules!` *can* see them,
which is not the ratified intent's "a consumer writes any number as a literal" and is not proposed as a
compromise notation.

**The vehicle: two entry points, connected to the sealed tower by an unsealed bridge trait declared in the
tower crate, and this shape is a structural necessity rather than a preference.** `raw_bias!(EXPR)` emits a
type expression; `numeral_face!(Name = EXPR)` emits an item, a concrete newtype implementing
`NumeralFace { type Encoding: Bias; const DISPLAY: &'static str; }`, itself deliberately unsealed since it is
an unbounded, per-literal vocabulary rather than a closed carrier. **A macro-minted type cannot implement
`Bias` directly, because the seal's private supertrait is unreachable from a different crate, and a
proc-macro's expansion crate is structurally forced to be a different crate by Cargo's own `proc-macro = true`
constraint.** The proc-macro needs no external dependency at all (`extern crate proc_macro;` alone, lighter
than the workspace's own `notko-macros-core` precedent): text parsing, digit extraction, decimal-point folding
into a rational, gcd reduction and bit decomposition are all ordinary host-side arithmetic, none of it visible
to the type checker because none of it produces a type-level obligation.

**Both ceilings, bisected to the exact bit.** A `Pos` is nameable structurally through **128 bits**, refused at
129 (`E0275`, overflow evaluating the `Pos` requirement). **`Pos::VAL` reads back through exactly `2^64 - 1`**
(64 bits), refused at 65 bits (`E0080`, a `u64` multiplication overflow at const-eval time), the exact boundary
a `u64` implies rather than an approximation of it. **The two walls are independent, and the tighter one bites
first**: any magnitude between 65 and 128 bits produces a `Pos` type that compiles, composes, and satisfies
ordinary bounds, and then panics the moment anything tries to read it back, which a ceiling guard set at the
looser figure would let through silently. **The `10^20` figure from the fifth consolidation is not folded into
either bisected number**; it stays open, plausibly a separate fact about a radix-ten accumulation pattern
(`10 * P::VAL + digit` rather than doubling; `10^19 < u64::MAX < 10^20` would make `10^20` a reasonable
just-past-the-ceiling figure for that pattern), unbuilt and unchecked. That figure left the record entirely at
`91` and returned zero hits in any panel file after `78` (`109:490`); it is on the open list.

**The vehicle's own refusal is two-tiered, adopted as the design recommendation.** Refuse at the tighter,
mechanism-specific readout threshold, print the actual decimal magnitude the macro has known since before any
type token existed, and name the refusal as the open widening question rather than a notation defect. A vehicle
refusing only at the looser structural figure defers an identical failure into a worse, later, less legible
position, for every magnitude in the sixty-four-bit-wide band between the two walls. **The structural and
readout ceilings and the adopted two-tiered refusal were dropped at `68:383-427`** (`109:402-403`), which is
the document that rewrites the section; they are the only verification evidence behind the adopted notation
vehicle.

**The whole-matrix test, and the bug it caught before file 61 existed.** **923 assertions**: hand-picked
boundary and near-miss cases plus an exhaustive block over every three-digit magnitude. The first run failed on
the brief's own near-miss family, **`37/53`**, an explicit-denominator combination that silently produced a
valid but wrong `Bias` (`37/1` instead of `37/53`) through a one-character copy error in the reduction
arithmetic. **All 923 pass after the one-line fix.** This is "red is the lifeblood" working as designed: a
strict test caught a real defect in the vehicle before any reviewer would have. Dropped at `68:383-427`
(`109:403-404`); it is the notation vehicle's entire verification evidence.

**Staging is priced, and reducing host-side rather than merely emitting host-side is the load-bearing half,
measured rather than asserted.** A pre-reduced, verified-only declaration costs **3.10 ms (raw form) to 2.80 ms
(face form) per composition**; the same declaration, with the type checker forced to run the tower's own
`Reduce` machinery on an unreduced pair, costs **13.80 ms, roughly 4.5x more**. This is a different operation
from section 1.24's composition-cost table and should not be read against it directly; it prices declaration of
one literal's own `Bias`, never composed with anything. The eighth consolidation derived a methodological
control from this measurement and closed by citing a measurement whose figures it no longer contained
(`109:404-405`).

**The pricing hazard's control is derivable rather than a trick to remember** (`68:412-417`, also stated in the
conventions section above): a declaration's cost is a fact about the obligations it forces, and an unused alias
forces none. **A measurement of a declaration's price states which bounds it forces, and two arms are
comparable only when they force the same ones.**

**`Adjustment` needs its own entry point, and the reason is keying rather than duplication** (`68:390-400`).
The duplication half is easy: parse, digit extraction, decimal-point folding, gcd reduction and bit
decomposition are the identical arithmetic on the identical digits for both roles, one generator serves both.
The harder half, under the layer-keying rule, is whether the **role** (scale versus offset) belongs in the
type, **and it does**, because the two roles enter the value map differently: the exponent-shift symmetry is
conditional on no `Numeral` member contributing a nonzero additive constant to the value, and an adjustment
scales while a bias offsets. Priced: with one shared face type and the role carried only by argument position,
`value::<E, X, Y>` and `value::<E, Y, X>` both compile, both run, and **silently denote 11 and 84.33
respectively**. Two doors over one shared generator (`raw_bias!`, `adjustment!`) refuse the swap, `E0277` on
both routes. **Closed: two entry points, one generator.** The `Adjustment` entry-point closure and the compiled
silently-wrong-value defect that forced it were dropped at `78:336-339` (`109:468-469`).

**Cross-call-site face identity should not be established, and the residual was posed about the wrong layer**
(`68:402-410`). Nothing that affects compilation is keyed on the face at all (a face cannot reach a numeral
position; the `NumeralFace::Encoding` projection is the only route and it erases the declaration site), so two
faces for one literal are interchangeable everywhere the type checker looks. Where the difference is
observable, **per-site is the correct answer**: a consumer's error at one declaration site should name that
site, and unifying two faces would make one site's diagnostic name another site's declaration, which is
strictly worse and is exactly the false-statement failure the layer-keying rule forbids. **Closed without a
mechanism**: face identity is per declaration site, deliberately.

**The decoder ring is a confirmed ceiling, not an open item to keep chasing** (`58:658-673`, `63:427-435`,
`68:419-427`). A raw type-equality mismatch (`E0308`) always prints the fully-expanded alias, for a
hand-written alias or a macro-emitted one alike. **The one lever that moves it is not a diagnostic attribute;
it is restating the comparison as a bound (`E0277`) rather than an equality**, which combined with a concrete
face produces the strongest diagnostic message this whole review has found:

```
error[E0277]: expected accumulator width `Q37`, this one is `Q53`
help: the trait `SameFaceAs<Q37>` is not implemented for `Q53`
```

**`SameFaceAs` is the probe's own spelling inside a quoted diagnostic, not a member of the design's
vocabulary**: the design content is the lever, restating a comparison as a bound rather than as an equality,
and the trait a real implementation reaches for at that position is `NumeralFace`'s own coarsening bound. The
clarification is here because a cold read met the name only in this diagnostic and had no way to tell which it
was (`115:122-124`).

That lever, restated at `58:785-787` as a general result (wherever a numeral mismatch can be expressed as a
bound rather than an equality the error is readable for free, independently found by three files), was dropped
at `63:427-435` (`109:376-377`). It is the strongest diagnostic result in the review.

**The general statement corrects the specific one** (`68:419-427`): an operation generic over the raw encoding
decays the face one hop in, and a message goes dead when a refusal fires on a bound over a projected associated
type, where rustc reports the innermost unsatisfied bound and an outer trait's `on_unimplemented` is never
rendered. Both are the layer-keying rule made visible rather than problems in the notation: an operation keyed
on the encoding names the encoding when it fails, because the encodings are what differ, and naming the face
there would name something that did not fail. **The spec sentence to retire is "the error names your numeral";
the sentence that is true and checkable is "an error names the layer the failing operation is keyed on."** That
instruction to the spec was dropped at `78:336-339`, which says the residuals stay closed and does not carry an
instruction that was never a residual (`109:423-424`); it stands as spec text.

**`#[diagnostic::on_unimplemented]` does not reach a solver-overflow diagnostic (`E0275`) at all**, confirmed
by direct annotation of the real `Reduce` trait, byte-identical before and after, on both an abstract operand
and a concrete rigid non-inhabitant (`58:675-680`). What answers the residual is the design's own
already-ratified architectural avoidance rather than a diagnostic fix.

**A macro is a binding-time decision, not a convenience** (`61:596-600`, quoted at `104:564-572`): "the macro
is not a convenience wrapped around the design; it is the design's own binding-time decision, made explicit and
paid for exactly once, at exactly the stage that has the information cheapest." Section 1.29 uses this sentence
to make the design's two macros one kind of object, and section 1.23 uses it to decide where the precision
bridge's table is emitted.

### 1.19 Claim provenance

The grounding registry, the transfer-ground vocabulary, the `tree-fact` and `tree-meaning` split with its
prohibition, the two added provenance classes, the three upkeep disciplines, and the archive's own conventions
about universal claims are all stated in full in this document's registry section above, before section 1,
rather than here, because two consolidations used them in eleven section footers and defined them nowhere and
the fix is to state them where a reader reaches them first.

**Two additions from the elementary-functions stretch.** `exhaustively-computed-or-cited` (section 1.16). And
the freshly-performed-search requirement, which governs the review's own audit deliverables the way the
definitional-completeness line and the separation requirement govern its design claims.

### 1.20 The algorithm crates

From `58:731-794` and `63:476-540` (`109:195`).

**File 04's `Precise`-exile question, carried at every consolidation as "still unanswered", is answered, and
the answer inverts the question's own framing. The exile was never the problem. The admission is.**
`upward_rank` returns `C::Array<W>`, the operand numeral, claiming an exactness it does not have. On a
four-node chain the true answer is `[400, 300, 200, 100]`; **there is no correct expected value expressible
against `UFixed<8, 0, Hot>`** (`E0080`, the literal does not fit). Widened to a type that can hold the
comparison, **`Hot` returns `[144, 44, 200, 100]` and, on two independent chains totalling 400 and 210, ranks
the longer path at 144 and the shorter at 210: the ordering inverts, silently**, on a graph hilavitkutin reads
for plan-stage DAG analysis. **`Precise` returns `[255, 255, 200, 100]`, wrong in value but never inverted,
only degraded to a tie.** File 04's three options (accept the exile, panic, bifurcate the crates) all took for
granted that the crates were correct for the presets they admit today. They are not.

**The design's own answer, already used everywhere else, had simply never been applied to a fold-shaped
algorithm.** `mul_full`, the MAC accumulator, and `div_exact` all compute a result numeral rather than reusing
the operand's. **A fold-shaped algorithm's result numeral is `foldnum(W, A)`, carrying `W`'s precision plus
`ceil(log2 A)`**; a numeral claiming the operand's own exactness for a computed sum is claiming something it
does not have. Compiled at three widths, with a negative control confirming the projected return type is
checked rather than inferred. `foldnum`'s characterisation is spec text as of `67b`: **sufficient always, tight
exactly for power-of-two arities and wide precisions, loose by at most one bit elsewhere**, with one compile
against the real `Exponent`-fixed contract still owed. That characterisation was dropped at `78:944`, leaving
the owed compile with no stated expected result anywhere in the document (`109:476-478`); it stands so the
compile has something to be checked against.

**The arity is the container's capacity, already in the signature, and `Capacity` owes it a `Pos` face
alongside its array-length const.** No simple path in a DAG on `C` nodes visits more than `C` of them, so
`foldnum`'s arity is bounded by exactly what every one of these functions already carries.

**The demand-driven clause, and it has now been paid for twice** (`55:163-165`, absorbed once at `58:754-761`,
dropped at `63:533-539`, rediscovered from the other side forty-five files later at `102:820-830`,
`109:44-55`):

> It fires at **use**, not at declaration, because an associated const nothing touches is not evaluated. A
> `Capacity` impl whose two spellings disagree survives until someone folds with it.

The mitigation stated with it: a `Pos` face held in agreement with the `usize` array-length spelling by **a
forced const assertion firing at use (`Capacity::filled` / `Capacity::from_fn`, every entry point) rather than
at declaration**. `Pos` has no zero, a real narrowing of `Capacity`'s domain, flagged and not resolved. Section
1.27 carries what became of this.

**Only three of the four named crates are numeric consumers at all, and one of the three carries the same
defect a second time.** `arvo-sparse` has no numeric contract anywhere in it (`W: Copy` and
`W: Copy + Default` are its only bounds; `rcm_reorder`, `block_diagonal`, `dulmage_mendelsohn` are structural,
computing on adjacency bit patterns, never on a stored value), which is why the droplist's `AddAssoc` entry
names three crates and not four, a deliberate distinction rather than an omission. **`arvo-comb`'s `bin_pack`
carries `upward_rank`'s identical defect**: wrapping over-fills a bin (too much work in one fiber grouping),
saturating under-fills (closes early), neither announced.

**`Monotone<Add>`, not `AddAssoc`, is the atom the ordering-returning algorithms actually need**, connecting
file 33's derivation to the droplist entry it was reaching past. Compiled: **wrapping addition is not monotone
(`200 + 200 = 144 < 200`), which is precisely why `Hot`'s ordering inverted; saturating addition is monotone,
which is why `Precise` degraded but never inverted.** **Two named entry points rather than one gate**, the
fold-beside-`fold_sequential` idiom applied a third time: the value-returning door (the widened `foldnum`
result) needs no monotonicity at all; the ordering-returning door (returning in the operand numeral) needs
`Monotone` to keep its ordering honest. That two-door design was dropped whole at `63:476-539`, which rewrites
the section and carries no occurrence of `Monotone` in it (`109:343-346`); it is the section's own mechanism.

**The idempotent-semiring rung stays empty until the float model's `Specials` lands as a real numeral, at which
point `longest_path`'s ground-roots-at-own-weight workaround and `matrix_chain_dp`'s whole parallel `Bool`
reachability matrix, both hand-rolled substitutes for a missing annihilator, are deleted rather than extended:
a rare case in this review of the design removing a mechanism instead of adding one.** That planned deletion
was dropped at the same rewrite; `annihilator`, `semiring`, `longest_path` and `matrix_chain` all returned zero
hits in `63` (`109:346-348`). It is stated because it is a scheduled removal that would otherwise never be
scheduled.

**The `TotalOrd` fork is a fact about each numeral's own injectivity, compiled per numeral, not declared once
for the whole design** (`63:478-483`). `arvo-graph` and `arvo-comb`'s shipped weight types (`UFixed`/`IFixed`)
clear it for free: injective by construction, no signed zero to repurpose, no NaN, no unnormalised-significand
cohort. **`arvo-spectral`, over floats, does not.** `arvo-sparse` stays excluded, carrying no numeric contract
at all.

**The shipped `TotalOrd` is IEEE's `totalOrder` under the wrong name, and it is a compiled fact rather than a
reading.** `total_cmp_f32`/`total_cmp_f64` reinterpret the bit pattern as a signed integer and compare;
**`-0.0` and `0.0`, the crossing contract's own textbook cohort, compare `Less`, not `Equal`, under this
order.** A `const` assertion stating "two data that denote the same value compare `Equal`" **refuses to compile
against the shipped order (`E0080`, a genuine build-time proof)** and compiles clean against a
canonicalise-then-compare reading built at the same cost. Per this review's standing reading of what a refused
`const` establishes, **this outranks a failing runtime assertion: there is no expected value to write down,
because the definition itself places the cohort apart.**

**The trait split is adopted as the settled shape.** Rename the shipped mechanism `totalOrder`,
non-law-usable, kept for consumers that want IEEE bit-order on purpose. Give `TotalOrd` the
canonicalise-then-compare body, so the name means what the fourth consolidation already said it should mean,
confirmed at a second independent compiled arrival, meeting the two-expert threshold on the direction.
`arvo-spectral`'s bound moves to the corrected trait. **Open, correctly**: where the canonical NaN class sits in
the value-level order (a real choice; one working placement puts it above every finite and infinite value,
matching IEEE's own convention), and **whether the split should be two traits or one trait with two methods**,
which is on the open list in section 5 rather than only in this prose.

**The consumer-pressure framing, corrected** (`63:513-531`). `hilavitkutin/BACKLOG.md.tmpl` and
`DESIGN.md.tmpl` both name `upward_rank` as the intended consumer of a weighted fold;
`hilavitkutin/src/plan/steps.rs:828-880` **ships a hand-rolled, unweighted hop count instead**, storing the
result as a bare `USize`, and a grep confirms `arvo_graph::upward_rank` and `arvo_comb::bin_pack`/`dp` are
imported nowhere in the engine's source. `arvo-spectral`'s consumption reads a sign and discards the magnitude
in the same statement. So the specific worry (a consumer with many nodes paying the widened numeral forever)
does not currently hold for the named consumer, because that consumer touches neither path. **This does not
weaken either live defect**; it removes the urgency framing attached to them. Whether hilavitkutin's own
hand-rolled reimplementation is the right call for its own design, or drift against using the stack it depends
on, is hilavitkutin's canon's question, not this review's.

**What is not settled.** Whether the widened result numeral is the right default for a consumer with many nodes
and narrow weights, who pays the widening forever in storage; a proposal, not a ruling, informed but not
settled by hilavitkutin's non-use, since a different consumer (vehje's pass-DAG use) could still hit it.
`Capacity`'s `Pos`-has-no-zero narrowing, flagged, not resolved (section 1.27 supersedes the framing).

### 1.21 The strategy door, both ratified preset tables, and the key both contracts take

**Both tables are reproduced here in full.** They exist as markdown in exactly one consolidation, the eighth
(`78:407-455`); the ninth carries a sentence pointing at the eighth; the tenth carries a sentence pointing at
the ninth **and states its own hop wrongly**, saying the tables stand as the ninth carries them when the ninth
carries the sentence and not the tables (`102:555`, `109:77-85`). `TowardNegative`, `ToEven`, `HostFloat` and
"in-range direction" each occur four, two, four and three times in `78` and **zero times in both `91` and
`102`**. That is the single clearest proof the standing-base claim had become decorative, and it is on ratified
material.

**The intent, quoted once rather than paraphrased at each row** (`78:398-405`). Op: "`Hot` is as fast as
possible, `Cold` stores as small as possible, `Precise` is the most precise at the price of both storage and
compute, `Warm` is the compromise that suits most default cases and behaves intuitively" (`talk:1659-1661`, an
op decision carrying no D-number and therefore cited by position). On `Warm` (`68b:62-67`): "I think we should
assume that it'll work the same as writing regular old floats would work... The intuition is that it works and
behaves as f32 and f64 etc in rust today without any framework on top of it." On `Cold` (`68b:69-73`), quoted
whole rather than elided: "It should be something between warm and precise. **Cold also tells us it's seldom
computed or used, it's on a cold path.** It can take more cost than warm, but shouldn't just be precise in
disguise."

**So `Cold` carries two meanings, not one: cold storage and cold path**, and `68b:76-78` draws the consequence
in the checkpoint's own voice: **seldom computed, so it may pay more than `Warm`**, and it must remain
distinguishable from `Precise` on at least two cells per D71 (from `202607301100`)'s own construction. **The
cold-path meaning is what licenses `Cold` paying more compute than `Warm`**, and it is the reason
`108b:126-128` names concurrent multi-column bandwidth contention as the measurement `Cold`'s intent is
actually waiting on. The middle sentence had been elided from the quotation inside the very passage the
standing base flagged as its flagship restoration of ratified material, which left the bench target standing
and its reason gone (`112:324-346`).

**The old table is void, in full**, per the `tree-meaning` correction. Nothing below carries forward any cell,
quote, or "shipped meaning" framing from files 59, `62b`, `63`, or `68`. Every row is re-derived from op's own
stated intent under the deletion test.

**Fixed-point, ratified in full at `70b`.**

| | `Hot` | `Cold` | `Warm` | `Precise` |
|---|---|---|---|---|
| in-range direction | `TowardNegative` | `ToEven` | `ToEven` | `ToEven` |
| `OverRange`/`UnderRange` | `ReduceModulo`/`ReduceModulo` | clamp (`TowardNegative`/`TowardPositive`) | clamp | `Refuse`/`Refuse` |
| `StoredWidth` | minimum | minimum | doubled | doubled |
| `Layout` | dense | bitpacked | dense | dense |
| `Door` | inert | inert | inert | inert |

`Hot` reproduces D71 (from `202607301100`)'s own row, now grounded entirely in "as fast as possible": an
arithmetic right shift rounds toward negative infinity for free, and reduce-modulo is native two's-complement
wraparound, so `Hot` pays for nothing beyond what the hardware already does. `Precise` refuses rather than
silently discards, because a hardware instruction is unconditional and infallible by construction and
`Precise`'s identity requires a refusing branch; doubled storage lets a chain of operations retain more than
one operation's exactness before a narrow forces a decision. `Warm` and `Cold` both round nearest and clamp for
the identical reason (a type nobody expects to crash has no reason to accept truncation bias), differing only
on the remaining two rows: `Warm`'s doubled, dense shape matches a naive hand-rolled fixed-point type; `Cold`'s
minimum, bitpacked shape is "stores as small as possible" **on the storage half of its intent, while the
cold-path half is what pays for the rounding**: seldom computed, so it may pay more than `Warm` (`68b:71-78`).

**The nearest-rounding ground is a re-derivation and is marked as one.** Op's own ground at `talk:1674-1678`
was that `Cold` "is already paying a widen and a narrow per operation, so the compare and increment that
nearest-even costs is small against what it has already spent". **That ground depends on the `Widening` axis,
which was ratified out at `39b`**, so the substituted ground above (a type nobody expects to crash has no
reason to accept truncation bias) is a correct re-derivation under section 0.3 rather than a restatement, and
it is stated as such rather than presented as op's own reasoning (`113:376-381`).

**Op stated three consequences of D71 (from `202607301100`) and the standing base carried one.** From
`talk:1702-1715`:

- **`Precise` is fallible.** Its arithmetic returns through the refusing branch of the quantisation's
  fallibility projection, so call sites unwrap. Carried above and in Thread B.
- **Only `Hot` folds for signed values.** Clamping and refusing are both unfaithful, so `AddAssoc` holds for
  `Hot` alone among the four once the operands are signed. **The marker chosen for speed is the only one whose
  signed folds the type system permits**, which reads backwards until you remember that wrapping is exactly the
  arithmetic of `ℤ/2ⁿℤ` and the others are deviations from a group. This is the same fact section 1.20's
  droplist entry reaches from the algorithm-crate side, and it is op's own statement of it, which the design
  had otherwise been reaching independently without crediting (`113:365-374`).
- **`Cold` now pays a compare and select on every store.** A real cost in the marker built for density, taken
  because **a bitpacked column that wraps silently corrupts a stored value rather than an intermediate, and a
  corrupted store is worse than a slower one.**

**`Door` is inert for fixed-point, and the reason is structural rather than asserted**: a native integer add
and `Hot`'s own software composition (`mul_full` then `ReduceModulo`) compile to the same instruction, measured
zero-cost at native and multi-limb width, **because an integer ALU has no rounding-mode control state to
distinguish**; every preset's effective door is the software composition, folded to the native instruction
wherever that instruction computes the same thing.

**Float, newly derived, ratified in full at `70b`.**

| | `Hot` | `Cold` | `Warm` | `Precise` |
|---|---|---|---|---|
| in-range direction | `ToEven` | `ToEven` | `ToEven` | `ToEven` |
| `OverRange`/`UnderRange` | far point | far point | far point | `Refuse`/`Refuse` |
| `StoredWidth` | minimum | minimum | **minimum** | doubled |
| `Layout` | dense | bitpacked | dense | dense |
| `Door` | `HostFloat<E>` | `Quantised` | `HostFloat<E>` | `Quantised` |

`Hot`'s in-range direction cannot be the fixed-point row's `TowardNegative`: no general-purpose FPU implements
that as its default, and the one rounding attribute every FPU implements for free is round-to-nearest,
ties-to-even. Every preset's `OverRange`/`UnderRange` reads "far point", per section 1.16's rule, rather than
the open cell file 70 originally left for `Warm` and `Cold`. `Precise`'s refusal needs no `Specials`
well-formedness condition at all; it is the one preset whose out-of-range row never has to ask what lies past
the edge.

**`Warm`'s `StoredWidth` diverges from its own fixed-point row, and this is the sharpest single finding the
re-derivation produced.** IEEE 754 requires a correctly-rounded result computed as if with unbounded
intermediate precision, delivered for free by the hardware, invisibly; doubling `Warm`'s float storage would
add bookkeeping the hardware never asks for and the "no framework on top of it" intuition explicitly forbids.
So `Warm`'s float row matches `Hot`'s on both `StoredWidth` and `Layout`, and diverges from `Warm`'s own
fixed-point row on both, **because the two number kinds needed the doubling for the same underlying reason
(correctly-rounded intermediates) and only one of them lacks hardware that gives it away for free.**

#### The preset key: both contracts are keyed on the exponent form

**One preset name denotes two rows, one per number kind, and that is the one gap the two tables create between
them.** Section 1.26 states it as a boundary sentence in its own words and the two tables exhibit it at four
cells. **A nullary associated type on a marker cannot carry it.** `Policy::Quantisation` and
`Lowering::StoredWidth` as nullary associated types make `Warm::StoredWidth` one type, and the ratified tables
say it is `doubled` for fixed-point and `minimum` for float. Under the layer-keying rule that is not a small
inconsistency, it is the rule's **dual failure**: a fact keyed on something that does not determine it, which
is a non-function presented as one. The row is a function of the pair, the preset and the number kind.

**The defect is a diagnostic rather than an argument.** Stating both ratified rows against nullary
declarations is refused outright, which no pass had shown before file 122 compiled it:

```
error[E0119]: conflicting implementations of trait `Lowering` for type `Warm`
  --> pA_current_defect.rs:34:1
   |
27 | impl const Lowering for Warm {
   | ---------------------------- first implementation here
...
34 | impl const Lowering for Warm {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Warm`
```

**Two spellings were offered against it and neither survives, both refuted by compiled diagnostics rather than
by preference.**

**Keying the contracts on the numeral over-keys, exit 0.** `impl const Lowering<U13F3> for Warm` with
`StoredWidth = DoubleLogical` and `impl const Lowering<U14F2> for Warm` with `StoredWidth = Minimum` compile
together: two `Implicit` numerals, one number kind, disagreeing on what `Warm` means, with nothing in the
language relating them. That is the layer-keying rule failing in its original direction, keyed too finely,
which the rule calls a false statement. Trading one direction of the rule's failure for the other is not a
repair.

**Per-kind markers admit a well-typed nonsense, exit 0.** `Number<Binary32, WarmFixed>`, a `Ranged` numeral
wearing the fixed-point preset's storage row, type-checks at a value position. (A type alias alone establishes
nothing here, since Rust does not check a type alias's bounds; the probe forces it at a function signature.)
That is fatal under a ratified decision rather than under taste: **D52 (from `202607300400`) makes compositions
public and bindable by anyone**, so the aliases hide the mispairing only for the four pairs the aliases cover.
Its second cost is also op's own text: **D72 (from `202607301100`) gives `arvo-strategy` "`Hot`, `Cold`,
`Warm`, `Precise`, and nothing else", and this spelling deletes all four of those types** in favour of eight,
so a ratified row names four things that would no longer exist.

**The third spelling, reached independently by two members, is to key both contracts on the exponent form.**
Files 119 and 122 each built it against the ratified tables and each compiled it before opening the other,
which is the two independent agreements the panel requires before a reading enters the text. The kind **is**
the exponent form, since `Numeral::Exponent` is bounded by `ExponentForm` and `ExponentForm` is sealed with
exactly the two constructors `Implicit` and `Ranged`, against exactly two tables. **So the exponent form is not
merely a key that works, it is the partition the two tables induce.**

**On whether the layer-keying rule forces it, stated carefully because one of the two reads it as forcing
directly.** The rule's primary clause is stated over layers of the identity tower, and all four of its own
instances are such layers, so the exponent form is not among the things that clause quantifies over. What fires
here is the rule's **dual**, which carries a determination requirement and no coarseness clause. **Taken whole
the rule still decides it, because both of its failure modes are live at once**: keying on the preset alone
does not determine (the dual failure), keying on the numeral is too fine (the primary failure), and the
exponent form is the unique key avoiding both. Read as one clause the rule admits more than one answer; read as
the rule with the two failure modes this document states, it admits one. That is the narrower of the two
readings and it reaches the same place.

**Compiled, at the shipped `pub const trait` spelling.** Both ratified tables transcribe cell for cell, the two
rows are recoverable and distinct, and a second numeral of the same kind reads the same row by construction
because there is no per-numeral impl to disagree at. **The wrong pairing has no spelling**, since the kind is
projected out of the numeral rather than supplied beside it, and a marker carrying only the fixed-point form
refuses against a float numeral with rustc naming the mismatch:

```
error[E0277]: the trait bound `FixedOnly: Policy<base::Ranged<...>>` is not satisfied
help: the trait `Policy<base::Ranged<...>>` is not implemented for `FixedOnly`
      but trait `Policy<base::Implicit<_, _, _>>` is implemented for it
    = help: for that trait implementation, expected `base::Implicit<_, _, _>`, found `base::Ranged<...>`
note: required by a bound in `Number`
```

**And over-keying stops being a review obligation.** Giving one numeral its own row, attempted in the declaring
crate so the orphan rule does not mask the result, is refused by coherence:

```
error[E0119]: conflicting implementations of trait `Lowering<base::Implicit<base::ENeg<base::I<base::H>>,
base::BZero, base::BZero>>` for type `Warm`
35 | impl<E: Exponent, A: Adjustment, B: Bias> const Lowering<Implicit<E, A, B>> for Warm {
   | ------------------------------------------------------------------------------------ first implementation here
92 | impl const Lowering<Implicit<ENeg<I<H>>, BZero, BZero>> for Warm {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Warm`
```

**The only route past that is specialisation, which is forbidden.** So the rule stops needing anyone to
remember it, which is the difference between a convention and a mechanism.

**The bound is spelled with the numeral's own member and mints nothing.** One of the two reads writes it
`Policy<FormOf<N>>`; `FormOf` is a name that would have to be declared, which this document's own completeness
line requires of the name being defined. `N::Exponent` is the associated type already declared at section 1.23
and needs nothing:

```rust
pub struct Number<N: Numeral, S: Policy<N::Exponent> + Lowering<N::Exponent>> {
    datum:    <S as Lowering<N::Exponent>>::Container,
    _numeral: PhantomData<N>,
}
```

Exit 0, and `Crosses` keeps its supertrait edge under the parameterisation as
`Crosses<N: Numeral>: Lowering<N::Exponent>`.

**The cost, measured rather than asserted.** At a concrete consumer position nothing changes: `Number<U13F3,
Warm>` is spelled exactly as before and the four semantic aliases expand onto it, which is what section 0.5's
consumer-facing half of the bar asks for. **The cost lands on a downstream crate generic over both
parameters**, an algorithm crate, whose signature gains the projection twice:
`where S: Policy<N::Exponent> + Lowering<N::Exponent>`, compiled, exit 0. **Two members become restated per
kind**, `Layout` and `Encoding`, which is the rule biting the other way; a restated cell is a redundancy and
the current shape is a falsehood, and those are not comparable costs.

**It does not disturb the contract split.** The load-bearing refusal at section 1.25 survives the
parameterisation verbatim, re-run rather than cited: a `Policy<N::Exponent>` bound projecting `S::Layout` is
`error[E0220]: associated type Layout not found for S`, with the same help line, and the parameter hole visible
in it. Nothing here adds a supertrait, so the prohibition at section 1.25 is untouched.

**What is owed op is a confirmation rather than a choice**, with both alternatives refuted by compiled
diagnostics. **One thing this does not establish**, stated because the impl count follows from it: whether the
five rows both tables state are the only members that vary by kind. `Encoding`'s own members are on the open
list, and if one of them turns out to be kind-dependent the impl count grows without the shape moving. The
check belongs with whoever writes `FieldLayout`'s members.

#### The door, and what a target does not implement

**Refusal, not silent fallback, binds every preset, design-wide, cited by rule rather than by any marker's own
meaning.** `arvo-toolbox-not-policer.md` forbids exactly the failure mode a silent hardware-to-software
fallback would be. **A door a target's silicon does not implement refuses to build rather than degrading ten to
seventeen times slower with no diagnostic**; this binds `Warm` exactly as it already bound `Hot`.

**That figure supersedes one inside ratified text, and the supersession is stated at the point of exercise.**
`78:459-460`, inside the `70b`-ratified section, says "degrading **thirteen to seventeen** times slower".
`63:611-613` had already measured that figure down: after `mock/benches/.gitignore` was found discarding every
artifact this review's benches had produced, the software column reproduced within 1% to 11% while the hardware
column, measured near the harness's own resolution floor, moved 15% to 25% between runs, so **"13x to 17x, at
every point of the sweep" softened to "roughly an order of magnitude, ten to seventeen across two runs."**
`68:806-807` recorded the correction as already committed. The eighth consolidation then restated the corrected
figure inside newly ratified text, which is **the only case in the archive where a consolidation does not
merely drop a correction but reverts one** (`109:438-443`, `109:621-625`). The figure carried here is **ten to
seventeen across two runs**, on `108b`'s own first principle: a ratification is made under the evidence
available at the time, and the evidence moved before the text was written. Nothing built on the number changes;
the software quantiser is expensive enough, at 10x as at 17x, that the door is worth having, which is the only
load the figure was carrying (`63:613-614`).

**The hardware-reachability theorem, corrected.** File 59's original claim, "the hardware door is reachable only
in a uniformly-`Hot` expression," was true under the void table where `Hot` was the only preset carrying
`HostFloat<E>`. Under the ratified table, `Warm` carries it too. `RANK`'s own ordering
(`Precise > Cold > Warm > Hot`, `arvo-strategy/src/lib.rs:104-107`, cited as `tree-fact`, its existence and
ordering, never its meaning) survives re-derivation on independent grounds: `Precise` is still the only preset
that ever refuses, `Hot` still the only preset whose door reaches hardware unconditionally, `Cold` still ranking
above `Warm` because "can take more cost than warm" is itself the conservative-preset property the old ordering
already encoded. **A mixed expression's resolved door is hardware exactly when both operands rank at or below
`Warm`: four cells of sixteen**, `(Hot,Hot)`, `(Hot,Warm)`, `(Warm,Hot)`, `(Warm,Warm)`, not the one cell the
void table's theorem named. Its precondition is checkable rather than prose: **the hardware door exists exactly
when all three width levels coincide at the format's own width**, which every IEEE interchange row satisfies
(`91:517-520`).

**`StoredWidth`'s own reading, forced by material already ratified, three ways** (`91:502-515`). If "minimum"
meant the dispatched primitive's rounded-up width, then under `Cold` (minimum, bitpacked) every stored value
would occupy a fixed slot with the same unused bit positions repeating with the type's own stride, which is
inter-value padding in the plainest sense and directly contradicts `Layout::Bitpacked`'s ratified single meaning
of zero inter-value padding. **`StoredWidth` denotes the carrier: `minimum` means equal to the fields' extent,
`doubled` means twice the logical width; the container is never `StoredWidth` and is never declared.**

**A hardware-float lowering is not a `Lowering` under the design's own ratified definition unless the
environment is pinned** (`58:798-806`), which is a derivation rather than a new rule. `Lowering` changes no
value; flush-to-zero turns a subnormal into a zero, a different value, **measured with the FPCR read and set
through inline assembly: `1.0/3.0` under the entry mode against a non-default rounding mode differ in the low
bit, and `MIN_POSITIVE * 0.5` differs by the whole value under FZ**. A const-folded float expression and the
identical runtime expression can disagree in value, underflow behaviour, and datum, and nothing in the type
system sees any of it. That measurement is the evidence for the entire strategy-door design and it was dropped
at `63:543-549`, which asserts the property as derived (`109:369-370`).

**Apple silicon shows no subnormal cliff on the hardware side** (`58:837-841`), confirming a stated guess rather
than leaving it unchecked: the historical x86 subnormal microcode trap does not apply here, **and the usual
argument for flush-to-zero does not transfer to this target.** The software side's own per-op cost falls as the
subnormal fraction rises (19.8 down to 15.8 ns/op), reported as a measured pattern with an unverified hypothesis
attached (branch-predictor friendliness on a uniform bottom-grid path), not as a finding. This refutation was
dropped at `63:607-614` (`109:349-350`) and is the reason the design does not adopt flush-to-zero as a default
anywhere.

**Open, stated rather than resolved.** Whether `Quantisation`'s declared type is consulted at all for a numeral
whose door is `HostFloat<E>` is a mechanical question about what a preset *is*, not a preset-content question.
File 59's diagnostic string naming `Warm` as an unconditional software-quantiser alternative to `Hot`'s refusal
is stale and needs correcting when a stub exists to correct it.

**Preset divergence** (a consumer wanting a preset with one axis overridden) **has a working, probe-verified,
unstable-feature-free mechanism** (a generic parameter default projecting off the parent preset), **noted at
op's seventh checkpoint as available and explicitly not adopted: op's call is that this deserves more than the
first mechanism that works, and a later member should take it further** (`40:693-696`). "preset divergence" and
"parent preset" returned zero hits in `49` and in every consolidation after until the eleventh, and zero in
files 41 through 57 (`109:160-166`). A working mechanism with an op instruction attached simply left the record;
it is on the open list with the instruction intact.

*Grounded on: ratified (`70b` in full, `68b:56,62-73`, `77b`, `82b`), settled shapes (`11:195-196`,
`35:109-114`, `58` section 1.14, `75:106-114`, `70:150-176`, `83` section 2, `85` section 2.1), compiled
(`70_probes/` all three, `50_probes/probe_5`, file 122's seven preset-key topologies, file 119's independent
arrival), measured (`63:607-614` the corrected software-quantiser sweep), tree-fact
(`arvo-strategy/src/lib.rs:104-107`), external (IEEE 754-2019 default rounding attribute).*
### 1.22 External images: three width levels, three maps, the digest contract, the mutation perimeter, and the footprint signal

**Three width levels and exactly one width axis**, confirmed on two independent reads (`91:531-543`). A level
is a place in the map chain where bits physically exist; an axis is a declaration a consumer makes.

- **The fields extent `W_F`**: what `Encoding::Fields` occupies, derived from the `Numeral`'s parameters, keyed
  on `Encoding`, statement 0's own quantifier domain.
- **The stored width `W_S`**: the carrier, the one declared level, the `Lowering` axis, statement P's own
  domain, whose vacuity at `minimum` is the true statement that no declared padding exists there.
- **The container width `W_C`**: what the dispatch actually allocates, never declared. A type-valued projection
  of the stored width through the dispatch menu under `Layout::Dense`, and the group arithmetic under
  `Layout::Bitpacked`.

Two candidate levels dissolve rather than join the table: `LogicalWidth` survives only as the friendly name of
the fields' extent, no longer a primitive `Numeral` axis; **the container is never an axis**, since declaring
it independently of its own projection would let a declaration disagree with a derived fact, exactly the
layer-keying rule's own named failure. **The level ordering `W_F <= W_S <= W_C` is a declaration-site refusal,
compiled**, failing with `E0080` in the `ByteCap`/`ShortCap` coverage shape.

**Three maps, not two.** `embed : D -> Carrier` carries statement P and the padding law, **forced by a purity
argument**: a one-argument pure function of the datum cannot express "preserve whatever padding was already
there", because a pure function has no prior state to preserve from, compiled both halves (a zero-padding
`From<D> for Carrier` impl is pure, two calls, one datum, bit-identical carriers; a second, genuinely different
operation taking the old carrier as a second argument is what "preserve existing padding" actually requires,
and no `From` impl can carry it). `place : Carrier -> Container` carries **statement C**, discharged by the
tower itself, an obligation only at the constructor that accepts foreign bytes, by the identical purity
argument. `materialise : Container -> Bytes` has the container, not the carrier, as its domain, which makes its
"pure relabelling" claim unconditionally true at both layouts rather than quietly assuming the stored width is
a multiple of eight.

**`Layout::Bitpacked` has one meaning, ratified in full at `77b`: zero inter-value padding.** The
byte-aligned-slot reading is not a second instance; it is what `Layout::Dense` already does at a narrow
`StoredWidth`, and the ratified table assigns `Dense` to `Hot`, `Warm`, and `Precise`. File 32's own earlier
bitpacked measurement is retroactively relabelled: it modelled byte-aligned slots, a correct measurement of
`Layout::Dense` at a narrow width, correctly built and mislabelled.

**`Layout::Bitpacked` selects the granularity at which the container level exists, and zero inter-value padding
is a theorem, not an obligation anyone discharges.** Under `Dense` the container is per value, the dispatched
primitive. Under `Bitpacked` it is per group: **`P = 8/gcd(W_S, 8)` elements in `G = W_S * P / 8` whole
bytes**, and `G * 8 = W_S * P` follows from `G`'s own definition by algebra alone, checked at every width 1
through 57 rather than sampled, and proved algebraically. A partial tail group's bits are container padding at
column granularity, canonicalised by the packer's own pure constructor.

**The niche collision: statement 0 stands unamended.** File 84's biased-niche lowering (store a bounded
numeral's datum shifted by one in `core::num::NonZero`, spending the excluded pattern, halving a refusing
column at nine extra instructions across sixty-four elements with no branch) collided, compiled, with statement
0's hardening: the niche's excluded pattern is reachable through an ordinary, compiling, unsafe transmute (a
warn-level lint, not a refusal), and a niche-shaped domain of size `2^w - k` is never a power of two, so no
field shrink expresses it. **The proposed repair, widening the quantifier to "every inhabitant of the carrier
type", was attacked and killed**: field-shrinking closes a decode with a hard `E0004` under any code path, safe
or unsafe, because the excluded state does not exist as a value of the type at all; a validity range closes it
only under a warn-level lint the optimiser is licensed to assume and any unsafe code anywhere can violate
silently, **a difference in kind, not degree**, compiled both ways. The resolution is `NonZeroCarrier` (section
1.12), a second, closed, sealable vocabulary discharging its own trusted-base obligation once.

**The digest contract, adopted in full at `90b`, confirmed at `108b:116-120`, and it needs no new mechanism.**
A digest is a composition of the tower's own established canonicalising projections, chosen by which equality
it is paired with, **at exactly two useful stopping points**.

- **A datum-keyed digest** masks the container **to the placement map's occupancy**, undoing statement C and
  statement P in one operation because both canonicalise to the identical fixed value. Compiled at the ratified
  `Warm`/`Precise` shape (fields 13, stored 26, container 32), a digest masking only to the stored width is
  **not** immune to statement-P dirt, confirming the correction has real content once a genuine three-level
  shape exists to test it.
- **A value-keyed digest** additionally applies `Encoding::Canonical`'s own class collapse, which reads the
  datum's content rather than discarding a fixed bit range and is therefore never a masking operation, at any
  construction discipline, compiled.
- **A third stopping point paired with raw carrier identity is not a digest** in this chapter's sense, since
  carrier identity has no equality this design should ever key facts on; what remains there is the byte image
  itself, already fully specified by statements 0, P, and C, with a hash applied for a consumer's own
  diagnostic convenience outside the digest law's scope.

**The one-word correction to ratified text, adopted, and it is a widening rather than a replacement.**
`91:628-631` says a datum-keyed digest "masks the container straight to **the fields' own width**". That is a
prefix mask, `(1 << W_F) - 1`, and **it is correct for a numeral and wrong for a placement map with an interior
hole**. A placement map need not be contiguous: reserved bits in a foreign register, an ignored lane, a field
removed from a declaration without renumbering its neighbours all leave a region strictly inside `[0, W_F)`
belonging to no field, **and no ratified statement covered it**, since statement P's domain was `[W_F, W_S)`
and statement C's is `[W_S, W_C)` and both are suffixes. Exhibited exhaustively: `Reg: 16` with `enable` at bit
0 and `divisor` at bits 5 through 13, interior hole at bits 1 through 4; over all **65,536** container values
against three perturbations each, the prefix mask `0b0011111111111111` **separates 65,536 pairs that agree at
every declared field** while the occupancy mask `0b0011111111100001` separates zero and conflates zero (`104`
section 5). **The repair is one word: a datum-keyed digest masks the container to the placement map's
occupancy**, of which "the fields' own width" is the contiguous special case, so the numeral's own statement is
unchanged and the bitfield's is an instance of it rather than an exception to it.

**Second-read confirmed as a widening** (`105` section 5): for an ordinary numeral the occupancy and the extent
are **identical sets, by the numeral's own ratified structure**, since `Encoding::Fields` occupies a contiguous
low run and the three-level model derives `W_F` as a single width with statement P's domain starting
immediately at `[W_F, W_S)`, structurally leaving no room for a gap inside the fields region. The prefix mask
and the occupancy mask compute the same bit pattern for every numeral that exists today, **so the fix widens
the statement's domain without touching its value on the domain it already covered.**

**The same word is owed to statement P, and it is one correction rather than two.** Making P's own region "the
complement of the occupancy" rather than "everything past `W_F`" is the same generalisation stated once,
because the digest's masking region and statement P's canonicalised region are, for a numeral, the same
complement of the same set. Both statements answer "which bits does an observation of a datum-level fact
legitimately discard", and a placement map's interior hole is exactly a region neither had a name for because
neither had ever needed one. Section 1.4 states statement P in the widened form.

**Cost, measured**: `and w0, w0, #0x3fff` against `mov w8, #16353 ; and w0, w0, w8`. **One extra instruction,
and only because the occupancy word is not an ARM logical immediate; where the occupancy is encodable the
general form is free**, and the mask is an associated const of the type either way, per the pricing pillar's
binding-time clause.

**The mutation gap reaches only the free shortcut, and the gap is smaller under `Bitpacked` than under
`Dense`.** A datum-keyed column digest of honestly-constructed data is a theorem, not an obligation, for both
layouts: hash the raw contiguous byte buffer directly, at zero per-element cost, because every padding bit at
every level is canonical by construction along any safe path, compiled. Under `Bitpacked` the dirt surface this
theorem protects is strictly smaller (one tail-group region for the whole column, not one region per element),
because interior groups carry no padding at all under the ratified single meaning. **This is exactly where the
mutation gap has real teeth, and only there**: a column never exposed through a raw accessor below the fields'
own width gets the shortcut as a structural theorem; one that has been, or might have been, so exposed inherits
the raw door's own trusted-base postcondition. **A value-keyed column digest never gets this shortcut, at any
construction discipline**, and pays a genuine per-element canonicalisation pass; under `Bitpacked` that pass
costs the same order as the ratified decode multiple.

**The choice between the two digest kinds is a real cost fork, not a style preference, and the design exposes
it as a named choice per `arvo-toolbox-not-policer.md`, never picking one silently** (`91:653-658`). Dropped at
`102:559-560`, which says the digest contract stands "exactly as the ninth consolidation states them", while
`value-keyed` occurs eight times in `91` and zero in `102` and `arvo-toolbox-not-policer.md` three times and
zero, in a document that at `102:587` invokes "the workspace toolbox rule's own authority" without naming the
rule (`109:507-513`). The obligation stands: **the design exposes the fork as a named choice.**

**Freshness of a memoized digest across an ordinary safe mutation is explicitly out of scope**: arvo ships the
pure digest function, not a cache, and invalidation is a consumer policy question with no more design content
than memoizing any other derived fact (`91:659-661`, dropped at `102`, `109:520`).

**A datum-keyed column digest is a straight raw-byte hash under both layouts with a smaller dirt surface under
bitpacking** (`108b:118-120`), which op names as the chapter's one free result.

**Bitpacking's cost against `Layout::Dense`: 1.50x on a sum, 1.29x under per-element work, ratified at `82b`.**
The 4.6x-to-5.5x sequential multiple ratified at the eighth consolidation **was a property of the decoder
computing at runtime what the type already knew, not of the layout**: the period, group stride, window offsets,
per-lane shifts, mask, load width, and read headroom are all functions of the stored width alone and belong as
associated consts, per the pricing pillar. Corrected and re-measured on a separately written harness,
cross-checked against three prior errors (a cache figure wrong by a factor of four; an instruction-count table
describing a standalone probe rather than the benched program and wrong by three to five times in the same
direction so the wrong ratio matched the true one; and a decoder silently wrong above 25 bits, now pinned by an
assertion): **with the decode plan on the type, a plain column sum costs 1.50x dense and a sum of a per-element
function costs 1.29x dense with a decode that gathers into natural-width lanes.**

**Which decode is optimal is a joint property of the layout and the consumer's own operation, so the design
ships both and picks on the operation's own lane width** (`91:674-676`, dropped at `102`, `109:515`). That is a
design commitment rather than a measurement.

**The write granule** (`91:676-678`, dropped at `102` while `102:837` builds on it, `109:516-518`): **the
period is the column's write granule. Adjacent values share bytes, so no element is independently writable, and
a consumer partitioning the column for parallel writes must place every boundary on a multiple of `P`.**
Section 1.28 generalises this one dimension up ("an outer-axis partition is legal only when `inner mod P == 0`")
and the statement it generalises is here.

**Arvo's own byte-image guarantee is a same-process, same-build-target fact, not a wire format**
(`91:684-687`, dropped at `102`, `wire format` returning zero hits there, `109:513-515`), by the identical logic
that already scoped `Warm`'s hardware door. A plain `f32` gives no cross-target byte-order guarantee either;
the native representation is a target fact, decided once per compile. **Cross-target portability is a
downstream-contract item**: a transport or persistence layer needs the format's identity (radix, precision,
exponent form and bounds, domain, `Specials`, `Underflow`, `StoredWidth`, `Layout`) to travel with the bytes or
be agreed out of band, and every one of those is already a closed, const-derivable bundle of type parameters,
not a registry. No mechanism is proposed; the item is named as owed. **Scoping the guarantee is the whole
content of the sentence**, and a byte-image guarantee whose scope is not stated reads as a wire format.

**The pattern beneath all of this recurs four times, not once** (`78:590-600`): `V -> D` (cohorts,
`Encoding::Canonical`), `D -> Carrier` (padding, forced), the digest projection (chosen by the layer-keying
rule), and, stated as the one boundary in the chain that is **not** an instance of the pattern,
`Carrier -> Bytes` under `Layout::Dense` (a pure bijection with no fibre to collapse). **Every many-to-one layer
boundary in this tower owes exactly one canonicalising projection, established once, consumed by every
downstream consumer through that projection and no other door.**

**Parse is the quantiser, and the review had already built it without noticing** (`78:496-506`). A digit string
denotes an exact rational by positional notation alone, so parse decomposes as `quantise ∘ rational-of-digits`,
with every semantic decision the quantiser's own. Compiled over the whole in-range grid at a model instance
(radix 2, `p = 8`, `e` in `[-4, 4]`, **318,126** four-decimal-place strings): single rounding from the exact
rational equals nearest-ties-to-even on every string; **staging the identical parse through a wider
intermediate with round-to-nearest at both steps disagrees with the direct parse on 3.2% of strings**, a real
and dense defect class any naive parse-then-narrow implementation would carry; and the identical staging with
**round-to-odd at the intermediate agrees on all 318,126**, giving the sealed `ToOdd` vocabulary member its
job, licensed exactly for staged pipelines with two guard digits.

**Print is the same collapse the design was already built for** (`78:508-517`). Every correct float-printing
algorithm's expensive precondition, exact access to the value and its neighbour gaps, is handed over for free:
`decode` is total arithmetic into the rationals, and the neighbour gap is type-level arithmetic on the
numeral's own parameters. **The shortest correctly-rounded digit string that reparses to the same datum exists
for every one of 1152 model data, within a bound H that is tight at the model (93 of 1152 data need the full
bound)**, and both kernels are const-callable as written, closing a full parse-print-reparse round trip inside
a `const` item. The print buffer's length is the spine rule's tenth firing.

**Display completes the layer-keying rule's own clause for computed values**, as stated in the rule above: a
value-keyed display against a datum-keyed debug image.

**The first real footprint signal, ratified at `101b` and kept preliminary at `108b:122-128`.** The
bandwidth-contention bench the review had ranked top of its open list twice, believing it blocked on an
upstream mockspace fix, **was never blocked on what the review believed**: file 96 checked the belief before
building around it and found `mockspace_bench_core::Routine::build_input_bytes`'s own override contract,
`ByteRoutine`, already shipped the by-reference input path in the same crate every bitpack bench in this
repository already depends on. Three files and two checkpoints had carried the "needs building upstream" belief
forward without the one grep that would have dissolved it. Built on the mechanism that already existed,
cross-validated against an independently-written second decoder per layout (a gap the first sweep itself found
and closed, since the harness does not validate a single-variant section at all), the sweep separates at the
instantiation the separation requirement's own test demands: a size where the packed region fits this host's L2
and the dense region does not. **At 7,000,000 elements (packed 10.85 MiB, fits; dense 13.35 MiB, does not) the
packed and dense multiple for a plain column sum drops to 1.43x, from a peak of 1.66x at L1-resident sizes,
driven by dense's own per-element cost rising monotonically by roughly sixteen percent across the sweep while
packed's stays flat**, the correct signature for a column that has started leaving cache rather than a decoder
that has improved.

**Op's ruling, `108b:122-128`: "Confirm the digest, keep pushing the bench."** So **the footprint bench is
preliminary rather than ratified**, and the 1.43x-against-1.66x crossover **stands as a first honest signal and
not as `Cold`'s price**. **The multi-column multi-core contention regime is the measurement `Cold`'s intent is
actually waiting on, and it is the bench's next target rather than an open note.**

**The two figures are reconciled here, which no document did before the eleventh.** The `82b`-ratified figures
(1.50x on a sum, 1.29x under per-element work) were measured in a **compute-bound regime where a smaller
footprint buys nothing**; the new sweep's 1.66x L1-resident peak and 1.43x crossover measure **the same decode
multiple across a residency boundary**. They do not disagree: the ratified pair prices decode with the plan on
the type at cache-resident sizes on one harness, the sweep prices the same decode across sizes on another and
reports where residency starts paying it back. The 4.6x both replaced was wrong, for the reason stated above,
and a reader of `102` alone saw neither the ratified figures nor that the 4.6x they replaced was wrong
(`109:526-530`). `Cold`'s standing does not move either way; the workspace toolbox rule's own authority for the
marker plus one honest signal is the current state.

*Grounded on: ratified (`77b` via `78:552-556`, `78:142-144`, `78:643-648`, `68:138-139`, `90b`, `82b`,
`101b`, `108b:116-128`), settled shapes (`72`, `73` in full, `80` sections 1 and 6, `81` in full, `83` in full,
`84:354-411`, `104` section 5, `105` section 5), compiled (`72_probes/` all six, `73_probes/` all four,
`80_probes/`, `81_probes/`, `83_probes/`, `86_probes/`, `87_probes/`, `88_probes/`, `104_probes/p5`), measured
(`mock/benches/bitpack-*`, `mock/benches/bitpack-footprint-{dense,packed}_n*.csv`, forty warm samples per
variant per size, cross-validated at every reported size).*

### 1.23 The assembled trait table

**What a reader assembling the blocks in this document needs, stated at the first block that needs it rather
than in the verification section.** The eight Rust blocks in the base assemble to exit 0 only with three
additions the document never stated, and the wrong place for a reader to meet them is a verification appendix,
because a reader assembling the blocks starts here and at section 1.1 and would hit twenty-nine errors before
reaching anything the document decided (`121:126-134`):

```rust
#![feature(const_trait_impl)]      // without it, every `pub const trait` below is E0658
use core::marker::PhantomData;     // named by nine constructors here and both structs at 1.2
// plus a `notko::ConstTry` in scope, the one out-of-document name any block references
// (`Quantisation::Fallibility`, 3285)
```

The block is reproduced character for character from the standing base, so its own line citation is into that
document's numbering; the reference is the `Quantisation` declaration later in this section.
Reproduced independently on a superset of the declarations: dropping the gate gives 67 `E0658`, and dropping
the import gives 9 `E0425`. **None of the three is a design finding**, two are ordinary crate root material and
the third is a real dependency the crate table already carries. What is worth stating is that every "exit 0" in
this record rests on all three.

```rust
// Every member that denotes a number is drawn from one value-unique, sealed,
// type-level encoding, sealed and attacked on every introduction route (1.11, 1.12):
//   Nat ::= Z | Pz<P>            P: Pos       precision, widths, exponent bounds
//   Pos ::= H | O<P> | I<P>      P: Pos       magnitudes
//   Bias ::= BZero | BPos<N, D> | BNeg<N, D>  N, D: Pos, N: Gcd<D, Out = H>   signed rational
//   Exponent ::= EZero | EPos<P> | ENeg<P>    P: Pos      signed exponent, sealed (1.15)
//   Radix ::= Rad<P>             P: AtLeastTwo   sole constructor, sealed (1.2, 5 routes: 1.12)
//   Capacity: Nat                a direct instance, no second seal, no second arithmetic (1.12, 1.27)

pub const trait Numeral {                 // ratified: identity contract
    type Radix:     Radix;
    type Precision: Precision;
    type Exponent:  ExponentForm;
    type Domain:    SignDomain;
}

pub const trait Policy {
    type Quantisation: Quantisation;      // Growth removed from Policy: RATIFIED (1.10, 1.21)
}

pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;        // the carrier level; W_F <= W_S, declared (1.22)
    type Layout:      StorageLayout;      // {Dense, Bitpacked}, selects the container granularity
    type Door:        LoweringDoor;       // both presets ratified (1.21)
    type Container;                       // the container level W_C: derived, never declared
                                          // as an axis (1.22), and what Number holds (1.1)
    // Widening removed: RATIFIED.
}

pub const trait Underflow { /* Gradual | Abrupt, sealed, both change representability (1.16) */ }
pub const trait Specials  { /* the product {NoSpecials, NanOnly, InfOnly, IeeeSpecials}, sealed (1.16) */ }
pub trait NumeralFace {                   // the notation vehicle's face (1.18)
    type Encoding: Bias;                  // unsealed, per-literal, bridges to the sealed tower
    const DISPLAY: &'static str;
}
pub unsafe trait Crosses<N: Numeral>: Lowering {
    // Precondition (1.4): decode's codomain lies in V(N); statements 2 and 3 are
    // ill-typed without it, and it is placed in front of them rather than beside them.
    // Statement 0 (1.4): for every bit pattern of Encoding::Fields' width, decode is
    // total; a decode partial on that set does not satisfy this trait, and the
    // partiality is expressed by shrinking the fields (unamended, 1.22).
    // Statement P (1.4, 1.22): for every carrier this Lowering can produce, the bits
    // outside the placement map's occupancy are exactly the padding this Lowering
    // declares. (The region is the complement of the occupancy, not a suffix: 1.22.)
    // Statement C (1.22): the container's bits outside the carrier are canonical,
    // discharged once by the tower's own pure projection, an obligation only at the
    // constructor that accepts foreign bytes.
    // Every impl is `unsafe impl` and is an entry in the trusted base; the tower's own
    // generated impls satisfy all three for free, a hand-laid impl is where any
    // obligation actually bites.
}
```

**`Policy` and `Lowering` take the exponent-form key under section 1.21's settled reading**, spelled
`Policy<F: ExponentForm>` and `Lowering<F: ExponentForm>`, with `Crosses<N: Numeral>: Lowering<N::Exponent>`.
The block above is the nullary spelling the record carried before that derivation, kept because op's
confirmation of the key is owed and because the two differ by exactly the projection; nothing else in the table
moves either way.

**The vocabulary every member above is bounded by, declared rather than named.** The table declares four
contracts whose members are bounded by nineteen further traits, and not one of the nineteen existed as a
declaration in the standing base: a reader could see that `Numeral::Precision` is bounded by `Precision` and
had no way to learn what `Precision` is (`116:81-85`). **The nineteen are the block below**, in its order, so
the count is checkable against a list rather than asserted: `Pos`, `Nat`, `AtLeastTwo`, `Exponent`, `Radix`,
`Bias`, `Gcd`, `Dec`, `Precision`, `Capacity`, `Adjustment`, `ExponentForm`, `SignDomain`, `SignIndexing`,
`FieldLayout`, `Canonicalisation`, `StoredWidth`, `StorageLayout`, `LoweringDoor`. The BNF comment at the head
of this section was the same gap in the other direction: a closed and unambiguous enumeration, in a comment, of
the carrier the whole design rests on (`116:46-49`). Both are written out below, from what this document
already says about each. **Nothing here is a new decision**, and where writing one down would have required a
decision it is on the open list instead, named in section 5.

```rust
// ---- the sealed bottom carrier: the BNF above, as declarations ----------
mod sealed { pub trait Sealed {} }
use sealed::Sealed;

pub const trait Pos: Sealed { const VAL: u64; }   // readout ceiling at 10^20: 1.18
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);

pub const trait Nat: Sealed { const VAL: u64; }   // one seal, one ordering, one
pub struct Z;                                     // arithmetic, inherited whole (1.27)
pub struct Pz<P: Pos>(PhantomData<P>);

pub const trait AtLeastTwo: [const] Pos {}        // O<P> and I<P>, never H (1.2)

pub const trait Exponent: Sealed {}               // the signed exponent (1.15)
pub struct EZero;
pub struct EPos<P: Pos>(PhantomData<P>);
pub struct ENeg<P: Pos>(PhantomData<P>);

pub const trait Radix: Sealed {}
pub struct Rad<P: AtLeastTwo>(PhantomData<P>);

pub const trait Bias: Sealed {}                   // signed gcd-normalised rational, 44b
pub struct BZero;
pub struct BPos<N: Pos, D: Pos>(PhantomData<(N, D)>) where N: Gcd<D, Out = H>;
pub struct BNeg<N: Pos, D: Pos>(PhantomData<(N, D)>) where N: Gcd<D, Out = H>;

// The tower's type-level arithmetic. These two are the members whose codomain
// this document states; the rest of the family is in section 5.
pub const trait Gcd<Rhs>: Sealed { type Out: [const] Pos; }
pub const trait Dec: [const] Pos { type Out: [const] Nat; }   // the predecessor (1.27)

// Named semantic aliases over the carrier, on 74b's own pattern for Capacity:
// no second seal, no second arithmetic, no second ordering.
pub trait Precision:  Nat  {}   impl<T: Nat>  Precision  for T {}
pub trait Capacity:   Nat  {}   impl<T: Nat>  Capacity   for T {}
pub trait Adjustment: Bias {}   impl<T: Bias> Adjustment for T {}

// ---- what the three contracts' members are bounded by -------------------
pub const trait ExponentForm: Sealed {}           // Implicit | Ranged (1.2)

pub const trait SignDomain: Sealed {}             // a value fact (1.2)
pub struct NonNegative;  pub struct Symmetric;  pub struct AsymmetricLow;

pub const trait SignIndexing: Sealed {}           // a datum fact (1.3)
pub struct Unsigned;      pub struct TwosComplement;
pub struct SignMagnitude; pub struct OnesComplement;

pub const trait FieldLayout {
    type Extent: Precision;   // W_F, statement 0's own quantifier domain (1.22)
    // The hidden bit, the encoding bias and the reserved codes are this
    // contract's remaining content; their member spellings are in section 5.
}

pub const trait Canonicalisation {}   // signed zero, preferred cohort, NaN
                                      // canonicalisation; members in section 5

pub const trait StoredWidth: Sealed {}            // the carrier level, W_S (1.22)
pub struct Minimum;  pub struct DoubleLogical;

pub const trait StorageLayout: Sealed {}          // container granularity (1.22)
pub struct Dense;    pub struct Bitpacked;

pub const trait LoweringDoor: Sealed {}           // both preset tables (1.21)
pub struct Inert;    pub struct Quantised;
pub struct HostFloat<E>(PhantomData<E>);          // E is the environment (1.26)

pub struct Folded<const N: usize>;                // the site count (1.14)
```

**`Precision`, `Capacity` and `Adjustment` are named semantic aliases over the carrier rather than second
encodings**, which is `74b`'s own pattern for `Capacity` applied to the two names that share its shape, on op's
stated condition that "the mechanism unifies and the vocabulary does not". **`FieldLayout` and
`Canonicalisation` are declared with the one member this document fixes and their remaining members are on the
open list**, because the document names the facts they carry and never their spellings, **and inventing those
spellings here would put a decision nobody made into the canon's voice**. **One member is added to a ratified
trait**, `Lowering::Container`, and it is a projection rather than an axis: section 1.22 already states that
the container level is derived and never declared, and section 1.1 explains why `Number` has to be able to name
its result.

**`Folded`'s parameter is a const rather than a `Nat`, and the compiled evidence is what settles it.** A moved
count offered to it is refused with `E0435`, which is the error for a non-constant value in a constant position
and is available at no other spelling; a `Nat`-typed parameter would have refused with a name-resolution error
instead. What is not settled is whether the witness rides as a returned value or as a parameter on the fold's
result numeral, and that is one line in section 5.

**Everything above compiles.** The whole of this section's table, plus section 1.2's two structs and section
1.1's `Number`, built as one crate under the pin (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`--edition 2024 --crate-type=lib`), exit 0, one dead-code warning on `Number`'s own field, which is what a
declaration-only crate produces. **Two things had to change to get there and both are recorded where they
happened**: section 1.2's unit structs (`E0392`, seven times) and a bound spelling, since **a struct cannot
carry a `[const]` bound at all** (`error: [const] is not allowed here`, `note: structs cannot have [const]
trait bounds`), so every carrier constructor above takes its parameter's bound plain while the traits keep
theirs.

**The test that sorts an axis between `Numeral` and `Policy`, which is op's D54 (from `202607301100`) and was
invoked by name in the standing base without being stated once.** Op, `talk:352-356`, verbatim:

> The test that separates the two columns, stated so later additions sort themselves: change the axis and ask
> whether the set of representable values changed. If it did, the axis is identity. If the same values are
> still representable and only the arithmetic differs, it is policy.

Op's statement of the principle behind it, `talk:334-336`: **what the number *is* does not change through
strategies**, so the strategy marker carries only the compute policy and a separate contract carries the
identity. `spec:33-36` restates it as "the test that sorts any axis added later". **Thread B's delivery reframe
cites it by that name** ("is by this design's own axis-sorting test a `Lowering`-level choice"), and section
1.13's reading of flush-to-zero as a `Quantisation` resolution rather than a `Numeral` fact runs it without
naming it, which is what a test stated nowhere produces (`113:289-315`).

**The `Policy` axis's own vocabulary, stated because the table above names `Quantisation` and the standing base
defined it nowhere.** D64 (from `202607301100`) (`talk:1387-1393`): the policy axis is `Quantisation`, **one
axis over five situations**, because rounding and overflow are not two axes but the in-range and out-of-range
halves of one map from an exact value onto the representable set, which is what the field calls a quantiser. An
exact result sits in one of five situations relative to the representable set: strictly between two neighbours
below their midpoint, exactly on the midpoint, strictly between them above the midpoint, past the top of the
range, or past the bottom (`11:191-197`).

```rust
/// What is returned when an exact value is not representable.
pub const trait Resolution {}

/// The subset available when two neighbours exist, so the rule is a
/// choice between them.
pub const trait Direction: [const] Resolution {}

pub struct TowardNegative;  pub struct TowardPositive;
pub struct TowardZero;      pub struct AwayFromZero;
pub struct ToEven;          pub struct ToOdd;
// Direction, and therefore also Resolution

pub struct ReduceModulo;    pub struct SubstituteZero;
pub struct Refuse;
// Resolution only: meaningless where a neighbour exists

pub const trait Quantisation {
    type UnderMidpoint: Direction;
    type OnMidpoint:    Direction;
    type OverMidpoint:  Direction;
    type OverRange:     Resolution;
    type UnderRange:    Resolution;
    type Fallibility<T>: notko::ConstTry<Output = T>;
}
```

From `talk:1243-1266` and `11:198-216`. **`Direction`'s six members are IEEE's own rounding-direction
attributes spelled in full per D56 (from `202607301100`)** (`talk:1128-1140`, which is D63 (from
`202607301100`)'s own content), and the rounding rule is the triple over the three midpoint positions, so
IEEE's five modes and SystemC's seven are all rows rather than a table. **Clamping needs no name of its own**:
clamping a value above the range is `TowardNegative`, the same marker rounding already uses, which is why the
range positions take a `Resolution` rather than a fourth vocabulary. **The three `Resolution`-only members are
exactly the options a range end has that a midpoint does not**: take the one neighbour that exists (still a
`Direction`), return something unrelated to where the value was (`SubstituteZero`, `ReduceModulo`), or refuse
(`11:194-197`). **`ReduceModulo` is not a `Direction`, so it cannot be written at a midpoint position at all**,
which is the mechanism carrying the meaninglessness rather than a convention discouraging it.

**So the "four members of the `Resolution` axis" section 1.13 quantifies over** are the four things a range end
can do: **clamp** (a `Direction` at the range position), **`ReduceModulo`**, **`SubstituteZero`**, and
**`Refuse`**. The ordering "by how much they lie" that section 1.16 attaches to them is this document's own
reading and is stated in no source; it is left as a reading rather than promoted.

**`Number`'s own spelling, and the aliases** (D53 (from `202607301100`), `talk:326-332`): **there is one
numeric type, and every family arvo ships today becomes a semantic alias over a composition of it.** `UFixed`,
`IFixed`, `FastFloat` and `StrictFloat` **stop being four types and become four names for four compositions**.
The cost accepted with it is wordier rustc diagnostics, since the expanded composition is what an error prints,
which is the cost Thread A's modifier work is against. The precedent is D40 (from `202607291910`)'s `Rect` over
`Orthotope` (`spec:317-318`), and **D48 (from `202607292300`) and D31 (from `202607291720`) are satisfied
without special handling because an alias preserves a spelling exactly**: `UFixed<13, 3, Warm>`, `Uint<13>` and
`Bits<13, Hot>` each still read as themselves, width stays a const parameter publicly, and a migration changing
the spelling would charge every call site for an internal restructuring (`inherited:1894-1899`, D48, op,
2026-07-29).

**And compositions are public and bindable, so the presets are the default path rather than the only one**
(D52 (from `202607300400`), `inherited:2110-2114`, op, 2026-07-30): "Compositions are public and bindable by
anyone; semantic names and strategy presets are the default documented path, not the only path", citing
`arvo-toolbox-not-policer.md` as independently deciding it, because sealing the composition would be arvo
choosing which combinations a consumer may want. **The two ratified preset tables in section 1.21 are therefore
four documented points, not the surface.** Without that sentence a standing base carries two preset tables and
nothing saying they are one path among several, which makes Thread A's nominal-constructor work
unintelligible (`113:317-332`, `113:130-150`).

#### The bridge from a public const width to a type-level `Nat`

**What the aliases expand to had never been written down, and one step of it is not obvious.** D53 makes
`UFixed`, `IFixed`, `FastFloat` and `StrictFloat` four names for four compositions over `Number`, D48 and D31
keep the public spelling `UFixed<13, 3, Warm>` with its widths as const parameters, and the numeral inside the
composition takes `Precision` as a `Nat`. **Those two facts do not meet on their own**: `I + F` in type
position is a const expression and section 1.2 has already compiled every permitted door to it shut. The bridge
is a generated table, one impl per admitted width, plus the tower's own type-level addition, and it is
**compiled, exit 0, with no feature gate at all**, carrying a total width of 16 through a const assertion at a
concrete numeral.

**The result types take their own names, and this closes a token collision rather than making a free naming
call.** The paragraph that first stated the bridge spelled its result `Precision<13, 3>`, and `Precision` is
not a type in this design: the block above declares `pub trait Precision: Nat {}`, a nullary marker trait with
a blanket impl over the carrier. Written that way, against these declarations assembled as one crate, rustc
refuses:

```
error[E0782]: expected a type, found a trait
   --> probe_precision.rs:258:24
    |
258 | const _: () = assert!(<Precision<13, 3> as Nat>::VAL == 16);
    |                        ^^^^^^^^^^^^^^^^
    |
help: you can add the `dyn` keyword if you want a trait object
```

**Rustc's own suggestion is unavailable**, because `dyn` is forbidden across arvo, so the diagnostic has no
repair the design admits. **The collision refuses one step earlier than the use site**, which is what decides
that it must be closed rather than tolerated: declaring both under the one token, which is what a canon crate
would do, is refused at the declaration:

```
error[E0428]: the name `Precision` is defined multiple times
  --> pH_precision_collision.rs:28:1
   |
24 |   pub trait Precision: Nat {}
   |   ------------------------ previous definition of the trait `Precision` here
...
28 | / pub type Precision<const I: u16, const F: u16> =
   | |________________________________________________^ `Precision` redefined here
   |
   = note: `Precision` must be defined only once in the type namespace of this module
```

Two files reached that `E0428` independently. So this is not a bad citation at a use site, it is a pair of
declarations the type namespace forbids, and the `E0782` above is the downstream symptom.

**The branch that renames the marker trait dies on ratified text, which is why the two branches are not
symmetric.** `Precision` is one of three named semantic aliases over the sealed carrier, declared as a family
above, on **`74b`'s own pattern for `Capacity`, adopted on op's stated condition that "the mechanism unifies
and the vocabulary does not"**. The three names are that vocabulary. Renaming `Precision` also rewrites the
bound on `Numeral::Precision`, the bound on `FieldLayout::Extent`, and the family's own shape, for no gain.
**`16d:14-15`'s tiebreaker decides it**: where the current shape can be kept it should be, and rewrite cost
breaks a tie between designs otherwise equal on the intent.

**So the marker trait keeps `Precision` and the bridge's result types take names of their own**, `NatOf` at one
argument and `PrecisionOf` at two. Compiled together with the assertion and with the marker trait still
applying to the result, which is why both names have to coexist at all:

```rust
pub type NatOf<const I: u16> = <Idx<{ I }> as AdmittedWidth>::Nat;
pub type PrecisionOf<const I: u16, const F: u16> =
    <NatOf<{ I }> as NatAdd<NatOf<{ F }>>>::Out;

const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
fn takes_a_precision<T: Precision>() {}
fn both_names_live() { takes_a_precision::<PrecisionOf<13, 3>>(); }
```

Exit 0. **What remains genuinely op's is one word and not a fork**: whether `PrecisionOf` is the spelling, which
sits inside D56's ratified naming rule rather than beside it. It is on section 2's list with the `Exponent` and
`Dec` collisions, all three being the same defect, and this one is the first of the three to sit on the surface
a consumer writes.

**The braces in `NatOf<{ I }>` are required exactly when the const parameter's name also names a type in
scope, and here it does, structurally.** The public fixed-point spelling calls its integer width `I` and the
sealed carrier calls its odd constructor `I<P: Pos>`, and the table lives in the crate that declares that
carrier. In one crate holding both:

```
error[E0747]: type provided when a constant was expected
  --> pJ_collision.rs:25:37
   |
25 | pub type NatOf<const I: u16> = <Idx<I> as AdmittedWidth>::Nat;
   |                                     ^
   |
help: if this generic argument was intended as a const parameter, surround it with braces
```

Renaming the bridge's const parameters so they do not collide compiles unbraced, exit 0, which is why an
independent probe that renamed them saw nothing. **Two remedies, both compiled, and the design carries the
cause rather than the symptom**: brace the argument, or name the bridge's const parameters so they do not
collide with the carrier's constructors. Which one is a naming matter under D56 and goes with the item above.

**The table is forced rather than chosen.** An impl is the only case split Rust has over a const parameter, and
all three routes past that are compiled shut (`119:56-121`): recursion on the const needs a const expression in
type position, which is forbidden, and its base and recursive cases overlap at `E0119` even if the arithmetic
were free; a `type const` body may not compute from a generic parameter, so that opening carries a value out of
a type and never a computed value in; and separating the base case needs **full specialisation**, which
`unstable-features.md` forbids and which is one of the two bans the model-width transfer argument rests on.
**So the ban that makes a check at eight bits mean anything at sixty-four is the same ban that forces the
table**, which is one property with two consequences rather than a coincidence.

**The table lives in the crate that declares the sealed carrier, and the orphan rule settles it rather than
taste.** Both the index type and the indexing trait are the tower's, so both are foreign everywhere else and a
downstream crate adding a row is `E0117`. It is emitted complete, once, at that crate's compile, because the
only mechanism for a per-consumer variant is the downstream impl that diagnostic refuses.

**It is emitted by `macro_rules!` in that crate and not by a build script.** This is the design's own rule
deciding a question that would otherwise be argued: section 1.25 and `16b:50-53` state that **arvo grows no
build harness of its own**, and section 1.18 states the positive form, that the macro is the design's own
binding-time decision made explicit and paid once at the stage that has the information cheapest. A table of
literal impls over a fixed integer range is `macro_rules!` work at the crate root, and reaching for `build.rs`
or `OUT_DIR` would be arvo acquiring exactly the harness the design refuses.

**Each row is emitted with its own agreement assertion beside it**, in the free-const-item spelling section
1.27 already compiles, which moves the table's claim from `unargued` under the transfer scheme into the
four-bin ledger's first bin. **The per-row assertions are not redundant with the downstream one, and the case
that separates them is compiled.** A single corrupted row is caught by both, naming the wrong row:

```
error[E0080]: evaluation panicked: assertion failed: <<Idx<13> as AdmittedWidth>::Nat as Nat>::VAL == 13
```

But a row corrupted **together with a compensating change in the addition impl** passes the downstream
assertion at exit 0 and is caught only at the row. So a table emitted without per-row assertions is a thousand
`unargued` claims wearing a compiled-artifact costume, and the assertions belong in the canon's statement of
the table rather than in an implementation note.

**The downstream contract, which `16c:31-53` asks of every member.** The table is a total function from an
admitted width to a sealed numeral, resolved entirely in the trait solver before monomorphisation, so **nothing
downstream sees it**: no per-width symbol, no runtime table, no relocation, nothing a linker can observe. What
arvo needs back from any build layer or code generator is likewise nothing, which is the answer to state rather
than a gap to apologise for. The one downstream-visible consequence is a diagnostic, and section 2 carries the
width-range item that governs it.

**What op owes on the bridge** is whether the table is the design's answer, not whether one exists, and the
emission locus is settled by the orphan rule and the no-build-harness rule between them, on two independent
builds that reached the same mechanism.

**`Int` stays dropped**, confirmed at `68b`; one line to restore if op reverses, and section 1.15 states why it
survives the future case as well as the past one.

**Rewrite cost against the shipped tree remains near zero**: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth` both exit 1, empty. **What is
real and defective in the shipped tree** is `arvo-graph`, `arvo-comb`, `arvo-spectral`,
`arvo/src/traits/from_constant.rs`, and `arvo-strategy`'s container dispatch plus its facade, each carried in
section 3.
### 1.24 The cost model, printed rather than left in a CSV

From `58:890-934` (`109:198`). **Adopted as spec text.**

The cost has two terms: **a marginal cost per distinct composition, and a smaller marginal cost per repeated
site of an already-instantiated composition, both linear throughout, both measured independently twice** (an
original sweep, and a separately written generator reproducing both anchors within 0.2 ms).

| profile | marginal cost per distinct composition |
|---|---|
| dyadic (every shipped fixed-point numeral's shape) | ~2.1 to 2.3 ms |
| decimal, unit numerator (currency, sensor scale, the whole decimal fixed-point use case) | ~6.8 to 14.5 ms |
| decimal, two-digit numerator, small denominator | ~21.0 ms |
| decimal, wide numerator and denominator | ~78.9 ms |
| 16-bit random rational pair (arbitrary MATLAB-import slope/bias) | ~143 ms |
| repeated site of an already-instantiated 16-bit composition | ~28 ms |

**The realistic profiles sit two orders of magnitude below the sweep the review had been calling "realistic."**
The 16-bit random-rational sweep multiplies two large co-random magnitudes, forcing maximal Stein-gcd work;
**it is the adversarial worst case, not a representative one.** The design's own three named division constants
(44100, 48000, 4096) cost five milliseconds together. hilavitkutin's own twenty dyadic sites cost under a tenth
of a second. These are compile-once, per-declaring-crate costs, re-paid on each edit of that crate; at these
profiles the inner loop does not notice.

**The numerator, not the denominator, is the term an importer actually controls, and it dominates.** Unit
numerators over denominators to `10^9` cost 14.5 ms; **two-digit numerators over denominators only to `10^5`
cost 21.0 ms, more expensive despite the smaller denominator.** Wide magnitudes on both sides reach 79 ms, more
than half the worst named cliff. That finding and its two-digit-numerator row were dropped at `63:669-679`,
which prints four of six cost rows (`109:370-371`).

**The cliff has a name, and it is printed rather than left in a CSV. One hundred distinct arbitrary 16-bit
rational compositions cost 14.3 seconds; four hundred cost 63.7 seconds.** That figure sat in a committed CSV
for twelve files while the prose quoted only the per-unit rate; the fix is a number a reader feels, printed
here. The profile that pays is a code generator importing a MATLAB fixed-point model with many distinct
per-signal slope/bias pairs, **which is the axis's stated reason for existing.** The toolbox rule's answer is a
documented tradeoff, not a policed one: the consumer who chooses that profile does it with the number in hand.
**Whether the per-composition verification cost can be made cheaper for that specific bulk-import profile is on
the open list as an attempt, per the novelty posture, not as an accepted limitation.**

**Scope, stated honestly.** This prices **composition** (combining two already-declared numerals), the only
part of the design compiled in full at width. It is a different operation from section 1.18's **declaration**
pricing (one literal's own `Bias`, never composed) and the two figures should not be read against each other.
The grade projection, the exponent sums, and the notation macro's face check are separately priced by their own
sections as cheap at the single-composition grain (0.1 to 0.2 ms), and **none has been priced at aggregate
scale; the linearity result here predicts they are, and prediction is not measurement.** A real-consumer
compile-cost bench remains open. One figure does extend the model: section 2's width-ceiling measurement prices
the precision bridge's table at about 1.25 ms per distinct composition to 4096 rows and about 2.8 ms at 16384,
against the dyadic row's 2.1 ms, and that curve is **roughly quadratic past 4096** rather than linear, which is
the same character the per-width container dispatch already measured.

### 1.25 The downstream contract, the contract split, and the crate table

**arvo grows no build harness of its own.** A build layer reads every axis, acts freely on `Lowering`, acts on
`Policy` only inside its own declared envelope, and never acts on `Numeral`. The post-monomorphisation
verifier, the per-axis liveness check, the fold-detection assertion, the layout-assertion precedent, the
build-layer receipt requirement, and the three ways to cross Stage G stand exactly as file 26 recorded them
(`58:935-941`). **Six named standing contract mechanisms**, dropped at `63:681-689` (`109:373`), named here so
a reader can find them.

**Three receipt families sit beside them, all spec text, none built, all stated as owed to a build layer rather
than to arvo itself**: the hardware-float-lowering receipt (section 1.21: the declared control state,
invalidated process-wide by any code writing the FP control register); the reassociation-licence receipt
(section 1.8's four clauses); and the environment receipt with its split verdict (section 1.26).

**The crate split, D72 (from `202607301100`), op's own, and the single largest thing the standing base was
missing.** Op, `talk:1723-1741`, restated at `spec:289-300` with a seventh row added:

> **D72. One crate per contract, and `arvo-strategy` keeps only the presets.** Decision (op, 2026-07-30).
> Closes C1 through C4 together.

| Crate | Holds |
|---|---|
| `arvo-numeral` | `Numeral`, `ExponentForm`, `Adjustment`, `Bias`, `Underflow`, and their markers |
| `arvo-policy` | `Policy`, `Quantisation`, `Resolution`, `Direction`, `Growth`, and their markers |
| `arvo-lowering` | `Lowering`, `StoredWidth`, `Widening`, `StorageLayout`, and their markers |
| `arvo-strategy` | `Hot`, `Cold`, `Warm`, `Precise`, and nothing else |
| `arvo-numeric` | `Number<N, S>`, the semantic aliases, the `conv-*` alias sets |
| `arvo-algebra-contracts` | the ladder and the law markers |

**The table is transcribed from `spec:291-300` cell by cell**, which is the seven-row form; the talk file's own
six-row form omits `arvo-algebra-contracts` and is otherwise identical. **Two rows name axes this design has
since ratified out**: `Growth` left `Policy` entirely (section 1.10) and `Widening` left `Lowering` (section
1.10), both at `39b`, both after D72 was written, so those two cells are stale on the register's own side and
the crates are not. A reader building crates directly from the printed table would otherwise create a home for
a type the rest of this document says should not exist (`116:188-196`).

**`arvo-numeral`, `arvo-policy` and `arvo-lowering` each returned zero hits in the standing base**, searched
four ways, while its open list carried "what `arvo-numeric` ends up containing once the numeral, policy and
lowering definitions move out", **presupposing a move whose text was nowhere** (`113:94-128`). This matters
beyond bookkeeping: the taxonomy round op scheduled at `68b:14-21` builds the crate structure and will be
briefed off this document, so a crate split absent here is a crate split that round re-invents.

**What the split resolves and what it does not.** It resolves C1's question about `arvo-strategy`'s identity by
emptying it: the crate stops being where the strategy machinery lives and becomes where the four strategies
live, and it depends on `arvo-policy` and `arvo-lowering` rather than declaring what it implements, which is
the ordinary shape for a crate of instances (`talk:1735-1741`). **D23 (from `202607291200`) and D27 (from
`202607291700`) had already started that direction**: D23 (`inherited:1087-1094`, op, 2026-07-29) moves
`Identity` and `SignedIdentity` to `arvo-algebra-contracts`, on the reasoning that identity is algebraic
structure, `Monoid<Op>` is `Identity<Op> + Combine<Op>`, and keeping identity in the strategy-marker crate
splits one concept across two crates; `Additive` and `Multiplicative` move with it. **D32 (from
`202607291730`)** (`inherited:1324-1329`, op) splits the marker family on what it describes: `IntegerLike`,
`FractionLike`, `FloatLike` and `BoolLike` to `arvo-numeric-contracts` as numeric contracts, `BitPresentation`
to `arvo-bits-contracts` as the bit surface's question, **the split being on subject matter rather than on
current co-location**. **D33 (from `202607291730`)** (`inherited:1330-1340`, derived by an agent on op's
instruction and marked as such, so it sits a rung below the rest) sends each width const fn after its subject,
with `width_le_64` the one exception because it asks a question about container selection.

**And D73 (from `202607301100`) decides how that marker family is produced** (`talk:1758-1776`, op, closing C5
and C6). `IntegerLike`, `FractionLike` and `FloatLike` **become blanket impls conditioned on the exponent
form** rather than hand-written per type; the obstacle is that "fractional" means a negative exponent and an
inequality in a bound needs const-expression bounds, which are forbidden, and the cure is a macro-expanded
table declaring integrality on the `Implicit` markers so the bound becomes an associated-type equality rather
than a comparison. **`BoolLike` leaves, because it was never a member of the same family**: four of the five
markers classify numerals and the fifth classifies `Bool`, which has no numeral and is not a `Number`, so it
follows `Bool` rather than sitting among things that describe the shape of a number. D73's other half is
carried in the crate table's `arvo-container` row already, unnumbered: **the numeral's range is identity and
the carrier's range is lowering, and they differ exactly when the stored width is doubled**, so the two
questions have two places to be asked rather than one contract standing between them. **The marker half was
absent by every name** (`113:352-363`), which matters because section 1.30 works `Bool`'s placement at length
and never as this.

#### The contract split: three contracts, two parameters, unchanged

**Op reserved this call at the third checkpoint and it is now settled on two independent reads.** `08b:47-51`,
under the heading "What is not being asked":

> The next member is not asked to rule on whether the split is worth its cost, nor to choose between fused and
> split. **Those are op's**, and they are downstream of whether enforcement is possible. Report what is
> mechanically true and let the call follow.

Files 117 and 120 read it independently, the second forming and compiling its answer before opening the first,
and **both recommend keeping the split: three contracts, two parameters, unchanged.** That is the two
independent agreements the panel requires, each grounded in its own compiled evidence, so what remains for op
is a one-line ratification of a shape nothing proposes to change rather than a choice between two. **Nothing
that ships moves either way**: the arity, the consumer's call site, the crate edges and the measured diagnostic
cost are identical under both readings.

**Four sentences the canon carries**, in the form file 120 drafted them at `120:453-492`, with the perimeter's
own wording kept because a guarantee whose limits are unstated is a claim rather than a guarantee:

> **The split.** The design declares three contracts. `Numeral` names what a number is, `Policy` names how it
> behaves, and `Lowering` names what it costs. `Numeral` is separate because what the number is does not change
> through strategies, which is D54 and is ratified. `Policy` and `Lowering` are separate because a bound on one
> does not project the other, and that is what lets a crate read the semantics of a number without being able
> to name its cost.

> **The bound.** `Number<N: Numeral, S: Policy + Lowering>` carries two parameters, and the second implements
> both strategy contracts. The bound is written as this conjunction wherever both halves are read, and the
> weaker bound `S: Policy` is written wherever only the semantics are. **The weaker bound is a declaration that
> the code does not read the cost axis, and the compiler checks it**: a `Lowering` member projected off a
> `Policy` bound is refused at `E0220` whether or not the lowering crate is linked.

> **The prohibition.** No trait in the design may have both `Policy` and `Lowering` in its supertrait closure.
> A bound reaches every member of its trait's supertrait closure, so a convenience trait
> `Strategy: Policy + Lowering` hands the cost axis to every crate that bounds on it, silently, while a
> dependency-graph check still reports closed. If the name `Strategy` returns it returns as a type alias for a
> bound at consumer-facing positions, or as prose, never as a supertrait. **Edges running the other way are
> safe and are how the carrier is factored: `Lowering: Carrier` is permitted, `Carrier: Lowering` is not.**

> **The perimeter.** The contract split is enforced over every crate that consumes the contracts and is silent
> over the one crate that composes them. A crate bounding on `Policy` cannot mention a `Lowering` member
> (`E0220`, and `E0432` if it names the crate). A crate owning neither the law trait nor the numeric type
> cannot impl the law at all (`E0117`). **The crate that owns `Number` can condition a law on the cost axis,
> and no mechanism prevents it**, because its own field projects through `Lowering` and any route by which it
> attaches a law is a route it authors. Sealing the law trait moves that route without removing it. The
> residual surface is one blanket impl per law trait, in one crate, and it is a review obligation with a
> compile-fail test, named rather than closed.

**Where the two reads disagree, and it decides what the canon writes down.** File 117 locates the enforcement
entirely at the crate edge (`117:218-223`) and writes the dependency edge's absence into the design as
load-bearing. File 120 locates it at the bound first and the crate edge second, and the difference is not
emphasis: **`117:493-494`'s trusted-base sentence, that an unrelated `lowering` dependency makes the whole
mechanism go silently with no diagnostic anywhere, is false.** Re-run in the strongest form available, with
`Lowering` fully in scope in the same crate, the projection still refuses:

```
error[E0220]: associated type `Layout` not found for `S`
  --> pF2.rs:95:10
   |
95 | where S::Layout: IsDense {}
   |          ^^^^^^ there is an associated type `Layout` in the trait `Lowering`
help: consider further restricting type parameter `S` with trait `Lowering`
```

**The refusal names its own repair at the line a reviewer reads**, `S: Policy + Lowering` in the impl header. A
widening of a law's bound is a visible one-line change in the crate declaring the law; a `Cargo.toml` edge is
not. A design that believes its guarantee lives in a manifest treats a manifest edit as a catastrophe and a
bound widening as housekeeping, and the truth is the reverse.

**The prohibition's severity, reproduced independently.** A probe that leaks through `S::Layout` still needs a
lowering name and so reads as a partial leak. **The member that makes it total is `Container`**, since it
projects to a concrete storage type and primitives need no import. Compiled: a law bounding on a roof trait,
naming nothing whatever from the lowering vocabulary, discriminating on the cost axis anyway.

```
error[E0277]: the trait bound `u8: WideEnough` is not satisfied
  --> pM_supertrait.rs:36:27
   |
36 | pub fn packed() { needs::<Fact<U13F3, ColdPacked>>(); }
   |                           ^^^^^^^^^^^^^^^^^^^^^^^ the trait `WideEnough` is not implemented for `u8`
note: required for `Fact<base::U13F3, ColdPacked>` to implement `AddAssoc`
```

Two strategies with identical `Policy`, differing only in layout, disagreeing on whether addition is
associative, from a crate holding no lowering name and passing every check that would be used to verify it.
**That is exactly what D54 forbids**, reached by a route the crate graph reports closed. The directional half
compiles both ways: `Lowering: Carrier` refuses `S::Layout` off a `Carrier` bound at `E0220`, and
`Carrier: Lowering` projects it at exit 0.

**And sealing does not close the residual gap, which closes off a family of repairs before anyone builds one**
(`120:325-374`). Sealing the law trait refuses the owning crate's direct impl, and the owning crate conditions
the declared route one level down instead, with the discrimination intact. **Any route by which the owning
crate attaches a law is a route it authors, and a route it authors is a route it can condition**, which
disposes of sealing, module privacy and marker-token schemes together.

**What the split does not close, compiled** (`09:135-170`, `09:198-254`). Section 1.3's enforcement result
carries the positive half: a law key is a `const fn` parameter list and `Lowering` is not a parameter, and a
value-level fact in the algebra-contracts crate cannot name an `Encoding` or `Lowering` type. **File 09's
harder result is that the crate that legitimately owns `Number` can still condition a law on `L`, and the split
does nothing to stop it.** **The three-argument spelling in the next sentence is file 09's own probe topology,
where the lowering is a third free parameter, and it is not this design's `Number`, which takes two** (section
1.1). Read with that said, the result stands exactly as it did. The dishonest impl builds clean at the one
location a real D72 would put it, and `Number<Fix13_3Signed, Warm, MinWidth>` folds while
`Number<Fix13_3Signed, Warm, DoubleWidth>` refuses with `Bitpacked: IsDense is not satisfied`: **two numerals
equal in every identity and policy respect disagreeing on whether their addition is associative**, which is
exactly what D54 forbids. The reason is structural: `Number`'s own definition requires `Lowering` in scope, so
any crate that can write the impl at all is a crate where `L` has methods a where-clause can name. **A shape
that closes it completely exists and is verified**: make the type the law targets not require `Lowering` at
all, a phantom `LogicalNumber<N, P, L>` with `L` free and unbounded, proven inside the `Lowering`-blind crate,
with the hostile second impl refused by `E0117`. Its honest cost, stated rather than priced: **it is an
architectural change past D72's literal shape**, since the type a law is proven about becomes distinct from the
type that holds bytes. It stays available and unrecommended: file 120 shows the residual gap is invariant to
every mechanism that leaves the owning crate authoring the attachment, so the architectural cost buys the
closure of a surface that is otherwise one blanket impl per law trait in one crate.

**One line stays on the open list rather than being landed**: whether the container projection moves to a
one-member carrier contract below `Lowering`. `Number`'s declaration is unchanged either way, so it is a real
improvement off the critical path, and **the spelling rule is the load-bearing half**: the carrier's
inhabitants must not be nameable from the law crate, which a primitive always is.

#### The eleven-row periphery taxonomy

**The eleven-row taxonomy the design round drew before the panel opened survives its first recheck with zero
deletions and zero merges** (`78:664-697`), and the periphery correction at `101b` replaced the blanket "every
other row survives" sentence with a table at decision granularity (`102:645-673`). **Its "Round decisions"
column is keyed on the inherited-state file's numbering, which is the sequence that collides**, so every cell
below carries its round rather than leaving the disambiguation to a sentence a reader has to have read first.

| Subject | Round decisions | Verdict |
|---|---|---|
| `arvo-capacity` | D5, D6, D7, D9 (from `202607281220`), D18b (from `202607290500`) (op) | Resolved as a locus. Capacity is a parameter, not the far point's own subject; a direct `Nat` instance for its value; the row owns a genuinely new construction (a predecessor on `Pos`, `Dec`/`PosPred`). The array-grammar question is reopened, section 1.27. |
| `arvo-shape` | D1-D4 (from `202607281220`), D7, D8 (from `202607281220`), D43, D44 (from `202607292100`) (op) | Ratified, content-reviewed, two details overtaken (section 1.28). Gains a job: the column-shaped capacity file 73 flagged as homeless is this crate's subject. |
| `arvo-geom` | D2 (from `202607281220`) contents, D10, D11 (from `202607281220`), D40, D41 (from `202607291910`) (op) | Ratified, content-reviewed, one ground overtaken (section 1.28). Two inherited obligations: the still-undecided dependency edge onto the algebra-contracts crate (open since file 26, still nobody's call) and D10's motors, now discharged since division's hold lifted. |
| `arvo-platform` | D27 (from `202607291700`) (op) | Ratified, content-reviewed (section 1.30). The charter is stated; the naming door has six exits; the Bool-placement fork is priced on both branches and locked to branch B bound on the algebra. The crate's name collides with the principles document's own "platform". |
| `arvo-float` packaging | D29, D30 (from `202607291710`), D50 (from `202607300400`) (op) | Ratified, overtaken in the strongest sense: the tower absorbed the packaging's contents and forced the boundary rather than the boundary choosing it. What remains is real: IEEE interchange-format instantiations, hardware-door lowerings, `Crosses` impls for hand-laid IEEE layouts. |
| Predicate concept | D15, D16, D17 (from `202607290200`) (op) | Content-reviewed (section 1.30). D16 is not a rung but the risk annotation on rung 2. D15's emphasis inverts: the degenerate marker family is load-bearing at three ratified sites and the arity machinery has no panel consumer. D17 is placeable, compiled, for a reason D5's precedent does not supply. |
| `arvo-pseudorand` | D42 (from `202607291920`) (op) | Ratified locus; gains its digest law in full (section 1.22). **The uniform-sampling spec question is genuinely open**, the one row where the periphery's original description was right. |
| `arvo-container` | D27, D28 (from `202607291700`), D45 (from `202607292100`) (op) | Its contract is substantially filled in rather than merely rewritten: it owns the container projection (`place`), statement C, the only-door canonicalising projection at the container level, and the foreign-bytes constructor's own obligation. D45's placement overtaken twice, its distinction preserved both times. |
| `arvo-bitfield` | D25 (from `202607291210`) (op) | Ratified, content-reviewed (section 1.29). The decision stands; both its stated grounds are replaced by the category the design acquired since. |
| `arvo-num-systems` | D38, D39 (from `202607291900`) (op) | Ratified and panel-worked; not periphery at all. File 64's correction folded in before the crate ships: scope the "finest" fact to the real/Cayley-Dickson chain explicitly; independent predicates per branch elsewhere. |
| `notko-hlist` + `Cardinal` | D5, D6, D7, D9 (from `202607281220`), D18b (from `202607290500`) (op) | Ratified; **one binding-time sentence still owed**: a count that decides a type is a type-level `Nat`; a count computed at runtime is a `Cardinal`; the mirror between them is a one-way projection. `notko-hlist` was flagged at `26:661-666` as "the single cheapest, most repeatedly-flagged open item in the whole document" by six separate members independently, and it is still owed. |

Both colliding-range rows resolve to run one, `202607281220`, checkable by finding `**D1.` through `**D4.`
under that round's heading at `inherited:293`.

**Why this is a layering question rather than a naming pass** (`78:687-693`). Three newer mechanisms make crate
boundaries load-bearing in ways the original round could not have weighed: the seal's guarantee rests on which
crate declares the sealed vocabulary (compiled to survive a crate split); the law-key rule and the orphan rule
both turn "what may this impl read" into a dependency-edge question rather than a packaging one; and the spine
rule keeps minting capacity-shaped types (`ShortCap`, `ByteCap`, the column capacity) whose outputs need one
shared home or become their own fragmentation vector.

**The genuine ignorance is one bounded item, down from two.** The tenth consolidation named two: the
uniform-sampling spec question in `arvo-pseudorand` and the integer-saturating SIMD lane residue.
`arvo-bitfield`, `arvo-platform` and the predicate concept were all content-unread and have been read, so **the
one genuinely open row is the sampling spec**; the SIMD lane residue is not a periphery row but an owed
obligation.

**The saturating-reduction residue, with its priced obligation** (`26:442-450`, reduced at `40:662-665` to one
line in a list of owed codegen regression tests, `109:270-273`): saturating integer reductions do not
vectorise, correctly, because saturating addition is non-associative; unlike the float case, **source-level
regrouping does not recover parallel lanes** (four scalar saturating adds instead of one, regardless of how the
source is written), and there is no LLVM IR flag for integer saturating arithmetic to grant in the first place.
**The only route to lane parallelism here is arvo hand-writing per-architecture vector kernels itself** (the
hardware instructions exist: `uqadd` on aarch64, `paddus*` on x86), **which is a real, currently unpriced cost
that lands on arvo rather than on any build layer.** A test pinning the fact is not the obligation to write the
kernels, and reducing the obligation to a test is what the fourth consolidation did.

**The multi-limb fragility item** (`26:452-457`, dropped at `40`, `109:274-277`): a 256-bit carry chain
compiles cleanly because LLVM recognises the `carrying_add` idiom and there is nothing to vectorise in a serial
carry chain, **but `core::arch::aarch64` has no carry-propagating intrinsic to fall back to if that idiom
recognition ever regresses under a toolchain bump**, unlike x86_64 which has one. **This is a dependency on an
optimiser heuristic holding, not a guarantee, and it costs one codegen test to make falsifiable.**

**The model-inadequacy standing risk** (`26:104-109`, dropped at `40`, which rests more claims on bounded
exhaustion at a model width than `26` did and carries neither the risk nor an entry, `109:280-283`): **a model
can be inadequate in two structurally different ways, and the apparatus only guards one.** A runtime panic on
an unreachable refusal catches the case where a model *undercounts refusals*, loudly, by construction.
**Nothing catches the case where a model is too narrow to see a *value* disagreement and quietly returns a
wrong number with no refusal at all**; this was found once, by an accumulator sweep run out of curiosity rather
than by any standing rule. **No mechanical fix for the second case exists yet.** The transfer grounds narrow it
but do not close it, and the container-class coordinate is one instance of exactly this shape found later by a
different route.

**The four-bin ledger** (`11:813-868`, `26:99-103`, dropped at `26:99-101` in its contents and partly restored
at `40:31-32`, `109:250-253`). Every claim sorts into one of four bins: **machine-checked by construction;
machine-checked by bounded exhaustion at a model width (whose transfer to real widths rests on the
forbidden-feature bans in `unstable-features.md`); measured on the pinned toolchain and target; or reasoned
without a compiled artifact.** Two additions the second dive found missing entirely: that a composition's
declared axes are honoured by the bodies that run under them, and that any build layer acting on a licence
acted only inside it. Both sit at "validated per artifact" to the extent the mechanisms are built. **The
`TypeId`-and-specialisation dependency in the second bin is the item that later became a workspace rule**
(`unstable-features.md`'s "the forbidden list is verification infrastructure" section), and it is the reason a
model-width check transfers at all.

### 1.26 Naming, and the environment parameter

**Two naming rules stand, and they are orthogonal. The standing base carried one of them.** The first
constrains what a name may *claim*; the second constrains how a name is *spelled*, and it governs everything
the taxonomy round will mint.

**D56 (from `202607301100`), op's own** (`talk:399-404`), verbatim:

> **D56. No gratuitous abbreviation. Full, legible, recognisable words for every member.** Decision (op,
> 2026-07-30). An abbreviation is acceptable only where it is the stable form nearly everyone in the field
> already recognises. Coining short forms of words that were already a sensible length is not arvo's style and
> has never been. This applies to the whole round's output, not to one draft.

Applied by op in the same call, with the reason each short form was wrong (`talk:408-418`): `Under` to
**`Underflow`** (the full word is the axis, and it is four characters longer); `Over` to **`Overflow`** (`Over`
reads as a comparison); `Round` to **`Rounding`** (`Round` is a verb; the axis is the discipline, not the act);
`Grow` to **`Growth`** (same); `Total` to **`LogicalWidth`** (`Total` names no quantity); `fexp` to
**`canonical_exponent`** (Flocq's own prose calls it the canonical exponent; `fexp` is its Coq identifier, not
the concept's name). `Scale`, `Sign` and `Layout` stay, being complete words already. **Every member name this
document uses is a D56 output**, which is why the vocabulary reads the way it does and why a future proposal
reintroducing a short form is refused by a ratified call rather than by taste. Six searches for its wording
returned zero in the standing base (`113:219-239`), and **the taxonomy round mints every name in the design**,
so a standing base carrying one of the two rules is a standing base that licenses half the naming.

**The refined principle, adopted at `90b`, one word repaired, confirmed whole at `108b:104-108`.**

> *A name may freely denote type-level content, because the compiler checks the claim by construction. A name
> may promise behaviour only where the design **names in the record** the verifier that checks the promise;
> until the verifier exists, the promise is an entry in the trusted base, auditable as a list, with the
> verifier as its closing artifact. A name that promises behaviour with no designated verifier is forbidden.*

"Designated" became "named in the record" because the stricter reading would eat the design's most-written
names (`Cold`, `Hot`) whose verifier is a bench the review has not yet been willing to run; a named-but-unbuilt
verifier makes a promise an honest trusted-base entry with a closing artifact, the same accounting every
hand-laid `Crosses` entry already gets. Op at `108b:106-108`: **"Every existing name passes, so the principle
forbids only names nobody has written yet."**

**The `67b` principle is dead**, superseded by op's own `79b`: "a type name must not read as a verified
standards claim the architecture admits it cannot verify" forbids the very intent pillar op ratified
(namesake-asserting names kept as a full intent pillar, with differential parity suites as the designated
verifier).

**The class test is replaced by nothing; the widened definitional-completeness line does the work**
(`102:695-722`). File 90's sweep asked whether the compiler checks a name's claim. File 94 ran the same names
against what a reader at a call site, holding one token out of a declaration's four or five, concludes, and
found `91:850-851`'s "every existing name in the design passes" **false at three found names**:
`Hot`/`Warm`/`Cold`/`Precise` denote two different rows each, one per number kind, and `Warm`'s float row
diverges from its fixed-point row on the single sharpest cell the presets have (compiled);
`StoredWidth::Minimum` misled the review itself for two files; `IeeeSpecials` and `E4M3` name inhabitants, not
semantics, and a reader who assumes semantics concludes something the design has explicitly not committed to.
File 94's own replacement rule (an instantiation where denotation and what a competent reader concludes
diverge) **did not survive its own second read**: it shares the exact defect it correctly struck the parity
suite for, since the only reader available to the author of a ratifying text is the author, who has read the
panel.

**Adopted: the class-one clause is replaced by nothing.** The widened definitional-completeness line does the
work at **three mechanical addresses** that need no house style about a hypothetical stranger:

1. a name appearing in more than one ratified table or definition with different content;
2. a name whose correct reading requires a fact stated on a different axis or in a different section;
3. a name sharing a token with an external standard whose scope exceeds the design's own claim.

Against file 94's own divergence list, all six fail at least one address; the four clean names (`Fnv1a`,
`quantise`, `TotalOrd`, the namesake aliases) pass all three, matching file 94's finding by a different route.
**No new naming rule.** Address one is where the design's three live token collisions sit: `Exponent` against
`ExponentForm` (section 1.2), `Dec` against `PosPred` (section 1.27), and `Precision` the trait against the
bridge's result type (section 1.23, closed by giving the result types their own names).

**Six boundary sentences relocate to the definitions the reader reaches**, four of which already exist
elsewhere in the corpus and only move, per the confirmed condition that the move is within the ratifying text,
not a copy: the section that resolved a fork cites, and does not restate. `Specials` members name inhabitants,
not semantics. `E4M3` names identity axes; the deployed overflow mode is a separate axis on the lowering. A
preset name is a pair of rows keyed by number kind, stated above both tables (section 1.21 does this, and
section 1.21 also states the mechanism that makes the pair expressible). `StoredWidth::Minimum` is the carrier,
not the container. `Folded<N>`'s `N` is the site count, an upper bound on the moved count. `IeeeDefault`
denotes the assumed ambient control state.

**Three further token collisions, all address three or address one.** **"Platform"**:
`mock/PRINCIPLES.md.tmpl:322-326` defines it as "syscalls, threads, clocks, filesystems, which live in consumer
crates", and D27 (from `202607291700`) names a crate `arvo-platform` whose contents are five wrappers over
language-level primitives and zero syscalls. Nothing is violated; **one token now means "syscalls, which live
elsewhere" in one ratified document and "host primitive names, which live here" in another, and a reader
holding either concludes something false about the other**. Remedy: relocate the boundary sentence to the
definition the reader reaches, or change the name; candidates offered as suggestions rather than a call
(`arvo-primitives`, `arvo-host`, `arvo-named`). **If the name stays, the chapter owes one sentence stating that
"platform" here means the host's primitive types and not the platform the zero-platform-dep principle refuses,
next to both.** **"Bitfield"**: the ISA's own name for the instruction class the design's bitpacked decode is
built from (a two-literal bitfield extract, `ubfx`). One token, two ratified senses. The cheap remedy is that
**the design's own object is `bitfield!`'s output while the instruction is an extract.** **"`Bool`'s
residual"**: `arvo-storage/src/platform.rs:285-291` declares `BoolResidual` whose doc comment states bare
`core::convert::Infallible` **cannot** carry the impl (the orphan rule forbids implementing `core`'s `Residual`
for a foreign type), while `arvo-storage/tests/bool_consttry.rs:47-50` names
`<Bool as ConstFromResidual<Infallible>>::from_residual`, **using `Infallible` as `Bool`'s residual**. Both are
correct: two `Try` vocabularies, `core`'s and notko's const bridge, and the orphan argument applies to one and
not the other. **The duplication is forced by const-callability rather than chosen, so this is not the
two-organs defect**; it is a completeness item, and **"`Bool`'s residual" is a function of which `Try`
vocabulary is in scope, and neither declaration says so.** One sentence at the `BoolResidual` declaration
closes it.

**`IeeeDefault`'s artifact list loses one artifact incapable of failing and gains one that demonstrates the
others do.** The parity suite listed against the deployment residual is structurally incapable of failing on
that claim (a parity suite run in a process either has the assumed control state or does not, and either way it
reports nothing about a different deployment); struck, and kept against the arithmetic claim, where it can
fail. The receipt's mask itself was found missing a field this host latches (FZ16, half-precision
flush-to-zero), which means the remaining list had **no negative control**. **Adopted: a perturbation arm joins
the list.**

**The assumption sentence, confirmed at `108b:110-114`.** *An environment parameter denotes the **ambient**
control state the lowering's correctness is conditional on. It is an **assumption, never a witness**. A fact
the deployment cannot perturb is not environment, it is a lowering decision, settled where the code is emitted,
and belongs on the lowering.* The pre-authorised NaN-on-overflow mode is the worked case: no control register
holds it, it is a choice between two instruction forms, settled where the code is emitted, unperturbable by any
linked library. Modelling it as an environment assumption would defer to a runtime trusted-base entry a fact
compile time already settles.

**The operative test, sharpened**: **can a linked library change the fact at runtime**, applied
per-fact-as-lowered rather than per-fact, since the identical abstract fact (rounding mode) is ambient under
one lowering (addition) and instruction-encoded under another (a rounding cast) **on the same target**,
compiled and executed. The environment type's own field set is therefore keyed on the lowering, not on the
target alone, and a compile-time-settled fact adds no **new** trusted-base entry, riding on the standing
toolchain trust every compiled claim in this review already carries.

**The receipt, adopted as a per-target-per-lowering field set folded in const position, with its verdict split
rather than its declaration refused.** The hand-written receipt does not portably check what the name denotes
(the x86 form is a different register with a different field set, not three instructions; a naive
transliteration compiles and passes with a denormal-flushing deployment). The fix the pricing pillar already
implies: a per-target field set on the type, the receipt a fold over it in const position, **a zero mask
meaning the target cannot check the field, never that the field is satisfied.**

**The corrected verdict, and it is a reversal worth stating as one.** A proposed finisher ("a numeral whose
correctness depends on an uncheckable field refuses at declaration") was reversed: **refusing specifically the
uncheckable case converts an honest trusted-base assumption into refusal on ignorance, the policer posture by
the design's own boundary sentence, and blurs the provable-versus-trusted line from the direction nobody had
yet blurred it, treating unverifiable and false as the same thing.** Second-read confirmed at `105` section 2:
the finisher and the ratified assumption sentence make incompatible claims about the same thing and only one
can stand, and there is no type-level state for illegal states to be unrepresentable **of**, since an ambient
FPU control register mutable by any linked library is not a value any Rust type carries.

**Adopted: the receipt's verdict splits rather than the declaration refusing.**

- **Cannot check.** A target that cannot check a field yields a trusted-base entry with its artifact gap
  declared, through a split verdict: a checked mask, an expected word, and an unchecked field set, all
  const-derived, the consumer's acknowledgment a const assertion that refuses if the bundle later grows a new
  unchecked field, emitted body unchanged at four instructions.
- **Cannot provide.** A target that cannot provide the field's semantics at all (no gradual-underflow path at
  any cost) is a **statically known falsehood** and refuses at declaration, correctly, which is the design's
  ordinary treatment of a statically known falsehood and unrelated to the environment chapter.

**The cannot-check-versus-cannot-provide distinction has now been reused independently at two further layers**:
for a target whose `usize` is too narrow for the capacity model, which is a statically known falsehood and
refuses at declaration with the same `E0080` the capacity repair produces (`103` section 1.4); and for a
bitfield overlap, where **a stated overlap is a declaration and a silent overlap is a falsehood the compiler
can see** (`104` section 3, section 1.29 below). Three independent instances make it a general mechanism rather
than an environment-chapter clause.

**The required-field relation** (which fields a numeral's correctness actually depends on) is named open, a
genuinely new item, plausibly derivable from the numeral's own axes and unwritten.

**Held, still op's own: `Hot`'s default environment.** Nothing forces a choice; the concrete pick is reserved to
op. Renaming was tested and fails its own test: every noncommittal candidate either asserts something the door
does not do (`AmbientFloatEnv` reads the ambient state, which the door never checks) or is annotation smuggled
into an identifier at the cost of the grep surface op's intent pillar depends on.

*Grounded on: ratified (`79b:20-27`, `90b`, `95b`, `97` the verdict-split reversal, `108b:104-114` op's own
confirmation of both the naming principle and the assumption sentence, D56 op's own), settled shapes
(`71:288-310`, `78:325-326`, `78:409-441`, `105` section 2), compiled (`94_probes/probe_1` through `probe_5`,
`95_probes/`, `97_probes/probe_2`, `probe_4`, `90_probes/probe_1`, `103_probes/p6`).*

### 1.27 Capacity: is-a stands, and the array grammar's forcing argument is reopened

**Capacity is a type-level parameter that establishes an index domain, of the same kind as `Precision`,
`Exponent`, and `StoredWidth`, and not a value that flows through an operation and can land outside its own
domain** (`91:778-787`). Op's reframing at `77b` reads two literalisms into one sentence: capacity as a
supremum of a numeral's own value set (which the far-point rule already governs), or capacity as the supremum
of a collection's valid index set `{0, ..., N-1}`, whose lastmost member is `N-1`, one predecessor step off the
count itself. **The far-point rule's own subject is what an *operation* resolves to when its true result falls
outside a value set a *parameter* already established; capacity is the parameter, not the operation's own
subject, and the far-point rule fires one construction downstream, at index arithmetic.**

**Capacity's value is a direct instance of the tower's `Nat`, not a second encoding and not an alias.**
`Capacity: Nat`, one seal, one ordering, one arithmetic, inherited wholesale, with `SIZE` reading straight
through the supertrait; compiled, zero feature gates, and exercised at all four uses the far-point analogy
names (array length, index-bound membership, an iteration terminator, an arity), each consuming the same
`Nat`-typed count through the same projection. This closes the two-encodings finding from the seventh
consolidation at the value layer completely.

**Is-a stands, confirmed at `108b:34-43`.** Op:

> The panel's answer survives re-derivation against op's own "it contains a numeral" wording, and the compiled
> evidence carries it: **two ratified consumers need a capacity *inside* type-level arithmetic whose result is
> a type, and a const parameter cannot produce one.** Op's wording was about what a capacity denotes rather
> than how it is spelled.

**Two corrections ride with it, both `108b:41-43`.** **The claim that the pairing is "forced by the language" is
false; it is forced by the choice of an inductive numeral, compiled twice.** **And under the paired form a
generic capacity-producing operation cannot be given storage at all, only a caller-supplied literal.**

**The far-point rule's shape recurs one layer down, at index arithmetic, and this is genuinely new
construction** (`91:804-817`). The last valid index below a capacity is its predecessor, **total over nonzero
capacities by the identical supremum-over-the-ordered-set logic, and undefined (refused at the type level, not
clamped to a sentinel) over an empty capacity**, matching the far-point rule's own NaN-exclusion shape exactly
rather than merely resembling it: `last_index` is generic over `C: Pos + Dec`, and `Z` implements neither, so
`last_index::<Z>()` fails at the trait bound before monomorphisation, with rustc naming the missing bound
directly. Built at zero cost to the sealed vocabulary: **`Dec`/`PosPred` recurses structurally on the
constructor shape** (`I<Q>` steps to `O<Q>` with no recursion; `O<O<Q>>` recurses through a carry chain bounded
by the number of trailing zero bits), the identical family as `VAL`, `Cmp`, and `Gcd`, and confirmed on the
safe side of the recursion-depth ceiling. **This construction belongs in the same shared bottom carrier crate
proposed for `Nat`/`Pos`/`Bias`**, since it is pure `Pos` content with no capacity-specific or
numeral-specific meaning. **The construction is written two ways in one sentence, `Dec` and `PosPred`**, and
one construction with two spellings is, by the widened completeness line, defined nowhere; the naming call is
on section 2's list with the two other collisions.

**The agreement fact, and the repair it needs, in both halves.** `91:796-802`'s sentence that the array
grammar's agreement is "checked to agree in an inline const block at the one construction door" **is false
above rank 0**: the door the capacity resolution means is the inherent method, where the assertion lives; D4
(from `202607281220`)'s recursion is written against the **trait** method instead, which had no check at all,
because at rank 1 the inherent door was the only door. Compiled: a rank-3 shape whose middle axis declares a
`Nat` of 4 against a literal of 7 has two disagreeing const-evaluable counts and nothing raises (`E0080` fires
through the inherent door; the trait route silently returns the wrong array). **The repair lifts the agreement
to `const AGREES: bool` on `Capacity` itself**, a fact about the type rather than about one call site, exactly
the shape the level-ordering refusal already uses. The qualifier records which of the two live `D4`s this was
always about, which is `202607281220`'s "a shape is a cons-list of capacities" (`inherited:527`) and not
`202607282100`'s vetting-record decision (`inherited:798`).

**Both halves are needed and either alone leaves a door**, and the second half is the demand-driven clause
arriving at its own address: **an associated const nothing touches is not evaluated**, so a capacity whose
`COUNT` is read without touching `AGREES` still compiles with a disagreeing size (compiled, exit 0, exactly
what an unreferenced associated const predicts). Closing it needs a second reference, `assert!(Hd::AGREES)`
inside `COUNT`'s own definition. **Both halves are owed and neither is built.**

**Where a declaration-site refusal has to live to be one**, compiled at three placements against a violating
declaration that is never constructed (`104` section 3.3): an associated const **mentioned by nothing**
compiles silent; an associated const mentioned by a `const fn` in the same impl refuses with `E0080`; a **free
anonymous const item** beside the type refuses with `E0080`. The general form both the capacity finding and the
bitfield finding want is the same: **the well-formedness of a placement or a pairing is a fact about the type,
and it is stated where the type is, not where a value is made.** The free const item is the cheapest spelling
and it costs one line of emission, **and it is available exactly where the design owns the declaration site**,
which a macro does and a consumer-instantiated generic does not; where the design owns no declaration site the
fact must sit in a position every route resolves, which in practice is the associated item the routes already
consume.

**The array grammar's forcing argument, reopened on two independent compiles, and this is the one place in this
document where a ratified sentence's stated ground fails and its conclusion is left standing.** `91:791-798`,
ratified: "The array grammar is a paired, non-derived fact, **forced by the language, not chosen**. No
expression of `[T; K]` computed from a type-level `Nat` exists under the permitted feature set." **The second
clause is true and narrow. The first is false, twice over.** The reopening is authorised outright by
`04b:72-74`, op's own licence to argue against a ratified call provided the argument is made rather than
asserted.

- **False once because the pairing is conditional on the choice of an inductive numeral** (`106` section 6,
  confirmed at `108b:41-42`). A capacity whose value is a `const N: usize` supplies
  `type const VAL: usize = N`, which is a **path** rather than an expression, so `[T; <Self as Nat>::VAL]`
  resolves under `min_generic_const_args` alone. Compiled clean, exit 0, with no second name at all: a rank-3
  axis list has `COUNT == 60` and `size_of(Store) == 60`, asserted equal **through the trait route**, with no
  `AGREES`, no inline const block and no construction door anywhere in the program, extents chosen 3/4/5 so a
  transposed count would show. The shipped tree reaches the same place with no feature gate at all
  (`arvo-tensor/src/capacity.rs:44-48` declares `Dim<const N: usize>` with `type Array<T> = [T; N]` and
  `const CAP: Cap = cap(N)`, both from one `N`). **The bare-const-read leak closes too, because there is
  nothing to leak.**
- **False again because the pairing is conditional on obtaining the storage by its length** (`107` sections 4
  and 5). A storage derived from the numeral's own **structure** rather than from its value needs no literal:
  `O<P>` is two of `P`'s storage, `I<P>` is two of `P`'s plus one element, `H` is `[T;1]`, `Z` is `[T;0]`,
  `repr(C)`. Compiled, exit 0, **zero feature gates, every array length in the file a literal**.
  `Pz<Sum<N5,N7>>` has `VAL == 12` and `size_of::<Array<u32>> == 48`; `Pz<Sum<N47,N47>>` has `VAL == 94` and
  `size_of::<Array<Odd9>> == 846`. **Nobody declared a 12 or a 94.** Swept at 23 numerals across 4 element
  types including a nine-byte struct at align 1 whose size is not a power of two, **184 layout and alignment
  assertions plus 23 value assertions**, executed at capacities 1, 7, 13, 47 with every slot written and read
  back through the projections and the raw byte length checked so trailing padding would show. **The negative
  control fires through the generic projection**, which names no numeral, no length and no law, at a
  monomorphisation the caller chose: `error[E0080]: evaluation panicked: storage law violated: array grammar
  does not match the numeral`.

**Under the ratified paired form, a generic capacity-producing operation has no storage** (`107` section 3,
ratified at `108b:42-43`). `Slot<Sum<N5,N7>, 12>` compiles concretely, **and the `12` is a number a human did
arithmetic to produce.** The generic signature cannot be spelled at all: `error: generic parameters may not be
used in const operations`, "cannot perform const operation using `A`". The tenth consolidation already requires
this and did not notice: `102:911` says "The derived extent then pairs with its literal exactly as the capacity
resolution requires, and a wrong literal, **including D10's own original miscount**, is refused with the same
`E0080`." That sentence is a fair description of the situation the paired form leaves, and D10's original
miscount is cited there as the thing the check catches; **it is also evidence about who is doing the
arithmetic.**

**What the numeral column buys, exactly one capability**: operations whose argument is a capacity and whose
result is a capacity (`107` section 2). **That capability has at least two ratified consumers**: the bivector
extent (`102:904-913`, derivable through the identical structural-recursion family, checked at ranks 2 through
8) and shape composition (any concatenation, split, reshape or broadcast on a multi-dimensional index domain
produces an extent from other extents). **What merely needs the value** (index-bound membership, iteration
terminators, the element `COUNT` product, arity) is all four of the far-point analogy's own uses, and the const
column serves all four without difficulty. **Comparison, gcd and ordering are not the cost**; they are
reachable from either column at value position, gate-free. **The cost is exactly one thing: an operation whose
result is a type.**

**Compile-time measurement, and the design sentence it yields rather than a price** (`107` section 5). Flat in
the capacity for all three shapes, at rustc's process-start floor, nothing growing with `N` through
`N = 1048576`. On the monomorphisation axis the derived column first measured quadratic-looking (0.28, 1.22,
5.31 s at K = 25, 50, 100 against 0.15, 0.28, 0.73), **and that was wrong**: isolating a type-only body from a
constructing one showed 0.17 s against 0.83 s at one sum and 0.08 s against 3.24 s at another. **The type
machinery is free; the cost was one operation per element, emitted because `filled` had been written as
structural recursion and `-O` inlined it.** Rewritten as provided trait methods over the projected slice with
no recursion in any body: **3.24 s to 0.12 s, a 27x collapse, for identical semantics**, and at K = 400
distinct capacities the derived column costs 0.39 s against the const column's 0.22 s, a residual in tenths of
a second that under the pricing pillar is nothing. **The design sentence**: *where a type's shape is a
structural recursion, every function over it is written once against the projected view, never recurred
alongside it.*

**Codegen is identical.** Four operations at capacity 13, each written twice (derived storage against
`[u32; 13]`), in one binary: LLVM merged three of the four pairs into a single symbol, and `derived_sum`
vectorises to NEON and `native_sum` **is** that symbol. The fourth pair is instruction-for-instruction
identical, differing only in which `Location` constant is passed to `panic_bounds_check`, whose payload differs
at one byte recording the source column. **A source-location record, not code.**

**The perimeter rule fires here on its own terms.** A derived storage's correctness argument is "values of this
type are exactly `VAL` contiguous `T`", which is a property of every value of it, **which is the rule's own
antecedent**. So the combinators' fields are private and the two projections are the only doors, and the const
block is not a belt but the perimeter: there is no route to an element that does not resolve it.

**The three columns, priced.** This document does not pick; op does.

| | const parameter | numeral + literal (ratified) | numeral + derived storage |
|---|---|---|---|
| names for the number | one | **two, per declared capacity** | one |
| agreement fact | none | exists, needs a route | none |
| `AGREES`, its two-half repair, its reachability scope | not needed | **needed, repair unbuilt** | not needed |
| value arithmetic (`Cmp`, `Gcd`, ordering) | const fns at value position | inherited from the tower | inherited from the tower |
| **capacity-producing operations** | **refused** | type works, **storage cannot be spelled generically** | **works** |
| storage for a computed capacity | not applicable | a literal a human computes | derived |
| shares one encoding with the tower's exponents | no | yes | yes |
| falsifiable surface | none | **one per declared capacity, unbounded** | **three lines, fixed** |
| feature gates | none in the shipped form; `min_generic_const_args` for the `Nat` indirection | none | none |
| compile time, K = 400 | 0.22 s | between the two, not measured | 0.39 s |
| codegen | native | native | **native, same symbol** |
| `[T; N]` by name | yes | yes | **no, slices only** |
| `unsafe` | none | none | two projections, discharged per monomorphisation |

**The genuine cost of the const column is the last-but-three row**: the numeral tower needs type-level
arithmetic producing **types**, and a const parameter does not participate in it. **That is why is-a stands at
`108b:34-43` and why the column is not being recommended against here.** What is reopened is narrower and
specific: **the array grammar's pairing is not forced by the language, and the derived-storage column removes
it without leaving the tower.**

**The proposed replacement sentence for the ratified one**, offered rather than adopted, because it corrects
ratified text and this document's own rule is that a call about the canon needs two independent agreements:

> *The array grammar's pairing is forced neither by the language nor by the choice of an inductive numeral
> alone. It is forced by obtaining the storage as `[T; K]`, by its length. A storage derived from the numeral's
> own structure needs no literal, and was compiled with two controls at
> `76_probes/b1_structural_array.rs`, recommended against at `76:220-226` on four stated grounds, carried by
> the eighth consolidation as open item 12, and dropped by the ninth without a droplist entry.*

And the statement of what a capacity is:

> *A capacity is a numeral in a role. It is a direct instance of the tower's `Nat`, one seal, one ordering, one
> arithmetic, inherited wholesale, and what it adds is not a second number but a projection to storage, keyed
> on the capacity because the storage is a function of the count and the element type and nothing else. That is
> the `Lowering` charter at rank 1, the same statement section 1.28 makes for shape one dimension up.*

**Status.** Two independent reads complete on the is-a answer, ratified at `108b`. The array-grammar reopening
is two independent compiles from two members reaching it by different routes, and it is **one pass on the design
question** and goes to op with three columns on the record. Section 8 records that the third column's record
was lost once and how.
### 1.28 Shape and geometry

**Shape is not a geometry subject. It is the index-domain layer, the same kind of thing as `Precision`,
`Exponent`, and `StoredWidth`, and it already has its rank-1 case: `Capacity`.** A shape is a
multi-dimensional index domain; everything past that (rank, per-axis extents, element count, index and stride
arithmetic, the projection from a logical index domain to a physical container, rank-generic folds) is
numerics, not space. **`arvo-shape` is downstream of the capacity resolution and upstream of everything with a
container, the third instance of a pattern this design has already worked out twice**: a logical extent, a
declared carrier, and an allocated container, one dimension up from the width levels.

**D4 (from `202607281220`) survives the capacity resolution verbatim, compiled**: a rank-3 shape's
nested-array storage and its independently-computed element count agree by two unrelated routes, not by
restatement. **But D4's own sentence makes the array composition constitutive of the shape, while D43 (from
`202607292100`) (op, the same round, one day later) states the opposite**: "the shape abstraction supplies rank
and extent and nothing else, so a bit container implements both and neither one grows the other's surface."
**Resolved by writing D43's sentence literally**: `Shape` carries rank and extents; **a separate trait projects
a shape to storage, once per element domain.** Not a new mechanism, but the `Lowering` charter one dimension
up, exactly the layer-keying rule's own instance named in this document's rules section. **Adopted as a working
shape, second read owed.** D43 needs no qualifier, since it occurs once in the register.

**`Layout::Bitpacked` at rank N needs no new mechanism, and the exact coincidence condition with the per-axis
reading is derived rather than sampled.** Two readings exist at rank 2 and above (pack each innermost row
independently, or pack the whole shape as one run of `COUNT` elements); **the flattened reading is never worse
in footprint over an exhaustive 131,072-case sweep**, and under it the group arithmetic already ratified for
one dimension applies unchanged, since it is a function of the stored width alone. **The write granule becomes
a shape fact: an outer-axis partition is legal only when `inner mod P == 0`**, which generalises section 1.22's
own per-column statement. **The two readings coincide exactly when the whole per-axis padding fits inside one
byte (`outer * rowpad < 8`)**, a one-line proof once stated, and **the sampled-first guess (byte alignment
alone) mismatched over four percent of the swept grid**, which the exhaustive sweep caught and a spot-check
would not have. **Adopted, the flattened reading, no new mechanism.**

**A live asymmetry in a shipped bitmask type, offered as why-evidence rather than as a fix to propose.**
`BitMatrix`'s two axes of one rank-2 shape are declared in two vocabularies: the row extent is a `Capacity`,
checked and refused out of range; **the column extent is only a `Bits<N>` width, with no shape behind it and no
refusal.** A safe write past the declared column extent lands, and two values equal at every index the shape
declares end up with different byte images, **reaching the mutation gap's own signature through an ordinary
safe call, at rank 2, with no unsafe and no niche.** Compiled and executed; no source fix proposed,
`mock/crates` being outside the panel's scope. This is the same shape as the bitfield overlap defect (section
1.29), and **two independent instances of one shape is worth more than either.**

**Compile time is flat in rank, and the rank-generic fold folds away at small element counts, both measured
rather than assumed.** Metadata compile time sits below rustc's own process-start noise from rank 1 through
rank 16; the rank-generic fold compiles to five instructions at rank 3, count 8, and vectorises to NEON by rank
8; a dense shape's stack allocation is a function of element count alone (262,144 bytes at count 65,536, both
rank 2 and rank 4), the no-heap rule's own consequence rather than anything rank-specific.

**Geometry's multilinear half (points, orthotopes, affine maps, their predicates, their measure) composes over
the settled operation surface at no new growth class, no new failure kind, and no new vocabulary.** Every
operation is a sum of products; exact accumulator widths for dot product, affine apply, rotor compose, and
rotor sandwich are all derived from the already-settled multiplicative and fold classes. **Renormalisation is
the one operation touching the design's exponential division class, and it touches it only in its statement**
(a quotient never materialised as a numeral, since normalisation is `quantize(exact quotient)` through the
Euclidean pair), never in a byte. This discharges an open item file 74 recorded: `arvo-geom`'s motors need
normalisation, which needs division, which needed the hold lifted; the hold lifted at `90b` and the surface
adopted at section 1.13.

**D10 (from `202607281220`)'s rotor component count is wrong from rank 4, and the correction reverses D10's own
storage comparison at rank 7.** D10 grounds rotation on rotors and gives its storage argument in terms of
degrees of freedom, `1 + n(n-1)/2`; **the actual coordinate storage a rotor needs, as an element of the even
subalgebra `Spin(n)`, is `2^(n-1)`**, since `Spin(n)` is a curved manifold inside that space rather than a
linear subspace of it. The two counts agree at rank 2 and 3 (where the even subalgebra of `Cl(3)` **is** the
quaternions, D10's own comparison case) and diverge from rank 4: **7 against 8, 11 against 16, 29 against 128
at rank 8**, exhibited with an exact-rational unit rotor whose grade-4 component is nonzero and whose
degrees-of-freedom slots cannot hold it. **The consequence runs the opposite way from D10's own argument**:
rotor storage first exceeds matrix storage (`n^2`) at rank 7, which the degrees-of-freedom count would never
show, since `1 + n(n-1)/2 < n^2` at every rank. **What this does not touch: D10's decision stands on the
grounds it also gives** (rotors generalise where quaternions do not, motors extend to rigid motion in one
composable object, `Affine` is the rigid-motion-plus-scale type geometry already wants), and nothing above
argues for a matrix. **What changes is that the review's own bench matrix for rotation representations, ordered
by op (D41 (from `202607291910`)) to run at named requirements before any comparison, needs at least one rank
of 4 or above to measure the generality claim D10 actually rests on**, since rank 3 compares one representation
against itself.

**The identity rotation is not representable in the numeral geometry reaches for first, the same absent element
file 99's sqrt overflow band found independently in the same stretch.** A rotor component, a direction cosine,
a normalised colour channel, and a barycentric weight all live in a closed interval, and the obvious numeral is
purely fractional, **missing its representable one by exactly one quantum at every width** (`78:723`'s known
defect, now shown load-bearing for the operation the rotor formulation exists to make cheap: normalisation
itself). **The fix is a value of a parameter the identity contract already seals**, not a new mechanism:
`Adjustment = 1/(r^F - 1)` gives both endpoints exact at the same container width (Direct3D's UNORM rule,
converged on independently by the design round's own sibling colour pass), compiled at every `F` from 1 to 24.
**Two routes exist and the design should expose both rather than pick silently, per the toolbox rule**: one bit
of integer headroom (keeps the dyadic quantum, spends storage, crossing the same rank-7 boundary as the
rotor-versus-matrix comparison), or the rational adjustment (exact endpoints, a constant divide per multiply,
already priced as the exact division subfamily). **What the design should not ship is the numeral that carries
neither endpoint.** This closes the round's own D36 (from `202607291800`) (section 1.11) and answers, on the
arvo side, a question the design round's addendum flagged as falling between two rounds: **a UNORM-shaped type
is not a type, it is a parameter value**, and the sibling crate `kirjo` owes no new type and is owed no
request, only this sentence.

**D10's flagged const-expression hazard dissolves, and the fallback the round proposed compiles.** D10 worried
that a rotor's `1 + n(n-1)/2` extent as a computed const-position expression is exactly the hazard `Capacity`
exists to avoid, and named a fallback (deriving the basis count as a type-level fold over the axis list) that
the round's own prior-art pass found no library implementing. **Absence is not impossibility, and the compile
settles it**: the bivector-extent count is derivable through the identical structural-recursion family
(`Dec`/`PosPred`, `VAL`, `Cmp`, `Gcd`) the tower already uses, one impl-selection step per axis, **zero
arithmetic expressions in type position, zero feature gates, checked at ranks 2 through 8**.

**The bench requirements D41 named first and nobody had produced, supplied per row from what the design has
settled since.** Fixed-point representability resolves to the closed-interval question above, with its two
priced routes. Which operations must be cheap now names each operation's growth class and exact accumulator
width, with two operations (matrix inversion by transposition, rotor inversion by a sign flip) **free rather
than merely cheap, invisible to a naive bench**. Degradation under repeated composition restates as the site
count, **a compile-time number derivable before the bench runs rather than discovered by it**. Storage width is
the corrected counts above, in bits after the numeral choice, not in components. **Three rows the design has
acquired since D41 and could not have named then**: whether a form's identity is representable at all
(disqualifying, not merely costly); whether the form's exact carrier exists (the transcendental class's licence
structure governs the Lie-algebra exponential map on the rotation groups, the one row on D41's own variant list
that is not multilinear); and the bench's own measure obeying the pricing pillar.

*Grounded on: ratified (D1-D4, D7, D8, D10, D11 (from `202607281220`), D40, D41 (from `202607291910`), D43, D44
(from `202607292100`) all op, `91:113-126`, `91:157-159`, `91:811-814`, `91:288-291`, `101b`), settled shapes
(`74:64-65`, `78:723`, `99` section 2, `40:493`), compiled (`100_probes/probe_1` through `probe_8`), external
(Abel-Ruffini, cited not compiled), reasoned (the trait split, the requirements table, two-read status: one
pass throughout, adopted as working shapes at `101b` with second reads owed on the trait split and the `AGREES`
repair).*

### 1.29 The bitfield, and the composite level

**The convergent statement, ratified at `108b:148-153`, entering the ratifying text:**

> **A bitfield is a heterogeneous product of numerals sharing one container under a declared placement map, and
> a bitpacked column is the homogeneous product of one numeral under a derived one**, the two differing only in
> whether the index is a `Nat` or an hlist and whether the offsets are derived or declared.

**Everything a bitfield needs the design already has, and a member looked for a new mechanism and found none.**
The maps are `embed`, `place`, `materialise` unchanged. Statement C governs the container's bits outside the
map. The only-door projection governs a field read. The byte-sharing law decides which fields have byte images.
The group arithmetic transfers verbatim into a bitpacked column of bitfields, keyed on the element stride,
invariant under every repartition of that stride (compiled at **1,596 shapes**: every stride from 1 to 57
crossed with every two-field partition of it, zero disagreements).

**The four-cell classification, and all four cells have referents.** The usual explanation decomposes as:
*several values* is a product; *of declared widths* is the fields extent per member; *into one container* is
`place`; *typed access to each* is a projection indexed by the product's own index type.

| | offsets derived | offsets declared |
|---|---|---|
| **index is a `Nat`** (homogeneous) | a column: `Dense` (stride is the projected container) or `Bitpacked` (stride is `W_S`). Both ratified. | a foreign array: the stride is whatever an external document fixed, including its padding. A `Crosses`-shaped claim. |
| **index is an hlist** (heterogeneous) | a bitfield with no written offsets. Compiled, named by nothing in the corpus before this stretch. | a bitfield as shipped: `field: W at LO`. D25 (from `202607291210`)'s subject. |

**The unifying reading of the right-hand column: offsets get declared when an external document fixed them.**
An array-of-structs from C has a declared stride because a compiler somewhere chose the padding; a hardware
register has declared field positions because a datasheet chose them. **Where nothing external fixed anything,
the offsets are a prefix sum and writing them by hand is transcribing a derivation.** Both generalisations were
already forced elsewhere: heterogeneous type-level lists (`notko-hlist`) and a declared correspondence to a
foreign layout (`Crosses`'s hand-laid impls). **The bitfield is their composition, and that is the whole of what
it is.**

**Confirmed by an independent route** (`105` section 4): a homogeneous product indexed by a natural number and
a heterogeneous product indexed by a finite set of labels are the same categorical shape, both a dependent
product over a finite index type, differing only in whether the fibre is constant across the index or varies
per index. A vector specialises `Pi (i : I) . A i` at constant `A`; a record specialises it at `I` a finite
label set. **Two independent routes to the same conclusion is stronger corroboration than either alone.**

**A bitfield is not a numeral, and the composite level is datum-only by construction.** A numeral is
value-unique: it denotes one number, and **the ratified machinery keyed on that (the quantiser, `Identity`,
`Bounded`, the far point, the crossing contract's value-layer clauses, the resolution row, the operation
surface's admission test) has no subject when the thing in the container is a tuple.** What applies is exactly
the datum-layer machinery and nothing else: `place`, statements P and C, `materialise`, the byte image, the two
digest stopping points, the only-door projection, **a complete list checked section by section**. **Every
ratified law about a composite is a statement about placement, materialisation or digests, and none of them is
about a quantiser.** A column has the same property for the same reason, **and having two makes it a fact about
composites rather than a fact about columns.**

**The only-door law's reason, in one line**: a field read manufactures a datum out of a slice of container, and
the container's own bits are only canonical because statement C says so, **so a read that bypasses the
canonicalising projection is reading a region whose guarantee it did not consume.**

**Two obligations, one of them currently the author's, and this is where a bitfield is expensive.** A derived
placement proves both well-formedness facts by construction: the offsets are a prefix sum, so the fields are
disjoint by monotonicity, and the total is the occupancy, so containment is one comparison. **A declared
placement proves neither and owes both.** The general form: **derived is provable-by-construction, once;
declared is provable-by-checking, per instance, or trusted if nobody writes the check.** That is a real
structural difference in where the burden of proof lives, and it is exactly why `Dense`'s stride needed no
obligation while a bitfield's declared map needs two.

- **Containment is asserted and it fires.** The shipped macro asserts `$lo + $field_bits <= $n` per field, and
  a declaration of `wide: 8 at 12` in a 16-bit container, **in a crate that constructs nothing and uses
  nothing**, refuses with `error[E0080]: evaluation panicked: sub-range wide does not fit within N bits`.
- **Disjointness is not asserted, and the shipped macro says so** (`arvo/src/bitfield.rs:28-30`: "Overlap
  detection is deferred to a future macro version (for now, authors are responsible)"). Compiled and executed
  against the shipped macro: `Overlap: 16 { a: 8 at 0, b: 8 at 4 }`, bits 4..8 belonging to both, sets `a` to
  `0xFF` and reads back `0x0F`, **because writing `b` cleared four of its bits. Safe code, no `unsafe`
  anywhere, no warning, nothing in the emitted diagnostics at any level.** This is **the mutation gap's own
  signature reached without a raw door, without a niche and without a transmute, by an ordinary setter on an
  ordinary declaration**, and it is the same shape section 1.28 found in `BitMatrix` at rank 2.
- **Both are const-evaluable**; the second costs an `O(k^2)` loop over a list a human wrote, and at the largest
  shipped declaration `k` is three. Compiled: `error[E0080]: evaluation panicked: two fields overlap`.

**Overlap is refused unless declared, ratified at `108b:155-159`.** Op: **a deliberately aliasing view field is
a real idiom, so an overlap becomes a statement the author made rather than one they failed to notice.**
Refusing it outright would be the policer posture the toolbox rule names, since a 32-bit register with a
`word: 32 at 0` beside its named fields is how consumers read and write the whole thing. **An overlap that is
stated is a declaration; an overlap that is silent is a falsehood the compiler can see**, which is the
cannot-check-versus-cannot-provide distinction one layer down (section 1.26).

**The containment locus moves to an emitted free const item, ratified at `108b:158-159`.** The shipped check is
real and **hangs on two lines**: `_BOUNDS` is declared at `arvo/src/bitfield.rs:377` and mentioned by
`let _ = Self::_BOUNDS;` at `:393` and `:399`, inside `new()` and `from_bits()`. A refactor that drops or
renames either constructor takes the guarantee with it, silently, **and the `ConstDefault` impl at `:370-374`
already constructs a value without mentioning it.** Compiled: the unmentioning door builds the lying type and
compiles; the mentioning one refuses. **The free const item fires with no route, no mention and no
construction**, and it is the cheapest spelling at one line of emission.

**The digest's one-word correction lands with it**, ratified at `108b:159` and stated in full at section 1.22.

**Two defects, two repairs, and one sentence covering both would let a reader do one and believe they did the
other.** The disjointness gap is an **absent obligation** with no locus at all, so it is not an instance of
"checked at one route among several"; it wants an obligation written. The containment locus is route-shaped and
is the pricing pillar's own case; it wants a written obligation moved off two lines inside two constructors and
onto the type.

**The composite of two placements is a placement, and the algebra closes the category.** Element `i` of a
bitpacked column sits at absolute bit `i * W_S`; field `f` at intra-element bit `o`; so field `f` of element
`i` sits at `i * W_S + o`. **`slice(slice(b, i*W_S, W_S), o, w) = slice(b, i*W_S + o, w)` whenever
`o + w <= W_S`, which is the containment obligation.** The composite is a placement, **and the reason it is one
is the obligation a declaration owes; the check is not bureaucracy, it is what makes the composition valid.**
Swept over 4096 elements of stride 13 with fields `(0,3) (3,5) (8,5)`, **every element carrying a distinct
13-bit value so the element space is swept rather than sampled: zero disagreements between the two forms and
zero against the packed input.**

**A column of bitfields needs one decode plan, reused per field with a constant addend**, not one plan per
field, because `(j*W_S + o) mod 8` is `((j*W_S) mod 8 + o) mod 8` and each field's lane-shift sequence is the
first field's rotated.

**The group arithmetic keys on the stride, not on the field.** Under a bitfield, `W` splits: the period `P` and
the group byte count `G` are functions of the element stride, while the mask and the load width are functions
of the field. **This is the level-naming clause's first instance where the fields side is a list rather than a
scalar**: a bitfield has one occupancy but several field widths, and a per-field mask names one of them.

**Which composite to emit is not a property of the layout.** Measured, per group of eight: one field two-step
38 instructions, one field one-step 38; **all three fields two-step 399, all three one-step 467**. Reading one
field is identical either way; reading all three favours the two-step by **17 percent**, because the element
load amortises across the fields and the one-step has nothing to amortise. **So the answer is neither, and the
choice belongs to the consumer's access pattern**, which is section 1.22's own sentence arriving independently
one level in, at a different pair of alternatives. The one asymmetry the counts do not show: at wide strides the
composed read can use a narrower load (at stride 57 the element needs an eight-byte load where a three-bit
field needs two), the one place the one-step form is strictly cheaper. **These are static instruction counts
over differently-unrolled bodies and are not a runtime claim**; a runtime claim goes in the bench harness and
this one has not earned a bench.

**A refutation recorded because refutations are the point.** The composite was predicted to need stating, on
the reasoning that the intermediate element mask is dead work the optimiser cannot see through. **It is not**:
`((x >> s) & M_S) >> o & M_w` equals `(x >> (s+o)) & M_w` whenever the field is contained, and **LLVM performs
the collapse itself, at both binding times** (four bodies, 23/24/23/24 instructions, the composed forms one
instruction *worse* because of the address add). **The composition is a statement about what is true, not an
optimisation to emit.**

**Under a genuinely bitpacked column of bitfields, no field has a byte image.** File 73's law composed once:
field `f` of element `i` has an independently addressable byte image when `8 | (i*W_S + o)` and `8 | w`, for
every `i`, which needs `8 | W_S` and `8 | o` and `8 | w`. Under `Layout::Bitpacked` the interesting case is
exactly `W_S mod 8 != 0`. **So no field of any element has a per-field byte image, and the only byte image is
the column's.** That closes a question rather than opening one, and it means **the mutation perimeter's byte
owner under `Bitpacked` is the column group for a bitfield exactly as it is for a numeral, with no
bitfield-specific clause.**

**A foreign bitfield cannot be bitpacked.** A bitfield laid against a foreign format has a declared container:
the register is 32 bits because the device says so, and the fields sit at offsets measured from the container's
bit 0. Packing it into a column at a stride below 32 moves every field's absolute offset and destroys the
correspondence the declaration exists to state. **A foreign bitfield is pinned to `W_S = W_C` and cannot be
bitpacked without ceasing to mean what it was declared to mean.** An internal bitfield has no such pin: its
offsets are relative to the occupancy, its carrier is the occupancy, its container is projected, and it packs at
stride equal to the occupancy like any other carrier. **The shipped grammar spells only the first and the tree
uses it for the second**, which costs one sentence in the chapter and removes a question that would otherwise
be discovered by the first consumer who tries to pack one.

**What a bitfield costs the trusted base.** `Crosses` is per-numeral, and a bitfield is not a numeral, so
asking which `Crosses` entry a bitfield needs is the wrong question. **An internal bitfield adds no trusted-base
entry at all**: its fields are each a numeral with its own lowering, already covered, and its placement map is
derived and const-checked. **A foreign bitfield adds exactly one, and it is genuinely new: the placement map's
correspondence to the external document.** No compiler can check that bit 31 of a device register is the enable
flag; **the artifact is the datasheet or the wire specification, cited by position**, and the accounting is the
one `exhaustively-computed-or-cited` already established. **No file assigned a bitfield's foreign
correspondence anywhere before this stretch, and it is the only place a bitfield is expensive.**

**D25 (from `202607291210`) stands, and it stands on a category the design acquired after it locked, ratified
at `108b:148-149`.** Op: **the crate decision stands and both its stated grounds are replaced by the category
the design acquired since: the crate holds notation.** Both written grounds are overtaken. **The proc-macro
upgrade is not forced**: a bitfield's arithmetic is prefix sums over widths and the widths arrive as separate
tokens, so nothing needs decomposing, and a `macro_rules!` muncher accumulating the prefix sum as an
unevaluated token sequence compiles **with no feature gates at all**, checked by rebuilding a 16-bit
declaration from its fields at every one of 65,536 container values, both directions, zero mismatches. The
contrast with the notation vehicle is exact: **file 61's wall is structural (a decimal literal is one atomic
token and the declarative attempt cannot start), and this one is not a wall at all.** What the upgrade would
buy is real and is ergonomics: `#[bitfield] struct Flags { .. }` reuses Rust's own struct grammar, so rustfmt
formats it, an editor completes its field names, and go-to-definition lands on them. **That is a good reason
and a different reason from the one D25 gives.** **And the granularity ground prices at about a tenth of a
second**: a proc-macro crate's full build as a host dylib costs 187.1 ms against 66.4 ms for an ordinary lib,
**so roughly 121 ms is the floor price of reaching a proc-macro crate at all**, paid once per build of the
dependency graph, and the marginal cost of a second macro inside an existing proc-macro crate is comfortably
under that floor. **On cost alone, sharing wins**, and a proc-macro crate exports function-like, derive and
attribute macros together, so D25's own load-bearing sentence is an argument for *a* proc-macro crate rather
than for one per macro.

**The design's two macros are two instances of one thing**: a host-side stage that turns a notation into a
type, each existing because of a binding-time decision rather than a convenience (section 1.18's own quoted
sentence). **That is what `arvo-bitfield` holds, and it is what the crate should be described as holding.** The
crate-kind call (its own crate, a proc-macro crate, or sharing one with the notation vehicle) is op's, with all
three priced and none of the prices large.

**The macro dispatches its own container by a per-N macro (`__bitfield_container_ty!`), which is the container
projection written a second time. The design has one of those and it is not this one.**

**Which level a bitfield's declared width is, is named open rather than answered.** It declares `N` and calls it
the bitfield's width. Under the ratified three levels, `W_S` is the one declared level and `W_C` is never
declared. **A hardware register's 32 is its container, because a device fixed it. A design-internal bitfield's
32 is a carrier at best and is more honestly the occupancy.** The two cells need different answers, and the
question interacts with `StoredWidth`'s ratified reading and with whether an internal bitfield packs.

*Grounded on: ratified (D25 op, `108b:148-159` op's own confirmation of the crate decision, the convergent
statement, the overlap refusal, the containment locus and the digest correction), settled shapes (`91:544-550`,
`73:205-215`, `74:265-271`, `81:199-243`, `61:596-600`, `105` section 4), compiled (`104_probes/p1`, `p1b`,
`p2`, `p2b`, `p3`, `p4`, `p4b`, `p4c`, `p5`, `p6`, `p7`, `106_probes/p4`), measured (the 187.1/68.3/66.4 ms
crate-kind sweep, the 38/38/399/467 instruction counts).*

### 1.30 The platform crate, the predicate concept, and the truth contract

#### The platform crate's charter, confirmed at `108b:163-166`

> **The platform crate is the design's single naming door for the host's primitive types; it names each one
> once, exposes no route to the primitive except the ones it declares, and contains no fact that is not settled
> where the code is emitted.**

Op at `108b:163-166` confirms the charter, states **the number of routes is one**, and adds: **every
construction and extraction door is `#[inline(always)]`, because the crate boundary costs 34 instructions
against 22 and the failure mode is silent.**

**D27 (from `202607291700`)'s real content is a name-keying rule one level below the layer-keying rule.** The
layer-keying rule asks what a *fact* is keyed on; D27 asks what a *name* is keyed on, and answers that **a name
over a host primitive is keyed on the host, not on the container the primitive happens to carry.** Op's own
reason: "a platform primitive has to be named **once** and wrapped, and that is a different job from holding a
bit container." **The interesting part is "once".**

**The crate is not the design's hardest trusted-base case, and four of five items add no trusted-base entry.**
Applying the ratified operative test (can a linked library change the fact at runtime), compiled:

| Item | Fact it rests on | Can a linked library change it | Verdict |
|---|---|---|---|
| `USize` | `usize::BITS`, a target cfg | no | settled at emission, const-assertable |
| `Bool` | `bool` is one byte, align one, validity `{0,1}` | no | settled at emission, const-assertable |
| `BoolResidual` | uninhabitedness of an empty enum | no | settled at emission, a language fact |
| `AsBool` | nothing; it is a projection | not applicable | not a fact |
| `NUSize` | `NonZero`'s excluded pattern and its discriminant elision | no | **one trusted-base entry, already written** |

Compiled assertions that hold with no runtime check anywhere: pointer width against the four legal values;
`size_of::<bool>() == 1`; `align_of::<bool>() == 1`; `size_of::<Option<bool>>() == 1`;
`size_of::<Option<NonZeroUsize>>() == size_of::<usize>()`. **The fifth item is not new either**: `NUSize`'s
width claim is the same claim section 1.12 already accounts for in one sentence, and **the platform crate
inherits that entry verbatim; it does not open a second one.**

**The correction this makes to the periphery's stated grounds**: the design's genuinely unprovable host facts
are the environment chapter's (rounding mode, flush-to-zero, the ambient control word, all of which a linked
library can change under you at runtime). **The platform crate holds none of them. A pointer width is not that
kind of fact, and calling it one blurs the provable-versus-trusted line by treating *settled elsewhere* and
*unverifiable* as the same thing.**

**The chapter owes the cannot-check-versus-cannot-provide clause one layer down.** A target whose `usize` is
too narrow for the capacity model is a statically known falsehood and refuses at declaration, compiled and
firing with `error[E0080]: evaluation panicked: arvo's capacity model needs at least a 32-bit index domain`,
**the same `E0080` the capacity repair produces**. The refusal firing on a real 16-bit target is owed and was
not performed: no small target is installed, and both attempts returned `E0463: can't find crate for core`.
**Named rather than assumed.**

**The naming door has six doors, and the finding changes what gets built.** `Bool` reaches its `bool` through
six independent public routes: a public field (`arvo-storage/src/platform.rs:261`), `Transparent::raw`
(`:264`), `Deref<Target = bool>` (`:275`), the `Try` exit (`:293`), `AsBool::as_bool` (`:328`), and
`From<Bool> for bool` (`:342`). `USize` and `Cap` carry public fields too (`:45`, `:73`). **Only `NUSize`
(`:485`) is closed.** **A wrapper with six exits is not a wrapper, it is a suggestion.**

**What the chapter should say**: *the naming door declares exactly one route from each named type to the
primitive it names, and that route is stated in the type's own definition; every other route is absent, not
discouraged.*

**The five named types are quoted here as tree facts and are deliberately not declared.** `Bool`, `USize`,
`Cap`, `NUSize` and `BoolResidual` appear above by citation into the shipped tree because the chapter's subject
is how many doors each has, and **the door that survives is op's call**, listed in section 2. Writing a
declaration here would be writing that call. A cold read flagged all five as used without definition
(`115:107-110`), which is right about the text and wrong about the cause: **this is the one place in this
document where a missing declaration is the finding rather than a gap in it.**

**The ground for that sentence is redundancy, not soundness, and the distinction decides whose call it is.** An
earlier draft cited the workspace perimeter rule, and **the citation is struck**: the rule's own Boundary
section says "This is not 'make everything private'. A type with no invariant to protect loses nothing by
exposing its fields... The rule fires when a type's argument for correctness depends on values of it having a
property." **`Bool`, `USize` and `Cap` have no invariant**, compiled exhaustively: six doors modelled on the
shipped ones, over `Bool`'s whole two-element domain, all agreeing at every value, asserted in **const
position** so the agreement is a compile-time fact; **nothing separates, at any instantiation, ever.** The
contrast compiles too: the same six doors on a type carrying "the inner byte is never zero" gives five reads
that preserve it trivially and one public field that is a write breaking it **with no `unsafe` and no
diagnostic**, the positive half exhaustive over all 256 inputs.

**So: route multiplicity is a defect relative to a guarantee. Without one it is a redundancy in the surface.**
That matters because **the two grounds put the call in different hands.** Under D27's own "named once and
wrapped", the right number of doors is one because a name should have one spelling, **which is a vocabulary
decision and is op's**. Under the perimeter rule it would be one because a guarantee needs a closed perimeter,
**which is a soundness question and is nobody's to trade.** Carrying both converts a question op should answer
into one he cannot, **which is the policer posture arriving from the direction nobody watches: not refusing a
consumer's choice, but removing from the record that it was a choice.** The finding survives whole minus one
sentence: `Bool` has six spellings of one projection where D27 says one, and by the two-organs rule four of
them are three too many. **Which of the four survives is a consumer-ergonomics call and is op's.** The design
has already chosen `Try` as the intended route, because the shipped doc comment says so in its own words
("Preferred path in WU code is `?`") and because **`?` is the only exit that is also a control-flow
construct**.

**A crate boundary in Rust is a call unless the constructor says otherwise.** Measured across a real crate
boundary built for the question, three crates, `-C opt-level=3`, no LTO: with the wrapper's constructor
`#[inline(always)]` the model loop is **22 instructions with no calls**; without it the same loop is **34
instructions**, carrying a `bl` to `Bool::new` **once per element**. **Every construction and extraction door on
a named primitive is `#[inline(always)]`, and the reason is the crate boundary rather than taste.** One
attribute, and it must be written down, **because the failure mode is silent: the code is correct, the numbers
are two thirds of what they should be, and nothing reports it.** This is Kind-1 structural lowering per
`arvo-always-optimal-internals.md`, so it is a chapter sentence rather than a bench obligation. The public field
is free today precisely because a tuple-struct literal is not a call.

**The crate's name collides with the principles document's own "platform"** (section 1.26). Op's call.

#### The predicate concept

**D16 (from `202607290200`) is not a rung; it is the risk annotation on file 07's rung 2.** File 03 demolished
the derived and asserted dichotomy read literally, and file 07 rebuilt it as three rungs: **computed and
witnessed** (a const check refuses disagreement at instantiation); **declared, total, coherent** (human-typed,
every constructor forced to answer, no witness possible because the domain is not bounded or not decidable);
**promised** (claims about emitted code with no type-level referent, `unsafe impl`-shaped, discharged by a bench
artifact). **The ladder sorts by *what checks the fact*. D16 sorts by *what a false fact costs*. Those are
orthogonal, both are needed, and collapsing them is what made the dichotomy read as a dichotomy.**

**The ratifiable paragraph, offered as a working shape:**

> A property carried on a type sits on one of three rungs by what checks it, and carries one of two risk
> classes by what its falsehood costs. Rung 1 is refusable by the const evaluator and is the only rung where
> "cannot lie" is true. Rung 2 is declared: the compiler forces every constructor to answer and forbids
> contradiction, and nothing checks the answer. Rung 3 is promised, discharged by an artifact rather than by
> the type system. On rung 2 and rung 3, a property whose falsehood changes an answer rather than a cost is
> asserted through an `unsafe impl` with a stated contract, per D16; a property whose falsehood costs only
> speed is a safe declaration.

**D15 (from `202607290200`)'s one named property lands on rung 2, settled by compiler refusal.** The
fresh-accumulator guarantee (prose at `arvo-comb/src/greedy.rs:32-34`, guard at `:60-64`) **cannot reach rung
1**: the witness would have to decide, at const-eval time, that a consumer's closure accepts every item against
a fresh accumulator, and that does not compile (`error[E0277]: the trait bound F: [const] Fn(&u32, &u32) is not
satisfied`), because a consumer closure is not const-callable. **The witness has no expressible form, which is
a stronger statement than "the domain is too large", and it settles the rung by refusal rather than by
argument.** It reaches rung 2 **only if the const has no default**: with a default,
`impl Defaulted for Careless {}` compiles and has silently promised; without one, `impl Total for Silent {}` is
`error[E0046]: not all trait items implemented`. **A property const carries no default. Silence is not an
answer.** Its falsehood changes an answer, so by D16 it is an `unsafe impl`: a predicate that wrongly promises
the guarantee causes an item the algorithm should have skipped to open a group it cannot close, **which is a
different grouping, not a slower one.**

**The property buys two instructions, and that is deliberately not the argument.** Measured: `group_promising`
is 27 instructions with 7 branch-class operations; `group_silent` is 29 with 8. **One branch and two
instructions, at group-open rather than per item.** The number is small and it is recorded precisely **so that
a later reader cannot re-derive the mechanism as a speed argument, which is the shape this review has watched a
4.6x figure take once already.** The round's own topic says the value is not codegen: "the value is that the
property **cannot desync** from the thing it describes", and it is right.

**The emphasis inverts: D15's courtesy half is the load-bearing half.** D15 spends its length on the arity
machinery and disposes of the marker family in one clause, and the predicate topic is harsher still (the
markers gate nothing, no algorithm bounds on one, **and two of the five have identical bodies kept distinct so
a consumer can name a semantic that nothing reads**). Since then, that degenerate instance acquired a job it did
not have: **it is the mechanism by which a kind-2 failure is refused at the declaration.** A failure whose
admissible domain is expressible as a predicate on the operand is refused where the fact enters, and the design
already ships that vocabulary twice over (notko's `NonZeroable`, and the
`IsZero`/`IsNonZero`/`IsPositive`/`IsNonNegative` family). **Reused three times in ratified text**: at
`84:386-393`, at section 1.13's divisor domain, and at section 1.16's `sqrt` over a signed domain. **The marker
family that "gates nothing" now gates the design's entire kind-2 failure story.**

**The reverse half: the arity machinery has no consumer in the panel's record at all**, established by fresh
searches (`Pred2`/`Pred3` return nothing; `typestate predicate` returns two files, both table rows; **file 55,
whose whole subject is typing the algorithm crates that hold the arity machinery's only two callers, contains
the string "predicate" zero times**). The mechanism is real, the sketch is sound, and one hundred files of
design work have never needed it. **Both halves should be built, since D15 decided both; the chapter should say
plainly which one the rest of the design leans on, because a reader of D15 alone would guess the other one.**

**D17 (from `202607290200`) is orphan-legal, and for a reason its own analogy does not supply.** D5 (from
`202607281220`) records why `Cardinal` had to sit beside `Cons`: if notko declares both trait and list types,
an impl in arvo has two foreign items and cannot be written. **The truth contract has no such problem**: `Bool`
is arvo's, so one foreign trait and one local type is the orphan rule's ordinary permitted case, compiled across
three real crates. **D17 is placeable, and it is placeable for a reason D5's precedent does not supply.** One
consequence: D15's wrapper carries a type-level hlist, which is `notko-hlist`'s, while D18b (from
`202607290500`) puts the predicate in `notko`, **so `notko` depends on `notko-hlist`, or the wrapper moves**;
the notko-to-notko-hlist edge is explicitly not verified. Separately, **D18b and D19 (from `202607290500`)
disagree on a referent thirteen lines apart** (D18b assigns `Cardinal`, `Length`, `Cons` and `Empty` to
`notko-hlist`; D19 still says "the reasoning D6 used to keep `Cardinal` in notko"), a completeness item rather
than a conflict, closed by one sentence, and it matters here only because D17's analogy points at a referent
that changed crates between the two decisions.

#### The truth contract, and the fork

**The fork, stated in current vocabulary**: the tower's contracts emit derived booleans and membership
predicates, and `arvo-bridge-home-rule.md`'s test says a trait lives in the lowest layer where its **return
type** is reachable. **Branch A**: those contracts return `Bool` concretely, so `Bool` and therefore the
platform crate sit **below** the numeral contracts, inside the tower's dependency cone. **Branch B**: notko
declares the truth contract, the tower's contracts are generic over it, `Bool` is a peer that implements it,
and the platform crate stays out of the cone. **The fork is not about `Bool` and not about notko. It is about
whether the numeral contracts name a concrete truth type or a contract**, and everything else is a consequence.

**Both branches are cheap, so cost does not decide it.** Branch A: zero compile surface, zero runtime, and the
naming collision reads wrong even where it costs nothing. Branch B: **one type parameter and one bound at
fifteen trait declarations**, counted fresh in the shipped tree (92 signatures return `Bool`, of which 15 are
declarations: `arvo` 28, `arvo-storage` 26, `arvo-bits-contracts` 20, `arvo-numeric-contracts` 12,
`arvo-bitmask` 5, `arvo-mask-contracts` 1; declarations 8/4/2/1 across four crates), the other 77 sites
following their declaration and changing nothing but a name. Bound restatement through a call chain costs one
extra per level, reduced to nothing at consumer depth by deriving in one blanket and consuming through one
name. **Runtime: zero, by symbol identity rather than by measurement.** Compiled across three real crates at
`-C opt-level=3`, the assembler emitted **`_run_b1 = _run_a`**: the generic and concrete spellings lowered to
byte-identical code and LLVM merged them into one symbol. **Not "within noise". The same function.**
Independently reproduced and extended: a raw primitive, a concrete truth newtype with the exit by identity, and
the generic form all merged to one six-instruction symbol, **so the exit-by-identity is free against the
language's own floor rather than merely against the other branch.**

**A second truth type is shipped**, `arvo-mask-contracts/src/lib.rs:45-66`'s `MaskOps` with `empty`, `full`,
`union`, `intersection`, `complement`. **Those five are `FALSE`, `TRUE`, `or`, `and`, `not` under a
set-theoretic vocabulary**, and `arvo-bitmask/src/ops.rs` carries `BitAnd`, `BitOr` and `Not` impls besides. The
design ships **two Boolean algebras with disjoint vocabularies**, which is the fragmentation the design exists
to prevent, at a place nobody had looked.

**But the shipped second type is a witness, not the ground.** Under this document's own method a design
sentence must survive deleting its shipped-source citation, and "is there a second truth type? there is, and it
is shipped" does not: delete it and nothing is left. **The ground that does survive is a theorem**: Boolean
algebras form a variety in the universal-algebra sense, and varieties are closed under arbitrary direct
products with the operations defined pointwise, so **`Mask<W>` *is* `Bool^W` under the product construction,
structurally, not by resemblance.** A truth contract whose shape is Boolean-algebra-correct **has** finite
products as instances whether or not one currently ships, so **the unification is the free construction the
contract predicts rather than a convenient pun that happens to compile**, and `MaskOps` demotes to a witness
that the design already reached for the instance once, the right weight for a tree fact.

**The exit, and the split.** Rust's `if` takes `bool` and cannot be overloaded. `Bool` supplies its exit for
free (the identity). `Mask<W>` cannot, because `if` wants one bit and a mask carries `W`. **Any exit a `Truth`
impl for `Mask<W>` supplies is necessarily a reduction**: all lanes, some lane, a specific lane, a majority.
**These are not equivalent operations, they encode different policies, and none is derivable from the
Boolean-algebra structure alone.**

**The algebraic statement is sharper than that, and it is exhaustive.** The Boolean-algebra homomorphisms from
an n-lane truth algebra to the one-lane one **are exactly the n coordinate projections**, enumerated
exhaustively at n = 2 (all 16 candidates, exactly 2 survive) and n = 3 (all 256 candidates, exactly 3 survive),
**and neither `all` nor `any` is among them at either width**. So the exit is not merely underdetermined by the
algebra: **above one lane the operations consumers actually want are outside the algebra, and at exactly one
lane the unique homomorphism is the identity.** `all` is a meet-semilattice map and breaks `or`; `any` is a
join-semilattice map and breaks `and`. **The exit is a homomorphism exactly at one lane. Above one lane the
exit either preserves the structure and is useless, or is useful and is outside the structure. There is no third
option, and that is checked rather than argued.**

**This is the layer-keying rule's own dual failure, named for the first time.** The rule catches a fact keyed
too finely and calls that a false statement. **The dual failure is a fact keyed on something that does not
determine it, which is not a false statement but a non-function presented as one.** For a mask, `is_true` is not
a function of the mask; it is a function of the mask and a reduction. **A named reduction is exactly the supply
of the missing argument, and a default is exactly its silent supply.** Section 1.21's preset key is the second
instance of the same dual failure and is where both of the rule's failure modes fire at once.

**The corrected shape, ratified at `108b:130-144`.** Op:

> **Confirmed, corrected.** The exit split is real and more forced than argued: exhaustively, the algebra
> homomorphisms from an n-lane truth to a one-lane one are exactly the n coordinate projections, and neither
> `all` nor `any` is one, so above one lane the exit is genuinely outside the algebra.
>
> **But the persona's third clause was backwards.** Binding the fifteen declarations on the exit-carrying part
> refuses the multi-lane instance at the impl, and that instance is the entire thing the generic branch buys;
> all fifteen are producers and none branches on its own result. The slip is one inference made once and
> carried through three documents.
>
> **The exit belongs to the operation that branches, not to the truth contract**, and the operation that
> generalises is a **selector keyed on the pair** rather than on the truth. Written once against that, `max` is
> correct at both lane counts with no exit anywhere and is byte-identical to the raw primitive at one lane.

Mechanically: taking the shape and changing only the bound from the algebra to the exit gives
`error[E0277]: the trait bound Mask2: Branch is not satisfied`, `note: required by a bound in Compare::Truth`.
**The multi-lane instance is refused at the impl, and the multi-lane instance is the whole content of branch
B.** All fifteen declarations were read individually (`bit`, three `get_bit`, four `is_zero`, two `test`,
`is_non_negative`, `is_positive`, `is_zero_or_positive`, `const_bit_eq`, `const_eq`) and **every one is a
producer; none branches on its own result. Their callers are.**

**A wrapper repair is worse than the problem**: making the producer name `All<Mask2>` as its truth compiles, and
**relocates the choice from the trait to the impl**. There is exactly one associated type per trait and `Self`,
so the comparison decides once, for every caller, forever; the call site reads `a.eq(b).is_true()` and silently
means all-lanes; and a caller who wants any-lane has no route whatsoever. **A default on a trait is at least one
visible place. An impl-site choice is per-type, invisible from the consumer's bound, and there is no place a
reviewer would think to look.**

**The selector, keyed on the pair.** `max` at one lane is `if a < b { b } else { a }`; at two lanes the correct
answer is lane-wise, **and lane-wise is not a branch at all, it is a blend**. So the operation that generalises
across lane counts is not "reduce the truth to a bool and branch", it is "select, lane-wise, between two data":

```rust
pub fn max<T>(a: T, b: T) -> T
where T: Compare + Select<Truth = <T as Compare>::Truth> {
    T::select(a.lt(&b), b, a)
}
```

**The comparison borrows.** Written `a.lt(b)` with a by-value `lt` the body does not compile,
`error[E0382]: use of moved value: b`, because a by-value comparison moves `b` and both operands are used again
in the same expression; rustc's own note points at the repair, "consider changing this parameter type in method
`lt` to borrow instead if owning the value isn't necessary". The borrowing form is taken, since the alternative
repair is a `Copy` bound this document states for no numeric type anywhere. **This is the document's one
operational code sample** (`116:43-45`), presented as the resolution of a locked fork with a compiler
diagnostic quoted in support, and it had gone unbuilt through a full audit, a full restoration and two cold
reads.

That compiles at one lane and at two, and at one lane it is byte-identical to the raw primitive. **No exit
appears in the bound, in the body, or in the emitted code.** Routing through an exit above one lane is not
merely unavailable but **wrong**, executed: at `a = [7,2]`, `b = [3,9]`, lane-wise gives `[7,9]`; reducing with
`all` then branching gives `[7,2]`; reducing with `any` then branching gives `[3,9]`. **Two candidate
reductions, two different wrong answers, neither of them the max of anything**, from safe code with no
diagnostic at any level. That is D16's own register arrived at from a different direction: **a wrong reduction
returns a wrong answer, not a slow one.** The two-lane result was independently re-executed and is `[7, 9]`,
which is the answer this section states and neither of the two wrong reductions it prices, so **the headline
claim is confirmed rather than repeated**.

**`select` is a fact about the pair**, the truth and the datum: it needs to know how many lanes the datum has
and how to blend them, which is the datum's structure and not the truth's. **So under the layer-keying rule it
lives on the datum, parameterised by the datum's own truth type.**

**The three-way the design wants.** The **algebra** lives on the truth type, is product-closed, and is what the
fifteen declarations bind on. The **selector** lives on the datum, is keyed on the pair, exists at every lane
count, and requires no choice. The **exit** lives on the truth types that have one, which is not all of them,
and is required only by sites that perform genuine control flow, which above one lane means sites that are
scalar anyway.

**The thunked selector is the exit, and the two must not share a name.** A selector on the *truth* type taking
thunks (`fn select<R>(self, on_true: impl FnOnce() -> R, on_false: impl FnOnce() -> R) -> R`) is
interdefinable with `is_true` in both directions, both total for any type at all, compiled. **A truth contract
carrying that selector carries an exit whether it says so or not, and inherits the reduction problem in full,
while looking like structure rather than policy, which is strictly worse than an exit that admits what it is.**
Implementing it for a mask has no correct body: one of the reductions gets chosen and the choice is invisible
from every call site. Two things sharing a name, one of which is the exit renamed and one of which is the
answer, is exactly the condition the widened definitional-completeness line exists to catch.

**"Never a default" is a bound for two routes and a rule for three.** Five introduction routes for the exit were
enumerated and **all five compile clean today**: a default body on the exit trait
(`fn is_true(self) -> bool { self == Self::TRUE }`, which looks like structure and **silently means
all-lanes**); a blanket impl over the algebra with the same body; an inherent method named `is_true` on the
mask while the trait bound stays unsatisfied (inherent methods win resolution, so concrete call sites read as if
the trait were implemented, and **only the reader is fooled**); `Deref` (live rather than hypothetical, because
D15 ratified `Deref` as the vehicle for predicate call syntax; **silently means lane 0**); and `From`/`Into`
(`impl From<Mask2> for bool` is legal because the mask is the local type, and `m.into()` at a `bool` position is
an exit; **silently means any-lane**). **The first and last disagree with each other about what the default is,
and both read as the obvious thing to write.** `impl !Branch for Mask2 {}` compiles under `negative_impls`
alone (the forbidden `with_negative_coherence` is not needed, checked by deleting the gate) and converts the
first two from silent successes into `E0751` **naming both sites**; the other three are closed by grep rather
than by the type system (**no `Deref` and no `From<_> for bool` on a truth type, and no inherent method sharing
the exit's name**). **Anyone writing "never a default" as though the type system carries the whole weight will
be wrong in three places**, and the value of saying it exactly is that the three rule-shaped routes are now
named and can be grepped rather than discovered later by a wrong answer. Completeness is not claimed: five is
what an enumeration of language mechanisms producing a value at a `bool` position found, and a sixth would not
surprise its author.

**The exit is not a new door.** Any truth type that is not `bool` must already declare a projection to it and
already pays one at every branch, in either branch of the fork. **The proposal adds nothing to the common case;
it names something already there and states who may have it.** `arvo-storage/src/platform.rs:323-333` already
declares `pub const trait AsBool { fn as_bool(&self) -> bool; }` with a single implementor, `Bool`, already
`#[inline(always)]`: **the exit, already declared, already restricted to the one-lane type.**

**Branch B does not remove `bool`; it removes the crate.** Naming `bool` in the foundation is exactly what
notko's own idiom already does at these positions (`Maybe::is`, `Outcome::is_ok`,
`Maybe::filter<P: FnOnce(&T) -> bool>`), which is its floor as the zero-dependency foundation. **A foundation
naming the language's own primitive is not the layering problem; a numeric contracts crate depending on a
wrapper crate is.** If op picks branch B believing it removes `bool` from the design, he will be surprised
later.

**The lock: branch B, bound on the algebra.** The consolidation-ready statement:

> **The truth contract is a Boolean algebra.** It declares `TRUE`, `FALSE`, `and`, `or`, `not`, and nothing
> else. Every truth type in the design satisfies it, and finite products of truth types satisfy it by the
> closure of a variety under direct products, so a lane mask is an instance of the contract rather than a
> container of instances. The contract carries no route to the language's `bool`.
>
> **The exit is declared separately, and it is partial over truth types.** An exit is the route from a truth
> value to Rust's `if`, which takes `bool` and cannot be overloaded. A one-lane truth type has exactly one exit
> and it is the identity, because the structure-preserving maps out of an n-lane truth algebra are exactly the
> n coordinate projections and at one lane there is one of them. A truth type of more than one lane has no
> exit. Its reductions are inherent operations named by the consumer at the call site, never a trait impl,
> never a default, because they are not structure-preserving maps and the foundation cannot know which one a
> call site means. The absence is declared rather than left to discipline, and a later blanket or default is
> then a coherence error rather than a silent success.
>
> **The declarations that return a truth value bind on the algebra, not on the exit.** All fifteen are
> producers; none of them branches on its own result. Binding them on the exit would refuse every multi-lane
> instance at the impl, which is the instance the contract's shape exists to admit. A consumer that performs
> control flow adds the exit to its own bound, at its own site, where the fact that it is branching is visible.
>
> **A consumer that appears to branch on a truth value usually wants a selector, and the selector is keyed on
> the pair.** It takes a truth value and two data of one type and returns that type, lane-wise, with both arms
> evaluated. It exists at every lane count, requires no choice, and lowers to a conditional select rather than
> a branch. It lives on the datum, parameterised by the datum's own truth type, because how to blend is the
> datum's structure. A selector on the *truth* type taking thunks is not this: it is interdefinable with the
> exit and carries the same partiality, and the two must not share a name.
>
> **Consequences worth stating in the same place.** `Bool`'s route to its primitive is this contract's declared
> exit, which settles as derived what was handed over as taste, and the four redundant spellings of it are four
> spellings of one door. The number of routes is one, per D27's own "named once".

**The three organs, declared**, because the statement above names three things and the worked `max` is written
against two of them, and all three had been used by name for the length of the section and declared nowhere, so
a reader following the section's own advice compiled nothing (`116:71-77`).

```rust
/// The algebra. Working name; the contract's own name is on the open list.
pub const trait Truth: Sized {
    const TRUE: Self;
    const FALSE: Self;
    fn and(self, rhs: Self) -> Self;
    fn or(self, rhs: Self) -> Self;
    fn not(self) -> Self;
    // and nothing else: no route to the language's bool.
}

/// The exit, partial over truth types: one lane has it and it is the identity,
/// more than one lane has none.
pub const trait Branch: [const] Truth { fn is_true(self) -> bool; }

/// The producer. Fifteen declarations have this shape and none branches on its
/// own result, which is why the bound here is the algebra and not the exit.
pub const trait Compare: Sized {
    type Truth: [const] Truth;
    fn lt(&self, rhs: &Self) -> Self::Truth;
    // the rest of the comparison surface has the same shape; this document
    // names only `lt`, so only `lt` is written down.
}

/// The selector, on the datum, keyed on the pair, both arms evaluated,
/// lane-wise, at every lane count, requiring no choice.
pub const trait Select: Sized {
    type Truth: [const] Truth;
    fn select(t: Self::Truth, on_true: Self, on_false: Self) -> Self;
}
```

**Every member above is read off the locked statement rather than chosen.** `Truth`'s five members are its
first sentence; the working name is the one this section already uses ("any exit a `Truth` impl for `Mask<W>`
supplies"), and the contract's real name stays where it was, on the open list. `Branch` is the name in the
section's own compiled diagnostic (`Mask2: Branch is not satisfied`), and `is_true` is the member the
five-introduction-route enumeration spells. `Compare`'s associated `Truth` and its `lt` are what the `max`
example projects and calls. `Select`'s three arguments and its associated type are the statement's fourth
paragraph, and the associated-type spelling rather than a type parameter is forced by `max`'s own
`Select<Truth = <T as Compare>::Truth>`.

**The supertrait direction is the safe one, and it is worth stating because the other one is a live trap.** The
edge runs from the exit up to the algebra, so a crate bounding on `Truth` never gains a route to `Branch`.
`117:355-366` compiles the reverse case and finds that **projection travels through a supertrait edge that
nameability does not**: a crate that cannot write `use lowering::Lowering` can still project `S::Layout` if
anything it bounds on has `Lowering` above it, so the `E0432` check reports closed while the design is open. The
same shape here would put the exit back on every producer.

**Two names in this section are not declarations and should not be read as ones.** `Mask<W>` is the design's
existing lane-mask family used as the worked instance at `W` lanes, not a type this section introduces; the
statement's content about it is the product construction, which holds of any finite product of truth types.
`TruthAlgebra<N>` belongs to the priced alternative immediately below and is not a member of the locked shape.
Both were read as undeclared design vocabulary on a cold pass (`116:97-99`), which is what an undifferentiated
backtick does.

**An alternative, priced rather than recommended.** An arity-indexed algebra (`TruthAlgebra<N>`) with the exit
blanketed at one lane compiles, refuses the mask with a diagnostic that says *why* rather than only *that*
(`E0599`, "doesn't satisfy `Mask2: Branch` or `Mask2: TruthAlgebra<1>`"), refuses a one-lane type's attempt to
override its derived exit (`E0119`, correct, since at one lane the exit is the unique homomorphism and there is
nothing to choose), costs the same single symbol, and survives the design's own `pub const trait` idiom with the
exit reached in const position. **Under this shape the exit at one lane is *derived* in D16's exact sense, so it
cannot lie and needs no contract**, at the price of threading the lane count through the fifteen declarations.
**A middle route, a marker trait meaning one lane, is strictly worse under D16**, because arity would then be
an *asserted* property owing an `unsafe impl`. If op wants the shorter declarations, the marker with
`unsafe impl` is the honest spelling; if he wants the shorter bound story, the index is.

**One shipped default body is corrected by this.** `ConstPartialEq::const_ne` at
`arvo-storage/src/bridges.rs:44-48` reads `Bool(!eq.0)`, reaching through the public field to negate. Under a
truth contract that body becomes `eq.not()` in the algebra, **which stops a contract's own default from
reaching through a representation.**

**D17 does not decide this fork.** D17 settles the parameterised spelling for the predicate family; the fork is
about the numeral contracts, a different family at a different layer. **Nothing proposed here requires reopening
a ratified decision.**

*Grounded on: ratified (D15, D16, D17, D18b, D27, all op; `108b:130-144` op's own confirmation of the corrected
truth-contract shape; `108b:163-170` the platform charter and the deferred name), settled shapes (`03:56-94`,
`07:330-345`, `84:386-393`, `arvo-bridge-home-rule.md`, `what-you-can-observe-is-what-you-guaranteed.md` its own
Boundary section), compiled (`103_probes/p1` through `p8`, `105_probes/`, `106_probes/p5`, `108_probes/p1`
through `p9`, the `max` build at one and two lanes), measured (22 against 34 instructions across the crate
boundary; 27 against 29 for the fresh-accumulator property; the merged-symbol results), external (Birkhoff on
varieties, closure of Boolean algebras under products, cited rather than compiled).*
---

## 2. The lead designer's calls

**Op's earlier ratifications, restated in full rather than named.** D69 (from `202607301100`) ratified:
identity is parameterised in mathematical coordinates, not encoding coordinates (`30b`). D39 (from
`202607291900`) held: membership through algebraic structure stays a decision pending a positive
characterisation of its honest content (`30b`). D38 (from `202607291900`) ratified: the `arvo-num-systems`
crate with the ten-member vocabulary, shipped even if nothing uses them, vocabulary fixed by mathematics
(`40:209-212`). The novelty posture (`34b`). Widening leaves `Lowering`, `Growth` leaves the law key, the
finest-view mechanism replaces the three-relation fork (`39b`). The value-unique encoding ratified in full,
division held, every claim grounded (`44b`). The four standing directives and the standard (section 0). **The
intent outranks every instruction, is vague on purpose, and only op's calls are final, and even those go
stale.**

**The eleven checkpoints before `30b`, whose content no consolidation carried until the restoration pass.**
`04b`: the three open threads, the checkpoint cadence, the licence to argue against a ratified call provided
the argument is made rather than asserted, and the standing instruction that op will say when the panel is
ready for synthesis (sections 0.2, 0.5, 5). `06b`: the downstream-evidence correction, that what a consumer
currently writes is evidence of what was absent when they wrote it (section 5). `08b`: the enforcement job, and
the fused-versus-split call reserved to op, now settled on two reads (section 1.25). `12b`: the fidelity axis
held open with the ten-axis completeness claim left attackable, and the `WideBits` hole (section 1.10, the
registry section). `13b`: the relocation refused, with the separate place inside arvo worth designing rather
than defaulting (section 5). `13c`: the standard, and the review's own mode and stopping condition (sections
0.1, 0.5). `16b`: design the shape, the existing code is irrelevant, arvo declares and the build side lowers
(section 0.2). `16c`: the spec is the subject, every member owes its boundary a design, and novel answers
outrank observations (section 0.2). `16d`: the spirit outranks all, the intent is vague on purpose, keep the
current shape where it serves (section 0.2). `17b`: fidelity grants are checked rather than asserted, and
partial associativity named (section 6, the fidelity entry). `24b`: every member owes a constructive
deliverable (section 0.2).

**`68b`**: the panel's scope is design, not source, and `mock/crates` is out of bounds until the canon is
complete; **the four-phase sequence after the canon** (section 0.5); the `arvo-strategy` migration
authorization withdrawn; `Int` dropped, exponent bounds as types, `Radix` sealed, `Specials` as a product all
confirmed; the layer-keying rule, the transfer-ground scheme and the `TotalOrd` split confirmed; the
strategy-door table's justification named as a regression.

**`70b`**: both preset tables ratified in full (section 1.21); the `tree-fact` and `tree-meaning` split
adopted, both halves, with the mechanical deletion test as the standing check; one cell left open with op's own
instinct attached, closed at `74b`; **and op's statement of the end state**, a full spec that is proven, valid,
efficient and ergonomic, invisible for the most part to downstream consumers while doing real work underneath
and lowering transparently to optimal instructions (section 0.5).

**`74b`**: the far-point rule ratified as one statement covering three instances; one sealed bottom carrier
crate adopted for capacity with `Capacity` kept as a named semantic alias over it, on op's condition that "the
mechanism unifies and the vocabulary does not"; `Layout::Bitpacked`'s ambiguity sent to a compute-side expert.

**`77b`**: the pricing pillar named as the fourth design rule, in op's own words, with the guard clause
carried; `Layout::Bitpacked` ratified as one meaning; the facade fork ratified closed to route Z on soundness
before cost; the capacity unification's naive form corrected and op's reframing of what capacity denotes set as
the coming stretch's first question.

**`79b`**, recorded rather than dispatched, binding the implementation phase: the parity-suite mandate
(namesake aliases as an intent pillar, differential tests against the namesake's own implementation) and the
exhaustiveness mandate (both directions: what should hold must pass, what should be unrepresentable must fail
to compile, red is the starting state). **No member is briefed to design against it**, and `79b:64-69` states
why that is correct rather than a gap: the mandate **binds the last two phases of op's four-phase sequence**
(the stubs round and the per-piece implementation rounds), not this one, and it is recorded early "so it is not
rediscovered late or watered down when the volume becomes apparent" (section 0.5).

**`82b`**: the bitpack price corrected to 1.50x and 1.29x; three items opened and since closed; **the
owed-list discipline adopted** (an owed item names the artifact whose existence would close it).

**`86b`**: the separation requirement adopted as a requirement on the review's own models; two calls held and
since resolved.

**`90b`, `95b`, `101b`, `106b`, persona-decided.** Op walked `90b`, `95b`, `101b` and `106b` **individually
rather than confirming them as a block** at `108b:6-8`. What survives that walk is recorded in each section
above; the two calls that died are on the droplist and the reversals list. `90b`'s division instinct is dead on
its own stress test. Everything else across the four stands **only until op says otherwise**, and
`108b:190-191` restates that only op's calls are final and that they go stale when their evidence moves.

**`108b`, op's own, the most recent ratification in the record, and the widest since `77b`.** In order: the two
standing principles (section 0.3, and the archive's instrumentation defect, adopted in all three parts).
Capacity is-a stands, with two corrections riding (section 1.27). The counting split confirmed after seeing
both readings (section 1.14). Division's three solution-set clauses confirmed, with `Hot`'s cell going to the
consumer and the lowering-authorship residue adopted (section 1.13). The elementary-functions chapter confirmed
with the boundary stated as the family's own and `exhaustively-computed-or-cited` entering the registry
(section 1.16). The perimeter block confirmed whole (section 1.12). The naming principle and the
assumption-never-a-witness sentence both confirmed (section 1.26). The digest chapter confirmed and the
footprint bench kept preliminary with the contention regime named as its next target (section 1.22). The truth
contract confirmed and corrected, with the persona's third clause found backwards and the exit moved to the
operation that branches (section 1.30). The bitfield confirmed, the convergent statement entering the ratifying
text, overlap refused unless declared, the containment locus moved to an emitted free const item (section
1.29). The platform crate's charter confirmed, **its name deferred to the taxonomy round**, where the collision
with the principles document's own "platform" is recorded and decided rather than here (`108b:163-170`). The
three tautological tests **collected, not acted on** (section 4). And the order of work: **consolidation
eleven, then close the queue**; the remaining stretch works the open list down rather than opening ground
(`108b:184-186`).

**Standing** (`108b:188-193`): the panel produces canon, not source; `mock/research/` and `mock/benches/` are
its ground and **`mock/crates` is out of bounds until the canon is complete and earmarked as arvo's first full
canon**. Fable is unavailable; dispatches run at Opus until it returns.

### Loudest for op's read, ordered by op's own stated priority (`86b`: "the open list, not the interesting list, is the queue")

Each entry states the question and its current disposition together. Where two independent members have
already agreed, the entry says so and what remains is a confirmation rather than a choice.

1. **The array grammar's forcing argument** (section 1.27). Two independent compiles show the pairing is
   forced neither by the language nor by the inductive numeral alone, and a third column exists whose record
   the ninth consolidation dropped without a droplist entry. Is-a stands and is not in question; the question
   is whether the storage is paired or derived, and it is one line either way plus a decision about whether
   `[T; N]` by name is needed at a generic capacity. **Status: one pass on the design question, three columns
   priced, no second design read.**
2. **The truth-contract fork's lock**, branch B bound on the algebra (section 1.30). The corrected shape is
   `108b`'s own; what remains is the reduction spelling and the `negative_impls` adoption, both op's.
3. **The bound on `S`, and the exponent-form key it now carries** (sections 1.1 and 1.21). **Two agreements on
   each half.** `S: Policy + Lowering` was written by one read and given its reason by a second, independently:
   the conjunction is what makes the weaker bound `S: Policy` enforceable, so the bound and section 1.25's
   supertrait prohibition are one decision seen twice. The **key** was reached by two further members
   independently, each compiling before opening the other, and **both offered alternatives are refuted by
   compiled diagnostics**: keying on the numeral admits two same-kind numerals disagreeing on one preset, and
   per-kind markers admit `Number<Binary32, WarmFixed>` at exit 0 while deleting the four types D72 names.
   **What op owes is confirmation of the survivor, not a pick between two.**
4. **The contract split, fused or split, reserved at `08b:47-51`** (section 1.25). **Two agreements**: files
   117 and 120 read it independently, the second compiling its answer before opening the first, and both
   recommend keeping the split unchanged. **What is owed is a one-line ratification of the incumbent shape**,
   and nothing that ships moves either way. File 09's `LogicalNumber` shape stays available and unrecommended,
   because the residual gap is invariant to every mechanism that leaves the owning crate authoring the
   attachment.
5. **Whether the nested `Numeral` shape supersedes D68 (from `202607301100`)'s ratified four flat members**
   (section 1.2). Two op calls, contrary shapes, with the nesting's two grounds both postdating D68 and the
   supersession now stated rather than silent. **Op's word on the supersession.**
6. **The decision register's colliding identifiers, and the question grid's own marker** (section 0.4). The
   round-qualified citation form is adopted here, is op's own recovered practice in three places in his frozen
   text, and touches no committed text, so what is owed on that half is confirmation. **The genuinely open half
   is the grid**: its `D1` through `D3` are the fourth letter-group of an `A1`-through-`E5` question grid, not
   decisions, and a citation to one should say so. **One word, and it is op's**, because the panel naming a
   marker inside op's register is the thing this whole item exists to prevent.
7. **`W_MAX`, the admitted range for a written width** (section 1.24 and below). **One number, priced rather
   than argued**, and the two halves that looked open are settled.
8. **The platform crate's name**, deferred to the taxonomy round at `108b:168-170`, and with it the token
   collision's remedy (section 1.26).
9. **The three one-token-two-meanings naming calls, answerable in one sitting**: `Exponent` the trait against
   `ExponentForm` the bound (section 1.2); `Dec` against `PosPred`, one construction with two spellings
   (section 1.27); and `PrecisionOf` as the bridge's result-type spelling (section 1.23). **The third is
   narrowed rather than open**: the branch that renames the marker trait dies on `74b`'s ratified alias family
   under `16d:14-15`'s tiebreaker, so the marker keeps `Precision`, the result types take their own names, and
   what is left is the word itself, inside D56's ratified naming rule rather than beside it.
10. **Which of `Bool`'s four redundant spellings survives** (section 1.30). A vocabulary decision, and the
    ground for it moved from soundness to vocabulary, which if anything makes it more op's.
11. **The rotation and curve benches, D41 (from `202607291910`) and D11 (from `202607281220`)'s own**, now with
    named requirements and a named floor on the bench matrix's own rank of at least 4 (section 1.28).
12. **Division's grading axis**, held since checkpoint ten, untouched by the dissolution of the fork it sat
    beside.
13. **`Hot`'s default float environment**, held four checkpoints running.
14. **The uniform-sampling spec question in `arvo-pseudorand`**, the one periphery row that is genuinely open
    rather than merely unread.
15. **The workspace perimeter rule's own provable-versus-trusted clause**, op's hand, lean toward yes.
16. **The eleven-crate taxonomy's remaining per-row confirmations** (section 1.25), with three rows
    content-reviewed.
17. **`FromConstant`'s breaking-change fix**, vehicle held for its own second reads.
18. **The four `unstable-features.md` rule-wording edits**, awaiting op's own wording: the last-sentence
    correction (necessary promoted to sufficient, refuted by two compiled counterexamples, both now stated at
    section 1.5), the third-way clause (container-class const-tag dispatch, shipped, permitted, missing from
    the enumeration), the step-budget clause (the wall is a total-step-count budget, not a bit-width ceiling),
    and the wall-clock figure (`63:456-460`: the quadrupling and rustc's refusal at nine bits are structural
    and machine-independent, while **"only the specific wall-clock figure (28.45 seconds) is one machine's
    measurement through one harness; the proposal to op is to mark it as such wherever quoted"**, the figure
    occurring in twelve panel files and in no consolidation after the sixth while the workspace rule quotes it
    today as a durable fact, `111:349-355`). **The ban itself is untouched by all four**, and this is the
    largest single item on the list.
19. **Whether any transcendental ships in the first contracts crate**, a packaging call named and priced by
    section 1.16 rather than decided by it.
20. **The digest chapter's type-level history split**, still a lead deserving its own dispatch, still not
    built.
21. **The membership uniqueness scoping (D39) and the seven upper vocabulary members' reading** (section 1.6),
    both genuinely op's, with two symmetric readings on record.
22. **The preset-divergence mechanism**, working, probe-verified, unstable-feature-free, noted at op's seventh
    checkpoint as available and explicitly not adopted with the instruction that a later member should take it
    further (section 1.21). Off the record since the fourth consolidation.
23. **The checkpoint cadence op set at `04b:42-43`** (every two experts) against the two later restatements at
    `77b:101-105` and `86b:50-53` (four, then four-then-consolidate). **One line saying which is current**,
    since the record cannot show whether the drift was op's or the dispatcher's (section 0.5).
24. **The round's own seventeen open rows have never been reconciled with this list.** The talk file carries
    "**Status:** open" seventeen times against four settled, including A1 (the scale), A2 (the format
    decomposition), B1 (the composition type), B3 (what a preset is mechanically), C2, C3, C4, C6, E1 and E4.
    Two of them reach this document only because a different route rediscovered them, cited to file 11 rather
    than to the round (`113:443-448`). **One pass reconciling the two lists, which is a dispatch rather than an
    edit and belongs before the taxonomy round.**
25. **The three tautological tests**, now a checklist for the implementation phase rather than a report
    (section 4).
26. **Whether construction one stays in the spec as a recorded fallback** (`78:872-873`, item 12), the item the
    ninth consolidation dropped, which section 1.27 shows is not a fallback but the third column of a live
    fork.
27. **Where `Folded`'s witness rides**, as a returned value beside the fold's result or as a parameter on the
    result numeral (section 1.14). One line; the count itself is settled and compiled.
28. **Where the container projection lives**, as `Lowering::Container` or in a one-member carrier contract
    below `Lowering` (sections 1.1 and 1.25). `Number`'s declaration is unchanged either way, and the spelling
    rule is the load-bearing half.
29. **Whether the `TotalOrd` split is two traits or one trait with two methods** (section 1.20), stated open in
    prose since the sixth consolidation and never on a list until now.

#### The width ceiling, settled to one number

**A ceiling on each written width already exists and is forced rather than chosen.** The precision bridge is an
enumeration because an impl is the only case split Rust has over a const parameter and every other route is
compiled shut (section 1.23). **An enumeration has a largest row**, so "no ceiling" is unavailable on that axis
under any wording. The bound above it is the const parameter's own type, enforced by rustc's deny-by-default
`overflowing_literals` at exactly the wrap that would otherwise land on an admitted row and compile
(`error: literal out of range for u16`). `Pos`'s two ceilings are ceilings on a magnitude, so a width would have
to exceed `2^64` to meet either, and the full `u16` table compiles with every row present.

**Nothing bounds the sum, and nothing should, on three compiled grounds.** The sum is produced by the tower's
own addition and never touches the table, so `UFixed<40, 30>` compiles today at a total of 70 with nothing
anywhere admitting 70, and `<PrecisionOf<63, 63> as Nat>::VAL == 126` at exit 0. A marker enforcing a second
ceiling on the total is refuted three ways. **It refuses a legitimate product**: `mulnum` sums the operand
precisions, so a 20-by-20 fixed-point value squared has precision 80 and fails the marker. **It cannot be
narrowed to the written site to dodge that**, because a `where` clause on a type alias is refused outright
(`error: where clauses are not allowed after the type for type aliases`) and its repair is `lazy_type_alias`,
which appears on no list in `unstable-features.md` and is therefore unvetted and unshippable by that rule's own
terms; hanging the bound on a struct instead changes the public spelling and breaks D48 and D31. **And its
refusal is illegible** where the per-width refusal is not: a consumer who writes `UFixed<40, 30>` is shown
eleven numeral trees and no number, against section 0.5's consumer-facing bar, where a width outside the table
refuses with every token reading as a number (`a written width must lie in 0 ..= 63`). **So the marker is not
one extra line per row, it is one line per row plus a new type in the design, and it buys a worse diagnostic
than the design already has.**

**Nothing else forces one either, and this connects two open items.** The only mechanism that would have made
the sum need a row is a per-width container menu, which section 3's entry 6 already rejects on its own
measurement. A carrier is reachable from any numeral by five impls, gate-free, with `size_of` checked at widths
the table does not hold, so totality over twice the table's range is available. **The container fix op withheld
at `68b` on scope grounds is the same fix that keeps the sum unbounded**; were the per-width menu ever restored,
the sum ceiling returns with it and not as a choice.

**What is left is one number, and it is priced rather than argued.** The table costs 0.13 s at 1024 rows, 0.94 s
at 4096, 11.78 s at 16384, and 279.62 s at the whole `u16` range (1.30 GB peak), paid once by the crate that
declares the carrier, which the pricing pillar licenses in op's own words. **The number that does argue is
downstream**: linking the table costs a consumer nothing at any size, and using it costs about 1.25 ms per
distinct composition to 4096 rows and about 2.8 ms at 16384, against section 1.24's dyadic row at 2.1 ms. **The
cost is roughly quadratic past 4096**, each doubling costing between 3.3x and 5.6x, the same character the
per-width container already measured, so a linear reading of the first three rows does not survive the
extension. **The operating point the surcharge measurement names is a few thousand rather than a few tens of
thousands.**

**The paragraph the canon carries, at whatever `W_MAX` op sets** (4095 below, because that is the operating
point the measurement names):

> **Widths.** A written width lies in `0 ..= 4095`. A total width is the sum of two written widths and so lies
> in `0 ..= 8190`; it is not separately bounded, and every mechanism keyed on a total width is total over that
> range rather than over the table's own. Both numbers come from one place. The bridge from a written width to
> a type-level numeral is a table with one row per admitted width, and it is a table because an impl is the
> only case split Rust has over a const parameter and every other route needs a feature arvo forbids. The
> range is therefore the size of an enumeration rather than a judgement about what a workload should need, and
> it is set where the table stops being free rather than where a consumer stops being reasonable: 0.94 s to
> build, once, in the crate that declares the carrier, nothing at all for a downstream crate to link, and
> about 1.25 ms per distinct composition to use, against 2.1 ms for the composition itself. Nothing downstream
> sees the table: no per-width symbol, no runtime table, no relocation. A width outside the range is refused
> where it is written, naming the width and the range.

That paragraph is compatible with `arvo-toolbox-not-policer.md`'s "No bit-width cap below the largest container
the substrate is willing to dispatch through", because the cap it states is not below anything: it is the
dispatch's own extent, and the sentence says so rather than dressing a guess as a fact about the layer. **One
item is created rather than closed, and it is small**: under this shape the design should say, once, that the
tower's addition is total and correct over the whole reachable range, because the shape leans on it where the
table does not reach. That is one law over 21 impls rather than a per-row obligation, and it belongs with the
tower's own arithmetic rather than with the bridge. It is on section 5.

---

## 3. The live-defect registry

For defects in the shipped tree, as against findings about the still-unbuilt design. **All nine entries are
stated in full here rather than by number.** The tenth consolidation carried entries 1 through 7 as a two-hop
pointer (`102:1019-1020` points at `91:970`, which points at the eighth consolidation, `109:211-213`); that
chain ended at the eleventh.

**1. `upward_rank` and `bin_pack` silently return wrong orderings under both shipped presets.** Tree:
`arvo-graph/src/rank.rs:34-88`, `arvo-comb/src/binpack.rs:44-63`. Both return a fold-shaped computation in
their operand numeral, claiming an exactness they do not have. On a compiled four-node chain `Hot` inverts a
longer path's ranking against a shorter one; `Precise` degrades to a tie. The consumer-pressure framing was
corrected at the sixth consolidation (hilavitkutin's own shipped chain calls neither function); the defect
stands on its own correctness regardless of who calls it. The fix is designed (section 1.20) and not shipped.
Grounded `tree`, `pin`.

**2. `FromConstant` accepts an unrepresentable constant and silently produces a wrong bit pattern, or panics.**
Tree: `arvo/src/traits/from_constant.rs:40`, `arvo-numeric-contracts/src/lib.rs:85-88`. A partial map declared
total. Vehicle still held for its second reads.

**3. `arvo-graph/tests/rank.rs` never enters the breaking path.** Setup that helps, not a fabricated pass:
single-digit weights against a `u8` container.

**4. `arvo-spectral`'s ten test files never exercise an arvo numeral.** All run against a test-local newtype
over bare `f32`; `FastFloat` compiles clean against the same bounds.

**5. `mock/benches/src/main.rs` could not run any bench at all. Fixed**, and the fix stands.

**6. `arvo-strategy`'s shipped container dispatch, and its facade, are load-bearing on the forbidden
`generic_const_exprs` feature.** Tree: `arvo-strategy/src/lib.rs:11`, `arvo-strategy/src/container.rs:254-258`,
`arvo/src/lib.rs:25`, `arvo/src/ufixed.rs:35-36`. **The `arvo-strategy` half's structural-derivation fix is
measured, correct, and compile-neutral** (sixteen diagnostic spans collapse to one mechanism; thirty impls,
linear in what is instantiated and zero for what is not, checked against `tag_hot_cold`'s own body at every one
of 512 widths with a live negative control; `cargo check` clean at 19.6 s; identical test count; compile time
neutral at a three-run average; zero public signature changes; the 34 files across seven crates that reference
`BitsContainerFor` untouched), **but `67b`'s authorization to land it was withdrawn by op at `68b`, on scope
grounds rather than measurement grounds.** One condition attaches to the shape and is still unmet and still
owed: **the structural form must preserve the `#[diagnostic::on_unimplemented]` refusal-at-unserved-widths
story the current table gives for free, and if it cannot, the shape question returns rather than shipping a
worse diagnostic** (`68:637-640`, dropped at `78:701-708` which declares the measurement standing while
dropping the unmet condition, `109:428-430`). **The per-width alternative is quadratic in its ceiling: 0.42 s
at 256 widths, 5.3 s at 1024, 116 s at 4096, and past 25 minutes at 8192, paid by every build of every consumer
forever**, which is the whole reason the structural form is the design, and which was dropped in the document
that names the pricing pillar (`109:471-473`). That measurement is also what keeps section 2's width sum
unbounded. **The facade half's fork is closed to route Z**, measured at 16 ms added to a 6.35 s ± 0.09
whole-workspace check (0.25%, one sixth of the build's own noise), linear to 800 declarations, magnitude-free
to sixty-two bits, closing on route Y's compiled failure to express its own guarantee under `cargo check`
before the cost was consulted. **The facade must land atomically**, because `UFixed`'s parameter meaning
changes and no shape lets the old and new spellings coexist (`68:696-701`, dropped at `78:710-738`,
`109:430-431`); `arvo-strategy` needs no coexistence mechanism, lands alone, first, cheap, reversible, and
blocks nothing. **Nothing in this entry authorizes execution against `mock/crates`.** Grounded `tree`, `pin`,
`flags`.

**7. `arvo-spectral`'s degenerate-component classification is decided by NaN payload arithmetic rather than by
anything the design calls a value.** Tree: `arvo-spectral/src/partition.rs:59,156,181`. Two NaN bit patterns
differing only in their sign bit classify into opposite partition classes under the shipped order and the same
class under the value-level reading, compiled. It fires with no fold in the path at all, and is the
layer-keying rule's third instance, joined by the digest as a fourth at a different layer. Grounded `tree`,
`pin`.

**8. A bitfield overlap declaration compiles clean and one setter silently truncates its neighbour.** Tree:
`arvo/src/bitfield.rs:28-30` (the documented deferral), the containment assertion at `:377` mentioned only at
`:393` and `:399`, and the `ConstDefault` impl at `:370-374` that constructs without mentioning it. Compiled
and executed against the shipped macro: `Overlap: 16 { a: 8 at 0, b: 8 at 4 }`, `a` set to `0xFF` reads back
`0x0F`. **Safe code, no `unsafe`, no warning, no diagnostic at any level.** The repair is section 1.29's,
ratified at `108b:155-159`; **no test in the tree currently enters the path in either direction**, and two
tests are owed. Grounded `tree`, `pin`.

**9. `BitMatrix`'s two axes of one rank-2 shape are declared in two vocabularies, and a safe write past the
declared column extent lands.** Tree: the row extent is a `Capacity`, checked and refused out of range; the
column extent is only a `Bits<N>` width, with no shape behind it and no refusal. Two values equal at every
index the shape declares end up with different byte images, reaching the mutation gap's signature through an
ordinary safe call at rank 2, with no unsafe and no niche. Compiled and executed. **Offered as why-evidence for
the shape chapter's own need for a per-axis extent with a refusal behind it, not as a fix to propose**,
`mock/crates` being outside scope. Grounded `tree`, `pin`.

---

## 4. The implementation-phase tautology checklist

**Op's ruling, `108b:172-181`, adopted as the disposition rather than as a report.** Op:

> Keep collecting them in a list, so when we start impl, we have a clear checklist for tests to delete and only
> rewrite fresh if the same test feels relevant in the new shape. Keep collecting, don't act on them yet.

**So they are a checklist for the implementation phase, carried in the consolidation, and the panel keeps
adding to it rather than re-reporting each one as news. Deletion is the default when the phase opens; a fresh
rewrite happens only where the same test is still relevant in the new shape.**

The list, with the mechanism of each rather than its name, so a reader can check the entry rather than trust
it:

| Entry | The tautology |
|---|---|
| `arvo-tensor/tests/capacity.rs:14-18` | asserts `<Dim<3> as Capacity>::CAP == cap(3)` against an impl whose body is literally `const CAP: Cap = cap(N)` at `src/capacity.rs:48`. True by substitution. |
| `arvo-tensor/tests/const_capacity.rs:49-53` | the identical shape against the second impl at `src/capacity.rs:117`. |
| `arvo-hash/tests/aliases.rs:16-23` | `content_hash_roundtrip` **contains no round trip**: its body constructs the same value twice from the same literal and asserts the two compare equal, which tests reflexivity of `PartialEq` on a `Copy` type and nothing about hashing. |

**Two further test-suite findings that are not tautologies and belong on the same checklist**, because they are
the "setup that helps" shape rather than the fabricated-pass shape, and because in both cases the path that
breaks is the path the design's own finding turns on:

| Entry | The gap |
|---|---|
| `arvo-comb/tests/greedy.rs` | five tests, all real, all supplying the same predicate against items that always satisfy it at a fresh accumulator, so **the guard at `arvo-comb/src/greedy.rs:60-64`, the one path D15's property exists to delete, is entered by none of them.** Two tests owed, in both directions, before any property is allowed to delete that branch. |
| the four `bitfield!` declarations in the suite | **every one is pairwise disjoint, and nothing in the macro requires that**, so the overlap path is entered by none. Two tests owed, one declaring an overlap and asserting the refusal, one asserting a stated overlap behaves as stated. |

**And one gate finding that is larger than any of the above and is recorded here so the implementation phase
does not inherit it silently: no Boolean-algebra law is asserted anywhere in the tree.**
`arvo-bitmask/tests/mask_ops.rs` holds eleven tests, all per-operation examples at hand-picked bit positions,
named `union_ors_bits`, `intersection_ands_bits`, `complement_flips_all_bits`, `empty_union_is_self`,
`full_difference_is_empty`. **Not one asserts an equation.** There is no De Morgan, no distributivity, no
absorption, no double complement, no idempotence, on `Mask` or on `Bool` or anywhere else, and the only grep
hits are two test names containing the word `complement`. **The truth-contract fork's own ground (Boolean
algebras form a variety, varieties are closed under products) is a theorem about a class, and membership of the
design's own two candidates in that class is asserted nowhere**, so the argument's minor premise is the
unchecked one and 672 green tests say nothing about it. The closing artifact is the law suite: **the five
axioms plus De Morgan, absorption, double complement and idempotence, asserted over every truth type the design
ships and at every width, not a sample of them, since a law checked at a chosen subset of widths is a choice
about what not to find out.** It lives under `mock/crates` and is therefore op's own commit.

**The count, so a future reader can see the shape rather than the number.** The first entry has now been
carried forward by more than thirty consecutive files. **The tenth consolidation kept a tautological-test item
on its open list and lost, in the same stretch of documents, a load-bearing item addressed to op**, which
section 8 records.
---

## 5. What is open

**Every item names a closing artifact, per `82b`.** An item leaves this list when it is answered or explicitly
droplisted, **never by attrition**, which is the failure the audit found across three consolidation pairs
(`109:487-501`).

### Closed, named once so nobody re-opens them

The platform crate's content review, the predicate concept's content review, and `arvo-bitfield`'s content
review, all three of the periphery's remaining content-unread rows. The truth-contract fork, locked to branch B
bound on the algebra, with the persona's third clause reversed. The bitfield's classification, its two
obligations, and its digest correction. The `Adjustment`-entry-point and cross-call-site-face-identity
residuals, both closed in a form the record had lost. The composite-of-two-placements category. Whether a field
of a bitpacked column of bitfields has a byte image (no, and only the column does). D17's placeability. The
rung of D15's one property (rung 2, by compiler refusal). Whether `Bool` carries a guarantee (no, compiled
exhaustively). Whether the mechanism sentence at `55:163-165` is new (it is not; section 8). Whether both
halves of the capacity toolchain claim still hold on the pin (they do). The disposition of the level-naming
clause (absorbed at the ninth, dropped at the tenth; section 8). **The contract split** (three contracts, two
parameters, unchanged, two agreements; section 1.25). **The bound on `S`** (`S: Policy + Lowering`, two
agreements; section 1.1). **How a preset carries two rows** (both contracts keyed on the exponent form, two
agreements, both alternatives refuted by compiled diagnostics; section 1.21). **The `Precision` collision's
branch** (the marker trait keeps the name, the bridge's result types take their own; section 1.23). **Where the
bridge's table is emitted** (the crate that declares the sealed carrier, by `macro_rules!`, complete, with a
per-row assertion; section 1.23). **Whether a second ceiling on the total width is needed** (no, on three
compiled grounds; section 2). **Whether the eight Rust blocks in the standing base assemble** (they do, exit 0,
with three additions the document now states at the first block that needs them; section 1.23).

### Owed, each with its closing artifact

- **The Boolean-algebra law suite.** The largest owed item, and the truth-contract fork's own unchecked
  premise. **Artifact:** the five axioms plus De Morgan, absorption, double complement and idempotence,
  asserted over every truth type at every width. `mock/crates`, op's own commit.
- **The two bitfield overlap tests**, one declaring an overlap and asserting the refusal, one asserting a
  stated overlap behaves as stated. **Artifact:** the two tests. `mock/crates`, op's boundary.
- **The two `arvo-comb` greedy tests** for the fresh-accumulator guard, in both directions, **owed before any
  property is allowed to delete that branch. Artifact:** the two tests, in the existing file.
- **The capacity fork's cost column.** **Artifact:** one dispatch establishing whether any consumer needs
  `[T; N]` by name at a generic capacity, and whether the derived-storage column's loss of the named array type
  costs anything real. This decides section 1.27 and is deliberately not guessed.
- **The `AGREES` repair, both halves**, if the fork lands on the paired column; moot on the other two.
  **Artifact:** the assertion on `Capacity` itself plus the second reference inside `COUNT`'s own definition.
- **A second read on section 1.27's statement of what a capacity is.** **Artifact:** a read pointed at the
  layer-keying reading, which is where the argument is least sure.
- **A second read on section 1.29's four-cell classification.** **Artifact:** a read attacking whether "derived
  versus declared" is a real axis given that `Layout::Dense`'s stride is also derived; section 1.29 gives the
  mechanical answer (which forcing mechanism exists) and it is one pass.
- **A second read on section 1.30's discriminator** (route multiplicity is a defect only relative to a
  guarantee). **Artifact:** a read attacking whether a *naming* guarantee counts as a guarantee for the
  purpose, which would put `Bool` back in the group by a different door.
- **A second read on section 1.30's selector.** **Artifact:** a read attacking whether a selector keyed on the
  pair can be declared without the datum's crate depending on the truth's, which is the same layering question
  the fork itself is about, one level over.
- **The selector's exact shape at the design's real widths.** **Artifact:** one compile against a real `Mask64`
  and a real bitpacked column, checking the blend is expressible without the datum reaching into the truth's
  representation.
- **The `negative_impls` adoption**, if the declared absence of a mask's exit is wanted. **Artifact:** op's
  word, since it adds a WATCH-tier gate to a contracts crate. The fallback is a compile-fail pin, which the
  design already uses for the domain-preservation equation.
- **The shape trait split (D4 (from `202607281220`) against D43 (from `202607292100`)) and the `AGREES`
  repair**, both adopted as working shapes with one read. **Artifact:** a second independent read, per the
  named attack surface (whether widening `AGREES` onto `Capacity` itself over-obligates a capacity with no
  literal to pair with).
- **The occupancy-mask repair at a real `Numeral`-bearing model.** **Artifact:** one compile against the same
  `Specials`-bearing instance the float-division compile already owes.
- **The domain-preservation equation**, retired to a compile-fail pin rather than kept as spec text.
  **Artifact:** the pin itself, when the real `NonZeroCarrier` vocabulary lands.
- **The signed halves of file 43's division probes 2, 4, and 5.** **Artifact:** a signed re-run of each.
- **The float-division compile against a real `Specials`-bearing `Numeral` instance**, serving both division's
  owed path and the elementary functions' `ln`. **Artifact:** the single compile.
- **`foldnum` compiled against the real four-member `Numeral` contract with `Exponent` held fixed.**
  **Artifact:** the compile, unperformed since file 78 first named it, with its expected result stated
  (sufficient always, tight exactly for power-of-two arities and wide precisions, loose by at most one bit
  elsewhere).
- **The exact `foldnum` closed form, `1 + floor(log2(A * (2^p - 1)))`, built as a type-level computation.**
  Dropped from the verification bundle at `78:942-949` while its sibling survived (`109:475-478`).
- **Statement 0 against `quantize` and `roundToIntegralExact`**, the two operations carved out as
  datum-dependent by definition. Flagged forward by four files and performed by none. **Artifact:** the check.
- **The non-default `Canonical` compile.** **Artifact:** a declaration selecting the non-default cohort rule.
- **The constructive-extensibility compile** (a foreign crate minting a numeral and a capacity over the shared
  vocabulary without minting vocabulary itself). **Artifact:** the compile, named owed by five files now.
- **The nine-bit `u16`-class companion model for the container-class transfer coordinate**, at `2^18` pairs.
  **Artifact:** the model. Named owed by four consolidations; **what it is for is stated at the registry
  section above**, having been dropped at `78:946` (`109:469-471`).
- **The `x86` and RISC-V forms of the verdict-split receipt.** **Artifact:** compiled probes parallel to the
  aarch64 form.
- **The required-field relation for the environment receipt.** **Artifact:** a derivation per numeral axis,
  compiled at a binary16 `Underflow::Gradual` witness.
- **The `usize`-width refusal firing on a real 16-bit target.** **Artifact:** one cross-compile, once a small
  target is installed; both attempts so far returned `E0463: can't find crate for core`.
- **The notko-to-notko-hlist dependency edge D18b (from `202607290500`)'s split implies.** **Artifact:** one
  compile with the list in its own crate.
- **The IEEE primary-source reads**, one bundle: clause 7 and 7.6 on `invalid`'s raising condition; §4.3.1's
  overflow-tie sentence; §5.12's inexact-conversion signalling, confirmed genuinely distinct from the §5.2
  citation the review already has (flattened to "IEEE clause 5" at `102:1046`, `109:524-525`, carried here in
  its distinct form); clauses 5, 7.3 and 9.2 for the elementary functions; and the `Specials`-as-product check
  on the E4M3 exponent figure against the specification rather than vendor documentation, pending since `68b`.
  **Artifact:** verbatim, position-cited quotations.
- **The ISA bundle** (ARM DDI 0487, Intel SDM, RISC-V unprivileged ISA, LLVM LangRef). **Artifact:** the same
  shape of quotation.
- **The OCP mode-split facts** behind the declined NaN-on-overflow ground 4.
- **The `Crosses` mechanism's own second read**, now carrying three conditions on top of the first member's
  shape.
- **The `Cold` footprint bench's two remaining regimes**: past 33,554,432 elements, and **concurrent
  multi-column bandwidth contention, which `108b:126-128` names as the measurement `Cold`'s intent is actually
  waiting on and as the bench's next target rather than an open note.** **Artifact:** two further bench runs.
- **The bench harness's overwrite defect.** Its artifact was stated as "a per-section filter, or run artifacts
  landing beside rather than over committed ones, plus a by-reference input path"; the by-reference half was
  retracted as never having existed, **and the other half left the list without being closed**
  (`109:520-523`). **Artifact:** the mockspace-side fix, per-section filtering or beside-not-over artifacts.
- **The saturating-reduction lane kernels** (`uqadd` on aarch64, `paddus*` on x86), a real, currently unpriced
  cost that lands on arvo rather than on any build layer. **Artifact:** the kernels, and until then the codegen
  regression test that pins the fact.
- **The multi-limb carry-chain codegen test**, one test to make the optimiser-heuristic dependency falsifiable.
- **The round-to-odd validity bound `W >= F + 2`**, which must ship as a checked const bound alongside the
  credit given to round-to-odd. **Artifact:** the bound.
- **Whether every plausible `Canonicalisation` instance stays branchless**, unmeasured since the third
  consolidation.
- **The codegen-flag audit.** Narrowed from five named files' unswept instruction-count claims to one unrun
  reproduction at `63:870-872` (`109:383-384`); the five files are 24, 27, 43, 50 and 51, and the rule that
  narrows the work is that a claim about one exported function compiled alone is very unlikely to be sensitive
  while a claim comparing two or more functions in one compilation unit needs the sweep. **Artifact:** the
  sweep.
- **The `10^20` figure**, still open, plausibly a separate fact about a radix-ten accumulation pattern (section
  1.18). Returned zero hits in any panel file after `78`. **Artifact:** one bisection.
- **The `u64` readout ceiling at `2^63` on `Pos::VAL`**: whether to widen to `u128`, a multi-limb readout, or a
  comparison-only interface. **Artifact:** op's pick.
- **The tower's addition stated total and correct over the whole reachable range**, one law over 21 impls,
  owed because section 2's width shape leans on it where the bridge table does not reach. **Artifact:** the
  statement, with the tower's own arithmetic rather than with the bridge.
- **The reciprocal-table strength reduction** for the radix-ten quantiser's dominant division term, an attempt
  rather than a build, and the same shape of win the notation's staging discipline names.
- **The `InfOnly` `Specials` witness**, still unfound.
- **The precision axis's `unargued` status**, no induction argument existing for it or for the radix.
  `unargued` returned zero hits in `91`, `102`, and every file after `78` (`109:488-489`); the item stands
  along with the vocabulary that gives it meaning.
- **The container-class coordinate's companion model** (above) and the container-class transfer coordinate's
  own twelve-container fact.
- **The `algebraic_mul`-decoupled-from-`contract` question**, whether a same-format multiply with no adjacent
  add legitimately needs it (`58:1093-1094`, dropped at `63:863-868`, `109:379-381`).
- **The face-level sibling pricing question**: whether every operation in the design's surface needs a
  face-level sibling or only the ones a consumer is likely to chain (`58:1107-1108`, same drop).
- **The `on_unimplemented` `{Self}` interpolation sweep**, safe against the two carriers tested and unswept
  against the rest (`58:1108-1110`, same drop).
- **The `#[deprecated]`-shaped lint on a direct `Reduce` bound**, untried, belonging to a workspace-lint
  accounting the review has not opened (`58:1110-1111`, same drop).
- **The decimal face**, untested; the notation fixture is radix-two throughout (`58:1090`, same drop).
- **`float_algebraic`'s second independent vetting read.** Vetting complete and `ALLOWED` since `62b`, row
  drafted, still riding in the `unstable-features.md` package.
- **The eleven-crate taxonomy's remaining per-row confirmations.** **Artifact:** op's confirmation or
  correction per row.
- **`FromConstant`'s breaking-change fix.** Vehicle held.
- **The uniform-sampling spec question in `arvo-pseudorand`** (uniform over values or uniform over data, a real
  divergence the moment the grid is non-uniform, undecided by any ratified decision and imposed only as an
  obligation to say). **Artifact:** one dispatch on the named question.
- **The digest chapter's type-level history split.** **Artifact:** its own dispatch.
- **The still-undecided dependency edge from `arvo-graph`/`arvo-spectral`/`arvo-comb`/`arvo-sparse` onto the
  algebra-contracts crate**, open since file 26 and still nobody's call. **Op's `13b:10-22` instruction is
  discharged and this is what remains of it**: op refused relocating the algebraic laws to hilavitkutin and
  said the useful residue is that "a separate place within arvo is available and is worth designing rather than
  defaulting". **The separate place exists**: it is the algebra-contracts crate, which this document names at
  three places. What remains genuinely open is the crate's dependency edge, which is the same question one
  layer down and had been reading as unrelated to op's call (`112:440-447`).
- **`notko-hlist`'s binding-time sentence** (a count that decides a type is a type-level `Nat`; a count
  computed at runtime is a `Cardinal`; the mirror between them is a one-way projection).
- **The toolchain trap as a standing probe convention.** Four files have hit it in two opposite directions: run
  from inside the tree, `rustc` resolves to the pinned `1.98.0-nightly (57d06900f 2026-05-27)`; run from
  outside, to stable `1.94.0`, which does not parse `type const` and reports it as an ordinary parse error.
  **Artifact:** one line wherever the panel's probe conventions land.
- **The concurrent-edit citation hazard**, three files in a row recording it: two members editing one shared
  base at once, so a line citation from either still resolves and now points at different content, roughly 44
  lines low in one case and 437 in another (`122:19-27`). **Artifact:** one sentence in the panel's probe
  conventions, beside the toolchain trap, since both are conventions rather than design.
- **The pin-hash discrepancy flagged out-of-band for op and the workspace** (`49:779-783`, dropped at
  `58:55-66` which records the hash and not the discrepancy, `109:324-328`): `workspace.md` records `cced03bfd`
  for the pinned channel date; every measured record in this review records `57d06900f` for the same channel
  date. **Artifact:** a one-line reconciliation in `workspace.md`, outside this panel, with the measured record
  winning per the workspace's own provenance discipline.
- **The never-typed-from-the-outside item**, which file 54 discharged one third of with the explicit
  instruction that "the item should shrink to name them rather than stay at its original width"; deleted whole
  at `58:1068-1132` instead (`109:323-327`). **Artifact:** the shrunk item.
- **The two unbuilt grounding tiers** (a mockspace registry namespace; a probe-header line).
- **The rest of the tower's arithmetic family, named so "the rest" is a list.** Section 1.23 declares `Gcd` and
  `Dec`, the two whose codomain this document fixes. Five more are used by name: `VAL`, which is the readout
  and is an associated const rather than an operation; `Sum`, whose codomain is a `Nat` and which section 1.27
  exercises (`Pz<Sum<N5,N7>>`); `Reduce` and `BiasProduct`, which compute a numeral's adjustment and its bias
  respectively and whose codomain is therefore the rational carrier, with `Reduce`'s own domain, an unreduced
  pair, spelled nowhere; and **`Cmp`, which is the one that cannot be declared**, named at four sites and given
  a result type at none. It is the only member that answers a question rather than producing a number, and the
  design has no type-level ordering vocabulary for it to answer in. **Artifact:** one sentence naming what a
  type-level comparison projects to.
- **`FieldLayout`'s members past its extent, and `Canonicalisation`'s members.** Both are described by the
  facts they carry (field widths, the hidden bit, the encoding bias, reserved codes; signed zero, preferred
  cohort, NaN canonicalisation) and never by a spelling. Both are declared with what is fixed and nothing else.
  **Artifact:** the member lists, from the format instantiations that need them. **One rider**: whether any of
  `Encoding`'s members turns out to be kind-dependent, which would grow section 1.21's impl count without
  moving its shape.
- **Three instruments named as owed and none of them run.** They are distinct, keyed on different populations,
  and no amount of any one substitutes for another. **First**, a sweep of the ninety-nine numbered member files
  and the probe directories for material no consolidation ever absorbed, named owed at `109:643-646`, sampled
  six times by file 111 and confirmed untouched at `112:541-546`. **Second**, the same register diff run
  against the inherited-state topic file, which carries fifty-two decisions plus the duplicate sequence and of
  which this document cites twenty-one by number; **it has never been diffed against anything**
  (`113:450-457`). **Third**, a restoration ledger keyed on what a consolidation restored, one row per item,
  carrying the source range that *established* the statement rather than the last carrier, so an entailment
  check is two line ranges per row rather than a full re-read of the archive (`111:474-501`). **Artifact:**
  three dispatches. The first two find material; the third makes the next restoration checkable. **Section 9 of
  this document is the third one's shape at one document's scale**, which is not the same instrument and does
  not close it.

### Unchanged and untouched, carried forward

The reduction firing site and whether `FullRange` survives as its own named `Adjustment` constructor. The
dither-versus-`Refuse` interaction. `SC_WRAP<n>`/`SC_WRAP_SM<n>` with `n_bits > 0`. Richer canonicalisation's
branchlessness and cross-word bitpacked field extraction. `DatumDeterministic`. The
`Gcd`-for-a-local-`Rhs`-on-a-sealed-`Self` coherence question, confined to the proposed bottom carrier crate.
The membership uniqueness scoping (D39) and the seven upper vocabulary members' reading. The
integer-saturating SIMD lane residue, known-open since file 20.

### Three threads op explicitly kept open, with instructions to keep iterating

From `11:449-453`, `04b:47-60` and `06b:53-60` (`109:144-158`), absent from every consolidation between the
second and the eleventh. **None of the three is settled, and `04b:18-20`'s standing instruction is that op will
say when the panel is ready for synthesis.**

**Thread A, the consumer-facing surface.** Op, verbatim (`04b:26-27`): "**Option 1 but not just price, iterate
on; there might be ergonomics to be won when taking further and specializing, instead of stopping at this
solution.**" The state when it left the record: three candidate shapes were built and measured. Plain type
aliases are cheap and genuinely broken in the failure case, since the alias is destroyed the moment the failing
trait lacks the diagnostic attribute or the failure sits one where-clause away. Concrete newtype faces recover
the spelling of the numeral half but not the policy or lowering half, which is the half a consumer varies day
to day. **The strongest measured result is a third shape: nominal constructors at every position a consumer
selects, combined with small per-axis modifier types that delegate every member except the one being changed**,
under which **every axis of the composition renders for free in an error message** provided every value a
consumer can select is reached through a *named* type, because rustc prints the type arguments a consumer
applied but not the associated types those arguments project to. (The measurement was run against D69's
ten-axis set, which section 1.10 records as three members short since `39b`; the mechanism is indifferent to
the count and the figure below is not.) Three unresolved costs ride with it: it was measured under the fused
two-parameter form and costs roughly 1.8x more rendered length under a three-parameter split, so the diagnostic
win and the parameter-count question are coupled; bounding a law on a computed boolean produces an error naming
`False` rather than the composition that failed, and the four-line fix is verified and not folded into the
spec; **and the modifier types do not canonicalise**, so two different orderings of the same set of changes are
the same composition with different spellings and different rendered error text, which matters for anything
comparing error snapshots textually. `modifier`, `OverRangeOf`, `nominal` and `identifier` all returned zero
hits in `26` and `40`.

**Thread B, fallible arithmetic.** Op keeps `Precise` refusing and prefers widening the algorithm crates to
accept it, while stating there is untapped potential in the shape, so the live question is what the best
possible form of fallible arithmetic in a `no_std`, no-`alloc`, monomorphisation-only substrate is, **and what
it unlocks rather than what the current form costs** (`04b:57-60`).

**The thread's two positive results, stated because a consolidation carrying its three costs and no mechanism
leaves a future member to rebuild both** (`111:178-197`). First (`11:509-515`): **one generic arithmetic
function body can serve both a total and a fallible composition without duplicating the body, provided the
resolution rule constructs its own answer** in whichever carrier it needs, rather than the calling body
constructing a refusal generically. This matters because arvo cannot implement its own operator traits on a
foreign type such as `notko::Outcome`, the orphan rule forbidding it, **so any design where the calling body
builds a refusal directly hits a wall the panel initially mistook for a fundamental limit rather than a
consequence of one specific shape.** Second (`11:517-520`): with two range positions, **the return type any
composition needs is the join of the two resolutions' own carrier choices, connected by a lift where they
differ**, which is the same shape effect systems use for combining independently-installed handlers, **and it
scales cleanly to a third or fourth effect** (a divide-by-zero refusal, say) **without redesign.**

**The sharpest reframe of the thread**: whether a refusal *arrives* as a checked sum type, as an absorbing
bottom value carried inside the numeral's own spare bit pattern, or as an accumulated sticky flag read once at
the end, **is by this design's own axis-sorting test a `Lowering`-level choice**, since the representable set
and the mathematical function computed are identical across all three and only the cost and the shape of the
call site differ. Under that reading `Precise` keeps refusing exactly as designed while a consumer separately
picks how the refusal travels; a refusing composition delivered as an absorbing bottom was run unmodified
through the existing generic graph-ranking crate and produced correct results, settling once at the boundary.
**Three real costs, none resolved**: the absorbing-bottom delivery only costs nothing (branchless, two extra
instructions over a plain saturating baseline) when the numeral's storage has spare bit-pattern room, **which
is only reliably true for `Precise`**, and where there is none the identical mechanism costs eight times more
instructions and doubles the value's size because a companion flag has to be threaded alongside. A delivery
propagating a bottom through addition must also propagate it through **selection**, and a naive total-ordering
comparison silently discards it, **which is precisely the defect IEEE 754-2008 shipped in its own min/max
functions before the 2019 revision fixed it**, and at least one of this design's own generic algorithm crates
performs exactly that kind of selection today. And under the sum-type delivery a refusing operation's
short-circuit is measurably not constant time (two data-dependent branch exits per element against none for the
bottom delivery), **which means the `ConstantTime` derived marker is currently keyed on data that does not
decide it: delivery decides it, and delivery is not an axis of the composition at all** (not in D69's ratified
ten, and not among the members that survive it at section 1.10 or the trait table at section 1.23; the finding
was first written against the ten and does not depend on the count). `delivery` occurs fourteen times in `11`
and zero in `26` and every consolidation after; `ConstantTime` four times and zero (`109:153-158`).

**The boundary op drew around that marker.** D74 (from `202607301100`) (`talk:1829-1837`, op, 2026-07-30):
"`ConstantTime` is a derived marker, not an axis", joining `Deterministic` as a property computed over the
composition rather than requested through it. And op's own clause, recorded in the decision **"so a later
reader does not reopen it as an oversight"**: the marker **reports rather than requests**. A consumer can check
whether the composition they chose is constant-time; **they cannot demand it and be refused when it is not**,
and an internals change adding a data-dependent early exit for speed would silently withdraw the property.
**That is the accepted trade.** Two readings survive on whether the finding above is the reopening D74 forbids,
and the evidence does not force one: under the first it is, arriving because the clause was absent; under the
second D74's clause covers an internals change withdrawing the property while the finding is that the marker's
*key* is wrong, a defect D74 never addressed. File 113 leans to the second on the ground that the finding's
ground is delivery rather than an early exit (`113:334-350`). Either way the clause belongs beside the finding.

**Its sibling survives and its sibling's mechanism does not.** D70 (from `202607301100`) (`talk:1648-1654`,
op): **`Deterministic` is a derived marker over the composition**, a blanket impl keyed on the composition
making structural the qualification D49 (from `202607292300`) stated in prose, so **the marker holds for a
composition rather than for arvo and the claim a consumer can rely on is exactly the claim the type makes.**
D49 survives in the record by number as the determinism argument's source; the mechanism it was given was
absent (`113:241-254`), and the standing base carried one of the two derived markers op declared in the same
stretch.

**A converging finding from three independent directions** (layout cost, generated code shape, and the orphan
rule): whichever delivery mechanism is chosen, **the carrier holding a refusal should be arvo's own sealed
type, with a single `settle()`/`observe()` accessor as its only door**, which is the perimeter rule arriving at
this question before the perimeter rule existed. A fourth dissolution was proposed and never tested against a
real algorithm crate: a locally-installed handler at the call site, selected by a turbofish and free under
monomorphisation.

**Thread C, leaf truth.** Its state at `06b:44-51`: the type machinery delivers totality and coherence but
never the truth of a leaf fact; solver-free const checks were proposed, six were compiled, the counterexample
was reproduced mechanically, and the constraint found was that a `const fn` cannot call through a function
pointer so the oracle must be macro-instantiated; under the computed-truth encoding the diagnostic attribute
never fires and a consumer reads `False: IsTrue`, repairable in four lines. **What nobody has yet done is what
op asked for: find a shape where the check *is* the typestate rather than sitting beside it.** Section 1.30's
three-rung ladder is the closest the review has come and does not answer it.

**Op's standing correction on how downstream evidence is read**, stated with the threads because it governs all
three (`06b:18-40`). Op, verbatim: "**Hmm. The fact existing consumers do things one way, might just be because
no better existed (we know this, this is why we are here). Should be irrelevant, we focus on the optimal, what
the consumers would ideally deal with and in.**" **What a consumer currently writes is evidence of what was
absent when they wrote it. It is not evidence of what they need, and it is certainly not a requirement to
preserve.** The question a member asks about any downstream observation is **"what would the consumer ideally
be dealing with, and in what terms"**, and where the ideal answer breaks every existing call site, that is a
migration cost to state plainly and not an argument against the answer. **The error is only in treating the
*shape* of the workaround as the *shape* of the need.**

### Two further items op kept open that left the record

**Preset divergence** (section 1.21), with op's instruction that it deserves more than the first mechanism that
works.

**`arvo-num-systems` and `notko-hlist` as the cheapest repeatedly-flagged open item.** Six separate members,
independently, flagged the same unread pair as a likely input to the cost picture for any
type-level-set-shaped mechanism, and `26:661-666` calls it "the single cheapest, most repeatedly-flagged open
item in the whole document". The third consolidation then claimed its stretch "reads the two pieces of prior
art eight separate members had flagged and none had opened" while `hlist` returns zero hits in it and the
stretch reported on one of the two (`109:277-280`). `arvo-num-systems` has since been panel-worked;
`notko-hlist` still owes its binding-time sentence and has never been opened.
---

## 6. The droplist, cumulative

**Every real removal across the whole panel, in one place, one line each.** The droplist went delta-only at the
fifth consolidation, so a reader of the tenth saw eight entries out of roughly ninety (`109:560-564`); the
eleventh reassembled it and this one carries it forward. **Entries retiring a proposal born inside the stretch
that absorbed it are not on this list**; they are section 7, under their own name, because file 109 found that
sixty-six of the eighty-seven entries ever authored are of that kind and only twenty-one correspond to a real
removal from a predecessor (`109:539-558`).

**The stated purpose of an entry**, reproduced from the third consolidation's own preamble because the fourth
carried the preamble and stripped the reasoning it promises (`109:566-574`): entries are stated **with just
enough of their reasoning that a member who believes a retest would come out differently knows what has to be
overturned**. Where a later document stripped an entry's reasoning, it is restored here.

**Relocating the algebraic-law machinery to hilavitkutin**, on the theory that associativity is specifically the
contract of parallel reduction: refused by op directly, **and independently undercut by measurement: the
regrouping that would have motivated the move already happens inside arvo's own licensed internals, on a single
thread, worth roughly 2x, before any scheduler exists to relocate to.** The fourth consolidation carried this
entry with neither the theory nor the measurement, eight lines below the preamble promising both.

**Gating `arvo-graph`/`arvo-comb`/`arvo-spectral` on `AddAssoc` (or any associativity fact) by default**:
measured directly to admit the one preset (`Hot`, wrapping) whose recurrences return wrong answers under these
algorithms' own stated specifications, and refuse the two (`Warm`/`Cold`, saturating) that compute correctly,
because associativity and the distributivity these algorithms need are different, complementary laws that
invert across the same presets. `Monotone<Add>` is the atom the refusal was reaching past (section 1.20).

**"A documented traversal order substitutes for a law"**: wrong axis. Associativity is about grouping, not
order, and contiguous chunking preserves element order exactly while still changing grouping.

**Bounding a regrouping combinator on a numeric diameter budget rather than a boolean law**: refused by
measurement, since signed saturating addition's regrouping diameter grows to the entire representable range by
a five-element fold, so there is no useful budget to bound against.

**Predicting the accumulator-agreement threshold from the recovery map's monotonicity**: refuted by exhaustive
measurement. **Every non-homomorphism resolution reaches the same threshold (`K = n - 1`, interior safety)
regardless of whether it is monotone.**

**Computing type-level width arithmetic as a const generic under `min_generic_const_args`**: refused
structurally at the definition site; the feature's sound subset explicitly forbids arithmetic over a
still-generic const parameter on its own right-hand side. Replaced by type-level binary width encoding, itself
later replaced by the value-unique `Nat`/`Pos`/`Bias` encoding.

**Growing an accumulator's own *type* on every iteration of a runtime-bounded loop**: cannot work in principle,
not merely unbuilt, since a type cannot depend on a value only known at runtime. Replaced by fixing the
per-element product's type and checking accumulator sufficiency as a compile-time bound; a renormalising step
is the property that closes the gap (section 1.8).

**Declaring a fidelity-licence coercion as a trusted marker trait with no associated items**: compiles clean
when corrupted, with zero diagnostic, because a permission-shaped coercion carries no data for the compiler to
check against; a hand-verified wrong grant produced a silently wrong numeric answer. Two follow-up fixes also
failed: a fully generic blanket derivation hit the generic-const-in-type-position wall, and porting a
`WITNESS`-constant shape onto the same trait is disarmable exactly the way one existing resolution
constructor's classification was already disarmed, **because the implementor writing the lie also controls the
check for the lie inside the same impl block.** Replaced by recomputing the relation inline in an ordinary
`const {}` block at every consumption site. **The principle the entry was trying to satisfy is op's adoption
and no consolidation stated it**: `17b:19-30`, **"a fidelity grant is checked rather than asserted, on the same
footing as the recovery map that the earlier verification thread ended up witnessing."** Op also recorded what
that does *not* settle: the shape, since a licence witness is not a port of the recovery-map witness, there
being no returned value to check a grant against, "which is precisely why the corruption went undetected." The
replacement satisfies the adoption in substance, because recomputing inline is checking rather than asserting,
and without the principle stated a reader could learn only that one particular way of not checking failed
(`112:466-473`).

**A pushed, registered build-layer manifest** for monomorphisation recovery: strictly worse information than
the pull-shaped symbol-table read, since it records what a consumer *declared* rather than what got
*instantiated*, silently misses every composition reached only through generic code, and **cannot be written at
all for a generic function**, because Rust forbids an item declared inside a generic function body from naming
that function's own type parameters.

**Treating `f64::mul_add` as a source-expressible fidelity liberty (contraction)**: it lowers to `llvm.fma`, a
distinct, exact IEEE operation with one defined answer rather than a licence to pick either; it is unavailable
under `#![no_std]` without an unvetted or forbidden feature; and on a target with no hardware FMA unit it
compiles to a pessimising libm call, the opposite of what a licence should cost. **`Fused` belongs in the
design as a named operation, not as a fidelity permission.**

**Citing "the one shipped `Monotone` law implementation"** as existing, shipped design: it does not exist as any
implementation, only as an unlocked design-round proposal, **and two members built directly on the false
citation before a third caught it with a single grep.** Checked against its own admitted compositions and found
false: it names three of the five quantiser members that decide monotonicity, and asserts monotonicity for a
resolution (`ReduceModulo` at both range ends) that a separate proof rules out for every width.

**Assuming the recovery-map classification's cheapness transfers automatically to a new operation**: refuted
twice independently. **The classification is a property of the pair `(phi, Op)`, not of `phi` alone.**

**"Past the top is unreachable" once infinity is representable**: false. Infinity changes the over-range
position's neighbour rather than removing the position; the midpoint that decides overflow lives on the
round-first amendment's unbounded grid.

**The unsigned faithfulness blanket over every `Resolution` pair**: refuted by compiled counterexample.
`SubstituteZero` breaks associativity where clamping and modular reduction preserve it. **What it refutes,
named because the entry did not name it** (`113:427-438`): the two-impl law derivation in the round's own
ratified body, `impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}` at `talk:1187-1203` and
`spec:203-222`, which carries no D-number and is not among the items `spec:356-359` marks as the agent's own.
Its reasoning was that unsigned addition can only leave the range above, so one end is unreachable and the rule
is truncated addition whatever it does there. **The counterexample is that `SubstituteZero` at the reachable end
is not truncation.** The signed impl, bounded on `Faithful` rather than on `Recovery`, is unaffected. **Op's
part is only whether the refutation stands against his own text.**

**Classify-then-round as the quantiser's order**: disagrees with all three test standards on the band past the
largest representable but within half a quantum of it. Replaced by round-first, classify-second.

**Two round-trip theorems as the crossing contract** (`decode ∘ encode = id` and `encode ∘ decode = id`, both as
identities): the second is false the moment signed zero, NaN payloads, or decimal cohorts exist. Replaced by
the section-retraction triple.

**A single three-instance `Sign` axis bundling range and zero-count**: under-determines the set and mixes a
value fact with a datum fact. Split into `SignDomain` (identity) and `SignIndexing` (encoding), and section 1.2
states what that split buys.

**Block floating point as evidence for nesting `Adjustment`/`Bias`/`Underflow`/`Specials`**: the nesting
decision stands on the `Underflow` argument alone (section 1.2); BFP is a different kind of object, a composite
numeral over a shared external exponent, and is not evidence for either shape being cheaper to extend.

**Referential uniqueness as an alternative to value uniqueness**: fails the ordinary case of storing a product
in a declared numeral, and is an invariant living in a signature-writing convention, the class this review
keeps finding rots silently.

**Projecting a trait-level width computation back down into an ordinary const parameter** to dodge the
value-uniqueness obligation: refused, the feature named (`generic_const_args`) being neither the permitted nor
the forbidden one and unvetted.

**The claim that the shipped width chain and integer adjustments already satisfy value-uniqueness**: false for
the width chain, since `UInt<UTerm, B0>` is a second spelling of zero and the adder propagates rather than
normalises it.

**"Two spellings of one condition" for interior safety and total safety**: two distinct conditions serving two
distinct promises (section 1.8).

**The ordered three-relation ladder** (weak, then Kleene, then graded): replaced outright by the nine-point
view lattice, which is not a chain and contains two shipped presets at incomparable points.

**"Partial associativity" as a named gap in the algebra vocabulary**, adopted by op at `17b:40-50` after file 17
measured that `Precise` has zero numeric spread across groupings and that its regrouping sensitivity is
entirely about which groupings are defined at all, with op recording that "the design does not name it, and no
standard vocabulary carried in the spec covers it": **retired with the ladder above.** The nine-point view
lattice supersedes it, and the point it names is stated positively at section 1.7, `Precise` below interior
safety sitting where a view "preserves values and events while losing definedness", which is the definedness
reading the partial law was for. **The entry as first written retired the ladder and said nothing about op's
adopted name going with it**, so a reader found an op adoption in `17b` with nothing anywhere saying what
became of it (`112:377-397`).

**`ffl` as a member of the physical-grounds row** (`63:447` lists `pin`, `host`, `flags`, `model`, `ffl`; the
registry section above lists the first four): removed, because the transfer-ground scheme replaces what `ffl`
was credited with, per `68`'s own section heading, and `109:398` flagged it as used and undefined. **The
removal is probably correct and it was silent**, which under this document's own rule that section 6 is the
cumulative diff is the thing the rule forbids (`111:170-176`).

**The reification-stability generalisation** (that the graded relation is the only one stable under a
`Refuse`-to-special reification): true of one reifier and false in general. Under an out-of-set absorbing
special, Kleene is stable too; under `SubstituteZero`, nothing is.

**`Op::IS_EXACT` alone as the statement that an operation's grade monoid is trivial**: false in general,
corrected to the conjunction with `Total<Op>` (section 1.7).

**A consumer-declared required view as the mechanism gating a regrouping's licence**: killed by the compiler
mid-dispatch, because the licence check refused exactly the case the mechanism existed to handle. Replaced by
the transfer rule, carried by a type projection rather than a declared const.

**The subset-domain reading of the view parameter**: not closed under meet. Replaced by the
quotient-of-the-grade reading.

**`Bias` as a plain signed integer**: made a legal MATLAB numerictype unrepresentable (slope 1, bias 1/2),
which is the standard's own test failing. Corrected to a signed, gcd-normalised rational, built and sealed.

**Three separately-restated `Numeral` member lists across files 35, 36 and 38**: none of the review's compiled
results depended on any of them.

**The candidate closed form for the overflow band, `q_result <= 2 * lattice`**: refuted by exhaustive
enumeration in both directions (753/1000 addition, 639/1000 multiplication). Replaced by the two-clause
lattice-plus-reachability form (section 1.5).

**`Specials` as a three-instance chain** (none, infinities-only, IEEE): the middle rung's witness demand exposed
that the axis was the wrong shape entirely. Replaced by the two-fact product.

**Absorbing a decimal numeral's quantum into its rational adjustment**, dispensing with a separate exponent
axis: does not compile at any real decimal format's exponent range, against 64 ms flat for the
radix-and-exponent spelling of the same grid (section 1.17).

**A finer-grained reassociation licence than the four-flag `algebraic_add` bundle grants**: does not exist on
the stable-track surface as tested. The workaround (discharge each companion permission separately) is sound
because each is independently checked, not because the bundle became narrower.

**The claim that the vectorisable-loop-idiom finding held unconditionally**: it holds, but only under
`-C codegen-units=1`, inherited by accident from an earlier unrelated investigation and never identified as
load-bearing until it was checked directly against the real crate.

**A bounded numeral-notation table**: refused on principle, and a second, independent route to the identical
refusal appears at the notation macro's own face layer, where a const-generic face cannot be structurally
sealed.

**Treating the algorithm crates' `Precise` exile as the problem to solve**: the presets the design admits today
both return wrong answers under the exact bound they satisfy. **The exile was never wrong; the admission was
silently wrong**, and no amount of readmitting `Precise` addresses a defect living in the crates' own return
type.

**The three-instance reading of `Underflow`** as one axis carrying flush-to-zero alongside gradual and abrupt:
flush-to-zero changes no representable set and is a `Quantisation` resolution, not a `Numeral` fact wearing
one's clothes.

**The door as a projection from the strategy alone**, with a software fallback refined where the numeral is
host-implemented: refused by coherence (`E0119`) and, separately, by `min_specialization` twice; the only
opener is a forbidden feature.

**`Cold`'s door justified as "follows the semantics-first side"**, a storage fact that does not imply an
arithmetic-lowering answer: replaced by a justification that does.

**The framing that a datum-level `TotalOrd` makes none of the algorithm crates' outputs law-expressible**:
refuted by compile as a design-wide verdict, correct only for `arvo-spectral`.

**A pure `macro_rules!` decimal-to-binary muncher**: refused structurally, not merely found expensive. No
fragment specifier, restringify trick, or const-generic escape reaches a decimal literal's digits.

**Pricing a "checked" declaration sweep against an unused type alias** rather than one whose bound is actually
forced: produced a misleading result in the wrong direction on its own first attempt. Corrected by forcing the
bound in both arms before comparing, and the control is now a stated convention (the conventions section).

**A universal unreproducibility claim grounded on compiling one file in isolation** rather than reading the
directory it ran in: refuted by the committed recipe and the rebuild, and the `unreproducible` ground's own
exhibit was struck while the ground survived on its second exhibit.

**The last sentence of `unstable-features.md`'s transfer argument**, "without them, monomorphisation is uniform
and the transfer is sound": refuted twice by compiled counterexample with the bans in force, promoting a
necessary condition (implementation uniformity) to a sufficient one for a different claim (property uniformity)
the rule's own source had already named unproven.

**The two-mechanism enumeration of ways an instantiation can be observed** (`specialization`, `TypeId`) as
exhaustive: refuted by a shipped, permitted third mechanism, const-tag container dispatch, demonstrated with a
property true at eight bits and false at nine, no gate, one parametric body.

**The "refused at nine bits" wall as a width ceiling**: refuted; it is a total-step-count budget, and a cheaper
predicate compiles clean at nine and refuses one bit later.

**"Exactly one cell of the matrix leaks"**: refuted; the matrix held `Specials` fixed, the full product leaks at
six of eight cells, and the correct framing is a family of configurations (section 1.4).

**Widening the crossing contract's target through the quantiser** as an alternative to statement 0's obligation
on the encoding: refused on every escaping datum tried, against a 2,701-value negative control confirming the
quantiser is otherwise the identity. **Not a preference, an arithmetic fact.**

**Cross-call-site face identity as something needing a mechanism**: refused; nothing that affects compilation is
keyed on the face, per-site display is the better diagnostic, and unifying faces would build the exact
false-refusal failure the layer-keying rule forbids.

**A committed sketch's universal claim that the facade's "only live GCE constructs are two static asserts"**:
refuted by a whole-crate compile, two of 478 spans, and it is the origin of the whole-crate-compile convention.

**`63:816-817`'s claim that the facade fix "touches every consumer of `Bits`, `UFixed` and `IFixed`"**: refuted;
three of the four things a consumer writes are unaffected and the real public break is twenty-one call sites
naming `Fixed<I,F,S>`/`Signed<I,F,S>` directly.

**Route X for the facade migration** (const-keyed projection, only the computed width lifted to a type, `I`/`F`
staying bare consts): refused structurally, six ways across two compiled attempts.

**Route Y for the facade migration**: fails guarantee parity three separate ways, each a compiler diagnostic
rather than an argument. The two-dimensional impl table refuses correctly at type-check but is priced on a
ceiling the toolbox rule forbids the substrate to set below what it dispatches through, failing outright below
width 64 and costing 30.0 s at a 256 ceiling, roughly quartic. A host-staged witness compiles clean and fast
and **is caught only at `--emit=link`, not at `--emit=metadata`, which is the command a consumer's editor
actually runs**, silently re-opening the `UFixed<0, F>::ONE` defect. A consumer-emitted per-declaration impl is
refused by the orphan rule, `E0117`, **with rustc's own diagnostic naming route Z's shape as the remedy.**

**The capacity unification's naive spelling**, "the shared carrier answering directly for the backing array":
refused four ways, citing the forbidden `generic_const_exprs` and, behind the compiler's own suggested
successor, the inductive step `2 * P::VAL`, which `min_generic_const_args` cannot express either.

**The feasibility probe's implicit claim that the capacity unification's whole load-bearing path was compiled**:
it was not; the probe declared the capacity trait as a bare const and never reached the associated array type
the domain exists for.

**The working "two instances" resolution of `Layout::Bitpacked`**: superseded. The axis has one meaning; the
byte-aligned reading was always `Layout::Dense` at a narrow `StoredWidth`.

**File 32's own bitpacked measurement, treated as a measurement of `Layout::Bitpacked`**: it was always a
measurement of `Layout::Dense` at a narrow width, correctly built and mislabelled.

**The hardware-reachability theorem's original statement**, "reachable only in a uniformly-`Hot` expression":
corrected to four cells of sixteen once `Warm`'s door moved to `HostFloat<E>`.

**File 59's strategy-door table**, "every row below is derived from what the preset already means for
fixed-point arithmetic in the shipped tree": **void**, and it is the exhibit the whole `tree-meaning`
prohibition rests on.

**The nine-bit companion's original characterisation**, "the first point at which the padding half of the
crossing contract has observable content": superseded. It measured the ungoverned container level, not statement
P's content.

**`67b`'s naming principle** applied as written: dead. It forbids op's own `79b` intent pillar and was never
op's own hand.

**The "quantify over every inhabitant of the carrier type" amendment to statement 0's quantifier**: dead, killed
by a compiled asymmetry (`E0004` against a warn-level lint). **It laundered a trusted-base fact into a provable
one.**

**The one-clause fix for the mutation gap** ("re-canonicalises on release", stated with no enforcement mechanism
named): superseded by the two-tier repair.

**File 80's exact fold-width construction, `AllOnes` recursing on the value of `P`**: superseded. It does not
exist above binary128, refusing at rustc's default recursion limit and then at `Nat::VAL`'s carrier once that
limit is raised, both accidental ceilings coinciding at 128 and neither stated anywhere.

**The byte-image chapter's own prior framing**, "an invertible external image takes the crossing contract's
statement structure verbatim" (two statements): superseded; the structure has three.

**`90b`'s division instinct, alternative 1, `Hot`'s cell placed on the `Door`**: dead, killed on four compiled
or silicon-read facts, and the fork's other alternative carried the identical smuggle at a different address
(section 1.13).

**The proposed door-side domain-preservation equation**: superseded. It guards a region the entry-level totality
refusal, ratified at the same checkpoint that adopted the equation, already forecloses unconditionally, **and a
check on a door guarding a precondition the type's own construction has made unconditionally true for every
value that can exist is the definition of a vacuous guard**, which belongs in the suite as a regression pin, not
in the ratifying text.

**File 94's reader-quantified replacement naming test**: superseded. It cannot fail in the hands of the person
running it, the identical defect it correctly diagnosed in the parity suite it struck.

**File 93's citation of section 1.3's second, weaker sentence** as the guard the `Door` placement violates:
corrected. The operative sentence is the first.

**File 98's periphery assessment**, "five or six crate-level subjects no panel file has ever examined", with an
unbounded error bar: false. The ground is op-ratified, distilled into the panel's required reading from its
first hour, and row-rechecked at file 74 before the claim was written.

**D10's own storage argument for rotors over matrices, `1 + n(n-1)/2` components**: wrong from rank 4,
reversing its own comparison against matrix storage at rank 7. The decision survives on the grounds it also
gives; the count does not.

**The capacity claim "checked to agree in an inline const block at the one construction door"**: false above
rank 0. The trait-method route bypasses the door entirely.

**The ratified sentence that the array grammar's pairing is "forced by the language, not chosen"**: **the second
clause is true and the first is false, twice** (section 1.27). Recorded as a correction to ratified text rather
than a droplist entry, and the replacement sentence is offered rather than adopted, because a call about
ratified text needs two independent agreements and the record has two compiles from two members but one design
reading.

**The ratified sentence that a datum-keyed digest "masks the container straight to the fields' own width"**:
corrected by one word to the placement map's **occupancy** (section 1.22). A widening rather than a
replacement: the two masks compute the same bit pattern for every numeral that exists today.

**The persona's third clause of the truth-contract shape**, that the fifteen declarations bind on the
exit-carrying part: **backwards**, ratified as such at `108b:136-141`. Binding them on the exit refuses the
multi-lane instance at the impl, and that instance is the entire thing the generic branch buys.

**`103:198-201`'s perimeter-rule citation on `Bool`**: struck. The rule's own Boundary section excludes a type
with no invariant, and a decorative citation is worse than none **because it makes a taste question look
settled**, and because the two grounds put the call in different hands.

**"Is there a second truth type? there is, and it is shipped" as the ground for the fork's lean**: does not
survive deleting its shipped-source citation. Replaced by the variety-closure theorem, with `MaskOps` demoted
to a witness.

**The claim that the derived-storage construction is new to this panel**: false. It is
`76_probes/b1_structural_array.rs`, recorded as WORKS with two controls, thirty-one files before it was
re-derived.

**The claim that the derived-storage construction costs a quadratic in the number of capacities**: false. The
type machinery is free; the cost was one operation per element from a structurally recursive `filled`, and
rewriting it as a provided method over the projected slice collapsed 3.24 s to 0.12 s.

**The five-way grouping of the route-multiplicity finding**: retired. It is three defects with three existing
owners plus one non-instance, and **route multiplicity is a defect only relative to a guarantee.**

**The candidate fifth clause on the pricing pillar** for the route question: not adopted. The three instances it
would govern are governed better by the pricing pillar, which names the repair where the clause names the
symptom; the fourth is two-organs; and the survival mechanism is the separation requirement, which would have
caught three of the four in advance and was not run. **The precedent decides against its own citation:** the
correct response to a requirement that works but goes unrun is a moment naming when it runs, not a new
requirement.

**`91:12-13`'s claim** that the definitional-completeness line and the separation requirement were applied to
everything the ninth consolidation absorbed: false, at three of its own sentences, grep-checkable. A fabricated
diligence claim, not a design error, in the same register as file 79's search sentence.

**File 79's own diligence sentence** ("I searched `[Aa]rity` across every file; the hits are all fold-arity"):
false, confirmed independently three times. The substance of its conclusion survives on grounds it did not
give.

**File 82's three offered resolutions for `quantize`'s apparent new failure kind**: none adopted; the premise
dissolved instead.

**The eleventh consolidation's completeness line scoped to "this document's own new prose"**: struck. The line
quantifies over the whole ratifying text, and the exemption removed exactly the population a restoration
consists of, which is where six undefined terms were sitting (`111:503-520`).

**`117:493-494`'s trusted-base sentence** that an unrelated `lowering` dependency makes the contract split's
mechanism go silently with no diagnostic anywhere: **false**, refuted by re-running the projection with
`Lowering` fully in scope in the same crate, where it still refuses at `E0220` and names its own repair
(section 1.25).

**Keying `Policy` and `Lowering` on the numeral, as the repair for a preset denoting two rows**: refuted at exit
0. Two `Implicit` numerals, one number kind, disagreeing on what `Warm` means, with nothing in the language
relating them, which is the layer-keying rule failing in its original direction (section 1.21).

**Per-kind preset markers, as the other repair for the same defect**: refuted at exit 0 and on ratified text.
`Number<Binary32, WarmFixed>` type-checks at a value position, and D52 makes compositions public and bindable so
aliases hide the mispairing only for the four pairs they cover; D72 gives `arvo-strategy` four named markers
that the eight-marker shape deletes (section 1.21).

**Renaming the `Precision` marker trait, as the repair for the bridge's token collision**: not taken. It
rewrites a name `74b` ratified under op's own "the mechanism unifies and the vocabulary does not", plus two
bounds and a three-member alias family, for no gain, and `16d:14-15`'s tiebreaker decides against it (section
1.23).

**A second ceiling on the total width, enforced by an emitted marker per row**: refuted three ways. It refuses a
legitimate product at precision 80; it cannot be narrowed to the written site, because a `where` clause on a
type alias is refused and its repair is an unvetted feature; and its refusal shows a consumer eleven numeral
trees and no number (section 2).

**File 119's reading that the bridge table's cost is linear**: does not survive the extension past 4096 rows.
Each doubling costs between 3.3x and 5.6x, so the curve is roughly quadratic, the same character the per-width
container dispatch already measured.

**File 121's reading that the braces in `NatOf<{ I }>` are a general requirement**: half right. They are
required exactly when the const parameter's name also names a type in scope, which it does here structurally,
and renaming the parameters compiles unbraced (section 1.23).

---

## 7. Reversals inside a stretch, recorded separately

**These are proposals born and retired inside the stretch that absorbed them.** They are not removals from a
standing base, and mixing them with section 6 is what made the droplist read as a standing record while
reporting on nothing that left it (`109:554-558`). Recorded here in brief, because they are worth keeping and
are not what a reader diffing two documents needs.

The eleven-crate taxonomy's own suggested changes, offered and folded in. The persona checkpoints' calls that op
individually corrected or superseded at `68b`, `78` and `108b`. File 95's finisher clause on the uncheckable
field, adopted and reversed within the stretch. File 94's replacement naming test, adopted and retired within
the stretch. The five-way route grouping and its candidate clause, proposed and argued down one file later. A
draft claim that the derived-storage construction was new, corrected before shipping. A draft claim that it was
quadratic, corrected by its own first measurement. Two draft negatives that failed their own greps and were
narrowed rather than shipped. A prediction that the placement composite needed stating, refuted by the optimiser
performing the collapse itself. The eleventh consolidation's own draft ordering of section 1.27's three columns,
which changed once the compile-time measurement was isolated from the constructing body. **And from the
repair stretch**: a brief's premise that two compile failures were live in the standing base, stale by one file
and reproduced anyway before being set aside; a proposal to renumber or prefix op's frozen register, declined
in the same file that evaluated it; a proposal to mint D-numbers for op's unnumbered decisions, declined for
the same reason; and a first test-gate run whose green result came from a pipeline's last stage rather than
from `cargo`, caught and re-run to a log within the same file.
---

## 8. The format, what was restored, and the four instruments that checked it

### The format rule, and the second half a future author will be tempted to skip

**A consolidation states its own content or dies. "Unchanged" is permitted only when followed by the text.** No
section body is a pointer. No section body is a summary of a predecessor. Where a section genuinely did not
change, its content is written out anyway. That rule came from file 109's closing recommendation and file 106's
independent arrival at the same place, and the eleventh consolidation is the first document to satisfy it.

**And the second half**: **the check that a compression entails the prior text is performed by someone other
than the author of the compression**, because the author of the compression is the person who believes it
entails (`109:610-615`). The eleventh consolidation named that as its own residual and satisfied it
imperfectly, since file 109 audited the compressions and that document restored from the audit, leaving the
restoration itself unaudited when it first stood. **It has since been discharged four times, and each
instrument saw what the one before it could not.**

**File 111** checked the restorations against their sources and found two entailment failures, both repaired,
and six claims resting on nothing but repetition. **File 112** swept op's own twenty-three files item by item,
having first established that the working list in circulation named twelve and was missing the eleven earliest,
**and the eleven it dropped are the eleven that tell a member how to work, what the standard is, and when to
stop.** **File 113** diffed op's numbered decision register, which sits in the topic files outside the panel
directory and which no instrument had ever touched, because the standing base's own definition of the ratified
rung excluded it. **File 114** applied every repair the three identified. Then **files 115 and 116** read the
result cold, with the transcripts withheld, which is step three of op's own cycle, and **files 117 through 123**
closed what those reads exposed and landed four converged results.

**The rule the audits jointly earn, and it is one line longer than the format change**: **a restoration cites
the source that established the statement, not the last document that carried it, and where the two differ the
restoration says so** (`111:544-551`). Both entailment failures file 111 found were paraphrases produced while
restoring from the document that last carried the sentence; three further instances landed on op's own text
(`112:414-427`); and file 113's largest finding is the same defect one level up, a standing base whose oracle
enumeration omitted the register.

**And the droplist has a second half that is a diff against the predecessor rather than a record of the
stretch's own reversals.** Section 6 is the cumulative diff; section 7 is the reversals; the two do not share a
heading.

**Why the format failed rather than any author.** Every consolidation from the third onward opened with a
variant of "this document replaces it. It stands alone: no file in the panel directory is assumed read." **The
claim was true for the second, third and fourth. It was false from the sixth on, and the falsity is undetectable
from inside any single document**, because the mechanism that makes it false, the phrase "Unchanged this
stretch", is a **true statement about the stretch**. Nothing in a consolidation's own text distinguishes "this
section's content is below" from "this section's content is in a document I am replacing." The two read
identically, and the second is the one that shipped (`109:578-589`). **A stub is not re-derivable, which
collides with `108b`'s own first principle directly**: a file building on a ratified sentence whose grounds have
moved must re-derive it, and there is nothing at `102:180` to re-derive. **Pointer chains defeat the claim
transitively even where a pointer exists**: three hops on ratified material, one of them broken. **Eight
consecutive authors applied the discipline that existed and produced this**, which is why the fix is mechanical
rather than exhortative.

**This document adds one more mechanical half.** A correction folded into prose leaves no trace of itself, and
a document that folds fifty-one of them without a ledger has performed exactly the compression the format rule
exists to forbid, silently. **Section 9 is that ledger**, one entry per correction, out of the reading flow and
complete.

### What was restored, by kind

**Ratified or op-authored material, all eleven items file 109 ranked first.** Both preset tables, verbatim as
markdown, with their derivations (section 1.21). The `tree-meaning` prohibition, in its own paragraph (registry
section). The grounding registry entire: five rows, four transfer grounds, `unargued`'s own honesty rule, the
split, the deletion test, the perimeter (registry section). D38's enumerated ten-member vocabulary with "shipped
even if nothing uses them, vocabulary fixed by mathematics", and the Ostrowski refutation that the seventh
consolidation raised and the eighth dropped (section 1.6). Op's MATLAB/IEEE 754/SystemC standard from `13c`,
with all three clauses and the abstractions-are-what-matter consequence (section 0.1). The two standing
directives that left at the fifth consolidation: the no-single-angle rule and the rewrite-cost tiebreaker, both
op verbatim from `16d` (section 0.2). Op's constructive-deliverable directive (section 0.2). Thread A with op's
"iterate on, there might be ergonomics to be won" instruction and its three unresolved costs (section 5).
Thread B with its delivery reframe, its two positive results, its three costs, and the
`ConstantTime`-keyed-on-the-wrong-thing finding (section 5). Preset divergence with op's "deserves more than the
first mechanism that works" instruction (sections 1.21 and 2). The membership licence's `Specials = None`
constraint clause (section 1.6).

**The fourteen content-free subsections, all given content.** 1.1 (what a number is, plus the affine value map
and the UNORM8 worked example), 1.2 (the identity contract with `Implicit`/`Ranged`, plus `SC_SAT_SYM` and the
nested shape's own argument), 1.3 (the `Encoding` declaration, the charter blockquote, the double-duty
sharpening, the measurements), 1.6, 1.7 (the algebra in full, 1117 words' worth), 1.8 (the fold's two
conditions with their formal definitions), 1.9, 1.15, 1.17 (radix ten in full), 1.18 (the numeral notation in
full), 1.20 (the algorithm crates in full), 1.21 (the strategy door with both tables), 1.23 (the trait table),
1.24 (the cost model with all six rows and the cliff).

**Mechanisms and obligations whose loss would have cost a re-derivation.** The three explicitly refused
alternatives to statement 0's obligation. The crossing contract's three statements and its derivation
blockquote. The overflow band's two clauses, its zero-under-prediction measurement over 5,184 triples, its
six-row case table and "division has no row". The algebraic difference between `Implicit` and `Ranged`, the one
sentence three results follow from. The exact accumulator width formula and its real-format figures. The
positive enumeration of thirteen operations and the structural theorem it licenses. The `fold_compensated`
prohibition with its compiled `fsub s0, s1, s1`. The reassociation licence's four-clause receipt. `IS_EXACT`
and `Total<Op>` together. The direction-in-key derivation with all three conjuncts. The finest-view mechanism's
price against the alternative it beat. The strongest erasure measurement in the review. The 1.8x
rendered-diagnostic measurement justifying the nested shape. The `Monotone<Add>` two-door design and the planned
deletion of two hand-rolled workarounds. The unavailability of the standard's own carrier. The flush-to-zero
measurement that is the evidence for the strategy-door design. The Apple-silicon refutation. The
bound-rather-than-equality lever, "the strongest diagnostic message this whole review has found". Non-canonical
codes as a third and larger source of non-injectivity with its 209-of-768 measurement. The 923-assertion
whole-matrix test and the `37/53` bug it caught. The two bisected notation ceilings and the adopted two-tiered
refusal. The 4.5x staging measurement's figures. The `Adjustment` entry-point closure and the compiled
silently-wrong-value defect that forced it. The pricing hazard's own control. The seal's honest limit and the
six named carriers. The two dispatch conventions from `67b`. The `#[diagnostic::on_unimplemented]` condition on
the structural derivation, and the per-width table's 116-seconds-at-4096 pricing. The facade migration's
atomicity requirement. The cost model's numerator-dominates finding. The six named standing contract
mechanisms. The container-class coordinate and its twelve-container fact. The four-bin ledger, the
model-inadequacy standing risk, the saturating-reduction obligation, the multi-limb fragility item.
`foldnum`'s characterisation as spec text and its exact closed form. The `10^20` figure, the `unargued` status,
the `InfOnly` witness, the reciprocal-table strength reduction, the codegen-flag audit's five files. Five open
items that stopped being reported at the sixth consolidation and four that stopped at the ninth. The two
citation-adjacent restorations: the `IEEE §5.12` citation, flattened to "IEEE clause 5", and the bench
harness's overwrite defect, whose surviving half left the list when its retracted half was retracted.

**Op's own material from the sweeps** (sections 0.1, 0.2, 0.5, 2, 5, 6): the stopping condition and the
four-step cycle whose third step is the acceptance test; the post-canon four-phase sequence, twice-stated, and
where `79b` binds the verification mandate inside it; the end state; the checkpoint cadence and its two drifts;
the licence to argue; the four `16b`/`16c` posture directives; `Cold`'s cold-path meaning; the `WideBits` hole;
the reserved fused-versus-split call; the fidelity principle; `12b`'s hold on the axis-set completeness claim;
and three attribution repairs on op's text that cited the last carrier rather than op's file.

**The register** (sections 0.1, 1.1, 1.2, 1.9, 1.10, 1.23, 1.25, 1.26): D72's crate split with its table
transcribed cell by cell, D52, D53, D54, D56, D63, D64, D65's mis-keyed supersession, D66 and D67's shipping
half, D68's flat call and its silent supersession stated, D70, D71's two lost consequences, D73's marker half,
D74's accepted trade, D75's rename, D23, D31, D32, D33, D48.

**The counts and the claims resting on repetition** (the spine rule, section 1.5, the registry section, section
1.10, section 1.23): the eleven firings enumerated from `63:106-123` rather than downgraded, because the list
was recoverable one consolidation below the count that replaced it; the transfer refutation's second compiled
support and the four-legs analysis it rests on; D69's ten axes tabled with the three removals marked, replacing
two live uses of a stale count; the `Ranged` coordinates' lost figures; the growth surface's thirteen
operations against a count that matched neither of its own two candidate lists.

**Three silent supersessions made explicit** (D68, partial associativity, `ffl`), because a supersession that is
stated is legitimate under `108b:11-20` and one that is merely omitted is a drop wearing better clothes.

**The declarations, from the cold reads.** `Number` and its bound; the nineteen traits the trait table's
members are bounded by; the sealed carrier's BNF as declarations; `Folded`; `Lowering::Container`; the three
truth-contract organs and the producer; the bridge from a public const width to a type-level `Nat`. **Twenty-six
declarations in total**, of which two are types, twenty-three are traits, and one is a member added to a
ratified trait.

### The four citation defects, verified and corrected

**`78:41`** says "`68:816-817`'s facade-migration framing is superseded". **`68:816-817` is the membership
item** (the D38/D39 recommendation to scope the "finest" fact to the real/Cayley-Dickson chain). The facade
claim the eighth consolidation means lived at `63:816-817`, **which `68:26-30` had already corrected and
`68:965-967` had already droplisted**, so the eighth consolidation presents as newly superseded a claim its
predecessor had already refuted.

**`68:23-25`** attributes "exactly one cell of the matrix leaks" to "a sentence in the sixth consolidation's own
section 6 (file 66's finding, carried forward unchanged)". **`63`'s section 6 is Verification and contains no
such sentence**, and the sentence is file 66's, **written after `63`, so it cannot have been carried forward by
it.** Doubly wrong. **One correction to the audit**: the audit says the droplist entry at `68:949-951` inherits
the false attribution; it does not. That entry reads "File 66's 'exactly one cell of the matrix leaks':
refuted", **which attributes correctly**. The defect is confined to `68:23-25`.

**`26:523-525`** says "File 11's other open packaging questions (where `Width`/`Exponent` and the container
projection live, whether `Bits<N, S, Sign>`'s `S` should re-bound to `Lowering` alone) are untouched by this
dive and remain exactly as open as file 11 left them." **Neither question exists in file 11.** File 11's actual
§5.3 list is at `11:793-811` and is four different questions: what the composition type is called and its
parameter order and defaults; **what a preset is mechanically** (a plain type alias over one fixed composition,
or a nominal marker type from which axes are projected, with the note that the diagnostic and modifier work
needs the second reading and this has not been formally decided against the first); what `arvo-numeric` ends up
containing once the numeral, policy and lowering definitions move out; and whether `arvo-num-systems` depends on
the format concept or the reverse. **The sentence that asserts carry-over is the sentence that performs the
drop** (`109:255-260`). Of the four, the second is answered by the preset tables and the modifier work, the
fourth by the membership chapter, and the first and third are on the open list.

**`102:555`** says both ratified preset tables stand exactly as the ninth consolidation carries them. **The
ninth carries the sentence, not the tables.** Section 1.21 carries the tables.

### What the audit missed, found while working its list

**One further ratified-adjacent drop, and it is the largest single one after the preset tables.** The eighth
consolidation's open item 12, addressed to op and reading in full "**Whether construction one (the structural,
`unsafe`-discharged-at-the-door array) stays in the spec as a recorded fallback to construction two, or is
dropped**" (`78:872-873`), is **absent from the ninth and the tenth consolidations with no droplist entry**,
while item 13 from the same list, a tautological test to delete, is still being reported thirty files later.
The audit's `78` to `91` pair list does not contain it. **The consolidation kept the trivial item and lost the
load-bearing one, and the sentence that replaced it (`91:791-798`, the array grammar "forced by the language,
not chosen") asserts the opposite of what the lost construction proves.** **Credit where it belongs**: op had
already named this item at `108b:22-26`, "an item filed as an open question for op was carried by consolidation
eight and dropped silently by nine and ten while a sibling item from the same list is still reported
twenty-eight files later", and the eleventh consolidation presented it as its own discovery without citing him
(`111:326-333`). Under-crediting op is the harmless direction; it is recorded because it is evidence about
assembly, showing that at that point the document was working file 109's list rather than `108b`'s text.
Section 1.27 carries what the lost item proves.

**One correction to the audit's instance two, in the audit's favour.** File 106 reported the demand-driven
mechanism sentence (`55:163-165`) as having **zero occurrences in any of the ten consolidations**, on a grep
for `not evaluated|nothing touches|fires at use|AGREES`. That grep is right about what it searched and wrong
about the conclusion: **the fifth consolidation absorbed it at `58:754-761`**, which writes "a forced const
assertion **firing at use** (`Capacity::filled`/`from_fn`, every entry point) rather than at declaration". The
grep pattern was `fires at use`; the text says `firing at use`. **The audit's own account is the correct one**
(absorbed at five, dropped at six, rediscovered at one hundred, forty-five files later).

**One further drop, from the same pair the audit covers.** `102:90`'s "**Unchanged in statement**" over the
pricing pillar, which the audit reports as instance one, is also the drop of the **level-naming clause**
absorbed at `91:118-122` and offered at `83:290-316`. A later file reported that clause as never absorbed by any
consolidation, on a three-pattern grep that could not match because the ninth writes "names the width
**level**" with an intervening word and bold markers. **The correct disposition is neither "never absorbed" nor
"still standing": absorbed at the ninth, dropped at the tenth, with no droplist entry.** The clause is in the
pricing pillar's own statement, in general form.

**One universal negative in file 109 is false as stated, found by re-running it.** `109:270-273` reports that
`Contract` and `Fused` "return zero hits in every file of the panel after `26`". Both appear at `29:213-217`, in
the same sense, in a file whose own subject reaches the same classification from the quantisation side. The
finding the negative supports is unaffected (the fidelity residue is in no consolidation after the second), and
the negative itself is wrong. **Recorded because file 109 is the document that made re-running them a
requirement**, and it is owed the same treatment it gave everyone else.

**The archive is instrumented for error and not for loss, and that is the defect** (`108b:22-26`, adopted in
all three parts). The tenth consolidation's droplist has eight entries and **every one is a claim that turned
out false; not one is a sentence that was quietly not carried forward.** Section 6 is the second cumulative diff
the archive has had since the fourth consolidation, and section 7 keeps the reversals where they belong.

### The items file 109 found that are named rather than answered, and why

**Every item on file 109's ratified-or-op-authored list is restored.** That claim is true and it is a smaller
claim than it reads as: **restoring from an audit bounds the restoration by the audit's recall**
(`111:318-324`). File 109's list is eleven items it found by diffing consolidations against each other, so
anything that never reached a consolidation at all is outside both instruments. Three further sweeps have
measured that ceiling: file 111 found six ratified or op-addressed items in no consolidation and on no list,
**every one of them incidentally while checking something else**; file 112 swept op's own twenty-three files on
purpose and found fourteen absent, seven partially carried and three silently superseded out of ninety-three
items; file 113 diffed op's numbered register, which no instrument had ever touched, and found agreement by
number at fifteen of forty-six. **The population outside the consolidation chain was large, and the reason it
stayed invisible is that every instrument before file 112 was keyed on what a consolidation carried rather than
on what op said.** The remaining instrument nobody has run is the same sweep over the ninety-nine numbered
member files, which `109:643-646` named as owed and larger than its own list and which `112:541-546` confirms
is untouched.

**Recorded as correct removals rather than restorations, with the reason.** File 11's dead axis instances
(`Widening`'s three, `Growth`'s two, `LogicalWidth` as a primitive axis, `Underflow`'s `Unbounded`/`Flushed`
members, the `Narrowed<W, A>` shape) go with the axes themselves, ratified out at `39b`; the affine value map
and the UNORM8 example that shared their section **are** restored, because they survived the removal. File 11's
scope-section reading instruction ("Nothing below should be read as a statement about any of the untouched
rows") is not restored, because there are no untouched rows left: every one of the eleven has now had a content
review, and the instruction would be false if carried.

**Not answered, and the reader should know which.** Thirteen items from file 109's per-pair lists are neither
restored above nor carried on the open list, because no document in this chain could establish from the audit's
citation alone whether the item survived a later resolution or simply vanished, and **stating a design sentence
this document has not checked would be exactly the failure the whole exercise is against.** They are named here
so the next dispatch can work them rather than rediscovering them. **The list and the count agree at
thirteen**, which is the repair the eleventh consolidation's own final accounting needed, since it read "nine"
against bullets naming thirteen (`116:204-217`): a count cannot be checked and a list can, and enumerating is
the only repair that does not reproduce the defect.

1. **The preset-redefinition audit obligation** (`11:344-353`): "flipping them, test by test, in the same change
   that flips the implementation, is the audit obligation this redefinition carries." The second consolidation
   kept the question and dropped the obligation.
2. **`FullRange<0>`'s division by zero and the dropped `F >= 2` bound** (`11:674-678`).
3. **The affine formula's failure to cover `Stored` numerals** (`11:680-684`), one of three items from the same
   section of file 11, of which the second consolidation carried two and flagged each unresolved.
4. **The phantom-type closure's unpaid cost** (`11:750-758`): the second consolidation calls it "the real
   closure mechanism" and drops the qualification that connecting the proof type to the byte-holding type "is a
   real design exercise nobody has completed."
5. **The fidelity thread's own residue**, `Contract` (`26:438-441`), "the one real residue that genuinely cannot
   be expressed from portable `no_std` source today, and needs either the receipt-and-pass machinery or the
   unvetted feature path."
6. **The classification-versus-exhaustive-check overlap** (`26:92-97`), silently resolved by adoption rather
   than by ruling.
7. **The accumulator's three readings** (`26:619-622`), the same way.
8. **The `HostImplemented` locus item** (`63:621-623`), one of three narrow items the seventh consolidation
   closed by accounting for two.
9. **The third of the three residuals at `63:864-868`**, a host arithmetic wider than `u128` to exercise the
   structural ceiling, for which the seventh consolidation substituted a droplist entry, so the real residual is
   neither carried nor closed.
10. **The IEEE §4.3.1 overflow tie**, one of four items files 80 and 85 genuinely closed which left the ninth
    consolidation's open list **without appearing in its closed list**, in a document whose own preamble says
    items closed this stretch are named once so the next member does not re-open them.
11. **The OCP mode split**, from the same four.
12. **`Crosses`'s second read**, from the same four, and carried as owed on independent grounds at section 5.
13. **Statement 0 against `quantize` and `roundToIntegralExact`**, from the same four, and carried as owed on
    independent grounds at section 5.

**Naming them is the whole point: an item leaves the open list when it is answered or explicitly droplisted,
never by attrition, and an item this document could not verify is an open item rather than a silence.**

**One thing file 109 explicitly could not do and this document does not do either**: it checked that material
left the standing base, **not that any of it was correct**. A clause that vanished may have deserved to. Where a
droplist entry would have said so, that is precisely the entry that does not exist, and section 6's entries
carry their reasoning so a member who believes a retest would come out differently knows what has to be
overturned.
---

## 9. The correction ledger: the fifty-one repairs, folded above and recorded here

The eleventh consolidation carried **fifty-one inline correction blocks**, fifty labelled "Correction, file
NNN" and one labelled "Finding, file 118", interleaved with the prose they corrected. Every one is folded into
the prose above and every one is recorded here: what was wrong, what it now says, which file found it, and on
whose agreement it landed.

**How to read the agreement column.** A repair marked *audit* was found by an instrument checking the standing
base against its own sources and applied by the restoration pass; the agreement is the audit's citation plus
the source it was checked against. A repair marked *compiled* rests on a diagnostic reproduced under the pin. A
repair marked *two agreements* is a call about the canon and carries the two independent reads this panel
requires, each grounding its own answer before reading the other. A repair marked *stated, not decided* records
something without ruling on it, and the item is on section 2's list.

### Section 0, the standard and the provenance

**1. The conventions mechanism's shipping half.** Section 8 of the standing base recorded D67's `conv-*`
feature-gated alias sets as not restored "because op's standard from `13c` is what it was reaching for". That
reason covers one half of a two-half decision: `13c`'s standard is an acceptance test for the review and says
nothing about what ships, while D67's first half is a shipping mandate naming feature-gated alias sets on named
crates. It was dropped with no successor and no entry, in a sentence that read as though the whole decision
were absorbed. **Now**: section 0.1 states both halves, and the `conv-*` features are named as input to the
taxonomy round. **Found by** file 113 (`113:152-181`), **applied by** file 114. **Audit**, against op's own
`talk:1454-1470`.

**2. The three posture directives cited to the wrong document, and four absent entirely.** The novelty posture,
the convergence directive and the constructive-deliverable directive were each cited to the last document that
carried them rather than to op's own file, which is the method defect file 111 names and the standing base
committed three further times on op's text. The four `16b`/`16c` directives were absent: seven searches for
their wording returned zero. **Now**: section 0.2 carries all seven from op's own files with the last carrier
named beside rather than instead. **Found by** file 112 (`112:232-234`, `112:414-427`), **applied by** file
114. **Audit.**

**3. The oracle enumeration omitted op's own register.** The provenance section listed twenty-three checkpoint
files and not the three topic files carrying op's numbered decisions, which is the structural cause of the
register drift: the standing base's own definition of its oracle excluded the register, so nobody diffed it,
and the agreement rate by number came out at fifteen of forty-six. The persona list also ended with "and
`101b`'s siblings" after `101b` had already been named, reading as a trailing group that does not exist.
**Now**: section 0.4 names all three topic files as governing, and the persona list is nine files and closed.
**Found by** file 113 (`113:14-20`, `113:44-50`) and file 112 (`112:90-92`), **applied by** file 114.
**Audit.**

**4. The citation convention, evaluated before adoption.** The standing base carried a three-part proposal for
disambiguating the register's colliding identifiers and did not evaluate it. Checked against the three frozen
files: **file-qualification cannot work**, because both `D1` through `D4` runs live in one file, so it is
exactly as ambiguous as a bare citation and worse for reading as though it had been disambiguated;
**round-qualification works** and is derivable by reading upward; and it is **already op's own practice in
three places in his frozen text**. The two register-side halves (renumbering, minting numbers for unnumbered
decisions) are declined, one because it edits frozen text and one because it mints identifiers with no
definition site. **Now**: section 0.4 states the form, applies it to every `D`-number in this document, and
leaves the question grid's own marker open as op's. **Found by** file 121 (`121:194-268`). **Compiled** in the
sense that the refutation rests on two line numbers in one file rather than on a grep.

**5. The review's own mode and stopping condition did not exist in any consolidation.** Nine searches for op's
stopping-condition wording returned zero across all ten; `stub` occurred eight times meaning documentation
stubs and none meaning op's source stubs; `lowering transparently` and `optimal instructions` returned zero;
the cadence and the licence-to-argue qualifier returned zero. All four are op's own text on the ratified rung.
**Now**: section 0.5 states the four-step cycle, the post-canon four-phase sequence, the cadence with its two
drifts, the licence to argue, and the end state. **Found by** file 112 (`112:159-162`, `112:197-199`,
`112:217-218`, `112:352-353`, `112:370-371`), **applied by** file 114. **Audit.**

### The design rules and the registry

**6. The spine rule's eleven firings were quantified over and never enumerated.** The design's strongest
methodological conclusion was drawn from the number, and its provenance ran through `78:120-129` to
`68:98-101`, which reads "Nine occurrences stand from the sixth consolidation's count", so the chain terminated
in a count inherited from a document rather than in a list. **The list existed**, at `63:106-123`, one
consolidation below the count that replaced it. **Now**: the spine rule enumerates all eleven. **Found by** file
111 (`111:261-277`), **applied by** file 114. **Audit**, against `63:106-123`.

**7. The `Ranged` transfer coordinates lost their figures.** The two negative controls kept their existence and
lost their numbers, and a restored measurement without its numbers is a restored assertion. The
coordinate-collapse sentence and the cleared-by-luck sentence were absent, the second being a statement about
the reliability of the review's own prior work. **Now**: the registry section carries the 8-of-13 and 29-of-51
controls, the six-coordinates-not-seven collapse, and the cleared-by-luck sentence. **Found by** file 111
(`111:151-168`), **applied by** file 114. **Audit**, against `68:473-488`.

### Section 1.1, what a number is

**8. The subsection count disagreed with its own headings.** "Twenty-nine subsections" against thirty headings.
**Now**: section 1 states a range a reader can check against the headings. **Found by** file 115 (`115:167-173`),
**applied by** file 118. **Audit**, repaired by stating a checkable range rather than by moving the number.

**9. `Number` was declared nowhere.** Two cold reads reached that independently, each naming it the largest
single gap and each finding two arities in use with nothing reconciling them. **Now**: section 1.1 declares the
type and its bound, and marks the three-argument spelling as file 09's probe topology. **Found by** files 115
and 116 (`115:23-35`, `116:64-70`), **applied by** file 118. **Two independent cold reads**, which is the
strongest form the finding could take.

**10. The bound, the parameter count and the exponent-form key.** Three items converge on the declaration. The
bound `S: Policy + Lowering` was written by file 118 from `26:28-35` and given its reason by file 120
independently: the conjunction is what makes the weaker bound enforceable. The parameter count was closed on
`26:32-35`'s measured 1.8x diagnostic cost and only the trait count was ever open. The bound gains the
exponent-form key, derived at section 1.21. **Now**: section 1.1 carries the keyed declaration as current and
the unkeyed one beside it as the prior spelling, because op's confirmation of the key is owed and the two
differ by exactly the projection. **Found and landed by** file 122, **on two agreements**: files 118 and 120 on
the bound, files 119 and 122 on the key.

**11. One constant, two spellings, no convention named.** `256/255` at section 1.1 and
`Adjustment = 1/(r^F - 1)` at section 1.11 are both correct under different exponent conventions, `e = -F` and
`e = 0`, so under the widened completeness line the constant was defined nowhere. **Now**: the convention
travels with each spelling and both stand, because both are in use and neither is wrong. **Found by** file 111
(`111:381-388`), **applied by** file 114. **Audit.**

### Section 1.2, the identity contract

**12. The two identity structs did not compile.** Carried as unit structs, they produce seven instances of
`error[E0392]: type parameter E is never used`, reproduced under the pin. This is the identity contract's own
shape, the most-cited code block in the record, and it had never been put through a compiler. **Now**: both
carry `PhantomData`. **Found and repaired by** file 118. **Compiled.**

**13. The repair is better founded than rustc's suggestion, and the two forms differ on the seal.**
`PhantomData` is the design's own idiom at that position, carried by nine other sealed constructors, so the two
identity structs were the odd ones out. And a unit struct is constructible as a value by any downstream crate
while the tuple form refuses at `E0423` across a real crate boundary, so the pre-repair form left a value door
open in a sealed vocabulary. **Now**: section 1.2 states both grounds and records the inhabitedness question as
a refinement rather than a gap. **Found by** file 121 (`121:64-99`). **Compiled**, across two crates, which is
the only arrangement in which the distinction exists.

**14. Which shape is current, stated on the block's own face.** The reconciliation between D68's four flat
members and the nested shape was performed in prose while the code block carried no sign of it, so a reader
taking a compiling Rust item as the settled shape had no route to the standing conflict. **Now**: the block's
first line says so and the four paragraphs below it state the supersession and its two postdating grounds.
**Found by** file 116 (`116:178-186`), **applied by** file 118. **Audit.**

**15. D68 was superseded silently.** The standing base declared a shape op did not choose, argued it on
independent grounds, and nowhere said a ratified flat call existed. **Now**: the supersession is stated, not
reversed, and op's acceptance is on the open list, because the two contrary shapes are both his. **Found by**
file 113 (`113:207-218`), **applied by** file 114. **Audit**, against `talk:1529-1545`.

### Sections 1.5 through 1.10

**16. The transfer refutation's second compiled support was missing.** Two supports were named as two and
stated as one; the absorption-freedom result, the four-legs analysis it rests on, and the
necessary-promoted-to-sufficient statement of what is being refuted were all absent, on a document whose
largest pending item for op is three `unstable-features.md` wording edits whose first is exactly this
correction. Op was being asked to edit a ratified workspace rule on half the evidence the panel produced for
it. **Now**: section 1.5 states both supports and the target. **Found by** file 111 (`111:127-149`), **applied
by** file 114. **Audit**, against `68:438-455`.

**17. D39's objection was carried and its answer was left behind.** Op's held call reached the standing base
described only by what was said against it; `Inhabits` returned zero hits. The answer was in the same source
file, twenty lines further on. **Now**: section 1.6 carries objection and answer together. **Found and applied
by** file 111. **Audit**, against `40:227-235`.

**18. The additive lattice closure condition was restored wrong.** It read "bias and adjustment are integers",
a different and stronger condition than the compiled one, which made the following clause vacuous by repeating
its own content and was falsified by the parenthetical in its own sentence, since `AddClosed` gates on
`Bias = Zero` alone and constrains no adjustment. It had also inherited the third consolidation's loss of the
third conjunct. **Now**: section 1.7 carries `bias / adjustment` an integer, and the narrowed-multiplicative
condition with all three conjuncts, from `33` where the probe established them. **Found and applied by** file
111. **Audit**, and it is the origin of the rule that a restoration cites the source rather than the last
carrier.

**19. Every algebra-ladder rung name below `Dioid` was gone.** `Magma`, `Combine`, `Semigroup`, `Monoid` and
`semigroup` all returned zero, while the reconciliation D75 performed survived attributed entirely to D47.
**Now**: section 1.9 carries D75's rename and the ladder's own vocabulary, which the taxonomy round mints.
**Found by** file 113 (`113:274-283`), **applied by** file 114. **Audit.**

**20. The ten axes were quantified over twice and enumerated nowhere**, and at Thread B the stale count is a
premise in a live finding, which is worse than undefined. `12b`'s hold on the axis and its attackability
sentence were absent. **Now**: section 1.10 tables D69's ten axes with the three removals marked, both live uses
of the count are rewritten so neither rests on it, and `12b`'s hold is stated. **Found by** file 113
(`113:414-425`), file 111 (`111:279-284`) and file 112 (`112:399-412`), **applied by** file 114. **Audit**,
against `talk:1621-1641`.

**21. The growth surface's count matched neither of its own candidate lists.** "Eleven operations" against a
parenthetical naming twelve, or thirteen counting the Euclidean pair as two. **The miscount is at the source**:
`51:56-64` says eleven while the probe declares ten growth traits over thirteen operations. **Now**: section
1.10 states both numbers from the probe and carries the enumeration, and notes that nothing downstream moves
because the structural theorem quantifies over every operation the design could have. **Found by** file 115
(`115:175-185`), **applied by** file 118. **Audit**, against the probe file.

### Sections 1.18 through 1.23

**22. `SameFaceAs` read as design vocabulary.** It appears only inside a quoted diagnostic and is the probe's
own spelling; the design content is the lever, restating a comparison as a bound rather than an equality, and
the trait a real implementation reaches for is `NumeralFace`'s coarsening bound. **Now**: section 1.18 says so.
**Found by** file 115 (`115:122-124`), **applied by** file 118. **Audit.**

**23. The `Cold` quotation elided op's own middle sentence**, inside the passage the standing base flagged as
its flagship restoration of ratified material. `seldom` returned zero in every consolidation and `cold path`
zero. The row's justification then read `Cold` as "stores as small as possible" literally, the storage meaning
alone: **the bench target survived and the reason for it did not.** **Now**: section 1.21 quotes `68b:69-73`
whole and states both meanings. **Found by** file 112 (`112:324-346`), **applied by** file 114. **Audit**, and
it is a paraphrase weakening a claim while restoring it, which is the structural failure file 111 names,
occurring inside a restoration.

**24. Two of D71's three stated consequences were absent**, including the signed-fold consequence, which is the
one a consumer meets first and which the design reaches independently elsewhere without crediting op's
statement of it. **Now**: section 1.21 states all three. **Found by** file 113 (`113:365-374`), **applied by**
file 114. **Audit**, against `talk:1702-1715`.

**25. One preset name denotes two rows and the trait table's members cannot carry that.** `Policy::Quantisation`
and `Lowering::StoredWidth` are nullary associated types, so `Warm::StoredWidth` is one type and the ratified
tables say it is `doubled` for fixed-point and `minimum` for float. Under the layer-keying rule that is the
rule's dual failure, a fact keyed on something that does not determine it. Two spellings were offered and
neither was picked, because the record contained neither. **Now**: section 1.21 states the defect as a compiled
`E0119` and resolves it, per entry 26. **Found by** file 118, as a finding rather than a correction.

**26. The fork collapsed rather than being chosen between.** Both offered spellings are refuted by compiled
diagnostics: keying on the numeral over-keys and admits two same-kind numerals disagreeing on one preset;
per-kind markers admit `Number<Binary32, WarmFixed>` at exit 0 and delete the four types D72 names. **A third
spelling, keying both contracts on the exponent form, is the partition the two tables induce**, and the wrong
pairing has no spelling while over-keying is refused by coherence. **Now**: section 1.21 carries the derivation,
the refutations, the compiled evidence, and the cost, and section 2 records that what op owes is a confirmation
rather than a pick. **Found and landed by** file 122, **on two agreements**: files 119 and 122 independently,
each compiling before opening the other.

**27. The assembly's three preconditions were stated in the wrong place.** The eight Rust blocks assemble to
exit 0 only with a feature gate, an import and one out-of-document name, and section 9 is the wrong place for a
reader to meet them, because a reader assembling the blocks starts at the trait table and section 1.1 and hits
twenty-nine errors before reaching anything the document decided. **Now**: section 1.23 states them at the
first block that needs them. **Found by** file 121 (`121:126-134`), **landed by** file 122. **Compiled**,
reproduced independently on a superset of the declarations.

**28. Nineteen bound names were declared nowhere.** A reader could see that `Numeral::Precision` is bounded by
`Precision` and had no way to learn what `Precision` is, and the BNF was the same gap in the other direction, a
closed enumeration of the carrier the whole design rests on, in a comment. **Now**: section 1.23 declares all
nineteen plus the carrier, each from the text that determines it, with `FieldLayout` and `Canonicalisation`
carrying only what is fixed and their remaining members on the open list, because inventing those spellings
would put a decision nobody made into the canon's voice. **Found by** file 116 (`116:81-85`, `116:46-49`),
**applied by** file 118. **Audit.**

**29. D53's alias half had no stated expansion.** D48 and D31 keep the public spelling with widths as const
parameters, the numeral takes `Precision` as a `Nat`, and `I + F` in type position is a const expression the
design has compiled shut. Those two facts do not meet on their own. **Now**: section 1.23 states the bridge, a
generated table plus the tower's own addition, compiled gate-free. **Found and built by** file 118. **Compiled.**

**30. The bridge's result type was spelled with a token declared as a trait.** `Precision<13, 3>` against
`pub trait Precision: Nat {}` is `error[E0782]: expected a type, found a trait`, and rustc's own suggested
repair is `dyn`, which arvo forbids. `NatOf<{ I }>` was undeclared as well. **Now**: entry 31. **Found by** file
121 (`121:159-192`). **Compiled.**

**31. The collision is closed and the closing is not a free naming call.** It refuses one step earlier than the
use site, at the declaration, `error[E0428]: the name Precision is defined multiple times`, reached
independently by two files, so it must be closed rather than tolerated. The branch that renames the marker
trait dies on ratified text: `Precision` is one of three named semantic aliases declared as a family on `74b`'s
own pattern under op's condition that "the mechanism unifies and the vocabulary does not", and renaming rewrites
two bounds and the family's shape for no gain, so `16d:14-15`'s tiebreaker decides it. **Now**: the marker keeps
`Precision`, the result types take `NatOf` and `PrecisionOf`, compiled together, and what remains for op is one
word inside D56. The braces sentence is corrected to its cause: braces are required exactly when the const
parameter's name also names a type in scope, which it does here structurally. **Found and landed by** file 122,
**on two agreements**: files 119 and 122 on the `E0428`, and files 119 and 122 again on the braces reading, one
from each side.

**32. The bridge's emission locus was unstated.** **Now**: section 1.23 states that the table is forced rather
than chosen (three routes compiled shut, the last of them full specialisation, which is the same ban the
model-width transfer argument rests on), that it lives in the crate declaring the sealed carrier by the orphan
rule, that it is emitted by `macro_rules!` rather than a build script by the design's own no-build-harness
rule, that each row carries its own agreement assertion because a compensating change in the addition impl
passes the downstream assertion and is caught only at the row, and that nothing downstream sees it. **Found and
landed by** file 122, **on two agreements**: files 118 and 119 built the table independently and reached the
same mechanism.

**33. Five ratified decisions were absent by number and by content.** **D54** was invoked by name and stated
nowhere, so a reader could neither tell it was op's ratified call rather than a panel coinage nor apply it.
**D64**'s `Quantisation`, **D63**'s `Direction` and the `Resolution` members were four of the six undefined
terms the completeness line found, with `Quantisation` the sole content of `Policy` in the ratified trait
table. **D53**'s alias half and **D52** were absent, leaving a standing base carrying two preset tables and no
sentence saying they are one path among several, which makes Thread A's nominal-constructor work
unintelligible. **D48** and **D31**'s public-spelling constraint was absent. **Now**: section 1.23 states all of
them from the register at source. **Found by** file 113 (`113:289-315`, `113:317-332`, `113:130-150`,
`113:266-272`) and file 111 (`111:279-300`), **applied by** file 114. **Audit.**

### Section 1.25, the crate table and the split

**34. The contract split is settled rather than a live fork.** Two members read op's reserved call
independently, the second forming and compiling its answer before opening the first, and both recommend keeping
the split unchanged. **Now**: section 1.25 carries the four canon sentences, the disagreement between the two
reads about where enforcement lives, the refutation of the trusted-base sentence one of them wrote, the
prohibition's severity reproduced through `Container` rather than `S::Layout`, and the finding that sealing does
not close the residual gap. **Found and landed by** file 122, **on two agreements**: files 117 and 120.

**35. D72 was absent entirely.** `arvo-numeral`, `arvo-policy` and `arvo-lowering` each returned zero hits,
searched four ways, while the open list carried "what `arvo-numeric` ends up containing once the numeral,
policy and lowering definitions move out", presupposing a move whose text was nowhere. An earlier file recorded
D72 as carried by the periphery table and file 113 corrected that at source: the periphery table is the
eleven-row taxonomy and not one of D72's rows appears in it. **The later reading wins because it shows its
work.** D23, D32 and D33 were absent by number and content, and file 09's harder enforcement result and op's
reserved call were absent. **Now**: section 1.25 carries D72's table cell by cell with its two stale cells
marked, plus D23, D32, D33 and D73. **Found by** file 113 (`113:94-128`, `113:256-272`) and file 112
(`112:269-298`), **applied by** file 114. **Audit.**

**36. The periphery table's colliding citations.** Two rows cite the colliding `D1` through `D4` range, and the
prose above the table said which sequence the column is keyed on, leaving the disambiguation to a sentence a
reader has to have read first. **Now**: every cell in the table carries its round. **Found and applied by** file
121 (`121:263-268`). **Audit**, checkable by finding the definitions under `inherited:293`.

### Sections 1.26 through 1.30

**37. D56 was absent.** Six searches for its wording returned zero, and section 1.26's own principle is a
different rule adopted at `90b` about what a name may promise. The two do not conflict and neither substitutes
for the other, and **the taxonomy round mints every name in the design**, so a standing base carrying one of
the two rules licenses half the naming. **Now**: section 1.26 carries both, with op's own applications of D56.
**Found by** file 113 (`113:219-239`), **applied by** file 114. **Audit.**

**38. A bare `D4` in the capacity section.** **Now**: qualified by round, `202607281220`, recording which of
the two live `D4`s it was always about, which is "a shape is a cons-list of capacities" and not the
vetting-record decision. The finding it states is untouched. **Found and applied by** file 121. **Audit.**

**39. A bare `D4` in the shape section.** **Now**: qualified by round, with the second mention in the same
sentence inheriting it; `D43` beside it needs no qualifier, occurring once in the register. **Found and applied
by** file 121. **Audit.**

**40. Five named types used without definition, and the right repair was to leave them undeclared.** A cold read
flagged `Bool`, `USize`, `Cap`, `NUSize` and `BoolResidual` as used without definition, which is right about the
text and wrong about the cause: the chapter's subject is how many doors each has, the door that survives is
op's call, and writing a declaration would be writing that call. **Now**: section 1.30 says so explicitly and
names it the one place in the document where a missing declaration is the finding rather than a gap in it.
**Found by** file 115 (`115:107-110`), **answered by** file 118. **Stated, not decided.**

**41. The worked `max` did not compile.** The body read `a.lt(b)` with a by-value `lt`, which moves `b` while
both operands are used again in the same expression: `error[E0382]: use of moved value: b`, with rustc's own
note pointing at the repair. The borrowing form is taken, since the alternative is a `Copy` bound this document
states for no numeric type anywhere. **This is the document's one operational code sample**, presented as the
resolution of a locked fork with a compiler diagnostic quoted in support, and it had gone unbuilt through a full
audit, a full restoration and two cold reads. **Now**: section 1.30 carries the borrowing form, and the
two-lane result was independently re-executed as `[7, 9]`. **Found and repaired by** file 118, **re-verified by**
file 121. **Compiled.**

**42. The truth contract's three organs were undeclared.** The locked statement names an algebra, an exit and a
selector, binds fifteen declarations on the first, and the worked `max` is written against two of them, all
declared nowhere, so a reader following the section's own advice compiled nothing. **Now**: section 1.30
declares `Truth`, `Branch`, `Compare` and `Select`, each member read off the locked statement rather than
chosen, with the two names that are not declarations marked as such. **Found by** file 116 (`116:71-77`,
`116:97-99`), **applied by** file 118. **Audit.**

### Section 2, the width ceiling

**43. The width-ceiling question was three halves and two are settled.** The open-list bullet asked whether a
ceiling on the total width is needed and offered an emitted marker as the mechanism. **A ceiling on each
written width already exists and is forced rather than chosen**, because the bridge is an enumeration and an
enumeration has a largest row, so "no ceiling" is unavailable on that axis under any wording. **No second
ceiling on the sum**, on three compiled grounds: the marker refuses a legitimate product at precision 80, it
cannot be narrowed to the written site because a `where` clause on a type alias needs an unvetted feature, and
its refusal shows eleven numeral trees and no number. **Nothing else forces one**, and the container fix op
withheld at `68b` is the same fix that keeps the sum unbounded. **What is left is one number**, priced at every
plausible value, and file 119's linearity reading does not survive the extension. **Now**: section 2 carries the
derivation and the canon paragraph. **Found and landed by** file 123. **Compiled**, from a scratch tree outside
the repository.

### Sections 5, 6 and 8

**44. Thread B's two positive results were missing.** The standing base carried the thread's three costs and no
mechanism, so a future member opening it read three costs and would rebuild both results. The accounting was
honest and the section text was not: it presented itself as the thread's state and the thread's state included
two positive results surviving in one file in the corpus. **Now**: section 5 carries both, the one-body result
and the join-of-carriers result. **Found by** file 111 (`111:178-197`), **applied by** file 114. **Audit**,
against `11:509-520`.

**45. The fidelity droplist entry droplisted a mechanism and not the principle it was trying to satisfy.**
`17b:19-30` is op's adoption that a fidelity grant is checked rather than asserted, and no sentence in the
standing base stated it, so a reader could learn only that one particular way of not checking failed. **Now**:
section 6's entry states the principle at the entry it governs, with op's own note about what it does not
settle. **Found by** file 112 (`112:466-473`), correcting file 111's earlier report that the adoption was
neither restored nor droplisted nor named open, **applied by** file 114. **Audit.**

**46. "Every item on file 109's list is restored" is a smaller claim than it reads as.** Restoring from an
audit bounds the restoration by the audit's recall, and file 109's list is what it found by diffing
consolidations against each other, so anything that never reached a consolidation is outside both instruments.
Three sweeps have since measured that ceiling. **Now**: section 8 states the ceiling and names the one
instrument nobody has run. **Found by** file 111 (`111:318-324`), **applied by** file 114. **Audit.**

**47. The final accounting's count disagreed with its own list.** "Nine items" against bullets naming thirteen:
one bullet carried two joined by "and" and one carried four in a parenthetical. **This is the document's own
thesis failing in its own final accounting of itself**, restated at least three times and applied to a
predecessor two sections earlier. **Now**: section 8 enumerates thirteen, one per entry, which is the only
repair that does not reproduce the defect. **Found by** file 116 (`116:204-217`), **applied by** file 118.
**Audit.**

### Section 10, the verification

**48. "No term in this document is left undefined or uncited" was false on the document's own text.** `Compare`
and `Select`, the two traits the worked example is written against, were neither defined nor on the open list;
`Number`'s arity was neither resolved nor listed; and fourteen further names were used as bounds with no
declaration anywhere. **Now**: all are declared or named open, and section 10 re-performs the line over the
document as a whole. **Found by** files 115 and 116 independently (`116:139-152`, `116:60-99`, `115:71-124`),
**applied by** file 118. **Two independent cold reads.**

**49. The completeness line had been scoped to "this document's own new prose".** That exemption removes exactly
the population a restoration consists of, while the requirement quantifies over the whole ratifying text. With
it struck, six terms fell out at once. **Now**: the line as stated at the design-rules section quantifies over
the whole text, and section 10 performs it that way. **Found and applied by** file 111 (`111:503-520`).
**Audit.**

**50. All six terms disposed of, from the source that established each.** The spine rule's eleven enumerated
from `63:106-123`; the ten axes tabled from `talk:1621-1641` with the three removals marked; `Quantisation`,
`Direction` and the `Resolution` members declared from `talk:1243-1266` and `talk:1128-1140`; the transfer
refutation's second support restored from `68:451-455`. **One reading is left standing rather than promoted**:
the ordering of the four resolutions "by how much they lie" is stated in no source and is marked as the
document's own. A seventh term, op's axis-sorting test invoked by name and stated nowhere, is restored from
`talk:352-356`. **Applied by** file 114, **from** files 111 and 113. **Audit.**

**51. The code-block sweep did not exist.** Every compile claim in the standing base was made about a block
someone had reason to build, and no pass had ever extracted the blocks mechanically and tried all of them,
which is why two defects fell out of writing declarations rather than out of looking, and why one survived a
full audit, a full restoration and two cold reads. **Now**: section 10 reports the sweep, its counts, the three
additions the assembly needs, the one defect it found, and the two things a declaration-only exit 0 does not
establish, checked separately. **Found and performed by** file 121 (`121:101-158`). **Compiled.**

### What the ledger shows about the instruments

Fifty-one repairs, and they sort by instrument rather than evenly. **Twenty came from the three audits of the
restoration**, found by files 111, 112 and 113 and applied by 114, and every one of those is a drop, a
paraphrase, or a missing citation rather than a design error. **Three more came from the entailment check
alone**, file 111 applying its own findings inline, and two of the three are the worst entries in the ledger,
a mathematical condition restored wrong and a held call restored with only the objection against it.
**Fourteen came from the two cold reads and file 118, the pass that closed them**, and every one of those is a
declaration that was decided and never written down, or a count that disagreed with its own list. **Seven came
from the mechanical code-block sweep and from checking a proposal before adopting it** (file 121). **Six came
from the landing pass** (file 122), all six carrying two independent agreements. **One came from a dispatch on
a single open question** (file 123), and it settled two halves and refuted the mechanism the question had
offered. Twenty plus three plus fourteen plus seven plus six plus one is fifty-one, which is the number the
section opens with.

**The distribution is the finding.** An audit keyed on what a consolidation carried found drops. A cold read
keyed on what a reader can build found absent declarations. A mechanical sweep keyed on nothing but the text's
own fenced blocks found a defect all three had walked past. **No one instrument would have found the other
two's population**, which is the same lesson section 8 records one level up about the sweeps over op's material
and the register, and it is the reason section 5 keeps three further instruments on the owed list rather than
treating any of them as covered.
---

## 10. Verification

**The canon gate reproduces fresh.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and
the same with `FullRange\|UTerm\|AddWidth` **both exit 1, empty**, run at the arvo repository root on
2026-08-06 for this document, at `e5904b2`. The numeral tower still has no shipped source, and nothing in this
document touches `mock/crates`.

**The toolchain** is pinned in the tree at `nightly-2026-05-28`, which resolves to
`rustc 1.98.0-nightly (57d06900f 2026-05-27)` on `aarch64-apple-darwin`, read from `rust-toolchain.toml` at the
repository root. **Run from outside the tree the identical command resolves to stable `1.94.0`**, which does
not parse `type const` and reports it as an ordinary parse error; four files hit that trap in two opposite
directions, and it is on the owed list as a probe convention.

**The test gate is inherited rather than re-run, and the inheritance is stated because the requirement forbids
citing another file's search as performing one.** `cargo test --offline --workspace`, summed per binary rather
than taken from a headline, reports **155 binaries, 672 passed, 0 failed, 9 ignored** from a clean committed
tree, and that count is identical across files 102 through 108, 118, 119 and 122, each of which ran it
independently. **This document did not run it.** One of those runs is worth recording for its method rather
than its number: a first attempt wrapped the run in `timeout`, which does not exist on that host, so the
pipeline's exit status came from `tail` rather than from `cargo` and reported exit 0 having run nothing
(`122:38-45`). **A green line whose provenance is a pipeline's last stage is not a measurement**, and that one
was caught only because the harness printed no test-result lines to count. **The number is also the weakest
evidence in the record**: it measures a tree the canon replaces, and the instrument that measured anything in
the repair stretch was the compiler, more than fifty times across files 117 through 123.

### The freshly-performed-search requirement, performed

Every universally quantified negative in this document was re-run over the panel directory's `.md` files on
2026-08-06, not inherited. The results:

| term | `58` | `63` | `78` | `91` | `102` |
|---|---|---|---|---|---|
| `TowardNegative` | 0 | 0 | 4 | **0** | **0** |
| `Implicit` | 19 | 7 | **0** | **0** | **0** |
| `Ranged` | 24 | 9 | **0** | **0** | **0** |
| `tree-meaning` | 0 | 0 | 2 | 1 | **0** |
| `unargued` | 0 | 0 | 2 | **0** | **0** |
| `IS_EXACT` | 2 | **0** | 0 | 0 | 0 |
| `integer k` | 1 | 0 | **0** | **0** | **0** |
| `ExactWindow` | **0** | 0 | 0 | 0 | 0 |
| `Specials = None` | 1 | **0** | 0 | 0 | 0 |
| `wire format` | 0 | 0 | 1 | 1 | **0** |
| `FormOf` | 0 | 0 | 0 | 0 | 0 |

Every bolded cell is a negative this document asserts, and every one reproduces. `tree-meaning` at `91` is the
single compressed mention in a list that the prohibition's own account describes, not a statement of the
prohibition.

**The honest limit, stated because it is not boilerplate here and because this document has the largest search
surface of any file in the chain**: these searches verify that this document's negatives were checked with
**this document's vocabulary**. They do not verify that a discussion using none of these terms exists somewhere
in one hundred and twenty-three files. **The repair stretch produced six demonstrations that a grep's
vocabulary is what fails**, one of which corrects a claim in the audit that made re-running them a requirement
(section 8). A second reader with different terms is the only check on it.

### The definitional-completeness line, performed over the whole document

Terms this document introduces or leans on, with dispositions. The line quantifies over the whole ratifying
text rather than over new prose, per section 0's statement of it.

*Standing base*, *stub*, *pointer chain*, *silent drop*, *compression that lost content*: taken from file 109's
own definitions at `109:666-668` without modification, and used in section 8 in exactly that sense.
*Cumulative droplist* against *reversals*: defined by the split sections 6 and 7 perform, on file 109's own
accounting distinction. *Correction ledger*: defined at section 9's own opening, one entry per folded
correction, with its four fields named there. *Exit*: defined at section 1.30 as the route from a truth value to
Rust's `if`. *Reduction*: defined there as a map from a multi-lane truth value to a one-lane one that is not a
coordinate projection. *Blending selector* against *thunked selector*: both defined at section 1.30, and
explicitly given different names because they must not share one. *Placement map*, *occupancy*, *extent*,
*interior hole*, *derived* and *declared placement*, *internal* and *foreign bitfield*, *element stride*: all
defined at section 1.29, with element stride explicitly distinguished from the group stride `G`. *Risk
annotation*: defined at section 1.30 as the axis orthogonal to the three rungs. *Naming door*: defined at
section 1.30's charter sentence. *Capacity-producing operation*, *derived storage* against *paired storage*,
*falsifiable surface*: all defined at section 1.27. *Forcing mechanism*: defined at sections 1.27 and 1.29 as
the syntactic construct causing a const-position fact to be evaluated, distinguished from *placement*, which is
where the fact is written. *Loss* against *error*: defined at section 8. *Round-qualified citation*: defined at
section 0.4, with its mapping stated and its derivation rule given. *The dual failure of the layer-keying
rule*: defined at the rule's own statement and instantiated twice, at sections 1.21 and 1.30. Used from the
record without redefinition: the four design rules, the three requirements, the five grounds, the four transfer
grounds, the three width levels, the three maps, statements 0, P and C, the only-door projection, the
byte-sharing law, the deletion test, the site count and the moved count, working shape, overtake,
ratified-unread, byte owner, write granule, residence.

**The seven terms the eleventh consolidation used and defined nowhere are all disposed of**, from the source
that established each rather than from a later carrier: the spine rule's eleven firings (enumerated), the ten
axes (tabled), the `Resolution` axis's four members, `Quantisation`, `Direction`, the transfer refutation's
second support, and op's axis-sorting test. **One reading is left standing rather than promoted**: the ordering
of the four resolutions "by how much they lie" is stated in no source and is marked at section 1.23 as this
document's own reading.

**The declarations the two cold reads found missing are all present**: `Number` and its bound, the nineteen
bound names, the sealed carrier as declarations, `Folded`, `Lowering::Container`, `Truth`, `Branch`, `Compare`,
`Select`, and the bridge's `NatOf` and `PrecisionOf`.

**Named open rather than defined**, each with its line in section 2 or section 5: which level a bitfield's
declared width is; the truth contract's own name; the platform crate's name, deferred to the taxonomy round;
the required-field relation; whether a *naming* guarantee counts as a guarantee for the route-multiplicity
discriminator; the convention parameter for `arvo-geom`'s model-choice half; whether the `TotalOrd` split is
two traits or one trait with two methods; `Cmp`'s codomain, the ordering carrier the tower's comparison
projects to, named at four sites and given a result type at none; `FieldLayout`'s members past its extent and
`Canonicalisation`'s members, both described by the facts they carry and never by a spelling; where `Folded`'s
witness rides; where the container projection lives; `W_MAX`; the question grid's own citation marker; and the
three one-token-two-meanings naming calls (`Exponent` against `ExponentForm`, `Dec` against `PosPred`,
`PrecisionOf` as the bridge's spelling).

**No term in this document is used undefined and unlisted**, subject to the same honest limit every other
performance here carries: it verifies that this document's terms are placed, not that a third reader with a
different vocabulary finds nothing. That limit is not decorative. It is exactly the check that failed at the
eleventh consolidation, where the identical line was asserted and was false at four names on the document's own
text, and where two independent cold readers found it and the author had not.

### The separation requirement, performed

Four models this document states as its own, each checked where the distinction is nonvacuous and each stated
where it is vacuous.

**First, section 6 against section 7**, the split between a removal from the standing base and a reversal of an
in-stretch proposal. **Nonvacuous at exactly the twenty-one entries file 109 found in its second column against
the sixty-six in its first**: an entry retiring a proposal a member made and a later member killed reports on
nothing that left the base, and an entry retiring a predecessor's own text is the only kind that does.
**Vacuous at a consolidation that absorbed nothing**, where the two columns coincide, which is why no
consolidation before the fifth needed it.

**Second, section 1.27's three columns.** **Nonvacuous at exactly a capacity the type system produced**: under
the paired form the generic signature cannot be spelled at all, under the const form the operation is refused
outright, and under the derived form it compiles, so the three columns give three different answers to one
question. **Vacuous at a capacity a human declared**, where all three deliver the same storage, the same size
and the same codegen, **which is why twenty-three files carried the ratified answer without strain and why an
eight-capacity sweep could not have separated them either.**

**Third, section 1.30's algebra-against-exit split.** **Nonvacuous at exactly two lanes**, where the algebra's
homomorphisms out are the two coordinate projections and neither `all` nor `any` is one, so a contract carrying
both the algebra and the exit must decide something the algebra does not determine. **Vacuous at one lane**,
where the unique homomorphism is the identity and the exit is free, **and one lane is where every returning site
in the design sits today**, which is why the fifteen declarations could be counted and read without the
distinction ever arising and why the check had to be run at two.

**Fourth, this document's own model: folding a correction into prose while keeping its trail.**
**Nonvacuous at exactly a correction that changes what a sentence says**, where the folded prose and the ledger
entry carry different information (the current statement, against what it replaced and who checked it), so
deleting either loses something. **Vacuous at a correction that is purely additive**, of which there are four in
the fifty-one (the paragraphs at sections 1.18, 1.27, 1.30 and the second declaration block at 1.30 that their
own markers describe as adding without changing the text above), where the folded prose and the ledger say the
same thing twice and the ledger entry is bookkeeping rather than information. **The split is worth performing
anyway**, because a reader cannot tell which kind a given correction was without the entry, and four of
fifty-one is not a rate at which the distinction can be assumed away.

### The table-diff obligation, performed

Every table above was checked against the prose of the section it sits in and against the source that
established each row, before it stands. Four things the diff caught rather than a source naming them: the
preset tables are transcribed from `78:409-441` cell by cell rather than from any later paraphrase, because
every later document carries a sentence and not the cells; the periphery table's `arvo-capacity` row carries the
reopened array-grammar question rather than the ninth consolidation's settled phrasing, which the array-grammar
finding makes stale; the three-column capacity table's compile-time row is the isolated measurement rather than
the first one, because the first measured a constructing body rather than the type machinery; and **every cell
of the periphery table's decision column now carries its round**, which is the citation convention applied to
the one table where the collision actually bites.

**One table is new and its rows are checked against the ledger's own scope.** The negative-search table above
is the output of one command run for this document, and every bolded cell corresponds to a negative asserted in
the prose; a cell that is not bolded is context rather than a claim.

### The code blocks, and what this document did and did not do to them

**This document reproduces every Rust block from the standing base verbatim and compiled none of them.** That
is stated first because it is the load-bearing limit of this section. Mechanical checks that were performed:
the blocks were extracted by position from both documents and compared, **twelve Rust blocks in each, all
twelve character-identical**, so every compile claim the standing base earned transfers and none of it is
re-earned here.

**The assembly's provenance is two passes rather than one, and the split matters.** File 121 extracted the
blocks by position rather than by reading and found **eight compilable Rust blocks at that time, none of which
builds standalone** (every failure `E0405`, `E0425` or `E0433` on a name another block declares, which is the
expected shape for a specification whose declarations are distributed across its sections and is a report about
the format rather than the content), **assembling to exit 0 under the pin** with one dead-code warning on
`Number`'s own field. **Four further Rust blocks arrived with the landing pass that followed** (the
exponent-form-keyed `Number` at two sites, the assembly preamble itself, and the `NatOf`/`PrecisionOf` pair),
each compiled by that pass at the point it was written, and **no single sweep has covered all twelve at once.**
That gap is small and it is real, and it is named rather than papered over. **Three additions are needed to
assemble the set and this document states them at the first block that needs them** rather than here (section
1.23), because a reader assembling the blocks starts there.

**Two things a declaration-only exit 0 does not establish, both checked separately rather than assumed.**
**First, that the tower carries an instance**: a concrete `Numeral` built from the declared constructors
compiles and `<<U13F3 as Numeral>::Precision as Nat>::VAL == 16` passes as a const assertion, so the type-level
readout the whole design rests on is not vacuous. **Second, that the worked `max` does what section 1.30 says**:
implemented at one lane over a `bool` wrapper and at two over a two-lane mask, both compile, and the two-lane
result at `a = [7,2]`, `b = [3,9]` is `[7,9]`, which is the answer that section states and neither of the two
wrong reductions it prices.

**The one defect the sweep found is closed rather than carried.** Section 1.23's bridge paragraph spelled a type
this document declares as a trait, `E0782`; section 1.23 now gives the bridge's result types their own names,
compiled, and section 2 carries the one word that remains op's.

**The block inventory, counted rather than asserted: twenty-three fenced blocks, twelve Rust and eleven
diagnostic transcripts.** The Rust twelve are section 1.1's two `Number` declarations (the keyed form and the
prior spelling), section 1.2's identity contract, section 1.3's `Encoding`, section 1.21's keyed `Number`,
section 1.23's assembly preamble, trait table, sealed carrier and quantiser block, the bridge's
`NatOf`/`PrecisionOf` pair, and section 1.30's `max` and its four truth-contract organs. The transcripts are
section 1.2's `E0423`, section 1.18's diagnostic-lever quotation, section 1.21's three (`E0119` at the nullary
declarations, `E0277` at the wrong pairing, `E0119` at over-keying), the bridge's four (`E0782`, `E0428`,
`E0747` and the corrupted-row `E0080`), and section 1.25's two (`E0220` and the `WideEnough` leak).
**Diagnostic transcripts are not compile targets** and are reproduced as evidence, exactly as they were quoted
in the sources that produced them.

**What this does not establish.** The blocks compile and one instance runs through them; **that is not a
statement that the shape is correct**, and no sweep is evidence for any verdict this document reaches. It also
cannot see a block that should exist and does not, which is the failure mode the two cold reads found and no
compiler will.

### The honest limit of all four performances

They verify that this document's own terms are placed, its models have content, its negatives were searched,
and its tables were diffed. **They do not verify that every verdict above is correct.** This document is a fold
of one standing base and thirteen files that repaired it, and it is not a fresh re-read of every probe any of
them produced. Where a verdict rests on a predecessor's own compiled or measured claim rather than on an
independent re-derivation, the citation says so.

**Three residuals, named rather than smoothed.**

**First, the instrument gap has not closed.** No sweep has been run over the ninety-nine numbered member files
and the probe directories for material no consolidation ever absorbed, which `109:643-646` named as owed and
larger than its own list and which `112:541-546` confirms is untouched. The register diff has been run against
the two topic files carrying D53 onward and **never against the inherited-state file**, which carries
fifty-two decisions plus the duplicate sequence. And the restoration ledger keyed on source ranges rather than
last carriers is still owed as an instrument; **section 9 is that shape at one document's scale**, which is not
the same thing and does not close it.

**Second, this document ran no compiler.** Every "compiled", "exit 0" and quoted diagnostic above is a
predecessor's, carried with its source. That is the correct handling for a fold, and it means the strongest
class of evidence in this record is the class this document contributed none of.

**Third, the format's own new half is untested.** Folding fifty-one corrections into prose is a compression, and
the rule this archive earned is that a claim that a compression entails the prior text is checked by someone
other than the author of the compression. **Section 9 is the mechanism that makes that check cheap** (one entry
per correction, naming what changed and who found it, so an entailment check is one paragraph against one
block), **and it is not the check.** The check is owed, and it is the third step of op's own cycle applied to
this document: a reader given only this file, with the transcripts withheld.

**Only op's calls are final, and even those go stale when their evidence moves.** Everything above that is not
cited to the ratified rung is evidence and suggestion.
