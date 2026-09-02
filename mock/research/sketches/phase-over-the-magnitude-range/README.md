# The additive identity over the magnitude range

Two ad-hoc quick spikes. Neither is a bench, nothing here is timed, and nothing
here decides a fork on cost. They establish existence claims, refute one
universal, and answer three yes-or-no questions about what the toolchain accepts.

## The question

`has_additive_identity` reads the phase coordinates and the slot range. It reads
no coordinate of the quantum at all, so it answers at magnitude zero while its
name and the design's sentence quantify over the format's whole set.

The design states membership at `mock/crates/arvo-format/DESIGN.md.tmpl:29`: a
value `v` is representable exactly when there is a magnitude `m` and a slot `i`
within the slot range such that `v = phase + i * quantum(m)`. Zero is therefore
in the set exactly when some admitted pair cancels the phase, and the existential
runs over both coordinates.

## `enumerate_against_the_predicate.rs`

Compares the shipped function, transcribed verbatim, against an enumerator over
the real grid that never mentions it. Output in `enumerate_output.txt`.

The cancellation equation is solved in exact integers. From
`(PN/PD) * r^BASE + i * r^(BASE + SLOPE*m) = 0`, dividing by `r^BASE` and
clearing the denominator gives

```
i * PD * r^(SLOPE*m) = -PN
```

so `BASE` leaves the problem entirely and no rational arithmetic is needed. Every
multiplication is checked, so a `false` is always "checked and not equal" rather
than "the arithmetic gave up".

Three disagreements out of eight cases:

| case | shipped | actual | witness |
|---|---|---|---|
| `Indexed` + `Signed<2>`, phase 4 | false | true | slot -2 at magnitude 1 |
| shrinking quantum, phase 1/2 | false | true | slot -1 at magnitude 1 |
| no magnitudes, phase 0 | true | false | the set is empty |

Three controls, all passing. The enumerator separates a zero phase from a
half-step phase, so it is not stuck at one verdict. It agrees with the shipped
function on all five shipped points, so the disagreements are about the cases
under test rather than about the instrument. And moving `BASE` by seven moves no
answer, which is the derivation above checked rather than asserted.

Two of the three are under-reporting and one is over-reporting, and both
directions come from the same root: the existential over the magnitude was
dropped.

The first case needs no outside `Quantum`. `Indexed` is slope one and
`Signed<2>` is exactly that range, both shipped, so an outside `Format` reaches
it alone. The second needs an outside `Quantum` with a shrinking step, and it is
the one that refutes the design's own gloss: a **fractional** phase keeps the
identity there, because the quantum has halved by the magnitude that cancels it.

## `const_shape.rs`

Asks whether the widened shape is expressible. Output in
`const_shape_output.txt`, emitted assembly in `const_shape_asm.txt`.

A `const fn` carrying a bounded `while` loop, a `match` on `Option` and a forced
associated const compiles on the pinned toolchain with no feature gate, and
evaluates in a `const` binding, which is what makes it a compile-time predicate.
Its control cuts the magnitude range to one and both widened cases go back to
false, so the search is what found them.

It folds. `folds_half` is `mov w0, #0; ret` and `folds_becomes_whole` is
`mov w0, #1; ret`, and the other two are emitted as aliases of the latter
(`_folds_zero = _folds_becomes_whole`), which is the compiler reporting that all
three are the same constant function. No loop, no call and no branch survives, so
the widened predicate costs nothing at the lowering.

## What these do not establish

The transcription is faithful by inspection and is not the crate. The permanent
form of every check here is a test inside `arvo-format` against the real
functions, which is where they land; these two files are what established the
claims before the round opened.

Neither file measures anything, so nothing here carries a cost, a ratio or a
timing, and no sentence here should be cited as if it did.
