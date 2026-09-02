# Probes for `10_lattner_fresh_eyes_on_the_container_derivation.md`

Pin for every compile below: `rustc +nightly-2026-05-28`, reporting
`1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, aarch64-apple-darwin. A bare `rustc` outside
this tree resolves to stable, so the toolchain is passed explicitly every time.

Standard invocation, with `-O` added where codegen is the subject:

```
rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib --emit=metadata -o out/<n>.meta <n>.rs
```

`out/` carries the emitted metadata, the captured `stdout` and `stderr` of every probe, the two
assembly listings and their diff. `reproduced/` carries five sources copied unmodified from
`202607301300_formalization-spec-panel/137_probes/` and recompiled here; they are cited for what they
prove, never for how they are written.

`ladder.rs` is a copy of `137`'s include fragment. It has no imports of its own and does not compile
standalone by design; it is `include!`d by several probes here.

## What each probe checked

**Confirmation of the attempt.** `reproduced/p4_structural_wide.rs`,
`reproduced/p5_total_ladder.rs`, `reproduced/p6_surface_end_to_end.rs`, `reproduced/p7_law_site.rs`
all compile on the pin with no feature gate and no flag. `grep -n '#![feature(' ` over `137_probes/`
returns nothing.

**Whether the const-to-type bridge can be dissolved.** Seven probes, six of them refutations.

| Probe | Question | Outcome |
|---|---|---|
| `p01` | associated const as an array length, ungated | refused, anonymous constant |
| `p02` | the same under `min_generic_const_args` | refused, wants `type const` |
| `p03` | `type const` with an expression RHS | refused, wants a const block |
| `p04` | `type const` with a const block RHS | refused, names `generic_const_args` |
| `p05` | recursive bridge, halving the const in a where-clause | refused, names `generic_const_args` |
| `p06` | reading a `type const` through a generic type parameter | compiles, wrong direction |
| `p07` | `type const` RHS referencing a generic type parameter | refused, so `133`'s refusal stands |

**Improving the diagnostics without moving the mechanism.** All gate-free.

| Probe | Question | Outcome |
|---|---|---|
| `p08a` / `p08b` | does the digit tower print because the nat is a type alias | yes, opaque structs shorten it, but the idea cannot reach the computed side |
| `p09a` | control: `137`'s law-relation diagnostic reproduced | three digit towers |
| `p09b` | the law relation behind a named trait with its own message | no towers, written coordinates only |
| `p09c` | does inference survive the named relation | yes, `OI = 26` and `OF = 6` still solve |
| `p10` | `on_unimplemented` on the bridge trait | message fixed, width table still dumped |
| `p11` | `do_not_recommend` on each bridge row | table dump suppressed, 1002 bytes of error text removed |
| `p12` | all of the above on `137`'s full construction | compiles; assembly identical to the control |
| `p13` | the three site classes side by side | every message numeric |
| `p16` | collapsing four repeated bounds into one named trait | compiles, erasure survives, `_native16 = _arvo16` |

**Two facts about the bridge.**

| Probe | Question | Outcome |
|---|---|---|
| `p14` | does an unused bridge row cost anything | no. A row whose nat has no ladder impls at all compiles |
| `p15` | is a dense table even expressible | yes at 513, 2049 and 8193 rows |

`p15`'s wall-clock figures are an **ad-hoc quick spike with no substance**. No harness ran, so the
magnitude is unpriced and the probe supports an existence claim only. A dense table is a shape op has
refused four times and it is recorded as a residue, not offered as a proposal.

## Codegen comparison

```
rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib --emit=asm -o out/p07ctl.s reproduced/p7_law_site.rs
rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib --emit=asm -o out/p12.s    p12_improved_full.rs
diff <(grep -v '^\s*\.file\|p7_law_site\|p12_improved' out/p07ctl.s) \
     <(grep -v '^\s*\.file\|p7_law_site\|p12_improved' out/p12.s)      > out/asm.diff
```

32 differing lines, every one of them an anon symbol hash that is content-addressed on the filename.
Six bodies and 95 instructions on each side.

## Note on `out/`

Emitted `.meta` binaries are removed after the fact; they carry no information the sources and the
captured diagnostics do not, and one of them was 4.1 MB. Every `.err` and `.out` is the captured
`stderr` and `stdout` of the invocation named above. `out/p15_scale.md` records how the two larger
scale arms are regenerated and what they did and did not establish.
