# A value over the derived carrier: findings

This sketch asks how a number type in arvo holds its value in the carrier the placement derives, and how `add` on it can be a `const fn` written once and still reach every carrier on the ladder. Five arms answer it, each in its own crate. Each arm is used from a consumer crate with no `#![feature]` line, and is checked against `arvo_format::add` as the oracle. Everything is pinned: arvo at `d841c5e19325fc60648cc9a4bb48dcbb8b348df4` (arvo `dev` when the sketch was cut), toolchain `nightly-2026-05-28`, trybuild `=1.0.110`, plus `Cargo.lock`.

The outcome is that arms A, B and D build and pass, arm C is refused in the consumer, and arm E is refused in its own crate. B is the one to build on, because it is the only arm where a function generic over the signature bounds it by `Sole` alone. Its cost is one reviewed `unsafe` bit copy, sound only between two plain machine integers of equal size and so bounded on a sealed trait that admits nothing else. Its codegen matches A's byte for byte in the one reading taken.

Every number below comes from a file in `outputs/` that records the command it ran, or from a committed `.stderr`; `outputs/cargo-test.txt` is the clean run of the whole default suite, trybuild pass twins included. The assembly is an ad-hoc reading of one build and nothing more. Nothing here is a benchmark.

## The map

`common` writes the map once, as a list of admitted widths per rung. `Signed<W>` and `Unsigned<W>` for W in 1..=64 give 128 rows, and each row's `Carried::Repr` is the carrier `narrowest_carrier` names. Every row carries `assert_carrier!`: a `const` assertion that the carrier's size in bits equals `derive_sole::<Signature<point, Adapt<TowardZero, Wrap>>, Access>().carrier.count()` for the canonical point over that slot family. All 128 hold.

The wrong row (`Signed<13>` mapped to `i32`) is refused at check time, `common/tests/ui/fail/wrong_row.stderr`:

```
error[E0080]: evaluation panicked: the carrier the map names is not the one the placement derives
```

Its twin maps to `i16` and passes. The refusal is a const-evaluation panic, not a type error: the map has no type-level path from a width to a carrier that would let a mismatch fail to typecheck. That is what arm E tries, and it is refused.

Width 0 and width 65 have no row, so they are refused where a value is named. This happens before any arm's code runs, in each consumer's `width_0` and `width_65` pairs:

```
error[E0277]: the trait bound `Signed<0>: common::Carried` is not satisfied
error[E0277]: the trait bound `Signed<65>: common::Carried` is not satisfied
```

## The arithmetic, once per rung

`common::rung` is the arithmetic, and a macro expands it once per rung (8, 16, 32, 64) to `adapt_s/u`, `add_s/u` and `narrow_s/u` on the machine integer. `add` stays total, with no assertion, as the coordinator's first correction asked. Wrap is a shift pair for signed and a mask for unsigned. Saturate is a clamp to the width's range inside the carrier. No arm's `add` calls the oracle; the oracle lives in `common::oracle` and only tests read it.

`common/tests/rungs.rs` checks each rung exhaustively against the oracle over every pair of members. Widths 1 to 8 are checked in all four rungs, and widths 9 and 10 in rung 16. Each check covers `Integer<W>` and `UFixed<W, 0>` under `Wrap` and `Saturate`. All pass. The map never sends a width of 1 to 8 to rungs 32 and 64, so their exhaustive check is on widths they do not carry; the widths they do carry (17 to 64) are reached only by the arms' edge-set pass, never exhaustively.

One mutant per rung (`mutant-8` and the others) makes the rung's `adapt` return its input untouched, and each is killed. The pairs the mutant gets wrong, from `outputs/common-mutant-*.txt`, are 32512 at rung 8, 130560 at each of rungs 16, 32 and 64 over widths 1 to 8, and 1964544 at rung 16 over widths 9 and 10. The same four mutants are also killed through each building arm's own value tests (`outputs/arm-{a,b,d}-mutant-*.txt`). Those tests run the exhaustive pass for widths 1 to 10 plus an edge-set pass for every width 1 to 64.

## The arms

| arm | outcome | what it writes | case (3) bound |
|---|---|---|---|
| A | builds, passes | 8 inherent impl blocks, one per carrier | `Sole<Repr = i64>` |
| B | builds, passes | 1 impl, a dispatch over size and signedness | `Sole` |
| C | refused in the consumer | 8 `const impl`s | none that builds |
| D | builds, passes | 8 concrete types and an alias | `Sole<Repr = i64>` and a turbofish |
| E | refused in its own crate | 4 impls of a carrier-per-rung trait | not reached |

Arm A puts the carrier in a defaulted second parameter, `Num<S: Sole<Repr = C>, C = <S as Sole>::Repr>`, and writes `from_raw`, `raw` and `add` as `const fn` in one inherent impl per carrier, `impl<S: Sole<Repr = i16>> Num<S, i16>`. It needs no feature and no `unsafe` (`#![forbid(unsafe_code)]`). Its cost shows up in the consumer. A function generic over the signature can only call `add` once it has pinned the carrier, so case (3) is generic over the signatures of one carrier, never over all of them. A carrier named wrongly is refused with E0271 (`consumer-a/tests/ui/fail/carrier_named_wrongly.stderr`):

```
error[E0271]: type mismatch resolving `<Signature<Integer<13>, Adapt<TowardZero, Wrap>> as Sole>::Repr == i32`
```

Arm B holds `S::Repr` directly in `Num<S: Sole>(S::Repr)`. A `const fn` body cannot call a trait method on the carrier, so `from_raw` and `add` match on `(size_of::<S::Repr>(), <S::Repr as Machine>::SIGNED)`. Those are constants of the instantiation. Inside the matching arm, the bits move to the machine integer of the same size and signedness by `copy`, the rung's function is called there, and the bits move back. `copy` is the arm's one `unsafe`, a `ptr::read` guarded by an ordinary `assert!` on equal size. The guard cannot be an inline `const` block: every arm of the match is monomorphised, the dead ones included, and a `const` assertion fails in those (`copy::<u64, u32>` at a 32-bit carrier). As an ordinary assertion it is evaluated only on the arm taken, and it folds away. A bit copy is sound only between two types whose every bit pattern is a valid value. So `copy` is bounded `T: Bits, U: Bits`, where `common::Bits` is a sealed trait implemented for the nine plain machine integers (the eight carriers and `i128`) and nothing else. Both `Machine` and its `Wide` are `Bits`. With that bound and the size assertion, `copy` is a safe `const fn` that is sound for every type it accepts. The match names each size, 1, 2, 4 and 8, and ends in `unreachable!()`, so no other size falls through to a 64-bit path. Case (3) bounds `S: Sole` and nothing else.

Arm C is the only arm with a feature, `const_trait_impl`, in the value crate alone. It declares `pub const trait Carrier` with a `const impl` for each of the eight integers, and `add` bounds `S::Repr: [const] Carrier`. The value crate builds, and the consumer is refused both ways (`outputs/arm-c-*.txt`; the long paths are elided as `...` below). With the conditionally-const bound, case (2), `add` in a `const` item, is refused:

```
error[E0658]: cannot call conditionally-const associated function `Num::<...Integer<13>...>::from_raw` in constants
error[E0658]: cannot call conditionally-const method `Num::<...Integer<13>...>::add` in constants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
```

With the always-const bound (`const Carrier`, feature `always-const`), case (2) builds, and case (3) is refused because the consumer's bound cannot name the const-ness without the feature:

```
error[E0277]: the trait bound `<S as Sole>::Repr: const Carrier` is not satisfied
```

Writing the bound as `Sole<Repr: const arm_c::Carrier>` in the consumer is refused the same way:

```
error[E0658]: const trait impls are experimental
```

So arm C cannot give a featureless consumer both a `const` item and a function generic over the signature: the feature leaks into the consumer.

Arm D writes eight concrete types, `NumI8` to `NumU64`, each with its own const `from_raw`, `raw` and `add`. `pub type Num<S> = <<S as Sole>::Repr as ValueOf>::Value<S>;` picks the right one. Cases (1), (2), (4) and (5) build. Case (3) needs `Sole<Repr = i64>`, as in A. Even with that bound, the call cannot infer `S` through the alias, because `Num<S>` is a projection. `consumer-d/tests/ui/fail/generic_call.stderr` shows this; its twin names `S40`:

```
error[E0283]: type annotations needed
   = note: the type must implement `Sole`
```

Inside `consumer_cases`, where the result is used, the same refusal surfaced as E0282 at the use.

So a consumer turbofishes every generic call. The shared `consumer_cases` does that for all arms, and the comment there says why. The `Lane` impl had to name the accumulator as `<$c as Machine>::Wide`. `S::Wide` did not normalise to `i128` through the projection and was refused with E0308.

Arm E tries to make the rung a type-level fact under `min_generic_const_args`: `trait RungOf { type const RUNG: u32; }`, with `pub type SignedRepr<const W: u32> = <Rung<{ <Signed<W> as RungOf>::RUNG }> as SignedCarrier>::T;`. The refusal was expected and is recorded verbatim (`outputs/arm-e*.txt`). The brief's `#[type_const]` attribute is not known on this nightly:

```
error: cannot find attribute `type_const` in this scope
```

Once the syntax is `type const`, the impl's value `narrowest_carrier(...).count()` is refused:

```
error: complex const arguments must be placed inside of a `const` block
```

Wrapped in `const { }` (feature `const-block`), it is refused for using the width:

```
error: generic parameters may not be used in const operations
```

The help names `generic_const_args`, which is not on the vetted list, so the arm stops there.

## What the consumers wrote

The five cases are written once, in `common::consumer_cases!`. Each arm's consumer crate expands them with no `#![feature]` line, so what expands is what a consumer writes. Case (1) holds a `Num<S13>` in a struct field. Case (2) is `pub const SUM: Num<S13> = Num::<S13>::from_raw(4000).add(Num::<S13>::from_raw(300));`, which wraps once in 13 bits to `4300 - 8192`. Case (3) is the bound in the table above. Case (4) is generic over `const FRAC: i32` on `UFixed<64, FRAC>` with no bound at all, in every building arm. Case (5) is a kernel generic over `L: Lane` calling `widen`, `wide_add`, `wide_sub` and `narrow`, and it builds in every building arm.

The compile-fail pairs are committed as `.rs` beside a twin differing in one token, or in one `.push` line for the counts, with the diagnostic as `.stderr`:

| pair | diagnostic | where |
|---|---|---|
| width 0 against 1 | E0277, `Signed<0>: Carried` | consumers A, B |
| width 65 against 64 | E0277, `Signed<65>: Carried` | consumers A, B |
| carrier named `i32` against `i16` | E0271 | consumer A |
| add across `S14` and `S13` | E0308 | consumers A, B |
| the field `a.0` against `a.raw()` | E0616 | consumers A, B |
| the map row `i32` against `i16` | E0080 | common |
| a generic call without and with `::<S40>` | E0283 | consumer D |
| 129 `Counted` terms of `UFixed<8, 0>` against 128, limit 256 | E0080, the count | consumer B |
| 257 `Counted` terms of `Integer<8>` against 256, limit 512 | E0080, the count | consumer B |
| 129 `Counted` terms in a `const` item against 128, limit 256 | E0080, the exact add | consumer B |
| 128 `Counted` terms against 127, default limit | E0320 | consumer B |

The private field's twin reads through `raw()`, since no single token makes the field reachable. Arm B has no carrier parameter, so the wrong carrier has no pair there.

## The assembly, an ad-hoc reading

`asm-probe` takes and returns the value type, so what is read is `add` alone. `outputs/asm-release-x86_64.txt` is one `release` build for x86_64 with the default target features. It is an ad-hoc reading, not a measurement. Arm B's probes are emitted as aliases of arm A's (`b_i13_wrap = a_i13_wrap` and so on), so the two compile to the same bytes. The full-width wrapping add is one instruction, and the native operation is folded into it:

```
a_i64_wrap:
	leaq	(%rdi,%rsi), %rax
	retq
native_i64_wrapping_add = a_i64_wrap
```

At 13 bits in 16, signed wrap is an add, `shll $3`, `movswl` and `shrl $3`. Unsigned wrap is `leal` and `andl $8191`.

Saturate is two steps. The first is the carrier's own saturating add. For signed, `addw` sets the overflow flag and `cmovnol` picks the wrapped sum over the saturated one built from its sign. For unsigned, `cmovael` picks the sum over 65535 on carry. The second step is two `cmov`s that clamp to the width's range ends, -4096 and 4095, or 8191.

At a width below the carrier the first step is dead. Two 13-bit members sum within 14 bits and never overflow an `i16`, so the `cmovnol` and the four instructions feeding it, or the `cmovael`, are wasted. The real crate can take this: where the width is below the carrier, a saturating add is the wrapping machine add followed by the clamp, which is what `rung::add_*` should emit there.

The reading without `#[inline]` on the rung functions is kept beside it (`outputs/asm-release-x86_64-without-inline.txt`, built with `--features common/outline`). There every probe is a tail jump through the GOT to the rung function in `common`, which is not inlined across the crate. `#[inline]` on each rung function is what makes the two arms equal.

## The accumulator, on arm B

`arm-b/src/wide.rs` gives the accumulator. It holds a second type per signature, `Wide<S>`, with the exact sum in the signed integer twice the carrier's width. `Num::widen`, exact `add` and `sub`, and `narrow` are all `const fn`, and `narrow` is the one adaptation, through `rung::narrow_*`. The results are in `arm-b/tests/accumulator.rs` and `outputs/arm-b-accumulator.txt`.

`narrow` of the sum of four members equals the oracle's `adapt` of their exact sum in every order at width 8. The check runs over 12 edge-heavy members per signature, every 4-tuple and all 24 orders, for `Integer<8>` and `UFixed<8, 0>` under `Wrap` and `Saturate`. That is 497664 cases each, with zero wrong.

An accumulator that adapts at each step agrees under `Wrap`, since wrapping is a homomorphism onto the residues. It also agrees for unsigned `Saturate`: members are never negative, so the partial sum only climbs and `min(MAX, sum)` does not depend on order. It fails for signed `Saturate`, in 104352 of the 497664 cases. `[127, 1, -1, -127]` gives -1 stepwise where the exact answer is 0, and the reordering `[-1, 127, 1, -127]` gives 0.

The term bound is how many members the register holds the exact sum of, whatever they are. Signed, it is `2^(R - W)` for register width R and member width W, since that many of the lowest member reach the register's bottom exactly. Unsigned, it is `floor((2^(R-1) - 1) / (2^W - 1))`. At each ladder carrier's full width:

| carrier | register | signed bound | unsigned bound |
|---|---|---|---|
| 8 | i16 | 2^8 = 256 | 128 |
| 16 | i32 | 2^16 | 2^15 |
| 32 | i64 | 2^32 | 2^31 |
| 64 | i128 | 2^64 | 2^63 |

The bound grows as the width falls below the carrier. At `Integer<33>` in i128 it is 2^95, and at `Integer<1>` in i16 it is 2^15. `MAX_TERMS` is a `u128` because a signed width of 1 in i128 reaches 2^127. A `USize` term count, as the name page has it, cannot hold the bound at the 64-bit rung. At the bound the sum is exact: 256 copies of -128 give -32768, and 128 copies of 255 give 32640, each equal to the oracle's adapt.

One term past the bound, what happens depends on where the add runs:

- In a build with overflow checks it panics: `attempt to add with overflow`.
- In a build without them it wraps silently and `narrow` answers wrong. 257 copies of -128 under signed `Saturate` wrap to 32640 in the register, and `narrow` gives 127, where the oracle gives -128.
- In const evaluation it is refused at the add: `error[E0080]: attempt to compute 32640_i16 + 255_i16, which would overflow`. This is shown by `consumer-b/tests/ui/fail/terms_past_the_bound_in_a_const.stderr`, 129 terms of `UFixed<8, 0>` in a `const` item against a twin of 128, with the recursion limit at 256 so no other error fires.

A type-level term count builds under the vetted gates, with no feature at all. `Counted<S, N>` carries `N` as a Peano type (`Zero`, `Succ<N>`). `push` returns `Counted<S, Succ<N>>`. `narrow` holds `const { assert!(N::COUNT <= Wide::<S>::MAX_TERMS, ...) }`. No `generic_const_exprs` is involved: the count is a type and the check is a constant of it. Consumer B uses it in a `const` item (`counted::EDGE`, the edge case above, exactly 0).

The catch is the default `recursion_limit` of 128. The nested type is refused at a depth of 128, before any check runs (`consumer-b/tests/ui/fail/terms_past_the_recursion_limit.stderr`, twin at 127):

```
error[E0320]: overflow while adding drop-check rules for `Counted<Signature<UFixed<8, 0>, ...>, Succ<Succ<...>>>`
```

The smallest bound on the ladder is 128 (`UFixed<8, 0>`), so at the default limit the count's own check can never fire. With `#![recursion_limit = "256"]` in the consumer, which is stable, the 129th term of `UFixed<8, 0>` is refused by the count (`terms_past_the_bound.stderr`, twin at 128). With the limit at 512, the 257th term of `Integer<8>` is refused the same way (`signed_terms_past_the_bound.stderr`, twin at 256):

```
error[E0080]: evaluation panicked: more terms than the register holds the sum of exactly
```

A Peano count therefore guards a bound only once the consumer raises its recursion limit past it. The two full-width 8-bit bounds were probed, at depths 129 and 257.

Below full width, the 8-bit rung's bounds climb toward 2^15, at `UFixed<1, 0>` and `Integer<1>`. `UFixed<7, 0>` is 258, the same order as the probed cases; the rest of the climb was not probed. The 16-bit rung starts at 2^15 and the wider rungs go far past it. No depth near those was tried, so whether a nesting that deep builds is open. The count covers the bounds at the 8-bit rung's full width and just below it, and nothing past those was shown.

## Built on, and what it costs

Build on arm B. It is the only arm where a function generic over the signature bounds `S: Sole` and nothing more, so a kernel is written once for every signature. It needs no feature in either crate. `add` is a `const fn` written once over the rung arithmetic, and in the one reading taken its codegen is the same as arm A's.

It costs four things:

- the `unsafe` in `copy`, whose precondition is that both sides are plain machine integers of equal size: every bit pattern valid, no padding, no niche. The sketch holds it with a sealed `Bits` bound on both type parameters plus a size assertion. The real crate keeps that bound, or makes `copy` an `unsafe fn` carrying the precondition to each call site, and the review checks that no type outside `Bits` can reach it;
- the sealed `Bits` and `Machine` traits, which close the carrier set to the eight ladder integers and their accumulators and make that closure part of the contract;
- a dispatch macro that names all eight rung functions per operation;
- `#[inline]` on every rung function, without which generic code calls them out of line.

Arm A is the fallback with no `unsafe`, at the price of a carrier pinned in every generic bound. Arm C waits on `const_trait_impl` reaching consumers. Arm E waits on generic const arguments that are not vetted.

## What was not as the brief said

The brief spelled the associated const as `#[type_const]`, which this nightly does not know. It takes `type const NAME: T`.

The brief listed width 65 as an E0277 refusal. Both width 0 and width 65 are refused with E0277 at `Carried`, because the map has no row for either. The width check in arvo-format is never reached.

Arm B needed `#[inline]` on the rung functions to match arm A. Arm D needed `<$c as Machine>::Wide` for `Lane`'s accumulator, and a turbofish on every generic call. `MAX_TERMS` needs `u128`, not `USize`. Under unsigned `Saturate` the stepwise accumulator does not fail; only signed `Saturate` does.
