# 134c. The bridge is consumer-extensible, so the cap was never forced

**Author:** the dispatching agent, not an expert. **This is one probe by one party and it has had no second
read.** It is recorded here rather than in chat because it falsifies a claim three files are now built on,
and a claim that load-bearing should not live in a transcript.
**Date:** 2026-08-07
**Position:** settles the first of the seven pushbacks in `134b`, which asked for exactly this check and
priced it at ten minutes.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, two-crate probe outside the
repository tree at `/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/e0117/`.

## What was claimed

File `134` established the fork's binding constraint from a coherence refusal:

> a consumer cannot populate a const-keyed bridge for its own widths, `E0117`, so the bridge must be a
> capped table inside arvo (`d14_consumer.rs`)

and then relocated op's blocking question onto it:

> `E0117` says the literal spelling and the absent cap cannot both be had structurally.

File `134b` pushed back, noting that `134` itself writes `<Idx<14> as ToNat<Marker>>::N` four hundred lines
earlier, and that coherence admits `impl ForeignTrait<Local> for Foreign`.

## What compiles

The pushback is correct. A bridge trait carrying a marker type parameter is populated by a downstream crate,
for widths arvo never listed, with no feature gate and no flag. Both crates exit 0.

Arvo declares the bridge, its own marker, and its own populated widths:

```rust
pub struct Idx<const N: u32>;
pub trait ToNat<M> { const VAL: u32; }
pub struct Arvo;
impl ToNat<Arvo> for Idx<3>  { const VAL: u32 = 3; }
impl ToNat<Arvo> for Idx<13> { const VAL: u32 = 13; }

pub struct Fixed<const I: u32, const F: u32, S, M = Arvo>(PhantomData<(S, M)>)
    where Idx<I>: ToNat<M>, Idx<F>: ToNat<M>;
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, S>;

pub fn widths<const I: u32, const F: u32, S, M>(_: &Fixed<I, F, S, M>) -> (u32, u32)
    where Idx<I>: ToNat<M>, Idx<F>: ToNat<M>
{ (<Idx<I> as ToNat<M>>::VAL, <Idx<F> as ToNat<M>>::VAL) }
```

A consumer brings its own marker and its own widths, and both surfaces work through arvo's own generic law:

```rust
pub struct Mine;
impl ToNat<Mine> for Idx<777> { const VAL: u32 = 777; }
impl ToNat<Mine> for Idx<41>  { const VAL: u32 = 41; }

pub fn arvo_width(x: &UFixed<13, 3, Warm>) -> (u32, u32) { widths(x) }
pub fn my_width(x: &Fixed<777, 41, Warm, Mine>) -> (u32, u32) { widths(x) }
```

Three things hold at once, which `134` reports as impossible: **D48's literal spelling is unchanged**, the
marker is defaulted so a consumer never writes it, and **there is no cap**, because a width arvo did not
list is added by whoever needs it rather than by raising a number in arvo.

## Why the earlier probe refused

`134`'s `d14_consumer.rs` implements a bridge with no marker parameter. Then the trait and the type are both
foreign to the consumer and `E0117` is correct. The marker is what makes the impl local, and it is the same
move the shipped tree already uses: `arvo-strategy`'s projection is keyed on a tag plus a strategy marker
rather than on a width alone.

The bridge is also **per written coordinate, not per sum**. A consumer writing `UFixed<13, 3, Warm>` needs
`Idx<13>` and `Idx<3>`, and the addition on top of them is structural and gate-free. So the population is
over the widths a program actually writes, which is op's own stated property at `127b:36-50`, reached
without the enumeration he refused.

## What this does and does not settle

**Settles:** the cap is not forced, and `134:462-469`'s relocation of op's blocking question onto `E0117`
does not stand. The fork's third route loses the cost that distinguished it.

**Does not settle:** whether a consumer extending a bridge is an acceptable thing to ask of one, what the
diagnostic looks like when they have not, whether arvo's shipped range is then a convenience rather than a
limit, and how the marker interacts with the container projection, the laws and the four families. Nor does
it touch the compile-cost or diagnostic comparisons in `134`, which stand.

**And it is one probe by the dispatcher.** Two things are wrong with that provenance and both matter: it is
a party with an interest in the panel converging, and it has no second read. Nothing here enters the canon
until an expert reproduces it and works the consequences. The immediate obligation is that no later file
repeats `134`'s claim as settled, because it is not.

## The pattern this is the sixth instance of

A refusal was compiled, reported accurately, and generalised one step too far. The probe answered "can a
consumer implement this trait for this type" and the conclusion drawn was "a consumer cannot extend the
bridge". Every file after it inherited the second sentence. `108b:22-32` names this and `134b` caught it by
reading `134` against itself rather than by running anything.
