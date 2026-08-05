# The missing declarations: what the standing base decided and never wrote down

**Date:** 2026-08-05
**Position in the panel:** after `115_fresh_read_a.md`, `116_fresh_read_b.md` and
`117_spj_fused_or_split.md`. This file is not a fourth read. It is the record of closing the gap the first
two name and the sentence the third names, and of what it refused to write.

Two readers were handed `110_consolidation_eleven.md` and nothing else, worked independently, never saw each
other, and returned the same verdict: it cannot be implemented from. The convergence is worth more than either
read, because the two took different routes to it. Reader A worked forward from "where would I start" and got
as far as a handful of stub traits before stopping; reader B built an inventory of every named type and marked
each defined, named-open, or assumed, and found a third category nobody had counted. **Both stopped at the
same place, and it was not a design question.** `Number<N, S>`, the object the whole design is organised
around, had no declaration anywhere in 5,867 lines, and around it sat a vocabulary of names used as bounds
with nothing to bind to.

That is a good failure to have. A design that cannot be built because its central mechanism is undecided is in
trouble; a design that cannot be built because nobody wrote down the decisions it already made is one pass
from being buildable. **Everything in this file is that pass. Nothing in it is a new decision**, and the five
places where writing one down would have required a decision are on `110`'s open list addressed to op rather
than in its text.

## What landed, in one line

`110` went from 5,867 lines to 6,283, `wc -l` on 2026-08-05. **Seventeen edit sites, eighteen inline marks**,
in the form the file already uses: a blockquote beginning "Correction, file 118" stating what was wrong or
missing and what the text now says. Nothing was silently overwritten, including at the four sites where the
repair is purely additive and the text above it does not change; those say so in their own marker.

**Twenty-six declarations land**: twenty-three traits, two types (`Number` and `Folded`), and one associated
type added to a ratified trait (`Lowering::Container`, a projection rather than an axis), together with the
twenty-six constructor markers the sealed vocabulary needs. **Four counts that disagreed with their own lists
are fixed by enumerating**, never by adjusting the number. **Two code blocks that do not compile are
repaired**, and both repairs are compiler output rather than opinion. **Five questions go to op and four
artifacts go to section 5**, each one line.

## The gates, both of them, and the honest state of the second

**The canon gate passes.** The governing material is op's own: `13c`'s standard, the four standing directives,
`16b`'s posture correction, `108b`'s two standing principles, and the numbered register. The work this file
was dispatched for is what `16b` asks for in its own words, "design the fucking shape it should be", and what
`108b:184-186` orders for the remaining stretch, working the open list down rather than opening ground.
Writing declarations is the form `110` itself already uses for shape: its trait table is Rust, and the
question here is only that the table stopped halfway. **Nothing in this pass touches `mock/crates`**, per
`108b:188-193`.

**The test gate, run rather than cited.** `cargo test --offline --workspace` from inside the tree, 2026-08-05:
**155 binaries, 672 passed, 0 failed, 9 ignored**, summed per binary rather than read off a headline. That
reproduces exactly what files 102 through 108 each report independently, on a tree nothing has moved.

**I did not audit the test bodies**, and I want to be exact about why rather than wave at the brief: op
ruled at `108b:174-181` that the tautological tests found in this review are
**collected, not acted on**, and become an implementation-phase checklist, with deletion the default when that
phase opens. `110` section 4 carries that checklist. An audit here would produce a fourth report of the same
three findings, which is what op's ruling exists to stop. **The suite covers a tree the canon replaces**, and
the surface this pass touches has no tests because it is a design document.

**One thing about the suite is worth saying anyway, because it bears on this pass rather than on the tests.**
A green suite over the shipped tree says nothing about whether the canon is buildable, and the two cold reads
are the only instrument in this review that measured that. They are worth more than the pass rate, and the
correct response to them was not to argue.

## 1. `Number`, and an arity conflict that was never a conflict

The declaration, at section 1.1, where the document's only description of the type lives:

```rust
pub struct Number<N: Numeral, S: Policy + Lowering> {
    datum:    <S as Lowering>::Container,
    _numeral: PhantomData<N>,
}
```

**Two parameters, settled from the document's own evidence and not from preference.** Section 1.3 states that
`Encoding` nests inside `Lowering` **specifically so that a third parameter is never paid**, on `26:32-35`'s
measured 1.8x diagnostic cost; D72's crate table gives `arvo-numeric` the row "`Number<N, S>`"; and `117:47-52`
separates the two questions that had been read as one, finding the parameter count closed on measurement while
the trait count stays open. Three independent supports, no contrary evidence.

**The three-argument spelling both readers found is file 09's own probe topology.** `09` was working a
three-parameter model throughout, in which the lowering is a free phantom parameter, and `110` quotes its
`Number<Fix13_3Signed, Warm, MinWidth>` verbatim without saying whose type that is. Both readers took it for a
second arity of the shipped type and neither could reconcile it, which is exactly right: it does not
reconcile, because it is a different type with the same name. Section 1.25 now says so at the point of use.
**This is worth a line of its own as a failure mode**: the quotation was faithful, the citation was correct,
and the effect was to put a contradiction into the document that nothing in it could resolve.

**The bound is the sentence `117` went looking for and could not find.** Four searches, zero hits in 5,900
lines. `S` is one type that implements two of the three contracts, and the document declares three contracts
and a two-parameter type and never joins them. The only statement of the join anywhere in the record is
`26:28-35`, which is agent output. It is now stated in `110` on that basis, **flagged as owed op's word**, and
tied to the fused-versus-split call he reserved at `08b:47-51`, because both are questions about the same
parameter and answering them together costs one sitting.

**`Container` is the one addition to a ratified trait, and it is a projection rather than an axis.** Section
1.22 already states that the container level is a type-valued projection of the stored width, derived, and
never declared as an axis. `Number` has to hold something, and the something is that projection's result. It
is spelled as a `Lowering` member because that is where its inputs already are; `117:452-458` argues it should
move to a one-member carrier contract below `Lowering`, on grounds about the residual discrimination surface,
and that is on the open list. **The declaration above is unchanged under either**, which is why it was safe to
write.

## 2. Nineteen names used as bounds, none of them declared

Section 1.23 is titled "the assembled trait table" and declares four contracts whose members are bounded by
nineteen further traits, not one of which existed as a declaration anywhere in the document. A reader could
see that `Numeral::Precision` is bounded by `Precision` and had no route to what `Precision` is. The BNF
comment at the head of the same section is the same gap from the other side: the carrier the entire design
rests on, given as a closed and unambiguous enumeration **inside a comment**.

All nineteen are now declared in the section that already names them, each from the text that determines it:
`Pos`, `Nat`, `AtLeastTwo`, `Exponent`, `Radix`, `Bias`, `Gcd`, `Dec`, `Precision`, `Capacity`, `Adjustment`,
`ExponentForm`, `SignDomain`, `SignIndexing`, `FieldLayout`, `Canonicalisation`, `StoredWidth`,
`StorageLayout`, `LoweringDoor`, with their constructors, plus `Folded`.

Three of the nineteen deserve their reasoning stated, because they are the ones where a lazier answer was
available.

**`Precision`, `Capacity` and `Adjustment` are named semantic aliases over the carrier, not second
encodings.** `74b` settled exactly this shape for `Capacity`, one sealed bottom carrier with a named alias
over it, and section 1.27 states the payoff in its own words: one seal, one ordering, one arithmetic,
inherited wholesale. `Precision` is described at section 1.27 as "of the same kind as" a capacity, and
`Adjustment` is stated at section 1.2 to be a signed gcd-normalised rational, which is the `Bias` carrier's
own grammar. Declaring any of the three as its own encoding would have minted a second arithmetic the design
spent a checkpoint removing.

**`FieldLayout` gets one member and `Canonicalisation` gets none.** The document says what facts they carry
(field widths, the hidden bit, the encoding bias, reserved codes; signed zero, preferred cohort, NaN
canonicalisation) and never how those facts are spelled. `FieldLayout` gets `type Extent: Precision` because
section 1.22 fixes it: the fields extent `W_F` is statement 0's own quantifier domain, and statement 0 cannot
be stated without it. The rest is on the open list. **Writing the member lists would have been the exact
failure this pass exists to avoid**, since a plausible invention in the canon's voice is indistinguishable
from a decision, and the format instantiations that need those members do not exist yet to argue back.

**`Folded`'s parameter is a const, and the document's own compiled evidence settles it.** A moved count
offered to it is refused with `E0435`, which is the error for a non-constant value in a constant position and
is reachable at no other spelling; a `Nat`-typed parameter would have produced a name-resolution error
instead. The site count was already settled and confirmed by op; only the spelling was unstated. Where the
witness rides, as a returned value or as a parameter on the fold's result numeral, is one line on the open
list.

## 3. The truth contract had three organs and declared none of them

Section 1.30 locks a shape, in op's own confirmed words, that names an algebra, an exit and a selector, states
which of the three the fifteen declarations bind on, and works an example against two of them. **None of the
three was declared**, and `Compare` and `Select` were not on section 9's named-open list either, so a reader
had no signal that they were missing until the example failed to build. All four (`Truth`, `Branch`,
`Compare`, `Select`) are now declared at the lock, every member read off the locked statement:

- `Truth`'s five members are the statement's first sentence, and its working name is the one the section
  already uses in prose. **The contract's real name stays on the open list**, where it already was.
- `Branch` is the name in the section's own compiled diagnostic, and `is_true` is the member its
  five-introduction-route enumeration spells.
- `Compare`'s associated `Truth` and its `lt` are what the worked example projects and calls. **Only `lt` is
  written**, because only `lt` is evidenced; the rest of the comparison surface is noted as having the same
  shape rather than invented.
- `Select`'s three arguments and its associated type are the statement's fourth paragraph, and the
  associated-type spelling rather than a type parameter is forced by the example's own
  `Select<Truth = <T as Compare>::Truth>`.

One consistency check came free and is stated in `110` because it is cheap to lose. **The supertrait edge runs
from the exit up to the algebra**, so a crate bounding on `Truth` never gains a route to `Branch`. `117:355-366`
compiles the reverse case on the strategy side and finds that projection travels through a supertrait edge
that nameability does not, so the `E0432` check reports closed while the design is open. The same shape here
would put the exit back on every producer, which is the outcome op's own correction at `108b:130-144` reversed.

## 4. Two blocks that do not compile, and what that is worth

**Section 1.2's identity contract.** The two structs carrying `Numeral`'s shape were unit structs with
unused type parameters:

```
error[E0392]: type parameter `E` is never used
 --> p1_asdeclared.rs:9:21
  |
9 | pub struct Implicit<E: Exponent, A: Adjustment, B: Bias>;
  |                     ^ unused type parameter
  |
  = help: consider removing `E`, referring to it in a field, or using a marker such as `PhantomData`
```

Seven instances across the two declarations. The repair is rustc's own third suggestion and it is taken.
**This is the most-cited code block in the document**, restored twice, quoted in the trait table a second
time, and it had never been through a compiler.

**Section 1.30's worked `max`.** The document's one operational code sample, presented as the resolution of a
locked fork with a compiler diagnostic quoted in support:

```
error[E0382]: use of moved value: `b`
 --> p3_max.rs:9:24
  |
7 | pub fn max<T>(a: T, b: T) -> T
  |                     - move occurs because `b` has type `T`, which does not implement the `Copy` trait
8 | where T: Compare + Select<Truth = <T as Compare>::Truth> {
9 |     T::select(a.lt(b), b, a)
  |                    -   ^ value used here after move
  |                    |
  |                    value moved here
note: consider changing this parameter type in method `lt` to borrow instead if owning the value isn't necessary
```

A by-value comparison moves the operand the selector still needs. The repair is the borrowing signature, which
is rustc's own note, rather than a `Copy` bound this document states for no numeric type anywhere. One
character changes in the example.

**Both were found by writing the declarations, not by looking for them**, and that is the argument for this
pass having been declaration-shaped rather than prose-shaped. A design stated in prose cannot be wrong in this
way, which sounds like an advantage and is the opposite of one.

**What compiles now.** The whole of section 1.23's trait table, plus section 1.2's two structs, plus section
1.1's `Number`, plus section 1.30's four traits and the corrected `max`, built as one crate under the pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024 --crate-type=lib`, **exit 0**, one dead-code
warning on `Number`'s own field, which is what a declaration-only crate produces. Probes are in a scratch
directory outside the tree, since the dispatch forbade adding files; each is short enough to retype and the
invocation is one line.

One incidental fact from getting there is recorded in `110` because it will bite whoever writes this in
source: **a struct cannot carry a `[const]` bound at all** (`error: [const] is not allowed here`, `note:
structs cannot have [const] trait bounds`), so every carrier constructor takes its parameter's bound plain
while the traits keep theirs.

## 5. Four counts, fixed by enumerating

The document's own thesis, restated at least three times and applied to a predecessor in its opening section,
is that a count cannot be checked and a list can. It failed that thesis four times, and one of those is in its
final accounting of itself.

| Where | Said | Its own list holds | Fixed by |
|---|---|---|---|
| Section 1, head | twenty-nine subsections | thirty, `1.1` through `1.30` | stating the range, which the headings check |
| Section 1.10 | eleven operations | thirteen operations, ten growth traits | enumerating both from the probe |
| Section 8 | nine unrestored items | thirteen, across nine bullets | splitting the bullets, and sub-listing the four that share one reason |
| Section 9 | no term left undefined | false on the document's own text | declaring them, and listing what stays open |

**The operations one is worth its own sentence, because the count is wrong at the source and not in
transcription.** `51:56-64` says eleven, `51:245` says "none of the eleven trait declarations", and
`51_probes/probe_1_growth_surface_enumeration.rs` declares **ten** growth traits over **thirteen** operations,
counted from the file: the four in-numeral operations share one growth trait because none of them grows.
Eleven matches neither of its own two candidate lists. Nothing downstream moves, because the claim the design
actually leans on is the structural theorem, which quantifies over every operation the design could have
rather than over this list. **That is precisely why the miscount survived every carrier that repeated it: it
was load-bearing for nothing except checkability.**

## 6. D68, reconciled rather than reopened

Reader B reports a ratified decision contradicted by a code block the document presents as settled, and asks
which is current. **The nested shape is current, the supersession is already stated, and it is not reversed.**
Both of the nesting's grounds postdate D68: `Underflow` having no bottom to fall off under a constant
exponent, and D69's overturn at `30b`, which makes `Precision` and `Radix` primitive top-level members that
D68 could not have been ruling on. `108b:11-20` is exactly the principle for this case, a ratification made
under the evidence available at the time, and file 114 already stated the supersession rather than performing
it silently.

**What was actually wrong is smaller and worse.** The code block carried no sign of any of it. A reader who
takes a compiling Rust item as the settled shape, which is what a reader of a specification does, had no route
from that block to the standing conflict forty lines below it. The block now says so on its first line.
**Op's acceptance of the supersession is still owed and still on the open list**; nothing here rules on it.

## 7. What I refused to write, and why the refusals matter more than the declarations

Five things a declaration would have required a decision for. Each is one line on `110`'s open list addressed
to op, and each is stated there with both spellings where two exist.

**The bound on `S`.** Written down, because a two-parameter type with no stated relation between its second
parameter and two of its three contracts cannot be built against at all. **But it is written as the record's
own sentence, marked as agent-origin, and marked as owed op's word.** The difference between that and adopting
it is the whole of this file's discipline.

**How a preset carries two rows.** This one nobody had found, and it is the most interesting thing in the
pass. One preset name denotes two rows, one per number kind: the document states it as a boundary sentence in
its own words, and the two ratified tables differ at four cells, including the sharpest single cell the
presets have. **A nullary associated type on one marker cannot be a function of two things.** Under the
document's own layer-keying rule this is not a small inconsistency, it is the rule's **dual failure**, named
for the first time in section 1.30 and immediately instantiated by section 1.21 without anyone noticing: a
fact keyed on something that does not determine it, which is a non-function presented as one. Two spellings
close it, keying the two contracts on the numeral exactly as `Crosses<N: Numeral>` already is, or mapping each
preset to a per-kind marker inside the semantic aliases, which D53's alias mechanism admits. **One line either
way, and the bound in section 1.1 is the only text that changes.** I picked neither, because the record
contains neither.

**How a public const width reaches a type-level `Nat`.** D48 and D31 keep `UFixed<13, 3, Warm>` spelled that
way with its widths as const parameters; the numeral it composes over needs `Precision` as a `Nat`; and
`I + F` in type position is a const expression, which section 1.2 has already compiled shut in both
directions. **So D53's "four names for four compositions" has no stated expansion**, and that is the gap
between the ratified alias half and anything a consumer can write. I did not leave this one as a complaint. A
generated table plus the tower's own addition supplies the bridge and it is **compiled for this file, exit 0,
with no feature gate at all**, carrying `<Precision<13, 3> as Nat>::VAL == 16` through a const assertion, with
one spelling detail recorded because it costs a confusing diagnostic otherwise (a const forwarded into a
table's index position needs braces, `NatOf<{ I }>`, or `E0747`). **What is open is not whether the bridge
exists but whether it is the design's answer and where the table is emitted**, which is a binding-time
question the design already has a rule for.

**`Dec` or `PosPred`.** One construction, two spellings, in one sentence, which by the document's own widened
completeness line means it is defined nowhere. A naming call, one word, op's.

**`Exponent` against `ExponentForm`.** `Numeral`'s member named `Exponent` is bounded by `ExponentForm`, while
the trait named `Exponent` is the sealed signed integer, and `Implicit`'s own parameter is bounded by that
one. One token, two ratified definitions, different content: address one of section 1.26's own three
mechanical addresses. A rename or a relocation, one line, op's.

Four more are artifact-shaped rather than op-shaped and went to section 5 with their closing artifacts: the
rest of the tower's arithmetic family, of which `Cmp` is the one member that cannot be declared because the
design has no type-level ordering vocabulary for it to answer in; `FieldLayout`'s and `Canonicalisation`'s
remaining members; where `Folded`'s witness rides; and where the container projection lives.

**And four names were reclassified rather than declared, because declaring them would have invented a type for
a usage.** `SameFaceAs` is a probe's own spelling inside a quoted diagnostic, and the design content at that
site is the lever (restate a comparison as a bound rather than an equality) rather than the trait; the
diagnostic stays because it is evidence, and the name is now labelled. `Mask<W>` is the design's existing
lane-mask family used as the worked instance, not a type section 1.30 introduces. `TruthAlgebra<N>` belongs to
the priced alternative and is not a member of the locked shape. The platform crate's five wrappers are quoted
tree facts, and **their declarations are deliberately absent**, because which of `Bool`'s doors survives is
op's call: writing the declaration would be making it. That last one is the only place in this pass where the
missing declaration is the finding rather than a gap in it.

## 8. Second reads owed, named because one expert's word is not a call

Three things in this pass are derivations rather than transcriptions, and the workspace's own rule is that a
call about what the canon permits needs two independent agreements grounded in quoted text. **I am the first
read on all three.**

1. **That the bound on `S` is `S: Policy + Lowering`.** Grounded in `26:28-35` plus the absence of any
   contrary statement, which is weaker than it sounds because the absence is what makes it a question.
2. **That a preset's row is a function of the pair and the current members cannot carry it.** Grounded in the
   two ratified tables, the boundary sentence at section 1.26, and the dual-failure statement at section 1.30.
   The entailment is what I am confident about; **the choice between the two spellings is not mine and I did
   not make it.**
3. **That `Precision`, `Capacity` and `Adjustment` are aliases over the carrier rather than encodings of their
   own.** Grounded in `74b` and section 1.27, and the least contestable of the three.

## 9. Could a fresh reader implement from `110` now

**From the numeral tower, the identity contract, the strategy contracts, the quantiser and the truth
contract: yes.** Those five surfaces now have declarations, they compile together as one crate under the pin,
and every member traces to a sentence in the document rather than to a member's judgment. A reader who starts
where reader A started would not stop where reader A stopped.

**From the consumer-facing surface: not yet, and the reason is one item.** The four semantic aliases are what
a consumer writes, and their expansion has no stated form until op says whether the const-to-`Nat` bridge is
the mechanism and where the table is emitted. The bridge compiles, so this is a decision rather than a
research problem, and it is one line.

**And the two structural questions were already op's before this pass and still are**: the array grammar's
three columns, which block the capacity crates as a group, and the fused-versus-split call, which now has a
subject it can be answered about. Neither moved, and neither should have moved here.

The honest summary is that the document was one pass from buildable and is now one decision from it, with
five one-line questions and four artifacts between here and a reader who can start work without asking
anything. **That is a different position from where the two cold reads found it**, and the distance was
entirely bookkeeping the design had already done and never written down.
