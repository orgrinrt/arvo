
# 15: What a law is for

**Reviewer:** Max Willsey (equality saturation and e-graphs lens: what a law actually licenses
operationally, once you have one, and what breaks when a design has laws but no rewrite engine, or a
rewrite engine but no laws).

**What I read.** `11_current_shape_draft.md` in full. `13c_op_the_standard_and_the_mode.md`,
`13_mcsherry_where_the_laws_belong.md`, `13b_op_checkpoint_five.md`, `14_dolan_which_algebra_is_this.md`
and its `14_probes/`, per the brief. `12_lattner_fresh_read.md` in full, because file 14 cites it as
required background and its finding on the "arithmetic fidelity" axis (section 1) turned out to be the
single most load-bearing thing I read for this question, closer to my subject than anything in files
13 or 14. `mock/research/202607281616_prior_art/04_algebraic_structure_hierarchies.md` in full, which
file 14 already flagged as containing the design round's own hand-derivations, cited nowhere in the
draft. `ls` on the panel directory and on `mock/research/` and `mock/design_rounds/` before reading
inside either, per the standing instruction; nothing in either directory outside what the brief and
files 13/14 already named turned out to bear on this question.

On source, all named in the brief: `arvo-comb/src/`, `arvo-graph/src/`, `arvo-spectral/src/`,
`arvo-sparse/src/` (`dm.rs` and `block.rs` specifically, which files 12 through 14 left unread and
file 14 flagged as an open guess), `arvo-strategy/src/identity.rs`, `arvo-strategy/src/arith_macros.rs`
in full (the trait declarations in `arith.rs` too, for the doc comments describing per-strategy
semantics). Also `arvo/src/float.rs`, because file 12's fidelity-axis finding is about it directly and
nobody in files 13 or 14 followed up on it.

**What I compiled and ran**, as distinct from what I reasoned about: one probe,
`15_probes/01_distributivity_over_add_shipped_mul.rs`, `rustc -O`, exhaustive over a 16-value signed
model. Results in section 3. Everything else here is argument, offered as directions, and where I hold
more than one reading I say so and do not resolve it.

## 0. A premise check, and where this dive sits relative to the others

There is no ratified canon governing this specific question. `13c` states the standard directly (op
declined to pick among the three placement options offered and named the fixed test instead:
optimal, ideal, representative of the mathematics, capable of representing MATLAB/IEEE-754/SystemC)
and `panels-argue-the-intent-not-the-wording.md` is the operative posture: no locked design to defend,
only intent and spirit, argued against. The nearest thing to a ratified call bearing on my subject is
D47 (`mock/design_rounds/202607300800/202607292300_topic...md:38-63`, op, 2026-07-29): "the algebraic
ladder goes as deep as the theory does... every rung that goes in is sketched and benched." Nothing in
what follows proposes a rung; I checked, and I return to the point explicitly in section 6, because
the honest answer to "should arvo build something eqsat-shaped" is no, and D47's depth mandate is
part of why the answer is no rather than in tension with it.

## 1. What a law is for, stated the way my field states it

An algebraic law is not a fact you assert about a type and then admire. It is a fact you *use* when
you go from one program to a different, provably equal program. "Addition is associative" is not
interesting in the abstract; it is interesting the moment something wants to compute
`((a + b) + c)` as `(a + (b + c))` instead, or as `(a + c) + b`, or split the sum across four
accumulators and recombine, and needs to know the answer does not change. The law is the *permission
slip* for a rewrite. Congruence closure is the machinery that tracks, once you start handing out
permission slips, everything that follows: if `x = y` is licensed and `y` sits inside a bigger term,
every term built around `y` is now provably equal to the same term built around `x`, whether or not
anyone asked for that consequence directly. That is the whole of what an e-graph is: a hashconsed term
DAG plus a union-find over which subterms are interchangeable, kept honest by rebuilding congruence
after every merge, so that the set of "programs equal to this one" is always exactly what the laws you
have actually licensed say it is, no more and no less.

This design has, right now, laws in the first sense (facts asserted about a type, computed and checked
per the draft's own excellent Thread C machinery) and nothing at all in the second sense (a mechanism
that *uses* a law to go from one term to a different, provably equal one). That is not a defect. It is
the single most important thing to be precise about before saying anything else, because the two uses
of "law" this brief asks me to compare, a law that **gates** a call site and a law that **licenses a
rewrite**, are not two flavours of the same object. They are different objects, doing different jobs,
and this design currently has exactly one of them.

## 2. The gate and the license are different objects, and conflating them is the mistake to avoid

A **gate** is a static, binary precondition consumed once, by a human-authored combinator, at the
combinator's own boundary. `arvo-graph/src/rank.rs:39`'s bound `W: Add<Output = W> + TotalOrd + Copy +
FromConstant` (per file 12) is a gate in embryo; McSherry's and Dolan's proposed
`W: Monotone<Additive, TotalOrd>` (`13_mcsherry...md:194-199`, `14_dolan...md:242-245,378-380`) is the
gate this design actually needs, stated properly. The combinator's *body* is written by a person who
trusts the gate to mean "it is safe for me to do what I am about to do", and the gate is checked once,
at compile time, per composition. Nothing about a gate generates a new program. It only refuses or
admits the one the author already wrote.

A **rewrite license** is a directed or bidirectional equality between two *terms*, consumed by a
search procedure that does not know in advance which of the equal terms is best, and whose job is to
explore the space those licenses generate and extract the one with the lowest cost. `a * (b + c) = a *
b + a * c` read left to right is "expand"; read right to left is "factor." Neither direction is
uniformly better. A system that greedily applies one direction can miss the improvement the other
direction would have found, and worse, if you apply both directions greedily in some fixed order, you
can genuinely loop or explode the term size, which is exactly the failure equality saturation exists
to remove: run every licensed rewrite to a fixed point (or a budget) *without commitment*, so every
form the licenses generate coexists in one structure, and only then pick the best one. Extraction
happens once, at the end, decoupled from search. This is a decision procedure over a possibly large,
possibly infinite space of equal terms. It has no analogue anywhere in arvo's current design, because
arvo has no term representation for its own arithmetic expressions to search over: what a consumer
writes is monomorphised Rust source, and this design correctly, deliberately, does not propose to
macro-rewrite consumer expressions (nothing in the draft, in D47, or in `arvo-toolbox-not-policer.md`
asks for that, and I want to say plainly that it should not: see section 6).

So: does the design want the gate, the license, or both? On the evidence read so far, **it wants the
gate, has already half-built the gate (the derived-truth-value machinery in draft 3.4, Thread C), and
should not build the license.** McSherry's reading two (`13_mcsherry...md:186-199`) and Dolan's section
6 (`14_dolan...md:360-384`, "no arvo trait bound gates ordinary usage on an algebraic law by default...
a law is required only by the specific combinator that performs the regrouping") already land here
independently, from different angles, and I agree with both, for a reason neither states quite this
way: what they have converged on **is** the correct scope for a gate-only design, and knowing that it
is gate-only, rather than license-generating, tells you exactly what NOT to build next, which
generic "let's make the laws generate the rewrites automatically" instinct would otherwise reach for.

## 3. Multiplication and distributivity, tested for the first time

Both prior files flagged multiplication and distributivity as entirely untested (draft 5.2,
`14_dolan...md:451-457`, "run the exhaustive small-model probe first, before assuming distributivity-
over-max-or-min generalises to `*`"). I ran it, against the *shipped* code, not against the draft's
stated intent for the presets, and the two disagree with each other in a way worth stating before the
law result itself.

**The shipped fixed-point multiply does not implement the preset table's stated rounding mode, for any
strategy.** Draft 3.5's table (`11_current_shape_draft.md:327`) states the in-range resolution as
`truncate` for `Hot` and `nearest, ties to even` for `Warm`/`Cold`/`Precise`. The shipped
`u_mul_fixed`/`i_mul_fixed` bodies, in every one of the six strategy-shaped macros in
`arith_macros.rs` (`impl_u_arith_wrapping` at lines 33-34, `impl_u_arith_saturating` at lines 95-101,
`impl_i_arith_wrapping` at lines 147-148, `impl_i_arith_saturating` at lines 214-220, and the two
128-bit-container widened variants at lines 464-465 and 510-511), compute the product then apply an
unconditional `>> FRAC`, a floor, with no round-to-nearest and no ties-to-even logic anywhere in the
file. This is true for `Warm` and `Cold` and `Precise` exactly as much as it is for `Hot`. I want to be
precise about what this is and is not: the draft's own preamble to 3.5 says the table states each
preset's *intent*, "redefined from what its name states as intent rather than from what it happens to
do today," so this is not news that the design contradicts itself; it is news that **nobody has yet
said, in this dive or the last one, that the gap between intent and shipped code is total for
multiplication specifically**, and total in a way it is not for addition: addition's shipped `u_add`
already does what its documented per-strategy semantics claims (wrap for Hot/Warm/Cold, saturate for
Precise, per `arith.rs`'s own doc comment and the strategy-semantics tests file 12 cites). Draft
section 4.3's own Thread C, the fifth-pass mechanism that proves the checked function and the executed
function agree, has been built and verified for `+` and for nothing else; there is no checked function
for `*` at all yet, so there is nothing for a future `*_mul_fixed` checked function to disagree with
in the way Thread C's fourth pass caught for addition (draft, section 4.3). Whatever multiplication's
`Quantisation` ends up being, it needs its own version of that exercise, from scratch, and today's
shipped truncation is the honest starting point for it, not the table.

**Against that shipped truncation, distributivity and multiplicative associativity both fail, for both
Hot and Saturate.** `15_probes/01_distributivity_over_add_shipped_mul.rs`, exhaustive over a signed
Q2.2 model (raw range `[-8,7]`, representing reals `[-2.0, 1.75]`), modelling `u_mul_fixed`/
`i_mul_fixed` exactly as shipped (`wrapping_mul` then `>> FRAC`, then the same range recovery every
other op gets):

```
distributivity of shipped * over shipped +:
Wrap (Hot): a*(b+c) == a*b+a*c: NO at (a=-7,b=-8,c=-8): lhs=0 rhs=-4
Saturate (Warm/Cold/Precise shape): a*(b+c) == a*b+a*c: NO at (a=-8,b=-8,c=1): lhs=7 rhs=5

associativity of shipped * alone:
Wrap (Hot): (a*b)*c == a*(b*c): NO at (a=-8,b=-8,c=-7): lhs=0 rhs=4
Saturate (Warm/Cold/Precise shape): (a*b)*c == a*(b*c): NO at (a=-8,b=-8,c=-4): lhs=-7 rhs=-8
```

In real values, the first counterexample: `a = -1.75, b = -2.00, c = -2.00`, and `a*(b+c) = 0.00`
against `a*b + a*c = -1.00`. A whole unit of error, on a range four units wide, from a distributivity
failure nobody had named a counterexample for before this probe.

**Why this is a structurally different failure from addition's, and why it matters for a future
`Distributes<Mul, Add>` fact.** Addition's non-associativity, per files 13 and 14, comes entirely from
the range-boundary recovery map (clamp or wrap firing only when a sum leaves `[LO, HI]`). Every
`u_mul_fixed` call **always** quantises, whether or not the product would have overflowed the logical
range at all, because the `>> FRAC` shift is unconditional: it is the mechanism that returns a scaled
product to the right scale, not a boundary-only correction. So multiplication has two independent
sources of law failure where addition has one: the same range-recovery non-associativity addition has,
*plus* an always-firing truncating quantisation that addition never pays. Any atomic-fact vocabulary
built for addition (the `AddAssoc` shape, or McSherry's and Dolan's proposed `Monotone`/`Distributes`
atoms) needs a genuinely new lemma for this, not a renamed copy of the addition-side one, because the
always-firing shift changes which inputs disagree even in cases where the boundary-only reasoning
would have said they should not.

## 4. This is the field's canonical hard case, and it is already colliding with something in this design

Distributivity of `*` over `+` under rounding is not an obscure corner of my field. It is close to the
textbook motivating example for equality saturation itself (Tate, Lattner et al., "Equality Saturation:
a New Approach to Optimization," POPL 2009): expanding `a * (b + c)` grows the term and factoring
`a*b + a*c` shrinks it, greedy rewriting in either fixed direction alone provably misses cases the
other direction would have found, and this is one of a small number of examples the field reaches for
whenever it needs to explain, to someone who has not seen it before, why "apply the rules in some
order until nothing changes" is not the same thing as "compute the best equal program." I mention it
because file 12's independently-derived "FMA collision" (`12_lattner_fresh_read.md:92-96`) is exactly
this, arriving from a completely different direction: `arvo-always-optimal-internals.md` licenses
lowering `a*b+c` to `llvm.fmuladd`, a single rounding step, in place of two separate rounded
operations, and Lattner's point is that nothing in this design currently distinguishes a composition
where that substitution is a silent value change (a `Policy`-shaped fact this design's own sorting test
should have caught, per D54) from one where it is not, because no axis carries the distinction at all.
That is the same fact my probe found empirically, arrived at by a completely different route: rounding
interacts with `*` and `+` together in a way it never does with `+` alone, and this design has, so far,
exactly one place that acknowledges it (Lattner's proposed fidelity axis) and zero places that check it
against a real model the way this dive's earlier probes checked `+`. I would want the fidelity-axis
question and the multiplication dive to happen together, or at minimum for whoever runs the
multiplication dive to read file 12 section 1 first, because they are the same question asked twice.

## 5. Is the atomic-fact mechanism a rewrite system in disguise

No, and it is worth being exact about why, because the "no" is doing real work and a sloppier answer
would license the wrong thing next.

Dolan's reading two (`14_dolan...md:255-296`, each of `Associative`, `Commutative`, `HasIdentity`,
`Idempotent`, `DistributesOver`, `Monotone` as its own atomic, independently-derived marker, named
structures as derived blanket impls over conjunctions of atoms) gives you exactly the *applicability
condition* layer a rewrite engine would need: for each candidate rewrite (regroup this fold, factor
this product, fuse this multiply-add), you would ask "does the composition in front of me satisfy the
atomic fact this rewrite's soundness depends on," and the atomic-fact machinery answers that question
cleanly, cheaply, and (per Thread C's fourth and fifth passes) in a way that is provably connected to
the code that actually runs. That is the *conditions* half of a rewrite system, and it is good, and I
would build it regardless of anything else in this file, because McSherry and I both find the same
combinator-level gate is what the shipped consumers actually need (section 2 above).

What it is missing, and what would need to exist before "rewrite system" is an honest word for it, is
the other two-thirds: a **term representation** (something the mechanism can see as a syntax tree with
subterms, not a fixed set of hand-enumerated combinator shapes each written once by a human), and a
**search-and-extract procedure** (something that explores what the licensed equalities generate and
picks the best result by a cost function, rather than a human picking one candidate shape, benching it
once, and freezing the choice). Arvo, right now, has neither, and per section 6 below I do not think it
should grow either inside its own crates. So the honest statement is: **the atomic facts are the
applicability-condition oracle a rewrite system would need, not a rewrite system, and the gap between
the two is exactly the two pieces that need dynamic, unbounded structures to hold** (a term DAG that
grows with the expression being optimised; a worklist that grows and shrinks as rewrites fire). Saying
this costs one paragraph in the design's own text. Not saying it costs a plausible-looking next step
someone eventually takes: reading Dolan's "each `Resolution` constructor states its own lemmas... a
type-level fold combines them" and concluding the natural next move is to make the fold itself
generative, searching over which lemma-licensed regroupings exist rather than only checking one. That
next move is where the constraints in section 6 bite hard, and it is cheaper to rule it out explicitly
now than to let someone build partway into it later.

One thing worth crediting on the way past, because it is a real and useful property and it is not an
accident: marker-trait conjunctions genuinely do not collide under Rust's coherence check the way the
draft's original per-`Resolution` blanket impls did (`14_dolan...md:298-305`). `T: Associative<Op> +
Commutative<Op>` being simultaneously true is not competing evidence, it is a conjunction, and nothing
downstream has to choose a winner. This is the same shape as an e-class in an e-graph carrying several
simultaneously-true equivalences without contradiction: congruence is additive, never competitive,
which is exactly what makes deferred rebuilding sound in the first place (you can merge classes in any
order and re-establish the invariant afterward, because nothing about one equivalence disputes
another). Dolan found the Rust-coherence version of the same fact independently, from a completely
different direction, and it is worth saying so plainly rather than letting it read as a coincidence.

## 6. Where equality saturation for this exact problem already lives, and why it should stay there

The honest field answer to "should arvo build an e-graph" starts with: **the mechanism categorically
does not survive `no_std`, no `alloc`, const-generic-only sizing, monomorphisation-only dispatch.** A
real e-graph is hashconsing (a growing interned table of terms) plus union-find (a growing disjoint-set
structure) plus a worklist of pending merges, and its whole value proposition is that none of those
three sizes is known in advance: the term DAG grows exactly as large as the program you feed it. Thread
C's fifth pass already does something that looks adjacent (`[const]` generic function, checked
exhaustively at a small model width, executed unmodified at the real width) and it is worth being
precise about why that is a *different, much smaller* job: it proves *one specific claimed boolean
fact* true against a bounded, small, fully-enumerable input space. It does not search a *growing space
of candidate rewritten programs* and it never needs to hold more than one small proof obligation in
memory at a time. Reusing that machinery for search would mean holding the state of an in-progress
saturation, which is unbounded by the size of the term being optimised, inside a `const fn`, on a
target this design explicitly forbids `alloc` on. It does not fit, and no amount of cleverness about
`ConstParamTy` or `min_generic_const_args` changes that, because the thing that does not fit is not a
representation problem, it is a growth problem.

But the *problem* equality saturation exists to solve, "which of several algebraically-equal
regroupings of this arithmetic is fastest, and how do I avoid committing to one before I have checked,"
is real here and already has a home, one layer down from where arvo would ever touch it: **the
compiler backend arvo already depends on.** LLVM's fast-math pipeline (`InstCombine` plus the
`Reassociate` pass, gated by the `reassoc`/`contract` flags `float.rs:4,27-29` and `12_lattner...md:26`
already name as `FastFloat`'s entire differentiator from `StrictFloat`) is a real, shipping, ad hoc,
fixed-pass-order rewriter over exactly this kind of term, and it is the textbook cautionary tale my
field points to for what greedy, order-dependent rewriting costs: `-ffast-math`-class flags are widely
and correctly distrusted in numerical computing precisely because which answer you get can depend on
pass order, target, and compiler version, with no confluence guarantee at all, which is the same
"folklore versus benchmark" problem in different clothes. Cranelift, a different backend in the same
ecosystem, has already had this argument and reached my field's answer directly: its mid-end optimiser
is an *acyclic e-graph* (Chris Fallin, "ægraphs: Acyclic E-graphs for Efficient Optimization in a
Production Compiler," CGO 2023), built explicitly in egg's lineage, replacing exactly the kind of
single-pass greedy peephole rewriting LLVM's `InstCombine` still does for most of its rule set. I flag
this as field knowledge, not as something I verified against this repository; I found no evidence
either way in this workspace of which backend arvo's consumers actually build against, and I did not
go looking, because it is out of the scope I was dispatched into. It is offered as a direction: **if
this design ever wants the actual regrouping search Cranelift's mid-end already performs, correctly,
for exactly this class of problem, the right move is not to build a bespoke arvo-level e-graph, it is
to make sure arvo emits the annotations (fast-math flags today; per Lattner's fidelity axis, something
richer tomorrow) that let whichever backend the build target uses do that search safely, on arvo's
behalf, using facts arvo already derives.** That is a mapping problem (per-composition atomic fact to
backend-level license), not a search-engine-building problem, and it is a much smaller and much more
honest piece of work than the alternative.

Today's actual state, worth stating precisely because it is better than it looks at first glance:
**fixed-point arithmetic (`Hot`/`Warm`/`Cold`/`Precise`) gets no reassociation license from LLVM at
all, because LLVM has no concept of "wrapping-fast-math" or "saturating-fast-math" for integers.** So
the correctness risk McSherry's finding raises (a signed-saturating regrouping is unsound; my probe
adds that a signed multiply-then-add regrouping is unsound too) is currently averted *by accident*, not
by design: nothing tells the compiler it may reassociate this arithmetic, so it does not, and the only
place unsound regrouping can currently enter is a human hand-writing one (McSherry's four-way
accumulator, section 7 below). Float gets the license unconditionally under `FastFloat`, which is
honestly the correct shape for float specifically, because essentially no nontrivial algebraic law
holds exactly for IEEE arithmetic regardless of grouping (file 14 section 1, `04_algebraic_structure_
hierarchies.md:249-254`), so "best-effort, no guarantee, opt in explicitly" is not a corner cut, it is
the accurate license for a domain with no exact law to violate. Neither IEEE 754, SystemC, nor MATLAB
ships a general-purpose reassociating optimiser of its own; each defines a fixed evaluation order and
gates any deviation from it behind an explicit, user-selected relaxation (which is precisely
`FastFloat` versus `StrictFloat`'s existing shape). So the "capable of representing MATLAB, IEEE 754,
SystemC" test this dive is measured against does not call for an eqsat-shaped mechanism either: none
of the three target systems has one, and building one anyway would be adding depth the mathematics
these systems actually need does not call for, which fails "optimal, not adequate" in the wasteful
direction as surely as under-building fails it in the other.

## 7. What does transfer: three specific pieces of discipline, not the mechanism

None of the hashcons-and-union-find survives. Three things from how my field thinks about this problem
do, and I want to name them as concrete, actionable pieces rather than as a mood.

**Separate the congruence question from the extraction question, and this design currently conflates
them inside one hand-written function body.** "Is this regrouping legal for this composition" (a
congruence fact) and "which legal regrouping is fastest" (an extraction decision) are different
questions, and equality saturation's whole design is built around answering them in that order, never
the reverse. McSherry's bench (`13_mcsherry...md:364-396`, `fold_sequential`/`fold_paired`/
`fold_quad`) answers the second question well, with real numbers, through the proper harness. It never
asks the first: nothing in the bench, or in `arvo-always-optimal-internals.md`'s licence to write any
of those three variants, checks that the regrouping being benched is sound for the composition it will
ship against. Today that is fine, because FNV-1a (the bench's own subject) is explicitly non-
associative and the bench prices the shape of breaking a dependency chain, not a law-preserving
regrouping, as McSherry's own honest caveat says (`13_mcsherry...md:399-402`). The moment someone
benches the *actually law-preserving* regrouping the design wants (a checked, associative,
identity-having fold, split four ways), the congruence question has to be asked and answered first, or
the bench is measuring the speed of a program that is not the one anybody meant to ship.

**"If it cannot be pulled out and reused, it is not finished," and right now this specific mechanism
cannot be.** Every place in the workspace that needs a checked regrouping is, today, separately and
bespokely getting it right or wrong: arvo's own hot loops (licensed freely by
`arvo-always-optimal-internals.md`, checked by nothing but a human and a bench), hilavitkutin's
`merge_accums` (associative, in-order, correct per McSherry's audit), and hilavitkutin's
`ConvergenceBuffer::combine` (already found broken by McSherry and recorded in `13b`: an unbounded
`fn(T, T) -> T` folded across every slot including ones no core wrote, with an `init` unrelated to the
constructor's `zero`, so `combine(0, max)` on `[-3, -1]` over two live slots of four returns `0`, a
value present nowhere in the data). I am not re-investigating that defect; McSherry found it, `13b`
recorded it as `13c` explicitly ruled the hilavitkutin side is not this review's to act on, and I am
citing it here only because it is the cleanest illustration on hand of exactly what this section is
about. The defect is a value folded in without its equivalence to the combiner's identity ever having
been established, which is precisely what an unmerged e-class looks like when you translate it out of
my field's vocabulary: a missed equivalence, sitting quietly, until an extraction reads it and produces
an answer that is wrong in a way no local inspection traces back to the thing that was skipped. The
draft already ships the exact fact that would have caught it, `Identity<Additive>` (`identity.rs:51`),
and nothing routes it into the combiner that needed it. Whatever this design's ladder settles into,
the one piece of substrate I would build first, ahead of naming any more structures, is a single,
generic, reusable "regroup this fold, checked against the composition's own derived laws" combinator
that every one of those three call sites can reach for instead of writing its own. That is the sharing
this field's whole engineering case rests on: two consumers reinventing the same checked regrouping,
one of them wrong, is exactly the state e-graphs existed to get compiler writers out of, applied here
to a library instead of a compiler pass.

**Congruence closure is the whole soundness boundary, and it is a general lesson, not only an e-graph
one.** Every operation on a structure that claims an equivalence has to preserve that claim, all the
way, or the claim is a lie the structure tells about itself. This design's own instinct is already
pointed the right way (Thread C's fourth pass finding, draft section 4.3, that a checked classification
and an executed arithmetic pipeline can both individually pass every test while silently disagreeing
with each other, "the single sharpest finding produced anywhere in this review"), and it is the same
finding, in the same shape, as an e-graph whose rebuild is skipped on the reasoning that most merges do
not actually need it: the unmerged class is a missed equivalence, the missed equivalence is a wrong
extraction, and nothing local catches it. Thread C's fifth-pass fix (one definition, monomorphised
twice, checked at a model width and executed at the real one) is the correct discipline, applied
honestly, and I would not change anything about it. I would extend the same standard to whatever comes
next: a `Distributes<Mul, Add>` fact, once it exists, earns the name only once its checked function and
`u_mul_fixed`'s actual body are the same text, the way `AddAssoc` now is, not before.

## 8. Whether a set of laws should be closed under consequence

Three different things hide under this one question, and they deserve three different answers.

**Closed under structural composition, which is what named structures already mean and should keep
meaning.** `Monoid<Op>` as `Semigroup<Op> + HasIdentity<Op>`, derived rather than declared, is closure
in exactly the sense that matters here: nothing about `Monoid` should ever assert anything its
constituent atoms did not already establish. Dolan's reading three (`14_dolan...md:307-328`) already
gets this right, and it is the correct amount of closure to build, now, for every named structure that
follows.

**Closed under second-order mathematical consequence, which nothing here attempts and nothing here
currently needs.** A genuine dioid (idempotent semiring) licenses real further structure once you have
one: matrix powers over it, Kleene closure, all-pairs shortest paths via repeated squaring, which
Dolan's own footnote names (`14_dolan...md:372-373`) as "a strictly stronger requirement than anything
currently shipped needs." That is true today and I would not build toward it yet, on D47's own terms:
the depth mandate says go as deep as the theory goes, not deeper than a consumer needs, and no shipped
consumer performs a chained `⊗` today (section 6 of file 14 already establishes this cleanly). What I
would do instead, per `catalogue-edge-cases-as-tests.md`, is write down, now, as a catalogued and
explicitly red or ignored test, the specific consequence a future tropical matrix-power routine would
need (associativity and identity of `⊗` as well as `⊕`), so the moment a consumer actually asks for it
the gap is a known, named, tracked thing rather than a rediscovery.

**Closed under rewrite consequence, which is a third, different notion, and does not apply, because
there is no rewrite engine to be closed for.** This is the sense in which "closed under consequence"
would matter to an e-graph specifically: given the equalities you have licensed, what further equalities
follow, transitively, that a search could exploit? Section 5 already answers this: the atomic facts are
the condition layer such a system would need, not the system, so there is nothing here for this third
kind of closure to be closed *over*. I raise it separately from the second kind because it would be easy
to read "the markers are not closed under consequence" as one complaint with one fix, when it is really
three questions, two of which have real, different, actionable answers and one of which is not this
design's problem to solve at all.

## 9. What I would flag for the next member, unresolved

**The reusable checked-regrouping combinator named in section 7 is a real design gap, sketched here in
one paragraph and nowhere built.** I would want it prototyped against `arvo`'s own crates first (per
`13c`'s "not our concern" ruling on hilavitkutin, and per this file staying inside arvo), generic over
`Op` and bounded on whatever the ladder settles the associativity/identity atoms as, before anyone
proposes it as a cross-repo primitive.

**The fidelity axis (Lattner, section 1 of file 12) and the multiplication dive should happen together,
not sequentially.** My probe is evidence for the same underlying fact Lattner's FMA-collision finding
names from the compiler side; whoever runs the next multiplication-focused pass should read both before
starting, because treating them as two separate gaps risks two separate, incompatible fixes for one
problem.

**I did not check whether any consumer outside arvo performs its own hand-written regrouping of arvo
values**, which would be a third site (alongside arvo's own internals and hilavitkutin's merge shapes)
where the congruence-versus-extraction conflation in section 7 could be live today. Out of the scope I
read.

**Whether `arvo-sparse`'s `dm.rs` and `block.rs` are a fifth algebraic shape, which file 14 flagged as
an open guess (Boolean semiring, reachability), is settled by reading them: they are not.** Both are
pure combinatorics over node classification and DFS visitation
(`arvo-sparse/src/dm.rs:66-96,107-137`, `arvo-sparse/src/block.rs:29-83,95-155`); the only arithmetic
in either file is an unsigned index increment by `<USize as Identity<Multiplicative>>::IDENTITY`
(`dm.rs` has none at all; `block.rs:45,114`), which is natural-number successor and carries no law
question worth a member's time. I would close that open item rather than leave it for a later dive to
re-open.

**Whether the "closed under second-order consequence" question in section 8 changes once
`arvo-num-systems` enters the picture** (Dolan's section 5, `14_dolan...md:330-358`, on where ring and
field talk actually belongs) is not something I looked at; I read no file from that crate.
