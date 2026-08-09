# 83_probes outcomes

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved from
the repo's `rust-toolchain.toml`, run inside the tree. Edition 2024 throughout. The bench harness was
not run at all (its orchestrator overwrites committed artifacts, per `81:38-44`); nothing here is a
runtime performance claim.

## probe_1_three_levels_model.rs

`rustc --edition 2024 --crate-type lib`, **compiles clean, exit 0**, zero feature gates, `no_std`.
Every assertion is a `const` item, so the compile is the result. Establishes, at compile time:

- The four-preset width matrix at a 13-bit fixed-point model: `Hot` (13, 13, 16), `Cold` (13, 13,
  group of 8 elements in exactly 13 bytes), `Warm` and `Precise` (13, 26, 32), `Warm` float binary32
  (32, 32, 32). Warm/Precise fixed-point is the first configuration at which all three levels are
  pairwise distinct.
- Zero inter-value padding under `Bitpacked` as a theorem of the group projection: for every stored
  width 1 through 57 (the whole range the decode plan serves, not a sample),
  `group_bytes(w) * 8 == w * period(w)`, and the period is minimal (no smaller element count lands on
  a byte boundary).
- The level split feeding different consts: at a model bitpacked lowering with declared headroom
  (fields 13, stored 16), the plan consts (`period(16) = 1`, `group = 2` bytes) key on the stored
  width while the value mask (`0x1FFF`) keys on the fields' extent, and the two masks differ; at
  `Cold`'s ratified minimum the two widths coincide (13 = 13), which is the coincidence that hid the
  split.
- Declaration-site coverage (`fields' extent <= stored width`) in the `ByteCap`/`ShortCap` shape.

## probe_2_ungoverned_region_misorders.rs

`rustc --edition 2024 -O`, built and **run, exit 0**. Model: `Hot` fixed 13-bit under the forced
reading (fields extent = stored width = 13, dense container u16, physical padding at bits 13..16).
Whole matrix, counts asserted so an empty loop cannot pass:

- 57,344 same-value pairs (8,192 data times 7 nonzero container-padding patterns): a compare keyed on
  the raw container misorders **every** pair; the canonical projection reports Equal on every pair.
- Witness of raw order inverting value order: a dirty zero (raw 0x2000) above the largest clean datum
  (raw 0x1FFF).
- The container clause discharged by purity: the container constructor is a one-argument pure
  function, two calls bit-identical, and the committed padding (zero above bit 13) is observable
  through a transmute with no declared API, per file 73's perimeter argument applied at the second
  map.

Output line: `OK: 57344 pairs, raw misorders all, canonical door equal on all, constructor pure,
padding canonical`.

This is file 80 probe_3's finding re-instantiated at the preset table's own width, to pin **where**
the region sits in the three-level picture: at `[stored, container)`, not `[fields, stored)`.

## probe_3_coverage_refused.rs

`rustc --edition 2024 --crate-type lib`, **refuses, exit 1, E0080** (two occurrences of the code in
the diagnostic), at the declaration `const _: () = coverage_holds::<Undersized>();` for a lowering
declaring 8 stored bits under a 13-bit fields extent. The level ordering `fields <= stored` is a
declaration-site refusal before any use site exists. Kept so a later softening of the coverage check
surfaces as this file beginning to compile.
