# Sketch findings: dissolving a hand-capped arity family into an hlist

**Date:** 2026-07-28
**Outcome:** **WORKS, two ways.** Zero feature gates. 16 assertions, mutation-checked.
**Unblocks:** the placement decision for the `Pred` family, and the general question of what to do
with any hand-written per-arity set in the stack.

## The problem

`arvo/src/predicate.rs` ships `Pred<A>`, `Pred2<A, B>` and `Pred3<A, B, C>`, three trait aliases over
`Fn(..) -> Bool` at arities one, two and three. Only `Pred2` has callers, twice, both in `arvo-comb`.
`Pred` and `Pred3` are declared, re-exported through the facade, and used nowhere.

That set is the shape D4 already rejected for capacities: "needs impls generated per arity and caps
rank at whatever is written". The round rejected it for shapes and left it standing for predicates in
the same crate.

## What was tried, in order, and why the first three failed

**1. Blanket bridge on the wildcard, one impl.** `impl<L: ArgList, F> Pred<L> for F where F:
Fn(&L::Args) -> Bool`. Fails with `E0119`, conflicting implementations, against the per-arity impls.
And standing alone it is wrong anyway: `F: Fn(&(A, B)) -> Bool` describes a closure taking one tuple,
not two arguments, so no ordinary two-argument closure satisfies it.

A first version of this test appeared to pass and was worthless: `ArgList` had no impls, so the bound
was unsatisfiable and the impl applied to nothing. It compiled because it was vacuous. Recorded
because it is exactly the tautological-test failure the workspace rules name, and it was caught only
by making `ArgList` real.

**2. `min_specialization`.** Same `E0119`. The reason is structural rather than a feature limitation:
specialization orders **nested** impl sets, where one is strictly more specific. `Fn(&(A,B)) -> Bool`
and `Fn(&A, &B) -> Bool` are **disjoint** sets of closures, not nested ones, so there is no general
case for the specific ones to specialize. Specialization cannot merge disjoint sets.

**3. `fn_traits` / `unboxed_closures`.** Not attempted. `Fn<Args>` with `Args: Tuple` is the only way
to be generic over `Fn` arity, and it fails the vetting gate: perma-unstable-ish, not on a
stabilisation path. Ruled out on the same test that forbids `generic_const_exprs`.

## Shape A: dispatch on the carrier, not the wildcard

The `Capacity` move. Stop blanket-impl'ing over `F`, and put the dispatch on a named carrier that
answers for itself. The hlist already knows its own arity, and `Cons<A, Empty>` and
`Cons<A, Cons<B, Empty>>` are structurally distinct types, so impls keyed on them cannot overlap.

```rust
pub trait Apply<F> { type Args; fn apply(f: &F, args: Self::Args) -> Bool; }

pub trait Pred<L> { type Args; fn test(&self, args: Self::Args) -> Bool; }
impl<L: Apply<F>, F> Pred<L> for F {
    type Args = <L as Apply<F>>::Args;
    fn test(&self, args: Self::Args) -> Bool { L::apply(self, args) }
}
```

**One public trait, one blanket impl, no arity in either.** Call sites keep ordinary closures with
ordinary arity: `F: Pred<P2<u8, u16>>` and `|a: &u8, b: &u16| ...`.

**The non-obvious part, recorded because it cost a compile cycle.** `Args` must live on `Pred`. With
`pub trait Pred<L: Apply<Self>>` and `fn test(&self, args: <L as Apply<Self>>::Args)`, every call site
must restate the `Fn` bound to discharge `L: Apply<F>`, and rustc suggests exactly that, which defeats
the whole point. Moving `Args` onto `Pred` makes the user's bound carry no where-clause.

**What it does not fix:** `Apply` still has one impl per arity. The enumeration relocates from three
public traits a consumer must choose between to N impls on a carrier nobody reads, but it is still an
enumeration and still capped at whatever is generated.

**One property worth having anyway:** positional bugs become unrepresentable. Mutating the binary impl
to `f(&a.1, &a.0)` does not compile, because `A` and `B` are distinct type parameters. The
hand-written family had three separate bodies, each able to carry its own typo.

## Shape B: recursive, with zero per-arity impls

`recursive.rs`. The hlist is recursive by construction, so the application recurses over it. That
requires the function side to peel one argument at a time, which means the predicate is curried.

```rust
pub trait Chain<F> { type Args; fn run(f: &F, args: Self::Args) -> Bool; }

impl Chain<Bool> for Empty {                       // base: all args consumed
    type Args = ();
    fn run(f: &Bool, _: ()) -> Bool { Bool(f.0) }
}

impl<H, T, F, G> Chain<F> for Cons<H, T>           // step: consume one, recurse
where F: Fn(&H) -> G, T: Chain<G>,
{
    type Args = (H, <T as Chain<G>>::Args);
    fn run(f: &F, args: Self::Args) -> Bool { <T as Chain<G>>::run(&f(&args.0), args.1) }
}
```

**Two impls total, and neither mentions an arity.** The test suite exercises arity seven and nothing
was added anywhere to support it. The value-level argument list is itself recursive, `(A, (B, (C,
())))`, which is an hlist at the value level.

**The cost is the call shape.** The predicate is curried: `|a: &A| move |b: &B| Bool(..)` rather than
`|a: &A, b: &B| Bool(..)`. That is a real ergonomic change at every call site, and it is the whole
price of the arity genuinely vanishing rather than relocating.

## Verification

16 assertions across both shapes, all passing on the pinned nightly with no feature gates.

Every predicate under test is **asymmetric on purpose**. A symmetric one passes even when arguments
are threaded in the wrong order, which is the bug this construction is most likely to have. Coverage
includes argument order at arities two, three and four, heterogeneous payloads where each position
keeps its own type, a positional-swap case built from two same-typed positions around a distinct one
so a swap is expressible and therefore catchable, capturing closures, plain `fn` items rather than
only closures, two distinct predicates at the same arity and payload, and the consumer-shaped
wrappers matching what `arvo-comb` actually writes.

**Mutation-checked, because passing tests prove nothing until they are shown able to fail.** Breaking
the recursive base case to return `true` unconditionally fails all six recursive assertions. Breaking
Shape A's binary impl to swap arguments does not compile at all, which is a stronger result than a
failing assertion and is recorded above as a property of the shape.

## What this leaves for the round

Whether the `Pred` family is deleted outright, given two callers and a shape both variants show is
avoidable, or replaced by Shape A, or replaced by Shape B at the cost of curried call sites.

Whether either shape belongs in `notko-hlist` rather than in arvo, since neither has numeric content
and `Chain` in particular is a general fold over an hlist applied to a function rather than anything
predicate-specific. The `notko-hlist` design as written names `Length`, `Concat`, `ContainsAll` and a
value-level fold, and covers neither.

Whether the same treatment is owed anywhere else. A scan found `Pred2` and `Pred3` to be the only
hand-written per-arity trait family in arvo, hilavitkutin or kolli, so this is isolated rather than
systemic.

## Shape C: macros over Shape B

`ergonomics.rs`. `pred!(a: u32, b: u32 => a < b)` and `args!(1, 2)`, each defined once with a
`macro_rules!` repetition, which is the variadic mechanism the type system lacks. Works to arity six
with nothing added.

**Rejected on reading.** The call site becomes
`feasible::<argl!(u32, u32), _>(&lt, args!(1u32, 2u32))`: three macros and a turbofish, which is worse
than the `|a, b|` it replaces. Kept in the sketch as the record of a dead end.

## Shape D: the consumer names its own arity

`inferred.rs`. The turbofish in Shape C was the tell. `L` was a parameter nobody could infer, but a
**consumer function knows its own arity**, so it names the list once in its own signature and presents
an ordinary call outward.

```rust
pub fn holds2<A, B, F>(f: &F, a: A, b: B) -> Bool
where Cons<A, Cons<B, Empty>>: Chain<F, Args = (A, (B, ()))>,
{ <Cons<A, Cons<B, Empty>> as Chain<F>>::run(f, (a, (b, ()))) }
```

Callers write `holds2(&lt, 1u32, 2u32)`. No list, no tuple, no turbofish, no macro. The library keeps
its two impls and no arity; the arity appears once, in the signature of the function that has one.

**This is the best ergonomic result found**, and it needs no feature gate.

## Why the list cannot simply be inferred

Recorded because it is the natural next idea and it does not work.

```
impl<A, G, F> Chained for F where F: Fn(&A) -> G, G: Chained
error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
```

A type may implement `Fn` at more than one argument type, so `A` is not determined by `F` alone. The
typestate has to name the argument types somewhere. That is not a flaw in the approach; it is exactly
why `Chain` is keyed on the list, where `A` and `B` are named and therefore constrained.

## Shape E: literal `f(a, b)` on a custom type, and why it is not taken

Tested and it works. Implementing `FnOnce` / `FnMut` / `Fn` for a wrapper makes `p(1u32, 2u32)` legal
call syntax on a non-closure. It requires `#![feature(unboxed_closures, fn_traits)]`.

**Vetted 2026-07-28 and forbidden.** Tracking issue #29625 has been open since 2015 carrying
`S-tracking-design-concerns`, with no FCP and two documented defects (#45510, #42736). It carries no
`I-unsound`, so it fails the second half of the gate rather than the first: it is not on a
stabilisation path. The std-internal carve-out does not rescue it, because that carve-out requires
first checking whether a stable wrapper suffices, and Shape D is that wrapper.

## The standing fact behind all of this

The pinned toolchain documents 247 unstable language features and **not one of them is variadic
generics**. The only variadic support is `c_variadic`, the C ABI's varargs, which does not apply to
Rust generics. Arity genericity can only be built from recursion over a type-level list, from macro
repetition, or from a per-arity impl table. Three adjacent features were checked and none help:
`impl_trait_in_fn_trait_return` (#99697, implementation incomplete, three open design questions, and
unnecessary here since `Chain` binds the chain through a trait rather than by naming the type),
`fn_delegation` (#118212, unrelated despite the name), and `const_closures` (#106003, would matter
only if predicates had to be const-callable).

## Final verification

27 assertions across four shapes, passing on the pinned nightly with no feature gates in the sketch
crate. Mutation-checked: breaking the recursive base case fails all six recursive assertions, and
breaking Shape A's argument order does not compile at all.

## Shape F: literal `f(a, b)`, no gates. This is the answer.

`call_syntax.rs`. Reading `core::ops::function` explains why every earlier attempt hit a wall: `Fn`
carries `#[lang = "fn"]` and **`#[rustc_paren_sugar]`**, so the call syntax is a lang-item attribute
that a user trait cannot attach, and all three `Fn*` traits are `#[fundamental]`. Implementing them
directly needs `unboxed_closures`, vetted forbidden.

**But call position autoderefs, and `Deref` is stable to implement.**

```rust
pub struct Pred<L, F>(F, PhantomData<L>);

impl<L: Describes<F>, F> Pred<L, F> {          // validation
    pub fn new(f: F) -> Self { Pred(f, PhantomData) }
}

impl<L, F> Deref for Pred<L, F> {              // invocation: ONE impl, no arity
    type Target = F;
    fn deref(&self) -> &F { &self.0 }
}
```

Consumers write `lt(&1, &2)`. Ordinary call syntax, on a typestate-carrying wrapper, at any arity,
with no feature gate. Verified at arities one through four, with capturing closures, plain `fn` items,
heterogeneous payloads, and two distinct predicates at one arity staying distinct.

**The split this achieves.** Invocation is one `Deref` impl with no arity in it, because the arity
lives in the closure and never reaches the wrapper. Validation is `Describes<F>`, a marker trait with
no methods and no bodies, whose impls are per-arity. So the arity survives only as a compile-time
table that never dispatches and never runs.

**Without the validation bound the typestate lies, and it compiled.** This was tested:

```rust
let liar = Pred::<L2<u32, u32>, _>::new(|a: &u8| Bool(*a > 0));   // compiled fine
```

A type parameter claiming a shape it does not enforce is worse than no parameter. With
`L: Describes<F>` on `new`, the same line fails:

```
error[E0593]: closure is expected to take 2 arguments, but it takes 1 argument
```

Recorded because the unenforced version looked correct, ran, and would have shipped a lying
typestate.

## Where the arity ends up, across all six shapes

| Shape | Consumer writes | Arity lives in | Gates |
|---|---|---|---|
| Today | `impl Pred2<A, B>` | three public traits | none |
| A carrier dispatch | `impl Pred<P2<A,B>>` | `Apply` impls, which dispatch | none |
| B recursive | curried, nested tuple | nowhere | none |
| C macros | `pred!` and `args!` | macro repetition | none |
| D consumer names it | `holds2(&f, a, b)` | the consumer's own signature | none |
| **F deref wrapper** | **`f(a, b)`** | **marker impls that never run** | **none** |
| E `fn_traits` | `f(a, b)` | `Fn` impls | **forbidden** |

Shape F gets the syntax E wanted without the forbidden feature, and pushes the residual arity further
than any of the others: into empty marker impls that exist only to reject a mismatch at compile time.

## Final verification

35 assertions across six shapes, all passing on the pinned nightly, no feature gates in the sketch
crate. Mutation-checked twice: breaking the recursive base case fails all six recursive assertions,
and breaking Shape A's argument order fails to compile rather than failing an assertion.

## Shape G: recursive validation, zero arity anywhere

`recursive_describes.rs`. Shape F pushed the arity into marker impls that never run. This removes the
table entirely.

```rust
pub trait Describes<F> {}
impl Describes<Bool> for Empty {}                    // base
impl<H, T, F, G> Describes<F> for Cons<H, T>         // step
where F: Fn(H) -> G, T: Describes<G> {}
```

**Two impls. No arity in either.** `Deref` still supplies real call syntax, so a consumer writes
`lt(1)(2)` and `g(1)(2)(3)(4)(5)`. Verified at arities one, two, three and five with nothing added.

**The wall it gets around, stated precisely.** `Fn(&A, &B) -> Bool` names both types in **one** bound,
and generating that bound from a recursive structure needs variadic generics, which this toolchain
does not have. Currying gives the recursion somewhere to go, one argument per step. That is why
Shape F needs a table and Shape G does not, and it is not a cleverness gap between them.

**Enforcement is complete**, verified by compile-fail on all three mismatch kinds:

| Attempted construction | Result |
|---|---|
| `Pred::<G2<u32,u32>,_>::new(\|a: u32\| Bool(a > 0))` (too few) | rejected |
| `Pred::<G2<u32,u32>,_>::new(\|a: u8\| move \|b: u32\| ...)` (wrong type) | rejected |
| `Pred::<G1<u32>,_>::new(\|a: u32\| move \|b: u32\| ...)` (too many) | rejected |

**The price is the call shape**, `f(a)(b)` rather than `f(a, b)`, and by-value arguments, which is
what makes currying read cleanly. Fine for the `Copy` scalars a predicate over arvo primitives sees;
a large payload would prefer Shape F.

## The two banked answers

Both are complete, enforced, and gate-free. The choice between them is one trade and nothing else.

| | Shape F | Shape G |
|---|---|---|
| Call site | `f(a, b)` | `f(a)(b)` |
| Impls | 1 `Deref` + N markers | **2 total** |
| Arity lives in | markers that never run | **nowhere** |
| Arguments | by reference | by value |
| Enforcement | complete | complete |
| Feature gates | none | none |

Shape F buys familiar call syntax with a generated table. Shape G buys a genuinely arity-free
construction with curried call sites. There is no third option that has both, and the reason is the
absence of variadic generics rather than a missing trick.

## Final verification

41 assertions across seven shapes, all passing on the pinned nightly, no feature gates in the sketch
crate. Mutation-checked: breaking the recursive base case fails all six of Shape B's assertions, and
breaking Shape A's argument order fails to compile rather than failing an assertion. Shape G's three
mismatch cases were each verified rejected.

## Shape F, boilerplate removed

The `Describes` table was the only thing left in F that read as hand-written arity trickery. It is
pure repetition, so it collapses to one macro invocation:

```rust
macro_rules! describes {
    () => {};
    ($h:ident $(, $r:ident)*) => {
        impl<$h, $($r,)* F> Describes<F> for hl!($h $(, $r)*)
        where F: Fn(&$h $(, &$r)*) -> Bool {}
        describes!($($r),*);
    };
}

describes!(A1, A2, A3, A4, A5, A6, A7, A8);   // the entire table
```

The recursion peels the head, so one invocation emits arity 8 down to 1. Raising the cap is editing
that line. Verified to arity eight; the sketch's 41 assertions pass unchanged after the swap.

**What Shape F now costs, in full:** a marker trait (1 line), two macros (about 12 lines), one
invocation (1 line), the wrapper struct and its two impls (about 10 lines). Roughly 25 lines for the
entire mechanism at any arity up to the cap.

**Against what it replaces:** three hand-written traits, three blanket impls and three doc blocks, 44
lines, capped at three with no way to extend but writing a fourth by hand.

## Why Shape F is the semantically correct one, which is not an ergonomic argument

Checked at the source. `Pred`, `Pred2` and `Pred3` have no relationship to each other: each has one
supertrait, its own `Fn` arity and its own blanket impl, and nothing in `predicate.rs` references
anything else in `predicate.rs`. The family is three unrelated aliases whose sequence exists only in
the names. Nothing would break if they were called `Foo`, `Bar` and `Baz`.

So the original is naive per-arity matching, not a chain. And the real call site confirms which
semantics the domain has:

```rust
pub fn greedy_group<N: Capacity, M: Capacity, A, T>(
    items: &Array<T, N>,
    feasible: impl Pred2<A, T>,
    ...
```

`feasible(&acc, &item)` is a joint test on an accumulator and an item. It is atomic: there is no
meaningful value after applying only the accumulator. Currying it would express a chain the domain
does not have, and the intermediate closure would be an artifact of the encoding rather than
something a reader could name.

**Shape G is the more elegant construction and the wrong semantics for both existing callers.** It
remains banked for any predicate that genuinely is a chain of successive refinements. Nothing in arvo
currently is one.
