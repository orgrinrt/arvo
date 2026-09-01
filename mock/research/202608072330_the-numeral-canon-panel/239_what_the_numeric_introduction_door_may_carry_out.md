# Independent derivation: what the numeric-introduction door may carry out

**Answers** `question::what_the_numeric_introduction_door_may_carry_out` (topic
`operating_constraints`, decider `panel`). Cold, independent seat. Nothing here
was read against a parallel derivation.

## Verdict, stated first

None of the four registered options is correct as written. `Width` and `Bool`
cannot honestly close nine of the ten, `min_adt_const_params` is both the wrong
gate for the position and the wrong position for the question, narrowing the
exemption alone does not unblock an external implementor, and shrinking the
obligation runs directly into `ruling::the_operating_constraints_are_intents_and_rules`
("public API positions using the stack's own primitives rather than bare
integers, floats, `bool` or `usize` ... not to be questioned"). The answer is a
fifth option: retype every one of the ten to an arvo primitive, using no
compiler feature, and accept that the door's own "two types and no more"
sentence does not survive contact with its own trait definitions and has to be
corrected to name however many primitives the ten coordinates actually need,
which the evidence below puts at more than two but fewer than ten.

## First task: breaking the brief

Two claims in the brief were checked against source before anything else, per
`panels-argue-the-intent-not-the-wording.md`'s standing instruction.

**The nine-coordinate-type claim is false.** The brief states the shipped
`arvo-format` "now declares nine coordinate types beyond the two the design
names": `Radix(u32)`, `Exponent(i32)`, `Magnitude(u32)`, `MagnitudeCount(u32)`,
`Slot(i64)`, `SlotCount(i64)`, `Arity(u32)`, `Phase{i64,i64}`, `Fraction{i64,i64}`.

```
grep -rn "^pub struct\|^struct" mock/crates/arvo-format/src/*.rs
```

names every struct the crate declares: `Width`, `Bool`, the operation-family
markers (`Exact`, `Dither`, `Adapt`, `Signature`, `TowardZero`, `Floor`, `Ceil`,
`HalfUp`, `HalfEven`, `Stochastic`, `BinaryRationals`, `UnsignedBinaryRationals`,
`DecimalRationals`, `Wrap`, `Saturate`, `Clamp`, `Signed<const BITS: u32>`,
`Unsigned<const BITS: u32>`, `Constant<const EXP: i32>`,
`Indexed<const MIN_EXP: i32, const COUNT: u32>`), and a handful of test-only
markers. None of the nine named types exists, anywhere in the tree:

```
grep -c "Radix(u32)\|Exponent(i32)\|Magnitude(u32)\|MagnitudeCount(u32)\|Slot(i64)\|SlotCount(i64)\|Arity(u32)\|Phase{i64,i64}\|Fraction{i64,i64}" -r mock/
```

returns nothing. This does not bear on the answer below, which is reached
independently of it, but it does not get to stand uncorrected: the claim was
checkable in two greps and was wrong. (It is worth noting, without leaning on
it, that the shape it describes, a small family of coordinate newtypes paired
by role, is close to what the evidence below independently arrives at. That is
a coincidence of a plausible architecture, not a fact about the tree.)

**The `note` field's central claim is false for at least two of the ten.**
`question::what_the_numeric_introduction_door_may_carry_out` argues, in its own
prose rather than in a ratified ruling: "The associated constants move to
`Width` and `Bool` ... so nine of the ten close with no compiler feature and no
cost to a consumer." This is checked in full below and does not survive the
check. The registry's own prose is a `question` row's `note`, not a `ruling`,
so it is presumed wrong exactly like any other unratified artifact
(`expert-dispatch-defends-the-canon.md`'s provenance ladder), and it argues for
the option it accompanies. It was written that way honestly; it is still
wrong.

## What each of the ten actually is

`mock/crates/arvo-format/src/*.rs` read in full: `format.rs`, `ambient.rs`,
`quantum.rs`, `slots.rs`, `adapt.rs`, `width.rs`, `lib.rs`, `tests.rs`.
`Width` is `#[repr(transparent)] pub struct Width(u32)`
(`mock/crates/arvo-format/src/width.rs:25`), documented as "a count of bits ...
Used wherever the design says a width: a declared width, a carrier's capacity,
an access width, a stride" (`width.rs:19-21`), non-negative by construction and
by every existing use.

| Constant | Type | Trait | Proven sign in shipped, tested code | Can honestly be `Width` |
|---|---|---|---|---|
| `RADIX` | `u32` | `Ambient` | always non-negative (2, 10) | no: not a bit count, a numeration base |
| `SIGNED` | `bool` | `Ambient` | n/a | `Bool`, uncontested |
| `PHASE_NUM` | `i64` | `Format` | 0 or 1 in every shipped point | signed by type; untested negative |
| `PHASE_DEN` | `i64` | `Format` | 1 or 2, "never zero" | non-negative in practice, paired to a signed sibling |
| `BASE` | `i32` | `Quantum` | **negative**, proven | no |
| `SLOPE` | `i32` | `Quantum` | 0 or 1 in shipped code | signed by type; untested negative |
| `MAGNITUDES` | `u32` | `Quantum` | always positive, "never zero" | no: a count of magnitude classes, not a bit count |
| `MIN` | `i64` | `Slots` | **negative**, proven | no |
| `MAX` | `i64` | `Slots` | non-negative in shipped code, coupled to `MIN` | no: shares a comparison and a subtraction with `MIN` |
| `ARITY` | `u32` | `Operation` | always positive (2 in the one shipped instance) | no: a count of operands, not a bit count |

`BASE` is proven negative at `mock/crates/arvo-format/src/tests.rs:133` (`assert!(is_constant_family::<Constant<-4>>())`), `:137` (`Constant<-7>`) and `:144` (`exponent_at::<Constant<-4>>(0)` asserted to equal `-4`), and again at `mock/crates/arvo-format/src/tests.rs:384` (`type Quantum = Constant<-1>;`). This matches the design's own table: "Fixed point at fraction width `F` | `radix^-F`, constant | zero | one" (`mock/crates/arvo-format/DESIGN.md.tmpl:39`), and `Constant<const EXP: i32>`'s own doc: "`EXP` is the exponent, so `EXP = 0` is the integers and `EXP = -F` is fixed point" (`mock/crates/arvo-format/src/quantum.rs:35-37`). Fixed point is not a corner case this crate might someday support; it is one of the four points the format spine names (`ruling::the_format_spine_is_canon`), and its `BASE` is negative by definition. `MIN` is proven negative at `mock/crates/arvo-format/src/slots.rs` in the `admit_widths!` expansion (`const MIN: i64 = -(1i64 << ($w - 1));`), exercised by every one of the sixty-two admitted `Signed<N>` widths and asserted against directly at `tests.rs`'s slot-range tests, all fifty-one of which I ran clean (`cargo test -p arvo-format --lib`, 51 passed, 0 failed, reproduced locally, not otherwise committed since it is the crate's own existing suite and not a probe).

`MIN` and `MAX` are compared and subtracted together in the same `const` block (`slots.rs`'s `ADMITTED`: `Self::MIN <= Self::MAX`, `(Self::MAX as i128) - (Self::MIN as i128)`), so they cannot honestly take two different representations, one signed and one not; `MIN` being provably negative forces `MAX`'s type along with it regardless of whether `MAX` itself is ever negative in a shipped instance.

So the registry's count is wrong by a wide margin. Of the ten, exactly one
(`SIGNED`) unambiguously closes with the crate's two existing types. `BASE` and
`MIN` provably cannot be `Width`, a documented non-negative bit count, without
either corrupting `Width`'s own stated meaning or losing information. `MAX`
follows `MIN` by the coupling above. `SLOPE` and `PHASE_NUM` are typed signed
and are not proven negative in the current corpus, but nothing in the design
restricts them to non-negative either, and reusing `Width` for them carries the
same risk `BASE` demonstrates. `RADIX`, `MAGNITUDES`, `PHASE_DEN` and `ARITY`
are genuinely non-negative counts, but none of them is a *width* in the sense
`Width`'s own doc restricts itself to (a count of bits, a capacity, an access
width, a stride); folding "how many magnitude classes" and "how many operands"
and "what base the numeration counts in" into the one type built to mean "how
many bits" is exactly the failure mode `harness-the-type-system.md`'s
discipline exists to prevent: a type reused across unrelated meanings carries
no more information than the primitive underneath it, which defeats the point
of retyping at all.

## Testing option 2 directly: it is both misdirected and factually wrong

`question::what_the_numeric_introduction_door_may_carry_out`'s `bound` field
already establishes that the const-generic-parameter position is excepted by
`obligation::a_primitive_for_every_position_a_bare_number_would_take` ("A const
generic parameter is excepted") and that none of the ten is a const generic
parameter. So adopting a compiler feature to fix the const-generic-parameter
refusal does not touch the actual gap the question is about (the ten
associated constants), which needs no compiler feature at all: an associated
constant's type is unconstrained on stable Rust, proven by the crate's own
`Slots::WIDTH: Width` (`slots.rs:60`) and reproduced with zero feature gates in
`probe_2_associated_const_of_struct_type_needs_no_feature.rs`.

Having established it is answering a different question, it is also wrong on
its own terms. The topic file (`mock/design_rounds/202609011112/...topic....md`)
and the question's option 2 both name `min_adt_const_params` as the escape from
the const-generic-parameter refusal, exactly as the compiler's own diagnostic
suggests (`tests/ui/an_arvo_type_as_a_const_parameter.stderr`). Two things are
wrong with that:

**The name is not what is vetted.** `.claude/rules/unstable-features.md`
carries `adt_const_params` (tracking #95174) in the ALLOWED table, "largely
complete, 2026 stabilisation target ... sound", not `min_adt_const_params`.
Option 2's stated cost, "the gate has no row in the workspace's vetted set,
whose own rule is that an unvetted gate must not ship," is true of the literal
symbol `min_adt_const_params` and false in substance: the capability exists
under an already-allowed name.

**The two gates are not interchangeable, and the difference lands exactly on
`Width`.** `probe_3a_min_adt_const_params_refuses_arvo_formats_actual_width_shape.rs`
and `probe_3b_adt_const_params_accepts_the_identical_shape.rs` compile the
identical source, `arvo-format`'s real `Width` shape (`pub struct Width(u32)`,
a public struct with a private field, deriving `ConstParamTy`), once under each
gate. `min_adt_const_params` refuses it outright:

```
error: the trait `ConstParamTy` may not be implemented for this struct
  |
4 | pub struct Width(u32);
  |            ^^^^^ struct fields are less visible than the struct
```

`adt_const_params` accepts it with no error. So the gate the compiler's own
diagnostic suggests, and the one the topic file and the registry note both
name, is the wrong one: it would force `Width`'s field `pub`, which breaks the
crate's own stated invariant that `repr(transparent)` plus the `count`
accessor "is the whole observation surface, so the invariant this type carries
is what its constructors establish and nothing widens it" (`width.rs:37-42`).
Nobody who checked which gate actually compiles against arvo's own type would
have named `min_adt_const_params`. Both probes are committed with their raw
`rustc` output in `239_probes/`.

None of this makes option 2 correct once corrected. It establishes that the
const-generic-parameter position was never blocked by a vetting gap, only by
naming the wrong gate, and that the position does not need fixing at all under
the ratified exception. The two facts above are worth keeping regardless of
this question's answer, because the crate's own pinned refusal test and its
design's justifying prose both currently point at a gate that would not work.

## Testing option 3: correct as a hygiene fix, does not unblock anything

Narrowing `[primitive-introductions]` to the two types plus per-site
allowances for the rest changes what triggers the lint *inside*
`mock/crates/arvo-format`. It changes nothing about what an external crate must
write, because the lint an external implementor hits is driven by the *type*
the trait declares for `PHASE_NUM` and the rest, not by which crate is or is
not exempt. `[primitive-introductions]` at `mockspace.toml:1397-1408` is a
crate-scoped lint-skip; retyping a trait's associated constant is a source
change to the trait. The two are independent, and option 3 only does the
first. `unblocks` on the question row is explicit: "Whether the crate's public
traits can be implemented at all from outside it ... Ten hard errors today, one
per associated constant, in any crate that writes an impl." Option 3 leaves
every one of those ten hard errors exactly where they are for whatever
fraction of the ten does not also get retyped. As a hygiene measure paired
with retyping everything that can honestly be retyped, it is a reasonable
follow-on and not a competing answer; alone, it does not answer the question.

## Testing option 4 against the governing text directly

`ruling::the_operating_constraints_are_intents_and_rules` (`rung = "in_force"`,
`key = "I14"`) is the sharpest text in the registry on this exact point:

> Public API positions use the stack's own primitives rather than bare
> integers, floats, `bool` or `usize`.

and op's own quote behind it: "No std, no alloc, all that is explicitly already
in place and not to be questioned." This is not a ratified convergence subject
to being outweighed by a design's own aesthetic preference; the row's own
`kind` is `"intent"` and its status is that it is "already in place, enforced
by the mockspace lints and the workspace and repo rules, and ... not to be
questioned." A trait's public associated constant is squarely a public API
position and squarely not a const generic parameter, so it is squarely inside
what I14 forbids leaving bare. Option 4 proposes narrowing the *obligation* to
exclude exactly the position this crate happens to have left bare, which is
the shape `do-not-question-the-tier-above.md` names directly: design and code
do not get to shrink the intent above them to fit what was already built.
Option 4's own stated cost admits as much: "it makes the obligation smaller
than the sentence it came from, which says every public API position." That
sentence is op's, in force, and not this panel's to narrow.

## The fifth option

**Retype all ten associated constants to arvo primitives of their own, using
no compiler feature, and correct the design's "two types and no more" sentence
to name however many primitives the ten coordinates actually require rather
than the two it currently claims.**

What this concretely means, stated at the level the topic decision needs and
no further, since the exact type boundaries (one signed primitive shared by
`BASE`/`SLOPE`/`MIN`/`MAX`/`PHASE_NUM` versus several role-specific ones; how
`RADIX`/`MAGNITUDES`/`ARITY` are named) is source-and-doc-CL work for the round
that carries this, not a thing to over-specify here:

- `SIGNED` moves to `Bool`. Uncontested, zero new types.
- `BASE`, `SLOPE`, `MIN`, `MAX` and (conservatively, since its type admits
  negative values even though none is shipped) `PHASE_NUM` need a signed
  arvo primitive that does not yet exist anywhere in the stack. `MIN`/`MAX`
  and `BASE`/`SLOPE` are each a coupled pair; whether all five share one type
  or split into a slot-index family and an exponent family is exactly the
  kind of shape question `ruling::the_canon_does_not_police_what_shape_a_law_takes`
  (`I16`: "the canon does not police what shape a law takes ... case by
  case") reserves for the implementing round rather than settling here.
- `RADIX`, `MAGNITUDES`, `PHASE_DEN` and `ARITY` are non-negative counts that
  are not, in the crate's own stated sense, widths, and reusing `Width` for
  them would defeat the purpose of having retyped them at all. They need
  their own primitive or primitives, or a documented, deliberate widening of
  `Width`'s own stated meaning if the crate's authors decide the distinction
  is not worth a third type. Either is a design call, not a canon call.
- None of this needs `min_adt_const_params`, `adt_const_params`, or any other
  gate: every position in question is an associated constant, which is
  unconstrained on stable Rust, demonstrated in the crate's own shipped
  `Slots::WIDTH: Width` and reproduced in `probe_2`.
- The const-generic-parameter position is untouched, stays bare under the
  ratified exception exactly as today, and the pinned refusal test
  (`tests/ui/an_arvo_type_as_a_const_parameter.rs`) stays exactly as it is.

## What this costs and what it forecloses

**Cost.** The door widens past two types. This is a real cost against the
design's own stated narrowness, and the design's sentence has to be corrected
rather than quietly violated, because a design silently disagreeing with its
own trait definitions is exactly the shape `design-is-the-oracle.md` and
`the-canon-design-code-chain.md` warn against: a lower tier surviving a change
above it becomes a claim about a document that no longer holds. The correction
is cheap relative to the alternative (a new nightly dependency, or a narrowed
obligation), but it is not free, and it is the honest name for what closing
this gap requires. It also means a round of real design work: naming the new
primitive(s), their construction surface, and whether any existing consumer
(`arvo-placement`, `arvo-strategy`) reads these constants in a way a type
change would touch. I did not find any such read in either crate; grepping
both for `PHASE_NUM`, `PHASE_DEN`, `BASE`, `SLOPE`, `MAGNITUDES`, `MIN`, `MAX`
or `ARITY` returns nothing, so the blast radius looks to be `arvo-format`
alone, but that is a claim worth a second, deliberate check rather than resting
on an absence this dispatch happened to observe once.

**Forecloses.** It forecloses ever calling the door "two types" again without
that being false on its face; the design's own sentence has to change, which
this file recommends as an explicit, acknowledged design correction rather
than a silent one. It also forecloses option 4's shrink of the obligation
being available later as a fallback if the retyping round turns out to be more
work than expected: `ruling::the_operating_constraints_are_intents_and_rules`
already forbids it now, and nothing found here weakens that.

## What I could not settle

Whether `BASE`/`SLOPE`/`MIN`/`MAX`/`PHASE_NUM` share one signed primitive or
split into two or three is genuinely open and is not this dispatch's to close;
I16 reserves shape questions like this for the implementing round. I also did
not attempt to price the actual implementation (how many lines, how many
crates touched) since the question asked which option, not what it costs in
engineering hours, and the blast-radius grep above is a single observation
rather than an exhaustive check across the whole workspace (only
`arvo-placement` and `arvo-strategy` exist downstream of `arvo-format` today,
per `mock/crates/`, so the search space was small, but I did not extend the
grep to `mock/research/` or design documents that might already assume the
bare types in prose rather than in source).

## Probes

`239_probes/`:

- `probe_1_adt_const_params_closes_the_parameter_position.rs` (+ `.out`): the
  const-generic-parameter position, closed under `adt_const_params`, with the
  correct external-crate construction syntax (`Width::bits(8)`, not the
  private-field `Width(8)` the registry note's cost line names).
- `probe_2_associated_const_of_struct_type_needs_no_feature.rs` (+ `.out`):
  confirms an associated constant of struct type needs zero feature gates.
- `probe_3a_min_adt_const_params_refuses_arvo_formats_actual_width_shape.rs`
  and `probe_3b_adt_const_params_accepts_the_identical_shape.rs` (+ shared
  `.out`): the gate the topic file names refuses arvo's real `Width`; the gate
  actually vetted in this workspace accepts it.

All three ran under `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the pinned
toolchain (`rust-toolchain.toml`), from inside this worktree.
