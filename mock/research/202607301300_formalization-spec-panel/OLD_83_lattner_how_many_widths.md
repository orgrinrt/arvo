# 83. How many widths: three levels, one axis, and the ratified tables already decided the question nobody wrote down

Chris Lattner, file 83. I wrote file 12 (a fresh read, three of whose framings later members
corrected by compiling them; I treat none of it as standing) and file 74 (the taxonomy recheck,
whose crate rows this file's answer lands on). The question is op's, set at `82b:31-35`: the
lowering charter names two width levels, the design's evidence needs three, and where padding
bits live decides whether a ratified crossing statement has content on two of the four presets.
This is the first of the two independent reads the call requires; a second read is owed before
anything below hardens.

**What I read.** `78_consolidation_eight.md` in full, the standing base, plus its verification
section against the tree fresh. The deliverables since: `79_dolan_what_capacity_is.md`,
`80_leroy_the_verification_bundle.md`, `81_fog_is_the_bitpack_cost_inherent.md`,
`82_pesce_the_stretch_assembled.md`, each in full, and op's own `79b` and `82b`. Behind the
consolidation, with licence since each is a derivation this question sits on: file 73 sections 1
through 4 (the two maps and the forced padding law, in the words that produced them), file 75
sections at `75:95-125` (the one-meaning derivation and its reading of "minimum"), file 70 at
`70:140-180` and `70:333-347` (the preset rows' own arguments), file 11 at `11:100-180` (the
charter's original axis table, for what `StoredWidth`'s instances were named), file 67 via
`68:116-141` (the layer-keying rule's full statement), and my own file 74 sections 2 through 5.
One `ls` of the panel directory, current through `82_probes`. The shipped tree I touched for
two purposes only: the standing canon-gate greps, and confirming three factual claims before
reasoning from them (the tautological test at `arvo-tensor/tests/capacity.rs:14-18`, the git
state of `mock/Cargo.toml` at HEAD, and the count of the word "container" in the consolidation).
No conclusion below rests on shipped source, and every one survives deleting the tree citations.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1,
empty. Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary from every
`test result:` line: **666 passed, 0 failed, 9 ignored**, matching files 81 and 82. One process
item closes with it: the manifest gap file 82 section 5.1 found is **fixed at HEAD**, commit
`2e2b423` ("bench: register the plan-driven and mac bitpack variants") adds the ten bench
crates to `mock/Cargo.toml` (10 hits) and the lock (20 hits), so the 666 is now reproducible
from a fresh clone, which it was not when file 82 measured. I touch no shipped surface; the one
disqualifying test on record (`arvo-tensor/tests/capacity.rs:14-18`, three tautological lines,
`cap(3) == cap(3)` after monomorphisation) stands exactly as `78:874-876` and file 82's
confirmation carry it, flagged for deletion, outside the panel's scope to touch. Toolchain
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside the
tree this session. The bench harness was **not run** (its orchestrator overwrites committed
artifacts, `81:38-44`); nothing below is a runtime performance claim, so nothing below needed
it.

**What is compiled, what is reasoned.** Sections 1, 3 and 4 lean on three probes in
`83_probes/` (outcomes and exact commands in `83_probes/OUTCOMES.md`), written and run fresh
this session: a `no_std`, zero-gate, compile-only model of the three levels with the whole
preset matrix and the group-exactness theorem asserted in const position; a run enumeration of
the ungoverned region's misordering over its whole 57,344-pair matrix at the preset table's own
width; and an expected-fail pinning the level ordering as an E0080 declaration-site refusal.
Sections 2, 5 and 6 are reasoned from ratified text and settled shapes, and say so per claim.
Everything is offered as suggestion; the calls are op's.

---

## 1. The answer, stated once

**The design has three width levels and exactly one width axis.** The other two levels are
derived projections, one downward from the encoding, one upward through the dispatch. A level
is a place in the map chain where bits physically exist; an axis is a declaration a consumer
makes. The charter conflated the two, and the repair is to add a level, not an axis.

| level | quantity | declared or derived | keyed on | governed by |
|---|---|---|---|---|
| fields extent, `W_F` | what `Encoding::Fields` occupies | derived (from the `Numeral` parameters, by the encoding) | `Encoding` | statement 0, quantified over every bit pattern of this width (file 80's closure, `80:90-102`) |
| stored width, `W_S` | what `Lowering` declares: the carrier | **declared**, the one axis (`minimum` = `W_F`, `doubled` = twice the logical width) | `Lowering` | statement P, over the bits at `[W_F, W_S)` (`78:643-648`) |
| container width, `W_C` | what the dispatch allocates | derived (a type-valued projection of `W_S` through the menu under `Dense`; the group arithmetic under `Bitpacked`) | the (`Lowering`, `Layout`) pair | **nothing today**; the container clause of section 3 |

And three maps, confirming file 82's count and correcting one of its assignments: `embed : D ->
Carrier` carries the padding law, forced (`73:139-191`); `place : Carrier -> Container`, the
previously unnamed middle map, owns the `[W_S, W_C)` region and the container clause; and
`materialise : Container -> Bytes`, whose domain is the container and which is thereby a pure
relabelling **unconditionally**, at both layouts, because a container is whole bytes by
construction (a dispatched primitive under `Dense`; a group of `G = W_S * P / 8` whole bytes
under `Bitpacked`, whole by the theorem in section 4). File 73's per-value-materialise
nonexistence under `Bitpacked` and file 81's group arithmetic become one mechanism: `Layout`
does not remove the container level, it sets the granularity at which the level exists.

Two candidate levels dissolve, per the dispatch's own instruction to say so. **`LogicalWidth`
is not a fourth level**: file 11's charter carried it as a primitive `Numeral` axis
(`11:164`), the ratified trait table no longer does (`78:620-626`, `Numeral` is `Radix`,
`Precision`, `Exponent`, `Domain`), and the quantity survives only as the friendly name of the
fields' extent, which the encoding derives from those four. And **the container is not an
axis**: it is computed from the declaration, it must appear in type position (it is the repr),
so by the spine rule it is a type-valued projection, and the review has already counted it,
without naming it as a level, in the spine rule's own firing list ("container widths",
`78:128`, the firings through the shipped dispatch at `68:99-100`). The level was in the design
as a firing the whole time; it was never in the charter as a place bits live. Declaring it
independently would let a declaration disagree with the projection, which is the layer-keying
rule's named failure, a false statement with the compiler's authority behind it (`68:126-128`).

*Grounded on: ratified (`78:409-441` both preset tables, `78:552-556` the one meaning,
`78:643-648` statement P), settled shapes (`73:61-88`, `80:90-102`, `82:343-408`,
`68:98-101`, `68:116-141`), compiled (`83_probes/probe_1`, the full matrix in const position),
reasoned (the level-versus-axis distinction and the dissolutions).*

## 2. What "minimum" means is forced by ratified material, so the fork file 82 left for op is narrower than it looks

File 82 posed the fork as open: "does `StoredWidth` denote the fields' extent or the
container's?" (`82:386-393`), and op carried it forward as the gap to derive (`82b:31-35`).
Derived: **the ratified material already forecloses the container reading, three ways, and the
fields-relative reading is the only one under which the tables mean anything.**

First, `Cold`'s own ratified row is `StoredWidth` minimum with `Layout` bitpacked, and
`Layout::Bitpacked`'s ratified one meaning is zero inter-value padding (`78:552-556`, op's
`77b`). If minimum meant the container's width, a 13-bit numeral's packed stream would carry
16 bits per value, three of them padding, which is the byte-aligned-slot reading the
ratification names as **not** `Bitpacked` but `Dense` at a narrow stored width. The container
reading of "minimum" contradicts a ratified call outright.

Second, file 75 derived this in a locked deliverable before the ratification leaned on it:
"`Hot` and `Cold` share the word 'minimum' for `StoredWidth` while differing only in `Layout`.
That is the tell that 'minimum' names the carrier's *logical* width (the exact bit count a
numeral needs, 13 here, identical for both rows), not a rounded, native-register width that
would differ between a speed-first preset and a storage-first one" (`75:106-109`). The
argument survives the deletion test: it reasons from the table's own shape, not from any tree.

Third, the axis's original instance names say it: `Minimum` and `DoubleLogical` (`11:164`).
"Double the logical" is a logical-relative quantity by its own name; an axis whose second
instance is logical-relative does not have a container-relative first instance.

So `StoredWidth` denotes **neither** horn of file 82's fork as a redefinition: it is the
middle level, the carrier, and "minimum" pins the carrier to the fields' extent. The fork
dissolves rather than resolves, and what remains genuinely open is not which meaning the word
carries but what governs the level above it, which is section 3. One word of spec text closes
the ambiguity permanently: *`StoredWidth` is the carrier's width; `minimum` means equal to the
fields' extent; `doubled` means twice the logical width; the container is not `StoredWidth`
and is never declared.*

*Grounded on: ratified (`77b` via `78:552-556`, `70b` via `78:409-441`), settled shapes
(`75:106-114`, `70:150`, `70:172-176`, `11:164`), reasoned (the three-way foreclosure).*

## 3. The ungoverned region, its law, and what the ratified statements' vacuity actually means

With the reading forced, the preset matrix is (compiled, `83_probes/probe_1`):

| preset, 13-bit fixed | `W_F` | `W_S` | `W_C` | `[W_F, W_S)` (statement P) | `[W_S, W_C)` (ungoverned) |
|---|---|---|---|---|---|
| `Hot` (minimum, dense) | 13 | 13 | 16 | empty | **3 bits** |
| `Cold` (minimum, bitpacked) | 13 | 13 | group: 8 in 13 bytes | empty | **empty, by theorem** |
| `Warm` (doubled, dense) | 13 | 26 | 32 | 13 bits, declared | **6 bits** |
| `Precise` (doubled, dense) | 13 | 26 | 32 | 13 bits, declared | **6 bits** |
| any IEEE interchange float, minimum | 32 | 32 | 32 | empty | empty |

**Statement P's vacuity on `Hot` and `Cold` splits into a truth and a gap, and the two need
opposite treatment.** On `Cold`, P quantifies over zero bits and zero padding bits exist: the
packed stream carries exactly `W_S` bits per value, so the vacuity is the true statement that
there is nothing to govern. On `Hot`, P quantifies over zero bits while **three physical
padding bits exist one level up**, and that is where file 80's matrix lives: its nine-bit
model (`W_F = W_S = 9` at minimum, u16 container) put 65,024 same-value pairs through a
raw-container compare and every one misordered (`80:365-380`). Re-instantiated at the preset
table's own width this session: 57,344 pairs at 13-in-u16, every one misorders raw, every one
is Equal through the canonical projection, and a dirty zero sits above the largest clean datum
(`83_probes/probe_2`, run, whole matrix, counts asserted). So the crossing contract's padding
guarantee, read as "the padding bits of a stored value are governed," is **false on `Hot`
today**, not because statement P is wrong but because the level the padding lives at had no
name and therefore no law.

**One consequence for file 80 before it hardens, stated plainly because that file is one-pass
and this is its second read on this point.** Its section 6 headline, "it is the first point at
which the padding half of the crossing contract (statement P) has observable content"
(`80:377-380`, echoed at `80:457-459`), holds only under the container reading of
`StoredWidth`, which section 2 forecloses. Under the forced reading its model's seven bits sit
at `[W_S, W_C)`, statement P remains vacuous there, and what the probe measured is the
**ungoverned third level**, which is a sharper finding than the one the file claims: it did
not give statement P content, it demonstrated that no statement has any. The probe is right,
its label is wrong, and its closing advice (any padding model must sit in a class where the
logical width is strictly inside the container) survives unchanged.

**The law the region owes, and who discharges it.** The repair is not to re-scope statement P
to the container. P is a declaration obligation, biting at the one site a hand-laid format is
asserted (`unsafe impl Crosses`), and the container is not declared by anyone, so a
declaration obligation cannot reach it; re-keying P onto the (`Lowering`, dispatch) pair would
put a consumer obligation on a tower-owned fact, keying it more finely than the declarer's own
identity. The container clause is instead **discharged once, by the projection itself**, and
the discharge is file 73's own purity argument applied verbatim at the second map: `place :
Carrier -> Container` is a one-argument pure function wherever the tower constructs a
container, "preserve whatever padding was already there" is not a policy a pure function can
express, so the container's bits outside the carrier are canonical (fixed, and zero is the
cheap choice), by construction, for every tower path. Compiled in shape
(`83_probes/probe_2`'s purity half: two constructions bit-identical, committed padding
observable through a transmute with no declared API, per the perimeter argument at
`73:172-184`). Foreign bytes entering whole (FFI, a deserialiser) already have their
different-shaped precondition assigned to the constructor that accepts them (`78:576-578`);
nothing new is owed there.

**And the observation law completes it.** The only-door sentence, already ratified as the
layer-keying rule's general fix ("a value-keyed operation must consume its operand through a
canonicalising projection, and that projection must be the only door", `68:138-139`), gains
its domain named: **the projection's domain is the container**, not the carrier, because the
container is what a consumer can actually observe (`repr(transparent)` makes every container
bit reachable regardless of shipped API, `73:172-178`). The misordering probe is exactly a
compare that walked around the door; with the domain named, the defect class the registry
records at three layers is closed at the fourth by the same sentence.

So the crossing contract's statement set, complete: statement 0 over the fields' width
(declaration, per encoding); statement P over `[W_F, W_S)` (declaration, per lowering);
**statement C** over `[W_S, W_C)`: *the container's bits outside the carrier are canonical,
established by the projection's pure constructor, and every value- or datum-keyed observation
consumes the container through the canonicalising projection as its only door.* Discharged by
the tower once, a theorem for every generated path, an obligation only at the foreign-bytes
constructor where one already exists.

*Grounded on: ratified (`78:643-648`, `68:138-139` via the rule's ratification), settled
shapes (`80:365-380`, `73:139-191`, `73:172-184`, `78:576-578`), compiled
(`83_probes/probe_2`, whole matrix, run this session; `83_probes/probe_1`, the matrix table),
reasoned (the P-versus-C discharge-site argument, the file-80 relabel).*

## 4. `Layout` is the container-granularity axis, and `Bitpacked`'s zero padding is a theorem

The third case op's dispatch names, bitpacked storage having "neither in the usual sense,"
resolves the same way file 82 suggested and one step further: `Layout` does not delete the
container level, it **selects the granularity at which the level exists**. Under `Dense` the
container is per value, the dispatched primitive. Under `Bitpacked` it is per group: `P = 8 /
gcd(W_S, 8)` elements in `G = W_S * P / 8` bytes, file 81's plan consts. Compiled over the
whole width range the plan serves, 1 through 57, not a sample (`83_probes/probe_1`):
`G * 8 == W_S * P` at every width, and `P` is minimal (no smaller element count lands on a
byte boundary). So **zero inter-value padding is not an obligation anyone discharges; it is a
theorem of the group projection**, statement C's instance at the group granularity, vacuous
and true exactly the way `Cold`'s statement P is. The two layouts give the same three-level
picture with the container at two granularities, and every downstream fact file 81 derived
(the write granule, the partition boundary, `materialise`'s totality on the group) is a fact
about the container level, now nameable as such.

One region remains at a coarser granularity and inherits the same law rather than needing a
new one: a column whose length is not a multiple of `P` has a partial tail group, and the tail
bits past the last value are container padding at the **column** granularity, canonicalised by
the packer's pure constructor by the identical argument. The column capacity that file 73
flagged homeless and my file 74 assigned to `arvo-shape` (`74:64`) is the quantity this
region keys on; nothing else changes.

*Grounded on: ratified (`78:552-556`), settled shapes (`81:199-214`, `81:344-361`,
`82:394-403`), compiled (`83_probes/probe_1`, the 1..=57 theorem), reasoned (the granularity
statement and the tail region).*

## 5. Where each level lives: the layer-keying rule decides the placements, and the crate rows from file 74 receive them

The dispatch asked that the layer-keying rule place the levels rather than be applied after
the fact. Applied, with the rule's own coarsest-layer test at each row:

**The widths themselves are `Nat`s from the shared bottom carrier.** All three levels'
quantities are type-level naturals; under the capacity result (`79`, seconded at `82:215-225`)
they inherit the one seal, one ordering, one arithmetic. No width-specific number vocabulary
exists at any level, which is the two-encodings lesson of my file 74 applied before the
fragmentation happens rather than after.

**`W_F` keys on `Encoding`** and is derived there; its projection is declared where the
encoding contract is, the numeral/lowering contract crates. Keying it on `Numeral` alone would
be too coarse the day a second encoding of the same numeral exists (the design already allows
`Encoding` to change which datum carries a value, `78:193-195`).

**`W_S` keys on `Lowering`**, the declared axis, in the lowering contract crate. Its coverage
condition (`W_F <= W_S`) is a declaration-site E0080 refusal in the `ByteCap`/`ShortCap`
shape, compiled (`83_probes/probe_3`, expected-fail, refuses at the declaration with rustc
naming the assert). This is the check my file 74 left as "the container keeps only the
coverage check" (`74:200-206`), now stated at the right level: it is a **carrier** coverage
check, and it is the level ordering's own enforcement.

**`W_C` keys on the (`Lowering`, `Layout`) pair**, through one projection with one
definition. Under `Dense` the projection consumes the menu of dispatchable primitives, which
is the one genuinely platform-shaped input, and the menu lives below with the storage/platform
side of the taxonomy; the projection itself, the container clause, and the canonicalising
door are the **`arvo-container` row's contract**. That row spent file 74 losing authority
(saturation to the preset `Resolution` axis, refit meaning to the crossing contract,
`74:179-243`); this file gives it what it actually owns, and the crate named "container"
turns out to own the container level: the projection, statement C, the only-door canonical
projection, and the foreign-bytes constructor. Under `Bitpacked` the group consts (`P`, `G`,
window offsets, lane shifts, read headroom) sit as associated consts on the `Layout` instance
in the lowering contract, exactly where file 81 put them; their truth depends on `W_S` alone,
so keying them on the container type or the numeral would be the rule's too-fine and
too-coarse failures respectively.

**The bitfield row's law sharpens for free.** File 74 gave `arvo-bitfield` the only-door
sentence at its fifth site (`74:69`, `74:265-271`); with the levels named, a bitfield is
precisely a hand-laid `place` map (several carriers sharing one container), the byte-sharing
law (`73:205-215`) is the statement that per-field containers exist only at `W mod 8 = 0`
boundaries, and the field-read obligation is statement C's per-field instance. One mechanism,
not a bespoke bitfield rule.

**The hardware door is a container-level requirement.** `HostFloat<E>` exists exactly when
all three levels coincide at `E`'s format width (section 3's matrix, the float row), which is
why the ratified refusal ("a door a target's silicon does not implement refuses to build",
`78:457-461`) is a build-time fact: it is the container projection failing to produce the
format the door names. Stating the door's precondition as "levels coincide at the format
width" replaces a prose condition with a checkable one.

*Grounded on: ratified (`70b` tables, `44b` seal via `68:336-341`), settled shapes (`74`
sections 1, 4, 5; `73:205-215`; `81:199-214`; `79` section 3), compiled (`83_probes/probe_3`),
reasoned (every placement, offered as suggestions to the same op items file 74's section 6
already carries).*

## 6. The fourth rule's clause: every width-derived const names its level

The pricing pillar's test asks whether anything done at runtime had a compile-time
alternative. This stretch showed the sharper failure: a quantity **can** be settled at compile
time, is, and is still wrong, because it was computed from the wrong level. The reason nobody
hit it yet is coincidence, literally: at `Cold` minimum, `W_F = W_S`, so file 81 could say
"every parameter is a function of the logical width alone" and be right at the only
configuration measured. The moment a bitpacked lowering declares headroom (expressible under
the axis as ratified), the plan consts and the value mask part company: the period, stride,
window offsets and lane shifts are functions of `W_S`; the value mask `(1 << W_F) - 1` and
everything statement-0-shaped are functions of `W_F`; `materialise` and the write granule are
functions of the container. Compiled at a model that separates them
(`83_probes/probe_1`, fields 13 in stored 16: `period = 1`, `group = 2` bytes, mask still
`0x1FFF`), against the `Cold` cell where they coincide.

The spec sentence, offered: *a const derived from a width names the level it is a function of
(fields, stored, container), because two levels coinciding at the measured preset is how a
compile-time fact computed from the wrong level survives review.* The known assignments: the
decode and encode plans on the stored width; the value mask and the far point's raw encoding
on the fields side (the far point itself is a value fact of the `Numeral`, not a width fact at
all, per `82:530-533`); `materialise`, the write granule and the partition boundary on the
container. File 82's two unchecked candidates (the far point and the padding mask as
associated consts, `82:715-719`) each gain their level from this table when they are checked.

*Grounded on: ratified (`78:152-166`, the pillar and its test), settled shapes (`81:73-77`,
`81:199-239`, `82:456-506`), compiled (`83_probes/probe_1`), reasoned (the clause and the
assignment table).*

## 7. Out of scope, reported under the standing obligation

**File 82 committed the exact defect it named three paragraphs earlier, and its count is wrong
twice over.** Its section 3.1 states "the word 'container' appears in the consolidation
exactly three times, all as a *transfer coordinate*" citing `78:128`, `78:946`, `78:956`
(`82:361-363`). Grepped this session: the word appears **eight** times in `78` (lines 128,
656, 680, 869, 900, 903, 946, 956), and of the three cited, `78:128` is not a transfer
coordinate at all, it is the spine-rule firing list's "container widths". Two of the
uncounted five are the crate name, three are the shipped dispatch's defect entry. The
substance survives, and is in fact strengthened: not one of the eight names a structural
level a map crosses, and the `78:128` firing is the level appearing in the design without a
name, which section 1 uses. But this is the third file in a row publishing a count it did not
re-derive, and file 82 itself named that pattern as a process defect at its section 5.3
(`82:627-631`) before adding to it. The table-diff obligation still does not reach a member
file's inline counts; at three instances in three consecutive files, that is no longer minor,
and the cheap fix is the one already adopted for the owed list at `82b:50-58`: a count in a
member file names the command that produced it, so a checker is a paste rather than a
re-derivation.

**File 82's section 5.1 finding is closed at HEAD** (commit `2e2b423`, verified: 10 manifest
hits, 20 lock hits), noted so the ninth consolidation records it closed rather than carrying
it.

## 8. What a consolidation could take, close to verbatim

*The design has three width levels and one width axis. The fields extent is what
`Encoding::Fields` occupies, derived from the `Numeral`'s parameters by the encoding, keyed on
`Encoding`, and it is statement 0's quantifier domain. The stored width is the carrier, the
one declared level, the `Lowering` axis, with `minimum` meaning equal to the fields' extent
and `doubled` meaning twice the logical width; the container reading of `minimum` is
foreclosed by ratified material, since `Cold`'s minimum-plus-bitpacked row under the ratified
zero-inter-value-padding meaning would otherwise pack padding into the stream it is defined
not to have. Statement P governs the bits between the fields' extent and the stored width,
and its vacuity at `minimum` is the true statement that no declared padding exists. The
container width is what the dispatch allocates, never declared: a type-valued projection of
the stored width through the menu under `Dense`, and the group arithmetic (`P = 8/gcd(W,8)`
elements in `G = W*P/8` whole bytes, exact at every width 1 through 57, compiled) under
`Bitpacked`, so `Layout` is the axis that selects the granularity at which the container
level exists, per value or per group, with a column's partial tail group as the same level at
column granularity. The level ordering, fields at most stored at most container, is a
declaration-site E0080 refusal in the coverage shape. The region between stored and container
width is where every measured padding misordering lives (65,024 pairs at the nine-bit model,
57,344 at the thirteen-bit preset-table width, every pair misordering raw and Equal through
the canonical door), and it is governed by statement C: the container's bits outside the
carrier are canonical, established once by the projection's pure constructor by the same
forced-purity argument as the padding law, an obligation only at the constructor that accepts
foreign bytes; and every value- or datum-keyed observation consumes the container through the
canonicalising projection as its only door, which places the raw-compare defect class at its
fourth layer under the one existing sentence. `materialise`'s domain is the container, making
its pure-relabelling claim unconditionally true at both layouts. The hardware door's
precondition is that all three levels coincide at the format's width, which every IEEE
interchange row satisfies and which is why the float table never showed the gap. Every const
derived from a width names the level it is a function of: decode and encode plans on the
stored width, value mask and statement-0 machinery on the fields, write granule and partition
boundary on the container, because two levels coinciding at the measured preset is how a
compile-time fact computed from the wrong level survives review. File 80's nine-bit model
measured the ungoverned container region, not statement P's content; its probe stands, its
label is corrected, and its advice that padding models must sit in a class where the logical
width is strictly inside the container stands unchanged.*

## 9. What this leaves open

The second independent read of this whole answer, owed by the convention op named at `82b`
before any of it hardens; sections 2 and 3 are the load-bearing halves to attack (the
forced reading of `minimum`, and the discharge-site argument for statement C being
tower-side rather than a `Crosses` condition). Whether statement C's canonical value is
zero or merely fixed is genuinely free (`73:163-165` already noted the argument needs only
fixedness); zero is cheapest and I assumed it in the probes. Warm's doubled carrier needs one
sentence somewhere about what its headroom bits are between operations (at rest they are
statement P's declared padding, canonical; mid-chain an intermediate result is a different
numeral whose fields occupy them, per the widening dissolution at `78:228-234`, and facts
about the chain key on that numeral); I state the sentence, a second reader should check it
against file 35's three-fact decomposition. And the constructive-extensibility compile
(`78:947-949`, still owed) gains one clause when performed: a foreign hand-laid lowering
must be checkable against statements 0 and P while statement C remains tower-discharged,
which is the boundary between what a consumer asserts and what the design proves, and one
compile would pin it.

Only op's calls are final, and even those go stale. Everything above is evidence and
suggestion, not a ruling.

*Grounded on: ratified (`77b`, `70b`, `82b`, `78:152-166`, `78:409-441`, `78:552-556`,
`78:643-648`, `68:138-139`), settled shapes (`73`, `75:95-125`, `80` section 6, `81` sections
2 through 4, `82` section 3, `74` sections 2 through 5, `11:100-180`, `70:140-180`), compiled
(`83_probes/probe_1`, `probe_2`, `probe_3`, all fresh this session on the pinned toolchain
inside the tree, commands and outcomes in `83_probes/OUTCOMES.md`), measured (none; the bench
harness was not run), tree-fact (`arvo-tensor/tests/capacity.rs:14-18` and the HEAD state of
`mock/Cargo.toml`, existence only), reasoned (sections 2, 5, 6 and every placement and
suggestion, mine).*
