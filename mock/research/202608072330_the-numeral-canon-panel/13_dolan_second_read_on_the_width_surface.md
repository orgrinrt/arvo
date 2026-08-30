# 13. A second read on the width surface, derived before reading the first

**Date:** 2026-08-08. **Status:** breadth pass, nothing settles. **Register:** appears to hold /
compiles, here is the command / closed, here is the diagnostic.

## The order I actually worked in

This dispatch exists because `RULES.md` records that cumulative sequential reading makes the
`TWO EXPERTS` rung unreachable by construction. So the order is stated first, and it is checkable
against the probe timestamps in `13_probes/`.

1. `RULES.md`, `01_op_answers.md`, `04_op_no_settlements_tonight.md`, `SETTLED.md`.
2. `seed/SETTLED_surface.md:135-159`, the establishing text for the ergonomics bar, read directly
   rather than through `SETTLED.md`'s compression of it.
3. Toolchain check, then **derivation and probes `p01` through `p27`, with `12`, `11` and `10`
   unread**.
4. Everything in this file from "Reading the bar" down to "What I did not cover" written to disk, in
   that state, still with `12`, `11` and `10` unread. That is the independent derivation and it is
   what the `TWO EXPERTS` rung is about.
5. Then `12` in full, then `11` sections 7 and 8, then `10` sections 7 and 8.
6. Then `p28` and `p29`, and the three sections after "What I did not cover".

**Two sections sit above the comparison but were written after step 5**, and I say so rather than let
the layout imply otherwise: "Extending the bridge does not require spelling a digit tower" and "Every
probe, and one command that runs them". Neither depends on anything in `12`; the first came out of
trying to break my own arrangement D and the second is bookkeeping. They sit where they sit because
they belong with the derivation, not because they preceded it.

Everything else above "What `12` says and where I stand" was on disk before I opened `12`, and the
probe files it cites (`p01` through `p27`) all predate that read.

## Reading the bar from the establishing text, not from the compression

`seed/SETTLED_surface.md:135-159` is op quoted at `142c:322-386`:

> To write the domain aliases, it has to be easy and intuitive to write the types they alias.
> UInt<5> is great and easy, intuitive. Ufixed as in the example too. But if there were precision in
> there, container types, it'd be fucking ass for anyone who doesn't know nor care about all that
> plumbing, to write. So the ergonomics of UFixed, FastFloat, all those, are crucial and perhaps more
> important than the plumbing itself.

Three readings that matter and that the compression in `SETTLED.md:109` loses or blurs.

**The bar binds at one site.** The sweep's own wording is "disqualified **at that site specifically**",
the site being a domain-alias definition. A type-level magnitude is disqualified *there*. It is not
disqualified in the machinery, not at operation outputs, and not in an internal projection the
consumer never spells. This is the qualifier `SETTLED.md` dropped, and it changes the question from
"can the design avoid type-level nats" to "can the design avoid the consumer *writing* one".

**Two numbers are at the bar, not one.** `UFixed<5, 0>` is named by op as at the bar. So the surface
may carry two consts. It may not carry a third; the sweep's own disqualifier list ends with "extra
arity in general".

**What op himself disqualifies is short.** His words name two things: precision in the spelling, and
container types. The sweep's four further demands and two further disqualifiers are the panel's
paragraph over him. I take his two as binding and the sweep's additions as strong guidance, which
matters below exactly once: at whether a *macro* is disqualified. It is not in op's sentence. I do
not reach for it anyway, for a different reason given in the macro section.

## The question, restated so it can be answered

The consumer writes a number. Rust parses a bare integer literal in generic-argument position as a
**const** argument, so the surface hands the machinery a `const`. The machinery wants to branch: pick
a container, add widths on multiply, compare against a bound. Branching on a const in **type
position** is what `generic_const_exprs` exists for, and it is forbidden.

So the shape of the whole problem is one sentence, and I will justify it by probe rather than assert
it:

> Any function of the consumer's literal, evaluated in type position, needs either
> `generic_const_exprs` or one impl per value.

If that is right, the bridge is forced rather than chosen, and the interesting question is not "can
the bridge be removed" but "what is the bridge's domain, and does the operation algebra enlarge it".

## The derivation, before any predecessor was read

### Step 1: which direction is free

Type to const is free and needs nothing. A type-level numeral can carry `const VALUE`, and an impl
body may compute anything into an associated const. Const to type is the refused direction.

An associated **type** projection over a const parameter is also free: `<M<N> as Tr>::Out` where `M`
is `struct M<const N: u32>`. No feature. The refusal is specifically about a const *expression* in a
const-argument slot.

So the wall is narrow: it is not "consts and types do not mix", it is "an expression over a const
parameter may not appear where a const argument goes".

### Step 2: the codomain of the container map is finite, and that does not help

Container choice has a small closed codomain: the native primitives and one byte-sequence family. A
map from an unbounded domain onto seven shapes needs a const test to pick the shape, and the test is
the refused direction again. Finiteness of the codomain buys nothing while the domain is a const.

### Step 3: what the type-level ladder actually buys

Type-level arithmetic is free and total. `Add<I1, I2>` as an associated type needs no feature and no
table, and it is closed: no operation on nats can produce something outside the nats. That is the
entire reason a ladder is worth having, and it is why the ladder is not the problem.

The consequence I want to test, because it cuts against how the ceiling is usually stated: if
everything internal is nat-shaped, an operation's **output never re-enters the bridge**. The bridge is
entered when a consumer writes a literal, and at no other time.

### Step 4: therefore the candidate answer

Keep the const at the surface, keep nats in the machinery, and make the bridge's domain be *the set of
literals appearing in consumer source* rather than *the set of widths reachable by the algebra*. Then
the question "does a finite table catch up with doubling" is the wrong question, because doubling
happens on nats and never asks the table anything.

Whether that works turns on one type-identity: does the nat produced by type-level addition normalise
to the same type as the nat the bridge produces for the sum's literal. If yes, a consumer can name a
multiply's result as `UFixed<26, 6>` and it is the same type. If no, there are two nats for one width
and the design has a canonicity problem that no table size fixes.

That is a compile-or-refuse question, so I compiled it.

## The wall, located exactly, in four compiled refusals

Every route below was compiled on the pin. Command shape throughout:
`rustc +nightly-2026-05-28 --edition 2021 --crate-type lib <file>`.

**The wall itself, no features** (`13_probes/p01_the_wall.rs`). `Store<{ (N + 7) / 8 }>` where `N`
is a const parameter:

```
error: generic parameters may not be used in const operations
   = help: const parameters may only be used as standalone arguments here, i.e. `N`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

The same file compiles `struct PassThrough<const B: usize>([u8; B])` without complaint. So the
refusal is narrow and worth stating precisely: **a bare const parameter is fine anywhere; a function
of one is refused in const-argument position.**

**The `min_generic_const_args` escape, closed in three steps**, each step following rustc's own
suggestion from the previous one. `p02`, an associated-const path as a const argument:

```
error: use of `const` in the type system not defined as `type const`
help: add `type` before `const` for `Bytes::B`
```

`p03`, taking that suggestion:

```
error: complex const arguments must be placed inside of a `const` block
```

`p04`, taking that one:

```
error: generic parameters may not be used in const operations
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

The chain terminates in `generic_const_args`, which is forbidden and which additionally needs
`-Znext-solver=globally`, also forbidden. So mGCA does not move the arithmetic into an impl body. It
moves the refusal there.

**Const-parameter defaults, closed** (`p05`). `struct Numeral<const BITS: usize, const BYTES: usize =
{ (BITS + 7) / 8 }>` would have let the consumer write one number and the machinery receive the
derived one with no table anywhere. Same refusal, same text.

**The reverse direction, closed** (`p24`). A structural nat carries its value as an ordinary
associated const for free, and putting that const back into a const-argument slot fails identically:

```
error: generic parameters may not be used in const operations
   = note: type parameters may not be used in const expressions
```

That fourth one is the load-bearing one, and it is the finding I did not expect. **The bridge is one
wall with two directions, and both are refused by the same rule.** Whether a design pays a table
depends entirely on how many times, and in which directions, it crosses.

So the claim from my step 1 holds as stated, and is now compiled rather than asserted.

## What is free, and it is more than I expected

With no feature gates at all, the following are ordinary Rust and were compiled in `p06` through
`p11`.

**Type-level addition normalises.** A binary nat, LSB first, canonical, with add and add-with-carry
in nine plus nine impls. The identity that matters is whether the sum of two width types is *the same
type* as the width type for the sum. It is, over the **whole** matrix rather than a sample:
`gen_p07_matrix.py 64` emits 4225 pairs covering every `a, b` in `0..=64`, and
`rustc ... p07_matrix.rs` exits 0. The negative control `p08` asserts one false identity and is
refused, so the matrix is not vacuous:

```
error[E0277]: the trait bound `E<O<E<O<O<Z>>>>>: Same<E<E<E<E<E<O<Z>>>>>>>` is not satisfied
```

**Ceiling division to bytes is structural.** Halving a binary nat with a ceiling is three impls, and
`ceil(n/8)` is that composed three times. `p10` checks it against every width `0..=130`, whole matrix.

**An exact byte buffer needs no const at all.** Build it from the nat's binary structure:
`Buf<Z> = Nil`, `Buf<O<n>> = (u8, Buf<n>, Buf<n>)`, `Buf<E<n>> = (Buf<n>, Buf<n>)`, each `#[repr(C)]`.
Size is exactly `n` bytes and alignment is 1, at log depth. `p11` confirms it at runtime for every
count `0..=128`: `buffer sizes checked 0..=128, mismatches = 0`.

**Container selection across the rungs is finite.** Type-level comparison is nine impls, the
three-valued ordering feeds two selectors of three impls each, and the five native rungs plus the
wide rung compose into one expression written once. `p10` checks the selected container against every
width `0..=130`, whole matrix, and it is right at every rung boundary including `128` and `129`.

The consequence is worth stating plainly, because it is the opposite of how the container derivation
is usually described: **container derivation is not the hard part.** It is a fixed number of impls,
it needs no feature, and it is total over all widths. The hard part is exclusively the crossing.

## The bound avalanche is real, and it collapses to one name

The obvious objection to a type-level design is that projections in field types drag their bounds
everywhere. That happened, on the first attempt (`p12`):

```
error[E0277]: the trait bound `I: Add<F>` is not satisfied
```

The repair is to name the entire derivation as one trait. `p13_surface_bounded.rs` puts eleven
where-clauses on a single `Shape` impl, and everything downstream carries the single bound
`Pair<I, F>: Shape`. That compiles, exit 0, zero feature gates
(`grep -c '#!\[feature' p13_surface_bounded.rs` returns 0).

The part that matters for the ergonomics bar: **none of it reaches the consumer.** A type alias is
transparent and its body is not bound-checked, so the domain-alias author writes a number and sees no
bounds at all. The avalanche is arvo's cost, paid once, in one impl.

## Arrangement A: structural width types. The algebra never asks the table anything

Consumer spelling, extracted by `grep -n 'pub type StrHandle' p14_multiply_past_the_table.rs`:

```
pub type StrHandle = UInt<5>;
pub type Money = UFixed<13, 3>;
```

Character for character, that is a name and a number, and a name and two numbers. At the bar and at
the bar, by op's own two examples.

The bridge is `impl Lit for L<K> { type N = <structural nat> }`, one row per literal, 65 rows in the
probe (`grep -c 'impl Lit for L<' p13_surface_bounded.rs` returns 65).

Then the question the ceiling is about. Multiply produces `Numeral<Sum<I1, I2>, Sum<F1, F2>>`, a
projection over nats, and **it does not consult the bridge.** `p14` multiplies four times from a
16-bit numeral, taking the total width to 256, which is four times past the table's last row:

```
pub type M2 = Prod<Money, Money>;   // 32 bits
pub type M4 = Prod<M2, M2>;         // 64
pub type M8 = Prod<M4, M4>;         // 128
pub type M16 = Prod<M8, M8>;        // 256
```

`rustc ... --crate-type lib p14_multiply_past_the_table.rs` exits 0. `p15` prints the derived
containers:

```
StrHandle  UInt<5>        size=1 align=1
Money      UFixed<13,3>   size=2 align=2
M2         w=32           size=4
M4         w=64           size=8
M8         w=128          size=16
M16        w=256          size=32
```

Every rung, including a 256-bit value whose width appears in no table row anywhere.

**So in this arrangement the ceiling does not exist.** The bridge's domain is the set of literals
written in consumer source, and the operation algebra never enlarges it. A finite table does not have
to catch up with doubling, because doubling happens on the far side of the crossing.

That is a claim I would want a third instance of before anyone leans on it, and it is stated as
*appears to hold* rather than settled.

## Arrangement A's cost, and it is severe

The width mismatch is the most common error a numeric library produces. Here is what a consumer reads
when `UFixed<12, 4>` meets `UFixed<13, 3>` (`p20`, verbatim):

```
error[E0308]: mismatched types
    |                   ------- ^ expected `Numeral<O<E<O<O<Z>>>>, O<...>, ..., ...>`,
    |                                found `Numeral<E<E<O<O<Z>>>>, E<...>, ..., ...>`
    = note: expected struct `Numeral<O<E<O<_>>>, O<O<Z>>, _, _>`
               found struct `Numeral<E<E<O<_>>>, E<E<O<Z>>>, _, _>`
```

None of `12`, `4`, `13` or `3` appears anywhere in that message. The consumer wrote four numbers and
is shown two binary digit towers with elisions. `#[diagnostic::on_unimplemented]` does not reach
E0308, and rustc offers no other hook, so this is not repairable by annotation.

The other bad case is a width with no bridge row, and it has two defects (`p16`). First the message:

```
error[E0277]: the trait bound `L<100>: Lit` is not satisfied
    = help: the following other types implement trait `Lit`:
              L<0>  L<10>  L<11>  L<12>  L<13>  L<14>  L<15>  L<16>
            and 57 others
```

The "other types" list is in lexicographic order, so the eight shown are `0, 10, 11, 12, 13, 14, 15,
16`, which tells the reader nothing about where the supported range ends. That is what an enumeration
looks like when it reaches a diagnostic.

Second, and worse, the **locality**. `p17` contains `pub type Wide = UInt<100>;` and nothing else,
and it compiles clean, exit 0. Type aliases are transparent and unchecked, so the alias-definition
site, the exact site the ergonomics bar governs, reports nothing. The error surfaces at first use,
possibly in another crate.

`#[diagnostic::on_unimplemented]` fixes the headline and adds actionable notes (`p18`):

```
error[E0277]: no numeral of width L<100> exists in this build
    = note: widths 0 through 64 are available out of the box
    = note: to use a wider numeral, name it once: `impl Lit for L<100> { type N = ...; }`
```

It does not suppress the lexicographic list, and it does not move the error to the alias.

**`lazy_type_alias` does not fix the locality, and makes things worse** (`p19`). It eagerly
bound-checks alias bodies, and every helper in a type-level design is an unbounded projection alias,
so the whole vocabulary fails at once:

```
error[E0277]: the trait bound `A: Add<B>` is not satisfied
  --> pub type Sum<A, B> = <A as Add<B>>::Out;
error[E0277]: the trait bound `A: AddC<B>` is not satisfied
error[E0277]: the trait bound `A: Cmp<B>` is not satisfied
```

Route closed. Marked as resting on an unvetted feature, and the result is negative anyway, so nothing
downstream rests on it.

## Arrangement B: the width type is the literal. Readable, and the ceiling is real

Since the diagnostic is the cost, the obvious counter-move is to make the width type be `L<K>`
itself, with the structural nat demoted to a hidden `Repr`. `p21_named_widths.rs` does that, compiles,
zero features, and the consumer spelling is unchanged:

```
pub type Money = UFixed<13, 3>;
pub type StrHandle = UInt<5>;
```

The mismatch diagnostic becomes what it should have been all along (`p22`):

```
error[E0308]: mismatched types
    |                    ------- ^ expected `13`, found `12`
    = note: expected struct `Numeral<L<13>, L<3>, _, _>`
               found struct `Numeral<L<12>, L<4>, _, _>`
```

The price is exact and it is the ceiling. An operation's output is a structural nat, and naming it
back as an `L<K>` is the reverse crossing that `p24` closed. So it needs a reverse table, and the
table must now cover **every width any operation produces**. `p23` multiplies three times and hits it:

```
error[E0277]: the trait bound `E<E<E<O<E<O<O<Z>>>>>>>: Named` is not satisfied
    = help: the following other types implement trait `Named`:
              E<E<E<E<E<E<O<Z>>>>>>>  E<E<E<E<E<O<Z>>>>>>  ...  and 24 others
note: required for `Numeral<L<52>, L<12>, Unsigned, Warm>` to implement
      `MulShape<Numeral<L<52>, L<12>, Unsigned, Warm>>`
```

That is the worst message in the whole set: a digit tower is reported as not implementing a trait,
with eight further digit towers offered as help. It is also, exactly, the ceiling the brief describes.

**So the ceiling is a property of crossing back, not a property of the bridge.** Arrangement B pays it
because it names outputs. Arrangement A does not name outputs and does not pay it. Both use the same
forward table.

## Arrangement D: declare the output width, and get both

If the ceiling comes from *computing* an output name, the third move is to stop computing it. Let the
consumer declare the output width and have the type system check the declaration is wide enough. The
check is a type-level comparison, which is free, and no reverse table exists.

`p25_declared_output.rs` compiles, exit 0, zero features. The width types are `L<K>`, so diagnostics
carry the consumer's numbers. There is no `Named` trait at all, so there is nothing to run out of.

The refusal when the declared output is too narrow, with the trait annotated (`p27`):

```
error[E0277]: the declared output width is narrower than the product needs
    | fn _too_narrow() where Money: MulInto<Money, TooNarrow> {}
    |                        ^ widen the declared output, or state the rounding explicitly
    = note: an integer part of I1+I2 and a fraction of F1+F2 is what a product occupies
```

Unannotated it reads `the trait bound Gt: IsLe is not satisfied`, which is meaningless, but the
`note:` line already carries all four numbers:

```
note: required for `Numeral<L<13>, L<3>, Unsigned, Warm>` to implement
      `MulInto<Numeral<L<13>, L<3>, ...>, Numeral<L<20>, L<6>, ...>>`
```

The cost of D is a design choice rather than a mechanism defect: `let c = a * b;` no longer infers,
and a product site states its output shape. For a fixed-point library that is arguably the honest
shape, since the output's integer and fraction split is a decision somebody has to make. It is a
cost, and it lands at call sites rather than at the alias-definition site, which is where the
ratified bar binds.

I want to be careful not to oversell D. It was built to answer one question and it answers it; the
inference consequence for tier one, whose whole premise is `T: Add` and no typestate, is **not**
worked out here and is the first thing I would attack next.

## The composition

| | consumer spelling | mismatch diagnostic | ceiling | table domain | features |
|---|---|---|---|---|---|
| A, structural widths | `UInt<5>`, `UFixed<13, 3>` | digit towers, no numbers | none | literals written | none |
| B, literal widths, computed outputs | same | `expected 13, found 12` | real, at the last row | literals plus every output | none |
| D, literal widths, declared outputs | same | `expected 13, found 12` | none | literals written | none |

All three meet the ergonomics bar identically, because all three put the same characters at the alias
site. They differ only in what the machinery receives and what the consumer reads when wrong.

## Routes I opened and closed, so nobody spends the effort again

Beyond the compiled refusals above, these were reasoned to a wall rather than built, and each is
stated so a later member can attack it rather than rediscover it.

**One container family parameterised by a byte count, with the count passed through.** Removes the
table entirely, since `[u8; B]` accepts a bare parameter. Dies because the consumer writes bits and
`ceil(bits/8)` is the refused expression, and writing bytes at the surface loses exact widths, which
is arvo's reason to exist.

**A finite codomain buying finiteness.** The container map lands in about seven shapes, so I expected
the table to shrink to seven rows. It does not: finiteness of the codomain is irrelevant while the
domain is a const, because picking the row still requires testing the const in type position.

**Structural recursion on the const, peeling one bit at a time.** Would make the bridge a blanket
impl. Needs `M<{N >> 1}>` in type position, which is the wall.

**Overlapping blanket impls made disjoint by a width condition.** There is no `where N < 8` in Rust,
and manufacturing one as a marker trait puts the table back.

**`min_specialization` on const values.** A specialising impl must be more specific in type
structure; `impl Tr for W<5>` is a table row by another name.

**`adt_const_params` carrying a struct-typed width.** The consumer's literal still has to be mapped
into the struct value, which is an expression over a parameter.

**Base-16 or base-256 digit types, to shorten the tower.** Untried, and I want to name the price
rather than leave it looking free: base 16 shortens the printed tower by a factor of four and costs
roughly 512 fixed impls for digit addition with carry, base 256 shortens it by eight and costs on the
order of 65536. Both are fixed rather than width-indexed, which is a different thing from a per-width
table, but "no enumeration if it can be helped" points away from them and I did not pursue it.

**A macro at the alias site.** Not pursued. Op's own sentence does not name macros, so I am not
leaning on the sweep's addition; I set it aside for a mechanism reason instead. A macro would compute
`ceil` at expansion and remove the table, and it would do so by moving the crossing into a stage where
the type system is not looking, which means the validation clause of the erasure gate would be
enforced by the macro rather than by the typestate. That is a different design, not a repair of this
one.

## On the first clause of the erasure gate

The brief says to read it carefully. The gate is `135b:12-16`: "There *is* a way to express usage
through bits and bytes *and* have the typestate derive the matching container and numeral
representations, then validate, and erase on lowering."

**Bits and bytes**, both. Every arrangement above takes bits at the surface and derives bytes. Nothing
in the gate says the consumer may not write a byte count directly, and `p10`'s `Bytes<W>` is already
the map between them, total and checked over every width `0..=130`. A surface that admits both units
is expressible with the machinery as built, and neither the bar nor the gate forbids it. I did not
build it, and I flag it because a later member reading "bits and bytes" as a synonym pair will not
notice that the gate names two units and the current surface offers one.

The erasure clause I did check. `p15` is compiled at `-O` and the sizes are the derived containers,
so the type-level structure is gone by codegen. That is one instance, it is not a bench, and the
question of what the *operations* lower to is untouched here and unpriced.

## What I did not cover

I read `RULES.md`, `01`, `04`, `SETTLED.md` in full, and `seed/SETTLED_surface.md:100-190`. After the
derivation I read `12` in full, `11` sections 7 and 8, and `10` sections 7 and 8. I did not read the
rest of `11` or `10`, and I did not read `02`, `03`, `05` through `09`, `CANON_CANDIDATE.md`,
`DROPLIST.md`, `MORNING.md` or `PERSONA_CALLS.md` at all. So where `12` reports something from `11`'s
survey sections or from `10`'s route list, I am relaying `12` rather than checking it, and the two
exceptions are `11` section 7.3 and `10` section 7, which I opened myself.

I did not recompile `11`'s or `12`'s probes. `12` recompiled `11`'s `b03` and reports exit 0; I took
that rather than repeat it, so my agreement with `11` section 7.3 rests on `11`'s own text plus my
independent `p14`, not on rerunning `b03`.

I did not verify that arithmetic *operations* (as opposed to shapes) work through the containers; the
probes carry shapes and sizes, not addition on the values. I did not touch signed numerals, the
strategy axis beyond a phantom, `Cold` packing, or the wide rung's alignment story, where the byte
buffer is align 1 and a `Hot` wide container reportedly wants align 16. I did not price anything: no
harness bench was run, and every compile-time question here is **unpriced**.

## Extending the bridge does not require spelling a digit tower

`SETTLED.md:105` records that the bridge is consumer-extensible, at ONE EXPERT, compiled. Nobody
priced what a consumer types to extend it, and the obvious guess is bad: a row maps a literal to a
nat, and a nat is a digit tower, so extending the table looks like hand-typing
`O<E<E<O<O<E<O<Z>>>>>>>` correctly.

It is not. The row can be written in arithmetic over widths the library already ships
(`p29_extension_without_towers.rs`, exit 0):

```rust
impl Repr for L<104> { type R = Sum<N64, Sum<N32, N8>>; }
impl Repr for L<208> { type R = Sum<Sum<N64, N64>, Sum<N64, N16>>; }
```

and the same file checks both against the algebra:

```rust
ReprOf<L<104>>: Same<Sum<N52, N52>>,
ReprOf<L<208>>: Same<Sum<Sum<N52, N52>, Sum<N52, N52>>>,
```

Exit 0. So the extension line is readable arithmetic rather than plumbing, and a consumer who needs
width 777 writes one line naming numbers they already understand. This removes what I expected to be
the strongest objection to the whole family, and it is worth a second read because I found it while
trying to break arrangement D rather than while trying to support it.

## Every probe, and one command that runs them

`13_probes/verify.sh` re-runs the lot and prints the expected outcome beside the actual one. Its
output is committed as `13_probes/out_verify.txt`. Nine files must compile clean, twelve must refuse,
two produce runtime output. Last line of the committed run: `unexpected outcomes: 0`.

Every probe here is a spike. Its names, arities and field orders are scaffolding chosen to reach a
check, not design decisions, and nothing in it should be read as a proposed spelling.

Feature gates: `grep -c '#!\[feature' p13_surface_bounded.rs p14_multiply_past_the_table.rs
p21_named_widths.rs` returns 0, 0, 0. The two that carry gates are `p02` through `p04`
(`min_generic_const_args`) and `p19` (`lazy_type_alias`), and all four are refusals, so nothing
positive in this file rests on an unvetted or forbidden feature.

## What `12` says and where I stand

I opened `12` after everything above was on disk. Then `11` sections 7 and 8, and `10` sections 7 and
8.

### Where we agree, and it was reached separately

**The bar is met by a nat-keyed design.** `12`'s C4 is my arrangement A: the numeral is keyed on nats,
the const survives in exactly one place, a type alias with a const parameter, and the consumer's
spelling does not move. `12` measures it as byte-for-byte identical to the const-keyed surface; my
`p14` line reads `pub type Money = UFixed<13, 3>;` and I reached that by asking which direction of the
crossing is free rather than by counting characters. Same answer, two routes.

**The table's domain is bounded by the program's source text.** `12` section 6 puts it in a table; I
put it in `p14`, which multiplies four times to 256 bits with a 65-row table and exits 0. `11` section
7.3 got there first and localised it to the const surface. Three instances, and they are not three
hats on one model: `11` reached it by removing the const keying from its own failing case, `12` by
substituting a door into `11`'s case, and I by asking what the algebra actually consults.

**The reverse crossing is closed in four positions.** `12`'s `p07a` through `p07d` and my `p02`,
`p03`, `p04`, `p24` are the same four syntactic positions with the same terminal refusal naming
`generic_const_args`. Independently produced, identical result. I would call this one settled if
anything settled tonight.

**The binary-nat mismatch diagnostic cannot be acted on.** `12`'s K1 and my `p20` are the same message
from different crates, and both of us checked that E0308 has no annotation hook.

**The alias-definition site is silent.** `12`'s `p12` and my `p17` both compile an undeclared width at
an alias with no error at all, and both find the error at first use. `12` goes further and shows the
span lands on a name the consumer never wrote, in another file. That is the more damaging half and I
did not produce it.

**Nat canonicity holds.** `12` checks three points. I checked the whole matrix, `0..=64` by `0..=64`,
4225 pairs, with a negative control proving the instrument bites. Same conclusion, and the record now
has an exhaustive instrument on it rather than a sample, which matters because this identity is what
makes the multiply's output nameable at all.

### Where I was wrong and `12` is right

**I called `lazy_type_alias` a closed route. It is not closed, it is costed.** My `p19` turned the
gate on over the whole ladder, got the cascade, and I stopped. `12`'s `p14` writes the bounds on a
minimal case and the alias-site error lands on the line the consumer typed, with the full-ladder cost
measured at fifteen bounds. My conclusion was too strong and I withdraw it. The real objection to the
route is the one `12` names and does not resolve: the gate has to be on in the **consumer's** crate,
which is a serious thing for a library to require, and neither of us has thought it through.

### Where I go further than `12`

**The ceiling and the reverse wall are one fact.** `11` localised the ceiling to the const surface.
`12` inherited that and separately closed the reverse direction. Neither joins them, and joining them
is the part that generalises: an operation output can only be *named* by crossing nat to const, that
crossing is the same refusal as const to nat, and a table is the only implementation of either. So the
ceiling is not a property of the bridge and not really a property of the const surface. It is the
price of crossing **back**. `p23` shows it firing exactly that way, as a missing `Named` row reported
against a digit tower.

The design rule that falls out is one sentence and it is the thing I would put in the canon's intent
if anything went in tonight, which it does not: **cross once, at literals, in one direction.**

**A third arrangement, D, which is not in `12`.** `12`'s two repairs to the diagnostic are consts in
front with nats defaulted (`p06`), which costs a second head constructor for derived values (`p08`),
and a base-ten ladder (`p09`, `p10`), which costs roughly sixty impls. Arrangement D pays neither.

Let the width type be `L<K>`, the consumer's literal, with the structural nat behind a `Repr`
projection. Do not compute operation outputs at all: let the consumer declare the output width and
have the type system check it is wide enough by type-level comparison, which is free. There is no
reverse table, so there is nothing to run out of, and the printed type carries no tower at all:

```
error[E0308]: mismatched types
    |                    ------- ^ expected `13`, found `12`
    = note: expected struct `Numeral<L<13>, L<3>, _, _>`
               found struct `Numeral<L<12>, L<4>, _, _>`
```

That is cleaner than `12`'s K4, which recovers the headline but still prints the towers as noise.
`p25` compiles, exit 0, zero features. The refusal when the declaration is too narrow reads, with the
relation annotated (`p27`): "the declared output width is narrower than the product needs", and the
unannotated `note:` already carries all four numbers.

**D's cost, stated plainly and not undersold.** `let c = a * b;` no longer infers an output type, and
a product site states its shape. For a fixed-point library that may be the honest shape, since
somebody has to decide where the point goes in a product, but it is a real change to the use tier and
the bar does not govern there so nothing protects it. More seriously, I have **not** worked out what D
does to tier one, whose premise is `T: Add` with no typestate at all. That is the first thing I would
attack next and I did not attack it. D is one instance, unpriced, and it is a residue offered for
attack rather than a proposal.

`p28` shows the computed form and the declared form coexisting in one crate without coherence trouble,
so the two are not exclusive: a design could compute outputs where the table covers them and require a
declaration past that. I have not decided whether that composition is better than either alone and I
am not going to pretend to.

**The extension price.** Nobody had said what a consumer types to add a bridge row. `p29` says it is
arithmetic over shipped literals, verified against the algebra.

### One caution about how `12` will be read

`12` section 9's headline sentence is "the trade nobody was offered turns out not to be a trade". Its
own section 8 says something more careful: C4 pays the diagnostic unless one of two repairs is taken,
and each repair has a price. Both are in the file and only the first is quotable. A morning reader who
carries the section 9 sentence without the section 8 table will believe the nat keying is free, and it
is not. It is cheaper than the ceiling, which is the actual finding, and that is a comparison rather
than an absence.

I say this as a compression hazard rather than as a disagreement about the mechanism, where I think
`12` is right.

## What appears to hold, in the register the night allows

- The crossing is one wall with two directions, and both are refused by one rule. **Compiled, four
  positions, twice independently.**
- Type-level arithmetic normalises, so an operation output and a written literal are one type.
  **Compiled over the whole 4225-pair matrix with a negative control.**
- Container derivation over the rungs, the byte buffer, and ceil-to-bytes need no feature and no table.
  **Compiled over every width 0 to 130, with runtime sizes confirmed 0 to 128.**
- A nat-keyed design meets the ergonomics bar with the consumer's spelling unchanged. **Compiled,
  twice independently.**
- The ceiling is the cost of naming outputs, not of the bridge. **Compiled, and it fires as a reverse
  table miss.**
- Extending the bridge costs one readable line of arithmetic. **Compiled and checked against the
  algebra.**
- Every arrangement is silent at the alias-definition site. **Compiled, twice independently.**

Nothing above is settled, including the parts where `12` and I agree.

## What is op's

Stated as questions rather than as recommendations, because `04` is standing.

**Which cost the design would rather pay**, given that the writing bar does not distinguish the
candidates at all. The choices are the ceiling on the law algebra, the width-mismatch diagnostic, a
second head constructor, a base-ten ladder rewrite, or declared output widths. That is five options
and each has a compiled instance behind it now.

**Whether a product site declaring its output shape is acceptable or repugnant.** Arrangement D turns
entirely on this and it is a taste question about what a fixed-point consumer should have to say, not
a mechanism question. Nothing in the record answers it.

**Whether the alias-definition site being silent is tolerable.** It is a property of Rust type aliases
rather than of any candidate, it affects the exact site the ergonomics bar was written about, and the
only lever anyone has found requires a feature gate in the consumer's crate.

**Whether "bits and bytes" in the acceptance criterion names two units the surface should offer.** The
machinery to accept either already exists and is checked over every width; the surface offers one.
