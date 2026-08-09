# 74. The taxonomy rechecked: eleven crate decisions against the design that grew up after them

Chris Lattner, file 74. I wrote file 12, a fresh read taken when the review was ten files old; three
of its framings were later corrected by members who compiled what I had reasoned about, and I treat
nothing in it as still standing without re-derivation here. The subject this time is the one my whole
career keys on: whether a decomposition decided early survives the contracts that were designed after
it, and where the crate boundaries are load-bearing for what can be proven rather than merely tidy.

**What I read.** `68_consolidation_seven.md` in full (the standing base), `68b_op_checkpoint_sixteen.md`
and `70b_op_checkpoint_seventeen.md` in full (required; the preset table there supersedes `68`'s, and
`68`'s lowering-door table is void), `72_giesen_the_unexamined_ground.md` and `73_arntzen_the_byte_image.md`
in full (the two most recent), `11_current_shape_draft.md` lines 24 to 70 only (the coverage table this
dispatch is about; the rest is superseded and I did not read it). One `ls` of the panel directory:
files `00` through `73` plus checkpoints and probe directories, nothing after `73`. Because the eleven
rows are decisions, I read them in their ratified form rather than through file 11's paraphrase:
`mock/design_rounds/202607301000_topic.inherited-state-from-the-formalization-round.md` (cited below as
`inherited:NNN`), the decision blocks D1/D2 (`inherited:495,499`), D4 through D9 (`inherited:527-599`),
D10/D11 (`inherited:601,620`), D15/D16/D17 (`inherited:935,949,958`), D25 (`inherited:1133`), D27/D28
(`inherited:1215,1231`), D29/D30 (`inherited:1262,1274`), D43/D44/D45 (`inherited:1765,1787,1795`).
Targeted reads for carried-forward shapes: `26_consolidation_two.md` sections 1.6 and 1.7
(`26:333-431`, `26:505-527`, the six-crate split and the undecided dependency edge) and
`63_consolidation_six.md` section 1.24 (`63:681-689`), which `68:599-604` carries unchanged. Workspace
rules leaned on: `unstable-features.md` (the ratified record of the capacity-as-a-type migration op
led), `arvo-bridge-home-rule.md`, `what-you-can-observe-is-what-you-guaranteed.md`.

**Gates.** Canon gate, reproduced fresh from the repo root this session: `grep -rln
"Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test gate, run fresh and summed per binary:
`cargo test --offline --workspace` from `mock/`, 658 passed, 0 failed, 9 ignored, matching `68:64-65`
and both files since. The suite is green over a tree the canon replaces; per the review's standing
(file 52, the live-defect registry) that green is a measurement of the deprecated implementation and
says nothing about the design, and my surface here (research documents and probes) adds only the
probes' own const assertions. Toolchain confirmed inside the tree: `rustc 1.98.0-nightly (57d06900f
2026-05-27)`, `aarch64-apple-darwin`.

**Standing of the question, stated before answering it.** The eleven rows are not agent drift to be
policed; every one is an op decision, marked `Decision (op)` inline in the inherited-state topic. So
this file checks newer op-ratified ground (the three contracts, the three design rules, the preset
tables at `70b`, the crossing and byte-image chapters) against older op-ratified ground, under op's
own standing that his calls go stale when new material surfaces and under the `70b` steer at genuinely
unexamined ground. Everything below is a suggestion with its reasoning attached; where a row's change
touches an op decision, the change is op's to make. One factual sharpening of the dispatch's premise,
checked: file 72 *pointed at* the table (`72:52-60`) and recommended starting from it, but re-checked
no row; this file is the first row-by-row recheck, sixty-three files after the table was written.

**What is compiled, what is reasoned.** Two probes in `74_probes/` (one four-crate feasibility build,
one expected-fail attack), compiled and run fresh this session inside the repo tree on the pinned
nightly, `--edition 2024`, `#![no_std]`, zero feature gates, outcomes verbatim in
`74_probes/OUTCOMES.md`. Everything else is reasoned from ratified calls and settled shapes, tagged
per claim in the consolidation's provenance form. Per the method constraint ratified at `70b`, no
shipped source or comment is read as design meaning anywhere below; the one migration fact cited
(capacity became a type) is cited from the ratified workspace rule that records it, not from the tree,
and every conclusion survives deleting its citations to anything shipped.

## 1. The verdict table, first, so the next consolidation can lift it

Eleven rows. Zero deletions, zero merges. Three rows change substance (capacity, container, float);
five survive with a new force behind them worth recording; three survive untouched with one sentence
of new obligation each. The two deep findings are sections 3 and 4.

| Row (`11:46-56`) | Verdict | What now forces or changes it |
|---|---|---|
| `arvo-capacity` | **Survives as locus; its ground has moved under it.** | Capacity is already a type (the spine rule's founding firing, op-led, recorded in `unstable-features.md`), and the tower now carries a second, sealed, value-unique type-level natural encoding (`68:549-556`). Two encodings of one concept is the fragmentation this design exists to prevent. Section 3: one bottom carrier crate, both domains alias it per D7. Compiled feasible (`74_probes/`). |
| `arvo-shape` | **Survives, for a stronger reason than it was decided on.** | D43's own load-bearing sentence ("which primitive holds the bits is answered one layer down... invisible to the shape abstraction", `inherited:1765-1780`) is now the `Lowering` charter verbatim: `Lowering` changes no answer (`68:175-177`). The decision predates the contract that proves it right. Also gains a job: the column-shaped capacity file 73 flagged as homeless (`73:443-448`) is a rank-times-width quantity, and rank is this crate's subject. |
| `arvo-geom` | **Survives, untouched by the tower's content.** | Two inherited obligations, not changes: it joins `arvo-graph`/`-spectral`/`-comb`/`-sparse` in the still-undecided dependency edge onto the algebra-contracts crate (`26:505-510`, "nobody has made this call", still true 48 files later); and D10's motors (`inherited:601`) need normalisation, which sits behind division's hold (`68:357-358`). D11's bench matrix is now unambiguously in-bounds work (`68b`: benches encouraged). |
| `arvo-num-systems` | **Survives, with file 64's correction folded in before the crate's type shape ships.** | The membership mechanism is sound for every numeral arvo builds; its uniqueness justification is false against the full ten-member vocabulary (`68:296-313`). The row's fix is already on op's list (68's item 12): scope "finest" to the real/Cayley-Dickson chain, independent predicates per branch. Nothing else in the tower touches this crate. |
| `arvo-platform` | **Survives.** | One new dependency-direction question (section 5): the tower's derived booleans (`68:189-191`, statement 3 "a derived boolean") either place `Bool` below the numeral contracts per `arvo-bridge-home-rule.md`, or the contracts go generic over notko's truth contract per D17 (`inherited:958`). Two spellings, both workable, op's pick. |
| `arvo-container` | **Survives as locus; its contract is substantially rewritten by three ratified results, and D45's placement dissolves.** | The row that flagged itself. Section 4 in full: the saturation-limits contract migrates to the preset `Resolution` axis; the padding law becomes the crate's constructor obligation; the `Layout::Bitpacked` reading becomes its spec question; the refit family becomes an instance of the crossing contract rather than ad-hoc masking. |
| `arvo-bitfield` | **Survives as packaging (D25's proc-macro argument is untouched).** | Two inherited laws: a field read that hands out a `Number` consumes the carrier through the datum's canonicalising projection, the only-door sentence at its fifth site (`68:138-139`); and 73's byte-sharing law (`73:205-215`, W mod 8) decides which bitfield shapes have per-field byte images at all. |
| `arvo-float` | **Survives as packaging; the contents the decision was about have migrated into the tower.** | Section 5: `Specials`, `Underflow`, the quantiser, and the float preset table (`70b`) are `Numeral`/`Policy`/`Lowering` facts now. D30's NaN-as-typestate intent (`inherited:1274`) is fulfilled by the tower (`Specials` product, `Encoding::Canonical` payload collapse), not by a wrapper carve-out. What remains for the crate is real: the IEEE numeral instantiations and the hardware-door lowerings. The "packaging, not a mathematical claim" boundary argument is now forced rather than chosen. |
| predicate concept (notko) | **Survives, strengthened.** | D16's derived-safe/asserted-`unsafe impl` split (`inherited:949`) has been generalised by the panel into the design's own discipline: `Crosses` cites it by name (`68:585`, "D16's safe-blanket-or-unsafe-impl discipline applies"). A decision made for predicates turned out to be a design-wide pattern. Nothing to change. |
| `arvo-pseudorand` | **Survives; inherits two tower-imposed contract sentences.** | What a hash of a `Number<N, S>` consumes is the tower's law, not the hash's: a digest factors through the layer's canonicalising projection, only door (`72:284-308`). And uniform sampling must say whether it is uniform over values or over data, which differ the moment the grid is non-uniform (`72:72`). The hash/PRNG/noise unification itself is untouched. |
| `notko-hlist` + `Cardinal` | **Survives.** | One binding-time sentence owed (section 5): `Cardinal` (`const ZERO; fn succ`) is the value-level counting contract; the shared type-level naturals of section 3 are the type-level one. Same concept, two binding times, and the spec should say which counts where, in the layer-keying rule's own spirit. No mechanism needed. |

*Grounded on: ratified (the D-numbered decisions at the cited `inherited:` lines, `70b`, `68b`),
settled shapes (`68` sections 1.2-1.4, 1.18-1.22, `72`, `73`), compiled (`74_probes/`, capacity row
only), reasoned (everything marked as an obligation or a suggestion).*

## 2. Why this is a layering question and not a naming pass

Three of the design's newer mechanisms make crate boundaries load-bearing in ways the round that drew
the eleven rows could not have weighed, because none of the three existed yet.

**The seal is built from visibility, and visibility is per-crate.** The value-unique encoding's whole
guarantee (`44b`, `68:336-341`) rests on the sealed carriers having exactly one introduction route.
A seal is a private supertrait; a private supertrait is unreachable from any other crate; therefore
*which crate declares the sealed vocabulary decides who can extend it, forever*. That is a correctness
fact, not packaging. Compiled both directions in `74_probes/`: the seal survives the crate split (the
attack crate is refused with rustc's own "sealed trait" diagnostic naming the inaccessible supertrait),
and downstream crates still freely name the types and declare their own local traits over them.

**The orphan rule decides where an impl can live, and the law-key rule decides what it may read.** A
law is keyed on `Numeral` alone and may not read `Lowering` (`68:175-181`); `Crosses` is keyed on the
pair and must read `Lowering` (`68:583-586`). Those are also dependency statements: the crate that
declares laws needs no edge onto the lowering crate, and the crate that declares `Crosses` does. The
six-crate split (`26:514-527`) was ruled packaging because the *closure* mechanism is the phantom-type
proof, and that ruling stands; what the newer contracts add is that the split's edges are no longer
arbitrary even so.

**The spine rule keeps minting capacity-shaped types.** `ShortCap` (`72:212-224`), `ByteCap`
(`73:289-311`), the column capacity (`73:443-448`): eleven firings in, the rule's outputs are
accumulating in whatever crate the firing happened to occur in. A rule that fires this often needs its
outputs to land on one shared definition, or the firings themselves become the fragmentation vector.
That observation is what section 3 is.

*Reasoned, from the cited settled shapes; the seal claim compiled.*

## 3. The capacity row: two type-level natural encodings is one too many

**The situation, stated from ratified ground only.** D1/D2 (`inherited:495-513`, 2026-07-28) lifted
the dimensional foundation out from under the containers, with `arvo-capacity` depending on nothing
but `USize`. Op's own sketch-led migration then made the capacity a type rather than a const generic,
recorded in the ratified `unstable-features.md` (the `generic_const_exprs` FORBIDDEN row: "the
capacity is a TYPE... so no `cap_size` expression sits in type position"). Independently, the panel
built and op ratified (`44b`) a sealed, value-unique type-level encoding of naturals, positives, and
signed rationals for the numeral tower (`68:549-556`: `Nat ::= Z | Pz<P>`, `Pos ::= H | O<P> | I<P>`).
Nobody decided these should be two encodings. The capacity decision predates the tower's encoding by
days, and the two rows of the design have never been read against each other, which is precisely what
`11:46` recorded and what this dispatch asks.

**Why leaving it as two is the failure my whole subject warns about.** A capacity *is* a type-level
natural: an extent, a width, a precision, an exponent magnitude, and an array bound are one concept at
five sites. Two encodings of it means two comparison machineries (the tower needs `Cmp`/`Gcd` over
`Pos` for `Bias`'s normalisation and route Z's predicates, `68:654-657`; the capacity side needs
ordering for its own coverage checks), two arithmetic families, and laws that cannot reach across:
a claim quantified over precisions cannot mention a capacity and vice versa, not because the
mathematics separates them but because the types do. Each copy then grows its own semantics, and the
ecosystem splits along the copies. The eleven spine-rule firings make this concrete rather than
hypothetical: `ShortCap` and `ByteCap` are capacities minted inside the tower's orbit, and the column
capacity is a capacity that must compose with rank, which lives on the other side.

**The replacement, and it is smaller than the problem sounds: one bottom crate, and D7 already names
the pattern.** The sealed value-unique vocabulary becomes its own crate at the bottom of the graph,
below both the capacity/shape stack and the numeral contract crates. Every domain that needs
type-level numbers consumes it and aliases it to its own semantics, which is D7 verbatim
(`inherited:570`, "each domain aliases the cell and the leaf to its own semantics", op's own ratified
pattern for exactly this shape of sharing, stated there for the hlist). Capacity keeps its name, its
crate, and its `Dim<N>`/`Capacity` surface; what changes is that `Cap` is an alias over the shared
carrier rather than a second encoding. The numeral crates keep `Precision: Nat` and friends unchanged;
they already bound on the vocabulary rather than owning it in any ratified text I can find (the
assembled table at `68:549-587` states the encoding but assigns it no crate, so this proposal fills
an open slot rather than moving a settled one).

**Compiled, the whole load-bearing path** (`74_probes/`, four crates plus one attack, no_std, no
gates, pinned nightly): the sealed vocabulary alone in a bottom crate; a capacity crate declaring a
local `Capacity` trait with one blanket impl over every foreign sealed `Nat` (orphan-legal) and
aliasing `Cap13` per D7; a numeral crate naming the identical type as a precision; a fourth crate
proving the payoff, that the capacity crate's semantics reach the numeral's precision with zero glue
(`<<Binary13 as Numeral>::Precision as Capacity>::SIZE == 13` as a const item), and that the two
domains' names unify as one type; and the attack crate refused at the crate boundary with rustc's own
sealed-trait diagnostic. The seal costs the split nothing; the split costs the seal nothing.

**What rides in the bottom crate and what does not, offered as a sorting test rather than a list.**
Value-unique encodings of plain number sets go down: `Nat`, `Pos`, `Bias` (which drags `Gcd`, and
usefully confines the open `Gcd`-for-a-local-`Rhs` coherence question, `68:929-930`, to one crate),
and plausibly the signed `Exponent` spelling, which is a signed integer encoding with no
numeral-specific content. Wrappers that carry numeral *semantics* stay up in the numeral crates and
are D7 aliases over the shared carrier: `Rad<P>` with its `AtLeastTwo` bound, the `ExponentForm`
structures, `Precision` as a role name. The test is the same one the tower already applies elsewhere:
does the type say what a number *is*, or what a number is *for*.

**The consequence for the route Z bench, and it raises the bench's value rather than its cost.** The
facade fork is set presumptively to route Z, gated on the real-consumer compile-cost bench
(`68:703-710`), and route Z's stated need is "the numeral tower's own `Nat`/`Pos`/`Cmp` machinery
built and unpriced against a real consumer's build" (`68:655-657`). Under this section's proposal
that machinery is not facade infrastructure; it is the shared carrier crate that capacity and shape
consume regardless of which route the facade takes, because their own ratified design (D1/D2 plus the
spine rule) needs type-level naturals either way. So the bench's exit condition should be restated:
it prices the shared carrier for every consumer, and a cliff verdict sends the *facade* back to route
Y while leaving the carrier crate standing for the capacity stack. One bench, two decisions informed,
and the machinery it prices gets built once instead of twice.

*Grounded on: ratified (D1/D2, D7, `44b` via `68:336-341`, `unstable-features.md`), settled shapes
(`68:549-587`, `68:654-657`, `68:703-710`, `68:929-930`), compiled (`74_probes/` both probes),
reasoned (the fragmentation argument and the sorting test). The one tree-adjacent citation, the
capacity migration, is to a ratified rule, and the argument survives deleting it: two type-level
encodings of one concept under one feature regime is the finding whether or not either has shipped.*

## 4. The container row: the flagged gap, and what the contracts built over it actually did to it

File 11 flagged this row in its own words: unreviewed "despite being exactly what the new `Lowering`
contract governs from above" (`11:51`). Sixty-three files later the governing is specific, and the
row's recorded content changes in three places.

**D45's distinction survives; D45's placement dissolves, and its own motivating scenario is what
dissolves it.** D45 (`inherited:1795-1806`, op, 2026-07-29) split saturation from the `Identity`
lattice on the reasoning that "saturating arithmetic clamps to what the container can physically
hold, a storage fact", and placed the new representational-limits contract with the container crate
via `arvo-bridge-home-rule.md`. The distinction was right and has only strengthened: the ratified
preset tables (`70b`) carry out-of-range resolution as its own axis (clamp, reduce modulo, refuse,
per preset), fully separate from any lattice fact, and file 71's far-point work made the clamp target
a derived value fact. But those same tables settle the *placement* the other way. The clamp target is
the numeral's far point, the largest finite representable magnitude of the *value set* (`70b:26-38`);
the modulus of `Hot`'s reduce is the numeral's range; the refusal is the numeral's. None of these is
"what the container can physically hold": under the new design the carrier routinely holds more than
the datum (padding, `73:139-191`), and a clamp to carrier capacity would be wrong exactly when
carrier and numeral diverge, which is the divergence D45 itself foresaw ("gives a wrong answer the
moment a type's representable range and lattice bounds diverge") with the roles now assigned by the
layer-keying rule: out-of-range resolution is a value fact, keyed on the numeral, and keying it on
the carrier would be the rule's named failure, a false statement about the numbers (`68:126-128`),
at the layer file 73 already established almost nothing may key on (`73:126-133`). What legitimately
remains on the container side is the coverage condition, that `StoredWidth` covers the datum, which
is a declaration-site check in the `ByteCap` shape file 73 compiled (`73:289-311`), not an arithmetic
contract. Suggested disposition, op's call since D45 is his: the distinction is kept, the contract's
arithmetic half is recorded as realised by the preset `Resolution` axis on `Policy`, and the
container keeps only the coverage check.

**The crate's real contract, assembled from what the panel has ratified or settled since the row was
written.** Carriers, with carrier identity named and almost nothing keyed on it (`73:111-137`). The
padding law as the constructor obligation, forced rather than chosen (`73:139-191`). The
`Layout::Bitpacked` reading as the crate's one open spec question, with opposite byte-image
consequences (`73:193-238`, still open, op's). And the widened `Crosses` obligation at the boundary
where a hand-laid carrier enters (`73:240-287`).

**The refit family, absorbed by D28, turns out to be an instance of the crossing contract, and D28's
own framing is the tell.** D28 (`inherited:1231-1238`) called `Narrow`/`Widen` "strictly container
things: masking to the low N bits, zero-extending, sign-extending... not mathematics." Under the
design as it now stands, half of that is exactly right and half is a category the contracts have
since claimed. Widening is exact: no value moves, pure container work, D28 unchanged. Narrowing
shrinks the representable set, and an operation that shrinks the value set is, by the design's own
maps, `encode ∘ quantise ∘ decode`: the semantic decisions in a narrow are the quantiser's rounding
and the preset's out-of-range resolution, not an unconditional mask (a mask *is* reduce-modulo,
which is one preset's answer, not the operation's definition). And a chain of refits is a staged
conversion, which inherits file 72's compiled result wholesale: naive staging through a wider
intermediate is wrong at measured density, and the licence is round-to-odd intermediates at two
guard digits, spelled in the design's own sealed vocabulary (`72:142-164`). So the refit family's
law is the crossing contract's, stated once, rather than a bespoke masking semantics stated in the
container crate; the container keeps the mechanism and loses the authority to define its meaning.
This is the same shape as the D45 finding, and I read the pair as one lesson: **the container crate
ended up holding two value-semantics contracts because, when the rows were drawn, there was no
numeral contract to hold them; there is now.**

**The `ConstDefault` orphan, a suggestion in passing.** D27 left `ConstDefault` unplaced, explicitly
refusing to file it by proximity (`inherited:1249-1254`). The padding law gives it a subject-matter
home for carriers: construction canonicalises, so the const-construction contract for a carrier is
"the canonical carrier of the zero datum", which sits beside the padding law it depends on. For
value-level types the same trait is value-keyed and belongs with the numeral contracts. Offered as a
sorting, not a ruling; the open item stays op's.

*Grounded on: ratified (D27/D28/D45 at the cited lines, `70b`), settled shapes (`68:126-139`,
`72:129-164`, `73` sections 3 through 7), reasoned (the relocation argument, the refit reading, the
ConstDefault sorting). No compile was needed: every load-bearing mechanism cited is already compiled
by files 72 and 73, and this section only re-keys whose contract each one is.*

## 5. The shorter rows, the residue worth one paragraph each

**`arvo-platform` and the truth-value direction.** The tower's contracts emit derived booleans
(`68:189-191`) and membership predicates. `arvo-bridge-home-rule.md`'s ratified test says a trait
lives where its return type is reachable, which would pull `Bool`, and so `arvo-platform`, below the
numeral contracts. D17 (`inherited:958`) offers the other spelling: notko declares the truth
*contract* the way `Cardinal` is the count contract, the tower's contracts go generic over it, and
`Bool` stays a peer that merely implements it. The second spelling keeps the platform crate out of
the tower's dependency cone and matches the D6/D17 pattern of properties-in-notko; I lean to it, and
either works. One line for op when the crate graph is drawn.

**`notko-hlist`, `Cardinal`, and the binding-time sentence.** With section 3's shared naturals, the
workspace holds two counting vocabularies on purpose: `Cardinal` counts at value level (`const ZERO;
fn succ`, D6), the sealed `Nat` counts at type level. That is not a duplication, it is two binding
times, but left unstated it will read as one, and someone will bridge them ad hoc. The spec owes one
sentence in the layer-keying rule's spirit: a count that decides a type is a type-level `Nat`; a
count computed by a fold at runtime is a `Cardinal`; the mirror between them (a `Nat`'s `VALUE`) is
the projection, and it runs one way. `74_probes/carrier.rs`'s `const VALUE` is that mirror,
incidentally compiled.

**`arvo-bitfield`'s fifth instance.** The layer-keying rule has fired at the comparator, the spectral
partition, the notation face, and the digest (`68:135-141`, `72:284-308`). A bitfield read is the
fifth candidate site: it manufactures a datum from a slice of carrier, and if the field is a
`Number`, the read must land on the canonical datum (the projection as the only door) rather than on
raw extracted bits that might encode a non-canonical cohort member. Stating this now, in the crate's
contract, costs one sentence; discovering it later costs the same defect the registry already records
twice at other layers.

**`arvo-float`'s changed contents, said plainly.** D29 moved "the exceptions" into a float crate;
the panel then moved the exceptions into the tower's own vocabulary, where they became ordinary:
`Specials` is a `Numeral` axis, `Underflow` likewise, NaN canonicalisation is `Encoding::Canonical`,
and the float preset table (`70b`) reads them uniformly with fixed point. What D29's crate still
owns is real and load-bearing: the IEEE interchange-format instantiations (the `Numeral` bundles for
binary16/32/64 and the decimal shapes file 54 built), the hardware-door lowerings (`HostFloat<E>`,
`HostImplemented`), and the `Crosses` impls for IEEE-shaped hand-laid layouts. That is a bindings
crate, and "packaging rather than a mathematical claim" (`11:53`) turns out to be exactly the right
description, now enforced by the design rather than asserted by the row.

*Grounded on: ratified (D6, D17, D29, `70b`), settled shapes (`68:135-141`, `72:284-308`, file 54
via `68`'s carry), compiled (only the `VALUE` mirror, incidental to probe 1), reasoned (the rest).*

## 6. What this hands forward

**For the next consolidation, in provenance form.** *The design round's eleven-row taxonomy
(`11:44-56`) survives its first recheck with zero deletions and zero merges. Three rows change
substance: `arvo-capacity` re-grounds on a single sealed type-level number crate at the bottom of the
graph, which the capacity stack, the numeral contracts, and the facade's route Z all consume, per
D7's alias pattern, compiled feasible with the seal holding across crates (`74_probes/`);
`arvo-container` keeps its carriers and gains the padding law, the `Layout` question, and the
coverage check, while D45's saturation contract is realised by the preset `Resolution` axis and the
refit family's narrowing half becomes an instance of the crossing contract with the round-to-odd
staging licence; `arvo-float` becomes the IEEE bindings crate, its former exceptional content now
ordinary tower vocabulary. Five rows survive with new force (shape by the `Lowering` charter,
predicate-in-notko by `Crosses`'s adoption of D16, num-systems modulo file 64's scoping, bitfield
under the only-door law's fifth site, pseudorand under the digest law). The route Z compile-cost
bench prices shared machinery, not facade machinery, and its exit condition should say so.*
(Grounded: ratified `inherited:495-1806` decisions as cited, `70b`, `44b`; settled shapes `68`,
`72`, `73` as cited; compiled `74_probes/` both probes.)

**Open, stated rather than resolved, all op's.** The carrier crate's name and exact contents (the
sorting test of section 3 is offered; the list is not). D45's disposition as suggested in section 4.
The platform-versus-notko truth-value direction. The algebra-contracts dependency edge for the L2/L3
crates and now geom, undecided since file 26 and still nobody's call. The `Layout::Bitpacked`
reading, already open at file 73. And the six-crate numeral packaging gaining a seventh, bottom
member is a packaging change under `26:514-516`'s own ruling, cheap by that ruling's own terms, but
it is a change to a stated table and goes to op with the rest.

**For the exploring rhythm.** This file closes the `11:44-56` recheck as a question; what it opens
is one dispatch-sized item the recheck surfaced rather than resolved: the constructive-extensibility
compile file 72 already named (`72:71`) now has a sharper form, since section 3's carrier crate makes
"extend the tower from outside" precise: a foreign crate can mint numerals and capacities over the
shared vocabulary but can never mint vocabulary, and one honest foreign-numeral compile against the
model tower would check the first half the way `74_probes/` checked the second.

## 7. Table-diff self-check and verification

The verdict table in section 1 was checked row by row against the prose of sections 3 through 5 and
against the cited source lines, re-read at the moment of writing: every `inherited:NNN` citation was
re-grepped this session (the decision headings at 495, 499, 527, 542, 560, 570, 601, 620, 935, 949,
958, 1133, 1215, 1231, 1262, 1274, 1765, 1787, 1795 all confirmed), and every `68`/`72`/`73` citation
was checked against the file rather than against my memory of reading it. The dispatch's own factual
premises were checked: the table is at `11:44-56` as stated; "one row says yes" is exact; "eleven say
no" counts two qualified rows (`11:49`, `11:53`) as not-reviewed, which their own wording supports;
"has not returned to that table" is precise up to file 72's pointer at it, stated in section 0. Canon
gate, test gate, and toolchain reproduced fresh at the top. Two probes compiled and run this session
inside the repo tree on the pinned nightly; the expected-fail refused with the expected diagnostic,
reproduced verbatim in `74_probes/OUTCOMES.md`. Every design conclusion survives deleting its
tree-adjacent citations, checked sentence by sentence against the `70b` deletion test: the
two-encodings finding is a claim about one concept under one feature regime regardless of what
shipped; the D45 relocation follows from `70b`'s ratified tables and the layer-keying rule alone; the
refit reading follows from the design's own maps and file 72's compiled staging result.
