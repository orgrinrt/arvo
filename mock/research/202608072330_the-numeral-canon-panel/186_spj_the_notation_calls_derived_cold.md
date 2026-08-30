# 186. The notation calls, derived cold (Simon Peyton Jones)

**Phase one.** Written from the phase-one list alone: `mockspace.toml`'s `[[registry.namespace]]`
declarations, `mock/registry/dimension.toml` with its header, `INTENTS.md` in full, `RULES.md` in
full, the workspace rule `every-finding-carries-its-predicate.md`, and the one licensed data file
`183_probes/unblock_value.out`. I have not opened `179`, `183`, the rest of `183_probes/`, any
`proposal*.toml`, `question.toml`, `ruling.toml`, `law-the-later-topics.toml`, or the panel's commit
log beyond the tip needed to confirm my worktree. Where I lean on a figure the dispatch brief stated
(fifteen of seventeen proposal rows `normative`, 115 keys, 96 undeclared, 60 blocked spans), I say
so and it is the brief's figure, verified in phase two rather than here.

**Canon gate: passed.** Checked against `INTENTS.md` (I13 the sole RATIFIED entry, its two
paragraphs and the two instructions recorded beside it), `RULES.md`'s provenance ladder, and the
schema's own stated reasoning in `mockspace.toml`. The assigned work is a second independent reading
of five calls, which is exactly the two-reader mechanism the rules demand; nothing in it builds on
unratified state.

**Test gate: run.** `cargo test` from `mock/`: 43 tests across five files, all green, one ignored
with an honest catalogue reason inline (the empty `probe` namespace, tracked to its dispatch).
`cargo mock --lint-only` deferred to the end of phase two alongside the second run, since my only
writes are this file and its probes. I have not yet read the bodies of the checks I am asked to
judge in question 2; that is phase-two reading by the dispatch's own ordering, and my phase-two
section owes the verdict on whether those bodies keep their names' claims.

---

## 0. The one principle all five questions are instances of

Let me be concrete about what I think the notation *is*, because the five calls all turn on it.

A predicate in this registry is a **positive, closed claim over regions of the world**: the
situations arvo will find itself in. Op's omission semantics quantify over the world, not over the
registry's vocabulary. "Unmeasured or unknown does not list in the predicate. It's not known, it's
assumed not true until proven true" was said before this registry existed and makes no reference to
a declared axis set. The `dimension` namespace is the *checker's* vocabulary: it is what lets
`mock/checks` parse a span and refuse an inexact one. It is not what a claim's meaning quantifies
over. That distinction, world against vocabulary, decides question 1 outright and colours the
other four.

And the registry's own design states the second half of the principle in three separate places:
a blessing is a new row naming what it promotes, never a flag flipped on the old one; a superseded
row stays; a predicate is never widened in place, the widening arrives as a later claim. So:

**The scope of any act, a refusal, a signature, a withdrawal, a declaration, is what its ground
reaches, and the registry's one mechanism for tracking scope is a new row pointing at old material.
Meaning is never mutated in place.**

Every one of the five questions is somebody asking whether to mutate meaning in place or to append
a pointing row. The answer is the same each time, and the delightful thing is that op has already
ratified it once, in I13's own sitting, for the hardest of the five cases.

---

## 1. Declare the axis. The "silent rewrite" is not a rewrite, and op has already ruled on this exact case

### 1.1 The seat's reasoning, tested

The declining seat's ground: declaring a new axis silently rewrites the negative space of every
committed predicate, because an axis a predicate does not name is one the claim does not hold on
at all, so a row that never considered the ambient domain would come to say it holds for no ambient
domain whatsoever.

The reasoning mistakes **revelation for rewriting**. Under op's semantics the claim *already* holds
for no ambient domain whatsoever, and it always did, from the moment it was committed: nobody
varied the ambient domain, so nothing was established along it, and "unmeasured... is assumed not
true until proven true." The ambient domain did not start existing when somebody proposed a slug
for it. The world's dimensions are not brought into being by TOML. What the declaration changes is
that the *checker* can now parse spans naming the axis, and that *readers* stop over-reading rows
that were always this narrow. A reader who believed a committed row held across ambient domains was
wrong yesterday and is corrected today. That is the discipline working, not the discipline eating
its own corpus. The instability I13 exists to kill is precisely a claim whose implied scope each
reader supplies from context; leaving the axis undeclared *preserves* that instability for the
biggest blocker in the census.

Two checks on my own reasoning, because the counter-position has one good tooth in it:

**The apparent absurdity.** If absence is world-quantified, is not every committed row already void,
since there is always some axis nobody named? No, and the workspace rule itself says why: a finding
states its region "as an explicit predicate over **every dimension that could move it**". The
obligation runs over dimensions that could move the claim's truth. A dimension that cannot move it
wants `any`, written explicitly ("threads don't matter is a statement for adding threads any...
nothing infers it"), and a dimension that could move it and was not varied is exactly the case op
priced: not true until proven true. Under-claiming is the sound direction. A row that says less than
its author hoped misleads nobody; a row read as saying more than its evidence supports is the
failure this whole notation was built against.

**Does the append-only rule answer it?** Yes, and more directly than the dimension header's hedge
suggests. Op's paragraph is not merely *analogous* to this case, it is *about* this case, verbatim:
"If a later expert **finds another dimension** having some predicate, be it any or an actual
predicate, they then correct in their deliverable and it ends up in consolidation. The original
does not get updated." Discovering the ambient-domain axis is a later expert finding another
dimension. The mechanism is ratified-adjacent (his instruction inside I13's sitting), it is the
workspace rule, and it is already the schema's shape (`supersedes` on `proposal`). The original
rows stand as their evidence supported; widenings arrive as new rows carrying `ambient domain: any`
or a real value, each with the argument or instrument that earns it.

### 1.2 The timing argument the declining seat's caution inverts

Here is the part I would put most firmly. The cost the seat fears, committed rows narrowing under a
new axis, is **monotonically increasing in the size of the predicated corpus**. Today, by the
brief's own figures, seventeen proposal rows exist and fifteen are `normative`, which carry no
predicate at all; the predicated surface is a handful of rows plus whatever `law` carries. Sixty
spans are *blocked from migration* waiting on the vocabulary. Declining to declare does not protect
the corpus; it holds sixty findings out of the canon in order to protect at most a handful of rows
from a narrowing that, per the paragraph above, is not a narrowing at all but a correction of
over-reading. And every month of delay grows the corpus that will "narrow" when the axis finally
lands, because land it must: an axis that is the sole blocker of four spans and appears in
twenty-six does not go away by being undeclared. The decline maximises the exact damage it was
meant to avoid. Declare now, while the blast radius is a rounding error.

### 1.3 The gateability test, attacked as instructed

The test offered: an arm can be gated on `fraction_width: 0` and cannot be gated on `arms = 5`,
so gateable keys are design axes and ungateable ones are instrument parameters.

It is the right instinct and the wrong test, and the declared set already refutes it as stated.
`threads` is a declared axis, correctly, and no const predicate in arvo can gate on thread count:
op's I10 says arvo takes no stance on cores, and a thread count is not const-available, which is
the very category I13's "whatever is available at const time" instruction bounds. `access_pattern`
is declared, correctly, and is a property of what the consumer's loop does, not anything an arm
gates on. So if gateability were the axis-hood test, two of the sixteen declared rows fail it, and
nobody proposes deleting them.

What I13 actually uses gateability for is **value exactness**, not axis-hood: "`F = 0` qualifies;
'usually small F' does not, because nothing can gate on it." That polices the grammar of a value,
and it is doing that job well. The header of `dimension.toml` extends it to key classification and
stamps the extension "comes straight from I13", which over-claims its provenance; I return to that
in section 6.

The test that does the work, and I offer it as a replacement:

**An axis indexes situations; an instrument parameter indexes runs.** Ask of a key: can a consumer
of arvo, or the design itself, *be at* a value of this key? A workload has an ambient domain, a
format has a radix, a build has a feature set, a deployment has a thread count. Those index
situations, and a claim's truth is a function of the situation, so they are axes, whether or not
any arm can const-gate on them today. Nothing a consumer does puts them at `arms = 5`; the bench
was at `arms = 5`. That indexes the run that produced the evidence, and it belongs on the `probe`
row (`establishes`: "...at the widths and shapes it checked", the schema already says exactly
this). Gateability then falls out as a *corollary* for the subset of axes that are also
const-available, which is the subset arms are built over; the axes beyond that subset still bound
where claims hold, which is what a predicate is for.

Applying it to the top of `unblock_value.out`, without prejudging the tail: **the ambient domain**
(a workload's values live in a domain; sole blocker of four spans) and **radix** (a format has one;
sole blocker of one, present in fourteen) are design axes and should be declared. **The cost-model
population** (25 spans) and **arms / selector / baseline** shapes index the instrument and belong
on probe rows. **The compilation environment** (9 spans) indexes situations the same way the
already-declared `target_features` and `build_profile` do, so it wants declaring or folding into
those two. **Accumulator width** and **the staged-narrowing widths** are design choices an
implementation is at a value of, situation-indexing, axes. The middle of the list ("assignment and
observation sets", "term shapes", "declarations") looks like formal apparatus from the topic's own
statement language rather than either category, and I hold that judgment for phase two, where I may
read what the spans actually say.

### 1.4 The third option, and why it is refused

Is there a way to declare an axis that does not reach backwards? Every shape I can construct is one
of three, and each dies on op's words:

- **Timestamp the vocabulary** (a row's negative space bounded by the axis set at its commit time).
  This makes the meaning of a row a function of a date join, so a predicate stops being exact on
  its face, and it institutionalises the fourth state, "written before we knew", which is
  "unmeasured" wearing a timestamp. Op: "No adding 'unsure' into the predicate."
- **Per-axis softened absence** (absence of a *young* axis read as silence rather than the strong
  negative). Same fourth state, minus the timestamp. And note the corpus already contains one
  instance of this shape, the `access_pattern` note; section 6.
- **Grandfather clause in the checker** (old rows exempt from the new axis). The checker never
  enforced world-semantics on absence anyway, so there is nothing to exempt; this is a no-op that
  reads as a guarantee.

There is no third option because there is no need for one: the backwards reach is imaginary. What
must be enforced instead, and cheaply can be, is that the **axis vocabulary is itself append-only**:
a `dimension` row is never deleted and never renamed, because *that* genuinely rewrites meaning,
turning a written span into an unparseable one and a written absence into nonsense. If the checks
do not pin this today, they should.

**Decision.** Declare the ambient-domain axis, and with it the small set of clear
situation-indexing blockers (radix, the compilation environment or its fold into the two declared
neighbours, accumulator width). Route instrument parameters to `probe` rows. The committed rows are
untouched, unedited, and un-widened, per the ratified mechanism; migrating seats writing new rows
from old spans write them under the current vocabulary, which is not retroactivity, because a
registry row authored today is a new claim by op's own paragraph. The append-only rule does answer
the seat's worry, and the worry itself, stated as "rewrite", is wrong in a way that matters,
because it would license per-axis absence semantics as the lesser evil, and that is the greater
one.

---

## 2. An enumeration owes its instrument whenever the walk was performed by code

The schema's five kinds split by *how the sentence came to be true*: proved (`theorem`), produced
by an instrument (`measured`), walked (`enumeration`), imposed (`normative`), reasoned
(`argument`). The gate as designed refuses a `measured` row naming no committed instrument. The
coordinator extended it to `enumeration`. I am asked to judge the edit, and my judgment is:
**right in effect, and the schema prose now owes one sentence of catching up, plus one boundary
the gate cannot see and reviewers must.**

The derivation. What distinguishes a 4096-triple walk from a timing? Noise, and nothing else. Both
are executions of code whose output is the claim; both are wrong if the code is wrong; the walk is
*deterministically* wrong, which is worse, because a rerun reproduces the defect instead of
scattering it. And an enumeration makes a claim a sample never makes: exhaustiveness. "All 4096"
is false if the loop bound was 4095, and the panel's own record has precisely this scar: "one
expert's headline counts turned out to be an artifact of its own enumeration bound" (`RULES.md`,
the citations section). The workspace's evidence rule has a second one: a fabricated enumeration
figure, 21,204 of 32,768, that could not have come from any walk of the claimed shape and travelled
through two briefs because no committed instrument existed to check it against. Enumerations do not
merely deserve the instrument requirement; they are the kind with the strongest claim to it,
because their numbers look the most exact and are the least often controlled.

The `probe` namespace's own description seals it: a probe row is "the difference between evidence
and assertion", and its `control` field is required because "an instrument that cannot come out any
other way produces a number and is not an instrument." A bounded walk has a natural control, and it
is worth writing into the guidance: plant a member at the last index, or check the walked
cardinality against the closed form, so the off-by-one that produced the panel's scar cannot
recur silently.

**The boundary the gate cannot see.** There is a class of enumeration that owes no instrument: the
one short enough that the row itself carries the walk. "The overflow policies seen in the corpus
are wrap, saturate, saturate-both-ends, clamp, panic", with the five listed, is verified by reading
it. But notice what that sentence *is*: a proof by exhaustion whose route is printed inline, and the
schema already has a kind for a proved sentence whose route is stated. It is a `theorem`. So the
clean rule, needing no schema change at all:

- **If the full walked list is in the row or its cited prose, the sentence is a `theorem` by
  exhaustion**, route inline, no instrument owed.
- **If the row carries a summary of a walk performed by code, it is an `enumeration` and owes the
  walker as a committed probe with a control**, exactly as the extended gate demands.

That gives the gate a crisp escape that is not a dodge: reclassifying to `theorem` requires
printing the route, and a 4096-triple route does not print, so the expensive cases cannot slip
out. The dodge pressure lands instead on `argument`, and there the corpus review, not the checker,
stands watch; a number in an `argument` row's `says` is the tell.

**Process note.** The edit was one expert's, on a gate that guards the canon's evidence spine, so
it wanted this second reading before it counted; it now has one, derived from the schema and the
scar record without reading the edit. Whether the *implementation* keeps this shape, and what the
two tests named for it actually assert, is my phase-two homework, and I will say plainly if the
bodies do not keep the names' claims.

**Decision.** Keep the extension. Add the theorem-by-exhaustion boundary as guidance (a
reviewer-applied test, in the schema comment or the checks' own documentation). Update the
`evidence` field's description, which currently names `measured` alone, so the prose and the gate
stop disagreeing.

---

## 3. Yes to a `definition` kind, with the violability test as its border guard

### 3.1 Is a definition an imposition?

No, and the difference is observable, not philosophical. Ask: **could the design conceivably
violate the sentence?** "Every chain lowers to one path" can be violated; it is a constraint on
artifacts, an imposition, `normative`. "A chain is a sequence of operations with no intermediate
observation" cannot be violated by any artifact; a thing either is one or is not. A definition
stipulates a meaning; it is not established (no evidence bears on it), not measured, not imposed
on the design. It *does* bind, but what it binds is every later speaker's vocabulary, which is a
different obligation with different consequences, and a schema that cannot tell the two apart
pays twice:

- **Compliance sweeps are polluted.** The moment anything asks "which rows can the design violate",
  which is what a canon's normative rows exist to answer, half the `normative` population (by the
  author's own accounting) returns false positives. A definition can never be complied with, so
  every coverage or conformance question over the current corpus starts by hand-sorting the kinds
  the schema was supposed to sort.
- **The ask to op is misdescribed.** He ratifies rows. Blessing an imposition is accepting a
  constraint on arvo; blessing a definition is accepting a name. Presenting fifteen rows uniformly
  as impositions asks the only ratifying human to bless constraints where half of them are namings,
  and misdescribing the ask to the one human in the loop is a provenance defect, not a taxonomy
  nicety.

### 3.2 Does a definition carry a region?

No, and the dispatch's own suggestion is the right one: a definition of a chain over `F = 0` and
one over `F any` are **two definitions**, not one definition with a region. The domain restriction
is part of the *definiendum* and belongs in `says`. The predicate machinery marks where a claim was
*established*, and a definition is established nowhere because establishment does not apply to it.
So `definition` joins `normative` as predicate-free, and the existing rationale transfers exactly:
a region on a stipulation would say the term means this in the measured region and who-knows-what
elsewhere, which inverts stipulation the same way a region on an imposition inverts it.

### 3.3 What a `definition` row owes, and what stops it becoming the dodge

It owes three things:

1. **No predicate**, as above, and the existing normative-with-region check extends to it in one
   branch.
2. **Uniqueness per definiendum.** One term, one live definition; a second definition of the same
   term carries `supersedes`. Two live definitions of one term is the one defect class unique to
   definitions, it is mechanical to check once rows name their definiendum, and it is exactly the
   two-theories-of-one-phenomenon failure. Worth a field (`defines = "<term>"`) so the check does
   not parse prose; that is the only schema surface the kind genuinely needs beyond the enum value.
3. **The violability test at review.** If the design could conceivably violate the sentence, it is
   `normative`, whatever its author called it.

And the dodge worry dissolves on inspection: **the predicate gate cannot be dodged through
`definition` any harder than it already can be through `normative`**, because `normative` is
already predicate-free and already admits anything a hopeful author writes into it. Adding
`definition` does not open a new door; it splits the existing predicate-free population into a
half that can be complied with and a half that cannot, which *narrows* the hiding space, because a
`definition` row asserting anything empirical is caught by a test (`could it be violated?`) that no
one can apply to the undifferentiated pile today. The symmetric abuse, a stipulation that smuggles
an empirical claim ("define the chain error to be negligible"), is caught by the same test: that
sentence is violable, so it is not a definition.

One more thing, since the schema borrows discipline from its neighbours: the builtin `vocab`
namespace ("small closed sets the project names") is where some of this material likely *lands*
after ratification. That is a locus question for the porting dispatch, not this one; the
`definition` kind is the transit marking either way, because a proposal must pass through the
panel-to-op pipeline whatever namespace its ratified form settles in.

**Decision.** Add `definition` to `sentence_kind` (name it `definition`; `stipulation` is more
exact and worse to search for). Predicate-free, `defines` field for the term, uniqueness checked,
violability policing the border. The fifteen `normative` rows get re-sorted by their author in a
later dispatch, which is a re-marking, not a re-arguing.

---

## 4. The refusal is right, `retirement` is wrong, and the fourth thing is one ruling row recording the withdrawal itself

The material: op's prior calls, whose authority op withdrew in his own words, "not as calls, not as
ratified intents, but as historical log of my calls, explicitly connected to a *failure*", and
"none of it absolute. The answers are likely wrong, and the questions they answer, are also
probably wrong. So substance itself is only good as extra stuff to consider or explore, nothing
more." He simultaneously licensed mining it for reference and taste.

**The seat's refusal to port it into `ruling` is right**, and the schema says why in its own
description: `ruling` is "the highest-authority material in the registry and the only material
with a human in the loop", where *being in the namespace* is the provenance claim. Every `rung`
value describes a live standing of his word: `ratified`, `in_force`, `stated`, `open`. "His words,
authority withdrawn by him" is not a fifth standing of his word; it is the absence of one. A sixth
value whose sole function is to be filtered out of every query that touches the namespace is a
misfiled row wearing a warning label, and every consumer of `ruling` forever after carries the
filter or inherits the laundering hazard the rules section already names: "an agent that cannot
support a position finds a prior op call agreeing with it and cites that."

**`retirement` is also wrong**, in the direction the dispatch already senses. Its description says
"must not be cited again"; op licensed citing it, for taste and for things-to-test, and RULES.md
canonises both uses as real. And its `kind` enumeration cannot say what happened: the claims are
not established `wrong` (op says *likely* wrong, which is a withdrawal of warranty, not a
determination), not `superseded` (nothing specific replaces them), not `unpayable`,
`misattributed`, or `unpredicated`. A retirement row would be exact about the one thing that is
false (unciteable) and silent about the one thing that is true (unwarranted).

**The fourth thing** follows from the schema's own pattern, stated on the `ratifies` field: "a
blessing is a new row naming what it promotes rather than a flag flipped on the old one." A
withdrawal is the same act with the sign reversed: **a new row naming what it demotes.** And here
is the pleasing part: the withdrawal *is itself an op statement*, made with a human in the loop, in
his own quotable words. It is the one piece of this material that belongs in `ruling` at full
strength:

- One `ruling` row. `kind = process` (a call about how the work is conducted: how that corpus is to
  be read). `rung = stated`. `says`: the prior-calls corpus carries no authority and is not to be
  cited as calls, intents, or canon. `quote`: his words above, verbatim. `instead`: mine it for
  reference, taste, and things to test; a claim from it re-enters, if at all, on its own current
  merits. `provenance`: where he said it, and the corpus it governs.

The withdrawn corpus itself gets **no rows at all**. A registry row exists to be cited, and this is
material op has said must not be cited as authority; it stays in the frozen research tree, which is
precisely the tree's job ("the audit trail: what was argued, by whom... and what op ruled").
Any individual claim out of it that the panel still wants re-enters as a fresh `proposal`, argued
on today's evidence, citing the old file in `provenance` as where the idea came from. That is
exactly what the corpus now is: material with an idea's standing and not a ruling's, and the
schema already has the namespace for established-but-unblessed material. Nothing new is needed:
no sixth rung, no retirement rows, no third namespace. One row, and the mechanism that guards
against the laundering failure is that `refsto` and the row's keywords make the withdrawal
findable from anywhere the corpus is met.

**Decision.** Refusal upheld on independent grounds. Port the withdrawal as one `ruling` row;
port none of the withdrawn material; re-entry is by fresh proposal per claim. I note the dispatch
says `179` section 17 was written as a brief to be dispatched against, carrying the strongest case
against the seat's own refusal; I derive the above without it and will meet that case in phase two.

---

## 5. A refusal's scope is what its ground reaches, so the severable parts stand, under three conditions

The situation as the dispatch states it: a clause was refused by a signature, not on wording but
because **no wording of it is true on both branches of a premise** until op rules (this is
recognisably the container premise; `dimension.toml`'s `container` row names it as blocking
"several downstream clauses" for exactly this reason). A seat took the two parts of the clause
that both signatures had signed and wrote them as a row, leaving the refused sentence out.

**Is a partially-signed refused clause a proposal, or is a refused clause refused whole?** Neither
horn as stated; the fork dissolves when the refusal is read as a reasoned act rather than a curse.
A refusal has a ground, and its scope is what the ground reaches. This one's ground is
branch-dependence: the joint sentence asserts something whose truth flips across the two branches
of an open premise, so no fixed wording of *the joint* is assertable now. That ground reaches any
part that is itself branch-dependent, and it does not reach a part whose truth is the same on both
branches. Refusing the reachable part along with the unreachable ones would be treating the refusal
as contaminating by adjacency, which no principle here supports, and which the registry's own
grammar contradicts: `ruling` distinguishes `declines` from `refusal`-of-the-thing precisely
because "declining the paper and refusing the thing are different acts." Op's catalogue models
partial adoption within a single statement twice over: I8, where one clause of a verbatim
quotation is the intent and the next clause is provenance-only filler by op's explicit word; and
I18, where two of his sentences stand with "the second is what fixes the first."

So the row is legitimate **if and only if** three conditions hold, and they are checkable in phase
two rather than assumable here:

1. **The parts were signed severably.** A signature on "A and B, jointly, in the context of C" is
   not a signature on A and on B. If the signatures endorsed the parts as standalone claims, the
   row records two-expert agreement that exists; if only in the joint's context, the honest
   `standing` is `contested` or `one_expert`, not `two_experts`.
2. **Each written part is itself true on both branches of the premise.** Otherwise the refusal's
   ground reaches it and it goes down with the joint. This is the condition I most want verified,
   because a part can *look* premise-free while quantifying over something the premise decides.
3. **The row's `gap` names the excluded sentence and the premise it waits on**, and the premise
   exists as a `question` row with `decider = op`. Without the gap, a reader reconstructs the joint
   by adjacency, and the row has smuggled in exactly what was refused. The dropped sentence must be
   visibly dropped, not silently absent, or the compression rules' scar repeats: the material most
   likely to be lost is the open part.

One boundary worth saying plainly: the workspace's "never re-propose a refused shape" binds against
shapes *op or the brief* refused. This refusal is a signature's, an agent-tier act, conditional on
an open premise op has not ruled. Recording the signed remainder is not laundering a refusal into a
decision, *provided* condition 3 holds; omitting the gap is what would convert it into one.

**Decision.** Not refused whole. The seat's call stands conditionally; the three conditions are the
audit, and the seat flagging it as its own most overturnable call is the correct instinct pointed
at the correct place (condition 1 is where it would overturn). Phase two verifies all three against
the signatures' actual text.

---

## 6. Findings outside the five questions

Reported under the standing instruction, each with its citation.

**6.1 The `access_pattern` note licenses a fourth omission state, against op's recorded mechanism.**
`dimension.toml`, `access_pattern`, `moves`: "a correctness claim leaving it out is not making the
strong negative statement the notation otherwise assigns to an absent axis." That is per-axis
softened absence: a claim class for which omission is read as silence. Op, quoted in `RULES.md` and
in the workspace rule: "threads don't matter is a statement for adding threads any... **it has to
be written**, because nothing infers it", and "the notation gives no middle ground where a hedge
could sit." The honest form for a correctness claim untouched by access pattern is
`access pattern: any`, written, with the structural argument as its warrant, exactly as a
compile-time result writes `threads any`. The note infers what op said nothing infers. One row,
one sentence, and it is the thin end of the wedge question 1 is about: if absence can soften
per-axis by annotation, the ambient-domain "rewrite" worry becomes coherent, and the notation's
exactness dies by a thousand humane exceptions. The `moves` sentence should state that the axis
moves cost only; the omission-semantics gloss should go.

**6.2 The `dimension.toml` header claims I13's warrant for a test I13 does not state.** Header:
"The test is gateability, and it comes straight from I13." `INTENTS.md`, I13, final paragraph:
"The scope of this entry is those two paragraphs... Anything further, including the dimension
list... and the exactness bar for a predicate, is elaboration... and is **not** part of what was
ratified." Gateability-as-axis-test is elaboration twice removed, it conflicts with the declared
set's own contents (section 1.3), and stamping it "straight from I13" is provenance inflation on
the one entry whose scope op explicitly bounded. The test I offer in 1.3 does not claim his
warrant either; it is one expert's elaboration awaiting its second reader, and says so.

**6.3 Two grammars admit spellings their own notes forbid.** `operation`: grammar admits
`operation any`, note says it "is not writable while the operation set is open." `strategy`:
grammar admits `S any`, note says a row writing it "is claiming more than the corpus can currently
close." A checker enforces grammar and cannot read notes, so the admissible-but-forbidden spelling
will eventually be written and will validate. Either the grammars drop `any` while the sets are
open, or the check learns the two exceptions; the current split is a gate with a gap the width of
the exact failure it guards.

**6.4 Minor, same class:** the schema comment on `proposal.evidence` names `measured` alone while
the gate (per the dispatch) now also covers `enumeration`; one sentence of drift between prose and
check, noted in section 2, fix alongside.

---

## 7. What phase one could not answer

Stated per the dispatch's instruction, so the phase-two append is honest about which agreements
are corroboration and which are verification.

- **Q5 conditions 1 and 2**: whether the signatures endorsed the parts severably, and whether each
  part is branch-independent. Needs the signature files themselves.
- **Q2's implementation**: whether `shape.rs` draws the gate where section 2 argues, and whether
  the test bodies (`an_enumeration_owes_an_instrument_and_a_theorem_does_not`, and the measured
  sibling) assert what their names claim. Needs the code, which is phase-two reading here by the
  dispatch's ordering.
- **Q1's blast radius**: the exact count of committed rows carrying predicates today (the brief
  says fifteen of seventeen proposals are normative; `law` rows carry `holds`/`fails` and I have
  not counted them). Counts are measurements; phase two produces it with a command.
- **Q4's strongest counter-case**: `179` section 17 was written to be dispatched against and I have
  not read it, so my section 4 is a cold position, not a reply to it.
- **Whether a `question` row for the container premise exists** with `decider = op`, which Q5's
  condition 3 needs.
- **Which spans the middle of the unblock ranking actually sit in**, for the axis-against-apparatus
  sort of keys like "assignment and observation sets"; I declined to classify them unread.
