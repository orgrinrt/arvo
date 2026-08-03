# 38. What the design establishes

**Member:** Xavier Leroy. I wrote file 10, whose certification pass produced the two standing rules
(the forbidden-feature list as the transfer guarantee from model width to real width, and the
observation-surface reading of a type's guarantee), and file 28, on what identity must express. One of
my own statements has since been corrected: file 28's rational adjustment carried no normal form, file
34 compiled the consequence (`34:314-324`), and file 36 built the repair. I carry neither file forward
unexamined. The habit of mind this dispatch asks for is my own: a verification claim is exactly its
statement, its evidence, and its scope, and a claim whose perimeter is not stated is not a claim you
can trust. Before a consolidation, the perimeters are the work.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: summed independently
from the per-binary results rather than read off a headline, 654 passed, 0 failed, 9 ignored (1 + 6 +
2 across three binaries), matching the counts files 31 through 37 each report. I re-ran the negative
greps rather than trusting the seven-file citation chain: `grep -rn
"Monotone\|Magma\|AddAssoc\|Distributes\|Associative" crates/ --include="*.rs"` returns nothing, and
`grep -rln "Adjustment\|Numeral\|FullRange\|UTerm\|AddWidth" crates/ --include="*.rs"` returns
nothing, so every mechanism this stretch designed is at design stage, no shipped source, no shipped
tests to audit beyond the identity surface. I read `crates/arvo/tests/identity_laws.rs` in full (364
lines): the module doc states the full-matrix discipline and the body keeps it, both signednesses, all
four strategies, splits from zero integer bits up, the wide 65..=128 band that only `Hot`/`Cold`
reach, negative operands with the logical extremes, and the unwritable cases pinned as nine
compile-fail pairs under `crates/arvo/tests/ui/`. Nothing tautological found. Canon gate:
`26_consolidation_two.md`, `30b_op_checkpoint_seven.md` and `34b_op_checkpoint_eight.md` govern, all
read in full; nothing below overturns a D-numbered call or either checkpoint. Where I certify a claim
as not established, I say against which statement and with what artifact.

**What I read:** `26_consolidation_two.md` in full. `30b` and `34b` in full. `35`, `36`, `37` in full,
then `34` in full. Reached back for specific derivations only: `31_arntzen_settling_the_identity_
contract.md` section 4 (the settled contract this file certifies restatements against),
`30_pesce_the_identity_half_assembled.md:74-95` (the contract's origin), `33_lamport_the_laws_
restated.md:230-243`, `33:279-283`, `33:483-493`, `33:786-790` (the key table, the funnel rule, the
atom table, the unmeasured items, each re-read at the cited lines rather than through the citing
files). `35_probes/probe_1`, `36_probes/vu_nat.rs`, `36_probes/probe_2` as source, recompiled where I
built on them. The directory listed once: 37 numbered deliverables plus probe directories.

**What I compiled or measured, separated from what I reasoned.** Two artifacts in `38_probes/`, each
with a row in `38_probes/OUTCOMES.md`, both against the workspace pin (`rustc 1.98.0-nightly
(57d06900f 2026-05-27)`, confirmed from inside the repo). Probe 1 is a codegen inspection at
`-C opt-level=3 --emit=asm`, no LTO, which is file 34's corrected shape for codegen-quality questions
(`34:110-118`); no timing claim appears anywhere in this file, and the one number class I report is
symbol identity, which the assembler states rather than a stopwatch. Sections 1 and 2 are compiled or
verified by grep except where marked; sections 3 through 5 are reasoning built on those results and on
the predecessors' compiled artifacts, marked as such.

## 0. The verdict, stated first

**The three results compose, and the composition is now compiled rather than asserted.** File 35's
collapse survives file 36's encoding replacement: rebuilt on the value-unique `Z`/`Pz`/`H`/`O`/`I`
encoding, `mul_full` into the named product numeral followed by `quantize` still folds to the direct
wrapping multiply, all three symbols aliased to one `mul` instruction (probe 1). File 36's normal form
and file 37's view lattice interlock exactly as file 37 stated (`37:314-336`): type identity, datum
canonicalisation, and the computed view are three quotients at three levels, each enforced by the
mechanism its decidability permits. No pair of the three recent results is jointly inconsistent on
anything either of them compiled.

**Where they are jointly wrong is in what they wrote down, and it is the consolidation's own named
failure mode arriving in the sections marked "for the next consolidation."** The three files state
three different member lists for `Numeral`, none matching the settled contract at `31:328-333`, and
each in a section written to be taken verbatim (section 2.1). File 37's spec section fuses two atoms
file 33 had already separated, and the fusion is false on a model the design's own type-level work
makes plausible (section 2.2, probe 2). One citation in file 37 points at lines that do not exist in
the cited file (section 2.4). And two files treat as settled what file 35 itself marks as a
recommendation awaiting ratification (section 2.3). None of these is caught by any mechanism this
review has built, because they are all prose about the design's own state, which is checked by nothing
(`26:111-119`); catching them is what this dispatch is for, and section 5 states the merged shape with
each of them repaired, in the form the third consolidation takes.

## 1. The ledger: what is established, by what, at what scope

File 10's four bins, applied to the assembled design as of file 37. A claim sits in the strongest bin
its artifact supports, and moving a claim up a bin is a build, not a wording change.

### 1.1 Machine-checked by construction (width-independent, holds at every instantiation)

- **Value-uniqueness of the naturals, positives and signed integers.** Non-canonical spellings have no
  type (`36:154-186`, probes 2/2b/6, uniqueness by induction with the sealed perimeter as the
  induction's closed-world hypothesis, probes 5/5b). Scope: exactly the sealed encoding; the seal is
  the perimeter and without it the guarantee is void one crate away (`36:169-179`).
- **Coprimality of a rational adjustment.** Enforced where observed: `Ratio<Six, Twelve>` is a type
  and is not an `Adjustment` (`36:199-207`, probe 4b, E0271). Idempotence of reduction holds as a type
  identity (`36:210-218`).
- **Unstatability of the Lattner gap.** `Growth::Exact` with `Widening::None` has no spelling in the
  post-collapse vocabulary (`35:263-267`); this is removal-by-construction, stronger than the
  compatibility predicate `26:52-59` anticipated. Conditional on the removals being ratified
  (section 2.3).
- **A key mismatch on the exactness fact.** Bound as an associated const on the operation marker,
  the vocabulary to pair it wrongly does not exist (`35:204-214`, probe 2 of file 35).
- **The caller contract on published grades.** A regrouping that fails to preserve a generator class
  publishes it in its result type, and a caller that cannot accept it does not typecheck
  (`37:431-449`, E0308, probe 4d of file 37). The published grade is declared-and-checked, not
  computed; understatement refuses, overstatement is pessimistic and compiles (`37:455-460`).
- **The identity surface's refusals.** Nine compile-fail pairs pin that shapes with no representable
  one have no `Identity` impl (`crates/arvo/tests/ui/`), the one part of this stretch's subject matter
  that ships today.

### 1.2 Machine-checked by bounded exhaustion at a model width (transfer rests on the forbidden-feature list)

- **The view lattice's two closures** (downward, join), hence existence and uniqueness of a finest
  view per law: exhaustive at a three-bit signed model, arities through 4/5, nine views, all 4096
  inputs (`37:126-143`). The pullback argument generalises it; the model is what is checked.
- **The finest views of the shipped compositions**, including the two incomparable points
  (`37:169-196`) and the uniform top row at interior safety (`37:571-587`).
- **Event invariance and its `Domain` dependence**: unsigned wrapping graded-associative, signed not,
  witness (-4, -3, 3) (`37:199-225`), closing the standing unmeasured item from `33:786-790`.
- **The reification correction**: Kleene stable under an out-of-set absorbing special, nothing stable
  under `SubstituteZero`, grades identical on both sides of the flip (`37:244-281`, probe 2 of 37).
- **The two accumulator contracts** (interior safety against total safety, `34:192-219`), the biased
  MAC accumulator's four-monomial gcd (`34:221-245`), law equality as the canonical quotient at model
  scale (`34:247-277`).
- **Width arithmetic on the value-unique encoding**: the adder's 18 impls, associativity and
  commutativity as type identities, gcd at 32 instantiations, exact division, reduction over 16
  assertions (`36:255-301`).
- **The `IS_EXACT`/`Total` separation**: my probe 2, section 2.2 below. Exhaustive at an eight-value
  model.

The standing scope caveat from file 10 is unchanged and applies to every entry here: an exhaustive
check is available only at model widths (28.45 s at eight bits, refused at nine, `26:71-74`), and the
transfer to real widths is exactly what the `specialization`/`TypeId` bans purchase. Nothing in this
stretch weakened that, and file 36's sealed encoding strengthens it at the margin: a sealed,
value-unique type-level number admits no instantiation-detecting trick the bans were not already
excluding.

### 1.3 Measured on this pin, this target, this build shape (regression-test territory, not theorems)

- The three-way symbol fold of direct, composite and widening multiply, at native width on the old
  encoding (`35:107-116`), at multi-limb width (`35:126-137`), and now at native width on the new
  encoding (my probe 1). Three measurements, one optimiser heuristic class.
- The multi-limb carry chain compiling clean (`26:452-457`).
- The vectorisation results under the corrected flag discipline (`34:65-127`).
- The compile-cost sweeps: gcd 5.08 ms, full reduction 12.07 ms, dyadic 0.50 ms per composition,
  metadata 1.3/1.9 KB per composition (`36:302-352`); the view mechanism 0.130 ms and 907 bytes per
  composition against the derived-marker shape's 0.193 ms and 1854 (`37:377-405`). Both are
  `--emit=metadata` mechanism prices, and both files correctly mark them as neighbours to, not answers
  for, the real-consumer cost item at `26:668-674`, which stays open and belongs in `mock/benches/`.

Every entry in this bin is a fact about `57d06900f` on this target. The consolidation should carry
them as what they are: the design's evidence that its shapes lower well today, plus an obligation of
one codegen regression test per question class (`26:452-457`, `35:139-145`). By my count that
obligation now covers four classes: the carry chain, the fold-vs-direct multiply at native and
multi-limb width, the saturating-reduction non-vectorisation (`26:443-450`), and the vectorisable-loop
idiom (`34:119-127`). None of the four tests exists yet.

### 1.4 Trusted with nothing beneath, or reasoned without an artifact

- **`Growth` leaving `Policy`** (as distinct from leaving the key): file 35's own §2.2, marked by its
  author as reasoned, not compiled (`35:309-310`). File 37 cites it as the accomplished ground
  (`37:293`); that upgrade is drift-shaped and I return it to this bin. What is compiled is the
  key-side removal (probe 2 of 35); the axis-side removal is a type-shape argument I find correct and
  that remains an argument.
- **The `IS_EXACT`-trivialises-the-grade sentence** as written at `37:301-303`: false in general,
  section 2.2. The corrected conjunction is compiled (my probe 2); the corrected sentence's coverage
  of every future operation is, like any such sentence, a discipline.
- **The evaluation-strategy sentence** the design owes (`37:227-242`): measured to change no verdict
  in the model, unchosen, and the grade it changes is a published object.
- **File 36's §5 claim that nothing in it disturbs file 35's shapes** (`36:362-364`): was reasoned;
  now compiled (probe 1) and moved up a bin.
- **The prose restatements of the settled contract**: three files, three lists, none checked against
  `31:328-333` by anyone including their authors. Section 2.1.
- **`arvo-num-systems` and `notko-hlist`**: still unread by every member of this stretch, me included,
  now flagged by eight files. The cheapest open item in the review remains unspent.

## 2. The joint findings

### 2.1 Three "settled shape" sections state three different `Numeral`s, and none matches the settled contract

The settled identity contract, restated by file 31 from file 30 under op's D69 call and gate-verified
by file 34 against the topic files, is four members (`31:328-333`, unchanged from `30:74-81`):

```rust
pub const trait Numeral {
    type Radix:     Radix;        // 2 and 10 instantiated; any r expressible
    type Precision: Precision;    // significand digit count, primitive (D69)
    type Exponent:  ExponentForm; // nests Adjustment, Bias, Underflow, Specials
    type Domain:    SignDomain;
}
```

Against that:

- **File 35's residual-axis list** (`35:295-299`) names `ExponentForm`, `Precision`, `Domain`,
  `LogicalWidth`. It resurrects `LogicalWidth`, which D69 made derived ("precision primitive, width
  derived", `30:93-95`; "total width... derived on the physical side", `30b:10-12`) and which appears
  in neither file 30's nor file 31's contract, and it omits `Radix`, the member that closed the
  consolidation's own open radix item (`26:643-647`). The list is the old table's Numeral row with
  D69 applied to three of its five members and forgotten for the other two.
- **File 35's count sentence** (`35:295`): "Nine axes remain of the original ten." The enumeration in
  the same sentence lists eight; the correct post-collapse count against the settled contract is also
  eight (four on `Numeral`, `Quantisation` on `Policy`, three on `Lowering`), and no counting I can
  construct yields nine. An arithmetic slip, cheap to fix, expensive to cite.
- **File 36's restated trait** (`36:378-382`) has three members: `Precision`, `Exponent`, `Domain`.
  It silently drops `Radix`. Its section 5 also lists `LogicalWidth` among the "one kind of object"
  compression (`36:416`), a member that does not exist.
- **File 37** does not restate the trait and inherits nothing; its key statement names numerals as
  wholes, which is why it escapes.

No compiled result in any of the three depends on the wrong lists; every probe instantiates concrete
widths and never names the trait. The defect is confined to exactly the sections the consolidation
was invited to take verbatim, which is what makes it worth a finding rather than a footnote: this is
`26:111-119`'s prose-unchecked-by-anything failure mode, reproduced inside the convergence stretch by
three careful members in a row, each restating the contract from memory. The repair is in section 5,
and the discipline it argues for is small: **a "settled shape" section quotes the governing statement
by line, or it derives its list from one, and never does both from recall.**

### 2.2 `Op::IS_EXACT` does not trivialise the grade monoid; the conjunction with `Total<Op>` does (compiled)

File 37's §4.1, carried into its verbatim-intended spec at `37:530-532`: "`Op::IS_EXACT` is the
statement that the operation's grade monoid is trivial. An exact operation generates no causes and no
events" (`37:301-303`). File 35 defines `IS_EXACT` as quantiser presence between the exact operation
and the result (`35:204-214`, from `33:241`). Exactness kills quantisation events and
quantiser-generated causes. It says nothing about causes no quantiser generates, and the design
already names one: divide-by-zero, "a refusal cause with no home in the vocabulary" (`26:322-324`).
File 33's atom table had already separated the two facts, `Total<Op>` keyed on "resolutions only"
(`33:492`); file 37 itself uses `Total<Op>` to derive the existence equation (`37:477-479`) and then
fuses the two three sections earlier.

Probe 2 compiles the separation at an eight-value model, exhaustively: an exact-and-total operation
(the `mul_full` shape) has unit grade on all 64 pairs; a total-but-inexact operation (wrapping add)
generates events, so totality alone does not trivialise; an exact-but-partial operation (a
`div_exact` model, no quantiser anywhere, no event can ever fire) generates refusal causes, so
exactness alone does not trivialise, and two views disagree on its grades, so the nine-view collapse
fails for it. A scratch copy with one assertion negated fails compilation with E0080, so the checks
demonstrably run.

Scope stated honestly: no shipped or currently-designed operation is exact-and-partial, so nothing
file 37 measured is wrong, and every consequence it drew for `mul_full` stands. But file 36 built
exact division at the type level (`36:280-301`), its value-level twin is exactly the operation a
rational-adjustment design will eventually want, and the sentence as written would then be false by
construction rather than by oversight. The corrected sentence for the spec, one line: **an
operation's grade monoid is trivial exactly when `IS_EXACT` and `Total<Op>` both hold; `IS_EXACT`
alone kills the event generators, `Total` alone the cause generators, and each without the other
leaves a nontrivial monoid.** Section 5 carries it.

### 2.3 What is settled and what is a recommendation, sorted, because two files stopped sorting it

File 35 is precise about its own status: "the removal is a recommendation for op's ratification, not a
unilateral change to the table" (`35:322-324`). Files 36 and 37 then write "Widening removed, per file
35 section 1" (`36:398`) and "File 35 removed `Growth` from `Policy` and from the key" (`37:293`)
as accomplished facts, and 37 grounds the `Policy`-side removal in file 35's §2.2, the half its author
explicitly marked reasoned-not-compiled (`35:309-310`). This is how a held reading hardens into a
premise in two files, the exact mechanism `26:111-119` documents, and the checkpoint after me is where
it gets stopped: the table is ratified, so the table's changes are op's.

The tick-list this stretch actually owes op, each with its evidence bin:

1. **`Widening` leaves `Lowering`** (`31:347-352` still carries it). Evidence: compiled at native and
   multi-limb width on the old encoding (35 probes 1/3), at native width on the new encoding (my
   probe 1), preset cost accounting at `35:149-167`. The strongest-evidenced item on the list.
2. **`Growth` leaves the law key.** Compiled (35 probe 2, plus 33's own two rows stating one fact,
   `33:238,241`).
3. **`Growth` leaves `Policy`.** Reasoned only (`35:216-247`). I concur with the argument, and concur
   is not a bin; if op wants it compiled first, the check is the one file 35 names: no operation
   exists or is designed whose growth behaviour is not read off (primitive, target numeral).
4. **The value-unique encoding replaces the width chain** (`36:409-414`), with the seal. Compiled and
   priced; rewrite cost zero against the tree (re-verified by my grep: no shipped source names any of
   it).
5. **The finest-view mechanism replaces the three-relation fork** (`37:497-569`), with the transfer
   rule. Compiled at model width; its compile price measured against the marker alternative.
6. **The corrected `Numeral` restatement** (section 2.1) is not a change at all: it is `31:328-333`
   as ratified, defended against its own restatements.

### 2.4 The citation defects, named because a consolidation inherits citations

- `37:307` cites `35:396-403` for "file 35's own preference between its two spellings". File 35 is
  330 lines long; the lines do not exist. The content described (the funnel makes the operation-name
  spelling grep-checkable) lives at `34:399-403`. The finding survives the fixed citation; the
  citation as written points a future reader at nothing.
- `34:328-331` ("the shipped width chain already satisfies it") remains in file 34's text with no
  retraction marker; file 36 disproved it (probes 1/1b of 36). The consolidation's droplist should
  carry it so it cannot be re-cited, alongside 34's reification-stability generalisation
  (`34:176-190`, overturned at `37:244-281`) and 34's §3.3 relation ladder ("three relations,
  ordered", `34:367-377`), which the lattice with two incomparable shipped points replaces outright.
  None of 34's three dead claims is marked dead anywhere except in the files that killed them.

## 3. What survives of file 34's assembly, stated so the consolidation does not have to diff it

Superseded: §3.3 in full (the ordered ladder and the reification-stability property, replaced by
`37:497-521` and `37:244-281`). §2.6's "already met" reassurance (`34:328-331`, replaced by 36 §1).
§3.5's two held readings (both resolved by file 35, pending ratification per 2.3).

Surviving, with a second pass on each: §1's flag discipline (one flag set per question class,
independently reproduced by file 35's inline-never negative control, `35:131-137`, and observed by my
probe 1's build shape); §2.1's distributivity split, now to be restated in view vocabulary (the
partial-operation failure is a view statement: monotonicity gives the trivial view only, and the
strict/suppressing max variants remain both required per the IEEE test); §2.2's two accumulator
contracts, carried verbatim into `37:551-557`; §2.3's biased-MAC gcd, untouched and unchallenged;
§2.4's law equality as the canonical quotient, carried verbatim into `37:503-506`; §3.6's crate-level
enforcement of the value/datum split (laws live where `Lowering` does not resolve, E0433), which
neither 35, 36 nor 37 touched and which composes with all three; §3.7's cost join, extended by the
two new sweeps.

## 4. The standing risks, re-certified

**The model-narrowness hole is still open and this stretch widened what rests on it.** Nothing catches
a model too narrow to see a value disagreement (`26:104-109`); the view lattice's closures, the finest
views, the reification correction and my probe 2 all now live at model widths. Every one of them is
honest about it. The list of things a wider model could in principle contradict is growing, and the
one mitigation remains file 10's: the bans that make monomorphisation uniform, plus a member per
stretch re-running a predecessor's model at a different width or shape. Nobody has yet re-run 37's
nine-view lattice at four bits or with three generator classes; it is the newest and least-replicated
of the load-bearing models.

**Optimiser-heuristic dependencies now number four** (section 1.3) and have zero regression tests
between them. Each is one small committed test; the consolidation should stop carrying the
recommendation and start carrying the tests.

**The prose-restatement failure mode has no mechanism and got three new instances** (2.1). The
quote-or-derive discipline in 2.1 is a discipline, and I note, without proposing to build it now, that
the settled contract is exactly the kind of text a design-doc lint already checks backticked names
against source for; a variant checking a research file's restated trait against a governing file's
stated one is the same shape. That is a mockspace question, not an arvo one, and I leave it named.

## 5. The assembled shape, stated for the third consolidation (reasoned on the compiled results)

What follows merges `31:328-355`, file 35's removals, file 36's encoding, and file 37's law
statement, with sections 2.1 and 2.2's repairs applied. Ratification markers per section 2.3.

```rust
// Every member that denotes a number is drawn from one value-unique, sealed,
// type-level encoding (file 36):
//   Nat ::= Z | Pz<P>            P: Pos       precision, widths, exponent bounds
//   Pos ::= H | O<P> | I<P>      P: Pos       magnitudes
//   Int ::= Z0 | Zpos<P> | Zneg<P>            biases
// No normalisation operator exists anywhere; non-canonical spellings have no
// type. Pos, Nat, Int are sealed; the seal is the perimeter of the guarantee.

pub const trait Numeral {                 // ratified: 31:328-333, unchanged
    type Radix:     Radix;                // 2 and 10 instantiated; any r expressible
    type Precision: Precision;            // a Nat; significand digit count, primitive (D69)
    type Exponent:  ExponentForm;         // Implicit<E, A: Adjustment, B: Bias> |
                                          //   Ranged<EMIN, EMAX, U: Underflow, S: Specials>
    type Domain:    SignDomain;           // NonNegative | Symmetric | AsymmetricLow
    // LogicalWidth is DERIVED (D69), not a member. Nothing else is a member.
}
// Adjustment = Ratio<N: Pos, D: Pos> with the impl conditional on
// N: Gcd<D, Out = H>; the consumer spelling is the normalising alias
// Reduced<N, D>. Bias is an Int: one zero, by construction. Signed zero is a
// datum fact and lives in Encoding::Canonical. (file 36, adopted per 34b.)

pub const trait Policy {                  // pending ratification (tick 3, section 2.3)
    type Quantisation: Quantisation;      // the single approximator; unchanged
    // Growth removed: a relational fact about an operation's signature, not a
    // unary fact about one numeral; it lives in the operation names
    // (mul_full, add_exact, quantize::<Src, Dst>).
}

pub const trait Lowering {                // pending ratification (tick 1)
    type Encoding:    Encoding;           // SignIndexing, Fields, Canonical; unchanged
    type StoredWidth: StoredWidth;        // a Nat, same encoding
    type Layout:      StorageLayout;
    // Widening removed: every exact intermediate is a named Numeral (mul_full's
    // return type, a fold's checked accumulator), with its own StoredWidth and
    // Layout. Measured to cost nothing at native and multi-limb width, on both
    // encodings (35 probes 1/3, 38 probe 1).
}
```

The laws, taking file 37's section 7 with one sentence corrected: law equality is the canonical
quotient; a term's meaning is a grade (free commutative monoid over refusal causes and quantisation
events) and a value; a view is a monoid homomorphism out of the grade; every law reports the finest
view under which it holds, which exists and is unique by the two compiled closures; the lattice is
not a chain and two shipped presets sit at incomparable points. A law is a `const fn` whose
parameters are its key and whose return type is `Never` or the finest view, derived not declared
(D51). The key: the operation, whose marker carries `IS_EXACT` (quantiser presence), **with the grade
monoid trivial exactly when `IS_EXACT` and `Total<Op>` both hold, and neither alone sufficient (38
probe 2)**; the operand numerals and, for widening operations, the result numeral; the `Quantisation`
resolutions and, where a quantiser is present, its `Direction`; for folds, the accumulator numeral
and arity. `Growth` is not in the key; `Lowering` cannot be named where laws live. Interior safety is
the law's condition, total safety the specification's, related by the refinement order, which is the
conformance relation and not a law relation. The transfer rule: a regrouping publishes, in its own
result grade, exactly the generator classes its law fails to preserve; tolerance is a transfer, never
a waiver; a weak-equation failure is refused outright. The design owes one stated sentence on the
evaluation strategy of a refusing operand's sibling, measured to change grades and no verdicts.

The droplist gains, from this stretch: file 34's ordered relation ladder and its
reification-stability property; `34:328-331`'s "already met"; referential uniqueness in place of
value uniqueness (`36:240-253`); the subset-domain view parameter (`37:115-124`); the
consumer-declared required view (`37:407-429`); `LogicalWidth` as a `Numeral` member (derived per
D69; resurrected once by restatement-from-memory, section 2.1); and `IS_EXACT` as a sufficient
condition for a trivial grade (38 probe 2).

## 6. What this file does not decide

The five ratification ticks in section 2.3 are op's, and this file's contribution to them is the
evidence sort, not a preference. The what-is-`Precise`-for question stands where files 33 and 37 left
it, now with the mathematics settled under it. The `TotalOrd` level annotation, the dither/`Refuse`
choice, D39's honest content, the reduction firing site (`36:433-437`), the exponent-as-type fork
(`36:445-448`), the evaluation-strategy sentence, and division stand exactly as their files left
them. I have not re-run any predecessor's model at a different width (section 4 names the gap); my
two probes are new evidence, not replication, and the distinction matters to the bin they land in.

## 7. Open, net

Closed by this file, each with an artifact or a line citation: the untested composition of files 35
and 36 (probe 1, compiled, moving `36:362-364` up a bin); the `IS_EXACT` fusion (probe 2, with the
corrected conjunction); the three-way `Numeral` restatement drift and the count slip (section 2.1,
grep against `31:328-333`); the ratification sort (section 2.3); the dead citation and the unmarked
dead claims (section 2.4). Opened by this file, none: every defect found came with its repair in the
same section, and the assembled shape in section 5 is the deliverable.

Standing from the predecessors, unchanged and untouched by me: the real-consumer compile-cost bench
(`26:668-674`); division (`26:676-681`); richer canonicalisation branchlessness (`32:341-350`); the
four missing codegen regression tests (section 1.3, consolidated count); `arvo-num-systems` and
`notko-hlist` unread by anyone across two consolidations, which I also did not read, and which I flag
with the observation that this review has now built two type-level-number towers and one
type-level-set mechanism while the workspace's own type-level-list crate sits unexamined at design
stage; the next member who touches any type-level mechanism should start there.

## 8. Standing

Nothing here overturns a D-numbered call, `30b`, or `34b`. Where I certify a predecessor's claim as
unestablished, the citation names the claim and the artifact that unsettles it, and where I found a
defect the repair is in the same section, per the convergence directive. The two files I correct are
files 35 and 37, on points (a member list, a fused sentence, a citation) that none of their compiled
results depend on; their compiled results I re-verified where cheap, extended where the extension was
the missing joint check, and carry forward as the strongest material this stretch has produced. My
own prior file's rational adjustment needed file 36's repair, and this file's section 2.1 discipline
(quote, do not recall) is written by someone who watched three members better-calibrated than most
restate one four-line trait three different ways in nine days. The failure mode is not carelessness;
it is that prose about the design's own state is checked by nothing, and the only defence found so
far is the one exercised here: re-grep the claim before building on it, and before consolidating it.
