# A value whose carrier no consumer can name

Outcome: WORKS, on the toolchain in `outputs/toolchain.txt`, with no feature gate.

The question is whether `Num<S>` can store the machine carrier the placement derives while a crate that
depends on the value crate cannot name that carrier's type by any route: not through `Sole`, not
through the carrier map, not through the trait the carriers implement.

## The encoding

The carrier map `Carried`, with its `type Repr`, and the trait the carriers implement, `Machine`, are
`pub` traits in a module the crate does not export, which is the sealed-trait shape. `Sole` declares no
associated type. It names the map only through an associated type bound in its own supertrait list:

    pub trait Sole: DeclaredSignature<Format: Format<Slots: sealed::Carried>> {}

The compiler elaborates a bound in that position wherever `S: Sole` holds, so `Num<S: Sole>` can store
`<<S::Format as Format>::Slots as Carried>::Repr`, and every `impl<S: Sole>` and every consumer generic
over `S: Sole` sees `Num<S>` as well formed without naming `Carried`. `cargo check --lib` reports
nothing from this crate, so the bound draws no `private_bounds` warning.

## What each example shows

Each file under `examples/` is a crate of its own, so it sees only what the library exports.

`a_generic_consumer` builds and runs: a function generic over `S: Sole` takes and returns `Num<S>`,
and `size_of::<Num<S13>>()` is 2, the carrier's size, observed through the host's layout and nothing
else.

`the_carrier_by_projection` is refused with E0220: `S::Repr` with `S: Sole` is not found, because a
bound in that position is not a supertrait with associated items to project. The compiler's note names
the sealed trait and suggests bounding by it, which a consumer cannot do.

`the_carrier_by_a_fully_qualified_projection` is refused with E0576: `Sole` has no `Repr`.

`the_carrier_by_path` is refused with E0603: the module holding the map is private.

The output of each run is under `outputs/`, with the warning `arvo-format` prints about its own dead
code removed.

## The one thing found on the way

A derived `Clone` and `Copy` on `Num<S>` bound `S: Copy`, so a generic consumer moving a value twice
failed with E0382. That is the derive's bound and not the encoding: written by hand over `S: Sole`, as
the value crate already writes them, the consumer builds.

## What it does not show

It is one probe with two carriers and one signature. It does not show that the full map of 128 rows,
the dispatch over the carrier's size and signedness, or the diagnostics on the sealed traits behave the
same; the value crate's own build and its compile-fail tests are what show that.
