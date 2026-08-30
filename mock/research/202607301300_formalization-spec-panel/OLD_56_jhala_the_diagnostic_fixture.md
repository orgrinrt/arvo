# The diagnostic fixture: what a person reads, and what actually moves it

Ranjit Jhala, file 56. I wrote file 03, on what is provable, near the start of this review. Fifty-two
files have landed since and the design I looked at then is not the design that exists now: the value
carried a plain signed integer bias where it now carries a sealed, gcd-normalised rational; the fold
had no grade; the exponent was not yet known to need to be a type at all. Nothing from file 03 is
reused here except the discipline it was written in, which is that a checker's whole value is in what
the person on the other end of the red squiggle reads, and that claim gets tested the same way every
other claim in this review gets tested: compiled, not argued.

**What I read.** `49_consolidation_four.md` in full, as the standing base, then `50` through `55` in
full, then `53b_persona_checkpoint_twelve.md` in full, then an `ls` of the panel directory. Behind
those I opened the probe files my own artifacts had to reproduce or extend rather than reinvent:
`46_probes/vu_nat_sealed_adj.rs` and `vu_bias_sealed_adj.rs` (the sealed tower, copied unmodified),
`46_probes/probe_2_vu_core_lib.rs` and `probe_3_direct_impls_refused.rs` (the seal's own attack,
rebuilt fresh under this dispatch to reproduce its diagnostic rather than trust a transcription),
`47_probes/probe_1b_a_wrong_digit_is_silent.rs`, `probe_1c_the_diagnostic.rs`, and
`probe_6_the_caller_contract_diagnostic.rs` (the decoder-ring finding and the bound-not-equality
lever, both of which this dispatch builds directly on), `48_probes/probe_1_the_wall_is_one_refactor
_away.rs` (the composition-wall reproduction I re-annotate), and `55_probes/OUTCOMES.md` (where I
found, reading it for a different purpose, that the decoder-ring defect had recurred unflagged four
files after the consolidation named it, section 3 below).

**Gates.** Test gate, run fresh from `mock/`: `cargo test --workspace`, summed per binary rather than
a headline, **654 passed, 0 failed, 9 ignored**, matching every file since 41. Canon gate: the surface
this file is about (the numeral tower, the seal, the composition wall, the notation macro) has no
shipped source. `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the
`FullRange\|UTerm\|AddWidth` variant, both from the repo root, both exit 1, empty, reproduced fresh.
Toolchain: every build in `56_probes/` ran on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, resolved from inside `56_probes/`, verified before running anything, because
file 52 paid for the outside-the-tree trap and I decline to pay for it twice.

**What is compiled, and what is reasoned.** Eleven probe files plus a price sweep in `56_probes/`,
every diagnostic in this document copied verbatim from a fresh build, not transcribed from memory or
from a prior file's OUTCOMES.md. `56_probes/OUTCOMES.md` carries the full table and the verbatim text
for every probe. Nothing here is a timing claim; the one price sweep this file runs (`56_probes/
price/`) is `--emit=metadata`, trait-solve-only, min-of-3, the identical harness shape files 41, 42,
53 and 54 used, and it says so at the point it is used. Section 6's design recommendation and section
7's ceiling statement are reasoning, and every claim in them cites the probe it rests on.

## 0. The question, restated as an experiment rather than an essay

The design's whole correctness argument is that illegal states are unrepresentable, so the compiler
refuses them. That argument has a silent premise: that the refusal, once it fires, tells the person
reading it something they can act on. Nobody in fifty-five files had tested that premise directly. Two
threads had found pieces of it and left them exactly where the checkpoint says: the decoder ring
(a numeral's encoding prints as nested constructors, not as a number, and file 47 recorded that it did
not fix this) and the face (whether a notation shorthand's own name survives into an error, or the
thing it expanded to). The checkpoint's phrasing of the ceiling, "the first error in an expression
names the face", is the sentence this file tests against a compiler, not against an argument.

The method: build the errors a real consumer hits, capture what rustc says today with no tuning, then
apply the instruments this design already has access to (bound shape, `#[diagnostic::on_unimplemented]`,
associated-type projection, newtype against alias, `adt_const_params`) one at a time and measure what
each one moves. Where an instrument does nothing, that is a result, stated as plainly as a positive
one. Section 5 is the one place this file predicted a wall before hitting it (the a-priori read that
`#[diagnostic::on_unimplemented]` cannot reach a solver-overflow error); it was still built and
confirmed rather than left as prediction, because the review's own record is that predicted walls
keep turning out to be one formulation's problem, and this one is stated as compiled because it is.

## 1. What already works, unflagged, and should be named before anything else

The most useful finding this fixture produced was not built by me. It was sitting in the tower,
unremarked, because nobody had asked what rustc says about the seal without any diagnostic tuning at
all.

**Rebuilt fresh** (`56_probes/probe_3_direct_impls_refused.rs`, against `56_probes/probe_2_vu_core_lib
.rs`'s tower, itself an unmodified copy of `46_probes/`'s sealed originals):

```
error[E0277]: the trait bound `EvilPos: nat::sealed::PosSealed` is not satisfied
  --> probe_3_direct_impls_refused.rs:20:14
   |
20 | impl Pos for EvilPos {
   |              ^^^^^^^ unsatisfied trait bound
   |
help: the trait `nat::sealed::PosSealed` is not implemented for `EvilPos`
   ...
   = note: `Pos` is a "sealed trait", because to implement it you also need to implement
     `vu_core::nat::sealed::PosSealed`, which is not accessible; this is usually done to
     force you to use one of the provided types that already implement it
   = help: the following types implement the trait:
             vu_core::nat::H
             vu_core::nat::O<P>
             vu_core::nat::I<P>
```

Read this as a message to a person who has never seen this review. It names the trait, states in
plain English what a sealed trait is and why one would exist, and lists the exhaustive set of types
that do satisfy it. Nothing in the tower asked for this. No `#[diagnostic::on_unimplemented]`
anywhere in `vu_nat_sealed_adj.rs`. This is a stock rustc feature: the compiler recognises the
private-supertrait pattern this design uses for every carrier (`49:404-410`) and explains it
unprompted. The design's entire sealing discipline, applied four separate times across this review
(`49:74-87`), gets this diagnostic for free at every site, and the fixture is the first place anyone
checked.

The consequence for the spec: **the carrier-at-birth rule is also a diagnostic-quality rule, and
nobody had to spend anything to make it one.** Sealing a carrier the way this design already seals
one is simultaneously the correctness mechanism and the best diagnostic this fixture found. That is
worth stating in the spec text next to the rule itself, because it is a case where the two concerns
this dispatch was told to keep separate (is it right, is it legible) turn out to be the same act.

*grounded on: `pin`, `tree` of the copied tower files, reproduced fresh this dispatch.*

## 2. The decoder ring, confirmed and found recurring where nobody was looking for it

### 2.1 Reproduced fresh

`56_probes/probe_1_alias_expands_newtype_survives.rs`, the identical shape file 47's probe 1c built,
rebuilt independently:

```
error[E0308]: mismatched types
   |
37 |     needs_face37(c);
   |     ------------ ^ expected `Container<Pz<I<O<I<O<O<H>>>>>>>`, found `Container<Pz<I<O<I<O<I<H>>>>>>>`
```

`49:600-603` states the general claim: "rustc expands type aliases in diagnostics, so the intended
decimal value never surfaces in an E0308 regardless of the notation layer". This confirms it
independently rather than only inheriting it.

### 2.2 It recurred, one file before this one, unflagged

Reading `55_probes/OUTCOMES.md` for its own purpose (the algorithm-crate fixture) turned up a second,
independent instance nobody named. File 55's own `probe_4b_the_constant_and_the_numeral_refuse.rs`
gives its aliases readable names on purpose:

```rust
type P8 = O<O<O<H>>>;         // 8
pub type P8Public = P8;
type P10 = O<I<O<H>>>;        // 10
pub type P10Public = P10;
```

The E0308 it produced (`55_probes/OUTCOMES.md`, "probe_4b, the constant and the numeral refuse"):

```
error[E0308]: mismatched types
   |
27 | pub fn wrong_result_numeral() -> Ranks<Cap64, Num<p4::P10Public>> {
   |                                  -------------------------------- expected `Ranks<Cap64, Num<O<I<O<H>>>>>` because of return type
28 |     upward_rank_widening::<Cap64, Num<p4::P8Public>>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Ranks<Cap64, Num<O<I<O<H>>>>>`, found `Ranks<Cap64, Num<O<I<I<H>>>>>`
   |
   = note: expected struct `Ranks<_, Num<tower::bias::nat::O<tower::bias::nat::I<tower::bias::nat::O<tower::bias::nat::H>>>>>`
              found struct `Ranks<_, Num<tower::bias::nat::O<tower::bias::nat::I<tower::bias::nat::I<tower::bias::nat::H>>>>>`
```

`P8Public` and `P10Public` never appear. The message is worse than file 47's own witness, because the
fully-qualified module path (`tower::bias::nat::O<tower::bias::nat::I<...>>>`) is printed for every
constructor, which the shorter, path-free form in file 47's probe did not carry. File 55 built the
`FromConstantKeyed` fix for a completely different defect in the same probe (an out-of-range constant)
and, in the same file, produced a second live specimen of the decoder ring without naming it as one.

This is the finding worth stating plainly rather than politely: **the decoder ring is not a known,
contained, tracked defect. It is the default outcome of using a type alias anywhere near a numeral,
and it will keep recurring, silently, in every future probe that gives an alias a readable name and
then puts it in a signature, until the design either stops relying on aliases at exactly the positions
where a person reads a diagnostic, or accepts the ceiling and routes around it.** Section 4 gives the
route.

*grounded on: `pin` for the fresh reproduction; `tree` at `55_probes/probe_4b_the_constant_and_the
_numeral_refuse.rs:27-33` for the recurrence, read directly, not inferred from file 55's own prose,
which does not mention it.*

## 3. The face survives declaration, and decays at the first operation unless the operation is also built on it

### 3.1 Alias against newtype at declaration

`56_probes/probe_1_alias_expands_newtype_survives.rs`, second half, a const-generic newtype standing
in for what a notation macro would emit if it minted a distinct type per numeral rather than an alias
over the raw encoding:

```rust
pub struct NFace<const V: u64>(PhantomData<()>);
pub trait NumeralFace { type Encoding: Nat; const V: u64; }
impl NumeralFace for NFace<37> { type Encoding = Face37; const V: u64 = 37; }
impl NumeralFace for NFace<53> { type Encoding = Face53; const V: u64 = 53; }
```

```
error[E0308]: mismatched types
   |
61 |     needs_nface37(c);
   |     ------------- ^ expected `37`, found `53`
   |
   = note: expected struct `FaceContainer<NFace<37>>`
              found struct `FaceContainer<NFace<53>>`
```

`expected 37, found 53`. Not a nest. A newtype is a genuinely distinct type to the checker; an alias
is transparent to it. That is the entire mechanism, and it costs nothing exotic: `NFace<const V: u64>`
is ordinary, stable const generics, no unstable feature, no forbidden gate.

### 3.2 The operation decides whether it survives past declaration

File 04's own residue, carried forward at `53:295-304`: "operations are keyed on the numeral type, so
`mul_full` over two faces delivers a `Number`, not a face, and the unreadable type reappears one
operation into any expression." `56_probes/probe_2_does_the_face_survive_composition.rs` builds both
readings of that sentence, side by side, over the identical mismatch.

**Shape 1**, the operation generic over the raw `Nat`, the face only a call-boundary label:

```rust
pub fn shape1_sum_decays_to_raw(a: Container<Enc37>, b: Container<Enc53>) -> Container<Enc90> {
    sum(a, b)   // sum is generic over Nat, not over the face
}
```

```
error[E0308]: mismatched types
   |
85 |     shape1_needs_the_wrong_sum(s);
   |     -------------------------- ^ expected `Container<Pz<I<O<I<O<O<H>>>>>>>`, found `Container<Pz<I<O<I<I<O<I<H>>>>>>>>`
```

Decayed, one hop in, exactly as file 04 predicted and exactly what section 2 just found recurring
unflagged in file 55.

**Shape 2**, the operation defined on the face itself, its own result a face whose const parameter is
computed:

```rust
pub trait FaceAdd<Rhs: NumeralFace>: NumeralFace { type Out: NumeralFace; }
impl FaceAdd<NFace<53>> for NFace<37> { type Out = NFace<90>; }
```

```
error[E0308]: mismatched types
    |
115 |     shape2_needs_the_wrong_sum(s);
    |     -------------------------- ^ expected `37`, found `90`
    |
    = note: expected struct `FaceContainer<NFace<37>>`
               found struct `FaceContainer<NFace<90>>`
```

Survived. This answers the checkpoint's question directly: **"the first error in an expression names
the face" is achievable, and it is achievable exactly as far as the operation surface is re-derived at
the face layer.** It is not a property of the notation layer alone; it is a property of every
operation the design defines having a face-level sibling. That is a real, priceable obligation
(section 6), not a free consequence of picking a better notation vehicle.

*grounded on: `pin`, both shapes compiled in the same file, this dispatch.*

## 4. The multi-field face, and where it breaks

### 4.1 It scales past a single integer, readably, including negative fields

A real numeral needs more than one number legible at once (precision, a signed rational bias,
`49:99-132`), not a single scalar. `56_probes/probe_3_multi_field_const_face.rs` uses
`adt_const_params` (allowed, `unstable-features.md`) to carry a whole spec as one const value:

```rust
#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Spec { pub precision: u16, pub bias_num: i32, pub bias_den: u32 }
```

```
error[E0308]: mismatched types
   |
55 |     declare_q15(x);
   |     ----------- ^ expected `Spec { precision: 15, bias_num: 0, bias_den: 1 }`, found `Spec { precision: 15, bias_num: 1, bias_den: 2 }`
```

Field-labelled, whole-struct, and the negative-bias witness in the same probe prints the negative
field with no special handling anywhere. This is a genuinely strong result for the notation macro
question: a face is not limited to one integer, and it does not need one type-level trait per member
to stay legible. The whole `Spec` prints as a struct literal, the way a person would write it by hand.

### 4.2 It cannot be sealed structurally, and the reason is the design's own wall recurring

The internal `Adjustment` refuses an unreduced pair at the bound: `N: Pos + Gcd<D, Out = H>` is a
type-level condition, and a non-coprime pair simply never reaches a position bounded by it
(`49:295-346`). A `Spec` const struct has no equivalent. `56_probes/probe_4_the_face_is_a_new_carrier
_and_needs_its_own_seal.rs` builds the cheapest available check, a forced-use const assertion:

```
error[E0080]: evaluation panicked: bias is not reduced to lowest terms
   |
55 |     pub const REDUCED: () = assert!(S.is_reduced(), "bias is not reduced to lowest terms");
```

It works, and it works only when something calls the checking constructor.
`56_probes/probe_4b_unchecked_bad_compiles_silently.rs` is the negative control: with nothing forcing
`checked()`, the identical bad spec compiles clean, exit 0. This is file 46's own lesson (a bare
alias defers its bound check, `46_probes/probe_3d`'s own finding, restated at `52:213-221`) recurring
one layer up, at the notation layer, and it is a real hazard under
`what-you-can-observe-is-what-you-guaranteed.md`: a downstream consumer can hold, pass around, and
store a value carrying a malformed `NFace<S>` indefinitely, and nothing observes the defect until
something happens to touch the one constructor that checks it.

**The obvious fix does not exist, and the reason is the spine rule firing a sixth time.**
`56_probes/probe_6_the_face_seal_as_a_real_bound.rs` tries the standard bridge from a computed boolean
to a real bound, `Assert<{S.is_reduced()}>: True`:

```
error: generic parameters may not be used in const operations
   |
60 |     Assert<{ S.is_reduced() }>: True,
   |              ^ cannot perform const operation using `S`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

`S.is_reduced()` is computed from a generic const and needs to appear in type position. That is
precisely the shape section 1.15 of the consolidation already names four times over (the fold's
grade, `49:66-68`; the `Ranged` exponent bounds, `49:552-563`, compiled by file 50; `Implicit`'s
single exponent, file 54 section 5; `Capacity`, file 55 section 1.1). This is the fifth reachable
position and the first one outside the numeral tower's own recursive expansion: the notation layer
itself. The generalisation the review has been building toward is confirmed a fifth time from a
direction nobody had checked before: **any design surface that wants to gate a type-level position on
a property computed from a generic const value hits this wall, structurally, regardless of what that
value denotes.** This is not a numeral-specific fact; it is a fact about `adt_const_params` combined
with the forbidden-feature list, and any future carrier this design mints should expect to hit it
before building on the assumption that it will not.

### 4.3 The resolution is a second representation, connected by a trusted mapping, not one representation doing both jobs

State this precisely, because it is the load-bearing sentence of this file and it generalises past
numerals. **Legibility and structural sealing pull on the same axis in opposite directions, for the
reason this whole tower exists in the first place.** Values-as-types is what buys bound-level refusal
(a malformed value literally has no type to inhabit). Printing a value-as-types nest is what makes a
diagnostic illegible. The two properties are the same mechanism looked at from two sides, and no
single representation gets both for free. A refinement-types reader will recognise the shape: this is
the smart-constructor pattern. A value validated once, at a single trusted entry point, need not carry
its own re-checkable proof term everywhere it goes; the proof is discharged at construction and the
type downstream is free to be whatever is convenient to read.

The macro is exactly that entry point, and it already has everything it needs to be one: it sees the
literal digits the consumer wrote, at expansion time, concretely, not generically. `49:589-591`
already proposes the macro compute the internal encoding at expansion time and emit literal
constructors. `56_probes/probe_7_concrete_newtype_per_numeral_has_no_sealing_question.rs` tests the
missing half: a concrete, non-generic newtype, minted only by the macro's own emitted `impl`, with no
public constructor a consumer could reach independently:

```rust
pub struct Q37;
impl Numeral for Q37 { type Encoding = Enc<37>; const DISPLAY_VALUE: u64 = 37; }
```

```
error[E0308]: mismatched types
   |
58 |     needs_q37(x);
   |     --------- ^ expected `Container<Q37>`, found `Container<Q53>`
```

There is no sealing question left to ask, because there is no attacker position: a consumer cannot
independently spell a second, malformed `Q37`. Section 4.2's hazard does not apply to this shape at
all, for the same reason it does not apply to the tower's own constructors: nothing outside the
trusted emitter can mint an inhabitant.

**What this costs against the const-generic form.** No generic reuse across numerals; every distinct
value is its own type, minted at its own macro invocation site. That sounds like the bounded table
the design already refused (`49:1004-1007`), and it is not, for the same reason the digit-emitting
macro itself is not: nothing is stored, nothing has a ceiling, and the cost is paid once per distinct
numeral a consumer actually writes, at the point they write it, exactly the shape of the accepted
proposal, with one addition (a paired face, not only the raw constructors).

**The recommendation for the notation macro's vehicle**, stated so the next consolidation can take it
directly: the macro emits, per invocation, both the concrete constructor chain (already proposed) and
a concrete, non-generic newtype implementing a bridge trait to it, with no public constructor of its
own. Operations that want section 3.2's Shape 2 result (legibility surviving past declaration) define
a face-level sibling for that specific operation; operations that do not, decay to the raw encoding at
that point, honestly, which is the ceiling stated as a cost rather than discovered as a surprise.

*grounded on: `pin` throughout; the generalisation in 4.2 grounded on `49:59-72` (the spine rule) and
its four prior firings cited above; the smart-constructor framing is reasoned, not compiled, and is
the one place in this file the analogy is doing real work rather than decoration.*

## 5. The E0275 residual: tried, and it does not reach

Two separate residuals need distinguishing before either is answered, because the consolidation names
both and this section tests both. `49:839-842` ("the decoder-ring diagnostic") is about `E0308`, a
mismatched numeral, and asks whether a member with deeper diagnostic knowledge can do better than
file 47's non-fix in twenty minutes; sections 2 through 4 and section 7 answer that one. `49:868-871`
("the E0275-diagnostic residual") is the one this section is about: file 46 section 6.2's finding that
the composition wall fires with the identical "overflow evaluating the requirement" text on a
**concrete, rigid non-inhabitant** (a real type with no `Pos` impl at all) as on a fully abstract type
parameter, which the consolidation calls "worse" and "anonymous" because a consumer hitting the wall
with a genuinely wrong concrete type would reasonably expect a clear "not implemented" refusal rather
than the generic recursion-limit message. This section tests both shapes against the same instrument.

The a-priori read, before compiling anything: `#[diagnostic::on_unimplemented]` is documented as
firing when a trait bound is checked and **no candidate impl exists** (the E0277 shape). E0275,
"overflow evaluating the requirement", is a different failure: the solver did not finish deciding
whether a candidate exists, because confirming one candidate's own obligations recursed past the
depth limit. Those are different code paths inside the solver, and an attribute that customises the
first has no obvious reason to touch the second.

**Confirmed directly**, not merely predicted. `56_probes/tower_annotated.rs` is file 46's real,
ratified `Reduce` trait, copied unmodified except for one `#[diagnostic::on_unimplemented]` attribute
placed on its declaration:

```rust
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be reduced to lowest terms with an abstract operand",
    ...
)]
pub trait Reduce { ... }
```

`56_probes/probe_5b_on_unimplemented_on_the_real_reduce.rs` reproduces 48's own probe 1 (the
fold-signature refactor that spells interior safety as a reduced headroom ratio) against this
annotated copy:

```
error[E0275]: overflow evaluating the requirement `Pz<O<_>>: ExactDivOdd<_>`
   |
21 |     Ratio<Hd, Am1>: Reduce,
   |                     ^^^^^^
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute
   = note: required for `Pz<O<O<_>>>` to implement `ExactDivOdd<_>`
   = note: 126 redundant requirements hidden
   ...
```

**Byte-identical to the unannotated baseline**, down to the "126 redundant requirements hidden" count.

**The rigid-non-inhabitant case, `46:6.2`'s own specific residual, tested directly and settled the
same way.** `56_probes/probe_5c_on_unimplemented_on_the_rigid_non_inhabitant.rs` reproduces
`46_probes/probe_5`'s own attack (`LocalNat`, a real, concrete type implementing `Dbl` but never
`Pos`, entered at `Ratio<LocalNat, H>: Reduce`) against the annotated tower:

```
error[E0275]: overflow evaluating the requirement `Pz<O<_>>: ExactDivOdd<_>`
   |
28 | pub const ATTACK: u64 = <<Ratio<LocalNat, tower_annotated::nat::H> as Reduce>::N as Pos>::VAL;
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: required for `Pz<O<O<_>>>` to implement `ExactDivOdd<_>`
   = note: 126 redundant requirements hidden
```

Identical text, identical hidden-requirement count, to the abstract-parameter case above. The
attribute is not merely weak on either shape; it is inert on both, entirely. This is worth stating
without hedging: `#[diagnostic::on_unimplemented]` does not reach a solver-overflow diagnostic, full
stop, on this toolchain, for this class of error, whether the offending type is abstract or a
concrete non-inhabitant. Both named residuals (`49:839-842`'s "twenty minutes" challenge, so far as
it bears on this instrument, and `49:868-871`'s rigid-versus-abstract question) have an answer now:
no, not with this instrument, on either shape.

**What actually answers the residual is not a diagnostic fix, and the design already has it.** The
composition wall's own resolution, ratified since section 1.11 (`49:317-320`), is architectural:
"every trait in a chain that reaches a consumer-facing signature either pattern-matches on
constructor heads or has finite, non-recursive obligations; `Reduce`, and anything routed through it,
never appears in such a chain, only at concrete numerals." The right response to a bad E0275 at a
consumer-facing signature is not a better E0275; it is not reaching that signature at all, which the
design's own rule already forbids by construction (file 47's projection chain proves it is
achievable, file 48's collision proves what happens when the rule is violated). Improving the message
for the case the design already says should not exist is polishing a symptom of a shape the rest of
the review spent two files closing off.

**One instrument I considered and did not build, stated honestly as untried rather than as another
result.** A `#[deprecated]`-shaped lint on any direct consumer-facing bound naming `Reduce` (rather
than a diagnostic on the trait itself) could in principle warn at the point of use, before the wall
is ever reached, the way a clippy-style lint warns on a call rather than on a failure. I did not build
this; it is a workspace-lint-shaped mechanism, not an arvo-diagnostic-shaped one, and per section 6 it
belongs on the far side of the same accounting this file already applies to receipts and build-layer
contracts.

*grounded on: `pin`, `tree` for the `LocalNat` attack (an unmodified copy of `46_probes/probe_5`'s own
construction); the local reproduction attempt (which failed to reach the wall at all, recorded as
inconclusive in `56_probes/OUTCOMES.md` rather than discarded) and the two direct annotations of the
real trait (probes 5b and 5c), which are the results this section reports.*

## 6. What each instrument costs, priced rather than assumed

The design's compile-cost budget is measured and printed (`53b`, call 1). A diagnostic improvement
that adds trait machinery spends from it, and this file prices what it added.

`#[diagnostic::on_unimplemented]` on a trait declaration is inert on the success path by construction:
it is consulted only when producing a diagnostic, never during ordinary trait resolution, so it adds
no candidate for the solver to consider and no monomorphisation. This is not measured in this file
because measuring a zero on a construct whose documented behaviour is "read only on the error path"
would be measuring noise; every prior file that used the attribute (`47:probe_6`, `48:probe_3`, this
file's `probe_5b`) reports it costing nothing to declare, and nothing here contradicts that.

**The face's own seal check is priced, because it is genuinely new machinery and its cost was not
obvious in advance.** `56_probes/price/`, `--emit=metadata`, min-of-3, N = 0 against N = 60, the same
harness shape files 41, 42, 53 and 54 used:

| variant | marginal ms/item |
|---|---|
| bare `Tag<const V: u64>`, no seal check | **0.127** |
| const-struct `Spec` face, forced seal check via `.checked()`, reduced pairs | **0.178** |

Both are one to two orders of magnitude below the internal tower's own composition cost (2.1
ms/composition dyadic, 143 ms/composition worst-case 16-bit random rational, `53:129-137`), because
the face's own check is a single O(1) const-fn integer `gcd`, not the tower's O(depth) recursive
trait resolution. The face layer, whichever vehicle the macro ends up using, is not where this
design's compile-cost story lives; the tower underneath it, already priced by files 53 and 54,
remains the whole of that story.

*grounded on: `pin`, `host`, `flags` (`--emit=metadata`, no codegen), this dispatch's own sweep.*

## 7. The ceiling, stated by having hit it rather than by predicting it

Three things this fixture could not make legible, established by compiling the attempt, not by
declining to try.

**A raw type equality (E0308) never names anything but the fully-expanded type, for any alias,
regardless of instrument.** Confirmed twice (section 2), including the recurrence nobody had flagged.
The only lever that changes this is not a diagnostic attribute at all; it is restating the comparison
as a bound (E0277) instead of an equality, which is file 47's own finding (`47:probe_6`), reconfirmed
here in combination with a concrete face (`56_probes/probe_8_the_strongest_combination.rs`):

```
error[E0277]: expected accumulator width `Q37`, this one is `Q53`
   |
42 |     needs_q37(acc);
   |     --------- ^^^ declared with the wrong numeral face
   |
   = note: faces are minted only by the numeral-literal macro; if this is the right VALUE but
     the wrong SPELLING, re-emit it from the macro rather than editing the face by hand
help: the trait `SameFaceAs<Q37>` is not implemented for `Q53`
```

This is the strongest message this fixture produced anywhere, and it costs one bound (`N:
SameFaceAs<Wanted>` instead of an exact type parameter) and one attribute. Where the design's own
signatures can state a numeral requirement as a bound rather than as a type equality, this is
available for free; where they cannot (a fold's declared accumulator numeral, a division's declared
result type, anywhere the design genuinely needs exact-type identity rather than a satisfiable
relation), the decoder ring is the ceiling and no instrument this fixture found moves it.

**`#[diagnostic::on_unimplemented]` cannot reach a solver-overflow diagnostic.** Section 5, confirmed
directly, byte-identical before and after. The residual the consolidation carried as untried is now
tried, and the honest answer is that the instrument category does not apply to this error class at
all; the design's already-ratified architectural avoidance is the actual and only fix.

**A const-generic face cannot be structurally sealed without the forbidden feature.** Section 4.2,
compiled. The spine rule reaches a fifth position outside the numeral tower proper, at the notation
layer itself, and the resolution (section 4.3) is not a diagnostic fix either; it is a different
representation choice (a macro-minted concrete type with no public constructor) that sidesteps the
question rather than answering it within the const-generic shape.

Per the design's own downstream-contract discipline (`49:705-713`, and the receipt shape files 50 and
51 both extended it with): where this file cannot deliver something itself, it states what it needs
rather than reporting a limitation and stopping. The one item worth naming in that form: **a future
Rust diagnostic surface that can customise a solver-depth failure (an on-overflow analogue of
`on_unimplemented`) would close section 5's ceiling entirely, since the design's own architectural
avoidance already keeps the wall out of consumer-facing signatures; what remains unreachable is
purely the message a maintainer sees while developing the tower itself, not anything a consumer of
the finished design encounters.** That is a request to file upstream if the review wants one, not a
mechanism arvo can build.

## 8. What I would put in the spec, verbatim

Four sentences, in the form the next consolidation could take.

> The carrier-at-birth rule's seal is also the review's best diagnostic, and it costs nothing extra:
> rustc's own sealed-trait detection explains a private-supertrait refusal in plain English and lists
> every legal inhabitant, unprompted, with no `#[diagnostic::on_unimplemented]` anywhere in the
> tower.

> A raw type-equality mismatch (E0308) always prints the fully expanded type, for an alias or a
> macro-produced alias alike; this has now recurred unflagged once beyond its first discovery. Where a
> numeral requirement can be stated as a bound rather than an equality, `#[diagnostic::on_unimplemented]`
> reaches it and can name the mismatch in plain language; where it cannot, the decoder ring is a
> ceiling, not a defect to keep chasing.

> The notation macro emits a concrete, non-generic newtype per numeral alongside its constructor
> chain, minted only at its own expansion site with no public constructor. This keeps declaration-site
> errors legible with no sealing question to ask, because nothing outside the macro can mint a second
> inhabitant. Legibility surviving past the first operation is a property of that operation having a
> face-level sibling, not a property of the notation layer alone, and each operation that wants it
> pays that cost explicitly.

> `#[diagnostic::on_unimplemented]` does not reach a solver-overflow diagnostic (E0275); this is
> confirmed by direct annotation of the ratified `Reduce` trait, byte-identical before and after. The
> composition wall's residual diagnostic is closed by the design's own architectural avoidance, already
> ratified, not by a diagnostic-layer fix.

## 9. What I did not settle, stated as owed

**Whether a face-level sibling should exist for every operation in the design's current surface, or
only for the ones a consumer is likely to chain.** Section 3.2 shows the mechanism; it does not price
doubling the trait surface for `mul_full`, `mulnum`, `divnum`, `foldnum` and the fold combinators, and
that price is a real number somebody should run before the macro's vehicle locks.

**Whether the `#[deprecated]`-shaped lint on a direct `Reduce` bound (section 5's untried instrument)
is worth building.** I considered it, did not compile it, and it belongs to a different accounting
than this file's (a workspace lint, not an arvo diagnostic).

**Decimal's own face.** File 54 built the type-level machinery for radix ten; this file's fixture is
radix-two throughout. Whether a decimal `Spec`'s cohort-member choice (`54:334-348`) needs its own
face field, or is orthogonal to the diagnostic question entirely, is untested here.

**Whether `on_unimplemented`'s `{Self}` interpolation is safe against every carrier this design
ships**, or only against the two I tested (`Reduce`, a local `SameFaceAs`). I did not sweep the whole
carrier list; the mechanism is documented and stable, and I have no reason to expect it differs by
carrier, but I have not compiled the sweep.

## Provenance summary

Compiled, this dispatch, fresh, on the pinned nightly, from inside `56_probes/`: all eleven probe
files and the price sweep listed in `56_probes/OUTCOMES.md`, including the seal's own diagnostic
(section 1), the alias-versus-newtype comparison (section 3.1), the composition-survival comparison
(section 3.2), the multi-field face and its sealing hazard (section 4), the E0275 annotation on both
the abstract-parameter and the concrete rigid-non-inhabitant shapes (section 5), and the
combined-instrument result (section 7). Read and quoted rather than recalled:
`49_consolidation_four.md`, `50` through `55` in full, `53b_persona_checkpoint_twelve.md`,
`46_probes/vu_nat_sealed_adj.rs`, `46_probes/vu_bias_sealed_adj.rs`,
`47_probes/probe_1b_a_wrong_digit_is_silent.rs`, `probe_1c_the_diagnostic.rs`,
`probe_6_the_caller_contract_diagnostic.rs`, `48_probes/probe_1_the_wall_is_one_refactor_away.rs`,
`55_probes/OUTCOMES.md`. Reasoned, not compiled, marked at the point of use: section 4.3's
smart-constructor framing, section 6's zero-cost claim for the attribute itself (not separately
measured, argued from documented behaviour and every prior file's silence on it), and section 9 in
full.

I suggest; op decides.
