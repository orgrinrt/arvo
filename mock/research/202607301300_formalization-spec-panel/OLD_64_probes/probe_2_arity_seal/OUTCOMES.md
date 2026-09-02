# Probe 2 outcomes: the `Arity` carrier-at-birth gap, closed and reproduced

Four compiles, two libraries and two downstream attackers, all on the pinned toolchain
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`).

## 1. `lib_unsealed.rs` (the shipped shape, reduced)

```
rustc --edition 2021 --crate-type lib lib_unsealed.rs -o libarity_lib_unsealed.rlib
```

Exit 0. `InteriorSafety<A>`'s trait declaration carries no bound on `A`; the ordinary blanket is
scoped to `A: Pos`; the `Unbounded` blanket is disjoint by parameter. This reproduces
`55_probes/probe_2b`'s mechanism exactly, reduced to a two-inhabitant `Pos` stand-in.

## 2. `attacker_unsealed.rs` against it: the hazard, live

```
rustc --edition 2021 --crate-type lib --extern arity_lib_unsealed=libarity_lib_unsealed.rlib attacker_unsealed.rs -o libattacker_unsealed.rlib
```

Exit 0. A downstream crate defines `MyOwnArity` and writes `impl InteriorSafety<MyOwnArity> for
Big { type Out = Safe; }`, asserting `Safe` for a real tower type against an arity that is neither
`Pos` nor `Unbounded`, with no routing through `Cmp` and no obligation to justify the claim against
anything the tower's comparison machinery establishes. Compiles clean. The orphan rule's "uncovered
local type parameter" carve-out (the same rule that lets any crate `impl ForeignTrait<MyType> for
i32`) is what makes this reachable: `MyOwnArity` is local to the attacker, so `Self` may be foreign.
The first draft of this file tried the fully generic form (`impl<Hd: Pos> InteriorSafety<MyOwnArity>
for Hd`) and hit `E0210` (`Hd` uncovered before the local `MyOwnArity`); the attack still goes
through the moment `Self` is a concrete tower type, which is the ordinary shape of algorithm code
written against one specific numeral.

## 3. `lib_sealed.rs` (Knuth's proposed fix, built)

```
rustc --edition 2021 --crate-type lib lib_sealed.rs -o libarity_lib_sealed.rlib
```

Exit 0. `Arity: sealed::Sealed` with `sealed` a private (non-`pub`) module; `Fin<P: Pos>` and
`Unbounded` are its sole constructors; `InteriorSafety<A: Arity>` carries the bound the unsealed
version lacked. The legitimate mechanism (`assert_unbounded_is_always_unsafe`,
`assert_finite_is_checked`) compiles unchanged in behaviour, spelled at `Fin<P>` where the unsealed
version spelled `P` bare, exactly the cost Knuth named ("at the cost of spelling `Fin<P>` where `P`
now sits bare", 62:224-225).

## 4. `attacker_sealed.rs` against it: both routes refuse

```
rustc --edition 2021 --crate-type lib --extern arity_lib_sealed=libarity_lib_sealed.rlib attacker_sealed.rs -o libattacker_sealed.rlib
```

Exit 1, two `E0277`s, both against `MyOwnArity: tower::sealed::Sealed`:

- **Route 1**, `impl Arity for MyOwnArity {}`: refuses because `sealed::Sealed` is unreachable
  outside the defining crate.
- **Route 2**, `impl InteriorSafety<MyOwnArity> for Big { type Out = Safe; }`, the exact forgery
  that compiled clean in outcome 2: refuses because the trait declaration's own `A: Arity` bound
  cannot be discharged.

Both errors carry rustc's own exhaustive "the following types implement" listing
(`tower::Fin<P>`, `tower::Unbounded`), unprompted, reproducing the seal-as-free-diagnostic dividend
this review has now found at `Rad<P>` (56, 62), the strategy door's `HostImplemented` (59), and here
a third time, at a vocabulary this review itself proposed rather than one already shipped.

## Reading

Sufficiency of the mechanism (probe 2b's coherence argument: `Unbounded` is not `Pos`, the two
blankets are disjoint, no specialisation needed) was never in question and is not reproduced again
here beyond outcome 3's clean compile. What outcomes 1 and 2 establish, freshly, is that the gap
Knuth named from reading is a real, reachable, silent one: nothing about the unsealed shape stops a
downstream crate from asserting the opposite of the guarantee `Unbounded` exists to make, for a real
tower type, with no error anywhere in the chain and no trace that anything was bypassed. Outcomes 3
and 4 establish that the two-line fix he named is sufficient to close it, at the cost he named and no
more.
