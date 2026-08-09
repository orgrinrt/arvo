# The profile axis and the tiers: what a program demands and what its context supplies

**Date:** 2026-08-07
**Position:** after `142c_op_checkpoint_thirtyfive.md`, the checkpoint that opened this question.
**Probes:** `143_probes/`, fourteen of them, with `run.sh` and the captured `output.txt`.

The design has a mechanism it has never written down, and the checkpoint that found the omission then
described the mechanism two incompatible ways within one file before op corrected it twice. That is not a
criticism of the checkpoint, which did the valuable thing. It is the reason this file starts by separating
three things the word "profile" has been carrying, rather than by answering the six questions in order.

The separation is the whole of the contribution. Everything else here follows from it, and most of the
confusion in the stretch `142c` diagnoses follows from its absence. There are three genuinely different
sources of variation in this design and they have been arriving under one name. One is the strategy index
itself, which is what the preset tables are. One is the build condition, which reaches exactly one strategy
and reaches it because that strategy is defined by imitation. And one is the *supply site*, the question of
where the strategy at a given use came from, which is not a variation in the answer at all but a variation
in who provided it. Collapsing any two of these produces a false statement, and the record shows all three
collapses having been made.

The framing I bring to it is the one I have spent a career on and it fits without being forced. An
effect describes what a computation does to its context; a coeffect describes what it demands from it. The
strategy at a use site is a demand: this expression requires a strategy in order to have a meaning at all,
and it does not produce one. Everything the six questions ask is a question about how that demand is
discharged, by whom, at what granularity, and what happens when two parties try to discharge it at once. The
last of those is the soundness obligation, and it is the one I would put in front of op first.

Two more preliminaries, then the work.

**On the gate.** I checked the brief's cited rule text before reasoning from it. `no-bare-primitives.md:25`
says the container is chosen by `Strategy` "and any active notko optimisation profile" and that bare
primitives "bypass the strategy/profile pipeline"; `arvo-always-optimal-internals.md:19` names "notko profile
hooks: AST-rewriting macros (`#[optimize_for]`) that retarget the substrate per build profile";
`hilavitkutin--cookbook.md:37-41` shows `#[optimize_for(hot)]`. All three quotes are accurate. **Two of the
three are wrong about the mechanism they name**, and the correction is in the next section. The brief is
otherwise sound and its account of what went missing from the standing base reproduces: `110` and `124`
carry "profile" only in the deployment and workload senses.

**On what kind of statement this file is making.** The canon carries intent, not implementation. So the
sections below state what must be true, name the quantities and relations exactly, and point at probes for
the question of whether a thing is doable at all. The probes are spikes. They prove one thing each, their
naming and arities are scaffolding rather than proposals, and they are cited for what they established and
never for how they were written.

## What the shipped notko already answers, and what it refutes in the brief

`#[optimize_for]` does not exist. The shipped attribute is `#[profile]`, it lives in `notko-macros`, and
its argument is a capitalised tier ident. Both rule files that name it are stale on the spelling, and one of
them prints an argument that fails the build: `#[profile(hot)]` is refused, live, against the built
proc-macro crate, with

```
error: unknown profile tier `hot`. built-ins: Hot | Warm | Cold. custom tier expected at
`notko-optimizers/hot.rs` (crate-local) or $NOTKO_OPTIMISERS_PATH/hot.rs (via notko-build).
```

(`143_probes/p13_notko_attribute/lib.rs`, output at `143_probes/output.txt:215`). The diagnostic is good and
is worth copying: it enumerates the built-ins and names where an extension would live. Note also that the
built-ins are three. **There is no `Precise` tier**, while `arvo-strategy` holds four strategies by D72
(`124:3602`). Whatever the relation between a profile value and a strategy turns out to be, it is not a
bijection in shipped form.

More useful than the naming defect is that the shipped mechanism already exhibits the structure this file is
trying to establish, and exhibits it clearly enough to settle two of the six questions on evidence.

**Three sorts wear the word, and only two of them are named.** `notko-macros-core/src/tiers.rs` declares a
`Tier` trait with `NAME`, `STRATEGY` and `INLINE`, ZST markers `Hot`, `Warm` and `Cold` implementing it, and
a separate `Strategy` enum with variants `Passthrough`, `Hot` and `Cold`. So a **tier** is an open-ended
named thing selected at a site; a **rewrite strategy** is a closed three-element set the tiers map onto; and
the map is not injective, since `Warm` maps to `Passthrough`. Custom tiers are declared in
`notko-optimizers/<Name>.rs` with a `based_on` field naming a built-in plus parameters, and `resolve_tier`
in `discover.rs` searches built-ins, then crate-local files, then an accumulated path, then errors. **So the
name space is open and the behaviour space is closed**, which is a good shape and one arvo should consider
adopting rather than re-deriving.

**And the realisation is a function of the tier and the build condition together.** `rewrite/hot.rs:26-30`
emits two functions, gated `#[cfg(any(not(feature = "internal"), debug_assertions))]` and
`#[cfg(all(feature = "internal", not(debug_assertions)))]`. The `Hot` tier yields `Outcome<T, E>` in one and
`Just<T>` with `Err` lowered to a panic in the other. `Cold` yields `Outcome` unconditionally. `Warm`
rewrites nothing at all.

Two things follow, and the second is the one that matters for arvo.

The lexical selection and the build-condition dependence are **both present in the shipped mechanism, at
different levels**. That is why `142c`'s first section and its correction both felt right to their author
and contradicted each other. They are each describing one level.

**And the macro does not read the build profile.** It cannot: a proc macro's own `cfg!(debug_assertions)`
reports the macro crate's compilation, not the consumer's. The shipped design sidesteps this by emitting
both arms and letting `cfg` choose, which is the only correct move available. For arvo the same constraint
binds and the same move is available in a cheaper form, because arvo's cells are already resolved by a
const-trait projection rather than by a macro. An attribute can select the strategy; it must not try to
resolve the cell.

## The reconciliation: three sources of variation, kept apart

This is the section I would ask op to read if he reads one.

**First, variation across the strategy index.** `<Warm as Lowering<K>>::StoredWidth` and
`<Hot as Lowering<K>>::StoredWidth` differ. That is not a defect and it is not news: it is what the tables
at `124:2604` and `124:2653` are. Op's sentence, "none of the table items or strategies are the same through
profiles", read against his own correction that "the notko profiles roughly translate to arvo strategies",
most plausibly says exactly this. The complaint was that the panel had been arguing over single cells as
though a cell were a universal constant, when a cell is indexed and the index is the point. I offer that as
a reading rather than as his meaning, and I name below what would distinguish it from the other one.

**Second, variation of one strategy's cells with the build condition.** This is separate, it is real, and it
survives the correction. `140b:19-21` defines `Warm` as behaving like a native Rust primitive, and
`142b:19-22` draws the consequence: Rust's integer overflow behaviour panics under `debug-assertions` and
wraps in release, so `Warm`'s overflow row is a function of the build condition rather than a cell. That is
a cargo-profile dependence and nothing in the correction touches it.

It reaches `Warm` and not the others, and the reason is structural rather than accidental. `Hot`, `Cold` and
`Precise` are defined by rules: as fast as possible, as small as possible, most precise at a price. A rule
does not vary with a build. `Warm` alone is defined by **imitation**, and an imitation inherits every
variation of the thing imitated. So the correct general statement is not "cells vary with the profile" and
not "cells are constants". It is:

> A cell is a constant unless the strategy's definition is an imitation, in which case the cell is whatever
> the imitated thing does, including where that varies.

That is one sentence, it is checkable against each of the four definitions, and it predicts which cells move
without anyone having to enumerate them. It also explains why this keeps being rediscovered: the table's
form invites reading every entry as a value, and one entry is a reference.

**And it contradicts `142c`, which I should say plainly rather than fold in.** `142c:70-72` calls the
build-condition dependence "a general property of the axis" and faults `142b` for having made it "sound like
a special property of one strategy". My rule says the opposite: it *is* a property of one strategy, and it is
one because that strategy alone is defined by imitation. I hold that reading because I can check it against
each of the four definitions at `124:2578-2580` and only `Warm`'s is a reference to something external, while
I can find no derivation anywhere in the base by which "as fast as possible" or "as small as possible" would
yield a different cell in a debug build. If someone has one, my rule is wrong and `142c` is right.

This is the first item in the closing section for that reason. Two readings of one sentence of op's, they
make different demands on the canon, and the disagreement is between this file and a checkpoint rather than
inside either.

**Third, variation in the supply site.** Where did the strategy at this use come from? This is not variation
in the answer. It is the coeffect question, and it has three answers, treated in the next four sections.

The three collapses that produced the confusion, named so a later reader can check for them. Reading the
third as the second: `142c`'s first section read the lexical attribute as a build knob, and op's correction
at `142c:139-146` retracts it. Reading the first as the third: arguing over which value a cell holds, which
`142c:88-90` records as the false premise the whole stretch ran on. And reading the first as the second, or
the second as the first, which is the live disagreement above and is unresolved.

## What the attribute rewrites

**Granularity, from the shipped mechanism.** `rewrite/mod.rs:24` parses the annotated item as a `syn::ItemFn`
and nothing else. Not a block, not an expression, not an impl, not a module. The rewrite replaces
`sig.output` and then runs a `VisitMut` over the body. So today the unit is exactly a function item, and
op's phrase "targeted function or item scope optimizations" is presently the narrower of its two readings.

For arvo the granularity question separates from the rewrite question, and I think that separation is the
useful move. There are four routes and they differ in what they touch, not in what unit they cover.

**R1, rewrite the elided strategy position.** The attribute walks the item, finds arvo type spellings whose
strategy parameter is omitted, and fills it. Bare primitives untouched, explicit strategies untouched.
Syntactic, local, and it requires the spellings to have an elidable strategy position, which they do.

**R2, rewrite bare primitives as well.** `u32` inside the scope becomes the corresponding arvo numeral at the
attribute's strategy. This is the only route that reaches the tier-one consumer who never writes an arvo
type. It changes caller-visible parameter types, and it requires deciding that a given `u32` is a numeric use
rather than a discriminant or an index, which is a judgement a syntactic pass cannot make.

**R3, no type rewriting at all: the attribute injects a scope selection.** If the strategy-elided spellings
are a per-strategy set of type aliases, then selecting a strategy for a region is importing that set into the
region, and Rust's own name resolution supplies every property the mechanism needs. `p7_scoped_supply_and_precedence.rs`
compiles all four at once with no proc macro in the file: a module-scope import supplies the ambient
strategy; an inner-scope import overrides it; nesting works because lexical scopes nest and the innermost
selection wins with the outer one restored on exit; and a fully written spelling naming its own strategy is
untouched by any enclosing import.

That is the cheapest mechanism I can find that does the job, and its cheapness is the argument for it: the
attribute's arvo half becomes one injected `use` at the top of the item's body, and every question about
precedence and nesting is answered by rules Rust already has and consumers already know. It also composes
with the notko fallibility rewrite rather than competing with it, since that rewrite is doing something a
scope selection cannot do.

R3 has two limits and both are worth stating rather than hiding.

**It does not reach the signature.** `p12_body_scope_does_not_reach_the_signature.rs` establishes it: a body
selection leaves the declared parameter and return types resolving at the enclosing scope, and the mismatch
surfaces as an ordinary `E0308` naming both strategies. So under R3 alone, `#[profile(Hot)] fn f(x: UInt<5>)`
has a `Warm` signature and a `Hot` body. Loud rather than silent, which is the important half. Whether it is
a gap or the correct behaviour is a design call: the signature is the contract with callers, and there is a
real argument that an attribute should not silently change what callers must construct. notko's shipped
attribute takes the other view for the return position specifically, and documents it as the point.

**It requires an alias set per strategy.** Four sets, one per ratified strategy. I flag this against
`137b:29-42`, where op refused an enumeration three times on the ground that the information is in the
typestate and only the spelling is missing. This is a different object: what is enumerated is the four
strategies D72 already fixes as a closed set (`124:3602`), not the open set of widths. But it is close enough
to the refused shape that I will not assume the distinction is accepted, and I name it as something for op
rather than deciding it.

**R3b, the combination.** The attribute injects the scope selection for the body and rewrites the elided
strategy positions in the signature. It gets R1's reach at R3's cost for the body, and it is the route I
would expect to end up wanting. Naming it separately because the choice between R3 and R3b is exactly the
signature question above, and it should be decided as that question rather than as a route preference.

## What inference can derive, and what it must assume

This is the question I was sent for and the honest answer has two halves that pull opposite ways.

**From a bare `T: Add<Output = T>`, arvo derives nothing about width, range, resolution, container or
signedness.** Not approximately nothing. Nothing. `p8b_bound_determines_no_width.rs` asks the bound for the
one quantity a container derivation needs and gets `E0576: cannot find method or associated constant WIDTH in
trait Add`. There is no associated item there to carry a width, no bound relating `T` to a bit count, and
`Add<Output = Self>` does not even establish that `T` is a number.

So if op's sentence is read as "arvo derives the typestate from the trait bound", it is false and no
mechanism makes it true. But I do not think that is what it says. Re-reading it:

> The algos and all will work on contracts... Then **as consumer supplies whatever they have**, they only see
> and understand that this trait bound needs to be implemented.

The consumer supplies `T`. Every representational fact travels **with** `T`, because the consumer already
fixed it when they chose the type. Arvo does not derive a representation from nothing; it **reads** the one
that arrived. That is the second half:

> Under the hood we derive the arvo end fully to cover soundness and validity and **infer the special
> compile-time branches and behavioral arms**, and extend the typestate through the public api.

What is derived is which internal arms apply given a supplied `T`, and the typestate of the *output*. What is
propagated is the representation of the *input*. Those are different operations and the design should not use
one word for both.

Stated in the frame that makes it precise: **the algorithm's bound is its coeffect, its demand on the
context. The context discharges it by supplying a type. Arvo's derivation runs downstream of the discharge,
never upstream of it.** An algorithm that tried to derive a width from its own bound would be inventing a
demand's answer, which is the direction that never composes.

**One thing the bound genuinely does determine, and it is not nothing.** `Add<Output = T>` asserts totality,
and totality excludes a refusing strategy. `Precise` is fallible by ratified decision (`124:2631`: "its
arithmetic returns through the refusing branch of the quantisation's fallibility projection, so call sites
unwrap") and its `OverRange`/`UnderRange` row is `Refuse`/`Refuse` (`124:2605`). A numeral whose addition
returns a refusing wrapper cannot satisfy `Add<Output = Self>`, and the compiler says so at the call site
with both types named:

```
error[E0271]: type mismatch resolving `<PreciseNum as Add>::Output == PreciseNum`
note: expected this to be `PreciseNum`
   = note: expected struct `PreciseNum`
                found enum `Result<PreciseNum, Refused>`
note: required by a bound in `fold_total`
```

(`p8_what_a_bound_determines.rs`, `output.txt:115-134`.) **So a total signature is a real, checked,
diagnostically decent statement that one of the four strategies does not apply here.** That is a genuine
inference, it costs nothing, and it is the kind of thing worth having in the canon because a consumer reading
that error learns something true about the design.

**What must be assumed, and it turns out to be one thing.** When the supplied `T` is a bare Rust primitive,
arvo needs a numeral and a strategy for it. The numeral is forced: `u32` is thirty-two bits, integral,
unsigned, and there is nothing to choose. The strategy looks like a default and is not. `140b:19-21` defines
`Warm` as what a native Rust primitive does, so **the strategy a bare primitive bridges to is `Warm` because
that is what `Warm` means**, not because someone picked it. The tier-one entry point therefore has no free
parameter at all, which is a much stronger position than having a well-chosen default, and it is the reason
the tier-agreement obligation below discharges by construction rather than by care.

I will flag one consequence that follows and is not obviously wanted. If R2 is taken and the attribute
rewrites bare primitives, then `#[profile(Hot)]` over code written in `u32` changes the debug-build overflow
behaviour of that code from panic to wraparound. That is a real semantic change to code whose author never
mentioned arvo. It is defensible, since it is what "targeted optimizations that rewrite the scoped content"
asks for, and it is exactly why the envelope question in the boundary section below is not ceremony.

## The precedence rule

Two sources of one fact must not disagree silently. That is the soundness obligation and it is the only one
in this file that I would call load-bearing rather than useful.

The grade at a use site must be uniquely determined. When more than one source offers one, there are exactly
three sound dispositions and no fourth.

**Ordered precedence.** One source always wins; the other is discarded. Sound, and silent, which is the
failure mode this obligation names.

**Refusal.** Disagreement is an error. Sound and loud, and it makes the attribute unusable over any body
that pins one thing.

**Join.** The two grades combine by an operation on the grade set. Sound if and only if the set carries such
an operation and it is associative and idempotent, so that nesting depth and annotation order do not change
the answer.

**The join is unavailable, and the reason is a ratified decision rather than a preference.** A join needs the
strategy set to be closed under it. The four strategies are not a chain and are not a lattice under any order
op has given: `124:2578` records them as three extremes on three different axes (fastest, smallest, most
precise) plus a compromise. Under the product of those three orders, `join(Hot, Cold)` is a point that is
both as fast as possible and as small as possible, and there is no marker there. D72 gives `arvo-strategy`
"`Hot`, `Cold`, `Warm`, `Precise`, and nothing else" (`124:3602`), so producing the missing point requires a
fifth marker a ratified decision forbids. **The disposition is therefore forced to be precedence or refusal**,
and anyone reaching for a join later should be sent to this paragraph rather than allowed to invent an order.

Within precedence, op's sentence settles the direction: the attribute rewrites "**IF** no specific config is
being used that declares intends use of specific thing". Explicit wins. And that direction is not only op's
call, it is entailed by the already-ratified downstream contract, which I take up in the boundary section.

So the shape I would put to op is precedence **plus a report**, which is neither of the two silent options:

> Where an enclosing supply and an explicit declaration both reach a use, the explicit declaration wins, and
> the mechanism reports every use it did not rewrite for that reason.

`arvo-toolbox-not-policer.md` already fixes this shape: "diagnostic, not directive", warn where a choice has
a non-obvious consequence, never refuse a legitimate one. A consumer who writes `#[profile(Hot)]` over a body
constructing an explicitly `Precise` value has asked for two things and should be told which one they got. A
report costs a lint and buys the only property the obligation actually demands, which is that the
disagreement is not silent.

**Nesting is sound without further argument.** For any use site, the enclosing supply sites are the
attributes and scopes lexically containing it, and lexical containment is a tree, so that set is a chain and
has a unique innermost member. Innermost wins is therefore a total function of position, and uniqueness of
the grade is preserved at arbitrary depth. `p7`'s `nested` exercises it and shows the outer selection
restored on the way out.

**What counts as "a specific config declaring intended use", which is the part op left open.** I would draw
it at one line and it is checkable: a declaration is anything that names a strategy in a position the
compiler resolves, and it is *outside the reach* of any mechanism that supplies an elided one. Three cases
fall out and the third is the one that matters.

A strategy written in the type at the use site is a declaration. Obvious.

A strategy written in a domain alias's definition is a declaration, **and the reach limit here is forced
rather than chosen**. `p6_alias_identity_across_scopes.rs` is the argument. If a scoped mechanism could reach
through `StrHandle` and make it mean `UInt<5, Hot>` inside one function and `UInt<5, Warm>` outside it, then
the two occurrences denote different types, and a value of one cannot be passed to a function taking the
other:

```
error[E0308]: mismatched types
     expected `UInt<5, Warm>`, found `UInt<5, Hot>`
```

Tier two's whole proposition is one name meaning one thing everywhere. A mechanism that reaches through the
alias breaks that at every function boundary. So **an alias defined outside a scope keeps its own site's
resolution**, and `p7`'s fourth case shows the scoped mechanism doing exactly that with no special
handling: name resolution gives it for free.

A strategy *elided* in a spelling written inside the scope is not a declaration. It is the absence of one,
which is what the mechanism exists to fill.

I offer that line as a proposal. What recommends it is that it is stated without reference to any particular
mechanism, so it survives choosing among R1, R2, R3 and R3b, and that the second case is not a taste call at
all but a consequence someone will otherwise discover by shipping it.

## The profile value against the strategy

Op said "roughly translate" and "pretty much analogous" rather than "are", twice, and the shipped notko says
what the residue is. There are three sorts here and the design currently spends one word on all three.

A **tier name** is a lexical key written at a site. Its set is open: built-ins plus whatever
`notko-optimizers/<Name>.rs` files a project adds, resolved in the order `discover.rs:36-42` fixes.

A **strategy** is a type-level marker occupying a parameter position, and its set is closed at four by D72.

A **rewrite strategy** in notko's own sense is a third thing, a closed three-element behavioural set
(`Passthrough`, `Hot`, `Cold`) that tier names map onto many-to-one.

The residue in "roughly translate" is exactly this: **a tier name and a strategy are different sorts, and
the map between them is neither total nor injective in shipped form.** Not total, because `Precise` has no
tier. Not injective, because a custom tier declares `based_on` a built-in and several may share one base.
And they are different sorts because one is a key resolved by a macro against a search path while the other
is a type resolved by the trait solver.

Two readings of what to do, and I do not think this is mine to settle.

**Identify them.** Make the attribute's argument name a strategy directly, so the tier set is the strategy
set, and drop the extra level. Cheapest, removes the three-sort confusion at its source, and the shipped
attribute already takes `Hot` / `Warm` / `Cold` idents that read as the strategies. Its cost is that the
extension mechanism goes: a project can no longer name a tier of its own without adding a strategy, and D72
says there are four and nothing else.

**Keep them apart and say so.** A tier is a named, extensible key that resolves to a strategy plus
parameters. Arvo's four strategies stay closed; the open extension lives entirely on notko's side. Costs a
level of indirection and a paragraph of canon, buys the extension point back and keeps D72 intact.

**What would distinguish them:** whether anyone wants a project-local tier that is not one of the four. The
shipped `notko-optimizers/<Name>.rs` path is evidence that op wanted that on the fallibility axis. Whether it
transfers to the numeric axis is a question about intent and belongs to him.

**And one thing I would ask for regardless of which is chosen.** Three sorts sharing the word "strategy",
with overlapping value names, is precisely the condition that produced the loss `142c` records: a reader who
meets "profile" or "strategy" cannot tell which sort is meant and reconstructs the wrong model. `140b:49-52`
already names this as a presentation defect in the canon and asks for a form that makes an intent stick. A
naming that distinguishes the sorts would do more for that than another restatement would.

## Agreement between the tiers

The obligation: a cell reached by tier one and a cell reached through a tier-two domain alias must land in
the same place, or the design answers one question twice.

**It discharges by construction, and the construction is the derivation in the inference section.** Tier one
hands arvo a bare primitive; the primitive bridges to a `Warm` numeral; and it bridges to `Warm` because
`Warm` is *defined* as what a native Rust primitive does (`140b:19-21`). Tier two writes
`type Handle = UInt<32, Warm>` and gets `Warm` because they wrote it. Same row, and not by coincidence: the
tier-one path has no free parameter to get wrong.

`p11_tier_agreement.rs` states the obligation as a type equality the compiler checks, with a live negative
control asserting the same thing against the wrong row, which fails at `E0277` naming both types. A test
whose negative control does not fail is not a test, and this one's does.

Three things follow that are worth having in the canon.

**The agreement is a theorem with one hypothesis**, and the hypothesis is the `Warm` definition. If `Warm`
ever stops being defined by imitation, the tier-one bridge stops being derived and becomes a choice, and this
section becomes a review obligation rather than a construction. That is worth writing down at the point the
`Warm` definition is stated, since it is a consequence of it that nobody would otherwise connect.

**The agreement is per-tier-one-entry, not global.** It says the *default* path agrees. It says nothing about
a tier-one consumer under an enclosing attribute, where the ambient strategy is `Hot` and the bridge yields
`Hot`. That is not a disagreement between tiers; it is the attribute doing what it was written for, and the
tier-two consumer inside the same scope gets `Warm` from their alias by the reach limit above. **So under an
attribute the two tiers genuinely do diverge**, correctly, and the canon should say so rather than let a
reader infer that agreement is unconditional.

**And the third tier is the one the agreement is silent about.** `142c`'s closing correction is that rarity
raises the ergonomic bar rather than lowering it, because the alias-definition site is written by someone who
does not know the plumbing and never builds fluency in it. The agreement obligation is about *what* the two
routes compute. It says nothing about whether the tier-two writer can spell the alias correctly on first
contact, which is the constraint op ranks above the plumbing. I have nothing to add to the bridge question
here and I note that this file does not touch it.

## The output generic and its default

Op names a mechanism: "it will have a generic to describe the output, and they can override its default
whatever that might be, to have our end do the simplification and arvo erasure for the return Val".

**The mechanism as literally described has no spelling in Rust, and I established that four ways before
looking for an alternative.**

The default cannot sit on the function's own type parameter. `p1_fn_type_param_default.rs`:

```
error: defaults for generic parameters are not allowed here
   = note: `#[deny(invalid_type_param_default)]` (part of `#[deny(future_incompatible)]`) on by default
```

It can sit on a trait's type parameter, syntactically. `p2_trait_type_param_default.rs` compiles. But the
default is inert where it would have to work. `p4_default_taken_without_context.rs` removes every contextual
clue and the call fails:

```
error[E0283]: type annotations needed
note: multiple `impl`s satisfying `Piped: Dot<Piped, _>` found
```

So `p2` succeeded only because its return position supplied the answer, and the default did nothing.
`p5_struct_default_is_syntactic.rs` separates the two halves and confirms the general rule: a written type
path elides to the default, and an inference variable never receives one. **Type parameter defaults in Rust
are a syntactic elision on a written path, not an inference fallback.**

The output cannot be an associated type either, because an associated type is determined by the impl and the
consumer has nowhere to say otherwise. Two impls for one self type is `E0119`
(`p3_assoc_type_cannot_be_overridden.rs`), and the only route past `E0119` is specialisation, which is
forbidden.

And no nightly gate buys past it. The feature that would have done exactly what op describes existed:

```
error[E0557]: feature has been removed
   = note: removed in 1.82.0; see <https://github.com/rust-lang/rust/pull/127655>
   = note: never properly implemented; requires significant design work
```

(`p9_default_type_parameter_fallback.rs`.) `default_type_parameter_fallback`, rust-lang/rust#27336, made a
parameter's default act as an inference fallback. It was never properly implemented and it is gone. So there
is nothing to vet, nothing to wait for, and the wall is the language's rather than the spelling's.

**The intent survives the mechanism, and it survives by moving the choice from the output to the input.**
Rust determines outputs and consumers determine inputs, so a demand expressed at the output has nowhere to
sit and the same demand expressed at the input has a natural home. `p10_output_by_projection.rs` builds it
and compiles clean: the algorithm's return type is a projection off its input type, there is one impl per
input so nothing is ambiguous, doing nothing yields the same shape back (op's "perhaps the very same `T: Add`
they piped in"), and asking for the simplified form is a zero-sized wrapper at the input. Both call sites in
the probe are unannotated, including one with no return-position clue at all, which is precisely where the
defaulted-parameter route failed.

Three alternatives I looked at and did not take, with what closed each.

**A second method.** `dot` and `dot_erased`. Works, costs nothing, and is what most libraries do. Closed
against the projection route only on the ground that the surface doubles per operation and the erasure
question is a policy that belongs in one place rather than in every signature. This is a real candidate and I
would not argue hard against it.

**A turbofishable free function with the output first.** `dot_as::<Simplified, _>(a, b)`. Works and is
honest, and it is the same thing as the second method with the name reused. Closed for the same reason.

**Contextual inference as the mechanism, with no default at all.** `let r: Simplified = a.dot(b);`. This is
what `p2` was accidentally testing. It works wherever the surrounding code pins the type and fails at
`E0283` wherever it does not, which means the ergonomics depend on the call site's context rather than on
the design. Closed because a mechanism that works in some call positions and not others is one a consumer
cannot learn.

**What "simplification and arvo erasure for the return value" would mean, offered as a reading.** The default
projection is the identity: hand back the type that arrived, so tier one sees the type it supplied and arvo
is invisible in the signature. The override asks the projection to return the numeral's lowered form instead,
so the value crosses out of the typestate at the return rather than being carried by the caller. The erasure
gate at `135b` already requires the typestate to erase on lowering; this makes the *point* of erasure a thing
the consumer can name. I am not confident that is what op means and I would want him to say, because the
alternative reading (simplify the *numeral*, normalising a redundant representation, rather than erase the
*wrapper*) is equally consistent with his sentence and is a different mechanism.

## Where the boundary sits, and why the attribute is the envelope

The standing base already settles this and nobody connected it. `124:3579`:

> **arvo grows no build harness of its own.** A build layer reads every axis, acts freely on `Lowering`, acts
> on `Policy` only inside its own declared envelope, and never acts on `Numeral`.

The attribute is a build layer in exactly that sense: it lives outside arvo, it reads arvo's axes, and it
acts on them. So the boundary question has a ratified answer and the answer has teeth.

**Substituting a strategy is a `Policy` act, not a `Lowering` act.** A strategy names a row in both contracts
(`124:786`, `Number<N: Numeral, S: Policy<N::Exponent> + Lowering<N::Exponent>>`), so swapping `Warm` for
`Hot` changes the overflow row, which is observable semantics. Under the sentence above, a build layer may do
that **only inside its own declared envelope**.

**And the attribute annotation is the envelope declaration.** That is the connection worth making. The
consumer writing `#[profile(Hot)]` on an item is declaring the envelope; the envelope's extent is the item's
lexical scope; and the mechanism is licensed by an already-ratified contract rather than by a new permission.
Two consequences follow that would otherwise have to be argued for.

The precedence direction is entailed, not chosen. An explicitly written strategy is a `Policy` fact stated
*outside* any envelope the build layer declared. The build layer may not act on it. So explicit wins, and
op's "IF no specific config is being used" is the ratified contract restated rather than a fresh call.

And the reach limit on aliases is entailed too. A domain alias defined at module scope, outside the
attribute's extent, is outside the envelope. `p6` shows independently that reaching it would break tier two;
the envelope sentence says the same thing from the contract side. Two arguments from different directions
reaching one limit is worth more than either alone.

**What stays on each side.** The types, the four strategies, the two contracts and the projections stay in
arvo, and arvo builds nothing to read an attribute. The attribute, the tier name space, the search path and
the rewrite stay in notko. The interface between them is exactly the strategy-elided spelling: arvo publishes
one alias set per strategy and the attribute selects one. That is a very thin interface, it is the whole of
the coupling, and it needs no cooperation from arvo beyond publishing the sets.

## The alternatives I did not take

Recorded so the next member starts from the list rather than from nothing.

**Making the profile a genuine second axis.** `142c`'s first section proposed cells as functions of a build
profile orthogonal to the strategy, which would multiply the table. Closed by op's own correction, and worth
noting that it would also have required a cell to be keyed on something the trait solver cannot see, which is
the layer-keying rule's dual failure, named at `124:464-465` and refused at `124:2681-2682`.

**A lattice on the strategies with a join at disagreements.** Attractive because it makes nesting order
irrelevant by construction. Closed above: the four are not a lattice under any order op has given, and the
missing join point needs a fifth marker D72 forbids.

**Refusal at disagreement rather than precedence.** Sound, loud, and it makes the attribute unusable over any
body that pins one value, which is most real bodies. Closed on that, but it is the fallback if the report
turns out not to be implementable as a lint.

**Resolving the cell in the macro rather than selecting the strategy.** Closed by the shipped notko's own
shape: a proc macro cannot see the consumer's build condition, which is why `rewrite/hot.rs` emits both arms
under `cfg`. Arvo does not need the two-arm trick because its cells resolve in the trait solver, but only if
the macro stays out of the cell and confines itself to the strategy.

**Deriving the numeral from the trait bound.** Closed at `E0576` by `p8b`, and it is worth keeping on the
list because the phrase "the inference comes in from the arvo public APIs" invites exactly this reading and
it will be re-attempted.

**Putting the erasure default on a trait, a function, or an associated type.** All three closed by compiled
diagnostics above, and the removed feature gate closes the "wait for nightly" route as well.

## What is op's

Six things, and I would put them in this order.

**The reading of his own sentence.** "None of the table items or strategies are the same through profiles"
has two readings and this file takes one of them: that the cells are indexed by the strategy and the panel
was treating an index as a constant. The other reading is that a broader class of cells varies with the build
condition than just `Warm`'s overflow row. The two make different demands on the canon and I cannot tell
which he meant.

**Whether the imitation rule is right.** "A cell is a constant unless the strategy's definition is an
imitation, in which case the cell is whatever the imitated thing does." One sentence, and it either captures
what he has been saying about `Warm` for three checkpoints or it narrows it wrongly.

**Whether the four alias sets are an enumeration he refuses.** They enumerate the four ratified strategies,
not an open set of widths, so I believe the `137b` refusal does not reach them. I am not confident enough to
assume it.

**Tier name against strategy: identify them, or keep them apart.** Turns on whether a project-local tier that
is not one of the four is wanted on the numeric axis, as it evidently was on the fallibility axis.

**Whether the attribute may rewrite a signature**, which is the choice between R3 and R3b, and separately
whether it may reach a bare `u32` at all, which is R2 and which changes debug-build overflow behaviour in
code whose author never mentioned arvo.

**What "simplification and arvo erasure for the return value" means.** Erase the wrapper at the return, or
simplify the numeral. Two mechanisms, one sentence, and the probes settle only that the described spelling
does not exist.

## Probe index

Every claim above that is not a citation rests on one of these. All are committed under `143_probes/`, with
`run.sh` regenerating `output.txt` on `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`).
Each is a spike: it checks one thing, its names and arities are scaffolding, and it should be presumed flawed
outside the one claim it carries.

| Probe | Result | What it establishes |
|---|---|---|
| `p1_fn_type_param_default.rs` | refused | a function type parameter takes no default |
| `p2_trait_type_param_default.rs` | compiles | a trait type parameter takes one syntactically |
| `p3_assoc_type_cannot_be_overridden.rs` | `E0119` | an associated output admits one answer only |
| `p4_default_taken_without_context.rs` | `E0283` | the trait default is inert at a method call |
| `p5_struct_default_is_syntactic.rs` | mixed | written paths elide, inference variables do not |
| `p6_alias_identity_across_scopes.rs` | `E0308` | reaching through an alias breaks tier two |
| `p7_scoped_supply_and_precedence.rs` | compiles | ambient, override, nesting, explicit, alias reach |
| `p8_what_a_bound_determines.rs` | `E0271` | a total signature excludes a refusing strategy |
| `p8b_bound_determines_no_width.rs` | `E0576` | the bound carries no width to read |
| `p9_default_type_parameter_fallback.rs` | `E0557` | the fallback feature was removed in 1.82.0 |
| `p10_output_by_projection.rs` | compiles | the output as a projection off the input |
| `p11_tier_agreement.rs` | mixed | the two tiers land on one row; the control fails |
| `p12_body_scope_does_not_reach_the_signature.rs` | `E0308` | a body selection leaves the signature alone |
| `p13_notko_attribute/lib.rs` | refused | the shipped spelling, live, against the built macro |

Eleven of the fourteen carry a compile failure and that is deliberate: a contract with no expressible form
says more than one returning a wrong value, and four of these say the mechanism as described has no spelling
at all. `p5` and `p11` are the two that carry both colours in one file, and `p11`'s red half is its negative
control, which fails as it must. A tier-agreement test whose control passes would be no test.

The independence bar is met on the two claims that carry weight. That defaults never reach an inference
variable rests on `p1`, `p4`, `p5` and `p9`, four probes with four different mechanisms and one removed
feature gate. That the alias reach limit is forced rests on `p6` from the type side, `p7` from the resolution
side, and `124:3579`'s envelope sentence from the contract side. Everything else here rests on fewer
instances and should be treated accordingly.
