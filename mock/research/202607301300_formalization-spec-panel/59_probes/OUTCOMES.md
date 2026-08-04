# Probe outcomes, file 59

Every probe here was built fresh for this dispatch on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
host `aarch64-apple-darwin`, run from inside the repo tree so a bare `rustc` resolves
`rust-toolchain.toml`'s pin (checked: `rustc --version` from the repo root and from `mock/` both
report the pin). Command shape throughout, no other codegen flags:

```
rustc --edition 2024 [-O] <probe>.rs -o /tmp/<name>
```

The bench artifacts are not here. They are real crates under `mock/benches/variants/` and are built
and run through the `arvo-benches` orchestrator; commands in the main file, section 3.3.

## probe_1_door_carrier.rs, WORKS

The door and the float environment as two carriers born sealed. `LoweringDoor ::= Quantised |
HostFloat<E: FloatEnv>`; `FloatEnv ::= IeeeDefault | FlushingNearest | DirectedUp`, every inhabitant
naming a control state and none meaning "unspecified". The receipt a build layer reads is a
`const fn` returning `Option<(Rounding, bool, bool)>` off the lowering type. Runs, prints:

```
probe_1 WORKS: door sealed, env sealed, receipt is four scalars off the type
```

## probe_2a_fallback_overlap.rs, REFUSED as predicted (E0119)

The door as a projection from the strategy alone, software fallback refined by a hardware impl where
the numeral is host-implemented. Verbatim:

```
error[E0119]: conflicting implementations of trait `DoorFor<_>` for type `Hot`
  --> probe_2a_fallback_overlap.rs:34:1
30 | impl<N: Numeral> DoorFor<N> for Hot {
   | ----------------------------------- first implementation here
34 | impl<N: Numeral + HostFormat> DoorFor<N> for Hot {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Hot`
```

## probe_2b_min_spec.rs, REFUSED twice over

The same shape under `min_specialization`, the only specialisation door the workspace permits:

```
error[E0658]: specialization is experimental
27 |     default type Out = Quantised;
   = help: add `#![feature(specialization)]` to the crate attributes to enable

error: cannot specialize on trait `HostFormat`
30 | impl<N: Numeral + HostFormat> DoorFor<N> for Hot {
   |                   ^^^^^^^^^^
```

Two independent refusals: `min_specialization` does not reach an associated type at all, and it does
not specialise on an ordinary trait. The only opener is the forbidden full `specialization`.

## probe_3_door_projection.rs, WORKS

The surviving shape with host-implementedness carried as a projected tag type
(`Numeral::Host = Hosted | NotHosted`). No feature gate. Prints:

```
probe_3 WORKS: door = (strategy -> default lowering -> door), no feature gate
```

## probe_3a_hot_refuses.rs, REFUSED as designed

`Hot` at a numeral the host does not implement. The refusal is legible and enumerates the legal
inhabitants, but its headline names the CARRIER (`NotHosted`), not the numeral:

```
error[E0277]: `NotHosted` is not a numeral this target's floating-point unit implements
183 |     assert_door::<Hot, Ranged11Abrupt, Quantised>();
note: required for `Hot` to implement `DefaultLowering<Ranged11Abrupt>`
```

## probe_3b_named_numeral.rs, the attribute is DEAD TEXT

probe_3a plus a `#[diagnostic::on_unimplemented]` on `DefaultLowering<N>` naming `{N}`. Output is
byte-identical to probe_3a: rustc reports the innermost unsatisfied bound and the outer trait's
attribute is never rendered. Recorded as a new instance of the decoder-ring ceiling
(58:658-673), at a position (`on_unimplemented` on a trait whose bound fails through a projected
associated type) nobody had tested.

## probe_3c_hosted_marker.rs, WORKS, and is the recommended shape

Host-implementedness as a sealed marker trait ON the numeral rather than a projected tag type. One
impl for `Hot`, no second impl, so probe_2a's E0119 has nothing to overlap with. Prints:

```
probe_3c WORKS: hosted-ness is a sealed marker ON the numeral
```

## probe_3d_hosted_refuses.rs, REFUSED, and the diagnostic is the best available

```
error[E0277]: this target's floating-point unit does not implement the numeral `Ranged11Abrupt`
help: the trait `HostImplemented` is not implemented for `Ranged11Abrupt`
    = note: The `Hot` preset lowers a float operation to the host instruction, which exists only
      for the numerals the target provides (binary16/32/64 on aarch64-apple-darwin). Choose `Warm`,
      `Cold` or `Precise`, which lower through the software quantiser at every numeral, or choose a
      numeral the host implements.
help: the following other types implement trait `HostImplemented`
 76 | impl HostImplemented for Binary32 {}   `Binary32`
 79 | impl HostImplemented for Binary64 {}   `Binary64`
note: required for `Hot` to implement `DefaultLowering<Ranged11Abrupt>`
```

The exhaustive "other types implement" list is rustc's, unprompted, and is exhaustive because the
marker is sealed. File 56's seal-as-free-diagnostic dividend, reproduced at a new position.

## probe_4_law_cannot_read_the_door.rs, REFUSED twice, plus a bonus wall

Two attempts to make a law's holding depend on the lowering: once through the door type, once
through the receipt's own const face.

```
error: generic parameters may not be used in const operations
49 | impl<N: Numeral, L: Lowering> AddCommutes<N> for (Witness, [(); L::HAZARDOUS_FACE as usize]) {}
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0207]: the type parameter `L` is not constrained by the impl trait, self type, or predicates
45 | impl<N: Numeral, L: Lowering<Door = Quantised>> AddCommutes<N> for Witness {}

error[E0207]: the type parameter `L` is not constrained by the impl trait, self type, or predicates
49 | impl<N: Numeral, L: Lowering> AddCommutes<N> for (Witness, [(); L::HAZARDOUS_FACE as usize]) {}
```

The const face is unreachable from a law by two independent walls, and the second is the
forbidden-feature list doing verification work again.

## probe_5_shipped_tag_needs_gce.rs (+ `_5b`, `_5c`), the shipped precedent is on forbidden ground

The `where Picker: Project<{ tag_hot_cold(N) }, ...>` shape of `arvo-strategy/src/container.rs:254`,
reduced to essentials with no arvo dependency, compiled three ways:

| gate | outcome |
|---|---|
| none | `error: generic parameters may not be used in const operations` (x2), help points at `generic_const_exprs` |
| `#![feature(min_generic_const_args)]` | `error: complex const arguments must be placed inside of a `const` block` (x2) |
| `#![feature(generic_const_exprs)]` | compiles, with the incomplete-feature warning |

Only the forbidden feature admits it. `arvo-strategy/src/lib.rs:11` carries that gate today.

## probe_6_cross_strategy_door.rs, WORKS, and carries a theorem

The whole 4x4 cross-strategy resolution matrix under the shipped rank ordering
(`arvo-strategy/src/lib.rs:105-108`), with each resolved strategy's default door read off. Prints:

```
probe_6 WORKS: 1 of 16 cells reaches the hardware door, and it is (Hot, Hot)
```

The assertion is on the whole matrix, not on a sample: `hardware_cells == vec![("Hot", "Hot",
"Hot")]` fails if any other cell reaches hardware or if that one stops doing so.

## probe_7_precise_has_no_hardware_door.rs, MEASURED

Whether `Precise`'s door is a preference or is forced. `Precise` owes saturation on overflow
(`arvo-strategy/src/lib.rs:135-139`); this asks the host what it delivers, under the entry control
state:

```
f32::MAX * 2.0      = inf (is_infinite true)
-f32::MAX * 2.0     = -inf (is_infinite true)
f32::MAX saturating = 340282350000000000000000000000000000000
probe_7: the host delivers infinity where `Precise` owes saturation
```

A different value, so the hardware door is not a `Lowering` for `Precise` at any pinning of the
environment (58:798-806). The one control state under which IEEE itself saturates is a directed
rounding mode, and it saturates only on the side rounding moves toward, so it is not `Precise`'s
two-sided clamp either; that clause is reasoned from the standard's overflow behaviour, not measured.
