# 34. The three halves assembled

**Member:** Fabian Giesen. Basement lens: the seams. A design split into what a number is, how the
laws over it are stated, and what both cost when lowered is three views of one object or it is three
objects, and the difference is invisible from inside any one view. Compression instincts apply: two
statements of one condition are an unfactored model, and the copies will decorrelate. Where a claim is
cheap to compile, compile it before building on it.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed,
9 ignored, matching the counts files 31 through 33 each report from their own runs. Files 31 and 33
walked the bodies of the shipped surface this stretch touches (`identity_laws.rs`, the nine
compile-fail pairs, the absence of any shipped algebra trait); I re-verified the negative greps myself
rather than trusting them (`grep -rn "Monotone\|Magma\|AddAssoc\|Distributes\|Associative"
mock/crates/ --include="*.rs"` returns nothing; no `arvo-algebra-contracts` directory exists) and read
`TotalOrd`'s declaration directly (`mock/crates/arvo-numeric-contracts/src/lib.rs:59-68`), which
section 2.4 needs verbatim. Canon gate: the governing calls are the D-numbered ones in
`202607301000_topic.inherited-state-from-the-formalization-round.md` and
`202607301200_topic.the-formalization-spec.md`; I verified D16 (`202607301000:949`), D39
(`202607301000:1499`), D51 (`202607301000:2102`), D69 (`202607301200:121`) and D70
(`202607301200:234-236`) against the topic files directly rather than through files 31 and 33's
citations of them. All subordinate to op's seventh checkpoint (`30b_op_checkpoint_seven.md`): D69
overturned, D39 held, and the dispatch is for convergence. Nothing below overturns a D-numbered call
or the checkpoint; where I touch the ratified axis table I am answering a question the consolidation
itself holds open, and I say so in place.

**What I read:** `26_consolidation_two.md` in full. `30b_op_checkpoint_seven.md` in full.
`31_arntzen_settling_the_identity_contract.md`, `32_aaltonen_does_identity_lower_well.md`,
`33_lamport_the_laws_restated.md` in full, with `31_probes/OUTCOMES.md`, `32_probes/OUTCOMES.md`,
`33_probes/OUTCOMES.md` and the probe sources I name below. The directory listed once. Of the design
rounds, the D-call passages named in the gate, read directly.

**What I compiled or measured, separated from what I reasoned.** Six artifacts in `34_probes/`, each
with a one-row summary in `34_probes/OUTCOMES.md`. `probe_0_revectorise.sh` ran file 32's own
committed model crate, unmodified, under three build shapes, and its output is quoted in section 1.
Probes 1 through 4 are exhaustive `const`-assertion models compiled against the pinned nightly
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`). Probe 5 compiles; probe 5b is committed refusing, on
purpose, and its E0308 is the finding. Two of my own first drafts were refused by the compiler and
both refusals are kept in the probe headers: probe 2's first model size hit
`#[deny(long_running_const_eval)]` at 29 seconds, reproducing the consolidation's section 1.3 cliff,
and probe 5b's first witness was withdrawn when I noticed componentwise pair multiplication is itself
associative, so the honest witness is a spelling mismatch rather than a bracketing mismatch. Sections
1 and 2 are compiled or measured except where marked; sections 3 through 5 are reasoning built on
those results, marked as such.

## 0. The verdict, stated first

The three halves are one design, and the joins are better than anyone stated: three of the four
mechanisms one half was missing turn out to be already built in another half, unconnected. The
crossing contract's canonicalisation (identity half) is the definition of law equality the algebra
half reached for and misnamed. The quantiser's two-stage factorisation (identity half) is exactly the
two conditions of the accumulator theorem (algebra half). The check-build flag discipline (lowering
half, consolidation section 1.6) is the root cause of the lowering half's own unresolved anomaly. Each
join is stated in section 3 in the form the next consolidation takes.

Against that, five cracks at the seams, each found by compiling rather than reading, each with its
repair in hand: file 32's vectorisation anomaly (closed, one flag, section 1); file 33's
distributivity theorem missing its own relation slot (probe 1); file 33's "two spellings of one
condition" being two different contracts (probe 2); file 33's MAC clause failing for biased product
numerals, repaired by one more argument to file 31's own gcd (probe 3); and file 33's law equality
reading a datum through `TotalOrd`, repaired by the mechanism file 30 built for a different reason
(probe 4). Plus one obligation visible only from the lowering side: numeral encodings must be
value-unique as types, or every law about a numeral-producing operation splits into a true value half
and an ill-formed type half (probe 5/5b).

## 1. File 32's vectorisation anomaly, closed, and what it teaches (measured)

File 32 found that a loop which autovectorises as a standalone crate stops vectorising when co-located
in its model crate, "under the same flags that vectorised it standalone" (`32:269-271`), ruled out the
identity contract as the cause, and left the root cause as its open item 4 (`32:358-363`).

The root cause is one flag, and it was hiding in the methodology, not the code. The codegen-inspection
command in `32_probes/OUTCOMES.md:21-23` carries `-C lto=fat` on an unlinked
`--emit=asm --crate-type lib` build. Under LTO, the pre-link pipeline defers the loop vectoriser (with
the rest of the late loop passes) to the LTO backend at link time, and `--emit=asm` on a lib never
runs a link step, so the vectoriser never runs on anything in the crate, including a verbatim copy of
a control that vectorises without the flag. The "identical flags" premise was false in effect: the
standalone control was built without the flag that mattered. Measured, all three shapes, against file
32's committed crate unmodified (`34_probes/probe_0_revectorise.sh`, output verbatim):

```
== shape A: no LTO, --emit=asm ==
  probe_vectorises_verbatim_control: NEON .2d lines = 4
  probe_elementwise_add_fixed_no_assert: NEON .2d lines = 4
== shape B: lto=fat, --emit=asm (file 32's inspection command) ==
  probe_vectorises_verbatim_control: NEON .2d lines = 0
  probe_elementwise_add_fixed_no_assert: NEON .2d lines = 0
== shape C: lto=fat, staticlib (the LTO backend actually runs) ==
    0000000000000400 T _probe_elementwise_add_fixed_no_assert
    0000000000000400 T _probe_vectorises_verbatim_control
    ldp q / add.2d lines: 8
```

Three consequences, in increasing order of value.

**The anomaly is a measurement artifact, not a fact about compilation units.** File 32's corrected
section 6 finding is: nothing about crate size, function count, or `const_trait_impl` defeats the
vectoriser; the flag does, in every crate, deterministically.

**The identity contract's path vectorises identically to the raw control**, in both build shapes where
the vectoriser runs at all. Shape A: four NEON `.2d` adds each, the same eight-lanes-per-iteration
structure. Shape C is the stronger form: under fat LTO with the backend actually running, the full
identity-contract path (`probe_elementwise_add_fixed_no_assert`, through
`add_inlinable::<FixNumeral, HotLowering>`) and the bare-`wrapping_add` control compile to
byte-identical machine code and LLVM folds them to one symbol address. That is the consolidation's own
`assert_ne!(op::<A> as usize, op::<B> as usize)` mechanism (`26:399-407`) observed from the equal
side: the contract's cost over raw code, measured by the compiler's own code-folding, is zero, in the
build shape a consumer ships. File 32's section 5 erasure claim (size and alignment) extends to
emitted code.

**The lesson generalises, and it is the same lesson section 1.6 of the consolidation already carries
from the other direction.** The check-build flags exist because the shipping build inlines every
monomorphisation away (`26:363-376`); they deliberately suppress the optimiser to keep the composition
legible. A codegen-quality question asked of a check-shaped build gets a wrong answer for the same
reason a symbol-visibility question asked of a shipping build does. One flag set per question class:
axis legibility reads the check build, codegen quality reads a shipping-shaped build, and the flag
that crossed over here (`lto=fat`, correct in the fold-detection assertion methodology it was carried
from) manufactured the anomaly. Worth one sentence in the next consolidation's section 1.6, because
this review will keep disassembling things.

What survives of file 32's caution: the posture ("do not lean on autovectorisation as a guarantee",
`32:325-334`) stands on `arvo-always-optimal-internals.md`'s own grounds, and one of file 32's
sub-findings is real and sharpened by shape A: the `assert!(equal lengths)` loop idiom defeats the
vectoriser on this pin with or without the identity contract (`probe_elementwise_add_fixed_equal_len_idiom`
and its no-generic ablation both scalar under shape A), while the plain bounds-checked
`for i in 0..a.len()` shape vectorises with a runtime disjointness check. What does not survive is the
claim that embedding in a large compilation unit defeats vectorisation; that claim should not reach
the consolidation.

## 2. File 33 attacked, and what the attack yields (compiled except 2.6)

File 33's frame survives and is the instrument I used against it: a law names its terms, relation,
value set, quantifier and key, and every crack below is a slot file 33 itself left unfilled in one of
its own claims. The vocabulary import is genuine prior art correctly attributed (the weak, existence
and Kleene equations are the standard partial-algebra ladder; nothing there needed inventing, only
importing, exactly as the file says). The product-numeral associativity I verified by derivation
before trusting the probe: `gcd` distributes over multiplication, so the nested two-step gcd flattens
to the gcd of the seven arity-three monomials, a permutation-symmetric set; associativity and
commutativity are the symmetry. The three-line proof file 33's probe 3 measures is sound. The
accumulator-as-combinator-parameter resolution (its section 4.1) survives on the
`arvo-toolbox-not-policer.md` argument and I carry it forward unchanged.

Four cracks, one per subsection, each with the repair.

### 2.1 The distributivity theorem is missing its own relation slot (probe 1)

File 33's section 6.1: "monotone in each argument holds if and only if distributivity over max holds"
on a chain, checked over three operations (`33:511-517`), with the stated hypothesis that the value
order is total (`33:526-527`). All three operations in its probe are total; the design's one partial
resolution never enters. The file that introduced the relation ladder in its section 1 stated its own
flagship theorem without a relation, which is the exact failure its section 0 names.

Filling the slot splits the theorem. Measured exhaustively on the same sixteen-value model
(`34_probes/probe_1_partial_ops_split_the_distributivity_relation.rs`):

- `Precise` addition (`Refuse` past both ends) is monotone where defined, and distributivity over max
  holds as a **weak** equation, under both readings of max below.
- As a **Kleene** equation it fails under the strict reading of max (an undefined operand poisons the
  result, IEEE 754-2019 `maximum`'s shape): witness `a=-5, b=-4, c=0`, where the left side is defined
  and the right side dies on the low-refusing branch.
- It also fails under the suppressing reading (an undefined operand is dropped, `maximumNumber`'s
  shape), in the complementary direction: witness `a=5, b=-1, c=4`, where the left side refuses high
  and the right side survives through the suppressed branch.
- For the total saturating control, all four readings coincide exhaustively; the splits are properties
  of partiality, not the model.

So the corrected theorem: **on a totally ordered value set, for a total operation, distributivity over
the lattice operations is monotonicity; for a partial operation, monotonicity gives the weak equation
only, and the Kleene-level statement additionally depends on which of the two lattice-operation
variants is meant.** Both variants must exist in the design regardless of this theorem, because IEEE
754-2019 ships both families (5.10's `maximum`/`minimum` and `maximumNumber`/`minimumNumber`, the
2019 revision having replaced 2008's `minNum`/`maxNum` over exactly this class of NaN-interaction
trouble), and `13c`'s standard says an abstraction that cannot express one of them is a defect. That
half is reasoned prior art, not compiled; the split itself is compiled.

**The reification lemma, and what it does to the consolidation's open relation question.** Probe 1's
claim E: replacing `Refuse` with an absorbing special (the `Specials = WithInfNaN` shape the identity
half supplies) converts the Kleene failure into a **weak** failure on the same witness. Both sides
become total and disagree on the value. This generalises: a refusal reified as an absorbing value
turns every definedness split into a value split. Which means the weak/Kleene distinction, the entire
substance of the consolidation's open question about how `Precise` reads (`26:608-617`), is **not
stable under a transformation the identity contract explicitly supports**. The one relation invariant
under the Refuse-to-special reification is the graded one: the multiset of refusal causes and
quantisation events is the same fact however the composition chooses to deliver it. This upgrades the
consolidation's graded reading (`26:205-221`, held as "one reading among several") with a property no
other relation on the table has, and it does so from the identity side, which the graded reading's own
author could not see. I am not resolving the fork (which surface `Precise` ships remains the
what-is-`Precise`-for question, `26:608-617`, untouched); I am reporting that one of its two readings
now carries a stability theorem and the other does not, and that for total compositions the ladder
collapses anyway (Kleene equals weak when nothing refuses, so the ladder is two rungs for total
compositions, three for partial ones, derivably from the resolutions already in the key).

### 2.2 "Two spellings of one condition" is two conditions (probe 2)

File 33's section 4.2 reads the consolidation's two accumulator formulas (`ceil(log2(n-1))` at
`26:157-158`, `ceil(log2 n)` at `26:269`) as one condition spelled twice, proposes shipping the n-1
form as the definition, and calls the n form "one digit wider, so both are safe" (`33:379-382`). They
back different promises, and a spec that ships only the n-1 form silently weakens one of them.

Measured (`34_probes/probe_2_two_accumulator_bounds_two_contracts.rs`, exhaustive over an eight-value
clamp destination, arity four, all five bracketings):

- At the n-1 bound, the fold is grouping-invariant, exhaustively. This is file 33's own section 4.3
  theorem, confirmed. It is the condition the **law** needs.
- At the n-1 bound, the fold is **not** the function `quantize . exact_sum`: four elements of 7 on a
  [0,7] destination have exact total 28, which escapes the 21-top accumulator under every grouping, so
  the fold refuses, grouping-independently, while the destination's clamp resolution specifies
  delivery of 7. A refusal the destination would have absorbed surfaces as an accumulator refusal.
- At the n bound, the fold equals `quantize . exact_sum`, exhaustively. This is the condition the
  operation's **specification** needs, and it is what the DSP guard-bit sizing the consolidation cites
  (`26:269-271`, eight guard bits for 256 MAC steps) actually encodes.

So: two named conditions, not one. **Interior safety** (n-1): no quantiser fires in the interior;
grouping invariance follows; the law's side condition. **Total safety** (n): the accumulator is
invisible in the delivered function; agreement with `quantize . exact_sum` follows; the specification's
side condition. File 33's value-coordinate restatement (its section 4.2, the two conditions "the round
stage is the identity in the interior" and "the resolve stage is") survives for both, with the range
factor `n-1` against `n` as the only difference, and its own good sentence gets a twin: interior
safety is "the quantiser is the identity in the interior", total safety is "the accumulator never
speaks". Ship both, named, with the law keyed on which one the combinator checked.

### 2.3 The MAC clause cracks for biased products, and the repair is one more gcd argument (probe 3)

File 33's fold clause: "For a multiply-accumulate, apply the same two conditions with N replaced by
mulnum(N1, N2)" (`33:704`), with lattice refinement glossed as "the additive-closure condition of
section 3.2 applied to the pair" (`33:366-367`). For biased operands both halves of the gloss fail at
once, measured (`34_probes/probe_3_biased_products_break_pairwise_closure.rs`):

- The product numeral of two biased operands (`A=4, B=2` gives `adjustment 8, bias 4` under file 31's
  formula, `31:399-400`) contains every product (exhaustive; the formula is right about products,
  confirmed a third time) and is **not additively closed**: `2*2 + 2*2 = 8` is not of the form
  `8m + 4`. So no accumulator sharing mulnum's lattice holds even a two-term sum, and the pairwise
  closure predicate (bias over adjustment an integer: 4/8) correctly reports it. The two files agree
  the lattice is open; neither says what the accumulator then is.
- The repair is the same shape as the disease: a j-term sum of products is a Z-combination of the four
  monomials `A1A2, A1B2, A2B1, B1B2`, because the bias monomial now enters with coefficient j and
  joins the lattice instead of standing outside it as a fixed offset. So the biased-MAC accumulator is
  the **zero-bias** numeral with `adjustment = gcd(A1A2, A1B2, A2B1, B1B2)`, one more argument to file
  31's own gcd, bias moved to zero. Checked exhaustively (j = 1, 2 over the full window, j = 4 over a
  reduced one, plus end-to-end grouping invariance of the four-term MAC).

This slots into file 33's frame rather than replacing it: define lattice refinement as **span
containment** (the accumulator's lattice contains the additive span of the value set, which is what
"every exact sum of members" already says if read at arbitrary arity), give the closed form above for
the biased product case, and note the same safe-direction caveat as everywhere else in this stretch:
the span lattice contains the sums and is not claimed finest.

### 2.4 Law equality reads a datum through `TotalOrd`, and the fix was built two files earlier (probe 4)

File 33's section 2.2: law equality is "the equality induced by the composition's total order"
(`33:196-199`), leaning on the shipped `TotalOrd` (`arvo-numeric-contracts/src/lib.rs:59-68`, "a
strict-NaN-policy total order"). File 33 flagged this as its one uncompiled claim in sections 1
through 6 and asked for the check. Here it is, and the claim fails as stated.

The shipped trait's declaration does not say which side of the value/datum split it orders, and the
precedent implementation its doc comment gestures at, `f64::total_cmp` and IEEE 754-2019 5.10
`totalOrder`, is a **datum** order: it separates -0 from +0 and orders NaNs by sign and payload.
Measured on a sign-magnitude model (`34_probes/probe_4_law_equality_is_the_canonical_quotient.rs`,
exhaustive over 32 data): the 5.10-shaped order is total and antisymmetric over the data, and its
induced equality separates the two data of the one zero value and separates two NaN payloads. An
equality induced by it reads the encoding, which the charter forbids of every law (`31:361-363`), in
the very definition of law equality. The observable consequence is a law, not a technicality: under
IEEE 754-2019 6.3 the exact zero from `x - x` is -0 under roundTowardNegative and +0 elsewhere, so
"x minus x equals zero" is rounding-attribute-dependent under the order-induced equality and
attribute-independent under the value-level one; a law whose truth flips on the sign of a zero is
reading a datum.

The repair needs nothing new, which is the point of this file: the crossing contract's idempotent
canonicalisation (`31:370-374`) already collapses exactly the data that carry one value, and comparing
after it coincides with value equality over every datum pair, exhaustively, including reflexivity at
NaN (probe 4, claim C). So:

**Law equality is the canonical quotient: two results are law-equal when canonicalisation sends their
data to the same datum, equivalently when they decode to the same value with each special as one
value-level class.** It is defined by the identity half (`Encoding::Canonical`), needed by the algebra
half, and already priced by the lowering half: zero instructions for every `Specials = None`
composition, a small branchless constant for a range-based collapse (`32:164-203`). The three halves
meet in one definition, and each contributed the part the others could not.

`TotalOrd` survives, reclassified rather than rejected. The charter itself contains the
classification: "every operation whose result depends on [which datum carries a value] is declared a
datum-level operation, and no law may read one" (`31:361-363`). A 5.10-shaped total order **is** such
an operation, and it is genuinely needed (sorting, canonical layouts, and `13c`'s IEEE-as-test
requires 5.10 expressible). So the trait carries a level annotation: either `TotalOrd` is declared
datum-level (5.10-compatible, usable for order, forbidden to laws), or it is specified value-level
(one NaN class placed consistently; usable to induce order on values; still not the definition of law
equality, which the canonical quotient already is). What the design must not do is what file 33's
sentence did: leave the level unstated and let the equality it induces flow into the law layer. The
declaration as shipped states neither; one sentence fixes it, and which sentence is a design choice I
hold open with the two options stated.

### 2.5 What file 33 got right that nobody should reopen

For the checkpoint's benefit, the parts of file 33 I attacked and could not dent, each now with a
second pass on it: the five-slot frame (used throughout this file); the three-relation import and its
ordering (extended, not corrected, by 2.1); the `Direction`-in-the-key predicate and the lattice
closure conditions of its section 3.2 (probe 3 exercised the additive one on a biased numeral and it
behaved exactly as stated); the product-numeral associativity including the n-ary monomial form
(verified by independent derivation, 2.0 above); the accumulator as a combinator parameter with a
derived side condition (strengthened by 2.2 into two named side conditions); the atom-signature
account of recovery maps (its section 5.1; not re-tested here, and its fix is the same shape D51
already ratifies, so it needs no defence from me); and the D16 reconciliation for `Monotone` in both
risk classes (its section 6.1 tail), which my 2.1 refines but does not displace.

### 2.6 One thing neither file 33 nor the identity half can see: the type level (probe 5/5b, and this
subsection is half compiled, half reasoned)

File 33 establishes that `mul_full` is a family of maps between numerals and that its laws are
statements about the numeral-level map first (`33:284-325`). Its probe verified that map's
associativity **as arithmetic**. But the numerals are types, and "the same numeral" in a generic
signature means the same type, judged by rustc, not by mathematical equality of what the types denote.
Whether the two coincide is a property of the encoding, and nothing in files 28, 30, 31 or 33 states
it.

Compiled, both directions (`34_probes/probe_5_*.rs` and `probe_5b_unreduced_refuses.rs`, the latter
committed refusing on purpose): with the rational adjustment (file 28's pair, carried into the settled
contract at `31:204-205` for the closure gap at `26:326-331`) reduced at construction, the two
bracketings of a triple product are one type and a type-equality demand accepts them. Unreduced, "the
same numeral" is spelling-dependent: the product of the reduced numerals 2/3 and 3/4 is spelled 6/12
and does not inhabit the type of the numeral 1/2 a consumer writes directly, refused with
`error[E0308]: expected Adj { num: 6, den: 12 }, found Adj { num: 1, den: 2 }`. The refusal is the
deliverable: no correct expected value exists to write down, because the encoding does not have one
type per numeral. Note the precise shape of the crack: componentwise pair multiplication is itself
associative, so pure product chains unify with each other; the failure is between a product's spelling
and the numeral it denotes, which is exactly where a generic consumer of `mul_full`'s result stands.

The obligation for the spec, stated generally: **every numeral encoding is value-unique, one type per
denoted numeral, established by a stated normal form; a law about a numeral-producing operation is
only well formed as a type-level statement over such an encoding.** The shipped width chain already
satisfies it (typenum-style binary, no leading zeros). Integer adjustments under file 31's biased
formula satisfy it for free, because gcd's output is canonical; the formula is self-normalising, a
property nobody had named. The rational extension does not satisfy it until reduction is stated as
part of the encoding, and the type-level gcd this requires at trait level (the const-generic route is
walled off by the droplist's `generic parameters may not be used in const operations`, `26:719-724`)
has stable-Rust prior art in `typenum`'s `Gcd` but no build and no price in this design. Reasoned on
top of the compiled halves: this is the one genuinely new open item this file adds, and it is the
lowering half's counterpart to the two joins above, the obligation only visible when the algebra's
"same numeral" meets monomorphisation's "same type".

## 3. The assembly (reasoned on the compiled results; the section a consolidation takes)

### 3.1 The design in one paragraph

A number is an integer drawn from a finite interval plus a type-level rule injecting it into the
rationals, with `Specials` as the identity-level extension of the value set and everything about bit
patterns nested inside `Lowering` as `Encoding`, which may choose which datum carries a value and may
never change which value is carried (files 27 through 31, settled). Arithmetic is exact wherever a
numeral exists to hold the result (`mul_full` into the product numeral; fold interiors into an
accumulator numeral); the single approximator is the quantiser, `resolve . classify . round`, fired at
named sites. A law is a claim over values, in five named slots, keyed on everything its proof used and
structurally unable to read `Lowering`; every failure of every law is attributable to a quantiser
firing, and the laws return, unchanged, wherever the quantiser is kept out of the interior. Lowering
is the priced half: every axis is a zero-sized type, the composed operations fold to the raw code
byte for byte in shipping-shaped builds (section 1, shape C), and the apparatus that verifies all of
this runs at a model width whose transfer to real widths is what the workspace's forbidden-features
list protects.

### 3.2 Law equality, defined

Per 2.4: law equality is the canonical quotient. `Encoding::Canonical` is thereby load-bearing twice,
as the crossing contract's idempotent second statement (`31:370-374`) and as the definition of the
equality every law is stated under. Its cost is measured (`32:164-203`): zero for `Specials = None`,
branchless small for range-based collapse. `TotalOrd` carries an explicit level annotation (2.4's two
options; the choice is a design call, not made here). `DatumDeterministic` (`31:404-408`) and a
datum-level `TotalOrd` are the two members of a named class: datum-level derived facts, which may read
`Encoding` and which no law may cite.

### 3.3 The relation ladder, finalised

Three relations, ordered, per file 33's import: weak, Kleene (weak plus definedness invariance),
graded (Kleene plus event invariance). Two additions from this file: for a total composition the first
two coincide, so the ladder is two rungs there and three for partial compositions, derivable from the
resolutions already in the key; and the graded relation is the only one stable under the
refusal-to-special reification (2.1), which is the property that makes it the right relation for any
fact meant to survive a composition changing how it delivers failure. The lattice-operation variants
(strict and suppressing min/max) are both in the design's vocabulary, per the IEEE test, and every
distributivity-family law names which variant it is stated over. The corrected distributivity theorem
of 2.1 replaces file 33's section 6.1 statement.

### 3.4 The fold, with both side conditions

Interior safety (n-1 range factor, plus span refinement): no quantiser fires in the interior; all
three relations hold at once by file 33's section 4.3 argument, which survives unchanged. Total safety
(n range factor): the fold is `quantize . exact_sum`; the accumulator is unobservable. A combinator
states which it checked, and the law it derives is keyed accordingly. Span refinement is defined as
containment of the additive span (2.3), with the closed form for biased products: accumulator bias
zero, adjustment dividing `gcd(A1A2, A1B2, A2B1, B1B2)`. The gcd formula is self-normalising over
integer adjustments and its rational extension carries a stated reduction (2.6).

### 3.5 The key, with one slot moved and one axis question answered

File 33's key table survives with one change and one consequence held as two readings.

The change: **`Growth` leaves the key.** File 33's own section 3.2 establishes that quantiser presence
is syntactically visible in the operation's definition because every quantise call goes through one
funnel (`33:279-282`, resting on `26:236-243`); its key table nonetheless carries `Growth` as a
never-elided slot "deciding whether a quantiser is present" (`33:241`). Those two statements name the
same bit twice. The key's operation slot, spelled as the named operation (`mul_full` against
`mul-then-quantize`, `add_exact` against the composition's bound operator), already determines
quantiser presence; carrying `Growth` beside it is the same condition stored twice, and the copies
will drift. The alternative spelling (a coarse `Op` marker plus `Growth` in the key) has identical
content, and I recommend against it only because the funnel makes the operation-name spelling
checkable by grep where the axis spelling is checkable only by trusting the table; one of the two must
be picked and the content does not care which.

The consequence, held as two readings because it touches the ratified axis table exactly where the
consolidation itself holds it open (`26:54-57`, "whether `Widening` collapses to nine axes is open"):
once `mul_full` targets the product numeral, folds carry explicit accumulator numerals, and `quantize`
is the only narrower, every exact intermediate's carrier is named by a numeral in the term, and
`Widening` as a hidden `Lowering` axis has no referent left. Under that reading `Widening` collapses,
and the Lattner gap the consolidation carries as its oldest structural hole (`26:52-59`,
`Growth::Exact` with `Widening::None` having no carrier) dissolves rather than needing a compatibility
predicate: the unlowerable axis point no longer exists because the axis no longer exists, and
`Precise`'s deviant preset row becomes the preset naming its accumulator default. The other reading
keeps `Widening` as a derived, diagnostic `Lowering` fact (what the presets report about their own
lowering), which costs a redundancy but preserves the ratified table's shape. I hold both; the first
is where the compression instinct points, and the table is ratified, so the collapse is op's call to
make on this file's argument, not mine.

### 3.6 Where facts live, structurally

The value/datum split of 3.2 gets a structural enforcement for free, and it gives the crate split its
first real correctness job since file 11 demoted it to packaging (`26:513-521`). File 11's test showed
a crate boundary cannot stop the `Number`-owning crate from naming `Lowering`; true, and beside the
point for the **facts**, which do not live in the `Number`-owning crate. Value-level facts (the laws,
the atoms, the fold side conditions) live in the algebra-contracts crate, whose dependency set
includes the numeral and policy crates and excludes the lowering crate; a law body there cannot name
an `Encoding` or `Lowering` type even by mistake, because the name does not resolve (`E0433`), which
is the same scope-resolution enforcement the const-fn key discipline already uses (`26:174-186`),
applied one level up and made structural. Datum-level facts (`DatumDeterministic`, a datum-level
`TotalOrd`) live where `Encoding` is nameable, and the crate boundary is now the declaration-site
marking file 33 asked for at `33:174-179` ("mark the distinction at the declaration") with no new
mechanism. The remaining packaging question, which algorithm crates gain the dependency edge
(`26:505-510`), is untouched and remains op's.

### 3.7 Costs, joined

The compiled cost story now covers the joins, not only the halves: law equality is priced (3.2);
`Specials` costs the arm its `ExponentForm` can express (`32:124-149`, unchanged); the contract's
emitted code folds to the raw code's address in shipping shapes (section 1, the strongest form of
erasure available); and the check-build discipline gains the one-flag-set-per-question-class sentence
(section 1). The one edge of the triangle still unpriced is laws-to-compile-time: the atom ladder and
the const-fn key discipline against a real consumer's composition set, which `26:668-674` already
routes to `mock/benches/` and which files 32 and 33 both re-flagged. Nothing in this file changes
that, and I did not fake a probe-sized answer to a bench-sized question; the type-level gcd of 2.6
joins the same bench when it exists.

## 4. What this file does not decide

The `Growth` key spelling and the `Widening` collapse are argued and left as two readings each, the
second explicitly on the ratified table's ground (3.5). The `TotalOrd` level annotation's choice
between datum-level-declared and value-level-specified is stated as a fork with both options live
(2.4). Which combinator surface `Precise` ships remains the standing what-is-`Precise`-for question
(`26:608-617`), now with the reification-stability result of 2.1 as an input to it rather than an
answer. D39's honest content, the dither design choice, division, and the preset-divergence question
op reserved (`30b:30-37`) are all untouched.

## 5. Open, net

Closed by this file, each with an artifact: file 32's open item 4 (probe 0); the relation slot of file
33's distributivity theorem (probe 1); the two-spellings conflation (probe 2); the biased-MAC
accumulator (probe 3); the law-equality definition, at model scale (probe 4). Added by this file, one
item: the value-unique-encoding obligation and its unpriced type-level gcd (probe 5/5b). Standing from
the predecessors, unchanged: event invariance still has no direct measurement (`33:787-789`; probe 1
measures definedness and values, not event multisets); the atom ladder's compile cost (`26:668-674`);
richer canonicalisation branchlessness and cross-word extraction (`32:341-350`); division
(`26:676-681`). One net item added against five closed, and the added one is a compile-fail already in
hand rather than a question.

## 6. Standing

Nothing here overturns a D-numbered call or op's seventh checkpoint. Section 2.1 extends a theorem
file 33 stated one file ago; sections 2.2 and 2.3 correct two of its glosses with its own frame;
section 2.4 completes the check file 33 itself requested and lands on machinery file 30 built; section
3.5's axis consequence answers a question the consolidation holds open and leaves the call where the
ratified table says it belongs. Two of my own drafts were refused by the compiler mid-dispatch and
both refusals are recorded in the probe headers, because a probe that only ever passed is not evidence
it was checking anything.
