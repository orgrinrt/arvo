# Sketch: a packed word of named fields over the bit container

**Outcome:** WORKS

The question is whether a word of a declared width, divided into named fields of declared widths, builds over `arvo-bits` as it stands on `work/arvo-bits-tier` (`1fd4d78e`) under the pinned `nightly-2026-05-28`, with no feature beyond `const_trait_impl`: every field read and written in a constant, a field one bit wide typed as a bit, and a layout whose fields do not tile the word refused at build rather than checked at run time. The need is hilavitkutin's identity handle, `hilavitkutin-sym/src/handle.rs:19` and `:73` on hilavitkutin `dev` `ad6da29e`, a 32-bit word of a 28-bit id, a 3-bit kind and a 1-bit flag.

## The shape probed

A consumer names a layout and its fields as marker types. `Layout` gives the container the word is held in and the fields as a list `(A, (B, (C, ())))`. `Field` gives the layout the field belongs to, the type the field reads as, and `BELOW`, the count of the word's bits under the field. `Word<L>` is `repr(transparent)` over `L::Container`, so `of` and `bits` move the container in and out and nothing else. `get::<F>` and `with::<F>` read and replace one field. `Lane` is the one contract both a container and a field value meet: a width and a way in and out of the low bits of a `Bits<64>`. `Bits<N>` meets it through `cast`, and `Bool`, the type the tier already reads one bit as, meets it through the tier's `bit` and `with_bit`, so a one-bit field reads as a bit and never as a number.

Three checks run at build, each as an assertion inside a `const` item forced by every entry point: no two fields share a bit, the widths sum to the container's width, and the fields' union covers the container. The first two together make the third redundant for a well-formed list, and it is kept as the check a reader can see. A fourth refuses a field read through a word whose layout does not list it, by asking the list whether it holds a field at that place and width.

## Attempts

The first attempt put the `[const] Lane` bound on the inherent impl block, and the compiler refused it: `inherent impls cannot have [const] trait bounds`, with `E0277` at each call of `low` and `from_low` inside it. The bound moved onto each `const fn`, where it is accepted.

The second attempt derived `Clone`, `Copy`, `PartialEq`, `Eq` and `Debug` on `Word<L>`, and the example's `assert_eq!(back, H)` refused with `E0369`: the derive bounds each impl on the layout marker `L`, which the consumer has no reason to derive anything on. The impls are written by hand, bound on `L::Container`. This is the failure hilavitkutin's own design describes for its handle (`hilavitkutin-sym/DESIGN.md.tmpl`, the paragraph on bounding equality on the layout rather than the shape).

The third attempt builds. `outputs/the_handle.txt` is empty because the example ran and every assertion held.

## What must fail, and does

Each refusal is an example that has to fail, and each failure names its rule. Outputs are in `outputs/`.

| Example | What is wrong | Refused with |
|---|---|---|
| `short_sum.rs` | 28 and 3 bits in a 32-bit word | `the field widths of a packed word do not sum to the width of its container`, evaluating `Word::<Handle>::TILED` |
| `overlap.rs` | 28 and 4 bits sum to 32 but share bit 27 | `two fields of a packed word share a bit` |
| `unlisted.rs` | a field naming the layout that the layout does not list | `a field read or written through a word whose layout does not list it`, evaluating `Listed::<Stray>::NEW` |

The control is `the_handle.rs`, which builds and runs with the same machinery and the right widths, so the refusals are the checks firing and not the machinery failing to build.

## What this does not establish

That the round trip to the container is free. `Word<L>` is `repr(transparent)` and `of` and `bits` are moves in the source, which is a claim about the source; the claim about the lowered code needs `cargo mock bench disasm`, and nothing here was disassembled or timed. Nor that `get` and `with` lower to one shift and one mask: the body goes through `Bits<64>` and the compiler is trusted to fold it, which is the same claim and the same instrument.

Whether the refusal reaches a call made at run time as well as one in a constant. Every example here calls from a `const` item. The tier's `Within::NEW` is forced the same way and its design records that a call reaches it at codegen, so `cargo build` refuses where `cargo check` may not; the crate's own `compile_fail` tests are where that is pinned.
