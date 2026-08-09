# 140. Warm without widening: what wrapping at a declared width requires of the machine

**Author:** Agner Fog (persona dispatch)
**Date:** 2026-08-07
**Position:** after `139_ovadia_the_derivations_that_stop_short.md` and op's checkpoint `139b`, taking the
one question `139b` left open: the headroom rule is condemned, both offered replacements are refused, and
the answer has to be better than either.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2024, `-O`, aarch64-apple-darwin.
**Evidence:** ad-hoc quick spikes with no substance, checked into `140_probes/` beside this file. Sources,
emitted assembly, generator and counting scripts, all of it. Read the standing on evidence below before
reading anything else, because it governs what every claim here is allowed to be.

## The standing on evidence, first, because it invalidates half of what I set out to write

`.claude/rules/evidence-lives-in-the-repo-or-it-never-happened.md` landed today, while this dispatch was in
flight. It binds this file harder than it binds most, and I would rather apply it to myself than have the
next reader apply it for me.

Part two of that rule:

> Any benches that are done outside the benches/ harness of mockspace, if mockspace is used in the repo,
> can not be named "bench" or "benchmark" or anything similar, rather, they may only be referred to as
> "ad-hoc quick spike with no substance".

arvo uses mockspace and has a live harness at `mock/benches/`, with `bench.toml`, cdylib variants under
`variants/`, and a committed CSV plus meta plus findings trail. **Everything in this file was produced by
`rustc -O --emit asm` on short standalone files and counted with a script.** So none of it is a bench, none
of it is a measurement, and none of it prices anything. It is an ad-hoc quick spike with no substance, it is
called that throughout, and per the rule it may support only the qualitative claims it is capable of
supporting: that something compiles, that something is refused, that a symbol was folded, that a loop did or
did not vectorise, that an instruction was or was not selected.

I set out to answer "what does the rule cost at the instruction level" with ratios. I have ratios. **Under
this rule they decide nothing and I am not presenting them as a decision.** What I am presenting is a set of
structural facts about what the machine does, which do not need a magnitude to be load-bearing, plus a named
list of what the harness is owed.

Part one applies too. The probe directory is checked into `140_probes/` beside this file and must be
committed in the same act as the file. **Until that commit lands, everything here is void by the rule**, and
I would rather state that as the condition than have it discovered later.

## Verdict

**No strategy takes headroom in storage.** The container for a numeral of width `W` is the minimum aligned
native that holds `W`, identically for `Hot`, `Warm`, `Cold` and `Precise`. Nothing widens at or below 64
bits, and the crossing into the structural wide payload is at 129 bits for all four. The width discipline
moves onto the operation's result: a mask for unsigned wrapping, a sign-extend for signed wrapping, a clamp
for saturating.

This is the second of the two refused options, deletion, and I am aware op refused it. The case for it does
not rest on a number, which is what it lacked before and what the evidence rule says it could not have had
anyway. It rests on three structural facts, each of which an ad-hoc spike is competent to establish:

**The headroom does not do the job it is named for.** `131:277` scopes it to "single-operation overflow
room". Below the rung, wrapping at `W` in a container of `C > W` requires projecting the result back to `W`
regardless, because the container wraps at `C` and the semantics wrap at `W`. That projection is present in
every program with or without the headroom, so the headroom's only distinct effect is to let an unprojected
chain accumulate to the container width instead of wrapping at `W`, which is not Warm's semantics under any
reading. `139:225-228` reached this first. I confirm it and add that it is not a matter of degree: the
mechanism is redundant where it appears to work and wrong where it appears to do something extra.

**The projection is not per-operation, and this needs no design mechanism.** `x mod 2^W` is a ring
homomorphism out of `Z/2^C` for `+`, `-`, `*` and `<<`, so a mask feeding another such operation is dead
code. LLVM performs the elimination. Written with a mask after every one of three operations and written
with one mask at the end, the two forms **fold to the same symbol**, which the assembler records as an
alias. So arvo writes the mask everywhere, which is simple and always correct, and the compiler removes the
ones that cannot be observed.

**The headroom container forfeits hardware, categorically rather than by a margin.** At 33 to 64 bits the
headroom container is `u128`; aarch64 NEON has no 128-bit lane and the loop does not vectorise at any unroll
factor, while the minimum container's does. For `Precise` it is worse in kind: at an exactly-filled width
the machine has `uqadd.2d`, a single instruction that *is* saturating addition, and no construction over a
wider container can be that instruction.

**`Precise` does not differ from `Warm` here**, which is the check `139:242-246` argued and marked as owed. It
is run, and its conclusion is the stronger of the two.

## 0. A correction to the number op ruled on, and a harder point about it

`139b:16-17` records the finding op acted on: "Warm's `u128` form at 64 bits is a rolled scalar loop against
thirty-two `add.2d`, roughly 1600 instructions against 81 over sixty-four elements." Op ruled at
`139b:19-21`: "1600 instructions for a simple loop ... is unacceptable."

**The 1600 is wrong on its own terms, by a factor of four**, and this is an arithmetic claim rather than a
priced one, so an ad-hoc spike can carry it. The rolled loop is unrolled four elements per iteration, so
its trip count over sixty-four elements is sixteen, not sixty-four. From my own reproduction of the same
shape (`140_probes/p2.s`):

```
_v_w64_headroom:
        add x8, x1, #32 ; add x9, x0, #32 ; mov w10, #64
LBB8_1:
        ... 21 instructions, four u128 elements per pass ...
        subs x10, x10, #4          <- the step is four
        b.ne LBB8_1
```

Twenty-one times sixteen plus a three-instruction prologue is 339, not 1600. `139` multiplied the loop body
by the element count rather than by the trip count, and every rolled-loop figure in that file carries the
same factor. `140_probes/rolled.py` recomputes them from the `subs` step and the emitted body.

**The harder point is that the number should not have decided anything at either value.** The evidence rule's
own explanation of why the naming matters uses this exact figure as its worked example:

> "1600 instructions against 81" reads as settled fact regardless of whether it came from a calibrated
> harness with cdylib-isolated variants, a shared realistic workload, warmups, repeated runs and a committed
> artifact trail, or from one `rustc -O` invocation on a thirty-line file.

It came from the second. So did mine. The container fork is a fork about how much, it has been treated as
decided by instruction counts twice now, and the harness that exists for exactly this has not been run on it.

What I would say to op, plainly: **his ruling stands and does not need revisiting, because it was not really
a ruling about 1600.** It was a ruling that a loop over values fitting a native register must not lose its
vector form, and that fact is structural and survives any correction to the count. But the record should
carry 339 rather than 1600, and the fork should carry a harness result rather than either.

## 1. What this brief takes for granted, and what survives checking

**"The rung ladder."** Not a design choice and not attackable. Rust exposes integer types at 8, 16, 32, 64
and 128 bits and nothing between; LLVM has `iN` for arbitrary `N` and no stable or unstable Rust surface
reaches it. A thirteen-bit numeral has to live in something and the something is a `u16`. The ladder is the
target language rather than arvo's invention, and every route on the table keeps it. I record that I looked,
because the brief asked me to start there.

**"The current rule widens 64 of the 64 widths at or below 64 bits."** Confirmed against the shipped ladder
at `mock/crates/arvo-strategy/src/container.rs:15-19`, which states it in its own module documentation:
"**Warm / Precise**: 2x logical width (one bucket up; carries single-op overflow headroom for Warm wrapping
and Precise saturating semantics). `1..=8 -> u16`, `9..=16 -> u32`, `17..=32 -> u64`, `33..=64 -> u128`."
Every native width maps one rung up. The same file at `:19-21` carries the consequence op reopened: "**No
native bucket above N=64** by design: Warm / Precise at `N=65..=128` falls into the wide bucket directly (no
native u256 ladder)."

**"Below the rung a mask is required regardless."** True, and it is the load-bearing finding. `139` states it
as a finding rather than a proposal, correctly.

**"Warm's semantics are wrapping at W."** Taken from `131:275-280` and not tested, because it is a semantics
question rather than a codegen one. Everything below is conditional on it. If Warm's wrap is at the container
width rather than at `W`, this file prices the wrong obligation, and someone should say so before the rule is
written.

The premise I did find worth attacking is not in the brief: that the numeral has to sit at the **bottom** of
its container. Section 4 reports that route and it loses.

## 2. What wrapping at W requires, structurally

Two cases, genuinely different obligations rather than two shades of one. Both established by reading emitted
assembly, which is what an ad-hoc spike can honestly do.

**W exactly fills the container.** The container's own wrap is the wrap at `W`. `wrapping_add` on a `u64` at
`W = 64` emits `add x0, x1, x0 ; ret` (`140_probes/p1.s`, `exact64`). There is nothing to add and nothing to
remove. Headroom here does not merely fail to help: it takes a case the hardware implements exactly and
replaces it with one the hardware does not implement.

**W is below the container.** The result needs projecting back to `W`. Unsigned that is one `and`; signed it
is one sign-extend, `sbfx` on this target. What the emitted code shows, and it is the interesting part:

```
sub13      (u16, wrap at 13)   add w8, w1, w0 ; and  w0, w8, #0x1fff   ; ret
exact16    (u16, wrap at 16)   add w8, w1, w0 ; and  w0, w8, #0xffff   ; ret
w_i13_min  (i16, wrap at 13)   add w8, w1, w0 ; sbfx w0, w8, #0, #13   ; ret
w_i16_exact(i16, wrap at 16)   add w8, w1, w0 ; sxth w0, w8            ; ret
```

**At an `extern "C"` boundary the logical projection occupies the same instruction slot the container's own
width was going to occupy.** `and #0x1fff` stands where `and #0xffff` stood; `sbfx ..., #13` where `sxth`
stood. That is a real fact about the boundary and a misleading one about the interior, where the
container-width extension is not emitted every step and the logical projection is. I record it because it
explains something about how the headroom rule survived: at scalar call boundaries there is nothing visible
to fix, and 131's rule and the minimum container emit the same three instructions.

## 3. The three structural findings

Each is a qualitative claim about what the compiler emitted, which is what this evidence can carry. Sources
and assembly in `140_probes/p1..p5`.

### The projection sinks, and the compiler does it unaided

Written eagerly, a mask after every operation, at four depths:

```
chain13_eager   3 adds, mask after each      add ; add ; add ; and ; ret          one mask
_chain13_lazy = _chain13_eager               folded to a symbol alias

mix13_eager     add, mul, sub, mask each     add ; neg ; madd ; and ; ret         one mask
_mix13_lazy = _mix13_eager                   folded

v_w13_three_eager   3 ops per element, 64 elements, mask after each
                    8 add.8h + 8 mla.8h + 8 neg.8h + 8 bic.8h       one mask per vector, not three
_v_w13_three_lazy = _v_w13_three_eager       folded

v_w13_reduce_eager  64 accumulations, mask after each
                    7 add.8h + addv.8h + fmov + and                 one scalar mask, not sixty-four
_v_w13_reduce_lazy = _v_w13_reduce_eager     folded
```

The symbol aliases are the strongest form this claim can take: the eagerly-masked and lazily-masked forms are
not similar, they are the same code, and the assembler says so. The mechanism is the ring homomorphism, and
it is exact about where it stops: `cmp13` keeps its mask ahead of a comparison, and `low13` has its mask
absorbed entirely into the consumer's own `& 0x7`. Masks in front of `>>`, `/`, `%`, comparisons and
narrowing stores survive, correctly.

**Design consequence, and it is the one that removes work rather than adding it.** arvo writes the projection
after every operation. No deferred-normalisation machinery, no annotation, nothing in the typestate, no
choice for a consumer to get wrong. The optimiser already implements the amortisation that a design mechanism
would have had to encode.

### The projection does not obstruct vectorisation; the headroom container prevents it

Every masked loop I compiled vectorised at the container's full lane count, at every rung from `u8` to `u64`,
unsigned and signed. This was the thing most worth checking and the thing I expected to go wrong.

Every headroom loop at 33 to 64 bits did not vectorise at all. The container is `u128`, there is no 128-bit
NEON lane, and the emitted body is a scalar `adds` / `adc` chain with a back edge. That is categorical: it is
not a worse vector form, it is the absence of one.

Two details worth recording because the inflated figure hid them. At `W = 64` the `u128` body issues one
`add` per element rather than `adds`/`adc`, because the mask to 64 bits proves the high half zero and LLVM
drops its arithmetic, storing `xzr`; so the cost there is doubled memory traffic and the lost vector form
rather than a carry chain. The genuine carry chain appears *below* 64 bits, at `W = 60`, where the mask does
not prove the high half zero.

### Precise wants the minimum container more strongly than Warm does

Saturation at `W` in a container of `C` is a different obligation from wrapping: wrapping is a total
projection, saturation must detect that the true sum left `[0, 2^W)` and substitute a bound. The detection
still needs no container headroom.

Below the rung, the container's spare bits make `a + b` exact, since two values under `2^W` sum under
`2^(W+1)` and `W + 1 <= C`. The clamp is a compare and select; vectorised, `umin.8h`, at full lane count.

At the rung, the machine has the whole operation as one instruction. `vs_u64_exact` emits `uqadd.2d`,
detection included, one instruction per vector. The headroom form emits `adds` / `adc` / `cmp` / `csinv` in a
rolled scalar loop. **This is the sharpest structural point in the file:** no construction over a wider
container can be an instruction that is exactly the semantics, so for `Precise` the headroom rule does not
trade cost for safety, it discards a hardware primitive and rebuilds it worse.

The signed side carries the only real asymmetry I found. Signed sub-rung wrapping is one `sbfx` scalar, but
**two** vector instructions, `shl.8h` then `sshr.8h`, because NEON has no vector bitfield extract. Signed
sub-rung saturation clamps on both sides. Both still sink across chains for the same homomorphism reason, but
the signed sub-rung case is the most expensive corner of the proposal and should be recorded as such rather
than averaged away.

## 4. The premise worth attacking, and it loses

The projection exists because the numeral sits at the bottom of its container. Put it at the top, scaled by
`2^(C - W)`, and the container's own wrap **is** the wrap at `W`: the carry leaving bit `C - 1` is the carry
that left bit `W - 1`, and the low `C - W` bits are zero in every operand and stay zero through addition and
subtraction. No mask at all. This is old and not mine; it is why fixed-point DSP code is often written
left-justified.

It works as advertised (`140_probes/p5.rs`). The high-aligned add emits `8 add.8h` and no mask, matching the
exactly-filled control; the high-aligned multiply swaps the mask for a `ushr`; comparison folds to the same
symbol as the low-aligned form; and every read out of the type pays a shift.

**It loses on three grounds and the third is decisive.** What it saves is what the previous section shows the
compiler already gives away, since the mask sinks. What it costs moves to every boundary crossing, on the
same schedule the mask was on, so at low operation density it is a straight swap. And `raw()` on a
left-justified `UFixed<13, 0, Warm>` yields `8192 * v` rather than `v`. That is not a performance question:
it breaks the `Transparent` unwrap door that `arvo-transparent` exists to provide for every
`repr(transparent)` primitive, it breaks any `repr(C)` crossing, and it makes `135b`'s part 4 erase to a
container holding something other than the numeral. A representation change that buys nothing net and costs
the unwrap door is not a trade.

Dead route, reported as one so the next reader does not spend the afternoon on it.

## 5. The boundary cases

`140_probes/p4.rs` sweeps `W` at 1, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128 under the proposed
rule. The structure has exactly one discontinuity and it is not where the design's vocabulary suggests.

**One below a rung, exactly a rung, one above.** The local optimum is always the exactly-filled width, where
the projection vanishes because the container's wrap is the semantics. One bit past it moves to the next
rung, which is the ladder's granularity and is unavoidable in any design storing a numeral in a machine
integer. Nothing here is a defect and nothing is strategy-specific.

**The one discontinuity is at 65 bits, and it is the vector unit rather than a rung.** Below 65 the container
is a NEON lane type and the loop vectorises; at 65 it is `u128`, there is no lane, and it does not. That
cliff is a property of the target, and it is where `137`'s structural wide payload takes over. Under the
proposed rule all four strategies meet it at the same place, which is the point: op ruled at `137b:72` that a
declared width above 64 going multi-limb is a consequence rather than a defect, and this puts Warm and
Precise on the same footing as Hot and Cold instead of making them meet it at 33.

**The wide side is untouched.** `137b:48-53` adopted ragged for `Cold` and `Precise` and word-rounded for
`Hot` and `Warm`; `131:280` keeps `Hot` at align-16 above the native rungs. Neither is about headroom, so
neither is affected. I compiled 127 and 128 and did not touch 129, because `137` did and its work governs
there.

## 6. The rule

> **No strategy takes headroom in storage.** The container for a numeral of width `W` is the minimum aligned
> native that holds `W`, identically for `Hot`, `Warm`, `Cold` and `Precise`, and the crossing into the
> structural wide payload is at 129 bits for all four.
>
> The width discipline lives on the operation's result rather than in the container. Where `W` equals the
> container width, the machine's own wrap or its saturating instruction is the semantics and nothing is
> emitted. Where `W` is below it, one instruction projects the result back: a mask for unsigned wrapping, a
> sign-extend for signed wrapping, a clamp for saturating. The container's spare bits make that projection
> exact, and they exist at every width that is not a rung.
>
> The projection is written after every operation. It is a ring homomorphism for `+`, `-`, `*` and `<<`, so
> the redundant ones are dead and the optimiser removes them.

Three things this deletes and one it does not.

It deletes the `Warm` / `Precise` bucket table at `container.rs:15-21`, so `tag_warm_precise` and
`tag_hot_cold` collapse into one function of `W` and the native ladder stops being strategy-keyed. What stays
strategy-keyed above the native rungs is alignment and the ragged-versus-word-rounded payload. This is a claim
about the native ladder only: `Cold`'s bitpacked storage is a separate representation and I touched none of
it.

It deletes `131:277`'s "single-operation overflow room", which names a mechanism that does not do that job.
Whatever replaces the container rule, that sentence should not survive it.

It does not delete the widened intermediate at a fixed-point multiply. A product of `UFixed<I1, F1>` and
`UFixed<I2, F2>` has `I1 + I2` integer and `F1 + F2` fractional bits and needs `2W` bits computed exactly
before the shift. That is headroom **at the operation**, local to one instruction sequence, and unrelated to
what the operands are stored in. The distinction is the finding compressed: headroom in storage is paid on
every load, every store and every lane, for every value, for the life of the program; headroom at an
operation is paid once, where the operation needs it. 131's rule put it in the wrong place.

## 7. What the harness is owed, named concretely

The magnitude question is unpriced. Per the evidence rule, the honest sentence is that it is unpriced rather
than that it was measured informally, and the harness at `mock/benches/` is where it gets priced. It is
already set up for exactly this shape, with cdylib variants under `variants/` and a committed CSV plus meta
plus findings trail, so what is owed is a bench rather than infrastructure.

The fork has two arms and they should be separate benches, because the two mechanisms are different and
averaging them would hide both:

**The lane-count arm, `W` from 1 to 32.** Variants: minimum container with projection, against headroom
container. Both vectorise, so this measures memory traffic and lane utilisation, and it wants sizes that
cross the cache levels, which is where a doubled footprint stops being a constant factor. The existing
`bitpack-footprint-*` variants are the closest prior shape and their sizing is the precedent.

**The vector-form arm, `W` from 33 to 64.** Same two variants. This one measures the loss of the vector form
and it is where instruction count is least informative, because the headroom side is a serial dependence
chain with a mispredictable back edge and the minimum side is largely independent work. Instruction counting
cannot see either effect.

**A `Precise` pair for both arms**, since `uqadd` versus a rebuilt clamp is a different comparison from
wrapping and the `Precise` gap is structurally the larger one.

**An operation-density sweep**, one, three and eight operations between observations, which is the variable
the projection's cost divides by and the one no instruction count can settle.

Until those run, the correct status of every ratio in this area, `139`'s and mine alike, is that it is an
ad-hoc quick spike with no substance and it prices nothing.

## 8. What compiles, and what I did not check

Everything below is from `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2024, `-O`,
`--crate-type lib -C panic=abort`, aarch64-apple-darwin, sources and emitted assembly in `140_probes/`.

Qualitative positives, which this evidence can carry: eagerly-masked and lazily-masked forms fold to
identical symbols at operation depths of three, four and sixty-four, scalar and vector (`p1.s`, `p2.s`); the
projection never blocked autovectorisation at any width, sign or rung tested (`p2.s`, `p3.s`, `p4.s`);
`uqadd.2d` is selected for saturation at an exactly-filled width in the minimum container and is not
reachable from the headroom container (`p3.s`); a mask in front of a comparison survives and one in front of
a narrower mask is absorbed (`p1.s`); the left-justified form emits no mask for addition and pays a shift at
every read (`p5.s`).

Qualitative negatives: `u128` does not vectorise on this target at any width tested, so every headroom
container from 33 to 64 bits is a rolled loop with a branch (`p2.s`, `p4.s`); signed sub-rung wrapping needs
two vector instructions rather than one because NEON has no vector bitfield extract (`p3.s`, `vw_i13_min`);
and `139`'s rolled-loop figures are four times too large, because the loops are unrolled by four and the body
was multiplied by the element count rather than the trip count (`rolled.py`).

**Unpriced, and named as such**: how much any of this is worth. That is section 7's list.

**Not checked at all, flagged so none of it is read as established.** Everything is aarch64 NEON on one
machine; x86-64 will differ and I did not run it. `Cold`'s bitpacked storage is untouched. Division,
remainder and right shift need the projection before rather than after, and I did not look at them. Compile
time is not considered. And every probe here is a spike: each checks one thing, takes shortcuts everywhere
else, is presumed flawed by construction, and should be read for what it proved rather than for how it was
written. Mine are no exception and the naming, arity and shape of each is scaffolding rather than design.

## 9. What is op's

**The rule is a call and it is his**, because it is the second of the two options he refused. I am not
restating it unchanged. What it lacked was a case, and the case I supply deliberately does not rest on a
ratio: the headroom is redundant where it works and wrong where it appears to do more, the projection it
would replace is amortised by the optimiser with no design mechanism, and the container it mandates forfeits
a vector form and, for `Precise`, a hardware instruction that is exactly the semantics. Those hold whatever
the harness eventually says about magnitude.

**The corrected number is his too, and it comes before the rule.** He ruled on 1600 against 81; the figure
from the same kind of spike is 339 against 81. I do not think his ruling needs revisiting, because what he
ruled was that a loop over values fitting a native register must not lose its vector form, and that survives.
But the record should carry 339, and the fork should carry a harness result rather than either number.

**One item in the rule is a correction rather than a call**: `131:277`'s "single-operation overflow room".

**One item is genuinely open and I did not settle it.** `Warm` is the default and everything here assumes its
semantics are wrapping at `W`. Nobody has tested that; `131:275-280` asserts it and four files have carried
it. If the default's semantics are the real question, the container rule is downstream of a decision nobody
has made.

**And the left-justified representation is his to know about rather than to decide.** It is the only
mechanism I found that removes the projection outright, it works, and it costs the `Transparent` unwrap door.
I would not take it. I would rather he saw it and refused it than that it resurface in six months as a fresh
idea.

## 10. Where I concede

Being exact about which part of this is an answer.

**The structural findings are solid and are what the evidence can carry.** What wrapping requires at each of
the two cases, that the projection sinks and the compiler does it, that the headroom container cannot
vectorise and cannot reach `uqadd`, that `Precise` wants the same answer more strongly, where the boundary
sits, and that the number in the record is four times too large. Those are compiled, committed beside this
file, and I stand behind them.

**The quantitative half is conceded outright.** I was asked what the rule costs at the instruction level and
I have instruction counts, but the rule that landed today says an instruction count taken outside the harness
is an ad-hoc quick spike with no substance and cannot price a fork. It is right, the fork is a how-much
question, and I did not run the harness. So the cost is unpriced, section 7 says what would price it, and I
would rather hand that back than let a second set of uncalibrated ratios harden into a premise the way the
first set did.

**The rule I propose is not a new answer.** It is deletion, which op has already seen and refused, now
carrying the case it was missing. I looked for a third option; the only one I found is the left-justified
representation, which loses. If op's refusal was of deletion *as a shape* rather than *as presented*, then I
have not found what he asked for, and the honest thing is to say I did not find it rather than to dress the
same shape in new headings.

**The framing I would most want attacked is my own.** I read "find a better answer" as "find a better
container rule" and priced container rules. The better answer may not be a container rule at all: it may be
that `Warm`'s *semantics*, rather than `Warm`'s storage, are what should change, so the default stops needing
room to wrap in. That is not a codegen question, I am not the right reader for it, and I have deliberately
not guessed. Someone who reasons about what a default should promise should look at it before this rule is
written down.
