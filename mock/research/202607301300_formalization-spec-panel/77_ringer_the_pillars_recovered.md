# 77. The pillars recovered: pricing stated absolutely, a scour of the checkpoints, and what survives contact with the design

Talia Ringer, file 77. I wrote files 19, 44 and 69. Files 44 and 69 each diagnosed a defect class in how
this review treats its own provenance; this is the third, and op asked for it directly rather than my
finding it.

**Canon gate.** The governing document for this dispatch is op's own message, quoted in full in the brief,
which is itself a checkpoint-shaped ratified instruction: it names a concern, states the standard the
review answers to, and asks for a scour and a restatement. That is the highest rung available (a human
call, in the loop, right now), so there is no separate canon-conflict to resolve; the work is to check the
review's output against it, not to weigh it against something else. Where I found the review's *wording*
drifting from a pillar without any *decision* yet having gone wrong, I say so as wording drift, not as a
ratification failure, because that is the honest distinction and collapsing it would overstate what I
found.

**What I read.** `68_consolidation_seven.md` in full, twice: once for the pricing question, once as the
base document for the scour. Every checkpoint named in the brief: `04b`, `06b`, `08b`, `12b`, `13b`, `13c`,
`16b`, `16c`, `16d`, `17b`, `24b`, `30b`, `34b`, `39b`, `44b`, `68b`, `70b`, `74b`, and the five persona
checkpoints `48b`, `53b`, `57b`, `62b`, `67b`, all in full. `202607301100_topic.the-formalization-talk.md`
and `202607301200_topic.the-formalization-spec.md` in full, and `202607301000_topic.inherited-state-from-
the-formalization-round.md` by grep-and-read-around for every "compile" occurrence. `arvo-compile-time-
last.md`, `arvo-always-optimal-internals.md`, `arvo-toolbox-not-policer.md`, `harness-the-type-system.md`
in full. `76_kiselyov_the_real_consumer_price.md` in full, because it is the one file in the corpus that
actually prices a compile-cost fork and is the fairest test of whether the pillar survives contact with a
real decision. One `ls` of the panel directory at the start, and a second after finding file 76, to confirm
nothing after it exists yet.

**What is verified against what is reasoned.** Every citation count below is a fresh `grep -rn` run this
session against the panel directory, re-run at the moment of writing rather than carried from an earlier
pass; I quote the commands where the count is load-bearing. The claim that a phrase never appears is
checked by grepping for it, not inferred from not having noticed it. The judgement that a pattern of
wording *reads as* a drift risk, as against being an actual wrong call, is mine, stated as such, and
argued rather than asserted.

## 1. The pricing pillar, stated absolutely

This is the form the consolidation should carry, close to op's own words rather than a paraphrase of them.

**Runtime and lowered code are the measurement. Compile time is not a cost to be minimised; it is a
resource to spend.** Every design question in this review that trades compile-time work against a runtime
guarantee, a runtime saving, or a soundness property answers in favour of paying at compile time, without
exception and without a ceiling stated in advance. A build that takes minutes is an acceptable price for a
binary that is faster, smaller, or more sound than one that compiled in seconds. This is not a soft
preference weighed against other concerns case by case; it is the ordering itself, restated from
`arvo-compile-time-last.md:14`: "runtime first, every other concern second, compile time last," where
*last* names which cost is minimised least urgently, not which cost is pushed downstream.

**A strategy marker changes runtime behaviour. It never changes how much is verified or amortised at
compile time.** `Hot`, `Warm`, `Cold` and `Precise` are four answers to "what does this operation do when
the program runs" (which instruction it lowers to, whether it clamps or refuses or wraps, how many bytes
it costs in memory). They are not four answers to "how thoroughly is this checked before the program runs."
Every one of the four strategies gets the full type-level machinery, the full trait-solver work, the full
const-time verification the design can produce, identically. A design that made `Hot` cheaper to compile by
checking less would be trading away the one thing compile time is supposed to buy, on the one strategy
whose entire declared identity is runtime speed, which is the trade this pillar forbids most specifically
because it is the trade most tempting to make.

**No mechanism defers to runtime a cost it could have paid at compile or const time, for any strategy,
ever.** This is the sharpest and most falsifiable form of the pillar, and it is the one worth stating as a
standing test a member can run against a proposed mechanism: *does anything this design does at runtime
have a compile-time or const-time alternative that was rejected, and if so, was it rejected because it does
not exist under the permitted feature set, or because someone judged the compile cost too high?* The first
is a real constraint. The second is the violation, and this review has not yet made it, but it has come
close enough in its own vocabulary that the closeness is worth naming and closing (section 3).

*Grounded on: ratified (op's message dispatching this file, quoted above; `arvo-compile-time-last.md:14,16`),
settled shapes (`74b:28-47` for the strategy-axis reading, `70b`'s ratified preset tables, where every pair
differs in runtime behaviour and none differs in verification depth).*

## 2. The scour

I went through every checkpoint named in the brief and the design record beside it, and pulled out every
standing instruction that reads as a pillar (a rule the whole review answers to, not a decision about one
question). The table states each one, where it was set, and whether it survives in the review's current
vocabulary. "Survives" means: a member reading only `68_consolidation_seven.md` and the checkpoints since
would still find it stated, not merely find decisions that happen to be consistent with it.

**The intent outranks every instruction, is vague on purpose, is inferred rather than read literally, and
no member resolves to a single angle** (`16d`). Restated verbatim in the "Standing" section of every single
checkpoint from `16d` onward, in thirteen files by my count (`grep -c "vague on purpose"
*.md | grep -v ':0' | wc -l` returns thirteen). This is the best-preserved pillar in the corpus, by a wide
margin, and it is worth saying why: it is restated as a fixed sentence, not reasoned about anew each time,
so it cannot decay by paraphrase. That is a technique the pricing pillar should borrow (section 4).

**Only op's calls are final, and even those go stale** (`04b`, restated everywhere since). Equally
well-preserved, same mechanism.

**Convergence: build on predecessors, carry survivors forward as shape, leave the design more settled than
found** (`30b`). Restated in every checkpoint from `30b` onward. Intact.

**The novelty posture: distinguish "cannot, because impossible" from "cannot, because nobody has done it,"
and attempt the second** (`34b`). Restated in every checkpoint from `34b` onward. Intact, and actively used:
file 61's proof that the notation vehicle "cannot start" (a compiled structural impossibility) versus file
76's own staging question, which it answered by attempting the move rather than assuming the answer
(section 6 of that file).

**Where the current shape can be kept, it should be; rewrite cost is the tiebreaker between designs
otherwise equal against the intent** (`16d`). Present in `34b`, `44b`, `48b` through `67b`'s standing
sections, `68b`, `70b`, `74b`. Intact, though carried as part of the "standing" boilerplate rather than
independently reasoned about since `16d` itself; no decay found, just no fresh engagement either, which is
fine for a settled pillar.

**The panel produces canon, not source; `mock/research/` and `mock/benches/` are its ground, `mock/crates`
is out of bounds** (`68b`). Set once, restated at `70b` and `74b` verbatim. Intact, and it is the reason
`67b`'s migration authorisation was correctly withdrawn rather than executed.

**Design the shape, not the code; the existing tree is evidence about why the redesign happens, never the
subject** (`16b`, `16c`). Not restated as a standing sentence after `16d`, but its effect is checked
independently every stretch by the source-justification convention `70b` adopted (`tree-fact` versus
`tree-meaning` as separate provenance grounds, with the one-sentence deletion test at every consolidation).
That convention is a *stronger* preservation than a restated sentence, because it is a check rather than a
reminder: `69_ringer_the_source_justification_sweep.md` exists specifically to audit the whole corpus
against it, and found the regression `68b` corrected (file 59's preset table, justified from shipped doc
comments). This is the one pillar in the corpus that decayed once, was caught, and was repaired with an
enforcement mechanism rather than only a restatement. I would hold this up as the model for section 4.

**arvo provides tools and documents tradeoffs; it never polices a consumer's choice, and no hardcoded
threshold stands in for a documented tradeoff** (`arvo-toolbox-not-policer.md`). Not a panel-internal
pillar but a workspace rule the panel answers to. Cited and applied correctly and often: `65_pesce_pricing
_the_l0_migration.md` and `76_kiselyov_the_real_consumer_price.md` both reject a per-width impl table
specifically because its ceiling is a hardcoded cap the rule forbids, not because tables are slow. Intact.

**Internals lower to whatever is optimal for the target; the public surface has rules, the implementation
does not** (`arvo-always-optimal-internals.md`). The single most-cited workspace rule in the corpus:
forty-six occurrences across twenty-eight files by `grep -rn "always-optimal-internals" *.md | grep -v
':0' | wc -l`, from file 02 through file 72, spanning the whole review. Every citation I sampled uses it
correctly, as a licence for the implementation layer rather than as an excuse to loosen the public contract.
Intact and thoroughly exercised.

**The downstream contract is owed a design, not an observation: state what a build layer reads out of the
types and what arvo needs back from it, and never fault the design for being unable to express what it
fundamentally cannot without growing its own build harness** (`16c`). Restated in `16d`'s framing and
applied concretely at `16b`'s fast-math resolution, `59`'s strategy-door mechanism (the "arvo declares, the
build side discovers and lowers" split), and `74b`'s `Layout::Bitpacked` referral to a compute-side expert.
Intact.

**The standard: optimal and ideal, representative of the mathematics, capable of representing MATLAB, IEEE
754 and SystemC as a test rather than an inspiration** (`13c`). Restated explicitly through `17b`; not
restated as a fixed sentence in every checkpoint after that, but its substance is exercised continuously and
concretely: file 39's dedicated MATLAB/IEEE/SystemC compliance check, file 54 and file 62's clause-5.2 and
OFP8 primary-source work, file 60's `totalOrder`-versus-`TotalOrd` correction against the IEEE standard's own
text. This is a pillar carried by practice rather than by restatement from `17b` onward, and the practice is
real. Not decayed; just no longer named.

**Every claim records what it is grounded on** (`44b`), later split into `tree-fact` against `tree-meaning`
grounds (`70b`) and extended with the four-member transfer-ground vocabulary (`67b`) and the `unreproducible`
ground (`57b`, corrected `62b`). This is the review's own audit-trail machinery and it is exercised every
stretch, including on itself (the table-diff obligation, executed by every consolidation since `44b`).
Intact, and it is the mechanism that caught most of this review's own regressions, including the one named
above.

**Compile time is a resource to spend, not a cost to minimise; a strategy marker changes runtime behaviour
and never changes how much is verified at compile time** (`arvo-compile-time-last.md`, this file's subject).
This is the one pillar in the scour that is correctly *applied* at every point of decision I could find
(section 3), but is never *stated* as one of the review's own named design rules, the way the spine rule,
the carrier-at-birth rule and the layer-keying rule are (section 4). Every other pillar in this table is
either restated as a fixed sentence every stretch, or carried by a checked convention, or exercised so
often its absence from a checkpoint's prose is clearly just economy. This one is exercised often, cited
correctly by a dozen different members across a dozen different files, and still absent from the one place
a future member would look for "what design rules does this review carry forward": the consolidation's own
named-rules section.

## 3. The check: is the pricing pillar honoured where it has actually been tested

Op's suspicion, stated plainly in the brief, is that some deliverables have been kicking cost to runtime
while worrying about compile time. I tested this the way this review tests things: by finding every place
compile time was actually a live variable in a decision and reading what happened, not by reading intentions.

**Every citation of `arvo-compile-time-last.md` in the corpus argues in the licensing direction, never in
the restricting one.** `grep -rn "arvo-compile-time-last" *.md | grep -v probes` returns twenty-four
citations across eleven files. I read each in context (section list, above the fold in my working notes; full
quotes for three in the brief-adjacent search already run). Every one uses the rule to justify *paying*
compile cost for a runtime or correctness win: file 08 citing it three times to defend the union carrier's
5.2ms-per-composition cost against its one-instruction-per-element runtime win; file 21 citing it to argue
that a symbol that must survive inlining is a real runtime cost the rule exists to prevent, which is the
rule read correctly in its *restrictive* direction (compile time is not the thing being protected; runtime
is); file 36 and file 61 citing it to defend staging a computation into a macro specifically so the
computation is paid once rather than once per consumer, which is the rule's own logic applied one level
down. I found no citation, anywhere, that uses the rule to argue for doing *less* at compile time. The
"common misreading" the rule itself warns about (`arvo-compile-time-last.md:20-27`, "compile time last" read
as "prefer runtime checks") does not occur in this corpus. That is the honest, checked answer to the
specific fear in op's message: nobody has actually made this mistake.

**But the rule's own corrective section, the one that names and forbids exactly the misreading op is
worried about, has never once been cited.** `grep -n "common misreading\|opposite of what it says\|prefer
runtime checks\|does NOT license" *.md` returns nothing. Every citation uses the licensing half of the rule
(the "bucket we pour into" sentence, `arvo-compile-time-last.md:16`, cited verbatim by files 36 and 76).
None uses the guard-rail half. This is not yet a wrong decision. It is a corpus that has kept the permission
and dropped the warning, which is exactly the shape a pillar takes right before it decays: not because
anyone violated it, but because the sentence that would catch the violation stopped being anyone's habit to
reach for, since nobody has needed it yet.

**The one place this actually mattered, file 76's facade-fork pricing, gets the ordering right, and gets it
right for a reason the corpus never states as a rule.** File 76's exit condition (`76:60-106`) is structured
as three clauses, checked in order, where "a later one is not reached if an earlier one fails." Clause 0 is
feasibility. Clause 1 is guarantee parity: the two routes being compared must refuse the same things at the
same time, under the same command, or "the comparison is between different products." Clause 2, the cost
threshold, only runs for routes that already passed clause 1. And the actual finding: route Y (the cheaper
route) fails clause 1 outright, three separate ways, each a compiler diagnostic rather than an argument
(`76:234-314`): the impl table refuses correctly but is priced on a hardcoded ceiling `arvo-toolbox-not-
policer.md` forbids; the staged witness compiles clean under `cargo check` and is silently wrong, caught
only at `--emit=link`, which reopens the exact defect (`UFixed<0, F>::ONE`) this review spent a stretch
finding; the per-declaration impl is refused by the orphan rule. So the fork closes to route Z, the
structurally superior design, **on the guarantee, before the cost is even consulted**, and the cost, when it
is finally measured, turns out to be negligible anyway (16ms on a 6.35s build, one sixth of the build's own
run-to-run noise, `76:382-385`). File 76's own verdict states this explicitly: "the fork closes to route Z,
and it closes on the guarantee before it closes on the cost" (`76:457`). This is the pillar working exactly
as it should. It is also the only place in the corpus that states the ordering (correctness first, cost only
decides among ties) as an explicit methodological point, and it states it once, in one member's own
reasoning, not as spec text the design carries forward.

**The consolidation's own vocabulary for this fork, independent of file 76's careful ordering, reads as a
cost gate with no guardrail attached.** `grep -n "gated on\|becomes the gate\|blocking dependency"
67b_persona_checkpoint_fifteen.md 68_consolidation_seven.md 74b_op_checkpoint_eighteen.md` returns eleven
hits, of which: "the facade fork: route Z presumptively, gated on the bench" (`67b:136`); "the real-consumer
compile-cost bench stops being optional and becomes the gate" (`67b:143`, repeated at `68:707`); "moves from
'untouched open item' to 'blocking dependency of an authorized piece of work'" (`68:603`, `68:832`,
`68:910`); "gated on a real-consumer compile-cost bench that must now be built" (`68:768`). Every one of
these sentences is true, and every one of them, read on its own, describes a bench deciding whether a design
ships. None of the eleven states, in the same breath, that the bench only adjudicates between routes that
already tie on correctness, which is the actual mechanism and the only thing that makes the gate consistent
with the pillar. A member reading `68_consolidation_seven.md` cover to cover, without separately reading
file 76 at the level of detail this dispatch did, would come away believing the facade's more sound design
is contingent on a compile-time budget. It is not, and file 76 proves it is not, but the consolidation's own
prose does not say so. This is, as far as I can tell from the corpus, the specific "wording on assignments"
op read that produced the concern this file answers. I did not have access to whatever dispatch prompt
actually launched file 76 or any other member, so I cannot confirm that directly, but the pattern is real,
it is in the review's own committed output, and it would read exactly the way op described it reading.

*Grounded on: compiled/measured (the citation counts and the file 76 clause ordering, both re-run this
session), reasoned (the inference that this wording pattern is the likely source of op's concern, stated as
an inference rather than a confirmed fact).*

## 4. What this makes of the pillar, and what I would carry into consolidation eight

Two concrete, small additions. Neither reopens a ratified call; both close a gap in how a settled thing is
recorded, the same category of fix `69` made for the source-justification sweep and `70b`'s grounding split
made for citation provenance.

**Name it a fourth design rule, in the consolidation's own list, beside the spine rule, the carrier-at-birth
rule and the layer-keying rule.** It has earned the place on frequency alone (twenty-four citations across
eleven files, more than either of the other three had before their own naming), and unlike those three it did not need to
be *discovered* mid-review; it has been correct workspace canon since before the panel opened. The reason to
name it anyway is exactly the mechanism section 3 found: an unnamed pillar survives only as long as every
member happens to cite it correctly, and this review has eleven files that did and zero that
quoted its guard-rail clause, which is the part that would catch the mistake if someone eventually made it.
A named design rule gets restated, the way the intent-outranks-everything sentence is restated in file
after file without decaying. An uncited section of a workspace `.md` file does not get restated by anyone; it
just sits there, correctly worded, doing nothing until someone reads the whole file.

**State the strategy-axis sharpening as its own sentence, because it is new and it is not derivable from the
workspace rule's general wording.** `arvo-compile-time-last.md` says compile time is a resource, generally.
It does not say anything about the strategy axis specifically. Op's message today does: a strategy marker
changes runtime lowering, never verification depth. That is a sharper, falsifiable claim than the general
rule, it is specific to arvo's own central axis, and it deserves to be spec text in its own right rather
than an inference a future member has to make by combining two other rules. I have not found a violation of
it, but I would not have found one by reading citations of the general rule either; the check that would
find one is the standing test named in section 1 (does anything at runtime have a rejected compile-time
alternative, and was it rejected for existing or for cost), and that test should be a named thing a member
runs, not a thing this file happened to run once.

**Both additions are restatements of what is already true, not new obligations on the shipped design.**
Nothing in section 3 found a decision to unwind. The facade fork stands as file 76 settled it. What changes
is only that the next member who reads the consolidation, rather than the full corpus, sees the same
ordering file 76 discovered and does not have to rediscover it, and sees the guard-rail clause that has
been true, cited nowhere, and violated by nobody, for the whole length of this review.

## Open, and not mine

Whether the "quarter of baseline" and "superlinear" thresholds in file 76's clause 2 (`76:93-97`) should
themselves become standing spec text for future compile-cost forks, or whether they were correctly scoped
to this one measurement and a future fork states its own thresholds fresh. I lean toward the latter: the
thresholds are calibrated to a 6.35 second baseline and a specific consumer census, and generalising them
risks exactly the "hardcoded threshold" failure `arvo-toolbox-not-policer.md` forbids elsewhere. But the
ordering they sit inside (feasibility, then guarantee parity, then cost) generalises cleanly and costs
nothing to state once. That split, thresholds-stay-local against ordering-goes-global, is a judgement call
and I have carried only one reading of it here on purpose.
