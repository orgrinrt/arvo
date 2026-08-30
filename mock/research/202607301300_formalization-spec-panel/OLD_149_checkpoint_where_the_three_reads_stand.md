# Checkpoint: where the three reads of the conversion story stand

**Date:** 2026-08-07
**Author:** the dispatcher, not an expert. This records what agreement exists and what does not.
**Covers:** `145` (Dolan), `146` (Chlipala), `148` (SPJ), and the standing corrections at `145b` and `145c`.

Three files now answer the same four questions, and they do not agree. This states which answers
have the two independent agreements the workspace requires before a call is made, which have none,
and what op has to settle. Nothing here is adopted beyond what the agreements carry.

## Settled: the inclusion conditions

**Two experts independently agree that `145`'s two-condition order is wrong**, and each derived its
own before reading the other's. That is the standard, and it is met.

`145` gave inclusion as two conditions on integer and fraction width, over 2,025 ordered pairs.
`146` found four, adding phase alignment plus both endpoints, and produced a compiled counterexample:
a target on a strictly finer grid whose range strictly contains the source's, representing none of
its values. `148` derived the same four independently and explained why they are four, by giving the
value set as an affine lattice intersected with an interval, so inclusion is inclusion in each
factor.

`148` sharpens the verdict in a way that matters. `145`'s conditions are not incomplete, they are
**unsound**: 17,037 false positives over its own sweep, 6,549 of them sole phase failures and 5,229
sole floor failures. An unsound order admits conversions that lose values.

Both also explain why `145` could not have seen it: every numeral in its sweep has bias zero, so the
phase condition read as a triviality throughout. That is the sampling failure
`catalogue-edge-cases-as-tests.md` names, arriving in a design rather than a test suite.

**Adopted.** The order carries the grid, phase and both endpoint conditions. Whether the canon states
it as four conditions or as `148`'s two coordinates is a wording question for the consolidation, and
`148`'s form says why the four are four, which is the better reason to prefer it.

## Settled: `145`'s "no new key column" does not hold

`146` and `148` agree the schema does not currently say which of a conversion's two strategies
adjudicates, and both show `145` could not have detected it: all readings coincide exactly on the
embedding region where `145`'s three checks live. `146` measures 33 percent disagreement across
lossy conversions.

**Adopted as a negative only.** The gap is real. The remedy is not settled, see below.

## Not settled: the algebraic structure

Three files, three answers, no two agreeing. This is the clearest open question in the panel.

- `145`: a lattice, exact meets, joins overshooting.
- `146`: join-semilattice, the meet failing under bias at 663,026 of 1,016,064 pairs.
- `148`: **meet**-semilattice and **not** a join-semilattice, the exact inverse of `146`.

They agree on nothing except that the unbiased slice behaves better than the general case. `148`
gives a mechanism for its direction, that a covering numeral's window may take only the sizes its
radix and precision allow, so where none matches the hull no least cover exists, and it draws the
consequence that **a common target for two numerals must be a named rule rather than a derived
consequence**. That consequence is load-bearing for the canon if the direction is right.

**Not adopted.** A third read is owed, and it should be dispatched against the mechanism rather than
against the counts, since counts from three differing models cannot adjudicate between the models.

## Not settled: what the sign domain is

`146` and `148` agree it is not a partition, which refutes `145`'s treatment. They disagree on what
it is: `146` makes it a coordinate of the order, `148` makes it one of two inputs to the range
coordinate, coupled with precision and never touching the lattice coordinate.

**The refutation is adopted; the replacement is not.**

## A probe that used forbidden features, and its replacement

`146`'s `From` spelling was reported as compiling. `148` found that the probe carries
`#![feature(generic_const_args)]` and its runner passes `-Znext-solver=globally`, and that it exits 1
without the flag. Neither is inside what the design permits.

Under `conceding-is-an-answer-and-expert-code-is-a-spike.md` a probe is evidence for exactly what it
proved, and this one proved something about a configuration the design does not allow. The claim as
`146` stated it does not stand.

`148` supplies a compliant replacement: carry the condition as a trait bound rather than as a const
argument, which compiles gate-free on the default solver over the design's own width encoding to 128
bits, with `#[diagnostic::do_not_recommend]` restoring the error message the encoding otherwise
loses.

**What survives from `146` here is the coherence argument**, and `148` strengthens rather than
weakens it: the overlap with `core`'s reflexive impl fails at the head constructor, above where
substitution happens, so it is structurally impossible rather than merely untriggered on the cases
tried. Two experts now agree on that, independently.

`148` then argues the by-reference surface is the right call-site spelling and the wrong bound
surface, and that an arvo-owned `Embed<T>` by value has no coherence question at all, with both
erasing to the same symbol. That is one read and is not adopted.

## An unasked finding, which is how the rules want it

`148` reports that `146`'s third instance of the coupling pattern rests on a `Ranged` model the
design's own quoted sentence contradicts, that its counterexample is a clean inclusion with no
missing values, and that its proposed condition would make the design refuse valid conversions.

Recorded, not adjudicated. It is one read against one read.

## For op

Carried forward from `146` and `148`, with the panel's readings attached where they differ:

- **The `Ranged` model**, which `148` disputes in `146` and which its own third-instance claim
  depends on.
- **The rule for picking a common target where no join exists**, if `148`'s direction survives the
  third read. It says this cannot be derived and must be named.
- **Whether `Embed` ships beside `From`**, or the `From` alone.
- **The width encoding's residual cost**, that widths print as `Pv<I<O<I<H>>>>` in diagnostics.
- **`Precise` on `inexact`**, unchanged and open since `145`.

## What the dispatcher owes next

A third read on the algebraic structure, briefed on the mechanism rather than the counts, and
briefed without either prior answer's framing. That is the only question here where the panel has
produced three answers and zero agreements, and it is the one whose consequence, whether a common
target is derivable or must be named, reaches furthest into the canon.
