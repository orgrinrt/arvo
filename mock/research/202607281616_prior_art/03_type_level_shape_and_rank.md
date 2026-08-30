# Type-level shape and rank: the state of the field

**Date:** 2026-07-28
**Kind:** prior-art research, not design
**Governs this pass:** `00_context.md` (baseline, provenance ladder, what a deliverable here is), and
`mock/design_rounds/202607281220_topic.the-ndim-and-shape-design.md`, decisions D4 through D9. Both
are read and cited directly below rather than summarised from memory.

**Revised 2026-07-28, correcting a baseline error in the framing, not in the underlying literature.**
The first revision of this pass treated `generic_const_exprs` (GCE) as a live constraint arvo's design
leans on, following the description in the `00_context.md` this pass originally read, which stated GCE
was "already in use" across the stack. That description was itself wrong, and the error was in the
baseline document, not in this pass's reading of it. The corrected `00_context.md` states, verified by
grepping every crate root directly rather than by reading an inventory: `generic_const_exprs` is
forbidden outright by the lead designer, alongside full `specialization`, under a standing gate
("allowed only if not proven unsound or unstable, and, absent a very strong reason, itself on the
stabilisation path"), GCE fails that gate on its own terms because `min_generic_const_args` is its own
documented successor. Two commands confirm the corrected picture directly against this pass's own
read of source: `grep -n 'generic_const_exprs' mock/crates/arvo/src/lib.rs` and the equivalent for
`arvo-strategy/src/lib.rs` both still show a live `#![feature(generic_const_exprs)]` gate today, each
carrying an inline comment restricting its use to "const-expression bounds and const-fn `where`
clauses"; `mock/crates/arvo-graph/src/lib.rs:17` states plainly that `generic_const_exprs` "is not
needed. Working arrays are the capacity's [own array]"; and `mock/crates/arvo-spectral/tests/capacity_threading.rs:12`
states its own "GCE-escape proof is the absence of `#![feature(generic_const_exprs)]`." A sketch
(`mock/research/sketches/202607282100_container-projection-without-gce/`) has since reproduced the
remaining live use, arvo-strategy's Pattern C container projection, under zero feature gates of any
kind, and its findings record that the sketch "compiles clean with no `#![feature(...)]` gate of any
kind, on the pinned toolchain," settling that GCE is not load-bearing anywhere in arvo. Every section
below that discussed GCE has been corrected against this baseline. The correction changes the framing
of one finding, not its citations: the Rust project's own disavowal of GCE, cited below, was reported in
the first revision as a risk finding weighing against a mechanism arvo depends on. It is not that. It is
independent, external corroboration of a migration arvo had already made, in a language and a design
independent of arvo's own reasoning for making it. **Everything else in this pass, the shapeless and
frunk compile-time material, the Accelerate precedent, the Futhark size-type failure mode, Remora and
the rank-polymorphism distinction, `generic-array` and `typenum` as precedent for a count parameterised
by its carrier, the binary-over-Peano encoding note, and the folds material, is unaffected by the
correction and is carried forward unchanged.** A new section has been added at the end of the Rust-
specific material, addressing the cost question the corrected baseline actually raises: not what GCE
costs, since arvo does not use it, but what the mechanism that replaced it costs.

The question this pass was asked is narrower than "how does the field represent array shape." It is:
given that a shape is a rank-generic sequence of per-axis extents, and given that the reason for
choosing a cons-list of capacities over a stride-and-flat-length scheme is that rank and element count
must stay associated consts (value position) rather than becoming a const expression that reaches type
position, does the field's experience with this exact tradeoff support, contradict, or complicate that
choice. The field has been at this problem for over twenty years, under at least four different type
systems, and the record is more mixed than a single design round can see from the inside.

## The hlist lineage, and what building it at scale actually cost

`notko-hlist` (D5, D9) is a Rust incarnation of a line that starts with Oleg Kiselyov, Ralf Lämmel and
Keean Schupke, "Strongly Typed Heterogeneous Collections," Haskell Workshop 2004, ACM, pages 96 to 107
(https://dl.acm.org/doi/10.1145/1017472.1017488, preprint at https://okmij.org/ftp/Haskell/HList-ext.pdf).
That paper is the origin of the `HCons` / `HNil` shape and of using the structure for extensible records
as well as plain heterogeneous lists, which is precisely the D7 move of aliasing the cell and the leaf
per domain rather than shipping one fixed vocabulary.

The paper is table stakes here; what the design round needs is what happened after twenty years of
people actually building on it.

**Scala's shapeless** (Miles Sabin, https://github.com/milessabin/shapeless) carried the pattern into
industrial Scala 2 codebases and produced the sharpest documented cost data available anywhere in this
survey: a GitHub issue against shapeless reports lens composition over roughly 70 lenses, about 60 of
them nested and all used within one class, taking around ten minutes to compile, against a few seconds
without shapeless (https://github.com/milessabin/shapeless/issues/102, "Lenses compile time insanely
slow"). Compilation performance was widely enough recognised as a shapeless-adoption blocker that the
Scala compiler team wrote tooling specifically to profile implicit-derivation and macro-driven compile
time (Adriaan Moors et al., "Speeding Up Compilation Time with scalac-profiling," Scala blog, 2018,
https://www.scala-lang.org/blog/2018/06/04/scalac-profiling.html), and the summary from that lineage is
blunt: heavy use of typeclass derivation over an HList encoding is "particularly prone to slow
compilation times." This is not a paper claim, it is a load-bearing library used in production Scala
services for close to a decade, reporting a two-order-of-magnitude compile-time cost from exactly the
mechanism arvo is proposing to adopt (recursive typeclass induction over a cons-list).

**Scala 3's own maintainers' response was not to fix shapeless's induction, it was to remove the need
for it.** Scala 3 ships native tuple types (`*:`, `EmptyTuple`), match types, and a compiler-synthesised
`Mirror` typeclass that reads a case class's shape directly, and the language blog frames this explicitly
as bringing "generic programming to Scala 3" without shapeless
(https://www.scala-lang.org/2021/02/26/tuples-bring-generic-programming-to-scala-3.html). The mechanism
that replaced typeclass-driven HList recursion is match types: pattern matching performed by the
compiler's own type checker rather than by resolving a chain of implicit instances. `shapeless-3` itself
now defines its `HList` type in terms of Scala 3's native tuples rather than the Scala 2 cons-cell
encoding (https://github.com/typelevel/shapeless-3), and its own issue tracker still surfaces
`StackOverflowError` failures from derivation-heavy code
(https://github.com/scala/scala3/issues/17142), so the underlying tension between "structural recursion
over a heterogeneous sequence" and "the compiler's own resource limits" did not vanish; it moved from
implicit resolution depth to a different resource ceiling.

The lesson that generalises past Scala: **the compile-time cost that hlist-based libraries pay in
practice is not intrinsic to the concept of a heterogeneous list. It is specific to encoding every
operation on that list as recursive trait or typeclass resolution**, one step of instance search per
element. Where a host language later gave the type checker native support for the same shape (Scala's
match types, described below also for row-typed alternatives), the cost dropped by construction rather
than by disciplined use. Rust has no equivalent native construct for this today; `notko-hlist`'s
`Contains`, `ContainsAll`, `Concat` and any general `Fold` (raised for later in D13) will be recursive
trait resolution unless and until the language grows something like match types. This is a real,
unresolved cost question for the extraction, not a solved one, and it is exactly D4's target: rank 4 is
four levels of recursive trait resolution, multiplied across every distinct instantiation the crate
monomorphises.

**Rust's own hlist libraries have hit the mechanical form of the same wall, at a much smaller scale.**
`frunk` (Lloyd, https://github.com/lloydmeta/frunk) is the closest Rust analogue to `notko-hlist` and
predates const generics. Its own author's blog post walks through hitting rustc's trait-solver recursion
limit directly from HList operations: `error[E0275]: overflow evaluating the requirement
frunk_core::hlist::HList: frunk_core::hlist::Plucker<bool, _>`, resolved by doubling
`#![recursion_limit]` from the default of 64 to 128
(https://beachape.com/blog/2017/03/12/gentle-intro-to-type-level-recursion-in-Rust-from-zero-to-frunk-hlist-sculpting/).
This is a first-hand account rather than a rigorous benchmark, and it predates const generics by four
years, but it is the closest thing this survey found to a direct measurement of the exact failure mode
D4 is designed around: rustc's trait solver, walking a cons-list one recursive step at a time, hitting a
hard ceiling that scales with list length. `notko-hlist`'s `Length<N: Cardinal>` fold (D5) is exactly
this shape of recursive instance resolution, one `succ` call per element, and the frunk experience says
this becomes visible in practice at list lengths well under what a shape's rank would ever reach (rustc's
default ceiling is 64; a shape's rank realistically tops out at low single digits), so rank alone is
unlikely to be where this bites. Where it could bite is a fold that walks the union of several long
domain-specific hlists (kolli's `CommandSet`, hilavitkutin's `AccessSet`) at once, which is closer to
what frunk was doing when it hit the limit. No source in this survey measures the Rust-1.9x trait solver
against an hlist of the length this design actually needs; that measurement, per this workspace's own
`bench-and-sketch-discipline.md`, is a bench question, not a literature question, and this survey did not
find anyone else's bench that answers it either.

`hlist2` (https://docs.rs/hlist2) is worth noting as a small, currently-maintained Rust crate that ships
the identical `Cons<H, T>` / `Nil` shape to `notko-hlist`'s proposed cell, but treats the deep-nesting
problem as an ergonomics issue rather than a compile-time one: its `HList!` macro exists specifically so
a consumer never has to spell `Cons<i32, Cons<f64, Cons<bool, Nil>>>` by hand. This maps directly onto
D7's own answer (alias the cell and the leaf per domain) but is worth flagging as a second, independent
confirmation that hand-writing the nested type is considered a real ergonomics defect worth a macro,
across at least two unrelated Rust crates built years apart.

## Shape-typed and dimension-typed array programming

This is the part of the field most directly answering whether the field treats "shape as a
type-level sequence of extents" as settled or as one of several live alternatives, and the answer is the
latter, sharply.

**Naperian functors** are the closest existing formal account of exactly what a shape's `Array<E>`
composition is doing. Jeremy Gibbons, "APLicative Programming with Naperian Functors," ESOP 2017 /
Springer LNCS 10201 (https://www.cs.ox.ac.uk/people/jeremy.gibbons/publications/aplicative.pdf,
https://link.springer.com/chapter/10.1007/978-3-662-54434-1_21), shows that multidimensional array
structure is captured by lax monoidal applicative functors with strength, commutative up to isomorphism
under composition, and that rank-polymorphic lifting (an operation on a scalar automatically lifting to
operate elementwise on an array of any rank) falls out of this structure rather than needing to be
special-cased per rank. `Cons<H, T>::Array<E> = H::Array<T::Array<E>>` (D4) is, read categorically, a
composition of Naperian functors, one per axis, exactly matching Gibbons's account, though nothing in
D4 constructs the `map`/lift operation over that composition yet; per D13 that is future scope
(cascades, functor over containers). The one limitation Gibbons's own line records: his encoding's
`map` always operates over the full shape, and later work (search summarised, not independently
retrieved in this pass) extends it so only some axes are mapped, which is directly the shape of
"reduce along one axis" that D13 assigns to `arvo-shape` and `arvo-tensor`.

**Accelerate**, a working, shipped Haskell array library
(https://hackage.haskell.org/package/accelerate), is the closest prior art in production use to D4's
actual construction, and it is a genuinely direct hit: its shape type is `Z :. Int :. Int :. ...`, a
heterogeneous snoc-list (built right-to-left rather than D4's left-to-right cons) where `Z` is the rank-0
leaf and `:.` adds one axis, with the stated convention that the rightmost index is the fastest-varying
one adjacent in memory. This is the same construction as D4 under a different growth direction: rank as
list length, per-axis extent as the element type, a distinguished leaf for rank 0. Accelerate has shipped
this design in a runtime array-fusion compiler for over a decade; it is evidence the construction survives
real production use, not merely a toy example. What this survey did not find is any Accelerate-side
report of GHC-side compile-time cost from the snoc-list shape encoding itself (Haskell's type-level list
recursion has a different cost profile from Rust's trait solver, and GHC's own defaulting and
type-family-reduction limits are a different mechanism from `recursion_limit`), so the closest working
precedent for the exact shape does not carry a transferable compile-time data point.

**Dex** (Google Research, https://github.com/google-research/dex-lang; Maclaurin, Radul, Paszke et al.,
"Dex: Array Programming with Typed Indices," OpenReview, ICLR 2021 workshop track,
https://openreview.net/forum?id=rJxd7vsWPS) takes a different route entirely: rather than typing the
shape as a sequence, it types the **index space itself**, so an array of shape `n` is a function from a
finite index type of cardinality `n` to elements, and rank-polymorphic reduction is ordinary function
composition over index types rather than recursion over a shape list. This sidesteps the cons-list
question altogether by not representing "a sequence of axes" as a first-class type at all; instead each
axis is a separate index-typed dimension and multi-dimensional arrays are curried functions of several
index arguments. It is a substantively different design point from D4, worth naming precisely because it
shows the cons-list-of-capacities approach is not the only way to get static shape safety; Dex gets there
by typing what an index *is* rather than typing how many axes there are.

**Futhark**'s size types (Troels Henriksen and Martin Elsman, "Towards Size-Dependent Types for Array
Programming," ARRAY 2021, ACM, https://doi.org/10.1145/3460944.3464310,
https://futhark-lang.org/publications/array21.pdf) are the field's clearest real-world demonstration of
exactly the failure mode D4 cites as the reason to avoid a stride-and-flat-length scheme. Futhark ships,
uses production sizes as ordinary integer-valued type indices, and deliberately keeps size equality
**purely syntactic** rather than doing arithmetic reasoning over it, because full arithmetic-aware size
checking is undecidable in general. The team's own retrospective blog post,
"Static and Dynamic Challenges of Size Types" (2023,
https://futhark-lang.org/blog/2023-05-12-size-type-challenges.html), documents the concrete cost of that
choice in production: because the checker "cannot do arithmetic, and does not understand that addition
commutes," an expression like `zip (concat A B) (concat A C)` is rejected as ill-typed even though both
occurrences of `concat` produce arrays of the identical size `n+m`, because each occurrence is given a
fresh existential size variable rather than being recognised as equal to the other. Futhark's team also
records having to abandon strict type erasure (binding actual size parameters at runtime) to make
`map` over empty polymorphic arrays behave correctly, a compromise directly traceable to sizes being
computed rather than being carried as an associated const the way D4's capacities are. This is a
published, maintained, real-world language reporting the exact class of bug that D4's "rank and element
count are associated consts, so the arithmetic stays in value position" design decision is built to
avoid, from the opposite direction: Futhark chose to let size be a value-level expression precisely
because the alternative (full dependent types) was judged too costly to adopt, and paid for that choice
with exactly the commutativity and aliasing failures D4's design sidesteps by keeping rank and per-axis
extent as fixed, per-type associated consts rather than arbitrary size expressions.

**A very recent (2026, unpublished until ECOOP) paper takes a third route that neither D4 nor Futhark
nor Dex use**: Takashi Suwa and Atsushi Igarashi, "Compile-Time Tensor Shape Checking via Staged
Shape-Dependent Types," Kyoto University / Imiron Co. (PDF retrieved directly, ECOOP 2026 submission).
Rather than proving shape equalities (full dependent types, the Idris-style `Mat p q -> Mat q r -> Mat p
r` the paper cites as its baseline) or restricting size expressions to a syntactic-equality fragment
(Futhark's route), the paper splits tensor computation into two explicit stages via multi-stage
programming in the MetaOCaml/staged-computation tradition: stage 0 verifies shape consistency by
**assertion checking evaluated as ordinary compile-time computation**, generating a specialised stage-1
program that is proven, by construction, never to hit a shape mismatch at runtime. The paper is explicit
that this is a deliberate alternative to proof-carrying dependent types precisely because proof
obligations are "unhandy for continuous software development" and automated proving carries
"unforeseeable time consumption." This is squarely in the "known to work but not yet productised" band
the brief asked for: a prototype implementation exists (verified against a set of `ocaml-torch` examples),
it is not shipped as a language people use, and it is the only source in this survey that treats
compile-time shape checking as a staging problem rather than a type-system problem. It has no direct
analogue in D4's design (D4 stays inside ordinary Rust generics with no staged compilation phase), but it
names the same target Futhark and Dex name from a different angle: get static shape safety without
paying dependent-type proof cost, and it is evidence that this is still an open research problem in 2026,
not one the field has closed.

**A second, independently very recent paper (2025) rejects the arithmetic-vs-shape framing
altogether**: Jakub Bachurski, Alan Mycroft and Dominic Orchard, "Structuring Arrays with Algebraic
Shapes," ARRAY 2025 (11th ACM SIGPLAN International Workshop on Libraries, Languages and Compilers for
Array Programming), https://doi.org/10.1145/3736112.3736141. Their language, Star, represents an array's
indices and shape using **structural record and variant types with subtyping**, so shape mismatches are
caught by structural subtyping rather than by resolving arithmetic constraints on sizes at all, sitting
between "nearly untyped" (element type and rank only, the industrial NumPy/PyTorch norm) and full
dependent types. The tradeoff, stated plainly by the authors and confirmed in public discussion of the
paper (https://lobste.rs/s/iwoipn/structuring_arrays_with_algebraic): the calculus requires "pointful,"
explicitly-indexed operations (`Φ p [|a| ⊓ |b|] . a[p] + b[p]` rather than `a + b`) and the authors state
outright that the technique "is not directly applicable to popular point-free array programming
libraries... such as NumPy or PyTorch." This is the sharpest available demonstration that a shape-safety
mechanism can be sound and still fail the ergonomics bar a working numeric library needs, and it is
directly relevant to D4 because it shows a third distinct family (record/variant subtyping) alongside
"shape as recursive type-level list" (D4, Accelerate) and "shape as size-typed index space" (Dex): the
field has at least three live, mutually incompatible answers to "how do I type an array's shape," none
of which has displaced the others as of 2025 to 2026.

**Dependently-typed languages** (Idris 2, Agda) give the field's oldest working answer, `Vec n a` /
`Mat m n a`, and the field's own literature is candid that the ergonomic cost is proof burden rather than
compile time per se: Suwa and Igarashi's own background section (above) cites this directly, that
requiring "nearly complete amendment of proofs" on every change is what makes fully dependent shape
typing unwieldy for continuous development. Idris 2's architectural rewrite (Edwin Brady,
"Why is Idris 2 so much faster than Idris 1?", https://www.type-driven.org.uk/edwinb/why-is-idris-2-so-much-faster-than-idris-1.html)
is itself evidence that dependently-typed elaboration speed was a first-order concern worth a from-scratch
compiler rewrite based on Quantitative Type Theory, though the material this survey retrieved describes
the elaborator architecture rather than measured compile times specifically attributable to `Vec`/`Mat`
code at realistic array sizes. This survey did not find a rigorous, current (post-2020) benchmark of
Idris or Agda compile time specifically as a function of tensor rank or dimension, only informal
walkthroughs of small matrix examples (e.g. https://timmyjose.github.io/docs/2020-09-01-matrix-operations-in-idris.html).
That is a negative result worth recording plainly: the dependently-typed-array literature argues from
proof-burden ergonomics, not from measured compile-time curves, and nobody in this survey has published
"compile time versus rank" for a dependently-typed shape system the way frunk's author informally did for
Rust's trait solver.

## Rank polymorphism, formally

APL and J's implicit elementwise lifting (a scalar operation applied to a rank-2 array applies elementwise
without the programmer writing a loop or a rank annotation) is given its first rigorous formal treatment
in Justin Slepak, Olin Shivers and Panagiotis Manolios, "The Semantics of Rank Polymorphism," arXiv
1907.00509, 2019 (https://arxiv.org/abs/1907.00509), and its companion, Slepak's dissertation, "A Typed
Programming Language: The Semantics of Rank Polymorphism," Northeastern University
(https://ccs.neu.edu/~jrslepak/Dissertation.pdf). The core language, Remora, formalises the mechanism
that lifts a function operating on rank-`r` arrays to operate on any rank `r' > r`, and proves progress
and preservation for a dependent type system with existential shape/rank abstraction, establishing that
"array shape errors cannot occur at run time in a well-typed program." This is the field's answer to the
specific gap D4 names as missing today: "a function generic over rank is precisely what the design says
is missing today," and Remora's lifting rule is the formal object that gap is asking for. Remora's own
route to it is existential quantification over shape inside a dependently-typed core calculus, which is
closer to Dex's and the Idris/Agda line than to D4's cons-list-of-capacities; nothing in this survey found
a treatment of rank polymorphism formalised specifically over an hlist-of-capacities encoding, which
means D4, if it wants a rank-generic function (not merely a rank-generic *type*), is in territory Remora's
authors formalised under a different representation and did not directly validate under this one.

## Rust specifically: what is real on the current nightly versus folklore

`generic_const_exprs` (GCE) is the feature that would let a const expression, rather than an associated
const, reach type position. arvo's design does not lean on it: it is forbidden by standing gate, and the
last live use in the crate tree (arvo-strategy's Pattern C container projection) has been reproduced
under zero feature gates by `mock/research/sketches/202607282100_container-projection-without-gce/`,
whose findings record the projection "compiles clean with no `#![feature(...)]` gate of any kind, on the
pinned toolchain." Two gates remain textually present in source today (`arvo/src/lib.rs` and
`arvo-strategy/src/lib.rs`, both confirmed directly by this pass), each restricted by its own inline
comment to const-expression bounds and const-fn `where` clauses rather than to the container-projection
mechanism the sketch has now shown does not need it; `arvo-graph/src/lib.rs:17` and
`arvo-spectral/tests/capacity_threading.rs:12` both record, in source, that the capacity system already
carries the same escape for the algorithm crates. So what follows is not a report on a risk to a
mechanism arvo depends on. It is what the field's own record says about the feature arvo has moved away
from, offered because the corrected baseline changes what that record means, not because it changes what
the record says.

Per the Rust project's own 2026 project goals page, GCE is considered to have a "fundamentally flawed"
design that introduces "significant complexity to the compiler," and the project's own retrospective goal
explicitly plans to "communicate why that design did not work out"
(https://rust-lang.github.io/rust-project-goals/2024h2/min_generic_const_arguments.html,
https://rust-lang.github.io/rust-project-goals/2026/const-generics.html). The replacement effort,
`min_generic_const_args` (tracking issue #132980, https://github.com/rust-lang/rust/issues/132980), is a
ground-up rewrite scoped specifically to let a generic parameter appear inside a const generic argument
(the `Foo<{ T::ASSOC }>` shape), deliberately narrower than GCE, chosen precisely because the narrower
scope has a real path to soundness and stabilisation where the broad one does not. Read against the
corrected baseline, this is not a warning about a wall arvo might hit. It is corroboration, from a
process entirely independent of arvo's own reasoning, that arvo's own standing gate ("not proven unsound
or unstable, and, absent a very strong reason, itself on the stabilisation path") reached the same
verdict on GCE that the language's own maintainers reached, for the same underlying reason: a feature
whose own documented successor exists specifically to replace it is not on the stabilisation path it
would need to be on. As of the most recent nightly this survey could check reports against
(early-to-mid 2026), GCE is additionally showing fresh regressions unrelated to the retrospective (issue
#153393, unification failures between a const generic and its associated-const value), which is further
corroboration in the same direction rather than a new finding.

The sketch's own record is worth citing directly here, because it independently reaches the field's
conclusion from the opposite side, by trying to build the thing rather than by reading about it. Its
findings state: "Three builds against the real crate established the ladder": the gate removed outright
fails with "generic parameters may not be used in const operations" at sixteen sites; `min_generic_const_args`
fails with "complex const arguments must be placed inside of a const block"; wrapping in a `const { ... }`
block escalates to needing `generic_const_args`, GCE's own full, un-narrowed successor. So the inline
const-expression form the crate's remaining gates defend cannot be expressed under the narrower
replacement either. What settles the question is not that the narrower successor covers what GCE covered
(it does not, for this exact call shape); it is that the crate never needed the call shape to be in type
position at all. The sketch's finding names this precisely: "the projection's GCE dependency comes from
computing the tag and the byte count with const functions in const-generic argument position," and
"carrying the selection as typestate, the way `Capacity` carries `Array`, removes the expression from
type position entirely, so no const-generic feature is needed." This is the same move D4 makes for shape:
an associated type standing in for a computed value, so the compiler is never asked to evaluate an
expression where a type is expected.

`adt_const_params` (letting a struct or enum, rather than only an integer, `char` or `bool`, be a const
generic parameter, load-bearing for D4's `Cons<H, T>` cell if a capacity or a rotor-basis marker is ever
used directly as a const parameter rather than only as a type parameter) remains, per the Rust Unstable
Book and the 2026 project goals page, incomplete: only ADTs satisfying `ConstParamTy` (structural equality,
no private fields, no interior mutability) are currently accepted, and the project's own stated next step
is to carve out a narrower `min_adt_const_params` that excludes structs with private fields via an RFC,
precisely mirroring the GCE-to-`min_generic_const_args` narrowing pattern. This is consistent with this
workspace's own `unstable-features.md` classification of `adt_const_params` as "largely complete" and on
a real stabilisation path, and this survey found nothing that contradicts that. Unlike GCE, nothing in
the corrected baseline changes this: `adt_const_params` is genuinely in use across arvo today and remains
so.

**Type-level counts parameterised by their own carrier type, the specific shape D5 and D9 need, already
has a direct, shipped Rust precedent that predates const generics entirely.** `generic-array`
(https://github.com/fizyk20/generic-array, https://crates.io/crates/generic-array) defines
`GenericArray<T, N: ArrayLength>` where `ArrayLength` is implemented for `typenum`'s unsigned type-level
integers rather than being hardcoded to `usize`, which is structurally the same move as D5's `Length<N:
Cardinal>`: the count is a trait bound over an abstract carrier, not a fixed concrete type. `typenum`
itself (https://docs.rs/typenum) made the choice to encode its type-level naturals in **binary**
(`UInt<U, B>` built from bits `B0`/`B1`), not in unary Peano form (`Zero`/`Succ<N>`), specifically because
unary encoding produces one type-level recursion step per unit of magnitude, which is catastrophic for
compile time at any nontrivial count; binary produces one step per bit. This is the one piece of directly
transferable, decades-settled folklore this survey found: **`Cardinal`'s `succ` operation (D5), if it is
ever asked to count anything larger than a handful (rank, certainly; but if `Cardinal` is later reused
for something that counts elements rather than axes, less certainly), inherits typenum's exact lesson**,
that unary successor-based counting is a real, well-documented compile-time hazard at scale, independent
of any Rust-version-specific trait-solver quirk. `generic-array`'s own migration path, adding a
`Const<N>` wrapper and an `IntoArrayLength` bridge trait so const-generic callers and `typenum`-based
callers can both satisfy `ArrayLength`, is itself a working precedent for D5's stated intent that the
count type is a genuine, swappable choice rather than one fixed carrier.

## The shape that actually ships: what a per-width associated-type impl table costs

The sketch that settled the GCE question did not settle a compile-time question; it changed which one is
live. `WidthFor<F: Family>` maps a width, carried as typestate (`Wid<N>`, the direct analogue of `Dim<N>`),
to a bucket, carried as an associated type, one impl per concrete width per strategy family. The sketch's
own findings name the shape and its scope directly: "Per-width impls, one row per supported width per
family. The sketch covers the boundaries plus representative interior widths; the real crate expands its
full range by macro," and states plainly that this cost "is already licensed by a ratified rule,"
quoting `arvo-compile-time-last.md`'s own statement that the substrate is allowed to "spend trait-solver
work on per-N const-trait impls (4 strategies x 64+ widths x 2 sign = hundreds of impls) when the
alternative is a runtime container check." So the multiplication the corrected brief names, the width
range times two tag families times four strategies times two signs, is not a new question this pass is
raising; it is an already-ratified cost whose actual size, in trait-solver terms, this survey can now
try to locate in the literature.

**This is a different trait-solver workload from the recursive hlist question above, and the difference
matters for which prior art applies.** The shapeless and frunk material is about **recursive structural
induction**: resolving `Contains<T>` or `Plucker<T, _>` over a cons-list requires the solver to walk one
cell at a time, so cost scales with how deep the chain of obligations goes before it bottoms out, and
that is exactly the axis that hit rustc's `recursion_limit` in frunk and shapeless's implicit-search
budget in Scala. The per-width table is not that shape. `W: WidthFor<HotCold>` is a direct lookup: for a
concrete `Wid<13>`, there is exactly one matching impl, found by ordinary impl selection against a
concrete self type, with no chain of intermediate obligations to walk through first. The obligation count
is proportional to how many impls exist in total (breadth), not to how many steps a single resolution has
to take to reach an answer (depth). Whether rustc's trait solver scales differently across those two axes
is, in principle, exactly the kind of question the frunk and shapeless findings should bear on, because
both are reports of what became expensive in a solver doing typeclass-shaped work; read against the per-
width table, they say the expensive part in both reported cases was the chain, not the count of available
instances, which is some evidence, though not direct evidence, that a flat, non-recursive table of the
size D2's crate split will produce is not the same hazard.

**This survey was not able to confirm that reading against a source that measures it.** The
`rustc-dev-guide`'s own chapter on trait resolution (https://rustc-dev-guide.rust-lang.org/traits/resolution.html)
describes candidate assembly ("searches for impls/where-clauses/etc that might possibly be used to
satisfy the obligation"), matching ("unifying the impl header... while ignoring nested obligations"), and
winnowing (narrowing ambiguous candidates), but carries its own unresolved `**TODO**: Talk about _why_ we
have different candidates, and why it needs to happen in a probe`, and states nothing about how candidate
assembly cost scales with the number of non-overlapping impls a trait carries, nor any fast-rejection or
indexing mechanism keyed on the self type's outermost constructor. This survey found no benchmark, blog
post, or paper, in the time available and against the search budget this pass had left, that measures
rustc's compile time as a function of a large, flat, macro-generated set of concrete trait impls of the
kind `num-traits`-shaped or `generic-array`-shaped crates ship, as distinct from the recursive-induction
cost the hlist lineage already documents well. `generic-array` itself, cited above, does not answer this
either: `typenum`'s `UInt<U, B>` composition is a binary tree of cons cells, which is closer in shape to
the recursive hlist question than to a flat per-width table, so it is not a working example of the
breadth question either.

**The honest state of this question, after this pass, is: the literature that exists supports treating a
flat per-width impl table as a materially different, and plausibly cheaper, trait-solver workload than
the recursive hlist question the rest of this pass covers in depth, but nothing found here measures that
difference directly, for Rust, at the table sizes D2's crate split will actually generate.** That is
itself the answer to what the corrected brief asks for: not a citation that settles the cost, because none
exists, but a precise statement of which comparison the existing citations do and do not license, so a
bench (per `bench-and-sketch-discipline.md`, the same resolution named throughout this pass for compile-
time questions the literature leaves open) is aimed at the right shape rather than at re-measuring the
hlist-recursion question the field has already measured well.

## Compile-time cost of deep type-level lists: measured, or folklore

The brief asks directly whether anyone has measured this. The honest answer, after this pass, is
**partially, and not for the case D4 actually needs measured**.

What exists: frunk's author hitting rustc's default 64-level `recursion_limit` from HList `Plucker`
resolution and needing to double it (cited above), which is a real, dated (2017, pre-const-generics)
data point that the failure mode is real in Rust's trait solver specifically, not merely in Scala's
implicit resolution. What does not exist, as far as this survey could find: any benchmark, in Rust, C++,
Scala, or Haskell, that varies hlist/shape length systematically and reports compile time as a curve,
under the specific combination of features D4 proposes (`Cons<H, T>` with associated-const `RANK` and
`Array<E>`, under `adt_const_params` plus `const_trait_impl` on the current pinned nightly). Boost.Hana
(Louis Dionne, CppCon 2014, "Metaprogramming with Boost.Hana: Unifying Boost.Fusion and Boost.MPL,"
https://www.youtube.com/watch?v=L2SktfaJPuU) is the closest thing to a rigorous benchmark suite for this
exact question in any language: Hana replaced Boost.Fusion's and Boost.MPL's older cons-list-style
heterogeneous sequence encodings and is documented, in its own manual, as achieving "faster compilation
times and runtime performance on par or better than previous metaprogramming libraries." This survey was
not able to retrieve the specific mechanism credited for the win (whether it is variadic-template-based
rather than recursive-cons-based, or whether the benchmark suite in Hana's own `benchmark/` directory
still relies on nested template instantiation for some operations) from the sources it could reach, so
this citation is offered with that gap named rather than the mechanism overclaimed. C++ template
instantiation and Rust's trait solver are, in any case, different compilation models (C++ instantiates
per unique type combination into the AST; Rust's trait solver performs goal-directed proof search with
its own distinct caching and recursion-limit behaviour), so even a fully-specified Hana result would
transfer only as a qualitative signal ("array-based encodings beat cons-list encodings for compile time
in at least one heavily-templated language"), not as a quantitative one.

The workspace's own `bench-and-sketch-discipline.md` and `bench-in-bench-harness-never-sketches.md` rules
already name the right resolution for this open question: whether `notko-hlist` at the rank depths
`arvo-shape` actually needs (low single digits) produces measurable compile-time cost on the pinned
nightly is a bench question, answerable directly and cheaply, not a literature question. This survey did
not find that anyone else has run that specific bench, for this specific feature combination, at this
specific depth. That is the sharpest negative result in this pass, and the per-width table question
directly above is a second, distinct instance of the same shape of gap.

## Type-level folds, membership, and satisfaction: what `Contains`, `ContainsAll`, `Concat` already are

D5 lists `Contains`, `ContainsAll`, `Concat`, and a general `Fold` (raised in D13) as the hlist's payload
beyond rank. All four already have working names and known implementations in the field, which the
literature above already supplies rather than needing separate citation: Kiselyov, Lämmel and Schupke's
original 2004 paper defines membership predicates and concatenation directly on `HCons`/`HNil` as the
paper's worked examples; `frunk` ships `Plucker` (extract one element by type, which is `Contains`
witnessed constructively rather than as a boolean) and `Sculptor` (reshape by extracting several, which
is `ContainsAll` witnessed constructively) as its central operations, and its own README states these are
"checked at compile-time to make sure the type you ask for can be extracted"; `shapeless`'s `Selector` and
`shapeless-3`'s tuple-based reimplementation are the same operation under Scala's typeclass and match-type
mechanisms respectively. None of this is new territory for the field; what is genuinely open, and what
this survey found no direct prior art for at all, is a **general `Fold`** raised in D13 that would let a
cascade reduce across ranks using an arbitrary `Monoid`-shaped combine. Every fold-like operation found in
this survey (`Length`, membership, `Concat`, Accelerate's shape folds) is a fixed, specific fold hardcoded
to its own semantics; nothing in the hlist lineage surveyed here ships a `Foldable`-style abstraction
generic over the combining operation the way Haskell's `Foldable` typeclass does for ordinary lists. If
`arvo-shape`/`notko-hlist` wants that generality, it is closer to unexplored territory within this
specific lineage than to a solved problem, notwithstanding that Haskell's own `Foldable` (outside the
hlist lineage specifically) is of course exactly this abstraction for homogeneous lists.

## What this pass could not find

No source found a rank-polymorphic function (not merely a rank-generic *type*) formalised specifically
over a cons-list-of-capacities encoding; Remora's formal rank polymorphism is over an existentially
quantified dependent shape, a different representation. No source found a systematic, varying-depth
compile-time benchmark of hlist or shape-list encodings specifically under Rust's current trait solver
and const-generic feature set; the frunk anecdote is the closest and it predates const generics. No
source found a published account of fixed-point arithmetic combined with any of the shape-typing schemes
surveyed here (Naperian functors, Accelerate's shape lists, Dex's typed indices, Futhark's size types,
Star's algebraic shapes, Remora's rank polymorphism); every source in this survey assumes IEEE float or
an unspecified numeric element type, and none discusses what changes, if anything, when the element type
itself carries a strategy tag the way arvo's does. No source was found specifically discussing the
compile-time or ergonomic cost of pairing a cons-list shape encoding with `adt_const_params` (as opposed
to plain type parameters), which is the specific combination D4 and `202607281547`'s note on rotor
component counts (D10) both lean on; the closest is the general `adt_const_params` incompleteness tracked
by the Rust project itself, cited above, which is about the feature in isolation rather than about this
combination. No source in the C++ metaprogramming literature this pass could reach specified the actual
mechanism (variadic template versus cons-recursion) behind Boost.Hana's documented compile-time win over
Boost.Fusion and Boost.MPL, so that citation stands as a directional signal only, not a mechanism-level
one. And, added by the revision, no source measures rustc's trait-solver cost as a function of the number
of non-overlapping impls in a flat, macro-generated table (the shape `arvo-strategy`'s per-width
container projection actually uses, and the shape the corrected brief asks this pass to weigh), as
distinct from the recursive-induction cost the hlist lineage documents well; the `rustc-dev-guide`'s own
resolution chapter has an open documentation gap at exactly this question, and no external source found
in this pass closes it.
