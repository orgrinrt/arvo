# 100. Shape and geometry: shape is the index domain and settles now, geometry composes with no new growth class, and two of the round's own counts are wrong

**Member:** Inigo Quilez, file 100, first dispatch. I am sent at ground the review recorded as its own
blind spot at file 11 and never returned to across ninety-nine files: `arvo-shape` (rank and per-axis
extents) and `arvo-geom` (points, orthotopes, affine maps, rotation, curves).

The dispatch frames these as decisions "made before this design had any of what it now has", and
invites me to treat my own reading as worth more than the sketch's. **The framing is half right and
the half that is wrong changes what this file may do**, so I state the correction before anything
else. There is no sketch. There is a design round, `mock/design_rounds/202607300800/`, thirty-two
topic files and four changelists, in which the lead designer decided D1 through D13 on 2026-07-28 and
D40 through D49 on 2026-07-29, each marked **"Decision (op)"** inline, with rejected alternatives
recorded. Its doc and src changelists locked at 08:00 and 09:00 on 2026-07-30 and this panel opened at
13:00 the same day. So the shape and geometry calls are one to two days older than the panel, not
five weeks, and they sit on the governing rung of the provenance ladder, not the presumed-wrong one.

What *is* true, and is the whole of my licence here, is that everything the panel has built since is
younger than them: the numeral tower, the quantiser, the far point, the three width levels, the
crossing contract's three statements, the capacity resolution, the layer-keying rule, and an
operation surface that as of file 99 reaches the transcendentals. Op's own standing sentence covers
exactly this case: only op's calls are final, **and even those go stale, when new material surfaces
or when someone works out something better**. So I do not reopen D4 or D10 as choices. I check them
against material that did not exist when they were made, and where a stated ground turns out to be
false I say so with the arithmetic that shows it.

Two of them are false. Both are counts, both are small, and both change what gets built.

## Gates, run before the work

**Canon gate.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same
with `FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD `75710b6`. The governing canon I
checked this dispatch against is `91_consolidation_nine.md` as the standing base, plus the op-ratified
design round `202607300800` for the two subjects assigned. **Gate passed**, with the framing
correction above recorded rather than treated as a refusal: the dispatch's question is the right one,
its premise about the artifact's provenance is not.

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary: **155 binaries,
672 passed, 0 failed, 9 ignored**, matching files 98 and 99 exactly, with the same attribution to the
concurrent dispatch's uncommitted `mock/Cargo.toml` and `mock/benches/*` work, which I did not touch.
I read the body of every test in the two surfaces nearest my subject, `arvo-tensor/tests/` (six files)
and `arvo-bitmask/tests/` (nine files), rather than their names.

The bitmask tests are real: `bitmatrix.rs` asserts adjacency, closure, fan-out and disconnection with
values that could be wrong, `capacity_threading.rs` threads a caller's own generic capacity end to
end. `array.rs`, `matrix.rs`, `container_capacity.rs`, `const_capacity.rs` and `enumerator.rs` are
likewise real, and `capacity.rs:33-64`'s `reverse_fill_then_assert` even checks that slack slots past
the live region stay undisturbed, which is a genuine perimeter assertion.

**Two tautologies, and one of them is new.** The standing disqualified test,
`arvo-tensor/tests/capacity.rs:14-18`, is now **twenty-three files** past its flagging; I re-verified
it at source rather than taking it from file 98. `impl<const N: usize> Capacity for Dim<N>` declares
`const CAP: Cap = cap(N)` at `src/capacity.rs:48`, so `<Dim<3> as Capacity>::CAP` **is** `cap(3)` by
substitution and the assertion is `assert_eq!(cap(3), cap(3))`. Its disposition is ruled at
`95b:145-149` as op's own trivial commit outside the panel, so I add only the count.

What no file has flagged: **`arvo-tensor/tests/const_capacity.rs:49-53` is the identical tautology
against the sibling impl.** `impl<const N: usize> const ConstCapacity for Dim<N>` declares
`const CAP: Cap = cap(N)` at `src/capacity.rs:117`, and the test asserts
`<Dim<3> as ConstCapacity>::CAP == cap(3)` and `<Dim<13> as ConstCapacity>::CAP == cap(13)`. Two more
assertions structurally incapable of failing, in the green total, in a file whose other four tests are
substantial. It is the same defect at the second impl, and it went unfound for the same reason the
first one nearly did: nobody read the impl beside the test. It should be deleted with its sibling in
the same commit, not improved.

Nothing else in either surface disqualifies. I did not refuse the work.

## What I read

`91_consolidation_nine.md` in full, the standing base. `98_knuth_how_complete_is_the_canon.md` and
`99_smith_the_elementary_functions.md` in full, the two files that name my ground and the most recent
operation chapter. `11_current_shape_draft.md:29-60`, the blind-spot table and its introducing
paragraph, and nothing else from that superseded file. One `ls` of the panel directory, read rather
than skimmed, current through `99_probes`.

Then, because the `ls` discipline applies to the directories around my subject and not only to the
panel's own: `mock/design_rounds/`, which turned up `202607300800/` and its thirty-two topic files,
and `mock/research/202607281616_prior_art/`, nine external passes commissioned by that round. From
the round I read in full `202607281220_topic.the-ndim-and-shape-design.md` (D1 through D13 with the
research addendum), `202607292100_topic.rank-spans-both-domains.md` (D43 through D45),
`202607291910_topic.the-box-and-the-rotation-bench.md` (D40, D41, and its same-day retraction), and
`202607300700_topic.consolidated-round-state.md` at the shape and geometry sections plus the PGA
required-reading entry and the strategy-axes research. From the review's own record I read
`74_lattner_the_taxonomy_rechecked.md:64-65` and `40_consolidation_three.md:440-500`, the latter
because the `Adjustment` semantics turned out to decide a question I had first answered wrongly
without them.

Shipped source at four factual-check points, recorded where used, none read for meaning:
`arvo-tensor/src/capacity.rs:46-52` and `:115-121` (the two `CAP` declarations, for the test gate),
and `arvo-bitmask/src/matrix.rs:29-68` (the axis asymmetry, why-evidence).

## What I compiled or measured, separated from what I reasoned

Eight probes in `100_probes/`, outcomes in `100_probes/OUTCOMES.md`. Probes 1, 2, 5, 6 and 7 are Rust
at the pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml` inside the tree; the identical command outside resolves to stable `1.94.0` and
does not build the const-position work. **Every Rust probe compiles with zero `#![feature(...)]`
gates**, which is itself a result and not a convenience. Probe 8's compile-time figures are
`--emit=metadata`, best of three, so they measure trait-solver work and not codegen; its instruction
counts are `-O --emit=asm` on that target. Probes 3 and 4 are exact integer and exact rational
Python, the latter an exact Clifford algebra over `Fraction`; no float enters any load-bearing
comparison anywhere in this file.

Reasoned and marked as such in place: the enumeration in section 1, the shape-versus-layout split in
section 2.1, the requirements list in section 4.3, the locus suggestion in section 4.5, and every
spec-shaped sentence. The Abel-Ruffini and Lindemann-Weierstrass appeals are external standard
mathematics, cited rather than compiled, per the review's practice at `91:209-212`.

I did not run the bench harness and touched nothing under `mock/benches/` or the workspace manifest,
per the dispatch's boundary. No number below is a timing claim about anything but compilation.

---

## 1. What these are, enumerated before anything is decided

The dispatch asks me to enumerate first. Here is my reading, formed before I read D2's contents list,
and then reconciled with it.

**Shape is not a geometry subject at all. It is the index-domain layer, and the design already has its
rank-1 case.** `91` section 1.26 settles that a capacity "is a type-level parameter that establishes
an index domain, of the same kind as `Precision`, `Exponent`, and `StoredWidth`". A shape is a
multi-dimensional index domain, which is the same kind of thing with more than one axis. Everything
that follows from that is numerics: rank, per-axis extents, the total element count, index and stride
arithmetic, the projection from a logical index domain to a physical container, the relations that say
whether two shapes may be zipped or contracted, and the rank-generic folds D13 calls cascades. Not one
of those mentions space.

That reading has a consequence the round did not draw and that I think is the most useful thing in
this file: **`arvo-shape` is downstream of the capacity resolution and upstream of everything with a
container, and it is the third instance of a pattern the design has already worked out twice.** The
design has a logical extent, a declared carrier and an allocated container at one dimension
(`91:531-543`, `W_F <= W_S <= W_C`). A shape is the same three levels with a rank. Section 2 is that
claim, compiled.

**Geometry is two things wearing one crate name, and the seam is sharp.** One half is points,
orthotopes, affine maps, their predicates (containment, intersection), and their measure. Every one of
those is a shape instantiated over a numeral, with operations that are multilinear: sums of products.
Section 3 shows that half composes over the settled operation surface with no new growth class, no new
failure kind and no new vocabulary, which means it is finished as soon as shape is.

The other half is rotation and curves. Both require a **choice of model** before any arithmetic
happens: which algebra and which convention for rotation, which representation for curves. Op has
already ruled that neither is picked by argument (D41: bench broadly after naming the requirements;
D11: a bench matrix, not a pick). I do not reopen either. What I can do, and what section 4 does, is
the thing D41 explicitly asks for **first** and which nobody has produced: name the requirements the
bench is run against, derived from what the design has settled since D41 was made.

**What I did not find a place for, and where it goes.** Colour, which `202607281127` flagged as a
genuine open question and D12 sent to an imaging domain; the sibling repo `kirjo` now owns it, and
section 3.4 hands it back one fact it needs. Layout, which `202607281005` sent to `lato` on the ground
that layout is policy over shapes rather than arithmetic on them. Both exits are right and I reinforce
the second in section 4.5 by applying the same test one crate further in.

Reconciled with D2's own contents list: `arvo-shape` holds "rank, per-axis extent sequences, generic
over rank", and `arvo-geom` holds "`Point`, extent, box, `Affine`, and the semantic aliases".
**D2's geom list is exactly the multilinear half.** Rotors arrive from D10 and curves from D11, and
neither of those decisions names a crate. So the seam I am pointing at is already latent in op's own
decisions rather than proposed against them.

---

## 2. Shape

### 2.1 D4 survives the capacity resolution verbatim, and the split it needs is one D43 already made

D4 (op, `202607281220`) declares rank and per-axis extents as a type-level list of capacities, "and
the backing storage is the recursive composition of each capacity's array":

```
Nil::Array<E>        = E
Cons<H, T>::Array<E> = H::Array<T::Array<E>>
```

That was written when a capacity was `Dim<const N: usize>` with `type Array<T> = [T; N]`, one const
generic doing both jobs. Since then `91` section 1.26 split the job: the capacity's value is a direct
`Nat`, and the array grammar is "a paired, non-derived fact, forced by the language" carried by
`Slot<N, const K: usize>`. So the first question is whether the recursion survives when the length is
a paired literal rather than a derived one.

**It does, unchanged.** Compiled (`100_probes/probe_1`, claim B): `Axis<E3, Axis<E4, Axis<E5,
Scalar>>>` has `Store<u32> = [[[u32; 5]; 4]; 3]` and `size_of` is 240, which equals `COUNT * 4` where
`COUNT` is a fold over the `Nat` values. The two routes are independent (one is the language's layout
of a nested array, the other is arithmetic over the sealed tower) so their agreeing is content rather
than restatement. Rank, extents and count are all const-position folds, zero feature gates.

**The rank-generic function exists.** `202607281127` calls this "the concrete missing piece ... with
no worked example anywhere in arvo" and D4 calls it "the one thing actually missing". One body, every
rank, in two flavours: one that walks the storage and one that reads only the shape (`probe_1`, claim
C). This is the thing D4 was chosen for and it had never been run.

**What D4's own sentence blurs, and D43 already fixed without saying so.** D4 makes the array
composition constitutive of the shape. D43 (op, `202607292100`) says the opposite in the same round:
"the shape abstraction supplies rank and extent and nothing else, so a bit container implements both
and neither one grows the other's surface." Those cannot both be the shape's own trait. If the
recursive `Array` composition lives in `Shape`, then the bit domain implementing `Shape` inherits
`[[bit; 4]; 3]`, which is precisely the storage D43's own correction rejects.

**The resolution is to write D43's sentence literally: `Shape` carries rank and extents; a separate
trait projects a shape to storage, once per element domain.** Compiled in `probe_1` as `Shape` (no
element type anywhere in it) and `Dense: Shape` (the projection). That is not a new mechanism. It is
the design's own `Lowering` charter one dimension up: `Lowering` changes no value and `Encoding`,
nested inside it, may change which datum carries a value (`91:157-159`). A shape changes no index
domain; a layout, nested beside it, chooses which bytes carry it. File 74 saw this coming from the
other side and said so: `arvo-shape` "survives, for a stronger reason than it was decided on ...
D43's own load-bearing sentence is now the `Lowering` charter verbatim" (`74:64`).

*Grounded on: ratified (D4 and D43, `202607281220` and `202607292100`, both op; `91:157-159`,
`91:531-543`), settled shapes (`74:64`), compiled (`100_probes/probe_1`, all four claims, const
position, zero gates), reasoned (the trait split, mine, offered as the shape D43's sentence already
requires).*

### 2.2 Section 1.26's "one construction door" is not one door once capacities compose

This is the defect, and it is small, live, and fixed below.

`91:796-802` states the array grammar is "checked to agree in an inline const block at the one
construction door". The door it means is `Slot`'s **inherent** method, which is where
`79_probes/probe_1_capacity_is_a_nat.rs` puts the assert. D4's recursion does not call it. It calls
the **trait** method, `Capacity::filled`, because a composition is written against the trait. And the
trait method has no check, because at rank 1 the inherent door was the only door.

Compiled, and the divergence is a real number rather than a hypothetical (`probe_1`, claim D): a
rank-3 shape whose middle axis declares a `Nat` of 4 against a literal of 7 has `COUNT == 12` and
`size_of(Store) == 21`. **Both compile. Both are const-evaluable. They disagree, and nothing raises.**
Through the inherent door the identical type is refused with `E0080`; through the trait it returns
`[u8; 7]` (`probe_2`, claim A, both routes compiled).

**The repair is one clause and it is the shape the design already uses for the width levels.**
`91:560-561` establishes the precedent: the level ordering `W_F <= W_S <= W_C` "is a declaration-site
refusal, compiled ... `E0080`". Lift the agreement from an inherent method to `const AGREES: bool` on
`Capacity` itself, so it is a fact about the type rather than about one call:

```rust
pub trait Capacity: Nat {
    type Array<T: Copy>: AsRef<[T]> + AsMut<[T]> + Copy;
    // the paired fact, checked where it is declared rather than where it is used
    const AGREES: bool;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
}
```

Compiled (`probe_2`, claim B): the trait route now refuses, naming the offending capacity.

**And the honest scope, which is the part worth stating in the record rather than leaving for someone
to find.** As written the repair fires only where `AGREES` is *reached*, so a lying capacity whose
`COUNT` is merely read still compiles, and `COUNT` and `size_of` still disagree (checked: exit 0,
`probe_2` claim C). Adding `assert!(Hd::AGREES)` inside `Axis`'s own `COUNT` closes that too, and a
bare const read then refuses with no construction anywhere. Both halves are needed; either alone
leaves a door.

I claim no novelty for the mechanism. What is new is the finding that section 1.26's sentence has a
rank in it that nobody wrote down, and that it is false above rank 0.

*Grounded on: ratified (`91:796-802`, `91:560-561`), compiled (`100_probes/probe_1` claim D,
`probe_2` claims A through C, three refusals reproduced with their `E0080` text in `OUTCOMES.md`),
reasoned (the diagnosis that the trait method is the composition's door, mine).*

### 2.3 Which axis `Layout::Bitpacked` packs, and the answer needs nothing new

`91:563-570` ratifies the group arithmetic for one dimension: `P = 8/gcd(W_S, 8)` elements in
`G = W_S * P / 8` whole bytes, with `G * 8 = W_S * P` by algebra alone. `91:645-651` then prices the
mutation gap against it: under `Bitpacked` "the dirt surface this theorem protects is strictly
smaller (one tail-group region for the whole column, not one region per element)".

**Neither sentence has a rank, and at rank 2 there are two readings.** Pack each innermost row
independently, which gives one tail region per outer index. Or pack the whole shape as one run of
`COUNT` elements, which gives one tail region for the shape. Exact integer arithmetic over the whole
grid `W` in 1..64, inner in 1..64, outer in 1..32, 131072 cases (`probe_3`):

- **Flattened is never worse in footprint.** Per-axis is cheaper in 0 of 131072 cases, and its worst
  excess is 28 bytes against 4, an eight-fold footprint, at `(W, inner, outer) = (1, 1, 32)`.
- **Under the flattened reading the rank-N container is exactly the rank-1 container of `COUNT`
  elements.** `P` and `G` are functions of `W` alone, so nothing about them is rank-sensitive, and the
  shape contributes index arithmetic and nothing else. 0 violations.
- **The write granule becomes a shape fact.** `91:676-679` already states that adjacent bitpacked
  values share bytes so a parallel partition must place every boundary on a multiple of `P`. At rank
  N that reads: an **outer-axis** partition is legal only when `inner mod P == 0`. Six of my twelve
  model shapes fail it, including `3x4 of 7-bit` and `17x13 of 23-bit`.

**My first statement of when the two coincide was false, and the exhaustive sweep is what caught it.**
I wrote "byte-identical exactly when the innermost row is byte-aligned", which mismatches 5632 of
131072. Adding "or outer == 1" halves it to 2816, which is the shape of a patch rather than a
derivation. The exact condition, zero mismatches over the same grid, is

> **the two readings coincide exactly when `outer * rowpad < 8`**, that is, when the whole per-axis
> padding fits inside one byte, and they differ the moment it reaches a whole byte.

With a one-line proof once it is stated: per-axis padding is `outer * ((-x) mod 8)` and flattened
padding is `(-outer*x) mod 8`, which is the same quantity reduced mod 8, so they are equal exactly
when the first is already below 8. I record the false version rather than quietly replacing it,
because it is a clean instance of the separation requirement earning its keep: **at every shape a
reviewer would reach for, the two layouts are byte-identical**, and a model checked over byte-aligned
rows separates nothing.

**So the suggested sentence for the record, and it adds no mechanism:** under `Layout::Bitpacked` a
shape's container is the rank-1 bitpacked container of its `COUNT` elements, the ratified group
arithmetic applies unchanged because it is a function of the stored width alone, the single tail
group is the shape's whole padding surface, and the write granule's partition condition is `inner mod
P == 0` on any axis a consumer splits.

*Grounded on: ratified (`91:563-570`, `91:645-651`, `91:676-679`), measured (`100_probes/probe_3`,
exact integer, 131072-case exhaustive sweep, the false first claim recorded with its mismatch count),
reasoned (the derivation of the coincidence condition, checked to zero mismatches rather than argued).*

### 2.4 Why-evidence: what a two-vocabulary rank-2 shape ships today

Factual check at source, why-evidence only, not read for meaning. In `arvo-bitmask/src/matrix.rs`,
`BitMatrix<W, C: Capacity>` carries `pub rows: C::Array<Mask<W>>` at `:34`. `edge` guards the **row**
index against `cap_size(C::CAP)` at `:52-55`. `set_edge` guards the same row index at `:63-66` and
then calls `self.rows.as_mut()[row_idx].insert(j.0)` at `:67` with the **column** index passed
through unguarded. The column's only bound is `W`'s physical bit width.

The two axes of one rank-2 shape are declared in two vocabularies: the row extent is a `Capacity`, the
column extent is a `Bits<N>` width. Only one of them has a shape to be checked against.

Reproduced standalone and executed (`probe_7`, four claims, nothing depends on building the tree):

- a write past the declared **row** extent is refused and the byte image is untouched;
- a write past the declared **column** extent lands, reads back true, and moves the byte image;
- two values equal at every index the shape declares have **different byte images**, so the free
  raw-buffer digest shortcut of `91:643-651` separates them;
- one guard generated from the shape covers both axes, and the images then cannot diverge.

The third of those is the mutation gap `91:598-627` quantifies for one dimension, reached at rank 2
through an ordinary **safe** call, with no unsafe, no transmute and no niche. `91:590-596` already
established that a niche's validity range is a warn-level lint while a field shrink is a hard `E0004`,
"a difference in kind, not degree". This is the same distinction at the shape level: an axis with a
declared extent has a refusal available; an axis whose extent is only a container width does not.

`bitmatrix.rs:119-124` is the test that would have found it and does not, because it checks the
out-of-range read on an **empty** matrix, where the bit reads false for want of a writer. The setup
helps. I am not proposing a fix to shipped source, which is out of the panel's scope; I am naming why
the shape chapter needs a per-axis extent with a refusal behind it rather than a naming convention.

*Grounded on: verified at source (`arvo-bitmask/src/matrix.rs:29-68`, why-evidence only), compiled
and executed (`100_probes/probe_7`, four claims), ratified (`91:598-627`, `91:643-651`, `91:590-596`),
reasoned (the reading that the asymmetry is a vocabulary fact rather than a missing bounds check).*

### 2.5 What the rank recursion costs, measured

The fourth design rule makes compile time a bucket to pour into, so the question is not whether the
recursion is free but whether it is bounded and what the runtime buys. Both halves measured
(`probe_8`), and the round's own worry named directly (`202607281220` addendum: the `shapeless` and
`frunk` compile-time warning "does not transfer ... a per-width impl table is flat breadth", and D4's
cost question is "real, narrower than feared, and a bench question").

**Compile time is flat in rank.** `--emit=metadata`, best of three: 82, 81, 80, 82, 85, 82, 82, 84,
82 ms at ranks 1, 2, 4, 6, 8, 10, 12, 14, 16. Metadata grows linearly at roughly 142 bytes per axis
(24584 bytes at rank 1, 26715 at rank 16). **The marginal cost of an axis is below rustc's own
process-start noise at every rank measured**, so the answer to the round's open cost question is that
there is nothing to bench on this axis.

**The rank-generic fold folds away at small `COUNT`.** At rank 3, `COUNT` 8, `total` is five
instructions:

```asm
	mov	w8, w0
	add	x8, x8, x8
	add	x8, x8, x8
	lsl	x0, x8, #1
	ret
```

At rank 8, `COUNT` 256, thirteen instructions, vectorised to NEON (`uaddl.2d` then four `add.2d`). The
nested build and the recursive walk are both gone. That is D43's own "a per-element surface could look
like it forfeits the cheap path; it does not" measured rather than asserted.

**Where it stops being free, and what the variable is.** Holding `COUNT` fixed and varying rank, the
emitted instruction count for `total` is: at `COUNT` 4096, 44 at rank 1, 76 at rank 2, 248 at rank 3,
168 at rank 4, 423 at rank 6, 907 at rank 12. Non-monotone in rank, and at `COUNT` 256 the flat case
emits 62 where rank 2 emits 11, because the nested case vectorises and the flat one does not. **That
is a codegen artifact, not a design fact**, and the design should read nothing off it; per
`bench-in-bench-harness-never-sketches.md` it is bench work in `mock/benches/`, which the concurrent
dispatch owns this round.

**One design-relevant fact does fall out.** The stack allocation is a function of `COUNT` alone:
262144 bytes at `COUNT` 65536 for both rank 2 and rank 4, emitted with a stack-probe loop. A dense
shape is a stack object in a design with no heap, so `COUNT * size_of(E)` runs into the platform's
stack limit and into nothing the design controls. Worth one sentence in the chapter rather than
leaving a consumer to find it at rank 4 by extent 16.

Two cases did not finish inside the probe's budget (`COUNT` 65536 at ranks 8 and 16) and are recorded
as unmeasured rather than dropped.

*Grounded on: measured (`100_probes/probe_8`, commands and full tables in `OUTCOMES.md`, toolchain and
target stated inline), ratified (the pricing pillar, `91:113-126`), settled shapes (`202607281220`
addendum's flat-breadth finding, now measured rather than argued).*

---

## 3. Geometry, the multilinear half

### 3.1 The composition is free, and this is the answer to the dispatch's second question

The question is whether every accumulator-sufficiency condition, growth class and published fact
composes, or whether it multiplies into something impractical. It composes, and the reason is that
every operation in the multilinear half is a sum of products, which is two settled classes stacked.

Exact widths, derived and enumerated (`probe_4`, claim D). Let `p` be the numeral's precision:

| Operation | Exact accumulator width | Why it is not new |
|---|---|---|
| dot product, rank n | `2p + ceil(log2 n)` | `mul_full` at 2p, then the fold at `w + log` |
| affine apply, rank n | `max(2p + ceil(log2 n), p + F) + 1` | a dot product plus a translation at the operand's own quantum |
| rotor compose, rank n | `2p + ceil(log2 2^(n-1))` | the geometric product is bilinear: one product layer, one fold |
| rotor sandwich, rank n | `3p + ceil(log2 T)` | two product layers, so 3p rather than 2p |

**No new growth class.** File 43's three (addition linear, multiplication linear at 2p, division
exponential) plus file 99's roots absorb the whole multilinear half without an edit, which is the same
corroboration file 99 drew for the failure taxonomy and is worth as much here.

**The one operation that leaves the multilinear world is renormalisation, and its exponential class is
reached in the statement and never in the storage.** A norm needs a square root, which file 99 has
just placed in the **linear** class with a three-instruction branchless rounding decision and ties
impossible by parity (`99` section 2). Then it needs a reciprocal, and general division is the
design's one exponential class (`91:288-291`). But the exponential width there is the width of the
exact **quotient as a numeral**, and normalisation never materialises one: it is
`quantize(exact quotient)`, whose carrier is the Euclidean pair, not a wide numeral. So geometry
touches the exponential class exactly once, in a sentence, and never in a byte.

**This closes an open item file 74 named.** `74:65` records that `arvo-geom` carries "D10's motors
[which] need normalisation, which sits behind division's hold (`68:357-358`)". The hold was lifted at
`90b` and the surface adopted at `91` section 1.13, with the `x/0` fork dissolved at `95b`. So the
obligation is discharged rather than merely unblocked, and the discharge is the paragraph above.

**Composition depth is exponential, and it is not a discriminator.** Composing k transforms exactly,
with no quantiser, takes 16 bits to 1036 at depth 6, **identically for every representation**
(`probe_4`, claim E). The design's answer is already written and needs no geometry-specific text: a
quantiser fires per composition and the site count is a function of the monomorphised type
(`91:373-377`). What geometry adds is the observation that depth is a type-level fact exactly when the
transform chain is typed, which is the only way that count is computable at compile time, which is the
fourth design rule pointing at how a transform chain should be spelled.

*Grounded on: ratified (`91:288-291` division's class, `91:373-377` the site count, `91` section 1.9
the multiplicative half, `91` section 1.8 the fold), settled shapes (`99` section 2 the root class,
`74:65` the open item this closes, `43` sections 2 and 5), compiled (`100_probes/probe_4`, claims C
through E, exact rational Clifford algebra), reasoned (the width table's derivation from the settled
classes, and the reading that normalisation reaches the exponential class only in its statement).*

### 3.2 D10's rotor component count is wrong from rank 4, and the correction changes the argument

D10 (op, `202607281220`) grounds rotation on rotors, and gives its reason:

> Rotors are the general form ... at rank N a rotor carries a scalar plus n(n-1)/2 bivector
> components, which is exactly the degrees of freedom a rotation has ... a matrix is a poor general
> form, carrying N squared components for n(n-1)/2 degrees of freedom.

**Degrees of freedom and storage are two different counts, and the argument uses the first where it
needs the second.** `SO(n)` has dimension `n(n-1)/2`, which is right. `Spin(n)` sits inside the even
subalgebra of `Cl(n)`, whose dimension is `2^(n-1)`, and a rotor stored as coordinates in a basis
needs that many numbers because `Spin(n)` is a curved manifold inside that space and not a linear
subspace of it.

The two counts agree at rank 2 (2 and 2) and rank 3 (4 and 4, the quaternions) and diverge from rank
4 on: 7 against 8, 11 against 16, 29 against 128 at rank 8 (`probe_4`, claim A).

**The divergence is not bookkeeping.** Exhibited with exact rational unit rotors, so no float and no
approximation is involved (`probe_4`, claim B):

```
R1 = 3/5 + 4/5 e01,  R2 = 5/13 + 12/13 e23
R = R1 R2  ->  3/13 + 4/13 e01 + 36/65 e23 + 48/65 e0123
R R~ = 1
```

The grade-4 part is `48/65`, nonzero, and `R R~ = 1` so this is a legitimate unit rotor and not a
stray. The `1 + n(n-1)/2` slots cannot hold it.

**And the consequence runs the wrong way for D10's own argument.** Rotor storage first exceeds matrix
storage at rank 7: 64 against 49. So the count D10 rejects the matrix on reverses at rank 7 under the
correct count, and D10's own count would never show it because `1 + n(n-1)/2 < n^2` at every rank.

**What this does not touch.** D10's decision stands on grounds it also gives and which survive
unchanged: rotors generalise where quaternions do not, motors extend to rigid motion in one composable
object, and `Affine` is a rigid motion plus scale so this is directly the type geometry wants. Nothing
above argues for a matrix. It argues that **the storage comparison in D10's reasoning is wrong from
rank 4 and reverses at rank 7**, which matters because D41's bench matrix will be read against exactly
that comparison.

**A third fact the bench needs, and which the storage table hides.** The closed-form bivector
exponential, which is what makes rotor interpolation cheap and is the real reason to prefer a rotor
over a matrix under repeated composition, survives to rank 4 (where the split into two commuting
simple bivectors is closed form via the dual). At rank 5 and up it is a degree-`floor(n/2)` root
problem, and at rank 10 Abel-Ruffini forecloses a closed form entirely (`probe_4`, claim F). That is
the same ceiling this round already recorded from its own prior-art pass for exact distance to a cubic
Bezier, arriving in a second place.

*Grounded on: ratified (D10, `202607281220`, op), compiled (`100_probes/probe_4` claims A, B, F, exact
rational Clifford algebra over `Fraction`, the 4D counterexample exhibited in full), external
(Abel-Ruffini, cited not compiled, per `91:209-212`), reasoned (the reading that the argument
substitutes degrees of freedom for storage; the correction is arithmetic).*

### 3.3 The identity rotation is not representable in the numeral geometry reaches for first

Every quantity geometry normalises lives in a **closed** interval: a rotor component and a direction
cosine in `[-1, 1]`, a colour channel and a barycentric weight in `[0, 1]`. The obvious numeral is
purely fractional, and `78:723` already records that `UFixed<0, F>` has no representable one, its raw
encoding being `1 << F` when the container is exactly `F` bits. File 99 then found the same absent
element opens sqrt's overflow band at exactly those numerals (`99` section 2, "the same absent element
breaks the multiplicative identity and opens the root's overflow band").

**So the design's two known defects meet at the one operation the rotor formulation exists to make
cheap.** A normalisation calls sqrt, over the numeral with no one, to produce a rotor whose identity
is the value that is missing.

Compiled in const position at every F from 1 to 24, exhaustively over the endpoint (`probe_5`, claim
A): the unsigned dyadic form misses `+1`, the signed form reaches `-1` and misses `+1`, and **the gap
to `+1` is exactly one quantum at every width**. That is the worst place for it. No rounding mode
recovers a value one quantum outside the set, and the identity rotation is the operand every
composition chain starts from.

**The fix is a value of a parameter the design already seals, and I got this wrong once before
checking.** My first derivation proved no numeral in the grammar can carry the value set
`{k/(r^F - 1)}` at `F` digits, by a parity argument on `r^p = r^F` against `r^-q = r^F - 1`. That
proof is correct and irrelevant, because it forgets `Adjustment`. `40:493` gives
`Exponent = Implicit<E, A: Adjustment, B: Bias>` with `A` and `B` gcd-normalised rationals, the MATLAB
slope-bias model, so a value is `A * m * r^E + B`. Setting `A = 1/(r^F - 1)` gives exactly the closed
interval, both endpoints exact, at the same container width. Compiled at every F from 1 to 24
(`probe_5`, claim B). That is Direct3D's UNORM rule, which this round already recorded from the
sibling colour pass, and it needs **no new mechanism**.

**What it costs, stated rather than smoothed.** The adjustment composes multiplicatively, so a product
of two closed-interval values lands at `1/(r^F - 1)^2` and renormalising is division by the fixed,
nonzero, representable constant `r^F - 1`. `91:288-291` names that the exact subfamily "at zero new
mechanism". Checked: one acts as one exactly, at every operand, at every width to 20 (`probe_5`, claim
C). So the closed-interval route pays a constant divide per multiply where the dyadic route pays a
shift, and the constant divide is already priced.

**The two routes, and the choice is a real fork rather than a style preference:**

- **One bit of integer headroom.** Keeps the dyadic quantum and the shift, spends one bit per
  component: `2^(n-1)` bits for a rotor against `n^2` for a matrix, crossing over at the same rank 7
  (`probe_5`, claim D).
- **The rational adjustment `1/(r^F - 1)`.** Both endpoints exact at the declared width, at the cost
  of a constant divide per multiply, and it spends one bit pattern in the signed case, which is a
  niche of exactly the kind `91` section 1.12's `NicheCarrier` vocabulary already governs.

Per `arvo-toolbox-not-policer.md` the design should expose both and name the trade, not pick one
silently. What it should not do is ship the numeral that has neither.

**The general sentence I suggest for the record**, because this is not a geometry fact:

> A numeral whose value set is a closed interval, which is what a direction cosine, a rotor component,
> a normalised colour channel and a barycentric weight all are, requires either one radix digit of
> headroom above the fractional part or the rational adjustment `1/(r^F - 1)`. The purely fractional
> dyadic numeral is exactly the one that carries neither endpoint, and it is the one a consumer reaches
> for first.

*Grounded on: ratified (`40:493` and `91:153-154` the Adjustment grammar, `91:288-291` the exact
subfamily, `78:723` the missing one), settled shapes (`99` section 2 the root overflow band, the
sibling colour pass's UNORM finding recorded at `202607281220`), compiled (`100_probes/probe_5`, four
claims, const position, exhaustive over F to 24), reasoned (the two-route framing; my first
impossibility proof is recorded as wrong because it omitted `Adjustment`).*

### 3.4 One fact handed back across a boundary

`202607281220`'s addendum records the sibling colour pass's finding and flags it as falling between two
rounds: "Whether a UNORM-shaped type belongs in arvo, in kirjo, or nowhere should not fall between the
two rounds."

Section 3.3 answers the arvo half of it, and the answer is that the question was mis-shaped. **A
UNORM-shaped type does not belong anywhere, because it is not a type.** It is `Adjustment =
1/(r^F - 1)` on an ordinary significand, which is a value of a parameter the identity contract already
carries. So arvo owes `kirjo` no new type and `kirjo` owes arvo no request; what arvo owes is one
sentence saying the parameter reaches this case, and the closed-interval sentence above is that
sentence. The colour channel and the rotor component are then the same numeral wearing two names,
which is exactly the compression the design is for.

---

## 4. Geometry, the model-choice half

### 4.1 What I do not decide

D41 (op, `202607291910`) settles the rotation representation "by a broad bench, after the requirements
are named", and states the ordering explicitly: "The requirements and constraints get identified and
written down first, and then the bench matrix covers as many variants as can be thought of."  D11 (op)
does the same for curves. I do not pick, do not narrow the matrix, and do not reopen either. Section 3
supplies arithmetic the bench will be read against; sections 4.2 through 4.4 supply the requirements
D41 asks for first, which is the artifact that is owed and does not exist.

### 4.2 The bench matrix cannot be run at rank 3 alone, and that is not a preference

At rank 3 the even subalgebra of `Cl(3)` **is** the quaternions, which D10 says in as many words.
So a bench comparing "rotors" against "quaternions" at rank 3 compares one representation against
itself and measures the two implementations, not the two representations. `probe_6` compiles the two
component counts side by side so the point is a diff rather than an argument: they agree at ranks 2
and 3 and separate from rank 4.

That is the separation requirement's own shape, applied to an artifact nobody has applied it to. Rank
3 is the instantiation everyone reaches for; it is exactly where the distinction the matrix exists to
measure is vacuous. **The matrix must include at least one rank of 4 or above**, or its generality
claim, which is D10's whole reason for preferring rotors, goes unmeasured. Rank 4 is not academic
here: a motor for 3D rigid motion is an element of a four-dimensional projective algebra, which is
precisely the rank where D10's count first goes wrong.

### 4.3 The requirements, derived from what the design has settled since D41

D41 names five: "fixed-point representability of each form, which operations must be cheap (compose,
apply to a point, invert, interpolate, renormalise), how each degrades under repeated composition at
finite precision, storage width, and whether the degenerate metric is needed at all for arvo's cases
or only for ikiuni's." Each now has content it did not have on 2026-07-29. Suggested, per row:

| Requirement | What the design now says it means |
|---|---|
| fixed-point representability | Which numeral carries the components, answered by section 3.3: the closed-interval question, with its two priced routes. A form whose components are unbounded (a Cayley or Rodrigues parameter, which runs to infinity at a half-turn) needs the far-point rule (`91` section 1.16), which is a different answer from a bounded form's and should be stated per row. |
| which operations must be cheap | Each of compose, apply, invert, interpolate, renormalise names its **growth class and its exact accumulator width** from the table in section 3.1, before any timing. Two entries are free rather than cheap and the bench cannot see it: a rotation matrix inverts by transposition and a rotor by a sign flip on its odd-grade parts, both zero arithmetic. |
| degradation under repeated composition | Restated in the design's own vocabulary as the **site count** (`91:373-377`): how many quantiser sites per composition, and what the worst-case error bound at that count is. That is a compile-time number, not a measurement, and it should be derived per row before the bench so the bench checks it rather than discovers it. |
| storage width | `2^(n-1)` for a rotor, `n^2` for a matrix, `n(n-1)/2` for a bivector-plus-exponential form, with the crossover at rank 7 (section 3.2). Per row, and in bits after section 3.3's numeral choice, not in components. |
| the degenerate metric | Unchanged, and genuinely op's. |

**And three the design has acquired since, which D41 could not have named:**

- **Whether the form's identity is representable.** Section 3.3. A representation whose identity is
  one quantum off the grid fails before any timing runs, and this disqualifies a form rather than
  ranking it.
- **Whether the exact carrier exists at all.** File 99's carrier-kind sort applies here unchanged. A
  form requiring `exp` or `log` of a bivector reaches the transcendental class and inherits its
  licence structure (`99` section 4): correct rounding only where a hardness const is exhausted or
  cited, licensed approximation with a type-level error bound otherwise. The Lie-algebra exponential
  map on `so(3)` and `se(3)`, which is on D41's own list of variants, is exactly this row, and it is
  the only row on the list that is not multilinear.
- **The measure of the bench itself, per the pricing pillar's standing test.** Every per-element or
  per-step quantity that is a function of the type's parameters alone belongs on the type as an
  associated const rather than in a `const fn` called from value position (`91:113-126`). For a
  rotation kernel that is the component count, the blade sign table, the sandwich's term list and the
  accumulator width. `91:663-682` records that missing this once turned a 4.6x figure into 1.50x, so a
  bench built without it will measure the decoder rather than the layout, again.

### 4.4 D10's flagged const-expression hazard dissolves, and the fallback compiles

D10 names one concern and one fallback:

> A rotor's 1 + n(n-1)/2 components is a computed extent, and putting that in type position is the
> const-expression hazard `Capacity` exists to avoid ... The fallback is in any case the hlist itself,
> since a bivector basis is the set of 2-subsets of the axes and so is derivable by a type-level fold.

The prior-art pass reports no library derives a basis that way: "No library was found that derives the
bivector basis as a compile-time type-level fold over an axis list", against three strategies in
industrial use (bake at generation time, generate per signature at build time, defer to a runtime
JIT). **That is an absence, not an impossibility, and it is worth separating the two.** Compiled
(`probe_6`, claims A and C, zero feature gates):

```rust
pub trait TwoPow { type P: Pos; }
impl TwoPow for Scalar { type P = H; }              // 2^0
impl<Hd, Tl: TwoPow> TwoPow for Axis<Hd, Tl> { type P = O<Tl::P>; }  // one constructor, not a multiply

pub trait RotorExtent { type P: Pos; }
impl RotorExtent for Scalar { type P = H; }
impl<Hd, Tl: TwoPow> RotorExtent for Axis<Hd, Tl> { type P = Tl::P; }  // 2^(n-1), the list decrements
```

Not one arithmetic expression appears in a type. Each step is impl selection, which is the family
`91:811-814` already names (`Dec` and `PosPred` recursing "structurally on the constructor shape ...
the identical family as `VAL`, `Cmp`, and `Gcd`"), and the decrement is the list's own shape rather
than a subtraction. Checked at ranks 2, 3, 4, 5 and 8 against 2, 4, 8, 16 and 128.

The derived extent then pairs with its literal exactly as section 1.26 requires, so the rotor's
storage is an ordinary rank-1 shape with no rotor-specific capacity machinery anywhere. And the wrong
literal is refused, which is worth showing because the wrong literal here is D10's own count:

```
error[E0080]: evaluation panicked: capacity's declared length disagrees with its value
   evaluation of `<Slot<Pz<O<O<O<H>>>>, 7> as Capacity>::AGREES` failed here
```

`Pz<O<O<O<H>>>>` is 8. The literal 7 is `1 + n(n-1)/2` at rank 4. **The mechanism catches the count
error of section 3.2 at compile time, without being told about it.** That is the strongest argument I
have for the section 2.2 repair, and I did not construct it: while generating probe 8's fixed-count
cases I wrote a `Pos` literal of 128 for an extent of 256, and the same assert caught that too, before
any number was read.

*Grounded on: ratified (D10, `202607281220`, op; `91:811-814` the structural-recursion family;
`91:796-802` the pairing), settled shapes (the prior-art pass's absence finding, `202607281220`
addendum), compiled (`100_probes/probe_6`, three claims, zero gates, the refusal reproduced),
reasoned (the reading that an absence of prior art is not an impossibility, which the compile settles).*

### 4.5 A locus suggestion, offered and not ruled

`202607281005` sent layout to `lato` on a test I think should be applied one crate further in: layout
"is policy over shapes rather than arithmetic on them, a different axis from everything arvo holds
today, and named as the axis along which arvo would become a monolith if it absorbed layout."

Apply it to `arvo-geom` as the round left it. The multilinear half (point, orthotope, affine,
predicates, measure) is arithmetic on shapes and depends on nothing but a shape and a numeral. After
section 3 it has no open question. The model-choice half (rotation's algebra and convention, curves'
representation) depends additionally on a metric signature, a convention, and eventually on the
transcendentals, and it is entirely open questions held behind two benches.

That is a seam, and `a-homeless-document-is-a-design-problem.md`'s reading applies to a crate as well
as to a document: a thing that resists filing is usually doing more than one job. **D2's own contents
list for `arvo-geom` is already the multilinear half**, and rotors and curves arrive from D10 and D11
without naming a crate, so this is surfacing a split latent in op's decisions rather than proposing
against them. My suggestion, and it is a suggestion: settle the multilinear half now, and let the
algebra sit downstream of it and of the elementary-functions chapter until D41's bench runs. The
alternative, one crate that cannot ship until a bench that has not been designed resolves a fork
between two conventions, delays a finished half behind an unstarted one.

**And one thing the design should not do, which follows from a rule it already has.** The PGA
convention fork is real (the round asserted it, retracted the assertion on two shallow fetches, then
retracted the retraction against Lengyel's own primary text, all within one day and all recorded at
`202607291910`). `arvo-toolbox-not-policer.md` says the substrate exposes the choice where the
consumer is the one who knows the answer. A convention is exactly that: it changes which operations
are cheap and how normalisation behaves, and both bivector.net and Lengyel ship code and have
consumers. So the convention wants to be a type parameter in the way `Strategy` is, with the bench
measuring instantiations rather than resolving a fork on arvo's behalf. The basis derivation in
section 4.4 is convention-independent, since it counts the even subalgebra and says nothing about
which duality names which blade, so nothing above prejudges it.

---

## 5. What this file does not decide

**Which rotation representation wins.** D41's, by bench, and section 4.3 is the input to it rather
than a substitute for it.

**Which curve representation wins.** D11's, by bench, untouched.

**Whether `arvo-geom` splits.** Section 4.5 is a suggestion with its test named. Op's.

**Whether the section 2.2 repair lands as written.** It compiles and its scope is stated; it is
one-pass, and the second read should attack whether making `AGREES` a `Capacity` member widens the
trait's obligation for a capacity that has no literal to pair with, which my model does not exhibit
because every capacity in it is a `Slot`.

**The signed half of the closed-interval numeral.** `probe_5` models the signed field's endpoints but
does not build the spent-pattern niche against `91` section 1.12's `NicheCarrier` vocabulary, which is
where it would have to live. Owed, one compile.

**The bit domain's own layout projection.** Section 2.3 derives which axis is packed; it does not
build the projection. Owed, one compile against a real `Layout`-bearing shape, and it is the same
artifact the `foldnum` compile already owes in a different chapter.

**Anything about runtime cost.** No timer ran. The emitted-code counts in section 2.5 are static
instruction counts under `-O` on one target, and the harness is the concurrent dispatch's this round.

## 6. The two requirement performances, on this text, before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions.
*Multilinear half* (defined, section 1: the part of geometry whose operations are sums of products,
namely point, orthotope, affine map, their predicates and their measure). *Model-choice half*
(defined, section 1: the part requiring a choice of algebra or representation before any arithmetic,
namely rotation and curves). *Flattened reading* and *per-axis reading* (defined, section 2.3, by the
two packings they name). *Rowpad* (defined, section 2.3: `(-inner*W) mod 8`, the padding one packed row
carries). *Closed-interval numeral* (defined, section 3.3: one whose value set includes both endpoints
of the interval it spans). *Storage count* against *degrees of freedom* (defined, section 3.2, by the
two things they measure: coordinates in a basis, and the dimension of the manifold). Terms used from
the record without redefinition: capacity, the three width levels, statement C, the site count, the
far point, the growth classes, the exact subfamily, the quantiser, `Adjustment`, `NicheCarrier`, and
the pricing pillar's standing test. **Named open rather than defined**: the *convention parameter* of
section 4.5, which I name and do not define, because defining it presumes the fork's resolution and
that is D41's; and the *layout projection* of section 2.3, whose trait I compile a dense instance of
and do not state in general. Checked by grep over this file's own emphasised and quoted terms.

**The separation requirement, performed.** This file carries two models and each is checked where it
is nonvacuous.

The first is section 2's split of `Shape` from its storage projection. What it separates: *the index
domain* from *the bytes that carry it*. Nonvacuous at the bit domain, which is the instantiation
chosen because a wrong model coincides everywhere else: at the dense value domain the two are the same
nesting and a model conflating them reads correct, while at the bit domain they diverge in kind, since
D43's own correction rejects `[[bit; 4]; 3]` and the container projection answers the storage question
one layer down. Section 2.4 is the same separation observed in shipped code: `BitMatrix`'s two axes
are one index domain expressed in two vocabularies, and only the one with a shape behind it has a
refusal. At a rank-1 dense shape all of these distinctions are vacuous, and my section 2 verdicts
there rest on the compile alone.

The second is section 3.2's separation of *storage count* from *degrees of freedom*. Nonvacuous at
rank 4 and above, and chosen for exactly that reason: at ranks 2 and 3 the two counts are equal (2 and
2, 4 and 4), so a model checked at the rank every reader reaches for cannot tell them apart, which is
how the count survived from D10 to here. `probe_6` compiles both counts in one file so the separation
is a diff. At rank 3 my section 3.2 verdict has no content, and I say so rather than let the table
imply otherwise.

**The honest limit of both performances**, inherited from `97` and restated by `98`: they verify that
my terms are placed and my models have content. They do not verify that my verdicts are correct. This
file is one reader's first pass over two subjects with ninety-nine files of context behind them, and
its most attackable judgements are the section 2.1 trait split (which reads D43 against D4's own
wording and could be read the other way) and the section 4.5 locus suggestion (which applies a test
from a different round to a crate op has already listed).

## 7. Standing

Shape is not a geometry subject. It is the index-domain layer, its rank-1 case is already ratified as
`Capacity`, and D4's list of capacities survives the capacity resolution verbatim, with per-axis
extents, a rank-generic function body, and a compile-time cost below rustc's own startup noise at
every rank to 16. What it needs is one sentence D43 already wrote and D4's wording blurs, separating
rank and extent from the bytes that carry them, and one clause closing a door that section 1.26
believed was singular and is not above rank 0. The bitpacked question at rank N answers to the group
arithmetic already ratified, with the flattened reading never worse and the coincidence condition
derived exactly rather than sampled.

Geometry's multilinear half composes over the settled operation surface at no new growth class, no new
failure kind and no new vocabulary, and touches the design's one exponential class only in a sentence.
Its model-choice half is behind two benches that are op's, and what it was missing is the requirements
list D41 asked for first, which section 4.3 supplies with three rows the design has acquired since.
Two counts in the round's own reasoning are wrong: a rotor's storage is the even subalgebra rather than
the bivector count, diverging from rank 4 and reversing D10's argument at rank 7, and the numeral a
consumer reaches for to hold a rotor component is exactly the one with no representable identity,
which the ratified rational adjustment fixes at no new mechanism.

Every spec-shaped sentence above is offered in the consolidation's provenance form for the next
consolidation to take or strike. The locus suggestion, the bench matrix's contents and the two benches
themselves are op's. Only op's calls are final, and even those go stale.

*Grounded on: ratified (`91` sections 1.4, 1.8, 1.9, 1.12, 1.13, 1.14, 1.16, 1.22, 1.26 as cited in
place; the design round `202607300800`'s D1 through D13, D40, D41, D43 and D44, all op), settled
shapes (`74:64-65`, `98` section 4, `99` sections 1 through 4, `11:43-56`, `40:440-500`,
`202607281616_prior_art`'s absence findings as recorded in `202607281220`'s addendum), compiled
(`100_probes/probe_1`, `probe_2`, `probe_5`, `probe_6`, `probe_7`, all at the pin, zero feature
gates), measured (`100_probes/probe_3` and `probe_4`, exact integer and exact rational;
`100_probes/probe_8`, compile time and emitted code, commands and target stated inline), verified at
source (`arvo-tensor/src/capacity.rs:46-52` and `:115-121`, `arvo-bitmask/src/matrix.rs:29-68`,
why-evidence and test-gate only), external (Abel-Ruffini, cited not compiled), reasoned (the
enumeration of section 1, the trait split of section 2.1, the requirements of section 4.3, the locus
suggestion of section 4.5, mine, offered as suggestions and not as rulings).*
