# Sketch findings: selecting an algorithm from a property carried in the type

**Date:** 2026-07-29
**Outcome:** **WORKS.** No feature gates. Verified from emitted assembly, not from reasoning.
**Rests on:** the `Pred` investigation in `202607282230_hlist-arity-dissolution/`, which asked why a
typed predicate should exist at all. This is the answer that survived.

## The question

`arvo-comb` takes predicates as `impl Fn(&A, &T) -> Bool`. A closure type carries no properties, so
an algorithm cannot know anything about the predicate it was handed. If a property could be carried
in the type, the library could select a provably-legal-only-under-that-property algorithm without the
consumer ever naming one.

That is the same shape as `Strategy`, one level up. `Hot` / `Warm` / `Cold` / `Precise` are properties
in the type that select storage. `MONOTONE` would be a property in the type that selects an algorithm.

## Result 1: it works, and the discarded algorithm is not merely cold, it is absent

`breadth.rs`. Three properties (`MONOTONE`, `SORTED_OK`, `CHEAP`), three nested static branches, four
algorithms, three predicate types.

Proved by marking each algorithm with an `#[inline(never)]` symbol and reading which one each
instantiation references:

```
_sel_general  (line 37)  ->  bl _algo_b_marker  (line 50)   linear scan
_sel_monotone (line 80)  ->  bl _algo_a_marker  (line 94)   binary search
```

Two `bl` instructions in the entire file, one per instantiation. The other algorithm is not behind a
branch and not in a cold section; it does not exist in that instantiation. The `if P::MONOTONE`
never reaches runtime.

**The `bl` is instrumentation.** Without the `#[inline(never)]` markers the selected algorithm
inlines and no call remains. It is there only to make the selection observable.

**Correctness, not just speed.** Binary search is *wrong* for a non-monotone predicate. The type
system is what keeps it away from `Parity`.

## Result 2: at one choice, typestate buys nothing over the alternatives

`encodings.rs`. The same single selection written four ways, each behind a real function boundary so
inlining cannot rescue anything:

```
_cg = _cf
_ts = _cf
```

LLVM emitted **one body** and aliased the other two to it. Typestate, const-generic `bool` and
`const fn` are byte-identical, not merely comparable. The runtime-`bool` version differs only because
it was forced across an `#[inline(never)]` boundary; inlined against a literal it folds too.

**So "typestate gives better codegen" is false and this experiment falsifies it.** At a single
choice the compiler does not care how the constant arrived.

## Result 3: at breadth the aliasing disappears, because the bodies genuinely differ

Same `breadth.rs`, three predicates through the nested selection:

| Instantiation | Properties | Algorithm chosen | Instructions |
|---|---|---|---|
| `p_parity` | none, `CHEAP` | scan | 14 |
| `p_budget` | `MONOTONE` + `SORTED_OK` | galloping | 18 |
| `p_costly` | `MONOTONE`, not `SORTED_OK`, not `CHEAP` | bisect | 98 |

Three distinct bodies, no merging. Result 2 was "three encodings of *the same* choice are identical",
which is a narrower claim than it first appears and does not generalise to "typestate buys nothing".

## Result 4: `const fn` cannot express this at all, and that is the scaling difference

`constfn_limit.rs`. The natural alternative is a `const fn` returning the property. It does not work,
and it fails silently rather than loudly:

```rust
const fn is_monotone<P>() -> bool { true }   // no way to inspect P
pub fn select<P>(v: &[u32]) -> usize {
    if is_monotone::<P>() { bisect(v) } else { scan(v) }
}
```

This **compiles**, and `select::<Budget>` and `select::<Parity>` both take the same branch. Printed
output is `2 2`. A `const fn` has no way to dispatch on a type parameter, so it can only carry a
global constant, never a per-type one.

The only construct that makes a compile-time constant vary by type **is an associated const**, which
is typestate. So at breadth the choice is not between three equivalent encodings:

- `const fn` is not expressible.
- const-generic `bool` is expressible, but the promise lives at the **call site**, so every call site
  must state every property correctly and stay correct as predicates change.
- typestate attaches the promise to the **predicate type**, where it cannot desync.

## What the value actually is

Not codegen. Result 2 rules that out. The value is that **the property cannot desync from the thing
it describes**.

`Budget` carries `MONOTONE = true`. `Parity` cannot accidentally claim it. A consumer cannot pass the
wrong flag because there is no flag to pass. With a `const fn` the property is a free function tied to
nothing; with a const-generic `bool` it is a value restated at every call site.

That is a correctness and maintenance property, and it is the sole justification for the mechanism,
which makes the obligation below load-bearing rather than incidental.

## The soundness obligation this creates

`MONOTONE = true` is an **unchecked promise**. If a consumer asserts it falsely the library runs a
different algorithm and returns a wrong answer, silently.

This is a different risk class from `Strategy`, where a wrong choice costs performance and never
correctness. It is closer to an `unsafe` contract than to a tuning knob, and it should be treated as
one: a stated contract per property, and ideally a debug-mode assertion cross-checking the selected
algorithm against the naive one.

Not designed here. Named because the mechanism is worthless without it.

## What is proven and what is not

**Proven:** the mechanism works; the discarded algorithm is absent rather than cold; the three
compile-time encodings are identical at one choice and diverge at breadth; `const fn` cannot
discriminate on a type.

**Not proven:** that any property is worth encoding. `MONOTONE`, `SORTED_OK` and `CHEAP` were invented
to exercise the mechanism. Whether `greedy_group`, `matrix_chain_dp` or `bin_pack` have a property
that legitimately unlocks a better algorithm is a **bench question**, and per
`bench-in-bench-harness-never-sketches.md` it belongs in `mock/benches/` rather than here. The
existing arvo and hilavitkutin benches already show variants dominating in different bands; the
properties worth encoding are the ones that **name a band boundary**.

## Reproducing the assembly

The module sources have `#[unsafe(no_mangle)]` stripped so the sketch builds as an ordinary library.
To re-derive the asm claims, restore it on the entry points and:

```
rustc --edition 2024 -O --crate-type=lib --emit=asm -o out.s src/breadth.rs
grep -nE '^_p_[a-z]+:|^_p_[a-z]+ = ' out.s
```

Symbols carry a leading underscore on this target.

## Result 5: the shape alone specialises a single generic kernel, 8x

`shape_kernels.rs`. One generic `sum<C: Capacity>(v: &C::Array<u32>)`, four instantiations. Extent
lives in the type via `Capacity`'s associated `Array`, so no const expression reaches type position
and no feature gate is needed.

| Instantiation | Instructions | Vector ops | Conditional branches |
|---|---|---|---|
| `Dim<16>` | 8 | 2 | **0** |
| `Dim<17>` | 10 | 2 | **0** |
| `Dim<1024>` | 20 | 2 | 1 |
| runtime `&[u32]` | **65** | 3 | **9** |

Same source. Eight instructions and zero branches against sixty-five and nine, purely because the
extent is a type-level fact. `Dim<17>` has no tail **loop**: the remainder is two inline instructions.

Worth recording that the first attempt at this kernel was written as `&[u32; C::N]` with a
`where [(); C::N]:` bound and failed with `generic parameters may not be used in const operations`.
That is precisely the wall `Capacity`'s associated `Array` exists to remove, met by accident.

## Result 6: DERIVED properties select explicit microkernels, and nobody states anything

`microkernel_selection.rs`. `Capacity` gains two properties the type computes **from itself**:

```rust
const LANE_ALIGNED: bool = N % 4 == 0;
const SMALL: bool = N <= 8;
```

One entry point selects among three hand-written kernel bodies:

| Instantiation | Derived properties | Kernel | Instructions | Vector ops | Branches |
|---|---|---|---|---|---|
| `Dim<8>` | `SMALL` | flat | 5 | 1 | **0** |
| `Dim<64>` | `LANE_ALIGNED` | unrolled by 4 | 26 | 8 | **0** |
| `Dim<66>` | neither | pairwise | 39 | 8 | **0** |

Three distinct bodies, no merging, zero conditional branches in any of them. The consumer wrote
`Dim<8>`, `Dim<64>`, `Dim<66>` and nothing else.

**Why this is the finding.** Hand-written microkernel dispatch means writing the cross product by
hand: extent band times alignment times strategy times sign times rank, as cfg branches or a dispatch
table, every entry a place to get it wrong and to keep in sync. Here **the type is the cross
product**, the properties are derived from it, and impls exist only for combinations that earn one,
with a generic fallback for the rest. That is what makes per-shape microkernel selection practical at
a granularity that was not practical before.

**Derived and asserted properties are different risk classes and must stay visibly separate.**
`N % 4 == 0` is computed and cannot lie. `MONOTONE = true` is a promise and can. From a call site the
two mechanisms look identical, which is exactly why the distinction has to be explicit in whatever
ships.

## Why this matters to notko and arvo specifically

This is the mechanism notko's `#[profile]` AST rewriting is reaching for and the one arvo's `Strategy`
markers already are, generalised. `Strategy` is a property in the type selecting **storage**;
`Capacity` is typestate carrying an associated **array**; `AccessSet` is an hlist that already proves
**disjointness**. Nothing new has to be invented for the same solver-driven selection to reach kernel
choice: it is the trick the stack already uses in three places, pointed at a fourth.

It also lands squarely inside two existing rules rather than needing an exception.
`arvo-always-optimal-internals` licenses asm microkernels and cfg-gated intrinsics chosen by what
benches fastest; this adds a selection axis that is per-shape rather than per-target.
`arvo-toolbox-not-policer` requires the substrate not to hardcode thresholds or guess the workload;
a derived property does neither, because the consumer states a shape and the substrate reads a fact
about it.

## What is now unknown and matters more than before

**Monomorphisation cost.** Every distinct shape can instantiate a distinct kernel body.
`arvo-compile-time-last` explicitly licenses that trade, but where the binary-size and compile-time
curve bends has not been measured, and this makes it matter more than it did.

**Which properties earn their place.** Still the open question from Result 4, now sharper: derived
properties like `LANE_ALIGNED` are free and safe, asserted ones like `MONOTONE` carry a correctness
obligation. The bench bands already known in arvo and hilavitkutin are where the answer lives.
