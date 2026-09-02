# 69. The source-justification sweep, and how far one bad column travelled

Talia Ringer, file 69. I wrote files 19, 44 and 52. File 44's subject was a claim written under one
set of coordinates, carried forward after the coordinates moved, found because the review had produced
two instances of that failure by accident and nobody had gone looking for a third on purpose. This
dispatch is the same habit of mind aimed at a different coordinate: not "did the ground move under a
claim" but "was the claim ever standing on ground the design owns in the first place." Op named the
defect class directly in `68b`, named its exhibit, and asked whether it recurs. It does, in exactly one
place beyond the exhibit itself, and that place is the exhibit's own shadow: three files that repeated
it without re-deriving it.

**What I read.** `68_consolidation_seven.md` and `68b_op_checkpoint_sixteen.md` in full, both required.
An `ls` of the panel directory (files `00` through `68b`, sixty-nine numbered deliverables plus
checkpoints and probe directories, nothing landed after `68b`). Then, because this dispatch's subject is
the panel's own citation discipline rather than any one coordinate, I read across the whole review as the
brief licenses: full reads of `59_fog_the_lowering_door.md` (the exhibit), `62b_persona_checkpoint_
fourteen.md` section on the strategy door, `63_consolidation_six.md` section 6 and its live-defect
registry, `64_chlipala_the_owed_second_reads.md` section 6, and `44_ringer_what_the_overturn_left_
behind.md` (my own, for the precedent this file extends). For every other file, targeted reads at the
lines a grep sweep surfaced (below), not full reads; I name which for each finding so the coverage claim
is checkable.

**Gates.** Canon gate: reproduced fresh from the repo root, `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test
gate: `cargo test --offline --workspace` from `mock/`, summed per binary rather than trusted from a
headline: **658 passed, 0 failed, 9 ignored**, matching every consolidation since file 65. Toolchain
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, resolved from `rust-toolchain.toml`, `aarch64-apple-
darwin`.

**How the sweep was run, and its honest limit.** I grepped the whole panel for `doc comment`, `shipped
meaning`, `shipped tree`, `already means`, and `keep the shape where it serves` (the phrase invoked to
justify reusing shipped mechanism), read every hit's surrounding twenty to forty lines, and sorted each
into one of two bins: a claim about what currently **exists** or currently **behaves**, checked against
source as evidence (licensed), or a claim about what the design **should mean**, resting on source or
its prose as the reason (the defect). Forty-one files carry at least one hit. I read all forty-one hits'
contexts. I did not re-read every file end to end looking for source-justification outside those grep
patterns; a conclusion resting on shipped source without the word "doc comment," "shipped," "already
means" or the keep-the-shape phrase near it would not surface this way. That is this sweep's real
coverage boundary, and I state it rather than imply exhaustiveness the method does not have.

## 1. The exhibit, briefly, because a second dispatch re-derives it and this file's job is not to

`59_fog_the_lowering_door.md` section 2.3 builds a table assigning each strategy marker's float
lowering door. Its own words: "Every row below is derived from what the preset already means for
fixed-point arithmetic in the shipped tree, carried across to float by the same reading" (`59:215-217`),
column header "shipped meaning" (`59:219`), and each cell quotes a doc comment verbatim:
`arvo-strategy/src/lib.rs:110-114` for `Hot`, `:118-124` for `Warm`, `:128-131` for `Cold`, `:135-139`
for `Precise`. I read the shipped file fresh (`mock/crates/arvo-strategy/src/lib.rs:109-139`) and the
quotes are accurate, word for word, against what is there today.

Accurate quotation is not the defect. The defect is what the quotation is asked to do: stand in for a
design derivation. Op's correction names it exactly: "we shouldn't reference or compare existing code
or its comments, they are by definition deprecated and wrong on the new design. And also, we are fully
free to restructure the strategies and their meanings" (`68b`). A doc comment describing what `Hot`
means for **fixed-point** arithmetic today is not evidence about what `Hot` should mean for a **float**
lowering door tomorrow. It is evidence about what one prior author wrote down once, for a different
axis, under a design this review exists to replace. Op's own re-derivation (`68b`) confirms two of the
four rows on independent grounds (the talk topic's stated intent, `202607301100_topic.the-formalization-
talk.md:1659-1661`) and reopens the other two, which is the sharpest evidence available that the shipped
prose was never load-bearing for the rows that happened to agree with it either: agreement with intent
is what makes a row survive, and the doc comment supplied neither the agreement nor the intent.

I am not re-deriving the table; a separate dispatch does that. What follows is new: how far this
specific table travelled uncited-as-suspect, and what let it.

## 2. The propagation, traced file by file, and it is worse than "no reader caught it"

Op's own count in `68b` is "no reader caught it across three subsequent files." I traced which three,
and the shape of what each one did with it, because the shape matters for the rule this file proposes
in section 4.

**`62b_persona_checkpoint_fourteen.md:166`, first repetition, verbatim framing kept.** "Warm, Cold and
Precise are forced by their own shipped doc comments rather than chosen." No new grounding offered; the
persona checkpoint adopts file 59's own sentence, restructured, with the same subject (the doc comments)
doing the same work (forcing the rows).

**`63_consolidation_six.md:576-591`, second repetition, and the one that made it a table again.** "Every
row is derived from what the preset already means for fixed-point arithmetic in the shipped tree,
carried across to float by the same reading, rather than assigned by preference" (`63:576-578`),
followed by a four-row table whose "shipped fixed-point meaning" column reproduces file 59's quotes in
paraphrase, and whose "is it a choice" column repeats "Forced" for three of four rows on the identical
basis. This is where the claim stopped reading as one member's derivation and started reading as
consolidated fact: a consolidation's own table-diff obligation (`57b`'s own standing rule, executed by
every consolidation since, per `68:71-78`) checks a table against the prose in the same document and
against the source file that established each row. It does not, and by its own stated scope cannot,
check whether the source file that established the row was itself entitled to. The obligation caught
formatting drift and missed provenance drift, because it was never asked the provenance question.

**`64_chlipala_the_owed_second_reads.md:406-413`, the one file that touched the neighbourhood and did
not repeat the defect.** Chlipala's own sentence: "which Fog believed the shipped tree had already
answered (`arvo-strategy/src/container.rs:104-112`)" (`64:409-410`), attributed as a belief, not
restated as a fact, and confirmed independently by reading the cited precedent fresh and quoting it
(per `68:536-537`, "File 64 confirmed the refusal-versus-fallback call directly against its cited
precedent... read fresh and quoted"). Chlipala's confirmation is narrow by construction: it verifies
that the shipped container-projection precedent (`BitsContainerFor`'s refusal-not-fallback posture)
still reads the way file 59 said it read, which is a factual check on a **mechanism's current
behaviour**, licensed under the "evidence about why" and "checking a factual claim" exceptions this
dispatch's brief states. It never touches the "shipped doc comments force Warm/Cold/Precise" claim at
all. That claim is not confirmed by file 64; it is simply not visited. Op's own confirmation in `68b`
lands on the same row Chlipala independently checked (`Hot`'s refusal call) and on no other, which is
consistent with Chlipala having checked the one row that was checkable this way and having been silent,
correctly, on the three that were not.

**`68_consolidation_seven.md:534-535`, third repetition, by omission rather than restatement.** "The
mechanism, the derived door assignment, and the corrected per-preset table all stand as `63:541-624`
states them." No new quote, no new grounding, a pointer forward through the chain that started at file
59. This is the file op read and corrected. The consolidation's own verification section (`68:973-985`)
runs the canon gate and the test gate and confirms every claim tagged compiled or measured traces to a
probe or artifact, and that check is real and it passed, because the exhibit's defect is not a compiled
claim without a probe. Every quote in file 59's table has a real citation and a real probe file behind
the mechanism (`59_probes/probe_3c`, `probe_3d`, `probe_6`, `probe_7`). The verification section checks
that a citation exists and resolves. It has no mechanism for checking what kind of thing the citation is
evidence for, and a citation to a doc comment resolves exactly as cleanly as a citation to a compiled
counterexample. That is the same failure mode as section 3's table-diff gap, restated: **the review's
existing verification machinery is aimed entirely at "is this claim traceable," and the defect this
sweep is for is a claim that is perfectly traceable and wrong anyway**, because the thing it traces to
was never authoritative for the question being asked.

So: one origin (`59`), three carriers (`62b`, `63`, `68`), one file that touched the neighbourhood and
came away clean because its own check was narrower than the claim it was near (`64`). Four files, not
three, if the near-miss is counted; op's "three subsequent files" is the count of files that repeated
the defect, which is right, and I add that a fourth file sat one paragraph away from it and did not
catch it either, for a reason worth naming: `64` was never asked to check the doc-comment framing, only
the refusal call, and a reader skimming `64:406-413` alongside `63`'s table could reasonably come away
believing the whole table had been independently confirmed. It had not. Nothing in `64`'s own text
claims that; the risk is in how a later reader compresses two adjacent, differently-scoped confirmations
into one.

## 3. The rest of the panel, swept, and what survives

Forty-one files carry a grep hit. I sort the contexts below by what they were being asked to establish,
which is the test this dispatch's brief sets: does the file's **conclusion about the design** rest on
the shipped source's meaning, or does the file use the shipped source to establish a fact (that
something exists, that something currently behaves a certain way, that a prior claim about the tree was
right or wrong) and derive the design conclusion from elsewhere.

**Clean: checking a factual claim before reasoning from it (the brief's second licensed use).**

- `06_muratori_the_consumer_surface.md:57-67`. Recounts a consumer-usage claim from file 04 against
  source (`arvo/src/aliases.rs:35-39`'s doc comment says "use this at consumer call sites," measured
  usage is zero). The conclusion is about **current consumer behaviour**, used to correct a prior file's
  premise, not about what the design should be. Licensed.
- `12_lattner_fresh_read.md:20-45`. A premise-check pass, explicitly framed as one ("given this panel
  family's recorded history of inheriting a false GCE premise, I checked this one against source
  first"). Every citation here answers "does the draft's factual claim about the tree hold," not "what
  does the tree mean for the design." Licensed, and it is doing exactly what `panels-argue-the-intent-
  not-the-wording.md`'s "first thing a panel does is try to break its own brief" asks for.
- `13_mcsherry_where_the_laws_belong.md:245-266`. Reads `path.rs:23`'s doc comment ("the maximum path
  weight ending at any node") against what the shipped DP actually computes under non-associative
  addition, to establish that the **doc comment overclaims**, i.e. evidence about why the crate needs a
  design that states its order honestly. The design conclusion (documented order is a real function
  contract, a specification fiction) is McSherry's own argument, not derived from the comment's content.
  Licensed.
- `15_willsey_what_a_law_is_for.md:110-135`. Cites `arith.rs`'s doc comment to establish that shipped
  `u_add` matches its own documented semantics, as a **contrast** against shipped `u_mul_fixed` failing
  to match its own documented semantics. Both citations are factual checks on current behaviour, used to
  find a gap in the shipped implementation, not to define what the new `Quantisation` axis should be.
  Licensed.
- `30_pesce_the_identity_half_assembled.md:17`. `no_multiplicative_identity.rs:1-11`'s doc comment
  explaining a test's own scope. Evidence about a test, not a design derivation. Licensed.
- `33_lamport_the_laws_restated.md:186-198`. The one case I read twice, because it sits closest to the
  exhibit's shape. Lamport establishes that law equality should be induced by a total order rather than
  `PartialEq` (a mathematical argument, stated independently: `Specials` makes value equality a partial
  equivalence relation, so a law needs a total order to stay reflexive at NaN), then notes the mechanism
  already exists: `TotalOrd` is `pub const trait` in `arvo-numeric-contracts/src/lib.rs:65`, "with a doc
  comment that says in as many words that it exists so float-bearing arvo types can have a strict-NaN-
  policy total order" (`33:189-190`). The design conclusion (law equality uses the total order) does not
  rest on the doc comment; it rests on Lamport's own math, stated first, in the paragraph before the
  citation. The doc comment is offered as corroboration that the needed mechanism is already declared,
  which is a fact about the tree ("does this trait exist"), not a claim about what it should mean. This
  survives, but the phrase "already justified in its own doc comment for exactly this reason" (`33:198`)
  is doing rhetorical work the math did not need and should not have been reached for: it reads, out of
  context, exactly like the exhibit's framing, and the review would be safer without it. Flagged as a
  wording risk, not a finding.
- `46_spj_what_the_seal_guarantees.md:235-252`. Grounding-registry mechanics (a claim discharges to
  `grounded on: tree` once the seal lands in the shipped tree). This is the registry's own vocabulary
  for "the artifact now exists," not a claim about meaning. Licensed, and outside this sweep's subject
  entirely.
- `51_fallin_the_last_tick_and_the_licence.md:180-196`. Reads rustc's own `core::intrinsics` doc comment
  to establish a fact about an **external, third-party** mechanism (`fadd_fast` has no stable
  counterpart) as part of the unstable-feature vetting procedure `unstable-features.md` requires. Not
  arvo's design, not arvo's source. Outside the defect class entirely; the rule targets treating arvo's
  own deprecated tree as design authority, not reading upstream documentation to vet a language feature.
- `60_dolan_value_or_datum.md:145-165, 265-290`. Reads `fiedler.rs`'s doc comment and hilavitkutin's
  actual consumption code to establish what a **real consumer currently does** with a value, feeding an
  engineering question (does a widened accumulator's cost matter to any consumer that exists). This is
  the brief's first licensed use, evidence about why a design choice matters, applied to a downstream
  repo's actual behaviour rather than to arvo's own deprecated meaning. Licensed.
- `65_pesce_pricing_the_l0_migration.md:188-193`. Establishes that `OneRepresentable`'s shipped doc
  comment matches what the review already knows the predicate does (the fix for the `UFixed<0, F>::ONE`
  defect files 30, 33 and 39 established independently, by compiled test failure, per the catalogued-
  edge-case discipline). The design conclusion, that a type with zero integer bits has no representable
  one, was established elsewhere and earlier; this citation only confirms the shipped predicate now
  encodes it. Licensed.

**Every other hit** (files `08`, `14`, `16`, `18`, `19` itself, `23`, `27`, `34`, `40`, `41`, `42`, `45`,
`49`, `52`, `55`, `58`, `61`, `63` outside section 6, `65` outside the passage above, `67`, `67b`) reads
as one of: a rewrite-cost measurement ("no shipped source names X," licensed, it is a claim about the
tree's current absence of a coordinate, not about meaning), a defect report against the shipped tree
(licensed under "evidence about why the redesign is happening," the review's own stated purpose is
finding these), or a citation to a **test's** doc comment explaining its own scope (`52_ringer_the_
tests_that_were_owed.md`'s repeated pattern, which is my own earlier file auditing whether tests were
honest about what they measured, not deriving design meaning from them). I read each hit's context and
found no second instance of the exhibit's shape: a design conclusion whose stated reason is what a piece
of arvo's own deprecated prose says a construct means.

**The sweep's verdict is narrow and I want it stated as narrow.** One defect, one origin file, three
carriers, one adjacent near-miss. Not a systemic pattern across the panel. The panel's citation
discipline (the `grounded on:` tag, the compiled/measured/reasoned separation, the table-diff
obligation) is working for the failure modes it was built to catch, which is exactly why this one got
through: it was built to catch untraceable claims and unverified tables, and the exhibit's claim was
traceable, verified, and wrong for a reason the machinery never asked about.

## 4. One exposure that is not yet a violation, named because the next dispatch touches it

Section 2.4 of file 59 (carried at `63:594-598` and standing in `68` unchanged) derives "the hardware
door is reachable only in a uniformly-`Hot` expression" from `Strategy::RANK`'s shipped cross-strategy
resolution order, `Precise > Cold > Warm > Hot` (`arvo-strategy/src/lib.rs:104-107`), compiled against
the whole sixteen-cell matrix (`59_probes/probe_6`). I read this closely because it has the same shape
as the exhibit at first glance: a shipped ordering, cited, feeding a design conclusion. It is not the
same defect, and the reason is worth stating precisely rather than waved past. `RANK`'s ordering is not
prose describing intent; it is a compiled, load-bearing mechanism that the review's own "keep the shape
where it serves" instruction licenses reusing (`panels-argue-the-intent-not-the-wording.md`: "where the
current shape can be kept, it should be," rewrite cost is a real tiebreaker). Reusing a working
mechanism is different from reading a comment's stated purpose as the design's meaning; `59:255-257`
says so itself ("a rank ordering that shipped long before anyone asked the door question, delivering a
safety property the door question needed. It is the review's 'keep the shape where it serves'
instruction paying out"), correctly, and the exhibit's own citation of the same phrase at the same
coordinate did not extend to section 2.3's table two subsections earlier.

The exposure is downstream, not in this file. D71's own preset definitions are now stated as
"overtaken" (`68b`, "ratified panel calls have moved the ground under two of its rows: `Widening` was
removed as an axis entirely, and `Growth` left the law key"), and the preset re-derivation dispatch
op sets alongside this one will restate what `Hot`/`Warm`/`Cold`/`Precise` mean across six axes,
including for float for the first time. If that re-derivation changes what any preset means in a way
that changes its rank (op's own `Cold` statement, "seldom computed... can take more cost than warm,
shouldn't just be precise in disguise," is a different axis than the storage-density framing `RANK`
currently encodes), the uniformly-`Hot` theorem inherits that ground shift silently, because it is
grounded on `tree` for the ordering itself and nothing in this file's or file 63's or file 68's text
re-checks that grounding against the redefined presets. This is not a defect in what exists today; it
is a dependency the preset re-derivation dispatch should discharge explicitly rather than the next
consolidation inheriting it the way `63`'s table was inherited. I flag it here rather than fix it,
because fixing it is that dispatch's job and this file's job is the sweep.

## 5. The rule, proposed for the review to adopt

The review already has a `grounded on:` discipline separating `ratified decisions`, `settled shapes`,
`physical grounds`, `tree grounds`, and `unreproducible`, and this stretch added `transfer ground` for
claims established at a model instance and relied on at a real one (`68:429-434`). None of these
distinguish, within `tree`, between "the tree proves a fact exists or currently behaves this way" and
"the tree's stated purpose is offered as the design's reason." The exhibit shows those are not the same
grounding and the existing vocabulary does not separate them.

**Proposed: split the `tree` ground into two, and require the split at the point of citation, not at
consolidation time.**

- `tree-fact`: the shipped source establishes that a mechanism exists, compiles, currently behaves a
  stated way, or currently disagrees with its own documentation. Licensed for any claim about current
  state, and licensed as input to a design conclusion only when the design conclusion is stated and
  argued independently of the citation, with the citation offered as corroboration that a needed piece
  already exists (Lamport's `TotalOrd` use, section 3 above, is the worked example of this done right).
- `tree-meaning`: the shipped source's own prose (doc comment, variable name, module doc) is offered as
  the reason a design construct should mean what it means. **This ground is forbidden.** Per op (`68b`)
  and per the panel's own founding instruction that shipped code and its comments are "by definition
  deprecated and wrong on the new design," no claim may carry it. A file reaching for it has, by
  definition, reached for prose this review has already been told to distrust.

The mechanical check this buys: a table column, a section header, or a "grounded on" tag that would
have to read `tree-meaning` to be honest is the tell, checkable at the point a member writes the
sentence rather than only in retrospect. File 59's own table header, "shipped meaning," would have
failed this check on sight; so would `62b:166`'s "forced by their own shipped doc comments"; so would
`63:576-578`'s "derived from what the preset already means... in the shipped tree." None of the clean
citations in section 3 would fail it: every one of them, reread against this test, is asking "does X
exist / does X currently do Y," never "what does X mean for the new design."

**A second, cheaper check for the same defect, for consolidations specifically.** The table-diff
obligation (`57b`, executed since) checks a table against the prose in the same document and the source
that established each row. Extend it by one question, asked once per table at the point a consolidation
absorbs it, not reinvented per table: **for each row whose citation is to shipped source, does the row's
justification survive if the citation is deleted and only the design's stated intent remains?** If yes,
the row is `tree-fact` and the table stands. If no, the row was never grounded in the design at all, and
the consolidation should say so rather than compress a member's confident phrasing into settled fact.
This question would have caught the exhibit at `63`'s own table-diff pass, because `63:576-578`'s
justification sentence names the shipped tree as the **entire** reason, and deleting the citation
deletes the row's justification along with it. It would not have flagged Lamport's `TotalOrd` use,
because deleting the doc-comment citation there leaves the mathematical argument for a total-order-based
law equality fully standing.

## 6. What this file leaves for the next one

**Not this file's job, and intentionally left to the dispatch op named alongside it.** The preset
re-derivation itself: what `Hot`/`Warm`/`Cold`/`Precise` mean across the current axis set, and what a
float reading of each is, governed by op's two verbatim statements in `68b`. Section 4 above names one
dependency that dispatch should check explicitly (whether `RANK`'s shipped ordering still holds once the
presets are restated) rather than silently inherit.

**Carried forward as a finding, for the next consolidation to fold in.** The exhibit's own table
(`63:582-586`, standing unchanged in `68`) is void pending the re-derivation; it should not be restated
as settled a fourth time. The propagation trace in section 2, so a future reader who finds `62b`, `63`
or `68` citing the strategy-door table understands why those three specific citations carry no
independent weight beyond file 59's original one.

**A proposed rule, not adopted by me.** The `tree-fact`/`tree-meaning` split in section 5, offered as a
suggestion per the panel's own standing instruction to suggest rather than legislate. If op or a later
member finds it too heavy a mechanism for what is, so far, a single traced defect, the lighter version is
just the one-sentence test itself: before citing shipped source in a design conclusion, ask whether the
conclusion survives the citation's deletion, and if it does not, the citation was never evidence.

**Open, and genuinely so.** Whether the forty-one-file grep sweep's coverage boundary (stated in the
gates section) hides a fifth instance that does not use any of the four search phrases. I did not find
one in the files I read closely, and I did not read every file end to end hunting for it without the
grep's help. A member with time to spare could run the check this file's rule proposes, sentence by
sentence, across every file that cites `tree` at all, rather than only across the phrase-matched subset
this dispatch used.
