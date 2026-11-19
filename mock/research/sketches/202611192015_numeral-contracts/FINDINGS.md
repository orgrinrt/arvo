# Three arms of the contracts a consumer bounds on

**Outcome:** WORKS, for all three arms, on the one test the obligation fixes. Each arm's refusals are
committed as compile-fail twins below with their compiler output, and one control beside them builds.

The plan for arvo's step 5 (`mock/research/202611130950_canon-double-check/PLAN.md`, the section on the
contracts a consumer bounds on) asks for a sketch of three arms before any design, each judged on one
thing that `obligation::the_unstable_machinery_does_not_reach_a_consumer` fixes: a consumer crate with
no feature line builds a function generic over a numeral that adds, orders and has an additive
identity. This sketch builds that consumer against each arm, over the real `Num` from `arvo-numeral`
read by path, and against an outside instance the consumer declares itself.

The three arms declare the same three contracts, `Sum`, `Order` and `Zero`, and implement them for
`Num<S, C>` by calling the value's inherent `const fn add`, its `cmp` and `Identity<Additive>`. `Zero`
is an associated constant in every arm, which needs no const trait anywhere. The arms differ in
whether `Sum` and `Order` are const traits.

| Arm | Crate | How the contracts are declared | Gate in the contracts crate |
|---|---|---|---|
| one | `arm-plain` | plain traits beside the inherent `const fn`s | none |
| two | `arm-opt` | `const trait` behind the `const` feature, plain without it, by two module paths | `const_trait_impl`, only under the feature |
| three | `arm-const` | `const trait` alone | `const_trait_impl`, always |

Arm two takes the shape `notko-hlist`'s `Cardinal` and `arvo-numeral`'s count already take: one trait
name, declared plain in `plain_path.rs` and const in `const_path.rs`, the feature choosing the module.

## The consumers

Four consumer crates share one body, `shared/consumer_body.rs`, so they differ only in which arm the
name `contracts` points at and which cargo feature their manifest turns on. No consumer carries a
`#![feature]` line. Each body holds `best_running_sum<T: Sum + Order + Zero + Copy>`, a signed 8-bit
saturating `Num` run through it, and `Tally`, an outside instance implementing the three contracts
from the consumer.

| Consumer | Arm | Arm two's feature | Result |
|---|---|---|---|
| `consumer-plain` | one | not applicable | builds, 2 tests pass |
| `consumer-opt-off` | two | off, plain path | builds, 2 tests pass |
| `consumer-opt-on` | two | on, const path, the unification case | builds, 2 tests pass |
| `consumer-const` | three | not applicable | builds, 2 tests pass |

`outputs/workspace-test.txt` is the whole workspace under `cargo test --workspace`, where cargo unifies
arm two's feature on for every member; `outputs/consumer-alone.txt` is each consumer run alone, where
`consumer-opt-off` rebuilds `arm-opt` on its plain path. Under arms two and three, `Tally` implements a
const trait with an ordinary `impl`, from a crate with no gate, and that builds: an outside instance
does not need the consumer to opt into const anything.

## The compile-fail twins

Each twin is a standalone crate under `twins/`, built alone, with its output in
`outputs/twin-<name>.txt`.

| Twin | Arm | What it writes | Result |
|---|---|---|---|
| `plain-generic-const-fn` | one | a generic `const fn` calling `Sum::sum` | refused, E0015: "method `sum` is not const because trait `Sum` is not const" |
| `opt-on-const-bound-no-gate` | two, feature on | a `[const] Sum` bound, no gate | refused, E0658: "const trait impls are experimental", issue 143874 |
| `const-const-bound-no-gate` | three | a `[const] Sum` bound, no gate | refused, E0658, the same message |
| `const-const-bound-with-gate` | three | the same, with `#![feature(const_trait_impl)]` | builds, the control |
| `opt-off-const-bound-with-gate` | two, feature off | a `[const] Sum` bound with the gate | refused: "`[const]` can only be applied to `const` traits" |

The control shows the second and third twins refuse for the missing gate and nothing else.

## What it establishes

Holds for: the three contracts as declared here, `Num<S, C>` over every `S: Sole`, an outside instance
in the consumer, toolchain `nightly-2026-05-28` (`outputs/toolchain.txt`), a consumer with no
`#![feature]` line, cargo resolver 3.

1. The obligation's test does not separate the arms. All three let a consumer with no feature line
   build and run the generic function, against the value and against its own instance.
2. A generic `const fn` over the contracts is out of every consumer's reach that names no gate, in
   every arm. Under arm one it is out of reach entirely, because the trait is plain; under arms two and
   three the consumer has to name `const_trait_impl` to write `[const]`, which is the unstable machinery
   reaching the consumer, by the consumer's own choice. A site naming its signature still computes at
   build time in every arm, through the value's inherent `const fn`s, which each arm keeps.
3. Arm two makes a contract's constness a property of the whole build rather than of the crate reading
   it. A gated consumer leaning on `[const] Sum` breaks when no crate in the build turns `const` on
   (`opt-off-const-bound-with-gate`), and builds when one does. A plain-bounding consumer is unaffected
   either way.
4. Arm three carries `const_trait_impl` unconditionally in the crate every consumer of the contracts
   depends on, and gives the gated consumer a constness it can rely on regardless of the rest of the
   build. The gate is on the workspace's allowed list (`rules show unstable-features`), and no
   consumer's own source pays for it.

## What it does not show

Anything about time or codegen: the generic sum against the concrete one is the design round's
reading, under the bench harness's `disasm`. Whether the contracts' laws hold is the law suite's.
Where the contracts live is the shape's question, read by an expert seat after this sketch. Only the
three contracts the obligation names are sketched; the full set is the design round's.
