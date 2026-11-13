# Whether a wrapper can re-bind a declared signature's overflow coordinate

Op floated `Saturating<Foo>` and `Wrapping<Foo>` as the mechanism for overriding
an overflow default per case, on 13 November 2026. His words are quoted whole in
`mock/research/202611130950_canon-double-check/OPS-WORDS.md`.

In the tree the overflow policy is not a property of the value type. It sits
inside the declared signature, under the adaptation, and the value type `Num<S,
C>` is keyed on that signature. So the wrapper cannot be a newtype holding a
value of some other type. It has to be a type-level function from one signature
to another, substituting the overflow coordinate and keeping the rest.

Whether that function is writable, whether its result is still admitted, and
whether a value can be moved through it from outside the crate are compile
questions. Nothing about them can be settled by reading.

## Hypothesis

Five things, each a separate way the shape could fail.

The substitution is expressible with ordinary associated types and no feature
gate, and it re-binds what it says. The projected signature is admitted by
`Sole`, the same as a written one. The re-binding goes generically, with the
signature held abstract. A consumer can write the crossing from the public
surface alone. MATLAB's documented `fimath` default is a signature the value
tier admits.

## Outcome

`WORKS` on all five. `out/run.txt` is the output of `cargo run --release` in
this directory, and `out/refused-stochastic.txt` and `out/refused-clamp.txt` are
the compiler's refusals of the two controls under `examples/`, from
`cargo build --release --example <name>`. All three were produced in the main clone
with the crates as they are at `326b52f5`, so the paths in them are that
clone's.

The substitution is one type alias over the two halves of the signature, with no
feature gate and no trait of its own, because every piece of it is already an
associated type. It re-binds the coordinate: slot 300, past an unsigned 8-bit
range, completes as 44 under the wrapper naming `Wrap` and as 255 under the one
naming `Saturate`, from the same written signature.

The projected signature is admitted, and the admission discriminates. The two
controls are signatures of the same shape that `Sole` refuses at compile time,
one for a rounding mode that reads a dither and one where the substitution is
handed `Clamp`, which the alias itself accepts because it is only a projection.
Each fails with the admission's own message.

Generically it goes, with one bound the caller has to write out:
`WithOverflow<S, O>: Sole`. The projection normalises on its own, since the
`DeclaredSignature` impl on `Signature` is concrete, but normalising is not
proving the admission. `S: Sole` says nothing about any sibling signature, not
even one sharing its format, so the obligation does not come along for free. That
is a cost a wrapper carries into every generic signature it appears in, and it
is the thing to design against.

The crossing is writable from outside the crate as
`Num::adapted(Exact::on_grid(value.slot()))`: the value's slot, that slot as an
exact position already on the grid, and the target signature's adaptation over
it. All three doors are public. The sketch moves 201 from `Wrap` to `Saturate`
and back and reads the same member each time, and the value that arrives adds
under the policy it crossed into, 255 and 1 giving 255 where the pair it came
from gives 0.

MATLAB's default is admitted. MathWorks documents the default `fimath` as
`RoundingMethod: Nearest` and `OverflowAction: Saturate`, and
`arvo-format::standards` maps Nearest onto `HalfUp`.
`Signature<Fi<16, 8>, Adapt<Nearest, Saturate>>` builds as a value type and
saturates at both ends of the signed 16-bit range. The reference page is stored
in the workspace's fetched store under the title "MathWorks fimath reference:
fixed-point math settings", fetched on 13 November 2026.

## What this leaves open

The rounding half of MATLAB's default. The check feeds a slot that is already on
the grid, so it shows the pair saturating at both ends and never exercises
`Nearest`. Rounding under that pair is what `tests/matlab_fi_parity.rs` in
`arvo-format` holds, and the sketch adds nothing to it.

Whether the crossing lowers to a move. Both signatures share a format, so the
position handed to the adaptation is on the target's grid and inside its range,
and nothing is rounded or completed. That argues the composition should cost
nothing, and it is an argument rather than a measurement: a sketch does not
answer a codegen question, and one that matters goes to the bench harness.

Whether a named door is wanted beside the composition. Three calls in a row is
writable and is not what a consumer would want to type at every site, and the
`WithOverflow<S, O>: Sole` bound travels with it. A named crossing between
sibling signatures is a design question about ergonomics, with a working
composition under it, and no longer a missing capability.

## What an earlier version of this file said

The first version reported `FAILS` on the crossing, on the ground that
`Num::carrier` and `Num::from_carrier` are crate-private. That was wrong. The
arm it rested on never attempted the crossing: it discarded its argument and
returned a constant. The same version named `Floor` as MATLAB's default
rounding, which is not what MathWorks documents, and its assertions compared a
constant with itself. This version replaces all three, and the documents beside
`OPS-WORDS.md` that repeated the earlier result are corrected in the same
change.
