# 17. What would actually certify this design, and what is merely being trusted

**Date:** 2026-08-08. **Register:** breadth pass. Nothing here settles. **Dispatch:** separate what
the panel has proved from what it is assuming, design the instrument that is missing, and apply the
result backwards to the panel's own claims.

I did not run `git log` in this repository before my answer was on disk. `RULES.md:193-201` records
why, and it cost `16` the provenance of its headline count.

## The distinction this file is about

A verification claim is worth exactly what its statement says, and most confusion in this field comes
from statements whose perimeter was never written down. The acceptance criterion is a good statement.
What has grown around it is a set of instruments, each of which establishes something narrower than
the clause it is filed under, and the difference between the clause and the instrument is the part
nobody has enumerated.

So this file does one thing four times. For each clause: what would an instrument have to establish,
what do the panel's instruments establish, and what sits between the two. The part in between is
being trusted. That is not a criticism, since every verified artifact rests on a trusted base and the
useful question is only whether the base is named and small. This panel's base is neither, yet.

Three words are kept apart throughout, because they are not synonyms. **Proved** means an instrument
establishes it over the whole domain the claim is quantified over. **Validated** means an instrument
checks each instance as it arises, which is weaker in scope and can be exactly as strong in force.
**Trusted** means nothing checks it and the design proceeds as though it held.

## 0. The premises I was handed, checked before I reasoned from them

The brief asserts two blindness results from `16` and instructs me to verify rather than accept them.
Both reproduce, and one of them I can state more strongly than `16` did.

**The certifying instrument is per-value by construction.** Reproduced.
`16_probes/p3_blind_suite.rs`, rebuilt on the pin:

```
rustc +nightly-2026-05-28 --edition 2021 -O p3_blind_suite.rs -o bin/p3 && ./bin/p3
```

Four of four green against a carrier-only derivation of `UFixed<13,0,Cold>`, with the array extent
2000 bytes against the 1625 the declaration promised, 23.1% over.

One correction to how that probe should be cited. Its `check_codegen_equality` is a **runtime value
comparison, not a codegen comparison**, and the probe says so itself in a comment at
`16_probes/p3_blind_suite.rs:89-93`. So `p3` does not actually demonstrate that the panel's erasure
check is blind; it demonstrates that a stand-in for it is. `16` is honest about this and the
distinction matters for anyone citing it, so I ran the real instrument instead.

**The real instrument, run independently.** `15_probes/q12_erasure_asm.rs`:

```
rustc +nightly-2026-05-28 --edition 2024 -O --emit asm --crate-type lib \
  q12_erasure_asm.rs --out-dir asm
```

The assembler's own output is three folded symbols:

```
_q12_arvo_hot   = _q12_arvo_cold
_q12_native_i16 = _q12_arvo_signed
_q12_native_u16 = _q12_arvo_cold
```

The premise holds, and the sharper statement is available from that output directly. The instrument
does not merely fail to observe the second output. **Its green result is the assertion that `Hot` and
`Cold` are the same function.** That assertion is correct at the scalar, and it is exactly the
collapse a carrier-only derivation would produce, so the instrument cannot distinguish a two-output
derivation that has correctly collapsed at the scalar from a one-output derivation that never had
anything to collapse. Blindness understates it: the instrument's output, read literally, is the
symptom.

**The catching check is data-dependent.** Reproduced. `16_probes/p4_access_width.rs`:

```
values 0..64 (a small-value test)  -> 0 of 64 wrong with a 2-byte access
values with the top bit set        -> 32 of 64 wrong with a 2-byte access
```

Both premises stand. The rest of this file goes past them.

## 1. The criterion, restated as four quantified statements

Op's words, at `SETTLED.md:67-71`, establishing source `seed/SETTLED_container.md:34-37`:

> There *is* a way to express usage through bits and bytes *and* have the typestate derive the
> matching container and numeral representations, then validate, and erase on lowering to be exactly
> what you describe before that caveat.
>
> Anything less than that, no caveats left, is unacceptable for this design and canon.

The clause I want to draw attention to first is not one of the four. It is **"no caveats left"**.
That phrase makes the criterion a statement about the *absence of a residue*, which is a much harder
thing to certify than any of the four positive clauses, because it quantifies over things nobody has
thought of. A design cannot prove it has no caveats. It can only enumerate the ones it knows and
name what remains unexamined. That is what a trusted-base list is for, and it is why the deliverable
below is a list rather than a verdict.

Now the four, each written with its quantifier explicit, because the quantifier is what decides which
instrument could possibly settle it.

**C1.** For every usage a consumer has, there is a declaration in bits and bytes expressing it, and
there is no path by which a consumer expresses a container instead.

**C2.** For every declaration, the typestate derives outputs that match the declaration.

**C3.** For every declaration outside the admissible set, the typestate refuses it, and for every
declaration inside it, the typestate does not.

**C4.** For every program written against the derived surface, the lowering is the lowering of the
program the consumer described.

Three of the four quantify over an infinite or unbounded set. C2 is the exception: the declaration
space is bounded by the ladder, so C2 is the only clause an exhaustive instrument can close. That
asymmetry decides most of what follows.

## 2. The proved, validated and trusted split, clause by clause

The table first, then each row argued.

| Clause | An instrument would have to establish | The panel's instruments establish | Rung |
|---|---|---|---|
| C1 expressibility | every usage is expressible | a chosen catalogue of usages compiles | trusted |
| C1 no-container-path | no observation admits a container | nothing, until section 3.4 | trusted, now instrumented |
| C2 derivation | outputs match a spec, over the domain | outputs exist, and agree with each other | validated in part |
| C3 validation | both refusal directions, against a spec | usage-mismatch diagnostics only | trusted, and ambiguous |
| C4 layout erasure | typed layout equals described layout | by construction, unmeasured | proved by construction |
| C4 dispatch erasure | no runtime dispatch on the typestate | by the forbidden-feature list | proved by construction |
| C4 operation erasure | every program lowers as described | one program, now widened | validated at one point |

### C1, and the half of it nobody has instrumented

The expressibility half is not certifiable and never will be. "Usage" is not an enumerable set, so
any instrument works against a catalogue, and the catalogue is chosen by the people who built the
surface. That is the setup-that-helps shape in its purest form, and there is no repair for it other
than saying plainly that the claim is a coverage statement over a chosen list. The ergonomics bar at
`SETTLED.md:109` is op's own and is a human judgment about "easy and intuitive to write", which no
probe can evaluate. It belongs on the trusted list permanently and without embarrassment.

The other half is different, and it is missing. **C1 says the consumer expresses usage in bits and
bytes. The ratified companion at `SETTLED.md:93` says the container is never written by a consumer.
Together those are a perimeter claim, and a perimeter claim is decidable.**

The question is: over the whole public surface, is there any observation through which a container
enters or leaves? Every public constructor, every field, every accessor, every `From` and `Into`,
every `Deref`, every iterator item, every associated type a consumer can name. If any one of them is
container-shaped, the guarantee holds only up to that hole, whatever the declaration syntax looks
like. A property holds over the operations through which its type can be observed, and no further.

Nothing in this panel enumerates that surface. It is the single clearest gap I found in C1, it is
cheap, and section 3.4 builds it rather than leaving it named.

### C2, where the existing evidence is consistency rather than correctness

The panel has three kinds of layout evidence and none of them is a differential against an
independent specification.

`15_probes/q13_cold_packed.rs` computes a stride from the derivation and compares it against
`8 * size_of(container)`, which is a consistency condition **between the derivation's two outputs**.
It is a good check and it found two real defects, per `15:345-349` and `15:351-354`. It cannot find a
defect the two outputs share.

`16_probes/p1_fibre_count.rs` counts fibres, which is a structural fact about the map and not a
correctness claim about any particular declaration.

`16_probes/p3_blind_suite.rs:105-114` names the third kind and disqualifies it in the same breath:
`size_of::<Num>() == 2` asks the derivation to agree with itself, because the newtype is the ladder's
output. It is structurally incapable of failing.

So C2's rung is *validated in part*: each derivation instance that has been looked at agrees with the
other output it produced. Nothing has compared either output against a rule written from the intent.
That is the instrument I built, and it is section 3.

### C3, which is the least instrumented clause and is also ambiguous

Two problems, and the second is worse.

**The instrument problem.** A validation claim is quantified over the complement of the accepted set,
and a suite drawn from the accepted set carries no information about it. Worse, the claim has two
independent error directions, and the two are observed by different suite shapes:

- **Under-refusal**, accepting a declaration the design cannot serve. A compile-fail suite observes
  this.
- **Over-refusal**, refusing a declaration the design should serve. A compile-fail suite is blind to
  it, and so is every test written from declarations somebody thought of.

Over-refusal is the one with no natural author. A consumer who never wrote `UFixed<34, 0>` never
files a bug, and a suite of declarations somebody wrote stays green while a band of the range is
quietly unreachable. `t4_validation_directions.py` demonstrates both directions being separately
missed; the numbers are in section 3.

**The ambiguity problem, which I think is the more useful finding.** Op's clause is two words, "then
validate", and the panel has never said what they mean. At least three readings are available and
each demands a different instrument:

1. **Admissibility.** The typestate refuses declarations it cannot serve. Instrument: the
   two-directional sweep above. Panel evidence: none.
2. **Usage.** The typestate refuses operations that violate the declared invariants, with a
   diagnostic a consumer can read. Instrument: a diagnostic battery. Panel evidence: real and
   substantial, `12_probes/p11_diag_battery.rs`, `12_probes/p12_first_day_errors.rs`,
   `15_probes/q10_diag_tag.rs`.
3. **Self-validation.** The derived container actually holds the declared range, checked at
   derivation time rather than assumed. Instrument: a range assertion per declaration. Panel
   evidence: incidental.

I checked before asserting this. Fifteen probes in the panel declare themselves expected-to-fail:

```
grep -rlniE 'EXPECTED TO FAIL|does not compile, on purpose|MUST NOT COMPILE' */*.rs | wc -l
  -> 15
```

Reading their headers, they are about feature refusals and about **usage-mismatch** diagnostics.
`12_probes/p11_diag_battery.rs:4-5` states its subject as "return a 26.6 numeral where a 13.3 was
declared", which is reading 2. Not one of the fifteen is about whether a declaration is admissible,
which is reading 1.

So the panel has good evidence for one reading of a clause and no evidence for another, and has never
recorded which reading it is answering. That is a question for op rather than a defect to fix, and it
is sharper than "validation is uncertified".

### C4, which decomposes into three parts with three different rungs

This is the part I would most like the canon to carry, because it moves most of the clause off the
measured pile and onto the by-construction pile, which is the direction that makes a correctness
argument finite.

**Layout erasure.** The typed value occupies what the container occupies. This is guaranteed by
`repr(transparent)`, which is a language guarantee rather than an optimisation. It holds at every
optimisation level, on every toolchain, forever, and it needs no codegen inspection at all. It can be
asserted as a const assertion for every declaration in the domain, cheaply and totally. **Proved by
construction.**

**Dispatch erasure.** No runtime branch selects on the strategy or the width. This is guaranteed by
monomorphisation, and monomorphisation is guaranteed to be the whole story by the ban on `dyn` and
`TypeId`. The forbidden-feature list is doing certification work here, not hygiene. **Proved by
construction, conditional on the ban list holding.** That makes the ban list part of the trusted base
and it belongs on the written list as such.

**Operation erasure.** The body of a derived operation lowers to the body of the described operation.
This is the only part that requires looking at emitted code, and it is the only part the panel has
measured. **Validated at one point.**

Naming that split is worth more than any single measurement, because it says which part of clause
four survives a compiler upgrade and which part has to be re-checked after one. The first two do.
The third does not, and section 4 measures how much it does not.

## 3. The instrument that is missing, and what I built of it

Two instruments are missing, one per remaining clause. I built both far enough to show the shape
works and to produce results, and neither is finished.

Everything is under `17_probes/`. Nothing is committed, per the dispatch. Every number below carries
the command that produced it, and `17_probes/verify.sh` rebuilds and reruns the lot from source on the
pin in one command. There are no feature gates anywhere in the directory, which `verify.sh` checks by
grep and reports. **No bench harness has run in this panel**, so nothing here is a
timing, and every magnitude question about what any of this costs is unpriced.

### 3.1 A layout conformance oracle, and why its shape is forced

Four properties are forced by what went wrong, rather than chosen.

**The specification must be written from the intent, not from the mechanism.** This is the
non-negotiable one. `16`'s `size_of` check fails only because it compares the derivation against
itself. So the oracle is a bit-at-a-time reference: element `k` occupies bits `[kW, kW + W)`, written
and read one bit at a time, which is the specification restated in the most obvious form available.
The thing under test is the word-load form an implementer would write. The oracle is dumb and the
subject is clever, which is the correct way round.

**It must see across an aggregate**, because the information the carrier loses is aggregate
information and no per-value check can recover it.

**It must be adversarial about data.** `16` established one requirement here. Working through the
failure modes produced four more, and three of them are not in the panel.

**It must be adversarial about procedure**, which is the requirement I did not expect and which is
the sharper half of the data point.

The instrument is a defect matrix: seed a packed accessor with one plausible implementation defect at
a time, run every candidate test procedure against each, and record which procedures observe which
defects. A procedure whose row is empty certifies nothing about layout, whatever it looks like on the
page.

```
rustc +nightly-2026-05-28 --edition 2021 -O t1_defect_matrix.rs -o bin/t1 && ./bin/t1
```

Six defects, eight procedures, swept over `W = 1..=48` in both sign domains. The per-declaration
matrix is in `17_probes/t1_defect_matrix.out`. The summary:

| Procedure | Defect instances observed, of 528 |
|---|---|
| P1 carrier round trip | 0 |
| P2 `size_of` against the ladder | 0 |
| P3 run, ascending, small values | 174 |
| P4 run, ascending, width-filling values | 236 |
| P5 one write into a poisoned buffer | 96 |
| P6 aggregate extent equals `N * W` | 84 |
| P7 values spanning the sign domain | 144 |
| P8 tail element of an exactly sized run | 216 |

The two procedures that observe nothing are the two a person writes first, and both are in `16`'s
green-and-blind suite. That is the result reproduced from the other direction: not "these pass
against a broken derivation" but "these cannot fail against anything".

The union matters more than the rows. The suite a person writes without already believing in a second
output, P1 plus P2 plus P3, observes **174 of 528**. All eight together observe **420**.

The remaining 108 are the interesting number, so I made the probe answer it rather than leaving it as
a gap. A defect that is not observable at a declaration cannot be missed there: `StrideRoundedToBytes`
is not a defect at `W = 8`, because the stride is already byte-aligned. So the probe now computes, for
each pair, whether the defected accessor differs from the specification on **any** index, value or
poison pattern it is exercised with:

```
  total defect instances          : 528
  observed by the suite           : 420
  vacuous at that declaration     : 108
  OBSERVABLE AND MISSED           : 0
```

So the eight-procedure suite is complete over this defect family at these declarations, and the naive
three-procedure suite observes 174 of the 420 that were there to observe, which is 41%.

**The perimeter of that completeness, stated plainly.** It is complete over the defects I chose. The
columns of the matrix are as good as my imagination and no better, and no instrument of this shape
can do otherwise. What the probe establishes is a lower bound on what the naive suite misses and an
existence proof that a procedure set closing this family exists. It does not establish that the
family is all there is.

### 3.2 The four adversarial requirements, three of which are new

**D1, values must fill the declared width.** `16`'s finding, reproduced. A too-narrow access returns
the right answer whenever the truncated bits were zero.

There is a wrinkle underneath it that I hit by accident and that is worth recording, because it
explains why a careful person still ends up blind. A plain `0..n` counter **overflows the declared
range** at narrow widths: at `W = 5` signed the range is -16 to 15, and the value 20 is not
representable, so the test fails against a correct implementation. The natural repair is to bring the
counter into range. That repair is exactly what removes the high bits. **The blindness in the P3 row
is not carelessness. It is what a careful person gets when they fix the obvious problem with the
obvious data.**

**D2, the poison pattern must be varied.** New. A single write into a buffer pre-filled with `0xFF`
hides a write that spills a one bit; a buffer pre-filled with `0x00` hides a write that spills a zero
bit. P5 catches the spill defect only because it runs both patterns. With one pattern it drops to
roughly half.

**D3, the write order must not repair the damage.** New, and it is the procedural half. A writer that
spills one bit into the next element is **invisible** to a test that writes the whole run in ascending
index order and then reads it back, because element `k+1`'s own write overwrites the spilled bit
before anything reads it. Only the final element's spill survives. Confirmed in the matrix: P3 and P4,
both of which write the whole run ascending, observe the spill defect at zero declarations, while P5,
which writes one element into a poisoned buffer, observes it at 96. **A correct data choice with the
wrong procedure observes nothing.**

**D4, the data must span the sign domain.** New here, and it is the hole `16:545-549` named as its
largest. P7 observes 144 defect instances, and it is the only procedure that observes the missing
sign extension at every signed declaration. An unsigned packed read is a shift and a mask; a signed
one needs the sign bit replicated from an arbitrary bit position, and a test whose values are all
non-negative cannot tell the two apart.

**D5, the tail element is a separate case, and it is a memory-safety case rather than a correctness
one.** New, and it fell out of the first run as a control failure I could not explain away.

The obvious word-load packed read fetches `floor((W+6)/8) + 1` bytes starting at the element's byte.
For the last element of an **exactly sized** run, that window extends past the buffer. At `W = 13`,
`N = 1000`, the buffer is 1625 bytes, the last element starts at bit 12987, byte 1623, and a 3-byte
access reads bytes 1623, 1624 and 1625, of which the last does not exist.

This is not an injectable defect. It is a property of the technique, and my first version of the probe
had it, which is why the control was failing. A correct implementation must either clamp the access at
the buffer end or over-allocate by the access width, and both are real choices with different costs.
`16_probes/p3_blind_suite.rs:163` papers over it with `.unwrap_or(&0)`, which is right for a probe and
is exactly the line that would hide it in shipping code.

**This belongs in the canon as a relation, not left to be rediscovered**: a packed run's allocation is
not `ceil(N*W/8)` bytes if elements are read by word load. It is that plus the access slack, or the
tail is a special case.

### 3.3 A validation oracle, and the two directions

```
python3 t4_validation_directions.py
```

Domain `W in 0..=69`, 70 declarations, each compiled separately on the pin. The admissibility rule is
written independently of the typestate: admissible iff `1 <= W <= 64`. Five configurations, one
validation defect each.

| Defect seeded | Under-refusals | Over-refusals | Observed by |
|---|---|---|---|
| none (control) | 0 | 0 | nothing |
| admits `W = 0` | 1 | 0 | a compile-fail suite |
| admits `W = 65, 66` | 2 | 0 | a compile-fail suite |
| refuses the top rung `W = 64` | 0 | 1 | a these-all-compile suite |
| refuses `W = 33..35` | 0 | 3 | a these-all-compile suite |

The control is quiet, so the procedures are not reporting noise. Neither suite shape observes both
columns, and only a sweep against a separately written predicate does, because only the predicate
knows which widths **should** have compiled.

The rung selection in that probe carries the width in the type rather than in a bound, because a
where-clause does not disambiguate impls. `16_probes/p6_first_attempt_note.md` reached the same wall
from the other side with `E0119`, and it is the closed panel's structural-keying result arriving a
third time.


### 3.4 The C1 perimeter instrument, built

I named this in a first pass as the cheapest missing thing and then left it, which is a blocker
reported rather than attacked. So it is built.

`t5_perimeter.rs` declares a numeral type with six public doors. Three are legitimate: a constructor
taking a width-carrying bit value, an accessor returning the declared width, and an operation taking
and returning numerals. Three leak the container: a public constructor taking `u16`, a `From<u32>`
impl, and a public field on a neighbouring column type holding `[u16; 8]`.

Every one of them compiles, and the behavioural suite in the same file, three const assertions
including two `size_of` checks, is green in both the leaky and the clean version. **Zero compile
errors either way.** So no test shape anywhere in this panel distinguishes them.

The instrument that distinguishes them enumerates the surface from rustdoc's own JSON:

```
rustdoc +nightly-2026-05-28 --edition 2021 -Z unstable-options \
  --output-format json --out-dir json t5_perimeter.rs
python3 t5_perimeter.py json/t5_perimeter.json
```

| variant | public positions | holes reported |
|---|---|---|
| leaky, three seeded leaks | 13 | 3 |
| control, leaks removed | 8 | 0 |
| alias, one leak behind a type alias | 10 | 1 |

Three of three found in the leaky variant, zero false positives in the control. The `width() -> usize`
and `behaviour_checks_all_pass() -> bool` positions are correctly not flagged, which is what the
exception list is for: an instrument with no exception list flags everything and gets switched off.

**The third row is the more interesting one, because it is a failure I did not design.** The first
version of the instrument read declared types and did not follow aliases, so I listed that as a known
gap. Rather than leave a stated weakness stated, I built the variant that exercises it: one line,
`pub type Storage = u16`, and a constructor taking `Storage`. The instrument reported the surface
clean. rustdoc records the argument as `resolved_path: Storage`, not `primitive: u16`, so the leak was
invisible.

That gap is now closed, because rustdoc carries the alias target in the same index and following it
costs one dictionary lookup. The alias variant now reports its one hole with the path it took,
`from_storage(raw via alias Storage)`. The failing version is preserved as `alias/`, since a gap that
was demonstrated and then closed is worth more in the record than a gap that was only ever asserted.

**What the instrument still cannot see**, printed by the instrument itself rather than only written
here: associated-type projections, transitive `Deref` targets, and impls authored in other crates,
which the crate-id filter excludes correctly for authorship and wrongly for reachability. So a clean
report is necessary and not sufficient. A dirty report is decisive, because every row in it is a
position a consumer can reach holding a type the criterion says a consumer never writes.

This is the cheapest item on the trusted list to move off it, and it is now a script rather than a
proposal.

## 4. Two regimes in which the certifying instrument is wrong, not merely narrow

This is the section I would put in front of op first, because it changes what the existing
certification means in both directions at once.

`16` establishes that the panel's erasure instrument cannot see the second output. I set out to
confirm that and found something further: **pointed at the cases it cannot see, the instrument does
not go quiet. It reports failure.** Symbol identity is not a conservative oracle. It has false
negatives, and they are in the two regimes that matter most.

### 4.1 At the aggregate arity, symbol identity reports erasure failing where it holds

`t2_aggregate_erasure.rs` builds the comparison the clause actually asks for: a typed walk over a
packed column, against the hand-written walk a consumer would write if they packed it themselves. The
latter is the "exactly what you describe" side.

```
rustc +nightly-2026-05-28 --edition 2021 -O --emit asm --crate-type lib \
  t2_aggregate_erasure.rs --out-dir asm
```

The assembler folds the scalar pair and does not fold the aggregate pair:

```
_t2_scalar_typed = _t2_scalar_native
```

Read through the panel's instrument, that says the scalar erases and the aggregate does not. Read
through the assembly, the aggregate bodies are **66 instructions each, 14 distinct opcodes each, and
the opcode multisets are identical**. The diff is register allocation, instruction scheduling and
label names, all three of which a compiler is free to choose.

```
python3 t3_opcode_oracle.py asm/t2_aggregate_erasure.s
```

```
  t2_typed_sum  vs  t2_handwritten_sum
    instructions      : 66 vs 66
    distinct opcodes  : 14 vs 14
    opcode multiset   : IDENTICAL
```

So the aggregate erases, and the panel's oracle says it does not. **The instrument cannot be repaired
by pointing it at larger programs**, which was the obvious response to `16`'s finding. It has to be
replaced, and the replacement has to be insensitive to the choices a compiler is licensed to make and
sensitive to the ones it is not. An opcode multiset is one such oracle, and section 6 says what it
still misses.

### 4.2 Below `-O2` the instrument fails, and the design does not

Symbol folding is an identical-code-folding pass. It does not run at low optimisation levels, so the
instrument's answer is conditioned on a build setting nobody has stated.

```
for o in 0 1 2 3 s z; do rustc +nightly-2026-05-28 --edition 2024 -C opt-level=$o \
  --emit asm --crate-type lib q12_erasure_asm.rs --out-dir /tmp/q12_O$o; done
```

| opt-level | symbol identity, cold against native | opcode multiset | instructions |
|---|---|---|---|
| 0 | reports NOT ERASED | ERASED | 14 |
| 1 | reports NOT ERASED | ERASED | 3 |
| 2 | ERASED | ERASED | 3 |
| 3 | ERASED | ERASED | 3 |
| s | ERASED | ERASED | 3 |
| z | ERASED | ERASED | 3 |

**Erasure holds at every optimisation level, including zero.** The typed and native bodies are
opcode-identical at all six. What fails below `-O2` is the instrument.

Both halves of that are worth carrying. The design's operation erasure survives more build settings than the
certification claimed, which is a result in its favour and one nobody had. And the instrument that
certified it is narrower and more fragile than assumed, which is a result against it. Those point
opposite ways and both are true, which is the usual shape when an instrument and its subject have
been conflated.

### 4.3 What is stable, which is a result worth keeping

I expected the certification to be toolchain-fragile and it is not.

```
for tc in nightly-2026-03-28 nightly-2026-05-28 nightly-2026-06-18; do
  rustc +$tc --edition 2024 -O --emit asm --crate-type lib q12_erasure_asm.rs --out-dir /tmp/q12_$tc
done
```

All three produce the identical three folded symbols, across roughly three months of nightlies. The
panel's existing result survives a check it had not been given, and that is worth saying rather than
only reporting what broke.

### 4.4 The finding I did not go looking for: packing blocks vectorisation

The control arm in `t2` compares the packed walk against a byte-aligned walk of the same width, at a
matched iteration count of 400. The access-width variant folded into the byte-aligned one, confirming
the only difference is the stride:

```
_t2_typed_sum_warm = _t2_typed_sum_aligned_access3
```

| walk | instructions | SIMD-shaped opcodes |
|---|---|---|
| stride 13, packed | 51 | 1 |
| stride 16, byte-aligned | 92 | 83 |

The byte-aligned walk vectorises to NEON. The packed walk does not, and lowers to scalar shift and
mask. This is the shape the workspace already knows: one property the backend must prove before it
can act, removed by packing, and the vectoriser stops.

**What this costs is unpriced and I will not put a number on it.** Instruction count is not a proxy
for speed, and the vectorised form does more elements per instruction. What the comparison does
establish is that the `Cold` trade has two halves and this panel has quantified one of them. Every
packed-storage figure in `15` and `16` is a storage figure. Nobody has measured the walk, and until a
bench harness runs, the sentence "packed storage saves 23.1%" is half of a trade whose other half is
unmeasured. That belongs in front of op beside the storage tables, not underneath them.

## 5. What cannot be certified by measurement at all

Four things, and a design that does not know which four is not in a position to state its own
guarantee.

**The specification.** Every instrument above compares an implementation against a rule someone wrote
by hand. If the rule is wrong, every check is green and every check is wrong. There is no measurement
that certifies the oracle, only review, and the only real defence is that the oracle is written from
the intent by someone other than the author of the mechanism, and is dumb enough to be read in one
sitting. This is the irreducible core and it does not shrink.

**That a bounded domain transfers.** The C2 instrument runs over `W = 1..=48` or `1..=128`. That the
result transfers to every width the design admits is an argument, not a measurement, and it rests on
monomorphisation being uniform, which rests on no type being able to observe which instantiation it
is in. That is exactly what the bans on full `specialization` and `TypeId` buy. **So the forbidden
list is load-bearing for the transfer argument**, and if either ban were relaxed, every exhaustive
check at a model width would stop establishing anything about the real widths. That consequence is
larger than the features' own soundness and it belongs on the trusted list explicitly.

**That erasure holds for programs nobody wrote.** C4's operation half is quantified over programs. A
generator widens the sample and cannot close the quantifier; closing it would mean reasoning about the
compiler, which is not on the table. So the honest form of that clause is permanently a statement
about a sample, and the useful discipline is to say which programs, rather than to say "it erases".

**Ergonomics.** Op's bar is "easy and intuitive to write" for someone who does not know the plumbing,
per `SETTLED.md:109`, which also warns that the row is a lossy compression and sends the reader to the
source. No instrument evaluates that, and pretending otherwise would be worse than leaving it on the
trusted list.

## 6. The trusted base, as a written list

The value of a verification claim is measured by how small and how explicit this list is. Ours is
neither yet, which is fine at this stage and is not fine unstated. Everything below is trusted, not
proved, in the current state of the panel.

1. The hand-written admissibility rule and the hand-written layout specification any conformance
   oracle checks against.
2. That the declaration catalogue used for expressibility is representative of consumer usage.
3. That no public observation admits a container. Decidable, and no longer uninstrumented: section
   3.4 builds the check. It stays on this list until it is run against the real surface rather than
   against a probe, and until the three gaps it prints about itself are closed.
4. That the bans on full `specialization` and `TypeId` hold, since the transfer from a bounded check
   to the full width range rests on them.
5. That `repr(transparent)` means what it says, which is a language guarantee and the strongest item
   on this list.
6. That the programs sampled for operation erasure are representative of the programs consumers write.
7. The compiler, the assembler and the toolchain pin, which no instrument here checks and which
   section 4.3 gives three months of evidence about rather than a guarantee.
8. That the defect families used in any defect matrix are the defect families that occur.
9. Which of the three readings of "then validate" is the intended one, which is unresolved.

Item 9 is the one I would move off this list first, because it costs op a sentence and nothing else
can settle it. Item 3 is next and is now most of the way there.

## 7. Applying it backwards: which of the panel's claims survive an adequate instrument

The dispatch asks for classes rather than a re-audit, which is the right unit. Counts first, each with
its command, run from the panel directory.

```
ls */*.rs | grep '_probes/' | wc -l          ->  131 rust probe files
ls */*.py | grep '_probes/' | wc -l          ->   36 python probe files
grep -rl 'emit asm' */*.rs */*.sh | wc -l    ->    2 probes emitting assembly
grep -rn '#!\[feature' */*.rs | wc -l        ->   44 feature gates in probe dirs
```

The second number is the one to sit with. **Clause four of the acceptance criterion, across this
entire panel, rests on a single probe**, `15_probes/q12_erasure_asm.rs`. The other file is mine. A
clause certified by one instrument is a clause whose certification is exactly as good as that
instrument, and section 4 is about that instrument.

The feature-gate audit comes back clean on the thing that matters. All 44 are
`min_generic_const_args` (14) and `lazy_type_alias` (3), counted by name; none is on the forbidden
list. `lazy_type_alias` is neither allowed nor forbidden in the workspace's vetting tables, so it is
**unvetted**, and `12_probes/p14_lazy_type_alias.rs:6-7` says so itself and declines to make an
admissibility argument on it, which is the correct handling.

One stale header worth a line and not more. `13_probes/p19_lazy_type_alias.rs` opens with
`#![feature(lazy_type_alias)]` on line 1 and a doc comment on line 3 describing the code as having
"ZERO feature gates". The comment is inherited from the `p9` core the file was copied from. Without
the gate the file compiles clean; with it, 45 errors, which is the finding. `13:579` reads it
correctly as a cost measurement rather than a refusal, so the citing file is right and only the probe's
own header would mislead someone reading it standalone.

Now the classes.

**Class A: compile refusals.** "This does not compile, and here is the diagnostic."
`16_probes/p5b_const_to_type.rs` with its committed `.err`, `10`'s three refusals, the `E0119` in
`p6_first_attempt_note.md`, and my own `t4` rows. **These survive an adequate instrument, and they
are the panel's strongest evidence.** A refusal is a decidable negative claim about the compiler's
accept set, and there is no map for it to be green over. Its one failure mode is that the probe
refused a different program from the intended one, which is a review question and not a coverage one.
The panel should be more confident in this class than in any other, and it currently is not.

**Class B: existence claims.** "This shape compiles, gate-free." `16_probes/p6`, `15_probes/q07`,
`10`'s ladder. **These survive as existence claims and do not survive as adequacy claims**, and the
distinction is where the panel is most at risk of reading more than is there. A probe showing that a
trait can emit two associated items proves that it can. It does not prove those two are the right two,
and `16:486-488` flags exactly this about its own `p6` scaffolding.

**Class C: counts and enumerations.** `16`'s 1024 declarations, 10 carriers, 256 pairs; `15`'s
461-against-476; my own 528 and 420. **These are the most fragile class**, and the fragility is never
in the arithmetic. It is that the count's domain and the claim's domain are stated in different places
or not at all. `RULES.md:124-126` records a prior panel's headline counts turning out to be an artifact
of the enumeration bound, and the open list at `SETTLED.md:143-146` currently carries an unresolved
81-against-zero discrepancy of exactly this kind. The discipline that fixes it is one sentence: a count
carries its domain in the same sentence as its value.

**Class D: layout and size assertions.** Split down the middle and the two halves look identical on
the page. An assertion comparing a type's size against **another type's** size is real evidence. An
assertion comparing a type's size against a number the same derivation produced is
`p3_blind_suite.rs:105-114`'s tautology. Anyone auditing this class has to read which side of that line
each assertion falls on, and the page will not tell them.

**Class E: per-value behavioural agreement.** Round trips, arithmetic agreement against a reference.
**This is precisely the class that came back green over an insufficient map**, and my P1 row measures
it at zero defect instances observed out of 528. These claims are not wrong. They carry no information
about anything aggregate, and they are the bulk of what a suite looks like.

**Class F: erasure and codegen-equality claims.** One probe, now with two known false-negative regimes.
Survives as a statement about the program it ran, at `-O2` and above, on three toolchains. Does not
survive as a statement about the design.

**Class G: timings.** None exist, and the panel has been consistently honest about it. Every magnitude
is unpriced, and section 4.4 adds one more thing to the unpriced list that currently reads as though it
were settled.

## 8. What is op's, and what is not

Not a request for a ruling on anything measurable. `01:96-98` is explicit that a measurement dispute is
not escalated, and nothing here is one.

**One question, and it is small.** Which of the three readings of "then validate" at `SETTLED.md:68`
did he mean: that the typestate refuses declarations it cannot serve, that it refuses operations
violating the declared invariants, or that the derived container is checked to hold the declared range?
The panel has substantial evidence for the second, none for the first, and has never recorded which it
is answering. One sentence from him decides which instrument is owed, and no amount of expert
convergence can decide it, because it is a question about what he meant.

**One caveat, not a question.** `16:711-715` states that the gate has been met on one of its two nouns.
I would put it slightly differently after section 4, and the difference is worth having in front of him.
The gate has been met on **one program, at one arity, at one optimisation level band, with an oracle
that has two false-negative regimes**, and separately, the part of clause four that is genuinely proved
is proved by construction rather than by that instrument at all. Whether any of that reopens anything is
his call. It is recorded so the morning's reader does not carry "the erasure gate is met" forward as a
statement about the design when it is a statement about a probe.

## 9. Attacks on my own answer that did not land

**Is the opcode multiset a good enough oracle?** Not on its own, and I want that on the record rather
than implied. It is insensitive to operand values, so it would equate a shift by three with a shift by
four, and it would equate two loops with the same instructions in different orders where the order was
load-bearing. It is strictly better than symbol identity at the aggregate arity, which is what section
4 needs it for, and it is not a correct erasure oracle. A real one probably compares normalised
instruction sequences with registers alpha-renamed, and I did not build that.

**Could the defect matrix be replaced by property-based testing?** Partly, and the part it cannot
replace is the interesting part. A generator over values would find D1 and D4 given enough draws. It
would not find D3, because D3 is a property of the **procedure** and not of the data, and a generator
that writes the whole run ascending generates ascending writes forever. Nothing in a value generator
proposes writing one element into a poisoned buffer.

**Could the layout oracle be exhaustive over the true domain rather than a bounded one?** No, and the
reason is on the record from a prior pass in this workspace: exhaustive const evaluation quadruples in
cost per bit and rustc refuses at nine bits under `long_running_const_eval`. So a model-width check is
the only form available, and its transfer to the real widths is the argument in section 5 rather than a
measurement.

**Is the tail-element finding an artifact of my probe?** I thought so at first and it is not. It
appeared as a control failure I could not explain away, and the arithmetic is independent of the probe:
at `W = 13`, `N = 1000`, the last element starts at byte 1623 and a 3-byte access reads byte 1625 of a
1625-byte buffer. Any word-load reader of an exactly sized packed run has it.

**Does the vectorisation finding survive a fair comparison?** It survives the two confounds I could
name. Iteration counts are matched at 400, and the access-width variant folded into the byte-aligned
one, so stride is the only difference. It does not survive being turned into a performance claim, and I
have not made one.

## 10. Coverage, and what I did not do

I read `RULES.md`, `01`, `04`, `SETTLED.md`, `16` in full, `15` in the sections `16` cites and the
sections bearing on erasure, `15_probes/q12_erasure_asm.rs` and `q13_cold_packed.rs` in full,
`16_probes/p3_blind_suite.rs` and `p4_access_width.rs` in full, and the headers of the fifteen
expected-to-fail probes. I did **not** read `02`, `03`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`
or `14` in full, so every statement I make about them is from a grep or from a header, and is marked as
such where it appears. I did not read `CANON_CANDIDATE.md`, `MORNING.md`, `DROPLIST.md` or
`PERSONA_CALLS.md`. I did not read `mock/crates`, which is being nuked.

I did not audit any individual claim in the panel against my instruments. Section 7 classifies kinds,
which is what the dispatch asked for, and a class assignment is not a verdict on any member of it.

I did not extend the layout oracle above 128 bits, so nothing here bears on the wide rung, and
`16:541-544` has the same gap. The access-width arithmetic assumes a field inside a machine word and I
did not check it against a multi-limb payload.

I did not build a normalised-instruction-sequence oracle, so section 4's conclusions rest on an opcode
multiset whose weaknesses I named in section 9.

I ran no bench harness, so every magnitude here is unpriced, including the one in section 4.4 that
looks most like it wants a number.
