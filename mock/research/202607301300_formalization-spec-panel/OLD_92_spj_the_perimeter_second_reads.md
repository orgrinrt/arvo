# 92. The perimeter second reads: the seal holds over its routes and not over its wording, and the mutation theorem is true of a domain nobody has yet written down

Simon Peyton Jones, file 92. I wrote file 46 (what the seal guarantees), which closed the numeral
tower's perimeter on the fourth attempt by enumerating introduction routes by compiler error class
rather than by attacks anyone had imagined, and whose method this file applies twice more. The two
subjects are the two working shapes adopted at `90b` with second reads owed (`91:995-999`): the
sealed `NicheCarrier` vocabulary, attacked the way file 42 attacked `Pos`'s seal, and the two-tier
mutation repair, asked what its structural theorem is quantified over, plus the combined case the
open list names in the same breath.

## What I read

`91_consolidation_nine.md` in full, the governing reference per the dispatch's standing instruction,
its verification section reproduced fresh against the tree (below). `87_arntzen_partiality_and_
mutation.md` in full, the file under second read, with all five of its probes rebuilt and, where they
execute, re-run this session rather than trusted from its OUTCOMES.md. `90b_persona_checkpoint_
twentytwo.md`, which sets this dispatch and is persona-decided, so every ratification it performed is
a line op can strike. One `ls` of the panel directory, current through `91`. My own file 46, re-read
for the route taxonomy (`46:78-87`, `46:187-210`). Targeted greps rather than full reads, each to
check one factual claim before reasoning from it: `88_wronski_the_digest_contract.md` for the
immunity finding the dispatch cites (`88:228-249`), `08_fog_the_union_and_what_it_costs.md` to
establish that its "union" is a union of design proposals and not a Rust `union` (`08:63-75`, and
`08_probes/a_union.rs` greps clean of the keyword), `notko/src/maybe.rs:30-45` for `Maybe`'s shape
and notko's own layout disclaimer, and `mock/crates/arvo-transparent/src/lib.rs:64-125` for the
`Transparent` door's actual surface (by-value `raw`, no `&mut`-shaped door). The workspace perimeter
rule `what-you-can-observe-is-what-you-guaranteed.md`, read fresh for its item 1. Every tree read
above is either a factual-claim check or evidence about why the redesign exists; no conclusion below
reads the tree for meaning, and each survives deleting the tree citation.

## Gates

Canon gate, reproduced from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test gate:
`cargo test --offline --workspace` from `mock/`, summed per binary across every `test result:` line:
**666 passed, 0 failed, 9 ignored**, matching `91:43-44` exactly, from a clean tree at HEAD
(`5dae109`). The one disqualifying test on record, `arvo-tensor/tests/capacity.rs:14-18`, stands
exactly as the registry carries it (`91:957-958`): three tautological assertions, flagged for
deletion, disposition already ruled, outside this panel's scope to touch; I re-read its body this
session and it is what the registry says it is. The dispatch's claim that the bench orchestrator's
artifact-destroying defect is fixed checks out at the source of truth: HEAD itself is `5dae109 fix:
add a section filter to the bench orchestrator`. No number below needed it. Toolchain `rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside the tree
immediately before the first compile.

**What is compiled, what is reasoned.** Nine probe files in `92_probes/` (six for the seal, one each
for the vocabulary shape, the width claim, and the combined case), all built fresh this session,
commands and outcomes verbatim in `92_probes/OUTCOMES.md`; probe 4 is executed as well as built,
because its honest half is a runtime claim about values. Additionally all five of `87_probes/` were
rebuilt and re-run as the first act of this second read; every one reproduces exactly as file 87
recorded it, which I state up front because a second read that trusts the first read's outcome files
is not a second read. Everything else is reasoned and says so at each claim. Everything here is a
suggestion; the calls are op's.

---

## 0. The verdict, stated first

**First read: the `NicheCarrier` seal holds, and I can say exactly over what: the four introduction
routes by which a foreign crate could put an unaudited type under the one audited entry, each
refused by the compiler at a distinct error class, compiled. The hole is not in the seal. It is in
the vocabulary's wording, which admits members (char, bool, references) for which the audited
entry's own trusted sentence is false or meaningless, and in the entry's sentence-scope, which
currently over-collects: three of the four facts it bundles are const-checkable at the declaration
and belong in the provable tier, leaving exactly one sentence that is irreducibly trusted. Narrow
the vocabulary to the `NonZero` family and the entry's trusted base shrinks to one line.**

**Second read: the two-tier mutation repair survives, and the structural theorem is real, but as
worded it is quantified over a domain nobody has stated, and this review has twice found exactly
that gap the hard way. The honest quantification is per byte-owner and per level, mirroring the
crossing contract's own three statements, and it must name the whole perimeter (fields, accessors,
constructors, granules), not only "no raw accessor". The combined case the open list owes is
compiled, and it produces one binding amendment rather than a clean pass: transplanting the
integer-typed raw door onto a niche carrier silently upgrades the door's failure tier from
decorrelation (safe but wrong) to undefined behaviour (unsound), with zero diagnostics, quieter even
than the transmute file 87 exhibited. The repair composes cleanly if and only if the raw door for a
niche-carrying numeral is typed at the niche, in which case the two trusted-base obligations never
co-occur on one door and the padding obligation is const-provably vacuous for every `NonZero`
member. File 88's immunity finding narrows the theorem's urgency, not the theorem.**

Statement 0's ratification at `90b` stands untouched by both reads: I reran file 87's probes and the
compiled asymmetry (a hard `E0004` against a warn-level `invalid_value` lint) reproduces verbatim.
The provable-versus-trusted distinction the design adopted is the right one, and both of my findings
below are applications of it, not exceptions to it.

---

## 1. First read: the fabrication attack on the sealed niche vocabulary

### 1.1 The seal itself, attacked by route, and it holds

File 87 proposed the vocabulary and did not build it (`87:511-514`, its own first open item). I
built it at model scale (`92_probes/probe_1_tower.rs`: the sealed trait with a private supertrait,
one explicit member, and the one audited blanket `unsafe impl<C: NicheCarrier> Crosses for
ViaNiche<C>`), then attacked it on the routes file 46 enumerated for the numeral tower, each as its
own foreign compile against the built rlib:

- **Direct foreign impl** of `NicheCarrier` for a local type: refused, `E0277`, the private
  supertrait bound unsatisfiable (`probe_1a`).
- **The supertrait route**, implementing `sealed::Sealed` first: refused, `E0603`, module private
  (`probe_1b`).
- **The layout-identity route**, a `#[repr(transparent)]` wrapper over an honest member claiming
  membership by being byte-identical to one: refused, `E0277`, same as any other foreign type
  (`probe_1c`). This is the nonvacuous instantiation the separation requirement demands: the model
  separates membership-by-impl from layout-identity at a type where the two genuinely diverge, and
  it is the exact asymmetry file 87 built its whole case on, now confirmed from the seal's side:
  `repr(transparent)` exposes every bit and proves nothing at the trait layer.
- **Reaching the audited entry directly** at a non-member instantiation, `ViaNiche<NotANiche>`:
  refused, `E0277`, at the struct's own bound, before `Crosses` is ever consulted (`probe_1d`).
- The **re-impl** (`E0117`) and **downstream-blanket** (`E0210`) routes are closed by the orphan
  rules with no probe needed, exactly as at `46:124-126`.

Two positive controls so the refusals are refusals of the attack and not of the probe: the honest
member compiles through the entry, and a foreign hand-laid `unsafe impl Crosses` also compiles,
deliberately, because that is the trusted-base tier's own front door working as designed
(`80:104-108`); the seal's guarantee was never "no foreign lowering", it is "the one audited niche
entry covers no unaudited type", and that is what the four refusals establish (`probe_1e`).

**The file 42 route, checked by declaration inventory rather than by attack.** File 42's hole was a
public blanket one layer below the seal that granted membership. The model's blanket inventory is
exactly one impl, and it consumes membership rather than granting it; the design text should state
this as a standing condition on the vocabulary (no granting blanket, ever), because it is the one
route the compiler will not refuse for you if a later convenience adds one.

**Upstream growth is closed by construction, and the two halves of the argument now meet.** A future
toolchain shipping a new niche type cannot silently widen the vocabulary, because membership is by
explicit per-type impl and the language has no way to quantify over "carries a niche" at all. File
87's `probe_1c` established the absence of a general custom-niche mechanism from the author's side
(`pattern_types` refusing, `E0554` outside the tree); my seal establishes it from the vocabulary's
side (membership only by a sentence someone writes). Between them the closure claim is compiled in
both directions.

### 1.2 The hole: the vocabulary's wording admits members its own entry is false for

File 87 words the set as "`core::num::NonZero<T>` at every native width, and whatever else std
documents" (`87:213-214`: "`NonZero<T>` at every native width, `bool`, `char`, references, and
whatever else std documents"), and the consolidation carries the same phrase (`91:260-262`). The one
audited entry's trusted sentence is shaped for a single excluded run at zero with a bias-by-one
debias. Checked member by member, in const position (`92_probes/probe_2`, all compile facts):

- **char is a counterexample, not a member.** Its validity set has a non-inhabitant (0xD800)
  strictly between two inhabitants (0xD7FF, 0xE000), so it is not one excluded run at zero, no
  bias-by-k maps its inhabitants onto a contiguous domain, and the entry's scope condition ("the
  shift must not wrap", `86:92-102`) does not even parse against it. An audited entry quantified
  over a vocabulary containing char asserts a false sentence about one of its members.
- **bool is redundant, not unsound.** Two inhabitants equal 2^1 exactly, so ordinary field-shrinking
  already expresses the domain and the trusted entry buys nothing; its presence dilutes the entry's
  own "genuinely needed" justification.
- **References are a locus error.** A `&T` carries a lifetime; a byte-image carrier with a lifetime
  parameter has no `materialise` (a pointer's validity is temporal, not a property of its bytes),
  and admitting one would put a lifetime inside the `Crosses` vocabulary. Reasoned, not compiled;
  there is nothing to compile because the mistake is in what the sentence names, not in what rustc
  would do with it.
- **The `NonZero` family is exactly right and exactly alone.** One excluded pattern, at zero,
  inhabitant count 2^w - 1, never a power of two (the collision, re-confirmed at both widths file 87
  used), so field-shrinking cannot express it and the entry is genuinely needed there and, among the
  named members, only there.

**The repair is one word: the vocabulary is the `NonZero` family, closed, enumerated, and nothing
else.** "And whatever else std documents" is the phrase to delete: it converts a closed vocabulary
into an open-ended license, and the seal, which is airtight against foreign types, cannot protect
the entry from its own author admitting a member the sentence is false for. If a second niche shape
ever earns admission, it earns a second audited entry with its own sentence, exactly as a second
hand-laid `Lowering` earns its own `unsafe impl`; one entry per shape, never one entry per phrase.

### 1.3 The entry's sentence over-collects, and three quarters of it is provable

The adopted shape bundles into the one audited entry: (i) the excluded pattern's unreachability in
safe code, (ii) the totality of the debias over the inhabitants, (iii) the width claim ("the same
width as the value itself"), and (iv) the no-wrap scope condition. Only (i) is irreducibly trusted.
The other three are functions of type parameters, and the pricing pillar's own clause (`91:113-121`)
says where such facts live:

- **(ii) is a const equation.** The decode is total over the inhabitant set exactly when the
  numeral's domain cardinality equals 2^w - 1, or the decode is declared cohort-style many-to-one,
  which are the identical two options the fields level already has (value-unique, or `Canonical`'s
  class collapse). The concrete failure is compiled: a 2^13-value bounded domain biased into
  `NonZeroU16` leaves 57,343 inhabitants with no decode (`probe_2`), which is an unenforced domain
  side-condition, the exact thing statement 0's hardening forbids at the fields level
  (`80:99-102`). Stated as a declaration-site refusal, this is `E0080` in the same shape as the
  level-ordering refusal at `83_probes/probe_3`; stated as prose inside the trusted entry, it is a
  side-condition wearing an audit as a disguise. It must be the former.
- **(iii) is a const assertion, and the stack already owns the mechanism.** The "same width" claim
  rests on discriminant elision, which std documents as a guarantee for `Option` over the `NonZero`
  family and does not document for any other enum. The stack's own fallible vocabulary is
  `notko::Maybe`, not `Option`, and notko itself already says, in the comment directly under the
  enum, that `Maybe`'s "layout depends on whether `T` happens to carry a niche", shipping
  `MaybeNull<T>` to pin layout "per instantiation via sealed-trait bound + const assertion"
  (notko/src/maybe.rs:40-45). Compiled (`probe_3`): `size_of::<Option<NonZeroU16>>() == 2`
  (documented), the Maybe-shaped model enum also 2 on this pin (a per-pin fact the assertion itself
  pins), and the no-niche negative control at 4 (so the assertion has content). The niche entry
  should require the MaybeNull-style assertion at the construction door and say nothing about width
  in its trusted sentence.
- **(iv) is already per-witness arithmetic** (`86:92-102` checks it at E4M3); it needs only to be
  stated as a declaration-site const refusal rather than an audit obligation.

What remains is one sentence: *the excluded pattern is unreachable in safe code, per the member
type's own documented contract in `core`, not per anything the tower proves.* That is the whole
trusted base of the niche vocabulary, one line, auditable as one line, and everything else the
construction needs refuses at the declaration. This is a strict improvement on the adopted shape on
its own terms: `90b` adopted the split precisely so the design stops confusing the two tiers, and a
trusted entry that quietly carries three provable facts is a small instance of the same confusion in
the other direction, facts that could refuse at compile time being merely promised instead. The
design rule that compile time is the bucket to pour into decides this without taste entering it.

*Grounded on: ratified (`90b` the working shape, `80:99-102`, `91:113-121` the pricing pillar's
clause), settled shapes (`87` sections 1.2-1.3, `86:66-102`, `46:78-87` the route method), compiled
(`92_probes/probe_1_tower` and the four attacks plus controls, `probe_2`, `probe_3`, all fresh this
session), verified at source (notko/src/maybe.rs:30-45), reasoned (the references locus point, the
one-entry-per-shape principle, mine, offered as suggestions).*

---

## 2. Second read: the mutation theorem's quantification, and the combined case

### 2.1 The theorem is true of a domain the wording does not state

The adopted tier 1: "the safe surface never exposes a raw accessor below the fields' own width,
which closes the gap unconditionally and structurally for the entire safe surface" (`91:612-615`,
from `87:361-369`). File 87's probe 3 establishes it at one shape (a single Dense carrier newtype
with a private field), and I re-ran that probe this session; it reproduces. But a structural theorem
is a claim that something is impossible to express, and this review's own history is that such
claims hold for the shapes their authors tried and fail for one they did not (`42`, `80`'s nine-bit
companion). So: what is this one quantified over? Walking it honestly:

- **"No raw accessor" undercounts the perimeter.** The workspace rule's own worked example is a
  property-carrying type that shipped with two public fields
  (`what-you-can-observe-is-what-you-guaranteed.md`, "Why this is easy to miss"): a `pub` field is a
  raw mutable door with no accessor anywhere, reachable as `carrier.0 = dirty` in wholly safe code.
  The theorem's domain must name the whole perimeter as that rule already defines it: no public
  fields, no `DerefMut` to the container, no safe constructor accepting foreign bytes without
  canonicalising (that constructor is statement C's own named obligation site), and no raw
  accessor. The `Transparent` door as actually shipped is inside this domain already: its surface is
  by-value `raw(self) -> Self::Inner` (arvo-transparent/src/lib.rs:67-76, checked as a factual
  claim), and a by-value read cannot leave anything behind.
- **Whole-value replacement is inside the safe surface and harmless, and the theorem should say
  so.** `*place = other`, `mem::replace`, `mem::swap`, `Cell::set`, atomics: all whole-value, and a
  whole value that was canonical at construction stays canonical when moved. Interior mutability is
  therefore not a route. Reasoned; the shape is forced by Rust's own move semantics.
- **A safe `union` field write would be a route, and the design has none.** Writing a union field is
  safe in Rust even though reading one is not, so a literal `union` anywhere in the lowering chain
  would be a safe raw door tier 1 cannot close. Checked as a factual claim: file 08's "union" is a
  union of five design proposals, not a Rust `union` (`08:63-75`; its probe greps clean of the
  keyword). The theorem should carry the exclusion explicitly, one clause, so a future
  representation choice cannot walk through it unannounced.
- **"Below the fields' width" is Dense-shaped wording, and under `Bitpacked` the granule is the
  group.** Adjacent values share bytes (`91:676-678`), so a per-element safe write is a
  read-modify-write of shared bytes, and the byte owner at that level is the column, not the
  element. A column whose safe surface hands out `&mut [u8]` of its backing bytes, an API every
  storage crate is tempted to ship, reopens the gap at column granularity, tail-group padding
  included, with no per-element accessor anywhere in sight. The theorem must therefore be stated
  **per byte-owner, per level**, in exactly the shape the crossing contract already has: the
  carrier's owner re-establishes statement P's region on every safe write, the container's owner
  statement C's, the column's owner the tail group, and each names its own write granule. One rule
  three times, not one rule about one level.

None of this breaks the repair; every clause above is the repair holding, once its domain is written
down. The consolidation sentence to amend is the single word "accessor" and the single phrase
"below the fields' own width": the former becomes the perimeter as the workspace rule defines it,
the latter becomes "below the write granule of the level whose bytes it owns".

### 2.2 File 88's immunity finding narrows the urgency, not the theorem

The dispatch asks which. File 88 found the masked (datum-keyed) digest immune to padding dirt in one
operation, with only the free raw-buffer shortcut exposed (`88:228-249`). That does not narrow the
theorem's quantification, because the shortcut is not an optional extra the design could quietly
drop: under `Bitpacked` the per-element canonicalising alternative costs the decode multiple
(1.29x to 1.50x, `91:652-657`), so the shortcut is the economically load-bearing digest path for
exactly the layout `Cold` exists for, and a repair that protected only the masked path would protect
the path that needed no protection. What the finding does narrow is urgency and blast radius: the
observable consequence of a violated write-postcondition is confined to raw-byte-keyed consumers,
every value-keyed observation stays correct, and so the defect class is "wrong digest", not "wrong
arithmetic". Worth one sentence in the design text, because it tells a future auditor where to look
when a digest diverges: at the doors, not at the laws.

### 2.3 The combined case, compiled: the doors do interact, and one amendment is binding

The open list asks for "a compiled combined case checking the two trusted-base obligations do not
interact" (`91:997-999`). I built it (`92_probes/probe_4`), and the honest answer is that they
interact decisively, in a way neither file 86 nor file 87 stated, and the interaction dictates the
door's type:

- **The integer-typed raw door, transplanted unchanged onto a niche carrier, upgrades its own
  failure tier silently.** File 87's probe 3 door is `unsafe fn to_raw_mut(&mut self) -> &mut u16`,
  and for a padding carrier its violated postcondition costs decorrelation: safe, wrong, observable.
  The identical door shape on a biased `NonZeroU16` carrier admits `*door = 0`, which is not
  decorrelation but undefined behaviour, the optimiser being licensed on the validity range. And it
  is quieter than anything file 87 exhibited: the value-transmute route at least drew the
  warn-level `invalid_value` lint; the place-store through the integer borrow draws **no diagnostic
  of any kind** (compiled this session, zero output; the violating body is compiled and never
  executed, because executing it is the UB). So composing the two working shapes as adopted, with
  no further clause, produces a door whose documented postcondition reads like a correctness
  obligation ("leave it canonical") while its actual violation consequence is unsoundness. A
  trusted-base entry whose stated tier understates its consequence is the exact confusion the
  provable-versus-trusted sentence was adopted to end, one rung further down.
- **The repair is to type the door at the niche, and then the composition is clean.** A door shaped
  `fn typed_mut(&mut self) -> &mut NonZeroU16` is *safe*: every store through it is a
  safely-constructed `NonZeroU16`, which cannot be zero, so the soundness obligation returns to the
  type system and vanishes from the caller entirely; ten arbitrary mutations executed this session,
  the excluded pattern structurally unreachable throughout. The niche's own trusted sentence (the
  one line section 1.3 left standing) stays where it was, in the audited entry, and no obligation
  sits on the door at all.
- **The two obligations then never co-occur on one door, and the padding half is provably vacuous
  where the niche lives.** Every `NonZero` member width is whole-byte, so at `Dense` the container
  equals the carrier and under `Bitpacked` the group arithmetic yields zero pad bits at every
  member width, const-checked across all five widths (`probe_4`). A niche-carrying numeral has no
  padding region for a write-postcondition to govern; a padded numeral has no niche for a store to
  violate. The combined case is clean **because** the amendment separates the doors, and only
  because of it.

The binding amendment for the consolidation, then: *the two-tier repair's unsafe integer-typed door
exists only for lowerings whose carrier has no validity range; a `NicheCarrier` lowering's mutable
door, if it ships one, is typed at the niche member and is safe, and an integer-typed door onto a
niche carrier is forbidden outright, because its violation tier is unsoundness and no documented
postcondition can honestly carry that as a correctness clause.* This is `harness-the-type-system`'s
ladder doing the work the audit would otherwise do badly: the door's type makes the illegal store
unrepresentable, which is strictly better than trusting anyone to not write it.

*Grounded on: ratified (`90b` the working shape, `91:612-626`, `91:997-999` the owed artifact,
`what-you-can-observe-is-what-you-guaranteed.md` items 1-4), settled shapes (`87` section 2 in full,
`88:228-249`, `91:652-657`, `91:676-678`, `08:63-75`), compiled and run (`92_probes/probe_4`, plus
the full re-run of `87_probes/`), verified at source (arvo-transparent/src/lib.rs:64-125, the
by-value door), reasoned (the quantification walk in 2.1, the urgency reading in 2.2, mine, offered
as suggestions).*

---

## 3. What a consolidation could take, close to verbatim

*The `NicheCarrier` seal holds, second-read confirmed by a compiled fabrication attack on every
introduction route (direct impl `E0277`, supertrait `E0603`, layout-identity wrapper `E0277`,
non-member instantiation of the audited entry `E0277`; re-impl and downstream blanket closed by the
orphan rules), with two positive controls, and with upstream growth closed by construction because
membership is per-type impl and the language cannot quantify over "carries a niche". Two amendments
land with the confirmation. First, the vocabulary narrows to the `NonZero` family, closed and
enumerated; the phrase "and whatever else std documents" is deleted, because char's validity set is
not one excluded run (a non-inhabitant sits strictly between two inhabitants, compiled), bool is
expressible by an ordinary 1-bit field shrink (compiled), and a reference is not a byte-image
carrier at all; a future second niche shape earns a second audited entry with its own sentence,
never admission under the first. Second, the audited entry's trusted base shrinks to one sentence,
the excluded pattern's unreachability in safe code per the member's own documented contract;
inhabitant-totality of the decode (domain cardinality 2^w - 1, or a declared cohort-style
many-to-one decode, the identical two options the fields level has), the width claim (a
MaybeNull-style const assertion per instantiation, the mechanism notko already ships, since
discriminant elision is documented only for `Option` and the stack's vocabulary is `Maybe`), and the
no-wrap condition are all const equations over type parameters and refuse at declaration, per the
pricing pillar.*

*The two-tier mutation repair stands, second-read confirmed, with its structural theorem's
quantification now stated rather than implied: per byte-owner and per level, mirroring statements 0,
P, C. The safe surface of the level's byte owner (carrier at Dense, column group at Bitpacked)
exposes no public field, no `DerefMut` to the representation, no foreign-bytes constructor outside
statement C's named obligation site, and no accessor below that level's own write granule;
whole-value replacement and interior mutability are inside the safe surface and preserve
canonicality by move semantics; a Rust `union` anywhere in the chain would be a safe raw door and
is excluded by name, no instance existing in the design. The combined case with a niche carrier is
compiled and rules the doors apart: an integer-typed raw door onto a niche carrier is forbidden,
because the identical door shape whose violation costs decorrelation on a padded carrier costs
undefined behaviour on a niche carrier, with zero diagnostics on the violating store; a
`NicheCarrier` lowering's mutable door, if shipped, is typed at the niche member and is thereby
safe, the soundness obligation returning to the type system. With the doors so separated the two
trusted-base obligations never co-occur: every `NonZero` member width is whole-byte, so the padding
region is empty at both layouts, const-checked. File 88's immunity finding narrows the gap's blast
radius (raw-byte-keyed consumers only), not the theorem's domain, because the exposed shortcut is
the economically load-bearing digest path under `Bitpacked`.*

Definitional completeness, applied to my own text: "write granule" is defined at `91:676-678` (the
period `P` under `Bitpacked`, the value itself under `Dense`); "byte owner" I define as the type
whose safe surface can reach a level's bits without `unsafe` (carrier newtype, container projection,
column), and if that term survives into a ratifying text it needs that one definition carried with
it; "inhabitant set" is the type's own validity range as std documents it. Nothing else above uses
a term the corpus has not already defined.

## 4. What this leaves open

- **The seal probe is a model, not the tower's own generated impls.** The routes and error classes
  transfer (they are properties of the trait grammar, not of the member list), but the real
  vocabulary's declaration should be attacked once more in place when it is written, the same way
  file 46's committed adversary rides the real tower. Cheap: the six files in `92_probes/` are the
  template.
- **The cohort instance of the mutation gap (file 87's fourth instance) is still named, not
  compiled.** My per-level quantification covers it in statement form (the datum's own bits are
  inside statement 0's domain, so a raw write there is a legal datum change, and the divergence is
  between datum-keyed and value-keyed observations, not a canonicality violation), but the compiled
  decimal analogue file 87 called cheapest remains unbuilt.
- **Whether `Maybe` itself should carry the niche pinning, or the construction door should.** I
  showed the assertion mechanism exists and where notko already ships it; whether the fallible tier
  routes through `MaybeNull`, through a new `Maybe`-with-assertion door, or through the numeral's
  own declaration is a placement question with more than one defensible answer, and I have
  deliberately not resolved it. The distinguishing evidence would be which placement keeps the
  assertion adjacent to the audited entry's one trusted sentence.
- **The forbidden integer door on niche carriers wants a compile-fail test when the real vocabulary
  lands**, per `catalogue-edge-cases-as-tests.md` and the standing practice that a refusal nothing
  pins can be deleted by accident. Nothing to write today; the canon is not source. Named so it is
  not lost.
- **Both of my amendments are themselves one-pass.** The vocabulary narrowing and the door-typing
  rule each have one read (mine). Per the review's own discipline neither should harden without its
  own second look, though both are small and each rides on a compiled artifact rather than on my
  say-so.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (`90b`, `91` sections 1.12, 1.22, and the open list at `91:993-1002`),
settled shapes (`87` in full, `88:228-249`, `46:78-210`, `86:66-102`), compiled (`92_probes/` nine
files, `87_probes/` five files re-run, commands and outputs verbatim in `92_probes/OUTCOMES.md`, all
fresh this session on the pinned toolchain inside the tree), verified at source
(notko/src/maybe.rs:30-45, arvo-transparent/src/lib.rs:64-125, `08_probes/a_union.rs` keyword grep,
HEAD `5dae109`), reasoned (sections 1.2-1.3, 2.1-2.3, and the amendments, mine, offered as
suggestions and evidence, not as rulings).*
