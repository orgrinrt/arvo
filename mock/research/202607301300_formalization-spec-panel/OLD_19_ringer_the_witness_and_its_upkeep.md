# 19: The witness and its upkeep

**Reviewer:** Talia Ringer (proof engineering and repair lens: a verification apparatus is a piece of
software with a maintenance lifecycle, not a certificate to file away; the question that matters is
not "does this check pass today" but "what happens to it the next time the thing it checks moves").

**What I read.** `16b_op_design_the_shape_not_the_code.md`, `16c_op_the_downstream_contract.md`,
`16d_op_the_spirit_outranks_all.md`, `17b_op_checkpoint_six.md`, `13c_op_the_standard_and_the_mode.md`
first, as the brief directs. Then `11_current_shape_draft.md` in full. Then the dive in order:
`13_mcsherry_where_the_laws_belong.md`, `14_dolan_which_algebra_is_this.md`,
`15_willsey_what_a_law_is_for.md`, `16_fallin_laws_as_backend_licences.md`,
`17_orchard_are_these_all_grades.md`, `18_lamport_say_what_is_claimed.md`, and their probe
directories. Then, because my subject is the verification apparatus specifically and its history
predates this sub-dive, `07_spj_is_the_type_story_sound.md`, `09_chlipala_enforcement_and_attack.md`
and `10_leroy_what_is_actually_certified.md` in full, plus `07_probes/a7_door_checks_directly.rs` and
`10_probes/` by file name (not re-run; Leroy already ran them and I had no reason to distrust a member
whose whole dispatch is re-running everyone before him). I `ls`'d the panel directory and confirmed
nothing postdates file 18 or sits outside the numbered sequence.

On source I read almost nothing, per `16b`: `arvo-strategy/src/identity.rs:47-54` to confirm what
ships (nothing named `AddAssoc`, `Magma`, `Monotone`, `ViewC` or `CGrade` exists in `mock/crates/`,
which I reconfirmed myself, see below), and `arvo-strategy/src/cross_strategy.rs:3` for the wording
Orchard and Fallin both cite, re-confirmed directly (`3 | // \`Resolve<S1, S2>::Out\` projects the more
conservative of two`). Everything else on the arvo side is grep, not audit.

**What I compiled and ran**, as distinct from what I reasoned about: two probes at `19_probes/01` and
`19_probes/02`, both `rustc +nightly-2026-05-28`, against the pinned toolchain, with no
`feature(...)` gate opened anywhere except the one arm built specifically to demonstrate a forbidden
one refuses. I also reran `cargo test --workspace` in `arvo/mock` (654 passed, 0 failed, 9 ignored,
reproducing every prior member's figure) and grepped `mock/crates/` for `ViewC`, `CGrade`,
`LIBERTIES`, `liberties_subset`, `Magma`, `AddAssoc`, `Monotone`: zero hits on all seven, confirming
Lamport's finding extends to my own surface (`arvo-graph/tests/waist.rs:46` and
`arvo-sparse/src/csr.rs:325` both still reproduce exactly as Lamport reported: a test function name
and a doc comment using the lowercase word, neither a trait or an impl). The suite is green and my
subject is not in it. Everything else in this file is argument, offered as directions rather than
rulings, and where I hold more than one reading I say so and do not resolve it for whoever comes
after me.

## 0. Where this sits, a premise I checked before trusting it, and what was already in the directory

There is no ratified canon governing this question, the same standing as every file in this dive:
`13c` states the fixed test and `panels-argue-the-intent-not-the-wording.md` is the operative posture.
I am not defending a locked design. I am doing two things op asked for directly at `17b`: build the
witness Orchard's file found missing, and then step back from every individual piece and ask what the
whole apparatus costs to keep true as the design under it keeps moving, which nobody in this dive has
been asked to do yet because nobody before file 17 had a second axis of the same shape to compare
against.

The premise I checked: the brief says the fidelity witness "is not a port of the recovery-map
witness" and that I should assume the first two attempts at any new piece of this apparatus have had
a hole the next member found by compiling. Both held, and the second one held about my *own* first
attempt, inside this file, not about a predecessor's. Section 1 is that failure, kept in rather than
smoothed over, because it is itself the sharpest piece of evidence in this file for section 2's
argument, and section 2a is a second, independent instance of the same pattern, found on a different
probe, by a different construction, at a different layer of the problem.

**Four probes were already sitting in `19_probes/` beyond the ones I built myself, unattributed, dated
in the window before this dispatch and self-consistent with each other in two separate pairs that do
not cite each other.** `01_the_grant_becomes_unwritable.rs` and `02_the_wrapper_is_not_the_worker.rs`
are the two this file's own sections 1 and 2 build directly on and quote from; they are, on the
evidence of their own content, an earlier pass at exactly this dispatch, and I am treating them as my
own prior work rather than a stranger's, since the file you are reading was itself already partly
written and signed with my name when I resumed it. `01_marker_conjunction_first_attempt_has_a_hole.rs`
and `02_the_door_states_the_bound_directly.rs` are a second, independent pair, reaching the identical
destination by a different technical route (named per-liberty marker traits and a `Dominates<Target>`
relation, rather than a `LIBERTIES: u8` bitmask and a bare `assert!`), never cited anywhere in this
file's own sections 1 through 10 before this resumption. I re-ran all four, every cfg arm each names,
from a clean build. All four reproduce exactly, every claimed error, every claimed clean build, every
printed number. Section 2a is where I fold the second pair in, alongside one further finding neither
pair states explicitly, found by two probes of my own (`19_probes/01_liberties_disconnected_from_body.rs`,
`19_probes/02_grants_close_the_hole_by_construction.rs`) that I built before finding either of the
`01_marker_conjunction`/`02_the_door` pair sitting in the directory.

## 1. The witness, built, broken once, and rebuilt against the compiler

Orchard's finding (`17_probes/06_two_lattices_opposite_variance.rs`, `17_orchard...md:285-310`):
`impl ViewC<Relaxed> for Strict {}`, the exact false grant this axis exists to forbid, compiles clean.
`ViewC<G>: CGrade {}` has no associated items, so there is nothing for the compiler to check against,
and the effect side's own missing coercion (`impl LiftE<Total> for Fallible`) fails with `E0004`
precisely because it *would* have to write a function with no case for `Or::Refused`, while the
coeffect side's missing coercion fails with nothing, because a permission carries no data a signature
can refuse to accept.

**My first fix, and the hole in it, found by compiling it.** I tried to make the false grant
unwritable rather than merely checked: one blanket impl, `impl<A, G> ViewC<G> for A where
Assert<{ liberties_subset(G::LIBERTIES, A::LIBERTIES) }>: IsTrue`, so no per-pair impl would exist
anywhere for a human to write a lie on. Compiled:

```
error: generic parameters may not be used in const operations
   --> 01_the_grant_becomes_unwritable.rs:101:35
    |
101 |         Assert<{ liberties_subset(G::LIBERTIES, A::LIBERTIES) }>: IsTrue,
    |                                   ^ cannot perform const operation using `G`
    = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

(`19_probes/01_the_grant_becomes_unwritable.rs`, `blanket_attempt` arm, no feature gate opened
anywhere in the file.) That is the literal forbidden pattern named in `arvo/CLAUDE.md`: "a const
expression computed from a generic parameter, used in type position." A fully generic, closed-form
derivation of a coercion relation as one impl, gated by a const condition over two still-abstract type
parameters, is not expressible under this workspace's feature ban, on this toolchain, full stop. This
is worth stating plainly because it explains something the rest of the dive has been circling without
quite landing on: Dolan's atomic facts (`14_dolan...md:255-296`), Willsey's applicability-condition
layer (`15_willsey...md:195-204`), and my own first instinct all reach for "derive the whole relation
generically, once," and every one of those, the moment it tries to compute the derived fact from two
still-generic type parameters *in a position that selects a type*, hits this exact wall. The workspace
did not choose this wall for this design. It inherited it from banning `specialization` and
`generic_const_exprs`, and the ban is correct (`unstable-features.md`'s forbidden table, and Leroy's
own finding at `10_leroy...md:262-272` that the ban is what makes model-width transfer sound at all),
but a member reaching for "derive it generically" should know in advance that the reach itself needs
checking, not merely the derivation's mathematics.

**My second attempt, still wrong.** It ports Thread C's own `Resolution::WITNESS` shape verbatim
(`07_probes/a7_door_checks_directly.rs:67-79`) onto `ViewC`. This compiles:

```rust
pub trait ViewC<G: CGrade>: CGrade {
    const WITNESS: () = {
        assert!(liberties_subset(G::LIBERTIES, Self::LIBERTIES), "...");
    };
}
```

and it reproduces the exact move that disarmed `a7`'s own `SubstituteZero` (`a7...rs:113-127`, `const
WITNESS: () = ();` overriding the default): `19_probes/01`, `witness_style` build, run output:

```
witness_style: the disarmed grant compiled and IS reachable:
  licensed_view::<Strict, Relaxed>() = Relaxed   <-- WRONG, and it compiled
```

Same disease Orchard found, in a shape that *looks* fixed because it now has a `WITNESS` const with an
`assert!` in it, and is not, because the implementor writing the lie also controls the check for the
lie, in the same impl block. This is not a hypothetical risk. It is the exact move `a7` already made
once, for a different reason (making `SubstituteZero`'s classification not force eagerly), and the
probe shows it works identically as an attack.

**The third attempt is what I would carry forward**, and it is smaller than either of the first two,
because it stops trying to make the trait *declare* something true and instead recomputes the
relation, un-disarmably, at the one place any fidelity-gated body actually branches:

```rust
pub fn licensed_view<A: CGrade, G: CGrade>() -> &'static str {
    const {
        assert!(
            liberties_subset(G::LIBERTIES, A::LIBERTIES),
            "a value under this licence may not be viewed at a grade granting a liberty it does not hold"
        );
    }
    G::NAME
}
```

No per-pair impl, no `ViewC` trait at all, no feature this workspace forbids. `G::LIBERTIES` and
`A::LIBERTIES` are still projected off type parameters that are generic at the point the `const`
block is written, and this is fine, because a `const { ... }` block inside an ordinary function body
is computation, not a const generic *argument used to construct a type*; that is exactly Thread C's
own door (`resolve::<R>` at `a7...rs:187-196`), and the wall from the first attempt never applies to
it. Confirmed directly, separate from the probe file, against a minimal reproduction:

```
error[E0080]: evaluation panicked: grade grants a liberty the operand does not hold
    |     const { assert!(liberties_subset(G::LIBERTIES, A::LIBERTIES), "..."); }
    |     evaluation of `licensed_view::<Strict, Relaxed>::{constant#0}` failed here
```

A hard compile-time refusal, same `E0080` shape Leroy's own preservation door produced
(`10_probes/b_lying_carrier_caught.rs`, cited at `10_leroy...md:198-203`), at every call site,
regardless of whether anyone remembered to write a witness for that particular pair. And it survives a
grown axis with zero new code, which I tested directly (`19_probes/01`, `grown_axis` build): a third
grade, `Wild`, with liberties `{reassoc, contract, arcp, nsz}`, is admitted or refused correctly by the
same formula with no new impl written anywhere:

```
grown_axis: a third grade, zero new code beyond its own CGrade impl:
  licensed_view::<Wild, Strict>()   = Strict
  licensed_view::<Wild, Relaxed>()  = Relaxed
```

**What this design gives up, honestly.** `A: ViewC<G>` as a propagatable bound, with the readable
`error[E0277]: the trait bound Strict: ViewC<Relaxed> is not satisfied` diagnostic Orchard's own
`bad_join` arm produced (`17_orchard...md:262-264`). A caller composing several fidelity-gated
operations cannot state "needs to be viewable at G" in its own signature the way it could with a
marker trait; it gets an `E0080` panic at its own call site instead of an `E0277` at the point the
bound is unsatisfied, one level up. I would trade this, because the E0277-shaped diagnostic was never
sound in the first place (section 1 above is the proof), and a correct E0080 beats a friendlier lie.
But I hold the alternative honestly: a hybrid keeps `ViewC` as a *purely advisory* per-pair marker
(hand-written, giving back the nice bound and the nice diagnostic for the common case) while making
the door's inline assert the load-bearing check regardless of whether `ViewC` is even satisfied, so a
missing or wrong `ViewC` impl degrades the diagnostic but never the soundness. That costs one more
trait declaration and buys back the error message. Section 2a builds the diagnostic half of exactly
this hybrid, verified, though not yet wired to the door.

## 2. The hole in the fix itself, found by trying to attack it a second way

The brief asked me to try to find my own hole before the next member does. Section 1's third attempt
is sound as far as it goes, and "as far as it goes" turned out to be narrower than I first thought.

Orchard's own probe (`17_probes/06...rs:184-198`) splits the mechanism in two functions:
`sum4::<L>(xs)`, which performs the liberty-gated branch and reads `L::LIBERTIES` directly, and
`sum4_in_context::<A, L>(xs)`, which carries the `ViewC` bound and forwards to `sum4`. The entitlement
check sits on the wrapper. I built `19_probes/02_the_wrapper_is_not_the_worker.rs` to ask whether the
worker is reachable without going through the wrapper at all, and it is:

```
straight to the worker, bypassing the wrapper entirely:
  sum4::<Relaxed>(xs)  = 0.0   <-- no entitlement check ran, none was in scope to run
  sum4::<Strict>(xs)   = 2.0   <-- also fine, also unchecked; the worker cannot tell
```

`sum4` is `pub fn sum4<L: CGrade>(...)`. Nothing about its signature says anything about what grade
the caller's data actually is; by the time a function has erased to plain `f64`s there is no
type-level record left of that at all. The entitlement bound was never on the function that performs
the fidelity-gated branch. It was on a second function next to it, and nothing forces a caller, or a
future author of a third combinator, to go through the second function rather than the first.

This is Thread C's original disease, at a different seam. Leroy's own diagnosis of what went wrong
before the fifth pass (`10_leroy...md:47-68`): "the design had two semantics for one construct... the
checked function and the function the runtime pipeline executed were two independently authored
pieces of code that never touched each other." Here the two independently reachable things are not two
authored definitions of the same computation; they are a checked wrapper and an unchecked worker, and
*reachability*, not authorship, is the seam. My section 1 fix closes the grant-side hole completely
for any code that calls through a doored entry point. It does nothing at all about whether a given
combinator is a doored entry point, and Orchard's own probe, written before mine and for a different
purpose, already demonstrates the bypass exists in exactly the shape the design would ship it in.

The fix that follows directly: fold the assert into the function that actually performs the branch,
keyed on the same parameter that decides it, so there is exactly one function reading `LIBERTIES` for
dispatch and it is unconditionally the checked one (`19_probes/02`, `sum4_doored`). Verified against
the same catastrophic-cancellation quartet Orchard's own probe uses:

```
the doored worker: same call shape as sum4, the check travels with it
  sum4_doored::<Strict, Strict>(xs)  = 2.0
  sum4_doored::<Relaxed, Relaxed>(xs) = 0.0
```

`sum4_doored::<Strict, Relaxed>` was left commented rather than run in `main`, its refusal already
confirmed by the same `E0080` reproduction section 1 quotes, since the assert body is identical.

**What this does not fix, and I want to be exact about the shape of what remains open.** Quantisation
has exactly one funnel every arithmetic operation passes through, `pipeline_add` (`10_probes/a_one_
definition.rs`, `10_leroy...md:124-146`), because there is exactly one thing addition can mean.
Fidelity, per Fallin's and Orchard's own framing (`16_fallin...md:212-233`, `17_orchard...md:562-586`),
"gates which of several already-written function bodies compiles," and there can be arbitrarily many
such bodies, each independently authored by whoever writes the next combinator. There is no single
funnel for fidelity the way there is for quantisation, and my fix does not create one; it only shows
what the check has to look like wherever a funnel exists. Whether the design should *mandate* one
canonical fidelity-gated entry point (a `licensed_view`-shaped helper every future combinator must
build its liberty-conditioned dispatch on top of, rather than reading `LIBERTIES` directly), the way
`pipeline_add` is mandatory for addition, is a real design question I am putting in front of the next
member rather than answering, because it changes how combinators are authored workspace-wide and that
is bigger than the witness question I was sent to close. I lean toward yes, because section 4 below
argues the alternative is an obligation that grows forever and is checked by nobody in particular.

## 2a. A sharper form of the disconnection, and a second, independently found route to the same fix

Sections 1 and 2 close two things: a coherence-shaped hole (a false grant that simply had no data to
check against) and a reachability-shaped hole (a checked wrapper beside an unchecked worker). There is
a third, narrower question sitting between them, upstream of the wrapper/worker split, and it is the
one Orchard's own finding (`17_orchard...md:285-310`) stops one step short of naming.

`19_probes/01_liberties_disconnected_from_body.rs` builds Orchard's own proposed fix in isolation,
before folding it into any door: a real, working subset witness relating `LIBERTIES(G)` to
`LIBERTIES(A)`, exactly as file 17 section 4.3 proposes ("the liberty sets are data, inclusion between
them is decidable, and a const check can refuse an impl whose declared direction disagrees with the
sets"). Given that witness, correctly built and passing, the probe then asks a question upstream of
section 2's wrapper/worker split: does anything relate either *declared set* to the *code that runs
under it*. Nothing does. The probe compiles, clean, a function that regroups (the `reassoc` shape)
under a grade whose own `LIBERTIES` array does not list `"reassoc"` at all: the array says `false`,
the body runs the regrouped shape anyway, and the witness in section 1.1 has nothing to say about it,
because it only relates two declared sets to each other. The same probe compiles, equally clean, a
grade declaring `"contract"` with no body anywhere reaching for it. Neither is a wrapper/worker gap in
section 2's sense; there is no wrapper here to bypass, and no bound anywhere is unsatisfied. This is
`09_chlipala`'s exact disease (`10_leroy...md:47-68`, "two semantics for one construct") arriving a
layer earlier than section 2's own instance of it: not two functions that disagree, but a declared
*fact* and an *executed body*, authored independently and checked against nothing but each other.

Section 1's door and section 2's `sum4_doored` both already close the under-claiming half of this,
though neither states that they do: once the relevant fact is recomputed *inline*, from the same
`L::LIBERTIES` the branch itself reads, at the point the branch is chosen, there is no longer a
separately-declared classification for a body shaped like `sum4_doored` to drift away from. The
over-claiming half is not closed by this move and I do not think it can be closed by any mechanism
internal to arvo: it is a promise with nothing behind it until the day something reads it, which per
`16c`'s own downstream-contract obligation is exactly the moment it needs to be true. I would record
it rather than fix it, the same way `catalogue-edge-cases-as-tests.md` asks: a comment or a catalogued
test at each `CGrade` impl naming which combinator, if any, currently exercises each declared liberty,
so the promise's emptiness is visible rather than merely unfalse.

**The second, independently found route to the same repair, and the third time this exact coherence
gap was found by compiling rather than by reasoning about it.** Before finding either
`01_the_grant_becomes_unwritable.rs` or `01_marker_conjunction_first_attempt_has_a_hole.rs` sitting in
the directory, I tried, on my own, to keep a nameable `ViewOf<G>` trait and derive it from one shared
per-liberty declaration via a blanket impl conditioned on an `Implies<Target>` relation with the
refused row simply absent (`19_probes/02_grants_close_the_hole_by_construction.rs`, my own file, kept
with its own trailing note as the audit trail this dive's discipline asks for). It breaks the same way
`01_marker_conjunction_first_attempt_has_a_hole.rs` independently found, by a different construction
(marker-trait conjunctions rather than associated-type equality): `impl ViewOf<Strict> for Relaxed {}`,
hand-authored alongside the conditional blanket, compiles clean, no diagnostic, because rustc's
coherence checker determines the blanket's `where` clause is unsatisfied for that pair and treats the
pair as outside the blanket's coverage, leaving room for a second, wrong impl to fill exactly the gap
the condition excludes. This is not a mistake specific to either probe. It is a general fact about how
Rust resolves overlap for conditionally-implemented blanket traits, found independently three times now
across this dive's history of this one question (mine, `01_marker_conjunction`, and structurally the
same lesson `01_the_grant_becomes_unwritable`'s `blanket_attempt` arm teaches from the adjacent
direction, a fully generic blanket refused outright rather than accepted with a gap). Two independent
constructions finding the identical, surprising fact by compiling an attack, rather than one
construction finding it and a second trusting the report, is real corroboration, in the sense this
review actually credits: not agreement between two pieces of unratified prose, but agreement between
two independently compiled programs and the compiler itself.

The repair that holds, verified (`19_probes/02`, third and final attempt in the file): make the
blanket *unconditional*, computing a total `Ok: TruthMarker` answer for every pair with no `where`
clause narrowing coverage, and move the refusal to the *consuming* bound (`Ok: IsTrue`) rather than to
the relation trait's own impl set.

```rust
pub trait ViewOf<G: CGrade>: CGrade {
    type Ok: TruthMarker;
}
impl<A: CGrade, G: CGrade> ViewOf<G> for A
where
    A::ReassocGrant: ImpliesOutput<G::ReassocGrant>,
{
    type Ok = <A::ReassocGrant as ImpliesOutput<G::ReassocGrant>>::Out;
}
```

`impl ViewOf<Strict> for Relaxed { type Ok = True; }` now fails:

```
error[E0119]: conflicting implementations of trait `ViewOf<Strict>` for type `Relaxed`
   --> 02_grants_close_the_hole_by_construction.rs:140:1
    |
132 | / impl<A: CGrade, G: CGrade> ViewOf<G> for A
    | |____________________________________________________- first implementation here
140 |   impl ViewOf<Strict> for Relaxed {
    |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Relaxed`
```

because the unconditional blanket already answers every substitution and there is no gap left. This
is a second, independently useful destination alongside section 1's door-only shape, not a third
failed attempt on the same road: it keeps a nameable relation trait, at the cost section 1's own
closing paragraph already names honestly (the extra coherence surface a future relaxation could
reopen, exactly the mechanism that broke the second attempt in the first place), and in exchange it
buys back precisely the friendly `E0277`-shaped diagnostic section 1 says it traded away, composing
directly with the `IsTrue`/`Proves<C>` vocabulary Thread C already established
(`07_spj...md:114-134`). `02_the_door_states_the_bound_directly.rs`, the independently-found second
pair's own member, reaches the door-only shape by a cleaner encoding again (named `True`/`False`
markers and a `Dominates<Target>` relation with the refused row simply never written, rather than a
`u8` bitmask), and adds one thing neither my probe nor `01_the_grant_becomes_unwritable.rs` tests:
that *widening the axis itself* by adding the previously-absent row is not an attack and should
succeed, because that is the design's genuine, honest trust boundary, verified with `--cfg
attack_widen_the_axis` compiling clean where an attack would have been refused. I hold this as the
correct amount of closure and would put it in front of whoever owns this next: unattackable from
outside the leaf declaration, editable at the leaf declaration itself, matching `10_leroy`'s own
standard for where trust should bottom out (`10_leroy...md` section 8, bin three).

**What I would put forward as the answer to section 1's own open hybrid proposal.** Section 1's
closing paragraph names a hybrid, ViewC kept as a purely advisory marker while the door's inline
assert stays the load-bearing check regardless of whether the marker is satisfied, and says it is
proposed and not built. `19_probes/02`'s final `ViewOf` is the diagnostic half of exactly that hybrid,
verified: a nameable, unconditionally-derived relation, refused at the consumer, giving back the
`E0277`-shaped error. What I did not do, and flag as the next thing to compile rather than argue, is
wire it to `sum4_doored`'s own inline check so the trait becomes genuinely advisory: a missing or
stale `ViewOf` impl should degrade the diagnostic and never the soundness, since `sum4_doored`'s own
assert fires either way. That wiring, not the diagnostic mechanism itself, is the open item, and I
would want it compiled, per this dive's whole record on this question, before anyone trusts it stated
only as prose.

## 3. What the apparatus is, now, stated as one inventory

Four pieces exist or are proposed, in the order they were built, and I want the inventory written down
once because no file in this dive states it in one place; each file states the piece it is adding
against the pieces the design already had.

1. **The recovery-map witness.** `phi` as a `[const]` trait method generic over `Payload`, instantiated
   once at a small model width (checked exhaustively) and once at the real width (executed), the same
   text (`10_probes/a_one_definition.rs`, `10_leroy...md:124-156`). Three prior passes each had a hole
   the next found by compiling: totality without truth (03/07), a checked classification pointed at a
   private copy of `phi` the runtime pipeline never called (09), and the fix, making the checked and
   executed functions the same text (10).
2. **Bounded exhaustive const checks at a model width.** The mechanism the recovery-map witness and the
   law derivation both run on. Cost quadruples per bit, 28.45 seconds at eight bits, refused at nine
   under `#[deny(long_running_const_eval)]` (`08_fog...md:449,570`, cited throughout the dive).
3. **Structural classification of the recovery map.** Homomorphism, partial identity, retraction, from
   which Kleene-associativity-at-every-arity and existential-associativity-at-every-arity follow by
   proof, checked at a single argument, no width, no search (`18_probes/03`, `18_lamport...md:246-317`).
4. **The fidelity-licence door**, this file, sections 1, 2 and 2a: recompute the coeffect relation at
   every consumption site rather than declare and trust it, closing three distinct, independently
   found holes along the way (a coherence gap in conditional derivation, a checked-wrapper/unchecked-
   worker reachability gap, and a declared-classification/executed-body disconnection).

Piece 3 is the newest and, on the evidence of this section, the outlier in a specific way worth naming
before I get to what each piece costs to keep true: it is the only one of the four whose checked object
is a *fact about the recovery map alone*, independent of arity, accumulator, or fold shape. Pieces 1
and 2 are checks about a *fold* or a *pipeline*, which means they are checks about however many
elements, however wide an accumulator, and however many compositional steps the fold in question
actually has. Piece 4 is a check about a *pair of grades*, independent of how many combinators consume
it, provided section 2's funnel obligation is actually kept. That difference in what the check
quantifies over is the whole content of section 5.

## 4. Where a proof breaks silently, demonstrated rather than asserted

The brief asks where a proof breaks silently rather than loudly. This dive already produced the
sharpest instance of it, and it did not happen in code. It happened in prose, across three files, and
took a fourth to catch.

`11_current_shape_draft.md:699-703` states: "The one shipped `Monotone` law implementation only covers
the 'nearest, with some tie rule' family of rounding rows." McSherry read this and built on it directly
(`13_mcsherry...md:194-196`, "`Monotone` already exists as a partial implementation on the rounding
side"). Dolan read McSherry and built further (`14_dolan...md:419-428`, an open item about whether this
`Monotone` and a differently-shaped one unify). Neither checked the draft's claim against source.
Lamport did, sixth in the sub-dive, eighteenth overall (`18_lamport...md:41-63`): nothing named
`Monotone` exists in `mock/crates/` at all; what exists is an **open**, unlocked proposal in a
design-round topic file, `impl<T: Direction> Monotone for (TowardNegative, T, TowardPositive) {}`. And
it is not merely unshipped. Lamport built `18_probes/03` and checked it against exactly the composition
its own premise admits, and it is false: `ReduceModulo`/`ReduceModulo` gives `phi(-24) = -4 >
phi(-8) = -8`, a monotonicity violation the impl's premise, naming only the three midpoint members of
a five-member quantiser, cannot see because it never mentions the two range members that decide the
answer (`18_lamport...md:434-463`).

I re-ran the grep myself (section 0, and its header). It reproduces: `Monotone` does not exist as any
kind of implementation in `mock/crates/`, and the only two hits in the tree are a test function name
and an unrelated doc comment (`arvo-graph/tests/waist.rs:46`, `fn no_waist_when_width_monotone`;
`arvo-sparse/src/csr.rs:325`, a doc comment on `row_ptr`). The claim was wrong at the moment it was
written, wrong when McSherry cited it, wrong when Dolan built an open question on top of the citation,
and the apparatus that exists to catch exactly this kind of thing (Thread C, the classification move,
every const check in this dive) caught none of it, because **none of it was a claim about
arithmetic**. It was a claim about the state of the design, propagated between members the same way a
`WITNESS` const gets propagated between call sites: by trust, unchecked, until someone happened to
grep.

This is the same disease Thread C spent three passes closing, restated at a different layer. Leroy's
own name for the disease, "two semantics for one construct" (`10_leroy...md:47-68`), applies here
without modification: the draft's sentence about `Monotone` and the actual state of `mock/crates/` are
two things that were supposed to agree and were never checked against each other, and the checking
apparatus this whole dive has been building has nothing in it that watches *itself*, only the
arithmetic underneath it. Two members' worth of dispatch-time built on the gap before it closed. That
is not a criticism of McSherry or Dolan; the panel's own standing instruction is to check a claim
before reasoning from it when it is cheap to check, and this one was cheap (a single grep), and two
careful members in a row did not run it, on a claim that read as background rather than as the
question they were sent to answer. That is exactly the shape a silent break takes: it hides in the
sentence that reads as scenery.

**The general lesson, stated as a rule rather than a story.** Every mechanism in this apparatus checks
something *against* something else. The recovery-map witness checks the executed pipeline against
`phi`. The classification checks a structural property against the recovery map. The door built in
sections 1, 2 and 2a checks a consumption site against a grade's own leaf declaration. None of them
checks a *prose claim about the design* against the *design*, because that was never their job and
could not sensibly be made to be. The only thing that catches this class of drift is a member (or, for
a shipped surface, a lint) re-grepping the specific claim before building on it, and the dive's own
record is that this costs real, non-zero dispatch time even when the discipline is followed correctly
by everyone in the chain, because the check does not run until *someone specifically decides to run
it*, and nothing forces that decision the way `E0046` forces a new `Resolution` constructor to answer
every classification question. **Omission is the hardest failure mode for a check-based apparatus to
catch, because a check that never ran and a check that would have passed look identical from
outside**, and this is true whether the omitted check is a `phi` call nobody wired in (Thread C's
original disease), a fidelity assert nobody put in the worker (section 2), a `LIBERTIES` array nobody
related to a body (section 2a), or a grep nobody ran on a claim that read as background (this section).
The pattern is the same at every layer this dive has looked at, and I do not think it has a mechanical
fix at the level of individual claims; it has, at best, the fix section 6 proposes at the level of
*which claims get funneled through one place at all*.

## 5. How much more robust the classification move is, measured rather than asserted

Lamport's own framing (`18_lamport...md:288-317`) already states the trade: a large mechanical check
becomes a small mechanical check plus a short proof. I want to say *how much* smaller, and where the
robustness runs out, because "more robust" without a boundary is exactly the kind of claim this dive
has spent its whole run refusing to accept unmeasured.

**What survives an axis change for free, and why.** The classification (`18_probes/03`, section 4.1
of `18_lamport...md`) is a fact about `phi` alone, checked at a single argument. It does not mention
arity, accumulator width, or grouping anywhere in its statement, and Lamport verified this is not an
accident of the small models used: the partial-identity check held over all 256 subsets of an
eight-value model crossed with arities 2 through 5, 1024 pairs, every one agreeing
(`18_lamport...md:282-286`). So any axis that changes *which values are representable* (width, radix,
a `Growth` narrowing that only moves the boundary, a new preset built from existing axis values)
changes nothing about whether the classification's *derivation* holds; it only changes which model the
existing check needs to re-run against, which is the same O(1)-per-argument shape it already is,
cheaper than anything else in this apparatus, and not next to the wall.

**What does not survive an axis change, and it is not the one the dive expected.** File 18 section 5
found this directly: for `phi` to be a partial identity *on an operation*, the exact result of that
operation on two representable values must be representable whenever it is in range. That holds for
addition (a sum of two multiples of the quantum is a multiple of the quantum) and fails for
multiplication for the mirror-image reason (a product carries `2F` fractional bits, generically not a
multiple of the quantum). Measured directly, `18_probes/04`: `Precise` addition disagrees on zero of
256 operand pairs; `Precise` multiplication disagrees on 128 of 256, and existential associativity,
which held at every arity for addition, fails outright for multiplication at `n = 3` with a witnessed
counterexample. **The classification is a property of the pair `(phi, Op)`, not of `phi` alone**, and
nothing about how carefully it was checked for addition transfers to multiplication; the transfer has
to be re-run, from the same argument, against the new operation's own recovery-map behaviour, and it
comes out differently.

So the honest statement of robustness has a shape, not a single number: **the classification is
maximally robust along every axis that only changes which values a fixed operation's `phi` sees
(width, radix, arity, accumulator, growth boundary), because its proofs are structural and arity-
generic by construction rather than searched at a sampled arity. It earns zero free transfer across a
new operation**, and this dive's own record (both file 15's and file 18's independent multiplication
probes) already caught that specific failure once, empirically, before the structural argument for
*why* it fails was written down. I would not bet on it transferring across a new *carrier* either
(a `Growth::Narrowed` two-site refusal, or a delivery mechanism that changes what "returns" means),
though nobody has tested that boundary yet and I did not either; I flag it rather than claim it.

**Where the preservation-door check (piece 2 in section 3) sits by comparison.** It is O(n squared) in
the model span (`10_leroy...md:238-249`), the same quadratic shape Fog measured to the wall at eight
bits. Its statement quantifies over the fold's own compositional shape, not over `phi` alone, so every
axis that adds a compositional term to the pipeline (Growth's two-site refusal, an accumulator whose
width differs from the operand numeral's, per Lamport's own finding in section 6 below) adds a term to
the equation the check has to cover, per `10_leroy...md:490-491`'s own unbuilt extension: "the equation
extends mechanically; the probe does not exist." That is not a statement that the piece is wrong. It
is a statement that its cost curve, unlike the classification's, moves with every axis that touches
the operation's own structure, and it is already the piece nearest the wall this dive has repeatedly
hit.

## 6. What the accumulator finding does to the ledger, and the taxonomy it exposes

Lamport's accumulator finding (`18_lamport...md:369-427`) is, read at the level of the apparatus
rather than the level of the arithmetic, the single most concrete demonstration in this dive of "the
model stops being adequate and nothing said so." Signed saturating addition goes from a regrouping
diameter of 7 to full Kleene associativity with **no axis changed at all**, purely by widening the
accumulator the fold's intermediate lives in from scale 1 to scale 3. The draft's own headline
consequence, "only `Hot` folds for signed values" (`11_current_shape_draft.md:334-338`), is a fact
about a `(numeral, accumulator)` pair in which the accumulator was silently taken to be the numeral
itself, and every claim built on that headline (McSherry's over-strictness finding, three subsequent
files' framing of which presets "fold") inherited an unstated quantifier.

Read this next to section 1's finding. **A model can be inadequate in two structurally different
ways, and this apparatus currently only guards against one of them.** Leroy's own guard
(`10_leroy...md:282-288`) is a runtime panic on `Total::refused()` becoming reachable at a width the
model's grade computation said it could not be: that catches the case where the model *undercounts
refusals*, and it fires loudly, by construction, exactly where the transfer argument's weakest leg
lives. It does nothing for the case Lamport's own finding demonstrates: a model too narrow to see a
*value* disagreement, where the fold never refuses at all, just quietly returns a number that
disagrees with what a wider accumulator would have returned. Nothing refuses. Nothing panics. The only
reason anyone knows this happened is that Lamport happened to sweep four accumulator scales instead of
trusting one. **The apparatus has a loud failure mode for "the model missed a case where the answer
should not exist" and no failure mode at all for "the model missed a case where the answer exists and
is wrong,"** and the second is the shape every genuinely dangerous silent-clamping bug in this whole
dive's history (the original Thread C disease included) actually takes. I have not built the fix; the
shape I would want is a mechanical requirement that any model-width check report its own verdict at
more than one width, structurally analogous to how Fog's own cost measurement swept per-bit rather
than trusting one point, and treat a verdict that moves between widths as itself the finding, the way
Lamport's own accumulator sweep did by accident of curiosity rather than by any standing rule.

**The taxonomy this exposes, projected forward to three more axes and two more presets**, held as a
direction and not a headcount. I did not build a fifth axis to test this; it is read off how the four
pieces in section 3 behaved under every change this dive actually made to them.

*Value-widening axes* (radix, a new preset built from existing axis values, a wider numeral): cheap
across the whole apparatus. The classification re-runs at O(1) per argument at the new setting. The
preservation-door check re-runs at the same quadratic shape, just at a different model width, and does
not gain a new compositional term. Two more presets, in this sense, cost close to nothing, because a
preset is a point in an axis space every piece of the apparatus already knows how to check.

*Operation-structure axes* (the accumulator as a real Policy axis, `Growth::Narrowed`'s two-site
refusal, any axis that changes what "one operation" composes into): expensive, and the expense is not
merely "one more dimension of the same search." The preservation equation gains a compositional term
per such axis, each term its own sub-obligation, and per section 5, the classification's own transfer
has never been tested across this kind of axis either, only across width/arity/subset, which are
value-widening in this taxonomy's sense, not operation-structure. Three more axes of this kind is not
three times the cost of one; it is closer to three new preservation equations, each needing its own
proof that the new term composes soundly with the old ones, which is exactly the kind of work
`10_leroy...md:478-491` lists as not done for even the one operation-structure axis
(`Growth::Narrowed`) already on the table.

*Permission axes* (fidelity, and anything shaped like it: a future axis that gates which body compiles
rather than what value returns): cheap to extend per new grade, as section 1 demonstrated directly,
and expensive in a completely different currency, sections 2 and 2a's currency: every new *consumer*
of the axis is a fresh place the wrapper/worker gap or the declared/executed disconnection can reopen,
and that cost scales with how many combinators get written, not with how many axes or presets exist.
Leroy's own vocabulary fits this exactly: it belongs in the "validated per artifact" bin
(`10_leroy...md:384-386`), "a bin expected to grow over time," except that today nothing populates a
ledger for it the way section 7 of `10_leroy...md` proposes for optimised arithmetic arms. If fidelity
gets siblings, the number of places this specific check needs re-verifying grows with the number of
things that read a grade's own axis data, forever, and that is a different, and I think
under-recognised, shape of upkeep cost from either of the other two.

**A second operation is the one none of the three categories above quite name, and it is the most
expensive of all.** Multiplication is not a new axis or a new preset; it is a new *operation*, and
section 5 already showed the classification earns zero free transfer to it. Every measurement in this
entire dive, McSherry's associativity tables, Dolan's distributivity finding, Willsey's and Fallin's
multiplication probes, Lamport's own, is about addition, or about multiplication tested once each and
found to fail differently than addition does. A second real operation is not "one more thing to check
with the existing machinery." It is the first genuine test of whether *any* of the apparatus, the
classification included, generalises past the one operation it was built and repeatedly re-verified
against, and on the evidence gathered so far (Willsey section 3, Fallin sections 6-7, Lamport section
5, three independent probes, none contradicting the other two) the honest prior is that it does not,
cleanly, and needs its own pass through every piece in section 3's inventory rather than a re-run of
the existing checks at a new operand type.

## 7. Overlap between pieces, named rather than left implicit

The brief asks whether any two pieces of the apparatus overlap or contradict. Two real overlaps, no
contradictions I found.

For every resolution the classification covers (`Wrap` as homomorphism, `Precise`'s addition as
partial identity), its Kleene- or existential-associativity is established **twice** if both pieces
stay in the design: once by the classification's structural proof, and once by whatever exhaustive
fold-level check the preservation door or an equivalent mechanism still performs for the same claim.
Lamport's own words leave this open rather than closed: "record the derivation next to the check
rather than instead of it" (`18_lamport...md:317`). That is a real, considered position, and Leroy's
own reading three on the preservation door generally (`10_leroy...md:429-436`, "structure beats
validation where only one is affordable... if the round adopts only one half, the single definition is
the half to keep") is adjacent but not the same question; Leroy was asking whether the door earns its
keep against the single-definition move, not whether the classification earns its keep against the
door. Nobody has asked the second question directly, and I want to be precise about why it matters:
**the const-eval savings the classification move is credited for are not actually realised unless the
exhaustive check it is redundant with is dropped for the cases the classification covers.** Keeping
both is defensible as belt-and-braces (two independent mechanisms both have to be wrong for a lie to
slip through, which is a real property and not a small one given this dive's own history of holes
found by whoever compiled next). But it is a different claim from "the classification is cheaper," and
claiming both benefits at once, cheaper *and* redundant-on-purpose, overstates what either choice
alone delivers. This is a decision the design owes itself before the next axis lands, not after,
because which way it goes changes whether section 6's cost projection for operation-structure axes is
additive or multiplicative against the model-width wall.

The second overlap is inside this very file: sections 1, 2a and `02_the_door_states_the_bound_directly.rs`
built two independently-arrived-at, equally valid closures for the same coherence gap (the door-only
shape and the unconditional-blanket-plus-`IsTrue` shape). They are not a contradiction, they are
alternatives with a real, stated tradeoff (diagnostic quality against coherence surface), and I flag
keeping both around, unreconciled, as a small instance of the same overlap-without-decision pattern
named above: whichever the design carries forward, the choice should be made once and stated, not left
as two working probes with no arbitration between them.

## 8. Is a design that must be re-verified on every move even the right shape

I hold two readings here, genuinely, and the evidence in this file supports both.

**In favour of the apparatus as it stands, cost included.** Every real verification apparatus needs
re-checking when its subject moves; that is not a defect specific to this one, it is what verification
*is*. The alternative this dive replaced was worse in the specific way that matters: prose claiming
`Hot` folds, checked by nothing, silently clamping at runtime while every existing check passed
(`11_current_shape_draft.md:606-617`). The apparatus's re-verification burden is the price of the
guarantee being real rather than asserted, and D47's own standing call ("every rung that goes in is
sketched and benched") already prices this in as the cost of the depth mandate, not as a surprise
bill. Section 4's Monotone finding is not evidence the apparatus failed; it is evidence the apparatus,
applied honestly by a sixth member checking a claim nobody before him had checked, is exactly what
catches this class of drift, eventually, at the cost of however many dispatches pass before someone
runs the grep.

**Against it, in the specific place the evidence points sharpest.** The form some of these checks take
was fixed before the classification insight existed; Thread C's fourth pass (the disconnection
finding) predates file 18's classification move by most of a dive. Continuing to pay the exhaustive,
quadratic, wall-adjacent form for a claim that the classification could establish at O(1) per argument
is not rigor, it is habit, once the cheaper form is known to exist and to cover the same ground for the
cases it reaches (section 5's boundary). Nobody has gone back through the apparatus asking, piece by
piece, "does this specific exhaustive check now have a classification-shaped replacement," only found
individual instances of it as a side effect of a different question (Lamport found it for
associativity; nobody has asked it for the grade check or for `AddClosed`). That sweep is real,
currently unbudgeted work, and until it happens the apparatus is carrying cost it does not need to be
carrying, on top of the cost it does need to carry. Section 2a's own history sharpens this further: I
found, myself, inside one dispatch, that the "obvious" fix for a coherence-shaped hole breaks the
identical way three separate times before landing (mine, twice, and independently
`01_marker_conjunction_first_attempt_has_a_hole.rs` once more), which is direct, first-hand evidence
that this style of design is not merely re-verified when it moves, it is *expensive to get right even
once*, by a member who already knew, going in, exactly which failure shape to expect.

I do not resolve this. What I would put in front of whoever decides it: the two readings are not about
whether to have an apparatus, they agree completely on that. They are about whether *this specific
mix of check-shapes* is the mix the design would choose if it were designed fresh today, knowing what
section 5 now knows, rather than the mix that accreted one panel-finding at a time. Section 9 is a
concrete, cheap first step either reading can act on without waiting for the other to be settled.

## 9. What I would actually do, cheap first, held as proposals rather than rulings

**Adopt a standing rule for which shape a new obligation gets, before the next axis, not after.**
Lamport's own table (`18_lamport...md:584-591`) already sorts obligations by the form of their
quantifier. What is missing is not a new row in that table; it is a decision procedure that runs
*before* a member reaches for an exhaustive fold-level check: is the fact a property of the recovery
map alone, checkable by classification (cheap, arity-independent, survives value-widening axes for
free), or does it genuinely need the fold's own compositional shape (expensive, grows with every
operation-structure axis, sits next to the const-eval wall)? Section 6's taxonomy is a first cut at the
axis-side half of that decision; Lamport's own row-classification is the claim-side half. Neither
alone decides it; put together they would have caught the accumulator gap before it needed a sixth
member to surface it, because "does the claim's statement mention the accumulator" would have been
asked at the point `AddAssoc`'s key was first drawn, not four files later.

**Require a funnel for any permission-shaped axis, the way `pipeline_add` is required for arithmetic.**
Section 2's finding is not a bug in one probe; it is a structural gap that reopens for every new
combinator a permission axis gets, forever, unless the design states, as a rule and not a convention,
that a liberty-conditioned branch may only live inside a function that also carries its own liberty
check, keyed on the same parameter, the way `sum4_doored` does and `sum4`/`sum4_in_context` does not.
I would treat this the same way `arvo-always-optimal-internals.md` treats optimised arms: an
obligation named once, discharged per artifact, tracked in a ledger, per Leroy's own "validated per
artifact" bin, because I do not think this one closes any other way given what fidelity actually is (a
fact that gates source selection rather than a value anything can be checked against). Section 2a
sharpens what the funnel actually has to guarantee: not merely reachability (does every path to a
liberty-gated branch pass through a check), but identity (does the check read the *same* projection of
the *same* type parameter the branch itself reads), because a funnel that checks one declared array
while the branch reads a different one has the funnel's shape without its content.

**Build the compile-fail test Lamport already named twice and nobody has built.** `18_lamport...md:
734-736`: "the two feature bans now carry a load nobody has pinned... it does not exist, it is small,
and the argument that most depends on it is the one the draft calls its irreducible core." Leroy named
the same gap at `10_leroy...md:270-272`. Two members, in two different files, have now said the same
cheap, load-bearing thing is missing. I am naming it a third time not because a third naming adds
anything the first two did not already establish, but because a rule that gets stated three times
without landing is itself a data point for section 4's argument: prose claims drift even when they are
correct and even when more than one careful member has already said so, and the fix for *that* is
never another sentence, it is the test. This is the cheapest item in this whole file and the one with
the largest ratio of load-bearing-ness to cost; I would want it before the next axis, not after.

**Ask, once, per existing exhaustive check, whether a classification replaces it.** Not a re-derivation
of every check from scratch; a pass, per obligation currently in Leroy's "machine-checked by bounded
exhaustion" bin (`10_leroy...md:354-360`), asking Lamport's own question from section 9 of his file:
does this property's statement mention a value in a way that forces a search, or can it be read off the
recovery map's structure at a single argument the way associativity now can. Where the answer is the
second, replace it, and record the redundancy decision from section 7 explicitly rather than leaving it
implicit in whichever form happened to get built first.

## 10. What I would flag for the next member, unresolved

**The hybrid `ViewC`-as-advisory-marker-plus-door shape, section 1's closing paragraph, is now half
built.** Section 2a's `ViewOf` (`19_probes/02_grants_close_the_hole_by_construction.rs`) is the
diagnostic half, verified: an unconditional blanket computing a total truth marker, refused at the
consumer, buying back the `E0277`-shaped error section 1 traded away. What is not built is wiring it
to `sum4_doored`'s own inline check so the trait becomes advisory (a missing or stale `ViewOf` impl
degrading the message, never the soundness, since `sum4_doored`'s own assert would still fire either
way). That wiring, not the diagnostic mechanism itself, is what I would want compiled before trusting
the hybrid, by whoever picks it up.

**Whether the design should mandate one canonical fidelity-gated entry point is a real open call,
section 2's close.** I lean toward yes and did not build the general version, only the two-argument
toy `sum4_doored`. Whoever designs arvo's actual arithmetic-side fidelity dispatch should decide this
before authoring the first real combinator, not after several exist and each has to be independently
audited for the section 2 and 2a gaps.

**The over-claiming half of section 2a's disconnection (a grade promising a liberty no body exercises)
is named and not fixed, on the grounds that I do not think it can be fixed internally.** I proposed a
catalogued-test recording obligation rather than a mechanism; whoever owns the fidelity axis next
should decide whether that recording obligation is worth the ceremony before a second liberty axis
exists to make the promise-tracking genuinely necessary, the same judgement section 9's `P(Cause)`-
shaped deferral question makes for a different axis (per file 17's own open item, `17_orchard...md:
214-222`).

**Section 6's model-inadequacy taxonomy (undercounted refusals versus wrong values) is argued from one
demonstrated instance (Lamport's accumulator finding) and one existing mechanism (Leroy's
`Total::refused()` panic).** I did not build a second instance to confirm the pattern generalises past
the accumulator specifically, and I did not build the sweep-and-compare mechanism I proposed as a fix.
Both are cheap next probes.

**Section 7's overlap question (does the classification replace the fold-level check it duplicates, or
run alongside it on purpose) is a design decision I put in front of whoever owns the apparatus next,
not something I resolved.** Both readings are defensible; I only established that claiming both
benefits (cheaper, and redundant-on-purpose) at once is not available. The same is true of section 7's
second overlap (the door-only shape versus the unconditional-blanket shape for the fidelity relation
itself): both work, both are verified, and nobody has arbitrated between them.

**I did not read `arvo-num-systems` or `notko-hlist`**, both flagged by two prior members
(`17_orchard...md:622-624`, `18_lamport...md:729-732`) as possibly changing the cost picture for
`P(Cause)` and for any future type-level-set-shaped axis. Whatever this file's cost taxonomy says about
permission axes should be re-checked once someone has actually read those two crates; I have not.
