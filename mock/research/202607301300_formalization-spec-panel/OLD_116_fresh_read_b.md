# Fresh read B: 110_consolidation_eleven.md, read in isolation

This is a single-pass read of `110_consolidation_eleven.md` alone, roughly 5,867 lines, no other file in
the panel directory opened, no source tree opened, no probe directory opened, per the instruction that
governed this read. Several places in the document cite compiled probes, benches, or tree state as the
actual evidence for a claim, and at each of those places I wanted to open the cited file to check the
claim rather than the citation. I did not. Those wants are marked inline below where they bear on a
finding, because the wanting is itself information about how load-bearing the citation is.

One property of the document has to be named before anything else, because it changes what "can this be
implemented from" means. The great majority of the text is not a specification of `arvo`. It is a record
of a design review's own history: what a prior consolidation said, what it dropped, what a correction
restores, which op checkpoint said what, and which search returned how many hits in which prior file.
Section 0 through section 9 spend more words establishing the provenance of a sentence than stating the
sentence. A reader looking for "here is the trait, here is the law, here is the type" has to extract it
from a document whose primary subject is its own editorial history. That is not a stylistic complaint. It
directly produces several of the findings below: definitions get referred to by citation to a source
outside this file, restated in prose, and never re-issued as a compileable unit inside the document that
claims to be self-contained.

## 1. Inventory, and the third category

I built this by writing down every named type, trait, axis, marker and constant as I met it, then marking
each: **defined** (a compileable Rust item, or an unambiguous closed enumeration, appears in the text),
**named-open** (the document itself lists it as open, undefined on purpose), or **assumed** (used as
though its shape is already settled, with no declaration anywhere in the file and no listing under
"named open" either).

### Defined (a real Rust item, or a closed BNF-style enumeration, appears)

- `Numeral` (trait, lines 787-798), `Implicit<E, A, B>` and `Ranged<EMIN, EMAX, U, S>` (struct
  signatures, no bodies, lines 795-796, repeated at 898-903).
- `Policy` (trait, lines 905-907, 2905-2907), `Lowering` (trait, lines 909-915, 2909-2915).
- `NumeralFace` (trait, lines 919-922, 2919-2922).
- `Crosses<N: Numeral>: Lowering` (unsafe trait, lines 923-938, 2923-2938). The body is comments only;
  see section 2 below on what this actually enforces.
- `Resolution` (empty trait, line 2965), `Direction: [const] Resolution` (empty trait, line 2969).
- `TowardNegative`, `TowardPositive`, `TowardZero`, `AwayFromZero`, `ToEven`, `ToOdd` (unit structs,
  lines 2971-2973), `ReduceModulo`, `SubstituteZero`, `Refuse` (unit structs, lines 2976-2977).
- `Quantisation` (trait, lines 2980-2987).
- `Encoding` (trait, lines 869-874).
- `AsBool` (`pub const trait AsBool { fn as_bool(&self) -> bool; }`, line 3993, but this is quoted as a
  tree-fact from `arvo-storage/src/platform.rs:323-333`, not authored as design here).
- `max<T>` (a working function, lines 4204-4209), the one operational code sample in the whole document
  outside trait declarations.
- The tower's BNF shorthand (lines 2888-2896): `Nat ::= Z | Pz<P>`, `Pos ::= H | O<P> | I<P>`,
  `Bias ::= BZero | BPos<N,D> | BNeg<N,D>`, `Exponent ::= EZero | EPos<P> | ENeg<P>`, `Radix ::= Rad<P>`,
  `Capacity: Nat`. This is a comment block, not compiled Rust, but the enumeration is closed and
  unambiguous, so I count it defined rather than assumed.

### Named-open (the document itself says so)

`FullRange`'s survival as a named `Adjustment` constructor (line 782), which level a bitfield's declared
width is (line 3929), the platform crate's own name (line 4035, deferred to the taxonomy round), the
required-field relation for the environment receipt (line 3425), whether a naming guarantee counts as a
guarantee for the route-multiplicity discriminator (line 3767), the truth contract's own name (line 5766
list). Section 9's own "named open rather than defined" paragraph (lines 5766-5769) is the document's
attempt at a complete list of these; see the finding below about what it misses.

### Assumed: named and used, defined nowhere, not on the open list

This is the category the task asked me to watch for, because it does not announce itself.

- **`Number<N, S>`**, called "the design's central object" is never given a struct or type declaration
  anywhere in 5,867 lines. It is used repeatedly as a two-parameter form (line 877: "the two-parameter
  fused form", explicitly contrasted with a rejected three-parameter split) and, separately, as a
  three-argument concrete instantiation, `Number<Fix13_3Signed, Warm, MinWidth>` (line 3158) and
  `Number<Fix13_3Signed, Warm, DoubleWidth>` (line 3159). Nowhere does the document state whether `S`
  is a strategy preset that itself bundles a `Policy` and a `Lowering`, or whether `Number` is generic
  over more than two parameters and the "two-parameter fused form" language is stale. See section 4.
- **`Compare`** and **`Select`**, the two traits the worked `max` example (lines 4204-4209) depends on.
  The signature reads `T: Compare + Select<Truth = <T as Compare>::Truth>`, which presupposes `Compare`
  declares an associated type `Truth` and a method `.lt()`, and that `Select` declares an associated
  type `Truth` and an associated function `select(truth, on_true, on_false)`. None of this is declared.
  Neither name appears under section 9's "named open" list. I wanted to check whether `Compare` is
  declared in an earlier consolidation this document claims to supersede, and did not, per the
  constraint; inside this file it is simply used.
- **`Bias`**, **`Adjustment`**, **`Nat`**, **`Pos`** as concrete types with real fields: the BNF shorthand
  names the constructors but never their payload types, their arithmetic signatures (`VAL`, `Cmp`, `Gcd`,
  `Dec`/`PosPred` are named and discussed at length, e.g. lines 3472-3475, but never given a signature).
- **`Precision`**, **`SignDomain`**, **`SignIndexing`**, **`FieldLayout`**, **`Canonicalisation`**,
  **`StoredWidth`** (the trait, distinct from the associated type of the same name), **`StorageLayout`**,
  **`LoweringDoor`**: every one of these appears as a bound in the `Numeral`, `Encoding`, or `Lowering`
  trait declarations (lines 787-798, 869-874, 909-915) and is discussed extensively in prose, but none is
  itself declared as a trait or enum with variants.
- **`TotalOrd`**: the trait split described at length (lines 2441-2448, "rename the shipped mechanism...
  give `TotalOrd` the canonicalise-then-compare body") is a design decision stated entirely in prose. No
  trait signature for either half is given.
- **`UFixed`, `IFixed`, `FastFloat`, `StrictFloat`**: said to become "four names for four compositions"
  over `Number` (lines 3006-3009), with the composition itself never spelled out, which compounds the
  `Number` arity gap above.
- **`Branch`**: appears only inside a quoted compiler diagnostic, `Mask2: Branch is not satisfied`
  (line 4186), as the bound name for the exit trait the document is in the middle of designing. Never
  declared.
- **`LogicalNumber<N, P, L>`**: proposed once (line 3165) as the shape that would fully close the
  `Number`/`Lowering` enforcement gap, described as "proven and verified" by a cited probe, never shown.
- **`Mask<W>`**, **`TruthAlgebra<N>`**: `Mask<W>` is discussed at length (section 1.30) and asserted
  isomorphic to `Bool^W`; `TruthAlgebra<N>` is offered as a priced alternative (lines 4303-4312). Neither
  is declared.

The pattern across this category is consistent: a name gets a page of behavioral prose, sometimes a
compiled fact cited by file path, and stands in for a declaration a reader is expected to trust exists
somewhere outside this document. For a document whose own stated goal (lines 29-35) is that "a reader can
reconstruct the design from this file alone," this is the load-bearing gap.

## 2. Three guarantees, checked

**Guarantee one: the crossing contract, `Crosses<N: Numeral>: Lowering` (lines 923-938).** The trait
carries three named obligations, "statement 0" (decode is total on the fields' width), "statement P"
(padding bits equal the declared padding), "statement C" (container bits outside the carrier are
canonical). All three are stated as comments inside an otherwise-empty trait body. The trait compiles as
written with zero associated items, so nothing about it is type-checked beyond the ordinary `unsafe impl`
requirement. To believe a given `impl Crosses<N> for MyLowering` actually satisfies statement 0, 1 would
have to trust that whoever wrote the `unsafe impl` manually verified totality over the field width by
hand, because the trait offers no associated const or method the compiler could refuse on. The document
itself is honest about this ("every `unsafe impl Crosses` is an entry in the trusted base, named
explicitly as such," line 1004), which is good practice, but the surrounding apparatus (three numbered
"statements," a derivation blockquote at lines 1007-1008, a whole subsection of case analysis) reads as
considerably more rigorous than "an `unsafe` trait with a doc comment," which is what it compiles to. A
reader skimming the trait table (section 1.23) rather than the prose would have no way to see that the
three statements carry zero compiler assistance.

**Guarantee two: the sealed vocabulary is closed against foreign implementation (section 1.12, lines
1622-1738).** The claim, stated repeatedly, is that six named carriers (`Rad<P>`, the strategy door's
`HostImplemented` marker, the notation macro's `Bias`/`Adjustment` constructors, `Arity`'s `Fin<P>`/
`Unbounded`, `WidthFor<Family>`, `NumeralFace`'s coarsening bound) are closed. But the document names its
own limit on this in one sentence, restored at lines 1640-1644: "the enumeration is verified as 'every
attack found lands in one of the routes,' not as 'the routes are the whole space.'" That is an inductive
claim over a fixed set of attempted attacks (direct impl, supertrait, layout-identity wrapper, non-member
instantiation), not a proof that those four routes exhaust what Rust's trait system permits. To believe
the closure guarantee, one has to trust that the private-supertrait pattern is correctly implemented in
code this document never shows (the actual sealing mechanism, `mod private { pub trait Sealed {} }` or
equivalent, is never given), and that the four enumerated attack routes are exhaustive. Section 1.12
itself states this is not proven, in one clause, forty lines into a section whose headline reads
"Eleven firings stand... capacity closed, the niche vocabulary narrowed" (line 1622), which reads as a
much stronger claim than the section's own qualifier supports. I wanted to open `92_probes/` to see the
actual attack code and judge whether the four routes really are the natural ones to try; I did not.

**Guarantee three: the document's own definitional-completeness line, self-certified in section 9.**
This one is checkable entirely from inside the file, which makes it the strongest finding here. Section
9 runs the document's own "definitional-completeness line" (every term either defined or named open) and
concludes: "No term in this document is left undefined or uncited" (line 5771). This is false by the
document's own evidence gathered in section 1: `Compare` and `Select`, used in the `max` function at line
4206, are neither defined nor listed among the "named open rather than defined" items at lines 5766-5769.
`Number`'s own arity is neither resolved nor named open anywhere; it is simply used two different ways
(lines 877 and 3158-3159). The mechanism that this guarantee rests on (an author running a checklist
against their own prose, "performed by the author... before it stands," line 518) is exactly the
mechanism the document spends section 8 documenting as having failed across ten prior consolidations for
unrelated reasons (stub sections, dropped restorations). Here it fails on the same document that names
it, in the same document's final verification section. This is not a citation-chain problem the way the
other findings in section 4 below are; it is the guarantee failing on direct inspection of the text that
states it.

## 3. Where a competent implementer goes wrong

Not where the document is silent. Where it is precise enough to act on, and the action is wrong or
underdetermined in a way that would not surface until deep into the build.

**The strategy preset tables look complete and are not, on the one axis that matters most for `Cold`.**
The fixed-point table (lines 2504-2510) gives `Warm` and `Cold` identical rows for in-range direction,
`OverRange`/`UnderRange`, and even the numeric-behavior cells; they diverge only on `StoredWidth` and
`Layout`. An implementer building from the table alone would give `Warm` and `Cold` byte-identical
arithmetic lowering. The distinguishing property op actually states, that `Cold` "can take more cost than
warm" because it is on a cold path (quoted at lines 2480-2482, restored as a correction because an
earlier version of this same document elided the sentence, lines 2491-2496), is carried nowhere in the
table itself, only in the surrounding prose, and the document's own text admits the row content does not
encode it: "`Cold` now pays a compare and select on every store" (line 2541) is a fact about the
implementation of the row, not a value that appears in the row. A reader who trusts the table (the object
that is actually shaped like an implementation spec) over the prose builds the wrong thing.

**`Number<N, S>`'s undetermined arity (section 1) is the single largest trap**, because every operation
signature in the document (`mul_full`, `div_floor`, `foldnum`, the whole operation-surface admission test
at lines 2129-2132) is stated in prose against "the operand numeral" or "the result numeral" without ever
giving the concrete generic parameter list those functions would need. An implementer confident enough to
start from the two-parameter form used at line 877 will hit the three-argument usage at lines 3158-3159
partway through and have no way to know which is current without asking.

**The D68/nested-`Numeral` contradiction is presented as settled by a code block.** The `Numeral` trait
declaration at lines 787-798 shows `Adjustment` and `Bias` nested inside `ExponentForm`. That is a
compileable Rust item, which reads as authoritative. But the surrounding correction (lines 843-851)
states plainly that op's ratified D68 calls for four *flat* members (`ExponentForm`, `Adjustment`, `Bias`,
`Sign`) and that "the nesting's argument is section 1.2's own... amending D65... neither of the two
options op chose between," closing with "whether op accepts the supersession is op's, and it is on the
open list." A reader who takes the code block at face value (as most readers of a spec would, since it
compiles and reads as the settled shape) has no way to know from the code alone that it may be overturned
by a standing, unresolved ratification conflict.

**The D72 crate table assigns a ratified-removed axis to a crate.** The table at lines 3110-3117 lists
`arvo-policy` as holding "`Policy`, `Quantisation`, `Resolution`, `Direction`, `Growth`, and their
markers." But `Growth` is stated, twice, as having "left `Policy` entirely" at `39b` (line 1539, restated
at line 1590: "`Growth` leaves the law key, ratified; and it leaves `Policy` entirely"). The document
flags this itself three lines below the table (lines 3120-3123: "Two rows name axes this design has since
ratified out... those two cells are stale on the register's own side and the crates are not"), but an
implementer building crates directly from the printed table, which is exactly what a "crate table" invites
a reader to do, will create a home for a type that the rest of the same document says should not exist.

**The worked `max` function invites copying code that does not compile against anything declared in this
file.** It is presented as the resolution to a real design fork (section 1.30), with a compiler error
message quoted to support it (line 4186), which makes it read as verified. It is verified against traits
(`Compare`, `Select`) that this document never declares (section 1, "assumed" category).

## 4. Internal consistency

**The clearest failure is the document's own count of its own unresolved list, in its own closing
section**, and it is worth stating plainly because the document's central thesis, restated at least three
times (lines 1647, 422-424, and the correction narrative running through all of section 8), is that "a
count cannot be checked and a list can." Lines 5611-5645 give nine bulleted entries under "Not restored,
and the reader should know which." But the bullets themselves are not one item each: the sixth bullet
(lines 5633-5634) names two distinct things joined by "and" ("the classification-versus-exhaustive-check
overlap... and the accumulator's three readings"), and the ninth bullet (lines 5640-5645) explicitly
enumerates four named items in parentheses ("the IEEE §4.3.1 overflow tie, the OCP mode split, `Crosses`'s
second read, and statement 0 against `quantize` and `roundToIntegralExact`"). Counting the document's own
named items rather than its bullet points gives thirteen, not nine. The closing sentence, "That is nine
items this document names and does not answer" (lines 5647-5649), repeats the undercount. This is the
exact failure mode the document spent its own first section correcting in a predecessor (the spine rule's
"eleven firings," degraded to a bare count and restored to a list at lines 415-424) and it recurs here,
uncaught, in the document's own final accounting of itself.

**`Number<N, S>` versus `Number<Fix13_3Signed, Warm, MinWidth>`.** Quoted above in sections 1 and 3;
repeating the citations here because this is squarely a "statements that cannot both hold" case. Line 877
states plainly that the *reason* `Encoding` nests inside `Lowering` rather than becoming a third type
parameter is "so the two-parameter fused form survives." Lines 3158-3159 use a concrete three-argument
instantiation as a compiled example inside the same document, with no note that the third argument is
anything other than an ordinary generic parameter of `Number` itself.

**The `Adjustment` constant is stated twice, in two different exponent conventions, and left standing
both ways rather than reconciled.** Line 772 gives the UNORM8 worked example under `e = -F` as
`256/255`. Line 1616 gives the same underlying fact under `e = 0` as `Adjustment = 1/(r^F - 1)`. The
document's own correction (lines 776-784) catches that this violates its own "widened definitional-
completeness line" (a name defined twice with different content is defined nowhere) and repairs it by
naming the convention at each site, then states: "both are correct and they are correct under different
exponent conventions... the convention is named at both sites and the two spellings are left standing,
because both are in use and neither is wrong" (lines 780-783). That is a resolution by assertion of
equivalence, not a derivation of it; the document never actually shows the algebra connecting `256/255`
to `1/(r^F - 1)` under the stated change of convention. An implementer still has to do that derivation
themselves before trusting the two are the same fact.

**The two provable-versus-trusted arguments for `Bool`'s "one route" rule contradict each other in a way
the document resolves but only after stating both as though either would do.** Lines 4001-4020 draw the
distinction explicitly (route multiplicity is either a soundness defect, if a guarantee is at stake, or a
"redundancy" if not) and then walk back an earlier draft's citation of the workspace perimeter rule as the
ground, calling the citation "struck" because `Bool` has no invariant to protect. This is presented as a
correction inside the document rather than a standing contradiction, so I record it as evidence of the
same drift pattern (a claim asserted first, corrected mid-document) rather than a live inconsistency, but
it means a reader who stops reading section 1.30 before line 4001 carries the wrong ground for the rule
for the preceding four thousand words of the same section.

## 5. Verdict

This cannot be implemented from as it stands, and the reasons are structural, not a matter of missing
polish.

The object the whole design is organized around, `Number<N, S>` (or `Number<N, P, L>`, arity
unresolved), is never declared. Every operation, every alias, every consumer-facing example depends on
knowing its shape, and the document uses two incompatible arities for it without noting the conflict
(section 4). The foundational value-unique tower (`Nat`, `Pos`, `Bias`, `Adjustment`, `Radix`/`Rad<P>`)
exists in the document only as an informal BNF comment; none of the arithmetic the design leans on
(`VAL`, `Cmp`, `Gcd`, `Dec`) is given a real signature, so an implementer has to invent the encoding from
prose descriptions of its behavior rather than build against a declared contract. A working `max` example
is offered as the resolution of a real design question and depends on two traits (`Compare`, `Select`)
that are used but never declared anywhere in the file, and are not listed as open either, so a reader has
no signal that they are missing until the code fails to compile. At least one ratified decision (D68's
flat `Numeral` members) is directly contradicted by a compileable code block the document presents as the
current shape (section 1.2), with the contradiction acknowledged only in surrounding prose and explicitly
left to op. The document's own self-audit (section 9) claims completeness of its own vocabulary and is
false on inspection of its own text (section 2, guarantee three), and its own final list of unresolved
items miscounts itself by the exact failure mode (count without an enumeration matching it) that the
document spent its first section correcting in a predecessor (section 4).

None of this means the underlying design work is bad. Two things worked cleanly enough that an
implementer really could build from them directly: the `Quantisation`/`Direction`/`Resolution` block
(lines 2963-2987) is a complete, self-contained, compileable set of items, six unit structs plus two
traits plus one aggregating trait, with no forward references outside itself except `notko::ConstTry`,
which is a reasonable external dependency rather than a gap; and the two strategy-preset tables (lines
2504-2510, 2557-2563), read together with the crossing-contract's three statements (lines 918-922) and
the E4M3 leak-percentage table (lines 961-970), give enough concrete, checkable content that a reader
could write the quantiser's classification logic and get it right, aside from the `Cold`-versus-`Warm`
gap noted in section 3.

The smallest set of additions that would make this implementable, ordered by how much each unblocks:

1. **State `Number`'s real generic parameter list once, as a compileable declaration**, and reconcile the
   two-parameter and three-parameter usages against it. Nothing else in the document can be checked for
   consistency until this exists, because every operation signature is written against "the operand
   numeral" rather than a concrete type.
2. **Give the tower types (`Nat`, `Pos`, `Bias`, `Adjustment`, `Rad<P>`, `Exponent`) as real Rust**, with
   the arithmetic operations the document already names (`VAL`, `Cmp`, `Gcd`, `Dec`) as actual trait
   methods or associated consts, not comment shorthand.
3. **Resolve the D68 flat-versus-nested `Numeral` conflict.** This is explicitly op's call and explicitly
   still open, but it blocks the identity contract's actual field layout, which blocks everything built
   on `Numeral`.
4. **Fix the crate table** so it does not assign the ratified-removed `Growth` axis a home, and settle
   the array-grammar/capacity storage fork (section 1.27), which the document itself carries as three
   live, unresolved columns and which blocks `arvo-shape`, `arvo-capacity`, and `arvo-tensor` as a group.
5. **Declare `Compare` and `Select`** (or drop the worked example that depends on them) before the truth-
   contract fork's lock (section 1.30) is treated as implementation-ready.

Item 1 alone unblocks the largest share of the rest; items 2 through 5 are each independently blocking a
distinct region of the design (the tower, the identity contract, the crate boundaries, the truth
contract) and none of the four depends on the others resolving first.
