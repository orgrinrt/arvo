# 82. The stretch assembled: three levels where the charter names two, a fold width that stops one binade short of the design's own commitment, and the const-position rule with its boundary drawn

Angelo Pesce, file 82. I wrote file 30 (the identity half assembled) and file 65 (pricing
the L0 migration). This is an assembly pass over the four deliverables since the eighth
consolidation, all of which are one-pass and two of which correct work that was already
ratified.

**What I read.** `78_consolidation_eight.md` in full, the standing base;
`77b_op_checkpoint_nineteen.md` and `79b_op_the_verification_mandate.md` in full, both
op's; and the three deliverables since, `79_dolan_what_capacity_is.md`,
`80_leroy_the_verification_bundle.md`, `81_fog_is_the_bitpack_cost_inherent.md`, each in
full. Behind those, for specific derivations rather than as background:
`80_probes/OUTCOMES.md` and `80_probes/probe_1_foldexact_type_level.rs` in full, because
this file rebuilds what that probe built; `79_probes/probe_1` and `probe_2` at their
`last_index` and `in_bounds` definitions, because file 81's rule bears on their shape;
`62_probes/primary_sources.md` in full, to test a staleness claim rather than repeat it;
`mockspace/bench-core/src/lib.rs` at `build_input_bytes`, because file 81 cites it for a
structural cap. One `ls` of the panel directory, current through `81_probes`. The shipped
tree I touched for four things and no more: the canon-gate greps, the flagged test at
`arvo-tensor/tests/capacity.rs`, its `Capacity` impl in `arvo-tensor/src/capacity.rs`, and
`arvo-strategy/src/lib.rs` at the `RANK` citation. All four are checks of a factual claim
before reasoning from it. **No conclusion below survives on a shipped-source citation, and
every one of them survives its deletion.**

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1,
empty. Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary by
parsing every `test result:` line, **666 passed, 0 failed, 9 ignored**, matching file 81's
count. Section 5.2 below records why that number is not reproducible from the committed
tree, which is a finding rather than a discrepancy. I read the bodies of the tests in the
surface I touch. The one disqualifying test on record is real and I confirmed it rather
than carried it: `arvo-tensor/tests/capacity.rs:14-18` asserts `<Dim<3> as Capacity>::CAP
== cap(3)` against an impl whose body is `const CAP: Cap = cap(N);`
(`arvo-tensor/src/capacity.rs:48`), so after monomorphisation the assertion is `cap(3) ==
cap(3)`. All three of its lines are the same tautology, not just one. It is not a weak
test, it is not a test, and it should be deleted rather than improved, exactly as
`78:874-876` carries it. It is outside the panel's scope to touch. Toolchain `rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`, confirmed inside the tree this session.

**What is compiled and what is reasoned.** Sections 1.1, 1.2, 3.3 and 4 are compiled: they
trace to `82_probes/` (three probes, outcomes and timings in `82_probes/OUTCOMES.md`),
built on the pinned toolchain inside the tree this session. Section 1.3's host facts are
measured with `sysctl` and its CSV figures are recomputed from the committed artifact.
Sections 2, 3.1, 3.2 and 3.4 are reasoned, and say so at each claim. Everything I offer is
a suggestion; the calls are op's.

---

## 1. The one-pass files, attacked

Three deliverables, none read by anyone. Two of them corrected ratified work, which is the
condition under which a second reader matters most, because a correction that lands
unchecked becomes the new unexamined base.

### 1.1 File 80's exact fold width does not exist at binary256, and stops fifteen units short of the default limit at binary128

This is the sharpest finding in the file and it is compiled.

File 80 built `foldexact(P, A) = bitlen(A * (2^P - 1))` at the type level, verified it on
114 cells, priced it at under a millisecond per cell, and recommended retiring `foldnum`
to prose (80:196-204). The construction is correct and the recommendation is right. But it
named one bound and dismissed it in a sentence that is wrong on both of its halves:

> One honest bound: `AllOnes` recurses on the *value* of P, so a pathological precision in
> the thousands would meet the default recursion limit; real precisions are two orders of
> magnitude below it. (80:192-194)

rustc's default `recursion_limit` is **128**, not thousands. A precision two orders of
magnitude below 128 is about one. And the design commits to IEEE interchange-format
instantiations (`78:682`), a family that includes binary128 at p = 113 and binary256 at
p = 237.

**Compiled** (`82_probes/probe_1_allones_at_real_precisions.rs`, file 80's machinery
verbatim, grammar literals asserted against their decimal values first so a refusal cannot
be a mis-typed literal). binary16, binary32, binary64 and binary128 all compile. binary256
refuses:

```
error[E0275]: overflow evaluating the requirement `O<I<I<I<O<I<H>>>>>>: AllOnes`
    = help: consider increasing the recursion limit by adding a
            `#![recursion_limit = "256"]` attribute to your crate
    = note: 126 redundant requirements hidden
```

**Two independent ceilings, both landing at 128, and they coincide by arithmetic accident
rather than by design.** Located exactly by sweeping P from 100 to 139 in two modes:

- With the projection forced to normalise, P <= 128 compiles, P = 129 gives `E0320`
  (drop-check rules overflow), P >= 130 gives `E0275`. That is rustc's default recursion
  limit.
- With `#![recursion_limit = "1024"]` and `Nat::VAL` forced to evaluate, P <= 128 compiles
  and P >= 129 gives `error[E0080]: attempt to compute 2_u128 * u128::MAX, which would
  overflow`. That is the `VAL` carrier, and `2^128 - 1` is exactly `u128::MAX`.

So raising the recursion limit clears binary256 for trait resolution and the carrier
ceiling fires instead. **Neither ceiling is stated anywhere in seventy-nine files.**

**Why this is a design finding rather than a probe detail, and the argument does not depend
on whether op wants binary256.** `arvo-toolbox-not-policer.md:60`, ratified and cited by
this review's own facade fork one stretch ago (`78:718`), reads: "No bit-width cap below
the largest container the substrate is willing to dispatch through. If we dispatch up to
256 bits via multi-value containers, the meta-newtypes carry that range." A precision
ceiling at 128, imposed by the internal shape of one type-level construction rather than by
anything the consumer chose, is exactly that. And arvo's whole identity is that a precision
is a free parameter a consumer names exactly; a consumer who writes p = 200 is the
consumer arvo exists for.

**And the fix is not a bigger carrier or a raised limit.** Those treat a symptom. The cause
is that file 80 built the *definition* rather than a closed form of it, and `2^P - 1` is a
value that need never exist.

### 1.2 The replacement, built, verified over 1.2 million cells, and compiling at binary256

**Reasoned first, then compiled.** With `L = bitlen(A)` and `R = A - 2^(L-1)`:

```
foldexact(P, A) = P + L - 1 + bit
    bit = 0                   if R = 0            (A a power of two)
    bit = 1                   if R >= 1 and P >= L
    bit = [ (R << P) >= A ]   if R >= 1 and P <  L
```

Every recursion is structural, that is logarithmic in the value rather than linear in it.
The shift branch is entered only when P < L, so its depth is bounded by `bitlen(A)` and
never by P. The largest intermediate is below `A^2`. **P appears in the answer only as a
summand**, which is what removes both ceilings at once.

**Verified in exact integer arithmetic** over P in 1..=299 by A in 1..=4099, **1,225,601
cells, zero mismatches** against `bitlen(A * (2^P - 1))` computed in arbitrary-precision
integers, including both cells where `foldnum` is loose by one and the (p=2, A=3)
tight-non-power case file 80 pinned.

**Built at the type level** (`82_probes/probe_2_foldexact_without_allones.rs`, zero feature
gates, `#![no_std]`), over the same sealed grammar, with new machinery: `CmpP` (three-way
structural comparison, with `DemoteToLt` / `PromoteToGt` at the mixed constructor pairs),
`ClearTop`, `ShlP`, and a three-level dispatch chain. One construction detail is worth
lifting because it is the carrier-at-birth rule applied to an intermediate: `ClearTop`
returns a closed two-member kind `TopZero | TopSome<R>` rather than a `Nat`, because `Z`
against `P: Pos` is not a distinction rustc's coherence can draw without negative
reasoning, and a closed vocabulary at the point of dispatch is what makes the two branches
non-overlapping.

**Checked at compile time on 56 cells over file 80's own matrix** against independent u128
ground truth, so the two constructions are compared on the same shapes rather than on a
convenient subset. Then binary128 against ground truth, and **binary256 against literals
computed offline** (`foldexact(237, 3) = 239`, `(237, 256) = 245`, `(237, 257) = 246`,
`(237, 4096) = 249`), with the width placed in type position at binary256 so the spine
rule's own requirement is met where the other construction has no expression at all.
Negative control fires with `E0080`. Priced at 0.09 to 0.20 s wall against a 0.04 s empty
baseline, comparable to file 80's 0.145 s while covering strictly more.

**The suggestion.** Adopt this form rather than file 80's. File 80's recommendation to
retire `foldnum` to prose stands and is strengthened, because the objection "the exact form
does not exist at every precision the design supports" is now answered rather than
unnoticed.

**The general lesson, which is worth more than the fold width.** The consolidation's own
staging rule says "stage a type-level reduction whose cost is superlinear in the value, do
not stage one that is logarithmic" (`78:764-766`). This stretch found the missing middle,
and it is not a staging question at all: **a type-level construction that recurses linearly
in the value of a parameter is bounded at 128 by rustc's default recursion limit and at
`Nat::VAL`'s carrier by arithmetic, and is therefore not a construction the design may
contain, at any staging.** Every other reduction in the tower (`VAL`, `Cmp`, `Gcd`,
`BitLen`, file 79's `Dec`) recurses structurally and is safe. `AllOnes` was the first that
did not, and it arrived in this stretch.

### 1.3 File 81's corrections all hold, checked independently, and two of them are sharper than it said

File 81 corrected a ratified measurement, which is exactly where a second read is owed.
Everything I checked reproduces.

**The cache figure.** Confirmed on this host this session: `hw.perflevel0.l1dcachesize =
131072` (128 KB) and `hw.perflevel0.l2cachesize = 12582912` (12 MB). File 75's "typical
32KB L1" and the consolidation's "spanning cache-resident to past-L1" (`78:561-562`) are
wrong by a factor of four on the performance cores. **One detail file 81 did not record and
the next member needs**: the un-suffixed `hw.l1dcachesize` reports **65536**, the
efficiency-core figure. So a naive `sysctl hw.l1dcachesize` gives 64 KB, still twice the
assumed 32 KB, and reading the wrong key is the likeliest route to the original error. The
key to read on a heterogeneous-core host is the `perflevel0` one.

**The CSV medians.** Recomputed from the committed artifact this session: the medians of
`algo_ns` in `bitpack-sequential-sum_n16384.csv` are 1668.1 and 7700.0 ns over 80 samples
each, which at 16384 elements is **0.1018 and 0.4700 ns per element**, against the 0.111
and 0.509 file 75's table states. File 81's 8% offset and preserved ratio (4.617 against
4.60) are exact.

**The harness cap, and it is worse than stated.** `build_input_bytes` materialises
`Self::Input` as a local before copying it (`mockspace/bench-core/src/lib.rs:148-158`,
confirmed by reading), so a flat input transits the stack. File 81 concludes this "caps a
flat input at a few megabytes". Put that beside the L2 figure above: **12 MB of L2 on this
host means no bench under this harness, on this host, can construct a working set that
leaves cache at all.** That converts file 81's owed "bandwidth-contention shape" from a
bench somebody should write into a **harness change somebody must make first** (an input
built in place or behind a reference rather than returned by value), and it explains why
the gap has never been measured rather than leaving it as an omission.

**One thing file 81 got slightly wrong in its own attack, worth noting because it is the
same class of error it was correcting.** File 80's arity-grep counts ("25 occurrences in
file 64 and 14 in file 55", 80:392-393) are 27 and 15 on my count this session. The
substance is untouched: 53 files hit `[Aa]rity`, and the artifact file 79 said it could not
find is in files 55, 62 and 64. But two files in a row have now published a count they did
not re-derive, and the discipline that catches it is cheap.

### 1.4 File 79's substance survives, its diligence claim does not, and its constructions are on the safe side of the line file 81 drew

**Confirmed independently, and it is a real defect.** File 79 states it "searched
`[Aa]rity` across every file; the hits are all fold-arity, an unrelated subject in files 18
and 19" (79:137-140). Run fresh: **53 of the panel's markdown files hit**, with 27
occurrences in file 64 and 15 in file 55. That search was not performed as described. In a
file offered as one of two independent reads on a design question, a false diligence
sentence is worse than an absent one, because the convention's whole value is that the two
reads were actually independent and actually performed.

**Its substantive answer survives, and I agree with it on my own reading.** Capacity is a
parameter of the same kind as `Precision`, not a value an operation can leave; the far-point
rule's subject is one layer down, at index arithmetic; `Capacity: Nat` with `SIZE` reading
through; the array grammar is a paired declared fact forced by the language. I re-derived
the parameter-versus-event distinction from the far-point rule's own statement (`78:275-286`)
before reading file 79's argument for it, and it is forced rather than chosen: the rule
quantifies over a value set an operation can exceed, and a capacity is what establishes such
a set rather than a member of one. File 80's re-grounding of the no-new-seal conclusion
(composite closure: `Capacity: Nat` has no generic parameter slot, so the uncovered-parameter
forgery route does not exist, 80:404-408) is the right correction and should be the sentence
that ships, not file 79's ordinal-versus-cardinal reasoning.

**And one check nobody has run, which matters because I have just found a construction that
fails it.** File 79's `Dec`/`PosPred` recurses structurally, on the constructor shape, not
on the value: `I<Q>` steps to `O<Q>` with no recursion, and `O<O<Q>>` recurses through a
carry chain whose depth is the number of trailing zero bits. So its depth is bounded by
`bitlen(N)`, not by N, and it sits on the safe side of section 1.1's ceiling. That is worth
recording explicitly rather than assuming, because the same stretch produced one
construction on each side and only one of them was tested.

### 1.5 File 80's membership theorem is right, its branch picture rests on an unstated choice, and it stopped one step short of the design consequence

**Where I agree.** Every arvo value is `m * r^q` with integer `m` and `q` and integer radix
`r >= 2`, so every arvo value set is a finite set of rationals, so no arvo numeral's finest
inhabited system is above ℚ. The sub-ℚ fragment ℕ ⊂ ℤ ⊂ ℚ is a genuine chain with canonical
embeddings. Therefore the finest inhabited system exists, is unique, and lies on that chain
for every arvo numeral, independent of every branch above. That is a clean theorem, it does
not need the whole vocabulary to be a chain, and it survives every branch the vocabulary
grows. It is a better answer than the scoping file 64 proposed, and I reached it
independently by asking what an arvo value actually is before reading how file 80 got there.

**Where the branch picture is softer than it reads.** File 80 corrects file 64 by moving
from literal containment to structural embedding, correctly, and then counts "three
mutually incomparable branch families above ℚ" (80:242-245). But the correction leaves the
*signature* of the embedding unstated, and the count depends on it. ℂ, ℍ and 𝕆 are not
ordered fields, so they cannot be compared to ℝ, *ℝ or No by ordered-field embedding at all;
under plain field embedding, on the other hand, *ℝ embeds into ℂ as readily as into No,
since every characteristic-zero field of cardinality at most the continuum does, and the
"incomparable families" picture flattens. So the three-family count is an artifact of
comparing some pairs in one signature and others in another. **This is not a defect in the
theorem**, which never touches the branches; it is a caution against shipping the branch
count as if it were established, and a note that if the vocabulary above ℚ ever needs a
comparison relation, the first thing to state is which signature it is a relation in.

**The step file 80 stopped short of, and it is a substantial design consequence.** If the
finest inhabited system is always on the sub-ℚ chain, then **seven of the ten ratified
vocabulary members are never the answer to the question the "finest inhabited" mechanism
asks.** The mechanism's output type is a three-element chain. That does not make the other
seven useless, but it does mean they are there for a different fact, and the crate's design
should say which. Two readings, offered symmetrically because I do not think the evidence
picks between them: the upper members exist as *upward closure*, answering "may this value
be used where a ℂ is expected", which is a query about a consumer's required algebra rather
than about the numeral; or they exist as the vocabulary a *future* value set (a numeral
with an imaginary or hyperreal component) would need, in which case they are anticipatory
and the finest mechanism genuinely returns from a three-element chain today. Which one is
op's call and it changes `arvo-num-systems`' shape substantially: under the first reading
the crate ships two relations, not one.

---

## 2. The assembled shape, in the consolidation's own form

Where the pieces agree, stated once. Nothing in this section is new; it is the stretch's
four deliverables composed, with the corrections above already applied.

**Capacity.** A capacity is a type-level parameter that establishes an index domain, of the
same kind as `Precision`, `Exponent` and `StoredWidth`, and not a value an operation can
leave. Its value is a direct instance of the tower's `Nat`, one seal, one ordering, one
arithmetic, inherited wholesale, with `SIZE` reading straight through; no capacity-specific
comparison and no capacity-specific `Gcd` exist, so there is no second machinery to
diverge. Its array-length grammar is a paired, declared fact checked to agree with the
value at the one construction door, forced by the language rather than chosen. No new seal
is owed, and the reason is composite rather than categorical: `Capacity: Nat` has no
generic parameter slot, so the uncovered-type-parameter forgery route the arity carrier
needed sealing against does not exist here, and the seal on `Nat` plus the orphan rule
close foreign impls together. The far-point rule fires one layer downstream, at index
arithmetic: the last valid index below a capacity is its predecessor, total over nonzero
capacities and refused at the type level over an empty one, by the same
supremum-over-the-ordered-set logic and with the same theorem-not-case shape as the
numeral rule's own NaN exclusion.

**The crossing contract.** `Crosses<N: Numeral>: Lowering` is confirmed on a second
independent read, derived rather than chosen from two already-ratified rules composed, and
the presumptive marker can drop. Three precisions before it hardens: every impl is spelled
`unsafe impl`, and what D16 splits is who discharges the proof rather than the spelling;
statement 0 quantifies over every bit pattern of `Encoding::Fields`' width, with partiality
expressed by shrinking the fields rather than by a domain side-condition; and every
consumer-site `unsafe impl Crosses` is a named entry in the trusted base, auditable as a
list.

**Operations and laws.** `roundToIntegralExact` decomposes into a value-keyed law plus a
datum-keyed exponent selection, so its value half is law-eligible. `quantize` does not
decompose: its result value reads the operand's datum, so it is pair-keyed as a whole and
can never be a law. The standard's own prose carve-out lands in arvo as a keying fact,
which is where this design wants it. Section 3.2 below adds what this costs the preset
tables.

**The fold width.** `foldexact(P, A) = bitlen(A * (2^P - 1))`, exact, with `foldnum`
retired to prose as the readable bound it is. It is expressed as a closed form whose every
recursion is structural, so it exists at every precision the design admits rather than
stopping at 128 (section 1.2).

**Bitpacking.** `Layout::Bitpacked`'s cost against `Layout::Dense` is a property of the
decoder, not of the layout. Every decoder parameter is a function of the logical width
alone (period `P = 8/gcd(W,8)`, group stride `G = W*P/8` bytes, window offsets, per-lane
shifts, mask, load width, read headroom, and the well-formedness refusal), so all of them
belong on the layout type as consts. With them there, the measured multiple falls from
4.6x to 1.50x on a plain sum and 1.29x on a per-element kernel with a gather decode. Which
decode is optimal is a joint property of the layout and the consumer's operation, so the
substrate ships both and picks on lane width rather than shipping one and calling the
other's regime the layout's price. The same period is the write granule: adjacent values
share bytes, so no element is independently writable and a partition boundary must fall on
a multiple of `P`. And the multiple is not the whole cost model, because it prices decode
in a compute-bound regime where a smaller footprint buys nothing; `Cold`'s footprint intent
is realised under bandwidth contention, which no artifact in this review measures and which
section 1.3 shows no artifact under this harness can.

**Membership.** Every arvo value set is a finite set of rationals, the ratified sub-ℚ
vocabulary is a chain with canonical embeddings, so the finest inhabited system exists, is
unique, and lies on that chain for every arvo numeral, independent of every branch above.

---

## 3. What the pieces cost each other

This is the part no single file could see. Four collisions, in descending order of how much
they change.

### 3.1 The `Lowering` charter names two width levels and the stretch's own evidence needs three

**Reasoned, from three files that each hold one third of it.**

File 73 drew the byte boundary as two maps: `embed : D -> Carrier`, where `Carrier` is a
bit pattern of exactly `StoredWidth` bits, and `materialise : Carrier -> Bytes`, "a pure
relabelling for every `Layout::Dense` numeral at any `StoredWidth`" (`78:526-536`).

File 80 built the first model in this review with padding at all: nine logical bits in a
u16, seven padding bits, and over 65,024 same-value-different-padding pairs **a compare
keyed on the raw carrier misorders every single one** while the canonical compare is Equal
(80:365-380). Its conclusion is that every eight-bit model checked the padding claims only
where they are vacuous.

Put those together and the question is: **where do those seven bits live?** Not "outside
`Encoding::Fields`' width" within a nine-bit carrier, because a nine-bit carrier has none.
They live between the carrier and the u16 the dispatch actually allocates. And the charter
has no name for that place. Its trait table (`78:629-635`) has `Encoding`, `StoredWidth`
and `Layout`, and the word "container" appears in the consolidation exactly three times,
all as a *transfer coordinate* for the verification argument (`78:128`, `78:946`,
`78:956`), never as a structural level a map crosses.

So the honest picture has **three levels, not two**:

| level | what it is | which statement governs it |
|---|---|---|
| fields width | what `Encoding::Fields` occupies | statement 0, quantifier domain per file 80 |
| `StoredWidth` | what `Lowering` declares | statement P, per file 73 |
| container width | what the dispatch allocates | **nothing** |

and **three maps, not two**: `embed : D -> Carrier` (padding law, forced canonicalisation);
an unnamed `Carrier -> Container`, which is where file 80's seven bits and its misordering
matrix live and which owns no law; and `materialise`, whose domain is the **container**
rather than the carrier.

**Two things fall out, and one of them is a simplification rather than a cost.**

First, `materialise`'s "pure relabelling" claim becomes *unconditionally* true for
`Layout::Dense` once its domain is named correctly, instead of quietly assuming that
`StoredWidth` is a multiple of eight. A nine-bit carrier does not lay onto octets; a u16
container does. The conditionality file 73 did not notice moves into the newly named middle
map, where it belongs and where it can be given statement P's twin.

Second, and this is the question I think is genuinely open rather than merely unstated:
**does `StoredWidth` denote the fields' extent or the container's?** The ratified tables say
`Hot` and `Cold` take `StoredWidth` "minimum" (`78:409-414`, `78:435-441`), and minimum of
which is not said. If `StoredWidth` is the container width (16 for a 13-bit value), the
padding is inside the carrier and statement P reaches it. If it is the fields' extent (13),
statement P is vacuous on `Hot` and `Cold` and the misordering file 80 measured is
ungoverned. **The same word is currently carrying both readings, and which one it carries
decides whether the design's padding guarantee has content on two of its four presets.**

**And file 81 supplies the third case, which resolves the shape rather than complicating
it.** Under `Layout::Bitpacked` in a column there is no per-value container at all; file 73
already said the per-value `materialise` does not exist, and file 81 computed exactly what
does: a group of `P = 8/gcd(W,8)` elements in `G = W*P/8` whole bytes. **The bitpacked group
is the container**, and it is the smallest unit for which the middle map is a relabelling.
So `Layout` is not merely a packing choice; it is the axis that decides what the container
level *is*, and the two `Layout` members give the same three-level picture with the
container at two different granularities. Stating it that way makes file 81's period and
file 73's map structure one mechanism instead of two adjacent findings.

*Grounded on: settled shapes (`78:526-536`, `78:629-635`, `78:409-441`), compiled by others
(`80_probes/probe_3`, `73_probes/probe_2`), measured by others (`mock/benches/bitpack-*`),
reasoned (the three-level statement, the `materialise` domain correction, the group-as-
container identification, all mine and all offered as suggestions).*

### 3.2 `quantize` is the first operation whose failure is not a range event, and three of four presets have no way to say so

**Reasoned, from file 80 against the ratified tables.**

File 80 measured `quantize`'s refusal density at 35.5% of operand pairs at its model and
concluded: "a `NoSpecials` numeral offering `quantize` must route that branch through the
same `Refuse`/grade machinery as every other range event" (80:133-135).

That sentence assumes a machinery the ratified tables deny to three of the four presets.
`Refuse` appears in the `OverRange`/`UnderRange` row for `Precise` and for no other preset,
in either table (`78:412`, `78:438`). More fundamentally, **`quantize`'s failure is not a
range event**: the value is in range, and what cannot be done is represent it at the
requested quantum within `p` digits. It is an *exactness* failure, and the preset tables
have no row for one.

Under `Warm` (round to even, clamp to the far point) the operation has no correct behaviour
available: rounding to `p` digits changes the value, which is precisely what `quantize` is
defined not to do, and clamping to the far point is meaningless for an in-range operand.

I do not think this reopens the preset tables and I am not proposing that. Three
resolutions, stated symmetrically because the evidence does not pick between them:

- **`quantize` is offered only where a refusal exists**, that is on `Precise`. Cheapest,
  and it makes a decimal-conformance operation preset-conditional, which may be exactly
  wrong for a design whose standard is representability of IEEE 754.
- **The exactness failure is a grade generator**, alongside the overflow grade and the
  far-point kind. This is the most design-consonant reading: the design already asks
  whether the far-point kind is "a parameter of the existing overflow grade generator or a
  sixth generator" (`78:314-316`), and an exactness event is the same shape of question one
  step over. It also keeps `quantize` total on every preset, delivering a value plus a
  grade rather than a refusal, which matches what every other non-`Precise` preset does
  with a bad event.
- **The presets gain a second resolution axis** for exactness events distinct from range
  events. Most expressive, most expensive, and the one I would want a positive reason for
  rather than the absence of a negative one.

**What I would say for the spec either way**, because it holds under all three: the preset
tables' `OverRange`/`UnderRange` rows govern *range* events, and the design should say so in
those words, so that the next operation whose failure is not a range event is recognised as
needing an answer rather than silently assigned one.

*Grounded on: compiled by others (`80_probes/probe_2`, exhaustive at the stated model),
physical (IEEE 754-2019 clause 5.2, position-cited at `62_probes/primary_sources.md:47-56`,
read this session), ratified (`78:409-441`, both preset tables), reasoned (the exactness-
versus-range distinction and the three resolutions, mine, offered as suggestions).*

### 3.3 The const-position rule is right, and its blanket form condemns constructions that are fine

**Compiled.** File 81's rule reads:

> a fact the fourth rule requires to be settled at compile time has to be written in a const
> position to be settled there. An associated const on the layout type is; a `const fn`
> called from the decode is not. (81:236-238)

Taken literally that condemns file 79's `last_index` and `in_bounds`, which are `const fn`s
called from value position (`79_probes/probe_2:139-144`, `79_probes/probe_1:117-119`), and
most of the tower's projection surface with them. But those bodies are `<C as Dec>::Out::VAL`
and `i < C::SIZE`: a single read of a value the trait solver has already produced. File 81's
failing case was a body performing recursion LLVM then has to fold. Those are two different
things, and a rule that condemns the harmless shape produces noise and gets ignored, which
is how a rule stops protecting anything.

Three shapes compiled at `-O` and disassembled (`82_probes/probe_3_where_const_position_bites.rs`):

| shape | body | emitted |
|---|---|---:|
| `const fn` reading an associated const | `N::VAL` | **3 instructions**, `cmp x0, #0xd` / `cset` / `ret` |
| `const fn` computing by recursion | `8 / gcd(N::VAL, 8)` | **10 instructions**, a `udiv`/`msub`/`cbnz` back-branching loop plus a second `udiv` |
| associated const | `const PERIOD: usize` | **2 instructions**, `mov w0, #0x8` / `ret` |

That reproduces file 81's own disassembly independently, in a separate file. **And the
in-loop case is what decides how strongly the rule can be stated.** Two identical decode
loops differing only in where the period, stride, width and mask are written:
`loop_from_assoc_const` is 236 instructions with **zero** division instructions;
`loop_from_const_fn` is 238 instructions with **three** division instructions surviving at
`-O`. So the loop body folded in both cases and the const-fn version still carries division
residue the other does not.

**The statement I would suggest for the spec, which is stronger than "it fails" and stronger
than "it usually works":**

> rustc guarantees const evaluation in a const position and nowhere else. A `const fn`
> called from value position folds or does not fold at the optimiser's discretion; on this
> target it leaves division residue in both the standalone and the in-loop shape. A fact the
> pricing pillar requires settled at compile time is therefore written where the language
> guarantees it, as an associated const, a const block, or a const generic. The boundary: a
> `const fn` whose body is a *projection* (a read of a value the trait solver has already
> produced) is a load of a constant and folds reliably; a `const fn` whose body *computes*
> does not.

An intermittent guarantee is not a guarantee, and the fourth design rule asks for a settled
fact rather than a usually-settled one. This is also the one clause in this whole stretch a
lint could enforce mechanically, which is worth something on its own.

*Grounded on: compiled (`82_probes/probe_3`, disassembly and counts in
`82_probes/OUTCOMES.md`), settled shapes (`81:220-239`, `79_probes/probe_1:117`,
`79_probes/probe_2:139`), reasoned (the boundary sentence, mine).*

### 3.4 The fourth rule generalises, and the generalisation has a testable shape rather than being a mood

The brief asks whether file 81's finding is specific to packing. **Reasoned**, and I think
it is not, but the useful form is narrower than "ask it of every mechanism".

File 81's decoder failed for a specific reason: a quantity that is a function of type
parameters alone was recomputed from a *running index* inside a per-element loop, because
the running index was the seductive source and nothing in the program stated the period.
That is the shape, and it gives a test sharper than the pillar's own general clause:

> **Is any quantity computed inside a per-element or per-step loop a function of the type's
> parameters alone?** If yes, it belongs on the type as a const, and the loop should be able
> to unroll against a literal.

Applied across the design as it now stands, four sites and one non-site:

- **The bitpacked decode plan.** Found and fixed (file 81). 4.6x to 1.50x.
- **The bitpacked encode plan.** Not measured by anyone, and it needs no new work: the
  write path reads the same period, stride, window offsets, lane shifts and mask, plus a
  clear-mask `!(mask << shift)` which is the same const negated. The finding transfers for
  free, and the design text should publish the plan once for both directions rather than
  letting the write path rediscover it.
- **The far point.** `Warm` and `Cold` clamp to it on every out-of-range event
  (`78:438`), and it is exactly a value the type system holds: the largest finite magnitude
  is a function of the numeral's parameters alone. It must be an associated const on the
  numeral, not a `const fn` the clamp calls. A live candidate, unchecked.
- **`embed`'s padding mask.** `(1 << fields_width) - 1`, a function of the type. Same
  treatment. A live candidate, unchecked.
- **The canonicalising projection `V -> D`.** Not a site. Decimal canonicalisation removes
  trailing zeros from a *datum*, which is genuinely data-dependent, and no type-level fact
  determines it. Naming a non-site matters, because a rule that fires everywhere fires
  nowhere.

And one candidate I checked and cleared rather than assumed: file 79's `last_index` and
`in_bounds` are projections, not computations, and section 3.3 places them on the safe side.

*Grounded on: measured by others (`mock/benches/bitpack-decoder-shape_*`), compiled
(`82_probes/probe_3`), settled shapes (`78:152-166` the pillar, `78:438` the clamp row),
reasoned (the test's phrasing and the five-site sweep, mine, and the two live candidates are
flagged rather than checked).*

---

## 4. The owed list: one stale instance, and no mechanism protecting the other nine

The brief asks whether file 80's staleness finding is one instance or a pattern. I checked
rather than assumed, and the answer is more useful than either.

**It is one instance, and I verified both halves of it.** `62_probes/primary_sources.md`
reads the OFP8 document at position under its own heading (`62_probes/primary_sources.md:6`),
and the E4M3 check `68b:36-37` marked pending was performed there before op wrote that line.
Two consolidations then carried it as owed.

**But it is not one instance of a defect with nine clean neighbours; it is one instance of a
defect with nine unprotected neighbours.** I walked all ten items in `78` section 4 and asked
of each whether it names the artifact whose existence would close it. **Zero of ten do.** So
the only reason the other nine were correctly listed is that nobody had performed them,
which is luck rather than a mechanism. The base rate of the defect is the base rate at which
work gets done and forgotten, and nothing in the review measures that.

I also checked one item that reads like a second instance and is not, because a wrong
finding here would be worse than none. `78:936` lists "the IEEE 754-2019 §5.12
inexact-conversion-signalling citation" as owed. `62_probes/primary_sources.md:40` carries a
verbatim read of clause **5.2**, decimal exponent calculation, which is a different clause;
§5.12 governs conversion between floating-point data and character sequences, which is file
72's parse chapter. Genuinely owed. Similarly the OCP mode-split facts were genuinely absent
from `62_probes` and file 80 sourced them fresh.

**The risk is at its highest right now, and that is the actionable part.** File 80 closed
five owed items in one file: the `Crosses` second read, statement 0 against both operations,
the E4M3 primary source, the overflow tie, the OCP mode split, and the nine-bit companion.
File 81 closed the `Layout::Bitpacked` follow-up. If the ninth consolidation absorbs their
headlines but not their item-by-item closures, **seven items ride at once**, which is seven
times the defect that took two stretches to notice at one.

**The suggestion, which is file 80's with one addition.** An owed item names the artifact
whose existence would close it, so closing is a grep rather than a memory. The addition:
**a consolidation's own table-diff obligation gains one row, checking the open list against
the artifact set**, because the obligation as written checks tables against sources
(`78:87-97`) and the open list is the one table in the document that was never in scope.

*Grounded on: settled shapes (`78` section 4 in full, `62_probes/primary_sources.md` read
this session), reasoned (the zero-of-ten count and the spike argument, mine).*

---

## 5. Three process defects, one of them new and worse than the one it compounds

### 5.1 File 81's own numbers are not reproducible from the committed tree

**Verified this session.** File 81's commit `3c873b1` committed ten new bench-variant crates
with their sources, the CSVs, the meta files and the findings reports. It did not commit the
`mock/Cargo.toml` edit that makes those ten crates workspace members, and it did not commit
the lock. Both sit modified in my working tree:

```
$ git show HEAD:mock/Cargo.toml | grep -c "bitpack-plan\|bitpack-mac"
0
$ git show HEAD:mock/Cargo.lock | grep -c "bitpack-plan\|bitpack-mac"
0
```

So at HEAD there are ten crate directories no manifest references. My test gate reports 666
only because my working tree carries the uncommitted manifest; **at HEAD it would report
661, and a fresh clone cannot build or re-run the benches this stretch's headline finding
rests on.** That is strictly worse than file 80's lock note (80:419-421), which it compounds:
that was a lock regeneration gap, this is a manifest gap, and it makes the measurement
unreproducible from the record rather than merely awkward. One commit closes it, and it
should be made before the ninth consolidation cites the numbers.

### 5.2 The bench orchestrator has no per-section filter and overwrites committed artifacts

File 81 recorded this, worked around it by comparing then restoring with `git checkout`, and
noted that anyone re-running pays the same tax (81:38-44). I did not re-run the harness and
so did not pay it. Carrying it forward because it is a real hazard: an artifact trail whose
only protection is that each member remembers to restore it is one forgetful member away
from losing a stretch of committed measurements, and the restore has to happen *after* a run
that already overwrote them.

### 5.3 Two files in a row published a count they did not re-derive

Section 1.3's last paragraph. Minor individually; worth naming because the review's own
table-diff obligation exists for exactly this and does not reach a member file's inline
counts.

---

## 6. What a consolidation could take, close to verbatim

*The `Lowering` charter names three width levels, not two. The fields width is what
`Encoding::Fields` occupies and is statement 0's quantifier domain. `StoredWidth` is what
`Lowering` declares and is statement P's domain. The container width is what the dispatch
allocates, and the design owes it a name and a law, because the padding between the carrier
and the container is where a raw-carrier compare misorders every same-value pair (65,024 of
them at the first model built with padding at all), and neither existing statement reaches
it. Three maps follow rather than two: `embed : D -> Carrier` carries the padding law and
its forced canonicalisation; an unnamed `Carrier -> Container` owns the second padding
region and needs statement P's twin; and `materialise`'s domain is the container rather than
the carrier, which makes its "pure relabelling for every `Layout::Dense` numeral" claim
unconditionally true instead of quietly assuming `StoredWidth` is a multiple of eight.
Whether `StoredWidth` denotes the fields' extent or the container's is currently carried
both ways by one word, and which it carries decides whether the padding guarantee has any
content on `Hot` and `Cold`. `Layout` is the axis that decides what the container level is:
under `Dense` it is the dispatched primitive, and under `Bitpacked` in a column there is no
per-value container, the group of `P = 8/gcd(W,8)` elements in `G = W*P/8` whole bytes is
the container, and that is the same `P` that is the unroll factor and the write granule.*

*The preset tables' `OverRange`/`UnderRange` rows govern range events. `quantize` is the
first operation in the catalogue whose failure is not one: the operand is in range and what
cannot be done is represent it at the requested quantum, at a density of 35.5% of operand
pairs at a real decimal model. Three of the four presets carry no refusal, so the operation
has no stated behaviour on them, and the design should say which of three it takes (offer
`quantize` only where a refusal exists, make the exactness failure a grade generator beside
the overflow grade, or give the presets a second resolution axis for exactness events).
Whichever it takes, the tables should name their own scope, so the next operation whose
failure is not a range event is recognised as needing an answer rather than silently
assigned one.*

*The exact fold width is `foldexact(P, A) = bitlen(A * (2^P - 1))`, and it is expressed as
a closed form rather than as its definition: with `L = bitlen(A)` and `R = A - 2^(L-1)`,
`foldexact = P + L - 1 + bit` where the bit is zero when `R = 0`, one when `P >= L`, and
`(R << P) >= A` otherwise. Every recursion is structural, the shift branch's depth is
bounded by `bitlen(A)` and never by P, and P appears in the answer only as a summand.
Verified over 1,225,601 cells in exact arithmetic and compiled at the type level with zero
feature gates at every IEEE binary interchange precision including binary256. The form that
computes `2^P - 1` as a type does not exist above p = 128, refusing with E0275 at rustc's
default recursion limit and with E0080 at `Nat::VAL`'s carrier once that limit is raised, so
it caps precision at 128 by internal construction rather than by consumer choice, which
`arvo-toolbox-not-policer.md:60` forbids by name. The general rule: a type-level construction
that recurses linearly in the value of a parameter is not a construction this design may
contain at any staging, and every other reduction in the tower (`VAL`, `Cmp`, `Gcd`,
`BitLen`, `Dec`) recurses structurally and is safe.*

*rustc guarantees const evaluation in a const position and nowhere else. A fact the pricing
pillar requires settled at compile time is written as an associated const, a const block or
a const generic; a `const fn` called from value position folds at the optimiser's
discretion, and on aarch64 at `-O` it leaves a hardware division loop standing in the
standalone shape and three division instructions in the in-loop shape, against zero for the
associated const. The boundary: a `const fn` whose body is a projection, a read of a value
the trait solver has already produced, is a load of a constant and folds reliably; a `const
fn` whose body computes does not. The standing test the pillar gains: is any quantity
computed inside a per-element or per-step loop a function of the type's parameters alone? If
so it belongs on the type as a const, and the loop should unroll against a literal. Known
sites: the bitpacked decode plan (found, 4.6x to 1.50x), the bitpacked encode plan (the same
consts, unmeasured, transferring for free), the far point that `Warm` and `Cold` clamp to,
and `embed`'s padding mask. A known non-site: the canonicalising projection `V -> D`, whose
trailing-zero removal is genuinely data-dependent.*

*Every arvo value is `m * r^q` over an integer radix, so every arvo value set is a finite
set of rationals and the finest inhabited system exists, is unique, and lies on the sub-ℚ
chain for every arvo numeral, independent of every branch above. The consequence the crate's
shape depends on: seven of the ten ratified vocabulary members are therefore never the
answer the finest-inhabited mechanism returns, so the mechanism's output is a three-element
chain and the upper seven exist for a second fact the design has not named. If that fact is
upward closure ("may this value be used where a ℂ is expected") the crate ships two
relations rather than one. Any comparison relation among the members above ℚ must state its
signature first, because ordered-field embedding and field embedding give different answers
and the two have been mixed.*

*An owed item names the artifact whose existence would close it, so closing is a grep rather
than a memory, and the consolidation's table-diff obligation gains one row checking the open
list against the artifact set.*

---

## 7. What this leaves open, named rather than performed

- **The `StoredWidth` reading** (section 3.1). Fields' extent or container width. This is a
  question for op, it is one word, and it decides whether statement P has content on two of
  four presets.
- **The far point and the padding mask as associated consts** (section 3.4). Flagged as live
  candidates for the const-position rule; I did not check either, and both are cheap.
- **`quantize`'s resolution** (section 3.2). Three shapes offered, none preferred.
- **The membership vocabulary's second fact** (section 1.5). What the seven members above ℚ
  are for, if the finest mechanism never returns one.
- **The bitpacked write path, measured.** File 81 named it owed and stated the structure;
  section 3.4 argues the const plan transfers for free, which is a prediction rather than a
  measurement.
- **The harness input path** (section 1.3). The bandwidth-contention shape cannot be built
  until `build_input_bytes` stops materialising the input on the stack, and on this host with
  12 MB of L2 that is a hard blocker rather than a convenience.
- **The manifest and lock commit** (section 5.1). One commit, and it should land before the
  ninth consolidation cites file 81's numbers.
- **My own replacement fold construction has not been read by anyone.** It is compiled,
  ground-truthed on 1.2 million cells offline and 60 cells in the type system, and it is one
  pass. The `CmpP` refinements at the mixed constructor pairs are the part I would look at
  hardest: they are correct because the encoding is canonical, and if the tower's `Pos` ever
  admits a non-canonical form they stop being correct.

Only op's calls are final, and even those go stale. Everything above is offered as
suggestion and evidence, not as a ruling.

*Grounded on: ratified (`77b` in full, `78:152-166` the pricing pillar, `78:409-441` the
preset tables, `78:275-286` the far-point rule, `arvo-toolbox-not-policer.md:60`,
`arvo-compile-time-last.md`), settled shapes (`78:526-536`, `79` sections 2 to 6, `80`
sections 1 to 6, `81` sections 1 to 6), compiled (`82_probes/probe_1`, `probe_2`, `probe_3`,
all run fresh this session, outcomes and disassembly in `82_probes/OUTCOMES.md`), measured
(`sysctl` on this host; the medians recomputed from `mock/benches/bitpack-sequential-sum_
n16384.csv`; the git state of `3c873b1`), tree-fact (`arvo-tensor/src/capacity.rs:48`,
`arvo-tensor/tests/capacity.rs:14-18`, `arvo-strategy/src/lib.rs:104-107`, existence and
shape only, no conclusion resting on any of them), reasoned (sections 3.1, 3.2, 3.4, the
three-level statement, the owed-list spike argument, and every suggestion in section 6).*
