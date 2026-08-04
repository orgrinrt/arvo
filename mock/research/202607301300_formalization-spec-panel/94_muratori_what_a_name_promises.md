# 94. What a name promises: the class test is keyed on the compiler and the reader is the one observing, three ratified names diverge, and one of `IeeeDefault`'s three artifacts cannot fail

Casey Muratori, second read on the refined naming principle adopted at `90b` and carried at
`91` section 1.27. I wrote files 06 and 47.

**What I read.** `91_consolidation_nine.md` in full, per the lead designer's standing instruction that
it is the only required reading, and `ls` of the panel directory. Then `90_torvalds_the_name_and_the_reckoning.md`
sections 0, 1, 2 and 7 (the proposal I am second-reading and what it left open), and
`90b_persona_checkpoint_twentytwo.md` in full (the adoption). Then, to check claims the consolidation
compresses rather than states, the two ratified preset tables at `78:409-441`, file 71 section 7 (the
declined NaN-on-overflow option and where it routed the mode), `78:295-330` (the same, as the
consolidation absorbed it), and `90_probes/probe_1_the_receipt_assertion_is_three_instructions.rs`.

**What I compiled**, all of it in `94_probes/`, five probes, three targets between them, outcomes in
`94_probes/OUTCOMES.md`: the x86 form of the receipt assertion owed at `91:1011-1012`; a run check of
whether file 90's own aarch64 mask covers what the name denotes; the two ratified preset tables
compared name by name across both number kinds; the receipt re-derived from the denotation and priced
in emitted assembly; and the pricing-pillar-correct associated-const form of that derivation.
Everything else below is reasoning, and is marked.

**The verdict, stated first.** The refined principle is a real improvement on `67b` and survives its
sweep. Its second clause, the trusted-base entry with a closing artifact, is the right mechanism and
costs no good name. But its **first** clause is keyed on the wrong observer, and the claim attached to
it at `91:850-851` ("Every existing name in the design passes") is false at three names, one of them
compiled here. The class test asks whether the compiler checks the claim. A name is not read by the
compiler. It is read by a person at a call site who is doing something else, and the class that
matters is what that person concludes. Those two classifications come apart at exactly the
instantiations this review has spent nine consolidations learning to look for, and the design's own
separation requirement already names the repair.

On the specific question I was sent to attack: `79b`'s intent-pillar license extends to an
environment parameter for one half of what the name claims and not the other, and the split is
visible in the artifact list. **One of `IeeeDefault`'s three named artifacts, the parity suite, is
structurally incapable of failing on the claim it is listed against.** It should come off the list.
The assumption-not-witness sentence survives the NaN-on-overflow attack, but only once it says
*ambient* control state, because the mode file 71 routed to this type is not ambient, is not an
assumption, and under the fourth design rule must not be modelled as one.

---

## 0. The two gates, before the assigned work

**Canon gate: passed.** Checked against `91_consolidation_nine.md` as the governing consolidated
statement and against the checkpoint that ratified the item (`90b` sections 3 and 5). The dispatch
asks for a second independent read of a principle the ninth consolidation itself lists as owed
(`91:1007-1010`), and the panel's scope boundary is respected: nothing below proposes a change to
`mock/crates`, every probe lives in `94_probes/`, and the two places I read shipped source are both
factual checks of a claim, recorded as such in section 1 and not used as evidence about what the
design means.

**Test gate: run, and it is not clean.** `cargo test --offline --workspace` from a clean tree at
`6519a4e` reports 666 passed, 0 failed, 9 ignored, reproducing `91:43-44` exactly. The 9 ignored
resolve to one honest catalogue-red (`arvo/tests/fixed_point_div.rs:111`, naming its tracked task)
plus doctests. I read the bodies rather than the names in the surface my probes touch, which is the
capacity surface item 14 has been carrying, and it is worse than "tautological" undersells:

```rust
// mock/crates/arvo-tensor/src/capacity.rs:48
const CAP: Cap = cap(N);

// mock/crates/arvo-tensor/tests/capacity.rs:14-18
assert_eq!(<Dim<3> as Capacity>::CAP, cap(3));
```

After one substitution that reads `assert_eq!(cap(3), cap(3))`. It cannot fail for any
implementation of anything, it was first flagged at file 76, it has been carried in every deliverable
since, and it is counted in the 666. It is not a weak test, it is not a test, and it should be
deleted on sight rather than waiting for "the implementation phase's first honest red commit"
(`91:957-958`) to justify the deletion. I do not refuse the assigned work over it, because the
surface is out of the panel's scope and the review has already found and recorded it. I do say
plainly that carrying a known fabricated pass through eighteen files while writing about verification
discipline is not a good look for the register this panel writes in.

## 1. Cheap factual checks on the consolidation, performed before reasoning from it

Four of five reproduce. One is stale.

The 666/0/9 count, the tower-absence grep (`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` exits 1, empty), and the pinned toolchain (`rustc 1.98.0-nightly (57d06900f
2026-05-27)` inside the tree) all reproduce as stated. The tautological test reproduces as above.

**`91:49-53` and `91:1101-1108` are stale.** The bench-harness overwrite defect they declare a
standing outage was fixed at commit `5dae109`, two minutes after consolidation nine landed. That
commit adds 25 lines to `mock/benches/src/main.rs` and nothing else, so it closes the section-filter
half of the closing artifact at `91:1025-1027` and leaves the other half open: the by-reference input
path is not in it, and the bandwidth-contention bench `Cold`'s footprint intent needs is still
unbuildable. This matters below, because that bench is `Cold`'s designated verifier and section 4
turns on whether "designated" means "named" or "exists".

*Grounded on: measured (the four checks, commands in `94_probes/OUTCOMES.md`), tree-fact
(`5dae109`'s diffstat, cited for its existence and extent only).*

## 2. The method: run the principle against the call sites a consumer writes

File 90's sweep (`90:136-183`) is a good sweep and it is conducted from inside the design. It asks,
of each name, whether the claim the name makes is one the compiler checks. That is the right question
if names are observed by the type system. They are not. A name is observed by a person, at a call
site, usually while doing something else, and usually without reading the document that explains what
the name was scoped to mean. File 47 was the first dispatch to write consumer code against this
design from outside, and the thing it found was not that the design was wrong; it was that a consumer
could not write a number at all, because nobody had checked from that chair. The same chair is what
this file brings to the naming question.

So the test I ran is: take the names the design actually has, write the declaration a consumer
writes, and ask what a competent reader who has not read the panel concludes from the token. Three
outcomes are interesting. The reader's conclusion is true (the name works). The reader's conclusion is
false and the principle forbids the name (the principle works). **The reader's conclusion is false
and the principle permits the name** (the finding).

One structural note about this design specifically, which makes the third case common rather than
exotic. The design's own factoring is what file 90 leans on: "the design's own factoring (identity
axes separate from lowering separate from policy) makes the name an alias for checkable content"
(`90:150-151`). That factoring is real and it is good. But it means a single declaration carries four
or five names from four or five different axes, and **the meaning of each depends on the others**. A
reader reads one token at a time. Where a name's content is a function of a sibling parameter, the
name cannot denote to the reader what it denotes to the compiler, because the compiler has the whole
declaration in hand and the reader has a token.

## 3. The enumeration, rather than agreement

`91:850-851` says "Every existing name in the design passes; the sentence forbids only the name
nobody has written yet." The dispatch flagged that as one file's assertion. It is false. Below is
every standards-asserting, algorithm-asserting or behaviour-suggesting name I could enumerate from
the consolidation's own trait table (`91:698-745`), its preset tables, and the members those tables
name. Verdict column is against the refined principle **as written**.

| Name | What it denotes to the compiler | What a reader concludes at a call site | As-written verdict |
|---|---|---|---|
| `IeeeSpecials` | which special values inhabit the value set (`78:637`) | operations on those values behave per IEEE | passes, **diverges** |
| `E4M3` | an identity-axis bundle, primary-sourced twice (`80:292-308`) | the deployed OCP format, including its overflow behaviour | passes, **diverges** |
| `Hot`/`Warm`/`Cold`/`Precise` | a row per number kind, two rows per name | one tradeoff, wherever written | passes, **diverges (compiled)** |
| `StoredWidth::Minimum` | equal to the fields' extent (`91:512-513`) | the smallest memory the value occupies | passes, **diverges** |
| `Folded<N>` | the site count (`91:400-401`) | N roundings happened | passes, **diverges** |
| `IeeeDefault` | the assumed control-state bundle | this computed under IEEE defaults | passes, **diverges** |
| `Fnv1a` | nothing at type level | conformance to published FNV-1a | passes cleanly |
| `quantise` | nothing at type level | IEEE 754's operation of that name | passes cleanly |
| `TotalOrd` | nothing at type level | a total order over the datum set | passes cleanly |
| MATLAB / SystemC aliases | nothing at type level | the namesake's behaviour | passes cleanly |

The four clean passes are all class two, all with verifiers named in the record: test vectors for
`Fnv1a` (named at `90:156`; the tree happens to carry them at `arvo-hash/tests/algo.rs:64-70`, which
is a factual check and not my justification), a model-exhaustive probe plus the parity mandate for
`quantise` (`80:114-165`, `79b`), model-width exhaustion plus a transfer argument for `TotalOrd`
(`78:944-947`), and the parity suites for the namesake aliases (`79b:20-27`). **The class-two half of
the principle is sound and I have no attack on it.** Everything below is about the class-one half.

Three of the six divergences deserve their evidence stated rather than tabulated.

### 3.1 The preset names, compiled

`94_probes/probe_3` transcribes both ratified tables cell for cell (`78:409-421` fixed point,
`78:433-441` float, both ratified at `70b`) and compares each name across the two kinds:

```
Hot      TWO bundles; cells that differ by number kind: ["in-range direction", "out-of-range", "Door"]
Cold     TWO bundles; cells that differ by number kind: ["out-of-range", "Door"]
Warm     TWO bundles; cells that differ by number kind: ["out-of-range", "StoredWidth", "Door"]
Precise  TWO bundles; cells that differ by number kind: ["Door"]
```

Not one of the four names denotes a row. `Hot` rounds toward negative infinity on fixed point and to
nearest even on float. `Warm` stores doubled on fixed point and minimum on float, which `78:441` sets
in bold as the sharpest single finding of its own re-derivation. A consumer who learns the fixed-point
semantics of `Hot` from one declaration and writes `Hot` in a float declaration has learned something
false, and nothing in the declaration tells them the token changed meaning, because what changed it
is the numeral's exponent form, a different parameter, somewhere else in the same type.

The third assertion in that probe is the one that explains why nobody caught this: `Cold` and
`Precise` agree on every cell a spot-check reads (in-range direction, out-of-range), differing only on
`Door`. A reader testing the hypothesis "does a preset name mean one thing" at either of those two
presets confirms it. This is the separation requirement's own shape, arising inside the naming
question, in its first stretch of applicability.

I want to be precise about what this does and does not attack. It does **not** say the tables are
wrong; they are ratified, they were each derived from the preset's stated intent, and the divergences
are consequences rather than accidents. `Warm`'s float row diverges because hardware gives correctly
rounded intermediates away for free and doubling would add the bookkeeping the `Warm` intent forbids
(`78:441-448`). That is good design. It says the **name** carries none of that, and the principle's
class-one clause licenses the name on the grounds that the compiler checks the claim, which it does,
against content the reader cannot see from the token.

### 3.2 `StoredWidth::Minimum`, which misled this review for two files

The strongest evidence that a name misleads is a record of it misleading someone. `StoredWidth`'s
reading was an open fork at `82b`, and closing it took file 83 section 2 plus an independent
confirmation at file 85 section 2.1, resolved by a three-way foreclosure argument from
`Layout::Bitpacked`'s ratified meaning (`91:504-515`). The answer is that `minimum` means equal to the
fields' extent, and the container is a different level that is never declared.

A name whose correct reading requires a three-way foreclosure argument against a different axis's
ratified meaning is not denoting. It is gesturing, and the panel itself read it the other way for two
files. Under the principle as written it passes, because once the fork was closed the compiler does
check the claim.

### 3.3 `IeeeSpecials` and `E4M3`, the same shape one layer up

`Specials` is "the product {NoSpecials, NanOnly, InfOnly, IeeeSpecials}" (`78:637`), and its members
name which special values inhabit the value set. Presence, not semantics. The design derives its own
behaviour on those values and, where derived, agrees with the standard as a theorem rather than a
citation, which is a genuine strength: `x/0` yields IEEE clause 7's two-way split from the far-point
rule plus a limit argument (`91:296-303`). Where it has not derived (NaN propagation through a fold,
comparison, the minimum and maximum operations), the name is promising agreement the design has not
committed to, and the design is explicitly free to diverge, because deriving rather than citing is its
method.

`E4M3` is the same, with a live instance rather than a hypothetical one: the identity-axis bundle is
primary-sourced twice over, and the deployed format's overflow behaviour is a mode split that the
bundle does not carry (`71:288-310`, NVIDIA's default conversion overflows to NaN and saturates only
under `satfinite`). A reader who writes `E4M3` because they are targeting deployed E4M3 silicon has
concluded something about overflow that the name does not carry and the design routed elsewhere.

*Grounded on: ratified (`70b` via `78:409-441`, `78:637`, `90b`), settled shapes (`91:504-515`,
`91:400-401`, `80:292-308`, `71:288-310`, `90:136-183`), compiled (`94_probes/probe_3`, run),
reasoned (the reader-side classification, mine, offered as a suggestion).*

## 4. Does the principle cost a good name? One word decides, and it is not in the sentence

The dispatch asked me to look for the symmetric failure, a principle that forbids a name that would
have been fine. I found the risk and it turns on a single ambiguous word.

The sentence: "A name may promise behaviour only where the design **names** the verifier that checks
the promise; until the verifier exists, the promise is an entry in the trusted base ... A name that
promises behaviour with **no designated** verifier is forbidden" (`91:846-850`).

Read "designated" as "named in the record", and the principle costs nothing. Read it as "existing",
and it eats good names immediately, starting with `Cold`. `Cold` promises a footprint win to every
reader who writes it; that is the whole reason the marker exists and the reason
`arvo-toolbox-not-policer.md` defends it by name. Its verifier is the bandwidth-contention bench, and
the review's own text says that bench "cannot currently be built at all" because the harness input
path caps a flat input below the host's L2 (`91:679-682`), which section 1 above confirms is still
true after `5dae109`. `Hot` is in the same position: it promises speed, and the review has no bench it
is currently willing to run.

So the sentence is ambiguous on exactly the axis that decides whether it is a useful principle or a
rename machine, and the two readings differ on the design's most-written names. **The fix is one
word.** Say "named in the record" and the second clause absorbs `Cold`, `Hot`, and every unbuilt
namesake alias as trusted-base entries with closing artifacts, which is the mechanism the principle
already has and already prices correctly. That is also the reading `90b` clearly intended
(`90b:73-74`), so this is a wording repair rather than a disagreement.

I record it because the `67b` principle died of exactly this failure: it read as absolute, nobody ran
it against the whole name set, and applied as written it forbade an intent pillar op had ratified
(`91:842-843`). The refined principle is one word away from the same trap, and one word is cheap
insurance.

*Grounded on: ratified (`90b` via `91:846-851`), settled shapes (`91:679-682`,
`arvo-toolbox-not-policer.md:38-48`), measured (`5dae109` closes only the section-filter half),
reasoned (the ambiguity, mine).*

## 5. The repair, which the design already owns

I am not proposing a fourth mechanism. The design adopted the exact test this needs three sections
earlier in its own document:

> A claim about a distinction is checked at an instantiation where the distinction is nonvacuous, and
> every model states what it separates. (`91:136-137`, adopted at `86b`)

Applied to names, that is one sentence, and it replaces the class-one escape rather than sitting
beside it:

> **A name's class is decided at an instantiation where its denotation and its behavioural reading
> diverge, not at one where they coincide.** Where no such instantiation exists, the name denotes and
> is free. Where one exists, the name promises behaviour over that region, and the region needs a
> verifier named against it, or the ratifying text must state the boundary inline.

Three properties recommend it. It is decidable by a sweep the review already knows how to run, and
the sweep is the same one file 90 performed, with the observer changed. It costs nothing at the
names that were already fine: `Fnv1a`, `quantise`, `TotalOrd` and the namesake aliases have no
denotation to diverge from, and the pure vocabulary members (`Nat`, `Pos`, `Rad`, `EZero`, `BPos`,
`Dec`) have no behavioural reading a consumer forms at all, so they pass without inspection. And it
composes with the definitional-completeness line (`91:128-133`) rather than duplicating it: the line
says every term in a ratified definition is defined or named open; this says which terms a name is
making a reader think are defined.

What it costs is honest and worth stating. Six names in section 3's table need one sentence each in
the ratifying text, and none of them needs renaming:

- `Specials` members name **inhabitants, not semantics**. One sentence at the `Specials` definition.
- `E4M3` names the **identity axes**; the deployed overflow mode is a separate axis and lives on the
  lowering, per file 71 ground 4.
- A preset name is a **pair of rows keyed by number kind**, and the two tables are both ratified. The
  sentence belongs above both tables, not in either.
- `StoredWidth::Minimum` is the **carrier**, not the container; `91:512-514` already wrote this
  sentence and it needs to sit at the definition rather than in the section that closed the fork.
- `Folded<N>`'s `N` is the **site count**, an upper bound on the moved count; `91:400-401` already
  wrote it, same relocation.
- `IeeeDefault` denotes the **assumed ambient control state**, section 6.

Four of the six sentences already exist in the corpus. What the rule changes is where they live: at
the definition the reader reaches, not in the file that resolved the confusion.

*Grounded on: ratified (`86b` via `91:136-137`, `90b` via `91:128-133`), settled shapes (`91:512-514`,
`91:400-401`, `71:306-310`), reasoned (the rule and the six sentences, mine, offered as suggestions).*

## 6. The environment parameter: the assumption clause survives, scoped, and one artifact comes off the list

Two attacks were named for this second read (`91:1007-1010`, `90:475-479`). Both land somewhere, and
neither kills the sentence.

### 6.1 Does `79b`'s intent-pillar license extend to an environment parameter?

Op wrote `79b` about namesake API aliases, with differential parity suites as what "will ensure that
the APIs do produce the namesake's behavior" (`79b:20-27`, as quoted at `90:54-56`). An environment
parameter is not an API alias. Nobody calls it. It is a type argument that changes what code gets
emitted, and its claim is about the machine the emitted code runs on rather than about what the
emitted code computes.

That difference is not cosmetic, and it shows up in the artifact list. `IeeeDefault`'s residual is
listed against three artifacts at `91:859-863`: the parity suite (behavioural half), the compiled
receipt assertion, and the build layer's three obligations. **The parity suite cannot fail on the
claim it is listed against.** A parity suite runs in a process, and the process it runs in either has
the assumed control state or does not; if it does, the suite passes and reports nothing about any
other deployment, and if it does not, the suite fails for a reason that has nothing to do with the
namesake comparison it was written for. It is not a weak verifier of the deployment claim. It is
structurally incapable of failing on it, which is the same shape as the assertion in section 0 that
this panel has been carrying since file 76, appearing in a design document instead of a test
file, and inflating an artifact count the way that assertion inflates a coverage count.

So the license splits, cleanly, along the line the name itself has:

- **The emitted arithmetic conforms to IEEE 754 given the environment.** Class two, a behaviour
  promise, and the parity suite is exactly its verifier. `79b` covers this in full.
- **The environment holds where the binary runs.** Not a behaviour promise about arvo's code at all.
  Permanently trusted base, receipt-checkable in a debug build, closed only by the build layer.
  `79b` says nothing about it, because op was not writing about it.

The repair is to strike the parity suite from `IeeeDefault`'s artifact list and keep it against the
arithmetic claim, where it does fail when the claim is false. That leaves two artifacts, both of which
can fail, which is a shorter and stronger list. I would rather the design carry two honest artifacts
than three where one is decorative, and I say that in the same register I would say it about a test.

### 6.2 Does the sentence survive the environment type becoming the home of the NaN-on-overflow mode?

File 71 declined NaN-on-overflow for the preset table on four grounds, and its fourth routed the mode
here: "if the review ever wants NaN-on-overflow, it is a `FloatEnv` fact on the door, not a
`Resolution` constant on the preset" (`71:306-310`, carried at `78:325-326`). So the type is
pre-authorised to carry something that is not a control-state bundle.

The sentence as adopted reads "An environment parameter denotes the control state the lowering's
correctness is conditional on. It is an assumption, never a witness" (`91:854-855`). Applied to the
NaN-on-overflow mode it is **wrong**, and wrong in the direction the fourth design rule forbids.

No control register carries that mode. It is a choice between two instruction forms at code
generation (`cvt` against `cvt.satfinite`), settled when the code is emitted, unperturbable by any
linked library, and knowable at compile time by inspection of what was emitted. Modelling it as an
assumption about the deployment would defer to a runtime trusted-base entry a fact that compile time
settles, which is the fourth design rule's own prohibition (`91:113-121`, and the pricing pillar's
standing test that a quantity which is a function of the type's parameters alone belongs on the type).

The sentence survives with one word and gains a second clause:

> An environment parameter denotes the **ambient** control state the lowering's correctness is
> conditional on. It is an assumption, never a witness. A fact the deployment cannot perturb is not
> environment: it is a lowering decision, settled where the code is emitted, and it belongs on the
> lowering beside `Door` rather than inside the environment parameter.

That keeps `IeeeDefault` exactly as adopted, keeps the trusted-base accounting exactly as adopted,
and forecloses the drift file 71 ground 4 would otherwise have licensed by accident. It also gives
the review a test for the next candidate: **can a linked library change it at runtime?** If yes it is
environment and gets a receipt. If no it is a decision and gets a type, checked by the compiler, with
nothing in the trusted base at all.

I note in passing that this is a better outcome for NaN-on-overflow than the routing file 71 gave it.
As a lowering decision it is compile-time visible, refusable at declaration where a `Specials` member
has no NaN, and free of the receipt machinery. As an environment fact it would have been an
unverifiable runtime assumption about something no runtime state carries.

*Grounded on: ratified (`79b:20-27`, `90b` via `91:854-857`, `70b`), settled shapes (`71:306-310`,
`78:325-326`, `91:859-863`, `91:113-121`), reasoned (the license split, the artifact strike, the
ambient scoping, mine, offered as suggestions).*

## 7. The receipt is part of the denotation, and it does not currently check what the name denotes

If `IeeeDefault` denotes an assumed bundle and the receipt is the artifact against it, then **what
the name denotes is exactly the field set the receipt checks.** Any gap between them is a gap in the
name's meaning, not a gap in a test. Two are compiled here.

**The mask omits a field this host latches.** `94_probes/probe_2` writes each candidate FPCR field,
reads back, and asks whether file 90's mask would have caught it:

```
FZ16[19]             latched_by_host=true  in_file_90_mask=false receipt_would_catch=false
...
fpcr with outside-mask bits set: 0x0000000000080000
file 90 receipt says: PASS  (should be FAIL)
```

FZ16 is flush-to-zero for half precision. The design commits to every IEEE interchange row
(`91:520`) and binary16 is among the instantiations it has compiled (`82:76`), so a half-precision
numeral under FZ16 is a numeral without gradual underflow, and the receipt reports the IEEE default
environment over it. FIZ and AH (the
FEAT_AFP input-flush pair, the aarch64 analogue of x86's DAZ) do not latch on this silicon, so that
half of the gap is architectural rather than observable here, and the probe reports it that way
rather than asserting it.

**The x86 form is a different register with a different field set, and is not three instructions.**
`94_probes/probe_1`, the artifact owed at `91:1011-1012`, built for both targets:

| form | target | body instructions | memory round-trips |
|---|---|---|---|
| file 90's shape | aarch64 | 3 | 0 |
| transliterated (RC + FTZ) | x86_64 | 5 | 1 |
| honest MXCSR (RC + FTZ + DAZ) | x86_64 | 5 | 1 |
| plus x87 FCW (RC + PC) | x86_64 | 12 | 2 |

`stmxcsr` has no register-destination form, so the read is a store and a reload. The transliterated
form is the finding rather than the count: it compiles, it looks like a port of the aarch64 receipt,
and it passes with DAZ set, which is a deployment where denormal inputs are flushed. And x87 carries a
precision-control field with no aarch64 counterpart at all, whose value decides whether a binary64
result on that path can double-round, so "read the control register" is not one obligation on this
target.

**So the verifier is not portable, and the answer is not to write a second one by hand.** Both defects
are the same defect: the receipt was hand-written per target, so it can disagree with the bundle the
name denotes and nothing catches the disagreement. The pricing pillar already says where this belongs
(`91:117-121`): a quantity that is a function of the type's parameters alone goes on the type as an
associated const.

`94_probes/probe_4` puts the field set on the environment type as data and derives the receipt as a
fold over it. `94_probes/probe_5` moves the fold to a defaulted associated const, which is the shape
the pillar actually asks for, since a `const fn` in value position folds at the optimiser's
discretion. Both compile with **no feature gates** on the pinned toolchain, and probe 5's
`const _: () = assert!(IeeeDefault::MASK == 0x1C80000)` in const position is the proof the evaluation
happened at compile time rather than at LLVM's convenience. Emitted aarch64 bodies:

| form | instructions | fields |
|---|---|---|
| hand-written, file 90 | 3 (`mrs`, `tst #0x1c00000`, `cset`) | RMode, FZ |
| derived, complete | 4 (`mrs`, `mov #29884416`, `tst`, `cset`) | RMode, FZ, FZ16 |

**Closing the hole costs one instruction**, entirely because the wider mask does not fit an AArch64
logical-immediate encoding. The fold itself costs nothing at either target; no loop survives into the
emitted code. On x86 the derived form emits the same five-instruction body as the honest hand-written
one, so the generic shape is free there too.

Stated as design text for the next consolidation:

> An environment type's denotation is a per-target field set: for each field, the mask that selects
> it and the value the environment declares it holds. The receipt is a fold over that set, computed
> in a const position, and is not written per target. A target that cannot express a member of the
> abstract bundle declares a zero mask for it, which is a claim that the target cannot check the
> field, never a claim that the field is satisfied.

The last clause is the one that keeps this honest, and it is what turns the x87 PC field and the
missing FEAT_AFP bits from silent omissions into declared holes. It also means adding a field to an
environment cannot leave a stale receipt passing, because there is one receipt and it reads the set.

*Grounded on: ratified (`90b`, `91:117-121` the pricing pillar), settled shapes (`63:600-605`,
`90:196-204`, `91:520`, `82:76`), compiled (`94_probes/probe_1`, `probe_2` run, `probe_4`, `probe_5`, all
with emitted assembly recorded), reasoned (the derived shape, mine, offered as a suggestion).*

## 8. What I am proposing, gathered, in the form the next consolidation could take

Five sentences. Two are one-word repairs to text already adopted, two are new, one is a strike.

1. **The class test, keyed on the reader.** A name's class is decided at an instantiation where its
   denotation and its behavioural reading diverge, not at one where they coincide. Where no such
   instantiation exists the name denotes and is free; where one exists the name promises behaviour
   over that region and the region needs a verifier named against it, or the ratifying text states
   the boundary inline. (Section 5. Replaces the class-one clause; the class-two clause is untouched
   and sound.)
2. **One word in the forbidding clause.** "A name that promises behaviour with no verifier **named in
   the record** is forbidden." (Section 4. Wording repair, not a disagreement; it is what `90b`
   intended and what stops the principle repeating `67b`'s death.)
3. **One word plus a clause in the environment sentence.** An environment parameter denotes the
   **ambient** control state the lowering's correctness is conditional on, an assumption and never a
   witness; a fact the deployment cannot perturb is a lowering decision, settled where the code is
   emitted, and belongs on the lowering. (Section 6.2.)
4. **The environment's denotation is a per-target field set and the receipt is a fold over it**,
   computed in a const position, with a zero mask meaning the target cannot check the field rather
   than that the field holds. (Section 7.)
5. **Strike the parity suite from `IeeeDefault`'s artifact list**, keeping it against the arithmetic
   claim where it can fail. Two artifacts that can fail beat three where one cannot. (Section 6.1.)

And six one-sentence boundary statements relocated to the definitions they govern, four of which
already exist in the corpus (section 5's list).

On the question as posed: **a name in this design may promise exactly what something in the design
can be false about.** Denotation is not a defence, it is the special case where nothing can diverge.
Where something can, the name is making a promise whether or not the compiler is watching, and the
design's own separation requirement is the instrument that finds it.

## 9. What I leave open, and what I did not do

- **A third read is owed on section 5's class test**, not on the refined principle, which I am
  confirming rather than disputing on its class-two half. The convention is two independent reads and
  I have introduced new text; it should get the same treatment. The attack surface is whether "an
  instantiation where denotation and behavioural reading diverge" is decidable without a house style
  for what a reader concludes, which is the softest joint in it.
- **The FEAT_AFP half of probe 2 is unmeasured**, because this silicon does not latch FIZ or AH. A
  member on Armv8.7 hardware, or a Foundation-model run, would close it. It does not change the
  verdict, since FZ16 alone establishes the hole.
- **I did not price the derived receipt on a third target.** aarch64 and x86_64 are the two the
  dispatch named; RISC-V's `fcsr` is a third register shape with its own field set and would be a
  useful third data point for the zero-mask clause specifically.
- **I did not touch `Hot`'s default environment.** It was reserved to op at `86b` and reaffirmed at
  `90b`, nothing above forces it, and section 6.2 does not bear on it beyond making the choice's
  content narrower and more honest.
- **The `Cold` footprint bench remains the load-bearing missing measurement**, now for a second
  reason: it is `Cold`'s designated verifier under section 4's reading, and `5dae109` closed only the
  section-filter half of its blocker. The by-reference input path is still owed.
- **I did not audit the shipped tree**, per the method constraint. The two places I read source
  (`arvo-tensor/capacity.rs:48` with its test, and `arvo-hash/tests/algo.rs:64-70`) are both factual
  checks of a claim the record already makes, and every judgement above survives deleting them.
