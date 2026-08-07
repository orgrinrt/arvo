# What is settled

**Purpose.** One place that says what this panel has actually decided, so the state of the work is
visible without reading 150 files. It exists because the settlements were real but scattered across
fifteen separate checkpoints, and a reader tracking the panel through its reports saw a stream of
live disputes over a body of agreements they were never shown.

**How to read a row.** Provenance decides, never recency or confidence.

- **RATIFIED** means the lead designer ruled it. His words are quoted. This governs.
- **TWO EXPERTS** means two experts agreed, each having derived its own answer before reading the
  other. Cumulative agreement, where one read the other first, does not count and is recorded as one.
- **ONE EXPERT** means asserted once and never contested. Weakest rung, and a candidate for a second
  read rather than a settled call.

**How this file is maintained.** Appended the moment a checkpoint lands, before anything else. A
settlement that is not written here within the same act as its checkpoint is a settlement the next
reader will lose, which is the failure this file was created to end.

**Status.** This is the index, seeded from the checkpoints of the 130 to 149 stretch. Four thematic
sweeps of the full panel history are in flight and merge into the sections below, each also carrying
a casualty list of claims that looked settled and did not survive. Until they land, the earlier
stretch is under-represented here, and its absence from this file is not evidence that it settled
nothing.

## The acceptance criterion

The one statement everything else answers to. Ratified at `135b:12-16`:

> There *is* a way to express usage through bits and bytes *and* have the typestate derive the
> matching container and numeral representations, then validate, and erase on lowering to be exactly
> what you describe before that caveat.
>
> Anything less than that, no caveats left, is unacceptable for this design and canon.

Four parts, all of which must hold at once: the consumer expresses usage in bits and bytes; the
typestate derives the container and representation; it validates; it erases on lowering. **RATIFIED.**

A later checkpoint records the gate as met, and how it was met as itself load-bearing (`137b:10`).

## The strategy and profile axis

| Claim | Where | Provenance |
|---|---|---|
| Everything varies granularly, not only `Warm`, and a constant is a function rather than the alternative to one. Declared canon outright: "I call this as intent, settled canon, right now." | `143b:10-12` | RATIFIED |
| `Warm` is what Rust does, including across profiles. Restated after the panel kept reopening it: "Intent holds. Whatever feels intuitive and is how rust behaves. That's warm." | `142b:12-13`, first at `140b:10` | RATIFIED, twice |
| The strategy cells are functions of the profile, which the panel had lost. | `142c:57` | RATIFIED |
| `Warm`'s crossover at 65 bits is wrong, with a precise reason. | `137b:55` | RATIFIED |
| The existing preset tables are one arm, plausibly the debug-assertions arm, and were always incomplete. Nothing is owed a correction pass on that account. | `143b:89` | RATIFIED |
| arvo and notko concepts do not correspond. They have synergy and no continuity, so a notko profile tier is not an arvo strategy. | `144b` | RATIFIED |

## Container derivation and erasure

| Claim | Where | Provenance |
|---|---|---|
| The container is never written by a consumer. | `130b:37` | RATIFIED |
| The aliases are aliases, which the panel kept slipping on. | `138b:9` | RATIFIED |
| The stored width derives, and the overshoot is not a limitation to accept. | `138b:46` | RATIFIED |
| The wide payload is a strategy consequence rather than a separate mechanism. | `137b:47` | RATIFIED |
| No enumerations. Recorded as the same finding arriving a seventh time. | `137b:28` | RATIFIED |
| The post-monomorphisation hole, ruled earlier and still standing. | `130b:82` | RATIFIED |

## The public surface

| Claim | Where | Provenance |
|---|---|---|
| Canonicity is withdrawn as a requirement. | `130b:11` | RATIFIED |
| The bridge is consumer-extensible, so the cap was never forced. | `134c` | ONE EXPERT, compiled |
| The alias-reach question is closed and out of scope. | `144c:16` | RATIFIED |
| Both diagnostics ship, belts and suspenders. | `130b:69` | RATIFIED |
| The coherence overlap between a by-reference `From` and `core`'s reflexive impl fails at the head constructor, above where substitution happens, so it is structurally impossible rather than untriggered. | `146`, `148` | TWO EXPERTS |

## Laws, algebra and conversion

| Claim | Where | Provenance |
|---|---|---|
| Inclusion between numerals needs the grid, phase and both endpoint conditions. The two-condition form is not merely incomplete, it is unsound, admitting conversions that lose values. | `146`, `148` | TWO EXPERTS |
| The antichain result holds as a cardinality argument, for every bias, adjustment, radix and sign domain, and needs no sweep. It is a stronger justification for the canonicity withdrawal than the arguments that were available when that withdrawal was made. | `146` | ONE EXPERT |
| The narrowing schema does not say which of a conversion's two strategies adjudicates. The claim that no new key was needed does not hold. | `146`, `148` | TWO EXPERTS, negative only |
| The sign domain is not a partition of the order. | `146`, `148` | TWO EXPERTS, refutation only |
| The structure question is a closure question about the shape space, not a question about the order. The ambient order on finite rational sets is a complete lattice for free; the numerals are a subset, and restriction does not preserve that. An operation can only fail by naming a shape the design does not admit. | `150` | ONE EXPERT |
| The deciding ingredient is whether refinement and reach move together. Within one radix, adding a digit refines the grid and multiplies the count, so the least cover is forced. Across radices it does not, and the finer cover can be the shorter one. | `150` | ONE EXPERT |
| Within one radix, zero bias, and a closed shape space, both operations are total and the meet is exact. | `145`, `150` | TWO EXPERTS |

### A correction to `149`

`149` recorded `146` and `148` as exact inverses of each other. **They are not.** `150` establishes
that they agree on the meet, and that `148` supplied the reconciliation itself at `148:396`. The
dispute was narrower than the checkpoint stated, and the checkpoint's framing of it as a
three-way standoff overstated the disagreement. `149` stands as written, as the audit trail; this
row is the correction.

## Open, and named as open

Listed so that absence from the tables above is never read as an oversight.

- **Is the numeral space one family, or several?** This is now the question, and it is op's. `150`
  shows every structural failure in the record reduces to this one tie, so a single sentence
  settles all of them. Both answers are workable and nothing mathematical decides it. If one
  family: does a zero-width numeral exist, and negative integer width? If several: does the tie
  break toward more refinement or more reach?
- **A discrepancy in the record.** `148` reports 81 decided join failures in the unbiased radix-two
  slice; two independent instruments in `150` find zero there, and `150` says a third instrument is
  owed. Either `148`'s slice admits shapes `150`'s reconstruction does not, or the two families
  differ. Unresolved, and it will poison a consolidation that quotes either number.
- **What the sign domain is**, as opposed to what it is not.
- **The remedy for the adjudicating-strategy gap**, where the two experts who found it differ.
- **`Precise` on `inexact`**, open since `145`.
- **The `Ranged` model**, disputed between `146` and `148`.
- **Whether `Embed` ships beside `From`**, or the `From` alone.
