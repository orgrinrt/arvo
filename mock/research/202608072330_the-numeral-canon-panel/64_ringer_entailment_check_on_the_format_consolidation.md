# 64. Entailment check on the format-concept consolidation

**Date:** 2026-08-09. **Position:** entailment check on `63`, unit two's consolidation of `55`
through `62` on "what is the one format concept, and what must it cover". Third run of this
instrument in the panel, after `21` (the map) and `31` (the option register). `54` ran it on unit
one and found a real gap despite a clean-looking consolidation; that is the pattern I hunted for
here and did not find in the same shape. What I found instead is a completeness gap of a different
kind, described below.

**Gates.** Canon gate: passes, situation two. There is no ratified canon; `00_brief.md:8-9` states
the panel is writing the first one, and this file proposes no mechanism and settles nothing. Test
gate: no suite exists (`mock/crates` is the nuked tree the brief describes); the substitute is the
probe-rerun discipline applied below. **Never cite `63`, `OPTIONS.md` or `INTENTS.md` by line**,
per the brief; citations into them are by section plus a `grep -F`-verified phrase.

## Method, stated so the two measurements are reproducible

Worked from the ten member files forward, not from `63` backward: read `55` (both phases), `55b`,
`56`, `57`, `57b`, `58`, `59`, `60` (both phases), `61`, `62` end to end before opening `63`
again for the check, so what counted as "what the unit established" was fixed before I looked at
what the consolidation claims to carry.

**Rerun.** Spot-checked five of `63`'s thirty-two claimed byte-identical reruns, chosen across
five different member files rather than clustered in one: `57_probes/p2`, `58_probes/p2`,
`60_probes/p_d`, `61_probes/q1`, `62_probes/p1`. All five diffed empty against the outputs
committed in `63_probes/rerun/`.

**Cold-derivation history.** `63` claims `55` phase one is committed before phase two, and
`60` phase one before phase two, both by commit hash. `git log` on both files confirms both
orderings exactly as `63` states them (`ce13af50` before `22a79c65` for `55`; `ff4cb6a6` before
`f3a266af` for `60`).

**The Q3 grep.** `63` claims `grep -c 'Q3'` returns 0 on `55`, `55b`, `56`, `57`, `58`, `57b`, and
10, 8, 4 on `60`, `61`, `62`. Reran the grep against all nine files directly: exact match, every
count.

**The anchor diff**, per `RULES.md`'s instrument and `54`'s method. Extracted every `file:line`
citation from the ten member files and from `63`, deduplicated, and diffed the sets rather than
the totals: 119 unique targets in the members, 147 in `63` (a rising count, which `59`'s own
finding on unit one's consolidation warns not to trust on its own). 103 of the 119 member targets
do not appear verbatim in `63`. I did not stop at that number, because most of it is exactly the
shape `59` and I both flagged before as a false alarm: citations of `08` and `42` by line, which
`63` explicitly declines to carry because it did not open either file itself and routes every
claim about them through `55`/`56` (for `08`) and `57`/`57b` (for `42`), stating so in its own
section 1. That is not a loss; it is a citation discipline `63` states plainly and I confirmed it
holds (`08:113-117`, `08:630-634`, `42:314-316`, `42:315-316` are all absent from `63`, and `63`
never claims to have read `08` or `42` at the source). Narrowing to probe-file citations
specifically (`NN_probes/...`) rather than prose-line citations: 81 unique targets in the members,
45 in `63`. The overwhelming majority of the difference is formatting (`63` cites `56_probes/q1`
where a member cited `56_probes/q1_two_law_families.rs:10-12`, same target, shorter form), which I
confirmed by checking a sample of a dozen against `63`'s prose and finding the substance present
under the shortened name in every case I checked. I did not check all sixty-odd remaining
formatting variants individually; that is a coverage bound, stated in the coverage section below.

**Verifying my own severe finding before reporting it**, per the standing instruction after a
sibling panel's checker got a severe finding half wrong. The one finding below that reads as a real
gap rather than a formatting artifact (the dropped D-C alternative) I traced to its source line in
`60`, confirmed by direct grep that no trace of it survives in `63` under any spelling I could
construct (`D-C`, `D-B`, `D-A`, `expression template`, `typed object`), and confirmed the omission
is total rather than partial (not summarised elsewhere in different words) by rereading `63`
sections 4.4, 5, 8, 9 and 10 in full a second time looking specifically for it.

## The two measurements, with numbers

**Rerun/history verification: 5 of 5 spot-checked reruns byte-identical; 2 of 2 spot-checked
cold-derivation orderings confirmed in git; 1 of 1 spot-checked internal grep count (Q3 across
nine files) exact.** Nothing in this sample contradicts `63`'s section 1 claims about its own
verification work. This is a sample, not an audit of all thirty-two reruns; see coverage.

**Anchor diff: 103 of 119 prose-line citations from the members do not appear verbatim in `63`,
and the large majority of that gap is citation-discipline by design** (targets in `08` and `42`
that `63` states it never opened) **or formatting compression** (a shortened probe-file name
carrying the same claim). Narrowed to probe-file citations: 45 of 81 targets survive verbatim,
and a spot check of a dozen of the non-surviving ones found the underlying claim present under a
shorter citation in every case checked. I did not find the pattern `54` found on unit one, a claim
carried with its supporting citation silently dropped and nothing standing in its place. What I
found instead is content dropped along with its citations, which the anchor-count instrument alone
cannot distinguish from a citation-only loss and which required reading both documents end to end
to catch. That is the finding below.

## What was dropped

**The D-C alternative (the chain as a first-class typed object) is entirely absent from `63`, and
it is a live, named, unkilled option in the source it consolidates.** `60:260-266` names it in
full: expression templates, the term itself a type, the schedule chosen at evaluation; states its
costs (type sizes grow with expression size, a second API vocabulary, arvo drifting from numerals
toward computation graphs); and states its discriminator against D-B, the direction `63` does
carry (whether any consumer needs to abstract over schedules at compile time; if none does, "D-C
is D-B with ceremony"). `60` also names D-A (`60:245-251`) beside it. `63:566` carries D-A, in the
specific form that matters (op's ruling on I7's chain clause is what would revive it). `63` never
mentions D-C, under any spelling: not the label, not "expression template", not "typed object",
not the discriminator question. I checked `63` sections 4.4, 5, 8, 9 and 10 twice for it.

This is not a false claim anywhere in `63`. Section 6's C9 candidate sentence ("A concept that
hides the adaptation inside each operation cannot state the chain-accuracy intent at all") is a
minimal constraint any of D-A, D-B or D-C would have to satisfy or fail on its own terms; nothing
in C9 forecloses D-C. The gap is completeness, not error, and it is against a discipline the panel
states explicitly for itself: `00_brief.md`'s "keep options open, and never ratify early" section
says "carry them forward, all of them" and names killing an option as a real result that has to be
stated, never a silent narrowing. D-C was never killed. `60` itself treats it as a live shape with
an open discriminator test, not a rejected one. A consolidation whose job is to be the thing op
reads instead of the ten source files has, on this one point, made a three-way live choice read as
a two-way one, and the missing third way carries real and stated costs (`60`'s own words: "arvo
drifts from numerals toward computation graphs") that a reader of `63` alone would never learn
existed as a possibility, let alone as one with a concrete discriminator test attached.

The same pattern shows up once more, smaller. `60:245-251` and `60:253-266`'s companion "open
regardless of direction" material, whether schedules are strategy-implied defaults or
consumer-visible knobs, and whether the storage-versus-compute format distinction (`60` section 8)
is a format-concept axis or a strategy property, is also absent from `63`; I checked for "knob"
and "visible" and found neither anywhere in the file. `60` itself did not flag either item in its
own "what the register should gain" section (section 6), so this is a smaller and more defensible
omission than D-C, but it is the same shape of loss: a stated open question, never resolved by
anyone, that a reader of `63` alone would not know had been raised.

And a third, weaker instance from earlier in the unit: `55`'s section 7 names three explicit
"Alternative decompositions" (Alternative B, format = Q alone; Alternative C, the denotation
function, encoding-first; Alternative A-wide, wrap filed in R) with what would distinguish each
from the winning shape. `63` never uses the labels "Alternative B" or "Alternative C" and never
states, in those terms, which of the three the unit's eventual identity-plus-realisation split
subsumed, which it refuted, and which it left untouched. Unlike D-C, these were substantively
engaged by the unit (Alternative C was directly refuted by `55`'s own probe 3 and that refutation
is carried; Alternative A-wide is what the whole `56`/`55b` two-law-family exchange is about, and
that exchange is carried in full), so the loss here is bookkeeping rather than content: the
argument survives, the label that would let a reader check "was this specific named alternative
addressed" does not.

## What was miscited

**Nothing.** Every citation I checked at the source (five reruns, two commit orderings, one grep
count, a spot check of a dozen probe-file citations, and the specific numeric claims below) matched
what it was cited for. I looked for the `54`-shaped failure (a number carried without its
supporting derivation, or a claim attributed to the wrong file) specifically in the places most
likely to carry it: the rung table (section 3, 4), the numeric counts in section 7, and the
retraction chain in section 8. None of it miscites.

Spot-checked numbers, each confirmed against the source cited: the two-by-two's 476 and 897
(`56`); the sign-confinement 100 intervals, 19 associative (`57`); the 150/153 constant-collapsed
multiplicative exceptions on both sweeps (`57b`); the 96 of 256 and 168 of 256 wrap-section
divergence at `w=4, F=1` and `F=3` (`62`); the 952 signed-saturation additive failures at `w=4`,
stated F-invariant (`62`); the twenty-four-cell H1/H2 evaluation with zero residue (`57b`). Every
one reproduces the source's own number and the source's own scope.

## What is carried faithfully

The retraction chain is the part I hunted hardest and it holds. Every withdrawal or narrowing I
found in the ten member files survives in `63`, correctly scoped to what was actually withdrawn
rather than to a larger or smaller claim:

`55b`'s withdrawal of the wrap-as-domain filing, replaced by the induced-algebra theorem, is
carried (`63:213`). `57b`'s full concession on the `42` framing (its own disposition one: "the `42`
framing: conceded, in full, with no residue I would defend") is carried at the level of detail
that matters, that `42` refuted its own clamp-counting hypothesis and named the surviving one, and
that the prose summary's word choice, not the finding, is what is closed (`63:791-793`). `57b`'s
narrowing of the absorption-coherence identification from unconditional to "restricted to Q" is
carried with the mechanism (`63` section 4.1, citing `61:197-204` for the typing argument that
makes the restriction a statement of the law's subject rather than a caveat). `57b`'s withdrawal
of `57`'s "shared theorem" and its replacement by the H1/H2 frame is carried in full, including the
frame's own rung (ONE EXPERT) and `63`'s own flag that it is "the right kind of claim to attack
next" (`63:397`, `63:637`), which matches `57b`'s own hedged delivery of it. `57b`'s scope
correction to the coarsening attribution (unsigned-only on the converse) is carried via the cube
(`63` section 4.3). `61`'s wrap-ring collapse at `F > 0` is carried with the correct F=0/F>0 split
(`63:795-796`, and the ladder table in section 3.4 is correctly historical, describing the unit's
F=0 measurement at the point in the argument where that was all that had been measured, with the
correction arriving in the sections that follow it, matching the order in which the unit itself
argued it).

`63`'s own self-flagged connection (its author wrote `23`, and `23`'s one surviving register line
became load-bearing for the H1/H2 frame) is stated plainly in the file's own preamble rather than
buried, which is the right instinct for exactly the kind of thing this check exists to catch.

I did not find a single instance of a claim stated more strongly in `63` than its source supports.
Every rung I checked (the standard model's TWO EXPERTS with the shared-literature discount, the
(D,Q) refinement explicitly denied the TWO EXPERTS rung because `55b` conceded on reading rather
than deriving, the phase necessity correctly split into ONE EXPERT plus a concession plus a
construction rather than inflated to TWO EXPERTS) matches the discipline the unit itself used.
`59`'s claim that unit two inflated no rung, which `63` repeats, held up against my own check of
the same rungs from the source side.

## Candidate sentences against permanence and equivalence

Ten sentences, `63` section 6. My own read, from the member files, before consulting `63`'s
verdicts.

**C1 (the standard model).** Permanence: passes. No implementation named; the exact-plus-adaptation
factoring is a mathematical statement about what arithmetic on a format has to mean, and any
implementation either honours it or is wrong. Equivalence: passes, and more strongly than `63`
states: it is the oracle every probe in the unit used to check everything else, so three
independent implementations that disagreed on it would have disagreed on the definition of
correctness itself, not on an incidental choice.

**C2 (identity).** Permanence: passes. Equivalence: passes, with the caveat `63` itself states,
that the (D,Q) refinement is a converged-by-attack result rather than two cold arrivals; three
independent teams reading only the member files (not `63`) might have taken longer to reach it, but
would reach the same place, since the refutation of the tuple-equality alternative (`55`'s probe 3,
two's complement and offset binary denoting the same values) is exhaustive and undisputed.

**C3 (the representable set).** Permanence: passes. Equivalence: passes at the model widths
probed, matching `63`'s own hedge. I would add one thing `63` does not: the phase repair
(`56`'s affine form) was independently missed by two instruments erring in opposite directions
before being fixed, which is itself evidence for equivalence rather than merely for correctness: it
is exactly the kind of parameter three independent teams are likely to under-specify identically,
which argues for stating it explicitly in the canon sentence rather than leaving it inferable, which
is what `63` does.

**C4 (the adaptation slot).** Permanence: passes. Equivalence: passes; the two-by-two classification
is a measured fact about which combinations are inhabited, and any correct implementation of the
concept would reproduce it because it follows from the definitions rather than from a design choice.

**C5 (the criterion).** Permanence: passes. Equivalence: passes, and this is the sentence I trust
most in the whole set, because it survived the most attack: refuted once as `42`'s clamp-counting
form, refuted again as `56`'s unrestricted coherence, and the form that survived both attacks is the
one `63` states.

**C6 (the law frame).** Permanence: passes; nothing in it is implementation-specific. Equivalence:
I would not sign off on "passes" as flatly as `63` does, and I say so with the citation `63` itself
supplies: this is ONE EXPERT, unattacked, and `63` says so in the same breath (`63:637`,
"the claim this file most wants attacked next"). The twenty-four-cell mechanical evaluation is real
and I confirmed it reproduces, but "would three independent implementations behave the same" is a
question about whether the *frame itself* is complete, not only whether its predictions match the
cells measured so far, and nobody has tried to break it the way `57` broke `42`'s condition or `61`
broke `56`'s coherence law. `63`'s own hedge in the rung line is the right one; I would want the
"Equivalence: passes" line in section 6 to carry the same hedge rather than reading as a settled
verdict two lines below a flag saying it is not.

**C7 (the scale asymmetry).** Permanence: passes. Equivalence: passes conditional on Q3, and `63`
states the condition (`63:645`). I checked this is not a silent overclaim: section 9's Q3 entry
does carry the dependency as load-bearing and unresolved, so the hedge in section 6 is not
undermined elsewhere in the file.

**C8 (accumulators).** Permanence: passes. Equivalence: passes at the measured widths, and `63`'s
own honesty about the one-bit constant being ONE EXPERT and the sentence surviving without it is
the right shape for a canon sentence to take: the qualitative claim (no bounded closed form for
multiplication) is what should graduate, the specific constant should not yet.

**C9 (the chain).** Permanence: passes. Equivalence: I would sign off on this differently from how
`63` states it. The sentence itself (width algebra, named adaptation, exactness predicate) is a
minimal requirement, and any of D-A, D-B or D-C that could state I7's chain clause at all would have
to supply something answering to it, so in that narrow sense equivalence holds regardless of which
direction wins. But `63`'s own "Rests on" line cites only section 5, which is D-B's content, and
does not mention that D-A and D-C remain live and undecided. A reader taking C9 at face value would
reasonably conclude the chain question is D-B-shaped and settled in its broad strokes; it is not
settled, one of its two live alternatives is missing from the file entirely, and equivalence for a
sentence this general should be judged against the possibility that a canon eventually adopts D-C,
under which "the chain is (exact ops, edge formats, schedule)" is not quite the right shape (a
typed chain object is not that triple, it is a term that denotes one). I would keep C9's wording,
which survives either way, and add the missing alternative to whichever section is supposed to
carry live options forward.

**C10 (what the concept carries upward).** Permanence: passes. Equivalence: passes; the three
things named (width algebra, named adaptation, exactness predicate) are what `60`'s own statability
argument shows is required for I7 to be expressible at all, independent of which chain
representation eventually wins, so this sentence is actually the more defensible of the two chain
sentences and I would trust it over C9 if only one had to survive.

## What I could not determine

Whether the sixty-odd probe-file citation variants I did not individually check (beyond the dozen
I sampled) hide a second instance of the D-C-shaped loss, content dropped along with its citation
rather than merely re-formatted. My sample found only formatting compression; I did not read all
eighty-one targets' surrounding prose in both documents to rule out a second case.

Whether the register-state claims in `63` section 10 (what `OPTIONS.md` currently carries and
what it should gain) are accurate beyond the one phrase I spot-checked (`OPTIONS.md`'s Q12 caution,
which does say what `63` says it says, confirmed by grep). I did not read `OPTIONS.md` end to end
against `63`'s account of it, since the brief's citation discipline treats it as a live document I
should not cite by line, and a full audit of it was outside what I could complete inside this
dispatch alongside the member-file entailment work, which I judged the higher priority given the
brief's framing.

Whether `55`'s Alternative B and Alternative C, absent from `63` by name, would change any reader's
assessment of the unit's conclusions if restored. My own reading is they would not (both are
substantively subsumed or refuted in the carried argument), but I have not built the case for that
the way I built the case for D-C's genuine liveness, and I flag the difference in confidence rather
than asserting both omissions are equally serious.

## Coverage, bounded honestly

**Read end to end, before reopening `63`:** `55` (both phases), `55b`, `56`, `57`, `57b`, `58`,
`59`, `60` (both phases), `61`, `62`. **Reread in full, twice, for the D-C check specifically:**
`63` sections 4.4, 5, 8, 9, 10. **Read once, whole:** `63` in full.

**Reran:** five of thirty-two probe reruns `63` claims, spread across five different member
files' probe directories, all byte-identical. Did not rerun the remaining twenty-seven; I trust
`63`'s claim to the extent a five-of-thirty-two sample supports it and no further.

**Grepped and confirmed exactly:** the Q3-absence count across all nine relevant member files;
two commit-hash orderings for the two cold derivations; a dozen probe-file citation targets for
substance-under-a-shorter-name; six specific numeric claims against their cited sources.

**Not done:** a full read of `OPTIONS.md` against `63`'s account of it. A check of all
sixty-nine unchecked probe-file citation variants. A build of the case that `55`'s Alternative B
and C are genuinely subsumed rather than merely unmentioned.

**Verdict.** The consolidation entails its sources on every claim, number, and retraction I
checked, and I checked the places most likely to carry an error: the rung table, the numeric
counts a reader would quote, and the retraction chain, which is unusually long in this unit and
the part most likely to be flattened under compression. It was not flattened. What I found is a
narrower failure than `54` found on unit one: not a claim surviving with its support silently
gone, but a live, named, unkilled alternative (D-C, the chain as a typed object) and its stated
costs and discriminator, dropped from the file entirely, against the panel's own stated discipline
of carrying every live option forward. That is real and it is `60:260-266`'s to restore, not
`63`'s to have invented; I would not ask for `63` to be rewritten over it, and I would ask that
whoever next touches the chain material in this topic add the missing third option rather than
build further on the two that survived.
