# 31. Settling the identity contract

**Member:** Hans-Kristian Arntzen. Basement lens: trace one specific guarantee through its whole
lifetime, from the spec sentence that states it to the compiled artefact that either honours it or
does not, and refuse to call it established until something has actually run. A property that holds
"by inspection" is a property nobody has checked yet.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed,
9 ignored (1 unit-test ignore, the catalogued divide gap tracked #5 at
`crates/arvo/tests/fixed_point_div.rs:111`; 8 doctest ignores, matching file 28's reconciliation of
file 27's undercount). `crates/arvo/tests/identity_laws.rs:1-21` is the honest full-matrix shape its
own module doc describes: it names the four wrong constants a sampled suite missed and states that
sampling was the failure. Nine compile-fail cases under `crates/arvo/tests/ui/` (`ls`'d directly: nine `.rs`/`.stderr`
pairs, eighteen files, matching file 30's own count) pin refusals a runtime assertion cannot state. I found nothing tautological in the surface
this file touches. Canon gate: the governing calls are the D-numbered ones in
`202607301200_topic.the-formalization-spec.md` and `202607291900_topic.the-number-systems-crate.md`,
subordinate to op's seventh checkpoint (`30b_op_checkpoint_seven.md`), which overturns D69, holds D39,
and changes what this dispatch is for: converge, not catalogue. Where this file argues past a call
that checkpoint left standing, it says so in place.

**What I read:** `26_consolidation_two.md` in full; `30b_op_checkpoint_seven.md` in full;
`27_carmack_what_a_number_is.md`, `28_leroy_what_identity_must_express.md`,
`29_wronski_the_quantisation_contract.md`, `30_pesce_the_identity_half_assembled.md` in full, with
every probe and every `OUTCOMES.md` under `27_probes/`, `28_probes/`, `29_probes/`, `30_probes/`.
`202607301200_topic.the-formalization-spec.md` in full, read directly rather than trusted from
citation, because this stretch has already caught one file's citation drift against another
(`30:51-57`, catching file 29's own line-number miscite at `29:47-48`). Directory listed once; no
other panel file fetched.

**What I compiled:** I did not take the four files' probe claims on their word. I recompiled
`30_probes/probe_2_ieee_overflow_falls_out_of_round_first.rs`,
`30_probes/probe_3_sign_domain_against_sign_indexing.rs`,
`30_probes/probe_4_dither_manufactures_refusals.rs`, and
`30_probes/probe_5_biased_multiplication_is_closed.rs` directly against the pinned nightly
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2021 --crate-type lib`), each a clean exit.
These are the four probes carrying the five reconciliations this dispatch was sent to settle; I did
not re-run files 27's, 28's or 29's own probes, on the reasoning that file 30 already re-ran and
superseded the load-bearing ones and this dispatch's job is to settle file 30, not re-audit its
ancestry. I wrote and compiled two new probes: `31_probes/probe_1_signed_overflow_is_asymmetric.rs`, closing a
gap probe 2 disclaims three times over without ever converting the disclaimer into a checked claim,
and `31_probes/probe_2_biased_multiplication_negative_control.rs`, the negative control for probe 5's
closure formula that `30_probes/OUTCOMES.md` describes but never commits as compiled source. Full
record below and in
`31_probes/OUTCOMES.md`. **Everything else in this file is reasoned**, and marked as such.

## 0. What op's checkpoint actually changes here

Read literally, not summarised. Two things.

First, D69 is overturned: identity is parameterised in mathematical coordinates, not encoding
coordinates, and file 27's and file 28's independent readings are why (`30b:9-16`). File 30's whole
§1.1 already assumes this; so does everything below. I am not relitigating it.

Second, and this is the part that changes what this file is for: "Members build on each other toward
a spec that gets stronger with each file... Where a predecessor's proposal survives your attack,
strengthen it and carry it forward as shape, rather than restating that it survived. Where it fails,
replace it, and the replacement is the deliverable rather than the failure" (`30b:42-53`). File 30 has
had one pass, its own author's. My job is to attack it hard enough that whatever survives is provably
settled, and to say what the design *is*, not to add a fifth commentary on top of four.

## 1. The five reconciliations, checked myself, not trusted

Each of these is a probe I recompiled personally against the pinned nightly. Recompiling is not a
formality: the whole discipline this review runs on is that a claim untested by a second party is not
established, and "the previous file's `OUTCOMES.md` says it worked" is exactly the kind of prose claim
about the design's own state that `26_consolidation_two.md:111-119` names as the standing failure
mode. All four came back clean, matching their stated outcome exactly.

### 1.1 The crossing contract is a section-retraction triple, not two round-trip theorems

File 28 stated the contract between the mathematical-coordinate side and the physical-encoding side as
two round-trip theorems: `decode ∘ encode = id` on data, `encode ∘ decode = id` on bit patterns
(`28:84-86`). File 30 found file 28 contradicts itself between its own §1 and §2: §2 names three
entrances to non-injective interpretation (signed zero, NaN payloads, decimal cohorts, `28:119-138`),
and every one of them falsifies the second theorem (`30:219-241`).

I recompiled `30_probes/probe_3_sign_domain_against_sign_indexing.rs`. It holds, with the exact witness
`encode(decode(0b1000)) != 0b1000` under sign-magnitude indexing over a symmetric domain (probe lines
244-254), and the weaker replacement (idempotence of `encode ∘ decode`, not identity) holds
exhaustively over the full sixteen-datum model for every indexing tested (probe lines 259-286).

**This is settled: the crossing contract is three statements, not two.**

1. `decode ∘ encode = id` on values, always.
2. `encode ∘ decode` is idempotent on data, always. This is canonicalisation, and it is where a
   canonical encoding, a preferred exponent, and NaN canonicalisation all live.
3. `encode ∘ decode = id` on data iff the encoding is injective, which is a derived boolean, not an
   assumption.

I have nothing to add to the mechanism. What I add is where it sits in the larger contract, in section
3 below, because a canonicalisation function that exists and is never invoked is exactly the kind of
declared-but-unenforced fact section 1.6 of the consolidation already has a name for.

### 1.2 Signedness is a value fact and a datum fact, and SC_SAT_SYM is a numeral

File 28 proposed `Sign` with three instances on identity: `Unsigned`, `TwosComplement`,
`SignMagnitude` (`28:186-192`). File 30 found this bundles a value fact (the representable range) with
a datum fact (how many bit patterns carry zero), which is exactly what file 27's coordinate inversion
had just finished separating out of identity (`30:242-254`), and split it: `SignDomain`
(`NonNegative`/`Symmetric`/`AsymmetricLow`) on identity, `SignIndexing`
(`Unsigned`/`TwosComplement`/`SignMagnitude`/`OnesComplement`) on encoding.

I recompiled the same probe. Both halves hold. The independence claim (probe lines 113-167): the
symmetric domain is served by all three signed indexings, and two's complement serves two different
domains, so naming either one does not name the other. And a genuine finding that changes something in
`Policy`: with the axes split, the identical `TowardNegative` clamp delivers `-8` under `AsymmetricLow`
and `-7` under `Symmetric` (probe lines 296-316). **SystemC's `SC_SAT_SYM` is not a saturation mode; it
is `TowardNegative` clamping applied to a symmetric numeral.** One fewer thing in `Policy`, one fewer
special case in the `conv-systemc` alias set.

**Settled.** File 28's conclusion survives; it was one axis short, not one axis wrong, exactly as file
30 states (`30:267-268`), and I have nothing to add to that verdict beyond having checked it myself.

### 1.3 Round first, classify second, reproduces IEEE 754-2019 §7.4 exactly

File 28's amendment: round on the unbounded-exponent extension of the grid, then classify the rounded
result against the range (`28:235-238`). File 27 had proposed `Specials` as an identity member and then
overreached, claiming that with infinity representable "past the top is unreachable" (`27:189-192`).
File 30 found the overreach and the correct account: infinity does not remove the over-range position,
it changes what the position's neighbour is, and the midpoint that decides ties-to-even overflow lives
on the unbounded grid the round-first amendment supplies, not between the largest finite and infinity,
where no midpoint exists (`30:383-390`).

I recompiled `30_probes/probe_2_ieee_overflow_falls_out_of_round_first.rs`. It checks, at a model float
(radix 2, precision 3, emax 2), exhaustive agreement over real 1 through 9 against three oracles
written independently from the standard's own text, not from the pipeline: roundTiesToEven (overflow
at `2^emax(2 - 2^-p)`, derived from the format rather than a literal, verified at `RN_BOUNDARY == 120`),
roundTowardZero, and roundTowardPositive. All three agree exactly, including the boundary case (probe
lines 217-243), and the mode coupling is real information, not a restated default (probe lines
194-209, `coupling_matters()` is itself an assertion, not a comment).

**Settled, with one gap this file closes rather than leaves disclaimed:** every version of this probe,
including file 30's own summary, states three times that it says nothing about the negative half of
the range (`30_probes/probe_2` lines 245-252; `30:412-413`; `30:645-646`). That disclaimer was never
converted into a checked claim, and the thing worth checking is not symmetry, it is the *absence* of
symmetry. `TowardPositive` and `TowardNegative` are absolute directions on the value line. Under
IEEE's roundTowardPositive attribute, positive overflow delivers `+infinity`; deep negative
"underflow" delivers the negative largest finite, **never** `-infinity`, because rounding toward
positive never selects the more negative of two candidates. roundTiesToEven and roundTowardZero, by
contrast, are odd-symmetric and do mirror.

I wrote and compiled `31_probes/probe_1_signed_overflow_is_asymmetric.rs`, extending probe 2's model to
signed exact values via a magnitude/sign split, with sign entering in exactly one place
(`round_unbounded_signed`, shared with ordinary in-range rounding) and nowhere in the resolution logic,
which dispatches on the spec's own separate `OverRange`/`UnderRange` fields
(`202607301200_topic.the-formalization-spec.md:135-136`) using the same absolute `Direction` markers
probe 2 already verified. It reproduces the asymmetry exactly: `oracle_rp_signed(-1600) ==
-LARGEST_FINITE` and `!= NEG_INF` (lines 274-275 of the probe), the design's own pipeline reproduces it
with the identical assertion (lines 284-288), and a negative control that deliberately reuses the
`over` marker at the `under` position (the bug this probe exists to catch: mirroring the wrong end's
resolution) disagrees with the true oracle, confirming the assertion is live (line 333, the probe's
last line).
This closes the gap for the three tested attributes with no new mechanism: the same `OverRange` /
`UnderRange` split the spec already declares, and the same absolute-direction reading of `Direction`
the design already uses, are sufficient. Nothing needed inventing.

What this still does not show, honestly: underflow and subnormals, NaN propagation, and any attribute
beyond the three tested (roundTowardNegative and roundTiesToAway remain unchecked, though by the same
absolute-direction argument I expect them to fall out the same way; I have not compiled that
expectation and it is not a claim).

### 1.4 Dither and `Refuse` do not compose, and confinement is a real repair with a real cost

File 29 proposed `quantize_dithered(exact, noise) = quantize(exact + noise)`, stating of the range ends
only that the ordinary out-of-range resolution "takes over exactly as it would for any other value
that landed there" (`29:96-97`). File 30 found this false for exactly one resolution: `Refuse`, which
is `Precise`'s own out-of-range rule (`202607301200:250-257`), so a caller's decision to decorrelate
can silently turn a total computation into a fallible one, on inputs the numeral represents exactly
(`30:326-338`).

I recompiled `30_probes/probe_4_dither_manufactures_refusals.rs`. The finding is not a corner case: the
top of a sixteen-step range is exactly representable and total under the undithered path
(`quantize_precise(TOP) == TOP`), and one quantum of positive dither on that same input refuses
(`quantize_dithered(TOP, Q) == REFUSED`, lines 65-70). The candidate repair, confining the perturbed
value to the numeral's range before quantising, restores totality exhaustively at one and two quanta of
amplitude (lines 136-154) while leaving the interior mechanism, the decorrelation the whole point of
dithering, intact where it matters (lines 156-174 verify the confined path still delivers different
errors on the same input for two different noise draws).

**Settled as a mechanism, open as a design choice.** File 30 states this correctly and I have nothing
to add mechanically: the choice is between confining (which costs the dither its uniformity within one
amplitude of either end, a real and known cost, not a free repair) and offering `quantize_dithered`
only where the out-of-range resolution is total in the first place (`30:623-627`). That choice is about
what a `Precise` consumer expects, which the consolidation already has open under a different name (what
`Precise` is for, `26:608-617`), and I am not going to manufacture a preference where the design
genuinely has not decided. What I will say: whichever way this resolves, it must resolve as a stated
property of the *composition*, not as an implicit "and if you dither a `Precise` value near its edge,
good luck." A silently-added fallibility mode that only fires within one dither amplitude of either end
is exactly the shape of bug nobody writes a test for until a user reports one.

The probe also carries a correction to file 29's stated mechanism, and I note it because it is the kind
of factor-of-two error that survives review by looking plausible: two positions sharing a residue class
receive the identical error under *every* `Direction`, file 29 claimed (`29:69-72`). Not for `ToEven`
and `ToOdd`, where the tie breaks on the quotient's parity, so the period is `2Q`, not `Q`
(`30_probes/probe_4` lines 156-174). `ToEven` is what `Warm`, `Cold` and `Precise` all use
(`202607301200:250-257`). The conclusion survives (a memoryless rule cannot decorrelate regardless of
its period) but a reader sizing a dither amplitude off file 29's stated period would size it wrong by a
factor of two, on the three presets that matter. Recorded here so a fourth file does not have to find it
again.

### 1.5 Biased multiplication is closed, and the formula generalises the shipped rule

The consolidation records adjustment and bias as not closed under multiplication, with two candidate
fixes and no formula for either (`26_consolidation_two.md:326-331`). File 28 proposed the rational-pair
adjustment, addressing the adjustment half (`28:319-328`). File 30 supplies the bias half: for
`v1 = A1*k1 + B1` and `v2 = A2*k2 + B2`, every cross term of `v1*v2` lies in the lattice generated by
`A1*A2`, `A1*B2` and `A2*B1`, so the product numeral is `adjustment = gcd(A1*A2, A1*B2, A2*B1)`,
`bias = B1*B2` (`30:462-489`).

I recompiled `30_probes/probe_5_biased_multiplication_is_closed.rs`. Checked exhaustively over the full
window product for six operand pairs, including a MATLAB-shaped slope-and-bias pair at scale 1000
(lines 91-124). With both biases zero the formula collapses to `A1*A2` and bias zero exactly (lines
117-119), which is the load-bearing property: **the multiplicative half's verified width adder from
section 1.5 of the consolidation is the special case of this formula, not a second rule that has to be
kept in agreement with it.**

The probe file as committed carries only the positive assertions; `30_probes/OUTCOMES.md` describes a
negative control (dropping the cross terms, using the naive `A1*A2`) without compiling it inline. I did
not take that description on trust either, and wrote the negative control myself rather than working it
out by hand: for `(A1,B1,A2,B2) = (4,2,6,4)`, the naive adjustment is `A1*A2 = 24` and the bias is
`B1*B2 = 8`, and the exhaustive check against that wrong adjustment fails at its very first pair,
`k1=0, k2=1`, which is `v1 = 2, v2 = 10`, product `20`: `(20 - 8) mod 24 = 12 != 0`. Compiled and
confirmed (`const _: () = assert!(!check(3,3))` holds against the pinned nightly). The cross terms are
load-bearing, not an artefact of the numbers file 30 happened to pick.

**Settled**, with one honest limit file 30 already states and I confirm rather than extend: `L` is a
lattice *containing* the product set, not necessarily the finest one, which is the safe direction for
closure and the wrong direction for a tight width bound. The width computation itself, and whether
MATLAB's `fi` actually multiplies two biased objects, both remain unverified against anything outside
this probe (`30:491-494`), and I have not done either.

## 2. A dropped thread: `Deterministic` (D70), and the fix was already built and not connected

None of files 27, 29 or 30 mentions `Deterministic` (`grep -n "Deterministic" 27_*.md 29_*.md 30_*.md`
returns nothing). File 28 raised a real, load-bearing defect against a ratified spec sentence and it
was never carried forward.

`202607301200_topic.the-formalization-spec.md:234-236` states `Deterministic` as "a blanket
implementation keyed on the composition." File 28 found this false the moment a composition's
operations lower to hardware float instructions: NaN payload propagation is architecture-dependent,
which is a settled fact of shipping hardware (CompCert had to parameterise its own float semantics by a
per-target NaN payload policy to keep its correctness theorem true), so the *value* delivered is
portable and the *datum* is not (`28:152-161`). This is not a hypothetical; it is exactly the same
non-injective-interpretation structure file 28's own §2 already names for signed zero and cohorts,
applied to the third case.

What I add: file 28 already named the shape of its own fix in prose, and the mechanism to hang it on
was still two files away. File 28's own fork reads "either `Deterministic` is two facts (value-
deterministic portably, datum-deterministic only per target or after paying for NaN canonicalisation at
operation boundaries)" (`28:158-159`), with no named mechanism for canonicalisation anywhere in the
file (`grep -in canonical 28_leroy_what_identity_must_express.md` returns only that one line). File
30's `Encoding` contract, built two files later for an unrelated reason (the crossing-contract work of
section 1.1 above), carries exactly that mechanism: `type Canonical: Canonicalisation`, explicitly
scoped to "which datum is delivered where several carry one value. IEEE's canonical encoding; decimal's
preferred exponent; **NaN canonicalisation**" (`30:130-133`). The fix file 28 asked for was built,
under a different name, for a different reason, and nobody looked back.

**The fix, stated so it does not drop a second time:** `Deterministic` as ratified is a value-only
claim, and it stands unchanged; it was never wrong about values. What is missing is a companion fact,
keyed the same way the consolidation already keys everything derived (a composition-scoped `const fn`,
section 1.4 of `26_consolidation_two.md`):

```rust
/// A composition is datum-deterministic when every value-producing step
/// whose result could deliver a non-canonical datum is followed by
/// `Encoding::Canonical` before the datum is observed. A composition whose
/// numeral carries `Specials = None` cannot reach a non-canonical datum at
/// all, and is datum-deterministic for free: this is every fixed-point
/// composition in the design today.
const fn datum_deterministic<Comp>() -> bool { /* derived, not asserted */ }
```

This is not new machinery. It is the existing `Canonical` member, read against the existing
`Deterministic` claim, with the one sentence written down that neither file said: *`Deterministic` is
about values; add `DatumDeterministic`, derived from whether canonicalisation is paid at every step
that could need it, and the entire non-float half of the design gets it for free because `Specials`
being absent removes the divergence channel entirely.* Reasoned, not compiled: I have not written the
derivation as a real `const fn` against arvo's trait shapes, and I flag this as the next concrete thing
to check, in the same register file 28 used for its own unbuilt proposals (`28:395-399`).

## 3. A soft point in the nesting argument, worth naming so it is not cited as harder evidence than it is

File 30's strongest argument for nesting `Adjustment`/`Bias`/`Underflow`/`Specials` under `ExponentForm`
rather than a flat axis list with a `WellFormed` predicate is a forward-provision claim: block floating
point (a shared exponent per fixed-size block, the OCP microscaling family, current machine-learning
hardware) becomes an additive third branch under nesting, while a flat list needs every compatibility
row revisited (`30:103-112`). File 30 calls this "the argument I find most persuasive... about what the
shape costs the fourth person to touch it" (`30:111-112`).

I do not think this example survives its own scrutiny, and the reason is worth stating precisely
because it is a trap the same shape as the ones this review keeps finding: a plausible-sounding
motivating example that nobody checked actually fits the thing it is motivating.

Section 1's own opening states what a `Number<N, S>` value *is*: "an integer k, drawn from a finite
integer interval, together with a type-level rule that injects k into a set of rationals"
(`27:44-49`), unchanged by anything in files 28 through 30. A block-floating-point mantissa, taken in
isolation, is not this. Its value depends on a shared exponent stored *outside* the value it
parameterises, associated with a block of many such mantissas. There is no self-contained injection
rule for a single BFP element; decoding one requires external context (the block's shared exponent)
that no single `Numeral` instance carries. This is not a branch of `ExponentForm` under either shape,
nested or flat: it is a different kind of thing, a **composite numeral** built from a pair (a
shared-exponent numeral, its own self-contained thing; a per-element significand numeral, likewise)
with a derived per-element value, closer in shape to file 27's proposal that a Gaussian-dyadic complex
inhabitant derives its system from its components (`27:296-308`) than to anything `ExponentForm`
expresses today.

**This does not touch the nesting decision's standing.** The Underflow argument nesting was built on
("a constant exponent has no bottom to fall off and would have to carry a value meaning the axis does
not apply," `202607301200:98-99`, applied identically to `Adjustment`, `Bias` and `Specials` by file
27's `27:126-167` and adopted whole by file 30) is sound on its own and needs no forward-looking
example to justify it; I have nothing to overturn there and file 30's shape stands. What should not
survive into the next consolidation is BFP cited as *evidence* for it, because it is not evidence for
either shape being cheaper to extend: it names a real, separate, legitimately future design question
(composite numerals), and citing it under the nesting decision risks a later reader thinking nesting
somehow *solves* block floating point, which it does not, any more than a flat list would. Scope it the
way Leroy scoped decimal cohorts: a requirement the identity contract should not build itself shut
against, not a design built now.

## 4. The identity contract, assembled, hardened, and settled where the five items above settle it

I am restating file 30's §1 rather than diffing against it, per the checkpoint's instruction to state
what the design *is*. Everything here either matches file 30 exactly (marked unchanged), or is the
specific hardening sections 1 through 3 above establish.

### 4.1 Identity: which numbers exist

```rust
pub const trait Numeral {
    type Radix:     Radix;        // 2 and 10 instantiated; any r expressible
    type Precision: Precision;    // significand digit count, primitive (D69 overturned, 30b)
    type Exponent:  ExponentForm; // where the exponent lives; nests the rest
    type Domain:    SignDomain;   // NonNegative | Symmetric | AsymmetricLow, a VALUE fact (1.2)
}

pub struct Implicit<const E: Exponent, A: Adjustment, B: Bias>;
pub struct Ranged<const EMIN: Exponent, const EMAX: Exponent, U: Underflow, S: Specials>;
```

Unchanged from `30:74-90`. `SignDomain` (not the three-instance `Sign` file 28 proposed) is the settled
identity member per section 1.2; the corresponding encoding-side `SignIndexing` sits in `Encoding`
below. The nesting itself stands (section 3 above); block floating point is not evidence for it and is
not designed here.

### 4.2 Encoding: nested inside `Lowering`, and it changes no value

```rust
pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;
    type Widening:    Widening;
    type Layout:      StorageLayout;
}

pub const trait Encoding {
    type SignIndexing: SignIndexing; // Unsigned | TwosComplement | SignMagnitude | OnesComplement
    type Fields:       FieldLayout;  // field widths, hidden bit, encoding bias, reserved codes
    type Canonical:    Canonicalisation; // signed zero, preferred cohort, NaN canonicalisation
}
```

Unchanged from `30:117-134`, with the charter restated exactly as file 30 restated it: **`Lowering`
changes no value. `Encoding`, nested inside it, may change which datum carries a value. Every operation
whose result depends on that is declared a datum-level operation, and no law may read one** (`30:141-149`).
Section 2 above is this charter's first real payoff beyond signed zero and specials: `Canonical` is not
only where NaN canonicalisation lives, it is the mechanism that makes `DatumDeterministic` a statable,
derivable fact rather than a hole in D70.

### 4.3 The crossing contract

Settled per section 1.1: a section-retraction triple, not two round-trip theorems.

1. `decode ∘ encode = id` on values, always.
2. `encode ∘ decode` idempotent on data, always (canonicalisation).
3. `encode ∘ decode = id` on data iff the encoding is injective (derived, not assumed).

### 4.4 What the quantiser sees, and what it may deliver

Round on the unbounded-exponent extension of the grid by the direction triple, then classify the
rounded result against the range and resolve. Settled per section 1.3, including for both ends now:
the resolution logic reads the spec's own separate `OverRange`/`UnderRange` fields
(`202607301200:135-136`) against a value set that includes `Specials`, with sign entering only in the
rounding step and nowhere in the resolution dispatch, and this composition reproduces IEEE 754-2019
§7.4 exactly for roundTiesToEven, roundTowardZero and roundTowardPositive, on both signs, with no new
axis and no new `Resolution`.

`quantize_dithered(exact, noise) = quantize(exact + noise)` is a real, free, zero-state entry point
(file 29, unchanged), and its interaction with `Refuse` is settled as a mechanism (confinement restores
totality) and open as a design choice (confine, or gate the entry point on totality) per section 1.4.

### 4.5 What membership can say

Unchanged from file 30's `30:190-213`, and outside this dispatch's assigned five items: membership
licenses the exact operation family and nothing else (D39 held, not overturned, per `30b:18-28`).
`ExactWindow` gates on `Specials = None` as the honest first ship; I have not built or checked the
window-closure-under-specials case and it stays exactly as open as file 30 left it.

### 4.6 Biased multiplication

Settled per section 1.5: `adjustment = gcd(A1*A2, A1*B2, A2*B1)`, `bias = B1*B2`, generalising the
shipped exact-product rule (`A1*A2` falls out with both biases zero) rather than replacing it.

### 4.7 `Deterministic`, corrected

New in this file, section 2: `Deterministic` (D70) is a value-only claim and stands unchanged.
`DatumDeterministic`, a companion fact derived from whether `Encoding::Canonical` is paid at every step
that could deliver a non-canonical datum, is what the design needs to state honestly for
Specials-carrying compositions. Every composition with `Specials = None`, which is every fixed-point
composition shipped today, is datum-deterministic for free.

## 5. What remains open, stated plainly rather than left implicit

Everything file 30 already listed as open in its §7 and its "what I did not do" stands exactly as open
as it was; I did not touch the preset-divergence question (op's checkpoint explicitly reserves it for a
later member, `30b:30-37`), the `Like` mechanism, the shaper-as-scan resolution, or the radix-changes-
`ToEven` note. What this file adds to the open list:

1. **`DatumDeterministic` is stated, not built.** The derivation needs writing as a real `const fn`
   against arvo's actual trait shapes; I have reasoned it, not compiled it.
2. **Block floating point is a composite-numeral question, not an `ExponentForm` branch.** Scoped as a
   requirement, per section 3, not designed.
3. **roundTowardNegative and roundTiesToAway are untested for the signed case.** I expect the
   absolute-direction argument to cover them for free; I have not compiled that expectation.
4. **`30_probes/probe_5`'s own negative control, as narrated in its `OUTCOMES.md`, is still not
   committed inline in that file.** I wrote and compiled it myself instead
   (`31_probes/probe_2_biased_multiplication_negative_control.rs`, section 1.5), which is now the
   committed record; `30_probes/probe_5` itself still asserts only the positive case, and a future
   editing pass could fold the negative control back into it.
5. **The dither-versus-`Refuse` design choice (confine, or gate on totality) is unresolved**, and I
   deliberately did not manufacture a preference where the design has not decided one.

Five items is fewer than four files opened between them, and every one of the five is a stated,
checkable claim rather than a loose end. That is what "leave the design more settled than you found
it" means concretely, and it is the bar this file was sent to clear.

## 6. Standing

Nothing here overturns a D-numbered call or op's seventh checkpoint. Section 1's five items are
confirmations of file 30's reconciliations, independently recompiled rather than trusted, which is what
the checkpoint asked for: strengthen and carry forward, not restate that something survived. Section 2
is a correction to a gap between a ratified spec sentence (D70) and three panel files' silence, closed
using machinery file 30 already built. Section 3 is a narrowing of one argument's evidentiary weight,
not a reversal of its conclusion. Section 4 is the assembled identity contract in the form the next
consolidation could take close to verbatim, which is what op's checkpoint asked every member from here
to produce.
