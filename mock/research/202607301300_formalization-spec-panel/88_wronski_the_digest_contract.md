# 88. The digest contract: no new mechanism, a tiered promise, and a fold law of its own

Bart Wronski, file 88. I wrote file 29 (the quantisation contract) and file 70 (the presets
re-derived, which re-derived both preset tables from op's stated intent alone and is now
ratified in full). Neither touches the digest; nothing below carries either forward except by
citation.

**What I read.** `78_consolidation_eight.md` in full, the standing base, including its
verification section reproduced fresh. The eleven deliverables since, in order, each in full:
`79_dolan_what_capacity_is.md`, `79b_op_the_verification_mandate.md`,
`80_leroy_the_verification_bundle.md`, `81_fog_is_the_bitpack_cost_inherent.md`,
`82_pesce_the_stretch_assembled.md`, `82b_op_checkpoint_twenty.md`,
`83_lattner_how_many_widths.md`, `84_leijen_failure_that_is_not_a_range_event.md`,
`85_chlipala_the_closure_audit.md`, `86_giesen_the_levels_assembled.md`,
`86b_op_checkpoint_twentyone.md`, `87_arntzen_partiality_and_mutation.md`. The question is
stated across three of these (the digest's founding chapter at file 72, the three-level
correction at files 83/85/86, and the mutation gap at files 86/87/`86b`), so I read `72_
giesen_the_unexamined_ground.md` in full as well, since the dispatch names it as the one
compiled result to extend, and its own `72_probes/probe_4_digest_keys_on_a_layer.rs` source,
not merely its outcomes file, because section 1 below turns on exactly what that probe's model
widths were. One `ls` of the panel directory, current through `87_probes`. The shipped tree I
touched for nothing beyond the standing canon-gate greps and one existence check
(`arvo-tensor/tests/capacity.rs`, confirmed unchanged, see Gates). No claim below reads shipped
source for meaning, and every conclusion survives deleting every tree citation.

## Gates

Canon gate, reproduced fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1,
empty. Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary by parsing
every `test result:` line myself: **666 passed, 0 failed, 9 ignored**, matching files 81 through
87 exactly, from a clean tree at HEAD (`303d021`). The one disqualifying test on record,
`arvo-tensor/tests/capacity.rs:14-18`, stands exactly as `78:874-876` and files 82 through 87
carry it: three tautological lines (`dim_cap_is_typed_and_exact`, asserting `<Dim<N> as
Capacity>::CAP == cap(N)` against an impl whose own body is `const CAP: Cap = cap(N)`, so it is
`cap(N) == cap(N)` after monomorphisation on all three lines, confirmed by reading the test file
and its impl this session), flagged for deletion rather than improvement, outside this panel's
scope to touch. Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`,
resolved from `rust-toolchain.toml`, confirmed inside the tree this session. The bench harness
was **not** run; its orchestrator overwrites committed artifacts (`81:38-44`). Every performance
claim below is either compiled/emitted-code reasoning or a reasoned cost argument citing an
already-committed bench figure by name, never a fresh runtime number.

**What is compiled, what is reasoned.** Five probes in `88_probes/` (commands and outputs
verbatim in `88_probes/OUTCOMES.md`), written, compiled, and run fresh this session on the
pinned toolchain, standalone `rustc --edition 2021 -O` invocations touching nothing under
`mock/crates`. Sections 2, 5 and 6 are reasoned from ratified material and settled shapes, and
say so per claim. Everything is a suggestion; the calls are op's.

---

## 0. The verdict, stated first

**A digest needs no new mechanism at all. It is a composition of projections this review has
already established, chosen by which equality it is paired with, and the composition's own
domain is now the container, not the carrier, once the three-level correction (files 83, 86)
is applied to file 72's founding chapter. What is new, and load-bearing, is a tiered promise
about what survives the composition (a free theorem for the ordinary case, a named trusted-base
obligation for the door-touched case, mirroring file 87's own split for the byte image) and a
second, independent fold law for the column case that this review has not stated before,
because a digest fold's combine step needs grouping invariance for a reason the numeral fold's
own interior/total safety never had to answer: nothing about a hash accumulator can leave a
value set, so there is no quantiser to avoid firing mid-fold, and the numeral fold's machinery
simply does not apply. What does apply, unmodified, is the multiplicative half's own
exponent-offset shift argument, borrowed for a position weight instead of an exponent.**

Three corrections to the standing record, each stated because it is a statement rather than
merely an addition, per the discipline every consolidation since the fourth has kept:

**First**, file 72's own probe (`72_probes/probe_4`, part a) modelled the Hot preset's exact
widths (13-bit fields, 16-bit container, statement P vacuous because `W_F == W_S`). File 86
already noted, in passing, that the digest's canonicalising projection now has the container
rather than the carrier as its domain (`86:175-179`), but no file has built a case where both
tiers are simultaneously real to show what that correction actually changes. Section 1 builds
it: at the ratified `Warm`/`Precise` shape, a digest that undoes statement C alone (masks to
`W_S`) is still wrong, and only a digest that masks straight to `W_F` is right, compiled.

**Second**, the "datum-keyed" and "value-keyed" digests file 72 named are not two instances of a
digest-specific mechanism. They are the identical canonicalising-projection machinery statement
0, statement P, statement C and `Encoding::Canonical` already established, read off at two
different stopping points in one composition chain. The chain has exactly two useful stopping
points, not three: a third, "carrier-keyed" digest (paired with raw carrier identity) is not a
new design concept, because carrier identity is not an equality anything in this design should
pair a consistency law against (`78:142-144`, "almost nothing should ever be keyed on it"), and
what remains at that stopping point is not a digest in the technical sense at all, it is the
byte image itself, already fully specified by section 1.22's crossing-contract chapter, with a
hash function applied to it for a consumer's own convenience outside the digest law's scope.

**Third**, and this is where the second consumer's needs pay off in the opposite direction from
what a first look suggests: a **datum-keyed** column digest of honestly-constructed data is
computable as a straight hash of the raw contiguous byte buffer, at zero per-element cost, as a
**theorem**, not an obligation, for both layouts, and the dirt surface a raw mutation can expose
is smaller under `Layout::Bitpacked` than under `Layout::Dense`, because bitpacking removes the
per-value padding region entirely and leaves only one tail-group region for the whole column
(section 5, compiled). A **value-keyed** column digest never gets this shortcut, at any
construction discipline, because `Encoding::Canonical` reads the datum's own content rather than
discarding a fixed bit range (section 2, compiled); the two digest kinds are not two prices for
the same operation, they are categorically different operations, and the design should expose
the choice rather than pick one silently, per `arvo-toolbox-not-policer.md`.

---

## 1. The chain, at a shape where both tiers are real

Op's own instinct at the file's dispatch and file 72's own sentence agree on the general form:
"a digest factors through the canonicalising projection of the layer its paired equality lives
at, and that projection is the only door" (`72:290-291`). What neither file 72 nor file 86 built
is a model where the chain has more than one real tier to factor through, because file 72's own
compiled example used the Hot preset's numbers verbatim (`DATUM_MASK: u16 = 0x1FFF`,
`72_probes/probe_4_digest_keys_on_a_layer.rs:26`, a 13-bit datum in a 16-bit carrier). At that
shape `W_F = W_S = 13` (`83:135-141`'s own table, the `Hot` row), so statement P is vacuous and
the "padding" file 72's probe dirtied is entirely the container tier, exactly the relabel file 83
performed on file 80's nine-bit companion (`83:157-166`) and file 86 applied to file 72's own
mechanism in one sentence (`86:175-179`) without a compiled case to back it.

**Compiled**, at the ratified `Warm`/`Precise` shape (`W_F = 13`, `W_S = 26`, `W_C = 32`,
`83:135-141`, `88_probes/probe_1_three_level_digest_chain.rs`, module `warm_shape`). Three dirt
patterns tested independently against a clean datum: statement-P-only dirt (bits `[13,26)`),
statement-C-only dirt (bits `[26,32)`), and both at once. A digest masking straight to `W_F`
(the datum-keyed projection) is immune to all three, in one operation, confirming section 0's
first claim. An intermediate digest masking only to `W_S` (undoing statement C alone, which is
what file 72's own sentence, "a datum-keyed digest consumes the datum (padding masked, nothing
else collapsed)," `72:292`, could be read as describing if "the datum" meant the carrier) is
immune to container-tier dirt but is **not** immune to statement-P dirt, compiled and asserted:
this is the exact case file 72's own Hot-shaped probe could never distinguish, because at that
width the two masks are the same mask. Both scenarios of the probe are reproduced fresh and
`hot_shape` reproduces file 72's own probe_4(a) numbers exactly, confirming the correction is
additive to what already stood rather than a reversal of it.

*Grounded on: ratified (`70b` via `78:409-441`, `77b` via `78:552-556`), settled shapes
(`72:284-300`, `83:54-93`, `83:95-129`, `86:159-179`), compiled (`88_probes/probe_1`, run fresh
this session).*

## 2. Which something a digest is a function of, stated precisely

**A datum-keyed digest is a function of the fields-width datum: undo statement C, undo
statement P, hash what remains, and the two undoings collapse into one mask because both
levels canonicalise to the identical fixed value (zero, per `73:139-191`'s purity argument and
its extension to the container level at `83:167-183`), so the datum-keyed projection is
literally `hash(container & fields_mask)`, no sequential unwind of the levels required in
practice even though the chain has two of them in principle.**

**A value-keyed digest is a function of the class-collapsed canonical datum: everything a
datum-keyed digest is a function of, then `Encoding::Canonical`'s own cohort/NaN collapse on
top (the `V -> D` inverse, `78:592-599`).** This is not a mask. `82:536-539` already classes
`Encoding::Canonical`'s trailing-zero removal as genuinely data-dependent and therefore a
non-site for the fourth rule's const-position test; a value-keyed digest inherits that
non-siteness directly, and no construction discipline, tower-honest or otherwise, changes it.
**Compiled** (`88_probes/probe_3_value_keyed_never_free.rs`): a model decimal cohort pair and a
model NaN-payload pair, both correctly separated by the datum-keyed digest and both correctly
collapsed by the value-keyed digest, with the collapse traced explicitly to the canonicalisation
step's own content-reading rather than to any fixed mask. This is the categorical difference
section 0's third claim states, and section 5 prices what it costs at column scale.

**A third stopping point does not exist, and naming why closes a question the dispatch's own
framing could invite.** The carrier level (statement P's own domain, `[W_F, W_S)`) and the
container level (statement C's domain, `[W_S, W_C)`) are not two more places a digest could
plausibly stop; a digest that stopped there would be pairing itself with **carrier identity**,
which file 73 already named as a third identity notion strictly finer than the datum layer and
explicitly not a layer this design should key facts on (`78:142-144`, "almost nothing should
ever be keyed on it, because the padding bits it distinguishes carry no denotational content by
construction"). A hash of the raw, unmasked container is not wrong to compute; it is simply not
a digest in the sense this chapter's law governs, because it has no equality to be consistent
with in the first place. What it actually is, is the byte image, already fully specified by
`72`'s own crossing-contract chapter (statement 0, statement P, statement C govern exactly what
that image may contain), with an ordinary hash applied for a consumer's own diagnostic
convenience (a debug tool that specifically wants to see padding garbage, for instance, because
it is diagnosing the mutation gap section 3 addresses). That use is legitimate and needs no
design text of its own: it inherits whatever consistency the byte image has, which is none,
because raw carrier identity was never meant to be consistent with anything coarser.

**The pricing pillar's own test names one more site, and it is a known one, not a new one.**
File 83's own list of const-position assignments already names "the value mask... on the fields
side" as a known, live candidate (`83:298-300`, `82:513-521`'s own five-site sweep). The datum-
keyed digest's own field mask, `(1 << W_F) - 1`, is exactly that mask, with a new consumer. It
requires no new placement decision, only the observation that this chapter is a second consumer
of a site the review had already named and not yet checked; nothing here checks it either (see
what this leaves open), but the placement question the dispatch raised is answered by pointing
at existing spec text rather than by inventing a new clause.

*Grounded on: settled shapes (`72:284-300`, `73:139-191`, `82:536-539`, `83:167-183`,
`83:298-300`), ratified (`78:142-144` the seal section's third-identity finding, `78:592-599`),
compiled (`88_probes/probe_3`, run fresh this session), reasoned (the third-stopping-point
dissolution, the mask-site identification, mine).*

## 3. The mutation interaction, tested against the digest's own needs rather than adopted whole

Op held file 86's one-clause mutation fix at `86b` and asked for "the derivation first," not a
second clause layered on the first (`86b:40-48`). File 87 then built a general, two-tier repair
for the byte-image chapter as a whole and left the digest as one of the two facts the gap
reaches, named but not separately worked (`87:392-397`, "the byte image and any datum-keyed
observation below the door... decorrelate together"). The dispatch names this a hypothesis to
test, not a premise to build on, and that is what I did: I derived what the gap means for the
digest specifically, from the digest's own shape, before checking it against file 87's general
statement.

**The digest raises a question file 87's own general clause does not, and the two are worth
keeping separate before either is accepted.** File 87's finding is about a *fresh* observation:
a value-keyed or datum-keyed read, recomputed on demand, is corrupted the instant it reads raw
bytes without passing through the canonical door. A digest is usually not that. The entire
reason a consumer computes one is to avoid recomputing something later, which means a digest is
the one place in this chapter's whole subject where the design's own output is routinely
**cached**. Caching introduces a second failure mode file 87's general clause does not name at
all: a digest computed once, correctly, through the canonical door, and then held, goes stale
the instant the underlying value changes by any means whatsoever, including an entirely ordinary,
fully safe, value-keyed mutation that never touches a padding bit. That is not a defect in the
digest law; it is the ordinary cached-derived-value invalidation problem every language has, and
conflating it with the raw-byte corruption file 87 names would smuggle a second, unrelated
obligation into the digest chapter under the mutation gap's name. **I therefore separate the two
explicitly: the digest law states how to compute a fresh digest correctly; whether and how a
consumer memoizes that digest, and invalidates the memo on every mutation path, is a consumer
policy question with no more design content than memoizing any other derived fact, and arvo
ships the pure function, not a cache.** This is the toolbox-not-policer line drawn in the same
place file 84 drew it for `quantize`'s conformance bound: a diagnostic, not a directive, and here
not even that, because there is nothing arvo could check.

**What is squarely the digest law's business is the raw-byte-corruption half, and I derived it
independently before checking it against file 87's shape.** The load-bearing fact is the one
section 1 already established: a datum-keyed digest that masks to `W_F` before hashing never
reads the bits a raw mutation could dirty, so **for the masked construction the gap does not
reach the digest at all**, at any construction discipline, with no repair needed because there
is nothing to repair; the digest was never exposed. **Compiled** (`88_probes/probe_2_tier1_free_
theorem_vs_tier2_trusted.rs`): a 32-element column, one element's padding dirtied through an
ordinary safe `&mut` (the weakest form of the attack, needing no unsafe transmute at all, per
`87:384-386`), the masked per-element fold unchanged, exact reproduction of file 87's own
per-value finding at column scale.

**The gap only reaches the tier-1 shortcut, and only there does it need a promise.** Section 5's
own free theorem (hash the raw buffer, no masking) is exactly the case where the mutation gap
bites, because that shortcut's whole value is that it reads bytes the masked construction would
have discarded. The tiered structure this independently produces is the same shape file 87 built
for the byte image (a safe-surface theorem plus a named, audited, trusted-base obligation on the
unsafe door), arrived at from the digest's own cost incentive rather than borrowed from the
general clause: **a column never exposed through a raw mutable accessor below the fields' own
width gets the free shortcut as a theorem, structurally, at zero cost to prove per call; a
column that has been, or might have been, so exposed inherits that exposure as a trusted-base
fact the same way a hand-laid `Crosses` impl already is one, and a consumer relying on the free
shortcut anyway is relying on the door's own documented re-canonicalisation postcondition,
unenforced, exactly as unenforced as every other trusted-base fact this review has named.**

**One difference from file 87's own repair is worth stating, because it changes what "second
read" means here.** File 87's obligation is a runtime promise with no enforcement mechanism,
because Rust has no reference-expiry hook and the fact in question ("did this value ever cross
the door") is a history of an instance, not a property of a type. For the digest specifically,
that history *can* be made a type-level fact, if the design chooses to, by splitting the safe
construction path into two types (one that never exposes the raw door, one that has, with a
one-way conversion from the first to the second the moment the door is taken). Under that split
the "which tier" question is answered at compile time, per call site, with no runtime state and
no trust required at the call site that uses the shortcut, because a value of the "never
exposed" type could not have been dirtied. I have not built this; it is offered as a suggestion
consistent with the pricing pillar's own test (a fact settled at compile time beats an
unenforced runtime promise, and here the compile-time alternative genuinely exists, which is
exactly the condition under which the pillar prefers it, `78:163-166`), and it is squarely a
question about the byte-image chapter's own general repair, not about the digest alone, so I
name it here and leave its adoption to whichever member next attacks file 87's own open items.

**What this file does not do, stated plainly per op's own hold.** It does not propose a general
amendment to the layer-keying rule's own text, and it does not close file 87's open collision.
It tests, independently, what the mutation gap costs a digest specifically, under the digest's
own construction (masking, not raw reading), and finds the answer is smaller than file 87's
general framing might suggest: the ordinary datum-keyed digest was never at risk, and only the
optimisation this file itself is proposing (section 5's free shortcut) creates the exposure that
needs the trusted-base accounting. That is a second, independent line of evidence bearing on the
same open collision, offered for whoever performs the second read op asked for, not a
replacement for it.

*Grounded on: ratified (`86b:40-48`), settled shapes (`87:264-283` the postconditions-are-not-
lifetime-claims argument, `87:353-397` the two-tier repair, `84:453-458` the toolbox-not-policer
diagnostic line), compiled (`88_probes/probe_2`, run fresh this session, independent of
`87_probes/probe_2` though it reproduces the identical shape of finding), reasoned (the
caching/corruption separation, the type-level-history suggestion, both mine, offered as
suggestions).*

## 4. The fold vocabulary: not an instance, but a genuinely new law of the same shape

The dispatch's own framing asks whether a digest is an instance of the settled fold vocabulary
(interior safety, total safety, `40:328-345`) or something outside it, and says establishing
this is worth doing rather than assuming. It is outside it, categorically, and the reason is
short: **the numeral fold's two conditions both exist to characterise when a quantiser does or
does not fire mid-fold, and a hash accumulator has no value set to leave.** `fnv1a`'s running
state never overflows in the numeral sense; it wraps, by definition, over its whole domain,
which is precisely the shape `Hot`'s `ReduceModulo` already treats as free rather than as
something needing interior safety at all (`78:415-424`). There is no accumulator-versus-
destination refinement question for a digest, because a digest's accumulator has no destination
numeral it is protecting; it is its own answer.

**But the underlying reason interior safety exists at all, grouping-invariance for a fold this
design wants to dispatch across morsels in parallel, applies to a digest fold exactly as it
applies to a numeral fold, for a reason that has nothing to do with quantisation.** A column
digest is a fold of arity N. This design's own consumers (the workspace's own engine, dispatching
work across fibers and morsels) need a whole-column digest computable without serialising the
column through one thread, which is precisely the property interior safety buys the numeral fold
(a fold computed at any grouping gives the same result, because no quantiser fires in the
interior). The property a digest fold needs is the identical shape, minus the quantiser: **a
column digest's combine step is grouping-invariant when a digest computed by partitioning into
morsels and combining independent partial digests equals the digest computed as one sequential
fold, at every partition.**

**Compiled, and the two properties (grouping invariance, order sensitivity) are shown
independent, which they must be for a column digest to be useful at all.** A naive chained
running hash (the obvious construction, and the one a hand-rolled digest would reach for first)
is order-sensitive but **not** grouping-invariant: `88_probes/probe_4_column_grouping_
invariance.rs` shows a 64-element column's morsel-then-combine result diverging from its own
sequential fold at a genuine split. A positional (polynomial) combine, `sum_i elem_i * B^i`
wrapping in `u64`, **is** grouping-invariant at every tested split (including both degenerate
single-morsel splits) and remains order-sensitive (a two-element swap changes the digest under
both constructions, confirming the two properties are orthogonal and the positional construction
does not trade one for the other).

**The construction's own correctness argument is not new mathematics for this design; it is the
multiplicative half's own exponent-offset shift, applied to a position weight instead of an
exponent.** `68` section 1.9 states the shape directly: "the symmetry that lets an additive
claim transfer directly across a shifted window is not the symmetry a product needs (a product's
equivariant home is a window shifted by twice the offset)," compiled at 254,830,080 instances
with zero failures for `mulnum`'s own exponent sum. A morsel's own positional partial, computed
with local weights starting at `B^0`, is rescaled into the whole column's frame by multiplying
by `B^k` for the morsel's own starting offset `k`, exactly the identical shift-and-recombine
shape, with the position weight playing the exponent's role. **This design already owns the
algebraic fact a grouping-invariant digest combine needs; it has simply never been asked to
supply it for this operation before.**

**The general statement, offered for the design's own use, parallel in shape to `40:328-345`
without claiming to be the same fact:** *a column digest is a fold, and its combine step is
subject to a grouping-invariance requirement structurally analogous to the numeral fold's
interior safety, though it answers a different question (parallel-dispatch correctness, not
quantiser avoidance) and needs no quantiser-shaped machinery to state. A digest construction
whose combine step is a chained, stateful running hash does not have this property and should
not be dispatched across morsels without a serialising barrier; a construction whose combine
step is associative under a stated positional (or otherwise shift-equivariant) rescaling does,
and the rescaling argument is the multiplicative half's own exponent-offset shift, reused rather
than reinvented.* This is offered as design text a hash-family author (`arvo-pseudorand`'s own
row, `78:684`) would need before choosing a combine shape for a morsel-parallel column digest,
not as a ruling on which hash family to ship.

*Grounded on: settled shapes (`40:328-345` the fold's two conditions, `68` section 1.9 the
exponent-offset shift and its 254,830,080-instance check), ratified (`78:684` `arvo-pseudorand`'s
own inherited contract sentence), compiled (`88_probes/probe_4`, run fresh this session),
reasoned (the categorical non-instance argument, the grouping-invariance statement, the shift
reuse, all mine).*

## 5. The second consumer, priced: a free shortcut for datum-keyed columns, smaller under bitpacking

**The theorem, stated once for both layouts.** A `Layout::Dense` column built entirely through
the tower's safe surface (every element embedded via the pure constructor, never touched through
a raw accessor below the fields' own width) has every padding bit, at every level, canonical by
construction, for the identical reason statement C is a theorem rather than an obligation for
tower-generated paths (`83:167-183`). So **the raw contiguous byte buffer is a sound,
deterministic function of the datum sequence alone**, and a datum-keyed column digest reduces to
hashing that buffer directly, with no per-element unpacking, at zero cost beyond a linear byte
scan. **Compiled** (`88_probes/probe_2`): two independently constructed columns holding the same
datum sequence give the same raw-buffer digest, checked by rebuilding rather than by definition.

**Under `Layout::Bitpacked` the identical theorem holds, and the dirt surface it protects
against is strictly smaller, not larger.** File 81 established that interior groups pack `P =
8/gcd(W,8)` elements into `G = W*P/8` whole bytes with nothing left over, a compiled theorem at
every width from 1 to 57 (`81:199-214`, `83:216-218`), so under the ratified single meaning of
`Bitpacked` (zero inter-value padding, `78:552-556`) there is **no** per-value carrier padding
for a raw mutation to reach at all inside a group. The only padding a bitpacked column can carry
is the tail group's own, at column granularity, canonicalised once by the packer's pure
constructor (`83:224-228`). **Compiled** (`88_probes/probe_5_bitpacked_column_digest.rs`): a
W=13, N=65 column (65 = 8 full groups of period 8 plus one leftover value, forcing a genuine
tail), the group-is-whole-bytes theorem checked at this width, the round trip confirmed, the
tier-1 theorem confirmed by two independent honest packings agreeing, and a tail-only dirt
pattern (confirmed by the round trip to carry no live value) decorrelating the raw digest exactly
as the dense case does, but with a dirt surface of one tail region for the whole column rather
than one region per element.

**So the substrate's own cheapest column-digest implementation, a straight hash of the raw
buffer, is correct for both layouts, for free, provided (and only provided) the column's
construction history stays inside the safe surface.** This is the freebie the dispatch's own
framing invited: a promise nobody had made, available at zero cost, because the theorem was
already sitting inside statement C once it was named. It is also, per section 3, exactly the one
place the mutation gap has real teeth: the promise is conditional, and the condition is the same
one file 87's own repair states.

**Value-keyed column digests never get this shortcut, at either layout, for the reason section 2
establishes: `Encoding::Canonical` reads content, not padding.** A value-keyed column digest
therefore costs a genuine per-element canonicalisation pass, `O(N)` in the column's own decode
cost, before any hashing happens at all. For a `Layout::Bitpacked` column this canonicalisation
pass is the same per-element unpacking work file 81 already priced (the plan-driven decoder, not
the naive one, per `78:552-567`'s ratified reading and `81`'s own corrected multiple, **1.29x to
1.50x** dense, not 4.6x, cited by name rather than re-measured, per the gates section's standing
rule against fresh runtime claims): a value-keyed digest over a `Bitpacked` column pays exactly
that multiple on top of the canonicalisation itself, while a datum-keyed digest over the same
column pays nothing beyond the raw byte scan. **The choice between the two digest kinds is
therefore not a style preference; it is the same order-of-magnitude fork file 81 already
measured for decode, wearing the digest's name, and the design should expose it as a named
choice at the call site rather than let a consumer discover the cost by profiling**, per
`arvo-toolbox-not-policer.md`'s own standing instruction against silent policy.

*Grounded on: ratified (`78:552-567` `Layout::Bitpacked`'s single meaning), settled shapes
(`81:199-214`, `81:344-361`, `83:167-183`, `83:216-232`, the cited 1.29x-1.50x figure from
`81` section 3 and `82b`'s own ratification of it, not re-measured), compiled (`88_probes/
probe_2`, `probe_5`, both run fresh this session), reasoned (the choice-exposure statement,
mine).*

## 6. What the design cannot promise, and what it needs from the layer that can

**Cross-target and cross-build stability is out of scope by the same logic that already scoped
`Warm`'s hardware door, and this file adds nothing to that scoping beyond naming it as a digest
consequence.** `78:580-588` already states arvo's byte-image guarantee as a same-process,
same-build-target fact, not a wire format, because native representation is a per-compile target
fact the way `HostImplemented` already decides which float operations reach hardware. A digest
built on the raw byte image inherits that scope exactly: nothing in this design should promise a
digest computed on one target agrees with the identical value's digest computed on another,
and no mechanism is proposed here to change that, matching `78`'s own standing item.

**Freshness of a memoized digest is out of scope by design, not by omission, per section 3.**
Arvo ships the pure digest function (a container to a digest, correctly, per the composition
section 2 states); whether and when a consumer caches that result and how the cache is
invalidated on every mutation path is a consumer policy question this design has no more
business deciding than it has deciding how a consumer caches any other derived fact. Naming this
explicitly closes a scope-creep route the dispatch's own "to key a cache" framing could invite:
the digest law states how to compute correctly, not how to know when to recompute.

**What the design does need from a layer it does not itself own, stated once rather than left
implicit.** If the type-level history split section 3 suggests (a type that structurally proves
"never exposed through the raw door") is ever adopted, it needs the raw door itself, and the
byte-image chapter's own repair (`87`'s section 2.4), to exist first; this file's own free
theorem does not need that split to be *true*, only to be *checkable without trust* at the call
site, and until the split exists the theorem is available on the same trusted-base terms as every
other fact this review has audited that way, named and not silently assumed.

*Grounded on: ratified (`78:580-588`), settled shapes (`87` section 2.4), reasoned (the two
scope statements, mine).*

## 7. Placement, applied per file 83's own method rather than invented fresh

The dispatch does not ask for a crate placement, but the layer-keying rule's own coarsest-layer
test, applied the way file 83 applied it to the three width levels (`83:234-289`), settles it
in one paragraph and closes a question a future taxonomy check would otherwise have to redo.
Nothing in this chapter's composition is new machinery needing a new home: `Encoding::Canonical`
already lives with the encoding/numeral contract, statement P's masking already lives with
`Lowering`, and statement C's canonicalising projection already lives with `arvo-container`
(`83:257-269`). The digest law itself, the sentence stating which composition pairs with which
equality, is a corollary of those three placements rather than a fourth mechanism, and belongs
beside `Crosses`'s own statement 0/P/C in the lowering contract crate, exactly where the crossing
contract's other statements already sit (`78:643-648`). `arvo-pseudorand` (or `arvo-hash`, its
current shipped name, cited as `tree-fact` only) consumes the law and owns nothing about which
container it is fed, unchanged from `78:684`'s own row and the file 74 taxonomy check that
confirmed it (`74:72`, via `78:684`).

*Grounded on: ratified (`78:684`), settled shapes (`83:234-289`, `78:643-648`), reasoned (the
placement application, mine).*

---

## 8. What a consolidation could take, close to verbatim

*A digest needs no new mechanism. It is a composition of the tower's own established
canonicalising projections, chosen by which equality the digest is paired with: a datum-keyed
digest masks the container straight to the fields' own width (undoing statement C and statement
P in one operation, because both canonicalise to the identical fixed value); a value-keyed
digest additionally applies `Encoding::Canonical`'s own class collapse, which reads the datum's
content rather than discarding a fixed range and is therefore never a masking operation, at any
construction discipline. A third stopping point, paired with raw carrier identity, is not a
digest in this chapter's sense; it is the byte image with a hash applied for a consumer's own
diagnostic convenience, outside the digest law's own scope, because carrier identity was never a
layer this design keys facts on. The field mask a datum-keyed digest needs is the same
const-position site the review already named at the fields level; it needs no new placement.*

*The mutation gap does not reach a masked, datum-keyed digest at any construction discipline,
because a mask never reads the bits a raw mutation could dirty; it reaches only the free
raw-buffer shortcut a column digest can take, and there it is real, compiled, reproducing at
column scale under both layouts. A column never exposed through a raw accessor below the fields'
own width has the shortcut as a structural theorem, at zero cost; a column that has been, or
might have been, so exposed inherits the door's own trusted-base re-canonicalisation
postcondition, unenforced, exactly as unenforced as any other trusted-base fact in this design.
Freshness of a memoized digest across an ordinary, fully safe value mutation is a distinct,
out-of-scope problem this design does not attempt to solve; the digest law states how to compute
correctly, not when to recompute.*

*A digest is not an instance of the numeral fold's interior/total-safety machinery, because a
hash accumulator has no value set to leave and no quantiser fires mid-fold. It does need its own,
structurally analogous law for the reason a numeral fold needs one for a different reason:
morsel-parallel dispatch requires the combine step to be grouping-invariant, a claim about
partitioning rather than about quantisation, compiled to hold for a positional (polynomial)
combine and to fail for the obvious naive chained running hash, at every tested split, with order
sensitivity confirmed independent of grouping invariance in both constructions. The correctness
argument a grouping-invariant combine needs is not new mathematics: it is the multiplicative
half's own exponent-offset shift, applied to a position weight instead of an exponent.*

*A datum-keyed column digest of honestly-constructed data is a straight hash of the raw byte
buffer, at zero per-element cost, as a theorem for both layouts; under `Layout::Bitpacked` the
theorem's dirt surface is strictly smaller than under `Layout::Dense`, one tail-group region for
the whole column against one region per element, because interior groups carry no padding at all
under the ratified single meaning of `Bitpacked`. A value-keyed column digest never gets this
shortcut and costs a genuine per-element canonicalisation pass, the same order of decode cost the
review already measured for bitpacked decode; the two digest kinds are a real cost fork and
should be exposed as a named choice rather than picked silently.*

## 9. Out of scope, reported under the standing obligation

**Nothing found in the shipped tree beyond the standing gate; I did not read it for anything
else.** One correction to my own reading of file 72, stated because the dispatch's own charge
was to check cheap factual claims: file 72's section 7 sentence, "a datum-keyed digest consumes
the datum (padding masked, nothing else collapsed)" (`72:292-293`), is not wrong, but it is
under-specified in a way that let both this file's own predecessor probe and its consolidation
carry a mislabel forward for three files (`78`, `86`, and implicitly every reader of either
between them) before section 1 built the case that shows what the sentence actually requires
once a real carrier tier exists. The sentence itself needed no correction; only its one worked
example did, and that correction is section 1's, not a fault in file 72's own prose.

## 10. What this leaves open

- **The type-level history split proposed in section 3** (a construction path that structurally
  proves "never exposed through the raw door") is a suggestion, not a build. It is squarely a
  question about the byte-image chapter's own general repair, and belongs with whoever next
  attacks file 87's open collision rather than as a digest-chapter addendum.
- **File 87's own collision remains unresolved by this file, deliberately.** Section 3 is
  offered as independent evidence bearing on it (the ordinary masked digest was never at risk;
  only this file's own proposed optimisation creates the exposure), not as the second read op
  asked for at `86b`. That second read is still owed against file 87's own general statement.
- **The positional combine in section 4 is a model, not a proposed shipping algorithm**, in the
  identical sense every other digest probe in this review models FNV-1a rather than proposes it.
  A real hash family choice for `arvo-pseudorand` needs its own dispatch, once the panel takes
  that crate up (`78:684`, still panel-unreviewed per `11:55` and `72:72`).
- **The const-position placement of the field mask (section 2's closing paragraph) is named,
  not checked.** File 82's own two live, unchecked candidates (the far point, the padding mask,
  `82:715-719`) are joined by a third, the digest's own field mask, and none of the three has
  been compiled at the const-position rule's own standard.
- **Whether a consumer wanting the free shortcut without trusting the door's postcondition would
  rather have the design refuse at declaration (the `AbsorbingFarPoint`/conformance-bound shape,
  `78:311-316`, `84:453-458`) than rely on an unenforced trusted-base fact is a real design taste
  question**, symmetric to the one file 87 itself left open about its own repair
  (`87:527-531`), and I have a lean (declaration-site refusal, for the same reasons those two
  precedents were chosen) but no standing to settle it alone.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion,
not a ruling.

*Grounded on: ratified (`77b`, `78:684`, `78:552-567`, `78:580-588`, `78:409-441`, `82b`,
`86b:40-48`, `arvo-toolbox-not-policer.md`), settled shapes (`72` in full, `73:139-191`,
`81:199-214`, `82:536-539`, `83` in full, `84:453-458`, `85:435-453`, `86` in full, `87` in
full), compiled (`88_probes/probe_1` through `probe_5`, all fresh this session on the pinned
toolchain, commands and outputs verbatim in `88_probes/OUTCOMES.md`), tree-fact (`arvo-tensor/
tests/capacity.rs:14-18`, existence and current state only), reasoned (sections 2's third-
stopping-point argument, 3 in full, 4's categorical non-instance argument and grouping-
invariance statement, 5's choice-exposure statement, 6, 7, all mine, offered as suggestions and
evidence, not as rulings).*
