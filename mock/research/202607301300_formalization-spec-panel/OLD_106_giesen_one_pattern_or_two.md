# 106. One pattern or two: the survival mechanism is mine, the defect underneath is three different defects with three existing owners, one of the four instances is not an instance, and the material the proposed clause would add has been produced four times in this review and absorbed zero times

Fabian Giesen, file 106. I wrote file 34 (three of whose claims were later overturned by people who
recompiled them, section 5), file 48, file 72, and file 86, which named the pattern this dispatch asks
me to test against a new one and from which the lead designer adopted the separation requirement.

The short answer is that file 105 found something real and named it one register too wide, and that
the reason its finding felt new is worse than the naming. Its survival mechanism is the one I named,
exactly, with a pair of coinciding quantities I had not found, which is a genuine addition. Its defect
is three different defects wanting three different repairs, two of which an existing rule already
covers and states better than the proposed clause does. Its fourth instance is not an instance, and I
compiled the reason rather than argued it.

And the residue, the part I spent most of this dispatch expecting to be the one genuinely new thing,
is not new either. The mechanism sentence it would add was written at file 55, in almost the words
file 105 reaches for, and it appears in **none of the ten consolidations**. A second general clause,
covering the survival mechanism itself, was absorbed at the ninth consolidation and dropped by the
tenth under a heading saying nothing had changed. So the review has produced this material four times
across fifty files and carried it forward zero times, and file 105's candidate clause is the fourth
production rather than the first.

That reframes the answer. **The design does not need a clause. It needs the compression discipline
that already exists for tables extended to the four rules and three requirements, which are the only
prose that gets restated verbatim in every consolidation and are therefore the only prose that can
lose a sentence silently.**

## What I read

`102_consolidation_ten.md` in full, the standing base. `103_leijen_platform_and_the_predicate.md`,
`104_kiselyov_what_the_bitfield_is.md`, `105_chlipala_the_owed_second_reads.md`, all three in full.
`101b_persona_checkpoint_twentyfour.md` in full. One `ls` of the panel directory, current through
`105_probes`.

Behind them, at the primary text rather than at any compression, because every load-bearing citation
below is to a sentence and not to a summary of one: `91_consolidation_nine.md:113-126` (the pricing
pillar as the ninth states it, which is section 4.1) and `:780-802` (the capacity resolution),
`55_mcsherry_typing_the_algorithm_crates.md:145-170` (which is section 4.2 and the largest thing
here), `100_quilez_shape_and_geometry.md` section 2.2 in full plus `100_probes/probe_2` and its
`OUTCOMES.md`, `92_spj_the_perimeter_second_reads.md` section 2.1 in full, `79_dolan_what_capacity_is.md`
sections 3 and 4, `83_lattner_how_many_widths.md:290-316`, `25_xu_building_the_exact_product.md:54-110`
(the `type const` mechanism, already worked out sixty files before I reached for it), and my own
`86:240-330`, `34:176-190`, and `48_probes/probe_2`.

The workspace rule `what-you-can-observe-is-what-you-guaranteed.md` in full, including its Boundary
section, which is the citation section 3.2 turns on.

From the shipped tree, for the two licensed purposes only, evidence about why a redesign is happening
and checking a factual claim before reasoning from it: `arvo-storage/src/platform.rs:45,60-77,255-345`
(which routes and which invariants exist), `arvo-tensor/src/capacity.rs:19-58` and
`arvo-tensor/src/lib.rs:21` (whether an unpaired capacity is buildable and at what gate cost),
`arvo/src/bitfield.rs:28-30,370-374,377,393,399`, and the three known tautologies. Every judgement
below survives deleting its shipped-source citation; where one would not, I say so and withdraw it.

## Gates, run before the work

**Canon gate.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD `8a92eb4`, run 2026-08-05 07:22. The
governing material is the op-ratified round `202607300800`; `102` is the panel's standing base beneath
it. This dispatch touches no decision of that round directly. Gate passed.

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary: **155 binaries, 672
passed, 0 failed, 9 ignored**, matching `102`, `103`, `104` and `105` exactly, from a clean committed
tree, run by me this session.

I did not re-walk `greedy.rs` and `bitfield.rs`. Files 103 and 104 each audited the surface it
touched, file 105 declined a third walk, and a fourth would be citation theatre. What I did check
fresh at source: the three tautologies are present, unchanged, still in the green total
(`arvo-tensor/tests/capacity.rs:14-18`, asserting `<Dim<3> as Capacity>::CAP == cap(3)` against an impl
whose body is `const CAP: Cap = cap(N)` at `src/capacity.rs:48`; `arvo-tensor/tests/const_capacity.rs:49-53`,
the same shape against `:117`; `arvo-hash/tests/aliases.rs:16-23`, the same `from_raw` literal on both
sides of one `assert_eq!`). Twenty-seven files have now carried the first. The disposition stands at
`95b` as op's own trivial commit outside the panel; I add the count and nothing else.

**Toolchain, and a trap that cost me a nearly-inverted conclusion.** `rustc 1.98.0-nightly (57d06900f
2026-05-27)`, `aarch64-apple-darwin`, resolved from `rust-toolchain.toml` **inside the tree**. Partway
through probe 2 I ran a variant from `/tmp` and got a parse error saying `type const` is not a keyword
sequence at all. It is not: outside the tree rustup resolves to stable `1.94.0`. Had I recorded that
run I would have reported that the permitted feature cannot express the form, which is true at the
second step and false at the first, for entirely unrelated reasons, and I would have reported it with
a verbatim diagnostic attached. `100_probes/OUTCOMES.md` records the identical trap. Twice is a
convention, not a coincidence, and it belongs in the panel's probe conventions rather than being
rediscovered per file. Every probe below ran with `106_probes/` as cwd; commands and verbatim
diagnostics are in `106_probes/OUTCOMES.md`.

---

## 0. The answer, first

**Is file 105's pattern mine, a different one, or two things that look alike?** All three, at different
layers, and separating the layers is the answer.

**The survival mechanism is mine, and file 105 found a pair of quantities I did not.** My file 86
statement is that a wrong subject survives review because at the instantiations everyone reaches for,
the wrong subject and the right one coincide. File 105's four instances all survive by exactly that,
with the coinciding pair being **the routes an author had in mind and the routes that exist**, which
coincide precisely when the route set is a singleton: rank 0, one constructor, one truth type, one
declaration. That observation is worth having and I did not have it.

**The defect is not one defect, and calling it one would license fixing two thirds of it.** Sorted by
what would repair each, at the instance rather than at the description:

| Instance | What is wrong there | Existing owner | New clause |
|---|---|---|---|
| `AGREES` (file 100) | one number carries two names | pricing pillar, `91:113-126` | no |
| bitfield disjointness (file 104) | an obligation was never written | derived-versus-declared, `104` section 3 | no |
| bitfield containment locus (file 104) | a type fact sits at value position | pricing pillar, `91:113-126` | no |
| mutation gap (file 92) | a guarantee's perimeter is understated | perimeter rule, applied by `92:227` | no |
| `Bool`'s six doors (file 103) | one concept has four spellings | two-organs, and D27's own "once" | no, and it is not an instance |

**The fifth is not an instance of the sentence that groups it.** File 105's clause quantifies over "a
fact stated as true of every value of a type", and `Bool` has no such fact, nor do its neighbours
`USize` and `Cap`, all three checked at source. Compiled exhaustively over `Bool`'s whole domain: all
six doors agree at every value, so no route can have been the checked one (`p5`, claim A, const
asserted). Against a type that does carry a fact, the same six doors are not interchangeable and
exactly the write-shaped one breaks it (`p5`, claim B). **Route multiplicity is a defect relative to a
guarantee.** Without one it is a complaint about surface, which is a real complaint with a different
owner and, as section 3.2 shows, a different decision-maker.

**And the residue is not new.** I expected the one survivor of this sort to be a companion sentence
for the pricing pillar, saying that placing a fact on a type is not the same act as forcing it,
because const evaluation is demand-driven. It is a true sentence and probe 4 compiles all four of its
mechanisms. It is also **already in the corpus, at `55:163-165`**, in almost the words file 105
reaches for: "It fires at **use**, not at declaration, because an associated const nothing touches is
not evaluated." Searched this session across all ten consolidations: **zero occurrences, in any of
them.** Section 4.2.

So the honest count of what this stretch's pattern-finding produced is: one real new observation (the
coinciding pair), one grouping that should not survive (the five-way one), and one rediscovery of
material the review wrote at file 55, dropped, rediscovered at file 100 as an "honest scope" note,
confirmed at file 105, and would now adopt at file 106 as though it were new. **That is an archival
failure, not an analytic one, and a clause does not fix it.**

**The largest thing in this file is not about the pattern at all.** The `AGREES` fact only needs
checking because a capacity carries two names for one number, and `91:796-802` calls that pairing
"forced by the language, not chosen". Re-checked on the pin: forced **given the choice to make a
capacity an inductive type-level numeral**, and not otherwise. A capacity whose value is a const
parameter has no pair, no agreement fact, no check and no route question, compiled at rank 3 through
the exact route that leaked (`p3`, exit 0), and the shipped tree already reaches it gate-free. Files 79
and 55 both examined this ground and both kept the numeral; neither considered dropping it. Section 6
states the fork with both columns priced, and does not pick.

---

## 1. Why the two questions collapsed, and what that decides about method

The dispatch asks whether file 105's pattern is mine, and separately asks me to attack three one-pass
files. Three of file 105's four instances come from those three files. So testing whether the four
share a mechanism **is** attacking the one-pass material, provided the test happens at the instances
and not at the four descriptions of them. Testing it at the descriptions would be four unratified
documents agreeing with a fifth, which is the shared-drift shape this workspace names by rule and
which reads, from inside, exactly like corroboration.

So the method below is: go to each instance's own text and its own probe, establish what is wrong
there, and only then ask whether the descriptions describe one thing. Where a claim was checkable I
compiled it. Two of this file's three largest findings surfaced that way from questions that started
as bookkeeping, and neither was reachable from the summaries.

---

## 2. The four instances, at the instance

### 2.1 `AGREES`: two names for one number, and the routes are where that becomes visible

`91:796-802` states the array grammar's agreement is "checked to agree in an inline const block at the
one construction door". File 100 found the sentence false above rank 0: D4's recursion is written
against the trait method, the check lives on the inherent one, and a rank-3 shape whose middle axis
declares a `Nat` of 4 against a literal of 7 has `COUNT == 12` and `size_of(Store) == 21`, both
const-evaluable, disagreeing, nothing raised.

Reproduced at its smallest rather than cited, with a Peano `Nat` so the tower's own complexity does not
ride into the question (`p1`, claim A): `Slot<N3, 7>` gives `LYING_NAT == 3` and `LYING_LEN == 7`, and
nothing raises. File 100's finding holds.

Now the question nobody has asked of it. **A fact that is checked is a fact that could have been
false, and what makes this one able to be false is that `Slot<N, const K: usize>` carries two names for
one number.** The route question is downstream: with one name the routes could not disagree, because
there would be nothing to disagree about. Reading this as a route defect is not wrong; it is one level
too shallow, and the level it skips is the one where the repair is free. Section 6.

### 2.2 The bitfield's two defects, which want two different repairs

File 104 reports two things in one section and file 105's connective sentence lists them as one shape.
At the instance they are not one shape.

**The disjointness gap is an absent obligation.** The shipped macro asserts containment and documents
disjointness as the author's (`arvo/src/bitfield.rs:28-30`, re-read this session: "Overlap detection is
deferred to a future macro version (for now, authors are responsible)"). Nobody predicated the fact of
the wrong thing; nobody predicated it of anything. An absent check has no locus, so it cannot be an
instance of "checked at one route among several". What it is, exactly, is what file 104 says it is and
file 105 sharpened correctly: a **declared** placement owes what a **derived** one proves, and this
obligation was not written.

**The containment locus is route-shaped, and is the pricing pillar's own case.** `_BOUNDS` is declared
at `:377` and mentioned by `let _ = Self::_BOUNDS;` at `:393` and `:399` inside `new` and `from_bits`;
the `ConstDefault` impl at `:370-374` constructs and mentions nothing, re-read at source. Modelled and
compiled (`p4`, claim B): the unmentioning door builds the lying type and compiles, the mentioning one
refuses with `E0080: evaluation panicked: B: disagrees`.

Two defects, two repairs. The first wants an obligation written. The second wants a written obligation
moved off two lines inside two constructors and onto the type. One sentence covering both lets a reader
do the second and believe they did the first.

### 2.3 The mutation gap: an existing rule, applied by hand, working

File 92's finding is that "no raw accessor" undercounts the perimeter, because a public field is a raw
door with no accessor. Genuine, and route-shaped in file 105's sense: a guarantee exists and a route to
values does not preserve it.

What belongs on the record is that file 92 needed no new clause to find it. `92:227` cites
`what-you-can-observe-is-what-you-guaranteed.md`'s own worked example by name and applies it. One
existing rule, one instance, demonstrated rather than asserted, which is the form the dispatch says is
the stronger result.

### 2.4 `Bool`'s six doors: not an instance, and the reason compiles

File 103 finds `Bool` reaching `bool` through six public routes (public field `:261`,
`Transparent::raw` `:264`, `Deref` `:275`, the `Try` exit `:293`, `AsBool` `:328`, `From<Bool> for
bool` `:342`), against D27's own "named **once** and wrapped". All six confirmed at source.

File 105's sentence is "a fact stated as true of every value of a type is checked at exactly one of the
legal routes", and for `Bool` it concludes that "a guarantee stated once and reachable through six
unequal routes is guaranteed over none of the six until each is individually checked."

**There is no fact and no guarantee.** `Bool` is `pub struct Bool(pub bool)` with `TRUE`/`FALSE` consts
and a `From<bool>`; nothing validates anything, because `bool`'s validity is the language's. Its two
neighbours are the same: `USize(pub usize)` at `:45`, and `Cap(pub USize)` at `:73`, whose doc comment
states its own job in its own words, "the type prevents mixing capacities with unrelated integers",
which is a naming job. The one closed type in the file, `NUSize` at `:485`, is the one carrying a
documented invariant, which is the pattern holding rather than breaking.

Compiled, because the claim is exhaustively checkable and a checkable claim should be settled rather
than argued (`p5`, claim A): six doors modelled on the shipped ones, over the whole two-element domain,
all agreeing, asserted in const position so the agreement is a compile-time fact rather than a test
someone has to run. **Nothing separates, at any instantiation, ever.** And the contrast that shows the
sentence with a subject (`p5`, claim B): the same six doors on a type carrying "the inner byte is never
zero", where five are reads that preserve it trivially and the public field is a write that breaks it
with no `unsafe` and no diagnostic, the positive half exhaustive over all 256 inputs through the
establishing route.

So the discriminator is mechanical. **Route multiplicity is a defect relative to a guarantee.** With
one, exactly the write-shaped doors are the problem and a sweep names them. Without one, six doors is a
redundancy in the surface. File 103's complaint is legitimate and section 3.2 says on what ground; it
is not this ground, and including it makes the group's own sentence false at a fifth of its evidence.

---

## 3. Attacking the one-pass material

### 3.1 File 103's fork survives, and its lean survives by file 105's route rather than its own

File 103 prices the fork between the tower's contracts naming `Bool` concretely and naming a truth
contract, finds both cheap, finds branch B free at runtime by symbol identity (`_run_b1 = _run_a`,
which is stronger than a measurement and to which I have nothing to add), and leans to B on the ground
that a second truth type already ships.

The lean is right. Its stated ground has a method problem, and file 105 repaired it without saying so.

**The method problem.** "Is there a second truth type? There is, and it is shipped" answers a design
question with a fact about the tree. Delete the shipped-source citation, which is this dispatch's own
test, and nothing is left: `MaskOps`'s existence in `arvo-mask-contracts` is the only evidence offered
that the design wants two truth types.

**File 105's repair, presented as a sharpening.** Boolean algebras are an equationally axiomatised
class, varieties are closed under direct products, so `Bool^W` is a Boolean algebra by a theorem. That
argument cites nothing in the tree: a truth contract whose shape is Boolean-algebra-correct **has**
finite products as instances whether or not one currently ships. The lean survives deleting the
citation, by that route, and `MaskOps` demotes from the argument to a witness that the design has
already reached for the instance once, which is the right weight for a tree fact and exactly the
why-evidence register the method constraint licenses.

I raise it because file 105 offered an algebra as a strengthening of a sound argument, and it is
actually a replacement for an unsound one. The difference matters if op reads the two files in order
and takes the first ground as the reason.

### 3.2 One citation in file 103 should be struck, and striking it changes whose call the finding is

`103:198-201` supports the six-doors finding this way: "the perimeter rule the review adopted at file
10 says why in general terms: a guarantee about a type holds only over the operations through which
the type can be observed, and every public field is a hole in the guarantee rather than a stylistic
matter."

The rule says the opposite at the case in hand, in its own Boundary section, quoted because the wording
is load-bearing:

> This is not "make everything private". A type with no invariant to protect loses nothing by exposing
> its fields, and plain data should stay plain. [...] The rule fires when a type's argument for
> correctness depends on values of it having a property. Then the perimeter is part of the argument,
> and leaving it open means the argument does not hold.

`Bool`, `USize` and `Cap` have no invariant, and section 2.4 compiles the consequence. The rule's
antecedent is unsatisfied at all three, so the citation is decorative, and a decorative citation is
worse than none because it makes a taste question look settled.

**This is not pedantry about a footnote, because the two grounds put the call in different hands.**
Under D27's own "named once and wrapped", the right number of doors is one because a name should have
one spelling, which is a vocabulary decision and is op's, exactly as file 103 says three paragraphs
later. Under the perimeter rule it would be one because a guarantee needs a closed perimeter, which is
a soundness question and is nobody's to trade. Carrying both converts a question op should answer into
one he cannot, which is the policer posture arriving from the direction nobody watches: not refusing a
consumer's choice, but removing from the record that it was a choice.

**What survives is the whole finding minus one sentence.** `Bool` has six spellings of one projection
where D27 says one, and by the two-organs-for-one-fact rule that `102` records having paid for three
times, four of them are three too many. File 103's own claim, that the number is one and the chapter
states which, is untouched. Only the support changes, from a perimeter argument to a redundancy
argument, which is the argument it was making all along.

### 3.3 File 104's classification survives, and its named attack surface has a mechanical answer

File 104's convergent statement (a bitfield is a heterogeneous product of numerals under a declared
placement map; a bitpacked column is the homogeneous product of one under a derived map; the two axes
are independent) survives, and file 105 gave it an independent route through the dependent product. I
confirm both and add nothing to either.

Its named attack surface closes with a compiled fact rather than an epistemic one. File 104 asks
whether "derived versus declared" is a real axis or "a declared one with a cheap author", since
`Layout::Dense`'s stride is derived and nobody calls it a placement map. File 105 answers on the burden
of proof: derived is provable-by-construction once, declared is provable-by-checking per instance. That
is right and it is the deeper answer.

The mechanical half, from probe 4, is that the axis cashes out in **which forcing mechanism exists**. A
derived placement's well-formedness sits in a type position no route can avoid (`p4`, claim E: the
array length is the fact, so every route resolves it). A declared placement's sits in no type position
at all, so it needs an emitted free const item (`p4`, claim C, which fires with no route, no mention
and no construction) or it needs mentioning at a route (`p4`, claim B, the shipped bitfield, which
leaks through `ConstDefault`). The same axis, visible in emitted code rather than only in where the
burden lives. That is a real axis by any test.

### 3.4 File 105's `Truth` refinement is my own pattern arriving at file 103's fork, and neither file says so

File 105 proposes splitting the truth contract along the seam the algebra has: a Boolean-algebra core
that `Bool` and `Mask<W>` satisfy uniformly, and a separate exit obligation that `Bool` satisfies by
identity and a mask satisfies only through a named reduction, never a default. The reasoning is right
and the toolbox-rule grounding for the reduction being a consumer choice is right.

Neither file names the shape of it. **The tower needs a truth value it can branch on. The algebra file
105 cites is closed under products. Those two subjects coincide at `W = 1` and separate above it, and
`W = 1` is the only truth type the tower has.** All 92 `-> Bool` signatures file 103 counted are
one-lane. So the fork was posed about the first subject while the tower's whole surface needs the
second, and the reason nobody noticed is that at the design's only truth type the two are the same
thing.

That is my file 86 pattern, at the fork, produced by file 105's own argument and not named as such. It
also means the separation requirement was owed on file 103's model and was not run, which is now the
fourth time this stretch a standing requirement has been found working and unrun.

The consequence for the fork is small and worth stating so op is not surprised later, in file 103's own
register: **branch B with a split contract costs nothing at `Bool`**, because the reduction is the
identity and a blanket supplies it, which is file 105's own reading and which I checked against the
signature shape rather than assuming. What changes is that contracts generic over the algebra core do
not thereby become usable in `if` position, so the fifteen declarations file 103 counted take the
narrower branchable bound rather than the algebra one. Still one type parameter and one bound; a
different bound from the one the fork's statement implies.

---

## 4. The material this stretch was rediscovering, and where it went

This is the section I did not expect to write, and it is the one I would put in front of op first.

### 4.1 The clause naming the survival mechanism was absorbed at the ninth and dropped by the tenth

File 104 section 4.2 raises `83:290-316`'s level-naming clause ("a const derived from a width names the
level it is a function of") and reports: "**And the clause is not in the consolidation.** Fresh search,
`grep -rn "names the level\|names its level\|derived from a width" *.md`, 2026-08-05 06:53: exactly two
files, `83` where it is offered, and `84` where it is cited. Neither the ninth nor the tenth
consolidation carries it, by name or in general form." It offers the disposition to the next
consolidation.

It is in the ninth, inside the pricing pillar, `91:118-122`, in general form:

> If so it belongs on the type as an associated const, not a `const fn` called from value position,
> and it names the width **level** it is a function of, because two levels coinciding at the one preset
> everyone measures is exactly how a compile-time fact computed from the wrong level survives review
> (`83:290-316`, [...]).

File 104's grep could not find it, and I reproduced its three patterns this session, 2026-08-05 07:32:
they return `83`, `84` and `104` itself, nothing else. The ninth writes "names the width **level**",
with an intervening word and bold markers, so "names the level" does not match. That is precisely the
honest limit file 104 wrote for itself arriving one file later, and it is the cheapest possible
instance of it.

**The disposition is not the one file 104 expected. The clause was absorbed at the ninth and dropped at
the tenth, with no droplist entry.** `grep -c "two levels coinciding"` returns 1 for
`91_consolidation_nine.md`, 0 for `102_consolidation_ten.md`, 0 for `78_consolidation_eight.md`,
2026-08-05 07:32. The tenth's own pricing-pillar paragraph opens "**The pricing pillar.** Unchanged in
statement" (`102:90`) and then restates the statement without the clause the ninth had added to it.
The "unchanged in statement" sentence is what makes the loss invisible, because it tells the reader not
to diff.

The clause's own reason, in the ninth's words, is "two levels coinciding at the one preset everyone
measures is exactly how a compile-time fact computed from the wrong level survives review." That is my
file 86 sentence with `level` for `subject`, absorbed and then dropped, two documents before a stretch
spent four files rediscovering a version of it.

### 4.2 The mechanism sentence was written at file 55 and has never been in any consolidation

This one is sharper, and it is the reason section 7 shrank.

`55:163-165`, sixty files before file 100 found the `AGREES` gap:

> It fires at **use**, not at declaration, because an associated const nothing touches is not
> evaluated. A `Capacity` impl whose two spellings disagree survives until someone folds with it.

File 55 also names the pair (`Dim<const N: usize, P: Pos>`), names the risk ("the decorrelation risk
the review already names elsewhere"), cites my own `48_probes/probe_2:64-66` as an earlier instance,
names the mitigation `AGREES`, and reproduces its `E0080` verbatim rather than assuming it fires. Every
structural element of file 100's finding and file 105's clause is in that one paragraph.

Searched this session, 2026-08-05 07:41: `grep -rn "nothing touches\|not evaluated\|fires at use\|never
evaluated\|unreferenced" *.md` returns exactly two files across the whole panel, `55:164` and
`105:246`. Across all ten consolidations, `grep -c "not evaluated\|nothing touches\|fires at use\|AGREES"`
returns 0 for the second through the ninth and 4 for the tenth, all four of which are `AGREES` from
section 1.28's repair rather than the mechanism.

**So the sequence, stated plainly.** File 55 produces the mechanism and the mitigation. Ten
consolidations carry neither. File 100, at rank 3, rediscovers the mechanism as an "honest scope" note
appended to its own repair. File 105 confirms it ("exactly what an unreferenced associated const
predicts") and proposes a clause built on it. And this dispatch, arriving fourth, was going to offer it
as the one genuinely new residue of the sort. Four productions, zero absorptions, fifty files apart.

I want to be careful about the register here, because this reads like an indictment and I do not think
it is one. Every step was honest. File 55 stated it where it was relevant and moved on; consolidations
compress and a paragraph inside a two-honest-costs aside is exactly what compression drops; file 100
found it independently and said so; file 105 found the general shape and offered it as a suggestion
owed a second read. Nobody did anything wrong and the material was still lost three times.

**Which is the finding.** A defect nobody commits and everybody pays for is a defect in the mechanism,
and the mechanism here is compression. The tenth consolidation's droplist has eight entries and every
one is a claim that turned out false; not one is a sentence that was quietly not carried forward. The
droplist is instrumented for **error** and not for **loss**, and loss is what this stretch was
fighting.

### 4.3 What that suggests, and it is a discipline rather than a rule

`102` already performs a table-diff obligation on itself, by its author, before it stands. The four
rules and the three requirements are the only prose in a consolidation that gets restated verbatim
every time, which makes them the only prose that can lose a sentence without any table changing.

**Suggested, one line in the consolidation discipline rather than a new design rule:** *when a rule or
requirement's statement is compressed, either the compression is checked to entail the prior text, or
the difference is a droplist entry.* That is the table-diff obligation extended to the seven pieces of
prose that need it most, and it is the same shape as the moment clause `95b` adopted for the two
requirements: not a new rule, an existing obligation given a place where it fires.

Against this stretch's own record it would have caught section 4.1 outright, at the moment the tenth
consolidation wrote "unchanged in statement" over a statement that had changed.

---

## 5. My own file 34, checked, because the dispatch says to and because it is the same shape

Three claims from file 34 were overturned. I read the overturns at their own text rather than at the
droplist:

- `40:777-779`: file 34's passing sentence that the shipped width chain already satisfies
  value-uniqueness. False for the width chain: `UInt<UTerm, B0>` is a second spelling of zero.
- `40:786-788` and `37`: file 34's ordered three-relation ladder (section 3.3), replaced outright by
  file 37's nine-point view lattice, which is not a chain and contains two shipped presets at
  incomparable points.
- `40:790-793` and `37:244-262`: file 34's reification-stability generalisation, that the graded
  relation is the only one stable under a `Refuse`-to-special reification. True of one reifier and
  false in general; under an out-of-set absorbing special Kleene is stable too, and under
  `SubstituteZero` nothing is.

The second and third are the failure I am adjudicating in this file, committed by me. Both generalised
from the witnesses I had built to a claim about all of them, and both were overturned by someone who
built a witness I had not: file 37 tested two reifiers where I tested one. The third's correction is
almost word for word the separation requirement, two stretches before I proposed it, applied to me.

And file 55 cites `48_probes/probe_2:64-66`, my own, as a prior instance of the very decorrelation risk
section 4.2 traces. So my files sit at both ends of that chain: an early instance at 48, and at 106 the
file that nearly proposed its own already-written conclusion as new.

I record this because the dispatch asks whether a pattern is mine, and the honest answer includes that
the mechanism has caught me twice on the record and nearly a third time in this dispatch. It is also
why section 2 goes to each instance's own probe rather than its file's summary: a summary is where a
one-witness generalisation stops looking like one.

---

## 6. The capacity pair, and a fork the design has never priced

This section started as the bookkeeping question in section 2.1 and ended up the largest thing here.

### 6.1 What is claimed, and what re-checking confirms

The ratified sentence, from file 79 section 4: "The array grammar is a paired, non-derived fact, forced
by the language, not chosen. No expression of `[T; K]` computed from a type-level `Nat` exists under
the permitted feature set (the naive form and its rustc-suggested successor both refuse, citing
`generic_const_exprs` and, past that, an inductive step `min_generic_const_args` cannot express
either)."

Both halves re-checked on the pin, because a claim about a toolchain is dated by construction and this
one is load-bearing for a repair.

**The naive form: confirmed, and the feature is the forbidden one** (`p1`, claim B):

```
error: generic parameters may not be used in const operations
102 |         type Array<T: Copy> = [T; <N as Nat>::VAL];
    = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

**The successor: confirmed** (`p2`). With `min_generic_const_args`, which the workspace allows, a plain
associated const in array position is refused with rustc naming the fix (`use of const in the type
system not defined as type const`). The `type const` form then parses and the array position resolves.
What refuses is the inductive step, `type const VAL: usize = P::VAL + 1`, first as `complex const
arguments must be placed inside of a const block` and then, with the block supplied, as `generic
parameters may not be used in const operations [...] help: add #![feature(generic_const_args)]`.

**This is a reproduction, not a new result, and I overstated it in my own draft before checking.**
`79:154-157` already names the inductive step by its expression (`2 * P::VAL`) and already cites
`76_probes/OUTCOMES.md` part A for the `type const` attempt, and `25_xu_building_the_exact_product.md:54-110`
worked out the `type const` mechanism sixty files further back with the same `E0658`-adjacent
diagnostic. What probe 2 adds is a re-run on the current pin of a dated claim that a repair now leans
on. That is worth having and it is all it is.

### 6.2 The pairing is a consequence of a choice, and this part is new

`p3` compiles clean, exit 0. A capacity whose value is a `const N: usize` supplies `type const VAL:
usize = N`, which is a **path** rather than an expression, so `[T; <Self as Nat>::VAL]` resolves under
`min_generic_const_args` alone. There is no second name, so:

- `Dim<3>` has length 3 and value 3 because they are the same const read twice.
- `Axis<Dim<3>, Axis<Dim<4>, Axis<Dim<5>, Scalar>>>` has `COUNT == 60` and `size_of(Store) == 60`,
  asserted equal **through the trait route**, with no `AGREES`, no inline const block and no
  construction door anywhere in the program. Extents 3, 4, 5 rather than cubic, so a transposed count
  would show.
- The bare-const-read leak `100_probes/probe_2` claim C flags as still open after the repair closes
  too, because there is nothing to leak.

The shipped tree reaches the same place with no feature gate at all: `arvo-tensor/src/capacity.rs:44-48`
declares `Dim<const N: usize>` with `type Array<T> = [T; N]` and `const CAP: Cap = cap(N)`, both from
one `N`, and `arvo-tensor/src/lib.rs:21` carries only `const_trait_impl`. Read as a factual check that
the unpaired shape is buildable; the design claim is the compile, not the tree.

**So the ratified sentence is half right, and the half it gets wrong is the half that decides the
repair.** Given an inductive type-level numeral the pair is forced. The inductive numeral forces it,
and that is a design choice with a stated benefit rather than a language constraint.

Fresh search for whether anyone has said this, 2026-08-05 07:41: the panel's discussion of the pairing
is at `79:148-172`, `82:218` and `:286`, `91:796`, `100:173-176` and `55:150-165`, and every one of them
takes the numeral as given and asks what the array length costs beside it. `55` states "the const
generic cannot be removed" and keeps **both** spellings; `79` makes the `Nat` primary and the literal a
companion. **Nobody has considered making the const primary and dropping the numeral**, which is what
`p3` compiles.

### 6.3 The fork, both columns, and I am not picking

File 79 section 3 states the benefit plainly: `Capacity: Nat` gives "one seal, one ordering, one
arithmetic, inherited wholesale", and closes "the two-encodings finding from the seventh consolidation
at the value layer completely".

| | capacity as an inductive `Nat` | capacity as a const parameter |
|---|---|---|
| names for the number | two, paired | one |
| agreement fact | exists, needs checking at every route | does not exist |
| `AGREES`, its two-half repair, its reachability scope | needed | not needed |
| ordering, `Cmp`, `Gcd`, value-uniqueness | inherited from the tower | available as const comparisons, not inherited |
| seal | the tower's | a sealed trait with one impl |
| feature cost | none for the pair itself | none in the shipped form; `min_generic_const_args` if the `Nat` indirection is wanted |
| shares one encoding with the numeral tower's exponents | yes | no |

The last row is the genuine cost and is why I am not recommending a side. The numeral tower needs
type-level arithmetic producing **types**, and a const parameter does not participate in it. Whether a
capacity must be a member of that vocabulary, or is a different kind of number that merely reads like
one, is exactly what `79` answered and it answered it for a stated reason.

**What I will say, because it is a fact about the record rather than a preference.** The decision that
closed a two-encodings finding introduced a second name for one number one level down, and the `AGREES`
gap, its two-half repair, and its reachability scope are all downstream of that. That is a fix
relocating a defect rather than removing it, and this review has a name for the shape. `79`
acknowledged the pairing and argued it forced; the argument is right about the mechanism and imprecise
about the cause, and the imprecision is why nobody has looked at the other column.

**Owed, and it is one dispatch:** whether the tower's `Cmp`/`Gcd`/ordering are reachable from a
const-parameter capacity at acceptable cost, and whether any consumer needs a capacity inside
type-level arithmetic that produces a type. If neither, the pair goes and three findings go with it. If
either, the pair stays and file 100's repair is right as stated and should land in both halves.

---

## 7. What the design should say, in a form the next consolidation could take

Three sentences and one discipline line, and every one of them is a restoration or an existing rule
applied, rather than an addition.

**On the pricing pillar, restoring what the tenth dropped.** *A quantity that is a function of the
type's parameters alone belongs on the type as an associated const rather than computed at value
position, and it names the level or subject it is a function of, because two subjects coinciding at the
instantiation everyone builds is exactly how a fact predicated of the wrong one survives review.* This
is `91:118-122` in its general form; restoring it is a correction, not an addition.

**On the pricing pillar, the companion the corpus has and the consolidations do not.** *Placing a fact
on the type is necessary and not sufficient, because const evaluation is demand-driven: an associated
const nothing mentions never runs. Where the design owns the declaration site, as a macro does, the
fact is emitted as a free const item and fires with no route at all. Where the consumer instantiates a
generic and the design owns no declaration site, the fact must sit in a position every route resolves,
which in practice means the associated item the routes already consume; a fact reachable only through a
mention is guaranteed only through that mention.* The first clause is `55:163-165`, absorbed at last;
the two-mechanism split is compiled at `p4` claims C and D and is what section 3.3 makes the
derived-versus-declared axis cash out in.

**On the perimeter rule, its own scope restated where the design keeps reaching past it.** *The
perimeter rule fires on a type whose correctness argument depends on values of it having a property. A
type with several public routes and no such property has a redundancy, governed by the
two-organs-for-one-fact rule, and the number of spellings a name gets is a vocabulary decision rather
than a soundness one.* This is the workspace rule's own Boundary section, needing restatement in the
design's text only because two files have now cited the rule past it.

**On the capacity resolution, one word, and then a fork.** *The array grammar's pairing is forced by the
choice of an inductive type-level numeral, not by the language.* Then section 6.3's two columns,
unresolved, on the open list.

**And the consolidation discipline line from section 4.3**, which is the only one of the five that
would have prevented this file from being necessary.

**No fifth rule and no new clause on the route question.** The three instances file 105's clause would
govern are governed better by the pricing pillar, which names the repair where the clause names the
symptom; the fourth is two-organs; and the survival mechanism is the separation requirement, which
would have caught three of the four in advance and was not run. `102:110-117` establishes the precedent
in its own words: "The correct response to a requirement that works but goes unrun is a moment naming
when it runs, not a new requirement." File 105 cites that precedent and then proposes a clause anyway.
I think the precedent decides against its own citation, and I say so as a disagreement with a file
whose four confirmations I have no quarrel with at all.

---

## 8. What this file does not decide

The capacity fork of section 6.3. Both columns are on record with what each buys, the second column's
cost is real, and whether a capacity belongs in the numeral tower's own vocabulary is what `79`
answered and is op's to reopen or not.

Which of `Bool`'s four redundant spellings survives. File 103 puts it with op, correctly, and section
3.2 only moves the ground from soundness to vocabulary, which if anything makes it more op's.

Whether file 105's candidate clause should exist. I have argued it should not and named which existing
rule covers each instance, but a clause the lead designer wants is a clause, and my argument is that it
is redundant rather than that it is wrong.

Owed artifacts, each with what closes it:

- **The capacity fork's cost column.** *Artifact:* one dispatch establishing whether any consumer needs
  a capacity inside type-level arithmetic that produces a type, and whether `Cmp`/`Gcd`/ordering are
  reachable from a const-parameter capacity. This decides section 6.3 and I have deliberately not
  guessed it.
- **A second read on section 2.4's discriminator**, that route multiplicity is a defect only relative to
  a guarantee. One pass, mine, compiled at two shapes. The attack surface I would point a second reader
  at: whether a **naming** guarantee ("this type is the one name for that primitive") counts as a
  guarantee for the purpose, which would put `Bool` back in the group by a different door than the one
  file 105 used.
- **The two bitfield overlap tests** file 104 named, unchanged and still owed, in `mock/crates`, which is
  op's boundary.
- **The `AGREES` repair's second half**, if section 6.3 lands on the inductive column. On the other
  column the repair is moot, which is the reason the fork is worth an hour.
- **The toolchain trap as a standing probe convention**, having now cost two files a near-miss in
  opposite directions. *Artifact:* one line wherever the panel's probe conventions land.

---

## 9. The three requirements, performed on this text before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions.
*Survival mechanism* (sections 0, 2): defined at first use as the reason a defect passes review, as
distinct from the *defect* and from the *repair*, which is the three-way split section 0's table is
organised on and is the file's whole argument, so it is defined where it first carries weight.
*Forcing mechanism* (sections 3.3, 7): defined at first use as the syntactic construct causing a
const-position fact to be evaluated, enumerated at four members in `p4`, distinguished from
*placement*, which is where the fact is written. *Unpaired capacity* (section 6): defined at first use
as one whose value and array length are the same const read twice. *Loss*, as opposed to *error*
(section 4.2): defined there as a sentence not carried forward, distinguished from a claim that turned
out false, which is what the droplist is instrumented for. *Naming guarantee* (section 8): named open
rather than defined, because defining it would decide the second read it is owed. Terms used from the
record without redefinition: the pricing pillar, the perimeter rule, the two-organs defect, the
separation requirement, the definitional-completeness line, the table-diff obligation, the toolbox
rule, the layer-keying rule, `Capacity`, `Nat`, derived and declared placement, the three width levels,
rung and risk class. No term in this file's own new prose is left undefined or uncited.

**The separation requirement, performed.** Two models are this file's own and the requirement bites on
both.

The first is section 2.4's discriminator, separating *route multiplicity with a guarantee* from *route
multiplicity without one*. **Nonvacuous at exactly the pair `p5` compiles**: on a type carrying "the
inner byte is never zero" the six doors sort into five reads and one write and the sweep names which
breaks it; on a type carrying nothing the identical six doors agree at every value over the whole
domain and no sweep can name anything. **Where it is vacuous I say so:** at a type with exactly one
public route the two columns collapse and the distinction says nothing, which is why the level-ordering
refusal at `91:560-561` never needed it and why the shipped `Dim<const N>` does not either.

The second is section 7's split between *placing* a fact and *forcing* it. **Nonvacuous at exactly the
macro-declared versus consumer-instantiated pair**: a free const item is available to the first and
structurally unavailable to the second (`p4`, claims C and D), so "put it on the type" names two
different acts with two different guarantees depending on which kind of type it is said about. **Where
it is vacuous I say so:** where the fact already sits in a type position every route must resolve
(`p4`, claim E, an array length), placing and forcing are one act, and section 6's argument for the
unpaired capacity rests on precisely that collapse rather than on the distinction.

**The freshly-performed-search requirement, performed.** Every universally quantified negative above
carries its own search, run this session, quoted with its date. Two of my own draft's negatives failed
their check and were narrowed rather than shipped; both are recorded here as run rather than as
concluded.

- "The level-naming clause is in the ninth consolidation and not the tenth": `grep -rn "names the width"
  *.md`, 2026-08-05 07:32, one hit (`91:119`); `grep -c "two levels coinciding"` over the eighth, ninth
  and tenth consolidations, same time, returning 0, 1, 0. File 104's own three-pattern grep reproduced
  at the same time, returning `83`, `84` and `104` only.
- "The demand-driven const-eval sentence is in no consolidation": `grep -rn "nothing touches\|not
  evaluated\|fires at use\|never evaluated\|unreferenced" *.md`, 2026-08-05 07:41, two files across the
  whole panel (`55:164`, `105:246`); and `grep -c "not evaluated\|nothing touches\|fires at use\|AGREES"`
  over all nine numbered consolidations, same time, returning 0 for the second through ninth and 4 for
  the tenth, every one of the four being `AGREES` from section 1.28 rather than the mechanism.
- "Nobody has considered dropping the numeral and keeping the const": the pairing discussion is at
  `79:148-172`, `82:218,286`, `91:796`, `100:173-176`, `55:150-165`, located by `grep -rn "forced by the
  language" *.md` plus `grep -rn "Dim<" *.md` plus `grep -rn "const N: usize" *.md`, all 2026-08-05
  07:39 and 07:41, then each hit read. My draft's version of this claim was "`forced by the language`
  returns `79`, `91` and `102`", which is false: it returns `79`, `82` twice, `91` and `100`, and not
  `102` at all. Narrowed to the claim the reads support.
- "No panel file tested `min_generic_const_args` for this question": my draft claimed this and it is
  false. `grep -rln "min_generic_const_args" *.md` returns twenty-five files, 2026-08-05 07:39, and
  `79:154-157` plus `76_probes/OUTCOMES.md` part A plus `25:54-110` all test exactly the `type const`
  form. Section 6.1 was rewritten from "sharper than 79 recorded" to "a reproduction on the current
  pin", which is what it is.
- "`Bool`, `USize` and `Cap` carry no per-value invariant": checked at source rather than by grep,
  because the claim is about absence in a definition and a grep for absence is not evidence.
  `arvo-storage/src/platform.rs:45,60-77,255-345` read in full; the only validating constructor in the
  file is `NUSize`'s at `:485`, the one closed type and the one carrying a documented invariant.
- "No panel file connects file 103's six doors to a guarantee's absence": `grep -rn "no invariant\|has
  no invariant" *.md`, 2026-08-05 07:39, empty across the panel directory.

The honest limit, inherited from files 97, 98, 101, 103, 104 and 105 and now stated by seven files
running: these performances verify that this file's terms are placed, its models have content, and its
negatives were searched with my vocabulary. Sections 4.1 and 4.2 are this stretch's second and third
demonstrations that a grep's **vocabulary** is the thing that fails, so the limit is not boilerplate
here, and two of my own negatives above failed it before this file stood. A second reader with
different terms is the check on mine.

---

## 10. Standing

File 105's pattern is my pattern's survival mechanism with a pair of coinciding quantities I did not
find, and that observation is a real addition: nobody had noticed that the routes an author has in mind
and the routes that exist coincide exactly when the route set is a singleton, which is rank 0, one
constructor, one truth type, one declaration.

Its defect is three defects. One number with two names, an obligation never written, and a fact written
at value position want three different repairs, two of which the pricing pillar states better than the
proposed clause does. Its fourth instance is not an instance: a guarantee is what makes route
multiplicity a defect, and `Bool` has none, compiled exhaustively over its whole domain.

So the design gains no clause from the grouping, which is the shrinking answer this review keeps
finding when it looks properly. What it gains is three restorations and one discipline line, and the
discipline line is the one that matters, because the mechanism sentence file 105's clause is built on
was written at file 55 and has never appeared in a consolidation, and the general clause naming the
survival mechanism was carried by the ninth and dropped by the tenth under a heading saying nothing had
changed. Four productions of one body of material, fifty files apart, zero absorptions. **The failure
this stretch has been fighting is archival rather than analytic, and a clause is the one repair that
cannot fix it.**

And the largest thing here is not about patterns. The `AGREES` fact exists because a capacity carries
two names for one number; the ratified sentence says the language forces that, and the language forces
it only given a choice the design made for a stated reason; the unpaired form compiles, gate-permitted,
at rank 3, through the route that leaked. Taking it is a fork with a real cost in the other column, and
it is op's, with both columns on the record and neither picked here.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (`91:113-126` the pricing pillar including the clause section 4.1 finds dropped,
`91:560-561`, `91:780-802` the capacity resolution, `102:90-95` and `102:110-117`, the workspace rules
`what-you-can-observe-is-what-you-guaranteed.md` in full and `unstable-features.md`'s forbidden and
allowed tables, the persona-tier `95b`/`101b` as marked), settled shapes (`25:54-110`, `55:145-170`
which is section 4.2's whole subject, `79` sections 3 and 4, `83:290-316`, `86:240-330` mine,
`92` section 2.1 and `:227`, `100` section 2.2, `103` sections 1.3, 1.5 and 3, `104` sections 2.1, 3.1,
3.2 and 4.2, `105` sections 4.1, 4.2 and 6, each read at its own text), compiled (`106_probes/p1`
through `p5`, all at the pin from inside the tree, commands and verbatim diagnostics in
`106_probes/OUTCOMES.md`; `100_probes/probe_2` and `104_probes/p1` re-read at source rather than at
their prose summaries), verified at source
(`arvo-storage/src/platform.rs:45,73,261,264,275,285-291,293,328,342,485`,
`arvo-tensor/src/capacity.rs:19-58`, `arvo-tensor/src/lib.rs:21`,
`arvo/src/bitfield.rs:28-30,370-374,377,393,399`, the three tautologies, HEAD `8a92eb4`), overturned
(my own `34:176-190` and `34` section 3.3, at `37:244-262` and `40:786-793`, section 5; and two of this
file's own draft negatives, section 9), reasoned (the three-way sort in section 0, the struck citation
in section 3.2, the pattern-at-the-fork reading in section 3.4, the archival-loss reading and its
discipline line in section 4, and the fork's framing in section 6.3, all mine, all one pass, offered as
suggestion and not as a ruling).*
