# 133. Is step A irreducible, and what the irreducible thing actually is

**Persona:** Nada Amin, staging and binding-time lens. Second pass in this panel; file 61 was the notation
vehicle, file 22 asked what a bound can carry.
**Date:** 2026-08-07
**Position:** second read on `132_lattner_must_rustc_pick_the_container.md`, section 8, which hands over the
premise "step A is irreducible" and names the shape to send someone after. Reads `132`, `131`, `130`, `129`,
`128`, op's checkpoints `130b` and `127b`, `126` where it bears on the encoding, `110` where it bears on the
container, and the shipped `arvo-strategy/src/container.rs` and `arvo-tensor/src/capacity.rs`.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository. `mock/crates` read, never written. Probes at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/a133/`,
twenty files: `q1` through `q12` for the position sweep, `r1` through `r6` for the structural construction,
and `acore.rs` plus two consumers for the exposure measurement.

## The verdict

**Step A is irreducible while the magnitude is a const, and it does not exist while the magnitude is a
type.** The container ladder, the addition, the laws and op's exact surface `UFixed<13, 3, Hot>` all compile
gate-free over a structurally keyed magnitude (`r3_alias.rs`, exit 0, no `#![feature]`, no `-Z` flag), so
what is genuinely irreducible is not a const expression. It is one primitive nobody in this panel has named:
the bridge from `Idx<N>` to a type. Every gate-free implementation of that bridge is one impl per width,
which is the enumeration op refused at `127b:36-50`.

Three consequences, in the order they should change what happens next.

**`131` and `132` both say the alternative to `generic_const_args` is "no projection". There is an
alternative and it is the width table op already rejected.** `132:507-509` puts it as "derive it in the type
system, or write it and give up op's ruling", and `131:824-826` as "the alternative is not a cheaper
projection but no projection". Both are wrong, and the correction does not favour the alternative: it names
it, so that op's blocking question becomes a choice between two known things rather than a choice against
nothing. The known thing on the other side is a per-width enumeration, refused for the cap it forces, and
that refusal still holds.

**The purchase is smaller and more precisely located than `132` reports.** `132:339-345` says the feature
buys "one const expression". It buys one **item**: the impl body that maps a magnitude to a rung. Everything
else separates cleanly and compiles without it, including the piece nobody had tested, which is that the
three-parameter surface holds gate-free through a defaulted type parameter (`q10_default_typeparam.rs`, exit
0). Surface arity, derivation-not-declaration, and the projection mechanism are three independent questions,
and only the third is gated.

**A downstream layer that reads the typestate pays nothing, measured.** Op's ruling has a second half about
`hilavitkutin-build` reading the same semantics and typestate (`130b:41-44`), and nobody had priced it. A
consumer that names the projected container as a type, reads the associated consts, and unwraps the raw
container at concrete numerals including the wide rung compiles against a GCA library with **no feature gate
and no flag** (`c_read.rs`, exit 0). The flag reaches consumers that are themselves generic over widths, and
nothing else.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts op's own calls and the intent in the governing position.
The governing calls here are the container ruling (`130b:39-48`), the carry-and-read observation
(`127b:41-50`), the enumeration refusal (`127b:36-50`), D48's surface (`127b:56-59`), the no-ceiling call
(`127b:118-126`), and the convergence pressure (`127b:12-18`). Section 8 marks what I hand back.

**Test gate.** Not run, and I am naming it rather than letting it pass. `126:47-48` ran
`cargo test --offline --workspace` on a tree nothing has moved since and got 672 passed, 0 failed, 9
ignored; `129`, `130`, `131` and `132` each declined to re-run it on op's ruling at `108b:174-181`. My
deliverable touches no crate in that tree. The instrument here was the compiler.

**The toolchain.** `rustc +nightly-2026-05-28 --version --verbose` reports
`1.98.0-nightly (57d06900f 2026-05-27)`, matching the brief.

### The brief's factual claims

*"`132` established, compiled, that the derivation is two steps and only one of them costs anything."*
**Holds.** I rebuilt neither half from scratch but I did re-run the two files the claim rests on.
`lat132/q15_op_dispatch.rs` compiles on the pin with no gates, exit 0, and `lat132/q16_mgca_only.rs`
reproduces rustc naming `generic_const_args` as the requirement. The seam is real. Section 1 says why it is
also, as drawn, a consequence of a premise rather than a fact about the problem.

*"The governing constraint is that a const may be carried and read, never transformed on the way into a
type (`127b:41-50`, from the shipped `Capacity`)."* **Half true, and the half that is false matters.** The
carry-and-read half is exactly right and I extended it: a `type const` carrying a parameter reaches an array
length under `min_generic_const_args` alone (`q4_typeconst_carry.rs`, exit 0), and it chains through a
second projection (`q5_chain.rs`, exit 0). Neither had been run. But the constraint is **not** specific to
consts, and reading it that way is what kept the search inside one encoding for five files. rustc refuses
the same transform over **type** parameters, in as many words (`q12_typeparam_plain.rs`):

```
error: generic parameters may not be used in const operations
9 | pub struct Bad<X: Nat>(pub [u8; <X as Nat>::V]);
  |                                  ^ cannot perform const operation using `X`
  = note: type parameters may not be used in const expressions
```

Section 2 states the constraint in the form that actually holds, and section 3 is what falls out of it.

*"`110:3251` says the container level is derived, never declared as an axis."* **Holds, verbatim**, and I
read the line rather than the citation of it:

```
    type Container;                       // the container level W_C: derived, never declared
                                          // as an axis (1.22), and what Number holds (1.1)
```

*"`generic_const_args` is vetted WATCH; `min_generic_const_args`, `adt_const_params`, `min_specialization`
and the const-traits family are allowed."* Holds per `unstable-features.md` and `128:17-19`. I used mGCA,
GCA and `min_specialization` and nothing else, and section 1.1 reports that `min_specialization` refuses one
step earlier than `131` reasoned.

---

## 1. The categories, enumerated so the next reader does not repeat them

`126:345-357` built the first such table over the width question, and it is the right instrument, so this is
the same instrument turned on step A. Every row is compiled on the pin. The nine marked **new** are
positions no file in this panel has run.

| | The move | Probe | Result |
|---|---|---|---|
| 1 | inherent assoc const path as an array length, `[u8; Self::B]` | `q1` | refused, **new**, with its own diagnostic: "generic `Self` types are currently not permitted in anonymous constants" |
| 2 | the same path written out, `[u8; Rung::<I, F>::B]` | `q2` | refused, **new** |
| 3 | trait assoc const path, `[u8; <Rung<I,F> as Tagged>::B]` | `q2b` | refused, **new** |
| 4 | rustc's own brace repair, `<Rung<{ I }, { F }> as Tagged>::B` | `q3` | refused, **new**, and this is the informative one, section 2 |
| 5 | `type const` carrying a parameter, no arithmetic, under mGCA alone | `q4` | **works**, exit 0, reaches an array length, **new** |
| 6 | `type const` chained through a second projection, still no arithmetic | `q5` | **works**, exit 0, **new** |
| 7 | arithmetic over **type**-parameter projections in a `type const` body | `q6` | refused, **new**, and it closes the type-level escape, section 2 |
| 8 | unary nat read back to a `usize` by recursion in a `type const` | `q7` | refused, **new** |
| 9 | const peeled to a unary nat, `Idx<{N-1}>` | `q7b` | refused twice: "complex const arguments must be placed inside of a `const` block", then `E0119` |
| 10 | `min_specialization` partitioning the ladder by const ranges | `q8`, `q8b` | refused, **new**, for a different reason than `131:869-870` gives, section 1.1 |
| 11 | two blanket impls partitioned by disjoint where-clauses | `q9`, `q9b`, `q9c`, `q9d` | **works**, exit 0, **new**, section 1.2 |
| 12 | defaulted **type** parameter carrying a projection over standalone const args | `q10` | **works**, exit 0, no gates, **new**, section 5 |
| 13 | a width-dependent carrier with no computation, `([u8; I], [u8; F])` | `q11` | **works** and buys nothing, section 1.3 |
| 14 | reading a const off a type parameter as an array length | `q12` | refused, with the sharpest diagnostic in the file |
| 15 | structurally keyed magnitude, ladder by trait resolution | `r1`, `r2`, `r3` | **works**, exit 0, no gates, no flag, section 3 |
| 16 | structural byte count reaching `[u8; B]` at the wide rung | `r6` | refused, section 4 |

Rows 1 to 4, 7 to 10, 14 and 16 are step A refused. Rows 5, 6, 11, 12 and 13 are mechanisms that work and do
not carry step A over the line. Row 15 is the answer and row 16 is its one hole.

Add these to `126`'s nine, `130`'s seven, `129`'s eight and `132`'s three and the panel has now compiled the
same refusal in more than thirty positions. That is not thoroughness any more, it is a signal that the thing
being enumerated is not where the answer lives, which is section 2.

### 1.1 `min_specialization` cannot do it, and not for the reason on record

`131:869-870` says "I reasoned it cannot, because specialization is structural and ranges are not, and I did
not compile the refutation". The refutation compiles and the conclusion holds, but the mechanism refuses one
step earlier than that reasoning reaches. With `#![feature(min_specialization)]` enabled and nothing else
(`q8b_minspec.rs`):

```
error[E0658]: specialization is experimental
 --> q8b_minspec.rs:6:42
  |
6 | impl<const N: usize> Store for Rung<N> { default type T = u64; }
  |                                          ^^^^^^^^^^^^^^^^^^^^^
  = help: add `#![feature(specialization)]` to the crate attributes to enable
```

`min_specialization` admits no defaultable **associated type** at all, so the question of whether a range is
structural never arises. A specialised container ladder needs the full feature, which `unstable-features.md`
forbids, and which this panel's own model-width transfer argument depends on staying forbidden
(`unstable-features.md`, "The forbidden list is verification infrastructure, not only hygiene"). The row is
closed for a stronger reason than the one recorded, and the record should carry the stronger one.

### 1.2 The where-clause partition works, and knowing that it works is the point

This surprised me and I ran it four ways before believing it. Two blanket impls over the same self type,
separated only by disjoint where-clauses, are accepted (`q9_coherence.rs`, exit 0), resolve correctly
(`q9d_resolve.rs`: `Rung<3>` to `u8` and `Rung<40>` to `u64`, both as compile-time type equalities), refuse
an unclassified input with a clean `E0277`, and report overlap the moment one type genuinely satisfies both
classifiers (`q9c_overlap_control.rs`):

```
error[E0119]: conflicting implementations of trait `Store` for type `Rung<3>`
8 | impl<const N: usize> Store for Rung<N> where Rung<N>: Small { type T = u8; }
  | ----------------------------------------------------------- first implementation here
9 | impl<const N: usize> Store for Rung<N> where Rung<N>: Large { type T = u64; }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Rung<3>`
```

So a partitioned ladder is expressible with no gate and no specialization. What is not expressible is
**populating the partition**: getting from a width to its classifier without one impl per width is the same
wall, one level down. I record it in full because a reader who finds the mechanism will believe they have
found the answer, and the ten minutes that costs are better spent here.

### 1.3 There is a gate-free carrier that depends on the width, and it buys nothing

Stated so the irreducibility claim is not made more broadly than it holds.
`Split<const I, const F> { hi: [u8; I], lo: [u8; F] }` compiles with no gates, because both array lengths
are standalone (`q11_split_carrier.rs`, exit 0, every assertion holds):

```rust
const _: () = assert!(size_of::<Split<13, 3>>() == 16);      // 16 BYTES for a 16-bit numeral
const _: () = assert!(size_of::<Split<3, 0>>() == 3);        // 3 bytes for a 3-bit numeral
const _: () = assert!(size_of::<Split<200, 100>>() == 300);  // 300 bytes for a 300-bit numeral
```

Eight times the footprint, on the workload `arvo-toolbox-not-policer.md` names as arvo's reason to exist,
and no machine type is reachable from it, so every operation also pays `132:286-297`'s thirty-three times.
It is a counterexample to "no gate-free width-dependent carrier exists" and to nothing else. The claim that
survives contact is narrower, and section 2 states it.

---

## 2. Why the refusal is universal, stated as a binding-time fact

Thirty positions and one diagnostic is a pattern, and `130:490-493` already drew the right conclusion from
it: the rule is quantified over positions, so enumerating positions cannot find an exception. What it did
not do is say what the rule **is**, and the form it is usually stated in is wrong in a way that hid the
answer for five files.

The form on record is "a const may be carried and read, never transformed on the way into a type"
(`127b:41-50`, `132:331-333`). That is a true description of the const case and a false description of the
rule. Two compiled facts fix it.

**The rule fires on type parameters too.** `q6_typeparam_arith.rs` puts no const parameter anywhere in
scope and computes over two type-parameter projections in a `type const` body:

```
error: generic parameters may not be used in const operations
7 | impl<X: Nat, Y: Nat> Nat for Sum<X, Y> { type const V: usize = const { <X as Nat>::V + <Y as Nat>::V }; }
  |                                                                         ^
  = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

and `q12_typeparam_plain.rs` says it in the compiler's own words, with no gates at all: "type parameters may
not be used in const expressions".

**The rule fires on mention, not on arithmetic.** `q2b` and `q3` are the informative pair. `q3` takes
rustc's own repair suggestion and braces the parameters as standalone arguments inside a nested path, which
is the exact shape the carry-and-read reading says should work:

```
error: generic parameters may not be used in const operations
7 |     type T = [u8; <Rung<{ I }, { F }> as Tagged>::B];
  |                           ^ cannot perform const operation using `I`
  = help: const parameters may only be used as standalone arguments here, i.e. `I`
```

No arithmetic appears in that line. The whole const argument is an anonymous constant, and a generic
parameter mentioned anywhere inside it is a const operation, however standalone it looks locally.

So the rule, in the form that survives every probe in section 1:

> **A generic parameter of any kind may reach type position only as an argument of the item it parameterises.
> It may not be mentioned inside an anonymous constant.** `min_generic_const_args` adds one exception, a
> `type const` item, whose body may be a carry or a chain of carries (`q4`, `q5`, both exit 0) and may not be
> an operation. `generic_const_args` lifts that last restriction and nothing else does.

Read as a binding-time statement this stops being a list of refusals and becomes one design fact.
**Type-level computation in Rust is trait resolution, and trait resolution dispatches on types, not on
values.** A const generic is a value. The language provides exactly two value-to-type functions: impl
matching against literal values, which enumerates the domain, and an expression in type position, which is
gated. There is no third, and there is no position at which a third could hide, which is why thirty probes
found one diagnostic.

That is the honest irreducibility argument, and it is stronger than "I looked in three places". It is also
narrower than it sounds, because it is quantified over **a magnitude that is a value**. Section 3 changes
that quantifier.

Two independent legs then hold up step A, and the first one is op's own, not the toolchain's.

**The design leg.** A container that does not depend on the width is a fixed-width container, and a fixed
width is a ceiling. Op removed the ceiling (`127b:118-126`). So the container must depend on the width, for
a reason that has nothing to do with footprint and would hold if memory were free. `132:393-397` used this
against the fixed carrier specifically; it generalises, and generalised it is the reason step A exists at
all.

**The language leg.** Given that the container depends on the width, and given the rule above, the
dependence is either one impl per width or a const operation on generic parameters. Nothing else is
available.

---

## 3. The construction that has no step A

Section 2's rule is quantified over a magnitude that is a **value**. The way past a universally quantified
refusal is not another position, it is to not need the operation, which `130:497-500` states and which every
working construction in this panel is an instance of. Applied here it gives one move nobody tried: **stop
keying the numeral on a value.**

If the magnitude is a type, choosing a container from it is trait resolution, and trait resolution is what
Rust is for.

### 3.1 The construction, compiled

`r2_structural.rs`, exit 0, no `#![feature]`, no `-Z` flag, `no_std`. A little-endian binary numeral with
three constructors, and the ladder falling out of its structure.

```rust
pub struct Term;                      // 0
pub struct D0<T>(PhantomData<T>);     // 2n
pub struct D1<T>(PhantomData<T>);     // 2n + 1
```

The value is readable in value position, which is `126:190-215`'s door two and costs nothing:

```rust
pub trait Nat { const V: u32; }
impl Nat for Term { const V: u32 = 0; }
impl<T: Nat> Nat for D0<T> { const V: u32 = 2 * T::V; }
impl<T: Nat> Nat for D1<T> { const V: u32 = 2 * T::V + 1; }
```

Addition is a structural carry chain, nine impls plus a three-impl `Inc`. The rung is the binary digit count
of `W - 1`, which separates 8 from 9 where the digit count of `W` alone does not, and which is a **pattern**
rather than a comparison:

```rust
pub trait Len { type L; }                                        // digit count, as a unary tally
impl Len for Term { type L = Z; }
impl<T: Len> Len for D0<T> where T::L: Bump { type L = <T::L as Bump>::O; }   // trailing zeros are not digits
impl<T: Len> Len for D1<T> { type L = S<T::L>; }

impl Rung for L0 { type T = u8; }   /* L1, L2, L3 the same */
impl Rung for L4 { type T = u16; }
impl Rung for L5 { type T = u32; }
impl Rung for L6 { type T = u64; }
impl Rung for L7 { type T = u128; }
impl<T> Rung for S<S<S<S<S<S<S<S<T>>>>>>>> { type T = WideNil; }   // eight digits or more
```

Nine `Rung` impls, and the last is a structural catch-all, so the ladder is **total with no ceiling and no
width enumerated anywhere**. That is `127b:118-126` satisfied by construction rather than by a wide rung
someone remembered to parameterise.

The container then falls out, and these are type equalities rather than assertions about them, so the file
does not build if any rung is wrong:

```rust
pub fn q13_3(x: Fixed<N13, N3, Hot>) -> u16 { x.0 }        // 16 bits  -> u16
pub fn q3_0(x: Fixed<N3, N0, Hot>) -> u8 { x.0 }           // 3 bits   -> u8
pub fn q16_16(x: Fixed<N16, N16, Hot>) -> u32 { x.0 }      // 32 bits  -> u32
pub fn q30_3(x: Fixed<N30, N3, Hot>) -> u64 { x.0 }        // 33 bits  -> u64
pub fn q100_30(x: Fixed<N100, N30, Hot>) -> WideNil { x.0 } // 130 bits -> wide
```

I know they are load-bearing because I got them wrong twice. An off-by-one in `Dec` left a trailing zero
digit and rustc reported `expected u16, found u32` at exactly the site whose rung had shifted. The
construction refuses a wrong ladder at the ladder, not at a const assertion downstream.

### 3.2 D48's surface survives, which is the part I did not expect

The obvious objection is that this costs op's ratified surface, because a consumer would write
`Fixed<D1<D0<D1<D1<Term>>>>, ..>`. It does not have to. A **generic type alias projecting from standalone
const arguments** is ordinary type syntax with no anonymous constant in it, so it compiles gate-free
(`r3_alias.rs`, exit 0):

```rust
pub type UFixed<const I: u32, const F: u32, S> =
    Fixed<<Idx<I> as ToNat>::N, <Idx<F> as ToNat>::N, S>;

pub fn surface(x: UFixed<13, 3, Hot>) -> u16 { x.0 }
```

`UFixed<13, 3, Hot>` is D48's literal arity and D48's literal spelling (`127b:56-59`), and it resolves to
`u16` through the structural ladder. And the law is width-generic over structural coordinates with no gate
and no flag:

```rust
pub fn mul<I, F, J, K, S>(_a: Fixed<I, F, S>, _b: Fixed<J, K, S>)
    -> Fixed<<I as Add<J>>::O, <F as Add<K>>::O, S>
where I: Add<F> + Add<J>, J: Add<K>, F: Add<K>, /* ... */ { todo!() }

pub fn law_site(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> Fixed<N26, N6, Hot> { mul(a, b) }
```

This is the point where `132:252-257`'s macro finding inverts, and the inversion is worth stating because it
is the sharpest thing in this file. `132` argues a token-level mechanism "fails for width-generic code,
because inside `fn mul<const I: u32, ...>` there are no literals for the macro to compute with". True, and
it is a fact about **const-keyed** genericity. Under structural keying, width-generic code is generic over
**types**, needs no literals, and does its arithmetic by trait resolution. Literals appear only at concrete
sites, which is exactly where a token-level mechanism works.

### 3.3 So what is actually irreducible

Four `ToNat` impls in `r3_alias.rs`, written by hand:

```rust
impl ToNat for Idx<0>  { type N = Term; }
impl ToNat for Idx<3>  { type N = N3; }
impl ToNat for Idx<13> { type N = N13; }
impl ToNat for Idx<16> { type N = N16; }
```

That is the whole of step A, and it is the only thing in the construction that does not generalise. Every
route to a total `ToNat` is closed: peeling needs `Idx<{N-1}>` (`q7b`, refused), a catch-all
`impl<const N: u32> ToNat for Idx<N>` has no type to name, specialising past an enumerated set needs the
forbidden feature (`q8b`), and computing the nat in a `type const` is GCA.

**So the irreducible core of step A is the const-to-type bridge, and its only gate-free implementation is
one impl per width.** That is a materially different statement from `132:339-345`'s "one const expression",
and it changes three things: what a future stabilisation would relieve, where a future language feature
would have to land, and what the honest alternative to GCA is, which is the width table rather than nothing.

---

## 4. What the structural encoding costs

Three costs, one of them serious. I am pricing it rather than selling it, because the panel has twice
reported a gate-free answer that was gate-free only because the work had quietly moved somewhere else
(`131:25-27`), and this file would be the third if the prices were not on the page.

**Cost one, compile time, and it is small.** Sixty-four distinct compositions at four-digit widths, each
with a compile-time assertion on the sum and a function whose return type is the projected container
(`r4_scale.rs`, generated, `/usr/bin/time -p`, three runs): **0.30 s cold, then 0.11 s, 0.12 s.** The
surface plus alias plus law file is 0.05 s then 0.04 s. Against the same workload elsewhere in the panel:
`126:229-232`'s value-carrying construction at 0.04 s while doing no container selection, `125:245-250`'s
table at 0.06 s, its use-site realisation at 5.87 s, and `131:421-424`'s full GCA surface at 0.04 s. So the
structural encoding is roughly three times the GCA route and two orders below the realisation route, which
under `arvo-compile-time-last.md` is not a reason to refuse it. Note that this measurement **excludes the
bridge**, since the probe writes its nats directly.

**Cost two, the wide rung cannot reach `[u8; BYTES]`.** The byte count is a type in this encoding, and a
type cannot be an array length (`r6_widelen.rs`):

```
error: generic parameters may not be used in const operations
6 | pub struct Wide<B: Nat>([u8; <B as Nat>::V]);
  = note: type parameters may not be used in const expressions
```

This is section 2's rule arriving from the other side, and it is the exact complement of what the
value-carrying encoding cannot do. Stated as a pair, because it is the cleanest way to see why GCA is
tempting:

| | choose a type from a magnitude | produce a `usize` in type position |
|---|---|---|
| value-carrying nat (`126:190-215`) | no (`126:373-375`) | yes, it is the const |
| structural nat (section 3) | yes (`r2`, exit 0) | no (`r6`) |
| either, under GCA | yes | yes |

The native rungs need the first column and the wide rung needs the second, so **each gate-free encoding is
missing exactly what the other has, and the ladder wants both.** The structural side has a workaround that
is not a stopgap: a `#[repr(C)]` byte cons has the right size and alignment by construction, and its length
**is** readable in value position (`Nat::V` computes freely), so `from_raw_parts` over it hands the loop and
SIMD code a slice with a constant length. I did not build or measure that, and I am marking it unchecked
rather than claiming it.

**Cost three, and this is the serious one: the diagnostic.** A wrong output format prints the digit tower
(`r5_diag.rs`):

```
error[E0308]: mismatched types
    = note: expected struct `Fixed<D0<D0<D0<D0<D1<Term>>>>>, _, _>`
               found struct `Fixed<D0<D1<D0<D1<D1<Term>>>>>, _, _>`
```

A consumer has to decode little-endian binary to learn that they wrote 16 where 26 was produced. Against
that, `131:476-481` reports the GCA route's mismatch as `type mismatch resolving 17 == 16` and
`130:412-426`'s alignment failure as `expected 3, found 8`. Two files of this panel went to op specifically
on diagnostic quality (`130b:70-80`, both adopted), so this is a cost against a thing op has already spent
attention on, and it is not obviously fixable: `#[diagnostic::on_unimplemented]` reaches unimplemented
bounds, not `E0308` on a structural type.

---

## 5. What a consumer writes, under each answer

The three routes, with the surface each one gives, so the trade is legible without reading the sections
above.

| | consumer writes | container | gates and flags | diagnostic |
|---|---|---|---|---|
| GCA projection (`131`, `132`) | `UFixed<13, 3, Warm>` | derived | mGCA + GCA + `-Znext-solver=globally` | numeric, `17 == 16` |
| structural, bridge enumerated (section 3) | `UFixed<13, 3, Warm>` | derived | none | digit tower |
| structural, no bridge | `Fixed<N13, N3, Warm>` with `type N13 = ...` | derived | none | digit tower |
| gate-free fit check (`128:147-173`) | `UFixed<13, 3, u16, Warm>` | **written**, refused at `130b:39-48` | none | fit assertion |

The middle two are one design with the bridge populated differently, and the difference is exactly whether
`Idx<13>` has an impl. That is worth stating plainly because it makes the fork small: the encoding decision
and the bridge decision are separate, and only the bridge carries the enumeration.

**And the surface arity is settled independently of all of it**, which nobody had checked. A defaulted
**type** parameter accepts a projection over standalone const arguments with no gates at all
(`q10_default_typeparam.rs`, exit 0):

```rust
pub struct Fixed<const I: usize, const F: usize, S, C = <S as Store<I, F>>::T> { raw: C, /* .. */ }
pub type UFixed<const I: usize, const F: usize, S> = Fixed<I, F, S>;
pub fn three_params(_: UFixed<13, 3, Hot>) {}
```

Three written parameters, container derived, no feature. The wall is entirely inside `Store`'s impls. So
`130`'s refused `C: Container` parameter and op's ruling are not two shapes but one shape with the default
filled in, and whichever route wins for step A, the surface does not move. It also means a consumer **may**
override the container when they have a reason to, without ever having to, which is closer to
`arvo-toolbox-not-policer.md`'s posture than either a required parameter or no parameter. Whether that door
should be open is op's, section 8.

---

## 6. The `Capacity` precedent does not transfer, and the reason is the finding

Op named `Capacity` as the precedent for finding the workaround that abstracts further (`127b:41-44`), and
`126`, `130`, `131` and `132` have each invoked it. It is worth reading the shipped source rather than the
summary of it, because the move it makes is not the one the summary describes.

```rust
pub trait Capacity {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const CAP: Cap;
}
pub struct Dim<const N: usize>;
impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];        // N standalone, nothing transformed
    const CAP: Cap = cap(N);       // the transform, in value position
}
```

(`arvo-tensor/src/capacity.rs:19-59`.) The crate doc says what it is for in one line: "The capacity is a
TYPE, not a `Cap` const generic" (`capacity.rs:7-9`).

The move is not "carry the const and read it". The move is **inverting the arrow.** `Array<T, C: Capacity>`
has the consumer write `C`, so the thing that would otherwise have to be computed is the thing that is
written, and the derivation runs the easy way, from a written type to a read value. There is no crossing
into type position at all, which is why it needs no feature.

Applied faithfully to the container, the consumer writes the container. That is `128:147-173`'s gate-free
fit check, and it is precisely what op refused at `130b:39-48`.

**So op's two calls are in tension for this one derivation, and the tension is what makes step A hard.** The
precedent asks for the arrow to be inverted; the container ruling forbids the one party who could invert it.
That is not a criticism of either call. `130b:39-48` gives its reason and the reason is arvo's whole
proposition, and `127b:41-44` was offered as a heuristic rather than a theorem. It is a statement about why
four files have now reached for the precedent and come back with something that either needs a feature or
needs an enumeration.

Section 3 is the only escape from the tension I found, and it works by changing what "written" means:
`UFixed<13, 3, Warm>` writes a type that happens to be spelled with two literals, so the consumer writes the
input and the design still derives the container. The arrow is inverted at the coordinates rather than at
the container, which is the position op's ruling leaves open.

---

## 7. The downstream contract, measured

`panels-argue-the-intent-not-the-wording.md` asks that a boundary be designed rather than reported, and
op's ruling has a second half about a downstream layer reading the same semantics and typestate
(`130b:41-44`) that nobody in four files has priced. It costs nothing, and here is the measurement.

`acore.rs` is a GCA library in `131`'s Pattern C shape, built with the gate and the flag. It carries the
typestate a lowering layer would read:

```rust
pub trait Lowering { type Container: Copy; const STORED_WIDTH: u32; const BYTES: usize; }
```

`c_read.rs` is the downstream layer, built with **no feature gate and no `-Z` flag**, exit 0:

```rust
pub fn container_type(x: UFixed<13, 3, Hot>) -> u16 { x.to_raw() }
pub fn wide_container(x: UFixed<200, 100, Hot>) -> [u8; 38] { x.to_raw().0 }
pub const W1: u32   = <UFixed<13, 3, Hot>  as Lowering>::STORED_WIDTH;
pub const B1: usize = <UFixed<13, 3, Hot>  as Lowering>::BYTES;
pub fn takes_container(_: <UFixed<13, 3, Warm> as Lowering>::Container) {}
const _: () = assert!(W1 == 16 && B1 == 2 && W2 == 70);
```

Naming the projected container as a type, unwrapping the raw at both a native and the wide rung, and reading
the derived consts all work with nothing inherited. `c_generic.rs`, the same consumer calling a
width-generic law, fails without the flag and compiles with it, reproducing `131:384-396` a third time.

So the contract the design owes downstream, stated so a build layer can be written against it:

**What it reads.** `Lowering::Container` as a type, plus `STORED_WIDTH`, `BYTES` and the rung as ordinary
associated consts. Ordinary, not `type const`, because a value-position read computes freely and a
`type const` would drag the reader into the mechanism.

**What it costs.** Nothing, at any concrete numeral, including the wide rung, measured above. The flag is
inherited only by code that is itself generic over widths, which a lowering layer keyed on concrete types is
not.

**What the design owes in return.** That every fact a lowering layer needs is reachable as a value-position
const and not only through the projection, so that the mechanism behind the container can change (GCA to
structural, or structural to whatever stabilises) without the downstream contract moving. Section 3's
encoding satisfies this: `Nat::V` is an ordinary associated const and reads identically.

This also narrows the exposure sentence the panel has been carrying. `131:392-396` says the flag reaches
"any consumer doing arithmetic", which is true only of consumers whose own code is width-generic. Concrete
arithmetic is free (`131`'s row two), reads are free (measured here), and what pays is genericity over
widths in the consumer's own source. Whether hilavitkutin and vehje are width-generic in that sense is a
question about those trees, and I did not open them.

---

## 8. What is op's, separately from what I decided

**Mine, and compiled.** That the refusal blocking step A is not about const parameters and not about
arithmetic: it fires on **type** parameters (`q6`, `q12`) and on **mention** inside an anonymous constant
even where rustc's own brace repair has been taken and nothing is computed (`q3`). That a `type const` under
mGCA alone carries a parameter to an array length and chains through a second projection, so carrying works
and only transforming is gated (`q4`, `q5`). That `min_specialization` cannot partition the ladder because
it admits no defaultable associated type at all, one step earlier than `131:869-870` reasoned (`q8b`). That
two blanket impls partitioned by disjoint where-clauses are accepted, resolve, and report overlap correctly,
so a partitioned ladder is expressible and only its classifier is not (`q9`, `q9c`, `q9d`). That a defaulted
**type** parameter carries a projection over standalone const args with no gates, so the three-parameter
surface is independent of the projection mechanism (`q10`). That a width-dependent gate-free carrier exists
and costs eight times the footprint with no machine type reachable (`q11`). That the whole container ladder,
the addition, a width-generic law, and `UFixed<13, 3, Hot>` itself compile gate-free over a structurally
keyed magnitude, with the ladder total and no width enumerated (`r2`, `r3`), at 0.11 s on sixty-four
four-digit compositions (`r4`), with the wide rung's array length refused (`r6`) and the diagnostic degraded
to a digit tower (`r5`). That the only thing left un-generalised in that construction is `Idx<N>` to a type,
and that every route to it is closed gate-free. That a downstream layer reading the typestate at concrete
numerals pays no gate and no flag (`c_read.rs`), while a width-generic call site pays the flag
(`c_generic.rs`).

**Mine, and it is a reading rather than a ruling.** That step A is irreducible **under a const-keyed
magnitude**, and that the quantifiers matter more than the verdict. It is irreducible with respect to: this
pin, `rustc 1.98.0-nightly (57d06900f)`; the feature set `unstable-features.md` permits, with full
`specialization` and `generic_const_exprs` forbidden; a numeral whose magnitude is a const parameter; a
container that is derived rather than written (`130b:39-48`); no ceiling (`127b:118-126`); and no impl per
width (`127b:36-50`). Relax the third and it dissolves. Relax the sixth and it dissolves. Relax neither and
it is one const expression, priced by `131` and `132`.

**His, and it is the one that blocks, now with the alternative named.** Whether the container projection is
worth `-Znext-solver=globally` reaching consumers that write width-generic code. `131:823-827` and
`132:610-615` both hand this over saying there is no alternative. There is one, it is section 3, and its
price is a diagnostic that prints binary digit towers plus a per-width bridge, which is the table op already
refused. My reading, offered as a reading: take the GCA projection, because the enumeration was refused for
a reason that has not changed, and because section 3's diagnostic cost lands on the surface op has twice
spent attention on. But the choice is now between two known things and it should be made as one.

**His, because he has already ruled near it and this is genuinely close to the line.** Whether a
**defaulted** container type parameter is what `130b:39-48` forbids. The ruling is that "the container is
never written by a consumer", and a default is not written while remaining writable. It compiles gate-free
(`q10`), it keeps three written parameters, and it gives a consumer with a real reason an escape hatch that
`arvo-toolbox-not-policer.md` would ordinarily want. I did not treat the ruling as settling it either way,
because reading a ruling more narrowly than it was written is how drift starts.

**His, because it is a fork the panel has not seen.** Whether the numeral's magnitude is a const or a type.
This is not a spelling question, it is the encoding, and it decides whether step A exists. `126` deleted the
tower for good reasons (its arithmetic job was doable more cheaply, and canonicity was later withdrawn at
`130b:11-30`), and neither of those reasons was the container. The tower had a third job nobody had named
and it was deleted before anyone knew what it was for. That does not mean it should come back; it means the
deletion was decided without this input.

**His, because the rule has no answer.** Where `-Znext-solver=globally` sits in `unstable-features.md`.
Raised at `128:287-306`, unsettled through `131:829-832` and `132:617-620`. Section 7 adds one input: the
flag is inherited by consumer code that is itself width-generic, and not by code that merely uses arvo, so
the exposure is a property of the consumer's own source rather than of depending on arvo.

**Owed under the two-expert rule.** I am the second read on `132`'s step A / step B decomposition and I
agree with the seam while correcting where the purchase sits: it is one impl body, not one expression, and
the ladder half is free in more encodings than `132` found. I am the **first** read on everything in
sections 2, 3, 4, 5 and 7, and none of it should enter the canon on one expert's word. The premise a second
read should attack in my file: **that the structural encoding's diagnostic cost is unfixable.** I asserted
it from one `E0308` and a reading of what `#[diagnostic::on_unimplemented]` can reach, and I did not try
a display-side mitigation. If it is fixable, the fork in section 5 changes shape and my reading with it.

---

## 9. What I did not check

- **Whether the wide rung's byte cons recovers native codegen.** Section 4 sketches `#[repr(C)]` plus
  `from_raw_parts` over a value-position length and I neither built nor measured it. If it does not, the
  structural encoding is worse above 128 bits than `132:286-297`'s thirty-three times figure suggests.
- **Whether the structural diagnostic can be improved.** The premise I am handing to a second read. A
  `Display`-side or `#[diagnostic]`-side mitigation may exist and I did not look for one.
- **What the bridge costs when populated.** `r4`'s 0.11 s excludes it. `125:245-250` measured use-site
  realisation at 5.87 s and `126:365-375` attributes that to decimal conversion rather than to the
  parameter kind, so a binary-emitting bridge should be far cheaper. Nobody has measured one.
- **Whether a token-level bridge is acceptable at concrete sites.** Section 3.2 establishes that it would
  work where `132:252-257` says it cannot, but a macro at the surface touches D48 and I did not price it.
- **Whether hilavitkutin or vehje contain width-generic code.** Section 7 narrows the flag's reach to
  consumers that are themselves width-generic and does not say whether the named ones are.
- **The next-solver open bug list**, still unchecked, now at four files: `128:78-81`, `129:548`,
  `131:871-872`, and here.
- **The real law count**, still not in `110`, flagged at `131:663-666` and `132:658`, still open.
