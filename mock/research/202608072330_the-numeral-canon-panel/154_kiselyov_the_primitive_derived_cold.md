# 154. What a primitive is, derived cold

**Phase one. Written blind.** Read: `INTENTS.md`, `RULES.md`, this repository's `.claude/` and
`CLAUDE.md`, the workspace rules at `/Users/orgrinrt/Dev/clause-dev/.claude/rules/`, `mock/Cargo.toml`
with its comments, `rust-toolchain.toml`, and `mock/benches/` (layout, `bench.toml`, the variant
crates, the committed CSV/meta/findings). Nothing else. No panel file, no `AGREEMENTS.md`,
`OPTIONS.md`, `DROPLIST.md`, `HANDLES.md`, `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, no `SEED_*`, nothing
under `archive/` or `seed/`, no other member's probes, no `git log`, no commit subject.

**Declaration about the cut.** This dispatch was interrupted by a network error partway through the
test gate and resumed. Nothing outside the permitted premise list was read before the cut or after
it. The only thing on disk from before the cut was this file's header.

Phase two is appended at the end and phase one is not rewritten.

---

## 0. The two gates

### 0.1 The canon gate: passed, with one thing I am naming rather than resolving

Checked against `INTENTS.md` in full, including its "How to read an entry" section. The assigned
question ("what is a primitive, in arvo?") is licensed: `INTENTS.md:190-197` (I11) makes the base and
the contracts above it the stated purpose of the library, and `INTENTS.md:288-289` (I14) makes "public
API positions use the stack's own primitives rather than bare integers, floats, `bool` or `usize`" an
IN FORCE constraint, which presupposes that there is a determinate answer to what one of those is. A
canon cannot state I14 without stating what a primitive is.

**What I am naming rather than resolving.** The word "primitive" appears in the governing material in
at least three incompatible senses, and no intent disambiguates them. Section 1 below is that finding.
Under the panel's ambiguity handling this would be grounds to hand the call back; I am not doing that,
because the brief's own question ("whether a primitive is one thing or several things under one word")
is precisely this question, so the ambiguity is the assignment rather than a blocker to it. I flag it
so that a later reader does not mistake my section 1 for a decision op made.

### 0.2 The test gate: passed, and the suite is unusually good, with two real defects

`mock/crates/` is empty (`ls mock/crates/` returns nothing; `mock/Cargo.toml:19` has `members = []`
with a header explaining the deletion). The suite-bearing surface is therefore
`mock/benches/variants/` alone.

**Counts, each with its command.**

```
grep -rn "#\[test\]" mock/benches/variants/ mock/benches/src/ | wc -l      -> 124
ls mock/benches/variants/ | wc -l                                          -> 94
grep -rl "#\[test\]" mock/benches/variants/ | sed 's|.*/\([^/]*\)/src.*|\1|' | sort -u | wc -l -> 13
```

Thirteen of the ninety-four variant crates carry tests; they are the `*-shared` crates, which is the
right place for them (the arms are thin, the model is shared). `cargo test` at `mock/benches/` runs
**0 tests**, because the variants are path dependencies rather than workspace members, so the suite
has to be run crate by crate. That is a real trap and I record it: an agent that runs `cargo test` at
the bench root and reads `test result: ok. 0 passed` as green has measured nothing.

Per-crate results, `cargo test --offline --manifest-path variants/<c>/Cargo.toml`:

| crate | tests | result |
|---|---|---|
| bitpack-carrier-shared | 9 | ok |
| bitpack-contend-shared | 12 | ok |
| bitpack-footprint-shared | 6 | ok |
| bitpack-plan-shared | 5 | ok |
| bitpack-shared | 3 | ok |
| bitpack-wide-shared | 6 | ok |
| bitpack-write-contend-shared | 15 | ok, but see below |
| quantiser-fadd-shared | 1 | ok |
| quantiser-radix-shared | 3 | ok |
| satfold-shared | 11 | ok |
| warm-clamp-shared | 7 | ok |
| warm-container-shared | 15 | ok |
| wide-rung-shared | 30 | ok, 115.43s |

I read test bodies rather than names, and scanned all 123 parsed test bodies mechanically for the
absence of any assertion or panic (`154_probes/gate_scan.py`, committed with its output). Eighteen came
back with no assertion in their own body; seventeen of those delegate to a helper that asserts
(`bitpack-carrier-shared/src/lib.rs:361-363` calls `check_size::<16384>()`, and the same shape for the
rest), and the eighteenth is a macro body. **No tautological test found.** No test asserting a constant
against its own definition. No sampled law where the whole matrix was available: the opposite, in fact,
and the crates say so in their own comments.

`warm-container-shared/src/lib.rs:1503-1504`:

> /// The shipped rule widens every width at or below 64 bits. Asserted
> /// over all 64, since a sample would not establish "every".

`warm-container-shared/src/lib.rs:1352-1354` on its oracle:

> /// The value is also checked against the independent `u128` reference,
> /// so four agreeing arms sharing one wrong transform is caught rather
> /// than confirmed.

And `warm-container-shared/src/lib.rs:1420-1423` records a probe that was dead and how it was found:

> /// The first version of this check reported zero nanoseconds and was
> /// wrong: it exclusive-ored the result into a sink an even number of
> /// times, so the sink was provably zero and the whole loop was dead.
> /// `black_box` on both ends is what makes the call observable.

That is the negative-control discipline applied by the author to its own instrument, which is the
thing this workspace's rules were written because nobody did.

**Defect one: a test that structurally cannot fail, costing minutes of wall clock on every run.**
`bitpack-write-contend-shared/src/stress.rs:96-111`,
`naive_kernel_corruption_rate_under_real_concurrency`, runs 3000 concurrent trials and then asserts
nothing at all. Its own comment (`bitpack-write-contend-shared/src/stress.rs:105-111`) explains why, and the reasoning is sound: a
scheduler-dependent corruption rate is not a threshold anybody should gate on, and the sibling control
`naive_kernel_never_corrupts_when_the_split_is_aligned` (`bitpack-write-contend-shared/src/stress.rs:114-127`) does the gating. **The
reasoning is right and the placement is wrong.** A `#[test]` that cannot fail is not a test, it is a
diagnostic, and putting it in the default run means every `cargo test` in that crate pays thousands of
threaded trials for a number that is read off stderr. It belongs behind `#[ignore = "diagnostic: ..."]`
so the count that gets cited as coverage stops including it and the default run stops paying for it.
This is the one place in the suite where the gate's "tests that assert nothing" bullet lands, and it
lands softly, because the author declared it.

**Defect two: the suite is slow enough that it will stop being run.** `wide-rung-shared` takes 115s and
`bitpack-write-contend-shared` had not finished after twelve minutes of wall clock on this host, held
by the three `stress::` tests. A suite nobody runs is a suite that is not measuring, whatever its
quality, and the whole cost sits in two files. Moving the three stress tests behind `#[ignore]` (they
are explicitly "run outside the timed bench path", `bitpack-write-contend-shared/src/stress.rs:1`) takes the surface from twelve-plus
minutes to under two.

Neither defect touches the question I was dispatched for and neither is a reason to refuse the work.
I proceed.

---

## 1. The word is doing three jobs, and they do not pick out the same set

Before answering what a primitive is, I have to record that the governing material uses the word in
three senses that come apart, and that no intent disambiguates them.

**Sense A, the substitution sense.** The workspace `CLAUDE.md` table maps `u8..u128` to
`arvo::UFixed<I, F, S>`, `bool` to `arvo_bits::Bool`, `usize` to `arvo::USize`. Here a primitive is
*whatever stands in the position where Rust would have had a primitive*. It is a role in a
substitution table. `vocabulary.md` gives the same sense sociologically: "**primitives** | Named types
and traits consumer code uses directly." Under this sense, a thing is primitive because consumers
name it.

**Sense B, the generator sense.** A primitive is an irreducible element of a generating set: what
everything else is built from and which is not itself built from anything in the system. Under this
sense `UFixed<I, F, S>` is not primitive at all. It is a composite of a width, a radix point, a
signedness and a treatment, and a `Cap<MAX>` is a refinement over a `USize` rather than a generator.

**Sense C, the lowering sense.** A primitive is what survives to one machine operation. I15
(`INTENTS.md:304-307`, "everything ... go through one lowered path") makes this sense operative in
arvo specifically, in a way it is not in most libraries.

**They disagree, demonstrably.** Under A, `Cap<MAX>` is a primitive; under B it is a refinement.
Under A, a bitpacked 13-bit element is a primitive; under C, P2/F6 shows it is not a value at all.
Under C, P1/F4 and P3/F8 show that forty named primitives are twenty machine objects, so C
under-counts A by construction and does so by an amount that depends on what the bodies compute.

I am not asking op to disambiguate this, because it is not an intent question. It is the panel's
work, and the brief's own phrasing ("whether a primitive is one thing or several things under one
word") says so. What follows is my attempt at the decomposition. **Where I use "primitive" without a
qualifier below, I mean sense A**, because that is the one I14's public-API constraint is written in,
and I say which sense I mean everywhere it matters.

## 2. I15 does not merely prefer saturation. It entails it.

This is the load-bearing derivation and it is short.

I14 (`INTENTS.md:284-289`) says sizes are const and monomorphisation is the dispatch. I15
(`INTENTS.md:304-307`) says "Never any runtime checks, ever ... unused paths we clear out when
lowered. Period."

P1 measured what happens when a parameter of a numeral is left runtime. `sat.s:31-39`, arm
`b_unsat`, whose only difference from the const arm is that the width lives in the value:

```
	ldr	w8, [x0, #8]
	mov	x9, #-1
	lsl	x10, x9, x8
	cmp	w8, #63
	csinv	x8, x9, x10, hi
```

`cmp w8, #63` is a runtime check. It is the source's `if w >= 64` guard, and the compiler could not
remove it because it cannot know the width is in range. The const arm at the same arithmetic is
`and x0, x0, #0x7fffffffffff`, one instruction, no comparison (`sat.s:26`).

So: **any parameter of a numeral left runtime forces a check; I15 forbids checks; therefore every
parameter of a numeral but the value itself is resolved before lowering.** That is not a stylistic
preference about how to spell a type. It is I15 restated at the type level, and it gives a
definition rather than a taxonomy:

> A primitive is a construction with exactly one runtime degree of freedom, its value. Everything
> else about it is settled before lowering.

I13's refinement widens what "settled before lowering" ranges over and does not weaken it. Op
(`INTENTS.md:252-255`): the predicates "collapse to whatever is available at const time ... allows
using const functions and pipe in some data that is outside the typestate." So the saturating
parameters are **not** necessarily type parameters. A `const` in a consumer crate that an arm's
predicate reads is part of what saturates a primitive and is not in the type. **A primitive is
saturated at const time, not saturated in its generics**, and those are different claims. The second
is the one an implementation reaches for and it is narrower than the intent.

`holds for:` this derivation is over the intents rather than over a measurement, and the measurement
supporting its middle step holds for W in {13, 47}, container = u64, arity 1, target features =
baseline aarch64-apple-darwin, opt-level = 3, threads = 1, F = 0.

## 3. But "a construction with one runtime degree of freedom" is not uniformly a type

Section 2's definition is the strongest thing I can derive, and P2 shows it does not land where an
implementation would expect. It says nothing about *what kind of thing* the construction is, and at
the two ends of the range arvo declares, the kinds differ.

At the wide end, a primitive is a **value of a type**. `Dense13` (`fibre.rs:37`) is a `u16` newtype,
`Copy`, `Sized`, 16 bits for 13 bits of content, and an element-level contract is satisfiable by it
(`fibre.rs:75-80`).

At the packed end, a primitive is a **lens into a carrier**. P2/F6: the honest encoding of a
standalone 13-bit value refuses to compile, and `fibre_refuted.err` is the refusal:

```
error[E0080]: evaluation panicked: a packed 13-bit element does not occupy 13 bits as a standalone value
```

The nearest expressible standalone form, `[bool; 13]`, is 104 bits, **8x** the logical width
(`sizes.out`). There is no third option on a byte-addressed target. So the packed element exists only
as `(column, index)`, reached by `Packed13Col::get` (`fibre.rs:53-66`). That is a lens, not a value,
and no `Sized`-bounded contract can be written over it.

**This is not an edge case that can be dropped to make the account uniform.** I17
(`INTENTS.md:374-375`) says "the storage-minimising, aggressively bitpacked path is not
deprioritised", and I6 (`INTENTS.md:123-124`) says that path "aggressively minimises and bitpacks".
Both ends are declared, so an account covering only one is an account of half the library.

The evidence that both ends are real is not my probe. It is in the repository:
`bitpack-footprint-shared/src/lib.rs:92` declares `LOGICAL_BITS = 13`, and the same buffer holds it
twice, dense at `MAX_N * 2` bytes (`:109`) and packed at `(MAX_N * 13) / 8 + 16` bytes (`:105`),
1.2308x apart in footprint (`sizes.out`), with the harness measuring a 6.69x time spread across the
four arms over them (`bitpack-footprint-headtohead_n1048576_findings.md`, 4 variants, 40 samples
each).

So the honest form of section 2's definition is:

> A primitive is a construction with exactly one runtime degree of freedom. **Where that degree of
> freedom is a value it is a type; where it is a position it is a lens over a carrier.** Both are
> saturated at const time and both reach one lowered path; they differ in whether the thing carrying
> the value is the primitive or something the primitive indexes into.

## 4. What a name buys, given that the machine erases it

P1/F4 and P3/F8, two independent instruments, found the same thing. In `sat.s` the linker aliased
`_a_sat_13 = _a2_sat_13_wrap`: two distinct source-level primitives, one machine object. In P3 the
scale version: **forty named primitives, twenty machine bodies**, and the twenty that duplicated an
existing behaviour cost zero additional bodies, folding across family boundaries onto `_d_wrap_13`
(`grid.s:150-188`).

So a name buys nothing observable at runtime and costs nothing at runtime. Under I14
(`INTENTS.md:286`) arvo has removed the mechanisms that could have made the distinction observable
anyway: no `dyn`, no `TypeId`, no `std::any`. **The entire economics of naming is compile-time**, and
the case for or against naming a primitive cannot be made on emitted size except through the
behaviours the names happen to correspond to.

What is left is what a name lets you *state*, and I count three things, each of which I can tie to
governing text rather than to taste:

**A name is where a predicate can be attached.** I13 is the one RATIFIED entry
(`INTENTS.md:214-226`) and it requires every finding to carry the region it holds in. A predicate is
a formula over named dimensions. You cannot predicate an anonymous construction, so **naming is the
precondition for the ratified working method**, not a convenience over it.

**A name is where the perimeter is drawn.** `what-you-can-observe-is-what-you-guaranteed.md`: a
guarantee holds only over the operations through which the type can be observed. An anonymous
construction has no observation surface to quantify over, so it can carry no guarantee. This is what
makes sense-A naming load-bearing even though sense C erases it.

**A name is what makes two occurrences the same occurrence.** Not for code size, which P3 settles,
but for *identity*: two call sites that write the same name are talking about the same thing, and a
law proved once applies at both. Under I16 (`INTENTS.md:317-331`) the canon does not police what
shape a law takes, only that it works; a name is what a law has to be *about*.

## 5. What varies within one, what distinguishes two, and the surprise

Taking the decomposition from sections 2 and 3, and putting a measurement under each:

- **Within one primitive, the value varies and nothing else does.** Section 2, forced by I15.
- **Between two primitives, some const-time-observable thing differs.** That is the natural
  criterion, and P4 shows it is not the same as "some index differs".

P4 measured the map from index to primitive over 128 points, widths 1..64 crossed with
{wrap, clamp}, and found **127 distinct primitives** (`inj.out`). One collapse:

```
    2 indices -> one primitive : [(64, 'clamp'), (64, 'wrap')]
```

At `W = 64` in a u64 the mask is all-ones, so masking is the identity and clamping to the maximum is
the identity. The two indices denote one function, so no lowering could separate them. **"A
primitive is a point of the index space" is therefore false as a definition**, though wrong at only
1 point in 128 here, which is exactly the ratio that makes it dangerous rather than harmless: it will
read as true.

P4b is the part I did not expect. **The degenerate set is not a property of the width. It moves with
the container, and the container is chosen by the treatment** (`inj_container.out`):

```
wide container (always u64),        widths where wrap == clamp : [64]
minimal container (u8/u16/u32/u64), same                       : [8, 16, 32, 64]
```

So whether `(8, wrap)` and `(8, clamp)` are one primitive or two has **no treatment-free answer**.
Under a native-imitating treatment (I3, I4) they are two; under a storage-minimising one (I6) they
are one. The count of primitives is not a fact about the format.

The predicate is nameable and const-checkable, which is the form I13 asks for:

> The overflow policy has no content exactly where `declared_width == container_bits`.

Both arms satisfy that predicate. They disagree only about which widths meet it, because they
disagree about `container_bits`.

**And this is the test gate's own failure one tier up.** `the-test-gate.md` rejects "a constant a
type declares about itself, that no code reads and no check ties to the thing it describes", with the
test "ask what value would make it fail". At `W = 64` in a u64, the overflow-policy index is exactly
that: any value of it yields the same machine body. The suite-level discipline this workspace already
applies has a type-level twin nobody has written down, and P4b says where it bites moves.

Which gives a criterion I would offer rather than assert:

> A parameter earns its place in a primitive's index only where changing it changes something
> observable at compile time, and the region where it does is part of what the primitive declares.

## 6. What the tiers above need, and the arity they need it at

I11 (`INTENTS.md:190-197`) names the tiers: "the algo crates that hilavitkutin, vehje, pretty much
every single repo and project I have, downstream, use. As well as the contracts for things that
compose to bigger units than just numerals alone."

P2/F7 measured what such a contract can be written over. `fibre.rs` carries two signatures. The
element-level one (`:70-73`) requires `Copy`, hence `Sized`, and by F6 is unsatisfiable at the
packed end. The column-level one (`:120-124`) is satisfied by **both** instances (`:126-134` packed,
`:136-144` dense), and one algorithm written against it (`algo_sum_col`, `:148-157`) accepts both
without naming either.

So the answer to "what do the tiers above need from a primitive" is not a property of the primitive.
It is an arity:

> The tiers above need the primitive to be an **instance of a signature they can be written against,
> rather than a member of a list they must enumerate**. The widest arity at which a single signature
> covers arvo's declared range is the column, not the element.

That gives a sharp test for whether something has earned the name, and I like it because it is
falsifiable by inspection of a consumer rather than by argument:

> A construction is primitive when an algorithm crate can be written against it **without naming
> it**. If `arvo-graph` has to match on which primitive it received, that thing is not a primitive,
> it is a case.

This does not say arvo should have no element-level contract. It says an element-level contract is a
statement about a sub-range of the declared range, and under I13 it has to say which sub-range.

## 7. Is "composition" the right frame? Partly, and the part that fails is the important part

**Right for terms.** A term is a composition of operations, and this is uncontroversial.

**Right for the format.** `I` integer bits and `F` fraction bits genuinely compose as a product: they
are independent and the pair determines the logical value set.

**Wrong for the treatment, and P2/P4 say why.** If the treatment were a component of a product with
the format, then the representation would be a function of the format alone. F5 refutes that with a
shipped, measured case: one logical format, two representations, 1.2308x apart in footprint, 6.69x
apart on the harness. F12 refutes it harder: the *identity relation* on primitives is treatment-
relative too, so the treatment is not even a coordinate that can be projected away.

The frame that fits what I measured is a **fibration**, not a product. The format is a base; the
treatment picks a fibre; a primitive is a point of the total space. Composition is what happens
*inside* a fibre, where terms compose. The fibre does not compose onto the base, it is *chosen over*
it, and that is the tagless-final reading of I9 (`INTENTS.md:172-177`) almost verbatim: "strategies
are the variables that change what the 'correct' answer is". A signature has sorts and operations;
an interpretation gives every operation a denotation; the same term means different things under
different interpretations. I8 (`INTENTS.md:143-149`), that the strategies weigh different
measurements differently, is the statement that the interpretations genuinely differ rather than
being notational variants of one.

So my answer to the brief's question is: **composition is the wrong single frame, and the right
statement uses two words.** Terms compose. Treatments interpret. A canon that says "a primitive
composes a width, a radix point and a strategy" has put three things of two different kinds into one
list, and the consequence is not cosmetic: it licenses reasoning that a strategy can be attached to
or detached from a value, which F5 and F12 both refute.

## 8. Options I am opening, each with what would close it

Per the panel's mode, I am not choosing among these. Each states its discriminator.

**O-A. Primitive is defined at the element and the packed end is a separate concept with its own
name.** Cost: two vocabularies, and I17 makes the packed end first-class so the second vocabulary is
not a footnote. Benefit: the element-level definition stays a type, which is what every consumer
expects and what I14's public-API bullet reads as. **Closed by** answering whether any consumer needs
one signature spanning both; if none does, this is right and cheap.

**O-B. Primitive is defined at the column, and an element is the degenerate one-element case.**
Cost: `Bool` and `USize` become one-element columns, which is unnatural at the point of use and may
fight I3's ergonomics intent. Benefit: one vocabulary over the declared range, and P2/F7 shows the
signature exists and both instances satisfy it. **Closed by** writing the element-facing ergonomics
over the column signature and seeing whether it survives contact with a consumer; if the sugar is
thin, this wins.

**O-C. Primitive is defined as the saturated construction (section 2) and left deliberately silent
about whether it is a type or a lens.** Cost: the canon says less, and two implementers could produce
incompatible things, which is exactly the equivalence test `RULES.md:79-83` uses to reject a canon
sentence. Benefit: it is the only form I could derive that is true across the whole declared range.
**Closed by** the equivalence test: hand section 2's sentence to two designers and see whether their
designs interoperate. I expect they would not, which would close this option negatively, and I have
not run it.

**O-D. The index is deliberately allowed to over-count, so a currently-degenerate point can be split
later by a treatment that gives it content.** This is a real position and P4 does not close it. Cost:
a law proved at one index silently holds at another, and a reader counts that as two confirmations.
**Closed by** deciding whether the canon wants the over-count declared; F13's criterion says an
accidental over-count is a defect and a declared one is a design.

**O-E. "Primitive" is retired as a canon word, and the canon names the three senses separately.**
Cost: it breaks the substitution table every consumer repo's `CLAUDE.md` already carries, and I14 is
written in sense A. Benefit: section 1's ambiguity stops being load-bearing. **Closed by** checking
whether any canon sentence needs to quantify over all three senses at once; if none does, the word
can stay as an informal umbrella with the three named underneath it.

## 9. What I am carrying forward unchanged, and from whom

**Nothing, and that is the point of the dispatch.** This is a phase-one cold derivation; I read no
panel file, so I inherited no framing and carried nothing forward. Count: **0**.

What I did lean on that is not mine: op's intents as quoted in `INTENTS.md`, the workspace rules, and
the bench crates' own doc comments and committed artifacts, all cited inline. The bench crates are
agent output on the presumed-wrong rung and I used them for two things only: what they *measured*
(the footprint arithmetic and the harness findings) and what they *declare about themselves*
(`LOGICAL_BITS = 13`). I did not treat any of their design choices as authority.

## 10. Coverage, bounded honestly

**Read in full:** `INTENTS.md`, `RULES.md`, `mock/Cargo.toml` including its comments,
`rust-toolchain.toml`, `mock/benches/Cargo.toml`, `mock/benches/variants/bitpack-write-contend-shared/src/stress.rs`,
the `#[cfg(test)]` module of `warm-container-shared/src/lib.rs`, and the module documentation of
`bitpack-footprint-shared/src/lib.rs`.

**Grepped or scanned mechanically, not read:** all 123 test bodies in the bench variant tree (via
`154_probes/gate_scan.py`), the `bench.toml` sections for the footprint family, and the findings
headers of the committed harness runs.

**Not opened at all:** the other 88 variant crates' sources, every `.csv` and `.meta.json`, the
`src/main.rs` of the bench binary beyond its line count, and every panel file.

**Which sections would move if something I leaned on were wrong.**

- If `bitpack-footprint-shared`'s two regions do **not** hold the same logical column, F5 and section
  3 both weaken to a claim about two different data rather than two representations. I checked this
  against that crate's own `check_size` (`bitpack-footprint-shared/src/lib.rs:240-274`), which asserts the packed and dense
  regions decode to the same values, and that test passes. Residual risk: I trusted that test rather
  than re-deriving it.
- If identical-code folding is off in whatever configuration arvo actually ships, F8, F9, F10 and
  section 4 change: names would then cost bodies, and the argument that naming is free at runtime
  would need the folding configuration stated as part of its predicate. It is stated in each
  finding's predicate, and I did not sweep configurations.
- If a target exists where a 13-bit standalone value is expressible, F6 and section 3's two-ends
  claim narrow to byte-addressed targets. I tested one target and say so.
- Section 7's fibration reading is the one thing here that is a *frame* rather than a measurement.
  F5 and F12 refute the product reading, which is a real result; that the fibration is the *right*
  replacement is my proposal and is not established. It is the section I would attack first.

**Citation check, run rather than claimed.** `154_probes/citecheck.py` opens every `file:line` in
this document and in the four probe findings and prints what is there. **53 citations, 0 failures
after repair, 4 defects found and fixed:**

- Three citations into `warm-container-shared/src/lib.rs` were **off by roughly 1330 lines**, because
  I read them from a `sed -n '/#\[cfg(test)\]/,$p'` slice and recorded the slice's line numbers as
  the file's. All three resolved to real lines saying something else entirely, which is precisely the
  failure `RULES.md:126-133` describes: "A reference that resolves is not a reference that says what
  you claim."
- One citation was to a bare `stress.rs:N` with no crate prefix, unresolvable by anyone who does not
  already know which of the ninety-four variants I meant.

And a defect in the checker itself, found by the same run and worth more than the citations it
caught. Its first version had a basename fallback: when `bitpack-carrier-shared/src/lib.rs` did not
resolve, it retried on `lib.rs` alone, matched a **different crate's** `lib.rs`, and printed twelve
lines of it as verification. A checker that can silently retarget manufactures exactly the
confidence it exists to remove. The fallback is deleted, with the reason written where it was
(`154_probes/citecheck.py`), and unresolved is now reported as unresolved.

**What I settled:** that I15 entails saturation (section 2), and that this is a derivation from op's
words rather than a preference.

**What I moved:** the question of whether a primitive is a type. P2 makes it a measured no across the
declared range rather than an open matter of taste, and P4b makes "how many primitives are there" a
treatment-relative question rather than a countable one.

**What I could not:** I could not find a single account covering both ends of the range that passes
`RULES.md:79-83`'s equivalence test. Section 2's definition is true across the range and I believe it
is too weak to make two implementations interoperate; O-C records that, with the test that would
settle it, and I did not run that test because it needs a second designer rather than a probe. That
is the honest wall and it is where I would send the next dispatch.

---
---

# Phase two: reconciliation

Phase one above is unchanged and is not to be read as amended by anything below. Where phase two
withdraws a phase-one finding, the withdrawal is here and the original stands there, per
`RULES.md:543-556`.

**Read for this phase.** `AGREEMENTS.md` sections 0 through 2.1; `109` sections 1-7, 11, 13, 16 and its
gate; `110` in full through section 5, plus its gate and phase-two headings; `111` sections 0, 1, 2, 9.4,
9.5; `112` and `114` headings plus `114` section 1 and `112` section 1; op's `113`, `104`, `95`, `88`,
`87`, `85`, `83`; `OPTIONS.md` Q52 in full and its heading index; `DROPLIST.md` headings; `151` sections
0 and 1.

**Not read, and named because it bounds what I can claim:** `63`, `74` and `90` beyond what
`AGREEMENTS.md` says about them; `106`; every file between `115` and `153` except `116`'s gate section
reached by grep; the bodies of `112` and `114` past their stated answers; `PRIOR_CALLS.md`,
`PERSONA_CALLS.md`, `HANDLES.md`, `seed/` and `archive/`. Any claim of mine about novelty below is
bounded by grep over the panel root rather than by reading, and I say so at each.

## P2.0 Shared inputs, and what my conclusions ran through

The brief asks for this and it is the deliverable rather than a demerit.

**`what-you-can-observe-is-what-you-guaranteed.md` auto-loads, and section 4's second answer runs
straight through it.** "A name is where the perimeter is drawn" is that rule's thesis applied to
primitives. I did not derive it; I applied it. Discount accordingly: on that point I am one instance
of that rule, not an independent arrival.

**`the-test-gate.md` auto-loads, and the withdrawn F13 ran through it.** "A declaration nothing
constrains" is its wording, and my criterion was that sentence moved one tier up. It is withdrawn for
other reasons (P5), but it was never independent of the rule either.

**`a-refused-bound-wants-a-trait-not-a-feature.md` auto-loads, and P2's repair is its move.** When the
element-level signature refused, I climbed to a column-level trait. That is the rule's "break the
constraint into smaller pieces... then compose the bound out of them", and I reached for it because it
was in context.

**`every-finding-carries-its-predicate.md` auto-loads** and shapes every predicate line I wrote.

**One shared input I did *not* have, which matters because `110` did.** `110` declares contamination
from `arvo-always-optimal-internals.md`, which auto-loads into its context and already carries a claim
about when a law holds. **That rule is not in my loaded rule set.** I checked my own context rather than
assuming. So where `110` had to discount itself, I have no exposure, and equally I derived nothing about
law-holding, so the difference buys nothing here beyond being worth recording: the auto-loaded rule set
is **not identical across members**, which is a fact about this panel's independence accounting that I
have not seen stated.

**And the premises themselves.** `INTENTS.md` and `RULES.md` are shared with `109` and `110` by
construction. My section 2's entailment runs through I15, which is a shared premise; the *entailment* and
its assembly are mine, the premise is not.

## P2.1 What I withdraw

**F11, F12 and F13 are withdrawn.** Full account in `154_probes/p5_signature/FINDINGS.md`, built after
reading `110` section 3 and committed with its output.

P4 reported 128 index points naming 127 primitives, collapsing at `W = 64`, and P4b reported the
degenerate set moving with the container. Both measurements are correct and both are facts about **my
signature**, which was a single arity-1 operation applied to an already-in-range value. On such an input
the realisation map is never asked about an out-of-range exact result, so the overflow policy is
unreachable, not absent. Adding `add`:

```
widths where wrap == clamp, signature = arity-1 mask (what P4 used) : [64]
widths where wrap == clamp, signature = arity-2 add                 : []
widths where R itself does not read the policy (whole-line test)    : []
```

`110`'s distinction is the right one and I concede to it outright: only a **definitional** degeneracy,
where the definition of the map stops reading the axis, may be canonicalised away; a **reachability**
degeneracy is a fact about the operation set and evaporates when it grows. My collapse was the second
kind at the thinnest possible signature, and there is no definitional degeneracy anywhere in the grid I
swept.

**And this is at least the fourth instance of one failure mode, which is the part worth keeping.**
`110`'s P4 was falsified by assuming a structural degeneracy at `F = 0`. `110`'s P8 first run swept no
rounding modes, making a weak criterion look sound. `111` section 9.4 kept an under-controlled arm
"because wrapping at the container width is the bare add, so three of its four symbols aliased for a
reason having nothing to do with the bound" (`111:676-679`). And P4 here. Four authors, four
instruments, one mistake: **a criterion tested against a signature too thin to reach the case it fails
on.** `111`'s wording of its own instance is almost exactly the mechanism of mine, which I find more
persuasive than any of the four instances alone.

**My phase-one test-gate count of 124 is withdrawn** in favour of 123. The 124th is the string
`#[test]` inside a doc comment at `bitpack-write-contend-shared/src/stress.rs:68`. My loose
`grep -rn "#\[test\]"` counted it; my own parser returned 123 and I reported both without reconciling
them, which was sloppy. `110` found this first and corrected itself the same way.

**My phase-one reading of 115.43s for `wide-rung-shared` is withdrawn as contaminated**: it was taken
while the hung `bitpack-write-contend-shared` run was at 575% CPU on a four-performance-core host.

## P2.2 Where I agree, having derived blind, and what rung that earns

**F5, that one logical format has two shipped representations: second instance, independent.** `109`'s
P1 direction A holds the value set fixed and realises it twice, per element in a `u16` and as a dense
13-bit stream, and reports the footprints differing at 13 bits against 16. I reached the same
separation from the other end: not by constructing two realisations but by reading the two that
`bitpack-footprint-shared` already ships in one buffer, at the same `LOGICAL_BITS = 13`
(`bitpack-footprint-shared/src/lib.rs:92`), and pricing them from the committed harness run. Different
instrument, same conclusion, derived before reading `109`. **This is what the TWO EXPERTS rung is for**,
and I claim it for this one claim only.

**That `V` and the realisation are separable at all: agreement, and I go no further.** `109` section 2
establishes both directions; I established one. I do not claim the second.

**`109`'s const-availability criterion: I derived its ground independently and I think I strengthened
it.** `109` section 11 offers "a property belongs in the primitive iff it must be const-available in
order to decide whether a program is valid, or to select a lowering." My section 2 reaches the same
place from I15, and adds what I believe `109` does not have: **a demonstration that the "must" is
compelled rather than chosen.** `sat.s:31-39` shows a `cmp w8, #63` appearing in the emitted code the
moment a width is left runtime, which is the check I15 forbids in as many words. Grep over the panel
root for `runtime width`, `width in the value`, `width at runtime`, `csinv` and `variable shift`
returns nothing outside my own file, so I believe this direction is new; that is a grep, not a reading,
and `109`'s own probe set could contain it under other words.

**`110`'s realisation map as one map with two regions: I did not derive it and I accept it.** My P4
treated overflow policy as an axis beside the width, which is the two-mechanism shape `110` argues
against, and P5 is what happens to somebody who does that.

**Carried forward unchanged, with the count.** Four things, from three members:

1. `110`'s definitional-versus-reachability distinction, and its whole-line test for it. Carried, and
   it is what refutes my own P4.
2. `110`'s realisation map as one map, overflow and rounding being two regions of it.
3. `109`'s const-availability membership criterion, carried and, I claim, strengthened rather than
   amended.
4. `111`'s point that the law set cannot vary with the other coordinates held fixed. I did not test it
   and I have no independent instance; I am carrying it on `111`'s and `OPTIONS.md` Q52's account, which
   also warns not to cite `110`'s 0-of-48 count. I do not cite it.

Count: **4 carried, 0 amended, 3 of my own withdrawn.**

## P2.3 What of mine appears to be new, bounded by grep rather than by reading

**F6: at the packed end a primitive is not a type, and the refusal compiles into evidence.** Grep for
`no standalone size`, `not a Rust type`, `is a lens`, `Sized` across the panel root returns nothing on
this point outside my file; `109` section 13's alternative D ("Nobody computes on 13 bits; they widen")
is the closest and is a different claim, about the working form rather than about expressibility.
`fibre_refuted.err` is the compile failure, and `[bool; 13]` costs 104 bits, 8x the logical width. This
matters for exactly one reason: **a canon sentence saying a primitive is a type is false over the range
I17 forbids trading away**, and no amount of care about the algebra fixes it, because it is a fact about
what Rust can give a size to.

**F14's soundness half.** `110` found the hang first and diagnosed it better, with a stack sample; mine
is a second instance by isolation (each test alone: 0.31s, 1.86s, 0.59s; together: no completion in
180s, reproduced twice). What I have not seen anywhere, and grep for `use-after-free` and `use after
free` across the panel returns nothing: **the pool bug is not only a liveness bug.** Workers load
`vals` and `out` as raw pointers from shared fields (`bitpack-write-contend-shared/src/pool.rs:110-111`), and under two concurrent
coordinators a worker can write through a pointer to a per-trial buffer whose trial has already
returned (`bitpack-write-contend-shared/src/stress.rs:42-44`). Reachable from `cargo test`. And the consequence for the suite is worse
than slowness: the aligned-split control exists to show the observed corruption **is** the boundary
race, and cross-test pointer mixing would defeat exactly that isolation.

**F15, and it un-retires a register entry.** `OPTIONS.md` Q52 ends: *"Retired: the claim that
`wide-rung-shared` takes 107s. Three measurements now put it at 4.05s, 4.25s and under 5s. Dropped
rather than carried as contested."* **That retirement is wrong and should be reversed.** The three
measurements that killed it were taken at `--release`: `110`'s transcript shows `cargo test --release`,
and `111` F111-13 carries `--release, --test-threads=1` in its own predicate (`111:868`). Measured back
to back on one host in one session:

```
cargo test  ... -- --test-threads=1      30 passed, finished in 109.08s   # debug, cargo's default
cargo test --release ...                 30 passed, finished in   3.78s   # release
```

**29x.** The 107s figure was a correct debug measurement, and three correct release measurements were
used to retire it. Nothing was contested; a dimension was missing from all four. Under I13 as op states
it, an unlisted dimension is not a hedge, and `every-finding-carries-its-predicate.md` would have caught
this the moment anyone wrote `profile = release` into a predicate. Nobody did, including `111`, whose
F111-13 does carry `--release` and whose section 0.2 nonetheless calls 107s a third measurement of the
same thing.

The general form is the finding rather than the number: **`cargo test` defaults to debug, every
measurement this panel trusted was taken at `--release`, and a bare wall-clock figure for a suite is
meaningless without its profile.**

**F16 is not new.** `114:79` and `116:54` both record that the variants are path dependencies rather
than workspace members. Mine is a third instance and I withdraw any claim of novelty.

**F4 and F8 are not new either.** `111` section 9.4 does the symbol-merge measurement, with a better
control than mine, and records the same under-controlled-arm defect. My P1 and P3 are second and third
instances, arrived at blind, and I claim only that.

## P2.4 Where I hold against what I have read

**I hold section 3's two-ends claim against `110`'s "a primitive is a finite algebra over a declared
signature".** That answer is better than mine on identity and I concede identity to it entirely. But an
algebra is a set with operations, and the packed end has no set of standalone values to be the carrier
of one. `110`'s answer is a semantics; F6 says the semantics has no uniform *realisation* as a Rust
type over the range arvo declares. Those are compatible, and a canon that states only the semantics has
said something true and left the implementer to discover the discontinuity. `111` section 8 already
names something close ("one is a semantics and the other is a decision procedure, and nothing states
what they owe each other"); F6 is a third thing under that same gap, on the representation side, and I
do not think it is covered.

**I hold section 6's arity claim, tentatively.** The widest arity at which one signature covers arvo's
declared range is the column, not the element. This is a claim about expressibility rather than about
algebra, it follows from F6, and I have one instrument for it. It wants a second.

## P2.5 What I would hand whoever writes the canon text on this topic

Four sentences, offered rather than settled, each with what it rests on:

1. **A primitive has exactly one runtime degree of freedom, its value; every other parameter is
   resolved at const time.** Not a preference: I15 entails it, and a violation produces a check I15
   forbids. Rests on P1, which is mine and blind. Strengthens `109` section 11's criterion by supplying
   the compulsion behind its "must".
2. **A primitive's identity is its algebra up to denotation-preserving isomorphism, relative to a
   declared signature.** `110`'s, carried unchanged, and my P5 is a fourth instance of what goes wrong
   without the "relative to a declared signature" clause.
3. **An axis may be canonicalised away only where the realisation map's definition stops reading it,
   never where the current signature merely cannot reach it.** `110`'s, carried unchanged, and it is
   what refutes my own P4.
4. **Where the value is a position rather than a datum, the primitive is a view over a carrier and has
   no standalone size.** Mine, from F6, and the one I would most like attacked. If it is wrong, the
   canon can say a primitive is a type and mean it; if it is right, any sentence that says so is false
   over the storage-minimising path I17 protects.

## P2.6 Coverage of phase two, bounded

I read the curated list and stopped there. I did not read `115` through `153` except by grep, so my
novelty claims are grep-bounded and any of them could be a rediscovery under different words; I have
said which at each. I did not read `63`, `74`, `90` or `106` and took `AGREEMENTS.md`'s account of them,
which means anything I say touching the format or strategy consolidations rests on a compression I did
not check, and **`RULES.md:135-148` says the next dispatch depending on that material should read the
source rather than the account.** I am the second file to lean on `AGREEMENTS.md` this way that I know
of, and I name it here so the exposure is countable.

The sections that would move if I am wrong about something I leaned on: P2.3's F15 is the one I am most
confident in, because I measured both profiles myself on one host in one session and the two other
members' own predicates name `--release`. P2.2's TWO EXPERTS claim for F5 depends on `109`'s P1 direction
A saying what `109`'s prose says it says; I read the prose and its predicate block, not the probe
source. P2.4's hold against `110` depends on my reading of "finite algebra" as requiring a carrier set
of standalone values, which is standard but is my reading of its words rather than a claim `110` makes
about representability.

**What I settled in phase two:** that the 107s retirement in `OPTIONS.md` Q52 is a profile difference
and should be reversed, with the A/B on one host.

**What I moved:** the pool defect, from a liveness bug to a soundness one, with the pointer path named.

**What I could not:** I still cannot produce a single account of a primitive that covers both ends of
the declared range and passes `RULES.md:79-83`'s equivalence test. `110`'s algebra covers the semantics
and F6 says it does not determine the realisation; my saturation definition covers both and says too
little to make two implementations interoperate. Section 8's O-C records that with the test that would
settle it, and the test needs a second designer rather than a probe. That is the wall, it is where I
would send the next dispatch, and I would send it with F6 in hand as the thing to break.

## P2.7 Unlicensed mechanisms, reported whether or not they fall in my question

The standing instruction is to report these even outside the question asked, and not to soften them.
What I found, and what I checked and did not find.

**One register defect, and it is the sharpest thing here.** `OPTIONS.md` Q52's closing line retires a
true measurement. It is not a judgement call that went the wrong way: three measurements taken at
`--release` were used to retire a fourth taken at the default profile, and the two members whose
figures did the retiring both carry `--release` in their own predicates. **The mechanism that would
have caught it is the ratified one.** I13 requires the region a finding holds in; a profile is a
dimension; nobody listed it; and the register then destroyed a correct finding on the strength of
measurements of a different thing. That is the predicate discipline not being applied to the panel's
own bookkeeping, and it cost a true claim.

**One test that cannot fail, in the default run, costing thousands of threaded trials.**
`bitpack-write-contend-shared/src/stress.rs:96-111`. Its author's reasoning for asserting nothing is
correct and better than an arbitrary threshold; the placement is not. A `#[test]` that cannot fail is
a diagnostic, and leaving it in the default run puts it into every coverage count that gets cited and
makes every runner pay for a number read off stderr. One `#[ignore = "diagnostic: ..."]` fixes it.
`110` praised this test's restraint and I agree with the praise; the restraint is right and the
attribute is missing.

**One soundness bug reachable from `cargo test`**, stated in P2.3 and in
`154_probes/p6_testgate/FINDINGS.md`. `110` found the hang; the pointer path appears to be unreported.

**Checked and not found: an intent violation in the bench tree.** The variant crates use `std`
freely, including `std::vec::Vec`, `std::thread`, `Box::leak` and `println!`, which I14 forbids for
arvo. **This is licensed and I want to be explicit rather than leave a reader to wonder**:
`mock/Cargo.toml:32` excludes `benches` from the workspace with a comment saying bench crates "are not
governed by the canon/design/code chain the empty member list protects", and a harness that measures a
no-std library is not itself the library. I looked for a bench arm whose *measured kernel* depends on
`std` in a way that would make the measurement inapplicable to a no-std consumer, and did not find one;
the `std` use is in input construction, the thread pool and reporting. I did not audit all
ninety-four, so this is bounded at the crates named in this file.

**Checked, and my first statement of it was wrong.** I wrote that
`grep -rn '^#!\[feature' mock/benches/variants/ --include='*.rs'` returns nothing, then ran it, and it
returns **two**:

```
variants/spectral-bisection/src/lib.rs:21:#![feature(adt_const_params)]
variants/spectral-bisection/src/main.rs:20:#![feature(adt_const_params)]
```

**Neither is a violation.** `unstable-features.md` lists `adt_const_params` (#95174) as ALLOWED,
"largely complete", a 2026 const-generics stabilisation target, with the unsound unsized/borrow part
split out into `unsized_const_params`. And `spectral-bisection` is one of the two crates
`mock/benches/Cargo.toml`'s own header says are deliberately absent from the binary's dependency list
because they need the deleted `arvo` crate, so it does not build at all in the current tree.

I am leaving the error in rather than editing it away, because it is the third time in this dispatch
that writing a count before running it produced a false one, after the 124-versus-123 test count and
the contaminated 115.43s. **A count is a measurement** (`RULES.md:124`), and "returns nothing" is a
count. My own probes do carry none, by the same grep, and that one I ran first.
