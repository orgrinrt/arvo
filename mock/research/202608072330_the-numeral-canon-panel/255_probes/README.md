# 255 probes. What 251 and 252 measured, re-opened at this base, and one thing neither measured

Built for `255_leroy_the_primitive_established.md`. Run with `./run` from this
directory; every output lands under `out/` and is committed, so the numbers in
the file can be checked against the run rather than against the prose.

## The steps, and what each is for

| step | what it does | the control |
|---|---|---|
| 01 | rebuilds `251_probes/p01_the_value_layer_is_absent` against `arvo-format` at this base | the crate's own two controls, which 251 mutated and watched fire; here it is rebuilt and the build is the evidence |
| 02 | compiles what `mock/PRINCIPLES.md.tmpl:220` tells a consumer to write | `the_arm_control.rs` is the identical shape with `Width` in the field and builds; `lib.rs` asserts `Width` is not zero sized |
| 03 | greps every registry file for debug output and unstable-feature vocabulary, per file and for `ruling.toml` alone | `usize` on the same instrument, which must be non-zero and is |
| 04 | `mock the-positions '.@HEAD'` over the shipped tree | the tool's own zero rows per kind, which are results about the stack rather than the walk |
| 05 | `decider` and `answered` on the nine question rows the two files and this one turn on | two of the nine are answered and print an `answered` line, so an empty line is a real absence |
| 06 | counts `in_force` in the generated agent instructions' tier list | `stated` on the same file, non-zero |
| 07 | the principles page against the tree: `IFixed` in shipped source, `UFixed`'s declaration | `UFixed` on the same grep, non-zero |
| 08 | the two claims the dispatch brief made, re-queried | none needed; these are counts |

## What step 02 establishes

`p02_the_principles_locus/` is a consumer crate following the principles page.
Three arms, each a one-file `rustc` invocation against the same `arvo-format`
rlib the control crate built:

- `the_ifixed_arm.rs`: `use arvo_format::IFixed;` is refused with `E0432`, no
  `IFixed` in the root. The type the page names does not exist in the tree.
- `the_ufixed_field_arm.rs`: a field of `UFixed<32, 0>` asserted to be four
  bytes wide is refused with `E0080`. The type the page names exists and holds
  no value, because it is a format declaration and not a numeral.
- `the_arm_control.rs`: the identical shape with `Width` builds, so both
  refusals are about the named types and not about the invocation.

The `.stderr` files beside the sources are what each produced, committed.

## What this is not

Not a bench, not a design, and not a test in the suite. A hand check that wants
to be a lint: `design-doc-source-mismatch` already checks that every backticked
type in a crate's `DESIGN.md.tmpl` exists as a `pub` item, and the principles
page is exactly the surface it does not read. That extension is design work in
`mock/lints/`, which another agent is editing, so it is named here rather than
made.

## Region

    holds for: toolchain = nightly-2026-05-28, edition = 2024, crate_type = lib,
               base = b34d7a3c, std = none

A compile-time refusal, so no runtime dimension is claimed. Steps 03 to 08 are
reads of one repository at one commit and hold for that commit and no other.
