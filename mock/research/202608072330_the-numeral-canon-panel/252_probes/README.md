# 252 probes. Does arvo's unstable machinery reach a consumer

Built for `252_kiselyov_the_primitive_surface_locus.md`, against
`obligation::the_unstable_machinery_does_not_reach_a_consumer`, whose own `gap`
says of the question these answer: "it is unmeasured".

Every arm is a two-crate compile driven by `rustc` directly, so nothing about
cargo, a workspace or a feature resolver can be the cause. The library is built
first with whatever feature it needs; then a consumer crate carrying **no**
feature attribute of any kind is built against it. The question per arm is one
thing: does the consumer need the gate.

## Running

    ./run.sh && ./run2.sh && ./run3.sh && ./run4.sh

Output is captured under `out/` and is committed. `run.sh` writes `out/a`, `out/b`
and `out/c`; `run2.sh` writes `out/d`; `run3.sh` writes `out/e`; `run4.sh` writes
`out/f`. Exit status is not the result; the captured text is.

## The arms, and what each was for

| Arm | Shape | Consumer needs the gate | Where |
|---|---|---|---|
| A | const expression in a public **return type** | **yes** | `out/a`, `out/d` |
| A3 | const expression in a public **`where` bound** | no | `out/e`, `out/f` |
| B | stack-owned type at a **const generic parameter**, instantiated | no | `out/b`, `out/d` |
| C | coordinate set at **associated consts** | no feature anywhere | `out/c` |

## Why each arm has a control, and what each control rules out

An arm with no failing case proves nothing, and three of these read cleanly for
the wrong reason unless the control is there.

- **A.control** is a consumer naming the same crate's signature that carries no
  const expression. It builds, which rules out "this dependency cannot be
  consumed at all" as the explanation for A's failure.
- **A2** (`gated_user.rs`) is A's consumer with the feature turned on in the
  consumer. It builds, which is what makes A's `E0308` attributable to the
  absent feature rather than to unevaluated consts never normalising across a
  crate boundary for anybody. Without A2, A has two readings and no way to pick.
- **A3-control** (`bound_lib2.rs`) implements the bound's trait at exactly three
  widths. The ungated consumer at `6 + 7 = 13` builds and the one at `6 + 6 = 12`
  is refused with `E0277` naming `Bits<12>`. That is what makes A3's clean build
  a containment result rather than a report that the bound is never checked
  there: the diagnostic shows the expression normalised in the ungated consumer.
- **B2** forces the ADT const parameter through a const assertion, positive and
  negative. The negative fails with `E0080` evaluating
  `Thirteen::DECLARED == 12`, which is what separates "it compiled" from "it was
  actually resolved here".
- **C.negative** writes the one coordinate value a `u32`-shaped door cannot hold,
  `Width::bits(-4)`, and is refused with `E0600`. Without it, C's clean build
  says only that something compiled.

## What it measures, with the region

    toolchain = nightly-2026-05-28 (rustc 1.98.0-nightly 57d06900f)
    edition = 2024, crate_type = lib, std = none, linkage = rlib
    shapes = the four in the table above and no others

A compile-time refusal, so no runtime dimension is claimed and none is implied.
`Width` in arms B and C is reproduced at the shape `arvo-format` ships,
`repr(transparent)` over a `u32` with a private field on a public struct,
because the door ruling's `promotion` records that private field as the reason
`min_adt_const_params` refuses what `adt_const_params` accepts. A public-field
imitation would be measuring a different type.

## What it does not measure

Trait associated types, generic const expressions in a struct field type, an
impl-trait return position, a const expression reached only through a macro, and
anything at all under cargo feature unification. Named because a shape not
listed here is not claimed, in either direction.
