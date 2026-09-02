# Derived algebraic laws, derived cold

**Phase one.** Written from `INTENTS.md`, the workspace rules, arvo's own `.claude/rules/`, and the
acceptance criterion the dispatcher gave directly: "have the typestate derive the matching container and
numeral representations, then validate, and erase." That last sentence is worth flagging before anything
else: I did not find it, word for word, inside `INTENTS.md`'s twelve entries during this cold pass, and
under the protocol I am not permitted to open the numbered panel files where it may in fact live verbatim
(`32`, `36`, `37` are the ones I would expect to check, based on where the strategy intents that echo it
sit). I take it as given by the dispatcher rather than as something I have personally traced to a
`file:line`. Whoever reconciles this file against the panel should confirm the wording against its actual
source before treating it as op's own words rather than a paraphrase of them.

I read no panel file, no option register, no droplist, no archaeology, no probe, no git log, and no commit
message under this directory before writing this section. Three probes back the claims below, committed at
`77_probes/` in this same directory, and I built and ran them myself rather than citing anyone else's.

## What a law is not: a property of a type, and not quite a property of an operation either

The first move a canon could make is the one that turns out wrong on inspection: state laws as facts about
a type, the way a textbook states "the reals form a field." A single instantiation of arvo's numeral,
`UFixed<I, F, S>` for some concrete `I`, `F`, `S`, has no laws at all by itself. It is a bit pattern and a
decoding function. Laws only appear once an operation acts on it, so the type alone is the wrong unit.

The operation alone is also the wrong unit, for a more specific reason than "operations need operands."
`I9` states it directly: "strategies are the variables that change what the 'correct' answer is for what
we choose as the path." An abstract `Add` has no laws until `S` fixes what rounding, what overflow policy,
and what intermediate width the addition actually performs. So a law is at minimum a fact about a
triple, operation plus shape plus strategy, and even that triple turns out to be too coarse, because `I7`
draws a line the triple cannot express on its own: Precise is "accurate ... especially within chains and
ops, not only alone." A single well-formed add can be exact under every strategy and the same strategy can
still fail a law that only shows up once several ops are composed. So the unit a law is stated about has to
be able to name a sequence, not a single application.

The shape this settles into, and it is a familiar one from denotational semantics rather than something
invented for arvo: a law is a property of a **judgment**, an equivalence between two expression forms
(which may be chains of arbitrary finite length) evaluated under a fixed strategy, holding either exactly
or up to a stated tolerance. `x + 0 ~ x` is the one-step degenerate case of this judgment. `(a * b) * c ~ a
* (b * c)` is the two-step case. The general form covers both without needing a separate vocabulary for
"single-op laws" and "chain laws." I return to why this distinction matters mechanically, not just
notationally, in the next section.

## Two different things a law can be approximating, and conflating them is a real risk

Before asking where laws come from, it is worth being precise about what the "correct" answer a law is
measured against actually is, because arvo's own stated intents name two different ideals, not one.

`I3` and `I4` say Warm "behaves like native primitives in regular old rust would," and that mimicking
native behaviour is what makes Warm's choice "intuitive," even though `I4` is careful that imitation
"serves it rather than defines it." That ideal is **native Rust arithmetic semantics**: wraparound modular
arithmetic for integers, IEEE-754 semantics (including IEEE-754's own non-associativity) for anything
float-shaped. `I7`, by contrast, describes Precise as wanting "the most precise possible answer," which is
an ideal drawn from **the exact value the numbers are standing in for**, the rationals or reals the
fixed-point or floating-point encoding approximates.

These are not the same ideal, and a law that is true relative to one can be false relative to the other.
Native `u8` wrapping addition satisfies `(a + b) + c == a + (b + c)` exactly, associativity holds
unconditionally for modular addition. The same three values summed against the exact-rational ideal, with
saturation or a defect penalty instead of silent wraparound, need not agree with the wrapped answer at all,
even though both are "associative" in their own right. A canon sentence that says "arvo's algebraic laws
approximate field axioms" would already be a category error for Warm, whose laws are laws about *hardware
behaviour*, not about the field of real numbers. The literature this is well trodden in, Flocq's
`generic_format` and its rounding-correctness lemmas, gives one unified vocabulary for stating "how far a
rounded operation is from the operation over the reals it approximates," and that vocabulary is exactly
right for the Precise ideal and the fixed-point/float unification the project's own memory already notes.
It is the wrong vocabulary, on its own, for stating what Warm owes native Rust semantics, because that
target is a machine, not a mathematical structure with the reals' axioms. A canon that only reaches for
Flocq-style transport has silently decided every strategy answers to the same ideal, and `I3`/`I4` say at
least one of them does not.

## What "derived" can mean, and why the two readings are not actually in tension

The word carries two distinct senses and the prompt is right to ask which one a canon means.

**Reading A, derivation as proof.** A law is derived when it is a *theorem*, obtained from more primitive
facts (a rounding function is monotone, a rounding function is idempotent on values already representable,
a single rounding step's error is bounded by half the quantum under round-to-nearest) rather than asserted
as an independent axiom that happens to be true. Under this reading, "derived" is a property of how the
canon justifies a law: does it show the mechanism that makes the equation hold, or does it merely claim the
equation holds.

**Reading B, derivation as generation.** A law is derived, in the same sense op's acceptance criterion uses
the word for a container, when it is the **output of a procedure** parameterised by strategy and shape,
rather than a hand-written table entry. Given `(operation, I, F, S)`, the procedure decides: does the law
hold exactly, hold within a stated bound, or fail. This is the "generator of laws" the prompt names as an
open question, and it sits at the same pipeline stage op's own words describe for containers: derive
(compute what the law claims for this instantiation), validate (check the claim, at compile time where
possible), erase (nothing about the check survives into the artifact a consumer links against).

I do not think these are competing answers to choose between. Reading B is what makes Reading A checkable
by a machine rather than only arguable in prose. A canon that states the primitive lemmas (Reading A's
raw material) and a combinator that says how they compose across a chain (the mechanism Reading B needs to
be a generator rather than a case-by-case table) gets both: a human reading the canon sees the proof
obligation stated as a small number of named lemmas, and an implementation reading the same canon has
enough to build a mechanism that certifies any specific instance without the canon ever needing to
enumerate instances.

There is a real cost to the generator reading, and it deserves to be named rather than assumed away. A
list is directly auditable: a human can read "distributivity holds for Hot at widths up to 32" and check it
against a test suite entry. A generator is only as trustworthy as the generator itself, and asking "is the
generator correct" is the same kind of question one level up, subject to the same doability bar
(`the-canon-is-intent-not-implementation.md`: an intent not established as achievable is a wish). This does
not refute the generator reading; it says the canon owes the generator's own soundness an establishment,
not a free pass for being one level more abstract than the laws it produces.

## The generator is mechanically doable without the forbidden features, at two different levels

This claim needed checking rather than asserting, since the whole point of "derived" under Reading B is
that it names something a mechanism can be asked to do. Two probes back it, at two different levels of what
a law is about.

`77_probes/probe2_works_validate_erase.rs` states "this shape has a representable multiplicative identity"
as an associated const on a trait, computed inside a `const fn` body that runs `assert!(Self::I >= 1, ...)`
before it hands back the raw encoding. This is exactly derive, validate, erase as one compiler mechanism: the
raw one-value is *derived* from `I` and `F`, the assert *validates* it at compile time (an `E0080`
const-eval failure, not a runtime panic), and when the assert passes the whole thing lowers to a plain
constant, nothing about the proof obligation survives to the artifact
(`77_probes/probe2_works_output.txt` shows `Fixed<3,5>::ONE_RAW = 32` with no residual machinery). I checked
the naive alternative first and it fails exactly the way `a-refused-bound-wants-a-trait-not-a-feature.md`
predicts: `77_probes/probe2_fail_inline_const_expr.rs` writes the constraint as an inline
`where [(); (I >= 1) as usize]:` bound, and the pinned nightly refuses it and suggests the forbidden
`generic_const_exprs` feature by name (`77_probes/probe2_fail_output.txt`). The trait-and-associated-const
shape does not need that feature at all. I then checked that the validation actually bites rather than being
decorative: `77_probes/probe2_bad_case_refused.rs` instantiates the exact shape the project's own memory
records as a real bug, `Fixed<0, 8>`, the same shape as `UFixed<0, 8, Hot>` whose `ONE` held raw `0`, and
the assert refuses it at compile time with a named diagnostic rather than silently emitting the wrong
constant (`77_probes/probe2_bad_output.txt`).

`77_probes/probe3_strategy_resolve_lattice.rs` checks the same shape of claim one level up, about a law
concerning how *strategies themselves* compose rather than how numbers do. It deliberately avoids the
`Hot`/`Warm`/`Cold`/`Precise` names, since `I1` leaves the strategy set open, and models a strategy as a
point in a two-axis product order rather than a single ranking, since `I8`'s "they weigh different
measurements differently" reads more naturally as several independent axes than as one linear scale. The
probe states commutativity, idempotence, and associativity of the resolution operator as compile-time
`assert!`s inside a `const _: () = { ... };` block, checked against three concrete points, and it compiles
clean (`77_probes/probe3_output.txt`). To make sure the check was not vacuous, I broke the resolve rule
(biased it to always take the left operand's rank instead of the join) and reran: the same commutativity
assert now fails at compile time, named and specific
(`77_probes/probe3_negative_output.txt`, `77_probes/probe3_negative_check.rs`). The mechanism catches a
real violation, not only a hypothetical one.

Together these establish something the prompt asks for directly: laws are not only things the design
*asserts*, they are things a mechanism can be asked to *certify*, at both the level of a single
numeral's identity and at the level of how two strategies resolve against each other, and neither needs a
forbidden feature to do it. What the canon should state, per `the-canon-is-intent-not-implementation.md`, is
the intent that this certification exists and is possible, pointing at this audit trail. It should not
reproduce the `assert!` pattern itself as the canonical mechanism; that is exactly the kind of concrete
implementation spelling the canon rule excludes. The probes are the establishment; the canon sentence is
something closer to "every algebraic law this document names is stated as a compile-time-checkable
obligation over the shapes it claims to hold for, and a law with no such obligation is not yet a law, only
a conjecture."

## Whether a law that holds only on a subset is a law

Yes, with a condition that the `UFixed<0, 8, Hot>::ONE` history already paid for once. A law restricted to
a subset of shapes is a real law exactly when the subset is *nameable and established*, not when it is
merely "true of the cases someone happened to test." The bug the memory records is precisely the failure of
this condition: `Identity` was asserted (implicitly, by shipping the impl at all) for every `(I, F)`
including `I = 0`, where no raw encoding of one exists, and 401 passing tests never named the boundary
because none of them tested at `I = 0`. The subset "shapes for which `Identity` holds" was real, it was
just never written down, so the impl claimed a larger domain than the law actually covered.

Probe 2 is the mechanical answer to this specific failure mode: the domain is not left implicit in which
tests happen to be written, it is the compile-time obligation itself, and an instantiation outside the
domain does not silently pass, it refuses to compile. A subset law is honest exactly when its boundary is
something the type system enforces rather than something a docstring claims and a test suite samples.

## What a consumer is entitled to conclude from a law it did not check

Exactly as much as the compile-time obligation that instantiation actually discharged, and no more. This is
a trust-boundary question, and the erase step in op's criterion is what defines where the boundary sits.
Once `ONE_RAW` (or any law-bearing associated item) has been validated at compile time and erased, a
consumer holding a value of `Fixed<3, 5>` has a guarantee that is exactly as strong as the assert that ran
during their own build, for their own instantiation. They are not entitled to conclude the law holds for
`Fixed<7, 5>` because it held for `Fixed<3, 5>`; nothing about the erasure carries information across
instantiations, by design, since erasure is precisely what removes the machinery that could have told them.

This has a consequence for what the canon owes a reader who is not the compiler: if a law's domain is
stated only as "the obligation that gets checked when you instantiate it," a human reading the canon (or a
consumer reading generated documentation) cannot tell, without attempting every instantiation, which shapes
the law actually covers. The generator being mechanically sound (Reading B) does not by itself make the
domain *legible* to a reader who is not running the compiler. If the canon wants both properties at once,
it needs the domain stated in closed form somewhere a reader can see it (an intent-level sentence: "the
multiplicative identity is representable exactly when `I >= 1`"), with the compile-time obligation as the
enforcement of that closed-form claim rather than as its only expression. That is two things, not one, and
a canon that only states the mechanism and never the closed form has bought machine-checkability at the
price of human legibility.

## How a law survives, or fails to survive, a chain, and why the answer depends on which operation is chained

This is where the cold pass produced the most surprising result, surprising enough that I want to walk the
correction rather than only the conclusion.

My first attempt at `77_probes/probe1_chain_error.py` chained fixed-point *addition*, expecting to show that
naive per-step rounding accumulates error with chain length while a widened accumulator caps it. The
addition experiment was uninformative on purpose left visible in the script: it showed zero error at every
chain length, for both strategies, because addition of two already-quantized fixed-point values is exact
whenever it does not overflow. The sum of two multiples of a quantum is itself an exact multiple of that
quantum; there is no rounding step to introduce error in the first place. Chaining addition needs no
chain-level machinery to preserve a law, because the per-step law was already exact.

Multiplication is where the widened-accumulator machinery earns its keep, because the exact product of two
`F`-fractional-bit values needs `2F` fractional bits to represent without loss, so a naive strategy that
rounds back to `F` bits after every multiply genuinely discards information at every step, while a strategy
that keeps the full-width intermediate and rounds once at the end does not. The rerun, in
`77_probes/probe1_chain_error.py`, holds the input value's one-time representation error fixed across both
strategies (both start from the same already-rounded input) and measures only what each strategy's
*rounding schedule* contributes on top of that:

| Chain length `n` | naive error, relative to one rounding step | wide (round once at the end) |
|---|---|---|
| 1 to 8 | 0.0 | 0.0 |
| 16 to 256 | 2.0, fixed | 0.0 |
| 512 to 4096, `v` close to 1 | fluctuates: 4.0, 0.0, 2.0, 4.0, 2.0, 2.0, then 130.0 at `n = 4096` | 0.0 for every `n`, by construction |

The wide strategy's error is exactly zero at every chain length, in every scenario, because it pays exactly
one rounding step, at the end, and nothing else. That single step is the same cost the naive strategy also
pays on its very first step; every additional step the naive strategy takes is an additional cost the wide
strategy does not pay. That part matched the hypothesis cleanly.

What did not match a clean hypothesis is the *shape* of the naive strategy's error growth. For a value less
than one raised to increasing powers (decaying toward zero), the error plateaus at a small fixed multiple
of one rounding step rather than growing without bound, because the value's own shrinking magnitude caps how
much error there is room for. For a value close to one raised to increasing powers (not decaying, an EMA- or
decay-rate-shaped workload), the error is not monotonic in chain length at all: it moves 0, 4, 0, 2, 4, 2,
2, and then jumps to 130 rounding steps at `n = 4096`, once the accumulated value's magnitude has shrunk
enough that a fixed absolute rounding step becomes a large relative error. This is the fixed-point analogue
of the well-known loss-of-significance behaviour near the bottom of a representable range, and it means "a
chain law degrades with length" is not the right general statement; the honest statement is "a chain law
degrades as the *represented value's own magnitude* approaches the quantum, which chain length only
correlates with when the chain itself is shrinking the magnitude."

The general lesson, stated at the level the prompt asks for: **whether a law needs chain-level machinery to
survive is a property of the operation being chained, not a property of "chaining" in the abstract**, and
even once it does need that machinery, the failure mode of *not* having it is not simple linear growth, it
is magnitude-dependent and can spike sharply near the edge of the representable range. A canon that states
"Precise threads a wider intermediate through a chain" without saying which operations actually need that
threading (addition typically does not; multiplication, and anything built from repeated multiplication,
typically does) is stating a mechanism without stating the domain the mechanism exists to cover, which is
the same gap the identity-law discussion above names for single operations, one level up.

There is a clean piece of established theory this connects to, worth naming because it generalises past
arvo's specific case: in denotational semantics, a relation between two interpretations that holds for a
single reduction step is not automatically compatible with composition; showing that an `n`-step relation
follows from a one-step relation is a separate lemma (a compatibility or "fundamental property of logical
relations" argument), proved per syntactic form, not inherited for free. Probe 1 is the numerical instance
of exactly that fact: the one-step law for `+` and the one-step law for `*` are both individually true, and
they are compatible with composition to entirely different degrees, for reasons that have nothing to do
with the strategy and everything to do with which algebraic structure the operator's rounding behaviour sits
inside (multiples of a quantum are closed under addition; they are not closed under multiplication without
widening).

## What happens to a law when the strategy changes what "correct" means

`I9` is stated plainly enough that it should be taken at face value rather than softened into "strategies
are different implementations of the same spec." If the strategy is one of the variables that decides what
the correct answer *is*, then a law is not "P holds" in some strategy-independent sense that Hot merely
approximates and Precise merely achieves exactly. Two different strategies can have two different laws that
are each fully correct relative to their own ideal, the way native-Rust-wrapping associativity and
exact-rational-ideal saturating associativity can both hold, exactly, while disagreeing with each other on
the same three input values.

`I5` sharpens this rather than complicating it: "Hot can sacrifice soundness, that is its explicit purpose,
but it should not lose it for nothing, instead, provable meaningful gains." Read together with `I8`'s
measurement-weighting framing, this suggests a law under Hot is not simply "absent" where it would hold
under Precise; it is replaced by a **paired claim**, a defect bound alongside a benefit bound, both
provable rather than asserted. "This law fails by at most `d`, and that purchases a gain of at least `g`
measured however Hot weighs gains" is a stronger and more honest canon sentence than either "the law holds"
(false for Hot in the cases where it deliberately does not) or "the law does not apply to Hot" (which throws
away the fact that op explicitly wants the tradeoff quantified, not waved away). This is a genuine design
fork worth naming rather than resolving here: does the defect/benefit pair live in the type, as an
inspectable associated const the way `ONE_RAW` does, so a consumer can query it without leaving the type
system, or does it live only in the canon's prose and the bench evidence that established it, with nothing
reified at the type level at all. The first buys legibility at the cost of carrying more machinery per
strategy; the second buys a cleaner erased boundary at the cost of the same legibility gap the previous
section already found for domain-of-validity claims. I do not think the premises settle which one op wants,
and I would not want to settle it here.

## Composition, and what a canon should state about it instead of what it cannot state

`arvo-toolbox-not-policer.md` already documents a real cross-strategy case: "Hot wrapping + Precise
saturating -> Precise" resolves to the more conservative side, with a compile-time warning rather than an
error. Probe 3 shows that the *resolution rule itself* wants the same law treatment as a numeral operation:
commutativity, associativity, and idempotence are not free, they are properties of whichever concrete
resolution function gets written, and a resolution function that is not a genuine join (the negative check
biased toward the left operand) fails commutativity in a way the same const-assert machinery catches.

What a canon should not attempt, because probe 1 already shows why it would be false in general, is a
meta-law of the shape "the composed operation inherits every law each operand individually satisfies." An
operator that is exact under Hot and exact under Precise, composed under the resolved (more conservative)
strategy, is not guaranteed to inherit exactness from either side; the resolved strategy is a third thing
with its own laws, which happen to have been chosen (per `arvo-toolbox-not-policer.md`'s "more conservative
side" framing) to be at least as strong as the weaker operand's, not to be the union of both operands' laws.
The honest meta-law, and the one probe 3's lattice framing actually supports, is monotonicity: the resolved
strategy's laws are never weaker than whichever operand's laws the resolution rule is defined to dominate,
stated and checked as a property of the join operator itself (probe 3's associativity check is exactly this,
one level abstracted), not assumed as a property of the numeral operations the resolved strategy then goes
on to perform.

## Three directions left open, not a settlement

Per the mode this panel is running under, I want to leave these as directions rather than resolve one of
them by fiat, because the premises available to a cold pass do not decide between them and pretending they
do would be the wrong kind of confidence for this stage.

**Per-operation lemmas plus a chain combinator, versus per-bounded-chain-length enumerated cases.** The
generator reading (Reading B above) argues for the first: state a small number of primitive lemmas (one-step
rounding-error bounds, closure properties like "sums of quantized values are exact") and a combinator that
threads them through an arbitrary-length chain, mirroring how a real widened-accumulator implementation
would work. The alternative, enumerating laws for chains up to some bounded length and leaving longer chains
as an explicit unresolved conjecture, is more honest about what has actually been established (probe 1 only
checked specific chain lengths, not an unbounded family) at the cost of never closing the general case. This
is exactly the doability question `the-canon-is-intent-not-implementation.md` asks: has the general
combinator actually been shown correct for arbitrary `n`, or has a finite family been checked and generalised
by pattern. My probe did the second, not the first, and a canon built only on what I actually established
should say so plainly rather than claim the stronger, unproven, general result.

**Whether the strategy space is a single ranking or a genuine multi-axis lattice.** Probe 3 deliberately
built the harder case, a two-axis product order, to show the mechanism is not secretly limited to a single
linear scale. That the mechanism *can* handle a lattice says nothing about whether arvo's strategies actually
form one. `I8`'s "they weigh different measurements differently" is compatible with either reading: it could
mean several genuinely independent axes (a lattice, where two strategies can be incomparable), or it could
mean a single axis with strategy-specific weights that still resolve to a total order in every case that
matters in practice, which `I8`'s own second sentence, "for the most part, they probably agree," leaves
open as a stated uncertainty rather than closes.

**Whether a strategy's defect/benefit pair is a type-level fact or a canon-and-bench-level fact.** Named
above, in the `I9`/`I5` discussion, and repeated here because it is the fork that most directly touches
op's own acceptance criterion: "validate, then erase" is unambiguous about numeral representation
(`ONE_RAW` is checked, then gone), and it is genuinely ambiguous about whether a *quantified tradeoff claim*
should survive erasure in some queryable form or should be exactly as erased as everything else. Both
readings are consistent with everything I read under phase one; nothing in `INTENTS.md` forces one over the
other.

## What op's intents do not license, that I want flagged plainly

Two things, both small, both worth saying rather than silently absorbing.

First, the provenance gap noted at the top: the exact sentence "derive the matching container and numeral
representations, then validate, and erase" was given to me by the dispatcher and I have not personally
traced it to an op quotation inside `INTENTS.md`. It reads consistently with `I2`, `I3`, and `I9`, but
consistent-with is not the same claim as sourced-to, and this file should not be read as having verified
that quote against a primary source.

Second, and more structural: arvo's own `.claude/CLAUDE.md` is explicit that `#![no_std]`, no `alloc`, const
sizes, and "no `dyn`, `TypeId`, `std::any`" are **not** among op's ratified intents, they are long-standing
agent-authored discipline. None of the three probes in this file depend on `dyn`, `TypeId`, or
specialization existing or not existing, so nothing here rests on that discipline being settled, but I want
to be precise rather than silently borrow the authority of a discipline that arvo's own rules say is not
op's word to begin with. Where this file cites `no_std`-style constraints (for instance, in ruling out
`generic_const_exprs` as the fix in probe 2), it does so on the workspace's forbidden-feature list, which is
a standing rule independent of whether `no_std` itself is ratified, not on the ratified-intent catalogue.

## Phase two: reconciliation

**Coverage.** After committing phase one, I read `76_willsey_derived_laws_derived_cold.md` in full
(the parallel cold derivation named in the dispatch), `42_willsey_the_law_layer.md` in full (the
earlier, non-cold, exhaustively-probed file specifically on this topic), `61_absorption_against_coherence.md`
section 1 and its opening apparatus (adjacent topic, format-concept unit, not this unit), and grepped
`OPTIONS.md` and `DROPLIST.md` for `resolve`/`resolution`/`cross-strategy`, reading the surrounding
context at `OPTIONS.md` Q27 and `DROPLIST.md` lines 85 to 100. I did not read `35`, `40`, or `18`
directly; everything I say about them below is sourced through `42`'s citations of them, and I mark it
as such rather than pretending to a firsthand read I did not do. That is a real risk: if `42`'s account
of `35`/`40`/`18` is wrong anywhere, this section inherits the error, the same caveat `42` itself states
about its own reliance on `35`'s and `40`'s paraphrase of files it did not open.

### Against `76`: one instance meeting another, not confirmation

`76` and this file were dispatched cold in parallel on the same question, and per the protocol I treat
agreement between us as one independent instance meeting another rather than as corroboration of a
single claim. On that basis, several things converged without either of us reading the other:

Both files land on the same answer to "type, operation, pair, or context": a law belongs to a strategy's
semantics composed with a set of operations, not to a type or an operation alone. I called this a
judgment between chains under a strategy, drawing on logical relations; `76` called it a congruence
closed under the strategy's exposed operations, drawing on equality saturation. These are two different
formal traditions (denotational semantics and term rewriting) landing on the same structural object, a
relation compatible with composition, which is worth more as a data point than either framing alone,
precisely because neither of us was importing the other's vocabulary.

Both files independently reach for "derive, validate, erase" as a mechanism for a law and not only for a
container, using the identical concrete example (`UFixed<0, F>::ONE`, or `Fixed<0, F>` in my own probe's
naming) and the identical mechanism, a `const fn` body with an internal `assert!`, refusing at
compile time with no forbidden feature. `76`'s probe 2 goes one step further than mine: it inspected the
emitted assembly for a validated call site (`mov w0, #16 ; ret` on aarch64) and confirmed nothing about
the check survives, where I only confirmed the program compiled and ran. That is a real gap in my own
phase one, now closed by an independent instance rather than by my own further work, and I want to be
explicit that `76`'s codegen check is strictly stronger evidence for the erase half of the claim than
anything I built.

Both files independently propose the same two-reading structure for "derived" (mine: proof versus
generation; `76`'s: entailment versus per-instance generation) and independently converge on the same
subordination claim: the generative reading is only sound because the entailment reading holds
underneath it, an unentailed "derivation" per instance is an unverified fact wearing a computed one's
clothes. Two independent derivations of the same subordination claim, on the same day, from the same
premises, without either reading the other, is the kind of corroboration the panel's own bar
(`RULES.md`, cited by `42` at lines 116-118 and by `76` implicitly) asks for and rarely gets handed for
free.

`76` goes further than I did on one front I had not attempted: an exhaustive (not sampled) check of
which small laws hold under wrapping versus saturating addition over the entire `u8` and `i8` domains.
The finding that unsigned saturating addition is universally associative, against `76`'s own stated
working assumption that saturation breaks associativity, is a genuinely useful correction to a piece of
folk intuition I was carrying too (I had not stated it in phase one, but I would have been wrong to
state it if asked). `42` (below) explains the mechanism behind exactly this asymmetry, which neither
`76` nor I had access to during our cold passes, and which sharpens both of our findings considerably.

### Against `42`: the file this unit should have been dispatched to read, and largely was not

`42` predates this unit by a wide margin and did exhaustive, multiply-corroborated work directly on this
question, under names (`35`, `40`, `18`) I had no access to during phase one and only partial access to
now. Several of my phase-one claims survive contact with it; several are subsumed by something sharper;
one needs an explicit retraction of emphasis rather than of substance.

**What survives, sharpened.** My "closure" observation in the chain-error section, that fixed-point
addition needs no widening because sums of already-quantized values are exact while products do, is the
same fact `42` states more generally and more usefully at section 3.3: "closure (the operation does not
widen) is not a law in the algebraic sense; it is the precondition every other law is stated relative
to," backed by `35_probes/p1`'s compiled refusal of four independent formulations of a widening fold. I
arrived at the specific instance; `42` states the general principle it is an instance of, and I would
defer to that framing over my own were this file being rewritten rather than appended to.

**What is subsumed and sharper than what I had.** My subset-law discussion (a law restricted to `I >=
1` is a real law exactly when the restriction is type-enforced) is correct as far as it goes and is
considerably less sharp than `42` section 5.2's reachability mechanism. `42` shows, by refuting its own
first hypothesis on the record rather than hiding it, that associativity of a clamped operation does not
depend on how many clamps are coded, it depends on how many of them a *specific fold's declared operand
range* can reach: an unsigned saturating add has two clamps coded and one reachable, because the
non-negative domain structurally cannot trigger the floor, and that is the entire mechanism behind the
asymmetry `76` measured independently and I did not test at all. My own chain-error probe found a
related but distinct fact (multiplication chains need a widened accumulator because the exact product
does not fit the narrower container, a closure argument, not a reachability argument), and I want to be
precise that these are two different mechanisms producing superficially similar-looking "chains behave
worse than single ops" findings. `42`'s reachability condition is a fact about which boundary values a
computation's trajectory can touch; mine is a fact about whether an operation's exact result fits its
own operand type. A canon that named only one of these two mechanisms under a single heading
("chain-level laws need extra care") would be flattening two genuinely different reasons into one
folk generalisation, and I did not see that risk clearly enough in phase one.

**Where phase one needs a correction of emphasis, not of substance.** I framed "derivation as generation"
(my Reading B) around a single mechanism, the `const fn` plus `assert!` pattern, without distinguishing
it from the alternative `42` and its citation of `35`/`40` (Q9) already established: a trait-relation
bound on the axis *value*'s own marker traits (`S::Overflow: AbsorbingTop + MonotoneAdd`, stated on
`Saturate` or `Wrap` directly, not on a preset name), rather than a post-monomorphisation const
assertion. `42` reports, citing `35` section 3, that the trait-relation form composes upward through a
generic call site cleanly while the const-assertion form does not, and `42`'s own probe 2 demonstrates
the composed case directly: a fold's accumulator-width bound and a fold's law bound refuse
independently, at the same call site, entirely in the trait-relation form. My probe 2 (the identity
example) and probe 3 (the resolution-lattice example) both leaned on the const-assertion form because
that is the shape that gives the best local diagnostic, which is real and is exactly what `42` reports
Q9 already settled it is good for. But I did not build, or even attempt, the trait-relation form for
either claim, and a reader taking my phase one alone would come away thinking the const-assertion
mechanism is *the* answer to "derived, mechanically," when the panel's own established answer is that
both forms are needed for different jobs and neither subsumes the other. I would want any consolidation
drawing on this file to state both mechanisms, not the one I happened to reach for.

**A genuine safety gap I did not know to look for, and now think belongs in the derived-laws unit
directly.** `42` section 6.1 makes an argument I had no way to reach cold, because it depends on reading
`arvo-always-optimal-internals.md` together with a measurement I had not seen: that rule licenses arvo's
own internals to perform algebraic rewrites (a fused kernel exploiting distributivity, for instance)
freely, and `35` measured that a distributivity-licensed rewrite changes the answer at up to 70.5 percent
of triples once the fractional width is nonzero. I cited `arvo-always-optimal-internals.md` in my own
phase-one file as settled background discipline, in the section on the two ideals a law can approximate,
without noticing that the rule as written has an unstated soundness precondition that the law layer this
unit is discussing is precisely what would close. I want to flag this plainly rather than let it pass:
the derived-laws unit is not only answering an abstract question about what a law is, it is closing a
concrete gap in an already-ratified-by-workspace-discipline (if not by op) rule that currently licenses a
class of internal rewrite without checking whether the axis assignment it runs under actually carries
the property the rewrite depends on.

**Where I would push back, gently, on `42`'s framing.** Section 6.4 refuses, correctly and for reasons I
agree with (`arvo-toolbox-not-policer.md`'s facts-versus-decisions line), a rewriting or
equality-saturation *engine* living inside arvo. I want to be careful that my own "generator of laws"
language from phase one is not read as proposing exactly that engine, because on a plain reading it
could be. What I meant, and what I think `76`'s "reading two" also meant, is a generator of *facts*, a
procedure that answers "does this shape, strategy, and operation satisfy this named property" for a
given instantiation, which is precisely what probe 2 and probe 3 both build and precisely what `42`'s
canon-shaped sentence in section 6.4 already licenses ("the design states which algebraic properties...
in a form a downstream consumer can require and have checked"). It is not a procedure that decides
whether to *apply* a rewrite using those facts, and I did not build anything that decides that, but the
word "generator" is close enough to "engine" that I want the distinction stated explicitly rather than
left for a later reader to assume the more dangerous reading. `42`'s sentence is the better one to carry
forward; mine needed this correction to mean the same thing.

**One thing `42` and `76` both establish that changes how I would state my own "what does a consumer get
from an unchecked law" section.** I framed this as a trust-boundary question with a design fork
(defect/benefit as a queryable type-level fact, or as prose-and-bench-only). `76`'s congruence-closure
framing sharpens the failure mode: an unsound law does not fail loudly, it silently merges two
computations that were not equal, and the failure surfaces later with no local trace back to which law
produced it. `42` section 5 gives the concrete, measured version of exactly this: `18` found an
absorbing-top reading sound "exactly while the computation stays at it," failing at 936 of 5184 chains
the instant an operation moved off the endpoint, and `35`'s `p9` found the same shape for a different
pair of properties (absorption without monotonicity, 12.6 percent of shortest paths wrong). Both of
these are precise, quantified, silent-failure-until-you-look results that make the trust-boundary
question I raised in the abstract into a question with actual numbers attached, which is exactly the
kind of grounding my own phase one lacked.

### What I found that neither `76` nor `42` addresses, and is not a duplicate

My probe 3, the strategy-resolution product lattice, checks laws about how *two different strategies'*
axis assignments resolve when a single expression composes operands under both of them (the
`arvo-toolbox-not-policer.md` "Hot wrapping + Precise saturating -> Precise" case). `42`'s vocabulary is
about which properties a single axis *value* carries within one strategy's fold; it does not address what
happens when two different strategies meet in one expression. I checked this deliberately, before
reading the panel, and after reading it I found `OPTIONS.md` Q27 states plainly: "Which strategy's laws
govern a cross-strategy operation is adjacent to that and not the same question," and that `66` found
nothing in `63`, `64`, or `65` addressing it directly. I have not read `63` through `66` in full (they are
the number-systems unit, a different topic), so I cannot say with certainty that nothing anywhere in the
panel has touched this, only that Q27's own author, working from the number-systems side, reports the gap
as open on that side too. Probe 3's contribution, then, is a first mechanically-checked instance on a
question the panel's own register already flags as unaddressed, not a duplicate of `42`'s per-axis-value
work: commutativity, idempotence, and associativity of the resolve operator itself, checked over a
genuine two-axis product order rather than a single ranking, with a negative control proving the check
bites when the resolve rule is not a true join. I would want this named as a candidate item for
`OPTIONS.md` Q27 rather than folded silently into the law-layer vocabulary `42` already built, because it
answers a different question that vocabulary was not built to answer.

### What I would now say differently if I were rewriting phase one rather than appending to it

I would not claim the two-ideal framing (mathematical-reals ideal versus native-Rust-semantics ideal) is
mine alone; `42`'s group-theoretic collapse of wrapping addition into "addition in the cyclic group
Z/2^W" is exactly what makes the Warm ideal precise and checkable rather than merely gestured at, and I
would cite it directly rather than leaving Warm's ideal as an unstructured "whatever native Rust does." I
would also narrow my "chain error is not monotone, it spikes" finding to name it correctly as a closure
phenomenon (the operation's exact result outgrowing its own container) rather than let it read as
adjacent to `42`'s reachability phenomenon (a clamp being reachable or not by a fold's declared range),
since a reader who conflated the two would be conflating two mechanisms this unit's own record now shows
are genuinely distinct.

### What I would keep exactly as stated

The three open directions in phase one (per-operation lemmas plus a combinator versus bounded-length
enumeration; single ranking versus multi-axis lattice for the strategy space; type-level versus
canon-and-bench-level defect claims) survive contact with both `42` and `76` unresolved, and I think they
should stay open rather than be resolved by this reconciliation. `42`'s section 5.3 gives the second and
third of these a concrete grounding I did not have: the "exchange rate" `34` and `38` leave unset for Hot
and Warm respectively is, per `40`'s reading which `42` adopts, one unset quantity rather than two, and
every failure percentage `35`, `18`, and `42` itself measured (70.1, 12.6, 82.7, 70.5, and others) is a
candidate input to that rate once op sets it. This does not resolve my third open direction; it makes
clear the direction is not hypothetical, the panel already has the numbers such a decision would be made
against and is missing only the rate.

### What op's intents still do not license, restated after reading the panel

Both flags from phase one stand. On the first, the provenance of the dispatcher's acceptance-criterion
quote, neither `42` nor `76` traces it to a file:line either; `42` cites op's nine files by number
(`01`, `04`, `28`, `32`, `34`, `36`, `37`, `38`, `39`) as its fixed set and does not quote the
derive/validate/erase sentence verbatim from any of them. I still have not verified it myself. On the
second, `76` independently reports the identical finding about `no_std`/`dyn`/`TypeId` not appearing in
`INTENTS.md`, from a completely independent read, which is itself a small additional instance of the
same finding rather than a new one.
