# 130. The surface without compromise: the numeral was one coordinate short

**Persona:** Oleg Kiselyov, typed-embedding and interpretation lens. Fourth pass in this panel; file 02 read
the type-level encoding, file 36 read the normal form and its price, file 54 read the type-level float and
decimal, file 76 read the consumer price, file 104 read the bitfield, and file 119, mine, has since been
overturned on its load-bearing clause and I am not defending it.
**Date:** 2026-08-07
**Position:** after `129_jhala_canonicity_settled.md`. Reads `126`, `127b`, `128`, `129`, and `110` where it
bears on the numeral and the surface.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository. `mock/crates` untouched, `mock/design_rounds/` untouched. Probes at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/kis130/`,
twenty-three files named `a1` through `n2`, roughly in the order they appear here.

Op declined three ways out of a collision and said the design should already have what it needs in the
typestate, missing only its spelling out. He is right, and the missing spelling is one word long: the numeral
is a **format**, and a format is two coordinates, and file `129` gave it one.

`Fx<const P: u32, C, S>` keys the numeral on precision alone. Under it, `UFixed!(13, 3, u16, Warm)` and
`UFixed!(8, 8, u16, Warm)` are one type, which is what makes canonicity structural, and it is also what makes
the surface unwritable, because two numbers must be squeezed into one parameter and the squeezing is an
addition with no legal home. Both facts have the same cause. Restore the second coordinate and both go away
at once: the consumer's two numbers are the type's two parameters, the alias has nothing left to compute, and
there is no macro, no re-reading, no feature gate, and no `-Z` flag.

The whole surface compiles at `e1_capstone.rs`, exit 0, in plain type syntax:

```rust
let a: UFixed<13, 3, u16, Warm> = Fixed::zero();
let p: UFixed<26, 6, u32, Warm> = mul(a, a);
```

That is not a fourth compromise. It is cheaper than all three offered, and it also removes a defect the
precision-keyed numeral ships that nobody has named: **under it, a Q13.3 datum and a Q8.8 datum are
interchangeable, so the type system cannot refuse a scale mismatch, and file `129`'s own capstone performs one
on line 60 with a comment saying so.** Section 2 compiles a twenty-line program in which one, encoded as
Q13.3, decodes to thirty-two, at exit 0.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo yet; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts the intent and op's own calls in the governing position. The
governing calls here are op's canonicity ratification (`127b:22-31`), D48's surface (`127b:56-59`), the
convergence pressure (`127b:12-18`), and D69's overturn at `30b`, recorded at `110:869-873`, that identity is
parameterised in mathematical coordinates. Section 8 reports that my answer contradicts `127b:24` read
literally, states the reading under which it does not, and hands the call back rather than making it. Nothing
else here reopens anything.

**Test gate.** Not run, and I am naming that rather than letting it pass. `126:47-53` ran
`cargo test --offline --workspace` this pass and got 155 binaries, 672 passed, 0 failed, 9 ignored, on a tree
nothing has moved since; `129:47-52` declined to re-run it for the reason op ruled at `108b:174-181`, that a
further report of the same collected tautologies is what that ruling exists to stop. My deliverable touches no
crate in that tree and proposes replacing the type the suite covers. The instrument that measured anything
here was the compiler, twenty-three times.

**The brief's factual claims, checked before reasoning from them.**

*"`pub type UFixed<const I, const F, C, S> = Fx<{ I + F }, C, S>` is refused."* **Holds**, and the help line
is worth having because it names the forbidden feature rather than the watched one (`a1_alias.rs`):

```
error: generic parameters may not be used in const operations
10 | pub type UFixed<const I: u32, const F: u32, C, S> = Fx<{ I + F }, C, S>;
   |                                                          ^ cannot perform const operation using `I`
   = help: const parameters may only be used as standalone arguments here, i.e. `I`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

*"`associated_const_equality` was removed in 1.94.0 and merged into `min_generic_const_args`, so a relation
stated as a `where` clause works today. It fails only when the associated const must be a sum."* **Holds**,
reproduced independently against my own surface rather than `129`'s (`g1_precbound.rs`, then `g2_typeconst.rs`):

```
error: use of trait associated const not defined as `type const`
8 | pub fn wants_precision_16<A: Format<PRECISION = 16>>(_: A) {}
  = note: the declaration in the trait must begin with `type const` not just `const` alone
```

```
error: complex const arguments must be placed inside of a `const` block
8 | impl<const I: u32, const K: u32> Format for F<I, K> { type const PRECISION: u32 = I + K; }
  |                                                                                   ^^^^^
```

Section 6 says what this costs under my surface, which is less than it costs under `129`'s, for a reason that
is the file's argument in miniature.

*"The error reaches the caller rather than the author"*, of the post-monomorphisation hole (`129:305-309`,
restated in the brief). **False as stated, and the correction changes what the diagnostic work is.** rustc
attributes the failure to the innermost erroneous instantiation site, which is the wrapper's own body, in the
wrapper's own file, including across a crate boundary (`xc/downstream.rs`, section 9):

```
note: the above error was encountered while instantiating `fn arvolib::mul::<13, 3, 13, 3, 13, 3, u16, u16, Warm>`
 --> arvolib.rs:8:5
```

The author is named. What the consumer loses is not attribution but **their own line**, which appears nowhere
in the diagnostic. That is a different problem with a different fix, and section 9 designs for the real one.

*"`generic_const_args` does not deliver canonicity under a generic parameter."* **Not re-run.** `128:210-223`
and `129:154-168` compiled it twice from two directions and I have no reason to make it three. It does not
bear on anything below, because nothing below needs the feature.

---

## 1. The premise nobody attacked, and it is written down

Three converged conclusions in this panel have fallen to someone asking why an assumption was there, so before
answering I went looking for the assumption. It is in the standing document, stated flatly, twice, and never
questioned in six thousand lines.

`110:5353-5357`, the open-question list:

> **How a public const width reaches a type-level `Nat`.** D48 and D31 keep `UFixed<13, 3, Warm>` spelled
> exactly that way, with the width a const parameter publicly; the numeral it composes over needs `Precision`
> as a `Nat`; and `I + F` in type position is a const expression, which section 1.2 has already compiled shut
> in both directions.

And `110:3491-3495`, saying the same thing from the alias side:

> D53 makes `UFixed`, `IFixed`, `FastFloat` and `StrictFloat` four names for four compositions over `Number`,
> D48 and D31 keep the public spelling `UFixed<13, 3, Warm>` with its widths as const parameters, and the
> numeral inside the composition takes `Precision` as a `Nat`. **Those two facts do not meet on their own**.

The unexamined clause is **"the numeral needs `Precision` as a `Nat`"**, which the identity contract states as
`type Precision: Precision; // significand digit count, primitive (D69), a Nat` (`110:913`).

Why a `Nat`? Because the tower did type-level arithmetic and needed a type to do it on. `126:294-335` removed
that reason, compiled, and my own file `119:46-51` had asserted it as the whole case for the bridge and was
wrong. Why still a `Nat` after the tower went? Because the identity contract's other three members are
associated types and uniformity is comfortable. That is the entire justification, and it is not one.

D69's overturn at `30b`, which op made, says something narrower than the contract encodes:

> identity is parameterised in mathematical coordinates, not encoding coordinates. Precision and the exponent
> bounds are primitive; total width, the hidden bit, and field encoding are derived on the physical side
> (`110:869-873`).

That is a call about **which coordinates**, mathematical rather than physical. It is not a call about **what
kind of thing a coordinate is**, and reading "primitive" as "an associated type" is a step the checkpoint does
not take. A coordinate can be primitive and be a const. `126:112-114` states the rule that makes the
difference matter, in the compiler's own words: *a const may be carried and it may be read, it may not be
transformed on the way into a type.* Precision as a type is a transformation. Precision as a read is not.

So the collision `110` has carried as an open step is not between D48 and canonicity. It is between D48 and a
representation choice that outlived its reason, and the choice is the thing to move.

---

## 2. What the precision-keyed numeral costs, compiled

Before proposing anything I want the cost of the incumbent proposal on the record, because it is larger than
an ergonomic note and nobody has stated it.

`129`'s numeral is `Fx<const P: u32, C: Container, S>` and its surface macro expands `UFixed!(13, 3, u16, Warm)`
to `Fx<{13 + 3}, u16, Warm>`. **The 3 is discarded at expansion.** After it, nothing in the type says where the
binary point is. Its own capstone records the consequence as a property, `q13_capstone.rs:58-60`, quoted at
`129:394-395`:

```rust
let a: UFixed!(13, 3, u16, Warm) = Fx::new(0);
// 8 and 8: DIFFERENT scaling, SAME precision, SAME type as the above.
let b: UFixed!(8, 8, u16, Warm) = a;
```

That assignment is a rescaling performed by writing nothing. Here is what it costs, in a program that runs the
arithmetic instead of describing it (`a2_scale_collapse.rs`, `129`'s numeral copied unmodified, **exit 0**):

```rust
// Q13.3 one is raw 1<<3 == 8.  Q8.8 one is raw 1<<8 == 256.
pub const ONE_Q13_3: UFixed!(13, 3, u16, Warm) = Fx::new(1u16 << 3);
pub const ONE_Q8_8:  UFixed!(8,  8, u16, Warm) = Fx::new(1u16 << 8);

// The scale is not a function of the type, so a decode has to be told it.
pub const fn decode_q13_3(x: Fx<16, u16, Warm>) -> u32 { x.raw as u32 * 1000 / 8 }
pub const fn decode_q8_8 (x: Fx<16, u16, Warm>) -> u32 { x.raw as u32 * 1000 / 256 }

pub const A: u32 = decode_q13_3(ONE_Q13_3); // 1000, right
pub const B: u32 = decode_q8_8 (ONE_Q8_8);  // 1000, right
pub const C: u32 = decode_q13_3(ONE_Q8_8);  // 32000, wrong, and accepted
pub const D: u32 = decode_q8_8 (ONE_Q13_3); //    31, wrong, and accepted

const _: () = assert!(C == 32000);  // one, decoded as thirty-two
const _: () = assert!(D == 31);     // one, decoded as three hundredths
```

Both wrong values are asserted at compile time and both assertions pass, which is the point: the compiler
agrees with me about what the program computes. All four calls type-check because all four arguments have one
type. **A fixed-point library whose type system cannot refuse a scale mismatch has not typed the thing it is
for.**

I want to be exact about what this does and does not say, because `129` is a good file and its mechanism is
the one I am adopting. Its three parts are all correct: carry the container and read it, keep the arithmetic
in value position where equality is semantic, take the output as a parameter and state the relation as a
check. Section 2 of that file, the inversion against `generic_const_args`, is the sharpest thing written in
this panel and I am not disturbing a line of it. What is wrong is one choice inside a correct mechanism:
**which coordinates the parameter set names.** The mechanism is indifferent to that choice. The design is not.

There is also a reading on which this defect is not `129`'s at all, and it is the fair one. Its brief asked
for canonicity, it produced canonicity, and its section 5 explicitly hands the surface collision back as op's.
`Fx` may have been a vehicle rather than a claim. But its capstone is titled the whole mechanism, its consumer
section presents the collapse as canonicity working, and the brief I was handed presents it as settled, so the
correction is owed somewhere and this is where.

---

## 3. The surface

`e1_capstone.rs`, **exit 0, no `#![feature]`, no `-Z` flag, no macro**. Four parts, each individually sound,
which is the decomposition op asked for. Every one of them is `126`'s two-door rule or `129`'s inversion
applied at a different place.

**Part one, the container ladder.** Unchanged from `128:151-159` and `129:91-102`. Five rungs the hardware has,
carried by the consumer and read by the design, never derived. Section 7 says why derivation is unavailable and
why that is the right answer anyway.

**Part two, the numeral. The two numbers the consumer writes are the type's two parameters.**

```rust
pub struct Fixed<const I: u32, const F: u32, C: Container, G: Sign, S: Policy + Lowering> {
    raw: C,
    _m: PhantomData<(G, S)>,
}
pub type UFixed<const I: u32, const F: u32, C, S> = Fixed<I, F, C, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, C, S> = Fixed<I, F, C, Signed, S>;
```

Those two aliases are generic and they are legal, because every argument is standalone. That is door one, and
it is the door the arithmetic was blocking.

**Part three, the mathematical coordinates, as reads.** This is the part that was missing.

```rust
impl<const I: u32, const F: u32, C: Container, G: Sign + SignBits, S: Policy + Lowering> Format
    for Fixed<I, F, C, G, S>
{
    const PRECISION: u32 = G::EXTRA + I + F;   // door two: an associated const body
    const EXPONENT: i32 = -(F as i32);
    const INTEGER_DIGITS: u32 = I;
    const FRACTION_DIGITS: u32 = F;
    type Store = C;
}
```

D69's coordinates are all present and all primitive in the sense op's checkpoint means: they are what the type
determines, not what an encoding determines. They are consts rather than types, which is the one thing that
changed and the only thing that had to.

Compiled, at concrete instantiation, gate-free:

```rust
const _: () = assert!(<UFixed<13, 3, u16, Warm> as Format>::PRECISION
                   == <UFixed<8,  8, u16, Warm> as Format>::PRECISION);
const _: () = assert!(<IFixed<12, 3, u16, Warm> as Format>::PRECISION == 16);
const _: () = assert!(<UFixed<40, 30, u128, Warm> as Format>::PRECISION == 70);
const _: () = assert!(<UFixed<13, 3, u16, Warm> as Format>::EXPONENT == -3);
```

**Part four, the laws.** `129`'s output-parameter shape, one coordinate wider, with one addition that section
6 says is the real gain.

```rust
/// Alignment is an equality between coordinates, so it is a bound: the two
/// arguments share the parameter F.
pub fn add<const I: u32, const J: u32, const F: u32, const M: u32, ..>(
    a: Fixed<I, F, C, G, S>, b: Fixed<J, F, C, G, S>,
) -> Fixed<M, F, D, G, S> {
    let () = SumFormat::<I, J, M>::HOLDS;
    let () = Fixed::<M, F, D, G, S>::FITS;
    ...
}

/// The product's coordinates are sums, so the relation is a check.
pub fn mul<const I: u32, const F: u32, const J: u32, const K: u32,
           const M: u32, const N: u32, ..>(
    a: Fixed<I, F, C, G, S>, b: Fixed<J, K, C, G, S>,
) -> Fixed<M, N, D, G, S> {
    let () = ProductFormat::<I, F, J, K, M, N>::HOLDS;
    let () = Fixed::<M, N, D, G, S>::FITS;
    ...
}
```

And `rescale`, which is the operation that exists because the exponent is in the type. Under a precision-keyed
numeral it is an assignment, invisible, unwritten, and wrong. Under this one it has a name and the consumer
writes it.

---

## 4. What a consumer writes

From `e1_capstone.rs`, the consumer section, verbatim and compiling:

```rust
let a: UFixed<13, 3, u16, Warm> = Fixed::zero();
let b: UFixed<13, 3, u16, Warm> = Fixed::zero();
let c: UFixed<8, 8, u16, Warm> = Fixed::zero();
let _wide: UFixed<40, 30, u128, Warm> = Fixed::zero();
let _tiny: UFixed<3, 0, u8, Warm> = Fixed::zero();
let _frac: UFixed<0, 8, u8, Warm> = Fixed::zero();
let _sgn: IFixed<12, 3, i16, Warm> = Fixed::zero();

let p: UFixed<26, 6, u32, Warm> = mul(a, b);
let s: UFixed<14, 3, u32, Warm> = add(a, b);   // 17 bits: u16 is refused
let r: UFixed<8, 8, u16, Warm> = rescale(a);   // the scale change is written
let _bv: Bits<16, u16, Warm> = bits_of(a);
```

Plain type syntax, D48's two numbers, arbitrary widths, no table, no cap, no exclamation mark. The output
coordinates infer from the annotation, and from a callee's parameter type with no annotation at all
(`a3_surface.rs:132-134`, `wants_q26_6(mul(a, b))`).

That `add` line is worth its comment. I wrote `u16` first and the design refused me:

```
error[E0080]: evaluation panicked: arvo: the format does not fit its container.
   evaluation of `Fixed::<14, 3, u16, Unsigned, Warm>::FITS` failed here
note: the above error was encountered while instantiating `fn add::<13, 13, 3, 14, u16, u16, Unsigned, Warm>`
```

Adding two Q13.3 gives Q14.3, seventeen bits, which does not fit a `u16`. I had not noticed. The check had.

### What they see when they get it wrong

**A scale mismatch, refused where it is written, before any monomorphisation** (`n1_misaligned.rs`). This is
the case `129`'s numeral cannot express at all:

```
error[E0308]: mismatched types
 --> n1_misaligned.rs:5:47
  |
5 |     let _s: UFixed<14, 3, u32, Warm> = add(a, b);
  |                                        ---    ^ expected `3`, found `8`
  |                                        |
  |                                        arguments to this function are incorrect
  |
  = note: expected struct `Fixed<_, 3, _, _, _>`
             found struct `Fixed<8, 8, _, _, _>`
```

`expected 3, found 8` names the exponent coordinate directly, at the consumer's line, in a plain `E0308` with
no const evaluation involved. The same shape refuses `129`'s line 60 assignment (`a4_neg_scale.rs`):
`expected 8, found 13`.

**A wrong output format, refused at const eval, with the law named** (`e2_neg.rs`, section 9 designs the message):

```
error[E0080]: evaluation panicked: arvo: the product's format does not follow from its inputs.
   evaluation of `ProductFormat::<1000, 1000, 1200, 1300, 2201, 2300>::HOLDS` failed here
note: the above error was encountered while instantiating
      `fn mul::<1000, 1000, 1200, 1300, 2201, 2300, u128, u128, Unsigned, Warm>`
```

**A format that does not fit its container** (`d1_perimeter.rs`), naming the offending type with its own
coordinates:

```
error[E0080]: evaluation panicked: arvo: the format does not fit its container.
  ...
   evaluation of `UFixed::<9, 0, u8, Warm>::FITS` failed here
```

**An output format that cannot be inferred** (`n2_noannot.rs`). This is the ergonomic cost and I am not going
to soften it:

```
error[E0284]: type annotations needed for `Fixed<_, _, _, Unsigned, Warm>`
5 |     let c = mul(a, b);
note: required by a const generic parameter in `mul`
help: consider giving `c` an explicit type, where the value of const parameter `M` is specified
5 |     let c: Fixed<M, N, D, _, _> = mul(a, b);
```

It fires when an intermediate is bound with no annotation and no typed use, and it is one notch worse than
`129:413-421`'s because two coordinates and a container must be named instead of one. It is the price of not
deriving, it is the same price `129` pays, and it does not fire when the result is used in a typed position,
which in real code is most of the time. I have not measured how often that is, and I am not going to guess.

---

## 5. The perimeter, stated where the guarantee is stated

`what-you-can-observe-is-what-you-guaranteed.md` asks for the observation surface to be named alongside the
property, so here it is, checked rather than asserted.

An over-wide format is **nameable**. `pub type Bad = UFixed<9, 0, u8, Warm>;` and `fn f(x: Bad) -> Bad { x }`
compile, exit 0 (`d1_perimeter.rs`, first half). Rust has no way to attach a const check to a type's mere
existence, so this is not closable.

It is **not inhabitable**, and that is closable and closed. Every route to a value forces `FITS`: the
constructors do it (`from_raw`, `zero`), and every law forces it on the type it returns
(`let () = Fixed::<M, N, D, G, S>::FITS;` in `mul`, `add`, `widen_int`, `rescale`). So the illegal state is a
name with no inhabitants, which is the strongest statement available and it should be the sentence in the
canon rather than "the fit is checked".

The perimeter members and what each permits: `to_raw` hands out the container, which carries no format
invariant and is the declared unwrap door; `Format::Store` is an associated type equality, which is a bound and
therefore checkable at a signature; the four projections are consts and hand out numbers. The struct's field is
private. Nothing hands out a `Fixed` whose fit was not established.

---

## 6. What is a bound and what is a check, exhaustively

The panel has been treating "checked at declaration" and "checked at instantiation" as a single fork settled by
whether `generic_const_args` is adopted. It is not one fork. It is a per-relation question, and under this
surface more relations fall on the good side, which is the concrete gain from keeping the second coordinate.

| Relation | Mechanism | When |
|---|---|---|
| Two numerals have the same format | shared parameters, `Fixed<I, F, ..>` twice | declaration |
| Two numerals have the same exponent (alignment) | shared parameter `F`, `add`'s signature | declaration |
| Two numerals share a container | associated type equality, `Format<Store = u16>` | declaration |
| A numeral has a given exponent | shared parameter against a literal | declaration |
| Two numerals have the same precision | not statable as a bound | const eval |
| An output coordinate is a sum of input coordinates | `const` assert in a named law item | const eval |
| A format fits its container | `const` assert forced at every constructor | const eval |

The first four are new. Under a precision-keyed numeral, alignment is not a relation at all, because the
exponent is not in the type, so it cannot be a bound and it cannot be a check either: it is unstatable, which
is section 2. Under this surface it is the cheapest kind of guarantee the language has, an equality between
parameters, checked by ordinary unification with no const evaluation, no post-monomorphisation deferral, and an
`E0308` at the consumer's own line.

The fifth row is the near miss `129:246-263` found and I reproduced at section 0. It costs less here than
there, and the reason is worth stating because it is the file's argument again: **under precision keying, "same
precision" is the only agreement there is, so losing it as a bound loses everything; under format keying, "same
format" is the agreement a consumer actually wants and it is a bound.** Precision agreement is what you want
when talking about storage, and storage agreement is row three, which is also a bound.

The sixth and seventh rows are the residue, and section 9 is about them.

---

## 7. The exhaustion, and why enumerating positions was never going to work

The brief lists directions to open and asks me to exhaust the space rather than agree with the previous file. I
opened them, and every one arrives at one sentence, which is the sentence `126:105-110` already extracted and
which I now think is the whole content of this question.

| Position tried | Probe | Result |
|---|---|---|
| generic type alias, `type U<I, F, ..> = Fx<{I+F}, ..>` | `a1_alias.rs` | refused |
| newtype field, `struct U<I, F, ..>(Fx<{I+F}, ..>)` | `c1_newtype.rs` | refused |
| trait associated type, `type Of = Fx<{I+F}, ..>` | `c2_assoctype.rs` | refused |
| inherent associated type, `pub type Of = Fx<{I+F}, ..>` | `c3_inherent.rs` | refused |
| turbofish at a call, `mul::<.., {I+I}, {F+F}, ..>` | `b1_turbofish.rs` | refused |
| `type const` reduced to a path over a computing assoc const | `126:131-151` | refused |
| assoc-const-equality bound with a computed const | `g1`, `g2` | refused |

Seven positions, one diagnostic, verbatim in all seven:

```
error: generic parameters may not be used in const operations
  = help: const parameters may only be used as standalone arguments here, i.e. `I`
```

Add `126`'s four and `129`'s five and it is sixteen compiled refusals across four files. **The rule is
quantified over positions, so enumerating positions cannot find an exception**, and continuing to look is
looking for a counterexample to a universal. Anyone who reaches for an eighth position should read the help
line instead: it does not say "not here", it says "only as standalone arguments", which is a statement about
every here there is.

There is exactly one way past a universally quantified refusal, and it is not a position. It is to not need the
operation. Every construction in this panel that works does that and nothing else: `Capacity` does it by
keeping the const standalone (`126:73-117`), `128`'s ladder does it by having the consumer name the container,
`129`'s laws do it by taking the output as a parameter, and this file does it by having the consumer name the
coordinates the type is keyed on. Four instances of one move.

**The container is the same move a fifth time, and it explains why three independent files landed on carrying
it.** Selecting a type from a const requires a case split on the const's value; a case split on a const value is
one impl per value; the values here are the widths; and one impl per width is the enumeration op refused at
`127b:36-50`. So the consumer supplies the case split by naming the rung, and the design checks the fit. That
is not a workaround, it is `arvo-toolbox-not-policer.md`'s posture arriving as the only available mechanism:
"the consumer names the container and the design checks the fit rather than choosing for them"
(`129:436-438`).

---

## 8. Canonicity, and the ratified sentence

My answer contradicts `127b:24` read literally, so I am going to be careful.

Op ratified: *"Two numerals of equal precision are the same type."* The failure the ratification names:
*"a numeral reached by `13 + 3` and one reached by `8 + 8` are distinct types with equal precision, and the
compiler reports `E0308` where a consumer expects agreement."*

Under this surface, `UFixed<13, 3, u16, Warm>` and `UFixed<8, 8, u16, Warm>` are distinct types with equal
precision. Read literally, that is the thing refused.

Three observations, and then the call is his.

**One. The sentence describes a defect of the tower, and the tower is gone.** The `E0308` it names is
`126:238-243`'s, verbatim:

```
error[E0308]: mismatched types
11 | pub fn assign(x: Number<Sum<W<13>, W<3>>>) -> Number<W<16>> { x }
   = note: expected struct `Number<W<16>>`
              found struct `Number<Sum<W<13>, W<3>>>`
```

That is one format with two spellings, `Sum<W<13>, W<3>>` and `W<16>`, refusing to unify. It is a real defect
and canonicity is the right name for its absence. Under this surface it cannot arise, because a format has
exactly one spelling: the coordinates the consumer wrote. There is no route by which a consumer writes one
format two ways and is refused. `126:436-440` posed the question as newly well-posed and said "I did not answer
it, because the record contains nothing to answer it from". The record now contains `a2_scale_collapse.rs`, and
what it answers is that the two spellings problem and the two formats problem were being read as one question.

**Two. Canonicity holds, in both places it is wanted, and neither is the one the sentence quantifies over.**

The numeral is canonical: one type per format, reached however you like, at every level of genericity. Compiled
three spellings deep (`e1_capstone.rs`, `canonical`), and under a generic parameter where
`generic_const_args` fails (`e1_capstone.rs`, `interchange`): two composed widenings and one combined widening
both land in `UFixed<M, F, C, S>`, because `M` is a parameter rather than an expression. That is `129:169-194`'s
argument, undisturbed, one coordinate wider.

The precision is canonical: it is a const value, and `13 + 3` and `8 + 8` are `16` in value position, compared
semantically rather than definitionally. Compiled at section 3.

And the agreement the sentence is reaching for, that a Q13.3 and a Q8.8 are the same sixteen bits, holds as
**type identity of the container**, which is a bound (`e1_capstone.rs`, `same_store` and `agree`):

```rust
pub fn same_store<A: Format<Store = u16>, B: Format<Store = u16>>(_: A, _: B) {}
pub fn agree(a: UFixed<13, 3, u16, Warm>, b: UFixed<8, 8, u16, Warm>) { same_store(a, b); }
```

So the design gets the agreement, at the storage, where precision is the subject, checked at a signature.

**Three. The literal reading is unsound as a numeric statement**, and section 2 is the compiled proof. If two
formats of equal precision are one type, `rescale` cannot exist, alignment cannot be checked, and a decode is
not a function of a type. That is not an ergonomic loss, it is the library failing to type its own subject.

The call is op's, it is a change to a sentence he ratified fourteen hours ago, and by his own staleness
principle (`108b:11-20`, applied by him at `127b:105-107`) the evidence has moved: `129` compiled that
`generic_const_args` does not deliver the sentence, and this file compiles what delivering it costs. I would
restate the requirement as: **a numeral's type is determined by its format and by nothing else, in particular
not by the route taken to it.** That is what the ratification was protecting, it holds structurally here, and
it does not say that two different formats are one type.

---

## 9. The post-monomorphisation hole, made legible

Op ruled that the hole is monomorphisation working as intended, that closing it is not the task, and that
proper diagnostics are. Four findings, three of them better than the brief expects.

### The author is already named, and the consumer's line is not

Cross-crate, with the wrapper in the library and the call downstream (`xc/arvolib.rs`, `xc/downstream.rs`), the
tail of the diagnostic reads:

```
note: the above error was encountered while instantiating `fn arvolib::mul::<13, 3, 13, 3, 13, 3, u16, u16, Warm>`
 --> arvolib.rs:8:5
  |
8 |     mul::<I, F, I, F, I, F, C, C, S>(x, x)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

The library's own line, in the library's own file. Same-crate it is the same (`a5b_caller.rs`). So the
attribution is right and `129:305-309`'s reading of it is not.

What the consumer sees is an error every span of which is in someone else's crate, and **no span of their own**.
The chain is truncated to the innermost instantiation, so the call they wrote is not shown. In a large crate
they cannot find it from the diagnostic. That is the real cost and it is what the message has to answer.

One cosmetic degradation to know about: cross-crate, the primary span moves into libcore,
`library/core/src/panic.rs:62:9`, because the assertion's span does not survive the crate boundary. Same-crate
it points at the design's own const. Nothing in the design can change this.

### Name the law, so the failing instantiation prints as the law

An anonymous `const {}` block reports as `mul::<13, 3, 13, 3, 13, 3, u16, u16, Warm>::{constant#0}`, which
mixes the law's coordinates with the container and the strategy and gives the block a number. A named item
reports as the law (`xc/lawlib.rs`, and `e2_neg.rs` for the four-digit case):

```
evaluation of `ProductFormat::<1000, 1000, 1200, 1300, 2201, 2300>::HOLDS` failed here
```

Six numbers, in the law's own order, nothing else. The consumer can read the violated relation off the line and
can **search their own source for the last two numbers**, which is the only handle they have on finding their
call site. That is why the law is a named struct with an associated const rather than a const block, and it
costs one declaration per law.

### The message, compiled

`assert!`'s second argument is a format string, so a message containing `{` fails to build with
`error: invalid format string: the '+' sign flag must appear after ':'` (`xc/lawlib.rs`, first attempt). Braces
must be doubled or avoided. Const evaluation supports no formatting, so no value can be interpolated; the values
come from rustc's own instantiation line, which is why naming the law matters.

The message this produces, verbatim from `xc/lawdown.rs`:

```
error[E0080]: evaluation panicked: arvo: the product's format does not follow from its inputs.
                The law: Fixed<I, F> times Fixed<J, K> has format Fixed<I + J, F + K>.
                The line above prints ProductFormat::<I, F, J, K, M, N> with the actual
                digit counts, in that order.
                If you wrote the call, name the output with those first four numbers added
                pairwise. If the call is inside a function you did not write, that function
                states a format relation that does not hold, and the note below names the
                function and the line. Search your own source for the two output numbers
                printed above to find which of your calls reached it.
```

It serves both readers because the check cannot tell them apart: the const block is evaluated at
monomorphisation and has no way to know whether the caller is the author. So it names both cases, tells the
consumer which spans to trust, and gives them the one search that finds their line.

`#[diagnostic::on_unimplemented]` is not available here and I checked why rather than assuming: it attaches to
trait bounds, and the relation cannot be a bound, which is section 6's fifth row and the entire reason the check
is post-monomorphisation. There is no version of this that becomes a trait obligation without the arithmetic.

### The hole is closable by a finite obligation, and this is the finding I did not expect

A generic wrapper **cannot compute its output coordinates** (`b1_turbofish.rs`):

```
error: generic parameters may not be used in const operations
8 |     mul::<I, F, I, F, { I + I }, { F + F }, C, C, S>(x, x)
```

The rule that caused all of this closes the hole. A wrapper may supply, for each output slot, only one of its
own const parameters or a literal. That is the entire space, and it partitions:

- **The output slot is a parameter the wrapper's caller supplies.** The wrapper threads the coordinate and
  makes no claim; the check is deferred to its caller, correctly, and there is no hole.
- **The output slot is a parameter also used as an input coordinate**, `square_wrong`'s `M := I`. The wrapper
  claims the relation is an identity, which no law's relation is. One instantiation at a non-degenerate point
  refuses it.
- **The output slot is a literal.** The wrapper claims the relation is constant. Two instantiations at points
  differing in the relevant input coordinate refuse it. Compiled: a wrapper returning `UFixed<26, 6, ..>` for
  every input passes a witness at 13.3 and fails one at 7.2 (`b2_twowitness.rs`, exit 0 then
  `evaluation of mul::<7, 2, 7, 2, 26, 6, ..>` failing).

So a fixed, small set of instantiation witnesses refuses every expressible wrong wrapper. The witness must be a
**call from a concrete function**; naming the function item does not force its consts and quietly proves
nothing (`witness.rs`, exit 0, against `witness2.rs`, which fires). The capstone carries three:

```rust
#[doc(hidden)]
pub mod witnesses {
    use super::*;
    pub fn w_a(x: UFixed<13, 3, u32, Warm>) -> UFixed<27, 3, u32, Warm> { widen_twice::<13, 3, 7, 7, 20, 27, u32, Warm>(x) }
    pub fn w_b(x: UFixed<5, 11, u32, Warm>) -> UFixed<9, 11, u32, Warm> { widen_twice::<5, 11, 1, 3, 6, 9, u32, Warm>(x) }
    pub fn w_c(x: UFixed<1, 0, u8, Warm>) -> UFixed<7, 0, u8, Warm> { widen_twice::<1, 0, 2, 4, 3, 7, u8, Warm>(x) }
}
```

That converts a hole into an obligation: **one line per public generic wrapper, discharged in the crate that
declares it, failing that crate's own `cargo check`.** It does not protect a consumer's own wrappers, which are
theirs to witness, and it does not make the relation a bound. It does mean arvo cannot ship the bug, which is
the case the diagnostic was being designed for.

I have proved the space of expressible claims is {parameter, literal} and that these three shapes are refused
at two points each. **I have not verified that any specific witness triple is adequate for every law in the
design**, and for a maximum-shaped relation like `add`'s it needs one witness on each side of the maximum. That
selection is mechanical and I did not do it for the whole law set; it belongs with the laws when they are
written.

---

## 10. Four families interpreting one contract, which is what D53 was reaching for

One consequence of moving precision from a type to a read, worth stating because it closes an open step rather
than opening one. `110:3492-3495` records D53 as "four names for four compositions over `Number`" with no
stated expansion, and `110:5353-5357` lists the missing expansion as an open question. The expansion does not
exist, and the reason is now visible: an alias forces one struct to serve four families in one basis, and the
change of basis is the arithmetic with no home.

Drop the requirement that they be aliases and they become four interpretations of one signature, each written in
the basis natural to its kind (`h1_four_interpretations.rs`, exit 0, gate-free):

```rust
pub trait Numeral {
    const RADIX: u32;  const PRECISION: u32;  const EMIN: i32;  const EMAX: i32;  const SIGNED: bool;
    type Exponent: ExponentForm;   // the kind is the type; the value is a read
    type Store: Container;
}
```

`UFixed<13, 3, u16, Warm>` and `IFixed<12, 3, u16, Warm>` read their precision as `I + F` and `1 + I + F`;
`FastFloat<24, -126, 127, u32, Warm>` reads it as `P` and carries `Ranged`; `Decimal<16, -398, u64, Warm>`
carries radix ten. Every one reads as itself, which is D48 and D31's constraint, and a generic algorithm binds
`N: Numeral` and never a family. Compiled, gate-free:

```rust
const _: () = assert!(<FastFloat<24, -126, 127, u32, Warm> as Numeral>::PRECISION == 24);
const _: () = assert!(<Decimal<16, -398, u64, Warm> as Numeral>::RADIX == 10);
const _: () = assert!(quantum_digits::<UFixed<13, 3, u16, Warm>>() == -3);
```

The exponent form illustrates the same rule one level down and I want it noted because it will be reached for:
**the form's type carries the kind, and the exponent's value is a read**, because a value derived from a
parameter has no type position. `Constant` and `Ranged` are value-free markers; `EMIN` and `EMAX` are consts.
That is `126`'s two-door rule applied to the exponent, and it is what makes the exponent form usable as a
preset key (`127b:99-102`) without any of it computing.

I am the first read on this section and it is the part of the file I would most want a second one on, because it
touches D53 and the identity contract's shape rather than only the surface.

---

## 11. What it costs, measured

**Zero unstable features and zero compiler flags.** `e1_capstone.rs`, `a3_surface.rs`, `f1_arity3.rs` and
`h1_four_interpretations.rs` carry `#![no_std]` and nothing else. Not `min_generic_const_args`, not
`adt_const_params`, not the const-traits family, not `-Znext-solver=globally`. `128`'s vetting and its open
question about where the flag lands both become moot for this piece of the design.

**Compile cost.** 64 distinct compositions at four-digit widths, each a generic `mul` through the surface with
its own law check, `/usr/bin/time -p`, cold then three runs (`e2_scale.rs`): **0.05 s cold, then 0.04, 0.05,
0.04.** Against the panel's own table for the identical workload: 0.06 s through a 4096-row table and 5.87 s
through use-site realisation (`125:245-250`), 0.04 s for `126`'s structural numeral (`126:221-227`), 0.13 s
cold then 0.05 s for `129`'s (`129:466-473`). Level with the fastest thing measured, at widths no table holds,
while checking two coordinate relations and a container fit per composition rather than one. The negative
control fires with the law named (section 4).

**Parameter count.** Four written parameters, `UFixed<13, 3, u16, Warm>`, which is one more than D48's
`UFixed<13, 3, Warm>` and exactly what `129`'s macro surface also writes. The extra one is the container and
section 7 says it is unavoidable gate-free.

D48's literal arity is available if op wants it, by putting the container on the strategy, which is where
`110:856-859`'s own declaration already puts the `Lowering::Container` projection (`f1_arity3.rs`, exit 0):

```rust
let _a: UFixed<13, 3, Warm<u16>> = UFixed::zero();
let _b: UFixed<8, 8, Warm<u16>> = UFixed::zero();
```

Three parameters, plain type syntax, same checks, same projections. The container is still written; it is
written inside the strategy instead of beside it. I have no strong view between the two spellings and both
compile; it is a surface question and therefore op's.

**No enumeration survives anywhere over widths.** The ladder enumerates the five rungs the hardware has, which
is `127b:66-68`'s own placement. Widths are unbounded, capped only by the const parameter's own type, which
`123:44-51` located as not a design decision.

---

## 12. What is op's, separately from what I decided

**Mine, and compiled.** That the precision-keyed numeral loses the exponent and therefore cannot refuse a scale
mismatch, with a program in which one decodes to thirty-two at exit 0. That the collision between D48 and
canonicity is created by keying the numeral on one coordinate and disappears when it is keyed on two. That the
whole surface, the laws, the projections, the fit check and the four families compile with no feature gate and
no flag. That alignment, format identity and container identity are bounds checked at declaration, which is
strictly more pre-monomorphisation checking than either option in `129:328-337`'s table. That a generic wrapper
cannot compute its output coordinates, so the post-monomorphisation hole is closable by a small fixed set of
instantiation witnesses. That the brief's and `129`'s claim about the hole pointing at the caller rather than
the author is false, and the real loss is the consumer's own line.

**His, and it is the one that blocks.** Whether `127b:24` still says what he wants it to say, now that the
`E0308` it names is a defect of a representation the panel has removed and the literal reading costs the
library the ability to type a scale. Section 8 states the reading I would take and does not take it.

**His, because it is a surface question.** Which arity, `UFixed<13, 3, u16, Warm>` or
`UFixed<13, 3, Warm<u16>>`. Both compile, both check identically, and the second matches D48's shape literally.

**His, because it touches a ratified structure.** Whether `Precision` moving from an associated type to an
associated const is inside D69's overturn or outside it. My reading is that the overturn is about which
coordinates, not about their kind, and that the mathematical coordinates all survive as reads. That is a
reading of his own sentence and he is the one who wrote it.

**Owed under the two-expert rule.** This file overturns `129`'s numeral on compiled evidence and proposes a
surface no second expert has read, and section 10 touches D53 and the identity contract. Nothing here should
enter the canon on one expert's word. A second read should attack the premise I attacked: that the numeral is a
format rather than a width. If the numeral is genuinely a width and the scale belongs somewhere else, my
conclusion inverts and `129`'s stands.

---

## 13. What I did not check

- **Whether `129`'s numeral was meant as the whole numeral or as a vehicle for the canonicity argument.** Its
  section 5 hands the surface back, which reads like a vehicle; its capstone and consumer section read like a
  claim. I answered the claim.
- **`mock/research/sketches/202607282100_container-projection-without-gce`.** Named in `129:536-537` as
  unopened and still unopened. Nothing below depends on it; the construction here enumerates nothing over
  widths at any layer.
- **Whether a specific witness triple is adequate for every law**, section 9's last paragraph. The space of
  wrong wrappers is proved small and the shapes are refused; selecting the triple per law is mechanical and
  undone.
- **How often the missing-annotation error fires in real consumer code**, section 4. It is a question about
  arvo's downstream, which I did not read.
- **Whether the `Bits` layer should be reached through an output parameter**, which is how `bits_of` does it in
  the capstone, or whether `Bits` should carry the format's coordinates itself. Both compile; I did not think
  about which is right.
- **The float and decimal families beyond their declarations.** `h1_four_interpretations.rs` compiles four
  families against one contract. It does not implement a single float operation, and the exponent form's own
  laws are section 10's open half.
