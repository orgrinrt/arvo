# 134. The cost of a structural magnitude, and what the diagnostic actually costs

**Persona:** Daan Leijen, cost-of-a-mechanism lens. Third pass in this panel; file 84 was failure that is not
a range event, file 103 was the platform and the predicate.
**Date:** 2026-08-07
**Position:** second read on `133_amin_is_step_a_irreducible.md`, section 8, on the one premise it hands back:
that the structural encoding's diagnostic cost is unfixable. Reads `133`, `132`, `131`, `130`, op's
checkpoints `130b` and `127b`, `110` where it bears on the container and on the decoder ring, and the earlier
numeral-spelling material at `47`, `48b`, `58`, `61` and `62b` that nobody in the last six files has cited.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository, `mock/crates` read and never written. Probes at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/lj134/`,
fourteen files.

## The verdict

**The diagnostic is fixable, and the premise `133` handed back is false.** The digit tower is an artifact of
the numeral's **base**, not of structural encoding. At base ten the same construction prints
`expected Fixed<N1<N6<End>>, ...>, found Fixed<N2<N6<End>>, ...>`, where the decimal digits of 16 and 26 are
present in reading order, and at three digits it prints `N2<N0<N0<End>>>` against `N3<N0<N0<End>>>` for 200
against 300 with no elision on the differing coordinate. The whole construction compiles gate-free with the
container ladder, the carry-threaded addition and a decimal-to-binary conversion, in 332 impls, at 0.17 s on
the same sixty-four-composition workload where `133`'s binary version measures 0.12 s.

**This is larger than the fork I was asked about.** The standing base carries the opposite as settled: "The
decoder ring is a confirmed ceiling, not an open item to keep chasing" (`110:2501`, from `58:658-673`). That
paragraph is wrong, and the reason it is wrong is one sentence in the file it descends from. `47:417-419`
looked for a way to put a decimal value into a numeral's printed name and concluded there was none, because
"a distinct struct per number breaks the arithmetic". A distinct struct per **digit** does not, and nobody
tried it. The panel then built a whole face layer to route around the ceiling, priced at `58:655-656` as
"doubling relevant trait surface", and `61:490-500` found it decays one hop into any generic operation.

Two smaller results, both compiled and both negative in the useful direction. `#[diagnostic::do_not_recommend]`
on the blanket impl changes the output not at all, byte-identical with and without (`n3.rs`). And a
const-carrying head over the tower produces exactly the GCA route's `expected 16, found 26`, then fails,
because the head cannot survive addition and a mixed pair prints worse than a uniform tower (`d3_head.rs`).

**Three of the brief's own premises do not hold**, and section 7 states them. The one that matters most:
"A macro was refused by op earlier as a surface" appears nowhere in the record. What op refused was the dual
spelling (`127b:57-59`) and the capped width table (`127b:36-50`). A digit-emitting macro was **adopted** by
this panel at `48b:57-67` and closed as the only available vehicle at `62b:107-118`.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts op's own calls and the intent in the governing position.
The governing calls here are the container ruling (`130b:39-48`), the enumeration refusal (`127b:36-50`),
D48's surface (`127b:56-59`), the no-ceiling call (`127b:118-126`), the both-diagnostics adoption
(`130b:70-80`) and the convergence pressure (`127b:12-18`). Nothing I propose asks op to reopen any of them;
section 8 marks what is his.

**Test gate.** Not run, and I name it rather than let it pass. `126:47-48` ran `cargo test --offline
--workspace` on a tree nothing has moved since, 672 passed, 0 failed, 9 ignored; `129` through `133` each
declined to re-run it. My deliverable touches no crate in that tree, and the instrument here is the
compiler.

**The toolchain.** `rustc +nightly-2026-05-28 --version --verbose` reports
`1.98.0-nightly (57d06900f 2026-05-27)`, matching the brief.

### The brief's factual claims

*"`#[diagnostic::on_unimplemented]` and `#[diagnostic::do_not_recommend]` are stable and the shipped tree
already uses the first (`arvo-strategy/src/container.rs:110-113`)."* **Holds**, read at the line:

```rust
#[diagnostic::on_unimplemented(
    message = "strategy `{Self}` does not provide a container for {N}-bit width",
```

*"`131` found that particular one can never fire."* **Holds** (`131:126-131`), and the reason is worth
carrying: the wide rung is a catch-all, so the projection is total and there is no width the ladder refuses.
`131` marks it a source reading rather than a compiled result and I did not re-derive it.

*"Step B is gate-free with codegen byte-identical to a native container; step A is the whole cost."*
**Holds** as `132` and `133` report it, and I did not rebuild it. `133`'s own correction stands: what is
irreducible is the const-to-type bridge, one impl per width, not one const expression (`133:387-390`).

*"`133` asserts that the structural encoding's diagnostic cost is unfixable, and says plainly that it
asserted this from one diagnostic."* **Holds, and the self-report is exact.** `133:644-645`: "I asserted it
from one `E0308` and a reading of what `#[diagnostic::on_unimplemented]` can reach, and I did not try a
display-side mitigation." That honesty is what made this file cheap to write.

*"A macro was refused by op earlier as a surface."* **I can find no support for it**, and section 6 gives
the search. This is the brief's own premise and it is the one that changes the fork's shape.

---

## 1. What the attributes reach, and what they do not

Two facts, both compiled, and the second is the one that decides how much the attribute route can carry.

**`#[diagnostic::on_unimplemented]` fires on `E0277` and nothing else.** It is an annotation on a trait, and
it replaces the message when a bound on that trait is unsatisfied. An `E0308` between two concrete types is
a different object with no trait in it, so there is nothing to annotate. Neither of `133`'s three quoted
diagnostics is an `E0277`, which is why `133:449-450` is right that the attribute does not reach them.

**Its format arguments substitute generic parameters of the annotated trait, and nothing else**
(`d2_oniu_const.rs`). A const parameter of the trait substitutes as a number, which is why the shipped
`{N}` at `container.rs:110` would have printed a width had it ever fired. A parameter of `Self` does not
substitute at all, and rustc says so:

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
parameter of the failing trait, and under a structural magnitude there is no such const, because the whole
point of the encoding is that the magnitude is not a value. So the attribute cannot manufacture readability.
**What it can do is carry the decode rule and name the law**, which is section 3.3.

**`#[diagnostic::do_not_recommend]` does nothing here, measured.** Applied to the reflexive blanket impl
`impl<T> WidthIs<T> for T`, the emitted diagnostic is byte-identical with and without it, including the
`help: the trait ... is not implemented for` line it is meant to suppress (`d10_bound_fires.rs` against
`n3.rs`). It suppresses an impl from being **suggested** as a candidate, and rustc is not suggesting it here;
it is reporting the bound directly. Recorded as a negative because the brief named the attribute and a reader
would otherwise assume it was left untried.

---

## 2. Reshaping the error into a bound

The brief's second lever, and the panel has been here before. `58:661-668` states it as a general result and
`110:2503-2505` carries it into the standing base: "The one lever that moves it is not a diagnostic
attribute; it is restating the comparison as a bound (`E0277`) rather than an equality."

The lever is real, and it has a precondition nobody wrote down. **It works only where the expected width is
fixed by something other than the operation being checked.**

The natural attempt fails, and the failure is instructive (`d9_bound.rs`). Give the law an output type
parameter and constrain it:

```rust
pub fn mul2<I, F, J, K, S, OI, OF>(_a: Fixed<I, F, S>, _b: Fixed<J, K, S>) -> Fixed<OI, OF, S>
where <I as Add<J>>::O: WidthIs<OI>, <F as Add<K>>::O: WidthIs<OF>, /* ... */
```

and the annotated `E0277` does not appear. What appears is the same `E0308` as before, because the reflexive
impl `impl<T> WidthIs<T> for T` **resolves the inference variable**: rustc solves `<M13 as Add<M13>>::O:
WidthIs<?OI>` by taking `?OI = M26` before it ever compares against the annotation, so the bound is satisfied
and the return type is what mismatches. A relation that can drive inference will drive it.

Where the expectation is independent, the lever fires exactly as advertised. A declared accumulator is the
canonical case, which is why `58:664-668`'s witness was a fold:

```rust
pub fn fold<A: Accum, I, F, J, K, S>(_a: Fixed<I, F, S>, _b: Fixed<J, K, S>)
where <I as Add<J>>::O: WidthIs<<A as Accum>::W>, /* ... */
```

`<A as Accum>::W` is pinned by `A`, so nothing is left to infer, and the failure is the bound
(`d10_bound_fires.rs`, quoted in full at 3.3).

So the lever's reach is narrower than the standing base implies, and stating the precondition is worth more
than restating the lever. **It covers declared-accumulator positions and does not cover a plain annotated
return.** That matters because the plain annotated return is the common case, which is section 5.

And the lever does not solve the problem it was invented for. It changes `E0308` into `E0277`, and both print
the same type names. Under binary towers the `E0277` message contains a binary tower. The readability comes
from the base, not from the error class.

---

## 3. Reshaping the type, which is where the answer is

`133:436-450` reports the cost this way: "A consumer has to decode little-endian binary to learn that they
wrote 16 where 26 was produced." Read that sentence with the emphasis moved. The consumer decodes **binary**.
Nothing in the structural encoding requires binary. Binary is there because the addition is cheapest in
binary, nine impls in `133`'s construction, and because every prior attempt in this panel used it.

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
a head over the tower, `W<const V: u32, D>`. rustc then descends into the mismatch and reports the smallest
differing subterm, which is the const:

```
expected `16`, found `26`
```

That is comparable to the GCA route's `17 == 16` (`131:476-481`), out of a construction with no gates. It
does not survive contact with the arithmetic. A derived coordinate's head would carry `A + B`, a const
operation on generic parameters, which is `133:239-244`'s rule. So `Add` on heads drops the head, the two
sides of a mismatch then have **different shapes**, and rustc cannot descend at all (`d3_head.rs`):

```
expected `Fixed<W<16, D0<D0<D0<...>>>>, ..., ...>`, found `Fixed<D0<D1<D0<D1<D1<...>>>>>, ..., ...>`
```

Worse than either uniform encoding. The head is available on written coordinates and unavailable on derived
ones, and a design cannot use it on one side only.

### 3.1 The base-ten construction, whole, compiled

`d6_dec_full.rs`, exit 0, `no_std`, no `#![feature]`, no `-Z` flag, 332 impls, generated by `gen_dec.py`.
Ten digit structs stored big-endian, so a magnitude reads outermost digit first. Carry-threaded addition, one
impl per digit pair per carry-in; an increment for the carry tail; a reversal so the stored form is big-endian
while the arithmetic runs little-endian; a decimal-to-binary conversion; and then `133`'s ladder unchanged on
the binary form.

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

The ladder falls out as type equalities, so the file does not build if any rung is wrong:

```rust
pub fn q13_3(x: Fixed<M13, M3, Hot>) -> u16 { x.0 }      // 16 bits  -> u16
pub fn q3_0(x: Fixed<M3, M0, Hot>) -> u8 { x.0 }         // 3 bits   -> u8
pub fn q30_3(x: Fixed<M30, M3, Hot>) -> u64 { x.0 }      // 33 bits  -> u64
pub fn q100_0(x: Fixed<M100, M0, Hot>) -> u128 { x.0 }   // 100 bits -> u128
pub fn q100_30(x: Fixed<M100, M30, Hot>) -> WideNil { x.0 }  // 130 bits -> wide
```

Both halves are load-bearing, which I checked rather than assumed. Changing `q30_3`'s return to `u32` gives
`expected u32, found u64` at that site (`n1.rs`), and changing the 26 assertion to 27 gives
`E0080: evaluation panicked: assertion failed` (`n2.rs`). A construction whose assertions cannot fail is not
evidence; this one's can.

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
bound, and the attribute then fires on top of legible coordinates (`d10_bound_fires.rs`):

```
error[E0277]: law `mul_widths`: this product's integer coordinate is `N2<N6<End>>`, the annotation names `N1<N6<End>>`
    | pub fn fold_bad(a: Fixed<M13, M3, Hot>, b: Fixed<M13, M3, Hot>) { fold::<Acc16, _, _, _, _, _>(a, b) }
    |                                                                   ---------------------------- ^ the width arithmetic of `mul` disagrees with this annotation
help: the trait `WidthIs<N1<N6<End>>>` is not implemented for `N2<N6<End>>`
    = note: a magnitude is its decimal digits, outermost first: `N1<N6<End>>` is 16, `N2<N0<N0<End>>>` is 200
```

The law is named, both coordinates are legible, and the decode rule travels with the error rather than living
in a document the consumer does not have open. Op adopted named-item laws for the diagnostic at `130b:70-80`;
this is what that adoption looks like once the coordinates are readable.

## 4. What the readable encoding costs

## 5. Pricing what remains, weighted by the common case

## 6. The surface question, and whether a macro was ever refused

## 7. The premises this brief takes for granted

## 8. What is op's, separately from what I decided

## 9. What I did not check
