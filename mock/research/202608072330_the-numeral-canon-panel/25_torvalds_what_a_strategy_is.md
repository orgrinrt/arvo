# 25. What a strategy is

**Date:** 2026-08-08. **Author:** the `torvalds` persona. **Predecessor:** `24_amin_the_seam_between_two_vocabularies.md`.
**Probes:** `25_probes/`.

**Status: COMPLETE.**

## 0. The answer, before the working

A strategy is **a name the consumer writes to select one policy for how a numeral is represented and
how its arithmetic behaves.** It is not itself an axis, and calling it one is the mistake that kept the
definition from being written.

Stated as the canon sentence I am proposing:

> A **strategy** is a consumer-written name standing for one coherent policy over representation and
> arithmetic. The policies it fixes are independent of each other and each is an axis in its own right:
> how much headroom the container carries beyond the numeral's width, whether values are packed or
> individually addressable, what happens on overflow, and what precision an intermediate carries. A
> strategy assigns one value on every axis. It does not assign a constant: each assignment is a
> function of the build condition, of which a constant function is one case. The named strategies are
> therefore **sections over a product of axes**, not values of a single axis, and the substrate never
> picks one.

The discriminating test that falls out of it, which is what the dispatch asked for:

> A proposed new strategy **is** one when it can be written as an assignment of values on the axes that
> already exist. It **is not** one when it requires a new axis, because then every existing strategy
> also needs a value on that axis and what is being proposed is an extension of the space rather than a
> point in it.

**One axis or several: several, and the answer is not close.** Syntactically it is one type parameter
position, which is why the panel has been saying "the strategy axis" for two panels. Semantically it is
a section over at least four independent axes, and the four named strategies do not even span them:
they are the complete two-by-two of the two storage axes, with the two arithmetic axes filled in
partially and inconsistently. Section 4 shows the two-by-two and section 4.2 shows the inconsistency,
which is a real defect in the preset table and not a defect in the definition.

**What is op's, and it is most of the load:** whether the arithmetic column of the preset table is one
axis or two is a question the record does not answer, and it decides whether `Cold` has an overflow
policy at all. Section 8 states it. The definition above does not depend on the answer, which is why I
am shipping the definition rather than blocking on the question.

**And one thing found on the way that is bigger than my dispatch.** `mock/benches/` holds a committed
34-run family, `warm-clamp-arity-*`, that no file in this panel cites and that varies two of the axes
against each other under a fixed strategy. It confirms empirically that the right value on those axes
is not a constant, which is the load-bearing half of the definition. It also shows the shipped doubled
container losing in all 34 runs of that workload, which belongs to the headroom question rather than to
mine and should not sit uncited through a third panel. Section 6.2, probe `25_probes/p4*.py`. This is
the same failure `RULES.md:175-197` records costing eighteen files: a bench directory nobody was told to
look in.

## 1. The canon gate

**Passed, with one thing to hand back.**

Checked against `RULES.md`, `01_op_answers.md` section 0, `SETTLED.md`, and op's own checkpoint files
in the closed panel at `mock/research/202607301300_formalization-spec-panel/`.

The work is aligned: `23:1023-1029` names this exact hole and asks for exactly this sentence, and no
ratified text forbids writing it. Nothing I propose reopens a converged question.

**The one thing.** The dispatch says "Four of them exist by name", and `SETTLED.md:82-87` carries six
rows about the strategy axis, one of which is op declaring intent as settled canon outright. My
definition has to be consistent with those six rows rather than derived freely, and section 3.3 checks
it against each. That is not a refusal; it is a narrowing of what was open.

## 2. Checking the brief's own numbers

The dispatch says eight rows mention the axis, three name it inside their canon sentence, and none
defines it. All three are checkable and I checked them before reasoning from them, per `RULES.md:124-126`.

**Eight and three: reproduce exactly.** These originate at `23:1023-1025` and the script is committed
at `23_probes/count_strategy_mentions.py`. Re-run from the panel directory:

```
$ python3 23_probes/count_strategy_mentions.py
rows total: 30
rows mentioning 'strateg' anywhere: 8 -> S18, S19, S21, S22, S23, S25, S26, S29
rows whose canon sentence says 'strateg': 3 -> S19, S23, S26
```

So the brief inherited the numbers from `23` correctly, and `23`'s script reproduces. I note that the
"canon sentence" test is a heuristic (the row's leading blockquote), which is a reasonable operational
definition and not the only one. I did not re-derive the classification by hand.

**None defines it: holds, and it holds more widely than the brief claims.** `23`'s script does not test
this, so the third claim was unverified when it reached me. I wrote my own at
`25_probes/p1_hunt_for_a_definition.py`, which sieves for copular and defining frames with the strategy
as the grammatical subject, across every panel `.md` (including `CANON_CANDIDATE.md`, `SETTLED.md` and
`MORNING.md`), arvo's own generated agent rules under `.claude/`, and the workspace rules. Output at
`25_probes/p1.out`.

Fourteen hits in the panel, one in arvo's rules, zero in the workspace rules. **Not one of the fifteen
is a definition.** Reading them:

- Two are negative statements about the derivation, not about the strategy: `10:193` and its quotation
  at `11:544-545`, "The ladder does not know what a strategy is; it maps a width to a container."
- Two are the hole being named rather than filled: `23:1023`, `MORNING.md:108`.
- One is a partial functional description, `06:306`, "The strategy is what breaks that tie", which says
  where it is consulted and not what it is.
- Three are measurements of what a strategy costs: `22`'s sentence quoted at `23:813`, and `23:1096`.
- The rest are the panel disagreeing about where the strategy sits relative to the ladder (`23:709-710`).
- The single hit in arvo's own rules is a heading, `implementation.md:31`, "Strategy markers are
  load-bearing", followed by a table rather than a definition.

So the claim is right, and the sharper version of it is: **the record contains a great deal about where
a strategy is consulted and what it costs, and nothing about what it is.** That is the signature of a
concept that was introduced by example and never abstracted, which is exactly what section 3 finds.

## 3. What the record treats a strategy as

Per the dispatch, worked from uses rather than from statements. A thing is what its uses make it.

### 3.1 The preset table, which is the whole of the by-example introduction

arvo's own generated agent rule at `.claude/rules/implementation.md:31-64` is the most direct statement
in the repository. It is generated from `mock/agent/`, so it is design-tier rather than dead-tree
source, and it is the text every agent working in arvo has loaded. It says:

> The strategy drives storage container width, operation semantics (wrapping vs saturating), and SIMD
> lane count.

and then gives the table (`implementation.md:52-58`), which I reproduce because the decomposition in
section 4 is read directly off it:

| Strategy | Container | Arithmetic | Use case |
|---|---|---|---|
| `Hot` | minimum byte-aligned | wrapping | inner loops, known-safe range |
| `Warm` (default) | 2x logical, byte-aligned | wrapping (safe on single ops) | default |
| `Cold` | bitpacked minimum | widen-op-narrow | storage density |
| `Precise` | 2x logical, bitpacked | saturating | correctness-critical |

Three things this establishes about what the record treats a strategy as, before any interpretation.

**It determines more than one kind of thing.** The prose names three (container width, operation
semantics, SIMD lane count) and the table's columns name two more precisely. Whatever a strategy is, it
is not a single quantity.

**It is a fixed vocabulary of four, introduced by enumeration.** There is no rule given for what would
make a fifth legitimate. That absence is the hole this file exists to fill.

**It is required rather than optional.** `implementation.md:60-64`: "If you add a new numeric type and
the type admits a precision tradeoff, it MUST carry `S: Strategy` or justify the omission", backed by
the `strategy-marker-required` lint. So the record treats it as a property every numeral has, not as an
annotation some numerals opt into. That matters for the definition: a strategy is total over numerals.

### 3.2 Where it is consulted, which the panel disagrees about

`23:701-710` records the disagreement squarely and it bears on the definition:

> Under `10`'s, the strategy is upstream of the ladder and the ladder is strategy-blind. Under `15`'s,
> the strategy is a key of the ladder. Both compile.

My definition is deliberately neutral on this. Whether the derivation consumes the strategy directly or
consumes the axis values the strategy resolved to is a question about the derivation's factoring, and
both readings agree that the strategy is what supplies the values. `23` reports the observable
difference is what a diagnostic can say, which is a real question and not this one.

I flag it because a definition that accidentally picked a side would be smuggling an answer into a
question the panel has open. Mine does not, and section 7 states the sentence in a form that survives
either.

### 3.3 The six settled rows, which are the part I had to conform to rather than derive

`SETTLED.md:78-87` carries six rows on the strategy and profile axis. Four bear on the definition. I
went to op's own checkpoint files rather than reading rungs off the summary, because `23:918-926`
records making exactly that mistake inside a file auditing rungs, and `21` §2 found four of them in
`MORNING.md`.

**"Everything varies granularly, and a constant is a function rather than the alternative to one."**
Sourced at `143b:10-12`. Verified: `143b_op_checkpoint_thirtysix.md:10-12` is op's own words,

> Function can also be a constant. It's not either or there. And all things change and act granularly,
> not just warm. I call this as intent, settled canon, right now. This small bit in this association
> now governs future talks.

This is the single most consequential input to the definition and section 4.4 works out why.

**"The existing preset tables are one arm, plausibly the debug-assertions arm, and were always
incomplete."** Sourced at `143b:89`, which is the section heading. Verified, and the words under it are op's own, at
`143b_op_checkpoint_thirtysix.md:81-83`:

> Yes it is incomplete by a lot. I was under impression we are first tackling the basic shape, perhaps
> the one we reserve for debug assertions time, and we write separate arms for release and such then?
> It's always been incomplete. Nothing changes in standing base.

So the table in section 3.1 is **one evaluation of the strategies, not the strategies**. Any definition
that reads the table as the content of the four names is wrong on op's explicit intent.

**"The strategy cells are functions of the profile."** Sourced at `142c:57`, the section heading; the
claim itself is at `142c:59`. I read `142c` and it holds as a claim of that file; the generalisation at `143b` supersedes its framing (`143b:16-17` says the
either-or framing "is wrong" and was the dispatching agent's), and the surviving content is that the
cells vary.

**"arvo and notko concepts do not correspond."** Sourced at `144b`. Verified at
`144b_op_checkpoint_thirtyseven.md:10-16`, op twice:

> Notko or hv are not directly associated with arvo. The concepts need not align, they are different
> things for different purposes and in different projects. They have synergy, but no continuity as such.

> Again, arvo strategy is not the same as notko optimize for profiles. They have synergy, nothing more.

This is the boundary the dispatch asked me to put in the definition and section 5 does.

**One line from `144b` I am pulling forward because it is nearly the definition already.**
`144b:52` lists what survives the correction, and one item is:

> An attribute may select a strategy and must not resolve a cell.

That sentence presupposes the whole shape: strategies are selected, cells are resolved, and the two are
different acts performed by different things. Whoever wrote it had the distinction. Nobody abstracted
it into a definition.

## 4. One axis or several

Several. The evidence is arithmetic and I ran it rather than asserting it.

### 4.1 The four names are the complete two-by-two of two storage axes

Read the Container column of the section 3.1 table as two independent choices rather than one label:

- **Headroom**: does the container carry width beyond the numeral, or the minimum that fits?
- **Layout**: are values individually addressable, or packed?

Then:

| | byte-aligned | bitpacked |
|---|---|---|
| **minimum** | `Hot` | `Cold` |
| **doubled** | `Warm` | `Precise` |

Every cell filled, no cell repeated, nothing left over. Probe at
`25_probes/p2_decompose_the_preset_table.py`, which parses the table out of
`.claude/rules/implementation.md` rather than out of anything I typed, and checks the two-by-two is a
bijection.

**This is the finding that settles the question.** Four names filling a two-by-two exactly is not what
four values of one axis looks like. It is what a product of two binary axes looks like when somebody
gives each corner a name. The names are presets.

And it is corroborated independently, which matters under `RULES.md:116-118`. A workspace memory,
`arvo-strategy-is-a-preset`, reaches the same conclusion from prior art rather than from arvo's table:
measured against SystemC (IEEE 1666) `sc_fixed`, AMD Vitis HLS `ap_fixed`, Siemens Algorithmic C
`ac_fixed`, and MATLAB `fi`/`fimath`, it concludes

> the strategy markers are **presets** over (container width policy, overflow policy, intermediate
> precision), not an axis.

Four industrial systems agreeing on a decomposition is genuinely independent of arvo's own table, and
it reaches "preset, not axis" by a different route. That is two instances. The third is section 4.2,
which finds the decomposition by finding where the table breaks, and the fourth is section 6.2, where a
committed harness family varies two of the axes against each other 34 times.

I record one discrepancy honestly: the memory's three axes and my three are not the same list. It folds
headroom and layout into one "container width policy" and adds intermediate precision. Mine splits the
container in two because arvo's own table varies them independently, which the memory's sources do not
force. Both agree on the shape and disagree on the cut. Section 7's sentence names four axes, which is
the union, and I flag the cut as unsettled in section 9.

### 4.2 The arithmetic column is two axes, and each preset answers only one of them

This one I did not expect and it is the sharpest thing in the file.

The Arithmetic column reads: `Hot` wrapping, `Warm` wrapping, `Cold` widen-op-narrow, `Precise`
saturating. Treat that as four values of one axis and it is incoherent, because **widen-op-narrow is
not an answer to the same question that wrapping and saturating answer.**

- "Wrapping" and "saturating" answer: *what happens when a result does not fit.*
- "Widen-op-narrow" answers: *what precision does the intermediate carry before the result is stored.*

Those are orthogonal. A widen-op-narrow evaluation still has to say what happens when the narrowing
step does not fit, and the table does not say. Symmetrically, `Hot`, `Warm` and `Precise` do not say
what their intermediates carry.

So the table has **four cells covering two axes, with each cell filled on one axis and silent on the
other**. `Cold` has no stated overflow policy. `Hot`, `Warm` and `Precise` have no stated intermediate
precision.

This is not me finding a typo. It is the mechanism that hid the definition for two panels: as long as
the third column looks like one column, the four strategies look like four values, and nobody asks what
the values are values *of*. The moment the column splits, the four names are visibly points in a
product and the definition writes itself.

The prior-art memory reached the same split from outside, listing "intermediate precision" as a
separate axis that in arvo is "buried inside the strategy marker". Buried is right, and section 8 hands
op the consequence, because whether `Cold` clamps or wraps is a design question and not mine.

### 4.3 Why the table cannot settle how many axes there are, and what follows

I went looking for a fifth axis and found instead that the table is structurally incapable of
answering the question. This is the sharpest methodological point in the file and it changes what the
rest of the evidence is worth.

**Four presets are four data points. The two-by-two consumes all four, with zero degrees of freedom
left.** So once headroom and layout are fixed as axes, every further property attached to the presets
(overflow, intermediate precision, SIMD lane count) hangs on a point that is already uniquely
determined. Nothing in the table varies them independently, because there is nothing left to vary them
against.

Three consequences, and they are not small.

**Independence of the arithmetic axes cannot be established from the table.** Section 4.2 shows the
arithmetic column contains cells answering two different questions, which is a claim about the cells'
content and is sound. It does not follow from the table that intermediate precision can be set
independently of headroom, because the table never exhibits two presets sharing a headroom and
differing in it.

**It does follow from the benches, and I found that after writing the paragraph above rather than
before.** Section 6.2 is the evidence: a committed harness family holds strategy, overflow and layout
fixed and varies the accumulator against the container, across 34 runs. That is the independence the
table cannot show, demonstrated in arvo's own repository rather than borrowed from prior art. I have
left the table's limitation stated because it is true and because it explains the two panels, and the
correction sits next to it rather than replacing it.

**Whether SIMD lane count is a fifth axis is unidentifiable from the table, and on reflection the
question is malformed.** `implementation.md:34-36` says the strategy "drives" it, and driving is
downstream. Lane count is a function of the container width and the target's vector width, and
container width is a function of headroom and the numeral's width. So it derives, and a derived
quantity is not an axis. I state that as a derivation rather than as a measurement, precisely because
the table cannot measure it.

**And this is the general reason the definition had to come from somewhere other than the table.** A
table with exactly as many rows as its axes have combinations can be read back as almost any
decomposition. That is why eight rows could use the word and none define it: everyone was reading the
same four points, and four points support many stories. The definition in section 7 is anchored on op's
rulings about arms and functions, on the prior art, and on two bench families, none of which is the
table. The table is what the definition has to be consistent with, not what it is derived from.

### 4.4 A strategy assigns functions, not values, and that is op's intent

Section 3.3 established the table is one arm. Combine it with op's ruling that a constant is a function
and the general form falls out.

A strategy does not assign a value on each axis. It assigns **a function from build condition to a
value** on each axis, and a constant assignment is the special case where that function is constant.
`143b:24-27` puts the burden of proof exactly there:

> **Being constant is now the special case**, and it is a claim about a function's behaviour over its
> domain. It has to be established, not assumed.

So the type of a strategy, written as a relation rather than as an implementation:

```
strategy  ×  build condition   ->   headroom × layout × overflow × intermediate
```

curried the other way, a strategy is a name for a section: fix the strategy, and you have a function
from build condition to a point in the axis product.

**This is why "the strategy axis" is the wrong phrase and why it cost two panels.** The phrase makes
the four names look like four values of one primitive dimension. They are four named sections over a
four-dimensional product of axes, and the axes are the primitives. `S: Strategy` being one type
parameter position is a fact about the surface, not about the concept, and confusing the two is what
`23:1023` measured as eight rows using the word and none defining it.

## 5. What it is not

Three boundaries, and each is in the definition rather than in commentary, because a boundary stated
only in prose gets dropped by the next compression.

### 5.1 It is not notko's profile tier

`144b:10-16`, op twice, quoted in full in section 3.3. The two are separate projects with separate
purposes, sharing synergy and no continuity. Concretely: notko's `#[profile]` has three tiers against
arvo's four names, is lexical over what it annotates, and its own keys (`based_on`, `inline`,
`panic_fmt`) are about code generation rather than about numeral representation.

`144b:33-34` is worth carrying because it inverts what a panel would otherwise file as a defect: a
notko tier file cannot express an arvo posture, and **that absence is the separation working** rather
than a gap. The `Cold` name appearing in both is a coincidence of two words in two projects.

**And this boundary voided a claim that would otherwise have gone straight into my definition.**
`142c:59-60`, the same file `SETTLED.md` cites for "the strategy cells are functions of the profile",
continues:

> The strategy axis and the notko profile pipeline are one mechanism with two knobs

That is exactly the shape a definition-writer wants, and it is dead. `144b:10-16` is op saying twice
that they are not one mechanism. So the half of `142c` that survives is that the cells vary, and the
half that does not is what they vary with. My definition says a strategy's assignments are functions of
the build condition and deliberately does not say the build condition is notko's pipeline, because that
identification is the voided part. This is the clearest case in the file of the provenance ladder doing
real work: a well-written unratified sentence, sitting in a file the settled summary cites, and wrong on
a point op ruled on directly.

The definition's consequence: a build-condition mechanism may **select** a strategy. It may not
**resolve a cell**, which is `144b:52`. The strategy owns its own axis assignments; the environment
picks which strategy is in force. Those are different acts and collapsing them is the error the ruling
voided.

### 5.2 It is not the substrate deciding for the consumer

`arvo-toolbox-not-policer.md` is the workspace rule and it is unambiguous:

> The four strategy markers are not buckets the substrate picks for the consumer. They are explicit
> knobs the consumer turns based on knowledge the substrate cannot have.

It then lists what that knowledge is, and every item is about the consumer's workload rather than about
the numeral: what the hot path looks like, how many records flow through, whether access is contiguous,
whether overflow is a bug or acceptable, whether SIMD lanes matter on this target.

**This is load-bearing for the definition and not decoration.** It is why a strategy is a *name the
consumer writes* rather than a property the derivation computes. Everything else about a numeral in
this design derives: the container derives, the stored width derives, the representation derives.
`SETTLED.md:93` carries "The container is never written by a consumer" as ratified. The strategy is
the one input that does not derive, and that is its whole reason for existing. A definition that makes
it derivable has defined something else.

It also settles a boundary the dispatch asked about directly. A proposed fifth strategy justified by
"the substrate should pick this when the data looks like X" is not a strategy, because it is the
substrate deciding. It is a default, and defaults are a different mechanism.

### 5.3 It is not the numeral, and it does not change what the numeral denotes

This is the seam with `24`, whose sentence mine sits beside. `24` defines a numeral as a grid cut down
to a reach. A strategy does not move the grid and does not move the reach.

The check is `SETTLED.md:73-74`, the erasure gate: the consumer expresses usage in bits and bytes, the
typestate derives container and representation, validates, and erases on lowering. **A strategy is
consulted during derivation and is gone afterward.** Two numerals differing only in strategy denote the
same set of values; they differ in how those values are stored and what the arithmetic does at the
edges.

I want to be careful here rather than tidy, because this is where the definition could overclaim. The
overflow axis is *not* denotationally inert: wrapping and saturating are different functions on the
same domain, and the prior-art memory records the consequence precisely, that signed two-sided
saturating addition is not associative while wrapping is a group operation. So the honest statement is
narrower: **a strategy does not change which values a numeral denotes; it does change which functions
its operations denote.** Section 7's sentence says it that way.

## 6. Does it survive the bench evidence

The dispatch asks specifically, and names the flip with fold arity. Checked against `20` and `22`, and
then against a third family neither of them uses, which turned out to matter more than either.

### 6.1 What `20` and `22` already measured

**It survives, and one of the two results is direct positive evidence for the decomposition.**

`22`'s headline, quoted at `23:813`:

> The strategy axis is not carrying the trade the rule says it carries. It is carrying a footprint
> difference that is real for a consumer counting bytes and invisible to a consumer counting time.

Under a one-axis reading this is a refutation: the axis was supposed to trade time against precision
and it does not. Under the definition above it is not a refutation of anything, it is a measurement of
**one axis in isolation**. Layout (packed against addressable) is a footprint axis. It was never the
axis that trades time. Expecting a footprint axis to move a throughput number is a category error that
the one-axis framing invites and the decomposition removes.

`20`'s headroom result is the same shape from the other side, at `23:838-846`:

> The headroom rule guarantees the container exceeds the width at every width, and therefore guarantees
> the projection is a real instruction at every width. Its cost is not that a wider container is
> slower. Its cost is that it removes the case where the projection would have been free.

Ratios splitting exactly along filled against sub-rung widths, 44.2x / 0.98x / 21.0x / 7.0x / 0.99x /
2.45x. **That is the headroom axis carrying a real, large, width-dependent consequence, measured, while
the layout axis carries a footprint consequence and no time consequence.** Two axes, two different
kinds of consequence, opposite in what they move. A single axis cannot do that, and the two files
sitting next to each other is the empirical form of section 4.1's arithmetic.

**On the flip the dispatch names.** A definition making a strategy a fixed tuple of policies would be
embarrassed by a right answer that flips with fold arity, because then the right cell is not a property
of the strategy. The definition above is not, for the reason op already ruled: the assignments are
functions, and a function whose value flips with a condition is a function. The build condition is the
domain `143b` names; whether fold arity is in the same domain or in a different one is a real question
I am not answering, and I flag it in section 9 rather than assuming the domain is exactly the build
profile.

### 6.2 The family nobody in the panel has cited, and it settles the independence question

I went to `mock/benches/` to check `20` and `22`, and found a third family neither of them uses and no
file in this panel names: **`warm-clamp-arity-*`, 34 committed runs.** Probe at
`25_probes/p4_axes_vary_independently_in_the_committed_benches.py`, output at `25_probes/p4.out`. Meta
confirms the harness (`framework: mockspace-bench-harness`), the pinned toolchain
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), and an Apple M1 host.

Its own title states the arms, and I am quoting rather than classifying them myself:

> the shipped doubled container against minimum storage, against minimum storage with the fold
> lane-split, and against minimum storage with the accumulator sized by the design's own
> interior-safety rule

**Read that against section 4's axes and it is a controlled experiment on exactly the pair the table
cannot separate.** Strategy is held (Warm). Overflow is held (clamping). Layout is held. What varies is
the container's headroom and the accumulator's width, which is intermediate precision. So arvo's own
repository does vary an arithmetic axis independently of a storage axis, 34 times, with results.

Three findings, each a count I ran rather than a number I read:

**Three different arms win, and no arm wins everywhere.** `accfit` (accumulator by the interior-safety
rule) in 17 runs, `min-lanes` in 9, `minimum` in 8. **This is op's ruling holding empirically.** The
right value on these axes is not a constant; it is a function of the run's conditions, and a definition
that made a strategy assign constants would be refuted by arvo's own committed benches. Section 4.4 got
this from op's intent. The harness gets it independently, which is a second grounding for the part of
the definition carrying the most weight.

**The shipped doubled container wins zero of 34.** It is the baseline, and all 34 deltas are negative,
so a minimum-headroom arm beat it in every committed run of this family. I am flagging that and
deliberately not building on it: it is one workload shape (a clamping fold), it is the headroom rule's
territory rather than mine, and `20` and `22` are the files that own that question. It is reported here
because a family this size bearing on a ratified rule should not stay uncited for a third panel.

**Two runs in this family are uncommitted right now** and the probe skips them by design, printing which
ones. Under `RULES.md:106-109` a claim resting on them would be void, so the counts above are over the
34 tracked files only. The working tree arrived that way and I have not touched it.

**What this does not establish.** It varies headroom against intermediate precision. It does **not**
vary overflow against anything, so the independence of the overflow axis still rests on the prior art
alone. And the family's name asserts a clamping `Warm`, which is one side of op's own open question
about whether the default wraps or clamps. That does not damage the finding, since the independence
holds whichever the answer is, but it is worth op knowing a committed bench family has already picked a
side of a question he has not answered.

**One thing I will not do.** `22:614-618` says every arm wraps, and if the wide rung is supposed to
clamp then the bench measures the wrong semantics. That is `23`'s S26 blocker and section 8 is where it
goes. I am not adjudicating it, and my definition does not need it adjudicated: it needs only that
overflow is an axis, which is true under either answer.

## 7. The sentences

Two, plus a test. The first is the definition, the second is what makes it operational, and I have kept
both to the register `RULES.md:72-78` allows: naming, requirements, relations, no spelling of an
implementation.

> **A strategy is a consumer-written name for one coherent policy over how a numeral is represented and
> how its arithmetic behaves.** The policies it fixes are independent of one another, and each is an
> axis: the headroom a container carries beyond the numeral's width, whether values are packed or
> individually addressable, what an operation does when its result does not fit, and what precision an
> intermediate carries. A strategy assigns one value on every axis, and each assignment is a function
> of the build condition, a constant assignment being one case of that. Strategies are therefore named
> sections over a product of axes rather than values of a single axis. A strategy never derives: it is
> the one input a consumer supplies that the substrate cannot compute, because the knowledge it stands
> for is about the consumer's workload rather than about the numeral. It changes which functions a
> numeral's operations denote, and it does not change which values the numeral denotes.

> **A proposed strategy is one when it is an assignment of values on the axes that already exist.** It
> is not one when it requires a new axis, because then every existing strategy also needs a value on
> that axis, and what is being proposed is an enlargement of the space rather than a point in it.

**Permanence.** Both survive a total rewrite. Neither names a container, a width, a marker, a type
parameter, or a table cell. The four current names appear in neither sentence, which is deliberate:
`143b:81-83` says the present tables are one arm and always were incomplete, so a definition quoting
them would be describing an implementation and would need editing the moment the release arm is
written. It would fail permanence on op's own explicit intent.

**Equivalence.** Three teams implementing this independently produce things that behave the same on the
part that matters: every numeral carries a consumer-chosen strategy; the strategy determines headroom,
layout, overflow and intermediate precision; those four are independently settable; the values may
depend on the build condition; nothing about the strategy survives lowering except its effect. They
would differ on how many names ship and what they are called, which is correct, because that is the
arm and the arm is not the concept.

**Where it is weaker than I would like**, stated rather than hidden. The second sentence's test is sharp
only if the axis list is right, and section 4.2 shows the axis list is exactly what the record gets
wrong. If op splits the arithmetic column differently, the test still works and its inputs change. The
test's shape does not depend on the list; its application does.

## 8. What is op's

One question, and it is the one that blocks the axis list rather than the definition.

**Is the preset table's arithmetic column one axis or two, and if two, what is `Cold`'s overflow policy
and what are `Hot`'s, `Warm`'s and `Precise`'s intermediate precisions?**

The evidence that it is two is section 4.2: widen-op-narrow answers a different question from wrapping
and saturating, so the column as written has four cells covering two axes with every cell silent on one
of them. The prior-art memory reaches the same split from four industrial systems, each of which
separates overflow from intermediate precision as a matter of course.

**Why it is op's and not the panel's.** It is not a measurement dispute, so `01:96-98` does not send it
back. It is not a contested magnitude waiting for someone to build an arm. It is a question about what
the design intends the presets to mean, and the two answers are both coherent designs:

- **One axis.** Then widen-op-narrow is a legitimate value alongside wrapping and saturating, and the
  axis is "evaluation policy" rather than "overflow policy". `Cold` has no separate overflow answer
  because widening is how it avoids needing one, until the narrow, and the narrow's behaviour is then
  the thing that needs stating.
- **Two axes.** Then the table is incomplete in a specific, small, additive way: four cells to fill in,
  and per `143b:105-107` an append invalidates nothing.

**And it composes with a question already in front of him.** `MORNING.md`'s question five, carried at
`23:825-828`, asks whether the design's default strategy wraps or clamps. That is a question about a
value on the overflow axis. Mine is a question about whether the overflow axis is the axis that value
sits on. His answer to mine changes what his answer to that one is an answer about, so mine is prior.

**What I am not asking him.** Whether the wide rung clamps (`23`'s S26, blocked, and a measurement
question that `22` has already built the arm for). Whether the strategy keys the ladder or sits upstream
of it (`23:701-710`, a factoring question the panel should converge on first, per `RULES.md:62-66`).
Neither blocks the definition.

**A second read is owed on the axis list**, and I am the first expert on it. The definition itself rests
on four independent instances, which meets `RULES.md:116-118` with room: arvo's own table decomposing
into an exact two-by-two; four industrial systems decomposing the same way from outside; two committed
bench families measuring two axes with opposite-signed consequences; and a third committed family
varying headroom against intermediate precision across 34 runs with three different winners. The
specific four-item list in section 7's sentence is weaker than the definition it sits in: it rests on
one table, one memory and one bench family, and wants somebody to derive it before reading me.

**And the evidence for the split is now stronger than it was when I wrote section 8's question**, which
is worth saying plainly because it changes what op is being asked. Section 6.2's family holds overflow
fixed and varies the accumulator, which is intermediate precision behaving as an axis in arvo's own
harness rather than in SystemC's documentation. So the question is no longer "is this plausibly two
axes". It is "the design has been benching it as two axes for some time without saying so, and the
preset table still spells it as one column; which is the intent".

## 9. What I did not cover

Bounded honestly, per `RULES.md:103-104`.

**Read in full:** `RULES.md`, `01`, `23`, `SETTLED.md:70-99`, `143b`, `144b` (first 60 lines), arvo's
`.claude/rules/implementation.md` and `arvo-toolbox-not-policer.md`, the `arvo-strategy-is-a-preset`
memory.

**Read only in the parts that bear on this question:** `24` (the seam sentence and its section on what a
numeral denotes), `20` and `22` through `23`'s account plus spot checks of the quoted lines, `142c`
around lines 55 to 60, `15:426`. In `mock/benches/` I read the `warm-clamp-arity-*` family's titles,
meta and highlight lines across all 36 files and the full findings body of one; I did not read its
variant sources under `variants/`, so I am citing what the family measured and not how its arms are
written, per `RULES.md:110-114`.

**Did not read:** `CANON_CANDIDATE.md` beyond three greps, the panel files `02` through `22` in full,
the prior panel beyond the three op checkpoints, `MORNING.md` beyond confirming two line numbers exist
(the dispatch marks it contaminated and I did not reason from it).

**Did not verify:** that the four-axis list is complete. I found four; the field's fifth, rounding, is
absent from arvo entirely per the memory, and I have not checked whether a sixth is hiding the way
intermediate precision was. **The method that found the split is repeatable and somebody should run it
again**: take each preset's cell, ask what question it answers, and see whether two cells answer
different questions. That is how 4.2 happened and it took ten minutes.

**Did not verify:** the SIMD lane count claim from `implementation.md:34-36`. It is named in the prose and
absent from the table, so it is either a fifth axis or a consequence of layout and headroom. I lean
consequence and did not establish it, and if it is an axis then section 7's list is short by one.

**Did not touch:** the repository working tree, which arrived with modified and untracked files under
`mock/benches/` from `22`'s run. Left exactly as found, per the shared-clone rule. Nothing committed.

**Seven of my own citations were wrong before I checked them**, and I am reporting the number rather
than quietly fixing them, because it is the fifth instance of the same failure this panel has recorded
and the first where somebody counted. `25_probes/p3_verify_my_citations.py` opens every `file:line` I
use and tests it against a word I expect to find there, which is the part that matters: a citation
landing two lines off still resolves, and only the content test catches it. First run, 35 cites, 7
failures. After repair, 37 cites, 0 failures, output at `25_probes/p3.out`.

The failures were all the same shape, a heading or a paragraph boundary a line or two from the content,
and every one of them came from my having noted the line while reading rather than re-opening it while
writing. **The tooling cost about fifteen minutes and it should be standard**, because `23:918-926`
records making this mistake inside a file auditing rungs, `21` §2 found four in `MORNING.md`, and
`RULES.md:153-157` records a consolidation losing 61 of 78 citations. A script that reads the target and
checks a word is a complete answer to that class and nobody in either panel has written one.

**One thing the check found that is not an error**, and I record it so a later reader does not re-file
it as one. `SETTLED.md` cites `142c:57` and `143b:89`, both of which land on section headings rather
than on the sentences they carry. That is the citation style the workspace's own reference rule prefers,
since a heading fails loudly when renamed and a line number fails silently. Both resolve correctly. I
flagged them as suspects and cleared them, which is the right order.

**One route I closed and am recording so nobody repeats it.** I looked for a definition in the closed
panel's `CANON_CANDIDATE.md`, which carries a decomposition into associated types at its line 3163 with
a layout element and a comment naming container granularity. That is the shape of my answer expressed as
a mechanism, and it is **not** citable as the definition: it is implementation spelling, which
`RULES.md:76-78` forbids the canon from carrying, and it is one arm of the thing rather than the thing.
It is corroboration that somebody had the decomposition in hand and evidence that nobody lifted it into
an intent. That gap between having a mechanism and having the sentence is, as far as I can tell, the
whole story of why this hole existed.

**Status: COMPLETE.**
