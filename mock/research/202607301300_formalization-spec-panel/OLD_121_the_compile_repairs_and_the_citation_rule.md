# The compile repairs, the block sweep, and a citation rule that had to be checked before it could be offered

**Date:** 2026-08-05
**Position in the panel:** after `118_the_missing_declarations.md`, which found the two compile failures this
file was dispatched to fix. It fixes neither, because file 118 already did, and the useful part of the
dispatch turned out to be the part nobody had asked for as the headline.

The brief for this pass named two live compile failures in `110_consolidation_eleven.md`, gave their line
numbers and their error codes, and asked for them to be reproduced, repaired and written up. **Both had
already been repaired, in the committed tree, by the file this one follows.** The first thing done here was
therefore to try to break the brief rather than to execute it, and the second was to establish what the
brief's premise was actually true of.

That is worth stating plainly rather than buried, because the failure mode it avoids is the one this panel
has already suffered once: a pass that inherits a false premise and reasons confidently inside it produces
work in whatever direction the premise pointed. The premise here was not false in substance. It was **stale
by one file**, and the diagnostics it quoted are exactly the ones file 118 recorded. Reproducing them was
still worth doing, because a repair verified against the actual compiler output is worth more than a repair
verified against a description of it, and because doing so turned up something about the first repair that
file 118 did not record and that changes how well founded it is.

## What the brief said, and what the tree says

**Failure A, section 1.2's identity contract.** The brief quotes
`pub struct Implicit<E: Exponent, A: Adjustment, B: Bias>;` and predicts seven `E0392`s. **The committed
file has carried `(PhantomData<(E, A, B)>)` since file 118**, at `110:840`, under an inline mark that states
the repair and quotes the diagnostic. Reverting to the form the brief quotes and building it reproduces the
prediction exactly: **seven `error[E0392]`, three on `Implicit` and four on `Ranged`**, first one verbatim:

```
error[E0392]: type parameter `E` is never used
  --> regress_A.rs:89:21
   |
89 | pub struct Implicit<E: Exponent, A: Adjustment, B: Bias>;
   |                     ^ unused type parameter
   |
   = help: consider removing `E`, referring to it in a field, or using a marker such as `PhantomData`
```

**Failure B, section 1.30's worked `max`.** The brief quotes `a.lt(b)` and predicts `E0382`. **The committed
file has carried `a.lt(&b)` with a borrowing `lt` since file 118**, at `110:4467`. Reverting both halves
reproduces it:

```
error[E0382]: use of moved value: `b`
   --> regress_B.rs:209:24
    |
207 | pub fn max<T>(a: T, b: T) -> T
    |                     - move occurs because `b` has type `T`, which does not implement the `Copy` trait
209 |     T::select(a.lt(b), b, a)
    |                    -   ^ value used here after move
    |                    |
    |                    value moved here
note: consider changing this parameter type in method `lt` to borrow instead if owning the value isn't necessary
   --> regress_B.rs:195:22
    |
195 |     fn lt(self, rhs: Self) -> Self::Truth;
    |        --            ^^^^ this parameter takes ownership of the value
```

Both repairs build. The evidence for that is the sweep below rather than a separate claim, since both blocks
are in the assembled unit.

## The judgement the brief asked for on Failure A, answered, with one thing file 118 missed

The brief was right that Failure A is not purely mechanical: `PhantomData` is one answer and the design might
have wanted another. Reading what `110` says the type is for settles it, and settles it more firmly than file
118 did.

`Implicit` and `Ranged` are the two constructors of `ExponentForm`, a sealed trait, and section 1.2 states
that every exponent position is a type. They are type-level markers. **`PhantomData` is not an outside
suggestion here, it is the document's own idiom at this exact position**, carried by nine other sealed
constructors in the section 1.23 block: `O<P>`, `I<P>`, `Pz<P>`, `EPos<P>`, `ENeg<P>`, `Rad<P>`, `BPos<N, D>`,
`BNeg<N, D>` and `HostFloat<E>`. The two identity structs were the odd ones out. File 118 took the repair on
rustc's authority, which is a weaker footing than the one available.

**And the two forms are not equivalent on the axis the design cares most about.** A unit struct is nameable
and constructible as a value by any downstream crate. The tuple form carries a private field, so the same
attempt refuses across a crate boundary:

```
error[E0423]: cannot initialize a tuple struct which contains private fields
 --> consumer2.rs:4:55
  |
4 | pub fn try_build() -> Implicit<EZero, BZero, BZero> { Implicit(PhantomData) }
  |                                                       ^^^^^^^^
  |
note: constructor is not visible here due to private fields
```

Compiled as two crates, which is the only arrangement in which the distinction exists at all. **So the
pre-repair form was not merely unused-parameter noise: it left a value door open in a vocabulary whose seal
is what section 1.12 spends eleven firings establishing.** The repair closes it, and closes it for free.

**One question is left rather than answered**, and it is deliberately not on op's list: whether these markers
should be uninhabited outright rather than merely unconstructible. The document nowhere says, the answer
would apply to all eleven constructors at once, and the perimeter is closed under either, so it is a
refinement rather than a gap. It is recorded here so that whoever writes this in source sees the question was
asked.

## The sweep, which is where the dispatch's value actually was

Every claim of the form "this compiles" anywhere in this document has been made about a block someone had a
reason to build. **No pass had ever extracted the blocks mechanically and tried all of them.** That is why
file 118's two defects fell out of writing declarations rather than out of looking for them, and it is why
the one below survived a full audit, a full restoration, two independent cold reads and file 118 itself.

**Nine fenced blocks, extracted by position.** Eight are tagged `rust`, at `110:758`, `830`, `940`, `3001`,
`3068`, `3190`, `4464` and `4581`. The ninth, at `2401`, is a rustc diagnostic transcript and is not a
compile target.

**All eight fail in isolation, and not one of those failures is a defect.** Every one is `E0405`, `E0425` or
`E0433` on a name that another block declares. That is the expected shape for a specification whose
declarations are distributed across its sections, and reporting it as eight failures would be reporting on
the document's format rather than on its content. The honest unit is the assembly.

| | Count |
|---|---|
| Fenced blocks in `110` | 9 |
| Compilable Rust blocks | 8 |
| Blocks that build standalone | 0 |
| Blocks needing the assembly | 8 |
| Assembly result | `exit 0`, one dead-code warning |
| Blocks failing against their own prose | 1 |

**Three additions were needed to assemble it, and the document states none of them.** A
`#![feature(const_trait_impl)]` gate, without which all twenty-three `pub const trait` declarations are
`E0658`. A `use core::marker::PhantomData`, which nine constructors and both identity structs name. And a
stand-in for `notko::ConstTry`, the single out-of-document name any block references
(`Quantisation::Fallibility`, `110:3213`). None of the three is a finding on its own: two are ordinary
crate-root material and the third is a real dependency the crate table already carries. **What is worth
recording is that file 118's `exit 0` and this one both rest on all three silently**, so a reader who
assembles the blocks exactly as written meets twenty-nine errors before reaching anything the document
decided. That is now stated in `110`'s section 9.

**The assembly is `exit 0`** under the pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`--edition 2024 --crate-type=lib`, one dead-code warning on `Number::datum`. That reproduces section 1.23's
own claim at a wider scope than section 1.23 makes it: file 118 built sections 1.1, 1.2 and 1.23, and this
adds 1.3's `Encoding`, 1.23's quantiser block, and 1.30's four truth-contract declarations with the worked
`max`.

### Two things an `exit 0` on declarations does not establish, checked rather than assumed

A declaration-only crate compiling is weak evidence, and the weakness is exactly the one that would hide a
design error: a vocabulary that no instance ever passes through can be incoherent and still build.

**First, that the tower carries an instance.** A concrete `Numeral` was written from the declared
constructors, with `Radix = Rad<O<H>>`, `Precision = Pz<O<O<O<O<H>>>>>`,
`Exponent = Implicit<EZero, BZero, BZero>` and `Domain = NonNegative`, and the type-level readout the whole
design rests on was asserted at compile time: `<<U13F3 as Numeral>::Precision as Nat>::VAL == 16`, exit 0.
**The vocabulary is not vacuous**, and the const-assertion route works.

**Second, that the worked `max` does what section 1.30 says.** It was implemented at one lane over a `bool`
wrapper and at two lanes over a two-lane mask. Both compile, and the two-lane result at `a = [7, 2]`,
`b = [3, 9]` is `[7, 9]`, which is the answer the section states and neither of the two wrong reductions it
prices at `110:4481-4483`. **The section's headline claim is confirmed rather than repeated**, which matters
because it is the document's one operational sample and it is presented as the resolution of a locked fork.

### The one defect the sweep found

**Section 1.23's bridge paragraph names a type this document declares as a trait.** The paragraph at
`110:3247-3259` closes D53's alias expansion, and section 9 names that bridge as the single item standing
between a fresh reader and the four semantic aliases a consumer actually writes. It states its result as
`<Precision<13, 3> as Nat>::VAL == 16`. The block at `110:3104` declares `pub trait Precision: Nat {}`, a
nullary marker trait with a blanket impl over the carrier, and that is the only declaration of the token in
the document. Written as the paragraph spells it, against the document's own declarations:

```
error[E0782]: expected a type, found a trait
   --> probe_precision.rs:258:24
    |
258 | const _: () = assert!(<Precision<13, 3> as Nat>::VAL == 16);
    |                        ^^^^^^^^^^^^^^^^
    |
help: you can add the `dyn` keyword if you want a trait object
```

**Rustc's suggestion is unavailable**, since `dyn` is forbidden across arvo, so the diagnostic offers no
repair the design admits. The paragraph's other named type, `NatOf<{ I }>`, is undeclared as well, and the
two are presumably one construction under two spellings, which by the document's own widened completeness
line means it is defined nowhere.

**This is the third instance of the one-token-two-meanings defect**, joining `Exponent` against
`ExponentForm` (section 1.2) and `Dec` against `PosPred` (section 1.27). **It is the first of the three to
sit on the consumer surface.** Two spellings close it: the result type takes its own name and the marker
trait keeps `Precision`, or the marker trait is renamed and `Precision` becomes the type constructor. Both
are one line, both are naming calls, and **neither is chosen here**. It is on `110`'s section 2 list.

**What is not in question is that the bridge works.** The mechanism file 118 compiled is untouched by this,
and the instantiation probe above re-verifies the same readout at a concrete numeral. Only the name it is
written under is unavailable, which is a smaller defect than it first reads as and a worse one than it
sounds, because it sits at the exact step a first implementer reaches.

## The citation convention, evaluated before being endorsed

The brief carried a three-part proposal and asked whether it covers the defect. Checked against the three
frozen files, **one part is the right answer and is not a proposal at all, and the other two should not be
done.**

### Qualifying by file cannot work

The decisive fact is one line of `ls`-level checking. **Both `D1` through `D4` runs live in the same file**,
`202607301000_topic.inherited-state-from-the-formalization-round.md`, at `:495` and `:763`. A file-qualified
citation is therefore exactly as ambiguous as a bare one on the only collision that exists, and it is worse
than a bare one, because it reads as though it had been disambiguated. **The brief's stated goal is that a
stale reference fail loudly rather than resolve to the wrong decision, and file-qualification fails silently
on precisely the case it would be adopted for.**

### Qualifying by round works, and the reason is structural

The inherited file is a concatenation of prior-round summaries under `# <round-id> <title>` headings. Every
decision in it therefore already sits under exactly one round: run one under `# 202607281220` (`:293`), run
two under `# 202607282100` (`:719`), and `D14` resuming under `# 202607290050` (`:814`). **The qualifier is
derivable by reading upward rather than remembered**, which is what makes a wrong pairing fail loudly:
`D4 (from 202607282100)` is checkable and false, where a bare `D4` is uncheckable and silently one of two.
This is file 113's proposal at `113:470-476` and it is correct.

### And it is already op's own practice, which changes what is being asked of him

Three sites in op's own frozen text use round-qualification, in the same form:

- `inherited:821`: "D1 and D2 (from `202607281220`) stand"
- `inherited:1792`: "the same pattern as `Rect` aliasing rank 2 under D40 (from `202607291910`)"
- `talk:655`: "D1 and D2, `202607282100`"

**So the convention is recovered rather than designed.** That is a materially lower bar than adopting a
scheme a panel invented, and it means the form to standardise on is op's parenthetical
`D<n> (from <round-id>)` rather than file 113's compact `<round-id>/D<n>`, which is worth keeping only where
a table cell is tight.

### The two register-side halves are declined, for one reason

**Prefixing or renumbering the inherited sequence edits a file frozen at TOPIC phase** (`110:234`), and it
would invalidate every existing citation to those numbers across `110` and the whole panel. That is a larger
blast radius than the defect. File 113 got this right and said so in its own framing, "the cheapest repair
that changes no committed text"; the restatement in the brief lost it.

**Assigning numbers to op's unnumbered decisions mints identifiers with no definition site in the register.**
A reader following `D76` into the three topic files finds nothing, which trades a citation that resolves to
the wrong decision for one that resolves to nothing, and it puts agent-assigned numbers in the same namespace
as op's own, which is the thing this whole item exists to prevent. The additive form gets the same benefit:
an unnumbered decision is cited by `file:line`, which is what file 113 already does for both it found.

**Both it found, not three.** The brief says three decisions carry no number. This pass could establish
**two**: the faithfulness derivation (`talk:1187-1203`, `spec:203-222`, in the spec file's ratified body and
not among the four items `spec:356-359` marks as the agent's own) and the four preset intents
(`talk:1659-1661`, op's own voice inside a paragraph with no D-number). The other unnumbered items in file
113, at `113:212` and `113:361`, are **numbered decisions that `110` carried without their number**, which is
a different defect with a different repair. A grep for `Decision (op` outside a `**D<n>.` line across the
talk and spec files returns seven hits and all seven are continuation lines of numbered decisions. If a third
exists it is in the inherited file, which `113:6.3` records has never been diffed against anything.

### A third namespace the proposal does not cover

**The question grid is not the decision register and qualification only half-covers it.** The talk file's
grid is headed `ID` and its rows run `A1` through `E5` (`talk:55-79`), so its `D1` through `D3` are the
fourth letter-group of a question grid rather than decisions. Round-qualification happens to separate them,
but only because the talk file's own decisions start at `D53`, which is a fact the reader has to know rather
than one the citation carries. A grid citation that names itself as one closes it properly. **Naming that
marker is op's**, and it is on `110`'s section 2 list with the rest.

### Applied, to `110` only

Five citations in `110` use the colliding range: `110:3455` twice (the `arvo-shape` and `arvo-geom` rows of
the crate table), `110:3732`, and `110:3884` twice. **All five resolve to run one**, `202607281220`, and all
five are now qualified in place with a mark at each site. The register is untouched. Note that `110:3448`
already told a reader, in prose above the table, which sequence the column is keyed on; the repair moves that
from a sentence the reader must have read first into the citation itself.

## What goes to op, and what does not

Two items are added to `110`'s section 2 list, both naming calls and both one line:

The **`Precision` collision**, which should be answered alongside the `Exponent` and `Dec` calls already
there, since all three are the same defect and answering them separately costs three sittings.

The **citation form**, where what is owed is confirmation rather than a decision, plus the genuinely open
half, which is the grid's own marker.

**Nothing else here is op's.** The Failure A judgement resolved on the document's own evidence and does not
need him. The inhabitedness question is a refinement under a closed perimeter and is recorded rather than
escalated. The sweep's counts are facts.

## The honest limit

**The blocks compile and one instance runs through them. That is not a statement that the shape is correct**,
and the sweep is silent on every verdict `110` reaches. A compiler checks that what is written is
well formed; it has no opinion on whether it is the right thing to write, and this document's harder open
questions, the array grammar's third column and the fused-versus-split call, are exactly the kind it cannot
see.

**It also cannot see a block that should exist and does not**, which is the failure the two cold reads found
and the one that mattered most. A mechanical sweep would have caught neither of the gaps file 118 closed. It
catches a different class, it catches it cheaply, and the correct reading of it finding exactly one defect in
eight blocks is that file 118's declaration pass was good rather than that the document is now verified.

**And the citation finding rests on greps over three files**, which this stretch has now demonstrated seven
times is the weakest part of any claim here. The file-qualification refutation does not: two line numbers in
one file settle it, and either holds or does not.
