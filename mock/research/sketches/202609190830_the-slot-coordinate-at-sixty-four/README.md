# Sketch: which slot coordinate reaches a 64-bit platform width

Hypothesis. `ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family` puts a
platform-width type's slot range at `0 ..= 2^W - 1`, or its two's complement twin, with `W` the target's
pointer width, and names three repairs for the shipped coordinate that stops at 62: widen the host type
of a slot index, restate the range as a count, or restate it with an exclusive bound. The hypothesis is
that only the first reaches `W = 64`, because the other two keep a signed 64-bit index and the endpoints
of both 64-bit ranges are not all signed 64-bit values.

Cited by `mock/design_rounds/202609190823_topic.platform-width-types-on-a-wider-slot-coordinate.md`.

## Outcome

WORKS for option Y, FAILS for option Z, in `output.txt`.

Option Z, in `z_an_i64_index_restated.rs`, keeps the index in `i64` and carries the count in `u64`. Both
restatements are refused at 64 in both signednesses, by `literal out of range`: a signed 64-bit range
counts `2^64` slots and so does an unsigned one, one past `u64::MAX`; the exclusive end of an unsigned
64-bit range is `2^64` and of a signed one is `2^63`, neither of them a signed 64-bit value; and the
inclusive top of an unsigned 64-bit range, `2^64 - 1`, is not one either, so the shipped inclusive pair
cannot carry it whatever else changes. The controls build: both restatements reach 63, which is one
width past the shipped 62 and one short of the target.

Option Y, in `y_an_i128_index.rs`, carries the index in `i128` and keeps the impl ladder as the bound. It
builds at every width 1 through 64 at five targets across two pointer widths, with a compile-time
assertion per width that the count is exactly `2^width` in both signednesses, and at the running target's
own width the ends equal `usize::MAX`, `isize::MIN` and `isize::MAX`. Adding width 65 is refused at the
definition site, because the ends are written as shifts of the 64-bit host integers and `64 - 65` is a
shift the compiler refuses, so the ladder stays the bound and the carrier's own headroom does not leak
into it.

## Where each wins

Z wins nowhere the canon asks for. It buys the one width 63, and the obligation it would serve is at 64.

Y wins at every pointer width a Rust target has. Two members of a 64-bit range sum to at most
`2^65 - 2`, which leaves more than sixty bits of the carrier above every position addition produces, so
the intermediates that were computed one domain wider than an `i64` keep an exact answer without a
domain wider than `i128`.

Two shapes were argued and not built. An index stored as an offset from the range's lowest slot reaches
64 in a 64-bit carrier, and costs the absolute reading of a slot that the phase and the additive identity
are stated in, since slot zero would stop meaning the same position in two ranges. And the ladder could
run on past 64 in the same carrier, to 126, where the count stops fitting; that is a statement about how
wide arvo goes, which nothing settled asks for and which the placement ladder, stopping at a 64-bit
carrier, has no carrier for.

## What must fail

`past_the_ladder` must refuse, and it does. If it built, the bound would be the carrier and not the
ladder. The four Z arms must refuse; if any built, the hypothesis that the index type is what stops 64
would be wrong. Every refused arm has a control in the same file differing in one declaration.

## Next step

Unblocks the design round that widens `arvo-format`'s `Slot` and `SlotCount` to `i128` and admits the
ladder to 64, which is what the platform-width points need.

## Toolchain

The repository's pinned nightly, copied in as `rust-toolchain.toml`, and `run.sh` records the version it
ran under at the top of `output.txt`.
