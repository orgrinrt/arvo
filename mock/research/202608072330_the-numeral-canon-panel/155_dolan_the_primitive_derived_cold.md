# 155. The primitive, derived cold

Dispatched as file `155`. This is a cold derivation: phase one below was written before
reading any panel file, `AGREEMENTS.md`, `OPTIONS.md`, `DROPLIST.md`, `HANDLES.md`,
`PRIOR_CALLS.md`, `PERSONA_CALLS.md`, any `SEED_*` file, anything under `archive/` or
`seed/`, any other member's probe directory, `git log`, or any commit subject. Phase one
is committed before any of that is opened, and it is never rewritten afterward.

**Interruption note.** This dispatch was cut once by a network error before any content
existed on disk (state: nothing of mine had been written). Resumed with the same brief,
same constraints. I had not read anything outside the permitted premise list before the
cut; the only actions taken before the interruption were reads of the permitted premises
(`INTENTS.md`, `RULES.md`, this repo's `.claude/CLAUDE.md`, `mock/Cargo.toml`,
`rust-toolchain.toml`, the `mock/benches/` layout, `bench.toml`, and several bench
variant/shared source files) and one committed probe. Declaring it here per the resume
instruction, even though nothing needs correcting: no panel material was touched.

## What I read for phase one, and by what method

In full: `INTENTS.md` (395 lines), `RULES.md` (638 lines), this repo's `.claude/CLAUDE.md`
(the generated agent instructions, which is explicitly one of the permitted premises),
`mock/Cargo.toml`, `rust-toolchain.toml`.

Directory structure of `mock/benches/`: `find`, `ls`. Full text of `bench.toml` (49
`[bench.*]` sections, `grep -c '^\[bench\.'`), and its section headers.

Source read in full: `mock/benches/variants/warm-container-native/src/lib.rs` (57
lines), `mock/benches/variants/warm-container-shared/src/lib.rs`, all 1769 lines (the
first pass stopped at line 1359 on a context-size truncation; the remainder, the
macro-boilerplate arm-declaration tail and the full `tests` module, was read afterward,
in phase one, since it is the same permitted file). `mock/benches/Cargo.toml` in full,
including its comment about why `structural-decomposition` and `spectral-bisection` are
absent from the bin's own dependency list.

Source read in part, first 60-90 lines each, for orientation on a specific concept:
`mock/benches/variants/wide-rung-shared/src/lib.rs`, `.../shape.rs`, `.../column.rs`
(the wide, multi-limb numeral shape), `mock/benches/variants/bitpack-carrier-shared/src/lib.rs`
(the packed, sub-word numeral shape), `mock/benches/variants/warm-clamp-shared/src/lib.rs`
(the fold/arity/accumulator-width shape), `mock/benches/variants/quantiser-fadd-shared/src/lib.rs`
(the float-quantiser shape, which turned out not to bear on this question directly and is
noted rather than used).

Three probes, committed at `155_probes/` with their output alongside. `01_fraction_axis_grep.sh`:
a grep sweep across every `mock/benches/variants/**/src/*.rs` asking whether the
committed bench corpus ever sweeps a fraction-width axis (`F`, distinct from the declared
width `W`) or a signedness axis as a first-class `KEY` field, the way it demonstrably
sweeps `W`, element count, operation, and density. Positive control (`key_w` must be
found) passed at 3 hits. Both target searches returned zero genuine hits (the one
"signed" hit found by a broader manual follow-up is an oracle's internal arithmetic
detail, not a swept identity axis). **This is a real finding and it bounds what follows**:
everything below about fraction-width composition is inference from a workspace rule's
citation of panel-established results, not from anything I could verify against a
committed bench artifact myself. I say exactly where that inference enters. `02_warm_container_shared_test_gate.out`:
the full `cargo test --release` transcript for `warm-container-shared`, 15/15 passing,
run as the test gate (see below). `03_shared_test_counts.out`: the per-crate `#[test]`
counts across the thirteen `-shared` crates, summing to 124.

Not read, and named so the gap is visible: the bodies of `bitpack-carrier-shared` past
its header comment, `satfold-shared`, the full `wide-rung-shared` beyond the three files
skimmed, all of `quantiser-radix-shared`. None of the committed `.csv` / `.meta.json`
/ `_findings.md` artifacts (I read directory listings and file names only, not their
content; the finding below about carrier choice being an internals decision rests on the
*existence and structure* of the arm comparison, not on which arm won).

## The two gates, run before the assigned work

**Canon gate.** The work I am doing (deriving what a primitive is, from the intent
catalogue and from committed bench evidence, without presuming the removed crate tree)
aligns with `INTENTS.md` and `RULES.md` as read above, and with this repo's own
`.claude/CLAUDE.md`, which states the same posture in its own voice ("Do not reason from
the old architecture... Everything about the decomposition is open"). Nothing in the
brief asks me to build or assume anything the intents forbid. Aligned; proceeding.

**Test gate.** The bench variant crates under `mock/benches/variants/` are the
suite-bearing surface here, and I both read test bodies and ran a suite rather than
trusting names or counts.

I read the full body of every test in `warm-container-shared/src/lib.rs`'s `tests`
module (15 tests, `grep -h -c '#\[test\]' mock/benches/variants/warm-container-shared/src/*.rs`)
and ran it: `cargo test --release` from
`mock/benches/variants/warm-container-shared/` (a standalone crate, no workspace ancestor
claims it: `mock/Cargo.toml` excludes `benches`, and `mock/benches/Cargo.toml` and this
crate's own `Cargo.toml` both carry no `[workspace]` table). Result: **15 passed, 0
failed, 0.18s.**

None of the fifteen is decorative by the standard `the-test-gate.md` sets. The load-bearing
one (`all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`, cited in
section 1) asserts pairwise equality of all six arms *and* equality against an
independently-coded oracle, over the whole declared `ALL_KEYS` matrix, not a sample. Two
of the fifteen are explicit negative controls, exactly the shape `the-test-gate.md` and
this panel's evidence rules ask for and which is usually the thing missing:
`the_oracle_is_sensitive_to_a_perturbed_column` (would fail if the oracle were
accidentally constant, which is precisely the failure mode `wide-rung-shared`'s own doc
comment names, about a sibling family: "the answer did not depend on the input, so an arm
that read no data at all would have passed", `wide-rung-shared/src/lib.rs:54-55`) and
`validate_output_refuses_a_wrong_sum` (checks the validator itself can fail, not only that
it can pass). `diag_sat_lanes_actually_runs` reads, from its name and the file's own
docstring about a sibling constant-folding failure (`:667-679`), as the same class of
control applied to the lane-parallel kernel. `key_encoding_round_trips_over_the_whole_declared_matrix`
is a law asserted over the whole matrix rather than a sample, which is the specific thing
`the-test-gate.md` names as most commonly faked by sampling.

I did not read every test body in every `-shared` crate; that would exceed what phase one
needs. I counted instead (`grep -h -c '#\[test\]' mock/benches/variants/*-shared/src/*.rs`,
summed): **124 tests across the 13 `-shared` crates.** I spot-checked test *names* across
all thirteen for the same negative-control vocabulary (`grep -lE
"sensitive|refuses|actually_runs|actually_computes"`) and found it present in at least
one file in nine of the thirteen. That is a naming-pattern check, not a body read, and I
say so rather than letting it stand in for one: it is evidence the negative-control
discipline is a house style across this bench directory, not proof that every one of the
124 is non-decorative. The one crate I fully audited (`warm-container-shared`) passes
cleanly; I extend that as a reasonable prior over the rest of the directory, not as a
verified fact about it.

## The question

What is a primitive, in arvo? What it names, what (if anything) it composes and out of
what, what varies within one and what distinguishes two, what a name buys over an
unnamed construction, and what the tiers above need from it to be statable at all.

## The premises, restated in the terms I will use

`INTENTS.md` never says the word "primitive" is settled vocabulary; it uses it four
times, all inside quotations of op's about `Warm`'s imitation of "native Rust
primitives" (`INTENTS.md:74-101`), which is about the strategy `Warm`'s ergonomic target,
not a definition of what arvo's own primitives are. So the question is genuinely open,
and I derive an answer rather than look one up.

What is fixed, from the permitted premises:

- **I3/I4** (`INTENTS.md:74-108`): a strategy's imitation of a native type is about
  ergonomics, not about where arithmetic boundaries land; the boundaries are the width
  and the overflow policy.
- **I8/I9** (`INTENTS.md:143-177`): the strategies weigh different measurements
  differently, and the strategy is what makes an answer *correct* for a given
  computation, not a decoration on top of a fixed answer.
- **I13** (`INTENTS.md:214-266`), RATIFIED: the work is predicated arms, composed. No
  universal solution. A predicate is over what is available at const time, including
  const functions and const data, with the typestate usable inside it but not the only
  source.
- **I14** (`INTENTS.md:268-297`), IN FORCE: `#![no_std]`, no `alloc`, const sizes,
  monomorphisation is the dispatch (no `dyn`, no `TypeId`), public API positions use the
  stack's own primitives rather than bare integers.
- **I15** (`INTENTS.md:299-315`): never a runtime check, ever. Branching is const-time,
  erased by monomorphisation, so everything reaches one lowered path.
- **I16** (`INTENTS.md:317-331`): the canon does not police the *shape* a law's proof
  takes (typestate, const expression, whatever); it only requires that the law actually
  hold and reach one lowered path.
- **I17** (`INTENTS.md:363-380`): Cold (the storage/bitpacking-minimising concern) is not
  to be deprioritised, independent of how the strategy set is finally named or sized.
- **I11** (`INTENTS.md:190-197`): arvo is a library; the value is the algo crates and
  contracts downstream code builds on top of it.

## Derivation

### 1. The bench corpus already builds and prices a distinction the vocabulary does not yet name

`mock/benches/variants/warm-container-shared/src/lib.rs:180-209` declares a `Carrier`
trait (`BITS`, `ZERO`, `ONE`, `MAX`, wrapping/saturating ops, `mask_to`) implemented for
`u8`/`u16`/`u32`/`u64`/`u128` (`:279-283`). Every one of the six arms declared at
`:1253-1322` (`headroom`, `minimum`, `plusone`, `kernel`, `lanes_deferred`, `native`) is
parameterised by a **declared width `W`** and a **carrier `C`**, and the file states
plainly what the fork is (`:1-11`): the shipped rule picks `C` one rung above the minimum
that holds `W` for `Warm`/`Precise`, and the minimum rung for `Hot`/`Cold`.

The load-bearing test at `:1355-1396` (`all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`)
asserts that every arm, for every key in `ALL_KEYS` (57 entries, `:1326-1332`), computes
the byte-identical `u64` result, cross-checked against an independent `u128` reference
(`reference`, `:845-921`) that shares no code with any arm's `Carrier` impl. That is not
incidental scaffolding; it is the operational definition of "these six things are the
same primitive": **six different carriers, two different projection schedules (eager
vs. once), two different accumulation shapes (single accumulator vs. eight independent
lanes), all required to produce one value for one declared width under one semantics.**

`wide-rung-shared` runs the identical pattern one register above the native width: a
numeral of declared width `W > 64` is realised as `limbs_of(W) = ceil(W/64)` 64-bit
limbs (`shape.rs:19-21`), stored **ragged** (exact bit count, `rag_bytes`), **word-rounded**
(whole limbs, `wr_bytes`), or **16-aligned** (`a16_bytes`), three shapes the doc comment
(`lib.rs:5-9`) states are the ratified rule's split between `Cold`/`Precise` (ragged) and
`Hot`/`Warm` (word-rounded), with the alignment shape as a fourth, currently-unpriced
candidate.

`bitpack-carrier-shared` runs the same pattern below one native word: several sub-word
values packed into one machine word (13 bits into a 64-bit slot alongside others),
compared against dense storage at three different native widths (`u16`/`u32`/`u64`) the
consumer might otherwise have reached for (`lib.rs:1-27`).

**These three benches are the same fact measured at three different points on one axis.**
The realisation of a declared-width numeral is not fundamentally a discrete choice among
a fixed list of machine types (`u8`..`u128`). It is a placement of some number of bits,
and the three regions differ only in whether that placement is below one machine word
(pack several values in), at one machine word (pick a rung), or above one machine word
(concatenate limbs). All three are governed by the same two forces the strategy axis
already names: `Hot`/`Cold` want fewer bits touched (minimum rung, ragged shape, tighter
pack), `Warm`/`Precise` want headroom for a chain of operations to retain more than one
step's exactness before a narrowing forces a decision (`warm-clamp-shared/src/lib.rs:16-19`,
quoting a ratified rung rule I did not open the citation for, but the *reasoning it
states* is corroborated independently by what `warm-container-shared` prices).

### 2. Two things are being conflated by one word, and the bench corpus keeps them apart on purpose

Every one of these benches holds one thing fixed across its arms and varies another, and
the harness's own cross-comparison test exists specifically to *prove* the fixed thing
stayed fixed while the varied thing moved.

**Fixed across every arm in a bench:** the declared width `W`, and the semantics
(wrapping-modulo-`2^W` or saturating-clamp-to-`[0, 2^W)`, called `OP` in the key). This is
what determines the *value* an operation must produce. Call this the numeral's **identity**.

**Varied across the arms of one bench:** which native carrier holds the value, how many
bits of headroom it carries above `W`, whether the projection back to `W` happens after
every operation or once, whether the fold is a single accumulator or eight lanes, whether
a saturating accumulate at width `W` is checked against `W`'s own limit or is provably
below the range where the check can even fire (the "theorem" arm at
`warm-container-shared/src/lib.rs:608-663`, and the "witness" arm at `:440-478`, both of
which delete a check because the typestate's own bound proves it dead). Call this the
**realisation**.

The harness's byte-for-byte agreement requirement, run over every declared key, is
exactly the machinery that makes this distinction checkable rather than asserted:
identity is what the oracle is compared against; realisation is what varies and gets
timed. A design document could assert "the carrier is an implementation detail" as a
sentence; this bench corpus asserts it as a passing test that would fail the instant a
carrier change altered the observable value.

So: **a primitive is not the carrier, and the carrier is not the primitive.** A `Warm`
13-bit unsigned numeral realised in a `u16` and the same numeral realised in a `u32`
(headroom rung) are the same primitive under two different arms, exactly as `headroom`
and `minimum` in the bench are two arms of one contract. What names the primitive is the
identity: the declared width, the semantics that follows from the strategy, and (I3/I4)
the intuitive-imitation posture attached to whichever strategy governs it. What realises
it is chosen, per instantiation, by whatever the bench evidence says is optimal for the
target and the workload, which is precisely `arvo-toolbox-not-policer.md`'s "the
substrate ships the choice rather than making it" applied one level below the strategy
axis rather than at it, and it is precisely what I13's "predicated arms" mechanism is
for: `headroom` and `minimum` are two arms, the predicate that selects between them is a
function of `W`, of the strategy, and (per op's I13 addendum, `INTENTS.md:248-261`) of
whatever else is available at const time, and the composed answer ("everywhere is
optimal") is the compile-time selection among these arms, not a single universal carrier
rule.

### 3. What composes, and out of what

Given the identity/realisation split, two different composition questions have two
different answers.

**The identity composes.** A declared width in these benches is a single quantity `W`
(integer bits, all benches in the committed corpus operate at `F = 0`; see the probe
result and the honesty note in section 5). A fixed-point primitive's identity, per the
vocabulary this workspace's own rule `arvo-always-optimal-internals.md` states and which
I am licensed to read as part of the repo's own rules, is `(I, F, S)`: integer bits,
fraction bits, strategy. That composition is load-bearing for *which laws hold*, and I
went back and read the rule's own text precisely rather than trust my first paraphrase
of it, which had overstated it. The accurate statement, quoting the rule directly: `F ==
0` is *necessary but not sufficient* for multiplicative associativity and distributivity
("every measured cell where they hold has `F == 0`"), distributivity holds in only 6 of
33 measured cells, and even inside the `F = 0` region a law permission has to name the
specific operation: at unsigned `F = 0` saturating, distributivity over addition holds
and distributivity over subtraction fails at 45.79% of triples at `W = 6`. So the
composing identity that actually gates law availability is not the bare triple `(I, F,
S)`; it is `(I, F, S)` together with signedness, the overflow policy, and the specific
operation named. I did not verify any of this myself against a bench artifact (the probe
found no `F` axis anywhere in the committed corpus), so I record all of it as inherited
from a permitted premise rather than as something I established. What I can say from the
benches directly: `W` alone (no fraction split, no signedness) is sufficient to determine
everything the committed corpus prices, because every swept case is an unsigned pure
integer. Whether signedness and fraction each compose as independent orthogonal axes of
the identity, or whether one is derived from the other, is not something this evidence
settles; I flag it as open in section 6.

**The realisation does not compose cleanly out of the identity as a deterministic
function of it.** It is *constrained* by identity (a `Hot` 13-bit numeral's realisation
must be some carrier or packing that holds at least 13 bits and computes the wrapping
semantics correctly) and it is *chosen* among the constrained options by bench evidence
(which of `headroom`/`minimum`/`plusone`/etc. is fastest, or smallest, or uses fewest
bytes, at this width, on this target, for this workload). The word "composition" fits the
identity axis (I and F genuinely combine to produce W and to gate which laws are
available); it fits the realisation axis only loosely, as "selection among a
predicate-gated arm set," which is a different relation. Forcing one word onto both is
part of why "what does a primitive compose" reads as a harder question than it is: the
honest answer is "the identity composes, the realisation is chosen," and conflating them
is the same conflation section 2 already names.

**One further composition is visible and is worth naming separately: aggregation.** A
column of primitives (`Cols` in `warm-container-shared/src/lib.rs:762-767`, `Column` in
`wide-rung-shared/src/column.rs:33-38`) is itself a construction whose shape (stride,
alignment, region layout) is *entirely derived from* the primitive's declared width and
chosen realisation, never from anything the column itself introduces. A bitpacked column
of 13-bit values and a dense `u16` column of the same logical values are two realisations
of the identical logical column, exactly one level up from how two carriers are two
realisations of one numeral. This suggests the identity/realisation split is not specific
to a single scalar numeral; it recurs at the container level, and a design that names it
once, generically, rather than once per level, is smaller. I did not verify this
generalises past what the two benches (`warm-container-shared`, `bitpack-carrier-shared`,
`wide-rung-shared`) directly show, so I state it as a pattern observed at the levels
tested rather than as an established generalisation.

### 4. What a name buys, and what an unnamed construction does not get

Three things the bench corpus and the intent catalogue together make concrete, none of
them available to a bare `u32` used ad hoc:

**A name is what a const predicate (I13) can be written over.** `I13`'s addendum
(`INTENTS.md:248-261`) says the predicate reaches "whatever is available at const time,"
with the typestate usable but not the only source. A bare native integer carries no
compile-time fact narrower than "this many bits, native." It cannot be the input to a
predicate distinguishing a `Warm` 13-bit value from a `Hot` 13-bit value from a plain
`u16`, because nothing in the type says which of those it is. The identity is exactly the
information I13's mechanism needs to have named, in the type, before it can select an
arm. Section 2's arm selection (`headroom` vs `minimum` vs the theorem-gated forms) is
literally op's "predicated arms" mechanism, already built and priced, and every one of
its predicates is a function of the identity (`W`, and in the widening-theorem case
(`:650-663`) also the element count `n`, which is not part of the numeral's own identity
but is carried by the column that holds it, i.e. `Cap`, per the same rule's own comment
at `:619-623` naming this exact fact).

**A name is what lets I15 hold at all.** "Everything reaches one lowered path" requires
something for the compiler to monomorphise over. A value that is only ever a `u32` at
runtime, with its declared width, strategy and semantics tracked nowhere in the type, has
nothing for monomorphisation to specialise on; every branch that would otherwise be
erased at compile time (which projection schedule, which theorem, which carrier) becomes
either a runtime check (forbidden by I15) or an ambient assumption nobody can verify. The
name is what turns "this is a 13-bit `Warm` value" from a comment into a fact the type
checker enforces and the optimiser can act on, which is exactly what the arms in
`warm-container-shared` demonstrate is being folded away: every method on `Carrier` is
`#[inline(always)]` specifically so that a caller passing a constant width folds the
width-dependent branches (`:181-186`), and every one of the arm macros
(`declare_arm!`/`declare_kernel_arm!`, `:1206-1241`, `:1169-1202`) exists to guarantee
that after inlining, nothing in the timed path branches on `W`, `D`, or `OP` at runtime
(`:1017-1027`'s comment states this is a deliberate consequence of `generic_const_exprs`
being forbidden and the width having to be recovered as an ordinary matched value that
folds after inlining, rather than as a const-generic arithmetic expression).

**A name is what lets the tiers above be generic over "a primitive" rather than over one
concrete carrier.** This is the direct answer to the "what do the tiers above need"
half of the question and is developed in section 5.

### 5. What the tiers above need

I11 (`INTENTS.md:190-197`) states plainly that arvo's value is the algo crates and
contracts that compose on top of it, not arvo used standalone. For that composition to be
generic (rather than a pile of per-carrier special cases hand-written per consumer), the
tier above needs three things from a primitive, each visible as a requirement the benches
already had to satisfy to be buildable at all:

1. **A fixed, nameable identity usable as a type parameter, not a runtime value.** I14
   forbids `dyn`/`TypeId`; monomorphisation is the dispatch. A generic fold, accumulator,
   or graph-weight computation written once against "a primitive of width `W` under
   strategy `S`" has to be able to name that contract in a trait bound or a generic
   parameter, the way `warm-container-shared`'s `Carrier` trait names the realisation
   contract every arm is generic over (`:187-209`). The identity is the corresponding
   contract one level up, for the *value*, not the container: something a consumer names
   once (declared width, split, strategy) and every downstream algorithm is generic over.

2. **A law set that is quantified over the identity and available at compile time.** The
   interior-safety predicate in `warm-clamp-shared/src/lib.rs:41-47` ("a fold of arity `n`
   over destination numeral `N` with accumulator numeral `M` is interior-safe when
   `(n-1) * [min V(N), max V(N)]` is contained in `[min V(M), max V(M)]`") is exactly the
   shape a downstream algorithm needs: a fact that is true or false as a function of the
   identities involved, checkable at compile time (per I15, never at runtime), that lets
   the algorithm elide work (skip the interior clamp) when the fact holds and keep it when
   it does not. Nothing about this predicate is specific to `warm-clamp-shared`'s own bench;
   it is exactly the shape the "predicated arms" intent (I13) describes in the abstract,
   already instantiated in shipped bench code, one level above the scalar primitive, at
   the fold.

3. **A derivable shape for aggregation**, per section 3's third paragraph: a column, a
   bitpacked buffer, a wide multi-limb layout, all computed from the primitive's identity
   and chosen realisation rather than independently specified by the consumer. `Cap`
   (mentioned in `warm-container-shared/src/lib.rs:619-623` as the thing that carries the
   element-count bound the widening theorem needs) is the one piece of this that is
   already named in a permitted premise as a distinct concept from the primitive itself:
   a capacity is not part of a numeral's identity, but a downstream algorithm's law (the
   widening theorem) needs both the numeral's identity and the column's capacity together,
   at compile time, to decide which arm is dead.

### 6. Is "primitive" one thing, or several under one word

My derived answer, subject to the honesty bound in the coverage section: **at least two**,
and I found one further candidate I cannot resolve from phase-one evidence alone.

**(a) The identity.** The compile-time-nameable contract: declared width (and, per the
inherited-not-verified fraction/signedness composition, presumably an integer/fraction
split and a sign), plus the strategy that determines its semantics (overflow resolution,
and by extension which laws hold). This is what a consumer writes as a type. This is what
a downstream algorithm is generic over. This is what a const predicate (I13) reads to
select an arm. Every one of sections 2 through 5 above is really about this sense, and it
is the sense that answers "what varies within one primitive and what distinguishes two":
within one identity, the realisation is free to vary (that is the whole content of a
bench's arm set); between two identities, any difference in width, split, sign, or
strategy is a difference in *which values are legal and which laws hold*, which is a
genuine distinguishing fact, not a cosmetic one, because I9 ("the strategy is what makes
an answer correct") makes the strategy load-bearing for correctness itself, not merely
for performance.

**(b) The realisation.** The machine-level placement chosen to hold an identity's values:
which native carrier, how many bits of headroom, packed density, limb layout, projection
schedule, lane arrangement. This is an internals decision, freely revisable per target and
per workload, checked only against the identity's own oracle (byte-for-byte agreement),
never itself named by the consumer. This is the sense the strategy markers bias (Hot/Cold
toward fewer bits, Warm/Precise toward headroom) but do not, on the evidence I have,
uniquely determine: `warm-container-shared`'s whole reason for existing is that the
shipped rule (`headroom`, one rung above minimum) is a *candidate*, being priced against
three competitors that also honour the same strategy split, which only makes sense if
"strategy" constrains realisation without dictating a single answer to it.

**A third candidate I could not resolve: is the bit itself a further "primitive" below
the identity, or is it the same identity-axis degenerate to `F = 0`, no sign, no
strategy?** The `Carrier` trait (`warm-container-shared/src/lib.rs:187-209`) is not
parameterised by fraction, sign, or strategy at all; it is a bare-bits container with a
wrapping and a saturating arithmetic vocabulary attached. If the design's eventual
identity type is layered (a raw bitfield primitive underneath a fixed-point numeral
primitive underneath a strategy-decorated public type), that is a third sense of
"primitive," genuinely distinct from (a) and (b), and I found no evidence in the
permitted premises settling whether it exists as a separate named tier or is simply (a)
with some fields defaulted. This is the one place where I genuinely could not find the
answer rather than chose not to look; I record it as unresolved rather than guessing.

**Whether "composition" is the right frame at all: partially, and the split matters.**
It is the right frame for identity (I and F, if the inherited claim is correct, compose
into W and gate law availability; a fold's numeral N and accumulator M compose into an
interior-safety fact). It is the wrong frame, or at best a loose one, for realisation,
which is chosen from a predicate-gated arm set rather than derived by composing
sub-parts. A document that describes the primitive as "composed of" carrier + width +
strategy, with no distinction between the load-bearing composition (identity) and the
selected-not-composed part (realisation), would be stating something true about neither
half precisely.

## What I am carrying forward, and what I would flag if wrong

Kept without qualification: the whole premise set (I3/I4/I8/I9/I11/I13/I14/I15/I16/I17)
as read and quoted above, since these are op's own words on the RATIFIED/STATED/IN FORCE
rungs and are not mine to revise.

The section most likely to move if I am wrong: section 6's third candidate (the bit as a
possible third tier). If phase two shows a prior member already settled this (named the
raw-bitfield tier explicitly, or explicitly rejected it as a separate concept), that is
the first thing I would update, and I would not have manufactured an answer to it here in
its absence, per the concede-honestly discipline.

The finding most exposed to a narrow evidence base: the fraction/signedness composition
claim in section 3. It rests entirely on one workspace rule's citation of a panel result
I am not licensed to open in phase one, not on anything I verified against a committed
artifact myself. The probe I ran establishes only that the committed bench corpus is
silent on it, which is compatible with the claim being true and simply unpriced, or with
it never having been the right frame at all. I flagged this at first mention rather than
letting it read as verified.


---

# Phase two: reconciliation

Appended after the phase-one commit. Nothing above this line is edited.

## What I read

`AGREEMENTS.md` in full (704 lines). The full text of `109_bellard_the_primitive_derived_cold.md`
(1018 lines, both phase one and phase two), `110_willsey_the_primitive_derived_cold.md` (1148 lines,
phase one, phase two, and the reply chain through R8), `111_jhala_the_primitive_attacked.md` (1481
lines, sections 0 through 26, i.e. the whole file including the reply chain through the point where
it records where the argument stood after three files had replied), and
`112_leijen_where_the_refinement_lives.md` (1412 lines, sections 0 through 14, stopping short of its
probe index). `114_leroy_formalising_the_primitive.md` sections 0, 1, 2, 8, 9's opening and the
coverage section (roughly 250 of its 1317 lines; its sections 3 through 7, the detailed mechanics of
the discharge-check arms W0/W1/S1/S2, I read at the level of section 1's summary rather than in the
probe-by-probe detail those sections carry). Op's seven files in full: `113`, `104`, `95`, `88`, `87`,
`85`, `83`. `OPTIONS.md` Q16 in full (the composition-sense entry every topic-five file cites) plus its
header index and its closing "not yet asked" section; I did not read Q1 through Q15 or Q17 in the body,
only their headings. `DROPLIST.md` grepped for `primitive|law set|refinement`, zero topic-five-specific
hits, so I did not read its body; both `OPTIONS.md` and `DROPLIST.md` predate topic five's own register
entries, which have not yet been harvested into either file. `151_leroy_the_candidate_revised_against_four_signatures.md`
sections 0 and 1 only (its opening scope statement and its first repair), roughly 110 of its 559 lines;
the rest is topic eight's internal repair bookkeeping and I did not need it once I had `AGREEMENTS.md`
section 11's account.

**Not read**, named so the gap is visible: `63`, `74`, `90` themselves (I have them only through
`109`'s, `110`'s, `111`'s and `AGREEMENTS.md`'s accounts of them), `106` beyond what `111` and `112`
quote from its sections 1, 11, 16, 17, 18, `108` beyond the clauses `111` and `112` open at source
(`108:822-827`), `115` through `124` themselves (the "realisation map" topic that `114` opens; I have
it only through `AGREEMENTS.md` section 9's checked pointer summary), `125` through `152` themselves
other than `151` sections 0-1 (the rounding-axis and strategy-object topics; I have them through
`AGREEMENTS.md` sections 10 and 11), `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, any `SEED_*` file,
`archive/`, `seed/`, and every panel file not named above (roughly 140 numbered files in the 1-108 and
113-154 ranges I did not open at all).

## The single most important correction: my phase-one working assumption was thinner than the panel's

Phase one asked "what does a primitive compose, out of what" against a self-derived identity/realisation
split. The panel was handed, and is answering, a sharper and more specific question: whether the
**four-part working assumption**, "a primitive is a named composition of a format, a number system, a
law set and a strategy," is correct. I did not know that assumption existed, because it was not in my
permitted premises; `109` section "The question, and the shape of the answer" states it as the panel's
starting point. My own file arrived at a coarser two-part split (identity, realisation) that turns out
to be a genuine sub-case of the richer structure `109` through `112` (and `114` past them) established,
but it is coarser, and phase two's job is to say exactly how.

## Where my phase-one derivation was right, restated in the panel's terms

**My identity/realisation split is `109`'s `(V, ρ, π)` against `λ`, collapsed one level too far.**
`109` section 1 derives four decisions prior to one operation lowering: which values (`V`), how a
value is stored (`ρ`), what happens on overflow (`π`), and which instructions compute it (`λ`). My
"identity" is `(V, ρ, π)` bundled together (declared width plus semantics); my "realisation" is `λ`
plus part of `ρ` (which carrier holds the bits). The panel's finer cut matters and mine missed it:
`109` section 3 shows `π` varies independently of `(V, ρ)` (wrapping and saturating disagree on 32640
of 65536 pairs at fixed value set and fixed realisation), which my file never tested and simply
asserted was part of "identity" without checking it was not itself decomposable. `110` section 2
further shows `π` (overflow) and rounding are **one map with two regions** rather than two axes, which
neither my file nor `109`'s first pass had. My file's collapse of `(V, ρ, π)` into one bucket is not
wrong, exactly, but it hides a real internal structure the panel measured and I did not.

**My "carrier choice does not change the primitive" claim is `110`'s denotational identity criterion,
independently arrived at and much less rigorously.** I asserted this from the bench harness's
byte-for-byte oracle agreement (section 2 of my phase-one file). `110` section 3 derives the same claim
as a formal criterion (denotation-preserving isomorphism), tests it as a **congruence** under four
constructions (`110` P8, 0 failures against 131 and 17 for two weaker candidate relations), and, per
`111` section 8, gives it a companion criterion (`109`'s const-availability test) that answers a
different question (membership, not identity) that my file conflated with it. My version is a special
case of theirs, stated with far less evidence and no congruence check at all.

**My "naming buys a compile-time existence claim" finding is `109` section 6, independently
compatible.** I wrote that a name is what a const predicate (I13) can be written over, and that the
set of names is the set of supported compositions. `109` section 6 states this more precisely: naming
is interesting exactly when it is partial, a name is an existence claim, and under I15 the naming
function is the validator. `109` P5b's compile-fail (`Ranged<0,200> + Ranged<0,100>` refused because
300 exceeds the container) is a sharper, compiled version of what I only argued from the bench arms'
compile-time width matching. I would defer to theirs entirely.

**My "aggregation" observation (a column's shape is entirely derived from the primitive's identity and
realisation, never independently specified) is a weak, under-derived instance of `110`'s composite work
and `112`'s section 8 extension of it.** `110` section 4 formalises this as **composition** proper (a
construction on primitives, distinct from **configuration**, which is what my working assumption's
four-part list actually described), shows a composite is a primitive under the same definition (F10),
and that constructions carry a predicate on their base (F12, `interval` requires monotonicity, 16/16).
`112` section 8 then shows the predicate is **discharge-able by a declared extent** (a wrapping base
restricted to a range where nothing wraps regains monotonicity, 8 of 40 declared extents) and that the
lifting rule for a construction's grade is **not inherited from the base and can be unsound if
borrowed** (componentwise complex multiplication is unsound on 26 of 81 pairs). My phase-one
observation named the shape of this fact from the bench evidence (a bitpacked column and a dense
column realise the identical logical column) but had none of this precision, and I explicitly flagged
it as unverified past the two benches I read; the panel has since verified it much further and found a
real hazard I had no way to find from bench evidence alone.

## Where my phase-one derivation was wrong or under-derived, and where the panel supersedes it

**I never found the refinement, and it is the panel's central result.** This is the single biggest gap
between my file and the converged state. `109` section 7's carried range (`Ranged<LO, HI>`, propagated
through associated consts, eliminating rather than choosing the completion) is the seed of it. `111`
section 9 names it explicitly, connects it to `82`'s declared operand window, and states the working
assumption has the law set and the refinement **exactly backwards**: it lists the law set, which
cannot be varied with the other coordinates held fixed (a lossy projection, not a coordinate), and
omits the refinement, which **can** be varied and separates verdicts with everything else fixed. `112`
then settles what the refinement actually is, definitively: **neither a coordinate of the primitive nor
a member of the declared semantics/strategy pair.** It is a **grade**, read only by arm selection,
transformed (not preserved) by operations, ordered by weakening, with as many parts as the realisation
map has regions. My phase-one section 6 spent real effort on "is primitive one thing or several" and
never once considered that a declared range or bound on a value could be a distinct kind of thing from
either the identity or the realisation. I had the raw material in front of me (`warm-clamp-shared`'s
interior-safety predicate, which I did cite in my own section 5 as an example of "a law set that is
quantified over the identity and available at compile time") and did not see that it was pointing at a
fourth concept rather than an instance of the law-quantified-over-identity idea. `111` section 9.1
identifies exactly this predicate (`82` F6's declared operand window) as the coordinate that passes the
freedom test the law set fails, which is the connection I missed entirely.

**I treated the law set as a legitimate thing to discuss ("a law set that is quantified over the
identity") without ever asking whether it could be varied independently, which is exactly the test that
kills it.** `109` section 4, `110` F3 (with its third bullet later withdrawn as a dead branch, but its
first two bullets and `90` R3 standing independently), and `111` sections 2.1-2.4 all converge, from
three different instruments, that a law set is not a coordinate: it is a lossy, non-injective function
of the algebra (40 algebras collapse to 7 law sets in `110`'s sweep) and, more fundamentally, it is
**analytic** that a function of the tables cannot vary while the tables are held fixed (`111` section
2.3). My phase-one section 5 came close to this insight when discussing the interior-safety predicate
as "a fact that is true or false as a function of the identities involved," but I never generalised it
into "therefore a law cannot be a fourth coordinate beside the ones that determine it," and my section 6
listed "a law set" nowhere as a candidate sense of "primitive" at all, so I neither affirmed nor denied
the working assumption's claim about it. I should have.

**My third open question ("is the bit itself a further primitive below the identity") is resolved by
the panel's machinery, and the resolution is that the question was malformed.** Under `110`'s
definition (a carrier with a total interpretation of a declared **signature**) and `111` section 5's
finding that identity is relative to the operation set and **saturates at the literal** (adding a
literal-carrying nullary operation to any signature reaches the finest identity that realisation map
supports, with every richer operation-only signature adding nothing further), a bare bitfield with only
bitwise operations and no arithmetic is not a layer beneath a fixed-point numeral. It is a **different
primitive**, over a different (smaller or disjoint) signature, of possibly the same value set. There is
no third tier; there is a family of primitives distinguished by which operation set each interprets, and
"the bit container" is one member of that family rather than a substrate underneath the others. I record
this as resolved by the panel's own criterion rather than as something I verified myself; I did not
build a probe to confirm it, and it follows directly from `110` F4 and `111` section 5's own numbers
(the identity relation partitioning identically whether a rich operation set is present or absent, once
a literal is present), which I checked by opening `111` section 5.2 at source rather than only reading
its prose.

**My fraction/signedness composition claim was correctly flagged as inherited-and-unverified in phase
one, and it remains exactly that after phase two.** None of `109`, `110`, `111` or `112` swept `F > 0`
or non-uniform value sets; `110`'s own "not read" section names non-uniform spacing as its single
largest gap, and `112`'s coverage section repeats it. So the `F == 0` necessary-but-not-sufficient
claim I traced to `arvo-always-optimal-internals.md` in phase one is still resting on that workspace
rule's citation of a different panel unit's (`35`'s) probes, not on anything topic five itself
measured. I neither strengthen nor weaken it here.

## Shared input, declared per the brief

Both my phase-one file and 110's phase-one file (and, by its own admission, 111's) had
`arvo-always-optimal-internals.md` auto-loaded as a workspace rule before writing anything. `110`
declares this explicitly in its own "Contamination declared" section and downgrades its own P2 finding
accordingly (distributivity over subtraction failing at unsigned `F = 0` saturating is not
corroboration from `110`, because `110` had read the claim before building the probe that reproduced
its shape). My phase-one file's fraction/signedness paragraph (section 3) rests on the same rule and
the same underlying source, and I made the same declaration there, independently: I flagged it as
"inherited from a permitted premise rather than as something I established" before reading any panel
file, which turns out to match exactly the discount `110` applied to itself for the same reason.
**This is not two independent instances of caution; it is one shared exposure, correctly flagged by
both of us for the same underlying reason**, and I note it rather than claim it as agreement, per the
brief's instruction to discount for shared inputs.

## What I am carrying forward from the panel, and from whom, with a count

From `109`: the four-decision derivation `(V, ρ, π, λ)` with `λ` excluded as derived rather than
identity-bearing (one item); the three sameness relations, nominal/representational/denotational, each
licensing a different operation (one item); the const-availability membership criterion (one item);
the naming-as-partial-existence-claim finding (one item); the chain-accuracy-is-not-a-per-value-property
result, `Mul` is not an endomorphism (one item, noted as untouched by any attacker through `114`'s own
coverage section, which I take on report rather than having verified myself). Five items.

From `110`: primitive-as-finite-algebra-over-a-declared-signature (one item); the realisation map as
one map with two regions rather than two axes (one item, itself a reproduction of `63` C1 per `110`'s
own phase two, so I am carrying it fourth-hand and say so); the composition-versus-configuration
distinction, with a composite being a primitive under the same definition (one item); the direction
count for classifying an axis, later refined by `111`/`112` into the two-versus-one-versus-zero
directions test (I carry the refined version, credited below). Three items, one shared with `109`.

From `111`: the refinement as the missing coordinate the working assumption should have carried instead
of the law set (one item, the single most important correction to my own file); the
denotation/type/adequacy three-layer statement, with soundness and completeness as the obligation a
type owes a denotation it cannot compute (one item); the spurious-versus-refinement-versus-declared-semantics
classification by direction count, two/one/zero (one item). Three items.

From `112`: the definitive settlement that a refinement is neither a primitive coordinate nor a member
of declared semantics, and is instead a grade read by arm selection (one item, superseding and
completing `111`'s own tentative lean the other way, which `111` itself withdrew at `111` section 18);
the finding that a missed merge's cost is a property of where two spellings meet rather than of the
split itself, with the storage boundary the one site with no repair, connecting directly to I17 (one
item); the finding that a construction carries its own grade transformer rather than inheriting the
base's (one item). Three items.

Fourteen items total, one double-counted between `109` and `110` and corrected for above, from four
different authors, none of which I derived myself. **Keeping is a result and I am carrying all of it
because I found no basis to attack any of it,** not because I checked and it survived; I checked only
the specific citations named above and took the rest on the record of the reply chain (section 26 of
`111`, which itself records who conceded what to whom).

## What I would now write as the answer, given the full picture

I defer to `112` section 9's statement, composing with `114` section 8's addition, as the best
available answer, over my own phase-one attempt. Restated in one paragraph rather than requoted in
full: **a primitive is a value set and one realisation map over a declared operation set, with identity
up to denotation-preserving isomorphism; a law is read off it and never declared; a refinement is a
declared restriction on where an operation's arguments lie, transformed rather than preserved by
operations, read only by arm selection, ordered by weakening, with as many parts as the realisation map
has regions; the type carries whatever must be const-available to decide validity or select a lowering,
and owes the denotation both soundness (no two denotations share a name) and completeness (no
denotation has two names); and a construction on primitives carries its own predicate on its base and
its own grade transformer, neither inherited.** My own phase-one answer (the identity/realisation split
plus the naming-buys-a-predicate-target claim) is a correct but strictly coarser fragment of this,
missing the refinement entirely and missing the composition/configuration distinction that would have
told me my own working four-element list ("declared width, semantics, strategy, aggregation") was
mixing a coordinate, a selector, and two different operations under one heading, exactly the class of
error the panel's own working assumption made and got attacked for.

## What remains open even after 109 through 114

Per `111` section 26, `114`'s own coverage, and my own reading: **no transfer argument to a real width
exists anywhere in topic five**, everything enumerative is at `W <= 6`. **Non-uniform value sets are
untested**, which is where arvo's float side lives and which `110` names as its own largest gap and
`112` repeats. **`109` section 8's chain result (a per-value primitive has no slot for chain accuracy,
so `Mul` should not be an endomorphism) is untouched by any attacker through three consecutive
members**, per `111` section 26's own accounting, which I take on report. **The cost of carrying more
than one sound propagation rule was located as an open, unpriced item by `111` section 26** (disjunction
against static selection), and `114` section 1 and section 8 appear to close it with a design
instruction (select the carrier before instantiation, and intersect rather than disjoin), though I have
not read `114`'s sections 3 through 7 closely enough to independently verify the mechanism behind that
instruction, and I flag this as taken on the summary in `114`'s own section 1 rather than checked at
its probes. Nothing in topic five or in what I read of the topic-six ledger prices anything on the
mockspace bench harness; every quantitative claim in both topics is a compiled or interpreted-model
result, not a benched one, and both authors say so of their own work repeatedly.
