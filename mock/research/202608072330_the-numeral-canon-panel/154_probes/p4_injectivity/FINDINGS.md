# P4 findings: the index does not name the primitive, and where it fails to is not fixed

`python3 inj.py`, `python3 inj_container.py`, toolchain `nightly-2026-05-28`, host
aarch64-apple-darwin. Generators, generated sources, emitted assembly and outputs committed beside
this file.

**Ad-hoc quick spike with no substance as a measurement.** Every number here is a count of distinct
machine bodies, which is exact. Nothing is timed and nothing is priced.

## The controls

P4 predicted, before running, that `(13, wrap)` and `(47, wrap)` stay distinct and that
`(64, wrap)` and `(64, clamp)` collapse. **Both passed** (`inj.out:1-2`). P4b predicted that under a
minimal container `W = 8` collapses and `W = 7` does not. **Both passed** (`inj_container.out:1-2`).

## F11. The index-to-primitive map is not injective

Over widths 1..64 and policies {wrap, clamp}, with a fixed u64 container:

```
index points in the grid : 128
distinct primitives      : 127
collapse classes         : 1  ->  [(64, 'clamp'), (64, 'wrap')]
```

At `W = 64` in a u64 the mask is all-ones, so masking is the identity and clamping to the maximum is
the identity. The two indices are one function. **This is a theorem, not a compiler artifact**: the
two source bodies denote the same mathematical function on `u64`, so no lowering could separate them.

Consequence for the canon: **"a primitive is a point of the index space" is false as a definition.**
It is a good working proxy, wrong at 1 point in 128 here, and being wrong there is not harmless: a
law proved of `UFixed<64,0,·>` under one overflow policy holds of the other **by coincidence of
denotation rather than by anything the law's proof mentions**, and a reader who takes the index as
the identity will read that as two independent confirmations.

`holds for:` W in 1..=64, policies in {wrap, clamp}, container = u64, unsigned, arity 1,
`nightly-2026-05-28`, target aarch64-apple-darwin, opt-level = 3, threads = 1, F = 0.

## F12. The degenerate set is not a property of the width. It moves with the container, which the treatment chooses.

```
wide container (always u64),        widths where wrap == clamp : [64]
minimal container (u8/u16/u32/u64), same                       : [8, 16, 32, 64]
degenerate index points: 1 of 128 (wide arm), 4 of 128 (minimal arm)
```

**So the identity criterion for primitives is treatment-relative.** Whether `(8, wrap)` and
`(8, clamp)` are one primitive or two is not answerable from the declared width; it depends on the
container, and the container is picked by the strategy. I6 (`INTENTS.md:123-124`) has one treatment
"aggressively minimises and bitpacks", which is the minimal arm, and I3/I4 have another behaving as
a native primitive would, which on a byte-addressed machine is the wide arm.

This is the sharpest thing I found and it generalises F5. F5 said the *representation* is a function
of the pair rather than of the format. F12 says the **identity relation on primitives** is too. Two
descriptions that name one primitive under one treatment name two under another, so "how many
primitives are there" has no treatment-free answer.

The predicate is nameable and const-checkable, which is what I13 asks for: **the overflow policy has
no content exactly where `declared_width == container_bits`.** Both arms satisfy it; they disagree
only about which widths meet it, because they disagree about `container_bits`.

`holds for:` W in 1..=64, policies in {wrap, clamp}, containers in {always-u64, minimal
u8/u16/u32/u64}, unsigned, arity 1, `nightly-2026-05-28`, target aarch64-apple-darwin, opt-level = 3,
threads = 1, F = 0.

## F13. A dimension that has no content at a point is exactly the test-gate's "declaration nothing constrains", one tier up

`the-test-gate.md` rejects "a constant a type declares about itself, that no code reads and no check
ties to the thing it describes... It reads as a contract and is a comment with a type", with the test
"ask what value would make it fail".

At `W = 64` in a u64, the overflow-policy parameter is that. Any value of it produces the same
machine body, so no observation distinguishes them. **The failure the test gate catches in a suite is
available one tier up, in the index of a primitive**, and F12 says where it bites is not fixed.

I state this as a criterion rather than a rule, because I cannot see the canon it would go into:
**a parameter earns its place in a primitive's index only where changing it changes something
observable at compile time, and the region where it does is part of what the primitive declares.**

`holds for:` as F12.

## What would refute each

F11: an observation distinguishing `(64, wrap)` from `(64, clamp)` on a u64. It would have to
distinguish two extensionally equal functions, so it needs an intensional observable, which under
`INTENTS.md:286-288` (no `dyn`, no `TypeId`, no `std::any`) arvo has removed.
F12: a container policy under which the degenerate set is empty or is width-only. Empty requires no
width ever to fill its container, which fails at the container's own width.
F13: a canon that wants the index to over-count deliberately, e.g. so that a later treatment can
split a currently-degenerate point. That is a real position and this finding does not close it; it
says the over-count has to be declared rather than accidental.
