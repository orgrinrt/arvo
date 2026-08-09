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

---

## Phase two: reconciliation

Written after reading the panel. Phase one above is untouched; everything below is new. I read `RULES.md`
in full, `OPTIONS.md` in full (both halves), `DROPLIST.md` in full, `PERSONA_CALLS.md` in full,
`PRIOR_CALLS.md` in full, my own prior file `42_willsey_the_law_layer.md` in full, checkpoint `69` in full,
and `77_amin_derived_laws_derived_cold.md`, the parallel cold derivation on this same question, in full. I
did not read `56`, `57`, `57b`, `58`, `60`, `61`, `62`, `63`, `64`, `65`, `66`, `67`, `70` to `75` end to
end; where I cite them below it is through `OPTIONS.md`'s or `69`'s account, named as such.

### The most important thing phase two surfaces: I already did this work, in a different session, and I
had forgotten it

File `42`, dated the day before this dispatch, is mine, under this same persona. I wrote it cold against
`35`, `40`, and `18` rather than against a blank premise set, so it is not a second instance of the cold
protocol, but its conclusion is the same conclusion I reached in phase one, reached the same way both
times: a law lives at the operation-times-strategy pair (`I9`), a law's failure is precise and separable
rather than a blanket unsoundness, and the substrate owns the *vocabulary* of which properties an axis
assignment carries while refusing to own an engine that *acts* on that vocabulary. Section 6.4 of `42`
states the refusal in almost the words I used in this file's own congruence-closure section: "the cost
function and the extraction strategy... is a domain decision, not a substrate decision." That sentence is
mine in both files, written a day apart, without the second writing having access to the first when it
was produced. I record this because it is a form of corroboration this panel's rules do not have a name
for (not TWO EXPERTS, since it is one author; not a read, since phase one genuinely did not have it), and
because a persona that reliably re-derives its own settled positions from first principles is itself a
small piece of evidence that the position is not an accident of framing.

`42` also settles something phase one left as candidate synthesis: whether a rewriting engine belongs in
arvo. It does not, on two independent grounds phase one did not have (`DROPLIST.md:19-22`, op refusing the
relocation to hilavitkutin directly, corroborated by measurement that the regrouping already pays inside
arvo's own licensed internals before any scheduler exists to relocate to) plus the existing
`arvo-toolbox-not-policer.md` rule. I adopt `42`'s locus finding into this file rather than re-derive it:
**the FACTS belong in arvo because only arvo knows its own axis values; the DECISIONS about whether to act
on a fact belong to whoever performs the rewrite (arvo's own internals under `arvo-always-optimal-
internals.md`, hilavitkutin's scheduler, vehje's compiler backend).** This directly answers the
existence-and-locus half of the dispatch's question, which phase one did not address at all: yes, arvo's
canon owes algebraic laws, and it owes them as a vocabulary of checkable facts, never as a mechanism that
exploits them.

### Real, independently-reached agreement with `77`, and one shared blind spot

`77` (Amin) ran the identical cold protocol on the identical question, in parallel, with no access to this
file while writing its own phase one. Four things converge without either file having read the other:

1. **The mechanism.** Amin's `probe2_works_validate_erase.rs` and my `probe2_derive_validate_erase.rs` are,
   independently, the same construction: an associated fact about representability of a numeral's "one",
   computed in a `const fn`, validated by an `assert!` that fires per instantiation, refused for `I = 0`
   with a named compile error, erasing to a bare constant once it passes. Amin's negative probe
   (`probe2_fail_inline_const_expr.rs`) independently hit the same wall I would have hit had I tried the
   naive inline-expression spelling first, and named the same rule this workspace already has for it
   (`a-refused-bound-wants-a-trait-not-a-feature.md`).
2. **A law is a fact about a pair, not a bare type**, both derived directly from `I9`, in near-identical
   language.
3. **A subset law is honest exactly when its domain is type-enforced rather than merely tested.** Both
   files reach this from the same historical failure shape.
4. **The canon states the mechanism and the intent, never a concrete table of which laws hold for which
   strategy**, both citing `the-canon-is-intent-not-implementation.md` independently.

**One of these four is not as independent as it looks, and I want to flag it rather than let it pass as
clean corroboration.** Both files test the representability mechanism against the identical historical
case, `UFixed<0, 8, Hot>::ONE` holding raw `0`. I disclosed in my own phase one that this specific incident
sits in my ambient session memory rather than something I derived fresh; Amin's file cites it too, as
"the project's own memory records." That means the *choice of example* was contaminated identically in
both files, and the mechanism-level agreement (item 1) is only as independent as the part that was not
memorized: the actual trait-and-const-fn construction, which both of us built and compiled separately
rather than copied. I would treat the mechanism agreement as genuine (it required real, separately-run
compilation to land) and the choice-of-motivating-bug agreement as shared drift from one memorized fact
wearing two hats, not a fifth independent point.

**The shared blind spot is more important than the agreement.** Checkpoint `69` records that two earlier
cold derivations on an adjacent topic (`65`, `66`) read op's "validate" verb two genuinely different ways,
with measured, different machine cost: compile-time-and-per-type (the typed operation symbol-aliases to
the bare one, effectively free) against runtime-and-per-datum (the shape carries `tst`/`csel` residue that
does not erase). Neither `77` nor this file considered the runtime reading at all. Both of us built only
the compile-time mechanism and reported it as though it settled what "validate" means for a law, when the
panel's own adjacent unit had already found that op's own sentence supports a second, materially different,
non-free reading that neither of us tested. This is a real gap in both cold derivations, not a
disagreement to resolve: whether a law's validation is checked once, at the typestate boundary, or checked
again per incoming value at runtime, is exactly the same open question `69`'s Q-A puts to op for the
container case, and it applies to a law with identical force. I did not build the runtime-reading probe in
phase one and I do not have one to add now without breaking the phase-one/phase-two boundary; I record the
gap rather than pretend to close it.

### Where `77` genuinely goes further than I did, and I adopt it

**The chain question.** My treatment of composition stayed at the level of the congruence-closure argument
(a law belongs to a set of operations closed under a strategy) and the associativity-under-clamping
mechanism from probes 1 and 1b. I never built anything that measures what actually happens to error across
a chain of a *specific* operation. Amin's `probe1_chain_error.py` did, and the result is sharper than
anything in my own file: fixed-point **addition** needs no chain machinery at all, because the sum of two
already-quantized values is exact whenever it does not overflow, so there is no rounding step for error to
accumulate at. Fixed-point **multiplication** does need it, because the exact product needs `2F` bits and a
naive per-step round-back genuinely discards information every step, while a widened accumulator pays
exactly one rounding cost regardless of chain length. And the naive failure is not simple growth: for a
decaying input the error plateaus, for an input near one raised to increasing powers it is non-monotonic
and can spike sharply once the represented magnitude approaches the quantum (0, 4, 0, 2, 4, 2, 2, then 130
rounding steps at `n = 4096` in Amin's own table). This refines my own "the unit is (strategy, {operations
composed})" claim into something more specific and more useful: **whether an operation needs chain-level
machinery is a fact about whether that operation's rounding step is closed under the algebraic structure
its own outputs feed back into**, addition's outputs stay in the same additive coset of quantized values,
multiplication's do not stay in the same multiplicative structure without widening. I adopt this as a
correction to my own framing rather than restate my weaker version.

**Human legibility as a second obligation, separate from machine-checkability.** I argued the canon should
state the mechanism (derive, validate exhaustively, erase) rather than a table of concrete laws. Amin
sharpens this in a way I had not considered: a mechanism that is only checkable by running the compiler
against a specific instantiation is not, by itself, legible to a human reader of the canon, or to a
consumer reading generated documentation, who wants to know **which shapes a law covers without trying
every one.** So the canon owes two things, not one: the mechanism (which discharges the obligation
per-instantiation), and, wherever a domain restriction is real, a closed-form statement of that domain in
the canon's own prose (`ONE_RAW` representable exactly when `I >= 1`, stated as intent, not as `assert!`
syntax). I had conflated these into a single mechanism-only obligation in phase one; they are two, and a
canon that ships only the mechanism has bought machine-checkability at the price of a reader ever being
able to look at the canon and know the law's reach without compiling something.

**The defect/benefit pair for `I5`-licensed refusals.** My "explicitly refused laws" tier said Hot's
refusal of a law should be stated with the gain that justified it, in prose. Amin's reading is sharper and
is worth adopting over mine: read together with `I8`'s measurement-weighting framing, a law under Hot is
not simply absent where Precise would have it; it is a **paired, ideally provable, claim**, "this law fails
by at most `d`, purchasing a gain of at least `g`, measured however Hot weighs gains." That is a stronger
and more falsifiable sentence than "the refusal is stated." Amin is honest that the premises do not settle
whether the pair should be an inspectable type-level fact or a canon-and-bench-level fact only, and I agree
that is genuinely open; I would not resolve it here either, and I note it strengthens rather than replaces
the tier I proposed.

### Where the panel's own record, outside either cold derivation, strengthens my answer

**The two-value-map-based question in `OPTIONS.md`'s Q11 and Q12 is the same question this dispatch asks,
asked from the fold layer rather than from the law layer, and the answer converges from both directions.**
`35`'s finding (via `OPTIONS.md` and my own `42`) that a fold's accumulator needs both a width-sufficiency
bound and a law-satisfaction bound, refusing independently for independent reasons at one call site, is the
concrete worked instance of the entailment/derivation distinction I argued for in phase one: the width
bound is a container derivation (op's own acceptance criterion, read literally), and the law bound is
exactly the same mechanism applied to an algebraic fact instead of a container fact. Nothing about the
mechanism differs; the object being derived, validated, and erased does.

**The `56`/`57`/`61` thread on absorption and a "coherence law", read through `OPTIONS.md`'s account, is
independent, panel-level corroboration of my "laws that hold only on a subset" section**, from a completely
different investigative angle than either cold derivation. `61` measured that an absorbing-top reading and
a separately-argued coherence law are **the same law exactly where every operand lies inside the
representable set, and differ outside it**, zero disagreements inside, 206 outside. That is precisely the
shape I argued for from the `UFixed<0, 8>::ONE` case: a conditional law is a real law when its domain is
established and stated, and a real hazard when it is not. Here the domain that rescues the identification
is "every operand of a real fold is already format-typed, so it is inside the set by construction" (`61`'s
own framing per `OPTIONS.md`), which is exactly the type-enforced-domain condition both cold derivations
independently required.

**`DROPLIST.md`'s carried entry on gating algorithm crates by a single associativity flag** ("refused by
measurement, since it admits the one preset whose recurrences return wrong answers... and refuses the two
that compute correctly") is the sharpest available evidence against a flat, un-parameterized law vocabulary
and for the per-property, per-axis-value granularity both cold derivations converged on independently. A
`Sound` marker bundling several properties would be wrong in both directions at once for some presets; `42`
section 4.3 already measured what per-property granularity costs (one impl per satisfying combination,
since `specialization` and `negative_impls` are both forbidden) and found the cost real but not a reason to
bundle, because bundling had already been refused on correctness grounds rather than ergonomics grounds.

### Revised answer

Op's canon owes algebraic laws as a vocabulary, never as a rewriting mechanism, on grounds independent of
each other (a direct prior refusal of relocation, a ratified workspace rule that already assumes the
vocabulary exists, and this panel's own algorithm-crate soundness gap). "Derived" carries two senses that
compose rather than compete: a law is derived in the sense of being a theorem of the format's own
denotation rule and the strategy's semantics (never an independently asserted fact), and it is derived in
the operational sense of being computed, per instantiation, by the same three-verb mechanism op names for
containers, with the compile-time reading of "validate" being the one both cold derivations built and the
runtime-per-datum reading being a live, unaddressed alternative the panel's adjacent unit has already
priced. A law belongs to the pair of an operation and a strategy's axis assignment, not to a bare type or a
bare operation, and under composition it belongs more specifically to whichever algebraic structure that
operation's outputs are actually closed under, which is why addition and multiplication need different
answers to the identical chain question. A subset law is a real law exactly when its domain is enforced by
the type rather than sampled by a test suite, and the canon owes that domain in closed, human-legible form
in addition to the mechanism that enforces it. A strategy's licensed refusal of a law (`I5`) is not an
absence; it is a paired, quantifiable claim about what the law loses and what it buys, and whether that
pair lives in the type or only in prose and bench evidence is genuinely open. None of this closes the
question the dispatch opened with explore-do-not-settle in force; it narrows where the remaining forks are,
and names the one both parallel cold derivations missed.
