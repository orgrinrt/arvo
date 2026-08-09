# 103 probe outcomes

All at `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, edition 2024, no
feature gates beyond `const_trait_impl` (allowed, WATCH tier). Compiled by hand with `rustc`, not
cargo, because three of the questions are cross-crate coherence questions and a single crate cannot
ask them. Commands are recorded per probe.

## Probe 1: does branch B of the truth-contract fork exist at all

Three crates: `p1_foundation.rs` (notko's position, zero deps, declares the truth contract with no
`bool` anywhere and D15's Shape F wrapper generic over it), `p1_arvo.rs` (the platform crate,
declares `Bool` and implements the contract), `p1_consumer.rs` (`arvo-comb`'s position, three
spellings of `greedy_group`).

```
rustc --edition 2024 --crate-type lib --crate-name p1_foundation p1_foundation.rs
rustc --edition 2024 -L . --crate-type lib --crate-name p1_arvo --extern p1_foundation=... p1_arvo.rs
rustc --edition 2024 -L . --crate-type lib --crate-name p1_consumer --extern ... p1_consumer.rs
```

**WORKS.** All three compile clean. `impl Truth for Bool` in the arvo-position crate is orphan-legal
(foreign trait, local type), which is the case D5's own orphan argument could NOT get for `Cardinal`
(foreign trait AND foreign type), and it is why the truth contract can sit where `Cardinal` could
not.

D15's `describes!` table survives genericisation over the truth type: the output parameter `B` is
constrained by the `Fn` bound's associated `Output`, so it is not an unconstrained impl parameter.
**Zero extra impl rows; one extra type parameter per row.**

Bound count through a three-level call chain, counted in the source: branch A restates 1 bound per
level (`F: Fn(&u32, &u32) -> Bool`), branch B restates 2 (`F: Fn(&u32, &u32) -> B`, `B: TruthHolds`).
Invocation needs no bound in either branch: Shape F's `Deref` carries it and the arity bound lives
only at `Pred::new`.

## Probe 2: what the fork costs in emitted instructions

```
rustc --edition 2024 -L . -C opt-level=3 -C panic=abort --emit asm ... p2_codegen.rs
```

With the wrapper's constructor `#[inline(always)]`:

| function | instructions | calls in loop |
|---|---|---|
| `run_a` (concrete `Bool`) | 22 | 0 |
| `run_b1` (generic, `holds()` exit) | **`_run_b1 = _run_a`** | 0 |
| `run_b2` (generic, `select`, no `bool`) | 16 | 0 |
| `run_bare` (no predicate abstraction) | 16 | 0 |

`run_b1` is not merely equal in count. The assembler emitted `_run_b1 = _run_a`, a symbol alias:
branch B lowered to byte-identical code and LLVM merged the two functions. **The fork costs zero at
runtime, established by symbol identity rather than by counting.**

`run_b2` is a different source program (a select over a pair rather than a branch), so its 16 against
`run_a`'s 22 is not a comparison of the two branches and is not offered as one. What it does show is
that a truth contract which never names `bool` can still drive control flow, and that the selector
lowers to `csel` rather than to a call.

**Without `#[inline]` on the wrapper's constructor**, the same four functions are 34 / aliased / 34 /
16, and the inner loop carries `bl __RNv...p1_arvoNtB2_4Bool3new`, one call per element. That is the
price of the crate boundary D27 creates, and it is entirely removable by one attribute.

## Probe 3: does the typestate seal survive the crate split

```
rustc --edition 2024 -L . --crate-type lib --crate-name p3 --extern ... p3_negative.rs
```

**FAILS, as required.** `error[E0593]: closure is expected to take 2 arguments, but it takes 1
argument`, with `required for Cons<u32, Cons<u32, Nil>> to implement Describes<{closure}>` and
`required by a bound in Pred::<L, F>::new`. The sketch verified this inside one crate; D15's
placement puts the wrapper in notko and the closure in a consumer, and the seal holds across that
split with the diagnostic pointing at the consumer's own line.

## Probe 4: what D15's property buys, in instructions

```
rustc --edition 2024 -L . -C opt-level=3 -C panic=abort --emit asm ... p4_property.rs
```

`group_promising` (property asserted true): 27 instructions, 7 branch-class ops.
`group_silent` (property absent): 29 instructions, 8 branch-class ops.

**The promise deletes one branch and two instructions, at group-open, not per item.** The predicate
call the round's topic also counts was already inlined and folded, because the closure is trivial;
against a non-trivial closure it would be a call, and this probe does not measure that case.

The number is small and is not the argument. The round's own topic says so
(`202607290100:56-58`: "the value is not codegen ... the value is that the property cannot desync"),
and this probe exists so nobody later re-derives the mechanism as a speed argument.

## Probe 5: can the property reach file 07's rung 1

```
rustc --edition 2024 -L . --crate-type lib --crate-name p5 --extern ... p5_witness_attempt.rs
```

**FAILS, and the failure is the result.** `error[E0277]: the trait bound F: [const] Fn(&u32, &u32) is
not satisfied`. A consumer's closure is not const-callable, so the const evaluator cannot invoke the
predicate once, let alone exhaustively over its domain. The witness has no expressible form. The
property is therefore not rung 1 under any encoding, which places it on rung 2 and settles a question
that would otherwise be argued.

## Probe 6: which platform facts are settled at compile time

```
rustc --edition 2024 --crate-type lib --crate-name p6 p6_platform_facts.rs
```

**WORKS.** Pointer width, `size_of::<bool>()`, `align_of::<bool>()`, `size_of::<Option<bool>>() == 1`,
`size_of::<Option<NonZeroUsize>>() == size_of::<usize>()`, and endianness are all const-assertable.
None needed a runtime check; none is an ambient control state a linked library could perturb.

The refusal shape fires: with the threshold moved to 128 to force it,
`error[E0080]: evaluation panicked: arvo's capacity model needs at least a 32-bit index domain`, the
same `E0080` the capacity repair uses.

**Owed:** the refusal firing on a real small target. `thumbv6m-none-eabi` and `msp430-none-elf` are
not installed on this host (`error[E0463]: can't find crate for core`), so the cross-target half is
unverified and is named rather than assumed.

## Probe 7: does an associated const reach rung 2's totality

```
rustc --edition 2024 --crate-type lib --crate-name p7 p7_totality.rs
```

**FAILS on the undefaulted encoding, as required.** With a default, `impl Defaulted for Careless {}`
compiles and has silently promised. Without one, `impl Total for Silent {}` is
`error[E0046]: not all trait items implemented, missing: FRESH_ALWAYS_ACCEPTS`. This is file 07's own
`a6` finding (an overridable default member is not a load-bearing site) at the property's address.

## Probe 8: is there a second truth type

```
rustc --edition 2024 -L . -C opt-level=3 -C panic=abort --emit asm ... p8_second_truth.rs
```

**WORKS.** One predicate declaration, `all_hold<F, B: Truth>`, monomorphises to both `Bool` and a
64-lane `Mask64` modelled on `MaskOps` (`arvo-mask-contracts/src/lib.rs:45-66`, whose
`empty`/`full`/`union`/`intersection`/`complement` are `FALSE`/`TRUE`/`or`/`and`/`not` under a
set-theoretic vocabulary). No gates, no `dyn`, both paths monomorphised.

Instruction counts (91 lane-path, 109 scalar-path) are **not** offered as a comparison: the two
functions compute different things and neither loop is the shape a real lane-wise predicate would
have. What the probe establishes is only that one declaration serves two truth types.
