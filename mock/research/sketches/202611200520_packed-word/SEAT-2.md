# The Packed Word: Seat 2

This is one independent read of the packed-word question, derived from the registry, the crates on
`origin/dev` at `6def8831` and the bit tier on `origin/work/arvo-bits-tier` at `1fd4d78e`. Nothing on
`work/arvo-packed-word` and no sketch named packed-word was opened. It is one seat's reading and binds
nothing until it is reconciled against the other seat's; where the two disagree the call is not mine
and not a third seat's.

The short form of the answer is that a packed word is not a new kind of thing. It is the bit tier's
masked cast, `Bits::cast`, taken at an offset other than zero and given a name. Everything below
follows from holding on to that: it lives where the cast lives, its field values are held to the same
contract its container is, and a one-bit field needs no rule of its own.

Every code shape in this file is a spike's scaffolding. Names, arities and trait spellings were chosen
to reach a check, not to be the design, and the section on evidence says exactly how little stands
behind them: one uncommitted spike, which may not be cited.

## The two gates, and what they found

### Canon gate: passes, with two things the canon does not say

Checked against the ratified rows `ruling::never_a_runtime_check_and_one_lowered_path`,
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`,
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`,
`ruling::the_trait_contract_structure_is_a_primary_paradigm`,
`ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`, the in-force
`ruling::the_operating_constraints_are_intents_and_rules`, the stated
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`, and the obligations
`a_primitive_for_every_position_a_bare_number_would_take`,
`an_exact_width_container_a_consumer_can_alias_and_pin` and
`the_unstable_machinery_does_not_reach_a_consumer`. Nothing proposed here conflicts with any of them.

Two gaps, stated because the brief handed me "arvo needs a packed word" as settled context and the
registry does not carry that sentence.

First, no obligation row names this need. A search of every row for packed word, bit field, named
field and sub-word finds nothing on the demand side. The eighteen obligations name a bit set, an
exact-width container, a count, a hash and the rest, and none of them is a consumer asking for named
fields in one word. The nearest licences are indirect: `obligation::a_primitive_for_every_position_a_bare_number_would_take`
("so that no such position anywhere in the stack has to fall back on the machine's own types"), under
which a consumer with a pinned 32-bit wire word otherwise writes shifts and masks on a bare `u32`; and
`ruling::cold_is_for_cold_paths_and_cold_storage` ("it aggressively minimises and bitpacks"). Those make
the thing derivable inside the canon's spirit. They do not make it a recorded need. Before a design
round declares canon for this, an obligation row with a consumer's own words is owed, or the design
declares the two rows above and says in terms that the need is derived and not quoted. I did not treat
the missing row as permission and I did not treat it as a refusal; the design call itself is the
two-seat mechanism the canon provides for exactly this.

Second, the canon does not say who owns a declared shared layout. `dimension::occupancy` and the two
ratified rulings above settle what shared occupancy is ("At a shared placement no per-element footprint
exists to observe") and that the derivation from a declared signature has three outputs there. They are
about a placement arvo derives. A packed word's layout is declared by the consumer, pinned by a
register map or a wire format, and nothing in the registry says whether a declared layout is the
placement crate's subject or the container crate's. My reading is in call 1. It is a reading, grounded
in quoted text, and it needs the second seat to have reached the same place independently.

### Test gate: passes on `dev`; the tier branch's suite was read and not run

The whole suite on `dev` at `6def8831` ran green: `cargo mock test`, 775 tests in 16 member binaries,
then the seven tool crates, the lint crate and the benches, ending `mock test: 10 tree(s) green`. It ran
in a copy of the clone under the scratchpad with a private cache directory, because running it in the
clone writes `mock/test-history/`, which the brief forbids, and because the shared lint cache belonged
to the clone's worktree and the tool refused to test it from another. The log is
`/tmp/claude-0/-home-user/d2e72047-4ad7-5165-b8cf-ce72637f96de/scratchpad/s2/test-dev.txt`.

That run is of `dev`, and the surface this design sits beside is not on `dev`. The bit tier is on
`origin/work/arvo-bits-tier`, and I did not run that branch's suite. I read every test body in it
instead: `src/tests.rs`, `src/bytes/tests.rs`, `tests/common/mod.rs`, `tests/the_bit_access.rs`,
`tests/the_bit_logic.rs`, `tests/the_bit_sequence.rs`, `tests/the_bit_tier_in_a_constant.rs` and
`tests/compile_fail.rs`. I found no tautology and no sampled law. The sweeps run all sixty-four widths
and every index below each, the oracles are built one bit at a time rather than by the crate's own
formula, the folded mask oracle is itself shown able to fail, and twelve mutant flags are recorded as
turning the tests red. The one place a sample is taken, values above eight bits, says so and names what
it takes. That suite describes the thing it covers. Whether it passes today is the branch's committed
record's claim (`mock/test-history/default/arvo-bits.json` there carries a green hash) and not mine.

The committed record on `dev` disagrees with the run. At `6def8831`, `mock/test-history/default/`
carries `"green": null` for `arvo-bits`, `arvo-digest`, `arvo-format`, `arvo-placement` and
`arvo-strategy`, and a green record only for `arvo-capacity`, `arvo-contracts` and `arvo-numeral`. The
members pass; the history that is supposed to say so was committed nulled for five of eight. That is a
record to repair in the round that next touches them, not a red suite.

One more thing noticed outside my scope. The generated `.claude/CLAUDE.md` says "The tree holds five
crates and nothing above them" and that `mock/Cargo.toml` "names those five and no others". The tree on
`dev` holds eight, and `mock/DESIGN.md.tmpl` says "Eight crates". An agent instruction that
misdescribes the tree by three crates is the kind of stale sentence that gets believed.

## What the sources say that decides the shape

Four facts, each from a primary source, carry most of the weight.

The tier branch already ships the one-bit case of this. `BitAccess::bit::<I>(self) -> Bool` and
`with_bit::<I>(self, to: Bool) -> Self` read and write a field one bit wide at offset `I`, and the value
type they chose is `arvo_format::Bool`, not `Bits<1>`.

The tier branch already ships the offset-zero case of this. `Bits::cast::<M>` is, in the design's own
words, "the same masking arithmetic at a different bound". A field read is that cast after a shift
down, and a field write is that cast's inverse after a shift up.

The canon already has the vocabulary. A field in a shared word is a value at shared occupancy: "the
element is reached through its carrier and no size-bounded contract ranges over it"
(`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`, ratifying
`proposal::at_shared_occupancy_no_per_element_footprint_observation_exists`). The unratified
`proposal::the_lens_degenerates_to_an_ordinary_value_at_sole_occupancy` names the same object a lens,
"a placement of the value's bits, a carrier, an offset and a width". It is one expert's and I cite it
for the word, not for authority.

The stack already has the type-level list. `notko-hlist` is a workspace dependency (`mock/Cargo.toml`,
optional on `arvo-numeral`), and it ships `Cons`, `Empty`, the positions `Here` and `There`, and
`PlaceOf<M, P>`, which finds a member's place with the place inferred, whose two impls do not overlap,
and which refuses a list naming a member twice by "finding two places and asking which". Its plain path
needs no unstable feature.

## Call 1. Placement: `arvo-bits`, a new module, on top of the bit tier

The packed word goes in `arvo-bits`, as a new module beside `tier` (my spike calls it `packed`; the
name is the design round's). It extends the crate; it does not extend an existing module, and it does
not get a crate of its own. It depends on the tier branch landing first, because it needs `BitWidth`
and the `arvo-format` edge that branch adds for `Bool` and `Width`. On `dev` as it stands `arvo-bits`
depends on notko alone and the root design still says naming a value at an exact width needs
"`arvo-bits` and nothing else either, including not `arvo-format`"; the tier branch rewrites that
sentence, and this module inherits the rewrite rather than causing it. It adds one edge of its own,
`notko-hlist`, default features off.

The reasoning is from the crate's own stated subject. The tier branch's design opens: "One type,
`Bits<const N: u32>`, holding a value known to be exactly `N` bits wide", and its tier is "the bit
operations over that value". A field read is a bit operation over that value with a result at a
narrower exact width. It is the operation the crate already has at offset zero (`cast`) and at width
one (`bit`), stated once for every offset and width. Putting it anywhere else would store the same
shift-and-mask fact in two crates, and the copies would decorrelate.

The alternatives, and why each loses.

`arvo-placement` is the tempting one, because its subject line is "where the bits sit" and occupancy is
its axis. It loses on its own charter twice. Its design says the crate derives placement "from a
declared signature" and "is checked against ever holding a host primitive itself"; a packed word holds a
value and its layout is declared, not derived. And the ratified rule that "no declared operation reads a
placement" would be violated the moment a field read consulted a `Placement`. The packed word must
compute from declared widths alone, which it does, so it has nothing to ask that crate. I also reject
the halfway form, the layout descriptor in `arvo-placement` and the accessors in `arvo-bits`: it gives
`arvo-bits` an edge onto the carrier ladder for a consumer who wanted a bit word.

`arvo-format` loses in one line of its own README: "A format here is not a container", and "There is no
machine container anywhere in this crate and you cannot reach one from here".

`arvo-numeral` loses because `Num<S>` "holds one member of a declared signature alone in the carrier",
which is sole occupancy by definition, and because a packed word carries no sign, quantum, rounding or
overflow policy. Numerals as field values are a real want and are addressed under call 4 as a bridge
this shape leaves room for, not as a reason to home the word there.

`arvo-strategy` binds a name to an objective and an adaptation. A consumer-declared layout has no
objective to select; the wire format already selected it.

A new crate, `arvo-packed` or similar, loses on what it would contain. Under the workspace's own
crate-subject reasoning (the one `proposal::a_value_contract_is_not_the_format_crates_subject` applied to
the numeral contracts) a crate is decided by what it is for. This is for a bit container's fields, and
the crate for bit containers exists. The split would also put the shared contract of call 4 in one
crate and its first implementor, `Bits<N>`, in another for no consumer's benefit: nobody wants fields
without the container.

Extending the `tier` module rather than adding a module loses on file concern. The tier is four
contracts over a container's own bits with one guard, `Within`. The packed word brings a layout, a
field list and a different admission proof. One module, one concern.

One thing I considered and am not proposing, recorded so the next seat does not start from nothing: a
positional tier contract, `field::<AT, W>(self) -> Bits<W>` with a `Spans<B, AT, W>` proof modelled on
`Within`. It compiles in principle without `generic_const_exprs`, since `W` is a parameter in the
return type. It is the obvious generalisation of `bit::<I>` and it is the wrong one for this question
on three counts. It cannot see a second field, so it cannot refuse an overlapping layout. It fixes the
value type to `Bits<W>`, which forces a one-bit field to read as `Bits<1>` against the tier's own
`Bool`. And a layout built above it could not call it with a derived offset, because an associated
constant cannot be passed as a const generic argument without the forbidden feature. It may still be
worth having as a tier operation in its own right. It is not the packed word.

## Call 2. Shape: a layout is an ordered list of named fields, and positions are derived

### What the consumer writes

A field is a unit type the consumer declares, and its declaration says one thing: the type its value
has. A layout is a second unit type, and its declaration says two things: the word type, and the
fields in order from the low bit. In the spike's spelling:

```
struct Version;  impl Field for Version { type Value = Bits<4>;  }
struct Length;   impl Field for Length  { type Value = Bits<12>; }
struct Urgent;   impl Field for Urgent  { type Value = Bool;     }
struct Tail;     impl Field for Tail    { type Value = Bits<15>; }

struct Header;
impl Layout for Header {
    type Word   = Bits<32>;
    type Fields = Cons<Version, Cons<Length, Cons<Urgent, Cons<Tail, Empty>>>>;
}
```

The word is `Packed<Header>`, a transparent wrapper over the word type. A field is read by naming it,
`header.get(Version)`, which returns the field's own value type, and written by value,
`header.with(Length, v)`, which takes the field's own value type and returns the word. The whole word
comes in through one total door and goes out through one total door. No offset is written anywhere, no
bare integer appears outside a const generic parameter, the consumer writes no macro and enables no
feature, and `Field` and `Layout` are plain traits with associated types only.

### Why positions are derived and not declared

A layout's information content is an ordered partition of the word's bits into named intervals. The
widths in order are that information exactly; offsets are their prefix sums and add nothing. A shape
that asks the consumer to write offsets as well asks them to store the same data twice, and then needs
a checker to confirm the copies agree: pairwise disjointness, every field inside the word, and a
membership proof that the field being accessed was among the ones checked. With derived positions two
of those three checks have no failing case left to detect. Overlap is not refused, it is
unrepresentable. A field outside the word is not refused, it is unrepresentable. That is the stronger
guarantee and the smaller mechanism.

Derived positions have one well-known failure, the one C bit-fields have: drop or resize a field and
everything above it moves silently. The declared word width is what closes it. The consumer states the
word type independently, and the layout's admission asserts that the fields' widths sum to exactly the
word's width. A dropped or resized field changes the sum and the build stops. That is why the check is
equality and not at-most. It is not a policy about tidy layouts; it is the checksum that makes derived
positions safe, and with at-most it stops detecting the one error it exists for. A gap in a wire format
is therefore a field like any other, named by the consumer and given a `Bits<K>` value. arvo ships no
special reserved type and decides nothing about what a consumer calls its padding.

### What is checked, and when

At build, by a constant forced in every door onto `Packed<L>`, the way `Bits::ADMITTED` and
`Within::NEW` are forced: the fields' widths sum to the word's width. This is `E0080` at `cargo build`,
on the same footing as the tier's refusals, and it wants the same `trybuild` discipline the tier's
`compile_fail.rs` describes (a passing case in the same `TestCases`, so the run builds rather than
checks).

At build, inherited and not restated: the word's own width is within one to sixty-four and each field's
width is at least one, because reading any `BitWidth::WIDTH` forces that type's own admission. A
`Bits<0>` field is refused by `Bits`, not by a second copy of its rule in the layout.

At type check, by the trait solver: naming a field the layout does not list is an unsatisfied bound;
handing `with` a value of another field's type is a type mismatch; a field listed twice is refused at
the access as an ambiguous place. That last one is honest about its limit. Distinctness of names
cannot be asserted at the declaration, because saying two types differ needs negative reasoning the
solver only has under `specialization`. `notko-hlist` says the same of itself. A duplicate nobody
accesses is a wasted interval, not a wrong read.

Never, because the case does not exist: a value too wide for its field. `with` takes the field's value
type, and that type has exactly the field's width, so there is nothing to truncate and nothing to
refuse. This is the largest practical win of the shape over the conventional shift-and-mask setter that
takes an integer and has to decide between masking silently and checking at run time.

Never, because the case does not exist: an invalid word. Every field's value type is in bijection with
its bits (call 4), and the fields tile the word, so every pattern of the word's width is a valid packed
word. The door in is total and validates nothing, which is what keeps it inside
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` under every reading of its open
question, for the same reason `masked` is.

At run time: nothing. A read is a shift by a constant and a mask; a write is a mask, a shift and an or.
The offset is an associated constant of a monomorphised type, so it is a literal by the time the
backend sees it.

### Const, dispatch, features

`get`, `with` and both doors are `const fn`, and how they are const is the one place the first shape I
wrote was wrong. I bounded the accessors `F::Value: [const] Pattern`, the way the tier's own generic
test bounds `const fn meet<B: [const] BitLogic>`. That makes them conditionally const, and a consumer
crate with no feature line that evaluates one in a constant is refused: `E0658`, "cannot call
conditionally-const method `Packed::<Header>::with::<Version, Here>` in constants", with the help line
telling the consumer to add `#![feature(const_trait_impl)]`. That is
`obligation::the_unstable_machinery_does_not_reach_a_consumer` failing in exactly its own words. The
tier escapes it because `Bits<N>` is concrete and its operations are inherent; a packed word is always
generic over the consumer's layout, so it has no concrete path to hide behind.

The mechanism is the conditional bound, so the repair is to make the bound unconditional and move it
off the accessor. The two declaring traits require it once, on their associated types: a field's value
type and a layout's word type must each have a const impl of the value contract. The accessors then
carry no const bound at all and are plain `const fn`. With that change the same feature-free consumer
crate builds the word in a constant and reads a field out of it in a constant.

The cost is real and lands on one party. A consumer who writes a value type of their own and wants it
as a field must give it a const impl of the contract, which does need the gate in their crate. A
consumer who uses arvo's value types, which is everyone declaring a register or a wire word out of
`Bits` and `Bool`, needs nothing. I think that is the right side for the cost to fall on, and it is
still a place where the obligation is not met in full; the design round should say so in its own
words rather than discover it.

Dispatch is monomorphisation throughout. No `generic_const_exprs`, no `specialization`, no
`adt_const_params` and no `min_generic_const_args` is needed. I looked hard at the last one, because
`Bits<F::WIDTH>` with a type constant is the direct way to say a field's value type, and rejected it:
the consumer would write the type constant in their own impl and carry the gate themselves, against
the same obligation. An associated type says the same thing with no gate.

The field list rides `notko-hlist`'s `Cons` and `Empty` and its `Here` and `There` places rather than a
list of arvo's own. arvo adds one trait over that list, carrying two constants: the total width, and
the offset of the member at a place.

### Alternatives rejected for the shape

Explicit offsets, each field a `Lens<const AT, const W>` alias or an associated `Width` constant. This
matches how a datasheet reads and it is the conventional answer. Rejected for the redundancy argued
above: it reintroduces overlap and out-of-range as representable states, then needs a disjointness walk
and a membership proof to refuse them. If the other seat chose it, the question to settle between us is
whether a consumer transcribing a register map is better served by writing offsets and having three
things checked, or by writing widths and having one thing checked. I hold the second, and the argument
that would move me is a real layout whose gaps make the reserved fields outnumber the named ones.

A tuple of fields rather than a cons list. Reads better, and needs one impl per arity, which is a
hardcoded limit on the consumer chosen by arvo. Rejected.

Fields named by index, `get::<2>()`. No lookup machinery at all, and the names the question asks for
are gone. Rejected.

A declarative macro that expands a struct-like block. arvo ships no proc macros; nothing forbids
`macro_rules!`, and nothing here needs one. A design that is only usable through a macro has a weaker
contract than one whose traits can be written by hand. If sugar is wanted later it expands to exactly
the declarations above and adds no semantics.

An in-place setter taking `&mut self`. Not proposed. The tier's `with_bit` is by value, and a by-value
write keeps the read-modify-write of a shared word visibly the consumer's, which matters because
`retirement` rows in the registry already record a sub-byte read-modify-write on shared storage as a
write hazard.

## Call 3. A one-bit field reads as `Bool`

A field reads as the value type its declaration names. A flag is declared with `Bool` and reads as
`arvo_format::Bool`, and that is the one-bit field the design should document, because it is what the
tier already decided: `bit::<I>` returns `Bool`. The law that pins it is that a `Bool` field at derived
offset `I` reads the same as `word.bit::<I>()` and writes the same as `word.with_bit::<I>(to)`, at
every offset of every word width.

There is no width-one rule in the mechanism, and this is the part I would defend hardest. The read's
return type is the field's associated type. Nothing maps a width to a type. A rule that made width one
read as `Bool` and every other width read as `Bits<W>` is a return type that differs at one value of a
constant, which is `specialization` by another name and is not available. So the choice is not between
`Bool` and `Bits<1>` as a default; it is between a shape where the question dissolves and a shape that
is forced to answer `Bits<1>` and then disagrees with `bit::<I>` forever.

`Bits<1>` stays admissible as a declared value type, for a one-bit code that is not a truth value (a
one-bit version, a one-bit selector between two encodings). The repository's own type-surface rule
draws exactly that line: "`Bool` for a truth value and `Bits<N>` for an opaque bit pattern of exactly
`N` bits". Which one a given bit is, is the consumer's domain knowledge. arvo refusing `Bits<1>` there
would be arvo deciding a consumer's meaning for them.

Rejected: always `Bits<1>`. It is what a width-keyed API is forced into, and it makes the tier
inconsistent with itself. Rejected: an implicit width-one-means-`Bool` mapping, as inexpressible.
Rejected: a dedicated flag accessor beside `get`, as a second door onto one operation.

## Call 4. One contract, shared by the container and every field value

Yes. The word type and every field's value type are held to the same contract, and `Packed<L>` itself
satisfies it.

The contract is: a type whose values are in bijection with the bit patterns of exactly its declared
width. Stated as laws over the two total maps it carries, to and from the low bits of the widest
carrier (the spike calls them `lower` and `lift`, and the trait `Pattern`; the names are scaffolding):

- `lift(lower(x)) = x` for every value `x`.
- `lower(x)` has no bit set at or above the type's `WIDTH`.
- `lift` reads only the low `WIDTH` bits and is total, so `lower(lift(b)) = b` masked to `WIDTH`.

`BitWidth` is its supertrait, so it adds nothing to how a width is named. `Bits<N>` satisfies it with
`cast::<64>()` and `cast::<N>()`. `Bool` satisfies it at width one. `Packed<L>` satisfies it at its
word's width by delegating to the word. Each of those impls is const, and the declaring traits require
that of whatever a consumer names as a word or a field value, for the reason given under call 2.

The reason it has to be one contract is that the two roles are the same role. A field is a word at a
smaller width; extraction is the masked cast the container crate already has. Two contracts would be
that fact written twice. And one contract buys closure, which is the part none of the narrower shapes
get: because `Packed<L>` is itself a pattern, a packed word can be the value of a field in a larger
packed word, with no nesting mechanism added. A sub-word falls out of the definition.

It also leaves the door open in the right place. A consumer's own type can be a field value by
satisfying the contract, on the same terms as arvo's, which is the trait-contract paradigm
(`ruling::the_trait_contract_structure_is_a_primary_paradigm`) applied here. The bijection law is what
keeps that honest under the no-runtime-check rule: a two-bit field whose domain type has three valid
values does not satisfy it, because `lift` would have to reject or invent. That consumer declares the
field `Bits<2>` and decodes at their own boundary, which is where the ingest ruling puts that decision.

What the contract deliberately does not require of the container: the tier's `BitLogic`, `BitAccess`
or `BitSequence`. A packed word needs none of them, and requiring them would refuse `Bool` as a word.

Three limits, stated as limits.

The interchange is `Bits<64>`. That is the container crate's existing bound ("Nothing past sixty-four
bits", in its own design) and not a new one, but it does mean a packed word over a wider container, an
array of `Bits<64>`, is not reached by this contract. The honest general form names the pattern type
rather than fixing it, and I did not chase it, because the wider container itself is the open half of
`obligation::set_operations_over_a_fixed_size_bit_set` and does not exist to design against.

Numerals as field values are not delivered. `Uint<3, A>` is a bijection with three bits and would
satisfy the contract, but the impl needs both `arvo-numeral` and `arvo-bits` in reach, and neither
depends on the other today. Under the orphan rule it lands in `arvo-numeral` behind a new edge, or in a
crate above both. That is a placement call of its own and I am not making it here.

The shipped `Bits<N>` is described in the root design as "the one exception" to contracts-plus-instances,
with "no open contract behind it". The tier branch already ended that with four traits; this adds a
fifth contract `Bits<N>` is an instance of, and the root design's paragraph wants rewriting when the
tier lands, not when this does.

Rejected: field values fixed to `Bits<W>`. A concrete lump where a contract belongs; it forces call 3
to `Bits<1>` and forbids nesting. Rejected: the container held to the tier and the field values held to
something else. Two statements of one fact. Rejected: the contract as a conversion to and from a host
integer. That puts a bare `u64` in a public trait position that outside implementors must write.

## Evidence, and how little of it there is

One ad-hoc spike, uncommitted, and nothing else. It is not a sketch, because a sketch is committed
under `mock/research/sketches/` and the brief made the clone read-only, so under the workspace's
evidence rule it cannot be cited and the claims below are owed a committed sketch before anything is
built on them. I describe what it showed so that the sketch has something to reproduce or refute. The
sources and the raw compiler output are at
`/tmp/claude-0/-home-user/d2e72047-4ad7-5165-b8cf-ce72637f96de/scratchpad/s2/spike/` (`src/lib.rs`, the
library half; `src/main.rs`, a consumer crate with no feature line; `src/lib_conditional.rs.txt`, the
refused first shape; `OUTPUT.txt`). It ran under `nightly-2026-05-28`, the repository's pin.

What it is and is not. It is one file of stand-ins: its own `Bits`, `Bool`, `Width`, `Cons`, `Empty`,
`Here` and `There`, written to have the shapes of arvo's and notko's and none of their code. It
instantiates one layout of four fields in a 32-bit word and one nested layout. It tests no law over
the matrix of widths and offsets. Its setup may make its invariants true rather than test them. It is
one instance from one author.

What it showed, on that footing:

1. The whole shape type-checks and runs: declaration by two plain traits, a field found by an inferred
   place from a marker passed by value, read and write in a constant and at run time, a `Bool` field
   between two `Bits` fields, and a packed word as the value of a field of a larger packed word.
2. A layout whose widths do not sum to the word's width passes `cargo check` and is refused at
   `cargo build` with `E0080`, "evaluation of `Packed::<Short>::ADMITTED` failed here", through a
   run-time call. Same footing as `Within::NEW`, including the check-versus-build gap the tier's
   `compile_fail.rs` already works around.
3. A field the layout does not list is refused at `cargo check`: `E0277`, "the trait bound
   `Empty: Holds<Version, _>` is not satisfied".
4. A field listed twice is refused at the access: `E0283`, "type annotations needed". The diagnostic
   is poor and wants an `on_unimplemented` note of the kind `notko-hlist` carries.
5. The conditionally-const accessor is refused from a feature-free consumer's constant, and the
   unconditional associated-type bound is accepted from the same consumer. This is the finding of the
   spike and the only one that changed the design.

What is still a hypothesis: that the same holds over arvo's real `Bits`, `Bool` and `BitWidth` and
notko's real list, where `Cons` is sealed and `PlaceOf` has `At` as a supertrait; that the impl of the
contract for `Packed<L>` sits beside the impl for `Bits<N>` under the real crate's coherence; and
every law over the full matrix, none of which the spike asserts.

The lowering is unpriced. I expect a read to be a shift and an and, and a write three or four
instructions, and I have no disassembly and no bench, so that expectation is worth nothing as evidence.
`cargo mock bench disasm` on a built arm is what would say, and the arms a bench would need are the
hand-written shift-and-mask on a bare integer, this shape, and the explicit-offset shape if the other
seat proposed it.

What would change my answer on shape: the real `notko-hlist` list refusing the offset walk, in which
case arvo carries a list of its own (a duplication I would want to see forced by a compiler error
before accepting), or a consumer's need for their own field value types proving common enough that the
gate they then carry outweighs constant evaluation for everyone else.
