# 125. Does the surface need a const at all: the binding-time question under the width table

**Persona:** Tiark Rompf, staging and binding-time lens. Second pass in this panel; file 122 was the first.
**Date:** 2026-08-06
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository. `mock/crates` untouched, `mock/design_rounds/` untouched.

Op read file 123's pricing of the width table and pushed back on the mechanism rather than on the number:
too arbitrary, too fragile, redundant, too explicit, and why did we end up there. The dispatch turns that
into a sharper question than op asked, and the sharper question is the right one. Files 119 and 123 refuted
three ways of escaping from a const parameter to a type. **All three refutations take the const parameter as
given.** Nobody compiled the surface without one.

I did, in four spellings, at five call-site positions each, and the answer has two halves that point in
opposite directions.

**The const parameter is not necessary.** A width can be a type parameter, every call site compiles, and a
consumer writes a width of 1000 against a table holding 31 rows. That half of op's instinct is right and it
is compiled below at `c_C.rs`.

**The table does not go away when the const parameter does**, and this is the part the framing in the
dispatch misses. The table is not only an escape from a value to a type. It is where the conversion from
decimal notation to a numeral is **performed once instead of at every use site**, and that is a binding-time
decision independent of which kind of parameter the surface takes. Remove the const and the same conversion
reappears, at the consumer, at 64 compositions in 5.87 seconds instead of 0.06.

So the two questions the panel has been treating as one come apart cleanly, and each has its own answer:

- **What kind of parameter does the surface take?** Either works. The type parameter removes the ceiling.
- **Where is decimal notation converted to a numeral?** Once in the declaring crate, or every time at the
  consumer. The table is the first answer and it is the right one.

One further thing turned up that nobody was looking for, and it is the strongest support op's word
"fragile" has. **The table's stated emission mechanism and the table that was measured are not the same
artifact.** `110:3633` says it is emitted by `macro_rules!` and not by a build script. `123:247` prices it
at 0.94 s. I reproduce 0.93 s, from a table a Python script wrote. The `macro_rules!` version of the same
table costs **33.4 seconds**, 36 times more, because `macro_rules!` cannot count and so cannot emit a
literal index. That defect is in the incumbent as landed, it is independent of everything else in this
file, and it has to be resolved whichever surface op picks.

---

## 0. The gates, and the brief's claims checked before reasoning from them

**Canon gate: passed, and the check that mattered is not the one the dispatch expected.** The governing
material is op's own: `13c`'s standard at `110:66-71`, the end-state criterion at `110:403-406`, the
tiebreaker at `16d:14-15`, and the pricing licence at `110:505`. Nothing here proposes anything those
forbid. The check worth reporting is different: **the const parameter on the consumer surface is not an
unexamined panel assumption, it is op's own ratified call.** `110:3482-3485`, verbatim, citing
`inherited:1894-1899`, D48, op, 2026-07-29:

> **D48 and D31 are satisfied without special handling because an alias preserves a spelling exactly**:
> `UFixed<13, 3, Warm>`, `Uint<13>` and `Bits<13, Hot>` each still read as themselves, width stays a const
> parameter publicly, and a migration changing the spelling would charge every call site for an internal
> restructuring

That is a ratified decision with a stated reason, and the reason is migration cost rather than mechanism.
It is not ambiguous and it is not stale in the ordinary way, but op is the one reopening it, which
`panels-argue-the-intent-not-the-wording.md` explicitly permits: op's calls are the only final ones and
even those go stale. So I proceed, and I flag it up front because **any recommendation that drops the const
spelling is asking op to overturn his own call from nine days ago**, and he should be told that in those
words rather than discovering it later. My recommendation does not ask him to.

**Test gate: run, and its result is the same one four members before me have reported.**
`cargo test --workspace` from the tree, 2026-08-06: **155 binaries, 672 passed, 0 failed, 9 ignored**,
reproducing `118:47-49` and `119:25-27`. I did not re-audit the bodies, for the reason `119:28-33` gives
and op ruled at `108b:174-181`: the collected tautologies are an implementation-phase checklist, and a
sixth report of the same three findings is what that ruling exists to stop. The suite covers a tree the
canon replaces, so it is the weakest instrument available here. The instrument that measured something
this pass was the compiler, fifty-one times.

**Three of the dispatch's factual claims, checked.**

*"Roughly 7000 lines"* for `110`: **6954**, close enough that nothing turns on it.

*"`0 ..= 4095` in the current proposal"*: **holds**, `123:246` names 4095 as the operating point, and
`123:263-266` correctly leaves the number itself to op.

*"Emitted by `macro_rules!` in the crate declaring the tower"*: **this is what the design says
(`110:3633`, `119:177`) and it is not what was measured.** Section 6. This is the one place the brief
passes on a claim that does not survive a compile, and it is not the brief's fault; it is repeating what
two panel files state.

---

## 1. What the question actually is

The dispatch frames it as an escape problem: a value has to become a type, three escapes are shut, so a
table. That framing is accurate about files 119 and 123 and it is not the deepest reading available.

Restate it as a binding-time question and the shape changes. There are two computations between what a
consumer writes and what the tower consumes:

1. **Decimal to numeral.** `13` becomes `Pz<I<O<I<H>>>>`. A total function, ten symbols of input alphabet,
   trivially computable, and computable at more than one stage.
2. **The two widths' sum.** `13` and `3` become the precision 16. Type-level arithmetic in the tower,
   already built, already total, 21 impls.

The second is settled and nobody is arguing about it. **Everything in dispute is where the first one runs.**

Three stages are available and each is a real design:

- **Stage one, in the crate that declares the tower.** Every admitted width is converted once, at that
  crate's compile, and the result is stored as an impl (the table) or as an alias (a name family). The
  consumer's cost is a lookup.
- **Stage two, at the consumer's call site.** The conversion is a macro over the digits, expanding into the
  tower's own arithmetic. Nothing is stored anywhere; every use site re-derives.
- **Stage zero, at the author's keyboard.** The consumer writes the numeral, or writes something isomorphic
  to it, and no conversion happens at all. Binary transcription is this stage.

The table is a stage-one artifact. That is the whole of what it is, and it is why it has a range: a
stage-one computation over an infinite domain is not a thing you can emit. **An enumeration is what a
stage-one answer to a total function looks like when the domain is unbounded and you truncate it.**

Once it is put that way, op's "arbitrary" stops being a complaint about the number and becomes a correct
observation about the stage. Section 9 takes that up.

---

## 2. The surface spellings, by category

Going wide on the category before going deep, as instructed. A surface spelling is answered by two
independent choices, and the panel has been reading them as one:

**Choice one, the parameter's kind.** Const argument, or type argument. Two options and no third; Rust has
no other kind of generic argument, and lifetime is not a candidate.

**Choice two, the notation the consumer writes.** Decimal literal, a name from a family, a macro over digit
tokens, a macro over binary digits, or the numeral itself. Five, and they compose with either kind of
parameter except that a decimal literal cannot be a type argument and a numeral cannot be a const argument.

The cross product yields the spellings worth compiling. Named for the rest of this file:

| | notation | parameter | table needed | ceiling |
|---|---|---|---|---|
| **A** | decimal literal | const | one row per admitted width | hard |
| **B** | call-site macro over literals | const, behind the macro | same table as A | hard |
| **C** | alias name (`W13`) | type | one alias per named width | soft |
| **D** | macro over decimal digits (`w!(1 3)`) | type | ten digit rows | none |
| **E** | macro over binary digits (`b!(1 1 0 1)`) | type | none at all | none |
| **F** | the numeral, written out | type | none | none |

**G, the per-composition alias family** (`UFixed13x3<Warm>`), is dismissed by arithmetic rather than by
compiling: at 4096 admitted widths it is 16.7 million aliases, and even at 256 it is 65536. It is listed
because the dispatch asked for the category sweep and because its failure is instructive: it is the
spelling that tries to make the *pair* stage-one rather than each width, and the pair's domain is the
square. Nothing else in this file has that problem.

**Everything from C down is one parameter kind and five notations over it.** That is the decomposition the
panel did not have, and it is what lets section 5 give an answer that is not a coin flip.

---

## 3. The call sites, compiled, five positions per spelling

The dispatch asks for the bound and the `where` clause first, on the ground that a macro spelling is most
likely to break there. **It does not break there.** I expected it to and it does not.

All four of A, B, C and D compile at all five positions. `c_A.rs`, `c_B.rs`, `c_C.rs`, `c_D.rs`, exit 0
each. The `where` clause and the bound, spelled under B, which is the one I thought would fail:

```rust
fn bound<S: Sink<ufixed!(13, 3, Warm)>>(_s: S) {}
fn whereclause<S>(_s: S) where S: Sink<ufixed!(13, 3, Warm)> {}
```

Exit 0. A `macro_rules!` invocation is accepted in type position inside a generic argument, inside a
bound, and inside a `where` clause, which is more than I would have claimed without running it.

**The four spellings produce the identical type, and this is the result that makes the rest a surface
question rather than a semantics question.** Three coercions with no conversion anywhere, `c_B2.rs`:

```rust
fn a(x: UFixed<13, 3, Warm>)        -> ufixed!(13, 3, Warm)      { x }
fn b(x: ufixed!(13, 3, Warm))       -> UFixedT<W13, W3, Warm>    { x }
fn c(x: UFixedT<W13, W3, Warm>)     -> w_ufixed!([1 3], [3])     { x }
```

Exit 0. A value written under any one of the four spellings is a value under all four. **Nothing
underneath moves when the surface spelling changes**, so this decision is reversible in a way almost
nothing else in this design is, and that is worth knowing before weighing it.

**One live cost of spelling A reproduced on the way**, and it is `122:325-345`'s finding rather than mine.
Writing the public alias exactly as `110:3492` spells it:

```
error[E0747]: type provided when a constant was expected
  --> arvo.rs:39:35
   |
39 |     Number<FixedNumeral<Sum<NatOf<I>, NatOf<F>>, NonNegative>, S>;
   |                                   ^
help: if this generic argument was intended as a const parameter, surround it with braces
```

The const parameter `I` collides with the carrier constructor `I<P>`, and by `119:164-167`'s own placement
ruling the two are in one scope. Spelling C has no such collision because a type parameter named `Iw`
cannot be confused with anything. This is small and it is a second-order tell: **the const spelling has to
be defended against the tower's own vocabulary, and the type spelling does not.**

---

## 4. Is the const parameter necessary? No, and here is what that buys

`c_C.rs`, exit 0, against a declaring crate holding a **31-row** table:

```rust
fn beyond(_x: UFixedT<W1000, W30, Warm>) {}
struct HeldBig { f: UFixedT<W40, W30, Warm> }
const _: () = assert!(<Sum<W1000, W30> as Nat>::VAL == 1030);
```

A width of 1000 and a total of 1030, from a crate whose table stops at 31. **The table's range and what a
consumer can write are unrelated quantities under a type parameter.** That is the whole of the finding and
it is one compile.

It is worth being exact about why, because the reason is not that a type parameter can do more. The
reason is that under a type parameter **there is no conversion to perform**, so there is nothing for the
table to be the answer to. `W1000` is the numeral. `w!(1 0 0 0)` is the numeral. The tower consumes them
directly and the arithmetic is the arithmetic it already has.

Two consequences follow and both are compiled below:

- Files 119 and 123's ceiling analysis is **conditional on the const spelling**, not on the design. `123:38`
  says the per-width ceiling "is not a policy arvo adopted; it is the shape of the only mechanism
  available". That sentence is true given a const parameter and false without one, and 123 had no reason
  to know that because the dispatch that produced it did not ask.
- The whole of `123`'s section 6, the two options for bounding the total, becomes moot under a type
  parameter, exactly as this dispatch predicted. There is no range to be inside or outside of.

**And the arithmetic is total over the reachable range, which I checked rather than assumed**, because
`123:270-273` leaves that as an item it created: `<Sum<W63, W63>>::VAL == 126`, `<Sum<W1000, W30>>::VAL ==
1030`, `<w!(6 5 5 3 5)>::VAL == 65535`, `<w!(1 2 3 4 5 6)>::VAL == 123456`, exit 0 each, with the negative
control firing (`error[E0080]: evaluation panicked: assertion failed: <Sum<W13, W3> as Nat>::VAL == 17`).

---

## 5. The table does not go away, and this is the finding

Here is where I part company with the dispatch's framing. Its closing sentence for the type-parameter
route is "no table, no admitted range, no ceiling, no enumeration, and the whole of `123` becomes moot".
The first, third and fifth of those are right. **The second is not, and the fourth is only right if the
consumer gives up decimal.**

Measured, 64 distinct compositions in a consumer crate, warm, two runs each, spread under 10 ms except
where shown:

| consumer spelling | widths near 10 | widths near 1000 |
|---|---|---|
| bare link, no compositions | 0.03 s | 0.03 s |
| **A**, const literal over a 4096-row table | 0.05 s | 0.06 s, 0.10 s |
| **D**, `w!` over decimal digits, no table | 0.10 s | **5.87 s, 5.80 s** |
| **E**, `b!` over binary digits, no table | 0.05 s | 0.07 s |

And the same conversion measured alone, one width, one crate:

| width | `w!` cost |
|---|---|
| 13 | 0.02 s |
| 4095 | 0.13 s |
| 65535 | 1.46 s |
| 123456 | 18.49 s |

**Read the two tables together and the shape is unmistakable.** Spelling D is not paying for the type
parameter. It is paying for decimal, at every use site, and the cost is roughly ten times per additional
digit because each digit multiplies the accumulated numeral by ten through the tower's own addition.
Spelling E, which is the same type parameter with a notation that needs no arithmetic at all, costs what
spelling A costs, to within noise, at every size.

So the honest decomposition, and I would put it in the canon in these words:

> **The parameter's kind costs nothing. The notation costs everything.** A width as a type parameter
> compiles at the same price as a width as a const parameter (0.07 s against 0.06 s for 64 compositions at
> four-digit widths). What costs is converting decimal to a numeral, and the only question the table
> answers is where that conversion runs. In the declaring crate it is 0.93 s once. At the consumer it is
> 0.09 s per four-digit composition, in every crate, every build.

**That is the binding-time answer and it is the reason I do not recommend deleting the table.** A table is
a staged computation: the information (which decimal spellings exist) is available in the declaring crate,
it is cheapest there, and the result is reusable by every consumer forever. Moving it to the consumer is
the classic binding-time error, running at stage two what was known at stage one, and the measurement
above is what that error costs when you make it.

**What the type parameter genuinely buys is not the deletion of the table. It is the demotion of the
table from a mechanism to a cache.** Under a const parameter the table is load-bearing: no row, no type,
refusal. Under a type parameter the table is a fast path with a correct fallback, and a missing row costs
a consumer 0.09 seconds rather than a compile error. Same artifact, entirely different standing, and the
difference is exactly what op's word "arbitrary" is reaching for.

---

## 6. The incumbent's emission mechanism does not survive a compile

Independent of everything above, and it has to be fixed under any surface.

`110:3633-3638` states, as landed design:

> **It is emitted by `macro_rules!` in that crate and not by a build script.** [...] A table of literal
> impls over a fixed integer range is `macro_rules!` work at the crate root, and reaching for `build.rs` or
> `OUT_DIR` would be arvo acquiring exactly the harness the design refuses.

`119:176-179` is where that came from and it is stated with the same confidence. **Neither compiled it.**

**The good half first, because it is a positive result and it should be credited.** A `macro_rules!` table
is buildable, and the way it is buildable is more interesting than the claim it vindicates. `macro_rules!`
cannot count, so it cannot write `Idx<13>`. What it can do is recurse **structurally on the numeral**,
doubling at each level, and let the index be computed from the numeral it is keying:

```rust
macro_rules! row {
    ($n:ty) => {
        impl AdmittedWidth for Idx<{ <$n as Nat>::VAL as u16 }> { type Out = $n; }
    };
}
macro_rules! emit {
    ( [] ; $($n:ty),* ) => { $( row!(Pz<$n>); )* };
    ( [ $_x:tt $($d:tt)* ] ; $($n:ty),* ) => {
        $( row!(Pz<$n>); )*
        emit!( [ $($d)* ] ; $( O<$n>, I<$n> ),* );
    };
}
impl AdmittedWidth for Idx<0> { type Out = Z; }
emit!( [x x x x] ; H );   // widths 1 ..= 31
```

Exit 0, with `<<Idx<13> as AdmittedWidth>::Out as Nat>::VAL == 13` and three siblings asserting through it.
A const argument may be an anonymous const block computing from a type, which is the door that makes this
work and which I had not expected to be open.

**This shape carries a verification consequence worth more than the mechanism.** `119:206-217` proposes
emitting a per-row agreement assertion so that the table's claim lands in bin one rather than as a thousand
`unargued` claims. Under the generator above **the per-row assertion is unnecessary, because the row's key
is computed from the row's value.** A miskeyed row is not expressible: `Idx<{<$n as Nat>::VAL}>` and
`type Out = $n` name the same `$n`. What remains checkable is that the enumeration has no gaps and no
duplicates, and duplicates are `E0119` by construction. **That is one law about a generator instead of
4096 claims about rows**, and it is the better answer to the same question 119 was asking.

**The bad half.** That generator costs, warm, two runs:

| rows | `macro_rules!`, index computed | generated source, index literal |
|---|---|---|
| 1 ..= 255 | 0.17 s, 0.17 s | 0.04 s, 0.04 s |
| 1 ..= 1023 | 1.98 s, 1.95 s | 0.13 s, 0.12 s |
| 1 ..= 4095 | **33.38 s, 33.56 s** | 0.93 s, 0.95 s |

The right-hand column reproduces `123:247`'s 0.94 s to within 0.02 s, from a table a Python script wrote.
**So the figure the design is priced on was measured on an artifact the design says it does not use.**

The cause is isolated rather than guessed. Two script-written 4096-row tables, identical except for the
index form:

| index form | cost |
|---|---|
| `Idx<13>`, a literal | 0.93 s |
| `Idx<{ 13 }>`, a const block round a literal | 1.52 s |
| `Idx<{ <Pz<I<O<I<H>>>> as Nat>::VAL as u16 }>`, computed | 33.60 s |

The const block is nearly free. **Evaluating the numeral per row during coherence is the 22x**, and it is
exactly what `macro_rules!` is forced to emit, because the literal form requires counting and
`macro_rules!` cannot count.

**So the incumbent has three exits and op should be shown all three rather than one.**

1. **Pay 33.4 s.** Inside `110:505`'s licence verbatim ("Compile time is nothing. That can be literal
   minutes for all we care") and paid once by one crate. Honest, and it makes `123`'s pricing paragraph
   wrong by 36x, so `123:246-252`'s canon sentence has to be rewritten either way.
2. **Ship the table as checked-in generated source**, 4096 written-out lines, regenerated by a script that
   is not part of the build. This is not `build.rs` and not `OUT_DIR`, so `110:3740`'s prohibition is not
   literally violated, but `110:3633`'s sentence is false as written and must be replaced with a statement
   of what actually produces the file.
3. **Shrink the table** until the `macro_rules!` cost is nothing. At 255 rows it is 0.17 s and the whole
   question evaporates. This is only available if something else covers the widths above the range, which
   is what section 10 recommends.

I have no preference between 1 and 2 as such. I flag that **`110:3633` is a claim about a mechanism, in
the canon, that was never built**, which is the same class of defect as `110:3756-3760`'s ratified
"forced by the language" that turned out false twice over, and which `119:132-136` cites as the reason to
compile forcing claims. The panel caught that one. It made the same mistake three files later.

---

## 7. Diagnostics, since a spelling with unreadable errors is not cheaper

**Spelling A, a width above the range.** Fully gated, `#[diagnostic::on_unimplemented]` reaches it, and
every token reads as a number:

```
error[E0277]: width `Idx<200>` is outside the widths arvo admits
4 | pub fn over(_x: UFixed<200, 3, Warm>) {}
  |                 ^^^^^^^^^^^^^^^^^^^^ the trait `AdmittedWidth` is not implemented for `Idx<200>`
  = note: a written width must lie in 0 ..= 31
  = help: the following other types implement trait `AdmittedWidth`:
            Idx<0>  Idx<10>  Idx<11> ... and 24 others
```

Reproduces `123:120-127`. **The reason it is this clean is worth naming because it is not obvious and it
counts in the table's favour:** the failing obligation's head is `Idx<200>: AdmittedWidth`, `AdmittedWidth`
has no blanket impl, so the failure is the head and the attribute lands on it. **The enumeration is what
gives the design a single named trait whose non-satisfaction is precisely the consumer's mistake.**

**Spelling C, a literal where a type goes.** Terse and correct:

```
error[E0747]: constant provided when a type was expected
4 | pub fn oops(_x: UFixedT<13, 3, Warm>) {}
  |                         ^^
```

**Spelling C, a non-width type.** Raw, this is the worst diagnostic in the file, because a type alias
cannot gate its own arguments and the failure surfaces from wherever the tower first needs the bound:

```
error[E0277]: the trait bound `u8: NatAdd<Pz<I<H>>>` is not satisfied
help: the following other types implement trait `NatAdd<Rhs>`
   --> tower.rs:133:1
133 | impl NatAdd<Z> for Z {
...  [four impls of the tower's internal arithmetic, printed in full]
```

I tried to gate it twice and both attempts failed. A `WrittenWidth` projection in the alias: not reported.
A `WidthPair<Rhs>` gate carrying the whole sum: not reported. In both cases rustc reports the blanket
impl's failed premise, not the gate's head.

**It is recoverable, with two attributes and one cost worth stating.** `#[diagnostic::do_not_recommend]` on
the gate's blanket impl plus `#[diagnostic::on_unimplemented]` on the tower's own `NatAdd`:

```
error[E0277]: `u8` is not a width arvo can write
5 | pub fn oops(_x: UF<u8, W3, Warm>) {}
  |                 ^^^^^^^^^^^^^^^^ the trait `WidthPair<Pz<I<H>>>` is not implemented for `u8`
  = note: a width is written `w!(1 3)` for 13, or a `W*` alias
```

Clean, and one numeral still leaks in the label. **The cost is that the message sits on `NatAdd`, which is
the tower's general arithmetic and fails for reasons that are not widths**, so the attribute will misfire
somewhere. That is a real cost and it is the price of not having a dedicated gate trait. The table is a
dedicated gate trait, which is the second time in this file that the enumeration turns out to be paying
for something other than the escape.

---

## 8. Pricing, both sides

**For the consumer.** Section 5's table is the whole of it and the summary is short. A and E cost the same
and both are free. D costs 0.09 s per four-digit composition and that is the only genuinely disqualifying
number in this file, at four digits and above. C with a named alias costs what A costs.

**For the implementation, the declaring crate.** Section 6's table, plus artifact size, which `123:103` and
`123:249-251` both state as zero and which is zero on the consumer side and not zero on the declaring side:

| table rows | declaring crate rlib | symbols |
|---|---|---|
| 0 | 7,536 bytes | 0 |
| 256 | 117,872 bytes | 0 |
| 4096 | 2,036,952 bytes | 0 |

`123:249-251`'s "no per-width symbol, no runtime table, no relocation" is **exactly true**, `nm` finds
zero. What is new is that the declaring crate's artifact grows 270x, from 7.5 KB to 2.0 MB, to hold an
enumeration, and that **consumer rlibs do not inherit it** (64 compositions: 55,744 bytes against 8,272
bare). So the metadata cost is real, bounded, confined to one crate, and it is the most concrete number
available for op's word "redundant".

**Where each spelling is emitted, and what a downstream crate may do.** `119:150-160`'s `E0117` result
governs all of them and I did not re-run it: `Idx` and `AdmittedWidth` are both the tower's, so a
downstream crate cannot add a row under any spelling. **Under A that is a hard boundary** and a consumer
needing width 5000 has no recourse but a patch to arvo. **Under C it is not a boundary at all**, because a
downstream crate can declare its own aliases (`pub type W5000 = ...;`) with no orphan-rule involvement
whatever, since an alias is not an impl. That asymmetry is the single largest difference between the two
parameter kinds and it does not appear anywhere in files 119 or 123.

**The downstream contract**, since `16c:31-53` asks each member for one. Unchanged from `119:180-190` and
re-verified: nothing downstream sees any of this. No symbol, no runtime table, no relocation, resolved
entirely by the trait solver before monomorphisation. What arvo needs back from a build layer or a code
generator is nothing. The only downstream-visible consequence is the diagnostic in section 7, which is a
front-end artifact.

---

## 9. Op's four words, one at a time

Taken literally, as instructed, against the mechanism as specified. Two land, one lands with a
qualification that changes what to do about it, and one does not land where op aimed it but lands
somewhere else.

**"Arbitrary": lands, and 123 says so in its own voice.** `123:246-247`, in the paragraph it proposes for
the canon: the range "is set where the table stops being free rather than where a consumer stops being
reasonable". A ceiling on a numeric type derived from a compile-time budget is arbitrary in the exact
sense op means. `123` handles it by being honest about it in the canon, which is the right instinct and an
incomplete answer, because **the honesty does not stop the number from being load-bearing.** Section 10's
recommendation is what makes it stop.

**"Fragile": lands, and harder than op knew.** Not for the reason he would have guessed (an enumeration
with a bad row) since section 6 shows the generator makes a miskeyed row inexpressible. It lands because
**the mechanism as landed in the canon was never built**, its stated emission costs 36x its measured
emission, and the resolution changes either a design sentence or a performance claim. A design paragraph
that names an artifact nobody compiled is fragile in the way that matters, which is that a reader who
implements it gets a different system from the one that was priced.

**"Redundant": lands, with a qualification that matters.** 4096 impls express a function that
`123:161-172` already showed is computable structurally from **five impls**. The table is the one place in
this design where a total function is written as an enumeration, and by the design's own standard at
`110:66-71` ("representative of the mathematics", "the structure the design names should be the structure
the mathematics has") an enumeration of a computable function is the wrong structure. **The qualification:
the redundancy is forced by the const spelling and only by it**, and `119:56-121`'s three refutations are
sound (I re-ran route one: `complex const arguments must be placed inside of a const block` plus
`E0119: conflicting implementations of trait ToNat for type Idx<0>`, both reproduce). So op is right that
it is redundant and wrong that it is avoidable while keeping his own D48 spelling. Both halves are true at
once and the recommendation is built on that.

**"Too explicit": does not land where op aimed it, and lands elsewhere.** On the consumer surface it is
false: the consumer writes `UFixed<13, 3, Warm>`, sees no table, links no table cost, and meets the
enumeration only in the error message for the one mistake the enumeration exists to catch, where it is
`123:120-127`'s clean number-shaped refusal. Against `110:403-406`'s own criterion, invisible for the most
part to downstream consumers, **the table passes**. Where it lands is on the declaring crate's artifact:
4096 explicit impls and 2.0 MB of metadata to say a thing that five impls say. That is explicitness in the
implementation, not on the surface, and it is the same observation as "redundant" seen from a different
side.

**Blanket verdict, since the dispatch asks for one and warns against both blanket answers:** two and a
half of four land, the half that does not is the one about the consumer, and none of the four is a reason
to delete the table. They are together a reason to stop the table being the only thing there is.

---

## 10. The recommendation

Keep the const spelling. Add the type spelling beside it. Shrink the number. Fix the emission sentence.

Concretely, four changes, one of which is one line:

**One. `UFixed<const I, const F, S>` stays exactly as D48 and D31 ratified it.** Op's own call, nine days
old, with a stated reason (`110:3485`: a migration changing the spelling "would charge every call site for
an internal restructuring"), and `16d:14-15`'s tiebreaker points the same way. Every call site in the
workspace keeps working. This recommendation asks op to overturn nothing.

**Two. A second alias, one line, taking the widths as types.** Both exist in one crate today and a value
crosses between them with no conversion (`c_B2.rs`, exit 0):

```rust
pub type UFixed  <const I: u16, const F: u16, S> = /* through the table */;
pub type UFixedAt<Iw, Fw, S>                     = /* directly */;
```

This is what removes the ceiling. A consumer above the range writes `UFixedAt<w!(1 0 0 0), w!(3), Warm>`
and it compiles today against a 255-row crate.

**Three. Set the range where the `macro_rules!` generator is free, not where the script-written one is.**
255 rows, 0.17 s, 185 KB. The number stops being load-bearing the moment step two exists, because it is no
longer a ceiling on what is expressible; it is the range over which the convenient spelling works.
`123`'s 4095 was derived to be generous because it had to be a ceiling. It does not have to be one.

**Four. The over-range diagnostic names the escape.** Compiled, `diagRec.rs`:

```
error[E0277]: width `Idx<1000>` is outside the widths arvo admits
6 | pub fn oops(_x: UFixed<1000, 3, Warm>) {}
  = note: the convenient spelling admits 0 ..= 255
  = note: for a wider width write the numeral form: `UFixedAt<w!(1 0 0 0), w!(3), Warm>`
```

The refusal stops being a refusal and becomes a redirect. That is what answers "arbitrary" in the place a
consumer actually meets it.

**What this costs.** One alias declaration, ten digit aliases, one `w!` macro of six rules, and a rewrite
of `123:246-252`'s canon paragraph, which has to be rewritten anyway because of section 6. **What it
buys:** no ceiling, a table that is a cache rather than a mechanism, a declaring crate at 185 KB and
0.17 s instead of 2.0 MB and 33.4 s, a downstream crate able to declare its own widths with no orphan-rule
involvement, and op's four words answered without touching a single existing call site.

**The one thing I would not do**, and I state it because it is the reading of this dispatch that would be
easiest to reach: **do not delete the table and make `w!` the surface.** Section 5's 5.87 s is what that
costs at four-digit widths, per consumer crate, per build, and it is the binding-time error this whole
design exists to avoid making.

---

## 11. What is genuinely op's, and what is not

**Not op's, because it is a defect rather than a choice:** section 6. `110:3633` says something that was
never compiled and `123:247` prices something else. One of the three exits has to be taken whatever the
surface. I recommend exit 3, which falls out of section 10 for free, but the sentence in the canon has to
change under any of them.

**Op's, and it is a real fork rather than a formality:** whether the second alias exists. It adds a name
to the public surface, and `110:3265-3266` under D52 already says the compositions are public and bindable
by anyone, so this is naming a door that is open rather than opening one. It is nonetheless a name, and
names are op's under D56.

**Op's, and now much smaller than it was:** the number. `123:263-266` correctly left `W_MAX` to him. Under
section 10 it stops being a ceiling and becomes the extent of a convenience, which is the kind of number
that can be changed later without breaking anything, so it should not hold up the round.

**Not open at all:** whether a value can escape from a const parameter to a type without a table. It
cannot. `119:56-121` is sound, I re-ran its load-bearing route, and nothing in this file weakens it. What
this file establishes is that the escape is a consequence of the surface spelling rather than of the
design, which is a different claim and does not disturb 119's.

---

## Verification

Fifty-one compiles this pass, all under the pin, all in a scratch tree outside the repository, nothing
touching `mock/crates` or `mock/design_rounds/`.

The tower is rebuilt from `110:3312-3382`'s own declarations (sealed carrier, `Pos`, `Nat`) plus a
carry-chain addition of 21 impls, verified at `13 + 3 == 16`, `63 + 63 == 126`, `40 + 30 == 70`,
`1000 + 30 == 1030`, with the negative control firing at `E0080`. Four surface spellings are declared over
it in one crate and the three cross-spelling coercions compile. The five call-site positions are compiled
once per spelling in four separate consumer crates.

Timings are `/usr/bin/time -p`, warm, two runs each after a discarded first, spread under 10 ms except
`consA_4d` (0.06 s, 0.10 s) which is noted where it appears. Artifact sizes are `stat -f%z` on the
produced rlibs; symbol counts are `nm`. The literal-index table at 4096 rows reproduces `123:247` to
within 0.02 s, which is the cross-check that the measurement apparatus here and there is the same one.

Two things I did not get working and should not be read as refuted. A cartesian `macro_rules!` generator
for a decimal alias family, which failed on the repetition-depth rule
(`attempted to repeat an expression containing no syntax variables matched as repeating at this depth`);
a recursive munch is the known repair and I did not spend the compile on it, because the row cost is the
`w!` conversion either way and that is what prices it. And `119`'s routes two and three, which I did not
re-run, since route one reproduces and the three stand or fall on the same feature bans.

**Naming collision to flag for whoever files this.** The directory already contains an untracked
`124_consolidation_twelve.md` (7,099 lines, written 2026-08-06 21:37). This file was dispatched as `124`
as well. Two different documents at one number, and one of them will need renumbering before the panel
reads back cleanly.
