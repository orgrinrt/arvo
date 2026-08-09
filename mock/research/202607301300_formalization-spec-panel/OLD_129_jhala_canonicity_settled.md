# 129. Canonicity settled: the precision is the parameter, and nothing computes it

**Persona:** Ranjit Jhala, refinement types and decidable-fragment lens.
**Date:** 2026-08-07
**Position:** after `128_the_generic_const_args_vetting.md`. Reads `126`, `127b`, `128` and the standing base.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository. `mock/crates` untouched, `mock/design_rounds/` untouched. Probes at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/jhala129/`,
twenty-four files named `q1` through `q17` and `gran/g1` through `gran/g5`, roughly in the order
they appear here.

Op ratified canonicity at `127b:22-31` and instructed the panel to find a cheaper way to it than the
enumeration. `128` vetted `generic_const_args` to WATCH and reported one limit on it, that canonicity holds
at concrete instantiation but not under a generic parameter. I went to check that limit before reasoning
from it, because it is the one claim in `128` that bears on whether the feature answers op's requirement at
all.

**It does not, and the limit is worse than `128` states.** Under `generic_const_args`, `(A + B) + C` and
`A + (B + C)` are distinct types under a generic parameter (`q10_assoc.rs`, quoted in section 4). That is
not a corner case. It is what two composed widenings against one combined widening produce, and it is a law
arvo will want to state. So GCA does not deliver "two numerals of equal precision are the same type". It
delivers *derivation* of the precision, which is a different and lesser thing, and the panel has been
treating them as one.

**The mechanism that does deliver op's requirement uses no feature gate at all, and it is one sentence:
make the precision the const parameter, and never compute it in type position.** Then canonicity is not a
property to be established, it is structural: `Fx<16, ...>` is one type, reached however you like, at every
level of genericity. What is given up is derivation, and derivation comes back as an output parameter with
its relation stated as a check. The whole thing compiles at `q13_capstone.rs`, exit 0, no `#![feature]`, no
`-Z` flag.

Op's non-negotiable clause is satisfied by construction rather than by a vetting: there is no forbidden
feature in it because there is no feature in it.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo yet; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts the intent and op's own calls in the governing position.
The governing calls here are op's canonicity ratification (`127b:22-31`), his instruction to find a cheaper
mechanism rather than accept the enumeration (`127b:30-31`), D48 keeping the `UFixed<13, 3, Warm>` surface
(`127b:56-59`), and the convergence pressure at `127b:12-18`. This file proposes nothing those forbid.
Section 5 does report that D48 and canonicity collide under one spelling, which is a finding about the
calls rather than a reopening of them, and section 8 hands the collision back.

**Test gate.** Not run, deliberately, and I am naming that rather than letting it pass. The panel rule's
"the spec is the subject" section is explicit that existing code is evidence of why rather than the
subject, and op ruled at `108b:174-181` that a further report of the same collected tautologies is what
that ruling exists to stop. `126:39-44` ran the suite this pass and got 155 binaries, 672 passed, 0 failed,
9 ignored on a tree nothing has moved. My deliverable touches no crate in that tree. The instrument that
measured anything here was the compiler, twenty-four times.

**The brief's factual claims about `128`, checked.**

*"Verdict WATCH, not forbidden."* **Holds**, `128:17-19`.

*"rustc enforces that `generic_const_args` requires `min_generic_const_args`."* **Holds**, and I
reproduced the other half nobody quoted: it also hard-requires the flag, in its own words
(`q7b_gca_clean.rs` without `-Znext-solver=globally`):

```
error: `generic_const_args` requires -Znext-solver=globally to be enabled
2 | #![feature(min_generic_const_args, generic_const_args)]
  |                                    ^^^^^^^^^^^^^^^^^^
```

Two mechanisms, not one, and the second is a whole-crate trait solver replacement. `128:298-302` is right
to call that a larger exposure than any feature gate in the tables.

*"`generic_const_args` is not needed for container selection."* **Holds**, and I rebuilt it rather than
citing it (`q13_capstone.rs:8-30`). The carry-and-read ladder is in the capstone and costs nothing.

*"Canonicity holds at concrete instantiation but not under a generic parameter."* **Holds and understates
it.** Section 4.

*"A committed sketch at `mock/research/sketches/202607282100_container-projection-without-gce` is GCE-free
but still enumerates widths."* **Unchecked.** I did not open it. It does not bear on the mechanism below,
which enumerates nothing at any layer.

---

## 1. The settled mechanism

`q13_capstone.rs`, no feature gates, no `-Z` flag, exit 0. Three parts, each individually sound, which is
the decomposition op asked for.

**Part one, the container ladder. Carry and read, never derive.** This is `Capacity`'s move from
`126:82-120`, applied to the container instead of to the width.

```rust
pub trait Container: Copy { const BITS: u32; }
impl Container for u8   { const BITS: u32 = 8;   }
impl Container for u16  { const BITS: u32 = 16;  }
// ...through u128

pub struct Fx<const P: u32, C: Container, S> { raw: C, _s: PhantomData<S> }

impl<const P: u32, C: Container, S> Fx<P, C, S> {
    const FITS: () = assert!(P <= C::BITS, "precision does not fit its container");
    pub fn new(raw: C) -> Self { let () = Self::FITS; Fx { raw, _s: PhantomData } }
}
```

Five impls, one per rung the hardware actually has, which is the enumeration `127b:66-68` says belongs
there if it belongs anywhere. The consumer names the container and the compiler checks the fit. Nothing
derives a type from a value, so nothing hits the wall.

**Part two, the numeral. The precision is the parameter.** `Fx<const P: u32, C, S>`. There is no `I + F`
in type position, no `Sum<W<I>, W<F>>`, no tower, no bridge. `P` is the precision, full stop.

Canonicity follows immediately and needs no argument, which is the point. Two numerals of precision 16 are
the same type because they are both `Fx<16, C, S>`. Compiled three ways at `q1_canonical_by_construction.rs`
and again at `q13_capstone.rs:70-73`: `Fx<{13 + 3}, ..>`, `Fx<{8 + 8}, ..>` and `Fx<16, ..>` are one type,
interchangeable in both directions, and all three satisfy a function that wants `Fx<16, ..>`.

The arithmetic in `{13 + 3}` is ordinary const eval on literals. No generic parameter is involved, so no
restriction applies and no gate is needed. That is the whole reason this works: **the design's arithmetic
is at concrete sites, where the language has never restricted it.**

**Part three, the laws. The output precision is a parameter; its relation to the inputs is a check.**

```rust
pub fn mul<const P: u32, const Q: u32, const R: u32, C: Container, S>(
    a: Fx<P, C, S>, b: Fx<Q, C, S>,
) -> Fx<R, C, S> {
    const { assert!(R == P + Q, "mul: output precision must equal the sum of the input precisions") }
    // ...
}
```

The addition lives in a const block **in the body**, which is a value computation and therefore
unrestricted. It is not a const argument, so it never meets the rule that refuses everything else. `R` is
inferred backwards from an annotation or from a downstream parameter type; both compiled at
`q4_output_param.rs:29-45`.

This is the shape refinement types have used for twenty years, and it is worth naming as such because it
tells you exactly what you get and what you do not. `mul` carries a refinement on its output,
`{R : u32 | R = P + Q}`, and the solver discharging it is const eval. The predicate is linear arithmetic
over integers, which is inside the decidable fragment, so it always answers and it answers immediately.
What is different from a refinement type system, and the honest limit, is *when* it is asked. Section 3.

---

## 2. Why this is settled and GCA is not: the inversion

The panel, and `128`, have been treating "canonicity" and "the compiler derives the precision" as the same
requirement. They are not, and separating them is what settles this.

Op's ratified requirement (`127b:24`) is *canonicity*: equal precision means equal type. Under part two it
holds unconditionally. Under GCA it holds only where const eval can run, which means at concrete
instantiation, and fails under a generic parameter, which is where a library states its laws.

`q10_assoc.rs`, under `#![feature(min_generic_const_args, generic_const_args)]` with
`-Znext-solver=globally`:

```
error[E0308]: mismatched types
14 | ) -> W<{ <T3<A, B, C> as R>::V }> { x }
   |      ----------------------------   ^ types differ
   = note: expected struct `W<const { A + (B + C) }>`
              found struct `W<const { (A + B) + C }>`
```

Addition is not associative in GCA's type language under a generic parameter. Nor commutative:
`q9_defeq.rs` reproduces `128`'s `I + F` against `F + I` independently, with the concrete case in the same
file compiling clean.

Now put that against the gate-free construction. `q11_gatefree_no_limit.rs`, exit 0, no gates:

```rust
pub fn widen_then<const P: u32, const A: u32, const B: u32, const R: u32>(x: Number<P, Warm>) -> Number<R, Warm> {
    const { assert!(R == (P + A) + B, "...") } // ...
}
pub fn widen_once<const P: u32, const A: u32, const B: u32, const R: u32>(x: Number<P, Warm>) -> Number<R, Warm> {
    const { assert!(R == P + (A + B), "...") } // ...
}
pub fn interchange<const P: u32, const A: u32, const B: u32, const R: u32>(x: Number<P, Warm>) -> Number<R, Warm> {
    let y: Number<R, Warm> = widen_then::<P, A, B, R>(x);
    let z: Number<R, Warm> = widen_once::<P, A, B, R>(x);
    let _ = z; y
}
```

Both routes produce the same type under a generic parameter, because the type is `Number<R>` and `R` is a
parameter rather than an expression. The associativity that GCA cannot see is not a question the
construction asks, because it does its arithmetic in a place where `(P + A) + B` and `P + (A + B)` are
integers rather than syntax.

**This is the load-bearing point of the file.** GCA moves the arithmetic into the type language, and the
type language's equality is definitional, so every algebraic identity arithmetic has is lost at exactly
the moment you need it. The gate-free construction keeps the arithmetic in the value language, where
equality is semantic, and pays for it by not deriving. Op's requirement is about equality. The construction
that keeps equality semantic is the one that meets it.

I want to be exact about what this does not say. GCA is not unsound and the vetting at `128` stands on its
own two gate answers. It buys something real, and section 3 prices it. What it does not buy is the
property op ratified, and the file that vetted it reported the failure as a rough edge rather than as a
miss against the requirement. That reading is what I am correcting.

---

## 3. Where the granular decomposition stops, and the diagnostic at each wall

Op's method instruction was to break the problem into individually sound parts and go more granular while
the solver stays happy. I did, in eight directions, and every one arrives at the same single point: **there
is no legal position anywhere for one addition on a generic parameter that reaches type position.** The
compiler says so in the same words each time.

Starting from the surface and taking each repair rustc offers, under `min_generic_const_args` alone:

```
error: complex const arguments must be placed inside of a `const` block
9 | ) -> Number<{ P + Q }, Warm> { todo!() }
```

(`q2_generic_computed_result.rs`.) Taking the repair:

```
error: generic parameters may not be used in const operations
9 | ) -> Number<const { P + Q }, Warm> { todo!() }
  = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

(`q3_constblock.rs`.) There is no third repair. That reproduces `128:175-201` and `126:126-150`
independently, from the signature rather than from an associated item.

The five granular routes, `gran/g1` through `gran/g5`, each compiled separately:

| Route | Shape | Result |
|---|---|---|
| `g1` | `type const V = add(P, Q)`, arithmetic moved into a `const fn` | refused, complex const argument |
| `g2` | `type const V = P`, then `W<{ <W<P> as S>::V }>` | **exit 0**, the round trip works |
| `g3` | `type const V = <T as S>::V + 1`, adding a literal one to a `type const` | refused, complex const argument |
| `g4` | `where W<P>: S<V = { R }>`, associated-const-equality bound | **exit 0**, the bound works |
| `g5` | `const R: u32 = { P + Q }` as a const parameter default | refused, complex const argument |

Two of those are the interesting ones and neither has been reported in this panel.

**`g2` confirms the transport is bidirectional and gate-free**, which is `126:157-171`'s finding reached
from a different direction. A const parameter travels out of a type through a `type const` and back into
type position with no feature at all.

**`g4` is new and it is the near miss.** Associated-const-equality bounds work under
`min_generic_const_args`, and rustc told me so itself: `associated_const_equality` was
`removed in 1.94.0` and `merged into min_generic_const_args` (`q8_assoc_const_eq.rs`, first attempt). So a
relation between a const parameter and an associated const **can** be stated as a `where` clause, checked
where the signature is rather than where the function is instantiated. That is precisely the
pre-monomorphisation guarantee section 3 of this file is about to say is unavailable.

It is unavailable anyway, and the reason is the same single wall. Following rustc's own repairs through
`q8_assoc_const_eq.rs`, four steps:

1. `Sum<S = R>` gives `expected type, found const parameter R`, with `help: consider adding braces here`.
2. `Sum<S = { R }>` gives `use of trait associated const not defined as type const`, with
   `note: the declaration in the trait must begin with type const not just const alone`.
3. Declaring `type const S: u32` moves the failure into the impl body:
   `error: complex const arguments must be placed inside of a const block` on `type const S: u32 = P + Q`.
4. The next repair is the const block, which is `q3`'s error and names GCA.

So the bound mechanism exists and is usable, and the only thing it cannot be given is a value to compare
against, because producing that value requires the addition, and the addition has no legal home. `g4`
compiles when the associated const is a bare parameter and dies the moment it is a sum.

**The answer to the brief's question two, stated plainly: no, the granular decomposition does not reach a
pre-monomorphisation guarantee without `generic_const_args`.** It stops at one addition, in eight
independent routes, and rustc names the feature at every stop. `126` found the same wall from three
directions and `128` from three more. Fourteen compiled refusals across three files is enough; I would
stop looking.

What the decomposition **does** reach is canonicity, which was the actual requirement, and it reaches it
with no gate. The pre-monomorphisation guarantee is a different property, and section 6 prices its
absence honestly rather than talking past it.

---

## 4. What the check catches, what it misses, and the exact size of the hole

The relation is checked at instantiation, not at declaration. That is a real weakness and I am going to
show it failing rather than describe it.

**What it catches.** A wrong output precision at a concrete call, with the design's own message
(`q5_neg_wrong_r.rs`):

```
error[E0080]: evaluation panicked: mul: output precision must equal the sum of the input precisions
13 |     const { assert!(R == P + Q, "mul: output precision must equal ...") }
   |             ^^^^^^^ evaluation of `mul::<16, 16, 31>::{constant#0}` failed here
note: the above error was encountered while instantiating `fn mul::<16, 16, 31>`
19 |     let _c: Number<31, Warm> = mul(a, b);
   |                                ^^^^^^^^^
```

The diagnostic names the offending instantiation with its actual values, quotes the design's sentence, and
points at the call site line. That is as good as this class of error gets.

**What it misses**, and this is the hole (`q6_postmono_hole.rs`, **exit 0**):

```rust
pub fn square_wrong<const P: u32>(x: Number<P, Warm>) -> Number<P, Warm> {
    mul::<P, P, P>(x, x)
}
```

That function is wrong for every `P`. It claims the product of two `P`-wide numerals is `P` wide. rustc
accepts it, because it is never instantiated in that crate. A consumer who writes it ships it, and the
error surfaces at whichever downstream call first instantiates it, pointing at their code rather than at
the library's.

**GCA closes exactly this hole and I compiled that too** (`q7_gca.rs`, same function, GCA plus the flag):

```
error[E0308]: mismatched types
32 | pub fn square_wrong<const P: u32>(x: Number<P, Warm>) -> Number<P, Warm> {
   |                                                          --------------- expected because of return type
33 |     mul::<P, P>(x, x)
   |     ^^^^^^^^^^^^^^^^^ types differ
   = note: expected struct `Number<P, _>`
              found struct `Number<const { P + Q }, _>`
```

Refused at declaration, uninstantiated. That is what the second mechanism buys, stated as precisely as I
can state it: **a wrong generic wrapper is a compile error where it is written rather than where it is
used.**

So the fork is exact, and it is not the fork the panel has been arguing.

| | Gate-free (section 1) | GCA plus `-Znext-solver=globally` |
|---|---|---|
| Canonicity at concrete sites | holds, structurally | holds, by const eval |
| Canonicity under a generic parameter | **holds, structurally** | **fails**, `q9`, `q10` |
| Precision derived rather than written | no, inferred or annotated | yes |
| Wrong generic wrapper caught | at instantiation | **at declaration** |
| Unstable mechanisms | **none** | two |

Both columns have a real entry the other lacks. Neither is strictly better, which is why this is op's call
and not mine. But the row op ratified is row two, and only one column holds it.

---

## 5. The collision with D48, which is the one thing I found that nobody has stated

The surface spelling `UFixed<13, 3, Warm>` and structural canonicity cannot both be had through a generic
type alias. `q13_capstone.rs` in its first form:

```
error: generic parameters may not be used in const operations
31 | pub type UFixed<const I: u32, const F: u32, C, S> = Fx<{ I + F }, C, S>;
   |                                                          ^ cannot perform const operation using `I`
   = help: const parameters may only be used as standalone arguments here, i.e. `I`
```

The alias is generic, so `{ I + F }` is a computed const argument on generic parameters, and it dies at the
same wall as everything else. D48 keeps the two-number surface (`127b:56-59`); canonicity requires the
numeral to be keyed on one number. The alias is where those two meet, and it is refused.

Three ways out, all compiled, and the choice among them is op's.

**Way one, a macro at the surface.** `q14_macro_surface.rs`, exit 0, gate-free:

```rust
macro_rules! UFixed {
    ($i:literal, $f:literal, $c:ty, $s:ty) => { $crate::Fx<{ $i + $f }, $c, $s> };
}
pub fn consumer(x: UFixed!(13, 3, u16, Warm), y: UFixed!(8, 8, u16, Warm)) {
    wants16(x); wants16(y);
}
```

The expansion happens at the consumer's concrete site, where the addition is ordinary const eval. It works
in parameter position, in return position, and in `let` annotations. The cost is one exclamation mark and
the loss of a type alias's ability to appear in a generic bound. The capstone is written this way.

**Way two, change what the two numbers mean.** `Fx<16, 13, C, S>` reading as precision 16 with 13 integer
bits. No arithmetic anywhere, plain type alias, no bang. It is a smaller change than it looks, since the
fraction width is recoverable as `P - I`, but it does change the surface op ratified and I am not going to
present a surface change as an implementation detail.

**Way three, GCA**, which makes the original alias legal at the price already priced in section 4.

I want to be clear that this collision is not created by my construction. It is created by the requirement,
and it was there the moment canonicity was ratified alongside D48. Any mechanism delivering canonicity over
a two-number surface has to resolve the sum somewhere, and there are exactly three somewheres: at the
consumer's concrete site (way one), never (way two), or in the type language (way three). The construction
made it visible rather than causing it.

---

## 6. What a consumer writes, and what they see when they get it wrong

The whole surface, from `q13_capstone.rs:49-66`:

```rust
let a: UFixed!(13, 3, u16, Warm) = Fx::new(0);
let b: UFixed!(8, 8, u16, Warm) = a;              // same type, different scaling
let _w: UFixed!(40, 30, u128, Warm) = Fx::new(0); // arbitrary widths, no table, no cap
let _x: UFixed!(3, 0, u8, Warm) = Fx::new(0);
let c: Fx<32, u16, Warm> = mul(a, b);             // R inferred from the annotation
let d: Fx<17, u16, Warm> = add(a, b);
```

`wants32(mul(a, b))` also works with no annotation at all, because `R` is inferred from the callee's
parameter type (`q4_output_param.rs:38-45`).

The three ways to get it wrong, each with its actual diagnostic.

**Wrong output precision.** Section 4's `E0080`, which names `mul::<16, 16, 31>`, quotes the design's
sentence, and points at the call. Good.

**Output precision not inferable.** The consumer writes `let c = mul(a, b);` with no annotation and no
downstream use (`q12_no_annotation.rs`):

```
error[E0284]: type annotations needed for `Number<_, Warm>`
19 |     let c = mul(a, b);
   |         ^   --------- type must be known at this point
note: required by a const generic parameter in `mul`
help: consider giving `c` an explicit type, where the value of const parameter `R` is specified
   |
19 |     let c: Number<R, _> = mul(a, b);
```

This is the ergonomic cost and I am not going to soften it. It is a real error a consumer will meet, the
`help` names `R` rather than suggesting the value, and the consumer has to know that the answer is 32. It
is exactly the error GCA would not produce. How often it fires depends on how often an intermediate result
is bound without an annotation and without a typed use, which is a question about arvo's actual consumer
code that I have not measured and am not going to guess at.

**Precision does not fit the container.** From the carry-and-read ladder, at const eval:

```
error[E0080]: evaluation panicked: precision does not fit its container
   const FITS: () = assert!(P <= C::BITS, "precision does not fit its container");
```

This one is worth a note against `arvo-toolbox-not-policer.md`. The consumer names the container and the
design checks the fit rather than choosing for them, which is the toolbox posture rather than the policer
posture, and it is what makes the ladder five impls instead of a table.

---

## 7. What this costs in mechanisms, precisely

Answering the brief's question one directly.

**The settled mechanism costs zero unstable features and zero compiler flags.** `q13_capstone.rs` carries
`#![no_std]` and nothing else. Not `min_generic_const_args`, not `adt_const_params`, not the const-traits
family, not `-Znext-solver=globally`. The const parameters are plain, the arithmetic is at concrete sites or
in value position, and the container ladder is five ordinary impls.

That is a stronger result than "no forbidden features", which is what op's non-negotiable clause asked for.
It removes the vetting question entirely for this piece of the design, and it means nothing here can be
invalidated by a feature's status changing upstream.

**If op takes the pre-monomorphisation guarantee instead**, the cost is `min_generic_const_args` (already
allowed) plus `generic_const_args` (WATCH per `128`) plus `-Znext-solver=globally` (tier unassigned,
`128:287-306`), and transitionally the Cargo-side `profile-rustflags` while GCE remains in the tree
(`128:263-285`). Three mechanisms, one of them uncategorised, and in exchange a property that fails on
associativity under a generic parameter.

**One thing neither option costs, worth saying because the panel priced it at length.** No enumeration
survives anywhere over widths. The container ladder enumerates the five rungs the hardware has, which is
`127b:66-68`'s own placement, and that enumeration is bounded by physics rather than by a design choice.
Widths are unbounded, capped only by the const parameter's own type, which `123:44-51` correctly located as
not a design decision.

**Compile cost, measured.** `q15_scale.rs`, 64 distinct compositions at four-digit widths, each a generic
`mul` call through the macro surface with its own compile-time assertion, `/usr/bin/time -p`, three runs:
**0.13 s cold, then 0.05 s, 0.05 s.** Set that against `125:245-250`'s own table for the same workload:
0.06 s through a 4096-row table, 5.87 s through use-site realisation, and `126:229-232`'s 0.04 s for the
structural numeral. The gate-free construction is level with the fastest thing the panel has measured, at
widths no table holds, and it is doing more than `126`'s numeral was because the width law is checked as
well as computed. The negative control fires
(`error[E0080]: evaluation panicked: assertion failed: 1000 + 1000 == 2001`, `q15_neg.rs`).

---

## 8. What is op's, separately from what I decided

**Mine, and compiled.** That canonicity is available with no feature gate, via making the precision the
const parameter. That GCA's canonicity fails on commutativity and associativity under a generic parameter,
so it does not satisfy the ratified requirement in the case where a library states its laws. That the
granular decomposition op described reaches canonicity and stops one addition short of a
pre-monomorphisation guarantee, in eight independent routes with rustc naming GCA at each stop. That
associated-const-equality bounds are live under `min_generic_const_args` and are the near miss.

**His, because it is a trade and not a soundness question.** Whether the pre-monomorphisation refusal of a
wrong generic wrapper (`q6` against `q7`) is worth three unstable mechanisms, given that the property it is
usually justified by, canonicity, is better served without them. My reading is that it is not, on
`127b:12-18`'s convergence pressure and on the plain fact that zero mechanisms beats three when the
zero-mechanism route holds the ratified requirement more completely. That is a reading, not a call.

**His, and it is the one that actually blocks.** The D48 collision in section 5. Canonicity and the
`UFixed<13, 3, Warm>` alias cannot coexist without resolving the sum somewhere, and the three somewheres
are a macro surface with a bang, a re-reading of the two numbers as precision and integer bits, or GCA. All
three compile. Choosing among them changes the surface a consumer writes, which is D48's territory and
therefore his.

**His, because the rule has no answer.** Where `-Znext-solver=globally` sits, which `128:287-306` raised
and explicitly did not settle. If op takes the gate-free route the question goes away for this piece of the
design, which is worth knowing before deciding it.

**Owed under the two-expert rule.** This file overturns `128`'s reading of the GCA limit, on compiled
evidence from a different direction, and it proposes a mechanism no second expert has read. Nothing here
should enter the canon on one expert's word, and `127b:63-65` is op's own statement of that. A second read
should attack the same premise I did: that canonicity and derivation are one requirement. If they are one
requirement after all, my conclusion inverts.

---

## 9. Two checks the file was written without, closed afterwards

Both were listed as outstanding when this file was first written, per the brief's instruction to write
early and extend rather than hold a finished investigation. Both now hold.

**The signed numeral.** `q16_signed.rs`, exit 0, gate-free. `IFixed!(12, 3, i16, Warm)` and
`IFixed!(7, 8, i16, Warm)` are the same type, both precision 16, because the surface macro expands
`1 + $i + $f` at the concrete site exactly as the unsigned one expands `$i + $f`:

```rust
macro_rules! IFixed { ($i:literal, $f:literal, $c:ty, $s:ty) => { Fx<{ 1 + $i + $f }, $c, Signed, $s> }; }
```

The sign marker keeps the two families apart as types while the precision parameter keeps each family
canonical within itself. One extra addition, at a place where addition was never restricted.

**The strategy bound.** `q17_bounded_s.rs`, exit 0, gate-free: the whole capstone with `S: Policy +
Lowering` on every item, which is the bound `127b:97-102` records as converged and never stated anywhere
in six thousand lines. Nothing perturbs, which is the expected result and is worth having compiled rather
than expected, because the panel's record of expected results is not good.

---

## 10. What I did not check

- **`mock/research/sketches/202607282100_container-projection-without-gce`.** Named in the brief, not
  opened. It enumerates widths per the brief; the mechanism here does not, at any layer.
- **Whether the post-monomorphisation hole is reachable in practice in arvo's own crates.** `q6` proves it
  is reachable in principle. How often a generic wrapper over a width law appears in real consumer code is
  a question about code I did not read, and it is the input that would settle section 4's fork on evidence
  rather than on taste.
- **The preset key on the exponent form** (`127b:99-102`). Section 9 puts `S: Policy + Lowering` on the
  capstone and nothing moves, but the preset key is a separate piece and I did not reach it.
- **Whether `Fx<P, C, Sign, S>` is the right parameter order or the right factoring**, as against carrying
  the sign inside the container marker. `q16_signed.rs` proves the mechanism is indifferent to which; it is
  a surface question and it belongs with section 5's.
- **The next-solver open bug list**, which `128:78-81` also left unchecked. It bears only on the GCA column
  of section 4's table, and the gate-free column does not touch the solver at all.
