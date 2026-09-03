# 259 probes. Whether the fused answer is reachable by composing a multiply and an add

Built for `259_fallin_whether_the_fused_result_composes.md`. Run with `./run`
from this directory. Everything lands under `out/` and is committed, so every
number in the panel file can be checked against a run rather than against the
prose around it. The whole run takes about three minutes.

Not a bench. No timing is taken anywhere here and no number in the panel file is
a duration, a cost or a ratio. These are exhaustive equality sweeps over slot
triples, so what they produce is counts and named witnesses. The release profile
is a runtime convenience: a debug build of step 01 is slow enough that somebody
would be tempted to sample, and the test gate refuses a law asserted over a
sample of the shapes it supports.

Steps 01 to 06 were written and run before `226_lattner_the_derivation_outputs.md`
was opened, and step 07 after. The two commits are separate and `git log` is what
says which is which.

## What the instrument is

Every adaptation point in steps 01 to 05 and 07 is `arvo_format::apply::adapt`,
the shipped realisation of the ratified factoring in
`ruling::the_format_spine_is_canon`. Nothing in those steps reimplements a
rounding mode or a range policy. That matters twice: a disagreement between two
arms is a disagreement between two schedules of one map rather than between this
code and the crate, and the numbers are about what arvo does rather than about
what a model of arvo does.

Step 06 is the exception and says so in its own header. It models the rounding
region, because the mode it measures is one the crate does not ship, and it
calibrates that model against the shipped map on the mode both have before using
it. The completion region there is still the shipped one.

Positions are slot coordinates. A numeral of fraction length `F` has quantum
`2^-F`, so the value on slot `k` is `k * 2^-F` and the exact product of slots `a`
and `b` sits at slot position `a * b / 2^F`. That position is carried as a
`Fraction`, exactly, which is what makes a tie a tie rather than whatever the
host's floating point happened to do.

The numerals are `arvo_format::standards::Fi` and `Ufi`, the MATLAB `fi`
declarations the crate already ships, at word lengths 3 to 8 and every fraction
length from 0 to `W - 1`.

## The steps

| step | binary | what it does | the arms that must fail |
|---|---|---|---|
| 01 | `sweep` | fused against the natural stepwise composition, exhaustively, 660 cells and 3,013,048,320 triples, widths 3 to 8 | A2, A5, A6 |
| 02 | `mechanism` | the applied map's two regions measured apart, and step 01 predicted from them cell by cell | B2, B3 separate |
| 03 | `existential` | is the fused answer reachable by any of twenty arms a consumer could write at one format, widths 3 to 6 | C1 |
| 04 | `widening` | the fused answer through an intermediate at a wider declared signature | D3, D4 |
| 05 | `derivable` | the same route written once, generically over the numeral, plus the blanket impl that must be refused | E2 |
| 06 | `tie_direction` | whether the one disagreement with a committed law row is about arithmetic or about a word | F3 |
| 07 | `one_placement` | the third schedule, one completion with the intermediate on the declared grid | G3, G4 |

## What each step establishes

**01.** Which cells the natural composition agrees on. The five deterministic
members of the ratified rounding vocabulary, both overflow policies, both
signednesses, widths 3 to 8, fraction lengths 0 to `W - 1`. `stochastic` is
excluded by name and not by omission: its result is not a function of the value,
so equality of two realisations is not a well posed question for it, and a
predicate listing it would be claiming something no run here supports.

**02.** The mechanism, so the table is predictive rather than tabular. The fused
realisation is `complete(round(ab + c))` and the stepwise one is
`complete(complete(round(ab)) + c)`, because the outer rounding is dead: a slot
plus a slot is on the grid. Two conditions decide the pair. E is equivariance of
the rounding region under translation by a representable value, measured through
a format wide enough that the completion never fires, and B1 measures that the
wide format never fires rather than asserting it. H is the completion region
being a homomorphism for that translation, measured on grid positions only so the
rounding is the identity.

**03.** The existential the ruling under review uses. An arm is a choice of the
multiply's mode, the multiply's policy and the add's policy; the add's mode is
not a choice and C3 measures that rather than assuming it. Twenty arms against
ten targets at every cell of widths 3 to 6.

**04.** The route the first three steps close off by holding the format fixed.
Three declared signatures, three adaptation points, with the two intermediate
ones measured to be the identity at every triple of every cell.

**05.** Whether that route is derivable or only writable. The widening relation
as an associated type with per-numeral impls builds and reproduces the answers;
the same relation as one blanket impl computing the wide width from the narrow
one is refused, and the refusal is committed.

**06.** The one place this seat disagrees with a committed law row, taken apart.
Nearest-with-a-tie has two readings on a signed domain and the crate implements
one of them. F2 calibrates a model of the rounding region against the shipped map
on that reading, F3 shows the other reading is a different operation, and F1
measures the other reading over the region the committed row claims.

**07.** The third schedule, added after reading seat 226, which runs it and this
seat's first six steps do not. The product is rounded onto the declared grid and
added to the addend with no completion between, so the whole expression completes
once. Step 02's decomposition predicts what that does before it runs: with one
completion on each side H cannot separate the two, so the pair agrees exactly
where E holds and the overflow policy drops out. G1 and G2 check both halves, and
G3 is the case that separates this schedule from step 01's.

## The one broken arm, and why it stays broken

`C2` in step 03 is BROKEN in `out/03_existential.txt` and in
`out/08_verdicts.txt`, and it is not repaired. It was stated before the run and
it claimed that outside signed saturating every target is reachable by some arm.
Seventy of two hundred and seventy are reachable by nothing. The break is the
finding of that step, so repairing the arm would delete the result. `C5` and `C6`
state what the run actually shows, and they are marked in the source as written
after the break and therefore worth less than the four arms that preceded it.

Thirty-five arms across the seven steps, thirty-four held, and that one did not.

## The refusals, committed

- `p01_the_composition_sweep/src/derivable_blanket.stderr`, copied to
  `out/05_e2_refusal.txt`: three `generic_const_exprs` errors, which is the
  feature this workspace forbids. `out/05_e2_exit.txt` carries the exit status.
- `p01_the_composition_sweep/src/derivable_blanket_control.stderr`: empty.
  `out/05_e3_exit.txt` carries the control's exit status and its one line of
  output, so the pair is a refusal and a build rather than a refusal alone.

## What this is not

Not a design, not a bench, and not a test in arvo's suite. The equivariance
property step 02 measures has no test in `crates/arvo-format/src/apply/tests/`
and it should have one; a panel seat cannot land it, because source is gated
behind a design round. `hand_check_half_up` in `mechanism.rs` is that test already
written, with its two controls, waiting for a round that can take it.
