# 265. What a platform-width type is, derived cold and built for three pointer widths

Seat 265, second read on `question::what_a_platform_width_type_is` (Q26), cut at `5644b8f0`.
Phase one of this file was written before `git fetch` and before any panel file other than
`INTENTS.md` and `OPTIONS.md` was opened; the commit carrying it is the evidence. Phase two, the
reconciliation against seat 260, is appended below it and committed separately.

**Gates.** The canon gate passed: the work below rests on `ruling::the_format_spine_is_canon`,
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`,
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`,
`ruling::the_operating_constraints_are_intents_and_rules`,
`ruling::never_a_runtime_check_and_one_lowered_path` and
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`, all at
`ratified` or `in_force`, and on nothing at a lower rung except where a row is named as one expert's
and treated as a hypothesis. Q26 is `decider = "panel"`, its option list is not a boundary
(`ruling::the_option_set_is_not_a_boundary`), and the panel finishes the canon without op
(`ruling::the_panel_finishes_the_canon_without_him`), so deriving it here is licensed. The test gate ran:
the whole suite in `mock/` is green (format 13 passed 1 ignored, placement 21 passed 1 ignored,
strategy 10, doc-tests 4 and 5, the trybuild cases), and the bodies of every test on the surface this
touches were read; the report is in section 8, with two things that are drift and one that is a hole.

**One expert.** This is one derivation. Under
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate` it needs a second
that agrees on quoted canon, which is what the reconciliation section is for.

## 1. The answer

**A platform-width type is an ordinary member of the format inventory whose width coordinate is
bound by a const-available fact of the compilation target rather than written as a literal.** Nothing
about it is degenerate, nothing about it is storage, nothing about it is an axis, and nothing about it
needs a concept the ratified format spine does not already carry. At every compilation it is exactly
one format: quantum one, phase zero, one magnitude, slot range `0 ..= 2^W - 1` or its two's
complement twin, with `W` the target's pointer width. Across compilations the one name denotes a
family of those formats indexed by the target, and the index is a coordinate of the declared
signature, which is what the ratified admission rule says it has to be once two realisations of one
name are seen to disagree.

Three corollaries, each measured below:

- **The line between a format and storage is binding time, and platform width is on the format
  side.** A representable set is a constant of the type when everything it depends on resolves before
  monomorphisation. The pointer width does; a runtime datum does not; the compiler draws the line in
  exactly that place (p2a builds, p2b is refused with `E0015`).
- **The pointer width is an axis of the predicate notation, and it is undeclared.** Claims move along
  it (p3, p4), `dimension.toml` has no row that indexes it, and `target_features` and `toolchain` do not
  cover it. Section 6 states the region every finding here holds in and names the missing axis.
- **The shipped `Slots` ladder cannot host a 64-bit platform width**, so the one consumer obligation
  that asks for one, `obligation::a_platform_sized_unsigned_integer_at_an_api_position`, is unmeetable
  in the shipped tree on every host the consumer builds on (p1 at 64 bits, p4). That is a fact about
  the representation the design chose for a slot index and not about the kind of thing a platform-width
  type is, and the design says so about itself, but the obligation is red and nothing in the tree
  says that.

Two things that are not platform-width types, though the question's keywords list them beside one:

- **`Cap`, a capacity at a const generic position, is the excepted position and not a format.** The
  host's `usize` is what that position takes, a stack-owned newtype there is refused without
  `adt_const_params`, and with that allowed-tier feature it is admitted (p5a, p5b, p5c). What crosses
  at that surface is unsettled and this file leaves it so.
- **The machine word as a carrier is a placement.** The placement ladder has no notion of a machine
  word and needs none: handed the pointer width as a `Width`, the narrowest shipped carrier covering it
  is the word at 16, 32 and 64 bits, by definition of pointer width (p6a).

## 2. The derivation, from the ratified rows

**What a format is.** `ruling::the_format_spine_is_canon` ratifies
`proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`: a format is
identified by its ambient domain and its representable set, and that set is a constant of the type,
with the rider that a value set depending on other data is not a format but storage. It also ratifies
`proposal::membership_of_the_representable_set_is_one_affine_predicate`: membership is one affine
predicate over a slot function, a quantum per magnitude and a phase, of which integers are a point.
So the question "what kind of thing is a platform-width type" reduces to two: is its representable
set a constant of the type, and is it a point of that predicate.

**The second is immediate.** The unsigned platform-width type on a target of pointer width `W` is
`{0, 1, ..., 2^W - 1}` and the signed one is `{-2^(W-1), ..., 2^(W-1) - 1}`. Both are the integer point:
quantum one, phase zero, one magnitude, a slot range. The Rust reference defines `usize` as "an
unsigned integer type with the same number of bits as the platform's pointer type" and `isize` as its
signed two's complement twin, and bounds both below at sixteen bits (`265_probes/fetched/`,
`rust-reference-types-numeric.html`). No coordinate the spine names is absent and none is extra.

**The first turns on what "depends on other data" means**, and the canon settles it from two
directions that agree. `ruling::never_a_runtime_check_and_one_lowered_path` puts the design's
branching at monomorphisation and const solving and forbids runtime validation, and
`ruling::the_predicate_is_whatever_is_available_at_const_time` says a predicate's admissible
category is whatever is const-available, the typestate being one source of that rather than the
only one. Under that reading a set is a constant of the type when every input to it is const-available
at that compilation, because a type is a compile-time object and "constant of the type" cannot mean
anything narrower than that without excluding every associated const. The pointer width is
const-available three ways, `usize::BITS`, `cfg(target_pointer_width)` and `size_of::<usize>()`, and
the three agree at every target (p2a). So the platform-width set is a constant of the type. The
storage rider is about a set that depends on a datum the type does not carry, the block exponent
shared over a column being the corpus's own example (`OPTIONS.md`, the O-B entry under
`#q33-to-q37-five-options-that-lived-only-in-member-files-until-the-unit-s-consolidation`), and
that dependence survives to runtime. The compiler draws the same line: a `Width` bound to
`usize::BITS` is a legal const and a `Width` bound to a function that reads a static is refused with
`E0015` (p2b), with nothing else differing between the two.

**What makes the target an index of the signature rather than a hidden input.**
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` ratifies that an
operation is admitted exactly when it is a function of the declared signature, and that where two
realisations of one name disagree, the signature is missing a coordinate. Build the name
`PlatformUnsigned` for a 32-bit target and for a 16-bit one and the nullary observation `WIDTH`
disagrees (p3: pinned at 32, it passes at three 32-bit targets and refuses at the 16-bit one by the
assertion's own text). Two realisations of one name disagree, so the signature is missing a
coordinate, and the coordinate is the pointer width. Declaring it, which `Unsigned<{ usize::BITS }>`
does in the only spelling the shipped contract offers, restores agreement per compilation. That is the
admission rule's diagnostic and repair applied once, and it is why the target-indexed reading is a
derivation from a ratified row rather than a proposal.

**What follows for behaviour.** `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
ratifies that every operation is a function of the declared width and never of the carrier. A
platform-width type's declared width is the pointer width, so wrapping addition on it wraps at `2^W`,
which is what a native `usize` does on that target and what MATLAB `fi` at word length `W` documents.
Nothing about the platform-ness changes how behaviour is stated; only the value of `W` moves, and it
moves per compilation and never per value.

**What follows for the two obligations.** `obligation::a_platform_sized_unsigned_integer_at_an_api_position`
asks for an unsigned integer of the platform's size covering a unix errno and a Windows `GetLastError`
value, and `obligation::a_primitive_for_every_position_a_bare_number_would_take` asks for a primitive at
every position a platform-sized index would otherwise sit. Both are met by one alias over the shipped
primitives, `USize = <the unsigned integer point at W = pointer width>`, in the same shape
`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives` gives every
convention. Whether the alias covers a `DWORD` is a fact about the target: it does at 32 and 64 bits and
does not at 16, where `2^16 - 1` is below `2^32 - 1` (p4). Whether the shipped tree can declare the alias
at all is section 5.

**What the operating constraints say and do not say.** `ruling::the_operating_constraints_are_intents_and_rules`
forbids a platform dependency and forbids bare `usize` at public positions. Neither touches this.
Reading `usize::BITS` is not a platform dependency in the sense the row names, which is `std::thread`,
`std::time`, `std::fs` and `std::net` (`INTENTS.md`, `#i14-the-operating-constraints-which-are-rules-as-much-as-intents`);
it is a constant of the compilation, and every `no_std` crate built for a target already carries it.
And a `USize` alias at an API position is exactly the stack-owned primitive the rule wants there
rather than the bare type it refuses.

## 3. The four recorded options, and where each is right

`ruling::the_option_set_is_not_a_boundary` means the space is not these four, and the answer above is
not any of them as written. Where each fits, fits badly, or dies:

**Storage rather than format.** Dies for platform width, on p2a against p2b. The criterion it relies on
is right and ratified: a set depending on other data is storage. What it gets wrong is which side the
pointer width falls, because the dependence resolves before monomorphisation. Read as "storage is
what depends on a runtime datum" it is the correct half of the ratified rider, and the Q26 register
entry's own caution, that "storage" was named once in passing in a file about a different topic
(`OPTIONS.md`, `#q26-what-kind-of-thing-is-a-platform-width-type`), was well placed.

**A degenerate instance of the shape family.** Nearest, and wrong in its adjective. It is an ordinary
instance: the same point as `Integer<W>` with `W` supplied by the target instead of the consumer. The
word "degenerate" came from the cost clause that a platform-width type becomes a one-element column,
which `retirement::r161_r13_the_one_element_column_cost_clause` withdrew, and nothing measured here
finds anything degenerate. p1 builds the type against the shipped `Format` contract with no change
to the contract and every membership and cardinality assertion holding at 16 and 32 bits.

**An orthogonal axis.** Half right and the half matters. The pointer width is an axis in the sense
`dimension.toml` gives the word: a situation the world can be in, that a claim is true or false at
(p3 and p4 move along it). The type is not an axis; it is a point whose width coordinate reads that
axis. Filing the type as an axis would put a format in the dimension namespace, which is a category
error, and it would still leave the axis undeclared, which is the real gap (section 6).

**A different kind of thing the format concept need not account for.** Dies on p1: the concept
accounts for it with zero additions. A crate outside `arvo-format` declares it by implementing `Format`
with the shipped `Unsigned` and `Signed` slot ranges, and the crate's own predicates answer for it.
A concept that hosts a thing unchanged is not a concept that need not account for it.

**The reading no option carried and this file lands on:** a point of the parameterisation with one
coordinate bound by reference to a const-available fact of the target. That reading is the one
`proposal::each_choice_in_the_sequence_has_an_owner_and_a_resolution_time` states in its last clause
as one expert's ("a platform-width numeral is a target-indexed family of formats whose exclusion
grounds apply only to dependence that survives to runtime"). It was not read before phase one
began; it was found in the registry after the derivation was written and it is cited here as an
independent prior arrival that this file's instruments now establish rather than as a source.
Under `question::the_ownership_key_as_a_structural_axis` that proposal's key is still open, and
nothing here needs the key: the target-indexed clause falls out of the admission rule alone.

## 4. The instruments, and what each established

Everything below is in `265_probes/`, every probe with its source, the script that built it, and one
output file per target with the exit code and the pointer width on the first two lines. The index is
`265_probes/README.md`. All are spikes: they check one thing each and their spellings are scaffolding.

The matrix is six targets at three pointer widths, the whole range the Rust reference admits:
`aarch64-apple-darwin` and `x86_64-unknown-linux-gnu` at 64, `i686-unknown-linux-gnu`,
`thumbv6m-none-eabi` and `wasm32-unknown-unknown` at 32, and `msp430-none-elf` at 16 with `core` built
from the pinned `rust-src`. The pinned toolchain throughout, `nightly-2026-05-28`.

**p1, `p1_target_bound_width`, against `p1c_literal_width`.** A `Format` with
`type Slots = Unsigned<{ usize::BITS }>` and a signed twin, plus four compile-time assertions: the
width read back through the contract equals `usize::BITS`, the cardinality forced through `slot_count`
equals `2^W`, `2^W - 1` is a member, `2^W` is not, all arithmetic done in `i128` so nothing but a
missing impl can refuse at 64. Builds at 16 and 32 with every assertion holding. Refused at 64 by
exactly four `E0277: Unsigned<64> / Signed<64> is not an admitted slot range` and nothing else. The
control differs in the binding of the width alone (`p1_vs_p1c.diff`) and builds at all six. So the
one thing that moves the outcome is the binding, and it moves it only where the target's width
exceeds the shipped ladder. This is the existence half of the answer: the kind is hosted by the
shipped concept unchanged.

**p2a, `p2a_const_bound_width`, against `p2b_runtime_bound_width`.** Three const spellings of the
pointer width asserted equal at compile time, and an arm `match usize::BITS` with a runtime entry.
Builds at all six. The arm lowered at `-O` for each precompiled target is a bare constant with no
branch: `mov w0, #3` at aarch64, `movl $3, %eax` at x86_64, `movl $2, %eax` at i686, `movs r0, #2` at
thumbv6m, `i32.const 2` at wasm32 (`p2a_const_bound_width/asm/`). p2b differs in one thing, the count
comes from a function reading a static, and is refused at all six with `E0015: cannot call non-const
function in constants`. This is the binding-time half: the pointer width is on the const side of the
line and a runtime datum is not, so a predicate can gate on it and one lowered path survives, which is
`ruling::the_work_is_predicated_arms_composed` and `ruling::never_a_runtime_check_and_one_lowered_path`
holding for this coordinate.

**p3, `p3_one_name_two_realisations`.** The same name with its width observation pinned at 32 in a
`const`. Builds at the three 32-bit targets, refused at 16 by "on this target the name
PlatformUnsigned does not denote the 32-bit set", refused at 64 by p1's `E0277`. This is the
admission-rule half: one name, realisations that disagree on a nullary observation, therefore a
missing coordinate, therefore the pointer width is a coordinate of the signature.

**p4, `p4_obligation_range`.** Whether the set reaches the two ranges the consumer obligation names.
An errno is bounded by `MAX_ERRNO`, 4095, on the Linux kernel side; `GetLastError` returns a `DWORD`,
`2^32 - 1` at most. Builds at 32; refused at 16 by "a Windows GetLastError DWORD does not fit the
platform width on this target" with the errno assertion holding; refused at 64 by p1's `E0277`. So
the obligation's range claim holds at 32, fails at 16, and is undeclarable at 64 in the shipped tree.

**p5, `p5_const_generic_position`.** Three files under bare `rustc` at the five precompiled targets.
`Buf<const N: usize>` builds. `Buf<const N: Cap>` with `Cap` a newtype is refused, "`Cap` is forbidden
as the type of a const generic parameter". The same with `#![feature(adt_const_params)]` and
`ConstParamTy` builds. Each pair differs in one thing. This is what settles that `Cap` is not a
question about formats: it is the position the obligation excepts, the shipped `tests/ui/an_arvo_type_as_a_const_parameter.rs`
already pins the refusal for `Width`, and the only route past it is an allowed-tier feature the
design declines to hand downstream (`mock/crates/arvo-format/DESIGN.md.tmpl:452-459`).

**p6, `p6_placement_reads_the_bound_width`, and `p6a_ladder_over_the_pointer_width`.** `narrowest_carrier`
over `Width::bits(usize::BITS)` returns a carrier whose count equals the pointer width at 16, 32 and 64
(p6a builds at all six), and `LADDER` is the same four-entry list at every target. `derive_sole` over
the platform-width signature under both objectives returns a carrier equal to the word and one
output at sole occupancy, at 16 and 32 (p6; at 64 it inherits p1's refusal). So the placement tier
reads a target-bound width the way it reads any width, the machine word falls out of the ladder by
definition of pointer width, and no concept of "the platform's word" exists in the tree or is needed.

**What was not measured.** No arithmetic was run on the platform-width point; membership,
cardinality and placement are what the shipped tree offers and what was checked. No Windows target is
installed, so the C-ABI-indexed family (`c_long` is 32 bits on 64-bit Windows and 64 on 64-bit Linux)
was not built; it is the same kind of thing indexed by a different const-available fact, and it is
stated as an extension in section 7 rather than as a finding. No thread was spawned and no timing was
taken; every result is a compile result.

## 5. The shipped tree refuses the consumer's platform, and says so about itself

`mock/crates/arvo-format/src/slots.rs:174-178` asserts a declared width of at most 62 inside
`Slots::ADMITTED`, and `slots.rs:245-249` writes the impls at 1 through 62 and nowhere else. The
design owns the bound in as many words: "the design admits declared widths of 1 through 62 bits"
(`mock/crates/arvo-format/DESIGN.md.tmpl:625-627`) and "the bound is a property of this representation
rather than a statement about how wide arvo goes" (`DESIGN.md.tmpl:725-727`). The cause is the carrier
of a slot index, `i64`, and the count of slots at 63 not fitting it.

The consequence nobody wrote down: **`USize` and `ISize` cannot be declared on any 64-bit target**,
which is every host the consumer that asked for them builds on. p1's four `E0277`s are that sentence
as a build result. `obligation::a_platform_sized_unsigned_integer_at_an_api_position` is therefore red
in the shipped tree, and the tree carries two compile-fail cases pinning that `Signed<63>` and
`Fi<64, 32>` are refused (`tests/ui/width_above_the_bound.rs`, `tests/ui/word_length_past_the_ladder.rs`)
without any test naming the obligation those refusals close off. That is honest in the design and
silent in the tests: a reader of the suite cannot learn from it that a consumer need is unmet.

This does not touch the answer to Q26. What kind of thing a platform-width type is does not depend on
whether one representation of a slot index can carry it, any more than `Fi<64, 32>` stops being a
format because the same ladder refuses it. It is reported here because the brief asks for anything
the canon does not license, and a design that meets an obligation nowhere on the consumer's platform
while the registry records the obligation as owed is exactly that. The repair is design-tier and is
not made here: whichever representation the design chooses for a slot range has to reach 64, or the
obligation has to be re-derived as something narrower than "the platform's size", and both of those
are a design round's to decide with the canon in view. What is licensed now, and is the least-bad
residue rather than a proposal, is an honest red: a test in the tree named for the obligation, failing
until the tree meets it, so the suite says what the design already admits.

## 6. Where the answer holds, and the axis it needs

Written in the registry's own grammar over declared axes, with the undeclared one stated beside it
in prose. `ruling::a_predicate_lists_only_what_holds` governs: nothing is written that was not
established.

**The kind: a platform-width type is a point of the parameterisation with the width coordinate
bound to the pointer width, hosted by the shipped `Format` unchanged (p1, p3).**

- `total_width: W in {16, 32}`, established; `W = 64` is refused by the shipped ladder and the claim
  is not established there.
- `fraction_width: F = 0`
- `signedness: signedness in {unsigned, signed}`
- `radix: radix = 2`
- `phase: phase = 0`
- `operation: membership and cardinality only, no arithmetic applied`
- `occupancy: occupancy any, by construction, no value is placed`
- `threads: threads any, by construction, a compile result`
- `target_features: target features any, by construction, a compile result`
- `toolchain: rustc = 1.98.0-nightly (nightly-2026-05-28), edition = 2024`
- `build_profile: opt level = 0, debug-assertions = on`
- undeclared: pointer width in {16, 32}; the refusal at 64 is a separate, established fact.

**The binding-time line: the pointer width is const-available and a runtime datum is not (p2a,
p2b), and an arm gated on it lowers to one path with no branch.**

- `total_width: W in {16, 32, 64}`
- `threads: threads any, by construction`
- `target_features: target features = host default per target`
- `toolchain: rustc = 1.98.0-nightly (nightly-2026-05-28), edition = 2024`
- `build_profile: opt level = 0, debug-assertions = on` for the compile results; `opt level = 2` for the
  five lowered listings
- undeclared: pointer width in {16, 32, 64}.

**The obligation's range (p4).** `total_width: W in {16, 32}`, `signedness = unsigned`, `F = 0`,
`radix = 2`, `phase = 0`, `threads any` and `target features any` by construction, toolchain and
profile as above; undeclared: pointer width in {16, 32}, holding at 32 and failing at 16.

**The placement tier (p6, p6a).** `total_width: W in {16, 32, 64}` for the ladder alone and
`W in {16, 32}` for the derivation; `occupancy = sole`; `container in {u16, u32}` for the derivation;
the objective in {Footprint, Access}, which is the placement ladder's key and not a strategy, so no `S`
is written; `threads any` and `target features any` by construction; toolchain and profile as above;
undeclared: pointer width as for the width.

**The missing axis.** Every predicate above wants to say "pointer width in {16, 32, 64}" and cannot,
because `dimension.toml` declares nothing that indexes the compilation target's pointer width.
`dimension::target_features` is "which instruction-set features the compiler was allowed to use",
which does not move between `i686` and `x86_64` in the sense needed here and does not exist for
`wasm32` at all. `dimension::toolchain` is which compiler and edition, and every result here was
taken under one. `dimension::container` is what a value is realised onto, and p1 through p4 place
nothing. So the axis is missing, and it is an axis by the file's own test: it indexes a situation the
world can be in, a claim is true or false at each value, the values are `16`, `32` and `64` as the
reference lists them, and it is const-available, which puts it in the gateable subset. It moves at
least one ratified-adjacent claim (which width a name denotes) and one obligation (whether a `DWORD`
fits). The set moves only on two independent readings; this is one, and the proposed spelling, offered
for the second reader to attack rather than adopt, is `pointer width = <n>`,
`pointer width in {<set>}`, `pointer width any`. Whether it should instead be the target triple, with
pointer width a projection of it, is the fork the C-ABI extension in section 7 opens, and it is not
decided here.

## 7. Alternatives not taken, and the routes that closed

Listed so the next seat starts from this list rather than from nothing.

**Making the platform-width type a distinct concept beside `Format`.** Not built, because p1 shows
the shipped concept hosts it unchanged and a second concept for a thing the first already covers is
the fragmentation the closed-concept-open-inventory clause exists to refuse
(`proposal::the_concept_is_closed_and_the_inventory_is_open`).

**Making the platform width a coordinate of the placement rather than of the format.** Considered
because the machine word is a carrier. Closed by the floor: every operation is a function of the
declared width and never of the carrier, so a type whose width is the carrier's would state behaviour
over the container, which the ratified premise dissolution refuted on the standards instrument
(`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`). p6a shows the
placement tier needs nothing anyway: the word is what the ladder returns for the width.

**Declaring the width through `cfg` arms rather than through `usize::BITS`.** Both are
const-available and p2a asserts they agree; the `cfg` form needs one item per width and the `BITS`
form needs one. Neither is a canon question. Recorded so nobody re-derives it.

**A `Slots` impl for `Unsigned<64>` in a probe, to reach 64 bits.** Not built, because the trait is
open and an outside crate may implement it, but the count of slots at 64 does not fit the `i64` a
`Slot` carries, so an outside impl would have to lie about `MAX` or fail `ADMITTED`'s width bound
(`slots.rs:174-178`). The refusal is the finding and a probe that routes around it would hide it.

**The C-ABI-indexed family.** `core::ffi::c_long` is 32 bits on 64-bit Windows and 64 on 64-bit
Linux, so an `errno`-shaped alias indexed by the target's C ABI is the same kind of thing as `USize`
with a different const-available index. Not built for want of a Windows target. It matters for
section 6: if the axis is the target triple, pointer width and the C ABI are two projections of one
axis; if the axis is pointer width, the C ABI is a second missing axis. That fork is open.

**The search skill's first two steps.** No SerpAPI MCP tool is loaded in this session and the REST
endpoint answers "Your account has run out of searches", so the two reference pages were fetched
directly, which is the skill's third step, and saved beside the probes with the fetch date.

## 8. The test gate, and three things outside the question

The suite is green across the workspace. On the surface this file touches, the `Slots` and `Width`
door of `arvo-format`, the tests read are `src/tests/the_coordinates.rs`, the `Slots::ADMITTED`
doc-tests, `src/tests/the_inventory.rs` in full, and the four `tests/ui/` cases about width and the
const parameter. They are real: the ladder-coherence sweep at `the_inventory.rs:234-282` walks all 62
admitted widths through `slot_count` rather than a sample, `the_admitted_set_is_the_contiguous_run_the_macro_names`
would fail on any gap, `the_widest_admitted_width_is_where_the_count_stops_fitting` derives 62
rather than restating it, the inverted-range doc-test refuses and its ordered control builds, and
the compile-fail cases commit their diagnostics. Nothing tautological was found on this surface.

**Drift, reported because the brief asks for anything unlicensed.**

1. **Bare `usize` in shipped test code**, nine sites, including a non-exempt crate:
   `mock/crates/arvo-placement/src/tests.rs:323` (`let mut differing = 0usize;`) and
   `mock/crates/arvo-format/src/apply/tests/the_ratio_coordinate.rs:129-131, 202-203`. The workspace
   rule `no-bare-primitives.md` says in terms that test code is not an exception, and
   `ruling::the_operating_constraints_are_intents_and_rules` says the constraints are enforced by lint
   and not to be questioned. Either the lint does not read test modules or the hook exempts them, and
   either way a commit gate that passes this is a gate with a hole exactly the shape of the rule.
   Not fixed here; it is outside this dispatch and a lint config is not a seat's to edit.
2. **The workspace's `unstable-features.md` still cites the deleted tree** as the reason for two
   allowed gates: `const_convert` "required by arvo-storage's `impl const From` (Bool/bool,
   Cap/USize)" and `const_unsigned_bigint_helpers` "adopted in arvo-strategy for the >64-bit-logical
   widening multiply". `arvo-storage` does not exist and the shipped `arvo-strategy` carries no
   widening multiply. A vetting row whose rationale names nothing is a rationale nobody can check.
3. **The repo's own agent rule templates teach `USize` and `Cap` as existing types**
   (`mock/agent/rules/type-surface.md.tmpl` and `cookbook.md.tmpl`, "`USize` (not raw `usize`). `Cap`
   when used as a const-generic capacity"). `cargo.md.tmpl` carries a banner saying its crate names are
   dead; these two do not, and an agent loading them is told a platform-width type exists in a tree
   where none does. Whichever way Q26 settles, those sentences are claims about nothing until the
   design tier writes the alias.

**The hole**, which is section 5: an obligation the shipped tree cannot meet on the consumer's
platform, with the refusal pinned and the obligation unnamed by any test.

## 9. Reconciliation with seat 260

Appended in phase two, after `git fetch origin` and after reading
`origin/dev:mock/research/202608072330_the-numeral-canon-panel/260_kiselyov_what_a_platform_width_type_is.md`
and its `260_probes/`. Nothing above this line was edited after the phase-one commit.

Seat 260's file is `origin/dev` at `22efc5d0`; its probes are `260_probes/p01` through `p05`, each
opened and read as source and raw output rather than through the file's account of them. The
reconciliation is written against what those instruments show, and where 260 states something by
reading rather than by instrument the sentence says so.

### 9.1 Verdict

**Two seats, two personas, two blind derivations, one answer.** 260 and 265 land on the same reading:
a platform-width type is, at any one compilation, an ordinary format in the full ratified sense, and
across compilations it is a family of those formats indexed by the target, with the storage rider
reaching only dependence that survives to runtime. 260 calls it "a target-indexed family of formats";
265 calls it "a point of the parameterisation with one coordinate bound by reference to a
const-available fact of the target". Those are the same object described from the family end and
from the member end, and each file says the other half in its own words (260 section 1.1 second
sentence; 265 section 1 first paragraph, last sentence).

The routes differ, which is what makes the agreement worth something. 260 derives it from the locus
clause's establishing source (the sibling-datum class in `08`) and then compiles the discrimination
(`260_probes/p02`: `E0435` when the coordinate is a value in hand, a build when it is a `cfg`-selected
const). 265 derives it from the binding-time ruling and the admission rule
(`ruling::never_a_runtime_check_and_one_lowered_path`,
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`) and compiles the
same line from the other side (`265_probes/p2b`: `E0015` when the width comes from a function reading a
static; `p3`: one name, disagreeing realisations, therefore a missing coordinate). Neither seat read
the other. 265 did not read `70`. 260 read the registry row that carries `70` L4's sentence before
forming a view, and read `70` itself, and says so; 265 read the row after the derivation was
written. So on the headline claim the instances are: `70` by argument, 265 blind by instrument,
260 by instrument as confirmation of the row. Three instances, two of them independent, and the
two independent ones are the two on the Lattner persona, which is the weakness in the count and is
stated rather than left to be found. The one on the other persona is the one that was not blind to the row. For the two-expert rule as
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate` states it, both
readings ground the same claim in the same ratified rows (`ruling::the_format_spine_is_canon`,
`ruling::never_a_runtime_check_and_one_lowered_path`) by different arguments, which is what the rule
asks for; the persona overlap is the coordinator's to weigh.

**Nothing in either file's answer has to change.** What has to change is one table in 260, which is
section 9.3 item 1, and one line in 265, which is section 9.4.

### 9.2 Agreements, and which dimensions each seat varied to reach each

For every agreement: the claim, how each seat established it, and what each seat varied. Where the
two varied the same thing over different ranges the union is written, because that union is what the
panel now has.

**1. The line between format and storage is binding time, and a target-derived width is on the
format side.**
260: `p02`, one target (the host, 64 bits), varied the *source of the coordinate* over four values:
a literal, a `cfg`-selected const, a field of `self`, a block passed by reference; refusals `E0424`
and `E0435`, controls identical but for the source. 265: `p2a` against `p2b`, six targets at three
pointer widths, varied the *binding* over four values: `usize::BITS`, `cfg(target_pointer_width)`,
`size_of::<usize>()`, a non-const function over a static; refusal `E0015`, one difference per pair,
plus the `-O` listing at five targets showing the const arm lowers to one constant and no branch.
Union: the coordinate's source varied over six spellings, the target over three widths, the refusal
observed under three different rustc error classes, and the lowered path checked, which neither seat
alone covers. 260 never varied the target for this claim; 265 never wrote the sibling-datum arm, so
the block-floating-point control that ties the line to the corpus's own example of storage is 260's
alone.

**2. Within one compilation it is an ordinary instance, and "degenerate" is the wrong word.**
260: `p02` arm two beside arm one, every coordinate read back through `Format` at const time, both
zero-sized; at one target, width `W - 8`. 265: `p1` against `p1c`, six targets, width `W` itself,
four assertions (width, cardinality through `slot_count`, top member in, past-top out), control one
token different per format. Union: the instance is measured at the pointer width itself at 16 and
32 and at eight below it at 64, with the literal control on both sides. Note that 260's `W - 8` is a
workaround for the wall in item 4 and 260 says so; 265 took the wall head-on and paid for it with
no instance at 64.

**3. The four options: 1 holds nowhere, 2 is right minus its adjective, 3 is not the type, 4 is a
true sentence reached by a false route, and the reading that covers the case is absent from the
row.** Both seats, by reading the row against the ratified rows, and both independently of `189`
(neither read it in phase one; 260 read it after, 265 has not). 260 adds a second ground against
option 1 that 265 did not give and now checks and accepts: "storage" as a tier of the number concept
is named nowhere, and the word is already taken twice in other senses
(`ruling::cold_is_for_cold_paths_and_cold_storage`;
`proposal::roles_derive_representations_and_a_realisation_variant_computes_nothing_new`;
`question::is_the_role_set_closed`, all three verified present). 265 adds the retirement that
explains where "degenerate" came from (`retirement::r161_r13_the_one_element_column_cost_clause`),
which 260 does not cite. No dimension varied; this is a reading, twice.

**4. The shipped tree cannot express a 64-bit platform width, the cause is the `i64` slot index and
the 62 bound, and it is a fact about the representation and not about the concept.**
260: `p01`, host only, three pairs, varied the *route to 64* over three values: inside the family
(`E0277`), an outside unsigned declarer (`arithmetic_overflow`), an outside signed declarer (`E0080`
from `ADMITTED`); control at 62 for the first, control with the obligation unforced for the third.
265: `p1` at two 64-bit targets, one route (inside the family, `E0277`, exactly four errors and
nothing else), plus the consequence measured downstream: `p4` (the obligation's range is undeclarable
at 64) and `p6` (the placement over the family inherits the refusal while the ladder alone does not,
`p6a`). Union: three routes closed at the host, the family route closed at two targets, and the
refusal followed into the obligation and into placement. **265 stated in section 7 that an outside
impl "would have to lie about `MAX` or fail `ADMITTED`'s width bound" and did not build it; 260
built both halves of that sentence and found the second half is the one that bites for a signed
range, whose constants all fit.** That is 260's instrument establishing what 265 only argued, and 265
withdraws the argument in favour of the measurement. 260 also found what 265 did not: the `E0277`
diagnostic offers an escape (`slots.rs`, "The trait is open ... what such an implementor owes is the
`ADMITTED` obligation") that `ADMITTED` then refuses unconditionally, so the crate contradicts itself
in two sentences about one trait. That is a defect in the shipped tree and it is design-tier.

**5. The one name denotes different representable sets on different targets, from one source.**
260: `p03`, four targets at two widths, the observation pinned at 64 (via `W - 8 = 56`), the arm
present and then cut out of a copy of the source so a refusal is the arm's and not the target's;
`arvo-format` and `notko` rebuilt per target under bare `rustc`. 265: `p3`, six targets at three
widths, the observation pinned at 32 at width `W` itself, under `cargo --target`, with `core` built
from `rust-src` for the 16-bit target. Union: pinned at 64 the name fails at 32; pinned at 32 it
fails at 16 and is unexpressible at 64. Of the six off-diagonal cells of the 3-by-3 matrix over pointer
width, three now carry a refusal (64 pinned at 32; 32 pinned at 16; 32 pinned at 64, by
unexpressibility rather than by the pin); nobody pinned at 16, and nobody built 260's 64-bit pin at
a 16-bit target, because 265 is the only seat with a 16-bit row at all. 260's cut-out control is
the cleaner instrument for "is it the arm or the target", and 265's
per-target `exit=` and `target_pointer_width=` header in every output is the cleaner provenance;
neither has both.

**6. There is no axis to predicate a target-indexed claim on, it is not `target_features`, and the
pointer width passes `dimension.toml`'s own test for an axis.**
260: `p04`, a census: 25 axes declared, 21 used, `pointer_width` matching zero declared axes and zero
registry lines, with `target_features` and `container` as the positive control on the grep. 265:
by reading the 25 rows and by the two instruments that move along the axis (`p3`, `p4`), and by
eliminating `target_features`, `toolchain` and `container` one at a time on their `what` fields. 260
declined to propose a spelling; 265 proposed `pointer width = <n> / in {<set>} / any` for the second
reader to attack and left open whether the axis is the pointer width or the target triple with
pointer width a projection. **`dimension.toml`'s header says the set moves on two independent
readings. Two now exist, on different personas, reached blind. That condition is met in substance;
the declaration is still not a seat's to make and this file does not make it.** The fork 265 opened
(pointer width against target triple) is one 260 does not name, and it should be closed before the
row is written, because a row keyed on the wrong projection is a row every C-ABI-indexed claim then
works around.

**7. `Cap` is not a platform-width type; it is the const-generic position, which the obligation
excepts and the door ruling already answers.**
260: by reading `PRINCIPLES.md.tmpl:170` and `:219` against
`obligation::a_primitive_for_every_position_a_bare_number_would_take` and
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`. 265:
`p5`, five targets, varied the parameter's type over three values (`usize`, a newtype, a newtype
under `adt_const_params` with `ConstParamTy`), refusal "`Cap` is forbidden as the type of a const
generic parameter", plus the shipped `tests/ui/an_arvo_type_as_a_const_parameter.rs` pinning the same
refusal for `Width`. Union: 260's three-positions split (API value, const-generic parameter, domain
newtype) is the sharper statement of why the row's keywords conflate; 265's instrument is the only
measurement in either file of what the position actually admits.

**8. "No platform dependency" is about std facilities and creates no tension with a target-indexed
width.** Both, from the same sentence of `INTENTS.md` at
`#i14-the-operating-constraints-which-are-rules-as-much-as-intents`. Same source, same reading, so
this is one instance twice and not two, and it is recorded as such.

### 9.3 Disagreements, and what decides each

**1. 260's region table is written in spellings the registry forbids.** 260 section 1.3 writes the
target-indexed family's region as `operation any, ... strategy any, ... build_profile any,
toolchain = nightly-2026-05-28` (260, "1.3 The regions, written out", the fenced block). The
registry's own rows decide this and they decide it against 260:

- `dimension::operation`, grammar: "**`operation any` is not admissible**, because `any` quantifies
  over a set nobody has closed."
- `dimension::strategy`, grammar: "**`S any` is not admissible**, because it quantifies over a set
  op has stated is open."
- `dimension::build_profile`, grammar: `debug-assertions = on | off | in {on, off}`, with `opt level
  = <n>` beside it; there is no `any`.
- `dimension::toolchain`, grammar: `rustc = <version>` and `edition = <year>` as separate entries,
  or `toolchain any` where independence of both is established.

Each of those rows carries a `note` saying that the grammar used to admit `any` while the note
forbade it, and that the gap was closed precisely so a checker reading the grammar refuses the
spelling. 260 wrote the spelling the rows were tightened to refuse, in the one section of the file
whose whole purpose is to state the region in the registry's grammar. **A predicate that would not
validate is not a predicate, and a region stated in an inadmissible spelling is a region left
unstated.** The fix is a rewrite of one fenced block, and 265 section 6 shows the shape: `operation`
named as the operations actually exercised (membership and cardinality), no `S` written at all
because no strategy was run, `opt level` and `debug-assertions` stated, `rustc` and `edition` as two
entries. The claim under the table is right; the table does not carry it.

**2. Whether the 64-bit refusal is an artifact of one coordinate or a wall.** 260 section 3.5 says
artifact and has an instrument: `p05`, the same affine predicate with endpoints derived rather than
declared, building at 64, 100, 126 and at the target's own width, with an agreement arm over all 62
admitted widths at both signednesses and a coverage arm that fails if the macro invocation is one
width short; both arms mutated and both fired, stderrs committed. 265 section 5 says the same thing
by reading ("a fact about the representation the design chose for a slot index and not about the
kind of thing a platform-width type is") and built nothing. **This is not a disagreement in the
conclusion; it is a disagreement in what each seat earned.** 260 earned it. 265 accepts `p05` as
establishing artifact-not-wall, with 260's own caveat carried: the sketch hands back a bare `i128`,
which the operating constraints forbid at a public position, and the sketch is not a design. What
decides the shape of the repair is a design round, and both files say so.

**3. Whether option 3 gets partial credit.** 265 section 3 says "half right": the pointer width is
an axis of the predicate notation, the type is a point that reads it. 260 section 1.2 says "holds
nowhere as an axis of the format" and files the axis finding separately. Both agree the type is not
an axis and the axis is real and undeclared. The row's text is "An orthogonal axis." with no "of
what", so which reading is right is a fact about how the row is read, not about the canon, and
`ruling::the_option_set_is_not_a_boundary` makes the difference immaterial: neither seat files the
type under it. Nothing decides it and nothing needs to.

**4. Whether the obligation's range was measured.** 260 section 7 says nothing in 260 speaks to
the errno and `GetLastError` ranges. 265 `p4` measures both: errno's 4095 bound holds at 16 and 32,
the `DWORD` bound holds at 32 and fails at 16, and both are undeclarable at 64 in the shipped tree.
Not a disagreement; an addition, and it means the obligation's own range claim now has a region:
`W in {16, 32}` measured, `W = 64` refused, pointer width the undeclared index.

**5. The residue for the red obligation.** 265 section 5 names a least-bad residue (a test in the
tree named for the obligation, failing until the tree meets it). 260 section 7 names the obligation
as not delivered and the `ADMITTED`-against-diagnostic contradiction as a `.rs` edit behind a round,
and offers no residue. These do not conflict. 265's residue is marked as residue and not proposal,
and it stays marked so.

### 9.4 A correction to phase one, made here rather than above the line

Line 18 of this file reports the test gate as "format 13 passed 1 ignored". That number is the
`tests/matlab_fi_parity.rs` binary's. Re-run whole in phase two, `cargo test --workspace
--all-targets` from `mock/` at `5644b8f0` gives: `arvo_format` unit 115 passed 1 ignored,
`tests/compile_fail.rs` 11, `tests/matlab_fi_parity.rs` 13 passed 1 ignored, `arvo_placement` unit 21
passed 1 ignored, `arvo_strategy` 10; doc-tests 4 and 5 for `arvo_format`, 0 for the other two.
170 passing, 3 ignored, 0 failing, which agrees with 260 section 0. The gate's verdict does not
move; the count on line 18 undercounts one binary by 102 and is corrected here because nothing above
the phase-two line is edited after the phase-one commit.

### 9.5 The gate at the base is broken, in the shape 260 section 6 describes, and it cost this seat
a commit

At `5644b8f0` the mockspace pre-commit gate refused the phase-one commit twice before it passed, and
neither refusal was about the deliverable. First, `E0308: expected LintPack, found LintPack` with
notes pointing at another worktree's tool sources: the workspace's shared `target/` (per
`.cargo/config.toml`) let the lint pack link tool rlibs built from a sibling worktree against a
different copy of `mockspace-lint-rules`, which is 260's two-copies-of-one-crate failure arriving
through a different door. Second, after a clean rebuild of the pack from this worktree's tool
sources, `the-tool-locks-pin-one-mockspace/the-tool-locks-disagree`: the five `mock/tools/*`
lockfiles pin `mockspace-lint-rules` at `cf0fb06` while the generated pack pins `b96752d`. The lint's
own instruction, `cargo update -p mockspace-lint-rules` in each tool directory, moves all five to
`b96752d` and the gate then passes. Those five lockfile edits are left uncommitted in this worktree
on purpose: they are not this dispatch's, the trunk already carries the proper repair at `1ac3c8de`
(the `https` spelling in the five manifests, which 260 section 6.1 records), and committing a second
repair on the `ssh` spelling would be exactly the divergence 260 measured. What is worth recording
is that a worktree cut at `5644b8f0` cannot commit at all until an agent repairs the tool locks by
hand, and that the shared `target/` makes the pack's freshness depend on which worktree built last,
so the repair does not stick while another worktree is building. Both are infrastructure findings
and neither is arvo's.
