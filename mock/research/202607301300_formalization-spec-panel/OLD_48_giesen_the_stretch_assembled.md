# 48. The stretch assembled

**Member:** Fabian Giesen. I wrote file 34, the previous assembly. Three of its claims were
subsequently overturned by people who recompiled them: the width-chain-already-value-unique
reassurance (false, file 36), the ordered three-relation ladder (replaced by file 37's lattice), and
the reification-stability generalisation (corrected to a hypothesis about the reifying element). All
three are on the droplist (`40:777-792`) and none is defended here; where this file assembles, it
assembles from the current pieces, and its own cheap factual claims are rechecked below rather than
carried on file 34's record.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed,
9 ignored, summed from the per-binary `test result:` lines, matching files 41 through 47 exactly.
The design surface has no shipped source: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` from the repo root, exit 1, empty, file 45's corrected command (`45:456-475`)
rerun rather than inherited. The test bodies in the surface this file touches are the review's own
probe files; I read files 47's, 46's and 42's as source and rebuilt file 47's rather than trusting
its outcome table (section 1). Canon gate: `40_consolidation_three.md` and `44b_op_checkpoint_ten.md`
in full before any code. Nothing below overturns a ratified call; section 1.2 examines whether file
47's second proposal touches one and concludes, with the citations, that it does not.

**What I read:** `40_consolidation_three.md` in full, twice. `41` through `47` and `44b` in full,
the deliverables since it. As source rather than through any paraphrase: `47_probes/` in full
(`tower.rs`, `probe_2`, `probe_3`, `probe_3b/c`, `probe_4`, `probe_5b`, `probe_6`, `probe_1b`,
`OUTCOMES.md`), because file 47 reports its probes were compiled by their author and read by nobody
and this dispatch was sent to change that; `46_probes/vu_nat_sealed_adj.rs` as the tower everything
composes with; and, outside this repository, the hilavitkutin canonical consolidation's Step 5
(`hilavitkutin/mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.
md:1328-1347`, plus lines 606 and 1868-1873), because file 47 makes a claim about that consumer and
the brief names external claims as the class this review is worst at checking. `ls` of the review
directory once: 47 numbered deliverables plus probe directories before this one.

**What I compiled, separated from what I reasoned.** Compiled, all against the pin
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`; every diagnostic and
refusal below is a `pin + host` fact in file 45's sense): eight of file 47's probes rebuilt fresh,
all eight reproducing exactly (`48_probes/OUTCOMES.md`, first table); five probes of my own, one
refusing on E0275, one compiling a whole-matrix algebra, two refusing on the grade seal, one
refusing on E0425, plus one uncommitted four-line negative control proving my own positive
assertions can fail. No timer ran anywhere; no runtime claim is made. Reasoned without an artifact,
and marked in place: the external-consumer analysis in 1.1 (grounded on the hilavitkutin canon's
text, cited by path and line), the exponent-fork derivation in 2.1, the `Int` resolution in 2.2,
and every grounding amendment in 2.4.

---

## 0. The verdict, stated first

**File 47's two recommendations survive the attack; two of its three load-bearing arguments do
not survive unchanged, and the repairs strengthen both.** The strict-evaluation recommendation
stands, but its most vivid piece of evidence, the RCM claim, is wrong against the consumer's own
canonical design: RCM permutes which column sits at which arena offset and which WU runs when; it
does not permute the records a fold traverses (section 1.1, with the citation file 47 did not
have). What actually exposes the short circuit downstream is stronger than what file 47 claimed:
the engine's consumer-pluggable stealing executor makes a short-circuit report nondeterministic
run to run, and the regrouping fold cannot implement a short circuit at all without a cross-morsel
cancellation protocol the design has already ruled is not arvo's. The grade projection stands, but
"the `generic_const_exprs` wall is not near the problem" (`47:434-441`) is false as stated: the
OTHER wall, files 41/42's E0275 eager-confirmation divergence, is one where-clause away from the
consumer-facing fold signature, compiled here (`48_probes/probe_1`), and the projection stands
clear of it only by a structural property of its bound chain that nothing states and nothing pins
(section 1.2). The alias table's seam with the seal, the one this dispatch was pointed at, is
benign and I can say why with the seal's own quantification rather than by assumption; the table's
REAL defect is elsewhere: its range does not survive file 43's exact-division constants, and the
out-of-range refusal is compiled here to show where the shape runs out (section 1.3).

**On assembly: the five pieces since the consolidation are one design, and the rule that makes
them one is already written down, in file 47, one section before its author undersold it.** "A
quantity that has to be computed and then appear in a type is a type; a quantity that only has to
be read is a const" (`47:503-508`). That sentence subsumes the width chain's replacement (36), the
rational bias (41/42), the grade projection (47), and, run forward, decides two things this review
has left open: the exponent-as-a-type fork (`40:690-691`), which it forces open and answers yes
the moment the exact widening family reaches `Ranged` numerals (section 2.1, reasoned), and the
`Int` tier's fate, where it supplies the third read neither file 45 nor 47 had: even `Int`'s one
prospective consumer would land on the constructor-sign shape the design has already sealed and
attacked twice, so the drop is safe against the future too (section 2.2). The one genuinely
missing piece of the assembled design was the join half of the grade algebra, which file 47's two
proposals jointly require and neither compiled; it is built, sealed, and checked over the whole
matrix in `48_probes/probe_2`, and with it the two open sentences (`40:632-641`) collapse into one
spec statement (section 2.5).

---

## 1. The attack on file 47

All of file 47's compiled claims reproduce. Eight probes rebuilt fresh against the pin: the three
clean ones compile, the five refusing ones refuse with the recorded error heads
(`48_probes/OUTCOMES.md`). The 81-term grouping-invariance check, the reordering table, and the
diagnostic-degradation table are const assertions and all hold. What follows is about the
arguments, not the artifacts.

### 1.1 Strict evaluation: right answer, wrong star witness

File 47's recommendation rests on three legs: the droplist contradiction (`40:711`), the
reordering consumer (`47:146-149`), and the degrading diagnostic (`47:151-172`). The first and
third stand. The second is the class of claim the brief names as the review's weakest, so I went
to the consumer's canonical design instead of taking the sentence.

**The RCM claim is false.** File 47, twice (`47:146-149` and `47_probes/probe_4:166-169`):
"hilavitkutin's RCM renumbering is exactly a permutation of a column's traversal order." The
hilavitkutin canonical consolidation, Step 5 (`202603181200_topic.hilavitkutin-design-
consolidation.md:1328-1347`): RCM produces a **row reordering**, which is WU execution order, and
a **column reordering**, which is "arena memory layout: RCM reorders columns so that co-accessed
columns have adjacent offsets within the co-located arena. When a fiber's morsel walks
`arena_base + col_offset + i * stride` ...". The per-record index `i` is untouched by both. RCM
decides which column sits where and which WU runs when; a fold over one column visits its records
in record order regardless. Turning on RCM changes no fold's traversal, so it changes no
short-circuit report, and the sentence "turning on a cache-layout optimisation changes what a
consumer's diagnostic says" (`47:148-149`) has no mechanism behind it as cited.

**What is actually true downstream is worse for the short circuit, on two counts the canon states
outright.** First, the same document at line 606: "Commutative: deterministic segments = always in
record order (T3 decision #100)". The engine's default execution is deterministic and
record-ordered, so under the DEFAULT executor the short-circuit report would at least be
deterministic, which weakens file 47's claim further. But at lines 1868-1873: "Work-stealing is
NOT the default ... the consumer can provide a stealing Executor via the Executor trait." Under a
stealing executor, morsel completion order is a race. A regrouped parallel fold under short
circuit either implements cross-morsel cancellation, a scheduler protocol the design has already
ruled belongs to hilavitkutin and not arvo (`40:203-205`), or short-circuits per morsel, in which
case the report depends on which morsels happened to complete before the refusing one, which is
run-to-run nondeterministic on unchanged data. Second, and sharper: **the short circuit is not
implementable on the regrouping fold at all** without that protocol, because a short circuit is a
sequential notion and the regrouping fold's whole purpose is that its parts run concurrently. So
the open sentence was never a free choice between two readings for both combinators; it was a
choice for `fold_sequential` only, and choosing short-circuit there would make the two named
combinators deliver different reports for identical data, handing a consumer who switches
combinators to get definedness-faithfulness a silent report-semantics change in the same edit.

**And the one cost file 47 concedes to strict is not a semantic cost.** `47:189-194`: strict means
"an operand whose sibling has already refused is still computed, which is real work thrown away."
That prices the semantics as if it were the implementation. The strict sentence fixes what is
*observable*: the report is the term's leaf multiset. Where the consumer never reads the report,
the event counting is dead, and an implementation is free to skip the sibling under the as-if rule,
because nothing observable distinguishes the skip. Where the consumer does read the report, the
work is not thrown away; it is the product. So strict costs the expensive-sibling case exactly
nothing when the diagnostic is unused and exactly the diagnostic's own price when it is used, and
the scalar-path argument file 47 leaves open for "a member who wants to argue that case" has no
case left to argue. (This is the same observability split the design already runs one level down:
`40:129-135`, what a law may read is defined by what its key can name, not by what the machine
happens to compute.)

**Verdict on 1.1: adopt file 47's sentence and corollary verbatim (`47:84-94`), strike the RCM
sentence from its supporting argument, and replace it with the executor-pluggability citation
above.** The recommendation comes out stronger: strict is not merely the reading the standards
tilt toward and the diagnostic needs, it is the only reading the regrouping combinator can carry
at all under the execution model the stack is built for.

### 1.2 The grade projection: adopt it, and pin the wall it did not dissolve

The mechanism reproduces: `probe_3` compiles clean with no unstable feature, `probe_3b`'s designed
E0277 fires with the free `help` list, `probe_3c` refuses both understatements with E0308. The
three questions the brief asks:

**Does it dissolve the wall or relocate it? Neither: it stands clear of one wall and next to
another, and file 47 only looked at the first.** The `generic_const_exprs` wall (computing a const
in type position) is genuinely not near the projection; that half of `47:434-441` is right. But
files 41 and 42 established a second wall in exactly this terrain: naming a trait with one
unconditional blanket impl as a bound forces eager confirmation, and confirmation over `Reduce`'s
chain diverges with E0275 (`41:section 3`, corrected boundary `42:185-227`, extended to rigid
non-inhabitants by `46:section 6.2`). File 47's chain avoids it only because of an unstated
structural property: `Cmp`'s impls pattern-match on constructor heads, so the solver has no
unconditional candidate for an abstract operand and defers, and `InteriorSafety`'s single blanket
impl has finite, non-recursive obligations. Nothing in file 47 names this property, and one
plausible refactor violates it: compute the safety margin as a reduced headroom ratio, spelled
`Ratio<Hd, Am1>: Reduce` on the fold's own signature. Compiled (`48_probes/probe_1`): **E0275,
`overflow evaluating the requirement Pz<O<_>>: ExactDivOdd<_>`**, the exact signature of the
composition wall, now in the consumer-facing combinator. The constraint the spec owes, stated for
the consolidation: **every trait in a projection chain that reaches a consumer-facing signature
either pattern-matches on constructor heads or has finite non-recursive obligations; `Reduce` and
anything routed through it never appears in such a chain, only at concrete numerals.** This is
files 41/42's spell-the-chain discipline arriving at the combinator surface, and it wants the same
pinning file 46 demands for the seal: the positive control is file 47's probe 3, the negative is
my probe 1, and both ship as the compile-fail pair when the mechanism lands.

**Does the grade retain its algebra? Not as delivered: the join half was missing, and it is the
half file 47's OWN first proposal requires.** Section 1.1's sentence is "an operation's grade is
the JOIN of its operands' grades" (`47:88-90`). Section 3.2 makes the grade a type. Put together,
the join is a type-level operation, and file 47 compiled a lattice ORDER (`WeakerThan`, nine
impls) but no join, no laws over either, and no operation signature that combines two graded
operands. `48_probes/probe_2` builds it: sixteen constructor-headed `Join` impls (no blanket,
so it sits on the safe side of probe 1's wall by construction), and the laws over the whole
matrix rather than a sample: join-bitmask agreement (16 cells), commutativity as type equality
(16), associativity as type equality (64), identity and absorption (4 + 4), and order-join
compatibility (`a WeakerThan b` iff `join(a,b) = b`), positive half instantiated for all nine
declared pairs with a compiled negative control proving the gadget can fail. The seven negative
order pairs are named for the compile-fail suite rather than sampled away. With the join in
hand, `combine<G1, G2>(..) -> Graded<<G1 as Join<G2>>::Out>` compiles, which is section 1.1's
semantics carried in section 3.2's mechanism: one signature, both proposals, one design.

**Does anything downstream of grade-as-const quietly break? Nothing shipped, three registry rows,
and one carrier obligation file 47 skipped.** The tree grep is empty; file 37's mechanism is probe
material; the consolidation's `Folded<0>`/`Folded<1>` clause (`40:305-307`) is the one-line prose
edit file 47 already names. The registry consequences are section 2.4. The skipped obligation:
file 46, two files earlier, wrote the rule for exactly this situation, "seal the carriers, open
the contracts" (`46:270-274`), and the grade is a carrier: a closed vocabulary a guarantee
quantifies over, since `.weaken`'s soundness is quantified over what can inhabit `Grade`. File 47
left `Grade` unsealed and defended the perimeter with `Folded`'s private field alone, which is a
correct but *distributed* argument, the exact shape file 46's checklist replaces with a local
per-trait property (`46:section 1`, and its 6.3 finding is the same point one token large).
Probe 2 seals it at two lines and zero cost; probes 2b/2c are the two-route attack, refused with
E0277 and E0603, rustc's automatic sealed-trait note firing. `Definite` (47's 3.3) inherits the
closure for free, since `Definite: Grade` and `Grade` is sealed.

**Does it touch the ratified mechanism? No, and here is the citation trail rather than the
assertion.** The ratified call at `39b` is "the finest-view mechanism replaces the three-relation
fork" (`40:589`); its content is the lattice, the unique finest view, and the law-as-const-fn
statement (`40:245-249`), all untouched: laws are still const fns returning views, and views were
already types in return position. The declared-never-computed sentence (`40:308-312`) is
consolidation prose recording file 37's mechanism, whose own stated ground is the
`generic_const_exprs` ban (`45:352`, the `ffl` ground "a design shape forced by a forbidden
feature"). The projection does not relax the ban; it removes the claim's dependence on it. The
transfer rule's ratified-adjacent wording, "tolerance is a transfer, never a waiver"
(`40:300-303`), is quoted into the new shape unaltered and enforced more strongly, since the
caller can no longer overstate by typo. And the `const-fn-key` ground (`45:155`), what a law may
not name failing E0425/E0433, survives with its enforcement site moved: for the `FoldGrade` impls
the unnameability of `Lowering` is carried by the crate DAG (the algebra crate does not depend on
the crate that declares `Encoding`, so the name fails E0433 at the impl exactly as it fails in a
const fn), which is the same structural argument one level up and owes the consolidation one
sentence saying so.

**Verdict on 1.2: adopt, with the four attachments above** (the chain constraint pinned as a
compile-fail pair, the join algebra, the seal, the one-sentence `const-fn-key` note). The 2.66x
metadata reduction is a bonus exactly as file 47 prices it; the compile-time figures do not
separate the mechanisms and file 47 is honest about that.

### 1.3 The alias table: the seal seam is benign, the range is the real defect

**The seam this dispatch was pointed at closes cleanly, and the seal's own quantification is what
closes it.** File 46's guarantee block: the seal "quantifies over inhabitant introduction only. It
does not quantify over ... observation (any downstream may read `VAL`/`NUM`/`DEN` and recurse
structurally over the public constructors)" (`46:228-239`). An alias is observation: `n::N37` and
`Pz<I<O<I<O<O<H>>>>>>` are one type (reproduced, `probe_2` clean), nothing new inhabits `Nat`,
and the concern that the macro "generates them in the consumer's crate" does not survive reading
the macro: `$crate::n::${concat(N, $v)}` resolves in the crate that DECLARES the macro, which is
arvo, so the consumer's crate gains no declaration at all. A consumer who generates a private
table of their own is likewise just observing. There is no perimeter question here, and saying so
with the quantification block beats assuming it.

**Two things file 47 half-stated, sharpened.** First, the per-row const assertion is not merely
"what makes the table trustworthy" (`47:410-412`); it is *mandatory* under file 46's own 6.1
finding, because a bare type alias defers its bound checks and a wrong row would otherwise sit
green until first use. The assertion is the forcing projection 46's lesson demands. Second, the
feature citation `crates/arvo/src/lib.rs:26` (`47:398`) has the wrong path: no `crates/` exists at
the repo root; the gate is at `mock/crates/arvo/src/lib.rs:26` (verified: line 26 is
`#![feature(macro_metavar_expr_concat)]`). This is file 45's finding 4.4, the inherited-path
class, recurring one file after the correction landed, which says the correction needs to live in
the next consolidation's text and not only in file 45's section 4.

**The real defect is the range, and file 43 is what exposes it.** File 47 bounds the table at
0..=1024 because that "covers every precision, width and exponent bound in every format the
standards test names" (`47:427-429`), and flags the power-of-two gap itself. But the assembled
design has a second consumer of numeral notation that file 47's workload did not visit: file 43's
exact division subfamily, division by ANY representable constant, held with its finding recorded
(`44b`), whose motivating constants are "a sample rate, a window length, a fixed gain"
(`43:186-191`). Those live at 44100, 48000, 4096, far past any table a metadata budget tolerates
at 668 asserted bytes per row. Compiled (`48_probes/probe_3`): `nat!(48000)` fails with
`E0425: cannot find type N48000 in module $crate::n`, which is loud and names the missing row, so
the bounded table at least fails honestly. But a notation that cannot spell the constants the
design's own operation surface divides by is not the notation; it is the diagnostic-friendly core
of one. The fork file 47 did not open, with its real costs: **(a)** the bounded table as proposed,
plus a `pow2!` sibling table, accepting that 43's constants stay hand-spelled; **(b)** a
digit-emitting macro that computes the encoding at expansion and emits the literal constructors,
unbounded range, zero table, zero metadata, no trait-solver cost, at the price of either a genuinely
hairy `macro_rules` decimal-to-binary muncher or a proc-macro crate (std at compile time only, the
notko `#[profile]` precedent; arvo currently ships only declarative macros, so this is a real
dependency-surface decision). A digit-munching form that builds the number through the tower's own
`Dbl`/`DblInc` is not a candidate, for the reason file 47 already gave: it puts a projection chain
in every type. My lean, stated as a lean: (b), emitted-constructors form, because a 1024-row table
is a stored copy of a computable function and the range defect is permanent, while (a)'s only
advantage over (b) is avoiding one compile-time-only crate; and diagnostics are identical either
way, since rustc expands aliases in messages regardless (file 47's own honest non-fix,
`47:416-421`). Suggested, not ruled, and the choice is op's; either way the table-or-macro is a
spelling layer over the sealed encoding and touches nothing ratified.

---

## 2. The assembly

The brief asks whether the pieces since the consolidation are one design. They are, and the test
is that each piece's rule, run against a neighbour's territory, produces the neighbour's result
rather than a conflict. Where that test surfaced something no single file could see, it is below.

### 2.1 The spine rule, and the fork it forces open

File 47 states the rule and immediately understates its reach: "op's tenth checkpoint ratified
moving every number in this design out of const-land ... the grade is the one quantity that was
left behind" (`47:503-508`). Both halves need correcting. What `44b` ratified is that the
value-unique encoding replaces the width chain (`44b:10-15`); "every number" overstates it,
because **the exponent is also still in const-land** (`Implicit<const E: Exponent, ..>`,
`Ranged<const EMIN, const EMAX, ..>`, `40:70-71`), and the consolidation carries that as an
explicitly unopened fork (`40:690-691`). So the grade was not the one quantity left behind; it was
one of two, and the rule that moved the grade decides the other.

The derivation, reasoned and marked as such: the exact widening family is the one operation family
membership licenses (`40:216-219`), and its numeral-level maps COMPUTE result numerals from
operand numerals; that is the whole content of `mulnum`. For `Implicit` numerals every computed
member is already a type (adjustment through `Reduce`, bias through `BiasProduct`). For `Ranged`
numerals the exact product's exponent bounds are `EMIN1 + EMIN2` and `EMAX1 + EMAX2`, which is
arithmetic over const parameters whose result must appear in the result numeral's type: a const
computed in type position, the wall that already pushed width arithmetic out of const generics
(`40:720-723`) and that file 47 found under the grade. So the moment the exact family reaches
`Ranged` (and `ExactWindow`'s `Specials = None` gate, `40:221-225`, already contemplates exactly
the `Ranged`-without-specials case), the exponent must be a type or the family is unwritable
there. The fork at `40:690-691` is not "a real fork nobody has opened"; the spine rule opens it
and answers yes. Not compiled, because building `Ranged` numerals is not this dispatch's scope;
the grounding is `d69 + vu + ffl`, and the compile that would move it bins is a `mulnum` over two
model `Ranged` numerals, which whoever builds the float model (already owed for the overflow band,
`45:331`) gets nearly for free.

**Stated for the consolidation, one sentence: in this design, a quantity that is computed and then
appears in a type is a type; a quantity that is only read is a const. The rule is the reason the
width chain, the bias, and the published grade all moved, and it commits the exponent to move when
the exact family reaches `Ranged`.**

### 2.2 `Int`: the third read, and it settles the drop

File 45's finding 4.1 (first read: `Int` has an empty grounding set, lean drop) and file 47's
section 4 (second read, independent route, agreeing) give the two-expert base. What neither saw,
because it needs 2.1 in view: the one prospective consumer either of them could name for `Int` is
the exponent fork, and 2.1 says that fork opens. So the natural objection to the drop is "the
exponent will need a signed tier; dropping `Int` now means re-adding it." The objection fails on
the review's own record: exponents are signed, but the design's proven signed shape is not `Int`.
File 41 built `Bias` with sign carried on the constructor over `Pos` pairs (`BZero | BPos<N,D> |
BNeg<N,D>`, `41:101-131`), deliberately not over `Int`; file 42 hardened it; file 46 sealed and
attacked it on every introduction route. A signed exponent lands on the same shape (`EZero |
EPos<P> | ENeg<P>` over the sealed `Pos`), reusing the seal, the attack suite, and, for the bound
sums 2.1 needs, the signed addition that shape's arithmetic gets built with, none of which `Int`
as spelled has. So even the future that the forward-provision option was reserving `Int` for does
not consume it. **Drop `Int` from the ratified table**, per file 45's own lean, now with the
future-consumer objection answered rather than undefended; the encoding is three lines in the
audit trail if a use ever materialises that the constructor-sign shape cannot serve, and nobody
has named one. Op's call; this file's contribution is that the strongest argument for keeping it
is now known to fail.

### 2.3 Carriers and contracts, the principle generalised one step

Running file 46's rule over the newest piece produced section 1.2's seal finding, and the general
form is worth one spec sentence, because the design will mint more closed vocabularies (the
`SignDomain`/`SignIndexing` instances when built, the view lattice's nine points, the grade's
four): **every closed vocabulary a guarantee quantifies over is a carrier and owes the two-line
seal and the per-introduction-route adversary at birth, not after three passes.** The review's own
history is the argument: the numeral tower needed files 36, 41, 42 and 46, four passes, to close
because the checklist arrived last. The grade needed one file and two probes because it arrived
first. That differential is the checklist paying for itself, and it belongs in the consolidation
next to file 46's quantification block.

### 2.4 The provenance discipline meets its first mechanism change, and works

Files 44/45 built the grounding registry so that "the next overturn's blast radius is a grep"
(`44b:44-45`). File 47's section 3.2 is the first mechanism change to land on it, and the grep
works; here are the amended rows, so the next consolidation absorbs them rather than re-deriving:

- `45:352` (the transfer rule): checked-by gains `47 (projected form, rebuilt 48)`; the `ffl`
  ground **retires from this row**, because the declared-not-computed shape was grounded on the
  `generic_const_exprs` ban and the projected shape needs no feature. This is the forward
  direction file 45's 4.2 named, an obligation discharging, and it is the first instance.
- New ground for the registry, `pin`-rung until pinned in code: the projection-chain structural
  constraint of section 1.2 (constructor-headed or finite-obligation traits only; `Reduce` never
  in a consumer-facing chain). Positive control `47_probes/probe_3`, negative `48_probes/probe_1`;
  both become the compile-fail pair when the mechanism ships.
- `45:155` (`const-fn-key`): unchanged for laws; gains the one-sentence note that for trait-impl
  carriers of the transfer mechanism the same unnameability is enforced by the crate DAG (E0433
  at the impl), so the ground's content is "Lowering is unnameable from where verdicts live",
  with two enforcement sites.
- The overflow-band row set (`45:323-331`) is already per-member; the float member's strike
  (file 45's 4.3) stands, nothing here touches it.

One meta-observation the discipline's authors will want: file 47 repeated the `crates/` path drift
one file after file 45 corrected it (section 1.3), which says a correction recorded only in a
finding section does not propagate; the canonical command and the canonical path belong in the
consolidation's own gate paragraph, stated once where every member's gate description will be
copied from.

### 2.5 The two open sentences are one, and here is the spec text

Assembling 1.1, 1.2, and probe 2, the evaluation-strategy sentence (`40:639-641`) and the
`Precise`-surface question (`40:632-637`) stop being two items:

> **Evaluation and the fold surface.** Every operand of an operation is evaluated: an operation's
> grade is the join of its operands' grades with the operation's own contribution, whether or not
> any operand refused, so a term's report is a function of the term, invariant under the
> regrouping the transfer rule licenses, the reordering commutativity licenses, and the schedule
> the executor picks. (An implementation may skip work this cannot observe.) The fold surface is
> two named combinators: `fold`, which regroups and publishes, by projection, exactly the grade
> classes its law fails to preserve; and `fold_sequential`, which regroups nothing, publishes
> nothing, is faithful by construction, and is named for what it costs. Both are strict; a
> short circuit is not implementable on `fold` under a pluggable executor without a cancellation
> protocol that belongs to the scheduler, and offering it on `fold_sequential` alone would make
> the two combinators' reports disagree on identical data. The caller's type picks the door;
> `Definite`-style bounds carry the remedies; overstatement is `.weaken`, explicit and bounded on
> the sealed grade lattice, whose join and order are checked over the whole matrix.

Every clause above has a compiled artifact behind it: the join and its laws (`48_probes/probe_2`),
the strictness tables (`47_probes/probe_4`, rebuilt), the two-door refusals (`47_probes/probe_5c`,
`probe_3c`), the diagnostic (`47_probes/probe_6`), the seal (`48_probes/probe_2b/2c`), and the
one absence that keeps it honest, shape C's impossibility (`47_probes/probe_5b`). The one
remaining sub-item genuinely open inside it is per-application versus per-value-moved event
counting (`43:325-329`), untouched here, still op's.

### 2.6 Is it one design? The seams checked, and the residue

The seam list, with outcomes: alias table against the seal, benign by the seal's own
quantification (1.3). Grade projection against the composition wall, one refactor from collision,
constraint stated and pinned (1.2). Grade against the carrier principle, sealed at two lines
(1.2, 2.3). Strict evaluation against division's exact-partial operations, no conflict: `div_floor`
/`rem`'s refusal causes have no quantiser origin and join into the report like any other cause,
and the exact subfamily's totality-by-construction (`43:164-203`) means its grade is trivial
before the sentence even applies. Strict evaluation against the engine's execution model,
corrected and strengthened (1.1). The notation against division's constants, real defect, fork
stated (1.3). The spine rule against the exponent fork and `Int`, both settled in the same
direction (2.1, 2.2). The provenance field against the first mechanism change, works, rows
amended (2.4).

Residue, named rather than absorbed: the float model is still the largest unbuilt object every
thread keeps arriving at (the overflow band's struck member, the IEEE cause split, division's
float path, and now 2.1's `Ranged` exponent compile all want it; four independent demands is a
scheduling signal). The real-consumer compile-cost bench (`40:657-660`) gains yet another
neighbour in probe 2's matrix and still has no answer. And the codegen regression tests stand at
five owed, none built, unchanged since `44b` listed them.

---

## 3. What the next consolidation takes

Nearly verbatim, per the convergence instruction: the strict-evaluation sentence and fold-surface
block (2.5). The spine rule sentence (2.1), with the exponent-fork commitment marked reasoned.
The `Int` drop with 2.2's future-consumer argument recorded beside file 45's 4.1 and file 47's
second read. The carrier-at-birth principle (2.3) next to file 46's quantification block. The
projection-chain constraint (1.2) as a named design rule with its compile-fail pair. The grounding
amendments (2.4), including the first retired ground. The RCM correction (1.1), so file 47's
strongest argument is cited in the form that is actually true. And the notation fork (1.3) as an
open item with two costed branches, replacing the alias-table proposal's unbounded-range silence.

## 4. What this file does not decide

**The exponent-as-type derivation (2.1) is reasoned, not compiled**, and under the two-expert rule
it is a first read; a second member should form its own from `40:690-691`, `40:224-228` and the
`Ranged` declaration before the fork's answer hardens. **The `Int` drop (2.2) is op's**, with the
two reads on the unconsumed fact standing and this file's addition being an argument, not a
ratification. **The notation fork (1.3) is op's**, and if (b) is taken, whether the proc-macro
crate is acceptable dependency surface for arvo is a workspace call this file only prices.
**The per-application versus per-value-moved event reading** stands exactly as file 43 left it.
**The evaluation-strategy sentence itself remains op's to ratify**; 2.5 is the strongest form the
panel can hand up, not the ratification. **Nothing here prices runtime**: the strict-versus-short
codegen question (branchless bottom delivery, `11:543-546`) is cited from file 11's measurement,
and any fresh number belongs in `mock/benches/` under the harness, not in a probe.

## 5. Standing

File 47 asked the question thirty files had not and its two mechanisms survive recompilation and
attack; what did not survive is one external citation, one "the wall is gone" sentence, and one
missing half of an algebra, and all three repairs made the proposals stronger rather than smaller,
which is what an attack inside a converging review is for. The assembly finding is that the
stretch's five pieces are one design under one already-written rule, and that the rule, taken
seriously, closes two items the review had been carrying as open (the exponent fork, `Int`) and
fuses two more (the evaluation sentence, the `Precise` surface) into a single spec statement with
compiled artifacts behind every clause. The checkpoint reader gets the droplist additions in
section 1, the consolidation text in sections 2.5 and 3, and five probes plus eight rebuilds whose
every claim carries a reproduction command. Only op's calls are final; four of them are queued
here, each with its evidence attached and its second-read status stated.
