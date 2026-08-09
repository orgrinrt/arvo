# 106_probes outcomes

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml` **inside the tree**. HEAD `8a92eb4`, all runs 2026-08-05.

**Read this before reproducing anything here.** The identical `rustc` invocation from `/tmp` resolves
to stable `1.94.0`, which does not parse `type const` at all and reports it as an ordinary parse
error (`expected identifier, found keyword const`). I hit this mid-probe and it very nearly cost a
wrong conclusion in the opposite direction from the one I was testing for. `100_probes/OUTCOMES.md`
records the same trap. Every command below must be run with the probes directory as cwd.

| Probe | Subject | Outcome |
|---|---|---|
| 1 | does the capacity pair have to exist | pair reproduced; naive dissolution REFUSED, forbidden feature |
| 2 | is the pair forced by the language | REPRODUCTION of `79:154-157`, not a new result |
| 3 | where the pair actually comes from | WORKS: a const-parameter capacity has no pair, gate-permitted. New. |
| 4 | what forces a type-level check to fire | four mechanisms, two available to one kind of type each |
| 5 | route multiplicity needs a guarantee to be a defect | WORKS: exhaustive both sides |

Probe 3 is the only one here whose result is not already somewhere in the corpus. Probe 2 reproduces a
claim files 79 and 76 already compiled and `25:54-110` already worked out; probe 4's claim A restates
`55:163-165`, which no consolidation carries; probes 1 and 5 establish at their own instances things
the corpus states in prose. Which is which is the point, and file 106 section 4 is about the
difference.

---

## Probe 1, `p1_does_the_pair_have_to_exist.rs`

```
rustc --edition 2024 -O p1_does_the_pair_have_to_exist.rs -o /tmp/p1
```

**Claim A reproduced.** `Slot<N3, 7>` declares a `Nat` of 3 against a literal of 7. Both
const-evaluable, `LYING_NAT == 3`, `LYING_LEN == 7`, nothing raises. File 100's finding, at its
smallest, with a Peano `Nat` rather than the tower's binary one so the model is not carrying the
tower's own complexity into the question.

**Claim B REFUSED, and the refusal is the finding.** `type Array<T> = [T; <N as Nat>::VAL]`:

```
error: generic parameters may not be used in const operations
   --> p1_does_the_pair_have_to_exist.rs:102:36
    |
102 |         type Array<T: Copy> = [T; <N as Nat>::VAL];
    |                                    ^ cannot perform const operation using `N`
    |
    = note: type parameters may not be used in const expressions
    = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

`generic_const_exprs` is FORBIDDEN (`unstable-features.md`, forbidden table, op 2026-07-28). So the
naive dissolution of the pair is unavailable, exactly as `79:148-172` says.

---

## Probe 2, `p2_is_the_pair_forced.rs`

```
rustc --edition 2024 --crate-type=lib p2_is_the_pair_forced.rs -o /tmp/p2.rlib
```

`79:153-157` says the "rustc-suggested successor" cannot express the inductive step either. That is a
claim about a toolchain's behaviour and it is dated, so it gets re-run rather than inherited.

**Claim A.** With `#![feature(min_generic_const_args)]` (ALLOWED, `unstable-features.md` allowed
table, #132980) and a plain `const VAL: usize`, the array position is rejected and rustc names the
successor form:

```
error: use of `const` in the type system not defined as `type const`
  --> p2_is_the_pair_forced.rs:47:31
   |
47 |     type Array<T: Copy> = [T; <N as Nat>::VAL];
   |                               ^^^^^^^^^^^^^^^
   |
help: add `type` before `const` for `Nat::VAL`
   |
28 |     type const VAL: usize;
   |     ++++
```

**Claim B.** `type const VAL: usize` parses and typechecks. The array position then resolves. What
refuses is the INDUCTIVE STEP, one line earlier:

```
error: complex const arguments must be placed inside of a `const` block
  --> p2_is_the_pair_forced.rs:42:29
   |
42 |     type const VAL: usize = P::VAL + 1;
   |                             ^^^^^^^^^^
```

and with the const block supplied (`= const { P::VAL + 1 }`):

```
error: generic parameters may not be used in const operations
   --> p2e_tmp.rs:42:37
    |
42 |     type const VAL: usize = const { P::VAL + 1 };
    |                                     ^
    |
    = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

`generic_const_args` is not in the allowed table, and the workspace rule records it as needing
`-Znext-solver=globally` plus a roughly 314-site rewrite.

**This is a reproduction and not a finding, and the first draft of the file said otherwise before the
corpus was checked.** `79:154-157` already names the inductive step by its expression (`2 * P::VAL`)
and cites `76_probes/OUTCOMES.md` part A; `25_xu_building_the_exact_product.md:54-110` worked out the
`type const` mechanism sixty files earlier, including the same `use of const in the type system not
defined as type const` diagnostic. What this probe adds is a re-run on the current pin of a dated
toolchain claim that a repair now leans on. That is worth having and it is all it is.

The committed file carries the `type const` form (claim B). Swap the commented line to reproduce A.

---

## Probe 3, `p3_where_the_pair_comes_from.rs`

```
rustc --edition 2024 --crate-type=lib p3_where_the_pair_comes_from.rs -o /tmp/p3.rlib   # EXIT=0
```

**Compiles clean.** A capacity whose value is a `const N: usize` rather than an inductive numeral
supplies `type const VAL: usize = N`, which is a PATH, so `[T; <Self as Nat>::VAL]` resolves under
`min_generic_const_args` alone. There is no second name, so:

- `Dim<3>` has length 3 and value 3 because they are the same const read twice.
- The rank-3 composition `Axis<Dim<3>, Axis<Dim<4>, Axis<Dim<5>, Scalar>>>` has `COUNT == 60` and
  `size_of(Store) == 60`, asserted equal **through the trait route**, with no `AGREES`, no inline
  const block, and no construction door anywhere in the program.
- The bare-const-read case `100_probes/probe_2` claim C flagged as still leaking is closed too,
  because there is nothing to leak: `COUNT == 7` and `size_of == 7` for `Axis<Dim<7>, Scalar>` with
  no value constructed.

The extents are 3, 4, 5 rather than cubic on purpose, so a transposed or collapsed count would show.

The shipped tree reaches the same place gate-free: `arvo-tensor/src/capacity.rs:44-48` declares
`Dim<const N: usize>` with `type Array<T> = [T; N]` and `const CAP: Cap = cap(N)`, both read from one
`N`, and `arvo-tensor/src/lib.rs:21` carries only `const_trait_impl`. Read as a factual check on
whether the unpaired shape is buildable, not for what the design means.

---

## Probe 4, `p4_what_forces_the_check.rs`

```
rustc --edition 2024 --crate-type=lib p4_what_forces_the_check.rs -o /tmp/p4.rlib   # EXIT=0
```

Four mechanisms for making a type-level fact fire, and which kind of type each is available to.

**A, associated const mentioned by nothing: COMPILES SILENT.** `W<3, 7>` with
`const AGREES: () = assert!(N == K)` on it, declared and constructed, no diagnostic. The pricing
pillar's letter ("belongs on the type as an associated const") satisfied, and the check does not run.

This claim is `55:163-165` compiled: "It fires at **use**, not at declaration, because an associated
const nothing touches is not evaluated." File 55 stated it, named the same `AGREES` const, and
reproduced the same `E0080`. No consolidation carries it, checked over all ten this session. Claim A
is therefore evidence for file 106 section 4.2 rather than a new result; what is new here is the
mechanism split across claims C and D.

**B, mentioned in one constructor: fires through that one only.** The `BYPASS` const builds `V<3, 7>`
through the unmentioning door and compiles. Uncommenting `REFUSED`:

```
error[E0080]: evaluation panicked: B: disagrees
  --> p4tmp.rs:51:28
   |
51 |         const AGREES: () = assert!(N == K, "B: disagrees");
   |                            ^^^^^^^^^^^^^^^ evaluation of `b_one_route::V::<3, 7>::AGREES` failed here
```

This is the shipped bitfield's shape (`arvo/src/bitfield.rs:377` declaring `_BOUNDS`, `:393` and
`:399` mentioning it inside `new` and `from_bits`, `:370-374` constructing without it).

**C, free `const _` item beside the type: fires with no route at all.** With the overlapping
declaration swapped in:

```
error[E0080]: evaluation panicked: two fields overlap
  --> p4tmp.rs:83:25
   |
83 | /                         assert!(
   | |_________________________^ evaluation of `c_macro_declared::_` failed here
```

No construction, no mention, no route. Available because a MACRO has a declaration site to emit into.
The `O(k^2)` disjointness loop is nine lines inside the emission.

**D, the same mechanism for a consumer-instantiated generic: STRUCTURALLY UNAVAILABLE.** A free
`const _` can only assert about types it can name, and the design cannot name `W<3, 7>` before the
consumer writes it. The module is empty and the emptiness is the result.

**E, the fact in a type position every route must resolve: fires everywhere, by construction.**
`type Store = [u8; N]` beside `const COUNT: usize = N`. No route can avoid resolving an array length.

---

## Probe 5, `p5_route_multiplicity_needs_a_guarantee.rs`

```
rustc --edition 2024 --crate-type=lib p5_route_multiplicity_needs_a_guarantee.rs -o /tmp/p5.rlib  # EXIT=0
```

**A.** Six public exit routes modelled on the shipped `Bool` (public field, `Transparent::raw`,
`Deref`, `From`, `AsBool`, the `Try` exit), on a type with no invariant. Every door agrees at every
value, exhaustively over the whole two-element domain, asserted in const position so it is a
compile-time fact rather than a test. **Nothing separates, at any instantiation, ever.**

**B.** The same doors on a type carrying "the inner byte is never zero". Now they are not
interchangeable: five are reads and preserve the fact trivially, and the public field is a WRITE that
breaks it, with no `unsafe` and no diagnostic. Both halves asserted in const position, the positive
half exhaustive over all 256 inputs through the establishing route.

The discriminator this establishes: route multiplicity is a defect **relative to a guarantee**. With
one, exactly the write-shaped door is the problem and the sweep names it. Without one, six doors is a
complaint about surface, not about values, and it needs a different rule.
