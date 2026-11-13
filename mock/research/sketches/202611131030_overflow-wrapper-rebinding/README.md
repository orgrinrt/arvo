# Whether a wrapper can re-bind a declared signature's overflow coordinate

Op floated `Saturating<Foo>` and `Wrapping<Foo>` as the mechanism for overriding
an overflow default per case, on 13 November 2026. His words are quoted whole in
`mock/research/202611130950_canon-double-check/OPS-WORDS.md`.

In the tree the overflow policy is not a property of the value type. It sits
inside the declared signature, under the adaptation, and the value type `Num<S,
C>` is keyed on that signature. So the wrapper cannot be a newtype holding a
value of some other type. It has to be a type-level function from one signature
to another, substituting the overflow coordinate and keeping the rest.

Whether that function is writable, and whether its result is still admitted, is a
compile question. Nothing about it can be settled by reading.

## Hypothesis

Four things, each a separate way the shape could fail.

The substitution is expressible with ordinary associated types and no feature
gate. The projected signature still satisfies `Sole`, whose blanket impl asserts
over the slot family, the policy, the rounding mode, the quantum and the phase. A
function generic in the starting signature can do the re-binding, which is where
a normalisation failure would show. And a consumer can write the crossing from
the public surface.

The third was expected to fail and did not. The fourth was expected to be
trivially true and is the one that failed.

## Outcome

`WORKS` on the first three, `FAILS` on the fourth. `out/run.txt` is the output of
`cargo run --release` in this directory.

The substitution is one type alias over the two halves of the signature, with no
feature gate and no trait of its own, because every piece of it is already an
associated type. The projected signature is admitted: `Sole`'s blanket impl
discharges for it exactly as for a written one, which follows from the
substitution leaving the format alone and `Sole` asserting nothing about the
overflow policy beyond it being wrapping or saturation.

Generically it goes, with one bound the caller has to write out:
`WithOverflow<S, O>: Sole`. The projection normalises on its own, since the
`DeclaredSignature` impl on `Signature` is concrete, but normalising is not
proving the admission. `S: Sole` says nothing about any sibling signature, not
even one sharing its format, so the obligation does not come along for free. That
is a real ergonomic cost and it is the thing to design against: a wrapper
appearing in a consumer's signature drags a bound with it.

What does not work is the crossing itself. `Num::carrier` and
`Num::from_carrier` are `pub(crate)`, and no public door takes a value of one
signature and gives one of another. The public doors are `constant`, which builds
from a literal, `adapted`, which runs the target's adaptation over an exact
position, and `slot`, which reads the coordinate out. So today a consumer can
name `Saturating<S>` as a type and cannot move a value into it, and the wrapper
mechanism needs a door that does not exist yet.

That door is a design question rather than a compile one. What it is worth saying
here is what it cannot be: a free bit copy is correct only because the
substitution leaves the format and therefore the carrier alone, which is a fact
about this particular substitution and not about re-binding in general. A door
spelled as a general signature change cannot be a copy.
