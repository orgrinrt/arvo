# Panel 03: what is actually provable

**Persona:** Ranjit Jhala, provability lens. Third member; read `01_knuth_mathematical_rigour.md` and
`02_kiselyov_type_level_encoding.md` in full before starting.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), panel files 01 and
02, the panel brief and the governing panel rule (`panels-argue-the-intent-not-the-wording.md`), and
all fourteen probes under `02_probes/` (I read every file; the ones I engage with by name below are
`c_computed.rs`, `c2_totality.rs`, `f2_refusal.rs`, `b_coherence.rs`, `b_minspec.rs`). **What I read in
part:** the talk, at the passages 01 and 02 cite plus the D16 passages themselves (`talk:280-320`,
`talk:990-1020`), which I went back to in source rather than trusting either summary; the inherited-
state file, not reopened beyond what 01 and 02 already located; `arvo-strategy/src/identity.rs:69-92`,
which both prior members cite as the shipped precedent and which I wanted to read myself before
proposing anything that claims to extend it.

**Directory listing.** `ls` across `mock/design_rounds/` (94 entries, all but the three flat files at
root are closed rounds, nothing postdates this round), `mock/research/` (nothing newer than the panel
directory itself), `mock/research/sketches/` (twenty entries; the two the spec cites plus
`202607291400_const-args-under-min-gca`, which 02 already pulled in and which I also read). Nothing
supersedes what the brief handed over. I did not find a false premise in the brief itself; 02's finding
that the two live `#![feature(generic_const_exprs)]` gates are drift against the workspace's own
current ruling is not a false premise, it is the same finding I would have made independently, and I
verified it holds: `.claude/rules/unstable-features.md`'s forbidden table names
`arvo-strategy/src/lib.rs:11` and `arvo/src/lib.rs` by path as the drift to remediate, and both still
carry the gate today (git status clean, nothing has moved since 02 wrote that).

**Gates.** I rely on 01's and 02's independently-run suite reports (654 passed, 0 failed, both re-run
rather than one inheriting the other's number) rather than running a third redundant pass. Working tree
is clean and matches what both of them reviewed; nothing has changed. I did read `identity.rs` myself
because it is the one piece of shipped machinery every subsequent claim in this file leans on, and I
wanted it verified in my own hand rather than through a citation.

Sections below are marked **verified** (I read it, hand-checked it, or it follows from a probe already
compiled by 02) or **reasoned** (argument, offered for the panel to weigh). I carry more than one
reading wherever the evidence does not force a single one, and I rule on nothing.

---

## 0. My lens, stated against 01 and 02 rather than beside them

01 asked whether the spec's mathematics is true. It found five places where it is not, or is not
well-formed. 02 asked whether the type-level construction is honest about what it encodes. It found
that the three-contract split is claimed and not typed (section 5), that the derivation mechanism has
a coherence ceiling (section 3), and that "computed cannot lie" is not quite right even on its own
terms (section 4, its own words: "every derived property bottoms out in some asserted primitive").

That last sentence of 02's is where my lens starts, and I think it undersells its own finding. The
question I am here to answer is not "is the mathematics true" (01) or "does the encoding say what it
claims to say" (02). It is: **once a property is stated correctly and encoded honestly, what in this
design actually checks that the stated leaf facts are true, as opposed to merely checking that a leaf
fact was stated for every case?** Those are different guarantees, the spec's own organizing principle
(D16) conflates them, and the gap between them is where a wrong lemma can still ship even after both
01's and 02's fixes land.

## 1. D16's dichotomy is not a dichotomy. Reasoned, building directly on 02 section 4.

D16, quoted at spec:190-193 and stated in the talk at `talk:1002-1007` in almost these words: "a
derived property is a safe impl; an asserted one is an `unsafe impl` carrying a contract." The spec's
self-description of its own contribution is "everything below is in the first category" (spec:189).

Read literally, "derived" means computed from something already established, the way `2 + 2 = 4`
follows from Peano's axioms. Nothing in this design derives anything in that sense, and nothing
_could_, because the properties in question (associativity of a recovered addition, membership in a
number system, absence of an escape channel) are facts about infinite or unbounded domains that no
finite type-level computation touches directly. What the design actually does, in every instance 01
and 02 examined, is: pick a finite index (a pair of markers, a signedness, a resolution), and attach to
each point of that finite index a **hand-proved or hand-asserted** fact, then use the type system to
guarantee the attachment is (a) present for every point (totality) and (b) not contradicted by a second,
different attachment to the same point (coherence). That is real and valuable machinery. It is not
derivation in the sense the word claims, and calling it that is what let 01's `SubstituteZero`
counterexample and 02's downstream-marker hole both ship as "safe."

Concretely, in 02's `c_computed.rs:73-88`, the leaf fact `impl Resolution for SubstituteZero {
StableOneSided = False, StableTwoSided = False }` is exactly as asserted, in exactly D16's second
sense, as the spec's current blanket `impl AddAssoc for ((A, B), Unsigned) {}` is. The difference
between them is not derived-versus-asserted. It is **total-versus-partial**: `c_computed.rs` forces
every `Resolution` implementor to answer, the blanket does not. A human still typed `True` or `False`
by hand at each leaf, in both encodings, and nothing checks that the human typed the right one. If
whoever writes `impl Resolution for SubstituteZero` in the real crate had typed `StableOneSided = True`
by mistake (the literal mirror image of 01's finding, an error of the opposite sign), `c_computed.rs`'s
own encoding compiles it, ships it, and is just as wrong as the spec is today. I did not need to
recompile the probe to see this: it is a fact about what the trait signature checks (totality) versus
what it cannot check (truth of the RHS), visible from the shape of the `impl` block itself.

So D16 is right about something real, and the something real is not "computed cannot lie." It is
**"a property whose obligation the type system forces to be discharged, once, at the point of
definition, for every constructor, cannot be silently omitted."** That is a totality guarantee. It is
worth having. 01's finding 1 and 02's finding in section 1 (the downstream-marker hole) are both
*omission* bugs, and totality is exactly the right cure for an omission bug. Neither of them is a
*falsity* bug caught by this machinery, because nothing here catches a falsity bug. I want the panel to
carry both readings of what D16 bought, because the spec's own prose oversells the second:

- **Reading A (what D16 claims):** derived properties cannot lie.
- **Reading B (what the machinery, correctly used, actually delivers):** derived properties cannot be
  silently omitted, and two contradictory claims about the same point cannot both stand. Whether the
  one claim that does stand is *true* is exactly as much a human responsibility as it was in the
  hand-written table the spec set out to replace.

## 2. Coherence is a no-contradiction check, not a no-falsehood check. Verified, against 02 section 3.

02's section 3 shows the coherence wall (`b_coherence.rs`, `b_minspec.rs`): three individually
plausible impls, no two of which the compiler can order, produce `E0119`. I want to name what that
error is actually protecting, because it clarifies what it is not.

`E0119` fires when two *different* impls both apply to the same concrete instantiation. It is Rust's
mechanism for "at most one answer is permitted to exist for this case," and in a design where impls
encode facts, that machinery incidentally catches the case where two people (or one person twice)
assert **opposite** things about the same composition. That is a real, useful check: it is a
poor-man's contradiction detector, free from a mechanism that was built for a different purpose
(dispatch ambiguity, not fact-checking).

It is not a truth check. A single false blanket impl, with no competing claim anywhere in the crate,
produces no coherence error at all, because coherence only fires on *conflict*, never on *isolated
falsehood*. That is exactly 01's finding 1: the spec ships with one impl for the unsigned case, it is
wrong, and nothing in the compiler's coherence pass has any way to notice, because there is nothing for
it to conflict with. So the two mechanisms this design leans on, totality (E0046, when a member is
missing) and coherence (E0119, when two members disagree), together buy "every case is covered by
exactly one stated fact." Neither, separately or together, buys "the one fact stated is correct." I
think this is worth stating plainly because the spec's rhetoric ("cannot lie") reads as though the
compiler is doing semantic work it structurally cannot do here, and a reader who takes that literally
will stop looking for the place a wrong lemma can still hide.

## 3. What would actually check a leaf fact: a decidable, bounded fragment, checked by brute force, not by a solver. Reasoned, offered as a genuinely new mechanism.

The brief asks what a verification-minded reading would do differently, given that arvo will not run a
compile-time solver. I want to name the actual shape of the obligation, because I think the "no
solver" constraint is being read as "no checking is possible," and that reading is too strong.

Every leaf fact in question, `AddAssoc` for a `(Resolution, Resolution, Signedness)` triple, dyadic
membership for an `Adjustment`, `FullRange<F>`'s boundary behaviour, is, **once the width is fixed**, a
statement in a decidable theory: bounded integer arithmetic with `min`, `max`, and modular reduction,
quantified over a finite representable set. This is Presburger-shaped and any SMT solver discharges it
for a fixed width in milliseconds. The reason no solver is in play is workspace policy stated in the
brief, not a fact about the mathematics: arvo does not want the dependency, not that the obligation is
undecidable.

Given that, the honest cheap substitute for a solver, and one that costs no new dependency and no new
unstable feature, is **bounded, exhaustive, compile-time enumeration**, expressed as a `const` block
that panics on the first counterexample. Sketched, not verified (I did not compile this, and I say so
plainly):

```rust
// at the definition site of each Resolution constructor, alongside the
// asserted StableOneSided / StableTwoSided members, a checked witness:
const _: () = {
    // phi is the recovery map this Resolution encodes, written once as an
    // oracle and reviewed like any other piece of trusted code.
    const fn phi_substitute_zero(exact: i32, max: i32) -> i32 {
        if exact > max { 0 } else { exact }
    }
    // brute force over a small representative width. cheap: a few hundred
    // iterations, well inside the const-eval step budget.
    let max: i32 = 7; // 3-bit unsigned representable set, [0, 7]
    let mut x = 0;
    while x <= 2 * max {
        // one-sided translation stability: phi(phi(x) + c) == phi(x + c)
        // for c in the representable set.
        let mut c = 0;
        while c <= max {
            let lhs = phi_substitute_zero(phi_substitute_zero(x, max) + c, max);
            let rhs = phi_substitute_zero(x + c, max);
            assert!(lhs == rhs, "SubstituteZero is not translation-stable");
            c += 1;
        }
        x += 1;
    }
};
```

This is not a proof for all widths, and I want to be explicit that it is not, because overclaiming
bounded checking is the same mistake as D16 overclaiming derivation. What it is: a **falsification
test that runs automatically, every build, for a decidable finite instance of the property**, and it
would have caught 01's finding 1 mechanically, at the point `SubstituteZero`'s lemma was written, with
no hand arithmetic required. The generalisation from "holds at width 3" to "holds at every width arvo
supports" is a separate step and needs the same kind of argument 01 already gave by hand: these five
resolutions are positional rules whose qualitative behaviour does not depend on the width, so a
width-uniformity argument (stated once, in prose, next to the checked constant) closes the gap the
brute force alone cannot. That argument is exactly what 01's Kulisch reframing (01 section 14) supplies
for free: idempotence, monotonicity and sign symmetry are properties of the *rule*, not the *width*, so
once the panel adopts that frame, the const check at one or two small widths plus the general argument
together are strong evidence, where the current spec has neither.

I read this as directly answering the brief's question about runtime-checked construction versus a
proven-thereafter type, but sideways from how the brief poses it. The properties here are static
(type-indexed, not value-indexed), so there is no "construction site" at runtime the way there would
be for, say, an array bound. The value-level analogue of a runtime-checked, proven-thereafter
construction already exists correctly in this design, at `02_probes/f2_refusal.rs`, where the runtime
`checked_add` branch is exactly the moment a refusal is constructed, and the type of that branch
(`Q::Fallibility<u32>`, bounded on `ConstFromResidual`) does not let a total quantisation reach it. That
part of the design is sound as typed; I checked it against 02's probe and it compiles as reported. The
static leaf facts have no such runtime moment to hang a witness on, which is exactly why a bounded
compile-time check, not a runtime one, is the shape that fits them.

**A second reading, weighed against the first.** The cost of writing and maintaining a brute-force
oracle (`phi_substitute_zero` above) per resolution is real, and the oracle itself is a second place a
human can get it wrong, symmetrically to the leaf assertion it is meant to check. This does not defeat
the proposal (an independent oracle catching a mismatch with an independent leaf-assertion is still
strictly more evidence than either alone, and the two are unlikely to be wrong in the same way, which
is the whole point of a cross-check), but it means "checked" here means "cross-checked twice by
different authors or different sittings," not "proved once." I think that is an honest, useful, and
underused standard, and it is cheaper than it sounds because the oracle for each resolution is two or
three lines.

## 4. `Faithful`, and the frontier this design cannot move. Reasoned, sharpening 01 finding 3.

01 shows `Faithful` as literally defined is vacuous, and proposes `TranslationStable` as a replacement,
verified case by case (01 section 3's table). I agree the replacement is better mathematics. I want to
name what neither replacement, nor any replacement, can do: certify that the **chosen** definition is
the one that captures what the design actually needs `AddAssoc` to mean for the rest of the system to
be sound. That is not a gap in this spec specifically. It is the standing limit of every refinement-
style discipline, including the one I have spent a career on: the specification itself, the predicate a
type is refined by, is trusted, not verified. What gets verified is that the implementation discharges
the specification, not that the specification is the right one to want. LiquidHaskell cannot tell you
that `{v : Int | v >= 0}` is the right refinement for a list length; it can only tell you, once you
wrote it, whether every site that claims a length actually produces a nonnegative one.

So the honest framing for the panel is a frontier, not a single fix:

1. **The predicate itself (what `Faithful` or `TranslationStable` means) is a spec.** It is authored by
   a human, reviewed by humans (01's table is exactly that review, done well), and no mechanism in this
   design or any design can certify it captures the intended real-world property rather than merely
   being internally consistent. This is not a defect to fix; it is the boundary of what formal machinery
   does. Say it explicitly in the round, because the spec's current rhetoric implies the machinery goes
   further than this.
2. **Whether a given leaf fact satisfies the chosen predicate, once fixed, is where machinery helps.**
   This is where totality (section 1), coherence (section 2), and bounded checking (section 3) all
   apply, and it is the layer the current spec conflates with layer 1.
3. **Whether the predicate, once discharged everywhere it is claimed, is *used* soundly downstream**
   (the composition-keyed law flows into the right places, nothing bypasses it) is a third, separate
   question, and it is the one 02's section 7 answers for `Fallibility`: even a correctly derived
   property can leave a hole if nothing forces every call site to go through it. I have nothing to add
   to 02's finding there beyond flagging that it is the same three-layer distinction, at the
   "consequence" layer rather than the "definition" layer.

## 5. A cross-crate soundness dependency the panel has not named. Verified, against 02's own probe.

02 section 7 shows the fallibility repair (`f2_refusal.rs`) works because `Just<u32>` does not
implement `ConstFromResidual<Outcome<Infallible, OutOfRange>>`, and the error 02 quotes
(`c_refusal.rs`'s sibling, `f2_refusal.rs:38-40`, the commented-out `add_or_refuse::<Total>` line) is
exactly that absence firing. I want to flag what kind of fact that absence is: it is a fact about
`notko`'s trait impls (`notko::Just`, `notko::ConstFromResidual`), not about `arvo` at all. arvo's
argument that "a total quantisation cannot construct a refusal" is, structurally, the same
absence-of-impl-as-proof pattern 02's section 4 already names as coherence-limited, except here the
absence lives in a **crate arvo does not own**, and nothing in this round or the sketches pins it.

If notko ever adds a blanket `impl<T, E> ConstFromResidual<Outcome<Infallible, E>> for Just<T>` for
some reason internal to notko's own fallibility ladder (a plausible-sounding ergonomic convenience:
"any infallible residual coerces into any Just"), arvo's soundness argument here silently stops
holding, and nothing anywhere would notice, because the property was never arvo's to assert in the
first place. This is exactly the shape `a-test-that-cannot-compile-is-the-finding.md` and
`catalogue-edge-cases-as-tests.md` exist for: pin the absence as a compile-fail fixture in arvo's own
test suite (`assert_not_impl_any!`-shaped, or a `tests/ui/*.rs` fixture whose `.stderr` names the
missing bound), so that if notko ever changes, arvo's CI reports the break at the actual point of
failure instead of shipping a quiet unsoundness. I did not find this pinned anywhere in `arvo/tests/ui/`
(02 already read that whole directory, section labelled "gates", and did not report one either); I read
it as an omission worth naming rather than a claim that one exists and I missed it.

## 6. D67's "falsifiable test" checks the wrong thing, structurally, for the same reason as section 2 above. Reasoned, extending 01 finding 4 and finding 8.

D67 (spec, "Conventions") states its own success criterion: "if a convention's mode cannot be written
as an alias, the abstraction is not general enough," and calls this falsifiable. 01 already shows two
concrete places this test passes while the semantics are wrong (finding 4, the range boundary; finding
8, IEEE's hidden bit and specials). I want to name the structural reason those slipped through, because
it is the same reason section 2's coherence gap slipped through, and it generalises.

D67's test, as stated, is a **type-checking** test: does the vocabulary have a slot the alias can fill.
That is necessary and it is genuinely useful (spec itself reports it found the MATLAB bias gap this
way). It is not a **semantic-fidelity** test: does the alias, once it type-checks, compute the same
numbers the vendor's format computes. Those are different properties, in exactly the sense section 1
distinguishes "a fact was stated" from "the fact stated is true." A convention alias that type-checks
against the ten-axis vocabulary has satisfied D67 completely and can still, as 01 shows twice, disagree
with the vendor on the boundary cases that matter most.

The same bounded-checking mechanism from section 3 applies here directly, and I think it generalises
better here than anywhere else in the spec, because vendor formats come with published, small,
canonical test vectors already (IEEE 754's own conformance suites, SystemC's regression tests). A
`conv-ieee754` feature claiming to alias binary32 rounding is a claim that is trivially checkable
against a handful of literal boundary values (last midpoint before MAX, first over-range midpoint, a
tie at MAX, one subnormal, one specials case) with no oracle-authoring burden at all, because the
correct answers are already published, not derived by arvo. I would treat this as the strongest
candidate in the whole spec for cheap bounded verification, stronger than section 3's algebraic case,
because the reference values do not need to be written, only looked up. 01's finding 4 proposes
restating D67's test as behavioural in exactly these terms ("the alias reproduces the vendor's result
on the boundary cases"); I want to add that this restatement is not just better prose, it names a
concretely different and mechanically checkable obligation, and the panel should treat it as a design
change to D67's test, not a wording note.

## 7. `Deterministic` and `ConstantTime`: the frontier from section 4 landing on a property with no type-level referent at all. Verified against arvo's own governing rule.

02 section 11 already separates these from the algebraic family and I agree with the separation and
will not re-argue it. I want to sharpen the provability question specifically, because it is different
in kind from every property discussed above, not just weaker.

`AddAssoc`, `TranslationStable`, dyadic membership are all facts about a **mathematical structure**
that the type parameters name. `ConstantTime` is a claim about the **generated machine code** for a
specific target, and arvo's own governing rule, `arvo-always-optimal-internals.md`, states as policy
that the generated code is explicitly and deliberately unconstrained ("internals unwrap to whatever is
most optimal for the specific build target... a hand-rolled asm microkernel when benchmarks show a win"
`.claude/rules/arvo-always-optimal-internals.md:9-20`). There is no type in this design, and there
could not be one without contradicting that rule, whose value determines the emitted instruction
sequence. So `ConstantTime` is not merely an asserted property in D16's second sense (a human-typed
leaf fact the type system enforces totality on); it is a claim about a fact that **lives entirely
outside the type system's field of view**, in LLVM's codegen and the target's microarchitecture, both
of which are one bench run away from changing without any source edit at all.

The mechanism that could actually check this is not a `const` block (section 3's technique needs the
property to be computable from the type-level data available at compile time, and codegen timing is
not). It is a statistical timing-variance test against the compiled artifact, the shape `dudect` and
similar constant-time auditing tools use: compile the lowering, run it against paired secret-dependent
and secret-independent inputs, and test whether the timing distributions are distinguishable. That is
exactly a bench, in exactly the sense `bench-and-sketch-discipline.md` already governs for this
workspace, and I think `ConstantTime`, if it survives the round at all, belongs there: a
`mock/benches/` artifact per lowering that claims it, re-run on toolchain or target changes, not a
type-level marker asserted once and trusted forever. 02's proposed repair (an `unsafe impl`-shaped
promise with a contract) is the right type-level shell for this; my addition is naming what actually
discharges the contract, since nothing at the type level can.

## 8. What I verified and found solid, so the panel does not re-litigate it.

The totality mechanism in `c2_totality.rs` genuinely works as claimed: adding `StochasticRound` without
its two associated types is `E0046` at the constructor's own definition, which I read directly rather
than trusting the summary, and it is the right place for an obligation to surface. `identity.rs:69-92`
is a real, working precedent for the sealed-witness-at-a-tag pattern both prior members cite, and its
documented reasoning ("that absence is how `Identity<Multiplicative>` stops existing for a purely
fractional type") is honest about being an absence-as-proof pattern, which is exactly the pattern
section 2 above says needs a coherence-not-truth caveat; the comment does not currently carry that
caveat and could. `f2_refusal.rs`'s runtime construction site is sound as typed, and I agree with 02
that it is the correct shape for the one place in this design where a genuine runtime-checked,
type-carrying-evidence construction belongs.

## 9. Where I would put the panel's remaining budget, offered as two directions rather than one.

**Direction one: write the bounded checks, not the theory.** Section 3 and section 6 both propose the
same mechanism (bounded, published-or-hand-written oracle, checked at a small representative instance,
generalised by a stated width-uniformity argument) applied to two different obligations (the algebraic
laws, the convention aliases). Writing three or four of these as sketches, before any of `arvo-policy`
exists, would tell the panel whether the const-eval budget and the trait-bounded generic-`const fn`
shape 02's probes already demonstrate for other purposes actually compose with this one. I have not
verified that composition and I say so.

**Direction two: separate the spec's rhetoric from its machinery before either changelist is written.**
Independent of any code, the phrase "cannot lie" (spec, throughout) should not survive this round as
written, for the same reason 01 wants "faithful" not to survive as written: both oversell what the
mechanism delivers. Replacing it with something like "every case must be answered, and no two answers
may disagree" is less satisfying to write and is the sentence that is actually true, and the panel
brief itself asks for the shape that survives contact with what the type system does, not the shape
that reads best in a design round.

I do not think these directions are exclusive, and I do not rank them; the next member should read this
section as two open threads rather than a recommendation.

---

**Summary for the next member.** D16's "derived, cannot lie" is real machinery doing a real job
(totality plus coherence), and it is not the job it claims: it forces every case to be answered and
forbids two contradictory answers to the same case, and neither of those is a check that the answer
given is true (section 1, verified against `c_computed.rs`; section 2, verified against `E0119`'s
actual firing condition). Where a leaf fact is a statement in a bounded, decidable arithmetic theory,
which every algebraic law and every convention-alias claim in this spec is once the width is fixed, a
compile-time brute-force check against a small hand-written or vendor-published oracle is a cheap,
mechanical, genuinely new (relative to sections 01 and 02) way to raise confidence in the leaf without
adopting a solver, and I have sketched but not compiled the shape (section 3). Two places already
depend on absence-of-impl as a soundness argument the way section 2 says is coherence-limited, not
truth-checked: one inside this spec (section 2, restated), one crossing into notko and currently unpinned
anywhere (section 5, a genuinely new finding). `Deterministic` and `ConstantTime` are not in the same
family as the algebraic properties at all; they are claims about generated code that only a runtime
statistical test can check, and belong in `mock/benches/`, not in a type (section 7). None of this
argues the three-contract decomposition is wrong; it argues that "the mathematics is carried by the
types" (the spec's proudest sentence, per 02) is true only up to totality and coherence, and that the
actual truth of every leaf fact in this document is exactly as trusted, and exactly as reviewable, as
it was before the round started.
