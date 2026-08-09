# Panel 04: does it earn its keep

**Persona:** Linus Torvalds, engineering-economics lens. Fourth member and the first non-formal one;
read `01_knuth_mathematical_rigour.md`, `02_kiselyov_type_level_encoding.md` and
`03_jhala_what_is_provable.md` in full before starting, plus every probe under `02_probes/`.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), panel files 01
through 03, the fourteen probes, the panel brief, and the governing panel rule. **What I read in
part:** the talk and the inherited-state file at the passages the spec and the prior members cite
(D31, D40, D48, D52 read in place at `202607301000_topic...md:1296,1547,1894,2110`); source at
`arvo/src/{float,ufixed,markers}.rs`, `arvo-strategy/src/{lib,axes,identity,container,arith_impls}.rs`,
`arvo-graph/src/lib.rs`, the whole of `arvo/tests/` including `tests/ui/`, and the downstream
consumers at `hilavitkutin/mock/crates/` (grep-driven, files read where hits landed).
**Directory listing done** across `mock/design_rounds/`, `mock/research/`, `mock/research/sketches/`
and the panel directory; nothing supersedes the spec, and the three flat files at `design_rounds/`
root are this round.

**Gates.** I re-ran the full suite myself rather than inheriting either prior count: 654 passed, 0
failed, 122 binaries, matching 01 and 02. I read test bodies on the touched surface in my own hand:
`strategy_semantics.rs` asserts concrete raw values per strategy (wrap to 44, saturate at logical
MAX), `tests/ui/no_multiplicative_identity.stderr` pins the real contract error with a full
remediation note, and `identity_laws.rs` is the whole-matrix suite 01 describes. No tautological
tests found; the suite on this surface is honest. I confirmed 02's drift finding still stands today:
`arvo/src/lib.rs:25` and `arvo-strategy/src/lib.rs:11` still carry `#![feature(generic_const_exprs)]`
against the workspace's forbidden ruling. And I checked the one brief premise the formal lenses had
no reason to test: the spec's claim that "pre-1.0 nobody depends on the current meanings" (spec:246).
Grep across the two named consumers: hilavitkutin has 20 `UFixed`/`IFixed` mentions across seven
files, vehje has zero. The claim is approximately true and now verified rather than inherited, which
matters because it is exactly the shape of claim the 2026-07-28 panel inherited falsely.

**Separation of evidence.** Sections marked *verified* rest on something I compiled, timed, ran or
read at a `file:line`. Sections marked *reasoned* are argument. Impressions are labelled as such.

---

## 0. My lens, and where I stand relative to the first three

01 established that some of the mathematics is false. 02 established that the encoding does not
enforce what it claims to enforce. 03 established that even fixed and enforced, the leaf facts are
trusted, not checked. All three conclude the three-contract decomposition itself survives. I agree
with that conclusion and will not re-litigate any of their findings; where I cite one it is to attach
a cost or a consumer to it.

My question is different: this is a numeric substrate that two real repos link against, that one
person maintains, and that has to keep compiling and keep making sense at call sites for years. The
spec proposes one type over ten axes where four types stand today, redefines behaviour under
unchanged spelling, and derives its properties through blanket machinery three panellists have
already had to correct. Some of that buys something a user can point at. Some of it is machinery
admiring itself. Sorting which is which is this file.

Up front, because the panel rule says to say what is good plainly: the identity/policy/lowering cut
is good taste in the exact sense I mean by the phrase. D54's sorting test ("change the axis, ask what
changed") is a test a future contributor can run without any context, which is what makes a
decomposition survive its author. The quantisation unification deletes a special case (saturation
stops being a named thing and becomes a pair of directions, spec:159-162), which is the move that
distinguishes design from cataloguing. Those two survived 01's review and they survive mine. Most of
what follows is about what was built on top of them.

## 1. The float half of the unification has no arithmetic behind it. Verified.

The spec's headline claim is that fixed point and floating point differ only in where the exponent
lives, and it spends two identity axes on the float side: `Stored<const BITS: Width, U: Underflow>`
(spec:95) with `Underflow`'s three Flocq instances (spec:98-103).

Read what floats in arvo actually are. `arvo/src/float.rs:31-42`: `FastFloat<F>` and `StrictFloat<F>`
are `repr(transparent)` wrappers over `F: Ieee`, and `Ieee` is sealed to `f32` and `f64`
(float.rs:12-13). Every arithmetic operation on them is the hardware's IEEE arithmetic, with LLVM
fast-math flags as the only lever. arvo does not implement floating-point arithmetic and, per
`arvo-always-optimal-internals.md`, should not: soft-float in a performance substrate would be
absurd.

The consequence for the axes: `Underflow` has three instances of which the hardware gives exactly
one per flag setting, and `Stored<5, Gradual>` describes a format whose arithmetic arvo cannot
execute. 01's finding 8 already showed `Stored` cannot express the formats people actually ship
(hidden bit, specials, exponent encoding). Put the two together and the honest statement is sharper
than either alone: the fully general `Stored<BITS, U>` is simultaneously **inexpressive of real IEEE
formats** and **operationally uninhabited beyond the two hardware ones**. The one bit of it that does
work is the distinction itself: exponent constant versus exponent per value, which 01's finding 2
needs anyway as the numeral-side condition on the law derivation (uniform spacing makes same-format
addition exact; varying spacing makes it round).

Three readings, and I hold them at different strengths.

First reading, the one I lean to: ship `ExponentForm` as the two-point operational axis it is
(`Implicit<E>`, and a `Hardware`-shaped marker for the two IEEE types), let the law derivation key on
it, and park the parameterised `Stored<BITS, U>` in the BACKLOG as an unbackticked promissory note
until a consumer names it. That is not caution about scope; it is that generality with no caller and
no executable semantics is the wrong kind of code to carry, because it cannot be tested (nothing
computes with it) and untestable machinery rots.

Second reading, the genuine counter: a storage-format consumer is plausible rather than hypothetical.
Half-float and UNORM columns are real in the renderer and colour arcs (kirjo, ikiuni-renderer), and
for those, `Stored<BITS, U>` would be operative for *layout and conversion* even though arithmetic
happens at f32. If that consumer is scheduled, the axis earns its slot, but as a conversion contract,
which may want a different shape than an arithmetic identity axis.

Third reading, weakest but worth recording: keep the full axis now because renaming identity axes
later ripples through every alias set. I do not find this persuasive, since this round is itself
renaming things earlier rounds declared, which is evidence that pre-declared vocabulary drifts rather
than evidence that it saves work.

Whichever reading wins, the spec should stop citing decimal64 (spec:73-75) as its proof case; 01's
finding 7 already showed no axis carries a radix, and this finding shows nothing would execute a
decimal composition anyway. A proof case the design can neither express nor run is decoration.

## 2. The alias story destroys the shipped error surface. Verified by probe.

The spec's compatibility claim is that "the public spelling does not change" and that `UFixed<13, 3,
Warm>` "reads as itself" (spec:315-318), with the composition internal to the aliases, on the model
D40 and D52 established (`Rect` *aliasing* rank 2, inherited-state:1547, 2110).

That claim is true for what a consumer types and false for what the compiler says back. I probed it
on the pinned nightly: a type alias `UFixed<const I, const F, S> = Number<Bin<Implicit<0>, Unit,
Zero, Unsigned, 13>, S>` and a failed bound at a call site written `fold::<UFixed<13, 3, WarmS>>()`.
rustc's E0277 reports, verbatim:

```
the trait bound `Number<Bin<Implicit<0>, Unit, Zero, Unsigned, 13>, WarmS>: AddAssoc` is not satisfied
```

The alias is gone. The consumer wrote four tokens and is answered in ten, five of which they have
never seen. And that probe is the *short* form; a real composition carries the quantisation
five-tuple and the growth and the three lowering members, so the rendered type in a real error will
run to a line or more of markers.

Measure that against what ships today. `UFixed` is a concrete struct, so the current error surface
names it directly, and the shipped diagnostics are genuinely good:
`tests/ui/no_multiplicative_identity.stderr` says "this type has no multiplicative identity", names
`UFixed<{ ibits(0) }, { fbits(8) }, Hot>`, and carries a three-sentence remediation. That quality was
paid for deliberately (`arvo-strategy/src/identity.rs:81-84`) and the spec, as written, throws it
away as a side effect of an implementation convenience. 02's diagnostic findings (the `Faithful`
error naming `TowardNegative`, the E0271 attribute gap) compound with this one: the failure mode of
the whole redesign, as a consumer experiences it, is "I wrote `Warm` and the compiler is talking
about a rounding marker inside a type I never spelled".

Two repairs, both compatible with everything the prior members proposed.

The structural one, which I think is also the better design and take up again in section 7: the four
families stay **concrete newtypes** over the composition rather than aliases,
`pub struct UFixed<...>(Number<...>)`, `repr(transparent)`, with impls forwarded through the inner
type. One behaviour implementation, four concrete faces. Errors name the face. The cost is
forwarding boilerplate, which is macro work, and the loss of "is literally one type" purity, which
was never worth anything to a user.

The procedural one, which should happen under either shape: the top ten consumer-facing failure
modes (wrong strategy for a width, no multiplicative identity, non-associative fold refused, fallible
arithmetic unhandled) get authored as `.stderr` fixtures in `tests/ui/` **before** the shape locks,
exactly the mechanism the repo already runs, with "the error names what the consumer wrote" as the
acceptance criterion. 02 proposed restating sketch obligation 3 behaviourally; this is the same
proposal widened from one refusal to the error surface as a whole, and it converts diagnostic quality
from a hope into a gate.

## 3. `Precise` becoming fallible exiles it from the entire generic algorithm surface. Verified bounds, reasoned consequence.

The spec prices `Precise`'s fallibility at "call sites unwrap" (spec:269-271). That stops one layer
too early.

arvo's L2/L3 crates are themselves generic consumers of the numeric surface. `arvo-graph/src/lib.rs:10-12`
states its weight contract: `TotalOrd` for max-selection, `FromConstant` for zero init,
`core::ops::Add` for accumulation. sparse, comb and spectral are bounded in the same style. Those are
*total* arithmetic bounds. A composition whose addition returns through `Outcome` does not satisfy
`Add<Output = Self>`, and 02's section 7 established (verified, `f2_refusal.rs`) that one generic
body serving both the total and the fallible path cannot be written under the permitted features.

So the concrete consequence, which nobody in the round has priced: today `Precise` saturates
(`strategy_semantics.rs:79-83`) and flows through every algorithm crate; under the spec it refuses
and compiles into none of them. Three ways to land that, none free.

Accept the exile. Defensible, and half of me likes it: a saturated rank in a topological sort is a
silently wrong rank, and a `Precise` consumer arguably wanted to know. But then the spec must say the
sentence "Precise weights do not enter the algorithm crates; use Warm there", because otherwise
hilavitkutin discovers it as a trait-resolution failure eight bounds deep, which is the exact
experience section 2 is about.

Give `Precise` a total spelling that panics on refusal, keeping the fallible surface as `checked_*`
methods. I raise it to reject it: a panic is the policer move, it puts the refusal back in the
runtime where the whole round's point was to put it in the type, and `arvo-toolbox-not-policer.md`
cuts against it.

Bifurcate the algorithm crates on the fallibility projection. Real cost, roughly doubling the generic
surface of four crates, and 02's finding says the shared body cannot be recovered later. If a fallible
topo sort has a named consumer, fine; nothing today names one.

I lean to the first with the sentence written down, and I flag that this decision belongs in the
spec's consequences section, not in a downstream repo's debugging session. The spec's consequence
list is honest about direct call sites and silent about generic bounds, and generic bounds are where
the substrate's actual consumers live, per its own CLAUDE.md ("algorithm crates are generic over
numeric trait bounds").

## 4. The behaviour change is small, verified, and must be loud. Verified counts.

"We do not break userspace" is my rule, so let me apply it honestly instead of reflexively. Userspace
here is: arvo's own 654 tests, 20 `UFixed`/`IFixed` sites in hilavitkutin, zero in vehje (verified by
grep, section 0). Pre-1.0, in-workspace, no external consumers, and the workspace's no-legacy-shims
rule applies. The spec's premise that the meanings can be redefined is therefore sound. I checked it
because it is precisely the kind of claim panels have inherited wrong; it holds.

But the *shape* of the change deserves more respect than the spec gives it. Of the four preset
redefinitions, `Precise` breaks loudly (the type changes, section 3) and that is the good kind of
break. `Warm` and `Cold` moving from wrap to clamp, and `Hot` changing division rounding, are
**silent value changes under unchanged spelling**, the worst migration shape there is: nothing fails
to compile, numbers just come out different. The consumer that will feel it exists today:
`hilavitkutin-providers/src/adapt_ema.rs` runs its EMA blend on `UFixed<0, 16, Hot>` and
`UFixed<64, 0, Hot>` (adapt_ema.rs:45,53) and its own comment documents a deliberate mix of Hot
wrapping multiplies with saturating final adds (adapt_ema.rs:72). That code was written against the
current semantics and no compiler error will invite anyone to re-read it.

The mitigation is already house discipline, it just needs to be named as a migration obligation in
the spec: `strategy_semantics.rs` pins the current semantics with concrete raw-value assertions;
the round that flips the behaviour flips those assertions in the same change, deliberately, test by
test, and a manual audit of the hilavitkutin sites (twenty of them; an afternoon) is listed as a
gating step. Silent value changes are acceptable exactly once, pre-1.0, done as a single loud
audited event. The spec should say that instead of noting the redefinition in two sentences under
"What does not change", which is, frankly, a heading it contradicts.

One more thing on this axis, in the spec's favour, because fairness matters: the current `Cold` is a
stub. `strategy_semantics.rs:69-71` says so in its own words: "Cold and Hot share container widths at
L0; widen-narrow lands later". That is a half-built design sitting in the tree growing dependents,
which is the failure mode I hate most, and the spec finishes it. Finishing `Cold` alone justifies a
round.

## 5. The algebra ladder is speculative bloat. Reasoned, two readings.

Spec:306-311 declares the ladder "to the depth the theory goes rather than the depth arvo's numerals
reach", `Magma<Op>` and the law markers, on D38's reasoning that "a vocabulary fixed by mathematics
cannot be got wrong in a way that later needs undoing".

That is the wrong test, and it is a seductive one because it is true. The question for shipping a
declaration is never whether it can be wrong; it is whether anything calls it. The law derivation
needs `AddAssoc` now and will need the multiplication and distributivity markers when D2 lands.
Nothing in arvo or its consumers names `Magma`, and a trait with no impl-site and no bound-site is
the architecture-astronaut layer: it cannot be tested, it cannot be exercised, and its only activity
is being renamed by future rounds. This round is itself renaming vocabulary previous rounds declared
(`Combine<Op>` dropped for `Magma<Op>`, spec:308), which is the empirical answer to "cannot be got
wrong": the mathematics cannot, the naming and placement can, and did, within one week.

The counter-reading, held honestly: declarations are nearly free, D75 attaches the sketch-and-bench
obligation to implementations only, and the crate (`arvo-algebra-contracts`) exists regardless. If
the ladder is five traits and twenty lines, the carrying cost is small. My response is that the
repo's own five-layer doc discipline has a designed home for exactly this: BACKLOG.md.tmpl,
"designed-but-deferred promissory notes, names unbackticked" (`mock-workspace.md`). Declare the rungs
the derivation uses this round; write the rest of the ladder into the backlog where it costs nothing
and lints nothing. That is not deferral of design, the design is done; it is refusing to ship
uncalled code, which is a different thing.

## 6. The costs nobody counted, with today's numbers attached. Verified baselines, reasoned projection.

02's section 9 made the asymptotic argument (table encodings grow multiplicatively, typestate
projections additively) and is right that the encoding choice precedes the bench. What the round
still lacks is a baseline, so here is one, measured today on this machine.

A `touch` of `arvo-strategy/src/lib.rs` (the L0 crate the spec dismembers) recompiles twenty crates;
`cargo check --workspace` completes in 6.5 seconds. The full test build is 122 binaries; the current
strategy machinery is roughly one hundred impls across `arith_impls.rs`, `container.rs` and
`identity.rs` (grep counts: 37, 44, 25), mostly macro-generated. That is the "before". Whatever shape
wins sections 2 and 8 of panel 02, the bench that compares encodings should publish its numbers
against these, and per `bench-and-sketch-discipline.md` it can be written before `arvo-numeral`
exists. A substrate whose edit-compile loop stays inside ten seconds is a substrate a maintainer
iterates on; one that drifts to a minute quietly stops being iterated on, and nobody ever decides
that, it just happens.

Second uncounted cost, small and recurring: the spec adds five crates (numeral, policy, lowering,
algebra-contracts, numeric) to a fifteen-crate workspace. Each crate in this repo carries an
auto-generated forbidden-imports matrix (`.claude/rules/lint-forbidden-*.md`) whose dependency policy
is maintained by hand in the mockspace config, and the maintenance of that matrix grows with the
square-ish of the crate count in the worst case. Twenty crates for a numeric core is defensible under
house style (small crates, parallel check); it is not free, and the crate table (spec:291-299) should
be costed as one, not assumed.

Third, an impression rather than a measurement, labelled as such: the maintainer-debugging cost of
projected bounds. 02's working impl header
(`((<S::Quantisation as Quantisation>::OverRange, ...), <N as Numeral>::Sign): AddAssoc`) is the
*simple* case, one law, one operation. When a trait-solver failure lands in the middle of that under
three more axes and a GAT, the person debugging it is op, alone, months from now, without this
panel's context. Every projection layer is a frame on that stack. This is not an argument against the
machinery; it is an argument for 02's parameter split and the newtype faces (sections 2, 7), both of
which shorten the stack, and for keeping the axis count at the minimum the derivations actually
consume, which is findings 1 and 5.

## 7. One type or four faces: the question under section 2, stated as design. Reasoned.

The spec's dedup instinct is correct and I want to say so precisely, because the duplicate-hating
half of my own doctrine is where this design is strongest. The thing that must exist exactly once is
the **behaviour**: one quantisation machinery, one law table, one container projection. Today those
truths are spread across four families with hand-kept consistency, and the spec kills that duplication
properly.

But "one implementation per mechanism" has never meant "one type per domain". The four families are
not duplicates of each other; they are four *interfaces* to one mechanism, and interfaces are exactly
the thing you keep concrete, because they are what error messages, rustdoc pages, and grep results
are made of. The middle shape, which my section 2 probe motivates and which the panel has not
considered as a category (the brief asked for families of approach nobody touched; this is one):

`Number<N, P, L>` exists, internal, carrying every derivation, with 02's split parameters so the
independence claim types. The four families are `repr(transparent)` newtypes over their compositions,
impls forwarded by macro from the inner type. Diagnostics name the face. The derivation machinery
runs once. `Transparent` (already the house pattern, float.rs:76-84) makes the unwrap free. The
"conventions as alias sets" story survives unchanged, aliasing the faces or the composition as each
convention prefers.

The cost is a forwarding-macro layer, which is real but bounded, and the loss of a sentence ("arvo's
four families are one type") that was only ever true for the spec's author, never for a consumer. The
alternative reading, that alias-transparency is acceptable and `on_unimplemented` attributes can
carry the diagnostic load alone, is testable: write the ten `.stderr` fixtures of section 2 under
both shapes and read them side by side. That is a one-day sketch and it would settle this the way
things should be settled here, by the artifact rather than the argument.

## 8. Engagement verdicts on the first three files, kept short.

02's parameter split (its section 5, `Number<N, P, L>`) I endorse from a different premise than 02
offers: not because the claim should be typed for honesty's sake, but because an impl header that
*cannot* name a lowering member is one a reviewer never has to check, and review-discipline does not
survive years while type-enforced boundaries do. Cheap, no consumer-visible cost, do it.

03's bounded const-eval falsification checks (its section 3) I endorse without reservation, and from
my lens they are the most important thing in the panel so far: they are "run the smallest program
that decides it" in const form. 01 caught `SubstituteZero` by hand arithmetic in a review file. Hand
arithmetic does not re-run. A `const` block re-runs on every build forever, and the per-resolution
oracle is three lines. 03's own caveat (the oracle is a second place to be wrong) is honest and does
not change the balance; two independent three-line statements that must agree beat one blanket impl
that nobody checks.

03's section 5 (the notko cross-crate absence, unpinned) deserves louder treatment than it got:
arvo's fallibility soundness currently rests on an impl *not existing* in a crate arvo does not own,
and a plausible ergonomic addition to notko would silently void it. Pin it as a `tests/ui/` fixture
in arvo before any of `arvo-policy` is written; that is `catalogue-edge-cases-as-tests.md` applied
across a crate boundary, and it is an afternoon.

01's Kulisch reframing (its section 14) is where I partially dissent, on scope rather than substance.
The citation is correct and the projection-properties frame is the right mathematics; 03's
width-uniformity arguments even need it. But the panel should not talk itself into a vocabulary
rewrite that costs another full renaming round for the same machinery. Adopt the frame where it
deletes something (finding 4's round-then-resolve composition, the law derivation's property
conditions) and cite it in prose everywhere else. A rename that changes no behaviour and no guarantee
is churn, and this design round has already renamed enough of its own recent past.

On 03's "cannot lie" demolition generally: fully agreed, and I would go one step blunter for the
spec's revision. The honest sentence is not just "every case must be answered and no two answers may
disagree"; it is that this design replaces *a table someone had to remember to extend* with *a table
the compiler refuses to let you forget to extend*. That is the actual win, it is a real one, and
stated that way nobody will mistake the compiler for a proof assistant.

## 9. What earns its keep, collected, so the synthesis does not have to reconstruct it.

Stated plainly per the brief. The three-contract cut with D54's sorting test: keep, it is the load-
bearing good idea and it survived three hostile reviews. The quantisation five-position unification:
keep, with 01's round-then-resolve boundary fix, and it gets better under correction rather than
weaker, which is the mark of a right shape. Conventions as off-by-default alias features with a
behavioural (not spelling) fidelity test per 01 finding 4 and 03 section 6: keep; costs nothing to
anyone who does not enable them and turns vendor-mode folklore into greppable code. Finishing `Cold`:
overdue, the current stub is the kind of half-built thing that calcifies. The preset-from-intent
derivation: right in principle, priced in section 4. The totality machinery of 02/03: adopt; it is
the type system doing enforcement work, which is what op keeps asking for and what this workspace's
own rules say to want.

What I would cut or defer, with the reasons above: the parameterised `Stored<BITS, U>` and the
`Underflow` axis until a storage-format consumer is scheduled (section 1); the algebra ladder beyond
the rungs the derivation consumes (section 5); the alias-only public story, replaced by newtype faces
or explicitly re-justified against side-by-side `.stderr` evidence (sections 2, 7).

What must be added before a changelist: the generic-bound consequence of `Precise` (section 3), the
loud-migration obligations (section 4), the diagnostic acceptance fixtures (section 2), the notko pin
(section 8), and the encoding bench against today's baselines (section 6).

---

**Summary for the next member.** From the first non-formal lens: the decomposition earns its keep,
much of the superstructure does not yet, and the gaps are concrete rather than philosophical. The
float half of the ten axes describes arithmetic arvo will never execute, since floats are sealed
hardware wrappers (`float.rs:12-13,31-42`), so two identity axes currently have no operative caller
(section 1, verified). The alias compatibility story is half-true: spelling is preserved on input and
destroyed on output, since rustc expands aliases in diagnostics, verified by probe against the pinned
nightly, and the shipped error surface this would discard is one of the best things in the current
tree (section 2). `Precise`'s fallibility silently exits it from every L2/L3 algorithm crate, whose
total `Add` bounds are cited at `arvo-graph/src/lib.rs:10-12`, a consequence the spec does not price
(section 3). The behaviour redefinition is legitimate pre-1.0 (consumer counts verified: 20 sites in
hilavitkutin, none in vehje) but two of its four changes are silent value changes and need a named
audit obligation (section 4). Today's baselines for the compile-cost question nobody measured:
6.5-second workspace check on an L0 touch, twenty crates, roughly one hundred strategy impls
(section 6). And one category the panel had not opened: concrete newtype faces over an internal
composition, one behaviour and four names, which resolves the diagnostics finding structurally and
is decidable by a one-day fixture sketch rather than by argument (section 7). I rule on nothing; op
decides.
