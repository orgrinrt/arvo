# 93. The strategy axis, derived cold

Phase one. Written before reading any panel file, any consolidation, `OPTIONS.md`, `DROPLIST.md`,
`PRIOR_CALLS.md`, any `SEED_*` file, any other member's probe directory, or the panel's commit log.
Reconciliation is appended after this is committed, and nothing above the reconciliation heading is
rewritten afterwards.

## What I read, and the one leak I have to declare

Read in full: `INTENTS.md`, `RULES.md`, `mock/DESIGN.md.tmpl`, `mock/PRINCIPLES.md.tmpl`,
`mock/WORKFLOW.md.tmpl`, `mock/Cargo.toml`, the workspace rules already in my context, and
`mock/benches/bench.toml` plus one generated findings file. Read partially, as source rather than as
argument: `notko/src/lib.rs`, `notko/notko-macros-core/src/tiers.rs`, `notko/notko-macros/src/lib.rs`,
and seven `mock/benches/variants/*-shared/src` test modules.

Not read: every `NN_*.md` in this panel, every other `NN_probes/` directory, `git log`, and every commit
message. `94_probes/` exists in the working tree while I write; I did not open it.

**The leak.** Late in the work, after P4 had already run and produced its answer, I opened
`mock/benches/variants/satfold-gate-true/src/lib.rs` to check which bench variant was which. Its first
paragraph names two panel files and states their headline: that such a gate erases. So on that one point
I learned, after the fact, that the panel already holds a result. I did not open either file, I do not
know their reasoning, and P4 was complete before I saw it. Everything else here is uncontaminated as far
as I can tell.

## The two gates

**Canon gate: passed.** I checked the assigned question against `INTENTS.md` I1 through I17. The question
is licensed: I1 demotes the strategy set to open in op's own words ("the strategy set is not closed at
exactly four... entirely open to discussion and exploration", `INTENTS.md:56-58`), and I2 marks its
enumeration OPEN while keeping the shape of the claim. Nothing in the intents forecloses asking what a
strategy is, and I17 makes the count explicitly beside the point of the intent it carries
(`INTENTS.md:326-328`). I found no misalignment in the work I was asked to do. I did find two tensions
*between* intents, which are reported in section 9 and handed back rather than resolved.

**Test gate: passed, with a caveat about what there was to gate.**

`cargo test --manifest-path mock/Cargo.toml` returns:

```
error: manifest path `.../mock` contains no package: The manifest is virtual,
and the workspace has no members.
```

That is not a decorative suite, it is an absent one, and it is absent for the right reason: the code tier
was nuked so the canon could be written, which `mock/Cargo.toml:1-15` states in its own comment and which
the mutation order requires. There is no shipped strategy implementation to gate.

So I gated the nearest thing that exists and that my question touches: the bench-shared crates, which are
where the strategy-shaped arms actually live. Seven suites, run individually because `mock/benches` is
excluded from the workspace (`mock/Cargo.toml:32`):

| crate | result |
|---|---|
| `satfold-shared` | 11 passed |
| `warm-clamp-shared` | 7 passed |
| `warm-container-shared` | 15 passed |
| `bitpack-shared` | 3 passed |
| `wide-rung-shared` | 30 passed |
| `quantiser-fadd-shared` | 1 passed |
| `quantiser-radix-shared` | 3 passed |

Seventy tests, all green. I read the bodies rather than the names, and they are real work. Three
properties I checked for specifically and found:

They assert **over every declared key rather than a subset**. `warm-container-shared`'s
`all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key` iterates `ALL_KEYS` and says in
its own doc comment why: "for **every** key the manifest declares, not for a chosen subset... if any of
them computes something else the timing comparison is meaningless and the fast one is fast because it is
doing less."

They check against an **independent oracle**, not against each other. Same file: "The value is also
checked against the independent `u128` reference, so four agreeing arms sharing one wrong transform is
caught rather than confirmed."

They carry **negative controls**. `satfold-shared` runs three deliberately wrong kernels against the
same oracle, with the reasoning stated: "The agreement above is worthless if the arms could not have
disagreed."

That is the opposite of a suite written to make a green line up, and saying so is a result rather than a
courtesy. I have no criticism of it to offer.

I also applied the same standard to my own probes rather than only to somebody else's. P3's 81
compile-time assertions are shown capable of failing by a committed mutant that breaks one join impl and
does not compile (`93_probes/p3_mutant_wrong_join.out`), and P5's compile-time refusal has its own
committed mutant (`93_probes/p5_mutant_const_overflow.out`). A check nobody has seen fail is not a check.

## 1. The answer, stated once before it is argued for

**A strategy is a preference: an ordering over candidate implementations of the same abstract operation,
computable at compile time, defined relative to a cost model whose inputs are also compile-time
available.**

Everything the shipped design attributes to a strategy is downstream of that. The container is what you
get when the preference is applied to the candidate containers. The codegen choice is what you get when
it is applied to the candidate lowerings. The overflow rule is what you get when it is applied to the
candidate ways of handling an unrepresentable result. Those are three consequences of one thing, not
three components of one marker, and `mock/PRINCIPLES.md.tmpl:288-292` lists them as three:

> `Hot` / `Warm` / `Cold` / `Precise` are not decoration. They drive:
> - Container physical width (u8 / u16 / u32 / u64 / arbitrary).
> - Codegen decisions (branch hints, fused-mul-add, prefetch).
> - Tradeoff clarity [...]

That list is not wrong. It is a list of effects presented where the cause belongs, and a canon that
copies it will have to keep it in sync with whatever else a strategy turns out to drive, forever.

**And "strategy" as currently used names at least two things.** They are not parallel, they are ordered,
and the order does not reverse. Section 3 derives that; sections 4 and 5 are what follows from it.

## 2. Why a preference, derived

Take an abstract operation, a format, and a target. There is more than one implementation. Each carries
a vector of costs: time, space, error against the exact result, divergence from some reference
behaviour, and definedness. Cost vectors in more than one dimension are only partially ordered, so
"which implementation" has no answer at all until something collapses the partial order into a decision.

**That collapse is the whole content of a strategy.** It is what I8 says in op's own words
(`INTENTS.md:136-139`):

> All of them should be decided by measurement, just measuring different things, and, this is I think
> the mental unlock: They weigh different measurements differently.

and what I9 says (`INTENTS.md:162-164`):

> strategies are the variables that change what the "correct" answer is for what we choose as the path.

Read together those two sentences are a definition rather than two observations. If every strategy is
decided by measurement, and the strategies differ in how they weigh measurements, then a strategy *is* a
weighting, and "the correct answer" is the argmin under that weighting. Nothing else is left over.

### The four presets, tested against it

Three of the four fall out immediately. I5's Hot weights time and is willing to spend accuracy and
definedness (`INTENTS.md:100-103`). I6's Cold weights space and is explicitly permitted to spend time
(`INTENTS.md:109-110`, "it has more leeway to do things non-efficient"). I7's Precise weights error and
is willing to spend both (`INTENTS.md:125-127`).

**Warm does not, and that is the interesting one.** I3 does not name a cost at all. It names another
system and says behave like it (`INTENTS.md:81`): "It should behave like native primitives in regular
old rust would". An imitation constraint is a different kind of specification from a weighting, and if
the frame above is right, either Warm is not a strategy in the same sense as the other three, or the
frame is missing an axis.

The frame is missing an axis, and I4 is what says so. Op's own refinement (`INTENTS.md:92-94`):

> Warm does not merely imitate, its intent is to be intuitive best choice for most every use case, and
> the intuitive part demands it mimics, but it does not make it absolutely required, if mimicking is
> consistently just worse choice.

So Warm's objective is minimal **surprise**, and imitation is the vehicle. Surprise is a measurable
quantity: how often, and by how much, does this arm differ from the behaviour a reader who knows Rust
would predict. That is a cost axis like any other, it is just not one a machine-performance mindset
would have written down, and I8's "measuring different things" is exactly this. Warm is a preference,
weighting an axis the other three ignore.

**This is a case where keeping the existing answer is the result.** I4 already resolves the tension and
op already stated it. The contribution here is only that the frame predicted a problem at Warm before
looking, and I4 was sitting there answering it.

### What the axes are, and one that nobody has named

If a strategy is a weighting, the canon owes the thing being weighted. From what a numeral operation can
actually be observed to differ in:

1. **Time**, which is not one number. Latency and throughput can disagree, and both are functions of the
   workload rather than constants. The bench tree already treats them that way: `bitpack-carrier-width`
   sweeps "from L1 to past a 12 MB L2" (`mock/benches/bench.toml`, that bench's `title`).
2. **Space**, split into bits per stored value and footprint in a working set. The second is what
   bitpacking trades against decode cost, and the two do not move together.
3. **Error against exact**, worst case and distribution.
4. **Divergence from a reference semantics**, which is the axis Warm exists on and which no other preset
   weights.
5. **Definedness**, which is a permission rather than a quantity. I5 makes it tradeable for Hot alone,
   and bounds the trade: "it should not lose it for nothing, instead, provable meaningful gains"
   (`INTENTS.md:102-103`).

And one more, which I believe is a genuine gap rather than an oversight I am misreading:

6. **Reproducibility across targets and builds.** Whether the same program on two machines produces the
   same bits. No preset names it, and **`Precise` does not imply it**: the most accurate arm on aarch64
   and the most accurate arm on x86-64 can be two different arms giving two different answers, and both
   are correctly serving I7. A consumer doing lockstep simulation, deterministic replay, or content
   addressing needs this above accuracy, and I11 says the value of arvo is what composes on top of it
   (`INTENTS.md:180-183`), which is where those consumers live.

I do not propose a preset for it. I report that the axis is demanded, is orthogonal to the five above,
and is currently unnameable in the design.

### Costs are functions, not constants, which forces the resolution to be computed

Time depends on n, on the access pattern, on how many cores are available. I10 says arvo takes no stance
on the last of those (`INTENTS.md:169-172`). So a strategy cannot be a fixed weight vector applied to a
fixed cost table; it has to be a weighting applied to a cost *model* parameterised by what is known.

Which is exactly I13's shape, and op's own addendum settles what "known" reaches (`INTENTS.md:238-240`):

> the above collapses to whatever is available at const time: Making the predicates const expressions
> for example, allows using const functions and pipe in some data that is outside the typestate.

So the sharpest form of the definition: **a strategy is a total order on candidate arms, computed from
whatever is available at const time.** I15 makes that mandatory rather than merely elegant, since a
resolution that is not const has to be a runtime branch and runtime checks do not exist
(`INTENTS.md:290-292`).

## 3. "Strategy" is two things, and they are ordered

Some of what a strategy controls **changes the answer**: the overflow rule, the rounding mode, the width
of an intermediate, whether an approximation of a reciprocal is acceptable. Some of it **does not**:
which container an identical value sits in, whether the loop vectorises, whether storage is bitpacked or
byte-aligned, prefetch, instruction selection.

Call the first the **policy layer** and the second the **lowering layer**. The distinction is not
stylistic and it is not a matter of taste, because the two have opposite properties on every axis that
matters to a design:

- The policy layer is **observable** to the consumer, so it must be in the type. Two values computed
  under different rounding are not interchangeable, and nothing downstream can recover which one it has.
- The lowering layer is observable only through time and space. It is free to vary per call site, per
  build and per target, and putting it in the type actively prevents the "always optimal internals"
  discipline from doing its job: a marker meaning "aligned and vectorised" is a lie in the type on a
  target with no vector unit.

### And the ordering, which is the part that is derivable rather than asserted

There is an apparent circularity. The preference ranks arms by cost. The set of available arms depends
on which rewrites are legal. Which rewrites are legal depends on which laws hold. Which laws hold
depends on the policy choices the preference was supposed to make.

**It resolves by stratification, and the stratification is forced.** The policy layer must be decided
first, because it *defines the space* the lowering layer optimises within. You cannot rank fused kernels
against unfused ones until you know whether the fusion is sound, and whether it is sound is a policy
question.

That is not an argument from neatness. It is measured. P2b (`93_probes/p2b_where_the_algebra_actually_varies.out`)
and P2c (`93_probes/p2c_the_boundary_across_widths.out`) establish that **which algebraic laws hold is a
function of a policy choice**, and identify which policy choice.

I got this wrong first and it is worth recording, because the wrong answer is the one a designer would
guess. P2 (`93_probes/p2_policy_selects_the_algebra.out`) predicted that wrapping, saturating and exact
would give three different law sets. They do not. Exhaustively, at W in {3,4,5,6}, all five laws checked
hold under all three. The reason is visible once the result is in hand: collapsing every value at or
above the maximum onto the maximum is a semiring congruence on the naturals, and so is reduction modulo
2^W, so all three are commutative semiring quotients of one structure and inherit its laws. **The
overflow axis does not move the algebra.**

The axis that does is the fraction width, through the rounding a fractional multiply forces. From P2c,
exhaustive over every triple, six widths, both overflow policies:

| W | F | overflow | triples | mul-assoc fails | distrib fails |
|---:|---:|---|---:|---:|---:|
| 6 | 0 | wrap | 262144 | 0 | 0 |
| 6 | 1 | wrap | 262144 | 137472 | 80384 |
| 6 | 0 | saturate | 262144 | 0 | 0 |
| 6 | 1 | saturate | 262144 | 8054 | 2946 |
| 8 | 0 | wrap | 16777216 | 0 | 0 |
| 8 | 1 | wrap | 16777216 | 9270976 | 5218304 |
| 8 | 0 | saturate | 16777216 | 0 | 0 |
| 8 | 1 | saturate | 16777216 | 132464 | 48286 |

Every `F = 0` row is zero and every `F > 0` row is nonzero, at W in {3..8}, both overflow policies, both
roundings. Additive associativity is insensitive to all of it.

Two separate things are in that table and conflating them would be a mistake:

- **The boundary is on the fraction axis.** Where a rewrite is *legal* is decided by F.
- **The magnitude is on the overflow axis.** At W=8, F=1, an unguarded reassociation changes the answer
  on 55% of triples under wrapping and 0.8% under saturation, a factor of seventy. So overflow decides
  how *wrong* an illegal rewrite is without moving where it is illegal. I note one caveat on that
  number: saturation's low rate is partly two clamped results agreeing by both being clamped, which is
  not the same as being right, and I have not separated those.

### The shipped design already stratifies, in the one place it had to

`mock/benches/bench.toml` registers `warm-container-width-l1`, titled "Container fork, declared-width
sweep, cache-resident (8192 elements, 3 ops/element, **wrapping**)", and `precise-container-width-l1`,
titled "Container fork under **saturating** semantics, declared-width sweep". The same lowering question
is benched twice, once per policy, because the answer to the second depends on the first. Whoever built
that bench matrix had already found the ordering; it is in the measurement design and not in the
document.

### Independent evidence from the sibling crate, and a vocabulary collision

`notko` ships tiers named `Hot`, `Warm` and `Cold`, and its enum for what they select is also called
`Strategy` (`notko/notko-macros-core/src/tiers.rs:53`, `:61`, `:69`, `:78`). What varies along notko's
axis is **how much information about a failure is carried**: Hot is `Just<T>` with no branch, Warm is
`Maybe<T>` with a one-bit discriminant, Cold is `Outcome<T, E>` with a full payload and a branch
(`notko/src/lib.rs:41-44`). It is applied **lexically to a function**, rewriting the body
(`notko/notko-macros/src/lib.rs:21-30`), and there is no `Precise`.

Three readings, and they do not exclude each other.

**As a name collision it is a defect.** Two crates one dependency edge apart both export `Strategy`,
meaning two unrelated things, and three markers whose names are shared and whose axes are not. Every
`Hot` a reader meets in this stack has to be disambiguated by crate.

**As evidence it supports section 2.** The names transfer because they name a *preference direction*,
and the axis is supplied by the domain. notko's Hot is willing to turn an error into a panic for speed,
which is I5's shape exactly, on a completely different axis. Preferences are domain-polymorphic; a
container choice is not.

**As a shape it corroborates section 3's ordering.** notko's tiers are three points on one totally
ordered axis, so a resolution between two of them is just the maximum and cannot fail to exist. arvo
took the same three names for a space that is not one axis and not totally ordered, and section 4 is
what that costs.

**And the 3-versus-4 mismatch is the seam.** `arvo-always-optimal-internals.md` records that notko's
three tiers face arvo's four strategies, so `Precise` has no tier. That is not an accident of scope. A
lexical, whole-function mechanism naturally carries a *preference*, which is what Hot, Warm and Cold are.
It cannot carry a *denotation*, which is what Precise is, because the denotation belongs to the values
and not to the region of source they were written in. The mismatch is the policy layer and the lowering
layer coming apart in an artifact that was built for only one of them.

## 4. What relates two strategies

The shipped design's answer is `Resolve<S1, S2>` (`mock/DESIGN.md.tmpl:35`), with the intended behaviour
illustrated in `arvo-toolbox-not-policer.md`: "Cross-strategy binary op where overflow policies disagree
(`Hot wrapping + Precise saturating -> Precise`)".

### Three laws are forced, not chosen

If `x: T<A>` and `y: T<B>` and `x + y` has type `T<Resolve<A,B>>`, then:

- **Commutativity is mandatory.** Otherwise `x + y` and `y + x` are different types, and addition stops
  commuting at the type level even where it commutes at the value level.
- **Associativity is mandatory.** Otherwise `(x+y)+z` and `x+(y+z)` are different types, so the type
  system forbids a reassociation the value semantics permits, which is precisely backwards given that
  finding where reassociation is legal is the work (I13).
- **Idempotence is mandatory.** Otherwise same-strategy addition escalates.

Commutative, associative and idempotent is a join semilattice. **So "what relates two strategies" has a
forced answer: a semilattice, or nothing.** The induced order follows from the operation rather than
being picked first.

### The flat four-element set, enumerated

P1 (`93_probes/p1_resolve_semilattice.out`) enumerates all 4096 commutative idempotent binary operations
on four labelled elements and filters for associativity. **76 are semilattices.**

I predicted none would survive the intents. **Nine did**, so the prediction was wrong and the flat-set
question is not settled by the three laws alone. P1b (`93_probes/p1b_demands_and_closure.out`) is the
sharper form, with three further constraints P1 had left out, each derived from an intent:

| constraint | grounds | survive |
|---|---|---:|
| all semilattices | | 76 |
| K1 `Resolve(Hot,Precise) = Precise` | toolbox-not-policer's own worked example | 31 |
| K2 `Resolve(Cold,Hot) != Hot` | I17, the storage path is not deprioritised | 24 |
| K3 `Resolve(Cold,Hot) != Cold` | I5, Hot's intent must be reachable in expressions | 9 |
| K4 `Resolve(Warm,Precise) = Precise` | K1's argument against the default | 7 |
| K5 `Resolve(Warm,Cold) != Warm` | I17 against the default | 5 |
| K6 `Resolve(Warm,Hot) != Warm` | I5 against the default | 4 |

Four tables survive all six, and **all four make Precise the top and escalate every mixed expression to
it.** `Hot v Cold = Precise` in each. That is a real design and it is what "resolve conservatively"
means, and its cost should be stated plainly: two operands neither of which asked for accuracy produce a
result carrying the most expensive policy in the set. Nobody asked and everybody pays.

### The constraint set was still too weak, and the theorem is in Part B

K2 through K6 forbid the result being *the other operand*. They do not forbid *losing the demand*, which
is what escalation does. The honest constraint is the one `arvo-toolbox-not-policer.md` states in its
own list of things the substrate does not provide: "Default selections that quietly change semantics
(auto-resolve to a more conservative strategy without flagging)."

Model each strategy as the set of demands it carries and resolution as the union of demands, since
mixing a value whose type demands minimal storage with one whose type demands maximal speed produces
something that has been asked for both. Then:

> Resolution loses nothing **iff** the strategy set is closed under union of the demands its members
> carry.

P1b Part B computes it. Four markers carrying one demand each leave **12 of 16 ordered pairs
unresolvable**: `Hot v Cold` needs {speed, space} and no member carries it, and so on for eleven more.
The smallest set closed under the four presets' own resolution has **15 elements**.

**That is a property of the set, not of the table, and no better table fixes it.** Either the design
carries the closure, or resolution is not a join and has to be something else.

### Three responses, and they are genuinely different designs

**(a) Carry the closure.** Stop naming a list and name the axes; a strategy is a point in the product,
and a preset is a point somebody gave a name to. Resolution is componentwise and its laws are free.
`Hot v Cold` becomes a defined element rather than an escalation. Cost: the space is larger than four,
so every "what does this strategy do" question needs an answer parameterised by axis rather than looked
up per marker.

**(b) Refuse the mix and require the consumer to name the result.** Resolution stays partial; a mixed
expression without a declared result strategy is a compile-time report that two demands conflict. This
is more consumer-controlled and needs no lattice. It sits awkwardly against
`arvo-toolbox-not-policer.md`, which lists "Refuse to compile cross-strategy ops 'for safety'" as an
incorrect shape, though I read a refusal that *names a genuine conflict* as the diagnostic that rule
wants rather than the policing it forbids. Cost: verbosity at every mixed site.

**(c) There is no join to compute, because the demands are on different roles.** `Cold` names what a
value costs at rest; `Hot` names what an operation costs in flight. Under that reading `Cold + Hot` is
not a conflict at all: it is decode from cold storage, compute hot, encode back, which is a pipeline and
a perfectly ordinary thing to want. The "conflict" exists only because one parameter is being asked to
carry two roles, and the four-element flat set cannot express the combination that is actually wanted.

**What would distinguish them.** (b) against (a) is decided by how often mixed-strategy arithmetic
actually occurs in consumer code, which is a grep over hilavitkutin and vehje that I did not run and
that would be reading a dead tier anyway. (c) against both is decided by whether the storage decision
and the compute decision are in fact independently chosen in real workloads. The bench tree says they
are: `warm-clamp-arity-w13`'s own title describes benching "the shipped doubled container against
minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the
accumulator sized by the design's own interior-safety rule". The accumulator's container is being chosen
separately from the stored operands'. If those were one decision the bench matrix would not factor that
way.

I do not resolve between them, and I lean toward (a) and (c) being the same answer seen from two sides:
both say the object is a product and the flat set is a diagonal slice through it.

### Is the product shape actually available under the pin

P3 (`93_probes/p3_product_lattice_in_the_type_system.out`) says yes, and this was worth checking rather
than assuming, because the obvious encoding is forbidden. Writing `join(A, B)` in type position is
arithmetic in a const argument and needs `generic_const_exprs`, which is forbidden. The bound that will
not go wants a trait: per-axis join as an associated type, whole-strategy join as one blanket impl over
the product, and the solver resolves it.

It compiles on `nightly-2026-05-28` with no forbidden feature, `#![no_std]`, no `dyn`, no `TypeId`, and
one allowed gate (`const_trait_impl`). The evidence factors deliberately: 81 of 81 ordered pairs assert
at compile time that the type-level join equals an independently written const-fn join on tags, and the
const-fn join is then checked exhaustively for idempotence (0 of 9 fail), commutativity (0 of 81) and
associativity (0 of 729). One impl, not nine, and it stays one impl however many points an axis grows.

The mutant confirms those 81 assertions can fail: breaking one join impl produces `error[E0080]:
evaluation panicked` at compile time (`93_probes/p3_mutant_wrong_join.out`).

## 5. How a strategy is carried

### The type parameter must name the preference, never the resolution

This follows from section 3 and it is the sharpest architectural consequence I have. If `Hot` in the
type meant "aligned, 64-bit container, vectorised", then a target without a vector unit makes the type
false, two targets give behaviourally different values at the same type, and every target change is a
type change. If `Hot` means "prefer time", the resolution is a compile-time function of the preference,
the format, and the target, and none of that leaks into the type.

**The shipped design already does this and it should be kept.** `mock/DESIGN.md.tmpl:35` describes the
container as reached "via Pattern C const-tag dispatch through `Project<TAG, Sign, BYTES, S>`": the
strategy is the input to a projection and the container is its output. That is exactly the right shape
and it survives everything above. What does not survive is
`mock/PRINCIPLES.md.tmpl:288-292` presenting the projection's output as one of three things the marker
"drives", which reads as though the container were part of the strategy rather than a value computed
from it.

### Does the preference erase

P4 (`93_probes/p4_preference_erases.out`, `93_probes/p4_asm_comparison.out`) emits four symbols at
`-O` on aarch64-apple-darwin and compares the bodies. A weighting over three cost axes, resolved by a
const-fn argmin over a three-arm cost table, against the arm written by hand with no strategy machinery
at all:

```
direct_wrap        9 instructions
pref_speed         9 instructions
direct_wrap == pref_speed : True   (local label numbering normalised, nothing else)
direct_wrap == direct_widen : False   <- or the comparison above would be vacuous
```

Nothing from the cost model survives: no table, no argmin, no branch on a weighting, and the two arms
the preference did not select are absent from the body entirely.

**And the honest addendum, because the other pair is not identical.** `direct_widen` and `pref_accuracy`
are 39 instructions each with an identical opcode multiset, and four lines differ: an `add` computing an
address and a `sub` computing a trip count are scheduled in the opposite order in the scalar tail of an
autovectorised loop, and the register allocator then swaps x9 and x10. Nothing is added and nothing is
removed. So the claim is narrower than "identical output": **the cost model leaves no residue**, while
the backend's instruction scheduling is not stable across two call sites of the same inlined body. I
report the narrower claim because it is the one the evidence supports.

### And it erases in time, which is a bench and not a spike

`mock/benches/satfold-const-gate_n10000_findings.md` is committed harness output for a bench whose title
is "Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a
const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false
so it selects the fallback". Confirmed against the variant sources: `satfold-lanes16` is the arm
directly, `satfold-gate-true` is the same computation reached through the gate.

| variant | median | 95% CI |
|---|---:|---|
| `satfold-gate-true` (through the const verdict) | 1438 ns | [1435, 1460] |
| `satfold-lanes16` (arm reached directly) | 1456 ns | [1454, 1460] |
| `satfold-gate-false` (law false, fallback selected) | 38391 ns | [38374, 38405] |

The gated and ungated forms are within overlapping intervals, and the gate is load-bearing: it is the
difference between 1.4 us and 38.4 us. That is the same conclusion as P4 by a different instrument, from
a different author, on the harness rather than an ad-hoc spike. Two independent instances, and P4's
mutant-checked compile-time refusal is a third line of the same argument.

This is where my declared leak sits. The variant's own header says two panel files already report that
such a gate erases, so the panel likely holds this; my contribution is that I reached it without them.

### What a marker on a value cannot carry

I7's clause "especially within chains and ops, not only alone" (`INTENTS.md:127`) is doing more work
than it looks like. A strategy whose objective is evaluated over a **chain** cannot be implemented by
picking a better lowering for each operation, because the per-operation optimum is not the chain
optimum.

P6 (`93_probes/p6_the_chain_clause.out`) measures both halves. At W=8, F=4, unsigned, rounding to
nearest, comparing rounding at every step against one rounding at the end:

| chain length | chains | differ | % | max abs error |
|---:|---:|---:|---:|---:|
| 2 | 65536 | 0 | 0.0% | 0 |
| 3 | 16777216 | 1364059 | 8.1% | 238 |
| 4 | 268435456 | 17908021 | 6.7% | 236 |
| 5 | 1073741824 | 53355204 | 5.0% | 239 |

Length 2 is the built-in control and correctly shows zero, since with one multiply the two are the same
computation. Past that the gap is not a last-bit matter: a max absolute error of 238 raw units in a
domain of 256.

The structural half is Part B. For the round-once arm to be exact after k multiplies, the intermediate
is scaled by 2^(kF) and its integer part reaches the product of k values, so the required width grows
linearly in chain length: 16 bits at length 2, 24 at 3, 64 at 8, against an 8-bit input type.

**No fixed input type holds it past length one.** So a preset whose objective is chain accuracy cannot
be implemented by an operator closed over its operand type. Its multiply must return something wider
than it consumed, and the collapse back to the declared format has to be a separate explicit step.

Which means: **a strategy chooses which operator surface exists**, not merely what a fixed operator
lowers to. Closed for a preset that rounds per operation, opening into a widening tower for one that
does not. That difference shows up in the signature rather than in the generated code, and a marker
attached to a *value* cannot make it on its own, because the decision is about an *expression*.

Two designs deliver it and they are not the same. Either the operator's result type opens and the
consumer writes the collapse, or a staging layer sees the whole expression and places the rounding.
`mock/benches/bench.toml`'s `warm-affine-collapse-l1` is titled "Wrapping reduction whose steps are all
affine: what the interior projection prevents the optimiser from doing", which is this question already
being measured, so the panel is not unaware of it. I have not read whatever concluded from that bench.

## 6. Findings, each with its predicate

Notation per I13 and `RULES.md`: a dimension listed with a range or `any` was established across it;
listed with a fixed value was established there only; **absent means the finding does not hold anywhere
that dimension is present.**

**F1. Multiplicative associativity and distributivity hold at F = 0 and fail at F > 0, and the boundary
does not move with the overflow policy or the rounding mode.**
`holds for: W in 3..8, F in 0..2, signedness = unsigned, overflow in {wrap, saturate}, rounding in
{truncate, nearest}, operations {mul, add}, arity 3, threads = 1, target features any`
Evidence: `93_probes/p2c_the_boundary_across_widths.out`, `93_probes/p2b_where_the_algebra_actually_varies.out`,
exhaustive over every triple, no sampling. `target features any` is claimed because the model computes
in exact `u128` integer arithmetic whose semantics rustc fixes independently of the target; that is the
transfer argument and it is the only one I am making.

**F1a. The F = 0 half extends to any width, by proof rather than by sweep.**
`holds for: W any, F = 0, signedness = unsigned, overflow in {wrap, saturate}, operations {mul, add},
arity 3, threads = 1, target features any`
At F = 0 the multiply performs no shift, so the operation is the one induced on the quotient of the
naturals by collapsing everything at or above the bound. That collapse is a semiring congruence for both
reduction mod 2^W and clamping at 2^W - 1, so the quotient is a commutative semiring and inherits every
law at any W. The sweep checks the argument; it is not the source of it. **The F > 0 half has no such
argument and is claimed only at the pairs swept.**

**F2. Additive associativity is insensitive to every axis varied.**
`holds for: W in 3..8, F in 0..4, signedness = unsigned, overflow in {wrap, saturate}, rounding in
{truncate, nearest}, operation add, arity 3, threads = 1, target features any`

**F3. On a flat four-element strategy set, no resolution table exists that loses no intent.**
`holds for: strategy set = four members each carrying one demand, demands = 4, resolution = union of
demands`
Evidence: `93_probes/p1b_demands_and_closure.out`. 12 of 16 ordered pairs are unresolvable; the closure
under the members' own resolution has 15 elements. This is a counting result about the set, not about
any particular table, so no table repairs it.

**F4. Exactly four join semilattices on the four named markers satisfy the six intent-derived
constraints, and every one of them escalates a mixed expression to Precise.**
`holds for: strategy set = {Hot, Warm, Cold, Precise}, constraints = K1..K6 as listed in section 4`
Evidence: `93_probes/p1_resolve_semilattice.out`, `93_probes/p1b_demands_and_closure.out`.

**F5. A product of per-axis chains, joined componentwise, is expressible in the type system on the pin
and satisfies idempotence, commutativity and associativity.**
`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, axes = 2, points per axis = 3, gates =
{const_trait_impl}, no_std, threads = 1, target features any`
Evidence: `93_probes/p3_product_lattice_in_the_type_system.out`, with
`93_probes/p3_mutant_wrong_join.out` as the negative control. The construction avoids
`generic_const_exprs` by carrying the join as an associated type rather than as a const expression in
type position.

**F6. A preference resolved by a const-fn argmin over a cost table leaves no residue in the emitted
body.**
`holds for: target aarch64-apple-darwin, rustc 1.98.0-nightly (57d06900f), opt-level 3, arms = 3, axes =
3, threads = 1`
Evidence: `93_probes/p4_asm_comparison.out`. One of the two pairs is instruction-identical after label
normalisation; the other has an identical opcode multiset and count, differing in the scheduling of two
independent instructions and the registers assigned to them. Corroborated in time, on the harness, by
`mock/benches/satfold-const-gate_n10000_findings.md`, at `n = 10000, samples = 40, threads = 1`.

**F7. A chain-weighting preset cannot be served by an operator closed over its operand type.**
`holds for: W = 8, F = 4, signedness = unsigned, rounding = nearest, operation mul, chain length 2..5,
threads = 1, target features any`
Evidence: `93_probes/p6_the_chain_clause.out`. The width the intermediate must hold grows linearly in
chain length, so no fixed input type holds it past length one. The structural half of this is
arithmetic and would extend; I claim it only where I measured it.

**F8. The two readings of "behave like a native Rust primitive" agree at every width Rust has a
primitive for and disagree at every width it does not.**
`holds for: W in 1..16, signedness = unsigned, operations {add, mul}, threads = 1, target features any`
Evidence: `93_probes/p5_what_warm_imitates.out`. Zero disagreement at W in {8, 16}; disagreement at all
fourteen non-native widths. The sweep steps uniformly above W = 11 and says so. **The specification is
total exactly where it is not needed, and ambiguous exactly on the widths arvo exists to provide.**

**F9. Overflow detection matching Rust's debug behaviour is available where both operands are const and
nowhere else.**
`holds for: const-evaluable operands, rustc 1.98.0-nightly (57d06900f)`
Evidence: `93_probes/p5_mutant_const_overflow.out`, which is the refusal as a compile error
(`error[E0080]: evaluation panicked: overflow at the declared width`). The complementary claim is
absence: for operands that are not const-available there is nothing, by I15, not a weaker check and not
a debug-only one.

## 7. What I would keep

Per `RULES.md`, keeping something is a result. Four things in the current design survive my derivation
and I would carry them forward unchanged:

**The strategy is a type parameter, not runtime configuration.** `mock/PRINCIPLES.md.tmpl:271-275`. Right
for the policy layer, for the reason that layer is observable, and I15 forces it for the rest.

**The container is projected from the strategy rather than named by it.**
`mock/DESIGN.md.tmpl:35`'s `Project<TAG, Sign, BYTES, S>`. This is the preference-not-resolution
discipline already implemented, and it is the single most important thing in the shipped design.

**Diagnostic, not directive.** `arvo-toolbox-not-policer.md`'s posture. Everything in section 4 that
looks like a refusal is a report of a conflict, and the rule's distinction is exactly the right one to
keep while doing that.

**`Resolve` as a name and as a concept**, provided it is a componentwise join over the **policy layer
only**. The lowering layer needs no resolution at all, because it is unobservable and each operation can
pick its own optimum. Note that `arvo-toolbox-not-policer.md`'s own three examples already behave this
way: the overflow-policy example *resolves* ("`Hot wrapping + Precise saturating -> Precise`") while the
SIMD-lane-width example only *warns* ("warn that the mixed expression broke vectorisation"). The rule
treats the policy layer as resolving and the lowering layer as merely reporting. It got the right answer
before the distinction was named.

## 8. What this does to the option space

**Fits well.** Any option that treats the strategy set as generated rather than enumerated; any option
that separates a storage decision from a compute decision; any option where the arm is computed from the
typestate at const time rather than tabulated per marker.

**Fits badly but survives at a cost.** A flat named set with a total order making Precise the top. It is
one of exactly four tables satisfying the six constraints (F4), so it is not arbitrary, and its cost is
that every mixed expression escalates to the most expensive policy and both operands' intents are lost
(F3). If it is chosen, the escalation should be stated in the canon as a known cost rather than
discovered later.

**Killed.** Any option asserting that a flat four-element marker set can carry a resolution which loses
no intent. F3 is a counting result about the set and no table repairs it. That is a closed route, and
what would reopen it is a demonstration that the presets carry overlapping rather than disjoint demands,
in which case the model in P1b Part B is the thing that is wrong rather than the conclusion.

**Also killed:** any option in which a strategy weighting chain accuracy is served by an operator closed
over its operand type. F7 is arithmetic.

## 9. Handed back rather than resolved

Two tensions between op's own stated intents. Neither is mine to settle, and per `RULES.md` a
converged thing goes to him rather than a fresh disagreement.

**T1. I3 against I15.** I3 says Warm behaves as a native Rust primitive would. A native Rust primitive
has two behaviours: wrap at the type's width in release, panic on overflow in debug. The panic is a
runtime check, and I15 is categorical: "Never any runtime checks, ever" (`INTENTS.md:290-292`). So the
imitation is available for the release half and unavailable for the debug half, permanently and by
construction. F9 names the region where something equivalent survives, which is the const-available one.
I read I4 as already anticipating this ("does not make it absolutely required, if mimicking is
consistently just worse choice"), so this may be a tension op has already dissolved. I report it rather
than assume so.

**T2. I3's remaining ambiguity at the widths that matter.** Even setting the debug half aside, "behave
like the native primitive" has two readings that agree at every native width and disagree at all
fourteen non-native ones (F8). Which of them Warm means is not decided by anything I read. I4 decides
*how* to decide it, by asking which is the intuitive best choice, and my own reading is that a consumer
who wrote thirteen bits meant thirteen, so the declared width should govern. I state that as my reading
and not as a finding, because it is a judgement about intuition and not something a probe settles.

## 10. Unlicensed mechanisms and defects noticed, in and out of scope

**The `Strategy` name collides across the stack.** `notko::notko_macros_core::tiers::Strategy`
(`notko/notko-macros-core/src/tiers.rs:78`) and arvo's `Strategy` (`mock/DESIGN.md.tmpl:35`) are
unrelated concepts one dependency edge apart, and three of arvo's four marker names are also notko's
tier names for a different axis (`notko/notko-macros-core/src/tiers.rs:53,61,69`). Nothing licenses
this and it is a real cost to every reader of both crates. It is not in my question and I am reporting
it anyway.

**`mock/PRINCIPLES.md.tmpl:32-38` names `feature(generic_const_exprs)` in the shipped feature set.**
That feature is FORBIDDEN under the workspace's `unstable-features.md`, on op's call, and the design
template still lists it among the gates arvo enables. There is no code left for it to be true of, so the
document is stating something about a tier that has been nuked, but the sentence is there and a reader
reconstructing arvo from these documents would enable a forbidden feature. It should not survive into
anything derived from this tier.

**`mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl` describe a sixteen-crate topology that does not
exist.** `mock/crates/` is empty. I understand why, and it is the mutation order working rather than a
defect. I flag it only because both documents are the design tier and the canon-design-code chain says a
lower tier surviving a change above it "becomes a claim about something that no longer exists" and gets
believed. Two documents in this repository currently are that.

## 11. What I did not establish

Bounded honesty about the edges, per `RULES.md`.

I did not measure anything on the bench harness myself. Every timing number I cite is from committed
harness output somebody else produced; my own probes are compile-time checks, exhaustive arithmetic
sweeps, and one emitted-assembly comparison. Where I say something about cost I am reading their
artifacts, and where nothing has been measured I have said unpriced rather than reaching for a number.

Everything I swept is **unsigned**. Signed fixed point clamps at both ends and the congruence argument
in F1a does not obviously carry, so I claim nothing about it. That is a real gap and it is the first
thing I would extend.

Everything I ran is **single-threaded**, so every finding is a `threads = 1` finding, which is a region
rather than a silence.

I did not attempt the design of the axis set. Section 2 lists six axes and I believe the sixth is
genuinely missing from the design, but "these are the right axes at the right granularity" is a claim I
have no evidence for and did not make. P3's two axes with three points each are scaffolding chosen to
reach the check, not a proposal, and its names, arities and field orders are not decisions.

I did not decide between the three responses in section 4. I said what would distinguish them and
leaned, and leaning is not deciding. In particular I did not grep hilavitkutin or vehje for how often
mixed-strategy arithmetic actually occurs, which is the measurement that would settle (b) against (a),
because both consumers are pinned to a tier that has been declared dead and I judged the number would
not mean what it looked like.

I did not read a single panel file, so where any of this is already known, restated, or refuted, I do
not know it yet. That is the point of the protocol and it is also the honest bound on this document.

---

# Phase two: reconciliation

*(To be appended after the phase-one commit. Nothing above this line is rewritten.)*
