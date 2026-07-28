# Shared baseline for the arvo prior-art research

**Date:** 2026-07-28
**Kind:** research context, not design
**Read this first.** Every deliverable in this directory was written against this document.

This is the common ground for a set of parallel research passes into what arvo should rest on. It
states what arvo ships today, what restructure has been prescribed for it, what constraints any
answer has to survive, and what a research deliverable here is and is not allowed to do.

The single most important line: **nothing in this directory decides anything.** The design calls
belong to the lead designer, and the design round is where they get made. Research supplies the
material those calls are made from. A deliverable that arrives at a recommendation has exceeded its
brief; a deliverable that lays out four candidates with their real costs and names which questions
separate them has done the job.

## Provenance, and how much trust each thing here is owed

Authority in this workspace comes from human ratification, and it does not accrue from an artifact
looking finished or having sat in the tree a long time.

The governing documents for this work are the design-round topic files under
`arvo/mock/design_rounds/`, specifically `202607281220_topic.the-ndim-and-shape-design.md` and
`202607281547_topic.identity-already-exists-and-what-algebra-still-needs.md`. Those record decisions
made with the lead designer in the loop, and mark them inline as `**Decision (op).**`. Where this
document and those files disagree, those files govern and this one is wrong.

Everything else, including this document, including the shipped arvo source, and including every
deliverable written into this directory, is agent output produced without a recorded human decision.
It is presumed wrong where it conflicts with the ratified design. Agreement between two unratified
artifacts is not corroboration, because agents copy each other's framing; it is shared drift.

The practical consequence for a research pass: the shipped tree described below is a **set of claims
to test**, not a description of what is correct. If the prior art says arvo's current shape is wrong,
say so, cite the source, and do not soften it.

## What arvo is

A numeric and analysis foundation. Fixed-point primitives carrying a strategy marker, bit-level
contracts, and a small set of algorithm crates over them. Sixteen crates today.

The constraints are not negotiable and every candidate has to survive all of them:

`#![no_std]` in every crate. No `alloc`, no `Vec`, no `String`, no `Box`, ever. Sizes are const at
type level, so nothing grows at runtime. No `dyn`, no `TypeId`, no `std::any`; monomorphisation is
the dispatch. No platform dependency at all, so no threads, no clock, no filesystem, no I/O.

Fixed point is the primary representation, and floats are a tagged wrapper rather than the default.
The toolchain is a pinned nightly (`nightly-2026-05-28`) with a deliberately vetted set of unstable
features, so "that needs a nightly feature" is not by itself an objection. `generic_const_exprs`,
`adt_const_params`, `const_trait_impl` and the const-traits family are all already in use.

Two standing principles shape what an acceptable answer looks like. **arvo ships tools and never
polices its consumers**: it exposes the full lattice of choices with the tradeoffs documented, rather
than picking a threshold on the consumer's behalf. And **the public surface and the implementation
have different rules**: the surface is typed and const-generic, while the internals are free to use
SIMD intrinsics, hand-written assembly, float intermediates, or anything else that benches faster,
because the newtypes are `repr(transparent)` and the unwrap is free.

## What arvo actually ships, by crate

Surveyed from `mock/crates/*/src/lib.rs` on 2026-07-28. This is the export surface, not the impls.

| Crate | Public surface |
|---|---|
| `arvo-transparent` | `Transparent`, `NumericPrimitive`, `ConstDeref`, `ConstAsRef`, free `raw()` |
| `arvo-strategy` | `Strategy` and the markers `Hot` / `Warm` / `Cold` / `Precise`; `Signedness`, `Unsigned`, `Signed`; `Resolve`; `Identity<Op>`, `Additive`, `Multiplicative`, `Bounded`, `SignedIdentity`, `OneRepresentable`; `UArith`, `IArith`, `USaturating`, `ISaturating`; `BitsContainerFor`, `Picker`, `Project`; `CrossStrategyOp`; `Ieee`, `FromU8Ieee`; `Width`, `width`, `bytes_for`; `WideBits`, `Align`; the axes family `Bitpacked`, `Dense`, `DoubleLogical`, `Min`, `OverflowPolicy`, `Saturating`, `Wrapping`, `StorageLayout`, `ContainerWidth`, `HasAxes`; `ConstFrom`, `ConstTryFrom`; widen and narrow families |
| `arvo-storage` | `Bits<N, S, Sign>`; `Bool`, `USize`, `NUSize`, `Cap`, `AsBool`, `BoolResidual`; `IBits`, `FBits`, `MetaCarrier`; the const bridges `ConstEq`, `ConstPartialEq`, `ConstBitEq`, `ConstOrd`, `ConstOrdering`, `ConstDefault` |
| `arvo-bits-contracts` | `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`, `BitPrim`, `IBitPrim`, `BitsBitPrim`, `UBitContainer`, `IBitContainer`, `Narrow`, `Narrowed`, `Widen`, `Widened`, `NarrowFromU64` |
| `arvo-mask-contracts` | `MaskOps` |
| `arvo-numeric-contracts` | `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, `Predicate` and the predicate family `IsZero` / `IsPositive` / `IsNonZero` / `IsNonNegative` / `IsZeroOrPositive` |
| `arvo-bits` | width aliases `Bit`, `Nibble`, `Byte`, `Word`, `DWord`, `QWord`; `BitsRefitCtor` |
| `arvo-bitmask` | `Mask`, `BitMatrix`, `NodeId`, `SetBitsIter`, `propagate_dirty`, `cap_size` |
| `arvo-tensor` | `Array<T, C>`, `Matrix<W, C>`, `Capacity`, `ConstCapacity`, `Dim<N>`, `cap`, `cap_size`, `Enumerator` |
| `arvo-refit` | re-export gateway for the narrow and widen family |
| `arvo-hash` | `Hasher`, `ConstHash`, `Fnv1a`, `XxHash3`, `ContentHash`, `fnv1a_64`, `xxhash3_64` |
| `arvo-graph` | `topo_sort`, `renumber`, `upward_rank`, `downward_rank`, `components`, `longest_path`, `spanning_tree`, `waist_detect`, `waist_detect_const` |
| `arvo-sparse` | `Csr`, `CsrBidirectional`, `rcm_reorder`, `block_diagonal`, Dulmage-Mendelsohn, adjacency |
| `arvo-comb` | `bin_pack`, `matrix_chain_dp`, `greedy_group`, `Range` |
| `arvo-spectral` | `Matrix`, `laplacian`, `fiedler_vector`, `power_iteration`, `LinearOperator`, `SparseLaplacian`, `spectral_bisection`, `k_way_partition`, `SpectralBipartitioner` |
| `arvo` | `UFixed<I, F, S>`, `IFixed<I, F, S>`, the aliases `Fixed` / `Int` / `Uint` / `Signed`, `FastFloat`, `StrictFloat`, `Float`, the `bitfield!` macro, the marker family, `Pred` / `Pred2` / `Pred3`, `EuclidDiv`, `ScalarEuclid`, `EvenShares`, `EvenSplittable`, and a wide re-export of everything below |

### The structural problem, verified from the manifests

`arvo` is documented as a terminal facade, and every `forbidden-imports` rule in `mockspace.toml`
from `arvo-transparent` through `arvo-hash` lists `arvo::*` as forbidden. The manifests say
otherwise. Seven crates carry a Cargo dependency on `arvo`: `arvo-bitmask`, `arvo-comb`,
`arvo-graph`, `arvo-hash`, `arvo-sparse`, `arvo-spectral`, `arvo-tensor`.

Meanwhile `arvo` itself depends only on `notko`, `arvo-transparent`, `arvo-strategy`, `arvo-storage`,
`arvo-bits-contracts`, `arvo-numeric-contracts` and `arvo-bits`. It does not depend on any of the
seven. So there is no cycle, which is why it compiles, but the crate that calls itself the facade
re-exporting the whole workspace is in fact a mid-level crate that seven downstream crates consume.

One rule is not merely violated but **unsatisfiable as written**. `arvo-tensor`'s rule forbids
`arvo::*` while `Capacity::CAP` is a `Cap` and `Cap` lives in the facade. The layer rule and the type
placement cannot both hold. The algorithm crates inherit the same contradiction through `UFixed` and
`IFixed`, which also live in the facade.

The diagnosed cause: arvo's convention is a contracts crate plus a concrete sibling, as
`arvo-bits-contracts` has `arvo-bits` and `arvo-mask-contracts` has `arvo-bitmask`.
`arvo-numeric-contracts` has no sibling, and its own README says the default impl bodies live in the
facade instead. The facade became load-bearing because it is standing in for a crate that was never
created.

## The prescribed restructure

Ratified in `202607281220`, D1 through D3. Four new crates, and the facade becomes a leaf.

`arvo-capacity` takes `Cap`, `Capacity`, `ConstCapacity`, `Dim<N>`, `cap` and `cap_size`, lifted out
from under both container crates so they become peers over one foundation. `arvo-shape` takes rank
and per-axis extent sequences, generic over rank. `arvo-numeric` takes `UFixed`, `IFixed`, `Int`,
`Uint`, the float wrappers and the default contract impls, completing the contracts-plus-sibling
pattern. `arvo-geom` takes `Point`, extent, box, `Affine` and the spatial aliases. `arvo` re-exports
and holds nothing.

**A shape is an hlist of capacities** (D4). Rank is the list length and extents are the elements,
with the backing storage the recursive composition of each capacity's array, so `Cons<H, T>::Array<E>
= H::Array<T::Array<E>>` and the leaf is a scalar. This survives the constraint that rules out the
alternatives, because rank and element count are associated consts and so the arithmetic stays in
value position and no const expression reaches type position. A stride-and-flat-length scheme fails
exactly there.

The hlist itself is extracted to `notko-hlist` (D5), carrying `Empty`, `Cons`, `Contains`,
`ContainsAll`, `Concat`, and a `Length<N: Cardinal>` whose count is **parameterised** rather than
fixed. The reason is an orphan-rule finding: notko cannot name arvo's `USize`, and a separate
counting crate in arvo cannot implement a notko trait for notko types. So the count parameterises in
notko and each consumer implements `Cardinal` for its own type. Each domain then aliases the cell and
the leaf to its own vocabulary (D7): `Axis` and `Scalar` in arvo-shape, `Access` and `Deny` in
hilavitkutin, `Command` and `EmptyInvoke` in kolli. Reuse is not forced (D8): a bit container wants
`[Mask<4>; 3]` rather than `[[bool; 4]; 3]`, and a sparse container mirrors no shape at all, so the
vocabulary is available and neither is obliged to take it.

**Rotation is grounded on rotors, extending to motors** (D10), not on quaternions and not on
matrices. Quaternions do not generalise, being a four-dimensional algebra special to 3D; what
generalises is the rotor in the even subalgebra of the Clifford algebra, which is the complex numbers
at rank 2 and the quaternions at rank 3, and carries a scalar plus n(n-1)/2 bivector components at
rank N. Motors from projective geometric algebra extend this to rigid motion, and a dual quaternion
is the 3D motor. Lineage named: Gunn's plane-based PGA, with Dorst and Lasenby.

**Curves are arvo's, and the representation is a bench matrix rather than a pick** (D11). Candidates
named: Euler spirals and clothoids, biarcs and piecewise-circular, implicit distance fields, and
classical Beziers. The winner is selected statically per workload so LLVM erases the alternatives.

**Colour is not geometry** (D12). Bit-level packing stays in arvo; the perceptual and raster domain
goes to `kirjo`.

**The algebra gap is narrow, and it is not what was first proposed** (`202607281547`). arvo already
ships `Identity<Op>` parameterised by ZST operation marker, with `Additive` and `Multiplicative`, one
`IDENTITY` const rather than separate `ZERO` and `ONE`, and an absence-is-a-statement principle: where
an operation has no identity in a type there is no impl, enforced by a sealed `OneRepresentable<TAG>`
witness so that `UFixed<0, F, S>`, which spans `[0, 1)`, has `Identity<Additive>` and no
`Identity<Multiplicative>`. What is missing is only the associative **combine** to pair with it.
Given that, `Monoid<Op>: Identity<Op> + Combine<Op>` is a supertrait line rather than a design. It
lands in `arvo-algebra-contracts` with `arvo-algebra` as the concrete sibling.

## What a deliverable in this directory is

One file per pass, named `NN_<topic>.md`, written by the agent that did the research, into this
directory. Not a report back to the dispatcher; an artifact on disk that outlives the session.

The framing that matters most: **the staple theory is table stakes, and the interesting part is the
recent and the unproven.** This stack has already left the mainstream on several axes at once, using
fixed point where the field uses float, refusing the heap entirely, putting sizes in the type system,
and treating the strategy tradeoff as a type parameter. Research that only surveys the settled
consensus will describe a world this project has deliberately stepped outside of. Find the papers
from the last five years, the talks, the blog posts by people actually building the thing, the
libraries with twelve stars that solved the exact problem, and the approaches that are known to work
but have not been productised. Say plainly when something is unproven, and say so as information
rather than as a warning.

State what you could not find. A negative result is a real finding: if nobody has published on
fixed-point rotors, that is worth knowing before the design round assumes a literature exists.

Cite properly. Author, title, venue, year, and a URL where there is one. A claim with no citation is
not usable in a design round, because the whole reason for this pass is that the previous round
reasoned from summaries and reproduced a mechanism arvo had already built in a better form.

Do not recommend. Do not decide. Do not write "arvo should". Lay out what exists, what it costs, what
it assumes, and which of the constraints above it violates or survives. Where two approaches differ,
name the question whose answer picks between them, and leave that question open.

Writing style, which is enforced in this workspace: **no em-dashes anywhere**, use periods, commas,
parentheses, colons or semicolons instead. No hype words, no marketing register, no exclamation
marks. Open with prose rather than a bulleted list. Tables need at least two columns and three rows
of real content. No ASCII box diagrams; a Mermaid fence is fine where the shape is the content.
