# What the record already answers, for the questions op asked on 13 November

Op's two answers of 13 November 2026 are quoted whole in `OPS-WORDS.md` beside
this file. Both of them lean on arvo's existing record rather than replacing it:
the first ends "like already designed in arvo (which you would know had someone
done the canon double check and ensure it's up to date and sensible etc)", and
the second says the contracts are arvo's own terrain. This file is that reading.
Five questions, each answered from rows and passages rather than from memory, and
each saying plainly where the record stops.

The method is the same throughout. A claim about what the design says carries the
row by namespace and slug, or the passage quoted with its file and line. Where
the registry holds nothing, the answer says the canon is silent, and silence here
is a claim about the registry, which was read whole, and not about the two panel
corpora, which `LEDGER.md` records as unread.

Two things are worth saying before the five. The record is in better shape than
the question implies: every one of the five has substantial material under it,
and three of them are answered outright. And the shape of what is missing is
consistent, which is itself a finding: the format tier is thoroughly specified,
the value tier above it is one crate old, and the contracts tier a consumer would
actually bound its generic code on does not exist in any tier.

## a. The standard-paired implementations

### Which standards the record names

`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` is op's own,
and it is the whole answer to scope. Two are in: MATLAB `fi` and `fimath`, and
IEEE 754. Three are reserved rather than excluded: SystemC `sc_fixed`, Vitis
`ap_fixed` and Algorithmic C `ac_fixed`. His words in the row's `quote`: "Let's
start with the two named, and reserve the rest named for future, so it can't
close them out." The row's note says the reservation is the operative half, and
that naming the three is what stops the answer being read later as having ruled
them out.

Nothing in the record names C or Rust integer behaviour as a standard to be
paired with. They appear instead as the baseline for intuition:
`ruling::warms_objective_is_the_intuitive_best_choice` is where that sits, and
Q formats appear only as a notation inside the fixed-point material rather than
as a convention with its own parity target. So the paired set is two now and
five at most, and that is a decided thing rather than an open one.

### What paired means

`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
is the demand, and the word it uses is alias. The obligation's own `need` is that
every standard convention be expressible as an alias over the primitives, which
is a claim about the primitives rather than about the standards: if a convention
needs a computation the primitives do not have, the primitives are short.

What pairing means concretely is in the shipped crate, and the module doc is the
clearest statement of it anywhere, `arvo-format/src/standards.rs:6-18`:

> A consumer already thinking in a standard's vocabulary keeps thinking in it and
> gets arvo underneath. There is no adapter and no second implementation of
> anything: a convention names a width, a scaling and a rounding-and-overflow
> pair, and each of those is one of this crate's coordinates under a different
> word.
>
> Nothing behind the naming is what makes it honest. A convention needing a
> computation to work would not be one arvo supports badly, it would be a gap in
> the primitives, which is what the standards bound is for. So the interesting
> output here is a refusal.

That is the definition to use: a standard is a naming of coordinates arvo already
has, and the interesting result is the refusal rather than the match. The crate
delivers it. `Fi<W, F>` and `Ufi<W, F>` are MATLAB's word length and fraction
length as a `Format` impl; `FractionLength<F>` is MATLAB's fraction length stated
as a quantum law, written as a law rather than as `Constant<{-F}>` because the
negation in a const generic argument would need a feature the workspace forbids;
`rounding_method` re-exports five of the six modes under MATLAB's names, so the
mapping is something the compiler agrees with rather than a sentence in a
document; `overflow_action` re-exports both of MATLAB's, and says so: "Both land,
so this map is total and nothing here is marked."

The two refusals the module advertises are the valuable part. MathWorks documents
a word length to 65535 and arvo's slot ladder stops at 64, so a declaration past
64 is refused where it is written, and the crate calls that a gap in the
primitives pinned as a compile-fail case rather than worked around. And MATLAB's
Round takes a tie away from zero, which is not one of the six modes and is not
meant to be one, because the canon reaches it as an alias. Both are claims about
arvo rather than about MATLAB, which is exactly what the module doc says the
output of this exercise should be.

The second half of the mandate is where it stops.
`ruling::the_standard_is_parity_in_output_not_in_the_internals` ranks the two
things being weighed and is worth quoting for what it decides, since it bears
directly on the overflow question in part b: "A standard covered with marked holes ships; a framework with
holes does not. So the standards are a check on whether the framework is
exhaustive rather than a gate on shipping." A hole in a standard is a FIXME and
is honest; a hole in the framework is a defect. The IEEE 754 half is a hole of the
first kind today: no operation admits a floating format, because the applied map
has no magnitude coordinate to land a result at, and what wrapping means over a
union of magnitude ranges is an open canon question.

### What a parity default would be

Op asked whether the shipped default might follow one of the parity standards.
The record answers what each standard's default is, which is the half that is
answerable.

MATLAB `fi` defaults are documented and the crate already carries both halves of
them: `Floor` for rounding and `Saturate` for overflow. Both re-export cleanly, so
a MATLAB parity default is available today as `Adapt<Floor, Saturate>` with
nothing new written.

IEEE 754's default rounding is round-to-nearest-ties-to-even, which is arvo's
`half_even`, and it is in the vocabulary. Its overflow behaviour is to infinity
rather than to a saturating end, and arvo has no infinity: every format's
representable set is finite and `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point`
is about ends rather than about an unbounded point. So IEEE 754 cannot supply an
overflow default without a design decision arvo has not made, and its rounding
default can be taken today without one.

That asymmetry is the useful thing: of the two standards op named, one can supply
a complete default today and the other can supply half of one.

## b. How overflow behaviour is chosen, and where a wrapper would sit

### The three places it is decided today

The overflow policy is one of two coordinates of an adaptation, and the adaptation
is half of a declared signature. `arvo-format/src/adapt.rs:27-38` states it:

> A total map from the ambient domain onto a representable set. Two independent
> coordinates, because they answer different questions and because switching one
> off does not switch the other off.

`Adapt<R, O>` is the only implementor the crate ships, "because an adaptation is
its coordinates and there is nothing else for a member to vary". The policy
itself is a trait with a const, `arvo-format/src/overflow.rs:26-29`, and the
`Policy` enumeration beside it carries the corpus's members and says it is not
closed: "a member arriving later extends it rather than being refused by it". So
the overflow axis is open in the sense `content-kinds-are-open` asks for, and a
consumer may bring its own policy.

That is the first place. The second is the strategy, and it is where the record
goes quiet in a way that matters.
`ruling::wrap_or_clamp_stays_open_and_both_get_priced` holds open which of wrap
and clamp `Warm` names, in op's name, and the value crate's aliases take the
adaptation as a parameter with no default because of it:
`arvo-numeral/src/alias.rs` says in its own words that "No adaptation is a
default: which one `Warm` names for integers is unclaimed, so the consumer names
it." So today the strategy selects nothing on this axis and the alias passes the
choice straight to the consumer.

The third is the debug check, which is a different axis and is often confused for
this one. `Num<S, C>` carries `C: DebugCheck` with `Abort` and `Quiet`, and
`arvo-numeral/src/check.rs` calls it "an axis of its own, beside the signature".
It decides what happens on a debug build when something the typestate could not
catch goes wrong. It does not decide what the value becomes, which is the
adaptation's job. Keeping them apart is worth stating because op's question was
about the first and
`ruling::the_overflow_panic_is_permitted_and_bounded` is about the second.

### Where a wrapper would sit

The mechanism op floated, `Saturating<Foo>` or `Wrapping<Foo>`, cannot be a
newtype around a value, because the policy is not a property of the value. It
lives in the signature the value is keyed on. So the wrapper has to be a
type-level function from one signature to another that substitutes the overflow
coordinate and keeps the rest.

Whether that is writable is a compile question and it was sketched rather than
reasoned about. `mock/research/sketches/202611131030_overflow-wrapper-rebinding/`
carries it, and the outcome is WORKS on three counts and FAILS on the fourth.

The substitution is one type alias over the two halves of the signature, with no
feature gate and no trait of its own. The projected signature is still admitted,
which was the real question: `Sole`'s blanket impl discharges for it exactly as
for a written signature. It works generically, with one bound the caller has to
write out, `WithOverflow<S, O>: Sole`, because `S: Sole` proves nothing about a
sibling signature even one sharing its format. That bound is the ergonomic cost
and it is the thing to design against, since a wrapper appearing in a consumer's
signature drags it along.

What fails is the crossing. `Num::carrier` and `Num::from_carrier` are
crate-private, and no public door takes a value of one signature and gives one of
another. The public doors are `constant`, `adapted` and `slot`. So a consumer can
name `Saturating<S>` as a type today and cannot move a value into it. The wrapper
mechanism needs a door that does not exist, and what that door is is a design
question rather than a compile one.

One thing the sketch settles about that door: a free bit copy is correct here only
because this particular substitution leaves the format alone and therefore leaves
the carrier alone. A door spelled as a general signature change cannot be a copy.

### What the record says against the wrapper

Weighed against the record the wrapper has one real problem and it is not an
implementation one.
`ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`
and `ruling::a_composed_expressions_region_is_never_inherited_from_its_parts` are
both ratified and both say that nothing about a part's declared behaviour
transports to a composite for free. A wrapper reads as though it transports:
`Saturating<Foo>` looks like a modifier applied to a thing, and the ratified rows
say a composite declares its own region rather than inheriting one. The sketch
shows the type system agrees, since the bound does not come along for free. So the
wrapper's spelling is honest only if the bound stays visible, and a design that
hides it would be stating exactly the transport the canon refuses.

That is a finding against the convenience of the spelling rather than against the
mechanism. His intent, a default that is easy to override per case, is served by
the mechanism as sketched.

## c. Flexible numbers built on arbitrary choices, composable from scratch

His phrase names something the record has, in three pieces, and they are the
format spine, the placement and the strategy.

### The format spine

`ruling::the_format_spine_is_canon` is the piece that does the work. A format is a
representable set together with the ambient domain it sits in, declared as a
phase, a quantum per magnitude and a slot range, and membership is one predicate
rather than a family of them. The crate's own README states what that buys: "You
give a phase, a quantum per magnitude and a slot range, and integers, fixed point,
scaled integers and floating point all come out as points of that one thing.
Subnormals are not a special case, they fall out of taking the maximum at the
smallest magnitude, which is the test that the unification is doing real work."

That is the arbitrary-choices half of his sentence, stated precisely. The choices
are the coordinates, and the four shipped points under `arvo-format::points` are
instances of an open inventory rather than the inventory: the module doc says
"A new numeral joins by implementing `Format`, and none of these is privileged by
being here."

Arithmetic is stated the same way throughout, which is the other half of the
spine: an exact operation in the ambient domain composed with a named total
adaptation back onto the representable set. The adaptation is nameable and
reasonable-about rather than a flag hanging off an operation, which is what makes
the composition from scratch possible at all.

### The placement

`arvo-placement` holds where the bits go, and the separation is enforced rather
than stated: `arvo-format` does not depend on it, so no operation in the format
crate can be a function of the container. The README says it plainly: "There is
no machine container anywhere in this crate and you cannot reach one from here.
Every operation is a function of the width you declared, never of whatever the
value happens to sit in."

### The strategy

`arvo-strategy` carries the presets, and this is the piece whose shape is least
settled. The two two-expert proposals disagree about what a strategy is, as
`CHECK.md` section 5 records, and `ruling::the_strategy_set_is_not_closed_at_four`
holds the membership open. What is settled is the direction: a strategy is
selected by the consumer, and `ruling::the_strategy_is_what_makes_an_answer_correct`
fixes that it cannot change the denoted answer in the service of cost.
`proposal-the-later-topics::no_cost_model_may_move_an_answer` says the same from
the other side and is a proposal rather than canon.

### What a consumer writes today

Concretely, composing from scratch at head means writing a `Format` impl and
pairing it with an adaptation. Four associated items and one const: an ambient
domain, a quantum law, a slot family, and a phase. Then `Signature<F, A>` with an
`Adapt<R, O>`, and `Num<S, C>` over that if the signature passes `Sole`.

The MATLAB point in `standards.rs` is the worked example of exactly this and it
is short, which is the evidence that the composition is real rather than
aspirational: `Fi<W, F>` is a struct with no fields and a `Format` impl naming
`BinaryRationals`, `FractionLength<F>`, `Signed<W>` and `Phase::ZERO`. A consumer
bringing its own convention writes the same four lines.

What the consumer cannot do today is bring its own overflow policy and have it
admitted into the value tier. `Sole` requires the policy be wrapping or saturation
(`arvo-numeral/src/admit.rs`, the `ADMITTED` text: "The overflow policy is
wrapping or saturation, and the rounding mode reads no dither"). The format tier
is open on this axis and the value tier is not, and that gap is not recorded
anywhere in the registry. It is the sharpest thing this reading found about his
"composable from scratch", and it belongs in the plan.

## d. The contracts a consumer bounds on

This is the part where the answer is that the record demands something it does not
have.

`obligation::the_surface_expressible_as_contracts_before_anything_implements_it`
is the demand and it is op's:

> Every part of arvo's surface stated as a trait contract that holds and is
> checkable before an implementation of it exists, so a consumer can build
> against the shape while the implementation behind it is unfinished.

Its `why` names hilavitkutin as the live instance, a consumer in exactly that
condition now. Its `gap` is the finding: "One place enforces where traits may
live, which is narrower than the approach and does not reach the surface as a
whole. Nothing here says what a contract has to carry to be usable before its
implementation exists, and no row anywhere establishes that the surface can be
stated that way."

`ruling::the_trait_contract_structure_is_a_primary_paradigm` is the ruling under
it, and `CHECK.md` section 1.8 records that the note declining to promote it rests
on a lint and a crate layout that are both gone.

`obligation::composition_contracts_above_the_numeral` is the second demand, for
contracts over units bigger than one numeral, so that what a chain of operations
guarantees is expressible at all. Its `why` says op names it as half of what arvo
is for, beside the algorithm surface, and explicitly not the numeral alone. Its
gap is unusual and worth repeating: the obligation is one slug wearing four
demands, which two independent later readings each found and each split
differently, so the coverage tool reports ten proposals against one obligation and
reads as a demand nearly met when none of the four sub-demands has more than one
or two answering it.

Against those two demands, what exists at head is traits in the ordinary sense
(`Format`, `Adaptation`, `Rounding`, `Overflow`, `Slots`, `Quantum`, `Sole`,
`DebugCheck`) and nothing a consumer would bound an algorithm on. There is no
`Add`-shaped contract, no ordering contract, no contract naming what a chain
guarantees. A consumer writing a generic routine over arvo numbers today bounds on
`Sole` and calls inherent methods on `Num`, which is not a contract surface and
does not survive a second value type.

On arithmetic contracts on a standard, which is what op's second answer named as
arvo's own terrain, the registry is silent. The standards material in the record
is about expressing a convention's declaration, the width and scaling and
rounding-and-overflow pair, and `ruling::the_standard_is_parity_in_output_not_in_the_internals` is
about reproducing a standard's output. Neither is a contract a consumer bounds on,
and no row proposes one. That silence is not permission, and it is the clearest
place where his 13 November words ask for something the record has not begun.

One row speaks against the obvious wrong shape, and it is worth having.
`proposal-the-value-stance::the_old_numeric_contracts_crate_is_gone` sends
absolute value, reciprocal and square root to kaski for the first slice. His
sentence ends "definitely arvo terrain to write the actual arithmetic contracts
on the standard too", so that re-homing is against his direction, and `CHECK.md`
section 5 records it as the sharpest of the five live refusals.

## e. The old families, and what the record says of each

`proposal-the-value-stance.toml` carries one row per old crate, sixteen in all,
and they are the record's own account of this question. All are at `one_expert`,
so none is canon, and his 13 November words now stand above all of them: every
family returns in function and feature, with the shape and the taxonomy open to a
whole rewrite. Read with that in front, the rows sort into three groups.

### Returning, with a slice and a shape named

`arvo` the facade returns in slice one changed, as aliases of the value crate over
declared signatures, with no facade crate, so a consumer names the crate that owns
the thing; its floats return in slice two. His earlier answer the same day, "Decide
at the port", governs the facade question itself.

`arvo-bits` returns in slice one as head's `Bits<N>`, with no strategy parameter
and with `cast` in place of the old narrowing and widening. It is in the tree
already.

`arvo-strategy` returns in slice one changed, as presets bound over a declared
signature with their cells unsettled, the identity markers moved to the value
crate. Also in the tree.

`arvo-tensor` returns changed in slice one as the capacity crate, a count as
typestate with its array projection and a const count; its `Array`, `Matrix` and
`Enumerator` return changed in slice two.

`arvo-bitmask` returns changed in slice two, first of that slice, as `Mask`,
`BitMatrix`, `NodeId` and set-bit iteration over `arvo-bits` and the capacity
crate.

`arvo-graph` returns changed in slice two over the value crate's index type.
`arvo-sparse` returns changed in slice two over the index type and the capacity
crate. `arvo-spectral` returns changed in slice two, last of it, over the sparse
crate. `arvo-hash` returns changed in slice two, its hash width named by the
consumer and its result carried in `Bits<N>`.

Every one of these has an obligation under it naming a consumer's stated need:
`ordering_a_directed_acyclic_graph`, `a_sparse_adjacency_a_plan_can_be_built_on`,
`a_spectral_partition_of_a_dependency_graph`, `a_content_hash`,
`set_operations_over_a_fixed_size_bit_set`. So the return of this group is owed by
the obligation namespace independently of any row about the old crate.

### Refused as a crate, with the function re-homed

Three rows say a crate does not return and name where the work goes, and these
agree with his words once the distinction in `CHECK.md` section 5 is made.

`arvo-storage` does not return as a crate; it was the bare-primitive layer at the
bottom of every old chain, and the placement crate is where that work sits now.
`arvo-mask-contracts` does not return as a crate of its own; it was one trait
declaration, `MaskOps`. `arvo-refit` does not return; it was a re-export gateway
for `Narrow` and `Widen`, and head's `cast` is named as its replacement.

`arvo-transparent` is in this group too, and its refusal is already superseded
inside the file by `the_old_transparent_crates_work_is_each_value_types_own_doors`,
which says what it carried returns as the doors each value type declares for
itself, with no general transparency contract beside them and no door giving or
taking a carrier. That last clause is what the sketch in part b ran into.

### Refused outright, against his words

Two rows say a crate does not return and re-home nothing, and both carry the
function his second answer names as arvo's own.

`arvo-bits-contracts` held the bit-level trait declarations: `HasBitWidth`,
`BitAccess`, `BitSequence`, `BitLogic`, `BitPrim`, `UBitContainer`,
`IBitContainer`. That is the contract surface of part d, for the bit tier.

`arvo-numeric-contracts` held `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`
and the `Predicate` family. That is the arithmetic contract surface, and its
`instead` sends the first three to kaski.

`arvo-comb` is the fourth of the original refusals and it is already superseded by
`the_old_comb_crate_answers_to_a_stated_need`, which points at
`obligation::a_cost_dynamic_program` and says its return is owed while leaving
which crate meets it, under which name, undecided. That supersession is the model
for what the other two want.

### What this says for the plan

The families sort cleanly into work with an obligation already under it, work
whose home is a taxonomy question his words have opened, and the contract surface,
which is both refused by a live row and demanded by two obligations. The last of
those three is the one to do first, because every algorithm family in the first
group would be written against it.
