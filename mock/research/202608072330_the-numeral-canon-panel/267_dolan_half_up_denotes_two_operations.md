# Seat 267: `half_up` denotes two operations, and the canon already says so

## Canon gate

`mockspace.toml` names `canon_paths = ["mock/registry/*.toml"]`. The question,
`question::which_operation_half_up_denotes`, is a filed panel question with
`decider = "panel"`, aligned with the canon process for resolving an
underspecified name in the ratified six-mode vocabulary. Nothing in the
question conflicts with a ratified row; it asks the panel to settle what a
ratified row's own name denotes. Proceeding.

## Phase one: derivation, before reading seat 229

### The governing row decides the shape before any measurement does

`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`
(`ratified_by = "op"`) says:

> The parity suite is built against MATLAB `fi`/`fimath` and IEEE 754.

`ruling::the_standard_is_parity_in_output_not_in_the_internals`
(`ratified_by = "op"`) says:

> The standards bound is met by agreeing with what a standard documents as its
> result, for the cases it covers.

Both are ratified and both bind before any question about `half_up`'s
denotation is asked. So the test for which reading `half_up` should carry is
not a preference between two conventions. It is: does the ratified name, read
either way, reproduce what MATLAB `fi` and IEEE 754 document as their result,
for the cases they cover.

**MATLAB `fi`/`fimath` documents six `RoundingMethod` values, and two of them
are exactly the two readings the question poses, under two different names.**
Checked against MathWorks' own documentation (`mathworks.com/help/fixedpoint`),
2026-09-18:

> `Nearest` — Round to the nearest representable value. ... The nearest
> function rounds ties to the nearest integer toward positive infinity.
>
> `Round` — Round to the closest representable number with ties rounded based
> on the sign of the value. ... The round function rounds ties to the nearest
> integer with greater absolute value.

("toward positive infinity" is reading 1 of the question; "greater absolute
value" on a signed domain is reading 2, ties away from zero.) MATLAB does not
have one name for this family. It has two, `Nearest` and `Round`, because the
two operations disagree at every negative tie and MathWorks needed both
addressable.

**IEEE 754 agrees with the split from its own side.** IEEE 754-2008 names five
rounding-direction attributes: `roundTiesToEven` (the required default),
`roundTiesToAway`, and the three directed roundings. `roundTiesToAway` is
defined as:

> ties are rounded up towards positive infinity for positive values, and for
> negative values, ties are rounded down towards negative infinity

which is reading 2 (magnitude increases on every tie, symmetric about zero) and
is IEEE's own name for it. IEEE 754 has no rounding-direction attribute at all
for reading 1 (ties toward positive infinity regardless of sign); that
convention does not appear in the standard under any name.

So: reading 1 is a real, standard-documented operation (MATLAB `Nearest`, no
IEEE 754 name). Reading 2 is a real, standard-documented operation (MATLAB
`Round`, IEEE `roundTiesToAway`). Neither standard treats them as the same
operation under different words; both standards that the canon is already
bound to reproduce treat them as two operations, and one of MATLAB's own two
names for this pair is spelled differently from `half_up` in either direction.

**This is the governing constraint, and it decides the question's shape before
any measurement of arvo's own code does.** A single arvo name `half_up`
denoting only one of the two readings cannot reproduce, for the cases MATLAB
`fi` covers, whichever of `Nearest` or `Round` it does not implement. The
standards bound is violated by construction the moment `half_up` is asked to
be one operation. `never-ask-which-single-rule-governs.md`: the question as
filed offers three options as if they were competing single answers, but the
third option (two names, one per reading) is not one candidate among equals,
it is the only one of the three that satisfies a ratified row that already
binds. Option 1 and option 2 are each a region where the *other* standard
operation goes unnamed and therefore unreachable under the ratified
vocabulary, which is not a naming inconvenience, it is a standards-parity gap
against rows `ratified_by = "op"`.

**Answer: option 3's shape, restated as arms rather than a pick.** `half_up`
does not get redefined to one reading; the vocabulary needs both readings each
under its own name, because both are standard-documented operations arvo is
already bound to reproduce. Which of the two keeps the name `half_up` is a
second, smaller decision (below), and it is not forced by the standards bound
alone.

### My own probe: the two readings are mathematically distinct operations, not a documentation ambiguity

Built independent of seat 229's probes (not opened at this point):
`267_probes/a_equivariance_proof.rs`, run and output committed at
`267_probes/a_output.txt`.

- **Reading 1 is `floor(x + 1/2)`**, an unconditional closed-form identity:
  `floor(y + k) = floor(y) + k` for every integer `k`, so reading 1 is
  *exactly* translation equivariant, everywhere, not just on the swept domain.
  The sweep (signed 6-bit container, `fraction_width` 0..=5, shifts -4..=4)
  found 35,775 checks, 0 failures.
- **Reading 2 (ties away from zero)** is not translation equivariant: 100
  failures out of 35,775 checks on the identical domain, all at fraction
  widths carrying a representable tie (`fraction_width >= 1`).
- **Control**: `half_even`, which the fma law's own `fails` region already
  lists as non-equivariant, fails 1,260 of 35,775 checks on the same harness,
  proving the check can report a failure rather than passing everything
  vacuously.
- **The two readings differ at every one of 160 negative ties checked**
  (`fraction_width` in {1..5}, every representable tie in the signed 6-bit
  domain), confirming the question's own claim that they differ at every
  negative tie.

`holds for: total_width = 6, fraction_width in 0..=5, signedness = signed,
target_features any (the equivariance identity is architecture-independent),
rounding = the two named readings of half_up plus half_even as control`. The
translation-equivariance identity for reading 1 is a closed-form proof and
holds unconditionally over width and target; the sweep is a check on the
proof, not the source of it.

### This matches, and exposes a live contradiction in, the ratified/measured law row

`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` has
`holds` naming `rounding: in {floor, ceil, half_up}` (the
translation-equivariant set) and `fails` naming
`rounding: in {toward_zero, away from zero, half_even}`. My probe reproduces
this partition exactly under reading 1 and breaks it under reading 2, which is
precisely what the question's own option 2 warns of: read `half_up` as
ties-away-from-zero and the law's `holds` region shrinks from three modes to
two.

**But `arvo-format`'s shipped `Mode::HalfUp` is reading 2**, not reading 1.
`mock/crates/arvo-format/src/apply/tests/mod.rs:268-279`
(`half_up_goes_away_from_zero_on_a_tie_and_half_even_goes_to_the_even_slot`)
asserts a tie at `-2.5` rounds to `-3` under `Mode::HalfUp`, magnitude
increasing, sign preserved: away from zero. So the measured law row and the
shipped crate already disagree about what `half_up` computes: the law was
measured against seat 226's own probe implementation (reading 1, per the
question row's own note), and the crate that ships under the same name
computes reading 2. **This is a standing defect independent of which way the
naming question is answered**: today,
`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` makes
a claim about `Mode::HalfUp` that `Mode::HalfUp` does not satisfy.

### The second-order decision: which reading keeps the name `half_up`

Not forced by the standards bound (either name could denote either reading);
forced by cost, and the costs are asymmetric:

- **Give `half_up` reading 1** (ties toward +inf, matching MATLAB `Nearest`).
  The measured law row needs no correction; it already describes reading 1.
  Cost: `arvo-format`'s shipped `Mode::HalfUp` is currently reading 2 and has
  to change, and its own test at `mod.rs:268` has to be rewritten (the tie at
  `-2.5` would then round to `-2`, not `-3`), and the new operation (reading 2)
  needs a new name and a re-measurement of the law region it moves into.
- **Give `half_up` reading 2** (ties away from zero, matching MATLAB `Round`
  and IEEE `roundTiesToAway`). The shipped crate is already correct and needs
  no change. Cost: the measured law row is wrong as written today and needs
  correcting (`half_up` drops out of `holds`, per the question row's own
  option 2 text), and the new operation (reading 1) needs a new name, under
  which the translation-equivariant identity gets re-established as a law for
  that new name rather than for `half_up`.

Both costs are real and neither is free; I do not pick between them, since
which one costs less is a question about how much else in the corpus already
assumes one reading of `half_up` by name, which is a census I have not run and
is properly the panel's or op's to weigh against the shipped-code cost versus
the measured-law cost. What I can say from the governing row alone: **whichever
reading loses the name `half_up`, both readings need a name**, because both are
standards-documented operations the canon is already bound to reproduce.

## Committing phase one

Committed here, before opening `229_*`.

---

## Phase two. Reconciliation against seat 229

Written after the phase-one text above was committed and pushed. Nothing above
this line was edited. Read:
`229_lamport_do_the_six_names_denote_one_operation_each.md`.

### Where we agree

**The two readings are the same two operations, defined the same way, and my
probe reproduces 229's core measured facts independently.** 229 names reading
one `floor(x + 1/2)` and reading two `roundTiesToAway`/away-from-zero at lines
176-183; my probe defines the identical two functions and derives the same
closed-form fact that reading one is exactly `floor(x + 1/2)`.

**Both of us find the ratified/measured fma law row is true only under reading
one.** 229's finding 4 and its account at lines 285-301 states the signed row
holds for `{floor, ceil, half_up}` (three of six) under the toward-positive-
infinity reading and drops to `{floor, ceil}` (two of six) under the other,
matching my own probe's independent reproduction of the same partition
(0 failures under reading 1, breaking to exclude `half_up` under reading 2).

**Both find the trade between the two properties is forced, not a preference.**
229's probe F enumerates the entire nearest-mode space at four domain points
and finds zero modes both translation equivariant and zero mean error (lines
327-341: "the trade is a theorem over these domains"). My probe establishes the
same fact by closed form for the two named readings specifically: reading one
is exactly equivariant, reading two is not, over the identical law-row domain.

**Both find the two readings differ exactly on the signed, non-zero-fraction
region.** 229's probe A gives the disagreement count `2^(W-1-F)` at lines
185-189; my probe finds the two readings differ at every one of 160 negative
ties checked over the signed 6-bit, `fraction_width` 0..=5 domain, which is
the same region stated the other way (by tie count rather than by disagreement
count over the whole domain).

### Where we disagree

**The shape of the answer.** My phase one answers that the question's shape
is wrong as posed: both readings are independently standards-documented
operations (MATLAB `fi`'s `Nearest` and `Round`, the second matching IEEE
754's `roundTiesToAway`), the ratified standards-parity rows already bind
arvo to reproduce both, and collapsing them under one name violates that bound
for whichever reading loses. 229 answers the opposite shape explicitly, at
lines 197-200: "What is missing is one sentence, not a seventh name and not a
rename," and again at lines 655-661 ("the gap is one sentence in one row"),
proposing to pin `half_up` to reading one alone and leaving reading two
unnamed, absorbed informally into the case for `half_even` (lines 356-360:
"the choice between those two is on other grounds").

**229's own canon gate did not check the rows my answer turns on.** The gate
list at lines 9-18 names eight rows and does not name
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` or
`ruling::the_standard_is_parity_in_output_not_in_the_internals`, though the
question is precisely about which operation matches which standard's
documented result. 229's admissibility test for a "real" second reading
(lines 36-39: written in a standard, or shipped, or a hardware realisation)
is close to the standards bound but never connects to it as a ratified
obligation arvo itself carries; IEEE 754 and MATLAB naming appear only as
evidence that reading two is a real convention (lines 518-520), not as a
constraint on how many names the vocabulary needs.

**229 never opened the crate's own shipped test, and it is load-bearing
against 229's proposed fix.** `mock/crates/arvo-format/src/apply/tests/mod.rs:268-279`
asserts `Mode::HalfUp` at a tie of `-2.5` gives `-3`, away from zero: reading
two, not reading one. 229's own file cites no `arvo-format` source or test
anywhere in it; its evidence for reading two is external (IEEE 754, Java,
Python, .NET) rather than internal. If 229's proposed one-sentence note pins
`half_up` to reading one without a matching change to `arvo-format`, the
ratified vocabulary would say `half_up` is reading one while the crate
shipping under that name computes reading two, which is the exact
contradiction my phase one names as a standing defect independent of how the
naming question is answered. This is not a refutation of 229's arithmetic,
which my own instrument reproduces; it is a gap in what 229's derivation
checked before proposing its fix.

### What this leaves for the panel or op

Two live proposals, not reconciled here: mine (two ratified names, one per
reading, because the standards bound already requires both) and 229's (one
name, `half_up`, pinned by note to reading one, with reading two left
unnamed and its niche use covered informally by `half_even`). Both agree on
every measured fact; they disagree on whether the standards-parity rulings
force a second name into the vocabulary. That disagreement, and the fact that
229's fix as stated would leave `arvo-format`'s shipped `Mode::HalfUp` test
contradicting a newly pinned `half_up`, is what I am handing forward rather
than resolving unilaterally, per `a-conflict-is-consolidated-never-selected.md`.
