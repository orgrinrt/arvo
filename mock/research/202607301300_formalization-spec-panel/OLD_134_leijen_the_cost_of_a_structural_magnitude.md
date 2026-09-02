# 134. The cost of a structural magnitude, and what the diagnostic actually costs

**Persona:** Daan Leijen, cost-of-a-mechanism lens. Third pass in this panel; file 84 was failure that is not
a range event, file 103 was the platform and the predicate.
**Date:** 2026-08-07
**Position:** second read on `133_amin_is_step_a_irreducible.md`, section 8, on the one premise it hands back:
that the structural encoding's diagnostic cost is unfixable. Reads `133`, `132`, `131`, `130`, op's
checkpoints `130b` and `127b`, `110` where it bears on the container and on the decoder ring, and the earlier
numeral-spelling material at `47`, `48b`, `58`, `61` and `62b` that no file since `126` has cited.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository, `mock/crates` read and never written. Probes at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/lj134/`,
sixteen files plus the generator.

## The verdict

**The diagnostic is fixable, and the premise `133` handed back is false.** The digit tower is an artifact of
the numeral's **base**, not of structural encoding. At base ten the same construction prints
`expected Fixed<N1<N6<End>>, ...>, found Fixed<N2<N6<End>>, ...>`, the decimal digits of 16 and 26 in reading
order, and at three digits `N2<N0<N0<End>>>` against `N3<N0<N0<End>>>` for 200 against 300, with no elision on
the differing coordinate. The whole construction compiles gate-free with the container ladder, carry-threaded
addition and a decimal-to-binary conversion, in 332 impls, at 0.17 s against 0.09 s for the binary encoding
on an identical sixty-four-composition workload, and it has no ceiling: a one-million-bit width resolves in
0.05 s.

**This is larger than the fork I was asked about.** The standing base carries the opposite as settled: "The
decoder ring is a confirmed ceiling, not an open item to keep chasing" (`110:2501`, from `58:658-673`,
repeated at `124:2406`). That paragraph is wrong, and the reason is one sentence in the file it descends
from. `47:417-419` looked for a way to put a decimal value into a numeral's printed name and concluded there
was none, because "a distinct struct per number breaks the arithmetic". A distinct struct per **digit** does
not, and nobody tried it. The panel then built a face layer to route around the ceiling, priced at
`58:655-656` as "doubling relevant trait surface", which `61:490-500` found decays one hop into any generic
operation.

Two smaller results, both compiled and both negative in the useful direction.
`#[diagnostic::do_not_recommend]` on the blanket impl changes the output not at all, byte-identical with and
without (`n3.rs`). And a const-carrying head over the tower produces exactly the GCA route's
`expected 16, found 26`, then fails, because the head cannot survive addition and a mixed pair prints worse
than a uniform tower (`d3_head.rs`).

**The fork itself is drawn on the wrong axes.** The brief's three routes conflate the magnitude's kind with
the surface vehicle and omit the base entirely, and the base is what decides the diagnostic. Separated, there
are four routes, one of which dies on an orphan-rule fact rather than on op's enumeration refusal (`d14`,
`E0117`), and the surface question turns on something the brief states as fact and I cannot support: that op
refused a macro. Sections 6 and 7.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts op's own calls and the intent in the governing position.
The governing calls here are the container ruling (`130b:39-48`), the enumeration refusal (`127b:36-50`),
D48's surface (`127b:56-59`), the no-ceiling call (`127b:118-126`), the both-diagnostics adoption
(`130b:70-80`) and the convergence pressure (`127b:12-18`). Nothing below asks op to reopen any of them.

**Test gate.** Not run, and I name it rather than let it pass. `126:47-48` ran
`cargo test --offline --workspace` on a tree nothing has moved since, 672 passed, 0 failed, 9 ignored;
`129` through `133` each declined to re-run it. My deliverable touches no crate in that tree, and the
instrument here is the compiler. Every construction I report carries a negative control, because a
construction whose assertions cannot fail is not evidence: `n1.rs` breaks a rung and gets
`expected u32, found u64`, `n2.rs` breaks a value assertion and gets `E0080`, `n4.rs` breaks it at a million
bits and gets the same.

**The toolchain.** `rustc +nightly-2026-05-28 --version --verbose` reports
`1.98.0-nightly (57d06900f 2026-05-27)`, matching the brief.

### The brief's factual claims

*"`#[diagnostic::on_unimplemented]` and `#[diagnostic::do_not_recommend]` are stable and the shipped tree
already uses the first (`arvo-strategy/src/container.rs:110-113`)."* **Holds**, read at the line:

```rust
#[diagnostic::on_unimplemented(
    message = "strategy `{Self}` does not provide a container for {N}-bit width",
```

*"`131` found that particular one can never fire."* **Holds** (`131:126-131`): the wide rung is a catch-all,
so the projection is total and there is no width the ladder refuses. `131` marks it a source reading rather
than a compiled result and I did not re-derive it.

*"Step B is gate-free with codegen byte-identical to a native container; step A is the whole cost."*
**Holds** as `132` and `133` report it, and I did not rebuild it. `133`'s own correction stands: what is
irreducible is the const-to-type bridge, one impl per width, not one const expression (`133:387-390`).

*"`133` asserts that the structural encoding's diagnostic cost is unfixable, and says plainly that it
asserted this from one diagnostic."* **Holds, and the self-report is exact.** `133:644-645`: "I asserted it
from one `E0308` and a reading of what `#[diagnostic::on_unimplemented]` can reach, and I did not try a
display-side mitigation." That honesty is what made this file cheap to write, and it is the reason a second
read was worth dispatching.

*"A macro was refused by op earlier as a surface."* **I can find no support for it.** Section 6 gives the
search. This is the brief's own premise and it changes the fork's shape.

---

## 1. What the attributes reach, and what they do not

**`#[diagnostic::on_unimplemented]` fires on `E0277` and nothing else.** It annotates a trait and replaces
the message when a bound on that trait is unsatisfied. An `E0308` between two concrete types has no trait in
it, so there is nothing to annotate. None of `133`'s three quoted diagnostics is an `E0277`, which is why
`133:449-450` is right that the attribute does not reach them.

**Its format arguments substitute generic parameters of the annotated trait, and nothing else**
(`d2_oniu_const.rs`). A const parameter of the trait substitutes as a number, which is why the shipped `{N}`
at `container.rs:110` would have printed a width had it ever fired. A parameter of `Self` does not
substitute, and rustc says so:

```
warning: there is no parameter `N` on trait `ToNat`
  = help: expect either a generic argument name or `{Self}` as format argument
```

`{Self}` and a type parameter of the trait both substitute, printing the type:

```
error[E0277]: no fixed-point numeral of width {N} exists
19 | pub fn bad(_: <Idx<14> as ToNat<Marker>>::N) {}
   = note: Self is `Idx<14>`, the type parameter T is `Marker`
```

The consequence for the fork: the attribute can put a number in a message only where the number is a const
parameter of the failing trait, and under a structural magnitude no such const exists, because the point of
the encoding is that the magnitude is not a value. **The attribute cannot manufacture readability.** What it
can do is carry the decode rule and name the law, which is section 3.3, and that turns out to be worth
having for a different reason than the one it was reached for.

**`#[diagnostic::do_not_recommend]` does nothing here, measured.** Applied to the reflexive blanket impl
`impl<T> WidthIs<T> for T`, the emitted diagnostic is byte-identical with and without it, including the
`help: the trait ... is not implemented for` line it is meant to suppress (`d10_bound_fires.rs` against
`n3.rs`). It suppresses an impl from being **suggested** as a candidate, and rustc is not suggesting one
here; it is reporting the bound directly. Recorded because the brief named the attribute and a later reader
would otherwise assume it was left untried.

---

## 2. Reshaping the error into a bound

The brief's second lever, and the panel has been here. `58:661-668` states it as a general result and
`110:2503-2505` carries it into the standing base: "The one lever that moves it is not a diagnostic
attribute; it is restating the comparison as a bound (`E0277`) rather than an equality."

The lever is real and it has a precondition nobody wrote down. **It works only where the expected width is
fixed by something other than the operation being checked.**

The natural attempt fails, and instructively (`d9_bound.rs`). Give the law an output type parameter and
constrain it:

```rust
pub fn mul2<I, F, J, K, S, OI, OF>(_a: Fixed<I, F, S>, _b: Fixed<J, K, S>) -> Fixed<OI, OF, S>
where <I as Add<J>>::O: WidthIs<OI>, <F as Add<K>>::O: WidthIs<OF>, /* ... */
```

The annotated `E0277` does not appear. What appears is the same `E0308`, because the reflexive impl
`impl<T> WidthIs<T> for T` **resolves the inference variable**: rustc solves
`<M13 as Add<M13>>::O: WidthIs<?OI>` by taking `?OI = M26` before it ever compares against the annotation, so
the bound is satisfied and the return type is what mismatches. A relation that can drive inference will drive
it.

Where the expectation is independent, the lever fires as advertised. A declared accumulator is the canonical
case, which is why `58:664-668`'s witness was a fold:

```rust
pub fn fold<A: Accum, I, F, J, K, S>(_a: Fixed<I, F, S>, _b: Fixed<J, K, S>)
where <I as Add<J>>::O: WidthIs<<A as Accum>::W>, /* ... */
```

`<A as Accum>::W` is pinned by `A`, nothing is left to infer, and the failure is the bound
(`d10_bound_fires.rs`, quoted in full at 3.3).

So the lever's reach is narrower than the standing base implies, and stating the precondition is worth more
than restating the lever. **It covers declared-accumulator positions and does not cover a plain annotated
return**, and the plain annotated return is the common case (section 5).

And the lever does not solve the problem it was invented for. It changes `E0308` into `E0277`, and both print
the same type names. Under binary towers the `E0277` message contains a binary tower. Readability comes from
the base, not from the error class.

---

## 3. Reshaping the type, which is where the answer is

`133:436-450` reports the cost this way: "A consumer has to decode little-endian binary to learn that they
wrote 16 where 26 was produced." Read that with the emphasis moved. The consumer decodes **binary**. Nothing
in the structural encoding requires binary. Binary is there because the addition is cheapest in binary, nine
impls in `133`'s construction, and because every prior attempt in this panel used it.

So I put four candidate magnitude shapes into one file at one mismatch, 26 produced against 16 annotated,
with no arithmetic anywhere so only the printing differs (`d4_print_shapes.rs`, four intended errors).

**Shape one, the shipped a133 encoding, little-endian binary.** The baseline, reproducing `133:439-442`:

```
expected `Fixed<D0<D0<D0<D0<D1<End>>>>>, ..., ...>`, found `Fixed<D0<D1<D0<D1<D1<End>>>>>, ..., ...>`
```

**Shape two, big-endian base ten.** Ten digit structs instead of two, same cons-list shape, same kind of
positional arithmetic:

```
expected `Fixed<N1<N6<End>>, N1<N6<End>>, Hot>`, found `Fixed<N2<N6<End>>, N1<N6<End>>, Hot>`
```

`N1<N6<End>>` is 16 and `N2<N6<End>>` is 26, digits in reading order, both coordinates printed in full.
**Shape four** is the same at three digits, where the binary tower is deepest and rustc's elision bites
hardest:

```
expected `Fixed<N2<N0<N0<End>>>, N2<N0<...>>, ...>`, found `Fixed<N2<N0<N5<End>>>, N2<N0<...>>, ...>`
```

200 against 205. The elision lands on the repeated coordinate, not on the differing one.

**Shape three, the near miss, recorded because it looks like the obvious fix.** Carry the value as a const on
a head over the tower, `W<const V: u32, D>`. rustc descends into the mismatch and reports the smallest
differing subterm, which is the const:

```
expected `16`, found `26`
```

Comparable to the GCA route's `17 == 16` (`131:476-481`), out of a construction with no gates. It does not
survive contact with the arithmetic. A derived coordinate's head would carry `A + B`, a const operation on
generic parameters, which is `133:239-244`'s rule. So `Add` on heads drops the head, the two sides of a
mismatch then have **different shapes**, and rustc cannot descend at all (`d3_head.rs`):

```
expected `Fixed<W<16, D0<D0<D0<...>>>>, ..., ...>`, found `Fixed<D0<D1<D0<D1<D1<...>>>>>, ..., ...>`
```

Worse than either uniform encoding. The head is available on written coordinates and unavailable on derived
ones, and a design cannot use it on one side only.

### 3.1 The base-ten construction, whole, compiled

`d6_dec_full.rs`, exit 0, `no_std`, no `#![feature]`, no `-Z` flag, 332 impls, generated by `gen_dec.py`.
Ten digit structs stored big-endian, so a magnitude reads outermost digit first. Carry-threaded addition, one
impl per digit pair per carry-in; an increment for the carry tail; a reversal so the stored form is big-endian
while the arithmetic runs little-endian; a decimal-to-binary conversion; then `133`'s ladder unchanged on the
binary form.

```rust
pub type M13 = N1<N3<End>>;
pub type M200 = N2<N0<N0<End>>>;
const _: () = assert!(<M13 as Val>::V == 13 && <M200 as Val>::V == 200);
const _: () = assert!(<<M13 as Add<M3>>::O as Val>::V == 16);
const _: () = assert!(<<M100 as Add<M30>>::O as Val>::V == 130);
const _: () = assert!(<<M200 as Add<M100>>::O as Val>::V == 300);
```

The conversion is the one piece that is new rather than transposed from `133`. From the little-endian form,
`bin(N_d<T>) = lit(d) + 10 * bin(T)`, and ten times a binary tower is eight times plus two times, both shifts
the tower already expresses:

```rust
impl<T: ToBin> ToBin for N3<T> /* ... */
{ type B = <Lit3 as BAdd<<Z0<Z0<Z0<<T as ToBin>::B>>> as BAdd<Z0<<T as ToBin>::B>>>::O>>::O; }
```

There is one definition of what the digits mean, and the value read goes through the same reversal the
arithmetic does, so the two representations cannot drift apart into separate answers about the same numeral.
That is the thing to hold onto if this is built: a second representation is acceptable only while it has one
definition.

The ladder falls out as type equalities, so the file does not build if any rung is wrong:

```rust
pub fn q13_3(x: Fixed<M13, M3, Hot>) -> u16 { x.0 }      // 16 bits  -> u16
pub fn q3_0(x: Fixed<M3, M0, Hot>) -> u8 { x.0 }         // 3 bits   -> u8
pub fn q30_3(x: Fixed<M30, M3, Hot>) -> u64 { x.0 }      // 33 bits  -> u64
pub fn q100_0(x: Fixed<M100, M0, Hot>) -> u128 { x.0 }   // 100 bits -> u128
pub fn q100_30(x: Fixed<M100, M30, Hot>) -> WideNil { x.0 }  // 130 bits -> wide
```

### 3.2 The diagnostic, at the sites a consumer actually hits

`d7_dec_diag.rs`, three shapes, all three readable:

```
    | pub fn wrong(a: Fixed<M13, M3, Hot>, b: Fixed<M13, M3, Hot>) -> Fixed<M16, M6, Hot> { mul(a, b) }
    = note: expected struct `Fixed<N1<N6<End>>, _, _>`
               found struct `Fixed<N2<N6<End>>, _, _>`

    | pub fn wrong_wide(a: Fixed<M200, M0, Hot>, b: Fixed<M100, M0, Hot>) -> Fixed<M200, M0, Hot>
    = note: expected struct `Fixed<N2<N0<N0<End>>>, _, _>`
               found struct `Fixed<N3<N0<N0<End>>>, _, _>`

    | pub fn wrong_arg(x: Fixed<M13, M3, Hot>) -> Fixed<M16, M3, Hot> { x }
    = note: expected struct `Fixed<N1<N6<End>>, _, _>`
               found struct `Fixed<N1<N3<End>>, _, _>`
```

16 against 26, 200 against 300, 16 against 13. The third is the common case, a mistyped width with no
arithmetic in sight, and it reads without a decoder.

### 3.3 The best achievable output, which is section 2's lever on top of this

Where the expected width is fixed by something other than the operation being checked, the failure is the
bound, and the attribute fires on top of legible coordinates (`d10_bound_fires.rs`):

```
error[E0277]: law `mul_widths`: this product's integer coordinate is `N2<N6<End>>`, the annotation names `N1<N6<End>>`
    | pub fn fold_bad(a: Fixed<M13, M3, Hot>, b: Fixed<M13, M3, Hot>) { fold::<Acc16, _, _, _, _, _>(a, b) }
    |                                                                   ---------------------------- ^ the width arithmetic of `mul` disagrees with this annotation
help: the trait `WidthIs<N1<N6<End>>>` is not implemented for `N2<N6<End>>`
    = note: a magnitude is its decimal digits, outermost first: `N1<N6<End>>` is 16, `N2<N0<N0<End>>>` is 200
```

The law is named, both coordinates are legible, and the decode rule travels with the error rather than living
in a document the consumer does not have open. Op adopted named-item laws for the diagnostic at `130b:70-80`;
this is what that adoption looks like once the coordinates are readable, and it is where the attribute earns
its keep after section 1 established it cannot manufacture a number.

---

## 4. What the readable encoding costs

Pricing it rather than selling it, on the same discipline `133:396-398` states, because a panel that reports
a gate-free answer without its price has reported half of it.

**Cost one, impl count: 32 to 332.** `133`'s binary construction is 32 impls; the base-ten one is 332,
generated. The bulk is the digit-pair table, one impl per pair per carry-in, which is 200 of them. It is a
table over **digits**, ten of them, not over widths, so it has no cap and no policy in it, and it is exactly
the kind of enumeration `127b:36-50` does not reach: the domain is the base, and the base is a design
decision rather than a guess about what widths a consumer will want.

**Cost two, compile time, measured on identical workloads.** Sixty-four distinct compositions, widths 20 to
111 against 14 to 91, each with a value assertion on both coordinates, an assertion on the sum, and a
function whose return type is the projected container. Same generator, same widths, two encodings, three runs
each after a warm run:

| encoding | file | impls | time |
|---|---|---|---|
| binary, `133`'s | `d13_scale64_bin.rs` | 32 | 0.09 s |
| base ten | `d8_scale64.rs` | 332 | 0.17 s |

So base ten costs 1.9x the binary structural encoding at the same workload. `133:405` cites `131:421-424`'s
full GCA surface at 0.04 s, which I did not re-measure and which is a different file rather than the same
workload, so read the GCA comparison as indicative. Under `arvo-compile-time-last.md` none of these is a
reason to refuse anything: compile time is the bucket poured into, and 0.08 s is what this buys.

**Cost three, print length, which is negative.** A binary tower is `floor(log2 W) + 1` levels deep and a
decimal one is `floor(log10 W) + 1`, so **base ten is strictly shorter as well as readable**, and the gap
widens with the width. At 1000000 the decimal form is seven levels and the binary form is twenty. There is no
tradeoff here to weigh; the readable encoding is also the compact one.

**Cost four, none: the ceiling is still absent.** `127b:118-126` is op's no-ceiling call, and `133`'s ladder
satisfies it by a structural catch-all. Base ten does not disturb that, checked at four magnitudes with the
value assertion load-bearing at each (`d12_big_*.rs`, and `n4.rs` as the control):

| width | time |
|---|---|
| 1000 | 0.06 s |
| 10000 | 0.06 s |
| 65536 | 0.05 s |
| 1000000 | 0.05 s |

**Cost five, unchanged and still the binding one.** The wide rung cannot reach `[u8; BYTES]`, because the
byte count is a type in this encoding and a type cannot be an array length (`133:410-418`). The base does not
touch this. `133` sketches a `#[repr(C)]` byte cons with `from_raw_parts` over a value-position length and
marks it explicitly unbuilt (`133:430-434`, `133:651-653`). **With the diagnostic fixed, this is now the
largest unpriced cost on the structural route**, and it is the one I would send the next dispatch after.

---

## 5. Pricing what remains, weighted by the common case

The brief asks for the residual cost by error kind, weighted toward the common case, which it names
correctly: a consumer mistypes a width far more often than they violate a law.

| error | GCA route | base ten, structural | binary, structural |
|---|---|---|---|
| mistyped width at a written position | `expected 13, found 16` | `expected N1<N3<End>>, found N1<N6<End>>` | `expected D1<D0<D1<D1<Term>>>>, found D0<D0<D0<D0<D1<Term>>>>>` |
| law violated at an annotated return | `type mismatch resolving 17 == 16` | `expected Fixed<N2<N6<End>>, ..>, found Fixed<N1<N6<End>>, ..>` | fully expanded, both towers, elided at depth |
| law violated at a declared accumulator | numeric, through the same bound | named law plus both coordinates plus the decode note (3.3) | named law plus two towers |
| width with no numeral declared | does not arise | `no numeral is declared for width Idx<14>` (`d14_use.rs`) | same |

**The residual against GCA is one decode step, and it is a lexical one.** A reader of `N1<N3<End>>` has to
know that the digit structs are digits and that the outermost is most significant. They do not have to
compute anything. A reader of `D1<D0<D1<D1<Term>>>>` has to evaluate a positional binary sum in their head,
in the reverse of the reading direction. That is the difference between a convention and an arithmetic
exercise, and it is the whole of what `47:417` was trying to buy.

Two things sharpen the weighting further.

**On the common case the bound lever does not help and the base does.** Section 2's precondition rules the
lever out at a plain annotated position, which is exactly where a mistyped width lands. So a design that
takes the lever and keeps binary has fixed the rarer error and left the common one. That is the wrong way
round, and it is the shape the standing base currently recommends (`110:2503-2505`).

**The best diagnostic in this whole exercise belongs to the route that dies for another reason.** An
undeclared width under a bridge is an `E0277` with a numeric `Self` and an actionable note:

```
error[E0277]: no numeral is declared for width d14_lib::Idx<14>
   = note: declare it once with `nat!(14)` in the crate that owns the numeral
help: the trait `d14_lib::ToNat` is implemented for `d14_lib::Idx<13>`
```

Worth recording because it is the strongest output available anywhere in the structural family, and section 6
is why it cannot be reached without either a cap or a change of surface.

---

## 6. The surface question, and whether a macro was ever refused

### 6.1 What the record says

I grepped every op checkpoint in the panel for a macro refusal and found none. What I found instead:

- **`48b:53-60`** ratifies the opposite: "The digit-emitting macro, not the bounded table, and the reasoning
  is not close. A 1024-row table is a stored copy of a computable function, and its bound is a hardcoded
  threshold of exactly the kind arvo refuses to ship anywhere else."
- **`48b:57-60`** states the intent in one line: "a consumer writes any number as a literal, unbounded range,
  emitted constructors, zero table."
- **`62b:107-118`** closes the vehicle: a declarative macro "cannot start, because a decimal literal is one
  atomic token and no fragment specifier, restringify trick or const-generic escape reaches its digits, all
  compiled", so the proc-macro vehicle is adopted with zero external dependencies, on the notko `#[profile]`
  precedent, std at compile time only.
- **`79b:25`**, an op checkpoint, asks for macro-driven volume in the parity suites: "likely best we run them
  via macros to actually make it maintainable too at the volume we need here". Op reaching for a macro, not
  refusing one.

**`48b` and `62b` are persona checkpoints, not op's**, so on the provenance ladder they are agent output and
presumed wrong where they conflict with op. They do not conflict with anything of op's here; they simply are
not ratification, and the brief should not be read as though the macro were settled either way.

What op did refuse, both at `127b`, are two different things: **the dual spelling** ("Op called it convoluted
and file 126 agreed independently: a second surface existing to escape the first one's ceiling is an
admission the ceiling should not be there", `127b:57-59`) and **the capped width table** (`127b:36-50`). A
digit-emitting macro is neither. It is one surface, not two, and it emits only what is written, which is
`127b:47-49` in op's own words: "only used widths realise on const time, but resolve just the same. Which
would theoretically allow any arbitrary widths to be defined and still work, without us choosing any actually
legitimately arbitrary caps or ranges for valid widths."

### 6.2 A macro invocation stands in type position, compiled

`d11_typepos.rs`. The digit decomposition is the macro's job and `61` already settled that it needs a proc
macro; the question here is only whether the position accepts an expansion at all. It does:

```rust
pub fn typepos(x: ufx!([1 3], [3], Hot)) -> u16 { x.0 }
pub fn typepos_struct() -> Option<ufx!([2 0 0], [0], Hot)> { None }
```

Both compile. The third function in the file mistypes the width on purpose and gets the base-ten `E0308`,
so the diagnostic under a macro surface is section 3.2's diagnostic unchanged.

### 6.3 The orphan rule is what kills the bridge, not the enumeration preference

This is the load-bearing fact of the section, and it is stated nowhere in the brief.

A bridge impl `impl ToNat for Idx<14>` names a foreign trait and a foreign type from a consumer's crate, so a
consumer cannot declare the widths they use (`d14_consumer.rs`):

```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
5 | impl ToNat for Idx<14> { type N = N1<N4<End>>; }
  |                `d14_lib::Idx` is not defined in the current crate
```

So the bridge must be populated inside arvo, and arvo cannot know which widths a consumer will write, so
populating it means generating a range, and a generated range is a cap. **The middle route is not refused
because op dislikes enumerations. It is unreachable, by a coherence rule, given that the consumer must be
able to write any width.** `62b:110-111` reached the same wall from the sealing side and phrased it as the
bridge being "the only shape"; this is the same fact from the surface side.

The consequence is that keeping D48's literal spelling `UFixed<13, 3, Warm>` **and** having no cap requires
the magnitude to be a const, which is the GCA route. Under a structural magnitude, one of the two moves.

### 6.4 The four routes, on the axes that actually separate them

The brief's table has three rows on two conflated axes. Separated into magnitude kind, surface vehicle and
base, there are four:

| | consumer writes | gates and flags | enumeration | diagnostic |
|---|---|---|---|---|
| A. const magnitude, GCA projection | `UFixed<13, 3, Warm>` | mGCA + GCA + `-Znext-solver=globally` | none | `17 == 16` |
| B. structural, const-keyed bridge | `UFixed<13, 3, Warm>` | none | forced by `E0117`, with a cap | base ten, plus the best undeclared-width message |
| C. structural, type-position macro | `ufixed!(13, 3, Warm)` | none | none | base ten |
| D. structural, declared aliases | `nat!(N13 = 13);` then `UFixed<N13, N3, Warm>` | none | none, one declaration per width per crate | base ten |

The base is orthogonal to all four and decides the last column for B, C and D. That is the axis the brief
omits and it is the one the question was about.

---

## 7. The premises this brief takes for granted

Four, in descending order of how much they would have cost if left alone.

**One. That the decoder ring is a ceiling.** Not in this brief, but in the standing base it descends from
(`110:2501`, `124:2406`, from `58:658-673`), and every file since has inherited it. It is false, and the
error is traceable to a single sentence at `47:417-419` that considered a struct per **number** and never a
struct per **digit**. Everything downstream of that sentence, including the face layer priced at
`58:655-656` as doubling the relevant trait surface, was built to route around a wall that a change of base
removes. `48b:67-70` even flagged the debt in the right words, "someone spends the twenty minutes before we
accept that the message stays in binary", and nobody spent them for eighty-six files.

**Two. That the fork's cost columns are gates and diagnostic.** `133` reports three costs and the brief's
table carries one. The compile-time cost is small and I have measured it (section 4). The wide rung's
`[u8; BYTES]` hole is not small, is explicitly unbuilt (`133:651-653`), and with the diagnostic fixed it
becomes the binding cost on the structural route. A fork table that omits the only unchecked cost is a table
that will produce a decision on the wrong grounds.

**Three. That a macro was refused.** Section 6.1. If the underlying memory is of `127b:57-59`, that refusal
was of a **second** surface existing beside a first, and a macro that replaces the surface is not that.

**Four. That the middle route's enumeration is a preference.** Section 6.3. It is a coherence rule, and the
distinction matters because a preference can be revisited and `E0117` cannot.

And one that is not a premise but a framing worth naming. **The fork asks op to choose an encoding, and under
routes C and D the encoding is not a surface decision at all.** If the consumer never names a coordinate
type, then GCA against structural is an implementation choice arvo can change later without moving anything
a consumer wrote, which is exactly the property `133:568-573` asks the design to owe downstream. Under route
A or B the choice is load-bearing forever. That does not tell op which to take; it says the size of the
decision differs by route, and a decision that can be revised is worth something against one that cannot.

---

## 8. What is op's, separately from what I decided

**Mine, and compiled.** That `#[diagnostic::on_unimplemented]` substitutes only generic parameters of the
annotated trait, so a const parameter of the trait prints as a number and a parameter of `Self` does not
substitute at all (`d2`), which means the attribute cannot manufacture a readable magnitude under a
structural encoding. That `#[diagnostic::do_not_recommend]` changes this output not at all, byte-identical
with and without (`n3` against `d10`). That restating the comparison as a bound fires only where the expected
width is fixed independently of the operation, because a reflexive relation resolves the inference variable
first (`d9` against `d10`), so the lever misses the common case. That a const-carrying head over the tower
makes rustc print `expected 16, found 26`, and that the head cannot survive addition, and that a mixed pair
prints worse than either uniform encoding (`d3`, `d4`). That base ten prints the digits of the magnitude in
reading order at one, two and three digits with no elision on the differing coordinate (`d4`, `d7`). That the
full base-ten construction, addition, reversal, decimal-to-binary conversion and `133`'s ladder, compiles
gate-free in 332 impls with load-bearing rung equalities and value assertions (`d6`, controls `n1` and `n2`).
That it costs 1.9x the binary encoding on an identical sixty-four-composition workload, 0.17 s against 0.09 s
(`d8` against `d13`). That it has no ceiling through one million bits at 0.05 s, control included (`d12`,
`n4`). That a macro invocation stands in type position and yields the same base-ten diagnostic (`d11`). That
a consumer cannot populate a const-keyed bridge for its own widths, `E0117`, so the bridge must be a capped
table inside arvo (`d14`).

**Mine, and it is a reading rather than a ruling.** That the standing base's decoder-ring paragraph should be
corrected rather than annotated, because it is not a nuance but a wrong result, and because a face layer
exists downstream of it whose justification I have not re-examined. I did not open the face layer's current
status and I am not proposing anything about it; I am saying the ground under it moved.

**His, and it is the one that blocks.** Whether D48's literal spelling `UFixed<13, 3, Warm>` survives, or the
surface becomes `ufixed!(13, 3, Warm)` or a declared alias. Section 6.3 is why this is now the real fork
rather than the diagnostic: with the diagnostic fixed, the structural family's remaining cost against GCA is
the surface, and `E0117` says the literal spelling and the absent cap cannot both be had structurally. D48 is
his (`127b:56-59`) and I am not reading it more narrowly than it was written.

**His, because it is a fork the panel has not seen.** The numeral's **base**. It is not a spelling question
and it is not the encoding question `133:627-632` handed over; it is a third axis, orthogonal to both, and it
alone decides what a consumer reads when they get it wrong. My reading, offered as a reading: if any
structural route is taken, take it at base ten, because the 0.08 s and the 300 generated impls buy the entire
gap between a convention and an arithmetic exercise, and because the encoding is strictly shorter as well.

**His, because it changes what a prior decision rests on.** `48b:53-60`'s digit-emitting macro and
`62b:107-118`'s proc-macro vehicle were adopted by the panel, not by him, and `126` deleted the tower they
sat on for reasons that were not the container (`133:628-632`) and not the diagnostic. Three deletions have
now been found to have removed something nobody had priced. Whether that material comes back is his.

**His, because the rule has no answer.** Where `-Znext-solver=globally` sits in `unstable-features.md`,
carried unresolved from `128:287-306` through `131:829-832`, `132:617-620` and `133:634-637`. I add nothing
to it beyond noting that routes B, C and D do not need it at all.

**Owed under the two-expert rule.** I am the second read on `133`'s handed-back premise and I overturn it. I
am the **first** read on everything in sections 3 through 7, including the base-ten construction, the
`E0117` wall, the bound lever's precondition and the correction to `110:2501`. None of it should enter the
canon on one expert's word. The premise a second read should attack in my file: **that base ten's arithmetic
is sound at every width, not only at the ones I asserted.** I checked sixty-four compositions plus four large
magnitudes with value assertions, which is a sample and not the matrix, and the digit-pair table is generated
so a generator bug would be uniform and invisible to a sample. The whole-matrix const-assert obligation
`48b:67-69` placed on the earlier macro applies to this table too, and I did not discharge it.

---

## 9. What I did not check

- **The whole matrix of the digit table.** Sixty-four compositions and four large magnitudes, generated from
  the same script that generated the impls, which is the failure mode `48b:67-69` already named. A whole-matrix
  assertion over digit pairs and carries is owed before any of this is believed.
- **Whether the wide rung's byte cons recovers native codegen.** Unchanged from `133:651-653` and now the
  largest unpriced cost on the structural route.
- **What the bridge or the macro costs when populated.** `d8`'s 0.17 s excludes both, exactly as `133:656-658`
  excluded the bridge.
- **Subtraction, comparison and the rest of the magnitude's surface at base ten.** I built addition because
  the law needs it. Whatever else the design asks of a magnitude has to be built in base ten too, and the
  digit table's shape suggests each such operation is another hundred-impl table.
- **The face layer's current status.** Section 8 says the ground under it moved; I did not open it.
- **Whether a proc macro is accepted in type position.** `d11` establishes the position accepts a declarative
  expansion; `61` and `62b` establish the decomposition needs a proc macro. I did not build the two together.
- **The next-solver open bug list**, now unchecked at five files: `128:78-81`, `129:548`, `131:871-872`,
  `133:663-664`, and here.
