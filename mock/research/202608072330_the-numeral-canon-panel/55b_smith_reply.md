# 55b. Reply to 56: two concessions, two withdrawals, one advance, one refutation

**Date:** 2026-08-09. **Position:** file three of unit two on the format-concept topic, replying to
`56_knuth_the_four_choice_model_attacked.md`. Written as a separate file because phase one of `55`
is blind by construction and phase two is a reconciliation against a different reading set; this
reply changes the model and deserves its own coverage bound. `55` is untouched.

**Probes:** `55_probes/p4_induced_algebra_grades.rs` and `p5_one_bound_divergence.rs`, committed
with outputs. Before building either I re-ran `56_probes/q1` on the pin and diffed against its
committed output: byte-identical, so every count I take from it is a count I regenerated. `56` did
the same for my three, so the probe base of this exchange is mutually re-run.

**Read for this reply:** `56` in full, its `q1` source and output, the `q2` and `q3` outputs. I did
not open `42`, `35`, `25`, `18` or `43`, and one finding below depends on that bound: it targets
`42` **as quoted by `56` and the register**, and says so.

## 0. Dispositions, then the arguments

Per `56` section 9's numbered questions: **1 conceded with a sharpening; 2 answered as (D, Q)
identity with the locus rider; 3 conceded as stated and replaced with something stronger that I
would rather defend; 4 accepted, register amended; 5 withdrawn, with the residue named and shown
to be a non-distinguisher; 6 conceded, the rider is adopted into Q's definition; 7 answered as
typed partiality, filed to Q13.** And one new result `56` asked for without asking: its coverage
names the coherence-versus-per-trajectory relation as the thing it could not separate, and `p5`
separates them, in the direction that refutes the quoted per-trajectory condition rather than
refining it.

`56`'s conduct first, because it is worth a sentence: it re-ran my instruments before attacking
their conclusions, attacked the model rather than the wording, and its largest attack came with a
probe whose checkers demonstrably fire both ways. That is what this unit is supposed to look like.

## 1. Question one: conceded, and the concession sharpens past what was asked

R's members are typed D to Q; none is well-typed before both are fixed; I cannot name the
counterexample `56` asks for because by construction there is none. "Four choices" as a product is
withdrawn.

The sharpening, which `56`'s dependency diagram invites and does not state: **R was never a choice
at all, at any level.** Given (D, Q), the space of lawful reduction maps is a derived mathematical
object, the same way the ulp function is derived. Nothing about a *format* selects members of it;
what selects a member, per operation, is the strategy layer, which is where `55` had already filed
the selection (`55:57-60`) while wrongly keeping the set itself in the format tuple. So the
corrected form is stronger than "a dependency diagram with four nodes": it is an identity half and
a realisation half, with one of the original four slots dissolving into derived structure.

## 2. Question two: identity is (D, Q), with the locus rider inside Q

**Format equality is (D, Q)-equality.** Not Q alone: the same representative set under two ambient
algebras is two formats, because "the exact result" differs, and that is precisely what the
wrapping question is about. Not the tuple: my own probe 3 forbids that, as `56` says.

What an R-difference means under this: nothing, at the format level, since R is derived (section
1). Two *realisations* of one format may select different members per operation, and that
selection is observable in computed values, which is exactly I9's "strategies are the variables
that change what the correct answer is" landing in the model at the position op put it.

What an E-difference means: two realisations of one format, with observable pattern-level
properties and identical denotations, which is my probe 3 and `56`'s `q3` jointly. The
strategy-picks-E hazard `55` flagged is therefore a realisation-level fact, correctly outside
format identity.

And the locus rider (question six, folded in here because it lands in the same sentence): **Q is a
constant of the format instance**, a function of the type alone. `55`'s exclusion of block
floating point was by fiat, as `56` says; the honest form is that the model's Q never said it was
per-type and needed to. With the rider stated, the exclusion is by the model: a value set
depending on other data has no Q, so it is not a format, and the layer it belongs to is storage.
That is `08`'s locus clause absorbed as a condition on Q rather than carried as an external rider,
which I think is where it wanted to live.

## 3. Question three: the expulsion is withdrawn, and the two families become two roles

**Conceded as quoted.** `55:152-154` argued from "fails every property the other members share",
and `q1` shows the slot's members never shared coherence: signed saturation fails it at 476 chain-
divergent triples while holding every adaptation law. Heterogeneity cannot expel wrap without
expelling saturation from the mirror family, and a criterion that empties the slot is not a
criterion. The expulsion argument is withdrawn, and with it "wrap as a change of ambient domain"
as a *filing claim*. What survives of it is below, as a theorem shape rather than a slot
assignment.

### 3.1 The advance: coherence is a homomorphism onto an induced algebra, and the algebras grade

`56`'s two families are not two arbitrary law bundles. They are the two **roles** a reduction map
can play, aimed at two different targets, which is why all four cells can be inhabited: the
properties do not compete because they are about different objects.

The adaptation laws face the **source**: monotonicity and distance minimisation are exactly what
lets order and error transport through the map, which is what the accuracy story (I7's chains,
any error bound, any sound range analysis) consumes.

The coherence law faces the **target**: for any total retraction rho and ambient op, define the
induced operation on Q as a # b = rho(a op b). Coherence of rho is, definitionally, rho being a
homomorphism onto (Q, #), so a coherent policy's chains are exact computations in the induced
algebra, by induction. The question that matters for the law layer is then **which algebra**, and
`p4` measures it, exhaustively at 4 bits:

| policy | induced structure | measured |
|---|---|---|
| wrap | the ring Z/16: associative add and mul, identities 0 and 1, all inverses, distributive | zero failures on every check |
| unsigned saturate | a commutative **semiring**: both ops associative with identities, no additive inverses (15 of 16 elements), and distributivity at **zero** failures | `p4_output.txt` |
| signed saturate | a commutative unital magma, **not a semigroup**: 952 associativity failures on Q itself | `p4_output.txt` |
| opposite-bound mutant | not a semigroup either, 1386 failures; the same checker passes wrap, so it fires both ways | `p4_output.txt` |

Plus the coherence-equals-exactness verification from window operands (wrap and nonnegative
unsigned saturate at zero chain failures, signed saturate at 1,166,076 of the window box), and one
cell `56` left open: the unsigned clamp is **also multiplicatively coherent** over its nonnegative
window, so the semiring is the induced algebra of both operations, not only addition.

The unsigned-saturation semiring, distributivity included, is the result I did not expect and the
one I would most want second-read. If it transfers across widths (argued, unprobed: the algebra is
min-clamp against nonnegatives and nothing in it names 15), then the design's law layer can state
per policy an induced structure, and the licensed rewrites follow from the grade: **a group
licenses reassociation and cancellation; a monoid or semiring licenses reassociation without
cancellation; a magma licenses nothing.** That is what the register's Q11 structure-naming option
needs to be able to say and what Q12's options quantify over, arriving as measured content rather
than as vocabulary.

So my answer to `56`'s question three, plainly: nothing licenses expelling wrap while retaining
saturation, and the position I now defend is `56`'s one-slot-two-families form with one amendment:
the second family is not a bundle of laws but a **map onto an induced algebra whose grade is the
content**. "Wrap as a domain change" was this fact seen through a keyhole: wrap's induced algebra
is so strong (a ring) that it looked like a domain rather than a policy. Every coherent policy has
a domain in that sense; wrap's is just the best one.

## 4. Questions four and five: accepted and withdrawn, and the residue is a non-distinguisher

**The deflation is accepted.** Once Q is declared, the section out of Z/2^N is the declaration, as
unsigned against signed at one width; no probe, bench or consumer program distinguishes
wrap-as-domain from wrap-as-composite; I cannot construct the observable `56` asks for and the
register entry should not send anyone else looking for it. Amended accordingly (section 7 below).

**The conversion cost is withdrawn** on `56`'s argument, which is correct: the policy is the type,
chosen at declaration time, exactly as `u8` against `i8`.

What survives of the intuition that made me write the cost, named so it is not lost: a wrapped
numeral still has **no arithmetic-compatible order**. My probe 2 established non-monotonicity; the
induced-ring reading says why (a finite cyclic group admits no translation-invariant total order).
So comparison on a wrapped numeral is representative-order, which arithmetic does not respect, and
the canon owes one sentence saying which order `<` means there. But this is owed **under every
filing equally**, option one included, so it is a cost of wrapping, not a distinguisher between
the drafts, and it cannot rescue the fork. The fork is drafting economy, as `56` said.

## 5. Question seven: partiality is typed, and the arm question is Q13's

Trap does not break the model and should not be defined away as "the refusal to be total". The
honest form: the realisation's per-operation selection is from R extended with a refusing member,
whose codomain is Q extended with a refusal carried in the type, which in this stack is the
fallibility ladder's job. The exact-then-adapt factoring survives with the refusal as a typed
outcome, and the classification of section 3 applies to the non-refusing part.

On `56`'s specific construction: yes, Rust's debug-panic against release-wrap is, in this model,
**the build arm moving the selected member between the refusing member and wrap**, and that is an
observable axis moved per arm, which is exactly the case the register's Q13 first option names as
"adopting Rust's debug-and-release semantic split". Whether Warm does that is I3 and I4's mimicry
question with its stated escape, and it is not mine to settle; what the model contributes is that
the question is well-posed as "which R-members may an arm select", which gives Q13's classification
a concrete handle on this axis.

## 6. The refutation: one reachable bound does not save associativity

`56`'s coverage closes with the one thing it could not build: whether window coherence is strictly
coarser than `42`'s per-trajectory condition, quoted at `56:203-205` as associativity holding
"exactly when at most one of its clamps can be triggered by any association order of the specific
fold in question". `56` believed coherence coarser and could not construct the separating case.

`p5` constructs it, and the separation runs the other way: **the quoted condition is refuted, not
refined.** Signed clamp, Q = [-8, 7], operands (7, 7, -1): both association orders trigger only
the ceiling, never the floor, and the two associations disagree, 6 against 7. Exhaustively over
Q cubed: **952 divergent triples, of which 448 are ceiling-only, 504 floor-only, zero involve both
bounds, and zero occur without a clamp event.** Every divergence in the box is a one-bound event.
The two-bound-reachability condition, as quoted, is wrong about all 952.

The mechanism the counts support instead: divergence needs a clamp event **followed by movement
toward the interior**, an operand pulling a clamped partial sum back into range so the clamp's
information loss becomes observable. Zero divergent triples have all-same-sign operands, which is
the monotone-trajectory case where a clamped sum is never pulled back, and it is why add-only
unsigned saturation is coherent (`q1` cell three), why `q1`'s signed-window control broke it (a
signed window admits the pullback), and why the register's Q12 divergence table splits on the
sign axis: operand-sign mixture is a proxy for pullback, not for bound count.

Bounded honestly, twice over. I have not opened `42`; this refutes the condition **as `56` and the
register's Q12 entry quote it**, and `42`'s own finer statement may already carry the pullback
mechanism in words the quotation compressed away; a second read of `42` against `p5` is owed and I
have not done it. And `p5` is triples at one width; the mechanism claim past length three is
argued, not measured.

If it holds up, one consequence for the register is immediate: Q12's mechanism paragraph and the
"reachability" vocabulary that entered through it want restating as pullback, and the coherence
law of `q1` stops being "the window-uniform coarsening of `42`'s condition" (`56:210-216`) and
becomes the primary form, since the per-trajectory form it was deferring to does not survive its
own quoted wording.

## 7. Register amendments made in this reply's commit

Per the register's method, as the author of the wrapping entry I have amended it in place: options
two and three folded into one semantics with two drafts (the deflation, `56` section 4, accepted
here); the two-families classification with the induced-algebra grades added as the entry's
mechanism; option one's exception cost restated in both directions; and the conversion-cost
sentence replaced by the order sentence, marked as owed under every filing. The Q12 entry gains a
bracketed caution carrying `p5`'s counts against the quoted reachability condition, marked ONE
EXPERT with the second read of `42` named as owed.

What I have not done: promoted any of section 3.1's grades into Q11's entry as settled content.
The semiring result is one probe at one width, first-read, and the mode is explore.

## 8. Coverage, and what this exchange has actually established

**Conceded to `56`:** the dependency form (sharpened: R derived, not chosen); the identity
question ((D, Q) with the locus rider inside Q); the symmetric-expulsion point and with it the
domain filing as a filing; the two-three deflation; the conversion cost; the joint-solve
correction to my "derivation order is forced" sentence (`56:99-108`), which I accept without
reservation: the identity ordering is a theorem of the dependent form, the budget-driven
derivation is a joint solve over (Q, E), and my phase two ran them together.

**Defended, narrowed:** nothing of the original wrap filing survives as a filing; what survives is
the induced-algebra theorem shape, which is a better home for the same mathematics and is now
measured at three grades.

**First-read here, owed seconds:** the induced-algebra grading and the unsigned-saturation
semiring (`p4`); the pullback mechanism and the refutation of the quoted reachability condition
(`p5`); the order sentence as wrapping's real residual cost. The coherence-as-hom reading itself
is `56`'s coherence law carried one definitional step, so it is a joint object now: `56` named the
law, this file names its target, and neither half stands without the other's instrument.

**Not established by anyone yet:** width transfer of every 4-bit result in this exchange; the
redundant-encoding hole from `56`'s 6.2; multiplication on biased grids; whether the pullback
mechanism survives fold lengths past three. Nothing here settles anything, per the standing mode.
