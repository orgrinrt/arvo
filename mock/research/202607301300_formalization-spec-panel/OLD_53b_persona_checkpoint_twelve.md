# Persona checkpoint twelve: five calls, made in op's place

**Date:** 2026-08-04
**Position:** after `53_torvalds_does_it_still_earn_its_keep.md`, immediately before consolidation five.
**Provenance:** same as `48b`. Op is asleep and delegated tonight's checkpoints to his persona at Fable
tier, by the instruction recorded in that file. Every call below is **persona-decided, not
op-decided**. Op reads this and any line dies the moment he says otherwise. The final synthesis still
waits on his word.

The persona read `49` in full, op's four checkpoints, `48b`, and files 50 through 53 in full, and
independently verified three cited claims at source before deciding: the `49:117` const spelling, the
63,665 ms figure in file 41's committed CSV and its absence from all prose, and the absence of a
`float_algebraic` row in `unstable-features.md`.

## 1. The cost model: printed in the spec, with the worst number in it, and the cliff goes on the attempt list rather than the accepted list

**The two-term cost-model paragraph is adopted as spec text**, with file 53's coefficients (2.1 ms per
distinct dyadic composition, 143 ms per distinct arbitrary 16-bit rational, roughly 28 ms per repeated
site, linear throughout), marked as measured on this pin and host and re-priced when a real consumer
crate exists. The paragraph prints the worst measured aggregate in words rather than only the marginal
rate: one hundred distinct arbitrary rationals is 14.3 seconds, four hundred is 63.7. File 53's
diagnosis of the quiet-reassurance pattern is correct and the fix is what he says it is, a number a
reader feels. A figure that sat in a committed CSV for twelve files while the prose quoted the per-unit
rate is the same compression defect file 44 diagnosed, wearing a denominator.

**The consolidation's "harder, more realistic case" wording is corrected.** The 16-bit random-rational
sweep is the adversarial worst case, not the realistic one; the realistic profiles (dyadic quanta,
division by real constants, occasional chained conversions) live two orders of magnitude below it, and
the design's own three headline constants cost five milliseconds together. Realism was backwards and
consolidation five says so plainly.

**The cliff is documented, not accepted as final.** The toolbox rule settles the near half: arvo
documents tradeoffs and does not police them, so a code generator that chooses to emit four hundred
distinct arbitrary rationals does it with the number in hand. But the profile that pays is the
MATLAB-import profile, and MATLAB interop is the axis's stated reason for existing. A cost that lands
hardest on the axis's own founding use case does not get filed under "documented property" and
forgotten. Per the `34b` posture, "whether the per-composition verification cost can be made cheaper
for the bulk-import profile" goes on the open list as an attempt rather than a limitation: nobody has
tried, which is an absence, not a wall. Worst case the attempt fails and the documented paragraph
stands, which is where we already are.

Repetition-is-not-free is adopted with it. Two coefficients in every future cost statement, per
distinct composition and per additional site.

## 2. The licence: the design shape is adopted; the feature gets vetted before anything ships under it

**The starve-what-safety-does-not-prove shape is adopted as spec text.** Grant `reassoc` on proven
interior safety; discharge `nsz` by reading the target numeral's own `Canonical` fact; starve
`contract` by routing MAC-shaped folds through the exact widening path, which is not a workaround but
the design's own existing accumulator construction answering a question it did not know it had; exclude
`fold_compensated` structurally by the combinator distinction the grade projection already makes at the
type level. The one-instruction destruction of the Kahan step (`fsub s0, s1, s1`, always zero) is the
best correctness cliff this stretch found and it was compiled rather than argued, which is why the
exclusion is structural rather than advisory. The four receipt clauses land in section 1.19's
downstream contract beside file 50's environment receipt, and file 53's caution is adopted with them:
this is contract text owed to a build layer, and no receipt plumbing gets built inside arvo ahead of a
consumer. The `51` codegen regression pair (interior-safe `fold` vectorises, `fold_compensated` on
identical data stays scalar and unfused) joins the owed test list so a toolchain change cannot leak the
licence across the combinator boundary silently.

**`float_algebraic` goes through the vetting procedure before it is anything more than spec text.**
File 51 did the honest thing: read the tracking issue and the stabilization PR directly, recorded a
clean reading (safe fn, const fn, no soundness concern, an open stabilization PR, and, tellingly, the
safe half of a split whose unsafe half rustc's own maintainers refuse to stabilise at all, which is the
`min_specialization` pattern pointing the right way), and declined to treat one member's reading as a
ruling. Correct. The next stretch runs the second independent vetting read; if it concurs, the row
lands in `unstable-features.md` as allowed, flagged to op because feature-table edits always are. This
is not the "genuinely unclear" case that is human-only by the rule's own terms, so it does not idle
waiting for morning, but nothing ships under the gate before the row exists. No shipped source exists
yet, so nothing is blocked by sequencing this correctly.

## 3. The two revived items: both adopted as next work

**The L2/L3 consumer-typing dispatch runs.** File 04's `Precise` exile is the oldest unanswered
consumer question in the review, it has grown teeth (the fold combinators now publish grades, and a
topo sort accumulating rank weights has to say which combinator it folds with and what the published
grade means for the returned ordering), and file 47 proved the method that finds exactly this class of
defect. The four algorithm crates are the design's oldest real consumers in its own repo and they have
never once been typed from the outside. This is the highest-leverage consumer dispatch available.

**The fixture sketch runs before the notation macro's vehicle sketch.** File 53's merge of the
decoder-ring diagnostic with the face question into one item is right: their ceiling is the same
sentence ("the first error in an expression names the face"), they share a fixture corpus, and the
macro's design should know whether it emits aliases or faces before its vehicle is sketched. Both are
sketch-decidable; the ordering is a cheap reversible call and it is made.

## 4. The consolidation defect: fixed in five, and the diff becomes a standing obligation

**`49:117` is corrected in consolidation five: the exponent bounds are spelled as types.** The fork
hardens with it. File 50 is the second read plus the compile that section 3 asked for, it arrived at
the same answer by an independent route (the stronger shape of corroboration, honestly flagged as
such), and the const route is compiled shut behind every permitted feature. The exponent-as-type
commitment moves from reasoned to settled, persona-level, **flagged as a ratified-table edit exactly as
loudly as the `Int` drop**, one line for op to reverse. The carve-out file 50 was honest about stays
open: whether `Implicit`'s single exponent moves to a type at the same time was not tested and is not
settled by this.

**The generalisation is adopted as a standing consolidation obligation**: every declaration line in a
consolidation's tables is a claim like any other, and one member of every consolidation pass diffs the
tables against the sections they compress before the document stands. Minutes of work, and `49:117` is
the exhibit of what skipping it costs. It stays a review convention for now; the persona recommends
graduating it to a workspace rule, since it is file 44's defect class recurring inside the document
built to prevent it, which makes it workspace-shaped rather than panel-shaped. That graduation is op's
to confirm. The registry-rendered-tables tier stays named-not-built, where 49 left it.

**File 50's struck-versus-unknown amendment is adopted with it.** Strike the claim, keep the question,
let the registry hold it under `unknown`. "Never derived" and "answered in the negative" are different
states, and file 50 is the case that shows the difference: four threads converged on the float model by
coincidence when an `unknown` row would have been the marker they converged on. Retroactively moot for
the float member, which now has its derivation, its control, and its restored place in the band
sentence; the convention applies forward.

## 5. Direction for the next four

Consolidation five is written first: absorb 50 through 53, restate the band sentence with all four
members now derived, fix `49:117`, print the cost-model paragraph, fold both receipt families into
1.19 (whose "untouched by every deliverable" sentence is stale in the harmless direction and is
corrected), take the one-sentence `Policy` defence and the `Specials` witness-or-`unknown` demand from
file 53, and carry the table-diff obligation in its own verification section.

Then the four, ordered by what unblocks the most:

1. **The float model's type-level join, then decimal.** Build the `Specials`-carrying type-level
   numeral, run the files-30/31 crossing check against it, then the radix-ten instance, which exercises
   the crossing contract's injectivity statement for real rather than vacuously. Both halves of the
   join exist; this completes the keystone.
2. **The L2/L3 consumer-typing dispatch**, carrying file 04's finding with its graded-fold teeth.
3. **The fixture sketch** (decoder-ring plus faces), then the notation vehicle sketch, in that order,
   sharing one corpus.
4. **The bench and audit debt**: the software-quantiser-against-`fadd` bench in `mock/benches/` under
   the harness with the subnormal fraction swept (file 50's one afternoon), the codegen-flag audit file
   52 scoped (whether `-C codegen-units=1` is the standing flag per question class, swept across the
   review's other instruction-count claims), and the licence-leak regression pair from call 2.

The membership second read and the `float_algebraic` second vetting read ride along with whichever
member has slack; both are reads rather than builds. Per-application against per-value-moved stays
exactly where `48b` left it: genuinely op's, declined again.

## What none of the questions asked, and op would have caught

**File 50's clause 2, "the default lowering of a `Ranged` operation is the software quantiser," is
adopted with one thread missing from it, and the thread is the strategy axis.** The derivation is
airtight as far as it goes: flush-to-zero and a moved rounding mode change delivered values, `Lowering`
changes no value by ratified invariant, so an unpinned hardware-float lowering is not a `Lowering` at
all. Nothing to argue. But stated as a blanket default it reads like a policer's posture wearing a
proof: a `Hot` float consumer's entire declared intent is hardware semantics at hardware speed, and the
design's own answer to who picks that trade has always been the strategy marker, never a global
default. The right shape is that the strategy preset selects the door: the receipt-carrying hardware
lowering is presumably what `Hot`-tier float presets carry as their declared environment requirement,
and the software quantiser is what the semantics-first tiers carry. Nobody has threaded `S: Strategy`
through the lowering choice, and file 50's own owed bench (software quantiser against `fadd`, subnormal
fraction swept) is exactly the measurement that conversation needs. Consolidation five records the
clause with this refinement attached rather than swallowing the blanket form, and the threading
question goes on the open list.

**File 52's honest limit on the seal enumeration deserves not to be lost**: the four-introduction-route
exhaustiveness is verified as "every attack found lands in one of four" and not as "four is the whole
space." That is the correct grading and section 1.12 should carry it as stated, `pin`-grounded, rather
than let the stronger reading calcify.

Beyond those, the four files are clean. Fifty and fifty-one both marked their reasoning honestly,
fifty-two audited its own artifacts against the question it was dispatched to ask of others, and
fifty-three re-judged its own file-04 verdicts by name instead of restating them, including burying
one. That last is the convergence directive working as intended.

## Standing

Convergence and the novelty posture hold unchanged. The intent outranks every instruction, is vague on
purpose, and is inferred rather than read literally. Only op's calls are final, and none of the above
is one: these are the persona's calls, grounded in the files and in three source-verified citations
rather than in anyone's summary, and every one is one line for op to overrule.

## Loudest for op's morning read

1. The exponent-as-type ratified-table edit (call 4).
2. The strategy-axis refinement on file 50's lowering default (the unasked item).
3. The `float_algebraic` row in `unstable-features.md`, once the second vetting read lands.
