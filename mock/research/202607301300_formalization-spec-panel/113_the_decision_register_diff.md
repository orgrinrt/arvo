# The decision register, diffed against the standing base

**Date:** 2026-08-05
**Position in the panel:** after `112_the_op_material_sweep.md`, whose section 6.1 names this diff, states it
has never been run, and states it covers a population of op's own text larger than the one that sweep walked
(`112:487-505`).

File 109 diffed consolidations against each other. File 110 restored from 109's list. File 111 found that
restoring from an audit inherits the audit's recall. File 112 swept the panel's own archive for op material
and found the working list missing eleven files of twenty-three. Every one of those four instruments is
bounded by the panel directory. The numbered decision register is not in the panel directory. It has never
been diffed against anything.

**The headline, stated before the method, and it is structural rather than a count.** `110`'s own provenance
section enumerates the ratified rung by name (`110:154-160`) and lists twenty-three op checkpoint files.
**The two topic files are not on it.** They are op's own text, they carry seventy-five numbered decisions
across their round and its inherited file, and the document that defines what governs the panel does not
name them as governing. That is why nobody ran this diff: the standing base's own definition of its oracle
excludes the register. `110` cites the talk file exactly once in four thousand nine hundred lines
(`110:2089`, a quotation of the preset intents) and cites the spec file zero times.

---

## 1. What the register actually contains, verified rather than inherited

The count of forty-seven was re-derived from source on 2026-08-05 rather than taken from `112`, and it
reproduces: `grep -oE '\bD[0-9]+\b'` over both topic files, sorted unique, returns **47 distinct tokens**.
The claim that `110` names thirty-six does not reproduce; the same command over `110` returns **35**.

Three corrections to what those forty-seven are, and each changes the diff.

**D3 is not a decision.** Its three occurrences in the talk file are all the question-grid row `D3`
(`202607301100_topic.the-formalization-talk.md:74`, "Whether the ladder's rungs are named now or grown as
they are needed"; `:1792`, its own section heading; `:1803`, "Closes E2 and D3"). The grid runs A1 through
A6, B1 through B3, C1 through C6, D1 through D3, E1 through E5, and its `D` rows collide with the decision
prefix. There is no decision D3 cited anywhere in either file. The register is **46 decisions**, not 47.

**D1 and D2 each carry two meanings inside the same two files.** As question-grid rows they are
"whether the law table is hand-written per pair" and "multiplication, rounding, and distributivity"
(`talk:72-73`, `:1784`, `:1788`). As decision citations they are the forbidden-features pair from round
`202607282100`: `generic_const_exprs` and full `specialization` forbidden, and const-to-type through
per-width impl tables (`talk:182`, `:655`, `:776`, `:976`, `:996`, `:1763`).

**And the collision is worse one level down, in the file the register actually lives in.** The two topic
files *establish* D53 through D75 and *cite* the other twenty-three. The cited ones are defined in
`202607301000_topic.inherited-state-from-the-formalization-round.md`, the third flat file of this same
active round, which the dispatch's two-file scope does not name. That file contains **two independent
D-numbered sequences that overlap**: D1 through D13 at `:495-644` (the dimensional foundation, the four new
crates, the hlist, curves) and a second D1 through D4 at `:763-798` (the forbidden features, the container
projection, the gate sweep, the vetting audit), with D14 resuming at `:820`.

Both sequences are live in both documents, pointing at different decisions:

| token | in the topic files | in `110` |
|---|---|---|
| `D1` | `generic_const_exprs` and `specialization` forbidden (`inherited:763`, cited `talk:655`) | `arvo-shape`'s dimensional foundation (`inherited:495`, cited `110:2590`, `110:3091`) |
| `D2` | container projection to typestate (`inherited:773`, cited `talk:976`) | the four new crates (`inherited:499`, cited `110:2591`) |

A reader following `110:2590`'s "`arvo-shape` | D1-D4" into the talk file's D1 reads a forbidden-feature
ruling as a shape-crate ratification. **A register whose identifiers are not unique is not a register**, and
this is the defect to fix before any of the dispositions below are acted on, because every one of them is
keyed on a number.

**Composition of the 46.** Twenty-three established in the two topic files (D53 through D75, every one
marked "Decision (op, 2026-07-30)"). Twenty-three cited from prior rounds (D1, D2, D11, D16, D17, D23, D27,
D28, D31, D32, D33, D34, D36, D38, D39, D40, D45, D47, D48, D49, D50, D51, D52).

---

## 2. The counts

| disposition | count | meaning |
|---|---|---|
| carried by number | 15 | the number appears in `110` and denotes the same decision |
| carried without its number | 9 | content present, identifier absent or colliding |
| carried but weakened | 4 | present, with a clause of op's own call lost |
| superseded | 7 | later evidence or a later op call moved the grounds |
| absent | 11 | in `110` nowhere, in no droplist, on no open list |

Of the seven superseded, **three were superseded inside the register itself** (D55, D57, D62, all overturned
by later decisions in the same transcript), so `110` owes them nothing. **Two are stated** (D69, and D65 by
the same sentence, mis-keyed, section 3.4). **Two are silent** (D68, D31), which is the shape `112:108-110`
names: not a drop, and not a record either.

Nineteen of the forty-six tokens appear in `110`, but two of those nineteen are the D1/D2 collision and
denote other decisions, so **the agreement rate between register and standing base, by number, is 15 of 46**.
Twenty-seven are absent by number. Of those twenty-seven, thirteen are carried in content, which is the
useful result: the absences are real but they are a third of what the number diff alone reports.

---

## 3. Absent, ranked by what depends on them

### 3.1 D72, the crate split, and it is not carried anywhere

Op, `talk:1723-1741`:

> **D72. One crate per contract, and `arvo-strategy` keeps only the presets.** Decision (op, 2026-07-30).
> Closes C1 through C4 together.

with a six-row table naming `arvo-numeral`, `arvo-policy`, `arvo-lowering`, `arvo-strategy`, `arvo-numeric`,
restated at `202607301200_topic.the-formalization-spec.md:291-300` with `arvo-algebra-contracts` added as a
seventh row.

**`arvo-numeral`, `arvo-policy` and `arvo-lowering` each return zero hits in `110`**, searched 2026-08-05,
and each was searched again as `arvo numeral`, `numeral crate`, `policy crate` and `lowering crate` before
the negative was recorded. `arvo-strategy` returns six hits, every one about the shipped tree's container
dispatch or its `RANK` constant (`110:370`, `:2173`, `:2223`, `:2517`, `:3837`, `:3856`) plus
`110:3703`, "the `arvo-strategy` migration authorization withdrawn" at `68b`.

**This corrects `112:499-500`, which states that "**D72**, the crate split, is section 1.25's crate table".**
It is not. Section 1.25's table (`110:2587-2599`) is the eleven-row periphery taxonomy, its rows are
`arvo-capacity`, `arvo-shape`, `arvo-geom`, `arvo-platform`, `arvo-float`, the predicate concept,
`arvo-pseudorand`, `arvo-container`, `arvo-bitfield`, `arvo-num-systems` and `notko-hlist`, and its
"Round decisions" column is keyed on the *inherited* file's numbering. Not one of D72's six rows appears in
it. `112` recorded that claim as demonstrable and it is false; it is offered here as an instance of the
defect `112:571-574` names about its own greps, arriving in the very section that names it.

**What is lost.** The only statement in the record of which crate declares which contract. `110:4673`
restores as an open question "what `arvo-numeric` ends up containing once the numeral, policy and lowering
definitions move out", which presupposes the move without ever stating it, so a reader meets the
consequence of a decision whose text is nowhere.

**What depends on it.** The taxonomy round. Op's sequence at `68b:14-21` is settle the canon, then a design
round creating the taxonomy and its docs, then stubs, then per-piece implementation, and `110:3749` already
defers the platform crate's name to that round. The round that builds the crate structure will be briefed
off `110`, and `110` does not carry the crate structure. This is the single largest absence in the register
because it is the one the next scheduled unit of work reads first.

### 3.2 D52, the call that makes the presets a default rather than a closed set

Op, cited at `talk:652` as fixed going in:

> Compositions are public and bindable; semantic names and presets are the default documented path, not the
> only one

The full call is `inherited:2110`. `D52` returns zero in `110`; `bindable`, `compositions are public` and
`default documented path` all return zero, searched again as `public composition`, `consumer marker`,
`arbitrary combination` and `preset is not the only` before recording.

**What is lost.** The standing base carries two ratified preset tables (`110:2099-2133`) and no sentence
saying they are one path among several. A reader of `110` alone reads four presets as the surface.

**What depends on it.** Thread A, `110:4110-4127`, whose strongest measured shape is "nominal constructors
at every position a consumer selects, combined with small per-axis modifier types", is a mechanism for
consumers naming their own compositions and is unintelligible without D52. And `110:4670-4672` restores as
open "what a preset is mechanically (a plain type alias over one fixed composition, or a nominal marker type
from which axes are projected)", which is the round's own B3 row (`talk:1717-1719`, status open) arriving
back on the list from a different source, with the call that constrains its answer absent.

### 3.3 D66 and D67, the conventions, dropped as a mechanism on a reason that covers half of them

Op, `talk:1459-1465`:

> **D67. arvo writes the abstraction; every established convention ships as an optional feature defining
> that convention's vocabulary as aliases over it.** Decision (op, 2026-07-30). This generalises D66 from
> the quantisation modes to a standing principle across the crate family: `conv-ieee754`, `conv-systemc`,
> `conv-matlab`, `conv-amd-vitis`, `conv-flocq` and whatever else earns one, each off by default, each
> containing type aliases and nothing else.

`110:4738-4740`:

> The conventions mechanism (`conv-ieee754` through `conv-flocq`) is not restored as a mechanism, because
> op's standard from `13c` is what it was reaching for and section 0.1 states that directly; the two
> unrepaired gaps it carried are restored to the open list as the adequacy question that standard's own test
> poses.

**The disposition is stated, which is more than most of this list gets, and the reason covers one half of a
two-half decision.** D67's second half is the falsifiable expressibility test, and `110:79-81` does carry it
in op's own later words from `13c`: "MATLAB, IEEE 754, SystemC and the rest are not inspirations to borrow
from; they are a **test**." That substitution is op-on-op and later wins, correctly.

D67's *first* half is a shipping mandate: what the crate family contains, feature-gated, off by default, per
`arvo-toolbox-not-policer.md`'s "arvo ships one structure and does not decide which vocabulary a consumer
arrives thinking in" (`talk:1404-1405`, restated `spec:276-278`). `13c`'s standard is an acceptance test for
the review. It says nothing about what ships. **The shipping half is dropped with no successor and no entry
saying so**, and `110`'s sentence reads as though the whole decision were absorbed.

**What depends on it.** The same taxonomy round as D72: the `conv-*` features are cargo features on named
crates, and they are the concrete artifact of the one test `110:0.1` says every design question is answered
against.

### 3.4 D65's supersession is stated under the wrong number, and D68's is not stated at all

These are one finding in two halves, both about the identity contract, and together they are the reason a
reader of `110` cannot reconstruct how `Numeral` got its current members.

**D65** (`talk:1394-1400`): "`Numeral` carries `ExponentForm`, `Adjustment` and `Sign` ... Precision and
minimum exponent stop being parameters, because both derive from the field width, which is how IEEE defines
its interchange formats."

**D68** (`talk:1529-1545`), which amends it: "`Numeral` carries four flat members. Decision (op,
2026-07-30). `ExponentForm`, `Adjustment`, `Bias` and `Sign`", chosen flat after op's own question about
grouping, with `talk:1509-1527` recording why flat won.

`110:488`:

> **D69 was overturned by op at `30b`**: identity is parameterised in mathematical coordinates, not encoding
> coordinates. Precision and the exponent bounds are primitive; total width, the hidden bit, and field
> encoding are derived on the physical side.

**The sentence that overturns is D65's, not D69's.** "Precision derives from the field width" is D65's own
reasoning verbatim; D69's content is the ten-axis table with `LogicalWidth` on `Numeral` (`talk:1621-1641`).
Both were overturned at `30b`, and `110` records one number for both, so a reader tracing D65 finds no
disposition and a reader tracing D69 finds a disposition that quotes a different decision.

**D68 is superseded silently.** `110:520` declares `pub struct Implicit<E: Exponent, A: Adjustment, B:
Bias>;` and `110:2468-2473` gives `Numeral` as `Radix`, `Precision`, `Exponent`, `Domain`. `Adjustment` and
`Bias` are nested inside the exponent form, which is neither of the two options op chose between. `110`
argues the nesting at `110:563-568` from `Underflow` alone and droplists block floating point as evidence
for it at `110:4285-4289`, and **nowhere says that a ratified flat call existed.** The one half of D68 that
survives by content is its closure gate, carried unnumbered at `110:973` and `110:984`: "the shipped
`AddClosed` gate on `Bias = Zero`".

**What depends on it.** The identity contract is the design's first section and the thing every later
section quantifies over. A future member re-deriving whether `Adjustment` should be flat has a ratified
answer in the register and a contrary shape in the standing base, with nothing connecting them.

### 3.5 D56, the naming rule that governs everything the taxonomy round will mint

Op, `talk:399-404`:

> **D56. No gratuitous abbreviation. Full, legible, recognisable words for every member.** Decision (op,
> 2026-07-30). An abbreviation is acceptable only where it is the stable form nearly everyone in the field
> already recognises. Coining short forms of words that were already a sensible length is not arvo's style
> and has never been. This applies to the whole round's output, not to one draft.

with an applied rename table at `talk:408-418` (`Under` to `Underflow`, `Over` to `Overflow`, `Round` to
`Rounding`, `Grow` to `Growth`, `Total` to `LogicalWidth`, `fexp` to `canonical_exponent`).

Searched in `110` on 2026-08-05: `gratuitous` 0, `abbreviation` 0, `abbrevia` 0, `full word` 0, `short form`
0, `legible` 1 (unrelated). Re-searched as `spell in full`, `spelled in full`, `naming style` and `coined`
before recording. `110`'s naming section (1.26, `110:2652-2700`) is a different principle entirely, adopted
at `90b` and confirmed at `108b`: a name may promise behaviour only where the design names the verifier that
checks the promise.

**Absent, no droplist entry, not superseded.** The two rules are orthogonal; `110:2654-2662`'s principle
constrains what a name may *claim*, D56 constrains how a name is *spelled*. The taxonomy round mints every
name in the design and will do it off a standing base carrying one of the two rules.

### 3.6 D70, the derived determinism marker

Op, `talk:1648-1654`: "**D70. `Deterministic` is a derived marker over the composition.** ... A blanket impl
keyed on the composition makes that qualification structural. The marker holds for a composition, not for
arvo, so the claim a consumer can rely on is exactly the claim the type makes."

`Deterministic` returns one hit in `110`, at `110:4100`, as `DatumDeterministic` in a list of items carried
forward untouched. `determinis` returns two, neither about the marker: `110:1530` is about a register's
nondeterminism and `110:3768-3769` cites D49's determinism argument for the FLX-family reinstatement
question. Re-searched as `derived marker` (two hits, neither this) and `keyed on the composition`.

**Absent.** D49 survives by number as the argument's source; the mechanism it was given is gone. Note that
its sibling D74 survives (section 4.3), so the standing base carries one of the two derived markers op
declared in the same stretch and not the other.

### 3.7 D23, D32, D33, D48: four placement and surface calls with no trace

Each returns zero by number in `110` and zero on a content search run twice.

**D23** (`inherited:1087`, cited `talk:1736` and `spec:304`): `Identity` and `SignedIdentity` move to
`arvo-algebra-contracts`. `SignedIdentity` returns zero in `110`; `Identity` appears twice as a bound name
(`110:1666`, `110:3138`), never as a placement. Feeds D72's crate split directly, and the spec file names
D23 and D27 jointly as the calls that "had already started moving pieces out" of `arvo-strategy`.

**D32** (`inherited:1324`): the marker family splits on what it describes. **D33** (`inherited:1330`): the
width const fns follow their subject. `IntegerLike`, `FractionLike`, `FloatLike` and `BoolLike` all return
zero in `110`. Both are inputs to D73 (section 4.4), which is itself half absent.

**D48** (`inherited:1894`, cited `talk:653`, `talk:661-662`, `spec:316`): width stays a const parameter
publicly, `Bits<13, Hot>`. `Bits<` returns three hits in `110`, all about a column's extent or the shipped
tree (`110:3014`, `:3877`, `:4667`). D48 and D31 jointly are the "public spelling does not change"
constraint the spec file hangs the whole alias story on (`spec:315-318`), and neither is in `110`.

### 3.8 D75's rename, and the half of it that did survive

Op, `talk:1802-1808`: "**D75. `Combine<Op>` is `Magma<Op>`, and the ladder is named in full.** ... A magma is
a set with a binary operation and no law claimed; a semigroup is a magma whose operation is associative."

`Magma`, `Combine`, `Semigroup`, `Monoid` and `semigroup` all return zero in `110`. `Dioid` returns one, at
`110:1164`, which carries the *depth* half under D47's number: "This is not grounds to drop the `Dioid` rung
under D47 (the ladder goes as deep as the theory does)". So op's reconciliation of D75 with D47 survives in
`110` attributed entirely to D47, and the rename that occasioned it is gone along with every rung name below
`Dioid`.

---

## 4. Carried but weakened, with both sides quoted

### 4.1 D54: the test is invoked by name and stated nowhere

Op, `talk:352-356`:

> The test that separates the two columns, stated so later additions sort themselves: change the axis and
> ask whether the set of representable values changed. If it did, the axis is identity. If the same values
> are still representable and only the arithmetic differs, it is policy.

restated as the round's own framing at `spec:33-36`, "it is the test that sorts any axis added later".

`110:4134-4135`, inside Thread B:

> **is by this design's own axis-sorting test a `Lowering`-level choice**, since the representable set and
> the mathematical function computed are identical across all three and only the cost and the shape of the
> call site differ.

That is the only occurrence. `change the axis` returns zero, `set of representable values` returns zero,
`only the arithmetic differs` returns zero, `sorting test` returns one (the same line). **The line number in
`112:498` is 4130; it is 4134-4135**, a five-line miss that matters only because the phrase is the single
thread connecting the two documents.

**Both harms apply, not one.** The content is uncitable, because a reader cannot tell that "this design's
own axis-sorting test" is op's ratified D54 rather than a panel coinage. And it is unusable, because the
test's text is nowhere in `110`, so a member told to apply it cannot. It is also a violation `110` does not
catch: `110:4863` claims "No term in this document is left undefined or uncited", and the file 111
correction at `110:4865-4877` lists six terms that break it. The axis-sorting test is a seventh and is on
none of the lists.

### 4.2 D53: the one-type half carried, the alias half lost

Op, `talk:326-332`: "**D53. There is one numeric type, and every family arvo ships today becomes a semantic
alias over a composition of it.** ... `UFixed`, `IFixed`, `FastFloat` and `StrictFloat` stop being four types
and become four names for four compositions."

`110:482` carries the first half in the design's opening sentence: "**A value of `Number<N: Numeral, S>` is
an integer `k`**". The second half returns nothing: `four compositions` 0, `four names` 0, `one numeric type`
0, `semantic alias` 1 (`110:3712`, about `Capacity`, a different subject). `UFixed`, `IFixed` and `FastFloat`
appear in `110` only as facts about the shipped tree (`110:2039`, `:3832`, `:3855`, `:4407`, `:4419`); `Uint`
returns zero. The `namesake aliases` at `110:2693` and `110:3721` are `79b`'s parity-suite intent pillar, a
different mechanism about names asserting standards conformance.

The precedent D53 rests on is also stranded: `spec:317-318` grounds it on "the relationship the one D40
already established for `Rect` over `Orthotope`", and `110` carries D40 by number twice (`110:2591`,
`110:3091`) purely as a geometry ratification, never as the alias precedent.

### 4.3 D74: the marker carried, the accepted trade lost

Op, `talk:1829-1837`: "**D74. `ConstantTime` is a derived marker, not an axis.**" and, in op's own words for
the record, "this means the marker reports rather than requests. A consumer can check whether the composition
they chose is constant-time; they cannot demand it ... **Recorded so a later reader does not reopen it as an
oversight.**"

`110:4149-4151` carries the marker and reports a defect in it: "**which means the `ConstantTime` derived
marker is currently keyed on data that does not decide it: delivery decides it, and delivery is not one of
the ten axes.**"

Two readings, and the evidence does not force one. Under the first, `110`'s finding is exactly the reopening
D74 forbade, arriving because the clause forbidding it is absent. Under the second, D74's clause covers an
internals change withdrawing the property, and `110`'s finding is that the marker's *key* is wrong, which is
a different defect D74 never addressed. I lean to the second, because `110`'s ground is delivery rather than
an early exit. Either way the accepted-trade clause is absent, `reports rather than requests` returns zero,
and the next reader has the defect without the boundary op drew around it.

### 4.4 D73: the range distinction preserved, the marker family gone

Op, `talk:1758-1776`, closes C5 and C6 in one call. Two halves.

The marker half, "`IntegerLike`, `FractionLike` and `FloatLike` become blanket impls conditioned on the
exponent form" and "`BoolLike` leaves, because it was never a member of the same family": **all four names
return zero in `110`**. `Bool`'s placement is worked at length in `110`'s section 1.30 and never as this.

The range half, "the *numeral's* range is identity and the *carrier's* range is lowering, and they differ
exactly when the stored width is doubled": carried, unnumbered, at `110:2596`, "D45's placement overtaken
twice, **its distinction preserved both times**". That is the better outcome of the two, and it is the only
sentence in `110` that reaches any part of D73.

### 4.5 D71: the tables carried by number, two of three consequences lost

The single best-carried decision in the register. `110:2099-2107` and `110:2125-2133` reproduce both preset
tables cell by cell, `110:2109` cites D71 by number, and `110:4909-4910` records that the tables were
transcribed from `78:409-441` rather than from any paraphrase.

Op stated three consequences at `talk:1702-1715`. `Precise` is fallible: carried (`110:2112`, `110:4129`).
The other two are absent. "Only `Hot` folds for signed values" returns zero on `folds for signed`, `only Hot`
and `wrapping is exactly`; `AddAssoc` appears three times in `110` and never in this sense. "`Cold` now pays
a compare and select on every store" returns zero on `compare and select` and `every store`.

One further divergence worth recording rather than resolving: `110:2114-2115` justifies `Cold` rounding to
nearest as "a type nobody expects to crash has no reason to accept truncation bias", where `talk:1674-1678`
justifies it as "it is already paying a widen and a narrow per operation, so the compare and increment that
nearest-even costs is small against what it has already spent". The talk's ground depends on the `Widening`
axis, which was ratified out at `39b`, so `110`'s substitution is a correct re-derivation under
`110:136-150`'s own principle. It is not marked as one.

---

## 5. Carried, and the entries worth reading

**Carried by number, same decision, fifteen:** D11 (`110:3775`), D16 (`110:930`, `:1097`, `:3397-3423`),
D17 (`110:3453-3462`), D27 (`110:3313-3377`), D28 (`110:2596`), D34 (`110:1206-1214`), D36 (`110:1225`,
`:3066`), D38 (`110:831-915`), D39 (`110:837`, `:3693`), D40 (`110:2591`), D45 (`110:2596`), D47
(`110:1164`), D49 (`110:3769`), D50 (`110:2593`), D51 (`110:930`).

**D34 is the model entry and should be the template for the fixes below.** `110:1206-1214` names the
decision, names what survives, names what dies, and names the provenance of the overtake in one paragraph:
"**The round's `Growth` vocabulary is dead; D34's own content survives, wearing different clothes, as the
`StoredWidth` axis's `doubled` instance on `Lowering`** ... D34's principle stands; its vocabulary does not;
the overtake is op-on-op, later checkpoint winning over earlier round text, provenance clean at every step."
Every silent supersession in section 3 and 4 is one paragraph of that shape away from being a record.

**Carried without its number, nine:** D1 and D2 (content everywhere, numbers colliding, section 1); D54
(section 4.1); D58, D59 and D60, the three contracts, present as `Numeral`/`Policy`/`Lowering` throughout
and reinforced at `110:580-584` with the 1.8x rendered-diagnostic measurement that keeps the fused
two-parameter form; D61, present as `Adjustment` and `FullRange` (`110:502-508`, `:1225-1232`) with its
instance set generalised to a signed gcd-normalised rational at `44b` and one member left open at
`110:4098`; D63, present as `Direction`, `ToEven` and `ToOdd`; D74 (section 4.3).

**Two of those carry a self-reported defect.** `110:4871-4873`, in the file 111 correction, lists among the
terms used and defined nowhere: "the **`Resolution` axis's four members**; **`Quantisation`**, which is the
sole content of `Policy` in the ratified trait table; **`Direction`**". Those are D63's and D64's
vocabularies, and `110` knows it does not define them. Three of D64's five members (`UnderMidpoint`,
`OnMidpoint`, `OverMidpoint`) return zero by name; the other two appear only inside the preset tables.

---

## 6. Everything else this diff found

### 6.1 `110` twice asserts a ten-axis set it has itself dismantled

`110:4118` ("under which ten axes render for free in an error message") and `110:4150` ("delivery is not one
of the ten axes"). The ten axes are D69's (`talk:1621-1641`). `110:4734-4737` records that `Widening`'s three
instances, `Growth`'s two, `LogicalWidth` as a primitive axis, `Underflow`'s `Unbounded`/`Flushed` members
and the `Narrowed<W, A>` shape "go with the axes themselves, ratified out at `39b`", and `110:2476` and
`110:2484` mark both removals RATIFIED in the trait table. **The set is not ten and has not been since
`39b`.** `110:4871-4872` already flags "the ten axes (quantified over twice, listed nowhere)" as undefined;
it is worse than undefined, it is a stale count carried inside two live arguments, one of which
(`110:4150`, Thread B's `ConstantTime` finding) uses the count as a premise.

### 6.2 Op decisions in the register with no D-number

The register's own completeness is not guaranteed and three items sit outside it.

**The faithfulness derivation.** `talk:1187-1203` and `spec:203-222` state the two-impl law derivation,
including `impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}`, with no D-number. It sits
in the spec file's ratified body and is not among the four items `spec:356-359` marks as the agent's own.
**`110:4271-4272` droplists it**: "The unsigned faithfulness blanket over every `Resolution` pair: refuted by
compiled counterexample. `SubstituteZero` breaks associativity where clamping and modular reduction preserve
it." The refutation is correct and compiled; the droplist entry does not say what it is refuting, so a
reader of the spec file finds the impl standing and a reader of `110` finds it dead, with nothing joining
them.

**The four preset intents.** `talk:1659-1661` states them in op's own voice inside a paragraph with no
D-number, and it is the one place `110` cites a topic file (`110:2089`). They ground D71 and are carried.

**The seventeen open rows.** `talk` carries "**Status:** open" seventeen times and "settled" four
(`talk:1032` through `:1847`). Sixteen question rows the round explicitly left open, including A1 (the
scale), A2 (the format decomposition), B1 (the composition type), B3 (what a preset is mechanically), C2,
C3, C4, C6, E1 and E4, are tracked in `110` only where a different route rediscovered them: `110:4670-4677`
restores two of file 11's four, which overlap B1 and C4, and does so citing file 11 rather than the round.
The round's own open list has never been reconciled with the panel's.

### 6.3 The scope this diff could not cover

The dispatch names two files. The register's other half lives in
`202607301000_topic.inherited-state-from-the-formalization-round.md`, the third flat file of the same active
round, and the twenty-three prior-round citations above were resolved against it by reading their definition
sites. **That file has itself never been diffed against anything.** It carries fifty-two decisions plus the
duplicate sequence, and `110` cites twenty-one of them by number. A reader wanting the same instrument
applied to it needs a fourth pass, and it is one command wider than this one.

I did not read `110` in full. I read sections 0, 1.1 through 1.5, 1.10, 1.21 through 1.26, 2, and 5 through
9 at the ranges cited, and grepped the rest, twice per negative with a second vocabulary. Every zero-count
claim above was run fresh on 2026-08-05 over `110` alone, none inherited from `109`, `111` or `112`, and
three verdicts changed when the second vocabulary found something the first did not (D68's `AddClosed`,
D73's range distinction, D75's depth half under D47's number).

---

## 7. The instrument, since `16c:31-53` asks a member to design its boundary rather than report it

Two mechanisms, and the first is one line.

**Make the identifiers unique, before anything else.** The register cannot be diffed by number while two
D1-D4 sequences and a question grid share the prefix. The cheapest repair that changes no committed text is a
round-qualified form used in every new citation, `202607282100/D1` against `202604.../D1`, with the bare form
read as ambiguous rather than as the nearest match. **A number that is not unique is worse than no number**,
because it resolves silently to the wrong decision, which is exactly what `110:2590`'s "`arvo-shape` |
D1-D4" does to a reader who follows it into the talk file.

**One row per decision, keyed on the register rather than on what a consolidation carried.** Three fields:
the decision's own `file:line`, the disposition (`carried` with the `110` line, `carried unnumbered`,
`weakened` with the clause named, `superseded` with the file that superseded it, `absent`), and, for a
supersession, whether the standing base states it. Forty-six rows, and the row set is append-only because
topic files are frozen at lock.

**What the checker can determine from it alone**, which is the test of whether it is worth committing: it
answers "is every op decision in this round reachable from the standing base" by reading two line ranges per
row. It cannot answer whether a carried decision is still correct; that is `110:136-150`'s subject and no
ledger automates it.

**What the design needs back from whoever builds the taxonomy round.** The register is the input to that
round, per `68b:14-21`'s sequence, and three of the absences above (D72's crate split, D67's `conv-*`
features, D23's placement) are exactly the material that round consumes. If the round is briefed off `110`
alone it will invent a crate structure that op already decided. **The minimum is that the two topic files
join the ratified rung at `110:154-160`**, which is one line and is where this whole finding started.

**Where this genuinely stops.** It is a reachability check on identifiers and content, run by one reader
against one document. It does not check the inherited file (section 6.3), it does not check whether any
carried decision survives its own evidence, and its negatives are greps, which this stretch has now
demonstrated six times is the weakest part of any claim.

---

*Grounded on: ratified (both topic files read in full at source on 2026-08-05;
`202607301100_topic.the-formalization-talk.md` at every range cited;
`202607301200_topic.the-formalization-spec.md` in full;
`202607301000_topic.inherited-state-from-the-formalization-round.md` at its decision-declaration lines;
`108b:11-20` the re-derivation licence; `16c:31-53` the boundary obligation this file's section 7 performs),
settled shapes (`110` at the sections listed in section 6.3, `112` in full, `111:307-355` and `111:544-551`),
verified at source (every D-number extraction, every citation count and every universal negative re-run over
`110` and the two topic files on 2026-08-05; the `arvo-numeral`/`arvo-policy`/`arvo-lowering` negative run
four ways before recording). Canon gate: `108b:190-193` places `mock/crates` out of bounds and gives the
panel `mock/research/` and `mock/benches/`; `mock/design_rounds/` is in TOPIC phase and its files are frozen,
so they were read and not edited. This file writes one document in `mock/research/` and touches nothing else.
Only op's calls are final, and nothing above is a design call.*
