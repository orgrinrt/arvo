# The current shape, seventh consolidation: the transfer argument corrected, the crossing contract's precondition found, one keying rule closing three residuals, and the L0 migration priced

File 63 stood as the reference after the sixth consolidation absorbed files 59 through 62 against the
third stand-in checkpoint, `62b`. This document replaces it, absorbing four further deliverables (files
64 through 67) read against the fourth and, this stretch, fifth stand-in checkpoint, `67b`, made
overnight while op slept a fifth night in a row. Op has not yet read `67b`, `62b`, `57b`, `53b`, or
`48b`; every call inside all five is persona-decided, not op-decided, and every one dies the moment he
says otherwise. Op's own four checkpoints (`30b`, `34b`, `39b`, `44b`) remain exactly what they were and
are restated below rather than re-litigated.

This document makes four corrections to the sixth consolidation that are statements rather than
additions, each named at the top per the standing discipline this review has kept since the fourth
consolidation's own defect. First, `63`'s account of `unstable-features.md`'s transfer argument (the
sentence crediting the `specialization`/`TypeId` bans with making "the transfer... sound") is refuted by
two independent compiled counterexamples, both run with the bans in force: a rounding tie reachable at
every even radix and no odd one, and an absorption-freedom property exhaustively true at one exponent
span and false at the next with precision, code and bans held fixed. The bans themselves are untouched;
what they were credited with was never theirs to claim, and a third mechanism by which an instantiation
can be observed and behave differently, already shipped in `arvo-strategy`, was missing from the rule's
own enumeration. Second, `63:184-198`'s crossing contract, stated as three round-trip statements, is not
missing a fourth statement but a precondition: two of the three are ill-typed without it, compiled here
as an `E0308` whose suggested fix is the identical unchecked coercion the design performs silently.
Third, a sentence in the sixth consolidation's own section 6 (file 66's finding, carried forward
unchanged) said "exactly one cell of the matrix leaks." That was a sampled law holding one axis (the
design's `Specials` product) fixed; the full product leaks at six of eight cells, and the correct framing
is a family of configurations rather than a single one. Fourth, `63:816-817`'s claim that the shipped
`arvo-strategy`/facade forbidden-feature defect (entry 6 of the live-defect registry) touches "every
consumer of `Bits`, `UFixed` and `IFixed`" is false. It is measured false: three of the four things a
consumer actually writes (`Bits<N>`, `Uint<N,S>`, `Int<N,S>`) are unaffected by either candidate fix, and
the real public break is twenty-one call sites naming `Fixed<I,F,S>`/`Signed<I,F,S>` directly.

The stretch's arc: file 64 (Chlipala) cleared five of the seven owed second reads outright and
discharged one carrier-at-birth condition rather than merely confirming it, reproducing a real, silent,
orphan-rule-legal defeat of `Unbounded`'s own stated guarantee before closing it. File 65 (Pesce) did
the pricing the sixth consolidation named as the highest-leverage item for this stretch, found a
committed sketch that had already answered a third of the question six days earlier and gone uncited,
performed one of the two migration gates on the real crate rather than merely proposing it, and measured
the other gate's cost as a genuine engineering fork rather than a line count. File 66 (Lamport) took the
ratified transfer argument apart, found the sentence it rests its own certification claims on promotes a
necessary condition to a sufficient one, refuted the promoted version by compiling counterexamples
against it, and built the replacement: a per-coordinate transfer-ground scheme drawn from a closed,
sealed vocabulary of four. File 67 (Rompf), reading the crossing-contract question independently before
opening file 66's answer per the panel's own convention, reached a sharper diagnosis (a precondition
rather than a missing statement), extended file 66's transfer scheme with a third coordinate the
mechanical bans do not close, and produced the stretch's second design rule: a fact is keyed on the
coarsest layer whose identity its truth depends on, which turns out to be the same defect the crossing
contract, the `TotalOrd` split and the notation's face layer all separately hit.

Op's four checkpoints made five calls between them: D69 ratified, D39 held, the value-unique encoding
ratified in full with division held, and the grounding registry adopted with a backfill obligation, all
unchanged since the sixth consolidation. The five persona checkpoints (`48b`, `53b`, `57b`, `62b`, `67b`)
make calls that are **persona-decided, not op-decided**, flagged throughout with that weight, and the
newest closes more of the standing ledger than any prior checkpoint has: five of seven owed second reads,
two of the notation's three residuals, and one authorized (but not executed) migration gate. Section 2
restates all five and closes with a single loudest-for-op list reflecting the current state of every
item, including where a later checkpoint or a later file already resolved an earlier one.

**Verification.** Every claim below tagged compiled or measured traces to a probe or a committed artifact
in `64_probes/` through `67_probes/`, each carrying its own `OUTCOMES.md`, or to the shipped tree at the
cited path and line, re-read fresh in this stretch. The design surface this review builds still has no
shipped source for the numeral tower: run fresh from the repo root for this document,
`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same command with
`FullRange\|UTerm\|AddWidth` in place of the first pattern both exit 1, empty, unchanged since file 45
first corrected the path this command uses. `cargo test --offline --workspace` reports 658 passed, 0
failed, 9 ignored for files 64 and 66 (neither touched a shipped crate); file 65's whole-crate
`arvo-strategy` migration was performed on a copy of the tree outside the repository and its own run
reports the identical 658/0/9 against the migrated crate, matching baseline exactly; file 67 touched no
shipped crate either. The pinned toolchain is `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, resolved from the repo's `rust-toolchain.toml`, confirmed fresh for this document.

**The table-diff obligation, executed on this document by its own author before it stands**, per the
standing rule the `57b` checkpoint set and every consolidation since has carried out on itself. Every
table below was checked line by line against the prose of the section it sits in and against the source
file that established each member. The assembled trait table in section 1.22 now carries the `Crosses`
trait file 67 proposed, which section 1.4's prose states but the sixth consolidation's table predates
entirely (a new member, not a correction of an old one). The live-defect registry's entry 6 was rewritten
rather than patched, and its citations were checked against file 65's own line numbers rather than
against this document's paraphrase of them.

**On the sentences below that are this document's own resolution rather than a restatement of a source
finding.** Four places in this document do original work beyond compression, and each is marked where it
occurs. The consolidated loudest-for-op list in section 2, which reflects this document's own judgment
about which items across five checkpoints are still live versus already resolved by a later checkpoint or
a later file. The bridging note in section 3 connecting the live-defect registry's `TotalOrd` and
`arvo-spectral` entries to the new face-keyed refusal under the layer-keying rule's reclassification,
which keeps the three entries separately listed for locatability while stating the shared cause file 67
established. The explicit statement, in this document's own words, that `63:816-817`'s "touching every
consumer" sentence is corrected rather than merely superseded. And the note in section 1.12 observing
that the seal-as-free-diagnostic dividend has now arrived independently at six carriers, which is this
document's own count rather than any single source file's tally. Everything else is a compression of a
claim a source file already made, tagged with that file.

## The three design rules

Two design rules have been the review's real product since the persona checkpoint at `48b` named them;
this stretch adds a third, of the same shape and the same weight.

**The spine rule.** A quantity that is computed and then has to appear in a type is a type; a quantity
that only ever has to be read is a const. Nine occurrences stand from the sixth consolidation's count (two
founding, at op's `44b`; seven since, through the shipped `arvo-strategy` container dispatch and its
facade). No new firing this stretch; file 65's pricing work is downstream of the ninth firing rather than
a tenth.

**The carrier-at-birth rule.** A closed vocabulary that a guarantee quantifies over owes its seal and its
adversary at birth, not after three passes. File 64 gave this rule its clearest instance yet: `Unbounded`'s
`Arity` vocabulary was proposed unsealed, and the condition was not merely named this time, it was
discharged. A downstream crate can, today, in the design's still-unbuilt tower, forge a `Safe`
interior-safety verdict for a loop whose trip count is a function of the data by writing its own arity
marker instead of the design's, silently, with the orphan rule's own blessing and zero diagnostics
anywhere in the chain. Sealing `Arity` (`Fin<P>` wrapping a finite `Pos`, alongside `Unbounded`) closes
both attack routes tried, at the cost already priced. The rule fires a second time this stretch on its own
vocabulary: `67b` adopts the four-member transfer-ground vocabulary (section 1.19 below) as sealed at
birth, citing file 64's `Arity` result as the reason not to repeat the mistake on a vocabulary the review
is proposing rather than one already shipped.

**The layer-keying rule, new this stretch.** A fact is keyed on the coarsest layer whose identity its
truth depends on. File 67 derived it from a narrower, incorrect three-layer framing (face, encoding,
value) inherited from that member's own earlier file in this review; the correction is that there are two
identity notions, not three, because the value-unique encoding ratified at `44b` makes the encoding the
value's own representative in the type system, so encoding identity *is* value identity. That leaves face
identity (established at expansion time, per invocation, syntactic and site-local) and
encoding-equals-value identity (established at type-check time, structural and global). A fact depending
only on what a numeral denotes belongs on the encoding: every law, every arithmetic result, every
comparison, every membership question. A fact depending on where something was written belongs on the
face, and nothing else does: diagnostics and display, full stop. "Coarsest" rather than "finest" is the
load-bearing word: keying a fact too finely is not imprecision, it is a **false statement**, because the
finer layer's extra distinctions are not value-carrying, and a fact that respects them asserts something
untrue about the numbers with a compiler's own authority behind it. Compiled: two macro-emitted faces for
the identical literal `1/3` (`Third` and `OneThird`, spelled `1/3` and `2/6`) refuse to unify under a law
keyed on the face, `E0308`, a refusal that is wrong about the arithmetic. The rule turns out to be already
enforced, and nobody arranged it: a macro-minted face can never reach a numeral position at all, because
the seal's private supertrait is unreachable from the expansion crate (file 61's structural necessity),
so the only route from a face into the tower is the `NumeralFace::Encoding` projection, and that
projection is a function whose argument nothing downstream recovers. File 61's bridge-trait "cost" is,
for free, the layer-keying rule's own enforcement mechanism. The rule reclassifies three previously
separate findings, the shipped `TotalOrd`'s bit-order comparator, `arvo-spectral`'s NaN-payload-dependent
partition classification, and the new face-keyed refusal, as one defect recurring at three layers rather
than three unrelated ones: **a value-keyed operation must consume its operand through a canonicalising
projection, and that projection must be the only door.** The `TotalOrd` split adopted at `62b` is this
general fix's special case at the datum layer, not a separate mechanism. Full treatment, including the
three notation residuals it closes or reduces, at section 1.18.

## 1. The agreed shape

### 1.1 What a number is

Unchanged from file 40 and every consolidation since. A value of `Number<N: Numeral, S>` is an integer k,
drawn from a finite interval, together with a type-level rule injecting k into a set of rationals (plus,
for floats, a handful of data that are not rationals at all: `Specials`, section 1.16). The numeral names
the representable set and the indexing; D69 (op, `30b`) put them on two sides of the design, identity
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

Every exponent position is a type. `Radix` is a sealed carrier, `Rad<P>` the sole constructor over the
sealed `Pos` bounded by `AtLeastTwo`, exhaustive by construction, now standing on five compiled routes
(section 1.12). `Bias` and `Adjustment` are signed, gcd-normalised rationals, value-unique and sealed, as
ratified at `44b`, and this stretch's layer-keying rule (above) is what value-uniqueness ultimately buys:
the encoding is the value's representative, not merely a convenient carrier for it.

### 1.3 Encoding, nested inside Lowering

Unchanged. `Lowering` changes no value; `Encoding`, nested inside it, may change which datum carries a
value. No law may read `Lowering`; a law's key is a `const fn` parameter list and `Lowering` is not a
parameter. File 67's section 4 finds the load-bearing sharpening this stretch adds: this correct rule has
been doing double duty as "*nothing* may read `Lowering`", which is exactly what left the crossing
contract's precondition unstated and the transfer argument's index set six coordinates short of the seven
it needed. Section 1.4 and the transfer-ground scheme (section 1.19) are both repairs to that double duty,
not to the rule itself.

### 1.4 The crossing contract: a precondition, not a fourth statement, and a family of leaking configurations rather than a cell

**Corrected this stretch, replacing the sixth consolidation's account in full.** The three statements
stand as file 54 and the review since have carried them:

1. `decode ∘ encode = id` on values, always.
2. `encode ∘ decode` is idempotent on data, always (canonicalisation).
3. `encode ∘ decode = id` on data iff the encoding is injective, a derived boolean.

File 66 found a real gap by answering an adjacent question honestly: what does `Underflow = Abrupt` mean
under an unnormalised significand, where no constant leading digit exists to hide and a value has one
datum per representable exponent shift rather than one? Exactly one meaning is available, and it settles
the question this stretch closes for good. The datum-level reading ("exclude data whose exponent field is
minimal and whose significand is small") is not an `Underflow` fact at all; it is an `Encoding::Canonical`
choice wearing an `Underflow` costume, the identical category error file 50 already caught once when it
moved flush-to-zero off `Numeral`. The value-level reading, the hole `(0, r^EMIN)`, is the meaning, needs
no new mechanism, and closes the residual.

**Answering it exposed the real gap, and file 67, reading the question fresh before opening file 66's
answer, diagnosed it more sharply than a missing fourth statement.** Writing out the three maps honestly:
`encode : V -> D`, and `decode`, at every numeral this review has built, is total arithmetic on the
physical fields landing in the rationals, `decode : D -> ℚ`. Nothing in that formula consults `V`.
Statement 2, `encode ∘ decode`, therefore requires `decode`'s output to lie in `encode`'s domain, which is
`V`, not `ℚ`. Under a hole, statement 2 is not false: it is **ill-typed**, and statement 3 inherits the
same defect. `67_probes/probe_1_statement_two_is_illtyped.rs` compiles this out honestly (a `Value`
newtype whose only constructor checks membership) and gets `E0308: mismatched types`, with rustc's own
suggested fix, `Value { inner: decode(f, d) }`, being exactly the unchecked coercion the design performs
silently today. So the missing item is a **precondition** the other two statements are stated over, placed
in front of them rather than beside them as a fourth co-equal item.

**The escape is a family, not the single cell file 66's own matrix reported.** File 66's sweep held
`Specials` fixed while varying radix, precision and normalisation, and concluded "exactly one cell of the
matrix leaks", `Abrupt` with an unnormalised significand. `Specials` is the design's other value-set-
shrinking axis (`63:656`, a four-point product, sitting on `Numeral`), and nothing couples it to the field
layout that decides which data exist (`Encoding::Fields`, sitting on `Lowering`). File 67's
`67_probes/probe_2_the_escape_is_a_family.rs` models an IEEE-shaped field layout at E4M3's own shape
across the whole `Specials` product:

| layout | `Specials` | data | escaping | percent |
|---|---|---:|---:|---:|
| ieee | `NoSpecials` | 128 | 8 | 6.2% |
| ieee | `NanOnly` | 128 | 1 | 0.8% |
| ieee | `InfOnly` | 128 | 7 | 5.5% |
| ieee | `IeeeSpecials` | 128 | 0 | 0.0% |
| ocp | `NoSpecials` | 128 | 1 | 0.8% |
| ocp | `NanOnly` | 128 | 0 | 0.0% |
| ocp | `InfOnly` | 128 | 1 | 0.8% |
| ocp | `IeeeSpecials` | 128 | 0 | 0.0% |

Six of eight cells leak. Under the IEEE layout, three of the four `Specials` members leak (the largest
leak is the entire top exponent code, one part in `2^ew` of the datum set); only the member the layout was
designed for, `IeeeSpecials`, does not. The correct framing is that the hazard fires whenever a `Numeral`
axis shrinks the value set and no `Lowering` axis correspondingly shrinks the datum set. OCP's own E4M3
format is not a counterexample: it is the existence proof that real format designers do this coupling by
hand and pay a standard's own paragraph for it (dropping infinities, raising `emax` from 7 to 8 until
`decode` was total again), and the same layout leaks the moment its value set moves again under
`NoSpecials` or `InfOnly`. arvo currently has nowhere to state this coupling and nothing that notices when
it is skipped.

**The repair is derived, not chosen: only one exists.** The alternative to excluding the escaping data at
the encoding (statement 0's obligation) is widening the target, composing through the quantiser
(`encode ∘ quantise ∘ decode`). `67_probes/probe_3_no_encode_side_repair.rs`, run against file 66's own
model verbatim, checks the quantiser on every escaping datum of every leaking cell and finds it refuses
without exception (1 of 9, 4 of 21, 9 of 297, 108 of 2,997), against a negative control confirming the
quantiser is the identity on all 2,701 values it does hold. Shrinking the source is not a preference
against widening the target; widening the target does not exist.

**The mechanism, adopted presumptively, one expert's shape wanting its second read.** File 67 diagnosed
the root cause: `63:179-181`'s correct rule that no *law* may read `Lowering` has been doing double duty
as nothing may. The design needs a second kind of claim beside the law, keyed on the pair rather than the
numeral alone, and its shape already exists in the tree, the notation vehicle's own coarsening-with-a-
checked-bound (`NumeralFace::Encoding: Bias`, section 1.18):

```rust
/// The obligation a Lowering owes a Numeral. Not a law: it reads Lowering,
/// which the design forbids a law from doing, and its key is the pair rather
/// than the numeral alone.
pub unsafe trait Crosses<N: Numeral>: Lowering {
    // Statement 0 is the safety condition: for every datum d of this encoding,
    // decode(d) is in V(N). Statements 1 through 3 are meaningful only where
    // this holds.
}
```

Where the encoding is one the tower generates, the impl is blanket and safe (D16); where a consumer brings
a hand-laid field layout, it is an `unsafe impl` stating something the consumer must know, at the format
declaration site, exactly as `NumeralFace`'s bound sits at the face declaration site. It weakens
`63:179-181` nowhere: a law stays keyed on `Numeral` alone, and no law may bound on `Crosses`. Rewrite cost
against the shipped tree is zero (no shipped source names any of this); against the design it is one new
trait beside `Lowering`. Explicitly not proposed: `decode` returning a `Maybe`, a runtime check, or a
per-operation well-formedness predicate. All three move a declaration-time fact into a use-time check,
which the design's own binding-time discipline exists to avoid.

**Still open, visibly so.** Statement 0 against the two operations `63:338` already carves out as
datum-dependent by definition, `quantize` and `roundToIntegralExact`. Two files have flagged it forward
and neither performed it; file 67's guess that it lives wholly inside `D` (a datum-dependent operation's
result depends on an operand datum, a statement inside `D` that does not obviously touch `decode`'s
codomain) is marked as a guess, not an answer.

Also adopted: file 66's toolbox note. `Abrupt` on a decimal numeral is a pure representability restriction
with no encoding-space or performance dividend, because the minimum-exponent, sub-normalised-significand
cohort that radix two's `Abrupt` frees up for flush-to-zero hardware does not exist under an unnormalised
significand; those encodings were never free there. The axis stays available per the workspace's own
warn-never-police discipline, documented at the point of choice.

### 1.5 The quantiser

Unchanged this stretch. Round-first, classify second, confirmed against binary32 on 41,380,159 operations
with zero mismatches (file 50); the radix-general kernel repaired and regression-checked bit-for-bit
against silicon at radix two (file 59). A tie is reachable only at an even radix, and this stretch's
transfer-argument work (section 1.19) re-derives that fact independently, from the quantiser itself over
radices two through thirteen, as one of two counterexamples that refute the ratified transfer sentence.

### 1.6 Membership and the number-system layer

**Sharpened this stretch, still held exactly as D39 left it.** File 39's candidate mechanism
(the derived "finest inhabited system", reporting the finest system a numeral's value set actually
inhabits) is confirmed sound, independently, for every numeral arvo has ever built or designed, all of
which sit on the one genuine chain in op's own ratified vocabulary, ℕ ⊂ ℤ ⊂ ℚ ⊂ ℝ. **The mechanism's
stated justification for uniqueness does not survive contact with the full ten-member vocabulary D38
ratified.** "Exists and is unique because the tower is a chain" is false against ℕ, ℤ, ℚ, ℝ, ℂ, ℍ, 𝕆,
Surreal, Hyperreal, p-adic taken together: the surreals and hyperreals are both ordered-field extensions of
ℝ, mutually incomparable and neither containing ℂ (an ordered field cannot contain ℂ at all, since squares
are non-negative in an ordered field and `i^2 = -1` is not); each p-adic completion, by Ostrowski's
theorem, is not an ordered field and is incomparable to the rest. So D38's vocabulary, read under D39's own
structural test, is a tree with at least three incomparable branches above ℚ, not a chain. This is number-
theoretic reasoning, reasoned rather than compiled, and file 64 states it as such. **The recommendation:
scope the single "finest" associated fact explicitly to the real/Cayley-Dickson chain, and give the
surreal, hyperreal, and each p-adic branch their own independent, non-competing membership predicates**,
before `arvo-num-systems`'s own type shape bakes in a uniqueness claim that silently assumes a sub-chain
op explicitly ratified as wider. Final hardening stays op's, since the hold was his.

### 1.7 The algebra: what a law is, and the finest view it holds at

Unchanged this stretch.

### 1.8 The fold: two conditions, two relations, and what the accumulator becomes for a float

Unchanged this stretch.

### 1.9 The multiplicative half

Unchanged in mechanism, and this stretch's transfer-ground work adds a second, independent payoff to it.
`mul_full: N1 x N2 -> mulnum(N1, N2)` computes the exponent sum at the type level, which section 1.19's
exponent-offset symmetry needs to hold multiplicatively: the symmetry that lets an additive claim transfer
directly across a shifted window is not the symmetry a product needs (a product's equivariant home is a
window shifted by *twice* the offset), and `mulnum`'s own construction already lands exactly there,
checked at 254,830,080 instances with zero failures.

### 1.10 Widening and Growth: two axes removed, closed shut

Unchanged this stretch.

### 1.11 The value-unique encoding: ratified, sealed, priced, and now load-bearing for a second design rule

Unchanged in substance from `44b`; this stretch's layer-keying rule (top of document) is built directly on
top of it. Value-uniqueness is what makes "keyed on the encoding" and "keyed on the value" the same
statement, which is the fact that collapses the notation's three apparent identity layers to two.

### 1.12 The seal: carriers sealed, contracts open, and the seal as free diagnostic

Unchanged in mechanism. `Rad<P>` stands on five compiled routes (file 62). **The seal-as-free-diagnostic
dividend has now arrived independently at six carriers, not the five the sixth consolidation counted, and
the count is this document's own tally rather than any single source file's.** `Rad<P>` (files 56, 62);
the strategy door's `HostImplemented` marker (file 59); the notation macro's `Bias`/`Adjustment`
constructors, unreachable from outside the tower crate (file 61); `Arity`'s sealed `Fin<P>`/`Unbounded`
pair (file 64); a per-width `WidthFor<Family>` table, the shape one candidate migration route for entry 6
of the live-defect registry would use (file 65); and `NumeralFace`'s own coarsening bound, whose free-
diagnostic character file 67 names as the layer-keying rule's own enforcement mechanism (section 1.18).
Six independent arrivals is past the point of coincidence; it is a property of how this design's sealed
carriers behave under rustc's own trait-resolution diagnostics, and it is worth stating as a design fact
in its own right rather than continuing to log each new instance as a surprise.

### 1.13 Division: held, unchanged this stretch

Not adopted, held exactly as `44b` left it. Untouched by any of files 64 through 67.

### 1.14 The grade is a type: projection, join, evaluation, IEEE convergence, the licence

Unchanged this stretch.

### 1.15 The exponent forces the spine rule open a second time

Unchanged.

### 1.16 The float model: closed on Underflow, one crossing-contract gap surfaced through it

`Underflow`'s two instances, flush-to-zero as a `Quantisation` resolution rather than a `Numeral` fact,
`Specials` as a four-point product, `TotalOrd`'s level fork (now the layer-keying rule's special case,
section 1.18): all stand as `63:296-321` states them. The one open residual from the sixth consolidation,
what `Abrupt` means under an unnormalised significand, closes this stretch (section 1.4): exactly one
meaning is available, the value-level hole, and answering the question is what surfaced the crossing-
contract gap that section 1.4 now carries as its own subject.

### 1.17 Radix ten: the chain, the section, and the standard's own text

Unchanged this stretch in every particular. File 66's transfer-argument work re-derives the tie-
reachability fact independently from the quantiser (section 1.19), which is a second read of `63:220-223`
rather than a new finding.

### 1.18 The numeral notation: two of three residuals closed by the layer-keying rule, the third's control derived

**Closed at the sixth consolidation as a vehicle question; this stretch's work is entirely about the
residuals it left.** The three named at `63:864-868` were: whether `Adjustment` needs its own entry point;
whether every literal a consumer writes twice should resolve to the same face type; and a pricing hazard
where an unused type alias measured cheaper than one whose bound was actually forced.

**`Adjustment` needs its own entry point, and the reason is keying rather than duplication.** The
duplication half is easy: parse, digit extraction, decimal-point folding, gcd reduction and bit
decomposition are the identical arithmetic on the identical digits for both roles, one generator serves
both. The harder half, under the layer-keying rule, is whether the *role* (scale versus offset) belongs in
the type, and it does, because the two roles enter the value map differently: file 66's own exponent-shift
symmetry (section 1.19) is conditional on no `Numeral` member contributing a nonzero additive constant to
the value, and an adjustment scales while a bias offsets. `67_probes/probe_6_adjustment_needs_its_own_
door.rs` prices the exchange: with one shared face type and the role carried only by argument position,
`value::<E, X, Y>` and `value::<E, Y, X>` both compile, both run, and silently denote 11 and 84.33
respectively. Two doors over one shared generator (`raw_bias!`, `adjustment!`) refuse the swap, `E0277` on
both routes. Closed: two entry points, one generator.

**Cross-call-site face identity should not be established, and the residual was posed about the wrong
layer.** Nothing that affects compilation is keyed on the face at all (a face cannot reach a numeral
position; the `NumeralFace::Encoding` projection is the only route and it erases the declaration site), so
two faces for one literal are interchangeable everywhere the type checker looks. Where the difference is
observable, per-site is the correct answer: a consumer's error at one declaration site should name that
site, and unifying two faces would make one site's diagnostic name another site's declaration, which is
strictly worse and, per the layer-keying rule, is exactly the false-statement failure the rule forbids.
Closed without a mechanism: face identity is per declaration site, deliberately, and the spec states this
rather than building anything against it.

**The pricing hazard's control is derivable rather than a trick to remember.** A declaration's cost is a
fact about the obligations it forces, and an unused alias forces none: no bound is forced, `Reduce` never
runs, and the type checker does the work its instantiation demands and no more, which is monomorphisation
behaving correctly rather than a benchmarking artifact. The control: a measurement of a declaration's price
states which bounds it forces, and two arms are comparable only when they force the same ones. File 61's
own corrected staging measurement already satisfies this control.

**The decoder ring is not a defect, and the general statement corrects the specific one the sixth
consolidation carried.** `63:428-435` recorded a decay (an operation generic over the raw encoding decays
the face one hop in) and a message that goes dead when a refusal fires on a projected associated type. Both
are the layer-keying rule made visible rather than problems in the notation: an operation keyed on the
encoding names the encoding when it fails, because the encodings are what differ, and naming the face there
would name something that did not fail. **The spec sentence to retire is "the error names your numeral";
the sentence that is true and checkable is "an error names the layer the failing operation is keyed on."**
File 59's independently-reached fix (write the message on the carrier whose bound actually fails) is the
same instruction stated a different way.

### 1.19 Claim provenance: the grounding registry, and the transfer-ground scheme replacing what `ffl` was credited with

Unchanged this stretch as far as the five-row grounding table goes (`ratified decisions`, `settled
shapes`, `physical grounds`, `tree grounds`, `unreproducible`). What is new is a second, orthogonal
annotation a claim owes when it is established by bounded exhaustion at a model instance and relied on at
a real one: a **transfer ground**, one per coordinate of the claim's index set.

**The correction that motivates it.** `unstable-features.md`, a ratified workspace rule, states: "Without
[full `specialization` and `TypeId`], monomorphisation is uniform and the transfer is sound." Every
sentence before the last is correct. The last promotes a necessary condition (**implementation
uniformity**: one parametric function, no instantiation gets a different body) to a sufficient one for a
different, unproven claim (**property uniformity**: the truth value of a claim about the function's
outputs does not move as the parameters move). The rule's own source, `10_leroy_what_is_actually_
certified.md`, already gave the transfer argument four legs and knew the difference: leg one is
parametricity, which the bans enforce and which the ratified sentence kept; leg three is "width-uniformity
of phi's behaviour... a property of the rule's shape, arguable in prose... never mechanical," named
correctly as unproved. The compression from four legs to one sentence kept the mechanical leg and attached
the other three legs' conclusion to it.

**Refuted by machine, twice, with the bans in force.** A rounding tie is reachable at every even radix and
at no odd one (`2 * lost == R^s` has no solution for odd `R`), re-derived here directly from the
quantiser over radices two through thirteen with the rounding counts recorded so the odd rows are not
vacuous (318 roundings at `r = 3`, 188,448 at `r = 13`, zero ties in either). And absorption-freedom (for
all nonzero `y`, `quantise(x + y) != x`) is exhaustively TRUE at exponent span `p` and FALSE at span
`p + 1`, with the precision, the code and the bans all held fixed: `EMAX` moved by one and the property's
truth value moved with it. Same precision, same code, same bans, and the outer quantifier failed anyway,
because nothing about implementation uniformity ever bore on it.

**The replacement, adopted as spec text.** Every claim established by bounded exhaustion at a model
instance names the index set it is quantified over, coordinate by coordinate, and carries one transfer
ground per coordinate, drawn from a closed, sealed vocabulary of four:

| ground | what it asserts | who supplies it |
|---|---|---|
| `symmetry` | an exact group action carries the model instance onto every target instance, under a stated condition | the claim's author, once per axis |
| `saturation` | the claim's dependence on the coordinate stops changing past a stated threshold, and the model's coordinate clears it | the claim's author, threshold stated |
| `induction` | the claim at `t + 1` follows from the claim at `t` by a stated argument | the claim's author, in prose |
| `unargued` | the claim is a fact about the model instance and nothing else | nobody: the default when no ground is named |

`unargued` as the default is what makes the scheme honest: a claim naming no ground does not silently
inherit one. The vocabulary is sealed at birth per the carrier-at-birth rule, citing file 64's `Arity`
result directly as the reason to seal a vocabulary the review is proposing rather than repeat the mistake
on it.

**For a `Ranged` numeral, worked out per coordinate.** `EMIN` and `EMAX` carry `symmetry`: the quantiser
commutes with scaling a value by `r^k` when the window shifts by `k`, checked over 509,660,160 instances
(every value and every exact pairwise sum, two radices, two precisions, four spans, both underflow
policies, five shifts) with zero failures, plus two negative controls that both correctly disagree (a
window-only shift disagrees on 8 of 13 values; adding a nonzero additive constant to the value map breaks
the symmetry on 29 of 51 checks). The condition, that no `Numeral` member contributes a nonzero additive
constant to the value, holds today because `Ranged` carries no `Bias` member; the day `Ranged` gains one,
the symmetry dies silently unless the condition is written down, which it now is. Two of the six
coordinates collapse into one: only the span matters, not the absolute position of the window.
Multiplication is equivariant into a window shifted by `2k` rather than `k`, exactly `mulnum`'s own
construction (section 1.9). The span carries `saturation`: the threshold is measured, not argued, at
`p + 1` under `Abrupt` and 2 under `Gradual`, independent of `p`, and the underflow policy is what the
sixth consolidation's own model checks (file 50's fold at span 8, its band model at six binades) cleared
by luck rather than by design, since nothing told their authors what the threshold was. Precision and
radix carry `unargued`; no induction argument exists for either, and the radix is known genuinely
non-uniform per the refutation above.

**Extended by file 67 with a third coordinate the mechanical bans do not close at all: container class.**
`arvo-strategy/src/container.rs:254-280` projects a width through a const-tag dispatch (`tag_hot_cold`,
`bytes_for_u16`) to a distinct associated-type container. This is a type observing which instantiation it
is in and behaving differently, is permitted (no forbidden feature, no gate), and is shipped. Compiled:
one parametric body, no specialization, no `TypeId`, a property TRUE at eight bits (`u8` wraps on doubling
200) and FALSE at nine (`u16` does not). The bans close the ways an instantiation can get a different
*body*; they never closed the ways it can get a different *type*, and this is that third way. It takes a
`saturation` ground with the cleanest threshold in the scheme, one width per container class, read
straight off `tag_hot_cold` rather than measured or argued. Twelve distinct container types exist across
the strategy markers (six classes for `Hot`/`Cold`, five for `Warm`/`Precise`), and every model claim this
review has run, including file 50's 41-million-operation binary32 check and file 64's exhaustive eight-bit
`TotalOrd` matrix, exercises exactly one of them, none saying so. The follow-up is cheap and owed: a
nine-bit companion model covering the `u16` class costs `2^18` pairs, well inside the budget once the
budget is correctly understood as a step count (below). Classes above `u32` are `unargued`, and the spec
states that in those words rather than leaving it implicit.

**A correction to the rule's own supporting measurement: the wall at nine bits is a step budget, not a
width ceiling.** The quadrupling per bit reproduces and is structural (0.10s at seven bits, 0.29s at
eight, 1.05s at nine on one cheap sweep, ratios 2.9x then 3.6x, matching file 8's own figures). The refusal
at nine bits is not structural: a cheaper predicate (one const-eval sweep with no per-instance stability
check) compiles clean at nine bits and refuses only at ten; file 8's own five-constructor stability check
refuses at nine because that check is more expensive per instance, not because nine bits is a wall. The
argument this rule needs is unchanged (exhaustive validation at a real width remains unavailable by a wide
margin) but a reader who takes "refused at nine bits" as a ceiling on width, rather than on total step
count, will under-budget every model that uses a cheaper predicate, exactly the situation the container-
class coordinate's own companion model is in.

**Three rule-wording edits go to op as one package, none touching the ban.** The last-sentence correction
(replace the promoted-necessary-to-sufficient claim with the transfer-ground scheme's own summary
sentence); the penultimate-sentence third-way clause (name container-class const-tag dispatch alongside
`specialization` and `TypeId` as a third, permitted way an instantiation is observed); and the step-budget
clause (state the wall as a total-step-count budget rather than a bit-width ceiling, with file 8's own
five-constructor predicate as the worked example). All three are compiled facts; the wordings themselves
are op's because `unstable-features.md` is ratified and no persona touches it.

### 1.20 The algorithm crates: unchanged this stretch, one reclassification noted

Unchanged in content from `63:476-539`. The `TotalOrd` split and the `arvo-spectral` NaN-classification
defect are, per this stretch's layer-keying rule, the same defect as the notation's face-keyed refusal,
recurring at three layers rather than three separate mechanisms; the entries stay separately listed in
section 3 for locatability, cross-referenced to the rule.

### 1.21 The strategy door: unchanged in mechanism, one settled call, one naming residual

The mechanism, the derived door assignment, and the corrected per-preset table all stand as
`63:541-624` states them. **Two of the two narrow items the sixth consolidation left for op close this
stretch, in different directions.** File 64 confirmed the refusal-versus-fallback call directly against
its cited precedent (`arvo-strategy/src/container.rs:104-112`, read fresh and quoted): refusal is settled,
no reopening. **`IeeeDefault` as a name is a new residual, narrower than the environment-choice question it
sits beside.** The design's own stated architecture admits it cannot verify, without a build layer that
does not exist, that the process's actual floating-point control state matches what the name promises;
`HostLowering<N, IeeeDefault>` reads, to anyone encountering the type, as a verified claim the design
cannot back. The recommendation is a rename (something like `AmbientFloatEnv`) or an explicit
"assumed, unverified" annotation at the declaration, independent of which concrete environment op
eventually picks as `Hot`'s default; the naming question and the environment-choice question are separable
and both go to op.

### 1.22 The assembled trait table, and what it costs to build against the tree

```rust
// Every member that denotes a number is drawn from one value-unique, sealed,
// type-level encoding, sealed and attacked on every introduction route (1.11, 1.12):
//   Nat ::= Z | Pz<P>            P: Pos       precision, widths, exponent bounds
//   Pos ::= H | O<P> | I<P>      P: Pos       magnitudes
//   Bias ::= BZero | BPos<N, D> | BNeg<N, D>  N, D: Pos, N: Gcd<D, Out = H>   signed rational
//   Exponent ::= EZero | EPos<P> | ENeg<P>    P: Pos      signed exponent, sealed (1.15)
//   Radix ::= Rad<P>             P: AtLeastTwo   sole constructor, sealed (1.2, 5 routes: 1.12)

pub const trait Numeral {                 // ratified: identity contract
    type Radix:     Radix;
    type Precision: Precision;
    type Exponent:  ExponentForm;
    type Domain:    SignDomain;
}

pub const trait Policy {
    type Quantisation: Quantisation;      // Growth removed from Policy: RATIFIED, closed a second way (1.10, 1.21)
}

pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;
    type Layout:      StorageLayout;
    type Door:         LoweringDoor;      // the float strategy door (1.21)
    // Widening removed: RATIFIED.
}

pub const trait Underflow { /* Gradual | Abrupt, sealed, both change representability (1.16) */ }
pub const trait Specials  { /* the product {NoSpecials, NanOnly, InfOnly, IeeeSpecials}, sealed (1.16) */ }
pub trait NumeralFace {                   // the notation vehicle's face (1.18)
    type Encoding: Bias;                  // unsealed, per-literal, bridges to the sealed tower
    const DISPLAY: &'static str;
}
pub unsafe trait Crosses<N: Numeral>: Lowering {
    // NEW this stretch (1.4): the pair-keyed crossing obligation. Not a law;
    // reads Lowering; D16's safe-blanket-or-unsafe-impl discipline applies.
}
```

`Int` stays dropped. Rewrite cost against the shipped tree remains near zero for the numeral tower itself:
no shipped source names `Adjustment`, `Numeral`, `Bias`, `FullRange`, `UTerm` or `AddWidth`, verified
fresh for this document. What is real and defective in the shipped tree is `arvo-graph`, `arvo-comb`,
`arvo-spectral`, `arvo/src/traits/from_constant.rs`, and `arvo-strategy`'s container dispatch plus its
facade, which section 3 states and which this stretch, for the first time, prices (section 1.25).

### 1.23 The cost model

Unchanged this stretch.

### 1.24 The downstream contract, and the crate table

Unchanged in shape from file 26 onward, with one addition: the real-consumer compile-cost bench, named
since the fourth consolidation as an owed item, is no longer merely owed. Section 1.25's facade-migration
fork is gated on it directly, so it moves from "untouched open item" to "blocking dependency of an
authorized piece of work."

### 1.25 The L0 spine-rule migration: priced, split, one gate performed and the other forked

**New this stretch.** Entry 6 of the live-defect registry (section 3), the forbidden-feature dependency
in `arvo-strategy`'s container dispatch and its facade, was named at the sixth consolidation as the
review's highest-leverage unpriced item. File 65 priced it, and the pricing overturns the sixth
consolidation's own framing at `63:816-817`, which stated the fix touches "every consumer of `Bits`,
`UFixed` and `IFixed`". **That sentence is corrected here: it is false**, measured false, and the
correction is the load-bearing result of this section.

**The workspace rule's own drift entry treats the fix as one thing, and that is what produced two
estimates two orders of magnitude apart.** It is two independent gates.

**The `arvo-strategy` gate is small, was performed on the real crate rather than merely estimated, and
touches no consumer.** Sixteen diagnostic spans collapse to one mechanism (two const fns, `tag_hot_cold`
and `bytes_for_u16`, each appearing twice across four impl blocks in one file, `container.rs`). File 65
rewrote the file to a bucket-as-type shape and ran it: `cargo check --offline --workspace --all-targets`
clean at 19.6s; `cargo test --offline --workspace` reports 658 passed, 0 failed, 9 ignored, identical to
this review's own baseline; compile time neutral at a three-run average (3.48-3.50s baseline against
3.29-3.48s migrated); zero public signature changes, `BitsContainerFor<const N: u16, Sign>` unchanged, the
34 files across seven crates that reference it untouched. **This gate is authorized by `67b`, on the
strength of that measurement, and is not executed**, because the round this review runs inside is open at
TOPIC phase and mockspace v1 sweeps every loose flat file into one archive keyed to the earliest
timestamp; opening a second, unrelated round mid-stream would conflate the two archives. The authorization
stands as recorded work for op to land, either after closing this round or as its own round on its own
branch. One design decision was made inside it, also by `67b`: the **structural derivation ships, not a
per-width table**. Thirty impls, linear in what is instantiated and zero for what is not, checked against
`tag_hot_cold`'s own body at every one of 512 widths with a live negative control (moving one boundary by
one makes the build fail at the expected width, confirming the check is not vacuous). A per-width table is
quadratic in its ceiling (measured: 0.42s at 256 widths, 5.3s at 1024, 116s at 4096, exceeding 25 minutes
at 8192, paid by every build of every consumer forever) and `arvo-toolbox-not-policer.md` forbids a
hardcoded cap below what the substrate dispatches, which `Width`'s own declared sixteen-bit range makes a
real constraint rather than a theoretical one. One condition attaches: the round must confirm the
structural form preserves the `#[diagnostic::on_unimplemented]` refusal-at-unserved-widths story the
current table gives for free; if it cannot, the shape question returns rather than shipping a worse
diagnostic.

**The facade gate is the real work, and its cost is dominated by one predicate rather than by edit
count.** 478 diagnostic spans reduce to 246 distinct lines across eight files, 93% of them one expression,
the logical width (`I + F` unsigned, `1 + I + F` signed). Three routes exist for making that width a type
rather than a computed const. Route X, keeping `I` and `F` as consts and lifting only the computed width,
does not exist: refused six ways across two compiled attempts, once by the arithmetic itself sitting in
const position regardless of the wrapping type, once by `E0119` coherence that only the forbidden full
`specialization` could break. Route Y re-parameterises `UFixed`/`IFixed` on total width and fraction point
directly, so no addition ever appears in type position; it makes roughly 260 edits mechanical (measured by
performing them: 478 to 56 to 25 to 2 to, when a relaxed guard revealed a second wave of type errors the
first pass had been silently short-circuiting, 103 to 102 on 38 lines to 8, with the residual staying one
error class throughout rather than fanning out) and leaves one predicate, `OneRepresentable`, the shipped
fix for the `UFixed<0, F>::ONE` defect this review spent a stretch finding, with no expression under the
permitted feature set except a quadratic two-dimensional impl table. Route Z, the tower's own shape
(`I`/`F` as type-level `Nat`s), makes `OneRepresentable` and its siblings one impl each, structurally, but
needs the numeral tower's own `Nat`/`Pos`/`Cmp` machinery built and unpriced against a real consumer's
build, exactly the kind of composition-cost question file 61 already found a real cliff inside.

**The consumer break is far smaller than the tree's own occurrence count suggests, because consumers write
aliases and three of the four survive unchanged.** `Bits<N, ...>` (770 lines, 91 files across the
workspace), `Uint<N, S>` (24 in arvo, 38 in hilavitkutin, 8 in vehje, 1 in kolli), and `Int<N, S>` (19
across the workspace, and its own internal `N - 1` arithmetic disappears under route Y) all pass through
route Y unchanged: `Uint<const N: u16, S> = UFixed<N, { fbits(0) }, S>` passes `N` straight through, and
`{ fbits(0u16) }` involves no generic parameter, so it was never a refusal site. Only `Fixed<I, F, S>` and
`Signed<I, F, S>`, written directly rather than through an alias, change meaning: seven lines in arvo,
fourteen in hilavitkutin, zero in vehje and kolli, twenty-one total. That is the entire public break, and
it is the fact `63:816-817`'s "touching every consumer" sentence obscured.

**A committed sketch had already established a third of this answer, six days before the sixth
consolidation named pricing as the stretch's highest-leverage item, and nobody in this review had cited
it.** `mock/research/sketches/202607282100_container-projection-without-gce/`, dated 2026-07-28,
`FINDINGS.md` records `WORKS, zero feature gates required` for the same three-build ladder file 59 and
file 62 each independently re-ran. **The sketch is also wrong about one thing, in the direction that would
have set the estimate too low**: it states "the facade's only live GCE constructs are two static asserts,"
and the measurement in this section says otherwise, two of 478 spans. A reader taking that sentence at
face value would have priced the facade at two lines and found out otherwise during the work. This is the
second time in three stretches a universal claim about the shipped tree was made from a partial read
(file 57's "cannot be reproduced," corrected at file 62). **Two conventions are adopted from it.** A
universal "only" claim about the shipped tree owes a whole-crate compile before it ships, symmetric with
`62b`'s "cannot" convention; one `cargo check` with the gate stripped costs four seconds and would have
caught this. And, adopted as standing dispatch practice, the surrounding-directories listing every dispatch
already owes names `mock/research/sketches/` explicitly; a committed sketch holding a third of the answer
sat one `ls` away for two dispatches before this one.

**The estimate.** `arvo-strategy`: one to two days including the round's own ceremony (src CL `## CHANGE:`
blocks, the affected `DESIGN.md.tmpl` edit, the ceiling decision, a review pass), already engineered and
green on the real crate. The facade: one to three weeks, and the spread is decisions, not edits. Route Y's
`OneRepresentable` fork needs its own round with the review's own two-independent-reads convention before
it hardens; the API break is public and, per `no-legacy-shims-pre-1.0.md`, lands in one commit with no
transition period; seventy-one doc-comment lines name a width and doc comments compile; the downstream
edit is twenty-eight lines in hilavitkutin, zero in vehje and kolli, each needing its own round and PR.
Two to four weeks total for one engineer, correcting both wrong figures previously on offer (an
unscoped afternoon, wrong by roughly forty times; a quarter, wrong by roughly four) in opposite
directions, both wrong for the identical reason: neither was formed against a whole-crate compile.

**Sequencing, and it is better than either-shape-first.** `arvo-strategy` needs no coexistence mechanism,
because it has no public-facing change at all; it lands alone, first, cheap, reversible, and blocks
nothing. The facade must land atomically, because `UFixed`'s parameter meaning changes and no shape lets
the old and new spellings coexist. So: `arvo-strategy` first; the four facade decisions, each through its
own round; the facade in one commit; the consuming repos' PRs sequenced immediately after with `Cargo.toml`
git refs updated per `branch-pr-flow.md`.

**The facade fork is set presumptively to route Z, gated on the compile-cost bench, and this is the call
op is most likely to want to revisit.** Route Y makes 99% of the work mechanical and leaves one predicate
with no honest expression; route Z makes every predicate trivial at the cost of a width computed as a
type-level sum at every instantiation, a composition nobody has priced against a real consumer's build.
**The real-consumer compile-cost bench stops being optional and becomes the gate**, in `mock/benches/`
under the harness, with an exit condition stated before it runs. If it shows a cliff a real consumer
cannot eat, route Y returns, with `OneRepresentable`'s quadratic-table hole as its named and accepted cost,
and the fork goes to op with both numbers attached.

## 2. The lead designer's calls

**Op's four checkpoints, restated, unchanged.** D69 ratified: identity is parameterised in mathematical
coordinates, not encoding coordinates (`30b`). D39 held: membership through algebraic structure stays a
decision pending a positive characterisation of its honest content (`30b`). The novelty posture (`34b`):
attempt what looks unsolvable, distinguish "cannot, because impossible" from "cannot, because nobody has
done it," treat the second as an absence to fill. Widening leaves `Lowering`, `Growth` leaves the law key,
the finest-view mechanism replaces the three-relation fork, all three ratified (`39b`). The value-unique
encoding ratified in full, division held, every claim grounded (`44b`). The convergence directive and the
novelty posture hold unchanged through every checkpoint since, in the same words each time: the intent
outranks every instruction, is vague on purpose, and only op's calls are final, and even those go stale.

**The persona checkpoints, five, made overnight across five separate nights, each explicitly not op's.**
All five carry the same provenance statement recorded first at `48b`: op delegated the checkpoint
mechanism to his own persona, dispatched at Fable tier, for the duration of each night's absence, and
every call inside all five dies the moment op reads it and says otherwise.

**`48b`, after file 48, five calls.** The grade projection adopted as spec shape with its projection-chain
constraint, seal and join algebra. The numeral notation intent ratified, vehicle left sketch-decidable,
closed by file 61 (section 1.18). The evaluation sentence adopted as the fused block with the `Precise`
combinator surface. `Int` dropped from the ratified table, still open, one line to restore. Direction set
for the float model, the exponent second read, tick 3, and the test debt.

**`53b`, after file 53, five calls.** The two-term cost model adopted with the cliff on the attempt list.
The reassociation licence's design shape adopted, `float_algebraic` sent through its own vetting procedure
(now complete, section 2 below). The L2/L3 consumer-typing dispatch and the decoder-ring-plus-face fixture
adopted as next work, fulfilled and extended. `49:117`'s defect corrected; the table-diff obligation
adopted, executed by every consolidation since. The strategy-axis refinement on the default lowering
adopted, compiled in full by file 59, one naming residual remaining (section 1.21).

**`57b`, after file 57, six calls, one now permanently corrected by `62b`.** The strategy-door mechanism
adopted with a presumptive per-preset table, compiled and settled since (file 59, closed further by file
64). The bench-harness fix graded as changing the debt ledger, not any conclusion. **The `unreproducible`
ground adopted on file 57's finding, with a targeted re-derivation demanded for the width-ceiling
dependent: this call's own factual premise was wrong**, corrected by `62b`, standing correction, resolved
in the good direction. `FromConstant`'s intent adopted, vehicle held for its own second reads, unchanged
and still open. Three ratified-table edits adopted (`Radix` sealed, `Specials` as a product, both exponent
lines corrected), each still standing, one line each to restore. The cadence correction adopted as the
standing loop shape, followed by every stretch since.

**`62b`, after file 62, seven calls.** The falsified-premise finding on `unstable-features.md`'s two
remaining `generic_const_exprs` gates adopted, wording proposed to op, ban untouched; the finding stands
and this stretch prices the remediation (section 1.25). The three clause 5.2 sharpenings and the
presumptive decimal `Canonical` default adopted (section 1.17, and this stretch's file 64 confirms the
default structurally sound). The notation vehicle closed with the bisected ceilings. The `TotalOrd` trait
split and the new spectral live defect adopted, with the consumer-pressure framing corrected. The compiled
strategy-door table adopted, replacing the presumptive one, with the softened bench sentence.

**`67b`, after file 67, the widest-ranging of the five, made in op's place.** Adopts the transfer-ground
scheme as file 66 stated it, extended with file 67's container-class coordinate, and packages three
rule-wording edits for `unstable-features.md` rather than editing the ratified rule itself (section 1.19).
Adopts the crossing contract's statement-0-as-precondition and family framing in file 67's sharper form
over file 66's own first answer (section 1.4). Adopts the layer-keying rule in full, closing two of the
notation's three residuals and reclassifying three separately-recorded defects as one at three layers
(top of document, section 1.18). Authorizes the `arvo-strategy` migration gate, structural derivation over
the per-width table, but records it as not executed this stretch, phase-blocked rather than judgment-
blocked (section 1.25). Sets the facade fork presumptively to route Z, gated on a real-consumer compile-
cost bench that must now be built. Adopts two conventions (a universal "only" claim owes a whole-crate
compile; dispatch briefs name `mock/research/sketches/` explicitly). Closes five of the seven owed second
reads outright: `Unbounded` ships with the sealed `Arity` wrapper, settled; `float_algebraic` vetting is
complete, `ALLOWED`, row drafted; `Hot`'s refusal call is settled, its presumptive marker dropped;
`TotalOrd` injectivity is closed over the full eight-bit matrix; `foldnum`'s characterisation is spec text
(sufficient always, tight exactly for power-of-two arities and wide precisions, loose by at most one bit
elsewhere), with one compile against the real `Exponent`-fixed contract still owed. Leaves genuinely open,
and says so: the membership hold (D39, still op's, now sharper); the decimal `Canonical` default's last
confirming compile; `IeeeDefault`'s naming and `Hot`'s default environment, bundled as one item for op;
division, held unchanged; per-application against per-value-moved event counting, declined a fifth time.

**Loudest for op's morning read, consolidated across all five checkpoints and the four deliverables since
the sixth consolidation, current status noted where a later checkpoint or a later file already resolved
an earlier item. This list is this document's own synthesis.**

1. `Int` dropped from the ratified table (`48b`). Still open, one line to restore.
2. The exponent bounds spelled as types, both `Ranged` and `Implicit` (`53b`, `57b`). Still open, one
   line to restore.
3. `Radix` sealed as `Rad<P>` (`57b`), standing on five compiled routes (`62b`). Still open, one line to
   restore.
4. `Specials` resolved to a product rather than a chain (`57b`), witnesses primary-sourced (`62b`). Still
   open, one line to restore.
5. **Closed.** The strategy-door table, compiled and settled since file 59, with `Hot`'s refusal call
   confirmed this stretch (file 64). One narrow item remains: `IeeeDefault`'s naming and default
   environment, bundled below at item 11.
6. `FromConstant`'s breaking-change fix, adopted in intent, vehicle still held for its own second reads
   (`57b`, unchanged).
7. **Resolved, no action needed.** The `unreproducible` ground's founding exhibit was never actually
   unreproducible; `57b`'s escalation was itself the error, corrected by `62b`.
8. **A ratified rule's own transfer sentence and its enumeration of observable-instantiation mechanisms
   are refuted and incomplete, respectively, and its supporting measurement is stated less precisely than
   what was actually measured.** Three compiled findings, three proposed clauses, packaged as one edit
   set with the compiled evidence attached to each line: the last-sentence correction (necessary promoted
   to sufficient, refuted by two counterexamples), the penultimate-sentence third-way clause (container-
   class const-tag dispatch, shipped, permitted, missing from the enumeration), and the step-budget clause
   (the wall is a total-step-count budget, not a bit-width ceiling). The ban itself is untouched by all
   three. This is the largest single item on this list.
9. The bench table softened from "13x to 17x at every point" to "ten to seventeen across two runs"
   (`62b`); the repair is already committed. No action needed beyond noting it.
10. The decimal `Canonical` default, presumptive since `62b`, now structurally confirmed against the
    `Strategy = Warm` precedent (file 64); one cheap confirming compile is what remains between reasoned
    and closed, named in the verification dispatch below. One line for op to kill the default entirely if
    he reads it otherwise.
11. **New.** `IeeeDefault`'s name asserts a verified-standards claim the design's own missing build layer
    cannot back, independent of which concrete environment op picks as `Hot`'s default. Rename or annotate
    the name; separately, pick the default environment. Both are op's, bundled as one item.
12. **New.** Membership's mechanism is sound for arvo's own numerals; its stated justification for
    uniqueness is false against the full ten-member vocabulary D38 ratified (file 64). Recommendation:
    scope the "finest" fact to the real/Cayley-Dickson chain explicitly, give the other branches
    independent predicates. Final hardening is op's, since the hold (D39) was his.
13. **New.** `float_algebraic` vetting is complete: `ALLOWED`, now in Final Comment Period with
    disposition to merge, no soundness hole, cleanly separated from its unsafe `fadd_fast` sibling family.
    Row drafted, rides in the rule package with item 8. Feature-table edits are always flagged to op.
14. **New.** The crossing contract's `Crosses<N: Numeral>: Lowering` mechanism and the transfer-ground
    scheme are both adopted presumptively, structurally sound and compiled against every check run so
    far, each one expert's shape wanting its second read before it hardens. No action required beyond
    awareness; neither touches a shipped crate.
15. **New.** The `arvo-strategy` migration gate is authorized (structural derivation over a per-width
    table, measured compile-neutral, zero consumer edits) but not executed, because the round is at TOPIC
    phase. Landing it, either inside this round's close or as its own round, is op's first move whenever
    he chooses to take it.
16. **New.** The facade migration's other gate is real work, priced at one to three weeks, with the
    fork between two live routes set presumptively to the tower's own shape and gated on a real-consumer
    compile-cost bench that does not yet exist. That bench is now a blocking dependency rather than an
    optional item; if op wants the cheaper, less elegant route instead, that is a call he can make now
    without waiting on the bench.
17. Division stays held, unchanged, whenever op picks it up.
18. Per-application against per-value-moved event counting: declined a fifth time running, genuinely
    op's.

## 3. The live-defect registry

For defects in the shipped tree, as against findings about the still-unbuilt design. Entries 1 through 5
and 7 carry forward unchanged from the sixth consolidation; entry 6 is substantially rewritten this
stretch with the pricing and the authorization file 65 produced.

**1. `upward_rank` and `bin_pack` silently return wrong orderings under both shipped presets.** Tree:
`arvo-graph/src/rank.rs:34-88`, `arvo-comb/src/binpack.rs:44-63`. Unchanged. Grounded `tree`, `pin`.

**2. `FromConstant` accepts an unrepresentable constant and silently produces a wrong bit pattern, or
panics.** Tree: `arvo/src/traits/from_constant.rs:40`, `arvo-numeric-contracts/src/lib.rs:85-88`.
Unchanged, vehicle still held for its second reads.

**3. `arvo-graph/tests/rank.rs` never enters the breaking path.** Unchanged.

**4. `arvo-spectral`'s ten test files never exercise an arvo numeral.** Unchanged.

**5. `mock/benches/src/main.rs` could not run any bench at all. Fixed.** Unchanged, the fix predates this
stretch and stands.

**6. `arvo-strategy`'s shipped container dispatch, and its facade, are load-bearing on the forbidden
`generic_const_exprs` feature, not on a stale annotation, and the remediation is now priced and split
rather than named unscoped.** Tree: `arvo-strategy/src/lib.rs:11`, `arvo-strategy/src/container.rs:254-
258`, `arvo/src/lib.rs:25`, `arvo/src/ufixed.rs:35-36`. Sixteen refusal sites in `arvo-strategy` reduce to
one mechanism (two const fns, four impl blocks). Four hundred seventy-eight in the facade reduce to 246
distinct lines, 93% one expression. The `arvo-strategy` half is measured, performed on the real crate, and
authorized for landing (structural derivation, thirty impls, compile-neutral, zero public signature
change, zero consumer edits); it is not yet executed in this round because the round's phase forbids
opening a second one alongside it. The facade half remains real work, one to three weeks, forked between
two live routes (route Y, mechanical but leaves the `OneRepresentable` guard with no expression under
permitted features; route Z, the tower's own shape, trivial predicates but an unpriced type-level
composition cost) and set presumptively to route Z pending a real-consumer compile-cost bench that does
not yet exist and is now a blocking dependency. `63:816-817`'s prior claim that the fix touches "every
consumer of `Bits`, `UFixed` and `IFixed`" is corrected here: measured false, twenty-one call sites are the
actual public break. Full pricing at section 1.25. Grounded `tree`, `pin`, `flags`.

**7. `arvo-spectral`'s degenerate-component classification is decided by NaN payload arithmetic rather
than by anything the design calls a value.** Tree: `arvo-spectral/src/partition.rs:59,156,181`. Unchanged
in content; **this document's own bridging note**: per this stretch's layer-keying rule (top of document),
this entry and entry 6's sibling `TotalOrd` finding at `arvo/src/traits/total_ord.rs` share one cause with
the notation's face-keyed refusal (section 1.18), a value-keyed operation consuming its operand without a
canonicalising projection as the only door. The entries stay listed separately here because they sit at
different tree positions and need independent fixes; the shared diagnosis is recorded so a future reader
does not treat them as three unrelated mechanisms. Grounded `tree`, `pin`.

## 4. What is open

**Closed this stretch, listed once so the next member does not re-open them.** The crossing contract's
`Underflow = Abrupt`-under-unnormalised-significand question (exactly one meaning, section 1.4). Two of the
notation's three residuals (`Adjustment`'s own entry point; cross-call-site face identity, closed without
a mechanism; section 1.18). Five of the seven owed second reads: `Unbounded` (ships with the sealed
`Arity` wrapper), `float_algebraic` (vetted `ALLOWED`), `Hot`'s door refusal call, `TotalOrd` injectivity
(the full eight-bit matrix, twice, zero counterexamples), and `foldnum`'s characterisation (sufficient
always, tight conditionally, the general behaviour stated). The `arvo-strategy` migration gate's own cost
question (measured, performed, authorized). The ratified transfer sentence's factual status (refuted, with
the replacement scheme adopted).

**Owed second reads, narrowed to two.** The membership candidate's final hardening, held exactly as D39
left it, genuinely op's. The decimal `Canonical` default's one remaining confirming compile (an explicit
non-default declaration, to verify the parameter swap is trivial rather than assumed to be). Both are
named in the verification dispatch below.

**The verification dispatch, one bundle, the next member's first pick per `67b`.** Statement 0 against
`quantize` and `roundToIntegralExact` (flagged forward twice, performed by neither). `foldnum` compiled
against the real four-member `Numeral` contract with `Exponent` held fixed. The exact `foldnum` closed
form, `1 + floor(log2(A * (2^p - 1)))`, built as a type-level computation. The non-default `Canonical`
compile. `Crosses`'s own second read. The nine-bit `u16`-class companion model for the container-class
transfer coordinate.

**The highest-leverage item for the next stretch, unchanged in kind from the sixth consolidation but now
with a real answer waiting to be landed rather than priced.** The real-consumer compile-cost bench that
gates the facade migration fork. It is now a blocking dependency of an authorized piece of design work
rather than a standing, deferrable open item.

**The float model's residuals, unchanged.** The model-width transfer argument extended to a `Ranged`
numeral's two-width shape is now the subject of section 1.19's own scheme rather than an unbuilt
extension; what remains open within it is the precision axis's own `unargued` status (no induction
argument exists) and the container-class coordinate's companion model above.

**Decimal's residuals, unchanged.** The `InfOnly` `Specials` witness is still unfound, carried `unknown`.
The `10^20` figure stays open, plausibly radix-ten-specific, unbuilt and unchecked. The reciprocal-table
strength reduction for the radix-ten quantiser's dominant division term, named as an attempt rather than
built.

**Codegen-flag audit, still not fully done.** Unchanged.

**Unchanged from the sixth consolidation, untouched this stretch.** The reduction firing site and whether
`FullRange` survives as its own named constructor. The dither-versus-`Refuse` interaction. The
per-application-against-per-value-moved event-counting sub-item of the fold's grade, genuinely op's,
declined five times running. `SC_WRAP<n>`/`SC_WRAP_SM<n>` with `n_bits > 0`. Richer canonicalisation's
branchlessness and cross-word bitpacked field extraction. `DatumDeterministic`. The `Gcd`-for-a-local-
`Rhs`-on-a-sealed-`Self` coherence question.

## 5. The droplist

Carried forward from files 26, 40, 49 through 58, and 59 through 62; nothing this stretch resurrected any
of it. New entries follow.

The last sentence of `unstable-features.md`'s transfer argument, "without them, monomorphisation is
uniform and the transfer is sound": refuted twice by compiled counterexample with the bans in force,
promoting a necessary condition (implementation uniformity) to a sufficient one for a different claim
(property uniformity) the same rule's own source had already named unproven (section 1.19).

The two-mechanism enumeration of ways an instantiation can be observed (`specialization`, `TypeId`) as
exhaustive: refuted by a shipped, permitted third mechanism, const-tag container dispatch, demonstrated
with a property true at eight bits and false at nine, no gate, one parametric body (section 1.19).

The "refused at nine bits" wall as a width ceiling: refuted; it is a total-step-count budget, and a
cheaper predicate compiles clean at nine and refuses one bit later (section 1.19).

File 66's "exactly one cell of the matrix leaks": refuted, the matrix held `Specials` fixed; the full
product leaks at six of eight cells, and the correct framing is a family of configurations, not a cell
(section 1.4).

Widening the crossing-contract's target through the quantiser as an alternative to statement 0's
obligation on the encoding: refused on every escaping datum tried, without exception, against a
2,701-value negative control confirming the quantiser is otherwise the identity; not a preference, an
arithmetic fact (section 1.4).

Cross-call-site face identity as something needing a mechanism: refused; nothing that affects compilation
is keyed on the face, per-site display is the better diagnostic, and unifying faces would build the exact
false-refusal failure the layer-keying rule forbids (section 1.18).

A committed sketch's universal claim that the facade's "only live GCE constructs are two static asserts":
refuted by a whole-crate compile, two of 478 (section 1.25).

`63:816-817`'s claim that the facade fix "touches every consumer of `Bits`, `UFixed` and `IFixed`":
refuted; three of the four things a consumer writes are unaffected, and the real break is twenty-one call
sites (section 1.25).

Route X for the facade migration (const-keyed projection, only the computed width lifted to a type,
`I`/`F` staying bare consts): refused structurally, six ways across two compiled attempts, closed rather
than left open (section 1.25).

## 6. Verification

Every claim tagged compiled, measured or reasoned in files 64 through 67 traces to a probe, a sketch
directory, or a committed migration artifact, each carrying its own outcomes file or diff, as stated at
the top of each source file and cross-checked for this document. The canon gate reproduces fresh from the
repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth` in place of the first pattern both exit 1, empty. `cargo test --offline
--workspace`, run against the tree this document describes, reports 658 passed, 0 failed, 9 ignored,
matching every one of files 64 through 67's own independently reported counts; files 64, 66 and 67 touched
no shipped crate, and file 65's whole-crate `arvo-strategy` migration, performed on a copy of the tree
outside the repository, reports the identical count against the migrated crate, matching baseline exactly.
The toolchain is `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`.

The table-diff obligation was executed on this document, by its own author, before it stands: every table
above was checked against the prose of the section it sits in and against the source file that established
each row, and the correction named in the table-diff paragraph at the top of this document (the assembled
trait table in section 1.22 gaining the `Crosses` trait, and the live-defect registry's entry 6 being
rewritten rather than patched, checked against file 65's own line numbers rather than against this
document's paraphrase) were caught by that check rather than by a source file naming them directly.
