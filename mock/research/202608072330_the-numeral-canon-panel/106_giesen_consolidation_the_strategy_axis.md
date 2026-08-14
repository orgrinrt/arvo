# 106. Consolidation: The Strategy Axis

**Position:** the consolidation of the strategy-axis unit, after eight experts and three of op's own
files. **Author:** the `giesen` persona. **Standing:** this is the unit's canon candidate, which means it
is input to the canon rather than canon in miniature. Nothing here settles anything. Op decides, and
`87` fixes when: nothing moves into `mock/canon/` until every topic is done, at which point the canon is
written fresh from all the consolidations as one act, read alongside the members they compress.

**What it compresses.** `93` and `94`, the cold pair, derived blind and in parallel. `97`, which attacked
both. `98`, which second-read `97` and proposed inverting it. `100`, which attacked that. `101`, which
went inside the object. `102`, which asked whether the mechanism serves op's intents and proposed the
pair. `103`, which verified `102`'s central measurement and refuted it. Op's `95`, `104` and `105`. The
checkpoint `99` is the dispatcher's and carries no authority; where it and a member disagree, the member
governs.

**The one thing that decides whether this file is any good**, per the brief and per op at `95`: a unit
ends in agreement or it has not ended. So the accounting is explicit and countable rather than asserted.
This file **refutes or corrects five claims**, every one of them stated with what replaces it and all five
listed together in section 12. Against that it **states a definition in eight clauses** (section 1),
**sixteen findings placed by rung** (section 3), **the mechanisms in sections 5 through 11**, and
**eighteen live options plus five it closes** (section 13). `106_probes/p7_count_the_ratio.sh` counts them
from the file rather than from this paragraph.

There is one thing I want to say before the working, because it is the honest summary of eight files.
**The unit converged, and it converged more than its own located-disagreement sections suggest.** Three
of the four disagreements the members carried forward as open are resolved by material that arrived after
they were written: op's `104` resolves one, `103`'s measurement resolves a second, and a re-reading of two
members against each other collapses a third into a difference about which pair of intents was being
counted. That leaves one genuinely open, and it is not the one the checkpoint flagged.

---

## 0. The two gates

Both run before the assigned work. The brief can neither request nor waive them, and this one did not
try.

### 0.1 Canon gate: passed

Checked against `INTENTS.md` I1 through I18, read in full, and against op's own `95`, `104` and `105`.

Writing this unit's consolidation is licensed by I1, which demotes the strategy set to open in op's own
words, "the strategy set is not closed at exactly four" (`INTENTS.md`, `## I1`), and by `87`, which is op
establishing the consolidation-then-canon order this file sits inside. I17 makes the count explicitly
beside the point of the intent it carries. Nothing in the intents forecloses stating what a strategy is.

**One thing I checked because it would have been a refusal, and it is the same one `98` and `100` both
stopped on.** I16 says the canon "should not police what kind of laws there are or what shapes they take"
(`INTENTS.md`, `## I16`). Sections 4 and 6 below state what a strategy is and what artifacts ship, which
is adjacent to policing. I did not return early, and the reasoning is offered so somebody can disagree:
I16 governs how a **law** is expressed and its test is functional, that it reach one lowered path.
Defining the object the unit was convened to define is not policing, and section 6 states the artifacts as
four arms with disjoint predicates rather than as one required shape, which is the form I13 asks for and
the form op has now demanded four separate times.

**And one I flag rather than resolve.** Op's `104` closes Q50 as not his and returns it to the panel with
a decision procedure: "the experts converge on it, plural and iteratively, and the answer is whichever is
optimal". Section 10 is my attempt at that, and it is a contribution to a live question rather than a
compression of one, which is a thing a consolidation should do sparingly and should mark. It is marked.

### 0.2 Test gate: passed, at 123 tests across 13 crates, and one finding is mine

`mock/crates/` is empty by design, so `cargo test --manifest-path mock/Cargo.toml` errors with "the
manifest is virtual, and the workspace has no members". That is the intended state and not a defect. The
only executable surface this unit touches is the bench variant crates, so I ran those, per crate, rather
than taking four unratified files' word for it.

| crate | tests | crate | tests |
|---|---:|---|---:|
| `bitpack-carrier-shared` | 9 | `quantiser-fadd-shared` | 1 |
| `bitpack-contend-shared` | 12 | `quantiser-radix-shared` | 3 |
| `bitpack-footprint-shared` | 6 | `satfold-shared` | 11 |
| `bitpack-plan-shared` | 5 | `warm-clamp-shared` | 7 |
| `bitpack-shared` | 3 | `warm-container-shared` | 15 |
| `bitpack-wide-shared` | 6 | `wide-rung-shared` | 30 |
| `bitpack-write-contend-shared` | 15 | | |

**123, all green.** That is the fifth independent count, after `98`, `100`, `102` and `103`, and it
confirms `98`'s account including its explanation of why a grep returns 124. `96`'s note holds:
`bitpack-write-contend-shared` needs `--test-threads=1`, and `100` diagnosed why, three stress tests
sharing one four-thread pool.

**The finding is against my own first run and I am recording it because it is the exact shape the gate
exists to catch.** My first invocation piped each crate's output through `tail -4` to get the result line.
`cargo test` emits two result lines, the unit tests and then the doc tests, and the doc-test line is last
and reads `0 passed; 0 failed`. So the run reported **zero tests in all thirteen crates and exit code 0**,
and it looked like a completed gate. Nothing was wrong with the suite; the instrument was mine and it
produced a green line that meant nothing. The corrected run greps every result line, and its output is the
table above.

That is worth one paragraph rather than a footnote because it is the third distinct way this corpus
produces a meaningless green: `cargo test --workspace` from `mock/benches` reaches only the driver and
reports zero (`102`, `103`); `cargo test` without `--test-threads=1` hangs on one crate (`96`, `100`); and
the obvious result-line extraction reads the doc-test block. All three exit zero.

**And running the gate contaminates the next source grep, which is a fourth.** The suite build creates
`variants/*/target/`, and `outputs_may_differ` appears as a literal inside `target/debug/**/*.rmeta`, so
`grep -rl outputs_may_differ variants/` returns **133 files after the suite has run and 1 before**. Both
numbers reproduce and only one is about the source. My own first version of `106_probes/p2` recorded 133
and I caught it against a count I had taken earlier on a clean tree. Every count in section 7 is taken
with `--exclude-dir=target`, and the probe carries the contaminated number beside the clean one so the
difference is shown rather than asserted. This is the same class `102` caught in its own probe, where a
grep matched its file's own sentence claiming the thing was absent.

### 0.3 What the bodies say, and a correction that touches four member files

I read the bodies rather than the names in the surface this file rests on, which is the answer-discipline
machinery: `bitpack-shared`'s tests and validator in full, the three control-arm module headers, and
`101`'s and `103`'s classification instruments.

**`bitpack-shared` is characterised wrongly in four of the eight member files, and the error propagated.**

The crate has three `#[test]` functions, each calling `check_size::<N>()` at 256, 4096 and 16384. What
that body does (`mock/benches/variants/bitpack-shared/src/lib.rs`, `fn check_size`) is:

```rust
let a = extract_aligned(&col.aligned, i);
let z = extract_zeropad(&col.zeropad, i);
let expect = col.logical[i];
assert_eq!(a, expect, "aligned mismatch at seed {seed} index {i} N={N}");
assert_eq!(z, expect, "zeropad mismatch at seed {seed} index {i} N={N}");
```

over every index, every size, eight seeds each, plus a bijection check on the permutation. **Both arms are
asserted against the logical ground truth**, which is an oracle-backed cross-arm agreement assertion, at
full coverage rather than at a sample. And the module's own doc comment says so in its first line:
"Cross-checks both extraction paths against the logical ground truth, every index, every size, 8 seeds
each." The body is extracted whole at `106_probes/p5_what_bitpack_shared_actually_asserts.out` rather than
described.

Against that:

- `94` section 0 reports the three tests as "`check_size::<N>()` roundtrips" whose body "does assert a real
  property (the permutation is a bijection)". It names the bijection and does not name the ground-truth
  cross-check, which is the stronger of the two.
- `97` section 0 corrects `94` from "a sample" to "a redundancy", on the ground that every byte phase is
  exercised within the first eight indices at `N = 256`. That is right about the extraction property and
  says nothing about the bijection check, which is generated per size.
- `102` section 0.2 states that `bitpack-shared` has **"no cross-arm agreement assertion of either kind,
  mutual or oracle-backed"**, and builds from it that the crate's answer-equivalence "rests entirely on the
  harness's cross-variant byte comparison", which `96` reports was silently disabled.
- `103` section 0.2 corrects `102` in `102`'s favour and inherits the wrong half: "That is right about the
  tests and it understates `bitpack-shared`'s position", then classifies the crate ANSWER-PINNING on the
  validator alone. **It is not right about the tests.** The assertion `102` looked for is present.

The verdict `103` reached is correct and its reason is incomplete. Nothing downstream breaks, because the
crate is answer-pinned twice over rather than once. What the sequence shows is the failure mode the gate
names: three members read the test names or a predecessor's summary, the fourth opened the file, found a
stronger fact than the predecessor claimed, and still carried the predecessor's framing of the weaker one.

**The control-arm headers, confirmed at source.** `101`'s test-gate finding holds exactly.
`bitpack-carrier-d16-control/src/lib.rs` asserts byte-identity **and names the check that establishes it**.
`bitpack-contend-d16-control/src/lib.rs` and `bitpack-wide-d16-control/src/lib.rs` assert it and name
nothing. `101`'s own p0 then measured that the wide pair **is not identical**, differing at three
constant-pool vector loads, and its output is committed. I did not rebuild the dylibs; I confirmed the
three headers say what `101` says they say, which is the half a reader of this file needs.

**Nothing in the surface I examined is tautological, sampled where a matrix was available, or
assertion-free.** `103`'s two observations on `quantiser-radix-shared`, one redundant assertion and one
name that promises the conclusion where the body delivers the premise, are both real and both minor, and I
found nothing to add to them. There is nothing here to refuse on.

---

## 1. What a strategy is

Stated once, plainly, before any of it is argued for, in the register a canon could take. It is a
suggestion and op decides.

> A **strategy** is a pair.
>
> Its first component is an **assignment on the axes a consumer can observe**: those where moving the
> assignment changes what the program computes, or whether it computes at all. It is supplied and never
> derived, because a consumer of a value cannot recover it from the bits, so every consumer of that value
> must agree about it.
>
> Its second component is a **weighting over cost coordinates**, which selects among the arms that produce
> the answer the first component fixed. It is resolved and never observed, because nothing a consumer can
> see depends on which of those arms was taken.
>
> The two components have **different carriers**. The first travels with the value. The second is supplied
> where the operation happens, because only the site knows the arity, the access pattern and the target. A
> **named** strategy binds one point in each, so that a consumer states one intent rather than answering
> two questions.
>
> A cost coordinate is **measured** or **computed**. A measured one has a resolution the instrument
> reports; a computed one is exact. A weighting may read a measured coordinate only where the arms it
> ranges over compute the same value, and only where the cost ordering and the answer ordering do not
> conflict. Otherwise every coordinate it reads is computed, because otherwise the program's output is a
> function of a benchmark's noise.
>
> A quantity over which a strategy's answer may **differ** belongs to the **region**. A quantity on which
> a strategy's answer is **scored** belongs to the **cost vector**. Width, element count, arity, thread
> count and chain depth are the first kind. Time, footprint and error against a declared reference are the
> second.
>
> Two strategies are related by an order on their **first components** where one exists, and by **nothing**
> on their second, because two weightings are incomparable vectors and nothing ever asks them to combine.
> Where two first components disagree, the operation **reports a conflict that is real** rather than
> silently resolving toward either.
>
> The **number of strategies is not a design parameter.** It is bounded above by the coordinate set, which
> is countable exactly, and a name is a binding rather than a member of a closed set.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a container, a
width, a marker, a type parameter, a table cell, a crate, or a count of strategies.

**Equivalence.** Three teams implementing from this produce units that behave the same on what matters: a
consumer supplies the answer-fixing part and cannot supply the rest, the compiler resolves the rest and
tells nobody, the resolution is derived from a stated weighting over a committed table and checked at
build time, and nothing that changes an answer is decided by a timing. They differ on how many strategies
ship, what they are called, and how the two carriers are spelled, which are the arm rather than the
concept.

**Whose it is.** The pair is `102`'s, at ONE EXPERT, and section 4 says exactly which parts of it carry
more than that and which carry less. The two-carrier clause is `94`'s W9 and `97` section 5, at two
measured instances. The measured-or-computed clause is `102`'s with `103`'s predicate attached. The
region-against-cost-vector clause is `101`'s, close to verbatim, and `102` built and compiled both sides of
it. The reporting-rather-than-resolving clause is `97` section 4.4 standing on `93`'s counting result.

---

## 2. How the unit got there, which matters because the route is the evidence

Eight files, and the object changed shape four times. The sequence is worth stating because the rung each
claim sits on is a fact about who derived what before reading whom, and a reader who takes the final shape
without the route cannot tell a three-instance result from a restatement.

**`93` and `94` derived cold, blind and in parallel**, reading only the intents, the rules and the
repository. `93` reached "a strategy is a preference: an ordering over candidate implementations of the
same abstract operation, computable at compile time" and split it into a **policy layer** that changes the
answer and a **lowering layer** that does not, arguing the two are ordered by stratification. `94` reached
"a compile-time choice function over a shared set of arms, whose value at each region was decided offline
by measurement under a stated weighting" and split it into **cost, policy and licence**.

Both then reconciled and both corrected themselves. `94` withdrew licence as a co-equal axis, on the
ground that which laws hold is **computed** from the policy and the region rather than chosen, and its own
probe C was the evidence it had misread. `94` also withdrew "cost belongs at the site" for the storage
half, since a value's layout has to be on the value. `93` withdrew its notko evidence on op's direct
ruling that the two designs do not correspond, and withdrew the claim that the preference sits on the
value, conceding `94`'s W9 had the better evidence and noting that its own P4 had put the preference at
the call site without its author noticing.

**Nine claims matched across the two blind derivations**, which is the TWO EXPERTS rung by the panel's own
definition rather than by file count. The strongest of them: the preference is not a bundle of
implementation choices; the container, the codegen choice and the overflow rule are effects of one cause
rather than three components of a marker; the selection erases; chain accuracy is structurally different
and needs a widening construction; the set of named strategies is a set of points in a product.

**`97` attacked both and changed the shape.** Its three results that survived everything after: the
merge between `25` section 7 and the cold pair's definition does not hold, and the gap is measurable; the
distinction that does the work is **polarity** rather than stratification, because `93`'s circularity does
not exist and the structure is a dependent function; and "what relates two strategies" is one question
asked of three layers with three different answers.

**`98` second-read `97` and reproduced its arithmetic exactly**, from an independent implementation on the
same committed data, which is the strongest corroboration this panel can produce short of op. It corrected
which rung the criterion is stated at, and proposed inverting it.

**`100` attacked the inversion and found its own boundary.** Generation does not remove the check, it
relocates it; the motivating instability is largely an instrument artifact on the family it was measured
on, and holds on two of four families; and the two proposals compose into a compile-time assertion neither
author saw.

**`101` went inside the object** and found that what a weighting ranges over is one coordinate, that a
coordinate set is a countable ceiling on how many strategies can exist, and that normalisation is a change
of basis rather than a design fork.

**`102` asked whether any of it serves op's intents**, ran `97`'s own polarity test on `25`'s own axis
list, which nobody had done, and proposed the pair.

**`103` verified `102`'s central measurement, refuted it, and the conclusion survived one rung lower.**

Four of the eight corrected a claim of their own against their own evidence and kept the refuted output
committed. That is the discipline working, and it is why the numbers below are worth carrying.

---

## 3. What the unit settled, by rung

Rungs per `RULES.md`: RATIFIED is op after convergence and nothing here holds it; TWO EXPERTS requires two
independent derivations, each before reading the other; ONE EXPERT is a queue entry asking for the second
read it has not had. Predicates per I13: a dimension listed with a range or `any` was established across
it, listed with a fixed value was established there only, and **absent means the finding does not hold
anywhere that dimension is present**.

### 3.1 Three or more independent instances

**The rationalisability counts.** On the committed `bitpack-carrier-width` table, of 15625 sections, **72
are rationalisable by a non-negative weighting and 9 by a strictly positive one**, and **63 of the 72
select an arm no weighting can select**.
`holds for: regions = 6, arms = 5, cost coordinates = 2 (median algo_ns per record, declared bits per
element as 16/32/64/13/13), cost source = the committed bitpack-carrier-width_n* CSVs, arithmetic exact
rational, threads = 1, target features any`
Three independent implementations from three different geometries: `97`'s extreme-ray enumeration of a
pointed cone (`97_probes/p9_the_decider.py`), `98`'s interval arithmetic on the one-dimensional weight
simplex (`98_probes/p6_reproduce_the_predecessors_count_and_rung_it.py`), and `101`'s polygon clipping
written without opening either (`101_probes/p4_what_a_coordinate_buys.py`). `RULES.md` puts the bar at
three and these two numbers clear it.

**And the whole 72-against-9 gap is one tie**, which `101` established and which nobody had named:
`bitpack-carrier-packed` and `bitpack-carrier-packed-simd` both declare 13 bits, so a pure-size weighting
ties them at all six regions, making `2^6 = 64` sections weakly rationalisable, of which 63 name the
dominated arm. `101_probes/p10_the_two_knobs_are_separable.py`. That reduces a constraint about the sign of
every weight to a rule about ties, which is section 6.3.

**The suite is 123 tests across 13 crates.** Five independent counts: `98`, `100`, `102`, `103` and this
file. The earlier 108 and 96 are superseded and `98` explains both.

**The selection erases.** A strategy resolved at compile time leaves no residue in the emitted body, on
four instruments: `93`'s P4 comparing a const-fn argmin over a cost table against the hand-written arm, nine
instructions each and identical after label normalisation; `94`'s probes A, B and F reading entry bodies
and finding a single tail branch with zero conditionals; `100`'s p3 compiling both encodings and finding
the assembler emitted `_e2_weighted = _e1_named`, a symbol alias; and the committed harness output
`mock/benches/satfold-const-gate_n10000_findings.md`, where the arm reached through a const verdict is
1438 ns median against 1456 ns reached directly, overlapping intervals, with the false-verdict gate at
38391 ns.
`holds for: target aarch64-apple-darwin, rustc 1.98.0-nightly (57d06900f), edition 2024, opt-level 3, panic
= abort, no_std, feature gates = 0, arms in {2, 3, 4, 5}, threads = 1 for the timed instance and threads
any for the compile-time artifacts`
This is the single best-supported claim in the unit and it is the one the whole mechanism rests on.

**Multiplicative associativity and distributivity hold at `F = 0` and fail at `F > 0`, for unsigned
types.** `holds for: W in 3..8, F in 0..2, signedness = unsigned, overflow in {wrap, saturate},
operations {add, sub, mul}, arity 2 and 3, values exhaustive over the representable domain, threads = 1,
target features any`

**`F = 0` is necessary and it is not sufficient, and the unqualified form of this sentence is false.**
Restored on `107`'s check, from `93`'s F1 directly, which carries `signedness = unsigned` in its own
predicate. At **signed** saturating, `F = 0`, two independently written models measure the law failing:
`93_probes/p7` gives distributivity failing **47.72%** of triples at `W = 7`, and `97_probes/p2` gives
**34.52%** at `W = 6`. A one-sided clamp is a congruence and a two-sided one is not. `97` section 6.3
adds a second qualifier inside the surviving region: at unsigned `F = 0` saturating, distributivity over
**addition** holds while distributivity over **subtraction** fails at 45.79% of triples, so a law
permission names the operations it covers and not only the fraction width.

**This is the third time this sentence has lost its qualifier**, and the compression lost it again after
the loss had already been diagnosed twice in the same unit. It appears unqualified at `35:311`, where it
originated, and at `94:887`. Those are member files and are the historical record, so they stay as
written; whoever writes the canon must not take the sentence from either of them. It also appeared
unqualified in the workspace rule `arvo-always-optimal-internals.md`, which `97` found and which was
corrected during this unit precisely because it was a live licence to emit a wrong rewrite. `97`'s
criterion is cited below as support, and `97` section 6.3 built that criterion **to find this hazard**,
which makes citing it without the qualifier the sharpest form of the error.

The supporting measurements, which are correct and were never in question: `93`'s F1 measured
it exhaustively over every triple at `W` in 3 to 8; `94`'s probe C part 2 measured that rounding retraction
retracts exactly at `F = 0` over `W` in {4, 6, 8} and `F` in 0 to `W`, both rounding modes; `97`'s
criterion predicts every verdict in 552 cells with zero mismatches in either direction; and `97`'s p7
tested the same criterion against `35`'s 660 committed law rows, generated independently months earlier,
with 659 agreements, one conservative row and zero soundness mismatches.

### 3.2 Two experts, each deriving before reading the other

**A strategy is a preference over measurements resolved as an argmin over candidate arms at compile time**,
and the container, the codegen choice and the overflow rule are effects of it rather than components of a
marker. `93` section 2 from the partial order on cost vectors; `94` section 3.1 from I8 and I9 read
together. Blind, in parallel.

**Chain accuracy cannot be served by an operator closed over its operand type**, and the intermediate width
grows linearly in chain length. `93`'s F7 at `W = 8, F = 4`; `94`'s W7 at `W = 16, F = 8` with the growth
rule in closed form and the availability table against a 128-bit widest rung. Two parameter settings, two
authors, blind.

**The named strategies are points in a product and the flat set is a slice through it.** `93` from the
resolution side by counting; `94` from the component side by measuring that a fifth unnamed point costs one
alias symbol and eight bytes. Two disjoint arguments.

**175 of 254 committed regions were produced before the driver called cross-variant validation at all.**
`103`'s F-103-6, and this file. `103` named it as the finding it most wanted re-derived, so I redid the
join independently: resolve each `*.meta.json`'s `git_commit` against the commit time of `9db33f8c`, the
commit that added `harness::validate` to `mock/benches/src/main.rs`, verified present there and absent at
its parent. **175 before, 79 after, 0 unresolvable, over 254 meta files.** Exact reproduction.
`holds for: the committed corpus at HEAD of feat/arvo-shape-topic, files 254, producing commits 24, all
resolvable, threads = 1`
Evidence: `106_probes/p3_prewiring_join.py` and its output. One reconciliation with `103`, which is not a
correction: there are **24 distinct `git_commit` strings and 23 distinct commits**, because `defc747`
appears both clean and dirty. `103`'s 24 is the string count and is right as stated.

**And the reproduction has a precondition neither `103` nor I could have skipped, which sits 81 files back
and is uncited in this unit.** Every one of the 254 `git_commit` values but one carries a `-dirty` suffix,
so a naive resolution returns 253 unresolvable and the finding evaporates. The suffix is meaningless:
`22` established, at `22:188-193`, that the harness writes its artifacts into the tree it then hashes, so
every size row after the first is dirty by construction and `git diff --name-only HEAD` returns zero
tracked files. Stripping it is correct and it is correct **for a reason nobody in this unit carried**. A
later reader redoing the join without that fact will conclude the corpus has no resolvable provenance at
all.

### 3.3 One expert, and each is a queue entry rather than a doubt

**The pair itself** (`102`). Section 4 separates what inside it carries more.

**Polarity as a derivation** (`97`). This is a rung correction and section 12 states it: the register and
the checkpoint read as though the observable-versus-unobservable split is settled, and `102`, the file
usually cited as the second instance, says of itself that it "did **not** derive it independently: I read
`97` section 3.2 before building anything". What `102` did was point `97`'s test at a list `97` never
chose, which is a stress test rather than a second derivation, and `102` says so in its own section 2.5.

**Strict positivity as the requirement for the no-dominated-arm guarantee** (`98`), with `101`'s fourth
option beside it.

**The generate-against-check fork.** Flagged at the checkpoint as the first thing the second four should
attack. `100` attacked the surrounding machinery and `102` attacked the coordinate split. **Nobody
attacked the fork itself**, and `103` says so. This is the unit's largest unclosed item and it is stated
as one in section 13.

**Twenty of 254 committed regions are not answer-pinned** (`103`), eight with arms measured to differ or
two different algorithms by construction.

**The coordinate ceiling** at 1, 9 and 42 as coordinates are added (`101`).

**The measured-versus-computed split** (`102`), with `103`'s predicate: the hazard needs the cost ordering
and the answer ordering to conflict, not merely the arms to differ, and the corpus contains zero instances
because `radix2` is 1.18x to 1.64x faster **and** 66x more accurate than `radix10` at every committed size.

---

## 4. The object, and what is actually new in it

The pair is the unit's strongest candidate and it arrived in the seventh file, so nobody attacked it. That
is `103`'s reading and it is right. But "one expert" understates what is behind it, and separating the
layers is the most useful thing this section can do.

**The structure is not new and is well supported.** `97`'s three layers, and `102`'s pair, are the same
object counted differently. `97` enumerates objectives, observable mechanism coordinates and unobservable
mechanism coordinates. `102` enumerates the policy assignment and the weighting, and moves the third out of
the enumeration because it is what the second **produces** rather than a thing anyone supplies. `102` says
this itself: `97`'s three layers "are the pair with its second component's output named". They are not two
candidates and the consolidation should not present them as such.

**What is genuinely new in `102`, and is at ONE EXPERT, is three separable claims.** They should be
attacked separately, because they can fail independently.

**(a) The two components are what op's `88` answer decomposes into.** Op was asked whether a strategy is a
preset naming a point in a space of axes, an irreducible identity, or nothing but a weighting, and answered
"Mostly option 1, but a little bit of option 3 with it. Hard to put into words, hopefully you get my
meaning here", flagging his own difficulty. `97` read the mix as tiers: the design tier writes points, the
canon tier writes the objective, and the surviving bit of option 3 is the rationalisability constraint.
`102` reads it as components: option 1 is the policy assignment, option 3 is the weighting, and "mostly
with a bit of" is the honest proportion. **Both readings are coherent and they are not the same claim.**
What would distinguish them: whether the rationalisability constraint has any content once the pair is in
place. Under `102`, a table over answer-equivalent arms is exactly what a weighting produces and the
constraint is a check on it; under `97`, the constraint is the content of op's mix. Nobody has tested this
against the other.

**(b) I3 and I5 are not weighting-shaped, so a coordinate ceiling does not bound them.** Section 9 is
where op's `104` bears on this, and it lands on `102`'s side for I3.

**(c) `25` section 7 and the cold pair's definition differ by polarity rather than by count.**
`102_probes/p2_which_of_25s_axes_change_the_answer.rs` ran `97`'s observability test on `25`'s four axes,
one at a time, the others held, at a declared width of 13:

| axis | ring chains | past a non-ring step | verdict |
|---|---:|---:|---|
| headroom | 0/640 | 500/640 | observable only past a non-ring step |
| packing | 0/640 | 0/640 | unobservable everywhere swept |
| overflow policy | 511/640 | 511/640 | observable in both regimes |
| intermediate precision | 0/640 | 570/640 | observable only past a non-ring step |

`holds for: W = 13, operations in {+, -, *, >>}, column lengths in {1, 2, 3, 4, 8, 16, 32, 64, 128, 1024},
seeds 1 to 64, unsigned, threads = 1, host aarch64-apple-darwin, rustc nightly-2026-05-28`

**Three of `25`'s four axes change the value the program computes and one does not.** So `25` describes
the input layer and the cold pair describe the output layer, they have opposite polarity, and no merge was
available at any rationalisability count. `102` concedes its own prior file `25` on that ground rather than
on `97`'s counting ground, which is a stronger concession than the one it was asked for.

**And the probe's first version is worth more than its result.** It swept only additive chains ending in a
mask and reported headroom and intermediate precision as unobservable at 0 of 640. The zeros were not
noise: reduction mod `2^W` is a ring homomorphism, so any composition of `+`, `-` and `*` gives the same
low `W` bits whatever width it was computed at, and the sweep was proving the law it stood on rather than
testing the axis. Both versions are committed.

**The ring boundary that fell out of the fix is a licence nobody took.**

> Headroom and intermediate precision are **invisible across any composition of `+`, `-` and `*`, and
> become visible at the first step that is not a ring operation**: a shift, a division, a saturation, a
> comparison.

Inside a pure ring region those two axes are unobservable, which means they are the resolver's to choose
and a strategy need not fix them. That is a large region, it is const-visible wherever the chain is
written, and it is exactly the arm-with-a-predicate shape I13 asks for. It is unpriced and unbuilt and it
is carried as a live option in section 13.

---

## 5. Cross-strategy resolution: three answers on three regions

This is the question `93` and `94` both opened and both conceded, and it is the one place where the unit's
answer is cleaner than any single file's.

`93` enumerated four responses and could not choose: carry the closure, refuse and require the consumer to
name the result, the demands are on different roles, or keep the flat join. `94` reached three options
about what the marker is a claim about and leaned at R3 without calling it settled. `97` then showed the
four responses are not four competing designs: **they are the correct answers to three different questions
plus one wrong one**, and the side each is seen from is the layer. That is why `93` could not choose and
why it reported leaning toward two of them being the same answer seen from two sides. They are.

**On the weighting, the join is union and it is free.** `93`'s P1b reported that four markers carrying one
demand each leave 12 of 16 ordered pairs unresolvable and that the closure has 15 elements, and `93`'s prose
priced the closure as a cost. **Fifteen is `2^4 - 1`**: the closure of `d` one-demand generators under
union is the free join semilattice on `d` generators, whose carrier is the non-empty subsets. It is
generated rather than enumerated, so a design writes down `d` names and gets the rest.
`97_probes/p4b_the_closure_is_free.py` reproduces both of `93`'s numbers from an independent enumeration and
identifies the object; `97_probes/p4_demand_lattice.rs` compiles it on the pin, `no_std`, zero feature
gates, with all 256 ordered pairs asserting the join is the union and commutes, all 4096 triples asserting
associativity, and all 16 elements asserting idempotence, with a mutant that drops one coordinate failing
to compile at `E0080`.

**`93`'s own probe output already said this and its prose priced it as a cost anyway.** That is a
compression loss inside one file, between its probe and its own text, and the option register inherited the
prose. Worth recording because it is the same failure this consolidation exists to avoid, occurring at the
smallest possible scale.

Two things the demand lattice buys that a flat set cannot. **Silence becomes a first-class element**: the
absence of a demand is the statement that the consumer asked nothing there and the resolver is free, rather
than a cell somebody has to fill. And **the escalation pathology disappears**: `93`'s F4 found exactly four
join semilattices on the four named markers satisfying six intent-derived constraints, and **all four make
the accuracy-first preset the top and escalate every mixed expression to it**, which `93` priced as "nobody
asked and everybody pays". Under the union join, a speed demand joined with a storage demand is the element
demanding both, and `97`'s P4 asserts at compile time that it is neither operand's and has lost neither.

**On the observable axes, no join exists, so the operation reports.**
`97_probes/p3_does_a_conservatism_order_exist.py` computed three candidate orders exhaustively over the
whole representable domain and they disagree. By honoured laws, wrapping and saturating are **incomparable
in three of four configurations**. By how often the answer is wrong they are **exactly tied**, both being
wrong on precisely the pairs whose exact result is not representable. By worst-case magnitude, saturating is
above wrapping. So "resolve toward the more conservative side" has no referent until somebody says which of
the three is meant, and two of the three do not give an order at all.
`holds for: W in {5, 6}, F in {0, 1}, signedness in {unsigned, signed}, overflow in {wrap, saturate,
exact}, operations {add, sub, mul}, laws as enumerated in the probe, arity 3 for the algebraic family and 2
for the order family, values exhaustive over the representable domain, threads = 1, target features any`

The consequence for the resolution mechanism the superseded design tier named, a total join over the
markers resolving toward the more conservative side, is direct, and it is the unit's clearest
refutation of a shipped shape: **a total silent join over the observable axes is the wrong mechanism**, and
what replaces it is a report that two operands demand different computed answers and the site must say
which. That is the diagnostic `arvo-toolbox-not-policer.md` asks for rather than the refusal it forbids,
and the predicate separating the two arms is not "where the demands happen to have a join" but the far
simpler `observable(coordinate)`, which is static.

**And a fourth result in that probe kills a reading of Q41 outright.** Saturating and
exact-in-a-wider-rung are **incomparable at `F = 0`**, both signednesses, both widths, because the absorbing
top is a law only a lossy policy has. So the accuracy-first policy is not the top of the law order, and no
choice of inventory rescues it, because exactness destroys a law saturation provides.

**On the unobservable axes there is no mixing question at all**, and a design that has one has put an
unobservable coordinate in an input position.

---

## 6. What ships, as four arms with disjoint predicates

`100` section 6.1 is the composition and it is the form I13 asks for. Written as one recommendation it
reads as a single answer, which is the shape op has rejected four times; it is four arms, each applying on
its own region and nowhere else, and each predicate is a const-checkable property of the corpus rather than
a judgement.

**The artifacts.** Ship the weighting, the cost table, and the winner table generated from them, with a
const assertion that the third is the argmin of the first over the second at every region.

**Arm A, generate and assert equality.** `holds where every coordinate the strategy weighs has a value the
corpus can express, and the arms competing at each region are separated by more than the instrument's
resolution there, and the arms compute the same value.` The last clause is `102`'s and section 6.2 is why.

**Arm B, check without generating.** `holds where a coordinate the strategy weighs has no expressible
value, so no weight vector can be written at all.` This is the interim rather than the target, and `98`
says so about its own proposal: section 8 below establishes that two of op's four intents sit in exactly
this region today.

**Arm C, the band rather than the equality.** `holds where the region's competing arms are separated by
less than the coordinate's resolution, more than zero, and compute the same value.`

**Arm D, no differential.** `holds where the coordinate's resolution does not separate the arms it must
distinguish`, which `100` section 7.3 then showed is usually a property of the estimator rather than of the
coordinate, and therefore usually calls for changing the statistic before accepting the arm.

### 6.1 Why the check does not disappear under generation

`98` argued for inverting `97`: state the weighting, derive the table, and rationalisability becomes true by
construction so "there is nothing to check and nothing to police". **The first clause is right and the
second does not follow**, and `100` measured rather than argued it.

Generating removes every defect a human writing a table can introduce and admits one a human cannot: the
tool computes the wrong argmin. And a tool's wrong argmin is, in general, **the right argmin of something
else**. A coordinate read in the wrong unit is the exact argmin of the rescaled weighting; a column swap is
the exact argmin of the permuted one. So the output stays rationalisable, and the criterion is looking for
exactly the property those defects preserve.

`100_probes/p2_generation_relocates_the_check.py` injects five ordinary generator defects and runs three
detectors. **Rationalisability catches 0 of 190 unit errors, 0 of 147 column swaps and 0 of 152 dropped
coordinates**, at both the non-negative and the strictly positive rung. **Cone membership of the stated
weighting catches all 489**, and is invariant to tie-break policy where independent recomputation is not,
recomputation flagging a tie as a defect 48 of 48 times when a tie is precisely where the weighting
declines to choose.
`holds for: regions = 5, arms = 5, cost coordinates = 3, 400 random models across two families, plus the
committed carrier table at 6 regions and 5 arms with the noise-floor control dropped, arithmetic exact
rational, defect classes {unit scale, coordinate swap, tie-break policy, region off-by-one, dropped
coordinate}, threads = 1, target features any`

**So `98`'s proposal survives with its justification amended, which is a stronger position than the one it
argued.** The reason to generate is not that nothing is left to check; it is that generation removes a
defect class outright and leaves one detectable by a decider the panel has already built three times, in
exact arithmetic, at no compile-time cost.

**And the two proposals compose into something neither author saw.** The check is an assertion over two
artifacts both already present: the committed winner table equals the argmin of the stated weighting over
the committed cost table, at every region. `100`'s p3 compiles exactly that as a `const` item. It costs one
const evaluation, nothing at runtime, and **refuses at build time** rather than reporting, which is the
shape I15 asks for. Its mutant fails with `error[E0080]: evaluation panicked: the committed winner table
disagrees with the argmin of the weighting that is supposed to have generated it`, and the assertion has
been seen to fail twice, once as a mutant and once accidentally when p3's first hand-written winner table
was wrong.

### 6.2 The fork that has consumer-visible content is not the one that was asked

`100`'s sharpest structural finding. Check-against-generate are two maintainer workflows that **emit the
same artifact**, so the fork has nothing a consumer can observe. The axis that does is one `93` named in
its own phase-two withdrawal and nobody picked up: whether the compiler is handed a **winner table** or a
**cost table**.

`100_probes/p3_three_encodings.rs` compiles four entry points, `no_std`, zero feature gates, no `dyn`, no
`TypeId`, no `generic_const_exprs`, with the selection forced through an inline `const { }` block so the
claim is about const solving rather than backend folding. The assembler emitted:

```
	.globl	_e2_weighted
_e2_weighted = _e1_named
```

Not the same instruction count, not equivalent: **the same symbol**. And `e4_consumer`, a weighting nobody
tabulated supplied the way a consumer supplies one, reaches a different arm from the named strategy at that
region in one tail branch, with a compile-time assertion pinning that it resolves differently so the
comparison cannot go vacuous.

`100_probes/p9_does_the_cost_table_survive_into_the_binary.sh` adds that the cost table occupies **zero
bytes**: no constant-data section and no symbol, with a control adding one runtime read of the same table
emitting exactly 240 bytes of `__const`, which is `6 * 5 * 2 * 4`, the table to the byte. **The control is
the finding**; without it an empty section proves nothing.

**So on the two axes a spike can reach, emitted instructions and emitted bytes, the cost-table encoding
costs nothing over the winner table and buys a consumer a weighting nobody named.** That is
`arvo-toolbox-not-policer.md`'s posture and I11's "the value is what composes on top of it", reached by
measurement rather than by preference. **The one axis remaining is compile time and it is unpriced**, in
that word, because the harness's CSV schema is entirely runtime measurements of an already-loaded cdylib
and there is no compile-time arm to run. `100` checked that rather than assuming it. What is established is
that no const-evaluation limit is reached at 1024 regions by 64 arms by 4 coordinates.

**And a citation correction inside that finding, which `100` made and which matters for anyone building
here.** `98` cites `93`'s P4 as evidence that both sides of the encoding fork compile. P4 declares
`const ARM_COST: [[u32; AXES]; ARMS]`, a cost per arm with **no region dimension**, so its const argmin
runs once and is a constant fold with nothing region-shaped in it. The encoding the fork is about indexes
cost **by region**, so the argmin runs per monomorphisation. That is a different compilation question and
P4 does not answer it. `100`'s p3 is the fork compiled at the dimension P4 did not reach.

### 6.3 The guarantee, and the cheaper knob

`98`'s correction to `97` holds and is a theorem rather than a measurement: **a strictly positive weighting
cannot select a Pareto-dominated arm**, because if `b` beats `a` on every coordinate and strictly on one
then `<w, b> < <w, a>` for every `w > 0`. A non-negative one can, and whether it does is a property of the
table, so the guarantee at a zero weight is **unclaimed** rather than usually-holding.

**And `100` measured what the strict rung is a detector for, which is exactly one event and no other.**
Across four families of 300 models each, the rung fires precisely when a section selects an arm dominated
on the full coordinate set and never otherwise, and it is reachable only where the arm set contains a pair
**tied on every coordinate the effective weighting reads and differing on one it does not**: 0 of 230 on
independently drawn arms, 0 of 241 on arms sharing one coordinate, 0 of 234 on identical arms, **230 of 285
on that exact shape**. `100`'s own prediction that it would catch a dropped coordinate was wrong, 0 of 312,
and chasing why is what produced the mechanism.

**`101`'s fourth option is cheaper and buys the same thing.** Requiring the named arm to be the **unique**
argmin gives 9 sections with 0 selecting a dominated arm, without forbidding a zero weight, so a strategy
may still declare it does not care about a coordinate. And since the whole 72-against-9 gap is one tie
between two arms declaring the same 13 bits, **a rule about ties settles it where a rule about the sign of
every weight settles it by forbidding something nobody wanted to forbid.** `101` also measured that the
section a zero weight selects is reproduced by strictly positive rates from `1e-1` down to `1e-12`, so
under strict positivity indifference is expressible to any tolerance and what is actually forbidden is
admitting an arm that only ties.

---

## 7. What the weighting ranges over, and the ceiling

`101` is the only member to look inside the second component, and it found the constraint that bounds
everything above it.

**The corpus measures one coordinate.** Of the harness CSV's seventeen columns, across 254 committed files
and 104080 rows, nine carry information and **exactly three vary between arms at a fixed region**:
`e2e_ns`, `algo_ns` and `bridge_ns`, which are one timing and its decomposition. Eight are identically
empty or zero: `cooldown_ms`, `score`, `input_tag`, `instructions`, `cycles`, `setup_ns`, `first_ns`,
`digest`. I verified the reachable-surface half independently: **0 of 94 variant crates implement
`score_output` or `score_dimensions`, 0 implement `max_relative_error`, 15 define `validate_output`, 1
mentions `outputs_may_differ`, and there are 254 committed CSVs.**

**A coordinate set is a countable ceiling on how many strategies can exist**, and it is exact rather than
approximate. On the committed carrier table with the control dropped, at strictly positive weights:
`{time}` reaches **1** section, `{time, size}` reaches **9**, `{time, size, spread}` reaches **42**.

The degenerate case is the one a design falls into without noticing. **With one coordinate the weighting
cancels and exactly one section is reachable**, by algebra rather than by the arms happening to be similar:
a coordinate set of size one is a design with one strategy wearing several names. So the sharp form, which
is `101`'s and which I would carry close to verbatim:

> A strategy whose intent names a quantity with no coordinate is not unmeasured. It is **inexpressible**.
> There is no axis along which it can differ from any other strategy, so it and its opposite are the same
> point in the space, whatever the canon calls them.

Section 8 is what op's intents do to that sentence, and the answer is that the sentence is untouched while
its antecedent fails for some of them.

**Normalisation is a change of basis on the weighting, not a decision about the costs.** A fixed
per-coordinate affine map with positive scale is a bijection on weightings preserving every section, since
`sum w'_i (c_i - b_i)/a_i` differs from `sum (w'_i/a_i) c_i` by a term independent of the arm. Measured at
2000 of 2000 identical sections on each of four families. Three consequences: **the weights carry the
units**, so "half speed, half size" is not a statement until the exchange rate is named; **a weighting is a
ray rather than a point**, so with `d` coordinates the space is `(d-1)`-dimensional; and the normalisation
question is therefore not a design fork, there is one model plus one thing not to do.

**The one thing not to do**, and this is `100`'s finding corroborated by `101` on three further families
with a different instrument. Under min-max normalisation whose range is read off the arm set, **adding an
arm that no weighting can ever select changes what every weighting selects**, at up to 6 of 6 regions, and
dropping an arm dominated at every region moves the section for **961 of 2000 weightings** on the carrier
family. Under raw coordinates neither changes anything, 6 of 6 and 0 of 2000 in 4 of 4 families, which is
the theorem. Under normalisation with the range frozen as declared constants, likewise nothing. So a design
shipping normalised costs states the range as declared constants, and `101`'s algebra says that costs
nothing because there is no second model to state.

**And a bench arm added as a negative control could then change what every strategy selects**, which is a
coupling between the instrument and the answer and is worth knowing before anything is built on a
normalised cost table.

**The coordinates split before they split into present and absent.** `102`'s measured-versus-computed line:
time is measured and carries the instrument's floor; bits per element is declared and exact; error against
an exact reference is computed and exact. The rule that follows is in section 1 and its predicate is
`103`'s.

**And a corollary that bites `100`'s own mechanism.** `100` states its tolerance band as a percentage of
"the region's achievable objective range", whose denominator is `worst - best` over the arm set and is
therefore data-dependent in exactly the way `100`'s own independence finding is about. `101` measured that
adding the same unselectable arm grows a 1% band by **59 to 185 times**, so a differential that refused a
defect before accepts it after. The replacement is `100`'s own instrument used in the right currency: the
control pair's apparent gap, a median of **0.273%** and a maximum of 0.544% of runtime, stated per
coordinate and per region relative to the coordinate's own magnitude, with no arm set in the denominator.
`100` built the calibration and expressed the band in a different currency.

**Two more things about estimation, and they cut against each other in a way that is the answer rather than
a tie.** `100` found the interquartile range a poor estimator of its third coordinate and swapped in the
95th percentile, strictly better on both axes it measured: 3 distinct sections against 161, and 54 of 60 arm
pairs separated against 43 of 60. `101` then measured the third axis nobody had: **on the carrier family
`{median, p95}` reaches one section, which is what no second coordinate at all reaches**, and the 95th
percentile correlates with the median at 0.978 to 0.998 across four families. The swap buys stability by
deleting the coordinate. And the two admissibility tests **anti-correlate at -0.64, -0.71 and -0.67 across
three families**, with a mechanism that is not subtle once seen: an estimator separates the arms well
exactly when it agrees with coordinate one, and agreeing with coordinate one is what makes it add nothing.

So the criterion is **position-dependent** rather than a balance: for the first coordinate, separation is
the test; for every coordinate after the first, high separation is evidence against; and the estimator's own
noise floor applies at every position. That is `101`'s and it is the sharpest thing in the coordinate work.

---

## 8. What the corpus can and cannot show, corrected

`102` claimed, in bold, that **every** committed region is answer-equivalent and every number in the corpus
compares cost at a fixed answer. `103` was dispatched to verify it, built its own instrument before opening
`102`, and refuted it. The refutation is worth carrying in full because the conclusion survives it and the
survival is the interesting part.

**The measurement.** Over the right unit, which is a committed CSV rather than a shared crate:

| | regions |
|---|---:|
| arms pinned to one value by an exact-value oracle | 234 |
| arms pinned only to a property, so free to differ | 10 |
| arms with no oracle at all, consenting to differ | 4 |
| single-arm, where the question does not arise | 6 |
| | **254** |

**Twenty committed regions are not answer-pinned**, and for eight of them the arms are measured to differ
or are two different algorithms by construction. Three independent refutations, each sufficient alone: the
two arms of `decimal-quantiser-radix-sweep` emit different denoted values on 97.12% to 99.95% of lanes at
all four committed sizes by exact rational comparison; controlling the input so both see identical exact
integers they still differ on **53.72% of 200000 trials**, with `binary32` strictly closer on 106167 and
`decimal32` strictly closer on **zero**; and the harness's own byte-exact gate, run on the harness's own
100 seeds, **refuses that family 400 of 400** while accepting the `quantiser-fadd` control 600 of 600.

**Two mechanical causes, both citable and both instructive.** `102`'s census unit is `variants/*-shared/`,
which structurally cannot see `fnv1a-vs-xxhash3`, whose bridge is declared inline as `ByteRoutine<N, 8,
true>` with `MAY_DIFFER = true`. And it treats the presence of a `validate_output` as answer-pinning, where
two of the fifteen validators check a **property** rather than a value, which is exactly what a property
validator is for.

**And a third finding neither was looking for**, which is section 3.2's reproduced result: the harness gate
`102` leans on was not running for 175 of 254 regions.

**What survives, and it is the load-bearing part.** `102`'s conclusion holds: the corpus cannot exhibit
I5, I7 or I9. But **the barrier is the absent coordinate, not the arm sets.** The corpus does contain
answer-differing arms, and one region with a strict accuracy ordering between them, and it records nothing
about that ordering anywhere: `score` is empty in all 104080 rows and 0 of 94 crates implement
`score_output`. That puts the finding on `98`'s and `101`'s rung rather than upstream of them, which is a
smaller claim and a better one, because an absent coordinate is something you can go and add whereas "the
corpus is structurally unable to hold such arms" reads as a property to design around.

**And the remedy is much smaller than the unit had been treating it.** `103`'s constructive half:
`quantiser-radix-shared` **already carries the mechanism the corpus is said to lack**. It has two oracles,
one per arm, and none across arms: the radix-two arm checked against native `f32` bit for bit, and the
radix-ten arm checked against the **definition** rather than against the other arm, because no silicon
exists to check it against, with both neighbours tested in exact integer arithmetic and ties-to-even checked
separately, 32768 checks each, and the check count itself asserted so a shortened sweep fails rather than
passes. The general shape, which is `103`'s and which I would carry:

> A region whose arms may produce different answers is validated **arm by arm, each against its own
> declared semantics**, rather than arm against arm. Cross-arm agreement is then a consequence where the
> semantics coincide, and its absence is not a defect where they do not.

The harness already has the consent switch, already has an intermediate bounded-disagreement regime that
zero variants use, and one family already writes the per-arm oracles. **The missing piece is
`score_output`**, one hook, which is a smaller and far better specified piece of work than "add the missing
coordinates to the corpus".

**And the cost-only question is refused rather than answered**, per `never-ask-which-single-rule-governs.md`,
because it asks for one verdict over a category:

| region | verdict |
|---|---|
| arms answer-equivalent | cost-only is correct and complete; a fidelity column would measure a constant |
| arms answer-differing, no fidelity column | cost-only is correct and **incomplete**; the missing column is `score_output` |
| arms answer-differing, no per-arm oracle | not a scope question at all; the region is unvalidated |

The third row is the only defect and it has four instances, all `fnv1a-vs-xxhash3`.

---

## 9. What op settled during the unit, and what it does to the members

Three op files arrived after the eight, and they outrank every member. Each changes something specific and
the changes are worth stating precisely rather than absorbed.

### 9.1 I3 is about ergonomics, and it lands on `102`'s side of a located disagreement

Asked whether, at a declared width Rust has no primitive for, the imitation targets the **declared width**
or the **container**, op took neither: *"Neither, it's ergonomics"* (`104` section 1). I3 is about the
experience of using the type, that it is unsurprising and that a reader who knows Rust's primitives is not
caught out. Where the arithmetic boundaries land is answered by the width and the overflow policy.

**What that does to the unit, in four places.**

`93`'s F8 measured that the two arithmetic readings disagree at all fourteen non-native widths it swept.
**The measurement stands and is correct**, and it is about something I3 does not range over. `93` was right
to hand it back rather than assume, and `98` was right to mark its own agreement with `93`'s reading as
inherited.

**`101` section 6's reading of I3 does not survive.** It states that I3 and I4 "both readings need a
**divergence** coordinate: how far this arm's behaviour sits from what a native primitive would have done".
If I3 names an experience rather than a quantity, it is not a weighting over anything, and no coordinate
carries it.

**`102` is right on I3 and reached it from the design side before op spoke.** It argued I3 belongs in the
policy component, that "how far is wrapping from saturating" has no units, and that "`93`'s observation was
right and its resolution was wrong". Op's answer is not a ratification of `102` and must not be read as
one; it is a fact about I3 that lands where `102` said it would.

**So the `101`-against-`102` located disagreement resolves on I3 and narrows elsewhere**, and this is the
first of the three collapses this file reports. `101`'s sentence, that an intent naming a quantity with no
coordinate is inexpressible, is **untouched**: what changes is that I3's antecedent fails. And the two
files were counting different pairs of intents in the first place: `101`'s two-with-nothing are I7 and
I3/I4, `102`'s two-not-weighting-shaped are I5 and I3/I4. On I5 they agree more than they appear to, since
`102` concedes I5's bar "wants a sound-against-unsound bench that does not exist", which is `101`'s point
restated. **The genuine residue is I7 alone**, and `103` resolved that too: the corpus does hold
answer-differing arms with a strict accuracy ordering, the barrier is the absent coordinate, and the remedy
is one hook. `101` was right about the diagnosis and `102` was right that the remedy is reachable.

### 9.2 I18 is new, and reading it against the object finds a gap nobody could have seen

I18 permits a native-primitive-style overflow panic, with I15 bending for it, bounded to dev and debug
builds and to the concern where imitating the native primitive is the point rather than where cost is, and
held as a rule of thumb rather than a gate. Op corrected his own first wording and said the marker names in
both statements are vehicle rather than intent.

**No member could have read this against the object, because it arrived after all eight.** Reading it now
produces two things.

**One: it corrects a member finding, precisely.** `93`'s F9 states that overflow detection matching Rust's
debug behaviour is available where both operands are const and nowhere else, and adds as its complementary
claim that "for operands that are not const-available there is nothing, by I15, not a weaker check and
**not a debug-only one**". The finding's predicate is about const-evaluable operands and is untouched.
**The complementary sentence is exactly what I18 overturns**: a debug-only check is now the licensed case.
`93`'s T1, which handed the I3-against-I15 tension back rather than resolving it, was the right call and is
now answered.

**Two: it exposes one word missing from the definition of observable, and the fix is one clause rather than
a mechanism.** `97` defines an observable coordinate as one whose movement **changes the value the program
computes**. A panic produces no value, so on that definition the panic axis is not observable, and a build
arm may therefore move it freely, which is what I18's build bound does. That is the right answer and it is
reached by an accident of wording: a program that aborts and a program that returns are plainly
distinguishable by a consumer, and a definition that does not say so will be read literally by somebody
building on it.

So the definition wants: **observable means the assignment changes the value the program computes, or
whether a value is produced at all.** I18 sits in the second half, licensed explicitly by op, bounded to
non-release builds, which is why it does not reopen I15: a shipped program still contains no runtime
validation. Stated as a clause on a definition rather than as a new axis, because that is all it is.

**And one consistency worth naming without overclaiming.** Op bounded the panic "by concern", to where
imitating the native primitive is the point and not where cost is. That is op distinguishing the imitation
concern from the cost concern in his own words, which is the same line `102`'s pair draws between its two
components. It is **consistent with** the pair rather than evidence for it: he has drawn that line since
I2 through I7, and drawing it again is not a ruling on a decomposition he explicitly declined to rule on.

### 9.3 I9 returns to the panel with a decision procedure

Asked whether I9 describes the strategy pair or only its policy half, op declined: *"I think the intent is
clear and this is impl detail that already had answer: optimal and converged to by experts (plural,
iterative)"* (`104` section 3). The decline is the answer. It is not a call he owes, it must not be put to
him again, and it stays live as an implementation question. Section 10 is my attempt at the procedure he
named.

### 9.4 notko renames

Op, at `105`: *"notko renames"*. The name `Strategy` belongs to arvo's concept and the canon may use it
without qualification. This closes Q46 and it retires the disambiguation burden `93` reported outside its
question. It does not name notko's replacement and it is not a licence for this panel to edit notko.

---

## 10. Q50 attacked, since op returned it rather than ruling on it

**Marked as a contribution rather than a compression**, because a consolidation should mostly not do this
and should say when it does. This is one expert, it is mine, and it needs the second read every ONE EXPERT
claim needs.

The question: does I9's "strategies are the variables that change what the 'correct' answer is" describe
the pair, or only its first component. `102` states both readings and says it can build either. Op's
procedure is convergence on whichever is optimal.

**There is evidence on it that nobody connected to it, and it is measured.**

The two components have **different carriers**, and this is not a preference. `94`'s W9 measured policy on
the value with the plan supplied at the site: four sites, three sharing one value type, **zero conditional
instructions and zero casts**, with the same value type folding three different ways at three sites, and
the policy still travelling with the value so that two sites running the same plan get different arms
because the values differ. And `97` section 5 decided the one disagreement `93` and `94` left located, using
`warm-clamp-arity-w13`, which holds the declared width at 13, the element count at 8192 and the transform
fixed while sweeping the fold's arity through six points: **the best arm moves**, from the minimum container
at arity 2 to the arity-derived accumulator from arity 4 upward, three distinct contending sets across the
six arities. One stored column, two different right answers, decided by something the column does not know.

Both point the same way. **Component one is a property of the value; component two is a property of the
site.** Under type-carried cost, folding one column two ways requires a cast that changes no value, which is
free at runtime and not free in the design.

**So the argument.** If the two components live in two places, then a single word naming both names one
thing that is in two places, which is precisely the failure `94` diagnosed in the flat marker set: "a flat
set forces two roles through one slot". That is an argument for I9 attaching to the first component.

**And the counter-argument, which `94` also flagged and which I think wins on op's own priorities.** A
preset exists so that a consumer states one intent rather than answering two questions. I2 and I4 both
stress the intent and the intuition. Splitting the axes across two carriers means a consumer who wants
"fast" says it in two places, which is worse ergonomics, and I3 as amended by `104` is precisely about
ergonomics.

**My reading, which resolves both.** The word "strategy" as op uses it names **the named binding**, and a
named binding fixes one point in each component. I9 is then true of it, and true **because of** its first
component. Neither reading is wrong; they are true of two different objects, and the decomposition is:

> A **named strategy** binds one point in the observable assignment and one in the weighting. It is what a
> consumer writes and what op's intents describe. I9 is a statement about what that binding does, and the
> part of it that does so is the first component.

That keeps op's word attached to the object he was talking about, keeps the ergonomics I2 and I4 demand,
and keeps the two carriers the measurements found. It also makes `88`'s "mostly option 1, a little bit of
option 3" read cleanly: mostly the point, with a weighting attached.

**What would distinguish this from `102`'s two alternatives.** Whether anything ever needs to name the
second component on a value's type. If nothing does, the two-carrier reading holds and the binding is the
right object for the word. `94`'s W9 and `97` section 5 both say nothing does, on two different
instruments, and both are measured. If something does, the pair is one object with one carrier and this
reading is wrong.

**What it costs if I am wrong.** Nothing structural. The three readings agree on every mechanism in section
6 and differ only in which object the word names, which is why op declined it. The reason it is worth
settling at all is that the canon has to use the word.

---

## 11. Chains, which are a different shape and are the one place the object bends

The cold pair converged on this blind and nothing after them disturbed it, so it is stated compactly.

**A preference over a chain cannot be served by an operator closed over its operand type.** `93`'s P6
measured at `W = 8, F = 4` that rounding at every step against rounding once at the end differs on 8.1% of
triples at chain length 3 with a maximum absolute error of 238 raw units in a domain of 256, and that the
width required for the round-once arm grows linearly in chain length: 16 bits at length 2, 64 at length 8,
against an 8-bit input type. `94`'s probe E measured the same at `W = 16, F = 8` and derived the growth in
closed form, `k` multiplies at `I.F` growing to `kI.kF` and an `n`-term fold needing `ceil(log2 n)` extra
integer bits, with per-operation rounding diverging by up to **11,922,158 ulps** at a five-multiply chain
where the widening construction stays at 1 ulp. **The add fold is essentially free and the multiply chain is
what runs out**, linearly, and the chain length is compile-time visible wherever the chain is written.

**So chain accuracy is a representation discipline rather than a policy**: do not quantise in the interior,
let the declared width grow, and let the consumer say where the quantisation happens. Under that, the chain
is visible because the widths are, and no expression-tree machinery is needed.

**Two corrections to that arrived later and both sharpen it.**

`101` found that **accuracy over a chain cannot be a per-arm scalar**, because the per-operation and chain
rankings **cross**: an arm on a finer grid with truncation is twice as accurate at one operation and four
times worse at sixty-four, with the crossing at `k = 4`, because a bias accumulates linearly and an unbiased
error accumulates as a random walk. Its constructive answer keeps the table's shape: **chain length is a
region dimension rather than a coordinate**, which is what the corpus already does for thread count.

`102` then attacked that from both ends and both attacks improved it. **The crossing carries a predicate
its own statement does not: the chain must be non-contracting**, because a contraction damps error rather
than accumulating it and there is nothing inside one for an accuracy-weighing strategy to decide. And **the
two arms are the two ends of a family with `k + 2` members whose interior is on the Pareto front**, 65 of 66
arms at `k = 64`, with a sweep of exchange rates selecting six distinct ones. Reading the crossing as "pick
A below it and B above" takes two points from a front that has sixty-five, and **which interior point a
strategy takes is decided by a weighting**, which is the first instance in the unit of a strategy weighing
something other than time and bytes.

**And the depth reaches a const predicate with no forbidden feature**, which nobody had checked. The natural
spelling increments a const generic, `Fx<{D + 1}>`, which is arithmetic in a type argument and needs
`generic_const_exprs`. `102_probes/p4_is_chain_depth_const_available.rs` carries depth as a **type** with
the number as an associated const, so `D::VALUE + 1` is ordinary const evaluation in value position. It
compiles `#![no_std]` with zero feature gates on the pin, and a four-step chain switching policy at depth 2
emits **zero conditional branches**, truncating at its first step and rounding at its last three, with a
wrong depth failing at build time with `E0080`. That is `a-refused-bound-wants-a-trait-not-a-feature.md`
landing on a case nobody had pointed it at.

**And `102` cut against its own result honestly, which is the part to carry.** The three emitted sequences
are 9, 10 and 10 instructions, so on aarch64 rounding to nearest costs **one hoisted `mov` for the whole
chain** because the add-half fuses into `madd`. Its second coordinate is a count of round-to-nearest steps,
and on that target the count does not correspond to a per-step cost at all. So the arm family is real as an
accuracy structure and **its trade is a fact about the target**, unpriced, needing the harness. That is also
`25` section 7's own clause arriving from an unexpected direction: here is an axis whose existence as a
trade is a function of the build condition.

---

## 12. What this file refutes or corrects, listed so the ratio is checkable

Five, against thirty-one stated. Each carries what replaces it.

**One. `102`'s "every committed region is answer-equivalent".** False by twenty regions, on three
independent measurements plus two mechanical causes. Replaced by section 8's census and by the corrected
sentence: the corpus records cost and records no answer, and the barrier to I5, I7 and I9 is the absent
coordinate rather than the arm sets. `103`'s, verified here on the one part it asked to have re-derived.

**Two. `98`'s "there is nothing to check and nothing to police" under generation.** The first clause holds
and the second does not follow. Replaced by section 6.1: generation removes one defect class and admits
another that rationalisability cannot see, 0 of 489, and the composition in section 6 keeps both authors'
wins. `100`'s.

**Three. `100`'s estimator swap.** Right about resolution, wrong about what to do: the 95th percentile
reaches one section on the family it was measured on, which is what no second coordinate reaches, so the
swap buys stability by deleting the axis. Replaced by section 7's position-dependent criterion. `101`'s.

**Four. The characterisation of `bitpack-shared`'s tests in four member files.** `102`'s "no cross-arm
agreement assertion of either kind" is false and `103` inherited the wrong half of it while correcting the
right half. Replaced by section 0.3, which quotes the body. Mine, and it changes no downstream conclusion.

**Five. The rung the polarity distinction sits on.** The register and the checkpoint read as settled;
`102`, the file usually taken as the second instance, states in its own section 2.5 that it read `97`
before building and that a second reader deriving polarity independently "would earn the rung. I did not."
Replaced by section 3.3's classification. Mine, and it is a bookkeeping correction rather than a doubt about
the distinction, which I keep in full.

**And two member findings that op's later files correct**, listed here for completeness though they are not
mine: `93`'s F9 complementary sentence, overturned by I18 (section 9.2), and `101` section 6's reading of I3
as needing a divergence coordinate, overturned by `104` (section 9.1).

---

## 13. Live options, carried forward

`RULES.md` records that both prior consolidations lost a live option, each found only by the check
afterwards, and names the mechanism: an option no member resolved has no result attached, so there is
nothing for a compressor to grip, and **the options most likely to be lost are the ones the panel most needs
carried**. So this pass is separate from the results pass, and it lists each option with what would
distinguish it.

### 13.1 Open forks, with their discriminators

**The generate-against-check fork, which is the unit's largest unclosed item.** `98` proposed inverting
`97`; the checkpoint told the second four to attack it first; `100` attacked the surrounding machinery and
`102` the coordinate split, and **nobody attacked the fork itself**. It stands at one expert plus `93`'s
unregistered fork, unchanged across four files. What would distinguish: whether a stated weighting can be
written down precisely enough to generate a table at all, which needs the coordinates commensurable.
Section 8 says two of op's four intents have no coordinate today, so the answer is currently no for half of
them, which is `98`'s own argument for its proposal being the target and `97`'s the interim.

**Which object the word "strategy" names**, per Q50 returned by op. Three readings: the pair, the policy
half, and the named binding (section 10, mine). Discriminator: whether anything ever needs to name the
weighting on a value's type. Two measured instances say nothing does.

**Whether the rationalisability constraint has content once the pair is in place.** Section 4(a). `97` and
`102` read op's `88` mix two different ways and neither has been tested against the other.

**Q44's four options**: strict positivity, non-negativity, non-negativity with a separate dominated-arm
check, and `101`'s unique-argmin. Section 6.3 measures that the fourth buys the same guarantee more cheaply
and that the gap it is about is a single tie.

**Q45's three options** on arms no weighting can select, with `101`'s discriminator: keeping them is safe
exactly when the coordinates are not normalised against the arm set, and unsafe otherwise, so Q45 and the
normalisation question are the same question asked twice.

**Q48's four options** on the coordinates: add them, use the region instead, declare the ceiling, or
`103`'s add-fidelity-only-where-arms-differ, gated on the region's own declaration. The fourth composes with
the other three rather than competing, and is the smallest.

### 13.2 Options living in exactly one file, which is where a consolidation loses them

**`93`'s sixth axis: reproducibility across targets and builds.** `93` reports it as demanded by consumers
doing lockstep simulation, deterministic replay or content addressing, orthogonal to the five it names, and
**currently unnameable in the design**. `98`'s p10 is a second instance from the machine side: a section
that moves under a resample of its own bench run is a design whose emitted code is not reproducible across
builds. `98` says it would carry the axis forward "as a real axis rather than as a suspicion". **No option
register entry carries it.** It is the single option most at risk in this unit and it bears directly on
section 6.2, since generating per build is licensed for unobservable coordinates only.

**`94`'s R1/R2/R3 on what a marker is a claim about**, and its W8 underneath them: an operand-level accuracy
bound in ulps **is preserved by addition and is not preserved by multiplication**, by a factor up to the
other operand's magnitude. So the guarantees are not closed under the operations the type supports, and **no
finite marker set can carry an accuracy claim that composes, whatever the set's size**. `97`'s F3 and `94`'s
W8 kill two different readings by two different arguments, and together they say the marker is operational
or it is nothing, which is the ground under R3. In one file.

**`94`'s correctness-predicate-against-profitability-predicate distinction.** A correctness predicate must
be const; a profitability predicate merely wants to be. Choosing a reassociated arm where the law does not
hold is wrong at every length; choosing it where the run is too short is slow and right, so the second can
be resolved pessimistically without lying and the first cannot be resolved at all. `97` notes op's `83`
explicitly left open what happens to a non-const-available condition and that nothing has touched it. In one
file.

**`102`'s ring-boundary licence.** Headroom and intermediate precision are unobservable across any
composition of `+`, `-` and `*`, so a resolver may widen or narrow freely there without telling anybody.
Large region, const-visible, unpriced, and `102` says plainly "I ran out of question".

**`101`'s strategy margin.** At two coordinates the section is a piecewise-constant function of one number
and the pieces are intervals, 8 to 12 cells per family, with interior cells **0.29 to 1.48 decades wide**.
Two consequences nobody has built: two weightings in one cell are the same strategy, so a declared rate has
to be right to within a factor of a few rather than right; and a strategy has a **margin**, the distance to
the nearest cell boundary, computable at const time from the same table, in the exchange rate's own units.
"This weighting is 0.4 decades from changing its mind" is the honest thing to report next to a strategy.

**`101`'s log-scale objective.** The unit has assumed a linear objective throughout without anyone saying
so, `101` included. A weighted sum of logarithms is a weighted geometric mean, scale-invariant per
coordinate, which would dissolve the units question rather than answering it. Different objective, different
laws, and every count in section 7 would have to be recomputed. Untested.

**`100`'s single-declaration generator for the region grid, arm registry and key encoding.** The region set
is currently written down three times, in the crate's key encoding, in `bench.toml`'s integer literals and
in the block title's prose, joined by nothing. `100` audited the eight `warm-clamp` blocks and found **zero
disagreements**, which is still a result: the twins agree today and nothing keeps them agreeing, and a
member of this unit had to decode an integer key by hand to read a region. A design that automates the
weighting-to-table join and leaves the manifest-to-decoder join manual has automated the one that has not
yet gone wrong.

**`100`'s compile-time bench arm.** The only thing that would price section 6.2's one remaining cost. The
harness has no compile-time arm and building one is upstream work in mockspace. Live, unbuilt, on a scope
call `100` marks as attackable.

**`100`'s per-coordinate tolerance band.** `100` lists it as live and untested; `101` then closed it on the
algebra, since a single band across coordinates in different units is not expressible without a
normalisation and the normalisation is a weighting, so a global band's width depends on the weighting it was
meant to be independent of. Recorded as closed rather than open, because a closed option nobody records gets
reopened.

**`103`'s per-arm-oracle shape for answer-differing regions**, section 8, and the three-step cost of
building one: declare the consent, give each arm its own oracle, implement `score_output`. Only the third is
real work.

**`101`'s instruction-count coordinate.** `instructions` and `cycles` need a feature flag and root on the
Apple-Silicon host every committed run already used. It is the one candidate coordinate that does not move
with machine load. Live and unbuilt, and `101` declined it on the ground that taking a privileged bench run
on a shared clone is not its call, which is right.

**`101`'s `setup_ns` breakeven.** The harness derives `k* = (S_b - S_a) / (I_a - I_b)` directly from a matrix
run, which is a **region boundary computed from measurement rather than declared**. That is the shape every
predicate in this panel has been reaching for by hand. Live, unbuilt.

### 13.3 Options this file closes, with what closed them

**The four responses to cross-strategy resolution as four competing designs.** Closed by `97` section 4.4:
they are the correct answers to three different questions plus one wrong one, and section 5 states which is
which.

**A divergence coordinate for I3.** Closed twice over, on definition by `102` and by op's `104`. "How far is
wrapping from saturating" has no units, and I3 names an experience rather than a quantity.

**A value-level accuracy lattice over a finite marker set.** Closed by `94`'s W8 on arithmetic: the bound
does not survive multiplication, at any set size.

**The flat four-element marker set carrying a lossless resolution.** Closed by `93`'s F3, a counting result
about the set that no table repairs, and superseded constructively by the free join semilattice in section
5.

**Incrementing a const generic to carry chain depth.** Closed by the forbidden-feature list and replaced by
the trait route, compiled.

---

## 14. What the unit did not establish

Stated plainly, because a consolidation that reads as complete is worse than one that names its edges.

**Whether the pair is right.** One expert, unattacked, arrived seventh. Section 4 separates what inside it
carries more than that.

**The generate-against-check fork.** Section 13.1. Four files after the checkpoint told them to attack it
first, it is where it was.

**Anything about threads.** Every finding in this unit is `threads = 1`, and under I13 that is a real region
rather than a silence. **With one correction `101` made and which applies to this file too:** three of the
four control-bearing bench families are threaded, and their region key encodes the thread count in its last
digit, so a finding computed over `bitpack-contention`, `bitpack-contend-decode` or `bitpack-wide` spans
threads 1, 2 and 4 and `threads = 1` names a region it does not live in. Where a finding above is computed
over those families I have written the decoded set.

**Compile time**, everywhere, in that word. There is no compile-time arm in the harness and building one is
upstream work.

**Whether the axis set is complete.** `93` lists six and believes the sixth is missing; `102` measured four
of `25`'s at one width on one arithmetic and says the pair does not depend on the list being right while its
application does. `25` section 8's open question about whether the arithmetic column is one axis or two is
untouched, though `102`'s p2 adds that headroom and intermediate precision have the **same** observability
predicate, so if they are two axes they are two a consumer cannot tell apart except past a non-ring step.

**Whether the harness's bounded-disagreement regime is a cost coordinate or a validation gate.** `102` found
it, `103` confirmed zero variants use it, and neither could settle whether a validation tolerance and a
scored coordinate should be the same object. It wants somebody who has actually run the harness's quality
path, which is nobody yet.

**Whether the 155 pre-wiring answer-pinned regions in fact hold agreeing arms.** Their families' tests assert
it and pass, which is why `103` believes it and why I do. Nobody re-ran the arms. A rerun under the
now-wired driver would settle it and that is a bench job.

---

## 15. Anchor accounting

Per `a-compression-is-checked-by-someone-else.md`, and per the tier rule: for a canon candidate,
panel-internal and probe citations count and must survive, while citations into a nuked or superseded tier
do not and should not be restored.

**The union across the eight member files is 207 unique path anchors**, of which **125 are probe files**, 15
are in the live bench tree, 13 are workspace rules, and **4 are the superseded root design templates**
(`mock/DESIGN.md.tmpl`, `mock/PRINCIPLES.md.tmpl`, `mock/WORKFLOW.md.tmpl`).

**This file carries 34**, deliberately, and the classification of what is dropped is the point. The census
and the set difference are `106_probes/p6_anchor_census.sh`, which excludes this section from its own scan,
because an accounting paragraph that names the anchors it dropped makes them present and silently disables
the instrument it was written for.

- **Probe anchors carried:** the ones each stated claim rests on, named at the claim. **Probe anchors
  dropped:** the remainder of the 125, which support claims this file compressed away or did not carry.
  They are not lost: every member file is intact beside this one, and `87` fixes that the canon is written
  from the consolidations **read alongside the members they compress**. A dropped probe anchor here is one
  hop away rather than gone.
- **Superseded-tier anchors dropped on purpose, and not to be restored.** All four. `93` and `94` between
  them cite `mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl` nineteen times, for the four-marker set,
  for `Resolve<S1, S2>`, and for the list of what a marker "drives". Both documents now carry a superseded
  banner, both were asserting as settled the thing I1 demoted, and `mock/PRINCIPLES.md.tmpl` names a
  forbidden feature. A canon candidate anchored to them would be anchored to a dead tier. Section 5 keeps
  the **content** of the `Resolve` finding, which is that a total silent join is the wrong mechanism, and
  drops the citation into the document that proposed it.
- **Line-number anchors converted to heading anchors throughout**, per `how-to-run-a-panel.md` and because
  this unit paid for it twice: `93` and `94` carry nineteen line citations into the two templates that are
  now low by exactly eight, and `101` had **fourteen of thirty-seven citations fail on its first run, eight
  of them because `100` grew by 46 lines underneath it while it read.** I cite `INTENTS.md` by entry heading
  and members by section, not by line.

**One anchor recovered rather than carried.** Section 3.2's reproduction of `103`'s pre-wiring finding
depends on the `-dirty` suffix being harness noise, which is established at `22:188-193` and is cited by no
file in this unit. Recovering it is what makes the reproduction checkable by the next reader rather than a
number they cannot get to.

**The instrument caught me, which is the reason to run it rather than to describe it.** Its first run
reported **one superseded-tier anchor carried**, against this section's own statement that it carries none:
section 5 named `mock/DESIGN.md.tmpl` as the source of the resolution mechanism it was refuting. The
sentence is rewritten to name the mechanism without citing the dead document, the content of the finding is
unchanged, and the count is now 0. A consolidation that states a rule about anchors and then breaks it in
its own body is worth exactly nothing, and the only reason this one did not ship that way is that the rule
was written as a script.

**What the check after this file should run.** The set difference over panel-internal and probe anchors,
excluding section 15 for the reason above. The superseded-tier four should be **absent** and their absence
is correct. And it should re-run `p4`, whose 49 of 49 is a measurement rather than a property of a check
that cannot fail: three mutants, a phrase op did not say, a real phrase attributed to the wrong file, and a
near-miss on a real quotation, are all caught, and the record is in the probe's output.

---

## 16. Coverage, bounded rather than claimed

**Read in full:** `INTENTS.md` including I18, `RULES.md`, all eight member files `93`, `94`, `97`, `98`,
`100`, `101`, `102`, `103`, op's `95`, `104` and `105`, the dispatcher's `96` and `99`, and `OPTIONS.md`
entries Q43 through Q50 with their addenda and closures.

**Not read:** every panel file before `93` except `25` section 7 and `40` sections 0 and 3.2 through the
members' quotations of them, `DROPLIST.md`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`, the `SEED_*` files, and
the archive. So where any of this restates something earlier in the panel, I do not know it. In particular I
read `25` and `40` only through `97`, `98`, `101` and `102`, which matters because sections 4 and 5 lean on
`25` section 7's wording and on `40`'s two-space split, and my account of both is inherited.

**Verified independently rather than taken from a file:** the test suite, 123 across 13, run per crate; the
corpus counts, 94 variant crates, 0 `score_output`, 0 `score_dimensions`, 0 `max_relative_error`, 15 crates
defining `validate_output`, 1 mentioning `outputs_may_differ`, 254 committed CSVs; the existence of twenty
cited probe files across six probe directories; `bitpack-shared`'s test body and validator; the three
control-arm module headers; and `103`'s 175-against-79 join, rebuilt from the meta files and `git log`.

**Not verified, and named:** every timing figure, every bootstrap, every exact-rational law sweep, and the
rationalisability counts. All are read from committed probe output or committed harness output produced by
somebody else. I ran no bench and took no measurement, and where nothing has been priced I have written
unpriced.

**Citations checked by opening them.** Every quotation and named claim in this file is opened and its
content tested by `106_probes/p4_check_my_own_citations.py`, which normalises whitespace and strips
blockquote and doc-comment markers on both sides, because a verbatim quotation wrapped across lines or
carried inside a `>` block is still verbatim. **49 checked, 49 passing**, and the instrument is
mutation-tested rather than trusted for coming out green.

**One number I did not attempt.** A fourth implementation of the rationalisability decider. `101` gives the
reasoning and I agree with it: three independent implementations agreeing is the bar, and a fourth is an
echo rather than an instance.

**The probes.** Seven, in `106_probes/`, each committed with its output: the test gate run and the
instrument defect that made it read green at zero tests; the corpus counts with the build-artifact
contamination shown beside the clean numbers; the pre-wiring join rebuilt from the meta files and `git
log`, with the `-dirty` precondition stated in its header; the citation checker and its mutation test; the
`bitpack-shared` body extracted whole; the anchor census and set difference; and the ratio count. I ran no
bench and every number in them is a computation over committed artifacts or a suite run.

**The largest thing I did not do.** I did not attack the pair. It is the unit's strongest candidate, it
arrived seventh, `103` says plainly that nobody attacked it, and a consolidation is the wrong instrument
for it: attacking it requires an expert who has not read it, and by the time this file exists there is no
such expert in the unit. Section 4 does the next best thing, which is to separate the three claims inside it
so they can fail independently, and to say which parts of the structure are older and better supported than
the packaging. **If one dispatch follows this consolidation, that is what it should be.**

---

## 16. Droplist repairs, restored on `107`'s check

`107` found four one-file results absent from section 13.2's droplist, whose whole purpose is catching
exactly that class. Each is restored here from its establishing file rather than from the check's
description of it, because a repair that copies the checker has compressed the compression again.

**`97`'s F-H: a declared non-negative operand window recovers three laws.** The unit's only positive law
result, and its cleanest I13-shaped arm: a region where something that generally fails does hold, which
is what a predicated arm is made of.

> **F-H. Restricting a signed saturating type to a declared non-negative operand window recovers
> additive associativity, multiplicative associativity and distributivity over addition, all of which
> two-sided saturation loses.**
> `holds for: W in {4, 5, 6}, F = 0, signedness = signed, overflow = saturate, operand window =
> declared non-negative, operations {add, mul}, arity 3, values exhaustive, threads = 1, target
> features any`

`97` records that this was a **prediction of the criterion made before running**, on the ground that a
one-sided clamp is a congruence and a two-sided one is not, and that it independently retrodicts `82`'s
declared-window result, which `97` had not read. That makes it a prediction, a confirmation, and a
cross-file convergence at once, and dropping it lost all three.

**`97`'s F-B with `98`'s F-98-5: the 72-and-9 counts are a property of one table, and the bound is not.**
F-B states the gap between sections and argmins is **polynomial against exponential in the number of
regions**, which is what generalises the counts beyond the table they were measured on. F-98-5 states
the ratio of weighting-realisable sections to Pareto-admissible ones **varies by a factor of 47**. The
two belong together: the bound is structural and transfers, the ratio is not and does not, and a reader
given only the counts will take a number for a law.

**The exchange-rate reading of op's four intents, three-instance across `40`, `98` and `102`.** Each
intent names a concern and then declines to make it absolute, and that declining is the difference
between a lexicographic priority and a finite exchange rate. `98` measures the consequence: 4 available
behaviours under a priority reading against 58 on the real table. With it goes **`98`'s F-98-7**, that a
selection rule of the form "minimise one coordinate subject to a bound on another" is **not** expressible
as a weighting, which is the boundary of the whole weighting mechanism and was in one file.

**`98`'s five-rung ladder, including the 144 Pareto-admissible rung.** Of 46656 sections over the two
coordinates that survive scrutiny, **144 are Pareto-admissible** and 9 survive a strictly positive
weighting. `98` states that the Pareto rung is the only reading on which op's "a little bit of option 3"
carries content, so dropping it drops the interpretation that makes his own answer mean something.

**Why these four and not others.** Each exists in exactly one member file, none is reachable from any
other carried claim, and the canon is written from the consolidations. A one-file result dropped here is
gone at that point, which is what makes this class severe rather than untidy.

## 17. What the check found that is not repaired here, and why

`107` reports three further defects that stand as recorded rather than fixed, because fixing them would
be re-compressing rather than restoring:

- **The defence of the 177 dropped anchors quotes half its own source.** `87:26-29` says a dropped
  finding is recoverable, *"which is why ... a dropped item is a defect rather than a closed question"*,
  and section 15 carries the first clause only. The omitted clause is the one that makes the drop a
  defect. Left as written, with the full quotation recorded here.
- **`93_probes`, `94_probes` and `103_probes` are at zero anchors carried**, and 302 of 347 member
  citations point at whole files rather than at locations. That is a real loss of checkability and it is
  not repairable by editing this file; it is a fact about how this consolidation was written.
- **The census script computes no set difference** despite section 15 saying it does. The set difference
  `107` ran is the one that found the four items above, which is the argument for the instrument
  belonging to the checker rather than the author.

## 18. The pair attacked, and what it is after `108`

This consolidation named one thing it had not done: nobody had attacked the pair. `108` did, against
section 4's separation of it into independent claims. **The pair survives as a two-component object and
five of its eight clauses need repair**, each with a replacement drawn from material this unit already
held. Its section 7 carries the converged statement with all five applied, and that statement supersedes
section 4's rendering of the pair. Whoever writes the canon takes it from there rather than from here.

**Clause three was false, and it is the one that mattered.** Component two was defined as ranging over
"the arms that produce the answer the first component fixed". Section 8 of this file states that in that
region a fidelity column **would measure a constant**. So op's accuracy intent is expressible in neither
component, while `102:125` says the mechanism "serves I5 and I7". Both cannot hold, and `102` states the
two halves a page apart. **The repair: component one fixes the denoted answer, not the computed one.**
That single change restores the region in which an accuracy coordinate can vary at all, and it is why
this correction reaches further than the other four.

**Clause two had lost a qualifier**, the same failure class `107` found in the law bullet. "Observable"
as used here is `40`'s definition rather than `97`'s, and `40` attached a condition no file in this unit
carries. `108` measured it: same overflow-policy assignment, limit read at the declared width against
the container width, **0% against 89.081%**. And observability is a property of the **chain**, not of
the axis. The repair is a conservative closure plus a per-chain licence, with a sound const-checkable
predicate and **zero unsound predictions over 8019 exhaustively swept chains**.

**Clause seven was false because one word named two objects.** It said nothing relates two second
components; section 5 of this file says the join is union and free. `108` separates them: **supports
join canonically, 9 of 9; rates do not**, with six combinations disagreeing on **71.4% of 42 rate
pairs**.

**Clause four's third sentence was inconsistent and the fix is free** (three encodings compile to two
symbols, four of six being aliases), and **clause eight undercounts** by the size of component one.

**Section 4's leg (a) is void.** Op's `88` sentence has **five incompatible readings** across this
panel, he flagged his own difficulty wording it, and `104` supplies the test it fails. Leg (b) is right
about I3 and wrong about I5 once clause three is repaired, which restores `40` section 5.3's reading.

**A rung this consolidation failed to record.** The two-level structure is **`40`'s, at TWO EXPERTS**,
being `40` plus `93`'s blind phase one, and it appears here at no rung at all. `93`'s own claim of three
instances overcounts. What is genuinely `102`'s contribution is narrower and should be stated as such:
**the relocation of the observable assignment into the strategy.**

**A methodological finding worth carrying past this unit.** `108`'s citation probe tests **both
directions**, which no predecessor's did, and the reverse direction caught three misquotes in its own
draft, every one of them inheriting a later file's rendering of an earlier file's words. That is the
same mechanism as the qualifier losses `107` found, caught by an instrument rather than by a reader.
