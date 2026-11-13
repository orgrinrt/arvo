# An alias that names its adaptation today and takes a strategy later

Outcome: WORKS, on the toolchain in `outputs/toolchain.txt`, with no feature gate.

The question is how to spell `Uint`, while the `Warm` cell for integers is unclaimed, so that a
strategy parameter defaulting to `Warm` can be added later without breaking any consumer written
against the alias today.

## The shape

The alias reads its parameter through a projection rather than taking it as the adaptation:

    pub trait NamesAdaptation { type Adaptation: Adaptation; }
    impl<R: Rounding, O: Overflow> NamesAdaptation for Adapt<R, O> { type Adaptation = Self; }
    pub type Uint<const BITS: u32, A, C = Abort> =
        Num<Signature<UFixed<BITS, 0>, <A as NamesAdaptation>::Adaptation>, C>;

`value-now` is that alias. `value-later` is the same alias with `A = Warm` as its default and one more
impl, `NamesAdaptation for Warm`, whose adaptation is `<Warm as Strategy>::Adaptation`. The impl is
for `Adapt<R, O>` rather than a blanket over every `Adaptation`, so the impl for a strategy marker
overlaps nothing and may sit in whichever crate the dependency direction allows. Here the value
crate depends on `arvo-strategy` and writes it.

`C = Abort` is the debug check of `202611130550-m67x_the-debug-check-axis`, after the adaptation, so
both defaults trail and a consumer naming only the adaptation, or nothing, still spells a type.

## What each run shows

`consumer/src/written_today.rs` is written against today's alias and is never edited: a concrete
`Uint<8, Adapt<TowardZero, Wrap>>` and a function generic over `A: NamesAdaptation`. The binary `now`
includes it against `value-now` and the binary `later` includes it against `value-later`; both build
and run (`outputs/now.txt`, `outputs/later.txt`). `later` adds `Uint<8>` and `Uint<8, Warm>`, which
reach the default and name the strategy.

## The case that must fail

`consumer/examples/without_the_projection_today.rs` is the consumer a projection-free alias would have
produced, generic over `A: Adaptation`. Against the later alias it is refused with E0277, `A:
NamesAdaptation` not satisfied (`outputs/without_the_projection_today.txt`). So the projection has to
ship now: added later, it is the break.

## What it does not show

Which crate the alias and the projection end up in once the cell is claimed. Only the direction where
the value crate depends on `arvo-strategy` is built here; the other, where `arvo-strategy` writes the
impl beside its marker, is not tried. Nor anything about `Warm`'s adaptation itself, which this probe reads and
does not decide.
