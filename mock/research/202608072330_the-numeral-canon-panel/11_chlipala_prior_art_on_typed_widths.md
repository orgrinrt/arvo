# 11. Prior art on typed widths, and whether anyone has crossed the bridge

**Persona:** Adam Chlipala, certified-programming and proof-automation lens.
**Date:** 2026-08-08.
**Pin:** `rustc +nightly-2026-05-28`, reporting `1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`,
aarch64-apple-darwin. Other toolchains named where used, with versions.
**Probes:** `11_probes/`, sources plus `11_probes/out/` for emitted diagnostics. `11_probes/verify.sh`
recompiles all of them in one command and its output is committed at `11_probes/out/verify.txt`.
**Status:** nothing here settles anything, per `04`. This is a breadth pass looking outside the project.

## Canon gate

Passed. Checked against `RULES.md`, `01_op_answers.md` section 0 (an ack is not a ratification),
`04_op_no_settlements_tonight.md` (breadth, no settlements), and `SETTLED.md`. The dispatch asks for a
survey of prior art and a further attack on the bridge that `10` conceded. `SETTLED.md:165-168` lists
the bridge as open and names `10`'s concession as a concession rather than a closure, so attacking it
again is licensed rather than reopening a settled row. Nothing in this file enters `SETTLED.md`.


## The short answer, before the working

I went looking for a system that has arvo's bridge problem and solved it. There is not one, and the
reason is sharper than "nobody tried": **no other system in the survey is in the position that creates
the problem.**

The problem needs two conditions at once. The width must be a *generic parameter*, so a single
definition covers every width. And the type language must be unable to *compute on* that parameter, so
the parameter's value cannot pick a representation. Every system surveyed drops one of the two. C++,
Zig, GHC and Bluespec all compute on the parameter freely. Ada, VHDL and the Rust exact-width crates
never make the width generic in the first place. Chisel keeps the width out of the host type system
altogether. Rust holds both conditions, on purpose, and arvo sits exactly there.

So the property that removes the bridge is nameable and it is **not obtainable here**, which is a
closed route rather than an unexplored one. Section 4 gives the reason and section 5 gives the price
each system pays instead, because none of them gets it free.

Three things came out that I did not expect and that matter more than the survey.

**The Rust ecosystem has already built arvo's bridge, character for character, and shipped it.**
`typenum` declares `pub struct Const<const N: usize>`, `pub trait ToUInt { type Output; }` and one impl
per value, and generic code guards on `where Const<N>: ToUInt`. That is `Idx<N>`, `ToNat<M>` and
`where Idx<I>: ToNat<M>` under different names, arrived at independently. Its table is **1148 rows and
4758 lines of generated code**, dense over 0 to 1024 and sparse above. Section 3.10, with the commands.

**The table is not an ergonomic wart. It is a ceiling on the law algebra, and the design already
removed a ceiling once on structural grounds.** `10`'s own `mul` demands a bridge row for the *output*
width. Multiplication doubles widths, so the widths reachable from any declared set are unbounded, and
no finite table is closed under them. I compiled this: `b01` fails at the first step, `b02` adds the
missing row by hand and fails at the next octave. Section 7.

**The ceiling belongs to the const surface, not to the bridge.** `b03` is the same ladder, the same
nats, the same `Add` and the same `Container`, with the numeral keyed on nat types instead of const
coordinates. Three octaves of multiply compile with **no bridge anywhere**, ending at a 1636-bit
numeral in 208 bytes. Section 7.3. That localises the cost precisely and it moves the question from
`10`'s framing onto D48.

Two smaller corrections, both to things the record states more confidently than it should. `10`'s
diagnostic improvement (c) **prints a false sentence** when a table row is missing: `b01` emits
"width 48 is not the sum of widths 24 and 24". And `SETTLED.md:105`, "the bridge is consumer-extensible,
so the cap was never forced", survives across a real crate boundary but only in a form that **partitions
the numerals**: a consumer that adds one width gets a private universe in which arvo's own widths do not
exist until re-declared, and its numerals do not flow into anyone else's. Sections 8 and 9.

And one result I did not go looking for, which I would keep above the rest if I could keep one thing.
**The bridge is not blocked. Its codomain is.** `e01` writes an enumeration-free, total, uncapped
const-to-type bridge in a single blanket impl, no features, no flags, and builds a whole numeral on it
with no literal anywhere in the crate. It fails on one thing: the container it lands on is `N` bytes
for `N` bits, and closing that factor of eight is one division, refused in four further syntactic
positions that terminally name a forbidden feature. So the honest statement is that arvo is one `ceil`
away rather than a language away. Section 10.3.

I found no fourteenth route that dissolves the bridge. I found one shape that is genuinely not among
`10`'s thirteen, attacked it, and it dies on section 7 rather than on a refusal. Section 10.1.

Last, a precedent I nearly missed and which belongs to op rather than to me: **capacity was a const in
this workspace and became a type**, on his direction, and three crates dropped a forbidden feature gate
because of it. That is the same migration `b03` performs on width, one axis over, already taken.
Section 7.4.

## 1. The wall, restated more precisely than the record states it

`10:520-525` puts it this way:

> Rust admits a generic parameter into type position only as a standalone argument of the item that
> parameterises it. A const's *value* is therefore unreachable to the type system: it cannot be halved,
> compared, added, or matched.

That is right and it undersells one half of itself. The first sentence is not only a restriction, it is
a permission, and the permission is what the survey has to be measured against.

**A bare const parameter does reach type position.** `struct Foo<const N: usize>([u8; N]);` compiles on
stable with no feature. So there exists a family of generic const-to-type functions Rust admits: those
whose result is built from array types whose lengths are bare parameters. `Idx<N> -> [u8; N]` is a
total, uncapped, enumeration-free const-to-type map, written once.

So the wall is not "const to type is impossible". It is:

> The only generic const-to-type functions Rust admits are those in which every const appears as a bare
> parameter. Any function whose result depends on an *arithmetic* image of the parameter is refused, in
> every syntactic position, and the compiler names `generic_const_args` as the escape.

`10` established the second clause with six compiled refusals and I did not re-derive them; I checked
that its `p01` through `p07` are what it says they are and took the result. The first clause is what I
add, and it matters because arvo's required map is `N -> [u64; ceil(N/64)]`, whose only defect against
the permitted family is the `ceil` and the division. **The codomain is the problem, not the domain.**
That is a more actionable statement than "const to type is walled", and section 10 is what happens when
you try to act on it.

## 2. What each system was asked

Three questions per system, per the dispatch. What is the mechanism. Does it have arvo's bridge
problem, and if not, which property of the language removes it. Could that property be obtained here.

One framing note before the entries. "Arvo's bridge problem" is a specific thing and it is easy to
score a system as passing when it is answering a different question. The test I applied is the ratified
gate at `SETTLED.md:66-71`, all four parts: the consumer expresses usage in bits and bytes, the
typestate derives the container and representation, it validates, and it erases on lowering. A system
that derives but does not erase fails. A system that erases but makes the consumer name the container
fails. I mark which part each one misses rather than scoring pass or fail, because the misses are the
informative part.

## 3. The survey

### 3.1 C++, non-type template parameters. **No bridge problem.** Checked, compiled.

**Mechanism.** A template's non-type parameter may appear inside arbitrary constant expressions in
template-argument position. `std::array<std::uint64_t, (N + 63u) / 64u>` is a legal type built from
arithmetic on `N`. The container derivation is therefore an ordinary compile-time function, written
once, over every width.

**Does it have the problem? No.** The property that removes it: **a template argument is a constant
expression, and constant expressions are closed under arithmetic.** There is no boundary between the
value world and the type world to cross, because template arguments live in both.

**Checked**, not recalled. `11_probes/a01_cpp_nttp_derivation.cpp`, Homebrew clang 22.1.8, target
arm64-apple-darwin25.5.0:

```
clang++ -std=c++20 -O2 -c a01_cpp_nttp_derivation.cpp -o out/a01.o
```

Compiles clean. Six `static_assert`s on layout pass, including `sizeof(Fixed<4711, 1>) == 592`, a width
for which nothing is declared anywhere in the file. And erasure holds: `nm out/a01.o` gives `_arvo16`
at `0x0` and `_native16` at `0xc`, and `objdump -d` shows both as the identical three instructions
`add w8, w1, w0 / and w0, w8, #0xffff / ret`.

**Could the property be obtained here? No, and this is the important entry.** The property is exactly
`generic_const_exprs`, which is forbidden, and the reason Rust refuses it is the reason C++ can afford
it. C++ does not type-check a template before instantiation; a template body is checked when it is
instantiated, against the arguments it got. Rust checks a generic definition **once, abstractly,
against its bounds**, before any instantiation exists. Abstract checking requires the solver to reason
about `N` knowing nothing about its value, and a rule that branches on `N`'s value is precisely what it
cannot do.

So the bridge is not an oversight in Rust's design. **It is the shadow of pre-instantiation checking**,
which is the property Rust has and C++ does not. That reframing survives the rest of the survey and is
the single most useful thing in this file.

### 3.2 Zig, comptime. **No bridge problem.** Checked, compiled. And it is the strongest case.

**Mechanism.** Types are comptime values, so a function from a width to a type is an ordinary function
returning `type`. Zig goes further than C++ and ships arbitrary-width integers natively: `@Int(.unsigned, n)`
reifies an integer type of exactly `n` bits for any `n`, so the native rung of arvo's ladder does not
need to exist.

**Does it have the problem? No.** The property: **there is one language and one evaluator for terms and
types, so `const -> type` is function application.**

**Checked.** `11_probes/a02_zig_comptime_derivation.zig`, Zig 0.16.0:

```
zig build-obj a02_zig_comptime_derivation.zig -O ReleaseFast -femit-bin=out/a02.o
```

Compiles clean. Six comptime assertions pass including `@sizeOf(Fixed(4711, 1)) == 592` and
`@bitSizeOf(Container(13)) == 13`. Erasure is total rather than merely equal: `nm out/a02.o` puts
`_arvo16` and `_native16` **at the same address**, one body, `objdump` confirms a single six-instruction
function.

One correction to my own first attempt, recorded because a later reader will hit it. Zig 0.16 removed
`@Type` in favour of `@Int(signedness, bits)`; `std.meta.Int` is a deprecated wrapper over it
(`/opt/homebrew/Cellar/zig/0.16.0_1/lib/zig/std/meta.zig:754`). My first probe used `@Type` and failed
with `invalid builtin function`.

**Could the property be obtained here? No**, and for a stronger reason than C++'s. Zig has no generics
in Rust's sense at all: a "generic" function is a function over comptime values, and it is checked when
called. There is no abstract checking to preserve, so there is nothing the property costs Zig that it
would not cost Rust. Adopting it means giving up the thing arvo's typestate is for.

### 3.3 Haskell and GHC's `Nat`. **No bridge problem for literals.** Checked against the docs, not compiled.

**Mechanism.** `Nat` is a kind of type-level naturals with numeric literals inhabiting it directly:
`13` in type position *is* the natural. GHC ships type families for arithmetic and comparison on it.
Fetched from `GHC.TypeNats` in `base-4.22.0.0`: addition, multiplication, exponentiation, subtraction,
`Div`, `Mod`, `Log2`, and `CmpNat` with `(<=?)` and `(<=)`. So `Fixed (i + f)` needs no bridge; the
solver does the arithmetic.

**Does it have the problem? Not for written literals.** The property: **the compiler ships primitive
type-level arithmetic on a numeric kind.** This is the same property as C++'s, obtained differently:
rather than making type arguments be expressions, GHC makes numbers be types and teaches the solver
arithmetic on them.

**But the value-to-type direction is a different story and it is worth the space**, because it is the
closest anyone comes to the map arvo wants at runtime. `someNatVal :: Natural -> SomeNat` and
`withSomeSNat :: Natural -> (forall n. SNat n -> r) -> r` do promote a value to a type, over the
unbounded domain, with no table. They do it **existentially**: the resulting `n` is opaque, and what you
get with it is a runtime `KnownNat` dictionary. So the map is total and **non-erasing**, since nothing
downstream monomorphises on `n`. Against the ratified gate that fails part four outright.

That is a useful negative result for arvo. The one construction in the survey that maps an unbounded
set of values to types with no enumeration buys its totality by not erasing, which is exactly the trade
arvo's gate refuses.

**Could the property be obtained here?** The literal-arithmetic half is `generic_const_exprs` again, and
forbidden. The existential half is `dyn`-shaped and forbidden twice over, by the gate and by arvo's own
no-`dyn` rule.

**And GHC pays for the property, visibly.** The built-in solver is syntactic and incomplete: fetched
from `ghc-typelits-natnormalise`'s own description, GHC "cannot automatically prove equality between
algebraically equivalent expressions", and the plugin exists to normalise both sides to a sum-of-products
form first. So the currency for compiler-primitive type-level arithmetic is a permanent, incomplete
arithmetic decision procedure in the compiler, which third parties then patch. That is a real cost and
it is the one a `generic_const_exprs` future would import.

### 3.4 Bluespec. **No bridge problem.** Checked against the source, not compiled.

**Mechanism.** BSV has `numeric type` as a distinct kind, `Bit#(n)` indexed by it, and the compiler
ships numeric type functions. Fetched from `bsc`'s `Prelude.bs`: `TAdd`, `TSub`, `TMul`, `TDiv`, `TLog`,
`TExp`, `TMax`, `TMin`, alongside `SizeOf` in the `Bits` class and `valueOf` for the type-to-value
direction. So a container derivation is `TDiv#(TAdd#(i, f), 64)` and is written once.

**Does it have the problem? No.** Same property as GHC's, in a hardware-description setting: **the
compiler ships primitive arithmetic on a numeric kind.** Worth noting separately from GHC because it is
independent evidence that a language designed *for* arbitrary bit widths reaches for exactly this, which
tells you the requirement is real rather than arvo-specific.

**Could it be obtained here? No.** Same answer as 3.1 and 3.3.

### 3.5 VHDL and SystemC. **No bridge problem.** Recollection, not checked. Flagged.

**I have no VHDL toolchain on this machine** (`which ghdl` finds nothing), so this entry is recollection
and should be read at that weight.

**Mechanism, as I understand it.** VHDL's `std_logic_vector(N-1 downto 0)` takes `N` from a generic, and
`N-1` is an ordinary expression. Generics are resolved at *elaboration*, when the design hierarchy is
instantiated, and the elaborated design is what is checked and synthesised. SystemC's `sc_int<N>` and
`sc_bigint<N>` are C++ templates and inherit 3.1 exactly.

**The property, and it is a third one.** VHDL removes the bridge by **elaborating before checking**.
There is no notion of a generic entity being verified independently of its instantiations, so a
value-dependent shape never has to be reasoned about abstractly. This is C++'s property arrived at from
the other direction: C++ declines to check early, VHDL declines to have an unelaborated stage at all.

**Could it be obtained here? No**, and it is the same trade as 3.1. Post-monomorphisation checking is
available in Rust and arvo has already ruled on what it costs: `130b:82-86`, quoted in the closed
panel's checkpoint, accepts the post-monomorphisation hole as "monomorphisation working as intended"
and rules that the task is making it legible rather than closing it. A design that moved the *width
derivation* into that hole would be enlarging a hole op has already decided only to illuminate.

### 3.6 Chisel. **No bridge problem, because there is no type-level width.** Recollection, flagged.

**No Scala toolchain here either.** Recollection.

**Mechanism.** A Chisel width is a *runtime value of the host language*: `UInt(13.W)` constructs a
node in a circuit graph, and `13` is an ordinary Scala `Int`. The Scala program is a generator; its
output is FIRRTL, and width inference and checking happen as a pass over that output. So the Scala type
system never sees the width.

**The property.** **The width lives outside the type system entirely**, and correctness is recovered by
a separate checker over the generated artifact.

**Could it be obtained here?** It is available and it is the shape arvo exists to refuse. It is worth
naming precisely because it is the limit case of `10`'s reframing at `10:532-537`: if the const does not
have to be on the surface, the furthest version of that is the const not being in the type system at
all, and Chisel is what that looks like at scale. It works, it is used in production silicon, and it
gives up every guarantee arvo's typestate provides at the definition site.

### 3.7 Ada. **No bridge problem, because the width is never generic.** Recollection, flagged.

**No GNAT here** (`which gnatmake` finds nothing). Recollection, and this is the entry I would most
like someone with a compiler to check.

**Mechanism.** Ada's fixed point is declared, not parameterised. `type Frac is delta 2.0**(-15) range -1.0 .. 1.0;`
states a required resolution and a required range, both as static expressions, and the compiler derives
a machine representation satisfying them, with `'Small` and a representation clause available to pin it.
This is the oldest serious answer in the survey and the closest in *intent* to arvo's gate: the
programmer states usage, the compiler derives the container, and the result erases to machine
arithmetic.

**Does it have the problem? No, and the reason is the most interesting one in the survey.** Ada does not
compute a representation from a *generic parameter*. It computes one from a *declaration*, at the point
the declaration is written, once per type. There is nothing to be generic over, so there is nothing to
bridge.

**The property: the derivation runs at a declaration site, over concrete numbers, rather than at a
generic site over an abstract parameter.** That is a fourth distinct property and it is the only one in
the survey that is *available in Rust*, because it is not a language feature at all. A Rust type alias
`type Frac = Fixed<0, 15, Warm>;` is a declaration site with concrete numbers in it.

**Could it be obtained here? Structurally yes, and section 10 is what happens when I try.** This is the
route worth pursuing and it is the one I spent the rest of the dispatch on.

Two honest cautions about the entry. First, Ada makes you *write the declaration*: there is no Ada
program that uses a 13-bit fixed-point type without a `type` declaration for it somewhere. So the
comparison does not support "the consumer should have to write nothing", it supports "the consumer
writes one declaration and it looks like a type definition". Second, Ada generics over a formal fixed
point (`type T is delta <>`) are instantiated per use, which is 3.1's trade again, so the property holds
of Ada's *declarations* and not of its generics.

### 3.8 Refinement types: Liquid Haskell, F\*. **Different problem, and the difference is the finding.** Recollection, flagged.

**No toolchain here.** Recollection.

**Mechanism.** A numeric constraint is a *predicate* attached to an ordinary type, `{v:Int | 0 <= v && v < 8192}`,
discharged by an SMT solver. The number never becomes a type index; it stays a number, and the solver
reasons about it.

**Does it have arvo's bridge problem? It does not have arvo's bridge.** But it does not have arvo's
derivation either, and this is the part worth carrying: **a refinement constrains a representation, it
does not choose one.** Liquid Haskell's refined `Int` is an `Int`. F\*'s machine integers are
`Lib.IntTypes`-style, indexed by an enumerated `inttype` of the widths the library ships, which is a
table of five or so, chosen because the machine has five.

So refinement types answer part three of the ratified gate, validation, extremely well and do not
attempt part two, derivation. Against arvo they are a **validation technology, not a derivation one**,
and the record should not expect a route from them.

**Could the property be obtained here? Partially, and it is already there.** The Rust analogue of an
SMT-discharged predicate at a site where no abstract checking is required is a post-monomorphisation
`const` assertion, which is gate-free, unbounded, and carries a custom message. It validates; it derives
nothing. Section 10.3 measures what it can and cannot do.

### 3.9 Dependently typed languages: Idris, Agda, Coq. **No bridge, and the cleanest statement of why.** Recollection, flagged.

**No toolchain here** (`which idris2 agda coqc` finds nothing). Recollection, and I will keep it short
because it is the entry where recollection is safest: this is the part of the literature I know best and
the claim is structural rather than about any version's behaviour.

**Mechanism.** `Vect : Nat -> Type -> Type` takes a *value* as an index. There is one language, one
evaluator, and one notion of equality up to reduction, so a function `Nat -> Type` is an ordinary
function and `Vect (n + m) a` typechecks because the typechecker evaluates `n + m`.

**The property, stated as generally as it goes: terms and types inhabit one language, so a map from
values to types is a function rather than a relation that has to be tabulated.** Every other "no bridge"
entry in this survey is a partial approximation of this one. C++ approximates it for constant
expressions in argument position. GHC approximates it for one kind with a fixed operator set. Zig gets
it fully for compile-time values but pays by abandoning abstract checking.

**Could it be obtained here? No, and the reason is not that Rust is not dependently typed.** It is that
type checking in the presence of a `Nat -> Type` function requires *deciding equality of open terms*,
which in general is undecidable and in practice is what totality checkers, universe hierarchies and
convertibility algorithms exist to manage. Rust's trait solver is a coherence-preserving, terminating
procedure by design. `generic_const_exprs`'s history is what happens when you ask it to do a small
piece of this, and the workspace's own record of that history says so: `unstable-features.md` records
the const-generics team calling the design "fundamentally flawed" and starting `min_generic_const_args`
as a ground-up rewrite.

The point for the canon: **the enumeration is not the price of Rust lacking something Idris has. It is
the price of a decision procedure that terminates and stays coherent.** That is a sentence with
permanence in it, and it is the honest reason the design will not get what section 3.9 describes.

### 3.10 The Rust ecosystem. **It built arvo's bridge and shipped it.** Checked against vendored source.

This is the entry the dispatch asked for last and it should have been first.

`typenum` is the type-level numeral crate in Rust. Read from the copy in this machine's registry,
version 1.20.1, at `~/.cargo/registry/src/*/typenum-1.20.1/src/gen/generic_const_mappings.rs`. Its
first three declarations are:

```rust
pub type U<const N: usize> = <Const<N> as ToUInt>::Output;
pub struct Const<const N: usize>;
pub trait ToUInt { type Output; }
impl ToUInt for Const<0> { type Output = U0; }
```

Set beside `10_probes/p12_improved_full.rs:24-37`:

```rust
pub struct Idx<const N: u32>;
pub trait ToNat<M> { type N; }
impl ToNat<Arvo> for Idx<0> { type N = T0; }
```

Same carrier struct, same one-method trait, same one impl per value. And the guard in generic code is
the same too. typenum's own doc comment, at that file's line 40, shows:

```rust
impl<const N: usize> MyTrait for MyStruct<N>
where
    Const<N> : ToUInt,
{ type AssocType = U<N>; }
```

which is `10`'s `where Idx<I>: ToNat<M>` character for character.

**The two constructions were arrived at independently and they are the same construction.** That is
three independent instances of the shape if you count the closed panel's `137`, typenum, and `10`'s
re-derivation, and it is the strongest available evidence that this is the shape Rust forces rather
than a shape anyone chose.

**Its table, measured.** Commands, and both numbers are reproducible:

```
grep -c '^impl ToUInt for Const<' src/gen/generic_const_mappings.rs   ->  1148
wc -l < src/gen/generic_const_mappings.rs                             ->  4758
```

Population shape, from the same file, extracted with
`grep -oE '^impl ToUInt for Const<[0-9]+>' ... | grep -oE '[0-9]+' | sort -n` and then walked for the
longest contiguous prefix:

| Segment | Rows |
|---|---|
| dense, 0 through 1024 | 1025 |
| sparse above 1024 | 123 |
| total | 1148 |

The sparse rows are powers of two and their predecessors (2047, 2048, 4095, 4096, 8191, 8192, ...),
round decimals up to `10000000000000000000`, and a handful of oddities such as 3600 that read as
answered feature requests. The largest are `#[cfg(target_pointer_width = "64")]`-gated.

So route 13 of `10`'s list, "a larger table", is not hypothetical. **It is what the ecosystem ships, at
1148 rows and 4758 generated lines, and it is still capped at 1024 dense.** `10:480-485` measured its
own dense-bridge spike at 513, 2049 and 8193 rows and found nothing breaks; typenum is the production
data point for the same claim, and its population choices are what "pick the widths people want" looks
like after a decade of issues.

**And typenum does not have arvo's closure problem, for a reason that is the whole of section 7.3.**
Its arithmetic stays in tower-land: `Sum<A, B>` operates on `UInt` towers, and its main consumer
`generic-array` spells lengths as towers too. Read from `generic-array-0.14.7/src/lib.rs:179`:

```rust
pub struct GenericArray<T, U: ArrayLength<T>> { .. }
```

`U` is a type, not a const. So the bridge is crossed **once, at entry**, and never again. Arvo cannot
do that, because its ergonomics bar puts a decimal const at the surface, and section 7 is what that
costs.

## 4. The property that removes the bridge, named four ways

The survey produces four distinct properties, not one, and they are worth separating because they fail
for different reasons here.

**P1. Type arguments are constant expressions.** C++, and SystemC by inheritance. Removes the bridge by
erasing the boundary between the value world and the type world in argument position. **Unavailable:**
this is `generic_const_exprs`, forbidden, and forbidden because Rust checks a generic definition before
it is instantiated. C++ affords it by not doing that.

**P2. The compiler ships primitive arithmetic on a numeric kind.** GHC's `Nat`, Bluespec's `numeric type`.
Removes the bridge by making the numbers be types and teaching the solver to compute on them.
**Unavailable:** same feature, same reason. And it is not free where it exists: GHC's solver is
syntactic and incomplete on algebraic equivalence, which is why `ghc-typelits-natnormalise` exists.

**P3. Terms and types are one language.** Idris, Agda, Coq; Zig's comptime is the operational version of
it. Removes the bridge by making `Nat -> Type` an ordinary function. **Unavailable**, and not because
Rust "is not dependently typed": because type checking under such a function requires deciding equality
of open terms, and Rust's trait solver is a terminating, coherence-preserving procedure on purpose.

**P4. The derivation runs at a declaration site over concrete numbers, not at a generic site over an
abstract parameter.** Ada, and VHDL's subtype declarations. Removes the bridge by never making the width
generic. **Available.** This is not a language feature and Rust has declaration sites. Everything in
section 10 is an attempt to obtain it.

There is a fifth, which is a refusal rather than a property. **P5. The width is not in the type system.**
Chisel, and the FIRRTL pass that checks it afterwards. Available, and it is what arvo exists not to do.

The shape of the answer to the dispatch's first question is therefore: **three of the four properties
that remove the bridge are structurally unavailable here, each closed by a different consequence of
Rust's decision to check generic code abstractly. The fourth is available and is not a feature.**

## 5. What each system pays instead, which is the part the record needs

The comfortable reading of section 3 is "everyone else solved this, Rust is behind". It is wrong, and
the correct reading is that every system pays for the property, in a currency the survey makes visible.

| System | Property | What it pays |
|---|---|---|
| C++ | P1 | No pre-instantiation checking of templates. Errors surface at instantiation, in the consumer's build, against a body they did not write |
| Zig | P3 (operational) | No abstract checking at all. A generic is a comptime function, checked per call |
| GHC | P2 | A permanent, incomplete arithmetic decision procedure in the compiler, patched by third-party solver plugins |
| Bluespec | P2 | Same, in a compiler with a much smaller ecosystem to patch it |
| Idris, Agda, Coq | P3 | Undecidable conversion in general, managed by totality checking, universes and elaboration; type checking is a research-grade component |
| VHDL | elaboration-first | No independent checking of an unelaborated design |
| Ada | P4 | The programmer writes a type declaration per numeric type. There is no Ada program using a 13-bit fixed point without one |
| Chisel | P5 | Nothing is checked in the host type system; correctness is recovered by a pass over generated output |
| Refinement types | validation only | Derives no representation. F\*'s machine integers are an enumerated set of five |
| Rust plus arvo | none of P1 to P3 | A table |

Read down the last column and the table is not the outlier it looks like from inside. It is the
currency Rust's position forces, and it is the *most visible* currency in the list, which is a
different thing from being the most expensive one.

**That is the honest half of the "known price" answer, and section 7 is the half that undoes it.**
Because a price is something you pay once. What sections 7.1 and 7.2 establish is that this one keeps
being charged, and charges against a property the design has already ratified.

## 6. Where I stopped surveying and started attacking

At this point the survey had produced a clean negative: the property that removes the bridge is not
obtainable, three separate ways, and the fourth property is Ada's and is not a feature. That is a
defensible place to stop and it is the wrong one, because it answers the dispatch's first question and
leaves its second one resting on a comfortable framing.

The second question is whether the bridge is a defect or a price every system in this class pays.
Section 5's table makes it look like a price. So I went to test the framing rather than the answer,
starting from the one thing in `10` that struck me as underexamined: `mul` requires a bridge row for
the **output** width (`10_probes/p12_improved_full.rs:205`), and multiplication produces widths nobody
wrote.

What follows is three compiled results. They are the substance of this file.

## 7. The table is not a wart. It is a ceiling on the law algebra

### 7.1 The first octave. Compiled.

`11_probes/b01_table_caps_the_algebra.rs` is `10_probes/p12_improved_full.rs` copied unmodified, plus
one function at line 245:

```rust
pub fn closure_site(a: UFixed<24, 8, Hot>, b: UFixed<24, 8, Hot>) -> Fixed<48, 16, Hot> {
    mul(a, b)
}
```

Both input widths, 24 and 8, are in `137`'s table. The output widths are 48 and 16; 16 is in the table
and 48 is not.

```
rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib --emit=metadata \
      -o out/b01.meta b01_table_caps_the_algebra.rs
```

```
error[E0277]: arvo does not ship this width: Idx<48>
   --> b01_table_caps_the_algebra.rs:245:70
    |
245 | pub fn closure_site(a: UFixed<24, 8, Hot>, b: UFixed<24, 8, Hot>) -> Fixed<48, 16, Hot> {
    |                                                                      ^^^^^^^^^^^^^^^^^^
    = note: widths are opt-in per program. Add `impl ToNat<MyWidths> for Idx<48>` ...
```

**The ladder can build a 64-bit container without being asked.** `10:196` says so: "The ladder does not
know what a strategy is; it maps a width to a container", and it is total. Nothing about 48 bits is hard.
The refusal is the table's alone.

Something else is visible in `137`'s own table and worth naming, because it is the shape of the problem
sitting in the record already. The rows are:

```
0, 3, 8, 13, 16, 24, 6, 26, 30, 40, 64, 100, 200
```

Note 26 and 6, out of order, appended after the round numbers. They exist because `law_site` at line 223
multiplies 13.3 by 13.3 and needs 26 and 6 to typecheck. **The author had to close the table by hand for
one multiply**, and the ordering records that it happened afterwards.

### 7.2 Adding the row does not fix it. Compiled.

`11_probes/b02_the_table_chases_its_tail.rs` is the same file with one row added at line 41:

```rust
48 => D0<D0<D0<D0<D1<D1<Term>>>>>>,
```

Worth pausing on what that row is. It is not a width and a name; it is the **binary expansion of 48,
least significant digit first, computed by hand and spelled as a type**. That is what "one impl per
width" costs in practice, and neither `10` nor `SETTLED.md` says it. It also means route 13's dense
table needs a generator, which is why typenum's is 4758 lines of `// THIS IS GENERATED CODE`.

The row closes `b01`'s failure: `b02_now_compiles` at line 245 typechecks. Then line 250:

```rust
pub fn b02_next_octave(a: UFixed<48, 16, Hot>, b: UFixed<48, 16, Hot>) -> Fixed<96, 32, Hot> {
    mul(a, b)
}
```

```
error[E0277]: arvo does not ship this width: Idx<96>
error[E0277]: arvo does not ship this width: Idx<32>
error: aborting due to 4 previous errors
```

**The argument, and it does not depend on the table's size.** Multiplication of an `I.F` numeral by an
`I.F` numeral yields `2I.2F`. The closure of any nonempty width set under that map is unbounded:
24, 48, 96, 192, 384, and so on without end. So **no finite table is closed under the law algebra**,
at 13 rows, at 1148 rows, or at any size. Addition is no better: `n` plus `n` carries to `n+1`, which
walks the integers one at a time.

This is not the same complaint as "the enumeration is ugly". `SETTLED_container.md:157-169`, as `10`
reports it at `10:513-514`, killed a fixed-width carrier on the grounds that **a fixed width is a
ceiling and the ceiling was removed**. A finite bridge table is a ceiling of exactly that kind, moved
off the storage and onto the algebra. That is the same structural objection, against the same design,
one level up, and the record does not contain it.

**So route 13 should be reclassified.** `10:518` lists it as "Expressible (section 7.2), and the refused
shape. Not proposed." On this evidence it is expressible, refused, **and structurally insufficient**,
which closes it on grounds that would hold even if the refusal moved.

### 7.3 The ceiling belongs to the const surface, not to the bridge. Compiled.

Establishing a defect is worth less than localising it, so the next question is which ingredient causes
the ceiling: the table, or the fact that the surface spells widths as decimal consts.

`11_probes/b03_the_ceiling_is_the_const_surface.rs` is the same file with the failing site removed and
a second numeral added, keyed on the nat **types** rather than on const coordinates. Same ladder, same
`Add`, same `Container`, same digits. The only occurrence of `Idx`, `ToNat` or `Arvo` in the added half
is inside a comment, which `grep -nE "Idx|ToNat|Arvo"` over that half confirms: one hit, line 25, a
comment.

```
rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib --emit=metadata \
      -o out/b03.meta b03_the_ceiling_is_the_const_surface.rs
```

**Compiles clean.** Empty stderr, 114864 bytes of metadata. What compiles:

- `b03_octave_1`, 24.8 times 24.8, which `b01` could not write.
- `b03_octave_2`, the octave `b02` could not reach.
- `b03_arbitrary`, 777.41 times 777.41, giving 1554 integer bits and 82 fractional bits.

And two `const` layout assertions pass inside it: the 48.16 result is 8 bytes, one `u64`; the 1636-bit
result is 208 bytes, 26 words. So the container derivation is still exact at widths no table anywhere
contains.

**The finding.** The ladder and the laws are closed and unbounded. The ceiling is created entirely by
the surface carrying the width as a const, because that is what forces the output type to be spelled as
a const, which is what forces a bridge row for a number nobody wrote.

That is not a proposal. Spelling a width as a type at the alias site is route 12 and is refused at
`142c:375-377` on the ergonomics bar, in terms that do not soften: "the alias writer has to know that a
width is spelled as a type and which type, which is plumbing surfacing at the worst possible site." I am
not reopening it. I am reporting that the const surface's cost is larger than the record prices it, and
that the cost is not ergonomic.

It also relocates `10`'s closing reframing. `10:532-537` offers, as a read rather than a proposal, that
the thing to explore is whether the const has to be on the surface at all, and files it under D48 as
op's. Section 7.3 gives that read a compiled consequence: **the const on the surface is what caps the
algebra**, so D48 is not only an ergonomics question about spelling. It decides whether arvo's laws are
closed.

### 7.4 The workspace has already made this exact migration, on a different axis

I nearly finished without checking whether the move `b03` describes has a precedent here, and it does,
which changes how much weight the ergonomics refusal can carry on its own.

`unstable-features.md:74`, the forbidden-features table, recording op's 2026-07-28 ruling on
`generic_const_exprs`:

> **The sketch work op led settled it empirically: everything the stack needs works under the `min_`
> version.** The capacity-as-a-type migration is that result in shipped form, and `arvo-comb`,
> `arvo-graph` and `arvo-spectral` each dropped their gate because of it.

And the workspace rule that generalises it, `a-refused-bound-wants-a-trait-not-a-feature.md`, stated by
op on 2026-08-07, one day before this panel:

> **The wall is almost never real.** The same intent, often the same expression, expressed as a **trait
> contract** is valid in that position and the solver is happy. Nothing about what is being constrained
> changes. Only the spelling does. ... Carry the derived quantity as an associated type or associated
> const on a trait, and bound on the trait.

**Capacity was a const and became a type**, and three crates dropped a forbidden feature gate as a
result. That is the same migration `b03` performs on width, on a different axis, already taken, on op's
own direction, with the evidence recorded as a workspace rule rather than as an expert's proposal.

**Two things this does and two it does not.**

It removes "this is an exotic shape nobody has tried here" as an objection, and it means the
implementation risk is known rather than estimated. And it means the general rule op wrote one day ago
points at exactly the move section 7.3 compiled.

It does **not** override `142c:375-377`. Capacity is not written at an ergonomics-critical site the way
`UInt<5>` is; a consumer of `arvo-comb` does not spell a capacity in an alias definition, and the whole
force of `142c` is about the one site where an uninitiated writer has no context. So the precedent
establishes the mechanism and says nothing about the spelling, which is precisely what the refusal was
about.

And it does not make me a proposer of the shape. The refusal stands, I am not reopening it, and
`RULES.md` forbids re-proposing a refused shape as a least-bad option. What I am reporting is that the
refusal was made against a cost priced in ergonomics, and section 7.2 has since put a ratified property
on the other side of the scale, and the migration itself has a shipped precedent. Those three facts
together are op's to weigh and nobody else's.

Two honest cautions. `unstable-features.md` is a workspace rule and I am citing it for the historical
fact that the migration happened, not as evidence about what is correct now. And the crates it names
sit under `mock/crates`, which the brief declares is being nuked and is not evidence about what is
correct, so I have not opened them and no claim here rests on their current contents.

## 8. A correction to `10`: the improved diagnostic prints a false sentence

`10` section 5.1 moves the law's associated-type equality behind a named `WidthSum` relation carrying
`#[diagnostic::on_unimplemented]`, and reports the result as pure gain at identical codegen. The codegen
claim I did not re-derive and have no reason to doubt. The diagnostic claim does not hold in one case,
and `b01` produced it without being asked.

The relation is blanket-implemented at `b01:177-185`:

```rust
#[diagnostic::do_not_recommend]
impl<M, const A: u32, const B: u32, const C: u32> WidthSum<A, B, C> for M
where
    Idx<A>: ToNat<M>,
    Idx<B>: ToNat<M>,
    Idx<C>: ToNat<M>,
    <Idx<A> as ToNat<M>>::N: Add<<Idx<B> as ToNat<M>>::N, O = <Idx<C> as ToNat<M>>::N>,
```

The blanket impl requires `Idx<C>: ToNat<M>`. So a **missing table row** makes the relation
unimplemented, `do_not_recommend` suppresses the where-clause, and the message speaks:

```
error[E0277]: width 48 is not the sum of widths 24 and 24
    = note: the result of this operation is 24 + 24 bits wide; write that width, or let it be inferred
```

48 is the sum of 24 and 24. The consumer wrote the correct width and is told it is wrong, and the note
instructs them to write the width they already wrote. `b02` prints the same falsehood twice, at
"width 96 is not the sum of widths 48 and 48" and "width 32 is not the sum of widths 16 and 16".

This is not fatal to the improvement and I would still ship it. The repair looks small: the arithmetic
clause and the membership clauses want separating, so that a missing row reports as a missing row and
only a genuine mismatch reports as a mismatch. I did not build the repair, so **that is an untested
suggestion and should be read as one.** What is tested is the defect.

It is worth stating why this slipped through. `10` tested the relation with a *wrong* width that was in
the table (`p09a` chooses 30 "which is in the table, so the bridge does not mask the law", at
`10:242-243`). That is the right control for the case it was testing and it is precisely the case where
the two failure modes cannot be told apart. The untested case is the one that lies.

## 9. A second read of `SETTLED.md:105`, across a real crate boundary

`SETTLED.md:105` carries "The bridge is consumer-extensible, so the cap was never forced", at
`ONE EXPERT, compiled`. Under `RULES.md`'s rung discipline a `ONE EXPERT` row is a queue entry asking
for the second read it has not had, so I gave it one.

The claim had been compiled in a single file, where every type is local and the orphan rule is not in
play. `11_probes/c_orphan/` splits it into two crates.

```
rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib arvo_min.rs -o libarvo_min.rlib
rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
      --extern arvo_min=libarvo_min.rlib --emit=metadata -o ../out/c01b.meta consumer_ok.rs
```

**The row survives.** `consumer_ok.rs` compiles: `impl ToNat<Mine> for Idx<7>` is admitted from a
downstream crate, because the local marker sits in trait-parameter position ahead of any uncovered type
parameter. Extensibility is real.

**And the same impl against arvo's own marker is structurally impossible**, `consumer_bad.rs`:

```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside
 9 | impl ToNat<Arvo> for Idx<7> { type N = T7; }
   = note: impl doesn't have any local type before any uncovered type parameters
```

So the marker parameter is not a convenience. **It is load-bearing for coherence**, and any design that
removes it removes consumer extension entirely. I did not find that stated anywhere.

**What the second read adds, and it is not in the row.** A marker does not extend arvo's table. It
starts an empty one. `consumer_partition.rs` and its variant establish two consequences:

```
error[E0277]: the trait bound `arvo_min::Idx<13>: arvo_min::ToNat<LibA>` is not satisfied
help: the trait `ToNat<LibA>` is not implemented for `arvo_min::Idx<13>`
      but trait `ToNat<arvo_min::Arvo>` is implemented for it
```

A consumer that needs one extra width must **re-declare every arvo width it uses** against its own
marker, including `Idx<0>`, which `Fixed<13, 0, LibA>` needs for its fractional coordinate. And:

```rust
pub fn compose(x: Fixed<13, 0, LibA>) -> Fixed<13, 0, Arvo> { x }
```

does not typecheck. The same width, resolving to the identical nat `T13`, is a different numeral under a
different marker. **Two libraries that each add a width cannot exchange numerals**, at any width,
including widths neither of them added.

So the row's conclusion holds and its implication does not. The cap was not forced; a **partition** was,
and the partition is finer than the cap would have been. A consumer's choice is between accepting
arvo's shipped widths, or entering a private universe that composes with nobody. That is a cost of the
same kind as section 7's and it belongs beside it.

## 10. Is there a fourteenth route

Three candidates. One is genuinely not among `10`'s thirteen and dies on section 7 rather than on a
refusal. One collapses into a shape op refused in his own words. One is not a route at all but it
relocates the wall by one step, which is worth more than either.

I checked each against `10:500-518` before writing it up, per the standing instruction not to
re-propose a refused shape. Where a candidate is one of the thirteen I say which.

### 10.1 Route 14: obtain Ada's property by generating the table from declaration sites

**The shape.** Section 4's P4 is the one available property: run the derivation at a declaration site
over concrete numbers rather than at a generic site over an abstract parameter. Rust has declaration
sites. An attribute proc macro on a module could read the type aliases in it, extract the width
literals a consumer actually declared, and emit the bridge rows for exactly those. Nobody writes a
table and nobody writes a per-width line; the alias they were already writing *is* the width
declaration, which is precisely Ada's arrangement.

**It is not route 8.** Route 8 is "a proc macro at the surface", refused at `139b:30-35`, and op's words
there split the question rather than closing it:

> We'll gladly take all the proc macro crates we need and other optimisations alike. However, using a
> macro invokation in place of a type is not what we want, and I've already ruled on this.

`139b:37-39` states the split explicitly: "**proc-macro crates are welcome** ... **A macro invocation
standing where a type should be written is refused**. The mechanism is fine; the surface is not." An
attribute on a `type` alias does not stand where a type is written. The alias is a real type
definition and the spelling inside it is `UInt<5>`.

**And it is dead anyway, on section 7.2.** The macro changes who writes the table. It does not change
that the result is a table, and a table is finite, and no finite set of widths is closed under
multiplication. Worse than the general argument: the attribute can only see widths that are
*syntactically present*, and the widths a law produces are not present anywhere. `b01`'s failing width
48 appears in no source text; it is the sum of two 24s that rustc computed. So the generated table
would be strictly worse than a hand-written one, because it cannot even see the widths that break it.

That closes route 14 structurally rather than by refusal, which is the stronger closure. **It also
closes every variant of "generate the table somehow", of which route 14 is the best-behaved instance.**

Recording the second cost too, from section 9: the generated rows must target a local marker for
coherence, so a crate using the macro enters the partition, and its numerals stop composing with
anyone else's.

### 10.2 Route 15: carry the size as a bare const parameter

**The shape.** Section 1 establishes the one const-to-type family Rust admits generically. Take it
seriously: put the byte count on the surface as a third coordinate, and let the container be `[u8; B]`.
No bridge, no table, no nats, no marker.

`11_probes/d01_bare_parameter_carrier.rs`, gate-free, compiles:

```
rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
      --emit=metadata,asm -o out/d01.meta d01_bare_parameter_carrier.rs
```

**It has properties the bridge does not.** Three octaves of multiply compile, because no output width
can be unshipped when nothing is shipped. Arbitrary widths work with no declaration: `Fixed<4711, 1, 590, Hot>`
is a type alias and nothing else. And validation is available with a message of our own choosing.
`11_probes/d02_postmono_check_fires.rs` asks for 2 bytes at 40 bits:

```
error[E0080]: evaluation panicked: this numeral's byte count is too small for its bit width
   | evaluation of `Fixed::<40, 0, 2, Hot>::CHECK` failed here
```

**Two predictions of mine that the measurement refuted, recorded because I was wrong.** I expected a
byte array at align 1 to cost codegen against a native `u16`, and expected the vector case to lose
vectorisation. Reading `out/d01.s`:

- `_d01_native16 = _d01_arvo16` at line 144 of the assembly. A symbol alias. The scalar add is two
  instructions and the two functions are literally the same function.
- The 1024-element loop vectorises identically in both: 16 instructions, 10 SIMD lines, four `add.8h`
  on eight-lane halfwords, in both `_d01_arvo_vec` and `_d01_native_vec`. LLVM saw through
  `from_le_bytes` and `to_le_bytes` and recovered the lane structure.

Named accurately: **that is an ad-hoc quick spike, not a bench.** No harness ran, the instruction
counts are read off emitted assembly, and the magnitude of any difference is **unpriced**. What it
supports is the qualitative claim that vectorisation is not lost, which is a refutation of my
prediction and an ad-hoc spike may support that.

**It dies anyway, and on the ratified gate rather than on ergonomics.** `SETTLED.md:73-74` requires that
"the typestate derives the container and representation". Under route 15 the consumer computes
`ceil((I+F)/8)` by hand and writes it, at the type and at every law's output type. That is the consumer
naming the container in a thin disguise, which is route 11, refused by op directly at `130b:39-43`:

> Container naming is explicitly wrong. The entire idea of arvo is that the strategy guides container
> selection, not the user.

And it fails the ergonomics bar independently. `142c:379-381` prices arity as "one more thing an
uninitiated writer must know, at the one site where they have no context to draw on", and `UInt<5>`
is the standard. `UInt<5, 1>` is not `UInt<5>`.

So route 15 is **route 11 wearing a byte count**, and I am reporting it as closed rather than proposing
it. The part worth keeping is the two refuted predictions and the finding that post-monomorphisation
validation carries a custom message and names the offending instantiation. Its real weakness is
elsewhere: the error points at the assertion and at the library's own `let () = Self::CHECK;`, and at
neither of the consumer's two sites. That is a more precise statement of "degraded diagnostic" than the
record has.

### 10.3 Not a route: the bridge is not blocked, its codomain is

This is the result I would keep if I could keep one thing from this file.

Every route in `10` section 8 treats the bridge as the blocked thing. Section 1 predicted that is too
coarse, because a bare const parameter reaches type position freely. Testing the prediction:

`11_probes/e01_enumeration_free_bridge.rs`, no features, no flags, **compiles**:

```rust
impl<const N: usize, M> ToNat<M> for Idx<N> {
    type N = [u8; N];
}
```

**One impl. Total over every width. No literal anywhere in the file.** An enumeration-free const-to-type
bridge exists in Rust, in one line, and it always did. The file then blanket-implements `Container` over
its codomain, also in one impl, and builds a whole numeral `Fixed<I, F, S>` generic over the width with
no enumeration in the crate. `W13`, `W4711` and `W1636` all resolve.

Then the assertion that decides whether it is worth anything, which passes:

```rust
assert!(core::mem::size_of::<ContainerFor<13>>() == 13);     // want 2
assert!(core::mem::size_of::<ContainerFor<64>>() == 64);     // want 8
assert!(core::mem::size_of::<ContainerFor<1636>>() == 1636); // want 208
```

**The container overshoots by exactly eight.** `N` bytes for `N` bits. And closing that factor of eight
is one division:

`e02_closing_the_overshoot.rs`, no features:

```
error: generic parameters may not be used in const operations
  |     type P = [u8; (N + 7) / 8];
  = help: const parameters may only be used as standalone arguments here, i.e. `N`
  = help: add `#![feature(generic_const_exprs)]`
```

`e03` under the permitted `min_generic_const_args` with `type const`:

```
error: complex const arguments must be placed inside of a `const` block
```

`e04`, following that suggestion:

```
error: generic parameters may not be used in const operations
  |     type const B: usize = const { (N + 7) / 8 };
  = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

`generic_const_args` is forbidden by the brief and by `unstable-features.md`. This is a **fourth
independent syntactic position** producing the same terminal refusal, after `10`'s three, and it is
the position that matters most because it is the one where the bridge otherwise works.

**The restatement this buys.** `10:520-525` says a const's value is unreachable to the type system.
That is true and it is not where the wall is, because a bridge that reaches nothing useful still
compiles. The wall is:

> The const-to-type bridge is not blocked. The only codomain Rust admits for a generic one is the array
> family whose length is the bare parameter, and the distance between that codomain and the one arvo
> needs is exactly one division by eight, which is refused in every position, terminally naming a
> forbidden feature.

That is a permanent sentence, it is compiled, and it says something the record does not: **arvo is one
`ceil` away, not a language away.** It does not open a route. It replaces "this is impossible" with
"this is impossible for one nameable reason", which is what a canon sentence should say and is worth
more than the vaguer claim.

I looked for a way to buy the division without the feature and did not find one. `[u64; N]` moves the
overshoot to sixty-four and aligns it, which is worse and fails `SETTLED.md:95` harder. A union does
not change size. Choosing the array element type per width is a table. The division is the whole of it.

### 10.4 What `10.3` means against op's actual criterion, which is narrower than the record uses it

Checking the refusal's own text before leaning on it. `137b:30-32`:

> One enumeration remained: the bridge from a written literal to a type, where a consumer wanting a
> width arvo did not ship adds one `impl ToNat<Mine> for Idx<7>` line. It was offered as acceptable.
> **Refused.**

and the criterion at `137b:32-36`:

> It should come implicitly from the heavy typestate. No enumerations, if we can help it

and, immediately after, `137b:43-45`:

> Buying `generic_const_args` plus `-Znext-solver=globally` to remove the line is not the alternative he
> is asking for either; that trades an enumeration for a feature and a flag, and his sentence is about
> **the derivation being implicit** rather than about which mechanism pays for it.

Two things follow that change how `e01` should be read.

**Op refused the bridge on an understated cost.** He was told it costs "one line, for a width arvo did
not ship". Section 9 measures it: a consumer that adds one width must re-declare every arvo width it
uses including `Idx<0>`, and its numerals then compose with nobody. The refusal was right and the thing
refused was cheaper than the thing that exists.

**And `e01` is the first construction that meets the stated criterion.** The criterion is that the
derivation come implicitly from the typestate, and `e01`'s does: one blanket impl, no literal anywhere,
total over every width, no feature. It fails on a factor of eight in the container and on nothing else.
That is a different position from "no construction meets the criterion", which is what the record
currently reads as, and it is why section 10.3 is worth more than a closed route.

I want to be precise about what I am and am not claiming. **`e01` is not a candidate**, because
`SETTLED.md:95` is a ratified row and eight-times overshoot is not a limitation to accept. What it
establishes is that the criterion is satisfiable in Rust today for everything except the container's
size, and that the container's size is one division away.

## 11. Is the bridge a defect, or the price everyone pays

The dispatch flags this as the comfortable conclusion and asks for it to be held to a higher standard
than the uncomfortable one. Holding it there, it does not survive intact, and what replaces it is
better for the canon than either answer alone.

**The half that is a price, and it is real.** Section 5's table shows every system in this class paying
for the property that removes the bridge, in a currency the survey makes visible: C++ and Zig give up
pre-instantiation checking, GHC and Bluespec carry an incomplete arithmetic decision procedure in the
compiler, dependent languages carry undecidable conversion, Ada makes you write a declaration per type,
Chisel keeps the width out of the type system entirely. And the ecosystem's own answer under Rust's
exact constraints is arvo's answer: typenum ships the same three declarations and 1148 rows of table.
A design that treats its table as a unique embarrassment is misreading its position.

**The half that is a defect, and it is decisive.** A price is paid once. This one is charged again
every time the algebra produces a width, and the widths it produces are unbounded, so no payment
closes it. `b01` and `b02` are the evidence: the table fails at the first multiply, closing it by hand
moves the failure one octave up, and the argument does not depend on the table's size. That is
structurally the same objection that killed the fixed-width carrier, and `SETTLED_container.md:157-169`
killed that one because a fixed width is a ceiling and the ceiling was removed. **The bridge reinstates
the ceiling on the law algebra**, and the record does not say so anywhere.

Two things follow that I would put in front of op before anything else in this file.

**The canon should not say the enumeration is a spelling problem.** It is what op called it at
`137b:32-36`, from the ladder, where he was right, and `10:532-537` already flagged the same doubt. On
section 7's evidence it is a **closure problem**: the table decides which widths the laws may produce,
which is a statement about the algebra rather than about how anything is written.

**And the ceiling is caused by the const on the surface, not by the bridge.** `b03` compiles three
octaves and a 1636-bit result with no bridge at all, on the same ladder, the same `Add`, the same
`Container`. That is not a proposal, because spelling a width as a type at the alias site is refused at
`142c:375-377` and I am not reopening it. It is a localisation: **D48 is not only about spelling. It
decides whether arvo's laws are closed**, and the fact that it decides that was not visible before.

So the honest formulation, offered for the morning and not as a settlement:

> The table is what Rust's position costs, and every neighbouring system pays a comparable cost in a
> different currency. The defect is not that arvo pays. It is that this particular currency is charged
> against a ratified property, unboundedly, and the charge is a consequence of the const surface rather
> than of the bridge.

## 12. Coverage, stated honestly

**What I compiled**, and it is the part to trust: the two comparators in C++ and Zig, the three
algebra-closure probes, the five coherence files across two crates, the two route-15 files, and the
five refusal positions in `e01` through `f01`. Seventeen sources of my own plus `ladder.rs` copied
from `10_probes/` because `p12` includes it, every one with its command in its header, all under
`11_probes/`.

**What I read rather than recalled**: `typenum-1.20.1` and `generic-array-0.14.7` from this machine's
cargo registry, with the counts produced by the commands quoted in section 3.10. GHC's `Nat` operator
set and `someNatVal`'s signature from `GHC.TypeNats` in `base-4.22.0.0`. GHC's solver incompleteness
from `ghc-typelits-natnormalise`'s own description. Bluespec's numeric type functions from `bsc`'s
`Prelude.bs`. Op's refusals from `130b`, `137b`, `139b` and `142c` in the closed panel, read at the
cited ranges only, not the panel as a whole.

**What is recollection, and should be read at that weight.** I have no toolchain on this machine for
Ada (`gnatmake` absent), VHDL (`ghdl` absent), Idris, Agda or Coq (all absent), Chisel or Scala, or
Liquid Haskell and F\*. Sections 3.5 through 3.9 are therefore unverified. Of those, **the Ada entry is
the one I would most want checked**, because it carries P4 and P4 is the only available property, and a
misremembering of how Ada's `delta` and `'Small` interact would change what section 10.1 is trying to
obtain. The dependent-types entry I am most confident in, because its claim is structural rather than
about any implementation's behaviour.

**What I did not attack.** I did not attempt a proc-macro implementation of route 14; section 7.2 kills
it structurally before the implementation question arises, and building it would have priced a route
that is dead for a reason implementation cannot touch. I did not re-derive `10`'s six refusals, only
checked that its `p01` through `p07` are what it says. I did not re-check `10`'s codegen-identity claim
for its four improvements; my correction in section 8 is to its diagnostic claim only.

**What is unpriced.** Everything. No harness ran. The codegen readings in section 10.2 are an ad-hoc
quick spike off emitted assembly. The compile-time cost of a 1148-row table, of typenum's, of arvo's,
and of the `e01` blanket bridge is **unpriced**, and section 3.10's row counts are counts rather than
costs.

**Two of my own predictions were refuted by measurement** and are recorded in section 10.2 rather than
quietly dropped: I expected a byte-array carrier at align 1 to cost codegen against a native `u16`, and
expected it to lose vectorisation. Neither happened.

**And one thing I could not do at all.** I did not find a way to buy the division by eight. Section
10.3's wall is where I stopped, and five syntactic positions is where I stopped believing another
position would help.

## 13. What appears to be op's, and in what order

Nothing here settles anything. These are the things this file puts in front of him, ordered by how much
depends on them.

**First, whether the closure argument holds, because everything else moves if it does.** Sections 7.1
and 7.2 say a finite bridge table caps the law algebra, unboundedly, and that this is the same
structural objection that killed the fixed-width carrier. If that is right, the bridge stops being an
ergonomics question and becomes a conflict between two ratified things, and `SETTLED.md`'s open list
needs the sentence. It is compiled and it is one expert's, so it wants a second read before it is
leaned on.

**Second, D48, with a consequence it did not have before, and a precedent.** `10:532-537` offered, as a
read, that the thing to explore is whether the const has to be on the surface. `b03` compiles three
octaves and a 1636-bit numeral with no bridge at all, on the same ladder, so the question decides
whether arvo's laws are closed and not only how a width is spelled. And section 7.4: capacity was a
const and became a type on op's own direction, three crates dropped a forbidden gate because of it, and
`a-refused-bound-wants-a-trait-not-a-feature.md` states the move as a general rule dated one day before
this panel opened. I am not proposing the type-on-surface shape; `142c:375-377` refuses it and that
refusal stands, and capacity was never spelled at the site `142c` protects. I am reporting that the
refusal was priced against ergonomics alone, that a ratified property has since landed on the other
side, and that the mechanism has shipped here before.

**Third, whether the bridge's real cost changes his refusal.** He refused it at `137b` on a cost of
"one line". Section 9 measures the cost across a crate boundary: a consumer that adds one width
re-declares every width it uses and exits the numeral universe. The refusal does not need revisiting;
what may need revisiting is anything that was decided on the understated figure.

**Fourth, `10`'s improvement (c).** It ships a false sentence when a table row is missing, demonstrated
in `b01` and twice in `b02`. Worth fixing before it lands, and my suggested repair is untested and
marked as such.

**Fifth, and smallest, whether the canon wants section 10.3's sentence.** "The bridge is not blocked;
the only codomain Rust admits for a generic one is the array family whose length is the bare parameter,
and the distance to the one arvo needs is one division by eight, refused in five positions, terminally
naming a forbidden feature." It is permanent, it is compiled, and it replaces a vaguer claim with a
nameable one. Whether a canon should carry a sentence that precise about a language's limits is his
call and not mine.
