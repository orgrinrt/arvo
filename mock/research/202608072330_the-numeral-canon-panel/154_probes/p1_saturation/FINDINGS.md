# P1 findings: saturation is entailed by I15, not merely preferred by it

Build: `rustc --edition 2024 -O --crate-type lib --emit asm -C panic=abort -o sat.s sat.rs`,
`nightly-2026-05-28` per `rust-toolchain.toml`, host aarch64-apple-darwin. Sources and the full
emitted assembly are committed beside this file.

**This is an ad-hoc quick spike with no substance as a measurement.** It is read only for
qualitative presence and absence: is there a variable-operand shift, is there a comparison, is the
mask an immediate. No magnitude is claimed and nothing here prices anything. `mock/benches/` is the
only thing in this repository that can price anything, and it was not used.

## The negative control, stated before the run, and its outcome

Arm A (const width) had to show its mask as a literal immediate and no variable shift; arm B
(runtime width) had to show a variable shift. Either failing makes the arms indistinguishable and
voids the probe.

**Passed.** `a_sat_47` is `and x0, x0, #0x7fffffffffff` (`sat.s:26`). `b_unsat` is
`lsl x10, x9, x8` with `x8` loaded from the value (`sat.s:33-35`).

## F1. A width that is not compile-time known produces a runtime check, unbidden

`b_unsat` (`sat.s:31-39`) contains `cmp w8, #63` and `csinv`. That is the source's `if v.w >= 64`
guard surviving into the emitted code. The compiler had no choice: it cannot know `w < 64`.

I15 (`INTENTS.md:305-307`) reads:

> Never any runtime checks, ever. We catch invalids on compile time, and unused paths we clear out
> when lowered. Period.

So the entailment runs: a parameter left runtime forces a check; I15 forbids the check; therefore
I15 forbids the parameter being left runtime. **Saturation is not a taste about how to spell a
primitive. It is what I15 says, restated at the type level.**

`holds for:` N = 13 and 47, container = u64, mask-and and saturating-clamp operations, arity 1,
target features = baseline aarch64-apple-darwin, threads = 1, opt-level = 3, F = 0.

## F2. Runtime dispatch over a closed set produces exactly the shape I15 names and refuses

`c_dispatch` (`sat.s:42-48`) materialises both constants (`mov w8, #8191`, `mov x9,
#140737488355327`) and selects between them with `csel`. It is branchless and it is not one lowered
path: both paths are present, which is the opposite of "unused paths we clear out when lowered".

`INTENTS.md:313-316` records that I15 refuses `80` section 5.1's "value-gated arm, which materialises
both lowerings and selects with a `csel`". I have not read `80`. I reproduced that exact shape from
the assembly, cold, and it is the shape a closed-set runtime index lowers to. That is independent
corroboration of what I15 rules out, arrived at from the machine rather than from the file.

`holds for:` same predicate as F1, plus dispatch set size = 2.

## F3. Adding a second compile-time index costs nothing in the lowered form

`a2_sat_13_wrap` (`sat.s:19-22`) is one `and`. `a2_sat_13_satur` (`sat.s:11-16`) is three
instructions, branchless. Two compile-time indices, still flat. **The count of compile-time
parameters is not what costs. Whether any one of them is left runtime is.**

`holds for:` N = 13, container = u64, two indices (width and policy), policy in {mask, clamp},
arity 1, target features = baseline aarch64-apple-darwin, threads = 1, opt-level = 3, F = 0.

## F4. The unpredicted one: two distinct primitives collapsed to one symbol

The last two lines of `sat.s`:

```
	.globl	_a_sat_13
_a_sat_13 = _a2_sat_13_wrap
```

`a_sat_13` and `a2_sat_13_wrap` are distinct functions at distinct source-level types. The linker
aliased them, because their lowered bodies are identical.

**So identity at the primitive level has, by construction, no runtime residue.** Two primitives that
compute the same thing are the same machine object. This was not predicted and it is the finding I
would keep if I could keep only one, because it decides what a *name* can possibly buy: not a
distinction the machine can see, since the machine erases it, but a distinction that can be
*stated* -- a law, a predicate, a perimeter. Section 4 of the file builds on this.

`holds for:` N = 13, container = u64, mask operation, arity 1, target features = baseline
aarch64-apple-darwin, threads = 1, opt-level = 3, F = 0.

## What would refute each

F1: a lowering in which a runtime width produces no comparison and no variable shift. On this ISA
that needs the width to be provably bounded, which is the same as saturating it.
F2: a closed-set runtime index that emits only the taken arm's constants. That requires the index to
be const, i.e. not to be runtime.
F3: a construction whose lowered form grows with the number of const parameters. I did not sweep
past two and do not claim past two.
F4: two source-level primitives with identical semantics that do not merge. Trivially arranged by
making them differ; the finding is that when they do not differ, nothing preserves them.
