# 253. The debug row has no subject, the surface it wants is a canon call, and three arms are compiled

I was dispatched to design and implement debug output across the numeral shapes, spanning the crates
that carry them, through the mockspace ceremony in full.

**I did not run the ceremony.** The unit's subject does not exist, and the part of it that survives
that is a canon call the canon is silent on, which `mock/agent/MAIN.md.tmpl:79` reserves to two
independent agreements rather than to one expert inside a design. I am one expert. So this is an
ambiguity return, and the call goes back.

That is the short form. What follows is long because the work was: five probes, all committed and all
green, three of which exist because I refused to report a blocker without attacking it first. Two of
the three blockers I expected to be fatal turned out not to be, and saying so is most of the value
here, because the next reader should not spend their budget where I already spent mine.

## 0. The two gates

**Test gate: passed, and it is a real pass.** `cargo test --workspace --all-targets` from `mock/`:
112 in `arvo-format` (111 passing, 1 ignored), 8 `trybuild` compile-fail cases, 14 in the MATLAB
parity suite (13 passing, 1 ignored), 22 in `arvo-placement` (21 passing, 1 ignored), 10 in
`arvo-strategy`, and 9 doctests over two crates. 163 passing and 3 ignored across the five test binaries, plus 9 doctests, so 172 passing in total and nothing failing.

I read bodies rather than counting names, over the surface this touches, which is every public type
declaration in the three crates and the two `arvo-format` test files. The three ignored are
catalogue-reds whose `#[ignore]` reasons name what they wait on, and each names a real gap: a MATLAB
rounding mode with no name in the ratified six, a converse independence one packing rule cannot
exhibit, and a euclidean carry. Nothing tautological, nothing asserting a value against itself, and
the `the_control_*` arms are genuine: `arvo-placement`'s `the_control_the_ladder_rungs_are_distinct_and_ordered`
fails if the ladder stops discriminating, which is what the four arms above it rest on.

**Canon gate: the unit is misframed, and what survives is ambiguous.** Sections 1 and 2.

## 1. The brief's frame is false, and this is the load-bearing finding

The dispatch says to design debug output "across the numeral shapes, spanning the crates under
`mock/crates/` that carry them".

**No crate under `mock/crates/` carries a numeral.** Every point of the ratified parameterisation is a
zero-sized declaration. `crates/arvo-format/src/lib.rs:89` is the whole of it:

```rust
pub struct Integer<const BITS: u32>;
```

That declares which values exist. It holds none of them. `253_probes/p05_the_split_is_value_against_marker`
asserts it as const evaluation over the full population rather than a sample: 35 non-rendering public
types at zero bytes, 18 rendering ones above zero, every one listed. I mutated
`size_of::<Integer<8>>() == 0` to `== 4` and watched `E0080` fire, then restored and rebuilt, so the
instrument is known to be able to fail.

So "debug output from every numeral" ranges over the empty set in this tree, and the row asking for it,
`obligation::debug_output_from_every_numeral_shape`, cannot be discharged by anything I could write.
Supplying a numeral is `topic = the_primitive`, on which `ruling` carries zero rows. That is the
largest open question in this registry and it is not mine.

**251 reached the same place and I confirm it rather than re-deriving it.** Its `p04` re-runs green
here, four tests with two controls, and its committed `E0277` is real. Two unratified files agreeing
is shared drift, so I checked the claim at source instead: the `pub struct` declaration above, and my
own const assertions over all 53 types.

## 2. What the canon says, measured

### 2.1 Silent on debug output

Over `mock/registry/ruling.toml`, case-insensitive, with the counts:

| pattern | hits | what they are |
|---|---|---|
| `core::fmt` | 0 | |
| `\bfmt\b` | 0 | |
| `Display` | 0 | |
| `format!` | 0 | |
| `to_string` | 0 | |
| `debug` | 12 | every one the phrase "debug build", inside the two panic rulings |
| `print` | 8 | every one inside the word "footprint" |
| `render` | 1 | prose in a `note` about a work in progress |

Positive control on the same instrument over the same file: `usize` returns 1, `^id = ` returns 96. It
can produce a non-zero, so the zeros are about the corpus.

`mock/agent/MAIN.md.tmpl:80` says his silence is not permission, and line 79 gives the procedure:
derive it from the intent inside its spirit, **put it through two independent agreements**. A design
round that ships a public rendering contract on one expert's reading is not that.

### 2.2 The two reserved questions, and neither of them blocks this

Both confirmed by `cargo mock query`, `decider = panel`, `answered` empty:
`question::what_a_platform_width_type_is` and `question::the_width_surface_crossing`.

`MAIN.md.tmpl:81` is exact: a reserved call "may not be filled inside a design".

**Where they would bite is one position: a count of bytes.** A rendering that reports how much it
wrote puts a byte count at a public position, I14 forbids a bare `usize` there, and no type in the
stack means a count of bytes. `Width` counts bits, `SlotCount` counts slots, `MagnitudeCount` counts
magnitudes, `Arity` counts operands. None of them is it, and minting one is Q26.

**I attacked that and got past it twice, independently.** Section 4's arm B takes a
`W: core::fmt::Write` sink, so the caller owns the capacity and no count crosses any signature. Arm C
keeps the length private and takes the caller's size as a const generic, which is op's own excepted
position. Both compile. **So neither reserved question blocks a debug surface**, and a later reader
should not stop where I expected to.

I say that as a result rather than a relief, because it is the reverse of what I went looking for.

### 2.3 What is ratified and does constrain the content

`ruling::the_format_spine_is_canon`, `rung = ratified`, `ratified_by = both`:

> A format is identified by its ambient domain and its representable set, and that set is a constant
> of the type. ... The concept is closed and the inventory of admitted instances is open.

Two clauses, both load-bearing below. The identity fixes *what* a rendering of a declaration would
have to say, so that part is not a free choice. The open inventory fixes *who* it has to reach, and
that is where section 3 goes.

## 3. Three things I expected to block, attacked one at a time

### 3.1 "Every numeral" cannot be supplied by arvo, and this one is fatal

Under the open-inventory clause, "every" ranges over implementors of `Format` that this crate will
never see. The obvious supply is a blanket impl. It is refused.

`253_probes/p01_every_format_renders`, arm and control on the same rustc invocation, both committed
with their stderr:

```
error[E0210]: type parameter `F` must be used as the type parameter for some local type
12 | impl<F: Format> fmt::Debug for F {
   = note: implementing a foreign trait is only possible if at least one of the
           types for which it is implemented is local
```

The control is the identical shape with a local trait in `core::fmt::Debug`'s place: exit 0, zero
bytes of stderr, on the same command. So `E0210` is about foreignness, not about how I called rustc.

**Consequence, and it is the one that decides the row.** arvo cannot ship `Debug` for every format.
The only thing it can blanket is a trait of its own, and a trait of its own is a new contract rather
than a `Debug` impl. So the row's "every numeral" is not a gap in arvo's implementation; it is
unreachable from arvo's side by construction.

**And the control proves the capability is already available to consumers today.** That control crate
is a downstream crate: it depends on `arvo-format` by path, declares its own trait, blankets it over
`F: Format`, and reaches `Integer<32>`. Any consumer can do exactly that, in its own crate, with no
change to arvo. The need the row states is met without an arvo surface existing.

### 3.2 "At every width" does not block, and I expected it to

I went in assuming this was `question::the_width_surface_crossing` and therefore reserved. It is not.
A declaration's width arrives as its own const parameter and never crosses back through a written
literal, and `arvo-format` already makes the whole coordinate set const-reachable:
`radix::<F>()`, `slot_count::<F::Slots>()`, `declared_slot_width::<F::Slots>()`,
`smallest_step_exponent::<F>()`, `F::PHASE`. Arm B and arm C both read all five in const context.

Q9 is about what a consumer *writes* at a call site. Nothing here is at a call site.

### 3.3 "Under every strategy" is either vacuous or names something that does not exist

`crates/arvo-strategy/src/lib.rs:41` makes `Strategy` a trait with an open inventory, so quantifying
over it generically is expressible and `ruling::the_strategy_set_is_not_closed_at_four` is not
offended. That much is fine.

What is not fine is that **nothing in `Format` is keyed on a strategy.** A strategy binds an objective
and an adaptation; a declaration's identity is its ambient domain and its representable set, and no
strategy moves either. So a rendering of a declaration cannot vary with a strategy, and the row's
third quantifier asks for a dependence the ratified design does not have. It is not a hard blocker,
it is a clause that will never mean anything.

## 4. The composition, all three arms compiled

Handed forward rather than chosen, because choosing is the call I am returning.

### Arm A. Nothing in arvo

Consumers blanket their own trait over `F: Format`. Evidence: `p01`'s control, which is a downstream
crate doing exactly this.

Costs arvo nothing and ships nothing. **Its price is that every consumer's rendering differs**, so
there is no canonical statement of what identifies a format, and two consumers debugging the same
declaration see two different strings. That is a real argument for arvo shipping one, and it is
precisely why the question needs a second reader rather than my answer.

`holds for: F any Format, consumers any, threads any, target features any, no buffer position`

### Arm B. A local trait, blanket-implemented, into a `core::fmt::Write` sink

`253_probes/p02_the_identity_renders_over_the_open_inventory`, 6 tests, 2 controls, green.

Reaches the open inventory: the probe declares a format `arvo-format` does not know about and it
renders through the same blanket with no edit anywhere. Renders the ratified identity. Discriminates
on both the const parameter and the family, which is the thing section 5 shows `derive` cannot do.
No bare primitive at any position: the sink is generic, so no count, length or capacity appears in
any signature.

Its price is a `core::fmt::Result` on the sink, which is a runtime path. That is core's own signature
and `mock/PRINCIPLES.md.tmpl:226` names `fn fmt() -> fmt::Result` as a licensed escape hatch, so it is
not arvo validating anything. It is still not nothing.

`holds for: F any Format, sink any core::fmt::Write, threads = 1, target features any, no_std, no alloc`

### Arm C. A const rendering into a caller-sized buffer, refused at build time

`253_probes/p03_the_fit_is_refused_at_compile_time`, 6 tests including a `trybuild` case, green.

Everything a declaration renders is a constant of the type, per the ratified identity clause, so the
rendered length is a constant too and the fit is decidable before anything runs:

```
error[E0080]: evaluation panicked: the buffer is too small for this format's identity
```

**No runtime error path at all**, which is the only arm that fully honours "no runtime checks, ever".
The whole rendering is available in a `const` item, measured by `the_rendering_is_available_at_const_time`.

Its price is the byte count of section 2.2: the length has to stay private, because there is no stack
type for it. The reserved question is dodged rather than answered, and that is worth saying plainly.

`holds for: F any Format, N any usize const parameter, threads any, target features any, no_std, no alloc, no unstable features`

### Where the fork actually falls

Not "format concept against placement-side", which is how `252` section 12's O5 poses it. Neither.
**The line is whether the thing rendered has a value.** C works where everything is const, which is
every declaration; B works where a value arrives at runtime, which is every coordinate and every
numeral once one exists. That is the same line `p05` measured independently in section 5, arrived at
from the other direction, and I take the agreement seriously for that reason and not because it is
tidy.

### One measurement worth keeping on its own

Where a const gate becomes reachable, three arms, stderr captured and committed:

| arm | command | exit | stderr |
|---|---|---|---|
| const-bound | `--emit=metadata` | 1 | 1129 bytes, `E0080` |
| runtime call | `--emit=metadata` | 0 | 0 bytes |
| runtime call | full codegen | 1 | 1379 bytes, `E0080` |

**So a compile-fail case written as a runtime call is green under `cargo check` and red only under
`cargo build`.** Binding it in a `const` item is what makes the refusal reachable at check time.
`crates/arvo-format/src/format.rs:179` records the same distinction from the other side, in the
`Format::PHASE` doctest, which is how I knew to look.

**One reading of that disagreed and I chased it rather than picking.** Batched on one command line
with stderr discarded, arm two reported exit 1, which would have made the measurement say the binding
changes nothing. One arm per invocation with stderr captured, it reports exit 0 and zero bytes, three
times. The byte count is what settles it, and the empty `arm2_runtime_call_check.stderr` is committed
empty on purpose.

The commit hook then reformatted the probe sources, which moved the assert onto four lines and changed
what the diagnostics quote. Every stderr here was retaken against the reformatted source and the byte
counts in the table are the retaken ones. The refusal, the code and the monomorphisation are unchanged;
what moved was the rendering, which is why the artifacts were retaken rather than the test loosened.

## 5. What I refuse to ship, and it is the tempting one

The obvious act on 35 public types with no `Debug` is to add `#[derive(Debug)]`. I refuse it, on two
measurements.

**The split is already principled.** `p05` asserts, over the whole population and not a sample, that
every one of the 18 types that renders holds a value and every one of the 35 that does not is zero
sized. That is a rule. Adding derives to some of the 35 replaces it with a list, which is the pile of
heuristics standing where one rule should.

**And on 13 of the 35 the derive cannot discriminate.**
`253_probes/p04_derive_cannot_discriminate`, with its control:

```
AUnitStructWithAConstParameter<8>   ->  "AUnitStructWithAConstParameter"
AUnitStructWithAConstParameter<32>  ->  "AUnitStructWithAConstParameter"
```

`derive` renders the name and drops the const, so `Integer<8>` and `Integer<32>` render identically
while being different formats under the ratified identity clause. A rendering that cannot tell apart
the thing it renders carries no information about it, which is the tautology shape one tier over. The
control is the same derive on a plain marker, where the name is the whole identity and it does
discriminate.

So the 35 split 13 and 22, and shipping the 22 that would work while the 13 cannot is exactly the
arbitrary line the first measurement says not to draw.

**Keeping the current state is the result.** It is coherent, and I am not making it incoherent to
have something to put in the answer slot.

## 6. What is unsettled, stated as precisely as I can

**The call I am returning: does arvo owe a rendering surface at all, and if so which of A, B or C.**

Canon bearing on it, and how each fails to decide it:

- `ruling::the_operating_constraints_are_intents_and_rules`, `in_force`, I14. Bounds what a surface
  may look like. Says nothing about whether one exists.
- `ruling::the_format_spine_is_canon`, ratified. Fixes what a rendering must say and who it must
  reach. Says nothing about whether it must exist.
- `mock/PRINCIPLES.md.tmpl:226` licenses `fn fmt() -> fmt::Result` as an escape hatch, which is the
  nearest the design tier comes to the subject, and it is permission for an impl rather than a demand
  for a surface.
- `obligation::debug_output_from_every_numeral_shape` is the only statement of need. It is demand-side,
  its `consumer` is `any` rather than a named one, its `why` is refuted, and its provenance is a
  dispatcher note about a check written against a tree deleted on 2026-08-08. It cannot carry this.

**Both directions are live and I hold neither.** Arm A says the capability is already the consumer's,
proved by `p01`'s control, and `arvo-toolbox-not-policer` pushes that way. Against it: a foundation
where every consumer invents its own rendering of a format's identity has no canonical one, and the
identity is ratified, so there is an argument that its rendering should be too.

**I am the first read on this and a second is owed.** Whoever takes it should form their own reading
from the canon before opening this file, then reconcile; agreement reached from my conclusion is
confirmation, not corroboration.

## 7. Options, each with what closes it

**O1. Whether arvo ships a rendering surface at all.** *Closed by* a second independent derivation
from `ruling::the_format_spine_is_canon` and I14, reconciled with this one. If it agrees a surface is
owed, arms B and C are compiled and waiting; if it agrees none is, the row should be retired with a
statement that the capability is the consumer's and `p01`'s control as the evidence.

**O2. `obligation::debug_output_from_every_numeral_shape` should be retired rather than reworded.**
`251`'s O2 offered retirement or an edit. I go further than it did and say retirement, because all
three of the row's quantifiers fail separately: "every numeral" is unreachable from arvo (`E0210`),
"every strategy" names a dependence the design does not have, and the subject is empty. A row wrong in
its reason, its scope and its subject is not a wording problem. *Closed by* the retirement, with a
replacement stating the need over the value layer if one is still wanted once that layer exists.

**O3. The census in section 5 wants to be a lint and is currently nothing.** Nothing in the suite
asserts the value-against-marker split, so the rule holds by accident and the next person adding a
public type will not be told. `mock/lints/` is where it belongs, per this repo's own
`mock/Cargo.toml:14`. *I did not build it*, because a lint enforcing "every value-carrying type
renders" enforces a rule nobody ratified, and that is O1. *Closed by* O1, after which the lint is
mechanical.

**O4. `declared_slot_width` is public and unreachable from the root.**
`crates/arvo-format/src/slots.rs:323` declares it `pub const fn`, and
`crates/arvo-format/src/lib.rs:74` re-exports `slot_count` and `slot_in_range` from that module and
not it. It is reachable only as `arvo_format::slots::declared_slot_width`, which cost me a compile
error building arm B. A fix, not a design question, and I did not make it: the gate returned early and
an unrelated source change on the way out is scope. *Closed by* one line.

**O5. The published design tells readers to use a crate that does not exist.**
`docs/PRINCIPLES.md:228,230,231`, generated from `mock/PRINCIPLES.md.tmpl:219,221,222`, maps `usize`
to `USize` / `ISize` / `Cap` "in `arvo::newtype`", `f32` and `f64` to `FastFloat` / `StrictFloat`
there, and `bool` to `Bool` there. There is no `arvo` crate: `mock/Cargo.toml` has three members and
none is a facade. There is no `newtype` module. `Bool` is in `arvo_format::width`.

This is outside my unit and I am reporting it under the standing instruction. It is not bookkeeping:
`251`'s `p03` counted 1488 occurrences of `USize` across three consumer trees, and
`obligation::a_platform_sized_unsigned_integer_at_an_api_position` quotes a consumer using it. **The
published design is the document telling them to.** *Closed by* a doc round on the table, which is not
mine to open from a gate return.

## 8. Carried forward unchanged, and from whom

| # | Carried | From | Re-checked |
|---|---|---|---|
| 1 | `ruling::the_format_spine_is_canon` and its four | op and the experts, `213` | read whole from the registry |
| 2 | `ruling::the_operating_constraints_are_intents_and_rules`, I14 | op, `85` | read whole |
| 3 | `ruling::the_strategy_set_is_not_closed_at_four` | op, `39` | via `MAIN.md.tmpl` and the crate |
| 4 | `question::what_a_platform_width_type_is` and `the_width_surface_crossing` reserve two positions | `252` §3.5 | re-queried both, `decider`/`answered` |
| 5 | The canon is silent on debug output | `251` §2.5 and `252` §3.4 | re-measured with my own patterns and control |
| 6 | The row's `why` is false | `251` §4.4 | re-ran its `p04`, 4 green |
| 7 | The value layer is absent | `251` §3.1 | re-asserted at 53 types with a mutation check |

Five and seven I treated as claims rather than facts, because two unratified files agreeing is shared
drift. Both survived. Four I re-queried because it decides whether I may proceed at all.

## 9. Coverage

**Read in full.** `mock/agent/MAIN.md.tmpl`; `mockspace.toml`; `mock/Cargo.toml`;
`crates/arvo-format/src/lib.rs`, `width.rs`, `format.rs` and the type declarations of every module in
the three crates; `crates/arvo-strategy/src/lib.rs`; the `ruling` rows for the format spine;
`obligation::debug_output_from_every_numeral_shape` and the four beside it;
`question::what_a_platform_width_type_is` and `the_width_surface_crossing` whole; `251` whole; `252`
sections 0 to 6 and 12 and its appendix.

**Read in part.** `ruling.toml` by pattern with controls, as tabled in 2.1; `mock/PRINCIPLES.md.tmpl`
and `docs/PRINCIPLES.md` at the primitives table; the three crates' `DESIGN.md.tmpl` by pattern for
the same list.

**Ran.** The whole suite, once, unfiltered. Five probes of my own, all committed and all green.
`251`'s `p04`, re-run. Ten `cargo mock query` calls. The three-arm reachability measurement, twice,
which is why section 4 has an anomaly in it.

**Not read, and each could move something above.** `law.toml`, `law-the-later-topics.toml`,
`dimension.toml`, `probe.toml`, `strategy.toml`, `proposal.toml` other than the identity row, other
than by pattern. A law about a rendering would bear on section 6 directly. And no panel member file
before `250` except by reference; `161_leroy_the_canon_candidate_for_the_primitive` is named in `251`
as the candidate for the topic where the value layer belongs and I did not open it, which bounds
section 1's claim about the locus to what `251` and the registry say rather than to my own reading.

**The suite has no test over any of this**, before or after me, and I did not add one, because what a
test would assert is O1.

## 10. The one sentence

The row asks arvo for debug output from numerals it does not have, in a quantifier arvo provably
cannot satisfy, for a reason that is false, and the capability it describes is one any consumer can
already build in its own crate; what is genuinely open is whether arvo should ship a canonical one
anyway, that is a canon call the canon is silent on, and three arms are compiled and waiting for the
second reader who is owed it.
