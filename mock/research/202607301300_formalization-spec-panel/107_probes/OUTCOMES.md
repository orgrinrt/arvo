# 107_probes outcomes

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml` **inside the tree**. HEAD `be66678`, all runs 2026-08-05.

**The toolchain trap, confirmed a third time, from this directory.** `rustc --version` from
`107_probes/` reports the pinned nightly; the identical command from `/tmp` reports
`rustc 1.94.0 (4a4ef493e 2026-03-02)`. Stable does not parse `type const` and reports it as an
ordinary parse error, so probes 1 and 6 would report a refusal about the wrong thing.
`100_probes/OUTCOMES.md` and `106_probes/OUTCOMES.md` both record the same trap. Three files is past
convention: it belongs in the panel's probe conventions.

| Probe | Subject | Outcome |
|---|---|---|
| 1 | can a const-parameter capacity produce a capacity | REFUSED, both gate spellings |
| 2 | can an inductive numeral produce one | WORKS, zero gates, 98 compile-time assertions |
| 3 | where the numeral's value actually fails | value position WORKS, type position REFUSED, one grammar |
| 4 | the array derived from the numeral's structure | WORKS, zero gates. **REPRODUCTION of `76_probes/b1`**, extended |
| 5 | negative control and execution | good runs; broken `I<P>` refuses at `E0080` through the generic door |
| 6 | which direction a derivation can run | numeral to storage WORKS; const to numeral REFUSED generically |
| 7 | compile cost | flat in N; the cost is the constructor's shape, not the type's, 27x |
| 8 | emitted code | three of four operation pairs merged to one symbol by LLVM |
| 9 | the paired form at a computed capacity | concrete WORKS with a hand-computed literal; generic REFUSED |

**What is new here and what is not, stated first because probe 4 is not new.**
`76_probes/b1_structural_array.rs` built probe 4's construction, with a negative control (`b1b`) and a
perimeter control (`b1c`), and `76_probes/OUTCOMES.md:44` records it as "WORKS, zero feature gates".
File 76 section 3 named it "construction one", priced it, and recommended against it on four stated
grounds. Probe 4 reproduces it on the current pin and adds one case `b1` does not have: the law under a
capacity produced by type-level arithmetic rather than declared (claim C). Probes 1, 7, 8 and 9 are new.
Probe 2's arithmetic is new as a probe; the tower's own `Cmp`/`Gcd` family is settled shape. Probe 6's
claim C is new and refutes `106b`'s own suggested direction.

---

## Probe 1, `p1_const_capacity_cannot_produce_a_capacity.rs`

```
rustc --edition 2024 --crate-type=lib p1_const_capacity_cannot_produce_a_capacity.rs -o /tmp/p1.rlib
```

Reading a const-parameter capacity works (`READ_13 == 13`, `size_of::<Array<u8>> == 13`), reproducing
`106_probes/p3`. Producing one refuses. Under `min_generic_const_args`, first:

```
error: complex const arguments must be placed inside of a `const` block
60 |     type Out = Dim<{ A + B }>;
```

and with the const block supplied:

```
error: generic parameters may not be used in const operations
60 |     type Out = Dim<const { A + B }>;
    = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

`generic_const_args` is not in `unstable-features.md`'s allowed table and the rule records it as needing
`-Znext-solver=globally` plus a roughly 314-site rewrite.

Gate-free, the same operation gives the older diagnostic and rustc states the confinement in its own
words:

```
error: generic parameters may not be used in const operations
7 |     type Out = Dim<{ A + B }>;
  = help: const parameters may only be used as standalone arguments here, i.e. `A`
  = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

`generic_const_exprs` is FORBIDDEN (`unstable-features.md`, forbidden table, op 2026-07-28).

**"Standalone arguments here" is the whole cost of the const column, in the compiler's phrasing.**

## Probe 2, `p2_numeral_produces_capacities_gate_free.rs`

```
rustc --edition 2024 --crate-type=lib p2_numeral_produces_capacities_gate_free.rs -o /tmp/p2.rlib   # EXIT=0
```

Zero feature gates. `Inc`, `Add` and `AddC` over the sealed `H | O<P> | I<P>` grammar, twenty-one
disjoint impls, no specialization, no overlap. Every sum in 1..=7 by 1..=7 asserted at compile time
against the arithmetic on the values, both with and without carry: **98 assertions, all holding.**
`Sum<N5, N7>` is a type downstream code names.

**Claim B is the part no panel file has stated.** `const VAL: usize = 2 * P::VAL + 1` is an ordinary
associated const with a generic parameter inside the expression. That is stable Rust. The numeral's
value was never the thing that could not be obtained.

## Probe 3, `p3_the_scissors.rs`

One grammar, both blades.

```
head -57 p3_the_scissors.rs > /tmp/p3_value_half.rs
rustc --edition 2024 --crate-type=lib /tmp/p3_value_half.rs -o /tmp/p3v.rlib   # EXIT=0
```

The value half alone compiles: `<N13 as Pos>::VAL == 13`, and a `const fn count<C: Pos>() -> usize`
returning `C::VAL`, asserted at compile time. Adding the array-length half and nothing else:

```
error: generic parameters may not be used in const operations
62 |     type Array<T: Copy> = [T; <C as Pos>::VAL];
    = note: type parameters may not be used in const expressions
    = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

Same const, same impls, same grammar. **Type position is the entire refusal.**

## Probe 4, `p4_the_array_is_derived_not_paired.rs`

```
rustc --edition 2024 --crate-type=lib p4_the_array_is_derived_not_paired.rs -o /tmp/p4.rlib   # EXIT=0
```

Zero feature gates. `O<P>`'s storage is `Twin<P::Array<T>>`, `I<P>`'s is `TwinOne<P::Array<T>, T>`,
`H`'s is `[T; 1]`, `Z`'s is `[T; 0]`. Every array length in the file is a literal.

- 23 numerals asserted against their decimal values first, so a mis-spelling fails at the spelling.
  **This caught a real error: `N47` was first written `I<I<I<O<I<H>>>>>`, which is 55.** The storage law
  passed on it, correctly, because the law does not depend on knowing the decimal; the value assertion
  is what caught it, and the downstream `Sum<N47,N47> == 94` assertion caught it independently.
- The law `size_of::<Array<T>>() == VAL * size_of::<T>()` and `align_of::<Array<T>>() == align_of::<T>()`
  asserted at 23 numerals times 4 element types: `u8` (1,1), `u16` (2,2), `u32` (4,4), and a nine-byte
  `repr(C)` struct at align 1 whose size is not a power of two. **184 assertions.**
- `Z`: size 0, align 4 for `u32`, the degenerate instance rather than a written exception.
- **Claim C, which `b1` does not have.** `Pz<Sum<N5,N7>>` has `VAL == 12` and `size_of::<Array<u32>> == 48`;
  `Pz<Sum<N47,N47>>` has `VAL == 94` and `size_of::<Array<Odd9>> == 846`. Nobody declared a 12 or a 94
  anywhere in the file.
- The generic signature `concat_storage<A, B, T>() -> <Pz<Sum<A,B>> as Capacity>::Array<T>` compiles.
  Probe 9 claim B is the same signature under the paired form and cannot be written.

## Probe 5, `p5_negative_control_and_execution.rs`

```
rustc --edition 2024 -O p5_negative_control_and_execution.rs -o /tmp/p5good   # EXIT=0
/tmp/p5good   -> "p5 ok: 1, 7, 13, 47 round-tripped through the derived storage"
rustc --edition 2024 -O --cfg broken p5_negative_control_and_execution.rs -o /tmp/p5broken   # EXIT=1
```

Executed: every slot written through the mutable projection and read back through the shared one at
capacities 1, 7, 13 and 47, with the raw byte length checked so trailing padding would show.

The negative control changes exactly one line, `I<P>`'s storage from `TwinOne` to `Twin`:

```
error[E0080]: evaluation panicked: storage law violated: array grammar does not match the numeral
  --> p5_negative_control_and_execution.rs:93:9
   | evaluation of `as_slice::<I<I<H>>, u32>::{constant#0}` failed here
```

**It fires through the generic projection**, which names no numeral, no length and no law, at a
monomorphisation the caller chose. Same shape as `76_probes/b1c`.

## Probe 6, `p6_can_the_numeral_be_derived_from_a_const.rs`

```
rustc --edition 2024 --crate-type=lib p6_...rs -o /tmp/p6.rlib              # EXIT=0 (emission half)
rustc --edition 2024 --crate-type=lib --cfg refuse p6_...rs -o /tmp/p6r.rlib # EXIT=1 (generic half)
```

**Claim D, emission: works.** A macro handed the digits emits both the numeral and the const from one
invocation, with the agreement asserted inside the expansion. The first draft of those six lines had
five of six digit strings wrong and every one failed at the emission site.

**Claim C, generic projection: refused.**

```
error: generic parameters may not be used in const operations
92 |         Dim<{ N / 2 }>: Project,
   = help: const parameters may only be used as standalone arguments here, i.e. `N`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

So a derivation runs numeral-to-storage in the type system, and const-to-numeral only by emission,
which is a build-layer contract rather than a type-system one.

## Probe 7, `p7_compile_cost.py` and the two follow-ups

```
python3 p7_compile_cost.py    # cwd must be inside the tree; the script refuses otherwise
```

**Compile time is flat in the capacity, all three shapes, best of five.** Seconds:

| N | const | derived | paired | numeral depth |
|---|---|---|---|---|
| 1 | 0.042 | 0.045 | 0.043 | 1 |
| 47 | 0.042 | 0.045 | 0.044 | 6 |
| 4096 | 0.043 | 0.046 | 0.044 | 13 |
| 65536 | 0.043 | 0.048 | 0.045 | 17 |
| 1048576 | 0.042 | 0.045 | 0.045 | 21 |

All three sit at rustc's process-start floor. Nothing grows with N.

**The monomorphisation axis, where a real cost appeared and turned out to be the wrong thing.**
K distinct capacities `3, 10, ..., 3+7(K-1)`, `-O`, seconds:

| K | derived (recursive constructor) | const |
|---|---|---|
| 10 | 0.09 | 0.09 |
| 25 | 0.28 | 0.15 |
| 50 | 1.22 | 0.28 |
| 100 | 5.31 | 0.73 |

Isolating it at K = 100 between a body that only names the type and one that constructs:

| regime | type only | with recursive `filled` |
|---|---|---|
| sum of capacities 10,200 | 0.17 | 0.83 |
| sum of capacities 34,950 | 0.08 | 3.24 |

**The type machinery is free; the cost is one operation per element, emitted because `filled` was
written as structural recursion and inlined.** Writing `filled` and `slice` ONCE as provided trait
methods over the projected slice, with no recursion in any body:

| shape | K=100, sum 10,200 | K=100, sum 34,950 | K=400, sum 560,400 |
|---|---|---|---|
| derived, provided methods | 0.25 | 0.12 | 0.39 |
| const parameter, identical bodies | 0.09 | 0.09 | 0.22 |

3.24 s to 0.12 s, a 27x collapse, for identical semantics. The residual is 1.3x to 1.8x, in tenths of a
second at four hundred distinct capacities.

**The design instruction this yields: recur the type, never recur the code.**

## Probe 8, `p8_codegen.rs`

```
rustc --edition 2024 -O --crate-type=lib --emit=asm p8_codegen.rs -o /tmp/p8.s   # EXIT=0
```

Four operations at capacity 13, each written twice, against the derived storage and against
`[u32; 13]`, in one binary. LLVM merged three of the four pairs into a single symbol:

```
_native_sum  = _derived_sum
_native_fill = _derived_fill
_native_copy = _derived_copy
```

`derived_sum` vectorises to NEON (`ldp q1, q0`, `add.4s`, `addv.4s`), and `native_sum` is that symbol.

The fourth, `native_get` against `derived_get`, did not merge and is instruction-for-instruction
identical:

```
cmp x1, #12 / b.hi LBB / ldr w0, [x0, x1, lsl #2] / ret
```

with the only difference being which `Location` constant is passed to `panic_bounds_check`, whose
payload differs at one byte (the source column of the indexing expression). A source-location record,
not code.

**Symbol identity, which is the same class of evidence file 103 used for the truth-contract fork and is
stronger than a measurement.**

## Probe 9, `p9_the_paired_form_at_a_computed_capacity.rs`

```
rustc --edition 2024 --crate-type=lib p9_...rs -o /tmp/p9.rlib               # EXIT=0
rustc --edition 2024 --crate-type=lib --cfg refuse p9_...rs -o /tmp/p9r.rlib # EXIT=1
```

The ratified `Slot<P, const K: usize>` from `76_probes/b2`, fed a capacity nobody declared.

**Concrete: works, and the `12` is a number a human did arithmetic to produce.**
`Slot<Sum<N5,N7>, 12>` compiles and the agreement check is the only thing between that line and a wrong
answer.

**Generic: cannot be spelled.** The only honest thing to write in the literal's position is the value
the numeral already knows:

```
error: generic parameters may not be used in const operations
158 | pub fn concat_storage<A, B, T>() -> <Slot<Sum<A, B>, { <Sum<A, B> as Pos>::VAL }> as Capacity>::Array<T>
    | cannot perform const operation using `A`
    = help: add `#![feature(generic_const_exprs)]`
```

The nearest legal spelling takes `const K: usize` as a parameter and trusts the caller. It compiles, and
it is the finding: **under the paired form a generic capacity-producing operation has no storage, and
the only way to give it one is to ask its caller for a number.**
