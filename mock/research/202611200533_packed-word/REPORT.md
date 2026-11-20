# A Packed Word of Named Fields, Asked by Hilavitkutin

This is the record of the job `arvo-packed-word`, which put row A of hilavitkutin's port plan through arvo's flow: the need as an `obligation` row, then the word in `arvo-bits` through design round `202611200533-wwxt`. The plan is `mock/research/202611200345_arvo-port-plan/README.md` on hilavitkutin `dev`, section "The rows arvo still owes, in hilavitkutin's terms", and its job 2, identity and strings, waits on this row.

The sections below are written as the job reaches them. The demand side is first, because the obligation row cites it.

## The demand hilavitkutin states

Read at hilavitkutin `dev` `ad6da29e`.

Hilavitkutin's identity handle, `Sym`, is one 32-bit word. Its default shape lays out a 28-bit `id` at bits 0 to 27, a 3-bit `kind` at 28 to 30 and a 1-bit `flag` at 31 (`mock/crates/hilavitkutin-sym/DESIGN.md.tmpl`, the table under "How the 32 bits divide is a shape"). A second shape, the wide-tag one, divides the same word as 26, 5 and 1 (`hilavitkutin-sym/src/handle.rs`, the second layout declaration, at line 73). The sites written today with the removed tree's field macro are `hilavitkutin-sym/src/handle.rs:19` and `:73`; the field width projection that reads a field's type back is `hilavitkutin-sym/src/shape.rs:185`.

What the consumer asks of the word, each in its own document:

The fields are read and replaced by name: "`SymLayout` generates per-field accessors (`kind()`, `id()`, `flag()`), setters, and per-field mask constants."

The layout is a wire fact fixed when the consumer builds: "A shape is a wire format. Two `Sym`s built under different shapes divide the same 32 bits differently, so they are not comparable and must not meet in one store." (the same design, emphasis dropped).

A field one bit too narrow, or two fields overlapping, is the failure its tests exist to catch: "The maximum is the case that finds a field one bit too narrow", and "Writing one field disturbs no other. This is the bitfield failure every law here was previously blind to, because a law that reads one field at a time cannot see an overlap between two."

The handle is compared and persisted as its raw bits: "`to_bits()` (the raw `Bits<32, Hot>` view for tests and persistence)", and "Handle equality is raw-bit equality".

The flag is read as a bit, not a number: `fn get_flag(self) -> Bit<Hot>` in `handle.rs`, beside `type Id = Bits<28, Hot>` and `type Kind = Bits<3, Hot>`.

So the need, stated without a mechanism: a word of a declared total width divided into named fields of declared widths, each field read and written in a constant, a one-bit field with its own type, the widths summing to the word checked at build, and the whole convertible to and from the exact-width container at no cost. The nearest row, `obligation::an_exact_width_container_a_consumer_can_alias_and_pin`, asks for the container only.

## What the canon said, and the stamp

The registry has no row on a word divided into fields. Its nearest terms are the lens and shared placement, `proposal::the_lens_degenerates_to_an_ordinary_value_at_sole_occupancy` and `proposal::at_shared_occupancy_no_per_element_footprint_observation_exists`, both proposals at one expert, and the value-stance row's gap that the packed column waits on the carrier deferral. The packed word is not that case, since every field is read out into a value of its own at sole occupancy, and the round says so in its topic and builds nothing on those rows. `cargo mock already-said` from kaski's root found "packed" in no row and "bit field" in none; arvo carries no such tool of its own.

So placement and shape were the canon's silence, and went to stack-coord for a stamp by two independent seats on 20 November, with the sketch. Stack-coord dispatched a blind second seat and told the job to proceed on its shape. The four calls put to it: the word in `arvo-bits` beside the tier, `tier.rs` untouched; the marker-type shape of `Layout`, `Field` with `BELOW` and `Word<L>`; the one-bit field as `arvo_format::Bool`; and `Lane` as the one contract of container and field value. What the second seat returns, and the question ticket to op that follows a stamp, are stack-coord's.

## The round

Round `202611200533-wwxt`, on `work/arvo-packed-word`, cut from `work/arvo-bits-tier` at `1fd4d78e` because the tier has not landed on `dev` and the word builds on it (stack-coord agreed, 20 November). The sketch `mock/research/sketches/202611200520_packed-word/` came first and records two refusals on the way, a `[const]` bound on an inherent impl and a derive bounding equality on the layout marker. The obligation row `a_word_divided_into_named_fields_of_declared_widths` and the design went through the doc changelist; `src/word.rs`, the tests and the bench through the src changelist.

## Tests

`cargo mock test -p arvo-bits --cheap`: 31 run, 31 passed, the seven of `the_packed_word` and `compile_fail` with seven new cases among them (six refused at build, one that must build), on `nightly-2026-05-28`.

Two hand mutants of `src/word.rs`, each run and undone. A read moved up by one bit turned five of the seven law tests red. A write that keeps the field's old bits turned the two round-trip tests red; the test of other fields reading as they did stays green under it, as it must, since that mutant moves no other field.

The new `trybuild` fixtures were first written in a container whose `rust-src` was recorded as installed with no files, so rustc printed no source lines for `core` and every fixture of the tier mismatched as well. The tier's 37 were put back from `HEAD` untouched, `rust-src` reinstalled, and only the seven new ones blessed (ticket `4f5fbcf49470`).

## What it lowers to

`cargo mock bench disasm packed-word <arm> <symbol>` over the bench `mock/benches/packed-word/`, both arms in release as the harness builds them, `x86_64` in this container (`linux-x86_64-c4-container-intel`), `nightly-2026-05-28`, each an `extern "C"` function taking and returning the host's `u32`. Records under `mock/benches/results/packed-word/disasm/`.

| Operation | `packed-word-host`, by hand | `packed-word-word`, through `Word` |
|---|---|---|
| round trip `of` then `bits` | `mov`, `ret` | `mov`, `ret` |
| the 3-bit kind read | `mov`, `shr $28`, `and $7`, `ret` | the same |
| the 28-bit id written | `and $0xf0000000`, `and $0xfffffff`, `lea`, `ret` | the same |
| the one-bit flag read | `mov`, `shr $31`, `ret` | the same |

Instruction for instruction the same in all four. Nothing is claimed for another target, another width or a debug build, and nothing was timed.

## Problems ticketed

`ac412c811f0d`, the stand-up could not render arvo's agent files from a clone scoped to kaski. `3457a32e273c`, `cargo mock check` in arvo built every member's tests and filled the disk at 40.6 GB. `4f5fbcf49470`, the bootstrap left `rust-src` listed with no files.
