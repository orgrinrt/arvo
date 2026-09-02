# 252. The primitive surface: what the canon licenses, where it would sit, and why the five rows are not a cut

I was asked what the ratified canon says about a primitive surface at the positions five
`obligation` rows describe, where such a surface would sit across `mock/crates/`, and whether the
decomposition those five rows imply is the right one.

The short form, before the working. The canon **requires** a stack-owned type at every public API
position, in one `in_force` row and nowhere else. It **ratifies** exactly one part of the surface,
the coordinate set carried out of the introducing crate's door, and refuses to say how many types
that is. It is **silent** on debug output and on unstable containment, measured by grep and reported
as silence. It **reserves** two of the five to open questions whose decider is the panel, which is
not silence and may not be filled in a design. And it fixes **no crate**: the only document in this
repository that names a locus for `USize`, `Bits<N>` and `ContentHash` is a comment describing the
tree deleted on 2026-08-08.

On the decomposition, the answer is sharper than the question allows for. The five rows imply no
decomposition at all, and the namespace they live in says so in its own header. Reading them as a
cut is a category error, and it is one that has already shaped the question I was handed. What the
canon does imply is a cut, by kind of thing rather than by which consumer said it, and the five rows
sample four of its five kinds unevenly while missing the one kind the canon has actually ratified.

I also went after the one thing row five calls unmeasured, rather than reporting it. It is measured
now, in `252_probes/`, and the answer is not the one the row expects: the containment of
`generic_const_exprs` is position-dependent, the `where`-bound shape the row names by word is
contained, and the return-type shape is not, with a diagnostic that never mentions a feature.

## 0. Coverage. What I read, and what I did not

Read in full: `mock/registry/ruling.toml`, all 96 rows, by listing every `id` and reading every row
whose `topic` or keywords touch primitives, the format, the container premise, operating constraints,
binding time or the realisation map. `mock/registry/obligation.toml`, all 16 rows. `mock/registry/topic.toml`,
all 20. The three shipped crates' `lib.rs`, `mock/crates/arvo-format/src/width.rs` in full, and
`mock/crates/arvo-format/DESIGN.md.tmpl` from line 209 to the end of the door section.
`mock/crates/arvo-format/tests/ui/an_arvo_type_as_a_const_parameter.rs` and its `.stderr`.
`mock/agent/MAIN.md.tmpl`. `mockspace.toml` at the `[primitive-introductions]` block and its
preceding documentation. `mock/Cargo.toml`. `mock/tools/the-positions/src/supply.rs` header.

Read by targeted query rather than in full: `question.toml` (2025 lines), by listing every `id` and
opening the four that bear; `retirement.toml` (2217 lines), by listing every `id` matching the
primitive vocabulary and opening three; `proposal.toml` (2224 lines), by listing ids matching that
vocabulary, which returned one.

**Not read at all**, and named because a surface not named is not claimed: `law.toml`,
`law-the-later-topics.toml`, `dimension.toml`, `probe.toml` (1599 rows), `strategy.toml`,
`proposal-the-later-topics.toml`. No panel member file from this unit, with one exception noted in
section 11: I did not open `235_kiselyov_which_obligations_the_ratified_canon_supports.md`, which is
plainly the neighbouring question, and I say in section 11 what I intend to do about that and why I
did not do it first. I did not open the other reader's file on this question; none existed under
`251_` or `252_` when I listed the directory, and I list what I checked in section 1.

Not reachable from this worktree and therefore not read: hilavitkutin's, kolli's and tarina's own
design documents. Every consumer sentence I use is the `quote` field of the obligation row that
carries it, and I say so at each use, because a quotation of a quotation is a different tier from a
reading of the source.

## 1. The two standing gates

### 1.1 The canon gate: aligned, and I name what I checked it against

The work I was dispatched for is a reading of the canon, which the canon calls for: op has left the
canon work to the panel, in `ruling::the_panel_finishes_the_canon_without_him`
(`rung = ratified`, `ratified_by = op`), and `ruling::the_canon_must_support_a_full_design_and_impl`
sets the exhaustiveness bar the obligation namespace exists to answer against. Reading obligations
against the ratified rows is that bar being applied. Nothing about the dispatch asks me to fill a
reserved call, and where it would have, I have refused and said so, in section 12.

One thing in the brief is a compression I decline to inherit, and it matters enough to state at the
gate rather than in a footnote. The brief says "The `ruling` namespace holds rows ratified with the
lead designer in the loop and those govern." **That is not what the namespace holds.** Measured:
96 rows, of which 32 carry `rung = "ratified"`, 58 `rung = "stated"`, five `rung = "open"`, and one
`rung = "in_force"`. `ruling::an_ack_is_not_a_ratification` is the row that makes the difference
load-bearing, in op's own words: an opinion given before the experts converge "is only thing that
ratifies shit" for nothing, it is an ack, and it "may not be quoted later as though it did" close a
question. `mock/agent/MAIN.md.tmpl:42` states the tiering that follows: ratified rows govern,
`stated` "binds as direction and is not yet canon".

So a reading that treats all 96 as governing would have promoted 58 of op's directions and five of
his explicit deferrals into canon. I have not, and every citation below carries its rung.

### 1.2 The test gate: run, read, and one thing to say about it

`cargo test --workspace` from `mock/`, whole suite, nothing filtered: 172 passing across five
binaries, three ignored, zero failing. Bodies read rather than names counted, in the surface this
question touches, which is `arvo-format`'s public type surface and its two test files.

The suite is not decorative and I have nothing to refuse on. Specifically:

- The three ignored tests are catalogue-reds with stated gaps, not silenced failures:
  `crates/arvo-format/tests/matlab_fi_parity.rs:278` on a MATLAB rounding mode with no name in the
  ratified six, `crates/arvo-placement/src/tests.rs:164` on a converse independence the single
  shipped packing rule cannot exhibit, and
  `crates/arvo-format/src/apply/tests/the_ratio_coordinate.rs:438` on a euclidean carry. Each names
  its gap in the `#[ignore]` reason, which is the form `catalogue-edge-cases-as-tests` asks for.
- `tests/compile_fail.rs` and `tests/ui/` are eight refusals kept as tests with their diagnostics
  committed beside them. That is the strongest shape in the suite and it is the reason I could
  answer part of this question at all: `tests/ui/an_arvo_type_as_a_const_parameter.stderr` is a
  committed measurement of the exact seam between two of the five obligations.
- The doctests split deliberately between four that run and five `compile_fail`, and
  `arvo-format/DESIGN.md.tmpl:251` explains the split by where an obligation is forced, which is a
  real distinction rather than redundancy.

**One thing I will not let pass as clean.** The suite has no test anywhere over the property this
question is about. Nothing asserts that a public API position carries a stack-owned type, nothing
asserts that the 27 host-typed public positions are confined to constructors and accessors, and
nothing asserts that a consumer needs no feature gate. The first two are measurable today by a tool
that already exists, and the third I measured in `252_probes/`. Whether any of them may be asserted
as a lint is section 12, and the answer is not simply yes.

## 2. The brief's own claims, checked before I used them

Both check out.

`grep -rl 'USize' mock/crates/*/src` returns nothing, and so does the same grep over the whole of
`mock/crates/`. Positive control on the instrument: the same command with `Width` returns eleven
files, so the command can produce a non-zero, and the wider `the-positions` walk reports `USize`
under "on the list and reaching nothing", an independent instrument reaching the same zero.

Each of the five ids appears exactly once in `obligation.toml`, by `grep -c` per id: 1, 1, 1, 1, 1.

## 3. What the ratified canon says

### 3.1 The demand exists, it is one row, and it is the widest thing in the registry

`ruling::the_operating_constraints_are_intents_and_rules`, `rung = "in_force"`, `kind = "intent"`,
`topic = "operating_constraints"`, `key = "I14"`. Its `says` lists the constraints and ends: "public
API positions using the stack's own primitives rather than bare integers, floats, bool or usize.
They are already in place, enforced by the mockspace lints and the workspace and repo rules, and they
are not to be questioned."

That single clause is the licence and the demand at once. It is the only row in `ruling.toml`
containing the word `usize`: one hit, at that line. There is no second statement of it anywhere in
the canon.

**One honest qualification, and it is not a small one.** Op's `quote` on that row does not contain
the primitives clause. He said "They are very explicitly also arvo intents and rules... No std, no
alloc, all that is explicitly already in place and not to be questioned." The clause reaches the
`says` because it was in the list of constraints he was answering about. Under
`ruling::the_intent_is_not_every_clause_of_the_quotation` the `says` is where the intent-naming act
lives and is what carries authority, so I take the row as it stands and build on it. But a reader
should know that the sentence every downstream argument leans on is the panel's rendering of a list
he confirmed wholesale, not a sentence he wrote. `mock/tools/rulings-with-no-verbatim` exists for
precisely this class and I did not run it against this row; that is an unspent check and I name it in
section 10.

### 3.2 One part of the surface is ratified outright, and it is not one of the five

`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
`rung = "ratified"`, `ratified_by = "experts"`, `topic = "operating_constraints"`. Its `says`:
"What the door carries out is the coordinate set of the ratified parameterisation, spelled in types
the stack owns. **How many types that is, this ruling does not say**, because the two derivations
disagree about the count and neither the canon nor a third reading settles it."

Three things follow and all three bear on the question.

**A stack-owned type surface is not merely permitted, part of it is required by a ratified row.** The
coordinates a format is declared with must be stack-owned types, and an outside crate has to be able
to write them. That is a primitive surface, ratified, in the strongest tier this registry has short
of op's own stamp.

**The count is reserved.** Seat 238 answers one type per coordinate, seat 239 more than two and fewer
than ten, and the ruling refuses to pick because two instances agree about the intersection of their
claims and never the union. So "how many primitives" is not answerable from the canon and I do not
answer it.

**The two-type bound is dead and one shipped file has not been told.**
`mock/crates/arvo-format/src/width.rs:14` reads "Two types and no more. A count of bits, and a truth
value." The ratified row says that bound "is a sentence in the crate's design and appears nowhere in
the canon", and the crate's own design has already been corrected:
`mock/crates/arvo-format/DESIGN.md.tmpl:339` now reads "They are not the bound on what the door
carries. A count of types is not the bound at all." The source comment contradicts both the ratified
ruling and the design directly above it. It is one site; I grepped the whole tree for the sentence
and it appears nowhere else. Section 7.1.

### 3.3 Two ratified rows that refuse readings of a primitive surface

**No single uniform layer.** `ruling::arms_over_regions_are_the_fundamental_heart`,
`rung = "ratified"`, `ratified_by = "both"`: the space is filled "by small arms and spans that hold
where they are optimal and nowhere else, composed, rather than by one or a few general statements
uniform across the dimensions", each carrying a const predicate over its region. Op's own words on
it are "It's the fundamental heart within arvo."
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` says the same thing negatively and is
`rung = "stated"`, so I lean on the ratified one. A design that answers "what is arvo's primitive
surface" with one uniform layer of types is the shape this refuses. It does not refuse a set of
primitives; it refuses a single rule generating them.

**Nothing is stated over a machine carrier.**
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`, `rung = "ratified"`,
`ratified_by = "experts"`: "every operation the design declares is a function of the declared width
and never of the machine carrier, so arithmetic and encoding are stated over the declared width. The
footprint is nonetheless observable, through exactly one observation the design does not own, the
host language's layout observation on a sized type, and that observation exists only where the value
is the sole logical occupant of its allocation."

That is the sharpest instrument in the canon for this question and section 5.4 turns it on row two.

### 3.4 Where the canon is silent, measured rather than asserted

An absence claim is a claim about a place, so here is the place and the pattern.

Over `mock/registry/ruling.toml`, case-insensitive:

- `fmt`: **zero hits.** No row mentions formatting at all.
- `nightly`: **zero hits.**
- `feature gate`: **zero hits.**
- `generic_const_exprs`: **zero hits.**
- `Debug`: twelve hits, every one of them inside `ruling::the_overflow_panic_is_permitted_and_bounded`
  and `ruling::the_panic_bound_names_a_concern_not_a_marker`, and every one of
  them the word "debug build"
  rather than debug output. None is about debug
  output.
- `adt_const_params`: two hits, both inside the door ruling's `promotion` and `keywords`, and both
  are a measurement about which gate name refuses which shape rather than a rule about containment.

Positive control on those greps: the same instrument over the same file returns 1 for `usize`, 96
for `^id = `, and the expected rows for `primitive`. It can produce a non-zero.

So: **the canon says nothing about debug output from a numeral, and nothing about whether arvo's
unstable machinery reaches a consumer.** Silence, and per `mock/agent/MAIN.md.tmpl` silence is not
permission.

### 3.5 Where the canon is not silent but has reserved the call

Two of the five are governed by open questions whose `decider` is `panel` and whose `answered` field
is empty. Verified by `cargo mock query 'question.where(id=...).select(id,decider,answered)'`.

- `question::what_a_platform_width_type_is` (asks: "What kind of thing is a platform-width type?").
  Four options, `decider = panel`, unanswered. Its own keywords name `USize` and `Cap`. This
  governs `obligation::a_platform_sized_unsigned_integer_at_an_api_position` and nothing else does.
- `question::the_width_surface_crossing` (asks what the crossing is between a consumer's written
  width literal and the type system's representation of it). Seven options, `decider = panel`,
  unanswered. Its `bound` field quotes the very obligation row this dispatch is about and reads it
  as governing. This governs the "alias and pin" half of
  `obligation::an_exact_width_container_a_consumer_can_alias_and_pin`.
- `question::arbitrary_width_demands_in_the_canon` is also open and adjacent.

By contrast `question::what_the_numeric_introduction_door_may_carry_out` **is** answered, by the
ratified door ruling. So of the questions in this neighbourhood, one is closed and three are open,
and the closed one is the one that is not about any of the five rows.

**This is the difference between silence and reservation and it decides what may be done next.** A
silent canon leaves a design free to derive inside the intent, put it through two expert agreements
and proceed. A reserved question does not: it is the canon holding a call open, and filling it inside
a design is drift wearing a derivation's clothes.

## 4. Where the surface would sit

### 4.1 The canon fixes no crate, and fixes one boundary

`grep -in "layer" mock/registry/ruling.toml` returns one hit and it is about reference material, not
architecture. `grep -ic "crate"` returns sixteen, of which none states a crate layout: the nearest is
`ruling::arvo_is_a_library_and_the_value_composes_on_top` (`rung = "ratified"`, `ratified_by` unset,
ratified by op in the second round), whose `says` names "the algorithm crates that every downstream
repository uses" as the selling point and settles nothing about where anything lives.

So the canon fixes no crate, no count of crates, no naming and no layering.

What it does fix is a **boundary**, and the boundary is the useful thing.
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` splits declared-
signature facts from placement facts, and puts the footprint on the placement side with an observability
condition attached. The shipped tree already realises that split as `arvo-format` against
`arvo-placement`, and `mock/crates/arvo-placement/src/lib.rs:6` states it in the crate's own words:
"Where the bits sit. Nothing about what they mean."

**That gives a locus rule that survives a rename**, which a crate name does not. Each need lands on a
side of the ratified boundary, and the crate owning that side is where it goes, whatever that crate
is eventually called. I use that below rather than naming crates as if the names were canon.

### 4.2 The only document that names a locus describes a deleted tree, and it is still on disk

`mockspace.toml:500-508` carries a block headed "Layer hierarchy in arvo" which reads, in the
present tense:

    - `arvo` (L0) - defines the numeric substrate: `UFixed`, `IFixed`,
      `FastFloat`, `StrictFloat`, `USize`, `Cap`, `Bool`, strategy markers.
    - `arvo-bits` (L1) - supplements the numeric substrate with opaque-bit
      containers: `Bits<N>`, `BitPrim`, `BitWidth`, `BitAccess`.
    - `arvo-hash` (L2) - supplements the numeric substrate with hash
      identity: `ContentHash` (alias of `Bits<28>`), plus hasher contracts.
    - Every other crate (`arvo-bitmask`, `arvo-graph`, `arvo-sparse`,
      `arvo-comb`, `arvo-spectral`) is a consumer of the substrate

Eight of those nine crates do not exist. `mock/Cargo.toml` names three members and none of them is
on that list. The `[primitive-introductions]` table twenty lines below has already been repaired and
its repair comment at `mockspace.toml:1409` explains why, naming the stale entries it removed and
noting that one of them silently handed a live crate a dead crate's lint exemption. **The prose above
the table was not repaired with it.**

This matters more than a stale comment usually does, because it is the only place in the repository
that says where `USize`, `Cap`, `Bits<N>` and `ContentHash` live, and three of the five obligation
rows are about exactly those names. `mock/agent/MAIN.md.tmpl:22` says of that tree: "The architecture
git remembers is a dead tier. Do not reconstruct it, do not reason from it, and do not treat a name
that appears in git history as a name the design still uses." The comment is not git history. It is
in the working tree, in the file every tool reads, in the present tense. Section 7.2.

Note the tier: `mockspace.toml` is **not** canon. `canon_paths = ["mock/registry/*.toml"]`, so the
block above binds nothing. It is config, and it is wrong, and it reads as authoritative because of
where it sits.

### 4.3 Placing each of the five against the ratified boundary

**`a_primitive_for_every_position_a_bare_number_would_take` has no locus, because it is not a type.**
It is a rule quantified over positions. Its home is the lint configuration plus every crate, and its
current realisation is `mockspace.toml:1418`, `arvo-format = ["numeric"]`, which exempts one crate
entirely. The exemption is crate-scoped where the rule is position-scoped, which
the `bound` field of `question::what_the_numeric_introduction_door_may_carry_out` already names as
the gap that question sat in, and which the door ruling
answered only in part: it killed the two-type bound and left the positive shape open.

Measured now, with `mock/tools/the-positions` at `2ca4bc9aa44820dd1fa9ee12569c641ec20c8311`:

    130435 occurrences examined, 27 at a public API position, all 27 in arvo-format
    by kind:    fn-param 14, fn-return 13, every other kind 0
    by carrier: scalar 27, every other carrier 0
    by role:    count 11, rational 6, index 4, exponent 2, radix 2, truth 2

I listed all 27 individually by role and **every one of them is the constructor or the accessor of a
type this crate itself defines.** `Width::bits` and `Width::count` at `width.rs:38` and `:48`,
`Bool::of` and `Bool::get` at `:104` and `:112`, `Radix::of`/`base`, `Exponent::of`/`power`,
`Magnitude`, `MagnitudeCount`, `Slot`, `SlotCount`, `Arity`, and the two-argument constructors and
paired accessors of `Phase` and `Fraction`, which is where the count goes odd.

That is worth stating plainly because it is a result about the shipped tree rather than about the
rule: **the crate-wide exemption is currently doing no work beyond the position rule seat 238
proposed.** Zero positions in the shipped tree use the exemption for anything else. The exemption is
wide and its use is narrow, and nothing asserts that.

**`an_exact_width_container_a_consumer_can_alias_and_pin` splits cleanly across the ratified
boundary and needs no new concept.** Its three parts land in three places. The 28-bit declared
signature is a format, expressible today as a point of the shipped parameterisation. The carrier and
the masked cast to 32 bits are placement facts, which is `arvo-placement`'s side by the ratified
boundary, and the value is at sole occupancy so the footprint is observable, which is exactly the
condition `behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` attaches. The alias
and the boundary conversion are the consumer's own, by
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`, whose `instead` reads "The
consumer defines their own APIs in arvo's shapes and generics, and that is where external data
becomes typed."

That ruling is `rung = "stated"`, not ratified, and it is the **only** ruling in the whole registry
carrying an `obligation` edge, and it points at this row. Its `note` also records that
op was asked to bless it and declined in terms, that the promotion is blocked, and that the block
cannot be cleared by asking him because he has left. So the one edge the canon has to the demand side
hangs off a row that is explicitly stuck.

**`a_platform_sized_unsigned_integer_at_an_api_position` has no locus derivable today**, because
`question::what_a_platform_width_type_is` is open and reserved. Section 5.4 narrows it; it does not
close it.

**`debug_output_from_every_numeral_shape` is a capability over a concept, so it sits with the
concept**, which under `ruling::the_trait_contract_structure_is_a_primary_paradigm` (`rung = "stated"`)
means a contract rather than a set of impls. Its need spans all three shipped crates ("every numeral,
at every width and under every strategy"), so a contract stated over the format concept in
`arvo-format` and implemented outward is the shape the boundary allows. Partially met already and
nowhere recorded: the three crates carry eighteen `derive` attributes between them and every one of
the eighteen includes `Debug`, so every type that derives anything is printable. Twenty-five of the
thirty-nine `pub struct` declarations carry no derive at all, which is the marker types, the format
points and the carrier markers, and those are not printable. So the coordinate layer is covered, the
marker layer is not, and the numeral layer has nothing to cover yet, there being no numeral. The
row's own no-alloc, caller-supplied-buffer half is met by none of it and is addressed nowhere.

**`the_unstable_machinery_does_not_reach_a_consumer` has no locus, because it is not a type either.**
It is a property of every crate. Measured: zero `#![feature(...)]` attributes in any `.rs` file under
`mock/crates/`, with the positive control that the same pattern matches 369 files elsewhere in
`mock/`, 27 of them under `mock/research/sketches/`. So the property currently holds in the shipped
tree, vacuously, and section 6 is about whether it can keep holding.

## 5. The decomposition, which is the part of the question I decline to answer as asked

### 5.1 The five rows are a demand-side sample, and their own namespace says so

The header comment of `mock/registry/obligation.toml`, standing above its first row, is the
namespace's own account of itself and it is unambiguous:

    What arvo is asked for, in the asker's own terms.

    Read from outside the canon on purpose: a check that walks the canon can only
    report that the canon agrees with itself, so the enumeration has to come from
    somewhere the canon does not reach.

and

    An absence here means nobody has enumerated it, never that arvo does not owe
    it. `184` says exactly what was read.

So the rows are what consumers said, in consumers' words, gathered by reading consumer documents.
They carry no claim to completeness, no claim to disjointness, and no claim to partitioning
anything. **Five of them are not a decomposition and were never proposed as one.** Treating them as
the cut of a primitive surface is the reading the header exists to prevent, and it is the framing the
question I was handed carries.

I say this rather than answering around it because a well-answered wrong question launders the drift
it was built on. What I can answer, and do below, is what cut the canon implies and where each of
the five falls in it.

### 5.2 They are not five of a kind, and the coverage instrument cannot tell

Sorted by what kind of thing each is:

| Row | Kind of thing |
|---|---|
| `a_primitive_for_every_position_a_bare_number_would_take` | a rule quantified over positions |
| `a_platform_sized_unsigned_integer_at_an_api_position` | one type for one role |
| `an_exact_width_container_a_consumer_can_alias_and_pin` | one type plus a conversion at a boundary |
| `debug_output_from_every_numeral_shape` | a capability over a concept |
| `the_unstable_machinery_does_not_reach_a_consumer` | a property of the implementation |

Four kinds across five rows, and `mock/tools/obligation-coverage` measures all of them with one
instrument. Run per slug, it reports `tier: nothing` for four of the five and `tier: stated` for the
container row, named by the stuck ingest ruling.

**For row one that reading is false in substance.** `ruling::the_operating_constraints_are_intents_and_rules`
states that need almost word for word and carries no `obligation` edge, because only one edge exists
in the entire `ruling.toml` (line 997). So the coverage figure for the widest obligation in the
registry is an artifact of a missing edge rather than a measurement of the canon, and it reports the
canon as further from met than it is. Section 7.3.

I checked whether such an edge would even be admitted: `mock/lints/an_obligation_edge_comes_from_a_tiered_namespace.rs`
tiers `ruling` as a namespace that meets an obligation, so an edge from an `in_force` ruling is
legitimate and simply absent.

### 5.3 Rows one and five are one constraint seen from two sides, and neither row knows it

This is the finding I would keep if I could keep only one.

Row one carries op's own exception, quoted in its `quote` from a design round of `kolli`: "No bare
usize other than in const generics for smoother and more ergonomic api, and even there, only when
truly painful otherwise." The row's `why` reads that exception as ergonomics, twice bounded.

**It is not ergonomics. It is what makes row five satisfiable.**

`mock/crates/arvo-format/tests/ui/an_arvo_type_as_a_const_parameter.rs` puts `Width`, a stack-owned
type, at a const generic parameter position, and its committed `.stderr:8` is the compiler
demanding `#![feature(min_adt_const_params)]` **in the declaring crate**. The test's own comment says
it outright: "the escape exists, it is a gate, and a crate declaring a format of its own would have
to carry it too." Meanwhile an associated const takes any type at all, which is why seat 238's
ten-coordinate existence proof compiles gate-free, recorded in the door ruling's `promotion`.

So at every position except one, rows one and five are satisfied together: put the stack's type
there and nothing needs a gate. At the const generic parameter position they are in direct conflict:
the stack's type demands a gate in every crate that writes it, and row five forbids exactly that.
**Op's exception resolves the conflict by choosing row five**, and neither row's `gap` field mentions
the other. Row five's gap says the containment question "is unmeasured" while a measurement of the
adjacent case is committed in the same crate as the subject.

One correction travels with this, from the door ruling's own `promotion`: `min_adt_const_params` is
the wrong gate name, since it refuses the crate's real `Width` for having a private field on a public
struct, and `adt_const_params` accepts the identical source. The compiler's `help` line names the
wrong one. Either way it is a gate on the declaring crate, so the seam is where I say it is; the
spelling in the diagnostic is not the spelling of the fix.

**And I have to take part of this back, because the registry already had it.** The `bound` field of
`question::what_the_numeric_introduction_door_may_carry_out` states both halves: that row one "is
position-scoped and its exception is the const generic parameter", and that row five "bounds the
second option: whatever the crate holds internally, a consumer naming arvo's types carries no gate
of its own". So the connection between the two rows is recorded, and I found it because a lint
refused a line citation I had written into that file and made me open the row. What is mine is
narrower and is the part that field states as a bound rather than as a measurement: **that sentence
about a consumer naming arvo's types was an assumption, and section 6 measures it.** It holds for
`adt_const_params` and it does not hold for every shape of `generic_const_exprs`. The rows' own
`gap` fields still say none of this; the question's `bound` field does.

Section 6 measures the side of the seam that was assumed rather than measured.

### 5.4 Row two states a mechanism its own `why` disowns, and the mechanism is the one the canon most nearly forbids

The row, in full where it matters:

    need = "An unsigned integer of the platform's size, usable in a public error
            type, whose range covers both a unix errno and a Windows
            GetLastError value with no negative case."
    why  = "The consumer states the property rather than the type: non-negative
            across both platforms' ranges."

The `why` says the consumer gave a property. The `need` leads with a type. That is an internal
contradiction inside one row, checkable from the row alone, and it is the same defect
`obligation::a_build_flag_that_changes_float_semantics` has and diagnoses in its own `gap`: "This row
used to state the mechanism... which the field description forbids in as many words: the need, never
the mechanism that serves it."

Where the mechanism came from is not mysterious. The row's `quote` is the consumer writing
"OS error codes are carried as `arvo::USize` values", and `arvo::USize` is a name from the tree
deleted on 2026-08-08, still described in the present tense at `mockspace.toml:500`. The row
abstracted the consumer's sentence and kept the consumer's type.

**And the ratified canon bears on the mechanism directly.**
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` says "every operation
the design declares is a function of the declared width and never of the machine carrier". A
platform-sized type has no declared width: its width is a fact about the target. So if such a type
were admitted as a **format**, every operation over it would be a function of a machine fact, which
is the thing that row forbids.

I stop short of calling that a refutation, and the hinge is exactly one word.
`ruling::the_format_spine_is_canon` says a format's representable set "is a constant of the type".
Within one compilation a platform-sized type's set is constant; across targets it is not. Whether
"constant of the type" is read per-compilation or per-declaration decides whether option two of
`question::what_a_platform_width_type_is` ("a degenerate instance of the shape family") survives.
**The canon does not say which reading, and I am one expert.** So this narrows Q26 and does not close
it, and section 12 records it as an option with what would close it.

What I will say without hedging: **the row's `need` should not carry the words "of the platform's
size"**, because its own `why` says the consumer did not state a type, because the type it names is
from a dead tree, and because it prejudges an open question the canon reserves to the panel. The
property, stated without the mechanism, is a non-negative integer whose range covers both platforms'
error codes. What width that actually requires is a question I could not answer from this worktree,
since hilavitkutin is not cloned here; it is an option in section 12.

### 5.5 The cut the canon implies

Not by consumer, and not by which document happened to be read. By what kind of thing it is, which
is the axis the ratified rows are already stated along:

**Coordinate types.** What a declaration is written in: a count of bits, a radix, an exponent, a
magnitude and a count of them, a slot and a count of them, a phase, an arity, a fraction, a truth
value. Governed by `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
ratified, with the count reserved. Locus: the introducing crate's door.

**Numerals.** Values of a declared format. Governed by `ruling::the_format_spine_is_canon`, ratified
by both, which makes the surface a closed concept with an open inventory. So the numeral surface is a
trait an outside crate implements, not a list of types arvo ships, and "which primitives exist" is
the wrong question about this tier by construction.

**Placement facts.** Carrier, access width, stride, occupancy. Governed by
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`, ratified.

**Position rules.** Where a bare primitive may appear at all. Governed by
`ruling::the_operating_constraints_are_intents_and_rules`, in force, with the shape of the bound
open and one proposal standing at one expert
(`proposal::the_introduction_doors_bound_is_a_position_rule_not_a_count_of_types`).

**Build properties.** What the implementation may reach for, and what escapes. Canon silent.

Five kinds. The five obligation rows land as: one position rule, one placement-plus-format compound,
one type whose kind is reserved, one capability that belongs to no kind above, and one build
property. **They sample four of the five and miss the one the canon has actually ratified**, which is
the coordinate set. `grep -in "coordinate" mock/registry/obligation.toml` returns zero.

That miss is not a defect of the namespace, and I want to be exact about it rather than scoring a
point. The namespace is demand-side by design, and no consumer asked for a coordinate set. It is a
defect of **using the demand side as a decomposition**, which is what the question does: the axis
along which the canon is organised is invisible from there, so any cut derived from those rows will
be missing whatever no consumer happened to name.

## 6. The measurement. Does arvo's unstable machinery reach a consumer

`obligation::the_unstable_machinery_does_not_reach_a_consumer` says of its own subject: "Whether a
`generic_const_exprs` bound in a public signature can be hidden from a consumer at all is exactly the
open question, and it is unmeasured." Reporting that back would be a blocker reported and left, so I
measured it.

`252_probes/`, committed with sources, four run scripts and captured output under `252_probes/out/`.
Every arm is a two-crate compile driven by `rustc` directly, library first with whatever feature it
needs, then a consumer crate carrying no feature attribute of any kind. Every arm has a control that
must fail or must build, because an arm with no failing case is not an instrument.

### 6.1 What each arm returned

**Arm A, a const expression in a public return type: not contained.** The ungated consumer is
refused, `out/a_generic_const_exprs.txt`:

    error[E0308]: mismatched types
       expected `4`, found `leaky::::widen::{constant#0}`
       help: call `Into::into` on this expression

The control in the same crate, a consumer naming a signature with no const expression in it, builds.
So the dependency is consumable and the failure is about the expression. The discriminating control
`A2` is the identical consumer with `#![feature(generic_const_exprs)]` turned on in the **consumer**,
and it builds, `out/d_discriminating_controls.txt`. That is what attributes the failure to the absent
feature rather than to unevaluated consts never normalising across a crate boundary for anybody.

**The diagnostic never mentions a feature.** It names an opaque internal constant and suggests
`.into()`. A consumer hitting this has no signpost to the cause, which is worse than a gate error
rather than better.

**Arm A3, a const expression in a public `where` bound: contained.** This is the shape the obligation
row names by word, and arm A was not it, so I built it. The ungated consumer builds,
`out/e_bound_shape.txt`. Its own control, `out/f_bound_is_checked.txt`, implements the bound's trait
at exactly three widths: the ungated consumer at `6 + 7 = 13` builds and the one at `6 + 6 = 12` is
refused with

    error[E0277]: the trait bound `Bits<12>: Small` is not satisfied

in a consumer with no feature attribute. The diagnostic says `Bits<12>`, so the expression normalised
there and the bound was genuinely checked. Without that control the clean build had two readings and
no way to choose.

**Arm B, a stack-owned type at a const generic parameter, instantiated: contained.** A consumer with
no feature attribute writes `Signed<{ Width::bits(13) }>` and builds, `out/b_adt_const_params.txt`.
The forcing control proves it was evaluated rather than merely accepted: asserting `DECLARED == 13`
builds and asserting `== 12` fails with `error[E0080]: evaluation panicked: assertion failed:
Thirteen::DECLARED == 12`.

**Arm C, the coordinate set at associated consts: no feature anywhere.** A door crate with zero
feature attributes exposes a contract whose three coordinates are stack-owned types at associated
consts, and an outside crate implements it at radix 3 and exponent -4, naming no machine type in any
declaration, and forces it through a `const` item at check time. Both build,
`out/c_assoc_const.txt`. The negative control writes `Width::bits(-4)`, the value a `u32`-shaped
door cannot hold, and is refused with `error[E0600]: cannot apply unary operator to type u32`.

### 6.2 What that establishes, with its region

    holds for: toolchain = nightly-2026-05-28 (rustc 1.98.0-nightly 57d06900f),
               edition = 2024, crate_type = lib, linkage = rlib, std = none,
               shapes = {const expr in fn return type, const expr in where bound,
                         ADT const generic parameter instantiated from outside,
                         coordinate set at associated consts}

A compile-time refusal, so no runtime dimension is claimed and none is implied. Anything not listed
is not claimed: trait associated types, a const expression in a struct field type, an impl-trait
return position, a const expression reached through a macro, and anything under cargo feature
unification are all unmeasured and I say so rather than leaving them to be assumed either way.

Three results, and the middle one is the useful one:

1. **`adt_const_params` is contained at the declaring crate.** Using arvo's declaration costs a
   consumer nothing. Declaring one's own costs the gate, per the crate's own committed ui test. So
   `obligation::the_unstable_machinery_does_not_reach_a_consumer` is satisfiable at every naming
   position and at risk only at the declaring position, which is precisely the position op excepted.
   kolli's sentence, quoted in that row, says "a crate **naming** these types needs no feature gates
   of its own", and naming is the half that is safe.

2. **`generic_const_exprs` containment is position-dependent, and the row names the safe shape.** A
   `where` bound is contained; a const expression in a type the caller has to name is not. So the row
   as worded asks about the case that holds and the case that fails is next door with no row about it.

3. **The coordinate set at associated consts needs no unstable feature anywhere**, in either crate.
   This is a third independent instance of the door ruling's existence proof, built without opening
   seats 238 or 239, and it agrees with them.

### 6.3 What I did not do with this, and why

I did not write any of it as a lint or a test in the repository, and under
`a-hand-check-becomes-a-test-every-time` that needs a reason rather than an omission.

The reason is the chain. `the-canon-design-code-chain` says nothing may appear in code that the
design does not say, and `mock/crates/arvo-format/DESIGN.md.tmpl:357-360` names the position rule this
would enforce as "**Named here as open rather than adopted**, because this design is not where that
bound gets chosen." A lint enforcing an unadopted bound is an undeclared design change wearing the
leaf tier's freedom. The probes are evidence and they are committed as evidence; turning them into a
gate is design work, and section 12 records what would license it.

## 7. Findings outside the question, stated plainly

### 7.1 A shipped source file contradicts a ratified ruling

`mock/crates/arvo-format/src/width.rs:14`: "Two types and no more. A count of bits, and a truth
value."

`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`, ratified:
"The bound the question inherits, that the door is two types and no more, is a sentence in the
crate's design and appears nowhere in the canon."

The design directly above that source has already been fixed and now says the opposite
(`DESIGN.md.tmpl:339`). The source comment was not fixed with it. One site, grepped across every
`.rs`, `.tmpl` and `.md` outside `mock/research/`. It is a doc comment, so it ships in the rustdoc a
consumer reads, which makes it a stale claim on a public surface as well as a contradiction of the
canon.

### 7.2 The config file describes a crate tree that was deleted, in the present tense

`mockspace.toml:500-508`, quoted in section 4.2. Eight of the nine crates it names do not exist. It
is the only document in the repository that assigns a locus to `USize`, `Cap`, `Bits<N>`,
`BitPrim`, `BitWidth`, `BitAccess` and `ContentHash`, which is to say the only document that appears
to answer half of the question I was dispatched on, and it answers it about a tree that is gone.

The table twenty lines below it was repaired and its repair comment explains that a stale entry there
had silently handed a live crate a dead crate's lint exemption. The same fix pass did not touch the
prose. `mock/tools/the-positions` independently reports every one of those type names under "on the
list and reaching nothing", so an instrument in this repository already knows they are gone.

### 7.3 The demand side has one edge, and the widest obligation is not on it

`grep -c '^obligation = '` across `mock/registry/`: `ruling.toml` 1, `proposal.toml` 8,
`proposal-the-later-topics.toml` 3, `retirement.toml` 1, everything else 0.

One edge from the canon's governing namespace, on
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`, and it hangs off a `stated` row
whose own `note` records that its promotion is blocked and cannot be unblocked by asking op.
`ruling::the_operating_constraints_are_intents_and_rules` states
`obligation::a_primitive_for_every_position_a_bare_number_would_take` almost word for word and has no
edge, so the coverage tool reports the canon's widest constraint as met by nothing.

The lint that guards this class, `an_obligation_edge_comes_from_a_tiered_namespace`, guards against an
edge from an untiered namespace. Nothing guards against an absent edge, which is the failure that
actually happened, and which reports in the direction of understating coverage.

### 7.4 One instrument reads zero for a reason that is not a defect, and could be read as one

`mock/tools/the-positions '.@HEAD' --api-only` prints "0 naming an arvo crate or type" and an empty
"in use" list, which reads as though no stack-owned type appears at any public position. Run without
`--api-only` the same tool reports 347 resolving, `Width` at 24 uses and `Bool` at 35. The flag
restricts both halves of the fraction to positions where a host primitive sits, where the supply is
zero by construction.

I do not call this a defect, because the tool's `--api-only` mode is answering a narrower question
correctly. I record it because I nearly cited the zero, and a reader taking that line out of an
`--api-only` run would report the obligation as at zero supply when it is not.

## 8. What I settled

**That the canon licenses and in fact requires a stack-owned type at public API positions**, on one
`in_force` row, with the qualification in section 3.1 that the clause is the panel's rendering of a
list op confirmed wholesale rather than a sentence he wrote.

**That one part of the surface is ratified outright and is not among the five rows**: the coordinate
set carried out of the introducing crate's door, in types the stack owns, with the count reserved.

**That the canon fixes no crate and one boundary**, and that the boundary rather than the crate names
is the durable locus rule, since it survives a rename and the names are not canon.

**That the canon is silent on debug output and on unstable containment**, by grep with a positive
control, and reserved rather than silent on the platform-width type and the width crossing.

**That the five rows are not a decomposition**, on their own namespace's stated construction, and
that the cut the canon implies runs along a different axis, which section 5.5 states.

**That rows one and five are one constraint seen from two sides**, that op's const-generic exception
is what resolves their conflict, and that the conflict lives at exactly one position.

**That `adt_const_params` is contained at the declaring crate and `generic_const_exprs` containment
is position-dependent**, measured in `252_probes/` with a discriminating control per arm, over the
region stated in 6.2.

## 9. What I moved

**Row five's own open question, from unmeasured to measured**, in the direction that shows the row
names the safe shape. Its `gap` should now read that the `where`-bound case is contained, that the
return-type case is not, and that the diagnostic in the failing case never mentions a feature. That
correction goes in this deliverable and reaches the registry through consolidation, per
`every-finding-carries-its-predicate`; I have not edited the row.

**The door ruling's existence proof, from two instances to three.** Arm C is a coordinate set at
associated consts declared from outside with no feature attribute anywhere, built without opening
either prior seat's file.

**The reading of op's const-generic exception**, from ergonomics to load-bearing. Row one's `why`
calls it ergonomics and twice bounded. It is also the only thing standing between row five and a gate
in every consumer that declares a format. The registry had the connection already, in a `bound` field
rather than in either row; what moves is that the safe half of it is now measured rather than assumed.

**The claim that nobody has enumerated the positions.** Row one's `gap` says "it is satisfied by the
positions, and nothing has enumerated those". `mock/tools/the-positions` enumerates them and I ran
it: 27 in the shipped tree, all in one crate, all constructor or accessor positions of that crate's
own types. The row's gap is stale in that clause.

## 10. What I could not

**I could not settle how many primitives the surface is**, and I did not try, because the ratified
door ruling reserves the count and says the two derivations disagree.

**I could not settle what a platform-width type is.** I narrowed it, in 5.4, to a hinge on one phrase
in `the_format_spine_is_canon`, and the canon does not disambiguate that phrase. It is reserved to
the panel and one expert reading is not a ratification.

**I could not establish what range row two's need actually requires**, because hilavitkutin is not
cloned in this worktree and I would only have been reading the row's own quote of it a second time.
The structural contradiction inside the row stands without that; the width does not.

**I did not run `mock/tools/rulings-with-no-verbatim`** against
`the_operating_constraints_are_intents_and_rules`, which is the tool built for exactly the
qualification I raise in 3.1. Unspent check, named rather than quietly skipped.

**I did not read `law.toml`, `dimension.toml`, `probe.toml`, `strategy.toml` or either
`*-the-later-topics.toml`.** A finding in any of them about primitives would be invisible to this
file, and `probe.toml` at 1599 rows is the one where that seems most likely.

**And I have not reconciled against `235_kiselyov_which_obligations_the_ratified_canon_supports.md`**,
which is plainly the neighbouring question, written by the persona I am. I left it closed on purpose:
the dispatch asked for an independent derivation committed before I read anything by the other reader
on this question, and reading a same-persona file on the adjacent question first would have made
"independent" a word rather than a property. The honest consequence is that this file may restate or
contradict it. Reconciling is the next act and it belongs in an appendix to this file rather than
inside it, so the commit ordering stays checkable.

## 11. Carried forward unchanged, and from whom. Count: ten

Ten things I use and did not re-derive, each named with where it came from, because a claim I
inherited and a claim I established are different tiers and a reader cannot tell from the prose.

1. **The four format-spine propositions**, from `ruling::the_format_spine_is_canon`, ratified by both.
   Used in 5.5 to say the numeral tier is a closed concept with an open inventory.
2. **The container-premise dissolution and its four propositions**, from
   `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`, ratified by
   experts, promoted on seats 210 and 225. Used as the boundary in section 4 and as the instrument in
   5.4. I did not re-run its MATLAB sweep or its packed-column instrument.
3. **That the door carries the coordinate set and the count is open**, from the door ruling, ratified
   by experts on seats 238 and 239.
4. **That `min_adt_const_params` is the wrong gate name for the shipped `Width` shape**, from that
   ruling's `promotion`. I did not verify it; I used it to shape arm B's `Width` reproduction, giving
   it a private field on purpose, and I say so in the probe README.
5. **That three of the ten constants carry values `Width` cannot hold**, same `promotion`, corrected
   there from an earlier draft's six. Arm C's negative control is consistent with it at one constant
   and is not a re-derivation of the three.
6. **The arms-over-regions shape**, from `ruling::arms_over_regions_are_the_fundamental_heart`,
   ratified by both.
7. **The tiering of `rung` values**, from `mock/agent/MAIN.md.tmpl:42-47`, which is generated agent
   output and not canon, cross-checked against `ruling::an_ack_is_not_a_ratification` before I leaned
   on it.
8. **The `the-positions` walk**, from `mock/tools/the-positions`. I ran it and I read every one of the
   27 positions it reported, but I did not audit its parser, and its role classification is a reading
   off identifiers which the tool itself says is not a measurement.
9. **Every consumer sentence in the five rows**, from the rows' own `quote` fields. I could not reach
   the source documents, so each is a quotation of a quotation and I mark it so at each use.
10. **That rows one and five are connected at the const generic parameter position**, from the
    `bound` field of `question::what_the_numeric_introduction_door_may_carry_out`. I reached it by
    being refused a line citation into that file, which is the gate doing the job I was not doing.
    Section 5.3 says which half of it I added and which half was already there.

## 12. Options opened, and what closes each

**O1. Whether a platform-width type can be a format.** Closed by settling whether "a constant of the
type" in `ruling::the_format_spine_is_canon` is read per-compilation or per-declaration. Per-compilation
leaves option two of `question::what_a_platform_width_type_is` alive; per-declaration kills it and
leaves the storage reading. This is reserved to the panel, it is one phrase, and it can be closed by
two independent readings of that phrase grounded in quoted canon. It cannot be closed by a probe.

**O2. What range row two's need actually requires.** Closed by reading hilavitkutin's
`hilavitkutin-linking/DESIGN.md.tmpl` in a worktree that has it, and answering whether the error
codes it carries fit a fixed width. If they do, the words "of the platform's size" come out of the
row's `need` and O1 stops gating this row.

**O3. Whether the bound on bare primitives is a count or a position rule.** One expert stands on the
position rule, `proposal::the_introduction_doors_bound_is_a_position_rule_not_a_count_of_types`, and
the shipped tree satisfies it at all 27 positions, which is evidence and not a second instance,
because I measured conformance rather than deriving the rule. Closed by a second independent
derivation of the rule itself. If it closes, a lint over `the-positions`' output becomes licensed and
the design at `DESIGN.md.tmpl:357-360` is where the adoption is written.

**O4. Whether the crate-wide numeric exemption should be position-scoped.** Distinct from O3: O3 is
about the bound's shape, this is about the enforcement's granularity at `mockspace.toml:1418`.
Measured input: the exemption is currently used for nothing but constructor and accessor positions.
Closed by deciding the bound in O3, since a position rule makes the crate-wide flag unnecessary.

**O5. Where a debug capability sits and what it is.** The canon is silent, so this is derivable inside
the intent rather than reserved. Closed by two independent derivations of whether it is a contract on
the format concept or a placement-side rendering, both grounded in
`ruling::the_trait_contract_structure_is_a_primary_paradigm` and the no-alloc constraint. The
caller-supplied-buffer half is the part with no precedent anywhere in the tree.

**O6. Whether the unmeasured shapes in 6.2 also leak.** Struct field types, associated types,
impl-trait returns, macro-reached expressions, cargo feature unification. Closed by extending
`252_probes/run3.sh` with an arm and a control per shape. Cheap, and I stopped because the shape the
obligation names by word was answered and continuing would have been scope rather than depth.

**O7. Whether row five should be split.** Its subject is two things: what arvo may use internally,
and what escapes at each position. The measurement says those have different answers per feature and
per position, which is the arms-over-regions shape rather than one property. Closed by a second
reading of whether an obligation row may carry a predicate at all, which is a question about the
namespace rather than about this row.

**O8. Whether the missing obligation edge from `the_operating_constraints_are_intents_and_rules` is
mine to add.** I did not add it: `mockspace.toml` declares `mock/registry/*.toml` as `canon_paths`
and a panel is open. Closed by a consolidation pass, which is where an edge belongs, and it should be
added with the coverage figure for row one restated afterwards.
