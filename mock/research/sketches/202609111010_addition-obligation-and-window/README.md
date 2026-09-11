# Sketch: the shapes admitted addition rests on

Cited by `mock/design_rounds/202609111010_topic.addition-admitted-on-a-declared-signature.md` and by
the doc changelist of the same round.

## Hypotheses

Four questions, each of which the design would otherwise be asserting without having asked rustc or
the map.

H1. An admission obligation carried as an inherent associated const on a generic helper struct, and
forced inside a generic `const fn` with `let () = ...;`, refuses a magnitude-indexed format. A runtime
call reaches it at `cargo build` and not at `cargo check`; a call bound in a `const` item reaches it at
`cargo check`. That is the same split the crate already documents for `Slots::ADMITTED`, and it decides
whether a `trybuild` case can pin the refusal.

H2. A slot range over two const generic bounds, `Window<LO, HI>`, implements the open `Slots` trait
with its width derived inside the associated const by a `const fn`, with no `generic_const_exprs`, and
meets the slot range's own obligation. A test-local family of such windows is what lets the absorption
predicate be checked against brute force over ranges no shipped point declares.

H3. The exact step for addition, derived from the denotation `value = (phase + slot) * quantum`, is the
position `a + b + phase` in slot units, independent of the quantum and the radix. Composed with the
crate's own `adapt`, it reproduces the canon's witnesses: 952 divergent triples for signed saturating
addition at four bits (`law::additive_associativity_under_saturation`), zero for wrapping and zero for
unsigned saturation, and 4,177,792 at eight bits.

H4. The carry obligation, that the exact sum of two members stays inside the slot coordinate, is
reachable by an admitted format, so it is a refusal and not dead text.

## Code

`main.rs`, with `Cargo.toml` depending on the worktree's `arvo-format` by path and three features that
each plant one construction that must be refused. The toolchain file is the repository's pin.

## Outcome

`output.txt` holds the full run. `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.

H1 WORKS. `cargo build --features refuse_indexed_at_build` exits 101 with `E0080`, "addition over a
magnitude-indexed quantum is refused", at the forcing site. `cargo check` with the same feature exits 0,
which is the codegen-only half and the case that had to come out that way for the split to be real.
`cargo check --features refuse_indexed_at_check` exits 101 with the same diagnostic.

H2 WORKS. `Window<-4, 0>` derives width 3 and its sweeps run, so the obligation passed.

H3 WORKS. `Integer<4>` saturate 952, wrap 0, `UFixed<4, 0>` and `UFixed<4, -3>` saturate 0,
`Biased<4, -2, 0>` saturate 952 (fraction width 2, whole phase zero, the same count as fraction width
0), `Integer<8>` saturate 4,177,792. The half-step phase puts `1 + 2` at slot 3 on a tie, which is the
exact sum `3.5` in slot units.

Two windows came out as a side result and they are the reason the round builds the window family into
its tests. `Window<-4, 0>`, a signed range, is associative under saturation (0 divergent), and
`Window<-1, 7>` is not (70). A rule keyed on signedness gets the first wrong, and the absorption
predicate, which reads the range and the reach, has to get both right.

H4 WORKS, and only through an outside format. `FarPhase`, a phase of `i64::MAX` whole quanta over an
eight-bit signed range, exits 101 at `cargo check` with "the exact sum of two members leaves the slot
coordinate". The control prints that the widest phase a shipped point can declare,
`Biased<62, 0, i64::MAX>` and `Biased<62, 0, i64::MIN>`, is still carried: `Phase::halves` halves the
count, so no shipped point reaches the refusal.

The eight-bit timing in `output.txt` is an ad-hoc spike with no substance, a debug build and one run,
printed only so a reader knows why the eight-bit law sits behind `--ignored` in the crate.

## What must fail

The three refusal arms must exit nonzero with the named `E0080` message, and the `check` run of the
build-only arm must exit zero. If the last one ever refuses, the obligation has moved to a place `check`
evaluates and the `trybuild` shape the round chose is no longer the only one that can see it. If the
952 moves, the exact step is wrong.

## Next step

Unblocks the doc changelist: the obligation shape, the carry refusal as a `trybuild` case over an
outside format rather than a shipped one, and the window family as the test instrument for the
absorption predicate.

The unused-import warning in the default build is `Signed`, used only under the carry feature.
