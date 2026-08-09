# Panel 06: the consumer surface, and whether the architecture fights the person using it

**Persona:** Casey Muratori, tools and API usability lens. Sixth member; read
`01_knuth_mathematical_rigour.md`, `02_kiselyov_type_level_encoding.md`,
`03_jhala_what_is_provable.md`, `04_torvalds_does_it_earn_its_keep.md`,
`04b_op_checkpoint_and_directions.md` and `05_leijen_fallibility_without_poisoning.md` in full,
plus every probe under `02_probes/` and `05_probes/`, before starting.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), all six prior
panel files, the twenty-one probes under `02_probes/` and `05_probes/`, the panel brief, the
governing panel rule, every `.stderr` fixture and its `.rs` under `arvo/tests/ui/`,
`arvo/src/{lib,aliases}.rs`, `arvo/tests/strategy_semantics.rs`, and
`hilavitkutin-api/src/dispatch_codegen.rs`. **What I read in part:** the talk and the
inherited-state file at the passages the spec and prior members cite; `arvo/src/ufixed.rs`,
`arvo-strategy/src/{identity,container,axes}.rs`, `02_probes/{c4_diag,h_widthtype}.rs` line by line
because I contest a conclusion from each, and `hilavitkutin-providers/src/adapt_ema.rs`.

**Directory listing done** across `mock/design_rounds/` (94 entries, the three flat files at root
are this round), `mock/research/` (nothing postdates the panel directory), `mock/research/sketches/`
(nineteen entries) and the panel directory itself. Nothing supersedes the spec. I also listed
`arvo/tests/` and `arvo/tests/ui/`, which is where most of my evidence came from and which the
brief pointed me at.

**Gates.** I re-ran the whole suite rather than inheriting a count: 654 passed, 0 failed, 9 ignored,
122 binaries, matching 01, 02, 04 and 05. The one ignore inside `arvo` is a properly labelled
catalogue red (`arvo/tests/fixed_point_div.rs:111`, "catalogue: >64-bit-logical fixed-point divide
needs 256/128 long division; tracked #5"), which is the shape
`catalogue-edge-cases-as-tests.md` asks for. I read the nine `tests/ui/` fixtures and their
`.stderr` snapshots in my own hand rather than through 02's report, and I found something 02 did not:
they are real, and they are calibrated against an environment arvo's actual consumer is not in.
That is section 1.

**Separation of evidence.** Sections marked *verified* were compiled under the pinned
`nightly-2026-05-28` from six probes committed alongside this file at `06_probes/`, or read at a
`file:line`. Sections marked *reasoned* are argument. Impressions are labelled. I carry more than
one reading wherever the evidence does not force one, and I rule on nothing.

---

## 0. My lens, and two premises the panel is carrying that came back different

The other five members asked whether the design is true, whether it is honestly encoded, what it can
prove, what it costs to build, and what a refusal costs the code around it. I am asking one narrower
thing: what a person types, what the machine says back, and how many things they have to know that
they should not have to. A numeric foundation is a tool, and a tool that fights its user loses to a
worse tool that does not, regardless of how much mathematics is underneath.

Op's checkpoint at `04b` opens my thread explicitly, and I want to quote the whole instruction because
it decides the shape of this file: "not just price, iterate on; there might be ergonomics to be won
when taking further and specializing, instead of stopping at this solution". So the question is not
whether faces beat aliases. It is what the best available consumer surface for this design is, and
faces are one candidate among several, most of which nobody has written down.

Before that, the brief's standing first task. I checked two premises the panel is carrying and both
came back different from how they are stated.

**Premise one, from 04 section 0: "hilavitkutin has 20 `UFixed`/`IFixed` mentions across seven
files".** That count is right and the conclusion drawn from it is not, because it counts the wrong
spelling. Excluding comment lines, hilavitkutin writes `Uint<N, S>` at 22 sites and
`UFixed<..>`/`IFixed<..>` at 9, across ten files. The dominant consumer spelling is already an alias,
by better than two to one. `arvo::Fixed<I, F, S>` and `arvo::Signed<I, F, S>`, the two aliases whose
own doc comments say "use this at consumer call sites" (`arvo/src/aliases.rs:35-39`), are used zero
times. That reframes 04's section 2 from a forecast into a measurement: the alias story is not
something the spec would introduce, it is what ships, and its diagnostic consequences are observable
today rather than predictable.

**Premise two, from 04 section 2: the shipped error surface "is genuinely good" and the spec
"throws it away".** Half right. It is good, and it is conditional in a way nobody has stated: the
quality survives alias expansion at the caret and dies in the trailing note, and which of the two a
consumer sees is a property of whether the failing obligation's trait carries
`#[diagnostic::on_unimplemented]`, not a property of the alias. 04's probe used a bare trait and
therefore measured only the bad half. Section 2.

Neither of these breaks the brief, so I proceed. But both are the shape of thing the panel rule says
to check, and the second one changes what the repair should be.

## 1. The alias damage is not a forecast. It shipped, and the fixtures do not cover it. Verified.

`06_probes/a_alias_render.rs`. A downstream crate with a path dependency on arvo, three cases,
compiled on the pinned nightly.

Case one, the dominant consumer spelling, in a crate with no extra feature gates:

```
error[E0277]: the trait bound `UFixed<IBits(MetaCarrier(12)), FBits(MetaCarrier(0)), ...>:
              SignedIdentity` is not satisfied
  |
4 |     let _ = <arvo::Uint<12, Hot> as SignedIdentity>::NEG_ONE;
  |              ^^^^^^^^^^^^^^^^^^^ the trait `SignedIdentity` is not implemented for
  |              `UFixed<IBits(MetaCarrier(12)), FBits(MetaCarrier(0)), Hot>`
  = note: the full name for the type has been written to '...long-type-....txt'
```

Read the last line. Three type parameters, one of them a ZST marker, and rustc has already decided
the rendered type is too long to print and written it to a file. That is today, on the shipped
design, before a single one of the ten axes exists.

Case two is the one that matters, because it is the environment arvo's real consumer is actually in.
`hilavitkutin/src/lib.rs:24` carries `#![feature(generic_const_exprs)]`, and so do six of its test
files. With that gate on downstream, the same line renders as:

```
error[E0277]: the trait bound `UFixed<arvo::::aliases::Uint::{constant#0},
              arvo::::aliases::Uint::{constant#1}, ...>: SignedIdentity` is not satisfied
```

The consumer wrote `12`. The compiler cannot tell them the width. It names two anonymous const items
inside a private module, with a doubled `::`, and truncates the third parameter.

And now the fixture question. `arvo/tests/ui/` has nine `.stderr` snapshots, they are honest, and
02 read them and reported them honest. But none of them enables `generic_const_exprs`, and none of
them uses `Uint` or `Fixed`. So **the diagnostic suite pins the error text for a spelling the
consumer does not write, in an environment the consumer is not in.** That is not a fabricated test in
the sense the test gate is about, and I am not calling it one. It is a coverage gap of exactly the
kind `catalogue-edge-cases-as-tests.md` exists to close, and it is the reason a real, present,
shipped defect has sat there without anyone in this round noticing it: the fixture that would have
caught it was never written, because the surface it covers was never the one under test.

Two readings on what that means for the round.

The first, which I lean to: this is a second, independent reason to do the
`generic_const_exprs` remediation that 02, 03 and 04 all flag as forbidden-feature drift. That
removal has been argued purely on rule compliance so far. It is also a diagnostic repair, because the
`{constant#0}` render is a consequence of the gate being on in the consumer's crate. Nobody has
connected those and they are the same edit.

The second reading, weaker but worth stating: the render may improve on a later nightly, since this
is rustc's rendering of an unnormalised const rather than an arvo defect as such, and the workspace's
toolchain pin is a policy artifact that will move. I do not find this persuasive as a reason to wait,
because the pin is deliberate (`workspace.md`, "bumping the pin is a deliberate workspace-policy
act") and because a library at this layer cannot ship a surface whose legibility depends on a compiler version.

## 2. The alias survives at the caret and dies in the note, and the attribute decides which. Verified.

04's probe reported that rustc expands aliases in diagnostics, which is true and which I reproduce.
What it did not distinguish is where.

`06_probes/a_alias_render.rs`, case three. Same failure, through the ergonomic alias, on a trait that
carries the attribute:

```
error[E0277]: this type has no multiplicative identity
  |
4 |     let _ = <arvo::Fixed<0, 8, Hot> as Identity<Multiplicative>>::IDENTITY;
  |              ^^^^^^^^^^^^^^^^^^^^^^ the trait `OneRepresentable<1>` is not implemented for `Picker`
  = note: A purely fractional fixed-point type has zero integer bits, so it spans [0, 1) unsigned
          or [-1, 1) signed and one is not a value of it. ...
  = note: required for `UFixed<IBits(MetaCarrier(0)), FBits(MetaCarrier(8)), Hot>` to implement
          `Identity<Multiplicative>`
```

The message line is the authored text. The caret sits under `arvo::Fixed<0, 8, Hot>`, which is what
the consumer typed. The expansion is demoted to the last note, where it is noise rather than the
headline. That is a good error, through an alias, today.

The mechanism is simple and worth stating plainly because it is the lever for everything that
follows. When `#[diagnostic::on_unimplemented]` supplies a message, the message *is* the authored
text and the normalised type moves to a note. When it does not, the message line is the normalised
bound, which is what 04 measured. `06_probes/b_surface_rendering.rs` isolates this: the same
composition, two traits, one with the attribute and one without, and the difference is exactly which
line the expansion lands on.

So the repair 04 reaches for, concrete faces, is one way to shorten the note. There is a cheaper one
that is orthogonal and that arvo has already half-built: **every trait a consumer can name in a bound
carries the attribute.** `arvo-strategy/src/identity.rs:81-84` is the model and it works. Nine
fixtures in `tests/ui/` prove it works. The thing that is missing is not the mechanism, it is the
coverage: the attribute is on `OneRepresentable` and `SignedIdentity` and nowhere near the surface
the spec is about to add.

I want to be careful not to oversell this. The attribute does not fix the note, and section 3 shows
faces do not fix it either. What it fixes is the headline, which is the line a person reads first and
often the only line they read.

## 3. Newtype faces fix the numeral half and leave the other half intact. Verified, correcting 04 section 7.

04's section 7 proposes the four families as `repr(transparent)` newtypes over their compositions,
"impls forwarded by macro from the inner type. Diagnostics name the face." I built it and measured
it, alongside the alias and one other shape, against a composition carrying the spec's ten axes.
`06_probes/b_surface_rendering.rs`, message lines verbatim:

```
alias, with attribute:
  `Number<Bin<Implicit<0>, Unit, Zero, Unsigned, 13>, Pol<..., ...>, ...>`
  has no associative addition

face, with attribute:
  `UFixedFace<13, 3, Pol<Quant<ToEven, ToEven, ..., ..., ...>, ...>, ...>`
  has no associative addition
```

The face recovers `13, 3` and loses everything else, because the face's own generic parameters carry
the policy and lowering compositions. Both spill to a long-type file. The face is better and it is
not the fix.

This is not a small correction to 04, because 04's section 9 lists the alias-only story under "what I
would cut" and offers faces as the replacement, and 05's section 10 endorses faces on a separate
argument ("diagnostics are a cost that scales with the design's generality, which is an argument for
04's newtype faces that does not depend on taste"). Both are reasoning about the numeral. Under the
spec's shape the numeral is five axes and the strategy is five, and the strategy half is the half a
consumer varies. A repair that fixes the half they do not vary is aimed at the wrong side.

There is a reading in which faces still earn their place, and I hold it: a face is a
grep target, a rustdoc page, and a name in a crate index, and none of those is a diagnostic concern.
`UFixed` appearing in `docs/DESIGN.md` and on docs.rs as a struct with its own page is worth
something that a type alias expanding to a composition is not. That is a documentation argument
rather than an error-message argument, it is real, and it is not what faces were proposed for.

## 4. The category the thread was asking for: the numeral is a name, not a record. Verified.

Here is the direction I think op's "iterate on it, specialise" is pointing at, and it is a different
category from faces rather than a variant of them.

The diagnostic cost is not caused by having ten axes. It is caused by **spelling the axes
structurally at the type-parameter positions**. rustc prints the type arguments that were applied. It
does not print the associated types those arguments project to. So a composition can carry an
arbitrary number of axes behind projections and still render in five tokens, provided every value the
consumer selects arrives through a *named* type.

`06_probes/b_surface_rendering.rs`, third surface. The numeral is `Fix<const I, const F, Sign>`, a
nominal struct whose `Numeral` impl projects the five identity axes as associated types rather than
carrying them as parameters:

```
nominal, no attribute:
  the trait bound `Number<Fix<13, 3, Unsigned>, Pol<..., ...>, ...>: AddAssocPlain`
  is not satisfied
```

The whole numeral collapsed to `Fix<13, 3, Unsigned>` and every token in it is a token the consumer
wrote. The policy side still truncates, because in that probe it is still structural.

`06_probes/c_nominal_and_modifier.rs` makes both sides nominal, with `Warm` as a named ZST
implementing `Policy` and `Lowering` exactly as the spec's crate table has it (`spec:298`,
"`arvo-strategy` holds `Hot`, `Cold`, `Warm`, `Precise`, and nothing else"). Result, with the law
trait carrying the attribute:

```
error[E0277]: `Number<Fix<13, 3, Signed>, Warm>` has no associative addition
  |
  | pub fn case_a() { fold::<IFixed<13, 3, Warm>>() }
  |                          ^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
  = note: Its out-of-range rule is not translation-stable on a signed domain.
```

Complete. No truncation. No long-type spill. The rendered type is shorter than what ships today,
because `Fix<13, 3, Signed>` is shorter than `UFixed<IBits(MetaCarrier(13)), FBits(MetaCarrier(0)),
Hot>`.

The claim I would put to op, stated as a claim so it can be attacked: **ten axes are free in the error
surface, and the spec's diagnostic problem is entirely an artifact of one implementation choice,
which is whether `Numeral` is inhabited by structural records or by named constructors.** The spec
does not state which, and both of its worked examples read structural (`Implicit<const EXPONENT>`,
`Stored<const BITS, U>` at `spec:90-95` are the exponent forms, and 04's probe assembled
`Bin<Implicit<0>, Unit, Zero, Unsigned, 13>` from them, which is the natural reading). Making the
numeral nominal costs one struct and one `Numeral` impl per family, which is four, and it is
strictly less machinery than the forwarding-macro layer faces need.

Two readings, and the second is a real cost.

The first is that this subsumes the faces proposal for diagnostic purposes and leaves faces as a
documentation choice, which section 3 says is where their remaining value is.

The second, against my own claim: a nominal numeral is a closed constructor set. `Fix`, `Flt`,
`Unorm` and whatever else, fixed at four or five, and a convention alias set cannot introduce a
fifth. 02's section 10 argues that closing the constructor set is not a cost here, because "the
conventions ship as alias sets over the abstraction and nothing else (spec:276-278), so a `conv-*`
feature never wants to add a constructor". I think that is right and I think it is the load-bearing
assumption, so it should be written down as one rather than assumed. If a convention ever needs a
numeral shape none of the named constructors expresses, the structural form is the one that can
express it and the nominal one is not.

## 5. Changing exactly one axis of ten, without spelling the other nine. Verified.

The brief asks what the ten axes cost someone who wants to change exactly one. Under the spec as
written the answer is: they cost the other nine, because a `Policy` and a `Lowering` are supplied
whole and there is no way to say "Warm, except refusing above the range" other than writing out a
new marker with all five members.

That is the ergonomics question hiding behind the diagnostics question, and it has a clean answer
that nobody in the panel has proposed. `06_probes/c_nominal_and_modifier.rs`:

```rust
/// `OverRangeOf<S, R>` is `S` in every respect except its out-of-range
/// resolution, which is `R`.
pub struct OverRangeOf<S, R>(PhantomData<(S, R)>);

impl<S: Policy, R> Policy for OverRangeOf<S, R> {
    type Quantisation = OverRangeQ<S::Quantisation, R>;
    type Growth = S::Growth;
}
impl<S: Lowering, R> Lowering for OverRangeOf<S, R> {
    type StoredWidth = S::StoredWidth; type Widening = S::Widening; type Layout = S::Layout;
}
```

One delegating impl pair per axis, ten of them, each about eight lines. The consumer writes
`IFixed<13, 3, OverRangeOf<Warm, Refuse>>`, which names exactly the one thing they changed, and the
error names it back:

```
error[E0277]: `Number<Fix<13, 3, Signed>, OverRangeOf<Warm, Refuse>>` has no associative addition
```

They compose, and the composition still renders whole:

```
error[E0277]: `Number<Fix<13, 3, Signed>, LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>>`
              has no associative addition
```

That third one is the result I would most like op to look at, because it reads as the sentence the
consumer meant: a signed 13.3, Warm but refusing out of range, laid out bitpacked. Nine axes they did
not touch are not in it and are not in the error.

I verified the derivation still resolves through a modifier rather than merely rendering nicely: the
same probe implements the law by projecting `S::Quantisation::OverRange` through a truth-value fold,
and `fold::<IFixed<13, 3, OverRangeOf<Warm, ReduceModulo>>>()` compiles while
`fold::<IFixed<13, 3, Warm>>()` does not. The modifier is transparent to the derivation because it is
just another `Policy` impl.

Three things this buys that are worth naming separately from the ergonomics.

It is the toolbox posture rather than the policer posture, in the exact terms of
`arvo-toolbox-not-policer.md`: the four presets stop being the only reachable points and become
named starting points in a space the consumer can move around in by one step. The rule's own words
are "when an axis already exists in the design and a consumer would credibly want to adjust it,
expose it", and today the axes exist and are reachable only by supplying a whole new marker.

It makes the preset table honest about what a preset is. `spec:250-257` presents four rows of six
columns and the three consequences under it. With modifiers, a preset is a named point and the
consequences are properties of the point, which is exactly what `Deterministic` and `ConstantTime`
being "derived per composition" (`spec:234-241`) already assumes.

And it gives 05's section 8 its type-level counterpart. 05 proposes a locally installed handler,
`a.under::<ReduceModulo>() + b`, scoped to an expression. A modifier is the same thing scoped to a
type. They are not competitors: one is for a call site, one is for a declaration, and both are free
under monomorphisation. 05's own stated cost, that "two places a policy can come from is two places a
reader has to look", applies to the pair and is worth weighing once rather than twice.

**Two costs I will state against my own proposal.** Ten delegating structs is ten more names in a
crate whose naming this round is already churning, and the delegation is mechanical enough that a
missed member is a silent wrong answer rather than a compile error, so the impls want to be
macro-generated from one list. And modifiers commute semantically but not syntactically, so
`LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>` and `OverRangeOf<LayoutOf<Warm, Bitpacked>, Refuse>`
are the same composition with different spellings and different rendered types, which is a
canonicalisation problem for anything that compares types textually, including the `.stderr`
fixtures. I do not think either is disqualifying and I do think both should be answered before this
is written down.

## 6. Making the law's refusal name the type the consumer wrote. Verified, and it corrects a claim of 02's.

Section 4 and 5's results were measured with the law trait having no impl at all, so the attribute on
the law trait is what fires. That is not the encoding the panel is converging on, and when I wired up
the encoding the panel *is* converging on, the good error went away.

`06_probes/c_nominal_and_modifier.rs`, with the derivation present. 02's section 4 encoding, which 03
endorses and 04 calls "the most important thing in the panel so far" in its adjacent form, computes
the law's truth value through projections and bounds one impl on `<...>::Out: IsTrue`. The consumer
writes `IFixed<13, 3, Warm>` and reads:

```
error[E0277]: the trait bound `False: IsTrue` is not satisfied
  |
  | pub fn case_a() { fold::<IFixed<13, 3, Warm>>() }
  |                          ^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
help: the trait `IsTrue` is not implemented for `False`
note: required for `Number<Fix<13, 3, Signed>, Warm>` to implement `AddAssoc`
```

`#[diagnostic::on_unimplemented]` on `AddAssoc` **never fires**, because `AddAssoc` is not the failing
obligation. The failing obligation is a where-clause of `AddAssoc`'s impl, and the attribute has no
way to reach across that.

02's section 4 table has a row reading "the diagnostic is recoverable | `c4_diag.rs` routes through
an `IsTrue` marker | the `on_unimplemented` message fires", and 02 is right that it fires. I read
`02_probes/c4_diag.rs:17-20` to see what it fires *with*, and it is:

```rust
#[diagnostic::on_unimplemented(
    message = "this composition's addition is not associative; the derivation evaluated to `{Self}`"
)]
pub trait IsTrue {}
```

`{Self}` there is `False`. So the rendered message is "this composition's addition is not associative;
the derivation evaluated to `False`", which says what went wrong and cannot say what it went wrong
about. That is a materially weaker result than the row records, and it is the second time in this
panel that a probe's stated consequence has outrun what the probe shows. 05 caught the first one and
said the sequential panel should recompile prior conclusions rather than read them; this is that
practice paying again.

The repair is small and I compiled it. `06_probes/d_verdict_names_the_composition.rs`: parameterise
the verdict marker by the composition it is a verdict about, purely so the attribute has the
consumer's type in scope.

```rust
#[diagnostic::on_unimplemented(
    message = "`{C}` has no associative addition",
    label = "this composition cannot be folded",
    note = "Its out-of-range rule is not translation-stable on a signed domain: clamping the \
            intermediate and clamping the result can disagree. Fold under a wrapping out-of-range \
            rule, or accumulate in a wider numeral and quantise once at the end."
)]
pub trait Proves<C> {}
impl<C> Proves<C> for True {}
```

and the impl's last where-clause becomes `...::Out: Proves<Number<N, S>>`. Result:

```
error[E0277]: `Number<Fix<13, 3, Signed>, Warm>` has no associative addition
  |
  | pub fn bad() { fold::<IFixed<13, 3, Warm>>() }
  |                       ^^^^^^^^^^^^^^^^^^^ this composition cannot be folded
  = note: Its out-of-range rule is not translation-stable on a signed domain: clamping the
          intermediate and clamping the result can disagree. Fold under a wrapping out-of-range
          rule, or accumulate in a wider numeral and quantise once at the end.
note: required for `Number<Fix<13, 3, Signed>, Warm>` to implement `AddAssoc`
```

Message names the composition, label sits on the consumer's own span, note carries the remediation,
and the positive case (`Hot`) still compiles. The residual leak is one `help` line naming
`Proves<Number<...>>` and `False`, below the fold and not truncated.

The same probe answers 04's section 6 impression that "when a trait-solver failure lands in the
middle of that under three more axes and a GAT, the person debugging it is op, alone, months from
now". I put the failure four generic frames deep (`app` calls `schedule` calls `plan` calls
`upward_rank`, each carrying the bound) and the error is reported at the outermost concrete
instantiation and reads identically. Projected bounds do not stack frames on the diagnostic the way
they stack them in the impl header. That is one worry priced and discharged, and I say so because 04
labelled it an impression and asked for it to be priced.

I hold one counter-reading. `Proves<C>` is a mechanism whose only purpose is to make a message
readable, and a design that needs a workaround to produce a good error is, as 02 puts it in its
section 10, "worse than one that produces it naturally". The naturally-good shape is the one arvo
already ships, where the absence of the impl *is* the fact and the attribute sits on the thing that
is absent. If the panel lands on the computed-truth encoding for the reasons 02 and 03 give, then
`Proves<C>` is the price of that choice and should be counted against it rather than treated as free.

## 7. The width encoding is a three-way trade and only one arm has been priced. Verified, contesting 02 section 8.

02's section 8 is the finding it "would most like the round to take", and its conclusion is that
"every axis whose value is subtracted, compared or otherwise computed from becomes a type carrying
its derived facts as members, which is uniform, linear, gate-free, and matches the capacity
precedent". That conclusion is reasoned entirely from the trait solver. It has a consumer-surface
cost that separates two shapes 02 treats as one, and it rests on a premise I can falsify.

`06_probes/e_width_encoding.rs`, four parts.

**Peano widths render catastrophically.** A 13.3 fixed-point:

```
error[E0277]: the trait bound `Fix<S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>, S<S<S<Z>>>, Signed>:
              AddAssoc` is not satisfied
```

arvo's widths run to 128 in the native container tables and past 256 through `WideBits`, so this arm
is not usable at arvo's range whatever it does for the solver.

**A flat table of named nats renders cleanly and leaks at the arithmetic.** `Fix<N13, N3, Signed>`
is fine. But when a subtraction row is missing the consumer is told:

```
error[E0277]: the trait bound `N13: Sub<N3>` is not satisfied
  | pub fn bad_b() { needs::<Fix<N13, N3, Signed>>() }
help: the trait `Sub<N3>` is not implemented for `N13`
note: required for `Fix<N13, N3, Signed>` to implement `Numeral`
```

`02_probes/h_widthtype.rs:26-28` implements only `Sub<Z>` and comments "a full impl would recurse;
two rows suffice to show the shape resolves", so it never exercised the recursion and never saw
either of these two renders. That is not a criticism of the probe, which was answering a different
question. It is a reason the conclusion drawn from it should not be taken as settled.

**And the premise. The subtraction does not need type position.** The spec's own words at
`spec:118`: "the significand derives by subtracting the exponent field and the sign bit". Written as
an associated const in a const-fn body it compiles with zero feature gates and the width stays a
const parameter:

```rust
pub trait Numeral {
    const LOGICAL_WIDTH: u16;
    const EXPONENT_FIELD: u16;
    const SIGN_BITS: u16;
    const SIGNIFICAND: u16 = Self::LOGICAL_WIDTH - Self::EXPONENT_FIELD - Self::SIGN_BITS;
    const IS_INTEGRAL: bool;
}
```

`check::<Fix<13, 3, Signed>>()` returns `(17, 16, false)` and `check::<Flt<8, 23>>()` returns
`(32, 23, false)`, both evaluated at const time, no gates anywhere in the file. The second tuple
independently reproduces 01's finding 8, since IEEE binary32's precision is 24 and this derivation
gives 23, which is the hidden bit that no axis carries.

So the honest statement of the trade is three-way rather than two-way, and it is decided by *what
consumes the derived value*. Where a derivation feeds a const, an associated const does it, gate
free, with the width staying a const parameter that renders as `13`. Where a derivation feeds a type,
which in arvo is the container projection and nothing else I found, the width has to be a type, and
then the named-nat table is the only arm that both solves and renders. Peano is out on rendering
regardless.

Two readings on where that leaves 02's section 8.

The first: its recommendation survives with a narrower scope. Make the container-projection input a
type, keep everything else a const, and the mixed encoding it objects to becomes a deliberate
boundary rather than an accident. That is more work to specify and less pleasant to state than
"everything becomes a type", and I think it is what the evidence supports.

The second, which I hold weakly and which someone should test rather than argue: perhaps the
container projection does not need a type either, because it is already Pattern C taking the width as
a standalone const argument (`arvo-strategy/src/container.rs`, and the sketch at
`202607291400_const-args-under-min-gca` names that escape). If the standalone-argument form covers
it, the whole width axis stays a const and section 8's problem dissolves rather than narrows. I did
not build that and I say so.

## 8. A fallible call site costs one unwrap per operation, and the orphan rule decides it. Verified.

The spec prices `Precise`'s fallibility at "call sites unwrap" (`spec:269-271`). My lens asks how many
times, and the answer is not a design choice.

`06_probes/f_orphan_on_the_carrier.rs`. The spec's carrier is `type Fallibility<T>: notko::ConstTry`,
`Just<T>` or `Outcome<T, _>` (`spec:155-156`). A consumer writing `let t = a + b + c;` on a `Precise`
composition needs `Outcome<..> + c` to typecheck, which needs `impl Add for Outcome<..>` in arvo:

```
error[E0117]: only traits defined in the current crate can be implemented for types defined
              outside of the crate
   | impl Add for Outcome<MyNum, OutOfRange> {
   = note: impl doesn't have any local type before any uncovered type parameters
```

arvo does not own `Outcome` and cannot. So under a foreign carrier there is **one unwrap per
operation, not one per expression**, three-term accumulation is three `?` or three matches, and no
amount of design taste changes it.

The escape is an arvo-owned carrier, which can implement `Add` and short-circuit, so the chain is
written once and settled once. That is the same destination 05 reaches from layout (its section 5,
the doubling of every intermediate) and from codegen (its section 6, two data-dependent exits per
element), and this is a third independent road to it, from coherence. Three unrelated arguments
converging on one structural answer is the strongest signal this panel has produced about anything,
and I would say so plainly rather than let it read as my agreeing with 05.

What it means for op's Thread B, which asks what the best form of fallible arithmetic *unlocks*
rather than what the current form costs: the thing it unlocks is the expression. `let t = a + b + c;`
with one settle at the end is a call site a numerically careful person will actually write. `let t =
(a + b)?  + c;` repeated through a kernel is a call site they will route around, and routing around
is what a tool's users do when the tool fights them. The delivery question is not a performance
question at the surface. It is whether the arithmetic still looks like arithmetic.

## 9. What a consumer actually builds with arvo is a domain newtype, and arvo ships nothing for it. Verified.

This is the finding I would put highest if I were ranking, and it is outside the question I was
asked, which the panel rule says to report anyway.

`hilavitkutin-api/src/dispatch_codegen.rs` is 420 lines whose entire job is four identifier newtypes
over arvo aliases: `PhaseId(pub Uint<5>)`, `TrunkId(pub Uint<6>)`, `FiberId(pub Uint<7>)`,
`UnitId(pub Uint<16>)`. Each carries a hand-written `ADDRESSABLE`, `ZERO`, `from_constant`, `index`,
`from_index`, `Debug` and `Default`. That file carries **twenty `lint:allow(no-bare-numeric)`
escapes**, and their own stated reasons group into exactly four categories:

| Reason as written in source | Count |
|---|---|
| `const-generic array size` | 6 |
| `typed-id to array-index boundary` | 4 |
| `array-index to typed-id boundary` | 4 |
| `Display of the raw index value` | 4 |
| `2^(Uint<N> width): the addressable-id-count bound` | 2 |

Every one of those is a gap in arvo's surface, not a failing of the consumer. arvo has no conversion
between a numeric and a `USize` index, so eight escapes. arvo's primitives have no `Debug` that
respects the fixed-point scale, so four. arvo has no way to say "an array indexed by values of this
type", so six. And arvo *does* have the width: `BitPresentation::LOGICAL_WIDTH` is declared at
`arvo/src/markers.rs:41` and implemented for `UFixed` at `arvo/src/ufixed.rs:254`, as a `USize`
associated const rather than in a position usable there, so the consumer copies it by hand into `pub const ADDRESSABLE: usize = 1 << 5`
and writes the comment "Keep in sync with the `Uint<5>` width above"
(`dispatch_codegen.rs:53-60`). That is a hand-maintained duplicate of a fact arvo already
knows, annotated with a lint escape and a tracked task number, in shipped code, four times.

Now read the spec against that. It restructures the numeral into ten axes, three contracts and six
crates, and touches not one of the four categories. A consumer landing on the new arvo would write
the same 420 lines with the same twenty escapes.

I am not arguing the numeral work is wrong. 01 through 05 established it is mostly right and I have
nothing to add to that. I am arguing that the round has an unstated model of who the consumer is, in
which the consumer is someone who does arithmetic, and the observable consumer is someone who
declares a bounded identifier and indexes an array with it. The lint escapes are the evidence,
because they are the places the discipline had to be suspended, and a library whose own consumer
suspends its discipline twenty times in one file to do routine work has told you where its surface
ends.

Two readings on what to do with it, and I genuinely do not know which is right.

Either this is out of scope for a formalization round and belongs in its own topic, in which case the
right action is to write it down now, while the evidence is in front of someone, rather than after
six crates have been split and the surface question is harder to see.

Or it belongs *in* this round, because the spec is already deciding the shape of the type that all
twenty escapes are about, and a decision like "the numeral exposes its logical width as a usable
const" or "arvo ships an `index_newtype!` macro" is cheaper to make while the type is being designed
than after. The precedent cuts this way: `bitfield!` (`arvo/src/bitfield.rs`, 459 lines) is exactly
this kind of consumer-facing macro and arvo already ships it, so the category is established and this
would be its second member.

An impression, labelled as one, on the same axis: `arvo::Fixed` and `arvo::Signed` are used zero
times in hilavitkutin despite their own doc comments saying "use this at consumer call sites"
(`aliases.rs:35-39`). An ergonomic layer that ships and is not reached for is a data point about what
the consumer actually finds ergonomic, and it is a cheap one to investigate before another ergonomic
layer is designed on the same theory.

## 10. The convention alias sets need to be modules, and feature unification is why. Reasoned.

`spec:274-278` ships every convention as an optional feature containing type aliases, `conv-ieee754`,
`conv-systemc`, `conv-matlab`, `conv-amd-vitis`, `conv-flocq`, each off by default. The design is
right and it has one mechanical consequence the spec does not state.

Cargo features are unified across a dependency graph. If crate A enables `conv-matlab` and crate B
enables `conv-ieee754`, both are on in the single build of `arvo-numeric`, for both of them. The
alias sets then coexist, and they collide by name: MATLAB has `Nearest`, `Round` and `Convergent`,
IEEE has `roundTiesToEven` and friends, SystemC has `SC_RND` and its siblings, and the spec's own
observation at `spec:285-287` is that "only the third matches IEEE's default". Two features defining
`Nearest` in one module is a duplicate definition and the build fails, for a consumer who enabled one
feature and got the other transitively.

So the alias sets have to be modules rather than flat re-exports, `arvo_numeric::matlab::Nearest`
against `arvo_numeric::ieee754::round_ties_to_even`, and the feature gates the module rather than the
names. That also buys the thing the spec says the sets are for, since
`use arvo_numeric::matlab::Nearest;` in a file that ports MATLAB code reads as the port it is, and it
makes the trap the spec names visible at the use site rather than only in the documentation.

Two readings on the stronger version. Either the modules are always present and only their contents
are feature-gated, which costs nothing to a consumer who does not use them and removes the
unification hazard entirely. Or the modules themselves are gated, which is what "off by default"
naturally means and which keeps a consumer from discovering a vocabulary they did not ask for through
rustdoc. I lean to the first because the failure mode of the second is a transitively-enabled feature
changing what compiles, which is the class of problem that is hardest to diagnose from a consumer's
chair, and because the doc-discovery objection is answered by rustdoc's own feature labels.

## 11. The preset redefinition, read from the consumer's chair. Reasoned, partly against 04.

04's section 4 says the behaviour change is legitimate pre-1.0 and that two of its four changes are
"silent value changes under unchanged spelling, the worst migration shape there is". I agree with the
migration obligation it asks for and I want to complicate the framing, because it reads the change
as uniformly a risk and at the dominant consumer site it is a repair.

`arvo/tests/strategy_semantics.rs:5-6` states the shipped semantics in its own words: "Hot / Warm:
wrapping (single-op container overflow wraps modulo container width)". Read that against
`PhaseId(pub Uint<5>)`. `Uint<5>` defaults to `Warm` (`aliases.rs:71`), whose container is doubled,
so today a `PhaseId` wraps at the *container* width and not at five bits, and can hold 5000. The
consumer's own documentation asserts the opposite: "`PhaseId` wraps `Uint<5>`, so it addresses
indices `0 .. 2^5`" (`dispatch_codegen.rs:55-57`). Under the spec's redefinition, `Warm` clamps out
of range, and if "range" is the numeral's range rather than the carrier's, which `spec:230-232` says
it is, then `PhaseId` starts enforcing the bound its own documentation already claims.

So at 22 of the 31 arvo-typed sites in hilavitkutin, the redefinition changes behaviour from
something the consumer documents incorrectly to something the consumer documents correctly. That is
still a silent value change and still needs the audit 04 asks for. It is not uniformly a risk, and a
migration note that says so will get read more carefully than one that reads as a warning.

The other reading, which I hold and which is 04's: silent is silent, and a consumer who wrote code
against the observed behaviour rather than the documented behaviour has no compiler event to prompt
them. `hilavitkutin-providers/src/adapt_ema.rs:72` is 04's example and it is a real one, since its
own comment documents a deliberate mix of Hot wrapping multiplies with saturating final adds. The
difference between us is only which sentence goes first in the migration note.

One thing I would add to 04's mitigation list from my lens. The audit obligation should be discharged
by *reading the four `.stderr` fixtures and the twelve `strategy_semantics` assertions as a
specification of the old behaviour*, and writing the new ones before the implementation changes.
Those files are the only place the old semantics is stated precisely, the spec's preset table is the
only place the new one is, and a diff between two tables is a much better artifact for an afternoon's
audit of twenty sites than either table alone.

## 12. What hilavitkutin's sites look like after, written out. Reasoned.

The brief asks. Taking the shape of sections 4 and 5, and holding the numeral nominal:

```rust
// today
pub struct PhaseId(pub Uint<5>);
pub type Sample = UFixed<{ ibits(64) }, { fbits(0) }, Hot>;
pub type BlendFactor = UFixed<{ ibits(0) }, { fbits(16) }, Hot>;
type Mass = arvo::Uint<27, arvo::strategy::Cold>;

// after, if `Uint` and `UFixed` stay aliases over a nominal numeral
pub struct PhaseId(pub Uint<5>);                                   // unchanged
pub type Sample = UFixed<64, 0, Hot>;                              // the ibits/fbits wrappers
pub type BlendFactor = UFixed<0, 16, Hot>;                         //   go away with the meta-newtypes
type Mass = arvo::Uint<27, arvo::strategy::Cold>;                  // unchanged
```

Nothing at a declaration site changes except that the `{ ibits(..) }` and `{ fbits(..) }` boilerplate
can go, which is nine sites improved and twenty-two untouched. That is the correct outcome for a
restructuring round and it is what the spec claims at `spec:315-318`, so on the input side the
spec's compatibility claim holds and I want to say so plainly since most of this file is about the
output side.

What does change is the failure surface, which is sections 1 through 6, and one thing at a bound
site. The generic algorithm crates today write `W: Add<Output = W> + TotalOrd + FromConstant`
(`arvo-graph/src/lib.rs:10-12`). After the round they write, at minimum, `W: AddAssoc + ...` wherever
they fold, and 05's section 3 counts eleven `Add<Output = W>` sites across graph, comb and spectral
plus four operations wide at L3. Those bounds are one name each and do not leak an axis, which is the
question the brief asks: **no, the ten axes do not leak into a downstream signature.** A downstream
crate names traits, not axes, and probe D confirms the failure is still reported at the consumer's
own instantiation rather than inside the plumbing.

The one place machinery does leak is a downstream crate that wants to be generic over a *numeral*
rather than over a bound. The spec's sketch obligation 1 asks for exactly that ("a function generic
over a numeral threading the bound through its own generic code", `spec:328-330`). I did not build
it and I flag it as the place to look for leakage, because that is where 05's section 1 found a
five-line where-clause for one function and where the projection depth is highest.

## 13. Engagement with the prior five, kept short

**01's finding 4, the range boundary, is a consumer-surface finding and is filed as a mathematical
one.** Its two examples are what a person observes: `Precise` refusing results every standard rounds
to MAX, `Hot` wrapping a value at `MAX + 0.3q` to near the bottom. Both are the kind of thing that
gets reported as a bug against arvo by someone who was right. I would raise its priority on that
ground alone and I would add that its proposed repair, round-first on the extended lattice then
resolve, is the one that makes the `conv-*` aliases mean what a person porting code assumes they
mean, which is section 10's argument arriving from the other side.

**02's section 10, the tagless-final reading, and its own second reading against it.** 02 states the
cost of the dual encoding as "a design that needs a workaround to produce a good error is worse than
one that produces it naturally" and asks someone to write both `.stderr` fixtures before choosing.
Section 6 is half of that, done: the computed encoding needs `Proves<C>` to produce a good error, and
`Proves<C>` works. The other half, whether the sealed-blanket encoding produces one naturally at this
surface size, is a two-hour job and I did not do it. It is the highest-value remaining fixture in the
panel and it decides between two shapes the panel currently holds without a tiebreak.

**03's section 6, on D67's test checking type-checking rather than semantic fidelity.** Fully agreed,
and from my lens the strongest thing about its proposal is that the vendor test vectors are *already
published*, so the oracle-authoring cost 03 flags against its own section 3 is zero here. I would go
one step further than 03 does: the boundary cases it names (last midpoint before MAX, first
over-range midpoint, a tie at MAX) are also the exact inputs a consumer hits when they port code and
get a different answer, so the same table serves as the migration document for anyone arriving from
SystemC or MATLAB. One artifact, two jobs.

**04's section 5, the algebra ladder as speculative bloat.** I agree with the conclusion and I want
to sharpen the reason from my lens rather than from the economics one. A declared trait with no impl
site and no bound site still costs the consumer, because it appears in rustdoc, in the crate's item
index, and in the "the following other types implement trait" help lists that rustc prints on a
near-miss. The `no_signed_identity_on_unsigned.stderr` fixture is a live example of that last one:
its help lists eight implementors "and $N others". Every uncalled trait in the crate is a line in
someone's search results for a trait they will then have to rule out. That is a real cost and it is
paid by the reader rather than the maintainer, which is why it does not show up in a
carrying-cost argument.

**05's section 2, delivery as a `Lowering` member.** I think the claim holds and I want to add the
one consequence that falls in my lens and cuts against it. If delivery is `Lowering`, then two
compositions that are equal as numerals and as policies have different call-site shapes, so a
consumer reading a type cannot tell from the parts they care about whether their arithmetic returns a
value or a carrier. 05's own access-discipline caveat is the same worry from the semantic side. Under
the modifier shape of section 5 there is a mitigation: delivery is spelled by a named modifier
(`DeliveredAs<Precise, Absorbing>`), so it is visible in the type and in the error without being a
column in the preset table. That does not settle 05's sorting question and it removes the ergonomic
objection to whichever way it settles.

**05's section 0, on recompiling the prior member's conclusion.** It is the practice that caught 02's
section 7 and it caught 02's section 4 row again here, in section 6 above. I would make it explicit in
the panel's own record: the sequential panel's value is not that each member reads the last one, it is
that each member *re-runs* the last one where a conclusion is load-bearing. Two of the six files so
far have carried a consequence their own probe does not support, which is a rate high enough to plan
around rather than to note.

## 14. What I did not get to

The other half of 02's fixture request from section 13 above, which is the sealed-blanket encoding's
error text at this surface size, side by side with `Proves<C>`. It decides a question the panel is
otherwise going to settle by argument.

Whether the modifier composition of section 5 needs canonicalisation, and what a `.stderr` fixture
does when two spellings of one composition are both legal. I raised it against my own proposal and
did not build it.

Whether `Uint`, `Fixed` and the other aliases should survive at all as aliases once the numeral is
nominal, since `Number<Fix<12, 0, Unsigned>, Warm>` is not much longer than `Uint<12>` and is
self-describing. I suspect the answer is that the short aliases stay and that this is the one place a
newtype face genuinely earns its keep, and I did not test it.

The domain-newtype macro of section 9. It is a one-day sketch against
`hilavitkutin-api/src/dispatch_codegen.rs`, the target is four types with twenty lint escapes, and
the acceptance criterion writes itself, which is that the escape count goes to zero.

And a measurement rather than a probe: how many of arvo's own 654 tests would have caught any of the
six findings above. My impression, labelled as one, is none, because the suite tests values and the
findings are all about what the compiler says, and the only surface that tests what the compiler says
is the nine `tests/ui/` fixtures. That ratio, nine to 654, is itself worth someone's attention in a
library whose consumers meet it through the type system.

---

**Summary for the next member.** The alias problem is not a forecast, it shipped: hilavitkutin's
dominant spelling is already an alias, 22 sites against 9 for the raw form, and in the environment
that consumer is actually in, with `generic_const_exprs` enabled at `hilavitkutin/src/lib.rs:24`,
`Uint<12, Hot>` renders as `UFixed<arvo::::aliases::Uint::{constant#0}, ..., ...>` and spills to a
long-type file, while arvo's nine `tests/ui/` fixtures pin a spelling and an environment no consumer
uses (section 1, verified). 04's finding needs one refinement and one correction: the alias survives
at the caret and dies in the trailing note, and which one a consumer reads is decided by whether the
trait carries `#[diagnostic::on_unimplemented]` rather than by the alias (section 2); and newtype
faces recover the numeral half and leave the policy half rendering in full, `UFixedFace<13, 3,
Pol<Quant<ToEven, ToEven, ..., ..., ...>, ...>, ...>`, which is the half a consumer actually varies
(section 3, verified). The category the thread was asking for is that **ten axes are free in the
error surface if every axis value the consumer selects arrives through a named type**: with a nominal
numeral and nominal presets, `Number<Fix<13, 3, Signed>, LayoutOf<OverRangeOf<Warm, Refuse>,
Bitpacked>>` renders complete, untruncated, with no long-type spill, and reads as the sentence the
consumer meant (sections 4 and 5, verified), and the delegating modifier is how someone changes
exactly one of ten axes while the derivation still resolves through it. Under the computed-truth-value
encoding the panel is converging on, `#[diagnostic::on_unimplemented]` on the law trait never fires
and the consumer reads `False: IsTrue`; 02's `c4_diag.rs` fires an attribute that can say what went
wrong but not what it went wrong about, and parameterising the verdict marker by the composition
repairs it in four lines (section 6, verified). 02's section 8 conclusion that computed widths must
become types is two claims: Peano widths render as `Fix<S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>, ...>`
and are unusable at arvo's range, and the significand derivation needs no type position at all, since
an associated const in a const-fn body computes it gate-free and independently reproduces 01's
binary32 off-by-one (section 7, verified). A fallible call site costs one unwrap per operation rather
than per expression, and it is the orphan rule that decides it, since arvo cannot implement `Add` on
`notko::Outcome` (E0117, section 8, verified), which is a third independent road to the arvo-owned
carrier 05 reaches from layout and from codegen. And the finding outside the question I was asked:
what a consumer actually builds with arvo is a domain newtype, `hilavitkutin-api/src/dispatch_codegen.rs`
is 420 lines of four of them carrying twenty `lint:allow(no-bare-numeric)` escapes in four categories
that are all missing arvo surface, and the ten-axis spec touches none of them (section 9, verified).
I rule on nothing; op decides.
