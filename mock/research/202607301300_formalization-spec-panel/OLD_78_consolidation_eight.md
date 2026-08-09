# The current shape, eighth consolidation: the pricing pillar named, the presets and the far point ratified, the byte image charted, and the facade fork closed on soundness

File 68 stood as the reference after the seventh consolidation absorbed files 64 through 67 against
the fifth stand-in checkpoint, `67b`. This document replaces it, absorbing nine deliverables (files 69
through 77) read against four checkpoints that are, for the first time since `44b`, all op's own:
`68b`, `70b`, `74b`, `77b`. Op is back. The overnight persona-checkpoint mechanism recorded at `48b`
ends at `68b`; the five persona checkpoints (`48b`, `53b`, `57b`, `62b`, `67b`) stay in the record as
persona-decided, and op has spent this stretch walking their calls individually rather than confirming
them as a block, which is why several of them close, correct, or get overtaken by name below rather
than simply carrying forward.

Op also reset the review's own scope at `68b`: the panel produces canon, not source. `mock/research/`
and `mock/benches/` are its ground; `mock/crates` is out of bounds until the design is settled in full
and earmarked as arvo's first full canon. One standing consequence follows immediately: `67b`'s
authorization of the `arvo-strategy` migration round is **withdrawn**. It was never executed and, per
op, should not have been issued. Section 1.26 and the live-defect registry both carry the correction.

This document makes five corrections to the seventh consolidation that are statements rather than
additions, each named at the top per the standing discipline this review has kept since the fourth
consolidation's own defect. First, `68:534-535`'s strategy-door table (the "shipped meaning" column,
carried through `62b` and `68` from file 59) is **void**. Op's own correction: shipped source and its
comments are "by definition deprecated and wrong on the new design," and the table's justification for
three of its four rows was exactly that, a doc-comment reading rather than a design derivation. File 69
traced the propagation (one origin, three carriers, one near-miss) and file 70 replaced the table in
full from op's stated intent alone; both tables in section 1.21 below are the replacement, not an
amendment. Second, the sixth consolidation's "exactly one cell of the matrix leaks" framing (file 66,
already corrected once at file 67 to a family of six leaking cells) gains its closing rule this
stretch: the far point is the supremum of a numeral's ordered representable values, and the open cell
file 70 flagged (`Warm`/`Cold` out-of-range on a numeral with no infinity) was never a cell needing a
separate answer, it was an unstated instance of a rule the review had already ratified twice. Third,
`74b`'s working "two instances" reading of `Layout::Bitpacked` (byte-aligned slots as one instance,
zero-padding as another) does not survive; the axis has exactly one meaning, and the byte-aligned
reading is `Layout::Dense` at a narrow `StoredWidth`, a case the ratified preset table already assigns
to `Hot`. File 32's own bitpacked measurement is retroactively relabelled: it measured `Dense`, not
`Bitpacked`, correctly built and mislabelled. Fourth, `74b`'s adoption of one unified type-level
natural encoding for capacity is corrected in scope rather than reversed: the naive spelling that
follows from `74:144-151`'s own "whole load-bearing path" sentence does not compile, refuses citing the
forbidden `generic_const_exprs`, and the successor feature cannot express the inductive step either.
The feasibility probe that cleared the unification never built the associated array type the capacity
domain exists for; two working constructions exist, and op's own reframing of what capacity denotes
(section 1.26) is now the open question, not the closed one. Fifth, `68:816-817`'s facade-migration
framing is superseded by a settled fork rather than merely priced further: the fork **closes to route
Z**, and it closes on a compiled guarantee failure in route Y before the cost measurement is even
consulted, which this document's own fourth design rule states is exactly the order the design's
pricing discipline requires.

The stretch's arc: file 69 (Ringer) swept the whole panel for the source-justification defect op named
at `68b`, found it recurs in exactly one place beyond its own exhibit, and proposed the grounding split
op adopted in full. File 70 (Wronski) re-derived both preset tables from op's stated intent alone,
under file 69's own deletion test, and surfaced the fixed-point-versus-float divergence on `Warm`'s
stored width as the sharpest single finding of the pair. File 71 (Smith) stress-tested op's own
saturation instinct for the one open cell the presets left, found it was the bounded instance of a
rule the review had already used twice without naming it, and closed the cell by naming the rule
instead of filling it. File 72 (Giesen) picked the best unexamined ground from a self-audited coverage
sweep (the external images of a value: text, bytes, digest) and found every hard sub-problem already
answered by machinery the review had already ratified. File 73 (Arntzen) took the byte boundary apart
into two further structural maps, found one of them is forced rather than chosen by a purity argument
nobody had made, and found `Layout::Bitpacked` had been carrying two meanings across the whole corpus
unnoticed. File 74 (Lattner) rechecked the design round's own eleven-crate taxonomy against everything
ratified since, sixty-three files after the table was written, and found three rows change substance
while the split itself survives. File 75 (Aaltonen) took the compute-side dispatch op set for
`Layout::Bitpacked` and closed it against the ratified preset table plus a real bench, replacing the
working reading rather than confirming it. File 76 (Kiselyov) priced the facade fork on a real
consumer's declaration count, found the cheap route fails on soundness three separate ways before its
cost is even worth measuring, and found the unification op had just ratified does not compile as
written. File 77 (Ringer) scoured every checkpoint in the corpus for pillar decay at op's own request,
found one genuine defect (a rule's guard clause quoted nowhere in seventy-seven files) and one healthy
pattern worth generalising (file 76's own guarantee-before-cost ordering), and named the pricing
pillar as the review's fourth design rule.

**Verification.** Every claim below tagged compiled or measured traces to a probe or a committed
artifact in `69_probes/` through `77_probes/` (and, for file 75's bench work, `mock/benches/variants/
bitpack-*` and the committed CSVs alongside it), each carrying its own outcomes file, or to the shipped
tree at the cited path and line, re-read fresh in this stretch. The numeral tower still has no shipped
source: run fresh from the repo root, `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same command with `FullRange\|UTerm\|AddWidth` in place of the first pattern
both exit 1, empty, unchanged since file 45 first corrected the path this command uses. `cargo test
--offline --workspace` reports **661 passed, 0 failed, 9 ignored**, the standing 658 (unchanged since
file 65) plus the three tests file 75's `bench-bitpack-shared` crate adds (`Column<256>`,
`Column<4096>`, `Column<16384>` round-trip and permutation checks); no shipped crate is touched by any
deliverable this stretch. The pinned toolchain is `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, resolved from the repo's `rust-toolchain.toml`, confirmed fresh inside the tree
by two members this stretch (files 73 and 75 both also confirmed that the identical command run
*outside* the tree resolves to stable `1.94.0`, a check now worth carrying forward as standing
practice for any dispatch that builds a harness or a bench).

**The table-diff obligation, executed on this document by its own author before it stands**, per the
standing rule the `57b` checkpoint set and every consolidation since has carried out on itself. Every
table below was checked line by line against the prose of the section it sits in and against the
source file that established each member. Three corrections the diff caught rather than a source file
naming them directly: the spine rule's firing count (`68:98-101` counted nine; file 72's `ShortCap` is
the tenth, file 73's `ByteCap` the eleventh, and this document states eleven rather than silently
carrying nine forward); the hardware-reachability theorem's cell count (file 70 corrects "reachable
only in a uniformly-`Hot` expression," one cell, to four cells, and the assembled trait table's own
prose in section 1.21 below was rewritten to match rather than left citing the old figure); and the
live-defect registry's entry 6, rewritten in full against file 76's own line numbers rather than
against this document's paraphrase of the seventh consolidation's paraphrase.

**On the sentences below that are this document's own resolution rather than a restatement of a source
finding.** Four places do original work beyond compression. The consolidated loudest-for-op list in
section 2, reflecting this document's own judgment about which items across nine files and four
checkpoints are still live versus already resolved. The bridging note in section 1.22 connecting the
external-images chapter's canonicalising-projection pattern to the four prior instances the layer-
keying rule had already found separately (`TotalOrd`, the spectral defect, the notation face, and now
the digest and the padding law), which is this document's own count of a pattern file 73 itself named
but did not tally against the rule's earlier occurrences. The explicit statement, in this document's
own words, that the strategy-door table is corrected rather than merely superseded, and that the
capacity unification is corrected in scope rather than reversed. And the ordering of section 4's open
items, which follows op's own stated priority (`77b`: "the open list, not the interesting list, is the
work queue," capacity first) rather than any source file's own sequencing. Everything else is a
compression of a claim a source file already made, tagged with that file.

## The four design rules

Three design rules have been the review's real product since the persona checkpoint at `48b` named
the first two; this stretch names a fourth, and op states plainly that it has outranked compile-cost
framing in every decision the review has actually made, even where the review's own wording did not
say so.

**The spine rule.** A quantity that is computed and then has to appear in a type is a type; a quantity
that only ever has to be read is a const. Eleven occurrences stand, up from nine at the seventh
consolidation: two founding (op's `44b`), seven through the eighth (`68:98-101`), and two new this
stretch, both capacity-shaped: the print buffer's `ShortCap` (file 72, `72_probes/probe_3`, refusing
under the naive const-expression spelling with `generic_const_exprs` named in rustc's own help text,
shipping as an associated type with a declaration-site coverage assertion instead) and the byte
buffer's `ByteCap` (file 73, `73_probes/probe_1`/`probe_1b`, the identical refusal shape). Eleven
independent firings of one rule across unrelated quantities (grade projections, notation faces, seal
witnesses, container widths, text and byte capacities) is, per file 73, evidence the rule is a property
of this design's shape rather than a coincidence noticed repeatedly.

**The carrier-at-birth rule.** A closed vocabulary that a guarantee quantifies over owes its seal and
its adversary at birth, not after three passes. Unchanged in content this stretch; no new firing. The
capacity work in section 1.26 below is downstream of the rule rather than a new instance of it: the
shared bottom carrier the tower already sealed at `44b` is what the capacity unification reaches for,
not a fresh vocabulary needing its own seal.

**The layer-keying rule.** A fact is keyed on the coarsest layer whose identity its truth depends on.
Unchanged in statement; gains a fourth and fifth confirmed instance and one genuine extension this
stretch. File 72 found the digest is the rule's fourth site (a datum-keyed digest pairs with the
datum's total order, a value-keyed digest pairs with value equality, and mixing the pairings breaks
the consistency law either direction it is mixed, compiled). File 73 found carrier identity is a
**third** identity notion beneath the rule's own face/encoding-equals-value pair, strictly finer than
datum identity rather than coarser, and named what it is for: almost nothing should ever be keyed on
it, because the padding bits it distinguishes carry no denotational content by construction. The one
extension: the rule's own display clause ("a fact depending on where something was written belongs on
the face") quantifies over a layer that does not survive to runtime, since a face cannot reach a
numeral position and a computed value has no face by the time it exists. File 72's completion: at
runtime the honest split is a value-keyed display (canonical, shortest round-trip) and a datum-keyed
debug image (raw fields, NaN payload, cohort member), decided by the rule's own coarsest-layer test
applied a second time to its own clause.

**The pricing pillar, new this stretch, named at op's own direction.** Runtime and lowered code are the
measurement. Compile time is not a cost to be minimised; it is a resource to spend, without a ceiling
stated in advance, whenever spending it buys a runtime saving, a soundness property, or a correctness
guarantee. Op's own words, quoted rather than paraphrased because the wording is load-bearing: "Compile
time is nothing. That can be literal minutes for all we care... the important measurement is the actual
runtime and lowered code... We *want* long compile times, if it resolves to snappy optimal runtime with
the extra soundness, safety and numeric machinery amortized fully at compile." And the sharper claim
that does not follow from the general workspace rule alone and therefore earns its own sentence: **a
strategy marker changes what happens at runtime. It never changes how much is amortised at compile or
const time.** All four presets verify to the same depth; they differ only in what they then emit. The
standing test a member can run against any proposed mechanism, stated as its own falsifiable clause per
`77:60-64`: does anything this design does at runtime have a compile-time or const-time alternative that
was rejected, and if so, was it rejected because it does not exist under the permitted feature set, or
because someone judged the compile cost too high? Only the first is a real constraint; the second is the
violation.

**The guard clause, carried explicitly because its absence is what let the rule's wording decay.**
`arvo-compile-time-last.md`'s own corrective section names and forbids exactly the misreading this
pillar's own name invites: "compile time last" states which cost is minimised least urgently, not which
cost is pushed downstream, and it does not license preferring a runtime check because it is cheaper to
compile. Op's audit found this rule has never actually been violated in substance (all twenty-four
citations of the workspace rule across the corpus argue in the licensing direction, and the one place
compile cost genuinely forked a design decision, file 76's facade pricing, closed on the guarantee
before consulting cost at all, which is the pillar working). What had decayed was wording: the guard
clause is quoted nowhere in seventy-seven files, and the consolidation's own vocabulary for the facade
fork ("gated on the bench," "becomes the gate") never states in the same breath that the gate only
separates routes already tied on correctness. A reader of the base document alone could reasonably have
concluded compile time was deciding the design. It was not, and this document states the guard clause
inline so that the next reader does not have to rediscover file 76's own ordering to find that out.

## 1. The agreed shape

### 1.1 What a number is

Unchanged from file 40 and every consolidation since.

### 1.2 The identity contract

Unchanged. `Radix` sealed as `Rad<P>` over `AtLeastTwo`, `Bias`/`Adjustment` sealed, value-unique,
gcd-normalised.

### 1.3 Encoding, nested inside Lowering

Unchanged. `Lowering` changes no value; `Encoding`, nested inside it, may change which datum carries a
value. No law may read `Lowering`.

### 1.4 The crossing contract

Unchanged since file 68's own correction (a precondition rather than a fourth statement; a family of
leaking configurations rather than a single cell; `Crosses<N: Numeral>: Lowering` adopted
presumptively). Section 1.22 below extends the same statement structure to a boundary further out (the
byte image) and widens `Crosses` itself with a second condition; the crossing contract's own three
statements plus their precondition are untouched.

### 1.5 The quantiser

Unchanged. Round-first, classify second. File 72 found parse decomposes as the identical quantiser
applied once to a digit string's exact rational (section 1.22), which is a new consumer of the
quantiser's stated totality over ℚ, not a new fact about the quantiser itself.

### 1.6 Membership and the number-system layer

Unchanged this stretch, still held exactly as D39 and file 64 left it: sound for arvo's own numerals,
its uniqueness justification false against the full ten-member vocabulary, final hardening genuinely
op's since the hold was his.

### 1.7 The algebra

Unchanged this stretch.

### 1.8 The fold

Unchanged this stretch.

### 1.9 The multiplicative half

Unchanged in mechanism. File 71's far-point work (section 1.16) leans on `mul_full`'s exact-widening
construction to dissolve one of the three stress-test computations for the far-point cell; no new
finding about the multiplicative half itself.

### 1.10 Widening and Growth

Unchanged this stretch. Both ratified out of the axis table at `39b`, closed shut.

### 1.11 The value-unique encoding

Unchanged this stretch.

### 1.12 The seal: eleven firings, and a third identity notion beneath it

Unchanged in mechanism. The seal-as-free-diagnostic dividend stands at six carriers per the seventh
consolidation's own tally; nothing this stretch adds a seventh. **New this stretch**: file 73 named
carrier identity as a third identity notion, strictly finer than the datum layer the layer-keying rule
already covers, sitting below the type system's own binding times entirely (it is a runtime,
post-datum notion). Almost nothing should be keyed on it; the one fact that legitimately does is the
byte image itself, which is section 1.22's subject. File 74 separately confirmed the seal survives a
crate split compiled two ways (the sealed vocabulary and its consumers in separate crates, the attack
crate refused with rustc's own sealed-trait diagnostic naming the inaccessible supertrait), which
section 1.26's capacity work depends on.

### 1.13 Division

Held, unchanged this stretch.

### 1.14 The grade

Unchanged in mechanism. File 71 uses the value-carried grade (not a per-thread flag word, ratified at
file 58 section 1.14) as the carrier for the far-point kind (section 1.16), which is a new consumer of
the grade's existing shape rather than a change to it.

### 1.15 The exponent forces the spine rule open a second time

Unchanged.

### 1.16 The float model: the far point closes the last open cell

`Underflow`'s two instances, `Specials` as a four-point product, and the `TotalOrd` split all stand as
the seventh consolidation states them. **New and load-bearing this stretch**: the ratified preset
tables (section 1.21) left one cell open, what `Warm` and `Cold` do out of range on a float numeral
whose `Specials` carries no infinity. Op's own instinct (`70b`: clamp to the largest finite
representable magnitude, stress-tested before locking) closes it, and the stress test found something
stronger than survival.

**The far point is the supremum of a numeral's ordered representable values.** Ratified in full at
`74b`. Three instances of one rule, not three cells: an IEEE-shaped numeral's supremum is the signed
infinity (file 70's far-direction reading); a no-infinity float numeral's supremum is its largest
finite magnitude, a member of the set (op's clamp, file 71); a fixed-point numeral's supremum is the
already-ratified `Warm`/`Cold` clamp cell, which file 11 had already expressed in exactly this shape
("clamping above the range is simply `TowardNegative`... at a bounded top there is one candidate,"
`11:195-196`). NaN needs no exclusion clause: the supremum is taken over the *ordered* values, and NaN
is not in the order, so the agreement between the two no-infinity `Specials` members is a theorem of
the definition rather than a case written into it. Compiled as a total, const-callable projection
across the whole four-member `Specials` product, no feature gates, no refusal anywhere
(`71_probes/probe_1`).

**The overflow boundary and the tie.** An out-of-range event begins past the extended-grid rounding
boundary, half a top-binade ulp beyond the maximum, with the tie resolved by the ordinary even rule on
the extended grid. For every IEEE format the maximum finite's stored significand is odd (all-ones), so
the tie rounds up, off the finite set, delivering the standard's own overflow-at-the-tie behaviour with
no directional constant needed. E4M3's maximum finite is even (its all-ones slot in the top binade is
the NaN encoding instead), so the identical rule delivers the opposite parity: the tie at 464 rounds
down to 448, and `(448, 464]` is ordinary in-range rounding rather than an overflow event at all.
Compiled at the E4M3 model (`71_probes/probe_2`). The IEEE standard's own text confirming the tie
favours the infinity unconditionally, corroborating rather than establishing the derivation above, is a
primary-source read still owed (section 4).

**The cost, stated rather than smoothed.** A come-back sum (`(448 + 448) - 448` at E4M3) saturates to 0
against a true 448, a silent, full-scale, in-range error, and this is the worst number the stress test
produced. Three things bear on how much it weighs, all reasoned rather than merely asserted: the
ratified fixed-point table already accepts the identical shape of error under clamp; the alternatives
(NaN-on-overflow, refusal) do not recover the true value, they only fail louder, destroying the entire
remainder of a downstream fold rather than delivering a bounded, ordered, usable wrong answer; and the
design's own multiplicative half already removes the middle stress case (a product routed through
`mul_full` never overflows at the intermediate), leaving only the additive come-back exposed, which no
width short of the true accumulator range removes and which the grade must therefore witness.

**The mitigation, compiled in shape.** The far-point projection publishes a kind, `Absorbing` (the far
point is an infinity, self-witnessing in the datum) or `Finite` (the far point is finite, silent in the
datum), total over the `Specials` product. The kind joins through a fold with silence dominating: the
published grade records `Finite` the moment any operand's far point is finite, checked over the whole
two-element carrier's four join laws in const context, not a sample. A consumer needing the in-band
witness states it as a bound (`AbsorbingFarPoint`) and is refused a finite-far-point numeral at the
call site (`71_probes/probe_3`, `probe_3b`, both no-infinity `Specials` members exercised). Whether the
kind is a parameter of the existing overflow grade generator or a sixth generator is left to the grade
machinery's own keeper; both are expressible with no gates.

**The well-formedness alternative, declined with teeth.** Refusing a no-infinity numeral under
`Warm`/`Cold` at declaration would forbid the design's own deployed `Specials` witness, E4M3, under the
exact preset (`Cold`) whose deployment profile matches E4M3's real silicon use. It also crosses the
warn-never-police line by name. NaN-on-overflow was given its own hearing (it is a real deployed OCP
mode, not hypothetical) and declined for the preset table on four grounds: it cannot close the cell
alone (`NoSpecials` has no NaN either), it surrenders the total order, it manufactures the design's
already-catalogued NaN-poisoning defect from a range event rather than avoiding it, and it is a
deployment mode rather than a resolution constant, so if the review ever wants it, it belongs on the
hardware door's `FloatEnv` fact, not on `Resolution`.

*Grounded on: ratified (`70b:8-10`, `70b:30-38`, `74b:8-14`), settled shapes (`11:195-196`, `70:277-292`,
`58` section 1.14, `68:876-882` entry 7), compiled (`71_probes/` all four), physical (OCP/E4M3, secondary
reads, primary owed).*

### 1.17 Radix ten

Unchanged this stretch.

### 1.18 The numeral notation

Unchanged this stretch. All three residuals from the sixth consolidation stay closed as file 68 left
them.

### 1.19 Claim provenance: the grounding registry, the transfer-ground scheme, and a new split inside `tree`

The five-row grounding table (`ratified decisions`, `settled shapes`, `physical grounds`, `tree
grounds`, `unreproducible`) and the four-member transfer-ground vocabulary (`symmetry`, `saturation`,
`induction`, `unargued`) both stand unchanged from the seventh consolidation. **New this stretch, and
adopted in full at `70b`**: the `tree` ground splits into two, and the split is required at the point of
citation, not retrofitted at consolidation time.

**The defect that motivates it.** File 59's strategy-door table (`59:215-219`) justified three of its
four rows by quoting shipped doc comments verbatim, under the header "shipped meaning," with the
sentence "every row below is derived from what the preset already means for fixed-point arithmetic in
the shipped tree." Op's correction: shipped source and its comments are deprecated and wrong on the new
design by the review's own founding instruction, and the design is fully free to restructure what a
marker means. File 69 traced the propagation: one origin (file 59), three carriers that repeated the
claim without re-deriving it (`62b:166`, `63:576-591`, `68:534-535`), and one file that touched the
neighbourhood and correctly stayed silent on the claim because its own check was narrower than what it
sat beside (`64:406-413`, which confirmed only the `Hot` refusal call, not the doc-comment framing). The
review's own existing verification machinery (the table-diff obligation, the compiled/measured tags)
checked that every citation resolved and traced to a real probe; it had no mechanism for checking what
kind of thing the citation was evidence *for*, and file 59's citations were real, traceable, and
authoritative for nothing they were asked to support.

**The split, adopted.** `tree-fact`: the shipped source establishes that a mechanism exists, compiles,
or currently behaves a stated way. Licensed for any claim about current state, and licensed as input to
a design conclusion only when the conclusion is argued independently and the citation is offered as
corroboration that a needed piece already exists (Lamport's `TotalOrd` use at `33:186-198`, cited by
file 69 as the worked example: the mathematical argument for a total-order-keyed law stands first, the
doc comment only confirms the mechanism is already declared). `tree-meaning`: the shipped source's own
prose is offered as the reason a design construct should mean what it means. **This ground is
forbidden.** No claim may carry it.

**The mechanical test, run at the point a member writes the sentence and again at consolidation time.**
Does the row's justification survive if the citation is deleted and only the design's stated intent
remains? If yes, `tree-fact`, and the row stands. If no, the row was never grounded in the design, and
the consolidation says so rather than compressing a member's confident phrasing into settled fact. File
59's own table header ("shipped meaning") and `63:576-578`'s "derived from what the preset already
means... in the shipped tree" both fail this test on sight; every clean citation file 69 catalogued
(section 3 of that file) survives it. This document, and every deliverable in this stretch, states and
applies the test explicitly at the top of its gates section; section 1.21's preset tables are the direct
product of applying it to the void table's replacement.

*Grounded on: ratified (`68b`, `70b`, the source-justification correction and its adopted split),
settled shapes (`69` in full, the traced propagation and the worked Lamport example).*

### 1.20 The algorithm crates

Unchanged in content from the sixth consolidation. The `TotalOrd` split and the `arvo-spectral`
NaN-classification defect remain the layer-keying rule's second and third instances, now joined by a
fourth (the digest, section 1.22) and the third-identity-notion completion (section 1.12).

### 1.21 The strategy door: void, replaced by two ratified preset tables

**The old table is void, in full, per section 1.19's correction.** This section is the replacement,
not an amendment; nothing below carries forward any cell, quote, or "shipped meaning" framing from
files 59, `62b`, `63`, or `68`. Every row is re-derived from op's own stated intent for each preset,
under file 69's deletion test, and marked where it is `ratified`, `tree-fact`, compiled, or reasoned.

**The intent, quoted once rather than paraphrased at each row.** "`Hot` is as fast as possible, `Cold`
stores as small as possible, `Precise` is the most precise at the price of both storage and compute,
`Warm` is the compromise that suits most default cases and behaves intuitively"
(`202607301100_topic.the-formalization-talk.md:1659-1661`). Warm, this session: "I think we should
assume that it'll work the same as writing regular old floats would work... The intuition is that it
works and behaves as f32 and f64 etc in rust today without any framework on top of it" (`68b:62-67`).
Cold, this session: "It should be something between warm and precise... It can take more cost than
warm, but shouldn't just be precise in disguise" (`68b:69-73`).

**Fixed-point, ratified in full at `70b`.**

| | `Hot` | `Cold` | `Warm` | `Precise` |
|---|---|---|---|---|
| in-range direction | `TowardNegative` | `ToEven` | `ToEven` | `ToEven` |
| `OverRange`/`UnderRange` | `ReduceModulo`/`ReduceModulo` | clamp (`TowardNegative`/`TowardPositive`) | clamp | `Refuse`/`Refuse` |
| `StoredWidth` | minimum | minimum | doubled | doubled |
| `Layout` | dense | bitpacked | dense | dense |
| `Door` | inert | inert | inert | inert |

`Hot` reproduces D71's own row, now grounded entirely in "as fast as possible" rather than the row
D71 stated without argument: an arithmetic right shift rounds toward negative infinity for free, and
reduce-modulo is native two's-complement wraparound, so `Hot` pays for nothing beyond what the hardware
already does. `Precise` refuses rather than silently discards, because a hardware instruction is
unconditional and infallible by construction and `Precise`'s identity requires a refusing branch;
doubled storage lets a chain of operations retain more than one operation's exactness before a narrow
forces a decision. `Warm` and `Cold` both round nearest and clamp for the identical reason (a type
nobody expects to crash has no reason to accept truncation bias), differing only on D71's own remaining
two rows, restated fresh: `Warm`'s doubled, dense shape matches a naive hand-rolled fixed-point type;
`Cold`'s minimum, bitpacked shape is "stores as small as possible" literally. **`Door` is inert for
fixed-point**, and the reason is structural rather than asserted: a native integer add and `Hot`'s own
software composition (`mul_full` then `ReduceModulo`) compile to the same instruction, measured
zero-cost at native and multi-limb width (file 35), because an integer ALU has no rounding-mode control
state to distinguish; every preset's effective door is the software composition, folded to the native
instruction wherever that instruction computes the same thing.

**Float, newly derived, ratified in full at `70b`.**

| | `Hot` | `Cold` | `Warm` | `Precise` |
|---|---|---|---|---|
| in-range direction | `ToEven` | `ToEven` | `ToEven` | `ToEven` |
| `OverRange`/`UnderRange` | far point | far point | far point | `Refuse`/`Refuse` |
| `StoredWidth` | minimum | minimum | **minimum** | doubled |
| `Layout` | dense | bitpacked | dense | dense |
| `Door` | `HostFloat<E>` | `Quantised` | `HostFloat<E>` | `Quantised` |

`Hot`'s in-range direction cannot be the fixed-point row's `TowardNegative`: no general-purpose FPU
implements that as its default, and the one rounding attribute every FPU implements for free is
round-to-nearest, ties-to-even. Every preset's `OverRange`/`UnderRange` now reads "far point," per
section 1.16's rule, rather than the open `?` cell file 70 originally left for `Warm` and `Cold`.
`Precise`'s refusal needs no `Specials` well-formedness condition at all; it is the one preset whose
out-of-range row never has to ask what lies past the edge. **`Warm`'s `StoredWidth` diverges from its
own fixed-point row, and this is the sharpest single finding the re-derivation produced.** IEEE 754
requires a correctly-rounded result computed as if with unbounded intermediate precision, delivered for
free by the hardware, invisibly; doubling `Warm`'s float storage would add bookkeeping the hardware
never asks for and the "no framework on top of it" intuition explicitly forbids. So `Warm`'s float row
matches `Hot`'s on both `StoredWidth` and `Layout`, and diverges from `Warm`'s own fixed-point row on
both, because the two number kinds needed the doubling for the same underlying reason (correctly-
rounded intermediates) and only one of them lacks hardware that gives it away for free.

**Refusal, not silent fallback, binds every preset, design-wide, cited by rule rather than by any
marker's own meaning.** `arvo-toolbox-not-policer.md` forbids exactly the failure mode a silent
hardware-to-software fallback would be. A door a target's silicon does not implement refuses to build
rather than degrading thirteen to seventeen times slower with no diagnostic; this binds `Warm` exactly
as it already bound `Hot`.

**The hardware-reachability theorem, corrected.** File 59's original claim, "the hardware door is
reachable only in a uniformly-`Hot` expression," was true under the void table where `Hot` was the only
preset carrying `HostFloat<E>`. Under the ratified table, `Warm` carries it too. `RANK`'s own ordering
(`Precise > Cold > Warm > Hot`, `arvo-strategy/src/lib.rs:104-107`, cited as `tree-fact`, its existence
and ordering, never its meaning) survives re-derivation on independent grounds checked fresh rather than
assumed: `Precise` is still the only preset that ever refuses, `Hot` still the only preset whose door
reaches hardware unconditionally, `Cold` still ranking above `Warm` because "can take more cost than
warm" is itself the conservative-preset property the old ordering already encoded. A mixed expression's
resolved door is hardware exactly when both operands rank at or below `Warm`: **four cells of sixteen**,
`(Hot,Hot)`, `(Hot,Warm)`, `(Warm,Hot)`, `(Warm,Warm)`, not the one cell the void table's theorem named.

**Open, stated rather than resolved.** Whether `Quantisation`'s declared type is even consulted for a
numeral whose door is `HostFloat<E>` is a mechanical `B3` question about what a preset *is*, not a
preset-content question this section settles. The IEEE 754 primary-source read for the overflow tie
(section 1.16) is shared with this section's own derivation. File 59's diagnostic string naming `Warm`
as an unconditional software-quantiser alternative to `Hot`'s refusal is now stale and needs correcting
when a stub exists to correct it.

*Grounded on: ratified (`70b` in full, `68b:56,62-73`), settled shapes (`11:195-196`, `35:109-114`,
`58` section 1.14), compiled (`70_probes/` all three), tree-fact (`arvo-strategy/src/lib.rs:104-107`),
external (IEEE 754-2019 default rounding attribute, cited as a standard).*

### 1.22 External images: text, bytes, digest, and the maps beneath the datum

**New this stretch, closing a category the review's own coverage sweep found unexamined for
seventy-one files.** File 72's blind-spot audit (driven by `11:44-56`'s own twelve-row map, which the
panel wrote and never revisited) found text, bytes, and the digest are one subject wearing three coats:
what a value looks like outside the type system, and which layer each external image is keyed on. The
general claim, stated once and instantiated three times: **an invertible external image (text, bytes)
is a crossing and takes the crossing contract's statement structure verbatim; a one-way external image
(a digest) is a projection and factors through the canonicalising projection of the layer its paired
equality lives at, and that projection is the only door.**

**Parse is the quantiser, and the review had already built it without noticing.** A digit string
denotes an exact rational by positional notation alone, so parse decomposes as `quantise ∘
rational-of-digits`, with every semantic decision (rounding direction, out-of-range resolution) the
quantiser's own. Compiled over the whole in-range grid at a model instance (radix 2, p = 8, e in
[-4, 4], 318,126 four-decimal-place strings): single rounding from the exact rational equals
nearest-ties-to-even on every string; staging the identical parse through a wider intermediate with
round-to-nearest at both steps disagrees with the direct parse on 3.2% of strings, a real and dense
defect class any naive parse-then-narrow implementation would carry; and the identical staging with
round-to-odd at the intermediate agrees on all 318,126, giving the sealed `ToOdd` vocabulary member
(named a strength with no job at file 01) its job, licensed exactly for staged pipelines with two guard
digits (`72_probes/probe_1`).

**Print is the same collapse the design was already built for.** Every correct float-printing
algorithm's expensive precondition, exact access to the value and its neighbour gaps, is handed over
for free here: `decode` is total arithmetic into the rationals, and the neighbour gap is type-level
arithmetic on the numeral's own parameters. The shortest correctly-rounded digit string that reparses
to the same datum exists for every one of 1152 model data, within a bound H that is tight at the model
(93 of 1152 data need the full bound), and both kernels are const-callable as written, closing a full
parse-print-reparse round trip inside a `const` item (`72_probes/probe_2`, `probe_5`). The print
buffer's length is the spine rule's tenth firing (`72_probes/probe_3`, `probe_3b`): a `ShortCap`
associated type with a declaration-site coverage assertion, refusing an undersized capacity with E0080
before any use site exists.

**Display completes the layer-keying rule's own clause for computed values.** The rule assigns display
to the face layer, correctly, for compile-time diagnostics. But a face cannot reach a numeral position,
so by the time a fold's result exists, the layer the rule assigned display to has already been erased
by the rule's own enforcement mechanism: a computed value has no face. The completion, reasoned from
the rule's own coarsest-layer test applied a second time: a **value-keyed display** (canonical,
shortest round-trip) against a **datum-keyed debug image** (raw fields, NaN payload, cohort member).

**The byte image is the crossing contract's own datum, `D`, carried through two further structural
maps, and the two maps are not the same shape as each other or as the one they extend.** File 73's
finding, correcting file 72's own open framing ("a boundary the review has not yet pointed [the
crossing contract and the layer-keying rule] at"). `embed : D -> Carrier`, where `Carrier` is a bit
pattern of exactly `StoredWidth` bits, is a genuine crossing, needing no precondition of its own
because `Encoding::Fields` already defines `D` to be exactly `embed`'s domain (an open question,
whether `Encoding::Fields` ever declares a non-full domain, is flagged rather than resolved, no
instance found). `materialise : Carrier -> Bytes`, laying the carrier's bits onto octets, is a pure
relabelling for every `Layout::Dense` numeral at any `StoredWidth`, needing no crossing statements at
all; **under a `Layout::Bitpacked` numeral packed with zero inter-value padding, this map does not
exist at the per-value granularity at all**, only the whole packed word or column has one.

**`embed`'s canonicalisation is forced, not chosen, and the argument is a purity argument rather than a
cost tradeoff.** File 72 offered canonical-at-rest padding as a suggestion with a stated cost, leaving
declared-don't-care on the table as a coherent alternative. It is not coherent for construction: `embed`
is a one-argument pure function of the datum, and "preserve whatever padding was already there" is not
a policy a pure function can express, because a pure function has no prior state to preserve from.
Compiled, both halves: a zero-padding `From<D> for Carrier` impl is pure (two calls, one datum,
bit-identical carriers); a second, genuinely different operation taking the old carrier as a second
argument is what "preserve existing padding" actually requires, and no `From` impl (one parameter) can
carry it (`73_probes/probe_2`). The perimeter rule sharpens why this matters beyond a cost tradeoff:
`Bits<N, S, Sign>` is `repr(transparent)` (cited as `tree-fact`, the attribute's existence, not its
meaning), so its byte layout is observable through a raw bit-cast whether or not arvo ever ships a
`to_bytes()` method, and `73_probes/probe_2`'s second half compiles a `transmute` seeing exactly what
the constructor committed to, with zero dependence on any declared API.

**`Layout::Bitpacked` has one meaning, ratified in full at `77b`, superseding the working "two
instances" reading `74b` had leaned toward.** Zero inter-value padding. The byte-aligned-slot reading
(rounding each field up to a byte- or slot-aligned width) is not a second `Bitpacked` instance; it is
what `Layout::Dense` already does at a narrow `StoredWidth`, and the ratified table above already
assigns `Dense` to `Hot`, `Warm`, and `Precise`. File 75 established this by reading the ratified
preset table closely (there is exactly one `bitpacked` cell in the whole table, and `Cold`'s own
re-derived intent, "stores as small as possible," is quoted at file 70 with the emphasis "literally,"
which forecloses a reading that wastes up to seven of every eight stored bits) and by measuring the
compute-side cost of the genuine mechanism against native `Layout::Dense`: **roughly 4.6x to 5.5x
slower per element sequentially, roughly 2.2x under random access, both stable across three sizes
spanning cache-resident to past-L1** (`mock/benches/bitpack-*`, real bench under the harness, cross-
checked against correctness tests and the harness's own auto-generated findings reports). File 32's own
earlier bitpacked measurement is retroactively relabelled: it modelled byte-aligned slots, which is a
correct measurement of `Layout::Dense` at a narrow width, not of `Layout::Bitpacked`. Op asked for a
second look at whether the 4.6x-to-5.5x multiple is inherent to bitpacking or an artifact of the
specific access pattern measured; that follow-up is owed (section 4).

**The declaration-time obligation is a widened `Crosses`, not a second trait, on the strength of the
purity finding.** File 72 left the fork open between a second trait and a widening; file 73 takes it:
`Crosses<N: Numeral>: Lowering` gains a second condition, statement P, for every carrier a `Lowering`
can produce, the bits outside `Encoding::Fields`' width are exactly the padding this `Lowering`
declares. The tower's own generated impls satisfy statement P for free by the purity argument above; an
`unsafe impl` for a hand-laid format is where the obligation actually bites, at the identical
declaration site statement 0 already governs. A raw byte buffer's own shape (length, packing
convention) is a third, different-shaped precondition (arity, not value-membership), owned by whatever
build layer or constructor accepts foreign bytes, not by `Crosses`. The byte-count itself is the spine
rule's eleventh firing (`ByteCap`, `73_probes/probe_1`/`probe_1b`).

**Arvo's own byte-image guarantee is a same-process, same-build-target guarantee, not a wire format, by
the identical logic that already scoped `Warm`'s hardware door.** A plain `f32` gives no cross-target
byte-order guarantee either; the native representation is a target fact, decided once per compile the
way `HostImplemented` already decides which float operations reach hardware. Cross-target portability
is a downstream-contract item: a transport or persistence layer needs the format's identity (radix,
precision, exponent form and bounds, domain, `Specials`, `Underflow`, `StoredWidth`, `Layout`) to
travel with the bytes or be agreed out of band, and every one of those is already a closed,
const-derivable bundle of type parameters, not a registry. No mechanism is proposed; the item is named
as owed.

**The digest is the layer-keying rule's fourth confirmed site, and the pattern beneath it recurs four
times, not once.** A digest factors through the canonicalising projection of the layer its paired
equality lives at: datum-keyed digests pair with the datum's total order, value-keyed digests pair with
value equality, mixing the pairings breaks the consistency law in one direction or the other, both
compiled (`72_probes/probe_4`). File 73 named the pattern beneath this and beneath the padding law as
one mechanism used four times without ever being stated as one thing: `V -> D` (cohorts,
`Encoding::Canonical`), `D -> Carrier` (padding, this section, forced), the digest projection (this
section, chosen by the layer-keying rule), and, stated as the one boundary in the chain that is *not*
an instance of the pattern, `Carrier -> Bytes` under `Layout::Dense` (a pure bijection with no fibre to
collapse). Every many-to-one layer boundary in this tower owes exactly one canonicalising projection,
established once, consumed by every downstream consumer through that projection and no other door.

*Grounded on: ratified (`68b:62-67`, `68b:69-73`), settled shapes (`68:125-274`, `68:568-587`,
`11:212-218`, `01:318-320`), compiled (`72_probes/` all six, `73_probes/` all four), measured
(`mock/benches/bitpack-*`), tree-fact (`bits.rs:56`, `width.rs`, `narrow_from.rs:104`), external (IEEE
754-2019 §5.12, primary read owed).*

### 1.23 The assembled trait table, and what it costs to build against the tree

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
    type Quantisation: Quantisation;      // Growth removed from Policy: RATIFIED (1.10, 1.21)
}

pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;
    type Layout:      StorageLayout;      // {Dense, Bitpacked}, one meaning each (1.22)
    type Door:         LoweringDoor;      // void table replaced, both presets ratified (1.21)
    // Widening removed: RATIFIED.
}

pub const trait Underflow { /* Gradual | Abrupt, sealed, both change representability (1.16) */ }
pub const trait Specials  { /* the product {NoSpecials, NanOnly, InfOnly, IeeeSpecials}, sealed (1.16) */ }
pub trait NumeralFace {                   // the notation vehicle's face (1.18)
    type Encoding: Bias;                  // unsealed, per-literal, bridges to the sealed tower
    const DISPLAY: &'static str;
}
pub unsafe trait Crosses<N: Numeral>: Lowering {
    // Statement 0 (1.4): for every datum d, decode(d) is in V(N).
    // Statement P, NEW this stretch (1.22): for every carrier c this Lowering
    // can produce, the bits outside Encoding::Fields' width are exactly the
    // padding this Lowering declares. The tower's own generated impls satisfy
    // both for free; an unsafe impl is where either obligation bites.
}
```

`Int` stays dropped. Rewrite cost against the shipped tree remains near zero for the numeral tower
itself: no shipped source names `Adjustment`, `Numeral`, `Bias`, `FullRange`, `UTerm` or `AddWidth`,
verified fresh for this document. What is real and defective in the shipped tree is unchanged from the
seventh consolidation (`arvo-graph`, `arvo-comb`, `arvo-spectral`,
`arvo/src/traits/from_constant.rs`, `arvo-strategy`'s container dispatch and its facade), and section
1.26 restates the migration's current status with the withdrawn `arvo-strategy` authorization and the
closed facade fork.

### 1.24 The cost model

Unchanged this stretch.

### 1.25 The downstream contract, and the crate table

**New this stretch: the eleven-row taxonomy the design round drew before the panel opened
(`11:44-56`) survives its first recheck, sixty-three files after it was written, with zero deletions
and zero merges.** File 72 pointed at the table and recommended starting from it; file 74 is the first
member to check every row against everything ratified since. Every row is an op decision, marked
`Decision (op)` inline in the inherited-state topic file; nothing below is agent drift to be policed,
and every change suggested is offered with its reasoning attached, op's to confirm.

| Row | Verdict | What now forces or changes it |
|---|---|---|
| `arvo-capacity` | Survives as locus; its ground has moved under it | Capacity is already a type (op-led migration, `unstable-features.md`), and the tower carries a second, sealed, value-unique type-level natural encoding. Section 1.26 amends the unification the row was about to adopt whole. |
| `arvo-shape` | Survives, for a stronger reason than it was decided on | D43's own "which primitive holds the bits is answered one layer down" is now the `Lowering` charter verbatim. Gains a job: the column-shaped capacity file 73 flagged as homeless is this crate's subject. |
| `arvo-geom` | Survives, untouched by the tower's content | Two inherited obligations, not changes: the still-undecided dependency edge onto the algebra-contracts crate (open since file 26, still nobody's call), and D10's motors, waiting behind division's hold. |
| `arvo-num-systems` | Survives, with file 64's correction folded in before the crate ships | Scope the "finest" fact to the real/Cayley-Dickson chain explicitly; independent predicates per branch elsewhere. |
| `arvo-platform` | Survives | One dependency-direction question: `Bool` below the numeral contracts per the bridge-home rule, or the contracts go generic over notko's truth contract. Two workable spellings, op's pick. |
| `arvo-container` | Survives as locus; its contract is substantially rewritten | The saturation-limits contract migrates to the preset `Resolution` axis; the padding law becomes the constructor obligation; the `Layout` reading is now closed (1.22); the refit family's narrowing half is an instance of the crossing contract. |
| `arvo-bitfield` | Survives as packaging | Two inherited laws land on it: a field read must land on the canonical datum through the only-door projection; the byte-sharing law decides which bitfield shapes have per-field byte images at all. |
| `arvo-float` | Survives as packaging; its former contents migrated into the tower | `Specials`, `Underflow`, the quantiser, the float preset table are ordinary tower vocabulary now. What remains is real: IEEE interchange-format instantiations, hardware-door lowerings, `Crosses` impls for hand-laid IEEE layouts. |
| predicate concept (notko) | Survives, strengthened | D16's derived-safe/asserted-`unsafe impl` split is now cited by name as the design's own general discipline (`Crosses`). |
| `arvo-pseudorand` | Survives; inherits two tower-imposed contract sentences | What a hash of a `Number<N, S>` consumes is the digest law (1.22), not the hash's own business; uniform sampling must say whether it is uniform over values or over data. |
| `notko-hlist` + `Cardinal` | Survives | One binding-time sentence owed: a count that decides a type is a type-level `Nat`; a count computed at runtime is a `Cardinal`; the mirror between them is a one-way projection. |

**Why this is a layering question rather than a naming pass.** Three newer mechanisms make crate
boundaries load-bearing in ways the original round could not have weighed: the seal's guarantee rests
on which crate declares the sealed vocabulary (compiled to survive a crate split, `74_probes/`); the
law-key rule and the orphan rule both turn "what may this impl read" into a dependency-edge question,
not merely a packaging one; and the spine rule keeps minting capacity-shaped types (`ShortCap`,
`ByteCap`, the column capacity) whose outputs need one shared home or become their own fragmentation
vector, which is exactly section 1.26's subject.

*Grounded on: ratified (the cited `inherited:` decision lines, `70b`, `68b`), settled shapes
(`68:126-587`, `72`, `73` in full), compiled (`74_probes/` both probes), reasoned (the relocation and
sorting arguments, all flagged as suggestions).*

### 1.26 The L0 spine-rule migration: the facade fork closes to route Z, the capacity unification is amended, and the `arvo-strategy` authorization is withdrawn

**The `arvo-strategy` gate's authorization is void, correcting the seventh consolidation's framing in
full.** `67b` authorized landing the structural-derivation fix on the real crate; op withdrew that
authorization at `68b` because the panel's scope is design, not source, and the authorization should
not have been issued regardless of the measurement behind it. The measurement itself (thirty impls,
compile-neutral, zero consumer edits) is not disputed and is not superseded; it simply has no execution
path inside this review's own scope. Entry 6 of the live-defect registry (section 3) is rewritten to
match: the fix is priced and correct, and it stays unexecuted design, not authorized-but-blocked-by-
phase work.

**The facade fork, priced against a real consumer and closed, on the guarantee before the cost.**
File 76's exit condition, written before any number existed, is itself a worked instance of the pricing
pillar and is stated here because it is the shape a future compile-cost fork should follow: feasibility
first, then guarantee parity (the two routes must refuse the same things at the same time, under the
same command a consumer actually runs), then cost, with a later clause never reached if an earlier one
fails.

**Route Y fails guarantee parity three separate ways, each a compiler diagnostic rather than an
argument.** The two-dimensional impl table refuses correctly at type-check but is priced on a ceiling
`arvo-toolbox-not-policer.md:60` forbids the substrate to set below what it dispatches through: it
fails outright below width 64, where the census's own real widths already sit, and costs 30.0 seconds
at a 256 ceiling, roughly quartic in the ceiling. A host-staged witness compiles clean and fast, and is
caught only at `--emit=link`, not at `--emit=metadata`, which is the command a consumer's editor
actually runs, silently re-opening the `UFixed<0, F>::ONE` defect this review spent a stretch finding
and closing. A consumer-emitted per-declaration impl is refused by the orphan rule (`E0117`), whose own
diagnostic note names route Z's shape as the remedy. **Route Y has no expression of the guarantee that
is simultaneously correct under `cargo check`, extensible by a consumer, and not priced on a width
ceiling arvo is forbidden to choose.**

**Route Z's cost, measured against the widths a real consumer actually writes, is negligible.** Counted
this session across hilavitkutin's own tree (the design's heaviest real consumer): widths 1 through 7,
11, 14, 16, 27, 28, 64, one purely fractional numeral, three strategies, forty declaration sites. At
that count, with the machinery compiled as a dependency rather than pasted in-file, route Z adds **16 ms
on a 6.35 s +- 0.09 whole-workspace `cargo check`**, which is 0.25% and one sixth of that build's own
run-to-run standard deviation. Growth is exactly linear out to 800 distinct declarations, twice the far
edge of the region fixed before measuring, and the magnitude of a value is free to sixty-two bits of
encoding depth. Every figure survives `-Znext-solver`, `-Zthreads=1`, and `--emit=link`. **The fork
closes to route Z**, and it closes on the guarantee, before the cost was even consulted; the cost, when
finally measured, turned out not to matter.

**The capacity unification ratified at `74b` needs one amendment before it is buildable, and the
amendment is a correction in scope, not a reversal of the ratification.** The two type-level natural
encodings the design was carrying (the tower's sealed, inductive, value-unique `Nat`/`Pos`, and op's
own migrated `Capacity`, a type whose parameter is an array-length const) differ in kind, not in
spelling, and the obvious unification, the shared carrier answering directly for the backing array,
does not compile: rustc names the forbidden `generic_const_exprs`, and `min_generic_const_args` cannot
express the inductive step `2 * P::VAL` even with the const block its own diagnostic suggests. The
feasibility probe that cleared the unification declared the capacity trait as a bare `const SIZE: u128`
and never built the associated array type the capacity domain exists for; the number unifies, the seal
survives the crate split, and the array grammar was never reached.

**Two constructions compile with zero gates; one is preferred.** Construction one derives the storage
structurally (the encoding's own recursive shape becomes a `repr(C)` nest layout-identical to a flat
array), costs one `unsafe` discharged per monomorphisation inside the only door rather than trusted to
a maintained list, and is kept in the record as a fallback. **Construction two, preferred, is the
layer-keying rule applied to a place nobody had applied it**: a count's arithmetic and every law
quantified over it key on the shared value carrier (the unification op ratified, and it holds); the
array grammar `[T; K]` depends on nothing but the language's own const-generic array syntax, which
forces a literal, so it keys on the lowering side instead. `Slot<N, const K: usize>` pairs the two, with
their agreement checked in an inline const block at the one construction door, refusing a disagreeing
pair (`76_probes/b2`, `b2b`). The introduction route is host-staged: a declaration macro emits the
reduced encoding and the literal length already agreeing, so neither is computed by the type checker at
any use site, which is the correct binding time for a fact the consumer states rather than derives. A
companion staging result: staging the width sum this way against leaving it unstaged is indistinguishable
at every point of a sweep to 800 declarations, because binary addition on a type-level natural is linear
in bits; **stage a type-level reduction whose cost is superlinear in the value (a rational's gcd, the
review's own already-catalogued cliff), do not stage one that is logarithmic**, offered as spec text.

**Op's own reframing of what capacity denotes is the open question this section hands forward, not the
closed one.** `77b`, verbatim: "Capacity simply denotes a fixed length. It's not a numeral itself. But
it contains a numeral that expresses this length... Which means it is also the same as infinity on
infinite number sets, and the lastmost number in finite sets. Which means conceptually 'Capacity'
already must exist. We just haven't wired it up to concrete collection usage. Does it map directly, or
does Capacity become an alias to whatever expresses that length in theory side for us?" The lead worth
naming, per op's own note: the far-point rule (section 1.16) is exactly this statement applied to a
numeral's value set; op's reading applies the identical statement to a collection's index set. Whether
capacity maps directly onto the far-point rule or becomes its own alias over the theory-side
expression is what the next dispatch derives, with the review's own two-independent-reads convention.
This goes first in the next stretch's queue, per op's own stated order of work.

*Grounded on: ratified (`68b`, `74b:28-47`, `77b:68-100`, `arvo-toolbox-not-policer.md:60`,
`arvo-compile-time-last.md:16`), compiled (`76_probes/` in full, `a1` through `d`, `b1`, `b1b`, `b1c`,
`b2`, `b2b`), measured (`76_probes/results.csv`, `split_results.csv`, `deep_results.csv`), reasoned (the
construction-two recommendation, the staging rule).*

## 2. The lead designer's calls

**Op's five earlier checkpoints, restated, unchanged.** D69 ratified: identity is parameterised in
mathematical coordinates, not encoding coordinates (`30b`). D39 held: membership through algebraic
structure stays a decision pending a positive characterisation of its honest content (`30b`). The
novelty posture (`34b`): attempt what looks unsolvable, distinguish "cannot, because impossible" from
"cannot, because nobody has done it." Widening leaves `Lowering`, `Growth` leaves the law key, the
finest-view mechanism replaces the three-relation fork (`39b`). The value-unique encoding ratified in
full, division held, every claim grounded (`44b`). The convergence directive and the novelty posture
hold unchanged through every checkpoint since, in the same words each time: the intent outranks every
instruction, is vague on purpose, and only op's calls are final, and even those go stale.

**`68b`, the return, four calls plus a scope correction and a regression correction.** The panel's scope
is design, not source: the `arvo-strategy` authorization is withdrawn (section 1.26). `Int` dropped,
exponent bounds as types, `Radix` sealed, `Specials` as a product all confirmed, the last pending a
primary-source check that stays open (section 4). The layer-keying rule, the transfer-ground scheme,
and the `TotalOrd` split are all confirmed as design rules and mechanisms. The strategy-door table's
own justification is named as a regression (shipped-source-as-meaning) and two of its four rows revert
to open, setting file 69's sweep and file 70's re-derivation.

**`70b`, both preset tables ratified, the grounding split adopted.** Fixed-point and float presets both
ratified in full (section 1.21). The `tree-fact`/`tree-meaning` split adopted, both halves, with the
mechanical deletion test as the standing check (section 1.19). One cell left open with op's own instinct
attached, pending a stress test before locking: `Warm`/`Cold` out-of-range on a no-infinity float
numeral, closed at `74b`.

**`74b`, the far point ratified, capacity unified with a condition, `Layout::Bitpacked` referred out.**
The far-point rule ratified as one statement covering three instances, generalising past op's own
graceful-degradation framing of his instinct (section 1.16). One sealed bottom carrier crate adopted for
capacity, with `Capacity` kept as a named semantic alias over it, on op's own condition: "the mechanism
unifies and the vocabulary does not." `Layout::Bitpacked`'s two-meanings ambiguity sent to a
compute-side expert rather than settled from the type side, on op's own instinct that the cost and
complexity should confine to `Cold` (an instinct file 75 later honoured by a cheaper route than the one
that carried it).

**`77b`, the pricing pillar named, `Layout::Bitpacked` ratified single meaning, the facade fork closed,
capacity reopened.** The pricing pillar adopted as the fourth design rule, in op's own words, with the
guard clause carried explicitly (top of document). `Layout::Bitpacked` ratified as one meaning, zero
inter-value padding, superseding the working "two instances" reading; a follow-up owed on whether the
measured 4.6x-to-5.5x multiple is inherent or an access-pattern artifact. The facade fork ratified
closed to route Z, on soundness before cost. The capacity unification's naive form corrected, and op's
own reframing of what capacity denotes stands as the open question for the coming stretch, first in the
queue.

**The persona checkpoints, five, made overnight across five separate nights, each explicitly not op's,
now walked individually rather than confirmed as a block.** All five still carry the provenance
statement recorded first at `48b`: dispatched at Fable tier for the duration of op's absence, every call
inside dies the moment op says otherwise. This stretch, op's own checkpoints have individually confirmed,
corrected, or superseded specific persona calls rather than ratifying the set: the `Int`/exponent/
`Radix`/`Specials` table edits from `48b`/`53b`/`57b` are confirmed at `68b`; the strategy-door mechanism
adopted at `53b`/`57b`/`62b` is corrected at `68b` (the table's own justification voided) and replaced in
full at `70b`; the transfer-ground scheme and layer-keying rule from `62b`/`67b` are confirmed at `68b`;
nothing from `48b` on the grade projection, notation vehicle, or evaluation sentence is touched this
stretch and all three stand as the seventh consolidation carries them.

**Loudest for op's morning read, consolidated across four checkpoints and nine deliverables since the
seventh consolidation, current status noted where a later checkpoint or file already resolved an
earlier item. This list is this document's own synthesis, ordered by op's own stated priority
(`77b`: capacity first, then the rest of the open list).**

1. **The capacity dispatch, first in the queue per op's own order of work.** Op's own reframing of what
   capacity denotes (section 1.26) against the far-point rule's own shape, requiring the review's
   two-independent-reads convention before it hardens.
2. `Int` dropped, exponent bounds as types, `Radix` sealed: all three confirmed at `68b`. Each still one
   line to restore if op reverses.
3. `Specials` resolved to a product, confirmed at `68b`, **still pending the primary-source check on the
   E4M3 exponent figure against the specification** rather than vendor documentation. Not performed this
   stretch.
4. The `Layout::Bitpacked` follow-up op asked for at `77b`: whether the measured 4.6x-to-5.5x sequential
   / 2.2x random multiple is inherent to bitpacking or an artifact of the access pattern measured.
5. `IeeeDefault`'s naming and `Hot`'s default environment, bundled since the seventh consolidation as one
   item for op. Untouched this stretch.
6. The membership uniqueness scoping (D39), untouched this stretch, genuinely op's since the hold was
   his.
7. `FromConstant`'s breaking-change fix, vehicle still held for its own second reads. Untouched this
   stretch.
8. The decimal `Canonical` default, standing since `62b` as one compile from closed. Untouched this
   stretch.
9. The three `unstable-features.md` rule-wording edits packaged at the seventh consolidation (the
   last-sentence correction, the third-way clause, the step-budget clause), still awaiting op's own
   wording since the rule is ratified and no persona or member touches it. Untouched this stretch.
10. `float_algebraic`'s vetting, complete and `ALLOWED` since `62b`/`67b`, row drafted, still riding in
    the rule package above.
11. **New.** The eleven-crate taxonomy's own suggested changes (section 1.25): the shared bottom
    carrier crate's name and contents; `arvo-container`'s D45 disposition; the `arvo-platform`-versus-
    notko truth-value direction; the still-undecided algebra-contracts dependency edge, open since file
    26; the packaging table gaining a seventh, bottom member.
12. **New.** Whether construction one (the structural, `unsafe`-discharged-at-the-door array) stays in
    the spec as a recorded fallback to construction two, or is dropped (section 1.26).
13. **New.** The tautological test at `arvo-tensor/tests/capacity.rs:14-18` (file 76, section 4),
    comparing a computed result to itself; flagged for deletion, not for improvement, and not the
    panel's to touch under the current scope.
14. Division stays held, unchanged, whenever op picks it up.
15. Per-application against per-value-moved event counting: untouched again this stretch, genuinely
    op's.

## 3. The live-defect registry

For defects in the shipped tree, as against findings about the still-unbuilt design. Entries 1 through
5 and 7 carry forward unchanged from the seventh consolidation; entry 6 is rewritten this stretch to
carry the withdrawn authorization and the closed facade fork.

**1. `upward_rank` and `bin_pack` silently return wrong orderings under both shipped presets.** Tree:
`arvo-graph/src/rank.rs:34-88`, `arvo-comb/src/binpack.rs:44-63`. Unchanged. Grounded `tree`, `pin`.

**2. `FromConstant` accepts an unrepresentable constant and silently produces a wrong bit pattern, or
panics.** Tree: `arvo/src/traits/from_constant.rs:40`, `arvo-numeric-contracts/src/lib.rs:85-88`.
Unchanged, vehicle still held for its second reads.

**3. `arvo-graph/tests/rank.rs` never enters the breaking path.** Unchanged.

**4. `arvo-spectral`'s ten test files never exercise an arvo numeral.** Unchanged.

**5. `mock/benches/src/main.rs` could not run any bench at all. Fixed.** Unchanged, stands.

**6. `arvo-strategy`'s shipped container dispatch, and its facade, are load-bearing on the forbidden
`generic_const_exprs` feature. The remediation is now priced, closed on route Z for the facade half,
and its authorization for the `arvo-strategy` half is withdrawn rather than pending.** Tree:
`arvo-strategy/src/lib.rs:11`, `arvo-strategy/src/container.rs:254-258`, `arvo/src/lib.rs:25`,
`arvo/src/ufixed.rs:35-36`. The `arvo-strategy` half's structural-derivation fix is measured, correct,
and compile-neutral, exactly as the seventh consolidation states, **but `67b`'s authorization to land
it was withdrawn by op at `68b`**, on scope grounds rather than measurement grounds: the panel does not
land source, and the authorization should not have been issued. The facade half's fork is **closed**:
route Z, measured at 16 ms added to a 6.35 s +- 0.09 whole-workspace check (0.25%, one sixth of the
build's own noise), linear to 800 declarations, magnitude-free to sixty-two bits, closing on route Y's
compiled failure to express its own guarantee under `cargo check` before the cost was even consulted.
**One open dependency the facade fix now carries that the seventh consolidation did not**: the shared
bottom carrier the facade's route Z leans on is the same carrier the capacity unification needs, and
that unification's own naive spelling does not compile (section 1.26); the facade's fork is closed in
design, the capacity amendment it depends on is not yet hardened past its two constructions. Nothing in
this entry authorizes execution against `mock/crates`; the panel's scope remains design only. Grounded
`tree`, `pin`, `flags`.

**7. `arvo-spectral`'s degenerate-component classification is decided by NaN payload arithmetic rather
than by anything the design calls a value.** Tree: `arvo-spectral/src/partition.rs:59,156,181`.
Unchanged in content; the layer-keying rule's reclassification from the seventh consolidation stands,
now joined by the digest as a fourth confirmed instance of the same defect class at a different layer
(section 1.22). Grounded `tree`, `pin`.

## 4. What is open

**Closed this stretch, listed once so the next member does not re-open them.** The strategy-door
table's own regression, corrected and replaced with the ratified preset tables (section 1.21). The
`Warm`/`Cold` no-infinity out-of-range cell, closed by the far-point rule (section 1.16). The
`Layout::Bitpacked` ambiguity, closed to one meaning (section 1.22). The facade fork, closed to route Z
on the guarantee (section 1.26). The source-justification defect's own propagation, traced and its
grounding split adopted (section 1.19). The pricing pillar, named as the review's fourth design rule
with its guard clause carried inline.

**Owed second reads and primary-source checks, named rather than performed.** The `Specials`-as-product
primary-source check on the E4M3 exponent figure, still pending since `68b`. The IEEE 754-2019 §5.12
inexact-conversion-signalling citation (file 72's parse chapter) and §4.3.1's overflow-tie sentence
(file 71's far-point derivation, corollary only, the derivation itself does not depend on it). The OCP
mode-split facts behind file 71's declined NaN-on-overflow ground 4. The `Crosses` mechanism's own
second read, now carrying a second, independently-proposed condition (statement P) on top of the first
member's shape, per the standing discipline every crossing-contract instance this review has carried.

**The verification dispatch, one bundle, carried forward from the seventh consolidation, unperformed
this stretch and now joined by new items.** Statement 0 against `quantize` and `roundToIntegralExact`
(flagged forward three times now, performed by nobody). `foldnum` compiled against the real four-member
`Numeral` contract with `Exponent` held fixed. The non-default `Canonical` compile. The nine-bit
`u16`-class companion model for the container-class transfer coordinate. **New**: the constructive-
extensibility compile, named as owed by files 72, 74, and 76 in succession, now the last unchecked half
of the facade fork's own argument (a foreign crate minting a numeral and a capacity over the shared
vocabulary, without minting vocabulary itself).

**The highest-leverage item for the next stretch, per op's own stated order of work.** The capacity
dispatch (section 1.26, section 2 item 1), first in the queue, requiring two independent reads before
it hardens.

**The float model's residuals, narrowed.** The precision axis's `unargued` status (no induction
argument exists) and the container-class coordinate's own companion model, both unchanged since the
seventh consolidation. The overflow-tie primary-source read (above) is new.

**Decimal's residuals, unchanged.** The `InfOnly` `Specials` witness still unfound. The `10^20` figure
still open. The reciprocal-table strength reduction for the radix-ten quantiser's dominant division
term, still an attempt rather than a build.

**Codegen-flag audit, still not fully done. Unchanged.**

**Unchanged from the seventh consolidation, untouched this stretch.** The reduction firing site and
whether `FullRange` survives as its own named constructor. The dither-versus-`Refuse` interaction. The
per-application-against-per-value-moved event-counting sub-item, genuinely op's, declined again by
default rather than by any dispatch touching it. `SC_WRAP<n>`/`SC_WRAP_SM<n>` with `n_bits > 0`. Richer
canonicalisation's branchlessness and cross-word bitpacked field extraction. `DatumDeterministic`. The
`Gcd`-for-a-local-`Rhs`-on-a-sealed-`Self` coherence question, now confined to the proposed bottom
carrier crate should section 1.26's capacity dispatch adopt one (section 1.25).

## 5. The droplist

Carried forward from files 26, 40, 49 through 62, and 64 through 68; nothing this stretch resurrected
any of it. New entries follow.

File 59's strategy-door table, "every row below is derived from what the preset already means for
fixed-point arithmetic in the shipped tree": void. Shipped source and its comments are, by the review's
own founding instruction, deprecated and wrong on the new design; three of the table's four rows were
never chosen from intent at all, and the fourth (`Hot`'s refusal call) survived only because it happened
to be independently confirmable against a different, licensed citation (section 1.19, section 1.21).

The working "two instances" resolution of `Layout::Bitpacked` (byte-aligned slots as one instance,
zero-padding as a second): superseded. The axis has one meaning; the byte-aligned reading was always
`Layout::Dense` at a narrow `StoredWidth` (section 1.22).

File 32's own bitpacked measurement, treated as a measurement of `Layout::Bitpacked`: it was always a
measurement of `Layout::Dense` at a narrow width, correctly built and mislabelled (section 1.22).

The hardware-reachability theorem's original statement, "reachable only in a uniformly-`Hot`
expression": corrected to four cells of sixteen, once `Warm`'s door moved to `HostFloat<E>` under the
ratified preset table (section 1.21).

The capacity unification's naive spelling, "the shared carrier answering directly for the backing
array": refused, four ways, citing the forbidden `generic_const_exprs` and, behind the compiler's own
suggested successor, the inductive step `2 * P::VAL`, which `min_generic_const_args` cannot express
either (section 1.26).

The feasibility probe's own implicit claim that the capacity unification's "whole load-bearing path"
was compiled: it was not; the probe declared the capacity trait as a bare const and never reached the
associated array type the domain exists for (section 1.26).

Route Y's own remaining candidate this stretch, a consumer-emitted per-declaration impl: refused by the
orphan rule, `E0117`, with rustc's own diagnostic naming route Z's shape as the remedy (section 1.26).

## 6. Verification

Every claim tagged compiled, measured or reasoned in files 69 through 77 traces to a probe, a bench
artifact, or a committed migration artifact, each carrying its own outcomes file or CSV, as stated at
the top of each source file and cross-checked for this document. The canon gate reproduces fresh from
the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth` in place of the first pattern both exit 1, empty. `cargo test --offline
--workspace`, run against the tree this document describes, reports **661 passed, 0 failed, 9
ignored**: the standing 658 unchanged since file 65, plus the three tests file 75's `bench-bitpack-
shared` crate added, confirmed by this document's own count of that crate's `#[test]` attributes rather
than accepted from a headline. No shipped crate is touched by any deliverable in this stretch, and
`mock/crates` remains out of scope per op's own restated boundary. The toolchain is `rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from `rust-toolchain.toml`,
confirmed fresh for this document; the identical command run outside the repository tree resolves to
stable `1.94.0`, a distinction two members independently confirmed this stretch and worth carrying as
standing practice for any future harness or bench.

The table-diff obligation was executed on this document, by its own author, before it stands: every
table above was checked against the prose of the section it sits in and against the source file that
established each row, and the three corrections named in the table-diff paragraph at the top of this
document (the spine rule's firing count, the hardware-reachability cell count, and the live-defect
registry's entry 6 rewritten against file 76's own line numbers) were caught by that check rather than
by a source file naming them directly.
