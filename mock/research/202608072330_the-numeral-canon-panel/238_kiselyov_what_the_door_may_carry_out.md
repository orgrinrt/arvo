# 238. Kiselyov: what the numeric-introduction door may carry out

Seat 238, on `question::what_the_numeric_introduction_door_may_carry_out`,
`topic = "operating_constraints"`, `decider = "panel"`. Derived from the
registry and from the tree at `0cac9beb`, which is where this worktree was cut
and which resolved to `origin/dev` in this clone when I checked.

Eight probes, all committed in `238_probes/` with their raw output beside them.
Every one carries the case that had to fail before its number counted, and two
of them caught defects in themselves before they produced a number. Every
conclusion below names the instrument that established it and the region that
instrument ran over.

The short answer, so a reader can stop here if that is all they want: **none of
the four recorded options is right, and the first one is not merely costly but
arithmetically impossible.** What the door may carry out is one
`repr(transparent)` type per coordinate of the ratified parameterisation, and
the bound on the door stops being a count of types and becomes a rule about
positions. That is a fifth thing and it is derived below.

## 0. The two gates

**Canon gate: passed.** Checked against the rows the brief names as governing,
each read in full from `mock/registry/` before anything else.

`ruling::the_panel_finishes_the_canon_without_him` is `rung = "ratified"`,
`ratified_by = "op"`, and puts every remaining canon question with the panel:
"Every remaining canon question is the panel's... Nothing is parked awaiting
him." The question row carries `decider = "panel"`. So the work is licensed and
nothing below is routed to him.

`ruling::the_format_spine_is_canon` is `rung = "ratified"`,
`ratified_by = "both"`, and it is the row this whole question turns on. It
ratifies four propositions together, the last of which is that "The concept is
closed and the inventory of admitted instances is open", and it names
`proposal::the_concept_is_closed_and_the_inventory_is_open` as one of the four
it ratifies. That proposal says a new instance "earns admission by supplying the
concept's obligations rather than by amending the canon".

`ruling::the_operating_constraints_are_intents_and_rules` is `rung = "in_force"`
and carries op's own verbatim. Its `says`: "public API positions using the
stack's own primitives rather than bare integers, floats, bool or usize. They
are already in place, enforced by the mockspace lints and the workspace and repo
rules, and they are not to be questioned."

Nothing below reopens any of those. What is below is the consequence of holding
all three at once, which is where the tension is.

**Test gate: run, and it does not disqualify the suite, but it produced a
finding that is load-bearing for the answer rather than incidental to it.**

`cargo mock test`, which runs every tree mockspace owns rather than only the
members: 603 passed, 0 failed, 13 ignored, 8 of 9 trees green. The ninth is
`mock/benches`, which fails on variant manifests and is not mine and not a
defect: `mock/benches/variants/fnv1a/Cargo.toml:1-12` carries a marker saying the
variant has not built since 2026-08-08, that it inherits workspace dependencies
naming deleted crates, that it is kept because its committed results are
evidence the panel cites, and that repairing it would mean "an arm invented to
make a manifest parse measures nothing". That is a catalogue-red stated in the
right place.

`cargo test --workspace --all-features` over `mock/` alone: 81 passed, 0 failed, 1
ignored, across the three crates that exist. I read the body of every test in
`arvo-format`, which is the surface this question is about: `src/tests.rs` (601
lines), `src/apply/tests.rs`, `tests/compile_fail.rs` and both `tests/ui/`
cases. It is a good suite. It carries explicit controls
(`the_control_the_widths_produce_different_slot_ranges`), it sweeps the whole
admitted-width matrix rather than a sample
(`the_law_admits_every_range_this_crate_ships` over `ADMITTED_WIDTHS`), it keeps
a reviewer's lying construction permanently on the right side of the suite
rather than in a scratch file (`src/tests.rs:484-508`), and its two refusals are
`trybuild` cases with their diagnostics asserted rather than prose claiming a
refusal.

The finding is one test, and it is the one this question is about.

**`the_format_inventory_admits_a_member_this_crate_does_not_know_about`, at
`mock/crates/arvo-format/src/tests.rs:380-397`, is setup that helps, in the
exact sense the test gate names.** It is the only test anywhere asserting the
ratified open-inventory clause for `Format`. It declares `struct Ternary` and
implements `Format` for it, **inside `arvo-format`**, which is the one crate in
the stack where `mockspace.toml:1408` turns the bare-primitive rule off. So the
test proves the type system admits an outside format and is silent about the
rule refusing one, and the path that breaks is never entered.

It is narrow a second way that nobody has said out loud. `Ternary` borrows
`DecimalRationals`, `Constant<-1>` and `Signed<3>` from the crate it is meant to
be outside of, so it writes two of the ten associated constants at issue and
none of the other eight. A format that borrows nothing writes all ten. Measured,
in `k8`: appending `Ternary` verbatim to `arvo-placement`'s sources produces two
gate findings, and appending a format that supplies all four traits itself
produces ten.

I did not return early on this. The gate exists so a dispatch does not build on
a suite that lies, and I build on nothing this test says: I depend on the
opposite proposition and prove it with my own instruments. Refusing here would
also be circular, because the repair to this test is the thing the question
decides. So it is reported as a finding, with its repair named in section 7, and
the work proceeded.

## 1. The brief's stated fact is false at this commit, and I checked before I was told

The dispatch handed me one fact it said postdates the question row: that shipped
`arvo-format` now declares nine coordinate types beyond the two the design
names, listing `Radix(u32)`, `Exponent(i32)`, `Magnitude(u32)`,
`MagnitudeCount(u32)`, `Slot(i64)`, `SlotCount(i64)`, `Arity(u32)`,
`Phase{i64,i64}` and `Fraction{i64,i64}`, and told me to verify it against the
tree.

**It does not hold at `0cac9beb`.** A grep for every `pub struct`, `pub enum` and
`pub trait` in `mock/crates/arvo-format/src/` returns two stack-owned scalar
types and no others: `Width(u32)` at `src/width.rs:25` and `Bool(bool)` at
`src/width.rs:89`. A grep for the nine names across `mock/crates/` returns
nothing, against a positive control on the same pattern shape that returns
`Width` and `Bool` from the same command. `git rev-parse` says my `HEAD` and my
`origin/dev` are the same object, and `git merge-base --is-ancestor` confirms
`origin/dev` is not ahead of my base.

The coordinator sent a correction mid-run saying the same thing and naming
`feat/the-format-contract-is-spelled-in-stack-types`, pull request #55, as where
those types live. I have not read that branch and this file rests on nothing
from it. Two things follow that are worth saying rather than filing.

**First, my own derivation of the coordinate set, done from the positions in
section 5 before the correction arrived, produces that same list.** That is
interesting and it is not corroboration. Under the provenance ladder an unmerged
branch is agent output, and agreement between my derivation and it is agreement
between two unratified artifacts, which is the shape shared drift takes. What it
is evidence for is that the list is not arbitrary: two routes to it exist, one of
them being an enumeration of the crate's own public positions, which is a
mechanical procedure with no room for taste in it.

**Second, the coordinator's correction names `origin/dev` at `1e2e3d6b` and my
clone's `origin/dev` is `0cac9beb`.** I did not fetch, deliberately, because a
fetch brings the other seat's branch into my refs and this is a blind read.
Everything below therefore states `0cac9beb` as its tree and nothing here claims
anything about a later one.

## 2. What the ten actually are, measured rather than taken from the row

The row's `note` lists ten: `PHASE_NUM` and `PHASE_DEN` on `Format`, `RADIX` and
`SIGNED` on `Ambient`, `BASE`, `SLOPE` and `MAGNITUDES` on `Quantum`, `MIN` and
`MAX` on `Slots`, `ARITY` on `Operation`.

`k4` enumerates trait associated constants declared with a bare primitive across
`arvo-format/src/`, with two controls. It returns exactly those ten, at
`adapt.rs:95`, `ambient.rs:18`, `ambient.rs:25`, `format.rs:41`, `format.rs:46`,
`quantum.rs:27`, `quantum.rs:30`, `quantum.rs:34`, `slots.rs:54`, `slots.rs:57`.
The row is right and its `note` is right.

`k1` runs the real bare-primitive lint, the pack the repository pins at
`f34cc1b0`, over the impl an outside crate has to write. Four controls, all
zero: a clean source, the const generic parameter position, the shipped impl
under the introductions map that names `arvo-format`, and the repaired impl in a
checked crate. Arm A, the shipped impl in a checked crate, reports **exactly
ten**, one per constant, at the ten lines. Both `arvo-types-only` and
`no-bare-numeric` agree, which also settles the pack's own claim that the two
are equivalent today.

So the row's `unblocks` holds as written: ten hard errors, one per associated
constant, in any crate that writes an impl. That is measured now rather than
argued.

Two of those controls earn their place. **C2** puts `const BITS: u32`,
`const EXP: i32` and `const PHASE: i64` in parameter position and reports zero,
so the ten are a fact about associated constants and not about the tokens
appearing anywhere. **C3** applies the `[primitive-introductions]` map and the
same source reports zero, so the exemption is what silences the introducing
crate rather than something else about it.

## 3. The exemption is a crate-name lookup and a whole-crate skip

The question's `bound` says "the exemption itself is crate-scoped rather than
position-scoped, which is the gap the question sits in". That is right, and it is
worth pinning where it lives, because one of the four options proposes to change
it and no mechanism exists.

`mockspace.toml:1396-1408` declares `arvo-format = ["numeric"]`. In the pinned
pack, `src/lints/arvo_types_only.rs:78` is the whole of it:

```
if crate_introduces_category(ctx, categories::NUMERIC) { return Vec::new(); }
```

and `src/util.rs:60-65` is a `BTreeMap` lookup on `ctx.crate_name`. There is no
position, no type list, no file list and no per-item form. The exemption is on or
off, per crate.

`k3` measures what it is carrying. Both controls hold: `arvo-placement` and
`arvo-strategy` report zero with no exemption applied, and `arvo-format` reports
zero with it applied. With the exemption taken away, `arvo-format` reports **128
lines**: 82 in non-test source and 46 in its two test modules. `k4` narrows the
non-test half to **59 public positions** in declaration syntax, which includes the
crate's own impls of its own traits and the four declared unwrap doors on `Width`
and `Bool`.

**So the question names ten of a hundred and twenty-eight.** The ten are the
special ones and section 5 says why, but a reader should hold both numbers.

## 4. The four options, each attacked

### Option 1 is impossible, not merely costly, and the row's own note argues for it

The option says the ten move to `Width` and `Bool`, "so nine of the ten close
with no compiler feature and no cost to a consumer", with `SIGNED` as the tenth.
The design round that filed the question says the same thing at
`mock/design_rounds/202609011112/202609011112_topic.the-one-position-the-language-refuses-to-repair.md`:
"The second needs no gate and closes nine of the ten." The row's `note` then
says the first option "is the one that makes them agree".

`Width` is `pub struct Width(u32)` at `src/width.rs:25`, its only constructor is
`Width::bits(n: u32)`, and its own documentation at `src/width.rs:18` says it is
"A count of bits", "a count and not a value in any format". So:

- `Slots::MIN` is `-(1i64 << (BITS - 1))` on every signed width the crate ships.
  Negative.
- `Quantum::BASE` is `EXP`, and the crate's own doc says `EXP = -F` is fixed
  point at fraction width `F`. Negative, and negativity is the case that
  distinguishes fixed point from the integers rather than an edge of it.
- `Slots::MAX` at width 62 is `4611686018427387903`, which does not fit `u32` at
  all, sign aside.
- `Format::PHASE_NUM` is signed and 64-bit; `Biased` sets it from a `const
  PHASE: i64` parameter.
- `Quantum::SLOPE` is signed.

`k2` compiles this against the real crate rather than asserting it. Two controls
build: the shipped shape written from outside, including `const WIDTH: Width =
Width::bits(8)`, and `const SIGNED: Bool = Bool::TRUE`. Three arms are refused:

```
error[E0600]: cannot apply unary operator `-` to type `u32`
  const MIN: Width = Width::bits(-128);

error: literal out of range for `u32`
  const MAX: Width = Width::bits(4611686018427387903);

error[E0600]: cannot apply unary operator `-` to type `u32`
  const BASE: Width = Width::bits(-4);
```

**Six of the ten cannot be typed `Width` at any value the crate actually uses.**
`PHASE_NUM`, `PHASE_DEN`, `BASE`, `SLOPE`, `MIN`, `MAX` are signed or 64-bit or
both. Of the remaining four, `SIGNED` genuinely closes with `Bool`, and `RADIX`,
`MAGNITUDES` and `ARITY` would fit inside a `u32` while being category errors: a
radix of ten is not a count of bits, a count of magnitudes is not a count of
bits, an arity is not a count of bits. Writing them as `Width` states something
false in the type, which is the one thing a newtype is for.

So the correct figure is the mirror of the one in the row and in the topic file.
**One of the ten closes with the types the door has. Nine need something that
does not exist.** The option's stated cost, "a widened `Width` and `Bool`
surface", is not the cost; the cost is a type per coordinate, which the design's
own sentence forbids, and the option as written does not reach them.

I want to be exact about what is wrong here, because the topic file is careful
elsewhere and this is not carelessness. It reasoned from one instance,
`Ambient::SIGNED: bool` with `Bool` eleven lines away, which is the sharpest and
also the only one that works, and generalised from it without checking the other
nine. The generalisation is one `cargo build` away from refutation and nobody
ran it.

### Option 2 is refused, and both halves of the row's stated cost are wrong

The option says the whole surface moves and the crate holds
`min_adt_const_params`. Its stated cost is two things: a consumer writes
`Signed<{ Width(8) }>`, and "the gate has no row in the workspace's vetted set,
whose own rule is that an unvetted gate must not ship".

**The second half is false.** The workspace rule `unstable-features.md`, at line 55, carries a
row for `adt_const_params`, in the **Allowed** tier, described as "Largely
complete", 2026 stabilisation target, with the unsound part split out into
`unsized_const_params` which has its own row in the forbidden-or-watch section at
line 90. `min_adt_const_params` is the name the compiler's help text emits on
this pin, and `k6` measures what each of the two actually does:

- No gate: refused, reproducing the crate's own `tests/ui` case.
- `min_adt_const_params`: the feature name is accepted and the derive is not.
  `error: the trait ConstParamTy may not be implemented for this struct ...
  struct fields are less visible than the struct`. It needs `pub struct
  Width(pub u32)`, which destroys the encapsulation the design names at
  `src/width.rs:41` as "the whole observation surface".
- `adt_const_params` with the field still private: **builds.**
- `min_adt_const_params` with a public field: builds, at that price.

So the gate the option names is the unvetted one that also costs the newtype's
privacy, and the vetted one works with the field private. The compiler's help
text points at the wrong one of the two, which is presumably how the topic file
came to name it.

**The first half is also wrong, and this is the part worth keeping.** The
option's implicit worry, and the topic file's explicit one, is that
`obligation::the_unstable_machinery_does_not_reach_a_consumer` kills it: "a
crate outside this one declaring its own format would have to carry
`min_adt_const_params` to write the arvo type in that position, so the repair
moves the cost onto exactly the crate the obligation protects."

That obligation's own `gap` field says the containment question "is exactly the
open question, and it is unmeasured". `k7` measures it, for this one gate. A
door crate holds `#![feature(adt_const_params)]` and declares `pub struct
Signed<const BITS: Width>`. A consumer crate with no feature attribute at all
names `Signed<{ Width::bits(8) }>`, reads `<Eight as Slots>::WIDTH` back with its
value, and writes its own generic `const fn width_of<S: Slots>() -> Width`. It
builds, and the values cross with their values rather than merely the names: 8
and 13 read back as 8 and 13. The control is a `trybuild` case in that same
consumer declaring an ADT const parameter of its own, which is refused, so the
consumer really is ungated.

Predicate on that: `gate = adt_const_params`, `toolchain = nightly-2026-05-28`
(the pin), `position = a const generic parameter in a public signature`,
`consumer feature attributes = none`. It says nothing about `generic_const_exprs`,
which is the gate the obligation's own gap sentence names, and nothing about any
other.

**So option 2 is not refused by the obligation and is not refused by the vetting
rule. It is refused by op's own clause and by nothing else.**
`obligation::a_primitive_for_every_position_a_bare_number_would_take` carries his
words: "No bare usize other than in const generics for smoother and more
ergonomic api, and even there, only when truly painful otherwise." The exception
exists because the const generic parameter position is where the bare form buys
ergonomics, and `Signed<{ Width::bits(8) }>` at every declaration site in every
consumer, forever, is the archetypal instance of the cost that clause is about.
The escape being available and contained does not make the alternative
unpainful.

That is a judgement rather than a derivation and I flag it as one in section 8.
What is not a judgement is that the two reasons the row gives are both false, and
a later expert should start from the measurement rather than from them.

### Option 3 has no mechanism and does not answer the question

"The exemption narrows from the crate to the two types the design names."
Section 3 is the refutation: the exemption is a `BTreeMap` lookup on the crate
name returning a whole-crate skip, in a pack that lives in another repository.
There is no position scope, no type scope and no file scope to narrow to.
Implementing one is a change to `mockspace-extra-lints`, which is out of arvo's
tier entirely.

And it does not answer what was asked. Narrowing the exemption says which lines
get checked; it says nothing about what the ten constants become when they are.
Whatever answer that question gets is the same under a narrowed exemption as
under the present one, plus a lint-pack change. Its own stated cost, per-site
allowances, is what the pack's own documentation says to avoid:
`src/lints/arvo_types_only.rs:41-45` calls the allowance appropriate only for a
foreign-crate boundary and says "Prefer dropping the crate over a long-lived
allowance."

The one thing in it worth keeping is its instinct, that the exemption's extent
should equal the set of positions it is for. Section 7 keeps that instinct and
pays for it differently.

### Option 4 makes a ratified ruling false and is the only one that is unavailable outright

"The traits stay as they are and the obligation records that a trait's associated
constants are outside what it reaches."

`ruling::the_format_spine_is_canon` is ratified and says the inventory of
admitted instances is open, and the proposal it ratifies says an instance "earns
admission by supplying the concept's obligations". Supplying them means writing
ten associated constants. `k1` arm A says writing them in a crate with no
exemption is ten hard errors at every gate. **So the inventory is open in the
trait system and closed by the rule, today, and option 4 is the proposal to keep
it that way and write the closure down.**

The option's own cost sentence concedes most of this ("it leaves the crate
exporting the refusal into every implementor") and understates the rest: it is
not a smaller obligation, it is a ratified ruling that stops being true. That is
not a cost to weigh, it is the reason this question exists.

`slots.rs:52-63` is worth reading beside it. That trait's own documentation says
"The trait is open and an outside crate may implement it", and records that an
earlier version of the sentence claimed otherwise, "which was false and which
nothing enforced". The crate is already on record about this class of claim.

## 5. The answer

**The door carries out one `repr(transparent)` type per coordinate of the
ratified parameterisation, and the bound on the door is a rule about positions
rather than a count of types.**

Stated as the rule a later reader can check: **in the crate that introduces the
numeric category, a bare primitive may appear in the constructor and the
accessor of a primitive that crate itself defines, and in the type of a const
generic parameter. Nowhere else in a public position.** Bodies, private items and
the const generic parameter position stay as they are.

Three things fall out of that and each is checkable.

**The set of types is derived, not chosen.** It is one per coordinate that
appears at a position the rule reaches, so nobody has to defend a number. Read
off `arvo-format`'s public positions, the coordinates are: a bit count (`Width`,
exists), a truth value (`Bool`, exists), a radix, an exponent of the radix, a
magnitude index, a magnitude count, a slot index, a slot count, an arity, and a
rational in units of the quantum. Ten coordinates, two of which are already
there.

**Naming a coordinate narrows what crosses the boundary rather than widening
it.** This is the part I think the row has backwards. The door's width is not the
number of type names in `width.rs`; it is the number of concepts that cross the
boundary in a form a checked crate cannot write. Nine coordinates already cross,
as bare integers. An `i64` at `Slots::MIN` admits every 64-bit value and every
64-bit operation; a `Slot` admits slot indices and whatever the door decides a
slot index may do. The design sentence at `DESIGN.md.tmpl:235`, "two types, no
arithmetic beyond what a width derivation needs", was written when two
coordinates had been noticed, and its second and third clauses are the durable
part. The count is not.

**And "two types and no more" is not canon.** A grep for the phrase across
`mock/registry/*.toml` returns hits only inside the question row's own options
and note. It is a sentence in `mock/crates/arvo-format/DESIGN.md.tmpl`, which is
the design tier, derived from the canon and presumed wrong where it conflicts.
The canon says nothing about how many types the door holds. So this is not a
design being overridden by a preference; it is a design sentence that no canon
text supports, contradicted by its own crate, which carries the other nine
coordinates already and merely leaves them unnamed.

`k5` is the existence proof, because an intent that has not been shown doable is
a wish. Two crates: a door holding ten coordinate types with the pattern above,
and a crate outside it with no exemption declaring a format of its own. The
outside crate implements `Ambient`, `Quantum`, `Slots`, `Format` and `Operation`,
at a radix of three, a negative exponent, a signed eight-bit range and a
half-step phase, and names no machine type on any line. It builds. Its
assertions hold and are the ones the shipped crate's own laws assert: slot 0 is
in range, slot 200 is not, magnitude 1 is not one a constant law ranges over, a
half-step phase takes the additive identity off the grid, the radix reads back as
three, the slot count as 256, the arity as two. The mutation arm flips the phase
and watches the suite go red, so the assertions are load-bearing. Every
coordinate is the size of what it wraps, with a control that a non-transparent
newtype is not. And the real lint, run over both crates with no exemption
anywhere, reports 28 findings in the door and **zero in the crate outside it**.

The membership predicate stays a free `const fn` over the coordinates and stays
evaluable at stage zero: `k5`'s outside crate holds seven `const` items whose
values are the answers, so the compiler has answered them before anything runs.
`Slots::ADMITTED` keeps every one of its assertions, with the `i128` arithmetic
moved onto the accessors, which is where the door's own primitive is allowed to
be unwrapped.

**And the const generic parameters stay integers.** That is the position op
excepted, `k1`'s C2 confirms the lint agrees, and `tests/ui/an_arvo_type_as_a_const_parameter.rs`
stays exactly as it is, still red, still pinning the refusal. Nothing here needs
a compiler feature.

## 6. What it costs

**Eight new coordinate types and their two-line surfaces, plus the retyping.**
Eight rather than nine if one rational type serves both the phase and the
remainder that `Exact` and `Dither` carry, nine if those are two types. `k5`
uses one and I have no argument that decides it. `k4`
counts 59 public positions in non-test declaration syntax and `k3` counts 128
lines the exemption carries. Not all 128 move: bodies stay, the const generic
parameters stay, and the constructors and accessors are the ones that must keep
their machine type. On `k5`'s scale the door's own findings went from what would
have been two types' worth to 28 lines, which is the shape of the cost rather
than a prediction of the real crate's number.

**A trait shape change, in one place.** `PHASE_NUM` and `PHASE_DEN` are two
constants that are one coordinate; a `Phase` carrying both makes them one
constant. That is a change to what `Format` declares rather than a retyping of
it, and it is the only such change the answer needs. Ten constants become nine.

**The design document's sentence has to be rewritten**, which under the canon,
design, code chain means the code beneath it is written afresh from the changed
design in the same round. The crate is small enough that this is one round rather
than a migration.

**More ceremony per line, fewer lines.** An implementor writes
`const PHASE: Phase = Phase::of(1, 2);` where it wrote two constants, and
`const BASE: Exponent = Exponent::of(-4);` where it wrote `const BASE: i32 = -4;`.
Whether that is better to write is not something I can settle without a consumer
and I do not claim it. What I claim is that it is writable at all, which the
present shape is not.

**What it forecloses.** Reading the door as a fixed count, permanently: a
coordinate discovered later gets a type and the door grows by one with no
decision to make. And typing a coordinate as a numeral: these are declaration
coordinates, not values in a format, and `src/lib.rs:19-22` says there is no
machine carrier in the crate and none reachable from it. A `Slot` that became a
numeral would collapse that boundary.

**What it does not foreclose.** Option 2. The gate is vetted, contained and
measured now, so adopting it later is a separate decision taken on ergonomic
grounds with the technical question already answered.

## 7. How the rule gets enforced, because a rule nobody checks is a comment

I will not propose an invariant kept by vigilance. The exported half of this rule
needs no new mechanism at all.

**Move the open-inventory test to the other side of the door.**
`the_format_inventory_admits_a_member_this_crate_does_not_know_about` currently
sits inside the exemption, which is the test-gate finding in section 0. Put a
format that borrows nothing in a checked crate, and the existing gate enforces
the ten automatically, because an implementor in a crate with no exemption is
exactly what fires. `k8` measures it: `arvo-placement` reports zero unmodified,
two with `Ternary` appended verbatim, and **ten** with a format that supplies all
four traits itself.

That means the repair lands as a red gate that stays red until the coordinates
are typed, which is what
`strict-by-design-quality-pressure` calls the failing state being the
specification. It costs one test moved and it makes the ratified open-inventory
clause checkable for the first time.

**The contained half wants a lint of arvo's own**, in `mock/lints/`, permitting a
bare primitive in the introducing crate only on a line that is a constructor or
accessor of a type declared in that crate, or a const generic parameter type.
That is writable here: five source-side `CrateLint`s already live there and
`mock/lints/crate_lint_testkit.rs` is the harness. I have not written it, because
it enforces a rule that is not yet ratified and writing the enforcement before the
decision would be the wrong order. It is named so the next round does not have to
find it.

One mechanical fact for whoever writes it: **a `#[cfg(test)]` module is under
`src/` and is scanned like any other source.** I learned this the hard way in
`k5`, where a layout assertion comparing a coordinate against `u32` produced ten
findings in a crate that had none. The assertion belongs in the door, which is
also the right place on the merits, since `repr(transparent)` is the door's
guarantee to make.

## 8. What I could not settle

**Whether the ergonomic clause really refuses option 2.** I hold that it does and
I hold it as a judgement rather than a derivation. The clause is op's, the
position is exactly the one it names, and the cost is paid at every declaration
site forever. But he wrote it against a background where the alternative was
believed to be either impossible or to leak a gate into consumers, and both of
those are now measured false. A second reader should form its own view from
`obligation::a_primitive_for_every_position_a_bare_number_would_take` and from
`k6` and `k7` before this is treated as settled, and if the two of us disagree
the call goes to op rather than to a third seat.

**Whether `Exponent` covers both `BASE` and `SLOPE`.** A slope is a difference of
exponents rather than an exponent, and the two are in the same units, so one type
is defensible and two is defensible. `k5` uses one. I have no argument that
decides it and it is small enough to be the implementing round's.

**Whether the door should be its own crate.** Option 3's instinct, paid for
differently: a crate whose entire content is the coordinate primitives would make
the exemption's extent equal the position set mechanically rather than by a rule,
and `arvo-format` would then be checked like anything else. It is attractive and I
declined it. No canon text asks for it, the position rule reaches the same
guarantee inside one crate, and it puts the coordinates in a different crate from
the traits they parameterise. It is unpriced and I did not price it. The next
expert attacking from a different angle starts from that paragraph.

**Whether the other 118 lines matter as much as the ten.** They are the same
class and the ten are the urgent ones, because only the ten make a ratified ruling
false. I have ordered them that way. I have not established that the remaining
positions are worth their cost, only that the obligation's words reach them.

## 8b. The predicates

Every finding above, with the region it holds in stated over each dimension that
could move it. A dimension not listed does not hold at all, which is the reading
`every-finding-carries-its-predicate` fixes, so the absences below are meant.

**F1. An outside crate implementing the five traits takes exactly ten hard
errors.**
`holds for: crate = any crate not named in [primitive-introductions], lint pack
= mockspace-extra-lints @ f34cc1b0, lints = arvo-types-only and no-bare-numeric
both, gate = commit, build and push (the pack's default HARD_ERROR at all
three), format = one supplying all five traits itself, threads = 1`.
Instrument: `k1` arm A, four controls at zero, and `k8`'s third arm over the real
`arvo-placement` sources.

**F2. `Width` cannot carry six of the ten.**
`holds for: coordinate in {PHASE_NUM, PHASE_DEN, BASE, SLOPE, MIN, MAX},
Width = the shipped repr(transparent) over u32 with Width::bits(u32) as its only
constructor, value = any the crate's own impls produce, toolchain =
nightly-2026-05-28, threads = 1`.
Instrument: `k2`, three compile-refusals with two controls building. Stated as
six rather than nine because the other three (`RADIX`, `MAGNITUDES`, `ARITY`)
fit the payload and fail on meaning rather than on arithmetic, which is a
judgement and is marked as one in section 4.

**F3. An `adt_const_params` const parameter in a public signature does not reach
a consumer.**
`holds for: gate = adt_const_params, toolchain = nightly-2026-05-28, position =
the type of a const generic parameter on a public item, consumer feature
attributes = none, consumer use = naming the type, reading its associated
constant, and writing a generic function over the parameter, threads = 1`.
Instrument: `k7`, with a `trybuild` control confirming the consumer is ungated.
It says nothing about `generic_const_exprs`, nothing about any other gate, and
nothing about a const parameter appearing in a trait bound rather than on an
item, none of which were varied.

**F4. `min_adt_const_params` refuses a newtype whose field is private.**
`holds for: gate = min_adt_const_params, toolchain = nightly-2026-05-28, type =
a repr(transparent) tuple struct with a private field deriving ConstParamTy,
threads = 1`.
Instrument: `k6`, against three sibling arms including `adt_const_params` on the
same type, which builds.

**F5. The exemption carries 128 lines in `arvo-format` and both checked crates
carry none.**
`holds for: tree = arvo @ 0cac9beb, lint = arvo-types-only, scan surface =
every .rs under each crate's src/ including cfg(test) modules, introductions map
= the one mockspace.toml declares, threads = 1`.
Instrument: `k3`, with three controls. The figure moves with the tree and is a
measurement rather than a constant.

**F6. The shape in section 5 builds, computes the shipped answers, and leaves a
crate outside the door with nothing for the lint to report.**
`holds for: coordinates = the ten in section 5, door = a crate holding all of
them with one constructor and one accessor each, outside crate = one
implementing Ambient, Quantum, Slots, Format and Operation with no feature
attribute, radix = 3, exponent = -4, slot range = signed 8-bit, phase = 1/2,
toolchain = nightly-2026-05-28, threads = 1`.
Instrument: `k5`, with a mutation arm, a second-format control, a layout control
and the real lint over both crates.

It is an existence proof at one point of the parameterisation. It does not
establish that every format in the shipped `points` module retypes cleanly, and
nobody has run that.

## 9. Routes tried and closed

Kept because the enumeration is usually worth more than the answer.

1. **`Width` and `Bool` for the ten.** Closed by `k2`: three compile errors, two
   controls building. Six of the ten are signed or 64-bit.
2. **`Width` widened to mean any count.** Closed on the design's own words at
   `src/width.rs:18`, and it does not help the six signed ones anyway.
3. **A bare type alias**, `pub type PhaseNum = i64;` in the door, so an
   implementor writes `const PHASE_NUM: PhaseNum`. Passes the line scan and is
   refused by the lint's own message text, which `k1` prints: "Use an arvo type,
   or a domain alias grounded on one". An alias to `i64` is grounded on nothing,
   and if it were admissible the rule would be vacuous in one line everywhere.
4. **A `macro_rules!` that writes the impl inside the consumer.** The invocation
   line names no machine type, so the scan passes. Rejected on the same ground as
   3: the bare primitives are still at the public positions and the rule is about
   the design rather than about what the scanner sees. It also narrows the
   inventory to implementors willing to go through a macro, which is not what the
   ratified clause says.
5. **One bundle type per trait** (`QuantumLaw`, `SlotRange`) instead of one per
   coordinate. Five new types instead of nine, fewer constants for an implementor
   to write, and a bundle can validate at construction. Closed because it does
   nothing for the function signatures, which are the same class one position
   over, and because it loses the distinction between an exponent and a slot
   inside its own constructor. Worth reconsidering only if somebody decides the
   function signatures are out of scope.
6. **Sealing the traits** so no outside crate implements them and the inventory
   opens some other way. Closed by `ruling::the_format_spine_is_canon` directly,
   and by `slots.rs:52-63`, which records that this crate has already made and
   retracted the claim that its trait was closed.
7. **Widening the lint's exception to the trait-impl position.** Closed on
   locus: the pack is a line scan by design, the change is in another repository,
   and op excepted the const generic parameter and nothing else.
8. **`min_adt_const_params`.** Closed by `k6`: it refuses a newtype with a
   private field, so it costs the encapsulation as well as the gate, and it is the
   name with no row in the register.
9. **Narrowing the exemption to two types.** Closed on mechanism in section 3.
10. **Recording that the obligation does not reach associated constants.** Closed
    in section 4 as the one option that is unavailable rather than costly.

Two probes of mine were wrong before they were right, and both were caught by
their own controls rather than by re-reading them. `k4`'s first run walked one
directory too far up, so every count came off a path that does not exist; both
of its crate controls still returned zero, because a `grep` over a missing path
puts its complaint on stderr and the count of matching lines in that complaint is
zero. The third control, that the scan can match anything at all, is what caught
it. And `k5`'s harness piped its gate through `sed` and read the pipeline's exit
status, which is `sed`'s and is always zero, so it printed a pass over a gate
that had just failed with ten findings. Both are fixed in place with the reason
written into the file.

## 10. The probes

All in `238_probes/`, each with its raw output beside it.

| Probe | What it establishes |
|---|---|
| `k1_lint_over_the_impl` | The shipped impl in a checked crate is exactly ten findings; four controls at zero. Runs the pinned pack itself. |
| `k2_can_width_carry_the_ten` | `Width` carries neither a negative coordinate nor one above 2^32; `Bool` carries the one truth value. Two controls build, three arms refused. |
| `k3_the_class_the_row_names_ten_of` | The exemption carries 128 lines in `arvo-format`; both checked crates at zero. |
| `k4_the_public_positions.sh` | 59 public positions in non-test declaration syntax; the ten trait constants at their exact lines. |
| `k5_the_shape_built` | The answer built: a door of ten coordinates, a crate outside it declaring a format with no machine type on any line, the shipped answers reproduced, a mutation refused, and zero lint findings outside. |
| `k6_which_gate_the_second_option_needs` | `adt_const_params` admits a private-field newtype as a const parameter; `min_adt_const_params` does not. |
| `k7_does_the_const_param_gate_leak` | An `adt_const_params` parameter in a public signature does not reach a consumer, with the control that the consumer is genuinely ungated. |
| `k8_the_open_inventory_test_on_the_other_side` | The open-inventory test moved to a checked crate produces two findings as written and ten for a format that borrows nothing. |
