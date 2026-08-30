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

Appended after reading the panel, in a separate commit. Phase one above is unedited, and the two
commits are separately attributable so a later reader can see exactly what was derived blind.

## What I read in phase two, and the bound on it

In full: `94_wingo_the_strategy_axis_derived_cold.md` (the twin cold derivation on the same question),
`25_torvalds_what_a_strategy_is.md`, `DROPLIST.md`, `87_op_the_canon_is_written_once_at_the_end.md`,
`88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md`. Most of
`40_leijen_what_the_axes_actually_are.md`, through its section 9, which is where the register bearing
sits. In `OPTIONS.md`: Q3, Q5 through Q7 as quoted by `40`, Q13, Q14, Q15, Q16, Q27, Q41, Q42, and the
unasked-questions section.

Not read: files `02` through `24`, `26` through `39`, `41` through `86`, `89` through `92`, the
consolidations `53`, `63` and `90`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`, the `SEED_*` files, and
`archive/`. So where a finding of mine restates something in one of those, I do not know it, and the
first three sections below are only as complete as that list allows.

**The largest thing I did not do:** I did not read `35_mcsherry`, which `40`'s observable-axis table
cites for six of its eight rows and which is the panel's existing measurement of exactly the laws my
P2b, P2c, P7 and P8 measure. I chose not to, because I would rather my numbers stayed independent of
it, and the cost is that I cannot say whether any of my four law probes reproduces or contradicts it.
Somebody should diff them, and that is a real piece of work I am leaving undone.

## 12. Op answered my question three files before mine, and I derived the smaller half of his answer

This is the headline and it should come before anything I got right.

`88` section 1 records op being asked exactly the structural question I was dispatched on, with the
options written out. Option 1 was "a preset naming a point in a space of independent axes, with the
presets being the points worth naming". Option 3 was "nothing but a weighting over measurements, with
every concrete difference falling out of optimising under those weights". His answer, verbatim
(`88:20-21`):

> Mostly option 1, but a little bit of option 3 with it. Hard to put into words, hopefully you get my
> meaning here

**Option 3 is my phase one, near enough word for word.** My section 1 says a strategy is a preference
over candidate implementations and that the container, the codegen choice and the overflow rule are
consequences of applying it. That is "every concrete difference falling out of optimising under those
weights", and op put it second.

I am not withdrawing it, for a reason that is in `88` itself: op flagged his own difficulty wording the
answer, and the file records that "a later expert finding the two readings pull apart somewhere has
found something real rather than a contradiction to resolve away" (`88:28-30`). Section 14 is where I
think they pull apart, and it is the most useful thing in this file.

But the ranking is his and it is not mine to re-weight. **The point reading is mostly it and the
weighting reading is a little bit of it**, and anything I write that reads as though the weighting were
the whole answer is overstated.

### And leijen had already reconciled the two, 53 files earlier

`40` section 0 states it outright, before op spoke:

> **The mechanism space.** Points are assignments to headroom, layout, overflow policy, intermediate
> precision [...] **The objective space.** Points are weightings over measurements [...] A strategy
> lives in the **objective** space. A mechanism assignment is what a strategy produces when it is
> applied to evidence.

with the relation written as `resolve :  objective  ×  evidence   ->   mechanism` (`40:26-28`), and the
explicit framing that this "is a refinement of `25` rather than a replacement" (`40:33-34`).

That is op's "mostly option 1, a little bit of option 3" derived before he said it, and it is better
than my phase one, because it keeps both levels and names which one the strategy lives in rather than
collapsing to the generator. **The credit for the two-space reading is `40`'s and `25`'s, and mine is a
third independent arrival at it.** Under `RULES.md`'s rungs, three independent derivations of the same
structure is worth recording as such: `25` from decomposing the shipped preset table and from four
industrial fixed-point systems, `40` from op's four intent statements and the arithmetic of naming, and
mine from the partial order on cost vectors. None of the three read the others first on this point.

## 13. Where I converge with the twin cold derivation, and where we do not

`94_wingo` was written blind, at the same time, from the same premises, and neither of us read the
other. So the agreements below are the TWO EXPERTS rung and the disagreements are worth more than
either of us saying it alone.

**Converged, independently.**

*A strategy is a preference and not a bundle of implementation choices.* My section 1 against `94`
section 3.1's "a strategy names a preference over outcomes, and that preference is what makes one
answer correct rather than another. Not a set of implementation choices."

*The word covers more than one thing, and the parts are separable.* My two layers against `94`'s three
components (cost, policy, licence). Section 13.1 below is where those two decompositions differ.

*The presets are points in a product, the set is open structurally rather than by leaving a list open,
and a new point is nearly free.* My section 4 option (a) and F5 against `94`'s W3, which prices it at
eight bytes and one symbol.

*The selection erases.* My F6 against `94`'s W1 and W2, on different instruments: I compared emitted
bodies for a const argmin over a cost table, `94` compared emitted bodies for an associated-const
choice function and separately showed the runtime-fact version emitting a compare and a conditional.

*Chain accuracy is structurally different and the construction is to stop quantising in the interior.*
My F7 and P6 against `94`'s W6 and W7. `94`'s W6 is the sharper form of the same boundary I measured:
it says rounding retracts exactly at `F = 0` and nowhere else, over `W in {4,6,8}` and `F in 0..=W`,
for both rounding modes. My F1 says multiplicative associativity and distributivity hold exactly at
`F = 0`, over `W in 3..8`. **Two different law families, one boundary, two independent probes.**

*Neither of us could settle cross-strategy resolution, and both of us say so.*

*The `generic_const_exprs` line in `mock/PRINCIPLES.md.tmpl` is a live design document naming a
forbidden feature.* Both of us reported it unprompted.

*The root design documents surviving unbannered is a defect of the chain rule's own named kind.* `94`
made it sharper than I did by counting: `grep -cin 'nuked|dead tier|superseded|stale'` returns 0 for
both.

### 13.1 Where the two decompositions differ, and how I think they reconcile

`94` splits into **cost**, **policy** and **licence**, and says licence "is not a cost question either,
and it is not a policy question: it is a permission" (`94` section 4.1). I split into a **policy layer**
and a **lowering layer**, with which rewrites are sound being *derived* from the policy rather than
being a third thing.

I think `94`'s licence is two things and that separating them dissolves the disagreement.

**Soundness is derived, not granted.** Whether a rewrite preserves the answer is a fact about the
policy, the format and the operation, and it is exactly what P2b, P2c, P7 and P8 measure. Nobody grants
it and nobody can withhold it. `94`'s own W4 and W5 are measurements of this: wrapping retracts on all
nine chains because it is a ring homomorphism, saturation on six, and that is arithmetic rather than
permission.

**Permission to be unsound is granted, and exactly one intent licenses it.** I5, and only I5: "Hot
*can* sacrifice soundness, that is its explicit purpose, but it should not lose it for nothing, instead,
provable meaningful gains" (`INTENTS.md:102-103`). That is a genuine permission, it is not derivable
from the policy, and it has to be carried somewhere.

So the reconciliation I would offer: **cost, policy, and a permission-to-be-unsound that only one
objective holds**, with soundness itself computed from the policy rather than carried. That is `94`'s
three components with the third narrowed, and it is my two layers with the narrow third added back.
`40` reaches the same place from its own direction, calling it "accuracy is lexicographically prior for
every objective except `Hot`, and finitely weighted for `Hot`" (`40` section 5.3), which is the same
statement in the weighting vocabulary.

**Where `94` is straightforwardly ahead of me and I concede:** it separates the two independent
permissions and I did not. Its W4 shows `wrap`/`sub` retracts and does not associate, so knowing one
tells you nothing about the other, and a design carrying a single "may I be clever" bit would take the
conjunction and lose the arm that needs only the weaker one. I measured associativity and
distributivity and never asked whether the permissions themselves nest. They do not.

**And where I am ahead of it:** `94` explicitly did not read any findings file in `mock/benches/`, by
choice, and says so. So its W1 and W2 are emitted-code shapes with the cost unpriced. The committed
harness family `satfold-const-gate` prices exactly that question and is quoted in my section 5: the arm
reached through a const verdict computed by an exhaustive const-fn sweep is at 1438 ns median against
1456 ns for the same arm reached directly, with overlapping intervals, and the false-verdict gate
selects the fallback at 38391 ns. **W1 and W2 are priced and the answer is that the gate is free.**
`OPTIONS.md` Q42 records the same thing more strongly from `92`, that the gated timed region is
byte-identical to a size-matched ungated control.

## 14. Where I think op's two readings pull apart, which `88` invites

`88` section 1 says a later expert finding the two readings pulling apart has found something real.
Here is where I think they do, and it is the whole of my section 4.

**Under the point reading, cross-strategy resolution is a componentwise join and it is total, lossless
and lawful.** Two points in a product of chains combine coordinatewise; the join is commutative,
associative and idempotent for free; and P3 compiles it on the pin with no forbidden feature, 81 of 81
pairs agreeing with an independently written const-fn join, and the mutant confirming the check can
fail.

**Under the weighting reading, there is no join at all.** Two weightings over cost axes are
incomparable vectors, and P1b Part B is the arithmetic: four presets each carrying one demand leave 12
of 16 ordered pairs unresolvable, and the smallest set closed under their own resolution has 15
elements. Nothing about that is fixable by choosing a better table, because it is a property of the
set.

So the two readings agree on nearly everything and give **different answers to what `x + y` means when
`x` and `y` carry different strategies**. That is a concrete place to look rather than a general worry,
and it is checkable: P1's enumeration and P1b's closure count are both one command.

**And the resolution the disagreement suggests is op's own ranking applied.** If the point reading is
"mostly it", then resolution belongs on that side: componentwise over the policy coordinates, where the
join exists and is lawful. The weighting reading is what *names* a point rather than what gets joined,
and a weighting is never resolved against another weighting because nothing ever asks it to be. That
also matches the shape of my phase-one section 7: `Resolve` survives as a componentwise join over the
policy layer only, and the lowering layer needs no resolution because it is unobservable.

Stated in the register's own terms, and this matters because `never-ask-which-single-rule-governs.md`
would otherwise catch me: this is **two arms with two predicates**, not one policy for a category. On
the policy coordinates, join. On the cost coordinates, no resolution, because there is nothing
observable to resolve. Op's rejection at `88` section 4 is of a question asking for one rule over a
whole category, and the answer here is per region rather than universal.

## 15. What I withdraw

**The notko evidence, though not the structural argument it supported.** My section 3 used notko's
three tiers against arvo's four as evidence for the policy-lowering seam. Op has ruled directly against
using notko that way, quoted at `25` section 3.3 from `144b:10-16`:

> Notko or hv are not directly associated with arvo. The concepts need not align, they are different
> things for different purposes and in different projects. They have synergy, but no continuity as such.

> Again, arvo strategy is not the same as notko optimize for profiles. They have synergy, nothing more.

And `25` section 5.1 adds that a notko tier file being unable to express an arvo posture "is the
separation working rather than a gap". So the 3-versus-4 mismatch is two unrelated designs having
different cardinalities, not a seam, and I withdraw the inference. `94` section 7 reaches for notko the
same way for the same structural point, so this is a correction to both of us.

What survives untouched is the argument itself, which never needed notko: a mechanism scoped to a
region of source can carry a preference and cannot carry a denotation, because a denotation belongs to
the values rather than to where they were written. That stands on its own.

What also survives is the **vocabulary-collision report** in my section 10, because it is not a claim
about correspondence. Two crates one dependency edge apart both export a type called `Strategy`, and
three of arvo's four marker names are also notko's tier names for a different axis. That is a cost to
every reader of both crates whether or not the concepts correspond, and op's ruling that they do not
correspond makes the shared spelling worse rather than better.

**That the preference sits on the value's type.** My section 5 argued the type parameter must name the
preference rather than the resolution, and discussed it in terms of `UFixed<I, F, S>`, which put the
preference on the value. `94`'s W9 has evidence against that and I have none for it: policy on the
value, cost and licence at the site, four sites and three of them sharing one value type, zero
conditional instructions and zero casts, with the same value type folding three different ways at three
sites. Its argument is the one I should have made: "Which arm is cheapest depends on this loop's arity,
this target's features, this access pattern. The value has no opinion about any of that."

And my own P4 actually did it `94`'s way without my noticing: `fold::<PrefSpeed>(xs)` puts the
preference at the **call site**, not on the value. I described a site-carried preference as a
value-carried one. The preference-not-resolution half of the claim stands; the value-versus-site half
was never established by me and `94` has the better evidence.

**That the resolution is "computed from whatever is available at const time", as a complete
description.** `94` section 3.2 corrects it and the correction is right: a const function cannot
measure anything, so the objective is evaluated offline on the harness by a person, and what is const
is the predicate naming the region the answer came out in. `40` section 3.2 says the same thing and
draws the tier boundary from it: the objective is canon, the table is design, the marker is code.

I would add one thing rather than only conceding, because there is a real fork inside the correction.
**Two encodings exist and they are not equivalent in what they let a consumer do.** `94`'s W1 bakes the
*winner* per region as an associated const, which is simple and is what a table of measured results
naturally becomes. My P4 bakes the *cost table* and computes an argmin at const time, which also erases
completely. The difference is what a consumer can bring: under the second, a weighting nobody named
selects an arm nobody tabulated for it, which is `arvo-toolbox-not-policer.md`'s posture; under the
first, a consumer gets the weightings somebody wrote down. Neither is obviously right and the register
does not carry the fork.

**The prose around F2, though not its predicate.** I wrote "additive associativity is insensitive to
every axis varied here". The predicate said `signedness = unsigned` and was therefore correct, but the
sentence reads wider than the predicate licenses, and P7 now shows signed saturating addition failing
associativity on 24.8% of triples at `W = 7`. The finding was right and the sentence around it was
sloppy, which is exactly the failure the predicate notation exists to prevent and which it did prevent.

## 16. Two probes built in phase two, and what they settle

### P7. Signedness against the congruence argument

Phase one named its own largest gap: everything swept was unsigned, and F1a's proof that the laws hold
at `F = 0` for any width rests on a collapse being a semiring congruence, argued only for the one-sided
case. `25` section 5.3 reports, from the prior-art memory rather than from a measurement, that signed
two-sided saturating addition is not associative.

`93_probes/p7_signedness_breaks_the_congruence.out` settles it, exhaustively over the whole signed
domain at `W in 3..7`, both policies, `F = 0` throughout so signedness is isolated:

| configuration | add-assoc | mul-assoc | distributivity |
|---|---|---|---|
| signed, wrapping | holds at every swept width | holds | holds |
| signed, saturating, `W = 7` | fails 24.80% | fails 0.72% | fails 47.72% |

The prediction was made from the congruence argument before running: collapse everything at or above
MAX onto MAX, so MAX and MAX+1 are identified; multiply both by -1 and get -MAX and MIN, which are not
identified; so multiplication does not respect the collapse. A one-sided clamp has no such escape,
which is why unsigned saturation is a semiring and signed saturation is not.

**F10.** Signed wrapping satisfies commutativity, associativity and distributivity for both operations;
signed saturating satisfies none of the three.
`holds for: W in 3..7, F = 0, signedness = signed, overflow in {wrap, saturate}, operations {add, mul},
arity 3, threads = 1, target features any`

**What this does to the panel's existing boundary claim, precisely.**
`arvo-always-optimal-internals.md` carries the panel's result as "multiplicative associativity and
distributivity hold exactly at `F == 0` and fail everywhere else", with the supporting sentence "every
holding cell has `F == 0`". P7 does not contradict that, because it adds a non-holding cell at `F = 0`
rather than a holding cell above it. What it does is **narrow the converse**: `F = 0` is necessary and
is not sufficient, and the rest of the condition is signedness together with the overflow policy. A
reader taking "holds exactly at `F == 0`" as a licence to reassociate any `F = 0` numeral would be
wrong on signed saturating, on 47.7% of triples for distributivity.

### P8. Q41, which the register records as never engaged

`OPTIONS.md` Q41 asks whether the strategies are partially ordered by how many chain-level laws they
honour, offered by `76` as an explicit falsifiable candidate with the accuracy-first intent at the top,
dropped by the consolidation `90`, recovered by `91`'s check, and never engaged by any member of that
unit. It is testable exactly the way `76` said, and
`93_probes/p8_q41_do_the_honoured_law_sets_nest.out` runs the test: an eleven-law inventory decided
exhaustively per configuration, then the honoured sets compared for containment.

The inventory mixes two families on purpose. **Algebraic**: the identities a rewrite needs, including
the clamp-early retraction a fold needs before it may be split. **Order**: monotonicity of addition and
absorption at the top, which is what a tropical or min-plus algorithm needs.

| configuration | laws honoured of 11 |
|---|---|
| unsigned, wrapping, `F = 0` | 9 |
| unsigned, saturating, `F = 0` | **11** |
| signed, wrapping, `F = 0` | 9 |
| signed, saturating, `F = 0` | 7 |

**F11. Q41's three options are answers on different regions, and the predicate separating them is the
signedness.**
`holds for: W = 5, F in {0, 1}, signedness in {unsigned, signed}, overflow in {wrap, saturate},
operations {add, mul}, arity 3, law inventory as listed, threads = 1, target features any`

Unsigned: the sets nest, saturating honours everything wrapping does plus monotonicity and top
absorption, and Q41's option (a) holds with saturating on top. **That is `76`'s conjectured direction,
established rather than conjectured**, and this is the first evidence for it either way.

Signed: the sets stop nesting. Saturating still gains monotonicity and absorption and now loses
additive associativity, multiplicative associativity, distributivity and retraction, for P7's reason.
Q41's option (b) holds: a real partial order that is not a ladder, with ten incomparable pairs listed
in the probe's output.

**So there is no single ordering over "the strategies", and the reason is nameable rather than a
shrug.** The algebraic family and the order family agree on unsigned and conflict on signed. That is a
second instance of a phenomenon `DROPLIST.md` already records once, in the entry retiring the
`AddAssoc` gate on the algorithm crates: "associativity and the distributivity these algorithms need
are different, complementary laws that invert across the same presets."

What P8 does not reach, and by the notation therefore does not claim: `76`'s own phrase was chain-level
**accuracy** facts, a third family that is not in this inventory and that orders these configurations
by how little they lose rather than by which identities they satisfy. On that family the order plausibly
runs `76`'s way by construction. Nothing here measures it.

## 17. What my findings do to the live register

**Q15, are the axes independently resolvable and in what order.** Its second option is "resolvable in a
stated order, earlier coordinates fixed before later ones are measured. Coherent with the observable
split if observable axes come first", and the entry's own cost line says the ordering is something "the
canon must then justify rather than assert".

**My section 3 is that justification.** The ordering is derivable rather than a convention: the
lowering layer's candidate set is *defined* by which rewrites the policy layer's choices make sound, so
the policy coordinates cannot be settled after the coordinates that depend on them. P2b, P2c, P7 and P8
are the measurements that make it concrete, since each shows a law's truth value moving with a policy
coordinate and never the reverse. `40`'s `p7` reaches the same conclusion empirically from the other
end, finding the contending container set differing between wrapping and saturating at 5 of 6 widths.
**Two instruments, one from arithmetic and one from committed harness output, and they agree.**

**Q13, which axes may a build arm move.** My F1, F10 and F11 are direct evidence for the observable
side of `40`'s split being real rather than definitional: an arm that moves the overflow coordinate
moves which rewrites are sound, so it moves which arms are available on every coordinate downstream.
That strengthens the second option, unobservable axes only, and prices part of what the first option
costs.

**Q41.** Answered above. It should move from "never engaged" to carrying F11.

**Q14, the exchange rate.** My P1b bears on it obliquely and I want to say how, because it is not
obvious. If a strategy is a weighting, the exchange rate is *part of the weighting* rather than a
separate unset parameter: a lexicographic term is a weighting with an infinite ratio and a finite term
is one with a finite ratio, which is `40` section 5.3's reading. So Q14's "a lexicographic ordering with
no rate at all" and "a stated rate per objective" are the same option at two values of one parameter,
not two designs. That does not set the rate and it does reduce the option count.

**A question with no entry, which I would propose adding.** Cross-strategy resolution. Q27 names it
(`OPTIONS.md:1655-1656`: "Which strategy's laws govern a cross-strategy operation is adjacent to that
and not the same question") and hands it nowhere, Q3 is about mixed *numerals* rather than mixed strategies, and no
entry anywhere carries it. `mock/DESIGN.md.tmpl:35` names a mechanism for it, `Resolve<S1, S2>`, and
`arvo-toolbox-not-policer.md` describes its intended behaviour. The options, with what distinguishes
them, are my phase-one section 4:

- **A componentwise join over the policy coordinates**, total and lawful by construction, with the
  named presets as points. Compiles on the pin (P3). Costs: the strategy space is a product rather than
  a list, so every "what does this strategy do" question is answered per coordinate.
- **A join over a flat named set.** Exactly four semilattices on four markers satisfy the six
  intent-derived constraints and all four escalate every mixed expression to the accuracy-first preset
  (P1, P1b Part A). Costs: two operands neither of which asked for accuracy produce the most expensive
  policy in the set, and 12 of 16 demand pairs are unresolvable whatever table is chosen (P1b Part B).
- **No implicit resolution; the consumer names the result strategy.** Sits awkwardly against
  `arvo-toolbox-not-policer.md`'s list of incorrect shapes, though I read a refusal that names a
  genuine conflict as the diagnostic that rule wants. Costs verbosity at every mixed site.
- **The question is a category error, because the demands are on different roles.** Storage at rest
  against compute in flight, so `Cold + Hot` is a pipeline rather than a join. The bench matrix already
  factors that way (`warm-clamp-arity-w13`'s own title benches the accumulator's container separately
  from the stored operands').

What distinguishes them: how often mixed-strategy arithmetic occurs in consumer code, which is a fact
about hilavitkutin and vehje; and whether the storage decision and the compute decision are in fact
chosen independently in real workloads, which the bench matrix suggests they are.

**And the register correction I would flag rather than make**, since editing it is not mine: `40`
section 4.3 already says Q5's first corroboration should be struck. Nothing I found bears on that
either way; I note it because a reader of Q5 arriving after `40` will otherwise count four.

## 18. Two things I now know are weaker than phase one implied

**My section 5's bench citation sits next to a flagged arm.** I cited the existence of the matched pair
`warm-container-width-l1` against `precise-container-width-l1` as evidence that the shipped design
already stratifies, benching one lowering question twice, once per policy. `40` section 6.5 flags
`precise-container-width-l1`'s `kernel` arm as returning a flat 63 to 68 nanoseconds at every declared
width against a field two orders of magnitude above it, which is the signature of work that was removed
rather than work that was fast, and reports that no committed bench run in the repository cross-checks
that its arms computed the same answer, the digest column being zero across 214 CSVs and 82,960 rows.

My use survives, because I cited the *existence and shape of the pair* rather than its numbers, and the
existence of two matched families is what makes the stratification point. But anybody reading my
section 3 as though those two families' results were comparable should read `40` section 6.5 first.

**And the same risk applies to my own P4, which is why I checked it.** An emitted-code comparison of
two functions proves nothing if they compute different things. P4's `main` runs both pairs on a
thousand real values and prints the equality (`direct_wrap == pref_speed` and
`direct_widen == pref_accuracy`, both true), so the assembly comparison is between two functions known
to agree. That check exists because the failure mode `40` found is the obvious one, and I record that I
ran it rather than assuming it.

## 19. What I would hand back now, revised

Phase one handed back two tensions. Both survive, and one has moved.

**T1, I3 against I15**, that Rust's debug-overflow panic is a runtime check and therefore permanently
unavailable to an imitation-defined strategy, survives untouched. F9 names the region where something
equivalent exists, which is the const-available one.

**T2, which of the two readings of "behave like a native primitive" is meant**, survives and is now
smaller than I thought, because `40` section 6.3 supplies an argument I did not have. Under the
weighting model a preset is silent on an axis exactly where its objective is indifferent to it, and
`40` checks that reading against the four silences the record actually has: one explained, one refuted,
two open. `Warm`'s silence on intermediate precision is one of the two open ones. So T2 is a live
instance of a pattern the register already tracks rather than an isolated ambiguity, and it should be
asked as part of that rather than on its own.

**And one thing I would now not hand back.** Phase one implied the cross-strategy resolution question
might be op's. It is not, or not yet: `RULES.md` says a question is brought to him converged, with the
angles considered and the alternatives laid out, and this one has had exactly two files on it, both of
them mine and `94`'s, both of which concede. It is a panel question with four options and a
discriminator, and it should stay in the register until somebody either builds the arm or reads the
consumers.

## 20. Coverage of phase two, stated rather than claimed

I read six panel files in full or near enough, and eleven register entries. I did not read the
consolidations, the seeds, the prior calls, or fifty-odd member files, so any of the above may restate
something I have not seen. Every citation in this section was opened before it was written; the two
that were off by a line or two when I first noted them were corrected against the file rather than from
memory, which is the same failure `25` section 9 counted seven instances of in its own file and built a
script for.

I did not run the bench harness in phase two either, so `satfold-const-gate` remains the only priced
thing I cite and I cite it as somebody else's measurement.

The one number I would most like checked by someone else is P8's law inventory. Eleven laws is a
choice, and a different eleven could nest differently. The result that unsigned nests and signed does
not turns on monotonicity and top absorption being in the inventory at all, and those are in it because
`DROPLIST.md`'s `AddAssoc` entry says the algorithm crates need them. If somebody thinks a different
inventory is the right one, the probe takes a new law in about five lines and the answer may move.
