# 80. The verification bundle: seven owed checks, performed, and what each one actually establishes

Xavier Leroy, file 80. I wrote file 10 (what is actually certified), file 28 (what identity
must express), file 38 (what the design establishes), and file 45 (what each claim rests
on). File 10's transfer claim was later found to have promoted a necessary condition to a
sufficient one; I carry that correction rather than defend the original, and nothing below
leans on the compressed form.

**What I read.** `78_consolidation_eight.md` in full, the standing base; `79_dolan_what_
capacity_is.md` in full, the only deliverable since; `79b_op_the_verification_mandate.md`
in full (op's, recorded not dispatched; nothing below designs against it). Behind the
consolidation, with standing licence since every ledger item is a check of a derivation it
compresses: file 64 in full (the prior owed-second-reads bundle, three of whose items
recur here); file 67 sections 3, 7.5 and its closing statement-0 flag; the crossing
sections of `68_consolidation_seven.md` (1.4 and its `Crosses` block at 68:243-275, the
foldnum characterisation at 68:774, the nine-bit correction at 68:506-521); file 66 at
385-410 and 497-498; file 71 sections 1 to 2; file 39 at 90-104 and 340-360; the topic
file `mock/design_rounds/202607300800/202607291900_topic.the-number-systems-crate.md` at
20-90, read fresh; `62_probes/primary_sources.md` in full; `66_probes/model.rs` in full;
`63_consolidation_six.md` at 296-343 and 757. One `ls` of the panel directory, current
through `79b`. Shipped tree touched only through the standing canon-gate greps; no claim
below reads it for meaning.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit
1, empty. Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary
from every `test result:` line: **661 passed, 0 failed, 9 ignored**, matching the
consolidation exactly. I touch no shipped surface, so no test bodies fall inside my
touched perimeter; the one disqualifying test already on record
(`arvo-tensor/tests/capacity.rs:14-18`, tautological, flagged for deletion at 78:874-876)
stands exactly as the consolidation carries it. One tree side-effect to note: the suite
run regenerates `mock/Cargo.lock` to include file 75's committed `bench-bitpack-*`
manifests; the lock was not committed alongside them, and the modification in my working
tree is that pre-existing gap surfacing, not an edit of mine. Toolchain
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside
the tree; the identical command outside the tree resolves to stable `1.94.0`. Compiled
claims trace to `80_probes/` (four probes plus one compile-fail companion, outcomes and
timings in `80_probes/OUTCOMES.md`); every runtime number below is from a `-O` build on
this host; every compile-time number is `--emit=metadata` wall clock on this host, inside
the tree.

**The deletion test, applied.** No row below is justified from shipped source. The two
`tree-fact` citations that appear (the `Capacity` trait's shipped shape at file 79's own
citation, and the `repr(transparent)` attribute at 68:568-587 via file 73) are carried
from the files that made them, as facts of existence, and every design conclusion here
survives their deletion.

---

## 1. The crossing repair's second read: the mechanism holds; two compressions in the consolidated text do not

The record: file 66 found that none of the crossing contract's three statements says a
decoded datum lands in the value set, and that the composition is ill-typed without a
precondition; file 67 proved the repair is derived rather than chosen (the only
alternative, widening the target through the quantiser, does not exist: the quantiser
refuses on every escaping datum of every leaking cell, `67_probes/probe_3`, exhaustive);
and 67 proposed the home, `pub unsafe trait Crosses<N: Numeral>: Lowering`, adopted
presumptively at a persona checkpoint (68:250-268). Statement P joined it at file 73.
This is the independent read the adoption has owed since.

**My reading, formed from the record before re-reading 67's own advocacy.** The *kind* of
the obligation is forced, not chosen, by two already-ratified rules composed: statement 0
quantifies over the datum set (a `Lowering` fact) and the value set (a `Numeral` fact), so
by the layer-keying rule it keys on the pair; and a law may not read `Lowering`
(68:263-265 restating 63:179-181), so it cannot be a law. A pair-keyed claim in this
design's Rust is a trait on the `Lowering` parameterised by the `Numeral`, and the
supertrait `: Lowering` is what confines the obligation to things that can cross at all.
The alternatives eliminate cleanly: a use-time where-clause or runtime check moves a
declaration-time fact to use time, which 68:273-275 already refuses on binding-time
grounds and I re-derive the same way; refusing hand-laid layouts outright crosses the
toolbox line; and well-formedness-by-construction covers only tower-generated encodings,
which is exactly the case that needs no help. **The mechanism is confirmed. This is the
second read; with file 67's as the first, the presumptive marker can drop on op's word.**

Two compressions in the consolidated text need correcting before this becomes spec text,
and one perimeter sentence is owed. All three are precisions, not reversals.

**First, "the impl is blanket and safe (D16)" (68:271, carried into 78's section 1.4 and
the 1.23 trait table) is unspellable as written.** Rust admits no safe impl of an unsafe
trait. Compiled both ways: `80_probes/probe_4b` writes the literal sentence (`impl
Crosses<SomeNumeral> for GeneratedLowering`) and is refused, `error[E0200]: the trait
Crosses<SomeNumeral> requires an unsafe impl declaration`; `80_probes/probe_4` writes
what the D16 split has to mean here, one blanket `unsafe impl` inside the trusted crate
whose obligation is discharged by construction (the `Send`/`Sync` shape), beside a
per-declaration consumer `unsafe impl` for a hand-laid layout, and compiles clean. What
differs between the derived and asserted routes is who carries the proof, never the
spelling. The compile-fail file stays in the probes so a later softening of the trait's
marking surfaces as that file beginning to compile.

**Second, statement 0's quantifier domain is unstated, and it is the whole perimeter.**
"For every datum d of this encoding" leaves open what the datum set is: the full pattern
set of `Encoding::Fields`' width, or a declared subset. The pair (statement 0 over the
fields' width, statement P over the padding outside it) covers the full `StoredWidth`
carrier exactly when the fields' own domain is full; file 73 already flagged, without
resolving, whether `Encoding::Fields` ever declares a non-full domain (78:534-535). If it
ever does, statement 0 says nothing about the out-of-domain patterns, and
`repr(transparent)` makes every pattern reachable regardless of any shipped API, so the
guarantee would be quantified over less than what can be observed. The spec sentence I
suggest: *statement 0 quantifies over every bit pattern of `Encoding::Fields`' width;
an encoding whose decode is partial on that set does not satisfy `Crosses`, and
partiality is expressed by shrinking the fields, not by a domain side-condition.* That
closes the perimeter by construction and makes the file-73 flag moot rather than open.

**Third, the trusted-base sentence.** Every consumer-site `unsafe impl Crosses` is an
entry in the trusted base: an assertion the type system consumes and cannot check. That
is the correct design (it is the same honest boundary D16 draws everywhere), but the spec
should say it in those words, so the list of hand-laid `Crosses` impls is understood as
the list of trusted format declarations, auditable as a list.

*Grounded on: settled shapes (66:385-410, 67 section 3, 68:243-275, 78 section 1.4/1.22);
compiled (`80_probes/probe_4`, `probe_4b`); reasoned (the elimination, the quantifier
sentence, the TCB sentence).*

## 2. Statement 0 against `quantize` and `roundToIntegralExact`: performed, compiled, and the standing guess is half right

Flagged forward three times (66:497-498, 67:693-700, 78:943), performed by nobody. Done
now, exhaustively, at a decimal model with real cohorts: r = 10, p = 3, quantum
exponents -2..=1, |D| = 4000, exact i128 arithmetic, half-even everywhere
(`80_probes/probe_2`, 0.7 s).

**The structural answer first, because it dissolves most of the worry.** Statement 0 is a
per-encoding invariant quantified over all of D. It is not an operation property, so no
operation can violate it; what an operation can do is *leave* D, producing a result that
is not a datum at all, at which point statement 0 has nothing to say about it. So the
honest form of the owed check is closure: does each operation, given data of a `Crosses`
encoding, produce only data of that encoding (or refuse)?

**`quantize` is closed only because of its refusal branch, and the branch is dense.**
Over all 16,000,000 operand pairs: every result is a refusal or a datum of D. The
refusals number 5,679,000, 35.5% of pairs, and they are exactly, count-for-count, the
pairs where a naive non-refusing quantize would emit a mantissa of p+1 digits, a
non-datum. The standard's own shape (invalid, NaN) is arvo's refusal-or-grade here; the
point for the spec is that `quantize`'s totality story is the refusal branch, not the
encoding, and a `NoSpecials` numeral offering `quantize` must route that branch through
the same `Refuse`/grade machinery as every other range event.

**File 67's guess ("the dependence lives wholly inside D") holds exactly for
`roundToIntegralExact` and fails for `quantize`.** Compiled, both directions.
`roundToIntegralExact` is total over D with zero refusals (the mantissa-fit argument the
probe confirms exhaustively: a value below r^p rounded to an integer stays below r^p),
and it is fibre-preserving: over every value-equal operand pair, result values are equal;
only the result's cohort member differs (witness: (10, 0) and (1, 1), same value, results
(10, 0) and (1, 1)). That is a dependence wholly inside D, exactly as guessed. `quantize`
is different in kind: its result *value* reads the operand's *datum*. Witness: x = 1.23
against y = 1-at-exponent-0 gives 1; against 10-at-exponent-minus-1, the same value,
gives 1.2. Density: 2,889 of the 4,000 operand x's are affected by at least one cohort
pair. This is not a defect; it is the standard's own definition, and IEEE 754's clause
5.2 says it in one sentence the review already holds verbatim: "Except for the quantize
operation, the value of a floating-point result (and hence its cohort) is determined by
the operation and the operands' values" (`62_probes/primary_sources.md`). The design
consequence, stated for the spec: **`roundToIntegralExact` decomposes into a value-keyed
law (round to integer) plus a datum-keyed exponent selection, so its value half is
law-eligible; `quantize` does not decompose, is pair-keyed as a whole, and can never be
a law**, by the same layer-keying test that forced `Crosses` itself. The carve-out the
standard writes in prose lands in arvo as a keying fact, which is where this design
wants it.

Both operations preserve statement 0; the item closes. What it leaves behind is one
sentence for the operation catalogue (the keying split above) and one for `quantize`'s
refusal density, which at this model is large enough that a consumer-facing `quantize`
without a graded refusal story would be a trap.

*Grounded on: compiled (`80_probes/probe_2`, exhaustive at the stated model); physical
(IEEE 754-2019 clause 5.2 via `62_probes/primary_sources.md`, position-cited); reasoned
(the closure framing, the keying split).*

## 3. The exact fold width, built at the type level and priced: buildable, gate-free, and effectively free

File 64 stated the exact closed form, `bitlen(A * (2^P - 1))`, and explicitly did not
build it (64:98-103); the consolidation carries `foldnum(W, A) = W + ceil(log2 A)` as
sufficient always, tight only for power-of-two arities and wide precisions, loose by at
most one bit in a characterised band that includes A = 257 at p = 8 (68:774, 78:944).
The design's standard is optimal, so the exact form owed a build and a price.

**Built** (`80_probes/probe_1`, zero feature gates, `#![no_std]`): `Succ`, a nine-case
binary `AddP`, a shift-and-add `MulP`, file 79's predecessor rebuilt, `AllOnes` (2^P - 1
by value recursion through the predecessor), `BitLen` (structural depth), assembled as
`FoldExact<A>` with the result consumed in type position (`Acc<W>`), all ordinary
structural recursion over the sealed grammar, the same construction family as `VAL`,
`Cmp`, `Gcd` and `Dec`. **Verified**: 114 cells (six precisions across nineteen arities,
covering every behaviour class file 64 characterised), each asserted at compile time
equal to an independent u128 ground truth; the six named loose cells asserted loose by
exactly one bit; the power-of-two cells and the (p=2, A=3) tight-non-power
counterexample asserted exactly tight. A negative control (asserting 17 at the (8, 257)
cell) fails the build with E0080, so the assertions are live and the exact width there
is genuinely 16 against `foldnum`'s 17. **Priced**: 0.145 s wall for the whole file
against a 0.036 s empty baseline; roughly 0.11 s for the machinery plus 131 checked
cells, under a millisecond per cell. Per the consolidation's own staging rule
(78:764-766), the multiply inside `FoldExact` is superlinear in bits, but at fold-shaped
magnitudes (a 13-bit arity against a 16-bit mask is a 29-bit product) the absolute cost
is unmeasurable against build noise, so no staging is needed; the rule's threshold is
not reached. One honest bound: `AllOnes` recurses on the *value* of P, so a pathological
precision in the thousands would meet the default recursion limit; real precisions are
two orders of magnitude below it.

**The suggestion, and it is a suggestion.** Adopt `foldexact` as the fold's width, with
`foldnum` retired to prose as the human-readable bound it is. The design's exact-width
identity (a consumer never pays for a bit not asked for) is the whole reason arvo
exists; a formula that wastes a bit at A = 257, an ordinary graph size, with the exact
form now compiled, gate-free, and priced at noise level, has no remaining reason to
stand in the spec. The one residual file 64 named stays open and is not mine: `foldnum`
(either form) compiled against the real four-member `Numeral` contract with `Exponent`
held fixed (78:943-944), which is about the quantum sentence, not the width.

*Grounded on: settled shapes (64 section 1, 68:774); compiled (`80_probes/probe_1`, the
negative control, the timings); reasoned (the adoption suggestion, the staging note).*

## 4. The membership second read: the verdict stands, its mathematics needs one correction, and a cleaner uniqueness theorem is available

The constraint first, honestly. The convention wants the second reader's view formed
before reading the first reader's conclusion. Full blindness was not available to me:
the consolidation, which is required reading, already carries file 64's verdict in one
line (78 section 1.6). What I did instead: re-derived the whole question from the
primary texts (file 39 at 340-360, the topic file's D38 and D39 at
`202607291900:41-84`, both read fresh), checked every mathematical claim independently,
and only then reconciled against file 64's section 3. I have no stake: I did not
propose the candidate (file 39, Knuth), did not write the first read (file 64,
Chlipala), and none of my four files touches membership.

**Where I agree, independently re-derived.** The inhabits predicate is the right
reading and is decidable from the identity axes; the "finest inhabited system"
mechanism is sound for every numeral arvo has built or designed; and the stated
uniqueness justification, "exists and is unique because the tower is a chain"
(39:351-352), is false against the ratified ten-member vocabulary, because the
vocabulary is not a chain. Two of the three branch arguments hold exactly as file 64
gave them: no ordered field contains ℂ (squares are non-negative in an ordered field),
and no ℚ_p is orderable (−1 is a sum of squares in ℚ_p), so the Cayley-Dickson line
past ℝ and every p-adic branch are genuinely incomparable to the ordered-field line and
to each other. The verdict, mechanism sound and justification overclaiming, is
confirmed. Two reads are now done; the hold stays op's.

**Where file 64's own mathematics is wrong, and the branch count changes.** "Surreal
(No) and Hyperreal (*ℝ) are both ordered-field extensions of ℝ, and neither contains
the other" (64:240-242) is false in the only sense D39's structural test can mean.
Conway's surreals are the universal ordered field: every ordered field whose universe
is a set embeds into No, and a hyperreal field is an ultrapower of ℝ, set-sized, so *ℝ
embeds into No, while No, a proper class, embeds into nothing set-sized. Under
structural inhabitation the two are comparable, one way: ℝ ⊂ *ℝ ⊂ No is a chain. (As
literally constructed sets neither contains the other, but literal containment is not
well-defined across constructions at all, ℝ-as-cuts versus ℝ-as-Cauchy-classes, so the
structural reading is the only one the test can be using.) The corrected picture: a
tree with three mutually incomparable branch families above ℚ, the non-orderable
Cayley-Dickson line (ℂ, ℍ, 𝕆), the ordered-field line (ℝ, *ℝ, No, itself a chain), and
the p-adics (one per prime, pairwise incomparable), rather than file 64's four. The
correction shrinks the branch count by one and does not rescue the chain claim; the
verdict is untouched. It matters anyway, because the fix file 64 proposes ("give No,
*ℝ, and each ℚ_p their own independent membership predicates") would encode an
incomparability between No and *ℝ that is not mathematically there, and a vocabulary
that "cannot be got wrong" (D38's own ground) should not ship one.

**The cleaner uniqueness theorem, offered for the crate's actual shape.** The scoping
file 64 recommends (confine the "finest" fact to the real/Cayley-Dickson chain) is
broader than what is provable and needed. Every arvo numeral's value set is a finite
set of *rationals*: a value is m · r^q with integer m and integer q, for any radix and
any of the identity axes, so no arvo value set ever has ℝ, or anything above ℚ, as its
finest system. The vocabulary members at or below ℚ, in the ratified ten, are ℕ ⊂ ℤ ⊂
ℚ, a genuine chain; every other ratified member contains ℚ through the unique
characteristic-zero embedding (ℚ is the prime field, so these embeddings are canonical,
which is also why inhabitation is unambiguous exactly up to ℚ and only there). Hence:
**the finest inhabited system of any arvo numeral exists, is unique, and lies on the
sub-ℚ chain; the branches above ℚ are upward closure the finest fact never has to
name.** The uniqueness proof does not depend on the whole vocabulary being a chain, so
it survives every branch the vocabulary grows. One caveat to record, not resolve: if
per-radix localisations (ℤ[1/2], ℤ[1/3], ...) ever join the vocabulary as members, the
sub-ℚ fragment stops being a chain for coprime radices (ℤ[1/2] and ℤ[1/3] are
incomparable), each single numeral still has a unique finest (one radix per numeral),
but a mixed-radix expression's join is no longer a chain join, and the crate's shape
should not foreclose that day.

*Grounded on: ratified (D38, D39 at `202607291900:41-84`, read fresh); settled shapes
(39:340-360, 64 section 3); reasoned (the universality correction and the sub-ℚ
theorem are standard mathematics, Conway's universal embedding theorem, the
characteristic-zero prime field, Ostrowski for the p-adic branch; nothing in this
toolchain could compile them and I do not dress them as probes).*

## 5. The two primary-source reads: one was already done and carried as owed for two stretches; the other is now done, with one named residual

**E4M3. The check op marked pending at `68b:36-37` was already performed, before op
wrote that line, at file 62, and the consolidations have carried it as owed ever
since.** `62_probes/primary_sources.md` reads the OCP OFP8 specification itself
(Revision 1.0, from the Open Compute Project's own FP8 repository, not vendor
documentation) at position: prose at §5.1, Table 1 and Table 2, and the §4.2 defect.
Consolidation six absorbed it (63:300-315) and its own op-item list says "witnesses now
primary-sourced" (63:757). `68b` then wrote "pending the primary-source check on the
E4M3 exponent figure against the specification rather than vendor documentation," and
consolidations seven and eight carried that forward as unperformed (78:850-852, "Not
performed this stretch"). The review's own record contradicts its own open-items list,
and the table-diff obligation, which checks tables against sources, never checked the
open-items list against the record. That is two stretches of a performed check riding
the owed list because nobody grepped `62_probes/`.

**The independent second read, performed now, from a fresh copy of the document.**
Downloaded from the OCP FP8 repository this session and read by position
(`OCP 8-bit Floating Point Specification (OFP8) Revision 1.0`, dated June 20, 2023;
printed page numbers, which are the citation basis): §5.1 prose, printed page 12: "The
E4M3 format does not represent infinities and uses only two bit patterns for NaN (a
single mantissa-exponent bit pattern but allowing both values of the sign bit) in
order to increase emax to 8 and thus to increase the dynamic range by one binade."
Table 1, printed page 13: E4M3 exponent bias 7, emax (unbiased) 8, emin (unbiased) −6.
Table 2, printed page 13: infinities N/A, NaN S.1111.111, max normal S.1111.110 =
±448, min normal ±2^−6, max subnormal ±0.875 · 2^−6. And the defect file 62 found,
confirmed verbatim at §4.2, printed page 11: the abbreviations section states E4M3
with "an exponent bias of 15" and E5M2 with "an exponent bias of 7," transposed,
refuted by the document's own Table 1 and by its own value formula on page 12. Every
position citation in file 62's record verifies exactly. **Two independent reads of the
primary source now exist; the design's E4M3 figures (emax 8, bias 7, max finite 448,
even stored significand at the maximum) are primary-sourced twice over, and the item
can close on op's word, together with the stale open-items row.**

**A by-product that closes a third owed item.** Table 3, printed page 14 (§5.2,
conversion behaviour), is the OCP mode split itself: conversion of a value "greater
than max OFP8 magnitude" to E4M3 delivers ±max_E4M3 in saturating mode and NaN in
non-saturating mode, in the document's own table. The "OCP mode-split facts behind
file 71's declined NaN-on-overflow ground 4," listed as an owed primary check at
78:937-938, are hereby primary-sourced: NaN-on-overflow is a deployment *mode* of the
conversion, exactly as file 71 argued when it routed the option to the hardware door's
environment fact rather than to `Resolution`.

**The overflow tie.** Performed against IEEE Std 754-2008 (a full archived copy,
UMBC-hosted, read by clause this session), with the residual named below. Clause
4.3.1, first paragraph, verbatim: "In the following two rounding-direction attributes,
an infinitely precise result with magnitude at least b^emax (b − ½ b^(1−p)) shall
round to ∞ with no change in sign; here emax and p are determined by the destination
format (see 3.3)." The threshold b^emax(b − ½b^(1−p)) is the maximum finite plus half
a top-binade ulp (I verified the arithmetic: at b = 2 the difference from the maximum
finite b^emax(b − b^(1−p)) is exactly 2^(emax−p)), and "at least" includes equality,
so **the tie rounds to the infinity, unconditionally, under both nearest attributes,
by the standard's own fiat**. Clause 7.4 agrees from the exception side, verbatim:
"The overflow exception shall be signaled if and only if the destination format's
largest finite number is exceeded in magnitude by what would have been the rounded
floating-point result (see 4) were the exponent range unbounded," with default (a)
"roundTiesToEven and roundTiesToAway carry all overflows to ∞ with the sign of the
intermediate result." This corroborates file 71 exactly as file 71 predicted it
would: the standard states the tie-to-infinity as a threshold fiat; the design
*derives* the same behaviour for every IEEE format from parity on the extended grid
(the maximum finite's stored significand is all-ones, odd, so ties-to-even rounds
away), and derives the opposite parity for E4M3, which the standard does not govern.
The corroboration is of the conclusion, not the mechanism, and file 71 already said
so (71:98-105). **The residual, named:** the sentence was read in the 2008 text. The
2019 revision's change to clause 4.3.1 is, per the standard's own revision summaries,
the extension of roundTiesToEven to the both-nearest-neighbours-odd case (a decimal
cohort corner reachable in neither binary formats nor this sentence), and file 62's
2019 extract of clause 5.2 shows the 2019 numbering intact; but the 2019 wording of
4.3.1 itself I have only at one remove, and I say so rather than round it up. The
tie derivation depends on nothing in that delta.

*Grounded on: physical, primary (the OFP8 document read by printed-page position this
session; IEEE 754-2008 clauses 4.3.1 and 7.4 read verbatim this session); physical,
secondary (the 2019 delta at 4.3.1); settled shapes (62_probes/primary_sources.md,
63:300-315, 71:94-113); the staleness finding is a record check (68b:36-37 against
63:757 and 62_probes/, all cited above).*

## 6. The nine-bit companion: built, clean, and it is the first model at which statement P is not vacuous

The owed item (67:406-407 priced it; 78:944-945 carries it unbuilt): every model claim
in the review exercises the u8 container class without saying so; container class is a
transfer coordinate with a threshold of one width per class; nine bits is the cheapest
member of the next class. Built at `80_probes/probe_3` (0.42 s): the u16 container
witnessed (size 2; logical wrap and container wrap differ at nine bits and are asserted
to *coincide* at eight, the vacuity fact stated in code); the unsigned and the signed
two's-complement order matrices exhaustive over 2^18 pairs each, datum order equal to
value order throughout, injectivity everywhere, exactly one signed zero pattern. The
u16-class instance of the injectivity claim file 64 closed at eight bits now exists,
and the transfer coordinate has its second class.

**The finding the build produced, beyond the item as costed.** At every eight-bit
model, logical width equals container width, so the carrier has no padding bits and
every padding claim (statement P, the padding law, the canonicalisation-forced
argument of file 73) was checked only where it is vacuously true. Nine bits in a u16
is the first model with padding at all, seven bits of it, and the probe exercises what
that means: over the whole matrix of same-value-different-padding pairs (512 canonical
data times 127 nonzero padding patterns, 65,024 pairs), a compare keyed on the raw
carrier misorders every single one while the canonical compare is Equal, and a witness
shows raw order inverting value order outright (a dirty zero above a clean one). This
is the `TotalOrd`/layer-keying defect class, reproduced at the carrier-identity layer
exactly where file 73 predicted almost nothing may key, and it means the companion
model is not merely a second coordinate point: it is the first point at which the
padding half of the crossing contract (statement P) has observable content. Any future
model built to check a padding or byte-image claim must sit in a class where logical
width is strictly inside the container, or it checks nothing.

*Grounded on: settled shapes (67:400-410, 68:490-521, 78 section 1.22); compiled
(`80_probes/probe_3`, exhaustive at every stated matrix).*

## 7. Out of scope, reported under the standing obligation

**File 79's search report is false, and the artifact it could not find is in the
panel's own corpus.** File 79 states: "I could not locate the specific prior artifact
the brief points at inside this panel's own corpus (searched `[Aa]rity` across every
file; the hits are all fold-arity, an unrelated subject in files 18 and 19)"
(79:137-140). The grep it describes, run fresh this session, hits over forty files,
including 25 occurrences in file 64 and 14 in file 55; the artifact is the sealed
`Arity` carrier (`Fin<P>` | `Unbounded`), proposed at file 55, named as owing its seal
at file 62, and discharged with a compiled forgery and a compiled fix at file 64
section 2. That search sentence cannot have been run as described, and a false
diligence claim in a file offered as one of two independent reads on the capacity
question is exactly the kind of thing the second read must know. The substance
matters too: file 79 reasoned its substitute answer from an ordinal-versus-cardinal
distinction ("a marker that must witness *which* position needs a carrier; a count
does not"), but the record's actual reason the arity vocabulary needed sealing is
carrier-at-birth, a *guarantee* (`InteriorSafety`) quantified over an open vocabulary,
with the forgery compiling through the orphan rule's uncovered-type-parameter
carve-out (64:160-199). A capacity shares that reason the moment any guarantee
quantifies over capacities. File 79's conclusion (no new seal needed) happens to
survive, but on grounds it did not state: `Capacity: Nat` has no generic parameter
slot, so the uncovered-parameter forgery route does not exist, and the seal on `Nat`
plus the orphan rule close foreign impls compositely. That composite closure argument
is owed one written sentence in whatever hardens the capacity design; the capacity
second read, which is not my ledger, should carry it, and should weigh 79's
conclusions knowing its stated search was not performed as stated.

**The consolidation's open-items list has no verification mechanism, and it just cost
two stretches.** Section 5's staleness finding generalises: the table-diff obligation
checks tables against sources, and nothing checks the owed list against the record of
what was performed. The E4M3 row rode as owed through two consolidations after being
performed and absorbed. A one-line discipline fixes it: an owed item names the
artifact whose existence would close it, so closing is a grep, not a memory.

**The `mock/Cargo.lock` gap noted in the gates section**: file 75's committed bench
manifests were not accompanied by the lock regeneration; any member running the test
gate inherits a modified lock in their tree. One commit closes it.

## What a consolidation could take, close to verbatim

The crossing repair's second read confirms `Crosses<N: Numeral>: Lowering` as the
obligation's home, derived rather than chosen, and drops the presumptive marker on
op's word; the spec text corrects two compressions before hardening: every impl is
spelled `unsafe impl` (the safe blanket of 68:271 is refused with E0200, compiled) and
what D16 splits is who discharges the proof, never the spelling; and statement 0
quantifies over every bit pattern of `Encoding::Fields`' width, with partial decodes
expressed by shrinking the fields, closing the file-73 domain flag by construction,
and every consumer-site `unsafe impl Crosses` is a named entry in the trusted base.
Statement 0 is preserved by both datum-dependent operations, compiled exhaustively at
a cohort-carrying decimal model: `roundToIntegralExact` is total and fibre-preserving
(file 67's guess holds for it exactly), while `quantize` is closed only through its
refusal branch (35.5% of model pairs) and its result value reads the operand datum, so
it is pair-keyed as a whole and can never be a law, which is the standard's own clause
5.2 carve-out landing as a keying fact. The exact fold width `bitlen(A * (2^P - 1))`
is built at the type level, gate-free, verified against ground truth on 114 cells with
the loose band and the tight cells pinned, and priced at under a millisecond per
instantiation; `foldnum` can retire to prose. The membership verdict stands on two
independent reads with one correction to the first read's mathematics (the surreals
embed every set-sized ordered field, so No and *ℝ are comparable and the branch
families above ℚ number three, not four) and a cleaner uniqueness theorem: every arvo
value set is rational, the ratified sub-ℚ vocabulary is a chain with canonical
embeddings, so the finest inhabited system exists and is unique on the sub-ℚ chain for
every arvo numeral, independent of every branch above. The E4M3 primary-source check
op marked pending at 68b was already performed at file 62 and has ridden the owed list
for two stretches; a second independent read of the OFP8 document now exists,
position-cited, confirming bias 7, emax 8, max finite 448 and the §4.2 bias
transposition, and Table 3 (page 14) primary-sources the OCP saturation-mode split as
a by-product. The overflow tie is read verbatim at 754-2008 clause 4.3.1 ("magnitude
at least b^emax(b − ½b^(1−p)) shall round to ∞ with no change in sign") and clause
7.4, corroborating file 71's derivation, with the 2019 wording of 4.3.1 held at one
remove and named as the residual. The nine-bit u16-class companion is built, both
order matrices exhaustive and clean, and it is the first model at which statement P
has content at all: 65,024 same-value-different-padding pairs all misorder under a
raw-carrier compare, so padding claims are checkable only in classes where the logical
width sits strictly inside the container. And file 79's stated corpus search for the
prior arity artifact was not performed as described; the artifact is the sealed
`Arity` of files 55, 62 and 64, its reason was carrier-at-birth rather than
ordinal-versus-cardinal, and the capacity second read should re-ground that paragraph
before 79 hardens.
