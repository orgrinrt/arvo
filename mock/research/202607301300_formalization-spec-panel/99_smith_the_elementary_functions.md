# 99. The elementary functions: the family sorts by the kind of its exact carrier, and only one class needs anything the design does not have

**Member:** Julius O. Smith III. I wrote file 24 (the multiplicative half: rounding relocates from
the operation to the narrowing) and file 43 (division: the relocation survives by changing the kind
of the exact intermediate, from a wide product to the Euclidean pair). This file is the third
application of the same habit of mind, and the last the operation surface needs: for every function
past division, ask what the exact result IS, what finite object decides its rounding, and how wide
that object grows. The answer sorts the whole family into three classes, two of which extend the
design at zero new mechanism and one of which needs exactly one new thing, named in section 4. The
audit at file 98 called this hole "plausibly one decision plus one growth-class derivation"
(`98:315-319`); the estimate was close, and the derivation is below, compiled where it could be.

**Gates, run before the work.** Canon gate: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD
`037c9cd`. Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary: **155
binaries, 672 passed, 0 failed, 9 ignored**, matching file 98's count exactly, with the same
six-past-the-committed-666 attribution 97 and 98 both made (the concurrent dispatch's uncommitted
`mock/Cargo.toml` and `mock/benches/*` work, visible in `git status`, untouched by me per the
dispatch boundary). The test surface this file touches, read in full rather than counted:
`arvo/tests/numeric_contracts_const_probe.rs` (const-position compile pins on `Sqrt`/`Recip` at
integer and float widths, real assertions with stated tolerances) and `arvo-spectral/tests/power.rs`
(convergence assertions against known eigenvectors, magnitude-bounded, honest about the
unnormalised-seed contract at zero iterations). Nothing disqualifying; the one standing
disqualified test, `arvo-tensor/tests/capacity.rs:14-18`, is ruled at `95b:145-149` as op's own
trivial commit outside the panel, and I add only the count: twenty-two files now.

**What I read.** `91_consolidation_nine.md` in full, the standing base. `98_knuth_how_complete_is_
the_canon.md` in full (the dispatch source). `95b_persona_checkpoint_twentythree.md` in full (the
stand-in checkpoint that adopted the division surface this file extends). By exception, where the
consolidation compresses what I build against: `93_lattner_the_zero_divisor.md` section 5 (the
adopted solution-set derivation), `84_leijen_failure_that_is_not_a_range_event.md` sections 4 and 5
(the three-kind taxonomy and the four homes), my own files 24 (section 6) and 43 (reread as claim
lists, not trusted), `29_wronski_the_quantisation_contract.md` at the `quantize` signature
(`29:111-116`), and `72_giesen_the_unexamined_ground.md:73` (the row this dispatch descends from).
One `ls` of the panel directory, current through `98`. Shipped source at three factual-check
points, recorded where used, none read for meaning.

**What I compiled or measured, separated from what I reasoned.** Four probes in `99_probes/`, all
outcomes in `99_probes/OUTCOMES.md`, every count Python-precomputed before the Rust that pins it
(`precompute.py`, the file-43 discipline). Probes 1 and 2 are Rust at the pin (`rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`); probe 2's emitted-code findings
are under `-O --emit asm` on that target and say so. Probes 3 and 4 are exact-integer and
60-digit-decimal Python; no float enters a load-bearing comparison anywhere. No timer ran; no
runtime cost claim is made (the divider-ladder precedent, `43:335-338`, and the bench harness is
the concurrent dispatch's this round). Reasoned and marked as such: the Lindemann-Weierstrass
citations (external standard mathematics, the review's practice per the Conway precedent at
`91:209-212`), the IEEE cross-checks (secondary, primary reads owed into the existing bundle), the
class assignments of functions not probed (ln, sin, cos, atan), and every spec-shaped sentence in
sections 4 through 6.

## 0. Checking the brief before reasoning from it, per the standing discipline

Three factual claims in my dispatch, checked first.

**"Two of them already have a failure-kind classification in the record and no chapter behind it":
TRUE.** `Sqrt` and `Recip` appear in the ratified taxonomy as Kind 2 generators (`91:425-426`) and
in the adopted solution-set derivation (`93:302-305`), and a corpus grep confirms file 98's finding
that nothing else has landed since file 72 flagged the gap (`72:73`).

**"The first operation family in this design whose partiality is about the operand's value rather
than the result not fitting": FALSE.** Division is that family, and it is already ratified as such:
the adopted surface is "each exact, each partial on the divisor's nonzero-ness" (`91:284`), and the
divisor's zero is an operand value, not a result that failed to fit. More: the adopted solution-set
derivation already names `Sqrt` of a negative explicitly as its clause-3 instance (`93:302-305`,
"empty with no direction, invalid"). So the domain half of this family's story is not a candidate
for new design; it is an inheritance, and the chapter's job there is to say so (section 3). The
brief's instinct that "saying which is worth more than the chapter's other pages" was right; the
answer is just the opposite of the one the framing leaned toward.

**"The design has a settled quantiser": TRUE WITH ONE UNPINNED WORD, and this family is the first
consumer that can tell.** The quantiser is ratified (round-first, classify second, `91:177`), but
its input's domain is not: the signature takes an `ExactValue` (`29:111`) whose breadth no ratified
sentence states. Until now the gap was invisible because it was vacuous: every exact result of
addition, multiplication, division, and `quantize` itself is a RATIONAL (the membership theorem,
`91:184-195`, makes every arvo value one, and the field operations stay inside ℚ). The exact result
of `sqrt` is an algebraic irrational and the exact result of `exp` is transcendental, so this
family is the first whose quantiser input leaves ℚ, and the definitional-completeness line owes
`ExactValue` its definition the day any of it is ratified. The needed widening is one clause, in
section 2, and the separation requirement is what found it: the rational-versus-real distinction
had been vacuous at every instantiation the review ever checked.

*Grounded on: ratified (`91:284`, `91:177`, `91:184-195`), settled shapes (`93:302-305`,
`84:239-243`, `29:111-116`, `72:73`), measured (the corpus grep, this session), reasoned (the
vacuity observation, mine).*

## 1. The verdict: three classes by carrier kind, and the family extends the shape

**The question the dispatch poses, "does the family extend the shape the design has, or does it
need something the design does not have," has a compiled answer: it extends the shape, through the
same two moves that got multiplication and division through, and it needs exactly one new thing,
which is a provenance class for a const, not a mechanism.** The sort:

| Class | Members | Exact carrier | Decision width | Ties | New mechanism |
|---|---|---|---|---|---|
| Derived | `recip`, integer `pow` | division's pair; the fold | inherited | inherited | none, one sentence each |
| Root | `sqrt`, fixed n-th roots | the root-residue pair (m, r) | **linear**, P + F bits | **impossible**, parity | none |
| Radix-power exponential | `exp2`-shaped, on matching-radix grids | integer power comparison | **exponential in F** (13, 25, 57, 113 bits at F = 1..4) | impossible off integer exponents | none, but the carrier is refused at practical width, like division's lcm accumulator |
| Transcendental | `exp`, `ln`, `sin`, `cos`, `atan` | **none exists** | empirical (the hardness const: 11, 9, 10 bits at three model numerals) | impossible, Lindemann-Weierstrass | **one**: the exhaustively-computed-or-cited const (section 4) |

Every failure of every member is classified by machinery already adopted: the range events by Kind
1 and the preset `OverRange` rows, the domain events by the solution-set derivation's three clauses
(`93:287-305`), and the taxonomy needed no fourth kind, which is the strongest single corroboration
this file offers for the taxonomy: it was built on division and absorbed an entire family without
edit. The quantiser stays the only rounding authority, subject to the one-clause domain widening of
section 2. And the grade story is unchanged: every member charges one site (the site count is a
function of the type, `91:373-377`), and the moved count differs per class in a way that is itself
a theorem (section 3).

**Where the honest answer is "does not belong": nowhere, but two members belong as derivations
rather than chapters.** `recip` is general division with the dividend fixed at one:
`quantize(1/x)`, the domain event at zero resolved by solution-set clause 2 (empty with direction;
the direction is the dividend's sign against the approach side, and the sign clause of `93:307-310`
covers the single-zero convention). It is NOT the exact subfamily (that lifts the DIVISOR to type
position; `recip` fixes the DIVIDEND), so it inherits general division's quantiser path, growth
class, and grading, and its chapter is this paragraph. Integer `pow` is an iterated `mul_full` with
one quantisation at the root, which is the fold chapter verbatim (`43:300-310` made the identical
argument for division folds); its one wrinkle is that `x^0` requires the numeral's ONE, so over the
identity-free numerals (`UFixed<0, F>`, `78:723`) the exponent domain starts at one, a corollary
the `Identity` bound expresses with no new text.

*Grounded on: ratified (`91:276-294`, `91:420-429`, `95b` section 2), settled shapes (`93` section
5, `43` sections 2 and 5, `78:723`), compiled (`99_probes/` probes 1 through 4), reasoned (the
class assignments of ln, sin, cos, atan, from the same transcendence results as exp, marked
below).*

## 2. The root family: the carrier changes kind a second time, and gets cheaper (probes 1, 2)

File 43's finding was that division's exact intermediate is not a numeral but a pair bound by a
compiled law. The root family repeats the move with a different pair, and lands in a cheaper growth
class than division did.

**The root-residue carrier.** For the same-grid unsigned sqrt, operand index k, quantum 2^-F, the
exact result index is t = sqrt(k * 2^F). The finite exact carrier is the pair (m, r) with
m = isqrt(k * 2^F) and r = (k * 2^F) - m^2, bound by the defining law m^2 + r = k * 2^F with
0 <= r <= 2m, and **correct rounding to nearest is the single comparison r > m** (round up), by
squaring the midpoint: t > m + 1/2 iff 4(m^2 + r) > (2m + 1)^2 iff r > m. Compiled exhaustively at
nine (P, F) shapes against a definition-shaped oracle that never computes a root (cross-multiplied
squares, the `43` probe-4 pattern), in const position, so the compile is the verification (probe 1,
claims A, D). The widest integer the entire decision touches is the scaled operand itself, P + F
bits: **the root family joins multiplication's linear growth class, not division's exponential
one**, and file 43's three growth classes gain a member without gaining a fourth class. The n-th
root is the same shape with n-th powers (probe 1, claim E, cube root compiled, counts pinned), at
carrier width n-linear in the precision.

**Ties cannot occur, and this is a parity theorem, not an observation.** A nearest-rounding tie
requires (2m + 1)^2 = 4(m^2 + r), that is 4r = 4m + 1, even equal to odd. Zero ties at every sweep
(probe 1, claim B), and the classical shadow of this fact is that hardware square root has never
needed a tie path. Design consequence: the direction triple's tie rule is never consulted by a
root, which a law about roots may state and a verifier may pin.

**The overflow band has a closed-form emptiness criterion, and its inhabited side is the
identity-free numerals again.** Same-numeral sqrt overflows nothing when the far point is at least
1 - q (criterion M >= 2^F - 1; probe 1, claim C, predicted and confirmed at all nine sweeps), and
overflows on a NONEMPTY band otherwise: at (P, F) = (2, 4), three of the four nonzero operands
overflow. The reason is the fixed points: sqrt pins 0 and 1, so a value set with no 1 has nothing
to pin its top, and sqrt escapes upward. These are exactly the `UFixed<0, F>` numerals whose
missing ONE produced the `78:723` defect, and the coincidence is not one: the same absent element
breaks the multiplicative identity and opens the root's overflow band. UnderRange is empty
unconditionally (no nonzero operand rounds to zero, claim C'). Every event lands in rows the
presets already have; no new resolution text is owed.

**The domain is an axis the type already carries, and this is the cleanest sentence in the file.**
Over `Domain = Unsigned`, sqrt is TOTAL: the adopted division surface's own clause, "partiality is
refused at declaration wherever the divisor's domain is a predicate" (`91:284-285`), applies with
the predicate being the existing `SignDomain` axis rather than a new niche, so the refusal costs
nothing and the type system already spells it. Over a signed domain, the negative half is
solution-set clause 3 verbatim (`93:302-305`): empty with no direction, `invalid`, NaN where the
numeral carries one, partial otherwise. Inherited, not invented. The shipped tree's contract is the
motivating counterexample, cited as why-evidence: "Signed-input impls clamp negatives to zero or
panic per per-impl contract" (`arvo-numeric-contracts/src/lib.rs:49-51`), a per-impl unstated
convention, the divide-by-zero-returns-numerator pathology's third appearance.

**The const form and what each position emits (probe 2, `-O`, `aarch64-apple-darwin`).** The
correctly-rounded sqrt is a `const fn` from the residue rule; in const position it folds to
`mov w0, #362` (the guaranteed form, per the pricing pillar's clause at `91:115-119`); in value
position the body is fourteen instructions, a Newton isqrt loop with one `udiv` per iteration,
after which **the entire correct-rounding decision is `msub, cmp, cinc`: three instructions,
branchless**. The rounding is nearly free; the root extraction is the cost, and which ladder
computes it (hardware `fsqrt` through the door, Newton, restoring per target) is the
`arvo-always-optimal-internals.md` bin exactly as division's ladder was (`43:270-272`).

**The quantiser's one-clause widening, owed to the definitional-completeness line.** The
quantiser's input, `ExactValue` (`29:111`), must be defined as **any real number equipped with a
decidable ordering against the target grid's points and midpoints**, not as a rational. Every
existing operation's exact result is rational, so the widening changes no ratified behaviour and
no existing text; it makes the roots' irrational inputs legal, and the decidability condition is
precisely what the carrier classes grade: the roots decide by a linear-width integer comparison,
the radix-power exponential by an exponential-width one, the transcendentals by the empirical
bound of section 4. One clause, and the quantiser is the whole answer for the family, which is the
dispatch's first watch-item resolved: the settled quantiser IS the whole answer, once its domain
says what it quantifies over.

*Grounded on: ratified (`91:284-285`, `91:115-119`, `91:177`), settled shapes (`93:302-305`,
`43:270-272`, `78:723`, `29:111`), compiled (`99_probes/probe_1`, `probe_2`, all claims, const
position, exhaustive), verified at source (`arvo-numeric-contracts/src/lib.rs:49-51`, why-evidence
only), reasoned (the fixed-point explanation of the band criterion; the criterion itself is
compiled).*

## 3. The exponential family splits on the radix, and the split is algebraic against transcendental (probes 3, 4)

**The radix-power exponential is decidable and exponentially wide, division's class-D twin.**
2^(k/2^F) is algebraic (a root of y^(2^F) = 2^k), so exact hits and ties on a dyadic grid are
exact integer comparisons: hits occur exactly at integer exponents and ties never (probe 3, claims
A and B, exhaustive at F = 1..4). But the comparison object j^(2^F) doubles in width per
fractional bit: 13, 25, 57, 113 bits at F = 1 through 4 (claim C). Decidable in principle,
refused as a practical carrier, which is precisely the shape of division's lcm accumulator
(`43:145-155` the width table; `43:340-343`: exists, exponential, a derivation rather than a product). So even the decidable
half of the exponential family reaches the same operational posture as the transcendental half:
bounded working precision, correctly rounded where the bound is known sufficient.

**The transcendentals have no exact carrier at all, ties are impossible for a deeper reason, and
the moved count becomes a theorem.** For nonzero rational x, exp(x) is transcendental
(Lindemann-Weierstrass; external standard mathematics, cited per the review's practice,
`91:209-212` precedent), and so are ln(x) for positive rational x other than one, and sin, cos,
atan at nonzero rationals. Since every grid point and every midpoint of an arvo numeral is
rational (the membership theorem again), a transcendental result **never lands on either**: no
exact hits off the removable special points (exp(0) = 1, ln(1) = 0, sin(0) = 0, each an
enumerable, per-function, finite list), and no ties, ever, at any width. At the two-count layer
this is a theorem the record should carry: **for a transcendental member, the moved count equals
the site count at every operand off the removable list**, with no per-value bookkeeping needed to
know it, which is the one place in the design where the expensive count comes for free.

**The domain events are clause-2 and clause-3 theorems, and IEEE's own table drops out again.**
ln at zero is an empty solution set with a direction (e^q = 0 has no solution and the one-sided
limit runs to the low far point): solution-set clause 2, the range resolution borrowed downward,
which is IEEE's log(0) = negative infinity with `divideByZero` recovered as a derivation, the same
way `95b` section 2's adopted derivation recovered clause 7 for x/0. ln of a negative is clause 3,
`invalid`, exactly as sqrt of a negative. The adopted derivation was built for division and
classifies this family's every domain event without edit; that is its second family and the
strongest evidence yet that it is the design's general failure classifier, worth one sentence of
ratified text saying so. (IEEE cross-checks secondary; primary quotations owed into the bundle
`78:934-941` already carries, beside the existing clause-7 items.)

**The hardness const: the one genuinely new thing (probe 4).** Correct rounding of a
transcendental terminates per value (Ziv's staged refinement; termination because the exact value
is never a boundary), but the working precision that suffices for EVERY operand of a numeral is
not given by any formula. It IS a well-defined constant of the type: the value set is finite, so
the worst-case boundary distance over it exists, and the pricing pillar's own standing test ("a
function of the type's parameters alone", `91:119-121`) is passed to the letter. What fails is
the derivation economics: the const is computable only by exhaustion over the value set, or by
citation of published worst-case searches. Measured at three model numerals: 11 extra bits at
P = F = 8 (hardest operand 112/256), 9 at P = F = 6, 10 at P = 8, F = 4, zero ties anywhere, and
no visible formula connecting the three (probe 4). Two consequences the record should carry
verbatim:

1. **The hardness const does not transfer across widths.** It is a maximum over a different set
   at every width, so, unlike the laws whose model-width checks transfer by uniformity (the
   file-66 argument), hardness at width W is established at width W by exhaustion, or cited for
   that width, never transferred. Exhaustion is priced by the review's own measurements: the 2^p
   const-eval wall (`08`, via `unstable-features.md`) caps compile-time exhaustion at model
   widths, and published worst cases (the Lefevre-Muller searches) exist only for IEEE widths and
   specific functions. This is the family's honest boundary and it should be stated as one.
2. **The const's provenance is a new class: exhaustively-computed-or-cited.** It enters the
   design's grounding registry beside compiled and measured, and where it is cited rather than
   exhausted it is a trusted-base entry with the citation as its named artifact, under the same
   accounting as every hand-laid `Crosses` entry.

*Grounded on: ratified (`91:119-121`, `91:184-195`, `95b` section 2), settled shapes (`93` section
5, `66` the transfer argument, `08` via `unstable-features.md` the const-eval wall), compiled
(`99_probes/probe_3`, `probe_4`), external (Lindemann-Weierstrass, cited not compiled, the
`91:209-212` practice; IEEE 754-2019 clauses 7.3 and 9.2, secondary, primary reads owed), reasoned
(the class assignments of ln, sin, cos, atan; the moved-count theorem, from the no-boundary-hit
fact).*

## 4. What the family may promise: derived from the ratified naming principle, not invented

The shipping shape for the family is already decided by ratified text, and this section derives it
rather than proposes it. The naming principle (`91:846-851`): a name may promise behaviour only
where the design names the verifier; until the verifier exists the promise is a trusted-base entry;
a name promising behaviour with no designated verifier is forbidden.

Applied per class. **The roots may promise correct rounding**, because their verifier is the
defining law itself: the residue characterisation is compiled, exhaustive at model widths, and
cheap at every width (the three-instruction decision), so `sqrt` names correctly-rounded behaviour
with a verifier the implementation phase pins per `79b`'s exhaustiveness mandate. **The
radix-power exponential may promise correct rounding per width**, where the chosen working
precision is verified sufficient for that width (a bounded, decidable check). **A transcendental
may promise correct rounding exactly at the widths where its hardness const is exhausted or
cited**, and nowhere else; elsewhere it ships as a **licensed approximation**: the stated error
bound is a type-level const and IS the contract, the mechanism under it (degree, table size,
argument reduction) is internal per `arvo-always-optimal-internals.md`, the verifier is the
differential parity suite op's own `79b` mandate already designates, the grade charges its one
site, and the published fact is the Bound-family entry my file 24 defined for exactly this
(`24:380-385`). A `sin` that promises IEEE 9.2 correct rounding with no cited worst case is
already forbidden; nothing new needs adopting to forbid it.

Two postures the derivation respects. The toolbox rule: the design declines no function; it
prices them, and the bound is the knob the consumer holds (they know their budget; the substrate
cannot), while the mechanism is not. And IEEE's own posture corroborates the split: clause 5
REQUIRES correctly-rounded sqrt while clause 9.2 only RECOMMENDS correctly-rounded
transcendentals, which is this section's licence structure stated by the standard itself
(secondary; the primary quotation is owed with the others).

*Grounded on: ratified (`91:846-851` the naming principle, `79b:20-27` and `79b:53-57` the
verifier mandates, `arvo-toolbox-not-policer.md`, `arvo-always-optimal-internals.md`), settled
shapes (`24:380-385`), reasoned (the per-class application, which is the section).*

## 5. The closure sentence, offered in the form the audit costed

File 98 found no ratified sentence closes the operation catalogue, so a reader cannot tell
finished from stopped (`98:129-133`), and costed the fix at one line. The line should be an
admission test rather than a list, because a list reopens with every member and a test does not:

**"The operation surface is the following table, and an operation joins it by stating five
things: its solution-set characterisation (which classifies its failures under the adopted
derivation), its exact-carrier kind and width class, its grade sites at the site count, its
result-numeral rule, and, where its name promises behaviour, its designated verifier. An
operation absent from the table and not admitted through this test is not in the design."**

The table today: addition and the fold (linear, p + log n); multiplication and `mul_full` (2p);
division as `div_floor`, `rem`, `div` (the Euclidean pair; exponential over a divisor numeral,
linear over a constant); `quantize` in both forms; the roots (the root-residue pair, linear);
`recip` and integer `pow` as derived forms; the exponential-and-friends family through section 4's
licence structure. Every row's five statements exist in the record as of this file; the sentence
is one ratification away, and the definitional-completeness line then holds every future member to
the same five.

## 6. What this file does not decide

**Whether any transcendental ships in the first contracts crate** is a packaging call and op's;
what shipping would mean is now stated, which was this chapter's job. **The argument-reduction
laws** (exp(a + b) against exp(a) * exp(b) as a licensed composition; range reduction for sin
against a correctly-rounded multiple of pi, the classical Payne-Hanek territory) are flagged and
sized as the transcendental chapter's real second page, not designed here; nothing above depends
on them. **The n-th root's n in type against value position** is the same surface fork as
`div_or`'s fallback (`93:355-359`) and belongs to the same operation-chapter pass. **The signed
root halves** repeat file 43's owed signed re-runs and are owed the same way (a signed re-run of
probe 1 at a signed model). **ln's own compiled probe** is not built; its clause-2 and clause-3
sentences above are reasoned from the adopted derivation plus the standard, and the compile that
would move them bins is the same `Specials`-bearing model-numeral instance the float-division
compile already owes, one artifact serving both. **The runtime cost of any ladder** is a bench, in
`mock/benches/`, owned by the concurrent dispatch this round, and no number here speaks to it.

## 7. The two requirement performances, on this text, before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions:
*root-residue carrier* (defined, section 2: the pair (m, r) with m^n + r = X, 0 <= r bounded by
the next power gap); *carrier kind* and *width class* (defined by the section 1 table's columns:
what finite object decides rounding, and how its width grows in the type parameters); *removable
special point* (defined, section 3: an operand at which a transcendental member's exact result is
rational and representable, enumerable per function); *hardness const* (defined, section 3: the
maximum over the value set of the working precision that decides rounding, a type-level fact);
*exhaustively-computed-or-cited* (defined, section 3: the provenance class of a const passing the
pricing pillar's test whose value is obtainable only by exhaustion or citation); *licensed
approximation* (defined, section 4, by its three named components: a type-level bound as the
contract, an internal mechanism, a designated verifier); *admission test* (defined, section 5, by
its five statements). Terms used from the record without redefinition: site count and moved count
(`91` section 1.14), the far point, the solution-set clauses (`93` section 5), the quantiser and
direction triple, Kind 1 through 3 (`91:420-429`). Named open rather than defined: the
argument-reduction laws' composition licence, and `ExactValue`, whose definition this file OWES
and states in section 2 (any real with a decidable ordering against the target grid's points and
midpoints) as a proposal for the ratifying text, not as a term it may leave hanging.

**The separation requirement, performed.** This file's model is the three-class sort of section 1,
and it separates carrier kinds. Nonvacuous at two instantiations, each chosen because a wrong
subject would coincide elsewhere. First, sqrt against exp2: both algebraic, both decidable, both
tie-free off their exact points, so a model checking only decidability or only transcendence would
find them identical; they separate on carrier width (16 bits against 113 at comparable operand
widths, probes 1 and 3), which is the axis the sort claims is load-bearing. Second, exp2 against
exp: both exponential-shaped and both practically bounded-precision, so a model checking only the
operational posture would merge them; they separate on tie-decidability (an integer comparison
against a transcendence theorem) and on the hardness const's provenance (derivable per width
against exhausted-or-cited), which is where section 4's licence structure draws its line. At the
derived class (recip, pow) the sort's distinctions are vacuous by construction, and the verdicts
there are inheritance claims, not model outputs.

## 8. Standing

The elementary functions extend the design's shape: the quantiser is the whole rounding story
once its input domain gains one clause, the solution-set derivation classifies every domain event
in the family without edit and earns a sentence naming it the general classifier, the carrier-kind
move produces its third and cheapest instance at the roots, the growth classes absorb the family
without a fourth class, and the naming principle already dictates the licence structure for
everything past the roots. The one genuinely new thing is small and named: a provenance class for
one const, with the honest statement that it does not transfer across widths. Two of the
dispatch's three watch-items resolve to inheritances and the third to a one-clause widening; the
brief's "genuinely new partiality" framing is corrected against the record's own ratified text.
Everything spec-shaped above is offered in the consolidation's provenance form for the next
consolidation to take or strike; the packaging calls are op's; only op's calls are final, and even
those go stale.

*Grounded on: ratified (`90b`, `95b`, `91` sections 1.13, 1.14, 1.16, 1.27, `79b`), settled shapes
(`24`, `43`, `84`, `93`, `98`, `29:111-116`, `72:73`), compiled (`99_probes/probe_1` and `probe_2`
at the pin, exhaustive, const position), measured (`99_probes/probe_3`, `probe_4`, exact-integer
and 60-digit-decimal Python, values in `OUTCOMES.md`), external (Lindemann-Weierstrass; IEEE
754-2019 clauses 5, 7.3, 9.2, secondary, primary reads owed), reasoned (the unprobed class
assignments, the moved-count theorem, sections 4 through 6, marked in place).*
