## 76. Willsey, derived algebraic laws (cold derivation, phase one)

**Protocol note, read before the rest.** This file is phase one of a two-phase cold derivation. I read
only the premises named in the dispatch: `INTENTS.md`, the workspace and arvo rules under
`.claude/rules/`, and op's acceptance criterion for the typestate ("have the typestate derive the
matching container and numeral representations, then validate, and erase"). I did not read this panel's
prior files, `OPTIONS.md`, the droplist, the archaeology files, any probe here, any git log, or any
commit message in this directory. I built and ran my own evidence, committed in `76_probes/`, before
writing anything below. Phase two is a separate section appended after that commit and after reading the
panel; it does not touch this section.

**A disclosure the protocol asks me to make honestly rather than pretend around.** Listing the panel
directory to confirm the dispatch path exists showed me the filenames it contains, including
`42_willsey_the_law_layer.md`. I did not open it and nothing below cites it, but I know a prior pass of
mine already worked this ground, and a reader is owed that fact rather than a fiction of total isolation.
Separately, my session carries auto-loaded background memory about arvo from prior work, including a
specific incident (`UFixed<0, 8, Hot>::ONE` holding raw `0`, a fixed-point format with no representable
one, with 401 passing tests that never caught it because they sampled the width matrix rather than
covering it), and the framing that fixed-point and float are one formalisation (`Flocq`'s
`generic_format` covering both). That memory was not something I chose to read for this dispatch; it sits
in ambient context I cannot unsee. I have kept it out of the reasoning below wherever a fresh derivation
was available, and I flag every place it plausibly shaped a conclusion rather than let it pass as pure
cold work. Treat those flagged spots at reduced weight; the probes stand on their own regardless of where
the hypothesis that motivated them came from.

### The question, restated in the terms the premises actually give it

Op's acceptance criterion names three verbs for a typestate: **derive** the container and the numeral
representation, **validate**, **erase**. It says this about representations, not explicitly about laws.
The dispatch asks me to extend it: does a law owe the same three verbs, and if so, what does each one mean
when the object is not a container width but a claimed algebraic fact?

I9 is the premise that forces the extension rather than leaving it optional. *"The strategies aren't
orthogonal to the threaded question you had, or its answer, strategies are the variables that change what
the 'correct' answer is for what we choose as the path."* If the strategy changes what counts as correct,
then a law (a general statement of what output is correct given certain inputs) cannot be stated once for
a format and inherited by every strategy that instantiates it. Its truth is a function of the strategy,
the same way the container's width is a function of the strategy. That is the exact shape of "derive": a
law's applicable form, or its very holding, is *computed from* the typestate rather than *declared once
and reused*.

### What I derived and then checked, not assumed

I did not trust my own intuition about which laws survive which strategy. I built three independent,
exhaustive (never sampled) checks and let them answer. Full sources, compiler output, and run logs are
committed in `76_probes/`.

**Probe 1 and 1b: does a strategy's chosen arithmetic semantics change which laws are theorems, holding
the container fixed?** Checked exhaustively over the whole `u8` domain (`16,777,216` triples per law, not
a sample): `wrapping_add` is associative universally (it is arithmetic in `Z/256Z`, a ring, and the ring
argument is exactly why). `saturating_add` on an **unsigned** domain also turned out to be universally
associative, which contradicted my working assumption going in and is the more interesting finding for
that reason; the naive belief that "saturating breaks associativity" as a blanket claim about saturation is
false. It does break on `i8` (signed, clamped on both ends): `4,177,792` of `16,777,216` triples fail, first
counterexample `a=-128, b=-128, c=1`. And a law closer to what a consumer actually leans on when chaining a
gain and a loss, `(a+b)-c == a+(b-c)`, fails for `82.7%` of the `u8` domain under saturation while holding
universally under wrapping. Three separate laws, three separate verdicts, none of them predictable from the
strategy's name alone and all of them settled only by exhaustive computation.

**Probe 2 and 2b: can "derive, validate, erase" be extended past a container width to a claimed algebraic
fact, under the pinned nightly, without any forbidden feature?** I built a fixed-point format
`(I, F)` (integer bits, fractional bits) and derived, rather than declared, the raw encoding of the
multiplicative identity: `raw_one = 1 << F`, valid only when `I >= 1`. The validation is a `const {
assert!(I >= 1, "...") }` block inside a `const fn`, evaluated once per monomorphisation actually reached,
using only stable inline const blocks and const generics, no `generic_const_exprs`. For `I >= 1` it
compiles and runs (probe2). For `I = 0`, the format whose whole domain is `[0, 1)`, it refuses to compile
with a diagnostic naming exactly why (probe2b): `evaluation panicked: format has no integer bit: the
value 1 is outside [0, 1), so ONE cannot be derived for this format`. This is the reddest and most useful
kind of result this workspace's own instrument names: a contract test that will not compile says the
claim has no expressible form for this instantiation, not merely a wrong value. I then checked the
"erase" verb was not a hope: the emitted assembly for a validated call site (`get_one_4_4`, aarch64) is
`mov w0, #16 ; ret`, nothing else, no branch, no panic machinery, no trace that a check ever ran.

**Probe 3a and 3b: does the same law, tied to a strategy marker rather than to a runtime flag, hold or
refuse to compile as a compile-time fact rather than a philosophical one?** I re-derived the associativity
finding from probe1b through a completely different mechanism: const evaluation rather than execution, on
a narrow domain (`16` values, `4,096` triples, small enough for const-eval to finish without hitting the
long-running-const-eval wall this workspace's own unstable-features vetting already documented at wider
widths). Wrapping's version of the law is a top-level `const` item that evaluates clean at compile time
(probe3a). Saturating's version of the identical claim, over the identical domain, is a top-level `const`
item that refuses to compile (probe3b), with the compiler naming the failed assertion directly. Two
independent mechanisms (runtime execution at full width, const evaluation at narrow width) agree on the
same fact through different means, which is stronger corroboration than either alone.

Three independent instances, all committed, all reproducible by anyone who runs `rustc` against the
pinned nightly on the files in `76_probes/`.

### What "derived" means for a law, held open rather than settled

The premises do not force one reading and I will not manufacture a false convergence. Two readings
survive scrutiny and they are not equivalent.

**Reading one, entailment.** A law is derived if it is a theorem of the format's own denotation rule
(what a raw bit pattern *means* as a value) composed with the strategy's chosen arithmetic semantics,
rather than an independently asserted fact about one specific instantiation. Probe 2's `derive_raw_one`
is this reading in miniature: "one" is not looked up, it is computed from `(I, F)`, and where the
computation has no answer, the type has no one. Under this reading, a law is a *consequence*, and stating
it as an axiom anywhere in the design is a category error, because axioms are exactly the things that can
silently stop matching what they were supposed to describe.

**Reading two, per-instance generation, mechanical.** This is the closer reading of op's actual words:
the typestate *derives* the container, meaning it computes a concrete answer from generic parameters,
mirroring how a proc-macro `#[derive(...)]` generates code from a declaration. Under this reading, "derive
a law" means: for each `(format, strategy, width)` instantiation, generate (or select) which laws hold and
in what exact form, as a typestate-indexed computation, the same way the container type itself is
generated.

I do not think these compete; I think reading two is only sound *because* reading one is true underneath
it. If a law were not entailed by the format and strategy's own semantics (reading one false), then
"deriving" it per instance (reading two) would just be dressing up an independently declared, unverified
fact as a computed one, exactly the failure the probe 2/2b pair exists to catch. So the canon's obligation,
if it takes a position here, is to state reading one as the requirement (every claimed law is a theorem,
not a postulate) and reading two as the mechanism (the typestate is what computes, per instantiation,
which theorems apply and in what form). I hold this as the stronger candidate, not as settled; a second
reading in which the canon states only the mechanism and leaves entailment to per-strategy proof
obligations is also defensible and I have not ruled it out.

### The congruence-closure reading, offered because it is the one I am least likely to be wrong about

A law of the shape "`x OP c == x`" or "`(a OP b) OP c == a OP (b OP c)`" is, structurally, a rewrite rule.
The moment a consumer relies on it (substitutes one expression for another because the law says they are
equal), that consumer is running the substitution step of equality saturation by hand, whether or not
anything in arvo's design uses that name. The entire discipline of congruence closure exists because that
substitution is only sound when the rule is actually true for the specific terms it is applied to, and an
unsound rule does not fail loudly. It merges two things that were not equal, and every later extraction
built on that merged class inherits the error with no local signal that anything went wrong. Probe 2's
class of bug (an independently stated "one" that is not actually the format's one) is exactly this failure
mode, reconstructed from the format's own denotation rule rather than from memory of the specific
historical incident. A law that a consumer is entitled to rely on and a rewrite rule an e-graph is allowed
to apply are the same kind of object, and they carry the same soundness boundary: never trust it until it
is checked against the thing it claims to describe, over the whole domain it claims to cover.

This gives a direct answer to "what does a law's failure mean for a consumer who did not check it": it
means the consumer has silently merged two computations that a stated law claimed were the same and
were not. Nothing at the failure site says so. The failure surfaces later, downstream, in whatever decision
was made using the wrongly-substituted value, and by then there is no local trace back to which
"law" produced it. This is not a hypothetical category of bug for this workspace; the whole reason exhaustive
checking rather than sampling is already a standing rule here (`catalogue-edge-cases-as-tests.md`,
`a-test-that-cannot-compile-is-the-finding.md`) is that sampled coverage produces exactly this failure
shape and calls it a passing suite.

### Which laws a consumer is entitled to rely on, and which are convenience

Not every true statement about a type deserves the same trust. I would split what the canon calls a "law"
into three tiers, and the split follows directly from I5, I7, and I9 rather than from a general theory of
what a law is:

**Entailments.** Statements that are theorems of the format's denotation rule and the strategy's declared
semantics, checked over the whole domain they claim to cover (whole domain at a width small enough for
const-eval, or exhaustive runtime execution at a width too wide for that, per probe 1/1b vs probe 3a/3b).
These are what a consumer is entitled to build on without re-checking, because the VALIDATE step already
discharged them at the point the type was chosen.

**Convenience facts.** Statements that are true only under an unstated precondition (no overflow, a
narrower sub-range than the type's full domain) and are not themselves tied to that precondition anywhere
a compiler or a reader can see. These are exactly the trap: they read as laws, they are laws in the cases
someone happened to test, and they stop being true silently outside those cases. I7's insistence that
Precise be accurate "especially within chains and ops, not only alone" reads, under this framing, as a
direct rejection of convenience facts for that strategy: a fact that is only true per-operation and not
across a chain is a convenience fact wearing a law's clothing, and Precise's whole reason to exist (per
I7) is to refuse that substitution wherever chain-level correctness is the point.

**Explicitly refused laws.** I5 licenses Hot to give up a law "not for nothing, but for provable
meaningful gain." This is the one tier where the canon should not require entailment; it should require
the opposite, a stated refusal with the gain that justified it. A law Hot has explicitly given up is not
a bug in Hot, and treating it as an entailment failure would misread I5. The failure mode to avoid is a
refused law that is not written down as refused: then a consumer cannot tell whether Hot never had the
law or the law was never checked, and those are very different situations for a consumer to be in.

### Whether a law belongs to a type, an operation, a pair, or a context

The probes answer this more sharply than intuition would have. Associativity of `+` is not a fact about
`u8`, and it is not a fact about `wrapping_add` alone (`wrapping_add` composed with itself associates;
composed with `saturating_sub`, the mixed rearrangement law fails at 82.7% of the domain even though each
individual operation, alone, behaves exactly as documented). So a law belongs to **the set of operations
composed under a fixed strategy**, not to any single type or any single operation. This is the same object
an e-graph tracks: a congruence closed under a specific set of operations. Two operations that individually
preserve every law they are separately claimed to satisfy can still fail a law about their composition, and
the only way to know is to check the composition, not the parts.

This bears directly on I9 read together with I7: if "the strategy is what makes an answer correct," then
the natural unit for a law is `(strategy, {operations composed})`, and the canon's job is to state that
the unit is this pair, not to enumerate every law for every pair (that enumeration is design and
implementation work, per `the-canon-is-intent-not-implementation.md`, and belongs downstream of the
canon).

### Under composition, widening, and chains: not a lattice, not quite a ladder, closest to a strategy-indexed congruence

The dispatch's question offers three shapes: lattice, ladder, unordered set. None of the three is the
answer once the probes are in hand, though one candidate synthesis, offered at low confidence and as one
option among others rather than as a finding, comes closer than the other two.

For a **fixed** strategy, the set of pairs "these two expressions are provably equal under this strategy's
semantics" is closed under the operations that strategy exposes: if `a ~ b` and the strategy defines `f`,
then `f(a) ~ f(b)` follows, or it does not, and either way that follows from the strategy's own semantics
rather than needing a separate rule per pair. That is a congruence, not a lattice and not a ladder; it is
the object my own field spends its effort maintaining correctly (an e-graph's rebuild-after-merge
discipline exists entirely to keep this closure honest as new equalities are discovered).

**Across** strategies, I8 offers something closer to an order, and I want to be careful to attribute it
correctly: I8 says the strategies "weigh different measurements differently" and that "for the most part,
they probably agree, because in general, the best answer fits all," while explicitly marking the second
half as his own possibly-wrong instinct rather than a settled claim. Reading I7 and I5 alongside I8, one
candidate shape is: Precise's congruence is, in the cases where the underlying arithmetic actually agrees,
a refinement of (at least as strong as) Hot's, because Precise is defined to preserve chain-level facts
Hot is explicitly licensed to give up. That would make "how many chain-level laws are honored" a genuine
partial order with Precise at the top and Hot's honored set a subset of it wherever they overlap. I want to
be explicit that **this ordering claim is my own synthesis, not stated anywhere in the premises**, and I
have not checked it exhaustively the way I checked the associativity claims; it is offered as a candidate
the panel might want to test the same way I tested the associativity claims, not as a result.

Warm's position under this candidate order is set by I3 and I4 rather than derived independently: Warm's
congruence is "whatever native Rust primitives honor," because I3 states the imitation intent directly and
I4 states that imitation is not absolute where it is "consistently just a worse choice." That is itself a
kind of law: Warm's law-set is not independently authored, it is *derived from* Rust's own primitive
semantics, with an explicit, stated escape hatch. Cold's position is likewise not independent: I6 says Cold
"can use the same paths Hot uses… but if the path fights the intent, then it's not for Cold," which reads
as Cold *inheriting* whichever congruence its borrowed compute path carries, never authoring its own. Under
this reading Cold has no independent law-set to state in the canon at all; it has a storage-density
intent and a borrowing rule, and its laws are exactly whatever laws the path it borrowed already has.

### Laws that hold only on a subset

These are laws, and refusing to call them laws is worse than accepting them, provided the subset is
**visible to the type**, not merely true in practice for the inputs someone happened to try. Probe 2/2b is
the sharpest version of this: "multiplication by ONE is the identity" is a law that holds only where `I >=
1`, and the moment that restriction is enforced by the typestate itself (the `const` block, checked per
instantiation) rather than by a comment or a convention, the restricted law is exactly as trustworthy as
an unrestricted one, because a consumer cannot reach the untrue case without the compiler refusing them
first. A conditional law with no type-level enforcement of its condition is a convenience fact by another
name, and belongs in the middle tier above, not the top one.

### What the canon states

Consistent with `the-canon-is-intent-not-implementation.md`, which this workspace already commits to and
which I take as binding on how a canon may be written rather than as a claim about arvo specifically: the
canon should state the derivation mechanism and the discipline around it, not a table of which concrete
laws hold for which concrete strategy. A table like that is design, possibly implementation, and it goes
stale the moment a strategy's chosen semantics changes, which the canon is specifically supposed to
survive. What the canon can state, and what I believe the probes establish as doable rather than merely
wished:

A claimed algebraic fact about a numeral is a theorem of that numeral's denotation rule composed with its
strategy's semantics, never an independently asserted fact about one instantiation. Every such theorem is
checked over the whole domain it claims to cover, not a sample of it, at the point the typestate is
selected, using the mechanism op names: derive the concrete claim from the typestate, validate it
exhaustively, and let the check erase to nothing once it has passed. Where a strategy is explicitly
licensed to give up a law (I5), the canon should require that the refusal be stated, with the gain that
justified it, rather than left to be discovered by absence. Where a law holds only on a subset of a type's
domain, the subset is enforced by the type, not documented around it.

### What the premises do not license, said plainly

Nothing in I1 through I12 states a compile-time-validation requirement, an entailment-over-declaration
requirement, or the ordering candidate I offered above. I have built each of those from the acceptance
criterion, from I5/I7/I9 read together, and from the probes; none of them is op's word and I have tried to
mark each one as mine rather than let it read as ratified. I1 also means I must flag that nothing above
should be read as presuming exactly four strategies or their current names; every argument here is stated
in terms of what a strategy licenses (I5, I6, I7) rather than in terms of `Hot`/`Warm`/`Cold`/`Precise` as
fixed identities, and it should survive the set being reshaped.

One thing outside the assigned question, reported because the standing instruction asks for it regardless
of scope: the workspace's own `arvo/.claude/CLAUDE.md` and several generated rule files still assert
`#![no_std]`, no `alloc`, no `dyn`, no `TypeId`, "sizes are const" as long-standing constraints, while
explicitly noting these do not appear in `INTENTS.md` and are agent-authored rather than op-ratified. My
probes never needed `alloc`, `dyn`, or `TypeId` to establish any of the above, which is at least consistent
with those constraints being compatible with a laws-as-entailments design; it is not evidence that op has
ratified them, and I have kept that distinction visible everywhere I depended on the const-generic,
const-fn, no-runtime-dispatch style those constraints already assume.
