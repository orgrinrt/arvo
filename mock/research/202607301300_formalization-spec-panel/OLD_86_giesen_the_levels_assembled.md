# 86. The levels assembled: a compiled collision between two one-pass proposals, the carve-out and the totality as one fact, and what the stretch's pattern actually is

Fabian Giesen, file 86. I wrote file 34 (the three halves assembled), three of whose claims were
overturned by members who recompiled them, and I carry those corrections rather than defend the
originals; file 48 (the stretch assembled); and file 72 (the unexamined ground). One correction to
my own file 72 lands in this file's section 1.3: its "an invertible external image takes the
crossing contract's statement structure verbatim" was written when that structure had two
statements, and the structure now has three.

**What I read.** `78_consolidation_eight.md` in full, the standing base. The deliverables since, in
order, each in full: `79_dolan_what_capacity_is.md`, `79b_op_the_verification_mandate.md`,
`80_leroy_the_verification_bundle.md`, `81_fog_is_the_bitpack_cost_inherent.md`,
`82_pesce_the_stretch_assembled.md`, `82b_op_checkpoint_twenty.md`, `83_lattner_how_many_widths.md`,
`84_leijen_failure_that_is_not_a_range_event.md`, `85_chlipala_the_closure_audit.md`. One `ls` of
the panel directory, current through `85_probes`. Behind the consolidation, with licence since each
is a derivation this file's collisions sit on: `80:90-112` (the quantifier hardening in the words
that produced it), `84:354-411` and `84:415-463` (the biased niche and the typed quantum),
`83:131-205` (statement C and its discharge argument), `50:294-307` via file 84's quotation
(checked against file 84's citation, not re-derived), and the workspace rule
`what-you-can-observe-is-what-you-guaranteed.md`, read fresh because section 1.3 leans on it. The
shipped tree I touched for nothing beyond the standing canon-gate greps; no claim below reads it
for meaning, and every conclusion survives deleting every tree citation, there being none.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty.
Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary from every
`test result:` line by a stated command, **666 passed, 0 failed, 9 ignored**, matching files 81
through 85, from a clean tree at HEAD (`83303ab`). The one disqualifying test on record,
`arvo-tensor/tests/capacity.rs:14-18`, three tautological lines, stands exactly as `78:874-876` and
files 82 through 85 carry it: flagged for deletion, not improvement, outside the panel's scope to
touch. Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved
from `rust-toolchain.toml`, confirmed inside the tree this session. **The bench harness was not
run**; its orchestrator overwrites committed artifacts (`81:38-44`), nothing below is a runtime
performance claim, and every number I cite from files 81 and `82b` is cited, not re-measured.

**What is compiled and what is reasoned.** Sections 1.1, 1.2 and 1.4 trace to `86_probes/` (three
probes, commands and outputs in `86_probes/OUTCOMES.md`), written, compiled and where applicable
run fresh this session on the pinned toolchain inside the tree; probe 2's run is a `-O` build on
this host and its counts are asserted in the program, not read off a transcript. Section 1.5's
counts each name the command that produced them, per the discipline `82b` adopted. Sections 1.3, 2
and 3 are reasoned and say so per claim. Everything is a suggestion; the calls are op's.

---

## 1. The one-pass proposals, attacked

The audit at file 85 second-read four things (file 83's levels, file 84's section 2, file 82's
fold rebuild, file 79's search claim and closure argument). Those are twice-read and I do not
reopen them. What follows is the set nobody has read: file 80's own new spec sentences (its
`Crosses` second read was of file 67's mechanism; the quantifier sentence is file 80's own,
one-pass), file 84's sections 5 and 6, file 83's statement C (file 85's "found no counter-argument"
is a reading, not a derivation, and the discharge argument deserved one), and file 85 itself.

### 1.1 File 80's quantifier hardening and file 84's biased niche are inconsistent as written, compiled

**The two sentences.** File 80, closing the perimeter of statement 0: "statement 0 quantifies over
every bit pattern of `Encoding::Fields`' width; an encoding whose decode is partial on that set
does not satisfy `Crosses`, and partiality is expressed by shrinking the fields, not by a domain
side-condition" (`80:99-102`, carried into its consolidation-ready section at `80:431-432`, and
already adopted downstream at `82:296-303` and leaned on by `84:246-249` as kind 3's closure). File
84, four files later: store the datum biased by one in a `core::num::NonZero`, spending the zero
pattern, so `Option<T>` and the whole refusing tier are the same width as `T` (`84:354-383`,
compiled at probe_6 over its whole domain).

**The collision, compiled** (`86_probes/probe_1_niche_vs_statement0.rs`, zero gates, every
assertion in const position). The biased carrier's decode is partial on exactly one bit pattern of
the 16-bit fields' width: `NonZeroU16::new(0)` is `None`, asserted. And no field shrink expresses
that domain: a width-shaped domain has exactly `2^w` members, and `2^w != 65535` is asserted at
every `w` in 0..=16, not argued from arithmetic. So under file 80's sentence as written, the
biased-niche lowering **does not satisfy `Crosses`**: its partiality is real, it is exactly the
partiality the construction exists to create, and it is inexpressible by shrinking the fields,
because a niche-shaped domain (`2^w - k` for small `k`) is never a power of two. Either the
refusing tier's layout win cannot cross to the byte image at all, which is a cost nobody priced and
file 84 nowhere accepts, or the quantifier sentence is too strong. Neither author could have seen
it: file 80 landed before file 84 existed.

**The resolution, offered.** File 80's sentence was built to close a perimeter: `repr(transparent)`
makes every pattern reachable, so an unenforced domain side-condition quantifies the guarantee over
less than what can be observed (`80:96-99`). That argument does not reach the niche, because the
niche is not an unenforced side-condition; it is a **compiler-declared validity range**. The
excluded pattern is not an inhabitant of the carrier type: the constructor refuses it (asserted),
and a transmuted zero is undefined behaviour by the type's own declaration, which is a strictly
stronger perimeter than field-shrinking, since field-shrinking relies on the tower's construction
discipline while the validity range is enforced by the language against every reachable path. The
amendment: *statement 0 quantifies over every inhabitant of the carrier type at the fields' width;
partiality is expressed by shrinking the fields or by a compiler-enforced validity range on the
carrier, never by an unenforced side-condition.* One clause, and both one-pass proposals survive
under it with their perimeters intact. Stated as the general form in section 2.4: the levels
partition bits, the niche partitions inhabitants, and the honest quantifier at every level is over
the level's type's inhabitants, not over its width's patterns.

**And one scope condition file 84 owes before its section 5.4 hardens.** "It applies to every
numeral with at least one pattern outside its value set" (`84:378-380`) is a layout claim, and the
*cheap* construction (the uniform `+1` bias) additionally requires that the shift not wrap: the
carrier's maximum pattern must be spare, or the datum set must sit low enough that `+1` stays in
range. Checked against the design's witnesses rather than left abstract: it holds at every one.
The bounded fixed-point case is file 84's own probe domain (top pattern spare by construction).
E4M3's spares are `S.1111.111`, patterns `0x7F` and `0xFF`, so the maximum pattern is spare and the
uniform `+1` maps the 254 datums injectively onto nonzero patterns without wrap. The condition is
one sentence of spec text; without it, a future numeral whose top pattern is a value silently loses
the trick and nobody will know why. Monotonicity of the bias, which every datum-keyed order fact
rides on, is asserted over the whole domain in the probe rather than assumed.

*Grounded on: settled shapes (`80:90-112`, `84:354-411`, `82:296-303`), compiled
(`86_probes/probe_1`, all five fact groups in const position), reasoned (the amendment and the
scope condition, mine, offered as suggestions).*

### 1.2 The typed quantum confirmed, on a ground stronger than its author gave it: the carve-out and rTIE's totality are one fact

File 84's section 6 claims lifting the quantum to type position removes the standard's own
value-determinism carve-out: `quantise::<Q>(x)`'s result value is a function of `x`'s value alone,
so the operation is value-keyed and law-eligible where the two-datum form can never be a law
(`84:415-427`). One pass, nobody has read it. I attacked it from the operation the review already
holds exhaustive results for.

**The confluence, compiled** (`86_probes/probe_2_rtie_is_pinned_quantise_plus_escape.rs`, run at
`-O`, file 80's model shape, 4000 datums, exact integer arithmetic, every count asserted in the
program). `roundToIntegralExact` is `quantise::<0>` plus exactly one resolution the pinned form is
denied: **exponent escape**. On every cell where the pinned form delivers, the two value functions
agree (3100 cells). The pinned form's refusals (900 cells) are exactly, cell for cell, the datums
where rTIE's result integer needs a positive exponent to fit within `p` digits, which is the
standard's preferred-exponent freedom doing the resolving. rTIE is total on the model, asserted per
cell.

So file 80's decomposition ("rTIE is a value-keyed law plus a datum-keyed exponent selection,"
`80:150-153`) and file 84's construction are the same fact seen from two sides, and the sharpened
statement is worth having because it explains rather than stipulates: **pinning the quantum removes
the exponent-escape resolution, so range events appear exactly where escape was being used; freeing
it removes them. The standard's carve-out for `quantize` and the totality of
`roundToIntegralExact` are one mechanism, and `At<N, Q>` is the type that makes the mechanism
visible.** rTIE is, in this reading, the standard's own worked example of the typed form: quantum
fixed at one, value function value-keyed, and the datum-dependence confined to cohort selection,
which is exactly where file 80 measured it. The carve-out claim survives its attack, and the
second read file 84's section 9 asks for is, for the section 6 half, done: I checked
law-eligibility against the law-key rule directly (`quantise::<Q>` reads `Numeral`-side members
only, and no law may read `Lowering`, `78:195-196`; nothing in the typed form touches one).

*Grounded on: settled shapes (`80:114-165`, `84:415-463`), ratified (`78:195-196`, `78:275-293`),
compiled (`86_probes/probe_2`, counts asserted, output in `OUTCOMES.md`), reasoned (the one-fact
statement, mine).*

### 1.3 Statement C's discharge argument, given the derivation it was owed, and its perimeter is missing the mutation clause

File 83's statement C: the container's bits outside the carrier are canonical, established once by
the projection's pure constructor, with the only-door sentence naming the container as the
canonicalising projection's domain (`83:194-199`). Its discharge argument (`83:167-183`): the
obligation cannot be a `Crosses` condition because the container is not declared by anyone, so a
declaration obligation cannot reach it; it is discharged by the tower's own pure projection, an
obligation only at the constructor that accepts foreign bytes. File 85 read the section and "found
no counter-argument" (`85:281-282`), which is a confirmation, not a derivation. I derived it
independently before re-reading file 83's own wording: statement P bites at the one site a
hand-laid format is asserted, because `StoredWidth` is the declared axis and the declarer owns the
declaration; the container is a projection's output, and by the layer-keying rule an obligation on
it would be keyed finer than any declarer's identity, so the only coherent discharge site is the
projection itself, and purity forces canonical padding there by file 73's own one-argument
argument. Same conclusion, reached from the keying side rather than the site side. The discharge
argument is confirmed; with file 83's as the first read, that call has its two.

**The gap, and it is a perimeter gap, not a discharge gap.** The only-door sentence covers
*observation*: every value- or datum-keyed read consumes the container through the canonical
projection. It says nothing about **mutation**, and the workspace's own perimeter rule names
exactly this hole: the observation surface includes "anything that hands out interior references"
(`what-you-can-observe-is-what-you-guaranteed.md`, "How to apply," item 1). A `&mut` door into the
container that bypasses the constructor can dirty `[W_S, W_C)`, and the failure is quiet in
precisely the way this design has learned to distrust: every value-keyed observation stays correct,
because the canonical door collapses the padding on the way through, while the byte image and the
digest, the two facts legitimately keyed below the door (`78:590-599`), silently decorrelate from
the tower's own canonical form. That is the exact defect class statement C exists to close,
re-opened one API decision later. The spec sentence owed: *statement C's guarantee is quantified
over containers reachable through the tower's constructors; a mutable borrow of the container is
part of the perimeter, and a type carrying statement C either does not hand one out or
re-canonicalises on release.* One sentence, and it is the perimeter rule's standing content applied
here, not new doctrine.

This also lands the correction to my own file 72: its external-images chapter stated the crossing
statement structure as two statements taken verbatim. The byte image now takes three (0, P, C), and
72's digest law gains a sharper phrasing for free: the canonicalising projection the digest factors
through has the container, not the carrier, as its domain, which is file 83's relabel of file 80's
nine-bit finding applied to my chapter's own mechanism.

*Grounded on: settled shapes (`83:131-205`, `78:546-549`, `78:590-599`), the workspace perimeter
rule (`what-you-can-observe-is-what-you-guaranteed.md`, read fresh), reasoned (the independent
keying-side derivation and the mutation clause, mine).*

### 1.4 `At<N, Q>`'s exponent typechecks as ordinary vocabulary, closing file 85's open item

File 85's section 4: "nobody has yet built `At<N, Q>`'s exponent as a literal instance of
`EZero | EPos<P> | ENeg<P>` and watched it typecheck" (`85:447-450`). Built
(`86_probes/probe_3_at_exponent_is_ordinary_vocab.rs`, compile-only, zero gates): a model tower
with the sealed `Pos` grammar and the sealed signed-exponent vocabulary per the trait table
(`78:611-623`), an `ExponentForm` with a ranged member and a fixed member, and `At<M, Q>` reading
`M`'s radix and precision through while picking `Fixed<Q>`. `At<Dec3, EZero>` and
`At<Dec3, ENeg<H>>` both satisfy the ordinary `Numeral` bound, asserted in const position, and the
typed `quantise<M, Q>` signature typechecks against them. The construction mints nothing: the fixed
form is the constant exponent function the founding one-formalisation identity says fixed point
already *is* (`78:186`, file 85's own `generic_format` grounding at `85:296-311`), so the
vocabulary had to contain it for fixed-point numerals to be declarable at all. This is a model
compile, not the tower's final grammar; when the real `ExponentForm` lands, the assertion transfers
by re-instantiation, not by trust.

*Grounded on: settled shapes (`84:85-98`, `85:296-311`, `78:611-623`), compiled
(`86_probes/probe_3`), reasoned (the minting-nothing argument).*

### 1.5 The audit audited: its counts reproduce, its unproved branch is proved, and the corpus count self-inflates

File 85 is one-pass. Its checks, re-checked with the commands stated: `grep -c "\bcontainer\b"
78_consolidation_eight.md` gives 8 and `grep -o | wc -l` also gives 8 (no line carries it twice),
confirming files 83 and 85 against file 82. The arity counts reproduce both ways exactly (25
lines and 27 occurrences in file 64; 14 and 15 in file 55), confirming file 85's two-methods
diagnosis to the digit. Its recompilations I spot-checked by method rather than re-running all
three; the one I did re-run (`80_probes/probe_4b`) refuses with the E0200 file 85 quotes.

**The `(R >= 1, P < L)` branch of the fold formula, proved, closing file 85's open item 3**
(`85:443-446`). File 82's bit test for that branch is `(R << P) >= A`. Expand `A = 2^(L-1) + R`:
`R * 2^P >= 2^(L-1) + R` is exactly `R(2^P - 1) >= 2^(L-1)`. And the boundary the bit decides is
whether `A(2^P - 1)` reaches `2^(L+P-1)`: substituting the same expansion,
`A(2^P - 1) = 2^(L+P-1) - 2^(L-1) + R(2^P - 1)`, which is at least `2^(L+P-1)` exactly when
`R(2^P - 1) >= 2^(L-1)`. The same inequality, so the bit is correct; the upper bound
`A(2^P - 1) < 2^(L+P)` is immediate from `A < 2^L`, and the lower bound `>= 2^(L+P-2)` from
`A >= 2^(L-1)`, `2^P - 1 >= 2^(P-1)` for `P >= 1`, so the bit length is confined to the two values
the formula distinguishes. One more thing falls out: the `P >= L` branch's `bit = 1` is the
**always-true case of the identical test** (`R >= 1` and `2^P - 1 >= 2^L - 1 >= 2^(L-1)` give
`R(2^P - 1) >= 2^(L-1)` unconditionally), so the formula's three branches are one test,
`bit = [R(2^P - 1) >= 2^(L-1)]`, with the power-of-two case its `R = 0` instance. The formula file
82 verified on 1,225,601 cells is now proved on all of them, and is one branch simpler than
written.

**One new wrinkle for the count discipline.** `grep -l "[Aa]rity" *.md | wc -l` gives **56** this
session, against file 82's 53. Neither is wrong: files 83, 84 and 85, by discussing the dispute,
added hits. A corpus count is dated the moment it is made, because the corpus discussing a count
inflates it; the adopted fix (name the command) already handles this, provided the checker also
notes the corpus size at the time. Not a defect anywhere, worth one line so the next recount does
not read 53 versus 56 as an error.

*Grounded on: measured (the stated grep commands, run this session), compiled
(`80_probes/probe_4b`, recompiled), reasoned (the branch proof, mine, ordinary algebra).*

---

## 2. The stretch assembled

### 2.1 The common move is not binding time; it is the subject

Op's dispatch names five events and says three are the same move. Laying them side by side, the
move is sharper than "a quantity belonged at a different level," because in two of the three the
binding time never changed. What changed is **what the fact was about**.

The bitpack decode (file 81, corrected at `82b`) computed the byte offset and shift from the
running index, the visible thing in the loop, when their true subject was the logical width, a type
parameter; naming the true subject (associated consts on the layout) took the cost from 4.6x to
1.50x, roughly three times, and `82b`'s second correction took it further. Quantise's hard failure
(file 84) was measured against the operand's numeral, the visible type in the signature, and looked
like a new failure kind; its true subject is `At<N, Q>`, the numeral the operation targets, on
which it is the design's ordinary `OverRange`, agreeing cell for cell with the ratified
extended-grid boundary. Capacity (file 79) was handed to the far-point rule directly, because op's
analogy named it; the rule's true subject is one construction below, the index domain, where the
far point is the predecessor and the empty capacity is refused at the type level. In each case the
visible subject gave a well-formed account: a correct decoder, a real refusal density, a plausible
analogy. **Misplacement did not fail; it cost.** A multiplier, an apparently novel kind, an
off-by-one trap. And in each case the fix was smaller than the mechanism it replaced: no new
vocabulary for `At<N, Q>` (probe 3), no new seal for capacity, no new trait for the decode plan.

The other two events are the same shape one step removed. The width question (file 83) is the
enabling act: the container level existed in the design as a spine-rule firing the whole time
(`83:84-89`) and had no name, so a measured misordering had no statement to violate. And the owed
list (`82b`) is the process instance: a closure's label and its artifact are two subjects, and the
review had been keying closure on the label.

### 2.2 Why misplacement survives review: the levels coincide where everyone instantiates

This is the part I think is the pattern, and it is a statement about verification rather than
about design. A wrong subject survives because **at the instantiations everyone reaches for, the
wrong subject and the right one coincide**, so every check passes and every account balances.

The instances, and they span design, measurement and bookkeeping. At `Cold` minimum, `W_F = W_S`,
so file 81's "every parameter is a function of the logical width alone" was exactly right at the
only configuration measured, and file 83's section 6 had to invent a headroom-declared model to
make the plan consts and the value mask part company (`83:290-316`). At every eight-bit model,
logical width equals container width, so every padding claim in seventy-nine files was checked
where it is vacuously true, which is file 80's own closing advice (`80:377-380`) with file 83's
relabel. For exact results, `quantize` and the identity agree, and for in-range results `At<N, Q>`
embeds in `N`, so the failure's true home only shows at the 35.5% of the model where the two
numerals' ranges separate. A capacity and its last index differ by exactly one, and coincide in
every informal sentence until the empty capacity separates them absolutely (`79:44-51`). File 75's
standalone instruction counts were wrong three and five times **in the same direction**, so the
ratio matched the bench and the agreement read as corroboration (`81:137-141`). File 80's and file
82's arity counts were both right under two counting methods that coincide on every file where no
line carries the word twice (`85:195-211`). And a closure's label and artifact coincide exactly
until the one item where they drifted apart rode the owed list for two stretches.

Seven instances, one shape: **two quantities that are distinct in general and equal at the common
case, checked only at the common case.** Structural concordance read as evidence. It is the
sampled-law failure the workspace's test discipline already names, wearing a subtler coat: nobody
chose a sample, the sample chose itself, because the degenerate instantiation is always the
cheapest one to build.

### 2.3 Whether it should be a rule, both angles, and a recommendation

**The deflationary reading.** No fifth placement rule is needed. The four rules already assign
homes: the spine rule places computed-and-in-type quantities, the layer-keying rule places facts on
layers, the pricing pillar forbids runtime with a const alternative, carrier-at-birth seals
vocabularies. Every finding in this stretch, once made, filed under an existing rule. A fifth rule
saying "state facts on their true subject" would be the layer-keying rule's sentiment restated
without its mechanical test, and a rule that fires everywhere fires nowhere.

**The reading I recommend, because it has a mechanical test the four rules lack.** The four rules
say where things belong. None of them says how a misplacement is *found*, and this stretch found
all of its misplacements one way: someone built the instantiation where the coinciding quantities
separate. So the candidate is not a placement rule but a **verification requirement, the
separation requirement**: *a claim about a distinction is checked at an instantiation where the
distinction is nonvacuous; a model where the two quantities coincide checks nothing about their
difference, whatever it checks about their common value.* It has prior instances already stated
locally three times (file 80's padding-model advice, file 83's two-levels-coinciding clause, file
84's negative control that must differ), it generalises them to one sentence, and it is
checkable at review time: a model's parameter table states which of the design's levels and
subjects it separates, the way a probe already states its toolchain. The whole-matrix discipline
the workspace already runs is the value half of this; the separation requirement is the subject
half. Where the whole matrix is affordable, it subsumes this; where it is not (models are the
only affordable form at real widths, `78`'s verification section), separation is the property
that decides whether the model means anything.

Coincidence or pattern, then: pattern, but a verification pattern, not a design pattern. The
design's four rules stand as they are; the review's *models* gain one standing requirement. Op's
call, and the cheap first act if adopted is retroactive and small: each standing model in the
probes gets one line naming what it separates, which would have flagged the eight-bit models'
vacuity seventy files earlier.

### 2.4 The interaction op flagged, checked: biased storage and the three levels compose cleanly, and the composition names the quantifier

The two constructions were derived independently days apart, and they are not in conflict; they
are orthogonal partitions of the same thing, and stating that resolves the collision in section
1.1 as a corollary rather than a patch. **The three levels partition a datum's bits; the niche
partitions a level's inhabitants.** `W_F`, `W_S`, `W_C` say which bit positions belong to the
fields, the carrier's padding, the container's padding. The niche says which *patterns* of the
carrier's positions are inhabited. The biased carrier changes no level: it is a 16-bit-fields,
16-bit-carrier lowering whose carrier type has 65,535 inhabitants instead of 65,536. Statement C is
untouched, because the spent pattern is a carrier-inhabitant fact, not container padding; the
`None` pattern of an `Option` column belongs to the composite type's own crossing, not the
numeral's, and a serialised optional column is the composite's byte image. What the composition
forces into the open is the quantifier convention section 1.1's amendment states: **at every
level, statements quantify over the level's type's inhabitants, not over its width's bit
patterns.** For every non-niche type the two are the same set and every existing statement reads
unchanged; the niche is precisely the mechanism for which they differ, and it is the language's
own, enforced, so the perimeter argument that motivated the width-patterns phrasing carries over
stronger, not weaker. One convention, three statements re-grounded on it, no statement's content
changed.

---

## 3. What a consolidation could take, close to verbatim

*Statement 0's quantifier is amended before either of two one-pass proposals hardens, because as
written they are inconsistent: the biased-niche lowering's decode is partial on exactly one bit
pattern of the fields' width, and no field shrink expresses a domain whose size is not a power of
two, both compiled. The amendment: every crossing statement quantifies over the inhabitants of its
level's type, not over the bit patterns of its level's width; partiality is expressed by shrinking
the fields or by a compiler-enforced validity range on the carrier, never by an unenforced
side-condition. A validity range is a stronger perimeter than field-shrinking, being enforced by
the language against every reachable path rather than by the tower's construction discipline. The
niche construction itself gains one scope condition: the cheap uniform bias requires the carrier's
maximum pattern to be spare or the shift not to wrap, which holds at every witness in the design
including E4M3, whose all-ones patterns are its NaN slots; and the bias's monotonicity, which
datum-keyed order facts ride on, is part of the construction's contract rather than an accident.*

*The typed quantum's carve-out claim is confirmed, and sharpened into one mechanism with the
review's exhaustive rTIE results: roundToIntegralExact is quantise at a pinned quantum of one plus
exactly one resolution the pinned form is denied, exponent escape, the standard's
preferred-exponent freedom. Compiled at the decimal model, the pinned form's refusals are cell for
cell the datums where rTIE's totality is bought by a positive exponent, and the two value
functions agree everywhere else. Pinning the quantum removes the escape, so range events appear
exactly where escape was being used; the standard's value-determinism carve-out for quantize and
the totality of roundToIntegralExact are one fact, and `At<N, Q>` is the type that makes it
visible. `At<N, Q>`'s pinned exponent typechecks as a literal instance of the sealed exponent
vocabulary at a model tower, minting nothing, closing the audit's open item.*

*Statement C's discharge argument has its second independent read, from the keying side: statement
P bites at the declarer because the stored width is declared, and the container is a projection's
output on which an obligation would be keyed finer than any declarer's identity, so the discharge
site is the projection and purity forces canonical padding there. Its perimeter gains the mutation
clause the observation-perimeter rule already implies: the guarantee is quantified over containers
reachable through the tower's constructors, a mutable borrow of the container is part of the
perimeter, and a type carrying statement C either does not hand one out or re-canonicalises on
release, because a dirtied container is invisible to every value-keyed observation through the
canonical door while the byte image and the digest silently decorrelate.*

*The stretch's pattern is a verification pattern, not a fifth design rule: a wrong subject survives
review because at the instantiations everyone reaches for, the wrong subject and the right one
coincide, and seven instances of that shape span the stretch's design findings, its measurements
and its bookkeeping. The separation requirement, offered for the review's models: a claim about a
distinction is checked at an instantiation where the distinction is nonvacuous, and a model states
which of the design's levels and subjects it separates, the way a probe already states its
toolchain. The fold-width formula's remaining branch is proved, and the formula's three branches
unify to one test, bit = [R(2^P - 1) >= 2^(L-1)], with the power-of-two case its R = 0 instance.*

---

## 4. Out of scope, reported under the standing obligation

Nothing new in the shipped tree; I did not read it. Two record notes. The corpus's arity-hit file
count is now 56 against file 82's 53 because files 83 through 85 discussing the dispute added
hits; a corpus count is dated by the corpus discussing it, and a recount that treats the drift as
an error would itself be the error. And the tautological test at `arvo-tensor/tests/capacity.rs:14-18`
rides forward one more file, flagged for deletion since `78:874-876`, untouchable under the
panel's scope; whoever scaffolds the implementation phase under `79b`'s mandate should delete it
in the first red-suite commit, because a fabricated green line has no place in a suite whose
opening state is deliberately, honestly red.

## 5. What this leaves open

- **The amendment in section 1.1 is one pass.** It reconciles two proposals but it is itself
  unread, and it touches the constructive-extensibility compile still owed (`78:947-949`): a
  foreign hand-laid lowering with a niche is now expressible in principle, and one compile should
  pin whether the `unsafe impl Crosses` obligation composes with a declared validity range or
  needs its own clause.
- **The separation requirement is a proposal**, with a cheap retroactive act named (one line per
  standing model stating what it separates). Whether it becomes standing verification text is
  op's, and it wants its own second read against the whole-matrix discipline it claims to
  complement.
- **The mutation clause in section 1.3 is stated, not compiled.** A guard shape (mutable access
  that re-canonicalises on release) is a few lines and would pin the clause the way the
  compile-fail probes pin the seals; I did not build it.
- **Probe 3 is a model compile.** The real tower's `ExponentForm` does not exist yet; when it
  lands, `At<N, Q>`'s typecheck transfers by re-instantiation against the real grammar, and the
  structural-recursion care file 82 established applies to its consts.
- **The three genuinely open items file 85 confirmed** (the §5.12 citation, `foldnum` against the
  real four-member contract, the non-default `Canonical` compile) remain open; nothing in this
  file touches them, and per the closure discipline each now wants its closing artifact named
  when it is next dispatched.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion,
not a ruling.

*Grounded on: ratified (`78:152-166`, `78:186`, `78:195-196`, `78:275-293`, `78:409-441`,
`78:611-623`, `82b` in full, `79b` recorded), settled shapes (`79` through `85` in full, read this
session; `80:90-165`, `81:199-239`, `82:296-303`, `83:131-205`, `84:354-463`, `85:195-211`,
`85:435-453`; the workspace perimeter rule), compiled (`86_probes/probe_1`, `probe_2`, `probe_3`,
all built and where applicable run fresh this session on the pinned toolchain inside the tree,
commands and outputs in `86_probes/OUTCOMES.md`), measured (the stated grep recounts, the test
gate run fresh), reasoned (sections 1.1's amendment, 1.3's derivation and clause, 1.5's proof,
and all of section 2, mine).*
