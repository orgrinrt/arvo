# The value's add, read in the emitted assembly

Outcome: WORKS. An ad hoc reading of one build, not a measurement and not a harness artifact.

The design of `arvo-numeral` owes the disassembly of `add` at 13 bits in a 16-bit carrier and at the
full width of 64 bits beside the host's own add. The bench harness cannot commit the disassembly of a
named function yet, which is tracked as the ticket
`bench_harness_disassembly_passes_a_flag_gnu_objdump_rejects`, so this directory holds the reading
instead and says what it is. `src/lib.rs` exports each probe under a plain symbol. It was read with
`cargo rustc --release --lib -- --emit asm` on the toolchain in `outputs/toolchain.txt`, `release`
profile, x86_64 with the default target features, and the probes were cut out of the `.s` file with
the assembler directives dropped.

## What the reading shows

`outputs/asm-release-x86_64.txt` is the reading of the tree as committed. At the full width the value's
add is the host's: `u64_wrapping` is emitted as an alias of `host_u64_wrapping_add`, a single `lea`, and
`i64_saturating` as an alias of `host_i64_saturating_add`, the overflow-flag select the host's
`saturating_add` lowers to. The two compile to the same bytes, so nothing of the dispatch, the bit copy
or the policy match survives.

At 13 bits in 16 the add is the machine add and the adaptation onto the width, and nothing else.
`u13_wrapping` is an add and a mask. `i13_wrapping` is an add and the shift pair that sign-extends from
bit twelve. `u13_saturating` is an add, one compare against 8191 and a conditional move.
`i13_saturating` is an add and the two clamps, each a compare and a conditional move. No branch, no
call, no overflow test on the carrier: below the carrier width two members cannot leave it, so the
machine's wrapping add is the exact sum and only the adaptation remains.

## The reading that found a cost

`outputs/asm-release-x86_64-width-read-at-run-time.txt` is the first reading, of the source before it
read its two dispatch keys as constants. There every probe of the value carried calls through the GOT
to `arvo_format::Width::count` and `arvo_format::Bool::get`, 24 calls in all, because those accessors
in `arvo-format` are not `#[inline]` and were not inlined across the crate. The source now reads the
declared width, the carrier's signedness and the policy's assertion flag inside inline `const` blocks,
which the compiler evaluates at build time, and the rung functions take the width as a count. The
accessors themselves are `arvo-format`'s, and whether they carry `#[inline]` is that crate's to decide, filed as
`arvo_format_mark_the_const_accessors_inline_so_consumers_fol`.

## What it does not show

Anything about time. The reading says which instructions a build emits, for one target and one set of
target features, and the claim it supports is that the value's add at those widths is the instructions
named above. A claim that it is as fast as the host's add would need the bench harness.
