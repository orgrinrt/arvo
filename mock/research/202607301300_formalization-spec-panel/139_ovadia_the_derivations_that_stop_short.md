# 139. The derivations that stop short, and what the typestate is missing at each

**Author:** Evan Ovadia (persona dispatch)
**Date:** 2026-08-07
**Position:** after `138_knuth_the_families_and_their_laws.md`, taking the three places op has said the same
sentence about: the width bridge (`137b:28-45`), the stored-width overshoot (`137b:47` via `138:258-276`),
and Warm's crossover (`137b:55-85`).
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2024, probes at `139_probes/`, all built
outside the repository tree.

## Verdict, stated first

Two of the three share a missing quantity and it is exact. The stored-width overshoot and Warm's crossover
are the same mistake: **a rounding step applied to an already-rounded quantity, where the truth rounds
once over the composed thing.** In both cases the fix is to compose before rounding rather than after, in
both cases the fix is a one-line change to a formula the design already has, and in both cases the quantity
that was missing is already present in the typestate under another name.

The third, the width bridge, is a different shape and I will not pretend otherwise. It is not a rounding, it
is a notation change being attempted in the wrong layer. The brief's hypothesis that all three are one
problem is **half right**, which I think is a more useful answer than either yes or no would have been.

## A note on `138b`, which landed while this was in flight

Op's thirty-first checkpoint was committed partway through this dispatch and is required reading, so I state
where it touches this file. It does not conflict with anything here and it sharpens one thing.

`138b:11-15` records that there are no four families: the named types are points in a product of four axes,
and `138b:28-29` that "there is one representation" with the aliases as spellings over it. Section two below
concludes independently that the stored-width confusion is one derivation being asked two questions, and
that the second question is keyed on `Encoding`. Those are the same move at two scales, which I take as
corroboration only in the weak sense: `Encoding` is one of the axes, so saying the laid-out width is an
`Encoding` fact is saying it varies along an axis rather than distinguishing a family. That is what the axis
presentation is for, and it is a small piece of evidence that the presentation earns its keep.

`138b:60-61` states the clue this file was dispatched to chase: "something distinguishes decimal32 and
decimal64 from decimal128 and from binary, and naming it is most of the fix". Section two names it and the
criterion is exact at eleven of eleven in integer arithmetic. It is the layout slack reaching two.

## Two. The stored width overshoots, and the missing quantity is the layout's slack

Op's sentence: "there's simply something we need to make the derivation accurate and not overshoot, so all
we need is name that, thread it into the typestate so it reaches us there, and use that to make it accurate
and ideal."

There is such a thing, it is one rational number per numeral, `138` already computes it in a neighbouring
section without noticing it is the same quantity, and it is already keyed on something the typestate carries.

### The quantity

`138:263-266` reports that the per-field sum form overshoots at decimal32 and decimal64 and is exact
everywhere else. That is a correct measurement of a symptom. The cause is that
`ceil(x) + ceil(y) >= ceil(x + y)`, with the excess being a whole bit exactly when the two fractional
residues sum past one. Computed in `139_probes/a_residue.py`:

```
format       joint  perfield  actual     r_sig   r_exp   r_sum  verdict
binary16        16        16      16     0.000   0.093   0.093  ok
binary32        32        32      32     0.000   0.011   0.011  ok
binary64        64        64      64     0.000   0.001   0.001  ok
binary128      128       128     128     0.000   0.000   0.000  ok
binary256      256       256     256     0.000   0.000   0.000  ok
bfloat16        16        16      16     0.000   0.011   0.011  ok
E4M3(OCP)        8         8       8     0.000   0.093   0.093  ok
E5M2(OCP)        8         8       8     0.000   0.093   0.093  ok
decimal32       32        33      32     0.747   0.415   1.162  OVERSHOOT
decimal64       64        65      64     0.849   0.415   1.264  OVERSHOOT
decimal128     128       128     128     0.054   0.415   0.469  ok
```

The residues are shown as decimals for reading. **The verdict is decided by integers with no logarithm
anywhere**, which matters, because a float comparison at these magnitudes is exactly the kind of thing that
would make the finding worthless:

```
perfield exceeds joint exactly when 2^(sig+exp) >= 2 * card
  binary64    2^(sig+exp)=   9223372036854775808  2*card=  18428729675200069632  loses_a_bit=False
  decimal32   2^(sig+exp)=            4294967296  2*card=            3840000000  loses_a_bit=True
  decimal64   2^(sig+exp)=  18446744073709551616  2*card=  15360000000000000000  loses_a_bit=True
  decimal128  2^(sig+exp)=170141183460469231731687303715884105728  2*card=245760000000000000000000000000000000000  loses_a_bit=False
```

So the quantity to name is the ratio of the laid-out code space to the code space actually needed. Call it
the **layout slack**, `slack = 2^(sig + exp) / card`, a rational, one per numeral, and it is multiplicative
across the fields:

> `slack = product over fields of ( 2^bitlen(field cardinality) / field cardinality )`

A whole bit is lost exactly when the product reaches two. `139_probes/b_slack_is_one_quantity.py` asserts
that equivalence at all eleven formats and it holds at all eleven.

### The part I did not expect, which is that `138` already computed this

`138:283-300` computes a slack for the exponent field alone (`codes - span`) and uses it to derive FL8, the
law that E4M3 cannot carry `IeeeSpecials` because its exponent span leaves one spare code rather than two.
That is the same quantity, restricted to one field:

```
format       slack_sig  slack_exp  slack_all
binary16        1.0000     1.0667     1.0667
E4M3(OCP)       1.0000     1.0667     1.0667
decimal32       1.6777     1.3333     2.2370
decimal64       1.8014     1.3333     2.4019
decimal128      1.0385     1.3333     1.3846
```

At radix two the significand slack is exactly one, always, because the significand's cardinality is a power
of two. So `slack_all == slack_exp` at every binary format and only at binary formats, which is why a
one-field figure was sufficient in `138`'s section 3.2 and insufficient in its section 3.1. **The same
number, computed twice, once correctly for a narrower question and once not at all for the wider one.**
`138:340-344` even says FL8 "is scoped to radix two on purpose" and that the decimal analogue "has to be
derived from the combination field's own encoding". The multiplicative form is that derivation, and it
subsumes FL8 rather than sitting beside it.

That is one mechanism where the design currently has two, and deleting the narrower one is the part of this
I would defend hardest.

### Where it already lives in the typestate

Nowhere new. `138:343` names the right key in passing: a fact about how many bits a layout costs "is an
`Encoding::Fields` fact rather than a numeral one". Take that literally and the confusion dissolves, because
there were never one width and one broken derivation. There are **two different widths answering two
different questions**, and both derivations are correct:

- **The carrier width `W_S`** is a fact about the numeral. It is `sign + ceil(log2(R^(P-h) * span))`, one
  ceiling over the composed cardinality, exact at eleven of eleven per `138:236-250`. It is keyed on radix,
  `P`, `EMIN`, `EMAX` and the hidden digit, all of which the identity contract already carries.
- **The laid-out width `W_F`** is a fact about the encoding. It is the per-field sum, and at decimal32 the
  answer 33 is not an error: it is the correct width of a decimal32-shaped numeral laid out with one field
  per component and no combination field. IEEE spends the difference; a design that did not would pay it.

So the derivation was never inaccurate. It was **accurate about a question nobody had distinguished**, and
the fix is to key it on `Encoding` instead of on the numeral. There is nothing to add to the typestate.

### The consequence for a declaration the design currently makes

`110:3248` declares `type StoredWidth: StoredWidth;` on `Lowering` with the comment "the carrier level;
`W_F <= W_S`, declared (1.22)", quoted at `138:219-220`. Two things follow and both are load-bearing.

The declaration goes, per `138`'s X7, because `W_S` is derivable. That much `138` already argues.

**The inequality is backwards for a per-field layout, and nobody has said so.** At decimal32, the per-field
sum is 33 and `W_S` is 32, so `W_F <= W_S` fails at a shipped format. Whatever `W_F` was meant to denote at
`110:3248`, it cannot be the per-field sum; and if it was, the relation is `W_S <= W_F` with equality exactly
when the layout slack is below two. I flag this as a defect in the declaration rather than proposing wording
for it, because I cannot tell from `110:3248` alone which of the two `W_F` means, and guessing is how the
panel got the lattice claim at `110:1440` wrong.

## Three. Warm widens a numeral that fits, and the missing quantity is the operation's carry margin

`131:275-280` states the rule: "Hot and Cold take the minimum aligned native; Warm and Precise take one rung
of headroom, which is what carries single-operation overflow room for Warm's wrapping and Precise's
saturating semantics."

That rule is `rung(rung_bits(W) + 1)`. It rounds `W` up to a rung, then rounds the result up again. Exactly
the shape of section two. Composing first gives `rung(W + margin)`, and the difference is not marginal. From
`139_probes/c_warm_rung.py`:

```
   W    Hot   131 Warm   rung(W+1)   spare  no-headroom
  13    u16        u32         u16       3          u16
  16    u16        u32         u32       0          u16
  32    u32        u64         u64       0          u32
  63    u64       u128         u64       1          u64
  64    u64       u128        u128       0          u64
  65   u128       Wide        u128      63         u128

widths at or below 64 that 131's rule places in a container wider than Hot's:
  64 of 64, that is every one of them
widths at or below 64 that rung(W+1) places wider than Hot's:
  [8, 16, 32, 64]   (4 of 64)
  these are exactly the widths that exactly fill their rung, where spare == 0
```

So the double rounding is doing almost all of the damage. **Composing before rounding fixes sixty of the
sixty-four widths at or below 64 bits, and it fixes them without touching the semantics at all**, because
`W + 1` is the honest statement of what one operation's carry needs and `rung_bits(W) + 1` never was.

### The four that remain, and why they need no headroom either

The survivors are 8, 16, 32 and 64: the widths that exactly fill their rung and have no spare bit. They are
also, precisely, the widths a consumer reaching for arvo's algorithms on bare primitives will write, which is
the failure mode op named at `137b:78-82`. So fixing sixty of sixty-four fixes the wrong sixty.

The second half of the fix is to ask what the margin is for, and the answer is that Warm does not need it.

**Where `W` equals the rung width**, an add overflows the container, and the wrap the container performs is
the wrap at `W`. That is not an approximation of Warm's semantics, it is Warm's semantics, delivered by
`wrapping_add` at no cost. The margin is satisfied by the hardware, not by the container.

**Where `W` is below the rung width**, the container has `spare >= 1` bits, so the sum of two `W`-bit values
cannot lose a bit before the mask, and `(a + b) & mask` wraps at `W` exactly. One `and`.

Either way the container is the minimum aligned native, and Warm's wrapping is exact. Compiled at `-O`,
`139_probes/d_warm_codegen.rs`:

```
w64_headroom           3 instructions   add | mov | ret
w64_native             2 instructions   add | ret
w13_headroom           3 instructions   add | and | ret
w13_native             3 instructions   add | and | ret
```

The scalar difference is one instruction and would not be worth a design change on its own. **The case op
named is the one that matters, and there the difference is not one instruction:**

```
w64_headroom_vec: 25 ins, 1 branches ['b.ne LBB4_1'], 0 simd ops   -> ROLLED LOOP (per-iteration cost)
w64_native_vec:   81 ins, 0 branches,                32 simd ops   -> FULLY UNROLLED (total cost)
```

Sixty-four elements cost roughly 1600 instructions under 131's rule and 81 without it, near enough twenty
times, because the `u128` form has a carry chain LLVM cannot vectorise and the `u64` form autovectorises to
thirty-two `add.2d`. That is the same mechanism `135b:54-61` measured for the byte-array body, arriving from
a different direction and on the design's own default strategy.

### The finding I would defend hardest here, because it deletes something

For `W` below the rung the mask is required whether or not the headroom exists, since the container wraps at
the rung and the semantics wrap at `W`. So under `131`'s rule the mask is there **and** the headroom is
there, and the mask alone is sufficient. **The headroom and the mask are two mechanisms doing one job**, and
the headroom is the one that can go.

It is worse than redundant. `131:277` scopes the headroom to "single-operation overflow room", so a chain of
accumulations must mask anyway, which means the mask is present in every program regardless. What the
headroom buys is that an unmasked chain silently accumulates to the container width instead of wrapping at
`W`, which is not Warm's semantics under either reading. So the rule does not merely cost a rung; where it
appears to be doing work, the work it is doing is wrong.

### What replaces it

> Warm and Precise take the same container as Hot and Cold, the minimum aligned native for `W`. An
> operation's carry margin is satisfied by the container's spare bits where `W` is below the rung, by the
> hardware's own carry where `W` equals it, and by a widened intermediate at the operation for multiply,
> whose result numeral is at a different rung in any case.

This is right by default rather than right when configured, which is the property `137b:78-82` requires, and
Warm's crossover to the wide rung moves from 65 bits to 129, matching Hot and Cold. What remains
strategy-dependent is alignment (`131:280`, Hot takes align-16 at the wide rung) and the ragged-versus-
word-rounded wide payload op adopted at `137b:48-53`, neither of which is touched.

**What this costs.** Precise's saturating semantics need overflow detection, which `overflowing_add`
delivers as a flag with no widening, so I do not believe Precise loses anything either. But Precise is not
what op reopened, the detection path is one I have argued rather than compiled, and the strategies are
asymmetric enough (`131:271-278`) that I will not fold Precise into this on an argument. That check is owed
before the rule is written as covering all four.

## One. The width bridge, where the missing thing is not a quantity

Op's sentence here is different from the other two and it took me a while to see it. He does not say the
derivation is inaccurate. He says the spelling is the problem and that it should come implicitly from the
typestate. So the question is not "what number is missing" but "why does a crossing have to be spelled at
all", and the answer turns out to be that nobody had asked what kind of thing the crossing is.

### First, what the wall actually is, compiled

The brief says every direct route from a const to a type is closed and I checked rather than took it,
because `138` found that `130`'s section 10 cites five probe files that do not exist. The wall is real and it
is narrower than "const to type". It is exactly **arithmetic**, and `min_generic_const_args` gives more than
the panel has been using.

An associated const **can** sit in const-argument position, if it is declared `type const`
(`139_probes/p1_mgca_assoc_const.rs`):

```
error: use of `const` in the type system not defined as `type const`
help: add `type` before `const` for `Sum2::S`
```

And with that spelling, all three of these compile clean with no other feature (`139_probes/p1d.rs`):

```rust
impl<const A: u32, const B: u32> Sum2 for Pair<A, B> { type const S: u32 = A; }   // bare parameter
impl Sum2 for Lit { type const S: u32 = 16; }                                    // literal
pub fn a<const A: u32, const B: u32>() -> Idx<{ <Pair<A,B> as Sum2>::S }> { Idx } // projection into
                                                                                 // const-arg position
```

**So projection works and creation does not.** Every arithmetic RHS is refused, and the diagnostic names
precisely the feature op ruled out (`p1b.rs`, `p1c.rs`, `p1e.rs`):

```
error: complex const arguments must be placed inside of a `const` block      // = A + B
error: generic parameters may not be used in const operations                // = const { A + B }
  = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
error[E0658]: inherent associated types are unstable                         // = Self::HELPER
```

That last one closes the obvious dodge: routing the arithmetic through an inherent associated const does not
evade the rule, it hits a different unstable gate.

So the position is exact. **No derived width can reach const-argument position under the allowed feature
set.** That is why the magnitude has to be structural: 137's ladder does every selection by impl matching,
which needs no arithmetic anywhere, and it compiles with no `#![feature]` at all, which I confirmed by
building `137_probes/ladder.rs` standalone.

### Second, the thing the panel has been calling a solved case

`137_probes/ladder.rs` is described in the brief as "the derived ladder that compiles gate-free". It is. Its
last fifteen lines are:

```rust
pub type T0 = Term;
pub type T3 = D1<D1<Term>>;
pub type T8 = D0<D0<D0<D1<Term>>>>;
pub type T13 = D1<D0<D1<D1<Term>>>>;
...
pub type T777 = D1<D0<D0<D1<D0<D0<D0<D0<D1<D1<Term>>>>>>>>>>;
```

Fifteen hand-written width aliases, in the file that demonstrates the ladder needs no enumeration, and they
are there because the probe had to get its literals across somehow. I am not scoring a point off it; the
probe was answering a different question and it answered it. I raise it because it is the cleanest possible
statement of where the crossing sits: **everything above the aliases is derived and nothing enumerates a
width; the aliases are the whole of the problem.**

### Third, the reframe

The panel has been treating the crossing as something that has to happen **inside the type system**, and
inside the type system it is closed. But the crossing is not a semantic step. `13` and `D1<D0<D1<D1<Term>>>>`
are the same number in two notations, and going between two notations for one number is what a compile-time
expansion is for.

The literal is already in the consumer's source text. A **function-like macro is the one mechanism that sees
that text** before the type system does, which puts it on the correct side of the wall by construction rather
than by finding a loophole. `macro_rules!` cannot do it: its fragment matchers offer no way to decompose `13`
into digits, a literal is one token, and the only general rule available is a whole-literal match, which is
the table again (`139_probes/negctl_macrorules.rs` states the mechanism; it is a demonstration, not a proof).
A function-like proc macro can, in about twenty lines, with no table, no cap, no feature gate and no flag.

`139_probes/natmac.rs` is the whole of it. The load-bearing part:

```rust
fn structure(mut n: u128, out: &mut String) {
    let mut depth = 0usize;
    if n == 0 { out.push_str("Term"); return; }
    while n > 0 { out.push_str(if n & 1 == 1 { "D1<" } else { "D0<" }); n >>= 1; depth += 1; }
    out.push_str("Term");
    for _ in 0..depth { out.push('>'); }
}
```

### What it does, compiled

**It agrees with the hand-written aliases exactly.** `139_probes/consumer.rs` asserts type identity at all
thirteen of the aliases 137 wrote, through a `fn same<T>(PhantomData<T>, PhantomData<T>)` that only builds if
the two types are literally the same:

```rust
same::<nat!(13)>(PhantomData, PhantomData::<T13>);   // and 0, 3, 8, 16, 24, 30, 40, 41, 64, 100, 200, 777
```

Thirteen of thirteen. The crate compiles clean.

**There is no cap.** `139_probes/big.rs` builds `nat!(1152921504606846976)`, two to the sixtieth, along with
`nat!(4294967295)` and `nat!(1)`. No recursion-limit attribute, no depth annotation.

**The literal spelling survives.** One `macro_rules!` line carries the surface, and the only difference from
`UFixed<13, 3, Warm>` is the bang:

```rust
macro_rules! UFixed { ($i:literal, $f:literal, $s:ty) => { Fixed<nat!($i), nat!($f), $s> }; }
pub type A = UFixed!(13, 3, Warm);
pub type C = UFixed!(4099, 0, Hot);
```

**The derivation runs end to end and the answers are right.** `139_probes/e2e.rs` takes two written literals,
adds them with the ladder's own structural `Add` (`137_probes/ladder.rs:320-345`), projects the container
through `Container` (`ladder.rs:378-389`), and asserts each result against `131:266-272`'s table:

```rust
is::<ufixed_container!(13, 3)>(PhantomData, PhantomData::<u16>);   // 16 bits
is::<ufixed_container!(13, 4)>(PhantomData, PhantomData::<u32>);   // 17 bits
is::<ufixed_container!(60, 4)>(PhantomData, PhantomData::<u64>);   // 64 bits
is::<ufixed_container!(60, 5)>(PhantomData, PhantomData::<u128>);  // 65 bits
is::<ufixed_container!(100, 28)>(PhantomData, PhantomData::<u128>);// 128 bits
```

Eight of eight, plus `ufixed_container!(4099, 0)` and `ufixed_container!(65537, 1)`, widths no table lists.

**And it erases.** Compiled at `-O`, the object file (`139_probes/e2e.s`) is:

```
_derived16:  add w8, w1, w0 ; and w0, w8, #0xffff ; ret
_derived64:  add x0, x1, x0 ; ret
_native16 = _derived16
_native64 = _derived64
```

LLVM's identical-code-folding collapsed the hand-written native functions into the derived ones and emitted
symbol aliases, so `native16` and `native64` have no bodies at all. That is the same outcome `135b:40-48`
records for the transparent probe, reached here **through the bridge**, so the crossing costs nothing at the
machine. The `and` at sixteen bits is the logical-width mask from section three, not an artifact of the
bridge.

Compile cost for the whole end-to-end crate is 0.13 seconds and the expansion is linear in the digit count,
which against `08_fog`'s measured 28.45 seconds for an eight-bit exhaustive check is not a number worth
arguing about.

### What this costs, stated plainly because it is not free

**A proc-macro crate.** arvo has none today; the agent instructions list `bitfield!` as a declarative macro
and the fifteen crates contain no proc macro. Adding one is a real structural change: a host-side
compile-time crate that uses `std`, sitting beside a `#![no_std]` stack. It is the same split notko already
documents and ships for `#[profile]`, so the workspace has the precedent and the framing, but arvo taking it
is op's call and not a detail.

**The bang.** `UFixed!(13, 3, Warm)` is not `UFixed<13, 3, Warm>`. I think this is the closest anything gets
without buying `generic_const_args`, and it is one character, but it is a change to D48's spelling and op is
the one who decides whether that counts as the literal spelling surviving.

**Inference and diagnostics are unmeasured.** A type written through a macro elaborates to the same type, so
I expect error messages to name `D1<D0<D1<D1<Term>>>>` rather than `13`, which is worse than the table's
diagnostic and is exactly the axis `134` compared the routes on. I did not measure it and it should be
measured before this is chosen, because a derivation that is invisible and produces unreadable errors is a
different kind of spelling-out problem.

**It is one probe by one party with no second read**, and the last thing in this position that was taken as
settled was `134c`, which had to say the same about itself.

### The alternative I looked at and am reporting because it lost

Before the expansion I tried to remove the need for a structural magnitude entirely, by keying a derived
numeral on its construction (`Product<A, B>` rather than `UFixed<{I1+I2}, {F1+F2}>`), so that no computed
width ever reaches type position. It works for the container, since the width is recoverable as an ordinary
associated const at value level, gate-free. It fails on normal form: `Product<UFixed<13,3>, UFixed<5,2>>` and
`UFixed<18,5>` denote one numeral and are two types, so every law, every impl and every signature doubles.
That is `36_kiselyov`'s subject and I did not re-derive it. Recording it so the next reader does not spend
the same hour.

## Do the three share a missing quantity

Half. And the half that is shared is sharper than the brief's hypothesis, while the half that is not is a
genuinely different kind of thing, so I think the useful answer is to stop treating them as three instances
of one problem.

**Two and three share one quantity and one fix.** Both compute a rounded quantity and then round again:

| | rounds twice | rounds once | the quantity between them |
|---|---|---|---|
| Stored width | `ceil(log2 sig) + ceil(log2 exp)` | `ceil(log2(sig * exp))` | layout slack, `2^(sig+exp) / card` |
| Warm container | `rung(rung_bits(W) + 1)` | `rung(W + margin)` | the rung's spare bits, `rung_bits(W) - W` |

They are the same quantity in two settings: **the unused capacity a rounding-up leaves behind, which the
second rounding then throws away instead of spending.** In the numeral it is the code space a per-field
layout wastes; in the container it is the bits a rung has above the width. In both cases the design pays for
the waste twice because it never named it, and in both cases the fix is one line: compose, then round.

Both fixes also land on a quantity **already in the typestate**. The layout slack is an `Encoding` fact and
`Encoding` is carried. The spare bits are `rung_bits(W) - W` and both terms are carried. Op's instinct at
`137b:39-41`, that the information is present and what is missing is the spelling, is right at two of the
three, and this is the seventh time.

**One does not fit.** It is not a rounding and there is no wasted capacity in it. Its missing thing is not a
quantity at all: it is that a notation change was being attempted in the wrong layer. Forcing it into the
shared frame would mean calling the structural magnitude the composed form and the literal the rounded one,
which is a pun rather than a finding.

There is a weaker shape all three do share, and it is worth stating because it is a review question rather
than a design: **in each case the design compensated for a derivation it did not have with a written
artifact, and in each case the artifact read as a reasonable design element rather than as a gap.** A
declared `StoredWidth`, a headroom rule inside a table, a bridge impl. None of them looks like something
missing. That is why they survived, and it is a better predictor of where the next one is than any of the
three quantities.

## What compiles

Every claim above, on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2024, probes in `139_probes/`.

The compiled positives: `type const` associated constants reach const-argument position under
`min_generic_const_args` alone (`p1d.rs`); `137_probes/ladder.rs` builds standalone with no feature gate; the
expansion reproduces all thirteen of 137's hand-written aliases as identical types (`consumer.rs`); the
container derives correctly from written literals at eight widths checked against `131`'s table plus two
widths no table lists (`e2e.rs`); the expansion is uncapped to two to the sixtieth (`big.rs`); and the
derived operation folds to the native one with symbol aliases at `-O` (`e2e.s`).

The compiled negatives: an arithmetic `type const` RHS is refused three ways, with the diagnostic naming
`generic_const_args` (`p1b.rs`, `p1c.rs`, `p1e.rs`); and Warm's `u128` form at 64 logical bits is a rolled
scalar loop where the `u64` form autovectorises to thirty-two `add.2d`, roughly 1600 instructions against 81
over sixty-four elements (`d_warm_codegen.rs`, `d_warm.s`).

The computed results: the layout-slack criterion `2^(sig+exp) >= 2 * card` decides the overshoot at all
eleven formats in integer arithmetic with no logarithm (`a_residue.py`); that criterion is equivalent to the
multiplicative slack reaching two at all eleven, and the significand slack is exactly one at radix two and
only there (`b_slack_is_one_quantity.py`); 131's headroom rule widens 64 of the 64 widths at or below 64
bits, composing first widens 4, and no container headroom widens none (`c_warm_rung.py`).

The unmeasured claims, flagged so they are not read as compiled: that Precise's saturating semantics survive
the loss of container headroom via `overflowing_add`; that the expansion's diagnostics are worse than the
table's; and that the multiplicative slack subsumes `138`'s FL8 rather than merely agreeing with it at the
eight binary formats.

## What is op's

**Two is a correction rather than a call, and it needs no decision.** The joint form is already what `138`
compiled at eleven of eleven. What this file adds is that the per-field form is not wrong, it answers an
`Encoding` question, and the two want to be two named widths rather than one derivation with an exception.
The one thing that does need op is the backwards inequality at `110:3248`, where I cannot tell which of the
two `W_F` denotes.

**Three is a call and it is his.** Deleting Warm and Precise's rung of headroom changes a rule `131` states
and `137b:84-85` reopened. The replacement is stated above, it is right by default rather than when
configured, and it moves Warm's crossover from 65 bits to 129. I would take it. But it rests on the claim
that container headroom buys nothing the hardware does not already give, and that claim is compiled for Warm
and argued for Precise, so the Precise half is owed a probe before the rule is written as covering all four.

**One is a call about what arvo is willing to be.** The expansion closes the bridge with no table, no cap and
no feature, and it erases. It costs arvo a proc-macro crate and it costs D48 one character. Neither of those
is mine to spend. If op will not take the proc-macro crate, my honest reading is that the crossing does not
close under the allowed features, and the least-bad residue is the extensible bridge at `134c` with arvo's
shipped range framed as a convenience rather than a limit. I would rather he saw the expansion first, because
it is the option nobody had put in front of him.

**And the shared-quantity answer is half, which I would rather report than round up to a yes.** Two of the
three are one problem with one fix. The third is not, and the reason the three felt alike is that all three
compensated for a missing derivation with a written artifact that read as a design element. That pattern,
rather than any of the three quantities, is what I would go looking for next.
