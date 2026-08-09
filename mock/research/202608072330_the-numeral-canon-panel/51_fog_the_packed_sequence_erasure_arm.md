# 51. The packed sequence erasure arm

**Date:** 2026-08-09. **Persona:** Agner Fog. **Mode:** explore, do not settle (`00_brief.md`, `04`,
`28`). **Position in the unit:** seventh file on one topic, after `44` through `50`. Nothing here
settles anything.

**Status: COMPLETE.** Written to disk before the work and extended in place, per `RULES.md:328-329`.

My dispatch says the arm that would settle whether a packed sequence erases does not exist, and that
the topic's erasure evidence is therefore either scalar or known-blind. **The first half of that is
false and I checked it before doing anything else.** The arm exists, at
`17_probes/t2_aggregate_erasure.rs`. It was built, it was run, its result is on disk at
`17_probes/t3_opcode_oracle.out`, and it reproduces byte-for-byte on this machine.

What is true is the part underneath, and it is worth more than the part I was sent for. The existing
arm is quantified over **one width**. Swept across thirty-six, a typestate packed walk stops matching
its hand-written twin at declared width eighteen, and the failure is not extra instructions. It is
the reduction serialising: the typed arm collapses from four elements per iteration on five
independent accumulators to one element per iteration on one, so the loop-carried dependent work per
element roughly triples. Two attacks recover it, one of them at every width in the matrix, and
neither changes a character of what the consumer writes.

## 0. Gates

### 0.1 Canon gate

There is no canon to defend. `mock/canon/` does not exist on this branch: `ls` returns "No such file
or directory". The fixed material is `01`, `04`, `28`, `INTENTS.md`, the workspace discipline, the
forbidden-feature list, and the acceptance criterion quoted at `00_brief.md:144-146`. My question
sits inside the criterion's fourth clause, "it erases on lowering", and nothing below proposes a
mechanism the forbidden-feature list excludes. Every probe in `51_probes/` compiles gate-free:

```
$ grep -rn '#!\[feature' 51_probes/ | wc -l
0
```

**Gate: passes.** The mode forbids settling, and this file settles nothing: it reports what a set of
compiled artifacts do, kills one option, and hands two attacks forward.

### 0.2 Test gate

There is no suite to audit. `mock/crates` is empty by construction and `mock/Cargo.toml:1-17` says
why, so:

```
$ cargo test --workspace          # in mock/
error: manifest path `.../mock` contains no package: The manifest is virtual,
and the workspace has no members.
```

Nothing to refuse over. I did apply the gate's failure kinds to my own arms instead, which is where
they bit: my first harness produced 36 of 36 green and could not have produced anything else.
Section 3 is that audit and I ran it on myself before reporting a number.

### 0.3 Independence

I did not derive cold. My dispatch names `16`, `17`, `43`, `47`, `49` and `50` and I read all of
them, so where I agree with a predecessor that is a read and not an independent instance
(`RULES.md:359-375`). Where I attack, independence is not needed: an attack carries its own
citation.

## 1. The dispatch's premise is false, and the falsification took one command

`RULES.md` and `panels-argue-the-intent-not-the-wording.md` both put breaking the brief before the
assigned lens, so that is where I started. My brief says:

> the only probe combining a packed sequence with an assembly-level erasure check is
> `16_probes/p3_blind_suite.rs`

Two things are wrong with that sentence and they fail in opposite directions.

**`16_probes/p3_blind_suite.rs` is not an assembly-level erasure check at all.** It has a packed
store and it walks it, but its erasure check is a runtime value comparison, and its own author says
so in the code:

```rust
fn check_codegen_equality() -> Result<String, String> {
    // NOTE: this is a weak stand-in. The panel's real erasure check compares emitted symbols
    // and reports that LLVM folds the two into one.
```

`16_probes/p3_blind_suite.rs:98-100`. The function then compares two `u32` values and one
`size_of`. No assembly is emitted anywhere in that file. So the probe my brief names as the panel's
only packed-sequence erasure check is not one, and citing it as the panel's erasure evidence
overstates what it claims about itself.

**The arm the brief says nobody built is `17_probes/t2_aggregate_erasure.rs`.** Its header states the
exact gap my dispatch describes:

> The panel's instrument (15's q12) compares ONE typed operation on ONE value against ONE native
> instruction. That is a real result and it is quantified over a scalar. The clause says the design
> erases "to be exactly what you describe", and what a Cold consumer describes is a packed run, so
> the instrument has to compare a typed walk over an aggregate against the hand-written walk it
> claims to erase to.

`17_probes/t2_aggregate_erasure.rs:3-9`. It builds a 1625-byte packed column at stride 13, walks it
through a generic `Col<D: Derived>`, walks the same bytes through a hand-written twin, and emits
assembly for both. It is exactly the arm.

It reproduces. On this machine, on the pinned toolchain:

```
$ rustc +nightly-2026-05-28 --edition 2021 -O --emit asm --crate-type lib \
    ../17_probes/t2_aggregate_erasure.rs --out-dir repro
$ wc -c repro/t2_aggregate_erasure.s ../17_probes/asm/t2_aggregate_erasure.s
    6893 repro/t2_aggregate_erasure.s
    6893 ../17_probes/asm/t2_aggregate_erasure.s
```

Byte-identical output. `17`'s committed assembly is what its toolchain emitted, and it is what mine
emits.

**What that costs the register.** `OPTIONS.md` and this unit have been carrying "the packed-sequence
erasure arm is missing" as a live gap. It is not missing. It is one directory away and its result
has been on disk since file 17. `RULES.md:335-357` already records this exact failure once, where
eighteen files reported a trade as unpriced while `mock/benches/` held the measurement: **a negative
claim about evidence is a claim about a place, and it is checkable in one command.** This is the same
failure in the same panel, one axis over, and the fix is the same: before a brief says an arm does
not exist, grep for it.

I am not going to pretend that finding costs nothing. A dispatch slot was reprioritised onto a hole
that was already filled, on the strength of a census nobody re-ran. The census itself is also wrong
on its numbers, which I return to in section 7.

## 2. What 17's arm actually established, and where its instrument stops

`17` ran two oracles over that assembly and found them disagreeing:

```
--- oracle A: symbol identity, which is the panel's instrument ---
    t2_typed_sum           vs t2_handwritten_sum     -> reports NOT ERASED
--- oracle B: opcode multiset ---
  t2_typed_sum  vs  t2_handwritten_sum
    instructions      : 66 vs 66
    distinct opcodes  : 14 vs 14
    opcode multiset   : IDENTICAL
```

`17_probes/t3_opcode_oracle.out`. That is a real and important result, and `17` reads it correctly:
symbol identity produces a false negative on two bodies that differ only in register names.

But opcode multiset is a weak instrument in a way `17` does not name, and it is weak in the direction
that matters most for a reduction. **A multiset is a bag. It cannot see which operand feeds which, so
it cannot see the dependency structure, and the dependency structure is what a latency-bound loop is
limited by.** Two bodies with byte-identical opcode bags can differ in throughput by a large factor
if one has a serial accumulator chain and the other has four independent ones. That case is not
hypothetical here; section 4 is full of it.

So I built the instrument up, at `51_probes/oracle.py`. Five oracles, weakest first, each with its
blindness stated in the file:

- **O1, symbol identity.** Sound when it fires, false-negative otherwise. `17`'s oracle A.
- **O2, opcode multiset.** `17`'s oracle B. Blind to order and to dataflow.
- **O3, alpha sequence.** Identical instruction sequence after renaming registers by first
  occurrence. Sees order. Being itself order-sensitive, it cannot see through a reschedule.
- **O4, value-numbered dataflow.** Each instruction keyed on its mnemonic, its immediates and the
  value numbers of its operands, so two bodies match when they compute the same dataflow graph
  whatever registers or order the allocator chose. Live-ins are matched by use signature rather than
  by name or position.
- **O5, recurrence length.** Longest chain of dependent instructions through the loop block.

O4's bias is one-directional by construction and that is deliberate: it can fail to notice that two
equal bodies are equal, and it cannot report two different bodies as equal. When the instrument is
for finding failure, that is the direction to be wrong in.

Run on `17`'s own assembly, unchanged:

```
t2_scalar_typed vs t2_scalar_native        3/3    O1=1 O2=1 O3=1 O4=1 rec=2/2   ERASED: folded to one body
t2_typed_sum vs t2_handwritten_sum        66/66   O1=0 O2=1 O3=0 O4=1 rec=10/10 ERASED: same dataflow, rescheduled, same recurrence
t2_typed_sum_cold_400 vs ..._access3      51/92   O1=0 O2=0 O3=0 O4=0 rec=9/13  NOT ERASED
```

**This strengthens `17`'s conclusion rather than qualifying it.** `17` could say the two bodies carry
the same bag of opcodes. I can say they compute the same dataflow graph with the same loop-carried
recurrence, and that every textual difference between them is register allocation plus the schedule
of mutually independent operations:

```
$ diff <(awk '/^_t2_typed_sum:/,/cfi_endproc/' ...) <(awk '/^_t2_handwritten_sum:/,/cfi_endproc/' ...)
14c14
< LBB2_1:
---
> LBB0_1:
...
< 	and	x15, x15, #0x4
< 	lsr	x15, x1, x15
---
> 	and	x1, x15, #0x4
> 	lsr	x15, x15, x1
```

The phase mask is computed earlier in one and later in the other, and the two registers holding the
accumulated bytes and the phase are swapped. Nothing else. At W = 13 the typestate is free.

**And the caveat I owe on my own instrument.** A reschedule is free on an out-of-order core with
enough rename registers. It is not free on an in-order core, and `32_op_arvo_adapts_to_the_cores_it_finds.md`
puts cores arvo does not control inside the design's scope. O5 counts dependent instructions and has
no latency table, no port model, no issue width and no memory model, so it cannot tell you a longer
chain is slower, only that it is longer. **Everything in this file is a count read off emitted
assembly. Nothing is timed and no bench harness has run, so every magnitude here is unpriced.**

## 3. My first harness produced 36 of 36 green and could not have produced anything else

Before the result, the audit of the thing that produced it, because I nearly shipped a number that
meant nothing.

The obvious move once you have an oracle is to sweep the width axis, so I generated one crate per
width with two arms: a typed walk whose stride, access count and mask arrive as associated consts,
and a hand walk with the same numbers as literals. Thirty-six widths. Every one came back:

```
  1   1   128/128     75/75    13/13    1  1  1  1  ERASED: folded to one body
  ...
 48   7    14/14      10/10     4/4     1  1  1  1  ERASED: folded to one body

summary: 36 widths built, 0 failed to build or parse
         36 of 36 report ERASED under the strongest oracle that fired
```

`51_probes/width_matrix_O3_n1000.out`. Thirty-six of thirty-six, folded to one body, at O1.

**That result is worthless and the harness is why.** Both arms were the same MIR written twice. The
typed arm's `while i < D::ACCESS` and the hand arm's `while i < 3` are the same loop once the
associated const is resolved, so LLVM CSE'd them and the assembler aliased the symbols:

```
$ grep -nE '^_w13_' asm_O3_n1000/w13.s
5:_w13_hand:
68:_w13_typed = _w13_hand
```

The fold is a true statement, and what it is true about is that associated consts resolve before
codegen. Nobody doubted that. **Nothing in that harness could have failed**, which is the first
failure kind `the-test-gate.md` names, and the fact that it produced a full green table is what makes
it dangerous rather than merely useless. I kept the generator and the output in `51_probes/` because
the fold is the finding about the harness, and `verify.sh` step 1 reruns it so a later reader sees
what a rigged arm looks like next to a real one.

The workspace rule is blunter than I would be about it: "a bench with no real competitor is not a
bench, it is a demonstration". The same holds for an assembly comparison. The hand arm has to be code
a consumer would actually write, not the typed arm with the constant arriving from a different place.
`17`'s t2 got this right at W = 13 and I got it wrong before I looked at how `17` had written its
hand arm.

**The rebuild, at `51_probes/gen_v3.py`.** Five arms per width, and the ones that are supposed to
differ do:

- `typed`, the gather written as a loop over an associated const.
- `hand`, the gather unrolled flat to exactly this width's byte count, every constant spelled at the
  site. This is `17`'s shape.
- `native`, no packing at all, an unpacked array of the smallest carrier that holds W bits. The
  competitor the packing is traded against, which no oracle in this panel had compared against.
- `gather`, attack one, below.
- `wide`, attack two, below.

**And the arms are checked against an independent reference, by calling them.**
`41_dispatcher_note_no_bench_here_has_ever_checked_its_answers.md` measured the digest column zero
across 214 CSVs and 82,960 rows, so nothing committed here has ever confirmed that the arms of a
comparison compute the same answer and one arm was found doing no work at all. A cross-check that
reimplements the algorithm has the same defect one level up, so `51_probes/gen_check3.py` links the
emitted `no_mangle` symbols out of static archives and calls them:

```
all 36 widths: five arms agree with an independently built reference, and no reference sum is zero
```

180 emitted functions, one buffer built independently per width, and the zero-sum guard is there
because agreement on zero proves nothing. The archives are 405 MB and are rebuilt on demand rather
than committed; `verify.sh` step 3 does it in one command.

## 4. The erasure claim was a sampled law, and it breaks at width eighteen

With arms that can disagree, the sweep says something.

```
Erasure verdict against the hand-written twin, by arm:
  typed  : ERASED at 18/36 widths. NOT at [18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,40,48]
  gather : ERASED at 36/36 widths.
```

`51_probes/v3_matrix.out`. **The typed arm erases at exactly the widths at or below seventeen, and
fails at every width from eighteen up.** The panel's single-width evidence sits at W = 13, which is
inside the region where it holds.

That is the shape `the-test-gate.md` calls a sampled law, and it is the most invisible gap available
because every test present looks reasonable. `17` chose 13 for good reasons: it is coprime with 8, so
the bit phase cycles with period 8 and every phase is exercised. It is also, by luck rather than
design, on the working side of a boundary nobody knew was there.

**The failure is not extra instructions, and this is the part an opcode count could never have
found.** Per width and arm, loop-block instructions, loads, independent reduction accumulators,
elements per iteration, loop-carried dependent instructions per element:

| W | live bytes | typed | hand | gather | wide |
|---|---|---|---|---|---|
| 13 | 3 | 43i/9ld/5acc/4e/2.25 | 43i/9ld/5acc/4e/2.25 | 43i/9ld/5acc/4e/2.25 | 34i/4ld/5acc/4e/2.00 |
| 17 | 3 | 43i/9ld/5acc/4e/2.25 | 43i/9ld/5acc/4e/2.25 | 43i/9ld/5acc/4e/2.25 | 34i/4ld/5acc/4e/2.00 |
| 19 | 4 | **11i/1ld/1acc/1e/6.00** | 34i/4ld/5acc/4e/2.00 | 34i/4ld/5acc/4e/2.00 | 34i/4ld/5acc/4e/2.00 |
| 23 | 4 | **11i/1ld/1acc/1e/6.00** | 34i/4ld/5acc/4e/2.00 | 34i/4ld/5acc/4e/2.00 | 34i/4ld/5acc/4e/2.00 |
| 31 | 5 | **13i/2ld/1acc/1e/7.00** | 43i/9ld/5acc/4e/2.25 | 43i/9ld/5acc/4e/2.25 | 34i/4ld/5acc/4e/2.00 |

At W = 19 the typed arm emits **eleven** instructions in its loop against the hand arm's
thirty-four, and it is the worse code. It processes one element per iteration where the hand arm
processes four, and it accumulates into **one** register where the hand arm accumulates into five.
The typed body:

```
LBB2_1:
	lsr	x11, x9, #3
	ldr	w11, [x0, x11]
	and	x12, x9, #0x7
	lsr	x11, x11, x12
	and	x11, x11, #0x7ffff
	add	x8, x11, x8        <- the only accumulator
	add	x9, x9, #19
	subs	x10, x10, #1
	b.ne	LBB2_1
```

`51_probes/asm2_O3_n1000/w19.s`. Every iteration's `add x8, x11, x8` depends on the previous
iteration's. A thousand elements is a thousand serially dependent adds. The hand arm's four
accumulators cut that recurrence by four, which is the entire reason compilers unroll reductions.

**A count of instructions says the typed arm is three times smaller. The dependency structure says it
is three times worse on the quantity that bounds it.** That inversion is why symbol identity and
opcode multisets are not enough, and it is the concrete case for adding O5 to the panel's toolkit.

I want to be exact about what "worse" is licensed to mean here. The recurrence is a count of
dependent instructions per element, 6.00 against 2.00. It is not a cycle ratio and I have not
measured one. What it supports is a qualitative existence claim: the typed arm's reduction is serial
and the hand arm's is not.

## 5. The mechanism, and the control that refuted my first attribution

Two things move together at W = 17 to 18. The declared width crosses 17, and the number of live
access bytes crosses 3 to 4. Either could be what the compiler is responding to.

**First control, which failed and taught me something.** `51_probes/gen_access_control.py` fixes W at
13 and sweeps the access count from 1 to 8. Reading extra high bytes is harmless because the result
is masked to W bits, so every variant still computes the same answer.

```
ACC  loop-form           flat-form           verdict
3    43i/5acc/4e         43i/5acc/4e         ERASED: same sequence, different registers
4    43i/5acc/4e         34i/5acc/4e         NOT ERASED
8    43i/5acc/4e         34i/5acc/4e         NOT ERASED
```

The loop form's code is **identical for every access count from 3 to 8**, which is the tell. At W =
13 the mask is `0x1FFF`, so bytes past the third contribute nothing, LLVM proves those loads dead and
removes them, and the crate compiled at ACCESS = 8 emits what the crate at ACCESS = 3 emits. My
control did not vary what I thought it varied.

**And the reason it could not is structural.** The number of live access bytes is `floor((W + 14) /
8)`, a function of W alone. The width and the access window cannot be varied independently, so **no
experiment inside this design can attribute the boundary to one rather than the other.** That is a
fact about the design worth writing down: the two coordinates `16:126-141` treats as separate outputs
are, on this axis, not independently observable.

**Second control, which works.** `51_probes/gen_fixed_gather.py` holds the gather **shape** fixed
across every width, at eight bytes, and sweeps only W. Three shapes: written as a loop, written flat,
and one `u64` load.

```
W    live  loop8                  flat8                  wide
13   3     43i/9ld/5acc/9e        34i/4ld/5acc/8e        34i/4ld/5acc/8e
17   3     43i/9ld/5acc/9e        34i/4ld/5acc/8e        34i/4ld/5acc/8e
18   4     28i/8ld/4acc/8e        19i/4ld/4acc/5e        19i/4ld/4acc/5e
19   4     11i/1ld/1acc/6e        34i/4ld/5acc/8e        34i/4ld/5acc/8e
23   4     11i/1ld/1acc/6e        34i/4ld/5acc/8e        34i/4ld/5acc/8e
31   5     13i/2ld/1acc/7e        34i/4ld/5acc/8e        34i/4ld/5acc/8e
47   7     17i/4ld/1acc/9e        34i/4ld/5acc/8e        34i/4ld/5acc/8e
```

`51_probes/fixed_gather.out`. The source is character-identical across every row of a column except
the stride multiplier and the mask. **The collapse is a conjunction of two conditions and neither
alone suffices:**

- the gather is written as a **loop** rather than flat, and
- the width is 18 or above, equivalently the live access window is 4 bytes or more.

`flat8` at W >= 18 does not collapse. `loop8` at W <= 17 does not collapse. `loop8` at W >= 18 does.

**And there is a second, smaller effect that shows up much earlier.** From W = 10 upward, the
loop-written gather is not load-widened while the flat one is: 9 loads against 4, 43 instructions
against 34, at every width where both keep their accumulators. A three-byte gather has no native load
width so neither form can do better, but a four-byte gather is exactly a 32-bit load and only the
flat form is recognised as one. That is a small, situational, entirely real win available at every
width from 10 up, which is the shape `small-wins-compound-into-the-program.md` is about.

My honest statement of the proximate cause: the loop-written gather is a nested loop that has to be
unrolled and its loads recombined before the outer reduction can be considered, and by the time that
has happened the outer loop's unroll decision has been made against a body that looked different.
**I have not proven that.** It is the reading most consistent with the artifacts, it explains both
effects, and confirming it means reading LLVM pass output rather than emitted assembly, which I did
not do. Section 8 says what would settle it.

## 6. Two attacks, both of which land, and neither changes what a consumer writes

`a-refused-bound-wants-a-trait-not-a-feature.md` says that when a constraint will not go into one
place, the move is to break it into named pieces that each hold on their own. The gather is exactly
that case: the access count is trying to be a loop bound and it wants to be a contract.

**Attack one, the trait-supplied gather.** `D::ACCESS` stays, and a second associated item is added:
an associated type whose impl performs the gather flat for that access width. There are eight impls,
`G1` through `G8`, and the derivation names one. The consumer writes exactly what it wrote before.

```
  gather : ERASED at 36/36 widths.
```

**Thirty-six of thirty-six.** At every width in the matrix the trait-supplied gather emits what the
hand-written twin emits, including the entire band from 18 up where the loop-supplied one does not.
Where the typed arm serialised onto one accumulator, the gather arm has five:

```
  W=19  typed accs=1 chain=6  ->  gather accs=5 chain=8, wide accs=5 chain=8
  W=23  typed accs=1 chain=6  ->  gather accs=5 chain=8, wide accs=5 chain=8
  W=31  typed accs=1 chain=7  ->  gather accs=5 chain=9, wide accs=5 chain=8
  W=33  typed accs=1 chain=7  ->  gather accs=5 chain=9, wide accs=5 chain=8
```

Fifteen widths recover this way, listed in full in `51_probes/v3_matrix.out`.

**Attack two, the wide load.** The typestate knows the buffer carries a tail, so the gather becomes
one fixed 8-byte load and a shift, with no per-width gather at all. It costs eight bytes of buffer
slack.

This one does **not** erase against the hand-written twin at most widths, and reporting that as a
failure would be wrong. It is different code, and at most widths it is better code: at W = 13 it is
34 instructions and 4 loads against hand's 43 and 9, on the same five accumulators. At the
byte-aligned widths the gap is large: 14 against 55 at W = 8, 15 against 86 at W = 16, 19 against 121
at W = 24, 12 against 27 at W = 32.

And at W = 47 it does something the other arms cannot:

```
  W=47  typed accs=1 chain=9  ->  gather accs=1 chain=9, wide accs=5 chain=8
```

At W = 47 the **hand-written** arm also serialises onto one accumulator, and so does the trait
gather. Only the wide load recovers the split. **That is a case where the typestate arm is better
than what a competent consumer writes by hand**, which is the strongest form of the argument that the
typestate is worth its friction, and it exists because the derivation knows the buffer's tail and the
consumer at the call site does not.

**What the packing costs against not packing, in shape.** The `native` arm walks an unpacked array of
the smallest carrier holding W bits. At W = 13 it is 49 instructions and 3 loads with **zero**
reduction accumulators identified, against the packed arms' 34 to 43 with five. The two are not
comparable on instruction count because they touch different amounts of memory, which is the entire
trade. **The shape is visible here and the magnitude is not.** Pricing packing against not packing
needs the bench harness with both as arms, and `50` has just shown that is reachable
(`50_lamport_which_criterion_is_in_use.md:24-27` reports a harness run at 3.04x to 3.12x on an
adjacent question). Until that runs, the packing trade at these widths is **unpriced** and I use the
word rather than reaching for a number.

## 7. Three smaller things I checked because they were cheap

**No failure paths anywhere.** `43_rompf_what_a_composition_is.md:73` found that a run whose length is
bounded by its capacity still emitted two bounds-check failure paths, and `43_probes/p7.out` records
`BoundedRun::sum` at 143 asm lines with 2 fail calls against `sum_run_clamped` at 94 with 0. I looked
for the same thing across all 36 of my width crates and five arms each:

```
total files with any failure path: 0 of 36
```

Every index in my arms is `bytes[byte + i]` on a fixed-size array with a runtime index, which is
exactly the shape `43` found emitting checks, and here LLVM proves all of them dead. That does not
contradict `43`: its shape carries a capacity bound through a type and mine derives the index from a
loop counter the compiler can range-analyse. It does mean the packed walk does not pay `43`'s cost,
and that is worth knowing before anyone generalises `43`'s finding to packed access.

**The census in my brief is wrong on its numbers.** My brief says 178 probe files, 23 building arrays
and 19 looping. Measured now:

```
$ find . -type d -name '*_probes' -exec find {} -name '*.rs' \; | wc -l
196
$ grep -rlE '\[[A-Za-z_0-9<>, ]+;[ ]*[0-9A-Z_]+\]' --include='*.rs' . | wc -l
50
$ grep -rlE '\bfor\b .* in |while ' --include='*.rs' . | wc -l
59
```

196, 50 and 59, against 178, 23 and 19. Some of the gap is files added since the census was taken and
some of it is not: 196 minus the 20 probes in `49_probes` and `50_probes` and `47_probes` is 176, and
no subtraction reaches 23 from 50. `RULES.md:124` requires every number to be produced with a command
and to say which command, and a census that cannot be reproduced from its own definition should not
be the thing a dispatch is reprioritised on. Mine are above; disagree with the regex if you like, but
disagree with a command.

**The oracle found a bug in itself first.** My first O5 returned a recurrence of 1 for a 58-instruction
loop, because I ran the dependency analysis on register names that a previous normalisation pass had
already rewritten. My first O4 reported a difference between two bodies a plain `diff` showed to be a
reschedule, because I seeded live-in registers by first-use order and the two bodies use theirs in a
different order. Both are recorded here rather than quietly fixed, because an instrument that has
been wrong twice in one session is an instrument whose remaining output deserves the reader's
suspicion, including mine.

## 8. Bounded coverage, stated as specifically as I can

What this file does **not** establish, in descending order of how much it would change the answer.

**One host, one target, one toolchain.** Everything is `aarch64-apple-darwin`, `rustc 1.98.0-nightly
(57d06900f 2026-05-27)`. aarch64 has 31 general-purpose registers; a five-accumulator unroll is
cheap there and may not be on a register-poor target. The finding could look different on x86-64 and I
have not looked. `32_op_arvo_adapts_to_the_cores_it_finds.md` makes that gap matter more than it
would otherwise.

**No cycles, anywhere.** Every number is a count off emitted assembly. O5 has no latency table, no
port model, no issue width and no memory model. The 6.00-against-2.00 recurrence ratio at W = 19 is a
ratio of dependent-instruction counts and is not a claim about time. **Nothing in `51_probes/` is a
bench and I have not run the harness.** What the collapse costs is unpriced, and it is exactly the
kind of question `mock/benches/` exists for.

**The proximate mechanism is a reading, not a proof.** I can show the collapse requires the loop-form
gather and requires W >= 18. I cannot show *why* the unroller behaves differently, because that needs
`-C llvm-args=-print-after-all` or the remarks output, and I read assembly rather than pass output.
Someone should do that; it is a bounded afternoon and it would turn a plausible story into a fact.

**Three widths where nothing recovers.** At W = 22, 30 and 32 the flat gather and the wide load both
collapse to one accumulator, so the residual set is not empty and I do not know why those three. They
are not obviously special: 22 and 30 are even, 32 is byte-aligned, and 24 and 40 share those
properties without collapsing.

**My elements-per-iteration inference does not always resolve.** It reads the induction step from an
`add`/`sub` with an immediate against the width, and pointer-stepping loops defeat it. Roughly a third
of the matrix rows show `?` there, and the `chain-per-element` column is only trustworthy where it
resolved. The accumulator count is robust and is what I leaned on.

**One reduction, one operation.** Every arm sums. A reduction is the case where the loop-carried
chain dominates, which is why it exposes this, and it is also the friendliest case for
accumulator-splitting. A map, a filter, a scatter, or a walk with a loop-carried value that is not
associative would all behave differently and none is here.

**I read seven panel files in full** (`16`, `17`, `43`, `47`, `49`, `50`, `RULES.md`, plus
`00_brief.md`), grepped the rest, and did not open `OPTIONS.md` or `INTENTS.md` beyond confirming I
must not cite them by line. I did not verify `50`'s fixpoint result or `47`'s six compiled refusals
independently; I take them as reported and neither bears on my measurements.

## 9. Against the option register

Per `RULES.md:263-266`, what this fits, what it fits badly, and what it kills. I am reporting these
for the register's maintainer rather than editing it, since `OPTIONS.md` is not mine to touch.

**Kills: "the derivation's output may be a bare carrier plus a stride, delivered as constants."** Not
on the injectivity ground `16:126-141` gives, which stands on its own, but on a codegen ground that
is independent of it. A stride and an access count delivered as **values a generic body reads**
reproduce the hand-written codegen at 17 of 36 widths, and the failures are not marginal: the
reduction serialises. The same information delivered as a **contract the derivation names an impl of**
reproduces it at 36 of 36. That is a live distinction between two shapes that the "two outputs"
framing treats as one, and it comes down on the side of the outputs being types rather than
constants. What would reopen it: a demonstration that the collapse is a defect in one LLVM version
rather than a property of the shape, or the same sweep on x86-64 coming out flat.

**Fits well: `47`'s result that one richer output suffices if and only if it is a type.**
`47_wingo_one_richer_output.md:8-11` gets there from six compiled refusals of the value-valued
spelling. I get to a compatible place from the other end and by a different route: the value-valued
spelling that *does* compile lowers worse, at half the widths in the matrix. Two independent routes to
"it has to be a type" is worth more than either, and I derived mine from assembly rather than from
`47`, though I read `47` first and say so.

**Fits badly: any option that fixes the number of outputs without saying what kind of thing an output
is.** The whole finding lives in the difference between an output that is a const and an output that
is a type with an impl behind it, and a count does not distinguish them. If the register carries
options keyed on cardinality alone, they under-determine the answer.

**Adds an option, and this is the part I would most like someone to attack.** The derivation may have
an output that is neither the carrier nor the extent: **the access operation itself**, as a contract
with one impl per access width. It is not derivable from the other two at the codegen level, which is
the whole content of section 6, and `50`'s finding that the governing criterion admits output sets of
several sizes (`50_lamport_which_criterion_is_in_use.md:17-21`) means the register cannot currently
rule it in or out. The wide-load arm is a second, cheaper instantiation of the same slot, which
suggests the slot is real even if neither filling is the right one.

## 10. What I would do next, in the order I would do it

1. **Run the harness.** `mock/benches/` with `typed`, `gather`, `wide`, `hand` and `native` as arms
   over a realistic workload, at three widths chosen either side of the boundary. Everything in this
   file is a shape, and the shape is only worth what it costs. `50` has shown the harness is
   reachable from this panel.
2. **Read the pass output.** `-C llvm-args=-print-after-all` on the W = 19 pair, to turn section 5's
   reading into a fact or refute it.
3. **The same sweep on x86-64.** Cheap, and it is the single biggest thing that could make this
   finding local.
4. **W = 22, 30, 32.** Three widths where nothing recovers, and I do not know why.
5. **Something that is not a reduction.** A map or a filter, to find out whether the collapse is
   specific to accumulator-splitting or general to the loop form.

## 11. Reproduction

`51_probes/verify.sh` rebuilds and reruns everything above from source on the pinned toolchain, in
five steps: reproduce `17`'s t2 and run the five oracles on it; rerun the rigged two-arm matrix so a
reader can see what a harness that cannot fail looks like; run the three-arm matrix; link the emitted
symbols and cross-check every arm against an independent reference; run the five-arm matrix, the two
controls and the attacks. Roughly 400 MB of static archives are built and deleted inside step 3
rather than committed.

Files, all committed beside this one:

| file | what it is |
|---|---|
| `oracle.py` | the five erasure oracles, with each one's blindness stated |
| `loopshape.py` | loop-carried structure: accumulators, recurrence, induction step |
| `gen_width_matrix.py`, `run_width_matrix.py` | the rigged two-arm harness, kept because the fold is the finding |
| `gen_v2.py`, `run_v2.py` | three arms, where the arms are different code |
| `gen_v3.py`, `run_v3.py` | five arms, adding both attacks |
| `gen_check3.py`, `check3.rs` | the cross-check that calls the emitted symbols |
| `gen_access_control.py` | the control that failed, and why it could not have worked |
| `gen_fixed_gather.py` | the control that separates the gather shape from the width |
| `robustness.py` | opt levels, element counts and target-cpu |
| `repro/`, `asm2_*/`, `asm3/`, `asm_acc/`, `asm_fix/` | emitted assembly for every claim above |
| `*.out` | the raw output of every run cited |
