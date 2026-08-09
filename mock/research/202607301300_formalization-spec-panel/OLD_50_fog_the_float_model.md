# The float model: what the settled machinery does when the exponent moves

Agner Fog, file 50. I wrote file 08, which measured the union's cost and established the const-eval
width ceiling. Nothing in that file is relied on here except the ceiling itself, which still holds and
which is why everything below that is exhaustive runs at a model width.

**What I read.** `49_consolidation_four.md` in full, per the standing instruction that it is the only
required reading, plus an `ls` of the panel directory. Behind it I opened only two things, both for a
mechanism it compresses rather than for a claim: `46_probes/vu_nat_sealed_adj.rs` and
`46_probes/vu_bias_sealed_adj.rs`, the sealed tower, which I compose with rather than reinvent, copied
into `50_probes/` unmodified exactly as file 46 copied them from file 42.

**Gates.** Test gate: `cargo test --workspace` from `mock/`, summed per binary rather than read off a
headline, reports 654 passed, 0 failed, 9 ignored, matching the consolidation's own figure. Canon gate:
the surface this file designs has no shipped source. `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` returns nothing (exit 1), as does the same command with `FullRange\|UTerm\|AddWidth`,
which is the consolidation's own corrected verification command and it reproduces as written. So there
is no code for me to critique and no rewrite cost to weigh; the whole subject is design.

**What is compiled or measured, and what is reasoned.** Six probes in `50_probes/`, every one built and
run on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved from the
repo's `rust-toolchain.toml` pin. Where a number is an instruction count or an emitted-code fact it is
that target's and I say so at the point of use; I ran no timing loop, because a timing claim belongs in
`mock/benches/` under the harness and not in a probe. Section 7 lists the two places where a bench is
owed. Everything not marked compiled or measured is reasoning, and I mark the reasoning that has not
been checked as such rather than letting it inherit a sibling's evidence.

## 1. The shape, in one paragraph

A float numeral is not a new kind of numeral. It is a **finite union of `Implicit` numerals whose
adjustments form a geometric chain**, together with a rule selecting which member of the union a value
belongs to. The rule is the exponent function, and it reads the value's own magnitude. Everything the
review has settled survives that, with exactly one insertion: the quantiser gains a grid-selection step
in front of the rounding step it already performs. Nothing else in the machinery changes shape. What
changes is which answers come out, and four of those changes are load-bearing enough that the spec has
to state them rather than let a reader derive them.

Stated as the design would state it:

> A `Ranged` numeral denotes the union, over `e` in `[EMIN, EMAX]`, of the grids with quantum
> `radix^(e - p + 1)` restricted to `[radix^e, radix^(e+1))`, together with the bottom grid
> `radix^(EMIN - p + 1)` extended down to zero when `Underflow = Gradual`, omitted when
> `Underflow = Abrupt`, and collapsed to zero after the fact when `Underflow = FlushToZero`. Quantising
> to a `Ranged` numeral selects the grid from the exact value's own magnitude, rounds on the selected
> grid extended upward without bound, and then classifies the rounded result against `[EMIN, EMAX]`,
> `Specials` and `Underflow`. An `Implicit` numeral is the degenerate case where the union has one
> member and the selection step is the identity.

The measured support for that sentence is in section 2: the model implementing exactly it, with no
knowledge of IEEE beyond the format parameters, agrees with binary32 on **41,380,159 operations**
without a single disagreement (`50_probes/probe_1_model_vs_silicon.rs`).

**The algebraic difference, stated precisely, because three separate results follow from it.** As an
ordered set a `Ranged` numeral's value set is what an `Implicit` numeral's is: finite, totally ordered,
so meets and joins exist and the word lattice in the order-theoretic sense applies to both. That is not
where they part. An `Implicit` numeral's value set is an **interval of a rank-one subgroup of the
rationals**: it is closed under addition wherever the sum stays in range, and the quantum generates it.
A `Ranged` numeral's is a union of intervals of subgroups whose generators form a geometric chain, and
that union is **not a subgroup**: `1 + 2^-24` is not a binary32 value although both operands are, and
the machine agrees, delivering `0x3f800000` for a sum whose exact value is not one
(`1 + 2^-23` does differ from one, at `0x3f800001`, which is the neighbouring case worth checking
before believing the first). Three
of this file's results are that one sentence read in three places. The overflow band is inhabited
(section 3) because the exact result lies on a finer subgroup than the result's own. The fold needs an
exact accumulator (section 4.5) because in-range closure is what an in-format accumulator would have
needed. And associativity fails at the format width and holds through the accumulator (section 4.6) for
the same reason. The design should state the subgroup difference once, at the top, and derive the three
rather than discovering them separately, which is how this review discovered them.

*grounded on: `pin`, `host`, `model`; the design statement itself grounded on `round-first`, which it
extends rather than replaces.*

## 2. The quantiser: one new step, and it is the founding idea in code

`model.rs:39-47` is the whole difference:

```rust
pub fn quantum_exp(&self, e: i32) -> i32 {
    let unfloored = e - (self.p as i32) + 1;
    let floor = self.emin - (self.p as i32) + 1;
    if unfloored < floor { floor } else { unfloored }
}
```

For an `Implicit` numeral this function is a constant. For a `Ranged` numeral it is the above. That is
the entire content of "fixed point and floating point are one formalisation differing only in an
exponent function", made executable, and the `if` in the last line is the `Underflow` axis: the floor is
where gradual underflow stops refining the grid.

**Round-first survives unchanged, and the hardware is what says so.** File 39 checked the three
directions against the standard's text. This probe checks the whole pipeline against the machine that
executes it: for each of the ten thousand-odd operand pairs it sweeps, the exact result is formed as a
rational, quantised by the model, encoded back to bits, and compared with what the FPU delivered.

| operation | pairs compared | mismatches |
|---|---|---|
| binary32 add | 9,380,157 | 0 |
| binary32 multiply | 16,008,001 | 0 |
| binary32 divide | 15,992,001 | 0 |

Within that sweep, 1,255 results overflowed to infinity, 884 landed on subnormals (so gradual underflow
is exercised rather than assumed), and 2,090 are overflow-band cases in the precise sense of section 3.
The exactness bound is stated rather than hidden: the exact sum of two binary32 values spans up to 277
bits and the model computes in `u128`, so the add sweep filters to an exponent spread of at most 90 and
the whole-format exhaustive checks run at the model width instead. Multiplication and division carry no
such filter and sweep the full cross product.

*grounded on: `pin`, `host`, `model`, `round-first`.*

**One bug worth recording, because the hardware found it and no amount of reading would have.** The
first version of the rounding kernel shifted the denominator left to align the quantum. For the product
of two binary32 subnormals the scale is -298, the shift is 149, and `u128 << 149` silently masks the
shift amount to 21 and returns a plausible wrong answer: a nonzero subnormal result for a product 130
binades below the smallest representable value. Nothing about the code looked wrong. The mismatch
against `f32` is what surfaced it, and the repair (`model.rs:198-262`) is exact long division with an
explicit below-half-a-quantum short circuit. The general point for the spec, and it is not about my
probe: **a quantiser for a `Ranged` numeral has to be written against exponent spreads that no
`Implicit` numeral ever produces**, and the arithmetic that is obviously safe at one grid is not safe
across a family of them.

## 3. The overflow band: the struck member, restored with a derivation, and a closed form for the rest

The consolidation struck "every float operation" from the band sentence because it had no derivation
anywhere in forty-four files (`49:196-202`), and that was the right call on process. It has one now, it
is true, and the mechanism is exactly the moving exponent, which is why nobody derived it while
reasoning about fixed quanta.

**The float member, exhaustive at the model width** (`probe_2_band.rs`, part 1). The band is the set of
exact results strictly above the largest finite value and strictly below the midpoint between it and the
next point of the unbounded-above grid: values that round-first delivers as max and classify-first would
have refused on range before rounding.

| format | finite values | ordered pairs | band inhabited, add | mul | div |
|---|---|---|---|---|---|
| p=3, e in [-2,3] | 56 | 3,136 | 48 | 0 | 0 |
| p=4, e in [-3,4] | 144 | 20,736 | 144 | 20 | 0 |
| p=5, e in [-4,5] | 352 | 123,904 | 384 | 48 | 0 |
| p=4, e in [0,0] | 18 | 324 | **0** | 4 | 0 |

The last row is the control and it is the whole argument. A one-binade `Ranged` numeral is a fixed-point
numeral wearing a float's clothes, and it reports the band **empty for addition**, agreeing with the
consolidation's compiled same-format fixed-point result (`49:188-190`). Same code, same quantiser, same
sweep; the only thing that changed is whether the exponent may move. The first witness at the model
width is `0.015625 + 30`, exact sum 30.015625, largest finite 30.

The witness on real silicon needs no model at all: `f32::MAX + 1.0 == f32::MAX` is true, `MAX + ulp/2`
is infinite, and `MAX + ulp*0.4999` is finite. Measured, this target.

**Why floats differ from same-format fixed point.** The exact sum of two values of one fixed-point
numeral is a multiple of that numeral's quantum, so if it exceeds max it exceeds it by a whole quantum
and clears the band by construction. In a float the operands may come from finer grids than the result,
so the exact sum lies on a lattice strictly finer than the result quantum and can land anywhere inside
the band. The band's width is half the top binade's ulp, and every value from three binades down is
smaller than that.

**A closed form, offered as the proof the review recorded as owed.** `49:194-195` records the
dividing-quantum condition for mixed-format addition as compiled (36 inhabited, 4 empty out of 40) with
"the closed-form status ... a proof owed, not yet built". I built a candidate, and **measurement refused
it**, which is worth more than if it had passed.

The candidate was `q_result <= 2 * lattice`. Against exhaustive enumeration it scored 753/1000 for
addition and 639/1000 for multiplication (`probe_2_band.rs`, part 2), failing in both directions. The two
failure modes are the finding:

*Alignment.* With `q1 = 1`, `q2 = 1/2`, `q_r = 3/4`, the lattice is 1/2 and the criterion says empty. But
`max_r = 11.25` is not itself a lattice point, and 11.5 sits inside `(11.25, 11.625)`. The interval's
**position** relative to the lattice matters, not only its width. The candidate had silently assumed
`max_r` is on the lattice, which is true in every dyadic case the review had compiled and false in
general. That assumption is exactly the kind of thing a dyadic-only corpus hides.

*Reachability.* With `q1 = 1`, `q2 = 1/4`, `q_r = 1` under multiplication, 15.25 is on the lattice and
inside the interval, and it is not a product of two in-range operands: `15.25 = 61/4` needs an index
product of 61, prime and above the reachable set. **Products do not cover their lattice.**

The corrected statement is two clauses and only the first has a closed form
(`probe_2b_band_closed_form.rs`):

> **Lattice clause.** The band is empty unless some point of the exact-result lattice lies strictly
> inside `(max_r, max_r + q_r/2)`. For an operation whose exact results form a subgroup this is decidable
> from the three quanta alone, by one Euclidean division.
>
> **Reachability clause.** That point must be an actual exact result of two in-range operands.

Measured over 5,184 triples of rational quanta and sizes:

| operation | agreement with exhaustive enumeration | under-predictions |
|---|---|---|
| addition | 5,006 / 5,184 | **0** |
| multiplication | 4,057 / 5,184, 1,127 over-predicted | **0** |

Zero under-predictions in both is the useful half: **the lattice clause never claims empty when the band
is inhabited**, so it is a sound certificate for the one direction a compiler or a build layer would act
on. Its residual over-predictions are all reachability: for addition, sums of two bounded index ranges
do not fill their lattice near the top of the range; for multiplication they do not fill it anywhere.

Read on every member of the original sentence, with the lattice named per member:

| case | exact-result lattice | q_r | band |
|---|---|---|---|
| fixed, same format, add | q | q | empty |
| fixed, same format, mul | q² | q | inhabited |
| fixed, mixed, dividing quanta, add | the finer quantum | the finer quantum | empty |
| fixed, mixed, non-dividing, add | gcd(q1, q2) | the finer quantum | inhabited |
| float, both operands in the top binade | the top quantum | the top quantum | empty |
| float, one operand three binades down | the finer quantum | the top quantum | **inhabited** |

The dividing-quantum condition is the special case where `q_r` is itself a lattice multiple, and there
the clause reduces to `gcd(q1, q2) >= q_r/2`, which is the condition file 44 measured. Division has no
row because its exact results are not lattice-valued at all: the clause's premise fails, reachability is
the whole question, and that is precisely why file 43 had to compile that member rather than derive it.

*grounded on: `model`, `pin`, `ffl` for the exhaustive rows; `host` for the silicon witnesses.*

## 4. What the settled machinery does, member by member

### 4.1 The identity contract: unchanged, and the exponent becomes a type

`Ranged<EMIN, EMAX, U, S>` is already in the ratified table, and the table spells it

```rust
pub struct Ranged<const EMIN: Exponent, const EMAX: Exponent, U: Underflow, S: Specials>;   // 49:117
```

**with the exponent bounds as const parameters, which does not compile once `mulnum` exists.** The
consolidation carries the contradiction internally rather than in one place: section 1.2's table says
`const`, and section 1.15 (`49:552-563`) derives that they must be types, without editing the table it
contradicts. I flag it here at the same volume the review flagged the `Int` drop, because it is the same
shape of thing, an edit to a ratified declaration line, and because a reader taking the table at face
value will build the const form and hit the wall three files later.

What the float model forces is the spine rule's consequence, which section 1.15 derived as a first read,
reasoned and uncompiled, and asked a second member to check (`49:801-806`). **I compiled both halves and
the derivation holds.**

`probe_3_exponent_as_type.rs` builds the signed exponent as `EZero | EPos<P> | ENeg<P>` over the sealed
`Pos`, sealed at birth per the carrier-at-birth rule, with nine constructor-headed `ESum` impls, and
`mulnum` over two `Ranged` numerals. It compiles with no unstable feature at all, and every claim is
forced through a const assertion and through a function signature rather than left in an inert alias,
because file 46 already established that a bare alias defers its bound checks and a suite built from
aliases is green while asserting nothing.

```
M1 = p=4, e in [-3, 4]      M2 = p=3, e in [-2, 3]
M1 * M2 -> p=7, e in [-5, 7]                       asserted
binary32 * binary32 -> p=48, e in [-252, 254]      asserted
```

`probe_3b_exponent_as_const_refused.rs` is the negative control, and it is the part that makes this a
finding rather than a preference. The const form does not exist:

```
error: generic parameters may not be used in const operations
    type Out = Fl<{ P1 + P2 }, { E1N + E2N }, { E1X + E2X }>;
                    ^^ cannot perform const operation using `P1`
    = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

`generic_const_exprs` is forbidden. Under `min_generic_const_args` the same source reports "complex const
arguments must be placed inside of a `const` block"; written as a `const { }` block it reports "generic
parameters may not be used in const operations ... add `#![feature(generic_const_args)]`", and
`generic_const_args` requires `-Znext-solver=globally`, which the workspace's own record documents as
mutually exclusive with the rest of the arrangement. **Every permitted route through const position is
closed.** The exponent is a type or the exact-widening family is unwritable over `Ranged` numerals.

One repair worth recording, because the compiler taught it. The first version of the
negative-plus-positive impl reused the difference helper with its arguments swapped. rustc refused with
`the trait bound Z: Dec is not satisfied`: `Cmp<7, 4> = Gt` selects the branch computing `4 - 7`, which
walks the natural subtraction off the bottom of `Nat`. The natural subtraction refusing to go negative
is the tower working. The repair is to compute the magnitude difference once and apply the sign
afterward, through a three-impl `NegE`, which is the same separation `Bias` already makes between
magnitude and constructor sign. That is a third independent arrival at the constructor-sign shape, and
it is the argument section 1.15 owed the `Int` drop: the future signed-exponent consumer does not consume
`Int` either.

*grounded on: `pin`, `ffl`, `vu`, `seal-owed` discharged; the spine rule from `44b` via `49:59-72`.*

### 4.2 The value-unique encoding: extended, not disturbed

The exponent joins `Nat`, `Pos`, `Adjustment` and `Bias` as a carrier, and it owes the two-obligation
checklist at declaration time rather than after three passes. In `probe_3` it has both: a private
supertrait in its own `exp_sealed` module, and every impl sitting on a closed constructor whose type
argument is re-bounded on `Pos`. That is the carrier-at-birth rule applied to the newest carrier the
review has minted, and it cost two lines.

The projection-chain constraint holds by construction: nothing in `ESum`, `SignedDiff`, `NegE`, `Cmp` or
`NAdd` names `Reduce`, and every impl pattern-matches on constructor heads, so the solver has no
unconditional candidate to eagerly confirm for an abstract operand. `mulnum` reaches a consumer-facing
signature and compiles.

### 4.3 The grade: IEEE's flag word is the design's grade with the value thrown away

This is the convergence I did not expect to find and it is the strongest single result in this file.

IEEE 754-2019 clause 7 names five exceptions: invalid, divideByZero, overflow, underflow, inexact. The
design's grade is a free commutative monoid over refusal causes and quantisation events, joined by union
(`49:222-227`). Over a five-element generator set with no multiplicity, that monoid **is** a five-bit
word joined by bitwise or, which is exactly the sticky flag register, bit for bit. `model.rs:118-152` is
both objects at once, and it needed no adaptation to serve as either.

The design's own two-part split of the generator set lands exactly on the standard's:

| design | IEEE | why |
|---|---|---|
| quantisation event | inexact, underflow | raised by the quantiser, on a value it still delivers |
| cause with no quantiser origin | invalid, divideByZero | raised by the operation, on operands, before any rounding |
| either | overflow | raised by the classification step, which is the quantiser's second half |

Two consequences the spec should take.

**The design's carrier is strictly better than the standard's, and the reason is mechanical rather than
aesthetic.** IEEE's flag word is a single per-thread register that accumulates and is read later. The
design's grade rides on the value. Under a pluggable executor, which is the consumer's declared shape,
a per-thread accumulator is nondeterministic on unchanged data for exactly the reason section 1.14
already gives for the short circuit: the partition into threads is the executor's choice, and the flags
land in whichever thread's register did the work. The value-carried grade has no such dependence. The
review reached this conclusion for its own reasons before it knew the standard's mechanism has the
defect it was avoiding.

**And the standard's carrier is not available to us in any case.** On the pinned toolchain there is no
`fetestexcept`, `feclearexcept`, `fegetround` or `fesetround` anywhere in `core` or `std` (grep of the
`rust-src` component: zero files). There is no FPCR access in `core::arch::aarch64`. The x86 route,
`_mm_setcsr`, is deprecated since 1.75.0 with the note "use inline assembly instead". A design that
wanted to mirror IEEE's flag mechanism could not read it. The value-carried grade is not merely
preferable; it is the only carrier that exists.

*grounded on: `pin`, `tree` of the rust-src component; the mapping itself is reasoned from clause 7 and
compiled as a table in `probe_6_specials.rs`.*

### 4.4 Specials: the value half checks against the machine, the cause half cannot

`probe_6_specials.rs` writes the design's own class-level table from the standard and checks it against
binary32 over every combination of `{+0, -0, +1, -1, +3, +inf, -inf, qNaN, qNaN', sNaN}` under add,
multiply and divide: **300 cases, 0 mismatches.** The first draft had twelve, all of them mine, and two
of the three repairs are informative:

- The sign of a zero product is the xor of the operand signs, not the sign of whichever operand was
  matched first. An easy thing to get wrong and an easy thing to never notice, since it only shows in the
  sign bit of a zero.
- **Finite plus finite is not decidable at the class level.** Exact cancellation delivers a zero, which
  is a different class. The specials table is therefore not closed: it cannot be a total function from
  classes to classes, and the spec should say so, because the natural way to write it invites exactly
  that shape. `1.0 + (-1.0)` is the counterexample and it is not exotic.

**The cause split compiles** (`49:456-458` records it as reasoned and awaiting this model): `x/0` with
`x` finite and nonzero delivers a correctly-signed infinity and raises divideByZero only; `0/0` and
`inf/inf` deliver a quiet NaN and raise invalid; `inf/0` delivers infinity and raises nothing, because
divideByZero is defined on finite operands. The value half of every one of those agrees with the
hardware. The cause half agrees with nothing, because nothing can read it, which is the point of the
previous section.

**NaN payload propagation is silicon, and it is not commutative.** Measured on this target, with
constant folding defeated by `black_box`:

```
qNaN(payload 1) + qNaN(payload 2) -> 0x7fc00001
qNaN(payload 2) + qNaN(payload 1) -> 0x7fc00002
sNaN(payload 1) + 1.0             -> 0x7fc00001   quieted, payload preserved
0.0 / 0.0                         -> 0x7fc00000   the default NaN
```

The first two lines are the design-relevant one. **Addition is commutative at the value level and not at
the datum level, on the machine, today.** That is not a defect to route around; it is a direct
vindication of two already-ratified positions: that `Encoding` may change which datum carries a value
while `Lowering` changes no value (`49:151-152`), and that law equality is the canonical quotient
(`49:229-230`). A commutativity law for a float numeral is true under the quotient and false on the
datum, and the design already forbids a law from reading the encoding, so the law is statable and
correct. Had the design not made that split, the float model would have forced it.

The fourth line is the other half: an operation with no NaN operand produces the target's default NaN,
so the payload is not a function of the operands in general. `Canonical` has to be able to **describe**
this rather than fix it, since it differs by architecture, and it is a datum fact so no law reads it.

### 4.5 The fold: the accumulator that makes a float fold exact is a fixed-point numeral

The accumulator sufficiency condition was derived over fixed quanta, and the brief asks what it does
when the exponent moves. It survives, it stays finite, and the object it names is one the field already
knows: the exact accumulator, called a quire in the posit standard and a long accumulator in Kulisch's
work. The derivation is one line in the design's own vocabulary and I have not seen it stated that way:

> A `Ranged` numeral's entire representable set is contained in the single grid of quantum
> `radix^(EMIN - p + 1)` bounded by `radix^(EMAX + 1)`. Therefore the exact sum of `n` values of a
> `Ranged` numeral is exactly representable in an **`Implicit`** numeral of that quantum and width
> `(EMAX + 1) - (EMIN - p + 1) + ceil(log2 n)`, and interior safety for a float fold is satisfiable at
> that width, by a numeral of the design's other kind.

Checked, not asserted (`probe_4_accumulator.rs`), at p=4, e in [-3,4], accumulator quantum 2^-6:

- **2,924,207 ordered triples**, every one exactly representable in the accumulator, widest magnitude
  seen 13 bits against a formula predicting 13. At n=8 with the worst input the formula predicts 14 and
  14 is what it takes. The width is tight, not generous.
- **139,721 orderings** of sampled 4- through 8-tuples, every rotation and reversal, all agreeing. The
  exact accumulator is grouping- and permutation-invariant by construction and measured to be so.
- The same folds with the accumulator held **in the format**: 2,052,336 inexact interior quantisations,
  and **23.17% of triples deliver a different result under left- against right-association**. That number
  is what interior safety buys, quantified, for floats.

At the real formats the width is large and finite:

| format | exact sum accumulator | exact dot-product accumulator |
|---|---|---|
| binary32 | 277 bits + ceil(log2 n) | 554 bits + ceil(log2 n) |
| binary64 | 2,098 bits + ceil(log2 n) | 4,196 bits + ceil(log2 n) |

Three things follow that the spec should carry.

**The float fold's accumulator is not a float.** It could be one, at p >= 277 for binary32, but the
fixed-point form is exact by construction with no normalisation and no rounding logic in the loop, so
every interior step is an integer add. The design gets to say this in one sentence because both kinds are
one formalisation; a design that treated floats as a separate kind would have to introduce the quire as a
new object with new rules.

**It is a fourth reading of the growth-class question, and it agrees with division.** Fold over a fixed
numeral grows with `ceil(log2 n)`. Multiplication grows quadratically in precision. Division was found to
grow as `Theta(2^p)` (`49:434-437`). A float fold grows as `ceil(log2 n)` plus a constant that is
`Theta(2^w)` in the exponent field width, since `EMAX - EMIN` is exponential in `w`. So the exponential
class is not division's peculiarity; it is what appears whenever a field width indexes an exponent, and
division and the float accumulator are two instances of it. The design should name the class rather than
attach it to one operation.

**The condition's statement does not change.** Interior safety is still "no quantiser fires in the
interior", total safety is still "the accumulator is invisible in the delivered function". Only the
formula for the sufficient width changes, and it changes by substituting the exponent span for the fixed
quantum ratio. That is the strongest evidence in this file that the founding idea is right: the condition
was derived over one kind and needed no restatement for the other.

*grounded on: `model`, `pin` for the exhaustive rows; the real-format widths are arithmetic on the format
parameters and carry no evidence bin beyond that. The names quire and long accumulator are recalled, not
verified against a source in this dispatch; the widths above are derived here and should not be read as
quoting anyone's published constant, which differs by guard-bit convention.*

### 4.6 Laws and views: one law changes truth value, and the design already predicted which

Associativity of addition is false for a `Ranged` numeral at the format width (23.17% of model triples
disagree) and true for the same numeral folded through the exact accumulator, because the interior is
then exact and the single root quantisation takes a grouping-independent argument. That is not new; it is
the design's existing interior-safety statement. What is new is that the compiler agrees with it, and
that agreement is a licence the design can sell.

Measured, `rustc -O`, aarch64-apple-darwin, from `--emit asm`:

```
f_add:   fadd s0, s0, s1        one instruction, no rounding-mode operand
f_fma:   fmadd s0, s0, s1, s2   one instruction, exact product, single rounding
f_sum:   5 scalar `fadd s`, 0 vector fadds      LLVM will not reassociate a float reduction
i_sum:   8 vector adds                          LLVM reassociates an integer reduction freely
```

The vectoriser refuses the float reduction because it has no licence to reassociate, and takes the
integer one because associativity is a theorem there. **The design's interior-safety condition is exactly
the licence the vectoriser lacks.** A fold that runs through an exact accumulator may be reassociated
soundly, and a build layer that can read that condition off the monomorphised type can hand the
vectoriser the permission it will not take on its own. That is the concrete form of "laws as backend
licences" for the float case, and it is measured rather than argued.

The `fmadd` line is the droplist entry compiled: `mul_add` is one instruction computing an exact product
and rounding once, and it differs from multiply-then-add on real inputs (`5.9604638e-8` against `0`,
probe 5). It is a different operation, not a permission.

*grounded on: `pin`, `host`, `flags` (`-O`, `--emit asm`, no other flags).*

## 5. The three categories the brief asks for

### 5.1 Expressible in the design's own vocabulary, with no new axis

The representable set (a union of `Implicit` grids). The quantiser (round-first, plus grid selection).
The identity contract. The value-unique encoding, with the exponent as a fourth carrier. The grade, which
turns out to be IEEE's flag word. The specials table's value half. The fold's sufficiency condition. The
overflow band. Division's float path, including the cause split. The laws, under the canonical quotient
the design already requires.

That is more than I expected going in. The honest summary is that **the float model needs no new
mechanism, and the review's forty-nine files of machinery absorb it.**

### 5.2 Needs an axis, and the axis is already named

`Underflow` is in the ratified table and this is the file that populates it. Three instances, and they
are three different kinds of thing, which is worth stating because the natural reading is that they are
three values of one knob:

- **`Gradual`** extends the bottom grid to zero. It changes the **representable set**. It is a `Numeral`
  fact.
- **`Abrupt`** leaves a hole between zero and the smallest normal, and an underflowing result is a
  refusal rather than a value. It changes the **representable set and the totality of the operations**.
  It is a `Numeral` fact and it interacts with `Refuse` exactly as the dither question does.
- **`FlushToZero`** does not change the representable set at all. Zero is already in it. FTZ is a
  **rule about what an operation delivers**, applied after rounding. It is a `Policy` fact wearing a
  `Numeral` fact's clothes, and if it is placed on `Numeral` alongside the other two, the axis is
  carrying two different kinds of claim in one slot.

**Proposal.** `Underflow` on `Numeral` has two instances, `Gradual` and `Abrupt`, both of which answer
"what is representable near zero". Flush-to-zero is a `Quantisation` resolution, beside the existing
resolutions, because that is what it is: a rule for resolving a classification. This is the same split
D69 already made between mathematical and encoding coordinates, applied one level down, and it has a
practical payoff: FTZ is a property of the *execution environment* far more often than of the format
(section 5.3), and a consumer who has to name it should be naming a policy, not redeclaring their number
type.

One sub-fork, and I recommend the spec state it and then decline to carry it. IEEE permits tininess to be
detected **before** rounding (the exact value lies inside the smallest normal binade) or **after** (the
rounded value does). The two differ only on whether the underflow flag is raised, never on the delivered
value. `model.rs` implements both (`quantize`'s `tiny_after`, and `tiny_before` beside it) and, since
there is no way to read a flag from this language, **the fork is unobservable**. It should be recorded in
the spec as a known degree of freedom the design deliberately does not expose, rather than silently
resolved, because a reader coming from the standard will look for it.

`Specials` wants three instances rather than two: none, infinities only, and IEEE (infinities plus quiet
and signalling NaN). The middle one is not decorative; a saturating format with an infinity but no NaN is
a real shape, and `ExactWindow`'s `Specials = None` gate needs to distinguish it from full IEEE. A
signalling NaN needs no axis of its own: reading one is an operation, and the design's grade already
carries what an operation raises.

### 5.3 Properties of the execution environment, which the design can only declare a contract about

This is the category the brief says is real, and it is realer than I expected. Everything here is
measured, on this target, in `probe_5_execution_environment.rs`, by writing FPCR directly through inline
assembly and recomputing the identical Rust expression.

FPCR on entry to a Rust program on this target is `0x0000000000000000`: round-to-nearest-even, flush-to-
zero off. That is the good case and it is not guaranteed.

```
1.0/3.0 under the entry mode:            0x3eaaaaab
1.0/3.0 with RMode=toward -inf:          0x3eaaaaaa     differs
1.0/3.0 with RMode=toward zero:          0x3eaaaaaa     differs
MIN_POSITIVE * 0.5, entry mode:          0x00400000     a subnormal
MIN_POSITIVE * 0.5 with FZ=1:            0x00000000     flushed
```

The same source expression, the same binary, two answers, selected by a register no type mentions.

**And the compiler is on the other side of the same line.** Constant folding is performed by rustc's own
IEEE implementation, not by the FPU, and it does not consult a register that does not exist at compile
time:

```
const-folded 1.0/3.0     0x3eaaaaab   runtime under RZ+FZ 0x3eaaaaaa   agree: false
const-folded subnormal   0x00400000   runtime under RZ+FZ 0x00000000   agree: false
const-folded qNaN(1)+1.0 0x7fc00000   runtime               0x7fc00001   agree: false
```

The third line holds even at the default FPCR: constant folding canonicalises a NaN payload that the FPU
would have propagated. **A `const`-evaluated float expression and the identical runtime expression can
disagree in the value, in the underflow behaviour, and in the datum, and nothing in the type system sees
any of it.**

For a design whose quantiser is `const`-callable and whose whole verification story rests on a
model-width check transferring to real widths, that is not a footnote. It is a boundary, and the spec has
to state which side of it every claim lives on.

**What the design owes the layer below, stated as a contract rather than as a limitation.** Three
clauses, and the first is a derivation from already-ratified text rather than a new rule:

1. **A hardware-float lowering is not a `Lowering` under the design's own definition, unless the
   environment is pinned.** `49:151` states that `Lowering` changes no value and only `Encoding` may
   change which datum carries a value. Flush-to-zero turns a subnormal into a zero: a different **value**,
   measured above. A non-default rounding mode changes the delivered value of a division: measured above.
   So lowering a `Ranged` operation to a hardware float instruction, in an environment whose control
   state is not pinned, violates a ratified invariant. The design does not have to add a rule to forbid
   it; it has to notice that it already did.

2. **Therefore the default lowering of a `Ranged` numeral's operations is the software quantiser**, the
   same one `Implicit` numerals use, which reads its direction from the type and cannot be perturbed by a
   register. This is not a performance concession being smuggled in: it is the only lowering that
   delivers what the type says. A hardware-float lowering is an opt-in `Lowering` instance carrying a
   declared environment requirement.

3. **The requirement is discharged by the build layer, through the mechanism the design already has.**
   `49:706-713` keeps the post-monomorphisation verifier, the per-axis liveness check and the
   build-layer receipt. The receipt for a hardware-float lowering names the control state it assumed
   (rounding direction, FZ, DAZ, and on x86 the MXCSR equivalents). What the design asks back is one
   sentence: *any code in the process that writes the FP control register invalidates every such receipt
   in scope, and the build layer that issued them owes either a proof that no such write happens or a
   restoration discipline around every call that might.* That is not hypothetical: audio callbacks and
   some game runtimes set FTZ on entry as a matter of course, and a library compiled under a receipt
   assuming FZ=0 will silently deliver different values inside one.

4. **arvo can offer one checkable door and should.** Reading FPCR is three instructions of inline
   assembly, no syscall, no platform dependency, cfg-gated per architecture, which is Kind 1 structural
   lowering under the always-optimal-internals rule rather than a policy decision. A debug-build assertion
   at the entry to a hardware-float-lowered region, comparing the live control state against the one the
   numeral declared, converts a silent wrong answer into a loud one. It is a diagnostic, not a directive,
   which is the posture the toolbox rule asks for.

*grounded on: `pin`, `host`, `flags`; the FPCR bit assignments are read from the Arm A-profile
architecture reference and the behaviour is what the machine did, not what the manual says it should.*

## 6. What this does to the open list

Five items in section 3 of the consolidation are affected, three of them closed.

**The float model** (`49:795-800`, named the keystone). Built and checked as above. What remains unbuilt
of it: the `Specials`-carrying model numeral exists here in Rust as a runtime model, not as a type-level
numeral with `Specials` instantiated; probe 3 builds the type-level `Ranged` numeral with the axis
present but does not run the files-30/31 exhaustive crossing check against it. That join is the next
increment and it is small, because both halves exist.

**The exponent-as-type fork** (`49:801-806`, wanting a second independent read plus a compile). This file
is that second read and it agrees, with the const route compiled shut under every permitted feature. The
consolidation asked for the reading to be formed from `40:690-691`, the `Ranged` declaration and the
exact-widening family's own gate; I formed it instead from the float model's own requirements and arrived
at the same answer by a different route, which I think is the better shape of corroboration but is worth
naming so the second-read discipline is not recorded as satisfied on a false basis. **The two-expert
threshold is met on the mechanism** (file 48 reasoned it, I compiled it); it is not met on whether the
`Implicit` numeral's single exponent should move to a type at the same time, which I did not test.

**The overflow band's struck member** (`49:196-202`). Restored with a derivation and a control, plus a
closed form for the dividing-quantum condition that `49:194-195` recorded as owed. The closed form is
one clause of two and I say so.

**Division's float path and the cause split** (`49:456-458`). The value half is compiled and agrees with
the hardware on 300 class-level cases; the cause half is stated and shown to be unobservable.

**The `TotalOrd` level annotation** (`49:844-845`, "a one-sentence fork nobody has picked"). The float
model decides it, and I will state the fork's answer rather than leave it: the NaN payload measurements
show that any total order placing NaN consistently is a **datum-level** operation if it distinguishes
payloads and a **value-level** operation if it does not. IEEE's own `totalOrder` predicate distinguishes
them, and therefore is datum-level and forbidden to laws. The design should ship a value-level
`TotalOrd` that places one NaN class, usable by laws, and name IEEE's `totalOrder` as a separate
datum-level predicate that is not. That is two operations rather than one annotation on one operation,
which is why the fork looked like it had no good answer.

## 7. What I did not settle, and what I would measure

**The cost of a subnormal, and of the software quantiser.** Both are timing claims and both belong in
`mock/benches/` under the harness, not here. What is worth measuring, and what I would put in the bench:
the software quantiser for a `Ranged` numeral against a hardware `fadd` on the same data, at the four
strategy markers, with the subnormal fraction of the input as a swept parameter. The reason to sweep it
is that the historical penalty for subnormals is enormous on some x86 cores and reportedly absent on
Apple silicon, and if the second is true then the usual argument for FTZ does not apply on this target
and the design's default (software quantiser, gradual underflow) costs less than a reader would assume.
I am not going to assert either number from memory. The bench is the answer and it is one afternoon.

**Whether the model-width transfer argument holds for `Ranged` numerals.** File 08 established that an
exhaustive const-eval check quadruples per bit and rustc refuses at nine, and the workspace rule records
that the `specialization` and `TypeId` bans are what let a model-width check transfer to real widths. For
an `Implicit` numeral the transfer argument is about the precision. For a `Ranged` numeral there are two
widths, precision and exponent range, and the interesting behaviours cluster at the **ends of the
exponent range** rather than in its middle. A model with p=4 and a six-binade range exercises both ends;
a real format has the same two ends and a long uninteresting middle. I believe the transfer is sound and
I have not proved it, and it is a different argument from the precision one, so it should not be recorded
as covered by it.

**Decimal.** Everything above is radix 2. The `Radix` axis exists and decimal64 is the stated reason
(`49:120-121`). A decimal `Ranged` numeral has cohorts, which means the encoding is not injective and the
crossing contract's third statement (`49:166`) is exercised for real rather than vacuously. I did not
touch it and it is the obvious next probe after the type-level join in section 6.

**The single-binade control's multiply row.** In probe 2's table the one-binade format reports the band
empty for addition and inhabited for multiplication (4 cases). That is the correct fixed-point answer for
both, so the control behaves, but I did not check that those 4 are the same 4 a purpose-built fixed-point
sweep would find. A reader wanting the control to be airtight should.

## 8. What I would put in the spec, verbatim

Six sentences, offered in the form the next consolidation could take.

> A `Ranged` numeral denotes a finite union of `Implicit` grids indexed by an exponent interval, with the
> bottom grid's treatment given by `Underflow`. Quantising to it selects the grid from the exact value's
> magnitude, then rounds and classifies exactly as for an `Implicit` numeral.

> The exponent bounds are types, not const parameters, because `mulnum` computes them and they appear in
> the result numeral's type, and every const route to that is closed by the forbidden-feature list.
> `EZero | EPos<P> | ENeg<P>` over the sealed `Pos`, sealed at birth.

> The five IEEE exceptions are the grade's generator set for a float numeral. Inexact and underflow are
> quantisation events, invalid and divideByZero are causes with no quantiser origin, and overflow is
> raised by the classification step. The grade is per value; the standard's flag register is per thread
> and is not readable from this language in any case.

> Interior safety for a fold over a `Ranged` numeral is satisfied by an `Implicit` accumulator of quantum
> `radix^(EMIN - p + 1)` and width `(EMAX + 1) - (EMIN - p + 1) + ceil(log2 n)`. The condition's statement
> is unchanged from the fixed-quantum case; only the sufficient width differs.

> `Underflow` on `Numeral` is `Gradual` or `Abrupt`, both of which change what is representable.
> Flush-to-zero changes no representable set and is a `Quantisation` resolution. Tininess-before against
> tininess-after is a degree of freedom the design records and does not expose, because it changes no
> delivered value and no flag is readable.

> A hardware-float lowering changes values under a control state the type cannot see, so it is not a
> `Lowering` under this design's definition unless the environment is pinned. The default lowering of a
> `Ranged` operation is the software quantiser. A hardware-float lowering is opt-in, carries a declared
> environment requirement in its build-layer receipt, and is invalidated for the whole scope by any code
> that writes the FP control register.

## 9. Where I disagree with something the review settled

One, and it is small and about wording rather than substance.

`49:198-202` strikes "every float operation" from the band sentence with the reasoning that "a claim with
no derivation anywhere in forty-four files was never a claim". The process is right and I would not
reverse it. But the sentence that replaced it treats the member as **absent** rather than as **unknown**,
and those are different states: the consolidation's own droplist entry (`49:1017-1021`) says the member
"is struck rather than corrected", which reads to a later member as though the question had been answered
in the negative. It had not been answered at all. The distinction matters because the grounding registry's
whole point is that a claim carries what it rests on, and "no derivation exists" is a ground worth
recording under `unknown`, which the registry has a slot for (`49:637`), rather than a reason to remove
the row. Had the row survived as `unknown`, the four threads that converged here would have had a marker
to converge on rather than converging by coincidence.

Concretely: strike the claim, keep the question, and let the registry hold it. That is a small edit to a
convention that is otherwise working well, and this file is the case that shows the difference.
