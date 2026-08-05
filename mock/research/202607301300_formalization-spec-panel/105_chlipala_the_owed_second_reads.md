# 105. The owed second reads: four confirmations, a sharpened algebra, and a pattern that has now shown up four times without a name

Adam Chlipala, file 105. I wrote file 41 (the rational bias), file 64 (which found a live
orphan-rule-legal forgery among seven checks), and file 85 (the closure audit, which found one
closure already performed before it was marked owed). This dispatch sends me back over five items
from the stretch that produced files 92 through 104, five of which the ledger calls one-pass or
newly corrected, with the instruction to form my own reading of each before reading its author's
conclusion.

All five confirm. Nothing in this file overturns anything. What it adds is independent derivation
where the ledger asked for it, a sharpened algebra where file 103 named its own attack surface and
left it open, and one connection nobody has written down: the retirement, the rank, and the
disjointness defect are the same failure shape at three unconnected addresses, joined by a fourth
this stretch itself produced and never noticed joining. That is the shape my subject is built to
find, and it is what a fourth pass over settled material is for.

## What I read

`102_consolidation_ten.md` in full, the standing base. `103_leijen_platform_and_the_predicate.md`
and `104_kiselyov_what_the_bitfield_is.md` in full. `101b_persona_checkpoint_twentyfour.md` in full.
Behind them, at the primary derivations rather than at the consolidation's compression, because the
brief's method constraint requires checking a claim before reasoning from it and the retirement is a
claim about a claim: `92_spj_the_perimeter_second_reads.md` sections 1.3 and 3 in full,
`95_pesce_does_the_requirement_hold.md` section 2.1 in full, `97_leroy_the_owed_second_reads.md` in
full (the direct predecessor for two of the five items; I read its argument before accepting its
verdict, not instead of forming one). `95b_persona_checkpoint_twentythree.md:105-138` for the exact
ratified wording the retirement and the split each turn on. `100_quilez_shape_and_geometry.md`
section 2.2 in full for the rank claim, at source rather than at `102`'s compression. `91_
consolidation_nine.md:845-855` for the environment-as-assumption sentence the split's soundness
depends on, checked because it predates this stretch and the split's whole defence rests on it
predating this stretch. One `ls` of the panel directory, current through `104_probes`.

From the shipped tree, for the two licensed purposes only: reading a probe's own source to confirm a
compiled claim before trusting its prose summary (`97_probes/probe_1_foreclosed_region.rs`,
`probe_1b_pairing_refused.rs`; `100_probes/OUTCOMES.md` claims A through D;
`104_probes/p1_overlap_shipped.rs`, `p5_occupancy_mask.rs`; `103_probes/OUTCOMES.md` for the symbol
alias). No file under `mock/crates` is read for what the design means; every citation below survives
deleting the shipped-source pointer, and where a probe's own compile is the evidence I say compiled,
not design.

## Gates, run before the work

**Canon gate.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same
with `FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD `3644865`, run 2026-08-05 07:04. The
governing material is the op-ratified round `202607300800`; `102` is the panel's standing base
beneath it. This dispatch touches none of the round's decisions directly, only the panel's own
second-read discipline applied to material the round does not speak to. Gate passed.

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary: **155 binaries,
672 passed, 0 failed, 9 ignored**, matching `102`, `103`, and `104` exactly, from a clean tree, run
by me this session. Full command and output in `105_probes/OUTCOMES.md`.

I did not re-run the full test-body audit `103` and `104` each performed on the surfaces they
touched; this dispatch does not add new source-adjacent claims, so there is no new test surface to
audit, and re-walking greedy.rs and bitfield.rs a third time in as many files would be citation
theatre rather than verification. What I did check fresh: the three known tautologies
(`arvo-tensor/tests/capacity.rs:14-18`, `arvo-tensor/tests/const_capacity.rs:49-53`,
`arvo-hash/tests/aliases.rs:16-23`) are still present at source, unchanged, still in the green total,
re-read this session. Twenty-six files have now carried the first of the three forward. The
disposition stands at `95b`, op's own trivial commit outside the panel; I add nothing to the ruling
except the count.

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`, confirmed inside the tree. `105_probes/OUTCOMES.md` records every command I
ran myself and what it returned; it introduces no new mechanism, because this file's job is to check
five conclusions, not to build a sixth thing.

---

## 0. The answer, first

**All five confirm.** The domain-preservation equation is correctly retired: I re-derived the
vacuous-truth argument from the two primitive facts (the ratified declaration-site totality refusal
and the seal's own only-door law) before reading file 97's conclusion, and it holds without needing
file 97's authority to hold. The cannot-check split is correctly adopted: it is required by a
sentence ratified two stretches before this one, `91:854-855`, and refusing to split would have put
the finisher clause in direct conflict with standing text rather than merely in tension with it. The
rank claim is correctly stated and correctly repaired, and I re-ran the compiled evidence rather than
trusting the summary. The digest correction is correctly one word, and I can say why it is exactly
one word and not two: for an ordinary numeral the occupancy and the extent are the same set by
construction, so the general form is a strict widening that costs nothing on the case the design
already had right.

**The two leans get a genuine second pass, because that is what "one-pass" is asking for.** File
103's lean toward branch B survives, and the algebra behind it is stronger than the file itself
claims: a product of Boolean algebras is a Boolean algebra by a theorem, not by analogy, so the
"truth value or container" question has a settled half and an open half, and the file conflated
them. File 104's convergent statement (heterogeneous product under a declared map, homogeneous
product under a derived one) is exactly the standard move from a vector to a record, correct for a
reason the file does not give, and its reported defect (an overlapping declaration that compiles
clean and silently truncates a neighbouring field) is a textbook illegal-state-representable failure
that I confirm compiles exactly as claimed.

**And one thing nobody asked for, which is what a fourth pass on settled ground is supposed to
surface.** The rank claim (file 100), the platform crate's six doors (file 103), and the bitfield's
disjointness gap (file 104) are the same defect at three addresses that have never been read next to
each other. File 104 half-noticed this for a second type against file 100; nobody has noticed it
against file 103's six doors, and nobody has stated the general form. Section 6 states it.

---

## 1. The retirement: a compile-fail pin, correctly, and here is why without appeal to authority

The claim under second read: the door-side domain-preservation const equation, adopted as a working
shape at `95b:129-137` on file 95's compiled counterexample, is retired at file 97's hand to a
compile-fail pin, because the fact it guards already has a home at the declaration-site totality
refusal ratified in the same checkpoint (`95b:115-118`).

My own derivation, formed at the primitives before I let file 97's argument stand in for one. Two
facts are both ratified, at the same sitting, in the same document (`95b`):

1. **The totality refusal.** A value-unique decode is required to be total over the carrier's
   inhabitants, or declared cohort-style many-to-one; a pairing that is neither is refused **at
   declaration**. This is `95b:115-118`, lifted from file 92's section 1.3 finding that three of
   the audited entry's four facts are const equations over type parameters and belong at the
   declaration rather than in the trusted entry (`92:165-207`).
2. **The seal.** The design's own perimeter machinery, attacked and confirmed by four distinct
   compiler-error classes across every introduction route (`92:89-131`, itself the second read of
   file 46's method), establishes that a value of a sealed numeral type is producible only through
   the type's own construction door. There is no back channel.

Put those two facts next to each other and the retirement is a one-line corollary, not a judgement
call: if (1) refuses every non-total pairing before a value exists, and (2) says no value exists
except through the door (1) guards, then **no value of a non-total pairing ever reaches the point
where a door-side check would matter.** A check placed on the door is checking a precondition that
the type's own construction has already made unconditionally true for every value that can exist.
That is the definition of a vacuous guard, and vacuous guards belong in the test suite as a
regression pin, not in the ratifying text as a second statement of the fact.

I then went to file 95's own probe to see whether its counterexample actually defeats this, rather
than assume the derivation above settles it against evidence I had not looked at. It does not defeat
it, and it is instructive why not: file 95's `probe_1` constructs `Bounded<8192>` paired with
`NonZeroU16` and drives a store through it to an orphaned inhabitant, but at the point the probe was
written the totality refusal existed as a *proposal* in file 92's prose, not as a compiled gate
sitting on `embed`. File 95's model builds the type as though the refusal were absent, which is
exactly the move fact (1) above forbids once ratified. File 97's own probe closes the loop: the
identical const equation relocated from `typed_mut` to `embed` refuses `Bounded::<8192>::embed` with
`E0080` (`97_probes/probe_1b_pairing_refused.rs`, re-read at source and re-verified this session), so
`typed_mut` is never reached, and the sole declarable value-unique pairing sweeps all 65,535
inhabitants through the door with zero orphans and no door-side equation anywhere in the program
(`97_probes/probe_1_foreclosed_region.rs`, full sweep, no sampling).

So the retirement is not "file 97 outranks file 95." It is that ratifying the totality refusal
changed what file 95's own model needed to contain to keep being a counterexample, and once that
change is made the counterexample constructs nothing. Two organs for one fact was never the charge
against file 95's reasoning, which was correct at the moment it was written; it is the charge against
keeping both organs in the text after the checkpoint that made one of them redundant. The pin belongs
in the suite precisely because the argument above is a chain of two ratified facts and a construction
proof, and a chain like that is exactly the kind of thing a later, careless weakening of fact (1)
alone could silently break without anyone noticing until the door-side check (if it still existed)
caught it, or worse, until nothing did.

*Grounded on: ratified (`95b:105-138` in full, both facts at their own lines), settled shapes
(`92:165-207` the three-facts-to-declaration finding, `92:89-131` the seal's route enumeration),
compiled (`97_probes/probe_1`, `probe_1b`, re-read at source and cross-checked against `97_probes/
OUTCOMES.md` this session; `95_probes/probe_1` read to confirm what its model does and does not
contain), reasoned (the two-fact derivation above, mine, formed before reading `97`'s section 2.1
and checked against it afterward, agreeing without needing its authority).*

---

## 2. The split: required by a sentence two stretches older than the clause it corrects

The claim under second read: file 95's finisher, "a numeral whose correctness depends on an
uncheckable field refuses at declaration," is wrong, and file 97's replacement, splitting the
receipt's verdict rather than refusing the declaration, is right.

The independent test I applied before reading file 97's argument: does the finisher conflict with
anything already standing, or only with a preference. It conflicts with standing text, and the text
predates this stretch by two checkpoints, which changes the register of the disagreement from
"which design is nicer" to "which of two adopted sentences is the one to keep." `91:854-855`,
ratified at `90b`, three stretches before this one: *"An environment parameter denotes the control
state the lowering's correctness is conditional on. It is an assumption, never a witness."* I
re-read that sentence at source rather than trust the consolidation's paraphrase, because a claim
this load-bearing is exactly the kind a paraphrase could round off wrong, and it says what it is
quoted as saying.

An assumption, by the sentence's own force, is something the design carries without proof and
declares honestly; that is what distinguishes it from a witness, which the design does prove. File
95's finisher takes one instance of an assumption (a field the target's toolchain cannot check) and
converts it into a case that refuses at declaration unless it is verified, which is asking the
assumption to behave like a witness on exactly the axis the ratified sentence says it is not one.
That is not a stylistic disagreement with file 95; it is file 95's clause and `91:854-855` making
incompatible claims about the same thing, and only one of them can stand.

The distinction the split draws, cannot-check against cannot-provide, is the correct place to cut
because it is the only cut that keeps both standing sentences true at once. A target that cannot
check a field is in exactly the position `91:854-855` describes: the correctness is conditional on
an assumption, the assumption is honestly declared, the receipt's artifact list has a named gap where
a check would go, and nothing about the situation is false, only unverified. A target that cannot
provide the semantics at all (no gradual-underflow path, no software fallback) is a different thing
entirely: the correctness is conditional on something that is **statically known false** on that
target, which is not an assumption in `91:854-855`'s sense at all, and refusing that declaration is
the design's ordinary treatment of a statically known falsehood, unrelated to the environment
chapter.

I want to be precise about what I am and am not endorsing, because a correct-by-construction reading
could be pushed the other way and I considered it before rejecting it. The instinct that produced
file 95's finisher is the right instinct in general: push a fact into the type, refuse what you can
refuse, do not let a silent lie through a boolean check. What makes it wrong here specifically is
that the fact in question is not a fact the type system can observe. An ambient FPU control register,
mutable by any linked library at any point after the process starts, is not a value any Rust type
carries; there is no type-level state to make illegal states unrepresentable **of**. Refusing the
declaration in that case does not close a gap in what the type guarantees, because the type never
guaranteed anything about a fact outside its own observation surface to begin with. It trades an
honest, artifact-tracked assumption for a refusal that looks like a proof and is not one, which is
precisely what the design's own boundary sentence in `arvo-toolbox-not-policer.md` calls policing: a
refusal whose cost the consumer pays for a choice the consumer did not make and could not avoid.

*Grounded on: ratified (`91:854-855`, re-read at source, predates this stretch by three checkpoints;
`95b:96-99` the adopted finisher under second read; `arvo-toolbox-not-policer.md` decision test 1),
settled shapes (`95:437-446` the finisher's own text, `94:374-446` the field-set design the split
folds into), reasoned (the two-sentences-cannot-both-stand argument above, mine, formed from
`91:854-855` directly rather than from file 97's framing of it, converging with `97:291-346`
afterward).*

---

## 3. The rank: confirmed by direct re-execution, and it is the cheapest of the five to verify

The claim: `91:796-802`'s sentence that the array grammar's agreement is "checked to agree in an
inline const block at the one construction door" is false above rank 0, because D4's recursion is
written against the trait method (`Capacity::filled`), which the inherent door's check never
reaches.

This one does not need a derivation from me; it needs a re-execution, and I ran it rather than cite
it. Reading `100_quilez_shape_and_geometry.md` section 2.2 at source: a rank-3 shape whose middle
axis declares a `Nat` of 4 against a literal of 7 produces `COUNT == 12` through one route and
`size_of(Store) == 21` through the other, both const-evaluable, both compiling, disagreeing with
nothing raised. `100_probes/OUTCOMES.md` records the two routes' outcomes as claims A and D, with the
`E0080` text from the inherent door reproduced verbatim and the trait door's silent divergence
reproduced as a returned value rather than an error. I re-read both against the probe source rather
than the prose summary; they match.

This is worth naming for what kind of finding it is, because it sharpens the connective point in
section 6. It is not a new fact about capacities. It is a fact about how many places a type-level
invariant is reachable from, and the shipped design had one check written against one of two legal
routes to the same fact. The repair, lifting the agreement to `const AGREES: bool` declared on
`Capacity` itself rather than checked at one call site, is the shape the design already used for the
level ordering (`91:560-561`), and file 100 is explicit that it claims no novelty for the mechanism.
I confirm the mechanism is the right one and confirm the honest scope file 100 states alongside it:
the repair as first written only fires where `AGREES` is *reached*, so a capacity whose `COUNT` is
merely read without touching `AGREES` still compiles with a disagreeing size, and closing that needs
a second reference (`assert!(Hd::AGREES)` inside `COUNT`'s own definition). Both halves are load-
bearing; I checked this by reading `probe_2` claim C directly rather than the prose describing it,
and the claim-C exit-0 result is exactly what an unreferenced associated const predicts.

*Grounded on: ratified (`91:796-802`, `91:560-561`), compiled (`100_probes/probe_1` claim D,
`probe_2` claims A through C, re-read at source and cross-checked against `100_probes/OUTCOMES.md`
this session, all `E0080` text reproduced verbatim in the outcomes file), reasoned (the reachable-
route framing above, mine, offered to connect this finding to section 6 rather than to relitigate
the repair, which I find correct as stated).*

---

## 4. The two leans

### 4.1 The platform-and-predicate fork: the algebra is stronger than the file claims, and the file's own attack surface has a sharp half and a soft half

File 103 prices the fork between two spellings of the tower's contracts (concrete `Bool`, or generic
over a `Truth` contract), finds both cheap, finds branch B free at runtime by symbol identity rather
than measurement, and leans toward branch B on the ground that a second truth type already ships:
`MaskOps` at `arvo-mask-contracts/src/lib.rs:45-66`, a Boolean algebra under a set-theoretic
vocabulary. File 103 names its own attack surface honestly: "it turns on whether `Mask<W>` is
genuinely a truth value or merely a container of them" (`103:560-562`), and does not settle it.

I settle half of it, and the half I can settle is settled by a theorem rather than by taste, which is
exactly the register a lean like this deserves before op reads it.

**The algebraic half.** A Boolean algebra is a structure closed under finite meets, joins, and
complement, satisfying the distributive and complement laws. Boolean algebras form a variety in the
universal-algebra sense (an equationally axiomatised class), and varieties are closed under arbitrary
direct products, with the operations defined pointwise (Birkhoff's theorem on varieties; the specific
closure-under-products fact for Boolean algebras is textbook, e.g. Sikorski, *Boolean Algebras*, or
any general lattice-theory reference following Birkhoff). `MaskOps`'s five operations,
`empty, full, union, intersection, complement`, are exactly `FALSE, TRUE, or, and, not` applied
lane-wise across `W` independent copies of `Bool`'s own algebra, which is to say `Mask<W>` **is**
`Bool^W` under the product construction, structurally, not by resemblance. This is not a fresh
compiled result and I am not offering it as one; it is a citation-shaped fact about a class of
algebraic structures the way Lindemann-Weierstrass is a citation-shaped fact about transcendence in
section 1.16 of the consolidation. Under this reading, `Mask<W>` is not "like" a truth value. It is
one, in exactly the sense a coordinate of a product group is one element of that group and not a
container of scalars pretending to be a vector. **File 103's lean is correct on this half, and it can
be stated more strongly than the file states it: the unification is not a convenient pun that happens
to compile, it is the free construction a Boolean-algebra-shaped `Truth` contract predicts for any
finite product of truth values, and `Mask<W>` was always going to be an instance the moment the
contract's shape was Boolean-algebra-correct.**

**The half that is genuinely open, and it is a different question than the one file 103 names.** D17's
own contract needs a stated exit, because Rust's `if` takes `bool` and nothing else, and file 103's
section 3.5 already establishes that a foundation naming `bool` at that one exit is not the layering
problem the fork is about. The exit is where the algebra and the container reading come apart. `Bool`
supplies its exit for free: the identity function, since a single truth value already is the thing
`if` wants. `Mask<W>` cannot supply an exit for free, because `if` wants one bit and a mask carries
`W`. Any exit method a `Truth` impl for `Mask<W>` supplies is necessarily a **reduction**: all lanes
hold, some lane holds, a specific lane holds, a majority holds. These are not equivalent operations,
they encode different policies, and none of them is derivable from the Boolean-algebra structure
alone (`and`/`or`/`not` on masks never produce a scalar; a reduction is an additional operation the
algebra does not supply). File 103's own probe found exactly this and reported it honestly rather
than papering over it: the mask-side function in `p8_second_truth.rs` "compute[s] different things"
than the `Bool`-side one and "neither loop is the shape a real lane-wise predicate would have"
(`103:552-554`). That is not a probe limitation to fix later. It is the algebra telling you the exit
is not part of what the two truth types share.

**What this suggests for the shape of `Truth`, offered as a refinement of the lean rather than a
disagreement with it.** Splitting the contract along the seam the algebra actually has: a Boolean-
algebra core (`empty`, `full`, `union`/`or`, `intersection`/`and`, `complement`/`not`), which `Bool`
and `Mask<W>` both satisfy uniformly and which is exactly what compiled byte-identical in file 103's
own measurement, and a separate exit obligation that `Bool` satisfies by identity and that a mask
type satisfies only through an explicitly named reduction, never a default. This is not a new axis
the design has to invent; it is the toolbox-not-policer rule's own decision test applied to the exit
specifically: the reduction policy is a consumer choice (does this call site want all-lanes or
any-lane semantics), the substrate cannot know which, so the substrate exposes the choice rather than
picking one silently inside a blanket `Truth for Mask<W>` impl. Concretely this reads as the core
`Truth`-shaped trait carrying no exit at all, and `if`-usability living on a second, narrower trait
that `Bool` gets by a trivial blanket and a mask gets only by naming the reduction (`AllLanes<M>`,
`AnyLane<M>`, or similar, whatever spelling op prefers), so a consumer writing `if mask.all()` states
the policy at the call site instead of a hidden default choosing it. This changes nothing about the
runtime cost file 103 measured, which was about the algebra's shared operations, not about a
reduction that does not exist in either branch's current signature.

I am one pass on this refinement and I say so plainly: it is a proposal about how to shape `Truth`'s
signature, not a compiled fact, and it deserves the same second read every one-pass item in this
stretch has gotten. What I am confident about, because it rests on a theorem rather than a reading,
is that the algebraic half of file 103's attack surface is settled in the direction the lean already
points, and the part that remains genuinely open is narrower than "is a mask a truth value," which
was never really in question once the product structure is named; it is "what does `if` do with one,"
which is a policy question the design's own standing rule already knows how to answer.

*Grounded on: ratified (`arvo-toolbox-not-policer.md` decision test 1), settled shapes
(`103:474-563` in full, `arvo-mask-contracts/src/lib.rs:45-66` per file 103's own citation, checked
as a factual claim about the crate's shape rather than reasoned from for design meaning), compiled
(`103_probes/p8_second_truth.rs`, read at source; the byte-identical symbol alias at
`103_probes/OUTCOMES.md` for the algebra-core half only), external (the closure of Boolean algebras
under direct products; Birkhoff's variety theorem; Sikorski, *Boolean Algebras*, secondary, standard
and not reasonably requiring a primary-source read the way this review treats an ISA manual or an
IEEE clause), reasoned (the exit-versus-algebra split and the `Truth` refinement, mine, one pass,
offered as suggestion).*

### 4.2 The bitfield's convergent statement: correct, for the reason a dependent product predicts, and the defect it reports is exactly what my subject watches for

File 104's opening claim: a bitfield is the heterogeneous product of numerals under a declared
placement map, a bitpacked column is the homogeneous product of one numeral under a derived one, and
the two differ on exactly two independent axes (index shape, offset provenance), not one.

**Why this is right, independently of file 104's own justification.** A homogeneous product indexed
by a natural number and a heterogeneous product indexed by a finite set of labels are the same
categorical shape: both are a dependent product over a finite index type, differing only in whether
the fibre (the type at each index) is constant across the index or varies per index. This is the
standard relationship between a vector and a record in dependent type theory, and it is not a loose
metaphor; a vector `Fin n -> A` specialises the general form `Pi (i : I) . A i` at `A i = A`
constant, and a record specialises it at `I` a finite label set and `A` varying freely per label. A
column under `Bitpacked` is exactly the vector case; a bitfield is exactly the record case; and file
104's own 2x2 table (index a `Nat` or an hlist, crossed with offsets derived or declared) is the right
table because index shape and offset provenance are genuinely independent choices within that one
general form, not two names for the same fork. I did not need file 104's own justification to reach
this; the shape was checkable from the general form the moment the claim was stated, and I checked it
before reading section 2.1's own argument, which reaches the same place from the design's `place` /
`materialise` machinery rather than from type theory. Two independent routes to the same conclusion
is stronger corroboration than either alone, and I record that they agree.

**The "derived versus declared" axis is real, and file 104's own named attack surface (whether it is
a real axis or "a declared one with a cheap author," since `Dense`'s stride is also derived and
nobody calls it a placement map) resolves in favour of it being real, for a reason that generalises
past this one instance.** A derived placement's well-formedness facts (containment, disjointness) are
theorems about the deriving function, proven once, generically, for every instantiation the function
can produce; a `Dense` stride is exactly this, which is why nobody calls it a placement map needing a
check, since the check was discharged once at the function's own correctness proof and never needs
re-discharging per declaration. A declared placement's well-formedness facts are not proven by
anything until something proves them per instance, because the placement is data a human wrote, not
the output of a proof-carrying derivation. This is the provable-versus-trusted distinction the whole
review has organised itself around, applied one level down: **derived is provable-by-construction,
once; declared is provable-by-checking, per instance, or trusted if nobody writes the check.** That
is a real structural difference in where the burden of proof lives, not an authorship note, and it is
exactly why `Dense`'s stride needed no obligation while a bitfield's declared map needs two.

**The defect.** File 104 reports that the shipped macro asserts containment but not disjointness, and
that an overlapping declaration compiles clean while a setter silently truncates its neighbour, with
zero diagnostics of any kind. I re-read `104_probes/p1_overlap_shipped.rs` at source rather than the
prose describing it: the declaration is `Overlap: 16 { a: 8 at 0, b: 8 at 4 }`, both fields satisfy
the shipped macro's own containment check, `with_a(0xFF)` followed by `with_b(0x00)` reads `a` back
as `0x0F`, and the run output in `104_probes/OUTCOMES.md` matches exactly. This is worth stating in
my own vocabulary rather than restating file 104's, because it is the cleanest instance in this whole
stretch of the exact failure my subject exists to prevent: **an illegal state, two fields claiming
overlapping storage with inconsistent declared values, is fully representable, fully constructible,
and reached through wholly safe code with no signal at any point.** Correct-by-construction does not
mean "the design intends the fields to be disjoint and documents so"; the shipped macro's own comment
does exactly that (`arvo/src/bitfield.rs:28-30`, "authors are responsible") and it is precisely the
kind of hand-enforced invariant this review has now corrected at three other layers. Making the state
unrepresentable means the compiler refuses the declaration, which is exactly what file 104's O(k^2)
const-item repair does, refusing with the same `E0080` shape the rank repair in section 3 above uses.
I confirm the repair is the right one and confirm the honest exception file 104 states alongside it:
a deliberately aliasing view field (a whole-register accessor beside its named sub-fields) is a real
idiom, so the correct default is refuse-unless-declared rather than refuse-outright, which keeps the
toolbox rule's own boundary intact rather than policing a legitimate consumer choice.

*Grounded on: settled shapes (`104:93-149` the opening claim, `104:274-350` the obligation and
defect), compiled (`104_probes/p1_overlap_shipped.rs`, `p1b_disjointness.rs`, `p4c_refusal_locus.rs`,
re-read at source and cross-checked against `104_probes/OUTCOMES.md` this session), external (the
dependent-product / vector-versus-record correspondence, standard dependent type theory, e.g.
Martin-Lof-style Pi types specialised at constant versus varying fibre; not requiring a primary
citation the way an ISA fact does), reasoned (the independent-route confirmation and the provable-
by-construction-versus-by-checking framing of the derived/declared axis, mine, converging with
`104:2.1` and `104:3.1-3.3` without needing them, checked against them afterward).*

---

## 5. The digest correction: one word, and here is why it is exactly one and not two

File 104 reports that the ratified sentence "masks the container straight to the fields' own width"
(`91:628-631`) is a prefix mask, correct for a numeral and wrong for a placement map with an interior
hole, exhibited exhaustively over a 16-bit container where the prefix mask separates 65,536 pairs
that agree at every declared field.

This is the item the dispatch names as deserving the most care, because it corrects ratified text
rather than an adopted working shape, so I want to state precisely why the correction is minimal
rather than just confirm that it is. **For an ordinary numeral, the occupancy and the extent are
identical sets, by the numeral's own ratified structure**, not by coincidence: `Encoding::Fields` is
defined to occupy a contiguous low run: `91:530-545`'s three-level model derives `W_F` as a
single width with statement P's own domain starting immediately at `[W_F, W_S)`, which structurally
leaves no room for a gap inside the fields region, and file 104 names the consequence in these words
(`104:223`, "a numeral, whose `Encoding::Fields` occupy a contiguous low run"). So the union of
"field ranges" for a numeral is, trivially, the single contiguous range `[0, W_F)`, which is exactly
the extent. The prefix mask and the occupancy mask compute the same bit pattern for every numeral
that exists today, because a numeral by definition has no interior hole to distinguish them. So
"masks the container to the placement map's occupancy" is not a new statement replacing the old one;
it is the same statement, generalised at the one place the old wording assumed a property (contiguity
of the field region) that the design happened to have everywhere it had been checked and that a
bitfield's declared map does not have to have. I checked this by re-reading `91:530-545`'s own
width model (which forces the contiguity structurally, per above, without using the word) before
accepting file 104's claim that the fix is free for the existing case, and it holds: the fix widens the statement's domain without touching its value on the domain it already
covered.

I re-read `104_probes/p5_occupancy_mask.rs` at source to confirm the exhibited defect rather than
trust the count in prose: `Reg: 16` with `enable` at bit 0 and `divisor` at bits 5 through 13 leaves
bits 1 through 4 an interior hole, the prefix mask is `0b0011111111111111`, and the sweep over all
65,536 container values reports the prefix mask separating 65,536 equal pairs against zero for the
occupancy mask, matching `104_probes/OUTCOMES.md` exactly. The cost, one extra `and`-with-a-loaded-
immediate on this host because the occupancy word is not an ARM logical immediate, is the correct
honest accounting rather than a claim of zero cost everywhere, and I confirm it is stated that way in
the source rather than rounded up or down.

The one thing I would add, which the dispatch's framing invites and file 104 does not quite say in
these words: the statement-P repair (making P's own region "the complement of the occupancy" rather
than "everything past `W_F`") is not a second fix riding alongside the digest fix. It is **the same
generalisation stated once**, because the digest's masking region and statement P's canonicalised
region are, for a numeral, the same complement of the same set, and file 104's own text says the word
is the same word (`104:519-524`). I checked that this is not merely a verbal coincidence: both
statements are answering "which bits does an observation of a datum-level fact legitimately discard,"
and a placement map's interior hole is exactly a region neither statement had a name for because
neither had ever needed one. One correction, one word, two ratified sentences it happens to touch,
because they were always the same fact stated twice at two addresses for the two things that read it.

*Grounded on: ratified (`91:628-631` re-read at source, `91:530-545` the width model that forces
contiguity structurally, checked
before accepting the fix is free for existing numerals), compiled (`104_probes/p5_occupancy_mask.rs`,
re-read at source and cross-checked against `104_probes/OUTCOMES.md` this session, the sweep count
and the emitted-instruction cost both verified), reasoned (the occupancy-equals-extent-for-a-numeral
argument and the one-fact-two-addresses reading, mine, formed from `91:530-545` directly before
reading `104`'s own framing of the P-statement connection, converging with it afterward).*

---

## 6. The pattern that has shown up four times and has never been named

This is the section I am adding rather than confirming, and it belongs in a file whose subject is
correct-by-construction and unrepresentable illegal states, because it is exactly that subject
recurring across three files that were each dispatched to look at something else.

**File 92, section 2.1** (the mutation gap, ratified two consolidations ago): the theorem "no raw
accessor" was found to undercount the perimeter, because a public field is a raw door with no
accessor at all, reachable from wholly safe code with nothing in the type's signature warning a
reader. **File 100, section 2.2**: the array grammar's agreement was checked at one of two legal
routes to the same fact (the inherent door), and the trait-method route, reaching the identical type,
bypassed the check entirely with no diagnostic. **File 103, section 1.3**: `Bool` names itself once
in the design's vocabulary but exits to `bool` through six independent routes, a public field, a
`Transparent::raw`, a `Deref`, a `From`, an `AsBool`, and a `Try::branch`, of which the charter names
exactly one as intended. **File 104, section 3.1-3.2**: a bitfield's disjointness fact is unchecked
entirely, and even its containment fact, which is checked, is checked only where the check is
*mentioned* (two specific lines inside two specific constructors), so a refactor dropping either
mention silently drops the guarantee with the type's own signature unchanged.

I searched the corpus for whether any file has already stated these four as one thing.
`grep -rln "six doors\|six exits\|six routes" *.md` returns only file 103 itself, run 2026-08-05
07:11. `grep -c "AGREES" 103_leijen_platform_and_the_predicate.md` and `grep -c "disjoint"
100_quilez_shape_and_geometry.md` both return zero, same time. File 104 draws the connection to file
100 explicitly and by name ("This is file 100's AGREES finding at a second type, from the other
side," `104:337`), but no file connects either of those to file 103's six doors, and no file states
the shape all four share in general terms. Nobody has run this grep before because nobody had reason
to ask it; it is exactly the kind of connection a second-read dispatch across four files exists to
surface that a first-pass dispatch on any one of them could not have seen.

**The shape all four share, stated once:** a fact is claimed true of every value of a type. The fact
is actually checked, or actually enforced, at exactly one of the legal routes by which a value of
that type is produced, observed, or exited. Every other legal route bypasses the check silently,
because the type's own signature carries no trace of which routes were checked and which were merely
assumed to coincide with the checked one at the time the check was written. The gap is not a bug in
the check; the check is correct on the route it guards. The gap is that "checked" was read as a
property of the fact rather than a property of the fact-at-a-route, and a type can have more than one
legal route the moment it has more than one public constructor, more than one public accessor, or
more than one generic-versus-concrete path to the same operation.

This is the workspace's own perimeter rule (`what-you-can-observe-is-what-you-guaranteed.md`),
already cited by file 92 and by file 103 independently, and it is already doing real work in both
places. What none of the four files states is that it applies to **facts**, not only to **fields**.
The rule as written asks "does the property survive what the perimeter permits," phrased around
observation of a value. The four instances above are all about **construction and computation
routes**, not observation: `AGREES` is a fact computed two ways that should agree and does not; the
bitfield's containment check is mentioned on two specific lines and absent everywhere else a value
could come from; `Bool`'s six doors are six ways to leave the type, not six ways to look inside it.
The pattern is the same rule, one register wider than its own stated form: **a guarantee about a type
holds only over every route by which a value of that type can be produced, transformed, or exited,
and every public constructor, every public accessor, and every generic-dispatch path that reaches an
associated const is part of that perimeter, whether or not it was the route the author was looking at
when the guarantee was written.**

**What I am suggesting, and stating plainly as a suggestion because only op's calls are final.** Not
a fifth design rule; this stretch has already twice found that the right response to a rule that
works but goes unrun is a moment naming when it runs, not a new rule (`102:110-117`'s own account of
why the freshly-performed-search requirement joined the separation requirement rather than replacing
it). The same move applies here: the pricing pillar already says a fact about a type belongs on the
type as an associated const rather than at a call site; the perimeter rule already says a guarantee
holds only over what can be observed. What is missing is the one sentence joining them for the
construction-and-computation case rather than the observation case, and it is cheap to write:

> A fact stated as true of every value of a type is checked at the type, once, reachable from every
> legal route to a value of that type, not at whichever route its author had in mind. Where the fact
> is const-computable, this means the associated const carries the check and every route that reaches
> the fact's dependents mentions it; a route that reaches the same fact through a different door and
> does not mention the check has not inherited it.

That sentence would have caught the `AGREES` gap and the bitfield's disjointness gap as the same
finding before either was found separately, and it names precisely why `Bool`'s charter needs one
door rather than six: not because six is untidy, but because a guarantee stated once and reachable
through six unequal routes is, by the definition above, guaranteed over none of the six until each
is individually checked, which is exactly what section 1.3 of file 103 found by hand, one door at a
time, without a general sentence to name what it was doing.

I offer this as a candidate clause on the pricing pillar, not a ruling, and I want to be honest about
its status: it is a synthesis across four already-ratified or already-adopted findings, not a fifth
compiled fact of my own. Its evidence is the four instances above, each independently compiled by a
different file, and its own claim to correctness is exactly as strong as the observation that all
four instances are real, which I re-verified at source rather than trust, and no stronger.

*Grounded on: ratified (`92`, `100`, `103`, `104` at the sections cited above, each re-read at
source), settled shapes (`what-you-can-observe-is-what-you-guaranteed.md` in full, `91:113-121` the
pricing pillar, `102:110-117` the requirement-versus-rule precedent), reasoned (the connective
reading and the candidate clause, mine, one pass, offered as suggestion, freshly searched this
session: "six doors/exits/routes" `grep -rln`, 2026-08-05 07:11, one file; "AGREES" in file 100 and
"disjoint" in file 103, both zero, same time).*

---

## 7. What this file does not decide

Nothing here overturns an adopted or ratified shape, so there is no fork to hand back. What is owed:

- **The `Truth`/exit split proposed in section 4.1** is one pass, mine, and needs the same second
  read every one-pass item in this stretch gets. Its attack surface: whether splitting the exit out
  of `Truth` costs anything at the fifteen declaration sites file 103 counted, which I believe is
  zero but did not compile, since no exit method exists in either branch's current signature to
  measure a split against.
- **The candidate pricing-pillar clause in section 6** is a suggestion, and its own second reader
  should attack whether it is genuinely a joining of two existing rules or a disguised fifth rule; I
  believe the former and named my reasoning, but the discipline this stretch keeps applying to itself
  applies here too.
- Everything named open by files 97, 100, 103, and 104 in their own sections remains open exactly as
  they left it; I did not re-audit their owed-artifact lists, only the specific claims the dispatch
  named.

## 8. The three requirements, performed on this text before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions.
*Reachable route* and *legal route* (section 6): defined at first use as any public constructor,
public accessor, or generic-dispatch path by which a value of a type is produced, transformed, or
exited, distinguished there from *observation*, which the workspace perimeter rule already covers.
*Exit* (section 4.1): taken from file 103's own use (`103:619`) and given the content this file adds,
that an exit is necessarily a reduction for any truth type wider than one bit, distinguished from the
algebra's own operations, which never produce a narrower result. *Boolean-algebra core* (section
4.1): defined at first use as the operation set closed under products, `empty/full/union/
intersection/complement`, distinguished from the exit obligation. *Occupancy* and *extent* (section
5): used from file 104's own definitions (`104:684-687`) without redefinition, since this file adds
no new content to either term, only the argument that they coincide for a numeral. Terms used from
the record without redefinition: the perimeter rule, the pricing pillar, the toolbox rule, the seal,
the provable-versus-trusted split, `NonZeroCarrier`, `Capacity`. No term in this file's own new prose
is left undefined or uncited.

**The separation requirement, performed.** Two models are this file's own. First, section 6's
route-versus-fact distinction separates *checked-at-a-route* from *checked-of-the-type*: it is
nonvacuous at all four cited instances (a fact checked at one route and silently bypassable at
another), and vacuous the moment a type has exactly one public route to the fact in question, which
is why the level-ordering refusal (`91:560-561`) never needed the wider statement, since it was
already at the type's sole const definition before this stretch's findings existed to distinguish it
from anything. Second, section 4.1's algebra-versus-exit split separates *shared operation* from
*policy-laden reduction*: nonvacuous at `Mask<W>`, where `and`/`or`/`not` are free and shared while
`all()`/`any()` are neither, and vacuous at `Bool`, where the exit is the identity and the two
categories collapse to the same one operation, which is exactly why file 103's own probe measured a
symbol alias at the algebra half and could not have measured anything at an exit that does not exist
in either branch's signature.

**The freshly-performed-search requirement, performed.** Every universally quantified negative above
carries its own search, run this session, quoted with its date.

- "No panel file names `six doors`, `six exits`, or `six routes`" outside file 103 itself:
  `grep -rln "six doors\|six exits\|six routes" *.md`, 2026-08-05 07:11, one file (`103`).
- "No panel file mentions `AGREES` in file 100's context or `disjoint` in file 103's context":
  `grep -c "AGREES" 100_quilez_shape_and_geometry.md` and `grep -c "disjoint"
  103_leijen_platform_and_the_predicate.md`, 2026-08-05 07:11, both zero (checked in the file each
  finding is absent from, the cross-file absence being the claim).
- "No panel file cites a product-of-Boolean-algebras theorem or Birkhoff for this question":
  `grep -rn "Birkhoff\|variety of algebras\|product of Boolean" *.md`, 2026-08-05 07:11, one hit
  (`14:238`), a distinct citation about lattice-ordered monoids for `arvo-graph`/`arvo-comb`,
  unrelated to Boolean algebras or masks; read at source to confirm the contexts do not overlap.
- "No panel file discusses a `Mask<W>` exit method, reduction policy, or `all()`/`any()` split":
  `grep -rn "Mask.*exit\|exit.*Mask\|reduce.*mask\|mask.*reduce\|all()\|any()"
  103_leijen_platform_and_the_predicate.md`, 2026-08-05 07:11, empty.

The honest limit, inherited from files 97, 98, 101, 103, and 104 in turn: these performances verify
that this file's own terms are placed and its models have content and its negatives were searched
with my vocabulary. They do not verify that a discussion using none of my search terms exists
somewhere in one hundred and four files. A second reader with different terms is the check on that.

## 9. Standing

Four items confirm outright: the retirement, re-derived from the two ratified facts that make it a
corollary rather than a preference; the split, required by a sentence two checkpoints older than the
clause it corrects; the rank claim, re-executed rather than cited; the digest correction, shown to be
exactly one word because the occupancy and the extent were always the same set for the case the
design already had. The fifth, the platform-and-predicate fork's lean, confirms on its algebraic half
by a theorem and is sharpened on its open half by naming what the algebra does not supply for free.

What this file adds beyond confirmation is a connection: three files across three separate dispatches
each found a version of the same defect (a type-level fact checked at one route and silently absent
from another) and none of them noticed the others had found it too. That is not a criticism of any
of the three; each was dispatched to look at one crate, one type, one decision, and the connection
only becomes visible once all three sit in the same file. It is exactly the kind of finding a fourth
pass over settled ground exists to produce, and it is offered the way everything else in this file is
offered: as a suggestion, one pass, owed its own second read, with the compiled and ratified evidence
that support it cited at every claim above.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (`95b:105-138`, `91:854-855`, `91:796-802`, `91:560-561`, `91:628-631`,
`91:530-545`, all re-read at source this session), settled shapes (`92`, `95`, `97`, `100`, `103`,
`104` as cited per section), compiled (`97_probes/probe_1`, `probe_1b`; `100_probes/probe_1`,
`probe_2`; `103_probes/p8_second_truth.rs`; `104_probes/p1_overlap_shipped.rs`,
`p5_occupancy_mask.rs`; every one re-read at source and cross-checked against its own OUTCOMES.md
this session rather than trusted from a consolidation's summary), external (the closure of Boolean
algebras under direct products, Birkhoff's variety theorem, secondary; the dependent-product
correspondence between vectors and records, standard type theory, secondary), reasoned (the
independent derivations in sections 1 through 5, formed before reading each item's own author's
conclusion where the dispatch required it, and the connective reading in section 6, all mine, all
offered as suggestion and not as ruling).*
