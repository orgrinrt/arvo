# 141. The container fork, benched

**Author:** Haoran Xu (persona dispatch)
**Date:** 2026-08-07
**Position:** after `140_fog_warm_without_widening.md` and op's checkpoint `140b`, taking the one thing
`140b:69-71` said had to happen before this returns to op: harness benches with real competitor arms.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2024, aarch64-apple-darwin, Apple M1.
**Evidence:** nine harness bench sections in `mock/benches/`, 57 committed CSV plus meta plus findings
triples, driven from `mock/benches/bench.toml`. Six arm crates plus one shared transform crate at
`mock/benches/variants/warm-container-*`. A code-inspection artifact carrying no timings at
`141_probes/`.

STATUS: complete.

## Verdict

The fork was priced and it separates hard, but not on the axis either prior dispatch was arguing about,
and the investigation did not stop at the price. Six arms, four regimes, eight attacks on whichever arm
was losing, and two defects found in my own workload before its numbers were allowed to count.

**Delete the headroom.** It loses everywhere it was measured honestly: 45.3x at 8 bits, 21.2x at 16,
7.0x at 32 and 2.4x at 64 on a wrapping reduction, 1.8x to 5.9x on a wrapping elementwise transform, and
2.1x to 8.2x on a saturating elementwise transform, which is the regime `140` predicted and the one my
first saturating run got wrong. It never won a cell that survived checking.

**The mechanism is not footprint.** Crossing this host's 12 MiB L2 by doubling the column moved the
ratios by less than the noise: 45.3x becomes 40.7x, 2.4x becomes 2.4x. It is that the headroom container
makes the projection to `W` a real instruction where the minimum container makes it a no-op, and a real
projection on an accumulator turns a vectorisable reduction into a six-instruction serial scalar loop.
Structurally: **under the shipped rule no declared width is ever exactly filled, so the projection is
never free at any width, ever.** Under the deletion it is free at 8, 16, 32 and 64, which are the widths
a consumer arriving from plain Rust writes.

**And the container was never the biggest lever.** At the sixty widths that do not fill a rung the
container is worth 1.00x, and what is worth 14.6x to 26.4x there is where the projection sits, which
neither dispatch proposed changing because `140:196-206` concluded the optimiser handles it. It handles
it in a 64-element loop with a compile-time trip count, which is what `140`'s probe measured, and does
not at 8192 with a runtime one.

**On op's hypothesis that the answer is a matrix rather than one rule: the matrix is real, and it is not
on the container axis.** The container is monotone; minimum wins or ties in every measured cell. The
axis that genuinely flips is which *kernel shape* to emit, and it flips inside a single width sweep:
deferring the projection wins by 2.1x at 8 to 32 bits, and reassociating the fold into lanes wins by 10%
at 60 and 64. Composing both gains nothing over either, within 1%, because lane-splitting subsumes the
projection question entirely. That is three cells with three different answers, all decidable at compile
time from `W`, the container width and the operation kind.

**On abusing static knowledge to beat the compiler: it was tried twice and lost both times, and that is
the most useful thing in this file.** A range-witness shortcut that replaced `saturating_add` with an
add plus a clamp, licensed by arvo knowing every value is below `2^W`, ran 1.2x to 1.4x slower, because
the check it removed is one instruction this hardware provides. A hand-fixed eight-lane kernel ran 2.2x
slower than the compiler's own choice at every width where the compiler could already vectorise. Static
knowledge licensed both and measurement refused both.

**On reaching for the mathematical corpus: yes, and it produced the largest number in the file.** A
theorem arvo can evaluate at compile time and the machine code cannot carry, that a sum of `N` values
below `2^W` cannot reach a 64-bit saturation point, deletes every clamp in a saturating fold and turns a
serial loop into a vectorised one. Measured at **7.5x to 32x**, and it declines by itself at the widths
where the theorem is false. Section 12.

**On copy-and-patch and pre-compiled stencils: no.** Section 14 gives the reasoning and section 13 gives
the one cell where opacity would help, along with the arithmetic showing there is no win there either.

## 1. What Warm's intent implies, stated before anything is measured

`140:136-139` flagged, last, that its whole dispatch and `139`'s rested on an untested assertion, that
`Warm` wraps at the declared width `W`, traced to `131:275-280` and carried by four files. Op then
defined `Warm` by intent rather than by mechanism (`140b:16-21`):

> My standing call is "It should behave like native primitives in regular old rust would". Warm is the
> name for the default case that is not optimised for cold paths and cold storage, nor does it emphasise
> precision, but it also doesn't need to excessively shed any inefficiencies at the cost of accuracy and
> stability like Hot. It's the intuitive default way things behave, as they do behave with regular old
> primitives.

Four things follow, and the first two settle what `140` left open.

**A native primitive's wrap point is its declared width, and the two are the same number.** `u16 + u16`
is computed in `u16` and wraps at 16. No primitive in Rust has a declared width and a wrap point that
differ. So `Uint<13, Warm>` wraps at 13, and the assertion four files carried is right for a better
reason than the one they carried: it is not a mechanism someone chose, it is what the intent forces.

**A native primitive carries no headroom in storage.** `u16` occupies two bytes. It is not kept in a
`u32` so that one addition has somewhere to go. The headroom rule contradicts the intent on the storage
axis directly, before any performance number exists.

**The reading that would rescue headroom does not survive the rest of the design.** You could read
"behaves like native primitives" as "arvo's widths behave like the machine widths they land in", so
`Uint<13, Warm>` wraps at 16 and the container is the semantics. Refused by arvo's own identity, where
the exact width is the point, by the fact that a value wrapping at 16 does not fit the 13 bits a `Cold`
representation packs it into, and by `135b`'s gate, which has the consumer expressing usage in bits and
the typestate deriving the representation, requiring the declared bits to be the semantics.

**And the intent carries a fourth consequence that turned out to be the sharpest sentence in it.** A
Rust programmer's `u64` addition is one instruction with nothing around it. Whatever else `Warm` does,
the case where the declared width fills its container should compile to the machine's own operation and
nothing else. The shipped rule makes that case unreachable at every width, by construction, which is
section 5's finding stated in the intent's own terms before the benches ran.

So both prior dispatches priced the right obligation. I priced the same one. What they had wrong is in
sections 6 and 7.

## 2. What this brief takes for granted, checked before reasoning from it

**"The rule is `rung(rung_bits(W) + 1)`."** Confirmed at `arvo-strategy/src/container.rs:15-19`, stated
in its own module documentation: "**Warm / Precise**: 2x logical width (one bucket up; carries
single-op overflow headroom for Warm wrapping and Precise saturating semantics). `1..=8 -> u16`,
`9..=16 -> u32`, `17..=32 -> u64`, `33..=64 -> u128`." The dispatch is `tag_warm_precise` at `:77-91`
against `tag_hot_cold` at `:60-75`.

**"It widens every width at or below 64 bits."** Confirmed, and asserted over all 64 rather than sampled:
`the_shipped_rule_widens_every_width_to_64`.

**"Warm is arvo's default."** Confirmed: `S: Strategy = Warm` at `arvo/src/ifixed.rs:43`, and every alias
in `arvo/DESIGN.md.tmpl:42,71-72,537-546`.

**"`rung(W+1)` is a third candidate."** False, and it is the brief's own claim. `rung(W+1)` equals
`rung(rung_bits(W)+1)` at every width that fills its rung and equals `rung(W)` at every width below one,
so it is never a container the other two arms do not already name. Asserted over all 64 widths in
`plusone_is_never_a_third_container`.

I kept it as an arm anyway, for the reason a bench usually lacks. **An arm guaranteed to compile to the
same machine code as another arm measures the harness rather than the code.** Its spread against the arm
it aliases is the noise floor. Measured across every row it is mostly under 2%, reaching 8.5% once on a
saturating row and about 20% in one elementwise cell at 13 bits. That is what the ratios below have to
clear. It matters because the harness itself calls a 2.5% gap between two byte-identical arms
"significant" at p = 0.0064, and it is wrong.

## 3. The bench, the arms, and the two defects I found in my own workload

Six sections in `mock/benches/bench.toml`. Six arms. Every arm on every row computes the identical value
from the identical input, checked two ways: the harness compares outputs byte-exactly across arms and
drops any that disagrees, and
`all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key` runs every arm at every one of the
40 declared keys against a `u128` reference sharing no code and no carrier type with any of them. That
test is what makes the timing mean anything, and it is what caught the reassociation arms being legal.

The arms are alternatives someone might genuinely choose, not a proposal against a strawman:

- `headroom`: the shipped rule, `rung(rung_bits(W)+1)`, projection after every operation.
- `minimum`: `140`'s deletion, `rung(W)`, projection after every operation.
- `plusone`: `rung(W+1)`, the brief's third candidate, kept as the aliasing control.
- `native`: minimum container, projection written once before the value is observed. What a Rust
  programmer writes by hand, and the bar op's intent statement sets.
- `kernel`: minimum container, eager projection, fold reassociated into eight lanes. The arm that tests
  what arvo knows statically and LLVM will not infer.
- `lanes-deferred`: minimum container, deferred projection, fold reassociated into eight lanes. The
  composition.

All six call one transform per regime, differing only in the carrier they instantiate it at and two
const parameters. A bench whose arms each carry their own kernel measures several possibly-drifted
programs rather than one program in several containers.

### Defect one: the density sweep was measuring the optimiser

The first transform cycled add, multiply-by-three, subtract. All affine in the value with constant
coefficients, so a chain of any length composes to a single multiply-add, and LLVM performed that
composition: at `D = 8` the lazily-projected form emitted **one** vector `mla` rather than eight. The
eager form could not collapse, because a mask is not affine. So the sweep compared a chain the optimiser
had deleted against one it had not.

I found it by reading the emitted code for a number that looked too good, `native` flat at 390 ns from
`D = 2` to `D = 16`, which works out to 336 Gops/s on a machine that cannot issue that many. Those
numbers were discarded, not reported. The fix is a four-step cycle with an exclusive or in it, which is
bitwise and therefore commutes with reduction modulo `2^W` for any operand below `2^W`, so the eager and
lazy forms still agree, asserted rather than assumed. After the fix `D = 8` emits ten vector multiplies
where `D = 1` emits none.

### Defect two: the saturating fold was constant

The first saturating section reported the `kernel` arm at 3 to 4 ns for 8192 elements, which is about
2000 elements per nanosecond. The harness's own end-to-end column agreed at 62 ns, so it was not an
inner-timing artifact, and the same dylib on the wrapping rows tracked `native` to within 1%, so the
plumbing was sound.

The disassembly said what it was. For `D = 1` and `D = 3` the arm sets `x0 = -1` and jumps straight to
the epilogue, never touching the data. With an operand near a third of the container's range the
saturating accumulator pins at the limit after a handful of elements and the answer stops depending on
the input, and LLVM proved it and deleted the loop.

That is a defect in my workload, not in the kernel: a saturating fold over 8192 terms always saturates
at every width this bench sweeps, so it was never measuring saturation. **All six rows of
`precise-container-width-l1_*` are void.** They are left in the tree rather than deleted, because the
artifact trail should carry the mistake, and they are named as void here and nowhere cited. The
saturating question is re-measured elementwise in section 10, which is the shape `140`'s claim was
actually about.

I report both at length because they are the failure this whole exercise exists to correct. A number
wrong in a direction you like is the hardest kind to notice, and neither prior dispatch had a mechanism
that would have caught either.

## 4. Attempt one: price the fork

Medians in nanoseconds per call, 8192 elements, four passes of forty batches per arm. Full
distributions, per-pass series, bootstrap intervals and environment metadata are in the committed CSV,
meta and findings files beside `bench.toml`.

**Wrapping reduction, three operations per element** (`warm-container-width-l1`):

| W | headroom | minimum | plusone | native | kernel | lanes-deferred | h/m |
|---|---|---|---|---|---|---|---|
| 8 | 8415 | 190 | 8328 | 187 | 424 | 424 | **44.3x** |
| 13 | 8387 | 8591 | 8630 | 397 | 826 | 829 | 0.98 |
| 16 | 8351 | 397 | 8712 | 396 | 802 | 807 | **21.0x** |
| 32 | 5718 | 813 | 5715 | 802 | 896 | 898 | **7.0x** |
| 60 | 8385 | 8447 | 8443 | 2423 | 2226 | 2232 | 0.99 |
| 64 | 6015 | 2459 | 6073 | 2446 | 2255 | 2241 | **2.4x** |

**The same sweep at 1048576 elements** (`warm-container-width-l2`, four arms):

| W | headroom | minimum | native | h/m |
|---|---|---|---|---|
| 8 | 1078137 | 26496 | 26440 | **40.7x** |
| 13 | 1080565 | 1079468 | 54688 | 1.00 |
| 16 | 1071854 | 52554 | 53221 | **20.4x** |
| 32 | 738955 | 106560 | 106430 | **6.9x** |
| 60 | 1101298 | 1089092 | 331699 | 1.01 |
| 64 | 788104 | 329819 | 321628 | **2.4x** |

Two things fall out immediately.

**The container separates at exactly-filled widths and nowhere else.** 44.3x, 21.0x, 7.0x, 2.4x at 8, 16,
32 and 64. At 13 and 60 it is 0.98 and 0.99, inside the control gap. Two dispatches argued about a rule
whose effect at sixty of the sixty-four widths below the wide crossing is nothing.

**It is not a footprint effect.** At `W = 64` the minimum column is 8 MiB and fits this host's 12 MiB L2
while the headroom column is 16 MiB and does not, and the ratio moves from 2.4x to 2.4x. At `W = 8` it
moves from 44.3x to 40.7x. `140:331-336` predicted that crossing the cache levels is "where a doubled
footprint stops being a constant factor"; the prediction fails, and the whole effect is instruction
level.

**The size of the effect is monotone in the narrowness of the width**, which a footprint story does not
predict and an instruction-selection story does.

## 5. Attempt two: name the mechanism, from the emitted code

`141_probes/` holds a crate exposing one symbol per case so each arm's machine code can be read on its
own rather than hunted inside a 40-key dispatch. It carries no timings; it imports the bench's own
transform by path, so nothing there is a second copy of the kernel. `p141_asm.s` is the emitted
assembly, committed.

At an exactly-filled width in the minimum container the eager and lazy forms are not similar, they are
the same code, and the assembler says so:

```
_lazy_w64_min_d1 = _eager_w64_min_d1
_lazy_w64_min_d8 = _eager_w64_min_d8
```

`mask_to(64)` on a `u64` is the identity, the branch folds, nothing is emitted, and the loop vectorises.
That is `140:176-206`'s claim, exactly true, in the case where it holds.

Below the rung it is a different program. The whole of `eager_w13_min_d1`:

```
_eager_w13_min_d1:
        cbz     x1, LBB2_4
        mov     x8, x0
        mov     w0, #0
        lsl     x9, x1, #1
LBB2_2:
        ldrh    w10, [x8], #2
        add     w11, w2, w0
        add     w10, w10, w11
        and     w0, w10, #0x1fff
        subs    x9, x9, #2
        b.ne    LBB2_2
        ret
```

Six instructions, one element per iteration, a serial dependence through `w0`, no vector register
anywhere. The lazily-projected form of the same function vectorises to `add.8h` at eight lanes. The
accumulator's own projection is what does it: the reduction operator becomes `(a + b) & M`, which is
associative but which LLVM does not recognise as a reducible operator, so the loop is not vectorised at
all.

That explains every row above, including the ones about the container. **The headroom container is slow
at exactly-filled widths for the same reason the eager form is slow at sub-rung widths, because the
headroom container turns every width into a sub-rung width.** At `W = 8` the minimum container is a `u8`,
the mask is the identity, and it runs at 190 ns; the headroom container is a `u16`, the mask is real, the
reduction serialises, and it runs at 8415 ns. Same source, same data, one bucket of difference in the
container.

## 6. Attempt three: fix the losing arm by deferring the projection

The mechanism says the accumulator's projection is what serialises the loop. Reduction modulo `2^W` is a
ring homomorphism for `+`, `-`, `*` and `<<`, and exclusive or is bitwise, so every interior projection
may be dropped and one applied before the value is observed. That is not an approximation and not a
tolerance: the two forms produce byte-identical output over the whole swept matrix, asserted by
`eager_and_lazy_wrapping_agree_everywhere` and by the harness's own cross-variant comparison.

The `native` arm is that fix. It recovers the loss completely:

| W | eager (minimum) | deferred (native) | recovered |
|---|---|---|---|
| 8 | 190 | 187 | 1.0x, nothing to recover |
| 13 | 8591 | 397 | **21.6x** |
| 16 | 397 | 396 | 1.0x |
| 32 | 813 | 802 | 1.0x |
| 60 | 8447 | 2423 | **3.5x** |
| 64 | 2459 | 2446 | 1.0x |

Exactly the complement of the container result. The container matters at the four widths that fill a
rung; the projection placement matters at the sixty that do not; neither matters where the other does.
Across the density sweep at 13 bits the ratio runs from 24.6x at `D = 1` to 14.7x at `D = 16`, so it does
not depend on choosing a density, which is the thing `140:339-343` said an instruction count could not
settle and was right to say.

**And the reason `140` concluded no mechanism was needed is now precise.** `140:191-193` cites
`v_w13_reduce_eager`, a 64-element reduction, folding to a symbol alias. Sixty-four is a compile-time
constant small enough for rustc to fully unroll and then vectorise the straight-line result, and under
those conditions the masks really do fold. At 8192 elements with a runtime trip count they do not, and
the six-instruction scalar loop in section 5 is what is emitted instead. A harness bench sees this and a
thirty-line file compiled with `rustc -O` cannot.

## 7. Attempt four: fix it instead with a law the compiler will not use

The deferral works, but it changes what the program computes intermediately, and there is a second route
that does not. `(a + b) mod 2^W` is associative and commutative. A fold over it may therefore be
reassociated into independent lanes and combined at the end. LLVM does not do it, because with the `and`
interposed it does not recognise the accumulation as a reduction operator at all. arvo does know, because
the operator's algebra follows from the declared semantics and is available statically with nothing to
infer.

The `kernel` arm keeps the projection eager and splits the fold into eight lanes. It recovers most of the
same loss by a completely different route:

| W | eager serial | eager, eight lanes | deferred serial |
|---|---|---|---|
| 8 | 190 | 424 | 187 |
| 13 | 8591 | 826 | 397 |
| 16 | 397 | 802 | 396 |
| 32 | 813 | 896 | 802 |
| 60 | 8447 | 2226 | 2423 |
| 64 | 2459 | 2255 | 2446 |

Three readings, and the third is the one worth carrying.

**The law works.** At 13 bits the reassociation takes 8591 ns to 826 ns, a 10.4x recovery, from nothing
but knowing the operator is associative.

**It is beaten by deferral where deferral applies**, 826 against 397, a factor of 2.1.

**It wins where deferral runs out**, at 60 and 64 bits, by about 9%: 2226 against 2423, and 2255 against
2446. Those are the widths where the deferred form is still one serial dependence chain and this target
has no 64-bit vector multiply, so lane-level parallelism is the only parallelism left.

**And it loses badly where the compiler could already do the job.** At 8, 16 and 32 bits the hand-fixed
eight-lane shape runs 2.2x, 2.0x and 1.1x slower than simply letting LLVM vectorise the deferred form,
because eight fixed accumulators is worse than the sixteen `u8` lanes or eight `u16` lanes it would have
chosen. That is the microkernel lesson measured rather than asserted: a fixed kernel shape beats the
compiler only where the compiler declines, and loses to it everywhere else.

## 8. Attempt five: compose the two levers

They look independent, so the obvious next move is both at once. The `lanes-deferred` arm defers the
projection and splits the fold.

| W | kernel (eager, lanes) | lanes-deferred | difference |
|---|---|---|---|
| 8 | 424 | 424 | 0.0% |
| 13 | 826 | 829 | 0.4% |
| 16 | 802 | 807 | 0.6% |
| 32 | 896 | 898 | 0.2% |
| 60 | 2226 | 2232 | 0.3% |
| 64 | 2255 | 2241 | 0.6% |

**They do not compose. They are the same fix.** Every cell is inside the control gap. Once the fold is
lane-split, whether the projection is eager or deferred stops mattering, because the projection was only
ever costing anything by being on a loop-carried dependence and lane-splitting takes it off one.

This is a negative result and it is the most useful structural thing in the file after section 5, because
it collapses what looked like a two-dimensional decision into one. There is one question, "does this fold
have a serial dependence through a projection", and two answers to it, and picking either is enough.

## 9. Attempt six: use a range fact to delete a check, which loses

`140:222-235` argues that at an exactly-filled width the machine's saturating add is the semantics and no
construction over a wider container can be it. The dual of that argument is that **below** a filled width
the container's spare bits make the addition unable to overflow, so the overflow detection inside
`saturating_add` is dead and the operation is an add and a clamp. arvo can license that, because it knows
every value of a `W`-bit numeral is below `2^W` and it knows the container width. LLVM cannot: it sees a
load from a machine type and must assume the full range.

The `kernel` arm's saturating path takes the shortcut where `W < C` and falls back to the machine's
saturating add where `W == C`, chosen by a const comparison. It is semantics-preserving, which the
all-arms agreement test confirms at every key.

It loses:

| W | honest `saturating_add` | range-witness shortcut | |
|---|---|---|---|
| 8 | 142 | 170 | **1.20x slower** |
| 13 | 338 | 426 | **1.26x slower** |
| 16 | 256 | 255 | equal, shortcut not applicable |
| 32 | 504 | 503 | equal, not applicable |
| 60 | 1673 | 2299 | **1.37x slower** |
| 64 | 1000 | 994 | equal, not applicable |

The reason is that the check I removed is not a check. On this target `uqadd` is one instruction that
performs the saturating add outright, in scalar and vector form, and the replacement is an add and a
`umin`, which is two. The static knowledge was real, the licensed rewrite was sound, and it made the code
slower because it was reasoning about an abstract cost model rather than about the instruction set.

**This is the counter-example that keeps the rest of this file honest.** Two rewrites in this
investigation were licensed by facts arvo holds and the compiler does not. One of them, in section 12, is
worth 32x. This one is worth minus 26%. Nothing about the license predicts which.

## 10. Attempt seven: the saturating question, done again after the first answer was void

Section 3 records why the first saturating run is void: the fold's answer was input-independent and LLVM
deleted the loop. Redone as an elementwise transform, which is the shape `140`'s claim is about, the
answer is clean and it reverses what the void run appeared to say.

`precise-elementwise-width-l1`, four operations per element:

| W | headroom | minimum | h/m |
|---|---|---|---|
| 8 | 436 | 142 | **3.1x** |
| 13 | 696 | 338 | **2.1x** |
| 16 | 661 | 256 | **2.6x** |
| 32 | 1660 | 504 | **3.3x** |
| 60 | 10139 | 1673 | **6.1x** |
| 64 | 8211 | 1000 | **8.2x** |

**`Precise` does want the deletion, and more strongly than `Warm` does in the same regime.** Warm's
elementwise container ratios over the same widths are 2.6x, 1.9x, 2.0x, 5.9x, 1.0x and 1.9x. Precise's
are larger at five of six and much larger at the top two, where the headroom container is a `u128` and
there is no saturating instruction to reach at all.

So `140:222-235` is vindicated, on the shape it was about, and the void run that appeared to refute it
should not have been believed in either direction. I record the reversal in the open rather than quietly
reporting only the good version.

## 11. Attempt eight: the affine collapse, which is the law buying an entire optimisation

Section 3 calls the affine step chain a defect in my workload. It is, as a density measurement. As a
finding it is the largest thing in this investigation after section 12, and discarding it as merely a
defect would have thrown it away.

A chain of affine steps composes to one multiply-add, by ordinary algebra. LLVM performs that composition
when it can see the chain. With a projection between the steps it cannot, because a mask is not affine
and the compiler has no license to move it. arvo does have that license, because reduction modulo `2^W`
is a ring homomorphism. So removing the interior projections does not merely delete a mask, it hands the
chain to the optimiser in a form it can collapse.

`warm-affine-density-w13`, an affine-only chain at 13 bits, against the same sweep with an exclusive or
inserted to block the collapse:

| D | affine, eager | affine, deferred | mixed, deferred |
|---|---|---|---|
| 1 | 7946 | 332 | 322 |
| 2 | 8417 | 395 | 392 |
| 4 | 8365 | 410 | 257 |
| 8 | 13418 | 402 | 416 |
| 16 | 13248 | **401** | 752 |

**The deferred affine form is flat.** Sixteen operations per element cost what two do, 401 ns against
395, because after the interior projections are gone the whole chain is one multiply-add and the loop
cost is the memory pass. The mixed chain, which cannot collapse, grows to 752 ns. The eager affine form
grows to 13248 ns and pays for every operation, so at `D = 16` deferral is worth **33x** on this shape.

A fixed-point filter, a colour transform and a scale-and-bias pass are all this shape, so this is the
common case rather than a constructed one. And the size of the win is not the mask. It is the algebra the
mask was standing in front of.

## 12. Attempt nine: a theorem the compiler cannot know, which is the largest result here

Op's steer was to reach for the mathematical corpus, on the reasoning that arvo now holds the laws in
statically analysable form and some equivalent formulation may compute better in a way that is not
obvious. The strongest instance I found is not a reformulation of an operation. It is a theorem that
deletes one.

**The theorem.** Every element of a `W`-bit column is below `2^W`, and there are `N` of them, so the exact
sum is below `N * 2^W`, which is at most `2^(W + ceil(log2 N))`. When that bound is at or below the
accumulator's width, a saturating accumulation cannot reach its saturation point. Every clamp in the fold
is then dead and the operation is a plain wrapping sum, with identical results.

Both premises are things arvo holds and the machine code cannot carry. The element bound is the declared
width, which lives in the type and does not survive into the loaded value: LLVM sees a load from a `u16`
and must assume the full range of `u16`. The count bound is the column's capacity, which arvo carries as
a `Cap` and which LLVM sees as a runtime slice length.

**What it buys is not an instruction.** A saturating fold is a loop-carried dependence through an operator
LLVM will not reassociate, so it is serial at every width and in every container. Deleting the saturation
turns it into a plain wrapping reduction, which vectorises. The theorem does not make the loop cheaper, it
changes which loop is compiled.

`precise-widening-theorem-l1`, a `W`-bit column accumulated saturating into 64 bits, 8192 elements:

| W | theorem holds | honest fold | with the theorem | |
|---|---|---|---|---|
| 8 | yes | 5324 | 165 | **32.3x** |
| 13 | yes | 5293 | 679 | **7.8x** |
| 16 | yes | 5235 | 701 | **7.5x** |
| 32 | yes | 5315 | 674 | **7.9x** |
| 60 | **no** | 5502 | 5490 | 1.00x |
| 64 | **no** | 5446 | 5228 | 1.04x |

The predicate is `W + ceil(log2 N) <= 64`, a `const fn` of exactly the two quantities the typestate
already holds. At 8192 elements it is true up to 51 bits and false above, and the last two rows are the
same code because the arm falls back automatically. **The cell boundary is not a heuristic and not a
tuning parameter. It is where a theorem stops being true, evaluated at compile time.**

That is the shape of what op was asking whether exists. It exists, it is worth between 7.5x and 32x, it
is invisible to the compiler for a reason that is structural rather than a missing optimisation, and it
turns off by itself where it does not hold.

I want to be exact about one thing, because it is the difference between a result and an overclaim. This
is measured on **one** theorem, in **one** shape, on **one** target. It is an existence proof that the
category is real and worth something large. It is not evidence about how many such theorems there are,
nor that the next one pays, and section 9 is a fully worked instance of a licensed rewrite that costs
26%. The category is worth building for; each member of it still has to be benched.

## 13. Attempt ten: the one cell nothing recovers, and why an assembly kernel would not either

The elementwise sweep leaves one loss unexplained. At 60 bits it runs at 3466 ns against 267 ns at 13
bits, with the same element count and the same operation count, and the emitted code for the 64-bit cases
contains no vector register at all. Both containers are equally slow there, which is why the container
ratio at `W = 60` is 1.0x and looks like good news and is not.

Isolated one operation at a time in `141_probes/`:

```
op_addxor_u64          vec_ops=9        add and exclusive or vectorise
op_addxor_u16          vec_ops=12
op_mul3_u64            vec_ops=0        multiply by three does not
op_mul3_u16            vec_ops=7
op_mulk_u64            vec_ops=0        multiply by a runtime value does not
op_mulk_u16            vec_ops=5
op_mulk_u64_split      vec_ops=0        the 32-by-32 decomposition does not either
```

So any multiply at 33 to 64 bits drops the whole loop off the vector path, and writing the standard
32-by-32 decomposition in plain Rust does not recover it: LLVM keeps the pieces in 64-bit registers and
never narrows them to the `.2s` inputs `umull` needs.

Next attempt: write the decomposition with NEON intrinsics directly, which is what a hand-written kernel
would do. `op_mulk_u64_neon` in `141_probes/` uses `vmovn_u64`, `vshrn_n_u64`, `vmull_u32` and
`vaddq_u64`. The emitted code:

```
LBB27_3:
        ldp     x16, x17, [x13, #-32]
        mul     x17, x17, x9
        mul     x16, x16, x15
        fmov    d1, x16
        mov.d   v1[1], x17
        stur    q1, [x14, #-32]
```

**LLVM recognised the decomposition as the operation it decomposes, undid it, and emitted scalar
multiplies with vector loads and stores around them.** So intrinsics do not recover it either, and for a
reason that generalises: the compiler can see through any source-level identity, which is usually what
you want and here is precisely what defeats the rewrite.

That leaves `asm!`, which the compiler cannot see through, and this is where the attempt stops, on
arithmetic rather than on effort. The vector form needs `xtn`, `xtn`, `shrn`, `shrn`, `umull`, `umull`,
`umlal`, `shl`, `add` for two lanes, which is about 4.5 instructions per element. The scalar form is one
`mul` per element and this core has two integer multiply units. **A hand-written assembly kernel for this
cell would be roughly four times slower than what LLVM already emits, so it is not worth writing and I did
not write it.**

The honest conclusion is that this cell is a hardware limitation rather than a compiler failure. NEON has
no 64-by-64 multiply. The right response is not a microkernel but to know the number and put it in the
table: at 33 to 64 bits, an operation sequence containing a multiply costs about 13x the same sequence at
16 bits, whatever the container and whatever the strategy, and no rewrite available to arvo changes it.

## 14. On copy-and-patch and pre-compiled stencils

Op raised whether arvo could carry pre-compiled kernels patched with the values at use, in the shape of a
copy-and-patch JIT. I have built one of those and I do not think it belongs here, and the reason is
visible in every measurement above rather than a matter of taste.

Copy-and-patch exists because at run time you cannot afford a compiler backend. Its stencils are
pre-compiled precisely so that no optimiser has to run when the code is assembled, and the price of that
is opacity: a stencil cannot be inlined into its caller, cannot be fused with adjacent work, cannot have
its loads hoisted, and cannot be vectorised together with the loop around it. In LuaJIT Remake that price
is worth paying because the alternative is an interpreter dispatch or a real backend at run time.

arvo compiles ahead of time. LLVM is already the backend, with the whole consumer loop in view. So a
stencil buys nothing that monomorphisation does not already buy, and it costs exactly the property that
produced every win in this file. Section 6's 21.6x is LLVM vectorising a reduction once the mask left the
accumulator. Section 11's 33x is LLVM collapsing an affine chain once the interior projections were gone.
Section 12's 32x is LLVM vectorising a fold once the clamps were deleted. **All three are the optimiser
doing something large with a loop it can see, and a stencil is a loop it cannot see.** Section 7 measures
the cost of freezing a kernel shape even without opacity: 2.2x worse than the compiler's own choice at
every width where the compiler could do the job.

There is exactly one cell in this investigation where opacity would be an advantage, and it is section 13,
where the compiler defeats the rewrite by seeing through it. And section 13's own arithmetic says the
rewrite loses by a factor of four anyway. So the one place a stencil would help is the one place there is
nothing to help with.

**What survives from the idea, and it survives in a strong form, is the part that is not the pre-compiled
binary.** Recognise the situation statically, emit a different body for it, and let it be monomorphised
source the optimiser can still see. That is what the `kernel` and theorem arms are, it is where the 32x
came from, and it is copy-and-patch's actual insight (one semantic definition, many derived lowerings)
without the mechanism that only makes sense at run time.

## 15. The pattern underneath all of it

Op's steer was to look for a bigger pattern than individual instructions, and to ask whether the
mathematical corpus, now that the laws are in statically analysable form, supplies computations that are
better in ways that are not obvious. The measurements above are all instances of one pattern, and it is
worth stating as a pattern because the individual cells will change with the target and the pattern will
not.

**arvo's typestate holds facts that do not survive into the machine code, and each such fact licenses a
rewrite the compiler is forbidden from performing. The corpus is the catalogue of those rewrites. Which
of them are worth performing is not a mathematical question and cannot be answered from the corpus.**

The facts, from what was actually used here:

- Reduction modulo `2^W` is a ring homomorphism. Licenses dropping interior projections. Worth 21.6x on a
  reduction, 33x on an affine chain, nothing at an exactly-filled width.
- Wrapping addition modulo `2^W` is associative and commutative. Licenses reassociating a fold into lanes.
  Worth 10.4x where the compiler declines, minus 120% where it does not.
- Unsigned saturating addition is associative and commutative. Licenses the same for `Precise`. Unpriced,
  because the workload that would show it saturates and folds to a constant.
- Every value of a `W`-bit numeral is below `2^W`. Licenses deleting overflow detection when the container
  is wider. Worth minus 26%.
- A sum of `N` values below `2^W` is below `2^(W + ceil(log2 N))`. Licenses deleting saturation outright.
  Worth 7.5x to 32x.

Five laws, five licensed rewrites, and their measured values span from 32x to minus 26%. **The license and
the payoff are unrelated.** Nothing in the algebra distinguishes the theorem in section 12 from the range
fact in section 9; both are true, both are statically available, both are sound, and one is worth thirty
times and the other is worth less than nothing.

So the pattern is a two-stage one and both stages are required. The corpus generates candidates, cheaply
and mechanically and in whatever quantity the laws support. **The harness decides**, per candidate, per
width band, per operation shape, per target. A corpus-driven rewrite that has not been benched is a
hypothesis with a proof attached, and the proof is about correctness rather than about cost.

Three sharper observations, which are what I would carry forward rather than the individual numbers.

**The wins are where the rewrite changes what kind of loop is compiled, not where it removes work.**
Section 6 removes one `and` per element and is worth 21.6x, because the `and` was preventing
vectorisation. Section 12 removes one `csel` per element and is worth 32x, because the `csel` was
preventing vectorisation. Section 9 removes one instruction per element and is worth minus 26%, because
nothing was being prevented. The predictor is not instruction count, it is whether the removed operation
sits on a loop-carried dependence or blocks a legality check the vectoriser performs.

**The compiler's ability to see through a rewrite is usually the point and occasionally the enemy.**
Everywhere in this file, handing LLVM a simpler form let it do something larger than the simplification.
In section 13, handing it a decomposition let it undo the decomposition. Both follow from the same
property.

**A law that is true at every width will still only pay at some of them.** The homomorphism holds at 8
bits and buys nothing, because the projection there is already a no-op. Selection has to be per cell, and
the cell boundaries are things like "does `W` fill its container" and "is `W + ceil(log2 N) <= 64`", which
are const predicates over quantities the typestate already carries.

## 16. The composition, as a table

Which answer wins, per cell, for the shapes measured. Every predicate is decidable at compile time from
the declared width, the container width, the element count and the operation kind.

| regime | container | kernel shape | measured |
|---|---|---|---|
| wrapping reduction, `W` fills its container | minimum | anything; eager and deferred are one symbol | container worth 2.4x to 44.3x |
| wrapping reduction, `W` below its rung, `W <= 32` | minimum, worth 1.0x | **defer the projection** | 21.6x, and lane-splitting is 2.1x worse |
| wrapping reduction, `W` below its rung, `W` in 33..=64 | minimum, worth 1.0x | **lane-split**, either projection | 3.5x from deferral, a further 9% from lanes |
| wrapping affine chain, any sub-rung `W` | minimum | **defer**, which unlocks the collapse | up to 33x at `D = 16`, and flat in `D` |
| wrapping elementwise | **minimum**, worth 1.8x to 5.9x | either; the projection costs 0 to 10% | container is the whole story |
| saturating elementwise | **minimum**, worth 2.1x to 8.2x | machine `uqadd`; do not open-code it | the range shortcut costs 20% to 37% |
| saturating fold, `W + ceil(log2 N) <= 64` | minimum | **delete the saturation by theorem** | 7.5x to 32x |
| saturating fold, otherwise | minimum | honest clamped fold; lane-split unpriced | theorem correctly declines |
| any operation containing a multiply, `W` in 33..=64 | irrelevant, 1.0x | none available | about 13x versus 16 bits, hardware |

The container column has one value in every row. **That is the answer to whether the fork is a matrix: it
is not.** The matrix is real and it is entirely in the kernel column.

## 17. Dead routes, with what closed each

The most useful part of an investigation is usually the list of things that did not work, so the next
person does not spend the afternoon on them.

1. **Headroom container.** Loses 1.8x to 45.3x across every regime measured honestly. Never won a cell
   that survived checking.
2. **`rung(W+1)`.** Not a distinct container at any width from 1 to 64. Closed by a test over all 64, not
   by measurement.
3. **Left-justified representation** (`140` section 4). Not retested; closed there on the `Transparent`
   unwrap door, which is a correctness argument and does not need re-pricing.
4. **Keeping the projection eager with a serial fold.** The 21.6x loss. This is the shipped behaviour.
5. **Lane-splitting with the projection eager.** Recovers 10.4x of it, then loses 2.1x to deferral at
   `W <= 32` and wins 9% at 60 and 64.
6. **Deferral plus lane-splitting.** Identical to lane-splitting alone within 0.6% at every width. The two
   levers are one lever.
7. **Range-witness shortcut for saturating add.** 1.20x to 1.37x slower. The removed check is one
   instruction on this target.
8. **Saturating fold as a workload.** Constant-folded by LLVM at every width; six committed rows are void
   and are named as such.
9. **Affine step chain as a density workload.** Collapsed by the optimiser; discarded as a density
   measurement and repurposed as section 11.
10. **Source-level 32-by-32 decomposition of a 64-bit multiply.** Zero vector operations emitted.
11. **The same decomposition with NEON intrinsics.** LLVM canonicalises it back to a 64-bit multiply and
    lowers it scalar.
12. **An `asm!` kernel for the same cell.** Not written: about 4.5 instructions per element against one
    scalar `mul`, so it loses by roughly four before it is built.
13. **Copy-and-patch stencils.** Not built; section 14 gives the reasoning, and the one cell where opacity
    would help is item 12, which has no win in it.
14. **Footprint as the explanation for the container gap.** Refuted by the large-N sweep: crossing a 12 MiB
    L2 moves 44.3x to 40.7x and 2.4x to 2.4x.

## 18. What is unmeasured, and what would need more than I had

Named so none of it reads as established.

**One target.** Everything is aarch64 NEON on one Apple M1. x86-64 will differ in at least three places
that matter: `uqadd`'s availability and cost, whether a 64-bit vector multiply exists (AVX-512 has
`vpmullq`, so section 13's wall may simply not be there), and lane counts. The cells will move; I would
expect the pattern in section 15 to hold and the table in section 16 to need re-running.

**Instruction and cycle counts are zero in every committed CSV.** The harness has `instructions` and
`cycles` columns and populates neither, which needs elevated privileges for the performance counters on
this platform. With them, section 15's claim that the predictor is "does the removed operation block
vectorisation rather than cost an instruction" could be tested directly rather than inferred from
disassembly plus wall time. That is the single highest-value thing a follow-up with `sudo` could add, and
it would also settle section 9's mechanism, where I am inferring `uqadd`'s cost rather than counting it.

**The saturating fold's lane-parallel form is unpriced.** The reassociation is legal and the arm exists
and agrees, but the only workload I could build for it saturates and gets constant-folded. A workload
where a saturating fold neither pins nor is trivial needs the element magnitudes bounded relative to the
term count, which is exactly the condition under which section 12's theorem applies and deletes the
saturation entirely. So it is possible this cell is empty by construction, and I did not establish that
either way.

**Signed numerals.** Everything here is unsigned. `140:236-241` reports that signed sub-rung wrapping
needs two vector instructions rather than one because NEON has no vector bitfield extract, so the signed
side may have a different balance between deferral and reassociation. Not run.

**`Cold`.** Untouched. It is a different representation rather than a container choice.

**The wide side above 128 bits.** Untouched; `137b:48-53` governs there.

**Compile time.** Not considered anywhere. A rewrite catalogue selected by const predicates has a
monomorphisation cost, and section 12's arm compiles both forms of the fold and discards one at every
instantiation. At the scale of this bench it was not noticeable; at arvo's scale it might be.

## 19. Where I concede

**I did not find a cell where the shipped headroom rule wins.** I looked for one, because op's read was
that the answer might be a mixture, and the two cells that appeared to show it (13 and 16 bits under
saturating semantics) came from the void run. I can name an unmeasured candidate: a value multiplied far
more often than it is loaded or stored and never vectorised, where the storage width is paid rarely and
an in-register widening is paid often. Every workload here streams a column, so loads dominate and that
candidate is untested. I would not bet on it.

**I could not price the saturating fold's reassociation**, for the reason in section 18, and I do not
have a workload design that closes it. That one may want someone else.

**I did not settle whether the deferred projection should become a typestate.** It is worth 21.6x and up
to 33x, it is semantics-preserving, and `140`'s reason for saying no mechanism is needed does not survive.
But it is a typestate question and I read codegen; someone who reasons about `135b`'s erasure gate should
say whether a derived "projection pending" state can be introduced, validated and erased with no caveat.
My number should not substitute for that reading.

## 20. What is op's

**The container call.** Delete the headroom. It is the second of the two options refused at `139b:21-25`,
and I am not restating it unchanged: what it lacked was a price, and the price is 2.4x to 44.3x on a
wrapping reduction, 1.8x to 5.9x elementwise, and 2.1x to 8.2x under saturating semantics, with the arms
validated against each other and an independent oracle on every key, the noise floor measured rather than
assumed, and the mechanism read off the disassembly. The strongest form needs none of my numbers: under
the shipped rule no width ever fills its container, so the case op's own intent statement describes, a
native primitive's operation with nothing around it, is unreachable at every width by construction.

**Whether the projection becomes a typestate.** Worth 21.6x, and up to 33x on affine chains. Section 19
says why I am not the one to settle it.

**Whether the corpus becomes a rewrite catalogue.** Section 15 is the case for it and section 9 is the
case against doing it without a bench per candidate. The category contains at least one member worth 32x
and at least one worth minus 26%, and nothing in the mathematics tells them apart. If it is built, the
selection predicates are const functions over quantities the typestate already carries, and each entry
owes a harness row before it ships.

**`131:277`'s "single-operation overflow room" is a correction rather than a call.** It names a mechanism
that does not do that job: the projection to `W` is required in both containers, so a consumer can never
observe the extra bit.

**And the framing I would most want attacked is my own.** I read op's steer about the corpus as "find
laws that license rewrites" and found five. The larger reading, which I did not pursue, is that the corpus
might supply not rewrites of arvo's operations but different *operations*, where a numerically equivalent
formulation with a different instruction mix is chosen at the point the type is declared. I have no
evidence either way and I did not build anything for it, and someone who thinks in identities rather than
in loops should look at it before section 15 gets written down as the shape of the thing.

## 21. Every steer, and its why, written down so a droplist can take it

Op's steers during this dispatch named several angles. Some paid, some did not, and one I did not
attempt. Each is recorded with the reasoning rather than only the outcome, because a suggestion that
turned out badly is only useful to the next reader if the reason travels with it.

**"Use the whole matrix of situations, decide statically, mix container rules per case."** Partly right
and the correction is the useful half. The matrix is real, it has at least nine cells, and every
predicate that selects a cell is a const function of quantities the typestate already carries. But it is
**not on the container axis**: minimum wins or ties in all nine, so there is no case where mixing
container rules is the answer. The axis that genuinely flips is which kernel shape to emit, and it flips
three ways inside a single width sweep. Section 16 is the table. The why: the container decision affects
every load, store and lane for the life of a value, so its effects are monotone in the container's size,
while the kernel decision affects one loop and can therefore trade differently per loop shape. A
monotone axis has no matrix in it.

**"Even pure assembly microkernels, where the typestate proves what the machine cannot see."** Tried, in
the only cell where LLVM genuinely declines, and it prices out before it is written. Section 13: NEON has
no 64-by-64 multiply, so a vector form needs about 4.5 instructions per element against one scalar `mul`
on a core with two multiply units. The why, generalised: **an assembly kernel is worth writing only where
the hardware has an instruction the compiler is not selecting, not where the compiler is declining
because the hardware has nothing to select.** The second case looks identical from the Rust side (no
vector registers emitted) and is the opposite situation. Section 7 adds the milder version of the same
lesson with numbers: a hand-fixed eight-lane shape is 2.2x slower than the compiler's own choice wherever
the compiler could already vectorise, so a microkernel is a liability outside the cells that need it.

**"Run without checks, because we know the bounds statically."** Tried and it lost, 1.20x to 1.37x
(section 9). The why is worth carrying because it is counter-intuitive in exactly the direction that
makes it dangerous: the removed "check" was not a check. `saturating_add` on this target is `uqadd`, one
instruction, and the range-licensed replacement is an add plus a `umin`, which is two. **Reasoning about
statically-provable checks in terms of an abstract cost model rather than the instruction set produces a
sound rewrite that is slower.** The general form: before deleting a guard because a proof licenses it,
look at whether the guard exists in the emitted code at all.

**"The mathematical corpus has equivalent ways to compute, and some may vectorise better without it being
obvious."** Right, and it is the largest result here. Two instances measured: the ring-homomorphism
rewrite in sections 6 and 11 (up to 33x) and the no-saturation theorem in section 12 (up to 32x). The why
for both is one sentence: **they win where the rewrite changes what kind of loop is compiled, not where
it removes work.** Section 15 states the pattern and its two-stage discipline, and section 9 is the
counter-example inside the same category, which is what keeps the pattern from becoming a licence.

**"Shorthand theorems that math knows but instructions could not express."** This is section 12 exactly,
and it is the sharpest form of the idea. The theorem is not a faster way to compute a saturating sum. It
is a proof that the saturating sum is a different operation, which then compiles differently. The why it
is invisible to the compiler is structural rather than a missing optimisation: both premises, that
elements are below `2^W` and that there are at most `N` of them, are erased at the load. No amount of
LLVM improvement recovers them, because the information is not in the program it is given.

**"Instruction-level counters, with elevated privileges if needed."** Not attempted, and I should have
asked for it earlier rather than inferring from disassembly plus wall time. Section 18 says what it would
settle. The why it matters specifically here: section 15's central claim is that the predictor of a
rewrite's value is whether the removed operation blocks vectorisation rather than what it costs, and that
claim is currently supported by reading emitted code beside timings. An instruction count would test it
directly, and it would also settle section 9's mechanism, where I am asserting `uqadd` is one instruction
rather than counting it. **This is the highest-value follow-up in the file and it needs nothing but
privileges.**

### Candidates I did not attempt, with why, so the list is not mistaken for exhaustive

**Reformulating an operation rather than deleting one.** Every rewrite here either drops something
(projections, clamps) or reorders something (lanes). None replaces an operation with a numerically
equivalent but differently-shaped one, for example a division by a compile-time constant as a
multiply by a precomputed reciprocal at a width the typestate knows, or a fixed-point reciprocal by
Newton iteration at a chosen intermediate width. Not attempted because I ran out of dispatch on the
deletion category, not because it looked unpromising. It is the natural next probe and it has the
advantage that arvo knows both the constant and the width statically, which is exactly what a reciprocal
rewrite needs and what a general compiler has only sometimes.

**Signed numerals.** Untouched, and `140:236-241` reports that signed sub-rung wrapping needs two vector
instructions where unsigned needs one. So the balance between deferral and reassociation may differ on
the signed side and none of section 16's table is established there.

**Reassociating a saturating fold.** The arm exists, is legal and agrees, and could not be priced because
every workload I could build for it either saturates immediately or falls under section 12's theorem and
stops being a saturating fold. It is possible this cell is empty by construction, and I did not establish
that either way.

**Whether the catalogue should be a catalogue at all.** Section 15 describes a pattern; it does not
propose a mechanism, and I deliberately did not design one. A rewrite catalogue selected by const
predicates is one shape; another is that each law lives with the trait it belongs to and the selection is
ordinary monomorphisation. Which of those arvo wants is a design question above my lane, and the measured
result (the category contains members worth 32x and members worth minus 26%) constrains it without
deciding it.
