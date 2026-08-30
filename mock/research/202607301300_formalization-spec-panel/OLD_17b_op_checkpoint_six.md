# Op's sixth checkpoint, after the algebra dive's first four

**Date:** 2026-07-31
**Position in the panel:** after `17_orchard_are_these_all_grades.md`. **Required reading** with the
numbered files and every earlier checkpoint, especially `13c`, `16b`, `16c` and `16d`.

Three calls, and all three set work rather than closing it.

## Fidelity gets a witness

**Adopted.** File 17 demonstrated asymmetric enforcement: the effect side carries data, so two
deliberately corrupted implementations were refused by the compiler (`E0004`, `E0277`), while the
licence side carries permission, so a corrupted grant compiled clean and returned `0.0` where `2.0`
was asked for. Nothing in the type system can see it, because no value has the wrong type.

So a fidelity grant is checked rather than asserted, on the same footing as the recovery map that the
earlier verification thread ended up witnessing.

What that does **not** settle is the shape. A licence witness is not a port of the recovery-map
witness: there is no returned value to check a grant against, which is precisely why the corruption
went undetected. The recovery-map witness took three members to get right and each of the first two
had a hole the next found by compiling. Assume the same here.

## The build-layer contract: verify the dissolution, and design it anyway

Both, and op noted they are not mutually exclusive.

File 17 argued that nothing is owed to a build layer for fidelity: three earlier members imported a C
mental model where the compiler owns the operation, whereas arvo owns its own, so all four liberty
classes are source-expressible and the single residue closes through stable `core::arch`. It is a
strong claim that reverses three members, and this review's pattern is that such claims survive or
fall on whether someone compiles them. **Verify it before it is carried.**

And separately: **design the build-layer contract regardless.** Even if arvo can express the
liberties in source, a build layer reading intent out of the types may be the better mechanism for
other axes, and designing that once is worth more than proving one axis does not need it. The
obligation from `16c` stands for every boundary that is real; what file 17 may have shown is only
that this particular boundary is not one.

## Partial associativity is named, and the dive works it through

**Adopted.** File 17 measured that `Precise` has zero numeric spread across groupings: every grouping
that returns, returns the same number. Its regrouping sensitivity is entirely about which groupings
are defined at all. The law it needs is partial associativity, the design does not name it, and no
standard vocabulary carried in the spec covers it.

That is a genuine gap in the algebra vocabulary, found by measurement rather than argument, and this
dive is where it belongs. File 17's direction, Kleene equality fusing the numeric and definedness
readings, is the thread to pull.

## Standing

Only op's calls are final and even those go stale. The intent outranks every instruction in these
checkpoints, it is vague on purpose, and no member resolves to a single angle on anything
substantive. Where the current shape can be kept it should be, and rewrite cost is the tiebreaker
between designs otherwise equal against the intent.
