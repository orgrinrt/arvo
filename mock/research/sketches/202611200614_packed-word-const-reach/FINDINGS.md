# Whether a consumer with no feature line reaches the packed word in a constant

The second seat's read of the packed word (`../202611200520_packed-word/SEAT-2.md`) found two places where
its shape and the branch's differ: where a field's position comes from, and whether the accessors are
const for a consumer who enables no feature. This sketch settles both against the real `Bits`, `Bool` and
`notko-hlist`, on `nightly-2026-05-28`. Each file under `examples/` is a crate of its own with no
`#![feature]` line, so it sees what such a consumer sees.

**Outcome:** WORKS for the second seat's shape. The branch's shape `FAILS WITH E0658`.

## Const reach

`examples/no_gates.rs` uses the shape at the head of the branch (`Field` naming its layout and its
`BELOW`, the accessors bounded `[const] Lane`), and writes and reads the handle's fields in constants.
It is refused three times, once per call (`outputs/before.txt`):

```
error[E0658]: cannot call conditionally-const method `Word::<Handle>::get::<Kind>` in constants
error[E0658]: cannot call conditionally-const method `Word::<Handle>::with::<Flag>` in constants
error[E0658]: cannot call conditionally-const method `Word::<Handle>::with::<Kind>` in constants
```

That is `obligation::the_unstable_machinery_does_not_reach_a_consumer` failing in its own words. The
earlier sketch did not see it, because its examples carried `#![feature(const_trait_impl)]`.

`src/lib.rs` takes the second seat's repair: `Field::Value` and `Layout::Container` are bound
`const Lane` on the declaring traits, and the accessors carry no const bound and are plain `const fn`s.
`examples/derived.rs`, with no feature line, writes the handle's three fields in a constant, reads one
back in another, and runs (`outputs/derived.txt`, exit 0).

The cost lands on one party: a consumer whose own type is a field's value writes a `const impl` of
`Lane`, which needs the gate in that consumer's crate. A consumer whose fields are `Bits` and `Bool`
needs nothing.

## Positions

The branch's `Field` declares its `BELOW` and names its layout. The second seat derives each position
from the list order, low bit first, and a field names only its value. The widths in order are the
layout's whole content, and the offsets are their prefix sums, so declaring both stores one fact twice.
The branch then needs four checks to keep the copies honest (no overlap, the sum, the cover, the
membership by place and width), and the second review of the branch found three of them untested and the
fourth admitting an alias of another value type and a field of another layout.

Derived, overlap and a field past its container cannot be written, and one check remains: the widths sum
to exactly the container's width, which is also what catches a dropped or resized field moving the ones
above it. Membership is the list's own, by type: `notko-hlist`'s `PlaceOf` finds the field's place with
the place inferred, so the consumer writes `word.get(Kind)`. Since a field no longer names its layout,
`Flag` sits in `Handle` at bit thirty-one and in `Small` at bit zero (`examples/derived.rs`).

The second seat's call 2 names what would argue for `BELOW`: a real layout whose gaps make the reserved
fields outnumber the named ones. Hilavitkutin's handle has no gap, and its wide shape has none either.
`BELOW` does not earn its place.

## The refusals

`examples/refused_unlisted.rs` reads a field the layout does not list, and is refused at type check
(`outputs/refused_unlisted.txt`):

```
error[E0277]: `Cons<Flag, Cons<Rest, notko_hlist::Empty>>` has no place for `Other`
```

`examples/refused_short.rs` declares fields summing to seven bits over an eight-bit container, and is
refused at build (`outputs/refused_short.txt`):

```
error[E0080]: evaluation panicked: the field widths of a packed word do not sum to the width of its container
```

## What this does not show

No law is tested over the matrix of widths, and nothing here is timed or disassembled. The width of a
consumer's own `Lane` past sixty-four, or of zero, is not tried: `Bits` refuses those widths itself, and
an open `Lane` needs its own assertion, which the design round adds. A field listed twice is refused at
the access as an ambiguous place, as the second seat reports; it is not tried here.
