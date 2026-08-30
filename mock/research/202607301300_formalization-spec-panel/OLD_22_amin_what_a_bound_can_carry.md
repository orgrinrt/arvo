# 22: What a bound can carry

**Member:** Nada Amin. Definitional interpreters, staged and collapsed; reflective towers; the Dependent
Object Types calculus. The habit I bring is that a type member projected out through a path (`x.T` in
DOT, `F::N` in Rust) and a value computed at a later stage are the same idea looked at twice, and that a
design which cannot say what a bound is allowed to name has not yet said what the bound is for.

**Position:** ninth member of the algebra dive, file 22. Not a synthesis. The dive continues.

**What I read.** The brief's five op files (`16b`, `16c`, `16d`, `17b`, `13c`) first, as instructed, twice
each. `11_current_shape_draft.md` in full. Then `21_rompf_what_a_fact_is_keyed_on.md` and
`20_wingo_the_build_layer_contract.md` in full, since the brief points at both directly. Then
`16_fallin_laws_as_backend_licences.md` in full, because its closing claim ("the backend is arvo's own
dispatch... it never needs to leave the crate") is the claim my question tests. Then
`14_dolan_which_algebra_is_this.md`'s "reading two" and "reading three" sections closely, since Fallin
cites them and they turn out to be the file that already half-answers my question from a different
direction. `13_mcsherry_where_the_laws_belong.md`, `15_willsey_what_a_law_is_for.md`,
`17_orchard_are_these_all_grades.md`, `18_lamport_say_what_is_claimed.md`, and
`19_ringer_the_witness_and_its_upkeep.md` by their section headings and the passages the later files
cite, per the dive's own established practice at this depth. I listed the panel directory and
`mock/research/` before reading inside either, per the standing instruction; nothing postdates file 21.
On source I read exactly what the brief and the prior files license: `arvo-spectral/src/power.rs` in
full (the file Wingo's finding is about), the `pub fn` / `pub struct` signatures across `arvo-graph`,
`arvo-spectral`, `arvo-comb`, and the `arvo--lint-forbidden-*` rule files that state each algorithm
crate's allowed dependency edges. This is a re-check of a claim Wingo already made with a citation, done
because my whole file rests on that claim being exactly right, not a fresh audit.

**What I compiled, as distinct from what I reasoned about.** Five probe files at `22_probes/`, all
`rustc +nightly-2026-05-28`, no `#![feature(..)]` gate opened anywhere in any of them. Every claim in
sections 2 through 4 about what compiles, what fails, and which diagnostic fires cites one of these and
was run, not guessed. Section 5's orphan-rule claim was run twice, and the second run refuted my own
first prediction; I report the refutation rather than the prediction, per this dive's own practice of
keeping a wrong first draft as the audit trail when it was checked and the check disagreed.

**The test gate, before the assigned work.** `cargo test --workspace` in `arvo/mock`: green, matching
Rompf's and Wingo's prior reports exactly, no drift since either. The surface this file is about,
whether a fact reaches a trait bound, has no tests, because the mechanism does not exist in shipped
source. Nothing to audit, nothing to refuse.

---

## 0. The claim under test, and where it actually sits in the dive

Wingo's file, written two positions before mine, already found the concrete instance of what my brief
asks about, in passing, inside a section about something else:

> `Recip` is a trait in `arvo-numeric-contracts/src/lib.rs:44`, and `power_iteration` demands it in its
> own where-clause (`arvo-spectral/src/power.rs:47`). The liberty is not in a `Number`'s body; it is
> **in the bound of an algorithm generic over the number**, which has no `S` to read and no way to be
> told. (`20_wingo...md` section 0)

Wingo names this "the largest hole I found in the fidelity axis as anyone has drafted it" and proposes,
without building, that "liberties become bounds." My brief generalises the question past fidelity to
every law this dive has derived (associativity, distributivity, monotonicity, the whole apparatus
Rompf's file just finished keying correctly), and asks what a bound can actually carry, precisely, and
what the fix costs. That is the gap between Wingo's finding and mine: he found the wound and named a
plausible bandage. I build the bandage, find it needs two more shapes than the one he named, and check
where each one is actually the right tool.

I want to be exact about one thing before anything else, because it is easy to conflate and Rompf's
closing section already warns about exactly this conflation from a different angle. **This is not the
LLVM boundary Fallin and Wingo already mapped.** Fallin found that a derived fact never needs to leave
arvo because the consumer is arvo's own dispatch, checked and executed as one text
(`16_fallin...md` section 5). Wingo found that monomorphisation prints the composition into the symbol
table rather than erasing it, and that a post-hoc build-layer reader can recover it
(`20_wingo...md` section 5). Both are right, and neither touches my question, because both are about
what survives *after* a concrete `Number<N, S>` has already been chosen and compiled. My question is
about what a function can see *before* that choice is made at all: a generic algorithm crate, at the
point its own source is being type-checked, has not yet been handed a concrete composition. It has been
handed a name, `F`, and a promise about what `F` can do. Section 1 makes this a second, separate,
source-level boundary, and names it, because none of the four files before mine give it a name.

## 1. The premise, verified, and a second boundary named

The premise first, because this dive's own record is that a brief's generalisation can be broader than
what was actually checked, and mine claims every algorithm crate, not one function.

```
arvo-graph/src/components.rs:23   pub fn components<C: Capacity, B>(...)
arvo-graph/src/rank.rs:34         pub fn upward_rank<C: Capacity, W, B>(...)
arvo-graph/src/topo.rs:36         pub fn topo_sort<C: Capacity, B>(...)
arvo-spectral/src/fiedler.rs:54   pub fn fiedler_vector<Op, C: Capacity, F>(...)
arvo-spectral/src/power.rs:38     pub fn power_iteration<Op, C: Capacity, F>(...)
arvo-comb/src/dp.rs:36            pub fn matrix_chain_dp<N: Capacity, W>(...)
arvo-comb/src/binpack.rs:37       pub fn bin_pack<N: Capacity, B: Capacity, T, W>(...)
```

Every public entry point in every algorithm crate is generic over a bare type parameter constrained by
arithmetic-operation traits (`Add`, `Mul`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`) and never over
`Number<N, S>`, `UFixed`, or `IFixed` by name. This is not an accident of the current code that the
redesign will fix; it is a standing architecture rule, stated independently of this dive, and it forbids
the alternative directly: "Algorithm crates are generic over numeric trait bounds. They do not import
`UFixed` or `IFixed` directly" (arvo's own agent instructions, `arvo/.claude/CLAUDE.md`, "Architecture
rules"). The forbidden-imports lints go further and forbid these crates from even depending on the
facade at all: `arvo-graph` may not import `arvo::*`, full stop
(`.claude/rules/arvo--lint-forbidden-arvo-graph.md`). So Wingo's finding is not a bug in one file. It is
the intended shape of every consumer arvo has among its own siblings, load-bearing, on purpose, and (per
`16b`) still the shape after the rewrite, since nothing about this dive's redesign proposes changing
which crates may see the facade.

That gives me the second boundary, and I want to state it in the same register Rompf used for his three,
because it is the one his own picture does not have room for.

Rompf's file names three stages inside arvo (`21_rompf...md` section 1: type-written, operation-applied,
fold-run) and, separately, the reading he holds against his own (his section 10), that monomorphisation
is not a stage boundary in the technical sense because it prints rather than erasing. Both of those are
about *what arvo's own compiler sees*. Neither is about what a *second crate's compiler* sees while
compiling that crate's own generic body, before arvo's concrete types have ever been substituted in.
That is a fourth stage, and unlike Rompf's three it is not a stage of arvo's own design at all; it is a
stage the *language's ordinary parametric polymorphism* imposes the moment a function is written generic
over a bound rather than over a concrete type, and the workspace's own architecture rule chooses, on
purpose, to put every algorithm crate on the far side of it. Call it **Stage G**, for generic: the point
at which a numeral becomes an abstract name satisfying only the capabilities its bound spells out, and
nothing else about it exists as far as the body that received it is concerned.

Two things distinguish Stage G from every boundary the dive has named so far, and both matter for what
follows.

**It is a real erasure, in the sense LLVM's is and Rompf's three are not.** Rompf's stages T, O, F all
happen with the same concrete `(N, S)` in view throughout; nothing is forgotten between them, only fixed
in sequence. Wingo's channel survives *because* monomorphisation eventually re-supplies the concrete
type. Stage G is different: at the point `power_iteration`'s own body is type-checked, `F`'s concrete
identity is gone in the sense that matters, not merely deferred. Probe 1 (`22_probes/01`) makes this a
compiled fact rather than an analogy: a generic function bound only on `Add2 + Copy` compiles and runs
with no name `S` anywhere in its scope, and asking for that name by writing `let _proof: S = todo!();`
inside the body fails with `E0425: cannot find type S in this scope`, the compiler's own wording for "no
such name has ever been bound here," not "this name exists but you may not use it."

**It is not imposed by the language or the toolchain. It is imposed by this workspace, on purpose,
elsewhere, for a different and entirely sound reason**, and that is the load-bearing difference from the
LLVM boundary. LLVM's erasure is structural: by the time IR exists, there is no residual tag on an
`fadd` naming which Rust type produced it, on any mainstream compiler, ever
(`16_fallin...md` section 8, `20_wingo...md` section 5). Stage G has no such necessity behind it. It
exists because the workspace decided, independently and for good reasons (algorithm code that is
reusable across every numeral shape a consumer might ever supply, never coupled to arvo's own concrete
vocabulary, exactly the discipline `no-bare-primitives.md` and `use-the-stack-not-reinvent.md` ask for
everywhere else in this workspace), that algorithm crates should be written this way. Which means, unlike
the LLVM boundary, **Stage G is negotiable from arvo's own side of it**, at the cost of whatever the
generic function's bound is willing to name. Sections 3 and 4 are two different prices for that
negotiation, and section 5 is a third that pays no price on the generic side at all.

## 2. What a bound can carry, precisely, and what it cannot

Before pricing anything, the mechanism itself, stated plainly, because the brief asks for precision and
the vocabulary in the rest of the dive (witnesses, licences, grants) makes it easy to talk about a bound
as if it were a permission slip rather than what it actually is in this language.

A trait bound `F: SomeTrait` is a promise, checked once, at every call site where `F` is finally bound
to something concrete, that the concrete type implements `SomeTrait`. Inside the generic body, the bound
is the *entire* API a caller has purchased on `F`'s behalf. Three things follow, and none of them is new
to Rust, but stating them against this design's specific vocabulary is the point.

**A bound can carry a method.** `F: Add<Output = F>` lets the body write `a + b`. This is what every
algorithm crate signature already uses, and it is the baseline the rest of this file is added on top of.

**A bound can carry an associated type, and that type can itself be bounded, and it can be projected
further.** `trait Numeric { type N: Numeral; }` lets a caller of a function bound on `F: Numeric` write
`F::N`, and because `F::N: Numeral` is itself a bound, `F::N` can be passed as a generic argument to any
other function generic over `<T: Numeral>`. This is, syntactically and semantically, Rust's nominal
approximation of a DOT-style abstract type member projected through a path: `F::N` is `F.N` written with
`::` instead of `.`, upper-bounded by `Numeral` the way a DOT member is upper-bounded by a supertype, and
reachable from any context that has `F: Numeric` in scope, exactly the way a path-dependent type is
reachable from any context that has the path's prefix in scope. Section 4 is this move, built and
compiled.

**A bound can carry a marker with no items at all, whose entire content is that it was satisfied.**
`trait AssociativeAt<Op, const ARITY: usize> {}` carries nothing to call and nothing to project; its only
information is binary, present or absent, checked by the compiler rather than read by the body. Section
3 is this move, and it is the shape Dolan's file already builds for a different purpose.

And two things a bound in this language, on this toolchain, under this workspace's own forbidden-feature
table, cannot carry, stated so the "no dependent types, no higher-kinded types" half of the brief has a
direct answer rather than a gesture at the literature.

**No true dependent type.** A DOT calculus, or Coq, or Idris, can let a type depend on an ordinary runtime
value: "the type of a vector whose length is the value `n` computed three lines up." Rust has no such
thing; the substitute, everywhere in this design, is the const generic, which lets a type depend only on
a value known *at compile time* and syntactically restricted to appear in a small set of positions. This
is not a gap this file's mechanisms paper over; it is the reason Rompf's whole apparatus exists in the
shape it does (a derived fact is a `const fn`, checked by scope, never a proposition about a runtime
value), and every mechanism below inherits that same restriction rather than escaping it. A bound cannot
carry "this fact holds for the specific runtime accumulator I am about to build." It can carry "this fact
holds for every accumulator of this const-known arity," which is what section 3's `ARITY` parameter
already is.

**No higher-kinded abstraction, and, checked directly against this specific problem, it does not bite.**
Rust cannot write a function generic over "any type constructor shaped like `Numeral`", the way a
Haskell function can be generic over any `f :: Type -> Type`. I looked for a place in this design where
that absence would matter and did not find one: every one of the ten axes is a closed type parameter
(`Numeral`, `Policy`, `Lowering` and their members are all concrete, non-constructor types once chosen),
`Number<N, S>` itself is not parameterised over "any numeral-shaped constructor" anywhere the design asks
for, and neither `Numeric` (section 4) nor `AssociativeAt` (section 3) needs to abstract over a
constructor to do its job. This is worth stating as its own finding rather than a footnote: **HKT's
absence is a real limitation of this language and this is a place a member should say so honestly where
it bites, per the brief's own instruction, and it does not bite here.** The place it might have bitten,
a hypothetical "any container that carries a numeral" abstraction, is not a shape this design currently
asks anyone to write.

## 3. Move A: enrich the bound

`22_probes/02_move_a_enrich_the_bound.rs`. This is Wingo's own proposal, "liberties become bounds," built
rather than left as a direction, and it turns out to already be half-built by a different member for a
different reason: Dolan's "reading two" (`14_dolan...md:255-296`) proposes exactly this shape for the
algebra ladder, arrived at independently, to solve the coherence ceiling the draft's own Thread C hit
(section 3.4's "three implementations, none more specific, refused as conflicting"). I did not invent
the mechanism; I connect it to the reachability question neither file was asking, and I extend it with
the one dimension Dolan's file predates: the accumulator-and-arity key Rompf's measurement found the
design actually needs.

The shape: a marker trait carrying the operation and, where the fact's key needs it, a caller-supplied
const dimension, blanket-implemented once per composition that satisfies it.

```rust
trait AssociativeAt<Op, const ARITY: usize> {}

impl<N: Numeral, const ARITY: usize> AssociativeAt<Add, ARITY> for Number<N, Wrap> {}
impl<N: Numeral> AssociativeAt<Add, 1> for Number<N, Saturate> {}

fn fold_quad<F: Copy, const ARITY: usize>(x: F, n: F) -> F
where
    F: AssociativeAt<Add, ARITY>,
{ /* the regrouped body */ }
```

Compiled, both arms. `fold_quad::<_, 4>(w, w)` against a `Number<Fixed3, Wrap>` compiles and runs: Wrap's
impl covers every arity. `fold_quad::<_, 4>(s, s)` against a `Number<Fixed3, Saturate>` refuses, and the
diagnostic is exactly the shape Rompf gave up on for his own mechanism:

```
error[E0277]: the trait bound `Number<Fixed3, Saturate>: AssociativeAt<Add, 4>` is not satisfied
   |
   |         let _ = fold_quad::<_, 4>(s, s); // Saturate at arity 4: refused
   |                             ^ unsatisfied trait bound
help: the following other types implement trait `AssociativeAt<Op, ARITY>`
   | `Number<N, Wrap>` implements `AssociativeAt<Add, ARITY>`
   | `Number<N, Saturate>` implements `AssociativeAt<Add, 1>`
```

named at `fold_quad`'s own call site, in the calling crate, naming both the composition that failed and
the compositions that would have succeeded. This is the diagnostic Rompf's own file explicitly gave up:
"there is no propagatable `T: AddAssoc` bound and no `E0277`... this is file 19's own trade... I take it
for the same reason" (`21_rompf...md` section 7). It is not a trade this shape needs to make.

**Why this does not reopen the coherence ceiling.** Dolan already found the reason and I confirm it holds
under the extra dimension: `T: Associative<Op> + Commutative<Op>` being simultaneously true is a
conjunction, not competing evidence, because nothing downstream has to *choose* which impl fired
(`14_dolan...md:298-306`). The `ARITY` parameter does not change this: `Number<N, Wrap>: AssociativeAt<Add,
4>` and `Number<N, Wrap>: AssociativeAt<Add, 7>` are two distinct trait instantiations, not two
implementations competing for the same one, so the coherence checker never has to arbitrate between them.
What section 3.4's ceiling actually forbids is two *impls of the exact same trait instantiation*
disagreeing, which this shape never asks for: each `(Self, Op, ARITY)` triple has exactly one impl, by
construction, because the impl's own where-bound (`for Number<N, Wrap>`, `for Number<N, Saturate>`)
already partitions the space along the axis (`S`) the fact actually varies on.

**Where this shape already has a home in the workspace's own crate layering, unbuilt.** The design's
proposed crate table already puts "the algebra ladder and the law markers" in their own crate,
`arvo-algebra-contracts` (`11_current_shape_draft.md` section 3.7), described in exactly the vocabulary
this workspace already uses four times over for precisely this purpose: a small, trait-only crate an
algorithm crate can depend on without pulling in the type that implements the trait. `arvo-bits-contracts`,
`arvo-mask-contracts`, and `arvo-numeric-contracts` are the existing instances of the pattern; `Recip`,
`Sqrt`, `FromConstant`, and `TotalOrd`, the exact traits `power_iteration` already bounds on
(`power.rs:18`), live in one of them today. `arvo-algebra-contracts` declaring `AssociativeAt<Op, ARITY>`
and its siblings, with the blanket impls living wherever `Number<N, S>` itself lives, is not a new
architectural move; it is the same move the design has already made four times, applied to the
vocabulary this dive derived. What is genuinely undesigned, and belongs to `16c`'s obligation rather than
to a diagram I can draw for someone else: **whether `arvo-graph`, `arvo-spectral`, `arvo-comb`, and
`arvo-sparse` gain a new dependency edge onto `arvo-algebra-contracts`.** Today's forbidden-imports lints
do not name it, because it does not exist yet; the redesign has to say, explicitly, that these crates may
depend on the algebra-contracts crate the same way they already depend on the numeric-contracts crate, or
the marker traits Dolan and I both build have nowhere for an algorithm crate to import them from.

**The limit of this shape, stated honestly.** It costs a fresh marker trait per fact family (though not
per fact instance, since the const-generic key absorbs the instance-level variation), and it works
cleanly exactly when the fact's key is either fixed (`Op`) or something the *caller* already knows locally
(the combinator doing a four-way split knows it is doing arity four without needing to ask `F` anything).
It does not, by itself, give the generic body a way to *read* a value out of `F` that the caller did not
already supply. Section 4 is for that case.

## 4. Move B: make the composition reachable from the bound

`22_probes/03_move_b_project_the_identity.rs`. Rompf's own mechanism (`21_rompf...md` section 3, "a
derived fact is a `const fn` whose parameters are its key") never needs to *leave* arvo's crate in the
case he analyses, because everywhere he calls it, `N` and `S` are already concrete names in the enclosing
scope. The moment the caller is `power_iteration`, they are not, and his mechanism has nothing to call
with. This section supplies the missing connective tissue: one trait, one blanket implementation,
projecting a composition's own identity back out through the bound that erased it.

```rust
trait Numeric {
    type N: Numeral;
    type S: Resolve;
}
impl<N: Numeral, S: Resolve> Numeric for Number<N, S> {
    type N = N;
    type S = S;
}

fn fact_gated<F: Numeric + Copy, const ARITY: usize>(x: F) -> F {
    const {
        assert!(add_assoc_at::<F::N, F::S, ARITY>(), "...");
    }
    x
}
```

Compiled, both arms, against `add_assoc_at`, the same const-fn-is-the-key shape Rompf built, unmodified.
`fact_gated::<_, 4>` on a `Number<Fixed7, Saturate>` (numeral width 7, arity 4) evaluates the fact at
compile time inside the `const` block and returns cleanly. `fact_gated::<_, 4>` on a
`Number<Fixed3, Saturate>` (width 3, arity 4, below the stand-in interior-safety threshold) fails not
with a trait-bound error but with the same shape of diagnostic Rompf's own probes produce, `E0080`,
naming the composition and the failing evaluation:

```
error[E0080]: evaluation panicked: this composition's Add is not associative at this arity; \
  widen the numeral, or pick a resolution that commutes
   |
   | evaluation of `fact_gated::<Number<Fixed3, Saturate>, 4>::{constant#0}` failed here
```

This is stable Rust, exactly as much as section 3 is; no `#![feature(..)]` gate is opened in either
probe. `F::N` used as a generic argument to `add_assoc_at` is an ordinary associated-type projection
passed to an ordinary generic function; it does not touch `generic_const_exprs` at all, because nothing
here is a const expression computed from a generic parameter in *type* position, only a type projected
by name and passed onward, which is a different and entirely stable thing. This matters given how much of
this dive's cost has been toolchain risk (`08_fog...md`'s const-eval wall, `16_fallin...md`'s forbidden
`core_intrinsics`, `20_wingo...md`'s unvetted `core_float_math`); this mechanism carries none of it.

**Why this is a different tool from section 3, not a competing one.** Section 3's marker trait is
satisfied or not, decided once, by the compiler, with no further information available to the body about
*why*. Section 4's projection gives the body a way to *ask a new question it was not pre-authorised for*,
using whatever const-fn machinery already exists, including facts nobody thought to write a marker trait
for. The price is exactly the one Rompf already named and accepted for his own mechanism, inherited here
unchanged: no propagatable bound, no `E0277`, a panic at the use site rather than a refusal at the
signature (`21_rompf...md` section 7). I would reach for section 3 when the fact's key is small, stable,
and worth a friendly diagnostic (which is most of the algebra ladder, per Dolan's own list: associativity,
commutativity, has-identity, distributes-over, monotone). I would reach for section 4 when the algorithm
genuinely needs to make a decision that depends on a value it has no other way to obtain, such as the
numeral's own logical width feeding directly into an interior-safety bound the caller could not have
supplied, which is exactly what my stand-in fact does and exactly the shape Rompf's real accumulator
threshold (section 4 of his file, `K = n - 1` in the arity) will need the day an algorithm crate wants to
size its own accumulator adaptively rather than take the arity as a caller-supplied constant.

**Where this shape lives, and it is a smaller ask than section 3's.** `Numeric` declares only that `type
N: Numeral` and `type S: Resolve`; it names no concrete arvo type at its own declaration site, so it can
live in the same low contracts crate as `Numeral` and `Policy`/`Lowering` themselves, wherever those end
up, with the one blanket impl living beside `Number<N, S>` in the facade. An algorithm crate that bounds
on `Numeric` gains no new dependency beyond what it would already need to name `Numeral` at all.

## 5. Move C: keep the generic core honest, and license at the seam instead

Neither section 3 nor section 4 is the only way to close Stage G, and I want to give the third its own
section rather than a caveat, because it is the one the brief's "design honestly for [the generic case
operating without the facts]" reading actually wants, and because it is Fallin's own answer, correctly
scoped, applied to a place he did not scope it to.

Fallin's mechanism, a derived fact gating which monomorphised body compiles, never leaving the crate
(`16_fallin...md` section 5), is not wrong at the boundary he drew it at; it is scoped to consumers who
already have `N` and `S` concretely in view, which is true of everything inside arvo's own crate and false
of `power_iteration`. The honest reading of Stage G is that the deeply generic body, the one four
algorithm crates already ship, **should stay exactly as fact-blind as it is today**, correct for every
numeral shape a consumer could ever hand it, slower than it needs to be for the shapes that could go
faster, and that this is not a defect to route around inside the generic body at all. The fact-gated
dispatch belongs one layer up, at a new entry point closer to where the composition is still concrete,
which for these crates does not exist yet: today `power_iteration<Op, C, F>` *is* the public API, with no
shallower wrapper above it that a caller reaching for a concrete `Number<N, S>` could land on instead.

Concretely, per crate, this reads: `power_iteration_fast::<N, S>(...)` (bound on `Numeric`, or concretely
on `Number<N, S>`, arvo's choice) checks whichever fact licenses the reciprocal-approximation liberty
Wingo found being taken unconditionally today (`power.rs:74`, `arcp` with nothing gating it), and either
delegates to the existing generic `power_iteration` unmodified, or to a body written with the liberty
taken deliberately. This is section 3 or section 4's mechanism, once, at the seam, rather than threaded
through the generic core at all; it costs the algorithm crate one new public function per fact worth
specialising on, and it costs nothing to the generic function, which stays exactly as it is, still
callable directly by anyone who does not care.

I hold this as the reading I would build first, not because sections 3 and 4 are wrong, but because it
is the only one of the three that requires zero change to the four generic entry points that exist today,
and because it makes the honest thing visible in the API rather than implicit in a where-clause a future
reader has to notice is missing.

## 6. Whether a law belongs to the operation, and whether it changes any of this

Two members reached toward this from different directions and neither built it out. Dolan's atomic facts
are already parameterised by `Op` (`AssociativeAt<Op, ARITY>`, section 3 above), which is a real step
toward "the law is about the operation," but the impl's own Self type in every case is still the
composition, not the operation: `impl ... for Number<N, S>`, with `Add` sitting in the parameter list.
Rompf's file carries the fuller version of the question without resolving it, citing Willsey directly:
laws as properties of a magma (a set-plus-operation pair, the noun the draft already declares and never
uses, `Combine<Op>`/`Magma<Op>`) against laws as edges of a rewrite system with no carrying object at all
(`21_rompf...md` section 8, citing `15_willsey...md:44-57`).

I want to add a reading neither of those two considered, because it is available in this language and
because I checked it rather than argued it: **make the operation itself the Self type of the fact trait,
and the composition the parameter.**

```rust
struct Add;
impl<N: Numeral> Associative<Number<N, Wrap>> for Add {}
```

against Dolan's and section 3's shape, `impl<N> Associative<Add> for Number<N, Wrap>`. I expected this to
matter for who is allowed to extend the vocabulary: under the current shape, only arvo's own crate can
ever write a new `impl ... for Number<N, S>`, because Rust's orphan rule requires either the trait or the
Self type to be local, and neither is local to a consumer crate reaching in from outside. Flipping the
Self type to the operation looked, before I checked, like it would let a downstream crate mint its own
operation and grant it facts about arvo's existing numerals without needing arvo's cooperation at all,
which is exactly the "tools, not policy" posture `arvo-toolbox-not-policer.md` asks for everywhere else in
this substrate.

`22_probes/04` and `22_probes/05`, two real crates, not two modules in one file, so the orphan check is
the real cross-crate one rather than a stand-in. **I predicted an asymmetry and the compiler refused it.**
Both directions compile clean: a downstream crate defining its own `MyOp` may write either
`impl Associative<MyOp> for Number<Fixed3>` (its own type as the trait's parameter, arvo's numeral as
Self) or `impl Associative<Number<Fixed3>> for MyOp` (its own type as Self, arvo's numeral as the trait's
parameter). The orphan rule, correctly, asks only that *some* type in the impl header be local to the
crate writing it; it does not care which position that type occupies. So the extensibility question I
went looking for is not decided by which side of the trait carries the law; it is decided by whether the
*novel* thing (a new operation, in both my tests) is local to the crate doing the extending, which it was
in both directions I tried and would not be if the novel thing were instead a *new numeral* against
arvo's *existing* operations, a case I did not test and flag below.

So my own attempted sharpening did not survive contact with the compiler, and I would rather report that
than the argument I had before I ran it. What I am left with, honestly: the type-versus-operation question
is real and unresolved by anything in this dive, but on the evidence I actually gathered, **it does not
change the answer to my own question.** Whichever side of the trait carries the law, the mechanics a
generic algorithm crate needs to reach it are the same three moves in sections 3, 4, and 5, because Rust's
type system funnels both readings through the identical nominal-bound machinery. The magma-versus-
rewrite-edge question in Rompf's and Willsey's sense may still matter to how the vocabulary reads to a
consumer, per Dolan's own "reading three" (`14_dolan...md:307-320`, keeping named structures as
consumer-facing sugar over an atomic proof mechanism); I do not think it matters to whether a bound can
carry the result.

## 7. Direct answers

**What can a bound carry, precisely?** A method (already used everywhere). An associated type, itself
bounded, projectable onward as a generic argument to further generic code, with no unstable feature and
no dependent-typing risk, because the projection is resolved by name at the point it is used, never by a
runtime value (section 2, section 4). A zero-item marker whose presence or absence is the entire fact,
extensible with caller-supplied const-generic key dimensions, checked at monomorphisation with a clean
diagnostic (section 3). What it cannot carry: a claim indexed by a value only known at runtime (no
dependent types; the const-generic substitute is what every mechanism here actually uses), or an
abstraction over a type constructor rather than a type (no HKT; checked directly against this design and
found not to bite anywhere the ten axes currently ask for).

**Is the fix to enrich the bound, make the composition reachable, attach facts elsewhere, or accept the
gap?** Not one of the four alone. Enrich the bound (section 3) for facts whose key the caller already
holds, which on the evidence of `fold_quad`-shaped consumers is most of the algebra ladder. Make the
composition reachable (section 4) for facts the algorithm can only decide by reading a value out of the
type itself. Attach nothing new to the type and instead confine the licensed dispatch to a new,
shallower entry point above the generic core (section 5), which is the one I would build first because it
costs the existing public API nothing. "Accept the gap and design honestly for it" is section 5 read as a
design stance rather than a resignation: the deep generic body staying fact-blind by design, forever, is
not a compromise, it is what a function generic over every numeral shape a consumer could supply is
supposed to be, and the dishonesty was never the gap, it was `power.rs:74` taking a liberty through it
silently, which Wingo already found and which none of the three moves here leaves silent any longer.

**Is putting everything in the type the error the trait-bound case exposes?** No, and I want to be
precise about what the actual error was, because "move facts out of the type" is not available to move
them to. A fact about whether one specific composition's addition regroups is, irreducibly, a fact about
that composition; there is nowhere else for it to be true. The error was narrower and more specific:
Rompf's staging picture, and Fallin's dispatch mechanism built to serve it, both silently assumed every
consumer of a derived fact would have the concrete composition in view at the point it asks, because
every consumer either of them actually analysed does. The workspace's own architecture, independently and
for good reasons, put a second population of consumers on the far side of a boundary neither analysis
priced, not by oversight so much as by scope: Fallin's brief was the LLVM boundary, Rompf's was the fact's
key, and Stage G belonged to neither. Naming it is what this file adds; sections 3 through 5 are what it
costs to cross.

## 8. What I would flag for the next member, unresolved

**Section 6's orphan-rule check only tried the new-operation direction.** Whether a downstream crate can
grant an *existing* arvo operation (`Add`) a fact about a *new* numeral it defines itself
(`impl<S> Associative<Add> for Number<MyNumeral, S>`, with `Number` and `Add` both foreign and only the
inner type parameter local) is the case that actually matters for whether the ten-axis system is
extensible with new `Numeral` implementations from outside arvo, and I did not test it. My reading, held
weakly and unchecked: the local type sits nested inside a foreign generic constructor rather than at the
impl's own top level, which is usually where the orphan rule stops helping without a `#[fundamental]`
flag arvo does not control, so I would expect this direction to fail where my two tested directions
succeeded. This should be compiled before anyone treats the ten axes as third-party-extensible.

**I did not price the monomorphisation cost of either mechanism at a real consumer's composition set.**
Rompf flagged the identical gap for his own mechanism (`21_rompf...md` section 7) and it applies
unchanged here: section 4's `fact_gated` is a function call whose cost scales with the product of every
distinct `(F::N, F::S, ARITY)` triple an algorithm crate's callers actually instantiate, and nobody has
measured what that product looks like for a real graph or spectral workload. It belongs in
`mock/benches/`, per `bench-and-sketch-discipline.md`, and is not in this file.

**Section 5's entry-point layer is a design, not a build.** I did not write `power_iteration_fast`,
because doing so honestly needs the fidelity axis's own shape settled first (Wingo's own section 9 flags
the same dependency for his half of this question), and I would not want to be the design round the
current draft's own record warns about, where an unbuilt shape looked settled because nobody had
compiled it yet against the axis it actually depends on.

**Whether `arvo-graph`, `arvo-spectral`, `arvo-comb`, and `arvo-sparse` gain a dependency edge onto
`arvo-algebra-contracts` is a decision nobody has made**, and it is the one piece of `16c`'s obligation I
could not discharge alone: it is a crate-layering call for whoever owns the redesign's dependency graph,
not a fact I can derive from the mathematics. Section 3 needs it; section 4 needs the equivalent edge onto
wherever `Numeral`/`Policy` end up; section 5 needs neither, which is one more reason I would build it
first.

**I did not read `arvo-num-systems` or `notko-hlist`**, which files 17, 18, 19, and now Rompf have each
flagged in turn for themselves. Four members naming the same unread pair is a louder signal than any one
of our individual findings, and it is a cheaper dispatch than compiling another probe.
