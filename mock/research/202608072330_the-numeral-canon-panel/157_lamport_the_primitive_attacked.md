# 157. The primitive attacked: adequacy is two obligations, and only one of them is hard

Dispatched at `154`'s concession. Four targets in the order the brief gives them: the blind cold pair
`154` and `155`, and whether their agreement is arrival or artifact; `112:904-945`, the offered
statement of what a primitive is; `111:507-573`, the three-layer statement whose third layer is
adequacy; and `OPTIONS.md` Q52 with `111` section 18's answer to it.

The standing question underneath all four: whether a syntactic identity a compiler decides for free is
a sound and complete decision procedure for a denotational identity nothing can compute at a real
width, and what a design owes when it is not.

**The short answer, and it is the file's main result.** Adequacy is not one obligation. It is two, and
they are not the same kind of thing. **Soundness is free by functionality** and needs no enumeration at
any width. **Completeness is a conjunction of inequalities, and an inequality is discharged by one
witness**, so it is checkable at real width, at const time, with no transfer argument. `111` recorded
adequacy as "checkable the way `110` checks a congruence: at model widths, exhaustively, with the
transfer argument named rather than assumed" (`111:555-556`), which is true of neither half and is why
the obligation reads as unpayable. It is payable. The certificate compiles, at every width from 1 to
64, and the spurious-axis control fails to compile, which is the evidence
(`157_probes/p2_const_certificate/`).

**The second result, and it is about the corpus rather than about the primitive.** Across `110`, `111`,
`112` and `114`, eighty-two findings carry a width predicate and **not one carries `W any`**
(`157_probes/p3_predicate_audit.out`). Under I13's notation as op states it, an unlisted dimension
means the finding holds nowhere that dimension exists, and a narrowly listed one means it holds only
there. So topic five's corpus, read under its own rules, says nothing whatever about any width arvo
ships. Several of those eighty-two are **proofs recorded as measurements**, and `111:334-336` says so
about its own in the sentence after the predicate that traps it at three widths.

---

## 0. The two gates

### 0.1 The canon gate: passed

Checked against `INTENTS.md` read in full, including "How to read an entry", and against `RULES.md`
read in full.

The assignment is licensed. `INTENTS.md:288-289` (I14, IN FORCE) requires that "public API positions
use the stack's own primitives rather than bare integers, floats, `bool` or `usize`", which cannot be
stated without a determinate account of what one of those is, and `INTENTS.md:190-197` (I11) makes the
base and the contracts above it the library's stated purpose. Attacking the offered account is what
`RULES.md`'s attack-then-replace conduct asks for, and `113` is op's own instruction that a refutation
owes several replacements addressed to the party refuted.

Nothing I attack below is on the RATIFIED rung. `112:904-945` is one expert's offer, `111:507-573` is
one expert's proposal that names itself "nobody's yet", and `154` and `155` are two cold derivations.
I13 is the one RATIFIED entry and I use it as an instrument rather than argue with it.

**One thing I name rather than resolve, because it is op's and is already queued.** Section 2.3 below
shows that whether identity saturates at the literal depends on whether **footprint is observable**.
That is the same decision `156` item 1 puts to op as "the operation set the design ships". I do not
answer it. What I add is that the item is larger than `156` states: it decides not only the strategy
count but whether two primitives with identical value set and identical realisation map are one
primitive or two.

### 0.2 The test gate: passed, with `154`'s two defects reproduced, its soundness bug traced further, and one new fact

The suite-bearing surface is `mock/benches/variants/`'s thirteen `-shared` crates. `mock/crates/` is
empty and `mock/Cargo.toml` carries `members = []`.

**Counts, each with its command.** `157_probes/run_test_gate.sh`, output at
`157_probes/gate_release.out`:

```
bitpack-carrier-shared : 9 passed   bitpack-contend-shared : 12 passed
bitpack-footprint-shared : 6        bitpack-plan-shared : 5
bitpack-shared : 3                  bitpack-wide-shared : 6
quantiser-fadd-shared : 1           quantiser-radix-shared : 3
satfold-shared : 11                 warm-clamp-shared : 7
warm-container-shared : 15          wide-rung-shared : 30
bitpack-write-contend-shared : 15   (run separately, see below)
```

**108 in twelve crates plus 15 in the thirteenth is 123**, which reproduces `110`'s corrected count and
`154`'s corrected count. All at `--release`.

**My own first sweep measured nothing and exited 0, and I am recording it because it is the same class
as everything else in this section.** The first version wrapped each `cargo test` in `timeout`, which
does not exist on this host (`which timeout` returns nothing). Every invocation failed, the grep saw
nothing, thirteen crate headers printed with no results under them, and the script exited 0. A reader
skimming for the word `failed` would have called that green. The rewritten version at
`157_probes/run_test_gate.sh` asserts a positive pass count per crate and prints `MISSING OR ZERO`
otherwise, which is the negative control the first version did not have.

**Defect one reproduced at source.** `bitpack-write-contend-shared/src/stress.rs:97-112`,
`naive_kernel_corruption_rate_under_real_concurrency`, runs 3000 concurrent trials and contains no
assertion. Opened and read: the body ends in an `eprintln!` and a comment explaining that a
scheduler-dependent rate is not a threshold to gate on. The reasoning is right, and a `#[test]` that
cannot fail is a diagnostic in a coverage count. `154`'s reading of it is confirmed.

**Defect two reproduced, and it is worse than reported.** `154` observed the crate not finishing after
twelve minutes at the default profile, which is debug. I ran it at `--release`, so the profile cannot
be the explanation:

```
cargo test --offline --release ... -- --test-threads=1   15 passed, finished in 1.97s
cargo test --offline --release ...                       killed at 1724s, no result
```

Same host, same profile, same crate, one flag apart. **1.97s against more than 1724s, a factor above
870 and unbounded**, because the second was killed rather than finishing. The hang is not a debug-mode
artifact.

**The soundness bug traced past where `154` left it.** `154` names the raw pointer loads at
`bitpack-write-contend-shared/src/pool.rs:110-111`. Reading the protocol around them, the mechanism is
narrower and nastier than "two coordinators can race":

- `POOL` is a process-wide `OnceLock<PoolHandle>` (`bitpack-write-contend-shared/src/pool.rs:34`), so **all tests share one pool and one
  set of coordination fields**.
- `write_pass` stores the caller's `vals`, `out`, `n` into those shared fields and bumps `generation`
  (`bitpack-write-contend-shared/src/pool.rs:146-151`). Workers wake on `generation` and read whichever pointers are currently there
  (`bitpack-write-contend-shared/src/pool.rs:110-113`).
- `write_pass` then spins until `p.done` reaches `threads - 1` (`bitpack-write-contend-shared/src/pool.rs:158-160`), and `done` is
  **also shared and is reset by every caller** (`bitpack-write-contend-shared/src/pool.rs:150`).

So a second concurrent coordinator's workers can satisfy the first coordinator's `done` count. The
first returns, its `buf` (a per-trial local in `corruption_count`, `bitpack-write-contend-shared/src/stress.rs:41-50`) is dropped on the
next loop iteration, and a worker still holding the stale `out` pointer writes into freed memory. The
crate's own safety comment states the contract that is being violated: `out` "is not read or written by
anything else while this call is in flight" (`bitpack-write-contend-shared/src/pool.rs:129-131`).

**And it defeats the crate's own control.** `naive_kernel_never_corrupts_when_the_split_is_aligned`
(`bitpack-write-contend-shared/src/stress.rs:117-127`) asserts zero corruption at an aligned split, and exists to prove the observed
corruption is the boundary race rather than a harness defect. Cross-test pointer mixing can corrupt at
an aligned split, so under the default runner that control is not a control.

None of this touches my question and none is a reason to refuse. I proceed.

`holds for:` the timing figures hold at `profile = release, threads = 1 and default, host =
aarch64-apple-darwin four performance cores, crate = bitpack-write-contend-shared, toolchain =
the committed pin`. The source readings hold at `commit = db6710b1`.

---

## 1. The cold pair: two instances on the coarse claim, a located disagreement on the sharp one, and neither half read the other

### 1.1 The shared-input premise every brief in this unit carries is false for 23 of 75 rule files

`153:135` lists the unit's shared inputs and includes "the workspace's `.claude/rules/` which load
automatically". `110:113` states it as a fact about one file: "This workspace auto-loads
`arvo-always-optimal-internals.md` into every agent context." `155:602-616` restates that for itself,
`110` and `111`, and builds its independence accounting on it. `154` P2.0 contradicts it, saying that
rule "is not in my loaded rule set" and that it checked rather than assumed.

Measured. `157_probes/ruleset_diff.out`:

```
total workspace rule files: 75
loaded into 157's context:  52
carrying 'paths:' scope:    23

of the 23 NOT loaded, how many carry paths:   23
scoped rules that ARE loaded:                 (none)
```

The partition is exact. **The 23 files absent from my context are precisely the 23 carrying a `paths:`
frontmatter block**, and `arvo-always-optimal-internals.md` is one of them
(`/Users/orgrinrt/Dev/clause-dev/.claude/rules/arvo-always-optimal-internals.md:1-4` is
`---\npaths:\n  - "arvo/**"\n---`). So is `arvo-toolbox-not-policer.md`, which I17 rests on and which
`155` cites in its phase one.

`154`'s report is corroborated and `110:113` is false as stated. The mechanism neither of them named is
the frontmatter.

**What this costs the panel.** Every contamination declaration in this topic that says "I could not
have avoided reading it" is unreliable, in both directions. A member that read a path-scoped rule read
it deliberately, which is a different provenance from ambient exposure, and a member that did not read
it has no exposure to discount. `155`'s paragraph at `155:602-616` explicitly treats its own flag and
`110`'s as "one shared exposure, correctly flagged by both of us for the same underlying reason". On
this measurement that reading has no support: at least one member in the same unit, on the same
question, did not have the file.

**The honest bound on my own half of this.** The 52-file list is my report about my own context and
nobody can re-derive it from the repository; that half is unreproducible by construction. The
reproducible half is the 23/52 partition of the files, which anyone can run. What makes the pair worth
believing is that the two halves coincide exactly: a random 23-subset of 75 matching the scoped set is
one chance in `C(75,23)`.

**F157-1. The workspace rule set is not uniform across panel members, and the mechanism is `paths:`
frontmatter.** 23 of 75 workspace rule files carry it and none of those 23 was in this dispatch's
context, while all 52 unscoped ones were. `agent = one dispatch, session rooted at the workspace root,
workspace = clause-dev at commit db6710b1, threads any, target features any`. Evidence:
`157_probes/ruleset_diff.out`, `157_probes/loaded_rules_157.txt`. Second instance for the specific file
`arvo-always-optimal-internals.md`, after `154` P2.0; first instance for the mechanism.

### 1.2 Neither half of the pair read the other, so the unit produced two derivations and no reconciliation between them

Grepped. `154` contains zero occurrences of `155` or of its author's name. `155` contains three
occurrences of `154`, all in an addendum reporting that `154` at the time of reading "consists of its
header, its canon gate ... and a thorough, independently-run test gate" and "stops there"
(`155:698-706`).

So `155`'s reconciliation was written against a `154` that had no derivation in it, and `154`'s phase
two never saw `155` at all. **The pair is two independent instances and zero exchanges.** That is not a
criticism of either file, both of which handled it correctly by declaring it, and it is the fact that
governs what the pair earns.

### 1.3 Where they agree, and what each agreement is actually made of

Five overlaps. They are not worth the same.

**O1. A name is what a const predicate can be attached to.** `154` section 4 first bullet, citing
`INTENTS.md:214-226`; `155` section 4 first bullet, citing `INTENTS.md:248-261`. Both derive it from
I13, which both had. The inference from "the work is predicated arms" to "a predicate needs named
dimensions" is one step and the premise is shared. **One instance, not two.**

**O2. A name is what lets I15 hold, because monomorphisation needs something to specialise on.** `154`
section 4, `155` section 4 second bullet. Both from I15 and I14, both shared. **One instance.**

**O3. Composition is the right frame for one half of a primitive and the wrong frame for the other.**
`154` section 7 ("terms compose, treatments interpret"); `155` section 3 ("the identity composes, the
realisation is chosen"). Different instruments: `154` from its own footprint and container-relativity
probes, `155` from reading `warm-container-shared`'s arm set. Same corpus, which both briefs named as a
shared input. **Two readings of one shared object, which is weaker than two instruments and stronger
than one instance.** I would record it as a genuine convergence with the shared object named.

**O4. Something about a primitive is fixed and something varies.** True of both and nearly definitional.
Their sharp versions differ and are covered in 1.4. **Corroborates only at the coarse claim.**

**O5. What the tiers above need.** They conflict. See 1.4.

**F157-2. Two of the cold pair's five overlaps are single instances wearing two hats.** O1 and O2 are
both one-step inferences from a shared ratified premise with no instrument on either side. `topic =
the primitive, members = 154 and 155, shared premise = INTENTS.md I13 I14 I15, threads any, target
features any`. Evidence: the two files at the cited sections, both opened.

### 1.4 The located disagreement, which is the sharpest thing in the pair and which neither knows about

`154`, handed sentence 4 (`154` P2.5): "Where the value is a position rather than a datum, the
primitive is a view over a carrier and has no standalone size." Its section 6: "The widest arity at
which a single signature covers arvo's declared range is the column, not the element."

`155`, section 5 requirement 1: the tier above needs "a fixed, nameable identity usable as a type
parameter", and, in its own emphasis, "for the *value*, not the container".

These are opposites, stated in nearly the same vocabulary, on the same question, in the same unit.
`155` did not know because `154` had no derivation when it read it; `154` did not know because it never
read `155`.

**It resolves, and it resolves against `155`, on evidence `155` had in its own premise list.**

`155`'s instrument for requirement 1 is `warm-container-shared`'s `Carrier` trait, which it cites as
the model of a contract the tier above is generic over. `Carrier` is declared `pub trait Carrier: Copy
+ 'static` (`warm-container-shared/src/lib.rs:187`) and implemented for exactly `u8`, `u16`, `u32`,
`u64`, `u128` (`:279-283`). Every one of those is a **container**, and the bound is `Copy`, hence
`Sized`. The trait cannot reach a packed sub-word element, so it cannot disagree with `154`'s F6. This
is the identical defect `154` withdrew its own P4 for and `111` named in its section 9.4: an instrument
too thin to reach the case that breaks the claim.

**And the repository says the same thing four more times.** Every type declaration in the four
packed-end bench crates, `grep -nE "^(pub )?(struct|type|trait) "`:

```
bitpack-carrier-shared/src/lib.rs:191    pub struct CarrierColumn<const N: usize>
bitpack-footprint-shared/src/lib.rs:144  pub struct FootprintColumn<const N: usize>
bitpack-shared/src/lib.rs:206            pub struct Column<const N: usize>
bitpack-wide-shared/src/lib.rs:39        pub struct WideColumn<const N: usize>
```

Four crates at the packed end, four columns, **no element type in any of them**. At the native end
`warm-container-shared` declares both an element contract (`Carrier`) and a column (`Cols`,
`:762-767`). The shipped corpus's own vocabulary is exactly `154`'s arity claim: below one machine word
the column exists and the element does not.

**F157-3. At the packed end the bench corpus names a column and never an element, in four independent
crates.** Four of four packed-end `-shared` crates declare a `*Column<const N: usize>` and none
declares an element type; the one crate at the native width declares both. `region = packed sub-word
and native word, crates = bitpack-carrier-shared, bitpack-footprint-shared, bitpack-shared,
bitpack-wide-shared, warm-container-shared, commit = db6710b1, threads any, target features any`.
Evidence: the grep above, run and read. This takes `154` F6 from one instrument to five and settles the
`154`/`155` disagreement in `154`'s favour; `154` section 6 asked for a second instrument and has four.

### 1.5 Replacements owed to `155`

Per `113`, several, addressed to it rather than about it.

**S-1. Keep requirement 1 and move it one level down, to the lens.** What `155` wants from requirement
1 is that the tier above names a contract rather than enumerating cases, that the contract is const, and
that monomorphisation can dispatch on it. None of that needs the contract to be over a `Sized` value. A
**lens over a carrier** has all three properties: it is a type, it is const-parameterised, it
monomorphises. At the native end the carrier is one machine word and the lens is the identity, so
`Bool` stays `Bool` and nothing about the point of use changes. This is `154`'s O-B without O-B's stated
cost, which was that `Bool` and `USize` become one-element columns.

**S-2. Requirement 2 survives untouched and is the strongest thing in `155` section 5.** The
interior-safety predicate at `warm-clamp-shared/src/lib.rs:41-47` is a fact about identities, decided at
compile time, that deletes work when it holds. That is I13's arm-with-a-predicate instantiated in
shipped code, and it is independent of the element-versus-column question, because a predicate over a
fold's identities does not need an element type to exist.

**S-3. Your third open question is not resolved and should be reopened.** `155:571-585` records the
"is the bit a further primitive below the identity" question as resolved by `110`'s signature machinery
plus `111`'s saturation result. Section 2.3 below shows the saturation result has a premise that arvo
has not decided, and under the other branch of that premise a bit container and a numeral over the same
value set with the same realisation map are **two** primitives rather than one. Your instinct to record
it as unresolved was right and the resolution you accepted is conditional.

**S-4. Your identity/realisation split is undersold in your own phase two.** You call it "a correct but
strictly coarser fragment" of `112` section 9. It is coarser on the refinement and it carries something
`112` section 9 does not have at all: the split is what makes **adequacy statable**, because adequacy
is a relation between the two halves. `112`'s statement has no clause relating the type to the
denotation it decides. Section 3.4 below is that clause, and your split is the vocabulary it needs.

### 1.6 Replacements owed to `154`, at the concession

`154` section 8 O-C is the concession: it could not produce one account covering both ends that passes
`RULES.md:79-83`'s equivalence test, and expected the test to close O-C negatively.

**S-5. The equivalence test is about behaviour, not about spelling, and on that reading O-C closes
positively.** `RULES.md:79-83`: "would three teams implementing this independently produce things that
behave the same?" Two teams handed "a primitive is a value set with one realisation map over a declared
operation set" both find, at the packed end, that no `Sized` standalone form exists, because that is a
fact about the target rather than about the sentence; so both ship a lens there. At the native end both
can ship a value and both will. The teams converge **because the constraint that forced `154`'s wall is
external to the canon and binds every implementer equally.** The discriminator is cheap and I did not
run it: hand the sentence to two designers, ask each for a consumer-facing shape for a 13-bit packed
column and a 47-bit dense value, and compare behaviour rather than signatures.

**S-6. The arity question does not belong in the canon at all.** Whether an element type exists is the
concrete spelling of an implementation, which
`the-canon-is-intent-not-implementation.md` excludes by its permanence test: the sentence would need
editing the day a target with sub-byte addressing appears. What survives a rewrite is the *reason*: a
primitive's realisation is a placement of bits, and whether that placement has a standalone name is a
property of the target's addressing rather than of the primitive. Say that, and the canon is uniform
without saying too little.

**S-7. Your wall is `156` item 1 wearing different clothes, so it is not yours to break.** Whether an
element-level surface exists is a question about which observations the design ships. Topic eight
established that the observation set is a decision nothing measures (`146:646`, carried at `151:507`),
and `156` item 1 already puts it to op. Section 2.3 shows the same decision governs whether identity
saturates at the literal. Three separate walls, one decision. **Relocating a wall onto an existing
question is a better outcome than breaking it**, because it stops three topics each waiting on their
own version of it.

**S-8. Types are the degenerate case of lenses, not the other way round.** Stated as an account rather
than a taxonomy: a primitive's realisation is always a lens `(carrier, position)`; where the position
is const-zero and the carrier is one machine word, the lens is an identity and the thing is a value.
That is one vocabulary over the declared range, which is what O-B wanted, and it is uniform in the
direction that costs nothing, since the native end never has to mention the lens.

**S-9. Your F6 is now at five instruments, not one.** Section 1.4. You wrote that F6 is "the one I would
most like attacked". I attacked it with the crate that `155` cites against it and with the type
vocabulary of four more, and it survived both.

---

## 2. `112:904-945` attacked

### 2.1 What survives, with the instruments that could have refuted it and did not

**The first clause survives `154`'s F6 entirely, and `154` says so itself.** "A primitive is a value set
together with one realisation map taking an exact result back into it" (`112:904-905`) is a statement
about denotation. F6 is a statement about representability in Rust. A packed 13-bit element has a value
set (`0..8191`) and a realisation map; what it lacks is a `Sized` Rust type. `154` P2.4 concedes the
compatibility and is right to.

**The classification rule at `112:934-937` survives everything I threw at it, and section 3.4 shows it
is stronger than its author claims.**

**The refinement clauses survive my P5.** I built an instrument that could have shown a discharged
declaration changing an answer, at widths from 3 to 32, and it did not
(`157_probes/p5_output.txt`). Its two controls both fire, so the zero is a result rather than a dead
branch. Section 4 is about what that zero means.

**What I did not test:** the weakening order being total, the grade transformer clauses on
constructions, and the two-names-cost clause at `112:944-946`. Those stand un-attacked by me and I say
so rather than let silence read as endorsement.

### 2.2 "over a declared operation set" carries a theorem the statement does not state

`112:904-905` makes the operation set a parameter of the definition. `111` section 5.2 shows it is
almost not one, and gives the argument in prose: "Every term's argument to `R` is a rational. So no
signature can separate more than `R` differs somewhere on Q, and a signature containing a constant
injection over Q reaches that bound at depth one" (`111:334-337`).

That argument is a proof and it generalises further than either file states it. Written out:

> Let `f_op` be an operation's exact semantics on rationals. For every primitive `p` in a family sharing
> a value set, the denoted operation is `op_p(a, b) = R_p(f_op(a, b))`. If `f_op` does not depend on
> `p`, then any two primitives whose realisation maps agree on the whole of `Q` agree on every operation
> of every signature. Hence the partition a signature induces is determined by the subset of `R`'s
> domain that the signature's terms can reach, is monotone in that subset, and saturates exactly when
> the reachable subset is `R`'s whole domain. A full literal reaches it in one step.

Two things follow that neither `111` nor `112` writes down.

**The saturation is not a fact about literals.** It is a fact about **reach**. A signature with no
literal is coarser only because its terms cannot reach all of `Q`, which is why `111`'s richest
operation-only signature stops at 148 while `{literal}` gets 165 (`111:340-347`). Saying "identity
saturates at the literal" names the cheapest witness rather than the mechanism, and the mechanism is
what a canon wants, because it tells a design what to check when it ships a signature without a full
literal.

**The proof needs no width.** `111` states it as an argument and then records F111-7 with the predicate
`W in {2,3,4}` (`111:832-835`). Under I13's notation that finding holds at three widths and nowhere
else, forever, because `RULES.md` forbids widening a predicate in place. Section 5 is about how often
this happened.

### 2.3 The saturation theorem has a premise, and arvo has not decided it

The proof above needs `f_op` independent of `p`. Every arithmetic operation satisfies that. **An
observation does not.**

If the signature contains anything that reads the *container* rather than the value set, two primitives
with identical `(V, R)` and different containers are separated, and the bound fails. And arvo has a
stated reason to want exactly that observation: I17 (`INTENTS.md:363-380`) says the storage-minimising,
aggressively bitpacked path is not deprioritised, and I6 (`INTENTS.md:123-124`) says that path
"aggressively minimises and bitpacks". A consumer choosing that path is choosing on footprint, which is
a property of the container and not of `(V, R)`.

Measured. `157_probes/p4_saturation_bound.py`, output at `157_probes/p4_output.txt`, over a
64-configuration grid crossing `W in {3,4,5,6}`, `F in {0,1}`, both signednesses, both overflow
policies, and the two container rules `154` P4b identified (minimum rung against one rung above):

```
classes under {literal} alone                    : 32
classes under {literal, add, sub, mul}           : 32
classes under {literal} + container observation  : 64

D1 (operations add nothing to the literal)   : PASS   [32 -> 32]
D2 (container observation strictly refines)  : PASS   [32 -> 64]
D3 (classes the container splits)            : PASS   32 of 32
```

Three controls, all declared in the probe header before the run. D1 reproduces the saturation result on
an instrument built for something else. D2 is the attack. D3 makes D2 non-vacuous by confirming the
grid actually contains distinct containers over one denotation. The worked pair:

```
W=3 F=0 unsigned wrap, minimum-rung container   -> 8 bits
W=3 F=0 unsigned wrap, one-rung-above container -> 16 bits
```

Identical value set, identical realisation map, one primitive under `112:904-906` and two under any
design where a consumer can observe footprint.

**F157-4. The saturation bound holds only over signatures whose operations are functions of the value
set and the realisation map, and a container observation breaks it.** Adding a container observation to
`{literal}` doubles the class count, 32 to 64, splitting every class. `W in {3,4,5,6}, F in {0,1},
signedness any, overflow policy in {sat, wrap}, rounding = trunc, radix = 2, container rule in
{minimum, one rung above}, threads = 1, target features any`. Evidence:
`157_probes/p4_output.txt`. Refines `111` F111-7 and `112:904-905` by naming the premise; does not
contradict either, because both were measured under signatures satisfying it.

**And a smaller correction to the same claim, found by my own control failing.** Q52 and `111`'s
constant-injection result are quoted in the register as "rounding at `F = 0` is observable the moment
anyone writes a non-grid literal" (`OPTIONS.md` Q52). My first separation run used the literal `1/2`,
which is non-grid at `F = 0` and separates nothing: truncation sends it to 0 and ties-to-even sends it
to 0. `157_probes/p1b_literal_ties.out`, eight candidate literals at `W = 3` unsigned saturating:

```
1/2  non-grid  no          3/4  non-grid  SEPARATES
1/3  non-grid  no          2/3  non-grid  SEPARATES
5/2  non-grid  no          3/2  non-grid  SEPARATES
1    grid      no          2    grid      no
```

Four of six non-grid literals separate nothing. **Non-grid is necessary and not sufficient**, and the
sufficient condition at `F = 0` against ties-to-even is that the fractional part exceeds one half, or
equals one half over an odd floor. The existence claim the underlying argument needs survives; the
register's wording overstates it.

**F157-5. A non-grid literal is not sufficient to observe the rounding mode at `F = 0`.** Four of six
non-grid literals tested separate truncation from ties-to-even nowhere; two of six separate. All grid
literals separate nothing, which is the control. `W = 3, F = 0, signedness = unsigned, overflow policy =
sat, rounding in {trunc, near-ties-even}, radix = 2, threads = 1, target features any`. Evidence:
`157_probes/p1b_literal_ties.out`.

### 2.4 Replacements owed to `112`

**S-10. State the reach theorem instead of the literal.** "The identity a signature induces is
determined by the part of the realisation map its terms can reach; it is monotone in that part and
saturates when the reach is the whole domain." That survives a rewrite, says why, and tells a design
what to check. "Saturates at the literal" names one witness of it.

**S-11. State the premise beside it.** "The theorem holds over signatures whose operations are
functions of the value set and the realisation map. An observation that reads anything else, footprint
being the case arvo has a reason to want, separates primitives the theorem identifies." That sentence
is what makes `156` item 1 legible from this topic.

**S-12. Add the adequacy clause.** Your statement defines identity and classifies axes and never says
what the type owes the denotation. Section 3.4 shows your own classification rule at `112:934-937`
**is** that clause, per axis, and that stating it once is both shorter and stronger.

**S-13. Replace "over a declared operation set" with "over an observation set", and let operations be
the special case.** F157-4 says the thing that matters is what can be observed, not what can be
computed. Topic eight reached the same word from the other end (`146:646`). One vocabulary across two
topics is worth the edit.

---

## 3. `111:507-573` attacked: the adequacy obligation as stated cannot be discharged, and the one that can is a different obligation

### 3.1 The problem with the sentence

`111:555-556`:

> Adequacy is checkable the way `110` checks a congruence: at model widths, exhaustively, with the
> transfer argument named rather than assumed.

Set beside `111:520-523`, that the semantic relation "is undecidable at real widths ... Nothing in a
compiler will ever compute it for `W = 64`", and beside `112:960-962`, "Everything enumerative is at
`W <= 6` and I have made **no transfer argument** to any real width", the obligation as written is one
nobody in the unit can pay and nobody has paid.

Worse, the escape it names is closed by a rule already in force.
`unstable-features.md` says the honest default for a model-width check is that no transfer argument
exists, that its own enumeration of the mechanisms by which behaviour can vary per instantiation is not
exhaustive, and that a property whose quantifier range depends on the parameter can hold at one width
and fail at the next with code and gates held fixed. So "with the transfer argument named" is an
instruction to produce something the workspace's own rule says is usually unavailable.

**This is a real defect in the proposal and it is not fatal, because the obligation decomposes.**

### 3.2 Adequacy is two obligations, and they are not the same kind of thing

`111:531-535` states both halves correctly, and my elisions below are marked:

> **Soundness.** Syntactic equality never merges two different denotations. [...]
> **Completeness.** Syntactic inequality never splits one denotation into two names. [...]

Read as quantifiers, they are not symmetric.

**Soundness is a universal over pairs of a conjunction of equalities**, and it is free. If the type's
parameters are exactly what the realisation map reads, the denotation is a **function** of the
parameters. Equal parameters then give equal denotations by functionality. That is a one-line argument
holding at every width, with no enumeration over anything.

What soundness actually requires is that the denotation **factors** through the named parameters, which
is a structural property of the code rather than a property of the value space. Checked, with its
control. `157_probes/p1_separation_certificate.py`, RUN 1 and RUN 4:

```
RUN 1  parameters (W, F, signed, policy), sig {add,sub,mul}   SOUNDNESS: holds
RUN 4  parameters (W, F, signed), policy DROPPED              SOUNDNESS: FAILS
       counterexample: W=3 F=0 unsigned wrap  vs  W=3 F=0 unsigned sat
```

C3, declared before the run, was that a parameterisation carrying less than the map reads must fail.
It does. The check is not decorative.

**Completeness is a universal over pairs of a disjunction of inequalities**, and each disjunct is
discharged by **one witness**. To establish that two names denote different things, exhibit one input
on which they differ. That is `O(1)` per pair, at any width. Only the **refutation** of completeness
needs exhaustion, and refutation is not what a design has to do.

That asymmetry is the whole answer, and nothing in `111` or `112` uses it.

### 3.3 The certificate compiles, at real widths, and the spurious control does not

The claim is not merely arithmetic. `157_probes/p2_const_certificate/cert.rs` discharges the
completeness obligation for the overflow-policy axis **inside a const item**, which is the only form
I15 admits.

The certificate is one closed-form witness, uniform in the width: add the maximum to one. Wrapping
gives zero, saturating gives the maximum, and they differ whenever the maximum is non-zero, which is
every width from 1 up. No sweep, no model width, nothing to transfer.

```rust
const _: () = assert!(policy_separates_every_width());   // 1..=64, in one const loop
```

built with `rustc --edition 2021 -O cert.rs`, and the run at
`157_probes/p2_const_certificate/cert_run.out`:

```
separates_policy(64) = true
policy_separates_every_width() = true
separates_rounding_at_f0(64) = false
arity-1 mask at W=64: differing inputs in 0..100000 = 0   (154 P4's collapse, reproduced)
```

**The negative control was declared before the run and is the better half of the evidence.** A
genuinely spurious axis, two rounding modes at `F = 0` with an integer signature, must have no witness,
so the same assertion over it must fail to compile. Built with `--cfg control`,
`157_probes/p2_const_certificate/cert_control.err`:

```
error[E0080]: evaluation panicked: assertion failed: separates_rounding_at_f0(64)
  --> cert.rs:92:15
```

The compile failure is the finding. It says the certificate scheme rejects a parameter that denotes
nothing, at `W = 64`, at compile time, which is exactly the discipline
`a-test-that-cannot-compile-is-the-finding.md` ranks above every runtime result.

The line above it is worth its own sentence: **`154`'s P4 collapse reproduces**. Under an arity-1 mask
signature, wrapping and clamping agree on every one of 100000 inputs at `W = 64`. Under the arity-2
signature the certificate uses, they separate at `W = 64` and at every other width. `154` withdrew P4
for exactly this reason, and the certificate is what makes the withdrawal a rule rather than an
anecdote: **a signature with no separating witness for an axis has no business carrying that axis as a
parameter, and the certificate is what detects it.**

**F157-6. The completeness half of adequacy is dischargeable at real width by a separating witness, at
const time, with no transfer argument.** A single closed-form witness separates the overflow policy at
every width in 1..=64, verified inside `const` items, and the same construction refuses to compile for
a spurious axis. `W in 1..=64, container = u64/u128 exact intermediates, F = 0, signedness = unsigned,
overflow policy in {wrap, sat}, signature = arity-2 add, rustc = the committed pin, opt-level = 3,
threads any, target features any`. Evidence: `157_probes/p2_const_certificate/cert.rs`, `cert_run.out`,
`cert_control.err`. The `W any` extension is by the closed-form argument rather than by the sweep, and
is stated as `S-14` rather than claimed in this predicate.

**F157-7. The witness scheme and exhaustive denotational identity agree on every pair where exhaustion
is affordable.** 1128 of 1128 pairs, zero disagreements. `W in {2,3,4}, F in {0,1}, signedness any,
overflow policy in {sat,wrap}, rounding in {trunc, near-ties-even}, radix = 2, signature =
{add,sub,mul}, threads = 1, target features any`. Evidence: `157_probes/p1_output.txt` RUN 5. This is
the check that the cheap procedure is the same procedure, and it is the only part of the scheme that
needs a model width at all.

### 3.4 And `112`'s classification rule already is the adequacy condition, one axis at a time

This is the settlement the unit is short of, and it is a convergence rather than a refutation.

`112:934-937`:

> An axis is classified by **how many directions admit a total denotation-preserving map**. Two means the
> axis is spurious and must not be a parameter, because what is wanted there is equality and no
> equality exists. One means it is a refinement and may be a parameter, with the map as its weakening.
> Zero means it is part of the declared semantics and must be a parameter, because nothing coerces.

Verbatim, with the source's emphasis and no other. My own first draft of this block bolded the two
"parameter" clauses, which is the same defect S-18 reports in `111`'s quotation of `108`, committed by
me in the file that reports it. Caught by reading the source beside my rendering rather than by any
mechanism, which is why S-18 is worth stating rather than skipping.

Put that beside `111:531-535`'s two halves:

- **Two directions** means the two assignments are isomorphic, so no input separates them, so carrying
  the axis as a parameter creates two names for one denotation. **That is precisely a completeness
  violation**, and `112`'s "must not be a parameter" is its repair.
- **Zero directions** means neither assignment maps to the other, so dropping the axis from the type
  puts two denotations under one name. **That is precisely a soundness violation**, and `112`'s "must
  be a parameter" is its repair.

So `111` wrote that adequacy is "nobody's yet, and it is the obligation the other two are pointless
without" (`111:552-553`), while `112`, one file later, stated it per axis without naming it. Neither
noticed. `112`'s rule is the local form; `111`'s is the global form; they are the same obligation.

The measurement backs the identification. `157_probes/p1_output.txt` RUN 2 and RUN 3 carry the rounding
axis as a parameter at `F = 0`:

```
RUN 2  sig {add,sub,mul}            120 name-pairs, 8 with NO WITNESS
RUN 3  sig {add,sub,mul,literal=3/4} 120 name-pairs, 0 with NO WITNESS
```

C1 was that a spurious axis must have no witness. It fires: eight pairs, every one differing in nothing
but the rounding mode. C2 was that growing the signature must make it separable. It fires once the
literal is not a tie. **Which means the classification is not a property of the axis. It is a property
of the axis relative to the signature**, and the same axis is spurious under one and required under
another. `112` F112-2 measures this and calls it a direction count; the witness formulation makes the
signature-relativity explicit, which the direction count leaves implicit.

**F157-8. The axis classification and the adequacy condition are one obligation stated at two
granularities.** A two-direction axis carried as a parameter is a completeness violation; a
zero-direction axis omitted from the type is a soundness violation. Verified on the same grid on which
`112` F112-2 measured its direction counts, by the witness route rather than the map route, with both
controls firing. `W in {2,3,4}, F in {0,1}, signedness any, overflow policy in {sat,wrap}, rounding in
{trunc, near-ties-even}, radix = 2, signature in {{add,sub,mul}, {add,sub,mul,literal}}, threads = 1,
target features any`. Evidence: `157_probes/p1_output.txt` RUN 1 to RUN 5. This is a second instrument
for `112` F112-2 arriving from the adequacy side, and the identification itself is mine and is at one
expert.

### 3.5 Replacements owed to `111`

**S-14. Replace the adequacy sentence.** Instead of "checkable at model widths with the transfer
argument named" (`111:555-556`), write:

> Adequacy is two obligations. **Soundness** holds when the denotation factors through the parameters
> the type carries, which is a structural property of the design and needs no enumeration.
> **Completeness** holds when every pair of distinct parameter assignments is separated by some input,
> and a separating witness discharges one pair at any width. A design owes a witness per carried axis
> and a factoring argument for the whole.

That is checkable, it is auditable, and it is dischargeable at the widths arvo ships.

**S-15. Widen F111-7's bound half to `W any`, in your own file's own terms.** You wrote the argument
and then predicated the finding at three widths. `RULES.md` forbids widening a predicate in place, so
the widening has to be a later expert's claim, and this is it: the bound half holds at `numeral any,
signedness any, F any, W any, overflow policy any, rounding any, radix any, threads any, target
features any`, subject to F157-4's premise. The 165/148 magnitudes stay where you put them.

**S-16. Your three-relation lattice gains a fourth property, not a fourth relation.** You wrote that "a
design is exactly as good as the gap between nominal and denotational" (`111:567-568`). The certificate
makes that gap **measurable rather than only nameable**: it is the set of carried axes with no
separating witness under the shipped signature, and that set is computable at const time. A design can
assert it is empty and the compiler can check it.

**S-17. Do not require the signature to be closed to get the guarantee.** Your section 5.3 already says
a design that can write a literal needs no closed operation set for stable identity. The certificate
sharpens it: what a design needs is a witness per axis under **the signature it actually ships**, and
growing the signature can only add witnesses, never remove them, because a term that separated two
assignments is still a term. So the certificate is monotone in the signature and never has to be redone
when the signature grows. That is the property F4's "the day somebody adds `half`" worry was really
about.

**S-18. One small citation defect, said because you would want it said.** Your quotation of `108:825`
at `111:1140-1145` renders the whole membership clause bold. The source bolds only the word "any"
(`108:825`). Nothing turns on it and the sentence you rely on is there.

---

## 4. Q52's open item, `111` section 18, and whether the predicate covers what the conclusion needs

### 4.1 What was asked and what was measured

Q52's closing open item: "Whether the refinement is a **new coordinate** or already a member of `106`
section 1's first component, the observable assignment."

`111` section 18 answers it by applying the criterion at `108:825`, "An axis belongs here if there is
**any** reachable chain on which moving it is observable", and measuring at
`111_probes/r1_moving_only_the_refinement.py`: the primitive fixed at `uW3/sat`, one-sided declarations
`[0, b]`, six terms, 1753 declaration pairs changing the selected arm and 0 chains changing the answer,
with a control moving an observable axis and reporting differences in the tens of thousands.

### 4.2 The instrument is existential and the conclusion is a universal negative

The criterion quantifies over **any reachable chain**. Concluding that an axis does *not* belong
requires that no chain observes it, which is a universal over the term algebra. Six terms at one
primitive at one declaration shape is a spot check of that universal, and a spot check of a universal
establishes nothing about the cases it did not visit. Under this panel's own predicate notation the
finding holds at `W = 3`, `overflow policy = sat`, that declaration shape and those six terms, and
nowhere else.

**So the answer to the brief's question is no: the predicate does not cover what the conclusion needs.**

### 4.3 And it does not need to, because the zero is entailed

Attacking that blocker rather than reporting it. The zero is not an empirical fact about six terms. It
is the arm licence condition restated.

An arm is substituted for another on a term only where its licence proves it agrees with the general
arm on the declared box. Every value the sweep tests lies in the tighter of the two declarations, hence
in both boxes, hence in the region where the licence proves agreement. **Agreement there is what the
licence says.** The sweep therefore measures whether the licence is sound, which is a real and
falsifiable property, and the axis classification follows from the definition of a refinement, which is
a declared over-approximation of where the arguments lie.

Built and run, `157_probes/p5_r1_is_about_licensing.py`, output `157_probes/p5_output.txt`:

```
SOUND LICENCE, only the declared bound moves, primitive fixed
  W=3  sat add exhaustive     pairs=28    arm changed=16   answer changed=0
  W=5  sat mul exhaustive     pairs=496   arm changed=156  answer changed=0
  W=16 sat add sampled        pairs=2145  arm changed=248  answer changed=0
  W=32 sat mul sampled        pairs=2145  arm changed=65   answer changed=0
  E3 (zero survives past the model width) : PASS  total=0

CONTROL E1, an off-by-one licence, unsound by construction
  W=3  sat add exhaustive     pairs=28    arm changed=15   answer changed=3
  W=5  sat mul exhaustive     pairs=496   arm changed=175  answer changed=25
  E1 (an unsound licence is detected)     : PASS  total=65

CONTROL E2, move an observable axis with declarations fixed
  overflow policy moved: 644 of 1344 argument pairs disagree   PASS
```

`111`'s zero reproduces at `W = 3` and extends to `W = 32` on a second instrument with a different
declaration sampling and a different term set. E1 shows the instrument can report a difference when the
licence is wrong. E2 reproduces `111`'s own control.

**My own E1 failed on its first run and I am leaving the failure in the file and its output committed.**
The first version's unsound licence returned true for every bound in the value set, so both sides of
every pair selected the cheap arm, no pair differed, and the control reported zero. The reasoning in
the probe would then have been supported by an instrument that could not disagree with it. That is the
fifth instance of one failure mode in this topic, after `110`'s P4, `110`'s P8 first run, `111` section
9.4 and `154`'s P4, and this one happened to me inside the probe I built to point out the same class in
someone else's work. Output at `157_probes/p5_output_E1_control_failed.txt`; the corrected licence and
the reason it was corrected are in the source at the `licence_unsound` docstring.

**F157-9. The zero at `111` section 18.2 measures arm licence soundness, and the axis classification
follows from the definition of a refinement rather than from the count.** A second instrument
reproduces the zero at widths 3 to 32 over add and mul; an off-by-one licence is detected at 65
declaration pairs; moving the overflow policy disagrees on 644 of 1344 argument pairs. `W in {3,4,5}
exhaustive and W in {8,12,16,24,32} sampled at 64 bounds by 400 values, F = 0, signedness = unsigned,
overflow policy = sat with wrap as the control, operation in {add, mul}, arity = 2, declaration shape =
one-sided [0,b], threads = 1, target features any`. Evidence: `157_probes/p5_output.txt`.

### 4.4 So Q52's open item is closed, and closed by something better than the sweep

The refinement is not a member of the declared semantics. `112`'s verdict stands and `111`'s withdrawal
of its own lean stands. What changes is what the verdict rests on: not 1753 pairs at one primitive but
the definition of a refinement plus a licence-soundness obligation, which is checkable per arm and does
not have to be redone per term, per primitive or per width.

**S-19, to `111`.** Restate section 18.2's finding as two: the classification, which is analytic and
carries `W any` under the licence obligation; and licence soundness, which is empirical and carries
your measured predicate. As written the analytic half inherits the empirical half's predicate and is
trapped at `W = 3`.

**S-20, to whoever writes the ledger.** `AGREEMENTS.md` has sections for topics one, two, three, four,
six, seven and eight, and none for five (`grep -n '^## ' AGREEMENTS.md`). The absence is not that the
topic settled nothing. It settled the reach theorem, the classification, the law-set exclusion and the
refinement's location; what it never had was the machinery that records them.

---

## 5. The corpus finding: topic five states nothing about any width arvo ships

Counted rather than asserted. `157_probes/p3_predicate_audit.sh`, output
`157_probes/p3_predicate_audit.out`:

```
file                                     findings  W-fixed  W-any  threads-any  tfeat-any
110_willsey_the_primitive_derived_cold        16       10      0            1          0
111_jhala_the_primitive_attacked              20       21      0            6         13
112_leijen_where_the_refinement_lives         25       24      0            6         13
114_leroy_formalising_the_primitive           21       15      0            5         13
```

**Eighty-two findings, seventy of them carrying an explicit fixed or enumerated width, none carrying
`W any`.** The control is in the same table: `threads any` fires 18 times and `target features any` 39
times across the same files, so a zero in the `W any` column is the corpus and not the pattern.

Under I13 as op states it and as `RULES.md` restates it, absence means the finding does not hold in any
situation involving that dimension, and a fixed listing means it holds there and only there. Every
value arvo computes on has a width. So the whole of topic five, read under the discipline it was written
under, holds nowhere in the library.

**That is an artifact of the recording rather than of the work**, and the mechanism is specific. Several
of those findings are proofs. `111:334-336` says of F111-7 that "the bound half is structural rather
than enumerative ... which is an argument and not a sweep", in the sentence immediately after the
predicate `W in {2,3,4}`. `112` marks three findings `threads any` on exactly this ground, that a
compile-time structural result is not a function of a runtime thread count, and does not apply the same
reasoning to width for anything.

**The general form is what I would want carried.** A proved claim recorded with a measured predicate is
**under-claimed**, and under-claiming in this notation is not the safe error. It looks conservative and
it is a false negative that no later reader may repair in place, because a predicate is never widened
where it was written. The repair costs a later expert's file every time, which is what sections 2.4,
3.5 and 4.4 are.

**F157-10. Not one of the eighty-two findings in topic five's four principal files carries `W any`,
while `threads any` and `target features any` appear 18 and 39 times in the same files.** `corpus =
110, 111, 112, 114 at commit db6710b1, pattern = the greps in 157_probes/p3_predicate_audit.sh, threads
any, target features any`. Evidence: `157_probes/p3_predicate_audit.out`.

---

## 6. Options opened, each with what would close it

**Q157-A. Is footprint in the observation set?** If yes, two primitives with identical value set and
identical realisation map and different containers are two primitives, the saturation bound does not
hold over arvo's shipped signature, and identity is container-relative exactly as `154` P4b measured.
If no, the container is an internals decision and the bound holds. **Closed by** `156` item 1, which is
already op's, so this option is a restatement of that item's reach rather than a new fork. Costs: under
yes, the identity relation depends on a treatment and the count of primitives is not a fact about the
format; under no, a consumer choosing the Cold path cannot express that choice in the type, which
fights I17.

**Q157-B. Is the certificate obligation per axis or per pair of shipped instantiations?** Per axis is
cheaper and is what S-14 states. Per pair is stronger and is finite per compilation, because a program
names finitely many primitives, and it lands the check exactly where `112:944-946` says the cost of two
names lives. **Closed by** exhibiting an axis pair that is separated per-axis and unseparated at some
particular instantiation; if none exists the per-axis form is sufficient and is the one to ship. I did
not look for one.

**Q157-C. Does the lens formulation (S-8) survive a consumer?** Types as the degenerate case of lenses
gives one vocabulary across the declared range. **Closed by** writing the element-facing ergonomics
over the lens and checking whether the sugar is thin, which is `154` O-B's own discriminator applied to
a different carrier. If the sugar is thick, `154` O-A's two vocabularies is the honest answer.

**Q157-D. Is soundness's factoring obligation checkable mechanically, or only by inspection?** The
argument is structural, so a lint could plausibly check that no realisation-map input is absent from
the type. **Closed by** attempting it against a real shape; I did not, and until someone does, the
soundness half is an audit obligation rather than a gate, which is weaker than the completeness half and
should be said rather than glossed.

**Q157-E. What is the certificate's cost in compile time?** The witness check is `O(1)` per pair
evaluated at const time, and const evaluation is not free. **Unpriced.** Nothing on `mock/benches/` has
measured a const-evaluation budget, and my `rustc` invocation on a 100-line file is an ad-hoc quick
spike with no substance and decides nothing about magnitude. **Closed by** a harness arm with real
competitors: no certificate, a per-axis certificate, and a per-pair certificate, over a realistic
instantiation count.

---

## 7. What I am carrying forward unchanged, and from whom

**Nine items, from five authors, none of which I derived myself.**

From `112`: the first clause of `112:904-906`, that a primitive is a value set with one realisation map
over a declared operation set, which I attacked at both ends and could not break (1 item). The
classification rule at `112:934-937`, which I attacked and which turned out to be stronger than its
author claimed (1 item). The two-names-cost clause at `112:944-946`, untested by me and carried on its
account (1 item).

From `111`: the two halves of adequacy as stated at `111:531-535`, which are exactly right and which I
am only re-quantifying (1 item). The reach argument at `111:334-337`, which I restated as a theorem
rather than amended (1 item). The withdrawal of its own lean at `111:1192-1193` (1 item).

From `110`, second-hand through `111` and `154` and not verified by me: the definitional-versus-
reachability distinction (1 item).

From `154`: F6, at the packed end a primitive is not a `Sized` type, which I attacked with `155`'s own
instrument and with four crates and which survived both (1 item). This is the one carried item where I
am also an instrument, so I count it as carried rather than as my own.

From `108`: the membership criterion at `108:825`, which I applied rather than argued with (1 item).

**Amended: 2.** `111` F111-7's predicate (S-15) and `111` section 18.2's predicate (S-19), both widened
in my file rather than in theirs, per `RULES.md`'s rule that a predicate is never widened in place.

**Refuted: 1.** `155` section 5 requirement 1's "for the value, not the container", section 1.4.

**Of my own withdrawn: 0**, and I record that as a warning rather than a boast. `154` withdrew three,
`110` withdrew several, `111` withdrew a lean. A file that withdraws nothing has either been lucky or
has not built an instrument that could embarrass it. Mine embarrassed me once, at P5's E1 control,
which I fixed rather than withdrew because the reasoning survived the corrected control.

---

## 8. Coverage, bounded honestly

**Read in full:** `INTENTS.md`, `RULES.md`, `154` including both phases, `155` including both phases,
`153`, `156`, `113`, `OPTIONS.md` Q52 in full, `AGREEMENTS.md` section headings.

**Read in part, at the cited sections, opened rather than remembered:** `111` sections 5, 8, 9.1, 18,
19, and its findings block around F111-7 and F111-8; `112` section 9 in full plus its weaknesses
paragraph and its findings F112-1 to F112-3; `110` sections around its contamination declaration;
`109`'s section on the four decisions; `108:820-830`.

**Grepped, not read:** the rest of `109`, `110`, `111`, `112`, `114`; the whole of `115` through `152`;
`63`, `74`, `90`, `106`, `146`, `151`; `DROPLIST.md`; `PRIOR_CALLS.md`; `PERSONA_CALLS.md`; `HANDLES.md`;
every `SEED_*` file; `archive/`; `seed/`.

**Not opened at all:** every panel file numbered 1 to 107 except `108` at the cited lines; the four
finished candidates `63`, `74`, `90`, `151` beyond what `153` and `156` say about them; every
`.csv` and `.meta.json` under `mock/benches/`; 89 of the 94 variant crates.

**Which sections would move if something I leaned on were wrong.**

- **Section 1.1 rests on a self-report about my own context**, which nobody can re-derive. If the
  `paths:` mechanism is not what suppressed those files, F157-1's mechanism is wrong while its
  observation (`arvo-always-optimal-internals.md` was not in my context) stands, and `154`'s report
  stands with it. The 23/52 partition of the files is reproducible either way.
- **Section 2.3 rests on the container being observable being a live possibility.** If op settles
  `156` item 1 by excluding footprint from the observation set, F157-4 becomes a statement about a
  design arvo did not choose, and the saturation bound holds unqualified. The measurement does not
  change; its relevance does.
- **Section 3.4's identification is mine and is at one expert.** Both halves are quoted from files I
  opened, and the inference between them is a single step, but nobody else has made it. It wants a
  second read more than anything else in this file.
- **Section 1.4's resolution rests on `Carrier`'s impl list**, which I opened
  (`warm-container-shared/src/lib.rs:279-283`), and on the four-crate grep, which I ran. If any of the
  four packed crates declares an element type I did not see, F157-3's count drops and `155`'s
  requirement 1 gains an instrument.
- **Section 0.2's soundness trace rests on my reading of the pool protocol**, not on a reproduction of
  the use-after-free. I reproduced the hang and read the code; I did not run it under a sanitiser and
  I do not claim to have observed the write through a stale pointer.

**Citations checked by opening them.** Every `file:line` in this document was opened and its content
read rather than its resolution confirmed. Two were wrong on the first pass and are corrected here:
`112`'s statement is at `904-945` and not at the `890` I first wrote from a slice, and `111`'s three
layers are at `550-553` with the checkability sentence at `555-556`, inside section 8 whose heading is
at `507`. The brief's `507-573` is the right range for the section and the wrong one for any sentence
in it, and I cited `571-573` for the checkability sentence in a first draft, where those lines are a
blank line and a horizontal rule. Eight further citations in that draft were bare `pool.rs:N` and
`stress.rs:N` with no crate prefix, unresolvable by anyone who does not already know which of the
ninety-four variants is meant, which is the identical defect `154` found in its own file. Both classes
were found by `157_probes/citecheck.py`, which carries two deliberately wrong citations as its control
and reports them: **72 citations, 0 failures after repair, 2 controls firing**
(`157_probes/citecheck.out`).

**What I settled.** That adequacy decomposes into a free half and a certifiable half, and that the
certifiable half is dischargeable at real width in a const item, with the spurious-axis control failing
to compile. That `112`'s axis classification and `111`'s adequacy obligation are one thing.

**What I moved.** The saturation bound, from an unqualified result to a theorem with a premise arvo has
not decided, and the premise onto an item already queued for op. The `154`/`155` disagreement, from
unnoticed to located and resolved. `111` section 18.2's zero, from a measurement of the axis to a
measurement of licence soundness.

**What I could not.** I could not price anything. Q157-E is unpriced and the word is used deliberately:
nothing on `mock/benches/` measures a const-evaluation budget, and my `rustc` invocation on a 100-line
file is an ad-hoc quick spike with no substance. I also could not check whether the per-axis certificate
is sufficient or whether the per-pair form is needed (Q157-B); I looked for a counterexample by
inspection for about as long as that is worth and did not build an instrument for it, which is the one
place in this file where I stopped short of the rabbit hole rather than exhausting it.
