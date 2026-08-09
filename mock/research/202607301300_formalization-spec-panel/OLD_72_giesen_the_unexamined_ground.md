# 72. The unexamined ground, enumerated by category, and the best of it taken: the external images of a value

Fabian Giesen, file 72. I wrote files 34 and 48, both assemblies; three of file 34's claims were later
overturned by members who recompiled them, which I carry as calibration rather than defend: everything
below that can be compiled is compiled, and everything that cannot says so in the sentence that states
it.

**What I read.** `68_consolidation_seven.md` in full, `68b_op_checkpoint_sixteen.md`,
`69_ringer_the_source_justification_sweep.md`, `70_wronski_the_presets_re_derived.md`,
`70b_op_checkpoint_seventeen.md`, and `71_smith_the_far_point_without_infinity.md`, all in full, all
required. One `ls` of the panel directory before starting: files `00` through `71` plus checkpoints and
probe directories, nothing after `71`. Then, because this dispatch's first half is a coverage question,
targeted reads driven by a grep sweep rather than by memory: `11_current_shape_draft.md` lines 24 to 70
(its own coverage table, which turns out to matter more than anything else I found),
`06_muratori_the_consumer_surface.md` lines 555 to 585, `01_knuth_mathematical_rigour.md` lines 315 to
345, `61_amin_the_notation_vehicle.md` at the notation vehicle's binding-time statements, and the
signed-zero, extensibility, SIMD, hash, and serialisation hit contexts named per finding below. I did
not read all seventy-one files end to end for this dispatch; section 1 states exactly what the sweep
can and cannot have found.

**Gates.** Canon gate, reproduced fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test
gate: `cargo test --offline --workspace` from `mock/`, summed per binary in this session rather than
trusted from a headline, 658 passed, 0 failed, 9 ignored, matching `68:64-65`. Toolchain `rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed by running `rustc --version`
inside the repo tree this session. The method constraint from `70b` governs throughout: no shipped doc
comment is cited as design justification anywhere below, and every tree citation is `tree-fact` or a
defect record.

**What is compiled, what is reasoned.** Six probes in `72_probes/`, compiled and run fresh this
session, outcomes verbatim in `72_probes/OUTCOMES.md`; claims so grounded are tagged compiled. The
enumeration in section 1 is a grep-and-read coverage survey, tagged as such, with its blind spots
stated. The chapter in sections 3 through 7 is reasoning from ratified intent and settled shapes, with
the compiled probes carrying the load-bearing steps.

## 1. The enumeration, and the method that keeps it from permuting

The panel's documented failure mode is permuting the axes already in hand and calling that
exploration. The only defence I know is to derive the category list from outside the review's own
history: not "what have we discussed", but "what does a numeric foundation have to answer for a number
to live a whole life", and then check each category against the corpus by grep rather than by
recollection. A number in real use is computed with (seventy-one files deep on that), but it is also
written down, read back, stored, loaded, compared into containers, hashed, displayed when a test
fails, generated at random, and extended past what the designers anticipated. Those are the rows.

The sweep: roughly thirty grep patterns per category family across every panel file, reading the
surrounding context of every hit that could bear on coverage. The honest limit, stated the way file 69
stated its own: a category whose discussion uses none of my search terms would be missed, and I read
hit contexts, not all seventy-one files end to end. The findings below are therefore "no file
surfaced under any pattern I tried", not "no sentence anywhere touches this".

**One meta-finding before the table, because it reframes the whole question.** The panel wrote its own
blind-spot map at file 11 and never went back to it. `11:44-56` is a twelve-row table of concerns the
design round settled that the panel never reviewed, marked row by row: `arvo-capacity`, `arvo-shape`,
`arvo-geom`, `arvo-num-systems` (since partially taken up through D38/D39), `arvo-platform`,
`arvo-container` ("despite being exactly what the new `Lowering` contract governs from above",
`11:51`), `arvo-bitfield`, `arvo-float`'s packaging boundary, the unified predicate concept,
`arvo-pseudorand` ("hash, PRNG, noise as one family... Reviewed by the panel: No", `11:55`), and
`notko-hlist`. Sixty-one files landed after that table and none re-visited it. Whatever this file
picks, the next exploring dispatch should start from `11:44-56` before inventing its own list.

The categories, with corpus status:

| Category | Corpus status | Evidence |
|---|---|---|
| Text at runtime: printing a computed value, parsing runtime text | **Unexamined as design.** The need is on record and unanswered: a real consumer carries four `Display of the raw index value` lint escapes (`06:567`) and "arvo's primitives have no `Debug` that respects the fixed-point scale" (`06:571-572`), recorded at file 06 and never answered in sixty-six files since. The notation vehicle is macro-expansion-time only, by its own binding-time statement (`61:520`). Zero corpus hits for round-trip printing, shortest representation, or any of the standard algorithms (Steele-White, Grisu, Ryū, Dragon4, Schubfach). | this file, sections 3 to 5 |
| The byte boundary: datum to bytes, endianness, padding, foreign bytes | **Unexamined except one orphaned sentence.** File 01, on FTZ-as-identity: a loadable subnormal pattern becomes "a non-value, and something must say what deserialising it means. Not an error; an unstated boundary" (`01:332`). Never picked up in seventy files. Zero hits for endianness or byte order anywhere in the panel. File 54's BID/DPD work is the value-to-datum crossing, not datum-to-bytes. | section 6 |
| The digest: hashing a number into a container | **Unexamined, and it is the layer-keying rule's own next instance.** `arvo-pseudorand` is marked panel-unreviewed at `11:55`; the closest approach is one clause at `60:203` ("a hash-adjacent canonicalisation") in a file about sort keys. The rule's three known instances (`TotalOrd`, spectral NaN, notation face, `68:135-139`) have an obvious fourth nobody has written down. | section 7 |
| Signed zero | **Examined, thoroughly.** Files 27, 28, 30, 31; settled on the datum side inside `Encoding::Canonical` (`31:357`, `36_probes/OUTCOMES.md` probe 6: "a numeral parameter is value-level and must not carry two zeros"). I checked before claiming otherwise, and I was wrong to suspect it: this was my leading candidate before the sweep. | closed, no action |
| SIMD and lanes | **Examined where it matters.** The vector-lane residue on float reductions is named and its closure route established (`20:99-116`, `26:445-447`, `17:552`). Known-open on the integer saturating side, but known. | no action here |
| Constructive third-party extensibility | **Known-open, flagged twice, never compiled.** "Whether [the tower is] extensible with new `Numeral` implementations from outside arvo... I did not test it... should be compiled before anyone treats the ten axes as third-party-extensible" (`22:475-479`); the same asymmetry predicted at `26:497-498`. The adversarial half is deeply examined (files 41, 42, 46, 49); the constructive half never. A cheap dispatch: one foreign crate, one honest new numeral, compile it against the model tower. | recommended as its own dispatch |
| Randomness, noise, PRNG over numerals | **Decided in the design round, panel-unreviewed** (`11:55`). Uniform sampling over a numeral's value set is a real spec question (uniform over values and uniform over data differ the moment the grid is non-uniform), but it sits behind the digest question, which is smaller and earlier. | after the digest |
| Elementary functions past division | Division is held by op (`68:358`); `Sqrt`/`Recip` exist as shipped contracts. The correctly-rounded story for irrational results (the table-maker's dilemma at arbitrary radix and precision) surfaced in no hit I read, but my sweep here was shallow and division's hold plausibly extends to it by intent. | flagged, low confidence in the coverage claim |
| Batch and column semantics (cross-word bitpacked extraction) | **Known-open**, already on the consolidation's own list (`68:928-929`). | no action here |

## 2. The pick, by the regret criterion

The first three rows are one subject wearing three coats: **what a value looks like outside the type
system, and which layer each external image is keyed on.** Text is the image a human reads, bytes are
the image a process or a file holds, the digest is the image a container indexes by. I take the
cluster as one chapter rather than picking one coat, because the design shape is shared and stating it
three times separately would build the redundancy the shared statement removes.

Regret-late, per the dispatch's criterion, and each half of op's stated end state is hit by one of
these. The ergonomic half: text is the first surface a consumer touches when something is wrong, the
recorded four-escapes-per-consumer gap of `06:567`, and a numeric foundation whose values cannot be
printed faithfully fails "invisible for the most part" on day one, visibly. The efficient half and the
ossification risk: byte images outlive code. This stack's consumers persist packed columns (the
engine's columns; the language's eventual save data), and a persistence format designed after the
canon is earmarked inherits whatever the canon forgot to say about padding, endianness, and foreign
bytes, permanently. The digest is the quiet one: a hash inconsistent with its container's equality is
a silent lost lookup, the exact defect class entry 7 of the live-defect registry documents at another
layer (`68:876-882`). And the cluster is cheap now for a specific reason the sections below compile
out: every hard sub-problem is already solved by machinery this review has ratified. Discovering that
late would not change the machinery; it would mean re-deriving this chapter after the canon closed,
against a shipped format.

## 3. The general form: every external image is a crossing or a projection, and the review already owns both shapes

The design has one boundary construction it has now validated at three sites: the crossing contract's
statement structure (round-trip on values always; canonicalisation idempotent on data; identity on
data as a derived boolean; a statement-0 precondition guarding the way back in, `68:183-274`), reused
for the extended grid at overflow (`70:288-292`) and underflow (`68:198-199`), and file 71 made that
reuse-of-a-validated-construction move explicit as method (`71:122-125`). This chapter is the fourth
site, and the claim is structural:

**An invertible external image (text, bytes) is a crossing, and takes the crossing contract's
statement structure verbatim. A one-way external image (a digest) is a projection, and takes a
factoring law through the layer-keying rule instead: it factors through the canonicalising projection
of the layer its paired equality lives at, and that projection is the only door.**

Nothing in that sentence is new mechanism. It is the crossing contract and the layer-keying rule,
quantified over a boundary the review has not yet pointed them at. The rest of this file instantiates
it, compiles the load-bearing steps, and states what each instantiation costs.

One correction to the layer-keying rule's own text falls out immediately, and I state it as a
completion rather than a contradiction. The rule assigns display to the face layer: "A fact depending
on where something was written belongs on the face, and nothing else does: diagnostics and display,
full stop" (`68:125-126`). For compile-time diagnostics that is right and stays right. But a face
cannot reach a numeral position, and the `NumeralFace::Encoding` projection erases the declaration
site (`68:130-137`), so **a computed value has no face**: by the time a fold's result exists, the
layer the rule assigns display to has been erased by the rule's own enforcement mechanism. The display
clause quantifies over a layer that does not survive to runtime. The completion: at runtime the
honest split is a **value-keyed display** (canonical, shortest round-trip, section 5) and a
**datum-keyed debug image** (shows the datum: NaN payload, cohort member, the raw fields), and the
rule's "coarsest layer whose identity the fact depends on" test picks between them exactly as it picks
everywhere else. *Reasoned, from `68:125-137`; the rule's own test applied to its own clause.*

## 4. Parse is the quantiser, and the review built it without noticing

Write the parse map honestly, the way file 67 wrote the crossing maps: a digit string in any radix
denotes an exact rational, by positional notation and nothing else. So parse decomposes as
`quantise ∘ rational-of-digits`, where `rational-of-digits` is bookkeeping (the notation vehicle
already performs exactly this fold at macro-expansion time: "parse, digit extraction, decimal-point
folding, gcd reduction", `68:391-393`) and **every semantic decision in parsing is the quantiser's**.
The rounding direction is the `Quantisation` triple. Out-of-range text is the `Resolution` pair, so
`Warm` text saturates to the far point and `Precise` text refuses, per the ratified preset tables and
file 71's far-point rule, with no new vocabulary. An inexact parse is a quantisation event and grades
like one, so the design gets IEEE 754's required inexact-conversion signalling (§5.12 of the 2019
standard, cited as a standard) through the grade it already carries, where IEEE needed the per-thread
flag word the review already declined as unreachable (`58` section 1.14).

Three compiled results (`72_probes/probe_1_parse_is_the_quantiser.rs`, model at radix 2, p = 8, e in
[-4, 4], 1152 values; every four-decimal-place string in range, 318,126 of them, the whole in-range
grid rather than a sample):

First, **single rounding from the exact rational equals nearest-representable-ties-to-even on every
string**, with the rounding step implemented as the quantiser's own remainder-comparison kernel. Parse
is one map, not a pipeline with its own rounding rules.

Second, **the defect class this law forbids is real and dense**: staging the same parse through a
wider intermediate (p = 12) with round-to-nearest at both steps disagrees with the direct parse on
10,053 of 318,126 strings, 3.2%, first witness the string `0.0642`. Every implementation that parses
text into a wide type and narrows (the obvious implementation, and what any consumer gluing
`str::parse::<f64>` to a conversion would build) is wrong at that density. The spec sentence has to
forbid it explicitly, because it is the default thing to write.

Third, **the staging licence exists and the vocabulary already contains it**: the identical staging
with round-to-odd at the intermediate step (p_mid >= p + 2) agrees with the direct parse on all
318,126 strings. File 01 named `ToOdd`'s presence in the `Resolution` vocabulary as "the classical
cure for double rounding" and "a real point in [the vocabulary's] favour" (`01:318-320`), seventy-one
files ago, as a strength with no job. This is its job: staged conversion pipelines are licensed
exactly when every intermediate step is round-to-odd at two guard digits, and the licence is spelled
in the design's own sealed vocabulary.

One domain statement the spec owes for this to typecheck, and file 67 already needs the same one: the
quantiser's domain is ℚ (with the far points per file 71), not merely the exact results of arithmetic
on representable values. File 67's repair composes `encode ∘ quantise ∘ decode` and notes it
"typechecks if `quantise : ℚ -> V` is total" (`67:178`); parse is the second consumer of that
totality, arriving from an unrelated direction, which is what makes it spec text rather than one
mechanism's convenience. The compiled kernel form differs, and the cost is honest: quantising an
arithmetic result compares against neighbours in the format's own radix; quantising a foreign-radix
rational needs exact integer comparisons whose operand sizes grow with digit count. Bounded, because
the digit budget is bounded (section 5), and the multi-limb arithmetic it needs at real widths is the
`carrying_mul` family the workspace has already vetted and adopted for exactly this kind of widening
work in `arvo-strategy`.

*Grounded on: settled shapes (`11:212-218` the `Quantisation` vocabulary, `01:318-320` the `ToOdd`
strength, `67:178` the totality condition), compiled (`72_probes/probe_1`), standard (IEEE 754-2019
§5.12, cited as external prior art, primary-source read owed per the panel's own convention),
reasoned (the grade convergence).*

## 5. Print, the H bound, and the buffer that fires the spine rule

The expensive precondition of every correct float-printing algorithm in the literature is exact access
to the value and its neighbour gaps. IEEE implementations sweat for this (Ryū's precomputed tables,
Grisu's error-bounded wide arithmetic with a fallback, Dragon4's big-integer path), because the datum
hides the value behind a biased-exponent encoding. **This design hands both over by construction**:
`decode` is "total arithmetic on the physical fields landing in the rationals, `decode : D -> ℚ`"
(`68:205-207`), and the neighbour gap is type-level arithmetic on the numeral's own parameters. The
whole algorithm family collapses to integer arithmetic the tower already owns. Prior art, credited:
Steele & White (PLDI 1990) and Clinger (PLDI 1990) established the exact-rational formulation of
shortest printing and correct parsing respectively; Matula (1968) the digit-count bound; Loitsch's
Grisu (2010), Adams's Ryū (2018) and Giulietti's Schubfach line are per-format accelerations of the
same specification. The spec states the law; the accelerations are Kind 2 internal bench work under
`arvo-always-optimal-internals.md`, per format, later, and nothing in the law's statement depends on
them.

The law, compiled exhaustively at the model (`72_probes/probe_2_shortest_print_roundtrip.rs`): for
every one of the 1152 data, the shortest correctly-rounded significant-digit string that reparses (by
probe 1's quantiser) to exactly that datum exists, and its length never exceeds
H = ceil(log10(r^p)) + 1 = 4. The whole matrix, not a sample. The bound is **tight** at this model: 93
of 1152 data need all four digits (witness m = 206, e = -4), so the spec's H sentence is a measurement
here, not an inheritance from the literature. The digit-count distribution (24, 216, 819, 93 at k = 1
through 4) is in the outcomes file.

Both kernels are const-callable as written, which the brief demands of every mechanism:
`72_probes/probe_5_kernels_are_const.rs` re-declares the quantiser and the digit-rounding as `const
fn`, `#![no_std]`, zero gates, and closes a full parse-print-reparse round trip inside `const _: () =
assert!(...)` items (0.1 parses to 205/2048; prints to `100e-3`; reparses to 205/2048).

**The buffer is a spine-rule firing site, and I compiled both sides of it.** The print buffer's length
is a function of the numeral's parameters, and it has to appear in a type (an array length, sizes
const at type level, no alloc). The tempting spelling, `[u8; short_budget(N::RADIX, N::PRECISION)]`
inside a generic function, refuses on the pinned toolchain with `generic parameters may not be used in
const operations` and rustc's own help text naming the forbidden `generic_const_exprs`
(`72_probes/probe_3b`, expected-fail, error verbatim in the outcomes file). The rule's own answer, a
quantity that is computed and then has to appear in a type is a type: the capacity is an associated
type on the numeral (`type ShortCap: TextCap`), with a `const` assertion at each impl checking the
chosen capacity covers the computed bound at declaration time (`72_probes/probe_3`, gate-free,
`#![no_std]`). By the consolidation's count this is the **tenth firing** of the spine rule
(`68:98-101` counts nine). And the declaration-site check is not decorative: during this probe's own
authoring it refused an undersized capacity with E0080 at the impl, before any use site existed, which
is the same declaration-time-refusal dividend the review has now logged at six carriers (`68:344-354`).
A second capacity (the exact-expansion bound for the datum-keyed debug image, which for radix 2 into
decimal grows like p + |EMIN| digits rather than H) derives the same way and is stated, not probed.

*Grounded on: settled shapes (`68:205-207` decode's codomain, `68:98-101` the spine rule), compiled
(`72_probes/probe_2`, `probe_3`, `probe_3b`, `probe_5`), external prior art as cited, reasoned (the
exact-expansion bound).*

## 6. The byte image: datum-keyed, two declarations, and file 01's orphan finally picked up

The byte image's layer assignment needs no debate, because a ratified intent already forces it:
`Warm` "works and behaves as f32 and f64 etc in rust today" (`68b:62-67`), and a stored `f32` today
preserves its datum bit-exactly, signed zero, NaN payload and all. So **store/load is a crossing at
the datum layer**: `load ∘ store = id` on data, always, which is the whole point of a byte image, and
the value-level statement is then a theorem rather than a law. What the crossing contract's structure
adds is the two statements nobody writes down until a format has shipped wrong:

**The canonicalisation statement is the padding declaration.** A datum narrower than its carrier
(every `Bits<N>` with N not a multiple of 8, every bitpacked tail) has container bits that are not
datum bits, and the byte image must say what they hold. Two coherent choices exist: canonical-at-rest
(padding is zero, established at store time, one mask amortised across every subsequent read) or
declared-don't-care (every datum-keyed consumer masks, forever, and forgetting once is the probe-4
defect). The layer-keying rule's own phrasing decides my recommendation: the canonicalising projection
must be **the only door**, and canonical-at-rest makes the store that door, once, instead of asking
every reader to be one. Offered as a suggestion with its cost stated: one AND per store against one
AND per read times reads-per-write, and this stack's own `Cold` intent ("seldom computed... read far
more often than written", `68b`, `70:174-179`) makes the store side the cheap side for exactly the
columns that are bitpacked.

**The statement-0 sibling is file 01's orphaned boundary, generalised.** Bytes can arrive that decode
to no datum (an out-of-vocabulary pattern) or to a datum that is no value under the numeral (file
01's DAZ case: a loadable subnormal pattern under FTZ-as-identity, "a non-value, and something must
say what deserialising it means. Not an error; an unstated boundary", `01:332`, unanswered for
seventy-one files). This is the identical shape as the crossing contract's statement 0, one boundary
further out, and it takes the identical mechanism: a declaration-site obligation on whatever brings
foreign bytes in, blanket-safe where the byte image is one the tower generated, an explicit
declaration where the bytes are foreign, exactly `Crosses<N>`'s own safe-blanket-or-unsafe-impl
discipline (`68:250-274`). Whether it is a second trait beside `Crosses` or a widening of `Crosses`
to name its byte side is a shape call I leave open with both spellings on the table; the law is the
part I am confident of.

Endianness and bit order within a byte are `Lowering` facts with an exact slot shape already waiting
(`Encoding` nests inside `Lowering`, changes which datum carries a value and never which value,
`68:175-177`; byte order changes which bytes carry a datum and never which datum). I name the slot
and decline to design its members here: the numeral tower does not change shape either way, and the
one hard constraint is that the byte image's parameters are part of the format's identity for the
fingerprint question below.

One more spec sentence this boundary owes, cheap and easy to forget: a byte image crossing a process
boundary needs the format's identity to travel with it or be agreed out of band. The design is
unusually well placed here too, because a numeral is a closed bundle of sealed type-level parameters,
so a canonical format descriptor (radix, precision, exponent form and bounds, domain, specials,
underflow, plus the `Lowering` byte-image parameters) is a const-derivable value, not a registry. I
flag it as owed, with no mechanism proposed: it is a downstream-contract question in the
`16c` sense, what the design needs back from a transport layer for the intent to be realised.

*Grounded on: ratified (`68b:62-67`), settled shapes (`68:250-274`, `68:175-177`), tree ground as
defect context only (`01:332`), compiled (`72_probes/probe_4`, the padding half), reasoned (the
recommendation and its cost statement).*

## 7. The digest: two equalities, two digests, and the layer-keying rule's fourth instance

The design has two equalities on purpose: value equality, which is partial the moment `Specials`
carries NaN, and the datum-level total order the `TotalOrd` split established (`62b`, carried at
`68:136-139`). A digest is paired with an equality by whatever container uses it, and the consistency
law every hash container assumes (`k1 == k2` implies `h(k1) == h(k2)`) binds the pair, not the digest
alone. So the spec sentence is short: **a digest factors through the canonicalising projection of the
layer its paired equality lives at, and that projection is the only door.** A datum-keyed digest
consumes the datum (padding masked, nothing else collapsed) and pairs with the total order's
equality; a value-keyed digest consumes the class-collapsed canonical datum (NaN payloads collapsed,
cohorts collapsed to preferred member, per `Encoding::Canonical`) and pairs with value equality.
Mixing the pairings breaks the law in one direction or the other, and both breaks are compiled in
`72_probes/probe_4_digest_keys_on_a_layer.rs`: two carriers of one 13-bit datum separated by a
carrier-byte digest, and two NaN-payload data of the one NaN value separated by a datum digest paired
with value equality. Both restored by inserting the layer's projection in front of the digest, which
is the layer-keying rule's sentence verbatim at its fourth site (`TotalOrd` was the datum layer's
comparator; this is the datum and value layers' digests).

This is also the standing contract the unreviewed `arvo-pseudorand` family (`11:55`) inherits from
the tower whenever the panel or a later round takes it up: hash functions themselves are that
family's business, but *what a hash of a `Number<N, S>` consumes* is this chapter's law, decided by
the tower, not by the hash.

*Grounded on: settled shapes (`68:135-139` the layer-keying rule and the `TotalOrd` split), compiled
(`72_probes/probe_4`), reasoned (the pairing law's statement).*

## 8. Costs, binding times, and what this does not claim

Everything type-shaped here resolves at monomorphisation with zero feature gates: all six probes
compile with a bare `--edition 2024` on the pinned toolchain (compile times 0.04s to 0.33s each,
`aarch64-apple-darwin`, `-O` on the runnable ones, flags stated in the outcomes file), and the two
that must refuse, refuse with the expected errors. The parse and print kernels are const-callable as
written (probe 5), integer-only, allocation-free, with buffers sized by the numeral's own capacity
type. Runtime cost statements: the model kernels are exact-integer loops whose operand widths are
bounded by digit budget and format width; at real widths the same shapes need the multi-limb family
already adopted in `arvo-strategy`, and any per-format acceleration (Ryū-class tables for the IEEE
shapes) is bench-gated internal work, not spec. No performance number in this file is a runtime
claim about shipped code; the probes' timings are compile-and-run costs of the probes themselves,
stated so the next member can budget re-running them.

The model-instance honesty the transfer-ground scheme now requires (`68:429-471`): every compiled
claim above is established at one model instance (radix 2, p = 8, span 9) and one staging pair
(12 against 8). Per coordinate: the double-rounding witness density and the H tightness are facts
about the model instance, `unargued` elsewhere; the ToOdd staging cure carries `induction` (the
p_mid >= p + 2 argument is Boldo-Melquiond's, standard, prose); the single-rounding-equals-nearest
law carries `induction` over the kernel's own case analysis; the H bound itself carries `induction`
via Matula's argument with the model as its check rather than its proof. The container-class
coordinate (`68:496-504`) applies to these kernels as to everything else: the probes exercise the
`u128` class only, and say so.

## 9. What this hands forward

**For the next consolidation, the chapter's spine in one provenance-formed statement.** *Every
external image of a value is a crossing or a projection. Text and bytes are crossings and take the
crossing contract's statement structure: parse is the quantiser applied once to the digit string's
exact rational (staged implementations licensed only through round-to-odd intermediates at two guard
digits); display prints the shortest digit string that reparses to the same value, within the
per-format H bound; store/load is identity on data, with padding canonicalised at the store and
foreign bytes guarded by a statement-0 obligation at their declaration site. A digest is a projection
and factors through the canonicalising projection of the layer its paired equality lives at. The
debug image is datum-keyed, the display image value-keyed, completing the layer-keying rule's display
clause for computed values, which have no face.* (Grounded: ratified `68b:62-67`; settled shapes
`68:125-137`, `68:183-274`, `11:212-218`, `01:318-320`, `67:178`; compiled `72_probes/` all six;
external IEEE 754-2019 §5.12, primary read owed.)

**Open, stated rather than resolved.** The byte-level statement-0 mechanism's shape (second trait or
widened `Crosses`). The format-descriptor question, named as a downstream contract, no mechanism
proposed. The exact-expansion capacity bound, stated but not probed. The §5.12 primary-source read.
Whether parse's out-of-range behaviour under `Warm` on a no-infinity numeral simply inherits file
71's supremum rule (I believe it does, by construction, and did not probe it).

**For the exploring rhythm.** File 11's own coverage table (`11:44-56`) is the standing map of
decided-but-unreviewed ground; the constructive-extensibility compile (`22:475-479`, `26:497-498`)
is the cheapest named dispatch on it; uniform sampling over a numeral sits behind the digest law and
should follow it, not precede it.

## 10. Table-diff self-check and verification

The enumeration table in section 1 was checked row by row against the grep transcripts and the hit
contexts read for it, and each "unexamined" row states the pattern family that failed to surface it
rather than asserting a universal negative. The probe-outcome numbers quoted in sections 4, 5 and 7
were checked against `72_probes/OUTCOMES.md`, which was itself written from the probes' verbatim
output in this session, not from memory. Canon gate, test gate, and toolchain reproduced fresh at the
top of this document. Six probes compiled and run this session; the two expected refusals (probe 3b
entire; probe 3's first-compile capacity check) are reproduced with their error text in the outcomes
file. Every tree citation above is a `tree-fact` or a defect record; no shipped prose is read as
design meaning anywhere in this file, and every design conclusion survives deleting its tree
citations, which I checked sentence by sentence against file 69's one-sentence test before this
document stood.
