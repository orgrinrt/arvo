# 103. The platform crate and the predicate: one is a naming door with a token collision, the other is ratified and overtaken by the half it kept as a courtesy, and the fork they share is free at runtime and costs fifteen declarations

Daan Leijen, file 103. I wrote file 05 (fallibility without poisoning) and file 84 (the failure that
turned out to be an ordinary range event on a numeral nobody was looking at). This dispatch sends me
back to two of the three periphery rows file 101 left content-unread, with the instruction to say
whether they survive what the panel has settled since, and to price the fork they share so the lead
designer chooses between two costed things rather than two names.

Both survive. Neither is what its row description says it is. The fork is cheaper than either branch's
advocates think and is decided by a question nobody has asked, whose answer is already in the tree.

## What I read

The op-ratified round `mock/design_rounds/202607300800/` at the decisions this dispatch names, from
the topic files rather than from the consolidation, because a compression is not a decision:
`202607290100_topic.one-predicate-concept-implemented-twice.md` in full (139 lines),
`202607290200_topic.the-predicate-decisions.md` in full (D15, D16, D17, D18),
`202607290500_topic.the-placement-calls.md` in full (D18b, D19, D21),
`202607291700_topic.storage-decomposes-and-refit-comes-home.md` in full (D27, D28), and
`202607300700_topic.consolidated-round-state.md` at D5 (`:542-556`), D6 (`:560-568`), D7, D9
(`:588-600`), and the D27 restatement (`:1215-1240`). The round's own cited sketch
`mock/research/sketches/202607282230_hlist-arity-dissolution/FINDINGS.md` at Shape F
(`:199-260`, `:323-380`).

From the panel: `102_consolidation_ten.md` in full as the standing base, `101_knuth_the_periphery_re_audited.md`
in full, `101b_persona_checkpoint_twentyfour.md` in full. Behind them, only where they carry a
derivation I needed: `07_spj_is_the_type_story_sound.md:310-345` (the three-rung ladder and the
one-blanket-one-name principle), `03_jhala_what_is_provable.md:50-94` (D16's demolition),
`74_lattner_the_taxonomy_rechecked.md:61-73` and `:247-253` (the two rows and the fork's first
statement), my own `84:386-393` and `:508-518`. One `ls` of the panel directory, current through
`102`. One `ls` of `mock/design_rounds/` root, which is how file 98's error happened, and I opened
what it returned.

From the shipped tree, for the two licensed purposes only, evidence about why the redesign is
happening and checking a factual claim before reasoning from it:
`arvo-storage/src/platform.rs` (550 lines, in full), `arvo-mask-contracts/src/lib.rs:45-66`,
`arvo-comb/src/greedy.rs:25-70`, `arvo-comb/tests/greedy.rs` in full,
`arvo-storage/tests/bool_consttry.rs` in full, and `mock/PRINCIPLES.md.tmpl:322-326`. Every judgement
below survives deleting its shipped-source citation; the citations say why a sentence is needed, never
what the design means. Where a citation is doing more than that I say so and withdraw the claim.

## Gates, run before the work

**Canon gate.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD `d6a4d5c`, run 2026-08-05 06:25. The
governing material is the op-ratified round `202607300800`, which sits on the governing rung and
outranks every panel file including this one; `102` is the panel's standing base beneath it. Gate
passed. One framing note rather than a refusal: the dispatch calls the platform crate "where the
design bottoms out against facts it does not control". Section 1.2 shows that is false of four of its
five items, and I answer the question the material poses instead.

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary: **155 binaries,
672 passed, 0 failed, 9 ignored**, matching `102`'s own count exactly, from a clean tree.

Test bodies read rather than counted, in the surfaces this file touches:

- `arvo-comb/tests/greedy.rs`, five tests, all real: real assertions on real boundary shapes (empty
  input, single group, three-way split, tight cap, cap at M), no tautology, no assertion of a value
  against itself. **But every one of the five supplies the same predicate,
  `|acc, x| Bool(acc.total + x <= acc.cap)`, against items that always satisfy it at a fresh
  accumulator, where `acc.total == 0`.** The guard at `arvo-comb/src/greedy.rs:60-64`, the one path
  D15's property exists to delete, is entered by none of them. That is setup that helps in the test
  gate's own sense: the assertions are real and the breaking path is never reached. It does not
  disqualify the suite, which is honest about what it covers, and it does mean **the single property
  D15 rests its case on has no test in either direction**, which is owed before any property is
  allowed to delete a branch.
- `arvo-storage/tests/bool_consttry.rs`, five const-position pins plus one runtime test, all real,
  with the `Break` arms asserted unreachable by `panic!` rather than by omission. One finding it
  obliges me to record, section 1.5.
- `arvo/tests/predicate_family_const_probe.rs`, read by file 101 and re-read here: real, thirteen
  const pins, negatives present. Nothing to add to `101:44-50`.
- The three known tautologies (`arvo-tensor/tests/capacity.rs:14-18`,
  `arvo-tensor/tests/const_capacity.rs:49-53`, `arvo-hash/tests/aliases.rs:16-23`) are still present
  at source, re-verified this session, still in the green total, still disposed at `95b` as op's own
  trivial commit outside the panel.

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside the
tree. Probes in `103_probes/` with `OUTCOMES.md`; compiled versus reasoned is tagged per claim.

---

## 0. The answer, first

**The platform decision, D27: ratified, and the panel's provable-versus-trusted machinery survives it
easily, because the crate is not the hard case the periphery table implies.** Four of its five items
are compile-time-settled facts that add no trusted-base entry at all, verified by const assertion;
the fifth is one sentence the seal chapter has already written. What the crate does have is a
different problem, unnamed anywhere in one hundred and two files: it is the design's single naming
door for the host's primitives, and the thing it is made of today has six doors. And its name collides
with a token arvo's own ratified principles define to mean something else.

**The predicate decision, D15: ratified, and overtaken in the most interesting way this review keeps
finding.** The half D15 kept as a courtesy, the degenerate one-body instance, is the half the panel
has since made load-bearing: it is the vehicle by which a kind-2 failure is refused at declaration
(my own `84:386-393`). The half D15 built the decision around, the arity hlist with `Deref` call
syntax, has no panel consumer at all, and the panel's own pass over the crates that hold its two
callers never mentions a predicate. D16 is overtaken and preserved: it is not a rung, it is the risk
annotation on file 07's rung 2, and D15's one named property sits there by compiler refusal rather
than by argument. D17 is ratified and orphan-legal, compiled.

**The fork they share costs zero at runtime and fifteen declarations at compile time**, and the
question that decides it is whether a second truth type exists. One does, shipped, at
`arvo-mask-contracts/src/lib.rs:45-66`.

---

## 1. The platform decision

### 1.1 What D27 says, restated in the design's current vocabulary

D27 (op, 2026-07-29, `202607291700_topic.storage-decomposes-and-refit-comes-home.md:8-39`) is a
decomposition, not a placement: `arvo-storage` accumulated three unrelated families under a name that
describes one of them, so each family goes to its obvious home, the remnant keeps the crate, and the
crate is renamed to fit what is left. The platform row is one line of a seven-row table
(`:22`): `USize`, `Bool`, `BoolResidual`, `AsBool`, `NUSize` go to a new crate `arvo-platform`,
because, in op's own reason (`:34-36`), "a platform primitive has to be named once and wrapped, and
that is a different job from holding a bit container."

In the vocabulary the design has since built, that sentence is the layer-keying rule one level below
where the rule usually fires. The layer-keying rule asks what a fact is keyed on; D27 asks what a
*name* is keyed on, and answers that a name over a host primitive is keyed on the host, not on the
container the primitive happens to carry. That is correct and it is not the interesting part of the
sentence. The interesting part is "**once**". D27's whole content is that the design has exactly one
place where a host primitive acquires an arvo name, and that place is a crate.

**So the crate's charter, in one sentence the ratifying text can take verbatim:** *the platform crate
is the design's single naming door for the host's primitive types; it names each one once, exposes no
route to the primitive except the ones it declares, and contains no fact that is not settled where
the code is emitted.* Everything in section 1 below is a consequence of that sentence, and the third
clause is the one the design has since acquired the machinery to check.

### 1.2 The provable-versus-trusted machinery applied, and the crate is not its hardest case

The dispatch frames this crate as "where the design bottoms out against facts it does not control: a
pointer width, a boolean, whatever the host actually provides", and asks whether the machinery
survives its hardest case. The framing is the periphery table's, and it is false at four of the five
items, which is worth establishing before anything is built on it.

`102:748-756` ratified the test, in the environment chapter: an environment parameter denotes the
**ambient control state** a lowering's correctness is conditional on, an assumption and never a
witness; **a fact the deployment cannot perturb is not environment, it is a lowering decision, settled
where the code is emitted**, and file 97's second read sharpened the operative question to *can a
linked library change the fact at runtime*.

Applied item by item, and compiled (`103_probes/p6_platform_facts.rs`, WORKS):

| Item | The fact it rests on | Can a linked library change it | Verdict |
|---|---|---|---|
| `USize` | `usize::BITS`, a target cfg | no | settled at emission, const-assertable |
| `Bool` | `bool` is one byte, align one, validity `{0,1}` | no | settled at emission, const-assertable |
| `BoolResidual` | uninhabitedness of an empty enum | no | settled at emission, a language fact |
| `AsBool` | nothing; it is a projection | n/a | not a fact |
| `NUSize` | `NonZero`'s excluded pattern and its discriminant elision | no | **one trusted-base entry**, already written |

Every row's assertion compiles: pointer width against the four legal values, `size_of::<bool>() == 1`,
`align_of::<bool>() == 1`, `size_of::<Option<bool>>() == 1`, and
`size_of::<Option<NonZeroUsize>>() == size_of::<usize>()`. None needed a runtime check. By `102`'s own
sentence, a compile-time-settled fact "adds no **new** trusted-base entry, riding on the standing
toolchain trust every compiled claim in this review already carries", so **four of the five items
enter no trusted base at all.**

The fifth does, and it is not new either. `NUSize`'s width claim is the same claim `102:262-266`
already accounts for at one sentence: the excluded pattern's unreachability in safe code per the
member's own documented contract in `core`, with the width fact moving to a construction-door const
assertion in the `Maybe`-shaped vocabulary `notko` already ships. The platform crate inherits that
entry verbatim; it does not open a second one.

**So the answer to the question as posed is that the machinery survives, and the reason is that the
case is not hard.** The design's genuinely unprovable host facts are the environment chapter's:
rounding mode, flush-to-zero, the ambient control word, all of which a linked library can change under
you at runtime. The platform crate holds none of them. A pointer width is not that kind of fact and
never was, and calling it one blurs the provable-versus-trusted line from the direction file 97 warned
about: treating *settled elsewhere* and *unverifiable* as the same thing.

**One clause the chapter owes, and it is the cannot-check-versus-cannot-provide split one layer down.**
A target whose `usize` is too narrow for the capacity model is a statically known falsehood, and by
`102:771-779` it refuses at declaration rather than entering a trusted base. The refusal shape
compiles and fires: with the threshold forced,
`error[E0080]: evaluation panicked: arvo's capacity model needs at least a 32-bit index domain`, the
same `E0080` the capacity repair produces. **Owed:** the refusal firing on a real small target. No
16-bit target is installed on this host (`E0463: can't find crate for core` on both `msp430-none-elf`
and `thumbv6m-none-eabi`), so that half is named rather than assumed, and the artifact is one
cross-compile.

### 1.3 What the crate actually needs, and does not have: the naming door has six doors

Here is the finding, and it is the one that changes what gets built.

D27's own reason for the crate is that a host primitive "has to be named **once** and wrapped". The
thing being moved does not satisfy that sentence. `Bool` reaches its `bool` through five independent
public routes and a sixth in a different vocabulary: a public field (`arvo-storage/src/platform.rs:261`),
`Transparent::raw` (`:264`), `Deref<Target = bool>` (`:275`), `From<Bool> for bool` (`:342`),
`AsBool::as_bool` (`:328`), and `Try::branch` returning `bool` (`:293`). `USize` and `Cap` carry public
fields too (`:45`, `:73`). Only `NUSize` (`:485`) is closed.

That is why-evidence, and it survives deleting the citation, which is the test this dispatch sets. The
design sentence is: **a wrapper with six exits is not a wrapper, it is a suggestion**, and the
perimeter rule the review adopted at file 10 says why in general terms: a guarantee about a type holds
only over the operations through which the type can be observed, and every public field is a hole in
the guarantee rather than a stylistic matter. Moving these five items into a crate named for the job
of naming a primitive once, while leaving six ways to unname it, relocates the defect and christens
it.

**What the chapter should say, and it is one sentence plus a table:** *the naming door declares exactly
one route from each named type to the primitive it names, and that route is stated in the type's own
definition; every other route is absent, not discouraged.* For `Bool` the route the design has already
chosen is `Try`, because the shipped doc comment says so in its own words
(`arvo-storage/src/platform.rs:319-321`: "Preferred path in WU code is `?`"), and because `?` is the
only exit that is also a control-flow construct. `Deref`, the public field, `From`, and `AsBool` are
four spellings of one thing, and by the design's own two-organs-for-one-fact rule, which `102` records
having paid for three times, they are three too many.

I am not proposing which of the four survives; that is a consumer-ergonomics call and the toolbox rule
puts it with op. What I am proposing is that the number is one, and that the chapter states which.

### 1.4 The price of closing it, and the price of the crate boundary itself, measured

Two numbers, because a perimeter argument that does not carry its cost is a preference.

**Closing the perimeter is free if the door is inlined, and costs a call per construction if it is
not.** Measured across a real crate boundary built for this file (`103_probes/p2_codegen.rs`, three
crates, `-C opt-level=3`, no LTO): with the wrapper's constructor `#[inline(always)]`, the model loop
is 22 instructions with no calls. Without it, the same loop is 34 instructions and carries
`bl __RNv...p1_arvoNtB2_4Bool3new` **once per element**.

That is the price of D27's new crate, stated plainly: **a crate boundary in Rust is a call unless the
constructor says otherwise**, and the public field is free today precisely because a tuple-struct
literal is not a call. So the chapter owes a second sentence: *every construction and extraction door
on a named primitive is `#[inline(always)]`, and the reason is the crate boundary rather than taste.*
One attribute, and it must be written down, because the failure mode is silent: the code is correct,
the numbers are two thirds of what they should be, and nothing reports it.

The workspace's own always-optimal-internals rule already licenses this as Kind-1 structural lowering
rather than a Kind-2 bench question, so it is a sentence in the chapter, not a bench obligation.

### 1.5 Two definitional-completeness items, one of which is the crate's name

The widened definitional-completeness line (`102:97-108`, `:714-722`) fires at three mechanical
addresses. Two fire here.

**The name.** Address three: a name sharing a token with an external standard whose scope exceeds the
design's own claim. The external standard here is arvo's own ratified principles document.
`mock/PRINCIPLES.md.tmpl:322-326` defines the token: "**Zero platform dep.** arvo has no opinion about
syscalls, threads, clocks, or filesystems. Everything is pure compute on const-sized containers.
**Platform concerns live in consumer crates.**" The generated lint rule carries the same sentence
("arvo has zero platform dep; syscalls belong in consumer crates"). D27 then names a crate
`arvo-platform` whose contents are five wrappers over language-level primitives and zero syscalls.

Nothing is violated. The crate has no platform dependency in the principle's sense, and I checked
before writing this. What has happened is smaller and harder to see: **one token now means "syscalls,
which live elsewhere" in one ratified document and "host primitive names, which live here" in another,
and a reader holding either one concludes something false about the other.** That is the completeness
line's own mechanical form, and its own remedy applies: the boundary sentence relocates to the
definition the reader reaches, or the name changes.

D27's own text names the job precisely enough to rename from: the crate is where a host primitive is
"named once and wrapped". Candidates that carry that and do not collide, offered as suggestions and
not as a call, since a crate name is op's: `arvo-primitives`, `arvo-host`, `arvo-named`. If the name
stays, the chapter owes one sentence stating that "platform" here means the host's primitive types and
not the platform the zero-platform-dep principle refuses, next to both.

**`Bool`'s residual.** Address one: a name appearing in two ratified places with different content.
`arvo-storage/src/platform.rs:285-291` declares `BoolResidual` and its doc comment states that bare
`core::convert::Infallible` **cannot** carry the impl, because the orphan rule forbids implementing
`core`'s `Residual` for a foreign type. `arvo-storage/tests/bool_consttry.rs:47-50` then names
`<Bool as ConstFromResidual<Infallible>>::from_residual`, using `Infallible` as `Bool`'s residual.

Both are correct: they are two `Try` vocabularies, `core`'s and notko's const bridge, and the orphan
argument applies to one and not the other. The duplication is forced by const-callability rather than
chosen, so this is not the two-organs defect. It is a completeness item: **"`Bool`'s residual" is a
function of which `Try` vocabulary is in scope, and neither declaration says so.** One sentence at the
`BoolResidual` declaration closes it.

### 1.6 The platform decision's verdict

**Ratified, and never read by the panel, exactly as the periphery table says.** Restated in current
vocabulary at section 1.1. Not overtaken by anything: the provable-versus-trusted machinery, the
naming principle, and the layer-keying rule all apply to it cleanly and none of them moves it. What
the content review yields is what the two previous content reviews yielded, corrections to stated
grounds rather than a reopened decision: the crate is not the design's hardest trusted-base case and
should stop being described as one, its perimeter has six doors where its own charter says one, its
crate boundary is a call unless inlined, and its name collides with a token the principles document
defines otherwise.

---

## 2. The predicate decision

### 2.1 What D15, D16 and D17 say

**D15** (op, 2026-07-29, `202607290200:10-29`). The question was posed as open between four shapes and
it is not: the sketches settled it, and Family 1 is dead regardless. The unified predicate is the
typestate predicate, carrying two things: its argument list as a type-level hlist, with `Deref`
supplying ordinary `f(a, b)` call syntax at any arity and no feature gates (Shape F, chosen over the
arity-free Shape G because both existing callers perform a joint atomic test rather than a chain); and
its properties as associated consts, "which is what the closure could never carry and the whole reason
a typed predicate earns its existence". `arvo/src/predicate.rs` is deleted with its two callers
rewritten. `IsZeroOf` and its four siblings survive **as the degenerate instance of the same concept,
where the predicate has exactly one body and the property is its name**.

**D16** (op, `:31-42`). A derived property is computed by the type from itself, cannot lie, and is a
plain safe associated const. An asserted property is a promise whose falsehood selects a different
algorithm and returns a wrong answer rather than a slow one, so asserting one is an `unsafe impl`
carrying a stated contract, because from a call site the two mechanisms are indistinguishable.

**D17** (op, `:44-57`). notko is the home. The return-type blocker dissolves the way the count
question did: notko declares the **contract for a truth value**, exactly as `Cardinal` is the contract
a count type implements per D6, and arvo's `Bool` implements it. The predicate's output names that
contract rather than a concrete type, so nothing in the signature needs bare `bool` and nothing needs
`Bool` to be reachable from notko. The truth contract's name is explicitly not settled.

### 2.2 D16: overtaken, preserved, and it was never a rung

D16 is the row file 101 already marked overtaken by op-checkpointed panel work (`101:165`), and the
overtake is worth stating precisely rather than by citation, because what replaced it does not look
like what it replaced.

File 03 demolished the dichotomy (`03:56-94`): read literally, "derived" means computed from something
already established, and nothing in this design derives anything in that sense. What the machinery
actually delivers is totality and coherence, not truth. File 07 rebuilt it as three rungs
(`07:330-345`): **computed and witnessed** (a const check refuses disagreement at instantiation),
**declared, total, coherent** (human-typed, every constructor forced to answer, no witness possible
because the domain is not bounded or not decidable), and **promised** (claims about emitted code with
no type-level referent, `unsafe impl`-shaped, discharged by a bench artifact).

The reading that makes D16 and the ladder one thing rather than two: **D16 is not a rung and never
was. It is the risk annotation that sits on rung 2**, and it answers a question the ladder does not
ask. The ladder sorts by *what checks the fact*. D16 sorts by *what a false fact costs*. Those are
orthogonal, both are needed, and collapsing them is what made the dichotomy read as a dichotomy.

So, adopted as a working shape, one paragraph the ratifying text can take:

> A property carried on a type sits on one of three rungs by what checks it, and carries one of two
> risk classes by what its falsehood costs. Rung 1 is refusable by the const evaluator and is the
> only rung where "cannot lie" is true. Rung 2 is declared: the compiler forces every constructor to
> answer and forbids contradiction, and nothing checks the answer. Rung 3 is promised, discharged by
> an artifact rather than by the type system. On rung 2 and rung 3, a property whose falsehood
> changes an answer rather than a cost is asserted through an `unsafe impl` with a stated contract,
> per D16; a property whose falsehood costs only speed is a safe declaration.

### 2.3 Where D15's one property lands, settled by the compiler rather than by argument

D15's second half says properties as associated consts, and the round names exactly one property with
a consequence in shipped source: the fresh-accumulator guarantee, prose at `arvo-comb/src/greedy.rs:32-34`
and guard at `:60-64`. The round's own framing of the finding is precise and I am not improving on it:
"a required property, stated as prose in a doc comment, with no way for a caller to say it holds and
no way for the library to act on it" (`202607290100:87-88`).

Three things are now settled that were not when D15 locked, and all three are compiler output rather
than reasoning.

**It cannot reach rung 1.** The witness would have to decide, at const-eval time, that a consumer's
closure accepts every item against a fresh accumulator. The attempt does not compile:
`error[E0277]: the trait bound F: [const] Fn(&u32, &u32) is not satisfied`
(`103_probes/p5_witness_attempt.rs`). A consumer closure is not const-callable, so the evaluator
cannot invoke the predicate once, never mind exhaustively. **The witness has no expressible form**,
which is a stronger statement than "the domain is too large", and it settles the rung by refusal
rather than by argument.

**It reaches rung 2 only if the const has no default.** With a default,
`impl Defaulted for Careless {}` compiles and has silently promised; without one,
`impl Total for Silent {}` is `error[E0046]: not all trait items implemented, missing:
FRESH_ALWAYS_ACCEPTS` (`103_probes/p7_totality.rs`). This is file 07's own `a6` finding, that an
overridable default member is not a load-bearing site, arriving at the property's address. **So the
chapter's sentence is: a property const carries no default. Silence is not an answer.**

**Its falsehood changes an answer, so by D16 it is an `unsafe impl`.** A predicate that wrongly
promises the guarantee causes an item the algorithm should have skipped to open a group it cannot
close, which is a different grouping, not a slower one. D16's own test, unamended.

**And it buys two instructions.** Measured (`103_probes/p4_property.rs`, `-C opt-level=3`):
`group_promising` is 27 instructions with 7 branch-class operations; `group_silent` is 29 with 8. The
promise deletes one branch and two instructions, at group-open rather than per item, because the
predicate call the round's topic also counts was already inlined and folded against a trivial closure.

That number is small and it is not the argument, which is the point of measuring it. The round's own
topic says the value is not codegen (`202607290100:55-57`: "the value is that the property **cannot
desync** from the thing it describes"), and it is right. I record the two instructions so that a later
reader cannot re-derive the mechanism as a speed argument, which is the shape this review has watched
a 4.6x figure take once already.

### 2.4 The half D15 kept as a courtesy is the half the panel made load-bearing

This is the overtake, and it runs opposite to the direction D15's own text points.

D15 spends its length on the arity machinery and disposes of Family 2 in one clause: `IsZeroOf` and
its four siblings "survive as the degenerate instance of the same concept". The predicate topic is
harsher still about them, and correctly so at the time: the markers gate nothing, no algorithm anywhere
bounds on one, and two of the five have identical bodies kept distinct so a consumer can name a
semantic that nothing reads (`202607290100:32-46`).

Since then, in my own file 84, that degenerate instance acquired a job it did not have: **it is the
mechanism by which a kind-2 failure is refused at the declaration.** `84:386-393`, adopted through the
consolidations: a failure whose admissible domain is expressible as a predicate on the operand is
refused where the fact enters, and "the design already ships that vocabulary twice over (notko's
`NonZeroable`, and the `IsZero`/`IsNonZero`/`IsPositive`/`IsNonNegative` family in
`arvo-numeric-contracts`)". The division chapter then reused it: `102:368-370` sends a divisor's
domain to exactly this vocabulary ("refused at declaration where the divisor's domain is a
predicate"), and the elementary functions chapter reused it a third time for `sqrt` over a signed
domain (`102:468-469`, "the divisor-domain-as-predicate clause reused, not a new niche").

So the marker family that "gates nothing" now gates the design's entire kind-2 failure story, at three
call sites in the ratified text. **The predicate concept's degenerate instance is not degenerate; it
is the load-bearing one, and D15 kept it for the wrong reason and was right anyway.**

The reverse also holds, and it is the finding's other half. **The arity machinery has no consumer in
the panel's record at all.** Fresh searches, run 2026-08-05 06:13 over the panel directory:
`grep -rln "Pred2\|Pred3" .` returns nothing; `grep -rln "typestate predicate" .` returns two files,
`101` (this stretch's own audit) and `11` (the current-shape draft's inherited table); and file 55,
whose whole subject is typing the algorithm crates that hold Family 1's only two callers, contains the
string "predicate" zero times. The mechanism is real, the sketch is sound, and one hundred and two
files of design work have never needed it.

**What follows for the chapter, and it is a sequencing statement rather than a reversal of D15.**
The predicate concept has two halves with different urgency and different consumers. The property half
and the degenerate instance are on the critical path: they carry the kind-2 refusal, they are cited by
three ratified sections, and the rung-and-risk statement of section 2.2 is what they need. The arity
half serves two call sites in one algorithm crate, has no other consumer, and is fully specified by a
sketch that compiles. Both should be built, since D15 decided both; the chapter should say plainly
which one the rest of the design leans on, because a reader of D15 alone would guess the other one.

### 2.5 D17: the placement is orphan-legal, and the reason is not the one the analogy suggests

D17 grounds itself on `Cardinal`: notko declares the truth contract "exactly as `Cardinal` is the
contract a count type implements per D6". The analogy is right about the shape and wrong about the
obstacle, and the difference is what makes D17 work.

D5 (`round:542-556`) records why `Cardinal` had to be where it is, and it is an orphan argument: "a
separate counting crate in arvo is blocked by the orphan rule (if notko declares both trait and list
types, an impl in arvo has two foreign items and cannot be written; only notko can implement a notko
trait for notko types)". `Cardinal` sits beside `Cons` because the *implementing type* is also
notko's.

The truth contract has no such problem. `Bool` is arvo's. One foreign trait, one local type, which is
the orphan rule's ordinary permitted case. Compiled to be sure rather than reasoned
(`103_probes/p1_foundation.rs` + `p1_arvo.rs`, three real crates, WORKS): the contract declares in the
zero-dependency crate, `Bool` implements it in the crate above, and a third crate consumes both.
**D17 is placeable, and it is placeable for a reason D5's precedent does not supply.**

Two smaller items the placement raises.

**`Cardinal`'s own crate moved, and one sentence in the round did not follow it.** D18b (op,
`202607290500:18-29`) corrects a summary that had put `Cardinal`, `Length`, `Cons` and `Empty` in
notko: "They are **`notko-hlist`'s**", with notko itself taking the predicates. D19, thirteen lines
later in the same file (`:39`), still says "the reasoning D6 used to keep `Cardinal` in notko". Both
readings are recoverable (D6 argues for the notko family, D18b assigns the crate after D5/D9 extracted
the list), and the mechanism is unambiguous because D5's orphan argument forces `Cardinal` to sit with
`Cons`. It is a completeness item rather than a conflict, and it matters here only because D17's
analogy points at a referent that changed crates between the two decisions. One sentence.

**The predicate spans two crates under D18b, and the chapter should say so.** D15's wrapper carries a
type-level hlist, which is `notko-hlist`'s; D18b puts the predicate in `notko`. So `notko` depends on
`notko-hlist`, or the wrapper moves. Compiled here as one crate because the coherence question under
test is against the foundation and splitting the list into a fourth crate does not change which items
are foreign to arvo, so I have **not** verified the notko-to-notko-hlist edge and do not claim it.
**Artifact owed:** one compile with the list in its own crate.

### 2.6 The predicate decision's verdict

**D15: ratified, content-unread, and overtaken in its emphasis rather than in its content.** Nothing
in it is wrong. The half it presents as a courtesy is the half three ratified sections now rest on, and
the half it argues for has no consumer anywhere in the review. Its one named property is placed on
file 07's ladder by compiler refusal rather than by argument, and it needs two sentences D15 could not
have written: no default on the const, and `unsafe impl` per D16 because falsehood changes an answer.

**D16: ratified and overtaken, preserved whole.** Not a rung; the risk annotation on rung 2, orthogonal
to the ladder and still needed.

**D17: ratified, content-unread, and confirmed placeable.** Orphan-legal, compiled. Its remaining
content is the fork, below.

---

## 3. The fork, priced

### 3.1 What the fork actually is

File 74 stated it first (`74:67`, `74:247-253`) and every later file has carried the statement
compressed. In the design's current vocabulary:

The tower's contracts emit derived booleans and membership predicates. `arvo-bridge-home-rule.md`'s
ratified test says a trait lives in the lowest layer where its **return type** is reachable. If those
contracts return `Bool` concretely, then `Bool`, and therefore the platform crate D27 creates, sits
**below** the numeral contracts, inside the tower's dependency cone. That is **branch A**.

D17 offers the other spelling: notko declares the truth contract, the tower's contracts are generic
over it, and `Bool` is a peer that merely implements it. That is **branch B**, and it keeps the
platform crate out of the cone entirely.

The fork is therefore not about `Bool` and not about notko. **It is about whether the numeral contracts
name a concrete truth type or a contract**, and everything else is a consequence.

### 3.2 Branch A, priced

- **Compile surface:** zero. No new type parameter, no new bound, no signature changes anywhere.
- **Dependency:** `arvo-platform` enters the dependency cone of every contracts crate in the tower.
  The zero-platform-dep principle is not violated (section 1.5), and a crate named for the platform
  becoming a prerequisite of the numeric contracts reads wrong even where it costs nothing, which is
  the token collision doing damage rather than the dependency.
- **Runtime:** zero.
- **Consumer:** writes `Bool`, branches through whichever door survives section 1.3.
- **The cost that is easy to miss:** the crate boundary is a call per construction unless every door
  is `#[inline]`. Measured at 34 instructions against 22, with one `bl` per element in the model loop
  (`103_probes/p2_codegen.rs`). This cost belongs to D27 rather than to the fork, and branch A pays it
  at more sites because more crates cross the boundary.

### 3.3 Branch B, priced

- **Compile surface, counted in the shipped tree** (`mock/crates/*/src`, fresh count 2026-08-05 06:23):
  **92 signatures return `Bool`, of which 15 are trait declarations.** By crate: `arvo` 28,
  `arvo-storage` 26, `arvo-bits-contracts` 20, `arvo-numeric-contracts` 12, `arvo-bitmask` 5,
  `arvo-mask-contracts` 1; declarations `arvo-bits-contracts` 8, `arvo-numeric-contracts` 4,
  `arvo-storage` 2, `arvo-mask-contracts` 1. **Branch B adds one type parameter and one bound to those
  15 declarations; the other 77 sites follow their declaration and change nothing but a name.** Test
  files add another 27 sites, not counted above.
- **Bound restatement through a call chain:** one extra per level, because Rust has no implied bounds.
  Counted in probe 1's three-level chain: branch A restates `F: Fn(..) -> Bool`, branch B restates
  `F: Fn(..) -> B` plus `B: Truth`. File 07's own cure applies and reduces it to nothing at consumer
  depth: derive in one blanket, consume through one name (`07:310-320`).
- **Invocation:** free. D15's Shape F carries the arity bound at `Pred::new` only, so no call site in
  either branch restates it (compiled).
- **Dependency:** `arvo-platform` leaves the tower's cone. Orphan-legal (compiled, section 2.5).
- **Runtime: zero, and this is the sharpest number in the file.** Compiled across three real crates at
  `-C opt-level=3`, the assembler emitted **`_run_b1 = _run_a`**: the generic and the concrete
  spellings lowered to byte-identical code and LLVM merged them into one symbol. Not "within noise",
  not "same instruction count". The same function.

### 3.4 What the fork is actually a fork about, and the answer is in the tree

Both branches are cheap. That means the fork is not decided by cost, and the cost is what everyone has
been weighing. The question that decides it is one nobody has asked:

**Is there a second truth type?**

If `Bool` is the only truth type arvo will ever have, branch B's fifteen type parameters buy genericity
nobody instantiates, and branch A is the honest cheaper answer with a naming problem attached. If
there is a second, branch B's parameters are load-bearing and branch A forces the second type to grow
a parallel vocabulary.

There is a second, and it is shipped. `arvo-mask-contracts/src/lib.rs:45-66` declares `MaskOps` with
`empty`, `full`, `union`, `intersection`, `complement`, and `test(idx) -> Bool`. Those five are
`FALSE`, `TRUE`, `or`, `and`, `not` under a set-theoretic vocabulary, and `arvo-bitmask/src/ops.rs`
carries `BitAnd`, `BitOr` and `Not` impls for `Mask<W>` besides. **The design already ships two Boolean
algebras with disjoint vocabularies: `Bool`, the one-lane one, and `MaskOps`, the W-lane one.** That is
the fragmentation the design exists to prevent, at a place nobody has looked, and it is the same shape
as the finding that produced the predicate topic in the first place: one concept, implemented twice, at
the wrong level in both places.

Compiled, to check that the unification is real rather than a pun (`103_probes/p8_second_truth.rs`):
one predicate declaration, `all_hold<F, B: Truth>`, monomorphises to both `Bool` and a 64-lane mask
modelled on `MaskOps`, no gates, both paths monomorphised, no `dyn`. The instruction counts in the
probe are **not** offered as a comparison; the two functions compute different things and neither loop
is the shape a real lane-wise predicate would have. What the probe establishes is only that one
declaration serves two truth types.

**So my read, offered as evidence and suggestion and not as a ruling: branch B, and the reason is not
the dependency cone.** The reason is that a truth contract with two implementors lets one predicate
declaration serve the scalar path and the lane-wise path, which is the vectorised-predicate story the
always-optimal-internals rule wants and which branch A cannot express without a second vocabulary.
That reading is one pass, mine, and it turns on whether `Mask<W>` is genuinely a truth value or merely
a container of them, which is a real question I have not settled and which a second reader should
attack directly.

### 3.5 The thing neither branch's statement says, and op should have it before he picks

D17's sentence is that under branch B "nothing in the signature needs bare `bool`". That is true of
signatures and does not extend to the contract's body, and the difference is worth one paragraph
because it is the only place either branch surprised me.

Rust's `if` takes `bool` and cannot be overloaded. So a truth contract must either name `bool`
somewhere as its exit, or supply a selector that substitutes for `if`. Both compile
(`103_probes/p1_foundation.rs`, both spellings; `p2_codegen.rs`, both measured), and the selector
spelling never names `bool` anywhere in the foundation or the consumer, lowering to `csel` rather than
to a branch.

But the honest reading is that the selector is not needed for the reason it looks needed. **Branch B
does not avoid `bool`; it avoids `arvo-platform`.** Naming `bool` in the foundation is exactly what
notko's own idiom already does, as the round's own predicate topic records
(`202607290100:127-130`: "notko's own idiom at these positions is bare `bool` (`Maybe::is`,
`Outcome::is_ok`, `Maybe::filter<P: FnOnce(&T) -> bool>`), which is its floor as the zero-dependency
foundation"). A foundation naming the language's own primitive is not the layering problem; a numeric
contracts crate depending on a *wrapper crate* is. So branch B costs the foundation one `bool` in one
method and delivers everything it promised, and the selector is available on its merits (it exists,
it is branchless, it costs nothing) rather than as the thing that rescues the branch.

If op picks branch B believing it removes `bool` from the design, he will be surprised later. It
removes the crate.

---

## 4. What this file does not decide

The fork itself: op's, both branches on record with prices, and my lean stated as a lean with its own
attack surface named. The crate's name: op's; the collision is a finding, the three candidates are
suggestions. Which of `Bool`'s four redundant doors survives: a consumer-ergonomics call the toolbox
rule puts with op; my claim is only that the number is one and that the chapter states which. Whether
the arity half of D15 is built before or after the property half: a sequencing call, and D15 decided
that both are built.

Owed artifacts, each named with what closes it:

- The `usize`-width refusal firing on a real 16-bit target. **Artifact:** one cross-compile, once a
  small target is installed.
- The notko-to-notko-hlist dependency edge D18b's split implies. **Artifact:** one compile with the
  list in its own crate.
- A test for the fresh-accumulator guard at `arvo-comb/src/greedy.rs:60-64`, in both directions, owed
  before any property is allowed to delete it. **Artifact:** two tests in the existing file.
- A second read on the mask-as-truth-type reading of section 3.4, which is the load-bearing input to
  the fork and is one pass.
- `arvo-bitfield`, the third and cheapest of the periphery content reviews, untouched here.

## 5. The three requirements, performed on this text before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions.
*Naming door* (section 1.1): defined there, in the sentence offered as the crate's charter, and used
nowhere else in the corpus, checked by grep. *Truth contract* (section 3.1): taken from D17's own
wording and given the content D17 leaves open only as far as the fork requires, namely that it is a
Boolean algebra with a stated exit; its **name** is explicitly left where D17 left it, unsettled.
*Exit* (section 3.5): defined at first use as the route from a truth value to Rust's `if`.
*Risk annotation* (section 2.2): defined at first use as the orthogonal axis to file 07's rungs, and
distinguished there from a rung. Terms used from the record without redefinition: the three rungs, the
provable-versus-trusted line, the trusted base, the layer-keying rule, the pricing pillar, the
toolbox rule, `Capacity`, `Cardinal`, Shape F. Named open rather than defined: whether `Mask<W>` is a
truth value or a container of them (section 3.4), which this file does not define because defining it
would presume the fork's answer.

**The separation requirement, performed.** Two models are this file's own. First, the four-way split
in section 1.2 between *settled at emission*, *ambient and assumed*, *provable*, and *trusted base*.
It is nonvacuous at the instantiation where the design's other chapters live: the rounding mode is
ambient and assumed and a linked library can change it, while the pointer width is settled at emission
and no library can, and the two land in different columns under the same question. Where it is vacuous
I say so: at `AsBool` the split adds nothing, because a projection is not a fact and the row says so
rather than filing it somewhere. Second, section 2.2's separation of the ladder's rungs from D16's risk
classes. It is nonvacuous at exactly one instantiation, D15's fresh-accumulator property, which is
rung 2 by compiler refusal and high-risk by D16's test, so the two axes disagree about it and a
one-axis reading has to pick. At a derived property like `LANE_ALIGNED = N % 4 == 0` the distinction is
vacuous, since it is rung 1 and low risk together, and I say so rather than letting the two-axis claim
imply it matters everywhere.

**The freshly-performed-search requirement, performed.** Every universally quantified negative above
carries its own search, run this session, quoted with its date.

- "`AsBool` and `BoolResidual` appear in zero panel files", `grep -rln "AsBool" .` and the same for
  `BoolResidual`, over the panel directory, 2026-08-05 06:13, both empty. `NUSize` returns exactly one
  file (`11`).
- "`Pred2`/`Pred3` appear in zero panel files", `grep -rln "Pred2\|Pred3" .`, 2026-08-05 06:13, empty.
  "`IsZeroOf`/`IsNonNegative` appear in exactly one", same time, returns `84` alone, my own.
- "File 55 contains the string predicate zero times", `grep -c "predicate" 55_*.md`, 2026-08-05 06:13,
  returns 0 on a file that mentions `arvo-comb` eight times.
- "No panel file has a section heading about the platform crate", `grep -n "^#.*[Pp]latform" *.md`,
  2026-08-05 06:22, empty. "No panel file records the public fields on these types",
  `grep -n "pub usize\|USize(pub\|Cap(pub\|Bool(pub" *.md`, same time, returns one hit (`79:22`,
  `Cap(pub USize)` tagged as a tree-fact) and nothing on `Bool` or `USize`; `grep -n "public field" *.md`
  returns one hit (`10:208`), about the union's `Poison` carrier, a different type.
- The `-> Bool` counts in section 3.3 are a fresh count over `mock/crates/*/src`, 2026-08-05 06:23,
  broken down per crate rather than summed, because the earlier whole-tree count (119 sites, 30
  declarations) includes test files and would have overstated the design surface by a third.

The honest limit, inherited from files 97, 98 and 101 and stated rather than assumed: these
performances verify that this file's terms are placed and its models have content and its negatives
were searched. They do not verify that a discussion using none of my search vocabulary exists
somewhere in one hundred and two files. A second reader with different terms is the check on that, and
after file 98 nobody should treat my greps as exhaustive because they were mine.

## 6. Standing

The platform decision is ratified and was never content-read, and reading it yields the same shape the
two previous periphery reviews yielded: no decision overturned, and corrections to what the decision's
ground was said to be. The crate is not the design's hardest trusted-base case; four of its five items
are compile-time-settled and the fifth is one sentence already written. What it needs is a charter with
one door in it, an `#[inline]` sentence about its own crate boundary, and a name that does not collide
with the token its own principles document defines.

The predicate decision is ratified, and the panel has quietly inverted its emphasis: the degenerate
instance carries the design's kind-2 failure story at three ratified sites, while the arity machinery
it was decided for has never been named by a panel file. D16 survives whole as the risk annotation on
file 07's rung 2. D17 is placeable, orphan-legal, and its analogy points at a referent that changed
crates in the same round.

The fork is free at runtime, by symbol identity rather than by measurement, and costs one type
parameter and one bound at fifteen trait declarations. It is not decided by cost. It is decided by
whether a second truth type exists, and one does, shipped, in a crate nobody counted.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (the round `202607300800` at D5 `round:542-556`, D6 `round:560-568`, D9
`round:588-600`, D15/D16/D17 `202607290200:10-57`, D18b/D19 `202607290500:18-39`, D27
`202607291700:8-39`; `102` sections 1.12, 1.26 and its opening section, at the lines cited in place;
the persona-tier `95b`/`101b` as marked), settled shapes (`07:310-345`, `03:50-94`, `74:61-73` and
`:247-253`, `84:386-393` and `:508-518`, `101:163-171`, the Shape F sketch at
`sketches/202607282230_hlist-arity-dissolution/FINDINGS.md:199-260`), compiled (`103_probes/p1`
through `p8`, all at the pin, edition 2024, no gates beyond `const_trait_impl`, commands and outcomes
in `103_probes/OUTCOMES.md`), measured (the instruction counts and the `_run_b1 = _run_a` symbol
alias, from `p2b.s` and `p4.s`, committed beside the probes; the `-> Bool` site counts, commands
inline), verified at source (`arvo-storage/src/platform.rs:45,73,261,264,275,285-291,293,328,342,485`,
`arvo-mask-contracts/src/lib.rs:45-66`, `arvo-comb/src/greedy.rs:32-34,60-64`,
`arvo-comb/tests/greedy.rs`, `arvo-storage/tests/bool_consttry.rs:47-50`,
`mock/PRINCIPLES.md.tmpl:322-326`, HEAD `d6a4d5c`), reasoned (the crate charter of section 1.1, the
rung-and-risk statement of section 2.2, the emphasis inversion of section 2.4, and the second-truth-type
reading of section 3.4, all mine, all one pass, offered as suggestion and not as a ruling).*
