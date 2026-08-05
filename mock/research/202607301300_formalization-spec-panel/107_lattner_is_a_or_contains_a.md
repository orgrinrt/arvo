# 107. Is a capacity a numeral or does it contain one: the third column was built at file 76, recommended against for four reasons, carried as op's own open item by the eighth consolidation, and dropped by the ninth without a droplist entry

Chris Lattner, file 107. I wrote file 12 (the fresh read), file 74 (the taxonomy rechecked), and file 83,
which established three width levels and one axis and found that a fork another file had opened dissolved
rather than resolved. This one does something I did not expect and did not want: it finds that the
question I was dispatched to open was opened thirty-one files ago, answered with a compiled construction,
priced, and then lost by exactly the mechanism file 106 spent its longest section naming.

**The short answer.** A capacity is a numeral. What it contains is not a second number but a projection,
and the projection is derivable from the numeral's own structure. The ratified sentence that says
otherwise, "the array grammar is a paired, non-derived fact, forced by the language, not chosen"
(`91:791-798`), is false in its first clause and true only of one spelling in its second. File 106 found
half of that. The other half was compiled at `76_probes/b1_structural_array.rs`, three files before the
file that ratified against it.

Three of file 79's four spec sentences survive this intact. One does not.

---

## What I read

`102_consolidation_ten.md` in full, the standing base. `106_giesen_one_pattern_or_two.md` in full and
`106b_persona_checkpoint_twentyfive.md` in full, which set this dispatch. Behind the consolidation,
because this dispatch is about a provenance failure: `77b_op_checkpoint_nineteen.md` in full, op's own
words, and `79_dolan_what_capacity_is.md` in full, the answer to the dispatch that checkpoint set.

Then, because the trail did not stop where the dispatch said it did:
`76_kiselyov_the_real_consumer_price.md` sections 2 and 3 in full plus its closing section,
`76_probes/OUTCOMES.md` in full and `76_probes/b1_structural_array.rs` at source,
`78_consolidation_eight.md:750-785` (the capacity section) and `:866-878` (the open list), and
`91_consolidation_nine.md:780-805` (the capacity section) and its droplist at `:1043`. One `ls` of the
panel directory, current through `106b`, and one of `76_probes/`.

From the shipped tree, for the two licensed purposes only:
`arvo-tensor/tests/capacity.rs` and `tests/const_capacity.rs:1-70` in full (the test gate below, and one
factual check on how generic consumers reach a capacity's elements today),
`arvo-tensor/src/capacity.rs:44-56` and `src/lib.rs:21` (whether the unpaired shape is buildable and at
what gate cost, re-checking file 106's claim rather than inheriting it). **Every judgement below survives
deleting its shipped-source citation.** Where one would not, I say so and withdraw it. Nothing under
`mock/crates/` was modified.

## Gates, run before the work

**Canon gate.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD `be66678`, run 2026-08-05 05:20 UTC. The numeral
tower still has no shipped source. Gate passed.

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary by parsing every
`test result:` line rather than trusting a headline: **155 binaries, 672 passed, 0 failed, 9 ignored**,
from a clean committed tree. Matches `102`, `103`, `104`, `105`, `106` and `106b` exactly.

**Test bodies read in the surface I touch**, which is capacity. `arvo-tensor/tests/capacity.rs` in full
and `tests/const_capacity.rs:1-70`. The two tautologies are present and unchanged
(`capacity.rs:14-18` and `const_capacity.rs:49-53`, each asserting `<Dim<N> as Capacity>::CAP == cap(N)`
against an impl whose body is `const CAP: Cap = cap(N)` at `src/capacity.rs:48`). They are op's, disposed
at `95b`, flagged for deletion by file 76 as its own open item 13 and carried in every open list since.
Twenty-eight files have now reported them; I add the count and nothing else.

The rest of that file is not tautological and one part of it is load-bearing for this dispatch.
`capacity.rs:31-59` is generic over `C: Capacity`, builds through `filled`, and reaches every element
through `as_ref`/`as_mut` **slices**, reading `slots.len()` at runtime; `:67-96` composes two capacities
and does the same at both levels. That is a factual check on how a generic consumer reaches a capacity's
elements today, and section 6 uses it.

**Toolchain, and the trap that has now cost three files.** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`, resolved from `rust-toolchain.toml` inside the tree. From `/tmp` the identical
command reports `rustc 1.94.0 (4a4ef493e 2026-03-02)`, which does not parse `type const` and reports it as
an ordinary parse error. `100_probes/OUTCOMES.md` and `106_probes/OUTCOMES.md` both record it. Three is
past convention. Every probe below ran with `107_probes/` as cwd; commands and verbatim diagnostics are in
`107_probes/OUTCOMES.md`.

---

## 0. The answer, first

**Is a capacity a numeral, or does it contain one?** It is a numeral, and what it contains is a
projection rather than a second number. Four readings of op's sentence are available and the compiled
evidence separates them cleanly, which is the useful part.

| Reading | Shape | Second number that can disagree | Storage for a computed capacity |
|---|---|---|---|
| (a) wrapper | `struct Capacity { n: Numeral }` | no | unaddressed |
| (b) subtrait, as file 79 wrote it | `Capacity: Nat` | no | unaddressed, and the gap is what (d) arrived to fill |
| (c) numeral in a role | `Capacity: Nat` plus a derived `Array` | no | **yes, generically** (`p4`) |
| (d) pairing | `Slot<N, const K>` | **yes, one per declared capacity** | **no, not generically** (`p9`) |

File 79 considered (a), rejected it on the carrier-at-birth rule, and chose (b). That was right, and the
reason it gave is right. What (b) left open is the storage, and the ratified answer filled it with (d).
Reading (c) fills it with a projection instead, and (c) is what `76_probes/b1` compiled thirty-one files
ago.

**Three of file 79's four spec sentences survive whole.** Capacity is a parameter and not an event; its
value is a direct instance of the tower's `Nat`; the far-point rule fires one layer downstream at index
arithmetic with `Dec`/`PosPred`. None of those is touched by anything below, and the first is the sharpest
thing anyone has written about capacity in this panel.

**The fourth does not survive.** "The array grammar is a paired, non-derived fact, forced by the language,
not chosen." Its second clause, as file 79 phrased it, is about `[T; K]` specifically and is true. Its
first clause is false, and file 76's own outcomes table said so in one word before file 79 was written.

**And the largest thing in this file is not the design question.** Section 2.

---

## 1. What I was told was unposed, and where it actually sits

`106b:101-107` offers the derivation as an instinct nobody has posed, to be stress-tested, and says
plainly it is uncompiled and one pass. I set out to compile or kill it. It is already compiled, in this
panel, with two controls, and the record of what happened to it is the finding.

`76_probes/OUTCOMES.md:44`, verbatim:

> | `b1_structural_array.rs` | derive the array structurally from the binary encoding, `repr(C)` | WORKS,
> zero feature gates. Layout law asserted over 8 capacities (0, 1, 5, 7, 13, 28, 64, 4096) at 3 element
> types (`u8`, `u32`, `u128`), 24 compile-time assertions. The cast's precondition is discharged inside
> `as_slice` by an inline const block, so it is evaluated per monomorphisation rather than per remembered
> list |

File 76 named it **construction one**, built a negative control (`b1b`, which "REFUSES at exactly 4 of the
8 capacities, and exactly the 4 whose encodings contain `I`") and a perimeter control (`b1c`, which
refuses a capacity no assertion list names, at the door). It then recommended **construction two**, the
pairing, and gave four reasons (`76:220-226`). Section 6 takes those four reasons seriously, because they
are good ones and three facts that arrived after them are what change the balance.

It also filed this, in its own closing (`76:487-489`):

> **Open, and op's.** [...] Whether construction one is kept in the spec as a recorded fallback or
> dropped.

**The eighth consolidation carried it.** `78:752-754` states construction one in the capacity section, and
`78:872-873` carries it as open item 12:

> 12. **New.** Whether construction one (the structural, `unsafe`-discharged-at-the-door array) stays in
>     the spec as a recorded fallback to construction two, or is dropped (section 1.26).

**The ninth consolidation dropped it.** `grep -c 'construction one\|recorded fallback'` over
`91_consolidation_nine.md` returns **0**, and over `102_consolidation_ten.md` returns **0**, run
2026-08-05 05:29 UTC. It is not in the ninth's droplist (`91:1043` onward, scanned in full; the nearest
entry is about file 80's fold-width construction, an unrelated subject). Item 13 from the same list, the
tautological test, survived and is still being reported twenty-eight files later.

**And the sentence that replaced it asserts the opposite of what the dropped construction proves.**
`91:791-798`: "The array grammar is a paired, non-derived fact, forced by the language, not chosen."

I want to be exact about file 79's own share of this, because it is smaller than it looks and the register
matters. File 79's section 4 says "This half is not new; I confirm it rather than re-derive it", and it
confirms the `a1`-through-`a3b` half of file 76 precisely and correctly. It lists `76_probes/OUTCOMES.md`
among what it read. It does not mention the `b1` row. That is selective confirmation rather than an
error, and it is the same shape file 106 confesses about its own file 34 (`106:451-478`): a generalisation
from the witnesses in hand, overturned by someone who built a witness the author had not. Here nobody had
to build one. It was in the table.

**What compounded it is the compression.** File 79's clause is narrow ("no expression of `[T; K]`
computed from a type-level `Nat`") and narrowly true. The ninth's absorption is wide ("non-derived",
"forced by the language, not chosen") and false. That is precisely the failure file 106 section 4.3
proposes a discipline line for, found on a second body of material, one section over, in the same
consolidation pair. It is the third instance this stretch, and the first where the lost item was
explicitly addressed to op.

**So this dispatch's own framing needs correcting before its question can be answered.** The fork is not
two columns with a third instinct to be tested. It is **three columns, all three compiled, one of which
was already priced against the others once and lost its record.**

---

## 2. What the numeral column actually buys, compiled

The dispatch asks which of the design's settled mechanisms need a capacity inside type-level arithmetic
whose result is a type, and says that if the answer is none, that is a large finding. It is not none.

**Compiled, `p1`.** A const-parameter capacity cannot produce a capacity. The operation is concatenation,
the simplest capacity-producing operation there is, and rustc states the confinement in its own words:

```
error: generic parameters may not be used in const operations
7 |     type Out = Dim<{ A + B }>;
  = help: const parameters may only be used as standalone arguments here, i.e. `A`
  = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

Under `min_generic_const_args` the same line asks for `generic_const_args`, which is not in the allowed
table and which the workspace rule records as needing `-Znext-solver=globally` plus a roughly 314-site
rewrite. **"Standalone arguments here" is the entire cost of the const column**, and it is one sentence
rather than a list.

**Compiled, `p2`.** The inductive numeral performs the same operation gate-free: `Inc`, `Add` and `AddC`
over the sealed grammar, twenty-one disjoint impls, no specialization and no overlap, with every sum in
1..=7 by 1..=7 asserted against the arithmetic on the values, with and without carry. **Ninety-eight
compile-time assertions, all holding.** `Sum<N5, N7>` is a type downstream code names.

**The named consumers, from the canon rather than from imagination.** Two are ratified and live:

- **The bivector extent.** `102:904-913` records D10's flagged const-expression hazard dissolving because
  "the bivector-extent count is derivable through the identical structural-recursion family
  (`Dec`/`PosPred`, `VAL`, `Cmp`, `Gcd`) the tower already uses elsewhere, one impl-selection step per
  axis, zero arithmetic expressions in type position, checked at ranks 2 through 8". That is a capacity
  derived from a rank by type-level arithmetic, and it is exactly what `p1` refuses under a const
  parameter.
- **Shape composition.** `102:799-805` makes a shape "a multi-dimensional index domain" whose rank-1 case
  is `Capacity`, with "per-axis extents, element count, index and stride arithmetic" as its content. Any
  concatenation, split, reshape or broadcast on that domain produces an extent from other extents.

**And what merely needs the value, which is most of it.** Index-bound membership, iteration terminators,
the element `COUNT` product, arity. All four are file 79's own list (`79:128-134`), all four are value
position, and the const column serves all four without difficulty. `106_probes/p3` already compiled the
rank-3 `COUNT` under a const-parameter capacity.

So the numeral column buys one capability, precisely: **operations whose argument is a capacity and whose
result is a capacity.** That capability has at least two ratified consumers. It is not speculative and it
is not free to give up.

---

## 3. What the const column actually costs, and a correction to how it has been priced

`106:555-568`'s table lists, under the const column, "ordering, `Cmp`, `Gcd`, value-uniqueness: available
as const comparisons, not inherited". That is right and it undersells the column, in a direction that
matters for a fair fork.

**Compiled, `p3` value half, exit 0.** A numeral's value is readable as an **ordinary associated const
with a generic parameter inside the expression**: `const VAL: usize = 2 * P::VAL + 1`. That is stable
Rust, no `type const`, no gate, no const block. A `const fn count<C: Pos>() -> usize` returning `C::VAL`
compiles and const-evaluates. The same is true of any comparison, gcd or ordering written over values.

**Compiled, `p3` type half, exit 1.** The identical const, the identical impls, the identical grammar,
moved into array-length position:

```
error: generic parameters may not be used in const operations
62 |     type Array<T: Copy> = [T; <C as Pos>::VAL];
```

**Searched, 2026-08-05 05:33 UTC**: `grep -rn "ordinary associated const\|only in type position" *.md`
returns nothing across the panel. The three hits for "value position" are the pricing pillar's own phrase
about where a `const fn` is called from (`91:119`, `100:624`, `106:372`), a different subject.

**So the answer to the dispatch's second question is that comparison, gcd and ordering are not the cost.**
They are reachable from either column, at value position, gate-free, and reaching them reintroduces
nothing. The cost is exactly one thing: **an operation whose result is a type.** Under a const parameter
that operation cannot be written at all, so the question of whether reaching it reintroduces the pairing
does not arise in the form the dispatch poses. It arises in a sharper form, and `p9` answers it.

**Compiled, `p9`.** The ratified `Slot<P, const K: usize>` fed a capacity nobody declared:

- **Concrete: works.** `Slot<Sum<N5,N7>, 12>` compiles, and the `12` is a number a human did arithmetic to
  produce. The agreement check is the only thing between that line and a wrong answer that compiles.
  Every test of the paired form in this panel supplies a capacity a human declared, where the literal sits
  next to a numeral the same author wrote. This is the first time it has been fed one where the two came
  from different places.
- **Generic: cannot be spelled.** The only honest thing to write in the literal's position is the value
  the numeral already knows, and that refuses:

```
error: generic parameters may not be used in const operations
158 | pub fn concat_storage<A, B, T>() -> <Slot<Sum<A, B>, { <Sum<A, B> as Pos>::VAL }> as Capacity>::Array<T>
    | cannot perform const operation using `A`
```

The nearest legal spelling takes `const K: usize` as a function parameter and trusts the caller. It
compiles, and it is the finding: **under the paired form, a generic capacity-producing operation has no
storage, and the only way to give it one is to ask its caller for a number.**

`102:911` already requires exactly this and did not notice: "The derived extent then pairs with its
literal exactly as the capacity resolution requires, and a wrong literal, including D10's own original
miscount, is refused with the same `E0080`." The sentence is about a rank-`n` bivector extent nobody wants
to compute by hand, and it is a fair description of the situation the paired form leaves. D10's original
miscount is cited there as the thing the check catches. It is also evidence about who is doing the
arithmetic.

---

## 4. The third column, which is a reproduction, and what it adds

**Compiled, `p4`, exit 0, zero feature gates.** `O<P>`'s storage is two of `P`'s storage, `I<P>`'s is two
of `P`'s and one element, `H`'s is `[T; 1]`, `Z`'s is `[T; 0]`. Every array length in the file is a
literal. `repr(C)` on two homogeneous combinators makes the nest layout-identical to a flat array, because
a Rust type's size is always a multiple of its alignment, so a two-field `repr(C)` struct of equal-typed
fields has no padding, and by induction over three constructors the storage is exactly `VAL` contiguous
`T`.

**This is `76_probes/b1`, reproduced on the current pin, and I say so in the same register file 106 used
about its own probe 2.** The combinator names differ by a synonym. `b1` has the seal; mine does not.

What `p4` adds that `b1` does not have:

- **The law under a capacity produced by type-level arithmetic.** `Pz<Sum<N5,N7>>` has `VAL == 12` and
  `size_of::<Array<u32>> == 48`; `Pz<Sum<N47,N47>>` has `VAL == 94` and `size_of::<Array<Odd9>> == 846`.
  Nobody declared a 12 or a 94 anywhere in the file. `b1`'s sweep is over eight declared capacities.
- **The generic signature.** `concat_storage<A, B, T>() -> <Pz<Sum<A,B>> as Capacity>::Array<T>` compiles.
  `p9` claim B is the same signature under the paired form and cannot be written.
- A wider sweep: 23 numerals at 4 element types including a nine-byte struct at align 1 whose size is not
  a power of two, **184 layout and alignment assertions**, plus 23 value assertions so a mis-spelled
  numeral fails at the spelling rather than passing a law that is true of whatever number it happens to
  be. That guard earned itself: my first `N47` was `I<I<I<O<I<H>>>>>`, which is 55, and the value
  assertion caught it.

**Compiled, `p5`.** Executed at capacities 1, 7, 13 and 47: every slot written through the mutable
projection, read back through the shared one, with the raw byte length checked so trailing padding would
show. The negative control changes one line, `I<P>`'s storage from the odd combinator to the even one:

```
error[E0080]: evaluation panicked: storage law violated: array grammar does not match the numeral
   | evaluation of `as_slice::<I<I<H>>, u32>::{constant#0}` failed here
```

It fires **through the generic projection**, which names no numeral, no length and no law, at a
monomorphisation the caller chose. Same shape as `b1c`.

### 4.1 The direction question, which is genuinely new and which refutes the checkpoint's own instinct

`106b:101-107` asks whether the numeral can be derived from the const, so that a const-parameter capacity
keeps one name and gains a numeral view where the tower's arithmetic wants it. **Searched, 2026-08-05
05:33 UTC**: `grep -rn "derive.*numeral from\|numeral.*derived from.*const\|project.*numeral out" *.md`
returns nothing across the panel. The instinct is genuinely unposed.

**Compiled, `p6`, and it does not work.** A generic projection from `const N: usize` to a binary numeral
needs recursion on `N / 2` in type position:

```
error: generic parameters may not be used in const operations
92 |         Dim<{ N / 2 }>: Project,
   = help: const parameters may only be used as standalone arguments here, i.e. `N`
```

**Per instance, by emission, it works** (`p6` claim D): a macro handed the digits emits both spellings from
one invocation with the agreement asserted inside the expansion. That is a real mechanism and the design
already has a notation vehicle with a binding-time discipline for exactly this (`102` section 1.18), and
it is what file 76's own "host-staged" introduction route already describes for the paired form.

But it changes what the answer is. **A derivation runs numeral-to-storage in the type system, and
const-to-numeral only in a build step.** So the composition reading, taken const-primary, is a build-layer
contract, not a type-system one, and a build-layer contract cannot serve a generic operation whose
arguments are type parameters. The checkpoint's instinct was worth posing and the compile kills it in one
direction and confirms the other.

---

## 5. What it costs, measured, including one number that is a design instruction rather than a cost

The checkpoint says the derivation "costs monomorphisation, and compile time is a bucket to pour into". I
measured it, because the pricing pillar licenses a cost and does not excuse declining to measure one.

**Compile time is flat in the capacity, all three shapes**, best of five, seconds, at one instantiation:

| N | const | derived | paired | numeral depth |
|---|---|---|---|---|
| 47 | 0.042 | 0.045 | 0.044 | 6 |
| 4096 | 0.043 | 0.046 | 0.044 | 13 |
| 1048576 | 0.042 | 0.045 | 0.045 | 21 |

All three sit at rustc's process-start floor and nothing grows with N, which is the same shape `102:852`
found for rank.

**On the monomorphisation axis a real cost appeared, and it turned out to be the wrong thing.** At K
distinct capacities the derived column went 0.28, 1.22, 5.31 seconds at K = 25, 50, 100 against the const
column's 0.15, 0.28, 0.73. Quadratic-looking, and it would have been the honest answer to report.

Isolating it at K = 100 between a body that only names the type and one that constructs:

| regime | type only | with a recursive `filled` |
|---|---|---|
| sum of capacities 10,200 | 0.17 | 0.83 |
| sum of capacities 34,950 | 0.08 | 3.24 |

**The type machinery is free. The cost was one operation per element, emitted because I had written
`filled` as structural recursion and `-O` inlined it.** Writing `filled` and `slice` once as provided
trait methods over the projected slice, no recursion in any body:

| shape | K=100, sum 34,950 | K=400, sum 560,400 |
|---|---|---|
| derived, provided methods | 0.12 | 0.39 |
| const parameter, identical bodies | 0.09 | 0.22 |

**3.24 s to 0.12 s, a 27x collapse, for identical semantics.** The residual is 1.3x to 1.8x, in tenths of
a second at four hundred distinct capacities, which under the pricing pillar is nothing.

**So the measurement yields a design sentence rather than a price: recur the type, never recur the code.**
The storage's shape is a structural recursion; every function over it is written once, over the projected
slice. `76`'s b1 already wrote `as_slice` that way and did not generalise it to the constructor, and I did
not either until the number said so.

**Codegen, `p8`.** Four operations at capacity 13, each written twice, against the derived storage and
against `[u32; 13]`, in one binary. LLVM merged three of the four pairs into a single symbol:

```
_native_sum  = _derived_sum
_native_fill = _derived_fill
_native_copy = _derived_copy
```

`derived_sum` vectorises to NEON (`ldp q1, q0`, `add.4s`, `addv.4s`), and `native_sum` **is** that symbol.
The fourth pair did not merge and is instruction-for-instruction identical (`cmp x1, #12` / `b.hi` /
`ldr w0, [x0, x1, lsl #2]` / `ret`), differing only in which `Location` constant is passed to
`panic_bounds_check`, whose payload differs at one byte recording the source column. A source-location
record, not code.

That is symbol identity, the same class of evidence file 103 used for the truth-contract fork and which
`106b:111` accepted as stronger than a bench.

---

## 6. File 76's four reasons, re-examined against three facts that arrived after them

This is the part I care most about getting right, because file 76 made a considered recommendation with
both constructions in front of it and I am not in a position to overturn that by having compiled it again.
Its four reasons, at `76:220-226`, each taken at its own text.

**"It adds no `unsafe` to the bottom of the design."** True and real. Two `unsafe` blocks, in the two
projections. What has changed is what they are being compared against. File 76 wrote, of `b1c`
(`76:196-199`): "a capacity a downstream crate invents cannot reach the cast without the check running."
Three files later file 100 found that construction two's check **is** reachable-around: at rank 3 the
recursion is written against the trait method and the check lives on the inherent one, so a shape whose
middle axis declares a `Nat` of 4 against a literal of 7 has two disagreeing const-evaluable counts and
nothing raises (`102:820-830`). The repair adopted for it is "adopted as a working shape, second read
owed", in two halves, both still owed on `106`'s own list. So the comparison is not safe against unsafe.
It is **one `unsafe` whose obligation sits on the only road, against a safe check demonstrated to be
reachable-around whose repair is unbuilt.** File 76 could not have known this. The ninth consolidation
could have.

**Searched, 2026-08-05 05:33 UTC**: `grep -rn "b1c\|perimeter control" *.md` returns file 76 and the
eighth consolidation's grounding line only. Nobody has read `b1c` against file 100's finding.

**"It needs no layout argument."** True, and the argument is three sentences about `repr(C)` that are
const-asserted at every instantiation and every element type rather than argued. `p4` asserts it 184
times; `p5`'s negative control shows the assertion discriminates.

**"It puts `Capacity`'s two halves on the layers the design's own rule assigns them."** This is the
strongest of the four and I want to push on it rather than wave at it. File 76's reading is that the array
grammar "depends on nothing but the carrier", so keying it on the value is the layer-keying rule's own
named failure. I read that as true of `[T; K]`'s **spelling** and not of the storage **fact**: the storage
a capacity determines is a function of the count and the element type and nothing else, which makes the
capacity the coarsest layer whose identity it depends on. And `102:813-815` has since resolved the shape
case the same way, "a separate trait projects a shape to storage, once per element domain", calling it the
`Lowering` charter one dimension up. `Capacity::Array` is that projection at rank 1. **I offer this as a
reading and not as a refutation**; it is the one of the four where two competent readers could differ
honestly, and it is where a second read should be pointed.

**"It leaves `[T; K]` as the plain array every consumer and every debugger already understands."** True,
and it is the one cost that does not shrink under scrutiny. `TwinOne<Twin<TwinOne<[u32;1],u32>>,u32>` in a
debugger is worse than `[u32; 13]`, and a consumer or FFI surface wanting `[T; N]` by name cannot have it.
Against it, and this is why the shipped tests are worth reading rather than the shipped design:
`arvo-tensor/tests/capacity.rs:31-59` and `:67-96` reach every element through `as_ref`/`as_mut` slices
and never name an array type, because generic code cannot. Read as a factual check on how generic
consumers reach elements today, not for what it means. The loss is real at concrete instantiations and
absent at generic ones.

### 6.1 The perimeter rule fires here, and it is the first type in this stretch where it does

File 103 cited the perimeter rule on `Bool` and file 106 struck the citation, correctly, because `Bool`
has no invariant (`106` section 3.2, compiled exhaustively at `106_probes/p5`).

The derived storage does. Its correctness argument is "values of this type are exactly `VAL` contiguous
`T`", which is a property of every value of it, which is the rule's own antecedent. So the combinators'
fields are private and the two projections are the only doors, and then the const block is not a belt but
the perimeter: there is no route to an element that does not resolve it. **That is the rule firing on its
own terms for the first time in this stretch**, and it is worth noting that the type it fires on is one
the design does not yet have.

---

## 7. The three columns, and what I would say the design is

Priced across the axes that decide it. Every cell is compiled or measured in `107_probes/` except where
marked.

| | const parameter | numeral + literal (ratified) | numeral + derived storage |
|---|---|---|---|
| names for the number | one | **two, per declared capacity** | one |
| agreement fact | none | exists, needs a route | none |
| `AGREES`, two-half repair, reachability scope | not needed | needed, **repair unbuilt** | not needed |
| value arithmetic (`Cmp`, `Gcd`, ordering) | const fns, value position | inherited from the tower | inherited from the tower |
| **capacity-producing operations** | **refused** (`p1`) | type works, **storage cannot be spelled generically** (`p9`) | **works** (`p4`) |
| storage for a computed capacity | n/a | a literal a human computes | derived |
| falsifiable surface | none | **one per declared capacity, unbounded** | **three lines, fixed** |
| feature gates | none | none | none |
| compile time, K=400 | 0.22 s | not measured, between the two | 0.39 s |
| codegen | native | native | **native, same symbol** (`p8`) |
| `[T; N]` by name | yes | yes | no, slices only |
| `unsafe` | none | none | two projections, discharged per monomorphisation |

**What I would put in the next consolidation, as a correction and a lean rather than a ruling.**

**On the capacity resolution, replacing one sentence.** *The array grammar's pairing is forced neither by
the language nor by the choice of an inductive numeral alone. It is forced by obtaining the storage as
`[T; K]`, by its length. A storage derived from the numeral's own structure needs no literal, and was
compiled with two controls at `76_probes/b1`, recommended against at `76:220-226` on four stated grounds,
carried by the eighth consolidation as open item 12, and dropped by the ninth without a droplist entry.*

**On what a capacity is.** *A capacity is a numeral in a role. It is a direct instance of the tower's
`Nat`, one seal, one ordering, one arithmetic, inherited wholesale, and what it adds is not a second
number but a projection to storage, keyed on the capacity because the storage is a function of the count
and the element type and nothing else. That is the `Lowering` charter at rank 1, the same statement
`102:813-815` makes for shape one dimension up.*

**On the constructor, from the measurement.** *Where a type's shape is a structural recursion, every
function over it is written once against the projected view, never recurred alongside it. Measured at 27x
compile time for identical semantics at a hundred distinct instantiations.*

**My lean, and it is a lean.** The third column, under three conditions: the projections are provided
methods written once; the combinators' fields are private so the projections are the only doors; and the
loss of `[T; N]` by name is accepted, or bought back by an emitted per-instance escape for the concrete
capacities that need one. What moves me is not elegance. It is that the second column **cannot give
storage to a capacity the type system computed**, and the design has at least two ratified consumers that
compute one.

What would move me back: a consumer that genuinely needs `[T; N]` by name at a generic capacity, or a
reading of the layer-keying rule under which the storage projection is keyed wrongly. I have argued
against the second in section 6 and I would not call it settled.

**This is one read and it is op's call.** File 79's answer was one read too, and section 2 is about what
happened when it was carried as though it were two.

---

## 8. What this file does not decide

**Which column.** Three are on the record with what each buys and what each costs, and the third one's
record was already lost once. Whether a capacity belongs in the numeral tower's vocabulary is op's, and it
now has one more fact under it than it had at `77b`.

**Whether file 76's layer-keying reason survives section 6's reading.** One pass, mine, and the one place
in this file where I think two honest readers differ.

**Whether the eighth consolidation's open item 12 was dropped by decision or by loss.** I found no
disposition anywhere. If it was decided, the decision is not written down; if it was lost, the discipline
line file 106 proposes at its section 4.3 would have caught it, and this is the third instance and the
first addressed to op.

Owed artifacts, each with what closes it:

- **The second read on section 7's statement of what a capacity is.** One pass, mine. Point it at the
  layer-keying reading in section 6, which is where I am least sure.
- **The `[T; N]`-by-name consumer question.** Whether any settled mechanism needs a concrete array type
  rather than a slice at a generic capacity. `mock/crates` is op's boundary; the canon-side check is one
  pass over the shape and container chapters.
- **The `AGREES` repair's second half**, still owed on `106`'s list, and moot on the first and third
  columns. `106` said the fork is worth an hour for exactly this reason and it still is.
- **The toolchain trap as a probe convention.** Third file, opposite directions twice. One line.
- **The two bitfield overlap tests** from file 104, unchanged, still owed, `mock/crates`, op's boundary.

---

## 9. The three requirements, performed on this text before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions.
*Capacity-producing operation* (sections 0, 2): defined at first use as an operation whose argument is a
capacity and whose result is a capacity, distinguished from one that merely reads a capacity's value,
which is the distinction the whole file turns on and is therefore defined where it first carries weight.
*Derived storage* (sections 0, 4): defined at first use as a storage type obtained by recursion on the
numeral's own grammar rather than from its value, contrasted with *paired storage*, obtained from a
companion literal. *Falsifiable surface* (sections 6, 7): defined at first use in section 7's table as the
set of places a wrong statement can be written, and its cardinality is what the row reports.
*Projection*, as used in "a capacity contains a projection rather than a number" (section 0): defined
there as an associated type carrying no independent value. Terms used from the record without
redefinition: the pricing pillar, the perimeter rule, the layer-keying rule and its `Lowering` charter,
the carrier-at-birth rule, the separation requirement, the definitional-completeness line, the toolbox
rule, `Capacity`, `Nat`, `Pos`, `AGREES`, construction one and construction two (file 76's own names,
quoted). *Numeral in a role* (section 7) is stated as a reading rather than defined as a term, and section
0's table is what gives it content. No term in this file's own new prose is left undefined or uncited.

**The separation requirement, performed.** Two models here are mine and the requirement bites on both.

The first is section 0's four-reading table, separating *shapes with a second number that can disagree*
from *shapes without one*. **Nonvacuous at exactly the pair `p9` and `p4` compile**: at a capacity the
type system produced, (d) cannot be spelled generically and (c) can, so the two columns separate on a
fact rather than on taste. **Where it is vacuous I say so**: at a capacity a human declared, all four
readings deliver the same storage, the same size, and the same codegen, which is why twenty-three files
carried the ratified answer without strain and why `76`'s own eight-capacity sweep could not have
separated them either.

The second is section 5's split between *the cost of a type's shape* and *the cost of a function written
over it*. **Nonvacuous at exactly the type-only against with-`filled` pair**: 0.08 s against 3.24 s at one
K, on one storage, changing nothing but whether the constructor recurs. **Where it is vacuous I say so**:
at a single instantiation the two are indistinguishable at rustc's process-start floor for every N up to a
million, which is the regime every prior measurement in this panel has been taken in, and is why the cost
had not been seen.

**The freshly-performed-search requirement, performed.** Every universally quantified negative above
carries its own search, run this session, quoted with its date. Two of my own draft negatives failed and
are recorded as run rather than as concluded.

- "Construction one is in the eighth consolidation and in neither the ninth nor the tenth, with no
  droplist entry": `grep -rln "construction one\|b1_structural\|derive the storage structurally" *.md`,
  2026-08-05 05:29 UTC, returns `76`, `78`, `96` only; `grep -c 'construction one\|recorded fallback'`
  over the ninth and tenth returns 0 and 0 at the same time; `91`'s droplist read in full at `:1043`
  onward rather than grepped, because a claim about absence from a list is a claim about the list.
- "No panel file states that the numeral's value is readable in value position and refused only in type
  position": `grep -rn "ordinary associated const\|only in type position" *.md`, 2026-08-05 05:33 UTC,
  empty. The eleven files matching "value position" were each checked and all are the pricing pillar's
  own phrase about where a `const fn` is called from.
- "Nobody has posed deriving the numeral from the const": `grep -rn "derive.*numeral from\|numeral.*derived
  from.*const\|project.*numeral out" *.md`, 2026-08-05 05:33 UTC, empty. `106b:101-107` offers it as
  unposed and is right.
- "Nobody has read `b1c` against file 100's finding": `grep -rn "b1c\|perimeter control" *.md`,
  2026-08-05 05:33 UTC, two files, `76` and `78`'s grounding line.
- **My draft claimed the derivation was new to this panel. It is false.** `76_probes/b1_structural_array.rs`
  is it, and `76_probes/OUTCOMES.md:44` records it as WORKS. Found by `grep -rln "structural.*storage\|
  nested pair\|recur the storage" *.md`, 2026-08-05 05:28 UTC, one hit, which I then read at source.
  Sections 1 and 4 were rewritten from "new construction" to "reproduction plus three additions", which
  is what they are.
- **My draft claimed the derivation costs a quadratic in the number of capacities.** It is false, and the
  first measurement said so before the prose did. Section 5 was rewritten from a cost to a design
  instruction after the provided-method version measured 27x cheaper for identical semantics.

The honest limit, inherited from seven files running and now eight: these performances verify that this
file's terms are placed, its models have content, and its negatives were searched with my vocabulary.
Section 1 is this stretch's fourth demonstration that a grep's **vocabulary** is what fails, and I found
`b1` only because one search of five used a word file 76 happened to use. A second reader with different
terms is the check on mine, and on this subject the record says that check has been owed twice.

---

## 10. Standing

A capacity is a numeral. What it contains is a projection, not a second number, and the projection is
derivable from the numeral's own structure with no literal, no agreement fact, no construction door and no
feature gate, at a capacity the type system computed as well as one a human declared. Three of file 79's
four spec sentences survive that whole, including its sharpest one, that a capacity is a parameter and not
an event.

The fourth does not. "Forced by the language, not chosen" is false twice: file 106 found that it is
conditional on the choice of an inductive numeral, and this file finds it is also conditional on obtaining
the storage by its length. Both escapes were named in one file, `76`, which compiled both, recommended one
on four stated grounds, and filed the other as an open question for op.

**And the thing I would put in front of op first is not the design question.** An item explicitly
addressed to him, carried by the eighth consolidation as open item 12, naming the exact construction that
dissolves the fork he reopened at `77b`, is absent from the ninth and the tenth with no droplist entry,
while item 13 from the same list, a tautological test to delete, is still being reported twenty-eight
files later. The consolidation kept the trivial item and lost the load-bearing one, and the sentence that
replaced it asserts the opposite of what the lost construction proves.

That is the third instance this stretch of the failure file 106 named, on the third body of material, and
the first where the lost item was a question someone had put to the lead designer. **The discipline line
file 106 proposes is not a nicety about compression. On this record it is the thing standing between the
review and a design decision made from a document that had quietly stopped containing the alternative.**

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (`77b:68-100` op's own reframing verbatim, `91:780-805` the capacity resolution
including the one sentence this file corrects, `102:90-95` the pricing pillar, `102:799-830` and
`:904-913` the shape and bivector-extent consumers, `102:813-815` the `Lowering` charter one dimension
up, `unstable-features.md`'s forbidden and allowed tables, the persona-tier `95b`/`101b`/`106b` as
marked), settled shapes (`76` sections 2 and 3 in full and `76_probes/OUTCOMES.md:44` which is section
1's whole subject, `78:750-785` and `:866-878`, `79` sections 2 through 6 read at their own text, `100`
section 2.2, `103` section 1.3, `106` sections 0, 4 and 6, `106b` sections 2 and 8), compiled
(`107_probes/p1` through `p9`, all at the pin from inside the tree, commands and verbatim diagnostics in
`107_probes/OUTCOMES.md`; `76_probes/b1_structural_array.rs` re-read at source rather than at its prose
summary), measured (`107_probes/p7_compile_cost.py` and the two follow-up sweeps recorded in OUTCOMES,
`p8`'s emitted assembly), verified at source (`arvo-tensor/tests/capacity.rs` and
`tests/const_capacity.rs:1-70` in full, `arvo-tensor/src/capacity.rs:44-56`, `arvo-tensor/src/lib.rs:21`,
HEAD `be66678`), overturned (two of this file's own draft claims, section 9), reasoned (the four-reading
table in section 0, the archival trace in section 1, the re-examination of file 76's four reasons in
section 6, the perimeter reading in 6.1, and the lean in section 7, all mine, all one pass, offered as
suggestion and not as a ruling).*
