# The current shape, fifth consolidation: the float model absorbed, the exponent settled everywhere

File 49 stood as the reference after the convergence four (files 41 through 48) closed. This document
replaces it, absorbing eight further deliverables (files 50 through 57) read against three stand-in
checkpoints made overnight by op's own persona, dispatched at Fable tier while op slept a second and
third night in a row (`48b`, `53b`, `57b`). Op has not yet read any of the three; every call inside them
is persona-decided, not op-decided, and every one dies the moment he says otherwise. Op's own four
checkpoints (`30b`, `34b`, `39b`, `44b`) remain exactly what they were and are restated below rather than
re-litigated.

This document corrects two defects `49` carried. First, `49:116` and `49:117` both spelled the exponent
bounds as `const` (`Implicit<const E: Exponent, ...>` and `Ranged<const EMIN: Exponent, const EMAX:
Exponent, ...>`), contradicting `49`'s own section 1.15, which derives from the spine rule that both must
be types. Both are corrected here: the exponent is a type for `Ranged` numerals (compiled, file 50) and
for `Implicit` numerals (compiled, file 54, which also names that it overturns file 36's own earlier claim
the opposite). The two lines sat uncorrected through three sequential readers (files 52, 53, and the
`53b` checkpoint) who each took `49:117` as the whole defect and did not look one line up; file 54 found
the second line. Second, `49`'s description of the arbitrary-16-bit-rational compile-cost sweep as "the
harder, more realistic case" has realism backwards. File 53 measured that sweep as the adversarial worst
case, not the realistic one, and the realistic profiles (dyadic quanta, division by real constants,
occasional chained conversions) sit two orders of magnitude below it. Both corrections are folded into the
sections below rather than flagged as errata; section 1.23 states the corrected cost-model paragraph, and
every table below carrying the exponent members spells them as types.

The stretch's arc: file 50 (Fog) built the float model and found the settled machinery absorbs it with no
new mechanism, closing the keystone item and, along the way, finding that the design's grade **is** the
IEEE flag word with the value thrown away. File 51 (Fallin) closed the last open ratification tick
(`Growth` leaves `Policy` entirely, by a structural coherence theorem rather than a fourth round of
corroboration) and built the reassociation licence for float folds, with a compiled hazard showing where
the available mechanism over-grants. File 52 (Ringer) landed the review's owed test debt as fifteen real
artifacts and classified which are contracts and which are measurements, finding along the way that a
compiled claim the review had repeated four times had never actually been re-verified under its own
corrected methodology. File 53 (Torvalds) re-asked whether the design earns its keep and answered with
numbers: the consumer-facing contract shrank while the derivation grew, the four-layer carrier structure
has a measured counterfactual at every layer, and the aggregate compile cost has a real, nameable cliff
that belongs in the spec rather than in a CSV nobody quotes. File 54 (Kiselyov) joined the float model and
the numeral tower for real, built `Radix` as a fifth sealed carrier, resolved `Specials` from a
three-instance chain into a two-fact product, and ran the crossing contract against decimal, where it
found the injectivity boolean is finally exercised for real rather than vacuously. File 55 (McSherry)
reframed the oldest open consumer question in the review (file 04's `Precise` exile) by showing the
presets that are admitted today return silently wrong orderings, and built the mechanism that fixes it.
File 56 (Jhala) tested whether the design's refusals are legible to a reader who has not seen this review,
found the carrier-at-birth seal is also the review's best diagnostic for free, and found the ceiling on
what diagnostic tuning can do about a mismatched numeral. File 57 (Aaltonen) ran the runtime bench the
review had deferred for twenty-six files, found the bench harness itself could not run any bench at all
before this stretch, fixed it, and measured the strategy-axis question a real number.

Op's four checkpoints made five calls between them: D69 ratified (already reflected in `49`, unchanged
here), D39 held, the value-unique encoding ratified in full with division held, and the grounding
registry adopted with a backfill obligation. The three persona checkpoints make sixteen further calls,
listed in full in section 2, each flagged with its own weight: **persona-decided, not op-decided**, and
several edit ratified tables, which is why section 2 closes with a single list of everything that most
needs op's eye.

**Verification.** Every claim below tagged compiled or measured was run against the pinned toolchain,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved from the repo's
`rust-toolchain.toml`. The design surface this review builds still has no shipped source for the numeral
tower: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same command with
`FullRange\|UTerm\|AddWidth` in place of the first pattern, both run fresh from the repo root for this
document, both exit 1, empty. That is unchanged since file 45 first corrected the path this command uses.
`cargo test --workspace`, summed per binary rather than trusted from a headline, reported 654 passed, 0
failed, 9 ignored across every file from 41 through 56, and 655 passed, 0 failed, 9 ignored from file 57
onward (one correctness test added alongside the new runtime bench, section 3). This document did not
re-run the full suite; every deliverable since 41 has reproduced it independently and file 57's own delta
is accounted for by the test it names. Every probe directory from `41_probes/` through `57_probes/`
carries an `OUTCOMES.md` with the verbatim build commands and error text reproducing its file's claims.

**The table-diff obligation, executed on this document by its own author before it stands.** The last
checkpoint (`57b`) made this a standing rule for every consolidation from here forward: a declaration
line in a table is a claim like any other, and it gets diffed against the sections it compresses before
the document is trusted. `49:116` and `49:117` are the exhibit of what skipping it costs. Every table in
section 1 below was checked line by line against the prose of the section it sits in, and against the
probe or checkpoint that established each member, immediately before this document was considered
finished. Two corrections this pass itself caught, beyond the two named above: the assembled trait table
(section 1.22) originally carried `Radix` with an open `const R: u64` spelling inherited by habit from
`49:110`'s own wording, corrected to `Rad<P>` once checked against section 1.11's own prose; and the
grounding-registry table (section 1.19) was missing the `unreproducible` row on a first pass, added once
checked against section 1.19's own prose naming it.

## The two rules

Two design rules are the stretch's real product, named once by the persona checkpoint at `48b` and
restated by every file since as the frame it works inside. Both keep firing.

**The spine rule.** A quantity that is computed and then has to appear in a type is a type; a quantity
that only ever has to be read is a const (first stated `47:503-508`, generalised at `48b`). This stretch
is the rule proving itself repeatedly against material nobody had reasoned about when it was named. In
order: the width chain and the biased-product formula (op, `44b`, the rule's founding instances); the
fold's `Grade` (files 47/48); the `Ranged` exponent bounds, reasoned by file 48 and compiled by file 50;
`Implicit`'s single exponent, compiled by file 54, overturning file 36's own contrary claim; `Capacity`'s
size, found by file 55 to need a `Pos` face alongside its array-length const, the first firing outside the
`Numeral` contract entirely, at a different crate and a different layer; and the notation macro's own
const-struct face, found by file 56 to hit the identical wall trying to seal a computed reduced-fraction
condition. The files disagree on the exact ordinal (file 55 calls its own finding "a fourth time"; file 56
calls its own finding "the fifth reachable position" in one sentence and "a sixth time" in another, an
inconsistency the files themselves never reconciled and this document does not resolve on their behalf).
What every one of them agrees on is the shape: the wall is not a numeral-specific fact, it is a fact about
`generic_const_exprs`-shaped positions and the forbidden-feature list, and any future carrier this design
mints should expect to hit it before assuming it will not.

**The carrier-at-birth rule.** A closed vocabulary that a guarantee quantifies over owes its seal and its
adversary at birth, not after three passes (`48:2.3`). Four more carriers were born under it this stretch:
`Radix` (as `Rad<P>` over a sealed `AtLeastTwo` predicate, file 54), `Specials`, `Underflow`, and the
signed exponent type itself (file 50), each closed with the two-obligation checklist at declaration time
and each costing two lines. File 56 found the rule pays a second dividend nobody had checked for: sealing
a carrier the way this design already seals one is simultaneously the correctness mechanism and the best
diagnostic the review has found. rustc's own sealed-trait detection explains a private-supertrait refusal
in plain English and lists every legal inhabitant, unprompted, with no `#[diagnostic::on_unimplemented]`
anywhere in the tower. That was true the whole time and nobody had checked what a person reads when the
seal fires until file 56 asked.

## 1. The agreed shape

### 1.1 What a number is

Unchanged from file 40 and file 49. A value of `Number<N: Numeral, S>` is an integer k, drawn from a
finite interval, together with a type-level rule injecting k into a set of rationals (plus, for floats, a
handful of data that are not rationals at all: `Specials`, section 1.16). The numeral names the
representable set and the indexing; D69 (op, `30b`) put them on two sides of the design, identity
parameterised in mathematical coordinates, not encoding coordinates.

### 1.2 The identity contract

```rust
pub const trait Numeral {
    type Radix:     Radix;        // Rad<P>, one constructor family over sealed Pos
    type Precision: Precision;    // significand digit count, primitive (D69), a Nat
    type Exponent:  ExponentForm; // where the exponent lives; nests the rest
    type Domain:    SignDomain;   // NonNegative | Symmetric | AsymmetricLow, a value fact
}

pub struct Implicit<E: Exponent, A: Adjustment, B: Bias>;
pub struct Ranged<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>;
```

Every exponent position is a type, corrected against `49:116` and `49:117`. `Implicit`'s own `E` and
`Ranged`'s `EMIN`/`EMAX` are both computed by `mulnum` (the exponent sum for `Implicit`, the bound sums
for `Ranged`) and both have to appear in the result numeral's type, which is the spine rule firing on
exactly the shape it was stated for. `Exponent` itself is `EZero | EPos<P> | ENeg<P>` over the sealed
`Pos`, sealed at birth, arrived at independently three times (`Bias`'s own sign shape, file 42; a signed
exponent, file 50; and the fold's own headroom arithmetic during an early wrong attempt at the negative
impl, file 50 section 4.1, repaired by separating magnitude difference from sign the same way `Bias`
already does). No const route survives: bare const arithmetic needs the forbidden `generic_const_exprs`;
`min_generic_const_args` refuses the shape outright ("complex const arguments must be placed inside of a
`const` block"); a `const { }` block refuses with the identical "generic parameters may not be used in
const operations" and asks for `generic_const_args`, which needs `-Znext-solver=globally`, mutually
exclusive with the rest of the arrangement per the workspace's own record. Every permitted door is closed,
compiled shut in both directions by files 50 and 54 (`50_probes/probe_3b`, `54_probes/probe_4b`).

`Radix` is now a sealed carrier rather than an open trait. `49:110`'s spelling (`type Radix: Radix;` with
"2 and 10 instantiated; any r expressible") admitted `R = 1` (collapses the exponent's whole grid family
into one grid) and `R = 0` (a zero quantum) as compiling instances that falsify the float model's own
founding sentence. `Rad<P>` is the sole constructor over the sealed `Pos`, bounded by `AtLeastTwo`
(covering `O<P>` and `I<P>`, excluding `H`, exhaustive by construction rather than by enumeration, since
`Pos` has exactly three constructors and `Pos` is sealed): every radix from two upward is expressible by
naming a `Pos`, and radix zero has no `Pos` spelling at all, while radix one refuses at the bound
(`H: AtLeastTwo is not satisfied`) rather than at some later arithmetic that would have produced a wrong
answer. Compiled, positive and negative, file 54 section 2.1.

`Bias` and `Adjustment` are signed, gcd-normalised rationals, value-unique and sealed, as ratified at
`44b`; every reference elsewhere in this document assumes that shape.

### 1.3 Encoding, nested inside Lowering

Unchanged from `49`. `Lowering` changes no value; `Encoding`, nested inside it, may change which datum
carries a value. No law may read `Lowering`; a law's key is a `const fn` parameter list and `Lowering` is
not a parameter. File 50 gives this split its sharpest vindication yet (section 1.16): addition is
commutative at the value level and not, on real silicon today, at the datum level, and the design already
forbids a law from reading past the canonical quotient, so the law is statable and correct without change.

### 1.4 The crossing contract

Three statements over the finite datum set of a numeral, unchanged in form from `49:161-168`:

1. `decode ∘ encode = id` on values, always.
2. `encode ∘ decode` is idempotent on data, always (canonicalisation).
3. `encode ∘ decode = id` on data iff the encoding is injective, a derived boolean.

Statement 3's derivation, stated since file 31 and never written down, is now written down and compiled
against the whole configuration matrix rather than against a single witness:

> The encoding is injective iff no value has two data. A second datum arises from, and only from: an
> unrepurposed signed zero; more than one reserved NaN datum; a cohort, meaning an unnormalised
> significand over more than one exponent with room to shift; and a cohort of zeros, present even at
> precision one under an unnormalised encoding. The two infinity data are always distinct values and
> never a source.

Checked exhaustively (file 54, `probe_2` and `probe_3`) over both radices, the whole `Specials` product
(section 1.16), both underflow modes, both cohort-selection rules, signed and unsigned domains, and with
and without a repurposed negative zero. It agrees everywhere. `Specials::INF` never touches the boolean
(infinities add values one for one with data); `Specials::NAN` always can (NaN adds one value for as many
data as the encoding reserves). Statement 3 is genuinely two-valued, not a polite way of writing "always
false": an unsigned, no-specials, normalised numeral is injective; so is the OFP8 `E4M3FNUZ` variant,
which repurposes the negative-zero datum as its NaN and is injective while still carrying a special
(compiled, 512 data, 256 live, 256 values). Before file 54, the boolean's only compiled witness in the
whole review was signed zero (file 30), which is why the statement had been exercised vacuously for
twenty-four files.

Decimal is where the boolean stops being a formality: at a small model width, 600 data, 559 distinct
values, statement 3 false. Section 1.17 states the whole finding; the cohort census alone is worth
carrying here, because the extremal case is not what a reader expects. At p = 3, q in [-2, 2], the largest
cohort (10 members, every exponent row times both signs) belongs to **zero**, the value with no digits at
all.

### 1.5 The quantiser

Round-first, classify second, unchanged in mechanism from `49:170-202`, and confirmed independently
against the machine rather than only against the standard's text: file 50's model, implementing the
statement below with no knowledge of IEEE beyond the format parameters, agrees with binary32 on
**41,380,159 operations** with zero mismatches (`probe_1`, add/multiply/divide, including 1,255
overflows-to-infinity and 884 subnormals). The struck overflow-band member ("every float operation",
`49:196-202`) is restored with a derivation rather than left struck, and the review's own
struck-versus-unknown distinction (file 50 section 9, adopted `53b`) is the reason it is restored rather
than the review continuing to treat "no derivation exists" and "answered in the negative" as the same
state.

For a `Ranged` numeral the quantiser gains one step in front of rounding: it selects the grid from the
exact value's own magnitude before proceeding exactly as an `Implicit` numeral's quantiser does.

> A `Ranged` numeral denotes the union, over `e` in `[EMIN, EMAX]`, of the grids with quantum
> `radix^(e - p + 1)` restricted to `[radix^e, radix^(e+1))`, together with the bottom grid extended down
> to zero when `Underflow = Gradual`, omitted when `Underflow = Abrupt`. Quantising selects the grid from
> the exact value's own magnitude, rounds on the selected grid extended upward without bound, then
> classifies against `[EMIN, EMAX]`, `Specials` and `Underflow`.

**The algebraic difference from `Implicit`, stated once because three separate results follow from it.**
An `Implicit` numeral's value set is an interval of a rank-one subgroup of the rationals, closed under
addition wherever the sum stays in range. A `Ranged` numeral's is a union of intervals of subgroups whose
generators form a geometric chain, and that union is **not a subgroup**: `1 + 2^-24` is not a binary32
value (the machine agrees, delivering `0x3f800000`). The overflow band is inhabited because the exact
result lies on a finer subgroup than the result's own quantum; the fold needs an exact accumulator because
in-range closure is what an in-format accumulator would have needed; associativity fails at the format
width and holds through the accumulator for the same reason. Three results, one sentence.

**The overflow band, closed form, two clauses rather than one.** The candidate closed form file 44 left as
owed (`q_result <= 2 * lattice`) was built and refuted by exhaustive enumeration (753/1000 addition,
639/1000 multiplication, both directions of error) before the correct two-clause form was found:

> **Lattice clause.** The band is empty unless some point of the exact-result lattice lies strictly inside
> `(max_r, max_r + q_r/2)`. Decidable from the three quanta alone by one Euclidean division for an
> operation whose exact results form a subgroup.
>
> **Reachability clause.** That point must be an actual exact result of two in-range operands.

Measured over 5,184 triples: the lattice clause alone has **zero under-predictions** for both addition and
multiplication, which is the useful direction for a build layer to act on (it never claims empty when the
band is inhabited). Its over-predictions are all reachability failures. Every prior band member the
review had stated maps onto one row of this table (file 50, section 3):

| case | exact-result lattice | q_r | band |
|---|---|---|---|
| fixed, same format, add | q | q | empty |
| fixed, same format, mul | q² | q | inhabited |
| fixed, mixed, dividing quanta, add | the finer quantum | the finer quantum | empty |
| fixed, mixed, non-dividing, add | gcd(q1, q2) | the finer quantum | inhabited |
| float, both operands in the top binade | the top quantum | the top quantum | empty |
| float, one operand three binades down | the finer quantum | the top quantum | inhabited |

Division has no row: its exact results are not lattice-valued at all, which is why file 43 had to compile
that member rather than derive it.

Dither and shaping stand exactly as `49:179-186` states them, untouched this stretch.

### 1.6 Membership and the number-system layer

Unchanged and untouched. D38/D39 (op) hold; membership licenses only the exact, widening operation
family, gated on `Specials = None`. File 39's finest-inhabited-system candidate reading remains a
candidate the review's own two-expert discipline has not yet given a second independent read, queued
again at every checkpoint since (`48b`, `53b`, `57b`) and still not run.

### 1.7 The algebra: what a law is, and the finest view it holds at

Unchanged from `49:216-254`. A law is a claim over a numeral's value set, quantified over its grouping
class, keyed on every parameter its proof used, derived by blanket construction and safe, or asserted with
`unsafe impl` (D16). The finest-view mechanism (op, `39b`) stands. `Growth` is not in the key (section
1.10 below settles it further, out of `Policy` entirely). `IS_EXACT` and `Total<Op>` together, not
`IS_EXACT` alone, trivialise an operation's grade monoid. A regrouping publishes exactly the grade
generator classes its law fails to preserve, as a type projection (section 1.13/1.14).

Untouched this stretch except for one convergence worth naming: file 50 section 4.3 found that the design's
own grade, a free commutative monoid over refusal causes and quantisation events joined by union, **is**
IEEE 754's sticky flag register, bit for bit, over the five clause-7 exceptions with no adaptation needed.
Section 1.14 states the full finding.

### 1.8 The fold: two conditions, two relations, and what the accumulator becomes for a float

Interior safety and total safety remain two distinct conditions serving two distinct promises, unchanged
from `49:255-265`. What is new is the accumulator's shape once the operand numeral is `Ranged`.

> A `Ranged` numeral's entire representable set is contained in the single grid of quantum
> `radix^(EMIN - p + 1)` bounded above by `radix^(EMAX + 1)`. The exact sum of `n` values is exactly
> representable in an **`Implicit`** numeral of that quantum and width `(EMAX + 1) - (EMIN - p + 1) +
> ceil(log2 n)`, and interior safety for a float fold is satisfiable at that width, by a numeral of the
> design's other kind.

Checked, not asserted (file 50 `probe_4`): 2,924,207 ordered triples exactly representable at the predicted
width; 139,721 orderings agreeing under every rotation and reversal; the same folds held in-format instead
show 23.17% of triples disagreeing under left- against right-association. At real formats the width is
large and finite: binary32 needs 277 bits plus `ceil(log2 n)` for a sum, 554 for a dot product; binary64
needs 2,098 and 4,196. The condition's statement is unchanged; only the sufficient-width formula changes.
The object this accumulator is is the one other fields call a quire or a long accumulator (recalled, not
verified against a source in that dispatch).

This is also a fourth reading of the growth-class question, agreeing with division: a float fold grows as
`ceil(log2 n)` plus a term that is `Theta(2^w)` in the exponent field width, the same exponential class
division's `Theta(2^p)` belongs to (`49:434-437`). The class is what appears whenever a field width indexes
an exponent, not a peculiarity of either operation.

### 1.9 The multiplicative half

Unchanged from `49:267-278`. `mul_full: N1 x N2 -> mulnum(N1, N2)`, the biased-product closure formula
built in full at the type level; Monotone, not full dioid, is the correct rung, and it is derived rather
than asserted; no shipped preset is a `(max, +)` dioid.

### 1.10 Widening and Growth: two axes removed, and the second now compiled shut

`Widening`'s three old instances decompose into three pre-existing mechanisms, ratified. `Growth` leaves
the key, ratified. **Whether `Growth` also leaves `Policy` entirely, the review's last open ratification
tick, is now closed**, by file 51, compiled in both directions rather than corroborated a fourth time.

**The positive enumeration.** Eleven operations drawn from the design's current surface (in-numeral add,
sub, mul, div; `mul_full`; `mulnum` over `Ranged`; `div_exact`; the `div_floor`/`rem` pair; `fold`,
`fold_sequential`, `fold_compensated`; `quantize`), each with its own growth trait generic over the operand
numeral type(s) alone. None takes a `Policy` parameter (`51_probes/probe_1`, `grep Policy` against the
file confirms zero hits outside comments).

**The structural theorem, which is the stronger of the two results.** Every operation the design has or
could design computes its result numeral inside one trait impl. For that impl's answer to vary "by policy"
without the parameter being inert, two impls disagreeing on the answer would have to coexist for the same
generic domain, and coherence refuses that outright (`E0119`), independent of which operation or which two
numerals. Probe 2 shows threading `Policy` into a growth trait's parameter list compiles and computes
nothing (the two policy instantiations force-unify identical); probe 3 shows the only way to make it live,
two conflicting impls, is refused before any question of correctness is even reached. So: not "checked
eleven operations, found none," but "no operation expressible in this type system's dispatch discipline
can have policy-dependent growth."

`Policy` carries `Quantisation` alone. The ratified table's `// Growth removed from the key: RATIFIED.
Removed from Policy entirely: OPEN (tick 3).` comment and the line it annotated both drop; section 1.22
carries the corrected declaration.

### 1.11 The value-unique encoding: ratified, sealed, priced, unchanged this stretch

Everything in `49:291-346` stands untouched: `Nat ::= Z | Pz<P>`, `Pos ::= H | O<P> | I<P>`,
`Adjustment`/`Bias` signed gcd-normalised rationals, `Int` dropped from the ratified table, the composition
wall's design rule ("every trait in a chain that reaches a consumer-facing signature either pattern-matches
on constructor heads or has finite, non-recursive obligations; `Reduce`, and anything routed through it,
never appears in such a chain"), priced at 1.55 ms/composition over the dyadic case every shipped numeral
uses. Nothing this stretch touched the encoding itself; section 1.23 extends its pricing and section 1.16
extends its scope to `Radix`.

### 1.12 The seal: carriers sealed, contracts open, and the seal as free diagnostic

The tower's own seal (`Pos`, `Nat`, `Adjustment`, `Bias`) stands exactly as `49:348-421` states it: closed
by enumeration over the four introduction routes, compiled and verified twice over (files 46 and 47).

**Four new carriers were born sealed rather than sealed after the fact, and all four refuse all four
routes** (`Radix`, `Specials`, `Underflow`, the signed `Exponent`; file 54 `probe_1` through `probe_1d`).
`Grade` and the fold's exponent machinery (files 48, 50) already were. The carrier-at-birth rule has now
been applied prospectively six times across this review, and every one took one pass at zero measured
cost, against the four passes the original tower needed before the checklist existed.

**The seal's own diagnostic, checked for the first time this stretch, is better than anything the review
had built on purpose.** rustc's own sealed-trait detection names the trait, explains in plain English why
a sealed trait exists ("to force you to use one of the provided types"), and lists the exhaustive
inhabitant set, unprompted, with no attribute anywhere in the tower (file 56, `probe_3`, reproduced fresh
against a rebuilt copy of the tower). This is stated in section "The two rules" above because it changes
how the rule should be read: sealing a carrier is simultaneously the correctness mechanism and the review's
best legibility result, not two separate concerns that happen to share a construction.

The honest limit on "closed" stands as file 52 restated it: the four-route enumeration is verified as
"every attack found lands in one of the four," not as "four is the whole space." Section 1.12's own
quantification block should carry that phrasing rather than the stronger reading.

### 1.13 Division: held, its float path now compiled

**Not adopted this stretch, held exactly as `44b` left it.** File 43's finding stands, the operation
surface waits until the rest of the algebra settles, and division stays a third growth class at
`Theta(2^p)` accumulator bits.

**Division's float cause split, named reasoned and awaiting the model at `49:456-458`, is now compiled and
agrees with the hardware.** `x/0` with `x` finite and nonzero delivers a correctly-signed infinity and
raises `divideByZero` only; `0/0` and `inf/inf` deliver a quiet NaN and raise `invalid`; `inf/0` delivers
infinity and raises nothing, since `divideByZero` is defined only on finite operands. The value half of
every case agrees with the machine on 300 class-level cases (file 50, `probe_6`); the cause half cannot be
observed on this toolchain at all (section 1.14 states why), which is the same conclusion the value/cause
split reaches independently for every other operation.

### 1.14 The grade is a type: projection, join, evaluation, and now the IEEE convergence and the licence

The published grade as a type projection stands as `49:464-546` states it: `Folded<<(... ) as
FoldGrade>::Out>`, the join algebra whole-matrix, `Grade` sealed at birth, the fused evaluation-strategy
block adopted verbatim (persona, `48b`). Untouched this stretch in its mechanism; two real extensions
landed on top of it.

**The grade is IEEE's flag word, and the design's own carrier is strictly the better one.** File 50
section 4.3: over the five clause-7 exceptions with no multiplicity, a free commutative monoid joined by
union **is** a five-bit word joined by bitwise or, and the design's code needed no adaptation to serve as
either. The two-part generator split lands exactly on the standard's own: inexact and underflow are
quantisation events (raised by the quantiser on a value it still delivers); invalid and divideByZero are
causes with no quantiser origin (raised on operands, before rounding); overflow is raised by the
classification step, which is the quantiser's second half. Two consequences. First, the design's grade
rides on the value rather than on a per-thread accumulator, which is strictly sounder under a pluggable
executor for the identical reason section 1.13's old short-circuit finding already gave: a per-thread
register is nondeterministic on unchanged data because the thread partition is the executor's choice, not
the value's. Second, and stronger: the standard's own carrier is not merely worse, it is **unavailable**.
A grep of the pinned toolchain's `rust-src` component for `fetestexcept`, `feclearexcept`, `fegetround`,
`fesetround` returns zero files; there is no FPCR access in `core::arch::aarch64`; `_mm_setcsr` on x86 has
been deprecated since 1.75.0. A design that wanted to mirror IEEE's flag mechanism could not read it. The
value-carried grade is not preferable, it is the only carrier that exists.

`Specials`' value half checks against the machine on all three operations, all combinations, 300 cases,
zero mismatches, with one real subtlety: finite plus finite is not decidable at the class level, since
exact cancellation delivers a zero of a different class, so the specials table is not a total function
from classes to classes. NaN payload propagation is silicon, on this target, and is not commutative at the
datum level (`qNaN(1) + qNaN(2)` and `qNaN(2) + qNaN(1)` differ in the low payload bit); it is commutative
at the value level, which is exactly the design's own already-ratified split between value and datum doing
its job. `Canonical` describes the default-NaN-on-no-NaN-operand behaviour rather than fixing it, since it
is architecture-dependent and unread by any law.

**A fixpoint's published grade is trip-count independent, and interior safety is not, unless the step
contracts.** File 55 section 2 checked idempotence of the grade lattice's join over the whole four-point
carrier at widths one through four, both associations, which nobody had checked when file 48 verified
commutativity, associativity, identity and absorption. A fixpoint's grade is `join(seed grade, step
grade)`, unaffected by how many steps ran. Interior safety cannot follow the same argument (the droplist's
"growing an accumulator's own type on every iteration cannot work in principle" stands): an unnormalised
accumulating iteration's arity is `trips * step_arity`, which needs `generic_const_exprs` to state as a
bound. What closes the gap is a property of the algorithm, not of the trip count: a step whose output range
is bounded by its input range (renormalisation) has a per-step bounded arity the capacity already covers,
and "this step renormalises" is not derivable from any numeral, so it is an `unsafe impl` under D16, the
first consumer-side asserted fact the review has found as against the operand-side facts D16 was written
for. A non-renormalising, data-driven trip count is given the arity `Unbounded` (new vocabulary, file 55's
own, needs two independent reads before it is treated as settled), which is not a `Pos`, so
`InteriorSafety<Unbounded>` coexists coherently with the `Pos` blanket with no specialisation of any kind.
The resulting top-of-lattice grade is not decorative: file 55 checked and found it separates a
sign-reading consumer (correct at `EventsTransferred`, matching `fiedler.rs`'s own doc comment that only
the sign pattern matters for `spectral_bisection`) from a magnitude-reading consumer, which is refused
against both a wrapping and a refusing solver.

**The reassociation licence exists, is on the stabilisation path, is safe, and over-grants by three
permissions beyond the one interior safety proves.** File 51: `f32::algebraic_add` and its `f64`/`f16`/
`f128` siblings (`#![feature(float_algebraic)]`, tracking issue #136469, an open stabilisation PR #157029),
`const fn`, no `unsafe`, per-call-site rather than a whole-compilation-unit flag, unlike the sibling
`fadd_fast` family (`unsafe fn`, UB-on-non-finite-input, explicitly stated never to reach stable). Measured:
an eight-element `f32` reduction written with `.algebraic_add()` vectorises to the identical two-instruction
shape the integer reduction already gets for free, licensed by LLVM IR flags `reassoc nsz arcp contract`,
with `nnan`/`ninf` conspicuously absent (the mechanism does not assume operands are finite, which the
design's own `Specials` axis and grade tracking require). Interior safety establishes exactly one of the
four permissions, `reassoc`. The other three need separate accounting: `nsz` is a fact about the target
numeral's own `Canonical` axis, not about interior safety, and must be discharged by reading it; `arcp`
(reciprocal approximation) is inert wherever a fold's interior does not divide, which today's surface never
does; `contract` (fusion into a single-rounding `fmadd`) is a genuinely separate substitution and is not
discharged by interior safety at all. Compiled witness: a chained `algebraic_add(algebraic_mul(...))`
reaches the identical fused value as `mul_add` on a pair where fused and separately-rounded results differ,
through a route that never spells `mul_add`. The design's own existing mechanism already starves `contract`
structurally: a MAC-shaped interior routes its multiply through `mul_full` into `mulnum(N1, N2)`, exact by
construction, wider than either operand's format, with no same-format rounding on the multiply side for
`contract` to fuse with. **`fold_compensated` must never receive this licence, and the reason is compiled
rather than argued.** The Kahan-style compensation step, `(sum + y) - sum - y`, is algebraically zero as a
real-number identity and numerically the exact bits lost when `y` was added to `sum`. `reassoc` treats
those readings as interchangeable, because algebraically they are, and the compiled result is that the
entire expression collapses to one instruction, `fsub s0, s1, s1`, always zero: the compensation term the
combinator exists to compute is optimised away to nothing. The scope boundary is exactly the one the
design's own grade projection already draws at the type level (`fold` against `fold_sequential` against
`fold_compensated`), and the licence gates on that distinction directly.

**The receipt this earns is designed, not built, and it stays that way on purpose.** Four clauses (file 51
section 2.4): the call site's monomorphised type carries a closed, constructor-headed `FoldGrowth`
projection showing no interior quantisation; the target numeral's `Canonical` fact matches the `nsz` grant;
the interior contains no adjacent same-format multiply for `contract` to fuse; the combinator is `fold`,
never `fold_compensated`. This joins the sibling hardware-lowering receipt (section 1.21) as contract text
owed to a build layer that does not exist. Torvalds' own caution (file 53 section 5) stands: keep every
word of it, and do not start implementing receipt plumbing inside arvo ahead of a real consumer, since
`49:707-708` already states arvo grows no build harness of its own.

**`float_algebraic`'s vetting is one read deep of the two the workspace's own discipline requires.** File
51 read the tracking issue and stabilisation PR directly and found no soundness concern, an open PR, and a
motivating case (an 8x dot-product slowdown) that is this design's own problem stated in someone else's
words. It records this as one member's reading, not a ruling, per the workspace's two-expert vetting
convention. The second independent read is still owed (queued, `53b`, not run by any file through 57).

### 1.15 The exponent forces the spine rule open a second time, and closes with the second read op asked for

`49:552-563` derived, reasoned only, that the exact-widening family's own `Ranged` maps compute exponent
bounds that must appear in the result numeral's type. **This is now compiled, twice over, for both
exponent positions the design has.** File 50 compiled `mulnum` over two `Ranged` numerals; file 54
compiled `mulnum` over two `Implicit` numerals, closing the second half file 50 had left open ("it is not
met on whether the `Implicit` numeral's single exponent should move to a type at the same time, which I
did not test," `50:602-604`). File 54's own compile overturns file 36's earlier claim that `E` is "never
arithmetic in a way the wall blocks" (`36:446-447`): `E1 + E2` is exactly the arithmetic the wall blocks,
and it sits in the exact-widening family's own result type. Section 1.2 states the corrected declaration
line; both exponents are ratified-table edits, flagged for op alongside the `Int` drop (section 2).

Both compiles used the same instrument, the constructor-sign shape `Bias` already established, applied a
third and fourth time. That is the argument section 1.11's `Int` drop already made about a future signed
exponent consumer: it does not consume `Int`, and now it demonstrably does not.

### 1.16 The float model: absorbed with no new mechanism, and the two axes it populates

**What the float model needed, in the design's own vocabulary.** A `Ranged` numeral is a finite union of
`Implicit` grids indexed by an exponent interval; the quantiser (section 1.5) gains a grid-selection step;
the identity contract, the value-unique encoding, the grade, the fold's sufficiency condition (section
1.8), the overflow band, division's cause split, and the laws under the canonical quotient all absorb it
with no restatement. That is the honest summary file 50 reaches after building it: forty-nine files of
machinery absorb the float half of the design with nothing new required, and the two genuine additions are
axis populations the table already reserved.

**`Underflow`.** Two instances on `Numeral`: `Gradual` (extends the bottom grid to zero, changes the
representable set) and `Abrupt` (leaves a hole between zero and the smallest normal, an underflowing
result refuses rather than delivers a value; changes representability and totality together). Flush-to-zero
moves out of `Numeral` entirely, into a `Quantisation` resolution: it changes no representable set at all,
zero is already in it, and it is a rule about what an operation delivers, applied after rounding, which is
the same mathematical-versus-encoding split D69 already made one level down (adopted, file 50, confirmed
sound, `53b`). Tininess-detected-before against tininess-detected-after (IEEE's own permitted fork) differs
only in whether an unreadable flag fires; it is recorded as a degree of freedom the design deliberately
does not expose rather than silently resolved.

**`Underflow = Abrupt` under an unnormalised encoding is the first axis found that constrains `Encoding`
rather than staying independent of it.** Under a normalised encoding, realising `Abrupt` is free (the
subnormal row's data become non-data, `decode` was already partial there). Under any radix above two
(section 1.17, every radix above two stores its significand unnormalised) there is no subnormal row to
remove; realising `Abrupt` means declaring a contiguous datum region reserved, a real obligation on
`Encoding::Fields` the `Numeral` axis alone does not carry. Compiled for the normalised case (file 54,
`probe_2`); reasoned only for the unnormalised case.

**`Specials` is a product of two independent facts, not a three-instance chain.** File 50's original
proposal (none, infinities-only, IEEE) was demanded a witness for its middle rung (`53:236-243`) and got
answered by moving the axis instead: infinity presence and NaN presence are independent in shipping
formats, and the OCP 8-bit Floating Point Specification's `E4M3` is the witness the chain could not name
(no infinities, NaN only, the freed exponent code spent to raise `emax`). The corrected axis is the
four-point product:

| instance | INF | NAN | witness |
|---|---|---|---|
| `NoSpecials` | no | no | every fixed-point numeral; every integer type |
| `NanOnly` | no | yes | OCP OFP8 `E4M3`, and its `FNUZ` variant |
| `InfOnly` | yes | no | none found. grounded `unknown` |
| `IeeeSpecials` | yes | yes | binary32, binary64, decimal64, OFP8 `E5M2` |

The `InfOnly` witness demand is answered honestly rather than filled with a plausible sentence: nobody
found a format with infinity and no NaN, the row costs nothing to declare, and it carries `unknown` per the
grounding registry's own slot (section 1.19) rather than being deleted or guessed at. Signalling NaN stays
off the axis; reading one is an operation, and the grade already carries what an operation raises.

**The overflow band's closed form, division's cause split, and the accumulator sufficiency formula are
stated in sections 1.5, 1.8, and 1.13 respectively rather than repeated here.**

**`TotalOrd`'s level fork is answered, not merely stated.** The NaN payload measurements decide it: any
total order that places NaN consistently is datum-level if it distinguishes payloads (IEEE's own
`totalOrder` predicate does, and is therefore forbidden to laws) and value-level if it does not. The design
ships a value-level `TotalOrd` placing one NaN class, usable by laws, and names `totalOrder` as a separate,
non-law-usable, datum-level predicate. This answer is not yet built into the algorithm crates' own bounds,
where file 55 found the stakes are considerably higher than "a one-sentence fork" suggested (section 1.20);
building both readings and checking which the design's own consumers actually need is queued (`57b`,
section 4).

**Hardware-float lowering is not a `Lowering` under the design's own already-ratified definition unless the
environment is pinned, and the strategy axis now has a number to spend on the choice.** Section 1.21 states
this contract and the measured trade together; it is repeated there rather than here because it is where
the design's own toolbox rule (never police, always expose the choice) decides the shape.

### 1.17 Radix ten: the chain, the section, and where the design and the standard part company

**The chain, of which only the first link is about the radix.** Radix two normalises for free: a
normalised binary significand's leading digit is always one, so it need not be stored, and the hidden-bit
trick both enforces normalisation and costs nothing. No radix above two has a constant leading digit to
hide, so its significand is stored unnormalised, and a value has one datum per representable exponent
shift: a cohort. The remaining three links (unnormalised storage, the hidden digit's presence or absence,
the preferred-cohort choice) are already named by the design's own axes, `Encoding::Fields` and
`Encoding::Canonical`. Nothing new is needed, and that is the finding: the design was built to express this
before anyone checked that it did.

**Measured, radix ten, p = 2, e in [0, 2]:** 600 data, 600 live (every datum is live under the unnormalised
encoding, since there is no reserved significand band the way a normalised counterfactual would have 240 of
its 800 data dead), 559 distinct values, statement 3 false and predicted false. The value sets of the
normalised and unnormalised counterfactuals are **identical**: normalising a decimal numeral changes no
value, which means cohorts are a choice, not forced by the value set, and the design has to know whose
choice it is.

**`Encoding::Canonical` is a genuine choice under radix ten, and it is a formality under radix two.** Two
natural cohort-selection rules (smallest significand with the largest exponent; largest significand with
the smallest exponent) are the same function under radix two with a hidden digit (compiled, file 54's own
control), and different functions on the identical value set under radix ten, with a named witness: the
value 1 spells as both `1 x 10^0` and `10 x 10^-1`, and the two rules pick different data that both decode
back to 1.

**Non-canonical codes are a third, larger source of non-injectivity than cohorts, and live entirely on the
`Encoding` side.** Repacking a decimal significand is a bijection, so BID against DPD cannot itself change
any of the three statements. What is interesting is that a field wide enough to hold `10^p - 1` also holds
codes above it, which the standard reads as zero: compiled at a seven-bit significand field, p = 2, 209 of
768 data are redundant, against 41 of 600 in the tight encoding.

**Where the design and the standard genuinely part company, and it is the single most important sentence
this stretch produced.** IEEE 754 specifies, per operation, a preferred exponent for decimal results: which
cohort member an operation delivers, as a function of the operation and its operands' exponents rather than
of the result's value. The design's operations are value-valued; there is no place in that pipeline for an
operation to choose a datum, and `Canonical` cannot express the rule because `Canonical` is a function of
the value alone. Two rejected responses (carry the cohort member in the value coordinates, which falsifies
the founding sentence that a value is a rational; make operations datum-valued for decimal, which evaporates
the algebra for one radix) leave a third that is not a concession: **arvo's decimal `Ranged` numerals
deliver IEEE's values and are not conformant to its preferred-exponent rules; a consumer for whom the
quantum is part of the number uses a decimal `Implicit` numeral, where the exponent is a type, checked at
compile time, and cannot drift through an arithmetic chain.** That is strictly stronger than the standard's
own rule, because it is checked rather than propagated at runtime, and it is unavailable to a language with
only runtime decimals. Compiled in support: a single-exponent-row decimal `Implicit` numeral has no cohort
at all beyond the signed zero, and dropping to a non-negative domain makes it genuinely injective.

**The radix axis pays for itself, and the margin is a wall, not a percentage.** Absorbing a decimal
quantum into the rational adjustment (folding `A * radix^E` into one rational, which would make `Exponent`
redundant if it were free) is not merely expensive; it does not compile at any real decimal format's
exponent range. Two independent walls, both compiled: a `u64` readout ceiling at `10^20` (`Pos::VAL`
cannot be read past it), and a type wall at depth 130 (`Pos`'s own structural recursion against the default
recursion limit, attributed by an independent test to `Pos` itself rather than to the reduction machinery,
since the identical depth with no `Gcd` anywhere refuses identically). decimal64's own bottom grid
(exponent -398) compiles at **64 ms**, flat, in the radix-and-exponent spelling, from a 519-byte source;
the absorbed spelling does not compile at all, from a 4,486-byte source. `Exponent` is not a redundant axis;
it and `Radix` are what keep a decimal numeral off the cliff.

**The general fact:** a `Pos` may not exceed roughly `2^127` on any axis; a magnitude that would need more
is expressed as an exponent, never absorbed into a rational. Checked against every magnitude the design
actually spells (precision, exponent bounds, radix, MATLAB constants, the design's own division constants):
all comfortable. The `u64` readout ceiling at `2^63` is the tighter and more consumer-visible of the two,
and whether it should widen is an open question (section 4).

### 1.18 The numeral notation: the macro, the face, and the ceiling

**Intent ratified at `48b`, unchanged: a consumer writes any number as a literal, unbounded range, emitted
constructors, zero table.** The vehicle question splits in two, and file 56 answers the half nobody had
tested.

**Legibility and structural sealing pull in opposite directions, for the reason the whole tower exists.**
Values-as-types buys bound-level refusal (a malformed value has no type to inhabit); printing a
values-as-types nest is what makes a diagnostic illegible. A const-struct face over `adt_const_params`
(`Spec { precision, bias_num, bias_den }`, compiled, file 56 section 4.1) is a genuinely strong legibility
result, printing negative fields with no special handling and scaling past a single integer, and it
**cannot be structurally sealed**: the spine rule reaches the notation layer itself (the first firing
outside the numeral tower's own recursive expansion), because the condition that makes a `Spec` well-formed
is computed from a generic const and needs to appear in a bound, which is the identical wall closed
elsewhere by making the quantity a type. A const-struct face has no equivalent move available to it; a
bad `Spec` compiles clean unless something forces the checking constructor to run, and nothing forces it
by default.

**The resolution is a second representation connected by a trusted mapping, not one representation trying
to do both jobs: the smart-constructor pattern.** The macro is the single trusted entry point; it sees the
literal digits at expansion time, concretely, not generically. It mints both the raw constructor chain
(already proposed) and a concrete, non-generic newtype face with no public constructor of its own. There is
no sealing question left to ask for the face, because there is no attacker position: nothing outside the
trusted emitter can mint a second, malformed inhabitant, the same reason the tower's own constructors need
no separate attacker check once sealed.

**Legibility surviving past declaration is a priced, per-operation obligation, not a free consequence of
the notation layer.** File 56 compiled both readings of file 04's own residue: an operation generic over
the raw encoding decays the face to the fully-expanded nest one hop in (Shape 1, the default today); an
operation defined on the face itself, with its own computed result face, survives (Shape 2). The
checkpoint's ceiling, "the first error in an expression names the face," is achievable exactly as far as
the operation surface is re-derived at the face layer, which is a real cost (doubling relevant trait
surface) that nobody has priced.

**The decoder ring is a confirmed ceiling, not an open item to keep chasing.** A raw type-equality
mismatch (E0308) always prints the fully-expanded alias, for a hand-written alias or a macro-emitted one
alike; this has now recurred, unflagged, in a different member's own probe (file 55's own
`probe_4b`, using readable names `P8Public`/`P10Public` that never surface, worse than file 47's original
witness because the fully-qualified module path is printed for every constructor). The one lever that
moves it is not a diagnostic attribute; it is restating the comparison as a bound (E0277) rather than an
equality, which combined with a concrete face produces the strongest diagnostic message this whole review
has found:

```
error[E0277]: expected accumulator width `Q37`, this one is `Q53`
help: the trait `SameFaceAs<Q37>` is not implemented for `Q53`
```

Where a signature genuinely needs exact-type identity (a fold's declared accumulator, a division's declared
result), the decoder ring is the ceiling and no instrument found moves it.

**`#[diagnostic::on_unimplemented]` does not reach a solver-overflow diagnostic (E0275) at all, confirmed
by direct annotation of the real, ratified `Reduce` trait, byte-identical before and after, on both an
abstract operand and a concrete rigid non-inhabitant.** This closes both residuals the consolidation had
carried as untried (`49:839-842`'s twenty-minute challenge and `49:868-871`'s rigid-versus-abstract
question): the answer to both is no, not with this instrument, on either shape. What actually answers the
residual is not a diagnostic fix; it is the design's own already-ratified architectural avoidance (`Reduce`
never reaches a consumer-facing signature), which is what the review spent two files closing off and which
already keeps the bad E0275 out of a consumer's own path.

**Priced.** The face's own seal check costs 0.127 to 0.178 ms/item, one to two orders of magnitude below
the tower's own composition cost. The face layer is cheap; the tower underneath, priced in section 1.23,
remains the whole of the compile-cost story.

**The recommendation for the macro's vehicle**, stated so a consolidation can take it directly: the macro
emits, per invocation, both the concrete constructor chain and a concrete, non-generic newtype implementing
a bridge trait to it, with no public constructor of its own. Operations that want legibility past
declaration define a face-level sibling; operations that do not, decay honestly at that point. Whether the
declarative-macro-versus-proc-macro question inside that shape is still sketch-decidable, per `48b`; the
sketch itself has not run.

### 1.19 Claim provenance: the grounding registry, now with a third ground state

Adopted at `44b`, widened by file 45 to four kinds of ground (ratified decisions, settled shapes, physical
grounds, tree grounds), operated successfully on the grade's own move from const to type (file 48). File
57 found a fifth kind of gap the four-kind vocabulary could not name: a claim derived once, whose
derivation can no longer be rebuilt from the committed trail. Not "never derived" (which strikes), and not
silently kept as though nothing happened (which is exactly what the grounding field exists to expose).

| kind | rung | examples |
|---|---|---|
| ratified decisions | op-ratified, governing | `d69`, `vu`, `enc`, `seal-owed`, `div-held`, `grounding` |
| settled shapes | panel-settled, presumed correct, overturnable with evidence | `round-first`, `crossing`, `bias-rational` |
| physical grounds | facts about the environment, change by act not argument | `pin`, `host`, `flags`, `model`, `ffl` |
| tree grounds | facts about the shipped source at a commit | `tree` |
| unreproducible | derived once, derivation not rebuildable from the committed trail | file 8's five-shape instruction table |

**Adopted, `unreproducible`.** File 57 found file 8's own probe module (`spare.rs`/`fusion.rs`, imported by
`08_probes/e_codegen.rs` via `use union::*;`) missing from the panel directory entirely; the five-shape
instruction-count table it produced cannot currently be reproduced by anyone from what is committed. This
is neither a confirmation nor a refutation, a distinct third state, and file 57's own restraint is ratified
as convention alongside the ground: nobody reconstructs a missing artifact and presents it as a
reproduction of someone else's build. A fresh derivation under a new member's own name, or nothing.

**One dependent forces more, and it is not yet resolved.** The same file 8 probe run produced the
const-eval width ceiling (28.45 seconds at eight bits, refusal at nine), which `unstable-features.md`, a
ratified workspace rule, cites as part of the argument that model-width validation is the only form
available and that the `specialization`/`TypeId` bans are verification infrastructure. Whether that
specific citation's own probes rebuild from the committed trail (a different artifact from the five-shape
table's, and the sweep generator is committed) has not been checked. A workspace rule resting on a
possibly-unreproducible measurement is not a state to leave standing; this is queued, not closed (section
4).

The convention's own perimeter stands as file 45 stated it: no tier detects an unwritten grounding, and the
residual is caught only by the act of writing the field, the same act that catches a stale one. The two
tiers named and not built (a mockspace registry namespace; a probe-header line) stand unbuilt.

### 1.20 The algorithm crates: what the design's oldest real consumers need, and what they get today

File 04's `Precise`-exile question, carried forward at every consolidation since as "still unanswered" and
named at `53b` the oldest open consumer question in the review, is answered this stretch, and the answer
inverts the question's own framing.

**The exile was never the problem. The admission is.** `upward_rank` returns `C::Array<W>`, the operand
numeral, claiming an exactness it does not have. On a four-node chain the true answer is `[400, 300, 200,
100]`; there is no correct expected value expressible against `UFixed<8, 0, Hot>` (`E0080`, the literal
does not fit). Widened to a type that can hold the comparison, `Hot` returns `[144, 44, 200, 100]` and, on
two independent chains totalling 400 and 210, ranks the longer path at 144 and the shorter at 210: the
ordering **inverts**, silently, on a graph hilavitkutin reads for plan-stage DAG analysis. `Precise` returns
`[255, 255, 200, 100]`, wrong in value but never inverted, only degraded to a tie. File 04's three options
(accept the exile, panic, bifurcate the crates) all took for granted that the crates were correct for the
presets they admit today. They are not.

**The design's own answer, already used everywhere else, had simply never been applied to a fold-shaped
algorithm.** `mul_full`, the MAC accumulator, and `div_exact` all compute a result numeral rather than
reusing the operand's. A fold-shaped algorithm's result numeral is `foldnum(W, A)` (new vocabulary, file
55's own, needs two independent reads), carrying `W`'s precision plus `ceil(log2 A)`; a numeral claiming
the operand's own exactness for a computed sum is claiming something it does not have. Compiled at three
widths, with a negative control confirming the projected return type is checked rather than inferred.

**The arity is the container's capacity, already in the signature, and `Capacity` owes it a `Pos` face
alongside its array-length const.** No simple path in a DAG on `C` nodes visits more than `C` of them, so
`foldnum`'s arity is bounded by exactly what every one of these functions already carries. Stating that
obligation against `Capacity`'s current `const CAP: Cap` fails identically to every other spine-rule
firing, and the fix is the identical instrument: a `Pos` face, held in agreement with the `usize`
array-length spelling by a forced const assertion firing at use (`Capacity::filled`/`from_fn`, every entry
point) rather than at declaration. `Pos` has no zero, a real narrowing of `Capacity`'s domain, flagged and
not resolved.

**Only three of the four named crates are numeric consumers at all, and one of the three carries the exact
same defect a second time.** `arvo-sparse` has no numeric contract anywhere in it (`W: Copy` and `W: Copy +
Default` are its only bounds; `rcm_reorder`, `block_diagonal`, `dulmage_mendelsohn` are structural,
computing on adjacency bit patterns, never on a stored value), which is why the droplist's `AddAssoc` entry
names three crates and not four, a deliberate distinction rather than an omission. `arvo-comb`'s `bin_pack`
carries `upward_rank`'s identical defect: wrapping over-fills a bin (too much work in one fiber grouping),
saturating under-fills (closes early), neither announced.

**`Monotone<Add>`, not `AddAssoc`, is the atom the ordering-returning algorithms actually need, connecting
file 33's derivation to the droplist entry it was reaching past.** Compiled: wrapping addition is not
monotone (`200 + 200 = 144 < 200`), which is precisely why `Hot`'s ordering inverted; saturating addition is
monotone, which is why `Precise` degraded but never inverted. Two named entry points rather than one gate,
the fold-beside-fold_sequential idiom applied a third time: the value-returning door (the widened `foldnum`
result) needs no monotonicity at all; the ordering-returning door (returning in the operand numeral) needs
`Monotone` to keep its ordering honest. The idempotent-semiring rung file 33 already found empty stays
empty until the float model's `Specials` lands as a real numeral, at which point `longest_path`'s
ground-roots-at-own-weight workaround and `matrix_chain_dp`'s whole parallel `Bool` reachability matrix,
both hand-rolled substitutes for a missing annihilator, are deleted rather than extended: a rare case in
this review of the design removing a mechanism instead of adding one.

**`FromConstant` is a partial map declared total, and it has a live defect in the shipped tree.** Section 3
carries the finding and the repair as a live-defect-registry entry; the general lever it confirms belongs
here: wherever a numeral mismatch can be expressed as a bound rather than an equality, the error is readable
for free, independently found by file 47 and file 55 (and, differently, by file 56's face-plus-bound
combination).

**What is not settled.** The `TotalOrd` fork (section 1.16 states the answer; it is not yet built into
these crates' own bounds, and file 55 raised the stakes: if `TotalOrd` is datum-level, none of these four
crates' outputs is a law-expressible claim at all). Whether the widened result numeral is the right default
for a consumer with many nodes and narrow weights, who pays the widening forever in storage; a proposal, not
a ruling. `foldnum` and `Unbounded` (section 1.14), both new vocabulary needing two independent reads before
either is treated as settled.

### 1.21 The strategy door: hardware lowering, software quantiser, and the measured trade

**A hardware-float lowering is not a `Lowering` under the design's own ratified definition unless the
environment is pinned, which is a derivation from `49:151` rather than a new rule.** `Lowering` changes no
value; flush-to-zero turns a subnormal into a zero, a different value, measured (file 50, `probe_5`,
FPCR read and set through inline assembly, `1.0/3.0` under the entry mode against a non-default rounding
mode differ in the low bit, `MIN_POSITIVE * 0.5` differs by the whole value under FZ). A const-folded float
expression and the identical runtime expression can disagree in value, underflow behaviour, and datum, and
nothing in the type system sees any of it. So lowering a `Ranged` operation to hardware, in an environment
whose control state is unpinned, violates an invariant the design already ratified; it does not need a new
rule to forbid it.

**The default lowering of a `Ranged` numeral's operations is therefore the software quantiser.** A
hardware-float lowering is opt-in, carries a declared environment requirement (rounding direction, FZ,
DAZ, and their x86 equivalents) in a build-layer receipt, and is invalidated process-wide by any code that
writes the FP control register. arvo can offer one cheap, checkable door (a debug-build assertion comparing
live control state against the declared one, three instructions of cfg-gated inline assembly, Kind 1
structural lowering under the always-optimal-internals rule) and should.

**This is a strategy-marker decision, not a global default, per the toolbox rule the design already
carries, and the persona checkpoint adopted the mechanism on exactly that ground before the number
existed.** `Hot`'s whole identity is hardware semantics at hardware speed; a blanket software default would
quietly pin every `Hot` float consumer to a cost a policer, not a toolbox, decides. The strategy marker
selects between two legitimate doors, never between semantics and none. **Presumptive per-preset
assignment, reasoned rather than compiled**: `Hot` carries the receipt-carrying hardware lowering; `Warm`
(the default) and `Precise` carry the software quantiser, since the default must not quietly change
semantics; `Cold` follows the semantics-first side unless shown otherwise.

**The number the decision needed now exists.** File 57 built and ran the bench file 50 section 7 deferred,
in `mock/benches/` under the bench-harness discipline, per `bench-and-sketch-discipline.md`, with the
subnormal fraction of the input swept from 0 to 100 percent, correctness checked bit-for-bit against native
`+` over 98,304 operations, zero mismatches, before any timing was trusted:

| PCT subnormal | software ns/op | hardware ns/op | ratio |
|---|---|---|---|
| 0 | 19.82 | 1.20 | 16.5x |
| 25 | 19.84 | 1.19 | 16.7x |
| 50 | 17.40 | 1.41 | 12.4x |
| 100 | 15.85 | 1.20 | 13.2x |

The software quantiser costs **13x to 17x** a native `fadd` on this target, at every point of the sweep.
Apple silicon shows no subnormal cliff on the hardware side, confirming file 50's own stated guess rather
than leaving it unchecked: the historical x86 subnormal microcode trap does not apply here, and the usual
argument for flush-to-zero does not transfer to this target. The software side's own per-op cost falls as
the subnormal fraction rises (19.8 down to 15.8 ns/op), reported as a measured pattern with an unverified
hypothesis attached (branch-predictor friendliness on a uniform bottom-grid path), not as a finding.

**What remains to build.** The mechanism and the number both exist; the per-preset table stays reasoned
rather than ratified until a member threads `S: Strategy` through the lowering choice itself and compiles
it, queued first in the direction for the next four (`57b`). The radix-ten quantiser is unmeasured and
unbuilt (the long-division kernel's shift alignment is radix-two-shaped and needs its own repair).

### 1.22 The assembled trait table, and what it costs to build against the tree

```rust
// Every member that denotes a number is drawn from one value-unique, sealed,
// type-level encoding, sealed and attacked on every introduction route (1.11, 1.12):
//   Nat ::= Z | Pz<P>            P: Pos       precision, widths, exponent bounds
//   Pos ::= H | O<P> | I<P>      P: Pos       magnitudes
//   Bias ::= BZero | BPos<N, D> | BNeg<N, D>  N, D: Pos, N: Gcd<D, Out = H>   signed rational
//   Exponent ::= EZero | EPos<P> | ENeg<P>    P: Pos      signed exponent, sealed (1.15)
//   Radix ::= Rad<P>             P: AtLeastTwo   sole constructor, sealed (1.2)

pub const trait Numeral {                 // ratified: identity contract
    type Radix:     Radix;                // Rad<P>, sealed
    type Precision: Precision;            // a Nat; primitive (D69)
    type Exponent:  ExponentForm;         // Implicit<E: Exponent, A: Adjustment, B: Bias> |
                                          //   Ranged<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>
    type Domain:    SignDomain;           // a value fact
}

pub const trait Policy {
    type Quantisation: Quantisation;      // Growth removed from Policy entirely: RATIFIED (1.10)
}

pub const trait Lowering {
    type Encoding:    Encoding;           // SignIndexing, Fields, Canonical
    type StoredWidth: StoredWidth;        // a Nat, same encoding
    type Layout:      StorageLayout;
    // Widening removed: RATIFIED.
}

pub const trait Underflow { /* Gradual | Abrupt, sealed, both change representability (1.16) */ }
pub const trait Specials  { /* the product {NoSpecials, NanOnly, InfOnly, IeeeSpecials}, sealed (1.16) */ }
```

`Int` is dropped, per `49:337-346`; nothing above names it. `Radix` is spelled `Rad<P>`, corrected against
`49:110`'s open-trait wording, per section 1.2 above and this document's own table-diff pass. Both
`Implicit`'s `E` and `Ranged`'s `EMIN`/`EMAX` are typed, corrected against `49:116` and `49:117`. Rewrite
cost against the shipped tree remains near zero for the numeral tower itself: no shipped source names
`Adjustment`, `Numeral`, `Bias`, `FullRange`, `UTerm` or `AddWidth` (verified fresh, this document's own
opening paragraph). The shipped `arvo-graph`, `arvo-comb`, `arvo-spectral`, and `arvo/src/traits/
from_constant.rs`, by contrast, are real, and section 3 states their defects.

### 1.23 The cost model, printed rather than left in a CSV

**Adopted as spec text, correcting `49`'s own "harder, more realistic case" wording, which had realism
backwards.** The cost has two terms: a marginal cost per distinct composition, and a smaller marginal cost
per repeated site of an already-instantiated composition, both linear throughout, both measured
independently twice (file 53's original sweep, file 54's separately written generator reproducing both
anchors within 0.2 ms).

| profile | marginal cost per distinct composition |
|---|---|
| dyadic (every shipped fixed-point numeral's shape) | ~2.1 to 2.3 ms |
| decimal, unit numerator (currency, sensor scale, the whole decimal fixed-point use case) | ~6.8 to 14.5 ms |
| decimal, two-digit numerator, small denominator | ~21.0 ms |
| decimal, wide numerator and denominator | ~78.9 ms |
| 16-bit random rational pair (arbitrary MATLAB-import slope/bias) | ~143 ms |
| repeated site of an already-instantiated 16-bit composition | ~28 ms |

**The realistic profiles sit two orders of magnitude below the sweep the review had been calling
"realistic."** The 16-bit random-rational sweep multiplies two large co-random magnitudes, forcing maximal
Stein-gcd work; it is the adversarial worst case, not a representative one. The design's own three named
division constants (44100, 48000, 4096) cost five milliseconds together. hilavitkutin's own twenty dyadic
sites cost under a tenth of a second. These are compile-once, per-declaring-crate costs, re-paid on each
edit of that crate; at these profiles the inner loop does not notice.

**The numerator, not the denominator, is the term an importer actually controls, and it dominates.**
File 54's own decimal sweep: unit numerators over denominators to `10^9` cost 14.5 ms; two-digit
numerators over denominators only to `10^5` cost 21.0 ms, more expensive despite the smaller denominator.
Wide magnitudes on both sides reach 79 ms, more than half the worst named cliff.

**The cliff has a name, and it should be printed rather than left in a CSV.** One hundred distinct
arbitrary 16-bit rational compositions cost **14.3 seconds**; four hundred cost **63.7 seconds**. That
figure sat in a committed CSV for twelve files while the prose quoted only the per-unit rate; the fix is a
number a reader feels, printed here. The profile that pays is a code generator importing a MATLAB
fixed-point model with many distinct per-signal slope/bias pairs, which is the axis's stated reason for
existing. The toolbox rule's answer is a documented tradeoff, not a policed one: the consumer who chooses
that profile does it with the number in hand. Whether the per-composition verification cost can be made
cheaper for that specific bulk-import profile is on the open list as an attempt, per the novelty posture,
not as an accepted limitation.

**Scope, stated honestly.** This prices the bias/adjustment composition machinery, the only part of the
design compiled in full at width. The grade projection, the exponent sums, and the notation macro's face
check are separately priced by their own sections as cheap at the single-composition grain (0.1 to 0.2
ms), and none has been priced at aggregate scale; the linearity result here predicts they are, and
prediction is not measurement. A real-consumer compile-cost bench remains open (section 4).

### 1.24 The downstream contract, and the crate table

Unchanged from file 26 through file 49, extended by two receipt families this stretch. arvo grows no build
harness of its own; a build layer reads every axis, acts freely on `Lowering`, acts on `Policy` only inside
its own declared envelope, and never acts on `Numeral`. The post-monomorphisation verifier, the per-axis
liveness check, the fold-detection assertion, the layout-assertion precedent, the build-layer receipt
requirement, and the three ways to cross Stage G stand exactly as file 26 recorded them.

**Two receipt families now sit beside them, both spec text, neither built, both stated as owed to a build
layer rather than to arvo itself.** The hardware-float-lowering receipt (section 1.21: the declared control
state, invalidated process-wide by any code writing the FP control register) and the reassociation-licence
receipt (section 1.14: the four discharge clauses gating `algebraic_add`/`algebraic_mul` emission). Both
extend the same mechanism `49:706-713` already kept; `49:707-708`'s "unchanged by every deliverable in this
stretch" is stale in the harmless direction and is corrected here.

The six-crate split stands as packaging, unchanged.

## 2. The lead designer's calls

**Op's four checkpoints, restated, unchanged from `49`.** D69 ratified: identity is parameterised in
mathematical coordinates, not encoding coordinates (`30b`). D39 held, not overturned: membership through
algebraic structure stays a decision pending a positive characterisation of its honest content (`30b`).
The novelty posture (`34b`): attempt what looks unsolvable, distinguish "cannot, because impossible" from
"cannot, because nobody has done it," and treat the second as an absence to fill rather than a wall.
Widening leaves `Lowering`, `Growth` leaves the law key, and the finest-view mechanism replaces the
three-relation fork, all three ratified (`39b`). The value-unique encoding ratified in full, division held
for a later stretch, and every claim carries what it is grounded on, backfilled across the existing
consolidation (`44b`). The convergence directive (`30b`) and the novelty posture (`34b`) both hold
unchanged through every checkpoint since, restated in the same words each time: the intent outranks every
instruction, is vague on purpose, and only op's calls are final, and even those go stale.

**The persona checkpoints, three, made overnight, each explicitly not op's.** All three carry the same
provenance statement: op delegated the checkpoint mechanism, at Fable tier, by explicit instruction
recorded first at `48b`, and every call inside all three is persona-decided, dying the moment op reads it
and says otherwise.

**`48b`, after file 48, five calls.** The grade projection adopted as spec shape, with the projection-chain
constraint, the seal, and the join algebra as part of the adopted shape rather than follow-ups (section
1.14). The numeral spelling: the digit-emitting macro, the bounded table rejected on principle as a
hardcoded threshold of exactly the kind the workspace's own toolbox rule forbids (section 1.18); the
vehicle inside that intent left sketch-decidable. The evaluation sentence adopted as the fused block
(section 1.14), with the per-application-against-per-value-moved event-counting sub-item declined as
genuinely op's. `Int` dropped from the ratified table, flagged loudest for op's morning read, restorable in
one line. Direction for the next four: the float model, the exponent-as-type second read, tick 3 closed by
compile, the owed test debt landed as artifacts, in that order, "by what unblocks the most."

**`53b`, after file 53, five calls.** The two-term cost model adopted as spec text with file 53's
coefficients, the "harder, more realistic case" wording corrected, the cliff named on the attempt list
rather than the accepted list (section 1.23). The starve-what-safety-does-not-prove licence shape adopted
as spec text (section 1.14), `float_algebraic` sent through its own vetting procedure before it is
anything more than spec text (one read done, second owed). The L2/L3 consumer-typing dispatch and the
merged decoder-ring-plus-face fixture both adopted as next work, in that order (fulfilled by files 55 and
56). `49:117`'s defect corrected in this consolidation, flagged as a ratified-table edit exactly as loud as
the `Int` drop; the table-diff obligation adopted as a standing consolidation convention, with `49:117` as
its exhibit (this document executes it, per its own opening paragraph). File 50's clause on the default
`Ranged` lowering adopted with a thread attached that neither the checkpoint's own brief nor the file that
prompted it had asked for: the strategy axis selects the door, not a blanket default (section 1.21, the
persona's own "what none of the questions asked" catch).

**`57b`, after file 57, six calls.** The strategy-door mechanism adopted with a presumptive per-preset
table (section 1.21). The bench-harness fix graded as changing the debt ledger rather than any conclusion:
nothing this review concluded rested on an unrun bench, and "deferral into untested infrastructure" is
named a standing failure mode from here (a deferral naming a mechanism as its resolution path owes one run
of that mechanism). The `unreproducible` ground adopted, with a targeted re-derivation demanded for the
one dependent that is itself a ratified workspace rule (section 1.19). `FromConstant`'s intent adopted,
vehicle held for its own second reads, carried into the live-defect registry, the round's topic file, and
a failing IMPL-phase test in that order (section 3). Three more ratified-table edits adopted: `Radix`
sealed, `Specials` as a product, both exponent lines corrected, each flagged for op exactly as loudly as
the `Int` drop. The cadence correction: consolidation five (this document) is written now, absorbing all
eight deliverables at once, because a prior reading of "consolidation first, then the four" as optional
was wrong and cost four dispatches routed around two known-defective table lines; named so the loop shape
does not become something each dispatcher re-derives to taste.

**Loudest for op's morning read, consolidated across all three persona checkpoints, one list.**

1. `Int` dropped from the ratified table (`48b`).
2. The exponent bounds spelled as types, both `Ranged` and `Implicit` (`53b`, `57b`).
3. `Radix` sealed as `Rad<P>` (`57b`).
4. `Specials` resolved to a product rather than a chain (`57b`).
5. The strategy-door mechanism and its presumptive per-preset assignment (`57b`, unasked item from `53b`).
6. `FromConstant`'s breaking-change fix, adopted in intent, vehicle held (`57b`).
7. The cadence correction itself, since if op's own reading of the loop was the one the checkpoints
   deviated from, the restatement in this section dies (`57b`).
8. A ratified workspace rule (`unstable-features.md`'s width-ceiling citation) may rest on an
   unreproducible measurement, pending a rebuild check (`57b`).

## 3. The live-defect registry

New this stretch, for defects in the shipped tree, as against findings about the still-unbuilt design.
Entries 1 through 4 come from file 55; entry 5 comes from file 57 and is marked fixed.

**1. `upward_rank` and `bin_pack` silently return wrong orderings under both shipped presets.** Tree:
`arvo-graph/src/rank.rs:34-88`, `arvo-comb/src/binpack.rs:44-63`. Both return a fold-shaped computation in
their operand numeral, claiming an exactness they do not have. On a four-node chain (compiled), `Hot`
inverts the ranking of a longer path against a shorter one (144 for weight 400, 210 for weight 210);
`Precise` degrades to a tie rather than inverting. `bin_pack` over-fills under wrapping and under-fills
under saturating, silently either way. Grounded `tree`, `pin`. The fix (section 1.20) is designed and not
yet shipped.

**2. `FromConstant` accepts an unrepresentable constant and silently produces a wrong bit pattern, or
panics.** Tree: `arvo/src/traits/from_constant.rs:40`, `arvo-numeric-contracts/src/lib.rs:85-88`.
`UFixed<8, 16, Hot>::from_constant::<300>()` writes a raw value (19,660,800) into a `repr(transparent)`
container whose logical width cannot hold it (max representable 16,777,215). The trait's own const
parameter lives on the method rather than the trait, so no bound can express representability. The doc
comment already names the truncation, which makes this a documented perimeter breach, not an undiscovered
one, and `what-you-can-observe-is-what-you-guaranteed.md` carries no carve-out for admitting a breach in
prose. It is the `Identity`-at-`I=0` defect generalised: the same shift-does-not-fit shape, every other
constant. The fix (move the const parameter to the trait, `FromConstant<const C: USize>`) compiles clean
with no unstable feature and is carried per section 2's instructions: entered in the round's topic file,
landed as a failing whole-matrix test at the start of IMPL phase, before the fix.

**3. `arvo-graph/tests/rank.rs` never enters the breaking path.** Every weight is a single digit (1, 5, 2,
7) against a `u8` container; the assertions are exact and correct and the test never sums past 7. Setup
that helps, not a fabricated pass: the file 55 probe raising the same shapes to 100 and 200 finds the
inverted ordering in two tests.

**4. `arvo-spectral`'s ten test files never exercise an arvo numeral.** All run against a test-local
newtype over bare `f32` (`arvo-spectral/tests/common/mod.rs:21`), whose own comment names the orphan-rule
reason for `f32` specifically; the reason does not extend to `FastFloat`, which the crate's own bound is
satisfied by, confirmed by compiling `fiedler_vector` and `power_iteration` directly against it (both
pass). The L3 crate's numeric behaviour is unexercised at any type the substrate ships, and since its
bound reaches `Recip` (division, held per section 1.13), this is where the held item's consequences would
first show up.

**5. `mock/benches/src/main.rs` could not run any bench at all. Fixed this stretch.** The orchestrator
double-shaped every variant path (`resolve_variant_path` already produces the platform dylib name;
`main.rs` reshaped it a second time, producing filenames with no corresponding file), so every worker
timed out and the harness panicked on the first bench's empty sample set. This blocked all four
pre-existing benches, none of which had ever produced a committed CSV, and would have blocked file 57's
own new bench identically. Fixed by dropping the redundant reshape; all five benches now run clean. Not a
design finding; recorded because every prior file that deferred a runtime claim to "the bench, one
afternoon" was deferring into infrastructure that could not have run it.

## 4. What is open

**Closed this stretch, listed once so the next member does not re-open them.** The float model (built,
absorbed with no new mechanism, section 1.16). Both exponent-as-type forks, `Ranged` and `Implicit`
(compiled, section 1.15). Tick 3, `Growth` leaving `Policy` (compiled structural theorem, section 1.10).
The runtime bench for the strategy-axis question (measured, section 1.21). The decoder-ring twenty-minute
challenge and the rigid-non-inhabitant E0275 residual (both tried and confirmed as ceilings, section
1.18). The vectorisable-loop-idiom claim's own unstated `-C codegen-units=1` dependence (found and named,
file 52). The L2/L3 consumer-typing dispatch (run, section 1.20, with three sub-items left open below).
The decoder-ring-plus-face fixture (run, section 1.18).

**The float model's residuals.** The radix-ten quantiser is unbuilt (the long-division kernel's shift
alignment is radix-two-shaped). Whether the model-width transfer argument (the reason a check at eight
bits is trusted at sixty-four) extends soundly to a `Ranged` numeral's two-width shape (precision and
exponent range) rather than only to precision alone, is believed but unproved. `Underflow = Abrupt` under
an unnormalised encoding is reasoned only, not modelled.

**Decimal's residuals.** The `InfOnly` `Specials` witness is still unfound, carried `unknown`. The OCP
OFP8 `E4M3`/`E5M2` `emax` figures and IEEE clause 5.2's preferred-exponent characterisation are both from
secondary sources and owe the file-39 treatment (checked against the primary text) before either sentence
hardens. The `u64` readout ceiling at `2^63` on `Pos::VAL` is a real consumer-facing question (widen to
`u128`, a multi-limb readout, or a comparison-only interface) raised and not answered. Decimal's own face
is untested; file 56's fixture is radix-two throughout.

**The licence's residuals.** `float_algebraic`'s second independent vetting read is still owed. Whether a
same-format multiply with no adjacent add legitimately needs `algebraic_mul` decoupled from `contract`
(section 2.4's `mulnum`-routing answer sidesteps rather than tests this) is unchecked. The build-layer
receipt's four clauses (section 1.14, section 1.21) remain designed and unbuilt by design, since arvo
grows no build harness of its own and nothing should be built ahead of a real consumer.

**The algorithm crates' residuals.** The `TotalOrd` fork is answered in the abstract (section 1.16) and
not yet built into these crates' own bounds; file 55 raised the stakes (if datum-level, none of these
crates' outputs is law-expressible at all) and the persona checkpoint queued building it both ways,
attempted rather than surfaced as a question, per the novelty posture. `foldnum` and `Unbounded` are new
vocabulary needing two independent reads. `Capacity`'s `Pos`-has-no-zero narrowing is flagged, not
resolved. Whether the widened result numeral is the right default for every consumer, or whether the
operand-numeral door deserves more use than `Monotone`-gated access alone, is a proposal, not a ruling.

**The notation macro's residuals.** The declarative-versus-proc-macro vehicle question is still
sketch-decidable and not sketched. Whether every operation in the design's surface needs a face-level
sibling, or only the ones a consumer is likely to chain, is unpriced. Whether `on_unimplemented`'s `{Self}`
interpolation is safe against every carrier the design ships, or only the two tested, is unswept. The
`#[deprecated]`-shaped lint on a direct `Reduce` bound is untried and belongs to a workspace-lint
accounting the review has not opened.

**The provenance registry's residual.** Whether `unstable-features.md`'s own width-ceiling citation
rebuilds from the committed trail (section 1.19), distinct from and narrower than file 8's already-marked
five-shape table.

**Codegen-flag audit, not fully done.** Files 24, 27, 43, 50, and 51's own remaining instruction-count
claims are not swept for `-C codegen-units` sensitivity, though the rule file 57 extracted (a claim about
one exported function, compiled alone, is very unlikely to be sensitive; a claim comparing two or more
functions in one compilation unit needs the sweep) narrows the remaining work considerably.

**Unchanged from `49`, untouched this stretch.** The membership second read (D39's finest-inhabited-system
candidate, owed since file 39, queued at every checkpoint since and still not run). The reduction firing
site and whether `FullRange` survives as its own named constructor. The dither-versus-`Refuse` interaction.
The per-application-against-per-value-moved event-counting sub-item of the fold's grade, genuinely op's,
declined three times running (`48b`, implicitly `53b`, explicitly `57b`). The real-consumer compile-cost
bench (narrowed by section 1.23's sweep, not replaced by it). `SC_WRAP<n>`/`SC_WRAP_SM<n>` with `n_bits >
0`, the one cell of the ratified no-gaps claim with no construction anywhere in the review. Richer
canonicalisation's branchlessness and cross-word bitpacked field extraction, both named limits on file
32's own measurements, unmeasured beyond them. `DatumDeterministic`, reasoned and named, not built as a
real `const fn`. The `Gcd`-for-a-local-`Rhs`-on-a-sealed-`Self` coherence question, argued moot and
compiled in one direction only.

## 5. The droplist

Carried forward from files 26, 40 and 49, extended by this stretch's own. Proposals tested and found not
to work, decided against, or superseded, stated with enough of their reasoning that a member who believes
a retest would come out differently knows what has to be overturned.

Everything `49:887-1027` carried stands unchanged and is not repeated here; nothing this stretch resurrected
any of it. New entries follow.

The candidate closed form for the overflow band, `q_result <= 2 * lattice`: refuted by exhaustive
enumeration in both directions (753/1000 addition, 639/1000 multiplication), replaced by the two-clause
lattice-plus-reachability form (section 1.5).

`Specials` as a three-instance chain (none, infinities-only, IEEE): the middle rung's witness demand
exposed that the axis was the wrong shape entirely; replaced by the two-fact product (section 1.16).

Absorbing a decimal numeral's quantum into its rational adjustment, dispensing with a separate exponent
axis: does not compile at any real decimal format's exponent range (a readout wall at `10^20`, a type wall
at depth 130), against 64 ms flat for the radix-and-exponent spelling of the same grid (section 1.17).

A finer-grained reassociation licence than the four-flag `algebraic_add` bundle grants: does not exist on
the stable-track surface as tested; the workaround (discharge each companion permission separately) is
sound because each is independently checked, not because the bundle became narrower (section 1.14).

Gating the algorithm crates' ordering-returning door on `AddAssoc`: the droplist already carried this
refusal (`49:896-898`); file 55 supplies the atom the refusal was reaching past, `Monotone<Add>`, and
confirms the three-crate, not four-crate, scope was a deliberate distinction rather than a slip (section
1.20).

The claim that file 34's vectorisable-loop-idiom finding held unconditionally: it holds, but only under
`-C codegen-units=1`, inherited by accident from an earlier, unrelated investigation and never identified
as load-bearing for this specific claim until file 52 checked it directly against the real crate under the
flag set file 34's own corrected methodology used (section 4, and the codegen-flag audit above).

A bounded numeral-notation table, already refused on principle (`49:1004-1007`): a second, independent
route to the identical refusal appears at the notation-macro's own face layer, where a const-generic face
cannot be structurally sealed and needs the same macro-mints-a-trusted-newtype resolution the raw
constructors already use (section 1.18).

Treating the algorithm crates' `Precise` exile as the problem to solve: the presets the design admits
today, `Hot` and `Precise`, both return wrong answers under the exact bound they satisfy; the exile was
never wrong, the admission was silently wrong, and no amount of readmitting `Precise` addresses a defect
that lives in the crates' own return type (section 1.20).

The three-instance reading of `Underflow` as one axis carrying flush-to-zero alongside gradual and abrupt:
flush-to-zero changes no representable set and is a `Quantisation` resolution, not a `Numeral` fact wearing
one's clothes (section 1.16).
