# 12. Can the surface meet the bar

**Date:** 2026-08-08. **Register:** breadth pass. Nothing here settles anything, per `04`.
**Probes:** `12_probes/`, nineteen probe sources plus three shared includes and two scripts. Every
compile is reproducible by the command in its own header, and `./verify.sh` runs all nineteen in one
command with its output committed as `out/verify.txt`.

## 0. The canon gate

Checked against `SETTLED.md`, `seed/SETTLED_surface.md`, `01` section 0, and `04`. Passed with one
qualification I state up front because it changes what this file is allowed to conclude.

The row this dispatch tests (`SETTLED.md:109`, three consumer tiers plus the `UInt<5>` bar) is marked
RATIFIED. `SETTLED.md`'s own header says every RATIFIED row predates op's correction to what
ratification means, and that a row is terminal only where the record shows the experts had stopped
disagreeing before he spoke. I did not find that convergence in the record for this row: op's words at
`142c:322-386` are a **correction to the panel's own repricing**, which is the shape of a steer, not
the shape of a ruling on a converged thing. So under `01` section 0 I read the bar as very strong and
possibly not terminal.

That does not license overturning it and I do not. It licenses one thing: saying what the bar costs,
which the dispatch asks for anyway.

Second qualification, and this one is load-bearing for everything below. **The bar's disqualification
list is two documents deep.** Op's sentence names two disqualifiers. The panel's compression names
four. I quote both in section 1 and keep them apart, because three of the candidates below die under
the compression and survive under op's sentence, and a reader who cannot see the difference will think
this file is arguing with a ratified line when it is arguing with an unratified derivation of one.

## 1. The bar, exactly

`SETTLED.md:109` is a compression. The establishing text is `seed/SETTLED_surface.md:135-159`, and
inside that, op's own words, quoted there from `142c:322-386`:

> To write the domain aliases, it has to be easy and intuitive to write the types they alias. UInt<5>
> is great and easy, intuitive. Ufixed as in the example too. But if there were precision in there,
> container types, it'd be fucking ass for anyone who doesn't know nor care about all that plumbing,
> to write. So the ergonomics of UFixed, FastFloat, all those, are crucial and perhaps more important
> than the plumbing itself.

And the panel's claim built over it, same file, lines 137 to 144:

> A framework author writing a domain alias has no context on containers, no practice with parameter
> order, and no interest in acquiring either; they will write the line a handful of times and never
> build fluency in it, so it must be self-evident on first contact and stay self-evident after six
> months away. `UInt<5>` (a name and a number) is the bar; `UFixed<5, 0>` (a name and the two numbers
> someone already thinks in) is at the bar. Anything requiring a container type, a type-level
> magnitude, a memorised parameter order, or a macro call falls below it and is disqualified at that
> site specifically.

### What each demands

Op's sentence demands: easy and intuitive to write, for someone who does not know and does not care
about the plumbing. It names two disqualifiers by example, **precision in the spelling** and
**container types**. It ends by ranking the ergonomics above the plumbing, which is the sentence that
matters most for this dispatch and is the one nobody has quoted back at the mechanism.

The panel's paragraph demands four more things: self-evident on first contact, self-evident after six
months away, no fluency required, and it adds two disqualifiers op did not name, a **type-level
magnitude** and a **macro call**, plus **extra arity in general**.

### Three things the bar does not say, which is where the answer lives

**It does not say the width has to be a const generic parameter.** It says `UInt<5>` has to be what
the writer types. Those are the same requirement only if the surface spelling must be literally the
form the compiler receives, and in Rust it need not be, because a type alias with a const parameter
decouples the two. That decoupling is candidate C4 below and it is the reason this dispatch has an
answer rather than a concession.

**It is scoped to one site, in its own words.** "Disqualified at that site specifically." A mechanism
that runs somewhere other than the alias definition line is outside what this bar forbids. That is not
a loophole I am reading in; it is the sentence's own qualifier, written by the file that states the
bar.

**It says nothing about what the writer reads when they get it wrong.** The bar is a writing bar. Half
of ergonomics is the diagnostic, and section 5 measures that separately because the bar does not cover
it and somebody should.

## 2. What is actually on the table

Verified before reasoning from it, per the dispatch.

`11`'s no-bridge ladder does what the brief says. I recompiled
`11_probes/b03_the_ceiling_is_the_const_surface.rs` unmodified on the pinned toolchain:

```
rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib --emit=metadata \
      -o /tmp/b03_recheck.meta b03_the_ceiling_is_the_const_surface.rs
```

Exit 0, no output. Three multiply octaves and a 1636-bit numeral, with the container derived exactly
(208 bytes asserted at compile time), no bridge trait anywhere in that half of the file, no feature
gate on the crate. The claim holds.

What the brief did not say, and what I had to open `11_probes/ladder.rs` to find, is **how a width is
spelled** in that half. It is spelled as a hand-written little-endian binary digit tower:

```
pub type T777 = D1<D0<D0<D1<D0<D0<D0<D0<D1<D1<Term>>>>>>>>>>;
```

The `T13` and `T777` names in `b03` are convenience aliases the probe wrote for itself at
`ladder.rs:443-457`. They are not a mechanism, they are fifteen hand-typed lines. So the honest
statement of the situation is not "the nat ladder has no ergonomic problem", it is "the nat ladder has
no *derivation* problem, and its ergonomic problem has not been looked at". This dispatch is that
look.

## 3. Five candidate surfaces, and what a consumer types

All five are compiled in one crate at `12_probes/p04_five_spellings.rs`, so that what a consumer types
is read off text that builds rather than off my typing. The counts below are produced by
`12_probes/count.sh`, committed as `12_probes/out/count.txt`, and the command is one line of `grep`
plus `${#var}`.

**C0, const surface with const keying.** The design as it stands. The numeral's own parameters are the
written widths, and a bridge maps each to a nat.

**C1, raw nat surface.** `11`'s `b03` with no convenience layer.

**C2, nat surface with a shipped alias layer.** arvo ships `N0`, `N3`, `N5` and so on as names for the
towers.

**C3, nat surface with a declaration-site macro.** The consumer mints the names it wants, in one line,
somewhere other than the alias site.

**C4, the hybrid.** The numeral is keyed on nats. The const appears in exactly one place, a type alias
with a const parameter, which is what a consumer writes. `pub type UInt<const N: u32> = Fixed<NatOf<N>,
T0, Warm>;` compiles and resolves (`p02`, exit 0).

### What each costs to type

Type-expression characters, from `out/count.txt`. The alias-definition tier is the one the bar governs.

| Surface | `StrHandle` | `Coord` | tier 3 signature | a named product |
|---|---|---|---|---|
| C0 | `UInt<5>`, 7 | `UFixed<13, 3, Hot>`, 18 | 66 | `UFixed<26, 6, Hot>`, 18 |
| C1 | `Fixed<D1<D0<D1<Term>>>, Term, Warm>`, 35 | 46 | 122 | 54 |
| C2 | `UInt<N5>`, 8 | `UFixed<N13, N3, Hot>`, 20 | 70 | 20 |
| C3 | `UInt<W5>`, 8 | `UFixed<W13, W3, Hot>`, 20 | 70 | 20, plus a 53-character declaration line once |
| C4 | `UInt<5>`, 7 | `UFixed<13, 3, Hot>`, 18 | 66 | `UFixed<26, 6, Hot>`, 18 |

**C4 is byte-for-byte identical to C0 at every consumer site.** Not close to it, identical, because the
const surface is unchanged and only what sits behind it moved. So the writing half of the bar is met by
the nat-keyed design, and the assumption this dispatch was sent to test is false as stated.

C1 fails the bar by any reading, including op's own two-disqualifier one: a writer has to know the tower
is binary, is little-endian, and is spelled with `D0`, `D1` and `Term`. That is plumbing, and op's
sentence is about not having to know plumbing.

C2 and C3 are one character over the bar and fail it on the panel's added disqualifier, the type-level
magnitude, which I think is right: `N5` requires knowing that a width is a type and which type. They
also both run into the enumeration refusal, which section 6 takes up.

And all four nat-keyed surfaces are **the same type**, which `p04` proves by passing a value through
`c1::Coord -> c2::Coord -> c3::Coord -> c4::Coord`. The alias layer, the macro and the door are
spellings over one thing, which is survivor 5 of `seed/SETTLED_surface.md` arriving again from a
direction it had not been derived from.

### The ceiling, checked rather than assumed

`p03` is `11`'s `b01` case with the hybrid substituted. The door has **six** rows, `0, 3, 5, 8, 13, 24`,
and contains neither 48 nor 96 nor 192. It multiplies a 24.8 numeral by itself, then that result by
itself, then that by itself, and compiles at exit 0. The container stays exact at every step, asserted
at compile time: 4, 8, 16 and 32 bytes.

`b01` could not write the first of those. `b02` showed that adding the row it wanted moved the failure
one octave up. So the ceiling was the **keying**, not the table, and moving the const from the numeral's
parameters to a type alias in front of it is enough to lift it while changing nothing a consumer types.

### And the erasure holds through the door

`p03`'s emitted assembly, `out/p03.s`:

```
	.globl	_p03_native16
_p03_native16 = _p03_arvo16
	.globl	_p03_native_vec
_p03_native_vec = _p03_arvo_vec
```

The assembler aliased the symbols, so the numeral and the native `u16` are not merely equivalent, they
are the same code. This is an ad-hoc quick spike with no substance as a measurement and is named that;
it establishes an existence claim about codegen and prices nothing.

## 4. Nat canonicity, which everything above rests on and nobody had asked

If a consumer names a product width with a decimal and the algebra computed it structurally, the two
have to be the same type. `p01` checks it by passing a value of the computed type into a slot spelled
with the written type, at three points including a carry chain that lengthens the tower:

`Sum<T13, T13>` is `T26`. `Sum<T24, T24>` is `T48`. `Sum<T13, T3>` is `T16`. All three compile, exit 0.

So the ladder's addition is canonical, and the door's output and the algebra's output are one type. That
is what makes `p03`'s `UFixed<26, 6, Hot>` and its `Sum<T13, T13>` interchangeable, and it is a
precondition of the whole hybrid that nothing in the record states.

The same file records the hazard on the other side: `D1<D1<D0<Term>>>` and `D1<D1<Term>>` both have
`Nat::V == 3` and are **distinct types**. A padded tower is a second spelling of a number that no
operation will ever produce and that a hand-writing consumer can produce trivially. Under C4 nobody
hand-writes a tower, so the hazard is closed by the door rather than by the ladder. Under C1, C2 and C3
it is open.

## 5. What a consumer reads, which is where the answer stops being clean

The bar governs writing. Nothing in the record governs reading, and reading is where the nat keying
lands its bill.

### The ordinary mistake

Return a 26.6 numeral where a 13.3 was declared. This is the error a tier-two consumer makes on their
first day and again every week after. `p11_diag_battery.rs` makes it five ways in one file, one keying
per module, and the compiler's own text is the measurement (`out/p11.log`).

**K0, const keying.** Primary label `expected 13, found 26`. Full type `Fixed<13, 3, Hot>`. The
consumer reads the sentence and is done.

**K1, binary nat keying (this is C4).**

```
expected `Fixed<D1<D0<D1<D1<Term>>>>, ..., ...>`,
   found `Fixed<D0<D1<D0<D1<D1<...>>>>>, ..., ...>`
```

The primary label is a truncated binary digit tower. To act on it a consumer decodes little-endian
base two, and the innermost digits they would need are the ones rustc elided as `...`. This is not
"less pretty". It is a sentence that cannot be acted on without turning on verbose type printing and
then doing arithmetic.

So the honest answer to the dispatch's question is not yes. **The nat surface meets the writing bar
exactly and fails the reading side badly, and the reading side is not in the bar because nobody wrote
it down.**

### Attacking that, route one: put the consts back where rustc prints first

`p06`. A numeral may carry the widths as const parameters and the nats as **defaulted type parameters
projected off them**: `Fixed<const I: u32, const F: u32, S, WI = NatOf<I>, WF = NatOf<F>>`. That
construction compiles, which is itself a fact nobody in the record has stated: **a type parameter
default may be a projection off a const parameter of the same struct.**

rustc does **not** elide a defaulted parameter in a diagnostic, so the tower still appears. But it
picks the first differing parameter for the primary label, and the consts are first:

```
expected `Fixed<13, 3, Hot, D1<D0<D1<D1<Term>>>>, D1<D1<Term>>>` because of return type
     ^ expected `13`, found `26`
```

The headline is K0's headline. The towers become noise in the `note:` lines rather than the sentence
the consumer has to act on. That is most of the recovery for one parameter of arity that never appears
at a consumer site, because the defaults fill it.

### The residue, and four more refusals

Nothing ties the const to the nat. A consumer who writes all five parameters can produce a numeral whose
printed width is not its real width. The tie wants the nat's value in const position, and the nat
already carries its value as an associated const computed structurally with no table.

Four positions, four refusals, `p07a` through `p07d`:

| Position | Result |
|---|---|
| `impl<W: Nat> NatIs<{ <W as Nat>::V }> for W`, no gate | `generic parameters may not be used in const operations`, names `generic_const_exprs` |
| the same under `min_generic_const_args` | `use of const in the type system not defined as type const`, suggests `type const V` |
| `type const V: u32 = 2 * T::V;` | `complex const arguments must be placed inside of a const block` |
| `type const V: u32 = const { 2 * T::V };` | names `generic_const_args` |

All four are forbidden. This is the same wall `10` hit in three syntactic positions and `11` in four
more, reached from the opposite direction: those were const to nat, these are nat to const. **The wall
is symmetric, and that is worth knowing, because a reader of `10` and `11` could reasonably have hoped
the reverse direction was open.**

What that leaves is a tie by **construction rather than by a bound**: the defaults fire at every site
that omits the nat parameters, which is every ordinary site, and the decoupling is reachable only by
writing the full parameter list deliberately. When it is reached the damage is display and type
identity, not layout or laws, because everything below the surface follows the nat. That is a real hole
and a small one, and it is stated rather than hidden.

### Does the p06 shape keep the ceiling?

No, if it is used alone, and this is the part that decides the shape. `mul`'s output consts have nothing
to pin them, so either they are inferred (and nothing constrains them, so inference fails) or they are
pinned by a width-sum bound, which is `11`'s `p12` and is exactly what caps the algebra.

`p08` takes the other branch: two head constructors. The **named** numeral carries consts in front and
nats defaulted off them, so a consumer's declarations and mismatches read like K0. The **derived**
numeral is nat-only, so the algebra is closed. Entering the derived world is free; coming back names a
width, and only that naming touches the door. It compiles with a six-row door containing none of 48, 96
or 192, and the container is exact at 4, 8, 16 and 32 bytes.

The composition that falls out is the actual answer to the dispatch, and section 8 states it.

### Attacking that, route two: make the tower legible

There is no lever on the message itself. `#[diagnostic::on_type_error]` is an unknown attribute on this
toolchain, checked, so an E0308 cannot be given a message. The only remaining lever is the **spelling**
of the type rustc prints, and a reader decoding `D1<D0<D1<D1<Term>>>>` does two separate things: reads
little-endian, and converts base two. The second one is removable.

`p09` and `p10` move the ladder to base ten. 13 becomes `T<N3, T<N1, E>>`, the digits of the number,
reversed, and nothing else. This is not a sketch of an idea; both halves of the ladder are built:

**`p09`, base-ten structural addition with carry.** Digit sums go through a unary tally and a
twenty-row normaliser, and the tower recursion is one impl per carry state. Checked against `Nat::V`
at ten points including `8 + 8` (a carry that lengthens the tower), `99 + 1 = 100` (a carry chain), and
`777 + 777 = 1554`. Canonicity checked at four points, the same way `p01` checks it for base two.

**`p10`, the container derivation in base ten.** Halving is twenty rows, ten digits by two carry states.
`floor(log2) + 1` is the number of halvings before the tower empties. `ceil(W/64)` is six halvings plus
the same round-up identity the binary ladder uses. The native rung is asserted at every boundary and one
past it, 8, 13, 16, 17, 32, 33, 64, 65, 128, and the wide payload at 129, 200, 256 and 1636 bits, where
it lands on 208 bytes, which is `b03`'s number reached in a different base.

The whole table in both files is a table **of digits**, closed at ten. The binary ladder already has one
of these, sixteen `Add` and `AddC` impls at base two. No width appears in either.

What it buys, from `out/p11.log`:

```
K1  expected `Fixed<D1<D0<D1<D1<Term>>>>, ..., ...>`, found `Fixed<D0<D1<D0<D1<D1<...>>>>>, ..., ...>`
K3  expected `Fixed<T<N3, T<N1, E>>, T<N3, E>, Hot>`, found `Fixed<T<N6, T<N2, E>>, T<N6, E>, Hot>`
```

Two things changed and the second is the important one. The digits are readable, `N3 N1` against
`N6 N2`, reversed. And **the truncation is gone**: base two needs five levels for 13 and rustc elides
past its depth limit, base ten needs three and it does not. K1's message cannot be acted on at all; K3's
can be, once a reader knows to read it backwards.

And in combination, `K4`, base ten with the consts in front, gives both the K0 headline and an untruncated
readable full type:

```
expected `Fixed<13, 3, Hot, T<N3, T<N1, E>>, T<N3, E>>` ... expected `13`, found `26`
```

## 6. The enumeration question, which none of the five escapes

`SETTLED.md:110`: no enumeration, ever, if it can be helped, refused four times against a width table, a
per-width bridge population line and a macro escape.

Read against the candidates, that row disqualifies more than it looks:

- **C2's shipped `N0..N64`** is a width table. Refused on its face.
- **C3's declaration-site macro** is the macro escape. Refused on its face.
- **C0's bridge** is one impl per written width, which is the per-width bridge population line, and
  `SETTLED.md` lists it as still open with `10`'s concession behind it.
- **C4's door** is the same object.

So the choice is not between a design with a table and one without. All of them have one except C1, and
C1 fails the bar. What differs is **how much the table has to cover**, and that is the whole finding:

| | what the table must contain |
|---|---|
| C0 | every width any consumer writes **and every width the algebra produces**, which is unbounded (`11`'s `b01`, `b02`) |
| C4 | every width a consumer **writes**, which is bounded by the program's own source text |

`10` reports the bridge not dissolvable, with thirteen routes enumerated and six attacked. I did not
find a fourteenth and I did not try to; `11` closed four more positions and I closed four in the reverse
direction. What the door does is not dissolve it. It **bounds its domain to the finite set of numbers a
human typed**, which is a different claim and, I think, the one that matters, because an unbounded
requirement is a design defect and a bounded one is a line of setup.

## 7. What a consumer reads at the alias site, which is worse than anyone thought

This is the finding I did not go looking for and it is the one I would put in front of op first.

`p12` writes an undeclared width at the alias-definition site under C0 and under C4:

```rust
pub type Undeclared_C0 = c0::UInt<7>;
pub type Undeclared_C4 = c4::UInt<7>;
```

**Neither produces any error at all.** A Rust type alias does not check its bounds. The only diagnostic
in that file is C2's, an immediate `E0425: cannot find type N7 in module c2` pointing at the token the
consumer typed, with a "similarly named type alias exists" suggestion.

`p13` finds where the C4 error does land. The alias is written at line 24; the error arrives at line 40,
at the first **use**, and reads:

```
error[E0277]: no width literal `c4::Idx<7>` is declared in this program
  --> p13_where_the_door_error_lands.rs:40:20
40 | pub fn read_tag(t: PacketTag) -> PacketTag {
   |                    ^^^^^^^^^ this literal width is not declared
```

The span is on `PacketTag`, not on the `7`. The named type is `c4::Idx<7>`, an internal the consumer has
never seen. In real code the alias and the use are in different files, so the consumer is told about a
line they did not write, naming a type they do not have, about a mistake somewhere else.

**This is a defect of the const door and it belongs to the design as it stands, not to the hybrid**,
because C0 has exactly the same door and `p12` shows exactly the same silence. It has simply never been
looked at, because nobody had compiled the tier-two experience as a tier-two consumer would meet it.

It reverses the diagnostic ranking at this one site. The name-based surfaces, which the bar disqualifies
on writing, are the only ones that tell a consumer anything **at the line they typed**.

### Attacking that

`p14`. Rust has one lever: `lazy_type_alias`, which makes a type alias well-formedness-checked at its own
declaration. Under it, with a `where` clause on the alias:

```
error[E0277]: no width literal `Idx<7>` is declared in this program
  --> p14_lazy_type_alias.rs:59:1
59 | pub type PacketTag = UInt<7>;
   | ^^^^^^^^^^^^^^^^^^ this literal width is not declared
```

Line 59 is the line the consumer typed. The silence closes completely.

Two costs, both measured rather than guessed. The feature is `lazy_type_alias`, which is **not on the
workspace's vetted list** and carries `incomplete_features`; this file claims only what it would buy and
makes no admissibility argument. And turning it on over the whole ladder produces fifteen errors
(`p14b_lazy_over_full_ladder.rs`, captured in `out/p14_full.log`), all of the "this internal projection
alias now wants a bound" kind, against `Sum`, `Cont`, `Q6` and `R5`. `p14` shows writing those bounds is
what fixes it, so the cost is library-side work rather than a wall.

It is also, notably, a **consumer-side** feature: the gate has to be on in the crate writing the alias.
That is a serious constraint on a library and I have not thought it through.

## 8. The composition

Not a winner. Which shape wins where, and what flips the answer.

**The writing bar does not decide between C0 and C4 at all.** They are identical at every consumer site,
character for character, and `p04` compiles both. Anyone choosing between them on ergonomics grounds is
choosing on a difference that does not exist.

**What decides is which of two costs a design would rather pay.**

C0 pays an **unbounded table**. Every width the multiply algebra produces needs a row, and `b02` shows
adding rows chases its own tail one octave at a time. In exchange the diagnostics are clean at every
site except the alias site, where both are silent.

C4 pays **the diagnostic**, unless one of the two repairs is taken. In exchange the table's domain is
the finite set of numbers in the program's source, the algebra is closed to any depth, and `p15` shows
a third thing nobody was looking for: **the door dissolves the marker partition.** Under const keying,
`11`'s `consumer_partition.rs` proved two markers do not compose, so two crates that each declared width
13 have two incompatible 13-bit numerals. Under the door, the marker selects which nat a literal maps to
and is then gone from the type. `p15` compiles three crates with three markers passing one value between
them, plus a 4711-bit numeral declared by only one of them, at exit 0.

**The two repairs to C4's diagnostic are independent and compose.** Consts in front (`p06`) recovers the
headline. Base ten (`p09`, `p10`) recovers the full type and removes the truncation. Together (`K4`) the
diagnostic is as good as C0's plus untruncated noise. Each has a price: consts in front needs a second
head constructor for derived values (`p08`), and base ten needs the ladder rebuilt, which `p09` and
`p10` show is roughly sixty impls, all of them tables of digits.

**Where the answer flips.** If a design decides that the multiply algebra genuinely does not need to be
closed past one octave, C0's table is bounded in practice and its cost is imaginary, and C0 wins on
simplicity. `b01` and `b02` show it is not closed past the first product of two written widths, so this
turns entirely on whether a consumer ever multiplies twice, which is a question about consumers and not
about types. Nobody has asked it and I did not.

If a design decides diagnostics are worth a whole ladder rewrite, base ten is available and complete. If
it decides they are worth one parameter of arity that no consumer ever writes, `p06` is nearly free.

## 9. Was the bar priced with the ceiling visible

No, and this is the plainest statement I can make.

The bar was set at `142c`. `b01`, `b02` and `b03`, which established that the const keying caps the
multiply algebra, are `11`, from tonight. The document that states the bar could not have weighed a cost
that had not been found, and it does not mention one; it reasons entirely about what a framework author
finds easy to type.

That is not a criticism of the bar. It is the observation that the bar was priced against **one** cost,
ergonomics, and the panel has since found a **second**, and the two are not in conflict. `p04` shows the bar is satisfiable by the nat-keyed design and
`p03` shows the ceiling lifting under the same spelling. So the trade nobody was offered turns out not to be a trade, and `p03` is where that is compiled.

Where the bar **is** open to challenge, and I say this because the dispatch asked and because saying it
plainly is worth more than hedging: its disqualification list is two documents deep and the outer
document adds things op did not say. "A macro call" and "a type-level magnitude" are the panel's
extensions. Op's own sentence names precision in the spelling and container types, and ends by ranking
ergonomics above plumbing. Under op's sentence, C3's declaration-site macro is not obviously
disqualified, because the macro is not in the spelling and the spelling is `UInt<W5>`. Under the panel's
paragraph it plainly is. I do not need that distinction for anything here, because C4 satisfies both
readings, but a later member reading `SETTLED.md:109` alone will not know the list is layered, and
`SETTLED.md:109` is what a later member will read.

## 10. What I did not cover

Stated so the edges are visible rather than implied.

I did not read the closed predecessor panel, per the dispatch. Every claim about `137`, `142c` and the
`b01`/`b02` results is taken from `10`, `11`, `SETTLED.md` and `seed/SETTLED_surface.md`, except `b03`,
which I recompiled myself.

**Compile time is unpriced.** The mockspace bench harness did not run and no number appears in this
file. The concern has a shape worth naming for whoever prices it: `p10`'s decimal `ToTally` decrements
once per word, so a 1636-bit numeral walks twenty-six steps where the binary ladder's is structural, and
`p09`'s digit addition goes through a unary tally that is up to nineteen `S` deep. Both compiled; neither
is measured; anyone quoting a magnitude from this file is quoting nothing.

I did not build the strategy axis into any candidate. Every probe carries `Hot` or `Warm` as an inert
marker, and how a strategy interacts with the door or with base ten is untouched.

I did not test the signed case, the fractional-only case, or a zero-width numeral in any candidate.

I did not test whether `lazy_type_alias` composes with the rest of the ladder once bounds are written,
only that the ladder wants fifteen of them and that a minimal case works.

I did not build `p08`'s two-head split as anything a consumer would want to use. It has a `.derived()`
call at the entry to every law, which is an ergonomic cost at the use tier that the bar does not govern
and that I did not measure. Whether it can be hidden behind a bound on the law rather than a call is
open and I would attack that first if I were continuing.

I did not attack the alias-site silence from any direction other than `lazy_type_alias`. A macro that
declares the alias and asserts the bound in one line would close it without a feature, and it would put
a macro call at the exact site the bar disqualifies, so I did not build it.

**And one thing I would flag outside my lens.** `SETTLED.md:109`'s compression of the bar loses the "at
that site specifically" qualifier and loses the split between op's disqualifiers and the panel's. Both
are in `seed/SETTLED_surface.md` and neither survives into the file a later member is told to read. That
is the compression failure this panel's own rules describe, occurring in the index rather than in a
consolidation, and it cost me an hour to notice.
