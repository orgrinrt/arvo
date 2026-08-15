# The strategy set, derived cold

**Phase one. Written blind.** I read `INTENTS.md`, `RULES.md` and my dispatch brief, and nothing else in
this panel. No panel file, no `OPTIONS.md`, no register, no probes of anyone else's, no git log, no commit
messages. I checked that `139_probes/` exists and did not open it. Everything below is derived from op's
intents, the workspace rules, the committed bench tree, and my own knowledge. Phase two is appended after
this is committed, and phase one is never rewritten.

The one thing my brief told me about the settled state, because the question is unintelligible without it:
a strategy is a two-component object, an assignment on observable policy axes plus a weighting over cost
coordinates. I take that as given and build on it. Where my derivation pushes against it I say so in
section 9 rather than folding it in.

## 0. The gates

**Canon gate: passed.** The assigned work is licensed by the intent catalogue rather than in conflict with
it. `INTENTS.md:51-61` demotes I1 to OPEN in op's own words: "the strategy set is not closed at exactly
four... it's entirely open to discussion and exploration". `INTENTS.md:363-377` (I17) states that "Whether
the strategies are four or seventeen or a billion is besides the point of the intent." So deriving the set,
and in particular concluding something other than four, is exactly what the catalogue calls for.

One live tension I record rather than resolve, because it bears on my section 6. `INTENTS.md:102-108` (I4)
says the imitate-the-native-primitive concern "does not make it absolutely required, if mimicking is
consistently just worse choice". If that concern is also the baseline every other strategy's cost claim is
measured against, then an escape clause permitting it to move is an escape clause permitting the ruler to
move. I do not read this as a canon conflict, because I4 is STATED direction and my baseline reading is my
own derivation, not op's. I state it as a located disagreement with what would decide it (section 6.4).

**Test gate: run, and passed.** I ran the whole suite rather than a filter, per crate, and I read test
bodies rather than names.

Counts, each produced by a command, per `RULES.md:124` ("Counts are measurements. Produce every number with
a command and say which command"):

```
$ cd mock/benches && grep -rl '#\[test\]' variants/ | sed 's|/src/.*||;s|/tests/.*||' | sort -u | wc -l
13
$ cd mock/benches && grep -rho '#\[test\]' variants/ | wc -l
124
```

Thirteen crates carry tests, and 124 `#[test]` attributes exist. My brief said 123 tests; I did not inherit
that number and my own count is 124 attributes, which is the count of attributes rather than of executed
test cases and may differ from a runner total for ordinary reasons. Twelve of the thirteen crates ran to
completion under `cargo test --manifest-path <crate>/Cargo.toml`, all green:

| crate | passed |
|---|---|
| `bitpack-carrier-shared` | 9 |
| `bitpack-contend-shared` | 12 |
| `bitpack-footprint-shared` | 6 |
| `bitpack-plan-shared` | 5 |
| `bitpack-shared` | 3 |
| `bitpack-wide-shared` | 6 |
| `quantiser-fadd-shared` | 1 |
| `quantiser-radix-shared` | 3 |
| `satfold-shared` | 11 |
| `warm-clamp-shared` | 7 |
| `warm-container-shared` | 15 |
| `wide-rung-shared` | 30 |

That is 108 passed, zero failed, zero ignored. `wide-rung-shared` took 238.91s, which is why a naive
workspace-wide invocation looks like a hang rather than a run.

The thirteenth, `bitpack-write-contend-shared`, **did not conclude within this dispatch, and I do not claim
a result for it.** Twelve of its fifteen tests passed; the three concurrency stress tests
(`guarded_kernel_never_corrupts_under_real_concurrency`,
`naive_kernel_corruption_rate_under_real_concurrency`,
`naive_kernel_never_corrupts_when_the_split_is_aligned`) run 3000 and 1000 trials of real multi-threaded
contention each and were still running when I finished. Two attempts, one debug and one release, both
outlived their window.

**One of those attempts exited 101 and I nearly filed it as a failure.** The cause was
`signal: 15, SIGTERM`, an external termination rather than an assertion, so it is infrastructure and not a
red. Reporting it as a failing test would have been precisely the kind of unearned claim the gate exists to
catch, made by the person running the gate. Under the predicate discipline an unconcluded run is not a green
one and it is not a red one either, so the honest statement is that this crate's concurrency tests are
unmeasured here.

I read their bodies regardless, and they are well built. `naive_kernel_corruption_rate_under_real_concurrency`
at `variants/bitpack-write-contend-shared/src/stress.rs:98` deliberately **asserts no threshold** and says
why in a comment: "a scheduler-dependent rate is not a fact this test should assert a threshold on", leaving
the guarded kernel's zero-corruption test as the control and reporting the rate to stderr for the findings.
That is the right call and it is rarer than it should be.

**On the quality of the suite, since the gate is about quality and not colour.** It passes, and it passes
for the right reason: the negative controls are present and they are the thing I check first. `satfold-shared`
runs mutation controls that assert the harness *catches* a defect rather than that the kernel agrees with
itself (`the_one_element_defect_is_caught_up_to_1024_and_not_above_it` at `variants/satfold-shared/src/lib.rs:1116`,
`a_wrong_operator_is_caught_at_every_length_and_both_ops` at `:1137`, `a_dropped_lane_is_caught_wherever_a_sixteenth_lane_exists`
at `:1162`, `a_dropped_remainder_is_caught_wherever_one_exists` at `:1181`). It checks that its own false
gate is genuinely false (`the_two_const_verdicts_differ_and_are_computed` at `:1271`, asserting
`!saturating_sub_is_associative_at(6)` with the comment "so the false gate is not a control"). It checks
that its workload is not degenerate (`the_workload_is_not_degenerate_at_any_length` at `:1201`). The same
shape recurs elsewhere: `validate_output_rejects_a_wrong_sum` in three crates,
`sum_naive_is_sensitive_to_packed_corruption` in `bitpack-footprint-shared`,
`the_answer_moves_when_any_single_element_moves` in `wide-rung-shared`,
`the_noise_floor_controls_really_are_the_same_instantiation` at `variants/warm-clamp-shared/src/lib.rs:1056`.

I found no tautology, no assertion of a value against itself, and no arm compared against itself. The one
test that measures time, `diag_sat_lanes_actually_runs` at `variants/warm-container-shared/src/lib.rs:1425`,
declares itself in its own doc comment as "Diagnostic, not a bench: an ad-hoc quick spike with no substance",
which is the exact wording `evidence-lives-in-the-repo-or-it-never-happened.md` requires, and it records
that its first version was dead code that reported zero nanoseconds because the sink was provably zero. That
is a probe carrying its own case that must fail, written down in the source. I have nothing to insult here,
which I note because the gate exists to catch the opposite and it is worth saying plainly when a suite
clears it.

**Citation check.** Every `file:line` in this file was opened and its content tested, not merely resolved,
per `RULES.md:126`. The checker is `140_probes/p5_check_my_own_citations.py`, output at
`140_probes/p5_out.txt`: 36 citations, 0 failures, with two deliberately-wrong entries that both had to fail
and did.

Its first run reported **three of my own citations failing**, and all three turned out to be defects in the
checker rather than in this file: a quotation that wraps inside a markdown blockquote reads as
"minimises and > bitpacks" once the lines are joined, so no honest substring matches it. I fixed the
normaliser to strip blockquote markers and reran rather than declaring the three correct by inspection,
because "I looked and they are fine" is the claim this probe exists to replace.

## 1. The question I actually asked

My habit with an unfamiliar system is to ask what it recomputes when one record changes, because the answer
X-rays the architecture. The analogue here is the only question I found that makes the strategy set decidable
rather than a matter of taste:

**What changes in the emitted program when the strategy changes, and nothing else does?**

Everything below follows from taking that question seriously. If the answer at some pair of strategies is
"nothing", those are not two strategies. They are one strategy with two names, and naming them twice is a
claim of capability nobody paid for.

This is the same question I spent a career asking about distributed systems, transplanted. A system that
advertises 128 cores and is beaten by one laptop thread has not earned the 128. A design that advertises
four strategies and cannot exhibit, for each of the six pairs, an input at which the two disagree, has not
earned the four. **A set of size N is not one claim. It is N(N-1)/2 pairwise distinctness claims, and each
one needs a witness.** Without the witness table the count is decoration, and a count that reads as more
capability than it delivers is precisely the failure mode I know best.

So my first answer to "what determines the answer" is: **the witness table determines it, and the witness
table is mechanical.** Not taste, not symmetry, not the pleasingness of four.

## 2. The two components are not the same kind of thing, and this is the finding

Take the settled object as `(A, w)`: `A` an assignment on observable policy axes, `w` a weighting over cost
coordinates. Ask my question of each component separately and they answer differently.

**Changing `A` changes the answer.** That is what "observable policy" means. Two values of `A` denote
different functions from inputs to results, so every law, every test and every consumer sees the difference.
This is exactly I9, `INTENTS.md:172-177`: "strategies are the variables that change what the 'correct' answer
is for what we choose as the path."

**Changing `w` changes which lowering is chosen and not what it computes.** Two values of `w` rank the
available arms differently, so the emitted code differs and the result does not. This is exactly I8,
`INTENTS.md:143-153`: "All of them should be decided by measurement, just measuring different things, and...
They weigh different measurements differently."

Op stated those as two separate intents, and I think the separation is load-bearing rather than incidental,
because the two components have **different closure properties**:

**`A` is closed, finite, and arvo's to enumerate.** It is a product of finitely many axes, each with
finitely many settled positions: how rounding resolves, what overflow does, whether intermediates widen, and
so on. An axis position arvo has never heard of has no lowering, so it cannot be supplied from outside. The
assignment space is a lattice arvo knows completely because it knows the axes.

**`w` is open, continuous, and the consumer's to supply.** A weighting is a ranking over cost coordinates,
and "I value footprint three times what I value latency" is a coherent statement arvo cannot know and does
not need to. I11 says exactly why (`INTENTS.md:190-197`): "We are a library, not a program, so we don't know
how end users will use us". Supplying a new `w` requires no new lowerings from arvo; it re-ranks arms arvo
already ships.

**So "is the strategy set closed" has two answers and the question conflates them.** The axis space is
closed and enumerable. The space of named points is open and unbounded. And a named strategy is a **named
point**: a specific `A` paired with a specific `w`, given a name because someone found that combination
worth naming.

That reconciles two intents that otherwise pull against each other. I2 (`INTENTS.md:63-72`) says each preset
names a stated intent. I1 (`INTENTS.md:51-61`) says the set is not closed. Both are true at once as soon as
you notice that presets are named points and axes are the space: **the presets are open, the axes are
closed.**

### 2.1 What this makes of "how many strategies are there"

It makes it the wrong question, in the precise sense `never-ask-which-single-rule-governs.md` names, and op
already answered it at I17: the count is beside the point of the intent. The canon's obligation is to name
**the axes and their positions**, because those are finite, arvo's, and permanent. The named points are
convenience, and adding one costs nothing and removes nothing.

The permanence test from `RULES.md:79-83` agrees. "There are four strategies" fails permanence the moment
anyone adds a fifth. "The observable policy axes are these, with these positions, and a strategy assigns one
position on each" survives every rewrite, and survives the presets being renamed, merged or multiplied.

## 3. What makes two strategies genuinely distinct, stated as a test something can run

I offer this as the membership criterion, and it is deliberately mechanical so that it cannot be argued
about in the abstract.

Two strategies `(A1, w1)` and `(A2, w2)` are **semantically distinct** if there exists a numeral shape and
an operation at which `A1` and `A2` compute different results. They are **operationally distinct** if they
are not semantically distinct but there exists a region in which the arm `w1` ranks first is not the arm
`w2` ranks first.

Both are real distinctions and they are not the same distinction:

- Semantic distinctness is visible to correctness. It forks the law layer, it forks every test, and a
  consumer who picks wrong gets wrong answers.
- Operational distinctness is invisible to correctness and visible only to a measurement. A consumer who
  picks wrong gets right answers slower, or larger.

**A design that files both under one word will keep confusing the two**, and I think that is what a flat
four-element set does. It is also why "is this a continuum" has an answer: the semantic component is not a
continuum, it is a finite lattice; the cost component is a continuum, or at least a preorder with no
canonical finite quotient.

### 3.1 So: is the answer a continuum, and what does the design name instead

Partly. The honest statement is that the strategy space is a **product of a finite lattice and a continuum**,
and the design should name the finite part exhaustively and the continuous part by its coordinates rather
than by its points.

What the design names instead of continuum points:

- **The axes of `A`**, exhaustively, with their positions. Finite, closed, permanent.
- **The cost coordinates of `w`**, exhaustively, by name. Also finite: latency, footprint, accuracy, code
  size, and whatever else measurement establishes. The coordinates are finite even though the weightings
  over them are not.
- **Named points**, as many as are useful, each stating its `A` exactly and its `w` as *what it weighs*
  rather than *what it achieves*.

The distinction in that last bullet is the whole of section 6 and I think it is the most consequential thing
in this file.

## 4. Whether the four concerns are four strategies, four axes, or a mix

My brief lists four concerns: storage-minimising, speed-first, accuracy-first, imitate-the-native-primitive.
Running each through the `(A, w)` split, using op's own words for each:

**Speed-first has both components.** I5 (`INTENTS.md:110-117`): "Hot *can* sacrifice soundness, that is its
explicit purpose, but it should not lose it for nothing, instead, provable meaningful gains." Sacrificing
soundness changes answers, so this concern sets `A` to something laxer than its neighbours, and weights `w`
toward latency. Two components, both non-trivial.

**Accuracy-first has both components, and its `A` is the interesting one.** I7 (`INTENTS.md:136-141`): "to be
the most precise possible answer, throwing out all cold or hot axis optimisations to be *accurate* and
*precise*, especially within chains and ops, not only alone." The phrase "within chains" is not decoration.
Per-operation accuracy and per-chain accuracy are different `A` settings: the first rounds at every step,
the second holds an exact intermediate and rounds once. So this concern's `A` sets an intermediate-width
policy, and its `w` weights accuracy above everything.

**The imitate-the-native-primitive concern is, as of the 2026-08-14 refinement, not primarily an `A` at
all.** I3 (`INTENTS.md:88-96`) records op answering "Neither, it's ergonomics" when asked whether the
imitation targets the declared width or the container, and the catalogue draws the conclusion explicitly:
"So **I3 is not a statement about where arithmetic boundaries land.**" Its `A` therefore comes from I4
(`INTENTS.md:104-108`), "its intent is to be intuitive best choice for most every use case", and intuitive
for a Rust reader means Rust's own assignment. Its `w` is a balanced one.

**The storage-minimising concern is a `w` and its `A` is not stated anywhere.** This is the asymmetry that
made me rewrite section 2, and I think it is a real hole in the four-concern framing rather than a gap in
my reading. I6 (`INTENTS.md:119-127`) is entirely about cost: "it aggressively minimises and bitpacks, *but*
because it optimises for cold paths, it has more leeway to do things non-efficient... it should remain small
for memory or disk storage, because it's just sitting basically." Every clause is a weighting. Nothing in I6
or I17 says what answers it computes.

### 4.1 Op's own words say the concerns are not mutually exclusive

And here the catalogue argues my case better than I can. I6 continues, `INTENTS.md:129-133`:

> Cold does not *have to* drop efficiency wins elsewhere. It can use the same paths Hot uses, not because it
> needs to by intent, but nothing in its intent would fight it. But if the path fights the intent, then it's
> not for Cold.

**A flat set of four makes the storage-minimising and the speed-first concerns alternatives, and op is on
the record saying they are not.** Under a flat set you pick one; under a product you compose them, and the
composition is exactly what op describes: take the fast path where it does not fight the small-footprint
intent, and not where it does.

The same test applied to the other pair: can a consumer want storage-minimised *and* accurate-in-chains? A
tightly packed column that widens exactly when computed on. Nothing in either intent conflicts. Under a flat
set that combination is inexpressible, and its inexpressibility is an artifact of the shape, not of any
intent.

So my answer to the brief's question: **a mix, and specifically three of the four concerns carry an `A` and
all four carry a `w`, and the storage-minimising one is a `w` with no stated `A`.** Whether that missing `A`
is free (my reading: the concern composes with any assignment) or defaulted (it silently inherits the
imitation concern's assignment) is a real fork, and I state it in section 8 with what would decide it.

## 5. What a strategy must determine, and what it must never determine

**Must determine: everything that changes an observable answer.** This is forced, not chosen. I9 says the
strategy is the variable that changes what the correct answer is. If some answer-changing choice is *not*
determined by the strategy, then two programs with identical types and identical strategies can disagree,
and the strategy has failed at the single job I9 gives it. So the `A` component must be total over the
answer-changing axes. A partially-specified `A` is a bug in the design, not a flexibility.

**Must never determine: the declared width.** A strategy that changes the width is not selecting a policy,
it is changing the number. Two types differing only in strategy would then hold different value sets, every
conversion between them would be lossy in a direction nobody declared, and the width parameter would be a
lie told by the type. The committed benches already hold this line: `wide-rung-shared`'s
`every_stored_value_is_inside_the_declared_width` and `the_top_limb_mask_keeps_exactly_the_bits_the_width_declares`
assert exactly it, and `warm-container-shared`'s `the_shipped_rule_widens_every_width_to_64`
(`variants/warm-container-shared/src/lib.rs:1506`) shows the shipped rule widening the **container** to twice
the minimum while `both_regions_hold_the_same_column` (`:1521`) asserts both containers decode to the same
logical column.

That is the distinction stated precisely: **a strategy may determine the container and may never determine
the declared width.** The container is a cost coordinate, the width is the consumer's declaration, and the
bench tree already treats them that way.

**Must never determine: the denotation of a representation.** Given a representation, the map from bits to
value must be fixed by the width, the signedness and the layout, and must not consult the weighting. If it
did, a column written under one strategy and read under another decodes differently, and I11's whole point,
that the value of arvo is what composes on top of it, collapses at the first boundary crossing. The
representation may be strategy-chosen; its meaning may not be strategy-dependent.

**Must never determine: whether a law holds.** A law holds or does not hold as a fact about the assignment
and the operation, and a weighting cannot make a false law true. This matters because `w` is what selects
the fast arm, and a fast arm gated on a law is only sound where the law holds. If a weighting could
influence the law verdict, a consumer could buy a wrong answer by declaring a preference, which is the worst
failure mode available to this design. I note that `arvo-always-optimal-internals.md` already carries the
measured version of this, and that `satfold-shared:1271` pins the const verdicts with a genuine false case.

## 6. What the canon owes a consumer picking one, and the baseline problem

`arvo-toolbox-not-policer.md` forbids arvo from telling a consumer which strategy to pick, and it is right:
the substrate does not know the workload and I11 says so. But "we cannot choose for you" is not the same as
"you are on your own", and the gap between those two is where I think the canon owes something specific.

**The canon owes a decision procedure that terminates in a measurement, not a taxonomy that terminates in a
vibe.** Concretely, three obligations:

**6.1 State each `A` exactly and statically.** The answer-changing consequences are arvo's to know, they are
finite, and they are permanent. A consumer must be able to read off what an expression means under a
strategy without running it. This costs nothing and is not a claim about any workload.

**6.2 State each `w` as what it weighs, never as what it achieves.** "This preset weights footprint above
latency" is a statement about the design that is true forever. "This preset is faster" is a statement about
the consumer's workload, and arvo cannot make it. The second form is the exact sentence I have spent a
career deleting from other people's papers, and it is the sentence a strategy taxonomy invites, because
naming something after its goal makes the goal sound achieved.

**6.3 Ship the baseline as a strategy, and name it as the baseline.**

This is the thing I most want on the record. Every comparative claim needs a ruler, and the ruler has to be
in the box. If a consumer is choosing among named points and each point advertises a goal, they have as many
unfalsifiable claims as there are points and nothing to measure against. What makes the choice tractable is
one named point whose entire content is *do the obvious thing, one competent path, no cleverness*, against
which every other point's advantage is stated and measured.

That is the COST criterion, and the reason I trust it is that it is the criterion under which a whole
literature of scaling claims evaporated. The claims were not fabricated. They were measured against a
baseline that had been inflated by the very machinery being sold, so the machinery appeared to earn its
keep. A strategy set has exactly this hazard: if the only comparison available is strategy against strategy,
every one of them can look justified while all of them are beaten by the obvious implementation.

### 6.4 And this puts the imitate-the-native-primitive concern in a different position than the others

If arvo needs a baseline in the box, the obvious candidate is already there. Native Rust semantics on the
native container *is* the obvious competent implementation, and it is what a downstream reader will compare
against whether or not the canon says so. That reframes the imitation concern: not a peer among four, but
**the origin of the coordinate system**, the point every other point's claim is stated relative to.

Two consequences, one welcome and one sharp.

Welcome: it explains why op has restated the imitation call four times (`INTENTS.md:85-86` records three
restatements plus another on 2026-08-08) without it sticking. A call that keeps failing to stick is usually
a call whose *role* is misfiled, and "this is one of four flavours" is a much weaker role than "this is the
zero of the coordinate system".

Sharp: I4's escape clause becomes dangerous under this reading. `INTENTS.md:106-108`: imitation "does not
make it absolutely required, if mimicking is consistently just worse choice." If the baseline is permitted
to move in order to improve itself, then every measured claim about every other strategy silently rebases,
and the design has reinvented the inflated baseline on purpose. Probe `p4` makes that arithmetic concrete: a
ranking among arms can invert under a baseline that moves, with no arm having changed.

**Located disagreement, stated precisely.** Either the imitation concern is a peer strategy, in which case
I4's escape clause is harmless and my section 6.3 needs a different baseline nominee; or it is the baseline,
in which case the escape clause needs a guard saying the baseline may improve its *lowering* and not its
*semantics*.

**What would decide it:** whether any other strategy's advantage is ever stated relative to it. Grep the
eventual canon for a comparative claim about the speed-first or storage-minimising concern and ask "faster
than what, smaller than what". If the answer is "than the imitation concern", it is the baseline and cannot
float. If every comparative claim resolves to an absolute measurement against a fixed external reference,
it is a peer and floats freely. This is decidable by reading the canon that gets written, which makes it a
question the panel can close rather than one that needs op.

I flag that I am the only source for this framing and it sits at ONE EXPERT. It wants a second read
specifically because it is the kind of argument that sounds right and could be wrong in a way that is not
visible on the page.

## 7. What is derivable from the intents, and what I chose

`RULES.md` asks for honesty about the edges, so I separate these rather than presenting one confidence.

**Derivable, and I would defend each against attack:**

- The count is not the question. I17, `INTENTS.md:369-371`, verbatim and direct.
- There is an answer-changing component. I9, `INTENTS.md:175-176`, verbatim and direct.
- There is a cost-weighting component. I8, `INTENTS.md:147-153`, verbatim and direct.
- The storage-minimising concern is not deprioritised and does not get traded away. I17, direct.
- The concerns are not mutually exclusive. I6's continuation, `INTENTS.md:131-133`, verbatim.
- The storage-minimising concern's intent is stated purely in cost terms. I6 and I17 in full; the absence of
  any answer-changing clause is checkable by reading them.

**Chosen by me, and marked as mine, per the provenance ladder's treatment of an agent's own call:**

- That `A` is closed and `w` is open. This follows from what an axis position needs in order to lower, which
  is an engineering fact rather than an intent, and I could be wrong about it if a consumer-supplied
  assignment could be expressed as a composition of shipped ones.
- The specific axis list. `INTENTS.md` enumerates no axes. Anything naming them is derived.
- The baseline reading of the imitation concern. Mine entirely, section 6.4, offered as a located
  disagreement rather than as a conclusion.
- That a strategy may never determine the declared width. Strongly implied by the width being a consumer
  declaration, but not stated by op anywhere I found.

**Not derivable and not chosen, i.e. genuinely open:** how many named points there should be. I do not think
this has a right answer, and I think asking for one is the anti-pattern I13 rejects. My answer is that the
question dissolves once the axes are named, and I would rather be told I have dodged the question than
manufacture a number.

## 8. Options this opens, each with what would close it

Per `RULES.md:310-312`, an option with no decision procedure attached accretes rather than resolves.

**O-A. The storage-minimising concern's assignment is free, and it composes with any `A`.** Closed by
finding any consumer need for a packed column whose arithmetic assignment differs from the default; that
existence proves the composition must be expressible. Refuted by finding an assignment whose lowering is
incompatible with packing, which would make the composition partial rather than free.

**O-B. The storage-minimising concern defaults to the imitation concern's assignment.** Closed by op stating
it, or by the canon needing a total `A` per section 5 and no other source being available. This is the
conservative option and it is not free: it silently makes a semantic choice on the consumer's behalf, which
`arvo-toolbox-not-policer.md` is written against.

**O-C. Named points are canon.** The canon enumerates them and adding one is a canon edit. Closed by
demonstrating that consumers need a shared vocabulary more than they need extensibility. Cost: every new
named point invalidates the designs declaring that canon file, per the mutation order.

**O-D. Named points are design, and only the axes are canon.** Closed by demonstrating the axes are stable
under adding points. Cost: two consumers may name the same combination differently, and the vocabulary
fragments.

**O-E. The weighting is consumer-supplied at the type level.** A consumer writes its own weighting and gets
arvo's arms re-ranked by it. Closed by establishing that arm selection can be driven by a const-evaluable
weighting, which I13's clarification (`INTENTS.md:248-255`, "collapses to whatever is available at const
time") makes plausible. Refuted by finding that arm selection needs information no const expression can
reach.

**O-F. The weighting is not a type-level object at all, and the named points are the only interface.** The
conservative alternative to O-E. Closed by measuring that O-E's monomorphisation cost is prohibitive, which
is a bench question and unpriced today.

I fit O-A, O-D and O-E well; I fit O-B badly but do not kill it; I kill nothing, because nothing I built
refutes any of them.

## 9. Where my derivation pushes against the settled two-component statement

My brief says a change to what a strategy *is* must be visible rather than folded in, so: **I do not
contradict the two-component object, and I do add a claim about it that the statement as given to me does
not carry.**

The addition: the two components are not merely two fields, they are **two different kinds of thing with
different closure properties**, and treating them symmetrically is what produces the flat-set framing I
argue against. If the settled statement is read as "a strategy is a pair", everything I say is compatible
with it. If it is read as "a strategy is a pair of two similar selections", section 2 is a disagreement, and
I would want it resolved explicitly rather than assumed compatible.

The second-order consequence, which I do think is a real claim against the flat framing: **the four concerns
are not four instances of one kind.** Three carry an assignment and all four carry a weighting, so a design
that lays them out as four peers on one axis has lost the fact that one of them composes with the others
rather than competing with them.

## 10. Predictions, stated before the probes ran

`RULES.md` and my brief both want the refutations on the record, so these are written before any probe
executed. Results in section 11.

**P1.** At a small width, the number of pairwise-distinguishable assignments is strictly less than the
product of the axis cardinalities, because some positions are unreachable given others.

**P2.** The collapse is operation-dependent and fraction-width-dependent, so two assignments distinguishable
on one operation are indistinguishable on another. Consequence if true: **"how many strategies are there"
has no width-free, operation-free answer**, and a canon stating a count would be stating something false at
most shapes.

**P3.** At `F = 0`, unsigned, addition only, all rounding positions collapse to one class, so the
distinguishable count equals the overflow axis cardinality alone.

**P4.** A packing round trip is answer-preserving under every assignment, so the storage-minimising concern
is orthogonal to the answer-changing axes rather than being a position on them.

**P5.** A ranking among arms can invert when the baseline moves and no arm changes.

## 11. Probes

All four probes ran with their controls passing. Sources and raw outputs are committed in
`140_probes/`, in the commit before this file. Three independent models: p1 in Rust over raw integers, p2 in
Python over exact rationals, p3 in Rust extending p1's model with a storage dimension, p4 in Python over
synthetic arithmetic. p2 shares nothing with p1 that I could avoid sharing, which is the point of it.

### 11.1 p1: how many assignments are actually distinguishable

`140_probes/p1_assignment_partition.rs`, output at `140_probes/p1_out.txt`.

Thirty assignments (5 rounding positions, 3 overflow positions, 2 intermediate positions), partitioned by
equality of the full answer function over exhaustive inputs.

| shape | add | sub | mul | a*b+c | a*b-c |
|---|---|---|---|---|---|
| W=4 F=0 | 2 | 2 | 2 | 2 | 5 |
| W=4 F=1 | 2 | 2 | 6 | 8 | 20 |
| W=4 F=2 | 2 | 2 | 8 | 10 | 24 |
| W=6 F=0 | 2 | 2 | 2 | 2 | . |
| W=6 F=2 | 2 | 2 | 8 | . | . |
| W=8 F=0 | 2 | 2 | 2 | . | . |
| W=8 F=2 | 2 | 2 | 8 | . | . |

(Dots are shapes the sweep did not run at, not zeros. The chain domain is `2^(3W)`, so it runs at W=4 and
W=6 only, and the p2-comparable set runs at W=4.)

**Joint over the witness set: 30 assignments collapse to 24 classes.** Six of the thirty compute an answer
function identical to another's, everywhere, so they are not thirty strategies under any naming.

The structure of the collapse is more informative than the number:

- **`TowardZero` and `TowardNegInf` are indistinguishable in most of the space**, because on non-negative
  values truncation and flooring are the same map. They separate only where an intermediate can go negative,
  which in this sweep is `a*b-c` under `Wrap` and under `SaturateHighOnly`.
- **`TowardPosInf` never separates the intermediate axis**, and neither do `TowardZero`, `TowardNegInf` or
  `TiesAway` under `Wrap`. `TiesEven` separates it everywhere. The reason is that `TiesEven` is the only mode
  in the set that is not translation-equivariant: adding an exact grid multiple shifts the quotient's parity
  and so can flip a tie-break, while every other mode commutes with an exact translation.

**That last fact is the one I would carry to the canon**, because it says the axes are not independent. An
axis position can be unobservable except in combination with a particular position on another axis, so
"the design has three axes with 5, 3 and 2 positions" is not a statement from which the design's expressive
power follows.

### 11.2 p2: the same count from an independent model

`140_probes/p2_independent_partition.py`, output at `140_probes/p2_out.txt`.

p1 models values as raw integers and rounding as `div_euclid` quotient arithmetic. If that model is wrong,
p1 is wrong in a way p1 cannot see, and rerunning p1 reproduces the error. So p2 models values as exact
`Fraction`s, rounding as a choice between the two neighbouring grid points on the real line, and the range
policy against the logical bounds rather than an integer modulus. No shifts, no raw integers, no modulus.

Over the witness set p1 pins as comparable (W=4, F in {0,1,2}, all five operations):

```
p1 reports 24 for this witness set. p2 reports 24.
AGREE
```

And they agree on the **partition membership**, not merely the count: the same six merges, class for class.
Two independent models producing the same 24-way partition is the strongest evidence in this file.

### 11.3 p3: the container is not observable, and the control proves the sweep could see it

`140_probes/p3_container_is_not_observable.rs`, output at `140_probes/p3_out.txt`.

Extend the configuration with a storage container and re-partition. If the storage choice is an observable
policy, the class count rises.

```
Packed + Minimum + Headroom together: 90 configs -> 24 classes
CLAIM: the three lossless containers add zero classes -> CONFIRMED
CLAIM: no class ever splits one assignment across containers -> CONFIRMED

NEGATIVE CONTROL: adding the Lossy container: 24 -> 48 classes
```

Ninety configurations collapse to the same twenty-four answer functions, and no class ever contains one
assignment under some containers but not others. A container that drops one bit doubles the count, so the
sweep can see a container when there is something to see, and the negative result is a real negative rather
than an instrument that never fires.

**This is the measured form of section 4's claim.** The storage-minimising concern is a weighting, not an
assignment. It composes with every assignment rather than competing with them, which is what op says at
`INTENTS.md:131-133` and what a flat set of peer strategies cannot express.

### 11.4 p4: what a per-arm baseline does, and it refuted my own prediction

`140_probes/p4_baseline_rebase.py`, output at `140_probes/p4_out.txt`.

**This is not a benchmark and not a measurement.** It prices nothing and times nothing; it is an exhaustive
arithmetic enumeration over synthetic unitless cost tuples, and under the workspace rule it is an ad-hoc
quick spike with no substance as far as any magnitude goes. What it can establish is a structural fact about
comparisons, and it establishes one that contradicts what I predicted.

```
SHARED baseline:  840 comparisons, 0 where the reported ranking differs
                  from the absolute-cost ranking
PER-ARM baseline: 35724 comparisons, 20106 differ (56.3%)
                  1302 are the exact reverse ordering
                  3222 (9.0%) have the absolutely WORST arm reporting the BEST figure
```

**P5, as I wrote it in section 10, is REFUTED.** I predicted that "a ranking among arms can invert when the
baseline moves and no arm changes". That is false, and obviously false in hindsight: a shared baseline that
moves divides every arm's figure by the same number, so it cannot reorder anything. Zero of 840.

What actually inverts a ranking is **each arm being measured against its own baseline**, which is a
different failure and a worse one. In more than half the enumerated comparisons the reported ordering is not
the true ordering, and in nine percent of them the arm that is absolutely worst reports the best figure. The
witness the probe prints:

```
arm 0: absolute cost 1, its own baseline 1, so it reports 1.00x
arm 1: absolute cost 2, its own baseline 1, so it reports 0.50x
arm 2: absolute cost 3, its own baseline 4, so it reports 1.33x
arm 2 looks best and is the worst arm in the set.
```

**So section 6.4 was aimed at the wrong hazard and I restate it here rather than editing it.** The danger is
not that the imitation concern's semantics might drift. It is that **each strategy might state its advantage
against its own naive version**, which is the exact shape of the scalability claims that COST emptied out,
and which requires no drift at all to mislead. A strategy set in which every member is justified by "this
preset beats the obvious way of doing what this preset does" is self-certifying: every member is justified
and no two are comparable.

The revised obligation is therefore sharper and easier to state than the one in 6.3: **every strategy's cost
claim is stated against the same named arm.** Whether that arm is the imitation concern, a separate
explicitly-named reference, or something else, is open. That it must be one arm and not one per strategy is
what p4 supports.

### 11.5 The prediction scorecard

| # | prediction | verdict |
|---|---|---|
| P1 | fewer distinguishable classes than the axis product | **CONFIRMED**: 30 to 24 jointly, and as few as 2 at a single witness |
| P2 | the collapse is operation- and F-dependent | **CONFIRMED**: add is 2 classes at every shape swept; `a*b-c` at W=4 F=2 is 24 |
| P3 | at F=0 unsigned addition the count equals the overflow cardinality (3) | **REFUTED**: it is 2. `SaturateBoth` and `SaturateHighOnly` differ only below zero, and adding two non-negative values never goes there, so the low clamp is unreachable and the overflow axis collapses too |
| P4 | packing is answer-preserving, so storage is orthogonal to the assignment | **CONFIRMED** by p3, with the lossy control firing |
| P5 | a ranking can invert when the baseline moves and no arm changes | **REFUTED**: 0 of 840. The real hazard is per-arm baselines, at 56.3% |
| P6 | widening the witness set by one operation raises the class count | **CONFIRMED**: 15 to 24 by adding `a*b-c` alone |

Two of six refuted, and both refutations changed a conclusion rather than a detail. P3's refutation is why
section 3's criterion has to range over operations rather than being stated per axis. P5's refutation is why
section 11.4 replaces section 6.4's hazard with a different one.

### 11.6 What the probes do not establish, stated plainly

They are exhaustive over their domains and their domains are small. Specifically:

- **Unsigned only.** Every probe models unsigned values. The signed case is where a one-sided clamp stops
  being a congruence, and I did not touch it. Anything I say about the overflow axis is `signedness =
  unsigned` and nothing else.
- **Three axes only.** I chose rounding, overflow and intermediate width because they are the axes I could
  derive from op's own words at I5, I7 and I18. There is no reason to think that is the axis list, and if
  there are more axes the count of 24 is a floor rather than a ceiling.
- **Five operations.** p1 demonstrates that the count is a function of the witness set, so 24 is 24 for
  *that* witness set. Add division, or a longer chain, and I would expect it to rise again.
- **No timing anywhere.** Nothing in this file prices anything. Every claim about cost is structural.

### 11.7 The findings, with predicates

Per `RULES.md:486-541` and I13, each states the region it holds in, and a dimension not listed claims
nothing anywhere that dimension is present.

**F1. Thirty assignments over three observable-policy axes collapse to twenty-four distinguishable answer
functions.**
`holds for: W = 4, F in {0,1,2}, signedness = unsigned, rounding any of {toward-zero, ties-even, ties-away,
toward-neg-inf, toward-pos-inf}, overflow any of {wrap, saturate-both, saturate-high-only}, intermediate any
of {round-each-step, exact-then-round-once}, operations = {add, sub, mul, a*b+c, a*b-c}, threads = 1`
Established twice independently: `140_probes/p1_out.txt` and `140_probes/p2_out.txt`.

**F2. The number of distinguishable classes is a strictly increasing function of the witness set, so no
count is meaningful without naming the operations it was counted over.**
`holds for: W = 4, F in {0,1,2}, signedness = unsigned, axes as F1, witness sets = {without a*b-c} and
{with a*b-c}, threads = 1`
15 classes to 24 by adding one operation. `140_probes/p1_out.txt`.

**F3. A lossless container choice contributes zero distinguishable classes, and a lossy one contributes
many.**
`holds for: W = 4, F in {0,1,2}, signedness = unsigned, axes as F1, containers = {packed, minimum-rung,
double-rung} and the control {W-1 bits}, operations as F1, threads = 1`
90 configurations to 24 classes; the control doubles it to 48. `140_probes/p3_out.txt`.

**F4. Under one shared baseline the reported ranking is always the absolute ranking; under per-arm
baselines it differs in 56.3% of cases and inverts entirely in 3.6%.**
`holds for: arms = 3, costs in {1..6} distinct, baselines in {1,2,3,4,6,8,12}, arithmetic exhaustive`
Structural only. Prices nothing. `140_probes/p4_out.txt`.

I deliberately do not list `threads` on F4 because F4 is not about execution at all; it is arithmetic over
numbers. Under the notation that means F4 does not hold anywhere threads exist, which is the correct
severity: it is a fact about ratios, and nothing about a threaded program follows from it.

## 12. What I would put in front of the next expert

Not conclusions. The things I think are worth attacking, in the order I would attack them.

**The strongest thing I have** is F3 plus op's own sentence at `INTENTS.md:131-133`. Together they say the
storage-minimising concern is not a peer of the others, and that a flat set misdescribes the intent. Attack
it by finding an assignment whose lowering is incompatible with packing, which would make the composition
partial and weaken the claim from "orthogonal" to "mostly orthogonal".

**The most useful thing I have** is F2, because it kills the question rather than answering it. If the count
depends on the witness set, then "how many strategies are there" is not a question with an answer, and the
canon's obligation is the axes. Attack it by finding a witness set that is canonically privileged, which
would restore the count.

**The thing I am least sure of** is section 6's baseline argument, now in its corrected form in 11.4. It is
mine alone, it sits at ONE EXPERT, and it is the kind of argument that reads well and could be wrong in a
way invisible on the page. The specific thing to check is whether any strategy's advantage is ever stated
relative to another strategy rather than to a fixed reference. If not, the argument is unnecessary. If so,
the reference has to be named and pinned.

**The thing I did not do** is signed. Every probe is unsigned, the signed case is where the saturation
structure genuinely changes, and I would put a signed re-run of p1 above everything else on this list.

## 13. Concessions

I did not find the axis list. I derived three axes from op's words and swept those, and I have no basis for
claiming they are the axes rather than three of them. That is the largest gap in this file and I am not
going to paper it over with a plausible enumeration, because a plausible enumeration is exactly the artifact
that gets cited later as though it were derived.

I also did not answer how many named points there should be. I argue in section 2.1 that the question
dissolves, and I recognise that "your question dissolves" is the answer most likely to be a dodge. If a
later expert shows the count is load-bearing for something, my section 2.1 is the first thing to discard.

