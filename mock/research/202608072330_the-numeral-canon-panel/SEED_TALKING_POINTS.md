# Seed Talking Points: an Archaeology of the Closed Formalization Panel

**Date:** 2026-08-09
**Source:** `mock/research/202607301300_formalization-spec-panel/`, the closed, archived,
non-referenceable predecessor panel. 203 markdown files, 117 probe directories, roughly 1.18 million
words of markdown.
**Deliverable status:** a working surface beside `OPTIONS.md` and `DROPLIST.md`. It has no authority of
any kind.

## 0. What this file is, and the one rule that governs reading it

**Nothing in the source archive is evidence, and nothing below may bolster any claim.** The archive is
agent work produced under a process that has since been replaced, and op has said its early
ratifications locked the work into a mess. Every entry below is a claim that panel made, recorded here
so it can be **put back on the table and tested again under the current discipline**. An entry records
what the claim rested on and where the instrument sits on disk, because a re-test is cheap when the
original instrument survives; it does not record the claim as established. A claim that panel called
proven and a claim it called speculative stand at the same rung here, which is no rung at all.

Three narrow exceptions produce material that IS current evidence, and they are named as such where
they occur rather than mixed in:

1. **Re-runs performed for this file.** Two probes were recompiled on 2026-08-09 under the pinned
   toolchain (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), with sources and logs committed at
   `SEED_TALKING_POINTS_probes/`. Those two results are current. Everything else is not.
2. **Committed harness output in `mock/benches/`.** Per `RULES.md`, that directory is the one thing in
   this workspace that can price anything, and several bench families the old panel's members built
   (`warm-container-*`, `warm-clamp-*`, `bitpack-*`, `bitpack-footprint-*`, with their CSV, meta and
   findings trails and `.bench_history`) are committed there. The **artifacts** are citable as current
   evidence. The panel files that discuss them are not.
3. **Op's own text.** His words do not expire with the panel that transcribed them, but under `I12`
   nothing he said before experts converged is more than an ack, and op has explicitly demoted the old
   panel's ratification rung. Op material below is flagged for **him to see again**, never cited as
   binding.

**This file contains no suggestions and no proposals.** Where an old claim maps onto a live register
question, the entry says which of four relations holds: it **duplicates** a live option, it
**sharpens** one (adds a distinction, an instrument, or a cost the register entry lacks), it
**revives** something adjacent to the droplist, or it is **new** (no live question covers it). Those
marks are filing, not weighting.

**Entry format.** Each entry carries: the claim as that panel stated it; what it rested on, with the
probe or bench path where one survives; the status that panel assigned it (recorded because it tells a
re-tester how much work to expect, not because it transfers); the register mark; and, where useful,
what would settle it now.

**Citation discipline.** Every `file:line` below was opened during this pass. Where an op quote was
read through the archive's final consolidation (`124_consolidation_twelve.md`, cited as `124:<line>`)
rather than at its source checkpoint, the citation names the place actually read and the source it
quotes. Two op files (`13c`) and the decision-register talk file were opened at source to verify the
headline quotes. Paths of the form `NN_probes/...` are relative to the archive directory.

---

## 1. Op material the current panel's `INTENTS.md` does not carry

`INTENTS.md` holds I1 through I12 and closes with: "Anything op has said that is not quoted above is
missing rather than excluded, and the remedy is to add it." The archive holds twenty-three files of
op's own words plus his numbered decision register, and the sweep at `112_the_op_material_sweep.md`
built the definitive roster. The entries below are op sentences, in his own words, that appear nowhere
in `INTENTS.md`. Each is a talking point of the highest order: not because the old panel carried it,
but because op said it and the current panel's binding file does not know it. **Whether any of these
still reflect his intent is exactly what putting them in front of him again would establish.** Several
may be things he now considers part of the mess; the flag is for him, not for the canon.

### 1.1 The standard: optimal, representative of the mathematics, capable of representing the established systems

Op, verified at source (`13c_op_the_standard_and_the_mode.md:12-14`):

> This isn't my call. Already the instruction is clear: Optimal, ideal, representative of the math,
> and also, the principle that arvo has to be able to represent mathlab, ieee standard 754, systemc
> etc etc, which means the abstractions are what truly matter, the typestate

Read out there into three parts: optimal and ideal, not adequate; representative of the mathematics'
own structure; and **capable of representing MATLAB, IEEE 754, SystemC and the rest, as a test rather
than an inspiration**, with an abstraction that cannot express one of them being a defect rather than a
scope boundary. And the consequence: the abstractions are what matter, the typestate; packaging
follows.

No sentence of this is in `INTENTS.md`. The current panel's acceptance criterion (Q1) is a different op
sentence about a different thing (the erasure gate). This one is the *selection* standard, and the old
panel treated it as the answer to every "which design does op prefer" question. **Register mark: new.**
What would settle it: op restating or retiring it for the current canon.

### 1.2 D67: conventions ship as opt-in alias sets, and their expressibility is the adequacy test

Op's decision register, verified at source
(`mock/design_rounds/202608082157/202607301100_topic.the-formalization-talk.md:1454-1470`):

> **D67. arvo writes the abstraction; every established convention ships as an optional feature
> defining that convention's vocabulary as aliases over it.** ... `conv-ieee754`, `conv-systemc`,
> `conv-matlab`, `conv-amd-vitis`, `conv-flocq` and whatever else earns one, each off by default, each
> containing type aliases and nothing else.

And its second half: **if a convention's mode cannot be written as an alias over arvo's abstraction,
the abstraction is not general enough**, a falsifiable test runnable per axis before anything ships.
This is the concrete, shippable form of 1.1's third clause. **Register mark: new.** Nothing in
`OPTIONS.md` carries a conventions-as-aliases question or the adequacy test.

### 1.3 The stopping condition and the fresh-read instrument

Op, verified at source (`13c:38-42`):

> Don't poll this. I will literally say when we are done. ... we do this until our very design is both
> concrete, valid and critically, ideal, optimal, the dream achieved, nothing less will we stop for.

Read out into a four-step cycle whose third step is **a fresh read: a member given only the
consolidation, with the transcripts withheld**, so the next area is chosen by someone not carrying the
last one's assumptions. The old panel measured this instrument's track record: run twice, and both
times it produced the widest finding of its stretch (`124:340-349`, recording `12b:18-21` and the
files-115-and-116 cold reads that found the standing base could not be implemented from). The current
panel's `RULES.md` carries curated reading and cold derivation but **not the transcripts-withheld fresh
read as a scheduled step of each unit**, and not the stopping condition. **Register mark: new** (as
process material for op rather than canon content).

### 1.4 What happens after the canon: the four-phase sequence, and where the verification mandate binds

Op, twice, three checkpoints apart (quoted at `124:366-379` from `68b:14-21` and `79b:64-69`): settle
the canon in full; a design round creating the taxonomy and its documents; source stubs; then design
rounds implementing the canon piece by piece into the stubs. The verification mandate (differential
parity suites against namesake implementations; exhaustiveness in both directions, what should hold
must pass and what should be unrepresentable must fail to compile; **red as the starting state**)
binds the last two phases, "recorded now so it is not rediscovered late or watered down when the
volume becomes apparent". **Register mark: new.** The current panel has no recorded statement of what
follows its own consolidations.

### 1.5 The end state, the consumer-facing half of the bar

Op (quoted at `124:386-389` from `70b:52-57`): alternating settle-and-explore "until a full spec
emerges that is proven, valid, and importantly **efficient and ergonomic**", in his words **invisible
for the most part to downstream consumers while doing real work underneath and lowering transparently
to optimal instructions**. The old panel read 1.1 and this as the two halves of one standard:
abstraction-facing and consumer-facing. `INTENTS.md` carries neither half. **Register mark: new**, and
it bears on Q9's ergonomics-bar entries, which currently rest on the seed's `142c` quotes alone.

### 1.6 The pricing pillar, in op's own words

Op (quoted at `124:479-482` from `78:155-158`):

> Compile time is nothing. That can be literal minutes for all we care... the important measurement is
> the actual runtime and lowered code... We *want* long compile times, if it resolves to snappy optimal
> runtime with the extra soundness, safety and numeric machinery amortized fully at compile.

Plus the sharper clause the old panel derived beside it: a strategy marker changes what happens at
runtime, never how much is amortised at compile time. The workspace rule `arvo-compile-time-last.md`
carries the same posture, but the op-voiced form and the strategy clause are in no current-panel file.
**Register mark: sharpens** Q13/Q14 (what a build arm may move, at what exchange rate), which
currently carry no cost-side principle from op at all.

### 1.7 D54: the axis-sorting test

Op, verified at source (`talk:352-356`):

> The test that separates the two columns, stated so later additions sort themselves: change the axis
> and ask whether the set of representable values changed. If it did, the axis is identity. If the
> same values are still representable and only the arithmetic differs, it is policy.

One sentence, and the old panel used it as the decision procedure for a dozen placement questions
(flush-to-zero, delivery of a refusal, the identity/policy split itself). **Register mark: sharpens**
Q5 (one axis or two) and the what-a-strategy-is definition, both of which currently reason about axes
without a sorting test. Also directly relevant to Q18 (adaptation in identity or realisation), where
the current panel's three positions could each be run through this test.

### 1.8 D56: no gratuitous abbreviation, with op's own applied table

Verified at source (`talk:399-418`): full, legible, recognisable words for every member; abbreviation
only where it is the stable form the field already recognises; applied by op to six names in the same
call (`Under` to `Underflow`, `fexp` to `canonical_exponent`, and four more, each with the reason).
**Register mark: new.** The current panel mints vocabulary continuously ("format", "adaptation",
"absorption" are working names per `63` section 6) and has no naming rule from op on record.

### 1.9 D53, D52, D48, D31: one numeric type, public compositions, and the fixed literal surface

Four register decisions the old panel treated as the surface's spine (read at `124:3358-3376` quoting
`talk:326-332` and `inherited:2110-2114`): **there is one numeric type and every shipped family is a
semantic alias over a composition of it** (D53); **compositions are public and bindable by anyone**,
presets being the default documented path rather than the only one (D52, citing the toolbox rule as
independently deciding it); the three-parameter literal surface `UFixed<I, F, S>` is fixed (D48), and
width stays a public const parameter (D31). The current panel's Q9 assumes a written const width and
its seed carries D48 by effect, but none of the four decisions is quoted in any current binding file.
**Register mark: sharpens Q9** (D48/D31 are the constraint every arrangement is measured against) and
**bears on Q2** (which coordinates a consumer writes; D48 fixes the pair unless op moves it).

### 1.10 The two standing refusals: no enumeration, and no consumer-named container

Op refused a per-width enumeration **four separate times** on the same ground (a width table, a
per-width bridge population line, a macro-call surface, a blanket-`From` dodge; recorded with quotes at
`SETTLED_surface.md:57-82`): "No enumerations, if we can help it; and I think we have much to explore
to actually be able to help it", with the panel-end state being that **no gate-free zero-enumeration
mechanism for the width-to-container derivation was ever found and ratified**, and op's last word: "the
answer still evades us, and finding it is the job, *not* settling for a solution we've already ruled
out" (`139b`, quoted at `SETTLED_container.md:153-155`). And the container ruling (`130b:41-44`,
quoted at `SETTLED_surface.md:25-29`): "Container naming is explicitly wrong. The entire idea of arvo
is that the strategy guides container selection, not the user." **Register mark: sharpens Q9**
(several of whose arms are tables of exactly the refused kind, a tension the register already notes
via `SETTLED.md:110` but without op's fourth instance or his "finding it is the job" instruction).

### 1.11 Op's numbered decision register, D1 through D75, has never been fully diffed against anything

The register lives in three topic files, op's own text, frozen at TOPIC phase, now archived at
`mock/design_rounds/202608082157/` (confirmed present on disk during this pass). The old panel's own
audit (`112:485-505`, section 6.1) measured: the two later topic files name 47 distinct D-numbers, 28
of which appeared in no consolidation by number, **and nobody ever ran the diff**; a follow-up
(`113`, summarised at `124:219-309`) found the register agreement rate at fifteen of forty-six and
found the identifier space itself defective: **two overlapping `D1` through `D4` sequences live in one
file**, plus a question grid whose rows collide with the decision prefix, so a bare "D1" silently
resolves to one of three things. The old panel adopted a round-qualified citation form,
`D<n> (from <round-id>)`, which it found to be **op's own practice in his own frozen text, three
times** (`124:277-280`). Whether any of D1 through D75 is still op's intent is his call; that the
register exists, is his text, and is uncatalogued by the current panel is a fact. **Register mark:
new**, and arguably the largest single body of unswept op material anywhere in the repository.

### 1.12 Held-open items in op's own words: the fidelity axis, the ten-axis completeness claim, and partial associativity

Three op acts recorded at `112:377-412`: he held the arithmetic-fidelity axis open as "a proposal, not
an adoption", with "It stays attackable, **and so does the claim that the ten-axis set is complete**"
(`12b:28-33`); he adopted "a fidelity grant is checked rather than asserted" while recording what that
does not settle (`17b:19-30`); and he adopted the name "partial associativity" for a law the design
does not name (`17b:40-50`), whose replacement (the nine-point view lattice) later retired the ladder
it belonged to **without any record saying what became of op's adopted name**. **Register mark: new**
for the axis-completeness hold; the partial-associativity content itself is superseded inside the
archive (see section 5).

### 1.13 The fused-versus-split call op reserved and never made

`08b:47-51` (quoted at `112:270-276`): whether `Number` is one fused strategy parameter or a split, and
whether the split is worth its cost, "**Those are op's**, and they are downstream of whether
enforcement is possible." The old panel later recorded a two-independent-reads settlement recommending
the split unchanged (`124:3650-3663`) and framed what remained as a one-line ratification, which never
came. Under the current provenance rules that settlement is unratified agent output. **Register mark:
new** (the current panel has not reached type-shape questions; when it does, this is a call op
explicitly reserved).

### 1.14 The downstream-evidence correction

Op (quoted at `124:5688-5695` from `06b:18-40`):

> Hmm. The fact existing consumers do things one way, might just be because no better existed (we know
> this, this is why we are here). Should be irrelevant, we focus on the optimal, what the consumers
> would ideally deal with and in.

With the old panel's gloss: what a consumer currently writes is evidence of what was absent when they
wrote it, not of what they need. **Register mark: new.** Bears on I11 (the algorithm crates as the
selling point) whenever downstream shapes get consulted.

### 1.15 The licence to argue, and the checkpoint cadence that drifted

`04b:72-74` (quoted at `124:359-364`): any member may argue against any ratified call, "**provided the
argument is made rather than asserted**". And `04b:42-43`: a checkpoint with op every two experts,
which the archive measured drifting to four and then to four-then-consolidate with no record of whose
drift it was (`112:347-363`). The current panel's 4-4-1 cadence is op's own later instruction, so the
cadence half is history; the licence sentence is not in any current file. **Register mark: new** (the
licence), **superseded** (the cadence).

### 1.16 Rulings already carried, listed so this section is checkable

For completeness of the op-material sweep, these archive op-statements ARE reflected in current
binding material, and need no flag: `Warm` behaves as native Rust (I3, quoting `140b`); Warm's
imitation serving intuition (I4); Hot may sacrifice soundness for proven gain (I5); Cold's two
meanings (I6, restated by op 2026-08-08); Precise across chains (I7); strategies weigh measurements
(I8); strategy decides correctness (I9); core-count neutrality (I10); library posture (I11); the
ack-versus-ratification reading (I12). The posture directives from `16b`/`16c`/`16d` (spec is the
subject, boundary obligations, spirit outranks all, keep-what-serves) survive as workspace rules
(`panels-argue-the-intent-not-the-wording.md` and kin). The arvo/notko separation (`144b`/`144c`) is
carried in workspace rule `arvo-always-optimal-internals.md`'s corrected `#[profile]` note and the
SETTLED seed. `143b`'s "a constant is a function, all things act granularly, I call this as intent,
settled canon" is quoted inside `OPTIONS.md`'s what-a-strategy-is entry, though **only by its first
clause**; whether the current panel wants the full statement re-ratified is open.

### 1.17 Op ratifications that reversed inside the archive, as calibration for the current ACK rung

Recorded because I12 descends from exactly this history, and the instances are its evidence:
**canonicity** was ratified at `127b` and withdrawn by op himself at `130b` three files later
(`SETTLED_surface.md:423-430`); the **width enumeration** was converged-then-overturned by op the same
day (`SETTLED_container.md:397-403`); the **`Warm` clamp cell** of a table ratified in full at `70b`
was declared stale by op's own restated intent at `142b` (`SETTLED_container.md:405-408`); and the
**GCE greenlight** recorded in `unstable-features.md` was later superseded by op outright. A reader
calibrating how much weight an op ack deserves before convergence has, in this archive, a measured
answer: substantial, and reversible within days when the evidence moves. **Register mark: duplicates
I12's rationale**; kept here because the instances are concrete and the current panel's files do not
enumerate them.

---

## 2. Material bearing on the two questions currently in flight with op

Both are op's and neither is answered here. The archive bears on both, which is worth flagging
precisely because whoever handles his answers should know this material exists.

### 2.1 The verb "validate"

The acceptance criterion the current panel's Q1 decomposes is the same sentence the old panel lived
under (ratified there at `135b:12-16`, quoted at `SETTLED_container.md:33-37`). Two archive facts bear
on what "validate" meant in practice there. First, the panel's own reading was operational: "it
validates (the laws and refusals actually run)" (`SETTLED_container.md:29-31`). Second, a later op
checkpoint recorded the gate as **met**, "and how it was met matters" (`137b:10-26`, read at `SETTLED_container.md:60-66`), where the
how-it-was-met was a derived property (the payload-is-one-limb condition) rather than a discipline,
which op had predicted at `135b:65-68`, per the same sweep passage. Neither fact answers which verb op now means; both are
context he may want in front of him when he answers. **No position is taken here.**

### 2.2 The long-standing constraints, and what the archive says they were for

The constraints (`no_std`, no `alloc`, const sizes, no `dyn`, no `TypeId`) appear in the archive
throughout as workspace discipline, never as op-stated intent, which matches the current `CLAUDE.md`
banner's provenance caveat. What the archive adds is a **load-bearing role** nobody in the current
panel has restated: the old panel's four-bin evidence ledger placed every model-width exhaustive check
in a bin whose transfer to real widths "rests on the forbidden-feature bans in `unstable-features.md`"
(`124:3831-3841`), and then **refuted the transfer argument's strongest form twice by compiled
counterexample with the bans in force** (`124:1160-1187`: absorption-freedom exhaustively TRUE at
exponent span `p` and FALSE at `p + 1`, same code, same bans; and the const-tag container dispatch as
a shipped, permitted third way an instantiation can observe itself, `124:644-668`). Four
`unstable-features.md` wording edits were queued for op and never landed (`124:5134-5143`), including
marking the 28.45-second const-eval figure as one machine's measurement and correcting the rule's last
sentence from a sufficient-condition claim to a necessary one. **If the constraints question reopens,
these edits and refutations are the archive's sharpest material on what the bans do and do not buy.**
The claims would need re-establishing under current discipline; the probe trails are named in the
archive's sections 1.5 and its registry section.

---

## 3. Proved or measured there, worth re-testing here

The highest-value category: claims with instruments on disk. Each entry names the instrument so the
re-test starts from the artifact rather than from nothing. **Two entries carry current evidence from
re-runs performed for this file; the rest carry none.**

### 3.1 The width surface and the container derivation

**T1. The width-to-container ladder is total and gate-free under structural keying.** Claim: keying
the magnitude as a little-endian binary type (`Term`/`D0<T>`/`D1<T>`) makes the whole width-to-
container derivation expressible with no feature gate and no ceiling, native rungs by trait case
split, wide rung by a word cons whose size falls out of construction, no width enumerated anywhere.
Instrument: `137_probes/p5_total_ladder.rs`. **Re-run 2026-08-09: compiles, exit 0, under the pin;
log at `SEED_TALKING_POINTS_probes/p5_total_ladder.rerun.log`. That specific fact is current
evidence.** The old panel's status: the outcome op-checkpointed (`137b`), the chain beneath it
cumulative (one-expert, per `SETTLED_container.md:91-100`). **Register mark: sharpens Q9**, where the
current panel's C0 through D arrangements all wrestle with the const-to-type crossing; the structural
keying is a different route than any current arm and its diagnostic cost (digit towers) is already
priced in Q9's entries.

**T2. A backing array can be derived structurally from the numeral, no const arithmetic in type
position, layout-identical to `[T; N]`.** Instrument: `76_probes/b1_structural_array.rs` (with
negative controls `b1b`, `b1c` beside it). **Re-run 2026-08-09: compiles, exit 0, under the pin; log
at `SEED_TALKING_POINTS_probes/b1_structural_array.rerun.log`. Current evidence for the compile
fact.** The archive's fuller story: re-derived independently thirty-one files later (`107`), swept at
23 numerals and 4 element types with 184 layout assertions, codegen shown identical to native arrays
(LLVM merged three of four function pairs into single symbols), and a 27x compile-time collapse found
when a structurally-recursive `filled` was rewritten as provided methods over a projected slice,
yielding the design sentence "where a type's shape is a structural recursion, every function over it
is written once against the projected view, never recurred alongside it" (`124:4134-4145`).
**Register mark: new** (capacity and storage are untouched by the current register), and the archive's
own droplist carries the twin corrections (the construction was not new, and was not quadratic).

**T3. The const-to-type bridge table is forced, and everything about its emission was worked out.**
Claims: an impl is the only case split Rust has over a const parameter, with all three escape routes
compiled shut (`124:3473-3480`); the table lives in the crate that declares the sealed carrier by the
orphan rule; it is emitted by `macro_rules!` and not a build script because arvo grows no build
harness; each row carries its own agreement assertion, because a row corrupted together with a
compensating change passes every downstream check (`124:3494-3506`). Then the late-stretch attack:
the measured table had actually been **written by a Python script**, and the `macro_rules!` version
of the same table costs **33.4 seconds against 0.93**, because `macro_rules!` cannot count
(`125_rompf...md`, opening). And the ceiling pricing: roughly quadratic past 4096 rows
(`124:5207-5215`), with a full `W_MAX` canon paragraph drafted at `124:5217-5230`. **Register mark:
sharpens Q9** across several arms at once. The 33.4-second emission defect is the single most
actionable unresolved fact in this cluster.

**T4. The const parameter is not necessary, and the table's real job is binding time.** `125`'s two
halves: a width can be a type parameter (every call site compiles, no ceiling), but the table does
not disappear when the const does, because the table is where decimal notation converts to a numeral
**once** instead of at every use site (64 compositions: 0.06 s with the table, 5.87 s without).
`126` then read the shipped `Capacity` precedent: **keep the const standalone, compute in value
position**, one impl, any width, no cap, no enumeration, 0.04 s, with the price being canonicity and
type-from-width selection, which the table was actually paying for. `129` closed the loop: **make
the precision the const parameter and never compute it in type position**, canonicity by
construction, no feature gates (its capstone `q13` recorded in `129_probes/`), and refuted GCA's
usefulness for canonicity along the way (`(A + B) + C` and `A + (B + C)` are distinct types under a
generic parameter). Op then withdrew canonicity as a requirement entirely (`130b`), which reframes
all three. **Register mark: sharpens Q9** (the C0-through-D space maps onto this thread but lacks the
binding-time split and the `Capacity` precedent route), **and bears on Q2** (what a consumer writes).

**T5. The `generic_const_args` vetting.** `128` vetted the feature to WATCH (allowed, sound, named
rough edges), with the `-Znext-solver=globally` flag it hard-requires flagged as a separate exposure
whose tier is op's call. Probes recorded as recovered into `128_probes/` and `128a_probes/`. The
current workspace forbids probing with it, so this is record rather than an invitation. **Register
mark: duplicates** the droplist's structural refusals in part; the flag-tier question is **new** and
open.

**T6. Diagnostics: the lever is a bound, not an equality.** Wherever a numeral mismatch can be
expressed as a trait bound (`E0277`) rather than a type equality (`E0308`), the error is readable for
free, and `#[diagnostic::on_unimplemented]` reaches neither `E0308` nor solver overflow (`E0275`)
(`124:2406-2440`). Independently found three times there. The decoder-ring "confirmed ceiling" on
alias expansion was itself overturned late by a base-ten digit encoding that compiles gate-free with
readable towers (`SETTLED_container.md:436-445`). **Register mark: sharpens Q9's diagnostic-cost
entries**, which currently price repairs (tags, base-ten ladders) without the bound-versus-equality
lever.

**T7. Erasure above the native widths has an honest comparison target.** The erasure claim was
re-based from "byte-identical to the native primitive" (meaningless above 128 bits) to "within 0 to 2
instructions of what a competent author writes by hand", measured at 192 through 1024 bits, with the
payload-is-one-limb condition as a derived property (`SETTLED_container.md:45-72`). **Register mark:
sharpens Q1's erasure clause**, whose current evidence is one-program-one-arity per `17`'s own
caveats.

### 3.2 The strategy axis

**T8. One preset name denotes two rows, one per number kind, and the key that fixes it was compiled
twice.** The ratified fixed-point and float preset tables diverge at four cells (most sharply:
`Warm`'s `StoredWidth` is doubled for fixed-point and minimum for float, because IEEE hardware
delivers correctly-rounded intermediates free and fixed point has no hardware behind it,
`124:2666-2672`). A nullary associated type on a marker cannot carry a two-row fact (`E0119`
compiled); keying on the numeral over-keys (two same-kind numerals free to disagree); per-kind
markers admit well-typed nonsense and delete the four ratified marker types; **keying both contracts
on the exponent form** was reached independently by two members and refuses the wrong pairing with a
readable error (`124:2674-2795`). **Register mark: sharpens Q5** (the axis question: this is a
concrete instance of a strategy cell being a function of something, with the function's key derived)
**and the what-a-strategy-is definition** (sections over a product of axes gains a compiled instance
of "which product").

**T9. The headroom thread: benched, attacked, reframed, and left unsettled.** Sequence: `139b`
condemned `Warm`'s 65-bit crossover on an instruction-count measurement (originally misreported
1600-vs-81, corrected 339-vs-81, ruling unchanged); `140` proposed deleting headroom for every
strategy; op held pending harness benches (`140b:69-71`, per `SETTLED_container.md:410-420`); `141`
built the benches (committed: `mock/benches/warm-container-*`, 57 CSV+meta+findings triples) and
found headroom loses everywhere measured honestly (45.3x at 8 bits down to 2.4x at 64 on a wrapping
reduction), with the mechanism being **the projection to `W` becoming a real instruction** rather
than footprint, and the biggest lever being where the projection sits (14.6x to 26.4x), which
nobody had proposed changing; `142` then found the headroom's ratified purpose (chain accuracy) was
structurally invisible to `141`'s value-identical cross-validation, and that the purpose is exactly
interior safety, already served by the derived `W + ceil(log2 n)` accumulator rule, making headroom
"a fixed, always-paid, per-value approximation of a derived, per-fold, frequently-free quantity"
(`142_giesen...md`, opening; benches committed at `mock/benches/warm-clamp-*`). Op's last recorded
word treated the whole body as one instance of evidence, unaudited (`142b:60-64` per
`SETTLED_container.md:416-419`). **The bench artifacts are current evidence; the interpretation is
not.** **Register mark: sharpens Q5 and Q6 heavily** (the current register cites these bench families
already but not `141`'s mechanism attribution or `142`'s interior-safety reframe), **and connects to
Q11** (the accumulator options).

**T10. Only `Hot` folds for signed values, in op's own register.** `talk:1702-1715` (read at
`124:2628-2641`): clamping and refusing are both unfaithful for signed folds, wrapping is exactly
`ℤ/2ⁿℤ`, so the marker chosen for speed is the only one whose signed folds the type system permits.
The current panel measured the same fact from the other side (Q12's 70.1% signed-saturating
divergence) without knowing op had stated it. **Register mark: sharpens Q12** (adds op provenance to
a measured fact) and **bears on I8**.

**T11. The strategy door rests on one measurement.** A hardware-float lowering is not a `Lowering`
unless the environment is pinned: FPCR rounding-mode and flush-to-zero changes were measured changing
values (`1.0/3.0` low bit; `MIN_POSITIVE * 0.5` whole value), so const-folded and runtime float
expressions can disagree with nothing in the type system seeing it (`124:2839-2846`). Also: Apple
silicon shows no subnormal cliff, so the usual flush-to-zero argument does not transfer to this
target (`124:2847-2853`). And the door binds refusal-not-fallback design-wide, with the software
quantiser at roughly ten to seventeen times the hardware path (`124:2799-2816`, including the one
recorded case of a consolidation reverting a correction). **Register mark: new** (no current question
covers the door or the environment).

**T12. Preset divergence has a working mechanism op declined to adopt as-is.** A generic parameter
default projecting off the parent preset, probe-verified, feature-free, with op's instruction "this
deserves more than the first mechanism that works; a later member should take it further"
(`124:2860-2866`). **Register mark: revives** (an op-instructed open thread that fell out of every
record).

### 3.3 Quantiser, laws, folds

**T13. Round first, classify second, confirmed at scale.** The quantiser order was confirmed against
binary32 on 41,380,159 operations with zero mismatches, and the radix-general kernel regression-
checked bit-for-bit against silicon (`124:1157-1159`). Classify-then-round is droplisted with the
band that kills it. The current panel's format unit built its own adaptation model (C1 through C10 at
`63` section 6) without this instrument. **Register mark: sharpens** the format unit's C4/C5
(adaptation members and their laws) with a big-sweep instrument worth rebuilding.

**T14. The overflow band has a two-clause closed form with zero under-prediction.** Lattice clause
plus reachability clause, measured over 5,184 triples, with the candidate one-clause form refuted
by enumeration first, and a six-row table mapping every previously-stated band member onto it
(`124:1207-1233`). **Register mark: new** for the current register (no live question covers overflow
bands), though it slots under the format unit's adaptation work.

**T15. The finest-view mechanism, and the one identification inside it that is refuted and
unrepaired.** The grade-monoid/view-homomorphism construction with a unique finest view per law,
compiled exhaustively over nine views and 81 view pairs, with the nine-point lattice not a chain
(`SETTLED_laws.md:75-101`). Inside it, the Kleene-equation-at-(Presence, Ignore) identification was
compile-refuted wherever `Specials` is populated, because definedness is not recoverable from the
cause component; the probe that made it look safe had set its own definedness flag from its own
cause counter, "the model made the invariant true rather than testing it", which is why twenty-four
files carried it (`SETTLED_laws.md:374-405`). Three repair shapes were offered and none chosen; op
declared the resting calls stale and asked for a re-evaluation that never ran. **Register mark:
sharpens** the format unit's law layer (whose induced-algebra grading at `63` C4/C6 is a cousin
construction), and the unrepaired middle identification is a **blind spot** the current panel should
know exists before it reinvents a three-relation vocabulary.

**T16. The `TotalOrd` split is a precondition of distributivity, established by a compile refusal.**
The shipped bit-comparator is IEEE `totalOrder` under the wrong name (`-0.0 < 0.0` under it); a
`const` assertion that same-value data compare `Equal` refuses to compile against it and compiles
clean against canonicalise-then-compare; and under the shipped order the distributivity
biconditional's two sides quantify over different objects (`SETTLED_laws.md:103-133`). Two-experts
there; the workspace rule `what-you-can-observe-is-what-you-guaranteed.md` descends from the same
pass. **Register mark: new** for the current register (no live question covers order or comparison
vocabulary; `63` section 6 notes "the comparison vocabulary no unit has touched").

**T17. The closure laws: addition iff `bias/adjustment` is an integer; narrowed multiplication iff
adjustment and bias are integers and adjustment divides `bias² - bias`.** Compiled exhaustively both
directions, with the derived consequence that multiplication needs `mul_full` and addition does not
(`SETTLED_laws.md:138-160`). Plus the correction that travels with it: no `AddClosed` gate ever
shipped; the "shipped" wording was the drift. **Register mark: sharpens** the format unit's C3
(affine slot function) with exact closure conditions its current statement lacks.

**T18. Interior safety and total safety are two conditions with a stated relation, and a float fold's
accumulator is an `Implicit` numeral of computed width.** The `n-1` and `n` factors; binary32 needs
277 bits plus `ceil(log2 n)` for a sum, 554 for a dot product, binary64 2,098 and 4,196 (the quire,
derived rather than imported); checked at 2.9 million triples with in-format folds disagreeing at
23.17% as the control (`124:1470-1517`). Also: `fold_compensated` must never receive the
reassociation licence, compiled (the compensation term folds to `fsub s0, s1, s1`, always zero)
(`124:1552-1561`). **Register mark: sharpens Q11 and Q12** (the current fold options name an
accumulator relation; the archive has its exact form for floats and the licence interaction).

**T19. The site count and the moved count are two facts at two layers, and IEEE's own flag carrier
does not exist in Rust.** The grade's event generator split into a compile-time site count (one
instruction outside the loop) and a data-dependent moved count (seven to nine times cost, kills
unrolling), with the design committed to both in different organs for forty files because they agree
everywhere except conditional resolutions; and the measured fact that `fetestexcept`/FPCR access is
absent from the pinned toolchain's surface, so a value-carried grade is the only carrier available,
not merely the better one (`124:1961-2045`). **Register mark: new** (no current question covers
grades or flags; the format unit's exactness predicate is adjacent).

**T20. The algorithm crates return wrong answers under the presets they admit, and the atom is
`Monotone<Add>`.** `upward_rank` under `Hot` inverts a longer path's ranking against a shorter
(compiled, four-node chain); `Precise` degrades to a tie but never inverts; wrapping is not monotone
(`200 + 200 = 144`), which is the mechanism; the design's own fix is a fold-shaped result numeral
`foldnum(W, A)` plus a two-door split (value-returning door needs no monotonicity; ordering-returning
door needs `Monotone`) (`124:2460-2525`). Also the scheduled deletion: hand-rolled annihilator
substitutes get deleted when `Specials` lands as a real numeral. **Register mark: sharpens I11 and
Q11** (op's selling point is the algo crates; no current question yet covers their contract
discipline). Note the archive's consumer-pressure correction: hilavitkutin ships a hand-rolled hop
count and imports none of these functions, so urgency claims were wrong while the defects stand.

**T21. Parse is the quantiser, and print closes a const round trip.** Parse decomposes as
`quantise ∘ rational-of-digits`, checked over 318,126 strings: direct parse equals
nearest-ties-to-even everywhere; staging through a wider intermediate with nearest at both steps
disagrees on 3.2% of strings; round-to-odd at the intermediate agrees on all, licensing the `ToOdd`
member for staged pipelines with two guard digits. Print: shortest correctly-rounded digit string
exists for all 1152 model data, both kernels const-callable (`124:3041-3058`). The round-to-odd
validity bound `W >= F + 2` was named as a checked const bound owed and never shipped. **Register
mark: new.**

### 3.4 Order, conversion, and the family question

**T22. The inclusion order needs four conditions; two are unsound.** Grid refinement, phase
alignment, both endpoints; the two-condition form produced 17,037 false positives on its own sweep,
invisible because every numeral in the establishing sweep had bias zero (`SETTLED_laws.md:254-276`).
Two-experts there, derived independently. **Register mark: sharpens Q10** (the singleton amendment
question presupposes the predicate; the archive's four-condition form is the predicate's fuller
lineage) **and Q8** (every lattice claim rests on it).

**T23. The cardinality antichain.** Equal finite cardinality plus inclusion implies equality, for
every bias, adjustment, radix, sign domain, in every family including unwritten ones; compiled at
254,016 ordered pairs; **the coordinate-restricted "equal precision is an antichain" form is false
for float-shaped numerals**, where precision and cardinality come apart (`SETTLED_laws.md:315-334`,
417-431). **Register mark: sharpens Q8.**

**T24. The within-family/cross-family lattice dissolution, and a live numeric discrepancy.** Three
files gave three incompatible lattice answers; the final read dissolved it (each was answering a true
statement about a different shape space: within one family both operations total under two closure
conditions, across families joins fail and adding floats removed joins that had existed), one-expert,
unratified, with the panel ending on it (`SETTLED_laws.md:471-498`). And the discrepancy: one file
reports 81 decided join failures in the unbiased radix-two slice, two instruments in another find
zero there, a third instrument owed, "it will poison a consolidation that quotes either number"
(`SETTLED.md:130-133`). **Register mark: sharpens Q8 directly** (the current one-family-or-several
options, the tie-break reading D and step-set reading E, would all be tested against this material),
and the discrepancy is a standing hazard for anyone importing counts from either lineage.

**T25. Conversion needs an adjudicating-strategy key the schema lacks.** 33% disagreement across
lossy conversions depending on which of the two strategies' rows is consulted, with the no-new-key
claim shown undetectable from its own test setup (`SETTLED_laws.md:355-370`). Narrowing itself is the
quantiser with the operation set to identity, resolved by the target strategy's row, and `Hot`'s
narrowing is not monotone (`SETTLED_laws.md:336-353`). **Register mark: sharpens Q27** (which
strategy's laws govern a cross-strategy operation is exactly this question's sibling; the current
entry notes nothing addresses it directly).

**T26. `From` between numerals: the coherence walls and the compliant spelling.** By-reference
`From` never collides with core's reflexive impl (head-constructor argument, two-experts);
`TryFrom` cannot coexist beside it and the design does not want it to; a compliant gate-free
spelling carries the inclusion order as a trait bound rather than a computed const (one-expert,
second read owed); and op refused the "therefore no `From`" inference by name, his eighth refusal of
that shape (`SETTLED_surface.md:322-394`). **Register mark: new** (conversion surface is untouched by
the current register beyond Q3's existence question).

**T27. Membership: every arvo value is `m · r^q`, the finest inhabited system is unique on the
sub-ℚ chain, and the branch count depends on the embedding signature.** Two-read uniqueness theorem;
Ostrowski refutation of the chain justification; the seven upper vocabulary members' two symmetric
readings left as op's (`124:1281-1353`). **Register mark: new**, adjacent to Q20/Q21 (the number-
systems unit's open/closed and breadth questions), which it would sharpen with the worked decade of
vocabulary (ℕ through p-adic, D38/D39).

### 3.5 Crossing, encoding, storage

**T28. The crossing contract is three statements plus a precondition, and the leak is a family.**
`decode ∘ encode = id` always; `encode ∘ decode` idempotent always; identity-on-data iff injective
(derived, genuinely two-valued, with E4M3FNUZ as the injective-with-specials witness); the
precondition (decode's codomain inside the value set) whose absence makes statements ill-typed
rather than false; and the escape matrix at E4M3's shape where **six of eight cells leak** whenever a
`Numeral` axis shrinks the value set with no `Lowering` axis shrinking the datum set
(`124:1049-1110`). The repair (`Crosses`, an `unsafe impl` at the format declaration site, each impl
a named trusted-base entry) was derived as the only one after the quantiser refused every escaping
datum; three alternatives (a `Maybe` decode, a runtime check, a per-operation predicate) refused on
binding-time grounds. **Register mark: new**; the format unit's C2 (identity) is upstream of this
and would meet it the moment encodings enter.

**T29. Three width levels, one declared; bitpacked has one meaning and its group arithmetic is a
theorem.** Fields extent, stored width (the carrier, the only declared level), container width
(derived, never declared); `Bitpacked` means zero inter-value padding, with the byte-aligned reading
being `Dense` at a narrow width (a prior measurement retroactively relabelled); the period
`P = 8/gcd(W_S, 8)` and group byte count proved algebraically and checked at every width 1 through
57; **the write granule**: no element is independently writable, partition boundaries must be
multiples of `P` (`124:2873-3024`). Decode-plan-on-the-type corrected the old 4.6x sequential
multiple to 1.50x (sum) and 1.29x (per-element work), ratified there at `82b`, after three prior
measurement errors were found; the footprint sweep found 1.66x at L1-resident sizes dropping to
1.43x once dense leaves L2 (bench trail committed under `mock/benches/bitpack-*`). Op kept the
footprint bench preliminary and named **concurrent multi-column bandwidth contention as the
measurement `Cold`'s intent is actually waiting on** (`108b`, quoted at `124:3079-3082`). The
current panel's Q7 contention run (`26`/`27`) is precisely that owed measurement, performed under
current discipline. **Register mark: duplicates Q7 in part** (the current run supersedes the owed
item), **sharpens** it with the write-side and strided-access gaps both lineages still name, and the
group arithmetic itself is **new** to the register.

**T30. The digest contract, the mutation perimeter, and the niche doors.** A datum-keyed digest
masks to the placement map's occupancy (the one-word widening from "the fields' width", needed the
moment a placement has an interior hole, exhibited exhaustively at 65,536 container values); a
value-keyed digest is never a masking operation; the mutation theorem runs per byte-owner and per
level, with a compiled demonstration that a safe `&mut` into padding decorrelates a raw-byte digest
silently; an integer-typed door onto a niche carrier is UB with zero diagnostics while a niche-typed
door is unconditionally safe (`124:1791-1845`, `124:2928-3003`). **Register mark: new.**

**T31. The byte-image guarantee is same-process, same-target; portability is a downstream-contract
item.** The format's identity is a closed const-derivable bundle that must travel with the bytes or
be agreed out of band; no mechanism proposed, the item named as owed (`124:3026-3033`). **Register
mark: new.**

### 3.6 Operations: division and the elementary functions

**T32. Division dissolved into a general failure classifier, on silicon facts.** The solution-set
derivation (singleton; empty-with-direction, borrowing the range event's own resolution row;
everything-or-nothing, `invalid`, partial where no NaN exists); the `x/0` fork killed by compiled
and silicon-read facts (aarch64 returns 0 at every dividend, x86 faults, RISC-V defines a third
value, LLVM IR carries UB and deletes post-division guards); `Hot`'s cell delegated to the consumer
as an arity-two `div_or`-shaped fallback, with arity matching the failure taxonomy; and the residue
"a lowering may be a derivation input or an implementation of a stated value, never the author of
one", with the deletion test as its decision procedure (`124:1847-1959`). **Register mark: new**,
and the classifier was reused without edit by the elementary functions, which is the strongest
archive evidence that it is general.

**T33. The elementary functions sort into three classes by exact carrier, and the transcendental
boundary is the family's own.** Roots: the residue pair decides correct rounding in one comparison
(`r > m`), ties impossible by parity, linear growth class, three branchless instructions after the
root; radix-power exponential: decidable but width-doubling per fractional bit; transcendentals: no
exact carrier (Lindemann-Weierstrass), ties impossible, and correct rounding promised **exactly
where a hardness constant is exhausted or cited** (measured at 11, 9, 10 extra bits at three model
numerals, no visible formula, does not transfer across widths), elsewhere a licensed approximation
with a type-level error bound and a differential parity suite as verifier (`124:2135-2233`). The
admission test ("an operation joins the surface by stating five things") was op-confirmed there.
**Register mark: new.**

**T34. Radix ten: cohorts are a choice, the standard's preferred exponent is a datum rule the design
cannot express, and the sharper claim is that it does not need to.** The chain (only the first link
is about radix); measured non-injectivity sources (cohorts 41/600; non-canonical codes 209/768);
and the position: arvo's decimal `Ranged` numerals deliver IEEE's values without preferred-exponent
conformance, while a consumer for whom the quantum is part of the number uses a decimal `Implicit`
numeral where the exponent is a type, checked at compile time, "strictly stronger than the
standard's own rule" (`124:2245-2313`). Plus the two `Pos` ceilings bisected to the exact bit
(structural at 128 bits, `u64` readout at 65) and the wall: the absorbed-adjustment spelling does
not compile at any real decimal format's range while the radix-and-exponent spelling compiles in
64 ms. **Register mark: new**, and it is the archive's most complete worked case of the
represent-the-standards test (1.1/1.2) actually running.

### 3.7 The peripheral subjects, each a whole topic the current register does not yet carry

**T35. The truth contract.** Boolean algebras form a variety, so a lane mask is `Bool^W`
structurally; the homomorphisms from n-lane truth to one-lane are exactly the n coordinate
projections (exhaustive at n = 2 and 3), so `all` and `any` are outside the algebra and **the exit
exists exactly at one lane**; the operation that generalises is a selector keyed on the pair (truth,
datum), lane-wise, both arms evaluated, with the two wrong reductions producing two different wrong
answers from safe code; five silent exit routes enumerated, two closable by `negative_impls`, three
by grep (`124:4714-4929`). Op confirmed-and-corrected the shape there (the persona's third clause
was backwards; producers bind on the algebra, the exit belongs to the operation that branches).
**Register mark: new.**

**T36. The bitfield.** A heterogeneous product of numerals under a declared placement map, with the
bitpacked column its homogeneous sibling (four-cell classification, offsets declared exactly when an
external document fixed them); the composite of two placements is a placement, swept at 4096
elements; the shipped overlap defect (two fields sharing bits, one setter silently truncating the
other, safe code, no diagnostic) with the ruling **an overlap that is stated is a declaration, a
silent one is a falsehood the compiler can see**; a foreign bitfield is pinned to `W_S = W_C` and
cannot be bitpacked; an internal one adds no trusted-base entry while a foreign one adds exactly the
datasheet correspondence (`124:4319-4516`). **Register mark: new.**

**T37. Capacity and the array grammar.** Capacity as a direct `Nat` instance (one seal, one
arithmetic); the last-index predecessor construction (`Dec`, refusing at `Z` before
monomorphisation); the `AGREES` two-half repair (an associated const nothing touches is not
evaluated, so the check needs a second reference inside `COUNT`); and the reopened forcing argument
with three priced columns (const parameter; numeral-plus-literal; numeral-plus-derived-storage),
where the ratified sentence "forced by the language" was found false twice while is-a stands
(`124:4010-4200`). **Register mark: new**, and T2's re-run touches its third column.

**T38. Shape and geometry.** Shape as the index-domain layer with `Capacity` its rank-1 case; the
flattened bitpacked reading (never worse in footprint over 131,072 cases, coincidence condition
derived not sampled); the rotor storage correction (`2^(n-1)`, not `1 + n(n-1)/2`, reversing the
rotor-versus-matrix comparison at rank 7 while the decision survives on other grounds); and the
closed-interval finding: the obvious purely-fractional numeral misses its representable one by one
quantum at every width, and the fix is `Adjustment = 1/(r^F - 1)` (a parameter value, not a new
type), compiled to F = 24, with the both-routes-exposed ruling (`124:4202-4317`). **Register mark:
new**; the identity-not-representable finding independently corroborates the current panel's own
`UFixed<0, F>` no-representable-one material in the workspace test-gate lore.

**T39. The platform crate and `Bool`'s six doors.** The naming-door charter (one route per
primitive); `Bool` reaches its `bool` through six public routes today; route multiplicity is a
defect only relative to a guarantee, and `Bool` has none (compiled exhaustively), which moves the
pruning call from soundness to vocabulary, op's; and the crate-boundary measurement (constructor
without `#[inline(always)]`: 34 instructions against 22, silent) (`124:4518-4615`). **Register mark:
new.**

**T40. The notation vehicle.** A proc-macro is forced (a decimal literal is one atomic token;
`macro_rules!` cannot start), no external dependency needed; two entry points over one generator
because role (scale versus offset) belongs in the type (the swapped-argument probe silently denotes
11 versus 84.33); the 923-assertion whole-matrix test that caught a real reduction bug pre-review;
the two-tier refusal at the readout ceiling (`124:2315-2404`). **Register mark: new.**

**T41. The cost model.** Composition pricing measured twice independently (dyadic ~2.2 ms per
distinct composition; arbitrary 16-bit rationals ~143 ms; the cliff printed: 100 distinct rational
compositions cost 14.3 s), with the finding that the numerator dominates and the realistic profiles
sit two orders below the sweep previously called realistic; declaration pricing is a separate
operation (pre-reduced 2.8 to 3.1 ms against 13.8 ms forcing `Reduce`) (`124:3527-3575`). **Register
mark: new**, and the bulk-import (MATLAB) profile is the one that pays.

**T42. The environment parameter.** An environment fact is an **assumption, never a witness**; the
operative test is whether a linked library can change the fact at runtime, applied per-fact-as-
lowered; the receipt's verdict splits (cannot-check yields a trusted-base entry with a declared gap;
cannot-provide is a statically known falsehood and refuses), a distinction reused independently at
two further layers; and the FZ16 hole that showed the receipt list had no negative control
(`124:3949-3998`). **Register mark: new.**

---

## 4. Refuted or dropped, beyond what the carried droplist already holds

`DROPLIST.md` carries the archive's cumulative droplist verbatim (its sections 6 and 7), so the bulk
of the closed routes are already in front of the current panel with their diagnostics. What follows
is only what that carry-over does NOT show.

**T43. Ratified-then-reversed is a category, and the droplist under-represents it.** The droplist
records removals; it does not flag which removals had been *ratified* first. Four had: canonicity
(`127b` ratified, `130b` withdrawn by op, later structurally confirmed dead by the antichain);
the width enumeration (converged with two expert agreements and three compiled refutations of
alternatives, overturned by op the same day it would have entered the base); the `70b` `Warm` clamp
cell (declared stale under op's own restated intent at `142b`); and `Warm`'s headroom rule
(condemned at `139b`, its deletion benched at `141`/`142` and **left unratified at close**, so the
archive ends with the ratified table's `StoredWidth = doubled` cell and a committed bench trail
recommending its deletion standing unreconciled). A re-tester importing any preset-table cell from
the seed should know which cells sat in this category. Sources: `SETTLED_container.md:385-420`,
`SETTLED_strategy.md` casualties.

**T44. Claims that died of provenance rather than content.** File `130` cited five probe files that
existed nowhere (the incident behind the workspace's evidence rule); the probes were later recovered
in a bulk pass (fourteen directories, 361 files, recorded at `140b` per
`SETTLED_surface.md:449-458`), and the claim had by then been independently redone. File `146`'s
`From` compile carried `#![feature(generic_const_args)]` plus `-Znext-solver=globally`, both
disallowed, and was voided as evidence while its coherence argument survived on the permitted
solver (`SETTLED_laws.md:448-459`). File `59`'s strategy-door table was voided whole for
`tree-meaning` grounding (justifying design rows from shipped doc comments), the incident behind the
archive's forbidden-ground rule. The lesson content for the current panel duplicates `RULES.md`'s
evidence section; the instances are listed because re-testers will encounter these files' claims
quoted elsewhere in the archive.

**T45. The benches that had never been committed.** The archive discovered mid-run that
`mock/benches/.gitignore` had been discarding every bench artifact the panel produced, which forced
a re-measurement and softened a ratified figure (13-to-17x becoming 10-to-17x) and is the origin of
the workspace's committed-artifact discipline (`124:2804-2816`). Anyone comparing archive-era
numbers against the current committed trails should know the era boundary exists.

**T46. Refutations of the archive's own instruments, kept because instruments get rebuilt.** The
erasure oracle's two false-negative regimes (the current Q1 entry carries this); the carrier-only
derivation passing the erasure certifier at full marks while occupying 23.1% more memory than the
strategy promises, because the instrument compares one operation against one instruction and has no
array in it (`OPTIONS.md`'s derivation-outputs section carries this from the current panel's side);
the packed round-trip check that returns correct answers whenever the truncated bits happened to be
zero (data-dependent blindness); and the model-inadequacy asymmetry: a model that undercounts
refusals fails loudly, **a model too narrow to see a value disagreement returns a quietly wrong
number, and no mechanical guard for the second case was ever found** (`124:3821-3829`). That last
sentence is a standing hazard for every model-width sweep the current panel runs.

## 5. Theorised and abandoned inside the archive, with the reasons

An abandoned idea plus its reason outranks either alone. These are ideas the archive itself gave up,
distinct from droplist refutations in that the current droplist's terse entries do not convey the
arc.

**T47. The three-relation ladder (weak, Kleene, graded) as the law vocabulary.** Imported from
partial-algebra literature, then replaced outright by the nine-point view lattice because the named
relations turned out to be three points of a larger non-chain space, and the question "which
relation" dissolved into "which finest view" (`SETTLED_laws.md:75-101`). Abandoned with it: op's
adopted name "partial associativity" (see 1.12). If the current format unit's law layer ever
reaches for the literature's relation names, this arc is the warning that the archive tried it and
found the space bigger than the names.

**T48. `Growth` and `Widening` as axes.** Both ratified into the ten-axis table, both later ratified
out at `39b`: `Widening`'s three instances decompose into which primitive is named, what numeral its
return type is, and that numeral's own storage; `Growth` left the law key and then `Policy` entirely
after a compiled structural theorem (no operation expressible in the dispatch discipline can have
policy-dependent growth, coherence refusing the two-impl shape outright) (`124:1627-1702`). The
current panel's Q5 axis debate should know the archive both added and removed axes, and what the
removal argument looked like.

**T49. `LogicalWidth` as a primitive axis, and the ten-axis table itself.** D69's table was
overturned at `30b` (identity parameterised in mathematical coordinates; precision primitive, total
width derived), and the surviving members were reshaped continuously. The archive's final position:
no replacement count asserted, because the trait-table members and D69's axes are different
populations and an uncheckable count is the shape the discipline refuses (`124:1651-1658`).

**T50. The single `Sign` axis.** Split into `SignDomain` (a value fact) and `SignIndexing` (a datum
fact) after one cell (`SC_SAT_SYM` versus `SC_SAT`: the identical clamp delivering -8 under
`AsymmetricLow` and -7 under `Symmetric`) showed the single axis under-determined the set; the split
is what makes SystemC's own pair expressible, the standards test passing (`124:986-991`).
**Register mark: sharpens** the current panel's sign-digit question (the `Precision` counts-the-sign
open item in `OPTIONS.md`), which is the same territory from the coordinate side.

**T51. The step-A/step-B seam as a neutral comparison frame.** True of the const-keyed route,
dissolving entirely under structural keying, and flagged as a mistake when used as a comparison
table for every candidate (`SETTLED_surface.md:462-469`). A warning label for Q9 work: the seam
vocabulary is route-relative.

**T52. The ambient-and-realisation decomposition.** Offered as a technique (meet and join in the
ambient lattice of finite rational sets, a partial `realise` back), never as a fourth answer to the
family question; its content (every derived numeral is the tightest numeral containing the exact
result set) was independently re-derived twice and kept while the framing was dropped. The current
Q8 already carries this from the current panel's own `03`/`07`/`08`; the archive's parallel arrival
is corroboration-shaped history, not corroboration.

## 6. Blind spots: questions the archive asked and left, or never asked

**T53. The 29-item queue addressed to op at close.** `124` section 2 ends with a ranked list of
twenty-nine op-owed items, and the panel closed with nearly all of them unanswered (the panel was
closed, not finished). The list itself (`124:5075-5174`) is the densest single inventory of
unresolved design calls in the archive: the array-grammar fork, the truth-contract reductions, the
`S` bound confirmation, the contract split's one-line ratification, the D68 supersession, the
register's grid marker, `W_MAX`, the platform crate's name, three token collisions, `Bool`'s
surviving door, the rotation benches, division's grading axis, `Hot`'s default float environment,
the sampling spec, the perimeter-rule clause, the taxonomy rows, `FromConstant`, the four
`unstable-features.md` edits, transcendental packaging, the digest history split, membership
scoping, preset divergence, the cadence discrepancy, the seventeen unreconciled round rows, the
tautology checklist, construction one's status, `Folded`'s witness, the container projection's home,
and the `TotalOrd` one-or-two-traits question. Every one is a candidate talking point when its
subject area reopens; they are indexed here once rather than duplicated per section.

**T54. The owed-artifact list at close.** `124` section 5's owed items each name a closing artifact
and none was produced: the Boolean-algebra law suite (the largest: no law is asserted anywhere over
any truth type, and 672 green tests say nothing about the variety membership the truth-contract
argument rests on); the bitfield overlap tests (both directions); the IEEE primary-source reads
(verbatim, position-cited: clauses 5, 7, 7.6, 9.2, §4.3.1, §5.12) and the ISA bundle (ARM DDI 0487,
Intel SDM, RISC-V, LLVM LangRef), everything having been cited secondary; the nine-bit `u16`-class
companion model for the container-class transfer coordinate; the `foldnum` compile against the real
contract; statement 0 against `quantize` and `roundToIntegralExact`; the saturating-reduction
vector kernels (`uqadd`, `paddus*`), a real unpriced cost landing on arvo; the multi-limb
carry-chain codegen test (a dependency on an optimiser heuristic with no fallback intrinsic on
aarch64); the round-to-odd `W >= F + 2` const bound; the `notko-hlist` binding-time sentence
(flagged by six members independently, never opened); and the uniform-sampling question in
`arvo-pseudorand` (uniform over values or over data, divergent the moment the grid is non-uniform).
A re-tester picking any archive topic should check this list first, because the gap it names is
usually the cheapest attack on the topic's carried conclusion.

**T55. The three instruments named and never run.** A sweep of the ninety-nine member files and
probe directories for material no consolidation absorbed; the register diff against the
inherited-state topic file (fifty-two decisions, never diffed against anything); and a restoration
ledger keyed on establishing sources (`124:5564-5575`). The first two find material; this file is
not any of them (it swept consolidations, sweeps, op files and the late stretch, not the ninety-nine
member files individually).

**T56. Thread A, B, C: op explicitly kept all three open with instructions to keep iterating.**
Thread A, the consumer-facing diagnostic surface ("Option 1 but not just price, iterate on; there
might be ergonomics to be won"), whose strongest measured result was nominal constructors plus
per-axis modifier types rendering every axis for free in errors, with three unresolved costs.
Thread B, fallible arithmetic ("what does the best possible form unlock"), whose sharpest reframe
was that **delivery of a refusal (sum type, absorbing bottom in a spare pattern, sticky flag) is a
`Lowering`-level choice by the axis-sorting test**, with three real unresolved costs including the
IEEE-2008 min/max selection defect reappearing in a shipped algorithm crate, and the finding that
`ConstantTime` was keyed on data that does not decide it. Thread C, leaf truth ("find a shape where
the check IS the typestate rather than sitting beside it"), never achieved. (`124:5586-5696`.)
**Register mark: new**, all three; Thread B touches Q4's `Precise`-on-`inexact` material and I5.

**T57. Questions the archive never asked.** Noted while sweeping, as absences: nothing in 320 files
priced compile cost for a real consumer crate (named as open repeatedly, never done); nothing
measured any strategy on any target other than one Apple M1 (every silicon fact is one host plus
cross-compiled assembly reads); random and strided access patterns were never measured for any
layout (both lineages name it); the write side of packing was never priced (the current Q7 carries
the same gap); and no file ever asked what the canon's own document structure should be, which the
current panel now owns via the canon rules. These are gaps in both panels' evidence, stated so the
overlap is visible.

## 7. What re-testing costs, and where the instruments stand

**The probe corpus largely survives on disk.** 117 probe directories in the archive, recovered
where they had been scratch-only (the `140b` recovery pass; `128_probes/`, `129_probes/` and
kin exist). The two re-runs performed for this file each cost under a second of compile time; the
pattern (copy the probe beside this file, compile under the pin, commit source and log) is the
route by which any archive claim becomes current evidence. Claims resting on **exhaustive sweeps**
(the 41M-operation quantiser check, the 509M-instance symmetry check, the 65,536-value digest
sweeps) are minutes of compute; claims resting on **the bench harness** are already current where
their artifacts are committed in `mock/benches/`, and re-interpretation rather than re-measurement
is what those need.

**Priority order, if the current panel wants one instrument re-established per live question.** Q5
and Q6: the exponent-form preset key probes (archive files 119/122) and the `warm-clamp-*` trails
already committed. Q8: the within-family closure conditions and the 81-versus-zero discrepancy
(T24), which poisons any imported count until resolved. Q9: the total ladder (re-run done), the
33.4-second emission defect, and the `W_MAX` pricing curve. Q11 and Q12: the interior-safety
accumulator forms and the signed-fold material. The peripheral topics (T35 through T42) each carry
their own probe directories and none is blocked on anything.

## 8. Coverage, bounded honestly

**Read closely:** the archive's final consolidation `124_consolidation_twelve.md` in full except
parts of sections 3, 4, and 6 through 10 (its droplist section 6 is carried verbatim in the current
`DROPLIST.md` and was read there); `112_the_op_material_sweep.md` in full; `SETTLED.md` and all
four `SETTLED_*.md` sweeps in full (identical to the seed copies, diffed to confirm);
`13c_op_the_standard_and_the_mode.md` at the quoted ranges; the decision-register talk file at the
D54, D56 and D67 ranges; the openings of `125`, `126`, `128`, `129`, `141`, `142`;
`137_probes/p5_total_ladder.rs` and `76_probes/b1_structural_array.rs` (compiled). On the current
panel's side: `INTENTS.md`, `RULES.md`, `OPTIONS.md`, `DROPLIST.md` in full, and
`63_spj_consolidation_the_format_concept.md` sections 6 and 7.

**Skimmed via structure and cross-reference only:** the remaining op checkpoints (their content
reached through `124` section 2 and `112`, both of which quote them at line ranges; headline quotes
verified at source where they lead a section above); the post-124 files `130` through `151` beyond
what the four sweeps carry (the sweeps' own coverage statements were used as the map, and they
declare exactly what they read).

**Not opened:** the ninety-nine numbered member files `01` through `108` individually (reached only
through the consolidations and sweeps, which is the same exposure the archive's own instruments
warned about); the probe directories' contents beyond the two compiled and the listings checked;
`110_consolidation_eleven.md` (superseded by `124` on its own terms); `109`, `111`, `113` through
`123` (reached through `124`'s citations of them). **A talking point sourced only from `124` or a
sweep inherits those documents' compressions**, and the archive's own history says compressions
drop live options; the member-file sweep (T55, first instrument) is the corrective this file does
not perform.

**What this file would most mislead a reader about if read carelessly:** it is organised to look
like a map of established results. It is a map of claims. The archive's most instructive property,
demonstrated a dozen times above, is that its confident, compiled, consolidated claims kept being
overturned by the next instrument pointed at them. That property should be assumed to hold of
everything in this file, including the parts that look most settled.
