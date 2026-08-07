# 142. What rides on the headroom, and the composition that replaces it

**Author:** Fabian Giesen (persona dispatch)
**Date:** 2026-08-07
**Position:** after `141_xu_the_container_fork_benched.md`, taking op's refusal to accept its deletion
verdict until someone has gone looking for what the headroom is load-bearing for.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2024, aarch64-apple-darwin, Apple M1.
**Evidence:** nine new harness bench sections in `mock/benches/bench.toml`, 44 committed CSV plus meta
plus findings triples, six new arm cdylibs plus one shared transform crate under
`mock/benches/variants/warm-clamp-*`, seven contract tests over the full 46-key matrix. A
code-inspection artifact carrying no timings at `142_probes/`.

STATUS: complete. Two rows of the L2 section (`W = 60` and `W = 64`) were still running when this was
written and are named as absent in section 9 rather than reported.

## Verdict, stated first

**Op's hunch is right, and the thing that rides on the headroom is not in the source and not in `141`.
It is in the standing base, ratified in full at `70b`, and no file in the container argument has read
it.**

The ratified fixed-point preset table (`124:2600-2608`) assigns `Warm` and `Cold` the resolution
**clamp**, not wrap. It assigns `Warm` and `Precise` `StoredWidth = doubled`, and it states the reason
in the same passage (`124:2613-2615`): "doubled storage lets a chain of operations retain more than one
operation's exactness before a narrow forces a decision."

That is an accuracy property across a chain. It is not the "single-op overflow headroom for Warm
wrapping" the source comment claims (`arvo-strategy/src/container.rs:15-16`), and it is not something
any arm in `141` could have measured, because every arm in `141` computes a byte-identical value and
`141`'s own cross-validation drops any arm that does not (`141:132-136`). **The headroom's ratified
purpose is to make one answer differ from another. The measurement apparatus that made `141`
trustworthy is the same apparatus that made the headroom's job structurally invisible to it.**

And then the resolution, which dissolves the fork rather than picking a side. The property the doubling
buys is **interior safety of a fold**, and the design already has a ratified, derived, exact rule for
it: `124` section 1.8, the `n-1` interior-safety condition and the accumulator numeral sized at
`W + ceil(log2 n)`, the DSP guard-bit rule the design cites from the Motorola 56000. The headroom is a
fixed, always-paid, per-value approximation of a derived, per-fold, frequently-free quantity, and it is
a bad approximation in both directions: too much at every width that never folds, and nowhere near
enough for any fold longer than `2^W` terms.

So the composition is not a container matrix. **Storage takes the minimum container, always, and the
room moves onto the accumulator, where the canon already put it and where the source never built it.**
Section 11 states the predicates and sections 6 through 10 measure them.

Four results carry that, and each is a reversal of something a prior file concluded.

**The container axis is not monotone.** `141:41` says "minimum wins or ties in every measured cell".
Under the ratified clamp resolution the minimum container wins by 4.5x at one cell and loses by 44x at
another, inside one width sweep. The cell `141` looked for and could not find (`141:744-749`) exists,
and there are eleven of them.

**And the minimum container was still never the thing that cost anything.** Supplying the
reassociation LLVM will not perform recovers 41.0x of the 44x, back to within noise of the doubled
container, on minimum storage. Two independent mechanisms recover the loss and neither is a container.

**The design's own accumulator rule beats both**, at 25 of 38 fold cells and never decisively worse
than the doubled container at any of the 44 rows with data, on half the storage.

**And halving the storage pays again once the column leaves cache**, up to 2.13x, which reverses
`141:226-230`'s footprint conclusion for the reason section 9 gives.

Three static levers were pulled and priced: the fold arity as a compile-time constant, worth up to
20.5x and never costing anything; the accumulator narrowed to the derived width rather than pinned at
64, worth 1.1x to 4.0x and inverting at 64 bits; and reassociating a saturating fold, worth up to
41.0x at high arity and costing up to 8.0x at low. Section 10.

## 1. What this brief takes for granted, checked before reasoning from it

**"`110_consolidation_eleven.md` is the standing base."** False. `124_consolidation_twelve.md` opens
with "This document replaces `110` as the sole reference for the design's current state" (`124:6-7`),
is dated a day later, and folds fifty-one inline corrections that `110` carried. I read `124` as the
base and cite it throughout. Nothing in this file rests on a passage where the two differ, but the
brief's claim is wrong and a later reader taking it would cite a superseded document.

**"`141` ran the first real harness benches on the container fork."** Confirmed. `mock/benches/bench.toml`
carries nine sections whose names begin `warm-container-`, `warm-elementwise-`, `precise-container-`,
`precise-elementwise-`, `warm-affine-` and `precise-widening-`, each with committed CSV, meta and
findings, and six arm crates under `variants/warm-container-*`.

**"Under the shipped rule no width ever fills its container."** Confirmed at
`arvo-strategy/src/container.rs:77-91` (`tag_warm_precise`) against `:60-75` (`tag_hot_cold`), and the
module documentation states the ladder in words at `:15-19`.

**"`Warm` wraps at `W`."** **This is the brief's, and `141`'s, and `140`'s, and `139`'s load-bearing
premise, and the standing base contradicts it.** Section 2.

**"The matrix lives entirely in the kernel column, because the container axis is monotone."** True of
the workload `141` measured and false in general, for the reason in section 3: `141`'s arm-agreement
test can only admit arms that agree, so the axis on which the container is not monotone was excluded by
construction.

## 2. `Warm` clamps. It does not wrap. This is ratified and four files have carried the opposite

The standing base's fixed-point preset table, ratified in full at `70b` (`124:2600-2608`):

| | `Hot` | `Cold` | `Warm` | `Precise` |
|---|---|---|---|---|
| in-range direction | `TowardNegative` | `ToEven` | `ToEven` | `ToEven` |
| `OverRange`/`UnderRange` | `ReduceModulo`/`ReduceModulo` | clamp | **clamp** | `Refuse`/`Refuse` |
| `StoredWidth` | minimum | minimum | **doubled** | **doubled** |
| `Layout` | dense | bitpacked | dense | dense |

And the prose immediately under it (`124:2615-2616`): "`Warm` and `Cold` both round nearest and clamp
for the identical reason (a type nobody expects to crash has no reason to accept truncation bias)."

`ReduceModulo` is wrapping and it belongs to `Hot` alone. Four consecutive files have built on the
opposite. `131:275-280` asserted `Warm` wraps at `W`; `139` and `140` carried it; `140:136-139` flagged
that it was untested and traced it to that one line; `141:78-81` then concluded the assertion "is right
for a better reason than the one they carried" by deriving it from op's `140b` intent statement, and
built all six arms as wrapping. **Six arm cdylibs, nine bench sections, 57 committed artifact triples,
and the `Warm` half of them measures `Hot`'s resolution.**

The `Precise` half is unaffected in kind: `Refuse` is not saturate either, but `141`'s saturating arms
are a closer stand-in for a refusing one than a wrapping arm is for a clamping one, and the
elementwise-saturating numbers survive as a proxy. The `Warm` numbers do not.

**And one part of `141` measured the right resolution under the wrong label.** Its
`precise-elementwise-width-l1` section is clamping arithmetic, which under the ratified table is
`Warm`'s resolution as much as `Precise`'s, and it reports the minimum container winning by 2.1x to
8.2x (`141:417-424`). Section 6's chain rows reproduce that independently at 2.07x to 2.35x, on a
different chain, with six arms instead of two. So `141`'s clamping result is corroborated rather than
displaced, and it is corroborated exactly where it holds: **elementwise, with no fold**. Its clamping
*fold* is the run it declared void (`141:180-186`), and the fold is where the container stops being
monotone. The two files disagree about one shape and agree about the other, which is the strongest
form the reversal in section 6 could take.

**And op's `140b` intent statement does not rescue the wrapping reading.** "It should behave like
native primitives in regular old rust would" (`140b:17-21`) is ambiguous between three behaviours a
Rust primitive actually has: panic in debug, wrap in release, and the explicitly-spelled
`saturating_add`. It is not ambiguous about clamp being excluded, but it does not select wrap either,
and the `70b` table already selected clamp with a stated ground. **Two ratified statements of op's, one
older and specific, one newer and general, and they do not agree.** That is not mine to resolve and
section 9 hands it back.

What follows in this file treats the ratified table as governing for `Warm`, because it is specific,
because it carries its own derivation, and because `140b`'s general statement was made about a
mechanism question (does `Warm` widen) rather than about the resolution row. Where the answer differs
under the wrapping reading I say so.

## 3. Why `141` could not have found this, and it is a property of the harness rather than a mistake

`141`'s trustworthiness rests on one thing, stated at `141:132-136`: "the harness compares outputs
byte-exactly across arms and drops any that disagrees", plus an independent `u128` oracle at every one
of forty keys. That is the correct discipline and it is what makes its numbers mean anything.

It is also a filter on which arms may exist. **An arm can only enter the set if it computes the same
value as every other arm.** So the container axis, as `141` measured it, is by construction restricted
to the sub-space where the container makes no observable difference to the answer. On that sub-space
the container is pure overhead, so of course it is monotone and of course the minimum wins: `141`
proved, rigorously and correctly, that **where the headroom changes nothing it costs between 1.0x and
44.3x.**

The ratified reason for the headroom is that it changes something. "A chain of operations retains more
than one operation's exactness before a narrow forces a decision" (`124:2613-2615`) is a statement that
the eagerly-narrowed and the deferred-narrow answers **differ**, and that the design wants the second
one. Under `Hot`'s `ReduceModulo` they do not differ, because reduction modulo `2^W` is a ring
homomorphism, which is the whole of `141` sections 6 and 11 and is correct. Under `Warm`'s clamp they
differ, because clamping is not a homomorphism.

So the one axis on which the container is not monotone is the one axis whose arms `141`'s validation
had to reject. This is not a defect in `141`. It is the cost of a discipline that is otherwise exactly
right, and the repair is to bench a workload where the arms are allowed to disagree and are validated
against a *specification* rather than against each other. Section 7 does that.

## 4. Where clamping is and is not deferrable, which is the theorem the headroom was standing in for

Take a chain of `k` additions of values from a `W`-bit unsigned numeral, resolution clamp, limit
`L = 2^W - 1`, computed in a container of width `C >= W`.

**Clamping is a monoid retraction on non-negative addition.** For `a, b, c >= 0`,
`min(min(a + b, L) + c, L) = min(a + b + c, L)`, because once the running sum reaches `L` no further
non-negative term can bring it back below, and while it has not reached `L` the inner `min` is the
identity. So the eagerly-clamped and the once-clamped forms of a non-negative addition chain agree
**exactly**, with no approximation, and the design's "retained exactness" is available at zero
semantic cost.

**The condition is that the exact sum fits the container.** The deferral is legal for `k` terms when
`k * (2^W - 1) <= 2^C - 1`, that is `k <= floor((2^C - 1) / (2^W - 1))`. Beyond that the container
itself wraps and the deferred form is wrong rather than less accurate.

Three readings, and the third is the one that decides the fork.

**Under the minimum container at an exactly-filled width, `C = W`, so `k = 1`.** Every operation must
clamp, and the clamp sits on the loop-carried dependence of any reduction. `141:486-488` measured what
that costs: "a saturating fold is a loop-carried dependence through an operator LLVM will not
reassociate, so it is serial at every width and in every container."

**Under the shipped headroom container, `C = 2 * rung(W)`, so `k` is about `2^W`.** At `W = 8` that is
257 terms between clamps; at `W = 32` it is over four billion. So the shipped rule does buy the
retained exactness, exactly as the ratified sentence says, and it buys it at the price `141` measured.

**Under the accumulator rule the design already ratified, the room is derived per fold and is usually
free.** `124` section 1.8: a fold of arity `n` into accumulator numeral `M` is interior-safe when
`(n-1) * [min V(N), max V(N)]` is contained in `[min V(M), max V(M)]`, which for the unsigned case is
`W + ceil(log2 n) <= width(M)`. The accumulator is a numeral chosen for the fold, not a container
carried by every stored value for the life of the program. At `W = 8` and `n = 8192` it wants 21 bits,
so a `u32` accumulator over `u8` storage: **one quarter the memory traffic of the headroom rule, and
strictly more retained exactness than it, because 21 bits of room beats 16.**

That is the whole finding in one line. The headroom is a fixed 2x approximation to a derived quantity
that is usually smaller and occasionally much larger, and it is paid on storage, which is the one place
the quantity does not belong.

`141` rediscovered half of this empirically and did not recognise it. Its "theorem the compiler cannot
know" (`141:474-478`), `W + ceil(log2 N) <= 64`, is the design's own interior-safety condition with the
accumulator pinned at 64 bits, and it measured 7.5x to 32x. It is not a new theorem. It is
`124:1470-1482` and it traces to the Motorola 56000's eight guard bits for 256 MAC steps
(`124:1481-1482`, `24:164-165`, `25:262-271`). Section 7 measures what happens when the accumulator is
sized by the rule rather than pinned at 64.


## 5. The bench, the arms, and the two accidental controls that set the noise floor

Nine harness sections in `mock/benches/bench.toml`, six arms, 44 rows, every row with a committed CSV,
meta and findings triple. One shared transform crate at `variants/warm-clamp-shared`; every arm calls
one of two transforms and differs only in the carrier types it instantiates them at, so the comparison
is one program in several containers rather than six possibly-drifted programs.

The arms, each of which is an alternative someone might genuinely choose:

- `head`: the shipped rule. Storage `rung(rung_bits(W)+1)`, accumulator the storage type.
- `minimum`: `140`'s deletion and `141`'s verdict. Storage and accumulator the minimum native.
- `min-lanes`: minimum storage, eager clamping, fold reassociated into eight saturating accumulators.
  The arm that attacks whichever cell `minimum` loses.
- `acc64`: `141`'s theorem arm generalised to arity. Minimum storage, accumulator pinned at 64 bits.
- `accfit`: the design's own rule at `124:1474-1476`. Minimum storage, accumulator the narrowest rung
  satisfying `W + ceil(log2 n) <= width(M)`.
- `accfit-dyn`: `accfit` with the arity passed at run time rather than as a const generic. It exists to
  price one static fact on its own with everything else held.

**Every arm computes the same value at every key**, checked against a `u128` oracle sharing no code and
no carrier type with any of them, over all 46 declared keys rather than a sample
(`every_arm_agrees_with_the_oracle_on_every_key`). That test is what makes the `min-lanes` arm
admissible at all: reassociating a saturating fold is legal because unsigned saturating addition is
associative, and the test is what checks that rather than trusting it.

### The workload defect, found by a test before any number was taken

`141` section 3 records that its saturating fold was constant-folded because the accumulator pinned at
the limit and stopped depending on the input. My first cut had the same defect and
`chunked_answer_depends_on_every_element` caught it at the first key: flipping one input bit did not
move the answer. Elements drawn from the whole `W`-bit range make a fold of any arity above one
saturate immediately.

The fix is the distribution a clamping fold actually runs on. A MAC of `n` taps over a normalised
signal has terms of order `L / n`, so elements are drawn from `[0, ceil(2L/n)]`, which puts the
expected chunk sum at `L` and makes roughly half the chunks clamp.
`the_clamp_fires_on_a_real_fraction_of_chunks_at_every_chunked_key` asserts the fraction stays between
5% and 95% at every key, so a distribution drifting clamp-free or clamp-always is refused rather than
reported.

**arvo does not get to know this.** The interior-safety predicate is a function of the declared width,
so every arm still sizes its accumulator for `n * (2^W - 1)`. The data being friendlier than the
declaration is realistic and is not something the typestate may exploit.

### The arity sweep is ragged, and that is a finding

There is no distribution for a clamping fold of arity 256 into an 8-bit destination where the clamp is
neither dead nor absorbing, because the mean term `L / n` is under one quantum and every distribution
of non-negative integers with that mean is mostly zeros. Those cells are absent from the sweep and
named rather than quietly skipped.

### Two controls nobody designed, which is what sets the noise floor

A bench needs to know its own resolution and this one got it for free, twice, from arms that are
byte-identical by construction at some cells.

**At `W = 60`, `accfit` picks `rung(60 + ceil(log2 n))`, which is 64 at every arity up to 16, so at
those four cells `accfit` and `minimum` are the same instantiation of the same function.** Their
measured spread is 0.2%, 0.5%, 9.3% and 12.4%.

**At `W = 32`, `accfit` picks `rung(32 + ceil(log2 n))`, which is 64 at every swept arity, so `accfit`
and `acc64` are the same instantiation at all six cells.** Their measured spread runs 0.97x to 1.13x.

So **this bench's noise floor is about 13%**, and it grows with arity. Every ratio quoted below as a
result clears it; the ones that do not are named as unresolved rather than reported as small wins.

## 6. The container axis under clamping is not monotone, and it separates by 44x in both directions

`141:41` states "the container is monotone; minimum wins or ties in every measured cell". Under the
ratified clamp resolution that is false, and it is false in both directions inside one width sweep.

Medians in nanoseconds, 8192 elements, chunked clamping fold, arity `n`. Full distributions,
per-pass series and bootstrap intervals in the committed CSV and findings files.

| W | n | head | minimum | min-lanes | acc64 | accfit | head/minimum |
|---|---|---|---|---|---|---|---|
| 8 | 2 | 304 | **108** | 124 | 864 | 217 | 2.81 |
| 8 | 4 | 289 | **127** | 127 | 743 | 226 | 2.28 |
| 8 | 8 | 515 | 990 | 1025 | 668 | **468** | 0.52 |
| 8 | 16 | 263 | 1020 | 1022 | 652 | **237** | 0.26 |
| 13 | 2 | 564 | 255 | 309 | 828 | **258** | 2.21 |
| 13 | 4 | 564 | 318 | 392 | 712 | **273** | 1.77 |
| 13 | 8 | 535 | **524** | 1126 | 595 | 536 | 1.02 |
| 13 | 16 | 330 | 1085 | 1058 | 550 | **286** | 0.30 |
| 13 | 64 | 235 | 7134 | 598 | 537 | **208** | **0.03** |
| 13 | 256 | 223 | 9789 | 442 | 520 | **190** | **0.02** |
| 16 | 2 | 505 | 269 | **252** | 866 | 335 | 1.88 |
| 16 | 4 | 564 | **254** | 262 | 712 | 379 | 2.22 |
| 16 | 8 | 612 | 1011 | 952 | 608 | **526** | 0.61 |
| 16 | 16 | 332 | 1027 | 991 | 567 | **282** | 0.32 |
| 16 | 64 | 260 | 7358 | 1968 | 518 | **197** | **0.04** |
| 16 | 256 | 224 | 9751 | 238 | 515 | **189** | **0.02** |
| 32 | 2 | 1007 | **422** | 437 | 856 | 868 | 2.39 |
| 32 | 4 | 1150 | **506** | 505 | 836 | 837 | 2.27 |
| 32 | 8 | 629 | 1906 | 1852 | **550** | 565 | 0.33 |
| 32 | 16 | 531 | 2128 | 1743 | **416** | 416 | 0.25 |
| 32 | 64 | 490 | 3735 | 1789 | 398 | **390** | 0.13 |
| 32 | 256 | 481 | 5092 | **331** | 431 | 380 | 0.09 |
| 60 | 2 | 3048 | **1003** | 1845 | 1040 | 1005 | 3.04 |
| 60 | 4 | 2791 | 1178 | 2160 | 1152 | **1172** | 2.37 |
| 60 | 8 | 2381 | **621** | 1999 | 622 | 679 | 3.83 |
| 60 | 16 | 2244 | **499** | 3976 | 498 | 561 | 4.50 |
| 60 | 64 | 2501 | 9921 | 3622 | 9398 | **2480** | 0.25 |
| 60 | 256 | 2206 | 10369 | **1825** | 10330 | 2169 | 0.21 |
| 64 | 2 | 2064 | **824** | 826 | 838 | 1021 | 2.50 |
| 64 | 4 | 2045 | **1007** | 1028 | 1005 | 2124 | 2.03 |
| 64 | 8 | 2019 | 1837 | **1831** | 1834 | 2074 | 1.10 |
| 64 | 16 | 2132 | 2237 | **1779** | 2235 | 2106 | 0.95 |
| 64 | 64 | 2472 | 3782 | **1793** | 3687 | 2443 | 0.65 |
| 64 | 256 | 2159 | 5082 | 2035 | 5056 | **2135** | 0.42 |

Three things fall out and the third is the one op's hunch was about.

**The minimum container wins by up to 4.5x and loses by up to 44x, inside one sweep.** `head/minimum`
runs from 4.50 at `W = 60`, `n = 16` down to 0.02 at `W = 13` and `W = 16`, `n = 256`. The axis is not
monotone, and it flips on the arity rather than on the width.

**The cell `141` looked for and could not find exists, and there are eleven of them.** `141:744-749`
concedes: "I did not find a cell where the shipped headroom rule wins. I looked for one." Under the
ratified clamp resolution the shipped rule beats the deletion by 1.6x to 44x at every cell with arity
at or above 8 and width at or below 32, plus `W = 60` and `W = 64` at the top arities. Not because the
doubling is right, but because at a sub-rung width the eager clamp sits on a loop-carried dependence
through an operator LLVM will not reassociate, and the doubled container is the only place in the
shipped design that intermediate room comes from.

**And the design's own rule beats both.** `accfit` is the best or tied-best arm at **25 of the 38 fold cells**, is
never decisively worse than `head` at **any** of the 44 rows with data, and is decisively better than
`head` at **25** of them, while storing half the bytes. For comparison over the same 38 fold cells,
`minimum` is best-or-tied at 15, `min-lanes` at 13, and `head` at 8.

## 7. What the emitted code says, which is the mechanism rather than the correlation

`142_probes/` exports one symbol per case and imports the transforms from the bench's own shared
crate, so nothing there is a second copy of the kernel. `p142_asm_default.s` is committed.

`c_min_w16_a256`, the arm that runs at 9751 ns, is 26 lines with no vector register anywhere and this
inner loop:

```
LBB5_2:
        ldrh    w14, [x0, x13]
        add     x13, x13, #2
        add     w12, w14, w12, uxth
        cmp     w12, w9
        csel    w12, w12, w9, lo
        cmp     x13, #512
        b.ne    LBB5_2
```

One element per iteration, and the serial dependence runs `add` into `cmp` into `csel` and back into
the next `add`. The clamp is on the loop-carried chain.

`c_fit_w16_a256`, the arm that runs at 189 ns, is the same source with a wider accumulator. The
predicate `W + ceil(log2 n) <= width(M)` holds, `safe` folds to true, and the clamp leaves the loop:

```
LBB0_2:
        ldrh    w14, [x8, x13]
        add     x13, x13, #2
        add     w12, w12, w14
        cmp     x13, #512
        b.ne    LBB0_2
        cmp     w12, w9
        csel    w12, w12, w9, lo
```

and at the default profile that loop carries 16 vector operations where the eager one carries zero.
Vector-operation counts per symbol, from the committed assembly:

| symbol | lines | vector ops | measured |
|---|---|---|---|
| `c_min_w16_a256` | 26 | **0** | 9751 ns |
| `c_fit_w16_a256` | 43 | 16 | 189 ns |
| `c_lanes_w16_a256` | 93 | 31 | 238 ns |
| `c_head_w16_a256` | 115 | 64 | 224 ns |
| `c_min_w16_a4` | 47 | 28 | 254 ns |
| `c_min_w64_a16` | 55 | **0** | 2237 ns |
| `c_lanes_w64_a16` | 54 | **0** | 1779 ns |

So the predictor is the one `141:638-643` names and it survives being tested on a different
resolution: **the win is where the rewrite changes what kind of loop is compiled.** Deleting one
`csel` per element is worth 51x here for the same reason deleting one `and` per element was worth
21.6x there.

And the two zeroes at the bottom explain `W = 64`. There is no 64-bit vector saturating add on this
target, so the eager clamp at an exactly-filled 64-bit width is scalar whatever you do, which is why
lane-splitting rather than a wider accumulator is what wins there.

## 8. Attacking the arm that loses, which recovers 41x and changes what the fork is about

A variant that loses is a variant to improve. `minimum` loses by 44x at `W = 16`, `n = 256`, and the
emitted code says the mechanism is one serial dependence rather than anything about the container. So
supply the reassociation the compiler will not.

Unsigned saturating addition is associative and commutative, by the same retraction lemma: once a lane
reaches `L` no further non-negative term brings it below, so the lanes may be filled in any order and
combined by the same operation. `min-lanes` splits the chunk fold into eight independent saturating
accumulators. It is legal, and the all-arms oracle test at every key is what establishes that rather
than the argument.

| W | n | minimum | min-lanes | recovered |
|---|---|---|---|---|
| 13 | 64 | 7134 | 598 | **11.9x** |
| 13 | 256 | 9789 | 442 | **22.1x** |
| 16 | 64 | 7358 | 1968 | 3.7x |
| 16 | 256 | 9751 | 238 | **41.0x** |
| 32 | 64 | 3735 | 1789 | 2.1x |
| 32 | 256 | 5092 | 331 | **15.4x** |
| 60 | 64 | 9921 | 3622 | 2.7x |
| 60 | 256 | 10369 | 1825 | 5.7x |
| 64 | 64 | 3782 | 1793 | 2.1x |
| 64 | 256 | 5082 | 2035 | 2.5x |

**At `W = 16`, `n = 256` the recovery is complete**: 238 ns against `head`'s 224 ns, inside the noise
floor. The minimum container was never the thing that cost 44x. One serial dependence was.

And it costs where the compiler was already doing the job, which is `141:348-352`'s lesson arriving at
a different resolution with different numbers: `min-lanes` is 2.15x worse than `minimum` at `W = 13`,
`n = 8`, 3.2x worse at `W = 60`, `n = 8`, and 8.0x worse at `W = 60`, `n = 16`. A fixed kernel shape
beats the compiler only where the compiler declines.

**This is what dissolves the fork.** Two independent mechanisms recover the loss the deletion was
accused of, and neither of them is a container: sizing the accumulator by interior safety, and
supplying the reassociation. The doubled container is a third mechanism that recovers the same loss by
paying for it in every load, store and lane, forever, whether the fold that needs it ever runs or not.

## 9. Crossing L2, which reverses `141`'s footprint conclusion

`141:226-230` reports that crossing this host's 12 MiB L2 moved its ratios by less than noise and
concludes "the whole effect is instruction level". Under clamping, at arity 16 and 1048576 elements:

| W | head | minimum | acc64 | accfit | head/accfit |
|---|---|---|---|---|---|
| 8 | 31866 | 125756 | 78714 | **29358** | 1.09 |
| 13 | 52188 | 141991 | 69436 | **36356** | **1.44** |
| 16 | 56630 | 143464 | 72465 | **36304** | **1.56** |
| 32 | 120505 | 272465 | 59828 | **56469** | **2.13** |

(`W = 60` and `W = 64` were still running at write time and are absent rather than omitted.)

The gap between `head` and `accfit` **grows with width**, from 1.09x at 8 bits to 2.13x at 32, and 32
bits is exactly where `head` stores a `u64` column against `accfit`'s `u32`. That is a footprint
effect, it is absent from the L1 rows at the same widths and arity (1.11x and 1.28x), and it appears
only once the column stops fitting.

`141`'s conclusion was correct about the comparison it ran and does not generalise. Its L1 and L2
container ratios were nearly equal because at those cells the minimum container already won on
instruction selection by 20x to 44x, and a 2x traffic difference is invisible underneath a 44x
instruction-level one. Here the two arms are within 1.3x of each other on instructions, so the traffic
difference is the visible term. **Halving the storage is worth up to 2.13x on its own once the working
set leaves cache**, and that is a second, independent reason to prefer minimum storage that has
nothing to do with which loop is compiled.

## 10. Every static lever pulled, and what each measured

Op: "Static knowledge is always a potential lever, we should structure the bench and pull it. If it
doesn't work, now we know, we can add it to the droplist."

### Lever one: the fold arity as a compile-time constant. Worth up to 20.5x

`accfit-dyn` is `accfit` with the arity moved from a const generic to a runtime parameter and
everything else held. arvo carries a column's capacity as a `Cap` and a fold's arity is a type-level
quantity, so this is a fact the typestate has and the loaded slice length does not.

| W | n=2 | n=4 | n=8 | n=16 | n=64 | n=256 |
|---|---|---|---|---|---|---|
| 8 | **20.5x** | **19.0x** | 2.40x | 2.89x | | |
| 13 | **16.8x** | 7.03x | 2.16x | 2.64x | 1.34x | 1.03x |
| 16 | **12.9x** | **12.6x** | 2.61x | 2.88x | 1.55x | 1.05x |
| 32 | 6.51x | 3.67x | 4.89x | 2.09x | 1.19x | 1.02x |
| 60 | 4.28x | 4.04x | 2.24x | 1.75x | 1.14x | 1.20x |
| 64 | 4.22x | 1.50x | 1.36x | 1.40x | 1.17x | 1.33x |

Monotone in the arity and large where the arity is small, which is the shape a trip-count fact should
have: at arity 2 the const form is straight-line code and the runtime form is a loop with a test, and
by arity 256 the loop overhead has amortised. **It never costs anything at any cell.** This is the
cheapest lever in the file and the one with the fewest conditions on it.

### Lever two: the accumulator narrowed to the derived width rather than pinned at 64. Worth 1.1x to 4.0x

`141`'s theorem arm accumulates into 64 bits whenever the theorem holds. The design's rule says the
accumulator is a numeral sized by interior safety, which is usually much narrower, and a narrower
accumulator is more lanes.

`acc64` against `accfit`, at the widths where `rung(W + ceil(log2 n))` is genuinely below 64:

| W | n=2 | n=4 | n=8 | n=16 | n=64 | n=256 |
|---|---|---|---|---|---|---|
| 8 | **3.98x** | **3.29x** | 1.43x | **2.75x** | | |
| 13 | **3.21x** | **2.61x** | 1.11x | **1.92x** | **2.58x** | **2.74x** |
| 16 | **2.59x** | 1.88x | 1.16x | **2.01x** | **2.63x** | **2.72x** |

At `W = 32` the derived width is 64 at every swept arity, so the two arms are the same program; that
row is the control quoted in section 5 and reads 0.97x to 1.13x.

**And it inverts at `W = 64`**, where the derived width is 128 and `acc64` falls back to the machine's
eager `uqadd`: `acc64` is 2.1x faster at arity 4 and 1.2x faster at arity 2, while `accfit` is 1.5x
and 2.4x faster at arities 64 and 256. So the rule is not "narrowest always". It is "narrowest that
satisfies interior safety, and do not cross the widest native rung to get it", which is one more const
comparison.

### Lever three: reassociating a saturating fold. Worth 41x where the compiler declines, minus 8x where it does not

Section 8. `141:620-621` lists this law and reports it unpriced because its workload folded to a
constant. Priced here: 2.1x to 41.0x recovery at high arity, 1.8x to 8.0x loss at low arity and at
`W = 60`. Its selection predicate is not a width and not a container, it is the arity.

### Lever four: the interior-safety predicate itself, which is what deletes the clamp

The largest single effect in the file, and it is `141`'s section-12 theorem restated over the design's
own rule rather than pinned at 64 bits: 51x at `W = 16`, `n = 256`, 38x at `W = 13`, `n = 256`, 13x at
`W = 32`, `n = 256`. It declines by itself where `W + ceil(log2 n)` exceeds the widest rung, which is
the `W = 64` rows where every arm converges to within 1.3x.

### Lever five, which lost: nothing here reproduces `141` section 9's range-witness

I did not rebuild the range-witness shortcut, because `141` priced it at minus 20% to minus 37% with
the mechanism read off the instruction set (`uqadd` is one instruction and the licensed replacement is
two), and that reasoning transfers unchanged to the clamping resolution: every eager clamp at an
exactly-filled width in this bench compiles to `uqadd` and there is nothing to remove. It is on the
droplist and stays there.

### The three the typestate holds and I did not get to

Named so the list does not read as exhaustive, with what each would have to beat.

**Alignment.** arvo's columns are aligned by construction and LLVM sees a pointer of unknown
alignment, so every vectorised loop above carries a scalar prologue it could skip. Worth measuring at
small `n` where the prologue is a large fraction; probably worth nothing at 8192 elements.

**Disjointness.** A fold reads one column and writes an accumulator, and the typestate knows they do
not alias where `AccessSet` separates them. LLVM inserts no runtime alias check in these loops because
the accumulator is a local, so this one likely pays only where a fold writes a second column.

**Non-zero and exactness.** Untouched. The division rewrites `141:849-856` names, a divide by a
compile-time constant as a multiply by a reciprocal computed at the declared width rather than the
container width, is the natural next probe and needs a different workload from anything here.

## 11. The composition, resolved statically

Not a rule that wins on average. The best arm per cell, selected by const predicates over quantities
the typestate already carries: the declared width `W`, the minimum container `rung(W)`, the fold arity
`n`, and the derived accumulator `A_fit = rung(W + ceil(log2 n))`.

**Storage is the minimum native container at every cell, without exception.** That result survives from
`141` and gains an independent second reason in section 9: halving the column is worth up to 2.13x on
its own once the working set leaves cache. The doubled container is never the answer, at any width, any
arity, or any resolution measured.

What varies is the accumulator and the fold's shape:

| condition | accumulator and shape | measured against the next best |
|---|---|---|
| no fold (elementwise chain) | the container; clamp each step | minimum storage worth 2.07x to 2.35x at `W <= 32`, free at 60 and 64 |
| `n <= 4` | the container; clamp each step, no lane split | 1.6x to 2.4x over a widened accumulator at `W = 8`, `16`, `32`, `64` |
| `n >= 8` and `A_fit <= 64` | `A_fit`; interior-safe, one clamp at the root | 2.1x to 51x over the eager form; 1.1x to 4.0x over pinning at 64 |
| `n >= 8`, `A_fit > 64`, `W == rung(W)` | the container, eager, lane-split at `n >= 16` | 1.2x to 2.8x over both the wide accumulator and the serial form |
| `n >= 8`, `A_fit > 64`, `W < rung(W)` | `A_fit`, multi-limb, interior-safe | 2.7x to 4.0x over the eager form; the sub-rung clamp costs two instructions |

Every predicate is a `const fn` of `W` and `n`. No entry is a heuristic and no boundary is a tuning
parameter: `A_fit <= 64` is where a native rung stops existing, `W == rung(W)` is where the machine's
saturating add is the semantics, and `n <= 4` is where the chunk's serial chain is short enough that
parallelism across chunks covers it.

**One cell is unresolved and I mark it rather than smoothing it.** At `W = 60`, `n = 256` the table
selects the multi-limb interior-safe form at 2169 ns and `min-lanes` measures 1825 ns, a 1.19x
advantage that is only just clear of the 13% floor. The boundary between rows four and five may sit at
a width rather than at `W == rung(W)`, and one more sweep between 33 and 63 bits would say.

**And what the table does not have is a container column with two values in it.** `141:672-673` says
"the container column has one value in every row. That is the answer to whether the fork is a matrix:
it is not." That conclusion survives being tested against the resolution it was not tested against,
and it survives for a better reason: the matrix is real, it has five rows, and it is on the accumulator
rather than on the container, because the accumulator is a per-fold choice and the container is a
per-value one.

## 12. What this says about the fork, stated as the answer to op's hunch

Something does ride on the headroom. It is not the source's "single-op overflow headroom for Warm
wrapping" (`arvo-strategy/src/container.rs:15-16`), which `141:779-782` already showed names a
mechanism that cannot do that job. It is the ratified `StoredWidth = doubled` row and its stated
ground, that a chain of operations retains more than one operation's exactness before a narrow forces
a decision (`124:2600-2615`, ratified at `70b`), and the thing that consumes that room is a fold.

So the fork was never a choice between two containers. It was **one question wearing a container's
clothes**: where does a fold's intermediate room come from. The shipped rule answers it on the storage
axis, per value, permanently, with a fixed factor of two. The design's own section 1.8 answers it on
the accumulator axis, per fold, derived from the arity, with the exact number of bits. The second
answer is better at every cell measured and it is already ratified.

**The reason the question keeps returning is that the source has no accumulator.** `35:97-101` removed
the `Widening` axis and re-scoped `InContainer` as "a `StoredWidth` fact about that container, full
stop", correctly, because by then the multiplicative half had made every exact intermediate a named
numeral (`35:83-90`). The container projection at `container.rs` never learned that. So the only place
intermediate room comes from in shipped code is the doubling, and every attempt to delete it has run
into the fact that something needs the room, without anyone naming what.

**Deleting the headroom is safe, and it is only safe together with the accumulator rule.** Delete it
alone and `Warm`'s clamping folds lose their retained exactness with nothing to replace it: that is the
`minimum` column at 44x. Delete it and ship the interior-safety derivation and the same folds get more
room than the doubling ever gave them, in a narrower type, over half the storage.


## 13. A correction to my own method, and two defects in the harness it surfaced

Op checked, mid-dispatch, whether the harness already provides tooling for working with emitted
assembly and counting things in it. It does, I had not read it before building section 7's artifact,
and the instinct was right in a way that cost real effort.

**What exists.** `bench-harness/src/disasm.rs`, 118 lines. `extract_bench_entry` runs
`objdump -d --disassemble-symbols=bench_entry` against each **variant dylib**, falls back to
`otool -tv` on macOS, and normalises addresses away (`disasm.rs:13-59`). `check_duplicates`
(`disasm.rs:84`) compares the normalised text across variants and warns on an exact match, on the
reasoning that two variants with identical machine code will benchmark identically and waste the run.

**What it does not have**, so the counting in `142_probes/` duplicates nothing: no opcode counting, no
vector-against-scalar classification, no extraction of any symbol other than `bench_entry`, and no
comparison other than equality.

**But the extraction is duplicated, and the harness's is the better one.** It reads the artifact that
was actually timed. My probe crate is a separate compilation with its own exports, and that gap is
precisely where the trap recorded at `142_probes/README.md` lived: two programs, one timed and one read, disagreeing by
13x, with a plausible and wrong explanation available. Reading the dylib from the start would have
left no room for it. That is the lesson and it belongs to the method rather than to the result.

Two defects in `disasm.rs` fell out of using it, both worth sending upstream.

**The objdump path is dead on macOS.** `extract_bench_entry` passes `bench_entry`; the Mach-O symbol
is `_bench_entry`. objdump answers `failed to disassemble missing symbol bench_entry` and the code
falls through to the `otool` branch with no signal. Every macOS run has been on the fallback.

**And `check_duplicates` compares a shim rather than the arms.** Under `#[bench_variant]` the
`bench_entry` symbol is a generated dispatcher and the transforms are separate static functions.
Disassembled from all six arms of this bench it is **592 instructions in every one, with zero vector
operations and zero saturating adds in all six**, while the whole-dylib counts separate cleanly
(`min-lanes` 2246 vector operations and 460 `uqadd`, `accfit` 1658 and 0, `head` 1534 and 0). So the
duplicate check cannot see what an arm computes, and its silence during my runs is not evidence that
any two arms differ. That matters beyond this file: it is the mechanism the harness offers for
catching an aliasing arm, and on this bench shape it cannot fire on one.

**What I took from the harness anyway.** The whole-dylib counts above are a direct reading of the
timed artifact and they corroborate section 7's per-cell reading from the probe: the arm that
lane-splits carries the most vector operations and by far the most saturating adds, and the three arms
whose eager path runs in a container wider than `W` carry no saturating add at all, because there the
`sat_add` provably cannot overflow and LLVM lowers it to a plain add. Aggregate over 46 keys, so it
cannot decide a cell; it is a second instance agreeing with the first by a different route.

**And the noise-floor controls are no longer an inference.** Section 5 reads this bench's resolution
off two pairs of arms that select the identical accumulator carrier at some cells. That identity was
asserted from reading the selector, which is exactly the kind of claim that should not rest on
reading. `the_noise_floor_controls_really_are_the_same_instantiation` now checks it as a const fact at
every cell quoted as a control, and checks the complement, that no cell quoted as a **result** is
secretly a control. `check_duplicates` could not have established this, for the reason above.

## 14. The reading trap that cost a wrong conclusion, stated in the open

Recorded here rather than only in the probe README, because it invalidates a class of evidence this
panel has used repeatedly.

The first cut of `142_probes/` carried `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, on the
reasoning that a more aggressive profile is a better one to read. Every symbol came out with **zero
vector operations**, disagreeing with the harness by 13x, and it would have supported a confident
claim that the interior-safe arm does not vectorise either and that something else explains the 51x.

Isolating the three settings one at a time: `codegen-units` and `panic` were innocent, and **both
`lto = "fat"` and `lto = "thin"` took every symbol to zero**. That reads as a compiler finding and is
not one. `--emit=asm` on a **library** crate under LTO emits code from before the link step, and under
LTO the work that matters happens at the link step, so the file describes a program that never runs.
Building a binary and disassembling the linked result settles it: 991 vector operations at the default
profile and 568 under `lto = "fat"`, both nonzero.

So: **emitted assembly from a library crate compiled with LTO is a reading of nothing.** Read a linked
artifact, or a library built without it. This panel has cited `--emit=asm` output in many files and I
have not audited which of them carried an LTO profile; anyone rechecking a codegen claim should look
at that first, because the failure is silent and the output looks entirely normal.

## 15. Alternatives I did not take, so the next attack starts from a list

**Signed numerals.** Everything here is unsigned, and the retraction lemma that the whole bench rests
on is false for signed clamping: with terms of both signs a running sum can leave the range and come
back, so eagerly-clamped and once-clamped forms genuinely differ. That is not a gap in the measurement,
it is a different question, and it is the one where the ratified "retain more than one operation's
exactness" sentence has real semantic content rather than only a performance one. **This is the single
most valuable thing left undone in this file.**

**The accuracy half, on any workload.** Every arm here computes the same value, which is what makes the
timing comparable and is also what keeps the exactness claim untested. A workload where the deferred
form is *more accurate* rather than merely faster would test what the doubling was ratified for. It
needs a non-monotone chain, so it needs the signed case above, or a chain with a narrowing in it.

**Multiplication and the MAC.** `124:1491-1496` extends interior safety to a multiply-accumulate with
the product numeral in place of the destination, and the accumulator sizing there is
`product_width + ceil(log2 n)` (`25:234-236`). Nothing here multiplies. `141:517-567` reports that any
multiply at 33 to 64 bits drops the loop off the vector path on this target, so a MAC sweep would cross
that wall and the accumulator lever may behave differently on the other side of it.

**Alignment and disjointness as levers.** Named in section 10 with what each would have to beat.
Untried.

**Reciprocal and division rewrites.** `141:849-856` names them as the natural next probe and they need
a workload unrelated to anything here.

**Whether the accumulator should be a typestate at all.** I measured that sizing it by the arity is
worth 1.1x to 4.0x over pinning it at 64 and 2.1x to 51x over not having one. I did not design the
mechanism, and `135b`'s erasure gate is the constraint it has to meet: a derived accumulator numeral
must be derived, validated and erased with no caveat. `141:754-758` concedes the same thing about the
deferred projection and says someone who reasons about the gate should rule. That is still true and it
is now true of a second derived quantity.

**x86-64.** One target, one microarchitecture. Three of the boundaries in section 11's table are facts
about this instruction set: that `uqadd` exists in vector form at 8, 16 and 32 bits, that it does not
at 64, and that there is no 64-bit vector multiply. AVX-512 changes at least the last of those. The
table needs re-running; the shape of the argument should not.

## 16. Dead routes, with what closed each

1. **The doubled container as a general rule.** Loses 1.6x to 44x at every fold cell with arity at or
   above 8, loses 2.07x to 2.35x on every elementwise chain at 32 bits and below, and loses up to 2.13x
   again on footprint once the column leaves L2. It wins only against a *deletion that ships nothing in
   its place*, which is not a proposal anyone has to accept.
2. **`minimum` as a general rule, which is `141`'s verdict.** Loses by up to 44x under the ratified
   clamp resolution. Closed by section 6, and reopened as a component rather than a rule.
3. **Lane-splitting as a general rule.** Recovers up to 41x at high arity and costs up to 8.0x at low
   arity and at `W = 60`. Per cell or not at all.
4. **Pinning the accumulator at 64 bits**, which is what `141`'s theorem arm does. Costs 1.1x to 4.0x
   against the derived width wherever the derived width is narrower.
5. **Narrowest accumulator without a ceiling.** Inverts at `W = 64`, where crossing to 128 bits costs
   2.1x against falling back to the machine's eager saturating add. The rule needs the ceiling.
6. **Runtime fold arity.** Costs up to 20.5x and never pays. Not a route anyone proposed; measured so
   that the const-generic arity is priced rather than assumed.
7. **The range-witness shortcut.** Not rebuilt. `141` priced it at minus 20% to minus 37% and the
   mechanism (the removed check is one `uqadd`) transfers unchanged to this resolution.
8. **A clamping fold over a whole column as a workload.** Saturates in the first few terms at every
   width and arity, so the answer stops depending on the input. This is `141`'s void run reproduced,
   caught here by a test before any number was taken rather than by reading a suspicious figure.
9. **Clamping folds of arity 256 into an 8-bit destination.** No distribution exists where the clamp is
   neither dead nor absorbing. Those cells are absent by construction and named.
10. **`--emit=asm` on a library under LTO.** Reads a program that never runs. Section 14.
11. **`check_duplicates` as an aliasing control.** Compares a 592-instruction dispatcher identical
    across all six arms. Section 13.

## 17. Where I concede

**I did not test the accuracy claim.** The ratified ground for the doubling is retained exactness, and
every number in this file is a timing over arms that agree. I established that the room the doubling
provides is better provided elsewhere, and I did **not** establish that the exactness it was ratified
for is preserved by the replacement in the case where the two forms differ. For non-negative addition
they provably do not differ, which covers the fold; for a signed or non-monotone chain they do, and I
have no measurement and no test there. Section 15 says why that is the first thing to do next, and
someone who reasons about the quantiser rather than about codegen should do it.

**I could not settle the `W = 60`, `n = 256` cell.** Section 11 marks it. The boundary between the last
two rows may be a width rather than the fill condition, and locating it needs a sweep between 33 and 63
bits that I did not run.

**My noise floor is 13% and it is wider than I would like.** It comes from two accidental controls
rather than a designed one. `141` had the same problem and solved it better, by carrying an arm known
to alias another as a deliberate control (`141:122-127`). A designed control arm belongs in this bench
and is not there.

**And I am not the one to say whether any of this becomes a typestate.** Section 15's last two entries.

## 18. What is op's

**Whether `Warm` clamps or wraps.** This is the one that has to be settled before anything else in the
container argument means anything, and it is not mine. The `70b`-ratified fixed-point table says clamp
(`124:2600-2608`) with a stated ground. `140b:17-21` says "behave like native primitives in regular old
rust", which is a later and more general statement and which selects wrap in release. Four panel files
and six arm cdylibs have been built on the wrap reading; this file's nine sections are built on the
clamp reading. **One of the two bodies of work is measuring a resolution the design does not assign to
`Warm`, and which one depends on a call only op can make.** I have not assumed the answer: section 6's
table is about clamping and `141`'s is about wrapping, and both stand as measurements of their own
resolution.

**Whether the container question is closed by moving the room to the accumulator.** My reading is that
it is, that storage takes the minimum native container at every cell without exception, and that the
matrix op expected is real and sits on the accumulator rather than on the container. Section 11 is the
table and section 12 is the argument. What I would most want attacked is the claim in section 12 that
the fork was one question wearing a container's clothes, because if that is right then two prior
dispatches and this one have all been pricing the wrong axis, and the thing to build is the
interior-safety derivation rather than a container rule.

**Whether the accumulator becomes a derived typestate quantity, and how it meets the erasure gate.**
Worth 2.1x to 51x. `135b`'s four parts have to hold for it with no caveat, and I did not design the
mechanism.

**And a correction rather than a call.** `arvo-strategy/src/container.rs:15-16` documents the doubling
as "single-op overflow headroom for Warm wrapping and Precise saturating semantics". The ratified
ground is neither of those; it is retained exactness across a chain before a narrow
(`124:2613-2615`). The source comment has been read as the design by every file in the container
argument including mine before I went looking, and it is the proximate cause of three dispatches
pricing a mechanism against a purpose it does not serve. Whatever is decided about the container, that
sentence should not survive it.
