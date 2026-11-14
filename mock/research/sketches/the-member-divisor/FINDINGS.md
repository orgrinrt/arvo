# The member divisor's doors, refused at build

**Date:** 2026-11-14
**Outcome:** WORKS
**Toolchain:** `nightly-2026-05-28`, the repository's pinned one, named with `cargo +nightly-2026-05-28`.
**Run:** `cargo +nightly-2026-05-28 build` here, then once with each feature; the output is in `out.txt`.

The question was whether a member type with a const door, and a divisor door taking a member of a second
format, can be refused at build, with no check on the value, on the pinned toolchain and on stable language
features alone. The sketch depends on nothing. Its formats are toy traits carrying what `arvo-format`'s
verdicts read, the radix, a constant exponent, the reduced phase and the slot range, since the crate's own
verdicts are private and a sketch reading only its public surface could not state the obligation.

`Member<F>` holds a private slot. Its const door, `constant::<B>()`, reads an associated constant that
asserts `B` is in `F`'s range, the pattern `Divisor::constant` already uses. `Divisor::<F>::member::<G>`
reads an associated constant over the pair `(F, G)` that asserts the same radix, the same exponent, the same
reduced phase, that `G`'s range lies inside `F`'s, and that no member of `G` names the zero: the phase is whole and its zero slot `-n` is in
`G`'s range, or it is not.

The default build admits four cases at build time in a `const`: a member at the top of the byte range, a
byte divisor from a member of slots 1 to 255, the same format on both sides, and a phased byte range, which
holds no zero at any slot. Each feature plants one construction, and each fails to build with error
`E0080` naming its own assertion: a member constant one past the top, a divisor from the byte range itself,
which holds the zero, a divisor from another exponent, another phase and another radix, and a divisor from
the slots 1 to 65535, a range past the byte's.

What this does not show: the adapted door, which is `adapt` and asks nothing new of the language, and
anything about cost. Whether the pair obligation stays cheap to compile across many format pairs was not
measured.

Superseded in one respect, 2026-11-14: the sketch's door reads the radix and the exponent apart, so it
refuses radix ten at exponent zero beside the bytes, where every quantum is one and the slots name the same
values. The shipped door compares one quantum, `U_F V_G = U_G V_F`, and admits that pair (round
`202611140603`). What the sketch shows, that the door can be built as a const obligation over two formats,
does not rest on its radix rule.
