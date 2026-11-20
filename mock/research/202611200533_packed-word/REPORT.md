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

So placement and shape were the canon's silence, and went to stack-coord for a stamp by two independent seats on 20 November, with the sketch. Stack-coord dispatched a blind second seat and told the job to proceed on its shape. The four calls put to it: the word in `arvo-bits` beside the tier, `tier.rs` untouched; the marker-type shape of `Layout`, `Field` with `BELOW` and `Word<L>`; the one-bit field as `arvo_format::Bool`; and `Lane` as the one contract of container and field value.

The second seat's read is committed as `mock/research/sketches/202611200520_packed-word/SEAT-2.md`. The two seats agreed on placement, on one contract for the container and every field value, on `Bool` for a one-bit field with `Bits<1>` kept for a one-bit code, on `const` reads and writes and on nothing checked at run time, and stack-coord holds that as a provisional stamp and files the question ticket to op. They differed on where a field's position comes from and on whether the accessors are const for a consumer with no feature line, and stack-coord gave both to this job to settle by argument and a committed sketch. `mock/research/sketches/202611200614_packed-word-const-reach/` settles both for the second seat's shape, and round `202611200616-o8z9` carries it, as the section on that round says. The row was kept: it quotes hilavitkutin's own words, which is what the second seat asked of a recorded need.

## The round

Round `202611200533-wwxt`, on `work/arvo-packed-word`, cut from `work/arvo-bits-tier` at `1fd4d78e` because the tier has not landed on `dev` and the word builds on it (stack-coord agreed, 20 November). The sketch `mock/research/sketches/202611200520_packed-word/` came first and records two refusals on the way, a `[const]` bound on an inherent impl and a derive bounding equality on the layout marker. The obligation row `a_word_divided_into_named_fields_of_declared_widths` and the design went through the doc changelist; `src/word.rs`, the tests and the bench through the src changelist.

## Tests

After round `202611200616-o8z9`, `cargo mock test -p arvo-bits --cheap` is green on `nightly-2026-05-28`. There are eleven law tests in `the_packed_word`, over the 32-bit handle, its wide twin, two 64-bit words, every container width from one to sixty-four, and one field listed in two layouts. `the_packed_word_with_no_gates` is a test crate with no feature line that writes and reads the handle in constants. `compile_fail` has twelve cases of the word among its own: eleven refused and one that must build.

Two mutant flags in `src/word.rs` were run against the law tests in `mock/research/sketches/202611200603_packed-word-mutants/`. A read one bit up turns nine of the eleven red, and a write that keeps the field's old bits turns five red. The first round's two hand mutants were run and undone without a record, so they are not cited.

The new `trybuild` fixtures were first written in a container whose `rust-src` was recorded as installed with no files, so rustc printed no source lines for `core` and every fixture of the tier mismatched as well. The tier's 37 were put back from `HEAD` untouched, `rust-src` reinstalled, and only the seven new ones blessed (ticket `4f5fbcf49470`).

## What it lowers to

`cargo mock bench disasm packed-word <arm> <symbol>` over the bench `mock/benches/packed-word/`, both arms in release as the harness builds them, `x86_64` in this container (`linux-x86_64-c4-container-intel`), `nightly-2026-05-28`, each an `extern "C"` function taking and returning the host's `u32`. The records cited are under `mock/benches/results/packed-word/disasm/`, each from a clean tree: the runs stamped `774a9ced` are from the source of round `202611200556-vw98`, and those stamped `cf70c6a1` are from the source of round `202611200616-o8z9`. Both rounds' `Word` arm lowers to the same instructions, so the read's added mask compiles away. The first round's runs, stamped `192d5e77-dirty`, stay beside them and are not cited, since nothing shows which source they built.

| Operation | `packed-word-host`, by hand | `packed-word-word`, through `Word` |
|---|---|---|
| round trip `of` then `bits` | `mov`, `ret` | the same |
| the 3-bit kind read | `mov`, `shr $28`, `and $7`, `ret` | the same |
| the 28-bit id written | `and $0xf0000000`, `and $0xfffffff`, `lea`, `ret` | `and $0xfffffff`, `and $0xf0000000`, `lea`, `ret` |
| the one-bit flag read | `mov`, `shr $31`, `ret` | the same |

Three are the same instruction for instruction. The id write is the same three instructions with the two independent `and`s in the other order and `lea`'s operands swapped. Nothing is claimed for another target, another width or a debug build, and nothing was timed.

## The first review, and the round that answered it

A senior review of the branch at `0c8ffe54` returned merge after fixes. Round `202611200556-vw98` answered it, finding by finding.

`Fields` and `Holds` were public and open, so a consumer's own list could claim any sum, union and overlap and switch every refusal off. Both are now sealed by a private supertrait, nameable in a bound and implemented for `()` and `(H, T)` alone, and the design names them. Making them unnameable was tried first and undone in the round, because a function generic over a field could then not write the bound `get` and `with` need.

`Holds` matches a field by place and width, which the design had described as refusing every unlisted marker. The design now says a marker at a listed field's place and width is that field under a second name, and a test pins it reading and writing the same bits. Matching by marker type would need a type identity the crate cannot compare without an index the caller names.

A width of zero or past sixty-four, and a field reaching past bit sixty-four, are refused by assertions of their own before any shift, with cases for a field of zero bits and a field past bit sixty-four; before, they failed as an overflowing shift. A write now masks what `low` gives to the field's bits. The laws are tested over two 64-bit words as well as the 32-bit ones. The mutants and the disassembly are as above.

Two commit subjects of the first round, `adf89a0a` and `20d58a91`, name `arvo-bits` without backticks; they are pushed and stay as they are.

## The second seat and the second review, and the round that answered them

Round `202611200616-o8z9` replaced the declared positions with derived ones, as the second seat proposed. A field names only `Value`. A layout names its container and a `notko_hlist` list of field markers from the low bit up. A field's position is the sum of the widths listed before it.

`Field::BELOW`, `Holds`, `COVER` and `OVERLAP` were there to keep declared positions agreeing with the widths, and they went with them. Two fields sharing a bit, and a field past its container, can no longer be written. Membership is `notko_hlist::PlaceOf`, by type. A field's own width and the container's are asserted to be one to sixty-four, because `Lane` is open. The one remaining check is the widths summing to exactly the container's width.

The const bound moved off the accessors onto `Field::Value` and `Layout::Container`. The sketch records the accessors refused with `E0658` from a crate with no feature line before the change, and building after it.

A second senior review of the branch at `3bec4838`, pass two of three, returned merge after fixes. The round answers its findings as follows:

- The untested refusals of overlap, cover and a field past bit sixty-four are no longer states the shape can hold.
- The width and container refusals each have their own case, at zero and at sixty-five.
- The laws run over every container width.
- The alias of another value type is gone with `BELOW`, because membership is by type. A field listed in two layouts is now intended rather than refused: it sits at its own place in each, and a test pins it.
- `get` masks what it moves down.
- `README.md` and `mock/DESIGN.md.tmpl` name the word.
- The obligation row cites hilavitkutin's items rather than their lines.

The review's own probes in the scratchpad were never committed, and are not cited here.

## The third review, and what was left

A third senior review of the branch at `329f787e`, the last of three, returned merge after fixes with two blockers. `docs/BITS.md` and `docs/DESIGN.md` had not been regenerated after the templates changed, and now are. The test that compared `Lane` for `Bits` and `Bool` against the expressions of its own impl bodies is gone: the width matrix now states the contract over raw values at every width from one to sixty-four, a test states it for `Bool`, and a consumer `Lane` four bits wide that keeps and hands back every bit shows the masks in `get` and `with` keeping it inside its field. Taking either mask out turned that test red and nothing else, run by hand and undone; it is not a mutant flag yet.

What the review raised and the branch leaves as follow-ups, each ticketed below:

- An open container `Lane` whose `from_low` keeps bits above its declared `WIDTH` lets `with` carry them into the stored word, so two words reading equal at every field compare unequal. Either `with` and `of` mask to the container's width, or the design states that tiling holds over an honest `Lane::WIDTH`; neither is decided here, and the `Lane` laws are prose on the trait with no check.
- The design's sentence that a consumer implementing `Lane` for its own value type needs the gate in its own crate meets `obligation::the_unstable_machinery_does_not_reach_a_consumer`, and whether implementing a trait counts as naming it is the canon's call, not this job's.
- A field listed twice is pinned at `get` only, not at `with`, and the no-gates crate does not call `get` from generic code.
- Round `202611200616-o8z9` replaced the outputs of the mutant sketch in place instead of superseding the sketch.

## Problems ticketed

`ac412c811f0d`, the stand-up could not render arvo's agent files from a clone scoped to kaski. `3457a32e273c`, `cargo mock check` in arvo built every member's tests and filled the disk at 40.6 GB. `4f5fbcf49470`, the bootstrap left `rust-src` listed with no files. `ea6b50a08352`, the third review's follow-ups. `87985acb423f`, the question to op on whether implementing an arvo trait counts as naming its types.
